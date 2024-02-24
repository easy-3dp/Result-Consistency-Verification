#![allow(non_snake_case)]

use bufferGeometry::BufferGeometry;
use cgmath::{MetricSpace, Vector3};
use perlin::Perlin;
use std::collections::VecDeque;

pub mod sphere;
pub mod scrape;
pub mod perlin;
pub mod bufferGeometry;

extern crate rand;
use rand::{rngs::ThreadRng, Rng};



pub fn creat() -> String {
    let mut rng = rand::thread_rng();
    let rock_obj_params = vary_mesh(&mut rng);
    let mut rock = Rock(&mut rng, rock_obj_params);
    rock.ComputeVertexNormals();
    rock.parse()
}


fn Rock(rng: &mut ThreadRng, rock_obj_params:RockObjParams) -> BufferGeometry
{
    let (mut positions, indices, mut normals) = sphere::CreatSphere(1.0, 30, 30);

    // OPTIMIZATION: we are always using the same sphere as base for the rock,
    // so we only need to compute the adjacent positions once.
    let adjacentVertices = scrape::GetNeighbours(positions.len(), &indices);

    // 这段代码是在查找一些随机位置，并确保它们之间的距离不太近。
    // 它会随机选择一个位置，然后检查它与其他已选择的位置之间的距离是否小于某个最小值（rock_obj_params.scrapeMinDist）。
    // 如果是，则重新尝试找一个位置，否则就将它添加到选择的位置列表中。
    // 如果尝试次数超过了一定次数（100次），则无论它与其他位置的距离是否太近都将其添加到列表中，以避免无限循环。
    let mut scrapeIndices: Vec<usize> = Vec::with_capacity(rock_obj_params.scrapeCount as usize);
    let len = positions.len();
    for _ in 0..rock_obj_params.scrapeCount {
        let mut attempts = 0;

        // find random position which is not too close to the other positions.
        loop {
            let randIndex = rng.gen_range(0..len);
            let p = positions[randIndex];

            let mut tooClose = false;
            // check that it is not too close to the other vertices.
            for i in scrapeIndices.iter() {
                let q = positions[*i];
                if p.distance2(q) < rock_obj_params.scrapeMinDist {
                    tooClose = true;
                    break;
                }
            }
            attempts=attempts+1;

            // if we have done too many attempts, we let it pass regardless.
            // otherwise, we risk an endless loop.
            if tooClose && attempts < 100 {
                continue;
            } else {
                scrapeIndices.push(randIndex);
                break;
            }
        }
    }

    let mut traversed = vec![false; positions.len()];
    let mut stack = VecDeque::with_capacity(100);
    // now we scrape at all the selected positions.
    for si in scrapeIndices {
        traversed.iter_mut().for_each(|x| *x = false);
        stack.clear();
        scrape::Main(si, &mut positions, &mut normals, &adjacentVertices, rock_obj_params.scrapeStrength, rock_obj_params.scrapeRadius, &mut traversed, &mut stack);
    }

    /*
     * Finally, we apply a Perlin noise to slightly distort the mesh and then scale the mesh.
     */
    let perlin = Perlin::new(rng.gen());
    for p in positions.iter_mut() {
        
        let noise = rock_obj_params.meshNoiseStrength * perlin.Noise(rock_obj_params.meshNoiseScale * p[0], rock_obj_params.meshNoiseScale * p[1], rock_obj_params.meshNoiseScale * p[2]);

        p[0] += noise;
        p[1] += noise;
        p[2] += noise;

        p[0] *= rock_obj_params.scale[0];
        p[1] *= rock_obj_params.scale[1];
        p[2] *= rock_obj_params.scale[2];

        *p=*p*3.0;

        p[0] = ((p[0] + std::f64::EPSILON) * 100.0).round() / 100.0;
        p[1] = ((p[1] + std::f64::EPSILON) * 100.0).round() / 100.0;
        p[2] = ((p[2] + std::f64::EPSILON) * 100.0).round() / 100.0;
    }

   
    BufferGeometry::new(positions, FlatU32(indices))
}

// fn FlatF64(date:Vec<Vector3<f64>>) -> Vec<f64> {
//     let mut flat = Vec::with_capacity(date.len()*3);
//     for i in 0..date.len() {
//         flat[i*3+0] = date[i][0];
//         flat[i*3+1] = date[i][1];
//         flat[i*3+2] = date[i][2];
//     }
//     flat
// }

fn FlatU32(date:Vec<Vector3<u32>>) -> Vec<u32> {
    let mut flat = vec![0; date.len()*3];
    for i in 0..date.len() {
        flat[i*3+0] = date[i][0];
        flat[i*3+1] = date[i][1];
        flat[i*3+2] = date[i][2];
    }
    flat
}

#[allow(non_snake_case)]
pub struct RockObjParams {
    pub meshNoiseScale:f64,
    pub meshNoiseStrength:f64,
    pub scrapeCount:i32,
    pub scrapeMinDist:f64,
    pub scrapeStrength:f64,
    pub scrapeRadius:f64,
    pub scale:[f64;3],
}

const  MESH_NOISE_SCALE_MIN:f64 = 0.4f64;
const  MESH_NOISE_SCALE_MAX:f64 = 1.0f64;
const  MESH_NOISE_SCALE_VARY:f64 = 0.05f64;

const  MESH_NOISE_STRENGTH_MIN:f64 = 0.0f64;
const  MESH_NOISE_STRENGTH_MAX:f64 = 0.5f64;
const  MESH_NOISE_STRENGTH_VARY:f64 = 0.3f64;

const SCRAPE_COUNT_MIN:f64 = 0.0f64;
const SCRAPE_COUNT_MAX:f64 = 15.0f64;
const SCRAPE_COUNT_VARY:f64 = 3.0f64;

const  SCRAPE_MIN_DIST_MIN:f64 = 0.2f64;
const  SCRAPE_MIN_DIST_MAX:f64 = 1.0f64;
const  SCRAPE_MIN_DIST_VARY:f64 = 0.05f64;

const  SCRAPE_STRENGTH_MIN:f64 = 0.1f64;
const  SCRAPE_STRENGTH_MAX:f64 = 0.6f64;
const  SCRAPE_STRENGTH_VARY:f64 = 0.02f64;

const  SCRAPE_RADIUS_MIN:f64 = 0.1f64;
const  SCRAPE_RADIUS_MAX:f64 = 0.5f64;
const  SCRAPE_RADIUS_VARY:f64 = 0.05f64;

fn vary_mesh(rng: &mut ThreadRng) -> RockObjParams
{
    RockObjParams {
        meshNoiseScale:     vary_parameter(1.0 , MESH_NOISE_SCALE_VARY   , MESH_NOISE_SCALE_MIN   , MESH_NOISE_SCALE_MAX   , rng),
        meshNoiseStrength:  vary_parameter(0.5 , MESH_NOISE_STRENGTH_VARY, MESH_NOISE_STRENGTH_MIN, MESH_NOISE_STRENGTH_MAX, rng),
        scrapeCount:        vary_parameter(7.0 , SCRAPE_COUNT_VARY       , SCRAPE_COUNT_MIN       , SCRAPE_COUNT_MAX       , rng) as i32,
        scrapeMinDist:      vary_parameter(0.8 , SCRAPE_MIN_DIST_VARY    , SCRAPE_MIN_DIST_MIN    , SCRAPE_MIN_DIST_MAX    , rng),
        scrapeStrength:     vary_parameter(0.05, SCRAPE_STRENGTH_VARY    , SCRAPE_STRENGTH_MIN    , SCRAPE_STRENGTH_MAX    , rng),
        scrapeRadius:       vary_parameter(0.1 , SCRAPE_RADIUS_VARY      , SCRAPE_RADIUS_MIN      , SCRAPE_RADIUS_MAX      , rng),
        scale: [
            vary_parameter(1.0, 0.1, 0.1, 2.0, rng),
            vary_parameter(1.0, 0.1, 0.1, 2.0, rng),
            vary_parameter(1.2, 0.1, 1.0, 1.2, rng),
        ],
    }
}

fn vary_parameter(mut param: f64, variance: f64, min: f64, max: f64, rng: &mut ThreadRng) -> f64
{
    param += get_random_value(-variance * 1.0f64, variance * 1.0f64, rng);
    param.min(max).max(min)
}
fn get_random_value(min: f64, max: f64, rng: &mut ThreadRng) -> f64
{
    let random: f64 = rng.gen(); 
    random * (max - min) + min
}
