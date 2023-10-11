use qrcode::render::Renderer;
use qrcode::QrCode;

pub fn exec(text: &str) {
    let qr = QrCode::new(text).unwrap();
    let colors = qr.to_colors();
    let mut r = Renderer::<char>::new(&colors, (colors.len() as f64).sqrt() as usize, 2);
    println!("{}", r.module_dimensions(2, 1).build())
}

#[cfg(test)]
mod tests {
    use super::exec;
    #[test]
    fn test_exec() {
        let text = "https://example.com";
        exec(text);
    }
}
