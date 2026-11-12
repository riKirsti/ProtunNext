# Stage 4: GitHub Actions evidence

Baseline / prototype values retain their actual source identities. No cohorts are pooled. Full logs, individual samples, binaries and system descriptions remain in the linked artifacts. Paths internal to disposable builders are omitted here.

## Initial Linux 6.12 x64 unpaced

[Run](https://github.com/riKirsti/ProtunNext/actions/runs/42755533441) / [artifact](https://github.com/riKirsti/ProtunNext/actions/runs/42755533441/artifacts/13735602041)

```json
{
  "environment": {
    "kernel": "6.12.115",
    "machine": "x86_64",
    "sources": {
      "main": {
        "sha": "a44d520c337611f59d4c7186971919d57de36df8",
        "archive_sha256": "1ee2cfc21b363c16b1b81aec670b5a20b5590509d97d1de12e8c55d76f0dda6f",
        "effective_sources_sha256": "86119872b69a98b18b7982cb6bacf67fbfe773c0633ffe5e473b9eeb7d34794c",
        "runtime_sources_sha256": "5d448b4aabf1041e3a13920294314097f88a30eeb9a2d4c799724c2c04112fc7"
      },
      "erase": {
        "sha": "7da4431e14d70ba955f2fe8de615efb9f486160f",
        "archive_sha256": "9a959094922077c91c65ee9a8faf0859cde43b59df84413193593abd5c182a22",
        "effective_sources_sha256": "fcfa9aff4a859365697ba2b13b0b1cc80073c3bd2a3d15fd86fdfc1221f0e5d5",
        "runtime_sources_sha256": "2d81fbdfc0c767b11c7a6262969ae32198036286cd4e951bde954b7d42c8934e"
      }
    },
    "binaries": {
      "main": "26ea5baea840934f0f56e18a98ca4991e106314897bd93ed79ffe87a6c0fa9e6",
      "erase": "042ba197dfd6f65f3125bf31fa9b00aee3a5355a127c58fa899437122628ef3b"
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
      "prepare.py": "b48b568f1445237b5c64d2f72ec06f201ca9d4c6a67f9c5600d815e94f019c0d"
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
    "scope": "actual-kernel KVM; pinned logical CPUs, not physical-core or interrupt isolation",
    "cpu_layout": {
      "client": [
        2,
        3
      ],
      "peer": 0,
      "server": 0,
      "load": 1
    }
  },
  "same_runtime_sources": false,
  "observed_samples": 8,
  "expected_samples": 8,
  "errors": [],
  "blocks": [
    {
      "block": "udp-v4-http-1",
      "capacity_ranking": false,
      "rate_controlled": false,
      "control": {
        "Mbps": {
          "baseline": 1910.2745623191133,
          "candidate": 1908.324069947791,
          "change_percent": -0.10210534180774555
        },
        "CPU_ns_per_byte": {
          "baseline": 2.878481603906229,
          "candidate": 2.9014824556492522,
          "change_percent": 0.7990619676641275
        },
        "CPU_core_percent": {
          "baseline": 68.73241627588149,
          "candidate": 69.2109533851894,
          "change_percent": 0.6962320477533313
        },
        "peak_RSS_MiB": {
          "baseline": 3.9936205537022573,
          "candidate": 3.9862472623167924,
          "change_percent": -0.1846267387278333
        }
      },
      "result": {
        "Mbps": {
          "baseline": 1906.4110427165995,
          "candidate": 1564.9322646061407,
          "change_percent": -17.912127576845016
        },
        "CPU_ns_per_byte": {
          "baseline": 2.881073857768425,
          "candidate": 5.507236824835995,
          "change_percent": 91.15222645148361
        },
        "CPU_core_percent": {
          "baseline": 68.65539443064257,
          "candidate": 107.72904723392588,
          "change_percent": 56.912720591470055
        },
        "peak_RSS_MiB": {
          "baseline": 4.019225647396212,
          "candidate": 4.251715818624729,
          "change_percent": 5.784451822930925
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

## Initial native x64 200 Mbps

[Run](https://github.com/riKirsti/ProtunNext/actions/runs/42755530103) / [artifact](https://github.com/riKirsti/ProtunNext/actions/runs/42755530103/artifacts/13736025380)

```json
{
  "environment": {
    "kernel": "7.0.0-1017-azure",
    "machine": "x86_64",
    "sources": {
      "main": {
        "sha": "a44d520c337611f59d4c7186971919d57de36df8",
        "archive_sha256": "1ee2cfc21b363c16b1b81aec670b5a20b5590509d97d1de12e8c55d76f0dda6f",
        "effective_sources_sha256": "86119872b69a98b18b7982cb6bacf67fbfe773c0633ffe5e473b9eeb7d34794c",
        "runtime_sources_sha256": "5d448b4aabf1041e3a13920294314097f88a30eeb9a2d4c799724c2c04112fc7"
      },
      "erase": {
        "sha": "7da4431e14d70ba955f2fe8de615efb9f486160f",
        "archive_sha256": "9a959094922077c91c65ee9a8faf0859cde43b59df84413193593abd5c182a22",
        "effective_sources_sha256": "fcfa9aff4a859365697ba2b13b0b1cc80073c3bd2a3d15fd86fdfc1221f0e5d5",
        "runtime_sources_sha256": "2d81fbdfc0c767b11c7a6262969ae32198036286cd4e951bde954b7d42c8934e"
      }
    },
    "binaries": {
      "main": "5f680935ce430410cf81f981558d30ad01497c304feb0fcf3268a790e90f51ce",
      "erase": "e5d01a2dc90a0b7aec78fb52d1cf94ea4a0b2b4c0ac0977981db5c6b3eba09cf"
    },
    "host_cpus": [
      0,
      1,
      2,
      3
    ],
    "compiler": "rustc 1.99.1 (58cf8f9c3 2026-10-13)\nbinary: rustc\ncommit-hash: 58cf8f9c35b355199727627d07ae34a4e2846d67\ncommit-date: 2026-10-13\nhost: x86_64-unknown-linux-gnu\nrelease: 1.99.1\nLLVM version: 23.1.1\ncargo 1.99.1 (5f94df478 2026-08-27)\nrelease: 1.99.1\ncommit-hash: 5f94df4789f005f9a352888e8355ffc645b7ed0e\ncommit-date: 2026-08-27\nhost: x86_64-unknown-linux-gnu\nlibgit2: 1.9.6 (sys:0.21.0 vendored)\nlibcurl: 8.21.0-DEV (sys:0.4.90+curl-8.21.0 vendored ssl:OpenSSL/3.6.3)\nssl: OpenSSL 3.6.3 9 Jun 2026\nos: Ubuntu 24.4.0 (noble) [64-bit]\ntarget=x86_64-unknown-linux-gnu\nprofile=release thin-LTO codegen-units=1 panic=abort\nRUSTFLAGS=\n",
    "scope": "native diagnostic: actual host kernel; not supported-LTS kernel acceptance",
    "cpu_layout": {
      "client": [
        2,
        3
      ],
      "peer": 0,
      "server": 0,
      "load": 1
    }
  },
  "same_runtime_sources": false,
  "observed_samples": 8,
  "expected_samples": 8,
  "errors": [],
  "blocks": [
    {
      "block": "udp-v4-http-1",
      "capacity_ranking": false,
      "rate_controlled": true,
      "control": {
        "Mbps": {
          "baseline": 200.07675243039088,
          "candidate": 200.07646793571556,
          "change_percent": -0.00014219276945892645
        },
        "CPU_ns_per_byte": {
          "baseline": 1.7468541119455412,
          "candidate": 1.7348844322774832,
          "change_percent": -0.6852134695281897
        },
        "CPU_core_percent": {
          "baseline": 4.368777227561295,
          "candidate": 4.338838183061828,
          "change_percent": -0.6852957461550235
        },
        "peak_RSS_MiB": {
          "baseline": 7.585148967771249,
          "candidate": 7.550199230672764,
          "change_percent": -0.4607653356181185
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.07726324462223,
          "candidate": 200.0758753869662,
          "change_percent": -0.0006936608555663248
        },
        "CPU_ns_per_byte": {
          "baseline": 1.745813669514282,
          "candidate": 2.5092699767577487,
          "change_percent": 43.7306867608543
        },
        "CPU_core_percent": {
          "baseline": 4.366182746750489,
          "candidate": 6.275509136973088,
          "change_percent": 43.729878041490736
        },
        "peak_RSS_MiB": {
          "baseline": 8.040947073198799,
          "candidate": 8.241979225745308,
          "change_percent": 2.500105406943498
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

## Initial native ARM64 200 Mbps

[Run](https://github.com/riKirsti/ProtunNext/actions/runs/42755530103) / [artifact](https://github.com/riKirsti/ProtunNext/actions/runs/42755530103/artifacts/13736015406)

```json
{
  "environment": {
    "kernel": "7.0.0-1017-azure",
    "machine": "aarch64",
    "sources": {
      "main": {
        "sha": "a44d520c337611f59d4c7186971919d57de36df8",
        "archive_sha256": "1ee2cfc21b363c16b1b81aec670b5a20b5590509d97d1de12e8c55d76f0dda6f",
        "effective_sources_sha256": "86119872b69a98b18b7982cb6bacf67fbfe773c0633ffe5e473b9eeb7d34794c",
        "runtime_sources_sha256": "5d448b4aabf1041e3a13920294314097f88a30eeb9a2d4c799724c2c04112fc7"
      },
      "erase": {
        "sha": "7da4431e14d70ba955f2fe8de615efb9f486160f",
        "archive_sha256": "9a959094922077c91c65ee9a8faf0859cde43b59df84413193593abd5c182a22",
        "effective_sources_sha256": "fcfa9aff4a859365697ba2b13b0b1cc80073c3bd2a3d15fd86fdfc1221f0e5d5",
        "runtime_sources_sha256": "2d81fbdfc0c767b11c7a6262969ae32198036286cd4e951bde954b7d42c8934e"
      }
    },
    "binaries": {
      "main": "22246101a4dfa6e674fdc3fbe01ebc6b7c3a1e2ed9f5203f745216c0facf7205",
      "erase": "7a0f591c7f2fb600eb18cdbe28fb0b1f8d520954048751ee013dddc295a63fe3"
    },
    "host_cpus": [
      0,
      1,
      2,
      3
    ],
    "compiler": "rustc 1.99.1 (58cf8f9c3 2026-10-13)\nbinary: rustc\ncommit-hash: 58cf8f9c35b355199727627d07ae34a4e2846d67\ncommit-date: 2026-10-13\nhost: aarch64-unknown-linux-gnu\nrelease: 1.99.1\nLLVM version: 23.1.1\ncargo 1.99.1 (5f94df478 2026-08-27)\nrelease: 1.99.1\ncommit-hash: 5f94df4789f005f9a352888e8355ffc645b7ed0e\ncommit-date: 2026-08-27\nhost: aarch64-unknown-linux-gnu\nlibgit2: 1.9.6 (sys:0.21.0 vendored)\nlibcurl: 8.21.0-DEV (sys:0.4.90+curl-8.21.0 vendored ssl:OpenSSL/3.6.3)\nssl: OpenSSL 3.6.3 9 Jun 2026\nos: Ubuntu 24.4.0 (noble) [64-bit]\ntarget=aarch64-unknown-linux-gnu\nprofile=release thin-LTO codegen-units=1 panic=abort\nRUSTFLAGS=\n",
    "scope": "native diagnostic: actual host kernel; not supported-LTS kernel acceptance",
    "cpu_layout": {
      "client": [
        2,
        3
      ],
      "peer": 0,
      "server": 0,
      "load": 1
    }
  },
  "same_runtime_sources": false,
  "observed_samples": 8,
  "expected_samples": 8,
  "errors": [],
  "blocks": [
    {
      "block": "udp-v4-http-1",
      "capacity_ranking": false,
      "rate_controlled": true,
      "control": {
        "Mbps": {
          "baseline": 200.07525133931284,
          "candidate": 200.07528448331462,
          "change_percent": 1.6565767912979368e-05
        },
        "CPU_ns_per_byte": {
          "baseline": 2.3156496223596417,
          "candidate": 2.2941568563502415,
          "change_percent": -0.9281527655077282
        },
        "CPU_core_percent": {
          "baseline": 5.7912571748642625,
          "candidate": 5.737503332193128,
          "change_percent": -0.9281895285956465
        },
        "peak_RSS_MiB": {
          "baseline": 6.8450887407077525,
          "candidate": 6.729287476139167,
          "change_percent": -1.691742341920499
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.07610269048624,
          "candidate": 200.07603170704135,
          "change_percent": -3.547822250427046e-05
        },
        "CPU_ns_per_byte": {
          "baseline": 2.3043423333790063,
          "candidate": 2.8779483059183364,
          "change_percent": 24.892393991573925
        },
        "CPU_core_percent": {
          "baseline": 5.762955988822845,
          "candidate": 7.197553752401941,
          "change_percent": 24.893436048470164
        },
        "peak_RSS_MiB": {
          "baseline": 7.097381065799135,
          "candidate": 7.591226794343853,
          "change_percent": 6.9581402487244315
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

## Pinned Linux 6.12 x64 unpaced

[Run](https://github.com/riKirsti/ProtunNext/actions/runs/43150064916) / [artifact](https://github.com/riKirsti/ProtunNext/actions/runs/43150064916/artifacts/13914365297)

```json
{
  "environment": {
    "kernel": "6.12.115",
    "machine": "x86_64",
    "sources": {
      "main": {
        "sha": "a44d520c337611f59d4c7186971919d57de36df8",
        "archive_sha256": "1ee2cfc21b363c16b1b81aec670b5a20b5590509d97d1de12e8c55d76f0dda6f",
        "effective_sources_sha256": "86119872b69a98b18b7982cb6bacf67fbfe773c0633ffe5e473b9eeb7d34794c",
        "runtime_sources_sha256": "5d448b4aabf1041e3a13920294314097f88a30eeb9a2d4c799724c2c04112fc7"
      },
      "erase": {
        "sha": "543675241b948a6d3b8c656bb39feb7450b9470a",
        "archive_sha256": "d5f07e76f2185a6e2fd5fc7795bfe3cd7aec7c69250118ea338dd71b383b73c4",
        "effective_sources_sha256": "e5754db3b8d20259cfa1556f99d3ae13c2ef5413365db9f04c0ae0fb078ff86c",
        "runtime_sources_sha256": "393c906231e838ba6f0b76d8680ec6dfeb03cf0c241b2c09a29d4ef2cb154720"
      }
    },
    "binaries": {
      "main": "26ea5baea840934f0f56e18a98ca4991e106314897bd93ed79ffe87a6c0fa9e6",
      "erase": "68f9fe2ddf4841de91ae9b9658c04b611a5ec041624d6d813bc0e144f575bb6c"
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
      "prepare.py": "b48b568f1445237b5c64d2f72ec06f201ca9d4c6a67f9c5600d815e94f019c0d"
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
    "scope": "actual-kernel KVM; pinned logical CPUs, not physical-core or interrupt isolation",
    "cpu_layout": {
      "client": [
        2,
        3
      ],
      "peer": 0,
      "server": 0,
      "load": 1
    }
  },
  "same_runtime_sources": false,
  "observed_samples": 8,
  "expected_samples": 8,
  "errors": [],
  "blocks": [
    {
      "block": "udp-v4-http-1",
      "capacity_ranking": false,
      "rate_controlled": false,
      "control": {
        "Mbps": {
          "baseline": 4024.1151303256825,
          "candidate": 4011.2299716029074,
          "change_percent": -0.32019856056484386
        },
        "CPU_ns_per_byte": {
          "baseline": 1.7657965052547278,
          "candidate": 1.7682788971441472,
          "change_percent": 0.14058199130151383
        },
        "CPU_core_percent": {
          "baseline": 88.82025306062411,
          "candidate": 88.66042543262922,
          "change_percent": -0.17994502659860467
        },
        "peak_RSS_MiB": {
          "baseline": 4.03535099058155,
          "candidate": 4.09375,
          "change_percent": 1.4471853763093234
        }
      },
      "result": {
        "Mbps": {
          "baseline": 4020.7022655322126,
          "candidate": 2418.3189228280667,
          "change_percent": -39.85332006402721
        },
        "CPU_ns_per_byte": {
          "baseline": 1.760753342471414,
          "candidate": 3.6530817272242833,
          "change_percent": 107.47265611302348
        },
        "CPU_core_percent": {
          "baseline": 88.49148782992523,
          "candidate": 110.42778987250134,
          "change_percent": 24.789166258269056
        },
        "peak_RSS_MiB": {
          "baseline": 3.8744959349572174,
          "candidate": 4.167952275623486,
          "change_percent": 7.5740520984572735
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

## Pinned Linux 6.18 x64 unpaced

[Run](https://github.com/riKirsti/ProtunNext/actions/runs/43150503013) / [artifact](https://github.com/riKirsti/ProtunNext/actions/runs/43150503013/artifacts/13913546959)

```json
{
  "environment": {
    "kernel": "6.18.60",
    "machine": "x86_64",
    "sources": {
      "main": {
        "sha": "a44d520c337611f59d4c7186971919d57de36df8",
        "archive_sha256": "1ee2cfc21b363c16b1b81aec670b5a20b5590509d97d1de12e8c55d76f0dda6f",
        "effective_sources_sha256": "86119872b69a98b18b7982cb6bacf67fbfe773c0633ffe5e473b9eeb7d34794c",
        "runtime_sources_sha256": "5d448b4aabf1041e3a13920294314097f88a30eeb9a2d4c799724c2c04112fc7"
      },
      "erase": {
        "sha": "543675241b948a6d3b8c656bb39feb7450b9470a",
        "archive_sha256": "d5f07e76f2185a6e2fd5fc7795bfe3cd7aec7c69250118ea338dd71b383b73c4",
        "effective_sources_sha256": "e5754db3b8d20259cfa1556f99d3ae13c2ef5413365db9f04c0ae0fb078ff86c",
        "runtime_sources_sha256": "393c906231e838ba6f0b76d8680ec6dfeb03cf0c241b2c09a29d4ef2cb154720"
      }
    },
    "binaries": {
      "main": "26ea5baea840934f0f56e18a98ca4991e106314897bd93ed79ffe87a6c0fa9e6",
      "erase": "68f9fe2ddf4841de91ae9b9658c04b611a5ec041624d6d813bc0e144f575bb6c"
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
      "prepare.py": "b48b568f1445237b5c64d2f72ec06f201ca9d4c6a67f9c5600d815e94f019c0d"
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
    "scope": "actual-kernel KVM; pinned logical CPUs, not physical-core or interrupt isolation",
    "cpu_layout": {
      "client": [
        2,
        3
      ],
      "peer": 0,
      "server": 0,
      "load": 1
    }
  },
  "same_runtime_sources": false,
  "observed_samples": 8,
  "expected_samples": 8,
  "errors": [],
  "blocks": [
    {
      "block": "udp-v4-http-1",
      "capacity_ranking": false,
      "rate_controlled": false,
      "control": {
        "Mbps": {
          "baseline": 3775.019696849142,
          "candidate": 3706.9204243241593,
          "change_percent": -1.8039448266143454
        },
        "CPU_ns_per_byte": {
          "baseline": 1.4867615749933019,
          "candidate": 1.5082883447602706,
          "change_percent": 1.4478965645225106
        },
        "CPU_core_percent": {
          "baseline": 70.15614008582949,
          "candidate": 69.88799616977724,
          "change_percent": -0.38221018962019127
        },
        "peak_RSS_MiB": {
          "baseline": 3.7871048422162152,
          "candidate": 4.14063237027646,
          "change_percent": 9.335034090404548
        }
      },
      "result": {
        "Mbps": {
          "baseline": 3737.0363340780004,
          "candidate": 3960.680984066908,
          "change_percent": 5.984545773598571
        },
        "CPU_ns_per_byte": {
          "baseline": 1.495525664610488,
          "candidate": 2.1255794212230605,
          "change_percent": 42.129250705749065
        },
        "CPU_core_percent": {
          "baseline": 69.85930155620395,
          "candidate": 105.23310323145189,
          "change_percent": 50.63577918366195
        },
        "peak_RSS_MiB": {
          "baseline": 4.048729666705657,
          "candidate": 4.212509852547825,
          "change_percent": 4.045224041234441
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

## Pinned native x64 focused 200 Mbps

[Run](https://github.com/riKirsti/ProtunNext/actions/runs/43150062345) / [artifact](https://github.com/riKirsti/ProtunNext/actions/runs/43150062345/artifacts/13913601617)

```json
{
  "environment": {
    "kernel": "7.0.0-1017-azure",
    "machine": "x86_64",
    "sources": {
      "main": {
        "sha": "a44d520c337611f59d4c7186971919d57de36df8",
        "archive_sha256": "1ee2cfc21b363c16b1b81aec670b5a20b5590509d97d1de12e8c55d76f0dda6f",
        "effective_sources_sha256": "86119872b69a98b18b7982cb6bacf67fbfe773c0633ffe5e473b9eeb7d34794c",
        "runtime_sources_sha256": "5d448b4aabf1041e3a13920294314097f88a30eeb9a2d4c799724c2c04112fc7"
      },
      "erase": {
        "sha": "543675241b948a6d3b8c656bb39feb7450b9470a",
        "archive_sha256": "d5f07e76f2185a6e2fd5fc7795bfe3cd7aec7c69250118ea338dd71b383b73c4",
        "effective_sources_sha256": "e5754db3b8d20259cfa1556f99d3ae13c2ef5413365db9f04c0ae0fb078ff86c",
        "runtime_sources_sha256": "393c906231e838ba6f0b76d8680ec6dfeb03cf0c241b2c09a29d4ef2cb154720"
      }
    },
    "binaries": {
      "main": "5f680935ce430410cf81f981558d30ad01497c304feb0fcf3268a790e90f51ce",
      "erase": "fa32d71b97d0c89d1bfb900fe5b6bf444969d185b8276424e1a684fb0a7fa654"
    },
    "host_cpus": [
      0,
      1,
      2,
      3
    ],
    "compiler": "rustc 1.99.1 (58cf8f9c3 2026-10-13)\nbinary: rustc\ncommit-hash: 58cf8f9c35b355199727627d07ae34a4e2846d67\ncommit-date: 2026-10-13\nhost: x86_64-unknown-linux-gnu\nrelease: 1.99.1\nLLVM version: 23.1.1\ncargo 1.99.1 (5f94df478 2026-08-27)\nrelease: 1.99.1\ncommit-hash: 5f94df4789f005f9a352888e8355ffc645b7ed0e\ncommit-date: 2026-08-27\nhost: x86_64-unknown-linux-gnu\nlibgit2: 1.9.6 (sys:0.21.0 vendored)\nlibcurl: 8.21.0-DEV (sys:0.4.90+curl-8.21.0 vendored ssl:OpenSSL/3.6.3)\nssl: OpenSSL 3.6.3 9 Jun 2026\nos: Ubuntu 24.4.0 (noble) [64-bit]\ntarget=x86_64-unknown-linux-gnu\nprofile=release thin-LTO codegen-units=1 panic=abort\nRUSTFLAGS=\n",
    "scope": "native diagnostic: actual host kernel; not supported-LTS kernel acceptance",
    "cpu_layout": {
      "client": [
        2,
        3
      ],
      "peer": 0,
      "server": 0,
      "load": 1
    }
  },
  "same_runtime_sources": false,
  "observed_samples": 8,
  "expected_samples": 8,
  "errors": [],
  "blocks": [
    {
      "block": "udp-v4-http-1",
      "capacity_ranking": false,
      "rate_controlled": true,
      "control": {
        "Mbps": {
          "baseline": 200.069742582428,
          "candidate": 200.07185038802407,
          "change_percent": 0.0010535354166396615
        },
        "CPU_ns_per_byte": {
          "baseline": 2.5633424707443533,
          "candidate": 2.594369050434895,
          "change_percent": 1.210395413201737
        },
        "CPU_core_percent": {
          "baseline": 6.410528443472143,
          "candidate": 6.488181074830204,
          "change_percent": 1.2113296437696208
        },
        "peak_RSS_MiB": {
          "baseline": 7.756939150236306,
          "candidate": 7.490099666731955,
          "change_percent": -3.4400100134371003
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.07150083712537,
          "candidate": 200.06943601901358,
          "change_percent": -0.0010320400972374522
        },
        "CPU_ns_per_byte": {
          "baseline": 2.6169812613157757,
          "candidate": 4.029891861537398,
          "change_percent": 53.99009236738799
        },
        "CPU_core_percent": {
          "baseline": 6.544735488085046,
          "candidate": 10.078139156395308,
          "change_percent": 53.988487002155615
        },
        "peak_RSS_MiB": {
          "baseline": 8.02324541822171,
          "candidate": 7.939158829493327,
          "change_percent": -1.0480371015127221
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

## Pinned native ARM64 focused 200 Mbps

[Run](https://github.com/riKirsti/ProtunNext/actions/runs/43150062345) / [artifact](https://github.com/riKirsti/ProtunNext/actions/runs/43150062345/artifacts/13913237997)

```json
{
  "environment": {
    "kernel": "7.0.0-1017-azure",
    "machine": "aarch64",
    "sources": {
      "main": {
        "sha": "a44d520c337611f59d4c7186971919d57de36df8",
        "archive_sha256": "1ee2cfc21b363c16b1b81aec670b5a20b5590509d97d1de12e8c55d76f0dda6f",
        "effective_sources_sha256": "86119872b69a98b18b7982cb6bacf67fbfe773c0633ffe5e473b9eeb7d34794c",
        "runtime_sources_sha256": "5d448b4aabf1041e3a13920294314097f88a30eeb9a2d4c799724c2c04112fc7"
      },
      "erase": {
        "sha": "543675241b948a6d3b8c656bb39feb7450b9470a",
        "archive_sha256": "d5f07e76f2185a6e2fd5fc7795bfe3cd7aec7c69250118ea338dd71b383b73c4",
        "effective_sources_sha256": "e5754db3b8d20259cfa1556f99d3ae13c2ef5413365db9f04c0ae0fb078ff86c",
        "runtime_sources_sha256": "393c906231e838ba6f0b76d8680ec6dfeb03cf0c241b2c09a29d4ef2cb154720"
      }
    },
    "binaries": {
      "main": "22246101a4dfa6e674fdc3fbe01ebc6b7c3a1e2ed9f5203f745216c0facf7205",
      "erase": "433e598986e549b6efa1d37f5d57a9ee3f5febb781a4f9fa007903688759b318"
    },
    "host_cpus": [
      0,
      1,
      2,
      3
    ],
    "compiler": "rustc 1.99.1 (58cf8f9c3 2026-10-13)\nbinary: rustc\ncommit-hash: 58cf8f9c35b355199727627d07ae34a4e2846d67\ncommit-date: 2026-10-13\nhost: aarch64-unknown-linux-gnu\nrelease: 1.99.1\nLLVM version: 23.1.1\ncargo 1.99.1 (5f94df478 2026-08-27)\nrelease: 1.99.1\ncommit-hash: 5f94df4789f005f9a352888e8355ffc645b7ed0e\ncommit-date: 2026-08-27\nhost: aarch64-unknown-linux-gnu\nlibgit2: 1.9.6 (sys:0.21.0 vendored)\nlibcurl: 8.21.0-DEV (sys:0.4.90+curl-8.21.0 vendored ssl:OpenSSL/3.6.3)\nssl: OpenSSL 3.6.3 9 Jun 2026\nos: Ubuntu 24.4.0 (noble) [64-bit]\ntarget=aarch64-unknown-linux-gnu\nprofile=release thin-LTO codegen-units=1 panic=abort\nRUSTFLAGS=\n",
    "scope": "native diagnostic: actual host kernel; not supported-LTS kernel acceptance",
    "cpu_layout": {
      "client": [
        2,
        3
      ],
      "peer": 0,
      "server": 0,
      "load": 1
    }
  },
  "same_runtime_sources": false,
  "observed_samples": 8,
  "expected_samples": 8,
  "errors": [],
  "blocks": [
    {
      "block": "udp-v4-http-1",
      "capacity_ranking": false,
      "rate_controlled": true,
      "control": {
        "Mbps": {
          "baseline": 200.07371944198857,
          "candidate": 200.0755302011711,
          "change_percent": 0.000905045993837561
        },
        "CPU_ns_per_byte": {
          "baseline": 2.34327323481721,
          "candidate": 2.28147931100525,
          "change_percent": -2.6370771830533157
        },
        "CPU_core_percent": {
          "baseline": 5.860280565111078,
          "candidate": 5.705805209490111,
          "change_percent": -2.6359720137057785
        },
        "peak_RSS_MiB": {
          "baseline": 6.999646859116805,
          "candidate": 7.28515625,
          "change_percent": 4.078911359811377
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.07558757671785,
          "candidate": 200.07495456484403,
          "change_percent": -0.00031638636251996033
        },
        "CPU_ns_per_byte": {
          "baseline": 2.3236443047500153,
          "candidate": 3.033533694045376,
          "change_percent": 30.550690905841236
        },
        "CPU_core_percent": {
          "baseline": 5.811163315567705,
          "candidate": 7.586622021812403,
          "change_percent": 30.552552214259187
        },
        "peak_RSS_MiB": {
          "baseline": 7.53907970367208,
          "candidate": 7.932943643354138,
          "change_percent": 5.224297330219474
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

## Pinned native x64 full 200 Mbps

[Run](https://github.com/riKirsti/ProtunNext/actions/runs/43150498346) / [artifact](https://github.com/riKirsti/ProtunNext/actions/runs/43150498346/artifacts/13914580863)

```json
{
  "environment": {
    "kernel": "7.0.0-1017-azure",
    "machine": "x86_64",
    "sources": {
      "main": {
        "sha": "a44d520c337611f59d4c7186971919d57de36df8",
        "archive_sha256": "1ee2cfc21b363c16b1b81aec670b5a20b5590509d97d1de12e8c55d76f0dda6f",
        "effective_sources_sha256": "86119872b69a98b18b7982cb6bacf67fbfe773c0633ffe5e473b9eeb7d34794c",
        "runtime_sources_sha256": "5d448b4aabf1041e3a13920294314097f88a30eeb9a2d4c799724c2c04112fc7"
      },
      "erase": {
        "sha": "543675241b948a6d3b8c656bb39feb7450b9470a",
        "archive_sha256": "d5f07e76f2185a6e2fd5fc7795bfe3cd7aec7c69250118ea338dd71b383b73c4",
        "effective_sources_sha256": "e5754db3b8d20259cfa1556f99d3ae13c2ef5413365db9f04c0ae0fb078ff86c",
        "runtime_sources_sha256": "393c906231e838ba6f0b76d8680ec6dfeb03cf0c241b2c09a29d4ef2cb154720"
      }
    },
    "binaries": {
      "main": "5f680935ce430410cf81f981558d30ad01497c304feb0fcf3268a790e90f51ce",
      "erase": "fa32d71b97d0c89d1bfb900fe5b6bf444969d185b8276424e1a684fb0a7fa654"
    },
    "host_cpus": [
      0,
      1,
      2,
      3
    ],
    "compiler": "rustc 1.99.1 (58cf8f9c3 2026-10-13)\nbinary: rustc\ncommit-hash: 58cf8f9c35b355199727627d07ae34a4e2846d67\ncommit-date: 2026-10-13\nhost: x86_64-unknown-linux-gnu\nrelease: 1.99.1\nLLVM version: 23.1.1\ncargo 1.99.1 (5f94df478 2026-08-27)\nrelease: 1.99.1\ncommit-hash: 5f94df4789f005f9a352888e8355ffc645b7ed0e\ncommit-date: 2026-08-27\nhost: x86_64-unknown-linux-gnu\nlibgit2: 1.9.6 (sys:0.21.0 vendored)\nlibcurl: 8.21.0-DEV (sys:0.4.90+curl-8.21.0 vendored ssl:OpenSSL/3.6.3)\nssl: OpenSSL 3.6.3 9 Jun 2026\nos: Ubuntu 24.4.0 (noble) [64-bit]\ntarget=x86_64-unknown-linux-gnu\nprofile=release thin-LTO codegen-units=1 panic=abort\nRUSTFLAGS=\n",
    "scope": "native diagnostic: actual host kernel; not supported-LTS kernel acceptance",
    "cpu_layout": {
      "client": [
        2,
        3
      ],
      "peer": 0,
      "server": 0,
      "load": 1
    }
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
          "baseline": 200.0790734734766,
          "candidate": 200.07945809733454,
          "change_percent": 0.00019223592515604793
        },
        "CPU_ns_per_byte": {
          "baseline": 1.5045844974994993,
          "candidate": 1.4820427091441521,
          "change_percent": -1.4982068732470544
        },
        "CPU_core_percent": {
          "baseline": 3.762918171255103,
          "candidate": 3.7065494236249963,
          "change_percent": -1.498006203289437
        },
        "peak_RSS_MiB": {
          "baseline": 7.602700566785669,
          "candidate": 7.622342005988055,
          "change_percent": 0.25834818864489684
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.078999934194,
          "candidate": 200.07864142810945,
          "change_percent": -0.00017918226533630843
        },
        "CPU_ns_per_byte": {
          "baseline": 1.5704845632219095,
          "candidate": 2.080814010871106,
          "change_percent": 32.495031126077166
        },
        "CPU_core_percent": {
          "baseline": 3.9277334166303888,
          "candidate": 5.204045629842976,
          "change_percent": 32.49487879723614
        },
        "peak_RSS_MiB": {
          "baseline": 7.4712706585898925,
          "candidate": 8.431629540544602,
          "change_percent": 12.85402344312827
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
          "baseline": 200.70295641556118,
          "candidate": 200.70645805640837,
          "change_percent": 0.0017446882246918705
        },
        "CPU_ns_per_byte": {
          "baseline": 1.8250669672574007,
          "candidate": 1.7693500959116757,
          "change_percent": -3.0528672287271186
        },
        "CPU_core_percent": {
          "baseline": 4.57867317126586,
          "candidate": 4.438968444533437,
          "change_percent": -3.051205480425234
        },
        "peak_RSS_MiB": {
          "baseline": 8.172651731728509,
          "candidate": 8.352967389436184,
          "change_percent": 2.206329886879188
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.69852661273364,
          "candidate": 200.70481263049365,
          "change_percent": 0.0031320697097658723
        },
        "CPU_ns_per_byte": {
          "baseline": 1.6675913050128477,
          "candidate": 2.3083584359215608,
          "change_percent": 38.42471047807343
        },
        "CPU_core_percent": {
          "baseline": 4.18350251835234,
          "candidate": 5.791188143133836,
          "change_percent": 38.42917788931266
        },
        "peak_RSS_MiB": {
          "baseline": 8.288329815639127,
          "candidate": 8.842631886032294,
          "change_percent": 6.6877414717168016
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
          "baseline": 200.07762536195156,
          "candidate": 200.0785922894688,
          "change_percent": 0.00048327618615839896
        },
        "CPU_ns_per_byte": {
          "baseline": 1.5392274353844346,
          "candidate": 1.5506504427366634,
          "change_percent": 0.7421260230705107
        },
        "CPU_core_percent": {
          "baseline": 3.8495355607459887,
          "candidate": 3.8781210836096025,
          "change_percent": 0.7425706922960407
        },
        "peak_RSS_MiB": {
          "baseline": 7.812023422963955,
          "candidate": 7.895643515473422,
          "change_percent": 1.070402480664101
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.0779133458058,
          "candidate": 200.0767339388212,
          "change_percent": -0.0005894738528966492
        },
        "CPU_ns_per_byte": {
          "baseline": 1.4551312432964338,
          "candidate": 2.1192788711248256,
          "change_percent": 45.641768114595706
        },
        "CPU_core_percent": {
          "baseline": 3.6392170260400554,
          "candidate": 5.300192512242685,
          "change_percent": 45.641012182502024
        },
        "peak_RSS_MiB": {
          "baseline": 7.484124229227869,
          "candidate": 8.28758970186183,
          "change_percent": 10.735597753657999
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
          "baseline": 200.69717208918303,
          "candidate": 200.69460019066412,
          "change_percent": -0.00128148219137314
        },
        "CPU_ns_per_byte": {
          "baseline": 1.7638703018013973,
          "candidate": 1.7532262617855774,
          "change_percent": -0.6034479975624785
        },
        "CPU_core_percent": {
          "baseline": 4.425014333822721,
          "candidate": 4.398256700402359,
          "change_percent": -0.6046903219236732
        },
        "peak_RSS_MiB": {
          "baseline": 8.300515620749863,
          "candidate": 8.281226967833536,
          "change_percent": -0.2323789725557357
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.68997734109408,
          "candidate": 200.69153677154057,
          "change_percent": 0.0007770345421098668
        },
        "CPU_ns_per_byte": {
          "baseline": 1.7895509301993937,
          "candidate": 2.389579341317094,
          "change_percent": 33.52955207879133
        },
        "CPU_core_percent": {
          "baseline": 4.48927995936123,
          "candidate": 5.994566018841272,
          "change_percent": 33.53067915359473
        },
        "peak_RSS_MiB": {
          "baseline": 8.006836830806296,
          "candidate": 9.130887575156986,
          "change_percent": 14.03863683128781
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
          "baseline": 200.0768289398142,
          "candidate": 200.07974802519152,
          "change_percent": 0.0014589822283861409
        },
        "CPU_ns_per_byte": {
          "baseline": 1.516751693126914,
          "candidate": 1.5937568224870275,
          "change_percent": 5.076976654060017
        },
        "CPU_core_percent": {
          "baseline": 3.7933058793956036,
          "candidate": 3.985954382190895,
          "change_percent": 5.078644035581603
        },
        "peak_RSS_MiB": {
          "baseline": 6.825851935009422,
          "candidate": 7.007204979937391,
          "change_percent": 2.656855827736604
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.0795660863689,
          "candidate": 200.07863848204317,
          "change_percent": -0.00046361772163550086
        },
        "CPU_ns_per_byte": {
          "baseline": 1.5413353247056367,
          "candidate": 2.0131446593074203,
          "change_percent": 30.61042766225379
        },
        "CPU_core_percent": {
          "baseline": 3.8548423209629816,
          "candidate": 5.034805306109435,
          "change_percent": 30.609889767208067
        },
        "peak_RSS_MiB": {
          "baseline": 7.131717125640658,
          "candidate": 7.089136715778586,
          "change_percent": -0.5970569094640998
        }
      },
      "AA_pass": false,
      "timing_pass": true,
      "auxiliary_limited": false,
      "validity": "inconclusive-control/timing"
    },
    {
      "block": "tcp-v4-http-8",
      "capacity_ranking": false,
      "rate_controlled": true,
      "control": {
        "Mbps": {
          "baseline": 200.70995353924766,
          "candidate": 200.70430281308424,
          "change_percent": -0.002815369175157567
        },
        "CPU_ns_per_byte": {
          "baseline": 1.6697360970590776,
          "candidate": 1.6785186715166334,
          "change_percent": 0.525985781407301
        },
        "CPU_core_percent": {
          "baseline": 4.189127132258339,
          "candidate": 4.211044607857175,
          "change_percent": 0.5231991034614358
        },
        "peak_RSS_MiB": {
          "baseline": 7.810212555751325,
          "candidate": 7.814062343781241,
          "change_percent": 0.04929171904650698
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.70288753831957,
          "candidate": 200.7072581203765,
          "change_percent": 0.002177637855904635
        },
        "CPU_ns_per_byte": {
          "baseline": 1.7521952257237772,
          "candidate": 2.2175452066475163,
          "change_percent": 26.558112594532247
        },
        "CPU_core_percent": {
          "baseline": 4.395845393355571,
          "candidate": 5.563424957997214,
          "change_percent": 26.56097883711901
        },
        "peak_RSS_MiB": {
          "baseline": 7.640311455095229,
          "candidate": 8.085631787699091,
          "change_percent": 5.828562555612615
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
          "baseline": 200.07701618187613,
          "candidate": 200.07813334688154,
          "change_percent": 0.0005583674860476506
        },
        "CPU_ns_per_byte": {
          "baseline": 1.4549785162612787,
          "candidate": 1.4752046126085192,
          "change_percent": 1.390130240494103
        },
        "CPU_core_percent": {
          "baseline": 3.6388188785371964,
          "candidate": 3.68942041224708,
          "change_percent": 1.390603253389333
        },
        "peak_RSS_MiB": {
          "baseline": 7.043145319819389,
          "candidate": 6.976791053437125,
          "change_percent": -0.9421112779760876
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.07574883766398,
          "candidate": 200.07458570993242,
          "change_percent": -0.0005813436852353249
        },
        "CPU_ns_per_byte": {
          "baseline": 1.5470975005539656,
          "candidate": 2.302207359267003,
          "change_percent": 48.808162280829556
        },
        "CPU_core_percent": {
          "baseline": 3.869180444642157,
          "candidate": 5.75762354188298,
          "change_percent": 48.80731525085223
        },
        "peak_RSS_MiB": {
          "baseline": 7.123296165863215,
          "candidate": 7.479392669011677,
          "change_percent": 4.999041101996782
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
          "baseline": 200.6886243246145,
          "candidate": 200.6949007358623,
          "change_percent": 0.003127437476302397
        },
        "CPU_ns_per_byte": {
          "baseline": 1.8545230873067586,
          "candidate": 1.8627699872752141,
          "change_percent": 0.444691145928644
        },
        "CPU_core_percent": {
          "baseline": 4.65222873579157,
          "candidate": 4.67307202840196,
          "change_percent": 0.44802811284909705
        },
        "peak_RSS_MiB": {
          "baseline": 7.604890916030722,
          "candidate": 8.103962585965371,
          "change_percent": 6.562509251547999
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.6892623758252,
          "candidate": 200.69226591604306,
          "change_percent": 0.00149661231612086
        },
        "CPU_ns_per_byte": {
          "baseline": 1.8639316985951284,
          "candidate": 2.3827889533647513,
          "change_percent": 27.836709636983635
        },
        "CPU_core_percent": {
          "baseline": 4.675848480469332,
          "candidate": 5.977548258360677,
          "change_percent": 27.838792966205972
        },
        "peak_RSS_MiB": {
          "baseline": 7.885728098381378,
          "candidate": 8.171070183241184,
          "change_percent": 3.618462129303901
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
          "baseline": 200.07828780624754,
          "candidate": 200.07868981961576,
          "change_percent": 0.00020092803303395357
        },
        "CPU_ns_per_byte": {
          "baseline": 1.649330662873515,
          "candidate": 1.7328310625437684,
          "change_percent": 5.062684005690921
        },
        "CPU_core_percent": {
          "baseline": 4.124910569981421,
          "candidate": 4.333745107306476,
          "change_percent": 5.062765211076936
        },
        "peak_RSS_MiB": {
          "baseline": 5.18359375,
          "candidate": 5.035004725221994,
          "change_percent": -2.8665252707738897
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.0787579736041,
          "candidate": 200.07774181537943,
          "change_percent": -0.0005078791146861938
        },
        "CPU_ns_per_byte": {
          "baseline": 1.6811471808109608,
          "candidate": 2.2376753220918717,
          "change_percent": 33.104070103633035
        },
        "CPU_core_percent": {
          "baseline": 4.204485963764151,
          "candidate": 5.596322838082651,
          "change_percent": 33.103615669403496
        },
        "peak_RSS_MiB": {
          "baseline": 5.09956411855158,
          "candidate": 5.374885025569006,
          "change_percent": 5.398910585629135
        }
      },
      "AA_pass": false,
      "timing_pass": true,
      "auxiliary_limited": false,
      "validity": "inconclusive-control/timing"
    },
    {
      "block": "stealth-v4-http-8",
      "capacity_ranking": false,
      "rate_controlled": true,
      "control": {
        "Mbps": {
          "baseline": 200.69694591464628,
          "candidate": 200.6969390479158,
          "change_percent": -3.421442429374366e-06
        },
        "CPU_ns_per_byte": {
          "baseline": 1.9123101196961951,
          "candidate": 1.8824688883187288,
          "change_percent": -1.5604807541470866
        },
        "CPU_core_percent": {
          "baseline": 4.797398133791128,
          "candidate": 4.722529004432687,
          "change_percent": -1.5606194706061616
        },
        "peak_RSS_MiB": {
          "baseline": 6.538987828128009,
          "candidate": 6.537195447315787,
          "change_percent": -0.027410676687800173
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.70396590112665,
          "candidate": 200.70221321166085,
          "change_percent": -0.00087327096798262
        },
        "CPU_ns_per_byte": {
          "baseline": 1.809952546906251,
          "candidate": 2.3280710936049522,
          "change_percent": 28.62608456693081
        },
        "CPU_core_percent": {
          "baseline": 4.54077586328354,
          "candidate": 5.840563877447964,
          "change_percent": 28.62480010683719
        },
        "peak_RSS_MiB": {
          "baseline": 6.444100264789202,
          "candidate": 6.722600640749086,
          "change_percent": 4.321788372561808
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
          "baseline": 200.0765950290281,
          "candidate": 200.07713382098953,
          "change_percent": 0.0002692928482428414
        },
        "CPU_ns_per_byte": {
          "baseline": 1.7169456658863778,
          "candidate": 1.7161497202044438,
          "change_percent": -0.04635823356257118
        },
        "CPU_core_percent": {
          "baseline": 4.293976185863237,
          "candidate": 4.291997749682044,
          "change_percent": -0.04607468918216284
        },
        "peak_RSS_MiB": {
          "baseline": 5.16787426689897,
          "candidate": 5.134361091543852,
          "change_percent": -0.6484905325536916
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.0758965104938,
          "candidate": 200.07544072691542,
          "change_percent": -0.0002278053410464942
        },
        "CPU_ns_per_byte": {
          "baseline": 1.6693957170333857,
          "candidate": 2.258718851862558,
          "change_percent": 35.301584208951596
        },
        "CPU_core_percent": {
          "baseline": 4.175040604561215,
          "candidate": 5.648885169226706,
          "change_percent": 35.301322891454554
        },
        "peak_RSS_MiB": {
          "baseline": 5.0460752438118615,
          "candidate": 5.337890267677449,
          "change_percent": 5.783009760376601
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
          "baseline": 200.6860620238242,
          "candidate": 200.68916425246337,
          "change_percent": 0.0015458117060385135
        },
        "CPU_ns_per_byte": {
          "baseline": 1.8796110937992698,
          "candidate": 1.9259903683277741,
          "change_percent": 2.4674931256527977
        },
        "CPU_core_percent": {
          "baseline": 4.715105974692222,
          "candidate": 4.831526773379192,
          "change_percent": 2.4691024827828922
        },
        "peak_RSS_MiB": {
          "baseline": 5.133195975971926,
          "candidate": 5.142307736553878,
          "change_percent": 0.1775065792267272
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.6931633986369,
          "candidate": 200.68653424797407,
          "change_percent": -0.003303127296705455
        },
        "CPU_ns_per_byte": {
          "baseline": 1.853119010947659,
          "candidate": 2.3773074077458745,
          "change_percent": 28.286817721985003
        },
        "CPU_core_percent": {
          "baseline": 4.648818242187188,
          "candidate": 5.963627565333317,
          "change_percent": 28.282657110886177
        },
        "peak_RSS_MiB": {
          "baseline": 5.281105536115875,
          "candidate": 5.558041961368764,
          "change_percent": 5.2439100744911205
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

## Pinned native ARM64 full 200 Mbps

[Run](https://github.com/riKirsti/ProtunNext/actions/runs/43150498346) / [artifact](https://github.com/riKirsti/ProtunNext/actions/runs/43150498346/artifacts/13914247403)

```json
{
  "environment": {
    "kernel": "7.0.0-1017-azure",
    "machine": "aarch64",
    "sources": {
      "main": {
        "sha": "a44d520c337611f59d4c7186971919d57de36df8",
        "archive_sha256": "1ee2cfc21b363c16b1b81aec670b5a20b5590509d97d1de12e8c55d76f0dda6f",
        "effective_sources_sha256": "86119872b69a98b18b7982cb6bacf67fbfe773c0633ffe5e473b9eeb7d34794c",
        "runtime_sources_sha256": "5d448b4aabf1041e3a13920294314097f88a30eeb9a2d4c799724c2c04112fc7"
      },
      "erase": {
        "sha": "543675241b948a6d3b8c656bb39feb7450b9470a",
        "archive_sha256": "d5f07e76f2185a6e2fd5fc7795bfe3cd7aec7c69250118ea338dd71b383b73c4",
        "effective_sources_sha256": "e5754db3b8d20259cfa1556f99d3ae13c2ef5413365db9f04c0ae0fb078ff86c",
        "runtime_sources_sha256": "393c906231e838ba6f0b76d8680ec6dfeb03cf0c241b2c09a29d4ef2cb154720"
      }
    },
    "binaries": {
      "main": "22246101a4dfa6e674fdc3fbe01ebc6b7c3a1e2ed9f5203f745216c0facf7205",
      "erase": "433e598986e549b6efa1d37f5d57a9ee3f5febb781a4f9fa007903688759b318"
    },
    "host_cpus": [
      0,
      1,
      2,
      3
    ],
    "compiler": "rustc 1.99.1 (58cf8f9c3 2026-10-13)\nbinary: rustc\ncommit-hash: 58cf8f9c35b355199727627d07ae34a4e2846d67\ncommit-date: 2026-10-13\nhost: aarch64-unknown-linux-gnu\nrelease: 1.99.1\nLLVM version: 23.1.1\ncargo 1.99.1 (5f94df478 2026-08-27)\nrelease: 1.99.1\ncommit-hash: 5f94df4789f005f9a352888e8355ffc645b7ed0e\ncommit-date: 2026-08-27\nhost: aarch64-unknown-linux-gnu\nlibgit2: 1.9.6 (sys:0.21.0 vendored)\nlibcurl: 8.21.0-DEV (sys:0.4.90+curl-8.21.0 vendored ssl:OpenSSL/3.6.3)\nssl: OpenSSL 3.6.3 9 Jun 2026\nos: Ubuntu 24.4.0 (noble) [64-bit]\ntarget=aarch64-unknown-linux-gnu\nprofile=release thin-LTO codegen-units=1 panic=abort\nRUSTFLAGS=\n",
    "scope": "native diagnostic: actual host kernel; not supported-LTS kernel acceptance",
    "cpu_layout": {
      "client": [
        2,
        3
      ],
      "peer": 0,
      "server": 0,
      "load": 1
    }
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
          "baseline": 200.07455621992236,
          "candidate": 200.0750451575839,
          "change_percent": 0.00024437773136209984
        },
        "CPU_ns_per_byte": {
          "baseline": 2.3405816324527544,
          "candidate": 2.3431467321676447,
          "change_percent": 0.10959240555101779
        },
        "CPU_core_percent": {
          "baseline": 5.85354102537099,
          "candidate": 5.860013203564709,
          "change_percent": 0.11056859712208489
        },
        "peak_RSS_MiB": {
          "baseline": 7.267268169367897,
          "candidate": 7.400661500799374,
          "change_percent": 1.835536109617375
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.07501733023162,
          "candidate": 200.07552608162095,
          "change_percent": 0.00025428031751317803
        },
        "CPU_ns_per_byte": {
          "baseline": 2.342796223663041,
          "candidate": 2.978992020039628,
          "change_percent": 27.15540472324449
        },
        "CPU_core_percent": {
          "baseline": 5.859138477692317,
          "candidate": 7.450235809057992,
          "change_percent": 27.155823973499004
        },
        "peak_RSS_MiB": {
          "baseline": 7.091167231842791,
          "candidate": 7.560885674806886,
          "change_percent": 6.623993308955267
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
          "baseline": 200.69029045910503,
          "candidate": 200.68823219184716,
          "change_percent": -0.001025593840719985
        },
        "CPU_ns_per_byte": {
          "baseline": 2.3298019680961812,
          "candidate": 2.3475184680966144,
          "change_percent": 0.7604294374817799
        },
        "CPU_core_percent": {
          "baseline": 5.844506602532556,
          "candidate": 5.88888499737001,
          "change_percent": 0.7593180717466064
        },
        "peak_RSS_MiB": {
          "baseline": 7.80397484077375,
          "candidate": 7.656075611853248,
          "change_percent": -1.895178187245905
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.69173329353092,
          "candidate": 200.6922441200374,
          "change_percent": 0.00025453290881660706
        },
        "CPU_ns_per_byte": {
          "baseline": 2.306331145967264,
          "candidate": 2.931787269737703,
          "change_percent": 27.119094535235334
        },
        "CPU_core_percent": {
          "baseline": 5.785671378489505,
          "candidate": 7.354775262112843,
          "change_percent": 27.120515165397997
        },
        "peak_RSS_MiB": {
          "baseline": 6.837687276057325,
          "candidate": 8.091486433898343,
          "change_percent": 18.336596969435703
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
          "baseline": 200.07054482524376,
          "candidate": 200.07163849009592,
          "change_percent": 0.0005466396131081908
        },
        "CPU_ns_per_byte": {
          "baseline": 2.528549599301028,
          "candidate": 2.5502064069968378,
          "change_percent": 0.8564913143011443
        },
        "CPU_core_percent": {
          "baseline": 6.323526723473575,
          "candidate": 6.377749252332896,
          "change_percent": 0.8574729139364745
        },
        "peak_RSS_MiB": {
          "baseline": 6.90624005761445,
          "candidate": 7.212199620383355,
          "change_percent": 4.43019009209753
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.07114371628455,
          "candidate": 200.0688988197713,
          "change_percent": -0.00112204912290359
        },
        "CPU_ns_per_byte": {
          "baseline": 2.6210176840639106,
          "candidate": 3.162382370472906,
          "change_percent": 20.654751385332325
        },
        "CPU_core_percent": {
          "baseline": 6.554809887758747,
          "candidate": 7.908619944845944,
          "change_percent": 20.653689127055653
        },
        "peak_RSS_MiB": {
          "baseline": 7.372953486577436,
          "candidate": 7.367454677684443,
          "change_percent": -0.07458081626315893
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
          "baseline": 200.67047137034817,
          "candidate": 200.66755012026474,
          "change_percent": -0.0014557448654417549
        },
        "CPU_ns_per_byte": {
          "baseline": 2.5107019593386086,
          "candidate": 2.51053290897798,
          "change_percent": -0.006733191090235735
        },
        "CPU_core_percent": {
          "baseline": 6.297681435803369,
          "candidate": 6.297223016600119,
          "change_percent": -0.007279174215513162
        },
        "peak_RSS_MiB": {
          "baseline": 7.22238582849629,
          "candidate": 7.02929443291497,
          "change_percent": -2.6735126060348224
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.67613868950846,
          "candidate": 200.67272267826482,
          "change_percent": -0.001702250833579555
        },
        "CPU_ns_per_byte": {
          "baseline": 2.520904996487945,
          "candidate": 3.0923244731511788,
          "change_percent": 22.667235673669573
        },
        "CPU_core_percent": {
          "baseline": 6.32345303271232,
          "candidate": 7.7567496739000985,
          "change_percent": 22.666360195498992
        },
        "peak_RSS_MiB": {
          "baseline": 7.419027002910952,
          "candidate": 8.020711790542851,
          "change_percent": 8.110022882998269
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
          "baseline": 200.07606801894383,
          "candidate": 200.07581378010602,
          "change_percent": -0.00012707108867360262
        },
        "CPU_ns_per_byte": {
          "baseline": 2.181578266622599,
          "candidate": 2.1604062293026,
          "change_percent": -0.9704917602051721
        },
        "CPU_core_percent": {
          "baseline": 5.4559265773124865,
          "candidate": 5.40297435650562,
          "change_percent": -0.9705449671382871
        },
        "peak_RSS_MiB": {
          "baseline": 6.540861367713018,
          "candidate": 6.41015625,
          "change_percent": -1.9982860110474787
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.07657640596852,
          "candidate": 200.07614847188313,
          "change_percent": -0.00021388514991338425
        },
        "CPU_ns_per_byte": {
          "baseline": 2.1865204256951674,
          "candidate": 2.797671650214863,
          "change_percent": 27.950858237484354
        },
        "CPU_core_percent": {
          "baseline": 5.468304448500758,
          "candidate": 6.996789109676656,
          "change_percent": 27.95171109382839
        },
        "peak_RSS_MiB": {
          "baseline": 5.808064398794738,
          "candidate": 6.538967993268676,
          "change_percent": 12.584288745586436
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
          "baseline": 200.69424143239635,
          "candidate": 200.69501527994365,
          "change_percent": 0.00038558532708954374
        },
        "CPU_ns_per_byte": {
          "baseline": 2.200136134364114,
          "candidate": 2.2137550827297403,
          "change_percent": 0.6190048039714791
        },
        "CPU_core_percent": {
          "baseline": 5.519386009416864,
          "candidate": 5.553480759403278,
          "change_percent": 0.6177272241557841
        },
        "peak_RSS_MiB": {
          "baseline": 6.8859698082547025,
          "candidate": 6.716996500831864,
          "change_percent": -2.453878133771048
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.69487230488258,
          "candidate": 200.694424933902,
          "change_percent": -0.00022291101682281322
        },
        "CPU_ns_per_byte": {
          "baseline": 2.1947079925849655,
          "candidate": 2.7746944066635586,
          "change_percent": 26.42658686431787
        },
        "CPU_core_percent": {
          "baseline": 5.505738651624924,
          "candidate": 6.960767437828457,
          "change_percent": 26.42749462461511
        },
        "peak_RSS_MiB": {
          "baseline": 6.887382315237772,
          "candidate": 7.659136294283712,
          "change_percent": 11.205330903999599
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
          "baseline": 200.07111970845327,
          "candidate": 200.07015710780382,
          "change_percent": -0.0004811292358630759
        },
        "CPU_ns_per_byte": {
          "baseline": 2.4236947601557643,
          "candidate": 2.449321883465897,
          "change_percent": 1.0573577057403805
        },
        "CPU_core_percent": {
          "baseline": 6.061284032679242,
          "candidate": 6.125395779976043,
          "change_percent": 1.0577255075186143
        },
        "peak_RSS_MiB": {
          "baseline": 6.337184872589328,
          "candidate": 6.651699176053509,
          "change_percent": 4.962997131811209
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.07103221374373,
          "candidate": 200.07200740942744,
          "change_percent": 0.00048742472755769484
        },
        "CPU_ns_per_byte": {
          "baseline": 2.439868003176164,
          "candidate": 2.9883415669225144,
          "change_percent": 22.47964082615781
        },
        "CPU_core_percent": {
          "baseline": 6.101789952136895,
          "candidate": 7.473487329112977,
          "change_percent": 22.480245759618512
        },
        "peak_RSS_MiB": {
          "baseline": 5.839587682513023,
          "candidate": 6.4703738620762135,
          "change_percent": 10.801895850491562
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
          "baseline": 200.6730358008844,
          "candidate": 200.67399308225626,
          "change_percent": 0.00047703537648935423
        },
        "CPU_ns_per_byte": {
          "baseline": 2.317712524532558,
          "candidate": 2.329159309889166,
          "change_percent": 0.4938828795826078
        },
        "CPU_core_percent": {
          "baseline": 5.813728028049306,
          "candidate": 5.8424707706447645,
          "change_percent": 0.494394344846949
        },
        "peak_RSS_MiB": {
          "baseline": 6.898383307757129,
          "candidate": 7.035156249999999,
          "change_percent": 1.9826811028182734
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.6789832983548,
          "candidate": 200.67485211793226,
          "change_percent": -0.002058601431320639
        },
        "CPU_ns_per_byte": {
          "baseline": 2.330823433187592,
          "candidate": 2.894384394880881,
          "change_percent": 24.178620897189674
        },
        "CPU_core_percent": {
          "baseline": 5.846791046311851,
          "candidate": 7.260317369317592,
          "change_percent": 24.176104666805088
        },
        "peak_RSS_MiB": {
          "baseline": 6.450625177883682,
          "candidate": 7.255228196386425,
          "change_percent": 12.473256410268085
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
          "baseline": 200.07273069531678,
          "candidate": 200.0731655542545,
          "change_percent": 0.00021735042863291199
        },
        "CPU_ns_per_byte": {
          "baseline": 2.640285465789095,
          "candidate": 2.629136011428607,
          "change_percent": -0.4222821549015987
        },
        "CPU_core_percent": {
          "baseline": 6.603060239318747,
          "candidate": 6.575131730362444,
          "change_percent": -0.4229631101954734
        },
        "peak_RSS_MiB": {
          "baseline": 4.222193689974923,
          "candidate": 4.16015625,
          "change_percent": -1.469317717996299
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.07420332161715,
          "candidate": 200.07420341479923,
          "change_percent": 4.657376706518335e-08
        },
        "CPU_ns_per_byte": {
          "baseline": 2.6039356327342515,
          "candidate": 3.2319308217945784,
          "change_percent": 24.117154862269132
        },
        "CPU_core_percent": {
          "baseline": 6.512137899817604,
          "candidate": 8.08276318206537,
          "change_percent": 24.118427871310246
        },
        "peak_RSS_MiB": {
          "baseline": 4.28515625,
          "candidate": 4.386656138244176,
          "change_percent": 2.3686391422524267
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
          "baseline": 200.68433723177998,
          "candidate": 200.68077453377197,
          "change_percent": -0.001775274571580443
        },
        "CPU_ns_per_byte": {
          "baseline": 2.648572086127272,
          "candidate": 2.6510445202947674,
          "change_percent": 0.09334970267358411
        },
        "CPU_core_percent": {
          "baseline": 6.644030853973082,
          "candidate": 6.650005452329223,
          "change_percent": 0.08992430178988986
        },
        "peak_RSS_MiB": {
          "baseline": 5.658775819416604,
          "candidate": 5.78515625,
          "change_percent": 2.2333528419654636
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.68389948981806,
          "candidate": 200.6873653062783,
          "change_percent": 0.0017270027486304684
        },
        "CPU_ns_per_byte": {
          "baseline": 2.6713065031412357,
          "candidate": 3.2180697150382884,
          "change_percent": 20.468007368458263
        },
        "CPU_core_percent": {
          "baseline": 6.700989620633492,
          "candidate": 8.07275726405042,
          "change_percent": 20.47112025353719
        },
        "peak_RSS_MiB": {
          "baseline": 5.78515625,
          "candidate": 5.906921671464122,
          "change_percent": 2.104790539825485
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
          "baseline": 200.06854778011476,
          "candidate": 200.07054950323294,
          "change_percent": 0.0010005186424244528
        },
        "CPU_ns_per_byte": {
          "baseline": 2.847667528469526,
          "candidate": 2.9009614386217923,
          "change_percent": 1.8714934106408565
        },
        "CPU_core_percent": {
          "baseline": 7.1214897166806175,
          "candidate": 7.254903382678529,
          "change_percent": 1.8733954734978742
        },
        "peak_RSS_MiB": {
          "baseline": 4.16015625,
          "candidate": 4.222193689974923,
          "change_percent": 1.4912286040920542
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.0694907800391,
          "candidate": 200.0694349941297,
          "change_percent": -2.7883266551231856e-05
        },
        "CPU_ns_per_byte": {
          "baseline": 2.8379846772299415,
          "candidate": 3.41868175655159,
          "change_percent": 20.46160023275556
        },
        "CPU_core_percent": {
          "baseline": 7.097369167463868,
          "candidate": 8.549607059928665,
          "change_percent": 20.461636674082296
        },
        "peak_RSS_MiB": {
          "baseline": 4.16015625,
          "candidate": 4.388619287207183,
          "change_percent": 5.491693664322894
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
          "baseline": 200.66313433716402,
          "candidate": 200.66428791194187,
          "change_percent": 0.0005748812713646245
        },
        "CPU_ns_per_byte": {
          "baseline": 2.8019151532347695,
          "candidate": 2.798849540200175,
          "change_percent": -0.10941134427483457
        },
        "CPU_core_percent": {
          "baseline": 7.027888253620081,
          "candidate": 7.020297425006393,
          "change_percent": -0.10801009264451711
        },
        "peak_RSS_MiB": {
          "baseline": 4.347206990489878,
          "candidate": 4.529631647961116,
          "change_percent": 4.196364651380025
        }
      },
      "result": {
        "Mbps": {
          "baseline": 200.66410605111287,
          "candidate": 200.6655383473611,
          "change_percent": 0.0007137780026589269
        },
        "CPU_ns_per_byte": {
          "baseline": 2.7969733350072006,
          "candidate": 3.44550007520385,
          "change_percent": 23.186733033154905
        },
        "CPU_core_percent": {
          "baseline": 7.015589162535532,
          "candidate": 8.642342205210351,
          "change_percent": 23.187689657797584
        },
        "peak_RSS_MiB": {
          "baseline": 4.58951794056389,
          "candidate": 4.712990587030437,
          "change_percent": 2.6903184183081574
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
