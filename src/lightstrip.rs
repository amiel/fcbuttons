use bytes::BytesMut;
use opc::OpcCodec;
use std::default::Default;
use std::env;
use std::io::Write;
use std::net::{Shutdown, TcpStream};
use std::str::FromStr;
use std::sync::mpsc;
use std::thread;
use tokio_io::codec::Encoder;

pub struct PixelControl {
    pub stream: TcpStream,
    codec: OpcCodec,
}

#[derive(Clone, Debug, Copy, Default)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Message {
    pub color: Pixel,
    pub fade: std::time::Duration,
    pub delay: std::time::Duration,
}

pub type Sender = mpsc::Sender<Message>;
pub type Receiver = mpsc::Receiver<Message>;

pub fn setup() -> anyhow::Result<(Sender, thread::JoinHandle<anyhow::Result<()>>)> {
    let mut opc = init()?;

    let (sender, receiver): (Sender, Receiver) = mpsc::channel();
    let clear_pixels = vec![Pixel::default(); 64];

    let join_handle = thread::spawn(move || {
        for message in receiver.iter() {
            opc.emit(&clear_pixels).unwrap();
            opc.emit(&clear_pixels).unwrap();

            let pixels = vec![message.color.clone(); 64];
            opc.emit(&pixels).unwrap();

            thread::sleep(message.delay);

            opc.emit(&clear_pixels).unwrap();

            thread::sleep(message.fade);
        }

        Ok(())
    });

    Ok((sender, join_handle))
}

pub fn flash(sender: &Sender, color: Pixel) -> Result<(), mpsc::SendError<Message>> {
    sender.send(Message {
        delay: std::time::Duration::from_secs(1),
        fade: std::time::Duration::from_secs(1),
        color: color,
    })
}

fn init() -> anyhow::Result<PixelControl> {
    let endpoint = env::var("OPC_ENDPOINT").unwrap_or(String::from("127.0.0.1:7890"));
    let stream = TcpStream::connect(endpoint)?;
    stream.shutdown(Shutdown::Read)?; // Not a great listener...
    stream.set_nodelay(true)?;
    let codec = OpcCodec {};
    let opc = PixelControl { stream, codec };

    Ok(opc)
}

impl FromStr for Pixel {
    type Err = std::num::ParseIntError;
    // Parses a color hex code of the form '#rRgGbB..' into an
    // instance of 'RGB'
    fn from_str(hex_code: &str) -> Result<Self, Self::Err> {
        // u8::from_str_radix(src: &str, radix: u32) converts a string
        // slice in a given base to u8
        let r: u8 = u8::from_str_radix(&hex_code[1..3], 16)?;
        let g: u8 = u8::from_str_radix(&hex_code[3..5], 16)?;
        let b: u8 = u8::from_str_radix(&hex_code[5..7], 16)?;

        Ok(Pixel { r, g, b })
    }
}

impl PixelControl {
    pub fn emit(&mut self, pixels: &[Pixel]) -> std::io::Result<()> {
        let mut buffer = BytesMut::new();

        // : [[u8; 3]; 512]
        let pixels: Vec<[u8; 3]> = pixels
            .iter()
            .map(|pixel| [pixel.r, pixel.g, pixel.b])
            .collect();
        let message = opc::Message::from_pixels(0, &pixels);

        self.codec.encode(message, &mut buffer)?;

        self.stream.write_all(&buffer)?;

        Ok(())
    }
}
