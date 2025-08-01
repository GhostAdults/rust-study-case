use std::error::Error;
// collect
use std::cell::RefCell;
use std::fs;

/// 运行主要的文件搜索逻辑
/// 
/// 此函数读取指定文件的内容，并在其中搜索指定的查询字符串。
/// 
/// # 参数
/// 
/// * `config` - 包含查询字符串和文件名的配置对象
/// 
/// # 返回值
/// 
/// * `Result<(), Box<dyn Error>>` - 成功时返回 (), 失败时返回错误
/// 
/// # 错误
/// 
/// 如果文件无法读取，此函数将返回错误
pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename.take())?;
    println!("with text :\n{}", contents);
    // search
    println!("results:{:#?}", search(&config.query.borrow(), &contents));
    Ok(())
}

/// 文件配置接口
/// 
/// 定义了配置文件处理所需的基本方法
pub trait FileConfiguration {
    /// 从命令行参数中推送变量到配置中
    /// 
    /// # 参数
    /// 
    /// * `args` - 命令行参数向量
    /// 
    /// # 返回值
    /// 
    /// * `Result<(), &'static str>` - 成功时返回 (), 失败时返回错误信息
    fn push_variable(&self, args: Vec<String>) -> Result<(), &'static str>;
}

/// 为 Config 结构体实现 FileConfiguration trait
impl FileConfiguration for Config {
    /// 从命令行参数中提取查询字符串和文件名
    /// 
    /// 期望参数格式: [程序名, 查询字符串, 文件名, ...]
    /// 
    /// # 参数
    /// 
    /// * `args` - 命令行参数向量，至少需要3个元素
    /// 
    /// # 返回值
    /// 
    /// * `Result<(), &'static str>` - 成功时返回 (), 参数不足时返回错误
    /// 
    /// # 错误
    /// 
    /// 如果参数少于3个，返回 "not enough arguments" 错误
    fn push_variable(&self, args: Vec<String>) -> Result<(), &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        self.query.borrow_mut().push_str(&args[1]);
        self.filename.borrow_mut().push_str(&args[2]);
        Ok(())
    }
}

/// 配置结构体
/// 
/// 存储搜索所需的查询字符串和目标文件名
/// 使用 RefCell 来允许内部可变性
pub struct Config {
    /// 要搜索的查询字符串
    pub query: RefCell<String>,
    /// 要搜索的目标文件名
    pub filename: RefCell<String>,
}

impl Config {
    /// 创建一个新的空配置实例
    /// 
    /// # 返回值
    /// 
    /// * `Config` - 包含空字符串的新配置实例
    pub fn new() -> Config {
        Config {
            query: RefCell::new(String::new()),
            filename: RefCell::new(String::new()),
        }
    }
}

/// 在给定的文本内容中搜索匹配查询字符串的行
/// 
/// 此函数逐行搜索文本内容，返回所有包含查询字符串的行。
/// 
/// # 参数
/// 
/// * `query` - 要搜索的字符串
/// * `contents` - 要搜索的文本内容
/// 
/// # 返回值
/// 
/// * `Vec<&'a str>` - 包含查询字符串的所有行的向量
/// 
/// # 生命周期
/// 
/// 返回的字符串切片与输入的 `contents` 参数具有相同的生命周期
/// 
/// # 示例
/// 
/// ```
/// let query = "rust";
/// let contents = "rust is great\npython is good\nrust is fast";
/// let results = search(query, contents);
/// assert_eq!(results, vec!["rust is great", "rust is fast"]);
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod test {
    use super::*;
    
    /// 测试搜索功能是否能正确找到匹配的行
    #[test]
    fn one_result() {
        let query = "lbk";
        let contents = "\
Rust:
safe,fast,call,lbk
lak";
        assert_eq!(vec!["safe,fast,call,lbk"], search(query, contents))
    }
}
