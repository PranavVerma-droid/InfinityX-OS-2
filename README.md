# 🌍 InfinityX-OS-2 🌍
<i>A Complete Re-Write of [InfinityX-OS](https://github.com/PranavVerma-droid/InfinityX-OS).</i><br>

You can Visit V1 [Here](https://github.com/PranavVerma-droid/InfinityX-OS). (Might be worth it to check out once, can do really cool stuff!)

<i><b>Made By: [PranavVerma-droid](https://pranavv.co.in)</b></i><br>
<i>This Code is [Licensed](LICENSE).</i>

## Scripts
All Scripts can be Found in [scripts](scripts) directory.

## Dependencies Installation
Install all Dependencies using the provided installer script (Debian only):
```bash
./scripts/install.sh
```
## Build and Run the OS (Automatic):
Run the OS:
```bash
cargo clean
cargo run --release
```

## Build and Run the OS (Manual):
Build using the `bootimage` command:
```bash
cargo clean
cargo bootimage --release
```
It will be created at `target/x86_64-infinityx/release/bootimage-os.bin`

Run a standalone `.bin` file:
```bash
qemu-system-x86_64 -drive format=raw,file=target/x86_64-infinityx/release/bootimage-os.bin
```
