use std::time::Duration;

use chrono::Timelike;
use tokio::time::sleep;

pub fn init_cron() {
    tokio::spawn(async {
        loop {
            let now = chrono::Local::now();
            if now.second() == 0 {
                println!("Running cron job every minute");
            }

            // https://en.wikipedia.org/wiki/Busy_waiting
            sleep(Duration::from_secs(1)).await;
        }
    });
}
