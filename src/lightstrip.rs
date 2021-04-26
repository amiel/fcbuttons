use super::easyopc::*;

pub fn test() {
    let mut opc = PixelControl::default();
    let mut pixels = vec![Pixel { r: 0, g: 0, b: 0 }; 512];

    opc.emit(&pixels).unwrap();
    opc.emit(&pixels).unwrap();
    std::thread::sleep(std::time::Duration::from_secs(1));

    for i in 0..10 {
        pixels[i] = Pixel {
            r: 255,
            g: 0,
            b: 255,
        };
    }

    for i in 10..20 {
        pixels[i] = Pixel { r: 0, g: 255, b: 0 };
    }

    opc.emit(&pixels).unwrap();

    std::thread::sleep(std::time::Duration::from_secs(5));

    for i in 0..5 {
        pixels[i] = Pixel { r: 0, g: 0, b: 255 };
    }

    for i in 5..15 {
        pixels[i] = Pixel {
            r: 255,
            g: 255,
            b: 0,
        };
    }

    for i in 15..25 {
        pixels[i] = Pixel { r: 0, g: 0, b: 255 };
    }

    opc.emit(&pixels).unwrap();

    std::thread::sleep(std::time::Duration::from_secs(5));

    for i in 0..10 {
        pixels[i] = Pixel {
            r: 255,
            g: 0,
            b: 255,
        };
    }

    for i in 10..20 {
        pixels[i] = Pixel { r: 0, g: 255, b: 0 };
    }

    opc.emit(&pixels).unwrap();

    std::thread::sleep(std::time::Duration::from_secs(5));
}
