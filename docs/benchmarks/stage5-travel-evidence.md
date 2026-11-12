# Stage 5: sanitized Travel evidence

See [method and interpretation](stage5-travel.md).

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
          "seconds": 0.22155779999957304
        },
        {
          "requested_bytes": 12000000,
          "valid": false,
          "error": "SSLEOFError: [SSL: UNEXPECTED_EOF_WHILE_READING] EOF occurred in violation of protocol (_ssl.c:1010)",
          "seconds": 0.2614725000003091
        },
        {
          "requested_bytes": 12000000,
          "valid": false,
          "error": "SSLEOFError: [SSL: UNEXPECTED_EOF_WHILE_READING] EOF occurred in violation of protocol (_ssl.c:1010)",
          "seconds": 0.22221090000039112
        }
      ],
      "warmup_forward_counts": {
        "in": 676,
        "out": 5460
      },
      "warmup_state": {
        "sequence": 3,
        "state": "TunnelEstablished",
        "elapsed": {
          "secs": 0,
          "nanos": 31630131
        },
        "mono_timestamp": {
          "secs": 110640,
          "nanos": 751709037
        },
        "stats_generation": 0,
        "statistics": {
          "handshake_age": {
            "secs": 0,
            "nanos": 0
          },
          "sent_bytes": 152,
          "received_bytes": 0,
          "estimated_loss": 0.0,
          "handshake_rtt": {
            "secs": 0,
            "nanos": 27000000
          }
        },
        "counters": {
          "tun_reads": 2,
          "network_reads": 1,
          "decrypted_packets": 0,
          "invalid_packets": 0,
          "outbound_filtered": 0,
          "inbound_filtered": 0,
          "pool_pressure": 0,
          "polls": 2,
          "tun_backpressure": 0,
          "tun_write_calls": 0,
          "tun_written_packets": 0,
          "tun_coalesced_packets": 0,
          "tun_max_write_batch": 0,
          "tun_offload_fallbacks": 0,
          "network_backpressure": 0,
          "io": {
            "receive_calls": 3,
            "send_calls": 3,
            "received_datagrams": 1,
            "sent_datagrams": 4,
            "max_receive_batch": 1,
            "max_send_batch": 2,
            "truncated_datagrams": 0,
            "gso_datagrams": 2,
            "gro_datagrams": 0,
            "kernel_fallbacks": 0,
            "enobufs": 0,
            "received_bytes": 92,
            "sent_bytes": 396,
            "receive_would_block": 2,
            "send_would_block": 0,
            "receive_interrupted": 0,
            "send_interrupted": 0
          },
          "to_network_high_water": 3,
          "to_tun_high_water": 0,
          "pool_reserved_bytes": 405758,
          "outer_eof": 0,
          "outer_io_errors": 0,
          "in_place_encryptions": 2,
          "in_place_decryptions": 0,
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
          "seconds": 0.2671536999996533
        },
        {
          "requested_bytes": 12000000,
          "valid": false,
          "error": "SSLEOFError: [SSL: UNEXPECTED_EOF_WHILE_READING] EOF occurred in violation of protocol (_ssl.c:1010)",
          "seconds": 0.25126170000021375
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.8609614000006331,
          "Mbps": 111.50325670805846
        }
      ],
      "system_before": "cpu  2557158 0 571069 18834135 96 44893 52744 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 3.0542700000005425,
        "Mbps": 261.92838223204166
      },
      "worker": {
        "cpu_core_percent": 63.86048989757228,
        "process_cpu_ns": 1951343939,
        "rss_kib": 8748,
        "seconds": 3.055635718,
        "snapshot": {
          "counters": {
            "decrypted_packets": 83031,
            "in_place_decryptions": 83031,
            "in_place_encryptions": 16080,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 81816,
              "gso_datagrams": 15447,
              "kernel_fallbacks": 0,
              "max_receive_batch": 16,
              "max_send_batch": 16,
              "receive_calls": 4702,
              "receive_interrupted": 0,
              "receive_would_block": 1484,
              "received_bytes": 119349452,
              "received_datagrams": 83032,
              "send_calls": 2683,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 1593429,
              "sent_datagrams": 16082,
              "truncated_datagrams": 0
            },
            "network_backpressure": 0,
            "network_reads": 83032,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 3135,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 58,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 82682,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 16080,
            "tun_write_calls": 5361,
            "tun_written_packets": 83031
          },
          "elapsed": {
            "nanos": 696627489,
            "secs": 4
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 17363984,
            "secs": 110648
          },
          "sequence": 9,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.09264659881591797,
            "handshake_age": {
              "nanos": 657429658,
              "secs": 4
            },
            "handshake_rtt": {
              "nanos": 38000000,
              "secs": 0
            },
            "received_bytes": 115539396,
            "sent_bytes": 1078689
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2557263 0 571212 18834432 96 44906 52788 0 0 0\n",
      "forwarded_bytes": {
        "in": 100371317,
        "out": 974922
      },
      "valid": true,
      "cpu_ns_per_byte": 19.51343939
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
          "seconds": 0.8156163999992714,
          "Mbps": 117.70239048661325
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.7564196999992419,
          "Mbps": 126.91366975251466
        }
      ],
      "system_before": "cpu  2557399 0 571363 18834831 96 44912 52805 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 4.249512999999752,
        "Mbps": 188.25686613973102
      },
      "worker": {
        "cpu_core_percent": 49.34587326788896,
        "process_cpu_ns": 2097737967,
        "rss_kib": 9456,
        "seconds": 4.251090979,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91988,
            "in_place_decryptions": 91988,
            "in_place_encryptions": 17528,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 90588,
              "gso_datagrams": 16348,
              "kernel_fallbacks": 0,
              "max_receive_batch": 16,
              "max_send_batch": 16,
              "receive_calls": 6825,
              "receive_interrupted": 0,
              "receive_would_block": 2547,
              "received_bytes": 132141692,
              "received_datagrams": 91989,
              "send_calls": 3533,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 1684202,
              "sent_datagrams": 17530,
              "truncated_datagrams": 0
            },
            "network_backpressure": 0,
            "network_reads": 91989,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 4728,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 61,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 91479,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 17528,
            "tun_write_calls": 7051,
            "tun_written_packets": 91988
          },
          "elapsed": {
            "nanos": 80259377,
            "secs": 6
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 952497672,
            "secs": 110655
          },
          "sequence": 10,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.14556938409805298,
            "handshake_age": {
              "nanos": 41567209,
              "secs": 6
            },
            "handshake_rtt": {
              "nanos": 38000000,
              "secs": 0
            },
            "received_bytes": 127928170,
            "sent_bytes": 1123126
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2557694 0 571555 18835114 96 44928 52857 0 0 0\n",
      "forwarded_bytes": {
        "in": 100410808,
        "out": 907502
      },
      "valid": true,
      "cpu_ns_per_byte": 20.97737967
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
          "seconds": 0.7707149999996545,
          "Mbps": 124.5596621319723
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.763336199999685,
          "Mbps": 125.76371983935731
        }
      ],
      "system_before": "cpu  2558081 0 571761 18835240 96 44936 52877 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 3.1231552999997803,
        "Mbps": 256.15120708216347
      },
      "worker": {
        "cpu_core_percent": 63.29677421540401,
        "process_cpu_ns": 1977778454,
        "rss_kib": 9340,
        "seconds": 3.124611765,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91888,
            "in_place_decryptions": 91888,
            "in_place_encryptions": 17554,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 90308,
              "gso_datagrams": 16838,
              "kernel_fallbacks": 0,
              "max_receive_batch": 16,
              "max_send_batch": 16,
              "receive_calls": 5320,
              "receive_interrupted": 0,
              "receive_would_block": 1776,
              "received_bytes": 132135500,
              "received_datagrams": 91889,
              "send_calls": 3048,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 1721562,
              "sent_datagrams": 17556,
              "truncated_datagrams": 0
            },
            "network_backpressure": 0,
            "network_reads": 91889,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 3493,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 49,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 91493,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 17554,
            "tun_write_calls": 6176,
            "tun_written_packets": 91888
          },
          "elapsed": {
            "nanos": 923444039,
            "secs": 4
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 946139221,
            "secs": 110662
          },
          "sequence": 9,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.09562617540359497,
            "handshake_age": {
              "nanos": 893564491,
              "secs": 4
            },
            "handshake_rtt": {
              "nanos": 29000000,
              "secs": 0
            },
            "received_bytes": 127921670,
            "sent_bytes": 1159654
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2558252 0 571944 18835454 96 44950 52923 0 0 0\n",
      "forwarded_bytes": {
        "in": 100375592,
        "out": 938374
      },
      "valid": true,
      "cpu_ns_per_byte": 19.77778454
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
          "seconds": 0.8001978000002055,
          "Mbps": 119.97033733406333
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.8106964999997217,
          "Mbps": 118.4166947803931
        }
      ],
      "system_before": "cpu  2558341 0 572101 18835901 96 44957 52940 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 3.6024514000000636,
        "Mbps": 222.07100420563228
      },
      "worker": {
        "cpu_core_percent": 54.374866549524036,
        "process_cpu_ns": 1959443991,
        "rss_kib": 9568,
        "seconds": 3.603584,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91957,
            "in_place_decryptions": 91957,
            "in_place_encryptions": 17573,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 90860,
              "gso_datagrams": 16760,
              "kernel_fallbacks": 0,
              "max_receive_batch": 16,
              "max_send_batch": 16,
              "receive_calls": 5584,
              "receive_interrupted": 0,
              "receive_would_block": 1882,
              "received_bytes": 132142012,
              "received_datagrams": 91958,
              "send_calls": 3054,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 1735230,
              "sent_datagrams": 17575,
              "truncated_datagrams": 0
            },
            "network_backpressure": 0,
            "network_reads": 91958,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 3687,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 59,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 91553,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 17573,
            "tun_write_calls": 6215,
            "tun_written_packets": 91957
          },
          "elapsed": {
            "nanos": 445014401,
            "secs": 5
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 244674292,
            "secs": 110670
          },
          "sequence": 9,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.1120070219039917,
            "handshake_age": {
              "nanos": 404779622,
              "secs": 5
            },
            "handshake_rtt": {
              "nanos": 39000000,
              "secs": 0
            },
            "received_bytes": 127925759,
            "sent_bytes": 1172714
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2558492 0 572258 18836253 96 44970 52985 0 0 0\n",
      "forwarded_bytes": {
        "in": 100373028,
        "out": 940118
      },
      "valid": true,
      "cpu_ns_per_byte": 19.59443991
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
          "seconds": 0.7330433999995876,
          "Mbps": 130.96086807418772
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.6465123000007225,
          "Mbps": 148.48905426840713
        }
      ],
      "system_before": "cpu  2558629 0 572406 18836633 96 44976 53000 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 3.0482984000000215,
        "Mbps": 262.44149850946167
      },
      "worker": {
        "cpu_core_percent": 64.75187326610384,
        "process_cpu_ns": 1974840580,
        "rss_kib": 8988,
        "seconds": 3.049858607,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91938,
            "in_place_decryptions": 91938,
            "in_place_encryptions": 13303,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 90740,
              "gso_datagrams": 11907,
              "kernel_fallbacks": 0,
              "max_receive_batch": 16,
              "max_send_batch": 16,
              "receive_calls": 7335,
              "receive_interrupted": 0,
              "receive_would_block": 2786,
              "received_bytes": 132138028,
              "received_datagrams": 91939,
              "send_calls": 3512,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 1182726,
              "sent_datagrams": 13305,
              "truncated_datagrams": 0
            },
            "network_backpressure": 0,
            "network_reads": 91939,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 4567,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 58,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 91437,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 13303,
            "tun_write_calls": 6151,
            "tun_written_packets": 91938
          },
          "elapsed": {
            "nanos": 687521715,
            "secs": 4
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 831576985,
            "secs": 110676
          },
          "sequence": 9,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.06423473358154297,
            "handshake_age": {
              "nanos": 658394273,
              "secs": 4
            },
            "handshake_rtt": {
              "nanos": 28000000,
              "secs": 0
            },
            "received_bytes": 127925474,
            "sent_bytes": 756850
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2558742 0 572550 18836924 96 44988 53041 0 0 0\n",
      "forwarded_bytes": {
        "in": 100378443,
        "out": 574994
      },
      "valid": true,
      "cpu_ns_per_byte": 19.7484058
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
          "seconds": 0.7859995999997409,
          "Mbps": 122.13746673666456
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.7339148999999452,
          "Mbps": 130.80535631584422
        }
      ],
      "system_before": "cpu  2558882 0 572690 18837330 96 44994 53057 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 2.997664400000758,
        "Mbps": 266.8744373118611
      },
      "worker": {
        "cpu_core_percent": 65.08694122631256,
        "process_cpu_ns": 1952337315,
        "rss_kib": 8996,
        "seconds": 2.999583754,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91982,
            "in_place_decryptions": 91982,
            "in_place_encryptions": 16193,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 90351,
              "gso_datagrams": 15363,
              "kernel_fallbacks": 0,
              "max_receive_batch": 16,
              "max_send_batch": 16,
              "receive_calls": 5785,
              "receive_interrupted": 0,
              "receive_would_block": 1914,
              "received_bytes": 132143100,
              "received_datagrams": 91983,
              "send_calls": 3052,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 1583742,
              "sent_datagrams": 16195,
              "truncated_datagrams": 0
            },
            "network_backpressure": 0,
            "network_reads": 91983,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 3761,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 53,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 91567,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 16193,
            "tun_write_calls": 6144,
            "tun_written_packets": 91982
          },
          "elapsed": {
            "nanos": 767594710,
            "secs": 4
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 521197296,
            "secs": 110683
          },
          "sequence": 8,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.1129220724105835,
            "handshake_age": {
              "nanos": 728146201,
              "secs": 4
            },
            "handshake_rtt": {
              "nanos": 38000000,
              "secs": 0
            },
            "received_bytes": 127928185,
            "sent_bytes": 1065386
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2558997 0 572832 18837609 96 45007 53101 0 0 0\n",
      "forwarded_bytes": {
        "in": 100372349,
        "out": 841194
      },
      "valid": true,
      "cpu_ns_per_byte": 19.52337315
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
          "seconds": 0.7866155000001527,
          "Mbps": 122.04183619567803
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.7805702000005112,
          "Mbps": 122.98701641433036
        }
      ],
      "system_before": "cpu  2559136 0 572978 18838021 96 45013 53118 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 5.10398350000014,
        "Mbps": 156.7403186158376
      },
      "worker": {
        "cpu_core_percent": 44.81584117746076,
        "process_cpu_ns": 2287820952,
        "rss_kib": 8860,
        "seconds": 5.104938102,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91945,
            "in_place_decryptions": 91945,
            "in_place_encryptions": 18266,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 90159,
              "gso_datagrams": 16645,
              "kernel_fallbacks": 0,
              "max_receive_batch": 16,
              "max_send_batch": 16,
              "receive_calls": 8578,
              "receive_interrupted": 0,
              "receive_would_block": 3509,
              "received_bytes": 132137068,
              "received_datagrams": 91946,
              "send_calls": 4276,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 1762970,
              "sent_datagrams": 18268,
              "truncated_datagrams": 0
            },
            "network_backpressure": 0,
            "network_reads": 91946,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 6115,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 63,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 91308,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 18266,
            "tun_write_calls": 8069,
            "tun_written_packets": 91945
          },
          "elapsed": {
            "nanos": 926940512,
            "secs": 6
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 316624526,
            "secs": 110692
          },
          "sequence": 11,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.16782820224761963,
            "handshake_age": {
              "nanos": 887057281,
              "secs": 6
            },
            "handshake_rtt": {
              "nanos": 39000000,
              "secs": 0
            },
            "received_bytes": 127925229,
            "sent_bytes": 1178278
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2559535 0 573237 18838294 96 45034 53179 0 0 0\n",
      "forwarded_bytes": {
        "in": 100446569,
        "out": 922570
      },
      "valid": true,
      "cpu_ns_per_byte": 22.87820952
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
          "seconds": 0.7098628000003373,
          "Mbps": 135.23740080471097
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.755929400000241,
          "Mbps": 126.99598666220601
        }
      ],
      "system_before": "cpu  2559849 0 573408 18838514 96 45040 53197 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 3.078179000000091,
        "Mbps": 259.89391780009424
      },
      "worker": {
        "cpu_core_percent": 63.10592366322211,
        "process_cpu_ns": 1943817088,
        "rss_kib": 2548,
        "seconds": 3.080245047,
        "snapshot": {
          "counters": {
            "decrypted_packets": 92006,
            "in_place_decryptions": 92006,
            "in_place_encryptions": 10479,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 11306,
              "receive_interrupted": 0,
              "receive_would_block": 1928,
              "received_bytes": 130855386,
              "received_datagrams": 0,
              "send_calls": 2726,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 627820,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 2,
            "network_reads": 92007,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 3922,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 49,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 91449,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 10479,
            "tun_write_calls": 6240,
            "tun_written_packets": 92006
          },
          "elapsed": {
            "nanos": 814509351,
            "secs": 4
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 176892013,
            "secs": 110699
          },
          "sequence": 9,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.0008795857429504395,
            "handshake_age": {
              "nanos": 742695839,
              "secs": 4
            },
            "handshake_rtt": {
              "nanos": 71000000,
              "secs": 0
            },
            "received_bytes": 127927277,
            "sent_bytes": 439014
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2559980 0 573578 18838781 96 45053 53243 0 0 0\n",
      "forwarded_bytes": {
        "in": 100377683,
        "out": 356386
      },
      "valid": true,
      "cpu_ns_per_byte": 19.43817088
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
          "seconds": 0.7191043000002537,
          "Mbps": 133.49941030802643
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.6026556000006167,
          "Mbps": 159.29496050464272
        }
      ],
      "system_before": "cpu  2560113 0 573721 18839138 96 45060 53260 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 2.8033841999995275,
        "Mbps": 285.36937605631607
      },
      "worker": {
        "cpu_core_percent": 65.40313754990406,
        "process_cpu_ns": 1834957679,
        "rss_kib": 2576,
        "seconds": 2.805611088,
        "snapshot": {
          "counters": {
            "decrypted_packets": 92005,
            "in_place_decryptions": 92005,
            "in_place_encryptions": 9653,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 9110,
              "receive_interrupted": 0,
              "receive_would_block": 645,
              "received_bytes": 130857016,
              "received_datagrams": 0,
              "send_calls": 1709,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 568932,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 2,
            "network_reads": 92006,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 2371,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 63,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 91678,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 9653,
            "tun_write_calls": 5447,
            "tun_written_packets": 92005
          },
          "elapsed": {
            "nanos": 384834650,
            "secs": 4
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 407527747,
            "secs": 110705
          },
          "sequence": 8,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 4.3451786041259766e-05,
            "handshake_age": {
              "nanos": 335374920,
              "secs": 4
            },
            "handshake_rtt": {
              "nanos": 46000000,
              "secs": 0
            },
            "received_bytes": 127926542,
            "sent_bytes": 394994
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2560246 0 573875 18839354 96 45074 53302 0 0 0\n",
      "forwarded_bytes": {
        "in": 100350457,
        "out": 311142
      },
      "valid": true,
      "cpu_ns_per_byte": 18.34957679
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
          "seconds": 0.8794274000001678,
          "Mbps": 109.1619387796897
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.7568608999999924,
          "Mbps": 126.83968745115644
        }
      ],
      "system_before": "cpu  2560565 0 574068 18839573 96 45082 53322 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 3.3098672000005536,
        "Mbps": 241.70154017051385
      },
      "worker": {
        "cpu_core_percent": 60.845331566259524,
        "process_cpu_ns": 2015124232,
        "rss_kib": 2384,
        "seconds": 3.3118797779999998,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91985,
            "in_place_decryptions": 91985,
            "in_place_encryptions": 9560,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 9554,
              "receive_interrupted": 0,
              "receive_would_block": 905,
              "received_bytes": 130853856,
              "received_datagrams": 0,
              "send_calls": 1995,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 560154,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 2,
            "network_reads": 91986,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 2657,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 29,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 91569,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 9560,
            "tun_write_calls": 5858,
            "tun_written_packets": 91985
          },
          "elapsed": {
            "nanos": 292243819,
            "secs": 5
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 684013326,
            "secs": 110712
          },
          "sequence": 9,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.0,
            "handshake_age": {
              "nanos": 213645491,
              "secs": 5
            },
            "handshake_rtt": {
              "nanos": 77000000,
              "secs": 0
            },
            "received_bytes": 127925075,
            "sent_bytes": 387890
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2560882 0 574312 18839604 96 45100 53373 0 0 0\n",
      "forwarded_bytes": {
        "in": 100365133,
        "out": 314182
      },
      "valid": true,
      "cpu_ns_per_byte": 20.15124232
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
          "seconds": 0.602274099999704,
          "Mbps": 159.39586311290353
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.5593816000000515,
          "Mbps": 171.61808683015522
        }
      ],
      "system_before": "cpu  2561080 0 574471 18839893 96 45107 53389 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 2.7388670999998794,
        "Mbps": 292.0915731909866
      },
      "worker": {
        "cpu_core_percent": 66.93424968053918,
        "process_cpu_ns": 1834483742,
        "rss_kib": 2576,
        "seconds": 2.740725041,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91986,
            "in_place_decryptions": 91986,
            "in_place_encryptions": 10444,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 9044,
              "receive_interrupted": 0,
              "receive_would_block": 591,
              "received_bytes": 130860290,
              "received_datagrams": 0,
              "send_calls": 1696,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 633514,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 2,
            "network_reads": 91987,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 2294,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 54,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 91386,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 10444,
            "tun_write_calls": 5502,
            "tun_written_packets": 91986
          },
          "elapsed": {
            "nanos": 172679839,
            "secs": 4
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 912398736,
            "secs": 110718
          },
          "sequence": 8,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.0016063451766967773,
            "handshake_age": {
              "nanos": 122558564,
              "secs": 4
            },
            "handshake_rtt": {
              "nanos": 49000000,
              "secs": 0
            },
            "received_bytes": 127929447,
            "sent_bytes": 445338
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2561200 0 574603 18840138 96 45120 53431 0 0 0\n",
      "forwarded_bytes": {
        "in": 100359046,
        "out": 362790
      },
      "valid": true,
      "cpu_ns_per_byte": 18.34483742
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
          "seconds": 0.6428576999996949,
          "Mbps": 149.33320391129416
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.5529255999999805,
          "Mbps": 173.62191224281057
        }
      ],
      "system_before": "cpu  2561342 0 574747 18840481 96 45126 53448 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 2.7468283999996856,
        "Mbps": 291.2449863996206
      },
      "worker": {
        "cpu_core_percent": 66.81523354285306,
        "process_cpu_ns": 1836447759,
        "rss_kib": 2576,
        "seconds": 2.748546494,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91951,
            "in_place_decryptions": 91951,
            "in_place_encryptions": 10399,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 9126,
              "receive_interrupted": 0,
              "receive_would_block": 660,
              "received_bytes": 130851564,
              "received_datagrams": 0,
              "send_calls": 1728,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 617912,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 2,
            "network_reads": 91952,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 2346,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 51,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 91368,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 10399,
            "tun_write_calls": 5426,
            "tun_written_packets": 91951
          },
          "elapsed": {
            "nanos": 195415099,
            "secs": 4
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 50345224,
            "secs": 110725
          },
          "sequence": 8,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.0007824301719665527,
            "handshake_age": {
              "nanos": 144578725,
              "secs": 4
            },
            "handshake_rtt": {
              "nanos": 50000000,
              "secs": 0
            },
            "received_bytes": 127923396,
            "sent_bytes": 430546
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2561480 0 574891 18840698 96 45139 53490 0 0 0\n",
      "forwarded_bytes": {
        "in": 100349547,
        "out": 334218
      },
      "valid": true,
      "cpu_ns_per_byte": 18.36447759
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
          "seconds": 0.5880470000001878,
          "Mbps": 163.25225704742877
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.5976122000001851,
          "Mbps": 160.63929083102767
        }
      ],
      "system_before": "cpu  2561600 0 575036 18841066 96 45145 53505 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 2.748833399999967,
        "Mbps": 291.0325522092425
      },
      "worker": {
        "cpu_core_percent": 66.87292689125226,
        "process_cpu_ns": 1838938860,
        "rss_kib": 2784,
        "seconds": 2.749900364,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91999,
            "in_place_decryptions": 91999,
            "in_place_encryptions": 10195,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 9161,
              "receive_interrupted": 0,
              "receive_would_block": 660,
              "received_bytes": 130857180,
              "received_datagrams": 0,
              "send_calls": 1739,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 604460,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 2,
            "network_reads": 92000,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 2392,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 63,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 91274,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 10195,
            "tun_write_calls": 5672,
            "tun_written_packets": 91999
          },
          "elapsed": {
            "nanos": 200745481,
            "secs": 4
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 120914132,
            "secs": 110731
          },
          "sequence": 8,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.0003477334976196289,
            "handshake_age": {
              "nanos": 147009165,
              "secs": 4
            },
            "handshake_rtt": {
              "nanos": 53000000,
              "secs": 0
            },
            "received_bytes": 127927329,
            "sent_bytes": 420766
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2561748 0 575199 18841261 96 45158 53546 0 0 0\n",
      "forwarded_bytes": {
        "in": 100361739,
        "out": 326234
      },
      "valid": true,
      "cpu_ns_per_byte": 18.3893886
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
          "seconds": 0.5414025000000038,
          "Mbps": 177.3172454874134
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.5487254999998186,
          "Mbps": 174.95086340990483
        }
      ],
      "system_before": "cpu  2561831 0 575341 18841633 96 45164 53561 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 2.7428915000000416,
        "Mbps": 291.6630132836052
      },
      "worker": {
        "cpu_core_percent": 66.91422458749892,
        "process_cpu_ns": 1836245610,
        "rss_kib": 2884,
        "seconds": 2.744178269,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91951,
            "in_place_decryptions": 91951,
            "in_place_encryptions": 10048,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 9061,
              "receive_interrupted": 0,
              "receive_would_block": 608,
              "received_bytes": 130851724,
              "received_datagrams": 0,
              "send_calls": 1694,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 594494,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 2,
            "network_reads": 91952,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 2305,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 60,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 91376,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 10048,
            "tun_write_calls": 5575,
            "tun_written_packets": 91951
          },
          "elapsed": {
            "nanos": 82150201,
            "secs": 4
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 75731148,
            "secs": 110737
          },
          "sequence": 8,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 4.3511390686035156e-05,
            "handshake_age": {
              "nanos": 17352947,
              "secs": 4
            },
            "handshake_rtt": {
              "nanos": 60000000,
              "secs": 0
            },
            "received_bytes": 127924237,
            "sent_bytes": 413446
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2561961 0 575519 18841823 96 45177 53603 0 0 0\n",
      "forwarded_bytes": {
        "in": 100357762,
        "out": 329058
      },
      "valid": true,
      "cpu_ns_per_byte": 18.3624561
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
          "seconds": 0.7298300999991625,
          "Mbps": 131.53746330839215
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.729295699999966,
          "Mbps": 131.6338489312421
        }
      ],
      "system_before": "cpu  2562047 0 575664 18842268 96 45184 53620 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 3.895016299999952,
        "Mbps": 205.390668069864
      },
      "worker": {
        "cpu_core_percent": 51.05792412923574,
        "process_cpu_ns": 1989101301,
        "rss_kib": 2492,
        "seconds": 3.895773937,
        "snapshot": {
          "counters": {
            "decrypted_packets": 92007,
            "in_place_decryptions": 92007,
            "in_place_encryptions": 9617,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 11405,
              "receive_interrupted": 0,
              "receive_would_block": 1998,
              "received_bytes": 130855100,
              "received_datagrams": 0,
              "send_calls": 2622,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 563460,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 2,
            "network_reads": 92008,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 3873,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 63,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 91411,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 9617,
            "tun_write_calls": 6578,
            "tun_written_packets": 92007
          },
          "elapsed": {
            "nanos": 595012044,
            "secs": 5
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 566870907,
            "secs": 110744
          },
          "sequence": 9,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.0,
            "handshake_age": {
              "nanos": 525786243,
              "secs": 5
            },
            "handshake_rtt": {
              "nanos": 67000000,
              "secs": 0
            },
            "received_bytes": 127926245,
            "sent_bytes": 390170
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2562371 0 575868 18842454 96 45199 53669 0 0 0\n",
      "forwarded_bytes": {
        "in": 100387768,
        "out": 305742
      },
      "valid": true,
      "cpu_ns_per_byte": 19.89101301
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
          "seconds": 5.555039199999555,
          "Mbps": 17.281606221610044
        }
      ],
      "system_before": "cpu  2562766 0 576056 18843399 96 45206 53682 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 3.9751955000001544,
        "Mbps": 201.24796377938367
      },
      "worker": {
        "cpu_core_percent": 69.49017067254354,
        "process_cpu_ns": 2763744216,
        "rss_kib": 3276,
        "seconds": 3.977172871,
        "snapshot": {
          "counters": {
            "decrypted_packets": 83012,
            "in_place_decryptions": 83012,
            "in_place_encryptions": 9911,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 6243,
              "receive_interrupted": 0,
              "receive_would_block": 1522,
              "received_bytes": 121843325,
              "received_datagrams": 0,
              "send_calls": 2317,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 634550,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 3,
            "network_reads": 83013,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 3218,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 55,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 82613,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 9911,
            "tun_write_calls": 6199,
            "tun_written_packets": 83012
          },
          "elapsed": {
            "nanos": 788385174,
            "secs": 9
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 358749784,
            "secs": 110756
          },
          "sequence": 12,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.00015658140182495117,
            "handshake_age": {
              "nanos": 754737776,
              "secs": 4
            },
            "handshake_rtt": {
              "nanos": 32000000,
              "secs": 5
            },
            "received_bytes": 115541296,
            "sent_bytes": 404588
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2562992 0 576249 18843698 96 45223 53736 0 0 0\n",
      "forwarded_bytes": {
        "in": 100406578,
        "out": 342906
      },
      "valid": true,
      "cpu_ns_per_byte": 27.63744216
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
          "seconds": 5.639884400000483,
          "Mbps": 17.021625478705165
        }
      ],
      "system_before": "cpu  2563128 0 576370 18844930 96 45230 53750 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 3.5285807000000204,
        "Mbps": 226.72005205945703
      },
      "worker": {
        "cpu_core_percent": 72.45568441438256,
        "process_cpu_ns": 2557837269,
        "rss_kib": 3232,
        "seconds": 3.530209244,
        "snapshot": {
          "counters": {
            "decrypted_packets": 82998,
            "in_place_decryptions": 82998,
            "in_place_encryptions": 8918,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 2894,
              "receive_interrupted": 0,
              "receive_would_block": 435,
              "received_bytes": 121836168,
              "received_datagrams": 0,
              "send_calls": 1510,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 558751,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 3,
            "network_reads": 82999,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 1991,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 47,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 82725,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 8918,
            "tun_write_calls": 5186,
            "tun_written_packets": 82998
          },
          "elapsed": {
            "nanos": 413836664,
            "secs": 9
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 621912160,
            "secs": 110767
          },
          "sequence": 12,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 9.638071060180664e-05,
            "handshake_age": {
              "nanos": 386484581,
              "secs": 4
            },
            "handshake_rtt": {
              "nanos": 26000000,
              "secs": 5
            },
            "received_bytes": 115537106,
            "sent_bytes": 364424
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2563363 0 576538 18845180 96 45246 53796 0 0 0\n",
      "forwarded_bytes": {
        "in": 100364140,
        "out": 310622
      },
      "valid": true,
      "cpu_ns_per_byte": 25.57837269
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
          "seconds": 0.9944902999995975,
          "Mbps": 96.53186159788471
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.8428692000006777,
          "Mbps": 113.89667578305486
        }
      ],
      "system_before": "cpu  2563461 0 576697 18845655 96 45253 53816 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 3.642006500000207,
        "Mbps": 219.65913569894906
      },
      "worker": {
        "cpu_core_percent": 73.06780169064156,
        "process_cpu_ns": 2662088115,
        "rss_kib": 3328,
        "seconds": 3.643312175,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91960,
            "in_place_decryptions": 91960,
            "in_place_encryptions": 9590,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 4617,
              "receive_interrupted": 0,
              "receive_would_block": 1092,
              "received_bytes": 134899779,
              "received_datagrams": 0,
              "send_calls": 2152,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 609496,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 4,
            "network_reads": 91961,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 2992,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 34,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 91500,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 9590,
            "tun_write_calls": 6304,
            "tun_written_packets": 91960
          },
          "elapsed": {
            "nanos": 742647196,
            "secs": 5
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 224577455,
            "secs": 110775
          },
          "sequence": 9,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.0,
            "handshake_age": {
              "nanos": 633474688,
              "secs": 5
            },
            "handshake_rtt": {
              "nanos": 104000000,
              "secs": 0
            },
            "received_bytes": 127922770,
            "sent_bytes": 389090
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2563685 0 576867 18845938 96 45268 53861 0 0 0\n",
      "forwarded_bytes": {
        "in": 100368878,
        "out": 308062
      },
      "valid": true,
      "cpu_ns_per_byte": 26.62088115
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
          "seconds": 5.592413299999862,
          "Mbps": 17.166113241308963
        }
      ],
      "system_before": "cpu  2564228 0 577069 18846693 96 45277 53874 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 3.6377824999999575,
        "Mbps": 219.9141922311214
      },
      "worker": {
        "cpu_core_percent": 72.81594230635451,
        "process_cpu_ns": 2650230238,
        "rss_kib": 3388,
        "seconds": 3.639629117,
        "snapshot": {
          "counters": {
            "decrypted_packets": 83023,
            "in_place_decryptions": 83023,
            "in_place_encryptions": 8556,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 2711,
              "receive_interrupted": 0,
              "receive_would_block": 358,
              "received_bytes": 121842070,
              "received_datagrams": 0,
              "send_calls": 1481,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 533244,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 3,
            "network_reads": 83024,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 1845,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 44,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 82737,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 8556,
            "tun_write_calls": 5153,
            "tun_written_packets": 83023
          },
          "elapsed": {
            "nanos": 510096694,
            "secs": 9
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 611249688,
            "secs": 110786
          },
          "sequence": 12,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.0,
            "handshake_age": {
              "nanos": 480308710,
              "secs": 4
            },
            "handshake_rtt": {
              "nanos": 28000000,
              "secs": 5
            },
            "received_bytes": 115540257,
            "sent_bytes": 346068
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2564490 0 577236 18846939 96 45292 53920 0 0 0\n",
      "forwarded_bytes": {
        "in": 100363832,
        "out": 307186
      },
      "valid": true,
      "cpu_ns_per_byte": 26.50230238
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
          "seconds": 0.797230799999852,
          "Mbps": 120.41682283225613
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 1.0247919000003094,
          "Mbps": 93.67755541390503
        }
      ],
      "system_before": "cpu  2564652 0 577393 18847355 96 45300 53940 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 3.6256380000004356,
        "Mbps": 220.65082062795676
      },
      "worker": {
        "cpu_core_percent": 71.44823484495937,
        "process_cpu_ns": 2591767067,
        "rss_kib": 3052,
        "seconds": 3.627475294,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91935,
            "in_place_decryptions": 91935,
            "in_place_encryptions": 9768,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 5185,
              "receive_interrupted": 0,
              "receive_would_block": 1308,
              "received_bytes": 134895829,
              "received_datagrams": 0,
              "send_calls": 2287,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 626688,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 3,
            "network_reads": 91936,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 3198,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 31,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 91416,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 9768,
            "tun_write_calls": 6486,
            "tun_written_packets": 91935
          },
          "elapsed": {
            "nanos": 712856279,
            "secs": 5
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 179019969,
            "secs": 110794
          },
          "sequence": 9,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.00014138221740722656,
            "handshake_age": {
              "nanos": 603571129,
              "secs": 5
            },
            "handshake_rtt": {
              "nanos": 106000000,
              "secs": 0
            },
            "received_bytes": 127923623,
            "sent_bytes": 400110
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2564829 0 577550 18847688 96 45315 53987 0 0 0\n",
      "forwarded_bytes": {
        "in": 100375252,
        "out": 311302
      },
      "valid": true,
      "cpu_ns_per_byte": 25.91767067
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
          "seconds": 5.692839100000128,
          "Mbps": 16.863290585535402
        }
      ],
      "system_before": "cpu  2565020 0 577696 18848886 96 45322 53999 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 3.7159662000003664,
        "Mbps": 215.2872111699835
      },
      "worker": {
        "cpu_core_percent": 70.96115731765862,
        "process_cpu_ns": 2637922899,
        "rss_kib": 3980,
        "seconds": 3.71741809,
        "snapshot": {
          "counters": {
            "decrypted_packets": 83058,
            "in_place_decryptions": 83058,
            "in_place_encryptions": 8890,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 2833,
              "receive_interrupted": 0,
              "receive_would_block": 445,
              "received_bytes": 121844463,
              "received_datagrams": 0,
              "send_calls": 1514,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 553255,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 3,
            "network_reads": 83059,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 2014,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 48,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 82767,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 8890,
            "tun_write_calls": 5312,
            "tun_written_packets": 83058
          },
          "elapsed": {
            "nanos": 677102514,
            "secs": 9
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 778633424,
            "secs": 110805
          },
          "sequence": 12,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.0,
            "handshake_age": {
              "nanos": 638465597,
              "secs": 4
            },
            "handshake_rtt": {
              "nanos": 37000000,
              "secs": 5
            },
            "received_bytes": 115542469,
            "sent_bytes": 359344
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2565218 0 577826 18849235 96 45338 54045 0 0 0\n",
      "forwarded_bytes": {
        "in": 100370579,
        "out": 317582
      },
      "valid": true,
      "cpu_ns_per_byte": 26.37922899
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
          "seconds": 0.7950584000000163,
          "Mbps": 120.74584709751892
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.8000130000000354,
          "Mbps": 119.99805003168167
        }
      ],
      "system_before": "cpu  2565385 0 577973 18849629 96 45345 54062 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 4.730999800000063,
        "Mbps": 169.09744954966797
      },
      "worker": {
        "cpu_core_percent": 59.165397359749534,
        "process_cpu_ns": 2801473057,
        "rss_kib": 4252,
        "seconds": 4.73498562,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91988,
            "in_place_decryptions": 91988,
            "in_place_encryptions": 10032,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 3699,
              "receive_interrupted": 0,
              "receive_would_block": 725,
              "received_bytes": 134904716,
              "received_datagrams": 0,
              "send_calls": 1858,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 659905,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 3,
            "network_reads": 91989,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 2531,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 63,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 91588,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 10032,
            "tun_write_calls": 6316,
            "tun_written_packets": 91988
          },
          "elapsed": {
            "nanos": 594697641,
            "secs": 6
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 266752803,
            "secs": 110814
          },
          "sequence": 10,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.003509819507598877,
            "handshake_age": {
              "nanos": 482883870,
              "secs": 6
            },
            "handshake_rtt": {
              "nanos": 110000000,
              "secs": 0
            },
            "received_bytes": 127925659,
            "sent_bytes": 438014
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2565788 0 578213 18849858 96 45364 54126 0 0 0\n",
      "forwarded_bytes": {
        "in": 100383557,
        "out": 352586
      },
      "valid": true,
      "cpu_ns_per_byte": 28.01473057
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
          "seconds": 1.0532157000006919,
          "Mbps": 91.14941981964087
        }
      ],
      "system_before": "cpu  2566113 0 578384 18849981 96 45370 54139 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 3.6745578000000023,
        "Mbps": 217.71327151256116
      },
      "worker": {
        "cpu_core_percent": 70.75392611646174,
        "process_cpu_ns": 2600595596,
        "rss_kib": 3100,
        "seconds": 3.675549526,
        "snapshot": {
          "counters": {
            "decrypted_packets": 83014,
            "in_place_decryptions": 83014,
            "in_place_encryptions": 8732,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 3698,
              "receive_interrupted": 0,
              "receive_would_block": 749,
              "received_bytes": 121839128,
              "received_datagrams": 0,
              "send_calls": 1746,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 549500,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 3,
            "network_reads": 83015,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 2284,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 34,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 82640,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 8732,
            "tun_write_calls": 5606,
            "tun_written_packets": 83014
          },
          "elapsed": {
            "nanos": 6034081,
            "secs": 5
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 266816263,
            "secs": 110821
          },
          "sequence": 9,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.0,
            "handshake_age": {
              "nanos": 902713699,
              "secs": 4
            },
            "handshake_rtt": {
              "nanos": 102000000,
              "secs": 0
            },
            "received_bytes": 115538402,
            "sent_bytes": 353480
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2566343 0 578556 18850257 96 45387 54185 0 0 0\n",
      "forwarded_bytes": {
        "in": 100375372,
        "out": 321194
      },
      "valid": true,
      "cpu_ns_per_byte": 26.00595596
    }
  ],
  "started_utc": "2026-11-12T05:10:19.384425+00:00",
  "kernel": "6.12.112",
  "server": "US-NY#544",
  "mtu": 1280,
  "method": "laptop HTTPS; socket-bound interface; owned router policy/NAT only",
  "binaries": {
    "main": "dda353af661bb466083ee9611791fd85cc98d21989adf305f8a14e4f78d76848",
    "erase": "e2bf3390b1726709d82c0bc27c7477ae3c1b45a5b43718b7545dda0d9e4ac870"
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
    "seconds": 1.6838078999999198,
    "Mbps": 475.11358035559647
  },
  "shutdowns": [
    "{\"tun_reads\":24,\"network_reads\":22,\"decrypted_packets\":21,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":31,\"tun_backpressure\":0,\"tun_write_calls\":21,\"tun_written_packets\":21,\"tun_coalesced_packets\":0,\"tun_max_write_batch\":1,\"tun_offload_fallbacks\":0,\"network_backpressure\":0,\"io\":{\"receive_calls\":27,\"send_calls\":19,\"received_datagrams\":22,\"sent_datagrams\":26,\"max_receive_batch\":3,\"max_send_batch\":2,\"truncated_datagrams\":0,\"gso_datagrams\":8,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":5612,\"sent_bytes\":6636,\"receive_would_block\":14,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":3,\"to_tun_high_water\":3,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":24,\"in_place_decryptions\":21,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":16080,\"network_reads\":83032,\"decrypted_packets\":83031,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":3136,\"tun_backpressure\":0,\"tun_write_calls\":5361,\"tun_written_packets\":83031,\"tun_coalesced_packets\":82682,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":0,\"io\":{\"receive_calls\":4702,\"send_calls\":2683,\"received_datagrams\":83032,\"sent_datagrams\":16082,\"max_receive_batch\":16,\"max_send_batch\":16,\"truncated_datagrams\":0,\"gso_datagrams\":15447,\"gro_datagrams\":81816,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":119349452,\"sent_bytes\":1593429,\"receive_would_block\":1484,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":58,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":16080,\"in_place_decryptions\":83031,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":17528,\"network_reads\":91989,\"decrypted_packets\":91988,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":4729,\"tun_backpressure\":0,\"tun_write_calls\":7051,\"tun_written_packets\":91988,\"tun_coalesced_packets\":91479,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":0,\"io\":{\"receive_calls\":6825,\"send_calls\":3533,\"received_datagrams\":91989,\"sent_datagrams\":17530,\"max_receive_batch\":16,\"max_send_batch\":16,\"truncated_datagrams\":0,\"gso_datagrams\":16348,\"gro_datagrams\":90588,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":132141692,\"sent_bytes\":1684202,\"receive_would_block\":2547,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":61,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":17528,\"in_place_decryptions\":91988,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":17554,\"network_reads\":91889,\"decrypted_packets\":91888,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":3494,\"tun_backpressure\":0,\"tun_write_calls\":6176,\"tun_written_packets\":91888,\"tun_coalesced_packets\":91493,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":0,\"io\":{\"receive_calls\":5320,\"send_calls\":3048,\"received_datagrams\":91889,\"sent_datagrams\":17556,\"max_receive_batch\":16,\"max_send_batch\":16,\"truncated_datagrams\":0,\"gso_datagrams\":16838,\"gro_datagrams\":90308,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":132135500,\"sent_bytes\":1721562,\"receive_would_block\":1776,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":49,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":17554,\"in_place_decryptions\":91888,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":17573,\"network_reads\":91958,\"decrypted_packets\":91957,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":3688,\"tun_backpressure\":0,\"tun_write_calls\":6215,\"tun_written_packets\":91957,\"tun_coalesced_packets\":91553,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":0,\"io\":{\"receive_calls\":5584,\"send_calls\":3054,\"received_datagrams\":91958,\"sent_datagrams\":17575,\"max_receive_batch\":16,\"max_send_batch\":16,\"truncated_datagrams\":0,\"gso_datagrams\":16760,\"gro_datagrams\":90860,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":132142012,\"sent_bytes\":1735230,\"receive_would_block\":1882,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":59,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":17573,\"in_place_decryptions\":91957,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":13303,\"network_reads\":91940,\"decrypted_packets\":91939,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":4569,\"tun_backpressure\":0,\"tun_write_calls\":6152,\"tun_written_packets\":91939,\"tun_coalesced_packets\":91437,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":0,\"io\":{\"receive_calls\":7337,\"send_calls\":3512,\"received_datagrams\":91940,\"sent_datagrams\":13305,\"max_receive_batch\":16,\"max_send_batch\":16,\"truncated_datagrams\":0,\"gso_datagrams\":11907,\"gro_datagrams\":90740,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":132139292,\"sent_bytes\":1182726,\"receive_would_block\":2787,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":58,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":13303,\"in_place_decryptions\":91939,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":16193,\"network_reads\":91983,\"decrypted_packets\":91982,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":3762,\"tun_backpressure\":0,\"tun_write_calls\":6144,\"tun_written_packets\":91982,\"tun_coalesced_packets\":91567,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":0,\"io\":{\"receive_calls\":5785,\"send_calls\":3052,\"received_datagrams\":91983,\"sent_datagrams\":16195,\"max_receive_batch\":16,\"max_send_batch\":16,\"truncated_datagrams\":0,\"gso_datagrams\":15363,\"gro_datagrams\":90351,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":132143100,\"sent_bytes\":1583742,\"receive_would_block\":1914,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":53,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":16193,\"in_place_decryptions\":91982,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":18266,\"network_reads\":91947,\"decrypted_packets\":91946,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":6117,\"tun_backpressure\":0,\"tun_write_calls\":8070,\"tun_written_packets\":91946,\"tun_coalesced_packets\":91308,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":0,\"io\":{\"receive_calls\":8580,\"send_calls\":4276,\"received_datagrams\":91947,\"sent_datagrams\":18268,\"max_receive_batch\":16,\"max_send_batch\":16,\"truncated_datagrams\":0,\"gso_datagrams\":16645,\"gro_datagrams\":90159,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":132137532,\"sent_bytes\":1762970,\"receive_would_block\":3510,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":63,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":18266,\"in_place_decryptions\":91946,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":10479,\"network_reads\":92007,\"decrypted_packets\":92006,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":3923,\"tun_backpressure\":0,\"tun_write_calls\":6240,\"tun_written_packets\":92006,\"tun_coalesced_packets\":91449,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":2,\"io\":{\"receive_calls\":11306,\"send_calls\":2726,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":130855386,\"sent_bytes\":627820,\"receive_would_block\":1928,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":49,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":10479,\"in_place_decryptions\":92006,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":9653,\"network_reads\":92006,\"decrypted_packets\":92005,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":2372,\"tun_backpressure\":0,\"tun_write_calls\":5447,\"tun_written_packets\":92005,\"tun_coalesced_packets\":91678,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":2,\"io\":{\"receive_calls\":9110,\"send_calls\":1709,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":130857016,\"sent_bytes\":568932,\"receive_would_block\":645,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":63,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":9653,\"in_place_decryptions\":92005,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":9560,\"network_reads\":91986,\"decrypted_packets\":91985,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":2658,\"tun_backpressure\":0,\"tun_write_calls\":5858,\"tun_written_packets\":91985,\"tun_coalesced_packets\":91569,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":2,\"io\":{\"receive_calls\":9554,\"send_calls\":1995,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":130853856,\"sent_bytes\":560154,\"receive_would_block\":905,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":29,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":9560,\"in_place_decryptions\":91985,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":10444,\"network_reads\":91987,\"decrypted_packets\":91986,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":2295,\"tun_backpressure\":0,\"tun_write_calls\":5502,\"tun_written_packets\":91986,\"tun_coalesced_packets\":91386,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":2,\"io\":{\"receive_calls\":9044,\"send_calls\":1696,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":130860290,\"sent_bytes\":633514,\"receive_would_block\":591,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":54,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":10444,\"in_place_decryptions\":91986,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":10399,\"network_reads\":91952,\"decrypted_packets\":91951,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":2347,\"tun_backpressure\":0,\"tun_write_calls\":5426,\"tun_written_packets\":91951,\"tun_coalesced_packets\":91368,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":2,\"io\":{\"receive_calls\":9126,\"send_calls\":1728,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":130851564,\"sent_bytes\":617912,\"receive_would_block\":660,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":51,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":10399,\"in_place_decryptions\":91951,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":10195,\"network_reads\":92000,\"decrypted_packets\":91999,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":2393,\"tun_backpressure\":0,\"tun_write_calls\":5672,\"tun_written_packets\":91999,\"tun_coalesced_packets\":91274,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":2,\"io\":{\"receive_calls\":9161,\"send_calls\":1739,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":130857180,\"sent_bytes\":604460,\"receive_would_block\":660,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":63,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":10195,\"in_place_decryptions\":91999,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":10048,\"network_reads\":91952,\"decrypted_packets\":91951,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":2306,\"tun_backpressure\":0,\"tun_write_calls\":5575,\"tun_written_packets\":91951,\"tun_coalesced_packets\":91376,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":2,\"io\":{\"receive_calls\":9061,\"send_calls\":1694,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":130851724,\"sent_bytes\":594494,\"receive_would_block\":608,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":60,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":10048,\"in_place_decryptions\":91951,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":9617,\"network_reads\":92008,\"decrypted_packets\":92007,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":3874,\"tun_backpressure\":0,\"tun_write_calls\":6578,\"tun_written_packets\":92007,\"tun_coalesced_packets\":91411,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":2,\"io\":{\"receive_calls\":11405,\"send_calls\":2622,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":130855100,\"sent_bytes\":563460,\"receive_would_block\":1998,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":63,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":9617,\"in_place_decryptions\":92007,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":9911,\"network_reads\":83013,\"decrypted_packets\":83012,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":3219,\"tun_backpressure\":0,\"tun_write_calls\":6199,\"tun_written_packets\":83012,\"tun_coalesced_packets\":82613,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":3,\"io\":{\"receive_calls\":6243,\"send_calls\":2317,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":121843325,\"sent_bytes\":634550,\"receive_would_block\":1522,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":55,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":9911,\"in_place_decryptions\":83012,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":8918,\"network_reads\":82999,\"decrypted_packets\":82998,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":1992,\"tun_backpressure\":0,\"tun_write_calls\":5186,\"tun_written_packets\":82998,\"tun_coalesced_packets\":82725,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":3,\"io\":{\"receive_calls\":2894,\"send_calls\":1510,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":121836168,\"sent_bytes\":558751,\"receive_would_block\":435,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":47,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":8918,\"in_place_decryptions\":82998,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":9590,\"network_reads\":91961,\"decrypted_packets\":91960,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":2993,\"tun_backpressure\":0,\"tun_write_calls\":6304,\"tun_written_packets\":91960,\"tun_coalesced_packets\":91500,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":4,\"io\":{\"receive_calls\":4617,\"send_calls\":2152,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":134899779,\"sent_bytes\":609496,\"receive_would_block\":1092,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":34,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":9590,\"in_place_decryptions\":91960,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":8556,\"network_reads\":83024,\"decrypted_packets\":83023,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":1846,\"tun_backpressure\":0,\"tun_write_calls\":5153,\"tun_written_packets\":83023,\"tun_coalesced_packets\":82737,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":3,\"io\":{\"receive_calls\":2711,\"send_calls\":1481,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":121842070,\"sent_bytes\":533244,\"receive_would_block\":358,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":44,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":8556,\"in_place_decryptions\":83023,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":9768,\"network_reads\":91936,\"decrypted_packets\":91935,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":3199,\"tun_backpressure\":0,\"tun_write_calls\":6486,\"tun_written_packets\":91935,\"tun_coalesced_packets\":91416,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":3,\"io\":{\"receive_calls\":5185,\"send_calls\":2287,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":134895829,\"sent_bytes\":626688,\"receive_would_block\":1308,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":31,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":9768,\"in_place_decryptions\":91935,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":8890,\"network_reads\":83059,\"decrypted_packets\":83058,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":2015,\"tun_backpressure\":0,\"tun_write_calls\":5312,\"tun_written_packets\":83058,\"tun_coalesced_packets\":82767,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":3,\"io\":{\"receive_calls\":2833,\"send_calls\":1514,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":121844463,\"sent_bytes\":553255,\"receive_would_block\":445,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":48,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":8890,\"in_place_decryptions\":83058,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":10032,\"network_reads\":91989,\"decrypted_packets\":91988,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":2532,\"tun_backpressure\":0,\"tun_write_calls\":6316,\"tun_written_packets\":91988,\"tun_coalesced_packets\":91588,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":3,\"io\":{\"receive_calls\":3699,\"send_calls\":1858,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":134904716,\"sent_bytes\":659905,\"receive_would_block\":725,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":63,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":10032,\"in_place_decryptions\":91988,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":8732,\"network_reads\":83015,\"decrypted_packets\":83014,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":2285,\"tun_backpressure\":0,\"tun_write_calls\":5606,\"tun_written_packets\":83014,\"tun_coalesced_packets\":82640,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":3,\"io\":{\"receive_calls\":3698,\"send_calls\":1746,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":121839128,\"sent_bytes\":549500,\"receive_would_block\":749,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":34,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":8732,\"in_place_decryptions\":83014,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}"
  ],
  "direct_after": {
    "requested_bytes": 100000000,
    "valid": true,
    "tls": "TLSv1.2",
    "status": 206,
    "content_range": "bytes 0-99999999/148648524",
    "bytes": 100000000,
    "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
    "seconds": 1.6273989000001166,
    "Mbps": 491.58199627635406
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
  "elapsed_seconds": 189.82799999999952,
  "sources": {
    "main": {
      "sha": "4c7cc53a0932fa31579729b603d80a9b9e9ecabf",
      "archive_sha256": "e03987e7d757a65e2e07c48d0350f23fed633ab469bf107fb6941df66bea8810",
      "effective_sources_sha256": "a6b57c90a80e3ef1c883e406ccb2c5bfecaaabb13b9ff9c95abb98a055a3e2e6",
      "runtime_sources_sha256": "5d448b4aabf1041e3a13920294314097f88a30eeb9a2d4c799724c2c04112fc7"
    },
    "erase": {
      "sha": "905517cc70b2b9b05a3c33cac0c06b71924bfc65",
      "archive_sha256": "541649b72b3502b0a316019f9099504b416c972b99bc461a34f051eaaa877dae",
      "effective_sources_sha256": "f2fbe616489c80995ed09773a0e4e42685ec74074ebeb866132a0070eb8c651f",
      "runtime_sources_sha256": "6d1b450fb9abe512604fff33c89583cae6afe82ea5162ce755b27712726f7341",
      "tls_record_prototype_patch_sha256": "3cf2ac9ad7eef47777bd72a4846ddb86d9d2f4660594309d2cdeb9a99e69c5e8"
    }
  },
  "binary_sizes_bytes": {
    "main": 2457672,
    "erase": 4468352
  }
}
```

## Independent postflight

```json
{
  "utc": "2026-11-12T05:14:08.976531+00:00",
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

## Identical temporary driver

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
