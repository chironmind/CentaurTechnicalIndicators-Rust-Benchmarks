use rust_ti::basic_indicators::bulk as basic_indicators;
use rust_ti::candle_indicators::bulk as candle_indicators;
use rust_ti::chart_trends;
use rust_ti::momentum_indicators::bulk as momentum_indicators;
use rust_ti::other_indicators::bulk as other_indicators;
use rust_ti::strength_indicators::bulk as strength_indicators;
use rust_ti::trend_indicators::bulk as trend_indicators;
use rust_ti::{CentralPoint, ConstantModelType, DeviationModel, Position};

mod data_constants;

// Momentum indicators

pub fn compute_rsi() -> Vec<f64> {
    momentum_indicators::relative_strength_index(
        &data_constants::PRICES,
        ConstantModelType::SmoothedMovingAverage,
        14,
    )
}

pub fn compute_so() -> Vec<f64> {
    momentum_indicators::stochastic_oscillator(&data_constants::PRICES, 14)
}

pub fn compute_slow_so() -> Vec<f64> {
    momentum_indicators::slow_stochastic(
        &data_constants::PRICES,
        ConstantModelType::SimpleMovingAverage,
        14,
    )
}

pub fn compute_slowest_so() -> Vec<f64> {
    momentum_indicators::slowest_stochastic(
        &data_constants::PRICES,
        ConstantModelType::SimpleMovingAverage,
        14,
    )
}

pub fn compute_williams_r() -> Vec<f64> {
    momentum_indicators::williams_percent_r(
        &data_constants::HIGH,
        &data_constants::LOW,
        &data_constants::CLOSE,
        14,
    )
}

pub fn compute_mfi() -> Vec<f64> {
    momentum_indicators::money_flow_index(&data_constants::PRICES, &data_constants::VOLUME, 14)
}

pub fn compute_roc() -> Vec<f64> {
    momentum_indicators::rate_of_change(&data_constants::PRICES)
}

pub fn compute_obv() -> Vec<f64> {
    momentum_indicators::on_balance_volume(&data_constants::PRICES, &data_constants::VOLUME, 0.0)
}

pub fn compute_cci() -> Vec<f64> {
    momentum_indicators::commodity_channel_index(
        &data_constants::PRICES,
        ConstantModelType::SimpleMovingAverage,
        DeviationModel::StandardDeviation,
        0.015,
        14,
    )
}

pub fn compute_mg_cci() -> Vec<(f64, f64)> {
    momentum_indicators::mcginley_dynamic_commodity_channel_index(
        &data_constants::PRICES,
        0.0,
        DeviationModel::MeanAbsoluteDeviation,
        0.015,
        14,
    )
}

pub fn compute_macd() -> Vec<f64> {
    momentum_indicators::macd_line(
        &data_constants::PRICES,
        7,
        ConstantModelType::SimpleMovingAverage,
        14,
        ConstantModelType::SimpleMovingAverage,
    )
}

pub fn compute_mg_macd() -> Vec<(f64, f64, f64)> {
    momentum_indicators::mcginley_dynamic_macd_line(&data_constants::PRICES, 7, 0.0, 14, 0.0)
}

pub fn compute_co() -> Vec<(f64, f64)> {
    momentum_indicators::chaikin_oscillator(
        &data_constants::HIGH,
        &data_constants::LOW,
        &data_constants::CLOSE,
        &data_constants::VOLUME,
        7,
        14,
        0.0,
        ConstantModelType::SimpleMovingAverage,
        ConstantModelType::SimpleMovingAverage,
    )
}

pub fn compute_ppo() -> Vec<f64> {
    momentum_indicators::percentage_price_oscillator(
        &data_constants::PRICES,
        7,
        14,
        ConstantModelType::SimpleMovingAverage,
    )
}

pub fn compute_cmo() -> Vec<f64> {
    momentum_indicators::chande_momentum_oscillator(&data_constants::PRICES, 14)
}

// Candle Indicators

pub fn compute_cnst_env() -> Vec<(f64, f64, f64)> {
    candle_indicators::moving_constant_envelopes(
        &data_constants::PRICES,
        ConstantModelType::SimpleMovingAverage,
        3.0,
        10,
    )
}

pub fn compute_mg_env() -> Vec<(f64, f64, f64)> {
    candle_indicators::mcginley_dynamic_envelopes(&data_constants::PRICES, 3.0, 0.0, 10)
}

pub fn compute_cnst_bands() -> Vec<(f64, f64, f64)> {
    candle_indicators::moving_constant_bands(
        &data_constants::PRICES,
        ConstantModelType::SimpleMovingAverage,
        DeviationModel::StandardDeviation,
        2.0,
        5,
    )
}

pub fn compute_mg_bands() -> Vec<(f64, f64, f64)> {
    candle_indicators::mcginley_dynamic_bands(
        &data_constants::PRICES,
        DeviationModel::StandardDeviation,
        2.0,
        0.0,
        5,
    )
}

pub fn compute_icloud() -> Vec<(f64, f64, f64, f64, f64)> {
    candle_indicators::ichimoku_cloud(
        &data_constants::HIGH,
        &data_constants::LOW,
        &data_constants::CLOSE,
        5,
        10,
        15,
    )
}

pub fn compute_donchian_chnl() -> Vec<(f64, f64, f64)> {
    candle_indicators::donchian_channels(&data_constants::HIGH, &data_constants::LOW, 5)
}

pub fn compute_keltner_chnl() -> Vec<(f64, f64, f64)> {
    candle_indicators::keltner_channel(
        &data_constants::HIGH,
        &data_constants::LOW,
        &data_constants::CLOSE,
        ConstantModelType::ExponentialMovingAverage,
        ConstantModelType::SimpleMovingAverage,
        2.0,
        5,
    )
}

pub fn compute_supertrend() -> Vec<f64> {
    candle_indicators::supertrend(
        &data_constants::HIGH,
        &data_constants::LOW,
        &data_constants::CLOSE,
        ConstantModelType::SimpleMovingAverage,
        2.0,
        5,
    )
}

// Trend Indicators

pub fn compute_aroon_up() -> Vec<f64> {
    trend_indicators::aroon_up(&data_constants::HIGH, 5)
}

pub fn compute_aroon_down() -> Vec<f64> {
    trend_indicators::aroon_down(&data_constants::LOW, 5)
}

pub fn compute_aroon_indicator() -> Vec<(f64, f64, f64)> {
    trend_indicators::aroon_indicator(&data_constants::HIGH, &data_constants::LOW, 5)
}

pub fn compute_parabolic_tps() -> Vec<f64> {
    trend_indicators::parabolic_time_price_system(
        &data_constants::HIGH,
        &data_constants::LOW,
        0.02,
        0.2,
        0.02,
        Position::Long,
        0.0,
    )
}

pub fn compute_dir_mov_sys() -> Vec<(f64, f64, f64, f64)> {
    trend_indicators::directional_movement_system(
        &data_constants::HIGH,
        &data_constants::LOW,
        &data_constants::CLOSE,
        5,
        ConstantModelType::SimpleMovingAverage,
    )
}

pub fn compute_vpt() -> Vec<f64> {
    let reduced_volume: Vec<f64> = (0..2551).map(|i| data_constants::VOLUME[i]).collect();
    trend_indicators::volume_price_trend(&data_constants::PRICES, &reduced_volume, 0.0)
}

pub fn compute_tsi() -> Vec<f64> {
    trend_indicators::true_strength_index(
        &data_constants::PRICES,
        ConstantModelType::SimpleMovingAverage,
        10,
        ConstantModelType::SimpleMovingAverage,
        5,
    )
}

// Strength Indicators

pub fn compute_ad() -> Vec<f64> {
    strength_indicators::accumulation_distribution(
        &data_constants::HIGH,
        &data_constants::LOW,
        &data_constants::CLOSE,
        &data_constants::VOLUME,
        0.0,
    )
}

pub fn compute_pvi() -> Vec<f64> {
    strength_indicators::positive_volume_index(&data_constants::CLOSE, &data_constants::VOLUME, 0.0)
}

pub fn compute_nvi() -> Vec<f64> {
    strength_indicators::negative_volume_index(&data_constants::CLOSE, &data_constants::VOLUME, 0.0)
}

pub fn compute_rvi() -> Vec<f64> {
    strength_indicators::relative_vigor_index(
        &data_constants::OPEN,
        &data_constants::HIGH,
        &data_constants::LOW,
        &data_constants::CLOSE,
        ConstantModelType::SimpleMovingAverage,
        10,
    )
}

// Other Indicators

pub fn compute_roi() -> Vec<(f64, f64)> {
    other_indicators::return_on_investment(&data_constants::PRICES, 1000.0)
}

pub fn compute_tr() -> Vec<f64> {
    other_indicators::true_range(
        &data_constants::CLOSE,
        &data_constants::HIGH,
        &data_constants::LOW,
    )
}

pub fn compute_atr() -> Vec<f64> {
    other_indicators::average_true_range(
        &data_constants::CLOSE,
        &data_constants::HIGH,
        &data_constants::LOW,
        ConstantModelType::SimpleMovingAverage,
        5,
    )
}

pub fn compute_ibs() -> Vec<f64> {
    other_indicators::internal_bar_strength(
        &data_constants::HIGH,
        &data_constants::LOW,
        &data_constants::CLOSE,
    )
}

pub fn compute_pi() -> Vec<(f64, f64)> {
    other_indicators::positivity_indicator(
        &data_constants::OPEN,
        &data_constants::CLOSE,
        5,
        ConstantModelType::SimpleMovingAverage,
    )
}

// Basic Indicators

pub fn compute_mean() -> Vec<f64> {
    basic_indicators::mean(&data_constants::PRICES, 5)
}

pub fn compute_median() -> Vec<f64> {
    basic_indicators::median(&data_constants::PRICES, 5)
}

pub fn compute_mode() -> Vec<f64> {
    basic_indicators::mode(&data_constants::PRICES, 5)
}

pub fn compute_log() -> Vec<f64> {
    basic_indicators::log(&data_constants::PRICES)
}

pub fn compute_log_diff() -> Vec<f64> {
    basic_indicators::log_difference(&data_constants::PRICES)
}

pub fn compute_var() -> Vec<f64> {
    basic_indicators::variance(&data_constants::PRICES, 5)
}

pub fn compute_stdev() -> Vec<f64> {
    basic_indicators::standard_deviation(&data_constants::PRICES, 5)
}

pub fn compute_mean_abs_dev() -> Vec<f64> {
    basic_indicators::absolute_deviation(&data_constants::PRICES, 5, CentralPoint::Mean)
}

pub fn compute_median_abs_dev() -> Vec<f64> {
    basic_indicators::absolute_deviation(&data_constants::PRICES, 5, CentralPoint::Median)
}

pub fn compute_mode_abs_dev() -> Vec<f64> {
    basic_indicators::absolute_deviation(&data_constants::PRICES, 5, CentralPoint::Mode)
}

// Chart trends

pub fn compute_peaks() -> Vec<(f64, usize)> {
    chart_trends::peaks(&data_constants::PRICES, 5, 2)
}

pub fn compute_valleys() -> Vec<(f64, usize)> {
    chart_trends::valleys(&data_constants::PRICES, 5, 2)
}

pub fn compute_peak_trend() -> (f64, f64) {
    chart_trends::peak_trend(&data_constants::PRICES, 5)
}

pub fn compute_valley_trend() -> (f64, f64) {
    chart_trends::valley_trend(&data_constants::PRICES, 5)
}

pub fn compute_overall_trend() -> (f64, f64) {
    chart_trends::overall_trend(&data_constants::PRICES)
}

pub fn compute_break_down_trends() -> Vec<(usize, usize, f64, f64)> {
    chart_trends::break_down_trends(
        &data_constants::PRICES,
        1,
        0.75,
        1.0,
        0.5,
        1.5,
        2.0,
        3.0,
        2.0,
        3.0,
    )
}
