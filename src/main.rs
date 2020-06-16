use vast::{v05, v17};

fn main() {
    println!("Verilog 2005{}", v05::Unop::LogicalNegation);
    println!("Verilog 2017{}", v17::Unop::LogicalNegation);
    println!("{:?}", v05::Module{ par_stmt: vec!["a".to_string()] });
}
