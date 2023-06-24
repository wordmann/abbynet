use minifb::{MouseMode, Window, WindowOptions, ScaleMode, Scale, Key, KeyRepeat, clamp};
use raqote::{DrawTarget, SolidSource, Source, DrawOptions, PathBuilder, Point, Transform, StrokeStyle};
use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use std::net::{UdpSocket, SocketAddr, TcpStream};
use std::io::{self, Write, Read};
use std::env;
use std::path::Path;
use std::fs::{self, OpenOptions};
use std::process::Command;
const WIDTH: usize = 1200;
const HEIGHT: usize = 800;


fn main() {
    // let FILEPATH: String = String::from("enter.wuml");
    // println!("Searching this dir: {}", std::env::current_dir().unwrap().display());
    
    //     println!("In file {}", FILEPATH);
    
    //     let contents = fs::read_to_string(Path::new(&FILEPATH))
    //         .expect("Should have been able to read the file");

    let mut abMain = String::new();
    println!("Enter ABMAIN: ");
    std::io::stdin().read_line(&mut abMain).unwrap();

    let push_ab: Vec<&str> = abMain.split("&").collect();
    if push_ab.len() > 1 {

        let wuml_filepath = "enter.wuml".to_owned();

        let mut new_wumlfile = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(false)
        .open(wuml_filepath) {
            Ok(file) => file,
            Err(_err) => panic!("FUCK FUCK FUCK SHIT FUCK FUCK FUCK"),
        };

        let mut to_send: String = "".to_owned();
        new_wumlfile.read_to_string(&mut to_send);
        
        abMain.push_str(&to_send);

    }

    let contents = fetch_wuml("93.42.100.30:8080", &abMain);

    render_wuml(contents);

}

fn fetch_wuml(server_ip: &str, msg: &str) -> String {
    // Connect to the server
    match TcpStream::connect(server_ip) {
        Ok(mut stream) => {
            println!("Connected to server");

            // Prepare the data to send to the server
            let data: String = msg.to_string();

            // Send the data to the server
            stream.write_all(data.as_bytes()).expect("Write failed");

            // Receive the response from the server
            let mut buffer = [0; 1024];
            stream.read(&mut buffer).expect("Read failed");
            let received_data = String::from_utf8_lossy(&buffer).into_owned();
            received_data
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
            String::new()
        }
    }
}


fn render_wuml(contents: String){

    println!("{}", contents);

    let mut window = Window::new("Raqote", WIDTH, HEIGHT, WindowOptions {
        ..WindowOptions::default()
    }).unwrap();

    let font = SystemSource::new()
    .select_best_match(&[FamilyName::SansSerif], &Properties::new())
    .unwrap()
    .load()
    .unwrap();

    let size = window.get_size();
    let mut dt = DrawTarget::new(size.0 as i32, size.1 as i32);

    let v: Vec<&str> = contents.rsplit("---").collect();

    let doc_pages: Vec<&str> = v[0].split("::").collect();
    let mut cur_page: usize = 0;

    let mut doc_lines: Vec<&str> = doc_pages[0].split("\n").collect();
    // println!("{}", doc_lines.len());

    loop {

        dt.clear(SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0xff, 0xff));

        let mut line_num: f32 = 1.;
        for line in &doc_lines{

            dt.draw_text(&font, 36., line, Point::new(0., 36. * line_num),
                &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0, 0, 0)),
            &DrawOptions::new(),
            );

            line_num += 1.;
 
        }

        window.get_keys_pressed(KeyRepeat::No).iter().for_each(|key|{
            let mut keynum = *key as usize;
            
            if (keynum == 0) { keynum = 9 }
            
            else if keynum == 52{
                if (cur_page == 0) {cur_page = doc_pages.len() -1}
                else {cur_page -= 1;}
            }
            else if keynum == 53 {
                cur_page += 1;
            }

            else {
                keynum -= 1;
                cur_page = keynum;
            };
            
            println!("{}", cur_page);
        }
    );
    
    cur_page = {
        if cur_page >= doc_pages.len() {doc_pages.len() -1}
        else {cur_page}
    };
    
    doc_lines = doc_pages[cur_page].split("\n").collect();
    
    window.update_with_buffer(dt.get_data(), size.0, size.1).unwrap();
    
}

}
