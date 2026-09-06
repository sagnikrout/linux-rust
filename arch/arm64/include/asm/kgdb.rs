//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/kgdb.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// AArch64 KGDB support
//
// Based on arch/arm/include/kgdb.h
//
// Copyright (C) 2013 Cavium Inc.
// Author: Vijaya Kumar K <vijaya.kumar@caviumnetworks.com>
//

extern "C" {
    pub fn kgdb_handle_bus_error();
}
extern "C" {
    pub fn kgdb_brk_handler(regs: *mut pt_regs, esr: c_ulong) -> c_int;
}
extern "C" {
    pub fn kgdb_compiled_brk_handler(regs: *mut pt_regs, esr: c_ulong) -> c_int;
}

extern "C" {
    pub fn kgdb_single_step_handler(regs: *mut pt_regs, esr: c_ulong) -> c_int;
}

//
// gdb remote procotol (well most versions of it) expects the following
// register layout.
//
// General purpose regs:
// r0-r30: 64 bit
// sp,pc : 64 bit
// pstate  : 32 bit
// Total: 33 + 1
// FPU regs:
// f0-f31: 128 bit
// fpsr & fpcr: 32 bit
// Total: 32 + 2
//
// To expand a little on the "most versions of it"... when the gdb remote
// protocol for AArch64 was developed it depended on a statement in the
// Architecture Reference Manual that claimed "SPSR_ELx is a 32-bit register".
// and, as a result, allocated only 32-bits for the PSTATE in the remote
// protocol. In fact this statement is still present in ARM DDI 0487A.i.
//
// Unfortunately "is a 32-bit register" has a very special meaning for
// system registers. It means that "the upper bits, bits[63:32], are
// RES0.". RES0 is heavily used in the ARM architecture documents as a
// way to leave space for future architecture changes. So to translate a
// little for people who don't spend their spare time reading ARM architecture
// manuals, what "is a 32-bit register" actually means in this context is
// "is a 64-bit register but one with no meaning allocated to any of the
// upper 32-bits... *yet*".
//
// Perhaps then we should not be surprised that this has led to some
// confusion. Specifically a patch, influenced by the above translation,
// that extended PSTATE to 64-bit was accepted into gdb-7.7 but the patch
// was reverted in gdb-7.8.1 and all later releases, when this was
// discovered to be an undocumented protocol change.
//
// So... it is *not* wrong for us to only allocate 32-bits to PSTATE
// here even though the kernel itself allocates 64-bits for the same
// state. That is because this bit of code tells the kernel how the gdb
// remote protocol (well most versions of it) describes the register state.
//
// Note that if you are using one of the versions of gdb that supports
// the gdb-7.7 version of the protocol you cannot use kgdb directly
// without providing a custom register description (gdb can load new
// protocol descriptions at runtime).
//
pub const _GP_REGS: c_int = 33;
pub const _FP_REGS: c_int = 32;
pub const _EXTRA_REGS: c_int = 3;
//
// general purpose registers size in bytes.
// pstate is only 4 bytes. subtract 4 bytes
//

//
// Size of I/O buffer for gdb packet.
// considering to hold all register contents, size is set
//
pub const BUFMAX: c_int = 2048;
//
// Number of bytes required for gdb_regs buffer.
// _GP_REGS: 8 bytes, _FP_REGS: 16 bytes and _EXTRA_REGS: 4 bytes each
// GDB fails to connect for size beyond this with error
// "'g' packet reply is too long"
//

