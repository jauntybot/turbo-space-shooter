use std::sync::Arc;

use super::*;

#[turbo::serialize]
pub struct PlayerStats {
    pub max_hp: u32,
    pub speed: f32,
    pub damage: u32,
    pub rate_of_fire: u32,
    pub projectile_speed: u32,
}

// Struct for Player properties
#[turbo::serialize]
pub struct Player {
    pub hitbox: Hitbox,
    dx: f32, // dx and dy used for movement
    dy: f32,
    
    pub hp: u32,
    
    pub hit_timer: u32, // used for invincibility frames and drawing
    shoot_timer: u32, // used for rate of fire
    shooting: bool, // used for shooting animation
    
    pub stats: PlayerStats,
    
    // TODO: add functionality for different projectile types
    pub projectile_type: ProjectileType,
    
    // variables used by the HUD to display information
    pub score: u32,
    pub notifications: Vec<String>,
}

impl Player {
    pub fn new() -> Self {
        let (screen_w, screen_h) = resolution();
        Player {
            // Initialize all fields with default values
            hitbox: Hitbox {
                x: ((screen_w / 2) - 8) as f32,
                y: (screen_h - 64) as f32,
                w: 16,
                h: 16,
            },
            dx: 0.0,
            dy: 0.0,
            hp: 3,
            
            hit_timer: 0,
            shoot_timer: 0,
            shooting: false,
            
            stats: PlayerStats {
                max_hp: 3,
                speed: 2.0,
                damage: 1,
                rate_of_fire: 15,
                projectile_speed: 5,
            },

            projectile_type: ProjectileType::Basic,
            
            score: 0,
            notifications: vec![
                "Use arrow keys to move.".to_string(),
                "Press SPACE or A to shoot.".to_string(),
                "Defeat enemies and collect powerups.".to_string(),
                "Try to not die. Good luck!".to_string(),
            ],
        }
    }
    // update is called once per frame within the [turbo::game] loop
    pub fn update(&mut self, projectiles: &mut Vec<Projectile>, powerups: &mut Vec<Powerup>, enemies: &mut Vec<Enemy>) {
        let (screen_w, screen_h) = resolution();
        if self.hp != 0 {
            // Player movement handling (normalized for diagonal movement)
            self.dx = 0.0;
            self.dy = 0.0;
            if gamepad::get(0).up.pressed() {
                self.dy -= 1.0;
            }
            if gamepad::get(0).down.pressed() {
                self.dy += 1.0;
            }
            if gamepad::get(0).left.pressed() {
                self.dx -= 1.0;
            }
            if gamepad::get(0).right.pressed() {
                self.dx += 1.0;
            }
            if pointer::screen().pressed() {
                let (px, py) = pointer::screen().xy();
                if (px as f32) < self.hitbox.x + self.hitbox.w as f32 / 2.0 - (self.stats.speed / 2.0) {
                    self.dx -= 1.0;
                } else if (px as f32) >= self.hitbox.x + self.hitbox.w as f32 / 2.0 + (self.stats.speed / 2.0) {
                    self.dx += 1.0;
                }
                if (py as f32) < self.hitbox.y {
                    self.dy -= 1.0;
                } else if (py as f32) > self.hitbox.y + self.hitbox.h as f32 {
                    self.dy += 1.0;
                }
            }
            let len = ((self.dx as f32) * (self.dx as f32) + (self.dy as f32) * (self.dy as f32)).sqrt();
            if len > 0.0 {
                let speed = self.stats.speed;
                let nx = self.dx / len;
                let ny = self.dy / len;
                self.hitbox.x = (self.hitbox.x + nx * speed).clamp(0.0, (screen_w - self.hitbox.w) as f32);
                self.hitbox.y = (self.hitbox.y + ny * speed).clamp(0.0, (screen_h - self.hitbox.h) as f32);
            }

            // Shooting projectiles
            // check if shoot button is pressed
            if gamepad::get(0).start.pressed() || gamepad::get(0).a.pressed() || pointer::screen().pressed() {
                self.shooting = true; // flag shooting state for animation
                // if shoot timer is 0, shoot a projectile
                if self.shoot_timer == 0 {
                    self.shoot_timer += self.stats.rate_of_fire; // reset shoot timer
                    for i in 0..=1 {
                        projectiles.push(
                            Projectile::new(
                                self.hitbox.x + i as f32 * 13.0,
                                self.hitbox.y - 8.0,
                                5.0,
                                -90.0,
                                self.projectile_type.clone(),
                                ProjectileOwner::Player,
                            )
                        );
                    }
                    audio::play("projectile_player_shoot");
                }
            // if not shooting
            } else {
                self.shooting = false; // flag shooting state for animation
            }
            // decrement shoot timer
            self.shoot_timer = self.shoot_timer.saturating_sub(1);
        }
        
        // Handle player collecting power-ups
        // Iterate through all spawned power-ups, removing any the player collides with
        powerups.retain(|powerup| {
            // If player collides with this power-up
            if check_collision(
                &self.hitbox,
                &powerup.hitbox,
            ) {
                // Player collects this power-up
                self.collect_powerup(powerup);
                false // Remove this power-up after it's picked up
            } else {
                true
            }
        });

        // Iterate through enemies, checking if any collide with the player
        enemies.iter_mut().for_each(|enemy| {
            if enemy.hp > 0 && check_collision(
                &self.hitbox,
                &enemy.hitbox,
            ) {
                // Collision detected, both take damage
                self.take_damage(1);
                enemy.take_damage(self, enemy.hp);
            }
        });

        // hit timer
        self.hit_timer = self.hit_timer.saturating_sub(1);
        // Remove the camera shake
        if self.hit_timer == 0 {
            camera::remove_shake();
        }
    }

    pub fn reset(&mut self) -> bool {
        self.hit_timer = self.hit_timer.saturating_sub(1);
        // Remove the camera shake
        if self.hit_timer == 0 {
            camera::remove_shake();
        }
        // Restart
        if self.hit_timer == 0 
        && (gamepad::get(0).start.just_pressed()
        || gamepad::get(0).a.just_pressed())
        {
            return true;
        }
        false
    }
    // Function to handle player taking damage
    pub fn take_damage(&mut self, damage: u32) {
        self.hp = self.hp.saturating_sub(damage); // reduce HP by damage amount
        camera::shake(5.0); // camera shake
        self.hit_timer = 20; // invincibility frame timer and drawing flag
    }
    // Function to handle player collecting a power-up
    pub fn collect_powerup(&mut self, powerup: &Powerup) {
        // Apply the effect based on the power-up type
        match &powerup.effect {
            // Heal player
            PowerupEffect::Heal => {
                self.hp = (self.hp + 1).min(self.stats.max_hp);
                self.notifications.push("+1 HP".to_string());
            }
            // Heal and increase max HP
            PowerupEffect::MaxHealthUp => {
                self.stats.max_hp = (self.stats.max_hp + 1).min(10);
                self.hp += 1;
                self.notifications.push("MAX HP +1".to_string());
            }
            // Increase speed
            PowerupEffect::SpeedBoost => {
                self.stats.speed *= 1.1;
                self.notifications.push("1.1x SPEED BOOST".to_string());
            }
            // Increase damage
            PowerupEffect::DamageBoost => {
                self.notifications.push(format!("+1 DAMAGE"));
                self.stats.damage = (self.stats.damage + 1).min(2);
            }
            // Increase rate of fire
            PowerupEffect::RateOfFireBoost => {
                self.stats.rate_of_fire = (self.stats.rate_of_fire.saturating_sub(1)).max(5);
                self.notifications.push("+1 RATE OF FIRE".to_string());
            }
            // Increase projectile speed
            PowerupEffect::ProjectileSpeedBoost => {
                self.stats.projectile_speed = (self.stats.projectile_speed + 1).min(10);
                self.notifications.push("+1 PROJECTILE SPEED".to_string());
            }
        }
    }

    pub fn draw(&self) {
        // get reference to SpriteAnimation for player
        let anim = animation::get("player");
        
        // begin to construct the string for which sprite to use
        let mut sprite = String::from("player/player");
        // if the player is moving left
        if self.dx < 0.0 {
            sprite.push_str("_bankL");

        } else if self.dx > 0.0 {
            sprite.push_str("_bankR");
        }
        if self.shooting {
            sprite.push_str("_shooting");
        }
        // Assign the sprite string to the SpriteAnimation
        anim.use_sprite(&sprite);
        // Flash red when hit
        let color = if self.hit_timer > 0 && self.hit_timer % 10 < 5 {
            0xff0000ff
        } else {
            0xffffffff
        };
        sprite!(
            animation_key = "player",
            x = self.hitbox.x,
            y = self.hitbox.y,
            color = color);
    }
}
