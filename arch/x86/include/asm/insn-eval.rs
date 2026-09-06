//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/insn-eval.h
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


//
// A collection of utility functions for x86 instruction analysis to be
// used in a kernel context. Useful when, for instance, making sense
// of the registers indicated by operands.
//

extern "C" {
    pub fn pt_regs_offset(regs: *mut pt_regs, regno: c_int) -> c_int;
}
extern "C" {
    pub fn insn_has_rep_prefix(insn: *mut insn) -> bool;
}
extern "C" {
    pub fn insn_get_modrm_rm_off(insn: *mut insn, regs: *mut pt_regs) -> c_int;
}
extern "C" {
    pub fn insn_get_modrm_reg_off(insn: *mut insn, regs: *mut pt_regs) -> c_int;
}
extern "C" {
    pub fn insn_get_seg_base(regs: *mut pt_regs, seg_reg_idx: c_int) -> c_ulong;
}
extern "C" {
    pub fn insn_get_code_seg_params(regs: *mut pt_regs) -> c_int;
}
extern "C" {
    pub fn insn_get_effective_ip(regs: *mut pt_regs, ip: *mut c_ulong) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum insn_mmio_type {
    INSN_MMIO_DECODE_FAILED,
    INSN_MMIO_WRITE,
    INSN_MMIO_WRITE_IMM,
    INSN_MMIO_READ,
    INSN_MMIO_READ_ZERO_EXTEND,
    INSN_MMIO_READ_SIGN_EXTEND,
    INSN_MMIO_MOVS,
}

extern "C" {
    pub fn insn_decode_mmio(insn: *mut insn, bytes: *mut c_int) -> insn_mmio_type;
}
extern "C" {
    pub fn insn_is_nop(insn: *mut insn) -> bool;
}
//
// Write @val into *@reg following the x86 rules for writes to
// general-purpose registers (Intel SDM Vol. 1, "General-Purpose
// Registers in 64-Bit Mode"): an 8- or 16-bit write leaves the rest of
// the register untouched, a 32-bit write zero-extends the result into
// the upper 32 bits, and a 64-bit write replaces the whole register.
//
// @bytes is the width of the write, not a property of the instruction:
// an instruction that, say, sign-extends a 32-bit immediate into a
// 64-bit register does a 64-bit write here.
//
// @reg need not be 8-byte aligned: KVM's instruction emulator offsets
// the pointer by one byte to address the high-byte registers (AH, CH,
// DH, BH).  Use narrow stores for the sub-word cases so the access
// width matches @bytes and the adjacent bytes are left alone.
//
// (u8 *)reg = (u8)val;
// (u16 *)reg = (u16)val;
// A 32-bit write zero-extends into the upper 32 bits.
// reg = (u32)val;
// reg = val;
