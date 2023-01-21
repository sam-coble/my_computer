use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::env;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;


#[derive(Debug)]
struct Memory {
    data: [u8; 256]
}
impl Memory {
    fn get(&self, addr: u8) -> u8 {
        self.data[usize::from(addr)]
    }
    fn set(&mut self, addr: u8, val: u8) {
        self.data[usize::from(addr)] = val;
    }
}

struct CPU {
    r0: u8,
    r1: u8,
    r2: u8,
    r3: u8,
    pc: u8,
    ir: u8,
    imm: u8,
    mem: Memory,
    running: bool,
}
impl CPU {
    fn tick(&mut self) {
        self.load_instruction();
        self.execute();
    }
    fn load_instruction(&mut self) {
        self.ir = self.mem.get(self.pc);
        self.pc += 1;
        self.imm = self.mem.get(self.pc);
    }
    fn execute(&mut self) {
        match (self.ir & 0xf0) >> 4 {
            0x0 => self.ldi(),
            0x1 => self.ld(),
            0x2 => self.st(),
            0x3 => self.j(),
            0x4 => self.br(),
            0x5 => self.bgt(),
            0x6 => self.beq(),
            0x7 => self.halt(),
            0x8 => self.mv(),
            0x9 => self.add(),
            0xa => self.and(),
            0xb => self.inc(),
            0xc => self.shx(),
            0xd => self.not(),
            0xe => self.gpc(),
            _ => self.nop(),
        }
    }
    fn get_reg(&self, r: u8) -> u8 {
        match r {
            0 => self.r0,
            1 => self.r1,
            2 => self.r2,
            3 => self.r3,
            _ => panic!("invalid register"),
        }
    }
    fn set_reg(&mut self, r: u8, val: u8) {
        let reg: &mut u8 = match r {
            0 => &mut self.r0,
            1 => &mut self.r1,
            2 => &mut self.r2,
            3 => &mut self.r3,
            _ => panic!("invalid register"),
        };
        *reg = val;
    }
    fn ldi(&mut self) {
        self.set_reg((self.ir & 0x0c) >> 2, self.imm);
        self.pc += 1;
    }
    fn ld(&mut self) {
        self.set_reg(self.ir & 0x03, self.mem.get(self.get_reg((self.ir & 0x0c) >> 2)));
    }
    fn st(&mut self) {
        self.mem.set(self.get_reg(self.ir & 0x03), self.get_reg((self.ir & 0x0c) >> 2));
    }
    fn j(&mut self) {
        self.pc = self.mem.get(self.get_reg((self.ir & 0x0c) >> 2));
    }
    fn br(&mut self) {
        self.pc += ((self.ir & 0x0c) >> 2) * 4;
    }
    fn bgt(&mut self) {
        if self.get_reg(self.ir & 0x03) > 0 {
            self.br();
        }
    }
    fn beq(&mut self) {
        if self.get_reg(self.ir & 0x03) == 0 {
            self.br();
        }
    }
    fn halt(&mut self) {
        self.running = false;
    }
    fn mv(&mut self) {
        self.set_reg(self.ir & 0x03, self.get_reg((self.ir & 0x0c) >> 2));
    }
    fn add(&mut self) {
        self.set_reg(self.ir & 0x03, self.get_reg(self.ir & 0x03) + self.get_reg((self.ir & 0x0c) >> 2));
    }
    fn and(&mut self) {
        self.set_reg(self.ir & 0x03, self.get_reg(self.ir & 0x03) & self.get_reg((self.ir & 0x0c) >> 2));
    }
    fn inc(&mut self) {
        match (self.ir & 0x02) >> 1 {
            0x00 => self.set_reg((self.ir & 0x0c) >> 2, self.get_reg((self.ir & 0x0c) >> 2) + 1),
            _ => self.set_reg((self.ir & 0x0c) >> 2, self.get_reg((self.ir & 0x0c) >> 2) - 1),
        }
    }
    fn shx(&mut self) {
        match self.ir & 0x03 {
            0x00 => self.set_reg((self.ir & 0x0c) >> 2, self.get_reg((self.ir & 0x0c) >> 2) << 1),
            0x01 => self.set_reg((self.ir & 0x0c) >> 2, self.get_reg((self.ir & 0x0c) >> 2) << 2),
            0x02 => self.set_reg((self.ir & 0x0c) >> 2, self.get_reg((self.ir & 0x0c) >> 2) >> 1),
            _ => self.set_reg((self.ir & 0x0c) >> 2, self.get_reg((self.ir & 0x0c) >> 2) >> 2),
        }
    }
    fn not(&mut self) {
        self.set_reg((self.ir & 0x0c) >> 2, !self.get_reg((self.ir & 0x0c) >> 2));
    }
    fn gpc(&mut self) {
        self.set_reg((self.ir & 0x0c) >> 2, self.pc + (self.ir & 0x03) * 4);
    }
    fn nop(&self) {
        
    }
}

fn run(bin: &String) {
    let mut file = File::open(bin).unwrap();
    let mut data: [u8; 256] = [0x70; 256];
    file.read_exact(&mut data).unwrap();

    let mem = Memory {
        data: data,
    };
    let mut cpu = CPU {
        r0: 0,
        r1: 0,
        r2: 0,
        r3: 0,
        pc: 0,
        ir: 0,
        imm: 0,
        mem: mem,
        running: true,
    };
    println!("Starting CPU!");
    while cpu.running {
        cpu.tick();
    }
    println!("CPU halted!");
    println!("Memory = {:x?}", cpu.mem);
}
fn compile(_src: &String, out: &String) {
    let mut program:Vec<u8> = vec![
        0b0000_00_00, 0xf0,
        0b0000_01_00, 0xcc,
        0b0010_01_00,
        0b1011_00_00,
        0b1011_01_00,
        0b0010_01_00,
        0b1011_00_00,
        0b1011_01_00,
        0b0010_01_00,
        0b1011_00_00,
        0b1011_01_00,
        0b0010_01_00,
        0b1011_00_00,
        0b1011_01_00,
        0b0111_00_00,
    ];
    while program.len() < 256 {
        program.push(0x70);
    }
    fs::write(out, program).unwrap();
}
fn window() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() < 2 {
        println!("enter command!");
    } else {
        if args[1] == "run" {
            if args.len() < 3 {
                println!("enter program filename!");
            } else {
                run(&args[2]);
            }
        } else if args[1] == "compile" {
            if args.len() < 4 {
                println!("enter input and output filenames!");
            } else {
                compile(&args[2], &args[3]);
            }
        } else if args[1] == "window" {
            window();
        } else {
            println!("not a valid command!");
        }
    }
}