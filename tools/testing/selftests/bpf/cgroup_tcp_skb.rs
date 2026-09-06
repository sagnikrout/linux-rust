//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/cgroup_tcp_skb.h
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


// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2023 Meta Platforms, Inc. and affiliates.
// Define states of a socket to tracking messages sending to and from the
// socket.
//
// These states are based on rfc9293 with some modifications to support
// tracking of messages sent out from a socket. For example, when a SYN is
// received, a new socket is transiting to the SYN_RECV state defined in
// rfc9293. But, we put it in SYN_RECV_SENDING_SYN_ACK state and when
// SYN-ACK is sent out, it moves to SYN_RECV state. With this modification,
// we can track the message sent out from a socket.
//
