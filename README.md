# RustTI-benchmarks

Welcome to **RustTI-benchmarks**!  
This project provides fair, reproducible benchmarks for [RustTI](https://github.com/0100101001010000/RustTI), a high-performance Rust library for technical indicators.

## 🚀 Why Use This Repo?

- **See how fast RustTI runs** on your machine with realistic OHLCV data and common indicators.
- **Check for regressions** after making changes to RustTI.
- **Guide optimization** efforts and give users transparency about performance.

---

## 🗂️ Repo Structure

- **`benches/`** – Benchmarks for each indicator, powered by [Criterion](https://bheisler.github.io/criterion.rs/book/index.html).
- **`src/`** – Calls to each indicator in RustTI for benchmarking.

---

## 🏃 Getting Started

### 1. Benchmark RustTI on Your Machine

Curious how RustTI performs for your use case?  
Run the benchmarks and find out!

**To benchmark all indicators:**
```sh
cargo bench
```

**To benchmark a specific indicator:**
```sh
cargo bench --bench rsi
```

> **Tip:**  
> Indicators are highly configurable!  
> If you want to benchmark with your own configuration, edit the relevant code in `lib.rs`.  
> Different settings can make a huge difference in speed.

_For example, the `commodity_channel_index` with a simple moving average and standard deviation runs in **103.19 µs**.  
Switching to an exponential moving average and mode absolute deviation jumps to **2.3684 ms** (still fast but over 20x slower!)._

---

### 2. Check for Performance Regressions

If you've contributed changes to RustTI, it's important to make sure you haven't accidentally slowed anything down.

**Step 1:**  
Run the benchmarks before your changes to get a baseline.

```sh
cargo bench
```

**Step 2:**  
Point your `Cargo.toml` to your local RustTI for testing:
```toml
rust_ti = { path = "../rustti" }
# rust_ti = "2.x.x"
```

**Step 3:**  
Run the benchmarks again and compare the results.

---

## 📊 Benchmark Results

Benchmarks were run on a **Raspberry Pi 5 (8GB RAM)** using 10 years of daily data.  
Why a Raspberry Pi? It’s a familiar, modest baseline, your machine is probably much faster!  
For more details, see the [Raspberry Pi 5 benchmarks](https://www.raspberrypi.com/news/benchmarking-raspberry-pi-5/).

Benchmarks use [Criterion.rs](https://bheisler.github.io/criterion.rs/book/index.html).

---

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
| `aroon_down`                                  | 16.592 µs          |
| `aroon_indicator`                             | 66.468 µs          |
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
| `valleys`                                     | 92.119 µs          |
| `peak_trend`                                  | 188.14 µs          |
| `valley_trend`                                | 188.81 µs          |
| `overall_trend`                               | 10.337 µs          |
| `break_down_trends`                           | 14.655 ms          |

### Correlation Indicators

| Function                                      | Time per Operation |
|-----------------------------------------------|--------------------|
| `correlate_asset_prices`                      | 231.14 µs          |

### Moving Average

| Function                                      | Time per Operation |
|-----------------------------------------------|--------------------|
| `moving_average(Simple)`                      | 17.575 µs          |
| `moving_average(Smoothed)`                    | 76.601 µs          |
| `moving_average(Exponential)`                 | 78.505 µs          |
| `mcginley_dynamic`                            | 39.653 µs          |

### Volatility Indicators

| Function                                      | Time per Operation |
|-----------------------------------------------|--------------------|
| `ulcer_index`                                 | 65.959 µs          |
| `volatility_system`                           | 137.25 µs          |

---

## 🤝 Contributing

We welcome all contributions!  
- Add new indicators, benchmarks, or datasets.
- Suggest improvements or open issues.
- Open a PR—your help makes RustTI better for everyone!

---

**Thank you for checking out RustTI-benchmarks!**  
Feel free to open an issue or discussion if you have questions or ideas.

