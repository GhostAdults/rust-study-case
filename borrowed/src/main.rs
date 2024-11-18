use std::result;
use std::ffi::CString;
use std::os::raw::{c_char, c_int};

use libloading::{Library, Symbol};

#[tokio::main]
async fn main() -> result::Result<(), Box<dyn std::error::Error + 'static>> {
    #[cfg(target_os = "windows")]
    unsafe {
        let lib = Library::new("db_oppration.dll").unwrap();
        println!("加载dll成功..");

        let copy_users: Symbol<unsafe extern "C" fn(*const c_char) -> c_int> = lib.get(b"copy_users").expect("无法获取copy_users函数");

        let db_url = match CString::new("mysql://root:123456@localhost:3306/my_db") {
                Ok(url) => url,
                Err(e) => {
                    println!("创建CString失败: {}", e);
                    return Err(e.into());
                }
        };
        let result = copy_users(db_url.as_ptr());
        
        if result != 0 {
            return Err(format!("复制用户数据失败: {}", result).into());
            
        } else {
            println!("数据已成功从'user'表复制到'user_copy'表。");            
        }
    };

    Ok(())
}
