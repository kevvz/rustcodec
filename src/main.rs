use std::fs::File;
use std::io::Read;
mod encoder;
use std::fs::OpenOptions;
use std::io::Write;
fn main() {


    
    encoder::encode_video();
    let width: usize = 384;
    let height: usize = 216;

    let mut video = encoder::read_frames(width,height);


    println!("Hello, world!");

  //  println!("{}",video.len());

    //Encoding phase
    //Part one: RBG to YUV  conversion
    for i in 0..video.len() {
        println!("{}",i);

        let mut Y: Vec<u8> = Vec::with_capacity(width*height);
        let mut U: Vec<f64> = Vec::with_capacity(width*height);
        let mut V: Vec<f64> = Vec::with_capacity(width*height);

        for j in 0..(height*width) {
            let r = video[i][3*j] as f64;
            let g = video[i][3*j+1] as f64;
            let b = video[i][3*j+2] as f64;

            let y =  0.299 * r + 0.587 * g + 0.114 * b;
            let u = -0.169 * r - 0.331 * g + 0.449 * b +128.0;
            let v =  0.499 * r - 0.418 * g - 0.0813 * b + 128.0;

            Y.push(y as u8);
            U.push(u);
            V.push(v);
            



            

        };

        let mut uDS: Vec<u8> = Vec::with_capacity(width*height/4);
        let mut vDS: Vec<u8> = Vec::with_capacity(width*height/4);

        let x = height/2 as usize;
        let y = width/2 as usize;

        for j in 0..x {
            for k in 0..y {
             let u = (U[j*width+k*2] +  U[(j+1)*width+k*2]+U[(j+1)*width+k*2+1]+U[j*width+k*2+1])/4.0;

             let v = (V[j*width+k*2] +  V[(j+1)*width+k*2]+V[(j+1)*width+k*2+1]+V[j*width+k*2+1])/4.0;

             uDS.push(u as u8);
             vDS.push(v as u8);
            
            }
        }

        let mut result: Vec<u8> = Vec::with_capacity(uDS.len()+vDS.len()+Y.len());

        for j in 0..Y.len() {
            result.push(Y[j]);
        }

        for j in 0..uDS.len() {
            result.push(uDS[j]);
        }

        for j in 0..vDS.len() {
            result.push(vDS[j]);
        }


        video[i] = result;

        //
    };

    let mut file_ = OpenOptions::new()
    .create(true)
    .write(true)
    .open("result.yuv")
    .expect("Error writing");

    for frames in video {
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

