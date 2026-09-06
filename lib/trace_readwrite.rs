//! Automatically rewritten from C to Rust
//! Source: lib/trace_readwrite.c
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
// Register read and write tracepoints
//
// Copyright (c) 2021-2022 Qualcomm Innovation Center, Inc. All rights reserved.
//

// Macro flag: #define CREATE_TRACE_POINTS

    void log_write_mmio(u64 val, u8 width, volatile void __iomem *addr,
    unsigned long caller_addr, unsigned long caller_addr0)
    {
    trace_rwmmio_write(caller_addr, caller_addr0, val, width, addr);
    }
    EXPORT_SYMBOL_GPL(log_write_mmio);
    EXPORT_TRACEPOINT_SYMBOL_GPL(rwmmio_write);
    void log_post_write_mmio(u64 val, u8 width, volatile void __iomem *addr,
    unsigned long caller_addr, unsigned long caller_addr0)
    {
    trace_rwmmio_post_write(caller_addr, caller_addr0, val, width, addr);
    }
    EXPORT_SYMBOL_GPL(log_post_write_mmio);
    EXPORT_TRACEPOINT_SYMBOL_GPL(rwmmio_post_write);
    void log_read_mmio(u8 width, const volatile void __iomem *addr,
    unsigned long caller_addr, unsigned long caller_addr0)
    {
    trace_rwmmio_read(caller_addr, caller_addr0, width, addr);
    }
    EXPORT_SYMBOL_GPL(log_read_mmio);
    EXPORT_TRACEPOINT_SYMBOL_GPL(rwmmio_read);
    void log_post_read_mmio(u64 val, u8 width, const volatile void __iomem *addr,
    unsigned long caller_addr, unsigned long caller_addr0)
    {
    trace_rwmmio_post_read(caller_addr, caller_addr0, val, width, addr);
    }
    EXPORT_SYMBOL_GPL(log_post_read_mmio);
    EXPORT_TRACEPOINT_SYMBOL_GPL(rwmmio_post_read);
