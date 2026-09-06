//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/ibt.h
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
// The rules for enabling IBT are:
//
// - CC_HAS_IBT:         the toolchain supports it
// - X86_KERNEL_IBT:     it is selected in Kconfig
// - !__DISABLE_EXPORTS: this is regular kernel code
//
// Esp. that latter one is a bit non-obvious, but some code like compressed,
// purgatory, realmode etc.. is built with custom CFLAGS that do not include
// -fcf-protection=branch and things will go *bang*.
//
// When all the above are satisfied, HAS_KERNEL_IBT will be 1, otherwise 0.
//

pub const HAS_KERNEL_IBT: c_int = 1;

//
// Create a dummy function pointer reference to prevent objtool from marking
// the function as needing to be "sealed" (i.e. ENDBR converted to NOP by
// apply_seal_endbr()).
//

//
// Generate ENDBR64 in a way that is sure to not result in
// an ENDBR64 instruction as immediate.
//
// 4 byte NOP that isn't NOP4, such that it will be unique to (former)
// ENDBR sites. Additionally it carries UDB as immediate.
//
extern "C" {
    pub fn is_endbr(val: *mut u32) -> __noendbr bool;
}
extern "C" {
    pub fn ibt_save(disable: bool) -> __noendbr u64;
}
extern "C" {
    pub fn ibt_restore(save: u64) -> __noendbr void;
}

pub const HAS_KERNEL_IBT: c_int = 0;
// Macro flag: #define ASM_ENDBR
// Macro flag: #define IBT_NOSEAL(name)
// Macro flag: #define __noendbr

// Macro flag: #define ENDBR

