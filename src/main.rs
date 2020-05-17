// extern crate framing;
extern crate repng;
extern crate scrap;
extern crate winapi;

use scrap::{Capturer, Display};
use std::fs::File;
use std::io::ErrorKind::WouldBlock;
use std::thread;
use std::time::Duration;

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

fn print_action() {
	let one_second = Duration::new(1, 0);
	let one_frame = one_second / 60;
	let display = Display::primary().expect("Couldn't find primary display.");
	let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
	let (w, h) = (capturer.width(), capturer.height());

	loop {
		// Wait until there's a frame.
		let buffer = match capturer.frame() {
			Ok(buffer) => buffer,
			Err(error) => {
				if error.kind() == WouldBlock {
					// Keep spinning.
					thread::sleep(one_frame);
					continue;
				} else {
					panic!("Error: {}", error);
				}
			}
		};

        println!("Captured! Saving...");

        // Flip the ARGB image into a BGRA image.

        let mut bitflipped = Vec::with_capacity(w * h * 4);
        let stride = buffer.len() / h;

        for y in 0..h {
            for x in 0..w {
                let i = stride * y + 4 * x;
                bitflipped.extend_from_slice(&[buffer[i + 2], buffer[i + 1], buffer[i], 255]);
            }
        }

        // Save the image.

        repng::encode(
            File::create("screenshot.png").unwrap(),
            w as u32,
            h as u32,
            &bitflipped,
        )
        .unwrap();

        println!("Image saved to `screenshot.png`.");
        break;
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
