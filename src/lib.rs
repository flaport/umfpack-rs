mod c {
    extern "C" {
        pub fn example() -> i32;
    }
}

pub fn example() -> i32 {
    unsafe {
        c::example()
    }
}
