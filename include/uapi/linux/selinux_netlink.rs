//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/selinux_netlink.h
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
// Netlink event notifications for SELinux.
//
// Author: James Morris <jmorris@redhat.com>
//
// Copyright (C) 2004 Red Hat, Inc., James Morris <jmorris@redhat.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2,
// as published by the Free Software Foundation.
//

// Message types.
pub const SELNL_MSG_BASE: c_uint = 0x10;
// Multicast groups - backwards compatiblility for userspace
pub const SELNL_GRP_NONE: c_uint = 0x00000000;
pub const SELNL_GRP_AVC: c_uint = 0x00000001	/* AVC notifications */;
pub const SELNL_GRP_ALL: c_uint = 0xffffffff;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum selinux_nlgroups {
    SELNLGRP_NONE,

    SELNLGRP_AVC,

    __SELNLGRP_MAX
}

// Message structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct selnl_msg_setenforce {
    pub val: __s32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct selnl_msg_policyload {
    pub seqno: __u32,
}
