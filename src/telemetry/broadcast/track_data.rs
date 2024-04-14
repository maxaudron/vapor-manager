use std::collections::HashMap;

use nom::{
    number::complete::{le_i32, u8},
    sequence::tuple,
};
use serde::{Deserialize, Serialize};

use crate::telemetry::broadcast::read_string;

use super::{BroadcastNetworkProtocolInbound, InboundMessageTypes};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TrackData {
    pub name: String,
    pub id: i32,
    pub meters: i32,
    pub camera_sets: HashMap<String, Vec<String>>,
    pub hud_pages: Vec<String>,
}

impl BroadcastNetworkProtocolInbound for TrackData {
    const TYPE: InboundMessageTypes = InboundMessageTypes::TrackData;

    fn serialize(&self) -> Vec<u8> {
        todo!()
    }

    fn deserialize(input: &[u8]) -> nom::IResult<&[u8], Self>
    where
        Self: Sized,
    {
        let (input, (_connection_id, name, id, meters)) =
            tuple((le_i32, read_string, le_i32, le_i32))(input)?;

        let (mut input, camera_set_count) = u8(input)?;

        let mut camera_sets = HashMap::new();
        for _ in 0..camera_set_count {
            let camera_set_name;
            let camera_count;
            (input, camera_set_name) = read_string(input)?;
            (input, camera_count) = u8(input)?;

            let mut cameras = Vec::new();
            for _ in 0..camera_count {
                let camera_name;
                (input, camera_name) = read_string(input)?;
                cameras.push(camera_name.to_owned());
            }

            camera_sets.insert(camera_set_name.to_owned(), cameras);
        }

        let (mut input, hud_pages_count) = u8(input)?;
        let mut hud_pages = Vec::new();
        for _ in 0..hud_pages_count {
            let hud_page;
            (input, hud_page) = read_string(input)?;
            hud_pages.push(hud_page.to_owned());
        }

        Ok((
            input,
            Self {
                name: name.to_owned(),
                id,
                meters,
                camera_sets,
                hud_pages,
            },
        ))
    }
}
