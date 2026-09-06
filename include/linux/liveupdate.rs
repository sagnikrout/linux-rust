//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/liveupdate.h
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
// Copyright (c) 2025, Google LLC.
// Pasha Tatashin <pasha.tatashin@soleen.com>
//

//
// struct liveupdate_file_op_args - Arguments for file operation callbacks.
// @handler:          The file handler being called.
// @retrieve_status:  The retrieve status for the 'can_finish / finish'
// operation. A value of 0 means the retrieve has not been
// attempted, a positive value means the retrieve was
// successful, and a negative value means the retrieve failed,
// and the value is the error code of the call.
// @file:             The file object. For retrieve: [OUT] The callback sets
// this to the new file. For other ops: [IN] The caller sets
// this to the file being operated on.
// @serialized_data:  The opaque u64 handle, preserve/prepare/freeze may update
// this field.
// @private_data:     Private data for the file used to hold runtime state that
// is not preserved. Set by the handler's .preserve()
// callback, and must be freed in the handler's
// .unpreserve() callback.
//
// This structure bundles all parameters for the file operation callbacks.
// The 'data' and 'file' fields are used for both input and output.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct liveupdate_file_op_args {
    pub handler: *mut liveupdate_file_handler,
    pub retrieve_status: c_int,
    pub file: *mut file,
    pub serialized_data: u64,
    pub private_data: *mut c_void,
}

//
// struct liveupdate_file_ops - Callbacks for live-updatable files.
// @can_preserve: Required. Lightweight check to see if this handler is
// compatible with the given file.
// @preserve:     Required. Performs state-saving for the file.
// @unpreserve:   Required. Cleans up any resources allocated by @preserve.
// @freeze:       Optional. Final actions just before kernel transition.
// @unfreeze:     Optional. Undo freeze operations.
// @retrieve:     Required. Restores the file in the new kernel.
// @can_finish:   Optional. Check if this FD can finish, i.e. all restoration
// pre-requirements for this FD are satisfied. Called prior to
// finish, in order to do successful finish calls for all
// resources in the session.
// @finish:       Required. Final cleanup in the new kernel.
// @get_id:       Optional. Returns a unique identifier for the file.
// @owner:        Module reference
//
// All operations (except can_preserve) receive a pointer to a
// 'struct liveupdate_file_op_args' containing the necessary context.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct liveupdate_file_ops {
    pub file): *mut file,
    pub args): *mut *mut int (preserve)(struct liveupdate_file_op_args,
    pub args): *mut *mut void (unpreserve)(struct liveupdate_file_op_args,
    pub args): *mut *mut int (freeze)(struct liveupdate_file_op_args,
    pub args): *mut *mut void (unfreeze)(struct liveupdate_file_op_args,
    pub args): *mut *mut int (retrieve)(struct liveupdate_file_op_args,
    pub args): *mut *mut bool (can_finish)(struct liveupdate_file_op_args,
    pub args): *mut *mut void (finish)(struct liveupdate_file_op_args,
    pub file): *mut *mut unsigned long (get_id)(struct file,
    pub owner: *mut module,
}

//
// struct liveupdate_file_handler - Represents a handler for a live-updatable file type.
// @ops:                Callback functions
// @compatible:         The compatibility string (e.g., "memfd-v1", "vfiofd-v1")
// that uniquely identifies the file type this handler
// supports. This is matched against the compatible string
// associated with individual &struct file instances.
//
// Modules that want to support live update for specific file types should
// register an instance of this structure. LUO uses this registration to
// determine if a given file can be preserved and to find the appropriate
// operations to manage its state across the update.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct liveupdate_file_handler {
    pub ops: *const liveupdate_file_ops,
    pub compatible: [c_char; LIVEUPDATE_HNDL_COMPAT_LENGTH],
// private:
//
// Used for linking this handler instance into a global list of
// registered file handlers.
//
    pub list: list_head __private,
// A list of FLB dependencies.
    pub flb_list: list_head __private,
}

//
// struct liveupdate_flb_op_args - Arguments for FLB operation callbacks.
// @flb:       The global FLB instance for which this call is performed.
// @data:      For .preserve():    [OUT] The callback sets this field.
// For .unpreserve():  [IN]  The handle from .preserve().
// For .retrieve():    [IN]  The handle from .preserve().
// @obj:       For .preserve():    [OUT] Sets this to the live object.
// For .retrieve():    [OUT] Sets this to the live object.
// For .finish():      [IN]  The live object from .retrieve().
//
// This structure bundles all parameters for the FLB operation callbacks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct liveupdate_flb_op_args {
    pub flb: *mut liveupdate_flb,
    pub data: u64,
    pub obj: *mut c_void,
}

//
// struct liveupdate_flb_ops - Callbacks for global File-Lifecycle-Bound data.
// @preserve:        Called when the first file using this FLB is preserved.
// The callback must save its state and return a single,
// self-contained u64 handle by setting the 'argp->data'
// field and 'argp->obj'.
// @unpreserve:      Called when the last file using this FLB is unpreserved
// (aborted before reboot). Receives the handle via
// 'argp->data' and live object via 'argp->obj'.
// @retrieve:        Called on-demand in the new kernel, the first time a
// component requests access to the shared object. It receives
// the preserved handle via 'argp->data' and must reconstruct
// the live object, returning it by setting the 'argp->obj'
// field.
// @finish:          Called in the new kernel when the last file using this FLB
// is finished. Receives the live object via 'argp->obj' for
// cleanup.
// @owner:           Module reference
//
// Operations that manage global shared data with file bound lifecycle,
// triggered by the first file that uses it and concluded by the last file that
// uses it, across all sessions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct liveupdate_flb_ops {
    pub argp): *mut *mut int (preserve)(struct liveupdate_flb_op_args,
    pub argp): *mut *mut void (unpreserve)(struct liveupdate_flb_op_args,
    pub argp): *mut *mut int (retrieve)(struct liveupdate_flb_op_args,
    pub argp): *mut *mut void (finish)(struct liveupdate_flb_op_args,
    pub owner: *mut module,
}

//
// struct luo_flb_private_state - Private FLB state structures.
// @count:     The number of preserved files currently depending on this FLB.
// This is used to trigger the preserve/unpreserve/finish ops on the
// first/last file.
// @data:      The opaque u64 handle returned by .preserve() or passed to
// .retrieve().
// @obj:       The live kernel object returned by .preserve() or .retrieve().
// @lock:      A mutex that protects all fields within this structure, providing
// the synchronization service for the FLB's ops.
// @finished:  True once the FLB's finish() callback has run.
// @retrieve_status: Status code indicating whether retrieve() has been
// attempted. 0 means not attempted, 1 means successful,
// and negative value means it failed with that error code.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct luo_flb_private_state {
    pub count: refcount_t,
    pub data: u64,
    pub obj: *mut c_void,
    pub lock: mutex,
    pub finished: bool,
    pub retrieve_status: c_int,
}

//
// struct luo_flb_private - Keep separate incoming and outgoing states.
// @list:        A global list of registered FLBs.
// @outgoing:    The runtime state for the pre-reboot
// (preserve/unpreserve) lifecycle.
// @incoming:    The runtime state for the post-reboot (retrieve/finish)
// lifecycle.
// @users:       With how many File-Handlers this FLB is registered.
// @initialized: true when private fields have been initialized.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct luo_flb_private {
    pub list: list_head,
    pub outgoing: luo_flb_private_state,
    pub incoming: luo_flb_private_state,
    pub users: c_int,
    pub initialized: bool,
}

//
// struct liveupdate_flb - A global definition for a shared data object.
// @ops:         Callback functions
// @compatible:  The compatibility string (e.g., "iommu-core-v1"
// that uniquely identifies the FLB type this handler
// supports. This is matched against the compatible string
// associated with individual &struct liveupdate_flb
// instances.
//
// This struct is the "template" that a driver registers to define a shared,
// file-lifecycle-bound object. The actual runtime state (the live object,
// refcount, etc.) is managed privately by the LUO core.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct liveupdate_flb {
    pub ops: *const liveupdate_flb_ops,
    pub compatible: [c_char; LIVEUPDATE_FLB_COMPAT_LENGTH],
// private:
    pub private: luo_flb_private __private,
}

// Return true if live update orchestrator is enabled
extern "C" {
    pub fn liveupdate_enabled() -> bool;
}
// Called during kexec to tell LUO that entered into reboot
extern "C" {
    pub fn liveupdate_reboot() -> c_int;
}
extern "C" {
    pub fn liveupdate_register_file_handler(fh: *mut liveupdate_file_handler) -> c_int;
}
extern "C" {
    pub fn liveupdate_unregister_file_handler(fh: *mut liveupdate_file_handler);
}
extern "C" {
    pub fn liveupdate_flb_get_incoming(flb: *mut liveupdate_flb, objp: *mut c_void) -> c_int;
}
extern "C" {
    pub fn liveupdate_flb_put_incoming(flb: *mut liveupdate_flb);
}
extern "C" {
    pub fn liveupdate_flb_get_outgoing(flb: *mut liveupdate_flb, objp: *mut c_void) -> c_int;
}
extern "C" {
    pub fn liveupdate_flb_put_outgoing(flb: *mut liveupdate_flb);
}

