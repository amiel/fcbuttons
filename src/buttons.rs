use sysfs_gpio::{Direction, Pin};

const LCDD4: u64 = 100;
const LCDD6: u64 = 102;
const LCDD10: u64 = 106;
const LCDD12: u64 = 108;
const LCDD14: u64 = 110;
const LCDD18: u64 = 114;
const LCDD20: u64 = 116;
const LCDD22: u64 = 118;
const LCDCLK: u64 = 120;
const LCDVSYNC: u64 = 123;

// const XIOP2: u64 = 408;
// const XIOP4: u64 = 410;
// const XIOP6: u64 = 412;
// const XIOP8: u64 = 414;

const XIOP2: u64 = 1013;
const XIOP4: u64 = 1015;
const XIOP6: u64 = 1017;
const XIOP8: u64 = 1019;

pub const MODE_BUTTON_RED: u64 = XIOP2;
pub const MODE_BUTTON_BLUE: u64 = XIOP6;
pub const MODE_BUTTON_GREEN: u64 = XIOP4;

pub const RED_BUTTON: u64 = LCDD4;
pub const RIGHT_BLUE_BUTTON: u64 = XIOP8;
pub const LEFT_BLUE_BUTTON: u64 = LCDD14;
pub const GREEN_BUTTON: u64 = LCDD10;

pub const MODE_BUTTON_GREEN_LED: u64 = LCDD20;
pub const MODE_BUTTON_RED_LED: u64 = LCDCLK;
pub const MODE_BUTTON_BLUE_LED: u64 = LCDVSYNC;

pub const RED_BUTTON_LED: u64 = LCDD6;
pub const RIGHT_BLUE_BUTTON_LED: u64 = LCDD12;
pub const LEFT_BLUE_BUTTON_LED: u64 = LCDD18;
pub const GREEN_BUTTON_LED: u64 = LCDD22;

pub fn set_led(pin: u64, value: u8) -> anyhow::Result<()> {
    let my_led = Pin::new(pin);
    Ok(my_led.set_value(value)?)
}

pub fn setup(
    sender: &std::sync::mpsc::Sender<u64>,
) -> anyhow::Result<Vec<std::thread::JoinHandle<anyhow::Result<()>>>> {
    let mut threads = vec![];

    threads.push(interrupt(MODE_BUTTON_RED, &sender));
    threads.push(interrupt(MODE_BUTTON_BLUE, &sender));
    threads.push(interrupt(MODE_BUTTON_GREEN, &sender));

    threads.push(interrupt(RED_BUTTON, &sender));
    threads.push(interrupt(RIGHT_BLUE_BUTTON, &sender));
    threads.push(interrupt(LEFT_BLUE_BUTTON, &sender));
    threads.push(interrupt(GREEN_BUTTON, &sender));

    setup_led(MODE_BUTTON_RED_LED)?;
    setup_led(MODE_BUTTON_GREEN_LED)?;
    setup_led(MODE_BUTTON_BLUE_LED)?;

    setup_led(RED_BUTTON_LED)?;
    setup_led(LEFT_BLUE_BUTTON_LED)?;
    setup_led(RIGHT_BLUE_BUTTON_LED)?;
    setup_led(GREEN_BUTTON_LED)?;

    Ok(threads)
}

fn interrupt(
    pin: u64,
    channel: &std::sync::mpsc::Sender<u64>,
) -> std::thread::JoinHandle<anyhow::Result<()>> {
    let tx = channel.clone();

    std::thread::spawn(move || {
        let input = Pin::new(pin);
        let result = input.with_exported(|| {
            input.set_direction(Direction::In).unwrap();

            let mut prev_val: u8 = 255;
            loop {
                let val = input.get_value()?;
                if val != prev_val {
                    if val == 0 {
                        tx.send(pin).unwrap()
                    }
                    prev_val = val;
                }
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        });

        match result {
            Ok(result) => Ok(result),
            Err(error) => Err(anyhow::anyhow!("Error in interrupt {}", error)),
        }
    })
}

fn setup_led(pin: u64) -> anyhow::Result<()> {
    let my_led = Pin::new(pin);
    my_led.export()?;
    Ok(my_led.set_direction(Direction::Out)?)
}
