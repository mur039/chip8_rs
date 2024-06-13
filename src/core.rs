use core::fmt;
use  std::{fs, ops::Index};

extern crate sdl2;

use sdl2::{ pixels::Color, render::Canvas, video::Window, Sdl};

use rand::Rng;
/*


    let mut rng = 
    println!("Integer: {}", rng.gen_range(0..10));
    

*/



pub struct Core {
    pub vregisters : [u8; 16],//general purpose registers V0..F
    pub i_register : u16,
    pub dt_register : u8,
    pub st_register : u8,
    pub pc : u16,
    pub memory : [u8; 4096],
    pub screen : [[bool; 64] ; 32], //32*64 screen
    pub stack : Vec<u16>,
    pub key_stack : Vec<char>,
    should_screen_update : bool

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
            screen : [[false ; 64] ; 32],
            stack : Vec::new(),
            key_stack : Vec::new(),
            should_screen_update: false,
            
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
        
        /*_xy_ */
        let y   = (opcode & 0x00F0) >> 4;
        let x   = (opcode & 0x0F00) >> 8;
        let n   = (opcode & 0x0F00) >> 8;
        let kk  =  opcode & 0x00FF;
        let nnn =  opcode & 0xFFF;

        match opcode >> 12 {
            0x0 => {
                match nnn{
                    0x0E0 => {self.screen.fill([false; 64]);}, //clear screen
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
                self.pc = nnn;
                return; //jump over the +=2 
            }
            0x2 => {  //call addr
                self.stack.push(self.pc); //return address                
                self.pc = nnn;
                return;
            }, 

            0x3 => { //3nkk Skip if `Vn` equals `kk`
                if self.vregisters[n as usize] == kk as u8 {
                    self.pc+= 2;
                }
                
            }

            0x4 => { // 4nkk Skip if `Vn` does not equal `kk`
                if self.vregisters[n as usize] != kk as u8 {
                    self.pc+= 2;
                }
            }

            0x5 => { // 5xy0 Skip if `Vn` equals `Vy` 
                if self.vregisters[x as usize] == self.vregisters[y as usize] {
                    self.pc+= 2;
                }

            }

            0x6 => { // 6xkk Loads a literal value `kk` into `Vx`
                self.vregisters[x as usize] = kk as u8;
            }

            0x7=> { // * |  7xkk Add `kk` to register `Vx`
                self.vregisters[x as usize] += kk as u8;
            }

            0x8 => {
                match opcode & 0xF{
                    0x0=>{//8xy0 Loads register `Vy` into `Vx`
                        self.vregisters[x as usize] = self.vregisters[y as usize];
                    },
                    0x1=>{//8xy1 Bitwise OR the val ue in `Vy` with register `Vx`
                        self.vregisters[x as usize] |= self.vregisters[y as usize];
                    },
                    0x2=>{//8xy2 Bitwise AND the value in `Vy` with register `Vx`
                        self.vregisters[x as usize] &= self.vregisters[y as usize];
                    },
                    0x3=>{//8xy3 Bitwise XOR the value in `Vy` with register `Vx`
                        self.vregisters[x as usize] ^= self.vregisters[y as usize];
                    },
                    0x4=>{//8xy4 Add the value in `Vy` to register `Vx`
                        if (self.vregisters[x as usize ] as usize + self.vregisters[y as usize] as usize) > 255 {
                            self.vregisters[0xf] = 1;
                        }
                        self.vregisters[x as usize] += self.vregisters[y as usize];
                    },
                    0x5=>{//8xy5 Subtract the value in `Vy` from register `Vx`
                        if self.vregisters[y as usize] > self.vregisters[x as usize] { self.vregisters[0xf] = 1; }
                        self.vregisters[x as usize] -= self.vregisters[y as usize];
                    },

                    0x6=>{//8xy6 Shift `Vx` to the right with the value in `Vy`  
                        self.vregisters[x as usize] >>= self.vregisters[y as usize];
                    },
                    0x7=>{//8xy7 Subtract the value in `Vy` from `Vx`, no carry 
                        self.vregisters[x as usize] -= self.vregisters[y as usize];
                    },
                    0xE=>{//8xyE Shift `Vx` to the left with the value in `Vy`   
                        self.vregisters[x as usize] <<= self.vregisters[y as usize];
                    },
                    _=> {panic!("Invalid opcode => {:03x} {:02x}", self.pc, opcode);}
                }    
            }
            0x9 =>{ // * |  9xy0 Skip if `Vx` does not equal `Vy`
                if self.vregisters[x as usize] != self.vregisters[y as usize] {
                    self.pc+= 2;
                }
            }
            
            0xA =>{ // * |  Annn Loads `nnn` into register `I`
                self.i_register = nnn;
            }

            0xB => { // Bnnn - Jump to `v0 + addr`
                self.pc = self.vregisters[0x0] as u16 + nnn;
            }

            0xC => { //Cnkk random number AND `kk` into `Vn`
                let mut random = rand::thread_rng();
                self.vregisters[x as usize] = random.gen_range(0..255) & kk as u8;
            }

            0xD =>{ //Dxyn Draw a sprite of `n` rows at `Vx,Vy` n heigth from memory[I]
                self.should_screen_update = true;
                for i in 0..(opcode & 0xF) as u8 {
                    for bit in 0..8{
                        self.screen[((self.vregisters[y as usize])+ i) as usize ][(self.vregisters[x as usize]) as usize + bit] ^= if ((self.memory[ (self.i_register + i as u16) as usize ] >> (7 - bit)) & 1 ) == 1 {true} else { false} ;    
                    }
                }
                
            }

            0xE =>{ //both does not block
                let reg_index = (opcode & 0x0F00) >> 8;

                match opcode & 0x00FF { 
                    0x9E =>{ //En9E Skip if key in `Vn` pressed        
                        if let Some(key) = self.key_stack.pop(){
                            if key as u8 == self.vregisters[reg_index as usize]{
                                self.pc += 2;
                            }
                        }
                        
                    }

                    0xA1 =>{ //EnA1 Skip if key in `Vn` not pressed
                        if let Some(key) = self.key_stack.pop(){
                            if key as u8 != self.vregisters[reg_index as usize]{
                                self.pc += 2;
                            }
                        }
                    }
                    _=> {panic!("Invalid opcode => {:03x} {:02x}", self.pc, opcode);}
                }
                
            }

            0xF => {
                let reg_index = ((opcode & 0x0F00) >> 8) as usize;
                match opcode & 0xFF{
                    0x07 => { //fx07  Loads the delay timer into register `Vx`
                        self.vregisters[reg_index] = self.dt_register; }

                    0x0A => { //Fx0A - LD Vx, K //load key to vx, blocks
                            if let Some(key) = self.key_stack.pop(){
                                self.vregisters[reg_index] = key as u8; }
                            else{
                                self.vregisters[reg_index] = 0xFF as u8; }
                            }

                    0x15 => { //set DT to Vx
                        self.dt_register = self.vregisters[reg_index]; }

                    0x18 => { //set ST to vx
                        self.st_register = self.vregisters[reg_index]; }

                    0x1E => { //add I, Vx => I += Vx
                        self.i_register += self.vregisters[reg_index] as u16;}

                    0x29 => { //LD F, Vx Set I = location of sprite for digit Vx.
                        let mut digit = self.vregisters[reg_index] as u16;
                        digit *= 5;
                        self.i_register = digit;
                    }

                    0x33 => { //Store BCD representation of Vx in memory locations I, I+1, and I+2.
                        let bcd = self.vregisters[reg_index];
                        self.memory[self.i_register as usize ] = bcd/100;
                        self.memory[self.i_register as usize + 2] = bcd/10 - (bcd /100 )*10; //problematic
                        self.memory[self.i_register as usize + 2] = bcd - (bcd /10 )*10;
                    }
                    0x55 => { // Store registers V0 through Vx in memory starting at location I.
                        for i in 0..reg_index{
                            self.memory[self.i_register as usize + i] = self.vregisters[i];
                        }
                    }
                    0x65 => { // Read registers V0 through Vx from memory starting at location I.
                        for i in 0..reg_index{
                            self.vregisters[i] = self.memory[self.i_register as usize + i];
                        }
                    } 
                    _=> {panic!("Invalid opcode => {:03x} {:02x}", self.pc, opcode);}
                }
              
            }

            _  => {panic!("Invalid opcode => {:03x} {:02x}", self.pc, opcode);}
        }
        // println!("{:02x} : {:04x}", self.pc, opcode);
        self.pc += 2;

        //if key is not used, its better to delete it
        self.key_stack.clear();
    }
    
}


impl fmt::Display for Core{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        

        writeln!(f, "V Registers :{:x?}", self.vregisters).unwrap();
        writeln!(f, "Stack :{:x?}", self.stack).unwrap();
        writeln!(f, "Key Stack :{:x?}", self.key_stack).unwrap();   
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
    pub fn register_keypress(&mut self, key : char) -> () {
        self.key_stack.push(key);
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
    pub pixels : [[sdl2::rect::Rect; 64]; 32]


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
    
    //[[sdl2::rect::Rect; 64]; 32]
    let mut pixs  = [[sdl2::rect::Rect::new(0, 0, 0, 0); 64]; 32];
   
    for (irow, row) in pixs.iter_mut().enumerate() {
        for (icol, col) in row.iter_mut().enumerate() {
            col.set_x((scale * icol as u32) as i32);
            col.set_y((scale * irow as u32) as i32);
            col.set_height(scale);
            col.set_width(scale);
            
        }
    }



    // let timer_subsysten = sdl_context.timer().unwrap();    
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
     
     Chip8VM {
         core : core,
         canvas: canvas,
         pixels : pixs
     }
    }

    pub fn set_pixel(&mut self, x:usize, y:usize, value:bool) -> () {
        let color = if value == true { Color::RGB(255, 255, 255)} else {Color::RGB(0, 0, 0)};
        self.canvas.set_draw_color(color);
        self.canvas.draw_rect(self.pixels[y][x]).unwrap();
        self.canvas.fill_rect(self.pixels[y][x]).unwrap();
        self.canvas.present();
        return;
    }

    pub fn update_screen(&mut self) -> () {
        if !self.core.should_screen_update{return;}
        for (i_row, row) in self.core.screen.iter_mut().enumerate(){
            for (i, pixel) in row.iter_mut().enumerate(){
                // self.set_pixel(i, i_row, *pixel);
                if *pixel == false { self.canvas.set_draw_color(Color::RGB(0, 0, 0));}
                else {self.canvas.set_draw_color(Color::RGB(255, 255, 255));}

                self.canvas.draw_rect(self.pixels[i_row][i]).unwrap();
                self.canvas.fill_rect(self.pixels[i_row][i]).unwrap();
            }
            
        }
        self.canvas.present();
        return
    }
}