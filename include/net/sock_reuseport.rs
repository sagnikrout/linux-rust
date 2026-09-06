//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/sock_reuseport.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sock_reuseport {
    pub rcu: rcu_head,
    pub /: *mut *mut u16 max_socks; / length of socks,
    pub /: *mut *mut u16 num_socks; / elements in socks,
    pub /: *mut *mut u16 num_closed_socks; / closed elements in socks,
    pub incoming_cpu: u16,
// The last synq overflow event timestamp of this
// reuse->socks[] group.
//
    pub synq_overflow_ts: c_uint,
// ID stays the same even after the size of socks[] grows.
    pub reuseport_id: c_uint,
    pub bind_inany:1: c_uint,
    pub has_conns:1: c_uint,
    pub /: *mut *mut *mut bpf_prog __rcu prog; / optional BPF sock selector,
    pub __counted_by(max_socks): *mut *mut sock socks[],
}

extern "C" {
    pub fn reuseport_alloc(sk: *mut sock, bind_inany: bool) -> c_int;
}
extern "C" {
    pub fn reuseport_detach_sock(sk: *mut sock);
}
extern "C" {
    pub fn reuseport_stop_listen_sock(sk: *mut sock);
}
extern "C" {
    pub fn reuseport_attach_prog(sk: *mut sock, prog: *mut bpf_prog) -> c_int;
}
extern "C" {
    pub fn reuseport_detach_prog(sk: *mut sock) -> c_int;
}
extern "C" {
    pub fn reuseport_has_conns_set(sk: *mut sock);
}
extern "C" {
    pub fn reuseport_update_incoming_cpu(sk: *mut sock, val: c_int);
}
