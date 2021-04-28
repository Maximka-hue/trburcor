use libc::{c_double, c_int};

extern "C" {
    fn smooth_arr_zm_fur(
        Fs: *mut c_double,
        Nmax: c_int,
        smooth_intensity: c_double,
        Fi: *mut c_double,
        Ftd: *mut c_double,
    ) -> c_int;
}

fn main() {
    // Added these since they were missing!
    let width = 5;
    let all_steps = 5;
    let mut inner_vector = vec![];
    
    let smooth_intensity = 0.5;
    let mut prediction = vec![0_f32; width];
    let mut first_correction = vec![0_f32; width];
    let mut second_correction = vec![0_f32; width];
    
    unsafe {
        smooth_arr_zm_fur(
            inner_vector.as_mut_ptr() as *mut f64,
            all_steps as i32,
            smooth_intensity,
            first_correction.as_mut_ptr() as *mut f64,
            second_correction.as_mut_ptr() as *mut f64,
        );
    }
}
