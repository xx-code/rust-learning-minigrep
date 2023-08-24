use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    if is_two_args(&args) {
        if !is_txt_file_there(&args) {
            panic!("We accept only txt file")
        }
        let list_line = read_file(&args[2]);
        // verification
        /*for line in list_line {
            println!("- {line}");
        }*/
        let text_found = found_txt(&args[1], &list_line);
        if text_found.len() > 0 {
            for (text, line_text) in &text_found {
                println!("- {text}, line number: {line_text}");
            }
        } else {
            println!("Word not found in the text");
        }
    } else {
        panic!("We have to add the name you want to find and the file where we have to search")
    }
}

fn is_two_args(args: &Vec<String>) -> bool {
    args.len() == 3 
}

fn is_txt_file_there(args: &Vec<String>) -> bool {
    args[2].contains("txt")
}

fn read_file(txt_file: &String) -> Vec<String>{
    let txt = fs::read_to_string(txt_file)
        .expect("error while reading file");
    let list_txt = txt.split("\n");
    let mut lines = Vec::new();
    for line in list_txt{
        lines.push(line.to_string());
    }
    lines
}

fn found_txt(search_txt: &String, lines_txt: &Vec<String>) -> HashMap<String, u32>{
    let mut result = HashMap::new();
    let mut count = 1;
    for line in lines_txt {
        if line.to_uppercase().contains(&search_txt.to_uppercase()) {
            let mut get_line = line.clone();
            if get_line.contains('\r') {
                let idx_new_char = get_line.find('\r').unwrap();
                get_line.remove(idx_new_char);
            }
            
            result.insert(get_line, count);
        }
        count += 1;
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn accept_only_two_element() {
        let args_absent = vec!["base".to_string()];
        assert!(!is_two_args(&args_absent));

        let args_miss_one = vec!["base".to_string(), "arg1".to_string()];
        assert!(!is_two_args(&args_miss_one));

        let args_complet = vec!["base".to_string(), "arg1".to_string(), "arg2".to_string()];
        assert!(is_two_args(&args_complet));
    }

    #[test]
    fn accept_only_txt_for_second_element() {
        let mut args = vec!["base".to_string(), "arg".to_string(), "arg.file".to_string()];
        assert!(!is_txt_file_there(&args));
        
        let new_file = String::from("arg.txt");
        args[2] = new_file;
        assert!(is_txt_file_there(&args))
    }

    #[test]
    #[should_panic]
    fn return_txt_file_in_vector() {
        let result = vec!["element1", "element2", "element3"];
        let txt = read_file(&"test.txt".to_string());
        assert_eq!(result, txt);
    }

    #[test]
    fn return_line_and_num_line(){
        let mut result = HashMap::new();
        result.insert("Junior est un fou!".to_string(), 1);
        result.insert("serieux il a laisse cette belle fille, ce Junior est vraiment fou!".to_string(), 3);
        result.insert("le mot junior n'est pas seulement un nom mais aussi un titre.".to_string(), 4);
        result.insert("Dans cette Auguste assemble, les personnes qui avait ce titre de \"junior\" etaient vraiment calme.".to_string(), 6);

        let search_txt = "Junior".to_string();
        let lines_txt = read_file(&"test_find.txt".to_string());
        let elements_found = found_txt(&search_txt, &lines_txt);

        assert_eq!(result, elements_found);
    }
}