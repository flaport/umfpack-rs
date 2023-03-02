use num_complex::Complex64;

pub fn as_complex_array_mut(data: &mut [f64]) -> &mut [Complex64] {
    assert_eq!(data.len() % 2, 0, "data must have even length");
    let n = data.len() / 2;
    let complex_data = unsafe {
        let ptr = data.as_mut_ptr() as *mut Complex64;
        std::slice::from_raw_parts_mut(ptr, n)
    };
    return complex_data;
}

pub fn as_complex_array(data: &[f64]) -> &[Complex64] {
    assert_eq!(data.len() % 2, 0, "data must have even length");
    let n = data.len() / 2;
    let complex_data = unsafe {
        let ptr = data.as_ptr() as *const Complex64;
        std::slice::from_raw_parts(ptr, n)
    };
    return complex_data;
}

pub fn as_float_array_mut(data: &mut [Complex64]) -> &mut [f64] {
    let n = data.len() * 2;
    let complex_data = unsafe {
        let ptr = data.as_mut_ptr() as *mut f64;
        std::slice::from_raw_parts_mut(ptr, n)
    };
    return complex_data;
}

pub fn as_float_array(data: &[Complex64]) -> &[f64] {
    let n = data.len() * 2;
    let complex_data = unsafe {
        let ptr = data.as_ptr() as *const f64;
        std::slice::from_raw_parts(ptr, n)
    };
    return complex_data;
}
