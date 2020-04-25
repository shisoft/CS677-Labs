## Building Backend 

```
➜ sudo docker build --no-cache -t shisoft/pygmy-backend . 
[sudo] password for shisoft: 
Sending build context to Docker daemon  164.4kB
Step 1/15 : FROM rust:1 as builder
 ---> 3055be0d06f0
Step 2/15 : RUN apt install libsqlite3-dev -y
 ---> Running in c51434aba030

WARNING: apt does not have a stable CLI interface. Use with caution in scripts.

Reading package lists...
Building dependency tree...
Reading state information...
libsqlite3-dev is already the newest version (3.27.2-3).
0 upgraded, 0 newly installed, 0 to remove and 0 not upgraded.
Removing intermediate container c51434aba030
 ---> a65af89e578b
Step 3/15 : RUN rustup default nightly
 ---> Running in 99b8666fa8da
info: syncing channel updates for 'nightly-x86_64-unknown-linux-gnu'
info: latest update on 2020-04-25, rust version 1.44.0-nightly (3360cc3a0 2020-04-24)
info: downloading component 'cargo'
info: downloading component 'rust-std'
info: downloading component 'rustc'
info: installing component 'cargo'
info: installing component 'rust-std'
info: installing component 'rustc'

info: default toolchain set to 'nightly-x86_64-unknown-linux-gnu'
  nightly-x86_64-unknown-linux-gnu installed - rustc 1.44.0-nightly (3360cc3a0 2020-04-24)

Removing intermediate container 99b8666fa8da
 ---> 57301cb9fb30
Step 4/15 : WORKDIR /usr/pygmy-backend
 ---> Running in ff9fd939d2dc
Removing intermediate container ff9fd939d2dc
 ---> 43dc47a2f04d
Step 5/15 : COPY . .
 ---> 48fb9b75db5c
Step 6/15 : RUN cargo build --release
 ---> Running in 8d3b8bde2422
    Updating crates.io index
        Updating crates.io index
    Updating git repository `https://github.com/ShisoftResearch/bifrost.git`
Downloading crates ...
  Downloaded bytes v0.5.4
  Downloaded actix-connect v1.0.2
  Downloaded num-traits v0.2.11
  Downloaded kv-log-macro v1.0.4
  Downloaded regex v0.2.11
  Downloaded miniz_oxide v0.3.6
  Downloaded regex v1.3.6
  Downloaded hyper v0.13.4
  Downloaded crossbeam-channel v0.4.2
  Downloaded backtrace v0.3.46
  Downloaded actix-threadpool v0.3.1
  Downloaded actix-rt v1.0.0
  Downloaded brotli2 v0.3.2
  Downloaded crossbeam-deque v0.7.3
  Downloaded diesel_derives v1.4.1
  Downloaded failure v0.1.7
  Downloaded futures-task v0.3.4
  Downloaded httparse v1.3.4
  Downloaded indexmap v1.3.2
  Downloaded match_cfg v0.1.0
  Downloaded lru-cache v0.1.2
  Downloaded log v0.4.8
  Downloaded num-integer v0.1.42
  Downloaded mime_guess v2.0.3
  Downloaded percent-encoding v2.1.0
  Downloaded quick-error v1.2.3
  Downloaded pin-utils v0.1.0-alpha.4
  Downloaded num_cpus v1.12.0
  Downloaded net2 v0.2.33
  Downloaded maybe-uninit v2.0.0
  Downloaded bytestring v0.1.5
  Downloaded itoa v0.4.5
  Downloaded half v1.5.0
  Downloaded pin-project-internal v0.4.8
  Downloaded quote v1.0.3
  Downloaded rand_chacha v0.2.2
  Downloaded rustc-demangle v0.1.16
  Downloaded rand_core v0.5.1
  Downloaded simple_logger v1.6.0
  Downloaded sha1 v0.6.0
  Downloaded ryu v1.0.3
  Downloaded serde_urlencoded v0.6.1
  Downloaded pin-project-lite v0.1.4
  Downloaded serde_cbor v0.11.1
  Downloaded socket2 v0.3.12
  Downloaded threadpool v1.7.1
  Downloaded try-lock v0.2.2
  Downloaded tokio-util v0.3.1
  Downloaded unicode-bidi v0.3.4
  Downloaded hostname v0.3.1
  Downloaded futures-timer v3.0.2
  Downloaded futures-sink v0.3.4
  Downloaded futures-io v0.3.4
  Downloaded futures-executor v0.3.4
  Downloaded futures-core v0.3.4
  Downloaded fnv v1.0.6
  Downloaded dotenv v0.9.0
  Downloaded utf8-ranges v1.0.4
  Downloaded tokio-util v0.2.0
  Downloaded thread-id v3.3.0
  Downloaded actix-testing v1.0.0
  Downloaded actix-service v1.0.5
  Downloaded adler32 v1.0.4
  Downloaded actix-web-codegen v0.2.1
  Downloaded actix-tls v1.0.0
  Downloaded actix-router v0.2.4
  Downloaded want v0.3.0
  Downloaded actix-macros v0.1.1
  Downloaded atty v0.2.14
  Downloaded autocfg v1.0.0
  Downloaded aho-corasick v0.6.10
  Downloaded async-trait v0.1.29
  Downloaded arc-swap v0.4.5
  Downloaded bitflags v1.2.1
  Downloaded byteorder v1.3.4
  Downloaded awc v1.0.1
  Downloaded copyless v0.1.4
  Downloaded cc v1.0.50
  Downloaded colored v1.9.3
  Downloaded memoffset v0.5.4
  Downloaded actix-utils v1.0.6
  Downloaded pkg-config v0.3.17
  Downloaded parking_lot_core v0.7.0
  Downloaded tokio-macros v0.2.5
  Downloaded unicode-xid v0.2.0
  Downloaded tower-service v0.3.0
  Downloaded version_check v0.9.1
  Downloaded twox-hash v1.5.0
  Downloaded thread_local v1.0.1
  Downloaded actix-codec v0.2.0
  Downloaded unicase v2.6.0
  Downloaded ucd-util v0.1.8
  Downloaded resolv-conf v0.6.3
  Downloaded actix-server v1.0.2
  Downloaded cfg-if v0.1.10
  Downloaded async-task v1.3.1
  Downloaded crc32fast v1.2.0
  Downloaded base64 v0.11.0
  Downloaded either v1.5.3
  Downloaded crossbeam-epoch v0.8.2
  Downloaded failure_derive v0.1.7
  Downloaded derive_more v0.99.5
  Downloaded futures-macro v0.3.4
  Downloaded futures-timer v2.0.2
  Downloaded dtoa v0.4.5
  Downloaded crossbeam-utils v0.7.2
  Downloaded getrandom v0.1.14
  Downloaded fxhash v0.2.1
  Downloaded futures v0.3.4
  Downloaded flate2 v1.0.14
  Downloaded rand v0.7.3
  Downloaded heck v0.3.1
  Downloaded unicode-segmentation v1.6.0
  Downloaded url v2.1.1
  Downloaded aho-corasick v0.7.10
  Downloaded trust-dns-resolver v0.18.0-alpha.2
  Downloaded mio v0.6.21
  Downloaded reqwest v0.10.4
  Downloaded actix-web v2.0.0
  Downloaded futures-channel v0.3.4
  Downloaded enum-as-inner v0.3.2
  Downloaded actix-http v1.0.1

...
warning: 6 warnings emitted

warning: 11 warnings emitted

    Finished release [optimized] target(s) in 1m 43s
Removing intermediate container 9d945ebd92bf
 ---> 0adcefdca174
Step 7/15 : FROM debian:buster-slim
 ---> 4e22ed854b0a
Step 8/15 : WORKDIR /usr/pygmy-backend
 ---> Running in b93246292109
Removing intermediate container b93246292109
 ---> 2f60151b31e4
Step 9/15 : RUN apt update && apt install libsqlite3-0 -y && apt clean
 ---> Running in 597b9171f955

WARNING: apt does not have a stable CLI interface. Use with caution in scripts.

Get:1 http://security.debian.org/debian-security buster/updates InRelease [65.4 kB]
Get:2 http://deb.debian.org/debian buster InRelease [122 kB]
Get:3 http://deb.debian.org/debian buster-updates InRelease [49.3 kB]
Get:4 http://deb.debian.org/debian buster/main amd64 Packages [7907 kB]
Get:5 http://security.debian.org/debian-security buster/updates/main amd64 Packages [189 kB]
Get:6 http://deb.debian.org/debian buster-updates/main amd64 Packages [7380 B]
Fetched 8339 kB in 1s (5592 kB/s)
Reading package lists...
Building dependency tree...
Reading state information...
All packages are up to date.

WARNING: apt does not have a stable CLI interface. Use with caution in scripts.

Reading package lists...
Building dependency tree...
Reading state information...
The following NEW packages will be installed:
  libsqlite3-0
0 upgraded, 1 newly installed, 0 to remove and 0 not upgraded.
Need to get 641 kB of archives.
After this operation, 1319 kB of additional disk space will be used.
Get:1 http://deb.debian.org/debian buster/main amd64 libsqlite3-0 amd64 3.27.2-3 [641 kB]
debconf: delaying package configuration, since apt-utils is not installed
Fetched 641 kB in 0s (4960 kB/s)
Selecting previously unselected package libsqlite3-0:amd64.
(Reading database ... 6457 files and directories currently installed.)
Preparing to unpack .../libsqlite3-0_3.27.2-3_amd64.deb ...
Unpacking libsqlite3-0:amd64 (3.27.2-3) ...
Setting up libsqlite3-0:amd64 (3.27.2-3) ...
Processing triggers for libc-bin (2.28-10) ...

WARNING: apt does not have a stable CLI interface. Use with caution in scripts.

Removing intermediate container 597b9171f955
 ---> 8bbd5bf62d9e
Step 10/15 : COPY --from=builder /usr/pygmy-backend/target/release/catalog-server    /usr/local/bin/catalog-server
 ---> 6c03385d730d
Step 11/15 : COPY --from=builder /usr/pygmy-backend/target/release/order-server      /usr/local/bin/order-server
 ---> 2d55deb17ac6
Step 12/15 : COPY --from=builder /usr/pygmy-backend/bootstrap.sh                     .
 ---> 9c817d776b8a
Step 13/15 : COPY --from=builder /usr/pygmy-backend/db                               .
 ---> 4e9768dc237e
Step 14/15 : RUN chmod +x bootstrap.sh
 ---> Running in 13b5d9896b37
Removing intermediate container 13b5d9896b37
 ---> 7ae8d6cd943b
Step 15/15 : ENTRYPOINT ["./bootstrap.sh"]
 ---> Running in c9cbed3133e4
Removing intermediate container c9cbed3133e4
 ---> f330b3f7ae15
Successfully built f330b3f7ae15
Successfully tagged shisoft/pygmy-backend:latest
```
## Building Frontend

```
➜ sudo docker build --no-cache -t shisoft/pygmy-frontend .
Sending build context to Docker daemon  69.63kB
Step 1/9 : FROM rust:1 as builder
 ---> 3055be0d06f0
Step 2/9 : RUN rustup default nightly
 ---> Running in 2333e6b8f70a
info: syncing channel updates for 'nightly-x86_64-unknown-linux-gnu'
info: latest update on 2020-04-25, rust version 1.44.0-nightly (3360cc3a0 2020-04-24)
info: downloading component 'cargo'
info: downloading component 'rust-std'
info: downloading component 'rustc'
info: installing component 'cargo'
info: installing component 'rust-std'
info: installing component 'rustc'
info: default toolchain set to 'nightly-x86_64-unknown-linux-gnu'

  nightly-x86_64-unknown-linux-gnu installed - rustc 1.44.0-nightly (3360cc3a0 2020-04-24)

Removing intermediate container 2333e6b8f70a
 ---> 5c7995a935fe
Step 3/9 : WORKDIR /usr/pygmy-frontend
 ---> Running in 17d2d87a91c3
Removing intermediate container 17d2d87a91c3
 ---> 520d94dde8a6
Step 4/9 : COPY . .
 ---> 586ca795b379
Step 5/9 : RUN cargo build --release
 ---> Running in 51efafe41eba
    Updating crates.io index
 Downloading crates ...
  Downloaded futures v0.3.4
  Downloaded language-tags v0.2.2
  Downloaded num-integer v0.1.42
  Downloaded proc-macro2 v1.0.10
  Downloaded rand_core v0.5.1
  Downloaded proc-macro-hack v0.5.15
  Downloaded socket2 v0.3.12
  Downloaded pin-utils v0.1.0-alpha.4
  Downloaded serde_urlencoded v0.6.1
  Downloaded indexmap v1.3.2
  Downloaded backtrace v0.3.46
  Downloaded byteorder v1.3.4
  Downloaded either v1.5.3
  Downloaded flate2 v1.0.14
  Downloaded futures-io v0.3.4
  Downloaded futures-channel v0.3.4
  Downloaded hostname v0.3.1
  Downloaded httparse v1.3.4
  Downloaded getrandom v0.1.14
  Downloaded lazy_static v1.4.0
  Downloaded iovec v0.1.4
  Downloaded matches v0.1.8
  Downloaded log v0.4.8
  Downloaded itoa v0.4.5
  Downloaded atty v0.2.14
  Downloaded fnv v1.0.6
  Downloaded cfg-if v0.1.10
  Downloaded bitflags v1.2.1
  Downloaded idna v0.2.0
  Downloaded net2 v0.2.33
  Downloaded mime_guess v2.0.3
  Downloaded percent-encoding v2.1.0
  Downloaded miniz_oxide v0.3.6
  Downloaded parking_lot v0.10.2
  Downloaded proc-macro-nested v0.1.4
  Downloaded quote v1.0.3
  Downloaded ppv-lite86 v0.2.6
  Downloaded memchr v2.3.3
  Downloaded linked-hash-map v0.5.2
  Downloaded quick-error v1.2.3
  Downloaded serde_derive v1.0.106
  Downloaded scopeguard v1.1.0
  Downloaded slab v0.4.2
  Downloaded threadpool v1.7.1
  Downloaded synstructure v0.12.3
  Downloaded tokio-util v0.3.1
  Downloaded rustc-demangle v0.1.16
  Downloaded num-traits v0.2.11
  Downloaded version_check v0.9.1
  Downloaded unicode-xid v0.2.0
  Downloaded futures-macro v0.3.4
  Downloaded parking_lot_core v0.7.2
  Downloaded adler32 v1.0.4
  Downloaded base64 v0.11.0
  Downloaded unicode-bidi v0.3.4
  Downloaded crc32fast v1.2.0
  Downloaded futures-sink v0.3.4
  Downloaded failure_derive v0.1.7
  Downloaded cc v1.0.50
  Downloaded derive_more v0.99.5
  Downloaded thread_local v1.0.1
  Downloaded failure v0.1.7
  Downloaded dtoa v0.4.5
  Downloaded fxhash v0.2.1
  Downloaded serde v1.0.106
  Downloaded url v2.1.1
  Downloaded unicode-segmentation v1.6.0
  Downloaded unicode-normalization v0.1.12
  Downloaded rand v0.7.3
  Downloaded serde_json v1.0.51
  Downloaded try-lock v0.2.2
  Downloaded syn v1.0.17
  Downloaded mio v0.6.21
  Downloaded chrono v0.4.11
  Downloaded http v0.2.1
  Downloaded autocfg v1.0.0
  Downloaded encoding_rs v0.8.22
  Downloaded backtrace-sys v0.1.35
  Downloaded regex-syntax v0.6.17
  Downloaded unicase v2.6.0
  Downloaded tower-service v0.3.0
  Downloaded lru-cache v0.1.2
  Downloaded enum-as-inner v0.3.2
  Downloaded dotenv v0.9.0
  Downloaded actix-web-codegen v0.2.1
  Downloaded actix-macros v0.1.1
  Downloaded heck v0.3.1
  Downloaded simple_logger v1.6.0
  Downloaded signal-hook-registry v1.2.0
  Downloaded futures-executor v0.3.4
  Downloaded actix-codec v0.2.0
  Downloaded match_cfg v0.1.0
  Downloaded http-body v0.3.1
  Downloaded mio-uds v0.6.7
  Downloaded thread_local v0.3.6
  Downloaded actix-tls v1.0.0
  Downloaded actix-threadpool v0.3.1
  Downloaded sha1 v0.6.0
  Downloaded mime v0.3.16
  Downloaded num_cpus v1.12.0
  Downloaded actix-testing v1.0.0
  Downloaded actix-connect v1.0.2
  Downloaded utf8-ranges v1.0.4
  Downloaded pin-project-lite v0.1.4
  Downloaded futures-task v0.3.4
  Downloaded rand_chacha v0.2.2
  Downloaded copyless v0.1.4
  Downloaded actix-rt v1.0.0
  Downloaded bytestring v0.1.5
  Downloaded futures-core v0.3.4
  Downloaded ucd-util v0.1.8
  Downloaded want v0.3.0
  Downloaded pin-project v0.4.8
  Downloaded aho-corasick v0.7.10
  Downloaded smallvec v1.2.0
  Downloaded resolv-conf v0.6.3
  Downloaded bytes v0.5.4
  Downloaded pin-project-internal v0.4.8
  Downloaded ryu v1.0.3
  Downloaded time v0.1.42
  Downloaded brotli2 v0.3.2
  Downloaded colored v1.9.3
  Downloaded aho-corasick v0.6.10
  Downloaded async-trait v0.1.29
  Downloaded actix-router v0.2.4
  Downloaded actix-service v1.0.5
  Downloaded awc v1.0.1
  Downloaded actix-utils v1.0.6
  Downloaded arc-swap v0.4.5
  Downloaded lock_api v0.3.4
  Downloaded tokio-util v0.2.0
  Downloaded reqwest v0.10.4
  Downloaded trust-dns-resolver v0.18.0-alpha.2
  Downloaded regex v0.2.11
  Downloaded regex v1.3.6
  Downloaded regex-syntax v0.5.6
  Downloaded h2 v0.2.4
  Downloaded hyper v0.13.4
  Downloaded actix-http v1.0.1
  Downloaded trust-dns-proto v0.18.0-alpha.2
  Downloaded tokio v0.2.16
  Downloaded libc v0.2.68
  Downloaded brotli-sys v0.3.2
  Downloaded futures-util v0.3.4
  Downloaded actix-web v2.0.0
  Downloaded actix-server v1.0.2
   Compiling libc v0.2.68
   Compiling proc-macro2 v1.0.10
   Compiling cfg-if v0.1.10
   Compiling unicode-xid v0.2.0
   Compiling syn v1.0.17
   Compiling memchr v2.3.3
   Compiling lazy_static v1.4.0
   Compiling log v0.4.8
   Compiling slab v0.4.2
   Compiling futures-core v0.3.4
   Compiling futures-sink v0.3.4
   Compiling bytes v0.5.4
   Compiling fnv v1.0.6
   Compiling proc-macro-nested v0.1.4
   Compiling arc-swap v0.4.5
   Compiling proc-macro-hack v0.5.15
   Compiling smallvec v1.2.0
   Compiling futures-task v0.3.4
   Compiling pin-utils v0.1.0-alpha.4
   Compiling futures-io v0.3.4
   Compiling pin-project-lite v0.1.4
   Compiling autocfg v1.0.0
   Compiling cc v1.0.50
   Compiling itoa v0.4.5
   Compiling scopeguard v1.1.0
   Compiling getrandom v0.1.14
   Compiling bitflags v1.2.1
   Compiling matches v0.1.8
   Compiling copyless v0.1.4
   Compiling serde v1.0.106
   Compiling percent-encoding v2.1.0
   Compiling failure_derive v0.1.7
   Compiling rustc-demangle v0.1.16
   Compiling unicode-segmentation v1.6.0
   Compiling either v1.5.3
   Compiling ppv-lite86 v0.2.6
   Compiling ryu v1.0.3
   Compiling match_cfg v0.1.0
   Compiling httparse v1.3.4
   Compiling version_check v0.9.1
   Compiling byteorder v1.3.4
   Compiling linked-hash-map v0.5.2
   Compiling quick-error v1.2.3
   Compiling encoding_rs v0.8.22
   Compiling crc32fast v1.2.0
   Compiling regex-syntax v0.6.17
   Compiling adler32 v1.0.4
   Compiling dtoa v0.4.5
   Compiling mime v0.3.16
   Compiling base64 v0.11.0
   Compiling ucd-util v0.1.8
   Compiling try-lock v0.2.2
   Compiling language-tags v0.2.2
   Compiling sha1 v0.6.0
   Compiling regex v0.2.11
   Compiling tower-service v0.3.0
   Compiling utf8-ranges v1.0.4
   Compiling thread_local v1.0.1
   Compiling thread_local v0.3.6
   Compiling futures-channel v0.3.4
   Compiling unicode-bidi v0.3.4
   Compiling miniz_oxide v0.3.6
   Compiling lock_api v0.3.4
   Compiling regex-syntax v0.5.6
   Compiling unicode-normalization v0.1.12
   Compiling lru-cache v0.1.2
   Compiling want v0.3.0
   Compiling http v0.2.1
   Compiling bytestring v0.1.5
   Compiling aho-corasick v0.7.10
   Compiling aho-corasick v0.6.10
   Compiling heck v0.3.1
   Compiling indexmap v1.3.2
   Compiling num-traits v0.2.11
   Compiling num-integer v0.1.42
   Compiling unicase v2.6.0
   Compiling fxhash v0.2.1
   Compiling quote v1.0.3
   Compiling net2 v0.2.33
   Compiling iovec v0.1.4
   Compiling signal-hook-registry v1.2.0
   Compiling parking_lot_core v0.7.2
   Compiling num_cpus v1.12.0
   Compiling time v0.1.42
   Compiling socket2 v0.3.12
   Compiling hostname v0.3.1
   Compiling flate2 v1.0.14
   Compiling atty v0.2.14
   Compiling threadpool v1.7.1
   Compiling colored v1.9.3
   Compiling resolv-conf v0.6.3
   Compiling idna v0.2.0
   Compiling rand_core v0.5.1
   Compiling mio v0.6.21
   Compiling parking_lot v0.10.2
   Compiling mime_guess v2.0.3
   Compiling rand_chacha v0.2.2
   Compiling url v2.1.1
   Compiling rand v0.7.3
   Compiling mio-uds v0.6.7
   Compiling tokio v0.2.16
   Compiling chrono v0.4.11
   Compiling http-body v0.3.1
   Compiling regex v1.3.6
   Compiling simple_logger v1.6.0
   Compiling backtrace-sys v0.1.35
   Compiling brotli-sys v0.3.2
   Compiling dotenv v0.9.0
   Compiling tokio-util v0.2.0
   Compiling tokio-util v0.3.1
   Compiling actix-codec v0.2.0
   Compiling backtrace v0.3.46
   Compiling synstructure v0.12.3
   Compiling futures-macro v0.3.4
   Compiling pin-project-internal v0.4.8
   Compiling derive_more v0.99.5
   Compiling actix-macros v0.1.1
   Compiling serde_derive v1.0.106
   Compiling async-trait v0.1.29
   Compiling enum-as-inner v0.3.2
   Compiling actix-web-codegen v0.2.1
   Compiling brotli2 v0.3.2
   Compiling failure v0.1.7
   Compiling futures-util v0.3.4
   Compiling futures-executor v0.3.4
   Compiling h2 v0.2.4
   Compiling pin-project v0.4.8
   Compiling actix-service v1.0.5
   Compiling futures v0.3.4
   Compiling trust-dns-proto v0.18.0-alpha.2
   Compiling trust-dns-resolver v0.18.0-alpha.2
   Compiling hyper v0.13.4
   Compiling actix-threadpool v0.3.1
   Compiling actix-rt v1.0.0
   Compiling actix-utils v1.0.6
   Compiling actix-connect v1.0.2
   Compiling actix-server v1.0.2
   Compiling actix-tls v1.0.0
   Compiling actix-testing v1.0.0
   Compiling serde_urlencoded v0.6.1
   Compiling serde_json v1.0.51
   Compiling actix-router v0.2.4
   Compiling actix-http v1.0.1
   Compiling reqwest v0.10.4
   Compiling awc v1.0.1
   Compiling actix-web v2.0.0
   Compiling pygmy-frontend v0.1.0 (/usr/pygmy-frontend)
warning: unused import: `Serialize`
  --> src/main.rs:10:26
   |
10 | use serde::{Deserialize, Serialize};
   |                          ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused import: `dotenv::dotenv`
  --> src/main.rs:11:5
   |
11 | use dotenv::dotenv;
   |     ^^^^^^^^^^^^^^

warning: unused import: `dotenv::dotenv`
 --> src/configs.rs:1:5
  |
1 | use dotenv::dotenv;
  |     ^^^^^^^^^^^^^^

warning: unused variable: `req`
  --> src/main.rs:80:19
   |
80 | async fn list_all(req: HttpRequest) -> impl Responder {
   |                   ^^^ help: if this is intentional, prefix it with an underscore: `_req`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: unused `std::result::Result` that must be used
  --> src/main.rs:32:5
   |
32 |     simple_logger::init_with_level(Level::Debug);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_must_use)]` on by default
   = note: this `Result` may be an `Err` variant, which should be handled

warning: 5 warnings emitted

    Finished release [optimized] target(s) in 1m 11s
Removing intermediate container 51efafe41eba
 ---> fe578be4a6a6
Step 6/9 : FROM debian:buster-slim
 ---> 4e22ed854b0a
Step 7/9 : WORKDIR /usr/pygmy-frontend
 ---> Running in 774ef8aec7a4
Removing intermediate container 774ef8aec7a4
 ---> 40989c84e1de
Step 8/9 : COPY --from=builder /usr/pygmy-frontend/target/release/pygmy-frontend    /usr/local/bin/pygmy-frontend
 ---> 2120d716ad7b
Step 9/9 : ENTRYPOINT ["pygmy-frontend"]
 ---> Running in 513e12388b60
Removing intermediate container 513e12388b60
 ---> 8d5be04f1fdb
Successfully built 8d5be04f1fdb
Successfully tagged shisoft/pygmy-frontend:latest
```
