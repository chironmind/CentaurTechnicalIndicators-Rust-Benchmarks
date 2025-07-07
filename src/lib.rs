use rust_ti::momentum_indicators::bulk as momentum_indicators;
use rust_ti::{ConstantModelType, DeviationModel};

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
