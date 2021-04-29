use super::ModeTrait;
use crate::lightstrip;

use lightstrip::Pixel;

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
}

impl ColorMode {
    pub fn create(lightstrip: &lightstrip::Sender) -> anyhow::Result<ColorMode> {
        let lightstrip = lightstrip.clone();
        let current_color = 0;
        Ok(ColorMode {
            lightstrip,
            current_color,
        })
    }

    fn chase(&mut self, color: Pixel) -> anyhow::Result<()> {
        lightstrip::chase(&self.lightstrip, color).or_else(|err| {
            println!("Could not start chase {}", err);
            Ok(())
        })
    }

    fn unchase(&mut self, color: Pixel) -> anyhow::Result<()> {
        lightstrip::unchase(&self.lightstrip, color).or_else(|err| {
            println!("Could not start chase {}", err);
            Ok(())
        })
    }
}

impl ModeTrait for ColorMode {
    fn teardown(&mut self) -> anyhow::Result<()> {
        self.unchase(Pixel::default())
    }

    fn right_blue_botton(&mut self) -> anyhow::Result<()> {
        self.current_color = (self.current_color + 1) % COLORS.len();
        self.unchase(COLORS[self.current_color])
    }

    fn left_blue_button(&mut self) -> anyhow::Result<()> {
        self.current_color = self
            .current_color
            .checked_sub(1)
            .unwrap_or(COLORS.len() - 1);

        self.unchase(COLORS[self.current_color])
    }

    fn red_button(&mut self) -> anyhow::Result<()> {
        self.chase(Pixel { r: 255, g: 0, b: 0 })
    }

    fn green_button(&mut self) -> anyhow::Result<()> {
        self.chase(Pixel { r: 0, g: 255, b: 0 })
    }
}
