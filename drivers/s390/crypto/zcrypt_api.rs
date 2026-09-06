//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/crypto/zcrypt_api.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright IBM Corp. 2001, 2019
// Author(s): Robert Burroughs
// Eric Rossman (edrossma@us.ibm.com)
// Cornelia Huck <cornelia.huck@de.ibm.com>
//
// Hotplug & misc device support: Jochen Roehrig (roehrig@de.ibm.com)
// Major cleanup & driver split: Martin Schwidefsky <schwidefsky@de.ibm.com>
// Ralph Wuerthner <rwuerthn@de.ibm.com>
// MSGTYPE restruct:		  Holger Dengler <hd@linux.vnet.ibm.com>
//

//
// Supported device types
//
pub const ZCRYPT_CEX2C: c_int = 5;
pub const ZCRYPT_CEX2A: c_int = 6;
pub const ZCRYPT_CEX3C: c_int = 7;
pub const ZCRYPT_CEX3A: c_int = 8;
pub const ZCRYPT_CEX4: c_int = 10;
pub const ZCRYPT_CEX5: c_int = 11;
pub const ZCRYPT_CEX6: c_int = 12;
pub const ZCRYPT_CEX7: c_int = 13;
//
// Large random numbers are pulled in 4096 byte chunks from the crypto cards
// and stored in a page. Be careful when increasing this buffer due to size
// limitations for AP requests.
//
pub const ZCRYPT_RNG_BUFFER_SIZE: c_int = 4096;
//
// The zcrypt_wait_api_operational() function waits this
// amount in milliseconds for ap_wait_aqpn_bindings_complete().
// Also on a cprb send failure with ENODEV the send functions
// trigger an ap bus rescan and wait this time in milliseconds
// for ap_wait_aqpn_bindings_complete() before resending.
//
pub const ZCRYPT_WAIT_BINDINGS_COMPLETE_MS: c_int = 30000;
//
// Identifier for Crypto Request Performance Index
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum crypto_ops {
    MEX_1K,
    MEX_2K,
    MEX_4K,
    CRT_1K,
    CRT_2K,
    CRT_4K,
    HWRNG,
    SECKEY,
    NUM_OPS
}

// struct to hold tracking information for a userspace request/response
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zcrypt_track {
    pub /: *mut *mut int again_counter; / retry attempts counter,
    pub /: *mut *mut int last_qid; / last qid used,
    pub /: *mut *mut int last_rc; / last return code,
}

// defines related to message tracking
pub const TRACK_AGAIN_MAX: c_int = 10;
pub const TRACK_AGAIN_CARD_WEIGHT_PENALTY: c_int = 1000;
pub const TRACK_AGAIN_QUEUE_WEIGHT_PENALTY: c_int = 10000;
//
// xflags - to be used with zcrypt_send_cprb() and
// zcrypt_send_ep11_cprb() for the xflags parameter.
//
pub const ZCRYPT_XFLAG_USERSPACE: c_uint = 0x0001	/* data ptrs address userspace */;
pub const ZCRYPT_XFLAG_NOMEMALLOC: c_uint = 0x0002	/* do not allocate memory via kmalloc */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zcrypt_ops {
    pub ): *mut ap_message,
    pub ): *mut ap_message,
    pub ): *mut ap_message,
    pub ): *mut ap_message,
    pub ): *mut *mut *mut *mut long (rng)(struct zcrypt_queue , char , struct ap_message,
    pub /: *mut *mut list_head list; / zcrypt ops list.,
    pub owner: *mut module,
    pub variant: c_int,
    pub name: [c_char; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zcrypt_card {
    pub /: *mut *mut list_head list; / Device list.,
    pub /: *mut *mut list_head zqueues; / List of zcrypt queues,
    pub /: *mut *mut kref refcount; / device refcounting,
    pub /: *mut *mut int online; / User online/offline,
    pub /: *mut *mut *mut ap_card card; / The "real" ap card device.,
    pub /: *mut *mut *mut char type_string; / User space device name.,
    pub /: *mut *mut int user_space_type; / User space device id.,
    pub /: *mut *mut int min_mod_size; / Min number of bits.,
    pub /: *mut *mut int max_mod_size; / Max number of bits.,
    pub max_exp_bit_length: c_int,
    pub /: *const *const *const int speed_rating; / Speed idx of crypto ops.,
    pub /: *mut *mut atomic_t load; / Utilization of the crypto device,
    pub /: *mut *mut int request_count; / # current requests.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zcrypt_queue {
    pub /: *mut *mut list_head list; / Device list.,
    pub /: *mut *mut kref refcount; / device refcounting,
    pub /: *mut *mut int online; / User online/offline,
    pub zcard: *mut zcrypt_card,
    pub /: *mut *mut *mut zcrypt_ops ops; / Crypto operations.,
    pub /: *mut *mut *mut ap_queue queue; / The "real" ap queue device.,
    pub /: *mut *mut ap_message reply; / Per-device reply structure.,
    pub /: *mut *mut atomic_t load; / Utilization of the crypto device,
    pub /: *mut *mut int request_count; / # current requests.,
}

// transport layer rescanning

extern "C" {
    pub fn zcrypt_card_free(: *mut zcrypt_card);
}
extern "C" {
    pub fn zcrypt_card_get(: *mut zcrypt_card);
}
extern "C" {
    pub fn zcrypt_card_put(: *mut zcrypt_card) -> c_int;
}
extern "C" {
    pub fn zcrypt_card_register(: *mut zcrypt_card) -> c_int;
}
extern "C" {
    pub fn zcrypt_card_unregister(: *mut zcrypt_card);
}
extern "C" {
    pub fn zcrypt_queue_free(: *mut zcrypt_queue);
}
extern "C" {
    pub fn zcrypt_queue_get(: *mut zcrypt_queue);
}
extern "C" {
    pub fn zcrypt_queue_put(: *mut zcrypt_queue) -> c_int;
}
extern "C" {
    pub fn zcrypt_queue_register(: *mut zcrypt_queue) -> c_int;
}
extern "C" {
    pub fn zcrypt_queue_unregister(: *mut zcrypt_queue);
}
extern "C" {
    pub fn zcrypt_queue_force_online(zq: *mut zcrypt_queue, online: c_int) -> bool;
}
extern "C" {
    pub fn zcrypt_rng_device_add() -> c_int;
}
extern "C" {
    pub fn zcrypt_rng_device_remove();
}
extern "C" {
    pub fn zcrypt_msgtype_register(: *mut zcrypt_ops);
}
extern "C" {
    pub fn zcrypt_msgtype_unregister(: *mut zcrypt_ops);
}
extern "C" {
    pub fn zcrypt_api_init() -> c_int;
}
extern "C" {
    pub fn zcrypt_api_exit();
}
extern "C" {
    pub fn zcrypt_send_cprb(xcRB: *mut ica_xcRB, xflags: u32) -> c_long;
}
extern "C" {
    pub fn zcrypt_send_ep11_cprb(urb: *mut ep11_urb, xflags: u32) -> c_long;
}
extern "C" {
    pub fn zcrypt_wait_api_operational() -> c_int;
}
extern "C" {
    pub fn copy_from_user(_arg: to, _arg: from, _arg: n) -> return;
}
extern "C" {
    pub fn copy_to_user(_arg: to, _arg: from, _arg: n) -> return;
}
