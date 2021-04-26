mod buttons;
mod easyopc;
mod lightstrip;
mod music;

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

trait ModeTrait {
    fn setup(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
    fn teardown(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn red_button(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
    fn left_blue_button(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
    fn right_blue_botton(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
    fn green_button(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}

struct IdleMode {}

impl IdleMode {
    fn create() -> anyhow::Result<IdleMode> {
        Ok(IdleMode {})
    }
}

impl ModeTrait for IdleMode {}

struct MusicMode {
    client: music::MusicClient,
    playlists: Vec<String>,
}

impl MusicMode {
    fn create() -> anyhow::Result<MusicMode> {
        let mut client = music::new_client().expect("Error creating music client");

        let playlists = client
            .playlists()
            .unwrap()
            .iter()
            .map(|playlist| playlist.name.clone())
            .collect();
        // else: error handling?

        Ok(MusicMode { client, playlists })
    }
}

impl ModeTrait for MusicMode {
    fn setup(&mut self) -> anyhow::Result<()> {
        self.client.play_playlist(self.playlists[0].clone())?;
        Ok(())
    }

    fn teardown(&mut self) -> anyhow::Result<()> {
        self.client.stop()
    }

    fn right_blue_botton(&mut self) -> anyhow::Result<()> {
        self.client.next()
    }

    fn left_blue_button(&mut self) -> anyhow::Result<()> {
        self.client.prev()
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
