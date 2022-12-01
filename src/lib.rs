pub mod io {
    use std::env;
    use std::fs;
    use std::path::Path;

    pub fn get_input(day: u8) -> String {
        let fpath = format!("inputs/{:02}.txt", day);
        let path = Path::new(&fpath);
        if path.exists() {
            return fs::read_to_string(path).unwrap();
        }

        let session_id: String = env::var("AOC_SESSION_ID").unwrap();
        let uri = format!("https://adventofcode.com/2022/day/{}/input", day.to_string());
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(uri)
            .header("Cookie", format!("session={}", session_id))
            .send()
            .unwrap();

        let contents = response.text().unwrap().trim().to_owned();
        match fs::write(path, &contents) {
            Ok(_) => { contents }
            Err(e) => { panic!("Could not write input to file: {}", e); }
        }
    }
}