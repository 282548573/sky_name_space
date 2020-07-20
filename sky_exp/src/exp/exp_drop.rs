// use std::ops::Deref;
use std::thread;
use std::time::Duration;

pub  fn main_drop() {
    test();
}
struct DerefExample{}
impl Drop for DerefExample {
    fn drop(&mut self) {
        println!("drop");
    }
}
fn test() {
    for j in 0..10 {
        thread::spawn(move||{
            test2();
            thread::sleep(Duration::from_secs(2))
        });
    }
    thread::sleep(Duration::from_secs(20));
}

fn test2(){
    let sky = DerefExample{

    };
}