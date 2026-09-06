//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/feature-fixups.h
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
// Feature section common macros
//
// Note that the entries now contain offsets between the table entry
// and the code rather than absolute code pointers in order to be
// useable with the vdso shared library. There is also an assumption
// that values will be negative, that is, the fixup table has to be
// located after the code it fixes up.
//

// 64 bits kernel, 32 bits code (ie. vdso32)

//
// If we use the ifgt syntax above, clang's assembler complains about the
// expression being non-absolute when the code appears in an inline assembly
// statement.
// As a workaround use an .org directive that has no effect if the else case
// instructions are smaller than the body, but fails otherwise.
//

// CPU feature dependent sections

// CPU feature sections with alternatives, use BEGIN_FTR_SECTION to start

// MMU feature dependent sections

// MMU feature sections with alternatives, use BEGIN_FTR_SECTION to start

// Firmware feature dependent sections

// Firmware feature sections with alternatives

// LWSYNC feature sections

extern "C" {
    pub fn apply_feature_fixups();
}
extern "C" {
    pub fn update_mmu_feature_fixups(mask: c_ulong);
}
extern "C" {
    pub fn setup_feature_keys();
}

