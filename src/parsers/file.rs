use std::path::Path;
pub fn facebook_file_parser(folder: &String, name: &String) -> Vec<String> {
    let mut correct_paths: Vec<String> = [].to_vec();

    let files_path = Path::new(folder);
    let paths = files_path.read_dir().expect("Couldn't read directory");

    for folder_path in paths {
        let friend_path = folder_path
            .expect("Cannot read file")
            .file_name()
            .to_str()
            .expect("Cannot read file")
            .to_owned();

        let absolute_path = files_path.join(friend_path.clone());

        if friend_path.contains(name) {
            let mut mes_idx: i32 = 1;
            loop {
                let msg_json_path = absolute_path.join(format!("message_{mes_idx}.json").as_str());

                if msg_json_path.exists() {
                    mes_idx += 1;
                    correct_paths.push(msg_json_path.to_string_lossy().to_string());
                } else {
                    break;
                }
            }
            break;
        }
    }
    return correct_paths;
}
