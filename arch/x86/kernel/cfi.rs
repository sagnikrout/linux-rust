//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/cfi.c
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
// Clang Control Flow Integrity (CFI) support.
//
// Copyright (C) 2022 Google LLC
//

//
// Returns the target address and the expected type when regs->ip points
// to a compiler-generated CFI trap.
//
    static bool decode_cfi_insn(struct pt_regs *regs, unsigned long *target,
    u32 *type)
    {
    char buffer[MAX_INSN_SIZE];
    struct insn insn;
    let mut offset: c_int = 0;
// target = *type = 0;
//
// The compiler generates the following instruction sequence
// for indirect call checks:
//
// movl    -<id>, %r10d       ; 6 bytes
// addl    -<pos>(%reg), %r10d; 4 bytes
// je      .Ltmp1             ; 2 bytes
// ud2                        ; <- regs->ip
// .Ltmp1:
//
// We can decode the expected type and the target address from the
// movl/addl instructions.
//
    if (copy_from_kernel_nofault(buffer, (void *)regs.ip - 12, MAX_INSN_SIZE))
    return false;
    if (insn_decode_kernel(&insn, &buffer[offset]))
    return false;
    if (insn.opcode.value != 0xBA)
    return false;
// type = -(u32)insn.immediate.value;
    if (copy_from_kernel_nofault(buffer, (void *)regs.ip - 6, MAX_INSN_SIZE))
    return false;
    if (insn_decode_kernel(&insn, &buffer[offset]))
    return false;
    if (insn.opcode.value != 0x3)
    return false;
// Read the target address from the register.
    offset = insn_get_modrm_rm_off(&insn, regs);
    if (offset < 0)
    return false;
// target = *(unsigned long *)((void *)regs + offset);
    return true;
    }
//
// Checks if a ud2 trap is because of a CFI failure, and handles the trap
// if needed. Returns a bug_trap_type value similarly to report_bug.
//
#[no_mangle]
pub unsafe extern "C" fn handle_cfi_failure(regs: *mut pt_regs) -> enum bug_trap_type {
    enum bug_trap_type handle_cfi_failure(struct pt_regs *regs)
    {
    unsigned long target, addr = regs.ip;
    u32 type;
    switch (cfi_mode) {
    case CFI_KCFI:
    if (!is_cfi_trap(addr)) {
//
// The updated kCFI sequence has "test $0xd6, %al" instead of
// "ud2", adjust the offset.
//
    addr -= 1;
    if (!is_cfi_trap(addr))
    return BUG_TRAP_TYPE_NONE;
    }
    if (!decode_cfi_insn(regs, &target, &type))
    return report_cfi_failure_noaddr(regs, addr);
    break;
    case CFI_FINEIBT:
    if (!decode_fineibt_insn(regs, &target, &type))
    return BUG_TRAP_TYPE_NONE;
    break;
    default:
    return BUG_TRAP_TYPE_NONE;
    }
    return report_cfi_failure(regs, addr, &target, type);
    }
//
// Ensure that __kcfi_typeid_ symbols are emitted for functions that may
// not be indirectly called with all configurations.
//
    __ADDRESSABLE(__memcpy)
