use std::fs::File;
use std::io::Read;


pub fn read_frames(width: usize, height: usize) -> Vec<Vec<u8>> {

    let mut result: Vec<Vec<u8>> = Vec::new(); //create new frame object
 
    let mut file = File::open("video.rgb24").expect("Error reading file"); //read entire file
 
    let chunks_size = width * height * 3; //get single frame length (w * h * (r+g+b))
    loop {
 
        let mut frame = Vec::with_capacity(chunks_size); //create container
 
        let a = file //read frame
            .by_ref()
            .take(chunks_size as u64)
            .read_to_end(&mut frame)
            .expect("Error rerading rgb24");
 
         if a == 0 {
             break;
         }
 
         result.push(frame);//stuff into result

    }
     return result;
 }


 pub fn rbg_yuv(width: usize,height: usize,mut video: Vec<Vec<u8>>) ->Vec<Vec<u8>>{
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

    
    };

    return video;
 }