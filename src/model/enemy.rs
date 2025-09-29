use super::*;

#[turbo::serialize]
// Struct for Enemies
pub struct Enemy {
    enemy_type: EnemyType,
    strategy: EnemyStrategy,
    
    pub hitbox: Hitbox,
    pub hp: u32,
    pub points: u32,
    
    pub speed: f32,
    pub angle: f32,
    
    hit_timer: u32, // used for drawing
    pub destroyed: bool,
}

// AI States for enemy behavior
#[turbo::serialize]
enum EnemyStrategy {
    TargetPlayer(f32, f32, u32), // Moves down. Attacks with given intensity, speed, and size
    ShootDown(f32, f32, u32),    // Moves down. Attacks with given intensity, speed, and size
    MoveDown,                    // Moves down. Nothing fancy
    RandomZigZag(f32),           // Moves in a random zig zag pattern with a given angle
}

// Different types of enemies
#[turbo::serialize]
pub enum EnemyType {
    Tank,
    Shooter,
    Turret,
    Zipper,
    Meteor,
}

impl Enemy {
    // Initialize different enemy types with different properties
    pub fn new(enemy_type: EnemyType) -> Self {
        let (screen_w, _) = resolution();
        // Set initial properties based on enemy type
        match enemy_type {
            EnemyType::Tank => {
                Self {
                    enemy_type: EnemyType::Tank,
                    strategy: EnemyStrategy::TargetPlayer(1.0, 2.5, 8),
                    hitbox: Hitbox {
                        x: (random::u32() % screen_w).saturating_sub(32) as f32,
                        y: -32.0,
                        w: 32,
                        h: 32,
                    },
                    hp: 10,
                    points: 50,
                    speed: 0.25,
                    angle: 0.0,
                    destroyed: false,
                    hit_timer: 0,
                }
            },
            EnemyType::Shooter => {
                Self {
                    enemy_type: EnemyType::Shooter,
                    strategy: EnemyStrategy::TargetPlayer(3.0, 2.0, 4),
                    hitbox: Hitbox {
                        x: (random::u32() % screen_w).saturating_sub(16) as f32,
                        y: -16.0,
                        w: 16,
                        h: 16,
                    },
                    hp: 5,
                    points: 30,
                    speed: 1.0,
                    angle: 0.0,
                    destroyed: false,
                    hit_timer: 0,
                }
            },
            EnemyType::Turret => {
                Self {
                    enemy_type: EnemyType::Turret,
                    strategy: EnemyStrategy::ShootDown(2.0, 2.5, 2),
                    hitbox: Hitbox {
                        x: (random::u32() % screen_w).saturating_sub(16) as f32,
                        y: -8.0,
                        w: 16,
                        h: 16,
                    },
                    hp: 8,
                    points: 30,
                    speed: 0.5,
                    angle: 0.0,
                    destroyed: false,
                    hit_timer: 0,
                }
            },
            EnemyType::Zipper => {
                Self {
                    enemy_type: EnemyType::Zipper,
                    strategy: EnemyStrategy::RandomZigZag(1.0),
                    hitbox: Hitbox {
                        x: (random::u32() % screen_w).saturating_sub(16) as f32,
                        y: -16.0,
                        w: 16,
                        h: 16,
                    },
                    hp: 4,
                    points: 20,
                    speed: 0.5,
                    angle: 0.0,
                    destroyed: false,
                    hit_timer: 0,
                }
            },
            EnemyType::Meteor => {
                Self {
                    enemy_type: EnemyType::Meteor,
                    strategy: EnemyStrategy::MoveDown,
                    hitbox: Hitbox {
                        x: (random::u32() % screen_w).saturating_sub(8) as f32,
                        y: -8.0,
                        w: 8,
                        h: 8,
                    },
                    hp: 3,
                    points: 20,
                    speed: 1.0,
                    angle: 0.0,
                    destroyed: false,
                    hit_timer: 0,
                }
            },
        }
    }

    // update is called once per frame within the [turbo::game] loop
    pub fn update(&mut self, player: &mut Player, projectiles: &mut Vec<Projectile>){
        let (screen_w, screen_h) = resolution();

        // Logic for different enemy strategies
        match self.strategy {
            EnemyStrategy::TargetPlayer(intensity, speed, size) => {
                self.hitbox.y += self.speed;
                // Logic for attacking with specified intensity
                if random::u32() % (250 / intensity as u32) == 0 {
                    // Calculate angle from self to player
                    let angle = ((player.hitbox.y - self.hitbox.y).atan2(player.hitbox.x - self.hitbox.x)
                        * 180.0)
                        / std::f32::consts::PI;

                    // Create and shoot projectiles from enemy towards the player
                    projectiles.push(
                        Projectile::new(
                            self.hitbox.x + (self.hitbox.w as f32 * 0.5) - (size as f32 * 0.5),
                            self.hitbox.y + (self.hitbox.h as f32),
                            speed,
                            angle,
                            // damage: intensity as u32, // Damage based on attack intensity
                            ProjectileType::Laser, // Assuming enemy uses Laser
                            ProjectileOwner::Enemy,
                        )
                    );
                }
            }
            EnemyStrategy::ShootDown(intensity, speed, size) => {
                // Logic for attacking with specified intensity
                self.hitbox.y += self.speed;
                if random::u32() % (250 / intensity as u32) == 0 {
                    // Create and shoot projectiles from enemy towards the player
                    projectiles.push(Projectile::new(
                        self.hitbox.x + (self.hitbox.w as f32 * 0.5) - (size as f32 * 0.5),
                        self.hitbox.y + (self.hitbox.h as f32),
                        speed,
                        90.0,
                        ProjectileType::Laser,
                        ProjectileOwner::Enemy,
                    ));
                }
            }
            EnemyStrategy::MoveDown => {
                self.hitbox.y += self.speed;
            }
            EnemyStrategy::RandomZigZag(angle) => {
                // Logic for dodging attacks, using angle to determine movement
                self.hitbox.x += self.speed * self.angle.cos();
                self.hitbox.y += self.speed;
                // Reverse direction when heading out of bounds
                if self.hitbox.x < 0.0 || self.hitbox.x > screen_w as f32 {
                    self.angle = std::f32::consts::PI - self.angle;
                }
                // 5% chance to randomly change angle
                else if random::u32() % 20 == 0 {
                    self.angle += std::f32::consts::PI / angle; // Change angle
                }
            }
        }

        if self.hitbox.y > (screen_h + self.hitbox.h) as f32 {
            self.destroyed = true;
        }

        self.hit_timer = self.hit_timer.saturating_sub(1);
    }

    pub fn take_damage(&mut self, player: &mut Player, damage: u32) {
        self.hp = self.hp.saturating_sub(damage);
        self.hit_timer = 5; // frames to show hit effect
        if self.hp == 0 {
            self.destroyed = true;
            player.score += self.points;
        }
    }

    pub fn draw(&self) {
        let mut sprite = match self.enemy_type {
            EnemyType::Tank => "enemies/tank",
            EnemyType::Shooter => "enemies/shooter",
            EnemyType::Turret => "enemies/turret",
            EnemyType::Zipper => "enemies/zipper",
            EnemyType::Meteor => "enemies/meteor",
        };

        if self.hit_timer > 0 && (self.hit_timer / 4) % 2 == 0 {
            sprite!(
                &sprite,
                x = self.hitbox.x,
                y = self.hitbox.y,
                color = 0xff0000ff,
            );
        } else {
            sprite!(
                &sprite,
                x = self.hitbox.x,
                y = self.hitbox.y,
            );
        }
    }
}

