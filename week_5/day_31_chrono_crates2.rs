use chrono::Utc;
use chrono_tz::Africa::Lagos;

fn main() {
    let utc_now = Utc::now();

    // Convert UTC to Lagos timezone
    let lagos_time = utc_now.with_timezone(&Lagos);

    println!("UTC time: {}", utc_now);
    println!("Lagos time: {}", lagos_time);
}
/* Output:
UTC time: 2024-12-05 13:50:20.123456 UTC
Lagos time: 2024-12-05 14:50:20.123456 WAT*/
