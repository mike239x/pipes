// to format codebase use `$ cargo fmt`

use std::sync;
use std::sync::mpsc::{Receiver, RecvError, Sender};
use std::thread;

struct SquaringWorkerParams {
    input: Receiver<f32>,
    output: Sender<f32>,
}

#[allow(non_snake_case)]
fn SquareWorker(param: SquaringWorkerParams) {
    loop {
        match param.input.recv() {
            Ok(x) => {
                param.output.send(x * x).unwrap();
            }
            Err(RecvError) => {
                break;
            }
        }
    }
}

fn square_things(param: SquaringWorkerParams) -> thread::JoinHandle<()> {
    let worker = thread::spawn(move || SquareWorker(param));
    return worker;
}

fn half_things(input: Receiver<f32>, output: Sender<f32>) -> thread::JoinHandle<()> {
    let worker = thread::spawn(move || loop {
        match input.recv() {
            Ok(x) => {
                output.send(x / 2f32).unwrap();
            }
            Err(RecvError) => {
                break;
            }
        }
    });
    return worker;
}

fn add_things(
    input_a: Receiver<f32>,
    input_b: Receiver<f32>,
    output: Sender<f32>,
) -> thread::JoinHandle<()> {
    let worker = thread::spawn(move || loop {
        match input_a.recv() {
            Ok(x) => match input_b.recv() {
                Ok(y) => {
                    output.send(x + y).unwrap();
                }
                _ => break,
            },
            _ => break,
        };
    });
    return worker;
}

fn main() {
    let a = vec![1f32, 2f32, 3f32];
    let b = vec![6f32, 4f32, 2f32];
    let mut c = vec![];
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
        w1 = square_things(SquaringWorkerParams {
            input: ra,
            output: sa2,
        });
        w2 = half_things(rb, sb2);
        w3 = add_things(ra2, rb2, sc);
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
