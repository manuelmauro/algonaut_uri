use std::str::FromStr;

use algonaut_core::Address;
use algonaut_uri::AlgorandUrn;
use image::Luma;
use qrcode::QrCode;

const ALGO_ADDRESS: &'static str = "TMTAD6N22HCS2LKH7677L2KFLT3PAQWY6M4JFQFXQS32ECBFC23F57RYX4";

fn main() {
    let uri = AlgorandUrn::builder()
        .address(Address::from_str(ALGO_ADDRESS).unwrap())
        .label("Silvio".to_string())
        .build();

    // Encode some data into bits.
    let code = QrCode::new(uri.to_string()).unwrap();

    // Render the bits into an image.
    let image = code.render::<Luma<u8>>().build();

    // Save the image.
    image.save("./qrcode.png").unwrap();

    // You can also render it into a string.
    let string = code.render().light_color(' ').dark_color('#').build();
    println!("{}", string);
}
