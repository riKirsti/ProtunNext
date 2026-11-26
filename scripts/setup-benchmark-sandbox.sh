#!/usr/bin/env bash
set -euo pipefail

if [[ ! -x /usr/bin/bwrap ]]; then
  sudo apt-get -o Acquire::Retries=2 -o Acquire::http::Timeout=15 update
  sudo apt-get install -y --no-install-recommends bubblewrap
fi

# Ubuntu restricts namespaced network capabilities for executables without a profile.
if [[ $(cat /proc/sys/kernel/apparmor_restrict_unprivileged_userns 2>/dev/null || true) == 1 ]]; then
  sudo apparmor_parser -r <<'APPARMOR'
abi <abi/4.0>,
include <tunables/global>

profile /usr/bin/bwrap flags=(unconfined) {
  userns,
}
APPARMOR
fi

/usr/bin/bwrap --unshare-user --uid 0 --gid 0 --unshare-net \
  --ro-bind /usr /usr --ro-bind /lib /lib --ro-bind-try /lib64 /lib64 /usr/bin/true
