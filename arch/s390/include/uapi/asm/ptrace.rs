//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/uapi/asm/ptrace.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// S390 version
// Copyright IBM Corp. 1999, 2000
// Author(s): Denis Joseph Barrow (djbarrow@de.ibm.com,barrow_dj@yahoo.com)
//

//
// Offsets in the user_regs_struct. They are used for the ptrace
// system call and in entry.S
//
pub const PT_PSWMASK: c_uint = 0x00;
pub const PT_PSWADDR: c_uint = 0x08;
pub const PT_GPR0: c_uint = 0x10;
pub const PT_GPR1: c_uint = 0x18;
pub const PT_GPR2: c_uint = 0x20;
pub const PT_GPR3: c_uint = 0x28;
pub const PT_GPR4: c_uint = 0x30;
pub const PT_GPR5: c_uint = 0x38;
pub const PT_GPR6: c_uint = 0x40;
pub const PT_GPR7: c_uint = 0x48;
pub const PT_GPR8: c_uint = 0x50;
pub const PT_GPR9: c_uint = 0x58;
pub const PT_GPR10: c_uint = 0x60;
pub const PT_GPR11: c_uint = 0x68;
pub const PT_GPR12: c_uint = 0x70;
pub const PT_GPR13: c_uint = 0x78;
pub const PT_GPR14: c_uint = 0x80;
pub const PT_GPR15: c_uint = 0x88;
pub const PT_ACR0: c_uint = 0x90;
pub const PT_ACR1: c_uint = 0x94;
pub const PT_ACR2: c_uint = 0x98;
pub const PT_ACR3: c_uint = 0x9C;
pub const PT_ACR4: c_uint = 0xA0;
pub const PT_ACR5: c_uint = 0xA4;
pub const PT_ACR6: c_uint = 0xA8;
pub const PT_ACR7: c_uint = 0xAC;
pub const PT_ACR8: c_uint = 0xB0;
pub const PT_ACR9: c_uint = 0xB4;
pub const PT_ACR10: c_uint = 0xB8;
pub const PT_ACR11: c_uint = 0xBC;
pub const PT_ACR12: c_uint = 0xC0;
pub const PT_ACR13: c_uint = 0xC4;
pub const PT_ACR14: c_uint = 0xC8;
pub const PT_ACR15: c_uint = 0xCC;
pub const PT_ORIGGPR2: c_uint = 0xD0;
pub const PT_FPC: c_uint = 0xD8;
pub const PT_FPR0: c_uint = 0xE0;
pub const PT_FPR1: c_uint = 0xE8;
pub const PT_FPR2: c_uint = 0xF0;
pub const PT_FPR3: c_uint = 0xF8;
pub const PT_FPR4: c_uint = 0x100;
pub const PT_FPR5: c_uint = 0x108;
pub const PT_FPR6: c_uint = 0x110;
pub const PT_FPR7: c_uint = 0x118;
pub const PT_FPR8: c_uint = 0x120;
pub const PT_FPR9: c_uint = 0x128;
pub const PT_FPR10: c_uint = 0x130;
pub const PT_FPR11: c_uint = 0x138;
pub const PT_FPR12: c_uint = 0x140;
pub const PT_FPR13: c_uint = 0x148;
pub const PT_FPR14: c_uint = 0x150;
pub const PT_FPR15: c_uint = 0x158;
pub const PT_CR_9: c_uint = 0x160;
pub const PT_CR_10: c_uint = 0x168;
pub const PT_CR_11: c_uint = 0x170;
pub const PT_IEEE_IP: c_uint = 0x1A8;

pub const PT_ENDREGS: c_uint = 0x1B0-1;
pub const GPR_SIZE: c_int = 8;
pub const CR_SIZE: c_int = 8;

pub const NUM_GPRS: c_int = 16;
pub const NUM_FPRS: c_int = 16;
pub const NUM_CRS: c_int = 16;
pub const NUM_ACRS: c_int = 16;
pub const NUM_CR_WORDS: c_int = 3;
pub const FPR_SIZE: c_int = 8;
pub const FPC_SIZE: c_int = 4;

pub const ACR_SIZE: c_int = 4;
pub const PTRACE_OLDSETOPTIONS: c_int = 21;
pub const PTRACE_SYSEMU: c_int = 31;
pub const PTRACE_SYSEMU_SINGLESTEP: c_int = 32;

pub const FPC_EXCEPTION_MASK: c_uint = 0xF8000000;
pub const FPC_FLAGS_MASK: c_uint = 0x00F80000;
pub const FPC_DXC_MASK: c_uint = 0x0000FF00;
pub const FPC_RM_MASK: c_uint = 0x00000003;
// this typedef defines how a Program Status Word looks like
//
// The s390_regs structure is used to define the elf_gregset_t.
//
// The user_pt_regs structure exports the beginning of
// the in-kernel pt_regs structure to user space.
//
// Now for the user space program event recording (trace) definitions.
// The following structures are used only for the ptrace interface, don't
// touch or even look at it if you don't want to modify the user-space
// ptrace interface. In particular stay away from it for in-kernel PER.
//
pub const PER_EM_MASK: c_uint = 0xE8000000UL;
//
// Switching on storage alteration automatically fixes
// the storage alteration event bit in the users std.
//
// The single_step and instruction_fetch bits are obsolete,
// the kernel always sets them to zero. To enable single
// stepping use ptrace(PTRACE_SINGLESTEP) instead.
//
// These addresses are copied into cr10 & cr11 if single
// stepping is switched off
//
// S/390 specific non posix ptrace requests. I chose unusual values so
// they are unlikely to clash with future ptrace definitions.
//
pub const PTRACE_PEEKUSR_AREA: c_uint = 0x5000;
pub const PTRACE_POKEUSR_AREA: c_uint = 0x5001;
pub const PTRACE_PEEKTEXT_AREA: c_uint = 0x5002;
pub const PTRACE_PEEKDATA_AREA: c_uint = 0x5003;
pub const PTRACE_POKETEXT_AREA: c_uint = 0x5004;
pub const PTRACE_POKEDATA_AREA: c_uint = 0x5005;
pub const PTRACE_GET_LAST_BREAK: c_uint = 0x5006;
pub const PTRACE_PEEK_SYSTEM_CALL: c_uint = 0x5007;
pub const PTRACE_POKE_SYSTEM_CALL: c_uint = 0x5008;
pub const PTRACE_ENABLE_TE: c_uint = 0x5009;
pub const PTRACE_DISABLE_TE: c_uint = 0x5010;
pub const PTRACE_TE_ABORT_RAND: c_uint = 0x5011;
//
// The numbers chosen here are somewhat arbitrary but absolutely MUST
// not overlap with any of the number assigned in <linux/ptrace.h>.
//

//
// PT_PROT definition is loosely based on hppa bsd definition in
// gdb/hppab-nat.c
//
pub const PTRACE_PROT: c_int = 21;
// Sequence of bytes for breakpoint illegal instruction.

pub const S390_SYSCALL_SIZE: c_int = 2;
//
// The user_regs_struct defines the way the user registers are
// store on the stack for signal handling.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_regs_struct {
    pub psw: psw_t,
    pub gprs: [c_ulong; NUM_GPRS],
    pub acrs: [c_uint; NUM_ACRS],
    pub orig_gpr2: c_ulong,
    pub fp_regs: s390_fp_regs,
//
// These per registers are in here so that gdb can modify them
// itself as there is no "official" ptrace interface for hardware
// watchpoints. This is the way intel does it.
//
    pub per_info: per_struct,
    pub /: *mut *mut unsigned long ieee_instruction_pointer; / obsolete, always 0,
}

