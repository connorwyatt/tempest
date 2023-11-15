use std::{
    fs,
    path,
};

use svg::{
    node::{
        element::Rectangle,
        Text,
    },
    Document,
};

use crate::{
    data_model::NodeType,
    layout::Layout,
};

pub(crate) fn render(layout: &Layout, output_file_path: String) -> Result<(), ()> {
    let mut document = Document::new().set("viewBox", (0, 0, 2560, 1440));

    let background_rect = Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("width", layout.size.width)
        .set("height", layout.size.height)
        .set("fill", THEME.background_color.to_rgb_string());

    document = document.add(background_rect);

    for node_layout in &layout.node_layouts {
        let rect = Rectangle::new()
            .set("x", node_layout.position.x)
            .set("y", node_layout.position.y)
            .set("width", node_layout.size.width)
            .set("height", node_layout.size.height)
            .set(
                "fill",
                get_node_fill(&node_layout.node.node_type).to_rgb_string(),
            );

        let text = Text::new(&node_layout.node.text);

        document = document.add(rect).add(text);
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
