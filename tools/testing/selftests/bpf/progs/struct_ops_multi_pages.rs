//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/struct_ops_multi_pages.c
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
// Copyright (c) 2024 Meta Platforms, Inc. and affiliates.

    char _license[] SEC("license") = "GPL";

    SEC("struct_ops/tramp_" #x)		\
    int BPF_PROG(tramp_ ## x, int a)	\
    {					\
    return a;			\
    }
    TRAMP(1)
    TRAMP(2)
    TRAMP(3)
    TRAMP(4)
    TRAMP(5)
    TRAMP(6)
    TRAMP(7)
    TRAMP(8)
    TRAMP(9)
    TRAMP(10)
    TRAMP(11)
    TRAMP(12)
    TRAMP(13)
    TRAMP(14)
    TRAMP(15)
    TRAMP(16)
    TRAMP(17)
    TRAMP(18)
    TRAMP(19)
    TRAMP(20)
    TRAMP(21)
    TRAMP(22)
    TRAMP(23)
    TRAMP(24)
    TRAMP(25)
    TRAMP(26)
    TRAMP(27)
    TRAMP(28)
    TRAMP(29)
    TRAMP(30)
    TRAMP(31)
    TRAMP(32)
    TRAMP(33)
    TRAMP(34)
    TRAMP(35)
    TRAMP(36)
    TRAMP(37)
    TRAMP(38)
    TRAMP(39)
    TRAMP(40)

    SEC(".struct_ops.link")
    struct bpf_testmod_ops multi_pages = {
    F_TRAMP(1),
    F_TRAMP(2),
    F_TRAMP(3),
    F_TRAMP(4),
    F_TRAMP(5),
    F_TRAMP(6),
    F_TRAMP(7),
    F_TRAMP(8),
    F_TRAMP(9),
    F_TRAMP(10),
    F_TRAMP(11),
    F_TRAMP(12),
    F_TRAMP(13),
    F_TRAMP(14),
    F_TRAMP(15),
    F_TRAMP(16),
    F_TRAMP(17),
    F_TRAMP(18),
    F_TRAMP(19),
    F_TRAMP(20),
    F_TRAMP(21),
    F_TRAMP(22),
    F_TRAMP(23),
    F_TRAMP(24),
    F_TRAMP(25),
    F_TRAMP(26),
    F_TRAMP(27),
    F_TRAMP(28),
    F_TRAMP(29),
    F_TRAMP(30),
    F_TRAMP(31),
    F_TRAMP(32),
    F_TRAMP(33),
    F_TRAMP(34),
    F_TRAMP(35),
    F_TRAMP(36),
    F_TRAMP(37),
    F_TRAMP(38),
    F_TRAMP(39),
    F_TRAMP(40),
    };
