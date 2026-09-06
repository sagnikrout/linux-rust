//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/head-64.h
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
// We can't do CPP stringification and concatination directly into the section
// name for some reason, so these macros can do it for us.
//
// Fixed (location) sections are used by opening fixed sections and emitting
// fixed section entries into them before closing them. Multiple fixed sections
// can be open at any time.
//
// Each fixed section created in a .S file must have corresponding linkage
// directives including location, added to  arch/powerpc/kernel/vmlinux.lds.S
//
// For each fixed section, code is generated into it in the order which it
// appears in the source.  Fixed section entries can be placed at a fixed
// location within the section using _LOCATION postifx variants. These must
// be ordered according to their relative placements within the section.
//
// OPEN_FIXED_SECTION(section_name, start_address, end_address)
// FIXED_SECTION_ENTRY_BEGIN(section_name, label1)
//
// USE_FIXED_SECTION(section_name)
// label3:
// li  r10,128
// mv  r11,r10
// FIXED_SECTION_ENTRY_BEGIN_LOCATION(section_name, label2, start_address, size)
// FIXED_SECTION_ENTRY_END_LOCATION(section_name, label2, start_address, size)
// CLOSE_FIXED_SECTION(section_name)
//
// ZERO_FIXED_SECTION can be used to emit zeroed data.
//
// Troubleshooting:
// - If the build dies with "Error: attempt to move .org backwards" at
// CLOSE_FIXED_SECTION() or elsewhere, there may be something
// unexpected being added there. Remove the '. = x_len' line, rebuild, and
// check what is pushing the section down.
// - If the build dies in linking, check arch/powerpc/tools/head_check.sh
// comments.
// - If the kernel crashes or hangs in very early boot, it could be linker
// stubs at the start of the main text.
//

//
// .linker_stub_catch section is used to catch linker stubs from being
// inserted in our .text section, above the start_text label (which breaks
// the ABS_ADDR calculation). See kernel/vmlinux.lds.S and tools/head_check.sh
// for more details. We would prefer to just keep a cacheline (0x80), but
// 0x100 seems to be how the linker aligns branch stub groups.
//

//
// These macros are used to change symbols in other fixed sections to be
// absolute or related to our current fixed section.
//
// - DEFINE_FIXED_SYMBOL / FIXED_SYMBOL_ABS_ADDR is used to find the
// absolute address of a symbol within a fixed section, from any section.
//
// - ABS_ADDR is used to find the absolute address of any symbol, from within
// a fixed section.
//
// define label as being _in_ sname

// find label from _within_ sname

