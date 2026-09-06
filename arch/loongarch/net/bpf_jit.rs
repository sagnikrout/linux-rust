//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/net/bpf_jit.h
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
// BPF JIT compiler for LoongArch
//
// Copyright (C) 2022 Loongson Technology Corporation Limited
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jit_ctx {
    pub prog: *const bpf_prog,
    pub idx: c_uint,
    pub flags: c_uint,
    pub epilogue_offset: c_uint,
    pub offset: *mut u32,
    pub num_exentries: c_int,
    pub image: *mut loongarch_instruction,
    pub ro_image: *mut loongarch_instruction,
    pub stack_size: u32,
    pub arena_vm_start: u64,
    pub user_vm_start: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jit_data {
    pub header: *mut bpf_binary_header,
    pub ro_header: *mut bpf_binary_header,
    pub ctx: jit_ctx,
}

// BPF JMP offset is relative to the next instruction
//
// Whereas LoongArch branch instructions encode the offset
// from the branch itself, so we must subtract 1 from the
// instruction offset.
//
// Zero-extend 32 bits into 64 bits
// Signed-extend 32 bits into 64 bits
// Emit proper extension according to ABI requirements.
// Note that it requires a value of size `size` already resides in register `reg`.
//
// ABI requires unsigned char/short to be zero-extended
// lu12iw rd, imm_31_12
// ori rd, rd, imm_11_0
// lu32id rd, imm_51_32
// lu52id rd, rd, imm_63_52
// or rd, $zero, $zero
// addiw rd, $zero, imm_11_0
// ori rd, $zero, imm_11_0
// lu52id rd, $zero, imm_63_52
// lu12iw rd, imm_31_12
// ori rd, rd, imm_11_0
//
// If bit[51:31] is all 0 or all 1,
// it means bit[51:32] is sign extended by lu12iw,
// no need to call lu32id to do a new filled operation.
//
// lu32id rd, imm_51_32
// lu52id rd, rd, imm_63_52
// PC += jmp_offset if rj == rd
// PC += jmp_offset if rj != rd
// PC += jmp_offset if rj > rd (unsigned)
// PC += jmp_offset if rj < rd (unsigned)
// PC += jmp_offset if rj >= rd (unsigned)
// PC += jmp_offset if rj <= rd (unsigned)
// PC += jmp_offset if rj > rd (signed)
// PC += jmp_offset if rj < rd (signed)
// PC += jmp_offset if rj >= rd (signed)
// PC += jmp_offset if rj <= rd (signed)
//
// A large PC-relative jump offset may overflow the immediate field of
// the native conditional branch instruction, triggering a conversion
// to use an absolute jump instead, this jump sequence is particularly
// nasty. For now, use cond_jmp_offs26() directly to keep it simple.
// In the future, maybe we can add support for far branching, the branch
// relaxation requires more than two passes to converge, the code seems
// too complex to understand, not quite sure whether it is necessary and
// worth the extra pain. Anyway, just leave it as it is to enhance code
// readability now.
//
