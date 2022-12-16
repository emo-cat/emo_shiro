use anyhow::Error;
use emo_shiro::{print_results_and_save, read_file_to_target, ShiroVerify, EMO_ARGS};
use futures::channel::mpsc::unbounded;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use std::collections::HashSet;




#[tokio::main]
async fn main() {
    match start().await {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
}
async fn burst(mut sv: ShiroVerify) -> ShiroVerify {
    sv.burst_key().await;
    sv
}
async fn start() -> Result<(), Error> {
    let mut targets = HashSet::new();
    if let Some(target) = &EMO_ARGS.target {
        targets.insert(String::from(target));
    }
    if let Some(file_path) = &EMO_ARGS.file {
        targets.extend(read_file_to_target(file_path));
    }
    let mut vec_results: Vec<ShiroVerify> = Vec::new();
    let (verify_sender, mut verify_receiver) = unbounded();
    let (mut results_sender, mut results_receiver) = unbounded();
    let verify_handle = tokio::task::spawn(async move {
        let mut worker = FuturesUnordered::new();
        let mut targets_iter = targets.iter();
        for _ in 0..EMO_ARGS.thread {
            match targets_iter.next() {
                Some(target) => worker.push(ShiroVerify::new(target.to_string())),
                None => {
                    break;
                }
            }
        }
        while let Some(sv) = worker.next().await {
            if let Some(target) = targets_iter.next() {
                worker.push(ShiroVerify::new(target.to_string()));
            }
            verify_sender.unbounded_send(sv).unwrap_or_default();
        }
        true
    });
    let burst_handle = tokio::task::spawn(async move {
        let mut worker = FuturesUnordered::new();
        for _ in 0..3 {
            match verify_receiver.next().await {
                Some(sv) => {
                    worker.push(burst(sv));
                }
                None => {
                    break;
                }
            }
        }
        while let Some(sv) = worker.next().await {
            if let Some(sv) = verify_receiver.next().await {
                worker.push(burst(sv));
            }
            results_sender.start_send(sv).unwrap_or_default();
        }
        true
    });
    let (_r1, _r2) = tokio::join!(verify_handle, burst_handle);
    while let Some(sv) = results_receiver.next().await {
        vec_results.push(sv);
    }
    print_results_and_save(vec_results);
    Ok(())
}
