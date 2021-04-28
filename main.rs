pub(crate) use std::path::{self, PathBuf, Path};
extern crate libc;
extern crate cc;
use libc::{c_double, c_int};

//#[link(name = "smooth_correct_utf")]
extern "C" {
    fn smooth_arr_zm_fur(Fs:*mut c_double, Nmax: c_int, smooth_intensity: c_double, Fi: *mut c_double, Ftd: *mut c_double) ->  c_int;
}

fn main() {
    let env_vars = env::vars();
    for(key, value) in env_vars.into_iter() {
        println!("{} = {:?}", key, value);
    }
    cc::Build::new()
        .file("src/smooth_correct_utf.c")
        .compile("smooth_correct_utf");
    println!("Hello, world!, {}",  env::var("OUT_DIR").unwrap());
    let all_steps: i32 = 1000;
    let width = all_steps as usize;
    let mut exact_solvec = vec![vec![0_f32; (all_steps + 2) as usize], vec![0_f32;(all_steps + 2) as usize], vec![0_f32;(all_steps + 2) as usize]];
    let mut first_ex = exact_solvec[0].clone();
    let mut second_ex = exact_solvec[1].clone();
    let mut inner_vector = vec![0_f32; all_steps as usize + 2 as usize];
    let mut prediction = vec![0_f32; width];
    let mut first_correction = vec![0_f32; width];
    let mut second_correction = vec![0_f32; width];
    let smooth_intensity = 0.5;
    //unsafe{  smooth_arr_zm_fur(inner_vector.as_mut_ptr() as *mut f64, all_steps as i32, smooth_intensity,
    //    first_correction.as_mut_ptr() as *mut f64, second_correction.as_mut_ptr() as *mut f64);}
}
