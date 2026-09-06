//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-core.h
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
// Internal header file _only_ for device mapper core
//
// Copyright (C) 2016 Red Hat, Inc. All rights reserved.
//
// This file is released under the LGPL.
//

pub const DM_RESERVED_MAX_IOS: c_int = 1024;
pub const DM_MAX_TARGETS: c_int = 1048576;
pub const DM_MAX_TARGET_PARAMS: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_kobject_holder {
    pub kobj: kobject,
    pub completion: completion,
}

//
// DM core internal structures used directly by dm.c, dm-rq.c and dm-table.c.
// DM targets must _not_ deference a mapped_device or dm_table to directly
// access their members!
//
// For mempools pre-allocation at the table loading time.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_md_mempools {
    pub bs: bio_set,
    pub io_bs: bio_set,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mapped_device {
    pub suspend_lock: mutex,
    pub table_devices_lock: mutex,
    pub table_devices: list_head,
//
// The current mapping (struct dm_table *).
// Use dm_get_live_table{_fast} or take suspend_lock for
// dereference.
//
    pub map: *mut void __rcu,
    pub flags: c_ulong,
// Protect queue and type against concurrent access.
    pub type_lock: mutex,
    pub type: dm_queue_mode,
    pub numa_node_id: c_int,
    pub queue: *mut request_queue,
    pub holders: core::sync::atomic::AtomicI32,
    pub open_count: core::sync::atomic::AtomicI32,
    pub immutable_target: *mut dm_target,
    pub immutable_target_type: *mut target_type,
    pub name: [c_char; 16],
    pub disk: *mut gendisk,
    pub dax_dev: *mut dax_device,
    pub wait: wait_queue_head_t,
    pub pending_io: *mut unsigned long __percpu,
// forced geometry settings
    pub geometry: hd_geometry,
//
// Processing queue (flush)
//
    pub wq: *mut workqueue_struct,
//
// A list of ios that arrived while we were suspended.
//
    pub work: work_struct,
    pub deferred_lock: spinlock_t,
    pub deferred: bio_list,
//
// requeue work context is needed for cloning one new bio
// to represent the dm_io to be requeued, since each
// dm_io may point to the original bio from FS.
//
    pub requeue_work: work_struct,
    pub requeue_list: *mut dm_io,
    pub interface_ptr: *mut c_void,
//
// Event handling.
//
    pub eventq: wait_queue_head_t,
    pub event_nr: core::sync::atomic::AtomicI32,
    pub uevent_seq: core::sync::atomic::AtomicI32,
    pub uevent_list: list_head,
    pub /: *mut *mut spinlock_t uevent_lock; / Protect access to uevent_list,
// for blk-mq request-based DM support
    pub init_tio_pdu:1: bool,
    pub tag_set: *mut blk_mq_tag_set,
    pub stats: dm_stats,
// the number of internal suspends
    pub internal_suspend_count: c_uint,
    pub swap_bios: c_int,
    pub swap_bios_semaphore: semaphore,
    pub swap_bios_lock: mutex,
//
// io objects are allocated from here.
//
    pub mempools: *mut dm_md_mempools,
// kobject and completion
    pub kobj_holder: dm_kobject_holder,
    pub io_barrier: srcu_struct,

    pub zone_revalidate_map: *mut c_void,
    pub revalidate_map_task: *mut task_struct,

    pub ima: dm_ima_measurements,

}

//
// Bits for the flags field of struct mapped_device.
//
pub const DMF_BLOCK_IO_FOR_SUSPEND: c_int = 0;
pub const DMF_SUSPENDED: c_int = 1;
pub const DMF_FROZEN: c_int = 2;
pub const DMF_FREEING: c_int = 3;
pub const DMF_DELETING: c_int = 4;
pub const DMF_NOFLUSH_SUSPENDING: c_int = 5;
pub const DMF_DEFERRED_REMOVE: c_int = 6;
pub const DMF_SUSPENDED_INTERNALLY: c_int = 7;
pub const DMF_POST_SUSPENDING: c_int = 8;
pub const DMF_EMULATE_ZONE_APPEND: c_int = 9;
pub const DMF_QUEUE_STOPPED: c_int = 10;
extern "C" {
    pub fn get_capacity(_arg: md->disk) -> return;
}
extern "C" {
    pub fn test_bit(_arg: DMF_EMULATE_ZONE_APPEND, _arg: &md->flags) -> return;
}
pub const DM_TABLE_MAX_DEPTH: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_table {
    pub md: *mut mapped_device,
    pub type: dm_queue_mode,
// btree table
    pub depth: c_uint,
    pub /: *mut *mut unsigned int counts[DM_TABLE_MAX_DEPTH]; / in nodes,
    pub index: [*mut sector_t; DM_TABLE_MAX_DEPTH],
    pub num_targets: c_uint,
    pub num_allocated: c_uint,
    pub highs: *mut sector_t,
    pub targets: *mut dm_target,
    pub immutable_target_type: *mut target_type,
    pub integrity_supported:1: bool,
    pub singleton:1: bool,
// set if all the targets in the table have "flush_bypasses_map" set
    pub flush_bypasses_map:1: bool,
//
// Indicates the rw permissions for the new logical device.  This
// should be a combination of BLK_OPEN_READ and BLK_OPEN_WRITE.
//
    pub mode: blk_mode_t,
// a list of devices used by this table
    pub devices: list_head,
// events get handed up using this callback
    pub data): *mut *mut void (event_fn)(void,
    pub event_context: *mut c_void,
    pub mempools: *mut dm_md_mempools,

    pub crypto_profile: *mut blk_crypto_profile,

}

//
// One of these is allocated per clone bio.
//
pub const DM_TIO_MAGIC: c_int = 28714;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_target_io {
    pub magic: c_ushort,
    pub flags: blk_short_t,
    pub target_bio_nr: c_uint,
    pub io: *mut dm_io,
    pub ti: *mut dm_target,
    pub len_ptr: *mut c_uint,
    pub old_sector: sector_t,
    pub clone: bio,
}

//
// dm_target_io flags
//
// One of these is allocated per original bio.
// It contains the first clone used for that original.
//
pub const DM_IO_MAGIC: c_int = 19577;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_io {
    pub magic: c_ushort,
    pub flags: blk_short_t,
    pub lock: spinlock_t,
    pub start_time: c_ulong,
    pub data: *mut c_void,
    pub next: *mut dm_io,
    pub stats_aux: dm_stats_aux,
    pub status: blk_status_t,
    pub requeue_flush_with_data: bool,
    pub io_count: core::sync::atomic::AtomicI32,
    pub md: *mut mapped_device,
// The three fields represent mapped part of original bio
    pub orig_bio: *mut bio,
    pub /: *mut *mut unsigned int sector_offset; / offset to end of orig_bio,
    pub sectors: c_uint,
// last member of dm_target_io is 'struct bio'
    pub tio: dm_target_io,
}

//
// dm_io flags
//
extern "C" {
    pub fn dm_io_rewind(io: *mut dm_io, bs: *mut bio_set);
}
extern "C" {
    pub fn __dm_get_module_param(module_param: *mut c_uint, def: c_uint, max: c_uint) -> c_uint;
}
extern "C" {
    pub fn dm_issue_global_event();
}
