use std::collections::HashMap;

#[derive(Debug)]
struct Message {
    start_line: Start_Line,
    headers: Headers,
    content: String,
    original_message: Vec<u8>,
}

impl Message {
    fn from_request(original_message: Vec<u8>) -> Self {
        let message = String::from_utf8_lossy(&original_message);

        let message: Vec<&str> = message.split("\r\n").collect();

        let start_line = Start_Line::Request(Request_Line::new(message[0]));
        let headers = Headers::new(&message[1..message.len()]);
        let content = String::from(message[message.len() - 1]);

        //println!("{:?}", headers);

        //let start_line = message[0];

        Message {
            start_line,
            headers,
            content,
            original_message,
        }
    }
}

#[derive(Debug)]
enum Start_Line {
    Request(Request_Line),
    Response(Status_Line),
}

#[derive(Debug)]
struct Request_Line {
    method: String,
    uri: String,
    version: String,
}

impl Request_Line {
    fn new(start_line: &str) -> Self {
        let elements: Vec<&str> = start_line.split_whitespace().collect();
        let method = String::from(elements[0]);
        let uri = String::from(elements[1]);
        let version = String::from(elements[2]);

        Request_Line {
            method,
            uri,
            version,
        }
    }
}

#[derive(Debug)]
struct Status_Line {
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
struct Headers {
    headers: HashMap<String, String>,
}

impl Headers {
    fn new(headers: &[&str]) -> Self {
        let mut headers_map: HashMap<String, String> = HashMap::new();

        for header in headers.iter() {
            let header: Vec<&str> = header.split(": ").collect();

            if header.len() == 2 {
                headers_map.insert(header[0].to_string(), header[1].to_string());
            } else {
                headers_map.insert(header[0].to_string(), String::from(""));
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

    #[test]
    fn from_request_method() {
        let request = fs::read_to_string("mocks/request.txt").unwrap();
        let request = request.into_bytes();

        let message = Message::from_request(request);

        match message.start_line {
            Start_Line::Request(request_line) => {
                assert_eq!(String::from("GET"), request_line.method)
            }
            _ => println!("Oops, shouldn't be here!"),
        }
    }

    #[test]
    fn from_request_uri() {
        let request = fs::read_to_string("mocks/request.txt").unwrap();
        let request = request.into_bytes();

        let message = Message::from_request(request);

        match message.start_line {
            Start_Line::Request(request_line) => assert_eq!(String::from("/"), request_line.uri),
            _ => println!("Oops, shouldn't be here!"),
        }
    }

    #[test]
    fn from_request_version() {
        let request = fs::read_to_string("mocks/request.txt").unwrap();
        let request = request.into_bytes();

        let message = Message::from_request(request);

        match message.start_line {
            Start_Line::Request(request_line) => {
                assert_eq!(String::from("HTTP/1.1"), request_line.version)
            }
            _ => println!("Oops, shouldn't be here!"),
        }
    }
}
