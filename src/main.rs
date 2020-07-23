use std::{ env, fs };
fn main() {
    let args: Vec<String> = env::args().collect();
    let path: String = args[1].clone();
    let _settings: String = args[2].clone();
    let files: Vec<String> = vec![String::new(); 64];
    let _file_list = fetch_files(path, files);
}

fn fetch_files(_path: String, arr: Vec<String>) -> Vec<String> {
    let paths = fs::read_dir(_path).unwrap();
    for path in paths {
        let path_name: String = path.unwrap().path().display().to_string();
        if path.unwrap().path().is_dir() {
            let next: Vec<String> = fetch_files(path_name, arr;
            let mut full = Vec::<String>::with_capacity(next.len() + arr.len());
            full.extend(arr);
            full.extend(next);
        }
    }
    return arr;
}   
