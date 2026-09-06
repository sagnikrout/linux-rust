//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/kprobes.h
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
// Kernel Probes (KProbes)
//
// Copyright (C) IBM Corporation, 2002, 2004
//
// 2002-Oct	Created by Vamsi Krishna S <vamsi_krishna@in.ibm.com> Kernel
// Probes initial implementation ( includes suggestions from
// Rusty Russell).
// 2004-Nov	Modified for PPC64 by Ananth N Mavinakayanahalli
// <ananth@in.ibm.com>
//

pub type kprobe_opcode_t = u32;
// Optinsn template address
// Fixed instruction size for powerpc
pub const MAX_INSN_SIZE: c_int = 2;

pub const kretprobe_blacklist_size: c_int = 0;
extern "C" {
    pub fn __kretprobe_trampoline();
}
extern "C" {
    pub fn arch_remove_kprobe(p: *mut kprobe);
}
// Architecture specific copy of original instruction
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_specific_insn {
// copy of original instruction
    pub insn: *mut kprobe_opcode_t,
//
// Set in kprobes code, initially to 0. If the instruction can be
// eumulated, this is set to 1, if not, to -1.
//
    pub boostable: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prev_kprobe {
    pub kp: *mut kprobe,
    pub status: c_ulong,
    pub saved_msr: c_ulong,
}

// per-cpu kprobe control block
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kprobe_ctlblk {
    pub kprobe_status: c_ulong,
    pub kprobe_saved_msr: c_ulong,
    pub prev_kprobe: prev_kprobe,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_optimized_insn {
    pub copied_insn: [kprobe_opcode_t; 1],
// detour buffer
    pub insn: *mut kprobe_opcode_t,
}

extern "C" {
    pub fn kprobe_fault_handler(regs: *mut pt_regs, trapnr: c_int) -> c_int;
}
extern "C" {
    pub fn kprobe_handler(regs: *mut pt_regs) -> c_int;
}
extern "C" {
    pub fn kprobe_post_handler(regs: *mut pt_regs) -> c_int;
}

