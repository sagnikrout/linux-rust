//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/sstep.h
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
// Copyright (C) 2004 Paul Mackerras <paulus@au.ibm.com>, IBM
//

//
// We don't allow single-stepping an mtmsrd that would clear
// MSR_RI, since that would make the exception unrecoverable.
// Since we need to single-step to proceed from a breakpoint,
// we don't allow putting a breakpoint on an mtmsrd instruction.
// Similarly we don't allow breakpoints on rfid instructions.
// These macros tell us if an instruction is a mtmsrd or rfid.
// Note that these return true for both mtmsr/rfi (32-bit)
// and mtmsrd/rfid (64-bit).
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum instruction_type {
    COMPUTE,		/* arith/logical/CR op, etc. */
    LOAD,			/* load and store types need to be contiguous */
    LOAD_MULTI,
    LOAD_FP,
    LOAD_VMX,
    LOAD_VSX,
    STORE,
    STORE_MULTI,
    STORE_FP,
    STORE_VMX,
    STORE_VSX,
    LARX,
    STCX,
    BRANCH,
    MFSPR,
    MTSPR,
    CACHEOP,
    BARRIER,
    SYSCALL,
    SYSCALL_VECTORED_0,
    MFMSR,
    MTMSR,
    RFI,
    INTERRUPT,
    UNKNOWN
}

pub const INSTR_TYPE_MASK: c_uint = 0x1f;

// Compute flags, ORed in with type
pub const SETREG: c_uint = 0x20;
pub const SETCC: c_uint = 0x40;
pub const SETXER: c_uint = 0x80;
// Branch flags, ORed in with type
pub const SETLK: c_uint = 0x20;
pub const BRTAKEN: c_uint = 0x40;
pub const DECCTR: c_uint = 0x80;
// Load/store flags, ORed in with type
pub const SIGNEXT: c_uint = 0x20;
pub const UPDATE: c_uint = 0x40	/* matches bit in opcode 31 instructions */;
pub const BYTEREV: c_uint = 0x80;
pub const FPCONV: c_uint = 0x100;
// Barrier type field, ORed in with type
pub const BARRIER_MASK: c_uint = 0xe0;
pub const BARRIER_SYNC: c_uint = 0x00;
pub const BARRIER_ISYNC: c_uint = 0x20;
pub const BARRIER_EIEIO: c_uint = 0x40;
pub const BARRIER_LWSYNC: c_uint = 0x60;
pub const BARRIER_PTESYNC: c_uint = 0x80;
// Cacheop values, ORed in with type
pub const CACHEOP_MASK: c_uint = 0x700;
pub const DCBST: c_int = 0;
pub const DCBF: c_uint = 0x100;
pub const DCBTST: c_uint = 0x200;
pub const DCBT: c_uint = 0x300;
pub const ICBI: c_uint = 0x400;
pub const DCBZ: c_uint = 0x500;
// VSX flags values

// Prefixed flag, ORed in with type
pub const PREFIXED: c_uint = 0x800;
// Size field in type word

// Prefix instruction operands

#[repr(C)]
#[derive(Copy, Clone)]
pub struct instruction_op {
    pub type: c_int,
    pub reg: c_int,
    pub val: c_ulong,
// For LOAD/STORE/LARX/STCX
    pub ea: c_ulong,
    pub update_reg: c_int,
// For MFSPR
    pub spr: c_int,
    pub ccval: u32,
    pub xerval: u32,
    pub /: *mut *mut u8 element_size; / for VSX/VMX loads/stores,
    pub vsx_flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union vsx_reg {
    pub b: [u8; 16],
    pub h: [u16; 8],
    pub w: [u32; 4],
    pub d: [c_ulong; 2],
    pub fp: [float; 4],
    pub dp: [double; 2],
    pub v: __vector128,
}

//
// Decode an instruction, and return information about it in *op
// without changing *regs.
//
// Return value is 1 if the instruction can be emulated just by
// updating *regs with the information in *op, -1 if we need the
// GPRs but *regs doesn't contain the full register set, or 0
// otherwise.
//
// Emulate an instruction that can be executed just by updating
// fields in *regs.
//
extern "C" {
    pub fn emulate_update_regs(reg: *mut pt_regs, op: *mut instruction_op);
}
//
// Emulate instructions that cause a transfer of control,
// arithmetic/logical instructions, loads and stores,
// cache operations and barriers.
//
// Returns 1 if the instruction was emulated successfully,
// 0 if it could not be emulated, or -1 for an instruction that
// should not be emulated (rfid, mtmsrd clearing MSR_RI, etc.).
//
extern "C" {
    pub fn emulate_step(regs: *mut pt_regs, instr: ppc_inst_t) -> c_int;
}
//
// Emulate a load or store instruction by reading/writing the
// memory of the current process.  FP/VMX/VSX registers are assumed
// to hold live values if the appropriate enable bit in regs->msr is
// set; otherwise this will use the saved values in the thread struct
// for user-mode accesses.
//
extern "C" {
    pub fn emulate_loadstore(regs: *mut pt_regs, op: *mut instruction_op) -> c_int;
}
extern "C" {
    pub fn emulate_dcbz(ea: c_ulong, regs: *mut pt_regs) -> c_int;
}
