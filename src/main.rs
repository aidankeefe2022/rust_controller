#![warn(non_snake_case)]

use std::error::Error;
use gilrs::{Gilrs, Button, Event, Axis};
use rppal::pwm::{Channel, Pwm};

fn run_machine() -> Result<(), Box<dyn Error>>{
    let mut gilrs = Gilrs::new()?;
    let pwm = Pwm::new(Channel::Pwm0)?;




    loop {
        while let Some(Event { id, event: _event, time: _time }) = gilrs.next_event() {
            let active_xbox = Some(id);
            if let Some(gamepad) = active_xbox.map(|id| gilrs.gamepad(id)) {

                if (gamepad.axis_data(Axis::LeftStickX).map(|axis_data| axis_data.value()).unwrap_or(0.0)) < 0.0 {

                    println!("left {}", gamepad.axis_data(Axis::LeftStickX).map(|axis_data| axis_data.value()).unwrap_or(0.0));

                } else if (gamepad.axis_data(Axis::LeftStickX).map(|axis_data| axis_data.value()).unwrap_or(0.0)) > 0.0 {

                    println!("right {}", gamepad.axis_data(Axis::LeftStickX).map(|axis_data| axis_data.value()).unwrap_or(0.0));

                } else if gamepad.axis_data(Axis::LeftStickX).map(|axis_data| axis_data.value()).unwrap_or(0.0) == 0.0{
                    println!("{}", gamepad.axis_data(Axis::LeftStickX).map(|axis_data| axis_data.value()).unwrap_or(0.0))
                }


                if (gamepad.button_data(Button::RightTrigger2).map(|button_data| button_data.value()).unwrap_or(0.0)) != 0.0 {
                    println!("forward {}", gamepad.button_data(Button::RightTrigger2).map(|button_data| button_data.value()).unwrap_or(0.0));
                } else if (gamepad.button_data(Button::LeftTrigger2).map(|button_data| button_data.value()).unwrap_or(0.0)) != 0.0 {
                    println!("backward {}", gamepad.button_data(Button::LeftTrigger2).map(|button_data| button_data.value()).unwrap_or(0.0) );
                }

                if (gamepad.button_data(Button::RightTrigger).map(|button_data| button_data.value()).unwrap_or(0.0)) != 0.0{
                    return Ok(());
                }
            }

        }
    }
}

fn main() {
    if let Err(e) = run_machine() {
        eprintln!("Machine run failed: {}", e);
        std::process::exit(1);
    }
}
