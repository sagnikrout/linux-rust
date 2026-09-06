//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/block/rbd_types.h
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


//
// Ceph - scalable distributed file system
//
// Copyright (C) 2004-2010 Sage Weil <sage@newdream.net>
//
// This is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License version 2.1, as published by the Free Software
// Foundation.  See file COPYING.
//

// For format version 2, rbd image 'foo' consists of objects
// rbd_id.foo		- id of image
// rbd_header.<id>	- image metadata
// rbd_object_map.<id> - optional image object map
// rbd_data.<id>.0000000000000000
// rbd_data.<id>.0000000000000001
// ...		- data
// Clients do not access header data directly in rbd format 2.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rbd_notify_op {
    RBD_NOTIFY_OP_ACQUIRED_LOCK      = 0,
    RBD_NOTIFY_OP_RELEASED_LOCK      = 1,
    RBD_NOTIFY_OP_REQUEST_LOCK       = 2,
    RBD_NOTIFY_OP_HEADER_UPDATE      = 3,
}

pub const OBJECT_NONEXISTENT: c_int = 0;
pub const OBJECT_EXISTS: c_int = 1;
pub const OBJECT_PENDING: c_int = 2;
pub const OBJECT_EXISTS_CLEAN: c_int = 3;

//
// For format version 1, rbd image 'foo' consists of objects
// foo.rbd		- image metadata
// rb.<idhi>.<idlo>.<extra>.000000000000
// rb.<idhi>.<idlo>.<extra>.000000000001
// ...		- data
// There is no notion of a persistent image id in rbd format 1.
//

pub const RBD_MIN_OBJ_ORDER: c_int = 16;
pub const RBD_MAX_OBJ_ORDER: c_int = 30;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rbd_image_snap_ondisk {
    pub id: __le64,
    pub image_size: __le64,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rbd_image_header_ondisk {
    pub text: [c_char; 40],
    pub object_prefix: [c_char; 24],
    pub signature: [c_char; 4],
    pub version: [c_char; 8],
    pub order: __u8,
    pub crypt_type: __u8,
    pub comp_type: __u8,
    pub unused: __u8,
// C attribute field omitted
    pub image_size: __le64,
    pub snap_seq: __le64,
    pub snap_count: __le32,
    pub reserved: __le32,
    pub snap_names_len: __le64,
    pub snaps: [rbd_image_snap_ondisk; ],
    pub __attribute__((packed)): },
