//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/boot/ns16550.c
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
// 16550 serial console support.
//
// Original copied from <file:arch/ppc/boot/common/ns16550.c>
// (which had no copyright)
// Modifications: 2006 (c) MontaVista Software, Inc.
//
// Modified by: Mark A. Greer <mgreer@mvista.com>
//

pub const UART_LSR_THRE: c_uint = 0x20	/* Transmit-hold-register empty */;
pub const UART_LSR_DR: c_uint = 0x01	/* Receiver data ready */;

    static unsigned char *reg_base;
    static u32 reg_shift;
#[no_mangle]
unsafe extern "C" fn ns16550_open() -> c_int {
    static int ns16550_open(void)
    {
    out_8(reg_base + (UART_FCR << reg_shift), 0x06);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ns16550_putc(c: c_uchar) {
    static void ns16550_putc(unsigned char c)
    {
    while ((in_8(reg_base + (UART_LSR << reg_shift)) & UART_LSR_THRE) == 0);
    out_8(reg_base, c);
    }
#[no_mangle]
unsafe extern "C" fn ns16550_getc() -> c_uchar {
    static unsigned char ns16550_getc(void)
    {
    while ((in_8(reg_base + (UART_LSR << reg_shift)) & UART_LSR_DR) == 0);
    return in_8(reg_base);
    }
#[no_mangle]
unsafe extern "C" fn ns16550_tstc() -> u8 {
    static u8 ns16550_tstc(void)
    {
    return ((in_8(reg_base + (UART_LSR << reg_shift)) & UART_LSR_DR) != 0);
    }
#[no_mangle]
pub unsafe extern "C" fn ns16550_console_init(devp: *mut c_void, scdp: *mut serial_console_data) -> c_int {
    int ns16550_console_init(void *devp, struct serial_console_data *scdp)
    {
    int n;
    u32 reg_offset;
    if (dt_get_virtual_reg(devp, (void **)&reg_base, 1) < 1) {
    printf("virt reg parse fail...\r\n");
    return -1;
    }
    n = getprop(devp, "reg-offset", &reg_offset, sizeof(reg_offset));
    if (n == sizeof(reg_offset))
    reg_base += be32_to_cpu(reg_offset);
    n = getprop(devp, "reg-shift", &reg_shift, sizeof(reg_shift));
    if (n != sizeof(reg_shift))
    reg_shift = 0;
    else
    reg_shift = be32_to_cpu(reg_shift);
    scdp.open = ns16550_open;
    scdp.putc = ns16550_putc;
    scdp.getc = ns16550_getc;
    scdp.tstc = ns16550_tstc;
    scdp.close = core::ptr::null_mut();
    return 0;
    }
