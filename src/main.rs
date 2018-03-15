use std::u8;
use std::fs::File;
use std::io::Read;

struct TuringTape {
    fields : Vec<u8>,
    pointer : usize,
    input : Vec<u8>,
}
impl TuringTape {
    fn init() -> TuringTape {
        let mut f : Vec<u8> = Vec::new();
        let mut i : Vec<u8> = Vec::new();
        i.resize(0,0);
        f.resize(1000,0);
        TuringTape { pointer : 0, fields : f, input : i }
    }
    fn incr(&mut self) {
        if self.fields[self.pointer] < u8::MAX {
            self.fields[self.pointer] += 1;
        }
    }
    fn decr(&mut self){
        if self.fields[self.pointer] != 0 {
            self.fields[ self.pointer] -= 1;
        }
    }
    fn frwd(&mut self) {
        self.pointer += 1;
        if self.pointer == self.fields.len() {
            self.pointer = 0;
        }
    }
    fn back(&mut self) {

        if self.pointer == 0 {
            self.pointer = self.fields.len()-1;
        }
        else {
            self.pointer -= 1;
        }
    }
    fn putc(&mut self) { print!("{}", self.fields[self.pointer] as char) }
    fn getc(&mut self) {
        if self.input.len() == 0 {

            let  mut s = String::new();
            std::io::stdin().read_line(&mut s).expect("Could not read from stdin");

            self.input = s.into_bytes();// as Vec<u8>;
            self.input.pop();
            self.input.reverse();

        }
        self.fields[self.pointer] = self.input.pop().expect("XD");
    }
    fn execute(&mut self, instructions: Vec<u8>) {
        let mut iterator= 0 as usize;
        loop {
            if iterator == instructions.len(){ println!("\nRustfuck finished with exit code 0"); return;  }
            match instructions[iterator] {
                43 => self.incr(),  // +
                45 => self.decr(),  // -
                60 => self.back(),  // <
                62 => self.frwd(),  // >
                46 => self.putc(),  // .
                44 => self.getc(),  // ,
                91 => if self.fields[self.pointer] == 0 {  // [
                    let mut loops = 1;
                    while loops > 0 {
                        iterator += 1;
                        if iterator == instructions.len(){ println!("\nRustfuck finished with exit code 1"); return;  }
                        match instructions[iterator]{
                            91 => loops += 1,
                            93 => loops -= 1,
                            _ => {}
                        }
                    }
                    //iterator += 1;
                },
                93 => {                                    // ]
                    let mut loops = 1;
                    while loops > 0 {
                        if iterator == 0 { println!("\nRustfuck finished with exit code 2"); return; }
                        iterator -= 1;
                        match instructions[iterator]{
                            91 => loops -= 1,
                            93 => loops += 1,
                            _ => {}
                        }
                    }
                    iterator -= 1;
                },
                _ => {}
            }
            iterator+=1;
        }
    }
}

fn load_code_from_file(path : String) -> Vec<u8> {
    let mut f = File::open(path).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.into_bytes()
}
fn main() {
    let mut tt = TuringTape::init();
    tt.execute(load_code_from_file(String::from("hello.txt")));
}
