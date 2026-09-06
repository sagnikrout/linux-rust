//! Automatically rewritten from C to Rust
//! Source: arch/x86/hyperv/mshv-asm-offsets.c
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
// Generate definitions needed by assembly language modules.
// This code generates raw asm output which is post-processed to extract
// and format the required data.
//
// Copyright (c) 2025, Microsoft Corporation.
//
// Author:
// Naman Jain <namjain@microsoft.com>
//
// Macro flag: #define COMPILE_OFFSETS

#[no_mangle]
unsafe extern "C" fn common() -> void __used {
    static void __used common(void)
    {
    if (IS_ENABLED(CONFIG_HYPERV_VTL_MODE)) {
    OFFSET(MSHV_VTL_CPU_CONTEXT_rax, mshv_vtl_cpu_context, rax);
    OFFSET(MSHV_VTL_CPU_CONTEXT_rcx, mshv_vtl_cpu_context, rcx);
    OFFSET(MSHV_VTL_CPU_CONTEXT_rdx, mshv_vtl_cpu_context, rdx);
    OFFSET(MSHV_VTL_CPU_CONTEXT_rbx, mshv_vtl_cpu_context, rbx);
    OFFSET(MSHV_VTL_CPU_CONTEXT_rbp, mshv_vtl_cpu_context, rbp);
    OFFSET(MSHV_VTL_CPU_CONTEXT_rsi, mshv_vtl_cpu_context, rsi);
    OFFSET(MSHV_VTL_CPU_CONTEXT_rdi, mshv_vtl_cpu_context, rdi);
    OFFSET(MSHV_VTL_CPU_CONTEXT_r8,  mshv_vtl_cpu_context, r8);
    OFFSET(MSHV_VTL_CPU_CONTEXT_r9,  mshv_vtl_cpu_context, r9);
    OFFSET(MSHV_VTL_CPU_CONTEXT_r10, mshv_vtl_cpu_context, r10);
    OFFSET(MSHV_VTL_CPU_CONTEXT_r11, mshv_vtl_cpu_context, r11);
    OFFSET(MSHV_VTL_CPU_CONTEXT_r12, mshv_vtl_cpu_context, r12);
    OFFSET(MSHV_VTL_CPU_CONTEXT_r13, mshv_vtl_cpu_context, r13);
    OFFSET(MSHV_VTL_CPU_CONTEXT_r14, mshv_vtl_cpu_context, r14);
    OFFSET(MSHV_VTL_CPU_CONTEXT_r15, mshv_vtl_cpu_context, r15);
    OFFSET(MSHV_VTL_CPU_CONTEXT_cr2, mshv_vtl_cpu_context, cr2);
    }
    }
