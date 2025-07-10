# RustTI-benchmarks

Benchmarks for [RustTI](https://github.com/0100101001010000/RustTI)

## Purpose

- Fair, reproducible benchmarks for RustTI 
- Use realistic OHLCV data and common indicators.
- Help guide optimization and provide transparency for users.

## Structure

- `benches/`: Criterion benchmarks for each indicator.
- `src/`: Call to each of the indiciators in RustTI

## Running

```sh
cargo bench
```

## Benchmark Results

The following table shows the performance comparison of RustTI functions on 10 years of daily data:

### Momentum Indicators

| Function                                      | Time per Operation |
|-----------------------------------------------|--------------------|
| `relative_strength_index`                     | 573.86 µs          |
| `stochastic_oscillator`                       | 784.13 µs          |
| `slow_stochastic`                             | 28.866 µs          |
| `slowest_stochastic`                          | 28.866 µs          |
| `williams_percent_r`                          | 76.256 µs          |
| `money_flow_index`                            | 150.69 µs          |
| `rate_of_change`                              | 5.3984 µs          |
| `on_balance_volume`                           | 17.405 µs          |
| `commodity_channel_index`                     | 103.19 µs          |
| `mcginley_dynamic_commodity_channel_index`    | 66.044 µs          |
| `macd_line`                                   | 51.482 µs          |
| `mcginley_dynamic_macd_line`                  | 44.461 µs          |
| `chaikin_oscillator`                          | 258.33 µs          |
| `percentage_price_oscillator`                 | 58.060 µs          |
| `chande_momentum_oscillator`                  | 370.14 µs          |

### Candle Indicators

| Function                                      | Time per Operation |
|-----------------------------------------------|--------------------|
| `moving_constant_envelopes`                   | 37.572 µs          |
| `mcginley_dynamic_envelopes`                  | 39.264 µs          |
| `moving_constant_bands`                       | 119.70 µs          |
| `mcginley_dynamic_bands`                      | 43.219 µs          |
| `ichimoku_cloud`                              | 192.93 µs          |
| `donchian_channel`                            | 28.481 µs          |
| `keltner_channel`                             | 318.05 µs          |
| `supertrend`                                  | 148.80 µs          |

### Trend Indicators

| Function                                      | Time per Operation |
|-----------------------------------------------|--------------------|
| `aroon_up`                                    | 16.531 µs          |
| `aroon_down`                                  | 35.058 µs          |
| `aroon_indicator`                             | 72.880 µs          |
| `parabolic_time_price_system`                 | 43.939 µs          |
| `directional_movement_system`                 | 88.965 µs          |
| `volume_price_trend`                          | 6.2801 µs          |
| `true_strength_indx`                          | 705.25 µs          |

### Strength Indicators

| Function                                      | Time per Operation |
|-----------------------------------------------|--------------------|
| `accumulation_distribution`                   | 8.2935 µs          |
| `positive_volume_index`                       | 7.6977 µs          |
| `negative_volume_index`                       | 7.6167 µs          |
| `relative_vigor_index`                        | 505.34 µs          |

### Other Indicators

| Function                                      | Time per Operation |
|-----------------------------------------------|--------------------|
| `return_on_investment`                        | 40.962 µs          |
| `true_range`                                  | 3.4663 µs          |
| `average_true_range`                          | 122.08 µs          |
| `internal_bar_strength`                       | 5.3943 µs          |
| `positivity_indicator`                        | 20.683 µs          |

### Basic Indicators

| Function                                      | Time per Operation |
|-----------------------------------------------|--------------------|
| `mean`                                        | 5.7432 µs          |
| `median`                                      | 333.68 µs          |
| `mode`                                        | 931.09 µs          |
| `log`                                         | 20.335 µs          |
| `log_difference`                              | 42.223 µs          |
| `variance`                                    | 20.921 µs          |
| `standard_deviation`                          | 24.095 µs          |
| `absolute_deviation(Mean)`                    | 26.991 µs          |
| `absolute_deviation(Median)`                  | 345.14 µs          |
| `absoluite_deviation(Mode)`                   | 956.83 µs          |

### Chart Trends

| Function                                      | Time per Operation |
|-----------------------------------------------|--------------------|
| `peaks`                                       | 93.094 µs          |
| `valleys`                                     | 190.89 µs          |
| `peak_trend`                                  | 188.14 µs          |
| `valley_trend`                                | 379.52 µs          |
| `overall_trend`                               | 10.337 µs          |
| `break_down_trends`                           | 14.655 ms          |

## Contributing

- PRs welcome for new indicators, datasets, or improvement suggestions!
