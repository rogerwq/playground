use std::os::raw::c_char;
use std::ffi::CString;

#[no_mangle]
pub extern fn add_numbers(number1: i32, number2: i32) -> i32 {
    println!("Hello from Rust lib");
    number1 + number2
}

static mut STRING_POINTER: *mut c_char = 0 as *mut c_char;

#[repr(C)]
pub struct SampleStruct {
    pub field_one: i16,
    pub field_two: i32,
    pub string_field: *mut c_char,
}

#[no_mangle]
pub extern fn get_simple_struct() -> SampleStruct {
    let test_string: &'static str = "Hi, I'am a string in rust";
    SampleStruct {
        field_one: 1,
        field_two: 2,
        string_field: store_string_on_heap(test_string),
    }
}

fn store_string_on_heap(string_to_store: &'static str) -> *mut c_char {
    let pntr = CString::new(string_to_store).unwrap().into_raw();
    unsafe {
        STRING_POINTER = pntr;
    }

    return pntr;
}

#[no_mangle]
pub extern fn free_string() {
    unsafe {
        // let _ = CString::from_raw(STRING_POINTER);
        STRING_POINTER = 0 as *mut c_char;
    }
}
