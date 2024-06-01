
extern crate sdl2;
pub mod core;




use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub fn sdl_timer_callback() -> u32 { //this mf
    // vm.timer_callback()
    1
}
pub fn main() -> Result<(), String> {


    // vm.dump_memory();
    


    let sdl_context = sdl2::init()?;
    let mut chip8 = core::Chip8VM::initialize(&sdl_context, 16);
    chip8.core.load_program("./program.ch8");
    println!("{:#?}", chip8.core.stack);
    // chip8.core.dump_memory();
    // let video_subsystem = sdl_context.video()?;
    // let timer_subsystem = sdl_context.timer()?; 



    // canvas.set_draw_color(Color::RGB(255, 0, 0));
    // canvas.clear();
    // canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    
    'running: loop {

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                
                Event::KeyDown {keycode : Some(Keycode::Space), ..} => {
                    println!("{}", chip8.core);
                    chip8.core.step();

                },
                _ => {}
            }
        } 
        
        
        // break;
        // canvas.clear();
        // canvas.present();
        // if (timer_subsystem.ticks() % 16) == 0{ // about 60Hz
        //     vm.timer();
        // }
        // if vm.st_register == 0 { break 'running}
        // The rest of the game loop goes here...   
    }

    Ok(())
}