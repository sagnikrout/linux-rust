//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/inst.h
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
// Instruction data type for POWER
//

extern "C" {
    pub fn ppc_inst_prefix(_arg: *mut ptr, 1): *mut *mut (ptr +) -> return;
}
extern "C" {
    pub fn ppc_inst(_arg: *mut ptr) -> return;
}
extern "C" {
    pub fn ppc_inst_prefix(_arg: swab32(ppc_inst_val(x)), _arg: swab32(ppc_inst_suffix(x))) -> return;
}
extern "C" {
    pub fn ppc_inst_suffix(ppc_inst_suffix(y: x) ==) -> return;
}
//
// Return the address of the next instruction, if the instruction @value was
// located at @location.
//
extern "C" {
    pub fn ppc_inst_val(_arg: x) -> return;
}
// ptr = ppc_inst_val(x);
// (u64 *)ptr = ppc_inst_as_ulong(x);
// inst = ppc_inst_prefix(val, suffix);
// inst = ppc_inst(val);
extern "C" {
    pub fn __copy_inst_from_kernel_nofault(_arg: inst, _arg: src) -> return;
}
