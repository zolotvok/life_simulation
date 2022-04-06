use rand::Rng;
use std::{thread, time};
fn main(){
EletjatekSzimulator::eletjatek_szimulator(500, 500);
}
struct EletjatekSzimulator;
impl EletjatekSzimulator{
    fn eletjatek_szimulator(x:usize,y:usize){
    let mut matrix = vec![vec![0;x+2];y+2];
    let mut matrix1 = vec![vec![0;x+2];y+2];
    for n in 1..x-1{
        for b in 1..y-1{
            matrix[n][b]= rand::thread_rng().gen_range(0..2);
        }
    }
    
    println!("{:?}",matrix);
    loop{
    let ten_millis = time::Duration::from_millis(500);
let now = time::Instant::now();

thread::sleep(ten_millis);
print!("\x1B[2J\x1B[1;1H");
    
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
                
                if matrix[n+1][b]==1{
                    szomszed+=1;
                }
                if matrix[n-1][b]==1{
                    szomszed+=1;
                }
                if matrix[n][b+1]==1{
                    szomszed+=1;
                }
                if matrix[n][b-1]==1{
                    szomszed+=1;
                }
                if matrix[n+1][b-1]==1{
                    szomszed+=1;
                }
                if matrix[n+1][b+1]==1{
                    szomszed+=1;
                }
                if matrix[n-1][b-1]==1{
                    szomszed+=1;
                }
                if matrix[n-1][b+1]==1{
                    szomszed+=1;
                }
                if szomszed <2 {
                    matrix1[n][b]=0;
                }
                if szomszed >3{
                    matrix1[n][b]=0;
                }
            }
            if matrix[n][b]==0{
                if matrix[n+1][b]==1{
                    szomszed+=1;
                }
                if matrix[n-1][b]==1{
                    szomszed+=1;
                }
                if matrix[n][b+1]==1{
                    szomszed+=1;
                }
                if matrix[n][b-1]==1{
                    szomszed+=1;
                }
                if matrix[n+1][b-1]==1{
                    szomszed+=1;
                }
                if matrix[n+1][b+1]==1{
                    szomszed+=1;
                }
                if matrix[n-1][b-1]==1{
                    szomszed+=1;
                }
                if matrix[n-1][b+1]==1{
                    szomszed+=1;
                }
                
                if szomszed ==3{
                    matrix1[n][b]=1;
                }
            }
            
        }
        
    }
    for n in 1..x-1{
        for b in 1..y-1{
            if matrix1[n][b]==1{
                matrix[n][b]=1;
            }
            else{
                matrix[n][b]=0;
            }
            
        }}
       
        for n in 0..x{
            println!("");
            for b in 0..y{
                if n==0||b==0||n==x-2||b==y-1{
                    print!("X");
                    continue;
                }
                if matrix1[n][b]==0{
                    print!(" ");
                }
                else{
                    print!("S")
                }
                
            }
        }
        }
    }
}
