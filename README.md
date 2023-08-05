# Horcrust
A rust port of [horcrux](https://github.com/jesseduffield/horcrux).

Split a file into encrypted shards, no password required - secrecy preserved.

## Usage
Horcrust only has 2 commands split and bind.

### Splitting

```sh
hx split classified.txt --shards 4 --threshold 2
```

The split command supports pipes! 

```sh
cat ../files/classified.txt | hx split --shards 4 --threshold 2
```

### Binding

```sh
hx bind ../secrets
```

### Installation 
Build from source 

```sh
cargo build --release
sudo mv ./target/release/hx ~/usr/local/bin
```