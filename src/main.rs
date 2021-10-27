mod buttons;
mod lightstrip;
mod modes;
mod music;

#[macro_use]
extern crate lazy_static;

use modes::color::ColorMode;
use modes::idle::IdleMode;
use modes::music::MusicMode;
use modes::ModeTrait;

struct CurrentStatus {
    mode: Box<dyn ModeTrait>,
    current_led: Option<u64>,
}

impl CurrentStatus {
    fn set_mode(&mut self, mode: impl ModeTrait + 'static, led: Option<u64>) -> anyhow::Result<()> {
        buttons::set_led(buttons::MODE_BUTTON_GREEN_LED, 0)?;
        buttons::set_led(buttons::MODE_BUTTON_BLUE_LED, 0)?;
        buttons::set_led(buttons::MODE_BUTTON_RED_LED, 0)?;

        self.mode.teardown()?;

        if self.current_led != led {
            self.current_led = led;

            if let Some(led) = led {
                buttons::set_led(led, 1)?;

                self.mode = Box::new(mode);
                self.mode.setup()?;
            }
        } else {
            self.current_led = None;
            self.mode = Box::new(IdleMode::create()?);
            self.mode.setup()?;
        }

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    println!("Starting");
    let mut current = CurrentStatus {
        mode: Box::new(IdleMode {}),
        current_led: None,
    };

    let (sender, receiver) = std::sync::mpsc::channel();

    let mut threads = buttons::setup(&sender)?;

    current.set_mode(IdleMode::create()?, None)?;

    let (lightstrip_sender, thread) = lightstrip::setup()?;
    threads.push(thread);

    lightstrip::chase(
        &lightstrip_sender,
        lightstrip::Pixel {
            r: 255,
            g: 255,
            b: 255,
        },
    )?;

    println!("Starting event loop");
    for event in receiver.iter() {
        println!("BUTTON: {}", event);
        if let Err(error) = handle_event(event, &mut current, &lightstrip_sender) {
            println!("Error handling event {:?}", error);
        }
    }

    for child in threads {
        child.join().expect("oops! a child thread panicked")?
    }

    Ok(())
}

fn handle_event(
    event: u64,
    current: &mut CurrentStatus,
    lightstrip_sender: &std::sync::mpsc::Sender<lightstrip::Message>,
) -> anyhow::Result<()> {
    match event {
        buttons::MODE_BUTTON_GREEN => current.set_mode(
            MusicMode::create(&lightstrip_sender)?,
            Some(buttons::MODE_BUTTON_GREEN_LED),
        )?,
        buttons::MODE_BUTTON_RED => {
            current.set_mode(IdleMode::create()?, Some(buttons::MODE_BUTTON_RED_LED))?
        }
        buttons::MODE_BUTTON_BLUE => current.set_mode(
            ColorMode::create(&lightstrip_sender)?,
            Some(buttons::MODE_BUTTON_BLUE_LED),
        )?,

        buttons::RED_BUTTON => current.mode.red_button()?,
        buttons::RIGHT_BLUE_BUTTON => current.mode.right_blue_botton()?,
        buttons::LEFT_BLUE_BUTTON => current.mode.left_blue_button()?,
        buttons::GREEN_BUTTON => current.mode.green_button()?,

        _ => {}
    };

    Ok(())
}
