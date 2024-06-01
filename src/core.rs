use core::fmt;
use  std::{fs, ops::Index};

extern crate sdl2;

use sdl2::{pixels::Color, render::Canvas, /* timer::Timer, */ video::Window, Sdl};





pub struct Core {
    vregisters : [u8; 16],//general purpose registers V0..F
    i_register : u16,
    pub dt_register : u8,
    pub st_register : u8,
    pc : u16,
    pub memory : [u8; 4096],
    screen : [[u8; 64] ; 32], //32*64 screen
    pub stack : Vec<u16>,

}


impl Core {

    pub fn new() -> Core {
        let mut ret = Core
        {
            vregisters: [0; 16],
              i_register: 0,
            dt_register: 0,
            st_register: 0,
            pc : 512,
            memory : [0 ; 4096],
            screen : [[0 ; 64] ; 32],
            stack : Vec::new(),
            
        };
        let fonts  = [
                                    0xF0, 0x90, 0x90, 0x90, 0xF0,		// 0
	                                0x20, 0x60, 0x20, 0x20, 0x70,		// 1    
	                                0xF0, 0x10, 0xF0, 0x80, 0xF0,		// 2
	                                0xF0, 0x10, 0xF0, 0x10, 0xF0,		// 3
	                                0x90, 0x90, 0xF0, 0x10, 0x10,		// 4
	                                0xF0, 0x80, 0xF0, 0x10, 0xF0,		// 5
	                                0xF0, 0x80, 0xF0, 0x90, 0xF0,		// 6
	                                0xF0, 0x10, 0x20, 0x40, 0x40,		// 7
	                                0xF0, 0x90, 0xF0, 0x90, 0xF0,		// 8
	                                0xF0, 0x90, 0xF0, 0x10, 0xF0,		// 9
	                                0xF0, 0x90, 0xF0, 0x90, 0x90,		// A
	                                0xE0, 0x90, 0xE0, 0x90, 0xE0,		// B
	                                0xF0, 0x80, 0x80, 0x80, 0xF0,		// C
	                                0xE0, 0x90, 0x90, 0x90, 0xE0,		// D
	                                0xF0, 0x80, 0xF0, 0x80, 0xF0,		// E
	                                0xF0, 0x80, 0xF0, 0x80, 0x80		// F
                                ];
        for (pos, val) in fonts.iter().enumerate() {
            ret.memory[pos] = val.clone();
        }
        ret
    }

    pub fn load_program(&mut self, path :&str) -> bool{
        let file = fs::read(path).expect("[-]Failed to read file.");
        if file.len() > 3583 /*4095-512 */ { return false; } //too big  -to fit in the memory
        for (i, byte) in file.iter().enumerate(){ //why first line is not read in to the fucking memory
            self.memory[512 + i] = *byte;
        }
        true
    }

    pub fn dump_memory(&self) -> () {
        for (i, byte) in self.memory.iter().enumerate() {
            if i % 16 == 0 {
                print!("\n{:03x} :", i);
            }
            else{
                print!(" {:02x}", byte)
            }

        }
        println!("");
    }

    pub fn step(&mut self) -> () {
        let op_h = self.memory.index(self.pc as usize);
        let op_l = self.memory.index((self.pc + 1) as usize);
        let mut opcode : u16 = 0;
        opcode |= ((*op_h as u16 )<< 8) as u16;
        opcode |= *op_l as u16;
        
        match opcode >> 12 {
            0x0 => {
                match opcode & 0xFFF{
                    0x0E0 => {self.screen.fill([0x00; 64]);}, //clear screen
                    0x0EE => { //return
                        if self.stack.len() == 0 {panic!("Stack Underflow!");}
                        else if self.stack.len() >= 16 {panic!("Stack Overflow!");}

                        self.pc = self.stack.pop().unwrap();
                        return;
                    }, 
                    _ => {println!("System calls are not supported.")}
                }
            },
            0x1 =>{ //jp address
                self.pc = opcode & 0xFFF;
                return; //jump over the +=2 
            }
            0x2 => {  //call addr
                self.stack.push(self.pc); //return address                
                self.pc = opcode & 0xFFF;
                return;
            }, 
            _ => {panic!("Invalid opcode => {:03x} {:02x}", self.pc, opcode);}
        }
        // println!("{:02x} : {:04x}", self.pc, opcode);
        self.pc += 2;
    }
    
}


impl fmt::Display for Core{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        

        writeln!(f, "V Registers :{:x?}", self.vregisters).unwrap();
        writeln!(f, "Stack :{:x?}", self.stack).unwrap();
        writeln!(f, " I  = {:2x}", self.i_register).unwrap();
        writeln!(f, " ST = {:2x}", self.st_register).unwrap();
        writeln!(f, " DT = {:2x}", self.dt_register).unwrap();
        writeln!(f, " PC ={:3x}", self.pc).unwrap();

        Ok(()) // not a great idea right?
    }
}

impl Core {
    pub fn execute(&mut self) -> (){ //return nothing. like a void function

    }
    pub fn timer_callbacks() -> u32 {
        println!("Merhabalar efenim.");
        // if self.st_register != 0 { self.st_register -= 1;}
        // if self.dt_register != 0 { self.dt_register -= 1;}
        1
    }
}

pub struct Chip8VM{
    pub core : Core,
    pub canvas: Canvas<Window>,

}

impl Chip8VM{
    pub fn initialize(sdl_context: &Sdl, scale: u32) -> Chip8VM {

        // Set up video
        let video_subsystem = sdl_context.video().unwrap();
        
        let window = video_subsystem.window("Rusty CHIP-8", 64 * scale, 32 * scale)
        .position_centered()
         .opengl()
         .build()
         .unwrap();
     let mut canvas = window.into_canvas().build().unwrap();
    let core = Core::new();


    // let timer_subsysten = sdl_context.timer().unwrap();    
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
     
     Chip8VM {
         core : core,
         canvas: canvas
     }
    }
}