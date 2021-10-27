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

const N_LIGHTS: usize = 64;

pub struct PixelControl {
    stream: TcpStream,
    codec: OpcCodec,
    buffer: [Pixel; N_LIGHTS],
}

lazy_static! {
    static ref OFF_PIXELS: Vec<Pixel> = vec![Pixel::default(); N_LIGHTS];
}

#[derive(Clone, Debug, Copy, Default)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub enum Message {
    ColorSet(ColorSet),
    Chase(Pixel),
    Unchase(Pixel),
}

pub struct ColorSet {
    colors: Vec<Pixel>,
}

pub type Sender = mpsc::Sender<Message>;
pub type Receiver = mpsc::Receiver<Message>;

pub fn setup() -> anyhow::Result<(Sender, thread::JoinHandle<anyhow::Result<()>>)> {
    let mut opc = init()?;

    let (sender, receiver): (Sender, Receiver) = mpsc::channel();

    let join_handle = thread::spawn(move || {
        for message in receiver.iter() {
            match message {
                Message::ColorSet(one_color_flash) => {
                    do_alternating_color_set(&mut opc, one_color_flash)
                }
                Message::Chase(color) => do_chase(&mut opc, color),
                Message::Unchase(color) => do_unchase(&mut opc, color),
            }
        }

        Ok(())
    });

    Ok((sender, join_handle))
}

pub fn set(sender: &Sender, colors: Vec<Pixel>) -> Result<(), mpsc::SendError<Message>> {
    sender.send(Message::ColorSet(ColorSet { colors: colors }))
}

pub fn chase(sender: &Sender, color: Pixel) -> Result<(), mpsc::SendError<Message>> {
    sender.send(Message::Chase(color))
}

pub fn unchase(sender: &Sender, color: Pixel) -> Result<(), mpsc::SendError<Message>> {
    sender.send(Message::Unchase(color))
}

pub fn unchase_reset(sender: &Sender) -> Result<(), mpsc::SendError<Message>> {
    unchase(sender, Pixel::default())
}

fn do_clear(opc: &mut PixelControl) {
    let off_pixels = &OFF_PIXELS;
    opc.emit(off_pixels).unwrap();
}

fn do_chase(opc: &mut PixelControl, color: Pixel) {
    let frame_rate = std::time::Duration::from_millis(10);

    // let join_handle = thread::spawn(|| {
    opc.buffer = [color; N_LIGHTS];
    opc.emit_buffer().unwrap();

    for i in 0..N_LIGHTS {
        thread::sleep(frame_rate);
        opc.buffer[i] = Pixel::default();
        opc.emit_buffer().unwrap();
    }
    // });
}

fn do_unchase(opc: &mut PixelControl, color: Pixel) {
    let frame_rate = std::time::Duration::from_millis(16);

    for i in 0..N_LIGHTS {
        thread::sleep(frame_rate);
        opc.buffer[i] = color;
        opc.emit_buffer().unwrap();
    }
}

fn do_alternating_color_set(opc: &mut PixelControl, message: ColorSet) {
    do_clear(opc);
    do_clear(opc);

    let n_colors = message.colors.len();
    let mut pixels = vec![Pixel::default(); N_LIGHTS];

    for i in 0..pixels.len() {
        pixels[i] = message.colors[i % n_colors].clone();
    }

    opc.emit(&pixels).unwrap();
}

fn init() -> anyhow::Result<PixelControl> {
    let endpoint = env::var("OPC_ENDPOINT").unwrap_or(String::from("127.0.0.1:7890"));
    let stream = TcpStream::connect(endpoint)?;
    stream.shutdown(Shutdown::Read)?; // Not a great listener...
    stream.set_nodelay(true)?;
    let codec = OpcCodec {};
    let buffer = [Pixel::default(); N_LIGHTS];
    let opc = PixelControl {
        stream,
        codec,
        buffer,
    };

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
    fn emit(&mut self, pixels: &[Pixel]) -> std::io::Result<()> {
        let mut buffer = BytesMut::new();

        let pixels: Vec<[u8; 3]> = pixels
            .iter()
            .map(|pixel| [pixel.r, pixel.g, pixel.b])
            .collect();
        let message = opc::Message::from_pixels(0, &pixels);

        self.codec.encode(message, &mut buffer)?;

        self.stream.write_all(&buffer)?;

        Ok(())
    }

    fn emit_buffer(&mut self) -> std::io::Result<()> {
        let buffer = self.buffer;

        self.emit(&buffer)
    }
}
