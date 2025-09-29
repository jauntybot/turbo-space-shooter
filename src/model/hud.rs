use super::*;

#[turbo::serialize]
pub struct HUD {
    notification_timer: u32,
}
impl HUD {
    pub fn new() -> Self {
        HUD {
            notification_timer: 0,
        }
    }
    // update is called once per frame within the [turbo::game] loop
    pub fn update(&mut self, player: &mut Player) {
        // Notifications timer
        if player.notifications.len() > 0 {
            self.notification_timer += 1;
            // Remove current notification if timer expires
            if self.notification_timer >= 120 - 1 {
                self.notification_timer = 0;
                let _ = player.notifications.remove(0);
            }
        }
    }

    pub fn draw(&self, player: &Player) {
        let (screen_w, _) = resolution();
        // Drawing the HUD panel
        let hud_height = 16; // Height of the HUD panel
        rect!(
            x = 0,
            y = 0,
            w = screen_w,
            h = hud_height,
            color = 0x000000ff
        ); // Black background for the HUD

        // Drawing borders for the HUD section
        rect!(
            x = 0,
            y = hud_height as i32,
            w = screen_w,
            h = 1,
            // border = 1,
            color = 0xffffffff
        ); // White border

        // Displaying game information on the HUD
        let hud_padding = 4; // Padding inside the HUD
        let text_color = 0xffffffff; // White text color

        // Display Health
        let health_text = format!("HP: {}", player.hp);
        let health_text_x = 8;
        text!(
            &health_text,
            x = health_text_x,
            y = hud_padding,
            font = "large",
            color = text_color
        );

        // Display Score
        let score_text = format!("SCORE: {:0>5}", player.score);
        let score_text_x =
            screen_w as i32 - (score_text.chars().count() as i32 * 8) - hud_padding;
        text!(
            &score_text,
            x = score_text_x,
            y = hud_padding,
            font = "large",
            color = text_color
        )
    }

    pub fn draw_notifications(&self, player: &Player) {
        let (screen_w, _) = resolution();
        // Render notifications
        for notif in player.notifications.iter() {
            let len = notif.chars().count();
            let w = len * 5;
            let x = (screen_w as usize / 2) - (w / 2); // center the text based on width
            rect!(
                w = w as u32 + 4,
                h = 10,
                x = x as i32 - 2,
                y = 24 - 2,
                color = 0x22aaaaff
            );
            text!(
                &notif,
                x = x as i32,
                y = 24,
                font = "medium",
                color = 0xffffffff
            );
            break;
        }
    }

    pub fn draw_menu(&self, tick: u32,) {
        let (screen_w, screen_h) = resolution();

        text!(
            "SPACE SHOOTER",
            x = (screen_w as i32 / 2) - 48,
            y = (screen_h as i32 / 2) - 16,
            font = "large"
        );
        // blink start message
        if tick / 4 % 8 < 4 {
            text!(
                "Press START or A to begin",
                x = (screen_w as i32 / 2) - 64,
                y = (screen_h as i32 / 2) + 8,
                font = "medium"
            );
        }
    }

    pub fn draw_game_over(&self, tick: u32,) {
        let (screen_w, screen_h) = resolution();

        text!(
            "GAME OVER",
            x = (screen_w as i32 / 2) - 32,
            y = (screen_h as i32 / 2) - 4,
            font = "large"
        );
        // blink restart message
        if tick / 4 % 8 < 4 {
            text!(
                "PRESS START",
                x = (screen_w as i32 / 2) - 24,
                y = (screen_h as i32 / 2) - 4 + 16,
                font = "medium"
            );
        }
    }
}