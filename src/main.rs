mod buttons;
mod music;

enum Mode {
    Idle,
    Playing,
}

struct CurrentStatus {
    mode: Mode,
}

impl CurrentStatus {
    fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;

        buttons::set_led(buttons::MODE_BUTTON_GREEN_LED, 0);
        buttons::set_led(buttons::MODE_BUTTON_BLUE_LED, 0);
        buttons::set_led(buttons::MODE_BUTTON_RED_LED, 0);

        let led = match self.mode {
            Mode::Idle => buttons::MODE_BUTTON_RED_LED,
            Mode::Playing => buttons::MODE_BUTTON_BLUE_LED,
        };

        buttons::set_led(led, 1);
    }
}

fn main() -> anyhow::Result<()> {
    let mut current = CurrentStatus { mode: Mode::Idle };

    let (sender, receiver) = std::sync::mpsc::channel();
    let mut music_client = music::new_client().expect("Error creating music client");

    let threads = buttons::setup(&sender)?;

    if let Ok(playlists) = music_client.playlists() {
        for playlist in playlists {
            println!("Got {:?}", playlist.name);
        }
    }

    for event in receiver.iter() {
        println!("BUTTON: {}", event);

        match event {
            buttons::MODE_BUTTON_BLUE => {
                current.set_mode(Mode::Playing);
                music_client.play_playlist("sticks".to_string())?
            }

            buttons::MODE_BUTTON_GREEN => {
                current.set_mode(Mode::Playing);

                music_client.play_playlist("flute".to_string())?
            }

            buttons::MODE_BUTTON_RED => {
                current.set_mode(Mode::Idle);

                music_client.stop()?;
            }

            _ => {
                println!("Status: {:?}", music_client.status());
            }
        }
    }

    for child in threads {
        child.join().expect("oops! a child thread panicked")?
    }

    Ok(())
}
