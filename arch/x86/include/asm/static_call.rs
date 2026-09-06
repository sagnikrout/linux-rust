//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/static_call.h
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
// For CONFIG_HAVE_STATIC_CALL_INLINE, this is a temporary trampoline which
// uses the current value of the key->func pointer to do an indirect jump to
// the function.  This trampoline is only used during boot, before the call
// sites get patched by static_call_update().  The name of this trampoline has
// a magical aspect: objtool uses it to find static call sites so it can create
// the .static_call_sites section.
//
// For CONFIG_HAVE_STATIC_CALL, this is a permanent trampoline which
// does a direct jump to the function.  The direct jump gets patched by
// static_call_update().
//
// Having the trampoline in a special section forces GCC to emit a JMP.d32 when
// it does tail-call optimization on the call; since you cannot compute the
// relative displacement across sections.
//
// The trampoline is 8 bytes and of the general form:
//
// jmp.d32 \func
// ud1 %esp, %ecx
//
// That trailing #UD provides both a speculation stop and serves as a unique
// 3 byte signature identifying static call trampolines. Also see tramp_ud[]
// and __static_call_fixup().
//

extern "C" {
    pub fn __static_call_fixup(tramp: *mut c_void, op: u8, dest: *mut c_void) -> bool;
}
extern "C" {
    pub fn __static_call_update_early(tramp: *mut c_void, func: *mut c_void);
}

