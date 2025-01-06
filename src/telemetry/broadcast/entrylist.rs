use std::collections::HashMap;

use nom::{
    number::streaming::{le_i32, le_u16, u8},
    sequence::tuple,
};
use num_enum::FromPrimitive;
use serde::{Deserialize, Serialize};

use crate::telemetry::broadcast::read_string;

use super::{
    write_string, BroadcastNetworkProtocolInbound, DriverCategory, InboundMessageTypes, Nationality,
};

/// Entry list of the server with all cars and drivers
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EntryList {
    pub connection_id: i32,
    /// All cars in the entrylist
    ///
    /// Will only be populated when receiving the ENTRY_LIST_CAR message for the car
    pub cars: HashMap<i32, CarInfo>,
}

impl BroadcastNetworkProtocolInbound for EntryList {
    const TYPE: InboundMessageTypes = InboundMessageTypes::EntryList;

    fn serialize(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.extend(self.connection_id.to_le_bytes());
        out.extend((self.cars.len() as u16).to_le_bytes());
        for (_, car) in &self.cars {
            out.extend(BroadcastNetworkProtocolInbound::serialize(car));
        }

        out
    }

    fn deserialize(input: &[u8]) -> nom::IResult<&[u8], Self>
    where
        Self: Sized,
    {
        let (input, connection_id) = le_i32(input)?;

        // Read in car ids but discard for now
        // we will add these when we receive the full car structure
        let (mut input, car_num) = le_u16(input)?;
        for _i in 0..car_num {
            let (out, _car) = le_u16(input)?;
            input = out;
        }

        Ok((
            input,
            Self {
                connection_id,
                cars: HashMap::new(),
            },
        ))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CarInfo {
    id: u16,
    model_type: u8,
    team_name: String,
    race_number: i32,
    cup_category: DriverCategory,
    current_driver_id: u8,
    nationality: Nationality,
    drivers: Vec<Driver>,
}

impl BroadcastNetworkProtocolInbound for CarInfo {
    const TYPE: InboundMessageTypes = InboundMessageTypes::EntryListCar;

    fn serialize(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();

        out.extend(self.id.to_le_bytes());
        out.push(self.model_type);
        out.extend(write_string(&self.team_name));
        out.extend(self.race_number.to_le_bytes());
        out.push(self.cup_category as u8);
        out.push(self.current_driver_id);
        out.extend((self.nationality as u16).to_le_bytes());
        out.push(self.drivers.len() as u8);
        for driver in &self.drivers {
            out.extend(BroadcastNetworkProtocolInbound::serialize(driver));
        }

        out
    }

    fn deserialize(input: &[u8]) -> nom::IResult<&[u8], Self>
    where
        Self: Sized,
    {
        let (input, (id, model_type, team_name, race_number, cup_category, current_driver_id, nationality)) =
            tuple((le_u16, u8, read_string, le_i32, u8, u8, le_u16))(input)?;

        let (mut input, driver_num) = u8(input)?;
        let mut drivers = Vec::new();
        for _ in 0..driver_num {
            let (out, driver) = <Driver as BroadcastNetworkProtocolInbound>::deserialize(input)?;
            input = out;
            drivers.push(driver)
        }

        Ok((
            input,
            Self {
                id,
                model_type,
                team_name: team_name.to_string(),
                race_number,
                cup_category: DriverCategory::from_primitive(cup_category),
                current_driver_id,
                nationality: Nationality::from_primitive(nationality),
                drivers,
            },
        ))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Driver {
    first_name: String,
    last_name: String,
    short_name: String,
    category: DriverCategory,
    nationality: Nationality,
}

impl BroadcastNetworkProtocolInbound for Driver {
    const TYPE: InboundMessageTypes = InboundMessageTypes::EntryListCar;

    fn serialize(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.extend(write_string(&self.first_name));
        out.extend(write_string(&self.last_name));
        out.extend(write_string(&self.short_name));
        out.push(self.category as u8);
        out.extend((self.nationality as u16).to_le_bytes());

        out
    }

    fn deserialize(input: &[u8]) -> nom::IResult<&[u8], Self>
    where
        Self: Sized,
    {
        let (input, (first_name, last_name, short_name, category, nationality)) =
            tuple((read_string, read_string, read_string, u8, le_u16))(input)?;

        Ok((
            input,
            Self {
                first_name: first_name.to_owned(),
                last_name: last_name.to_owned(),
                short_name: short_name.to_owned(),
                category: DriverCategory::from_primitive(category),
                nationality: Nationality::from_primitive(nationality),
            },
        ))
    }
}
