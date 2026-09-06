//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/progs/bpf_tracing_net.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)

pub const AF_INET: c_int = 2;
pub const AF_INET6: c_int = 10;
// include/linux/net.h
pub const SOCK_TYPE_MASK: c_uint = 0xf;
pub const SOL_SOCKET: c_int = 1;
pub const SO_REUSEADDR: c_int = 2;
pub const SO_SNDBUF: c_int = 7;
pub const SO_RCVBUF: c_int = 8;
pub const SO_KEEPALIVE: c_int = 9;
pub const SO_PRIORITY: c_int = 12;
pub const SO_REUSEPORT: c_int = 15;

pub const SO_RCVLOWAT: c_int = 16;

pub const SO_RCVLOWAT: c_int = 18;

pub const SO_BINDTODEVICE: c_int = 25;
pub const SO_MARK: c_int = 36;
pub const SO_MAX_PACING_RATE: c_int = 47;
pub const SO_BINDTOIFINDEX: c_int = 62;
pub const SO_TXREHASH: c_int = 74;

pub const IP_TOS: c_int = 1;
pub const IP_TRANSPARENT: c_int = 19;
pub const SOL_IPV6: c_int = 41;
pub const IPV6_TCLASS: c_int = 67;
pub const IPV6_AUTOFLOWLABEL: c_int = 70;
pub const IPV6_TRANSPARENT: c_int = 75;

pub const TC_ACT_OK: c_int = 0;
pub const TC_ACT_SHOT: c_int = 2;
pub const SOL_TCP: c_int = 6;
pub const TCP_NODELAY: c_int = 1;
pub const TCP_MAXSEG: c_int = 2;
pub const TCP_KEEPIDLE: c_int = 4;
pub const TCP_KEEPINTVL: c_int = 5;
pub const TCP_KEEPCNT: c_int = 6;
pub const TCP_SYNCNT: c_int = 7;
pub const TCP_WINDOW_CLAMP: c_int = 10;
pub const TCP_CONGESTION: c_int = 13;
pub const TCP_THIN_LINEAR_TIMEOUTS: c_int = 16;
pub const TCP_USER_TIMEOUT: c_int = 18;
pub const TCP_NOTSENT_LOWAT: c_int = 25;
pub const TCP_SAVE_SYN: c_int = 27;
pub const TCP_SAVED_SYN: c_int = 28;
pub const TCP_CA_NAME_MAX: c_int = 16;
pub const TCP_NAGLE_OFF: c_int = 1;
pub const TCP_RTO_MAX_MS: c_int = 44;
pub const TCP_ECN_OK: c_int = 1;
pub const TCP_ECN_QUEUE_CWR: c_int = 2;
pub const TCP_ECN_DEMAND_CWR: c_int = 4;
pub const TCP_ECN_SEEN: c_int = 8;
pub const TCP_CONG_NEEDS_ECN: c_uint = 0x2;
pub const ICSK_TIME_RETRANS: c_int = 1;
pub const ICSK_TIME_PROBE0: c_int = 3;
pub const ICSK_TIME_LOSS_PROBE: c_int = 5;
pub const ICSK_TIME_REO_TIMEOUT: c_int = 6;
pub const ETH_ALEN: c_int = 6;
pub const ETH_HLEN: c_int = 14;
pub const ETH_P_IP: c_uint = 0x0800;
pub const ETH_P_IPV6: c_uint = 0x86DD;
pub const NEXTHDR_TCP: c_int = 6;
pub const TCPOPT_NOP: c_int = 1;
pub const TCPOPT_EOL: c_int = 0;
pub const TCPOPT_MSS: c_int = 2;
pub const TCPOPT_WINDOW: c_int = 3;
pub const TCPOPT_TIMESTAMP: c_int = 8;
pub const TCPOPT_SACK_PERM: c_int = 4;
pub const TCPOLEN_MSS: c_int = 4;
pub const TCPOLEN_WINDOW: c_int = 3;
pub const TCPOLEN_TIMESTAMP: c_int = 10;
pub const TCPOLEN_SACK_PERM: c_int = 2;
pub const CHECKSUM_NONE: c_int = 0;
pub const CHECKSUM_PARTIAL: c_int = 3;
pub const IFNAMSIZ: c_int = 16;
pub const RTF_GATEWAY: c_uint = 0x0002;
pub const TCP_INFINITE_SSTHRESH: c_uint = 0x7fffffff;
pub const TCP_PINGPONG_THRESH: c_int = 3;
pub const FLAG_DATA_ACKED: c_uint = 0x04 /* This ACK acknowledged new data.		*/;
pub const FLAG_SYN_ACKED: c_uint = 0x10 /* This ACK acknowledged SYN.		*/;
pub const FLAG_DATA_SACKED: c_uint = 0x20 /* New SACK.				*/;

// If in slow start, ensure cwnd grows to twice what was ACKed.
