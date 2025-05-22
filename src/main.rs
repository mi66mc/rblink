use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs::{self, ReadDir};
use std::path::{Path, PathBuf};
use std::io::Error;
use std::process::Command;

fn get_args() -> Vec<String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <port>", args[0]);
        std::process::exit(1);
    }
    args
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    let request_line = request.lines().next().unwrap();
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    let method = parts[0];
    if parts.len() < 2 {
        println!("Invalid request: {}", request_line);
        return;
    }

    let path = parts[1];

    println!("Request: {} {}", method, path);

    if method == "GET" {
        let response = route(path);
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", response);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let response = "HTTP/1.1 405 Method Not Allowed\r\n\r\n";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn html_list(list: Result<ReadDir, Error>) -> String {
    let mut dirs: Vec<PathBuf> = vec![];
    let mut files: Vec<PathBuf> = vec![];
    let mut res = String::new();
    res.push_str("<!DOCTYPE html>");
    res.push_str("<html lang=\"en\">");
    res.push_str("<head>");
    res.push_str("<meta charset=\"UTF-8\">");
    res.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">");
    res.push_str("<title>Directory Listing</title>");
    res.push_str("<style>");
    res.push_str("body { font-family: Arial, sans-serif; }");
    res.push_str("body { background-color: #121212; color: #ffffff; }");
    res.push_str("h1 { color: #ffffff; }");
    res.push_str("h2 { color: #ffffff; }");
    res.push_str("ul { list-style-type: none; padding: 0; }");
    res.push_str("li { margin: 5px 0; }");
    res.push_str("a { text-decoration: none; color: #007BFF; }");
    res.push_str("a:hover { text-decoration: underline; }");
    res.push_str("a:visited { color: #007BFF; }");
    res.push_str("a:active { color: #007BFF; }");
    res.push_str("a:focus { color: #007BFF; }");
    res.push_str("li a { display: block; padding: 10px; }");
    res.push_str("</style>");
    res.push_str("</head>");
    res.push_str("<body>");
    res.push_str("<h1>Directory Listing</h1>");
    res.push_str("<h2>Available files:</h2>");
    res.push_str("<ul>");
    if let Ok(entries) = list {
        for entry in entries {
            let entry = entry.unwrap();
            let file_path = entry.path();
            if file_path.is_dir() {
                dirs.push(file_path);
            } else {
                files.push(file_path);
            }
        }
    } else {
        println!("Error reading directory");
    }

    for dir in dirs {
        let dir_name = dir.file_name().unwrap().to_str().unwrap();
        let dir_path = dir.to_str().unwrap();
        let dir_path = dir_path.replace("//", "/");
        let dir_path = dir_path.trim_end_matches('/');
        res.push_str(&format!("<li><a href=\"{}\">📁 {}/</a></li>", dir_path, dir_name));
    }

    for file in files {
        let file_name = file.file_name().unwrap().to_str().unwrap();
        let file_path = file.to_str().unwrap();
        let file_path = file_path.replace("//", "/");
        res.push_str(&format!("<li><a href=\"{}\">📄 {}</a></li>", file_path, file_name));
    }

    res
}

fn route(path: &str) -> String {
    if path == "/" {
        let path = format!("./index.html");
        let path = Path::new(&path);
        if path.exists() {
            let content = fs::read_to_string(path).unwrap();
            return content;
        }
    }
    
    let path = format!("./{}", path);
    let path = Path::new(&path);
    if path.exists() {
        if path.is_dir() {
            html_list(Ok(fs::read_dir(&path).unwrap()))
        } else {
            let content = fs::read_to_string(path).unwrap();
            content
        }
    } else {
        let response = "HTTP/1.1 404 Not Found\r\n\r\n";
        format!("{}404 Not Found", response)
    }

}

fn open_browser(url: &str) {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "start", url])
            .spawn()
            .expect("failed to open browser");
    } else if cfg!(target_os = "macos") {
        Command::new("open")
            .arg(url)
            .spawn()
            .expect("failed to open browser");
    } else if cfg!(target_os = "linux") {
        Command::new("xdg-open")
            .arg(url)
            .spawn()
            .expect("failed to open browser");
    } else {
        eprintln!("failed to open browser: unsupported OS");
    }
}

fn main() {
    let args = get_args();
    let port: u16 = args[1].parse().unwrap();

    let listener = TcpListener::bind(format!("127.0.0.1:{port}"))
        .expect("Could not bind to address");
    println!("Listening on http://{}", listener.local_addr().unwrap());
    println!("Press Ctrl+C to stop the server.");
    
    let url = format!("http://127.0.0.1:{port}");
    open_browser(&url);
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}