//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/net/bpf_jit.h
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
// Common functionality for RV32 and RV64 BPF JIT compilers
//
// Copyright (c) 2019 Björn Töpel <bjorn.topel@gmail.com>
//

// verify runtime detection extension status

extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_RISCV_ISA_C) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rv_jit_context {
    pub prog: *mut bpf_prog,
    pub /: *mut *mut *mut u16 insns; / RV insns,
    pub ro_insns: *mut u16,
    pub ninsns: c_int,
    pub prologue_len: c_int,
    pub epilogue_offset: c_int,
    pub /: *mut *mut *mut int offset; / BPF to RV,
    pub nexentries: c_int,
    pub ex_insn_off: c_int,
    pub ex_jmp_off: c_int,
    pub flags: c_ulong,
    pub stack_size: c_int,
    pub tcc_offset: c_int,
    pub arena_vm_start: u64,
    pub user_vm_start: u64,
}

// Convert from ninsns to bytes.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rv_jit_data {
    pub header: *mut bpf_binary_header,
    pub ro_header: *mut bpf_binary_header,
    pub image: *mut u8,
    pub ro_image: *mut u8,
    pub ctx: rv_jit_context,
}

// Emit a 4-byte riscv instruction.
// Emit a 2-byte riscv compressed instruction.
extern "C" {
    pub fn ninsns_rvoff(from: to -) -> return;
}
// Return -1 or inverted cond.
extern "C" {
    pub fn ninsns_rvoff(from: to -) -> return;
}
// Instruction formats.
extern "C" {
    pub fn rv_r_insn(_arg: funct7, _arg: rs2, _arg: rs1, _arg: funct3, _arg: rd, _arg: opcode) -> return;
}
// RISC-V compressed instruction formats.
// Instructions shared by both RV32 and RV64.
extern "C" {
    pub fn rv_i_insn(_arg: imm11_0, _arg: rs1, _arg: 0, _arg: rd, _arg: 0x13) -> return;
}
extern "C" {
    pub fn rv_i_insn(_arg: imm11_0, _arg: rs1, _arg: 7, _arg: rd, _arg: 0x13) -> return;
}
extern "C" {
    pub fn rv_i_insn(_arg: imm11_0, _arg: rs1, _arg: 6, _arg: rd, _arg: 0x13) -> return;
}
extern "C" {
    pub fn rv_i_insn(_arg: imm11_0, _arg: rs1, _arg: 4, _arg: rd, _arg: 0x13) -> return;
}
extern "C" {
    pub fn rv_i_insn(_arg: imm11_0, _arg: rs1, _arg: 1, _arg: rd, _arg: 0x13) -> return;
}
extern "C" {
    pub fn rv_i_insn(_arg: imm11_0, _arg: rs1, _arg: 5, _arg: rd, _arg: 0x13) -> return;
}
extern "C" {
    pub fn rv_i_insn(imm11_0: 0x400 |, _arg: rs1, _arg: 5, _arg: rd, _arg: 0x13) -> return;
}
extern "C" {
    pub fn rv_u_insn(_arg: imm31_12, _arg: rd, _arg: 0x37) -> return;
}
extern "C" {
    pub fn rv_u_insn(_arg: imm31_12, _arg: rd, _arg: 0x17) -> return;
}
extern "C" {
    pub fn rv_r_insn(_arg: 0, _arg: rs2, _arg: rs1, _arg: 0, _arg: rd, _arg: 0x33) -> return;
}
extern "C" {
    pub fn rv_r_insn(_arg: 0x20, _arg: rs2, _arg: rs1, _arg: 0, _arg: rd, _arg: 0x33) -> return;
}
extern "C" {
    pub fn rv_r_insn(_arg: 0, _arg: rs2, _arg: rs1, _arg: 3, _arg: rd, _arg: 0x33) -> return;
}
extern "C" {
    pub fn rv_r_insn(_arg: 0, _arg: rs2, _arg: rs1, _arg: 7, _arg: rd, _arg: 0x33) -> return;
}
extern "C" {
    pub fn rv_r_insn(_arg: 0, _arg: rs2, _arg: rs1, _arg: 6, _arg: rd, _arg: 0x33) -> return;
}
extern "C" {
    pub fn rv_r_insn(_arg: 0, _arg: rs2, _arg: rs1, _arg: 4, _arg: rd, _arg: 0x33) -> return;
}
extern "C" {
    pub fn rv_r_insn(_arg: 0, _arg: rs2, _arg: rs1, _arg: 1, _arg: rd, _arg: 0x33) -> return;
}
extern "C" {
    pub fn rv_r_insn(_arg: 0, _arg: rs2, _arg: rs1, _arg: 5, _arg: rd, _arg: 0x33) -> return;
}
extern "C" {
    pub fn rv_r_insn(_arg: 0x20, _arg: rs2, _arg: rs1, _arg: 5, _arg: rd, _arg: 0x33) -> return;
}
extern "C" {
    pub fn rv_r_insn(_arg: 1, _arg: rs2, _arg: rs1, _arg: 0, _arg: rd, _arg: 0x33) -> return;
}
extern "C" {
    pub fn rv_r_insn(_arg: 1, _arg: rs2, _arg: rs1, _arg: 3, _arg: rd, _arg: 0x33) -> return;
}
extern "C" {
    pub fn rv_r_insn(_arg: 1, _arg: rs2, _arg: rs1, _arg: 4, _arg: rd, _arg: 0x33) -> return;
}
extern "C" {
    pub fn rv_r_insn(_arg: 1, _arg: rs2, _arg: rs1, _arg: 5, _arg: rd, _arg: 0x33) -> return;
}
extern "C" {
    pub fn rv_r_insn(_arg: 1, _arg: rs2, _arg: rs1, _arg: 6, _arg: rd, _arg: 0x33) -> return;
}
extern "C" {
    pub fn rv_r_insn(_arg: 1, _arg: rs2, _arg: rs1, _arg: 7, _arg: rd, _arg: 0x33) -> return;
}
extern "C" {
    pub fn rv_j_insn(_arg: imm20_1, _arg: rd, _arg: 0x6f) -> return;
}
extern "C" {
    pub fn rv_i_insn(_arg: imm11_0, _arg: rs1, _arg: 0, _arg: rd, _arg: 0x67) -> return;
}
extern "C" {
    pub fn rv_b_insn(_arg: imm12_1, _arg: rs2, _arg: rs1, _arg: 0, _arg: 0x63) -> return;
}
extern "C" {
    pub fn rv_b_insn(_arg: imm12_1, _arg: rs2, _arg: rs1, _arg: 1, _arg: 0x63) -> return;
}
extern "C" {
    pub fn rv_b_insn(_arg: imm12_1, _arg: rs2, _arg: rs1, _arg: 6, _arg: 0x63) -> return;
}
extern "C" {
    pub fn rv_bltu(_arg: rs2, _arg: rs1, _arg: imm12_1) -> return;
}
extern "C" {
    pub fn rv_b_insn(_arg: imm12_1, _arg: rs2, _arg: rs1, _arg: 7, _arg: 0x63) -> return;
}
extern "C" {
    pub fn rv_bgeu(_arg: rs2, _arg: rs1, _arg: imm12_1) -> return;
}
extern "C" {
    pub fn rv_b_insn(_arg: imm12_1, _arg: rs2, _arg: rs1, _arg: 4, _arg: 0x63) -> return;
}
extern "C" {
    pub fn rv_blt(_arg: rs2, _arg: rs1, _arg: imm12_1) -> return;
}
extern "C" {
    pub fn rv_b_insn(_arg: imm12_1, _arg: rs2, _arg: rs1, _arg: 5, _arg: 0x63) -> return;
}
extern "C" {
    pub fn rv_bge(_arg: rs2, _arg: rs1, _arg: imm12_1) -> return;
}
extern "C" {
    pub fn rv_i_insn(_arg: imm11_0, _arg: rs1, _arg: 0, _arg: rd, _arg: 0x03) -> return;
}
extern "C" {
    pub fn rv_i_insn(_arg: imm11_0, _arg: rs1, _arg: 1, _arg: rd, _arg: 0x03) -> return;
}
extern "C" {
    pub fn rv_i_insn(_arg: imm11_0, _arg: rs1, _arg: 2, _arg: rd, _arg: 0x03) -> return;
}
extern "C" {
    pub fn rv_i_insn(_arg: imm11_0, _arg: rs1, _arg: 4, _arg: rd, _arg: 0x03) -> return;
}
extern "C" {
    pub fn rv_i_insn(_arg: imm11_0, _arg: rs1, _arg: 5, _arg: rd, _arg: 0x03) -> return;
}
extern "C" {
    pub fn rv_s_insn(_arg: imm11_0, _arg: rs2, _arg: rs1, _arg: 0, _arg: 0x23) -> return;
}
extern "C" {
    pub fn rv_s_insn(_arg: imm11_0, _arg: rs2, _arg: rs1, _arg: 1, _arg: 0x23) -> return;
}
extern "C" {
    pub fn rv_s_insn(_arg: imm11_0, _arg: rs2, _arg: rs1, _arg: 2, _arg: 0x23) -> return;
}
extern "C" {
    pub fn rv_amo_insn(_arg: 0, _arg: aq, _arg: rl, _arg: rs2, _arg: rs1, _arg: 2, _arg: rd, _arg: 0x2f) -> return;
}
extern "C" {
    pub fn rv_amo_insn(_arg: 0xc, _arg: aq, _arg: rl, _arg: rs2, _arg: rs1, _arg: 2, _arg: rd, _arg: 0x2f) -> return;
}
extern "C" {
    pub fn rv_amo_insn(_arg: 0x8, _arg: aq, _arg: rl, _arg: rs2, _arg: rs1, _arg: 2, _arg: rd, _arg: 0x2f) -> return;
}
extern "C" {
    pub fn rv_amo_insn(_arg: 0x4, _arg: aq, _arg: rl, _arg: rs2, _arg: rs1, _arg: 2, _arg: rd, _arg: 0x2f) -> return;
}
extern "C" {
    pub fn rv_amo_insn(_arg: 0x1, _arg: aq, _arg: rl, _arg: rs2, _arg: rs1, _arg: 2, _arg: rd, _arg: 0x2f) -> return;
}
extern "C" {
    pub fn rv_amo_insn(_arg: 0x2, _arg: aq, _arg: rl, _arg: rs2, _arg: rs1, _arg: 2, _arg: rd, _arg: 0x2f) -> return;
}
extern "C" {
    pub fn rv_amo_insn(_arg: 0x3, _arg: aq, _arg: rl, _arg: rs2, _arg: rs1, _arg: 2, _arg: rd, _arg: 0x2f) -> return;
}
extern "C" {
    pub fn rv_i_insn(_arg: imm11_0, _arg: 0, _arg: 0, _arg: 0, _arg: 0xf) -> return;
}
extern "C" {
    pub fn rv_i_insn(_arg: 0, _arg: 0, _arg: 0, _arg: 0, _arg: 0x13) -> return;
}
// RVC instructions.
extern "C" {
    pub fn rv_ciw_insn(_arg: 0x0, _arg: imm, _arg: rd, _arg: 0x0) -> return;
}
extern "C" {
    pub fn rv_cl_insn(_arg: 0x2, _arg: imm_hi, _arg: rs1, _arg: imm_lo, _arg: rd, _arg: 0x0) -> return;
}
extern "C" {
    pub fn rv_cs_insn(_arg: 0x6, _arg: imm_hi, _arg: rs1, _arg: imm_lo, _arg: rs2, _arg: 0x0) -> return;
}
extern "C" {
    pub fn rv_ci_insn(_arg: 0, _arg: imm6, _arg: rd, _arg: 0x1) -> return;
}
extern "C" {
    pub fn rv_ci_insn(_arg: 0x2, _arg: imm6, _arg: rd, _arg: 0x1) -> return;
}
extern "C" {
    pub fn rv_ci_insn(_arg: 0x3, _arg: imm, _arg: RV_REG_SP, _arg: 0x1) -> return;
}
extern "C" {
    pub fn rv_ci_insn(_arg: 0x3, _arg: imm6, _arg: rd, _arg: 0x1) -> return;
}
extern "C" {
    pub fn rv_cb_insn(_arg: 0x4, _arg: imm6, _arg: 0, _arg: rd, _arg: 0x1) -> return;
}
extern "C" {
    pub fn rv_cb_insn(_arg: 0x4, _arg: imm6, _arg: 0x1, _arg: rd, _arg: 0x1) -> return;
}
extern "C" {
    pub fn rv_cb_insn(_arg: 0x4, _arg: imm6, _arg: 0x2, _arg: rd, _arg: 0x1) -> return;
}
extern "C" {
    pub fn rv_ca_insn(_arg: 0x23, _arg: rd, _arg: 0, _arg: rs, _arg: 0x1) -> return;
}
extern "C" {
    pub fn rv_ca_insn(_arg: 0x23, _arg: rd, _arg: 0x1, _arg: rs, _arg: 0x1) -> return;
}
extern "C" {
    pub fn rv_ca_insn(_arg: 0x23, _arg: rd, _arg: 0x2, _arg: rs, _arg: 0x1) -> return;
}
extern "C" {
    pub fn rv_ca_insn(_arg: 0x23, _arg: rd, _arg: 0x3, _arg: rs, _arg: 0x1) -> return;
}
extern "C" {
    pub fn rv_ci_insn(_arg: 0, _arg: imm6, _arg: rd, _arg: 0x2) -> return;
}
extern "C" {
    pub fn rv_ci_insn(_arg: 0x2, _arg: imm, _arg: rd, _arg: 0x2) -> return;
}
extern "C" {
    pub fn rv_cr_insn(_arg: 0x8, _arg: rs1, _arg: RV_REG_ZERO, _arg: 0x2) -> return;
}
extern "C" {
    pub fn rv_cr_insn(_arg: 0x8, _arg: rd, _arg: rs, _arg: 0x2) -> return;
}
extern "C" {
    pub fn rv_cr_insn(_arg: 0x9, _arg: rs1, _arg: RV_REG_ZERO, _arg: 0x2) -> return;
}
extern "C" {
    pub fn rv_cr_insn(_arg: 0x9, _arg: rd, _arg: rs, _arg: 0x2) -> return;
}
extern "C" {
    pub fn rv_css_insn(_arg: 0x6, _arg: imm, _arg: rs2, _arg: 0x2) -> return;
}
// RVZACAS instructions.
extern "C" {
    pub fn rv_amo_insn(_arg: 0x5, _arg: aq, _arg: rl, _arg: rs2, _arg: rs1, _arg: 2, _arg: rd, _arg: 0x2f) -> return;
}
extern "C" {
    pub fn rv_amo_insn(_arg: 0x5, _arg: aq, _arg: rl, _arg: rs2, _arg: rs1, _arg: 3, _arg: rd, _arg: 0x2f) -> return;
}
// RVZBA instructions.
extern "C" {
    pub fn rv_r_insn(_arg: 0x10, _arg: rs2, _arg: rs1, _arg: 0x4, _arg: rd, _arg: 0x33) -> return;
}
extern "C" {
    pub fn rv_r_insn(_arg: 0x10, _arg: rs2, _arg: rs1, _arg: 0x6, _arg: rd, _arg: 0x33) -> return;
}
// RVZBB instructions.
extern "C" {
    pub fn rv_i_insn(_arg: 0x604, _arg: rs1, _arg: 1, _arg: rd, _arg: 0x13) -> return;
}
extern "C" {
    pub fn rv_i_insn(_arg: 0x605, _arg: rs1, _arg: 1, _arg: rd, _arg: 0x13) -> return;
}
extern "C" {
    pub fn rv_i_insn(_arg: 0x80, _arg: rs, _arg: 4, _arg: rd, _arg: 0x3b) -> return;
}
extern "C" {
    pub fn rv_i_insn(_arg: 0x80, _arg: rs, _arg: 4, _arg: rd, _arg: 0x33) -> return;
}
extern "C" {
    pub fn rv_i_insn(_arg: 0x6b8, _arg: rs, _arg: 5, _arg: rd, _arg: 0x13) -> return;
}
extern "C" {
    pub fn rv_i_insn(_arg: 0x698, _arg: rs, _arg: 5, _arg: rd, _arg: 0x13) -> return;
}
//
// RV64-only instructions.
//
// These instructions are not available on RV32.  Wrap them below a #if to
// ensure that the RV32 JIT doesn't emit any of these instructions.
//

extern "C" {
    pub fn rv_i_insn(_arg: imm11_0, _arg: rs1, _arg: 0, _arg: rd, _arg: 0x1b) -> return;
}
extern "C" {
    pub fn rv_i_insn(_arg: imm11_0, _arg: rs1, _arg: 1, _arg: rd, _arg: 0x1b) -> return;
}
extern "C" {
    pub fn rv_i_insn(_arg: imm11_0, _arg: rs1, _arg: 5, _arg: rd, _arg: 0x1b) -> return;
}
extern "C" {
    pub fn rv_i_insn(imm11_0: 0x400 |, _arg: rs1, _arg: 5, _arg: rd, _arg: 0x1b) -> return;
}
extern "C" {
    pub fn rv_r_insn(_arg: 0, _arg: rs2, _arg: rs1, _arg: 0, _arg: rd, _arg: 0x3b) -> return;
}
extern "C" {
    pub fn rv_r_insn(_arg: 0x20, _arg: rs2, _arg: rs1, _arg: 0, _arg: rd, _arg: 0x3b) -> return;
}
extern "C" {
    pub fn rv_r_insn(_arg: 0, _arg: rs2, _arg: rs1, _arg: 1, _arg: rd, _arg: 0x3b) -> return;
}
extern "C" {
    pub fn rv_r_insn(_arg: 0, _arg: rs2, _arg: rs1, _arg: 5, _arg: rd, _arg: 0x3b) -> return;
}
extern "C" {
    pub fn rv_r_insn(_arg: 0x20, _arg: rs2, _arg: rs1, _arg: 5, _arg: rd, _arg: 0x3b) -> return;
}
extern "C" {
    pub fn rv_r_insn(_arg: 1, _arg: rs2, _arg: rs1, _arg: 0, _arg: rd, _arg: 0x3b) -> return;
}
extern "C" {
    pub fn rv_r_insn(_arg: 1, _arg: rs2, _arg: rs1, _arg: 4, _arg: rd, _arg: 0x3b) -> return;
}
extern "C" {
    pub fn rv_r_insn(_arg: 1, _arg: rs2, _arg: rs1, _arg: 5, _arg: rd, _arg: 0x3b) -> return;
}
extern "C" {
    pub fn rv_r_insn(_arg: 1, _arg: rs2, _arg: rs1, _arg: 6, _arg: rd, _arg: 0x3b) -> return;
}
extern "C" {
    pub fn rv_r_insn(_arg: 1, _arg: rs2, _arg: rs1, _arg: 7, _arg: rd, _arg: 0x3b) -> return;
}
extern "C" {
    pub fn rv_i_insn(_arg: imm11_0, _arg: rs1, _arg: 3, _arg: rd, _arg: 0x03) -> return;
}
extern "C" {
    pub fn rv_i_insn(_arg: imm11_0, _arg: rs1, _arg: 6, _arg: rd, _arg: 0x03) -> return;
}
extern "C" {
    pub fn rv_s_insn(_arg: imm11_0, _arg: rs2, _arg: rs1, _arg: 3, _arg: 0x23) -> return;
}
extern "C" {
    pub fn rv_amo_insn(_arg: 0, _arg: aq, _arg: rl, _arg: rs2, _arg: rs1, _arg: 3, _arg: rd, _arg: 0x2f) -> return;
}
extern "C" {
    pub fn rv_amo_insn(_arg: 0xc, _arg: aq, _arg: rl, _arg: rs2, _arg: rs1, _arg: 3, _arg: rd, _arg: 0x2f) -> return;
}
extern "C" {
    pub fn rv_amo_insn(_arg: 0x8, _arg: aq, _arg: rl, _arg: rs2, _arg: rs1, _arg: 3, _arg: rd, _arg: 0x2f) -> return;
}
extern "C" {
    pub fn rv_amo_insn(_arg: 0x4, _arg: aq, _arg: rl, _arg: rs2, _arg: rs1, _arg: 3, _arg: rd, _arg: 0x2f) -> return;
}
extern "C" {
    pub fn rv_amo_insn(_arg: 0x1, _arg: aq, _arg: rl, _arg: rs2, _arg: rs1, _arg: 3, _arg: rd, _arg: 0x2f) -> return;
}
extern "C" {
    pub fn rv_amo_insn(_arg: 0x2, _arg: aq, _arg: rl, _arg: rs2, _arg: rs1, _arg: 3, _arg: rd, _arg: 0x2f) -> return;
}
extern "C" {
    pub fn rv_amo_insn(_arg: 0x3, _arg: aq, _arg: rl, _arg: rs2, _arg: rs1, _arg: 3, _arg: rd, _arg: 0x2f) -> return;
}
// RV64-only RVC instructions.
extern "C" {
    pub fn rv_cl_insn(_arg: 0x3, _arg: imm_hi, _arg: rs1, _arg: imm_lo, _arg: rd, _arg: 0x0) -> return;
}
extern "C" {
    pub fn rv_cs_insn(_arg: 0x7, _arg: imm_hi, _arg: rs1, _arg: imm_lo, _arg: rs2, _arg: 0x0) -> return;
}
extern "C" {
    pub fn rv_ca_insn(_arg: 0x27, _arg: rd, _arg: 0, _arg: rs, _arg: 0x1) -> return;
}
extern "C" {
    pub fn rv_ci_insn(_arg: 0x1, _arg: imm6, _arg: rd, _arg: 0x1) -> return;
}
extern "C" {
    pub fn rv_ci_insn(_arg: 0x3, _arg: imm, _arg: rd, _arg: 0x2) -> return;
}
extern "C" {
    pub fn rv_css_insn(_arg: 0x7, _arg: imm, _arg: rs2, _arg: 0x2) -> return;
}
// RV64-only ZBA instructions.
// add.uw rd, rs1, ZERO
extern "C" {
    pub fn rv_r_insn(_arg: 0x04, _arg: RV_REG_ZERO, _arg: rs1, _arg: 0, _arg: rd, _arg: 0x3b) -> return;
}

// Helper functions that emit RVC instructions when possible.
// RV64-only helper functions.

extern "C" {
    pub fn bpf_jit_build_prologue(ctx: *mut rv_jit_context, is_subprog: bool);
}
extern "C" {
    pub fn bpf_jit_build_epilogue(ctx: *mut rv_jit_context);
}
