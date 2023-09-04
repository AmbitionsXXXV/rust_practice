use std::io;

// 根据控制台输入，打印pi的前n位
fn main() {
  println!("请输入要打印的π的前n位：");

  let mut input = String::new(); // 这一行就是变量声明，let mut 代表声明的是一个可变的变量，如果不加 mut 默认声明的就是一个不可变的变量
  io::stdin()
    .read_line(&mut input) // 这里我们拿到了 input 这个变量的可变引用，也因此在声明的时候需要声明称可变变量
    .expect("读取输入失败");

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
