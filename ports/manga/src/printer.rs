#[macro_export]
macro_rules! step_help {
    ( $( $msg:expr ),+ ) => {
        {
            use colored::*;
            let flag = "==>".bright_blue();
            $(
                println!("{} {}", flag, $msg.to_string().bright_blue());
                print!("{} ", flag);
            )+
            std::io::stdout().flush()
        }
    }
}

#[macro_export]
macro_rules! print_err {
    ( $( $e:expr ),+ ) => {
        {
            use colored::*;
            $(
                eprintln!("{}", $e.to_string().red());
            )+
        }
    }
}

#[macro_export]
macro_rules! exit_err {
    ( $( $e:expr ),+ ) => {
        {
            use std::process;
            use manga::print_err;
            $(
                print_err!($e);
                process::exit(233);
            )+
        }
    }
}

#[macro_export]
macro_rules! no_styled {
    ( $( $no:expr ),+) => {
        {
            use colored::*;
            $(
                $no.to_string().cyan()
            )+
        }
    };
}

#[macro_export]
macro_rules! name_styled {
    ( $( $name:expr ),+ ) => {
        {
            use colored::*;
            $(
                $name.bold()
            )+
        }
    };
}
