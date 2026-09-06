//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/uapi/asm/mce.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

//
// Fields are zero when not available. Also, this struct is shared with
// userspace mcelog and thus must keep existing fields at current offsets.
// Only add new, shared fields to the end of the structure.
// Do not add vendor-specific fields.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mce {
    pub /: *mut *mut __u64 status; / Bank's MCi_STATUS MSR,
    pub /: *mut *mut __u64 misc; / Bank's MCi_MISC MSR,
    pub /: *mut *mut __u64 addr; / Bank's MCi_ADDR MSR,
    pub /: *mut *mut __u64 mcgstatus; / Machine Check Global Status MSR,
    pub /: *mut *mut __u64 ip; / Instruction Pointer when the error happened,
    pub /: *mut *mut __u64 tsc; / CPU time stamp counter,
    pub /: *mut *mut __u64 time; / Wall time_t when error was detected,
    pub /: *mut *mut __u8 cpuvendor; / Kernel's X86_VENDOR enum,
    pub /: *mut *mut __u8 inject_flags; / Software inject flags,
    pub /: *mut *mut __u8 severity; / Error severity,
    pub pad: __u8,
    pub /: *mut *mut __u32 cpuid; / CPUID 1 EAX,
    pub /: *mut *mut __u8 cs; / Code segment,
    pub /: *mut *mut __u8 bank; / Machine check bank reporting the error,
    pub /: *mut *mut __u8 cpu; / CPU number; obsoleted by extcpu,
    pub /: *mut *mut __u8 finished; / Entry is valid,
    pub /: *mut *mut __u32 extcpu; / Linux CPU number that detected the error,
    pub /: *mut *mut __u32 socketid; / CPU socket ID,
    pub /: *mut *mut __u32 apicid; / CPU initial APIC ID,
    pub /: *mut *mut __u64 mcgcap; / MCGCAP MSR: machine check capabilities of CPU,
    pub /: *mut *mut __u64 synd; / MCA_SYND MSR: only valid on SMCA systems,
    pub /: *mut *mut __u64 ipid; / MCA_IPID MSR: only valid on SMCA systems,
    pub /: *mut *mut __u64 ppin; / Protected Processor Inventory Number,
    pub /: *mut *mut __u32 microcode; / Microcode revision,
    pub /: *mut *mut __u64 kflags; / Internal kernel use,
}

