//! Automatically rewritten from C to Rust
//! Source: drivers/auxdisplay/ks0108.c
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
// Filename: ks0108.c
// Version: 0.1.0
// Description: ks0108 LCD Controller driver
// Depends: parport
//
// Author: Copyright (C) Miguel Ojeda <ojeda@kernel.org>
// Date: 2006-10-31
//

//
// Module Parameters
//
    let mut ks0108_port: static unsigned int = CONFIG_KS0108_PORT;
    module_param(ks0108_port, uint, 0444);
    MODULE_PARM_DESC(ks0108_port, "Parallel port where the LCD is connected");
    let mut ks0108_delay: static unsigned int = CONFIG_KS0108_DELAY;
    module_param(ks0108_delay, uint, 0444);
    MODULE_PARM_DESC(ks0108_delay, "Delay between each control writing (microseconds)");
//
// Device
//
    static struct parport *ks0108_parport;
    static struct pardevice *ks0108_pardevice;
//
// ks0108 Exported Commands (don't lock)
//
// You _should_ lock in the top driver: This functions _should not_
// get race conditions in any way. Locking for each byte here would be
// so slow and useless.
//
// There are not bit definitions because they are not flags,
// just arbitrary combinations defined by the documentation for each
// function in the ks0108 LCD controller. If you want to know what means
// a specific combination, look at the function's name.
//
// The ks0108_writecontrol bits need to be reverted ^(0,1,3) because
// the parallel port also revert them using a "not" logic gate.
//

#[no_mangle]
pub unsafe extern "C" fn ks0108_writedata(byte: c_uchar) {
    void ks0108_writedata(unsigned char byte)
    {
    parport_write_data(ks0108_parport, byte);
    }
#[no_mangle]
pub unsafe extern "C" fn ks0108_writecontrol(byte: c_uchar) {
    void ks0108_writecontrol(unsigned char byte)
    {
    udelay(ks0108_delay);
    parport_write_control(ks0108_parport, byte ^ (bit(0) | bit(1) | bit(3)));
    }
#[no_mangle]
pub unsafe extern "C" fn ks0108_displaystate(state: c_uchar) {
    void ks0108_displaystate(unsigned char state)
    {
    ks0108_writedata((state ? bit(0) : 0) | bit(1) | bit(2) | bit(3) | bit(4) | bit(5));
    }
#[no_mangle]
pub unsafe extern "C" fn ks0108_startline(startline: c_uchar) {
    void ks0108_startline(unsigned char startline)
    {
    ks0108_writedata(min_t(unsigned char, startline, 63) | bit(6) |
    bit(7));
    }
#[no_mangle]
pub unsafe extern "C" fn ks0108_address(address: c_uchar) {
    void ks0108_address(unsigned char address)
    {
    ks0108_writedata(min_t(unsigned char, address, 63) | bit(6));
    }
#[no_mangle]
pub unsafe extern "C" fn ks0108_page(page: c_uchar) {
    void ks0108_page(unsigned char page)
    {
    ks0108_writedata(min_t(unsigned char, page, 7) | bit(3) | bit(4) |
    bit(5) | bit(7));
    }
    EXPORT_SYMBOL_GPL(ks0108_writedata);
    EXPORT_SYMBOL_GPL(ks0108_writecontrol);
    EXPORT_SYMBOL_GPL(ks0108_displaystate);
    EXPORT_SYMBOL_GPL(ks0108_startline);
    EXPORT_SYMBOL_GPL(ks0108_address);
    EXPORT_SYMBOL_GPL(ks0108_page);
//
// Is the module inited?
//
    static unsigned char ks0108_inited;
#[no_mangle]
pub unsafe extern "C" fn ks0108_isinited() -> c_uchar {
    unsigned char ks0108_isinited(void)
    {
    return ks0108_inited;
    }
    EXPORT_SYMBOL_GPL(ks0108_isinited);
#[no_mangle]
unsafe extern "C" fn ks0108_parport_attach(port: *mut parport) {
    static void ks0108_parport_attach(struct parport *port)
    {
    struct pardev_cb ks0108_cb;
    if (port.base != ks0108_port)
    return;
    memset(&ks0108_cb, 0, sizeof(ks0108_cb));
    ks0108_cb.flags = PARPORT_DEV_EXCL;
    ks0108_pardevice = parport_register_dev_model(port, KS0108_NAME,
    &ks0108_cb, 0);
    if (!ks0108_pardevice) {
    pr_err("ERROR: parport didn't register new device\n");
    return;
    }
    if (parport_claim(ks0108_pardevice)) {
    pr_err("could not claim access to parport %i. Aborting.\n",
    ks0108_port);
    goto err_unreg_device;
    }
    ks0108_parport = port;
    ks0108_inited = 1;
    return;
    err_unreg_device:
    parport_unregister_device(ks0108_pardevice);
    ks0108_pardevice = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn ks0108_parport_detach(port: *mut parport) {
    static void ks0108_parport_detach(struct parport *port)
    {
    if (port.base != ks0108_port)
    return;
    if (!ks0108_pardevice) {
    pr_err("%s: already unregistered.\n", KS0108_NAME);
    return;
    }
    parport_release(ks0108_pardevice);
    parport_unregister_device(ks0108_pardevice);
    ks0108_pardevice = core::ptr::null_mut();
    ks0108_parport = core::ptr::null_mut();
    }
//
// Module Init & Exit
//
    static struct parport_driver ks0108_parport_driver = {
    .name = "ks0108",
    .match_port = ks0108_parport_attach,
    .detach = ks0108_parport_detach,
    };
    module_parport_driver(ks0108_parport_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Miguel Ojeda <ojeda@kernel.org>");
    MODULE_DESCRIPTION("ks0108 LCD Controller driver");
