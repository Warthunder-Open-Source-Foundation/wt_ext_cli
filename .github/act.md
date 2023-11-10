
# Full test suite
```sh
sudo act -P ubuntu-latest=ghcr.io/catthehacker/ubuntu:rust-latest
```

# Run all tests
```sh
 sudo act -P ubuntu-latest=ghcr.io/catthehacker/ubuntu:rust-latest -W "./.github/workflows/rust.yml" 
```

# Run regression test
```sh
 sudo act -P ubuntu-latest=ghcr.io/catthehacker/ubuntu:rust-latest -W "./.github/workflows/regression.yml" 
```