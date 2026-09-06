//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/kprobes.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Kernel Probes (KProbes)
//
// Copyright IBM Corp. 2002, 2006
//
// 2002-Oct	Created by Vamsi Krishna S <vamsi_krishna@in.ibm.com> Kernel
// Probes initial implementation ( includes suggestions from
// Rusty Russell).
// 2004-Nov	Modified for PPC64 by Ananth N Mavinakayanahalli
// <ananth@in.ibm.com>
// 2005-Dec	Used as a template for s390 by Mike Grundy
// <grundym@us.ibm.com>
//

pub const BREAKPOINT_INSTRUCTION: c_uint = 0x0002;
pub const FIXUP_PSW_NORMAL: c_uint = 0x08;
pub const FIXUP_BRANCH_NOT_TAKEN: c_uint = 0x04;
pub const FIXUP_RETURN_REGISTER: c_uint = 0x02;
pub const FIXUP_NOT_REQUIRED: c_uint = 0x01;
extern "C" {
    pub fn probe_is_prohibited_opcode(insn: *mut u16) -> c_int;
}
extern "C" {
    pub fn probe_get_fixup_type(insn: *mut u16) -> c_int;
}
extern "C" {
    pub fn probe_is_insn_relative_long(insn: *mut u16) -> c_int;
}

pub type kprobe_opcode_t = u16;
// Maximum instruction size is 3 (16bit) halfwords:
pub const MAX_INSN_SIZE: c_uint = 0x0003;
pub const MAX_STACK_SIZE: c_int = 64;

pub const kretprobe_blacklist_size: c_int = 0;
// Architecture specific copy of original instruction
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_specific_insn {
// copy of original instruction
    pub insn: *mut kprobe_opcode_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prev_kprobe {
    pub kp: *mut kprobe,
    pub status: c_ulong,
}

// per-cpu kprobe control block
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kprobe_ctlblk {
    pub kprobe_status: c_ulong,
    pub kprobe_saved_imask: c_ulong,
    pub kprobe_saved_ctl: [ctlreg; 3],
    pub prev_kprobe: prev_kprobe,
}

extern "C" {
    pub fn arch_remove_kprobe(p: *mut kprobe);
}
extern "C" {
    pub fn kprobe_fault_handler(regs: *mut pt_regs, trapnr: c_int) -> c_int;
}

