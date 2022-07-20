extern crate hidapi;

use hidapi::HidApi;
use std::{thread, time};

fn main() {

    //check all devices
    match HidApi::new() {
        Ok(api) => {
            for device in api.devices() {

                //get devices with sony
                if device.manufacturer_string.as_ref().unwrap() == "Sony Computer Entertainment" {

                    //initialize the controller
                    let VID = device.vendor_id;
                    let PID = device.product_id;
                    let input = api.open(VID, PID).unwrap();
                    
                    //initalize varibles and stuff
                    let millis = time::Duration::from_millis(10);

                    let mut mode: &str = "text";
                    let mut glob_charset: [char; 4] = ['`','`','`','`'];
                    let mut curr_char: char = ' ';
                    let mut text: String = String::from("");

                    loop {
                        //sleep bc too fast for console
                        thread::sleep(millis);

                        //clear screen
                        print!("{}c",27 as char);

                        //print stuff
                        println!("text: {}, current letter: {}", text, curr_char);
                        println!("--------------------------------------------------------------------");
                        println!("{} mode", mode);
                        if glob_charset != ['`','`','`','`'] {println!("{:?}", glob_charset);}
                        

                        // Read data from device
                        let mut buf = [0u8; 10];
                        let res = input.read_timeout(&mut buf[..], 1000).unwrap();

                        //define controller vars
                        let stick_left_x = buf[3];  //x -
                        let stick_left_y = buf[4];  //y |
                        let stick_right_x = buf[5]; 
                        let stick_right_y = buf[6]; 

                        let buttons = buf[7];
                        let triggers = buf[8];

                        //check for dev mode
                        if mode == "dev" {println!("buttons: {}  |  triggers: {}", buttons, triggers);}

                        //get charset
                        let charset = get_charset(stick_left_x, stick_left_y);
                        if charset != ['`','`','`','`'] {glob_charset = charset;}

                        //get char
                        let letter = get_char(stick_right_x, stick_right_y, glob_charset);
                        if letter != '`' {curr_char = letter;}

                        //triggers: add char / backspace
                        if triggers == 8 {text.push(curr_char);}
                        else if triggers == 4 {text.pop();}

                        //mode switching
                        if buttons == 40 {mode = "text"}
                        else if buttons == 72 {mode = "numbers"}
                        else if buttons == 136 {mode = "emoji"}
                        else if buttons == 24 {mode = "dev"}
                 


                    } //loop

                }
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
        },
    }
}

fn get_char(x:u8, y:u8, charset: [char; 4]) -> char {

    //N     x: 0-255,   y: 0-64
    //E     x: 192-255  y: 64-192
    //S     x: 0-255,   y: 192-255
    //W     x: 0-64,    y: 64-192


    if in_range(x, 0, 255) && in_range(y, 0, 64) {                      //N
        return charset[0];

    } else if in_range(x, 192, 255) && in_range(y, 64, 192) {           //E
        return charset[1];

    } else if in_range(x, 0, 255) && in_range(y, 192, 255) {            //S
        return charset[2];

    } else if in_range(x, 0, 64) && in_range(y, 64, 192) {              //W
        return charset[3];
    } else {
        return '`';
    }
}

fn get_charset(x:u8, y:u8) -> [char; 4] {

    //N     x: 96-162,  y: 0-96
    //NE    x: 162-255, y: 0-96
    //E     x: 162-255, y: 96-162
    //SE    x: 162-255, y: 162-255
    //S     x: 96-162,  y: 162-255
    //SW    x: 0-96,    y: 162-255
    //W     x: 0-96,    y: 96-162
    //NW    x: 0-96,    y: 0-96

    if in_range(x, 96, 162) && in_range(y, 0, 96) {                 //N
        return ['a','b','c','d'];

    } else if in_range(x, 162, 255) && in_range(y, 0, 96) {         //NE
        return ['e','f','g','h'];

    } else if in_range(x, 162, 255) && in_range(y, 96, 162) {       //E
        return ['i','j','k','l'];

    } else if in_range(x, 162, 255) && in_range(y, 162, 255) {      //SE
        return ['m','n','o','p'];

    } else if in_range(x, 96, 162) && in_range(y, 162, 255) {       //S
        return ['q','r','s','t'];

    } else if in_range(x, 0, 96) && in_range(y, 162, 255) {         //SW
        return ['u','v','w','x'];

    }else if in_range(x, 0, 96) && in_range(y, 96, 162) {           //W
        return ['y','z','.',','];

    }else if in_range(x, 0, 96) && in_range(y, 0, 96) {             //NW
        return ['!','?','_','#'];

    } else {
        return ['`','`','`','`'];
    }
}
fn in_range(num: u8, start:u8, end:u8) -> bool {

    if num >= start && num <= end {return true}
    else {return false}

}