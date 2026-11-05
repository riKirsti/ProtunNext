#!/usr/bin/env bash
# Bounded actual-kernel worker screen; a guest owns all routes and TUN devices.
set -euo pipefail
cd "$(dirname "$0")/../.."
version=6.18.60
mkdir -p target/next/results
stage=$(mktemp -d "$PWD/target/ktls-worker.XXXXXX")
trap 'rm -rf -- "$stage"' EXIT
image="$PWD/target/ktls-worker-kernels/$version/bzImage"
if [[ ! -s "$image" ]]; then
  curl --fail --location --retry 2 "https://cdn.kernel.org/pub/linux/kernel/v6.x/linux-$version.tar.xz" -o "$stage/linux.tar.xz"
  tar -xJf "$stage/linux.tar.xz" -C "$stage"
  kernel="$stage/linux-$version"
  make -C "$kernel" defconfig
  "$kernel/scripts/config" --file "$kernel/.config" \
    -e TLS -e CRYPTO_AES -e CRYPTO_GCM -e CRYPTO_GHASH -e CRYPTO_AES_NI_INTEL \
    -e TUN -e DUMMY -e DEVTMPFS -e DEVTMPFS_MOUNT -e BLK_DEV_INITRD -e RD_GZIP \
    -e SERIAL_8250 -e SERIAL_8250_CONSOLE -e PROC_FS -e SYSFS -e TMPFS \
    -e IPV6 -e INET -e UNIX -e NET_NS -e RANDOM_TRUST_CPU \
    -e VIRTIO -e VIRTIO_PCI -e HW_RANDOM -e HW_RANDOM_VIRTIO -d MODULES
  make -C "$kernel" olddefconfig
  grep -x CONFIG_TLS=y "$kernel/.config"
  grep -x CONFIG_TUN=y "$kernel/.config"
  make -C "$kernel" -j"$(nproc)" bzImage
  mkdir -p "$(dirname "$image")"
  cp "$kernel/arch/x86/boot/bzImage" "$image"
  cp "$kernel/.config" "$(dirname "$image")/config"
fi
python3 experiments/ktls-worker/prepare.py
export CC_x86_64_unknown_linux_musl=musl-gcc
mkdir -p target/next/bin
for name in baseline candidate; do
  export CARGO_TARGET_DIR="$PWD/target/next/lts-cargo-$name"
  export NEXT_BUILD_ID="$(python3 -c "import json; print(json.load(open('target/next/builds.json'))['$name']['effective_sources_sha256'])")"
  source="$(python3 -c "import json; print(json.load(open('target/next/builds.json'))['$name']['path'])")"
  cargo build --locked --release --target x86_64-unknown-linux-musl --no-default-features --features unix --bin next-probe --manifest-path "$source/Cargo.toml" >"target/next/results/build-$name.log" 2>&1
  cp "$CARGO_TARGET_DIR/x86_64-unknown-linux-musl/release/next-probe" "target/next/bin/$name"
done
guest="$stage/root"
mkdir -p "$guest"/{bin,proc,sys,dev,tmp}
cp /usr/bin/busybox "$guest/bin/busybox"
for app in sh mount mkdir ip poweroff uname cat unshare; do ln -s busybox "$guest/bin/$app"; done
cp target/next/bin/{baseline,candidate} "$guest/bin/"
cat > "$guest/init" <<'SH'
#!/bin/sh
export PATH=/bin
mount -t proc proc /proc
mount -t sysfs sysfs /sys
mount -t devtmpfs devtmpfs /dev
export NEXT_PEER_CPUS=0 NEXT_LOAD_CPUS=1 NEXT_CLIENT_CPUS=2,3
echo "WORKER_KERNEL $(uname -r)"
/bin/baseline identity
/bin/candidate identity
result=0
for round in 0 1; do
  for proto in udp tcp stealth; do
    for cell in 0 1 2; do
      size=1200; window=32; flows=1
      case "$cell" in 0) size=64; window=1;; 2) flows=8;; esac
      order='baseline candidate'
      if [ "$round" = 1 ]; then order='candidate baseline'; fi
      for label in $order; do
        echo "WORKER_CASE $label $proto $size $window $flows $round"
        unshare -n "/bin/$label" client /bin/baseline "$proto" "$size" "$window" 3 "$flows" || result=1
      done
    done
  done
done
cat /proc/net/tls_stat
if [ "$result" = 0 ]; then echo WORKER_LTS_PASS; else echo WORKER_LTS_FAIL; fi
poweroff -f
SH
chmod +x "$guest/init"
(cd "$guest"; find . -print0 | cpio --null -o --format=newc | gzip -1) > "$stage/initramfs.gz"
set +e
sudo -n timeout 300 qemu-system-x86_64 -accel kvm -cpu host -m 2048 -smp 4 -nographic -no-reboot \
  -object rng-random,filename=/dev/urandom,id=rng0 -device virtio-rng-pci,rng=rng0 \
  -nic none -kernel "$image" -initrd "$stage/initramfs.gz" \
  -append 'console=ttyS0 rdinit=/init panic=-1 quiet' | tee target/next/results/guest.log
status=${PIPESTATUS[0]}
set -e
cp "$(dirname "$image")/config" target/next/results/kernel.config
[[ $status == 0 ]]
grep -q '^WORKER_LTS_PASS' target/next/results/guest.log
! grep -q '^WORKER_LTS_FAIL' target/next/results/guest.log
python3 experiments/ktls-worker/parse-lts.py
