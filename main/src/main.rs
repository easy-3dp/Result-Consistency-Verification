use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use rand::Rng;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

fn main() {

    fs::create_dir_all("obj").unwrap();

    let error = Arc::new(AtomicI32::new(0));
    let total = Arc::new(AtomicI32::new(0));
    let error2 = error.clone();
    let total2 = total.clone();
    thread::spawn(move||{
        (0..5000000).into_par_iter().for_each(|i| {

            let mut rng = rand::thread_rng();

            let trans: Option<[u8; 4]> = if rng.gen() {
                Some([
                    rng.gen::<u8>(),
                    rng.gen::<u8>(),
                    rng.gen::<u8>(),
                    rng.gen::<u8>(),
                ])
            } else {
                None
            };

            let par1 = rng.gen_range(6..=8);
            let par2 = rng.gen_range(6..=12);

            let geo_string = rock_obj::creat();
            let a = p3d::p3d_process(geo_string.as_bytes(), p3d::AlgoType::Grid2dV3a, par1, par2, trans);
            let b = p3d::p3d_process(geo_string.as_bytes(), p3d::AlgoType::Grid2dV3m, par1, par2, trans);
            if a.is_ok() && b.is_ok() {
                let a = a.unwrap();
                let b = b.unwrap();
                if a != b {
                    error.fetch_add(1, Ordering::SeqCst);
                    let file_path = Path::new("obj").join(format!("{i}.txt"));
                    let mut file = File::create(file_path).unwrap();
                    file.write_all(geo_string.as_bytes()).unwrap();
                    println!("-> {i} par1:{par1}, par2:{par2}, par1:{par1}, trans:{trans:?}");
                } else {
                    total.fetch_add(1, Ordering::SeqCst);
                }
            }
        });
    });

    loop {
        thread::sleep(Duration::from_secs(2));
        println!("{:?} {:?}", total2, error2);
    }
}
