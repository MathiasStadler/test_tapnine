use log::{debug, error, info, log_enabled, Level};

pub fn main() {
    env_logger::init();

    debug!("this is a debug {}", "message");
    error!("this is printed by default");

    if log_enabled!(Level::Info) {
        let x = 3 * 4; // expensive computation
        info!("the answer was: {}", x);
    }
    println!("Hello, world!");

    let x = true;
    if x == true {
        info!("x is true");
    } else {
        info!("x is false");
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::{test, TestResult};
    use super::*;

    #[test]
    fn test_main() {
        // Mock the log output
        let mut log_output = String::new();
        let mut guard = std::io::stdout().lock();
        guard.set_write_function(|_| {
            let bytes = guard
                .into_inner()
                .write_all(&log_output.as_bytes())
                .unwrap();
            Ok(bytes)
        });

        // Run the main function
        main();

        // Check the log output
        assert_eq!(
            log_output,
            "this is a debug message\nthis is printed by default\nHello, world!\n"
        );
    }
}

// test logger
// https://docs.rs/env_logger/latest/env_logger/
