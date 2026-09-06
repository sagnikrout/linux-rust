//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/bootflag.c
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
// Implement 'Simple Boot Flag Specification 2.0'
//

    int sbf_port __initdata = -1;	/* set via acpi_boot_init() */
#[no_mangle]
unsafe extern "C" fn sbf_write(v: u8) -> void __init {
    static void __init sbf_write(u8 v)
    {
    unsigned long flags;
    if (sbf_port != -1) {
    if (!parity8(v))
    v ^= SBF_PARITY;
    printk(KERN_INFO "Simple Boot Flag at 0x%x set to 0x%x\n",
    sbf_port, v);
    spin_lock_irqsave(&rtc_lock, flags);
    CMOS_WRITE(v, sbf_port);
    spin_unlock_irqrestore(&rtc_lock, flags);
    }
    }
#[no_mangle]
unsafe extern "C" fn sbf_read() -> u8 __init {
    static u8 __init sbf_read(void)
    {
    unsigned long flags;
    u8 v;
    if (sbf_port == -1)
    return 0;
    spin_lock_irqsave(&rtc_lock, flags);
    v = CMOS_READ(sbf_port);
    spin_unlock_irqrestore(&rtc_lock, flags);
    return v;
    }
#[no_mangle]
unsafe extern "C" fn sbf_value_valid(v: u8) -> bool __init {
    static bool __init sbf_value_valid(u8 v)
    {
    if (v & SBF_RESERVED)		/* Reserved bits */
    return false;
    if (!parity8(v))
    return false;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn sbf_init() -> int __init {
    static int __init sbf_init(void)
    {
    u8 v;
    if (sbf_port == -1)
    return 0;
    v = sbf_read();
    if (!sbf_value_valid(v)) {
    printk(KERN_WARNING "Simple Boot Flag value 0x%x read from "
    "CMOS RAM was invalid\n", v);
    }
    v &= ~SBF_RESERVED;
    v &= ~SBF_BOOTING;
    v &= ~SBF_DIAG;

    v |= SBF_PNPOS;

    sbf_write(v);
    return 0;
    }
    arch_initcall(sbf_init);
