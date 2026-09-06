//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireguard/device.h
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
// Copyright (C) 2015-2019 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct multicore_worker {
    pub ptr: *mut c_void,
    pub work: work_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypt_queue {
    pub ring: ptr_ring,
    pub worker: *mut multicore_worker __percpu,
    pub last_cpu: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prev_queue {
    pub peeked: *mut *mut *mut sk_buff head, tail,,
    pub sk_buff.: *mut *mut *mut { sk_buff next, prev; } empty; // Match first 2 members of struct,
    pub count: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wg_device {
    pub dev: *mut net_device,
    pub handshake_queue: crypt_queue encrypt_queue, decrypt_queue,,
    pub sock6: *mut *mut sock __rcu sock4,,
    pub creating_net: *mut net __rcu,
    pub static_identity: noise_static_identity,
    pub handshake_send_wq: *mut *mut *mut workqueue_packet_crypt_wq,handshake_receive_wq,,
    pub cookie_checker: cookie_checker,
    pub peer_hashtable: *mut pubkey_hashtable,
    pub index_hashtable: *mut index_hashtable,
    pub peer_allowedips: allowedips,
    pub socket_update_lock: mutex device_update_lock,,
    pub peer_list: list_head device_list,,
    pub handshake_queue_len: core::sync::atomic::AtomicI32,
    pub device_update_gen: unsigned int num_peers,,
    pub fwmark: u32,
    pub incoming_port: u16,
}

extern "C" {
    pub fn wg_device_init() -> c_int;
}
extern "C" {
    pub fn wg_device_uninit();
}
