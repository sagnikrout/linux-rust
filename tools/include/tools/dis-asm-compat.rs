//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/tools/dis-asm-compat.h
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


// SPDX-License-Identifier: GPL-2.0-only OR BSD-2-Clause

// define types for older binutils version, to centralize ifdef'ery a bit

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum disassembler_style {
    typedef int (*fprintf_styled_ftype) (void *, enum disassembler_style, const char*, ...);

//
// Trivial fprintf wrapper to be used as the fprintf_styled_func argument to
// init_disassemble_info_compat() when normal fprintf suffices.
//
    static inline int fprintf_styled(void *out,
    enum disassembler_style style,
    const char *fmt, ...)
    {
    va_list args;
    int r;

    (void)style;

    va_start(args, fmt);
    r = vfprintf(out, fmt, args);
    va_end(args);

    return r;
    }

//
// Wrapper for init_disassemble_info() that hides version
// differences. Depending on binutils version and architecture either
// fprintf_func or fprintf_styled_func will be called.
//
    static inline void init_disassemble_info_compat(struct disassemble_info *info,
    void *stream,
    fprintf_ftype unstyled_func,
    fprintf_styled_ftype styled_func)
    {

    init_disassemble_info(info, stream,
    unstyled_func,
    styled_func);

    (void)styled_func;
    init_disassemble_info(info, stream,
    unstyled_func);

    }
