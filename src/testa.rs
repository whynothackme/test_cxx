#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("testa.h");

        fn cxx_function() -> i32;
    }
}

pub use ffi::cxx_function;