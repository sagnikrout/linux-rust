//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/cacheflush.h
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
// Copyright (C) 2015 Regents of the University of California
//

extern "C" {
    pub fn volatile("memory": "fence.i" :::) -> asm;
}

pub const ARCH_IMPLEMENTS_FLUSH_DCACHE_PAGE: c_int = 1;

// This is accessed in assembly code. cpumask_var_t would be too complex.
extern "C" {
    pub fn DECLARE_BITMAP(_arg: new_valid_map_cpus, _arg: NR_CPUS) -> extern;
}
//
// We don't care if concurrently a cpu resets this value since
// the only place this can happen is in handle_exception() where
// an sfence.vma is emitted.
//

extern "C" {
    pub fn flush_icache_all();
}
extern "C" {
    pub fn flush_icache_mm(mm: *mut mm_struct, local: bool);
}

//
// RISC-V doesn't have an instruction to flush parts of the instruction cache,
// so instead we just flush the whole thing.
//

extern "C" {
    pub fn riscv_init_cbo_blocksizes();
}

extern "C" {
    pub fn riscv_noncoherent_supported();
}
extern "C" {
    pub fn riscv_set_dma_cache_alignment() -> void __init;
}

//
// Bits in sys_riscv_flush_icache()'s flags argument.
//

