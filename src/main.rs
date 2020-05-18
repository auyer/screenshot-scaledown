extern crate winapi;
extern crate image;

mod screenshot;
use std::fmt;
use std::time::{Duration, Instant};
use image::ImageFormat;
use image::imageops::FilterType;
use std::fs::File;

fn main() {
	stealth();
	loop {
		let print = unsafe { winapi::um::winuser::GetAsyncKeyState(winapi::um::winuser::VK_PRINT) };
		let snapshot =
			unsafe { winapi::um::winuser::GetAsyncKeyState(winapi::um::winuser::VK_SNAPSHOT) };
		if print == -32767
			|| print == -32768
			|| print == 1
			|| snapshot == -32767
			|| snapshot == -32768
			|| snapshot == 1
		{
			print_action();
			println!("PRINT")
		}
		unsafe { winapi::um::synchapi::SleepEx(1000, 1000) };
	}
}

// Remove after debug
struct Elapsed(Duration);
impl Elapsed {
    fn from(start: &Instant) -> Self {
        Elapsed(start.elapsed())
    }
}

impl fmt::Display for Elapsed {
    fn fmt(&self, out: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match (self.0.as_secs(), self.0.subsec_nanos()) {
            (0, n) if n < 1000 => write!(out, "{} ns", n),
            (0, n) if n < 1000_000 => write!(out, "{} µs", n / 1000),
            (0, n) => write!(out, "{} ms", n / 1000_000),
            (s, n) if s < 10 => write!(out, "{}.{:02} s", s, n / 10_000_000),
            (s, _) => write!(out, "{} s", s),
        }
    }
}

fn print_action() {
	let s = screenshot::get_screenshot(0).unwrap();

	println!("size {}", std::mem::size_of_val(&s));

	println!("{} x {} x {} = {} bytes", s.height(), s.width(), s.pixel_width(), s.raw_len());

	let origin = s.get_pixel(0, 0);
	println!("(0,0): R: {}, G: {}, B: {}", origin.r, origin.g, origin.b);

	let end_col = s.get_pixel(0, s.width()-1);
	println!("(0,end): R: {}, G: {}, B: {}", end_col.r, end_col.g, end_col.b);

	let opp = s.get_pixel(s.height()-1, s.width()-1);
	println!("(end,end): R: {}, G: {}, B: {}", opp.r, opp.g, opp.b);


	let mut img = image::ImageBuffer::new(s.width() as u32, s.height() as u32);
    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in img.enumerate_pixels_mut() {
		// *pixel = s.get_pixel(x as usize, y as usize);
		let pix = s.get_pixel(y as usize, x as usize);
	
		*pixel = image::Rgb([pix.b, pix.g, pix.r]);
    }
	let img = image::DynamicImage::ImageRgb8(img);
	let mut output = File::create(&"test.jpg").unwrap();
	img.write_to(&mut output, ImageFormat::Jpeg).unwrap();
	for &(name, filter) in [
        ("tri", FilterType::Triangle),
        ("cmr", FilterType::CatmullRom),
        ("lcz2", FilterType::Lanczos3),
    ].iter()
    {
        let timer = Instant::now();
        let scaled = img.resize(1920, 1080, filter);
        println!("Scaled by {} in {}", name, Elapsed::from(&timer));
        let mut output = File::create(&format!("test-{}.png", name)).unwrap();
        scaled.write_to(&mut output, ImageFormat::Jpeg).unwrap();
    }
}

fn stealth() {
	let stealth: winapi::shared::windef::HWND;
	unsafe {
		winapi::um::consoleapi::AllocConsole();
		stealth = winapi::um::winuser::FindWindowA(
			std::ffi::CString::new("ConsoleWindowClass")
				.unwrap()
				.as_ptr(),
			std::ptr::null(),
		);
		winapi::um::winuser::ShowWindow(stealth, 0);
	}
}
