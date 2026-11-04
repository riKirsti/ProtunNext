#!/usr/bin/env bash
set -euo pipefail
kernel=${1:?}
export CARGO_TARGET_DIR="$PWD/bench-target" CC_x86_64_unknown_linux_musl=musl-gcc
cargo build --manifest-path reference/Cargo.toml --release --no-default-features --features in-place-crypto,io-uring --bin decision --target x86_64-unknown-linux-musl
cargo build --manifest-path experiments/decision/tls/Cargo.toml --release --target x86_64-unknown-linux-musl
stage=$(mktemp -d "$PWD/decision-guest.XXXXXX")
trap 'rm -rf -- "$stage"' EXIT
mkdir -p "$stage"/{bin,proc,sys,dev,tmp} results
cp /usr/bin/busybox "$stage/bin/busybox"
for app in sh mount ip poweroff uname; do ln -s busybox "$stage/bin/$app"; done
cp bench-target/x86_64-unknown-linux-musl/release/decision "$stage/bin/decision"
cp bench-target/x86_64-unknown-linux-musl/release/decision-tls "$stage/bin/decision-tls"
cat > "$stage/init" <<'SH'
#!/bin/sh
export PATH=/bin
mount -t proc proc /proc
mount -t sysfs sysfs /sys
mount -t devtmpfs devtmpfs /dev
ip link set lo up
echo "DECISION_KERNEL $(uname -r)"
result=0
for round in 0 1 2 3; do
 for size in 64 1280; do
  cases='single:0:0:16 batch:0:0:16 batch:1:0:16 batch:0:1:16 batch:1:1:16 uring16:0:0:16 uring128:0:0:128 uring128:1:0:128 uring128:0:1:128 uring128:1:1:128'
  if [ "$round" = 1 ] || [ "$round" = 3 ]; then cases='uring128:1:1:128 uring128:0:1:128 uring128:1:0:128 uring128:0:0:128 uring16:0:0:16 batch:1:1:16 batch:0:1:16 batch:1:0:16 batch:0:0:16 single:0:0:16'; fi
  for spec in $cases; do
   oldifs=$IFS; IFS=:; set -- $spec; IFS=$oldifs
   echo "DECISION_CASE $round $size $spec"
   decision --udp "$1" "$size" "$round" "$2" "$3" "$4" || { echo "DECISION_CASE_FAILED $round $size $spec"; result=1; }
  done
 done
done
decision --cpu || result=1
decision --tun || result=1
decision-tls 67108864 || result=1
decision-tls --framed || result=1
echo "DECISION_FINISHED $result"
poweroff -f
SH
chmod +x "$stage/init"
(cd "$stage"; find . -print0 | cpio --null -o --format=newc | gzip -1) > "$stage/initramfs.gz"
uname -a > results/host.txt
lscpu >> results/host.txt
sha256sum bench-target/x86_64-unknown-linux-musl/release/{decision,decision-tls} > results/hashes.txt
timeout 600 sudo -n qemu-system-x86_64 -accel kvm -cpu host -m 2048 -smp 4 -nographic -no-reboot -object rng-random,filename=/dev/urandom,id=rng0 -device virtio-rng-pci,rng=rng0 -nic none -kernel "target/lts-kernels/$kernel/bzImage" -initrd "$stage/initramfs.gz" -append 'console=ttyS0 rdinit=/init panic=-1' | tee "results/lts-$kernel.log"
grep -F "DECISION_KERNEL $kernel" "results/lts-$kernel.log"
grep 'DECISION_FINISHED' "results/lts-$kernel.log"
