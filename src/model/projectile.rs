use super::*;

// TODO: add different behaviors for different projectile types
#[turbo::serialize]
#[derive(PartialEq)]
pub enum ProjectileType {
    Basic,
    Splatter,
    Fragment,
    Laser,
    Bomb,
}
// Enum to determine who fired the projectile
#[turbo::serialize]
#[derive(PartialEq)]
pub enum ProjectileOwner {
    Enemy,
    Player,
}

#[turbo::serialize]
pub struct Projectile {
    pub hitbox: Hitbox,
    anim_key: String, // unique, randomly generated key to be used for SpriteAnimations

    pub collided: bool, // Used to control the sprite and update state
    pub destroyed: bool, // Used to remove projectile from game

    pub velocity: f32,
    angle: f32,
    pub damage: u32,
    pub projectile_owner: ProjectileOwner,
    pub projectile_type: ProjectileType,
}

impl Projectile {
    pub fn new(x: f32, y: f32, velocity: f32, angle: f32, projectile_type: ProjectileType, projectile_owner: ProjectileOwner) -> Self {
        let audio = match projectile_owner {
            ProjectileOwner::Enemy => "projectile_enemy_shoot",
            ProjectileOwner::Player => "projectile_player_shoot",
        };
        audio::play(audio);
        Projectile {
            // Initialize all fields with default values
            hitbox: Hitbox {
                x,
                y,
                w: 6,
                h: 6,
            },
            anim_key: random::u32().to_string(),
            destroyed: false,
            collided: false,
            velocity,
            angle,
            damage: 1,
            projectile_type,
            projectile_owner,
        }
    }
    // update is called once per frame within the [turbo::game] loop
    pub fn update(&mut self, player: &mut Player, enemies: &mut Vec<Enemy>) {
        let (screen_w, screen_h) = resolution();

        // If the projectile hasn't collided, update it as normal
        if !self.collided {
            // update projectile position
            let radian_angle = self.angle.to_radians();
            self.hitbox.x += self.velocity * radian_angle.cos();
            self.hitbox.y += self.velocity * radian_angle.sin();

            // flag the projectile to be destroyed if it goes off screen
            if self.hitbox.y < -(self.hitbox.h as f32)
            && self.hitbox.x < -(self.hitbox.w as f32)
            && self.hitbox.x > screen_w as f32
            && self.hitbox.y > screen_h as f32
            {
                self.destroyed = true;
            }

            // Checking for collisions with player or enemies based on projectile owner
            match self.projectile_owner {
                // Check collision with player
                ProjectileOwner::Enemy => {
                    if check_collision(&self.hitbox, &player.hitbox) 
                    && player.hp > 0
                    && player.hit_timer == 0 {
                        player.take_damage(self.damage);
                        audio::play("projectile_hit");
                        self.collided = true;
                    }
                }
                // Check collision with enemies
                ProjectileOwner::Player => {
                    for enemy in enemies.iter_mut() {
                        if check_collision(&self.hitbox, &enemy.hitbox) && !enemy.destroyed {
                            enemy.take_damage(player, self.damage);
                            
                            audio::play("projectile_hit");
                            self.collided = true;
                            break; // Exit loop after first collision
                        }
                    }
                }
            }

        // if the projectile has collided, 
        } else {
            // get reference to the SpriteAnimation of the projectile
            let anim = animation::get(&self.anim_key);
            // flag projectile as destroyed when the hit animation is done
            if anim.done() {
                self.destroyed = true;
            }
        }
    }

    pub fn draw(&self) {
        let owner = match self.projectile_owner {
            ProjectileOwner::Enemy => "enemy",
            ProjectileOwner::Player => "player",
        };
        let anim = animation::get(&self.anim_key);
        if !self.collided {
            anim.use_sprite(&format!("projectiles/projectile_{}", owner));
        } else {
            anim.use_sprite(&format!("projectiles/projectile_{}_hit", owner));
            anim.set_repeat(0);
            anim.set_fill_forwards(true);
        }
        
        sprite!(animation_key = &self.anim_key, x = self.hitbox.x as i32, y = self.hitbox.y as i32);
    }
}