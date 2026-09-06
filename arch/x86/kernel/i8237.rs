//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/i8237.c
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
// 8237A DMA controller suspend functions.
//
// Written by Pierre Ossman, 2005.
//

//
// This module just handles suspend/resume issues with the
// 8237A DMA controller (used for ISA and LPC).
// Allocation is handled in kernel/dma.c and normal usage is
// in asm/dma.h.
//
#[no_mangle]
unsafe extern "C" fn i8237A_resume(data: *mut c_void) {
    static void i8237A_resume(void *data)
    {
    unsigned long flags;
    int i;
    flags = claim_dma_lock();
    dma_outb(0, DMA1_RESET_REG);
    dma_outb(0, DMA2_RESET_REG);
    for (i = 0; i < 8; i++) {
    set_dma_addr(i, 0x000000);
// DMA count is a bit weird so this is not 0
    set_dma_count(i, 1);
    }
// Enable cascade DMA or channel 0-3 won't work
    enable_dma(4);
    release_dma_lock(flags);
    }
    static const struct syscore_ops i8237_syscore_ops = {
    .resume		= i8237A_resume,
    };
    static struct syscore i8237_syscore = {
    .ops = &i8237_syscore_ops,
    };
#[no_mangle]
unsafe extern "C" fn i8237A_init_ops() -> int __init {
    static int __init i8237A_init_ops(void)
    {
//
// From SKL PCH onwards, the legacy DMA device is removed in which the
// I/O ports (81h-83h, 87h, 89h-8Bh, 8Fh) related to it are removed
// as well. All removed ports must return 0xff for a inb() request.
//
// Note: DMA_PAGE_2 (port 0x81) should not be checked for detecting
// the presence of DMA device since it may be used by BIOS to decode
// LPC traffic for POST codes. Original LPC only decodes one byte of
// port 0x80 but some BIOS may choose to enhance PCH LPC port 0x8x
// decoding.
//
    if (dma_inb(DMA_PAGE_0) == 0xFF)
    return -ENODEV;
//
// It is not required to load this driver as newer SoC may not
// support 8237 DMA or bus mastering from LPC. Platform firmware
// must announce the support for such legacy devices via
// ACPI_FADT_LEGACY_DEVICES field in FADT table.
//
    if (x86_pnpbios_disabled() && dmi_get_bios_year() >= 2017)
    return -ENODEV;
    register_syscore(&i8237_syscore);
    return 0;
    }
    device_initcall(i8237A_init_ops);
