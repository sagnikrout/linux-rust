//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pstore.h
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
// Persistent Storage - pstore.h
//
// Copyright (C) 2010 Intel Corporation <tony.luck@intel.com>
//
// This code is the generic layer to export data records from platform
// level persistent storage via a file system.
//

//
// pstore record types (see fs/pstore/platform.c for pstore_type_names[])
// These values may be written to storage (see EFI vars backend), so
// they are kind of an ABI. Be careful changing the mappings.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pstore_type_id {
// Frontend storage types
    PSTORE_TYPE_DMESG	= 0,
    PSTORE_TYPE_MCE		= 1,
    PSTORE_TYPE_CONSOLE	= 2,
    PSTORE_TYPE_FTRACE	= 3,

// PPC64-specific partition types
    PSTORE_TYPE_PPC_RTAS	= 4,
    PSTORE_TYPE_PPC_OF	= 5,
    PSTORE_TYPE_PPC_COMMON	= 6,
    PSTORE_TYPE_PMSG	= 7,
    PSTORE_TYPE_PPC_OPAL	= 8,

// End of the list
    PSTORE_TYPE_MAX
}

extern "C" {
    pub fn pstore_name_to_type(name: *const c_char) -> pstore_type_id;
}
//
// struct pstore_record - details of a pstore record entry
// @psi:	pstore backend driver information
// @type:	pstore record type
// @id:		per-type unique identifier for record
// @time:	timestamp of the record
// @buf:	pointer to record contents
// @size:	size of @buf
// @ecc_notice_size:
// ECC information for @buf
// @priv:	pointer for backend specific use, will be
// kfree()d by the pstore core if non-NULL
// when the record is freed.
//
// Valid for PSTORE_TYPE_DMESG @type:
//
// @count:	Oops count since boot
// @reason:	kdump reason for notification
// @part:	position in a multipart record
// @compressed:	whether the buffer is compressed
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pstore_record {
    pub psi: *mut pstore_info,
    pub type: pstore_type_id,
    pub id: u64,
    pub time: timespec64,
    pub buf: *mut c_char,
    pub size: isize,
    pub ecc_notice_size: isize,
    pub priv: *mut c_void,
    pub count: c_int,
    pub reason: kmsg_dump_reason,
    pub part: c_uint,
    pub compressed: bool,
}

//
// struct pstore_info - backend pstore driver structure
//
// @owner:	module which is responsible for this backend driver
// @name:	name of the backend driver
//
// @buf_lock:	spinlock to serialize access to @buf
// @buf:	preallocated crash dump buffer
// @bufsize:	size of @buf available for crash dump bytes (must match
// smallest number of bytes available for writing to a
// backend entry, since compressed bytes don't take kindly
// to being truncated)
//
// @read_mutex:	serializes @open, @read, @close, and @erase callbacks
// @flags:	bitfield of frontends the backend can accept writes for
// @max_reason:	Used when PSTORE_FLAGS_DMESG is set. Contains the
// kmsg_dump_reason enum value. KMSG_DUMP_UNDEF means
// "use existing kmsg_dump() filtering, based on the
// printk.always_kmsg_dump boot param" (which is either
// KMSG_DUMP_OOPS when false, or KMSG_DUMP_MAX when
// true); see printk.always_kmsg_dump for more details.
// @data:	backend-private pointer passed back during callbacks
//
// Callbacks:
//
// @open:
// Notify backend that pstore is starting a full read of backend
// records. Followed by one or more @read calls, and a final @close.
//
// @psi:	in: pointer to the struct pstore_info for the backend
//
// Returns 0 on success, and non-zero on error.
//
// @close:
// Notify backend that pstore has finished a full read of backend
// records. Always preceded by an @open call and one or more @read
// calls.
//
// @psi:	in: pointer to the struct pstore_info for the backend
//
// Returns 0 on success, and non-zero on error. (Though pstore will
// ignore the error.)
//
// @read:
// Read next available backend record. Called after a successful
// @open.
//
// @record:
// pointer to record to populate. @buf should be allocated
// by the backend and filled. At least @type and @id should
// be populated, since these are used when creating pstorefs
// file names.
//
// Returns record size on success, zero when no more records are
// available, or negative on error.
//
// @write:
// A newly generated record needs to be written to backend storage.
//
// @record:
// pointer to record metadata. When @type is PSTORE_TYPE_DMESG,
// @buf will be pointing to the preallocated @psi.buf, since
// memory allocation may be broken during an Oops. Regardless,
// @buf must be proccesed or copied before returning. The
// backend is also expected to write @id with something that
// can help identify this record to a future @erase callback.
// The @time field will be prepopulated with the current time,
// when available. The @size field will have the size of data
// in @buf.
//
// Returns 0 on success, and non-zero on error.
//
// @write_user:
// Perform a frontend write to a backend record, using a specified
// buffer that is coming directly from userspace, instead of the
// @record @buf.
//
// @record:	pointer to record metadata.
// @buf:		pointer to userspace contents to write to backend
//
// Returns 0 on success, and non-zero on error.
//
// @erase:
// Delete a record from backend storage.  Different backends
// identify records differently, so entire original record is
// passed back to assist in identification of what the backend
// should remove from storage.
//
// @record:	pointer to record metadata.
//
// Returns 0 on success, and non-zero on error.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pstore_info {
    pub owner: *mut module,
    pub name: *const c_char,
    pub buf_lock: raw_spinlock_t,
    pub buf: *mut c_char,
    pub bufsize: usize,
    pub read_mutex: mutex,
    pub flags: c_int,
    pub max_reason: c_int,
    pub data: *mut c_void,
    pub psi): *mut *mut int (open)(struct pstore_info,
    pub psi): *mut *mut int (close)(struct pstore_info,
    pub record): *mut *mut ssize_t (read)(struct pstore_record,
    pub record): *mut *mut int (write)(struct pstore_record,
    pub buf): *const char __user,
    pub record): *mut *mut int (erase)(struct pstore_record,
}

// Supported frontends

extern "C" {
    pub fn pstore_register(: *mut pstore_info) -> c_int;
}
extern "C" {
    pub fn pstore_unregister(: *mut pstore_info);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pstore_ftrace_record {
    pub ip: c_ulong,
    pub parent_ip: c_ulong,
    pub ts: u64,
}

//
// ftrace related stuff: Both backends and frontends need these so expose
// them here.
//

pub const PSTORE_CPU_IN_IP: c_uint = 0x1;

pub const PSTORE_CPU_IN_IP: c_uint = 0x3;

pub const TS_CPU_SHIFT: c_int = 8;

//
// If CPU number can be stored in IP, store it there, otherwise store it in
// the time stamp. This means more timestamp resolution is available when
// the CPU can be stored in the IP.
//

