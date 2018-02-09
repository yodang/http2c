extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;
extern crate tokio_core;
#[macro_use] extern crate hyper;

mod frame;
use bytes::Bytes;
use std::env;
use std::io::{self, Write};
use std::str::FromStr;
use hyper::header::{Header, Protocol, ProtocolName};
use tokio_core::reactor::Core;
use futures::Future;

fn help()
{
    println!("Usage: http2c url");
}

header! {(Http2Settings,"HTTP2-Settings")=>[String]}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len()==1
    {
        help();
        return;
    }

    assert!(args.len() != 1);
    let remote_str=&args[1];
    let remote=hyper::Uri::from_str(remote_str.as_str()).unwrap_or_default();

    let mut req=hyper::Request::new(hyper::Method::Get, remote);
    req.headers_mut().set(hyper::header::Upgrade(vec![Protocol::new(ProtocolName::H2c, None)]));
    req.headers_mut().set(Http2Settings("".to_owned()));
    println!("{:?}", req);

    let mut core=Core::new().unwrap();
    let client=hyper::Client::new(&core.handle());
    let worker=client.request(req).map(|res|{
        println!("Response {}", res.status());
    });

    core.run(worker);
}
