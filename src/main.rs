mod buttons;
mod easyopc;
mod lightstrip;
mod modes;
mod music;

use modes::idle::IdleMode;
use modes::music::MusicMode;
use modes::ModeTrait;

struct CurrentStatus {
    mode: Box<dyn ModeTrait>,
}

impl CurrentStatus {
    fn set_mode(&mut self, mode: impl ModeTrait + 'static, led: u64) -> anyhow::Result<()> {
        buttons::set_led(buttons::MODE_BUTTON_GREEN_LED, 0)?;
        buttons::set_led(buttons::MODE_BUTTON_BLUE_LED, 0)?;
        buttons::set_led(buttons::MODE_BUTTON_RED_LED, 0)?;

        self.mode.teardown()?;

        self.mode = Box::new(mode);
        self.mode.setup()?;

        buttons::set_led(led, 1)?;

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let mut current = CurrentStatus {
        mode: Box::new(IdleMode {}),
    };

    let (sender, receiver) = std::sync::mpsc::channel();

    let threads = buttons::setup(&sender)?;

    for event in receiver.iter() {
        println!("BUTTON: {}", event);

        match event {
            buttons::MODE_BUTTON_GREEN => {
                current.set_mode(MusicMode::create()?, buttons::MODE_BUTTON_GREEN_LED)?
            }
            buttons::MODE_BUTTON_RED => {
                current.set_mode(IdleMode::create()?, buttons::MODE_BUTTON_RED_LED)?
            }
            buttons::MODE_BUTTON_BLUE => {
                current.set_mode(IdleMode::create()?, buttons::MODE_BUTTON_BLUE_LED)?
            }

            buttons::RED_BUTTON => current.mode.red_button()?,
            buttons::RIGHT_BLUE_BUTTON => current.mode.right_blue_botton()?,
            buttons::LEFT_BLUE_BUTTON => current.mode.left_blue_button()?,
            buttons::GREEN_BUTTON => current.mode.green_button()?,

            _ => {}
        }
    }

    for child in threads {
        child.join().expect("oops! a child thread panicked")?
    }

    Ok(())
}
