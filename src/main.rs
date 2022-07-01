use soloud::*;
use std::io;
use std::io::prelude::*;
use flate2::read::ZlibDecoder;

use crossterm::{terminal::{SetSize, self, ClearType}, execute};
use std::io::{stdout};


fn cur_timestamp_micros() -> i128{
    std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .unwrap()
    .as_micros()
    .try_into()
    .unwrap_or(0)
}

// use zlib uncompress chars graphic
// original data length = 12774024
// compress data length = 1187473
fn zlib_decode_bufreader(bytes: Vec<u8>) -> io::Result<String> {
    let mut z = ZlibDecoder::new(&bytes[..]);
    let mut s = String::new();
    z.read_to_string(&mut s)?;
    Ok(s)
}

fn read_full_video() -> Vec<String>{
    let data = include_bytes!("charvideo_cp.txt");

    let data = zlib_decode_bufreader(data.to_vec()).unwrap().as_bytes().to_vec();

    let mut ret = vec![];

    let frame_size = 44*117;

    for i in 0..6571{
        let frame = String::from_utf8(data[i*frame_size..(i+1)*frame_size].to_vec()).unwrap();
        ret.push(frame);
    }

    ret
}

fn main() {
    let frames = read_full_video();

    let sl = Soloud::default().unwrap();

    let mut wav = audio::Wav::default();

    let wav_data = include_bytes!("BadApple.mp3");

    wav.load_mem(wav_data).unwrap();

    // calls to play are non-blocking, so we put the thread to sleep
    sl.play(&wav); 

    // set window size
    execute!(stdout(), SetSize(116, 44)).unwrap();

    // all video time = 219 secs = 219000 ms
    // so one frame time = 219000 / 6571 ms = 33.328 ms -33328 micros

    let start = cur_timestamp_micros();

    for i in 0..6571 {
        execute!(stdout(), terminal::Clear(ClearType::All)).unwrap();

        print!("{}" , frames[i]);

        let cur = cur_timestamp_micros();

        // control progress
        if  i as i128 * 33328 - (cur - start) > 0 { 
            std::thread::sleep(std::time::Duration::from_micros((i as i128 * 33328 - (cur - start)) as u64));
        }
    }
}