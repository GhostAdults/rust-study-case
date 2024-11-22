use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::os::windows::process;
use std::result;
use std::error::Error;

use libloading::{Library, Symbol};

fn main() {
    match start_into() {
        Ok(()) => println!("data inserted!"),
        Err(e) => {
            eprintln!("{e}")
        }
    }
    std::process::exit(0)
}

fn start_into() -> result::Result<(), Box<dyn Error + 'static>> {
    #[cfg(target_os = "windows")]
    unsafe {
        let lib = match Library::new("db_oppration.dll") {
            Ok(lib) => lib,
            Err(e) => return Err(format!("无法获取copy_users函数:{}", e).into()),
        };
        let copy_users: Symbol<unsafe extern "C" fn(*const c_char) -> c_int> =
            lib.get(b"copy_users").expect("msg");

        let db_url = match CString::new("mysql://root:123456@localhost:3306/my_db") {
            Ok(url) => url,
            Err(e) => {
                return Err(format!("没有找到对应数据库: {}", e).into());
            }
        };

        let result = copy_users(db_url.as_ptr());
        if result != 0 {
            return Err(format!("复制用户数据失败: {}", result).into());
        }
        Ok(())
    }
}
