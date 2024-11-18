
// db_operations/src/lib.rs

use std::result;
use mysql::Pool;
use mysql::Opts;
use mysql::params;
use mysql::prelude::Queryable;
use std::ffi::{CStr,c_char,c_int};

#[derive(Debug)]
pub struct User {
    pub name: String,
    pub age: u32,
    pub sex: String,
}


#[no_mangle]
pub extern "C" fn copy_users(db_url: *const c_char) -> c_int {
    let db_url = unsafe {   
        assert!(!db_url.is_null());
   
    match CStr::from_ptr(db_url).to_str() {
        Ok(db_url) => db_url,
        Err(_) => return -1,
    }
};
    let opts = match Opts::from_url(db_url) {
        Ok(opts) => opts,
        Err(_) => return -1,
    };
    let pool = match Pool::new(opts) {
        Ok(pool) => pool,
        Err(_) => return -1,
    };

    if let Err(_) = copy_users_internal(&pool) {
        return -1;
    }

    0
}

pub fn copy_users_internal(pool: &Pool) -> result::Result<(), Box<dyn std::error::Error>> {
    // 从 user 表中获取数据
    let mut conn = pool.get_conn()?;
    let users: Vec<User> = conn.query_map(
      "SELECT name, age, sex FROM user_t",
        |(name, age, sex)| User { name, age, sex },
    )?;

    // 将数据插入到 user_copy 表中
    for user in &users {
        conn.exec_drop(
            "INSERT INTO user_t_copy (name, age, sex) VALUES (:name, :age, :sex)",
            params! {
                "name" => &user.name,
                "age" => &user.age,
                "sex" => &user.sex,
            },
        )?;
    }

    Ok(())
}