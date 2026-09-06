//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/ctlreg.h
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
// Copyright IBM Corp. 1999, 2009
//
// Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctlreg {
    pub val: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct addrtype {
    pub \: char _[sizeof(array)];,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct addrtype {
    pub \: char _[sizeof(array)];,
}

extern "C" {
    pub fn system_ctlreg_lock();
}
extern "C" {
    pub fn system_ctlreg_unlock();
}
extern "C" {
    pub fn system_ctlreg_init_save_area(lc: *mut lowcore);
}
extern "C" {
    pub fn system_ctlreg_modify(cr: c_uint, data: c_ulong, request: c_int);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ctlreg0 {
    pub val: c_ulong,
    pub reg: ctlreg,
    pub 8: unsigned long :,
    pub /: *mut *mut unsigned long tcx : 1; / Transactional-Execution control,
    pub Program-: *mut *mut unsigned long pifo : 1; / Transactional-Execution,
    pub 3: unsigned long :,
    pub /: *mut *mut unsigned long ccc : 1; / Cryptography counter control,
    pub /: *mut *mut unsigned long pec : 1; / PAI extension control,
    pub 15: unsigned long :,
    pub /: *mut *mut unsigned long wti : 1; / Warning-track,
    pub 4: unsigned long :,
    pub /: *mut *mut unsigned long lap : 1; / Low-address-protection control,
    pub 4: unsigned long :,
    pub /: *mut *mut unsigned long edat : 1; / Enhanced-DAT-enablement control,
    pub 2: unsigned long :,
    pub /: *mut *mut unsigned long iep : 1; / Instruction-Execution-Protection,
    pub 1: unsigned long :,
    pub /: *mut *mut unsigned long afp : 1; / AFP-register control,
    pub /: *mut *mut unsigned long vx : 1; / Vector enablement control,
    pub 7: unsigned long :,
    pub /: *mut *mut unsigned long sssm : 1; / Service signal subclass mask,
    pub 9: unsigned long :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ctlreg2 {
    pub val: c_ulong,
    pub reg: ctlreg,
    pub 33: unsigned long :,
    pub 25: unsigned long ducto :,
    pub 1: unsigned long :,
    pub 1: unsigned long gse :,
    pub 1: unsigned long :,
    pub 1: unsigned long tds :,
    pub 2: unsigned long tdc :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ctlreg5 {
    pub val: c_ulong,
    pub reg: ctlreg,
    pub 33: unsigned long :,
    pub 25: unsigned long pasteo:,
    pub 6: unsigned long :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ctlreg15 {
    pub val: c_ulong,
    pub reg: ctlreg,
    pub 61: unsigned long lsea :,
    pub 3: unsigned long :,
}

