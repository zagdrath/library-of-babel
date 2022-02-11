# The Library of Babel

### Description
This repository contains the source code for [The Library of Babel](https://en.wikipedia.org/wiki/The_Library_of_Babel) developed in Rust.

### Usage
The pages in the library are identified by the wall, shelf, volume, page number(in Base10), and the room's hex address which is in Base64 seperated by colons. You are able to explore the library by either using the `search` or the `read` command.
`wall:shelf:volume:page:hex_address`

You can search for strings but you must remember to keep in mind that it must be in all lowercase a-z, have a comma, a period, or a space.
```sh
babel search "hello world"
# Or if you are running straight from cargo you can do
cargo run -- search "hello world"
```

You can read a specific library location using the `wall:shelf:volume:page:hex_address` format with the `read` command.
```sh
babel read 2:4:18:310:W3IRDhEX_n4me
# Or if you are running straight from cargo you can do
cargo run -- read 2:4:18:310:W3IRDhEX_n4me
```

### Licensing
Copyright (C) 2022, Zagdrath. This is open source software, you can modify and / or share it under the terms of the GPLv3 license file in the root directory of this project.
