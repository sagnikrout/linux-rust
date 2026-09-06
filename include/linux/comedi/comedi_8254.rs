//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/comedi/comedi_8254.h
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
// comedi_8254.h
// Generic 8254 timer/counter support
// Copyright (C) 2014 H Hartley Sweeten <hsweeten@visionengravers.com>
//
// COMEDI - Linux Control and Measurement Device Interface
// Copyright (C) 2000 David A. Schleef <ds@schleef.org>
//

//
// Common oscillator base values in nanoseconds
//
pub const I8254_OSC_BASE_10MHZ: c_int = 100;
pub const I8254_OSC_BASE_5MHZ: c_int = 200;
pub const I8254_OSC_BASE_4MHZ: c_int = 250;
pub const I8254_OSC_BASE_2MHZ: c_int = 500;
pub const I8254_OSC_BASE_1MHZ: c_int = 1000;
pub const I8254_OSC_BASE_100KHZ: c_int = 10000;
pub const I8254_OSC_BASE_10KHZ: c_int = 100000;
pub const I8254_OSC_BASE_1KHZ: c_int = 1000000;
//
// I/O access size used to read/write registers
//
pub const I8254_IO8: c_int = 1;
pub const I8254_IO16: c_int = 2;
pub const I8254_IO32: c_int = 4;
//
// Register map for generic 8254 timer (I8254_IO8 with 0 regshift)
//
pub const I8254_COUNTER0_REG: c_uint = 0x00;
pub const I8254_COUNTER1_REG: c_uint = 0x01;
pub const I8254_COUNTER2_REG: c_uint = 0x02;
pub const I8254_CTRL_REG: c_uint = 0x03;

// counter maps zero to 0x10000
pub const I8254_MAX_COUNT: c_uint = 0x10000;
//
// typedef comedi_8254_iocb_fn - call-back function type for 8254 register access
// @i8254:		pointer to struct comedi_8254
// @dir:		direction (0 = read, 1 = write)
// @reg:		register number
// @val:		value to write
//
// Return: Register value when reading, 0 when writing.
//
// struct comedi_8254 - private data used by this module
// @iocb:		I/O call-back function for register access
// @context:		context for register access (e.g. a base address)
// @iosize:		I/O size used to access the registers (b/w/l)
// @regshift:		register gap shift
// @osc_base:		cascaded oscillator speed in ns
// @divisor:		divisor for single counter
// @divisor1:		divisor loaded into first cascaded counter
// @divisor2:		divisor loaded into second cascaded counter
// @next_div:		next divisor for single counter
// @next_div1:		next divisor to use for first cascaded counter
// @next_div2:		next divisor to use for second cascaded counter
// @clock_src:		current clock source for each counter (driver specific)
// @gate_src:		current gate source  for each counter (driver specific)
// @busy:		flags used to indicate that a counter is "busy"
// @insn_config:	driver specific (*insn_config) callback
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct comedi_8254 {
    pub iocb: *mut comedi_8254_iocb_fn,
    pub context: c_ulong,
    pub iosize: c_uint,
    pub regshift: c_uint,
    pub osc_base: c_uint,
    pub divisor: c_uint,
    pub divisor1: c_uint,
    pub divisor2: c_uint,
    pub next_div: c_uint,
    pub next_div1: c_uint,
    pub next_div2: c_uint,
    pub clock_src: [c_uint; 3],
    pub gate_src: [c_uint; 3],
    pub busy: [bool; 3],
    pub data): *mut *mut comedi_insn insn, unsigned int,
}

extern "C" {
    pub fn comedi_8254_read(i8254: *mut comedi_8254, counter: c_uint) -> c_uint;
}
extern "C" {
    pub fn comedi_8254_update_divisors(i8254: *mut comedi_8254);
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENXIO) -> return;
}

