use std::thread;
use std::sync;
use std::sync::mpsc::RecvError;

trait Pipe<T> where T: Copy {
    fn push(t: T);
    fn pop() -> T;
}

fn main() {
    let a = vec!(1,2,3);
    let b = vec!(6,4,2);
    let mut c = vec!();
    // we what to get a^2 + b/2 using threads
    // w1 will do a^2
    // w2 will do b/2
    // w3 will get values from w1 and w2 and add those and store them in vector c
    let (sc, rc) = sync::mpsc::channel();
    let (w1, w2, w3);
    {
        let (sa, ra) = sync::mpsc::channel();
        let (sa2, ra2) = sync::mpsc::channel();
        let (sb, rb) = sync::mpsc::channel();
        let (sb2, rb2) = sync::mpsc::channel();
        w1 = thread::spawn(move || loop {
            match ra.recv() {
                Ok(x) => {
                    sa2.send(x*x).unwrap();
                }
                Err(RecvError) => { break; }
            }
        });
        w2 = thread::spawn(move || loop {
            match rb.recv() {
                Ok(x) => {
                    sb2.send(x/2).unwrap();
                }
                Err(RecvError) => { break; }
            }
        });
        w3 = thread::spawn(move || loop {
            match ra2.recv() {
                Ok(x) => {
                    match rb2.recv() {
                        Ok(y) => {
                            sc.send(x+y).unwrap();
                        }
                        _ => break
                    }
                }
                _ => break
            };
        });
        for x in a {
            sa.send(x).unwrap();
        }
        for x in b {
            sb.send(x).unwrap();
        }
    } // here sa is dropped
    w1.join().unwrap();
    w2.join().unwrap();
    w3.join().unwrap();
    loop {
        match rc.recv() {
            Ok(x) => c.push(x),
            _ => break,
        }
    }
    dbg!(c);
}
