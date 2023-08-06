# Xero Debugger
### Installation
Requirements
- cargo
- git

Navigate to desired target directory and run:
```
git clone https://github.com/xero-lib/xdb
cd xdb
cargo build --release
cp target/release/xdb .
```
The `xdb` binray should be in your working directory.
---
### Usage
`xdb` requires root privileges to run properly.
An example invocation would appear as:
```
sudo xdb 1
```
This would dump the memory of process with PID 1 to `stdout`.
