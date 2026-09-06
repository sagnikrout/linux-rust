//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ppp_defs.h
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
// ppp_defs.h - PPP definitions.
//
// Copyright 1994-2000 Paul Mackerras.
//

//
// ppp_proto_is_valid - checks if PPP protocol is valid
// @proto: PPP protocol
//
// Assumes proto is not compressed.
// Protocol is valid if the value is odd and the least significant bit of the
// most significant octet is 0 (see RFC 1661, section 2).
//
// ppp_skb_is_compressed_proto - checks if PPP protocol in a skb is compressed
// @skb: skb to check
//
// Check if the PPP protocol field is compressed (the least significant
// bit of the most significant octet is 1). skb->data must point to the PPP
// protocol header.
//
// Return: Whether the PPP protocol field is compressed.
//
extern "C" {
    pub fn unlikely(0x01: skb->data[0] &) -> return;
}
