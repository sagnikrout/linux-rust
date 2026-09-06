//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/bpf/diagnostics.h
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
// Copyright (c) 2026 Meta Platforms, Inc. and affiliates.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_diag_mod_reason {
    BPF_DIAG_MOD_WRITE,
    BPF_DIAG_MOD_SPILL,
    BPF_DIAG_MOD_VAR_WRITE,
    BPF_DIAG_MOD_REF_RELEASE,
    BPF_DIAG_MOD_PKT_DATA_CHANGE,
    BPF_DIAG_MOD_NON_OWN_REF,
    BPF_DIAG_MOD_CALLER_SAVED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_diag_context_kind {
    BPF_DIAG_CONTEXT_NONE,
    BPF_DIAG_CONTEXT_RCU,
    BPF_DIAG_CONTEXT_PREEMPT,
    BPF_DIAG_CONTEXT_IRQ,
    BPF_DIAG_CONTEXT_LOCK,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_diag_invalid_deref_kind {
    BPF_DIAG_DEREF_SCALAR,
    BPF_DIAG_DEREF_NULLABLE_PTR,
    BPF_DIAG_DEREF_MODIFIED_PTR,
    BPF_DIAG_DEREF_INVALID_PTR,
}

extern "C" {
    pub fn bpf_diag_enabled(env: *const bpf_verifier_env) -> bool;
}
extern "C" {
    pub fn bpf_diag_init(env: *mut bpf_verifier_env) -> c_int;
}
extern "C" {
    pub fn bpf_diag_init_frame(env: *mut bpf_verifier_env, state: *mut bpf_func_state);
}
extern "C" {
    pub fn bpf_diag_event_log_save(env: *mut bpf_verifier_env) -> u64;
}
extern "C" {
    pub fn bpf_diag_event_log_restore(env: *mut bpf_verifier_env, log_pos: u64);
}
extern "C" {
    pub fn bpf_diag_irq_depth(state: *const bpf_verifier_state) -> u32;
}
extern "C" {
    pub fn bpf_diag_free(env: *mut bpf_verifier_env);
}
extern "C" {
    pub fn bpf_diag_unreadable_reg(env: *mut bpf_verifier_env, insn_idx: u32, regno: c_int);
}
extern "C" {
    pub fn bpf_diag_leak(env: *mut bpf_verifier_env, ref_id: u32, alloc_insn: u32, fail_insn: u32);
}
extern "C" {
    pub fn bpf_diag_record_branch(env: *mut bpf_verifier_env, insn_idx: u32, cond_true: bool);
}
extern "C" {
    pub fn bpf_diag_mod_end(env: *mut bpf_verifier_env);
}
extern "C" {
    pub fn bpf_diag_record_ref_acquire(env: *mut bpf_verifier_env, insn_idx: u32, ref_id: u32);
}
extern "C" {
    pub fn bpf_diag_record_ref_release(env: *mut bpf_verifier_env, insn_idx: u32, ref_id: u32);
}
