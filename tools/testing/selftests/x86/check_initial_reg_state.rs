//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/x86/check_initial_reg_state.c
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
// check_initial_reg_state.c - check that execve sets the correct state
// Copyright (c) 2014-2016 Andrew Lutomirski
//
// Macro flag: #define _GNU_SOURCE

    unsigned long ax, bx, cx, dx, si, di, bp, sp, flags;
    unsigned long r8, r9, r10, r11, r12, r13, r14, r15;
    asm (
    ".pushsection .text\n\t"
    ".type real_start, @function\n\t"
    ".global real_start\n\t"
    "real_start:\n\t"

    "mov %rax, ax\n\t"
    "mov %rbx, bx\n\t"
    "mov %rcx, cx\n\t"
    "mov %rdx, dx\n\t"
    "mov %rsi, si\n\t"
    "mov %rdi, di\n\t"
    "mov %rbp, bp\n\t"
    "mov %rsp, sp\n\t"
    "mov %r8, r8\n\t"
    "mov %r9, r9\n\t"
    "mov %r10, r10\n\t"
    "mov %r11, r11\n\t"
    "mov %r12, r12\n\t"
    "mov %r13, r13\n\t"
    "mov %r14, r14\n\t"
    "mov %r15, r15\n\t"
    "pushfq\n\t"
    "popq flags\n\t"

    "mov %eax, ax\n\t"
    "mov %ebx, bx\n\t"
    "mov %ecx, cx\n\t"
    "mov %edx, dx\n\t"
    "mov %esi, si\n\t"
    "mov %edi, di\n\t"
    "mov %ebp, bp\n\t"
    "mov %esp, sp\n\t"
    "pushfl\n\t"
    "popl flags\n\t"

    "jmp _start\n\t"
    ".size real_start, . - real_start\n\t"
    ".popsection");
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main()
    {
    let mut nerrs: c_int = 0;
    if (sp == 0) {
    printf("[FAIL]\tTest was built incorrectly\n");
    return 1;
    }
    if (ax || bx || cx || dx || si || di || bp

    || r8 || r9 || r10 || r11 || r12 || r13 || r14 || r15

    ) {
    printf("[FAIL]\tAll GPRs except SP should be 0\n");

    SHOW(ax);
    SHOW(bx);
    SHOW(cx);
    SHOW(dx);
    SHOW(si);
    SHOW(di);
    SHOW(bp);
    SHOW(sp);

    SHOW(r8);
    SHOW(r9);
    SHOW(r10);
    SHOW(r11);
    SHOW(r12);
    SHOW(r13);
    SHOW(r14);
    SHOW(r15);

    nerrs++;
    } else {
    printf("[OK]\tAll GPRs except SP are 0\n");
    }
    if (flags != 0x202) {
    printf("[FAIL]\tFLAGS is 0x%lx, but it should be 0x202\n", flags);
    nerrs++;
    } else {
    printf("[OK]\tFLAGS is 0x202\n");
    }
    return nerrs ? 1 : 0;
    }
