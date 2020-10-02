use std::*;
use std::io::*;
use byteorder::{ByteOrder, NativeEndian};

pub fn send_msg(msg : String, stream : &mut Stdout){
    let data = msg.as_bytes();
    let mut len_buf = [0u8;4];
    NativeEndian::write_u32(&mut len_buf, data.len() as u32);

    stream.write(&len_buf).expect("Can't write");
    stream.write(&data).expect("Can't write");
    stream.flush().expect("Can't flush");
}

pub fn send_msg_packed(msg : String, stream : &mut Stdout){
    send_msg(format!("{{\"result\":\"{}\"}}", msg), stream);
}

pub fn receive_msg(stream : &mut Stdin) -> String{
    let mut len_buf = [0u8;4];
    stream.read_exact(&mut len_buf).expect("Can't read length of incoming message");
    let byte_count = NativeEndian::read_u32(&len_buf) as usize;
    let mut msg_buf = vec![0; byte_count];
    stream.read_exact(&mut msg_buf).expect("Error receiving message");
    let msg = String::from_utf8(msg_buf).expect("Cant parse utf8 message");

    msg
}