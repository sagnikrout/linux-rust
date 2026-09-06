//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/dvb/ca.h
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


// SPDX-License-Identifier: LGPL-2.1+ WITH Linux-syscall-note
//
// ca.h
//
// Copyright (C) 2000 Ralph  Metzler <ralph@convergence.de>
// & Marcus Metzler <marcus@convergence.de>
// for convergence integrated media GmbH
//
// struct ca_slot_info - CA slot interface types and info.
//
// @num:	slot number.
// @type:	slot type.
// @flags:	flags applicable to the slot.
//
// This struct stores the CA slot information.
//
// @type can be:
//
// - %CA_CI - CI high level interface;
// - %CA_CI_LINK - CI link layer level interface;
// - %CA_CI_PHYS - CI physical layer level interface;
// - %CA_DESCR - built-in descrambler;
// - %CA_SC -simple smart card interface.
//
// @flags can be:
//
// - %CA_CI_MODULE_PRESENT - module (or card) inserted;
// - %CA_CI_MODULE_READY - module is ready for usage.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ca_slot_info {
    pub num: c_int,
    pub type: c_int,
pub const CA_CI: c_int = 1;
pub const CA_CI_LINK: c_int = 2;
pub const CA_CI_PHYS: c_int = 4;
pub const CA_DESCR: c_int = 8;
pub const CA_SC: c_int = 128;
    pub flags: c_uint,
pub const CA_CI_MODULE_PRESENT: c_int = 1;
pub const CA_CI_MODULE_READY: c_int = 2;
}

//
// struct ca_descr_info - descrambler types and info.
//
// @num:	number of available descramblers (keys).
// @type:	type of supported scrambling system.
//
// Identifies the number of descramblers and their type.
//
// @type can be:
//
// - %CA_ECD - European Common Descrambler (ECD) hardware;
// - %CA_NDS - Videoguard (NDS) hardware;
// - %CA_DSS - Distributed Sample Scrambling (DSS) hardware.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ca_descr_info {
    pub num: c_uint,
    pub type: c_uint,
pub const CA_ECD: c_int = 1;
pub const CA_NDS: c_int = 2;
pub const CA_DSS: c_int = 4;
}

//
// struct ca_caps - CA slot interface capabilities.
//
// @slot_num:	total number of CA card and module slots.
// @slot_type:	bitmap with all supported types as defined at
// &struct ca_slot_info (e. g. %CA_CI, %CA_CI_LINK, etc).
// @descr_num:	total number of descrambler slots (keys)
// @descr_type:	bitmap with all supported types as defined at
// &struct ca_descr_info (e. g. %CA_ECD, %CA_NDS, etc).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ca_caps {
    pub slot_num: c_uint,
    pub slot_type: c_uint,
    pub descr_num: c_uint,
    pub descr_type: c_uint,
}

//
// struct ca_msg - a message to/from a CI-CAM
//
// @index:	unused
// @type:	unused
// @length:	length of the message
// @msg:	message
//
// This struct carries a message to be send/received from a CI CA module.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ca_msg {
    pub index: c_uint,
    pub type: c_uint,
    pub length: c_uint,
    pub msg: [c_uchar; 256],
}

//
// struct ca_descr - CA descrambler control words info
//
// @index: CA Descrambler slot
// @parity: control words parity, where 0 means even and 1 means odd
// @cw: CA Descrambler control words
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ca_descr {
    pub index: c_uint,
    pub parity: c_uint,
    pub cw: [c_uchar; 8],
}

// This is needed for legacy userspace support
pub type ca_slot_info_t = ca_slot_info;
pub type ca_descr_info_t = ca_descr_info;
pub type ca_caps_t = ca_caps;
pub type ca_msg_t = ca_msg;
pub type ca_descr_t = ca_descr;

