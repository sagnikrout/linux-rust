//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/skmsg.h
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
// Copyright (c) 2017 - 2018 Covalent IO, Inc. http://covalent.io

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum __sk_action {
    __SK_DROP = 0,
    __SK_PASS,
    __SK_REDIRECT,
    __SK_NONE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sk_msg_sg {
    pub start: u32,
    pub curr: u32,
    pub end: u32,
    pub size: u32,
    pub copybreak: u32,
    pub 2): DECLARE_BITMAP(copy, MAX_MSG_FRAGS +,
// The extra two elements:
// 1) used for chaining the front and sections when the list becomes
// partitioned (e.g. end < start). The crypto APIs require the
// chaining;
// 2) to chain tailer SG entries after the message.
//
    pub 2]: scatterlist data[MAX_MSG_FRAGS +,
}

// UAPI in filter.c depends on struct sk_msg_sg being first element.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sk_msg {
    pub sg: sk_msg_sg,
    pub data: *mut c_void,
    pub data_end: *mut c_void,
    pub apply_bytes: u32,
    pub cork_bytes: u32,
    pub flags: u32,
    pub skb: *mut sk_buff,
    pub sk_redir: *mut sock,
    pub sk: *mut sock,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sk_psock_progs {
    pub msg_parser: *mut bpf_prog,
    pub stream_parser: *mut bpf_prog,
    pub stream_verdict: *mut bpf_prog,
    pub skb_verdict: *mut bpf_prog,
    pub msg_parser_link: *mut bpf_link,
    pub stream_parser_link: *mut bpf_link,
    pub stream_verdict_link: *mut bpf_link,
    pub skb_verdict_link: *mut bpf_link,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sk_psock_state_bits {
    SK_PSOCK_TX_ENABLED,
    SK_PSOCK_RX_STRP_ENABLED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sk_psock_link {
    pub list: list_head,
    pub map: *mut bpf_map,
    pub link_raw: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sk_psock_work_state {
    pub len: u32,
    pub off: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sk_psock {
    pub sk: *mut sock,
    pub sk_redir: *mut sock,
    pub apply_bytes: u32,
    pub cork_bytes: u32,
    pub eval: u32,
    pub /: *mut *mut bool redir_ingress; / undefined if sk_redir is null,
    pub cork: *mut sk_msg,
    pub progs: sk_psock_progs,

    pub strp: strparser,
    pub copied_seq: u32,
    pub ingress_bytes: u32,

    pub ingress_skb: sk_buff_head,
    pub ingress_msg: list_head,
    pub ingress_lock: spinlock_t,
// @msg_tot_len: Total bytes queued in ingress_msg list.
    pub msg_tot_len: u32,
    pub state: c_ulong,
    pub link: list_head,
    pub link_lock: spinlock_t,
    pub refcnt: refcount_t,
    pub sk): *mut *mut void (saved_unhash)(struct sock,
    pub sk): *mut *mut void (saved_destroy)(struct sock,
    pub timeout): *mut *mut *mut void (saved_close)(struct sock sk, long,
    pub sk): *mut *mut void (saved_write_space)(struct sock,
    pub sk): *mut *mut void (saved_data_ready)(struct sock,
// psock_update_sk_prot may be called with restore=false many times
// so the handler must be safe for this case. It will be called
// exactly once with restore=true when the psock is being destroyed
// and psock refcnt is zero, but before an RCU grace period.
//
    pub restore): bool,
    pub sk_proto: *mut proto,
    pub work_mutex: mutex,
    pub work_state: sk_psock_work_state,
    pub work: delayed_work,
    pub sk_pair: *mut sock,
    pub rwork: rcu_work,
}

extern "C" {
    pub fn sk_msg_trim(sk: *mut sock, msg: *mut sk_msg, len: c_int);
}
extern "C" {
    pub fn sk_msg_free(sk: *mut sock, msg: *mut sk_msg) -> c_int;
}
extern "C" {
    pub fn sk_msg_free_nocharge(sk: *mut sock, msg: *mut sk_msg) -> c_int;
}
extern "C" {
    pub fn sk_msg_free_partial(sk: *mut sock, msg: *mut sk_msg, bytes: u32);
}
extern "C" {
    pub fn sk_msg_return(sk: *mut sock, msg: *mut sk_msg, bytes: c_int);
}
extern "C" {
    pub fn sk_msg_return_zero(sk: *mut sock, msg: *mut sk_msg, bytes: c_int);
}
extern "C" {
    pub fn sk_msg_is_readable(sk: *mut sock) -> bool;
}

extern "C" {
    pub fn sk_msg_iter_dist(_arg: msg->sg.start, _arg: msg->sg.end) -> return;
}
extern "C" {
    pub fn sg_page(_arg: sk_msg_elem(msg, _arg: which)) -> return;
}
extern "C" {
    pub fn test_bit(_arg: bit, _arg: &psock->state) -> return;
}
// Used by ioctl to read msg_tot_len only; lock-free for performance
extern "C" {
    pub fn READ_ONCE(_arg: psock->msg_tot_len) -> return;
}
// Use WRITE_ONCE to ensure correct read in sk_psock_get_msg_len_nolock().
// ingress_lock should be held to prevent concurrent updates to msg_tot_len
//
extern "C" {
    pub fn list_first_entry_or_null(_arg: &psock->ingress_msg, sk_msg: struct, _arg: list) -> return;
}
extern "C" {
    pub fn sk_psock_stop(psock: *mut sk_psock);
}

extern "C" {
    pub fn sk_psock_init_strp(sk: *mut sock, psock: *mut sk_psock) -> c_int;
}
extern "C" {
    pub fn sk_psock_start_strp(sk: *mut sock, psock: *mut sk_psock);
}
extern "C" {
    pub fn sk_psock_stop_strp(sk: *mut sock, psock: *mut sk_psock);
}

extern "C" {
    pub fn sk_psock_start_verdict(sk: *mut sock, psock: *mut sk_psock);
}
extern "C" {
    pub fn sk_psock_stop_verdict(sk: *mut sock, psock: *mut sk_psock);
}
//
// This specialized allocator has to be a macro for its allocations to be
// accounted separately (to have a separate alloc_tag). The typecast is
// intentional to enforce typesafety.
//

extern "C" {
    pub fn sk_psock_drop(sk: *mut sock, psock: *mut sk_psock);
}
// for udp only, sk is not locked

// We only have two bits so far.

