/*
                World Clock Server
        Written by Regan Koopmans, 15043143
               University of Pretoria
                    March 2017

*/

// For network IO

use std::io::{Read, Write, BufReader, BufRead};
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

// For file io

use std::error::Error;
use std::fs::File;
use std::path::Path;

// For json interaction

extern crate json;

// For environment/argument variabls

use std::env;

// In the main function I initialize a TcpListener listener, that accepts
// new connections, and spawns a thread to handle an incoming connection
// stream.

fn main() {
    let mut domain = String::new();
    domain.push_str("0.0.0.0:");
    domain.push_str(env::args().nth(1).unwrap().as_str());
    let listener = TcpListener::bind(domain.as_str()).unwrap();
    println!("Server listening on {}...", domain);
    println!("Press Ctrl-C to exit.");
    for stream in listener.incoming() {
        thread::spawn(|| {
            let stream = stream.expect("Could initialize stream!");
            read_request(stream);
        });
    }
}

// Function that will create a string from a given filename.
// optionally, the function caller can opt to have HTTP headers
// prepended to this string, using the *add_headers* flag.

fn get_file_bytes(file_name : &str, add_headers: bool) -> Vec<u8> {
    let path = Path::new(&file_name);
    let mut file = match File::open(&path) {
        Err(error) => panic!("Could not open {}, {}",
                                file_name,error.description()),
        Ok(file) => file,
    };
    let mut file_bytes = Vec::new();
    file.read_to_end(&mut file_bytes)
                                    .expect("Could not read file to string.");
    let mut return_string = String::new();

    // HTTP headers for successful, note \r\n is required for both Unix and 
    // windows support. A 404 error is written if writing the 404 page.

    if add_headers {
        if file_name != "static/html/404.html" {
            return_string.push_str("HTTP/1.1 200 OK\r\n");
        } else {
            return_string.push_str("HTTP/1.1 404 Not Found\r\n");
        }
        return_string.push_str("Content-Length: ");
        return_string.push_str(&(file_bytes.len()).to_string());
        return_string.push_str("\r\n");
        let mut content_type = "Content-Type: text/plain\r\n";
        if file_name.contains(".html") {
            content_type = "Content-Type: text/html\r\n";
        } else if file_name.contains(".css") {
            content_type = "Content-Type: text/css\r\n";
        } else if file_name.contains(".json") {
            content_type = "Content-Type: text/json\r\n";
        } else if file_name.contains(".js") {
            content_type = "Content-Type: text/javascript\r\n";
        } 
        return_string.push_str(content_type);
        
        if file_name.contains(".gz") {
            return_string.push_str("Content-Encoding: gzip\r\n");
        }
        return_string.push_str("Connection: close\r\n\r\n");
    }
    let mut return_vector;
    unsafe { 
        return_vector = return_string.as_mut_vec().to_owned(); 
    }
    return_vector.append(&mut file_bytes);
    return_vector.to_owned()
}

// Function that reads and interprets an HTTP 1.1 request.
// This function also maps abstract paths (like main.css) to
// absolute paths in the file system of the server.

fn read_request(stream: TcpStream) {
    let mut response = ("", true);
    let mut reader = BufReader::new(stream);
    for line in reader.by_ref().lines() {
        let line = line.unwrap();
        if line == "" {
            break;
        } else {
            let line_array: Vec<&str> = line.split(" ").collect();
            if line_array[0] == "GET"  || line_array[0] == "POST" {
                println!("\x1B[1;33m{}\x1B[0m", line);
                if line_array[1].contains("?") {
                    let get_array: Vec<&str> = line_array[1].split("?").collect();
                    let get_params: Vec<&str> = get_array[1].split("&").collect();
                    match get_array[0] {
                        "/add"      => add(&get_params),
                        "/delete"   => delete(&get_params),
                        "/edit"     => edit(&get_params),
                        _           => (),
                    }
                    response = ("static/html/interface.html", true);
                } else {
                    response = match line_array[1] {
                    
                    // Interface Data

                        "/"             => ("static/html/login.html",     true),
                        "/interface"    => ("static/html/interface.html", true),
                        "/main.css"     => ("static/css/main.css",        true),
                        "/main.js"      => ("static/js/main.js",          true),
                        "/manifest.json"=> ("static/conf/manifest.json",          true),
                        "/favicon.ico"  => ("static/html/404.html",       true),

                        // Appointments Data

                        "/get-appointments" => ("g-apps", false),
                        "/get-appointment"  => ("g-app", false),
                        _                   => ("static/html/404.html", true),
                    }
                }
            }
        }
    }
    match response.1 {
        true  => write_response(reader.into_inner(),response.0, true),
        false => write_response(reader.into_inner(),response.0, false),
    }
}

// Function that writes the HTTP response in binary format to the 
// TCPStream connected to the user agent.

fn write_response(mut stream: TcpStream, input:&str, is_file: bool) {
    if is_file {
        stream.write_all(get_file_bytes(input, true).as_slice()).unwrap();
    } else {
        stream.write_all(get_data(input, "").as_bytes()).unwrap();
    }
    stream.flush().expect("Could not flush stream!");
}

// Function that will generate an HTML page based on a country input, "za"
// for example. This function automatically prepends HTTP headers.

fn get_data(input: &str, param: &str) -> String {

    let mut data = String::new();
    let mut return_value = String::new();
    let file = File::open("dat/regan.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_string(&mut data).unwrap();
    return_value.push_str("HTTP/1.1 200 OK\r\n");
    return_value.push_str("Content-Type: text/json\r\n");
    return_value.push_str("Content-Length: ");
    return_value.push_str(& data.len().to_string());
    return_value.push_str("\r\n");
    return_value.push_str("Connection: close\r\n\r\n");

    if input == "g-apps" {
        return_value.push_str(&data);
    }
    return_value
}



fn add(params: &Vec<&str>) {

    let mut new_title = String::new();
    let mut param_line:Vec<&str>;
    let mut delete_string = String::new();
    for item in params {
        param_line = item.split("=").collect();
        if param_line[0] == "title" {
            new_title = decode(param_line[1]);
        }
    }

    if contains(&new_title) {
        println!("Deleting {} !", new_title);
        let mut delete_params:Vec<&str> = Vec::new();
        delete_string.push_str("app_to_delete=");
        delete_string.push_str(&new_title);
        delete_params.push(&delete_string);
        delete(&delete_params);
    } else {
        println!("Not deleting {} !", new_title);
    }

    let mut file_string = String::new();
    let mut file = File::open("dat/regan.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_string(&mut file_string).unwrap();
    let mut parsed = json::parse(&file_string).unwrap();
    let mut new_appoint = json::JsonValue::new_object();
    for item in params {
        param_line = item.split("=").collect();
        new_appoint[param_line[0]] = decode(param_line[1]).into();
    }
    if parsed["appointments"].len() == 0 {
        let mut new_appoint_array = json::JsonValue::new_array();
        new_appoint_array.push(new_appoint).unwrap();
        parsed["appointments"] = new_appoint_array;
    } else {
        parsed["appointments"].push(new_appoint).unwrap();
    }
    file = File::create("dat/regan.json").unwrap();
    file.write_all((json::stringify_pretty(parsed,4)).as_bytes()).unwrap();

}

fn contains(key: &str) -> bool {
    let mut file_string = String::new();
    let mut file = File::open("dat/regan.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_string(&mut file_string).unwrap();
    let mut parsed = json::parse(&file_string).unwrap();
    let mut param_line:Vec<&str>;

    // getting the name to delete
    for app in parsed["appointments"].members() {
        if app["title"].to_string() == key.to_string() {
            return true;
        }
    }
    false
}

fn delete(params: &Vec<&str>) {
    let mut file_string = String::new();
    let mut file = File::open("dat/regan.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_string(&mut file_string).unwrap();
    let mut parsed = json::parse(&file_string).unwrap();
    let mut new_appoint_array = json::JsonValue::new_array();
    let mut param_line:Vec<&str>;
    let mut name_delete;

    // getting the name to delete

    name_delete = String::new();
    for item in params {
        param_line = item.split("=").collect();
        if param_line[0] == "app_to_delete" {
            name_delete = decode(param_line[1]);
        }
    }
    for app in parsed["appointments"].members() {
        if app["title"].to_string() != name_delete.to_string() {
            new_appoint_array.push(app.to_owned()).unwrap();
        }
    }
    parsed["appointments"] = new_appoint_array;
    file = File::create("dat/regan.json").unwrap();
    file.write_all((json::stringify_pretty(parsed,4)).as_bytes()).unwrap();
}

fn edit(param: &Vec<&str>) {

}

// Function for decoding post data

fn decode(input: &str) -> String {
    let input = input.replace("+", " ");
    let input = input.replace("%3A", ":");
    let input = input.replace("%2F", "/");
    let input = input.replace("%28", "(");
    let input = input.replace("%29", ")");
    input
}