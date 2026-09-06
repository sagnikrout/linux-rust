//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/xfs_healthmon.h
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
// Copyright (c) 2024-2026 Oracle.  All Rights Reserved.
// Author: Darrick J. Wong <djwong@kernel.org>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_healthmon {
//
// Weak reference to the xfs filesystem that is being monitored.  It
// will be set to zero when the filesystem detaches from the monitor.
// Do not dereference this pointer.
//
    pub mount_cookie: uintptr_t,
//
// Device number of the filesystem being monitored.  This is for
// consistent tracing even after unmount.
//
    pub dev: dev_t,
//
// Reference count of this structure.  The open healthmon fd holds one
// ref, the xfs_mount holds another ref if it points to this object,
// and running event handlers hold their own refs.
//
    pub ref: refcount_t,
// lock for event list and event counters
    pub lock: mutex,
// list of event objects
    pub first_event: *mut xfs_healthmon_event,
    pub last_event: *mut xfs_healthmon_event,
// preallocated event for unmount
    pub unmount_event: *mut xfs_healthmon_event,
// number of events in the list
    pub events: c_uint,
// do we want all events?
    pub verbose:1: bool,
// waiter so read/poll can sleep until the arrival of events
    pub wait: wait_queue_head,
//
// Buffer for formatting events for a read_iter call.  Events are
// formatted into the buffer at bufhead, and buftail determines where
// to start a copy_iter to get those events to userspace.  All buffer
// fields are protected by inode_lock.
//
    pub buffer: *mut c_char,
    pub bufsize: usize,
    pub bufhead: usize,
    pub buftail: usize,
// did we lose previous events?
    pub lost_prev_event: c_ulonglong,
// total counts of events observed and lost events
    pub total_events: c_ulonglong,
    pub total_lost: c_ulonglong,
}

extern "C" {
    pub fn xfs_healthmon_unmount(mp: *mut xfs_mount);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xfs_healthmon_type {
    XFS_HEALTHMON_RUNNING,	/* monitor running */
    XFS_HEALTHMON_LOST,	/* message lost */
    XFS_HEALTHMON_UNMOUNT,	/* filesystem is unmounting */

// filesystem shutdown
    XFS_HEALTHMON_SHUTDOWN,

// metadata health events
    XFS_HEALTHMON_SICK,	/* runtime corruption observed */
    XFS_HEALTHMON_CORRUPT,	/* fsck reported corruption */
    XFS_HEALTHMON_HEALTHY,	/* fsck reported healthy structure */

// media errors
    XFS_HEALTHMON_MEDIA_ERROR,

// file range events
    XFS_HEALTHMON_BUFREAD,
    XFS_HEALTHMON_BUFWRITE,
    XFS_HEALTHMON_DIOREAD,
    XFS_HEALTHMON_DIOWRITE,
    XFS_HEALTHMON_DATALOST,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xfs_healthmon_domain {
    XFS_HEALTHMON_MOUNT,	/* affects the whole fs */

// metadata health events
    XFS_HEALTHMON_FS,	/* main filesystem metadata */
    XFS_HEALTHMON_AG,	/* allocation group metadata */
    XFS_HEALTHMON_INODE,	/* inode metadata */
    XFS_HEALTHMON_RTGROUP,	/* realtime group metadata */

// media errors
    XFS_HEALTHMON_DATADEV,
    XFS_HEALTHMON_RTDEV,
    XFS_HEALTHMON_LOGDEV,

// file range events
    XFS_HEALTHMON_FILERANGE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_healthmon_event {
    pub next: *mut xfs_healthmon_event,
    pub type: xfs_healthmon_type,
    pub domain: xfs_healthmon_domain,
    pub time_ns: u64,
// lost events
    pub lostcount: u64,
}

// fs/rt metadata
// XFS_SICK_* flags
// ag/rtgroup metadata
// XFS_SICK_(AG|RG)* flags
// inode metadata
// XFS_SICK_INO_* flags
// shutdown
// media errors
// file range events
extern "C" {
    pub fn xfs_healthmon_report_shutdown(mp: *mut xfs_mount, flags: u32);
}
