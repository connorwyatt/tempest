use std::{
    fs,
    path,
};

use svg::{
    node::element::Rectangle,
    Document,
};

use crate::data_model::{
    DataModel,
    NodeType,
};

const STICKY_NOTE_WIDTH: usize = 200;

pub(crate) fn render(data_model: &DataModel, output_file_path: String) -> Result<(), ()> {
    let mut document = Document::new().set("viewBox", (0, 0, 2560, 1440));

    let background_rect = Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("width", 2560)
        .set("height", 1440)
        .set("fill", THEME.background_color.to_rgb_string());

    document = document.add(background_rect);

    for (i, node) in data_model.nodes.iter().enumerate() {
        let rect = Rectangle::new()
            .set("x", 50 + i * (STICKY_NOTE_WIDTH + 50))
            .set("y", 50)
            .set("width", STICKY_NOTE_WIDTH)
            .set("height", STICKY_NOTE_WIDTH)
            .set("fill", get_node_fill(&node.node_type).to_rgb_string());

        document = document.add(rect);
    }

    let output_directory = path::Path::new(&output_file_path)
        .parent()
        .expect("should be able to get parent directory");

    fs::create_dir_all(output_directory).expect("should be able to create directory");

    svg::save(output_file_path, &document).expect("should be able to save file");

    Ok(())
}

fn get_node_fill(node_type: &NodeType) -> Color {
    match node_type {
        NodeType::Command => THEME.command_color,
        NodeType::Aggregate => THEME.aggregate_color,
        NodeType::Event => THEME.event_color,
        NodeType::Policy => THEME.policy_color,
    }
}

const THEME: Theme = Theme {
    background_color: Color(30, 30, 30),
    command_color: Color(100, 200, 255),
    aggregate_color: Color(200, 200, 100),
    event_color: Color(255, 150, 75),
    policy_color: Color(200, 100, 200),
};

pub(crate) struct Theme {
    pub(crate) background_color: Color,
    pub(crate) command_color: Color,
    pub(crate) aggregate_color: Color,
    pub(crate) event_color: Color,
    pub(crate) policy_color: Color,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Color(u8, u8, u8);

impl Color {
    fn to_rgb_string(&self) -> String {
        format!("rgb({}, {}, {})", self.0, self.1, self.2)
    }
}
