// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use tracing_subscriber::prelude::*;

// Asynchronous eepget:
//    cargo run --example eepget -- <host>
//
// Synchronous eepget:
//    cargo run --example hyper --no-default-features --features hyper,tokio -- \
//    'http://udhdrtrcetjm5sxzskjyr5ztpeszydbh4dpl3pl4utgqqw2v4jna.b32.i2p/hosts.txt'

#[cfg(all(feature = "tokio", not(feature = "sync")))]
#[tokio::main]
async fn main() {
    use http_body_util::{BodyExt, Empty};
    use hyper::{body::Bytes, Request};
    use yosemite::{style::Stream, Session, SessionOptions};

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .unwrap();

    let url = std::env::args().nth(1).expect("url");
    let parsed_url = url.parse::<hyper::Uri>().expect("could not parse url");
    let host = format!("{}/", parsed_url.host().expect("uri has no host"));
    tracing::debug!("creating session");
    let authority = parsed_url.authority().unwrap().clone();
    let mut session = Session::<Stream>::new(SessionOptions::default())
        .await
        .expect("could not create session");
    tracing::debug!("connecting to {host}");
    let stream = session.connect(&host).await.unwrap();
    tracing::debug!("connected");
    let (mut sender, conn) = hyper::client::conn::http1::handshake(stream)
        .await
        .expect("could not establish http connection");
    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });
    let req = Request::builder()
        .uri(url)
        .header(hyper::header::HOST, authority.as_str())
        .body(Empty::<Bytes>::new())
        .expect("could not create request");
    tracing::debug!("sending request");
    let mut res = sender.send_request(req).await.expect("could not send request");
    tracing::debug!("request received");
    while let Some(next) = res.frame().await {
        let frame = next.expect("frame");
        if let Some(chunk) = frame.data_ref() {
            let body = std::str::from_utf8(chunk).expect("decode");
            tracing::debug!("{}", body);
        }
    }
}

#[cfg(all(feature = "smol", not(feature = "sync")))]
async fn main_smol() {
    use http_body_util::{BodyExt, Empty};
    use hyper::{body::Bytes, Request};
    use yosemite::{style::Stream, Session, SessionOptions};

    let url = std::env::args().nth(1).expect("url");
    let parsed_url = url.parse::<hyper::Uri>().expect("could not parse url");
    let host = format!("{}/", parsed_url.host().expect("uri has no host"));
    tracing::debug!("creating session");
    let authority = parsed_url.authority().unwrap().clone();
    let mut session = Session::<Stream>::new(SessionOptions::default())
        .await
        .expect("could not create session");
    tracing::debug!("connecting to {host}");
    let stream = session.connect(&host).await.unwrap();
    tracing::debug!("connected");
    let (mut sender, conn) = hyper::client::conn::http1::handshake(stream)
        .await
        .expect("could not establish http connection");
    let _ = smol::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });
    let req = Request::builder()
        .uri(url)
        .header(hyper::header::HOST, authority.as_str())
        .body(Empty::<Bytes>::new())
        .expect("could not create request");
    tracing::debug!("sending request");
    let mut res = sender.send_request(req).await.expect("could not send request");
    tracing::debug!("request received");
    while let Some(next) = res.frame().await {
        let frame = next.expect("frame");
        if let Some(chunk) = frame.data_ref() {
            let body = std::str::from_utf8(chunk).expect("decode");
            tracing::debug!("{}", body);
        }
    }
}

#[cfg(all(feature = "smol", not(feature = "sync")))]
fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .unwrap();
    smol::block_on(main_smol())
}
