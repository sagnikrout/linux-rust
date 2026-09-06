//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/auto_fs.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// Copyright 1997 Transmeta Corporation - All Rights Reserved
// Copyright 1999-2000 Jeremy Fitzhardinge <jeremy@goop.org>
// Copyright 2005-2006,2013,2017-2018 Ian Kent <raven@themaw.net>
//
// This file is part of the Linux kernel and is made available under
// the terms of the GNU General Public License, version 2, or at your
// option, any later version, incorporated herein by reference.
//
// -----------------------------------------------------------------------

pub const AUTOFS_PROTO_VERSION: c_int = 5;
pub const AUTOFS_MIN_PROTO_VERSION: c_int = 3;
pub const AUTOFS_MAX_PROTO_VERSION: c_int = 5;
pub const AUTOFS_PROTO_SUBVERSION: c_int = 6;
//
// The wait_queue_token (autofs_wqt_t) is part of a structure which is passed
// back to the kernel via ioctl from userspace. On architectures where 32- and
// 64-bit userspace binaries can be executed it's important that the size of
// autofs_wqt_t stays constant between 32- and 64-bit Linux kernels so that we
// do not break the binary ABI interface by changing the structure size.
//

pub type autofs_wqt_t = c_ulong;

pub type autofs_wqt_t = c_uint;

// Packet types

#[repr(C)]
#[derive(Copy, Clone)]
pub struct autofs_packet_hdr {
    pub /: *mut *mut int proto_version; / Protocol version,
    pub /: *mut *mut int type; / Type of packet,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct autofs_packet_missing {
    pub hdr: autofs_packet_hdr,
    pub wait_queue_token: autofs_wqt_t,
    pub len: c_int,
    pub name: [c_char; NAME_MAX+1],
}

// v3 expire (via ioctl)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct autofs_packet_expire {
    pub hdr: autofs_packet_hdr,
    pub len: c_int,
    pub name: [c_char; NAME_MAX+1],
}

pub const AUTOFS_IOCTL: c_uint = 0x93;

// autofs version 4 and later definitions
// Mask for expire behaviour
pub const AUTOFS_EXP_NORMAL: c_uint = 0x00;
pub const AUTOFS_EXP_IMMEDIATE: c_uint = 0x01;
pub const AUTOFS_EXP_LEAVES: c_uint = 0x02;
pub const AUTOFS_EXP_FORCED: c_uint = 0x04;

// type = AUTOFS_TYPE_INDIRECT;
// type = AUTOFS_TYPE_DIRECT;
// type = AUTOFS_TYPE_OFFSET;
//
// This isn't really a type as we use it to say "no type set" to
// indicate we want to search for "any" mount in the
// autofs_dev_ioctl_ismountpoint() device ioctl function.
//
// type = AUTOFS_TYPE_ANY;
// Daemon notification packet types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum autofs_notify {
    NFY_NONE,
    NFY_MOUNT,
    NFY_EXPIRE
}

// Kernel protocol version 4 packet types
// Expire entry (umount request)
pub const autofs_ptype_expire_multi: c_int = 2;
// Kernel protocol version 5 packet types
// Indirect mount missing and expire requests.
pub const autofs_ptype_missing_indirect: c_int = 3;
pub const autofs_ptype_expire_indirect: c_int = 4;
// Direct mount missing and expire requests
pub const autofs_ptype_missing_direct: c_int = 5;
pub const autofs_ptype_expire_direct: c_int = 6;
// v4 multi expire (via pipe)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct autofs_packet_expire_multi {
    pub hdr: autofs_packet_hdr,
    pub wait_queue_token: autofs_wqt_t,
    pub len: c_int,
    pub name: [c_char; NAME_MAX+1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union autofs_packet_union {
    pub hdr: autofs_packet_hdr,
    pub missing: autofs_packet_missing,
    pub expire: autofs_packet_expire,
    pub expire_multi: autofs_packet_expire_multi,
}

// autofs v5 common packet struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct autofs_v5_packet {
    pub hdr: autofs_packet_hdr,
    pub wait_queue_token: autofs_wqt_t,
    pub dev: __u32,
    pub ino: __u64,
    pub uid: __u32,
    pub gid: __u32,
    pub pid: __u32,
    pub tgid: __u32,
    pub len: __u32,
    pub name: [c_char; NAME_MAX+1],
}

pub type autofs_packet_missing_indirect_t = autofs_v5_packet;
pub type autofs_packet_expire_indirect_t = autofs_v5_packet;
pub type autofs_packet_missing_direct_t = autofs_v5_packet;
pub type autofs_packet_expire_direct_t = autofs_v5_packet;
#[repr(C)]
#[derive(Copy, Clone)]
pub union autofs_v5_packet_union {
    pub hdr: autofs_packet_hdr,
    pub v5_packet: autofs_v5_packet,
    pub missing_indirect: autofs_packet_missing_indirect_t,
    pub expire_indirect: autofs_packet_expire_indirect_t,
    pub missing_direct: autofs_packet_missing_direct_t,
    pub expire_direct: autofs_packet_expire_direct_t,
}

