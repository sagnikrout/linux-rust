//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/uapi/asm/ptrace.h
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
// Copyright (C) 2001 PPC64 Team, IBM Corp
//
// This struct defines the way the registers are stored on the
// kernel stack during a system call or other kernel entry.
//
// this should only contain volatile regs
// since we can keep non-volatile in the thread_struct
// should set this up when only volatiles are saved
// by intr code.
//
// Since this is going on the stack, *CARE MUST BE TAKEN* to insure
// that the overall structure is a multiple of 16 bytes in length.
//
// Note that the offsets of the fields in this struct correspond with
// the PT_* values below.  This simplifies arch/powerpc/kernel/ptrace.c.
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//

// Used on APUS to hold IPL value.

// N.B. for critical exceptions on 4xx, the dar and dsisr

//
// Offsets used by 'ptrace' system call interface.
// These can't be changed without breaking binary compatibility
// with MkLinux, etc.
//
pub const PT_R0: c_int = 0;
pub const PT_R1: c_int = 1;
pub const PT_R2: c_int = 2;
pub const PT_R3: c_int = 3;
pub const PT_R4: c_int = 4;
pub const PT_R5: c_int = 5;
pub const PT_R6: c_int = 6;
pub const PT_R7: c_int = 7;
pub const PT_R8: c_int = 8;
pub const PT_R9: c_int = 9;
pub const PT_R10: c_int = 10;
pub const PT_R11: c_int = 11;
pub const PT_R12: c_int = 12;
pub const PT_R13: c_int = 13;
pub const PT_R14: c_int = 14;
pub const PT_R15: c_int = 15;
pub const PT_R16: c_int = 16;
pub const PT_R17: c_int = 17;
pub const PT_R18: c_int = 18;
pub const PT_R19: c_int = 19;
pub const PT_R20: c_int = 20;
pub const PT_R21: c_int = 21;
pub const PT_R22: c_int = 22;
pub const PT_R23: c_int = 23;
pub const PT_R24: c_int = 24;
pub const PT_R25: c_int = 25;
pub const PT_R26: c_int = 26;
pub const PT_R27: c_int = 27;
pub const PT_R28: c_int = 28;
pub const PT_R29: c_int = 29;
pub const PT_R30: c_int = 30;
pub const PT_R31: c_int = 31;
pub const PT_NIP: c_int = 32;
pub const PT_MSR: c_int = 33;
pub const PT_ORIG_R3: c_int = 34;
pub const PT_CTR: c_int = 35;
pub const PT_LNK: c_int = 36;
pub const PT_XER: c_int = 37;
pub const PT_CCR: c_int = 38;

pub const PT_MQ: c_int = 39;

pub const PT_SOFTE: c_int = 39;

pub const PT_TRAP: c_int = 40;
pub const PT_DAR: c_int = 41;
pub const PT_DSISR: c_int = 42;
pub const PT_RESULT: c_int = 43;
pub const PT_DSCR: c_int = 44;
pub const PT_REGS_COUNT: c_int = 44;

//
// Only store first 32 VSRs here. The second 32 VSRs in VR0-31
//

//
// Get/set all the altivec registers v0..v31, vscr, vrsave, in one go.
// The transfer totals 34 quadword.  Quadwords 0-31 contain the
// corresponding vector registers.  Quadword 32 contains the vscr as the
// last word (offset 12) within that quadword.  Quadword 33 contains the
// vrsave as the first word (offset 0) within the quadword.
//
// This definition of the VMX state is compatible with the current PPC32
// ptrace interface.  This allows signal handling and ptrace to use the same
// structures.  This also simplifies the implementation of a bi-arch
// (combined (32- and 64-bit) gdb.
//
pub const PTRACE_GETVRREGS: c_uint = 0x12;
pub const PTRACE_SETVRREGS: c_uint = 0x13;
// Get/set all the upper 32-bits of the SPE registers, accumulator, and
// spefscr, in one go
pub const PTRACE_GETEVRREGS: c_uint = 0x14;
pub const PTRACE_SETEVRREGS: c_uint = 0x15;
// Get the first 32 128bit VSX registers
pub const PTRACE_GETVSRREGS: c_uint = 0x1b;
pub const PTRACE_SETVSRREGS: c_uint = 0x1c;
// Syscall emulation defines
pub const PTRACE_SYSEMU: c_uint = 0x1d;
pub const PTRACE_SYSEMU_SINGLESTEP: c_uint = 0x1e;
//
// Get or set a debug register. The first 16 are DABR registers and the
// second 16 are IABR registers.
//
pub const PTRACE_GET_DEBUGREG: c_uint = 0x19;
pub const PTRACE_SET_DEBUGREG: c_uint = 0x1a;
// (new) PTRACE requests using the same numbers as x86 and the same
// argument ordering. Additionally, they support more registers too
//
pub const PTRACE_GETREGS: c_uint = 0xc;
pub const PTRACE_SETREGS: c_uint = 0xd;
pub const PTRACE_GETFPREGS: c_uint = 0xe;
pub const PTRACE_SETFPREGS: c_uint = 0xf;
pub const PTRACE_GETREGS64: c_uint = 0x16;
pub const PTRACE_SETREGS64: c_uint = 0x17;
// Calls to trace a 64bit program from a 32bit program
pub const PPC_PTRACE_PEEKTEXT_3264: c_uint = 0x95;
pub const PPC_PTRACE_PEEKDATA_3264: c_uint = 0x94;
pub const PPC_PTRACE_POKETEXT_3264: c_uint = 0x93;
pub const PPC_PTRACE_POKEDATA_3264: c_uint = 0x92;
pub const PPC_PTRACE_PEEKUSR_3264: c_uint = 0x91;
pub const PPC_PTRACE_POKEUSR_3264: c_uint = 0x90;
pub const PTRACE_SINGLEBLOCK: c_uint = 0x100	/* resume execution until next branch */;
pub const PPC_PTRACE_GETHWDBGINFO: c_uint = 0x89;
pub const PPC_PTRACE_SETHWDEBUG: c_uint = 0x88;
pub const PPC_PTRACE_DELHWDEBUG: c_uint = 0x87;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ppc_debug_info {
    pub /: *mut *mut __u32 version; / Only version 1 exists to date,
    pub num_instruction_bps: __u32,
    pub num_data_bps: __u32,
    pub num_condition_regs: __u32,
    pub data_bp_alignment: __u32,
    pub /: *mut *mut __u32 sizeof_condition; / size of the DVC register,
    pub features: __u64,
}

//
// features will have bits indication whether there is support for:
//
pub const PPC_DEBUG_FEATURE_INSN_BP_RANGE: c_uint = 0x0000000000000001;
pub const PPC_DEBUG_FEATURE_INSN_BP_MASK: c_uint = 0x0000000000000002;
pub const PPC_DEBUG_FEATURE_DATA_BP_RANGE: c_uint = 0x0000000000000004;
pub const PPC_DEBUG_FEATURE_DATA_BP_MASK: c_uint = 0x0000000000000008;
pub const PPC_DEBUG_FEATURE_DATA_BP_DAWR: c_uint = 0x0000000000000010;
pub const PPC_DEBUG_FEATURE_DATA_BP_ARCH_31: c_uint = 0x0000000000000020;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ppc_hw_breakpoint {
    pub /: *mut *mut __u32 version; / currently, version must be 1,
    pub /: *mut *mut __u32 trigger_type; / only some combinations allowed,
    pub /: *mut *mut __u32 addr_mode; / address match mode,
    pub /: *mut *mut __u32 condition_mode; / break/watchpoint condition flags,
    pub /: *mut *mut __u64 addr; / break/watchpoint address,
    pub /: *mut *mut __u64 addr2; / range end or mask,
    pub /: *mut *mut __u64 condition_value; / contents of the DVC register,
}

//
// Trigger Type
//
pub const PPC_BREAKPOINT_TRIGGER_EXECUTE: c_uint = 0x00000001;
pub const PPC_BREAKPOINT_TRIGGER_READ: c_uint = 0x00000002;
pub const PPC_BREAKPOINT_TRIGGER_WRITE: c_uint = 0x00000004;

//
// Address Mode
//
pub const PPC_BREAKPOINT_MODE_EXACT: c_uint = 0x00000000;
pub const PPC_BREAKPOINT_MODE_RANGE_INCLUSIVE: c_uint = 0x00000001;
pub const PPC_BREAKPOINT_MODE_RANGE_EXCLUSIVE: c_uint = 0x00000002;
pub const PPC_BREAKPOINT_MODE_MASK: c_uint = 0x00000003;
//
// Condition Mode
//
pub const PPC_BREAKPOINT_CONDITION_MODE: c_uint = 0x00000003;
pub const PPC_BREAKPOINT_CONDITION_NONE: c_uint = 0x00000000;
pub const PPC_BREAKPOINT_CONDITION_AND: c_uint = 0x00000001;

pub const PPC_BREAKPOINT_CONDITION_OR: c_uint = 0x00000002;
pub const PPC_BREAKPOINT_CONDITION_AND_OR: c_uint = 0x00000003;
pub const PPC_BREAKPOINT_CONDITION_BE_ALL: c_uint = 0x00ff0000;
pub const PPC_BREAKPOINT_CONDITION_BE_SHIFT: c_int = 16;

