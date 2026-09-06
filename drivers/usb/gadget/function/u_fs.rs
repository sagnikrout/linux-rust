//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/function/u_fs.h
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
// u_fs.h
//
// Utility definitions for the FunctionFS
//
// Copyright (c) 2013 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Author: Andrzej Pietrasiewicz <andrzejtp2010@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ffs_dev {
    pub ffs_data: *mut ffs_data,
    pub opts: *mut f_fs_opts,
    pub entry: list_head,
    pub name: [c_char; 41],
    pub mounted: bool,
    pub desc_ready: bool,
    pub single: bool,
    pub ffs): *mut *mut int (ffs_ready_callback)(struct ffs_data,
    pub ffs): *mut *mut void (ffs_closed_callback)(struct ffs_data,
    pub dev): *mut *mut *mut void (ffs_acquire_dev_callback)(struct ffs_dev,
    pub dev): *mut *mut void (ffs_release_dev_callback)(struct ffs_dev,
}

extern "C" {
    pub fn ffs_name_dev(dev: *mut ffs_dev, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn ffs_single_dev(dev: *mut ffs_dev) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ffs_state {
//
// Waiting for descriptors and strings.
//
// In this state no open(2), read(2) or write(2) on epfiles
// may succeed (which should not be the problem as there
// should be no such files opened in the first place).
//
    FFS_READ_DESCRIPTORS,
    FFS_READ_STRINGS,

//
// We've got descriptors and strings.  We are or have called
// functionfs_ready_callback().  functionfs_bind() may have
// been called but we don't know.
//
// This is the only state in which operations on epfiles may
// succeed.
//
    FFS_ACTIVE,

//
// Function is visible to host, but it's not functional. All
// setup requests are stalled and transfers on another endpoints
// are refused. All epfiles, except ep0, are deleted so there
// is no way to perform any operations on them.
//
// This state is set after closing all functionfs files, when
// mount parameter "no_disconnect=1" has been set. Function will
// remain in deactivated state until filesystem is umounted or
// ep0 is opened again. In the second case functionfs state will
// be reset, and it will be ready for descriptors and strings
// writing.
//
// This is useful only when functionfs is composed to gadget
// with another function which can perform some critical
// operations, and it's strongly desired to have this operations
// completed, even after functionfs files closure.
//
    FFS_DEACTIVATED,

//
// All endpoints have been closed.  This state is also set if
// we encounter an unrecoverable error.  The only
// unrecoverable error is situation when after reading strings
// from user space we fail to initialise epfiles or
// functionfs_ready_callback() returns with error (<0).
//
// In this state no open(2), read(2) or write(2) (both on ep0
// as well as epfile) may succeed (at this point epfiles are
// unlinked and all closed so this is not a problem; ep0 is
// also closed but ep0 file exists and so open(2) on ep0 must
// fail).
//
    FFS_CLOSING
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ffs_setup_state {
// There is no setup request pending.
    FFS_NO_SETUP,
//
// User has read events and there was a setup request event
// there.  The next read/write on ep0 will handle the
// request.
//
    FFS_SETUP_PENDING,
//
// There was event pending but before user space handled it
// some other event was introduced which canceled existing
// setup.  If this state is set read/write on ep0 return
// -EIDRM.  This state is only set when adding event.
//
    FFS_SETUP_CANCELLED
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ffs_data {
    pub gadget: *mut usb_gadget,
//
// Protect access read/write operations, only one read/write
// at a time.  As a consequence protects ep0req and company.
// While setup request is being processed (queued) this is
// held.
//
    pub mutex: mutex,
//
// Protect access to endpoint related structures (basically
// usb_ep_queue(), usb_ep_dequeue(), etc. calls) except for
// endpoint zero.
//
    pub eps_lock: spinlock_t,
//
// XXX REVISIT do we need our own request? Since we are not
// handling setup requests immediately user space may be so
// slow that another setup will be sent to the gadget but this
// time not to us but another function and then there could be
// a race.  Is that the case? Or maybe we can use cdev->req
// after all, maybe we just need some spinlock for that?
//
    pub /: *mut *mut *mut usb_request ep0req; / P: mutex,
    pub /: *mut *mut completion ep0req_completion; / P: mutex,
// reference counter
    pub ref: refcount_t,
// how many files are opened (EP0 and others)
    pub opened: c_int,
// EP0 state
    pub state: ffs_state,
//
// Possible transitions:
// + FFS_NO_SETUP        -> FFS_SETUP_PENDING  -- P: ev.waitq.lock
// happens only in ep0 read which is P: mutex
// + FFS_SETUP_PENDING   -> FFS_NO_SETUP       -- P: ev.waitq.lock
// happens only in ep0 i/o  which is P: mutex
// + FFS_SETUP_PENDING   -> FFS_SETUP_CANCELLED -- P: ev.waitq.lock
// + FFS_SETUP_CANCELLED -> FFS_NO_SETUP        -- cmpxchg
//
// This field should never be accessed directly and instead
// ffs_setup_state_clear_cancelled function should be used.
//
    pub setup_state: ffs_setup_state,
// Events & such.
    pub types: [u8; 4],
    pub count: c_ushort,
// XXX REVISIT need to update it in some places, or do we?
    pub can_stall: c_ushort,
    pub setup: usb_ctrlrequest,
    pub waitq: wait_queue_head_t,
    pub /: *mut *mut } ev; / the whole structure, P: ev.waitq.lock,
// Flags
    pub flags: c_ulong,
pub const FFS_FL_CALL_CLOSED_CALLBACK: c_int = 0;
pub const FFS_FL_BOUND: c_int = 1;
// For waking up blocked threads when function is enabled.
    pub wait: wait_queue_head_t,
// Active function
    pub func: *mut ffs_function,
//
// Device name, write once when file system is mounted.
// Intended for user to read if she wants.
//
    pub dev_name: *const c_char,
// Private data for our user (ie. gadget).  Managed by user.
    pub private_data: *mut c_void,
// filled by __ffs_data_got_descs()
//
// raw_descs is what you kfree, real_descs points inside of raw_descs,
// where full speed, high speed and super speed descriptors start.
// real_descs_length is the length of all those descriptors.
//
    pub raw_descs_data: *const c_void,
    pub raw_descs: *const c_void,
    pub raw_descs_length: unsigned,
    pub fs_descs_count: unsigned,
    pub hs_descs_count: unsigned,
    pub ss_descs_count: unsigned,
    pub ms_os_descs_count: unsigned,
    pub ms_os_descs_ext_prop_count: unsigned,
    pub ms_os_descs_ext_prop_name_len: unsigned,
    pub ms_os_descs_ext_prop_data_len: unsigned,
    pub ms_os_descs_ext_prop_avail: *mut c_void,
    pub ms_os_descs_ext_prop_name_avail: *mut c_void,
    pub ms_os_descs_ext_prop_data_avail: *mut c_void,
    pub user_flags: unsigned,
pub const FFS_MAX_EPS_COUNT: c_int = 31;
    pub eps_addrmap: [u8; FFS_MAX_EPS_COUNT],
    pub strings_count: c_ushort,
    pub interfaces_count: c_ushort,
//
// eps_count tracks the number of underlying hardware endpoints.
// epfiles_count tracks the total number of VFS endpoint files.
// When companion endpoints are active, epfiles_count > eps_count.
//
    pub eps_count: c_ushort,
    pub epfiles_count: c_ushort,
// filled by __ffs_data_got_strings()
// ids in stringtabs are set in functionfs_bind()
    pub raw_strings: *const c_void,
    pub stringtabs: *mut usb_gadget_strings,
//
// File system's super block, write once when file system is
// mounted.
//
    pub sb: *mut super_block,
// File permissions, written once when fs is mounted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ffs_file_perms {
    pub mode: umode_t,
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub file_perms: },
    pub ffs_eventfd: *mut eventfd_ctx,
    pub io_completion_wq: *mut workqueue_struct,
    pub no_disconnect: bool,
    pub reset_work: work_struct,
//
// The endpoint files, filled by ffs_epfiles_create(),
// destroyed by ffs_epfiles_destroy().
//
    pub epfiles: *mut ffs_epfile,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f_fs_opts {
    pub func_inst: usb_function_instance,
    pub dev: *mut ffs_dev,
    pub refcnt: unsigned,
    pub no_configfs: bool,
}

extern "C" {
    pub fn container_of(_arg: fi, f_fs_opts: struct, _arg: func_inst) -> return;
}
