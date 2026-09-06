//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kernel/soc.c
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
// Copyright (C) 2020 Western Digital Corporation or its affiliates.
//

//
// This is called extremely early, before parse_dtb(), to allow initializing
// SoC hardware before memory or any device driver initialization.
//
#[no_mangle]
pub unsafe extern "C" fn soc_early_init() -> void __init {
    void __init soc_early_init(void)
    {
    void (*early_fn)(const void *fdt);
    const struct of_device_id *s;
    const void *fdt = dtb_early_va;
    for (s = (void *)&__soc_early_init_table_start;
    (void *)s < (void *)&__soc_early_init_table_end; s++) {
    if (!fdt_node_check_compatible(fdt, 0, s.compatible)) {
    early_fn = s.data;
    early_fn(fdt);
    return;
    }
    }
    }
