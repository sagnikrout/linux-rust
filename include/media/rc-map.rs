//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/rc-map.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// rc-map.h - define RC map names used by RC drivers
//
// Copyright (c) 2010 by Mauro Carvalho Chehab
//

pub const __RC_PROTO_RC5_CODEC: c_int = 0;

pub const __RC_PROTO_JVC_CODEC: c_int = 0;

pub const __RC_PROTO_SONY_CODEC: c_int = 0;

pub const __RC_PROTO_NEC_CODEC: c_int = 0;

pub const __RC_PROTO_SANYO_CODEC: c_int = 0;

pub const __RC_PROTO_MCE_KBD_CODEC: c_int = 0;

pub const __RC_PROTO_RC6_CODEC: c_int = 0;

pub const __RC_PROTO_SHARP_CODEC: c_int = 0;

pub const __RC_PROTO_XMP_CODEC: c_int = 0;

pub const __RC_PROTO_IMON_CODEC: c_int = 0;

pub const __RC_PROTO_RCMM_CODEC: c_int = 0;

// All kernel-based codecs have encoders and decoders

//
// struct rc_map_table - represents a scancode/keycode pair
//
// @scancode: scan code (u64)
// @keycode: Linux input keycode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rc_map_table {
    pub scancode: u64,
    pub keycode: u32,
}

//
// struct rc_map - represents a keycode map table
//
// @scan: pointer to struct &rc_map_table
// @size: Max number of entries
// @len: Number of entries that are in use
// @alloc: size of \*scan, in bytes
// @rc_proto: type of the remote controller protocol, as defined at
// enum &rc_proto
// @name: name of the key map table
// @lock: lock to protect access to this structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rc_map {
    pub scan: *mut rc_map_table,
    pub size: c_uint,
    pub len: c_uint,
    pub alloc: c_uint,
    pub rc_proto: rc_proto,
    pub name: *const c_char,
    pub lock: spinlock_t,
}

//
// struct rc_map_list - list of the registered &rc_map maps
//
// @list: pointer to struct &list_head
// @map: pointer to struct &rc_map
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rc_map_list {
    pub list: list_head,
    pub map: rc_map,
}

//
// rc_map_list from rc-cec.c
//

// Routines from rc-map.c
//
// rc_map_register() - Registers a Remote Controller scancode map
//
// @map:	pointer to struct rc_map_list
//
extern "C" {
    pub fn rc_map_register(map: *mut rc_map_list) -> c_int;
}
//
// rc_map_unregister() - Unregisters a Remote Controller scancode map
//
// @map:	pointer to struct rc_map_list
//
extern "C" {
    pub fn rc_map_unregister(map: *mut rc_map_list);
}
//
// rc_map_get - gets an RC map from its name
// @name: name of the RC scancode map
//
// Names of the several keytables defined in-kernel

//
// Please, do not just append newer Remote Controller names at the end.
// The names should be ordered in alphabetical order
//
