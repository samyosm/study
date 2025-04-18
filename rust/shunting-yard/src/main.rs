use calculator::calculate;

mod calculator;
mod parser;
mod tokenizer;

fn main() {
    let input = "sin((0 - 1) * 2  * pi) + ln(e^4 * e^4 * e^pi / e^8) + log(10)";
    let result = calculate(input);
    println!("{result:.10}");
}
