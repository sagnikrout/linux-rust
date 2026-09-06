//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/ptp/ptp_private.h
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
//
// PTP 1588 clock support - private declarations for the core module.
//
// Copyright (C) 2010 OMICRON electronics GmbH
//

pub const PTP_MAX_TIMESTAMPS: c_int = 128;
pub const PTP_BUF_TIMESTAMPS: c_int = 30;
pub const PTP_DEFAULT_MAX_VCLOCKS: c_int = 20;

pub const PTP_MAX_CHANNELS: c_int = 2048;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timestamp_event_queue {
    pub buf: [ptp_extts_event; PTP_MAX_TIMESTAMPS],
    pub head: c_int,
    pub tail: c_int,
    pub lock: spinlock_t,
    pub qlist: list_head,
    pub mask: *mut c_ulong,
    pub debugfs_instance: *mut dentry,
    pub dfs_bitmap: debugfs_u32_array,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptp_clock {
    pub clock: posix_clock,
    pub dev: device,
    pub info: *mut ptp_clock_info,
    pub devid: dev_t,
    pub /: *mut *mut int index; / index into clocks.map,
    pub pps_source: *mut pps_device,
    pub /: *mut *mut long dialed_frequency; / remembers the frequency adjustment,
    pub /: *mut *mut list_head tsevqs; / timestamp fifo list,
    pub /: *mut *mut spinlock_t tsevqs_lock; / protects tsevqs from concurrent access,
    pub /: *mut *mut mutex pincfg_mux; / protect concurrent info->pin_config access,
    pub tsev_wq: wait_queue_head_t,
    pub /: *mut *mut int defunct; / tells readers to go away when clock is being removed,
    pub pin_dev_attr: *mut device_attribute,
    pub pin_attr: *mut attribute,
    pub pin_attr_group: attribute_group,
// 1st entry is a pointer to the real group, 2nd is NULL terminator
    pub pin_attr_groups: [*const attribute_group; 2],
    pub kworker: *mut kthread_worker,
    pub aux_work: kthread_delayed_work,
    pub max_vclocks: c_uint,
    pub n_vclocks: c_uint,
    pub vclock_index: *mut c_int,
    pub /: *mut *mut mutex n_vclocks_mux; / protect concurrent n_vclocks access,
    pub is_virtual_clock: bool,
    pub has_cycles: bool,
    pub debugfs_root: *mut dentry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptp_vclock {
    pub pclock: *mut ptp_clock,
    pub info: ptp_clock_info,
    pub clock: *mut ptp_clock,
    pub vclock_hash_node: hlist_node,
    pub cc: cyclecounter,
    pub tc: timecounter,
    pub /: *mut *mut mutex lock; / protects tc/cc,
}

//
// The function queue_cnt() is safe for readers to call without
// holding q->lock. Readers use this function to verify that the queue
// is nonempty before proceeding with a dequeue operation. The fact
// that a writer might concurrently increment the tail does not
// matter, since the queue remains nonempty nonetheless.
//
// Paired with WRITE_ONCE() in enqueue_external_timestamp(),
// ptp_read(), extts_fifo_show().
//
// Check if ptp virtual clock is in use
// Virtual clocks can't be stacked on top of virtual clocks.
// Avoid acquiring the n_vclocks_mux on virtual clocks, to allow this
// function to be called from code paths where the n_vclocks_mux of the
// parent physical clock is already held. Functionally that's not an
// issue, but lockdep would complain, because they have the same lock
// class.
//
// Check if ptp clock shall be free running
extern "C" {
    pub fn ptp_vclock_in_use(_arg: ptp) -> return;
}
//
// see ptp_chardev.c
//
extern "C" {
    pub fn ptp_disable_all_events(ptp: *mut ptp_clock);
}
// caller must hold pincfg_mux
extern "C" {
    pub fn ptp_open(pccontext: *mut posix_clock_context, fmode: fmode_t) -> c_int;
}
extern "C" {
    pub fn ptp_release(pccontext: *mut posix_clock_context) -> c_int;
}
//
// see ptp_sysfs.c
//
extern "C" {
    pub fn ptp_populate_pin_groups(ptp: *mut ptp_clock) -> c_int;
}
extern "C" {
    pub fn ptp_cleanup_pin_groups(ptp: *mut ptp_clock);
}
extern "C" {
    pub fn ptp_vclock_unregister(vclock: *mut ptp_vclock);
}
