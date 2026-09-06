//! Automatically rewritten from C to Rust
//! Source: drivers/input/serio/ioc3kbd.c
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
// SGI IOC3 PS/2 controller driver for linux
//
// Copyright (C) 2019 Thomas Bogendoerfer <tbogendoerfer@suse.de>
//
// Based on code Copyright (C) 2005 Stanislaw Skowronek <skylark@unaligned.org>
// Copyright (C) 2009 Johannes Dickgreber <tanzy@gmx.de>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioc3kbd_data {
    pub regs: *mut ioc3_serioregs __iomem,
    pub aux: *mut *mut serio kbd,,
    pub aux_exists: bool kbd_exists,,
    pub irq: c_int,
}

#[no_mangle]
unsafe extern "C" fn ioc3kbd_wait(regs: *mut ioc3_serioregs __iomem, mask: u32) -> c_int {
    static int ioc3kbd_wait(struct ioc3_serioregs __iomem *regs, u32 mask)
    {
    let mut timeout: c_ulong = 0;
    while ((readl(&regs.km_csr) & mask) && (timeout < 250)) {
    udelay(50);
    timeout++;
    }
    return (timeout >= 250) ? -ETIMEDOUT : 0;
    }
#[no_mangle]
unsafe extern "C" fn ioc3kbd_write(dev: *mut serio, val: u8) -> c_int {
    static int ioc3kbd_write(struct serio *dev, u8 val)
    {
    struct ioc3kbd_data *d = dev.port_data;
    int ret;
    ret = ioc3kbd_wait(d.regs, KM_CSR_K_WRT_PEND);
    if (ret)
    return ret;
    writel(val, &d.regs.k_wd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ioc3kbd_start(dev: *mut serio) -> c_int {
    static int ioc3kbd_start(struct serio *dev)
    {
    struct ioc3kbd_data *d = dev.port_data;
    d.kbd_exists = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ioc3kbd_stop(dev: *mut serio) {
    static void ioc3kbd_stop(struct serio *dev)
    {
    struct ioc3kbd_data *d = dev.port_data;
    d.kbd_exists = false;
    }
#[no_mangle]
unsafe extern "C" fn ioc3aux_write(dev: *mut serio, val: u8) -> c_int {
    static int ioc3aux_write(struct serio *dev, u8 val)
    {
    struct ioc3kbd_data *d = dev.port_data;
    int ret;
    ret = ioc3kbd_wait(d.regs, KM_CSR_M_WRT_PEND);
    if (ret)
    return ret;
    writel(val, &d.regs.m_wd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ioc3aux_start(dev: *mut serio) -> c_int {
    static int ioc3aux_start(struct serio *dev)
    {
    struct ioc3kbd_data *d = dev.port_data;
    d.aux_exists = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ioc3aux_stop(dev: *mut serio) {
    static void ioc3aux_stop(struct serio *dev)
    {
    struct ioc3kbd_data *d = dev.port_data;
    d.aux_exists = false;
    }
#[no_mangle]
unsafe extern "C" fn ioc3kbd_process_data(dev: *mut serio, data: u32) {
    static void ioc3kbd_process_data(struct serio *dev, u32 data)
    {
    if (data & KM_RD_VALID_0)
    serio_interrupt(dev, (data >> KM_RD_DATA_0_SHIFT) & 0xff, 0);
    if (data & KM_RD_VALID_1)
    serio_interrupt(dev, (data >> KM_RD_DATA_1_SHIFT) & 0xff, 0);
    if (data & KM_RD_VALID_2)
    serio_interrupt(dev, (data >> KM_RD_DATA_2_SHIFT) & 0xff, 0);
    }
#[no_mangle]
unsafe extern "C" fn ioc3kbd_intr(itq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t ioc3kbd_intr(int itq, void *dev_id)
    {
    struct ioc3kbd_data *d = dev_id;
    u32 data_k, data_m;
    data_k = readl(&d.regs.k_rd);
    if (d.kbd_exists)
    ioc3kbd_process_data(d.kbd, data_k);
    data_m = readl(&d.regs.m_rd);
    if (d.aux_exists)
    ioc3kbd_process_data(d.aux, data_m);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn ioc3kbd_probe(pdev: *mut platform_device) -> c_int {
    static int ioc3kbd_probe(struct platform_device *pdev)
    {
    struct ioc3_serioregs __iomem *regs;
    struct device *dev = &pdev.dev;
    struct ioc3kbd_data *d;
    struct serio *sk, *sa;
    int irq, ret;
    regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(regs))
    return PTR_ERR(regs);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return -ENXIO;
    d = devm_kzalloc(dev, sizeof(*d), GFP_KERNEL);
    if (!d)
    return -ENOMEM;
    sk = kzalloc_obj(*sk);
    if (!sk)
    return -ENOMEM;
    sa = kzalloc_obj(*sa);
    if (!sa) {
    kfree(sk);
    return -ENOMEM;
    }
    sk.id.type = SERIO_8042;
    sk.write = ioc3kbd_write;
    sk.start = ioc3kbd_start;
    sk.stop = ioc3kbd_stop;
    snprintf(sk.name, sizeof(sk.name), "IOC3 keyboard %d", pdev.id);
    snprintf(sk.phys, sizeof(sk.phys), "ioc3/serio%dkbd", pdev.id);
    sk.port_data = d;
    sk.dev.parent = dev;
    sa.id.type = SERIO_8042;
    sa.write = ioc3aux_write;
    sa.start = ioc3aux_start;
    sa.stop = ioc3aux_stop;
    snprintf(sa.name, sizeof(sa.name), "IOC3 auxiliary %d", pdev.id);
    snprintf(sa.phys, sizeof(sa.phys), "ioc3/serio%daux", pdev.id);
    sa.port_data = d;
    sa.dev.parent = dev;
    d.regs = regs;
    d.kbd = sk;
    d.aux = sa;
    d.irq = irq;
    platform_set_drvdata(pdev, d);
    serio_register_port(d.kbd);
    serio_register_port(d.aux);
    ret = request_irq(irq, ioc3kbd_intr, IRQF_SHARED, "ioc3-kbd", d);
    if (ret) {
    dev_err(dev, "could not request IRQ %d\n", irq);
    serio_unregister_port(d.kbd);
    serio_unregister_port(d.aux);
    return ret;
    }
// enable ports
    writel(KM_CSR_K_CLAMP_3 | KM_CSR_M_CLAMP_3, &regs.km_csr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ioc3kbd_remove(pdev: *mut platform_device) {
    static void ioc3kbd_remove(struct platform_device *pdev)
    {
    struct ioc3kbd_data *d = platform_get_drvdata(pdev);
    free_irq(d.irq, d);
    serio_unregister_port(d.kbd);
    serio_unregister_port(d.aux);
    }
    static const struct platform_device_id ioc3kbd_id_table[] = {
    { "ioc3-kbd", },
    { }
    };
    MODULE_DEVICE_TABLE(platform, ioc3kbd_id_table);
    static struct platform_driver ioc3kbd_driver = {
    .probe          = ioc3kbd_probe,
    .remove         = ioc3kbd_remove,
    .id_table	= ioc3kbd_id_table,
    .driver = {
    .name = "ioc3-kbd",
    },
    };
    module_platform_driver(ioc3kbd_driver);
    MODULE_AUTHOR("Thomas Bogendoerfer <tbogendoerfer@suse.de>");
    MODULE_DESCRIPTION("SGI IOC3 serio driver");
    MODULE_LICENSE("GPL");
