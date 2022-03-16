use rand::Rng;
use std::io::Write;
use std::time::SystemTime;

const N:usize = 24;
const M:usize = 40;
const SITE:usize = N * N;
const STEP:usize = 100000;
const HEAT:usize = 1000;

fn update(spin : &mut [i32;SITE],nbr :&[[usize;4];SITE],t:f64 ) {
    let mut rng = rand::thread_rng();
    for _i in 0..SITE{
        let r:usize = rng.gen_range(0..SITE);
        let s = spin[r] as f64;
        let sum:f64 = -2.0*s*((spin[nbr[r][0]] + spin[nbr[r][1]] + spin[nbr[r][2]] + spin[nbr[r][3]]) as f64)/ t;
        if  rng.gen::<f64>() < sum.exp() {
            spin[r] = -spin[r];
        }
    }
}

fn main() {
    let tic = SystemTime::now();
    let mut spin:[i32;SITE]=[1;SITE];
    //let mut nbr:Vec<[usize;4]> =Vec::new();
    //for i in 0..N {
    //    for j in 0..N{
    //        nbr.push([((i+1)%N)*N+j,((N-1+i)%N)*N+j,i*N+(j+1)%N,i*N+(N-1+j)%N]);
    //    }
    //}
    let mut nbr:[[usize;4];SITE] = [[0;4];SITE];
    for i in 0..N {
        for j in 0..N{
            nbr[i*N+j]=[((i+1)%N)*N+j,((N-1+i)%N)*N+j,i*N+(j+1)%N,i*N+(N-1+j)%N];
        }
    }
    let mut file=std::fs::File::create("1.txt").unwrap();
    for j in 0..M{
        let t:f64 = (j+1) as f64 * 0.1f64;
        for _i in 0..HEAT {
            update(&mut spin,&nbr,t);
        }
        let mut mag_t:f64 = 0.0;
        for _i in 0..STEP{
            update(&mut spin,&nbr,t);
            let sum = spin.iter().sum::<i32>() as f64;
            mag_t += sum.abs();
        }
        mag_t /= (STEP*SITE) as f64;
        // write mag in file 1.txt
        file.write_all(format!("{:.4} {:.4}\n",t,mag_t).as_bytes()).unwrap();

    }
    let toc= SystemTime::now();
    println!("{:?}",toc.duration_since(tic).unwrap());
}
