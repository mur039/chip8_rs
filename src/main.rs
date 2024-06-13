
extern crate sdl2;
pub mod core;




use sdl2::event::Event;
use sdl2::keyboard::Keycode;


pub fn main() -> Result<(), String> {

    
    let sdl_context = sdl2::init()?;
    let mut chip8 = core::Chip8VM::initialize(&sdl_context, 16);
    chip8.core.load_program("./program.ch8");
    chip8.core.i_register = 5;
    let mut event_pump = sdl_context.event_pump()?;
    
    'running: loop {

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,

                Event::KeyDown {keycode : Some(Keycode::A), ..} => {chip8.core.register_keypress('A')},
                Event::KeyDown {keycode : Some(Keycode::B), ..} => {chip8.core.register_keypress('B')},
                Event::KeyDown {keycode : Some(Keycode::C), ..} => {chip8.core.register_keypress('C')},
                Event::KeyDown {keycode : Some(Keycode::D), ..} => {chip8.core.register_keypress('D')},
                Event::KeyDown {keycode : Some(Keycode::E), ..} => {chip8.core.register_keypress('E')},
                Event::KeyDown {keycode : Some(Keycode::F), ..} => {chip8.core.register_keypress('F')},
                Event::KeyDown {keycode : Some(Keycode::Num0), ..} | Event::KeyDown {keycode : Some(Keycode::Kp0), ..} => {chip8.core.register_keypress('0')},
                Event::KeyDown {keycode : Some(Keycode::Num1), ..} | Event::KeyDown {keycode : Some(Keycode::Kp1), ..} => {chip8.core.register_keypress('1')},
                Event::KeyDown {keycode : Some(Keycode::Num2), ..} | Event::KeyDown {keycode : Some(Keycode::Kp2), ..} => {chip8.core.register_keypress('2')},
                Event::KeyDown {keycode : Some(Keycode::Num3), ..} | Event::KeyDown {keycode : Some(Keycode::Kp3), ..} => {chip8.core.register_keypress('3')},
                Event::KeyDown {keycode : Some(Keycode::Num4), ..} | Event::KeyDown {keycode : Some(Keycode::Kp4), ..} => {chip8.core.register_keypress('4')},
                Event::KeyDown {keycode : Some(Keycode::Num5), ..} | Event::KeyDown {keycode : Some(Keycode::Kp5), ..} => {chip8.core.register_keypress('5')},
                Event::KeyDown {keycode : Some(Keycode::Num6), ..} | Event::KeyDown {keycode : Some(Keycode::Kp6), ..} => {chip8.core.register_keypress('6')},
                Event::KeyDown {keycode : Some(Keycode::Num7), ..} | Event::KeyDown {keycode : Some(Keycode::Kp7), ..} => {chip8.core.register_keypress('7')},
                Event::KeyDown {keycode : Some(Keycode::Num8), ..} | Event::KeyDown {keycode : Some(Keycode::Kp8), ..} => {chip8.core.register_keypress('8')},
                Event::KeyDown {keycode : Some(Keycode::Num9), ..} | Event::KeyDown {keycode : Some(Keycode::Kp9), ..} => {chip8.core.register_keypress('9')},

                // Event::KeyDown {keycode : Some(Keycode::Space), ..} => {

                    
                // },

                // Event::KeyDown { ..} => { println!("{:#?}", event); },

                _ => {}
            }
        }


        println!("{}", chip8.core);
        chip8.core.step();
        chip8.update_screen();    
        
        
    }

    Ok(())
}