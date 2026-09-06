//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/virtio/virtio_crypto_common.h
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
// Common header for Virtio crypto device.
//
// Copyright 2016 HUAWEI TECHNOLOGIES CO., LTD.
//

// Internal representation of a data virtqueue
#[repr(C)]
#[derive(Copy, Clone)]
pub struct data_queue {
// Virtqueue associated with this send _queue
    pub vq: *mut virtqueue,
// To protect the vq operations for the dataq
    pub lock: spinlock_t,
// Name of the tx queue: dataq.$index
    pub name: [c_char; 32],
    pub engine: *mut crypto_engine,
    pub done_work: work_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto {
    pub vdev: *mut virtio_device,
    pub ctrl_vq: *mut virtqueue,
    pub data_vq: *mut data_queue,
// Work struct for config space updates
    pub config_work: work_struct,
// To protect the vq operations for the controlq
    pub ctrl_lock: spinlock_t,
// Maximum of data queues supported by the device
    pub max_data_queues: u32,
// Number of queue currently used by the driver
    pub curr_queue: u32,
//
// Specifies the services mask which the device support,
// see VIRTIO_CRYPTO_SERVICE_
//
    pub crypto_services: u32,
// Detailed algorithms mask
    pub cipher_algo_l: u32,
    pub cipher_algo_h: u32,
    pub hash_algo: u32,
    pub mac_algo_l: u32,
    pub mac_algo_h: u32,
    pub aead_algo: u32,
    pub akcipher_algo: u32,
// Maximum length of cipher key
    pub max_cipher_key_len: u32,
// Maximum length of authenticated key
    pub max_auth_key_len: u32,
// Maximum size of per request
    pub max_size: u64,
    pub status: c_ulong,
    pub ref_count: core::sync::atomic::AtomicI32,
    pub list: list_head,
    pub owner: *mut module,
    pub dev_id: u8,
// Does the affinity hint is set for virtqueues?
    pub affinity_hint_set: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_sym_session_info {
// Backend session id, which come from the host side
    pub session_id: __u64,
}

//
// Note: there are padding fields in request, clear them to zero before
// sending to host to avoid to divulge any information.
// Ex, virtio_crypto_ctrl_request::ctrl::u::destroy_session::padding[48]
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_ctrl_request {
    pub ctrl: virtio_crypto_op_ctrl_req,
    pub input: virtio_crypto_session_input,
    pub ctrl_status: virtio_crypto_inhdr,
    pub compl: completion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_request {
    pub status: u8,
    pub req_data: *mut virtio_crypto_op_data_req,
    pub sgs: *mut scatterlist,
    pub dataq: *mut data_queue,
    pub alg_cb: virtio_crypto_data_callback,
}

extern "C" {
    pub fn virtcrypto_devmgr_add_dev(vcrypto_dev: *mut virtio_crypto) -> c_int;
}
extern "C" {
    pub fn virtcrypto_devmgr_rm_dev(vcrypto_dev: *mut virtio_crypto);
}
extern "C" {
    pub fn virtcrypto_dev_get(vcrypto_dev: *mut virtio_crypto) -> c_int;
}
extern "C" {
    pub fn virtcrypto_dev_put(vcrypto_dev: *mut virtio_crypto);
}
extern "C" {
    pub fn virtcrypto_dev_started(vcrypto_dev: *mut virtio_crypto) -> c_int;
}
extern "C" {
    pub fn virtcrypto_dev_start(vcrypto: *mut virtio_crypto) -> c_int;
}
extern "C" {
    pub fn virtcrypto_dev_stop(vcrypto: *mut virtio_crypto);
}
extern "C" {
    pub fn virtio_crypto_skcipher_algs_register(vcrypto: *mut virtio_crypto) -> c_int;
}
extern "C" {
    pub fn virtio_crypto_skcipher_algs_unregister(vcrypto: *mut virtio_crypto);
}
extern "C" {
    pub fn virtio_crypto_akcipher_algs_register(vcrypto: *mut virtio_crypto) -> c_int;
}
extern "C" {
    pub fn virtio_crypto_akcipher_algs_unregister(vcrypto: *mut virtio_crypto);
}
