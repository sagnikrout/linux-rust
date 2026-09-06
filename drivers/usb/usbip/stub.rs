//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/usbip/stub.h
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
// Copyright (C) 2003-2008 Takahiro Hirofuchi
//

pub const STUB_BUSID_OTHER: c_int = 0;
pub const STUB_BUSID_REMOV: c_int = 1;
pub const STUB_BUSID_ADDED: c_int = 2;
pub const STUB_BUSID_ALLOC: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stub_device {
    pub udev: *mut usb_device,
    pub ud: usbip_device,
    pub devid: __u32,
//
// stub_priv preserves private data of each urb.
// It is allocated as stub_priv_cache and assigned to urb->context.
//
// stub_priv is always linked to any one of 3 lists;
// priv_init: linked to this until the comletion of a urb.
// priv_tx  : linked to this after the completion of a urb.
// priv_free: linked to this after the sending of the result.
//
// Any of these list operations should be locked by priv_lock.
//
    pub priv_lock: spinlock_t,
    pub priv_init: list_head,
    pub priv_tx: list_head,
    pub priv_free: list_head,
// see comments for unlinking in stub_rx.c
    pub unlink_tx: list_head,
    pub unlink_free: list_head,
    pub tx_waitq: wait_queue_head_t,
}

// private data into urb->priv
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stub_priv {
    pub seqnum: c_ulong,
    pub list: list_head,
    pub sdev: *mut stub_device,
    pub urbs: *mut urb,
    pub sgl: *mut scatterlist,
    pub num_urbs: c_int,
    pub completed_urbs: c_int,
    pub urb_status: c_int,
    pub unlinking: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stub_unlink {
    pub seqnum: c_ulong,
    pub list: list_head,
    pub status: __u32,
}

// same as SYSFS_BUS_ID_SIZE
pub const BUSID_SIZE: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bus_id_priv {
    pub name: [c_char; BUSID_SIZE],
    pub status: c_char,
    pub interf_count: c_int,
    pub sdev: *mut stub_device,
    pub udev: *mut usb_device,
    pub shutdown_busid: c_char,
    pub busid_lock: spinlock_t,
}

// stub_priv is allocated from stub_priv_cache
// stub_dev.c
// stub_main.c
extern "C" {
    pub fn put_busid_priv(bid: *mut bus_id_priv);
}
extern "C" {
    pub fn del_match_busid(busid: *mut c_char) -> c_int;
}
extern "C" {
    pub fn stub_free_priv_and_urb(priv: *mut stub_priv);
}
extern "C" {
    pub fn stub_device_cleanup_urbs(sdev: *mut stub_device);
}
// stub_rx.c
extern "C" {
    pub fn stub_rx_loop(data: *mut c_void) -> c_int;
}
// stub_tx.c
extern "C" {
    pub fn stub_complete(urb: *mut urb);
}
extern "C" {
    pub fn stub_tx_loop(data: *mut c_void) -> c_int;
}
