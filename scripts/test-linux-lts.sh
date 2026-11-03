#!/usr/bin/env bash
# Boot a disposable VM, never replace the runner's kernel or change host routes.
set -euo pipefail
cd "$(dirname "$0")/.."
version=${1:?kernel version required}
case "$version" in 6.12.*|6.18.*) ;; *) echo 'Unsupported kernel family' >&2; exit 1;; esac
[[ "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]] || exit 1
mkdir -p target
stage=$(mktemp -d "$PWD/target/lts.XXXXXX")
# This directory contains only this run's kernel sources, initramfs and logs.
trap 'rm -rf -- "$stage"' EXIT
image="$PWD/target/lts-kernels/$version/bzImage"
if [[ ! -s "$image" ]]; then
curl --fail --location --retry 2 "https://cdn.kernel.org/pub/linux/kernel/v6.x/linux-$version.tar.xz" -o "$stage/linux.tar.xz"
tar -xJf "$stage/linux.tar.xz" -C "$stage"
kernel="$stage/linux-$version"
make -C "$kernel" defconfig
"$kernel/scripts/config" --file "$kernel/.config" \
    -e TUN -e DEVTMPFS -e DEVTMPFS_MOUNT -e BLK_DEV_INITRD -e RD_GZIP \
    -e SERIAL_8250 -e SERIAL_8250_CONSOLE -e PROC_FS -e SYSFS -e TMPFS \
    -e IPV6 -e INET -e UNIX -e NET_NS -e RANDOM_TRUST_CPU -d MODULES
make -C "$kernel" olddefconfig
make -C "$kernel" -j"$(nproc)" bzImage
mkdir -p "$(dirname "$image")"
cp "$kernel/arch/x86/boot/bzImage" "$image"
fi
rustup target add x86_64-unknown-linux-musl
export CC_x86_64_unknown_linux_musl=musl-gcc
# Debug symbols are unnecessary for these functional checks and previously made
# the initramfs too large to unpack inside the test VM. Keep source assertions.
export CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0
cargo test --locked --lib --tests --all-features --target x86_64-unknown-linux-musl \
    --no-run --message-format=json > "$stage/artifacts.jsonl"
guest="$stage/root"
mkdir -p "$guest"/{bin,proc,sys,dev,tmp,tests}
cp /usr/bin/busybox "$guest/bin/busybox"
for app in sh mount mkdir ip poweroff uname; do ln -s busybox "$guest/bin/$app"; done
python3 - "$stage/artifacts.jsonl" "$guest/tests" <<'PY'
import json,pathlib,shutil,sys
out=pathlib.Path(sys.argv[2]); seen=set()
for line in open(sys.argv[1]):
    row=json.loads(line)
    exe=row.get('executable')
    if not exe or not row.get('profile',{}).get('test') or exe in seen: continue
    seen.add(exe)
    name=row['target']['name']
    shutil.copy2(exe,out/name)
assert (out/'linux_tun').is_file()
PY
strip --strip-debug "$guest/tests/"*
du -sh "$guest/tests"
cat > "$guest/init" <<'SH'
#!/bin/sh
export PATH=/bin
mount -t proc proc /proc
mount -t sysfs sysfs /sys
mount -t devtmpfs devtmpfs /dev
ip link set lo up
echo "PROTUN_KERNEL $(uname -r)"
result=0
for test in /tests/*; do
    if [ "$test" = /tests/linux_tun ]; then
        PROTUN_NEXT_NETNS_TEST=1 "$test" --ignored --nocapture --test-threads=1 || result=1
    else
        "$test" --nocapture --test-threads=1 || result=1
    fi
done
if [ "$result" = 0 ]; then echo PROTUN_LTS_PASS; else echo PROTUN_LTS_FAIL; fi
poweroff -f
SH
chmod +x "$guest/init"
(cd "$guest"; find . -print0 | cpio --null -o --format=newc | gzip -1) > "$stage/initramfs.gz"
set +e
timeout 300 qemu-system-x86_64 -machine accel=kvm:tcg -m 2048 -smp 2 -nographic -no-reboot \
    -nic none -kernel "$image" -initrd "$stage/initramfs.gz" \
    -append 'console=ttyS0 rdinit=/init panic=-1' | tee "$stage/guest.log"
status=${PIPESTATUS[0]}
set -e
mkdir -p target/lts-results
cp "$stage/guest.log" "target/lts-results/$version.log"
sed -i 's/\r$//' "$stage/guest.log"
[[ $status == 0 ]]
grep -F "PROTUN_KERNEL $version" "$stage/guest.log"
grep -x PROTUN_LTS_PASS "$stage/guest.log"
! grep -x PROTUN_LTS_FAIL "$stage/guest.log"
