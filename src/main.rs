mod date;

use std::io;

use tokio_util::bytes::BytesMut;
use tokio::net::{TcpListener, TcpStream};
use futures::sink::SinkExt;
use tokio_stream::StreamExt;
use tokio_util::codec::{Decoder, Encoder, Framed};
use crate::date::datev1::now1;

#[tokio::main]
async fn main() {

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();

    loop {
        let (stream, s_addr) = listener.accept().await.unwrap();

        let mut frame_request = Framed::new(stream, Http);
        while let Some(request) = frame_request.next().await {
            println!("inside next");
            match request {
                Ok(request) => {
                    let resp = format!("HTTP/1.1 200 OK\r\nServer: nginx/1.18.0 (Ubuntu)\r\nContent-Length: 0\r\nDate: {}\r\n\r\n", now1());
                    frame_request.send(resp.to_string()).await.unwrap();
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
    }
}



struct  Http;


impl Encoder<String> for Http {
    type Error = io::Error;

    fn encode(&mut self, item: String, dst: &mut BytesMut) -> io::Result<()> {
        // println!("inside encode! {}", item);
        dst.extend_from_slice(item.as_bytes());
        // println!("inside encode! {}", dst.len());
        return Ok(());

    }
}

impl Decoder for Http{
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) ->io::Result<Option<Self::Item>> {
        // println!("Inside decoder {}", src.len());
        if src.len() > 0{
            let raw_data = src.split_to(src.len());
            // println!("Inside decoder after{}", src.len());
            let data = unsafe{ String::from_utf8_unchecked(Vec::<u8>::from(raw_data.as_ref())) };
            // println!("{}", data);
            Ok(Some(data))
        }else{
            Ok(None)
        }

    }
}