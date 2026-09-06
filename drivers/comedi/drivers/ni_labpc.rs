//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/comedi/drivers/ni_labpc.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Header for ni_labpc ISA/PCMCIA/PCI drivers
//
// Copyright (C) 2003 Frank Mori Hess <fmhess@users.sourceforge.net>
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum transfer_type {
    isa_dma_transfer
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct labpc_boardinfo {
    pub name: *const c_char,
    pub /: *mut *mut int ai_speed; / maximum input speed in ns,
    pub /: *mut *mut unsigned ai_scan_up:1; / can auto scan up in ai channels,
    pub /: *mut *mut unsigned has_ao:1; / has analog outputs,
    pub /: *mut *mut unsigned is_labpc1200:1; / has extra regs compared to pc+,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct labpc_private {
    pub dma: *mut comedi_isadma,
    pub counter: *mut comedi_8254,
// number of data points left to be taken
    pub count: c_ulonglong,
// software copys of bits written to command registers
    pub cmd1: c_uint,
    pub cmd2: c_uint,
    pub cmd3: c_uint,
    pub cmd4: c_uint,
    pub cmd5: c_uint,
    pub cmd6: c_uint,
// store last read of board status registers
    pub stat1: c_uint,
    pub stat2: c_uint,
// we are using dma/fifo-half-full/etc.
    pub current_transfer: transfer_type,
//
// function pointers so we can use inb/outb or readb/writeb as
// appropriate
//
    pub reg): *mut *mut *mut unsigned int (read_byte)(struct comedi_device dev, unsigned long,
    pub reg): unsigned int byte, unsigned long,
}

extern "C" {
    pub fn labpc_common_detach(dev: *mut comedi_device);
}
