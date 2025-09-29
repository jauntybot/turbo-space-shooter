use turbo::*;
mod model;
use model::*;

// Different scenes in the game handled by a state machine
#[turbo::serialize]
pub enum Scene {
    Menu,
    Game,
    GameOver,
}

// The [turbo::game] macro sets up the game loop.
#[turbo::game]
// It expects a GameState struct with a new() and update(&mut self) method, which is executed once per frame.
// All game logic and rendering is handled within the update method, so every stored entity must be updated and drawn in this scope.
struct GameState {
    // Game management variables
    scene: Scene,
    tick: u32, // using our own tick counter for Scene management, able to reset to 0
    
    // Game entities
    // To keep the project organized, these entities are defined in separate files within the model/ directory
    hud: HUD,
    player: Player,
    enemies: Vec<Enemy>,
    projectiles: Vec<Projectile>,
    powerups: Vec<Powerup>,
}

impl GameState {
    // Req. for [turbo::game] macro
    // Initialize a new game state
    fn new() -> Self {
        Self {
            // Initialize all fields with default values
            scene: Scene::Menu,
            tick: 0,
            
            hud: HUD::new(),
            player: Player::new(),
            enemies: vec![],
            projectiles: vec![],
            powerups: vec![],
        }
    }
    // Req. for [turbo::game] macro
    // Update the game state each frame
    fn update(&mut self) {
        // Drawing all game elements, including player, enemies, environment, and UI
        self.draw_game_elements();
        // State machine for the game scene, updating game state contextually
        match self.scene {
            // Main menu scene
            Scene::Menu => {
                // Draw menu
                self.hud.draw_menu(self.tick);
                // Start game on button press
                if gamepad::get(0).start.just_pressed() || gamepad::get(0).a.just_pressed() {
                    self.scene = Scene::Game; // transition scene
                    self.tick = 0;
                }
                // Increment tick counter for menu animations
                self.tick += 1;
            }
            // Game and Game Over scenes
            Scene::Game | Scene::GameOver => { 
                // Update the player, or prompt for restart if dead
                if self.player.hp > 0 {
                    self.player.update(&mut self.projectiles, &mut self.powerups, &mut self.enemies);
                } else {
                    self.scene = Scene::GameOver;
                    if self.player.reset() {
                        *self = Self::new(); // reset entire game state to initial value
                    }
                }
                
                // Update the HUD
                self.hud.update(&mut self.player);
                
                // Spawn enemies periodically 
                self.spawn_enemies();
                // Update enemies, passing a mutable reference to the player and projectiles, and remove those flagged as destroyed
                self.enemies.retain_mut(|enemy| {
                    enemy.update(&mut self.player, &mut self.projectiles);
                    // If the enemy is destroyed, there is a chance to spawn a powerup
                    if enemy.destroyed && random::u32() % 10 == 0 {
                        // Spawn power up
                        self.powerups.push(
                            Powerup::new_random(
                                enemy.hitbox.x,
                                enemy.hitbox.y,
                                &mut self.player,
                            )
                        );
                    }
                    !enemy.destroyed
                });
                
                // Update projectiles, remove those flagged as destroyed
                self.projectiles.retain_mut(|projectile| {
                    projectile.update(&mut self.player, &mut self.enemies);
                    !projectile.destroyed
                });
                
                // Spawn heal powerups periodically 
                self.spawn_powerups();
                // Update spawned power-ups
                for powerup in &mut self.powerups {
                    powerup.update();
                }

                // Increment tick counter for game timing
                self.tick += 1;
            }
        }
    }

    fn spawn_powerups(&mut self) {
        let (screen_w, screen_h) = resolution();
        // Every 30s, if the player is missing HP, spawn a heal at a random location
        if self.tick % (60 * 30) == 0 && self.player.hp < self.player.stats.max_hp {
            self.powerups.push(
                Powerup::new(
                    (random::u32() % screen_w) as f32,
                    24.0 + (random::u32() % screen_h / 2) as f32,
                    PowerupEffect::Heal,
                    PowerupMovement::FloatHorizontal(0.75),
                )
            );
        }
    }

    fn spawn_enemies(&mut self) {
        // Start spawning enemies after intro dialog
        if self.tick > (self.player.notifications.len() as u32 + 1) * 240 {
            // Enemy spawning logic based on time elapsed
            // Define spawn intervals (in ticks) for enemies
            let initial_spawn_rate: u32 = 100; // Initial interval for enemy spawn
            let minimum_spawn_rate = 25; // Minimum interval after speeding up
            let speed_up_rate = 60 * 2; // Interval after which spawn rate increases

            // Calculate current spawn interval based on time elapsed
            let spawn_rate = std::cmp::max(
                minimum_spawn_rate,
                initial_spawn_rate.saturating_sub(self.tick / speed_up_rate),
            );
            // Spawn a new enemy if the tick is a multiple of the spawn rate
            if self.tick % spawn_rate == 0 && self.enemies.len() < 24 {
                // Spawn a random enemy with these probabilities
                self.enemies.push(match random::u32() % 8 {
                    0 => Enemy::new(EnemyType::Tank),
                    1 => Enemy::new(EnemyType::Tank),
                    2 => Enemy::new(EnemyType::Shooter),
                    3 => Enemy::new(EnemyType::Shooter),
                    4 => Enemy::new(EnemyType::Meteor),
                    5 => Enemy::new(EnemyType::Zipper),
                    6 => Enemy::new(EnemyType::Turret),
                    7 => Enemy::new(EnemyType::Turret),
                    _ => unreachable!(),
                });
            }
        }
    }

    // Renders all game elements each frame
    // Rendering is sequential from background to foreground
    fn draw_game_elements(self: &GameState) {
        let (screen_w, screen_h) = resolution();
        // Draw moving parallax stars in the background
        self.draw_stars(screen_w, screen_h);
        // Draw enemies
        for enemy in &self.enemies {
            enemy.draw();
        }
        // Draw powerups
        for powerup in &self.powerups {
            powerup.draw(self.tick);
        }
        // Drawing the player
        if self.scene == Scene::Game {
            self.player.draw();
        } else if self.scene == Scene::GameOver {
            self.hud.draw_game_over(self.tick);
        }
        // Draw projectiles
        for projectile in &self.projectiles {
            projectile.draw();
        }
        // Draw game HUD
        self.hud.draw(&self.player);
        if self.scene == Scene::Game {
            self.hud.draw_notifications(&self.player);
        }
    }

    fn draw_stars(self: &GameState, screen_w: u32, screen_h: u32) {
        // Define star layers with different speeds
        let star_layers = [
            (54321, 1, 0.15, 10),
            (12345, 1, 0.25, 10),
            (67890, 2, 0.35, 10),
        ];

        for &(seed, size, speed, count) in star_layers.iter() {
            for i in 0..count {
                let rand_x = rand_with_seed(seed + i + self.tick / 10) % screen_w;
                let rand_y = (rand_with_seed(seed + i + self.tick / 10) / screen_w) % screen_h;

                // Adjust position slightly based on player movement
                let adjust_x = self.player.hitbox.x * speed / 5.0;
                let adjust_y = self.player.hitbox.y * speed / 5.0;

                let x = rand_x as i32 - adjust_x as i32;
                let y = (self.tick as f32 * speed) as i32 + rand_y as i32 + adjust_y as i32;

                // Draw the star
                circ!(
                    x = x % screen_w as i32,
                    y = y % screen_h as i32,
                    d = size,
                    color = 0xFFFFFF44
                );
            }
        }
    }
}
