// 定义配置结构体
#[derive(Debug)]
pub struct Config {
  pub url: String,         // 查询字符串
  pub output_path: String, // 文件路径
}

impl Config {
  // 构建配置结构体的实例
  pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
    args.next();

    // 存储命令行参数
    let url = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a query string"),
    };

    let output_path = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a file output path"),
    };

    // 返回构建好的配置实例
    Ok(Config { url, output_path })
  }
}
