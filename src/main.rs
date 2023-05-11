use walkdir::WalkDir;

fn main() {
    let ext = ".pem";
    let mut counter = 0;
    print!("{ext}");

    for entry in WalkDir::new("/")
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok()) {
        let f_name = entry.file_name().to_string_lossy();
        let f_path = entry.path().to_string_lossy();

        if f_name.ends_with(ext) {
            counter += 1;
            println!("{}", counter.to_string() + ": " + &f_path + "/" +  &f_name);
        }
    }
}