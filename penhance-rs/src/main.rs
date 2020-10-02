mod coms;
mod conf;

extern crate gtk;
use gtk::prelude::*;
use gtk::{ButtonsType, DialogFlags, MessageType, MessageDialog, Window};

use std::io::*;
use std::vec::*;
use std::fs::*;
use std::thread::sleep;
use std::process::{Command, exit};

use coms::*;
use conf::*;

fn is_site_forbidden(site : String, blacklist : &Vec<String>) -> bool{
    for s in blacklist.iter() {
        if site.contains(s) {
            return true;
        }
    }

    false
}

fn punish(){
    let c = Command::new("killall").arg("/usr/lib/firefox/firefox").spawn().expect("Can't run punishment process");
    exit(0);
}

fn main() {
    let mut cout = stdout();
    let mut cin = stdin(); 

    let gtk_enabled = gtk::init().is_err();

    if !gtk_enabled {
        send_msg_packed(String::from("Can't initialize gtk..."), &mut cout);
    }

    



    
    /*let msg = "{\"result\": \"OK\"}".as_bytes();
    let mut len_buf = [0u8; 4];
    NativeEndian::write_u32(&mut len_buf, msg.len() as u32);*/

    let mut exit = false;
    let mut count = 0;

    let blacklist = load_blacklist();

    while !exit {
        send_msg(String::from("{ \"result\": \"OK\"}"), &mut cout);

        let msg = receive_msg(&mut cin);

        if msg == "\"kill-penhance\"" {
            exit = true;
            continue;
        }

        //sleep(std::time::Duration::from_millis(1000));

        if is_site_forbidden(msg.to_lowercase(), &blacklist) {
            send_msg(String::from("{\"result\":\"Cheating detected\"}"), &mut cout);
            MessageDialog::new(None::<&Window>,
                DialogFlags::empty(),
                MessageType::Info,
                ButtonsType::Ok,
                "You fool!\nPrepare for punishment!").run();
            punish();
        }

        send_msg(format!("{{\"result\": \"OK - {}\"}}", count), &mut cout);
        count += 1;

        //eprintln!("Test error log");
        /*{
            let mut f = OpenOptions::new().create(true).write(true).append(true).open("savefile-rs.txt").expect("Can't open file");
            f.write(msg.as_bytes()).expect("Cant write msg");
            f.write("\n".as_bytes()).expect("Can't write into file");
            f.flush().expect("Can't flus");
        }*/
    }

    
}
