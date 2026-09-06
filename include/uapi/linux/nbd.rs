//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/nbd.h
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


// SPDX-License-Identifier: GPL-1.0+ WITH Linux-syscall-note
//
// 1999 Copyright (C) Pavel Machek, pavel@ucw.cz. This code is GPL.
// 1999/11/04 Copyright (C) 1999 VMware, Inc. (Regis "HPReg" Duchesne)
// Made nbd_end_request() use the io_request_lock
// 2001 Copyright (C) Steven Whitehouse
// New nbd_end_request() for compatibility with new linux block
// layer code.
// 2003/06/24 Louis D. Langholtz <ldl@aros.net>
// Removed unneeded blksize_bits field from nbd_device struct.
// Cleanup PARANOIA usage & code.
// 2004/02/19 Paul Clements
// Removed PARANOIA, plus various cleanup and comments
// 2023 Copyright Red Hat
// Link to userspace extensions, favor cookie over handle.
//

//
// See also https://github.com/NetworkBlockDevice/nbd/blob/master/doc/proto.md
// for additional userspace extensions not yet utilized in the kernel module.
//
// userspace defines additional extension commands
// values for flags field, these are server interaction specific.

// there is a gap here to match userspace

// values for cmd flags in the upper 16 bits of request type

// These are client behavior specific flags.

// close by last opener.
//
// userspace doesn't need the nbd_device structure
// These are sent over the network in the request/reply magic fields
pub const NBD_REQUEST_MAGIC: c_uint = 0x25609513;
pub const NBD_REPLY_MAGIC: c_uint = 0x67446698;
// Do *not* use magics: 0x12560953 0x96744668.
// magic 0x668e33ef for structured reply not supported by kernel yet
//
// This is the packet used for communication between client and
// server. All data are in network byte order.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nbd_request {
    pub /: *mut *mut __be32 magic; / NBD_REQUEST_MAGIC,
    pub /: *mut *mut *mut __be32 type; / See NBD_CMD_,
    pub /: *mut *mut __be64 cookie; / Opaque identifier for request,
    pub /: *mut *mut char handle[8]; / older spelling of cookie,
}

//
// This is the reply packet that nbd-server sends back to the client after
// it has completed an I/O request (or an error occurs).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nbd_reply {
    pub /: *mut *mut __be32 magic; / NBD_REPLY_MAGIC,
    pub /: *mut *mut __be32 error; / 0 = ok, else error,
    pub /: *mut *mut __be64 cookie; / Opaque identifier from request,
    pub /: *mut *mut char handle[8]; / older spelling of cookie,
}
