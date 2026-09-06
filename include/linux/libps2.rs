//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/libps2.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 1999-2002 Vojtech Pavlik
// Copyright (C) 2004 Dmitry Torokhov
//

//
// enum ps2_disposition - indicates how received byte should be handled
// @PS2_PROCESS: pass to the main protocol handler, process normally
// @PS2_IGNORE: skip the byte
// @PS2_ERROR: do not process the byte, abort command in progress
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ps2_disposition {
    PS2_PROCESS,
    PS2_IGNORE,
    PS2_ERROR,
}

extern "C" {
    pub fn void(: *mut *mut ps2_receive_handler_t)(struct ps2dev, _arg: u8) -> typedef;
}
//
// struct ps2dev - represents a device using PS/2 protocol
// @serio: a serio port used by the PS/2 device
// @cmd_mutex: a mutex ensuring that only one command is executing at a time
// @wait: a waitqueue used to signal completion from the serio interrupt handler
// @flags: various internal flags indicating stages of PS/2 command execution
// @cmdbuf: buffer holding command response
// @cmdcnt: outstanding number of bytes of the command response
// @nak: a byte transmitted by the device when it refuses command
// @pre_receive_handler: checks communication errors and returns disposition
// (&enum ps2_disposition) of the received data byte
// @receive_handler: main handler of particular PS/2 protocol, such as keyboard
// or mouse protocol
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps2dev {
    pub serio: *mut serio,
    pub cmd_mutex: mutex,
    pub wait: wait_queue_head_t,
    pub flags: c_ulong,
    pub cmdbuf: [u8; 8],
    pub cmdcnt: u8,
    pub nak: u8,
    pub pre_receive_handler: ps2_pre_receive_handler_t,
    pub receive_handler: ps2_receive_handler_t,
}

extern "C" {
    pub fn ps2_sendbyte(ps2dev: *mut ps2dev, byte: u8, timeout: c_uint) -> c_int;
}
extern "C" {
    pub fn ps2_drain(ps2dev: *mut ps2dev, maxbytes: usize, timeout: c_uint);
}
extern "C" {
    pub fn ps2_begin_command(ps2dev: *mut ps2dev);
}
extern "C" {
    pub fn ps2_end_command(ps2dev: *mut ps2dev);
}
extern "C" {
    pub fn __ps2_command(ps2dev: *mut ps2dev, param: *mut u8, command: c_uint) -> c_int;
}
extern "C" {
    pub fn ps2_command(ps2dev: *mut ps2dev, param: *mut u8, command: c_uint) -> c_int;
}
extern "C" {
    pub fn ps2_sliced_command(ps2dev: *mut ps2dev, command: u8) -> c_int;
}
extern "C" {
    pub fn ps2_is_keyboard_id(id: u8) -> bool;
}
extern "C" {
    pub fn ps2_interrupt(serio: *mut serio, data: u8, flags: c_uint) -> irqreturn_t;
}
