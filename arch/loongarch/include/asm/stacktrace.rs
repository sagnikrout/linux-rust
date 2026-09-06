//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/stacktrace.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stack_type {
    STACK_TYPE_UNKNOWN,
    STACK_TYPE_IRQ,
    STACK_TYPE_TASK,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stack_info {
    pub type: stack_type,
    pub next_sp: unsigned long begin, end,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stack_frame {
    pub fp: c_ulong,
    pub ra: c_ulong,
}

extern "C" {
    pub fn in_irq_stack(stack: c_ulong, info: *mut stack_info) -> bool;
}
extern "C" {
    pub fn in_task_stack(stack: c_ulong, task: *mut task_struct, info: *mut stack_info) -> bool;
}
extern "C" {
    pub fn get_stack_info(stack: c_ulong, task: *mut task_struct, info: *mut stack_info) -> c_int;
}

// Save $ra
// Use $ra to save PC
// Restore $ra
extern "C" {
    pub fn __volatile__((regs->csr_badvaddr): CSRRD_ONE_REG(LOONGARCH_CSR_BADV) : "=r") -> __asm__;
}
extern "C" {
    pub fn __volatile__((regs->csr_crmd): CSRRD_ONE_REG(LOONGARCH_CSR_CRMD) : "=r") -> __asm__;
}
extern "C" {
    pub fn __volatile__((regs->csr_prmd): CSRRD_ONE_REG(LOONGARCH_CSR_PRMD) : "=r") -> __asm__;
}
extern "C" {
    pub fn __volatile__((regs->csr_euen): CSRRD_ONE_REG(LOONGARCH_CSR_EUEN) : "=r") -> __asm__;
}
extern "C" {
    pub fn __volatile__((regs->csr_ecfg): CSRRD_ONE_REG(LOONGARCH_CSR_ECFG) : "=r") -> __asm__;
}
extern "C" {
    pub fn __volatile__((regs->csr_estat): CSRRD_ONE_REG(LOONGARCH_CSR_ESTAT) : "=r") -> __asm__;
}
