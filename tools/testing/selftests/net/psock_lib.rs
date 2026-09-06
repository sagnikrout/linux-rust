//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/net/psock_lib.h
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
// Copyright 2013 Google Inc.
// Author: Willem de Bruijn <willemb@google.com>
// Daniel Borkmann <dborkman@redhat.com>
//

pub const DATA_LEN: c_int = 100;

pub const PORT_BASE: c_int = 8000;
// the filter below checks for all of the following conditions that
// are based on the contents of create_payload()
// ether type 0x800 and
// ip proto udp     and
// skb->len == DATA_LEN and
// udp[38] == 'a' or udp[38] == 'b'
// It can be generated from the following bpf_asm input:
// ldh [12]
// jne #0x800, drop	; ETH_P_IP
// ldb [23]
// jneq #17, drop		; IPPROTO_UDP
// ld len			; ld skb->len
// jlt #100, drop		; DATA_LEN
// ldb [80]
// jeq #97, pass		; DATA_CHAR
// jne #98, drop		; DATA_CHAR_1
// pass:
// ret #-1
// drop:
// ret #0
//
// must bind both to get consistent hash result
// Should really handle EINTR and EAGAIN
extern "C" {
    pub fn pair_udp_send_char(_arg: fds, _arg: num, _arg: DATA_CHAR) -> return;
}
