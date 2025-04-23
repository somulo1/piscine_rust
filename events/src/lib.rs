use chrono::Duration;
use colored::*;
use std::fmt;

// Position enum for notification position (Top, Bottom, Center)
#[derive(Debug, Eq, PartialEq)]
pub enum Position {
    Top,
    Bottom,
    Center,
}

// Notification struct to hold notification details
#[derive(Debug, Eq, PartialEq)]
pub struct Notification {
    pub size: u32,
    pub color: (u8, u8, u8),
    pub position: Position,
    pub content: String,
}

// Event enum to represent different types of events
#[derive(Debug)]
pub enum Event<'a> {
    Remainder(&'a str),
    Registration(Duration),
    Appointment(&'a str),
    Holiday,
}

// Implementing Display trait for Notification to format output
impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color = format!(
            "\x1b[38;2;{};{};{}m",
            self.color.0, self.color.1, self.color.2
        );
        write!(
            f,
            "({:?}, {}, {}{}{})",
            self.position,
            self.size,
            color,
            self.content,
            "\x1b[0m"  // Reset color
        )
    }
}

impl Event<'_> {
    // Notify method that returns a formatted Notification based on event type
    pub fn notify(&self) -> Notification {
        match self {
            Event::Remainder(text) => Notification {
                size: 50,
                color: (50, 50, 50),
                position: Position::Bottom,
                content: text.to_string(),
            },
            Event::Registration(duration) => {
                let hours = duration.num_hours();
                let minutes = duration.num_minutes() % 60;
                let seconds = duration.num_seconds() % 60;
                Notification {
                    size: 30,
                    color: (255, 2, 22),
                    position: Position::Top,
                    content: format!(
                        "You have {}H:{}M:{}S left before the registration ends",
                        hours, minutes, seconds
                    ),
                }
            }
            Event::Appointment(text) => Notification {
                size: 100,
                color: (200, 200, 3),
                position: Position::Center,
                content: text.to_string(),
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

// fn main() {
//     // Testing the notify method
//     let remainder = Event::Remainder("Go to the doctor");
//     println!("{}", remainder.notify());
    
//     let registration = Event::Registration(Duration::seconds(49094));
//     println!("{}", registration.notify());
    
//     let appointment = Event::Appointment("Go to the doctor");
//     println!("{}", appointment.notify());
    
//     let holiday = Event::Holiday;
//     println!("{}", holiday.notify());
// }
