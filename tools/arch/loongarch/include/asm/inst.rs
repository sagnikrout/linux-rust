//! Automatically rewritten from C Header to Rust Module
//! Source: tools/arch/loongarch/include/asm/inst.h
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

pub const LOONGARCH_INSN_NOP: c_uint = 0x03400000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reg0i15_op {
    break_op	= 0x54,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reg0i26_op {
    b_op		= 0x14,
    bl_op		= 0x15,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reg1i21_op {
    beqz_op		= 0x10,
    bnez_op		= 0x11,
    bceqz_op	= 0x12, /* bits[9:8] = 0x00 */
    bcnez_op	= 0x12, /* bits[9:8] = 0x01 */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reg2_op {
    ertn_op		= 0x1920e,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reg2i12_op {
    addid_op	= 0x0b,
    andi_op		= 0x0d,
    ldd_op		= 0xa3,
    std_op		= 0xa7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reg2i14_op {
    ldptrd_op	= 0x26,
    stptrd_op	= 0x27,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reg2i16_op {
    jirl_op		= 0x13,
    beq_op		= 0x16,
    bne_op		= 0x17,
    blt_op		= 0x18,
    bge_op		= 0x19,
    bltu_op		= 0x1a,
    bgeu_op		= 0x1b,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reg3_op {
    amswapw_op	= 0x70c0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg0i15_format {
    pub 15: unsigned int immediate :,
    pub 17: unsigned int opcode :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg0i26_format {
    pub 10: unsigned int immediate_h :,
    pub 16: unsigned int immediate_l :,
    pub 6: unsigned int opcode :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg1i21_format {
    pub 5: unsigned int immediate_h :,
    pub 5: unsigned int rj :,
    pub 16: unsigned int immediate_l :,
    pub 6: unsigned int opcode :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg2_format {
    pub 5: unsigned int rd :,
    pub 5: unsigned int rj :,
    pub 22: unsigned int opcode :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg2i12_format {
    pub 5: unsigned int rd :,
    pub 5: unsigned int rj :,
    pub 12: unsigned int immediate :,
    pub 10: unsigned int opcode :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg2i14_format {
    pub 5: unsigned int rd :,
    pub 5: unsigned int rj :,
    pub 14: unsigned int immediate :,
    pub 8: unsigned int opcode :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg2i16_format {
    pub 5: unsigned int rd :,
    pub 5: unsigned int rj :,
    pub 16: unsigned int immediate :,
    pub 6: unsigned int opcode :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg3_format {
    pub 5: unsigned int rd :,
    pub 5: unsigned int rj :,
    pub 5: unsigned int rk :,
    pub 17: unsigned int opcode :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union loongarch_instruction {
    pub word: c_uint,
    pub reg0i15_format: reg0i15_format,
    pub reg0i26_format: reg0i26_format,
    pub reg1i21_format: reg1i21_format,
    pub reg2_format: reg2_format,
    pub reg2i12_format: reg2i12_format,
    pub reg2i14_format: reg2i14_format,
    pub reg2i16_format: reg2i16_format,
    pub reg3_format: reg3_format,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum loongarch_gpr {
    LOONGARCH_GPR_ZERO = 0,
    LOONGARCH_GPR_RA = 1,
    LOONGARCH_GPR_TP = 2,
    LOONGARCH_GPR_SP = 3,
    LOONGARCH_GPR_A0 = 4,	/* Reused as V0 for return value */
    LOONGARCH_GPR_A1,	/* Reused as V1 for return value */
    LOONGARCH_GPR_A2,
    LOONGARCH_GPR_A3,
    LOONGARCH_GPR_A4,
    LOONGARCH_GPR_A5,
    LOONGARCH_GPR_A6,
    LOONGARCH_GPR_A7,
    LOONGARCH_GPR_T0 = 12,
    LOONGARCH_GPR_T1,
    LOONGARCH_GPR_T2,
    LOONGARCH_GPR_T3,
    LOONGARCH_GPR_T4,
    LOONGARCH_GPR_T5,
    LOONGARCH_GPR_T6,
    LOONGARCH_GPR_T7,
    LOONGARCH_GPR_T8,
    LOONGARCH_GPR_FP = 22,
    LOONGARCH_GPR_S0 = 23,
    LOONGARCH_GPR_S1,
    LOONGARCH_GPR_S2,
    LOONGARCH_GPR_S3,
    LOONGARCH_GPR_S4,
    LOONGARCH_GPR_S5,
    LOONGARCH_GPR_S6,
    LOONGARCH_GPR_S7,
    LOONGARCH_GPR_S8,
    LOONGARCH_GPR_MAX
}

