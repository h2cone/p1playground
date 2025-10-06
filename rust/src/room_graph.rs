use std::collections::HashMap;

#[derive(Copy, Clone)]
pub struct RoomNeighbors {
    pub left: Option<&'static str>,
    pub right: Option<&'static str>,
    pub up: Option<&'static str>,
    pub down: Option<&'static str>,
}

pub struct RoomGraph {
    graph: HashMap<&'static str, RoomNeighbors>,
}

impl RoomGraph {
    pub fn new() -> Self {
        let mut graph = HashMap::new();

        graph.insert(
            "room_0_0",
            RoomNeighbors {
                left: None,
                right: Some("room_1_0"),
                up: None,
                down: None,
            },
        );

        graph.insert(
            "room_1_0",
            RoomNeighbors {
                left: Some("room_0_0"),
                right: Some("room_2_0"),
                up: None,
                down: None,
            },
        );

        graph.insert(
            "room_2_0",
            RoomNeighbors {
                left: Some("room_1_0"),
                right: None,
                up: None,
                down: None,
            },
        );

        Self { graph }
    }

    pub fn get_neighbors(&self, room_name: &str) -> RoomNeighbors {
        self.graph.get(room_name).copied().unwrap_or(RoomNeighbors {
            left: None,
            right: None,
            up: None,
            down: None,
        })
    }

    pub fn all_rooms(&self) -> Vec<&'static str> {
        self.graph.keys().copied().collect()
    }
}
