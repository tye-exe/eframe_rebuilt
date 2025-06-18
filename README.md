This is a stripped down re-implementation of [eframe](https://github.com/emilk/egui/tree/main/crates/eframe) in the hopes of trying to solve [this issue](https://github.com/emilk/egui/issues/6757).

To run example app:
```
cargo run
```

To build eframe re-implementation
```
cargo build -F glow
```

### Comparing main thread to spawned thread
To generate the log outputs for eframe on the main thread and eframe on a spawned thread you can run:
```bash
bash compare.sh
```
The logs files `out_main` & `out_spawn` will be generated with the outputs for the main thread & spawned thread respectively.
The difference between the two can be analysed with a tool such as `diff`.
