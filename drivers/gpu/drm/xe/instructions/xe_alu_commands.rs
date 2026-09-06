//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/instructions/xe_alu_commands.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2025 Intel Corporation
//

// Instruction Opcodes
pub const CS_ALU_OPCODE_NOOP: c_uint = 0x000;
pub const CS_ALU_OPCODE_FENCE_RD: c_uint = 0x001;
pub const CS_ALU_OPCODE_FENCE_WR: c_uint = 0x002;
pub const CS_ALU_OPCODE_LOAD: c_uint = 0x080;
pub const CS_ALU_OPCODE_LOADINV: c_uint = 0x480;
pub const CS_ALU_OPCODE_LOAD0: c_uint = 0x081;
pub const CS_ALU_OPCODE_LOAD1: c_uint = 0x481;
pub const CS_ALU_OPCODE_LOADIND: c_uint = 0x082;
pub const CS_ALU_OPCODE_ADD: c_uint = 0x100;
pub const CS_ALU_OPCODE_SUB: c_uint = 0x101;
pub const CS_ALU_OPCODE_AND: c_uint = 0x102;
pub const CS_ALU_OPCODE_OR: c_uint = 0x103;
pub const CS_ALU_OPCODE_XOR: c_uint = 0x104;
pub const CS_ALU_OPCODE_SHL: c_uint = 0x105;
pub const CS_ALU_OPCODE_SHR: c_uint = 0x106;
pub const CS_ALU_OPCODE_SAR: c_uint = 0x107;
pub const CS_ALU_OPCODE_STORE: c_uint = 0x180;
pub const CS_ALU_OPCODE_STOREINV: c_uint = 0x580;
pub const CS_ALU_OPCODE_STOREIND: c_uint = 0x181;
// Instruction Operands

pub const CS_ALU_OPERAND_REG0: c_uint = 0x0;
pub const CS_ALU_OPERAND_REG1: c_uint = 0x1;
pub const CS_ALU_OPERAND_REG2: c_uint = 0x2;
pub const CS_ALU_OPERAND_REG3: c_uint = 0x3;
pub const CS_ALU_OPERAND_REG4: c_uint = 0x4;
pub const CS_ALU_OPERAND_REG5: c_uint = 0x5;
pub const CS_ALU_OPERAND_REG6: c_uint = 0x6;
pub const CS_ALU_OPERAND_REG7: c_uint = 0x7;
pub const CS_ALU_OPERAND_REG8: c_uint = 0x8;
pub const CS_ALU_OPERAND_REG9: c_uint = 0x9;
pub const CS_ALU_OPERAND_REG10: c_uint = 0xa;
pub const CS_ALU_OPERAND_REG11: c_uint = 0xb;
pub const CS_ALU_OPERAND_REG12: c_uint = 0xc;
pub const CS_ALU_OPERAND_REG13: c_uint = 0xd;
pub const CS_ALU_OPERAND_REG14: c_uint = 0xe;
pub const CS_ALU_OPERAND_REG15: c_uint = 0xf;
pub const CS_ALU_OPERAND_SRCA: c_uint = 0x20;
pub const CS_ALU_OPERAND_SRCB: c_uint = 0x21;
pub const CS_ALU_OPERAND_ACCU: c_uint = 0x31;
pub const CS_ALU_OPERAND_ZF: c_uint = 0x32;
pub const CS_ALU_OPERAND_CF: c_uint = 0x33;

// Command Streamer ALU Instructions

