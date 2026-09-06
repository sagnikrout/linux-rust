//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/uapi/asm/sigcontext.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigcontext {
    pub _unused: [c_ulong; 4],
    pub signal: c_int,

    pub _pad0: c_int,

    pub handler: c_ulong,
    pub oldmask: c_ulong,

    pub regs: *mut user_pt_regs __user,

    pub regs: *mut pt_regs,

    pub gp_regs: elf_gregset_t,
    pub fp_regs: elf_fpregset_t,
//
// To maintain compatibility with current implementations the sigcontext is
// extended by appending a pointer (v_regs) to a quadword type (elf_vrreg_t)
// followed by an unstructured (vmx_reserve) field of 101 doublewords. This
// allows the array of vector registers to be quadword aligned independent of
// the alignment of the containing sigcontext or ucontext. It is the
// responsibility of the code setting the sigcontext to set this pointer to
// either NULL (if this processor does not support the VMX feature) or the
// address of the first quadword within the allocated (vmx_reserve) area.
//
// The pointer (v_regs) of vector type (elf_vrreg_t) is type compatible with
// an array of 34 quadword entries (elf_vrregset_t).  The entries with
// indexes 0-31 contain the corresponding vector registers.  The entry with
// index 32 contains the vscr as the last word (offset 12) within the
// quadword.  This allows the vscr to be stored as either a quadword (since
// it must be copied via a vector register to/from storage) or as a word.
// The entry with index 33 contains the vrsave as the first word (offset 0)
// within the quadword.
//
// Part of the VSX data is stored here also by extending vmx_restore
// by an additional 32 double words.  Architecturally the layout of
// the VSR registers and how they overlap on top of the legacy FPR and
// VR registers is shown below:
//
// VSR doubleword 0               VSR doubleword 1
// ----------------------------------------------------------------
// VSR[0]  |             FPR[0]            |                              |
// ----------------------------------------------------------------
// VSR[1]  |             FPR[1]            |                              |
// ----------------------------------------------------------------
// |              ...              |                              |
// ----------------------------------------------------------------
// VSR[30] |             FPR[30]           |                              |
// ----------------------------------------------------------------
// VSR[31] |             FPR[31]           |                              |
// ----------------------------------------------------------------
// VSR[32] |                             VR[0]                            |
// ----------------------------------------------------------------
// VSR[33] |                             VR[1]                            |
// ----------------------------------------------------------------
// |                              ...                             |
// ----------------------------------------------------------------
// VSR[62] |                             VR[30]                           |
// ----------------------------------------------------------------
// VSR[63] |                             VR[31]                           |
// ----------------------------------------------------------------
//
// FPR/VSR 0-31 doubleword 0 is stored in fp_regs, and VMX/VSR 32-63
// is stored at the start of vmx_reserve.  vmx_reserve is extended for
// backwards compatility to store VSR 0-31 doubleword 1 after the VMX
// registers and vscr/vrsave.
//
    pub v_regs: *mut elf_vrreg_t __user,
    pub 32]: long vmx_reserve[ELF_NVRREG + ELF_NVRREG + 1 +,

}
