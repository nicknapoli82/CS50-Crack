// compile with -O for optimized code
// rustc -O crack.rs -lcrypt

use std::env;
use std::process;
use std::ffi::CString;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::string::String;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: ./crack password");
        process::exit(1);
    }

    let key = match CString::new(args[1].to_string()) {
        Ok(str) => str,
        Err(_) => process::exit(1),
    };

    let mut password: [c_char; 8] = [0; 8];
    let pass_ptr: *const c_char = &password[0];

    let salt = CString::new("50").expect("Salt FAIL");
    let mut index_position = 0;
    let mut pass_found = false;

    while !pass_found {
        for i in b'A'..=b'z'{
            password[0] = i as c_char;

            unsafe{
                let test = CStr::from_ptr(crypt(pass_ptr, salt.as_ptr()));
                let c = CString::from(test);
                if c == key {
                    pass_found = true;
                    break;
                }
            }
        }

        if pass_found == true { break };
        
        while password[index_position] == b'z' as c_char {
            password[index_position] = b'A' as c_char;
            index_position += 1;
            if index_position == 8 {
                println!("Password not found!");
                return ();
            }
        }
//        println!("{:?}", password);
        if password[index_position] == 0 {
            password[index_position] = b'A' as c_char;
        }
        else { password[index_position] += 1; }
        index_position = 0;
    }

    for i in password.into_iter() {
        let n = *i as u8;
        if n != 0 {
            print!("{}", n as char);
        }
    }
    print!("\n");
}


extern {
    fn crypt(key: *const c_char, salt: *const c_char) -> *const c_char;
}
