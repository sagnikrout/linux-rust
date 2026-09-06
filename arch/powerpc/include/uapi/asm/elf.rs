//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/uapi/asm/elf.h
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
// ELF register definitions..
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//

// PowerPC relocations defined by the ABIs
pub const R_PPC_NONE: c_int = 0;

pub const R_PPC_ADDR14_BRTAKEN: c_int = 8;
pub const R_PPC_ADDR14_BRNTAKEN: c_int = 9;

pub const R_PPC_REL14_BRTAKEN: c_int = 12;
pub const R_PPC_REL14_BRNTAKEN: c_int = 13;
pub const R_PPC_GOT16: c_int = 14;
pub const R_PPC_GOT16_LO: c_int = 15;
pub const R_PPC_GOT16_HI: c_int = 16;
pub const R_PPC_GOT16_HA: c_int = 17;
pub const R_PPC_PLTREL24: c_int = 18;
pub const R_PPC_COPY: c_int = 19;
pub const R_PPC_GLOB_DAT: c_int = 20;
pub const R_PPC_JMP_SLOT: c_int = 21;
pub const R_PPC_RELATIVE: c_int = 22;
pub const R_PPC_LOCAL24PC: c_int = 23;
pub const R_PPC_UADDR32: c_int = 24;
pub const R_PPC_UADDR16: c_int = 25;
pub const R_PPC_REL32: c_int = 26;
pub const R_PPC_PLT32: c_int = 27;
pub const R_PPC_PLTREL32: c_int = 28;
pub const R_PPC_PLT16_LO: c_int = 29;
pub const R_PPC_PLT16_HI: c_int = 30;
pub const R_PPC_PLT16_HA: c_int = 31;
pub const R_PPC_SDAREL16: c_int = 32;
pub const R_PPC_SECTOFF: c_int = 33;
pub const R_PPC_SECTOFF_LO: c_int = 34;
pub const R_PPC_SECTOFF_HI: c_int = 35;
pub const R_PPC_SECTOFF_HA: c_int = 36;
// PowerPC relocations defined for the TLS access ABI.

// keep this the last entry.
pub const R_PPC_NUM: c_int = 95;

pub type elf_greg_t64 = c_ulong;
pub type elf_greg_t32 = c_uint;
pub type compat_elf_gregset_t = elf_gregset_t32;
//
// ELF_ARCH, CLASS, and DATA are used to set parameters in the core dumps.
//

pub type elf_greg_t = elf_greg_t64;
pub type elf_gregset_t = elf_gregset_t64;

pub type elf_greg_t = elf_greg_t32;
pub type elf_gregset_t = elf_gregset_t32;

// Floating point registers
pub type elf_fpreg_t = double;
// Altivec registers
//
// The entries with indexes 0-31 contain the corresponding vector registers.
// The entry with index 32 contains the vscr as the last word (offset 12)
// within the quadword.  This allows the vscr to be stored as either a
// quadword (since it must be copied via a vector register to/from storage)
// or as a word.
//
// 64-bit kernel notes: The entry at index 33 contains the vrsave as the first
// word (offset 0) within the quadword.
//
// This definition of the VMX state is compatible with the current PPC32
// ptrace interface.  This allows signal handling and ptrace to use the same
// structures.  This also simplifies the implementation of a bi-arch
// (combined (32- and 64-bit) gdb.
//
// Note that it's _not_ compatible with 32 bits ucontext which stuffs the
// vrsave along with vscr and so only uses 33 vectors for the register set
//
pub type elf_vrreg_t = __vector128;

// PowerPC64 relocations defined by the ABIs

// PowerPC64 relocations defined for the TLS access ABI.

pub const R_PPC64_TLSGD: c_int = 107;
pub const R_PPC64_TLSLD: c_int = 108;
pub const R_PPC64_TOCSAVE: c_int = 109;
pub const R_PPC64_REL24_NOTOC: c_int = 116;
pub const R_PPC64_ENTRY: c_int = 118;
pub const R_PPC64_PCREL34: c_int = 132;
pub const R_PPC64_GOT_PCREL34: c_int = 133;
pub const R_PPC64_REL16: c_int = 249;
pub const R_PPC64_REL16_LO: c_int = 250;
pub const R_PPC64_REL16_HI: c_int = 251;
pub const R_PPC64_REL16_HA: c_int = 252;
// Keep this the last entry.
pub const R_PPC64_NUM: c_int = 253;
