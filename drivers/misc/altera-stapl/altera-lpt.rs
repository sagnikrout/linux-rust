//! Automatically rewritten from C to Rust
//! Source: drivers/misc/altera-stapl/altera-lpt.c
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
// altera-lpt.c
//
// altera FPGA driver
//
// Copyright (C) Altera Corporation 1998-2001
// Copyright (C) 2010 NetUP Inc.
// Copyright (C) 2010 Abylay Ospan <aospan@netup.ru>
//

    static int lpt_hardware_initialized;
#[no_mangle]
unsafe extern "C" fn byteblaster_write(port: c_int, data: c_int) {
    static void byteblaster_write(int port, int data)
    {
    outb((u8)data, (u16)(port + 0x378));
    };
#[no_mangle]
unsafe extern "C" fn byteblaster_read(port: c_int) -> c_int {
    static int byteblaster_read(int port)
    {
    let mut data: c_int = 0;
    data = inb((u16)(port + 0x378));
    return data & 0xff;
    };
#[no_mangle]
pub unsafe extern "C" fn netup_jtag_io_lpt(device: *mut c_void, tms: c_int, tdi: c_int, read_tdo: c_int) -> c_int {
    int netup_jtag_io_lpt(void *device, int tms, int tdi, int read_tdo)
    {
    let mut data: c_int = 0;
    let mut tdo: c_int = 0;
    let mut initial_lpt_ctrl: c_int = 0;
    if (!lpt_hardware_initialized) {
    initial_lpt_ctrl = byteblaster_read(2);
    byteblaster_write(2, (initial_lpt_ctrl | 0x02) & 0xdf);
    lpt_hardware_initialized = 1;
    }
    data = ((tdi ? 0x40 : 0) | (tms ? 0x02 : 0));
    byteblaster_write(0, data);
    if (read_tdo) {
    tdo = byteblaster_read(1);
    tdo = ((tdo & 0x80) ? 0 : 1);
    }
    byteblaster_write(0, data | 0x01);
    byteblaster_write(0, data);
    return tdo;
    }
