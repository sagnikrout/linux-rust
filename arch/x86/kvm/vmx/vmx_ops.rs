//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/vmx/vmx_ops.h
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

extern "C" {
    pub fn vmread_error(field: c_ulong);
}
extern "C" {
    pub fn vmwrite_error(field: c_ulong, value: c_ulong);
}
extern "C" {
    pub fn vmclear_error(vmcs: *mut vmcs, phys_addr: u64);
}
extern "C" {
    pub fn vmptrld_error(vmcs: *mut vmcs, phys_addr: u64);
}
extern "C" {
    pub fn invvpid_error(ext: c_ulong, vpid: u16, gva: gva_t);
}
extern "C" {
    pub fn invept_error(ext: c_ulong, eptp: u64);
}

//
// The VMREAD error trampoline _always_ uses the stack to pass parameters, even
// for 64-bit targets.  Preserving all registers allows the VMREAD inline asm
// blob to avoid clobbering GPRs, which in turn allows the compiler to better
// optimize sequences of VMREADs.
//
// Declare the trampoline as an opaque label as it's not safe to call from C
// code; there is no way to tell the compiler to pass params on the stack for
// 64-bit targets.
//
// void vmread_error_trampoline(unsigned long field, bool fault);
//
// The second VMREAD error trampoline, called from the assembly trampoline,
// exists primarily to enable instrumentation for the VM-Fail path.
//
extern "C" {
    pub fn vmread_error_trampoline2(field: c_ulong, fault: bool);
}

//
// VMREAD failed.  Push '0' for @fault, push the failing
// @field, and bounce through the trampoline to preserve
// volatile registers.
//
// Unwind the stack.  Note, the trampoline zeros out the
// memory for @fault so that the result is '0' on error.
//
// VMREAD faulted.  As above, except push '1' for @fault.

extern "C" {
    pub fn evmcs_read16(_arg: field) -> return;
}
extern "C" {
    pub fn __vmcs_readl(_arg: field) -> return;
}
extern "C" {
    pub fn evmcs_read32(_arg: field) -> return;
}
extern "C" {
    pub fn __vmcs_readl(_arg: field) -> return;
}
extern "C" {
    pub fn evmcs_read64(_arg: field) -> return;
}

extern "C" {
    pub fn __vmcs_readl(_arg: field) -> return;
}

extern "C" {
    pub fn __vmcs_readl(32: field) | ((u64)__vmcs_readl(field+1) <<) -> return;
}

extern "C" {
    pub fn evmcs_read64(_arg: field) -> return;
}
extern "C" {
    pub fn __vmcs_readl(_arg: field) -> return;
}

extern "C" {
    pub fn evmcs_write16(_arg: field, _arg: value) -> return;
}
extern "C" {
    pub fn evmcs_write32(_arg: field, _arg: value) -> return;
}
extern "C" {
    pub fn evmcs_write64(_arg: field, _arg: value) -> return;
}

extern "C" {
    pub fn evmcs_write64(_arg: field, _arg: value) -> return;
}
extern "C" {
    pub fn evmcs_write32(_arg: field, ~mask: evmcs_read32(field) &) -> return;
}
extern "C" {
    pub fn evmcs_write32(_arg: field, mask: evmcs_read32(field) |) -> return;
}
extern "C" {
    pub fn evmcs_load(_arg: phys_addr) -> return;
}
