//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/userio.h
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
// userio: virtual serio device support
// Copyright (C) 2015 Red Hat
// Copyright (C) 2015 Lyude Paul <thatslyude@gmail.com>
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU Lesser General Public License as published by the
// Free Software Foundation; either version 2 of the License, or (at your
// option) any later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more
// details.
//
// This is the public header used for user-space communication with the userio
// driver. __attribute__((__packed__)) is used for all structs to keep ABI
// compatibility between all architectures.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum userio_cmd_type {
    USERIO_CMD_REGISTER = 0,
    USERIO_CMD_SET_PORT_TYPE = 1,
    USERIO_CMD_SEND_INTERRUPT = 2,
    USERIO_CMD_SET_PORT_EXTRA = 3,
    USERIO_CMD_SET_PORT_ID = 4,
    USERIO_CMD_SET_PORT_PROTO = 5,
}

//
// userio Commands
// All commands sent to /dev/userio are encoded using this structure. The type
// field should contain a USERIO_CMD* value that indicates what kind of command
// is being sent to userio. The data field should contain the accompanying
// argument for the command, if there is one.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct userio_cmd {
    pub type: __u8,
    pub data: __u8,
    pub __attribute__((__packed__)): },
