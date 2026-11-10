# Stage 3 Travel raw evidence

Sanitized measurement output: no password, private key, saved-configuration fingerprints, private router identity, local source addresses or temporary directory name is published. `main` is baseline PR #7; `erase` is the latest PR code with negotiated vnet. Source and binary hashes identify the tested artifacts.

## Measurements

```json
{
  "samples": [
    {
      "build": "main",
      "protocol": "udp",
      "role": "aa",
      "slot": 0,
      "size": 100000000,
      "warmups": [
        {
          "requested_bytes": 12000000,
          "valid": false,
          "error": "SSLEOFError: [SSL: UNEXPECTED_EOF_WHILE_READING] EOF occurred in violation of protocol (_ssl.c:1010)",
          "seconds": 0.24271300000009433
        },
        {
          "requested_bytes": 12000000,
          "valid": false,
          "error": "SSLEOFError: [SSL: UNEXPECTED_EOF_WHILE_READING] EOF occurred in violation of protocol (_ssl.c:1010)",
          "seconds": 0.5991878999998335
        },
        {
          "requested_bytes": 12000000,
          "valid": false,
          "error": "SSLEOFError: [SSL: UNEXPECTED_EOF_WHILE_READING] EOF occurred in violation of protocol (_ssl.c:1010)",
          "seconds": 0.24953999999979715
        }
      ],
      "warmup_forward_counts": {
        "in": 676,
        "out": 5460
      },
      "warmup_state": {
        "sequence": 4,
        "state": "TunnelEstablished",
        "elapsed": {
          "secs": 1,
          "nanos": 64484409
        },
        "mono_timestamp": {
          "secs": 93996,
          "nanos": 743200279
        },
        "stats_generation": 0,
        "statistics": {
          "handshake_age": {
            "secs": 1,
            "nanos": 25503593
          },
          "sent_bytes": 5637,
          "received_bytes": 4564,
          "estimated_loss": 0.0,
          "handshake_rtt": {
            "secs": 0,
            "nanos": 38000000
          }
        },
        "counters": {
          "tun_reads": 22,
          "network_reads": 19,
          "decrypted_packets": 18,
          "invalid_packets": 0,
          "outbound_filtered": 0,
          "inbound_filtered": 0,
          "pool_pressure": 0,
          "polls": 31,
          "tun_backpressure": 0,
          "network_backpressure": 0,
          "io": {
            "receive_calls": 29,
            "send_calls": 19,
            "received_datagrams": 19,
            "sent_datagrams": 24,
            "max_receive_batch": 2,
            "max_send_batch": 2,
            "truncated_datagrams": 0,
            "gso_datagrams": 8,
            "gro_datagrams": 0,
            "kernel_fallbacks": 0,
            "enobufs": 0,
            "received_bytes": 5372,
            "sent_bytes": 6521,
            "receive_would_block": 15,
            "send_would_block": 0,
            "receive_interrupted": 0,
            "send_interrupted": 0
          },
          "to_network_high_water": 3,
          "to_tun_high_water": 2,
          "pool_reserved_bytes": 405758,
          "outer_eof": 0,
          "outer_io_errors": 0,
          "in_place_encryptions": 22,
          "in_place_decryptions": 18,
          "successful_crypto_copy_bytes": 0,
          "pool_recycle_wipe_bytes": 0,
          "to_network_bytes_high_water": 0,
          "to_tun_bytes_high_water": 0
        },
        "tun_present": true,
        "interface_error": null
      },
      "valid": false,
      "error": "RuntimeError: warmup failed"
    },
    {
      "build": "main",
      "protocol": "udp",
      "role": "aa",
      "slot": 1,
      "size": 100000000,
      "warmups": [
        {
          "requested_bytes": 12000000,
          "valid": false,
          "error": "SSLEOFError: [SSL: UNEXPECTED_EOF_WHILE_READING] EOF occurred in violation of protocol (_ssl.c:1010)",
          "seconds": 0.24762379999992845
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.990448699999888,
          "Mbps": 96.92576707911358
        }
      ],
      "system_before": "cpu  2094073 0 476674 16083846 104 38312 45176 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 6.298432399999911,
        "Mbps": 127.01573172397808
      },
      "worker": {
        "cpu_core_percent": 51.74396119478247,
        "process_cpu_ns": 3259502510,
        "rss_kib": 7436,
        "seconds": 6.299290651,
        "snapshot": {
          "counters": {
            "decrypted_packets": 82946,
            "in_place_decryptions": 82946,
            "in_place_encryptions": 13407,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 80182,
              "gso_datagrams": 12004,
              "kernel_fallbacks": 0,
              "max_receive_batch": 16,
              "max_send_batch": 16,
              "receive_calls": 6932,
              "receive_interrupted": 0,
              "receive_would_block": 2628,
              "received_bytes": 119339740,
              "received_datagrams": 82947,
              "send_calls": 3440,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 1177380,
              "sent_datagrams": 13409,
              "truncated_datagrams": 0
            },
            "network_backpressure": 0,
            "network_reads": 82947,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 4223,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 31,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_reads": 13407
          },
          "elapsed": {
            "nanos": 780765926,
            "secs": 7
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 490436466,
            "secs": 94006
          },
          "sequence": 12,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.11295289546251297,
            "handshake_age": {
              "nanos": 749256472,
              "secs": 7
            },
            "handshake_rtt": {
              "nanos": 30000000,
              "secs": 0
            },
            "received_bytes": 115534659,
            "sent_bytes": 748176
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "raw"
      },
      "system_after": "cpu  2094439 0 477049 16084260 104 38337 45232 0 0 0\n",
      "forwarded_bytes": {
        "in": 103148703,
        "out": 659086
      },
      "valid": true,
      "cpu_ns_per_byte": 32.5950251
    },
    {
      "build": "main",
      "protocol": "udp",
      "role": "aa",
      "slot": 2,
      "size": 100000000,
      "warmups": [
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.9552085000000261,
          "Mbps": 100.50161823308459
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 1.191080200000215,
          "Mbps": 80.59910659247184
        }
      ],
      "system_before": "cpu  2094769 0 477264 16084529 104 38346 45249 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 4.100515399999949,
        "Mbps": 195.09742604551855
      },
      "worker": {
        "cpu_core_percent": 75.42401693961713,
        "process_cpu_ns": 3093429867,
        "rss_kib": 6952,
        "seconds": 4.101385729,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91866,
            "in_place_decryptions": 91866,
            "in_place_encryptions": 14377,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 90007,
              "gso_datagrams": 13433,
              "kernel_fallbacks": 0,
              "max_receive_batch": 16,
              "max_send_batch": 16,
              "receive_calls": 6295,
              "receive_interrupted": 0,
              "receive_would_block": 2186,
              "received_bytes": 132133772,
              "received_datagrams": 91867,
              "send_calls": 3111,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 1279334,
              "sent_datagrams": 14379,
              "truncated_datagrams": 0
            },
            "network_backpressure": 0,
            "network_reads": 91867,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 3645,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 28,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_reads": 14377
          },
          "elapsed": {
            "nanos": 487735347,
            "secs": 6
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 982482394,
            "secs": 94014
          },
          "sequence": 11,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.06626009941101074,
            "handshake_age": {
              "nanos": 441677266,
              "secs": 6
            },
            "handshake_rtt": {
              "nanos": 42000000,
              "secs": 0
            },
            "received_bytes": 127924387,
            "sent_bytes": 819090
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "raw"
      },
      "system_after": "cpu  2094862 0 477551 16084913 104 38361 45293 0 0 0\n",
      "forwarded_bytes": {
        "in": 103150890,
        "out": 636930
      },
      "valid": true,
      "cpu_ns_per_byte": 30.93429867
    },
    {
      "build": "main",
      "protocol": "udp",
      "role": "aa",
      "slot": 3,
      "size": 100000000,
      "warmups": [
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.7589578000001893,
          "Mbps": 126.48924617412992
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.7902798000000075,
          "Mbps": 121.47596332336862
        }
      ],
      "system_before": "cpu  2095004 0 477724 16085278 104 38368 45309 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 4.123841900000116,
        "Mbps": 193.9938580089546
      },
      "worker": {
        "cpu_core_percent": 75.12149048520087,
        "process_cpu_ns": 3098479514,
        "rss_kib": 6896,
        "seconds": 4.12462465,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91893,
            "in_place_decryptions": 91893,
            "in_place_encryptions": 15573,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 88919,
              "gso_datagrams": 14790,
              "kernel_fallbacks": 0,
              "max_receive_batch": 16,
              "max_send_batch": 16,
              "receive_calls": 5271,
              "receive_interrupted": 0,
              "receive_would_block": 1632,
              "received_bytes": 132137436,
              "received_datagrams": 91894,
              "send_calls": 2849,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 1453154,
              "sent_datagrams": 15575,
              "truncated_datagrams": 0
            },
            "network_backpressure": 0,
            "network_reads": 91894,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 3020,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 23,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_reads": 15573
          },
          "elapsed": {
            "nanos": 903249195,
            "secs": 5
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 715421049,
            "secs": 94022
          },
          "sequence": 10,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.10093045234680176,
            "handshake_age": {
              "nanos": 874142095,
              "secs": 5
            },
            "handshake_rtt": {
              "nanos": 28000000,
              "secs": 0
            },
            "received_bytes": 127925201,
            "sent_bytes": 954638
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "raw"
      },
      "system_after": "cpu  2095138 0 478038 16085578 104 38387 45358 0 0 0\n",
      "forwarded_bytes": {
        "in": 103153800,
        "out": 769670
      },
      "valid": true,
      "cpu_ns_per_byte": 30.98479514
    },
    {
      "build": "main",
      "protocol": "udp",
      "role": "compare",
      "slot": 0,
      "size": 100000000,
      "warmups": [
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.7927630000003774,
          "Mbps": 121.09545980318747
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.854834899999787,
          "Mbps": 112.30238727972375
        }
      ],
      "system_before": "cpu  2095214 0 478209 16086027 104 38396 45374 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 4.4122206000001825,
        "Mbps": 181.3145970081294
      },
      "worker": {
        "cpu_core_percent": 72.6347258101057,
        "process_cpu_ns": 3205609042,
        "rss_kib": 6732,
        "seconds": 4.413328482,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91883,
            "in_place_decryptions": 91883,
            "in_place_encryptions": 14980,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 89116,
              "gso_datagrams": 13488,
              "kernel_fallbacks": 0,
              "max_receive_batch": 16,
              "max_send_batch": 16,
              "receive_calls": 7907,
              "receive_interrupted": 0,
              "receive_would_block": 2969,
              "received_bytes": 132135404,
              "received_datagrams": 91884,
              "send_calls": 3865,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 1294238,
              "sent_datagrams": 14982,
              "truncated_datagrams": 0
            },
            "network_backpressure": 0,
            "network_reads": 91884,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 4665,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 25,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_reads": 14980
          },
          "elapsed": {
            "nanos": 300615927,
            "secs": 6
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 864334601,
            "secs": 94030
          },
          "sequence": 10,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.06819933652877808,
            "handshake_age": {
              "nanos": 270506736,
              "secs": 6
            },
            "handshake_rtt": {
              "nanos": 29000000,
              "secs": 0
            },
            "received_bytes": 127923512,
            "sent_bytes": 814698
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "raw"
      },
      "system_after": "cpu  2095375 0 478494 16086381 104 38413 45423 0 0 0\n",
      "forwarded_bytes": {
        "in": 103149946,
        "out": 640522
      },
      "valid": true,
      "cpu_ns_per_byte": 32.05609042
    },
    {
      "build": "erase",
      "protocol": "udp",
      "role": "compare",
      "slot": 1,
      "size": 100000000,
      "warmups": [
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.827007599999888,
          "Mbps": 116.08115814173048
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.8091976000000614,
          "Mbps": 118.63604143165121
        }
      ],
      "system_before": "cpu  2095504 0 478654 16086788 104 38420 45440 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 3.1627134000000297,
        "Mbps": 252.9473584296296
      },
      "worker": {
        "cpu_core_percent": 62.214690298318494,
        "process_cpu_ns": 1968545260,
        "rss_kib": 9316,
        "seconds": 3.164116466,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91939,
            "in_place_decryptions": 91939,
            "in_place_encryptions": 17736,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 90436,
              "gso_datagrams": 16905,
              "kernel_fallbacks": 0,
              "max_receive_batch": 16,
              "max_send_batch": 16,
              "receive_calls": 6090,
              "receive_interrupted": 0,
              "receive_would_block": 2105,
              "received_bytes": 132141612,
              "received_datagrams": 91940,
              "send_calls": 3213,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 1726354,
              "sent_datagrams": 17738,
              "truncated_datagrams": 0
            },
            "network_backpressure": 0,
            "network_reads": 91940,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 4010,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 58,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 91446,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 17736,
            "tun_write_calls": 6398,
            "tun_written_packets": 91939
          },
          "elapsed": {
            "nanos": 44849016,
            "secs": 5
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 755110981,
            "secs": 94037
          },
          "sequence": 9,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.10447573661804199,
            "handshake_age": {
              "nanos": 3183929,
              "secs": 5
            },
            "handshake_rtt": {
              "nanos": 41000000,
              "secs": 0
            },
            "received_bytes": 127928335,
            "sent_bytes": 1158622
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2095607 0 478813 16087097 104 38433 45485 0 0 0\n",
      "forwarded_bytes": {
        "in": 100380649,
        "out": 893566
      },
      "valid": true,
      "cpu_ns_per_byte": 19.6854526
    },
    {
      "build": "erase",
      "protocol": "udp",
      "role": "compare",
      "slot": 2,
      "size": 100000000,
      "warmups": [
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 1.240769200000159,
          "Mbps": 77.37135963722157
        }
      ],
      "system_before": "cpu  2095985 0 479000 16087307 104 38440 45499 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 4.283598999999867,
        "Mbps": 186.7588446070757
      },
      "worker": {
        "cpu_core_percent": 52.18562974758214,
        "process_cpu_ns": 2236321988,
        "rss_kib": 8404,
        "seconds": 4.285321455,
        "snapshot": {
          "counters": {
            "decrypted_packets": 82959,
            "in_place_decryptions": 82959,
            "in_place_encryptions": 13873,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 80349,
              "gso_datagrams": 12238,
              "kernel_fallbacks": 0,
              "max_receive_batch": 16,
              "max_send_batch": 16,
              "receive_calls": 8702,
              "receive_interrupted": 0,
              "receive_would_block": 3547,
              "received_bytes": 119344828,
              "received_datagrams": 82960,
              "send_calls": 3878,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 1265932,
              "sent_datagrams": 13875,
              "truncated_datagrams": 0
            },
            "network_backpressure": 0,
            "network_reads": 82960,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 5474,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 36,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 82339,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 13873,
            "tun_write_calls": 6613,
            "tun_written_packets": 82959
          },
          "elapsed": {
            "nanos": 778665206,
            "secs": 5
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 136852260,
            "secs": 94046
          },
          "sequence": 10,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.08396363258361816,
            "handshake_age": {
              "nanos": 746396873,
              "secs": 5
            },
            "handshake_rtt": {
              "nanos": 31000000,
              "secs": 0
            },
            "received_bytes": 115540079,
            "sent_bytes": 821816
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2096297 0 479235 16087527 104 38460 45553 0 0 0\n",
      "forwarded_bytes": {
        "in": 100423610,
        "out": 707674
      },
      "valid": true,
      "cpu_ns_per_byte": 22.36321988
    },
    {
      "build": "main",
      "protocol": "udp",
      "role": "compare",
      "slot": 3,
      "size": 100000000,
      "warmups": [
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.909606800000347,
          "Mbps": 105.54010809941546
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.8708733000003122,
          "Mbps": 110.2341752812557
        }
      ],
      "system_before": "cpu  2096382 0 479399 16088011 104 38467 45568 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 4.869220399999904,
        "Mbps": 164.2973482983058
      },
      "worker": {
        "cpu_core_percent": 66.32614098964959,
        "process_cpu_ns": 3230337027,
        "rss_kib": 6920,
        "seconds": 4.870382897,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91880,
            "in_place_decryptions": 91880,
            "in_place_encryptions": 13186,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 89833,
              "gso_datagrams": 10742,
              "kernel_fallbacks": 0,
              "max_receive_batch": 16,
              "max_send_batch": 16,
              "receive_calls": 9635,
              "receive_interrupted": 0,
              "receive_would_block": 4009,
              "received_bytes": 132137116,
              "received_datagrams": 91881,
              "send_calls": 4648,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 1117278,
              "sent_datagrams": 13188,
              "truncated_datagrams": 0
            },
            "network_backpressure": 0,
            "network_reads": 91881,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 6132,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 27,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_reads": 13186
          },
          "elapsed": {
            "nanos": 901064662,
            "secs": 6
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 912010176,
            "secs": 94054
          },
          "sequence": 10,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.06144338846206665,
            "handshake_age": {
              "nanos": 862170943,
              "secs": 6
            },
            "handshake_rtt": {
              "nanos": 38000000,
              "secs": 0
            },
            "received_bytes": 127924291,
            "sent_bytes": 695146
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "raw"
      },
      "system_after": "cpu  2096536 0 479702 16088441 104 38486 45615 0 0 0\n",
      "forwarded_bytes": {
        "in": 103151313,
        "out": 530586
      },
      "valid": true,
      "cpu_ns_per_byte": 32.30337027
    },
    {
      "build": "main",
      "protocol": "tcp",
      "role": "aa",
      "slot": 0,
      "size": 100000000,
      "warmups": [
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.9114910999996937,
          "Mbps": 105.32192799253033
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.8425222000000758,
          "Mbps": 113.9435851067086
        }
      ],
      "system_before": "cpu  2096679 0 479872 16088852 104 38494 45634 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 4.2988711000002695,
        "Mbps": 186.09536815373457
      },
      "worker": {
        "cpu_core_percent": 69.26548931116008,
        "process_cpu_ns": 2978871977,
        "rss_kib": 2656,
        "seconds": 4.300658245,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91910,
            "in_place_decryptions": 91910,
            "in_place_encryptions": 12528,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 9330,
              "receive_interrupted": 0,
              "receive_would_block": 745,
              "received_bytes": 130856442,
              "received_datagrams": 0,
              "send_calls": 1861,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 732298,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 2,
            "network_reads": 91911,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 2531,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 62,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_reads": 12528
          },
          "elapsed": {
            "nanos": 304736300,
            "secs": 6
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 72212736,
            "secs": 94063
          },
          "sequence": 10,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.0,
            "handshake_age": {
              "nanos": 232579467,
              "secs": 6
            },
            "handshake_rtt": {
              "nanos": 71000000,
              "secs": 0
            },
            "received_bytes": 127927524,
            "sent_bytes": 506610
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "raw"
      },
      "system_after": "cpu  2096849 0 480147 16089193 104 38514 45684 0 0 0\n",
      "forwarded_bytes": {
        "in": 103152292,
        "out": 402902
      },
      "valid": true,
      "cpu_ns_per_byte": 29.78871977
    },
    {
      "build": "main",
      "protocol": "tcp",
      "role": "aa",
      "slot": 1,
      "size": 100000000,
      "warmups": [
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.9380080999999336,
          "Mbps": 102.34453199285464
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.8329717000001438,
          "Mbps": 115.25001389601043
        }
      ],
      "system_before": "cpu  2096925 0 480332 16089666 104 38523 45704 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 4.007697500000177,
        "Mbps": 199.61586422128036
      },
      "worker": {
        "cpu_core_percent": 75.21097844781539,
        "process_cpu_ns": 3015572554,
        "rss_kib": 2640,
        "seconds": 4.009484541,
        "snapshot": {
          "counters": {
            "decrypted_packets": 92450,
            "in_place_decryptions": 92450,
            "in_place_encryptions": 12816,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 9519,
              "receive_interrupted": 0,
              "receive_would_block": 812,
              "received_bytes": 131604706,
              "received_datagrams": 0,
              "send_calls": 1932,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 762578,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 2,
            "network_reads": 92451,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 2614,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 63,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_reads": 12816
          },
          "elapsed": {
            "nanos": 21060821,
            "secs": 6
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 970944937,
            "secs": 94070
          },
          "sequence": 10,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.004908204078674316,
            "handshake_age": {
              "nanos": 940866257,
              "secs": 5
            },
            "handshake_rtt": {
              "nanos": 79000000,
              "secs": 0
            },
            "received_bytes": 128659997,
            "sent_bytes": 531706
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "raw"
      },
      "system_after": "cpu  2097072 0 480643 16089952 104 38541 45753 0 0 0\n",
      "forwarded_bytes": {
        "in": 103887471,
        "out": 414210
      },
      "valid": true,
      "cpu_ns_per_byte": 30.15572554
    },
    {
      "build": "main",
      "protocol": "tcp",
      "role": "aa",
      "slot": 2,
      "size": 100000000,
      "warmups": [
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.9894268999996711,
          "Mbps": 97.02586416442882
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 1.0932932999999139,
          "Mbps": 87.80809321707866
        }
      ],
      "system_before": "cpu  2097305 0 480862 16090271 104 38549 45773 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 4.778707100000247,
        "Mbps": 167.40929779938983
      },
      "worker": {
        "cpu_core_percent": 64.05300128231534,
        "process_cpu_ns": 3061985057,
        "rss_kib": 2648,
        "seconds": 4.780392793,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91930,
            "in_place_decryptions": 91930,
            "in_place_encryptions": 12794,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 8907,
              "receive_interrupted": 0,
              "receive_would_block": 515,
              "received_bytes": 130856066,
              "received_datagrams": 0,
              "send_calls": 1559,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 747726,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 2,
            "network_reads": 91931,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 2227,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 61,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_reads": 12794
          },
          "elapsed": {
            "nanos": 110321519,
            "secs": 7
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 882030159,
            "secs": 94079
          },
          "sequence": 11,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.0,
            "handshake_age": {
              "nanos": 38242627,
              "secs": 7
            },
            "handshake_rtt": {
              "nanos": 69000000,
              "secs": 0
            },
            "received_bytes": 127925740,
            "sent_bytes": 517250
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "raw"
      },
      "system_after": "cpu  2097720 0 481255 16090337 104 38576 45835 0 0 0\n",
      "forwarded_bytes": {
        "in": 103150283,
        "out": 413902
      },
      "valid": true,
      "cpu_ns_per_byte": 30.61985057
    },
    {
      "build": "main",
      "protocol": "tcp",
      "role": "aa",
      "slot": 3,
      "size": 100000000,
      "warmups": [
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.8145589999999174,
          "Mbps": 117.8551829885984
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.8641983000002256,
          "Mbps": 111.08561541948757
        }
      ],
      "system_before": "cpu  2097849 0 481424 16090740 104 38583 45852 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 3.995270800000071,
        "Mbps": 200.23673989757734
      },
      "worker": {
        "cpu_core_percent": 74.87902448643314,
        "process_cpu_ns": 2992822831,
        "rss_kib": 2600,
        "seconds": 3.996877432,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91936,
            "in_place_decryptions": 91936,
            "in_place_encryptions": 11781,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 9208,
              "receive_interrupted": 0,
              "receive_would_block": 672,
              "received_bytes": 130851886,
              "received_datagrams": 0,
              "send_calls": 1868,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 692524,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 2,
            "network_reads": 91937,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 2473,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 55,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_reads": 11781
          },
          "elapsed": {
            "nanos": 918885729,
            "secs": 5
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 678476184,
            "secs": 94087
          },
          "sequence": 9,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 1.0848045349121094e-05,
            "handshake_age": {
              "nanos": 850898065,
              "secs": 5
            },
            "handshake_rtt": {
              "nanos": 67000000,
              "secs": 0
            },
            "received_bytes": 127924064,
            "sent_bytes": 480282
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "raw"
      },
      "system_after": "cpu  2098004 0 481722 16091019 104 38603 45902 0 0 0\n",
      "forwarded_bytes": {
        "in": 103150058,
        "out": 376698
      },
      "valid": true,
      "cpu_ns_per_byte": 29.92822831
    },
    {
      "build": "main",
      "protocol": "tcp",
      "role": "compare",
      "slot": 0,
      "size": 100000000,
      "warmups": [
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.8684465000001182,
          "Mbps": 110.5422153235541
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 1.4091997999998966,
          "Mbps": 68.12376782909494
        }
      ],
      "system_before": "cpu  2098111 0 481913 16091543 104 38611 45921 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 3.9467859999999746,
        "Mbps": 202.6965738704873
      },
      "worker": {
        "cpu_core_percent": 75.56271907518119,
        "process_cpu_ns": 2983545966,
        "rss_kib": 2456,
        "seconds": 3.948436481,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91932,
            "in_place_decryptions": 91932,
            "in_place_encryptions": 12472,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 9348,
              "receive_interrupted": 0,
              "receive_would_block": 759,
              "received_bytes": 130853414,
              "received_datagrams": 0,
              "send_calls": 1931,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 764594,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 2,
            "network_reads": 91933,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 2549,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 39,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_reads": 12472
          },
          "elapsed": {
            "nanos": 478724577,
            "secs": 6
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 142698,
            "secs": 94096
          },
          "sequence": 10,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.008359670639038086,
            "handshake_age": {
              "nanos": 409830384,
              "secs": 6
            },
            "handshake_rtt": {
              "nanos": 68000000,
              "secs": 0
            },
            "received_bytes": 127924779,
            "sent_bytes": 539914
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "raw"
      },
      "system_after": "cpu  2098206 0 482207 16091878 104 38628 45970 0 0 0\n",
      "forwarded_bytes": {
        "in": 103150694,
        "out": 427022
      },
      "valid": true,
      "cpu_ns_per_byte": 29.83545966
    },
    {
      "build": "erase",
      "protocol": "tcp",
      "role": "compare",
      "slot": 1,
      "size": 100000000,
      "warmups": [
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.7547351999996863,
          "Mbps": 127.19692946617556
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.7250857000003634,
          "Mbps": 132.39814272982062
        }
      ],
      "system_before": "cpu  2098336 0 482361 16092277 104 38634 45986 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 3.2661818999999923,
        "Mbps": 244.93430693495728
      },
      "worker": {
        "cpu_core_percent": 59.512926731180144,
        "process_cpu_ns": 1945088837,
        "rss_kib": 2500,
        "seconds": 3.268346801,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91994,
            "in_place_decryptions": 91994,
            "in_place_encryptions": 11920,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 11417,
              "receive_interrupted": 0,
              "receive_would_block": 2017,
              "received_bytes": 130854002,
              "received_datagrams": 0,
              "send_calls": 2786,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 723686,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 2,
            "network_reads": 91995,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 4137,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 42,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 91448,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 11920,
            "tun_write_calls": 6349,
            "tun_written_packets": 91994
          },
          "elapsed": {
            "nanos": 990413028,
            "secs": 4
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 890231822,
            "secs": 94102
          },
          "sequence": 9,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.00321805477142334,
            "handshake_age": {
              "nanos": 916109591,
              "secs": 4
            },
            "handshake_rtt": {
              "nanos": 73000000,
              "secs": 0
            },
            "received_bytes": 127927551,
            "sent_bytes": 508942
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2098429 0 482531 16092600 104 38648 46034 0 0 0\n",
      "forwarded_bytes": {
        "in": 100380535,
        "out": 418194
      },
      "valid": true,
      "cpu_ns_per_byte": 19.45088837
    },
    {
      "build": "erase",
      "protocol": "tcp",
      "role": "compare",
      "slot": 2,
      "size": 100000000,
      "warmups": [
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.7396693000000596,
          "Mbps": 129.78773081428724
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 1.5440269999999146,
          "Mbps": 62.17507854461438
        }
      ],
      "system_before": "cpu  2098559 0 482680 16093153 104 38654 46052 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 3.7523120999999264,
        "Mbps": 213.2018815812298
      },
      "worker": {
        "cpu_core_percent": 51.89913554047951,
        "process_cpu_ns": 1950922283,
        "rss_kib": 2648,
        "seconds": 3.759065084,
        "snapshot": {
          "counters": {
            "decrypted_packets": 92002,
            "in_place_decryptions": 92002,
            "in_place_encryptions": 11362,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 10508,
              "receive_interrupted": 0,
              "receive_would_block": 1456,
              "received_bytes": 130852194,
              "received_datagrams": 0,
              "send_calls": 2277,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 693130,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 2,
            "network_reads": 92003,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 3442,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 61,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 91451,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 11362,
            "tun_write_calls": 6209,
            "tun_written_packets": 92002
          },
          "elapsed": {
            "nanos": 296643291,
            "secs": 6
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 40614702,
            "secs": 94111
          },
          "sequence": 10,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.009826183319091797,
            "handshake_age": {
              "nanos": 224479531,
              "secs": 6
            },
            "handshake_rtt": {
              "nanos": 71000000,
              "secs": 0
            },
            "received_bytes": 127925689,
            "sent_bytes": 488430
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2098855 0 482902 16093323 104 38670 46101 0 0 0\n",
      "forwarded_bytes": {
        "in": 100373117,
        "out": 387086
      },
      "valid": true,
      "cpu_ns_per_byte": 19.50922283
    },
    {
      "build": "main",
      "protocol": "tcp",
      "role": "compare",
      "slot": 3,
      "size": 100000000,
      "warmups": [
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.9056189000002632,
          "Mbps": 106.00485480147564
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.8331144999997377,
          "Mbps": 115.23025946617209
        }
      ],
      "system_before": "cpu  2099192 0 483149 16093498 104 38679 46122 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 3.997086699999727,
        "Mbps": 200.14577116880017
      },
      "worker": {
        "cpu_core_percent": 75.1969218526218,
        "process_cpu_ns": 3007157502,
        "rss_kib": 2572,
        "seconds": 3.999043349,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91985,
            "in_place_decryptions": 91985,
            "in_place_encryptions": 11982,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 9202,
              "receive_interrupted": 0,
              "receive_would_block": 654,
              "received_bytes": 130857312,
              "received_datagrams": 0,
              "send_calls": 1850,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 703702,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 2,
            "network_reads": 91986,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 2462,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 51,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_reads": 11982
          },
          "elapsed": {
            "nanos": 8547262,
            "secs": 6
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 70516305,
            "secs": 94119
          },
          "sequence": 9,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.00019562244415283203,
            "handshake_age": {
              "nanos": 922379161,
              "secs": 5
            },
            "handshake_rtt": {
              "nanos": 84000000,
              "secs": 0
            },
            "received_bytes": 127929862,
            "sent_bytes": 487842
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "raw"
      },
      "system_after": "cpu  2099331 0 483451 16093793 104 38697 46171 0 0 0\n",
      "forwarded_bytes": {
        "in": 103152126,
        "out": 383174
      },
      "valid": true,
      "cpu_ns_per_byte": 30.07157502
    },
    {
      "build": "main",
      "protocol": "stealth",
      "role": "aa",
      "slot": 0,
      "size": 100000000,
      "warmups": [
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 5.956886399999803,
          "Mbps": 16.11580170473004
        }
      ],
      "system_before": "cpu  2099515 0 483606 16095010 104 38705 46185 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 4.866038900000149,
        "Mbps": 164.40476873293707
      },
      "worker": {
        "cpu_core_percent": 78.87465542500449,
        "process_cpu_ns": 3839492436,
        "rss_kib": 3276,
        "seconds": 4.867840519,
        "snapshot": {
          "counters": {
            "decrypted_packets": 82977,
            "in_place_decryptions": 82977,
            "in_place_encryptions": 10666,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 2763,
              "receive_interrupted": 0,
              "receive_would_block": 414,
              "received_bytes": 121837388,
              "received_datagrams": 0,
              "send_calls": 1591,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 661591,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 3,
            "network_reads": 82979,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 1935,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 37,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_reads": 10666
          },
          "elapsed": {
            "nanos": 59836342,
            "secs": 11
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 959456475,
            "secs": 94131
          },
          "sequence": 14,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 1.8537044525146484e-05,
            "handshake_age": {
              "nanos": 992481089,
              "secs": 0
            },
            "handshake_rtt": {
              "nanos": 61000000,
              "secs": 0
            },
            "received_bytes": 115539489,
            "sent_bytes": 433828
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "raw"
      },
      "system_after": "cpu  2099695 0 483920 16095419 104 38722 46237 0 0 0\n",
      "forwarded_bytes": {
        "in": 103149221,
        "out": 377714
      },
      "valid": true,
      "cpu_ns_per_byte": 38.39492436
    },
    {
      "build": "main",
      "protocol": "stealth",
      "role": "aa",
      "slot": 1,
      "size": 100000000,
      "warmups": [
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 5.708072100000209,
          "Mbps": 16.81828791195481
        }
      ],
      "system_before": "cpu  2099888 0 484067 16096601 104 38728 46250 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 5.47690270000021,
        "Mbps": 146.0679591769942
      },
      "worker": {
        "cpu_core_percent": 69.08619707842232,
        "process_cpu_ns": 3784770753,
        "rss_kib": 3064,
        "seconds": 5.478331292,
        "snapshot": {
          "counters": {
            "decrypted_packets": 83016,
            "in_place_decryptions": 83016,
            "in_place_encryptions": 10782,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 2578,
              "receive_interrupted": 0,
              "receive_would_block": 323,
              "received_bytes": 121844927,
              "received_datagrams": 0,
              "send_calls": 1498,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 667768,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 3,
            "network_reads": 83018,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 1811,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 39,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_reads": 10782
          },
          "elapsed": {
            "nanos": 426389795,
            "secs": 11
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 269983310,
            "secs": 94145
          },
          "sequence": 14,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.0003066509962081909,
            "handshake_age": {
              "nanos": 257635951,
              "secs": 1
            },
            "handshake_rtt": {
              "nanos": 160000000,
              "secs": 0
            },
            "received_bytes": 115543504,
            "sent_bytes": 439965
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "raw"
      },
      "system_after": "cpu  2100206 0 484443 16096908 104 38751 46313 0 0 0\n",
      "forwarded_bytes": {
        "in": 103149198,
        "out": 386858
      },
      "valid": true,
      "cpu_ns_per_byte": 37.84770753
    },
    {
      "build": "main",
      "protocol": "stealth",
      "role": "aa",
      "slot": 2,
      "size": 100000000,
      "warmups": [
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 5.783079799999996,
          "Mbps": 16.600151358796754
        }
      ],
      "system_before": "cpu  2100622 0 484654 16097865 104 38759 46326 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 4.624527299999954,
        "Mbps": 172.9906535528524
      },
      "worker": {
        "cpu_core_percent": 81.41278631522265,
        "process_cpu_ns": 3765761085,
        "rss_kib": 2976,
        "seconds": 4.625515543,
        "snapshot": {
          "counters": {
            "decrypted_packets": 82946,
            "in_place_decryptions": 82946,
            "in_place_encryptions": 10257,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 2641,
              "receive_interrupted": 0,
              "receive_would_block": 346,
              "received_bytes": 121835073,
              "received_datagrams": 0,
              "send_calls": 1540,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 637777,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 3,
            "network_reads": 82947,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 1820,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 30,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_reads": 10257
          },
          "elapsed": {
            "nanos": 668510877,
            "secs": 10
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 26863308,
            "secs": 94158
          },
          "sequence": 13,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.0004940629005432129,
            "handshake_age": {
              "nanos": 637587272,
              "secs": 5
            },
            "handshake_rtt": {
              "nanos": 27000000,
              "secs": 0
            },
            "received_bytes": 115539221,
            "sent_bytes": 418680
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "raw"
      },
      "system_after": "cpu  2100832 0 484969 16098190 104 38777 46377 0 0 0\n",
      "forwarded_bytes": {
        "in": 103149164,
        "out": 362122
      },
      "valid": true,
      "cpu_ns_per_byte": 37.65761085
    },
    {
      "build": "main",
      "protocol": "stealth",
      "role": "aa",
      "slot": 3,
      "size": 100000000,
      "warmups": [
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 5.934906400000273,
          "Mbps": 16.17548677768458
        }
      ],
      "system_before": "cpu  2100962 0 485113 16099473 104 38784 46390 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 5.743364999999812,
        "Mbps": 139.29116467437228
      },
      "worker": {
        "cpu_core_percent": 71.84796638627208,
        "process_cpu_ns": 4127634484,
        "rss_kib": 3292,
        "seconds": 5.744956596,
        "snapshot": {
          "counters": {
            "decrypted_packets": 82944,
            "in_place_decryptions": 82944,
            "in_place_encryptions": 10733,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 9018,
              "receive_interrupted": 0,
              "receive_would_block": 2700,
              "received_bytes": 121831325,
              "received_datagrams": 0,
              "send_calls": 3282,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 699388,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 3,
            "network_reads": 82946,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 4398,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 38,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_reads": 10733
          },
          "elapsed": {
            "nanos": 933746169,
            "secs": 11
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 816711629,
            "secs": 94171
          },
          "sequence": 15,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.0,
            "handshake_age": {
              "nanos": 851512276,
              "secs": 1
            },
            "handshake_rtt": {
              "nanos": 76000000,
              "secs": 0
            },
            "received_bytes": 115536555,
            "sent_bytes": 433220
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "raw"
      },
      "system_after": "cpu  2101174 0 485441 16099953 104 38809 46453 0 0 0\n",
      "forwarded_bytes": {
        "in": 103148276,
        "out": 382018
      },
      "valid": true,
      "cpu_ns_per_byte": 41.27634484
    },
    {
      "build": "main",
      "protocol": "stealth",
      "role": "compare",
      "slot": 0,
      "size": 100000000,
      "warmups": [
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 6.115990600000259,
          "Mbps": 15.696557807004467
        }
      ],
      "system_before": "cpu  2101471 0 485635 16101058 104 38816 46470 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 5.467158700000255,
        "Mbps": 146.3282929760138
      },
      "worker": {
        "cpu_core_percent": 72.11077581995563,
        "process_cpu_ns": 3943189912,
        "rss_kib": 3372,
        "seconds": 5.468239479,
        "snapshot": {
          "counters": {
            "decrypted_packets": 82942,
            "in_place_decryptions": 82942,
            "in_place_encryptions": 11542,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 2664,
              "receive_interrupted": 0,
              "receive_would_block": 362,
              "received_bytes": 121838626,
              "received_datagrams": 0,
              "send_calls": 1446,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 711928,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 3,
            "network_reads": 82944,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 1782,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 63,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_reads": 11542
          },
          "elapsed": {
            "nanos": 828521644,
            "secs": 11
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 539290087,
            "secs": 94185
          },
          "sequence": 15,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.0077199786901474,
            "handshake_age": {
              "nanos": 774920733,
              "secs": 1
            },
            "handshake_rtt": {
              "nanos": 41000000,
              "secs": 0
            },
            "received_bytes": 115539467,
            "sent_bytes": 471588
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "raw"
      },
      "system_after": "cpu  2101976 0 486079 16101104 104 38845 46542 0 0 0\n",
      "forwarded_bytes": {
        "in": 103150136,
        "out": 415666
      },
      "valid": true,
      "cpu_ns_per_byte": 39.43189912
    },
    {
      "build": "erase",
      "protocol": "stealth",
      "role": "compare",
      "slot": 1,
      "size": 100000000,
      "warmups": [
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 5.706777400000192,
          "Mbps": 16.822103486986677
        }
      ],
      "system_before": "cpu  2102106 0 486211 16102350 104 38851 46554 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 3.6636336999999912,
        "Mbps": 218.3624416382025
      },
      "worker": {
        "cpu_core_percent": 72.79519089949143,
        "process_cpu_ns": 2668082679,
        "rss_kib": 3476,
        "seconds": 3.665190854,
        "snapshot": {
          "counters": {
            "decrypted_packets": 83012,
            "in_place_decryptions": 83012,
            "in_place_encryptions": 11046,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 2877,
              "receive_interrupted": 0,
              "receive_would_block": 435,
              "received_bytes": 121846315,
              "received_datagrams": 0,
              "send_calls": 1523,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 689631,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 3,
            "network_reads": 83013,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 1923,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 54,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 82742,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 11046,
            "tun_write_calls": 5318,
            "tun_written_packets": 83012
          },
          "elapsed": {
            "nanos": 606633169,
            "secs": 9
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 956777696,
            "secs": 94196
          },
          "sequence": 12,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.0012753009796142578,
            "handshake_age": {
              "nanos": 575417384,
              "secs": 4
            },
            "handshake_rtt": {
              "nanos": 28000000,
              "secs": 0
            },
            "received_bytes": 115544195,
            "sent_bytes": 456708
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2102329 0 486387 16102617 104 38867 46603 0 0 0\n",
      "forwarded_bytes": {
        "in": 100370996,
        "out": 390342
      },
      "valid": true,
      "cpu_ns_per_byte": 26.68082679
    },
    {
      "build": "erase",
      "protocol": "stealth",
      "role": "compare",
      "slot": 2,
      "size": 100000000,
      "warmups": [
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.9920698000000812,
          "Mbps": 96.76738471425311
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.8314838999999665,
          "Mbps": 115.45623432997785
        }
      ],
      "system_before": "cpu  2102436 0 486530 16103109 104 38875 46622 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 3.6794299000002866,
        "Mbps": 217.42498749600793
      },
      "worker": {
        "cpu_core_percent": 69.80312474975294,
        "process_cpu_ns": 2569479060,
        "rss_kib": 3248,
        "seconds": 3.681037302,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91959,
            "in_place_decryptions": 91959,
            "in_place_encryptions": 11988,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 4660,
              "receive_interrupted": 0,
              "receive_would_block": 1102,
              "received_bytes": 134899238,
              "received_datagrams": 0,
              "send_calls": 2100,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 766094,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 3,
            "network_reads": 91960,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 2958,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 49,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 91481,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 11988,
            "tun_write_calls": 6230,
            "tun_written_packets": 91959
          },
          "elapsed": {
            "nanos": 760839159,
            "secs": 5
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 592736228,
            "secs": 94204
          },
          "sequence": 9,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.00028264522552490234,
            "handshake_age": {
              "nanos": 656769235,
              "secs": 5
            },
            "handshake_rtt": {
              "nanos": 101000000,
              "secs": 0
            },
            "received_bytes": 127926932,
            "sent_bytes": 503678
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2102662 0 486699 16103393 104 38891 46668 0 0 0\n",
      "forwarded_bytes": {
        "in": 100369043,
        "out": 406666
      },
      "valid": true,
      "cpu_ns_per_byte": 25.6947906
    },
    {
      "build": "main",
      "protocol": "stealth",
      "role": "compare",
      "slot": 3,
      "size": 100000000,
      "warmups": [
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 5.981335699999818,
          "Mbps": 16.04992677471738
        }
      ],
      "system_before": "cpu  2102790 0 486850 16104687 104 38898 46681 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 6.28842719999966,
        "Mbps": 127.21782006159557
      },
      "worker": {
        "cpu_core_percent": 61.51011168347423,
        "process_cpu_ns": 3869804925,
        "rss_kib": 3148,
        "seconds": 6.291331326,
        "snapshot": {
          "counters": {
            "decrypted_packets": 82989,
            "in_place_decryptions": 82989,
            "in_place_encryptions": 11753,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 3552,
              "receive_interrupted": 0,
              "receive_would_block": 694,
              "received_bytes": 121842532,
              "received_datagrams": 0,
              "send_calls": 1656,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 728246,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 3,
            "network_reads": 82991,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 2250,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 47,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_reads": 11753
          },
          "elapsed": {
            "nanos": 512512816,
            "secs": 12
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 982883220,
            "secs": 94218
          },
          "sequence": 16,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.0003032982349395752,
            "handshake_age": {
              "nanos": 441479169,
              "secs": 2
            },
            "handshake_rtt": {
              "nanos": 66000000,
              "secs": 0
            },
            "received_bytes": 115543763,
            "sent_bytes": 479484
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "raw"
      },
      "system_after": "cpu  2103403 0 487290 16104792 104 38928 46752 0 0 0\n",
      "forwarded_bytes": {
        "in": 103151204,
        "out": 426082
      },
      "valid": true,
      "cpu_ns_per_byte": 38.69804925
    }
  ],
  "started_utc": "2026-11-10T02:44:56.670305+00:00",
  "kernel": "6.12.112",
  "server": "US-NY#544",
  "mtu": 1280,
  "method": "laptop HTTPS; socket-bound interface; owned router policy/NAT only",
  "binaries": {
    "main": "4f745c6a8b81868c2d9aa9c9f073151002805cadc608750a40c95dcbf287e4bc",
    "erase": "e542b9ede783d06e20ba63a2ccd4ed59751f5fe61816a8d47ea64909a3e4e224"
  },
  "kernel_already_available": true,
  "hardware": "CPUS 2\nMEMORY 496724\nMODEL Cudy TR3000 v1 (OpenWrt U-Boot layout)\n",
  "direct_before": {
    "requested_bytes": 100000000,
    "valid": true,
    "tls": "TLSv1.2",
    "status": 206,
    "content_range": "bytes 0-99999999/148648524",
    "bytes": 100000000,
    "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
    "seconds": 1.6429473999996844,
    "Mbps": 486.92977024106415
  },
  "shutdowns": [
    "{\"tun_reads\":25,\"network_reads\":22,\"decrypted_packets\":21,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":36,\"tun_backpressure\":0,\"network_backpressure\":0,\"io\":{\"receive_calls\":33,\"send_calls\":21,\"received_datagrams\":22,\"sent_datagrams\":27,\"max_receive_batch\":2,\"max_send_batch\":2,\"truncated_datagrams\":0,\"gso_datagrams\":8,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":5612,\"sent_bytes\":6744,\"receive_would_block\":17,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":3,\"to_tun_high_water\":2,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":25,\"in_place_decryptions\":21,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":13407,\"network_reads\":82947,\"decrypted_packets\":82946,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":4224,\"tun_backpressure\":0,\"network_backpressure\":0,\"io\":{\"receive_calls\":6932,\"send_calls\":3440,\"received_datagrams\":82947,\"sent_datagrams\":13409,\"max_receive_batch\":16,\"max_send_batch\":16,\"truncated_datagrams\":0,\"gso_datagrams\":12004,\"gro_datagrams\":80182,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":119339740,\"sent_bytes\":1177380,\"receive_would_block\":2628,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":31,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":13407,\"in_place_decryptions\":82946,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":14377,\"network_reads\":91868,\"decrypted_packets\":91867,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":3647,\"tun_backpressure\":0,\"network_backpressure\":0,\"io\":{\"receive_calls\":6297,\"send_calls\":3111,\"received_datagrams\":91868,\"sent_datagrams\":14379,\"max_receive_batch\":16,\"max_send_batch\":16,\"truncated_datagrams\":0,\"gso_datagrams\":13433,\"gro_datagrams\":90007,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":132134508,\"sent_bytes\":1279334,\"receive_would_block\":2187,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":28,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":14377,\"in_place_decryptions\":91867,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":15573,\"network_reads\":91895,\"decrypted_packets\":91894,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":3022,\"tun_backpressure\":0,\"network_backpressure\":0,\"io\":{\"receive_calls\":5273,\"send_calls\":2849,\"received_datagrams\":91895,\"sent_datagrams\":15575,\"max_receive_batch\":16,\"max_send_batch\":16,\"truncated_datagrams\":0,\"gso_datagrams\":14790,\"gro_datagrams\":88919,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":132137884,\"sent_bytes\":1453154,\"receive_would_block\":1633,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":23,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":15573,\"in_place_decryptions\":91894,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":14980,\"network_reads\":91885,\"decrypted_packets\":91884,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":4667,\"tun_backpressure\":0,\"network_backpressure\":0,\"io\":{\"receive_calls\":7909,\"send_calls\":3865,\"received_datagrams\":91885,\"sent_datagrams\":14982,\"max_receive_batch\":16,\"max_send_batch\":16,\"truncated_datagrams\":0,\"gso_datagrams\":13488,\"gro_datagrams\":89116,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":132136012,\"sent_bytes\":1294238,\"receive_would_block\":2970,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":25,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":14980,\"in_place_decryptions\":91884,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":17736,\"network_reads\":91941,\"decrypted_packets\":91940,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":4012,\"tun_backpressure\":0,\"tun_write_calls\":6399,\"tun_written_packets\":91940,\"tun_coalesced_packets\":91446,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":0,\"io\":{\"receive_calls\":6092,\"send_calls\":3213,\"received_datagrams\":91941,\"sent_datagrams\":17738,\"max_receive_batch\":16,\"max_send_batch\":16,\"truncated_datagrams\":0,\"gso_datagrams\":16905,\"gro_datagrams\":90436,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":132142220,\"sent_bytes\":1726354,\"receive_would_block\":2106,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":58,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":17736,\"in_place_decryptions\":91940,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":13873,\"network_reads\":82960,\"decrypted_packets\":82959,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":5475,\"tun_backpressure\":0,\"tun_write_calls\":6613,\"tun_written_packets\":82959,\"tun_coalesced_packets\":82339,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":0,\"io\":{\"receive_calls\":8702,\"send_calls\":3878,\"received_datagrams\":82960,\"sent_datagrams\":13875,\"max_receive_batch\":16,\"max_send_batch\":16,\"truncated_datagrams\":0,\"gso_datagrams\":12238,\"gro_datagrams\":80349,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":119344828,\"sent_bytes\":1265932,\"receive_would_block\":3547,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":36,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":13873,\"in_place_decryptions\":82959,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":13186,\"network_reads\":91881,\"decrypted_packets\":91880,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":6133,\"tun_backpressure\":0,\"network_backpressure\":0,\"io\":{\"receive_calls\":9635,\"send_calls\":4648,\"received_datagrams\":91881,\"sent_datagrams\":13188,\"max_receive_batch\":16,\"max_send_batch\":16,\"truncated_datagrams\":0,\"gso_datagrams\":10742,\"gro_datagrams\":89833,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":132137116,\"sent_bytes\":1117278,\"receive_would_block\":4009,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":27,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":13186,\"in_place_decryptions\":91880,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":12528,\"network_reads\":91911,\"decrypted_packets\":91910,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":2532,\"tun_backpressure\":0,\"network_backpressure\":2,\"io\":{\"receive_calls\":9330,\"send_calls\":1861,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":130856442,\"sent_bytes\":732298,\"receive_would_block\":745,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":62,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":12528,\"in_place_decryptions\":91910,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":12816,\"network_reads\":92451,\"decrypted_packets\":92450,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":2615,\"tun_backpressure\":0,\"network_backpressure\":2,\"io\":{\"receive_calls\":9519,\"send_calls\":1932,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":131604706,\"sent_bytes\":762578,\"receive_would_block\":812,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":63,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":12816,\"in_place_decryptions\":92450,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":12794,\"network_reads\":91931,\"decrypted_packets\":91930,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":2228,\"tun_backpressure\":0,\"network_backpressure\":2,\"io\":{\"receive_calls\":8907,\"send_calls\":1559,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":130856066,\"sent_bytes\":747726,\"receive_would_block\":515,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":61,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":12794,\"in_place_decryptions\":91930,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":11781,\"network_reads\":91937,\"decrypted_packets\":91936,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":2474,\"tun_backpressure\":0,\"network_backpressure\":2,\"io\":{\"receive_calls\":9208,\"send_calls\":1868,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":130851886,\"sent_bytes\":692524,\"receive_would_block\":672,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":55,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":11781,\"in_place_decryptions\":91936,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":12472,\"network_reads\":91933,\"decrypted_packets\":91932,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":2550,\"tun_backpressure\":0,\"network_backpressure\":2,\"io\":{\"receive_calls\":9348,\"send_calls\":1931,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":130853414,\"sent_bytes\":764594,\"receive_would_block\":759,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":39,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":12472,\"in_place_decryptions\":91932,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":11920,\"network_reads\":91995,\"decrypted_packets\":91994,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":4138,\"tun_backpressure\":0,\"tun_write_calls\":6349,\"tun_written_packets\":91994,\"tun_coalesced_packets\":91448,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":2,\"io\":{\"receive_calls\":11417,\"send_calls\":2786,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":130854002,\"sent_bytes\":723686,\"receive_would_block\":2017,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":42,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":11920,\"in_place_decryptions\":91994,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":11362,\"network_reads\":92003,\"decrypted_packets\":92002,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":3443,\"tun_backpressure\":0,\"tun_write_calls\":6209,\"tun_written_packets\":92002,\"tun_coalesced_packets\":91451,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":2,\"io\":{\"receive_calls\":10508,\"send_calls\":2277,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":130852194,\"sent_bytes\":693130,\"receive_would_block\":1456,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":61,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":11362,\"in_place_decryptions\":92002,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":11982,\"network_reads\":91986,\"decrypted_packets\":91985,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":2463,\"tun_backpressure\":0,\"network_backpressure\":2,\"io\":{\"receive_calls\":9202,\"send_calls\":1850,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":130857312,\"sent_bytes\":703702,\"receive_would_block\":654,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":51,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":11982,\"in_place_decryptions\":91985,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":10666,\"network_reads\":82979,\"decrypted_packets\":82977,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":1936,\"tun_backpressure\":0,\"network_backpressure\":3,\"io\":{\"receive_calls\":2763,\"send_calls\":1591,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":121837388,\"sent_bytes\":661591,\"receive_would_block\":414,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":37,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":10666,\"in_place_decryptions\":82977,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":10782,\"network_reads\":83018,\"decrypted_packets\":83016,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":1812,\"tun_backpressure\":0,\"network_backpressure\":3,\"io\":{\"receive_calls\":2578,\"send_calls\":1498,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":121844927,\"sent_bytes\":667768,\"receive_would_block\":323,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":39,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":10782,\"in_place_decryptions\":83016,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":10257,\"network_reads\":82947,\"decrypted_packets\":82946,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":1821,\"tun_backpressure\":0,\"network_backpressure\":3,\"io\":{\"receive_calls\":2641,\"send_calls\":1540,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":121835073,\"sent_bytes\":637777,\"receive_would_block\":346,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":30,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":10257,\"in_place_decryptions\":82946,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":10733,\"network_reads\":82946,\"decrypted_packets\":82944,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":4399,\"tun_backpressure\":0,\"network_backpressure\":3,\"io\":{\"receive_calls\":9018,\"send_calls\":3282,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":121831325,\"sent_bytes\":699388,\"receive_would_block\":2700,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":38,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":10733,\"in_place_decryptions\":82944,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":11542,\"network_reads\":82944,\"decrypted_packets\":82942,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":1783,\"tun_backpressure\":0,\"network_backpressure\":3,\"io\":{\"receive_calls\":2664,\"send_calls\":1446,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":121838626,\"sent_bytes\":711928,\"receive_would_block\":362,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":63,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":11542,\"in_place_decryptions\":82942,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":11046,\"network_reads\":83013,\"decrypted_packets\":83012,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":1924,\"tun_backpressure\":0,\"tun_write_calls\":5318,\"tun_written_packets\":83012,\"tun_coalesced_packets\":82742,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":3,\"io\":{\"receive_calls\":2877,\"send_calls\":1523,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":121846315,\"sent_bytes\":689631,\"receive_would_block\":435,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":54,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":11046,\"in_place_decryptions\":83012,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":11988,\"network_reads\":91960,\"decrypted_packets\":91959,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":2959,\"tun_backpressure\":0,\"tun_write_calls\":6230,\"tun_written_packets\":91959,\"tun_coalesced_packets\":91481,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":3,\"io\":{\"receive_calls\":4660,\"send_calls\":2100,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":134899238,\"sent_bytes\":766094,\"receive_would_block\":1102,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":49,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":11988,\"in_place_decryptions\":91959,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":11753,\"network_reads\":82991,\"decrypted_packets\":82989,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":2251,\"tun_backpressure\":0,\"network_backpressure\":3,\"io\":{\"receive_calls\":3552,\"send_calls\":1656,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":121842532,\"sent_bytes\":728246,\"receive_would_block\":694,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":47,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":11753,\"in_place_decryptions\":82989,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}"
  ],
  "direct_after": {
    "requested_bytes": 100000000,
    "valid": true,
    "tls": "TLSv1.2",
    "status": 206,
    "content_range": "bytes 0-99999999/148648524",
    "bytes": 100000000,
    "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
    "seconds": 2.0901334999998653,
    "Mbps": 382.75067118920947
  },
  "cleanup": {
    "mode": true,
    "identity": true,
    "links": true,
    "rules": true,
    "default": true,
    "packages": true,
    "limits": true,
    "configs": true,
    "stage_absent": true,
    "firewall_owned_rules_absent": true
  },
  "elapsed_seconds": 232.70299999999997,
  "sources": {
    "main": {
      "sha": "f26fd3d741d125037797ec9953de1d50ebd93c8a",
      "archive_sha256": "e5fa86cc30e7a26383d5beddcd37429a68e5613ee4184cc32d5308b9912a7e5e",
      "effective_sources_sha256": "2dfc99c1301dbda490b095d8dea769a0816201dc01a9a1ed1478345f61d4868e"
    },
    "erase": {
      "sha": "6a754df933d7f77e5dd767c1a7f2380551f6f6f5",
      "archive_sha256": "9df12c721b5dd7d390be7400e396ebdb9c0d6890fd47be1476f8ef028c07dfeb",
      "effective_sources_sha256": "2061361c24eb47c91875b6e1f65179ed574f2387c84bdb5ef162f0512bf352c9"
    }
  }
}
```

## Independent postflight

```json
{
  "utc": "2026-11-10T02:50:07.337943+00:00",
  "checks": {
    "mode_unchanged": true,
    "identity_unchanged": true,
    "links_unchanged": true,
    "rules_unchanged": true,
    "default_unchanged": true,
    "packages_unchanged": true,
    "limits_unchanged": true,
    "configs_unchanged": true,
    "stage_absent": true,
    "interface_absent": true,
    "table_empty": true,
    "owned_processes_and_guard_absent": true
  },
  "all_passed": true
}
```

## Identical temporary Rust driver

Injected only into isolated test archives as `src/live_probe.rs`, with a temporary binary calling `protun_next::live_probe::run()`. It receives an existing private configuration path and an owned interface name from the external guarded test owner. It does not alter saved router configuration. The external owner adds/removes only scoped temporary routing and checks restoration. Both variants use the same driver; the candidate environment selects `NEXT_TUN_RECEIVE_MODE=vnet` and baseline `raw`.

```rust
//! Temporary router driver. No saved configuration or routing changes here.
//! The external owner creates/removes only its temporary interface/rules.
use crate::{config, connection::Connection, linux};
use std::{
    fs::File,
    io::{BufRead, Write},
    os::fd::{AsFd, AsRawFd},
    time::Duration,
};
fn cpu_ns() -> u64 {
    let mut t: libc::timespec = unsafe { std::mem::zeroed() };
    assert_eq!(
        unsafe { libc::clock_gettime(libc::CLOCK_PROCESS_CPUTIME_ID, &mut t) },
        0
    );
    t.tv_sec as u64 * 1_000_000_000 + t.tv_nsec as u64
}
pub fn run() {
    let args: Vec<_> = std::env::args().collect();
    assert_eq!(args.len(), 6);
    let mut cfg = config::load(std::path::Path::new(&args[1])).unwrap();
    cfg.endpoint.set_port(args[5].parse().unwrap());
    assert!(args[3].starts_with("pna") && args[3].len() < libc::IFNAMSIZ);
    let tun = File::options()
        .read(true)
        .write(true)
        .open("/dev/net/tun")
        .unwrap();
    let mut request = [0u8; 40];
    request[..args[3].len()].copy_from_slice(args[3].as_bytes());
    let mode=std::env::var("NEXT_TUN_RECEIVE_MODE").unwrap_or_else(|_|"raw".into());
    assert!(matches!(mode.as_str(),"raw"|"vnet"));
    let flags=libc::IFF_TUN|libc::IFF_NO_PI|if mode=="vnet"{libc::IFF_VNET_HDR}else{0};
    request[16..18].copy_from_slice(&(flags as i16).to_ne_bytes());
    assert_eq!(
        unsafe { libc::ioctl(tun.as_raw_fd(), libc::TUNSETIFF, request.as_mut_ptr()) },
        0
    );
    if mode=="vnet" {
        let size=10i32;let little=1i32;
        assert_eq!(unsafe{libc::ioctl(tun.as_raw_fd(),libc::TUNSETVNETHDRSZ,&size)},0);
        assert_eq!(unsafe{libc::ioctl(tun.as_raw_fd(),libc::TUNSETVNETLE,&little)},0);
        assert_eq!(unsafe{libc::ioctl(tun.as_raw_fd(),libc::TUNSETOFFLOAD,0)},0);
    }
    // Bring up only this owned, temporary interface before the worker reads it.
    // Reading a down TUN can return EIO and terminate an otherwise healthy worker.
    for address in &cfg.addresses {
        assert!(
            std::process::Command::new("ip")
                .args(["address", "add", address, "dev", &args[3]])
                .status()
                .unwrap()
                .success()
        );
    }
    assert!(
        std::process::Command::new("ip")
            .args([
                "link",
                "set",
                "dev",
                &args[3],
                "mtu",
                &cfg.mtu.to_string(),
                "up"
            ])
            .status()
            .unwrap()
            .success()
    );
    let transport = match args[2].as_str() {
        "udp" => linux::Transport::Udp,
        "tcp" => linux::Transport::Tcp,
        "stealth" => linux::Transport::Stealth(Default::default()),
        _ => panic!("protocol"),
    };
    let mut con = Connection::start(tun.as_fd(), cfg, args[4].clone(), transport).unwrap();
    println!(
        "LIVE_READY {{\"effective_sources_sha256\":\"{}\"}}",
        option_env!("NEXT_BUILD_ID").unwrap_or("unstamped")
    );
    std::io::stdout().flush().unwrap();
    println!("LIVE_MODE {}",mode);
    std::io::stdout().flush().unwrap();
    let mut start = None;
    for line in std::io::stdin().lock().lines() {
        match line.unwrap().as_str() {
            "start" => {
                let _=con.wait_for_stats(con.request_stats(),Duration::from_secs(2));
                start = Some((cpu_ns(), std::time::Instant::now()));
                println!("LIVE_START");
            }
            "stop" => {
                let end = cpu_ns();
                let (c, t) = start.take().expect("start required");
                let elapsed=t.elapsed().as_secs_f64();
                let snapshot=con.wait_for_stats(con.request_stats(),Duration::from_secs(2));
                println!(
                    "LIVE_METRICS {}",
                    serde_json::json!({"process_cpu_ns":end-c,"seconds":elapsed,"cpu_core_percent":(end-c) as f64/elapsed/1e7,"rss_kib":peak_rss(),"tun_mode":mode,"snapshot":snapshot})
                );
            }
            "state" => println!(
                "LIVE_STATE {}",
                serde_json::to_string(&con.snapshot()).unwrap()
            ),
            "quit" => break,
            _ => panic!("unknown command"),
        }
        std::io::stdout().flush().unwrap();
    }
    con.disconnect();
    let counters = con.disconnect_and_wait().unwrap();
    println!("LIVE_FINAL {}", serde_json::to_string(&counters).unwrap());
    // Keep no detached workers or TUN ownership after normal exit.
    drop(tun);
    std::thread::sleep(Duration::from_millis(1));
}

fn peak_rss()->u64 {
 std::fs::read_to_string("/proc/self/status").unwrap().lines()
 .find(|l|l.starts_with("VmHWM:")).unwrap().split_whitespace().nth(1).unwrap().parse().unwrap()
}

```
