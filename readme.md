<h1 align="center">Quotick by SIGDEV</h1>

<h5 align="center">Embedded tick market data (trade, quote, ..) database storage optimized for billions of data points.</h5>

<div align="center">
  <a href="https://crates.io/crates/quotick">
    crates.io
  </a>
  —
  <a href="https://github.com/sigset/quotick">
    Github
  </a>
</div>

<br />

```shell script
$ cargo add quotick
```

#### Usage

```rust
use serde_derive::{Deserialize, Serialize};

use quotick::quotick::Quotick;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct Trade {
    size: u32,
    price: u32,
}

impl quotick::tick::Tick for Trade {
    fn epoch(&self) -> u64 {
        // one day
        self.time / 86_400_000_000_000
    }
}

fn main() {
    let trade1 =
        (
            10, // time
            Trade {
                size: 1,
                price: 2,
            },
        );

    let trade2 =
        (
            11, // time
            Trade {
                size: 2,
                price: 3,
            },
        );

    let trade3 =
        (
            12,
            Trade {
                size: 3,
                price: 4,
            },
        );

    let quotick =
        Quotick::<Trade>::new(
            "SYMBL",
            "./db",
        );

    qt.insert(&Frame::new(trade1.0, trade1.1));
    qt.insert(&Frame::new(trade2.0, trade2.1));
    qt.insert(&Frame::new(trade3.0, trade3.1));

    qt.persist();
    
    // iterate over all epochs
    quotick
        .epochs()
        .for_each(
            |mut epoch| {
                // iterate over all frames
                // in a given epoch

                epoch
                    .frames()
                    .for_each(
                        |frame| {
                            frame.time(); // u64 time
                            frame.tick(); // your tick
                        },
                    );
            }
        );

    // obtain the frame with the lowest
    // time value of the first epoch (10)
    dbg!(quotick.oldest_frame());

    // obtain the frame with the highest
    // time value of the last epoch (12)
    dbg!(quotick.newest_frame());
}
```

#### Architecture

Quotick can contain an unlimited amount of symbols. One internal database is used per symbol, and each symbol is stored in a separate directory.

Ticks are separated by _epochs_. Epochs are used to separate and speed up lookups of ticks contained within a single window of time (i.e. one day).

The epoch index is a radix-trie and stored inside a file identified by `epochs.qti`.

When lookup up an epoch, a tick or inserting a tick, the radix trie is loaded into memory in full. It stays in memory until quotick goes out of scope and is dropped, or as long as the program is running.

When an epoch is located inside the epoch index, and if not, it is added to the index, the epochs' tick-index is loaded from `frameset/[epoch].qti`, and if it does not exist, it is initialized. It is a radix-trie and contains all ticks identified by their nano-second precision timestamp.

Timestamps must be in nano-second precision. Quotick is not designed to store ticks identified by arbitrary identifiers, and relies on the fact that ticks' timestamps must be sortable.

Tick data is stored in a file loaded from `frameset/[epoch].qtf`, called a frameset. Internally, every tick represents a frame.

When a tick is inserted into an epoch, it is appended to the end of the file and the offset of the tick is stored as a (u64 timestamp, u64 offset) tuple inside the tick-index radix-trie of the respective epoch.

When iterating over the tick-index of an epoch, returned ticks are loaded from the frameset file on-demand. The frameset will seek to the desired offset of the backing file, read the respective amount of bytes and attempt to deserialize them into a frame.

If you insert ticks in random order, you must either defragment an epoch to prevent significant read-head jumps on HDDs. It is absolutely recommended to use NVMe storage for Quotick.

#### Notes

Ticks stored inside Quotick must implement `quotick::tick::Tick` which depends on Default, Debug, Deserialize and Serialize.

Note that `Default` is required to be able to determine the size of the structure as serialized by `bincode`. It differs from `std::mem::size_of<T>()` and is therefore mandatory.

`Deserialize` and `Serialize` are required to be able to write and read ticks from file.

#### License

<b>If your organization revenue exceeds $1 million (or currency equivalent) you must obtain a usage license. Please contact [licensing@sig.dev](mainlto:licensing@sig.dev).</b>

The following license applies 

~~ Usage License ~~

Copyright (c) 2021 SIGDEV LLC
Copyright (c) 2021 Kenan Sulayman

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, and/or sell copies of the
Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

This license does not apply if the gross revenue of the organization or
individual, or group of individuals, if this software is directly or
indirectly benefiting inidividuals other than the original user, exceeds
$1 million (or currency equivalent) per year.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
