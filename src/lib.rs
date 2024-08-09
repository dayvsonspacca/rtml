use std::{collections::LinkedList, fs::read};


pub fn view(rtml: &str, variables: Vec<(&str, &str)>) -> String {
    let mut view = String::new();
    let file = read(rtml).expect("Failed to read file");

    let mut tag_stack: LinkedList<Element> = get_tag_stack(file);

    while !tag_stack.is_empty() {
        let mut tag = tag_stack.pop_front().unwrap();

        for (key, value) in variables.iter() {
            if tag.content.contains(format!("{{{{{}}}}}", key).as_str()) {
                tag.content = tag
                    .content
                    .replace(format!("{{{{{}}}}}", key).as_str(), value);
            }
            view.push_str(&format!("{}{}", tag.content, tag.tag_name));
        }
    }

    view
}

fn get_tag_stack(file: Vec<u8>) -> LinkedList<Element> {
    let mut current_element = String::new();
    let mut tag_stack: LinkedList<Element> = LinkedList::new();

    for c in file.iter().map(|&c| c as char) {
        current_element.push(c);

        if c == '>' {
            let mut tag = String::new();
            let mut inside_tag = false;

            for c in current_element.chars() {
                if c == '<' {
                    inside_tag = true;
                }

                if inside_tag {
                    tag.push(c);
                }
            }

            tag_stack.push_back(Element {
                tag_name: tag.trim().to_string(),
                content: current_element[..current_element.len() - tag.len()]
                    .trim()
                    .to_string(),
            });

            current_element.clear();
        }
    }

    tag_stack
}

#[derive(Debug)]
struct Element {
    tag_name: String,
    content: String,
}
