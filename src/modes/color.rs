use super::ModeTrait;
use crate::lightstrip;

use anyhow::anyhow;

use lightstrip::Pixel;

// Gamma brightness lookup table <https://victornpb.github.io/gamma-table-generator>
// gamma = 2.00 steps = 8 range = 0-255
static GAMMA_LUT: [u8; 8] = [0, 5, 21, 47, 83, 130, 187, 255];

lazy_static! {
    static ref COLORS: Vec<Pixel> = vec![
        Pixel { r: 255, g: 0, b: 0 },
        Pixel {
            r: 255,
            g: 255,
            b: 0
        },
        Pixel { r: 0, g: 255, b: 0 },
        Pixel {
            r: 0,
            g: 255,
            b: 255
        },
        Pixel { r: 0, g: 0, b: 255 },
        Pixel {
            r: 255,
            g: 0,
            b: 255
        },
    ];
}

pub struct ColorMode {
    lightstrip: lightstrip::Sender,
    current_color: usize,
    intensity: usize,
}

impl ColorMode {
    pub fn create(lightstrip: &lightstrip::Sender) -> anyhow::Result<ColorMode> {
        let lightstrip = lightstrip.clone();
        let current_color = 0;
        let intensity = GAMMA_LUT.len() - 1;
        Ok(ColorMode {
            lightstrip,
            current_color,
            intensity,
        })
    }

    fn set(&mut self, color: Pixel) -> anyhow::Result<()> {
        lightstrip::set(&self.lightstrip, vec![color]).or_else(|err| {
            println!("Could not set color {}", err);
            Ok(())
        })
    }

    fn unchase(&mut self, color: Pixel) -> anyhow::Result<()> {
        lightstrip::unchase(&self.lightstrip, color).or_else(|err| {
            println!("Could not start chase {}", err);
            Ok(())
        })
    }

    fn intensify(&self, val: u8) -> anyhow::Result<u8> {
        let multiplier = GAMMA_LUT
            .get(self.intensity)
            .ok_or(anyhow!("No gamma multiplier for: {}", self.intensity))?;

        let result = (val as u16 * *multiplier as u16) / 255;

        Ok(result as u8)
    }

    fn update_lights_chase(&mut self) -> anyhow::Result<()> {
        let color = self.get_color()?;
        self.unchase(color)
    }

    fn update_lights(&mut self) -> anyhow::Result<()> {
        let color = self.get_color()?;
        self.set(color)
    }

    fn get_color(&self) -> anyhow::Result<Pixel> {
        let color = COLORS
            .get(self.current_color)
            .ok_or(anyhow!("No color for: {}", self.current_color))?;

        Ok(Pixel {
            r: self.intensify(color.r)?,
            g: self.intensify(color.g)?,
            b: self.intensify(color.b)?,
        })
    }
}

impl ModeTrait for ColorMode {
    fn setup(&mut self) -> anyhow::Result<()> {
        self.update_lights()
    }

    fn teardown(&mut self) -> anyhow::Result<()> {
        self.unchase(Pixel::default())
    }

    fn right_blue_botton(&mut self) -> anyhow::Result<()> {
        self.current_color = (self.current_color + 1) % COLORS.len();
        self.update_lights_chase()
    }

    fn left_blue_button(&mut self) -> anyhow::Result<()> {
        self.current_color = self
            .current_color
            .checked_sub(1)
            .unwrap_or(COLORS.len() - 1);

        self.update_lights_chase()
    }

    fn red_button(&mut self) -> anyhow::Result<()> {
        self.intensity = match self.intensity {
            0..=6 => self.intensity + 1,
            _ => self.intensity,
        };

        self.update_lights()
    }

    fn green_button(&mut self) -> anyhow::Result<()> {
        self.intensity = match self.intensity {
            0 => self.intensity,
            _ => self.intensity - 1,
        };

        self.update_lights()
    }
}
