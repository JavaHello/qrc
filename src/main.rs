use qrcode::QrCode;
use std::env;
fn main() {
    let mut args = env::args();
    args.next();
    loop {
        let qc = args.next();
        let qcs = match qc {
            Some(s) => s,
            None => break,
        };
        println!();
        println!("--------------------------------------------------");
        println!("原始数据: {}", qcs);
        println!("--------------------------------------------------");
        println!();
        let code = QrCode::new(&qcs).unwrap();
        let qrcode = code
            .render::<char>()
            .quiet_zone(false)
            .module_dimensions(2, 1)
            .build();
        println!("{}", qrcode);
    }
}
