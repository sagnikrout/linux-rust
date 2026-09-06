//! Automatically rewritten from C to Rust
//! Source: lib/raid/xor/riscv/xor-glue.c
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
// Copyright (C) 2021 SiFive
//

    DO_XOR_BLOCKS(vector_inner, xor_regs_2_, xor_regs_3_, xor_regs_4_, xor_regs_5_);
    static void xor_gen_vector(void *dest, void **srcs, unsigned int src_cnt,
    unsigned int bytes)
    {
    kernel_vector_begin();
    xor_gen_vector_inner(dest, srcs, src_cnt, bytes);
    kernel_vector_end();
    }
    struct xor_block_template xor_block_rvv = {
    .name		= "rvv",
    .xor_gen	= xor_gen_vector,
    };
