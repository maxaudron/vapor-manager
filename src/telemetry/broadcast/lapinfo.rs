use nom::{
    number::complete::{le_i32, le_u16, u8},
    sequence::tuple,
    IResult,
};
use serde::{Deserialize, Serialize};

/// Information about a single Lap
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct LapInfo {
    /// If lap is In, Out or Regular Lap
    pub lap_type: LapType,
    /// Laptime in milliseconds
    pub laptime: Option<i32>,
    /// Index of the car
    pub car_index: u16,
    /// Index of the driver
    pub driver_index: u16,
    /// Timing Splits
    pub splits: Vec<Option<i32>>,
    /// Lap is invalidated
    pub invalid: bool,
    /// Lap is best time
    pub valid_for_best: bool,
}

/// Read a Lap
pub fn read_lap(input: &[u8]) -> IResult<&[u8], LapInfo> {
    let (input, (laptime, car_index, driver_index)) = tuple((le_i32, le_u16, le_u16))(input)?;
    let (input, splits) = read_lap_split(input)?;

    let (input, (invalid, valid_for_best, out_lap, in_lap)) = tuple((u8, u8, u8, u8))(input)?;

    let lap_type = LapType::new(out_lap > 0, in_lap > 0);

    let lap = LapInfo {
        laptime: if laptime == i32::MAX {
            None
        } else {
            Some(laptime)
        },
        car_index,
        driver_index,
        splits,
        invalid: invalid > 0,
        valid_for_best: valid_for_best > 0,
        lap_type,
    };

    Ok((input, lap))
}

/// Read Splits
/// first one u8 for the number of splits
/// then one i32 for each split time in ms
/// if the split does not have a time it will be set to i32::MAX
/// in which case we set it to None, and also ensure at least 3 splits exist
fn read_lap_split(input: &[u8]) -> IResult<&[u8], Vec<Option<i32>>> {
    let mut splits = Vec::new();
    let (mut input, number) = u8(input)?;
    for _i in 0..number {
        let (out, split) = le_i32(input)?;
        input = out;
        if split == i32::MAX {
            splits.push(None);
        } else {
            splits.push(Some(split));
        }
    }

    while splits.len() < 3 {
        splits.push(None)
    }

    Ok((input, splits))
}

#[repr(C)]
#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum LapType {
    #[default]
    ERROR = 0,
    Outlap = 1,
    Regular = 2,
    Inlap = 3,
}

impl LapType {
    pub fn new(out_lap: bool, in_lap: bool) -> LapType {
        if out_lap {
            LapType::Outlap
        } else if in_lap {
            LapType::Inlap
        } else {
            LapType::Regular
        }
    }
}

#[test]
fn test_lapinfo_deserialize() {
    let input = [
        255, 255, 255, 127, 0, 0, 0, 0, 3, 255, 255, 255, 127, 255, 255, 255, 127, 255, 255, 255,
        127, 0, 1, 0, 0, 0, 0, 0, 0,
    ];

    let lapinfo = LapInfo {
        lap_type: LapType::Regular,
        laptime: None,
        car_index: 0,
        driver_index: 0,
        splits: vec![None, None, None],
        invalid: false,
        valid_for_best: true,
    };

    let (_input, parsed) = read_lap(&input).unwrap();
    assert_eq!(lapinfo, parsed)
}
