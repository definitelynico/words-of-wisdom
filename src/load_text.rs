use std::{fs::File, io::Read};

pub fn get_text_from_file(files: Vec<File>) -> Vec<Vec<String>> {
    let mut text_vec: Vec<String> = vec![String::new(), String::new(), String::new()];

    let mut c = 0;
    for mut f in files {
        let mut tmp = String::new();
        f.read_to_string(&mut tmp)
            .expect("Couldn't read file content.");

        text_vec[c] = tmp;
        c += 1;
    }

    let mut list_1 = Vec::new();
    let mut list_2 = Vec::new();
    let mut list_3 = Vec::new();

    for i in text_vec[0].split(", ") {
        list_1.push(i.to_string());
    }
    for i in text_vec[1].split(", ") {
        list_2.push(i.to_string());
    }
    for i in text_vec[2].split(", ") {
        list_3.push(i.to_string());
    }

    let split_vec = vec![list_1, list_2, list_3];

    return split_vec;
}
