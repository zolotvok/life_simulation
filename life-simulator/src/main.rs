use rand::Rng;
use std::{thread, time};
fn main() {
    EletjatekSzimulator::Run(50,50);
}
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
struct EletjatekSzimulator;
impl  EletjatekSzimulator{
    
    fn EletjatekSzimulator(x:usize,y:usize){
      let mut matrix = vec![vec![0;x+2];y+2];
      for n in 1..x-1{
        for b in 1..y-1{
            matrix[n][b]= rand::thread_rng().gen_range(0..2);
        }
    }
       EletjatekSzimulator::KovetkezoAllapot(&mut matrix,x,y);
    }
    fn KovetkezoAllapot(mut matrix:&mut Vec<Vec<i32>>,x:usize,y:usize){
        loop{
        let ten_millis = time::Duration::from_millis(500);
        let now = time::Instant::now();

thread::sleep(ten_millis);
print!("\x1B[2J\x1B[1;1H");
        let mut matrix1 = vec![vec![0;x+2];y+2];
        for n in 1..x-1{
            for b in 1..y-1{
                if matrix[n][b]==1{
                    matrix1[n][b]=1;
                }
            }}
            for n in 1..x-1{
                for b in 1..y-1{
                    let mut szomszed = 0;
                    if matrix[n][b]==1{
                        szomszed+=matrix[n+1][b];
                        szomszed+=matrix[n-1][b];
                        szomszed+=matrix[n][b+1];
                        szomszed+=matrix[n][b-1];
                        szomszed+=matrix[n+1][b+1];
                        szomszed+=matrix[n+1][b-1];
                        szomszed+=matrix[n-1][b-1];
                        szomszed+=matrix[n-1][b+1];
                        
                        if szomszed <2 {
                            matrix1[n][b]=0;
                        }
                        if szomszed >3{
                            matrix1[n][b]=0;
                        }
                    }
                    if matrix[n][b]==0{
                        szomszed+=matrix[n+1][b];
                        szomszed+=matrix[n-1][b];
                        szomszed+=matrix[n][b+1];
                        szomszed+=matrix[n][b-1];
                        szomszed+=matrix[n+1][b+1];
                        szomszed+=matrix[n+1][b-1];
                        szomszed+=matrix[n-1][b-1];
                        szomszed+=matrix[n-1][b+1];
                        if szomszed ==3 {
                            matrix1[n][b]=1;
                        }
                }}

                for n in 1..x-1{
                    for b in 1..y-1{
                        if matrix1[n][b]==1{
                            matrix[n][b]=1;
                        }
                        else{
                            matrix[n][b]=0;
                        }
                        
                    }
                   
                    }
                     
                    }
                    EletjatekSzimulator::Megjelenit(matrix,x,y);
    }}
    fn Megjelenit(matrix:&mut Vec<Vec<i32>>,x:usize,y:usize){
        for n in 0..x{
            println!();
            for b in 0..y{
                if n==0||b==0||n==x-2||b==y-1{
                    print!("X");
                    continue;
                }
                if matrix[n][b]==0{
                    print!(" ");
                }
                else{
                    print!("S")
                }
                
            }
        }
    }
    
    fn Run(x:usize,y:usize){
        EletjatekSzimulator::EletjatekSzimulator(x,y);

    }
}
