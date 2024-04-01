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




pub fn encode_video() {

   println!("Bwpoj");

}