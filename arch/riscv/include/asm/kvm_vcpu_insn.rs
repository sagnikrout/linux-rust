//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/kvm_vcpu_insn.h
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
// Copyright (c) 2022 Ventana Micro Systems Inc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_mmio_decode {
    pub insn: c_ulong,
    pub insn_len: c_int,
    pub len: c_int,
    pub shift: c_int,
    pub return_handled: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_csr_decode {
    pub insn: c_ulong,
    pub return_handled: c_int,
}

// Return values used by function emulating a particular instruction
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kvm_insn_return {
    KVM_INSN_EXIT_TO_USER_SPACE = 0,
    KVM_INSN_CONTINUE_NEXT_SEPC,
    KVM_INSN_CONTINUE_SAME_SEPC,
    KVM_INSN_ILLEGAL_TRAP,
    KVM_INSN_VIRTUAL_TRAP
}

extern "C" {
    pub fn kvm_riscv_vcpu_wfi(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_riscv_vcpu_csr_return(vcpu: *mut kvm_vcpu, run: *mut kvm_run) -> c_int;
}
extern "C" {
    pub fn kvm_riscv_vcpu_mmio_return(vcpu: *mut kvm_vcpu, run: *mut kvm_run) -> c_int;
}
