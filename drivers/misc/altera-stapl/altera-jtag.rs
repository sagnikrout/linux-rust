//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/altera-stapl/altera-jtag.h
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
// altera-jtag.h
//
// altera FPGA driver
//
// Copyright (C) Altera Corporation 1998-2001
// Copyright (C) 2010 NetUP Inc.
// Copyright (C) 2010 Igor M. Liplianin <liplianin@netup.ru>
//
// Function Prototypes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum altera_jtag_state {
    ILLEGAL_JTAG_STATE = -1,
    RESET = 0,
    IDLE = 1,
    DRSELECT = 2,
    DRCAPTURE = 3,
    DRSHIFT = 4,
    DREXIT1 = 5,
    DRPAUSE = 6,
    DREXIT2 = 7,
    DRUPDATE = 8,
    IRSELECT = 9,
    IRCAPTURE = 10,
    IRSHIFT = 11,
    IREXIT1 = 12,
    IRPAUSE = 13,
    IREXIT2 = 14,
    IRUPDATE = 15

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct altera_jtag {
// Global variable to store the current JTAG state
    pub jtag_state: altera_jtag_state,
// Store current stop-state for DR and IR scan commands
    pub drstop_state: altera_jtag_state,
    pub irstop_state: altera_jtag_state,
// Store current padding values
    pub dr_pre: u32,
    pub dr_post: u32,
    pub ir_pre: u32,
    pub ir_post: u32,
    pub dr_length: u32,
    pub ir_length: u32,
    pub dr_pre_data: *mut u8,
    pub dr_post_data: *mut u8,
    pub ir_pre_data: *mut u8,
    pub ir_post_data: *mut u8,
    pub dr_buffer: *mut u8,
    pub ir_buffer: *mut u8,
}

pub const ALTERA_STACK_SIZE: c_int = 128;
pub const ALTERA_MESSAGE_LENGTH: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct altera_state {
    pub config: *mut altera_config,
    pub js: altera_jtag,
    pub 1]: char msg_buff[ALTERA_MESSAGE_LENGTH +,
    pub stack: [c_long; ALTERA_STACK_SIZE],
}

extern "C" {
    pub fn altera_jinit(astate: *mut altera_state) -> c_int;
}
extern "C" {
    pub fn altera_set_drstop(js: *mut altera_jtag, state: altera_jtag_state) -> c_int;
}
extern "C" {
    pub fn altera_set_irstop(js: *mut altera_jtag, state: altera_jtag_state) -> c_int;
}
extern "C" {
    pub fn altera_free_buffers(astate: *mut altera_state);
}
