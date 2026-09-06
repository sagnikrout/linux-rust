//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/progs/test_sockmap_kern.h
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
// Copyright (c) 2017-2018 Covalent IO, Inc. http://covalent.io

// Sockmap sample program connects a client and a backend together
// using cgroups.
//
// client:X <---> frontend:80 client:X <---> backend:80
//
// For simplicity we hard code values here and bind 1:1. The hard
// coded values are part of the setup in sockmap.sh script that
// is associated with this BPF program.
//
// The bpf_printk is verbose and prints information as connections
// are established and verdicts are decided.
//

extern "C" {
    pub fn bpf_sk_redirect_map(_arg: skb, _arg: &sock_map, _arg: ret, _arg: flags) -> return;
}

extern "C" {
    pub fn bpf_sk_redirect_hash(_arg: skb, _arg: &sock_map, _arg: &ret, _arg: flags) -> return;
}

extern "C" {
    pub fn bpf_msg_redirect_map(_arg: msg, _arg: &sock_map_redir, _arg: key, _arg: flags) -> return;
}

extern "C" {
    pub fn bpf_msg_redirect_hash(_arg: msg, _arg: &sock_map_redir, _arg: &key, _arg: flags) -> return;
}
