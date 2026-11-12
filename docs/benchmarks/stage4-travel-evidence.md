# Stage 4: sanitized Travel evidence

See [method and interpretation](stage4-travel.md). Historical `main` is retained serial; `erase` is rejected prototype.

## Metrics and identities

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
          "seconds": 0.24793699999997898
        },
        {
          "requested_bytes": 12000000,
          "valid": false,
          "error": "SSLEOFError: [SSL: UNEXPECTED_EOF_WHILE_READING] EOF occurred in violation of protocol (_ssl.c:1010)",
          "seconds": 0.28764779999994516
        },
        {
          "requested_bytes": 12000000,
          "valid": false,
          "error": "SSLEOFError: [SSL: UNEXPECTED_EOF_WHILE_READING] EOF occurred in violation of protocol (_ssl.c:1010)",
          "seconds": 0.24361599999997452
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
          "nanos": 42731981
        },
        "mono_timestamp": {
          "secs": 105683,
          "nanos": 875057168
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
            "nanos": 40000000
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
          "seconds": 0.26973400000008496
        },
        {
          "requested_bytes": 12000000,
          "valid": false,
          "error": "SSLEOFError: [SSL: UNEXPECTED_EOF_WHILE_READING] EOF occurred in violation of protocol (_ssl.c:1010)",
          "seconds": 0.2628841000000648
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.8169745999999805,
          "Mbps": 117.50671318300752
        }
      ],
      "system_before": "cpu  2436551 0 538670 18005692 96 42447 48942 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 3.4305519000000686,
        "Mbps": 233.19862906023488
      },
      "worker": {
        "cpu_core_percent": 58.05379301742264,
        "process_cpu_ns": 1992117685,
        "rss_kib": 9724,
        "seconds": 3.431503062,
        "snapshot": {
          "counters": {
            "decrypted_packets": 83000,
            "in_place_decryptions": 83000,
            "in_place_encryptions": 16909,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 81804,
              "gso_datagrams": 16209,
              "kernel_fallbacks": 0,
              "max_receive_batch": 16,
              "max_send_batch": 16,
              "receive_calls": 5344,
              "receive_interrupted": 0,
              "receive_would_block": 1845,
              "received_bytes": 119348252,
              "received_datagrams": 83001,
              "send_calls": 2928,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 1600480,
              "sent_datagrams": 16911,
              "truncated_datagrams": 0
            },
            "network_backpressure": 0,
            "network_reads": 83001,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 3499,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 59,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 82650,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 16909,
            "tun_write_calls": 5543,
            "tun_written_packets": 83000
          },
          "elapsed": {
            "nanos": 14417592,
            "secs": 5
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 492634311,
            "secs": 105691
          },
          "sequence": 9,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.0717233419418335,
            "handshake_age": {
              "nanos": 979423039,
              "secs": 4
            },
            "handshake_rtt": {
              "nanos": 34000000,
              "secs": 0
            },
            "received_bytes": 115539657,
            "sent_bytes": 1059212
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2436703 0 538851 18005981 96 42460 48989 0 0 0\n",
      "forwarded_bytes": {
        "in": 100375970,
        "out": 943598
      },
      "valid": true,
      "cpu_ns_per_byte": 19.92117685
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
          "seconds": 0.9761677000000191,
          "Mbps": 98.343757942409
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.8291682000000264,
          "Mbps": 115.7786803690698
        }
      ],
      "system_before": "cpu  2436844 0 539011 18006410 96 42468 49009 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 3.1212438999999677,
        "Mbps": 256.3080699973521
      },
      "worker": {
        "cpu_core_percent": 63.06995807541577,
        "process_cpu_ns": 1969278893,
        "rss_kib": 9140,
        "seconds": 3.122372288,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91917,
            "in_place_decryptions": 91917,
            "in_place_encryptions": 19012,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 89778,
              "gso_datagrams": 18214,
              "kernel_fallbacks": 0,
              "max_receive_batch": 16,
              "max_send_batch": 16,
              "receive_calls": 5926,
              "receive_interrupted": 0,
              "receive_would_block": 2001,
              "received_bytes": 132139900,
              "received_datagrams": 91918,
              "send_calls": 3226,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 1841918,
              "sent_datagrams": 19014,
              "truncated_datagrams": 0
            },
            "network_backpressure": 0,
            "network_reads": 91918,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 3837,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 63,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 91541,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 19012,
            "tun_write_calls": 6211,
            "tun_written_packets": 91917
          },
          "elapsed": {
            "nanos": 171489827,
            "secs": 5
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 493335935,
            "secs": 105698
          },
          "sequence": 9,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.09756124019622803,
            "handshake_age": {
              "nanos": 131928696,
              "secs": 5
            },
            "handshake_rtt": {
              "nanos": 39000000,
              "secs": 0
            },
            "received_bytes": 127927531,
            "sent_bytes": 1233354
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2436947 0 539164 18006705 96 42482 49056 0 0 0\n",
      "forwarded_bytes": {
        "in": 100376286,
        "out": 1008338
      },
      "valid": true,
      "cpu_ns_per_byte": 19.69278893
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
          "seconds": 0.6535221999999976,
          "Mbps": 146.8963104849389
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.6044067999999925,
          "Mbps": 158.83342146382404
        }
      ],
      "system_before": "cpu  2437083 0 539316 18007050 96 42488 49072 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 3.1337775000000647,
        "Mbps": 255.28296121852412
      },
      "worker": {
        "cpu_core_percent": 62.61230645402396,
        "process_cpu_ns": 1962847048,
        "rss_kib": 9488,
        "seconds": 3.134922125,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91907,
            "in_place_decryptions": 91907,
            "in_place_encryptions": 18902,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 90768,
              "gso_datagrams": 18268,
              "kernel_fallbacks": 0,
              "max_receive_batch": 16,
              "max_send_batch": 16,
              "receive_calls": 5064,
              "receive_interrupted": 0,
              "receive_would_block": 1611,
              "received_bytes": 132139132,
              "received_datagrams": 91908,
              "send_calls": 3016,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 1806366,
              "sent_datagrams": 18904,
              "truncated_datagrams": 0
            },
            "network_backpressure": 0,
            "network_reads": 91908,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 3390,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 61,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 91525,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 18902,
            "tun_write_calls": 5954,
            "tun_written_packets": 91907
          },
          "elapsed": {
            "nanos": 626309365,
            "secs": 4
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 996036244,
            "secs": 105704
          },
          "sequence": 9,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.09500271081924438,
            "handshake_age": {
              "nanos": 599451186,
              "secs": 4
            },
            "handshake_rtt": {
              "nanos": 26000000,
              "secs": 0
            },
            "received_bytes": 127925759,
            "sent_bytes": 1201322
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2437185 0 539488 18007351 96 42500 49118 0 0 0\n",
      "forwarded_bytes": {
        "in": 100370181,
        "out": 966346
      },
      "valid": true,
      "cpu_ns_per_byte": 19.62847048
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
          "seconds": 1.140684800000031,
          "Mbps": 84.15997127339419
        }
      ],
      "system_before": "cpu  2437609 0 539674 18007498 96 42506 49130 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 3.5365871000000197,
        "Mbps": 226.20678563239557
      },
      "worker": {
        "cpu_core_percent": 59.86638756400294,
        "process_cpu_ns": 2117755572,
        "rss_kib": 9776,
        "seconds": 3.53747012,
        "snapshot": {
          "counters": {
            "decrypted_packets": 82968,
            "in_place_decryptions": 82968,
            "in_place_encryptions": 17589,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 81742,
              "gso_datagrams": 17016,
              "kernel_fallbacks": 0,
              "max_receive_batch": 16,
              "max_send_batch": 16,
              "receive_calls": 4810,
              "receive_interrupted": 0,
              "receive_would_block": 1647,
              "received_bytes": 119344476,
              "received_datagrams": 82969,
              "send_calls": 2734,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 1740872,
              "sent_datagrams": 17591,
              "truncated_datagrams": 0
            },
            "network_backpressure": 0,
            "network_reads": 82969,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 3024,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 60,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 82584,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 17589,
            "tun_write_calls": 5679,
            "tun_written_packets": 82968
          },
          "elapsed": {
            "nanos": 929494078,
            "secs": 4
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 508648057,
            "secs": 105712
          },
          "sequence": 9,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.11682617664337158,
            "handshake_age": {
              "nanos": 892906675,
              "secs": 4
            },
            "handshake_rtt": {
              "nanos": 36000000,
              "secs": 0
            },
            "received_bytes": 115538027,
            "sent_bytes": 1177844
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2437879 0 539924 18007610 96 42523 49185 0 0 0\n",
      "forwarded_bytes": {
        "in": 100381785,
        "out": 1058466
      },
      "valid": true,
      "cpu_ns_per_byte": 21.17755572
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
          "seconds": 0.8151130999999623,
          "Mbps": 117.77506704284895
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.7541947000000846,
          "Mbps": 127.28808621963165
        }
      ],
      "system_before": "cpu  2437976 0 540051 18008064 96 42530 49201 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 3.5408217000000377,
        "Mbps": 225.93625654745378
      },
      "worker": {
        "cpu_core_percent": 64.0018122334199,
        "process_cpu_ns": 2266817483,
        "rss_kib": 9620,
        "seconds": 3.541802027,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91953,
            "in_place_decryptions": 91953,
            "in_place_encryptions": 16163,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 90956,
              "gso_datagrams": 14248,
              "kernel_fallbacks": 0,
              "max_receive_batch": 16,
              "max_send_batch": 16,
              "receive_calls": 8193,
              "receive_interrupted": 0,
              "receive_would_block": 3675,
              "received_bytes": 132144508,
              "received_datagrams": 91954,
              "send_calls": 4338,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 1509674,
              "sent_datagrams": 16165,
              "truncated_datagrams": 0
            },
            "network_backpressure": 0,
            "network_reads": 91954,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "parallel_batches": 3561,
            "parallel_control_barriers": 1,
            "parallel_helper_cpu": 1,
            "parallel_helper_packets": 42836,
            "parallel_max_batch": 32,
            "parallel_owner_cpu": 0,
            "parallel_owner_packets": 44208,
            "polls": 6417,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 63,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 91398,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 16163,
            "tun_write_calls": 7090,
            "tun_written_packets": 91953
          },
          "elapsed": {
            "nanos": 350860134,
            "secs": 5
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 682717294,
            "secs": 105719
          },
          "sequence": 9,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.10691428184509277,
            "handshake_age": {
              "nanos": 309465555,
              "secs": 5
            },
            "handshake_rtt": {
              "nanos": 40000000,
              "secs": 0
            },
            "received_bytes": 127928342,
            "sent_bytes": 992278
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2438157 0 540251 18008327 96 42548 49250 0 0 0\n",
      "forwarded_bytes": {
        "in": 100409888,
        "out": 781006
      },
      "valid": true,
      "cpu_ns_per_byte": 22.66817483
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
          "seconds": 0.6903682000000799,
          "Mbps": 139.05623115315694
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.6191512999999986,
          "Mbps": 155.0509544274561
        }
      ],
      "system_before": "cpu  2438245 0 540395 18008729 96 42554 49266 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 3.3855352000000494,
        "Mbps": 236.2994187743162
      },
      "worker": {
        "cpu_core_percent": 67.59877263927552,
        "process_cpu_ns": 2289238883,
        "rss_kib": 9148,
        "seconds": 3.386509538,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91950,
            "in_place_decryptions": 91950,
            "in_place_encryptions": 17105,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 90748,
              "gso_datagrams": 15634,
              "kernel_fallbacks": 0,
              "max_receive_batch": 16,
              "max_send_batch": 16,
              "receive_calls": 7735,
              "receive_interrupted": 0,
              "receive_would_block": 3403,
              "received_bytes": 132138764,
              "received_datagrams": 91951,
              "send_calls": 4210,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 1566346,
              "sent_datagrams": 17107,
              "truncated_datagrams": 0
            },
            "network_backpressure": 0,
            "network_reads": 91951,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "parallel_batches": 3575,
            "parallel_control_barriers": 1,
            "parallel_helper_cpu": 1,
            "parallel_helper_packets": 43087,
            "parallel_max_batch": 32,
            "parallel_owner_cpu": 0,
            "parallel_owner_packets": 44447,
            "polls": 5865,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 42,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 91473,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 17105,
            "tun_write_calls": 6797,
            "tun_written_packets": 91950
          },
          "elapsed": {
            "nanos": 927754735,
            "secs": 4
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 447508513,
            "secs": 105726
          },
          "sequence": 9,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.08697336912155151,
            "handshake_age": {
              "nanos": 896804403,
              "secs": 4
            },
            "handshake_rtt": {
              "nanos": 28000000,
              "secs": 0
            },
            "received_bytes": 127924739,
            "sent_bytes": 1018806
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2438441 0 540579 18008957 96 42572 49317 0 0 0\n",
      "forwarded_bytes": {
        "in": 100401498,
        "out": 784198
      },
      "valid": true,
      "cpu_ns_per_byte": 22.89238883
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
          "seconds": 0.8017317000000048,
          "Mbps": 119.74080605768665
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 1.1852480000000014,
          "Mbps": 80.99570722751685
        }
      ],
      "system_before": "cpu  2438580 0 540724 18009424 96 42580 49336 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 3.2033599000000095,
        "Mbps": 249.73778313201635
      },
      "worker": {
        "cpu_core_percent": 61.4388811350575,
        "process_cpu_ns": 1969085620,
        "rss_kib": 9420,
        "seconds": 3.204950324,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91939,
            "in_place_decryptions": 91939,
            "in_place_encryptions": 17930,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 90029,
              "gso_datagrams": 17221,
              "kernel_fallbacks": 0,
              "max_receive_batch": 16,
              "max_send_batch": 16,
              "receive_calls": 5507,
              "receive_interrupted": 0,
              "receive_would_block": 1800,
              "received_bytes": 132141404,
              "received_datagrams": 91940,
              "send_calls": 3043,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 1762374,
              "sent_datagrams": 17932,
              "truncated_datagrams": 0
            },
            "network_backpressure": 0,
            "network_reads": 91940,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 3640,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 51,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 91522,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 17930,
            "tun_write_calls": 6148,
            "tun_written_packets": 91939
          },
          "elapsed": {
            "nanos": 433370766,
            "secs": 5
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 716014300,
            "secs": 105733
          },
          "sequence": 10,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.10181611776351929,
            "handshake_age": {
              "nanos": 395358246,
              "secs": 5
            },
            "handshake_rtt": {
              "nanos": 37000000,
              "secs": 0
            },
            "received_bytes": 127927021,
            "sent_bytes": 1188434
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2438704 0 540883 18009717 96 42594 49383 0 0 0\n",
      "forwarded_bytes": {
        "in": 100372786,
        "out": 960298
      },
      "valid": true,
      "cpu_ns_per_byte": 19.6908562
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
          "seconds": 0.6280074000000013,
          "Mbps": 152.86444076932818
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.6167189000000235,
          "Mbps": 155.66249064200295
        }
      ],
      "system_before": "cpu  2439004 0 541071 18009926 96 42602 49402 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 2.935957099999996,
        "Mbps": 272.48354548504847
      },
      "worker": {
        "cpu_core_percent": 66.90446873019364,
        "process_cpu_ns": 1965151289,
        "rss_kib": 2616,
        "seconds": 2.937249673,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91995,
            "in_place_decryptions": 91995,
            "in_place_encryptions": 11035,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 8938,
              "receive_interrupted": 0,
              "receive_would_block": 517,
              "received_bytes": 130856916,
              "received_datagrams": 0,
              "send_calls": 1610,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 667196,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 1,
            "network_reads": 91996,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 2228,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 63,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 91679,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 11035,
            "tun_write_calls": 5379,
            "tun_written_packets": 91995
          },
          "elapsed": {
            "nanos": 477347976,
            "secs": 4
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 398796870,
            "secs": 105740
          },
          "sequence": 8,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.006361782550811768,
            "handshake_age": {
              "nanos": 427299053,
              "secs": 4
            },
            "handshake_rtt": {
              "nanos": 49000000,
              "secs": 0
            },
            "received_bytes": 127926934,
            "sent_bytes": 468382
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2439268 0 541298 18009957 96 42618 49451 0 0 0\n",
      "forwarded_bytes": {
        "in": 100351714,
        "out": 373202
      },
      "valid": true,
      "cpu_ns_per_byte": 19.65151289
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
          "seconds": 0.6164132999999765,
          "Mbps": 155.7396636315337
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.5615725000000111,
          "Mbps": 170.94854181783845
        }
      ],
      "system_before": "cpu  2439482 0 541458 18010222 96 42624 49466 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 2.7216891000000487,
        "Mbps": 293.93511551337207
      },
      "worker": {
        "cpu_core_percent": 66.78775508710342,
        "process_cpu_ns": 1818217623,
        "rss_kib": 2632,
        "seconds": 2.7223817009999998,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91946,
            "in_place_decryptions": 91946,
            "in_place_encryptions": 10557,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 8968,
              "receive_interrupted": 0,
              "receive_would_block": 553,
              "received_bytes": 130849842,
              "received_datagrams": 0,
              "send_calls": 1676,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 623704,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 2,
            "network_reads": 91947,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 2269,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 59,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 91008,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 10557,
            "tun_write_calls": 5791,
            "tun_written_packets": 91946
          },
          "elapsed": {
            "nanos": 142346955,
            "secs": 4
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 523323399,
            "secs": 105746
          },
          "sequence": 8,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 4.3511390686035156e-05,
            "handshake_age": {
              "nanos": 92070437,
              "secs": 4
            },
            "handshake_rtt": {
              "nanos": 49000000,
              "secs": 0
            },
            "received_bytes": 127922171,
            "sent_bytes": 433494
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2439594 0 541591 18010475 96 42636 49505 0 0 0\n",
      "forwarded_bytes": {
        "in": 100368032,
        "out": 334742
      },
      "valid": true,
      "cpu_ns_per_byte": 18.18217623
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
          "seconds": 0.7422460999999885,
          "Mbps": 129.3371565037546
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.7295937000000094,
          "Mbps": 131.5800835451276
        }
      ],
      "system_before": "cpu  2439726 0 541743 18010870 96 42643 49523 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 3.114248900000007,
        "Mbps": 256.8837705939298
      },
      "worker": {
        "cpu_core_percent": 62.40391049691412,
        "process_cpu_ns": 1944811546,
        "rss_kib": 2444,
        "seconds": 3.116489865,
        "snapshot": {
          "counters": {
            "decrypted_packets": 92002,
            "in_place_decryptions": 92002,
            "in_place_encryptions": 10698,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 11098,
              "receive_interrupted": 0,
              "receive_would_block": 1807,
              "received_bytes": 130854946,
              "received_datagrams": 0,
              "send_calls": 2715,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 634498,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 2,
            "network_reads": 92003,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 3834,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 36,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 91447,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 10698,
            "tun_write_calls": 6276,
            "tun_written_packets": 92002
          },
          "elapsed": {
            "nanos": 846484040,
            "secs": 4
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 267881536,
            "secs": 105753
          },
          "sequence": 9,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 4.3451786041259766e-05,
            "handshake_age": {
              "nanos": 770423452,
              "secs": 4
            },
            "handshake_rtt": {
              "nanos": 75000000,
              "secs": 0
            },
            "received_bytes": 127925491,
            "sent_bytes": 441750
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2439850 0 541925 18011124 96 42657 49569 0 0 0\n",
      "forwarded_bytes": {
        "in": 100376471,
        "out": 335322
      },
      "valid": true,
      "cpu_ns_per_byte": 19.44811546
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
          "seconds": 0.7521739000000025,
          "Mbps": 127.63006001670581
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.7240484999999808,
          "Mbps": 132.58780316512298
        }
      ],
      "system_before": "cpu  2439950 0 542067 18011549 96 42664 49586 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 3.3060262000000193,
        "Mbps": 241.98235331589183
      },
      "worker": {
        "cpu_core_percent": 59.03318113816245,
        "process_cpu_ns": 1952074618,
        "rss_kib": 2608,
        "seconds": 3.306741362,
        "snapshot": {
          "counters": {
            "decrypted_packets": 92102,
            "in_place_decryptions": 92102,
            "in_place_encryptions": 11619,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 11099,
              "receive_interrupted": 0,
              "receive_would_block": 1793,
              "received_bytes": 130991434,
              "received_datagrams": 0,
              "send_calls": 2618,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 719072,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 2,
            "network_reads": 92103,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 3809,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 55,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 91516,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 11619,
            "tun_write_calls": 6317,
            "tun_written_packets": 92102
          },
          "elapsed": {
            "nanos": 27641392,
            "secs": 5
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 157860755,
            "secs": 105760
          },
          "sequence": 9,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.00144195556640625,
            "handshake_age": {
              "nanos": 953574214,
              "secs": 4
            },
            "handshake_rtt": {
              "nanos": 73000000,
              "secs": 0
            },
            "received_bytes": 128059222,
            "sent_bytes": 509746
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2440105 0 542244 18011817 96 42678 49634 0 0 0\n",
      "forwarded_bytes": {
        "in": 100377751,
        "out": 387426
      },
      "valid": true,
      "cpu_ns_per_byte": 19.52074618
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
          "seconds": 0.71590470000001,
          "Mbps": 134.09606055107426
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.6922978999999714,
          "Mbps": 138.66862805737813
        }
      ],
      "system_before": "cpu  2440194 0 542372 18012252 96 42684 49651 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 3.1350826000000325,
        "Mbps": 255.17668976249357
      },
      "worker": {
        "cpu_core_percent": 61.68636353125458,
        "process_cpu_ns": 1935075568,
        "rss_kib": 2452,
        "seconds": 3.136958409,
        "snapshot": {
          "counters": {
            "decrypted_packets": 92012,
            "in_place_decryptions": 92012,
            "in_place_encryptions": 10621,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 10834,
              "receive_interrupted": 0,
              "receive_would_block": 1641,
              "received_bytes": 130869334,
              "received_datagrams": 0,
              "send_calls": 2546,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 629040,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 2,
            "network_reads": 92013,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 3659,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 38,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 91502,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 10621,
            "tun_write_calls": 6177,
            "tun_written_packets": 92012
          },
          "elapsed": {
            "nanos": 789141339,
            "secs": 4
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 818592953,
            "secs": 105766
          },
          "sequence": 9,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 9.781122207641602e-05,
            "handshake_age": {
              "nanos": 717857457,
              "secs": 4
            },
            "handshake_rtt": {
              "nanos": 70000000,
              "secs": 0
            },
            "received_bytes": 127938898,
            "sent_bytes": 437678
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2440347 0 542536 18012487 96 42699 49698 0 0 0\n",
      "forwarded_bytes": {
        "in": 100374387,
        "out": 333542
      },
      "valid": true,
      "cpu_ns_per_byte": 19.35075568
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
          "seconds": 0.5856156000000965,
          "Mbps": 163.93005924019815
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.6097523999999339,
          "Mbps": 157.4409547219665
        }
      ],
      "system_before": "cpu  2440446 0 542660 18012881 96 42706 49714 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 3.9785458000000062,
        "Mbps": 201.07849455949426
      },
      "worker": {
        "cpu_core_percent": 53.40420725075251,
        "process_cpu_ns": 2125495903,
        "rss_kib": 2596,
        "seconds": 3.9800158310000002,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91954,
            "in_place_decryptions": 91954,
            "in_place_encryptions": 11059,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 9263,
              "receive_interrupted": 0,
              "receive_would_block": 715,
              "received_bytes": 130854738,
              "received_datagrams": 0,
              "send_calls": 1920,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 677628,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 2,
            "network_reads": 91955,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "parallel_batches": 3000,
            "parallel_control_barriers": 1,
            "parallel_helper_cpu": 1,
            "parallel_helper_packets": 44624,
            "parallel_max_batch": 32,
            "parallel_owner_cpu": 0,
            "parallel_owner_packets": 46074,
            "polls": 2456,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 46,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 91581,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 11059,
            "tun_write_calls": 6215,
            "tun_written_packets": 91954
          },
          "elapsed": {
            "nanos": 420620593,
            "secs": 5
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 111091399,
            "secs": 105774
          },
          "sequence": 9,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.011534273624420166,
            "handshake_age": {
              "nanos": 366226530,
              "secs": 5
            },
            "handshake_rtt": {
              "nanos": 53000000,
              "secs": 0
            },
            "received_bytes": 127925166,
            "sent_bytes": 478382
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2440838 0 542892 18012972 96 42724 49771 0 0 0\n",
      "forwarded_bytes": {
        "in": 100382089,
        "out": 337238
      },
      "valid": true,
      "cpu_ns_per_byte": 21.25495903
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
          "seconds": 0.795497800000021,
          "Mbps": 120.67915209821757
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.7330474999999979,
          "Mbps": 130.96013559830746
        }
      ],
      "system_before": "cpu  2441187 0 543091 18013148 96 42731 49790 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 3.124758199999974,
        "Mbps": 256.01980978880437
      },
      "worker": {
        "cpu_core_percent": 68.84020074586292,
        "process_cpu_ns": 2152162611,
        "rss_kib": 2412,
        "seconds": 3.126316582,
        "snapshot": {
          "counters": {
            "decrypted_packets": 92044,
            "in_place_decryptions": 92044,
            "in_place_encryptions": 10727,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 12485,
              "receive_interrupted": 0,
              "receive_would_block": 2652,
              "received_bytes": 130874806,
              "received_datagrams": 0,
              "send_calls": 3307,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 638556,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 2,
            "network_reads": 92045,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "parallel_batches": 3345,
            "parallel_control_barriers": 1,
            "parallel_helper_cpu": 1,
            "parallel_helper_packets": 43455,
            "parallel_max_batch": 32,
            "parallel_owner_cpu": 0,
            "parallel_owner_packets": 44861,
            "polls": 5003,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 30,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 91390,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 10727,
            "tun_write_calls": 6801,
            "tun_written_packets": 92044
          },
          "elapsed": {
            "nanos": 901982153,
            "secs": 4
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 97745257,
            "secs": 105781
          },
          "sequence": 9,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.0010418891906738281,
            "handshake_age": {
              "nanos": 831296571,
              "secs": 4
            },
            "handshake_rtt": {
              "nanos": 69000000,
              "secs": 0
            },
            "received_bytes": 127943652,
            "sent_bytes": 445286
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2441312 0 543247 18013420 96 42747 49840 0 0 0\n",
      "forwarded_bytes": {
        "in": 100410780,
        "out": 356630
      },
      "valid": true,
      "cpu_ns_per_byte": 21.52162611
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
          "seconds": 0.5789902999999867,
          "Mbps": 165.80588655803422
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.5740425000000187,
          "Mbps": 167.2350043768482
        }
      ],
      "system_before": "cpu  2441456 0 543381 18013769 96 42753 49855 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 2.780762999999979,
        "Mbps": 287.69082442480936
      },
      "worker": {
        "cpu_core_percent": 65.33046894673174,
        "process_cpu_ns": 1817789648,
        "rss_kib": 2528,
        "seconds": 2.782453084,
        "snapshot": {
          "counters": {
            "decrypted_packets": 92014,
            "in_place_decryptions": 92014,
            "in_place_encryptions": 10841,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 9209,
              "receive_interrupted": 0,
              "receive_would_block": 700,
              "received_bytes": 130857770,
              "received_datagrams": 0,
              "send_calls": 1766,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 649608,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 2,
            "network_reads": 92015,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 2419,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 63,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 91677,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 10841,
            "tun_write_calls": 5279,
            "tun_written_packets": 92014
          },
          "elapsed": {
            "nanos": 224760187,
            "secs": 4
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 224034238,
            "secs": 105787
          },
          "sequence": 8,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.000684201717376709,
            "handshake_age": {
              "nanos": 175672576,
              "secs": 4
            },
            "handshake_rtt": {
              "nanos": 48000000,
              "secs": 0
            },
            "received_bytes": 127928380,
            "sent_bytes": 454286
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2441568 0 543528 18014024 96 42767 49898 0 0 0\n",
      "forwarded_bytes": {
        "in": 100345995,
        "out": 346694
      },
      "valid": true,
      "cpu_ns_per_byte": 18.17789648
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
          "seconds": 0.7160200000000714,
          "Mbps": 134.07446719364043
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.6949296000000231,
          "Mbps": 138.14348964268726
        }
      ],
      "system_before": "cpu  2441712 0 543681 18014389 96 42773 49917 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 3.5226956999999857,
        "Mbps": 227.09880958494463
      },
      "worker": {
        "cpu_core_percent": 72.77106579950669,
        "process_cpu_ns": 2564078397,
        "rss_kib": 3284,
        "seconds": 3.523486112,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91918,
            "in_place_decryptions": 91918,
            "in_place_encryptions": 10779,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 3265,
              "receive_interrupted": 0,
              "receive_would_block": 534,
              "received_bytes": 134899847,
              "received_datagrams": 0,
              "send_calls": 1736,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 677164,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 3,
            "network_reads": 91919,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 2282,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 55,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 91562,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 10779,
            "tun_write_calls": 5755,
            "tun_written_packets": 91918
          },
          "elapsed": {
            "nanos": 184402412,
            "secs": 5
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 334431963,
            "secs": 105794
          },
          "sequence": 9,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 5.441904067993164e-05,
            "handshake_age": {
              "nanos": 108282577,
              "secs": 5
            },
            "handshake_rtt": {
              "nanos": 74000000,
              "secs": 0
            },
            "received_bytes": 127924169,
            "sent_bytes": 444510
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2441926 0 543845 18014661 96 42789 49963 0 0 0\n",
      "forwarded_bytes": {
        "in": 100362532,
        "out": 333942
      },
      "valid": true,
      "cpu_ns_per_byte": 25.64078397
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
          "seconds": 0.816919700000085,
          "Mbps": 117.51461006508964
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.79529720000005,
          "Mbps": 120.7095913326414
        }
      ],
      "system_before": "cpu  2442028 0 544007 18015083 96 42796 49982 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 5.8041107999999895,
        "Mbps": 137.83334391204275
      },
      "worker": {
        "cpu_core_percent": 55.299887782958855,
        "process_cpu_ns": 3210118452,
        "rss_kib": 3356,
        "seconds": 5.804927606,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91987,
            "in_place_decryptions": 91987,
            "in_place_encryptions": 11067,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 11983,
              "receive_interrupted": 0,
              "receive_would_block": 4098,
              "received_bytes": 134905791,
              "received_datagrams": 0,
              "send_calls": 4335,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 765483,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 3,
            "network_reads": 91988,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 6933,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 63,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 90932,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 11067,
            "tun_write_calls": 9974,
            "tun_written_packets": 91987
          },
          "elapsed": {
            "nanos": 666005670,
            "secs": 7
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 846198492,
            "secs": 105803
          },
          "sequence": 11,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.0011510252952575684,
            "handshake_age": {
              "nanos": 563917721,
              "secs": 7
            },
            "handshake_rtt": {
              "nanos": 100000000,
              "secs": 0
            },
            "received_bytes": 127928263,
            "sent_bytes": 470470
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2442279 0 544262 18015637 96 42819 50050 0 0 0\n",
      "forwarded_bytes": {
        "in": 100532056,
        "out": 340586
      },
      "valid": true,
      "cpu_ns_per_byte": 32.10118452
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
          "seconds": 1.0005855999999085,
          "Mbps": 95.94381530176807
        }
      ],
      "system_before": "cpu  2442551 0 544438 18015840 96 42825 50064 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 5.826365800000076,
        "Mbps": 137.30686116549523
      },
      "worker": {
        "cpu_core_percent": 56.48071588543535,
        "process_cpu_ns": 3291200024,
        "rss_kib": 3308,
        "seconds": 5.827121651,
        "snapshot": {
          "counters": {
            "decrypted_packets": 82980,
            "in_place_decryptions": 82980,
            "in_place_encryptions": 8869,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 9860,
              "receive_interrupted": 0,
              "receive_would_block": 3168,
              "received_bytes": 121836060,
              "received_datagrams": 0,
              "send_calls": 3516,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 595900,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 3,
            "network_reads": 82981,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 5552,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 32,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 82222,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 8869,
            "tun_write_calls": 8973,
            "tun_written_packets": 82980
          },
          "elapsed": {
            "nanos": 109041328,
            "secs": 7
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 158254760,
            "secs": 105813
          },
          "sequence": 11,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.0,
            "handshake_age": {
              "nanos": 998184774,
              "secs": 6
            },
            "handshake_rtt": {
              "nanos": 104000000,
              "secs": 0
            },
            "received_bytes": 115537320,
            "sent_bytes": 358468
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2443044 0 544756 18016070 96 42852 50133 0 0 0\n",
      "forwarded_bytes": {
        "in": 100510831,
        "out": 315622
      },
      "valid": true,
      "cpu_ns_per_byte": 32.91200024
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
          "seconds": 0.655883499999959,
          "Mbps": 146.36745702553276
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.7083608000000368,
          "Mbps": 135.52415661622584
        }
      ],
      "system_before": "cpu  2443150 0 544885 18016469 96 42858 50150 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 3.5368394000000762,
        "Mbps": 226.1906491993905
      },
      "worker": {
        "cpu_core_percent": 75.08461197252889,
        "process_cpu_ns": 2656254783,
        "rss_kib": 3464,
        "seconds": 3.537681974,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91956,
            "in_place_decryptions": 91956,
            "in_place_encryptions": 10656,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 3073,
              "receive_interrupted": 0,
              "receive_would_block": 457,
              "received_bytes": 134903420,
              "received_datagrams": 0,
              "send_calls": 1709,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 668150,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 3,
            "network_reads": 91957,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 2175,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 52,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 91637,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 10656,
            "tun_write_calls": 5775,
            "tun_written_packets": 91956
          },
          "elapsed": {
            "nanos": 138740648,
            "secs": 5
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 108475584,
            "secs": 105820
          },
          "sequence": 9,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.0004348158836364746,
            "handshake_age": {
              "nanos": 60497372,
              "secs": 5
            },
            "handshake_rtt": {
              "nanos": 77000000,
              "secs": 0
            },
            "received_bytes": 127926874,
            "sent_bytes": 438306
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2443395 0 545032 18016713 96 42874 50197 0 0 0\n",
      "forwarded_bytes": {
        "in": 100365443,
        "out": 333462
      },
      "valid": true,
      "cpu_ns_per_byte": 26.56254783
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
          "seconds": 0.977452699999958,
          "Mbps": 98.21447114525759
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.9152928999999403,
          "Mbps": 104.88445829745457
        }
      ],
      "system_before": "cpu  2443542 0 545192 18017147 96 42882 50217 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 3.5047395999999935,
        "Mbps": 228.26232225641002
      },
      "worker": {
        "cpu_core_percent": 73.43923193432674,
        "process_cpu_ns": 2575077936,
        "rss_kib": 3176,
        "seconds": 3.50640641,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91940,
            "in_place_decryptions": 91940,
            "in_place_encryptions": 10434,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 4283,
              "receive_interrupted": 0,
              "receive_would_block": 954,
              "received_bytes": 134900365,
              "received_datagrams": 0,
              "send_calls": 2115,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 662745,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 3,
            "network_reads": 91941,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 2812,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 43,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 91536,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 10434,
            "tun_write_calls": 6134,
            "tun_written_packets": 91940
          },
          "elapsed": {
            "nanos": 648591179,
            "secs": 5
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 607574941,
            "secs": 105827
          },
          "sequence": 9,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 3.260374069213867e-05,
            "handshake_age": {
              "nanos": 540825227,
              "secs": 5
            },
            "handshake_rtt": {
              "nanos": 105000000,
              "secs": 0
            },
            "received_bytes": 127925524,
            "sent_bytes": 427962
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2443717 0 545349 18017459 96 42896 50263 0 0 0\n",
      "forwarded_bytes": {
        "in": 100367529,
        "out": 331950
      },
      "valid": true,
      "cpu_ns_per_byte": 25.75077936
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
          "seconds": 0.8152600000000803,
          "Mbps": 117.75384539900223
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.7064822999999478,
          "Mbps": 135.88450835924283
        }
      ],
      "system_before": "cpu  2443888 0 545510 18017822 96 42904 50282 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 3.30104039999992,
        "Mbps": 242.34783676080409
      },
      "worker": {
        "cpu_core_percent": 81.02524348223268,
        "process_cpu_ns": 2676085625,
        "rss_kib": 3128,
        "seconds": 3.302780109,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91965,
            "in_place_decryptions": 91965,
            "in_place_encryptions": 11235,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 3701,
              "receive_interrupted": 0,
              "receive_would_block": 693,
              "received_bytes": 134902219,
              "received_datagrams": 0,
              "send_calls": 1958,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 729234,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 3,
            "network_reads": 91966,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "parallel_batches": 2986,
            "parallel_control_barriers": 1,
            "parallel_helper_cpu": 1,
            "parallel_helper_packets": 44656,
            "parallel_max_batch": 32,
            "parallel_owner_cpu": 0,
            "parallel_owner_packets": 46124,
            "polls": 2518,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 32,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 91527,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 11235,
            "tun_write_calls": 5865,
            "tun_written_packets": 91965
          },
          "elapsed": {
            "nanos": 82627557,
            "secs": 5
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 644598759,
            "secs": 105834
          },
          "sequence": 9,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.0035215020179748535,
            "handshake_age": {
              "nanos": 4875015,
              "secs": 5
            },
            "handshake_rtt": {
              "nanos": 75000000,
              "secs": 0
            },
            "received_bytes": 127923687,
            "sent_bytes": 483494
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2444063 0 545681 18018074 96 42920 50330 0 0 0\n",
      "forwarded_bytes": {
        "in": 100360226,
        "out": 344670
      },
      "valid": true,
      "cpu_ns_per_byte": 26.76085625
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
          "seconds": 0.7172368000000233,
          "Mbps": 133.84700840781855
        },
        {
          "requested_bytes": 12000000,
          "valid": true,
          "tls": "TLSv1.2",
          "status": 206,
          "content_range": "bytes 0-11999999/148648524",
          "bytes": 12000000,
          "sha256": "389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67",
          "seconds": 0.6668561000000182,
          "Mbps": 143.95909402342934
        }
      ],
      "system_before": "cpu  2444231 0 545830 18018421 96 42928 50348 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 4.543356099999983,
        "Mbps": 176.0812893358729
      },
      "worker": {
        "cpu_core_percent": 64.64063070230011,
        "process_cpu_ns": 2937992719,
        "rss_kib": 3376,
        "seconds": 4.545117656,
        "snapshot": {
          "counters": {
            "decrypted_packets": 91956,
            "in_place_decryptions": 91956,
            "in_place_encryptions": 10893,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 4350,
              "receive_interrupted": 0,
              "receive_would_block": 965,
              "received_bytes": 134901355,
              "received_datagrams": 0,
              "send_calls": 2138,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 688814,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 3,
            "network_reads": 91957,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "parallel_batches": 3052,
            "parallel_control_barriers": 1,
            "parallel_helper_cpu": 1,
            "parallel_helper_packets": 44518,
            "parallel_max_batch": 32,
            "parallel_owner_cpu": 0,
            "parallel_owner_packets": 45977,
            "polls": 2793,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 35,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 91589,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 10893,
            "tun_write_calls": 6380,
            "tun_written_packets": 91956
          },
          "elapsed": {
            "nanos": 197012296,
            "secs": 6
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 726698147,
            "secs": 105842
          },
          "sequence": 10,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 0.000315248966217041,
            "handshake_age": {
              "nanos": 116075131,
              "secs": 6
            },
            "handshake_rtt": {
              "nanos": 79000000,
              "secs": 0
            },
            "received_bytes": 127924627,
            "sent_bytes": 445266
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2444626 0 546063 18018613 96 42949 50408 0 0 0\n",
      "forwarded_bytes": {
        "in": 100383468,
        "out": 344690
      },
      "valid": true,
      "cpu_ns_per_byte": 29.37992719
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
          "seconds": 5.554951599999981,
          "Mbps": 17.281878747602466
        }
      ],
      "system_before": "cpu  2445025 0 546244 18019564 96 42956 50421 0 0 0\n",
      "download": {
        "requested_bytes": 100000000,
        "valid": true,
        "tls": "TLSv1.2",
        "status": 206,
        "content_range": "bytes 0-99999999/148648524",
        "bytes": 100000000,
        "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
        "seconds": 3.553462599999989,
        "Mbps": 225.13252285249953
      },
      "worker": {
        "cpu_core_percent": 72.13924138104873,
        "process_cpu_ns": 2564031935,
        "rss_kib": 3264,
        "seconds": 3.554281811,
        "snapshot": {
          "counters": {
            "decrypted_packets": 83014,
            "in_place_decryptions": 83014,
            "in_place_encryptions": 9648,
            "inbound_filtered": 0,
            "invalid_packets": 0,
            "io": {
              "enobufs": 0,
              "gro_datagrams": 0,
              "gso_datagrams": 0,
              "kernel_fallbacks": 0,
              "max_receive_batch": 0,
              "max_send_batch": 0,
              "receive_calls": 2639,
              "receive_interrupted": 0,
              "receive_would_block": 342,
              "received_bytes": 121842792,
              "received_datagrams": 0,
              "send_calls": 1459,
              "send_interrupted": 0,
              "send_would_block": 0,
              "sent_bytes": 599945,
              "sent_datagrams": 0,
              "truncated_datagrams": 0
            },
            "network_backpressure": 3,
            "network_reads": 83015,
            "outbound_filtered": 0,
            "outer_eof": 0,
            "outer_io_errors": 0,
            "polls": 1868,
            "pool_pressure": 0,
            "pool_recycle_wipe_bytes": 0,
            "pool_reserved_bytes": 405758,
            "successful_crypto_copy_bytes": 0,
            "to_network_bytes_high_water": 0,
            "to_network_high_water": 63,
            "to_tun_bytes_high_water": 0,
            "to_tun_high_water": 63,
            "tun_backpressure": 0,
            "tun_coalesced_packets": 82747,
            "tun_max_write_batch": 32,
            "tun_offload_fallbacks": 0,
            "tun_reads": 9648,
            "tun_write_calls": 5049,
            "tun_written_packets": 83014
          },
          "elapsed": {
            "nanos": 363796408,
            "secs": 9
          },
          "interface_error": null,
          "mono_timestamp": {
            "nanos": 195648047,
            "secs": 105854
          },
          "sequence": 12,
          "state": "TunnelEstablished",
          "statistics": {
            "estimated_loss": 4.8160552978515625e-05,
            "handshake_age": {
              "nanos": 330266767,
              "secs": 4
            },
            "handshake_rtt": {
              "nanos": 27000000,
              "secs": 5
            },
            "received_bytes": 115542996,
            "sent_bytes": 393600
          },
          "stats_generation": 2,
          "tun_present": true
        },
        "tun_mode": "vnet"
      },
      "system_after": "cpu  2445268 0 546405 18019822 96 42972 50466 0 0 0\n",
      "forwarded_bytes": {
        "in": 100363758,
        "out": 345958
      },
      "valid": true,
      "cpu_ns_per_byte": 25.64031935
    }
  ],
  "started_utc": "2026-11-12T03:47:42.405122+00:00",
  "kernel": "6.12.112",
  "server": "US-NY#544",
  "mtu": 1280,
  "method": "laptop HTTPS; socket-bound interface; owned router policy/NAT only",
  "binaries": {
    "main": "e542b9ede783d06e20ba63a2ccd4ed59751f5fe61816a8d47ea64909a3e4e224",
    "erase": "cb736f08ff06c6a5bbd5797aaa4a90edcd04c1426cac289b08d88e470630f488"
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
    "seconds": 1.7841675999999325,
    "Mbps": 448.38836889540545
  },
  "shutdowns": [
    "{\"tun_reads\":25,\"network_reads\":22,\"decrypted_packets\":21,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":36,\"tun_backpressure\":0,\"tun_write_calls\":21,\"tun_written_packets\":21,\"tun_coalesced_packets\":0,\"tun_max_write_batch\":1,\"tun_offload_fallbacks\":0,\"network_backpressure\":0,\"io\":{\"receive_calls\":31,\"send_calls\":22,\"received_datagrams\":22,\"sent_datagrams\":27,\"max_receive_batch\":3,\"max_send_batch\":2,\"truncated_datagrams\":0,\"gso_datagrams\":8,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":5612,\"sent_bytes\":6744,\"receive_would_block\":16,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":3,\"to_tun_high_water\":3,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":25,\"in_place_decryptions\":21,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":16909,\"network_reads\":83001,\"decrypted_packets\":83000,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":3500,\"tun_backpressure\":0,\"tun_write_calls\":5543,\"tun_written_packets\":83000,\"tun_coalesced_packets\":82650,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":0,\"io\":{\"receive_calls\":5344,\"send_calls\":2928,\"received_datagrams\":83001,\"sent_datagrams\":16911,\"max_receive_batch\":16,\"max_send_batch\":16,\"truncated_datagrams\":0,\"gso_datagrams\":16209,\"gro_datagrams\":81804,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":119348252,\"sent_bytes\":1600480,\"receive_would_block\":1845,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":59,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":16909,\"in_place_decryptions\":83000,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":19012,\"network_reads\":91918,\"decrypted_packets\":91917,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":3838,\"tun_backpressure\":0,\"tun_write_calls\":6211,\"tun_written_packets\":91917,\"tun_coalesced_packets\":91541,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":0,\"io\":{\"receive_calls\":5926,\"send_calls\":3226,\"received_datagrams\":91918,\"sent_datagrams\":19014,\"max_receive_batch\":16,\"max_send_batch\":16,\"truncated_datagrams\":0,\"gso_datagrams\":18214,\"gro_datagrams\":89778,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":132139900,\"sent_bytes\":1841918,\"receive_would_block\":2001,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":63,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":19012,\"in_place_decryptions\":91917,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":18902,\"network_reads\":91908,\"decrypted_packets\":91907,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":3391,\"tun_backpressure\":0,\"tun_write_calls\":5954,\"tun_written_packets\":91907,\"tun_coalesced_packets\":91525,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":0,\"io\":{\"receive_calls\":5064,\"send_calls\":3016,\"received_datagrams\":91908,\"sent_datagrams\":18904,\"max_receive_batch\":16,\"max_send_batch\":16,\"truncated_datagrams\":0,\"gso_datagrams\":18268,\"gro_datagrams\":90768,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":132139132,\"sent_bytes\":1806366,\"receive_would_block\":1611,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":61,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":18902,\"in_place_decryptions\":91907,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":17589,\"network_reads\":82969,\"decrypted_packets\":82968,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":3025,\"tun_backpressure\":0,\"tun_write_calls\":5679,\"tun_written_packets\":82968,\"tun_coalesced_packets\":82584,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":0,\"io\":{\"receive_calls\":4810,\"send_calls\":2734,\"received_datagrams\":82969,\"sent_datagrams\":17591,\"max_receive_batch\":16,\"max_send_batch\":16,\"truncated_datagrams\":0,\"gso_datagrams\":17016,\"gro_datagrams\":81742,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":119344476,\"sent_bytes\":1740872,\"receive_would_block\":1647,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":60,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":17589,\"in_place_decryptions\":82968,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":16163,\"network_reads\":91954,\"decrypted_packets\":91953,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":6418,\"tun_backpressure\":0,\"tun_write_calls\":7090,\"tun_written_packets\":91953,\"tun_coalesced_packets\":91398,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":0,\"io\":{\"receive_calls\":8193,\"send_calls\":4338,\"received_datagrams\":91954,\"sent_datagrams\":16165,\"max_receive_batch\":16,\"max_send_batch\":16,\"truncated_datagrams\":0,\"gso_datagrams\":14248,\"gro_datagrams\":90956,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":132144508,\"sent_bytes\":1509674,\"receive_would_block\":3675,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":63,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":16163,\"in_place_decryptions\":91953,\"parallel_batches\":3561,\"parallel_helper_packets\":42836,\"parallel_owner_packets\":44208,\"parallel_max_batch\":32,\"parallel_control_barriers\":1,\"parallel_owner_cpu\":0,\"parallel_helper_cpu\":1,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":17105,\"network_reads\":91952,\"decrypted_packets\":91951,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":5867,\"tun_backpressure\":0,\"tun_write_calls\":6798,\"tun_written_packets\":91951,\"tun_coalesced_packets\":91473,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":0,\"io\":{\"receive_calls\":7737,\"send_calls\":4210,\"received_datagrams\":91952,\"sent_datagrams\":17107,\"max_receive_batch\":16,\"max_send_batch\":16,\"truncated_datagrams\":0,\"gso_datagrams\":15634,\"gro_datagrams\":90748,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":132139580,\"sent_bytes\":1566346,\"receive_would_block\":3404,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":42,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":17105,\"in_place_decryptions\":91951,\"parallel_batches\":3575,\"parallel_helper_packets\":43087,\"parallel_owner_packets\":44447,\"parallel_max_batch\":32,\"parallel_control_barriers\":1,\"parallel_owner_cpu\":0,\"parallel_helper_cpu\":1,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":17930,\"network_reads\":91941,\"decrypted_packets\":91940,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":3642,\"tun_backpressure\":0,\"tun_write_calls\":6149,\"tun_written_packets\":91940,\"tun_coalesced_packets\":91522,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":0,\"io\":{\"receive_calls\":5509,\"send_calls\":3043,\"received_datagrams\":91941,\"sent_datagrams\":17932,\"max_receive_batch\":16,\"max_send_batch\":16,\"truncated_datagrams\":0,\"gso_datagrams\":17221,\"gro_datagrams\":90029,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":132141900,\"sent_bytes\":1762374,\"receive_would_block\":1801,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":51,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":17930,\"in_place_decryptions\":91940,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":11035,\"network_reads\":91996,\"decrypted_packets\":91995,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":2229,\"tun_backpressure\":0,\"tun_write_calls\":5379,\"tun_written_packets\":91995,\"tun_coalesced_packets\":91679,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":1,\"io\":{\"receive_calls\":8938,\"send_calls\":1610,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":130856916,\"sent_bytes\":667196,\"receive_would_block\":517,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":63,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":11035,\"in_place_decryptions\":91995,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":10557,\"network_reads\":91947,\"decrypted_packets\":91946,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":2270,\"tun_backpressure\":0,\"tun_write_calls\":5791,\"tun_written_packets\":91946,\"tun_coalesced_packets\":91008,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":2,\"io\":{\"receive_calls\":8968,\"send_calls\":1676,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":130849842,\"sent_bytes\":623704,\"receive_would_block\":553,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":59,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":10557,\"in_place_decryptions\":91946,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":10698,\"network_reads\":92003,\"decrypted_packets\":92002,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":3835,\"tun_backpressure\":0,\"tun_write_calls\":6276,\"tun_written_packets\":92002,\"tun_coalesced_packets\":91447,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":2,\"io\":{\"receive_calls\":11098,\"send_calls\":2715,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":130854946,\"sent_bytes\":634498,\"receive_would_block\":1807,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":36,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":10698,\"in_place_decryptions\":92002,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":11619,\"network_reads\":92103,\"decrypted_packets\":92102,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":3810,\"tun_backpressure\":0,\"tun_write_calls\":6317,\"tun_written_packets\":92102,\"tun_coalesced_packets\":91516,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":2,\"io\":{\"receive_calls\":11099,\"send_calls\":2618,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":130991434,\"sent_bytes\":719072,\"receive_would_block\":1793,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":55,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":11619,\"in_place_decryptions\":92102,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":10621,\"network_reads\":92013,\"decrypted_packets\":92012,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":3660,\"tun_backpressure\":0,\"tun_write_calls\":6177,\"tun_written_packets\":92012,\"tun_coalesced_packets\":91502,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":2,\"io\":{\"receive_calls\":10834,\"send_calls\":2546,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":130869334,\"sent_bytes\":629040,\"receive_would_block\":1641,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":38,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":10621,\"in_place_decryptions\":92012,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":11059,\"network_reads\":91955,\"decrypted_packets\":91954,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":2457,\"tun_backpressure\":0,\"tun_write_calls\":6215,\"tun_written_packets\":91954,\"tun_coalesced_packets\":91581,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":2,\"io\":{\"receive_calls\":9263,\"send_calls\":1920,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":130854738,\"sent_bytes\":677628,\"receive_would_block\":715,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":46,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":11059,\"in_place_decryptions\":91954,\"parallel_batches\":3000,\"parallel_helper_packets\":44624,\"parallel_owner_packets\":46074,\"parallel_max_batch\":32,\"parallel_control_barriers\":1,\"parallel_owner_cpu\":0,\"parallel_helper_cpu\":1,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":10727,\"network_reads\":92045,\"decrypted_packets\":92044,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":5004,\"tun_backpressure\":0,\"tun_write_calls\":6801,\"tun_written_packets\":92044,\"tun_coalesced_packets\":91390,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":2,\"io\":{\"receive_calls\":12485,\"send_calls\":3307,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":130874806,\"sent_bytes\":638556,\"receive_would_block\":2652,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":30,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":10727,\"in_place_decryptions\":92044,\"parallel_batches\":3345,\"parallel_helper_packets\":43455,\"parallel_owner_packets\":44861,\"parallel_max_batch\":32,\"parallel_control_barriers\":1,\"parallel_owner_cpu\":0,\"parallel_helper_cpu\":1,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":10841,\"network_reads\":92015,\"decrypted_packets\":92014,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":2420,\"tun_backpressure\":0,\"tun_write_calls\":5279,\"tun_written_packets\":92014,\"tun_coalesced_packets\":91677,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":2,\"io\":{\"receive_calls\":9209,\"send_calls\":1766,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":130857770,\"sent_bytes\":649608,\"receive_would_block\":700,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":63,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":10841,\"in_place_decryptions\":92014,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":10779,\"network_reads\":91919,\"decrypted_packets\":91918,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":2283,\"tun_backpressure\":0,\"tun_write_calls\":5755,\"tun_written_packets\":91918,\"tun_coalesced_packets\":91562,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":3,\"io\":{\"receive_calls\":3265,\"send_calls\":1736,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":134899847,\"sent_bytes\":677164,\"receive_would_block\":534,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":55,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":10779,\"in_place_decryptions\":91918,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":11067,\"network_reads\":91988,\"decrypted_packets\":91987,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":6934,\"tun_backpressure\":0,\"tun_write_calls\":9974,\"tun_written_packets\":91987,\"tun_coalesced_packets\":90932,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":3,\"io\":{\"receive_calls\":11983,\"send_calls\":4335,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":134905791,\"sent_bytes\":765483,\"receive_would_block\":4098,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":63,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":11067,\"in_place_decryptions\":91987,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":8869,\"network_reads\":82981,\"decrypted_packets\":82980,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":5553,\"tun_backpressure\":0,\"tun_write_calls\":8973,\"tun_written_packets\":82980,\"tun_coalesced_packets\":82222,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":3,\"io\":{\"receive_calls\":9860,\"send_calls\":3516,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":121836060,\"sent_bytes\":595900,\"receive_would_block\":3168,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":32,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":8869,\"in_place_decryptions\":82980,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":10656,\"network_reads\":91957,\"decrypted_packets\":91956,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":2176,\"tun_backpressure\":0,\"tun_write_calls\":5775,\"tun_written_packets\":91956,\"tun_coalesced_packets\":91637,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":3,\"io\":{\"receive_calls\":3073,\"send_calls\":1709,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":134903420,\"sent_bytes\":668150,\"receive_would_block\":457,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":52,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":10656,\"in_place_decryptions\":91956,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":10434,\"network_reads\":91941,\"decrypted_packets\":91940,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":2813,\"tun_backpressure\":0,\"tun_write_calls\":6134,\"tun_written_packets\":91940,\"tun_coalesced_packets\":91536,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":3,\"io\":{\"receive_calls\":4283,\"send_calls\":2115,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":134900365,\"sent_bytes\":662745,\"receive_would_block\":954,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":43,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":10434,\"in_place_decryptions\":91940,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":11235,\"network_reads\":91966,\"decrypted_packets\":91965,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":2519,\"tun_backpressure\":0,\"tun_write_calls\":5865,\"tun_written_packets\":91965,\"tun_coalesced_packets\":91527,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":3,\"io\":{\"receive_calls\":3701,\"send_calls\":1958,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":134902219,\"sent_bytes\":729234,\"receive_would_block\":693,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":32,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":11235,\"in_place_decryptions\":91965,\"parallel_batches\":2986,\"parallel_helper_packets\":44656,\"parallel_owner_packets\":46124,\"parallel_max_batch\":32,\"parallel_control_barriers\":1,\"parallel_owner_cpu\":0,\"parallel_helper_cpu\":1,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":10893,\"network_reads\":91957,\"decrypted_packets\":91956,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":2794,\"tun_backpressure\":0,\"tun_write_calls\":6380,\"tun_written_packets\":91956,\"tun_coalesced_packets\":91589,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":3,\"io\":{\"receive_calls\":4350,\"send_calls\":2138,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":134901355,\"sent_bytes\":688814,\"receive_would_block\":965,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":35,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":10893,\"in_place_decryptions\":91956,\"parallel_batches\":3052,\"parallel_helper_packets\":44518,\"parallel_owner_packets\":45977,\"parallel_max_batch\":32,\"parallel_control_barriers\":1,\"parallel_owner_cpu\":0,\"parallel_helper_cpu\":1,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}",
    "{\"tun_reads\":9648,\"network_reads\":83015,\"decrypted_packets\":83014,\"invalid_packets\":0,\"outbound_filtered\":0,\"inbound_filtered\":0,\"pool_pressure\":0,\"polls\":1869,\"tun_backpressure\":0,\"tun_write_calls\":5049,\"tun_written_packets\":83014,\"tun_coalesced_packets\":82747,\"tun_max_write_batch\":32,\"tun_offload_fallbacks\":0,\"network_backpressure\":3,\"io\":{\"receive_calls\":2639,\"send_calls\":1459,\"received_datagrams\":0,\"sent_datagrams\":0,\"max_receive_batch\":0,\"max_send_batch\":0,\"truncated_datagrams\":0,\"gso_datagrams\":0,\"gro_datagrams\":0,\"kernel_fallbacks\":0,\"enobufs\":0,\"received_bytes\":121842792,\"sent_bytes\":599945,\"receive_would_block\":342,\"send_would_block\":0,\"receive_interrupted\":0,\"send_interrupted\":0},\"to_network_high_water\":63,\"to_tun_high_water\":63,\"pool_reserved_bytes\":405758,\"outer_eof\":0,\"outer_io_errors\":0,\"in_place_encryptions\":9648,\"in_place_decryptions\":83014,\"successful_crypto_copy_bytes\":0,\"pool_recycle_wipe_bytes\":0,\"to_network_bytes_high_water\":0,\"to_tun_bytes_high_water\":0}"
  ],
  "direct_after": {
    "requested_bytes": 100000000,
    "valid": true,
    "tls": "TLSv1.2",
    "status": 206,
    "content_range": "bytes 0-99999999/148648524",
    "bytes": 100000000,
    "sha256": "2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0",
    "seconds": 1.7797878999999739,
    "Mbps": 449.4917624735013
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
  "elapsed_seconds": 179.60899999999992,
  "sources": {
    "main": {
      "sha": "a44d520c337611f59d4c7186971919d57de36df8",
      "archive_sha256": "f7aefe84821ff32c90b7a6db9ad890d68a53791ab533a29e71c1f362ed44fcd5",
      "effective_sources_sha256": "2061361c24eb47c91875b6e1f65179ed574f2387c84bdb5ef162f0512bf352c9"
    },
    "erase": {
      "sha": "543675241b948a6d3b8c656bb39feb7450b9470a",
      "archive_sha256": "48334237837c476b88b672564e8a7528410e7a812489f84968d0d75ac3037f44",
      "effective_sources_sha256": "bb7ed36242d6ff91de7e67dd2c6aa0330edc9a92a72f9e96eac3a3b8970c22b4"
    }
  }
}
```

## Independent postflight

```json
{
  "utc": "2026-11-12T03:56:14.291875+00:00",
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
