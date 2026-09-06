//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/kvm/sys_regs.h
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
// Copyright (C) 2012,2013 - ARM Ltd
// Author: Marc Zyngier <marc.zyngier@arm.com>
//
// Derived from arch/arm/kvm/coproc.h
// Copyright (C) 2012 - Virtual Open Systems and Columbia University
// Authors: Christoffer Dall <c.dall@virtualopensystems.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sys_reg_params {
    pub Op0: u8,
    pub Op1: u8,
    pub CRn: u8,
    pub CRm: u8,
    pub Op2: u8,
    pub regval: u64,
    pub is_write: bool,
}

//
// The Feature ID space is defined as the System register space in AArch64
// with op0==3, op1=={0, 1, 3}, CRn==0, CRm=={0-7}, op2=={0-7}.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sys_reg_desc {
// Sysreg string for debug
    pub name: *const c_char,
    pub aarch32_map: },
// MRS/MSR instruction which accesses it.
    pub Op0: u8,
    pub Op1: u8,
    pub CRn: u8,
    pub CRm: u8,
    pub Op2: u8,
// Trapped access from guest, if non-NULL.
    pub ): *const sys_reg_desc,
//
// Initialization for vcpu. Return initialized value, or KVM
// sanitized value for ID registers.
//
    pub ): *const *const *const u64 (reset)(struct kvm_vcpu , struct sys_reg_desc,
// Index into sys_reg[], or 0 if we don't need to save it.
    pub reg: c_int,
// Value (usually reset value), or write mask for idregs
    pub val: u64,
// Custom get/set_user functions, fallback to generic if NULL
    pub val): *mut u64,
    pub val): u64,
// Return mask of REG_* runtime visibility overrides
    pub rd): *const sys_reg_desc,
}

// Look, we even formatted it for you to paste into the table!
// GCC warns on an empty format string
// Reset functions
extern "C" {
    pub fn __vcpu_sys_reg(_arg: vcpu, _arg: r->reg) -> return;
}
extern "C" {
    pub fn __vcpu_sys_reg(_arg: vcpu, _arg: r->reg) -> return;
}
extern "C" {
    pub fn __inline_bsearch()pval: *mut (void, _arg: table, _arg: num, _arg: sizeof(table[0]), _arg: match_sys_reg) -> return;
}
//
// Map the vcpu_id into the first three affinity level fields of
// the MPIDR. We limit the number of VCPUs in level 0 due to a
// limitation to 16 CPUs in that level in the ICC_SGIxR registers
// of the GICv3 to be able to address each CPU directly when
// sending IPIs.
//
extern "C" {
    pub fn kvm_arm_sys_reg_get_reg(vcpu: *mut kvm_vcpu, : *const kvm_one_reg) -> c_int;
}
extern "C" {
    pub fn kvm_arm_sys_reg_set_reg(vcpu: *mut kvm_vcpu, : *const kvm_one_reg) -> c_int;
}
extern "C" {
    pub fn triage_sysreg_trap(vcpu: *mut kvm_vcpu, sr_index: *mut c_int) -> bool;
}
extern "C" {
    pub fn kvm_finalize_sys_regs(vcpu: *mut kvm_vcpu) -> c_int;
}

