use super::*;


#[turbo::serialize]
pub enum PowerupEffect {
    Heal,                        // Heals the player when interacted with
    MaxHealthUp,                 // Increases max health
    SpeedBoost,                  // Temporarily increases player's speed
    DamageBoost,                 // Temporarily increases projectile's damage
    RateOfFireBoost,
    ProjectileSpeedBoost,
}

#[turbo::serialize]
pub enum PowerupMovement {
    Static,
    FloatVertical(f32), // Vertical floating speed
    FloatHorizontal(f32), // Horizontal drifting speed
}

#[turbo::serialize]
pub struct Powerup {
    pub hitbox: Hitbox,
    pub effect: PowerupEffect,
    movement: PowerupMovement,
}
impl Powerup {
    pub fn new(x: f32, y: f32, effect: PowerupEffect, movement: PowerupMovement) -> Self {
        Powerup {
            hitbox: Hitbox { x, y, w: 8, h: 8 },
            effect,
            movement,
        }
    }

    pub fn new_random(x: f32, y: f32, player: &Player) -> Self {
        // Create a list of possible effects based on player's current stats
        let mut effects = vec![
            PowerupEffect::SpeedBoost,
        ];
        // Add max HP if the player has less than 5 max HP
        if player.stats.max_hp < 5 {
            effects.push(PowerupEffect::MaxHealthUp);
        }
        // Add other effects based on score thresholds
        if player.score > 400 {
            effects.push(PowerupEffect::RateOfFireBoost);
        }
        if player.score > 800 {
            effects.push(PowerupEffect::ProjectileSpeedBoost);
        }
        if player.score > 1500 && player.stats.damage < 3 {
            effects.push(PowerupEffect::DamageBoost);
        }
        // Select a random effect from the constructed list
        let effect = effects[(random::u32() as usize) % effects.len()].clone();
        // Randomly choose a movement type between the 3 options
        let movement = match random::u32() % 3 {
            0 => PowerupMovement::Static,
            1 => PowerupMovement::FloatVertical(0.5 + (random::u32() % 100) as f32 * 0.01),
            2 => PowerupMovement::FloatHorizontal(0.5 + (random::u32() % 100) as f32 * 0.01),
            _ => unreachable!(),
        };
        Powerup {
            hitbox: Hitbox { x, y, w: 8, h: 8 },
            effect,
            movement,
        }
    }
    // update is called once per frame within the [turbo::game] loop
    pub fn update(&mut self) {
        let (screen_w, screen_h) = resolution();
        // update position based on movement type
        match self.movement {
            PowerupMovement::FloatVertical(speed) => {
                self.hitbox.y += speed;
                // reverse the direction if it reaches the screen bounds
                if self.hitbox.y <= 0.0 || self.hitbox.y >= screen_h as f32 {
                    self.movement = PowerupMovement::FloatVertical(-speed);
                }
            }
            PowerupMovement::FloatHorizontal(speed) => {
                self.hitbox.x += speed;
                // reverse the direction if it reaches the screen bounds
                if self.hitbox.x <= 0.0 || self.hitbox.x >= screen_w as f32 {
                    self.movement = PowerupMovement::FloatHorizontal(-speed);
                }
            }
            PowerupMovement::Static => {
                // Static powerups do not move
            }
        }
    }

    pub fn draw(&self, tick: u32) {
        // define the string for which sprite to use
        let sprite = match self.effect {
            PowerupEffect::Heal => "power_ups/powerup_heal",
            PowerupEffect::MaxHealthUp => "power_ups/powerup_max_hp",
            PowerupEffect::DamageBoost => "power_ups/powerup_damage",
            PowerupEffect::SpeedBoost => "power_ups/powerup_speed",
            PowerupEffect::RateOfFireBoost => "power_ups/powerup_rate_of_fire",
            PowerupEffect::ProjectileSpeedBoost => "power_ups/powerup_projectile_speed",
        };
        // Small oscillating value for bobbing effect
        let n = (tick as f32 * 0.15).cos() * 3.0;
        // Add the offset n based on movement type
        let xy = match self.movement {
            PowerupMovement::FloatVertical(_) => (self.hitbox.x + n, self.hitbox.y),
            PowerupMovement::FloatHorizontal(_) => (self.hitbox.x, self.hitbox.y + n),
            PowerupMovement::Static => (self.hitbox.x, self.hitbox.y + n),
        }; 
        // draw sprite
        sprite!(
            &sprite, 
            xy = xy, 
        );
    }
}