extern crate hidapi;

use hidapi::HidApi;
use std::{thread, time};
use enigo::*;
use enigo::Key::{Backspace, Return, Space, LeftArrow, RightArrow};

fn main() {

    //check all devices, i didnt make this part just the charicter selection stuff idk how to use this api
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
                    let millis = time::Duration::from_millis(1);
                    let mut prev_action: bool = false;
                    
                    let mut mode: &str = "text";
                    let mut glob_charset: [char; 4] = ['`','`','`','`'];
                    let mut curr_char: String = "".to_string();

                    //initalize enigo (sending keys)
                    let mut enigo = Enigo::new();

                    loop {
                        //sleep bc too fast for console
                        thread::sleep(millis);

                        //clear screen
                        print!("{}c",27 as char);

                        //print stuff
                        println!("current letter: {}", curr_char);
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
                        let charset = get_charset(stick_left_x, stick_left_y, mode);
                        if charset != ['`','`','`','`'] {glob_charset = charset;}

                        //get char
                        let letter = get_char(stick_right_x, stick_right_y, glob_charset);
                        if letter != "`" {curr_char = letter;}

                        //triggers: add char / backspace
                        if prev_action == false { //prevents uncontrolled deletion 
                            if      triggers == 4   {enigo.key_click(Backspace);  prev_action = true;}
                            else if triggers == 8   {enigo.key_sequence(&curr_char);  prev_action = true;}
                        } else {prev_action = false;}

                        //d pad navigation
                        if buttons == 0         {enigo.key_click(Return);}
                        if buttons == 4         {enigo.key_click(Space);}
                        if buttons == 2         {enigo.key_click(RightArrow);} 
                        if buttons == 6         {enigo.key_click(LeftArrow);}

                        //mode switching
                        if buttons == 40        {mode = "text";     glob_charset=['`','`','`','`'];}
                        else if buttons == 72   {mode = "numbers";  glob_charset=['`','`','`','`'];}
                        else if buttons == 136  {mode = "emoji";    glob_charset=['`','`','`','`'];}
                        else if buttons == 24   {mode = "dev";      glob_charset=['`','`','`','`'];}
                 


                    } //loop

                }
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
        },
    }
}

fn get_char(x:u8, y:u8, charset: [char; 4]) -> String {

    //N     x: 0-255,   y: 0-64
    //E     x: 192-255  y: 64-192
    //S     x: 0-255,   y: 192-255
    //W     x: 0-64,    y: 64-192


    if in_range(x, 0, 255) && in_range(y, 0, 64) {                      //N
        return String::from(charset[0]);

    } else if in_range(x, 192, 255) && in_range(y, 64, 192) {           //E
        return String::from(charset[1]);

    } else if in_range(x, 0, 255) && in_range(y, 192, 255) {            //S
        return String::from(charset[2]);

    } else if in_range(x, 0, 64) && in_range(y, 64, 192) {              //W
        return String::from(charset[3]);
    } else {
        return String::from('`');
    }
}

fn get_charset(x:u8, y:u8, mode: &str) -> [char; 4] {

    //N     x: 96-162,  y: 0-96
    //NE    x: 162-255, y: 0-96
    //E     x: 162-255, y: 96-162
    //SE    x: 162-255, y: 162-255
    //S     x: 96-162,  y: 162-255
    //SW    x: 0-96,    y: 162-255
    //W     x: 0-96,    y: 96-162
    //NW    x: 0-96,    y: 0-96

    if in_range(x, 96, 162) && in_range(y, 0, 96) {                 //N
        if mode == "text" || mode == "dev"  {return ['a','b','c','d']}
        else if mode == "numbers"           {return ['0','1','2','3']}
        else if mode == "emoji"             {return ['n','n','n','n']}
        else                                {return ['`','`','`','`']}

    } else if in_range(x, 162, 255) && in_range(y, 0, 96) {         //NE
        if mode == "text" || mode == "dev"  {return ['e','f','g','h']}
        else if mode == "numbers"           {return ['4','5','6','7']}
        else if mode == "emoji"             {return ['n','n','n','n']}
        else                                {return ['`','`','`','`']}

    } else if in_range(x, 162, 255) && in_range(y, 96, 162) {       //E    
        if mode == "text" || mode == "dev"  {return ['i','j','k','l']}
        else if mode == "numbers"           {return ['8','9','@','#']}
        else if mode == "emoji"             {return ['n','n','n','n']}
        else                                {return ['`','`','`','`']}

    } else if in_range(x, 162, 255) && in_range(y, 162, 255) {      //SE
        if mode == "text" || mode == "dev"  {return ['m','n','o','p']}
        else if mode == "numbers"           {return ['$','%','^','&']}
        else if mode == "emoji"             {return ['n','n','n','n']}
        else                                {return ['`','`','`','`']}

    } else if in_range(x, 96, 162) && in_range(y, 162, 255) {       //S
        if mode == "text" || mode == "dev"  {return ['q','r','s','t']}
        else if mode == "numbers"           {return ['(',')','[',']']}
        else if mode == "emoji"             {return ['n','n','n','n']}
        else                                {return ['`','`','`','`']}

    } else if in_range(x, 0, 96) && in_range(y, 162, 255) {         //SW
        if mode == "text" || mode == "dev"  {return ['u','v','w','x']}
        else if mode == "numbers"           {return ['{','}','<','>']}
        else if mode == "emoji"             {return ['n','n','n','n']}
        else                                {return ['`','`','`','`']}

    }else if in_range(x, 0, 96) && in_range(y, 96, 162) {           //W
        if mode == "text" || mode == "dev"  {return ['y','z','.',',']}
        else if mode == "numbers"           {return ['+','-','*','/']}
        else if mode == "emoji"             {return ['n','n','n','n']}
        else                                {return ['`','`','`','`']}

    }else if in_range(x, 0, 96) && in_range(y, 0, 96) {             //NW
        if mode == "text" || mode == "dev"  {return ['!','?','_',' ']}
        else if mode == "numbers"           {return ['=',';',':','~']}
        else if mode == "emoji"             {return ['n','n','n','n']}
        else                                {return ['`','`','`','`']}

    } else {return ['`','`','`','`'];}
}
fn in_range(num: u8, start:u8, end:u8) -> bool {

    if num >= start && num <= end {return true}
    else {return false}

}