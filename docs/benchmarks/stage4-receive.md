# Stage 4: bounded parallel receive/decryption

Evaluation in progress; no speed or resource improvement is claimed yet.
Baseline: merged PR #8, `a44d520c337611f59d4c7186971919d57de36df8`.

## Candidate and safety boundary

One helper authenticates an owned ciphertext prefix while the connection owner
authenticates the suffix. Only already available consecutive data messages are
batched: at most 32 originals, with batches below eight staying serial. The
existing packet pool owns every buffer; metadata vectors are reused. Channels
hold at most one outstanding batch. No payload copying or new crypto algorithm
is introduced. Linux can use the helper when its allowed affinity has two CPUs;
other production platforms retain serial delivery pending measurements.

Immutable receiving keys are leased through Arc; mutable tunnel state never
leaves the owner. Authenticated results own their buffers until ordered owner
commit. Commit rechecks key-instance identity and replay state before updating
timers, current session, IP validation/statistics or delivery. A failed tag cannot
consume a replay counter; concurrent duplicates are committed at most once.
Control messages end a group. Capacity reservations keep all results inside the
same owner processing budget, before timers, interface replacement and shutdown.
Both channel directions disconnect before helper join; abandoned packets retain
the existing erasing Drop. Session API tests cover deferred commit and stale-key
rejection; pipeline tests cover ordering, tamper/replay, real handshake barriers,
pool pressure and blocked-completion shutdown.

The extension remains within the existing declared vendored in-place patch
surface. It reuses the pinned ring AEAD; its [immutable key API](https://docs.rs/ring/0.17.14/ring/aead/struct.LessSafeKey.html)
supports shared immutable access. No session lock or unsafe sharing of the
single-owner protocol object is added.

## Fair comparison

Both baseline and candidate use negotiated vnet (`tun_mode=vnet`) and the same
two allowed client CPUs. GitHub four-CPU fixtures pin the fixed baseline peer
and its HTTP server to CPU 0, load to CPU 1, and clients to CPUs 2/3. Shared
auxiliary CPU cost is screened in aggregate; no independent-headroom assumption
or maximum-capacity inference is made from helper-limited tests. Actual client
affinity must match the recorded allocation. Candidate counters must prove the
helper and owner both authenticate batch slots. Same-binary controls and balanced
order, verified payloads, immutable identities and failure retention remain.

Report absolute client CPU %, RSS MiB and verified download Mbps by measured
platform and transport, with per-workload tables and explicit build-only targets.
The latest candidate also requires guarded temporary Travel comparisons for UDP,
TCP and Stealth, preserving settings/packages/key/Direct mode and independently
checking cleanup. Separate frame-pointer profiles do not supply speed estimates.

No candidate is selected until whole-download measurements justify its scheduling,
memory and maintenance cost. Multicore primitive throughput alone is insufficient.
