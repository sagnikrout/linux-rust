//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/progs/test_tcp_custom_syncookie.h
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
// Copyright Amazon.com Inc. or its affiliates.

// Macro flag: #define 

// linux/unaligned.h

extern "C" {
    pub fn bpf_ntohs(_arg: __get_unaligned_t(__be16, _arg: p)) -> return;
}
extern "C" {
    pub fn bpf_ntohl(_arg: __get_unaligned_t(__be32, _arg: p)) -> return;
}
// lib/checksum.c
// add up 32-bit and 32-bit for 32+c bit
// add up carry..

// asm-generic/checksum.h
extern "C" {
    pub fn csum_fold(_arg: csum_tcpudp_nofold(saddr, _arg: daddr, _arg: len, _arg: proto, _arg: sum)) -> return;
}
// net/ipv6/ip6_checksum.c
extern "C" {
    pub fn csum_fold(__wsum)sum: () -> return;
}
