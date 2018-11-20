mod ioutil;


pub fn run(program: &str) {
    let mut interp = Interpreter::new(program);
    interp.run();
}

struct Interpreter {
    // We are using a Vec<u8> instead of a String because indexing
    // bytes is much more efficient than indexing UTF-8 chars
    code: Vec<u8>,
    memory: Vec<u8>,
    loops: Vec<usize>,
    pc: usize,  // Code pointer
    ptr: usize  // Memory pointer
}

impl Interpreter {
    fn new(program: &str) -> Interpreter {
        let mut interp_mem = Vec::with_capacity(100);
        interp_mem.push(0);

        Interpreter {
            code: program.bytes().collect(),
            memory: interp_mem,
            loops: Vec::new(),
            pc: 0,
            ptr: 0
        }
    }

    fn run(&mut self) {
        while self.pc < self.code.len() {
            match self.code[self.pc] {
                b'+' => self.add(),
                b'-' => self.sub(),

                b'>' => self.point_next(),
                b'<' => self.point_prev(),

                b'.' => ioutil::write_byte(self.memory[self.ptr]),
                b',' => self.memory[self.ptr] = ioutil::read_byte(),

                b'[' => self.loop_start(),
                b']' => self.loop_end(),

                _    => {}
            }

            self.pc += 1;
        }
    }

    fn add(&mut self) {
        let cell = &mut self.memory[self.ptr];
        *cell = cell.wrapping_add(1);
    }

    fn sub(&mut self) {
        let cell = &mut self.memory[self.ptr];
        *cell = cell.wrapping_sub(1);
    }

    fn point_next(&mut self) {
        self.ptr += 1;

        // If there's not enough memory, push a new byte (0)
        if self.ptr == self.memory.len() {
            self.memory.push(0);
        }
    }

    fn point_prev(&mut self) {
        if self.ptr == 0 {
            panic!("Memory pointer points to adress -1 at byte {}", self.pc);
        }

        self.ptr -= 1;
    }

    fn loop_start(&mut self) {
        if self.memory[self.ptr] == 0 {
            // Skip loop
            let mut loop_count = 1;  // Count nested loops

            while loop_count > 0 {
                self.pc += 1;

                match self.code[self.pc] {
                    b'[' => loop_count += 1,
                    b']' => loop_count -= 1,
                    _    => {},
                }
            }
        }
        else {
            self.loops.push(self.pc);
        }
    }

    fn loop_end(&mut self) {
        if self.memory[self.ptr] == 0 {
            self.loops.pop();
        }
        else {
            match self.loops.last() {
                Some(x) => self.pc = *x,
                None    => panic!("Found no matching loop start to ']' at byte {}", self.pc),
            }
        }
    }
}