#!/usr/bin/env bash
# Functional kTLS tests in actual supported kernels; no host kernel replacement.
set -euo pipefail
cd "$(dirname "$0")/../.."
version=${1:?version}
[[ "$version" =~ ^6\.(12|18)\.[0-9]+$ ]] || exit 1
mkdir -p target/ktls-results
stage=$(mktemp -d "$PWD/target/ktls.XXXXXX")
trap 'rm -rf -- "$stage"' EXIT
image="$PWD/target/ktls-kernels/$version/bzImage"
if [[ ! -s "$image" ]]; then
  curl --fail --location --retry 2 "https://cdn.kernel.org/pub/linux/kernel/v6.x/linux-$version.tar.xz" -o "$stage/linux.tar.xz"
  tar -xJf "$stage/linux.tar.xz" -C "$stage"
  kernel="$stage/linux-$version"
  make -C "$kernel" defconfig
  "$kernel/scripts/config" --file "$kernel/.config" \
    -e TLS -e CRYPTO_AES -e CRYPTO_GCM -e CRYPTO_GHASH -e CRYPTO_AES_NI_INTEL \
    -e DEVTMPFS -e DEVTMPFS_MOUNT -e BLK_DEV_INITRD -e RD_GZIP \
    -e SERIAL_8250 -e SERIAL_8250_CONSOLE -e PROC_FS -e SYSFS -e TMPFS \
    -e IPV6 -e INET -e UNIX -e NET_NS -e RANDOM_TRUST_CPU \
    -e VIRTIO -e VIRTIO_PCI -e HW_RANDOM -e HW_RANDOM_VIRTIO -d MODULES
  make -C "$kernel" olddefconfig
  grep -x CONFIG_TLS=y "$kernel/.config"
  make -C "$kernel" -j"$(nproc)" bzImage
  mkdir -p "$(dirname "$image")"
  cp "$kernel/arch/x86/boot/bzImage" "$image"
  cp "$kernel/.config" "$(dirname "$image")/config"
fi
export CC_x86_64_unknown_linux_musl=musl-gcc
cargo build --locked --release --target x86_64-unknown-linux-musl --manifest-path experiments/ktls/Cargo.toml
guest="$stage/root"
mkdir -p "$guest"/{bin,proc,sys,dev,tmp}
cp /usr/bin/busybox "$guest/bin/busybox"
for app in sh mount mkdir ip poweroff uname cat grep; do ln -s busybox "$guest/bin/$app"; done
cp experiments/ktls/target/x86_64-unknown-linux-musl/release/ktls-next-probe "$guest/bin/probe"
cat > "$guest/init" <<'SH'
#!/bin/sh
export PATH=/bin
mount -t proc proc /proc
mount -t sysfs sysfs /sys
mount -t devtmpfs devtmpfs /dev
ip link set lo up
echo "KTLS_KERNEL $(uname -r)"
result=0
/bin/probe tls12 || result=1
/bin/probe tls13 >/tmp/tls13.log
code=$?
cat /tmp/tls13.log
if [ "$code" != 0 ]; then
  case "$(uname -r)" in
    6.12.*) grep -q '"status":"unsupported_key_update"' /tmp/tls13.log || result=1;;
    *) result=1;;
  esac
fi
case "$(uname -r)" in
  6.18.*)
    for round in 0 1; do
      for size in 1280 16384 65536; do
        order='unbuffered ktls'
        if [ "$round" = 1 ]; then order='ktls unbuffered'; fi
        for backend in $order; do
          NEXT_TLS_PEER_CPUS=0 NEXT_TLS_CLIENT_CPUS=1 /bin/probe bench-client "$backend" "$size" 1 || result=1
        done
      done
    done;;
esac
cat /proc/net/tls_stat
if [ "$result" = 0 ]; then echo KTLS_LTS_PASS; else echo KTLS_LTS_FAIL; fi
poweroff -f
SH
chmod +x "$guest/init"
(cd "$guest"; find . -print0 | cpio --null -o --format=newc | gzip -1) > "$stage/initramfs.gz"
set +e
sudo -n timeout 180 qemu-system-x86_64 -accel kvm -cpu host -m 1024 -smp 2 -nographic -no-reboot \
  -object rng-random,filename=/dev/urandom,id=rng0 -device virtio-rng-pci,rng=rng0 \
  -nic none -kernel "$image" -initrd "$stage/initramfs.gz" \
  -append 'console=ttyS0 rdinit=/init panic=-1' | tee "$stage/guest.log"
status=${PIPESTATUS[0]}
set -e
cp "$stage/guest.log" "target/ktls-results/$version.log"
cp "$(dirname "$image")/config" "target/ktls-results/$version.config"
[[ $status == 0 ]]
sed -i 's/\r$//' "$stage/guest.log"
grep -x "KTLS_KERNEL $version" "$stage/guest.log"
grep -x KTLS_LTS_PASS "$stage/guest.log"
! grep -x KTLS_LTS_FAIL "$stage/guest.log"
