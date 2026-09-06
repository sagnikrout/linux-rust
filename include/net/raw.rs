//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/raw.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// INET		An implementation of the TCP/IP protocol suite for the LINUX
// operating system.  INET is implemented using the  BSD Socket
// interface as the means of communication with the user level.
//
// Definitions for the RAW-IP module.
//
// Version:	@(#)raw.h	1.0.2	05/07/93
//
// Author:	Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
//

extern "C" {
    pub fn raw_abort(sk: *mut sock, err: c_int) -> c_int;
}
extern "C" {
    pub fn raw_icmp_error(: *mut sk_buff, _arg: c_int, _arg: u32);
}
extern "C" {
    pub fn raw_local_deliver(: *mut sk_buff, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn raw_rcv(: *mut sock, : *mut sk_buff) -> c_int;
}
pub const RAW_HTABLE_LOG: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_hashinfo {
    pub lock: spinlock_t,
    pub ____cacheline_aligned: hlist_head ht[RAW_HTABLE_SIZE],
}

extern "C" {
    pub fn hash_32(proto: net_hash_mix(net) ^, _arg: RAW_HTABLE_LOG) -> return;
}

extern "C" {
    pub fn raw_proc_init() -> c_int;
}
extern "C" {
    pub fn raw_proc_exit();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_iter_state {
    pub p: seq_net_private,
    pub bucket: c_int,
}

extern "C" {
    pub fn raw_seq_stop(seq: *mut seq_file, v: *mut c_void);
}

extern "C" {
    pub fn raw_hash_sk(sk: *mut sock) -> c_int;
}
extern "C" {
    pub fn raw_unhash_sk(sk: *mut sock);
}
extern "C" {
    pub fn raw_init();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_sock {
// inet_sock has to be the first member
    pub inet: inet_sock,
    pub filter: icmp_filter,
    pub ipmr_table: u32,
    pub drop_counters: numa_drop_counters,
}

extern "C" {
    pub fn inet_bound_dev_eq(_arg: true, _arg: bound_dev_if, _arg: dif, _arg: sdif) -> return;
}

