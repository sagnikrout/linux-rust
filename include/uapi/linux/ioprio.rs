//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/ioprio.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

//
// Gives us 8 prio classes with 13-bits of data for each class
//
pub const IOPRIO_CLASS_SHIFT: c_int = 13;
pub const IOPRIO_NR_CLASSES: c_int = 8;

//
// These are the io priority classes as implemented by the BFQ and mq-deadline
// schedulers. RT is the realtime class, it always gets premium service. For
// ATA disks supporting NCQ IO priority, RT class IOs will be processed using
// high priority NCQ commands. BE is the best-effort scheduling class, the
// default for any process. IDLE is the idle scheduling class, it is only
// served when no one else is using the disk.
//
// Special class to indicate an invalid ioprio value
//
// The RT and BE priority classes both support up to 8 priority levels that
// can be specified using the lower 3-bits of the priority data.
//
pub const IOPRIO_LEVEL_NR_BITS: c_int = 3;

//
// Possible values for the "which" argument of the ioprio_get() and
// ioprio_set() system calls (see "man ioprio_set").
//
// Fallback BE class priority level.
//
pub const IOPRIO_NORM: c_int = 4;

//
// The 10 bits between the priority class and the priority level are used to
// optionally define I/O hints for any combination of I/O priority class and
// level. Depending on the kernel configuration, I/O scheduler being used and
// the target I/O device being used, hints can influence how I/Os are processed
// without affecting the I/O scheduling ordering defined by the I/O priority
// class and level.
//

pub const IOPRIO_HINT_NR_BITS: c_int = 10;

//
// I/O hints.
//
// No hint
//
// Device command duration limits: indicate to the device a desired
// duration limit for the commands that will be used to process an I/O.
// These will currently only be effective for SCSI and ATA devices that
// support the command duration limits feature. If this feature is
// enabled, then the commands issued to the device to process an I/O with
// one of these hints set will have the duration limit index (dld field)
// set to the value of the hint.
//

//
// Return an I/O priority value based on a class, a level and a hint.
//

