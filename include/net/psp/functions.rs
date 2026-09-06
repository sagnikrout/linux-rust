//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/psp/functions.h
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

// Driver-facing API
extern "C" {
    pub fn psp_dev_unregister(psd: *mut psp_dev);
}
extern "C" {
    pub fn psp_dev_rcv(skb: *mut sk_buff, dev_id: u16, generation: u8, strip_icv: bool) -> c_int;
}
// Kernel-facing API
extern "C" {
    pub fn psp_assoc_put(pas: *mut psp_assoc);
}

extern "C" {
    pub fn psp_key_size(version: u32) -> c_uint;
}
extern "C" {
    pub fn psp_sk_assoc_free(sk: *mut sock);
}
extern "C" {
    pub fn psp_twsk_init(tw: *mut inet_timewait_sock, sk: *const sock);
}
extern "C" {
    pub fn psp_twsk_assoc_free(tw: *mut inet_timewait_sock);
}
extern "C" {
    pub fn psp_reply_set_decrypted(sk: *const sock, skb: *mut sk_buff);
}
extern "C" {
    pub fn rcu_dereference_check(_arg: sk->psp_assoc, _arg: lockdep_sock_is_held(sk)) -> return;
}
extern "C" {
    pub fn __psp_sk_rx_policy_check(_arg: skb, _arg: psp_sk_assoc(sk)) -> return;
}
extern "C" {
    pub fn __psp_sk_rx_policy_check(_arg: skb, _arg: rcu_dereference(tw->psp_assoc)) -> return;
}
extern "C" {
    pub fn psp_sk_get_assoc_rcu(_arg: skb->sk) -> return;
}

extern "C" {
    pub fn __psp_skb_coalesce_diff(_arg: one, _arg: two, _arg: 0) -> return;
}
