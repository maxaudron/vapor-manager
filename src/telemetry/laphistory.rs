use super::{Graphics, LapWheels, Physics, Wheels};

#[derive(Default, Debug, Clone)]
pub struct LapHistory {
    pub h_physics: Vec<Physics>,
    pub h_graphics: Vec<Graphics>,
}

macro_rules! avg_min_max {
    ($t:ident, $name:ident, $field:ident) => {
        paste::paste! {
            pub fn [<max_ $name>](&self) -> Wheels<$t> {
                self.h_physics
                    .iter()
                    .filter(|p| p.wheels.front_left.$field > 0.0
                                || p.wheels.front_right.$field > 0.0
                                || p.wheels.rear_left.$field > 0.0
                                || p.wheels.rear_right.$field > 0.0)
                    .fold(($t::MIN, $t::MIN, $t::MIN, $t::MIN), |mut wheels, p| {
                        if wheels.0 < p.wheels.front_left.$field {
                            wheels.0 = p.wheels.front_left.$field;
                        }
                        if wheels.1 < p.wheels.front_right.$field {
                            wheels.1 = p.wheels.front_right.$field;
                        }
                        if wheels.2 < p.wheels.rear_left.$field {
                            wheels.2 = p.wheels.rear_left.$field;
                        }
                        if wheels.3 < p.wheels.rear_right.$field {
                            wheels.3 = p.wheels.rear_right.$field;
                        }

                        wheels
                    })
                    .into()
            }

            pub fn [<min_ $name>](&self) -> Wheels<$t> {
                self.h_physics
                    .iter()
                    .filter(|p| p.wheels.front_left.$field > 0.0
                                || p.wheels.front_right.$field > 0.0
                                || p.wheels.rear_left.$field > 0.0
                                || p.wheels.rear_right.$field > 0.0)
                    .fold(($t::MAX, $t::MAX, $t::MAX, $t::MAX), |mut wheels, p| {
                        if wheels.0 > p.wheels.front_left.$field {
                            wheels.0 = p.wheels.front_left.$field;
                        }
                        if wheels.1 > p.wheels.front_right.$field {
                            wheels.1 = p.wheels.front_right.$field;
                        }
                        if wheels.2 > p.wheels.rear_left.$field {
                            wheels.2 = p.wheels.rear_left.$field;
                        }
                        if wheels.3 > p.wheels.rear_right.$field {
                            wheels.3 = p.wheels.rear_right.$field;
                        }

                        wheels
                    })
                    .into()
            }

            pub fn [<avg_ $name>](&self) -> Wheels<$t> {
                let wheels: Vec<&Physics> = self
                    .h_physics
                    .iter()
                    .filter(|p| p.wheels.front_left.$field > 0.0
                                || p.wheels.front_right.$field > 0.0
                                || p.wheels.rear_left.$field > 0.0
                                || p.wheels.rear_right.$field > 0.0
                                ).collect();
                let count = wheels.len() as $t;
                let wheels: Wheels<$t> = wheels.iter()
                    .fold(($t::default(), $t::default(), $t::default(), $t::default()), |mut wheels, p| {
                        wheels.0 += p.wheels.front_left.$field;
                        wheels.1 += p.wheels.front_right.$field;
                        wheels.2 += p.wheels.rear_left.$field;
                        wheels.3 += p.wheels.rear_right.$field;

                        wheels
                    })
                    .into();

                tracing::debug!("count: {:?} wheels: {:?}", count, wheels);
                wheels / (count, count, count, count).into()
            }
        }
    };
}

impl LapHistory {
    pub fn last_point(&self) -> Option<(&Physics, &Graphics)> {
        if self.h_physics.last().is_some() && self.h_graphics.last().is_some() {
            Some((
                self.h_physics.last().unwrap(),
                self.h_graphics.last().unwrap(),
            ))
        } else {
            None
        }
    }

    avg_min_max!(f32, brake_temperature, brake_temperature);
    avg_min_max!(f32, tyre_temperature, tyre_core_temperature);
    avg_min_max!(f32, tyre_pressure, tyre_pressure);
}

impl LapWheels {
    pub fn get_avg_min_max(&mut self, history: &LapHistory) {
        self.tyre_pressure = AvgMinMax {
            avg: history.avg_tyre_pressure(),
            min: history.min_tyre_pressure(),
            max: history.max_tyre_pressure(),
        };
        self.tyre_temperature = AvgMinMax {
            avg: history.avg_tyre_temperature(),
            min: history.min_tyre_temperature(),
            max: history.max_tyre_temperature(),
        };
        self.brake_temperature = AvgMinMax {
            avg: history.avg_brake_temperature(),
            min: history.min_brake_temperature(),
            max: history.max_brake_temperature(),
        };
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct AvgMinMax<T>
where
    T: std::fmt::Debug + Default + Clone + PartialEq,
{
    pub avg: T,
    pub min: T,
    pub max: T,
}
