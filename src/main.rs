#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::io;
use std::io::prelude::*;
use flate2::read::ZlibDecoder;
#[cfg(not(target_os = "windows"))]
use soloud::*;

use std::alloc::System;

#[global_allocator]
static A: System = System;

fn cur_timestamp_micros() -> i128{
    std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .unwrap()
    .as_micros()
    .try_into()
    .unwrap_or(0)
}

// use zlib uncompress chars graphics
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

    //6572 frames
    for i in 0..6572{
        let frame = String::from_utf8(data[i*frame_size..(i+1)*frame_size].to_vec()).unwrap();
        ret.push(frame);
    }

    ret
}

#[cfg(not(target_os = "windows"))]
fn play_sound(){
    let sl = Soloud::default().unwrap();

    let mut wav = audio::Wav::default();
    let wav_data = include_bytes!("BadApple.mp3");
    wav.load_mem(wav_data).unwrap();

    // calls to play are non-blocking, so we put the thread to sleep
    sl.play(&wav); 
}
#[cfg(target_os = "windows")]
fn play_sound(){

    let data = include_bytes!("BadApple.wav");

    let mut file = std::fs::File::create("BadApple.wav").unwrap();
    file.write_all(data).unwrap();

    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    let wide_file_name: Vec<u16> = OsStr::new("BadApple.wav").encode_wide().chain(once(0)).collect();
    unsafe { winapi::um::playsoundapi::PlaySoundW(wide_file_name.as_ptr(), std::ptr::null_mut() , winapi::um::playsoundapi::SND_ASYNC) };
}

use eframe::egui;

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.resizable = false;
    options.initial_window_size = Some(egui::Vec2::new(834.0,640.0));
    eframe::run_native(
        "BadApple by b23r0 # https://github.com/b23r0",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    );
}

struct MyApp {
    frames: Vec<String> ,
    start_time: i128,
    cur_frame : u32,
}

impl MyApp{

    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let ret = Self {
            frames: read_full_video(),
            // all video time = 219 secs = 219000 ms
            // so one frame time = 219000 / 6572 ms = 33.323 ms -33323 micros
            start_time: cur_timestamp_micros(),
            cur_frame : 0,
        };
        play_sound();
        ret
    }

    fn update_frame(&mut self , ctx: &egui::Context){
        if self.cur_frame == 6571 {
            return;
        }

        // control progress
        if  self.cur_frame as i128 * 33323 - (cur_timestamp_micros() - self.start_time) > 0 { 
            std::thread::sleep(std::time::Duration::from_micros((self.cur_frame as i128 * 33323 - (cur_timestamp_micros() - self.start_time)) as u64));
        }
        self.cur_frame += 1;
        ctx.request_repaint();
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(egui::RichText::new(&self.frames[self.cur_frame as usize]).text_style(egui::TextStyle::Monospace).strong());
            self.update_frame(ctx);
        });
    }
}