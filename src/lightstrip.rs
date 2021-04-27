use bytes::BytesMut;
use opc::OpcCodec;
use std::default::Default;
use std::env;
use std::io::Write;
use std::net::{Shutdown, TcpStream};
use std::str::FromStr;
use tokio_io::codec::Encoder;

pub struct PixelControl {
    pub stream: TcpStream,
    codec: OpcCodec,
}

#[derive(Clone, Debug, Copy)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

// TODO: thread!
pub fn full_flash_colors(colors: Vec<Pixel>) {
    let mut opc = PixelControl::default();
    let mut pixels = vec![Pixel { r: 0, g: 0, b: 0 }; 64];

    opc.emit(&pixels).unwrap();
    opc.emit(&pixels).unwrap();
    std::thread::sleep(std::time::Duration::from_secs(1));

    for color in colors {
        for i in 0..64 {
            pixels[i] = color;
        }

        opc.emit(&pixels).unwrap();

        std::thread::sleep(std::time::Duration::from_secs(2));
    }

    let pixels = vec![Pixel { r: 0, g: 0, b: 0 }; 64];
    opc.emit(&pixels).unwrap();
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

        self.codec.encode(message, &mut buffer);

        self.stream.write_all(&buffer);

        Ok(())
    }
}

impl Default for PixelControl {
    fn default() -> PixelControl {
        let endpoint = env::var("OPC_ENDPOINT").unwrap_or(String::from("127.0.0.1:7890"));
        let stream = TcpStream::connect(endpoint).unwrap();
        stream.shutdown(Shutdown::Read).unwrap(); // Not a great listener...
        stream.set_nodelay(true).unwrap();
        let codec = OpcCodec {};
        PixelControl { stream, codec }
    }
}
