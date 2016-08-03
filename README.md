## Crawler-rust

Crawler allows you to collect the news data from __NewYorkTimes__ and __WashingtonTimes__.

## How to Use

Just run

```
cargo build
```

you will get two executable files `main_wt`(for WashingtonTimes) and `main_nt`(for NewYorkTimes). Then just run:

```rust
./main_wt
```

You will get the title, summary, address and text for each news stories.

It is also possible to specify the page to be crawled:

```rust
./main_wt YourAddressStartWithHTTP SleepTimeBetweenDownloadEachStory
```
