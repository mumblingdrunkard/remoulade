use remoulade::inst::Instruction;

fn main() {
    println!(
        "{}, {}",
        std::mem::size_of::<Instruction>(),
        std::mem::align_of::<Instruction>()
    );
}
