#[cfg(target_os = "windows")]
extern crate winapi;

#[cfg(target_os = "linux")]
extern crate x11rb;

extern crate livesplit_hotkey;
extern crate image;

mod screenshot;
use image::imageops::FilterType;
use image::ImageFormat;
use std::fmt;
use std::fs::File;
use std::time::{Duration, Instant};
use std::{thread, time};
fn main() {
	key_loop()
}

#[cfg(target_os = "windows")]
fn key_loop() {
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

#[cfg(target_os = "linux")]
fn key_loop() {
	let hook = livesplit_hotkey::linux::Hook::new().unwrap();
	hook.register(livesplit_hotkey::KeyCode::Print, print_action).unwrap();
	loop{
		thread::sleep(time::Duration::from_secs(10));
	}
}

// #[cfg(target_os = "linux")]
// fn key_loop() {


// 	use x11rb::connection::{Connection, SequenceNumber};
// 	use x11rb::errors::{ConnectionError, ReplyError, ReplyOrIdError};
// 	use x11rb::protocol::xproto::*;
// 	use x11rb::protocol::Event;
// 	use x11rb::wrapper::ConnectionExt as _;
// 	use x11rb::COPY_DEPTH_FROM_PARENT;

// 	fn print_modifiers(mask: u16) {
// 		use x11rb::protocol::xproto::KeyButMask::*;
// 		let mods = [
// 			(Shift, "Shift"),
// 			(Lock, "Lock"),
// 			(Control, "Ctrl"),
// 			(Mod1, "Alt"),
// 			(Mod2, "Mod2"),
// 			(Mod3, "Mod3"),
// 			(Mod4, "Mod4"),
// 			(Mod5, "Mod5"),
// 			(Button1, "Button1"),
// 			(Button2, "Button2"),
// 			(Button3, "Button3"),
// 			(Button4, "Button4"),
// 			(Button5, "Button5"),
// 		];
	
// 		let active = mods
// 			.iter()
// 			.filter(|(m, _)| mask & u16::from(*m) != 0) // FIXME: This should be made nicer
// 			.map(|(_, name)| name)
// 			.collect::<Vec<_>>();
// 		println!("Modifier mask: {:?}", active);
// 	}

// 	// Open the connection to the X server. Use the DISPLAY environment variable.
// 	let (conn, screen_num) = x11rb::connect(None).unwrap();

// 	// let number = match x11rb::connect(None) {
//     //     Ok(number)  => number,
//     //     Err(e) => return Err(e),
//     // };
// 	// Get the screen #screen_num
// 	let screen = &conn.setup().roots[screen_num];

// 	// Ask for our window's Id
// 	let win = conn.generate_id().unwrap();

// 	// Create the window
// 	let values = CreateWindowAux::default()
// 		.background_pixel(screen.white_pixel)
// 		.event_mask(
// 			EventMask::Exposure
// 				| EventMask::ButtonPress
// 				| EventMask::ButtonRelease
// 				| EventMask::KeyPress
// 				| EventMask::KeyRelease,
// 		);
// 	conn.create_window(
// 		COPY_DEPTH_FROM_PARENT,   // depth
// 		win,                      // window Id
// 		screen.root,              // parent window
// 		0,                        // x
// 		0,                        // y
// 		150,                      // width
// 		150,                      // height
// 		10,                       // border_width
// 		WindowClass::InputOutput, // class
// 		screen.root_visual,       // visual
// 		&values,
// 	).unwrap();

// 	// Map the window on the screen
// 	conn.map_window(win).unwrap();
// 	conn.flush().unwrap();

// 	loop {
// 		let event = conn.wait_for_event().unwrap();
// 		match event {
// 			Event::Expose(event) => {
// 				println!(
// 					"Window {} exposed. Region to be redrawn at location ({},{}) \
// 							 with dimensions ({},{})",
// 					event.window, event.x, event.y, event.width, event.height
// 				);
// 			}
// 			Event::ButtonPress(event) => {
// 				print_modifiers(event.state);
// 				match event.detail {
// 					4 => println!(
// 						"Wheel Button up in window {}, at coordinates ({},{})",
// 						event.event, event.event_x, event.event_y
// 					),
// 					5 => println!(
// 						"Wheel Button down in window {}, at coordinates ({},{})",
// 						event.event, event.event_x, event.event_y
// 					),
// 					_ => println!(
// 						"Button {} pressed in window {}, at coordinates ({},{})",
// 						event.detail, event.event, event.event_x, event.event_y
// 					),
// 				}
// 			}
// 			Event::ButtonRelease(event) => {
// 				print_modifiers(event.state);
// 				println!(
// 					"Button {} released in window {}, at coordinates ({},{})",
// 					event.detail, event.event, event.event_x, event.event_y
// 				);
// 			}
// 			Event::KeyPress(event) => {
// 				print_modifiers(event.state);
// 				println!("Key pressed in window {}", event.event);
// 			}
// 			Event::KeyRelease(event) => {
// 				print_modifiers(event.state);
// 				println!("Key released in window {}", event.event);
// 			}
// 			_ => {
// 				// Unknown event type, ignore it
// 				println!("Unknown event: {:?}", event);
// 			}
// 		}
// 	}
// }


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

	println!(
		"{} x {} x {} = {} bytes",
		s.height(),
		s.width(),
		s.pixel_width(),
		s.raw_len()
	);

	let origin = s.get_pixel(0, 0);
	println!("(0,0): R: {}, G: {}, B: {}", origin.r, origin.g, origin.b);

	let end_col = s.get_pixel(0, s.width() - 1);
	println!(
		"(0,end): R: {}, G: {}, B: {}",
		end_col.r, end_col.g, end_col.b
	);

	let opp = s.get_pixel(s.height() - 1, s.width() - 1);
	println!("(end,end): R: {}, G: {}, B: {}", opp.r, opp.g, opp.b);

	let mut img = image::ImageBuffer::new(s.width() as u32, s.height() as u32);
	// Iterate over the coordinates and pixels of the image
	for (x, y, pixel) in img.enumerate_pixels_mut() {
		// *pixel = s.get_pixel(x as usize, y as usize);
		let pix = s.get_pixel(y as usize, x as usize);
		if cfg!(windows){
			*pixel = image::Rgb([pix.b, pix.g, pix.r]);
		} else {
			*pixel = image::Rgb([pix.r, pix.g, pix.b]);
		}
	}
	let img = image::DynamicImage::ImageRgb8(img);
	let mut output = File::create(&"test.jpg").unwrap();
	img.write_to(&mut output, ImageFormat::Jpeg).unwrap();
	for &(name, filter) in [
		("tri", FilterType::Triangle),
		("cmr", FilterType::CatmullRom),
		("lcz2", FilterType::Lanczos3),
	]
	.iter()
	{
		let timer = Instant::now();
		let scaled = img.resize(1920, 1080, filter);
		println!("Scaled by {} in {}", name, Elapsed::from(&timer));
		let mut output = File::create(&format!("test-{}.png", name)).unwrap();
		scaled.write_to(&mut output, ImageFormat::Jpeg).unwrap();
	}
}
