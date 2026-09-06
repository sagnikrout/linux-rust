//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sunrpc/xdrgen/_builtins.h
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
//
// Copyright (c) 2024 Oracle and/or its affiliates.
//
// This header defines XDR data type primitives specified in
// Section 4 of RFC 4506, used by RPC programs implemented
// in the Linux kernel.
//

// ptr = (*p != xdr_zero);
// p = val ? xdr_one : xdr_zero;
//
// De facto (non-standard but commonly implemented) signed short type:
// - Wire sends sign-extended 32-bit value (e.g., 0xFFFFFFFF)
// - be32_to_cpup() returns u32 (0xFFFFFFFF)
// - Explicit (s16) cast truncates to 16 bits (0xFFFF = -1)
//
// ptr = (s16)be32_to_cpup(p);
//
// De facto (non-standard but commonly implemented) signed short type:
// - C integer promotion sign-extends s16 val to int before passing to
// cpu_to_be32()
// - This is well-defined: -1 as s16 -1 as int 0xFFFFFFFF on wire
//
// p = cpu_to_be32(val);
//
// De facto (non-standard but commonly implemented) unsigned short type:
// 16-bit integer zero-extended to fill one XDR_UNIT.
//
// ptr = (u16)be32_to_cpup(p);
// p = cpu_to_be32(val);
// ptr = be32_to_cpup(p);
// p = cpu_to_be32(val);
// ptr = be32_to_cpup(p);
// p = cpu_to_be32(val);
// ptr = be32_to_cpup(p);
// p = cpu_to_be32(val);
// ptr = be32_to_cpup(p);
// p = cpu_to_be32(val);
// ptr = get_unaligned_be64(p);
//
// xdrgen_svc_decode_void - Decode a void argument
// @rqstp: RPC transaction context
// @xdr: source XDR data stream
//
// Return values:
// %true: procedure arguments decoded successfully
// %false: decode failed
//
extern "C" {
    pub fn xdrgen_decode_void(_arg: xdr) -> return;
}
//
// xdrgen_svc_encode_void - Encode a void result
// @rqstp: RPC transaction context
// @xdr: target XDR data stream
//
// Return values:
// %true: procedure results encoded successfully
// %false: encode failed
//
extern "C" {
    pub fn xdrgen_encode_void(_arg: xdr) -> return;
}
