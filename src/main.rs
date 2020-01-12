extern crate clap;
use clap::{App, AppSettings, Arg, SubCommand};

use rppal::gpio::Gpio;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Raspberry Peripherals Access Program")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .subcommand(
            SubCommand::with_name("gpio")
                .about("Control GPIO peripherals")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .arg(
                    Arg::with_name("PIN")
                        .help("Set the pin used, should be inside the range [0, 27].")
                        .required(true),
                )
                .subcommand(
                    SubCommand::with_name("output")
                        .about("Set pin as output.")
                        .arg(
                            Arg::with_name("state")
                                .help("Set state of pin.")
                                .possible_values(&vec!["low", "high"])
                                .required(true),
                        ),
                )
                .subcommand(SubCommand::with_name("INPUT").about("Print state of pin.")),
        )
        .get_matches();

    match matches.subcommand() {
        ("gpio", Some(gpio_subcommand)) => {
            let pin_number = gpio_subcommand
                .value_of("PIN")
                .unwrap()
                .parse::<u8>()
                .unwrap_or_else(|_| panic!("Expected PIN number between 0 and 27."));

            let gpio = match Gpio::new().unwrap().get(pin_number) {
                Result::Ok(val) => val,
                Result::Err(err) => panic!("Error: {}", err),
            };

            match gpio_subcommand.subcommand() {
                ("INPUT", Some(_)) => {
                    let pin = gpio.into_input();
                    println!("Pin {}:\t{}", pin.pin(), pin.read());
                }

                ("OUTPUT", Some(output_subcommand)) => {
                    let mut pin = gpio.into_output();
                    // Avoid reset state after finishing
                    pin.set_reset_on_drop(false);

                    let _ = match output_subcommand.value_of("state") {
                        Some("high") => pin.set_high(),
                        Some("low") => pin.set_low(),
                        _ => unreachable!(),
                    };
                }

                _ => unreachable!(),
            };
        }
        _ => unreachable!(),
    };
}
