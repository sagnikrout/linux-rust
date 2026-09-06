//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/android/binder.h
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
// Copyright (C) 2008 Google, Inc.
//
// Based on, but no longer compatible with, the original
// OpenBinder.org binder driver interface, which is:
//
// Copyright (c) 2005 Palmsource, Inc.
//
// This software is licensed under the terms of the GNU General Public
// License version 2, as published by the Free Software Foundation, and
// may be copied, distributed, and modified under those terms.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//

pub const B_TYPE_LARGE: c_uint = 0x85;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum flat_binder_object_flags {
    FLAT_BINDER_FLAG_PRIORITY_MASK = 0xff,
    FLAT_BINDER_FLAG_ACCEPTS_FDS = 0x100,

//
// @FLAT_BINDER_FLAG_TXN_SECURITY_CTX: request security contexts
//
// Only when set, causes senders to include their security
// context
//
    FLAT_BINDER_FLAG_TXN_SECURITY_CTX = 0x1000,
}

pub type binder_size_t = __u32;
pub type binder_uintptr_t = __u32;

pub type binder_size_t = __u64;
pub type binder_uintptr_t = __u64;

//
// struct binder_object_header - header shared by all binder metadata objects.
// @type:	type of the object
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_object_header {
    pub type: __u32,
}

//
// This is the flattened representation of a Binder object for transfer
// between processes.  The 'offsets' supplied as part of a binder transaction
// contains offsets into the data where these structures occur.  The Binder
// driver takes care of re-writing the structure type and data as it moves
// between processes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flat_binder_object {
    pub hdr: binder_object_header,
    pub flags: __u32,
// 8 bytes of data.
    pub /: *mut *mut binder_uintptr_t binder; / local object,
    pub /: *mut *mut __u32 handle; / remote object,
}

// extra data associated with local object
//
// struct binder_fd_object - describes a filedescriptor to be fixed up.
// @hdr:	common header structure
// @pad_flags:	padding to remain compatible with old userspace code
// @pad_binder:	padding to remain compatible with old userspace code
// @fd:		file descriptor
// @cookie:	opaque data, used by user-space
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_fd_object {
    pub hdr: binder_object_header,
    pub pad_flags: __u32,
    pub pad_binder: binder_uintptr_t,
    pub fd: __u32,
}

// struct binder_buffer_object - object describing a userspace buffer
// @hdr:		common header structure
// @flags:		one or more BINDER_BUFFER_* flags
// @buffer:		address of the buffer
// @length:		length of the buffer
// @parent:		index in offset array pointing to parent buffer
// @parent_offset:	offset in @parent pointing to this buffer
//
// A binder_buffer object represents an object that the
// binder kernel driver can copy verbatim to the target
// address space. A buffer itself may be pointed to from
// within another buffer, meaning that the pointer inside
// that other buffer needs to be fixed up as well. This
// can be done by setting the BINDER_BUFFER_FLAG_HAS_PARENT
// flag in @flags, by setting @parent buffer to the index
// in the offset array pointing to the parent binder_buffer_object,
// and by setting @parent_offset to the offset in the parent buffer
// at which the pointer to this buffer is located.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_buffer_object {
    pub hdr: binder_object_header,
    pub flags: __u32,
    pub buffer: binder_uintptr_t,
    pub length: binder_size_t,
    pub parent: binder_size_t,
    pub parent_offset: binder_size_t,
}

// struct binder_fd_array_object - object describing an array of fds in a buffer
// @hdr:		common header structure
// @pad:		padding to ensure correct alignment
// @num_fds:		number of file descriptors in the buffer
// @parent:		index in offset array to buffer holding the fd array
// @parent_offset:	start offset of fd array in the buffer
//
// A binder_fd_array object represents an array of file
// descriptors embedded in a binder_buffer_object. It is
// different from a regular binder_buffer_object because it
// describes a list of file descriptors to fix up, not an opaque
// blob of memory, and hence the kernel needs to treat it differently.
//
// An example of how this would be used is with Android's
// native_handle_t object, which is a struct with a list of integers
// and a list of file descriptors. The native_handle_t struct itself
// will be represented by a struct binder_buffer_objct, whereas the
// embedded list of file descriptors is represented by a
// struct binder_fd_array_object with that binder_buffer_object as
// a parent.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_fd_array_object {
    pub hdr: binder_object_header,
    pub pad: __u32,
    pub num_fds: binder_size_t,
    pub parent: binder_size_t,
    pub parent_offset: binder_size_t,
}

//
// On 64-bit platforms where user code may run in 32-bits the driver must
// translate the buffer (and local binder) addresses appropriately.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_write_read {
    pub /: *mut *mut binder_size_t write_size; / bytes to write,
    pub /: *mut *mut binder_size_t write_consumed; / bytes consumed by driver,
    pub write_buffer: binder_uintptr_t,
    pub /: *mut *mut binder_size_t read_size; / bytes to read,
    pub /: *mut *mut binder_size_t read_consumed; / bytes consumed by driver,
    pub read_buffer: binder_uintptr_t,
}

// Use with BINDER_VERSION, driver fills in fields.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_version {
// driver protocol version -- increment with incompatible change
    pub protocol_version: __s32,
}

// This is the current protocol version.

pub const BINDER_CURRENT_PROTOCOL_VERSION: c_int = 7;

pub const BINDER_CURRENT_PROTOCOL_VERSION: c_int = 8;

//
// Use with BINDER_GET_NODE_DEBUG_INFO, driver reads ptr, writes to all fields.
// Set ptr to NULL for the first call to get the info for the first node, and
// then repeat the call passing the previously returned value to get the next
// nodes.  ptr will be 0 when there are no more nodes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_node_debug_info {
    pub ptr: binder_uintptr_t,
    pub cookie: binder_uintptr_t,
    pub has_strong_ref: __u32,
    pub has_weak_ref: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_node_info_for_ref {
    pub handle: __u32,
    pub strong_count: __u32,
    pub weak_count: __u32,
    pub reserved1: __u32,
    pub reserved2: __u32,
    pub reserved3: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_freeze_info {
    pub pid: __u32,
    pub enable: __u32,
    pub timeout_ms: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_frozen_status_info {
    pub pid: __u32,
// process received sync transactions since last frozen
// bit 0: received sync transaction after being frozen
// bit 1: new pending sync transaction during freezing
//
    pub sync_recv: __u32,
// process received async transactions since last frozen
    pub async_recv: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_frozen_state_info {
    pub cookie: binder_uintptr_t,
    pub is_frozen: __u32,
    pub reserved: __u32,
}

// struct binder_extened_error - extended error information
// @id:		identifier for the failed operation
// @command:	command as defined by binder_driver_return_protocol
// @param:	parameter holding a negative errno value
//
// Used with BINDER_GET_EXTENDED_ERROR. This extends the error information
// returned by the driver upon a failed operation. Userspace can pull this
// data to properly handle specific error scenarios.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_extended_error {
    pub id: __u32,
    pub command: __u32,
    pub param: __s32,
}

//
// NOTE: Two special error codes you should check for when calling
// in to the driver are:
//
// EINTR -- The operation has been interrupted.  This should be
// handled by retrying the ioctl() until a different error code
// is returned.
//
// ECONNREFUSED -- The driver is no longer accepting operations
// from your process.  That is, the process is being destroyed.
// You should handle this by exiting from your process.  Note
// that once this error code is returned, all further calls to
// the driver from any thread will return this same code.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum transaction_flags {
    TF_ONE_WAY	= 0x01,	/* this is a one-way call: async, no return */
    TF_ROOT_OBJECT	= 0x04,	/* contents are the component's root object */
    TF_STATUS_CODE	= 0x08,	/* contents are a 32-bit status code */
    TF_ACCEPT_FDS	= 0x10,	/* allow replies with file descriptors */
    TF_CLEAR_BUF	= 0x20,	/* clear buffer on txn complete */
    TF_UPDATE_TXN	= 0x40,	/* update the outdated pending async txn */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_transaction_data {
// The first two are only used for bcTRANSACTION and brTRANSACTION,
// identifying the target and contents of the transaction.
//
// target descriptor of command transaction
    pub handle: __u32,
// target descriptor of return transaction
    pub ptr: binder_uintptr_t,
    pub target: },
    pub /: *mut *mut binder_uintptr_t cookie; / target object cookie,
    pub /: *mut *mut __u32 code; / transaction command,
// General information about the transaction.
    pub flags: __u32,
    pub sender_pid: __kernel_pid_t,
    pub sender_euid: __kernel_uid32_t,
    pub /: *mut *mut binder_size_t data_size; / number of bytes of data,
    pub /: *mut *mut binder_size_t offsets_size; / number of bytes of offsets,
// If this transaction is inline, the data immediately
// follows here; otherwise, it ends with a pointer to
// the data buffer.
//
// transaction data
    pub buffer: binder_uintptr_t,
// offsets from buffer to flat_binder_object structs
    pub offsets: binder_uintptr_t,
    pub ptr: },
    pub buf: [__u8; 8],
    pub data: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_transaction_data_secctx {
    pub transaction_data: binder_transaction_data,
    pub secctx: binder_uintptr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_transaction_data_sg {
    pub transaction_data: binder_transaction_data,
    pub buffers_size: binder_size_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_ptr_cookie {
    pub ptr: binder_uintptr_t,
    pub cookie: binder_uintptr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_handle_cookie {
    pub handle: __u32,
    pub cookie: binder_uintptr_t,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_pri_desc {
    pub priority: __s32,
    pub desc: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_pri_ptr_cookie {
    pub priority: __s32,
    pub ptr: binder_uintptr_t,
    pub cookie: binder_uintptr_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum binder_driver_return_protocol {
    BR_ERROR = _IOR('r', 0, __s32),
//
// int: error code
//

    BR_OK = _IO('r', 1),
// No parameters!

    BR_TRANSACTION_SEC_CTX = _IOR('r', 2,
    struct binder_transaction_data_secctx),
//
// binder_transaction_data_secctx: the received command.
//
    BR_TRANSACTION = _IOR('r', 2, struct binder_transaction_data),
    BR_REPLY = _IOR('r', 3, struct binder_transaction_data),
//
// binder_transaction_data: the received command.
//

    BR_ACQUIRE_RESULT = _IOR('r', 4, __s32),
//
// not currently supported
// int: 0 if the last bcATTEMPT_ACQUIRE was not successful.
// Else the remote object has acquired a primary reference.
//

    BR_DEAD_REPLY = _IO('r', 5),
//
// The target of the last transaction (either a bcTRANSACTION or
// a bcATTEMPT_ACQUIRE) is no longer with us.  No parameters.
//

    BR_TRANSACTION_COMPLETE = _IO('r', 6),
//
// No parameters... always refers to the last transaction requested
// (including replies).  Note that this will be sent even for
// asynchronous transactions.
//

    BR_INCREFS = _IOR('r', 7, struct binder_ptr_cookie),
    BR_ACQUIRE = _IOR('r', 8, struct binder_ptr_cookie),
    BR_RELEASE = _IOR('r', 9, struct binder_ptr_cookie),
    BR_DECREFS = _IOR('r', 10, struct binder_ptr_cookie),
//
// void *:	ptr to binder
// void *: cookie for binder
//

    BR_ATTEMPT_ACQUIRE = _IOR('r', 11, struct binder_pri_ptr_cookie),
//
// not currently supported
// int:	priority
// void *: ptr to binder
// void *: cookie for binder
//

    BR_NOOP = _IO('r', 12),
//
// No parameters.  Do nothing and examine the next command.  It exists
// primarily so that we can replace it with a BR_SPAWN_LOOPER command.
//

    BR_SPAWN_LOOPER = _IO('r', 13),
//
// No parameters.  The driver has determined that a process has no
// threads waiting to service incoming transactions.  When a process
// receives this command, it must spawn a new service thread and
// register it via bcENTER_LOOPER.
//

    BR_FINISHED = _IO('r', 14),
//
// not currently supported
// stop threadpool thread
//

    BR_DEAD_BINDER = _IOR('r', 15, binder_uintptr_t),
//
// void *: cookie
//
    BR_CLEAR_DEATH_NOTIFICATION_DONE = _IOR('r', 16, binder_uintptr_t),
//
// void *: cookie
//

    BR_FAILED_REPLY = _IO('r', 17),
//
// The last transaction (either a bcTRANSACTION or
// a bcATTEMPT_ACQUIRE) failed (e.g. out of memory).  No parameters.
//

    BR_FROZEN_REPLY = _IO('r', 18),
//
// The target of the last sync transaction (either a bcTRANSACTION or
// a bcATTEMPT_ACQUIRE) is frozen.  No parameters.
//

    BR_ONEWAY_SPAM_SUSPECT = _IO('r', 19),
//
// Current process sent too many oneway calls to target, and the last
// asynchronous transaction makes the allocated async buffer size exceed
// detection threshold.  No parameters.
//

    BR_TRANSACTION_PENDING_FROZEN = _IO('r', 20),
//
// The target of the last async transaction is frozen.  No parameters.
//

    BR_FROZEN_BINDER = _IOR('r', 21, struct binder_frozen_state_info),
//
// The cookie and a boolean (is_frozen) that indicates whether the process
// transitioned into a frozen or an unfrozen state.
//

    BR_CLEAR_FREEZE_NOTIFICATION_DONE = _IOR('r', 22, binder_uintptr_t),
//
// void *: cookie
//
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum binder_driver_command_protocol {
    BC_TRANSACTION = _IOW('c', 0, struct binder_transaction_data),
    BC_REPLY = _IOW('c', 1, struct binder_transaction_data),
//
// binder_transaction_data: the sent command.
//

    BC_ACQUIRE_RESULT = _IOW('c', 2, __s32),
//
// not currently supported
// int:  0 if the last BR_ATTEMPT_ACQUIRE was not successful.
// Else you have acquired a primary reference on the object.
//

    BC_FREE_BUFFER = _IOW('c', 3, binder_uintptr_t),
//
// void *: ptr to transaction data received on a read
//

    BC_INCREFS = _IOW('c', 4, __u32),
    BC_ACQUIRE = _IOW('c', 5, __u32),
    BC_RELEASE = _IOW('c', 6, __u32),
    BC_DECREFS = _IOW('c', 7, __u32),
//
// int:	descriptor
//

    BC_INCREFS_DONE = _IOW('c', 8, struct binder_ptr_cookie),
    BC_ACQUIRE_DONE = _IOW('c', 9, struct binder_ptr_cookie),
//
// void *: ptr to binder
// void *: cookie for binder
//

    BC_ATTEMPT_ACQUIRE = _IOW('c', 10, struct binder_pri_desc),
//
// not currently supported
// int: priority
// int: descriptor
//

    BC_REGISTER_LOOPER = _IO('c', 11),
//
// No parameters.
// Register a spawned looper thread with the device.
//

    BC_ENTER_LOOPER = _IO('c', 12),
    BC_EXIT_LOOPER = _IO('c', 13),
//
// No parameters.
// These two commands are sent as an application-level thread
// enters and exits the binder loop, respectively.  They are
// used so the binder can have an accurate count of the number
// of looping threads it has available.
//

    BC_REQUEST_DEATH_NOTIFICATION = _IOW('c', 14,
    struct binder_handle_cookie),
//
// int: handle
// void *: cookie
//

    BC_CLEAR_DEATH_NOTIFICATION = _IOW('c', 15,
    struct binder_handle_cookie),
//
// int: handle
// void *: cookie
//

    BC_DEAD_BINDER_DONE = _IOW('c', 16, binder_uintptr_t),
//
// void *: cookie
//

    BC_TRANSACTION_SG = _IOW('c', 17, struct binder_transaction_data_sg),
    BC_REPLY_SG = _IOW('c', 18, struct binder_transaction_data_sg),
//
// binder_transaction_data_sg: the sent command.
//

    BC_REQUEST_FREEZE_NOTIFICATION =
    _IOW('c', 19, struct binder_handle_cookie),
//
// int: handle
// void *: cookie
//

    BC_CLEAR_FREEZE_NOTIFICATION = _IOW('c', 20,
    struct binder_handle_cookie),
//
// int: handle
// void *: cookie
//

    BC_FREEZE_NOTIFICATION_DONE = _IOW('c', 21, binder_uintptr_t),
//
// void *: cookie
//
}
