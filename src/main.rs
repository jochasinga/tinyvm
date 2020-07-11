use std::convert::TryInto;

const NUM_REGS: usize = 4;

fn main() {
    let mut pc = 0;
    let prog = [0x1064, 0x11C8, 0x2201, 0x0000];
    let mut regs = [0; NUM_REGS];

    let mut instr_num = 0;
    // operands
    let mut reg1 = 0;
    let mut reg2 = 0;
    let mut reg3 = 0;
    let mut imm = 0;

    let mut running = true;

    while running {
	
	show_regs(regs);

	/* fetch */
	let instr = prog[pc];
	pc += 1;

	/* decode instructions */
	instr_num = ((instr & 0xF000) >> 12).try_into().unwrap();
	reg1 = ((instr & 0xF00) >> 8).try_into().unwrap();
	reg2 = ((instr & 0xF0) >> 4).try_into().unwrap();
	reg3 = (instr & 0xF).try_into().unwrap();
	imm = (instr & 0xFF).try_into().unwrap();

	match instr_num {
	    0 => {
		println!("halt");
		running = false;
	    },
	    1 => {
		println!("loadi r{} #{}", reg1, imm);
		regs[reg1] = imm
	    },
	    2 => {
		println!("add r{} r{} r{}", reg1, reg2, reg3);
		regs[reg1] = regs[reg2] + regs[reg3];
	    },
	    _ => {
		println!("unknown");
		running = false;
	    },
	}
    }
}

// Display all registers as 4-digit hexadecimal words.
fn show_regs(regs: [u16; 4]) {
    let mut s = "regs = ".to_string();
    for i in 0..NUM_REGS {
	s += &format!("{:0>4} ", format!("{:X}", regs[i]));
    }
    println!("{}", s);
}



