//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/timex.h
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


// SPDX-License-Identifier: GPL-2.0
//
// S390 version
// Copyright IBM Corp. 1999
//
// Derived from "include/asm-i386/timex.h"
// Copyright (C) 1992, Linus Torvalds
//

// The value of the TOD clock for 1.1.1970.
pub const TOD_UNIX_EPOCH: c_uint = 0x7d91048bca000000ULL;
// Inline functions for clock register access.
extern "C" {
    pub fn CC_TRANSFORM(_arg: cc) -> return;
}
extern "C" {
    pub fn CC_TRANSFORM(_arg: cc) -> return;
}
extern "C" {
    pub fn volatile("cc": *mut *mut "stcke %0" : "=Q" (tod) : :) -> asm;
}
extern "C" {
    pub fn volatile((time): "sckc %0" : : "Q") -> asm;
}
extern "C" {
    pub fn clock_comparator_work();
}
extern "C" {
    pub fn time_early_init() -> void __init;
}
// Function codes for the ptff instruction.
pub const PTFF_QAF: c_uint = 0x00	/* query available functions */;
pub const PTFF_QTO: c_uint = 0x01	/* query tod offset */;
pub const PTFF_QSI: c_uint = 0x02	/* query steering information */;
pub const PTFF_QPT: c_uint = 0x03	/* query physical clock */;
pub const PTFF_QUI: c_uint = 0x04	/* query UTC information */;
pub const PTFF_ATO: c_uint = 0x40	/* adjust tod offset */;
pub const PTFF_STO: c_uint = 0x41	/* set tod offset */;
pub const PTFF_SFS: c_uint = 0x42	/* set fine steering rate */;
pub const PTFF_SGS: c_uint = 0x43	/* set gross steering rate */;
// Query TOD offset result
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptff_qto {
    pub physical_clock: c_ulong,
    pub tod_offset: c_ulong,
    pub logical_tod_offset: c_ulong,
    pub tod_epoch_difference: c_ulong,
    pub __packed: },
    pub ptr: *mut c_uchar,
    pub 3): ptr = ptff_function_mask + (nr >>,
    pub 0: *mut *mut return (ptr & (0x80 >> (nr & 7))) !=,
// Query UTC information result
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptff_qui {
    pub 2: unsigned int tm :,
    pub 2: unsigned int ts :,
    pub 28: unsigned int :,
    pub pad_0x04: c_uint,
    pub leap_event: c_ulong,
    pub old_leap: c_short,
    pub new_leap: c_short,
    pub pad_0x14: c_uint,
    pub prt: [c_ulong; 5],
    pub cst: [c_ulong; 3],
    pub skew: c_uint,
    pub pad_0x5c: [c_uint; 41],
    pub __packed: },
//
// ptff - Perform timing facility function
// @ptff_block: Pointer to ptff parameter block
// @len: Length of parameter block
// @func: Function code
// Returns: Condition code (0 on success)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct addrtype {
    pub \: unsigned int reg0 = func;,
    pub \: unsigned long reg1 = (unsigned long)(ptff_block);,
    pub \: int rc;,
    pub \: : CC_CLOBBER_LIST("0", "1"));,
    pub \: CC_TRANSFORM(rc);,
    pub old: c_ulong,
    pub get_lowcore()->clock_comparator: old =,
    pub clock_comparator_max: get_lowcore()->clock_comparator =,
    pub old: return,
    pub comp: get_lowcore()->clock_comparator =,
pub type cycles_t = c_ulong;
    pub clk: tod_clock,
    pub clk.tod: return,
    pub clk: c_ulong,
    pub "cc"): asm volatile("stckf %0" : "=Q" (clk) : :,
    pub clk: return,
    pub clock): *mut int get_phys_clock(unsigned long,
    pub init_cpu_timer(void): c_void,
    pub tod_clock_base: extern union tod_clock,
    pub tod_clock_base.tod: return get_tod_clock() -,
//
// get_clock_monotonic - returns current time in clock rate units
//
// The clock and tod_clock_base get changed via stop_machine.
// Therefore preemption must be disabled, otherwise the returned
// value is not guaranteed to be monotonic.
//
    pub tod: c_ulong,
    pub __get_tod_clock_monotonic(): tod =,
    pub tod: return,
    pub 2: return (cycles_t)get_tod_clock_monotonic() >>,

//
// tod_to_ns - convert a TOD format value to nanoseconds
// @todval: to be converted TOD format value
// Returns: number of nanoseconds that correspond to the TOD format value
//
// Converting a 64 Bit TOD format value to nanoseconds means that the value
// must be divided by 4.096. In order to achieve that we multiply with 125
// and divide by 512:
//
// ns = (todval * 125) >> 9;
//
// In order to avoid an overflow with the multiplication we can rewrite this.
// With a split todval == 2^9 * th + tl (th upper 55 bits, tl lower 9 bits)
// we end up with
//
// ns = ((2^9 * th + tl) * 125 ) >> 9;
// -> ns = (th * 125) + ((tl * 125) >> 9);
//
    pub 9): *mut *mut *mut return ((todval >> 9)  125) + (((todval & 0x1ff)  125) >>,
    pub 9: *mut *mut return (todval  125) >>,
//
// tod_after - compare two 64 bit TOD values
// @a: first 64 bit TOD timestamp
// @b: second 64 bit TOD timestamp
//
// Returns: true if a is later than b
//
    pub b: return (long) a > (long),
    pub b: return a >,
//
// tod_after_eq - compare two 64 bit TOD values
// @a: first 64 bit TOD timestamp
// @b: second 64 bit TOD timestamp
//
// Returns: true if a is later than b
//
    pub b: return (long) a >= (long),
    pub b: return a >=,
