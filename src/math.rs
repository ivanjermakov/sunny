pub fn sq_diff_root(a: f32, b: f32) -> f32 {
    (a * a - b * b).sqrt()
}

pub fn approx_eq(a: f32, b: f32) -> bool {
    let e = 0.01;
    (a - b).abs() <= e
}
