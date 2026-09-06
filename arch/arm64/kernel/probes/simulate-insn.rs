//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/kernel/probes/simulate-insn.h
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
// arch/arm64/kernel/probes/simulate-insn.h
//
// Copyright (C) 2013 Linaro Limited
//
extern "C" {
    pub fn simulate_adr_adrp(opcode: u32, addr: c_long, regs: *mut pt_regs);
}
extern "C" {
    pub fn simulate_b_bl(opcode: u32, addr: c_long, regs: *mut pt_regs);
}
extern "C" {
    pub fn simulate_b_cond(opcode: u32, addr: c_long, regs: *mut pt_regs);
}
extern "C" {
    pub fn simulate_br_blr(opcode: u32, addr: c_long, regs: *mut pt_regs);
}
extern "C" {
    pub fn simulate_ret(opcode: u32, addr: c_long, regs: *mut pt_regs);
}
extern "C" {
    pub fn simulate_cbz_cbnz(opcode: u32, addr: c_long, regs: *mut pt_regs);
}
extern "C" {
    pub fn simulate_tbz_tbnz(opcode: u32, addr: c_long, regs: *mut pt_regs);
}
extern "C" {
    pub fn simulate_ldr_literal(opcode: u32, addr: c_long, regs: *mut pt_regs);
}
extern "C" {
    pub fn simulate_ldrsw_literal(opcode: u32, addr: c_long, regs: *mut pt_regs);
}
extern "C" {
    pub fn simulate_nop(opcode: u32, addr: c_long, regs: *mut pt_regs);
}
