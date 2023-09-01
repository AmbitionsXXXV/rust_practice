use std::io;

// 根据控制台输入，打印pi的前n位
fn main() {
    println!("请输入要打印的π的前n位：");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");

    let n: usize = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("无效的输入");
            return;
        }
    };

    if n == 0 {
        println!("请输入一个大于零的数");
        return;
    }

    let pi_digits = format!("{:.prec$}", std::f64::consts::PI, prec = n + 2);
    println!("{}", &pi_digits[..n + 2]);
}
