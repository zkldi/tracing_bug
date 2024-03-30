# tracing_subscriber bug

Seems to be a bad interaction with `tracing-tracy` and `eframe` or `egui`.

This is the most minimal reproduction I could get for the bug.

It's triggered by doing `cargo run` and then closing the app. You will get a panic a-la:

```
thread 'main' panicked at /rustc/e7bbe8ce933123a8e327c79e38bcb790595e6a65/library/std/src/thread/local.rs:262:26:
cannot access a Thread Local Storage value during or after destruction: AccessError
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at /home/zkldi/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sharded-slab-0.1.7/src/shard.rs:295:9:
Thread count overflowed the configured max count. Thread index = 18446744073709551615, max threads = 4096.
stack backtrace:
   0:     0x56465c955386 - std::backtrace_rs::backtrace::libunwind::trace::h514ead2753e1782f
                               at
```

and so on

## Trigger

This bug only occurs when `tracing-tracy` is a subscriber to tracing, i.e.
```rs
    let subscriber = tracing_subscriber::registry()
        .with(stdout)
        // VVVV this is the line that causes the panic!!
        // commenting this out will result in a working app.
        .with(tracing_tracy::TracyLayer::default());
```

I thought the bug might be something to do with having two subscribers to the tracing_subscriber registry, but no issue seems to occur if you have two stdout subscribers.

I suspect this is a weird poor interaction between
- eframe
- tracing_tracy
- tracing_subscriber

manifesting finally in a panic inside `sharded_slab`, for some reason.

## Fix

No idea. If you're having this issue, I'm having it too. Turn off tracing-tracy, I guess.