//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/spectre.h
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
// Interface for managing mitigations for Spectre vulnerabilities.
//
// Copyright (C) 2020 Google LLC
// Author: Will Deacon <will@kernel.org>
//
pub const BP_HARDEN_EL2_SLOTS: c_int = 4;

// Watch out, ordering is important here.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mitigation_state {
    SPECTRE_UNAFFECTED,
    SPECTRE_MITIGATED,
    SPECTRE_VULNERABLE,
}

//
// Note: the order of this enum corresponds to __bp_harden_hyp_vecs and
// we rely on having the direct vectors first.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum arm64_hyp_spectre_vector {
//
// Take exceptions directly to __kvm_hyp_vector. This must be
// 0 so that it used by default when mitigations are not needed.
//
    HYP_VECTOR_DIRECT,

//
// Bounce via a slot in the hypervisor text mapping of
// __bp_harden_hyp_vecs, which contains an SMC call.
//
    HYP_VECTOR_SPECTRE_DIRECT,

//
// Bounce via a slot in a special mapping of __bp_harden_hyp_vecs
// next to the idmap page.
//
    HYP_VECTOR_INDIRECT,

//
// Bounce via a slot in a special mapping of __bp_harden_hyp_vecs
// next to the idmap page, which contains an SMC call.
//
    HYP_VECTOR_SPECTRE_INDIRECT,
}

extern "C" {
    pub fn void(_arg: *mut bp_hardening_cb_t)(void) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bp_hardening_data {
    pub slot: arm64_hyp_spectre_vector,
    pub fn: bp_hardening_cb_t,
}

// Called during entry so must be __always_inline
extern "C" {
    pub fn arm64_get_spectre_v2_state() -> mitigation_state;
}
extern "C" {
    pub fn has_spectre_v2(cap: *const arm64_cpu_capabilities, scope: c_int) -> bool;
}
extern "C" {
    pub fn spectre_v2_enable_mitigation(__unused: *const arm64_cpu_capabilities);
}
extern "C" {
    pub fn has_spectre_v3a(cap: *const arm64_cpu_capabilities, scope: c_int) -> bool;
}
extern "C" {
    pub fn spectre_v3a_enable_mitigation(__unused: *const arm64_cpu_capabilities);
}
extern "C" {
    pub fn arm64_get_spectre_v4_state() -> mitigation_state;
}
extern "C" {
    pub fn has_spectre_v4(cap: *const arm64_cpu_capabilities, scope: c_int) -> bool;
}
extern "C" {
    pub fn spectre_v4_enable_mitigation(__unused: *const arm64_cpu_capabilities);
}
extern "C" {
    pub fn spectre_v4_enable_task_mitigation(tsk: *mut task_struct);
}
extern "C" {
    pub fn arm64_get_meltdown_state() -> mitigation_state;
}
extern "C" {
    pub fn arm64_get_spectre_bhb_state() -> mitigation_state;
}
extern "C" {
    pub fn is_spectre_bhb_affected(entry: *const arm64_cpu_capabilities, scope: c_int) -> bool;
}
extern "C" {
    pub fn get_spectre_bhb_loop_value() -> u8;
}
extern "C" {
    pub fn is_spectre_bhb_fw_mitigated() -> bool;
}
extern "C" {
    pub fn spectre_bhb_enable_mitigation(__unused: *const arm64_cpu_capabilities);
}
extern "C" {
    pub fn try_emulate_el1_ssbs(regs: *mut pt_regs, instr: u32) -> bool;
}
extern "C" {
    pub fn spectre_print_disabled_mitigations();
}

