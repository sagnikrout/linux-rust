//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/dm-ioctl.h
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


// SPDX-License-Identifier: LGPL-2.0+ WITH Linux-syscall-note
//
// Copyright (C) 2001 - 2003 Sistina Software (UK) Limited.
// Copyright (C) 2004 - 2009 Red Hat, Inc. All rights reserved.
//
// This file is released under the LGPL.
//

pub const DM_MAX_TYPE_NAME: c_int = 16;
pub const DM_NAME_LEN: c_int = 128;
pub const DM_UUID_LEN: c_int = 129;
//
// A traditional ioctl interface for the device mapper.
//
// Each device can have two tables associated with it, an
// 'active' table which is the one currently used by io passing
// through the device, and an 'inactive' one which is a table
// that is being prepared as a replacement for the 'active' one.
//
// DM_VERSION:
// Just get the version information for the ioctl interface.
//
// DM_REMOVE_ALL:
// Remove all dm devices, destroy all tables.  Only really used
// for debug.
//
// DM_LIST_DEVICES:
// Get a list of all the dm device names.
//
// DM_DEV_CREATE:
// Create a new device, neither the 'active' or 'inactive' table
// slots will be filled.  The device will be in suspended state
// after creation, however any io to the device will get errored
// since it will be out-of-bounds.
//
// DM_DEV_REMOVE:
// Remove a device, destroy any tables.
//
// DM_DEV_RENAME:
// Rename a device or set its uuid if none was previously supplied.
//
// DM_SUSPEND:
// This performs both suspend and resume, depending which flag is
// passed in.
// Suspend: This command will not return until all pending io to
// the device has completed.  Further io will be deferred until
// the device is resumed.
// Resume: It is no longer an error to issue this command on an
// unsuspended device.  If a table is present in the 'inactive'
// slot, it will be moved to the active slot, then the old table
// from the active slot will be _destroyed_.  Finally the device
// is resumed.
//
// DM_DEV_STATUS:
// Retrieves the status for the table in the 'active' slot.
//
// DM_DEV_WAIT:
// Wait for a significant event to occur to the device.  This
// could either be caused by an event triggered by one of the
// targets of the table in the 'active' slot, or a table change.
//
// DM_TABLE_LOAD:
// Load a table into the 'inactive' slot for the device.  The
// device does _not_ need to be suspended prior to this command.
//
// DM_TABLE_CLEAR:
// Destroy any table in the 'inactive' slot (ie. abort).
//
// DM_TABLE_DEPS:
// Return a set of device dependencies for the 'active' table.
//
// DM_TABLE_STATUS:
// Return the targets status for the 'active' table.
//
// DM_TARGET_MSG:
// Pass a message string to the target at a specific offset of a device.
//
// DM_DEV_SET_GEOMETRY:
// Set the geometry of a device by passing in a string in this format:
//
// "cylinders heads sectors_per_track start_sector"
//
// Beware that CHS geometry is nearly obsolete and only provided
// for compatibility with dm devices that can be booted by a PC
// BIOS.  See struct hd_geometry for range limits.  Also note that
// the geometry is erased if the device size changes.
//
// All ioctl arguments consist of a single chunk of memory, with
// this structure at the start.  If a uuid is specified any
// lookup (eg. for a DM_INFO) will be done on that, *not* the
// name.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_ioctl {
//
// The version number is made up of three parts:
// major - no backward or forward compatibility,
// minor - only backwards compatible,
// patch - both backwards and forwards compatible.
//
// All clients of the ioctl interface should fill in the
// version number of the interface that they were
// compiled with.
//
// All recognised ioctl commands (ie. those that don't
// return -ENOTTY) fill out this field, even if the
// command failed.
//
    pub /: *mut *mut __u32 version[3]; / in/out,
    pub in: *mut *mut __u32 data_size; / total size of data passed,
// including this struct
    pub data: *mut *mut __u32 data_start; / offset to start of,
// relative to start of this struct
    pub /: *mut *mut __u32 target_count; / in/out,
    pub /: *mut *mut __s32 open_count; / out,
    pub /: *mut *mut __u32 flags; / in/out,
//
// event_nr holds either the event number (input and output) or the
// udev cookie value (input only).
// The DM_DEV_WAIT ioctl takes an event number as input.
// The DM_SUSPEND, DM_DEV_REMOVE and DM_DEV_RENAME ioctls
// use the field as a cookie to return in the DM_COOKIE
// variable with the uevents they issue.
// For output, the ioctls return the event number, not the cookie.
//
    pub /: *mut *mut __u32 event_nr; / in/out,
    pub padding: __u32,
    pub /: *mut *mut __u64 dev; / in/out,
    pub /: *mut *mut char name[DM_NAME_LEN]; / device name,
    pub for: *mut *mut char uuid[DM_UUID_LEN]; / unique identifier,
// the block device
    pub /: *mut *mut char data[7]; / padding or data,
}

//
// Used to specify tables.  These structures appear after the
// dm_ioctl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_target_spec {
    pub sector_start: __u64,
    pub length: __u64,
    pub /: *mut *mut __s32 status; / used when reading from kernel only,
//
// Location of the next dm_target_spec.
// - When specifying targets on a DM_TABLE_LOAD command, this value is
// the number of bytes from the start of the "current" dm_target_spec
// to the start of the "next" dm_target_spec.
// - When retrieving targets on a DM_TABLE_STATUS command, this value
// is the number of bytes from the start of the first dm_target_spec
// (that follows the dm_ioctl struct) to the start of the "next"
// dm_target_spec.
//
    pub next: __u32,
    pub target_type: [c_char; DM_MAX_TYPE_NAME],
//
// Parameter string starts immediately after this object.
// Be careful to add padding after string to ensure correct
// alignment of subsequent dm_target_spec.
//
}

//
// Used to retrieve the target dependencies.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_target_deps {
    pub /: *mut *mut __u32 count; / Array size,
    pub /: *mut *mut __u32 padding; / unused,
    pub /: *mut *mut __u64 dev[]; / out,
}

//
// Used to get a list of all dm devices.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_name_list {
    pub dev: __u64,
    pub from: *mut *mut __u32 next; / offset to the next record,
    pub name: [c_char; ],
//
// The following members can be accessed by taking a pointer that
// points immediately after the terminating zero character in "name"
// and aligning this pointer to next 8-byte boundary.
// Uuid is present if the flag DM_NAME_LIST_FLAG_HAS_UUID is set.
//
// __u32 event_nr;
// __u32 flags;
// char uuid[0];
//
}

pub const DM_NAME_LIST_FLAG_HAS_UUID: c_int = 1;
pub const DM_NAME_LIST_FLAG_DOESNT_HAVE_UUID: c_int = 2;
//
// Used to retrieve the target versions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_target_versions {
    pub next: __u32,
    pub version: [__u32; 3],
    pub name: [c_char; ],
}

//
// Used to pass message to a target
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_target_msg {
    pub /: *mut *mut __u64 sector; / Device sector,
    pub message: [c_char; ],
}

//
// If you change this make sure you make the corresponding change
// to dm-ioctl.c:lookup_ioctl()
//
// Top level cmds
// device level cmds
// Table level cmds
// Added later
pub const DM_IOCTL: c_uint = 0xfd;
// Control device ioctls

// Block device ioctls

pub const DM_VERSION_MAJOR: c_int = 4;
pub const DM_VERSION_MINOR: c_int = 50;
pub const DM_VERSION_PATCHLEVEL: c_int = 0;

// Status bits

//
// Flag passed into ioctl STATUS command to get table information
// rather than current status.
//

//
// Flags that indicate whether a table is present in either of
// the two table slots that a device has.
//

//
// Indicates that the buffer passed in wasn't big enough for the
// results.
//

//
// This flag is now ignored.
//

//
// Set this to avoid attempting to freeze any filesystem when suspending.
//

//
// Set this to suspend without flushing queued ios.
// Also disables flushing uncommitted changes in the thin target before
// generating statistics for DM_TABLE_STATUS and DM_DEV_WAIT.
//

//
// If set, any table information returned will relate to the inactive
// table instead of the live one.  Always check DM_INACTIVE_PRESENT_FLAG
// is set before using the data returned.
//

//
// If set, a uevent was generated for which the caller may need to wait.
//

//
// If set, rename changes the uuid not the name.  Only permitted
// if no uuid was previously supplied: an existing uuid cannot be changed.
//

//
// If set, all buffers are wiped after use. Use when sending
// or requesting sensitive data such as an encryption key.
//

//
// If set, a message generated output data.
//

//
// If set with DM_DEV_REMOVE or DM_REMOVE_ALL this indicates that if
// the device cannot be removed immediately because it is still in use
// it should instead be scheduled for removal when it gets closed.
//
// On return from DM_DEV_REMOVE, DM_DEV_STATUS or other ioctls, this
// flag indicates that the device is scheduled to be removed when it
// gets closed.
//

//
// If set, the device is suspended internally.
//

//
// If set, returns in the in buffer passed by UM, the raw table information
// that would be measured by IMA subsystem on device state change.
//

