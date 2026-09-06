//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/vector.h
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
// Copyright (C) 2020 SiFive
//

extern "C" {
    pub fn riscv_v_setup_vsize() -> c_int;
}
extern "C" {
    pub fn insn_is_vector(insn_buf: u32) -> bool;
}
extern "C" {
    pub fn riscv_v_first_use_handler(regs: *mut pt_regs) -> bool;
}
extern "C" {
    pub fn kernel_vector_begin();
}
extern "C" {
    pub fn kernel_vector_end();
}
extern "C" {
    pub fn get_cpu_vector_context();
}
extern "C" {
    pub fn put_cpu_vector_context();
}
extern "C" {
    pub fn riscv_v_thread_free(tsk: *mut task_struct);
}
extern "C" {
    pub fn riscv_v_setup_ctx_cache() -> void __init;
}
extern "C" {
    pub fn riscv_v_thread_alloc(tsk: *mut task_struct);
}
extern "C" {
    pub fn update_regset_vector_info(size: c_ulong) -> void __init;
}
extern "C" {
    pub fn READ_ONCE(_arg: current->thread.riscv_v_flags) -> return;
}
extern "C" {
    pub fn riscv_has_extension_unlikely(_arg: RISCV_ISA_EXT_ZVE32X) -> return;
}
extern "C" {
    pub fn riscv_isa_vendor_extension_available(_arg: THEAD_VENDOR_ID, _arg: XTHEADVECTOR) -> return;
}
//
// CSR_VCSR is defined as
// [2:1] - vxrm[1:0]
// [0] - vxsat
// The earlier vector spec implemented by T-Head uses separate
// registers for the same bit-elements, so just combine those
// into the existing output field.
//
// Additionally T-Head cores need FS to be enabled when accessing
// the VXRM and VXSAT CSRs, otherwise ending in illegal instructions.
// Though the cores do not implement the VXRM and VXSAT fields in the
// FCSR CSR that vector-0.7.1 specifies.
//
// Similar to __vstate_csr_save above, restore values for the
// separate VXRM and VXSAT CSRs from the vcsr variable.
//
extern "C" {
    pub fn volatile("t4": THEAD_VSETVLI_T4X0E8M8D1 : : :) -> asm;
}

extern "C" {
    pub fn riscv_v_vstate_ctrl_init(tsk: *mut task_struct);
}
extern "C" {
    pub fn riscv_v_vstate_ctrl_user_allowed() -> bool;
}

//
// Return the implementation's vlen value.
//
// riscv_v_vsize contains the value of "32 vector registers with vlenb length"
// so rebuild the vlen value in bits from it.
//
