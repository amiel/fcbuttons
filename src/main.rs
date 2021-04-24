mod buttons;
mod music;

fn main() -> anyhow::Result<()> {
    let (sender, receiver) = std::sync::mpsc::channel();
    let mut music_client = music::new_client()?;

    let threads = buttons::setup(&sender)?;

    for event in receiver.iter() {
        println!("BUTTON: {}", event);

        match event {
            buttons::MODE_BUTTON_RED => {
                buttons::set_led(buttons::MODE_BUTTON_GREEN_LED, 0)?;
                buttons::set_led(buttons::MODE_BUTTON_RED_LED, 1)?;
                music_client.play_playlist("sticks".to_string())?
            }

            buttons::MODE_BUTTON_GREEN => {
                buttons::set_led(buttons::MODE_BUTTON_RED_LED, 0)?;
                buttons::set_led(buttons::MODE_BUTTON_GREEN_LED, 1)?;
                music_client.play_playlist("flute".to_string())?
            }

            buttons::MODE_BUTTON_BLUE => {
                music_client.stop()?;
            }

            _ => {
                println!("Status: {:?}", music_client.status());
            }
        }
    }

    for child in threads {
        child.join().expect("oops! the child thread panicked")?
    }

    Ok(())
}
