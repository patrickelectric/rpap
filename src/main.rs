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
                    SubCommand::with_name("OUTPUT")
                        .about("Set pin as output.")
                        .arg(
                            Arg::with_name("state")
                                .help("Set state of pin.")
                                .possible_values(&vec!["LOW", "HIGH"])
                                .required(true),
                        ),
                )
                .subcommand(SubCommand::with_name("INPUT").about("Print state of pin.")),
        )
        .get_matches();

    if matches.is_present("gpio") {
        let gpio_matches = matches.subcommand_matches("gpio").unwrap();
        let pin_number = gpio_matches
            .value_of("PIN")
            .unwrap()
            .parse::<u8>()
            .unwrap_or_else(|_| panic!("Expected PIN number between 0 and 27."));
        let gpio = Gpio::new().unwrap().get(pin_number);
        let gpio_unwraped = match gpio {
            Result::Ok(val) => val,
            Result::Err(err) => panic!("Error: {}", err),
        };
        let mut pin = gpio_unwraped.into_output();
        pin.set_reset_on_drop(false);
        if gpio_matches
            .subcommand_matches("OUTPUT")
            .unwrap()
            .value_of("state")
            .unwrap()
            == "HIGH"
        {
            pin.set_high();
        } else {
            pin.set_low();
        }
        return;
    }
}
