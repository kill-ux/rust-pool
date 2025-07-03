use chrono::{Duration, TimeDelta, TimeZone, Timelike, Utc};
use colored::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Position {
    Top,
    Bottom,
    Center,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Notification {
    pub size: u32,
    pub color: (u8, u8, u8),
    pub position: Position,
    pub content: String,
}

#[derive(Debug)]
pub enum Event<'a> {
    Remainder(&'a str),
    Registration(Duration),
    Appointment(&'a str),
    Holiday,
}

use std::fmt;

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({:?}, {} , {})",
            self.position,
            self.size,
            self.content
                .truecolor(self.color.0, self.color.1, self.color.2)
        )
    }
}

use Event::*;

impl Event<'_> {
    pub fn notify(&self) -> Notification {
        match self {
            Event::Remainder(str) => Notification {
                size: 50,
                color: (50, 50, 50),
                position: Position::Bottom,
                content: str.to_string(),
            },
            Event::Registration(duration) => {
                let date = Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap()
                    + TimeDelta::try_seconds(duration.num_seconds()).unwrap();
                let hours = date.hour();
                let minutes = date.minute();
                let seconds = date.second();
                Notification {
                    size: 30,
                    color: (255, 2, 22),
                    position: Position::Top,
                    content: format!(
                        "You have {hours}H:{minutes}M:{seconds}S left before the registration ends"
                    ),
                }
            }
            Event::Appointment(str) => Notification {
                size: 100,
                color: (200, 200, 3),
                position: Position::Center,
                content: str.to_string(),
            },
            Event::Holiday => Notification {
                size: 25,
                color: (0, 255, 0),
                position: Position::Top,
                content: "Enjoy your holiday".to_string(),
            },
        }
    }
}
