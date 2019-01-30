#[macro_export]
macro_rules! step_help {
    ( $( $msg:expr ),+ ) => {
        {
            use colored::*;
            let flag = "==>".green();
            $(
                println!("{} {}", flag, $msg.to_string().green());
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
