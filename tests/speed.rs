#[test]
fn speed() {
    let mut sum = 0;
    let num_tests = 10000000;

    let tb = chrono::Utc::now();

    for _ in 0..num_tests {
        sum += test();
    }

    let td = chrono::Utc::now().signed_duration_since(tb);

    let td_m = td.num_milliseconds();
    let td_s = td.num_seconds();

    let avg_ns = sum / num_tests;
    let avg_ms = avg_ns / num_tests;

    println!("The validation took an average of {avg_ns} nanoseconds ({avg_ms} miliseconds)");
    println!("Validating 10,000,000 imei codes took {td_m} miliseconds ({td_s} seconds)");
}

fn test() -> i64 {
    let time_before = chrono::Utc::now();

    assert!(imei::valid("490154203237518"));

    // pass "--nocapture" to show time difference
    let difference = chrono::Utc::now()
        .signed_duration_since(time_before)
        .num_nanoseconds()
        .unwrap();

    difference
}

#[test]
#[cfg(feature = "serde")]
fn test_serde() {
    use imei::Imei;
    use serde::Deserialize;

    #[derive(serde::Serialize, Deserialize)]
    struct ContainsImei {
        imei: Imei<String>,
    }

    let json = serde_json::to_string(&Imei::try_new("354406185514933").unwrap()).unwrap();
    assert_eq!(json, "\"354406185514933\"");
    let imei: Imei<String> = serde_json::from_str("\"354406185514933\"").unwrap();
    assert_eq!(imei, Imei::try_new("354406185514933".to_string()).unwrap())
}
