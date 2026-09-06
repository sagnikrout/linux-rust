//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/math-emu/mtfsf.c
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

    int
    mtfsf(unsigned int FM, u32 *frB)
    {
    u32 mask;
    u32 fpscr;
    if (likely(FM == 1))
    mask = 0x0f;
#[no_mangle]
pub unsafe extern "C" fn if(0xff): likely(FM ==) -> else {
    else if (likely(FM == 0xff))
    mask = ~0;
    else {
    mask = ((FM & 1) |
    ((FM << 3) & 0x10) |
    ((FM << 6) & 0x100) |
    ((FM << 9) & 0x1000) |
    ((FM << 12) & 0x10000) |
    ((FM << 15) & 0x100000) |
    ((FM << 18) & 0x1000000) |
    ((FM << 21) & 0x10000000)) * 15;
    }
    fpscr = ((__FPU_FPSCR & ~mask) | (frB[1] & mask)) &
    ~(FPSCR_VX | FPSCR_FEX | 0x800);
    if (fpscr & (FPSCR_VXSNAN | FPSCR_VXISI | FPSCR_VXIDI |
    FPSCR_VXZDZ | FPSCR_VXIMZ | FPSCR_VXVC |
    FPSCR_VXSOFT | FPSCR_VXSQRT | FPSCR_VXCVI))
    fpscr |= FPSCR_VX;
// The bit order of exception enables and exception status
// is the same. Simply shift and mask to check for enabled
// exceptions.
//
    if (fpscr & (fpscr >> 22) &  0xf8)
    fpscr |= FPSCR_FEX;
    __FPU_FPSCR = fpscr;

    printk("%s: %02x %p: %08lx\n", __func__, FM, frB, __FPU_FPSCR);

    return 0;
    }
