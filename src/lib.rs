#![feature(test)]
extern crate test;

use core::convert::TryFrom;
use std::collections::HashMap;

use std::error::Error;
use std::str;
struct Message<'a> {
    start_line: StartLine<'a>,
    headers: Headers<'a>,
    content: &'a str,
    // Keeping it as a ref to a slice keeps it read only
    original_message: &'a [u8],
}

impl<'a> TryFrom<&'a Vec<u8>> for Message<'a> {
    // You probably want your own error type here
    // I used this here because it just works
    type Error = Box<Error>;

    fn try_from(bytes: &'a Vec<u8>) -> Result<Self, Self::Error> {
        let message = str::from_utf8(bytes)?;

        let message: Vec<&str> = message.split("\r\n").collect();

        let start_line = StartLine::Request(RequestLine::new(message[0]));
        let headers = Headers::new(&message[1..message.len()]);
        let content = message[message.len() - 1];

        Ok(Message {
            start_line,
            headers,
            content,
            original_message: bytes,
        })
    }
}


#[derive(Debug)]
enum StartLine<'a> {
    Request(RequestLine<'a>),
    Response(StatusLine),
}

#[derive(Debug)]
struct RequestLine<'a> {
    method: &'a str,
    uri: &'a str,
    version: &'a str,
}

impl<'a> RequestLine<'a> {
    fn new(start_line: &'a str) -> Self {
        let elements: Vec<&str> = start_line.split_whitespace().collect();

        RequestLine {
            method: elements[0],
            uri: elements[1],
            version: elements[2],
        }
    }
}

#[derive(Debug)]
struct StatusLine {
    version: String,
    status: String,
}

enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    OPTIONS,
}

#[derive(Debug)]
struct Headers<'a> {
    headers: HashMap<&'a str, Option<&'a str>>,
}

impl<'a> Headers<'a> {
    fn new(headers: &[&'a str]) -> Self {
        let mut headers_map: HashMap<&'a str, Option<&'a str>> = HashMap::new();

        for header in headers.iter() {
            let header: Vec<&str> = header.split(": ").collect();

            if header.len() == 2 {
                headers_map.insert(header[0], Some(header[1]));
            } else {
                headers_map.insert(header[0], None);
            }
        }

        Headers {
            headers: headers_map,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use test::Bencher;

    #[test]
    fn from_request_method() {
        let request = fs::read_to_string("mocks/request.txt").unwrap();
        let request = request.into_bytes();

        let message = Message::try_from(&request).unwrap();

        match message.start_line {
            StartLine::Request(request_line) => {
                assert_eq!(String::from("GET"), request_line.method)
            }
            _ => unimplemented!("Oops, shouldn't be here!"),
        }
    }

    #[test]
    fn from_request_uri() {
        let request = fs::read_to_string("mocks/request.txt").expect("failed to read file");
        let request = request.into_bytes();

        let message = Message::try_from(&request).unwrap();

        match message.start_line {
            StartLine::Request(request_line) => assert_eq!(String::from("/"), request_line.uri),
            _ => unimplemented!("Oops, shouldn't be here!"),
        }
    }

    #[test]
    fn from_request_version() {
        let request = fs::read_to_string("mocks/request.txt").unwrap();
        let request = request.into_bytes();

        let message = Message::try_from(&request).unwrap();

        match message.start_line {
            StartLine::Request(request_line) => {
                assert_eq!(String::from("HTTP/1.1"), request_line.version)
            }
            _ => unimplemented!("Oops, shouldn't be here!"),
        }
    }

    #[bench]
    fn bench_from_request_version(b: &mut Bencher) {

        let mock = r#"
GET / HTTP/1.1
Host: localhost:7878
Connection: keep-alive
Pragma: no-cache
Cache-Control: no-cache
Upgrade-Insecure-Requests: 1
User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_14_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/76.0.3809.100 Safari/537.36
Sec-Fetch-Mode: navigate
Sec-Fetch-User: ?1
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3
Sec-Fetch-Site: none
Accept-Encoding: gzip, deflate, br
Accept-Lan
"#;

        let vec = mock.as_bytes().to_vec();
        let message = Message::try_from(&vec).unwrap();

        match message.start_line {
            StartLine::Request(request_line) => {
                assert_eq!(String::from("HTTP/1.1"), request_line.version)
            }
            _ => unimplemented!("Oops, shouldn't be here!"),
        }

        b.iter(|| from_request_version());
    }
}
