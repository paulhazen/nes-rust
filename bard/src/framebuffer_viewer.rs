use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 256;
const HEIGHT: usize = 240;

pub struct FramebufferViewer {
    window: Window,
}

impl FramebufferViewer {
    pub fn new() -> Self {
        let window = Window::new(
            "NES Framebuffer Viewer",
            WIDTH,
            HEIGHT,
            WindowOptions {
                scale: minifb::Scale::X2, // Scale up for better visibility
                resize: false,
                ..WindowOptions::default()
            },
        )
        .expect("Failed to create window");

        Self { window }
    }

    pub fn is_open(&self) -> bool {
        return self.window.is_open()
    }

    pub fn update(&mut self, framebuffer: &[u8; WIDTH * HEIGHT]) {
        // Convert NES framebuffer (grayscale 0-3) to RGB u32
        let buffer: Vec<u32> = framebuffer.iter().map(|&color| Self::convert_to_rgb(color)).collect();

        // Draw the framebuffer if the window is open
        if self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        }
    }

    fn convert_to_rgb(color: u8) -> u32 {
        match color {
            0 => 0xFF_000000, // Black
            1 => 0xFF_555555, // Dark Gray
            2 => 0xFF_AAAAAA, // Light Gray
            3 => 0xFF_FFFFFF, // White
            _ => 0xFF_000000, // Default to black
        }
    }
}
