#[test]
fn parse_basic_wind() {
    let wind = ::parse_wind("3127", 3000u32).unwrap();

    assert_eq!(wind.speed, 27u32);
    assert_eq!(wind.direction, 310u32);
}

#[test]
fn parse_bad_wind() {
    let wind = ::parse_wind("invalid", 3000u32);
    let parsed = match wind {
        Some(_) => true,
        None => false,
    };

    assert_eq!(parsed, false);
}

#[test]
fn parse_wind_with_negative_temp() {
    let wind = ::parse_wind("3127-32", 3000u32).unwrap();

    assert_eq!(wind.speed, 27u32);
    assert_eq!(wind.direction, 310u32);
}

#[test]
fn parse_wind_with_positive_temp() {
    let wind = ::parse_wind("3127+32", 3000u32).unwrap();

    assert_eq!(wind.speed, 27u32);
    assert_eq!(wind.direction, 310u32);
}

#[test]
fn need_update_0_12() {
    let forecast = ::WindsAloftForecast {
        forecast_time: 0000u32,
        time_retrieved: 0000u32,
        forecasts: Vec::new(),
    };

    assert!(forecast.needs_refresh_time_given(1200u32));
    assert!(forecast.needs_refresh_time_given(1250u32));
}

#[test]
fn need_update_12_12() {
    let forecast = ::WindsAloftForecast {
        forecast_time: 1200u32,
        time_retrieved: 1200u32,
        forecasts: Vec::new(),
    };

    assert!(!forecast.needs_refresh_time_given(1200u32));
    assert!(!forecast.needs_refresh_time_given(1250u32));
}

#[test]
fn need_update_12_10() {
    let forecast = ::WindsAloftForecast {
        forecast_time: 1200u32,
        time_retrieved: 1200u32,
        forecasts: Vec::new(),
    };

    assert!(forecast.needs_refresh_time_given(1000u32));
    assert!(forecast.needs_refresh_time_given(1050u32));
}

#[test]
fn need_update_12_0() {
    let forecast = ::WindsAloftForecast {
        forecast_time: 1200u32,
        time_retrieved: 1200u32,
        forecasts: Vec::new(),
    };

    assert!(forecast.needs_refresh_time_given(0000u32));
    assert!(forecast.needs_refresh_time_given(0050u32));
}

#[test]
fn need_update_18_0() {
    let forecast = ::WindsAloftForecast {
        forecast_time: 1800u32,
        time_retrieved: 1800u32,
        forecasts: Vec::new(),
    };

    assert!(forecast.needs_refresh_time_given(0000u32));
    assert!(forecast.needs_refresh_time_given(0050u32));
}

#[test]
fn test_parse_12_time_from_body() {
    let body = "
        000
        FBUS31 KWNO 061406
        FD1US1
        DATA BASED ON 061200Z
        VALID 061800Z   FOR USE 1400-2100Z. TEMPS NEG ABV 24000

        FT  3000    6000    9000   12000   18000   24000  30000  34000  39000
    ";

    assert_eq!(::WindsAloftForecast::parse_time_from_body(body), 1200u32);
}

#[test]
fn test_parse_1432_time_from_body() {
    let body = "
        000
        FBUS31 KWNO 061406
        FD1US1
        DATA BASED ON 061432Z
        VALID 061800Z   FOR USE 1400-2100Z. TEMPS NEG ABV 24000

        FT  3000    6000    9000   12000   18000   24000  30000  34000  39000
    ";

    assert_eq!(::WindsAloftForecast::parse_time_from_body(body), 1432u32);
}

#[test]
fn test_immediate_need_refresh() {
    let forecast = ::WindsAloftForecast::new();
    assert!(!forecast.needs_refresh());
}
