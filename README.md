# tetragon-grpc demo

How to use tetragon grpc server in Rust!

## Demo

_This may be outdated. `cargo run` should launch the demo._

![demo](https://github.com/Aditeya/tetragon-grpc-client/assets/22963960/2e9b5429-248a-4d0b-94ce-88156aa4eb76)


## Cloning

```sh
git clone https://github.com/SubconsciousCompute/tetragon-grpc-client.git
```

## Running Policy Manager

```sh
$ cargo run --example policy -- <Option>
```

### Options:
- `list`
- `add`
- `remove`

Create a file with some content `cat foo2thebar > /tmp/tetragon`. \
cat the said file and see some output. \
Then add the policy `cargo run --example policy -- add`. \
cat the said file again and see cat gets killed before it can read. \
Run `cargo run --example policy -- list` to list the policies. \
Then remove the policy `cargo run --example policy -- remove`. \
cat said file again to see that it works.

# Running demo

This demo mimic command `sudo tetra --server-address unix:///var/run/tetragon/tetragon.sock getevents`

> *NOTE:* Requires Root. The program will ask for sudo password.

```sh
$ cargo run
```

## Troubleshooting

If you're requests aren't happening as you expect, try checking the [tetra cli code](https://github.com/cilium/tetragon/blob/main/cmd/tetra/main.go)
and how they create their request objects.
