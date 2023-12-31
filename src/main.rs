use nokhwa::{
    nokhwa_initialize,
    pixel_format::{RgbAFormat, RgbFormat},
    query,
    utils::{ApiBackend, ApiBackend::MediaFoundation, RequestedFormat, RequestedFormatType},
    CallbackCamera,
};
use nokhwa_core::buffer::Buffer;
use image::{ImageBuffer, Rgba, RgbImage, GenericImageView};
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions, Scale, ScaleMode, Scale::FitScreen};
use std::{thread, time, ops::Sub};
use minifb_fonts::*;


//const WINHIGH:usize = 1180;
//const WINWIDE:usize  = 2048;
const WINHIGH:usize = 1080;
const WINWIDE:usize  = 1920;

const CAMHIGH:usize = 960;
const CAMWIDE:usize = 1280;

fn shrink(value: usize, low: usize, high: usize)-> usize {
    let l = low as u128;
    let h = high as u128;
    let v = value as u128;
    
    let o = l * v / h;
    let output = o as usize;
    output
}

fn pixeldistance (a: &u32, b:&u32) -> u32 {

let r =  ((a & 0x00ff0000).saturating_sub(b & 0x00ff0000) ) & 0x00ff0000;
let g = ((a & 0x0000ff00).saturating_sub(b & 0x0000ff00) ) & 0x0000ff00;
let b =((a & 0x000000ff).saturating_sub(b & 0x000000ff)  ) & 0x000000ff; 
let pixel = r + g + b;
pixel

}

fn image_distance(mona: &Vec<u32>, lisa: &Vec<u32>)-> Vec<u32> {
let len :usize= mona.len();
let image = (0..len).map(|i| pixeldistance(&mona[i], &lisa[i])).collect();
image
}
fn slowmix(mona: &Vec<u32>, lisa: &Vec<u32>)-> Vec<u32> {

    let length = 0..mona.len();
    let mut painting = vec![];

    for i in length {
         let ( mr, mg, mb) = split_u32_color(&mona[i]);
         let ( lr, lg, lb) = split_u32_color(&lisa[i]);
        let mixr = (3*(mr as u32) + 1* (lr as u32))/4 - ((mr>lr ) as u32) + ((mr<lr) as u32);
        let mixg = (3*(mg as u32) + 1* (lg as u32))/4 - ((mg>lg ) as u32) + ((mg<lg) as u32);
        let mixb = (3*(mb as u32) + 1* (lb as u32))/4 - ((mb>lb ) as u32) + ((mb<lb) as u32);

        let r:u32 = mixr * 0x10000;
        let g:u32 = mixg * 0x100;
        let b:u32 = mixb;
        let rgb = r+g+b;
        
        painting.push(rgb);
    }

painting

}
fn image_add(mona: &Vec<u32>, lisa: &Vec<u32>) -> Vec<u32> {
let mut portrait:Vec<u32> = vec![];
    for i in 0..mona.len() {
    let r =  ((mona[i] & 0xff0000).saturating_add(lisa[i] & 0xff0000) ) & 0xff0000;
    let g = ((mona[i] & 0xff00).saturating_add(lisa[i] & 0xff00) ) & 0xff00;
    let b =((mona[i] & 0x00ff).saturating_add(lisa[i] & 0xff)  ) & 0xff;
    let rgb=r+g+b;
    portrait.push(rgb);
    }
    portrait
}
fn blackwhitecolorcollapse(pixel:u32)->u32 {
    let mut r = pixel & 0x00ff0000;
    let mut g = pixel & 0x0000ff00;
    let mut b = pixel & 0x000000ff;
    r = r >> 16 ;
    g = g >> 8 ;
    let pixout = r+g+b;
    pixout
}

fn image_mean(mona: &Vec<u32>, lisa: &Vec<u32>)-> Vec<u32> {
    let arange = 0..mona.len();
    let portrait = arange.map(|i|   (((mona[i]&0xff0000 + lisa[i]&0xff0000)/2)&0xff0000) +
                                             (((mona[i]&0x00ff00 + lisa[i]&0x00ff00)/2)&0x00ff00) +
                                             (((mona[i]&0x0000ff + lisa[i]&0xff)/2)&0x0000ff) ).collect();
    portrait

}


fn image_stretch_to_window(image_width:usize, image_height:usize, image_data:Vec<u32> )-> Vec<u32> {

    let lish = image_data.len();
    let needish = WINHIGH*WINWIDE;
    let framed = image_height*image_width as usize;
    
    let mut  painting = vec![];

    for j in 0..WINHIGH {
        for i in 0..WINWIDE {
            let mut x = shrink(i, image_width, WINWIDE);
            let mut y = shrink(j, image_height, WINHIGH); 
            
            let image_index = x + image_width*y;
            painting.push(image_data[image_index]);
        }
    }    

    painting
    }

fn image_skew_area_of_intrest_to_full_window(tl: (usize,usize), tr:(usize,usize), bl:(usize,usize), br:(usize,usize),image_to_skew:&Vec<u32>)-> Vec<u32>{

let mut newbuff:Vec<u32> = vec![];
let ww = WINWIDE-1;
let wh= WINHIGH-1;
for y in 0..WINHIGH {
    for x in 0..WINWIDE {
       let (refx,refy) = fraction(
        &fraction(&tl, &tr, x,WINWIDE),
        &fraction(&bl, &br, x, WINWIDE), y, WINHIGH);
       
       let index = refx + WINWIDE * refy;
       newbuff.push(image_to_skew[index].clone());
    }
}
newbuff
}
fn split_u32_color(pixel: &u32) -> (u8, u8, u8) {
    let r = (0x00ff0000 & pixel >>16) as u8;
    let g = (0x0000ff00 & pixel >>8) as u8;
    let b = (0x000000ff & pixel) as u8;
    (r, g , b)
}

fn init_camera()-> CallbackCamera {
    let nokhwa_backend = nokhwa::native_api_backend();
   
    let nokhwa_ready = nokhwa::nokhwa_check();
    let nokhwa_query = nokhwa::query(MediaFoundation);
    nokhwa_initialize(|granted| {
        println!("Camera access? {}", granted);
    });
    println!("What art thou kæmrə?");
    dbg!(&nokhwa_backend);
    dbg!(&nokhwa_ready);
    dbg!(&nokhwa_query);

    let format = RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestResolution);

    let cameras = query(ApiBackend::Auto).unwrap();
    let first_camera = cameras.first().unwrap();

    let threaded = CallbackCamera::new(first_camera.index().clone(), format, |buffer| {
        let amage = buffer.decode_image::<RgbFormat>().unwrap();
        println!("{}x{} {}", amage.width(), amage.height(), amage.len());
    })
    .unwrap();
    threaded

}

fn fraction(a: &(usize, usize), b:&(usize,usize), numerator:usize, denominator:usize)-> (usize,usize) {
    let (ax,ay) = *a;
    let (bx,by) = *b;
    
    let xish = (denominator-numerator) * ax + numerator * bx;
    let yish = (denominator-numerator) * ay + numerator * by;
    let x = xish/denominator;
    let y = yish/denominator;
    
    (x,y)
}

fn photo_negative(img: &Vec<u32>)->Vec<u32>{
    let mut neg = vec![];
    for pixel in img {
        let putpix= 0x00ffffff - pixel;
        neg.push(putpix);
    }
    neg
}
fn u8rgb_to_u32(rgb: &Vec<u8>)-> Vec<u32>{
    let l=  rgb.len()/3;
    let outgo:Vec<u32> = (0..l).map(|i| 0x10000 * (rgb[3*i] as u32) + 0x100 * (rgb[3*i+1] as u32) + (rgb[3*i+2] as u32)).collect();
    outgo
}

fn build_window()->Window {
    let mut window = Window::new(
        "",
        WINWIDE,
        WINHIGH,
        WindowOptions {
            borderless: true,
            transparency: false,
            title: false,
            resize: false,
            scale: FitScreen,
            scale_mode: ScaleMode::UpperLeft,
            topmost: true,
            none: true,
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    let blackscreen = vec![0u32;WINHIGH*WINWIDE];
    window
    .update_with_buffer(&blackscreen, WINWIDE, WINHIGH)
    .unwrap();  
    // Limit to max ~60fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
 window
}

fn snapshot(mut callbackcam:  CallbackCamera) -> (CallbackCamera, Vec<u32>) {
let frame = callbackcam.poll_frame().unwrap();
let image = frame.decode_image::<RgbFormat>().unwrap();
let u8x3image= image.to_vec();
let u32image = u8rgb_to_u32(&u8x3image);
let screensized =  image_stretch_to_window(CAMWIDE,CAMHIGH, u32image);
(callbackcam, screensized)
}
fn half_negative(img: &Vec<u32>)->Vec<u32>{

    let mut neg = vec![];
    for pixel in img {
        let r:u32= (0x00ff0000 - (0x00ff0000 & pixel))&0x00ff0000;
        let g:u32= (0x0000ff00 - (0x0000ff00 & pixel))&0x0000ff00;
        let b:u32= (0x000000ff - (0x000000ff & pixel))&0x000000ff;
        let putpix:u32 =  r+g+b;
        neg.push(putpix);
    }

neg
}
fn camera_screen_corners (mut window: Window, mut optics: CallbackCamera) -> (Window, CallbackCamera, (usize,usize),(usize,usize),(usize,usize),(usize,usize)) {
let bs:Vec<u32> = vec![0u32;WINHIGH*WINWIDE];
let ws:Vec<u32> = vec![0xffffff;WINHIGH*WINWIDE];

let duration = std::time::Duration::from_millis(800);
let mut blackcam = vec![];
let mut whitecam = vec![];
window.update_with_buffer(&bs, WINWIDE, WINHIGH).unwrap();  //several dark frames.
thread::sleep(duration);
(optics,blackcam) = snapshot(optics);                                              //photo
thread::sleep(duration);
window.update_with_buffer(&ws, WINWIDE, WINHIGH).unwrap();
thread::sleep(duration);
(optics, whitecam) = snapshot(optics); 
thread::sleep(duration);
let mut  imagedeltas = image_distance(&whitecam, &blackcam);
window.update_with_buffer(&imagedeltas, WINWIDE, WINHIGH).unwrap();
let mut tl =(110000,110000);let mut tr=(110000,110000);let mut bl=(110000,110000);let mut br=(110000,110000);
let halfw = WINWIDE/2;
let halfh = WINHIGH/2;

let mut text = font6x8::new_renderer(WINWIDE, WINHIGH, 0xffffff);
    text.draw_text(&mut imagedeltas, 10, 20, "1>  center camera/projection.  2>Left click highlighted 4 corners to calibrate!");
    text.set_color(0xff_00_00);
    text.draw_text(&mut imagedeltas, 10, 180, "Press ESC to exit");

while window.is_open() && ((tl.1+tr.1+bl.1+br.1) > 100000) && !window.is_key_down(Key::Escape) {
 if window.get_mouse_down(MouseButton::Left) {
    let (fx, fy) = window.get_mouse_pos(MouseMode::Discard).unwrap();
    let x = fx as usize;
    let y = fy as usize;
    
    let tb = y<halfh;
    let lr= x<halfw;
    if tb && lr {//top left
        tl = (x,y); 
    }
    if tb && !lr {//top right
        tr = (x,y); 
    }
    if !tb && lr {//bottom left
        bl = (x,y); 
    }
    if !tb && !lr {//bottom left
        br = (x,y); 
    }
    
    };

    window.update_with_buffer(&imagedeltas, WINWIDE, WINHIGH).unwrap();
    thread::sleep(std::time::Duration::from_millis(16));
}

//let black = image_mean(&blackcamframe1, &blackcamframe2);
//let white = image_mean(&whitecamframe1,&whitecamframe2);

(window, optics, tl,tr, bl, br)
}
fn nthmod(mut image: Vec<u32>, offsets: Vec<usize>, clock: usize, modulus: usize, color: u32)-> Vec<u32> {
    let limit = image.len();
    for pixel in 0..limit  {
        for offset in 0..offsets.len() {
            let b = (!((pixel+clock)%modulus>0)) as usize; 
            image[(b*(pixel+offset))%limit] = color;
        }  
    }
    image
}
fn main() {
let mut minifb_buf: Vec<u32> = vec![0; WINHIGH * WINWIDE];

let duration = std::time::Duration::new(0, 250000);
let mut window = build_window();

let mut optics = init_camera(); 
let mut eye2:Vec<u32> = vec![];
let mut eye1:Vec<u32> = vec![];
let mut tl=(0,0); let mut tr=(0,0); let mut bl=(0,0); let mut br=(0,0);
 (window, optics, tl,tr,bl,br) = camera_screen_corners(window,optics);

let snowflake = vec![0, 1*WINWIDE+1, 2*WINWIDE+2,3*WINWIDE+3,4*WINWIDE+4,5*WINWIDE+5];
let mut clock = 0usize;
let mut eye3=eye1.clone();
let mut eye4 = vec![0x777777; WINHIGH*WINWIDE];
dbg!(tl);dbg!(tr);dbg!(bl);dbg!(br);

    while window.is_open() && !window.is_key_down(Key::Escape) {
    (optics, eye1) = snapshot(optics);
    let mut eye2 = image_skew_area_of_intrest_to_full_window(tl, tr, bl, br, &eye1);
    eye3 = photo_negative(&eye2);
    eye4 = slowmix(&eye3, &eye4);
   
    window.update_with_buffer(&eye4, WINWIDE, WINHIGH).unwrap();
    thread::sleep(std::time::Duration::from_millis(1));
clock = clock+1;}

}
