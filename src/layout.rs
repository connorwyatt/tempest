use crate::data_model::{
    DataModel,
    Link,
    Node,
};

const STICKY_NOTE_WIDTH: Precision = 200;

pub(crate) fn create_layout(data_model: DataModel) -> Layout {
    let mut node_layouts: Vec<NodeLayout> = Vec::new();

    for (i, node) in data_model.nodes.iter().cloned().enumerate() {
        node_layouts.push(NodeLayout {
            node,
            position: Position {
                x: (50 + (i * STICKY_NOTE_WIDTH) + (i * 50)),
                y: 50,
            },
            size: Size {
                width: STICKY_NOTE_WIDTH,
                height: STICKY_NOTE_WIDTH,
            },
        });
    }

    let link_layouts = data_model
        .links
        .iter()
        .map(|link| {
            let start_node_layout = node_layouts
                .iter()
                .find(|&node_layout| node_layout.node.id == link.start_id)
                .expect("should be able to find start node layout");
            let end_node_layout = node_layouts
                .iter()
                .find(|&node_layout| node_layout.node.id == link.end_id)
                .expect("should be able to find end node layout");

            LinkLayout {
                link: link.clone(),
                start_position: Position {
                    x: start_node_layout.position.x + start_node_layout.size.width,
                    y: start_node_layout.position.y + (start_node_layout.size.height / 2),
                },
                end_position: Position {
                    x: end_node_layout.position.x,
                    y: end_node_layout.position.y + (end_node_layout.size.height / 2),
                },
            }
        })
        .collect::<Vec<_>>();

    Layout {
        size: Size {
            width: 2560,
            height: 1440,
        },
        node_layouts,
        link_layouts,
    }
}

type Precision = usize;

pub(crate) struct Layout {
    pub(crate) size: Size,
    pub(crate) node_layouts: Vec<NodeLayout>,
    pub(crate) link_layouts: Vec<LinkLayout>,
}

pub(crate) struct NodeLayout {
    pub(crate) node: Node,
    pub(crate) position: Position,
    pub(crate) size: Size,
}

pub(crate) struct LinkLayout {
    pub(crate) link: Link,
    pub(crate) start_position: Position,
    pub(crate) end_position: Position,
}

pub(crate) struct Position {
    pub(crate) x: Precision,
    pub(crate) y: Precision,
}

pub(crate) struct Size {
    pub(crate) width: Precision,
    pub(crate) height: Precision,
}
