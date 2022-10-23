use ansi_escapes;
static LUMINANCE_CHARS: [char; 12] = ['.', ',', '-', '~', ':', ';', '=', '!', '*', '#', '$', '@'];

pub struct Canvas {
    height: usize,
    width: usize,
}
impl Canvas {
    pub fn new(height: usize, width: usize) -> Self {
        Canvas { height, width }
    }
    pub fn create(&self) {
        print!("{}", ansi_escapes::CursorHide);
        print!("{}", ansi_escapes::EraseScreen);
    }
    pub fn draw(&self, buf: &Vec<Vec<f32>>) {
        print!("{}", ansi_escapes::CursorTo::TopLeft);
        for y in 0..self.height {
            for x in 0..self.width {
                let lumi = buf[x as usize][y as usize];
                if lumi == -f32::MAX {
                    print!(" ");
                } else {
                    print!("{}", LUMINANCE_CHARS[(lumi.max(0.0) * 11.0) as usize]);
                }
                print!(" ");
            }
            println!();
        }
    }
    pub fn destroy(&self) {
        print!("{}", ansi_escapes::CursorShow);
    }
}
