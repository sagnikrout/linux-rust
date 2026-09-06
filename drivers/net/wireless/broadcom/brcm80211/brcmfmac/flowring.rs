//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmfmac/flowring.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2014 Broadcom Corporation
//

pub const BRCMF_FLOWRING_INVALID_ID: c_uint = 0xFFFFFFFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_flowring_hash {
    pub mac: [u8; ETH_ALEN],
    pub fifo: u8,
    pub ifidx: u8,
    pub flowid: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ring_status {
    RING_CLOSED,
    RING_CLOSING,
    RING_OPEN
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_flowring_ring {
    pub hash_id: u16,
    pub blocked: bool,
    pub status: ring_status,
    pub skblist: sk_buff_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_flowring_tdls_entry {
    pub mac: [u8; ETH_ALEN],
    pub next: *mut brcmf_flowring_tdls_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_flowring {
    pub dev: *mut device,
    pub hash: [brcmf_flowring_hash; BRCMF_FLOWRING_HASHSIZE],
    pub block_lock: spinlock_t,
    pub addr_mode: [proto_addr_mode; BRCMF_MAX_IFS],
    pub nrofrings: u16,
    pub tdls_active: bool,
    pub tdls_entry: *mut brcmf_flowring_tdls_entry,
    pub __counted_by(nrofrings): *mut *mut brcmf_flowring_ring rings[],
}

extern "C" {
    pub fn brcmf_flowring_delete(flow: *mut brcmf_flowring, flowid: u16);
}
extern "C" {
    pub fn brcmf_flowring_open(flow: *mut brcmf_flowring, flowid: u16);
}
extern "C" {
    pub fn brcmf_flowring_tid(flow: *mut brcmf_flowring, flowid: u16) -> u8;
}
extern "C" {
    pub fn brcmf_flowring_qlen(flow: *mut brcmf_flowring, flowid: u16) -> u32;
}
extern "C" {
    pub fn brcmf_flowring_ifidx_get(flow: *mut brcmf_flowring, flowid: u16) -> u8;
}
extern "C" {
    pub fn brcmf_flowring_detach(flow: *mut brcmf_flowring);
}
