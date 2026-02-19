use noiselib::{
    fractal::fractal_noise_add_2d,
    perlin::perlin_noise_2d,
    prelude::UniformRandomGen,
};

pub fn terrain_noise_2d(
    rng: &mut UniformRandomGen,
    x: f32,
    y: f32,
    seed: u32,
) -> f32
{
    // more octaves means more detail (rocky / jagged)
    let octaves = 6;

    // how much each successive layer contributes 0.5 means each one is half as
    // much as the previous
    let freq_falloff = 0.5;

    // lacunarity dictates how quickly the frequency increases per octave.
    // 2.0 means the detail level doubles each time.
    let lacunarity = 2.0;

    fractal_noise_add_2d(
        rng, x, y, perlin_noise_2d, octaves, freq_falloff, lacunarity, seed,
    )
}
