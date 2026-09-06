//! Automatically rewritten from C Header to Rust Module
//! Source: include/video/hecubafb.h
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
// hecubafb.h - definitions for the hecuba framebuffer driver
//
// Copyright (C) 2008 by Jaya Kumar
//
// This file is subject to the terms and conditions of the GNU General Public
// License. See the file COPYING in the main directory of this archive for
// more details.
//
// Apollo controller specific defines
pub const APOLLO_START_NEW_IMG: c_uint = 0xA0;
pub const APOLLO_STOP_IMG_DATA: c_uint = 0xA1;
pub const APOLLO_DISPLAY_IMG: c_uint = 0xA2;
pub const APOLLO_ERASE_DISPLAY: c_uint = 0xA3;
pub const APOLLO_INIT_DISPLAY: c_uint = 0xA4;
// Hecuba interface specific defines
pub const HCB_WUP_BIT: c_uint = 0x01;
pub const HCB_DS_BIT: c_uint = 0x02;
pub const HCB_RW_BIT: c_uint = 0x04;
pub const HCB_CD_BIT: c_uint = 0x08;
pub const HCB_ACK_BIT: c_uint = 0x80;
// struct used by hecuba. board specific stuff comes from *board
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hecubafb_par {
    pub info: *mut fb_info,
    pub board: *mut hecuba_board,
    pub char): *mut *mut *mut void (send_command)(struct hecubafb_par , unsigned,
    pub char): *mut *mut *mut void (send_data)(struct hecubafb_par , unsigned,
}

// board specific routines
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hecuba_board {
    pub owner: *mut module,
    pub ): *mut *mut void (remove)(struct hecubafb_par,
    pub char): *mut *mut *mut void (set_ctl)(struct hecubafb_par , unsigned char, unsigned,
    pub char): *mut *mut *mut void (set_data)(struct hecubafb_par , unsigned,
    pub int): *mut *mut *mut void (wait_for_ack)(struct hecubafb_par ,,
    pub ): *mut *mut int (init)(struct hecubafb_par,
}
