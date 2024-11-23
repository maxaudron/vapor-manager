use std::collections::HashMap;

use nom::{
    number::streaming::{le_i32, u8},
    sequence::tuple,
};
use serde::{Deserialize, Serialize};

use crate::telemetry::broadcast::read_string;

use super::{write_string, BroadcastNetworkProtocolInbound, InboundMessageTypes};

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
        let mut out: Vec<u8> = Vec::new();
        out.push(Self::TYPE as u8);
        out.extend(0i32.to_le_bytes()); // Connection ID
        out.extend(write_string(&self.name));
        out.extend(self.id.to_le_bytes());
        out.extend(self.meters.to_le_bytes());

        out.push(self.camera_sets.len() as u8);
        for (name, sets) in &self.camera_sets {
            out.extend(write_string(&name));
            out.push(sets.len() as u8);
            for cam in sets {
                out.extend(write_string(&cam));
            }
        }

        out.push(self.hud_pages.len() as u8);
        for page in &self.hud_pages {
            out.extend(write_string(&page))
        }

        out
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
