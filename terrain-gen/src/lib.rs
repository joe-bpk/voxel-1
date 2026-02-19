mod terrain_noise;
use noiselib;

/// # category
/// **client side processing**
///
/// basic voxel unit.
#[derive(Copy, Clone)]
pub struct Block
{
    pub block_id: usize,
}

pub struct WorldCfg
{
    pub world_size_b: usize,
    pub world_height: usize,
    pub seed:         u32,
}

pub fn block_gen(x: i32, y: i32, z: i32, cfg: WorldCfg) -> Block
{
    let mut rng = noiselib::prelude::UniformRandomGen::new(cfg.seed);
    let perlin_out = terrain_noise::terrain_noise_2d(
        &mut rng,
        (x as f32) / cfg.world_size_b as f32,
        (z as f32) / cfg.world_size_b as f32,
        cfg.seed,
    );

    let perlin_out_normal = (perlin_out + 1.0) / 2.0;
    let block_id = if (perlin_out_normal * (cfg.world_height as f32)) > y as f32
    {
        1
    } else {
        0
    };

    return Block {
        block_id,
    };
}

pub fn add(left: u64, right: u64) -> u64
{
    left + right
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn it_works()
    {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
