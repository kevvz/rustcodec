use std::fs::File;
use std::io::Read;
mod encoder;
use std::fs::OpenOptions;
use std::io::Write;
fn main() {


   // encoder::encode_video();
    let width: usize = 384;
    let height: usize = 216;

    let mut video = encoder::read_frames(width,height);

    println!("Hello, world!");

    //Encoding phase
    //Part one: RBG to YUV  conversion

    let video2 = encoder::rbg_yuv(width,height,video.clone());


    let mut file_ = OpenOptions::new()
    .create(true)
    .write(true)
    .open("result.yuv")
    .expect("Error writing");

    for frames in video2 {
        file_.write_all(&frames).unwrap();
    }



    //Part two: delta frame + sparsity
    //naive sparsity compression: you can compress a sonsecutive set of zeros into 

    // let mut encoded: Vec<Vec<u8>> = Vec::with_capacity(video.len());

    // encoded.push(video[0]);
    // for i in 1..video.len() {
    // let mut delta: Vec<u8> = Vec::with_capacity(video[i].len());
    // for j in 0..delta.len() {
    //     delta[j] = video[i][j] - video[i-1][j];
    // };

    // for j in 0..delta.len() {

    // };
    



    // };








    


    println!("lmao!");





}

