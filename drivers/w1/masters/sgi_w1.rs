//! Automatically rewritten from C to Rust
//! Source: drivers/w1/masters/sgi_w1.c
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
// sgi_w1.c - w1 master driver for one wire support in SGI ASICs
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgi_w1_device {
    pub mcr: *mut u32 __iomem,
    pub bus_master: w1_bus_master,
    pub dev_id: [c_char; 64],
}

#[no_mangle]
unsafe extern "C" fn sgi_w1_wait(mcr: *mut u32 __iomem) -> u8 {
    static u8 sgi_w1_wait(u32 __iomem *mcr)
    {
    u32 mcr_val;
    do {
    mcr_val = readl(mcr);
    } while (!(mcr_val & MCR_DONE));
    return (mcr_val & MCR_RD_DATA) ? 1 : 0;
    }
//
// this is the low level routine to
// reset the device on the One Wire interface
// on the hardware
//
#[no_mangle]
unsafe extern "C" fn sgi_w1_reset_bus(data: *mut c_void) -> u8 {
    static u8 sgi_w1_reset_bus(void *data)
    {
    struct sgi_w1_device *dev = data;
    u8 ret;
    writel(MCR_PACK(520, 65), dev.mcr);
    ret = sgi_w1_wait(dev.mcr);
    udelay(500); /* recovery time */
    return ret;
    }
//
// this is the low level routine to read/write a bit on the One Wire
// interface on the hardware. It does write 0 if parameter bit is set
// to 0, otherwise a write 1/read.
//
#[no_mangle]
unsafe extern "C" fn sgi_w1_touch_bit(data: *mut c_void, bit: u8) -> u8 {
    static u8 sgi_w1_touch_bit(void *data, u8 bit)
    {
    struct sgi_w1_device *dev = data;
    u8 ret;
    if (bit)
    writel(MCR_PACK(6, 13), dev.mcr);
    else
    writel(MCR_PACK(80, 30), dev.mcr);
    ret = sgi_w1_wait(dev.mcr);
    if (bit)
    udelay(100); /* recovery */
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sgi_w1_probe(pdev: *mut platform_device) -> c_int {
    static int sgi_w1_probe(struct platform_device *pdev)
    {
    struct sgi_w1_device *sdev;
    struct sgi_w1_platform_data *pdata;
    sdev = devm_kzalloc(&pdev.dev, sizeof(struct sgi_w1_device),
    GFP_KERNEL);
    if (!sdev)
    return -ENOMEM;
    sdev.mcr = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(sdev.mcr))
    return PTR_ERR(sdev.mcr);
    sdev.bus_master.data = sdev;
    sdev.bus_master.reset_bus = sgi_w1_reset_bus;
    sdev.bus_master.touch_bit = sgi_w1_touch_bit;
    pdata = dev_get_platdata(&pdev.dev);
    if (pdata) {
    strscpy(sdev.dev_id, pdata.dev_id, sizeof(sdev.dev_id));
    sdev.bus_master.dev_id = sdev.dev_id;
    }
    platform_set_drvdata(pdev, sdev);
    return w1_add_master_device(&sdev.bus_master);
    }
//
// disassociate the w1 device from the driver
//
#[no_mangle]
unsafe extern "C" fn sgi_w1_remove(pdev: *mut platform_device) {
    static void sgi_w1_remove(struct platform_device *pdev)
    {
    struct sgi_w1_device *sdev = platform_get_drvdata(pdev);
    w1_remove_master_device(&sdev.bus_master);
    }
    static struct platform_driver sgi_w1_driver = {
    .driver = {
    .name = "sgi_w1",
    },
    .probe = sgi_w1_probe,
    .remove = sgi_w1_remove,
    };
    module_platform_driver(sgi_w1_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Thomas Bogendoerfer");
    MODULE_DESCRIPTION("Driver for One-Wire IP in SGI ASICs");
