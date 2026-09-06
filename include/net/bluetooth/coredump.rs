//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/bluetooth/coredump.h
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
//
// Copyright (C) 2022 Google Corporation
//

//
// Max header size, shared by both the devcoredump core and
// the dmp_hdr() registered by driver via hci_devcd_register()
//
pub const HCI_DEVCD_HDR_SIZE_MAX: c_int = 512;

extern "C" {
    pub fn void(hdev: *mut *mut coredump_t)(struct hci_dev) -> typedef;
}
extern "C" {
    pub fn void(hdev: *mut *mut dmp_hdr_t)(struct hci_dev, skb: *mut sk_buff) -> typedef;
}
extern "C" {
    pub fn void(hdev: *mut *mut notify_change_t)(struct hci_dev, state: c_int) -> typedef;
}
// struct hci_devcoredump - Devcoredump state
//
// @supported: Indicates if FW dump collection is supported by driver
// @state: Current state of dump collection
// @timeout: Indicates a timeout for collecting the devcoredump
//
// @alloc_size: Total size of the dump
// @head: Start of the dump
// @tail: Pointer to current end of dump
// @end: head + alloc_size for easy comparisons
//
// @dump_q: Dump queue for state machine to process
// @dump_rx: Devcoredump state machine work
// @dump_timeout: Devcoredump timeout work
//
// @coredump: Called from the driver's .coredump() function.
// @dmp_hdr: Create a dump header to identify controller/fw/driver info
// @notify_change: Notify driver when devcoredump state has changed
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_devcoredump {
    pub supported: bool,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum devcoredump_state {
    HCI_DEVCOREDUMP_IDLE,
    HCI_DEVCOREDUMP_ACTIVE,
    HCI_DEVCOREDUMP_DONE,
    HCI_DEVCOREDUMP_ABORT,
    HCI_DEVCOREDUMP_TIMEOUT,
    } state;

    unsigned long	timeout;

    size_t		alloc_size;
    char		*head;
    char		*tail;
    char		*end;

    struct sk_buff_head	dump_q;
    struct work_struct	dump_rx;
    struct delayed_work	dump_timeout;

    coredump_t		coredump;
    dmp_hdr_t		dmp_hdr;
    notify_change_t		notify_change;
}

    pub state): *const *const char hci_devcd_state_name(enum devcoredump_state,
    pub hdev): *mut void hci_devcd_reset(struct hci_dev,
    pub work): *mut void hci_devcd_rx(struct work_struct,
    pub work): *mut void hci_devcd_timeout(struct work_struct,
    pub notify_change): dmp_hdr_t dmp_hdr, notify_change_t,
    pub dump_size): *mut *mut int hci_devcd_init(struct hci_dev hdev, u32,
    pub skb): *mut *mut int hci_devcd_append(struct hci_dev hdev, struct sk_buff,
    pub len): *mut *mut int hci_devcd_append_pattern(struct hci_dev hdev, u8 pattern, u32,
    pub hdev): *mut int hci_devcd_complete(struct hci_dev,
    pub hdev): *mut int hci_devcd_abort(struct hci_dev,

    pub "": return,
    pub -EOPNOTSUPP: return,
    pub -EOPNOTSUPP: return,
    pub -EOPNOTSUPP: return,
    pub -EOPNOTSUPP: return,
    pub -EOPNOTSUPP: return,
    pub -EOPNOTSUPP: return,

