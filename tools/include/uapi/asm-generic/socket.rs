//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/uapi/asm-generic/socket.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

// For setsockopt(2)
pub const SOL_SOCKET: c_int = 1;
pub const SO_DEBUG: c_int = 1;
pub const SO_REUSEADDR: c_int = 2;
pub const SO_TYPE: c_int = 3;
pub const SO_ERROR: c_int = 4;
pub const SO_DONTROUTE: c_int = 5;
pub const SO_BROADCAST: c_int = 6;
pub const SO_SNDBUF: c_int = 7;
pub const SO_RCVBUF: c_int = 8;
pub const SO_SNDBUFFORCE: c_int = 32;
pub const SO_RCVBUFFORCE: c_int = 33;
pub const SO_KEEPALIVE: c_int = 9;
pub const SO_OOBINLINE: c_int = 10;
pub const SO_NO_CHECK: c_int = 11;
pub const SO_PRIORITY: c_int = 12;
pub const SO_LINGER: c_int = 13;
pub const SO_BSDCOMPAT: c_int = 14;
pub const SO_REUSEPORT: c_int = 15;

pub const SO_PASSCRED: c_int = 16;
pub const SO_PEERCRED: c_int = 17;
pub const SO_RCVLOWAT: c_int = 18;
pub const SO_SNDLOWAT: c_int = 19;
pub const SO_RCVTIMEO_OLD: c_int = 20;
pub const SO_SNDTIMEO_OLD: c_int = 21;

// Security levels - as per NRL IPv6 - don't actually do anything
pub const SO_SECURITY_AUTHENTICATION: c_int = 22;
pub const SO_SECURITY_ENCRYPTION_TRANSPORT: c_int = 23;
pub const SO_SECURITY_ENCRYPTION_NETWORK: c_int = 24;
pub const SO_BINDTODEVICE: c_int = 25;
// Socket filtering
pub const SO_ATTACH_FILTER: c_int = 26;
pub const SO_DETACH_FILTER: c_int = 27;

pub const SO_PEERNAME: c_int = 28;
pub const SO_ACCEPTCONN: c_int = 30;
pub const SO_PEERSEC: c_int = 31;
pub const SO_PASSSEC: c_int = 34;
pub const SO_MARK: c_int = 36;
pub const SO_PROTOCOL: c_int = 38;
pub const SO_DOMAIN: c_int = 39;
pub const SO_RXQ_OVFL: c_int = 40;
pub const SO_WIFI_STATUS: c_int = 41;

pub const SO_PEEK_OFF: c_int = 42;
// Instruct lower device to use last 4-bytes of skb data as FCS
pub const SO_NOFCS: c_int = 43;
pub const SO_LOCK_FILTER: c_int = 44;
pub const SO_SELECT_ERR_QUEUE: c_int = 45;
pub const SO_BUSY_POLL: c_int = 46;
pub const SO_MAX_PACING_RATE: c_int = 47;
pub const SO_BPF_EXTENSIONS: c_int = 48;
pub const SO_INCOMING_CPU: c_int = 49;
pub const SO_ATTACH_BPF: c_int = 50;

pub const SO_ATTACH_REUSEPORT_CBPF: c_int = 51;
pub const SO_ATTACH_REUSEPORT_EBPF: c_int = 52;
pub const SO_CNX_ADVICE: c_int = 53;
pub const SCM_TIMESTAMPING_OPT_STATS: c_int = 54;
pub const SO_MEMINFO: c_int = 55;
pub const SO_INCOMING_NAPI_ID: c_int = 56;
pub const SO_COOKIE: c_int = 57;
pub const SCM_TIMESTAMPING_PKTINFO: c_int = 58;
pub const SO_PEERGROUPS: c_int = 59;
pub const SO_ZEROCOPY: c_int = 60;
pub const SO_TXTIME: c_int = 61;

pub const SO_BINDTOIFINDEX: c_int = 62;
pub const SO_TIMESTAMP_OLD: c_int = 29;
pub const SO_TIMESTAMPNS_OLD: c_int = 35;
pub const SO_TIMESTAMPING_OLD: c_int = 37;
pub const SO_TIMESTAMP_NEW: c_int = 63;
pub const SO_TIMESTAMPNS_NEW: c_int = 64;
pub const SO_TIMESTAMPING_NEW: c_int = 65;
pub const SO_RCVTIMEO_NEW: c_int = 66;
pub const SO_SNDTIMEO_NEW: c_int = 67;
pub const SO_DETACH_REUSEPORT_BPF: c_int = 68;
pub const SO_PREFER_BUSY_POLL: c_int = 69;
pub const SO_BUSY_POLL_BUDGET: c_int = 70;
pub const SO_NETNS_COOKIE: c_int = 71;
pub const SO_BUF_LOCK: c_int = 72;
pub const SO_RESERVE_MEM: c_int = 73;
pub const SO_TXREHASH: c_int = 74;
pub const SO_RCVMARK: c_int = 75;
pub const SO_PASSPIDFD: c_int = 76;
pub const SO_PEERPIDFD: c_int = 77;
pub const SO_DEVMEM_LINEAR: c_int = 78;

pub const SO_DEVMEM_DMABUF: c_int = 79;

pub const SO_DEVMEM_DONTNEED: c_int = 80;
pub const SCM_TS_OPT_ID: c_int = 81;
pub const SO_RCVPRIORITY: c_int = 82;
pub const SO_PASSRIGHTS: c_int = 83;

// on 64-bit and x32, avoid the ?: operator

