# A udp based peer to peer chat app

Initially written by GPT-4 in answer to the question:

"Write a rust app that uses udp for communication to implement a chat program. Two instances of this app can then be used to send text messages, read as lines of text, from stdin and received text messages are sent to stdout."

The initial commit, as proposed by GPT-4, didn't compile. It seems
GPT-4 didn't quite understand want I wanted, probably my fault.
In any case this now works, at least on this is working when both
instances are on the same computer:

Term1:
```
wink@3900x 23-05-06T18:48:36.844Z:~/prgs/rust/myrepos/rust-udp-client-server (main)
$ cargo run 127.0.0.1:4000 127.0.0.1:4001
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/rust-udp-client-server '127.0.0.1:4000' '127.0.0.1:4001'`
message from term1 to term2
this is from term2 to term1
^C
wink@3900x 23-05-06T18:49:11.964Z:~/prgs/rust/myrepos/rust-udp-client-server (main)
```

Term2:
```
wink@3900x 23-05-06T18:48:34.049Z:~/prgs/rust/myrepos/rust-udp-client-server (main)
$ cargo run 127.0.0.1:4001 127.0.0.1:4000
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/rust-udp-client-server '127.0.0.1:4001' '127.0.0.1:4000'`
message from term1 to term2
this is from term2 to term1
^C
wink@3900x 23-05-06T18:49:07.505Z:~/prgs/rust/myrepos/rust-udp-client-server (main)
```

Use IpV6 addresses to communicate between terminals on the same computer:

Term1:
```
wink@3900x 23-05-06T19:51:28.222Z:~/prgs/rust/myrepos/rust-udp-p2p-chat (main)
$ cargo run [2600:1700:8e80:55ff:6002:2be:f4e6:b8ac]:4001 [2600:1700:8e80:55ff:6002:2be:f4e6:b8ac]:4000
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/rust-udp-p2p-chat '[2600:1700:8e80:55ff:6002:2be:f4e6:b8ac]:4001' '[2600:1700:8e80:55ff:6002:2be:f4e6:b8ac]:4000'`
hi
hello v6
good v6
^C
wink@3900x 23-05-06T19:57:38.596Z:~/prgs/rust/myrepos/rust-udp-p2p-chat (main)
```

Term2:
```
wink@3900x 23-05-06T19:50:08.743Z:~/prgs/rust/myrepos/rust-udp-p2p-chat (main)
$ cargo run [2600:1700:8e80:55ff:6002:2be:f4e6:b8ac]:4000 [2600:1700:8e80:55ff:6002:2be:f4e6:b8ac]:4001
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/rust-udp-p2p-chat '[2600:1700:8e80:55ff:6002:2be:f4e6:b8ac]:4000' '[2600:1700:8e80:55ff:6002:2be:f4e6:b8ac]:4001'`
hi
hello v6
good v6
^C
wink@3900x 23-05-06T19:52:00.855Z:~/prgs/rust/myrepos/rust-udp-p2p-chat (main)
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
