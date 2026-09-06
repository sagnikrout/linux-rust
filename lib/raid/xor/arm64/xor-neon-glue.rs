//! Automatically rewritten from C to Rust
//! Source: lib/raid/xor/arm64/xor-neon-glue.c
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
// Authors: Jackie Liu <liuyun01@kylinos.cn>
// Copyright (C) 2018,Tianjin KYLIN Information Technology Co., Ltd.
//

    static void xor_gen_##_name(void *dest, void **srcs, unsigned int src_cnt, \
    unsigned int bytes)					\
    {									\
    scoped_ksimd()							\
    xor_gen_##_name##_inner(dest, srcs, src_cnt, bytes);	\
    }									\
    \
    struct xor_block_template xor_block_##_name = {				\
    .name   	= __stringify(_name),				\
    .xor_gen	= xor_gen_##_name,				\
    };
    XOR_TEMPLATE(neon);
    XOR_TEMPLATE(eor3);
