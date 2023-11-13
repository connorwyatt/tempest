use std::process::exit;

use lazy_regex::regex;
use strum::EnumString;

#[derive(Debug)]
pub(crate) struct DataModel {
    nodes: Vec<Node>,
    links: Vec<Link>,
}

impl DataModel {
    pub(crate) fn from(input: String) -> Self {
        let mut nodes = Vec::new();
        let mut links = Vec::new();

        let node_check_regex = regex!("^(?<node_type>command|aggregate|event|policy)"i);
        let node_regex = regex!(
            r#"^(command|aggregate|event|policy)\s*\((?<id>[a-z0-9]*)\)?\s*:\s*"(?<text>.*)"$"#i
        );
        let link_check_regex = regex!("^link"i);
        let link_regex =
            regex!(r#"^link\s*:\s*(?<start_id>[a-z0-9]*)\s*->\s*(?<end_id>[a-z0-9]*)$"#i);

        for (index, line) in input.lines().enumerate() {
            let line_number = index + 1;
            let trimmed_line = line.trim();

            if trimmed_line.is_empty() {
                continue;
            }

            if let Some(check_captures) = node_check_regex.captures(trimmed_line) {
                let node_type_string = check_captures
                    .name("node_type")
                    .unwrap()
                    .as_str()
                    .to_string();

                let node_type = node_type_string.parse().unwrap();

                let Some(captures) = node_regex.captures(trimmed_line) else {
                    eprintln!(
                        "Could not parse {} (line {}):",
                        node_type_string.to_lowercase(),
                        line_number
                    );
                    eprintln!("    \"{}\"", trimmed_line);
                    exit(1);
                };

                let node = Node {
                    id: captures.name("id").unwrap().as_str().to_string(),
                    node_type,
                    text: captures.name("text").unwrap().as_str().to_string(),
                };
                nodes.push(node);
            } else if link_check_regex.is_match(trimmed_line) {
                let Some(captures) = link_regex.captures(trimmed_line) else {
                    eprintln!("Could not parse link (line {}):", line_number);
                    eprintln!("    \"{}\"", trimmed_line);
                    exit(1);
                };

                let link = Link {
                    start_id: captures.name("start_id").unwrap().as_str().to_string(),
                    end_id: captures.name("end_id").unwrap().as_str().to_string(),
                };
                links.push(link);
            } else {
                eprintln!("Could not parse (line {}):", line_number);
                eprintln!("    \"{}\"", trimmed_line);
                exit(1);
            }
        }

        Self { nodes, links }
    }
}

#[derive(Debug, EnumString)]
pub(crate) enum NodeType {
    #[strum(serialize = "command")]
    Command,
    #[strum(serialize = "aggregate")]
    Aggregate,
    #[strum(serialize = "event")]
    Event,
    #[strum(serialize = "policy")]
    Policy,
}

#[derive(Debug)]
pub(crate) struct Node {
    id: String,
    node_type: NodeType,
    text: String,
}

#[derive(Debug)]
pub(crate) struct Link {
    start_id: String,
    end_id: String,
}
