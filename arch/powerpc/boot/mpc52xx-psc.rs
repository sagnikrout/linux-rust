//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/boot/mpc52xx-psc.c
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
// MPC5200 PSC serial console support.
//
// Author: Grant Likely <grant.likely@secretlab.ca>
//
// Copyright (c) 2007 Secret Lab Technologies Ltd.
// Copyright (c) 2007 Freescale Semiconductor, Inc.
//
// It is assumed that the firmware (or the platform file) has already set
// up the port.
//

// Programmable Serial Controller (PSC) status register bits
pub const MPC52xx_PSC_SR: c_uint = 0x04;
pub const MPC52xx_PSC_SR_RXRDY: c_uint = 0x0100;
pub const MPC52xx_PSC_SR_RXFULL: c_uint = 0x0200;
pub const MPC52xx_PSC_SR_TXRDY: c_uint = 0x0400;
pub const MPC52xx_PSC_SR_TXEMP: c_uint = 0x0800;
pub const MPC52xx_PSC_BUFFER: c_uint = 0x0C;
    static void *psc;
#[no_mangle]
unsafe extern "C" fn psc_open() -> c_int {
    static int psc_open(void)
    {
// Assume the firmware has already configured the PSC into
// uart mode
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn psc_putc(c: c_uchar) {
    static void psc_putc(unsigned char c)
    {
    while (!(in_be16(psc + MPC52xx_PSC_SR) & MPC52xx_PSC_SR_TXRDY)) ;
    out_8(psc + MPC52xx_PSC_BUFFER, c);
    }
#[no_mangle]
unsafe extern "C" fn psc_tstc() -> c_uchar {
    static unsigned char psc_tstc(void)
    {
    return (in_be16(psc + MPC52xx_PSC_SR) & MPC52xx_PSC_SR_RXRDY) != 0;
    }
#[no_mangle]
unsafe extern "C" fn psc_getc() -> c_uchar {
    static unsigned char psc_getc(void)
    {
    while (!(in_be16(psc + MPC52xx_PSC_SR) & MPC52xx_PSC_SR_RXRDY)) ;
    return in_8(psc + MPC52xx_PSC_BUFFER);
    }
#[no_mangle]
pub unsafe extern "C" fn mpc5200_psc_console_init(devp: *mut c_void, scdp: *mut serial_console_data) -> c_int {
    int mpc5200_psc_console_init(void *devp, struct serial_console_data *scdp)
    {
// Get the base address of the psc registers
    if (dt_get_virtual_reg(devp, &psc, 1) < 1)
    return -1;
    scdp.open = psc_open;
    scdp.putc = psc_putc;
    scdp.getc = psc_getc;
    scdp.tstc = psc_tstc;
    return 0;
    }
