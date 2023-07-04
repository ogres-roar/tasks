/// 控制随机值
use rand::distributions::{Distribution, Uniform};
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand_distr::Normal;

// 从 [0, boudary) 的均匀分布中采样
pub fn get_uniform(seed: u64, boundary: u64) -> u64 {
    let mut rng = StdRng::seed_from_u64(seed);
    let between = Uniform::new(0, boundary);

    return between.sample(&mut rng) as u64;
}

// 正态分布采样
#[allow(dead_code)]
pub fn get_norm(seed: u64, center: f32, var: f32) -> f32 {
    let mut rng = StdRng::seed_from_u64(seed);
    let normal = Normal::new(center, var).unwrap();
    return normal.sample(&mut rng);
}

// 从 u64 随机种子 生成 u64
pub fn generate_seed(seed: u64) -> u64 {
    return get_uniform(seed, u64::MAX);
}
