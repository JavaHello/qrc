use qrcode::QrCode;
use std::env;
use std::io;
fn main() {
    let mut args = env::args();
    args.next();
    let mut flag = true;
    for qrc in args {
        flag = false;
        print_qrcode(&qrc);
    }
    if flag {
        let stdin = io::stdin();
        let mut qrc = String::new();
        if let Ok(n) = stdin.read_line(&mut qrc) {
            if n > 0 && qrc != "\n" {
                print_qrcode(&qrc);
            }
        }
    }
}

fn print_qrcode(qcs: &str) {
    println!();
    println!("--------------------------------------------------");
    println!("原始数据: {}", qcs);
    println!("--------------------------------------------------");
    println!();
    let code = QrCode::new(qcs).unwrap();
    let qrcode = code
        .render::<char>()
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .build();
    println!("{}", qrcode);
}
