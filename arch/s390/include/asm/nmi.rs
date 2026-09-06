//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/nmi.h
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
// Machine check handler definitions
//
// Copyright IBM Corp. 2000, 2009
// Author(s): Ingo Adlung <adlung@de.ibm.com>,
// Martin Schwidefsky <schwidefsky@de.ibm.com>,
// Cornelia Huck <cornelia.huck@de.ibm.com>,
//

#[repr(C)]
#[derive(Copy, Clone)]
pub union mci {
    pub val: c_ulong,
    pub /: *mut *mut u64 sd : 1; / 00 system damage,
    pub /: *mut *mut u64 pd : 1; / 01 instruction-processing damage,
    pub /: *mut *mut u64 sr : 1; / 02 system recovery,
    pub /: *mut *mut u64 : 1; / 03,
    pub /: *mut *mut u64 cd : 1; / 04 timing-facility damage,
    pub /: *mut *mut u64 ed : 1; / 05 external damage,
    pub /: *mut *mut u64 : 1; / 06,
    pub /: *mut *mut u64 dg : 1; / 07 degradation,
    pub /: *mut *mut u64 w : 1; / 08 warning pending,
    pub /: *mut *mut u64 cp : 1; / 09 channel-report pending,
    pub /: *mut *mut u64 sp : 1; / 10 service-processor damage,
    pub /: *mut *mut u64 ck : 1; / 11 channel-subsystem damage,
    pub /: *mut *mut u64 : 2; / 12-13,
    pub /: *mut *mut u64 b : 1; / 14 backed up,
    pub /: *mut *mut u64 : 1; / 15,
    pub /: *mut *mut u64 se : 1; / 16 storage error uncorrected,
    pub /: *mut *mut u64 sc : 1; / 17 storage error corrected,
    pub /: *mut *mut u64 ke : 1; / 18 storage-key error uncorrected,
    pub /: *mut *mut u64 ds : 1; / 19 storage degradation,
    pub /: *mut *mut u64 wp : 1; / 20 psw mwp validity,
    pub /: *mut *mut u64 ms : 1; / 21 psw mask and key validity,
    pub /: *mut *mut u64 pm : 1; / 22 psw program mask and cc validity,
    pub /: *mut *mut u64 ia : 1; / 23 psw instruction address validity,
    pub /: *mut *mut u64 fa : 1; / 24 failing storage address validity,
    pub /: *mut *mut u64 vr : 1; / 25 vector register validity,
    pub /: *mut *mut u64 ec : 1; / 26 external damage code validity,
    pub /: *mut *mut u64 fp : 1; / 27 floating point register validity,
    pub /: *mut *mut u64 gr : 1; / 28 general register validity,
    pub /: *mut *mut u64 cr : 1; / 29 control register validity,
    pub /: *mut *mut u64 : 1; / 30,
    pub /: *mut *mut u64 st : 1; / 31 storage logical validity,
    pub /: *mut *mut u64 ie : 1; / 32 indirect storage error,
    pub /: *mut *mut u64 ar : 1; / 33 access register validity,
    pub /: *mut *mut u64 da : 1; / 34 delayed access exception,
    pub /: *mut *mut u64 : 1; / 35,
    pub /: *mut *mut u64 gs : 1; / 36 guarded storage registers validity,
    pub /: *mut *mut u64 : 5; / 37-41,
    pub /: *mut *mut u64 pr : 1; / 42 tod programmable register validity,
    pub /: *mut *mut u64 fc : 1; / 43 fp control register validity,
    pub /: *mut *mut u64 ap : 1; / 44 ancillary report,
    pub /: *mut *mut u64 : 1; / 45,
    pub /: *mut *mut u64 ct : 1; / 46 cpu timer validity,
    pub /: *mut *mut u64 cc : 1; / 47 clock comparator validity,
    pub /: *mut *mut u64 : 16; / 47-63,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcesa {
    pub vector_save_area: [u8; 1024],
    pub guarded_storage_save_area: [u8; 32],
}

extern "C" {
    pub fn nmi_alloc_mcesa_early(mcesad: *mut u64);
}
extern "C" {
    pub fn nmi_alloc_mcesa(mcesad: *mut u64) -> c_int;
}
extern "C" {
    pub fn nmi_free_mcesa(mcesad: *mut u64);
}
extern "C" {
    pub fn s390_handle_mcck();
}
extern "C" {
    pub fn s390_do_machine_check(regs: *mut pt_regs);
}

