use std::process::exit;

use lazy_regex::regex;
use strum::EnumString;

#[derive(Debug, PartialEq)]
pub(crate) struct DataModel {
    nodes: Vec<Node>,
    links: Vec<Link>,
}

impl DataModel {
    pub(crate) fn from(input: String) -> Self {
        let mut nodes = Vec::new();
        let mut links = Vec::new();

        for (index, line) in input.lines().map(|x| x.trim()).enumerate() {
            let line_number = index + 1;

            if line.is_empty() {
                continue;
            }

            if let Some(node) = Self::parse_node(line, line_number) {
                nodes.push(node);
            } else if let Some(link) = Self::parse_link(line, line_number) {
                links.push(link);
            } else {
                eprintln!("Could not parse (line {}):", line_number);
                eprintln!("    \"{}\"", line);
                exit(1);
            }
        }

        Self { nodes, links }
    }

    fn parse_node(input: &str, line_number: usize) -> Option<Node> {
        let check_regex = regex!("^(?<node_type>command|aggregate|event|policy)"i);
        let regex = regex!(
            r#"^(command|aggregate|event|policy)\s*\((?<id>[a-z0-9]*)\)?\s*:\s*"(?<text>.*)"$"#i
        );

        let Some(check_captures) = check_regex.captures(input) else {
            return None;
        };

        let node_type_string = check_captures
            .name("node_type")
            .unwrap()
            .as_str()
            .to_string();

        let node_type = node_type_string.parse().unwrap();

        let Some(captures) = regex.captures(input) else {
            eprintln!(
                "Could not parse {} (line {}):",
                node_type_string.to_lowercase(),
                line_number
            );
            eprintln!("    \"{}\"", input);
            exit(1);
        };

        Some(Node {
            id: captures.name("id").unwrap().as_str().to_string(),
            node_type,
            text: captures.name("text").unwrap().as_str().to_string(),
        })
    }

    fn parse_link(input: &str, line_number: usize) -> Option<Link> {
        let check_regex = regex!("^link"i);
        let regex = regex!(r#"^link\s*:\s*(?<start_id>[a-z0-9]*)\s*->\s*(?<end_id>[a-z0-9]*)$"#i);

        if !check_regex.is_match(input) {
            return None;
        }

        let Some(captures) = regex.captures(input) else {
            eprintln!("Could not parse link (line {}):", line_number);
            eprintln!("    \"{}\"", input);
            exit(1);
        };

        Some(Link {
            start_id: captures.name("start_id").unwrap().as_str().to_string(),
            end_id: captures.name("end_id").unwrap().as_str().to_string(),
        })
    }
}

#[derive(Debug, EnumString, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub(crate) struct Node {
    id: String,
    node_type: NodeType,
    text: String,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Link {
    start_id: String,
    end_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_a_string_works() {
        let string = include_str!("../sample_files/sample.tem");

        let data_model = DataModel::from(string.to_string());

        assert_eq!(
            data_model,
            DataModel {
                nodes: vec![
                    Node {
                        id: String::from("addCustomerCommand"),
                        node_type: NodeType::Command,
                        text: String::from("Add Customer"),
                    },
                    Node {
                        id: String::from("addCustomerAggregate"),
                        node_type: NodeType::Aggregate,
                        text: String::from("Customer"),
                    },
                    Node {
                        id: String::from("addCustomerEvent"),
                        node_type: NodeType::Event,
                        text: String::from("Customer Added"),
                    },
                    Node {
                        id: String::from("verifyCustomerPolicy"),
                        node_type: NodeType::Policy,
                        text: String::from("Verify Customer Policy"),
                    },
                    Node {
                        id: String::from("deleteCustomerCommand"),
                        node_type: NodeType::Command,
                        text: String::from("Delete Customer"),
                    },
                    Node {
                        id: String::from("deleteCustomerAggregate"),
                        node_type: NodeType::Aggregate,
                        text: String::from("Customer"),
                    },
                    Node {
                        id: String::from("deleteCustomerEvent"),
                        node_type: NodeType::Event,
                        text: String::from("Customer Deleted"),
                    },
                ],
                links: vec![
                    Link {
                        start_id: String::from("addCustomerCommand"),
                        end_id: String::from("addCustomerAggregate"),
                    },
                    Link {
                        start_id: String::from("addCustomerAggregate"),
                        end_id: String::from("addCustomerEvent"),
                    },
                    Link {
                        start_id: String::from("addCustomerEvent"),
                        end_id: String::from("checkCustomerPolicy"),
                    },
                    Link {
                        start_id: String::from("deleteCustomerCommand"),
                        end_id: String::from("deleteCustomerAggregate"),
                    },
                    Link {
                        start_id: String::from("deleteCustomerAggregate"),
                        end_id: String::from("deleteCustomerEvent"),
                    },
                ]
            }
        );
    }
}
