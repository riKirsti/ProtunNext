# Stage 5: GitHub Actions metrics and identities

Separate primitive, normal-release and diagnostic cohorts. No pooled effect estimate.

## Initial three-backend primitive screen: x64

[Run](https://github.com/riKirsti/ProtunNext/actions/runs/43157240325) / [artifact](https://github.com/riKirsti/ProtunNext/actions/runs/43157240325/artifacts/13916560969)

```json
{
  "kernel": "7.0.0-1017-azure",
  "machine": "x86_64",
  "original_allowed_cpus": [
    0,
    1,
    2,
    3
  ],
  "benchmark_cpu": 2,
  "compiler": "rustc 1.99.1 (58cf8f9c3 2026-10-13)\nbinary: rustc\ncommit-hash: 58cf8f9c35b355199727627d07ae34a4e2846d67\ncommit-date: 2026-10-13\nhost: x86_64-unknown-linux-gnu\nrelease: 1.99.1\nLLVM version: 23.1.1\n",
  "commit": "b3f305de0fdadb985756a035c38cfbdfa7f6703b",
  "binary_sha256": "2be73ca538338717910792b15a575d383ae86fc3cea8ad704067449c8deb2175",
  "manifest_sha256": "6d68f8619a660bd88129701de655c11d0e033f3fcb4d1a34841a8117a138e9a0",
  "lock_sha256": "dcd2eb994f19900165ca221d76ef7ffeb4815999fc195f69099e361ef31457c8",
  "source_sha256": "de6fa773d2c538d6f08bd6170b2d7f9013b516527bb35c628580f5c8e0c15102",
  "scope": "primitive-only preselection; not production/session/download performance",
  "rustflags": "",
  "order": "six rounds of ring/rustcrypto/aws-lc, alternating forward/reverse",
  "milliseconds_per_cell": 250
}
```

```jsonl
{"backend": "ring", "includes_input_restore": true, "iterations": 1472704, "ns_per_operation": 169.76193383055931, "operation": "open", "payload_bytes": 0, "seconds": 0.250009079, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1548480, "ns_per_operation": 161.45157250981606, "operation": "seal", "payload_bytes": 0, "seconds": 0.250004531, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1315904, "ns_per_operation": 189.9923201084578, "operation": "open", "payload_bytes": 64, "seconds": 0.250011654, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1321984, "ns_per_operation": 189.11745679221534, "operation": "seal", "payload_bytes": 64, "seconds": 0.250010252, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 909760, "ns_per_operation": 274.8065940467816, "operation": "open", "payload_bytes": 256, "seconds": 0.250008047, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 922816, "ns_per_operation": 270.91641562348286, "operation": "seal", "payload_bytes": 256, "seconds": 0.250006003, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 328000, "ns_per_operation": 762.3181920731707, "operation": "open", "payload_bytes": 1280, "seconds": 0.250040367, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 338624, "ns_per_operation": 738.3296281421282, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250016132, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 316672, "ns_per_operation": 789.5409508892482, "operation": "open", "payload_bytes": 1420, "seconds": 0.250025512, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 295296, "ns_per_operation": 846.7087193866495, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250029698, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 34816, "ns_per_operation": 7181.933823529412, "operation": "open", "payload_bytes": 16384, "seconds": 0.250046208, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 32704, "ns_per_operation": 7646.164200097846, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250060154, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 199424, "ns_per_operation": 1253.9422837772788, "operation": "open", "payload_bytes": 0, "seconds": 0.250066186, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 206592, "ns_per_operation": 1210.2768500232341, "operation": "seal", "payload_bytes": 0, "seconds": 0.250033515, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 139328, "ns_per_operation": 1794.4628287207165, "operation": "open", "payload_bytes": 64, "seconds": 0.250018917, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 139968, "ns_per_operation": 1786.6121542066758, "operation": "seal", "payload_bytes": 64, "seconds": 0.25006853, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 135040, "ns_per_operation": 1851.5155213270145, "operation": "open", "payload_bytes": 256, "seconds": 0.250028656, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 135040, "ns_per_operation": 1851.978843305687, "operation": "seal", "payload_bytes": 256, "seconds": 0.250091223, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 97728, "ns_per_operation": 2558.5250491159136, "operation": "open", "payload_bytes": 1280, "seconds": 0.250039536, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 97920, "ns_per_operation": 2554.3146854575166, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250118494, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 85952, "ns_per_operation": 2910.367600521221, "operation": "open", "payload_bytes": 1420, "seconds": 0.250151916, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 85888, "ns_per_operation": 2912.4078567436663, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250140886, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 19008, "ns_per_operation": 13196.607849326601, "operation": "open", "payload_bytes": 16384, "seconds": 0.250841122, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 19008, "ns_per_operation": 13191.81360479798, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250749993, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1563392, "ns_per_operation": 159.91428189475192, "operation": "open", "payload_bytes": 0, "seconds": 0.250008709, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1506304, "ns_per_operation": 165.96986664046568, "operation": "seal", "payload_bytes": 0, "seconds": 0.250001074, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1380736, "ns_per_operation": 181.0683838184852, "operation": "open", "payload_bytes": 64, "seconds": 0.250007636, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1289600, "ns_per_operation": 193.86652527915632, "operation": "seal", "payload_bytes": 64, "seconds": 0.250010271, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 941824, "ns_per_operation": 265.445111825564, "operation": "open", "payload_bytes": 256, "seconds": 0.250002577, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 900096, "ns_per_operation": 277.75919568579354, "operation": "seal", "payload_bytes": 256, "seconds": 0.250009941, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 329472, "ns_per_operation": 758.8426482371796, "operation": "open", "payload_bytes": 1280, "seconds": 0.250017405, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 309504, "ns_per_operation": 807.8090428556659, "operation": "seal", "payload_bytes": 1280, "seconds": 0.25002013, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 290240, "ns_per_operation": 861.4466372657112, "operation": "open", "payload_bytes": 1420, "seconds": 0.250026272, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 291904, "ns_per_operation": 856.476886236571, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250009029, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 35072, "ns_per_operation": 7135.846943430657, "operation": "open", "payload_bytes": 16384, "seconds": 0.250268424, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 34560, "ns_per_operation": 7239.495081018517, "operation": "seal", "payload_bytes": 16384, "seconds": 0.25019695, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1563840, "ns_per_operation": 159.86753440249643, "operation": "open", "payload_bytes": 0, "seconds": 0.250007245, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1506432, "ns_per_operation": 165.96144399481688, "operation": "seal", "payload_bytes": 0, "seconds": 0.25000963, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1380608, "ns_per_operation": 181.08496908608382, "operation": "open", "payload_bytes": 64, "seconds": 0.250007357, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1293824, "ns_per_operation": 193.22852258112385, "operation": "seal", "payload_bytes": 64, "seconds": 0.2500037, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 940992, "ns_per_operation": 265.6958868938312, "operation": "open", "payload_bytes": 256, "seconds": 0.250017704, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 906432, "ns_per_operation": 275.8244082291887, "operation": "seal", "payload_bytes": 256, "seconds": 0.25001607, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 331584, "ns_per_operation": 753.9593918886316, "operation": "open", "payload_bytes": 1280, "seconds": 0.250000871, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 336064, "ns_per_operation": 743.9366281422585, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250010319, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 326016, "ns_per_operation": 766.9361687769924, "operation": "open", "payload_bytes": 1420, "seconds": 0.250033462, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 291776, "ns_per_operation": 856.8607767602545, "operation": "seal", "payload_bytes": 1420, "seconds": 0.25001141, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 35072, "ns_per_operation": 7133.859203923358, "operation": "open", "payload_bytes": 16384, "seconds": 0.25019871, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 32704, "ns_per_operation": 7654.031678082192, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250317452, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 201984, "ns_per_operation": 1237.84247762199, "operation": "open", "payload_bytes": 0, "seconds": 0.250024375, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 204224, "ns_per_operation": 1224.187191515199, "operation": "seal", "payload_bytes": 0, "seconds": 0.250008405, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 138816, "ns_per_operation": 1801.4687427962197, "operation": "open", "payload_bytes": 64, "seconds": 0.250072685, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 139456, "ns_per_operation": 1793.2138308857277, "operation": "seal", "payload_bytes": 64, "seconds": 0.250074428, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 128832, "ns_per_operation": 1941.2782848981617, "operation": "open", "payload_bytes": 256, "seconds": 0.250098764, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 133952, "ns_per_operation": 1866.3868475274726, "operation": "seal", "payload_bytes": 256, "seconds": 0.250006251, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 94080, "ns_per_operation": 2657.519398384354, "operation": "open", "payload_bytes": 1280, "seconds": 0.250019425, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 97408, "ns_per_operation": 2567.363789421813, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250081772, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 85568, "ns_per_operation": 2922.6899658750935, "operation": "open", "payload_bytes": 1420, "seconds": 0.250088735, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 85824, "ns_per_operation": 2915.0463390193886, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250180937, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 19072, "ns_per_operation": 13150.284553271813, "operation": "open", "payload_bytes": 16384, "seconds": 0.250802227, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 18688, "ns_per_operation": 13390.206549657532, "operation": "seal", "payload_bytes": 16384, "seconds": 0.25023618, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1476736, "ns_per_operation": 169.29353113894427, "operation": "open", "payload_bytes": 0, "seconds": 0.250001852, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1553856, "ns_per_operation": 160.8924295430207, "operation": "seal", "payload_bytes": 0, "seconds": 0.250003667, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1331712, "ns_per_operation": 187.73591211913688, "operation": "open", "payload_bytes": 64, "seconds": 0.250010167, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1380992, "ns_per_operation": 181.02973225044028, "operation": "seal", "payload_bytes": 64, "seconds": 0.250000612, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 915712, "ns_per_operation": 273.02637292074365, "operation": "open", "payload_bytes": 256, "seconds": 0.250013526, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 930432, "ns_per_operation": 268.69746418867794, "operation": "seal", "payload_bytes": 256, "seconds": 0.250004719, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 330624, "ns_per_operation": 756.2831040698799, "operation": "open", "payload_bytes": 1280, "seconds": 0.250045345, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 338752, "ns_per_operation": 738.0098862884943, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250002325, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 321664, "ns_per_operation": 777.3419810734183, "operation": "open", "payload_bytes": 1420, "seconds": 0.250042931, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 292224, "ns_per_operation": 855.5087912012701, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250000201, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 35136, "ns_per_operation": 7125.840676229508, "operation": "open", "payload_bytes": 16384, "seconds": 0.250373538, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 32768, "ns_per_operation": 7631.331146240234, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250063459, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1480128, "ns_per_operation": 168.9064499827042, "operation": "open", "payload_bytes": 0, "seconds": 0.250003166, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1552384, "ns_per_operation": 161.04646530755275, "operation": "seal", "payload_bytes": 0, "seconds": 0.250005956, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1316288, "ns_per_operation": 189.9314686451597, "operation": "open", "payload_bytes": 64, "seconds": 0.250004513, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1376320, "ns_per_operation": 181.64445550453382, "operation": "seal", "payload_bytes": 64, "seconds": 0.250000897, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 916864, "ns_per_operation": 272.68386805458607, "operation": "open", "payload_bytes": 256, "seconds": 0.250014022, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 930496, "ns_per_operation": 268.69029420868014, "operation": "seal", "payload_bytes": 256, "seconds": 0.250015244, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 330240, "ns_per_operation": 757.0592478197675, "operation": "open", "payload_bytes": 1280, "seconds": 0.250011246, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 338752, "ns_per_operation": 738.0236810409974, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250006998, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 321536, "ns_per_operation": 777.657833026473, "operation": "open", "payload_bytes": 1420, "seconds": 0.250044989, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 295104, "ns_per_operation": 847.2791659889394, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250035471, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 35008, "ns_per_operation": 7143.021395109689, "operation": "open", "payload_bytes": 16384, "seconds": 0.250062893, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 34816, "ns_per_operation": 7187.272173713234, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250232068, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 204096, "ns_per_operation": 1224.9996962213859, "operation": "open", "payload_bytes": 0, "seconds": 0.250017538, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 206528, "ns_per_operation": 1210.6882408196466, "operation": "seal", "payload_bytes": 0, "seconds": 0.250041021, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 139392, "ns_per_operation": 1793.7708405073463, "operation": "open", "payload_bytes": 64, "seconds": 0.250037305, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 140096, "ns_per_operation": 1785.2319766445864, "operation": "seal", "payload_bytes": 64, "seconds": 0.250103859, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 134912, "ns_per_operation": 1853.6845721655598, "operation": "open", "payload_bytes": 256, "seconds": 0.250084293, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 135168, "ns_per_operation": 1849.901485558712, "operation": "seal", "payload_bytes": 256, "seconds": 0.250047484, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 97664, "ns_per_operation": 2560.5881286861077, "operation": "open", "payload_bytes": 1280, "seconds": 0.250077279, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 97856, "ns_per_operation": 2554.8811927730544, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250010454, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 85824, "ns_per_operation": 2913.516941648024, "operation": "open", "payload_bytes": 1420, "seconds": 0.250049678, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 86016, "ns_per_operation": 2907.412260509673, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250083973, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 18944, "ns_per_operation": 13212.31862331081, "operation": "open", "payload_bytes": 16384, "seconds": 0.250294164, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 18944, "ns_per_operation": 13209.652660472972, "operation": "seal", "payload_bytes": 16384, "seconds": 0.25024366, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1555136, "ns_per_operation": 160.7637164852463, "operation": "open", "payload_bytes": 0, "seconds": 0.250009443, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1516032, "ns_per_operation": 164.90865496242822, "operation": "seal", "payload_bytes": 0, "seconds": 0.250006798, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1379264, "ns_per_operation": 181.25723429307226, "operation": "open", "payload_bytes": 64, "seconds": 0.250001578, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1296704, "ns_per_operation": 192.8052485378313, "operation": "seal", "payload_bytes": 64, "seconds": 0.250011337, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 942400, "ns_per_operation": 265.2971508913413, "operation": "open", "payload_bytes": 256, "seconds": 0.250016035, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 906944, "ns_per_operation": 275.6596107367158, "operation": "seal", "payload_bytes": 256, "seconds": 0.25000783, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 331648, "ns_per_operation": 753.8413408191818, "operation": "open", "payload_bytes": 1280, "seconds": 0.250009973, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 334144, "ns_per_operation": 748.2462830396477, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250022006, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 325568, "ns_per_operation": 767.9159991153922, "operation": "open", "payload_bytes": 1420, "seconds": 0.250008876, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 292352, "ns_per_operation": 855.1800329739491, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250013593, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 35072, "ns_per_operation": 7133.863167198905, "operation": "open", "payload_bytes": 16384, "seconds": 0.250198849, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 34560, "ns_per_operation": 7234.194241898148, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250013753, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1563072, "ns_per_operation": 159.94517718953446, "operation": "open", "payload_bytes": 0, "seconds": 0.250005828, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1508416, "ns_per_operation": 165.74274205524205, "operation": "seal", "payload_bytes": 0, "seconds": 0.250009004, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1378624, "ns_per_operation": 181.34558878998186, "operation": "open", "payload_bytes": 64, "seconds": 0.250007381, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1296448, "ns_per_operation": 192.84313601471095, "operation": "seal", "payload_bytes": 64, "seconds": 0.250011098, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 942592, "ns_per_operation": 265.2263736590168, "operation": "open", "payload_bytes": 256, "seconds": 0.250000258, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 907072, "ns_per_operation": 275.62729639984474, "operation": "seal", "payload_bytes": 256, "seconds": 0.250013803, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 331648, "ns_per_operation": 753.8463642174836, "operation": "open", "payload_bytes": 1280, "seconds": 0.250011639, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 336384, "ns_per_operation": 743.2086662861492, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250003504, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 326016, "ns_per_operation": 766.9713480319986, "operation": "open", "payload_bytes": 1420, "seconds": 0.250044931, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 292736, "ns_per_operation": 854.1081418069523, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250028201, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 34816, "ns_per_operation": 7187.343031939337, "operation": "open", "payload_bytes": 16384, "seconds": 0.250234535, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 34048, "ns_per_operation": 7350.412535244361, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250266846, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 203904, "ns_per_operation": 1226.10641282172, "operation": "open", "payload_bytes": 0, "seconds": 0.250008002, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 206208, "ns_per_operation": 1212.6636745422097, "operation": "seal", "payload_bytes": 0, "seconds": 0.250060951, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 139520, "ns_per_operation": 1792.5651591169722, "operation": "open", "payload_bytes": 64, "seconds": 0.250098691, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 139904, "ns_per_operation": 1786.990657879689, "operation": "seal", "payload_bytes": 64, "seconds": 0.250007141, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 134848, "ns_per_operation": 1854.7468334717607, "operation": "open", "payload_bytes": 256, "seconds": 0.250108901, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 135232, "ns_per_operation": 1849.154179484146, "operation": "seal", "payload_bytes": 256, "seconds": 0.250064818, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 97408, "ns_per_operation": 2567.3754927726677, "operation": "open", "payload_bytes": 1280, "seconds": 0.250082912, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 97856, "ns_per_operation": 2555.865118132767, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250106737, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 85824, "ns_per_operation": 2914.4827087994036, "operation": "open", "payload_bytes": 1420, "seconds": 0.250132564, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 85952, "ns_per_operation": 2909.7292442293374, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250097048, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 19008, "ns_per_operation": 13183.616740319867, "operation": "open", "payload_bytes": 16384, "seconds": 0.250594187, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 18944, "ns_per_operation": 13199.886877111489, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250058657, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1479360, "ns_per_operation": 168.99216417910446, "operation": "open", "payload_bytes": 0, "seconds": 0.250000248, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1558528, "ns_per_operation": 160.41235062828517, "operation": "seal", "payload_bytes": 0, "seconds": 0.25000714, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1316160, "ns_per_operation": 189.95120578045223, "operation": "open", "payload_bytes": 64, "seconds": 0.250006179, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1375360, "ns_per_operation": 181.771295515356, "operation": "seal", "payload_bytes": 64, "seconds": 0.250000969, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 917120, "ns_per_operation": 272.6047060362875, "operation": "open", "payload_bytes": 256, "seconds": 0.250011228, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 929280, "ns_per_operation": 269.0349367252066, "operation": "seal", "payload_bytes": 256, "seconds": 0.250008786, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 330752, "ns_per_operation": 755.9956039570434, "operation": "open", "payload_bytes": 1280, "seconds": 0.250047058, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 339072, "ns_per_operation": 737.4073471121178, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250034184, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 321536, "ns_per_operation": 777.5364562599523, "operation": "open", "payload_bytes": 1420, "seconds": 0.250005962, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 295488, "ns_per_operation": 846.0929039419538, "operation": "seal", "payload_bytes": 1420, "seconds": 0.2500103, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 34880, "ns_per_operation": 7172.163073394496, "operation": "open", "payload_bytes": 16384, "seconds": 0.250165048, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 34816, "ns_per_operation": 7189.818991268382, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250320738, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1479104, "ns_per_operation": 169.02939347064168, "operation": "open", "payload_bytes": 0, "seconds": 0.250012052, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1554624, "ns_per_operation": 160.81145859063025, "operation": "seal", "payload_bytes": 0, "seconds": 0.250001353, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1320704, "ns_per_operation": 189.29610874200426, "operation": "open", "payload_bytes": 64, "seconds": 0.250004128, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1353600, "ns_per_operation": 184.69432180851064, "operation": "seal", "payload_bytes": 64, "seconds": 0.250002234, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 907904, "ns_per_operation": 275.3764704197801, "operation": "open", "payload_bytes": 256, "seconds": 0.250015399, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 922048, "ns_per_operation": 271.15245627125705, "operation": "seal", "payload_bytes": 256, "seconds": 0.25001558, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 330368, "ns_per_operation": 756.83514747191, "operation": "open", "payload_bytes": 1280, "seconds": 0.250034114, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 339008, "ns_per_operation": 737.5707947895035, "operation": "seal", "payload_bytes": 1280, "seconds": 0.2500424, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 321536, "ns_per_operation": 777.5364531498806, "operation": "open", "payload_bytes": 1420, "seconds": 0.250005961, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 295424, "ns_per_operation": 846.3832288507366, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250041919, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 34944, "ns_per_operation": 7158.647350045789, "operation": "open", "payload_bytes": 16384, "seconds": 0.250151773, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 32768, "ns_per_operation": 7630.075134277343, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250022302, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 201088, "ns_per_operation": 1243.3267673854234, "operation": "open", "payload_bytes": 0, "seconds": 0.250018093, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 204032, "ns_per_operation": 1225.5962986198244, "operation": "seal", "payload_bytes": 0, "seconds": 0.250060864, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 137600, "ns_per_operation": 1817.3172747093026, "operation": "open", "payload_bytes": 64, "seconds": 0.250062857, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 138048, "ns_per_operation": 1811.1553227862773, "operation": "seal", "payload_bytes": 64, "seconds": 0.25002637, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 133504, "ns_per_operation": 1872.660849113135, "operation": "open", "payload_bytes": 256, "seconds": 0.250007714, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 134016, "ns_per_operation": 1865.6465645893027, "operation": "seal", "payload_bytes": 256, "seconds": 0.25002649, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 94656, "ns_per_operation": 2641.163190922921, "operation": "open", "payload_bytes": 1280, "seconds": 0.250001943, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 95360, "ns_per_operation": 2621.792019714765, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250014087, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 83200, "ns_per_operation": 3004.972427884615, "operation": "open", "payload_bytes": 1420, "seconds": 0.250013706, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 84032, "ns_per_operation": 2975.5672838918504, "operation": "seal", "payload_bytes": 1420, "seconds": 0.25004287, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 19008, "ns_per_operation": 13157.894886363636, "operation": "open", "payload_bytes": 16384, "seconds": 0.250105266, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 18944, "ns_per_operation": 13218.644584037162, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250414003, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1564288, "ns_per_operation": 159.81793058567223, "operation": "open", "payload_bytes": 0, "seconds": 0.250001271, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1511360, "ns_per_operation": 165.42055235020115, "operation": "seal", "payload_bytes": 0, "seconds": 0.250010006, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1379584, "ns_per_operation": 181.21872173060868, "operation": "open", "payload_bytes": 64, "seconds": 0.250006449, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1296896, "ns_per_operation": 192.76860210841886, "operation": "seal", "payload_bytes": 64, "seconds": 0.250000829, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 941824, "ns_per_operation": 265.4439693615792, "operation": "open", "payload_bytes": 256, "seconds": 0.250001501, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 906304, "ns_per_operation": 275.86477605748183, "operation": "seal", "payload_bytes": 256, "seconds": 0.25001735, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 328832, "ns_per_operation": 760.3172683923706, "operation": "open", "payload_bytes": 1280, "seconds": 0.250016648, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 336384, "ns_per_operation": 743.322319135274, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250041735, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 326016, "ns_per_operation": 766.873162666863, "operation": "open", "payload_bytes": 1420, "seconds": 0.250012921, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 292864, "ns_per_operation": 853.7447245137674, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250031095, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 35072, "ns_per_operation": 7129.430001140512, "operation": "open", "payload_bytes": 16384, "seconds": 0.250043369, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 34624, "ns_per_operation": 7227.138747689464, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250232452, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1564416, "ns_per_operation": 159.80824793405333, "operation": "open", "payload_bytes": 0, "seconds": 0.25000658, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1506560, "ns_per_operation": 165.9425193819031, "operation": "seal", "payload_bytes": 0, "seconds": 0.250002362, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1379968, "ns_per_operation": 181.1700517693164, "operation": "open", "payload_bytes": 64, "seconds": 0.250008874, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1297600, "ns_per_operation": 192.66725955610357, "operation": "seal", "payload_bytes": 64, "seconds": 0.250005036, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 930240, "ns_per_operation": 268.76357176642586, "operation": "open", "payload_bytes": 256, "seconds": 0.250014625, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 907840, "ns_per_operation": 275.3913453912583, "operation": "seal", "payload_bytes": 256, "seconds": 0.250011279, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 331584, "ns_per_operation": 753.9599287058483, "operation": "open", "payload_bytes": 1280, "seconds": 0.250001049, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 336256, "ns_per_operation": 743.5864460411116, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250035404, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 325888, "ns_per_operation": 767.1551269147683, "operation": "open", "payload_bytes": 1420, "seconds": 0.25000665, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 292864, "ns_per_operation": 853.7834865329983, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250042447, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 34880, "ns_per_operation": 7176.789764908258, "operation": "open", "payload_bytes": 16384, "seconds": 0.250326427, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 34624, "ns_per_operation": 7221.865180221812, "operation": "seal", "payload_bytes": 16384, "seconds": 0.25004986, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 204416, "ns_per_operation": 1223.2685357310581, "operation": "open", "payload_bytes": 0, "seconds": 0.250055661, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 206336, "ns_per_operation": 1211.8521149968983, "operation": "seal", "payload_bytes": 0, "seconds": 0.250048718, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 139584, "ns_per_operation": 1791.1740099151766, "operation": "open", "payload_bytes": 64, "seconds": 0.250019233, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 139968, "ns_per_operation": 1786.8970050297212, "operation": "seal", "payload_bytes": 64, "seconds": 0.2501084, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 135104, "ns_per_operation": 1851.2496743249644, "operation": "open", "payload_bytes": 256, "seconds": 0.250111236, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 135296, "ns_per_operation": 1848.4571827696311, "operation": "seal", "payload_bytes": 256, "seconds": 0.250088863, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 97728, "ns_per_operation": 2558.9627435330713, "operation": "open", "payload_bytes": 1280, "seconds": 0.250082311, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 92672, "ns_per_operation": 2699.154027106354, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250136002, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 85952, "ns_per_operation": 2909.780447226359, "operation": "open", "payload_bytes": 1420, "seconds": 0.250101449, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 85952, "ns_per_operation": 2909.0282715934477, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250036798, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 19072, "ns_per_operation": 13131.929739932884, "operation": "open", "payload_bytes": 16384, "seconds": 0.250452164, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 18368, "ns_per_operation": 13618.80444250871, "operation": "seal", "payload_bytes": 16384, "seconds": 0.2501502, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1479680, "ns_per_operation": 168.9563567798443, "operation": "open", "payload_bytes": 0, "seconds": 0.250001342, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1557888, "ns_per_operation": 160.47569273272532, "operation": "seal", "payload_bytes": 0, "seconds": 0.250003156, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1322048, "ns_per_operation": 189.1074507128334, "operation": "open", "payload_bytes": 64, "seconds": 0.250009127, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1377152, "ns_per_operation": 181.53430412910123, "operation": "seal", "payload_bytes": 64, "seconds": 0.25000033, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 916160, "ns_per_operation": 272.88716490569334, "operation": "open", "payload_bytes": 256, "seconds": 0.250008305, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 931520, "ns_per_operation": 268.3816074802473, "operation": "seal", "payload_bytes": 256, "seconds": 0.250002835, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 330688, "ns_per_operation": 756.1058883297852, "operation": "open", "payload_bytes": 1280, "seconds": 0.250035144, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 339008, "ns_per_operation": 737.513099985841, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250022841, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 321344, "ns_per_operation": 778.0517171629157, "operation": "open", "payload_bytes": 1420, "seconds": 0.250022251, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 294592, "ns_per_operation": 848.6655374212471, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250010078, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 34752, "ns_per_operation": 7199.942046500921, "operation": "open", "payload_bytes": 16384, "seconds": 0.250212386, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 34816, "ns_per_operation": 7187.752872242647, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250248804, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
```

## Initial three-backend primitive screen: arm

[Run](https://github.com/riKirsti/ProtunNext/actions/runs/43157240325) / [artifact](https://github.com/riKirsti/ProtunNext/actions/runs/43157240325/artifacts/13916695898)

```json
{
  "kernel": "7.0.0-1017-azure",
  "machine": "aarch64",
  "original_allowed_cpus": [
    0,
    1,
    2,
    3
  ],
  "benchmark_cpu": 2,
  "compiler": "rustc 1.99.1 (58cf8f9c3 2026-10-13)\nbinary: rustc\ncommit-hash: 58cf8f9c35b355199727627d07ae34a4e2846d67\ncommit-date: 2026-10-13\nhost: aarch64-unknown-linux-gnu\nrelease: 1.99.1\nLLVM version: 23.1.1\n",
  "commit": "b3f305de0fdadb985756a035c38cfbdfa7f6703b",
  "binary_sha256": "28243c070b9c5f70192c197b7ef674ea60d3d06b62169053b44c302fca486de2",
  "manifest_sha256": "6d68f8619a660bd88129701de655c11d0e033f3fcb4d1a34841a8117a138e9a0",
  "lock_sha256": "dcd2eb994f19900165ca221d76ef7ffeb4815999fc195f69099e361ef31457c8",
  "source_sha256": "de6fa773d2c538d6f08bd6170b2d7f9013b516527bb35c628580f5c8e0c15102",
  "scope": "primitive-only preselection; not production/session/download performance",
  "rustflags": "",
  "order": "six rounds of ring/rustcrypto/aws-lc, alternating forward/reverse",
  "milliseconds_per_cell": 250
}
```

```jsonl
{"backend": "ring", "includes_input_restore": true, "iterations": 1059200, "ns_per_operation": 236.03979607250758, "operation": "open", "payload_bytes": 0, "seconds": 0.250013352, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1107200, "ns_per_operation": 225.80090408236998, "operation": "seal", "payload_bytes": 0, "seconds": 0.250006761, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 979712, "ns_per_operation": 255.18551676411025, "operation": "open", "payload_bytes": 64, "seconds": 0.250008313, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1010816, "ns_per_operation": 247.33292805020895, "operation": "seal", "payload_bytes": 64, "seconds": 0.250008081, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 517888, "ns_per_operation": 482.7791800543747, "operation": "open", "payload_bytes": 256, "seconds": 0.250025544, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 657472, "ns_per_operation": 380.28074807748465, "operation": "seal", "payload_bytes": 256, "seconds": 0.250023944, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 190720, "ns_per_operation": 1310.9145553691276, "operation": "open", "payload_bytes": 1280, "seconds": 0.250017624, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 175936, "ns_per_operation": 1421.2176643779558, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250043351, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 162048, "ns_per_operation": 1543.0621420813584, "operation": "open", "payload_bytes": 1420, "seconds": 0.250050134, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 168384, "ns_per_operation": 1484.913245914101, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250035632, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 17280, "ns_per_operation": 14503.172106481483, "operation": "open", "payload_bytes": 16384, "seconds": 0.250614814, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 17344, "ns_per_operation": 14449.709755535057, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250615766, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 1325824, "ns_per_operation": 188.56345186088046, "operation": "open", "payload_bytes": 0, "seconds": 0.25000195, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 1405376, "ns_per_operation": 177.88931289562365, "operation": "seal", "payload_bytes": 0, "seconds": 0.250001371, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 754880, "ns_per_operation": 331.1999390631624, "operation": "open", "payload_bytes": 64, "seconds": 0.25001621, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 771584, "ns_per_operation": 324.02916856751824, "operation": "seal", "payload_bytes": 64, "seconds": 0.250015722, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 324992, "ns_per_operation": 769.2781422311934, "operation": "open", "payload_bytes": 256, "seconds": 0.250009242, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 327232, "ns_per_operation": 764.0849305691374, "operation": "seal", "payload_bytes": 256, "seconds": 0.25003304, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 80000, "ns_per_operation": 3127.3490874999998, "operation": "open", "payload_bytes": 1280, "seconds": 0.250187927, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 80256, "ns_per_operation": 3115.925413676236, "operation": "seal", "payload_bytes": 1280, "seconds": 0.25007171, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 70464, "ns_per_operation": 3550.7412579473207, "operation": "open", "payload_bytes": 1420, "seconds": 0.250199432, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 70784, "ns_per_operation": 3535.023479882459, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250223102, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 6592, "ns_per_operation": 37927.85452063107, "operation": "open", "payload_bytes": 16384, "seconds": 0.250020417, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 6656, "ns_per_operation": 37804.700120192305, "operation": "seal", "payload_bytes": 16384, "seconds": 0.251628084, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1090176, "ns_per_operation": 229.33108232065283, "operation": "open", "payload_bytes": 0, "seconds": 0.250011242, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1083328, "ns_per_operation": 230.77883891120695, "operation": "seal", "payload_bytes": 0, "seconds": 0.250009178, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 998784, "ns_per_operation": 250.3194224176599, "operation": "open", "payload_bytes": 64, "seconds": 0.250015034, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 985152, "ns_per_operation": 253.76854231631262, "operation": "seal", "payload_bytes": 64, "seconds": 0.250000587, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 525056, "ns_per_operation": 476.1628664371038, "operation": "open", "payload_bytes": 256, "seconds": 0.25001217, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 647680, "ns_per_operation": 385.99690279150195, "operation": "seal", "payload_bytes": 256, "seconds": 0.250002474, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 191424, "ns_per_operation": 1306.4405142510868, "operation": "open", "payload_bytes": 1280, "seconds": 0.250084069, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 175296, "ns_per_operation": 1426.6359414932456, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250083574, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 162496, "ns_per_operation": 1538.5583522055927, "operation": "open", "payload_bytes": 1420, "seconds": 0.250009578, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 167680, "ns_per_operation": 1491.2890207538169, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250059343, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 17280, "ns_per_operation": 14504.619328703704, "operation": "open", "payload_bytes": 16384, "seconds": 0.250639822, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 17344, "ns_per_operation": 14443.132726014761, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250501694, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1089408, "ns_per_operation": 229.49301822641286, "operation": "open", "payload_bytes": 0, "seconds": 0.25001153, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1085824, "ns_per_operation": 230.2437024784864, "operation": "seal", "payload_bytes": 0, "seconds": 0.250004138, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 997888, "ns_per_operation": 250.52950531522575, "operation": "open", "payload_bytes": 64, "seconds": 0.250000387, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 985984, "ns_per_operation": 253.55445321627937, "operation": "seal", "payload_bytes": 64, "seconds": 0.250000634, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 524928, "ns_per_operation": 476.2602166392344, "operation": "open", "payload_bytes": 256, "seconds": 0.250002323, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 648384, "ns_per_operation": 385.5959215526602, "operation": "seal", "payload_bytes": 256, "seconds": 0.250014226, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 191424, "ns_per_operation": 1306.3348744149114, "operation": "open", "payload_bytes": 1280, "seconds": 0.250063847, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 175296, "ns_per_operation": 1426.2084873585247, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250008643, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 162560, "ns_per_operation": 1538.22452632874, "operation": "open", "payload_bytes": 1420, "seconds": 0.250053779, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 167680, "ns_per_operation": 1491.0336235687025, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250016518, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 17280, "ns_per_operation": 14507.1859375, "operation": "open", "payload_bytes": 16384, "seconds": 0.250684173, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 17344, "ns_per_operation": 14454.137684501846, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250692564, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 1320320, "ns_per_operation": 189.35609397721765, "operation": "open", "payload_bytes": 0, "seconds": 0.250010638, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 1400832, "ns_per_operation": 178.4710786161367, "operation": "seal", "payload_bytes": 0, "seconds": 0.250007998, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 754240, "ns_per_operation": 331.466416525244, "operation": "open", "payload_bytes": 64, "seconds": 0.25000523, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 771200, "ns_per_operation": 324.18870202282153, "operation": "seal", "payload_bytes": 64, "seconds": 0.250014327, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 324480, "ns_per_operation": 770.5436113165681, "operation": "open", "payload_bytes": 256, "seconds": 0.250025991, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 327424, "ns_per_operation": 763.5946754055904, "operation": "seal", "payload_bytes": 256, "seconds": 0.250019223, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 80000, "ns_per_operation": 3127.0812874999997, "operation": "open", "payload_bytes": 1280, "seconds": 0.250166503, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 80256, "ns_per_operation": 3115.558388157895, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250042254, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 70400, "ns_per_operation": 3551.5222443181824, "operation": "open", "payload_bytes": 1420, "seconds": 0.250027166, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 70720, "ns_per_operation": 3535.2913885746602, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250015807, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 6656, "ns_per_operation": 37912.489783653844, "operation": "open", "payload_bytes": 16384, "seconds": 0.252345532, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 6656, "ns_per_operation": 37875.872746394234, "operation": "seal", "payload_bytes": 16384, "seconds": 0.252101809, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1066688, "ns_per_operation": 234.3788764849703, "operation": "open", "payload_bytes": 0, "seconds": 0.250009135, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1107968, "ns_per_operation": 225.64638599670752, "operation": "seal", "payload_bytes": 0, "seconds": 0.250008975, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 983808, "ns_per_operation": 254.1307409575852, "operation": "open", "payload_bytes": 64, "seconds": 0.250015856, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1013056, "ns_per_operation": 246.78145334512607, "operation": "seal", "payload_bytes": 64, "seconds": 0.250003432, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 518912, "ns_per_operation": 481.8042828071041, "operation": "open", "payload_bytes": 256, "seconds": 0.250014024, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 657344, "ns_per_operation": 380.3456439854931, "operation": "seal", "payload_bytes": 256, "seconds": 0.250017927, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 190720, "ns_per_operation": 1310.9705851510066, "operation": "open", "payload_bytes": 1280, "seconds": 0.25002831, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 176192, "ns_per_operation": 1419.3997684344351, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250086884, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 161984, "ns_per_operation": 1543.7432585934414, "operation": "open", "payload_bytes": 1420, "seconds": 0.250061708, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 168576, "ns_per_operation": 1483.3025697608202, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250049214, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 17280, "ns_per_operation": 14505.82002314815, "operation": "open", "payload_bytes": 16384, "seconds": 0.25066057, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 17344, "ns_per_operation": 14449.344903136533, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250609438, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1062400, "ns_per_operation": 235.32099397590363, "operation": "open", "payload_bytes": 0, "seconds": 0.250005024, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1107712, "ns_per_operation": 225.69744482320317, "operation": "seal", "payload_bytes": 0, "seconds": 0.250007768, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 979456, "ns_per_operation": 255.2472556194459, "operation": "open", "payload_bytes": 64, "seconds": 0.250003456, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1010624, "ns_per_operation": 247.38111800234313, "operation": "seal", "payload_bytes": 64, "seconds": 0.250009295, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 518784, "ns_per_operation": 481.92482227670865, "operation": "open", "payload_bytes": 256, "seconds": 0.250014887, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 657216, "ns_per_operation": 380.4217502312786, "operation": "seal", "payload_bytes": 256, "seconds": 0.250019261, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 190784, "ns_per_operation": 1310.405510944314, "operation": "open", "payload_bytes": 1280, "seconds": 0.250004405, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 176064, "ns_per_operation": 1420.1405114049437, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250035619, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 162112, "ns_per_operation": 1542.7457498519543, "operation": "open", "payload_bytes": 1420, "seconds": 0.250097599, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 168384, "ns_per_operation": 1484.9753539528697, "operation": "seal", "payload_bytes": 1420, "seconds": 0.25004609, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 17280, "ns_per_operation": 14505.52696759259, "operation": "open", "payload_bytes": 16384, "seconds": 0.250655506, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 17344, "ns_per_operation": 14449.148812269374, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250606037, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 1313920, "ns_per_operation": 190.2791745311739, "operation": "open", "payload_bytes": 0, "seconds": 0.250011613, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 1397504, "ns_per_operation": 178.89407185954389, "operation": "seal", "payload_bytes": 0, "seconds": 0.250005181, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 754752, "ns_per_operation": 331.24032927372167, "operation": "open", "payload_bytes": 64, "seconds": 0.250004301, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 771968, "ns_per_operation": 323.86703593931355, "operation": "seal", "payload_bytes": 64, "seconds": 0.250014988, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 324480, "ns_per_operation": 770.5047707100591, "operation": "open", "payload_bytes": 256, "seconds": 0.250013388, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 327360, "ns_per_operation": 763.7126252443794, "operation": "seal", "payload_bytes": 256, "seconds": 0.250008965, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 80128, "ns_per_operation": 3122.3347768570284, "operation": "open", "payload_bytes": 1280, "seconds": 0.250186441, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 80320, "ns_per_operation": 3114.6684511952194, "operation": "seal", "payload_bytes": 1280, "seconds": 0.25017017, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 70528, "ns_per_operation": 3545.6726122958257, "operation": "open", "payload_bytes": 1420, "seconds": 0.250069198, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 70848, "ns_per_operation": 3531.4669433152662, "operation": "seal", "payload_bytes": 1420, "seconds": 0.25019737, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 6656, "ns_per_operation": 37879.974308894234, "operation": "open", "payload_bytes": 16384, "seconds": 0.252129109, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 6656, "ns_per_operation": 37858.876953125, "operation": "seal", "payload_bytes": 16384, "seconds": 0.251988685, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1089728, "ns_per_operation": 229.4215859370412, "operation": "open", "payload_bytes": 0, "seconds": 0.250007126, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1085376, "ns_per_operation": 230.33556389675098, "operation": "seal", "payload_bytes": 0, "seconds": 0.250000693, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1000128, "ns_per_operation": 249.96888098323416, "operation": "open", "payload_bytes": 64, "seconds": 0.250000877, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 987008, "ns_per_operation": 253.3004767945143, "operation": "seal", "payload_bytes": 64, "seconds": 0.250009597, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 524928, "ns_per_operation": 476.28989309009995, "operation": "open", "payload_bytes": 256, "seconds": 0.250017901, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 648576, "ns_per_operation": 385.4838862369252, "operation": "seal", "payload_bytes": 256, "seconds": 0.250015597, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 191424, "ns_per_operation": 1306.2628040371114, "operation": "open", "payload_bytes": 1280, "seconds": 0.250050051, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 175424, "ns_per_operation": 1425.4026530007297, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250049835, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 162688, "ns_per_operation": 1536.9678095495672, "operation": "open", "payload_bytes": 1420, "seconds": 0.250046219, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 167808, "ns_per_operation": 1489.9661756292905, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250028244, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 17280, "ns_per_operation": 14503.518229166666, "operation": "open", "payload_bytes": 16384, "seconds": 0.250620795, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 17344, "ns_per_operation": 14444.947359317342, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250533167, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1082368, "ns_per_operation": 230.982748935667, "operation": "open", "payload_bytes": 0, "seconds": 0.250008336, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1091136, "ns_per_operation": 229.12683661798346, "operation": "seal", "payload_bytes": 0, "seconds": 0.25000854, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 995136, "ns_per_operation": 251.23037956621005, "operation": "open", "payload_bytes": 64, "seconds": 0.250008395, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 995392, "ns_per_operation": 251.17337792548062, "operation": "seal", "payload_bytes": 64, "seconds": 0.250015971, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 523776, "ns_per_operation": 477.3151041666667, "operation": "open", "payload_bytes": 256, "seconds": 0.250006196, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 651072, "ns_per_operation": 383.993246522658, "operation": "seal", "payload_bytes": 256, "seconds": 0.250007251, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 191232, "ns_per_operation": 1307.3563158885543, "operation": "open", "payload_bytes": 1280, "seconds": 0.250008363, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 175552, "ns_per_operation": 1424.4273605541377, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250061072, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 162496, "ns_per_operation": 1538.6775428318233, "operation": "open", "payload_bytes": 1420, "seconds": 0.250028946, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 168000, "ns_per_operation": 1488.375476190476, "operation": "seal", "payload_bytes": 1420, "seconds": 0.25004708, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 17280, "ns_per_operation": 14509.454571759261, "operation": "open", "payload_bytes": 16384, "seconds": 0.250723375, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 17344, "ns_per_operation": 14443.773754612545, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250512812, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 1319680, "ns_per_operation": 189.44375833535403, "operation": "open", "payload_bytes": 0, "seconds": 0.250005139, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 1402176, "ns_per_operation": 178.30101499383815, "operation": "seal", "payload_bytes": 0, "seconds": 0.250009404, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 754432, "ns_per_operation": 331.37660650661695, "operation": "open", "payload_bytes": 64, "seconds": 0.250001116, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 771456, "ns_per_operation": 324.0770530010785, "operation": "seal", "payload_bytes": 64, "seconds": 0.250011187, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 324480, "ns_per_operation": 770.611073101578, "operation": "open", "payload_bytes": 256, "seconds": 0.250047881, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 327360, "ns_per_operation": 763.7209769061583, "operation": "seal", "payload_bytes": 256, "seconds": 0.250011699, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 79936, "ns_per_operation": 3129.1037329863893, "operation": "open", "payload_bytes": 1280, "seconds": 0.250128036, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 80192, "ns_per_operation": 3118.4188946528334, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250072248, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 70528, "ns_per_operation": 3547.0930268829406, "operation": "open", "payload_bytes": 1420, "seconds": 0.250169377, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 70784, "ns_per_operation": 3534.622965641953, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250194752, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 6656, "ns_per_operation": 37910.996995192305, "operation": "open", "payload_bytes": 16384, "seconds": 0.252335596, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 6656, "ns_per_operation": 37848.38070913462, "operation": "seal", "payload_bytes": 16384, "seconds": 0.251918822, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1068544, "ns_per_operation": 233.9652424233349, "operation": "open", "payload_bytes": 0, "seconds": 0.250002156, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1108160, "ns_per_operation": 225.60947426364427, "operation": "seal", "payload_bytes": 0, "seconds": 0.250011395, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 983680, "ns_per_operation": 254.1560700634353, "operation": "open", "payload_bytes": 64, "seconds": 0.250008243, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1012864, "ns_per_operation": 246.83500548938457, "operation": "seal", "payload_bytes": 64, "seconds": 0.250010291, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 518784, "ns_per_operation": 481.95361846163337, "operation": "open", "payload_bytes": 256, "seconds": 0.250029826, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 656960, "ns_per_operation": 380.5710941305407, "operation": "seal", "payload_bytes": 256, "seconds": 0.250019986, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 190720, "ns_per_operation": 1310.8381449244964, "operation": "open", "payload_bytes": 1280, "seconds": 0.250003051, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 176192, "ns_per_operation": 1418.9670529876496, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250010643, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 162048, "ns_per_operation": 1543.2161705173776, "operation": "open", "payload_bytes": 1420, "seconds": 0.250075094, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 168512, "ns_per_operation": 1483.776324534751, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250034116, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 17280, "ns_per_operation": 14505.512962962963, "operation": "open", "payload_bytes": 16384, "seconds": 0.250655264, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 17344, "ns_per_operation": 14449.643104243543, "operation": "seal", "payload_bytes": 16384, "seconds": 0.25061461, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1061824, "ns_per_operation": 235.45024128292448, "operation": "open", "payload_bytes": 0, "seconds": 0.250006717, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1107776, "ns_per_operation": 225.68618746028076, "operation": "seal", "payload_bytes": 0, "seconds": 0.250009742, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 979968, "ns_per_operation": 255.12351933940704, "operation": "open", "payload_bytes": 64, "seconds": 0.250012885, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1011456, "ns_per_operation": 247.175713031511, "operation": "seal", "payload_bytes": 64, "seconds": 0.250007358, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 519040, "ns_per_operation": 481.6814677096178, "operation": "open", "payload_bytes": 256, "seconds": 0.250011949, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 657984, "ns_per_operation": 379.95657797150085, "operation": "seal", "payload_bytes": 256, "seconds": 0.250005349, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 190848, "ns_per_operation": 1310.3526261737088, "operation": "open", "payload_bytes": 1280, "seconds": 0.250078178, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 176128, "ns_per_operation": 1419.7962958757269, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250065882, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 162048, "ns_per_operation": 1542.754628258294, "operation": "open", "payload_bytes": 1420, "seconds": 0.250000302, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 168384, "ns_per_operation": 1485.1705684625617, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250078961, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 17280, "ns_per_operation": 14502.879803240741, "operation": "open", "payload_bytes": 16384, "seconds": 0.250609763, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 17344, "ns_per_operation": 14451.836715867157, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250652656, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 1310208, "ns_per_operation": 190.8120344250684, "operation": "open", "payload_bytes": 0, "seconds": 0.250003454, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 1399360, "ns_per_operation": 178.65762491424653, "operation": "seal", "payload_bytes": 0, "seconds": 0.250006334, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 754496, "ns_per_operation": 331.3478640045805, "operation": "open", "payload_bytes": 64, "seconds": 0.250000638, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 771456, "ns_per_operation": 324.0663757881201, "operation": "seal", "payload_bytes": 64, "seconds": 0.25000295, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 324544, "ns_per_operation": 770.3839386955235, "operation": "open", "payload_bytes": 256, "seconds": 0.250023485, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 327552, "ns_per_operation": 763.3548871629542, "operation": "seal", "payload_bytes": 256, "seconds": 0.25003842, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 79936, "ns_per_operation": 3128.369020216173, "operation": "open", "payload_bytes": 1280, "seconds": 0.250069306, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 80384, "ns_per_operation": 3112.4415182125795, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250190499, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 70528, "ns_per_operation": 3547.099648366606, "operation": "open", "payload_bytes": 1420, "seconds": 0.250169844, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 70912, "ns_per_operation": 3527.981554602888, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250176228, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 6656, "ns_per_operation": 37749.808293269234, "operation": "open", "payload_bytes": 16384, "seconds": 0.251262724, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 6656, "ns_per_operation": 37851.909254807695, "operation": "seal", "payload_bytes": 16384, "seconds": 0.251942308, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1090752, "ns_per_operation": 229.21028153054039, "operation": "open", "payload_bytes": 0, "seconds": 0.250011573, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1083264, "ns_per_operation": 230.7849296201111, "operation": "seal", "payload_bytes": 0, "seconds": 0.250001006, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 998912, "ns_per_operation": 250.28561174557922, "operation": "open", "payload_bytes": 64, "seconds": 0.250013301, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 985024, "ns_per_operation": 253.81739023617695, "operation": "seal", "payload_bytes": 64, "seconds": 0.250016221, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 525184, "ns_per_operation": 476.0697641207653, "operation": "open", "payload_bytes": 256, "seconds": 0.250024223, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 647936, "ns_per_operation": 385.8676968095615, "operation": "seal", "payload_bytes": 256, "seconds": 0.250017572, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 191424, "ns_per_operation": 1306.4373485038448, "operation": "open", "payload_bytes": 1280, "seconds": 0.250083463, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 175296, "ns_per_operation": 1426.501283543264, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250059969, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 162752, "ns_per_operation": 1536.6354637731029, "operation": "open", "payload_bytes": 1420, "seconds": 0.250090495, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 167680, "ns_per_operation": 1491.1348878816796, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250033498, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 17280, "ns_per_operation": 14503.391203703706, "operation": "open", "payload_bytes": 16384, "seconds": 0.2506186, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 17344, "ns_per_operation": 14449.234317343171, "operation": "seal", "payload_bytes": 16384, "seconds": 0.25060752, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1082624, "ns_per_operation": 230.9331060460511, "operation": "open", "payload_bytes": 0, "seconds": 0.250013723, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1090368, "ns_per_operation": 229.29177213564594, "operation": "seal", "payload_bytes": 0, "seconds": 0.250012411, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 995008, "ns_per_operation": 251.25924213674665, "operation": "open", "payload_bytes": 64, "seconds": 0.250004956, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 994688, "ns_per_operation": 251.34408779436367, "operation": "seal", "payload_bytes": 64, "seconds": 0.250008948, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 523904, "ns_per_operation": 477.235894362326, "operation": "open", "payload_bytes": 256, "seconds": 0.250025794, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 651008, "ns_per_operation": 384.0245588379866, "operation": "seal", "payload_bytes": 256, "seconds": 0.25000306, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 191232, "ns_per_operation": 1307.500345130522, "operation": "open", "payload_bytes": 1280, "seconds": 0.250035906, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 175488, "ns_per_operation": 1424.6908449580599, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250016147, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 162560, "ns_per_operation": 1538.1061208169292, "operation": "open", "payload_bytes": 1420, "seconds": 0.250034531, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 167936, "ns_per_operation": 1488.9808558022105, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250053489, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 17280, "ns_per_operation": 14504.540682870369, "operation": "open", "payload_bytes": 16384, "seconds": 0.250638463, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 17344, "ns_per_operation": 14443.191766605167, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250502718, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 1307712, "ns_per_operation": 191.1753199481231, "operation": "open", "payload_bytes": 0, "seconds": 0.25000226, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 1398016, "ns_per_operation": 178.8286571827504, "operation": "seal", "payload_bytes": 0, "seconds": 0.250005324, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 753856, "ns_per_operation": 331.6288362764242, "operation": "open", "payload_bytes": 64, "seconds": 0.250000388, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 769792, "ns_per_operation": 324.76462862695377, "operation": "seal", "payload_bytes": 64, "seconds": 0.250001213, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 324544, "ns_per_operation": 770.3548455679354, "operation": "open", "payload_bytes": 256, "seconds": 0.250014043, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 327552, "ns_per_operation": 763.2673743405627, "operation": "seal", "payload_bytes": 256, "seconds": 0.250009755, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 79872, "ns_per_operation": 3131.0773111979165, "operation": "open", "payload_bytes": 1280, "seconds": 0.250085407, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 80256, "ns_per_operation": 3116.9998380183415, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250157939, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 70656, "ns_per_operation": 3541.442014832428, "operation": "open", "payload_bytes": 1420, "seconds": 0.250224127, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 70720, "ns_per_operation": 3536.6539309954755, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250112166, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 6656, "ns_per_operation": 37809.26968149038, "operation": "open", "payload_bytes": 16384, "seconds": 0.251658499, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 6656, "ns_per_operation": 37765.47055288462, "operation": "seal", "payload_bytes": 16384, "seconds": 0.251366972, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1061952, "ns_per_operation": 235.42450129572708, "operation": "open", "payload_bytes": 0, "seconds": 0.25000952, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1107712, "ns_per_operation": 225.69861028859484, "operation": "seal", "payload_bytes": 0, "seconds": 0.250009059, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 980352, "ns_per_operation": 255.01973474833528, "operation": "open", "payload_bytes": 64, "seconds": 0.250009107, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1011136, "ns_per_operation": 247.25199676403574, "operation": "seal", "payload_bytes": 64, "seconds": 0.250005395, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 518720, "ns_per_operation": 481.98163749228877, "operation": "open", "payload_bytes": 256, "seconds": 0.250013515, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 657408, "ns_per_operation": 380.29797477365656, "operation": "seal", "payload_bytes": 256, "seconds": 0.250010931, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 190848, "ns_per_operation": 1310.308266264252, "operation": "open", "payload_bytes": 1280, "seconds": 0.250069712, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 176128, "ns_per_operation": 1419.4807526344478, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250010306, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 162048, "ns_per_operation": 1542.9155867397317, "operation": "open", "payload_bytes": 1420, "seconds": 0.250026385, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 168384, "ns_per_operation": 1485.0380558722918, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250056648, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 17280, "ns_per_operation": 14505.606944444446, "operation": "open", "payload_bytes": 16384, "seconds": 0.250656888, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 17344, "ns_per_operation": 14452.177583025832, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250658568, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
```

## Current four-backend primitive screen: x64

[Run](https://github.com/riKirsti/ProtunNext/actions/runs/43157895797) / [artifact](https://github.com/riKirsti/ProtunNext/actions/runs/43157895797/artifacts/13916941373)

```json
{
  "kernel": "7.0.0-1017-azure",
  "machine": "x86_64",
  "original_allowed_cpus": [
    0,
    1,
    2,
    3
  ],
  "benchmark_cpu": 2,
  "compiler": "rustc 1.99.1 (58cf8f9c3 2026-10-13)\nbinary: rustc\ncommit-hash: 58cf8f9c35b355199727627d07ae34a4e2846d67\ncommit-date: 2026-10-13\nhost: x86_64-unknown-linux-gnu\nrelease: 1.99.1\nLLVM version: 23.1.1\n",
  "commit": "0b62c99f0939b988ad14583f6f725dc1d30276e5",
  "binary_sha256": "c9616d60f9670200c93c128e194fe74f090abb8771e1c58e93166a7c6fe8e594",
  "manifest_sha256": "72e4b85e6da0c806f16b33667ad36a93c172229e8404ff2e9cb5009735ef8220",
  "lock_sha256": "a2945dd4f7778d94f9b04292caa593c8cd3675f515b4f073230d869c1ca781b8",
  "source_sha256": "8f371c0afa998da399891b594e755f731fbf1385d7cbdb2e4ff3497fee329e93",
  "scope": "primitive-only preselection; not production/session/download performance",
  "rustflags": "",
  "order": "six rounds of ring/rustcrypto/rustcrypto11/aws-lc, alternating forward/reverse",
  "milliseconds_per_cell": 250
}
```

```jsonl
{"backend": "ring", "includes_input_restore": true, "iterations": 1474624, "ns_per_operation": 169.5392547523979, "operation": "open", "payload_bytes": 0, "seconds": 0.250006654, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1553920, "ns_per_operation": 160.88603531713343, "operation": "seal", "payload_bytes": 0, "seconds": 0.250004028, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1310592, "ns_per_operation": 190.7604792338119, "operation": "open", "payload_bytes": 64, "seconds": 0.250009158, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1340992, "ns_per_operation": 186.43391086598578, "operation": "seal", "payload_bytes": 64, "seconds": 0.250006383, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 911616, "ns_per_operation": 274.2387156434288, "operation": "open", "payload_bytes": 256, "seconds": 0.250000401, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 924352, "ns_per_operation": 270.4671413054767, "operation": "seal", "payload_bytes": 256, "seconds": 0.250006843, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 328320, "ns_per_operation": 761.554318957115, "operation": "open", "payload_bytes": 1280, "seconds": 0.250033514, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 338624, "ns_per_operation": 738.3583532177283, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250025859, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 321536, "ns_per_operation": 777.603975915605, "operation": "open", "payload_bytes": 1420, "seconds": 0.250027672, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 295488, "ns_per_operation": 846.1929621507472, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250039866, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 35008, "ns_per_operation": 7144.171875, "operation": "open", "payload_bytes": 16384, "seconds": 0.250103169, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 32832, "ns_per_operation": 7629.603801169591, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250495152, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 204544, "ns_per_operation": 1222.6150510403631, "operation": "open", "payload_bytes": 0, "seconds": 0.250078573, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 206464, "ns_per_operation": 1210.9332958772475, "operation": "seal", "payload_bytes": 0, "seconds": 0.250014132, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 139712, "ns_per_operation": 1789.9713482020159, "operation": "open", "payload_bytes": 64, "seconds": 0.250080477, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 139648, "ns_per_operation": 1790.580831805683, "operation": "seal", "payload_bytes": 64, "seconds": 0.250051032, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 135360, "ns_per_operation": 1846.9528959810877, "operation": "open", "payload_bytes": 256, "seconds": 0.250003544, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 135488, "ns_per_operation": 1845.1919653401037, "operation": "seal", "payload_bytes": 256, "seconds": 0.250001369, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 97664, "ns_per_operation": 2560.5784014580604, "operation": "open", "payload_bytes": 1280, "seconds": 0.250076329, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 97664, "ns_per_operation": 2559.8615457077326, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250006318, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 85696, "ns_per_operation": 2919.420474701269, "operation": "open", "payload_bytes": 1420, "seconds": 0.250182657, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 85888, "ns_per_operation": 2912.1358746274213, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250117526, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 19008, "ns_per_operation": 13161.447758838383, "operation": "open", "payload_bytes": 16384, "seconds": 0.250172799, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 18304, "ns_per_operation": 13690.218094405594, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250585752, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 211264, "ns_per_operation": 1183.4216572629507, "operation": "open", "payload_bytes": 0, "seconds": 0.250014393, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 211520, "ns_per_operation": 1182.0490591906205, "operation": "seal", "payload_bytes": 0, "seconds": 0.250027017, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 144320, "ns_per_operation": 1732.3919207317072, "operation": "open", "payload_bytes": 64, "seconds": 0.250018802, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 142592, "ns_per_operation": 1753.6921005385993, "operation": "seal", "payload_bytes": 64, "seconds": 0.250062464, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 139200, "ns_per_operation": 1796.1893103448276, "operation": "open", "payload_bytes": 256, "seconds": 0.250029552, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 138752, "ns_per_operation": 1801.99220912131, "operation": "seal", "payload_bytes": 256, "seconds": 0.250030023, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 99456, "ns_per_operation": 2513.778314028314, "operation": "open", "payload_bytes": 1280, "seconds": 0.250010336, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 99456, "ns_per_operation": 2514.325299629987, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250064737, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 87360, "ns_per_operation": 2862.917490842491, "operation": "open", "payload_bytes": 1420, "seconds": 0.250104472, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 87232, "ns_per_operation": 2866.0854273661043, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250014364, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 19008, "ns_per_operation": 13167.999894781144, "operation": "open", "payload_bytes": 16384, "seconds": 0.250297342, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 19072, "ns_per_operation": 13143.853030620805, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250679565, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1564352, "ns_per_operation": 159.81095878676922, "operation": "open", "payload_bytes": 0, "seconds": 0.250000593, "verified": true, "repeat": 0, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1516160, "ns_per_operation": 164.89675232165467, "operation": "seal", "payload_bytes": 0, "seconds": 0.25000986, "verified": true, "repeat": 0, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1357376, "ns_per_operation": 184.1831828469046, "operation": "open", "payload_bytes": 64, "seconds": 0.250005832, "verified": true, "repeat": 0, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1303040, "ns_per_operation": 191.86281388138508, "operation": "seal", "payload_bytes": 64, "seconds": 0.250004921, "verified": true, "repeat": 0, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 914688, "ns_per_operation": 273.3204732105374, "operation": "open", "payload_bytes": 256, "seconds": 0.250002957, "verified": true, "repeat": 0, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 900032, "ns_per_operation": 277.77596241022536, "operation": "seal", "payload_bytes": 256, "seconds": 0.250007255, "verified": true, "repeat": 0, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 331328, "ns_per_operation": 754.5972872802781, "operation": "open", "payload_bytes": 1280, "seconds": 0.25001921, "verified": true, "repeat": 0, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 335296, "ns_per_operation": 745.67948916778, "operation": "seal", "payload_bytes": 1280, "seconds": 0.25002335, "verified": true, "repeat": 0, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 323776, "ns_per_operation": 772.203918758648, "operation": "open", "payload_bytes": 1420, "seconds": 0.250021096, "verified": true, "repeat": 0, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 293248, "ns_per_operation": 852.6683796649935, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250043297, "verified": true, "repeat": 0, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 34944, "ns_per_operation": 7160.313873626373, "operation": "open", "payload_bytes": 16384, "seconds": 0.250210008, "verified": true, "repeat": 0, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 34432, "ns_per_operation": 7272.561280204462, "operation": "seal", "payload_bytes": 16384, "seconds": 0.25040883, "verified": true, "repeat": 0, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1563904, "ns_per_operation": 159.85693367367818, "operation": "open", "payload_bytes": 0, "seconds": 0.250000898, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1516032, "ns_per_operation": 164.9051794421226, "operation": "seal", "payload_bytes": 0, "seconds": 0.250001529, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1354624, "ns_per_operation": 184.55571656784468, "operation": "open", "payload_bytes": 64, "seconds": 0.250003603, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1307456, "ns_per_operation": 191.21869110700476, "operation": "seal", "payload_bytes": 64, "seconds": 0.250010025, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 943040, "ns_per_operation": 265.11311185103494, "operation": "open", "payload_bytes": 256, "seconds": 0.250012269, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 897856, "ns_per_operation": 278.45063350915956, "operation": "seal", "payload_bytes": 256, "seconds": 0.250008572, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 329984, "ns_per_operation": 757.6728689875873, "operation": "open", "payload_bytes": 1280, "seconds": 0.250019924, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 333824, "ns_per_operation": 748.97375563171, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250025415, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 324480, "ns_per_operation": 770.5808185404339, "operation": "open", "payload_bytes": 1420, "seconds": 0.250038064, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 292992, "ns_per_operation": 853.3897205384446, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250036361, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 34944, "ns_per_operation": 7159.531965430404, "operation": "open", "payload_bytes": 16384, "seconds": 0.250182685, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 34560, "ns_per_operation": 7244.895862268519, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250383601, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 210944, "ns_per_operation": 1185.497909397755, "operation": "open", "payload_bytes": 0, "seconds": 0.250073671, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 212288, "ns_per_operation": 1177.789375753693, "operation": "seal", "payload_bytes": 0, "seconds": 0.250030551, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 144704, "ns_per_operation": 1728.4515009951347, "operation": "open", "payload_bytes": 64, "seconds": 0.250113846, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 142208, "ns_per_operation": 1758.4236400202522, "operation": "seal", "payload_bytes": 64, "seconds": 0.250061909, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 138944, "ns_per_operation": 1799.7838913519113, "operation": "open", "payload_bytes": 256, "seconds": 0.250069173, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 138624, "ns_per_operation": 1804.2798577446906, "operation": "seal", "payload_bytes": 256, "seconds": 0.250116491, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 99456, "ns_per_operation": 2514.5971484877737, "operation": "open", "payload_bytes": 1280, "seconds": 0.250091774, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 99264, "ns_per_operation": 2519.541928594455, "operation": "seal", "payload_bytes": 1280, "seconds": 0.25009981, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 87296, "ns_per_operation": 2865.190810575513, "operation": "open", "payload_bytes": 1420, "seconds": 0.250119697, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 87104, "ns_per_operation": 2871.700599283615, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250136609, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 19136, "ns_per_operation": 13085.548756270902, "operation": "open", "payload_bytes": 16384, "seconds": 0.250405061, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 19008, "ns_per_operation": 13153.520833333334, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250022124, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 204416, "ns_per_operation": 1223.2304809799625, "operation": "open", "payload_bytes": 0, "seconds": 0.250047882, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 206848, "ns_per_operation": 1208.9164458926361, "operation": "seal", "payload_bytes": 0, "seconds": 0.250061949, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 139776, "ns_per_operation": 1788.6500114468865, "operation": "open", "payload_bytes": 64, "seconds": 0.250010344, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 139648, "ns_per_operation": 1790.6535575160403, "operation": "seal", "payload_bytes": 64, "seconds": 0.250061188, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 135488, "ns_per_operation": 1845.5810477680677, "operation": "open", "payload_bytes": 256, "seconds": 0.250054085, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 135680, "ns_per_operation": 1843.1943691037734, "operation": "seal", "payload_bytes": 256, "seconds": 0.250084612, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 97472, "ns_per_operation": 2565.144143959291, "operation": "open", "payload_bytes": 1280, "seconds": 0.25002973, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 97664, "ns_per_operation": 2560.8913622214945, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250106894, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 85888, "ns_per_operation": 2912.5863450074517, "operation": "open", "payload_bytes": 1420, "seconds": 0.250156216, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 86080, "ns_per_operation": 2905.0576092007436, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250067359, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 19008, "ns_per_operation": 13156.098274410773, "operation": "open", "payload_bytes": 16384, "seconds": 0.250071116, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 18944, "ns_per_operation": 13199.563555743243, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250052532, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1480000, "ns_per_operation": 168.9202554054054, "operation": "open", "payload_bytes": 0, "seconds": 0.250001978, "verified": true, "repeat": 1, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1561088, "ns_per_operation": 160.14909857740244, "operation": "seal", "payload_bytes": 0, "seconds": 0.250006836, "verified": true, "repeat": 1, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1316544, "ns_per_operation": 189.89508364323564, "operation": "open", "payload_bytes": 64, "seconds": 0.250005233, "verified": true, "repeat": 1, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1372288, "ns_per_operation": 182.17957090639862, "operation": "seal", "payload_bytes": 64, "seconds": 0.250002839, "verified": true, "repeat": 1, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 909632, "ns_per_operation": 274.8463015812988, "operation": "open", "payload_bytes": 256, "seconds": 0.250008991, "verified": true, "repeat": 1, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 928000, "ns_per_operation": 269.411119612069, "operation": "seal", "payload_bytes": 256, "seconds": 0.250013519, "verified": true, "repeat": 1, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 329344, "ns_per_operation": 759.1305200641275, "operation": "open", "payload_bytes": 1280, "seconds": 0.250015082, "verified": true, "repeat": 1, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 338624, "ns_per_operation": 738.3293653137404, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250016043, "verified": true, "repeat": 1, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 321536, "ns_per_operation": 777.548647740844, "operation": "open", "payload_bytes": 1420, "seconds": 0.250009882, "verified": true, "repeat": 1, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 295488, "ns_per_operation": 846.1054594433615, "operation": "seal", "payload_bytes": 1420, "seconds": 0.25001401, "verified": true, "repeat": 1, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 34880, "ns_per_operation": 7175.627264908257, "operation": "open", "payload_bytes": 16384, "seconds": 0.250285879, "verified": true, "repeat": 1, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 32768, "ns_per_operation": 7637.482879638673, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250265039, "verified": true, "repeat": 1, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1480512, "ns_per_operation": 168.86139119439758, "operation": "open", "payload_bytes": 0, "seconds": 0.250001316, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1559872, "ns_per_operation": 160.27355770217045, "operation": "seal", "payload_bytes": 0, "seconds": 0.250006235, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1319104, "ns_per_operation": 189.52484337853573, "operation": "open", "payload_bytes": 64, "seconds": 0.250002979, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1372096, "ns_per_operation": 182.21146698190216, "operation": "seal", "payload_bytes": 64, "seconds": 0.250011625, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 910912, "ns_per_operation": 274.45555992236353, "operation": "open", "payload_bytes": 256, "seconds": 0.250004863, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 928192, "ns_per_operation": 269.3440355012756, "operation": "seal", "payload_bytes": 256, "seconds": 0.250002979, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 329600, "ns_per_operation": 758.6041292475728, "operation": "open", "payload_bytes": 1280, "seconds": 0.250035921, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 335808, "ns_per_operation": 744.4852475223937, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250004102, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 321408, "ns_per_operation": 777.8309127339705, "operation": "open", "payload_bytes": 1420, "seconds": 0.250001078, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 296000, "ns_per_operation": 844.6319763513512, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250011065, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 35136, "ns_per_operation": 7126.004667577415, "operation": "open", "payload_bytes": 16384, "seconds": 0.2503793, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 32832, "ns_per_operation": 7624.582480506822, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250330292, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 202688, "ns_per_operation": 1233.687810822545, "operation": "open", "payload_bytes": 0, "seconds": 0.250053715, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 205248, "ns_per_operation": 1218.0557228328657, "operation": "seal", "payload_bytes": 0, "seconds": 0.250003501, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 139072, "ns_per_operation": 1797.9182006442707, "operation": "open", "payload_bytes": 64, "seconds": 0.25004008, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 138944, "ns_per_operation": 1799.3581874712115, "operation": "seal", "payload_bytes": 64, "seconds": 0.250010024, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 134848, "ns_per_operation": 1854.3824602515426, "operation": "open", "payload_bytes": 256, "seconds": 0.250059766, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 135360, "ns_per_operation": 1847.708348108747, "operation": "seal", "payload_bytes": 256, "seconds": 0.250105802, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 97216, "ns_per_operation": 2571.9470971856485, "operation": "open", "payload_bytes": 1280, "seconds": 0.250034409, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 97536, "ns_per_operation": 2564.3415764435695, "operation": "seal", "payload_bytes": 1280, "seconds": 0.25011562, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 85632, "ns_per_operation": 2921.539331091181, "operation": "open", "payload_bytes": 1420, "seconds": 0.250177256, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 85760, "ns_per_operation": 2916.370172574627, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250107906, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 19072, "ns_per_operation": 13145.032875419462, "operation": "open", "payload_bytes": 16384, "seconds": 0.250702067, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 18944, "ns_per_operation": 13207.880437077703, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250210087, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 211264, "ns_per_operation": 1183.5952599591033, "operation": "open", "payload_bytes": 0, "seconds": 0.250051069, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 211968, "ns_per_operation": 1179.5638964371983, "operation": "seal", "payload_bytes": 0, "seconds": 0.2500298, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 144704, "ns_per_operation": 1728.4763655462186, "operation": "open", "payload_bytes": 64, "seconds": 0.250117444, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 142400, "ns_per_operation": 1756.3564676966294, "operation": "seal", "payload_bytes": 64, "seconds": 0.250105161, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 139008, "ns_per_operation": 1798.5613202117866, "operation": "open", "payload_bytes": 256, "seconds": 0.250014412, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 138560, "ns_per_operation": 1804.7400837182445, "operation": "seal", "payload_bytes": 256, "seconds": 0.250064786, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 99456, "ns_per_operation": 2514.4466698841698, "operation": "open", "payload_bytes": 1280, "seconds": 0.250076808, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 99392, "ns_per_operation": 2516.3062620734063, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250100712, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 87360, "ns_per_operation": 2862.1940361721613, "operation": "open", "payload_bytes": 1420, "seconds": 0.250041271, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 87104, "ns_per_operation": 2871.011710139603, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250076604, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 19136, "ns_per_operation": 13104.336015886289, "operation": "open", "payload_bytes": 16384, "seconds": 0.250764574, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 19008, "ns_per_operation": 13154.386942340067, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250038587, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1564160, "ns_per_operation": 159.8305838277414, "operation": "open", "payload_bytes": 0, "seconds": 0.250000606, "verified": true, "repeat": 2, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1515264, "ns_per_operation": 164.98975096088867, "operation": "seal", "payload_bytes": 0, "seconds": 0.25000303, "verified": true, "repeat": 2, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1357248, "ns_per_operation": 184.20382568255764, "operation": "open", "payload_bytes": 64, "seconds": 0.250010274, "verified": true, "repeat": 2, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1301504, "ns_per_operation": 192.09130283118608, "operation": "seal", "payload_bytes": 64, "seconds": 0.250007599, "verified": true, "repeat": 2, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 942592, "ns_per_operation": 265.2358411698805, "operation": "open", "payload_bytes": 256, "seconds": 0.250009182, "verified": true, "repeat": 2, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 905856, "ns_per_operation": 275.9963824272291, "operation": "seal", "payload_bytes": 256, "seconds": 0.250012979, "verified": true, "repeat": 2, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 331328, "ns_per_operation": 754.6453272889705, "operation": "open", "payload_bytes": 1280, "seconds": 0.250035127, "verified": true, "repeat": 2, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 335296, "ns_per_operation": 745.6601689253674, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250016872, "verified": true, "repeat": 2, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 324608, "ns_per_operation": 770.1701344390774, "operation": "open", "payload_bytes": 1420, "seconds": 0.250003387, "verified": true, "repeat": 2, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 292864, "ns_per_operation": 853.6844610467656, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250013446, "verified": true, "repeat": 2, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 34816, "ns_per_operation": 7192.584731158087, "operation": "open", "payload_bytes": 16384, "seconds": 0.25041703, "verified": true, "repeat": 2, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 34432, "ns_per_operation": 7262.400412407064, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250058971, "verified": true, "repeat": 2, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1563648, "ns_per_operation": 159.88374877210217, "operation": "open", "payload_bytes": 0, "seconds": 0.250001904, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1516032, "ns_per_operation": 164.908373965721, "operation": "seal", "payload_bytes": 0, "seconds": 0.250006372, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1357376, "ns_per_operation": 184.18251022561176, "operation": "open", "payload_bytes": 64, "seconds": 0.250004919, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1296320, "ns_per_operation": 192.85950536904465, "operation": "seal", "payload_bytes": 64, "seconds": 0.250007634, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 942848, "ns_per_operation": 265.16872921191964, "operation": "open", "payload_bytes": 256, "seconds": 0.250013806, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 907456, "ns_per_operation": 275.50449388179703, "operation": "seal", "payload_bytes": 256, "seconds": 0.250008206, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 331456, "ns_per_operation": 754.3374263854025, "operation": "open", "payload_bytes": 1280, "seconds": 0.250029666, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 334848, "ns_per_operation": 746.7008284355887, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250031279, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 324992, "ns_per_operation": 769.3475531705396, "operation": "open", "payload_bytes": 1420, "seconds": 0.2500318, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 293248, "ns_per_operation": 852.569797577477, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250014388, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 34624, "ns_per_operation": 7222.860674676525, "operation": "open", "payload_bytes": 16384, "seconds": 0.250084328, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 34624, "ns_per_operation": 7228.155932301294, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250267671, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 211136, "ns_per_operation": 1184.0912871324645, "operation": "open", "payload_bytes": 0, "seconds": 0.250004298, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 212096, "ns_per_operation": 1178.9465289302957, "operation": "seal", "payload_bytes": 0, "seconds": 0.250049843, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 144512, "ns_per_operation": 1730.0142894707708, "operation": "open", "payload_bytes": 64, "seconds": 0.250007825, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 142400, "ns_per_operation": 1755.7345575842696, "operation": "seal", "payload_bytes": 64, "seconds": 0.250016601, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 138880, "ns_per_operation": 1800.170470910138, "operation": "open", "payload_bytes": 256, "seconds": 0.250007675, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 138624, "ns_per_operation": 1804.0043354686056, "operation": "seal", "payload_bytes": 256, "seconds": 0.250078297, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 99584, "ns_per_operation": 2511.461178502571, "operation": "open", "payload_bytes": 1280, "seconds": 0.25010135, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 99328, "ns_per_operation": 2517.184187741623, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250026871, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 87360, "ns_per_operation": 2863.6765109890107, "operation": "open", "payload_bytes": 1420, "seconds": 0.25017078, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 87232, "ns_per_operation": 2866.7661981841525, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250073749, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 18944, "ns_per_operation": 13219.677576013513, "operation": "open", "payload_bytes": 16384, "seconds": 0.250433572, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 18752, "ns_per_operation": 13378.512958617748, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250873875, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 202816, "ns_per_operation": 1232.6953987851057, "operation": "open", "payload_bytes": 0, "seconds": 0.25001035, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 205376, "ns_per_operation": 1217.4689545029605, "operation": "seal", "payload_bytes": 0, "seconds": 0.250038904, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 138624, "ns_per_operation": 1803.7868262349955, "operation": "open", "payload_bytes": 64, "seconds": 0.250048145, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 139008, "ns_per_operation": 1798.6348627417126, "operation": "seal", "payload_bytes": 64, "seconds": 0.250024635, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 134720, "ns_per_operation": 1856.506843824228, "operation": "open", "payload_bytes": 256, "seconds": 0.250108602, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 135488, "ns_per_operation": 1845.4919697685402, "operation": "seal", "payload_bytes": 256, "seconds": 0.250042016, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 97216, "ns_per_operation": 2572.0043305628706, "operation": "open", "payload_bytes": 1280, "seconds": 0.250039973, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 97600, "ns_per_operation": 2563.0442110655736, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250153115, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 85568, "ns_per_operation": 2922.724499813014, "operation": "open", "payload_bytes": 1420, "seconds": 0.25009169, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 85760, "ns_per_operation": 2915.881658115671, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250066011, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 19072, "ns_per_operation": 13147.55201342282, "operation": "open", "payload_bytes": 16384, "seconds": 0.250750112, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 18944, "ns_per_operation": 13205.095650337838, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250157332, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1480768, "ns_per_operation": 168.8323228216709, "operation": "open", "payload_bytes": 0, "seconds": 0.250001501, "verified": true, "repeat": 3, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1559360, "ns_per_operation": 160.3231524471578, "operation": "seal", "payload_bytes": 0, "seconds": 0.250001511, "verified": true, "repeat": 3, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1324544, "ns_per_operation": 188.74716506208932, "operation": "open", "payload_bytes": 64, "seconds": 0.250003925, "verified": true, "repeat": 3, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1369792, "ns_per_operation": 182.51237122132412, "operation": "seal", "payload_bytes": 64, "seconds": 0.250003986, "verified": true, "repeat": 3, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 910336, "ns_per_operation": 274.6341405810602, "operation": "open", "payload_bytes": 256, "seconds": 0.250009345, "verified": true, "repeat": 3, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 927616, "ns_per_operation": 269.51869847005656, "operation": "seal", "payload_bytes": 256, "seconds": 0.250009857, "verified": true, "repeat": 3, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 330048, "ns_per_operation": 757.4907407407408, "operation": "open", "payload_bytes": 1280, "seconds": 0.250008304, "verified": true, "repeat": 3, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 338368, "ns_per_operation": 738.9675412568564, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250042969, "verified": true, "repeat": 3, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 321728, "ns_per_operation": 777.1878387955043, "operation": "open", "payload_bytes": 1420, "seconds": 0.250043089, "verified": true, "repeat": 3, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 295744, "ns_per_operation": 845.379919795499, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250016039, "verified": true, "repeat": 3, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 34752, "ns_per_operation": 7202.419889502761, "operation": "open", "payload_bytes": 16384, "seconds": 0.250298496, "verified": true, "repeat": 3, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 32768, "ns_per_operation": 7629.731414794922, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250011039, "verified": true, "repeat": 3, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1480128, "ns_per_operation": 168.906083798158, "operation": "open", "payload_bytes": 0, "seconds": 0.250002624, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1559744, "ns_per_operation": 160.28887176357145, "operation": "seal", "payload_bytes": 0, "seconds": 0.250009606, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1321856, "ns_per_operation": 189.13522879950614, "operation": "open", "payload_bytes": 64, "seconds": 0.250009537, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1373056, "ns_per_operation": 182.079644967139, "operation": "seal", "payload_bytes": 64, "seconds": 0.250005549, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 910848, "ns_per_operation": 274.4879387120573, "operation": "open", "payload_bytes": 256, "seconds": 0.25001679, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 928448, "ns_per_operation": 269.28091072413315, "operation": "seal", "payload_bytes": 256, "seconds": 0.250013323, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 329344, "ns_per_operation": 759.2011422706956, "operation": "open", "payload_bytes": 1280, "seconds": 0.250038341, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 338496, "ns_per_operation": 738.6306160190961, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250023509, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 321792, "ns_per_operation": 777.0102177804296, "operation": "open", "payload_bytes": 1420, "seconds": 0.250035672, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 295872, "ns_per_operation": 844.9695172236643, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250002821, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 34944, "ns_per_operation": 7160.464142628206, "operation": "open", "payload_bytes": 16384, "seconds": 0.250215259, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 34688, "ns_per_operation": 7209.124135147602, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250070098, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 204288, "ns_per_operation": 1223.9136170504387, "operation": "open", "payload_bytes": 0, "seconds": 0.250030865, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 206656, "ns_per_operation": 1209.9878251780738, "operation": "seal", "payload_bytes": 0, "seconds": 0.250051244, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 139712, "ns_per_operation": 1789.6752676935407, "operation": "open", "payload_bytes": 64, "seconds": 0.250039111, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 139584, "ns_per_operation": 1791.1300865428702, "operation": "seal", "payload_bytes": 64, "seconds": 0.250013102, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 135104, "ns_per_operation": 1850.8126702392228, "operation": "open", "payload_bytes": 256, "seconds": 0.250052195, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 135616, "ns_per_operation": 1843.8949091552622, "operation": "seal", "payload_bytes": 256, "seconds": 0.250061652, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 97728, "ns_per_operation": 2559.2960359364765, "operation": "open", "payload_bytes": 1280, "seconds": 0.250114883, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 97600, "ns_per_operation": 2562.8819057377054, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250137274, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 85888, "ns_per_operation": 2912.0761573211626, "operation": "open", "payload_bytes": 1420, "seconds": 0.250112397, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 85952, "ns_per_operation": 2909.8505910275503, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250107478, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 19008, "ns_per_operation": 13163.713226010099, "operation": "open", "payload_bytes": 16384, "seconds": 0.250215861, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 18944, "ns_per_operation": 13222.521062077702, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250487439, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 211136, "ns_per_operation": 1184.3159574871174, "operation": "open", "payload_bytes": 0, "seconds": 0.250051734, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 212096, "ns_per_operation": 1178.8572863231745, "operation": "seal", "payload_bytes": 0, "seconds": 0.250030915, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 144512, "ns_per_operation": 1730.5282675487156, "operation": "open", "payload_bytes": 64, "seconds": 0.250082101, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 142400, "ns_per_operation": 1755.9074789325841, "operation": "seal", "payload_bytes": 64, "seconds": 0.250041225, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 139072, "ns_per_operation": 1798.3186191325358, "operation": "open", "payload_bytes": 256, "seconds": 0.250095767, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 138624, "ns_per_operation": 1803.572051015697, "operation": "seal", "payload_bytes": 256, "seconds": 0.250018372, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 99456, "ns_per_operation": 2514.6823218307595, "operation": "open", "payload_bytes": 1280, "seconds": 0.250100245, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 99136, "ns_per_operation": 2522.358104018722, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250056493, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 87360, "ns_per_operation": 2863.4488324175823, "operation": "open", "payload_bytes": 1420, "seconds": 0.25015089, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 87232, "ns_per_operation": 2866.9699078319877, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250091519, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 18880, "ns_per_operation": 13255.821080508475, "operation": "open", "payload_bytes": 16384, "seconds": 0.250269902, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 19072, "ns_per_operation": 13149.761377936244, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250792249, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1562432, "ns_per_operation": 160.01305976836113, "operation": "open", "payload_bytes": 0, "seconds": 0.250009525, "verified": true, "repeat": 4, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1515392, "ns_per_operation": 164.97495037587632, "operation": "seal", "payload_bytes": 0, "seconds": 0.25000172, "verified": true, "repeat": 4, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1356032, "ns_per_operation": 184.36958641094014, "operation": "open", "payload_bytes": 64, "seconds": 0.250011059, "verified": true, "repeat": 4, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1301568, "ns_per_operation": 192.07894247553722, "operation": "seal", "payload_bytes": 64, "seconds": 0.250003805, "verified": true, "repeat": 4, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 942144, "ns_per_operation": 265.3578295886829, "operation": "open", "payload_bytes": 256, "seconds": 0.250005287, "verified": true, "repeat": 4, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 908608, "ns_per_operation": 275.1514250369796, "operation": "seal", "payload_bytes": 256, "seconds": 0.250004786, "verified": true, "repeat": 4, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 332352, "ns_per_operation": 752.3788483294819, "operation": "open", "payload_bytes": 1280, "seconds": 0.250054615, "verified": true, "repeat": 4, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 333760, "ns_per_operation": 749.0453889022052, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250001389, "verified": true, "repeat": 4, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 324672, "ns_per_operation": 770.1458086930811, "operation": "open", "payload_bytes": 1420, "seconds": 0.25004478, "verified": true, "repeat": 4, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 292416, "ns_per_operation": 855.1025662070475, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250045672, "verified": true, "repeat": 4, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 35072, "ns_per_operation": 7137.021184990876, "operation": "open", "payload_bytes": 16384, "seconds": 0.250309607, "verified": true, "repeat": 4, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 34112, "ns_per_operation": 7339.407891651031, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250361882, "verified": true, "repeat": 4, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1563584, "ns_per_operation": 159.89515050038887, "operation": "open", "payload_bytes": 0, "seconds": 0.250009499, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1517824, "ns_per_operation": 164.70991564239333, "operation": "seal", "payload_bytes": 0, "seconds": 0.250000663, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1360832, "ns_per_operation": 183.71587602290364, "operation": "open", "payload_bytes": 64, "seconds": 0.250006443, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1303616, "ns_per_operation": 191.7791842076194, "operation": "seal", "payload_bytes": 64, "seconds": 0.250006413, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 942272, "ns_per_operation": 265.3217446766963, "operation": "open", "payload_bytes": 256, "seconds": 0.250005251, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 907840, "ns_per_operation": 275.3885133944308, "operation": "seal", "payload_bytes": 256, "seconds": 0.250008708, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 332224, "ns_per_operation": 752.552741523791, "operation": "open", "payload_bytes": 1280, "seconds": 0.250016082, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 332928, "ns_per_operation": 750.9206074586698, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250002496, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 324416, "ns_per_operation": 770.6338836555534, "operation": "open", "payload_bytes": 1420, "seconds": 0.250005962, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 292864, "ns_per_operation": 853.8046772563374, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250048653, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 35008, "ns_per_operation": 7146.677845063985, "operation": "open", "payload_bytes": 16384, "seconds": 0.250190898, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 32512, "ns_per_operation": 7691.551365649606, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250067718, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 211072, "ns_per_operation": 1184.7557705427532, "operation": "open", "payload_bytes": 0, "seconds": 0.25006877, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 212032, "ns_per_operation": 1179.2730484077874, "operation": "seal", "payload_bytes": 0, "seconds": 0.250043623, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 144704, "ns_per_operation": 1728.285665911101, "operation": "open", "payload_bytes": 64, "seconds": 0.250089849, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 142272, "ns_per_operation": 1757.5683198380564, "operation": "seal", "payload_bytes": 64, "seconds": 0.25005276, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 139136, "ns_per_operation": 1797.3923714926402, "operation": "open", "payload_bytes": 256, "seconds": 0.250081985, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 138752, "ns_per_operation": 1802.5826726821956, "operation": "seal", "payload_bytes": 256, "seconds": 0.250111951, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 99584, "ns_per_operation": 2511.4617910507714, "operation": "open", "payload_bytes": 1280, "seconds": 0.250101411, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 99200, "ns_per_operation": 2521.124405241935, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250095541, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 87296, "ns_per_operation": 2863.982519244868, "operation": "open", "payload_bytes": 1420, "seconds": 0.250014218, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 87168, "ns_per_operation": 2868.956314243759, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250081184, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 19136, "ns_per_operation": 13080.96493520067, "operation": "open", "payload_bytes": 16384, "seconds": 0.250317345, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 18944, "ns_per_operation": 13198.065139358108, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250024146, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 204288, "ns_per_operation": 1223.8702811716794, "operation": "open", "payload_bytes": 0, "seconds": 0.250022012, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 206592, "ns_per_operation": 1210.3290737298637, "operation": "seal", "payload_bytes": 0, "seconds": 0.250044304, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 139328, "ns_per_operation": 1794.7982745751035, "operation": "open", "payload_bytes": 64, "seconds": 0.250065654, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 139264, "ns_per_operation": 1795.503568761489, "operation": "seal", "payload_bytes": 64, "seconds": 0.250049009, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 135296, "ns_per_operation": 1848.336410536897, "operation": "open", "payload_bytes": 256, "seconds": 0.250072523, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 136000, "ns_per_operation": 1838.7885882352944, "operation": "seal", "payload_bytes": 256, "seconds": 0.250075248, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 97600, "ns_per_operation": 2563.07006147541, "operation": "open", "payload_bytes": 1280, "seconds": 0.250155638, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 97600, "ns_per_operation": 2562.3603381147536, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250086369, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 85824, "ns_per_operation": 2914.195679530201, "operation": "open", "payload_bytes": 1420, "seconds": 0.25010793, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 85888, "ns_per_operation": 2911.1019117921014, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250028721, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 19072, "ns_per_operation": 13135.769714765102, "operation": "open", "payload_bytes": 16384, "seconds": 0.2505254, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 18624, "ns_per_operation": 13454.527867268042, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250577127, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1480448, "ns_per_operation": 168.8680102239322, "operation": "open", "payload_bytes": 0, "seconds": 0.250000308, "verified": true, "repeat": 5, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1559616, "ns_per_operation": 160.2971122378842, "operation": "seal", "payload_bytes": 0, "seconds": 0.250001941, "verified": true, "repeat": 5, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1323264, "ns_per_operation": 188.93351893499712, "operation": "open", "payload_bytes": 64, "seconds": 0.250008924, "verified": true, "repeat": 5, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1371008, "ns_per_operation": 182.35527217930164, "operation": "seal", "payload_bytes": 64, "seconds": 0.250010537, "verified": true, "repeat": 5, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 910016, "ns_per_operation": 274.7394990857303, "operation": "open", "payload_bytes": 256, "seconds": 0.25001734, "verified": true, "repeat": 5, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 927552, "ns_per_operation": 269.53369730214587, "operation": "seal", "payload_bytes": 256, "seconds": 0.25000652, "verified": true, "repeat": 5, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 330624, "ns_per_operation": 756.1811574477352, "operation": "open", "payload_bytes": 1280, "seconds": 0.250011639, "verified": true, "repeat": 5, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 337088, "ns_per_operation": 741.7662242500473, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250040493, "verified": true, "repeat": 5, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 321728, "ns_per_operation": 777.1415916550626, "operation": "open", "payload_bytes": 1420, "seconds": 0.25002821, "verified": true, "repeat": 5, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 295744, "ns_per_operation": 845.3746990640554, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250014495, "verified": true, "repeat": 5, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 35136, "ns_per_operation": 7123.347506830601, "operation": "open", "payload_bytes": 16384, "seconds": 0.250285938, "verified": true, "repeat": 5, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 34560, "ns_per_operation": 7238.034924768518, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250146487, "verified": true, "repeat": 5, "position": 3, "cpu": 2}
```

## Current four-backend primitive screen: arm

[Run](https://github.com/riKirsti/ProtunNext/actions/runs/43157895797) / [artifact](https://github.com/riKirsti/ProtunNext/actions/runs/43157895797/artifacts/13916244240)

```json
{
  "kernel": "7.0.0-1017-azure",
  "machine": "aarch64",
  "original_allowed_cpus": [
    0,
    1,
    2,
    3
  ],
  "benchmark_cpu": 2,
  "compiler": "rustc 1.99.1 (58cf8f9c3 2026-10-13)\nbinary: rustc\ncommit-hash: 58cf8f9c35b355199727627d07ae34a4e2846d67\ncommit-date: 2026-10-13\nhost: aarch64-unknown-linux-gnu\nrelease: 1.99.1\nLLVM version: 23.1.1\n",
  "commit": "0b62c99f0939b988ad14583f6f725dc1d30276e5",
  "binary_sha256": "65ab8f36dcfd9e69103b3651b5d08a54037545a7bbe2cf80bb19d9de797a6ab9",
  "manifest_sha256": "72e4b85e6da0c806f16b33667ad36a93c172229e8404ff2e9cb5009735ef8220",
  "lock_sha256": "a2945dd4f7778d94f9b04292caa593c8cd3675f515b4f073230d869c1ca781b8",
  "source_sha256": "8f371c0afa998da399891b594e755f731fbf1385d7cbdb2e4ff3497fee329e93",
  "scope": "primitive-only preselection; not production/session/download performance",
  "rustflags": "",
  "order": "six rounds of ring/rustcrypto/rustcrypto11/aws-lc, alternating forward/reverse",
  "milliseconds_per_cell": 250
}
```

```jsonl
{"backend": "ring", "includes_input_restore": true, "iterations": 1045504, "ns_per_operation": 239.13014201762977, "operation": "open", "payload_bytes": 0, "seconds": 0.25001152, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1108480, "ns_per_operation": 225.53518331408776, "operation": "seal", "payload_bytes": 0, "seconds": 0.25000124, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 977280, "ns_per_operation": 255.8140307793058, "operation": "open", "payload_bytes": 64, "seconds": 0.250001936, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1013248, "ns_per_operation": 246.74237304194034, "operation": "seal", "payload_bytes": 64, "seconds": 0.250011216, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 517888, "ns_per_operation": 482.77214957674244, "operation": "open", "payload_bytes": 256, "seconds": 0.250021903, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 657280, "ns_per_operation": 380.3924157132424, "operation": "seal", "payload_bytes": 256, "seconds": 0.250024327, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 190656, "ns_per_operation": 1311.2701042715676, "operation": "open", "payload_bytes": 1280, "seconds": 0.250001513, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 176000, "ns_per_operation": 1420.8317102272727, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250066381, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 161792, "ns_per_operation": 1545.5221333564082, "operation": "open", "payload_bytes": 1420, "seconds": 0.250053117, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 168192, "ns_per_operation": 1486.4136760369101, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250002889, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 17280, "ns_per_operation": 14504.761516203705, "operation": "open", "payload_bytes": 16384, "seconds": 0.250642279, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 17344, "ns_per_operation": 14451.47918588561, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250646455, "verified": true, "repeat": 0, "position": 0, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 1304000, "ns_per_operation": 191.71782898773006, "operation": "open", "payload_bytes": 0, "seconds": 0.250000049, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 1395712, "ns_per_operation": 179.12571648019076, "operation": "seal", "payload_bytes": 0, "seconds": 0.250007912, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 752192, "ns_per_operation": 332.36688239172975, "operation": "open", "payload_bytes": 64, "seconds": 0.25000371, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 766464, "ns_per_operation": 326.19710253841015, "operation": "seal", "payload_bytes": 64, "seconds": 0.250018336, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 323264, "ns_per_operation": 773.4370143288456, "operation": "open", "payload_bytes": 256, "seconds": 0.250024343, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 326272, "ns_per_operation": 766.2558846606512, "operation": "seal", "payload_bytes": 256, "seconds": 0.25000784, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 79936, "ns_per_operation": 3127.5139236389114, "operation": "open", "payload_bytes": 1280, "seconds": 0.250000953, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 80256, "ns_per_operation": 3116.67254784689, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250131672, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 70528, "ns_per_operation": 3547.5394169691476, "operation": "open", "payload_bytes": 1420, "seconds": 0.25020086, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 70848, "ns_per_operation": 3530.165001129178, "operation": "seal", "payload_bytes": 1420, "seconds": 0.25010513, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 6656, "ns_per_operation": 37822.39633413461, "operation": "open", "payload_bytes": 16384, "seconds": 0.25174587, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 6656, "ns_per_operation": 37657.37469951923, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250647486, "verified": true, "repeat": 0, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 910080, "ns_per_operation": 274.70769163150493, "operation": "open", "payload_bytes": 0, "seconds": 0.250005976, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 930112, "ns_per_operation": 268.79649010011696, "operation": "seal", "payload_bytes": 0, "seconds": 0.250010841, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 504256, "ns_per_operation": 495.8011168930067, "operation": "open", "payload_bytes": 64, "seconds": 0.250010688, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 499456, "ns_per_operation": 500.59080079126085, "operation": "seal", "payload_bytes": 64, "seconds": 0.250023079, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 213824, "ns_per_operation": 1169.3245987354085, "operation": "open", "payload_bytes": 256, "seconds": 0.250029663, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 213504, "ns_per_operation": 1171.1715986585732, "operation": "seal", "payload_bytes": 256, "seconds": 0.250049821, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 85888, "ns_per_operation": 2912.053488263785, "operation": "open", "payload_bytes": 1280, "seconds": 0.25011045, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 86272, "ns_per_operation": 2899.7979993508898, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250171373, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 69824, "ns_per_operation": 3581.2181055224564, "operation": "open", "payload_bytes": 1420, "seconds": 0.250054973, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 70144, "ns_per_operation": 3564.6688811587596, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250040134, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 9024, "ns_per_operation": 27890.007313829785, "operation": "open", "payload_bytes": 16384, "seconds": 0.251679426, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 8960, "ns_per_operation": 27933.620312500003, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250285238, "verified": true, "repeat": 0, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1090560, "ns_per_operation": 229.2402811399648, "operation": "open", "payload_bytes": 0, "seconds": 0.250000281, "verified": true, "repeat": 0, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1081152, "ns_per_operation": 231.24658882377318, "operation": "seal", "payload_bytes": 0, "seconds": 0.250012712, "verified": true, "repeat": 0, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 999168, "ns_per_operation": 250.22088077280299, "operation": "open", "payload_bytes": 64, "seconds": 0.250012697, "verified": true, "repeat": 0, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 982272, "ns_per_operation": 254.51659825384414, "operation": "seal", "payload_bytes": 64, "seconds": 0.250004528, "verified": true, "repeat": 0, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 524480, "ns_per_operation": 476.6660921293471, "operation": "open", "payload_bytes": 256, "seconds": 0.250001832, "verified": true, "repeat": 0, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 648512, "ns_per_operation": 385.5047369979276, "operation": "seal", "payload_bytes": 256, "seconds": 0.250004448, "verified": true, "repeat": 0, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 191360, "ns_per_operation": 1306.4606657608695, "operation": "open", "payload_bytes": 1280, "seconds": 0.250004313, "verified": true, "repeat": 0, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 175232, "ns_per_operation": 1427.1008719868516, "operation": "seal", "payload_bytes": 1280, "seconds": 0.25007374, "verified": true, "repeat": 0, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 162368, "ns_per_operation": 1539.9162334942848, "operation": "open", "payload_bytes": 1420, "seconds": 0.250033119, "verified": true, "repeat": 0, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 167552, "ns_per_operation": 1492.1174620416346, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250007265, "verified": true, "repeat": 0, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 17280, "ns_per_operation": 14510.456423611113, "operation": "open", "payload_bytes": 16384, "seconds": 0.250740687, "verified": true, "repeat": 0, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 17344, "ns_per_operation": 14445.900196033212, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250549693, "verified": true, "repeat": 0, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1083200, "ns_per_operation": 230.79825147710486, "operation": "open", "payload_bytes": 0, "seconds": 0.250000666, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1092736, "ns_per_operation": 228.79126980350242, "operation": "seal", "payload_bytes": 0, "seconds": 0.250008457, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 995776, "ns_per_operation": 251.06344499164473, "operation": "open", "payload_bytes": 64, "seconds": 0.250002953, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 996160, "ns_per_operation": 250.97166017507232, "operation": "seal", "payload_bytes": 64, "seconds": 0.250007929, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 522816, "ns_per_operation": 478.18611710429667, "operation": "open", "payload_bytes": 256, "seconds": 0.250003353, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 652032, "ns_per_operation": 383.4221648630742, "operation": "seal", "payload_bytes": 256, "seconds": 0.250003521, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 191296, "ns_per_operation": 1307.1395063148211, "operation": "open", "payload_bytes": 1280, "seconds": 0.250050559, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 175488, "ns_per_operation": 1424.7534589259665, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250027135, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 162304, "ns_per_operation": 1540.5202521194797, "operation": "open", "payload_bytes": 1420, "seconds": 0.250032599, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 167808, "ns_per_operation": 1489.8144605739892, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250002785, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 17280, "ns_per_operation": 14514.144675925923, "operation": "open", "payload_bytes": 16384, "seconds": 0.25080442, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 17344, "ns_per_operation": 14444.502248616236, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250525447, "verified": true, "repeat": 1, "position": 0, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 913920, "ns_per_operation": 273.5567675507703, "operation": "open", "payload_bytes": 0, "seconds": 0.250009001, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 920512, "ns_per_operation": 271.59208896787874, "operation": "seal", "payload_bytes": 0, "seconds": 0.250003777, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 503744, "ns_per_operation": 496.3337409477829, "operation": "open", "payload_bytes": 64, "seconds": 0.250025144, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 497856, "ns_per_operation": 502.16486895809226, "operation": "seal", "payload_bytes": 64, "seconds": 0.250005793, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 214144, "ns_per_operation": 1167.4680261879857, "operation": "open", "payload_bytes": 256, "seconds": 0.250006273, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 213568, "ns_per_operation": 1170.8922169988014, "operation": "seal", "payload_bytes": 256, "seconds": 0.250065109, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 85888, "ns_per_operation": 2911.991453986587, "operation": "open", "payload_bytes": 1280, "seconds": 0.250105122, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 86208, "ns_per_operation": 2901.275450074239, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250113154, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 69888, "ns_per_operation": 3578.5452581272893, "operation": "open", "payload_bytes": 1420, "seconds": 0.250097371, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 70208, "ns_per_operation": 3561.2016864175025, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250024848, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 9024, "ns_per_operation": 27899.262965425532, "operation": "open", "payload_bytes": 16384, "seconds": 0.251762949, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 8960, "ns_per_operation": 27912.22455357143, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250093532, "verified": true, "repeat": 1, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 1308928, "ns_per_operation": 191.00054472056522, "operation": "open", "payload_bytes": 0, "seconds": 0.250005961, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 1407552, "ns_per_operation": 177.61785070817987, "operation": "seal", "payload_bytes": 0, "seconds": 0.250006361, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 755392, "ns_per_operation": 330.95758758366514, "operation": "open", "payload_bytes": 64, "seconds": 0.250002714, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 767360, "ns_per_operation": 325.81086321934947, "operation": "seal", "payload_bytes": 64, "seconds": 0.250014224, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 324160, "ns_per_operation": 771.2852171767029, "operation": "open", "payload_bytes": 256, "seconds": 0.250019816, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 327616, "ns_per_operation": 763.097849311389, "operation": "seal", "payload_bytes": 256, "seconds": 0.250003065, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 80000, "ns_per_operation": 3126.078725, "operation": "open", "payload_bytes": 1280, "seconds": 0.250086298, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 80128, "ns_per_operation": 3120.9235473242816, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250073362, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 70592, "ns_per_operation": 3542.827175883953, "operation": "open", "payload_bytes": 1420, "seconds": 0.250095256, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 70848, "ns_per_operation": 3530.20176998645, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250107735, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 6656, "ns_per_operation": 37826.983323317305, "operation": "open", "payload_bytes": 16384, "seconds": 0.251776401, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 6656, "ns_per_operation": 37687.64768629808, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250848983, "verified": true, "repeat": 1, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1065984, "ns_per_operation": 234.53749211995674, "operation": "open", "payload_bytes": 0, "seconds": 0.250013214, "verified": true, "repeat": 1, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1108864, "ns_per_operation": 225.46093840182382, "operation": "seal", "payload_bytes": 0, "seconds": 0.250005518, "verified": true, "repeat": 1, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 981184, "ns_per_operation": 254.80942718185375, "operation": "open", "payload_bytes": 64, "seconds": 0.250014933, "verified": true, "repeat": 1, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1016896, "ns_per_operation": 245.85063467650573, "operation": "seal", "payload_bytes": 64, "seconds": 0.250004527, "verified": true, "repeat": 1, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 518144, "ns_per_operation": 482.5116048820405, "operation": "open", "payload_bytes": 256, "seconds": 0.250010493, "verified": true, "repeat": 1, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 659264, "ns_per_operation": 379.2439644816037, "operation": "seal", "payload_bytes": 256, "seconds": 0.250021893, "verified": true, "repeat": 1, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 190848, "ns_per_operation": 1310.191414109658, "operation": "open", "payload_bytes": 1280, "seconds": 0.250047411, "verified": true, "repeat": 1, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 176128, "ns_per_operation": 1419.8173260356104, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250069586, "verified": true, "repeat": 1, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 162048, "ns_per_operation": 1542.8584925454186, "operation": "open", "payload_bytes": 1420, "seconds": 0.250017133, "verified": true, "repeat": 1, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 168384, "ns_per_operation": 1484.8155941182058, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250019189, "verified": true, "repeat": 1, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 17280, "ns_per_operation": 14503.866030092593, "operation": "open", "payload_bytes": 16384, "seconds": 0.250626805, "verified": true, "repeat": 1, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 17344, "ns_per_operation": 14450.037707564577, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250621454, "verified": true, "repeat": 1, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1065664, "ns_per_operation": 234.59882101675572, "operation": "open", "payload_bytes": 0, "seconds": 0.250003518, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1108928, "ns_per_operation": 225.4465628066024, "operation": "seal", "payload_bytes": 0, "seconds": 0.250004006, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 982464, "ns_per_operation": 254.46675094456387, "operation": "open", "payload_bytes": 64, "seconds": 0.250004422, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1016192, "ns_per_operation": 246.0243949962212, "operation": "seal", "payload_bytes": 64, "seconds": 0.250008022, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 518272, "ns_per_operation": 482.4188630680415, "operation": "open", "payload_bytes": 256, "seconds": 0.250024189, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 657664, "ns_per_operation": 380.1411632687816, "operation": "seal", "payload_bytes": 256, "seconds": 0.250005158, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 190784, "ns_per_operation": 1310.8061525075477, "operation": "open", "payload_bytes": 1280, "seconds": 0.250080841, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 176128, "ns_per_operation": 1419.9080214389533, "operation": "seal", "payload_bytes": 1280, "seconds": 0.25008556, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 162048, "ns_per_operation": 1542.9346675059242, "operation": "open", "payload_bytes": 1420, "seconds": 0.250029477, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 168576, "ns_per_operation": 1483.3601105732728, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250058914, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 17280, "ns_per_operation": 14505.394618055554, "operation": "open", "payload_bytes": 16384, "seconds": 0.250653219, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 17344, "ns_per_operation": 14450.386819649448, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250627509, "verified": true, "repeat": 2, "position": 0, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 1304512, "ns_per_operation": 191.64362918853945, "operation": "open", "payload_bytes": 0, "seconds": 0.250001414, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 1400768, "ns_per_operation": 178.4815629711701, "operation": "seal", "payload_bytes": 0, "seconds": 0.250011262, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 754496, "ns_per_operation": 331.35602972050214, "operation": "open", "payload_bytes": 64, "seconds": 0.250006799, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 765952, "ns_per_operation": 326.396940017129, "operation": "seal", "payload_bytes": 64, "seconds": 0.250004389, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 323904, "ns_per_operation": 771.8344139004148, "operation": "open", "payload_bytes": 256, "seconds": 0.250000254, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 327168, "ns_per_operation": 764.2209140258215, "operation": "seal", "payload_bytes": 256, "seconds": 0.250028628, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 80064, "ns_per_operation": 3122.969974020783, "operation": "open", "payload_bytes": 1280, "seconds": 0.250037468, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 80320, "ns_per_operation": 3112.5504731075703, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250000054, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 70720, "ns_per_operation": 3536.17443438914, "operation": "open", "payload_bytes": 1420, "seconds": 0.250078256, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 70912, "ns_per_operation": 3526.904205212094, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250099831, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 6592, "ns_per_operation": 37940.45130461165, "operation": "open", "payload_bytes": 16384, "seconds": 0.250103455, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 6656, "ns_per_operation": 37856.45973557692, "operation": "seal", "payload_bytes": 16384, "seconds": 0.251972596, "verified": true, "repeat": 2, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 910080, "ns_per_operation": 274.71370099331926, "operation": "open", "payload_bytes": 0, "seconds": 0.250011445, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 930624, "ns_per_operation": 268.6375657623272, "operation": "seal", "payload_bytes": 0, "seconds": 0.250000566, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 502912, "ns_per_operation": 497.1246440729193, "operation": "open", "payload_bytes": 64, "seconds": 0.250009949, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 499456, "ns_per_operation": 500.6084439870579, "operation": "seal", "payload_bytes": 64, "seconds": 0.250031891, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 214144, "ns_per_operation": 1167.6599017483563, "operation": "open", "payload_bytes": 256, "seconds": 0.250047362, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 213504, "ns_per_operation": 1171.1666057778777, "operation": "seal", "payload_bytes": 256, "seconds": 0.250048755, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 85888, "ns_per_operation": 2912.2806096311474, "operation": "open", "payload_bytes": 1280, "seconds": 0.250129957, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 86208, "ns_per_operation": 2900.763467427617, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250069017, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 69888, "ns_per_operation": 3577.3902959020143, "operation": "open", "payload_bytes": 1420, "seconds": 0.250016653, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 70208, "ns_per_operation": 3561.354646194166, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250035587, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 9024, "ns_per_operation": 27893.3516179078, "operation": "open", "payload_bytes": 16384, "seconds": 0.251709605, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 9024, "ns_per_operation": 27894.262854609926, "operation": "seal", "payload_bytes": 16384, "seconds": 0.251717828, "verified": true, "repeat": 2, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1091072, "ns_per_operation": 229.13862054933128, "operation": "open", "payload_bytes": 0, "seconds": 0.250006733, "verified": true, "repeat": 2, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1078080, "ns_per_operation": 231.9032177574948, "operation": "seal", "payload_bytes": 0, "seconds": 0.250010221, "verified": true, "repeat": 2, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 999168, "ns_per_operation": 250.22381421342558, "operation": "open", "payload_bytes": 64, "seconds": 0.250015628, "verified": true, "repeat": 2, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 982912, "ns_per_operation": 254.35980942342755, "operation": "seal", "payload_bytes": 64, "seconds": 0.250013309, "verified": true, "repeat": 2, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 524736, "ns_per_operation": 476.43175615928766, "operation": "open", "payload_bytes": 256, "seconds": 0.250000894, "verified": true, "repeat": 2, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 646976, "ns_per_operation": 386.43391563211003, "operation": "seal", "payload_bytes": 256, "seconds": 0.250013469, "verified": true, "repeat": 2, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 191360, "ns_per_operation": 1306.6512280518396, "operation": "open", "payload_bytes": 1280, "seconds": 0.250040779, "verified": true, "repeat": 2, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 175168, "ns_per_operation": 1427.3029605864083, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250017805, "verified": true, "repeat": 2, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 162496, "ns_per_operation": 1538.7256486313509, "operation": "open", "payload_bytes": 1420, "seconds": 0.250036763, "verified": true, "repeat": 2, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 167616, "ns_per_operation": 1491.506240454372, "operation": "seal", "payload_bytes": 1420, "seconds": 0.25000031, "verified": true, "repeat": 2, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 17280, "ns_per_operation": 14507.465682870372, "operation": "open", "payload_bytes": 16384, "seconds": 0.250689007, "verified": true, "repeat": 2, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 17344, "ns_per_operation": 14447.915532749077, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250584647, "verified": true, "repeat": 2, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1091456, "ns_per_operation": 229.06280509704467, "operation": "open", "payload_bytes": 0, "seconds": 0.250011973, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1082560, "ns_per_operation": 230.94506078185043, "operation": "seal", "payload_bytes": 0, "seconds": 0.250011885, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 999488, "ns_per_operation": 250.13577951911378, "operation": "open", "payload_bytes": 64, "seconds": 0.25000771, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 982656, "ns_per_operation": 254.4150526735704, "operation": "seal", "payload_bytes": 64, "seconds": 0.250002478, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 524928, "ns_per_operation": 476.2923315197512, "operation": "open", "payload_bytes": 256, "seconds": 0.250019181, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 649472, "ns_per_operation": 384.95203026458415, "operation": "seal", "payload_bytes": 256, "seconds": 0.250015565, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 191360, "ns_per_operation": 1306.4772836538464, "operation": "open", "payload_bytes": 1280, "seconds": 0.250007493, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 175232, "ns_per_operation": 1427.1144140339663, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250076113, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 162496, "ns_per_operation": 1538.6325571090981, "operation": "open", "payload_bytes": 1420, "seconds": 0.250021636, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 167616, "ns_per_operation": 1491.996139986636, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250082425, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 17280, "ns_per_operation": 14511.378993055556, "operation": "open", "payload_bytes": 16384, "seconds": 0.250756629, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 17344, "ns_per_operation": 14446.968692343171, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250568225, "verified": true, "repeat": 3, "position": 0, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 912064, "ns_per_operation": 274.11063806925836, "operation": "open", "payload_bytes": 0, "seconds": 0.250006445, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 921664, "ns_per_operation": 271.26130672349143, "operation": "seal", "payload_bytes": 0, "seconds": 0.250011781, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 502464, "ns_per_operation": 497.56890244873273, "operation": "open", "payload_bytes": 64, "seconds": 0.250010461, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 497344, "ns_per_operation": 502.6719011388496, "operation": "seal", "payload_bytes": 64, "seconds": 0.250000854, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 213952, "ns_per_operation": 1168.4886236165123, "operation": "open", "payload_bytes": 256, "seconds": 0.250000478, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 213312, "ns_per_operation": 1172.125918841884, "operation": "seal", "payload_bytes": 256, "seconds": 0.250028524, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 85824, "ns_per_operation": 2913.58158557047, "operation": "open", "payload_bytes": 1280, "seconds": 0.250055226, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 86208, "ns_per_operation": 2900.6808880846324, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250061898, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 69824, "ns_per_operation": 3580.5617266269483, "operation": "open", "payload_bytes": 1420, "seconds": 0.250009142, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 70208, "ns_per_operation": 3562.0527575205106, "operation": "seal", "payload_bytes": 1420, "seconds": 0.2500846, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 9024, "ns_per_operation": 27884.472850177306, "operation": "open", "payload_bytes": 16384, "seconds": 0.251629483, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 9024, "ns_per_operation": 27893.01307624113, "operation": "seal", "payload_bytes": 16384, "seconds": 0.25170655, "verified": true, "repeat": 3, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 1309376, "ns_per_operation": 190.9353745600958, "operation": "open", "payload_bytes": 0, "seconds": 0.250006197, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 1406336, "ns_per_operation": 177.77067642441065, "operation": "seal", "payload_bytes": 0, "seconds": 0.250005302, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 753408, "ns_per_operation": 331.83822444147125, "operation": "open", "payload_bytes": 64, "seconds": 0.250009573, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 768768, "ns_per_operation": 325.1989533903597, "operation": "seal", "payload_bytes": 64, "seconds": 0.250002549, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 324288, "ns_per_operation": 771.0000123347148, "operation": "open", "payload_bytes": 256, "seconds": 0.250026052, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 327360, "ns_per_operation": 763.7170485092864, "operation": "seal", "payload_bytes": 256, "seconds": 0.250010413, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 80064, "ns_per_operation": 3123.3133992805756, "operation": "open", "payload_bytes": 1280, "seconds": 0.250064964, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 80192, "ns_per_operation": 3118.449271747805, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250074684, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 70720, "ns_per_operation": 3537.2015695701352, "operation": "open", "payload_bytes": 1420, "seconds": 0.250150895, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 70912, "ns_per_operation": 3528.6067520306856, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250220562, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 6656, "ns_per_operation": 37806.910456730766, "operation": "open", "payload_bytes": 16384, "seconds": 0.251642796, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 6592, "ns_per_operation": 37928.13470873787, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250022264, "verified": true, "repeat": 3, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1065472, "ns_per_operation": 234.65077636953387, "operation": "open", "payload_bytes": 0, "seconds": 0.250013832, "verified": true, "repeat": 3, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1108928, "ns_per_operation": 225.4519608126046, "operation": "seal", "payload_bytes": 0, "seconds": 0.250009992, "verified": true, "repeat": 3, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 982272, "ns_per_operation": 254.5277071931196, "operation": "open", "payload_bytes": 64, "seconds": 0.25001544, "verified": true, "repeat": 3, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1016128, "ns_per_operation": 246.0439787113434, "operation": "seal", "payload_bytes": 64, "seconds": 0.250012176, "verified": true, "repeat": 3, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 518208, "ns_per_operation": 482.4627003056688, "operation": "open", "payload_bytes": 256, "seconds": 0.250016031, "verified": true, "repeat": 3, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 658112, "ns_per_operation": 379.9016565569387, "operation": "seal", "payload_bytes": 256, "seconds": 0.250017839, "verified": true, "repeat": 3, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 190784, "ns_per_operation": 1310.788221234485, "operation": "open", "payload_bytes": 1280, "seconds": 0.25007742, "verified": true, "repeat": 3, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 176064, "ns_per_operation": 1420.1476622137407, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250036878, "verified": true, "repeat": 3, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 162048, "ns_per_operation": 1542.929249358215, "operation": "open", "payload_bytes": 1420, "seconds": 0.250028599, "verified": true, "repeat": 3, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 168512, "ns_per_operation": 1483.7091720470946, "operation": "seal", "payload_bytes": 1420, "seconds": 0.2500228, "verified": true, "repeat": 3, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 17280, "ns_per_operation": 14503.7, "operation": "open", "payload_bytes": 16384, "seconds": 0.250623936, "verified": true, "repeat": 3, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 17344, "ns_per_operation": 14451.31953413284, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250643686, "verified": true, "repeat": 3, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1065920, "ns_per_operation": 234.5445296082258, "operation": "open", "payload_bytes": 0, "seconds": 0.250005705, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1108096, "ns_per_operation": 225.6241841862077, "operation": "seal", "payload_bytes": 0, "seconds": 0.250013256, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 981696, "ns_per_operation": 254.67430243171003, "operation": "open", "payload_bytes": 64, "seconds": 0.250012744, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1016256, "ns_per_operation": 246.007070068959, "operation": "seal", "payload_bytes": 64, "seconds": 0.250006161, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 518400, "ns_per_operation": 482.26810185185184, "operation": "open", "payload_bytes": 256, "seconds": 0.250007784, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 657664, "ns_per_operation": 380.13431934848194, "operation": "seal", "payload_bytes": 256, "seconds": 0.250000657, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 190784, "ns_per_operation": 1310.7131677708824, "operation": "open", "payload_bytes": 1280, "seconds": 0.250063101, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 176128, "ns_per_operation": 1419.8305096293604, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250071908, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 162048, "ns_per_operation": 1542.9838504640602, "operation": "open", "payload_bytes": 1420, "seconds": 0.250037447, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 168576, "ns_per_operation": 1483.4671364844346, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250076956, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 17280, "ns_per_operation": 14504.136979166668, "operation": "open", "payload_bytes": 16384, "seconds": 0.250631487, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 17344, "ns_per_operation": 14450.90676891144, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250636527, "verified": true, "repeat": 4, "position": 0, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 1303232, "ns_per_operation": 191.8383219564897, "operation": "open", "payload_bytes": 0, "seconds": 0.25000984, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 1400448, "ns_per_operation": 178.52143314139477, "operation": "seal", "payload_bytes": 0, "seconds": 0.250009984, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 753920, "ns_per_operation": 331.612255942275, "operation": "open", "payload_bytes": 64, "seconds": 0.250009112, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 765440, "ns_per_operation": 326.63226248954845, "operation": "seal", "payload_bytes": 64, "seconds": 0.250017399, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 324160, "ns_per_operation": 771.313644496545, "operation": "open", "payload_bytes": 256, "seconds": 0.250029031, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 327168, "ns_per_operation": 764.2162283597416, "operation": "seal", "payload_bytes": 256, "seconds": 0.250027095, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 80000, "ns_per_operation": 3125.0545999999995, "operation": "open", "payload_bytes": 1280, "seconds": 0.250004368, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 80192, "ns_per_operation": 3118.407671588189, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250071348, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 70592, "ns_per_operation": 3544.515228354488, "operation": "open", "payload_bytes": 1420, "seconds": 0.250214419, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 70784, "ns_per_operation": 3532.386386754069, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250036438, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 6656, "ns_per_operation": 37789.88882211539, "operation": "open", "payload_bytes": 16384, "seconds": 0.2515295, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 6656, "ns_per_operation": 37847.89573317308, "operation": "seal", "payload_bytes": 16384, "seconds": 0.251915594, "verified": true, "repeat": 4, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 911872, "ns_per_operation": 274.1636907372964, "operation": "open", "payload_bytes": 0, "seconds": 0.250002193, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 921472, "ns_per_operation": 271.3136731230032, "operation": "seal", "payload_bytes": 0, "seconds": 0.250007953, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 503616, "ns_per_operation": 496.41653958571607, "operation": "open", "payload_bytes": 64, "seconds": 0.250003312, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 497408, "ns_per_operation": 502.60969063625834, "operation": "seal", "payload_bytes": 64, "seconds": 0.250002081, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 214016, "ns_per_operation": 1168.1546846964711, "operation": "open", "payload_bytes": 256, "seconds": 0.250003793, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 213376, "ns_per_operation": 1171.9188521670665, "operation": "seal", "payload_bytes": 256, "seconds": 0.250059357, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 85824, "ns_per_operation": 2913.003355704698, "operation": "open", "payload_bytes": 1280, "seconds": 0.2500056, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 86208, "ns_per_operation": 2900.1783593170007, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250018576, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 69888, "ns_per_operation": 3578.778531364469, "operation": "open", "payload_bytes": 1420, "seconds": 0.250113674, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 70208, "ns_per_operation": 3561.4564437101185, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250042734, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 9024, "ns_per_operation": 27886.150265957447, "operation": "open", "payload_bytes": 16384, "seconds": 0.25164462, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 9024, "ns_per_operation": 27891.19082446809, "operation": "seal", "payload_bytes": 16384, "seconds": 0.251690106, "verified": true, "repeat": 4, "position": 2, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1091264, "ns_per_operation": 229.095420539851, "operation": "open", "payload_bytes": 0, "seconds": 0.250003585, "verified": true, "repeat": 4, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1077312, "ns_per_operation": 232.06827362918077, "operation": "seal", "payload_bytes": 0, "seconds": 0.250009936, "verified": true, "repeat": 4, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 999424, "ns_per_operation": 250.15104800365015, "operation": "open", "payload_bytes": 64, "seconds": 0.250006961, "verified": true, "repeat": 4, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 982080, "ns_per_operation": 254.5621548142717, "operation": "seal", "payload_bytes": 64, "seconds": 0.250000401, "verified": true, "repeat": 4, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 524608, "ns_per_operation": 476.57502744906674, "operation": "open", "payload_bytes": 256, "seconds": 0.250015072, "verified": true, "repeat": 4, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 646656, "ns_per_operation": 386.62357884253765, "operation": "seal", "payload_bytes": 256, "seconds": 0.250012457, "verified": true, "repeat": 4, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 191360, "ns_per_operation": 1306.5534280936454, "operation": "open", "payload_bytes": 1280, "seconds": 0.250022064, "verified": true, "repeat": 4, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 175168, "ns_per_operation": 1427.4827651169164, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250049301, "verified": true, "repeat": 4, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 162496, "ns_per_operation": 1538.9663499409214, "operation": "open", "payload_bytes": 1420, "seconds": 0.250075876, "verified": true, "repeat": 4, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 167552, "ns_per_operation": 1492.2529543067228, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250029967, "verified": true, "repeat": 4, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 17280, "ns_per_operation": 14507.049189814816, "operation": "open", "payload_bytes": 16384, "seconds": 0.25068181, "verified": true, "repeat": 4, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 17344, "ns_per_operation": 14446.807310885608, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250565426, "verified": true, "repeat": 4, "position": 3, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1083584, "ns_per_operation": 230.72067509302465, "operation": "open", "payload_bytes": 0, "seconds": 0.250005232, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 1088256, "ns_per_operation": 229.7321062323571, "operation": "seal", "payload_bytes": 0, "seconds": 0.250007343, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 994688, "ns_per_operation": 251.34208012964865, "operation": "open", "payload_bytes": 64, "seconds": 0.250006951, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 995584, "ns_per_operation": 251.10932276934946, "operation": "seal", "payload_bytes": 64, "seconds": 0.250000424, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 523392, "ns_per_operation": 477.70071762655914, "operation": "open", "payload_bytes": 256, "seconds": 0.250024734, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 652544, "ns_per_operation": 383.12862734160456, "operation": "seal", "payload_bytes": 256, "seconds": 0.250008287, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 191296, "ns_per_operation": 1307.2866866008696, "operation": "open", "payload_bytes": 1280, "seconds": 0.250078714, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 175424, "ns_per_operation": 1425.1965010488873, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250013671, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 162368, "ns_per_operation": 1539.8337972999607, "operation": "open", "payload_bytes": 1420, "seconds": 0.250019734, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 167872, "ns_per_operation": 1489.3038445958825, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250012415, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 17280, "ns_per_operation": 14511.432233796299, "operation": "open", "payload_bytes": 16384, "seconds": 0.250757549, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "aws-lc", "includes_input_restore": true, "iterations": 17344, "ns_per_operation": 14445.005304428045, "operation": "seal", "payload_bytes": 16384, "seconds": 0.250534172, "verified": true, "repeat": 5, "position": 0, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 911104, "ns_per_operation": 274.40283326601576, "operation": "open", "payload_bytes": 0, "seconds": 0.250009519, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 930816, "ns_per_operation": 268.59913667147964, "operation": "seal", "payload_bytes": 0, "seconds": 0.250016374, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 503168, "ns_per_operation": 496.8694332707962, "operation": "open", "payload_bytes": 64, "seconds": 0.250008799, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 499136, "ns_per_operation": 500.9219090588537, "operation": "seal", "payload_bytes": 64, "seconds": 0.250028158, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 214144, "ns_per_operation": 1167.6882284817693, "operation": "open", "payload_bytes": 256, "seconds": 0.250053428, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 213568, "ns_per_operation": 1170.9075657401859, "operation": "seal", "payload_bytes": 256, "seconds": 0.250068387, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 85888, "ns_per_operation": 2912.1197839046195, "operation": "open", "payload_bytes": 1280, "seconds": 0.250116144, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 86272, "ns_per_operation": 2899.7484699554893, "operation": "seal", "payload_bytes": 1280, "seconds": 0.2501671, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 69888, "ns_per_operation": 3577.2821514423076, "operation": "open", "payload_bytes": 1420, "seconds": 0.250009095, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 70272, "ns_per_operation": 3559.2278005464477, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250114056, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 8960, "ns_per_operation": 27914.92232142857, "operation": "open", "payload_bytes": 16384, "seconds": 0.250117704, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto11", "includes_input_restore": true, "iterations": 9024, "ns_per_operation": 27895.354166666668, "operation": "seal", "payload_bytes": 16384, "seconds": 0.251727676, "verified": true, "repeat": 5, "position": 1, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 1305920, "ns_per_operation": 191.43902229845625, "operation": "open", "payload_bytes": 0, "seconds": 0.250004048, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 1397440, "ns_per_operation": 178.90524387451342, "operation": "seal", "payload_bytes": 0, "seconds": 0.250009344, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 751808, "ns_per_operation": 332.5426584979144, "operation": "open", "payload_bytes": 64, "seconds": 0.250008231, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 766400, "ns_per_operation": 326.22021659707724, "operation": "seal", "payload_bytes": 64, "seconds": 0.250015174, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 324096, "ns_per_operation": 771.4792067782384, "operation": "open", "payload_bytes": 256, "seconds": 0.250033325, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 326912, "ns_per_operation": 764.8806651331245, "operation": "seal", "payload_bytes": 256, "seconds": 0.250048668, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 80064, "ns_per_operation": 3124.0729291566745, "operation": "open", "payload_bytes": 1280, "seconds": 0.250125775, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 80256, "ns_per_operation": 3115.2095419657103, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250014257, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 70592, "ns_per_operation": 3541.6811111740712, "operation": "open", "payload_bytes": 1420, "seconds": 0.250014353, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 70848, "ns_per_operation": 3529.1868648373984, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250035831, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 6656, "ns_per_operation": 37799.402644230766, "operation": "open", "payload_bytes": 16384, "seconds": 0.251592824, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "rustcrypto", "includes_input_restore": true, "iterations": 6656, "ns_per_operation": 37857.883112980766, "operation": "seal", "payload_bytes": 16384, "seconds": 0.25198207, "verified": true, "repeat": 5, "position": 2, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1065984, "ns_per_operation": 234.52975842038904, "operation": "open", "payload_bytes": 0, "seconds": 0.25000497, "verified": true, "repeat": 5, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1108672, "ns_per_operation": 225.50137281360043, "operation": "seal", "payload_bytes": 0, "seconds": 0.250007058, "verified": true, "repeat": 5, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 982784, "ns_per_operation": 254.38463996157853, "operation": "open", "payload_bytes": 64, "seconds": 0.250005154, "verified": true, "repeat": 5, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 1016768, "ns_per_operation": 245.89184651759297, "operation": "seal", "payload_bytes": 64, "seconds": 0.250014961, "verified": true, "repeat": 5, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 518464, "ns_per_operation": 482.20520807616344, "operation": "open", "payload_bytes": 256, "seconds": 0.250006041, "verified": true, "repeat": 5, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 658176, "ns_per_operation": 379.8703340747764, "operation": "seal", "payload_bytes": 256, "seconds": 0.250021537, "verified": true, "repeat": 5, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 190848, "ns_per_operation": 1310.2712001173709, "operation": "open", "payload_bytes": 1280, "seconds": 0.250062638, "verified": true, "repeat": 5, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 176128, "ns_per_operation": 1419.5278944858285, "operation": "seal", "payload_bytes": 1280, "seconds": 0.250018609, "verified": true, "repeat": 5, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 162048, "ns_per_operation": 1542.919036334913, "operation": "open", "payload_bytes": 1420, "seconds": 0.250026944, "verified": true, "repeat": 5, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 168576, "ns_per_operation": 1483.482470814351, "operation": "seal", "payload_bytes": 1420, "seconds": 0.250079541, "verified": true, "repeat": 5, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 17280, "ns_per_operation": 14504.564756944443, "operation": "open", "payload_bytes": 16384, "seconds": 0.250638879, "verified": true, "repeat": 5, "position": 3, "cpu": 2}
{"backend": "ring", "includes_input_restore": true, "iterations": 17344, "ns_per_operation": 14450.75415129151, "operation": "seal", "payload_bytes": 16384, "seconds": 0.25063388, "verified": true, "repeat": 5, "position": 3, "cpu": 2}
```

## Focused Linux 6.12 x64 · unpaced

[Run](https://github.com/riKirsti/ProtunNext/actions/runs/43158753819) / [artifact](https://github.com/riKirsti/ProtunNext/actions/runs/43158753819/artifacts/13917541273)

```json
{
  "environment": {
    "kernel": "6.12.115",
    "machine": "x86_64",
    "sources": {
      "main": {
        "sha": "4c7cc53a0932fa31579729b603d80a9b9e9ecabf",
        "archive_sha256": "461801494295379a66f8654d22f29fbb58d9036bb65cd8b5fa911c97bc327900",
        "effective_sources_sha256": "805c3dcf6912010b33a410f63b88f25a2be83c10a1d3b44a7abac6edf8d84437",
        "runtime_sources_sha256": "5d448b4aabf1041e3a13920294314097f88a30eeb9a2d4c799724c2c04112fc7"
      },
      "erase": {
        "sha": "905517cc70b2b9b05a3c33cac0c06b71924bfc65",
        "archive_sha256": "2c8fc2cd4e4d8518c581b89b2257958146b4558c8c272d192cfcc1878d4fa7c9",
        "effective_sources_sha256": "684e7c25d2337339f0ff0f6f9527c1b5f5837570be87d7f9e6c023caaeebdc8e",
        "runtime_sources_sha256": "6d1b450fb9abe512604fff33c89583cae6afe82ea5162ce755b27712726f7341",
        "tls_record_prototype_patch_sha256": "3cf2ac9ad7eef47777bd72a4846ddb86d9d2f4660594309d2cdeb9a99e69c5e8",
        "prototype_scope": "NoLocalAgent record-cipher comparison; dual-provider compatibility unresolved"
      }
    },
    "binaries": {
      "main": "5a0d52e9683678e7cc1242a3e4d3353233518714ebe1f42ec9e7a3c365d3166a",
      "erase": "e8aee260595ed69f45c2b39a396fc5e3e115ab37be6c343afb8f437ec8f3f512"
    },
    "kernel_image_sha256": "99dccb6ab8879ee111396024ee3e507a5e1e028f93babbecc9435b603f336cde",
    "kernel_builder_sha256": "66c97edcada6d0d9edc5dbdcd2b8abc66dbad7395f9c93cd1d49dea1a1fb7486",
    "harness_sha256": {
      "check_codegen.py": "225fc05e6cbe4c4cd322c0b596b090052753145ff4fd238492a559fa02ce733a",
      "native.py": "efe08efa880f6ab9ecbc3c7b3aded603b3ec287c2a806896cb8a652e43f45b71",
      "analyze.py": "9b77b45d16c021f2a176928109cff792b29f1c5b345780609e5f3cb50aa2299f",
      "run.py": "dc3e0f012e220a6eda5af855bfdb5b3a8001fb7a38ef733b85ac274ba12b4c8b",
      "negative_control.py": "756709eec532b6f5b66d6dff72b93c7a4fc2e64f49b4978368380124111e8237",
      "test_analysis.py": "1de785494ca0cab59d6081c643cb6ae9bd9f6deef5c662f27a5b8956f69f3dc8",
      "lts.py": "28fad55a908146ea3481d9d079251fc0fc408afe1c6223f85535bc2e9b84bf06",
      "prepare.py": "d480cca5d0e0c52528e6816fc0825bc5a81671a88545d30a4315d3492c8b64db"
    },
    "compiler": "rustc 1.99.1 (58cf8f9c3 2026-10-13)\nbinary: rustc\ncommit-hash: 58cf8f9c35b355199727627d07ae34a4e2846d67\ncommit-date: 2026-10-13\nhost: x86_64-unknown-linux-gnu\nrelease: 1.99.1\nLLVM version: 23.1.1\ncargo 1.99.1 (5f94df478 2026-08-27)\nrelease: 1.99.1\ncommit-hash: 5f94df4789f005f9a352888e8355ffc645b7ed0e\ncommit-date: 2026-08-27\nhost: x86_64-unknown-linux-gnu\nlibgit2: 1.9.6 (sys:0.21.0 vendored)\nlibcurl: 8.21.0-DEV (sys:0.4.90+curl-8.21.0 vendored ssl:OpenSSL/3.6.3)\nssl: OpenSSL 3.6.3 9 Jun 2026\nos: Ubuntu 24.4.0 (noble) [64-bit]\ntarget=x86_64-unknown-linux-musl\nprofile=release thin-LTO codegen-units=1 panic=abort\nRUSTFLAGS=\n",
    "host_cpus": [
      0,
      1,
      2,
      3
    ],
    "workload": "download",
    "warm_seconds": null,
    "measured_seconds": null,
    "scope": "actual-kernel KVM; pinned logical CPUs, not physical-core or interrupt isolation"
  },
  "same_runtime_sources": false,
  "observed_samples": 8,
  "expected_samples": 8,
  "errors": [],
  "blocks": [
    {
      "block": "stealth-v4-http-1",
      "capacity_ranking": false,
      "rate_controlled": false,
      "control": {
        "Mbps": {
          "baseline": 4060.0944649015287,
          "candidate": 4003.8811796016425,
          "change_percent": -1.384531463142924
        },
        "CPU_ns_per_byte": {
          "baseline": 1.8845475167727985,
          "candidate": 1.9152115254577706,
          "change_percent": 1.6271284439398404
        },
        "CPU_core_percent": {
          "baseline": 95.64112493339377,
          "candidate": 95.85194523076838,
          "change_percent": 0.22042850031451788
        },
        "peak_RSS_MiB": {
          "baseline": 2.8805091153599696,
          "candidate": 2.9392655930627467,
          "change_percent": 2.0397948886696993
        }
      },
      "result": {
        "Mbps": {
          "baseline": 4031.250860512292,
          "candidate": 4026.413766031565,
          "change_percent": -0.11998991499408973
        },
        "CPU_ns_per_byte": {
          "baseline": 1.898531906877116,
          "candidate": 1.8654564456246312,
          "change_percent": -1.7421598832589757
        },
        "CPU_core_percent": {
          "baseline": 95.66669773309842,
          "candidate": 93.88680302888092,
          "change_percent": -1.860516508245369
        },
        "peak_RSS_MiB": {
          "baseline": 2.9452192458711797,
          "candidate": 4.146447115500284,
          "change_percent": 40.785685864068434
        }
      },
      "AA_pass": true,
      "timing_pass": true,
      "auxiliary_limited": true,
      "validity": "usable"
    }
  ]
}
```

## Focused Linux 6.18 x64 · unpaced

[Run](https://github.com/riKirsti/ProtunNext/actions/runs/43158753819) / [artifact](https://github.com/riKirsti/ProtunNext/actions/runs/43158753819/artifacts/13917212415)

```json
{
  "environment": {
    "kernel": "6.18.60",
    "machine": "x86_64",
    "sources": {
      "main": {
        "sha": "4c7cc53a0932fa31579729b603d80a9b9e9ecabf",
        "archive_sha256": "461801494295379a66f8654d22f29fbb58d9036bb65cd8b5fa911c97bc327900",
        "effective_sources_sha256": "805c3dcf6912010b33a410f63b88f25a2be83c10a1d3b44a7abac6edf8d84437",
        "runtime_sources_sha256": "5d448b4aabf1041e3a13920294314097f88a30eeb9a2d4c799724c2c04112fc7"
      },
      "erase": {
        "sha": "905517cc70b2b9b05a3c33cac0c06b71924bfc65",
        "archive_sha256": "2c8fc2cd4e4d8518c581b89b2257958146b4558c8c272d192cfcc1878d4fa7c9",
        "effective_sources_sha256": "684e7c25d2337339f0ff0f6f9527c1b5f5837570be87d7f9e6c023caaeebdc8e",
        "runtime_sources_sha256": "6d1b450fb9abe512604fff33c89583cae6afe82ea5162ce755b27712726f7341",
        "tls_record_prototype_patch_sha256": "3cf2ac9ad7eef47777bd72a4846ddb86d9d2f4660594309d2cdeb9a99e69c5e8",
        "prototype_scope": "NoLocalAgent record-cipher comparison; dual-provider compatibility unresolved"
      }
    },
    "binaries": {
      "main": "5a0d52e9683678e7cc1242a3e4d3353233518714ebe1f42ec9e7a3c365d3166a",
      "erase": "e8aee260595ed69f45c2b39a396fc5e3e115ab37be6c343afb8f437ec8f3f512"
    },
    "kernel_image_sha256": "e216f51af56a3634e3757e25ca0e863a6d0e1c679a0f106b6ef079f2e27f462d",
    "kernel_builder_sha256": "66c97edcada6d0d9edc5dbdcd2b8abc66dbad7395f9c93cd1d49dea1a1fb7486",
    "harness_sha256": {
      "check_codegen.py": "225fc05e6cbe4c4cd322c0b596b090052753145ff4fd238492a559fa02ce733a",
      "native.py": "efe08efa880f6ab9ecbc3c7b3aded603b3ec287c2a806896cb8a652e43f45b71",
      "analyze.py": "9b77b45d16c021f2a176928109cff792b29f1c5b345780609e5f3cb50aa2299f",
      "run.py": "dc3e0f012e220a6eda5af855bfdb5b3a8001fb7a38ef733b85ac274ba12b4c8b",
      "negative_control.py": "756709eec532b6f5b66d6dff72b93c7a4fc2e64f49b4978368380124111e8237",
      "test_analysis.py": "1de785494ca0cab59d6081c643cb6ae9bd9f6deef5c662f27a5b8956f69f3dc8",
      "lts.py": "28fad55a908146ea3481d9d079251fc0fc408afe1c6223f85535bc2e9b84bf06",
      "prepare.py": "d480cca5d0e0c52528e6816fc0825bc5a81671a88545d30a4315d3492c8b64db"
    },
    "compiler": "rustc 1.99.1 (58cf8f9c3 2026-10-13)\nbinary: rustc\ncommit-hash: 58cf8f9c35b355199727627d07ae34a4e2846d67\ncommit-date: 2026-10-13\nhost: x86_64-unknown-linux-gnu\nrelease: 1.99.1\nLLVM version: 23.1.1\ncargo 1.99.1 (5f94df478 2026-08-27)\nrelease: 1.99.1\ncommit-hash: 5f94df4789f005f9a352888e8355ffc645b7ed0e\ncommit-date: 2026-08-27\nhost: x86_64-unknown-linux-gnu\nlibgit2: 1.9.6 (sys:0.21.0 vendored)\nlibcurl: 8.21.0-DEV (sys:0.4.90+curl-8.21.0 vendored ssl:OpenSSL/3.6.3)\nssl: OpenSSL 3.6.3 9 Jun 2026\nos: Ubuntu 24.4.0 (noble) [64-bit]\ntarget=x86_64-unknown-linux-musl\nprofile=release thin-LTO codegen-units=1 panic=abort\nRUSTFLAGS=\n",
    "host_cpus": [
      0,
      1,
      2,
      3
    ],
    "workload": "download",
    "warm_seconds": null,
    "measured_seconds": null,
    "scope": "actual-kernel KVM; pinned logical CPUs, not physical-core or interrupt isolation"
  },
  "same_runtime_sources": false,
  "observed_samples": 8,
  "expected_samples": 8,
  "errors": [],
  "blocks": [
    {
      "block": "stealth-v4-http-1",
      "capacity_ranking": false,
      "rate_controlled": false,
      "control": {
        "Mbps": {
          "baseline": 3613.09729199071,
          "candidate": 3622.7859115207702,
          "change_percent": 0.2681527439501119
        },
        "CPU_ns_per_byte": {
          "baseline": 1.7074797681803164,
          "candidate": 1.7021364662346556,
          "change_percent": -0.31293500779544514
        },
        "CPU_core_percent": {
          "baseline": 77.11556259284905,
          "candidate": 77.0803766482371,
          "change_percent": -0.045627553542870736
        },
        "peak_RSS_MiB": {
          "baseline": 3.0117162167628777,
          "candidate": 2.923681343384783,
          "change_percent": -2.9230799664358265
        }
      },
      "result": {
        "Mbps": {
          "baseline": 3641.806604576051,
          "candidate": 3629.1104151716986,
          "change_percent": -0.3486233834712471
        },
        "CPU_ns_per_byte": {
          "baseline": 1.7120360672961714,
          "candidate": 1.6579963312528916,
          "change_percent": -3.1564601398044734
        },
        "CPU_core_percent": {
          "baseline": 77.93572386057507,
          "candidate": 75.2125523841023,
          "change_percent": -3.494124826946432
        },
        "peak_RSS_MiB": {
          "baseline": 2.9295937484999515,
          "candidate": 4.060253284991129,
          "change_percent": 38.59441388657088
        }
      },
      "AA_pass": true,
      "timing_pass": true,
      "auxiliary_limited": true,
      "validity": "usable"
    }
  ]
}
```

## Focused native x64 · 200 Mbps

[Run](https://github.com/riKirsti/ProtunNext/actions/runs/43158751468) / [artifact](https://github.com/riKirsti/ProtunNext/actions/runs/43158751468/artifacts/13916848276)

```json
{
  "environment": {
    "kernel": "7.0.0-1017-azure",
    "machine": "x86_64",
    "sources": {
      "main": {
        "sha": "4c7cc53a0932fa31579729b603d80a9b9e9ecabf",
        "archive_sha256": "461801494295379a66f8654d22f29fbb58d9036bb65cd8b5fa911c97bc327900",
        "effective_sources_sha256": "805c3dcf6912010b33a410f63b88f25a2be83c10a1d3b44a7abac6edf8d84437",
        "runtime_sources_sha256": "5d448b4aabf1041e3a13920294314097f88a30eeb9a2d4c799724c2c04112fc7"
      },
      "erase": {
        "sha": "905517cc70b2b9b05a3c33cac0c06b71924bfc65",
        "archive_sha256": "2c8fc2cd4e4d8518c581b89b2257958146b4558c8c272d192cfcc1878d4fa7c9",
        "effective_sources_sha256": "684e7c25d2337339f0ff0f6f9527c1b5f5837570be87d7f9e6c023caaeebdc8e",
        "runtime_sources_sha256": "6d1b450fb9abe512604fff33c89583cae6afe82ea5162ce755b27712726f7341",
        "tls_record_prototype_patch_sha256": "3cf2ac9ad7eef47777bd72a4846ddb86d9d2f4660594309d2cdeb9a99e69c5e8",
        "prototype_scope": "NoLocalAgent record-cipher comparison; dual-provider compatibility unresolved"
      }
    },
    "binaries": {
      "main": "3b6f6428768595c75e26fd34e8e6718eaf8e301acefd430679a05e2eb397ed77",
      "erase": "997cbfbe972bcc32da3454744924a6052fc323dbc54940f95349efc35a847759"
    },
    "host_cpus": [
      0,
      1,
      2,
      3
    ],
    "compiler": "rustc 1.99.1 (58cf8f9c3 2026-10-13)\nbinary: rustc\ncommit-hash: 58cf8f9c35b355199727627d07ae34a4e2846d67\ncommit-date: 2026-10-13\nhost: x86_64-unknown-linux-gnu\nrelease: 1.99.1\nLLVM version: 23.1.1\ncargo 1.99.1 (5f94df478 2026-08-27)\nrelease: 1.99.1\ncommit-hash: 5f94df4789f005f9a352888e8355ffc645b7ed0e\ncommit-date: 2026-08-27\nhost: x86_64-unknown-linux-gnu\nlibgit2: 1.9.6 (sys:0.21.0 vendored)\nlibcurl: 8.21.0-DEV (sys:0.4.90+curl-8.21.0 vendored ssl:OpenSSL/3.6.3)\nssl: OpenSSL 3.6.3 9 Jun 2026\nos: Ubuntu 24.4.0 (noble) [64-bit]\ntarget=x86_64-unknown-linux-gnu\nprofile=release thin-LTO codegen-units=1 panic=abort\nRUSTFLAGS=\n",
    "scope": "native diagnostic: actual host kernel; not supported-LTS kernel acceptance"
  },
  "same_runtime_sources": false,
  "observed_samples": 8,
  "expected_samples": 8,
  "errors": [],
  "blocks": [
    {
      "block": "stealth-v4-http-1",
      "capacity_ranking": false,
      "rate_controlled": true,
      "control": {
        "Mbps": {
          "baseline": 200.08279150753873,
          "candidate": 200.08231873197377,
          "change_percent": -0.0002362899684693609
        },
        "CPU_ns_per_byte": {
          "baseline": 1.6507476299944206,
          "candidate": 1.6810279373059598,
          "change_percent": 1.8343389844298397
        },
        "CPU_core_percent": {
          "baseline": 4.128548207067046,
          "candidate": 4.2042713834848495,
          "change_percent": 1.8341356966157019
        },
        "peak_RSS_MiB": {
          "baseline": 5.099375608032505,
          "candidate": 5.068355988076904,
          "change_percent": -0.6083023165961432
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.0827110309857,
          "candidate": 200.08292300743383,
          "change_percent": 0.00010594441020028711
        },
        "CPU_ns_per_byte": {
          "baseline": 1.7234370268840788,
          "candidate": 1.6565848131193235,
          "change_percent": -3.879005308689587
        },
        "CPU_core_percent": {
          "baseline": 4.310345107275822,
          "candidate": 4.143149225916493,
          "change_percent": -3.87894419583954
        },
        "peak_RSS_MiB": {
          "baseline": 5.01952517022978,
          "candidate": 6.91498934465692,
          "change_percent": 37.761822286875216
        }
      },
      "AA_pass": true,
      "timing_pass": true,
      "auxiliary_limited": false,
      "validity": "usable"
    }
  ]
}
```

## Focused native ARM64 · 200 Mbps

[Run](https://github.com/riKirsti/ProtunNext/actions/runs/43158751468) / [artifact](https://github.com/riKirsti/ProtunNext/actions/runs/43158751468/artifacts/13917242501)

```json
{
  "environment": {
    "kernel": "7.0.0-1017-azure",
    "machine": "aarch64",
    "sources": {
      "main": {
        "sha": "4c7cc53a0932fa31579729b603d80a9b9e9ecabf",
        "archive_sha256": "461801494295379a66f8654d22f29fbb58d9036bb65cd8b5fa911c97bc327900",
        "effective_sources_sha256": "805c3dcf6912010b33a410f63b88f25a2be83c10a1d3b44a7abac6edf8d84437",
        "runtime_sources_sha256": "5d448b4aabf1041e3a13920294314097f88a30eeb9a2d4c799724c2c04112fc7"
      },
      "erase": {
        "sha": "905517cc70b2b9b05a3c33cac0c06b71924bfc65",
        "archive_sha256": "2c8fc2cd4e4d8518c581b89b2257958146b4558c8c272d192cfcc1878d4fa7c9",
        "effective_sources_sha256": "684e7c25d2337339f0ff0f6f9527c1b5f5837570be87d7f9e6c023caaeebdc8e",
        "runtime_sources_sha256": "6d1b450fb9abe512604fff33c89583cae6afe82ea5162ce755b27712726f7341",
        "tls_record_prototype_patch_sha256": "3cf2ac9ad7eef47777bd72a4846ddb86d9d2f4660594309d2cdeb9a99e69c5e8",
        "prototype_scope": "NoLocalAgent record-cipher comparison; dual-provider compatibility unresolved"
      }
    },
    "binaries": {
      "main": "7b1d9f62cb1631f1ae7b4b4e341d3780a01e902e594198743f0da259449969b5",
      "erase": "bd96cebd9d85155f9929cfdff98464733444bb57267bfd212959ed0a4008b796"
    },
    "host_cpus": [
      0,
      1,
      2,
      3
    ],
    "compiler": "rustc 1.99.1 (58cf8f9c3 2026-10-13)\nbinary: rustc\ncommit-hash: 58cf8f9c35b355199727627d07ae34a4e2846d67\ncommit-date: 2026-10-13\nhost: aarch64-unknown-linux-gnu\nrelease: 1.99.1\nLLVM version: 23.1.1\ncargo 1.99.1 (5f94df478 2026-08-27)\nrelease: 1.99.1\ncommit-hash: 5f94df4789f005f9a352888e8355ffc645b7ed0e\ncommit-date: 2026-08-27\nhost: aarch64-unknown-linux-gnu\nlibgit2: 1.9.6 (sys:0.21.0 vendored)\nlibcurl: 8.21.0-DEV (sys:0.4.90+curl-8.21.0 vendored ssl:OpenSSL/3.6.3)\nssl: OpenSSL 3.6.3 9 Jun 2026\nos: Ubuntu 24.4.0 (noble) [64-bit]\ntarget=aarch64-unknown-linux-gnu\nprofile=release thin-LTO codegen-units=1 panic=abort\nRUSTFLAGS=\n",
    "scope": "native diagnostic: actual host kernel; not supported-LTS kernel acceptance"
  },
  "same_runtime_sources": false,
  "observed_samples": 8,
  "expected_samples": 8,
  "errors": [],
  "blocks": [
    {
      "block": "stealth-v4-http-1",
      "capacity_ranking": false,
      "rate_controlled": true,
      "control": {
        "Mbps": {
          "baseline": 200.07678937688516,
          "candidate": 200.07736295409862,
          "change_percent": 0.0002866785373978331
        },
        "CPU_ns_per_byte": {
          "baseline": 2.73228177681582,
          "candidate": 2.7091680262078506,
          "change_percent": -0.8459504727549083
        },
        "CPU_core_percent": {
          "baseline": 6.833276392319642,
          "candidate": 6.775487670873267,
          "change_percent": -0.8456956535715099
        },
        "peak_RSS_MiB": {
          "baseline": 4.28515625,
          "candidate": 4.16015625,
          "change_percent": -2.917046490428443
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.07716605096732,
          "candidate": 200.07592301503706,
          "change_percent": -0.0006212782571846098
        },
        "CPU_ns_per_byte": {
          "baseline": 2.7310505821808007,
          "candidate": 2.7480860620422938,
          "change_percent": 0.6237702066978823
        },
        "CPU_core_percent": {
          "baseline": 6.8302063682467775,
          "candidate": 6.8727657659550285,
          "change_percent": 0.6231055902806482
        },
        "peak_RSS_MiB": {
          "baseline": 4.158202666304578,
          "candidate": 5.097273093837926,
          "change_percent": 22.58356561460979
        }
      },
      "AA_pass": true,
      "timing_pass": true,
      "auxiliary_limited": false,
      "validity": "usable"
    }
  ]
}
```

## Full native x64 · 200 Mbps

[Run](https://github.com/riKirsti/ProtunNext/actions/runs/43159560703) / [artifact](https://github.com/riKirsti/ProtunNext/actions/runs/43159560703/artifacts/13918016515)

```json
{
  "environment": {
    "kernel": "7.0.0-1017-azure",
    "machine": "x86_64",
    "sources": {
      "main": {
        "sha": "4c7cc53a0932fa31579729b603d80a9b9e9ecabf",
        "archive_sha256": "461801494295379a66f8654d22f29fbb58d9036bb65cd8b5fa911c97bc327900",
        "effective_sources_sha256": "805c3dcf6912010b33a410f63b88f25a2be83c10a1d3b44a7abac6edf8d84437",
        "runtime_sources_sha256": "5d448b4aabf1041e3a13920294314097f88a30eeb9a2d4c799724c2c04112fc7"
      },
      "erase": {
        "sha": "905517cc70b2b9b05a3c33cac0c06b71924bfc65",
        "archive_sha256": "2c8fc2cd4e4d8518c581b89b2257958146b4558c8c272d192cfcc1878d4fa7c9",
        "effective_sources_sha256": "684e7c25d2337339f0ff0f6f9527c1b5f5837570be87d7f9e6c023caaeebdc8e",
        "runtime_sources_sha256": "6d1b450fb9abe512604fff33c89583cae6afe82ea5162ce755b27712726f7341",
        "tls_record_prototype_patch_sha256": "3cf2ac9ad7eef47777bd72a4846ddb86d9d2f4660594309d2cdeb9a99e69c5e8",
        "prototype_scope": "NoLocalAgent record-cipher comparison; dual-provider compatibility unresolved"
      }
    },
    "binaries": {
      "main": "3b6f6428768595c75e26fd34e8e6718eaf8e301acefd430679a05e2eb397ed77",
      "erase": "997cbfbe972bcc32da3454744924a6052fc323dbc54940f95349efc35a847759"
    },
    "host_cpus": [
      0,
      1,
      2,
      3
    ],
    "compiler": "rustc 1.99.1 (58cf8f9c3 2026-10-13)\nbinary: rustc\ncommit-hash: 58cf8f9c35b355199727627d07ae34a4e2846d67\ncommit-date: 2026-10-13\nhost: x86_64-unknown-linux-gnu\nrelease: 1.99.1\nLLVM version: 23.1.1\ncargo 1.99.1 (5f94df478 2026-08-27)\nrelease: 1.99.1\ncommit-hash: 5f94df4789f005f9a352888e8355ffc645b7ed0e\ncommit-date: 2026-08-27\nhost: x86_64-unknown-linux-gnu\nlibgit2: 1.9.6 (sys:0.21.0 vendored)\nlibcurl: 8.21.0-DEV (sys:0.4.90+curl-8.21.0 vendored ssl:OpenSSL/3.6.3)\nssl: OpenSSL 3.6.3 9 Jun 2026\nos: Ubuntu 24.4.0 (noble) [64-bit]\ntarget=x86_64-unknown-linux-gnu\nprofile=release thin-LTO codegen-units=1 panic=abort\nRUSTFLAGS=\n",
    "scope": "native diagnostic: actual host kernel; not supported-LTS kernel acceptance"
  },
  "same_runtime_sources": false,
  "observed_samples": 96,
  "expected_samples": 96,
  "errors": [],
  "blocks": [
    {
      "block": "udp-v4-http-1",
      "capacity_ranking": false,
      "rate_controlled": true,
      "control": {
        "Mbps": {
          "baseline": 200.0766459003157,
          "candidate": 200.07680704809007,
          "change_percent": 8.054302071958119e-05
        },
        "CPU_ns_per_byte": {
          "baseline": 1.9840468101822275,
          "candidate": 1.9918919464454998,
          "change_percent": 0.39541084529914894
        },
        "CPU_core_percent": {
          "baseline": 4.961974848783063,
          "candidate": 4.981599795468955,
          "change_percent": 0.39550677470090445
        },
        "peak_RSS_MiB": {
          "baseline": 8.111923258401509,
          "candidate": 7.610513903475901,
          "change_percent": -6.1811402666599236
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.03780627099061,
          "candidate": 200.07622788283604,
          "change_percent": 0.01920717516437076
        },
        "CPU_ns_per_byte": {
          "baseline": 1.9785899306070311,
          "candidate": 1.9508301570512425,
          "change_percent": -1.4030079263201323
        },
        "CPU_core_percent": {
          "baseline": 4.947366323389387,
          "candidate": 4.878881220913656,
          "change_percent": -1.3842739348399968
        },
        "peak_RSS_MiB": {
          "baseline": 7.9614109126902335,
          "candidate": 7.899631308148974,
          "change_percent": -0.7759881410314473
        }
      },
      "AA_pass": true,
      "timing_pass": true,
      "auxiliary_limited": false,
      "validity": "usable"
    },
    {
      "block": "udp-v4-http-8",
      "capacity_ranking": false,
      "rate_controlled": true,
      "control": {
        "Mbps": {
          "baseline": 200.71262392775628,
          "candidate": 200.71262785868745,
          "change_percent": 1.9584872612554705e-06
        },
        "CPU_ns_per_byte": {
          "baseline": 2.186452659150413,
          "candidate": 2.1489766640196195,
          "change_percent": -1.7140089895820276
        },
        "CPU_core_percent": {
          "baseline": 5.485564770577403,
          "candidate": 5.391536422638889,
          "change_percent": -1.714105144521283
        },
        "peak_RSS_MiB": {
          "baseline": 8.525379662435679,
          "candidate": 8.352824901665944,
          "change_percent": -2.0240126258545654
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.71099677995207,
          "candidate": 200.71164867820585,
          "change_percent": 0.0003247944877049491
        },
        "CPU_ns_per_byte": {
          "baseline": 2.170263736937981,
          "candidate": 2.1665070909470265,
          "change_percent": -0.17309628903696694
        },
        "CPU_core_percent": {
          "baseline": 5.444899656824252,
          "candidate": 5.43549918881356,
          "change_percent": -0.17264722222952722
        },
        "peak_RSS_MiB": {
          "baseline": 8.37694743274754,
          "candidate": 8.976287120449634,
          "change_percent": 7.154631117286558
        }
      },
      "AA_pass": true,
      "timing_pass": true,
      "auxiliary_limited": false,
      "validity": "usable"
    },
    {
      "block": "udp-v6-https-1",
      "capacity_ranking": false,
      "rate_controlled": true,
      "control": {
        "Mbps": {
          "baseline": 200.07512081884735,
          "candidate": 200.07666629318345,
          "change_percent": 0.0007724470337810985
        },
        "CPU_ns_per_byte": {
          "baseline": 2.013536664205268,
          "candidate": 1.997667363311288,
          "change_percent": -0.7881307142844296
        },
        "CPU_core_percent": {
          "baseline": 5.035684032833255,
          "candidate": 4.996033243211975,
          "change_percent": -0.7873962973600546
        },
        "peak_RSS_MiB": {
          "baseline": 8.147609767402649,
          "candidate": 8.070347478445496,
          "change_percent": -0.9482816576005804
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.0751274175105,
          "candidate": 200.07543584669284,
          "change_percent": 0.00015415668421780993
        },
        "CPU_ns_per_byte": {
          "baseline": 2.062617104160467,
          "candidate": 2.0313711849540437,
          "change_percent": -1.51486764767913
        },
        "CPU_core_percent": {
          "baseline": 5.1584324828824455,
          "candidate": 5.080300653396609,
          "change_percent": -1.5146428637983766
        },
        "peak_RSS_MiB": {
          "baseline": 8.294855421575697,
          "candidate": 8.816384615890605,
          "change_percent": 6.287381368436651
        }
      },
      "AA_pass": true,
      "timing_pass": true,
      "auxiliary_limited": false,
      "validity": "usable"
    },
    {
      "block": "udp-v6-https-8",
      "capacity_ranking": false,
      "rate_controlled": true,
      "control": {
        "Mbps": {
          "baseline": 200.70687882335704,
          "candidate": 200.703690741817,
          "change_percent": -0.0015884266442278694
        },
        "CPU_ns_per_byte": {
          "baseline": 2.3155068397478638,
          "candidate": 2.3233093774216496,
          "change_percent": 0.33696888905045963
        },
        "CPU_core_percent": {
          "baseline": 5.809182899548998,
          "candidate": 5.828660472100054,
          "change_percent": 0.3352893666434431
        },
        "peak_RSS_MiB": {
          "baseline": 8.729965379783962,
          "candidate": 8.299710408869322,
          "change_percent": -4.928484274530865
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.70539951160308,
          "candidate": 200.70096896626066,
          "change_percent": -0.00220748687040917
        },
        "CPU_ns_per_byte": {
          "baseline": 2.350491797114773,
          "candidate": 2.3532683267285828,
          "change_percent": 0.11812547557994435
        },
        "CPU_core_percent": {
          "baseline": 5.896903723307772,
          "candidate": 5.903739402778693,
          "change_percent": 0.1159198079477397
        },
        "peak_RSS_MiB": {
          "baseline": 8.586395112634492,
          "candidate": 9.044931786123378,
          "change_percent": 5.340269897598482
        }
      },
      "AA_pass": true,
      "timing_pass": true,
      "auxiliary_limited": false,
      "validity": "usable"
    },
    {
      "block": "tcp-v4-http-1",
      "capacity_ranking": false,
      "rate_controlled": true,
      "control": {
        "Mbps": {
          "baseline": 200.07412816292407,
          "candidate": 200.07637394946252,
          "change_percent": 0.0011224772333395805
        },
        "CPU_ns_per_byte": {
          "baseline": 2.033456754473934,
          "candidate": 2.042109399323435,
          "change_percent": 0.4255140823852699
        },
        "CPU_core_percent": {
          "baseline": 5.085479318125191,
          "candidate": 5.107178348371203,
          "change_percent": 0.42668603859372656
        },
        "peak_RSS_MiB": {
          "baseline": 7.390392727256514,
          "candidate": 7.138264405693161,
          "change_percent": -3.411568652278496
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.0777467714065,
          "candidate": 200.07605366597596,
          "change_percent": -0.0008462237594430988
        },
        "CPU_ns_per_byte": {
          "baseline": 1.9829144611753327,
          "candidate": 1.978792614621721,
          "change_percent": -0.2078680969005875
        },
        "CPU_core_percent": {
          "baseline": 4.959165805881552,
          "candidate": 4.948820381967247,
          "change_percent": -0.20861218033960682
        },
        "peak_RSS_MiB": {
          "baseline": 7.837959492673401,
          "candidate": 7.791013421674919,
          "change_percent": -0.5989578160280784
        }
      },
      "AA_pass": true,
      "timing_pass": true,
      "auxiliary_limited": false,
      "validity": "usable"
    },
    {
      "block": "tcp-v4-http-8",
      "capacity_ranking": false,
      "rate_controlled": true,
      "control": {
        "Mbps": {
          "baseline": 200.711281788624,
          "candidate": 200.7126640479417,
          "change_percent": 0.0006886804296168947
        },
        "CPU_ns_per_byte": {
          "baseline": 2.1324731035349966,
          "candidate": 2.146117710326937,
          "change_percent": 0.6398489514039651
        },
        "CPU_core_percent": {
          "baseline": 5.350099045007265,
          "candidate": 5.3843723629019875,
          "change_percent": 0.6406109047029096
        },
        "peak_RSS_MiB": {
          "baseline": 7.729046459925539,
          "candidate": 8.247863492885026,
          "change_percent": 6.712561965431441
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.70855872308425,
          "candidate": 200.71223355611224,
          "change_percent": 0.0018309299071983531
        },
        "CPU_ns_per_byte": {
          "baseline": 2.086925186349208,
          "candidate": 2.1568909116452746,
          "change_percent": 3.352574675590647
        },
        "CPU_core_percent": {
          "baseline": 5.235749501578498,
          "candidate": 5.411378056296707,
          "change_percent": 3.354410952343301
        },
        "peak_RSS_MiB": {
          "baseline": 8.149512303276193,
          "candidate": 8.409828757267444,
          "change_percent": 3.19425806482434
        }
      },
      "AA_pass": true,
      "timing_pass": true,
      "auxiliary_limited": false,
      "validity": "usable"
    },
    {
      "block": "tcp-v6-https-1",
      "capacity_ranking": false,
      "rate_controlled": true,
      "control": {
        "Mbps": {
          "baseline": 200.0749222243413,
          "candidate": 200.07372095117094,
          "change_percent": -0.0006004116642865931
        },
        "CPU_ns_per_byte": {
          "baseline": 2.101216589963521,
          "candidate": 2.1555371409276742,
          "change_percent": 2.585195225642889
        },
        "CPU_core_percent": {
          "baseline": 5.254962137951238,
          "candidate": 5.390777844814132,
          "change_percent": 2.5845230336111413
        },
        "peak_RSS_MiB": {
          "baseline": 7.40037799591493,
          "candidate": 7.731876964018423,
          "change_percent": 4.479486970618041
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.07531989995505,
          "candidate": 200.0742905586891,
          "change_percent": -0.0005144768812304079
        },
        "CPU_ns_per_byte": {
          "baseline": 2.037866242016937,
          "candidate": 2.043442470737035,
          "change_percent": 0.273630751868148
        },
        "CPU_core_percent": {
          "baseline": 5.0965413022359405,
          "candidate": 5.110462178536778,
          "change_percent": 0.27314359828165546
        },
        "peak_RSS_MiB": {
          "baseline": 7.417376309280563,
          "candidate": 7.829540010584512,
          "change_percent": 5.556731708329976
        }
      },
      "AA_pass": true,
      "timing_pass": true,
      "auxiliary_limited": false,
      "validity": "usable"
    },
    {
      "block": "tcp-v6-https-8",
      "capacity_ranking": false,
      "rate_controlled": true,
      "control": {
        "Mbps": {
          "baseline": 200.71091484564565,
          "candidate": 200.71002275236114,
          "change_percent": -0.0004444667521918966
        },
        "CPU_ns_per_byte": {
          "baseline": 2.225734013705277,
          "candidate": 2.2203393869153403,
          "change_percent": -0.24237517855766333
        },
        "CPU_core_percent": {
          "baseline": 5.584061752576111,
          "candidate": 5.570509106210857,
          "change_percent": -0.242702301044595
        },
        "peak_RSS_MiB": {
          "baseline": 7.869012403165463,
          "candidate": 8.132811561899564,
          "change_percent": 3.3523795009902546
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.7107999414195,
          "candidate": 200.70695224372474,
          "change_percent": -0.0019170357030584562
        },
        "CPU_ns_per_byte": {
          "baseline": 2.179646782551514,
          "candidate": 2.202898366698947,
          "change_percent": 1.066759271895168
        },
        "CPU_core_percent": {
          "baseline": 5.468439744695189,
          "candidate": 5.526667539261577,
          "change_percent": 1.0647972234287373
        },
        "peak_RSS_MiB": {
          "baseline": 7.564210808554318,
          "candidate": 8.175526490596031,
          "change_percent": 8.081684891044816
        }
      },
      "AA_pass": true,
      "timing_pass": true,
      "auxiliary_limited": false,
      "validity": "usable"
    },
    {
      "block": "stealth-v4-http-1",
      "capacity_ranking": false,
      "rate_controlled": true,
      "control": {
        "Mbps": {
          "baseline": 200.07787604554005,
          "candidate": 200.07565284255665,
          "change_percent": -0.0011111688245302176
        },
        "CPU_ns_per_byte": {
          "baseline": 2.048294820530681,
          "candidate": 2.048952303725838,
          "change_percent": 0.03209905080885367
        },
        "CPU_core_percent": {
          "baseline": 5.122690756701,
          "candidate": 5.124261411488575,
          "change_percent": 0.03066073792412638
        },
        "peak_RSS_MiB": {
          "baseline": 5.128904762471224,
          "candidate": 5.136338507789529,
          "change_percent": 0.14493825997117238
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.07729428340116,
          "candidate": 200.07727010437716,
          "change_percent": -1.2084841549686587e-05
        },
        "CPU_ns_per_byte": {
          "baseline": 2.05305714097245,
          "candidate": 2.05966022534284,
          "change_percent": 0.3216220454177021
        },
        "CPU_core_percent": {
          "baseline": 5.13458267746017,
          "candidate": 5.151097340860265,
          "change_percent": 0.32163594273380003
        },
        "peak_RSS_MiB": {
          "baseline": 5.133195975971926,
          "candidate": 6.9524929488008125,
          "change_percent": 35.441798469118815
        }
      },
      "AA_pass": true,
      "timing_pass": true,
      "auxiliary_limited": false,
      "validity": "usable"
    },
    {
      "block": "stealth-v4-http-8",
      "capacity_ranking": false,
      "rate_controlled": true,
      "control": {
        "Mbps": {
          "baseline": 200.7133364046966,
          "candidate": 200.71287409523282,
          "change_percent": -0.00023033320658560896
        },
        "CPU_ns_per_byte": {
          "baseline": 2.2720934369705432,
          "candidate": 2.27148270557051,
          "change_percent": -0.02687967801393576
        },
        "CPU_core_percent": {
          "baseline": 5.700445662158018,
          "candidate": 5.698903353524379,
          "change_percent": -0.027055930799901873
        },
        "peak_RSS_MiB": {
          "baseline": 6.396266992966483,
          "candidate": 6.352794795126345,
          "change_percent": -0.6796495188199914
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.71319505304868,
          "candidate": 200.71217330023677,
          "change_percent": -0.000509061106634956
        },
        "CPU_ns_per_byte": {
          "baseline": 2.280127513577491,
          "candidate": 2.279863293334091,
          "change_percent": -0.011587959086789557
        },
        "CPU_core_percent": {
          "baseline": 5.72059614830621,
          "candidate": 5.719903118802325,
          "change_percent": -0.012114637809046425
        },
        "peak_RSS_MiB": {
          "baseline": 6.509484047784385,
          "candidate": 8.282042267526393,
          "change_percent": 27.23039501641191
        }
      },
      "AA_pass": true,
      "timing_pass": true,
      "auxiliary_limited": false,
      "validity": "usable"
    },
    {
      "block": "stealth-v6-https-1",
      "capacity_ranking": false,
      "rate_controlled": true,
      "control": {
        "Mbps": {
          "baseline": 200.07619481255335,
          "candidate": 200.07681372201293,
          "change_percent": 0.00030933688046630436
        },
        "CPU_ns_per_byte": {
          "baseline": 2.181923212354593,
          "candidate": 2.184824821866459,
          "change_percent": 0.13298403424266692
        },
        "CPU_core_percent": {
          "baseline": 5.456842769518923,
          "candidate": 5.464104797114209,
          "change_percent": 0.13308112221686041
        },
        "peak_RSS_MiB": {
          "baseline": 5.0718561065190295,
          "candidate": 5.13671875,
          "change_percent": 1.2788738899276009
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.07370803852874,
          "candidate": 200.07557447449395,
          "change_percent": 0.0009328741809699892
        },
        "CPU_ns_per_byte": {
          "baseline": 2.203903520450189,
          "candidate": 2.22127579587752,
          "change_percent": 0.7882502689492643
        },
        "CPU_core_percent": {
          "baseline": 5.511739220900391,
          "candidate": 5.555241654848344,
          "change_percent": 0.7892687263394649
        },
        "peak_RSS_MiB": {
          "baseline": 5.163990106811029,
          "candidate": 6.942587696657584,
          "change_percent": 34.442312108628514
        }
      },
      "AA_pass": true,
      "timing_pass": true,
      "auxiliary_limited": false,
      "validity": "usable"
    },
    {
      "block": "stealth-v6-https-8",
      "capacity_ranking": false,
      "rate_controlled": true,
      "control": {
        "Mbps": {
          "baseline": 200.70916203983916,
          "candidate": 200.706376355099,
          "change_percent": -0.0013879210654099694
        },
        "CPU_ns_per_byte": {
          "baseline": 2.3570318667524814,
          "candidate": 2.3966353602512696,
          "change_percent": 1.6802273256217637
        },
        "CPU_core_percent": {
          "baseline": 5.9134290056539545,
          "candidate": 6.012702003408325,
          "change_percent": 1.6787721245905551
        },
        "peak_RSS_MiB": {
          "baseline": 5.2597652623700215,
          "candidate": 5.4178786266837475,
          "change_percent": 3.0060916490878054
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.70700297061512,
          "candidate": 200.70747085014864,
          "change_percent": 0.00023311569929962417
        },
        "CPU_ns_per_byte": {
          "baseline": 2.363626138317458,
          "candidate": 2.3974967241393386,
          "change_percent": 1.4329925224972895
        },
        "CPU_core_percent": {
          "baseline": 5.929911728726476,
          "candidate": 6.0148919900846405,
          "change_percent": 1.4330780160940915
        },
        "peak_RSS_MiB": {
          "baseline": 5.13671875,
          "candidate": 6.984129216522697,
          "change_percent": 35.96479691481449
        }
      },
      "AA_pass": true,
      "timing_pass": true,
      "auxiliary_limited": false,
      "validity": "usable"
    }
  ]
}
```

## Full native arm · 200 Mbps

[Run](https://github.com/riKirsti/ProtunNext/actions/runs/43159560703) / [artifact](https://github.com/riKirsti/ProtunNext/actions/runs/43159560703/artifacts/13918390720)

```json
{
  "environment": {
    "kernel": "7.0.0-1017-azure",
    "machine": "aarch64",
    "sources": {
      "main": {
        "sha": "4c7cc53a0932fa31579729b603d80a9b9e9ecabf",
        "archive_sha256": "461801494295379a66f8654d22f29fbb58d9036bb65cd8b5fa911c97bc327900",
        "effective_sources_sha256": "805c3dcf6912010b33a410f63b88f25a2be83c10a1d3b44a7abac6edf8d84437",
        "runtime_sources_sha256": "5d448b4aabf1041e3a13920294314097f88a30eeb9a2d4c799724c2c04112fc7"
      },
      "erase": {
        "sha": "905517cc70b2b9b05a3c33cac0c06b71924bfc65",
        "archive_sha256": "2c8fc2cd4e4d8518c581b89b2257958146b4558c8c272d192cfcc1878d4fa7c9",
        "effective_sources_sha256": "684e7c25d2337339f0ff0f6f9527c1b5f5837570be87d7f9e6c023caaeebdc8e",
        "runtime_sources_sha256": "6d1b450fb9abe512604fff33c89583cae6afe82ea5162ce755b27712726f7341",
        "tls_record_prototype_patch_sha256": "3cf2ac9ad7eef47777bd72a4846ddb86d9d2f4660594309d2cdeb9a99e69c5e8",
        "prototype_scope": "NoLocalAgent record-cipher comparison; dual-provider compatibility unresolved"
      }
    },
    "binaries": {
      "main": "7b1d9f62cb1631f1ae7b4b4e341d3780a01e902e594198743f0da259449969b5",
      "erase": "bd96cebd9d85155f9929cfdff98464733444bb57267bfd212959ed0a4008b796"
    },
    "host_cpus": [
      0,
      1,
      2,
      3
    ],
    "compiler": "rustc 1.99.1 (58cf8f9c3 2026-10-13)\nbinary: rustc\ncommit-hash: 58cf8f9c35b355199727627d07ae34a4e2846d67\ncommit-date: 2026-10-13\nhost: aarch64-unknown-linux-gnu\nrelease: 1.99.1\nLLVM version: 23.1.1\ncargo 1.99.1 (5f94df478 2026-08-27)\nrelease: 1.99.1\ncommit-hash: 5f94df4789f005f9a352888e8355ffc645b7ed0e\ncommit-date: 2026-08-27\nhost: aarch64-unknown-linux-gnu\nlibgit2: 1.9.6 (sys:0.21.0 vendored)\nlibcurl: 8.21.0-DEV (sys:0.4.90+curl-8.21.0 vendored ssl:OpenSSL/3.6.3)\nssl: OpenSSL 3.6.3 9 Jun 2026\nos: Ubuntu 24.4.0 (noble) [64-bit]\ntarget=aarch64-unknown-linux-gnu\nprofile=release thin-LTO codegen-units=1 panic=abort\nRUSTFLAGS=\n",
    "scope": "native diagnostic: actual host kernel; not supported-LTS kernel acceptance"
  },
  "same_runtime_sources": false,
  "observed_samples": 96,
  "expected_samples": 96,
  "errors": [],
  "blocks": [
    {
      "block": "udp-v4-http-1",
      "capacity_ranking": false,
      "rate_controlled": true,
      "control": {
        "Mbps": {
          "baseline": 200.0788331905779,
          "candidate": 200.07826652023863,
          "change_percent": -0.00028322353257115296
        },
        "CPU_ns_per_byte": {
          "baseline": 2.3090958557316426,
          "candidate": 2.3008096565442426,
          "change_percent": -0.35885037716524293
        },
        "CPU_core_percent": {
          "baseline": 5.774973483896716,
          "candidate": 5.754232273784204,
          "change_percent": -0.3591568025437253
        },
        "peak_RSS_MiB": {
          "baseline": 7.136390549035231,
          "candidate": 7.04352769204699,
          "change_percent": -1.301258056858945
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.07916934471635,
          "candidate": 200.07843598841276,
          "change_percent": -0.00036653306088041404
        },
        "CPU_ns_per_byte": {
          "baseline": 2.3322870689970028,
          "candidate": 2.292555016587334,
          "change_percent": -1.7035661234770627
        },
        "CPU_core_percent": {
          "baseline": 5.83298053632801,
          "candidate": 5.733593245951046,
          "change_percent": -1.7038851708483516
        },
        "peak_RSS_MiB": {
          "baseline": 7.847407366013572,
          "candidate": 7.428707792267146,
          "change_percent": -5.33551470208844
        }
      },
      "AA_pass": true,
      "timing_pass": true,
      "auxiliary_limited": false,
      "validity": "usable"
    },
    {
      "block": "udp-v4-http-8",
      "capacity_ranking": false,
      "rate_controlled": true,
      "control": {
        "Mbps": {
          "baseline": 200.72315363042307,
          "candidate": 200.7225441122426,
          "change_percent": -0.0003036611220252894
        },
        "CPU_ns_per_byte": {
          "baseline": 2.219849268020464,
          "candidate": 2.206191277393136,
          "change_percent": -0.6152665779648792
        },
        "CPU_core_percent": {
          "baseline": 5.569644629756642,
          "candidate": 5.535360659276141,
          "change_percent": -0.6155504122710709
        },
        "peak_RSS_MiB": {
          "baseline": 7.710751485851589,
          "candidate": 7.7185276014898685,
          "change_percent": 0.10084770145357691
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.7231483971818,
          "candidate": 200.72212920721466,
          "change_percent": -0.0005077590578217439
        },
        "CPU_ns_per_byte": {
          "baseline": 2.213004397420737,
          "candidate": 2.214157329366846,
          "change_percent": 0.052098041352866
        },
        "CPU_core_percent": {
          "baseline": 5.5524715306686305,
          "candidate": 5.555335802697131,
          "change_percent": 0.05158553290511847
        },
        "peak_RSS_MiB": {
          "baseline": 7.658202875940446,
          "candidate": 7.970442528590399,
          "change_percent": 4.077192230450155
        }
      },
      "AA_pass": true,
      "timing_pass": true,
      "auxiliary_limited": false,
      "validity": "usable"
    },
    {
      "block": "udp-v6-https-1",
      "capacity_ranking": false,
      "rate_controlled": true,
      "control": {
        "Mbps": {
          "baseline": 200.07790676566555,
          "candidate": 200.07805986388712,
          "change_percent": 7.651930393759443e-05
        },
        "CPU_ns_per_byte": {
          "baseline": 2.546893052274952,
          "candidate": 2.5312411602447864,
          "change_percent": -0.6145484599828444
        },
        "CPU_core_percent": {
          "baseline": 6.369663473546092,
          "candidate": 6.330525894758632,
          "change_percent": -0.6144371511933477
        },
        "peak_RSS_MiB": {
          "baseline": 6.909025575246778,
          "candidate": 7.3473904291363255,
          "change_percent": 6.344814462115944
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.0774820892265,
          "candidate": 200.07751329294598,
          "change_percent": 1.5595817770552856e-05
        },
        "CPU_ns_per_byte": {
          "baseline": 2.598143733222782,
          "candidate": 2.5553412558521407,
          "change_percent": -1.6474253068958666
        },
        "CPU_core_percent": {
          "baseline": 6.497826461763511,
          "candidate": 6.3907785155398145,
          "change_percent": -1.6474423694387785
        },
        "peak_RSS_MiB": {
          "baseline": 7.136680264674556,
          "candidate": 7.089160392342619,
          "change_percent": -0.6658540185294881
        }
      },
      "AA_pass": true,
      "timing_pass": true,
      "auxiliary_limited": false,
      "validity": "usable"
    },
    {
      "block": "udp-v6-https-8",
      "capacity_ranking": false,
      "rate_controlled": true,
      "control": {
        "Mbps": {
          "baseline": 200.71766258403827,
          "candidate": 200.71600157292184,
          "change_percent": -0.000827536099734516
        },
        "CPU_ns_per_byte": {
          "baseline": 2.319923210526668,
          "candidate": 2.3188192529310685,
          "change_percent": -0.047585954163920086
        },
        "CPU_core_percent": {
          "baseline": 5.820568209225077,
          "candidate": 5.81775190077288,
          "change_percent": -0.048385455697153024
        },
        "peak_RSS_MiB": {
          "baseline": 7.966503589615671,
          "candidate": 7.720379738436709,
          "change_percent": -3.089483967593809
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.71723310523782,
          "candidate": 200.7168002292575,
          "change_percent": -0.00021566458127120214
        },
        "CPU_ns_per_byte": {
          "baseline": 2.3339030077546954,
          "candidate": 2.3155546494960824,
          "change_percent": -0.7861662715909046
        },
        "CPU_core_percent": {
          "baseline": 5.8556340858009985,
          "candidate": 5.809579569879231,
          "change_percent": -0.7864992116471625
        },
        "peak_RSS_MiB": {
          "baseline": 7.910156249999999,
          "candidate": 8.03515625,
          "change_percent": 1.5802469135802522
        }
      },
      "AA_pass": true,
      "timing_pass": true,
      "auxiliary_limited": false,
      "validity": "usable"
    },
    {
      "block": "tcp-v4-http-1",
      "capacity_ranking": false,
      "rate_controlled": true,
      "control": {
        "Mbps": {
          "baseline": 200.07951277332182,
          "candidate": 200.07925274710422,
          "change_percent": -0.00012996144083121663
        },
        "CPU_ns_per_byte": {
          "baseline": 2.1972745185436424,
          "candidate": 2.163691635460744,
          "change_percent": -1.5283881371890295
        },
        "CPU_core_percent": {
          "baseline": 5.495328609564508,
          "candidate": 5.41133392085891,
          "change_percent": -1.5284743583742544
        },
        "peak_RSS_MiB": {
          "baseline": 6.496214718131052,
          "candidate": 6.425555657134229,
          "change_percent": -1.0876958977296791
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.07858120327563,
          "candidate": 200.07944592572372,
          "change_percent": 0.0004321914134353122
        },
        "CPU_ns_per_byte": {
          "baseline": 2.1803939723257955,
          "candidate": 2.15392316052998,
          "change_percent": -1.214038019357555
        },
        "CPU_core_percent": {
          "baseline": 5.453087639771333,
          "candidate": 5.386907376381504,
          "change_percent": -1.213629190683696
        },
        "peak_RSS_MiB": {
          "baseline": 6.346897791225697,
          "candidate": 6.088233681082613,
          "change_percent": -4.0754415566715885
        }
      },
      "AA_pass": true,
      "timing_pass": true,
      "auxiliary_limited": false,
      "validity": "usable"
    },
    {
      "block": "tcp-v4-http-8",
      "capacity_ranking": false,
      "rate_controlled": true,
      "control": {
        "Mbps": {
          "baseline": 200.72351111034405,
          "candidate": 200.7238238853212,
          "change_percent": 0.00015582378736223035
        },
        "CPU_ns_per_byte": {
          "baseline": 2.147704728733781,
          "candidate": 2.1574095497073214,
          "change_percent": 0.4518694233756282
        },
        "CPU_core_percent": {
          "baseline": 5.388641603783849,
          "candidate": 5.412992129645794,
          "change_percent": 0.4518861644991601
        },
        "peak_RSS_MiB": {
          "baseline": 6.907657255521992,
          "candidate": 7.0141360735587694,
          "change_percent": 1.5414606442966994
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.72471329667147,
          "candidate": 200.72390624800107,
          "change_percent": -0.00040206741718229466
        },
        "CPU_ns_per_byte": {
          "baseline": 2.14975596298463,
          "candidate": 2.1403763874592485,
          "change_percent": -0.43630885025477406
        },
        "CPU_core_percent": {
          "baseline": 5.393819676446939,
          "candidate": 5.370266259680606,
          "change_percent": -0.43667416004252724
        },
        "peak_RSS_MiB": {
          "baseline": 6.964113069846681,
          "candidate": 7.338969521180239,
          "change_percent": 5.382687609662984
        }
      },
      "AA_pass": true,
      "timing_pass": true,
      "auxiliary_limited": false,
      "validity": "usable"
    },
    {
      "block": "tcp-v6-https-1",
      "capacity_ranking": false,
      "rate_controlled": true,
      "control": {
        "Mbps": {
          "baseline": 200.077684173999,
          "candidate": 200.07830005337414,
          "change_percent": 0.0003078201238126965
        },
        "CPU_ns_per_byte": {
          "baseline": 2.38816343669736,
          "candidate": 2.390705241348414,
          "change_percent": 0.1064334463879435
        },
        "CPU_core_percent": {
          "baseline": 5.972681733011813,
          "candidate": 5.9790552904395975,
          "change_percent": 0.10671182079831087
        },
        "peak_RSS_MiB": {
          "baseline": 6.47235449281512,
          "candidate": 6.296942850130728,
          "change_percent": -2.710167418659082
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.07790829409953,
          "candidate": 200.07854835234582,
          "change_percent": 0.00031990450706231854
        },
        "CPU_ns_per_byte": {
          "baseline": 2.336064231859577,
          "candidate": 2.4030848703334056,
          "change_percent": 2.8689552949697106
        },
        "CPU_core_percent": {
          "baseline": 5.842391384628719,
          "candidate": 6.010024475285351,
          "change_percent": 2.869254721579817
        },
        "peak_RSS_MiB": {
          "baseline": 6.247995283957836,
          "candidate": 6.233905057476012,
          "change_percent": -0.22551595898290255
        }
      },
      "AA_pass": true,
      "timing_pass": true,
      "auxiliary_limited": false,
      "validity": "usable"
    },
    {
      "block": "tcp-v6-https-8",
      "capacity_ranking": false,
      "rate_controlled": true,
      "control": {
        "Mbps": {
          "baseline": 200.7183404370491,
          "candidate": 200.7179723219181,
          "change_percent": -0.00018339885144991186
        },
        "CPU_ns_per_byte": {
          "baseline": 2.221673249902425,
          "candidate": 2.232277296800092,
          "change_percent": 0.47730002142001293
        },
        "CPU_core_percent": {
          "baseline": 5.574085209760368,
          "candidate": 5.600680174308487,
          "change_percent": 0.4771180121457608
        },
        "peak_RSS_MiB": {
          "baseline": 6.780549043913337,
          "candidate": 7.095179207966776,
          "change_percent": 4.640187129622952
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.71732992610518,
          "candidate": 200.719036378141,
          "change_percent": 0.0008501767318591646
        },
        "CPU_ns_per_byte": {
          "baseline": 2.216155353476205,
          "candidate": 2.2315748856679885,
          "change_percent": 0.6957784871713502
        },
        "CPU_core_percent": {
          "baseline": 5.560213569761046,
          "candidate": 5.598946796863226,
          "change_percent": 0.6966140170015844
        },
        "peak_RSS_MiB": {
          "baseline": 6.744098485333886,
          "candidate": 7.41015625,
          "change_percent": 9.876157148573107
        }
      },
      "AA_pass": true,
      "timing_pass": true,
      "auxiliary_limited": false,
      "validity": "usable"
    },
    {
      "block": "stealth-v4-http-1",
      "capacity_ranking": false,
      "rate_controlled": true,
      "control": {
        "Mbps": {
          "baseline": 200.07796652847676,
          "candidate": 200.07783555704611,
          "change_percent": -6.54601968075319e-05
        },
        "CPU_ns_per_byte": {
          "baseline": 2.6494726039688636,
          "candidate": 2.6293713918086965,
          "change_percent": -0.7586872998820882
        },
        "CPU_core_percent": {
          "baseline": 6.626213566010022,
          "candidate": 6.575938353789851,
          "change_percent": -0.7587321434682437
        },
        "peak_RSS_MiB": {
          "baseline": 4.152336400510899,
          "candidate": 4.222193689974923,
          "change_percent": 1.6823610306580417
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.07801876060833,
          "candidate": 200.0761870266312,
          "change_percent": -0.0009155098538382589
        },
        "CPU_ns_per_byte": {
          "baseline": 2.6351052596016484,
          "candidate": 2.687320729572377,
          "change_percent": 1.9815326078709283
        },
        "CPU_core_percent": {
          "baseline": 6.590285601180334,
          "candidate": 6.720812654391429,
          "change_percent": 1.9805978239807676
        },
        "peak_RSS_MiB": {
          "baseline": 4.222193689974923,
          "candidate": 5.033604420483802,
          "change_percent": 19.217752431288805
        }
      },
      "AA_pass": true,
      "timing_pass": true,
      "auxiliary_limited": false,
      "validity": "usable"
    },
    {
      "block": "stealth-v4-http-8",
      "capacity_ranking": false,
      "rate_controlled": true,
      "control": {
        "Mbps": {
          "baseline": 200.71462900281205,
          "candidate": 200.7147434006937,
          "change_percent": 5.69952883999747e-05
        },
        "CPU_ns_per_byte": {
          "baseline": 2.56605848500743,
          "candidate": 2.582870304995714,
          "change_percent": 0.6551612165704324
        },
        "CPU_core_percent": {
          "baseline": 6.43801796006899,
          "candidate": 6.480199868571461,
          "change_percent": 0.6552002303208626
        },
        "peak_RSS_MiB": {
          "baseline": 5.404712965663481,
          "candidate": 5.261705700150855,
          "change_percent": -2.645973364749643
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.71475645919818,
          "candidate": 200.71419000652574,
          "change_percent": -0.0002822177514238966
        },
        "CPU_ns_per_byte": {
          "baseline": 2.589733480151561,
          "candidate": 2.6036756750645194,
          "change_percent": 0.5383640834014436
        },
        "CPU_core_percent": {
          "baseline": 6.497419136537097,
          "candidate": 6.532375041897232,
          "change_percent": 0.5379967741894198
        },
        "peak_RSS_MiB": {
          "baseline": 5.4900742178036355,
          "candidate": 6.284245777745901,
          "change_percent": 14.465588777777615
        }
      },
      "AA_pass": true,
      "timing_pass": true,
      "auxiliary_limited": false,
      "validity": "usable"
    },
    {
      "block": "stealth-v6-https-1",
      "capacity_ranking": false,
      "rate_controlled": true,
      "control": {
        "Mbps": {
          "baseline": 200.07568333321456,
          "candidate": 200.0759287364138,
          "change_percent": 0.00012265518485321536
        },
        "CPU_ns_per_byte": {
          "baseline": 2.7997858789466936,
          "candidate": 2.8082376618140756,
          "change_percent": 0.30187247285358154
        },
        "CPU_core_percent": {
          "baseline": 7.002055275389792,
          "candidate": 7.023203455233403,
          "change_percent": 0.3020281761833665
        },
        "peak_RSS_MiB": {
          "baseline": 4.28515625,
          "candidate": 4.222193689974923,
          "change_percent": -1.4693177179963213
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.07589566820442,
          "candidate": 200.07488593279194,
          "change_percent": -0.0005046761925586729
        },
        "CPU_ns_per_byte": {
          "baseline": 2.859865563554106,
          "candidate": 2.8750524735970195,
          "change_percent": 0.5310358023976391
        },
        "CPU_core_percent": {
          "baseline": 7.152318914523851,
          "candidate": 7.190263922165016,
          "change_percent": 0.5305273449721648
        },
        "peak_RSS_MiB": {
          "baseline": 4.341740205011408,
          "candidate": 4.969120035847802,
          "change_percent": 14.449962485370449
        }
      },
      "AA_pass": true,
      "timing_pass": true,
      "auxiliary_limited": false,
      "validity": "usable"
    },
    {
      "block": "stealth-v6-https-8",
      "capacity_ranking": false,
      "rate_controlled": true,
      "control": {
        "Mbps": {
          "baseline": 200.70960201308185,
          "candidate": 200.7102604847971,
          "change_percent": 0.0003280718553710926
        },
        "CPU_ns_per_byte": {
          "baseline": 2.6818229589750744,
          "candidate": 2.67330165845468,
          "change_percent": -0.3177428432356688
        },
        "CPU_core_percent": {
          "baseline": 6.728286810567698,
          "candidate": 6.706931713608765,
          "change_percent": -0.31739278601191634
        },
        "peak_RSS_MiB": {
          "baseline": 4.472219547457845,
          "candidate": 4.45659301636088,
          "change_percent": -0.3494133266746302
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.70902772762884,
          "candidate": 200.70874559956044,
          "change_percent": -0.00014056570927234446
        },
        "CPU_ns_per_byte": {
          "baseline": 2.659623378280967,
          "candidate": 2.724849839086331,
          "change_percent": 2.4524698247886167
        },
        "CPU_core_percent": {
          "baseline": 6.6725732027660865,
          "candidate": 6.836210077756376,
          "change_percent": 2.452380363882023
        },
        "peak_RSS_MiB": {
          "baseline": 4.279292863559279,
          "candidate": 5.4683468043331835,
          "change_percent": 27.78622493682088
        }
      },
      "AA_pass": true,
      "timing_pass": true,
      "auxiliary_limited": false,
      "validity": "usable"
    }
  ]
}
```
