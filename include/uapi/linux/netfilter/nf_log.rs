//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter/nf_log.h
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
pub const NF_LOG_TCPSEQ: c_uint = 0x01	/* Log TCP sequence numbers */;
pub const NF_LOG_TCPOPT: c_uint = 0x02	/* Log TCP options */;
pub const NF_LOG_IPOPT: c_uint = 0x04	/* Log IP options */;
pub const NF_LOG_UID: c_uint = 0x08	/* Log UID owning local socket */;
pub const NF_LOG_NFLOG: c_uint = 0x10	/* Unsupported, don't reuse */;
pub const NF_LOG_MACDECODE: c_uint = 0x20	/* Decode MAC header */;
pub const NF_LOG_MASK: c_uint = 0x2f;
pub const NF_LOG_PREFIXLEN: c_int = 128;
