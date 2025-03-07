use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 256;
const HEIGHT: usize = 240;

pub struct FramebufferViewer {
    window: Window,
    buffer: [u32; WIDTH * HEIGHT], // Pre-allocated framebuffer
}

impl FramebufferViewer {
    pub fn new(title: &str) -> Self {
        let window = Window::new(
            title,
            WIDTH,
            HEIGHT,
            WindowOptions {
                scale: minifb::Scale::X2, // Scale up for better visibility
                resize: false,
                ..WindowOptions::default()
            },
        )
        .expect("Failed to create window");

        Self { 
            window, 
            buffer: [0; WIDTH * HEIGHT], }
    }

    pub fn is_open(&self) -> bool {
        return self.window.is_open()
    }

    const NES_PALETTE: [u32; 64] = [
        0x666666, 0x002A88, 0x1412A7, 0x3B00A4, 0x5C007E, 0x6E0040, 0x6C0700, 0x561D00,
        0x333500, 0x0B4800, 0x005200, 0x004F08, 0x00404D, 0x000000, 0x000000, 0x000000,
        0xADADAD, 0x155FD9, 0x4240FF, 0x7527FE, 0xA01ACC, 0xB71E7B, 0xB53120, 0x994E00,
        0x6B6D00, 0x388700, 0x0F9300, 0x009220, 0x008493, 0x000000, 0x000000, 0x000000,
        0xFFFFFF, 0x64B0FF, 0x9290FF, 0xC676FF, 0xF36AFF, 0xFE6ECC, 0xFE8170, 0xEA9E22,
        0xBCBE00, 0x88D800, 0x5CE430, 0x45E082, 0x48CDDE, 0x4F4F4F, 0x000000, 0x000000,
        0xFFFFFF, 0xC0E7FF, 0xD3D2FF, 0xE8C8FF, 0xFBC2FF, 0xFDC4EA, 0xFDBDAF, 0xE8D890,
        0xD4E675, 0xB7F070, 0x9DF09F, 0x99F1CC, 0xA0ECF0, 0xA4A4A4, 0x000000, 0x000000,
    ];

    fn convert_to_rgb(color: u8) -> u32 {
        let index = (color as usize) % 64; // Ensure it's within range
        0xFF000000 | Self::NES_PALETTE[index] // Add alpha channel
    }


    pub fn update(&mut self, framebuffer: &[u8; WIDTH * HEIGHT]) {
        // Convert NES framebuffer (grayscale 0-3) to RGB u32 in-place
        for (i, &color) in framebuffer.iter().enumerate() {
            self.buffer[i] = Self::convert_to_rgb(color);
        }

        // Draw the framebuffer if the window is open
        if self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.window.update_with_buffer(&self.buffer, WIDTH, HEIGHT).unwrap();
        }
    }

}
