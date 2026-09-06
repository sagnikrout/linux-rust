//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/char/vmur.h
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
// Linux driver for System z and s390 unit record devices
// (z/VM virtual punch, reader, printer)
//
// Copyright IBM Corp. 2001, 2007
// Authors: Malcolm Beattie <beattiem@uk.ibm.com>
// Michael Holzheu <holzheu@de.ibm.com>
// Frank Munzert <munzert@de.ibm.com>
//

pub const DEV_CLASS_UR_I: c_uint = 0x20 /* diag210 unit record input device class */;
pub const DEV_CLASS_UR_O: c_uint = 0x10 /* diag210 unit record output device class */;
//
// we only support z/VM's default unit record devices:
// both in SPOOL directory control statement and in CP DEFINE statement
// RDR defaults to 2540 reader
// PUN defaults to 2540 punch
// PRT defaults to 1403 printer
//
pub const READER_PUNCH_DEVTYPE: c_uint = 0x2540;
pub const PRINTER_DEVTYPE: c_uint = 0x1403;
// z/VM spool file control block SFBLOK
#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_control_block {
    pub reserved_1: [c_char; 8],
    pub user_owner: [c_char; 8],
    pub user_orig: [c_char; 8],
    pub data_recs: __s32,
    pub rec_len: __s16,
    pub file_num: __s16,
    pub file_stat: __u8,
    pub dev_type: __u8,
    pub reserved_2: [c_char; 6],
    pub file_name: [c_char; 12],
    pub file_type: [c_char; 12],
    pub create_date: [c_char; 8],
    pub create_time: [c_char; 8],
    pub reserved_3: [c_char; 6],
    pub file_class: __u8,
    pub sfb_lok: __u8,
    pub distr_code: __u64,
    pub reserved_4: __u32,
    pub current_starting_copy_number: __u8,
    pub sfblock_cntrl_flags: __u8,
    pub reserved_5: __u8,
    pub more_status_flags: __u8,
    pub rest: [c_char; 200],
// C attribute field omitted
pub const FLG_SYSTEM_HOLD: c_uint = 0x04;
pub const FLG_CP_DUMP: c_uint = 0x10;
pub const FLG_USER_HOLD: c_uint = 0x20;
pub const FLG_IN_USE: c_uint = 0x80;
//
// A struct urdev is created for each ur device that is made available
// via the ccw_device driver model.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct urdev {
    pub /: *mut *mut *mut ccw_device cdev; / Backpointer to ccw device,
    pub /: *mut *mut mutex io_mutex; / Serialises device IO,
    pub /: *mut *mut *mut completion io_done; / do_ur_io waits; irq completes,
    pub device: *mut device,
    pub char_device: *mut cdev,
    pub /: *mut *mut ccw_dev_id dev_id; / device id,
    pub /: *mut *mut *mut *mut size_t reclen; / Record length for write CCWs,
    pub /: *mut *mut int class; / VM device class,
    pub /: *mut *mut int io_request_rc; / return code from I/O request,
    pub /: *mut *mut refcount_t ref_count; / reference counter,
    pub /: *mut *mut wait_queue_head_t wait; / wait queue to serialize open,
    pub /: *mut *mut int open_flag; / "urdev is open" flag,
    pub /: *mut *mut spinlock_t open_lock; / serialize critical sections,
    pub /: *mut *mut work_uevent_work; / work to send uevent,
}

//
// A struct urfile is allocated at open() time for each device and
// freed on release().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct urfile {
    pub urd: *mut urdev,
    pub flags: c_uint,
    pub dev_reclen: usize,
    pub file_reclen: __u16,
}

//
// Device major/minor definitions.
//

//
// We map minor numbers directly to device numbers (0-FFFF) for simplicity.
// This avoids having to allocate (and manage) slot numbers.
//
pub const NUM_MINORS: c_int = 65536;
// Limiting each I/O to 511 records limits chan prog to 4KB (511 r/w + 1 NOP)
pub const MAX_RECS_PER_IO: c_int = 511;
pub const WRITE_CCW_CMD: c_uint = 0x01;

