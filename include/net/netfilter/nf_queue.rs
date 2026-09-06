//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/netfilter/nf_queue.h
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

// Each queued (to userspace) skbuff has one of these.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_queue_entry {
    pub list: list_head,
    pub hash_node: rhash_head,
    pub skb: *mut sk_buff,
    pub skb_dev: *mut net_device,
    pub id: c_uint,
    pub /: *mut *mut unsigned int hook_index; / index in hook_entries->hook[],

    pub bridge_dev: *mut net_device,
    pub physin: *mut net_device,
    pub physout: *mut net_device,

    pub state: nf_hook_state,
    pub nf_ct_is_unconfirmed: bool,
    pub /: *mut *mut u16 size; / sizeof(entry) + saved route keys,
// extra space to store route keys
}

// Packet queuing
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_queue_handler {
    pub queuenum): c_uint,
    pub net): *mut *mut void (nf_hook_drop)(struct net,
}

extern "C" {
    pub fn nf_register_queue_handler(qh: *const nf_queue_handler);
}
extern "C" {
    pub fn nf_unregister_queue_handler();
}
extern "C" {
    pub fn nf_queue_entry_get_refs(entry: *mut nf_queue_entry) -> bool;
}
extern "C" {
    pub fn nf_queue_entry_free(entry: *mut nf_queue_entry);
}
// jhash_initval = get_random_u32();
// packets in either direction go into same queue
extern "C" {
    pub fn jhash_3words(_arg: a, _arg: b, _arg: c, _arg: initval) -> return;
}
extern "C" {
    pub fn hash_v4(_arg: iph, _arg: initval) -> return;
}
extern "C" {
    pub fn hash_v6(_arg: ip6h, _arg: initval) -> return;
}
