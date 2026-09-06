//! Automatically rewritten from C Header to Rust Module
//! Source: security/landlock/errata/abi-4.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// DOC: erratum_1
//
// Erratum 1: TCP socket identification
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// This fix addresses an issue where IPv4 and IPv6 stream sockets (e.g., SMC,
// MPTCP, or SCTP) were incorrectly restricted by TCP access rights during
// :manpage:`bind(2)` and :manpage:`connect(2)` operations. This change ensures
// that only TCP sockets are subject to TCP access rights, allowing other
// protocols to operate without unnecessary restrictions.
//
// Impact:
//
// In kernels without this fix, using ``LANDLOCK_ACCESS_NET_BIND_TCP`` or
// ``LANDLOCK_ACCESS_NET_CONNECT_TCP`` would incorrectly restrict non-TCP
// stream protocols (SMC, MPTCP, SCTP), potentially breaking applications
// that rely on these protocols while using Landlock network restrictions.
//
