//! Automatically rewritten from C to Rust
//! Source: drivers/input/serio/ambakmi.c
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
// linux/drivers/input/serio/ambakmi.c
//
// Copyright (C) 2000-2003 Deep Blue Solutions Ltd.
// Copyright (C) 2002 Russell King.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amba_kmi_port {
    pub io: *mut serio,
    pub clk: *mut clk,
    pub base: *mut void __iomem,
    pub irq: c_uint,
    pub divisor: c_uint,
    pub open: c_uint,
}

#[no_mangle]
unsafe extern "C" fn amba_kmi_int(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t amba_kmi_int(int irq, void *dev_id)
    {
    struct amba_kmi_port *kmi = dev_id;
    let mut status: c_uint = readb(KMIIR);
    let mut handled: c_int = IRQ_NONE;
    while (status & KMIIR_RXINTR) {
    serio_interrupt(kmi.io, readb(KMIDATA), 0);
    status = readb(KMIIR);
    handled = IRQ_HANDLED;
    }
    return handled;
    }
#[no_mangle]
unsafe extern "C" fn amba_kmi_write(io: *mut serio, val: c_uchar) -> c_int {
    static int amba_kmi_write(struct serio *io, unsigned char val)
    {
    struct amba_kmi_port *kmi = io.port_data;
    unsigned int timeleft = 10000; /* timeout in 100ms */
    while ((readb(KMISTAT) & KMISTAT_TXEMPTY) == 0 && --timeleft)
    udelay(10);
    if (timeleft)
    writeb(val, KMIDATA);
    return timeleft ? 0 : SERIO_TIMEOUT;
    }
#[no_mangle]
unsafe extern "C" fn amba_kmi_open(io: *mut serio) -> c_int {
    static int amba_kmi_open(struct serio *io)
    {
    struct amba_kmi_port *kmi = io.port_data;
    unsigned int divisor;
    int ret;
    ret = clk_prepare_enable(kmi.clk);
    if (ret)
    goto out;
    divisor = clk_get_rate(kmi.clk) / 8000000 - 1;
    writeb(divisor, KMICLKDIV);
    writeb(KMICR_EN, KMICR);
    ret = request_irq(kmi.irq, amba_kmi_int, IRQF_SHARED, "kmi-pl050",
    kmi);
    if (ret) {
    printk(KERN_ERR "kmi: failed to claim IRQ%d\n", kmi.irq);
    writeb(0, KMICR);
    goto clk_disable;
    }
    writeb(KMICR_EN | KMICR_RXINTREN, KMICR);
    return 0;
    clk_disable:
    clk_disable_unprepare(kmi.clk);
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn amba_kmi_close(io: *mut serio) {
    static void amba_kmi_close(struct serio *io)
    {
    struct amba_kmi_port *kmi = io.port_data;
    writeb(0, KMICR);
    free_irq(kmi.irq, kmi);
    clk_disable_unprepare(kmi.clk);
    }
    static int amba_kmi_probe(struct amba_device *dev,
    const struct amba_id *id)
    {
    struct amba_kmi_port *kmi;
    struct serio *io;
    int ret;
    ret = amba_request_regions(dev, core::ptr::null_mut());
    if (ret)
    return ret;
    kmi = kzalloc_obj(*kmi);
    io = kzalloc_obj(*io);
    if (!kmi || !io) {
    ret = -ENOMEM;
    goto out;
    }
    io.id.type	= SERIO_8042;
    io.write	= amba_kmi_write;
    io.open	= amba_kmi_open;
    io.close	= amba_kmi_close;
    strscpy(io.name, dev_name(&dev.dev), sizeof(io.name));
    strscpy(io.phys, dev_name(&dev.dev), sizeof(io.phys));
    io.port_data	= kmi;
    io.dev.parent	= &dev.dev;
    kmi.io		= io;
    kmi.base	= ioremap(dev.res.start, resource_size(&dev.res));
    if (!kmi.base) {
    ret = -ENOMEM;
    goto out;
    }
    kmi.clk = clk_get(&dev.dev, "KMIREFCLK");
    if (IS_ERR(kmi.clk)) {
    ret = PTR_ERR(kmi.clk);
    goto unmap;
    }
    kmi.irq = dev.irq[0];
    amba_set_drvdata(dev, kmi);
    serio_register_port(kmi.io);
    return 0;
    unmap:
    iounmap(kmi.base);
    out:
    kfree(kmi);
    kfree(io);
    amba_release_regions(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn amba_kmi_remove(dev: *mut amba_device) {
    static void amba_kmi_remove(struct amba_device *dev)
    {
    struct amba_kmi_port *kmi = amba_get_drvdata(dev);
    serio_unregister_port(kmi.io);
    clk_put(kmi.clk);
    iounmap(kmi.base);
    kfree(kmi);
    amba_release_regions(dev);
    }
#[no_mangle]
unsafe extern "C" fn amba_kmi_resume(dev: *mut device) -> c_int {
    static int amba_kmi_resume(struct device *dev)
    {
    struct amba_kmi_port *kmi = dev_get_drvdata(dev);
// kick the serio layer to rescan this port
    serio_reconnect(kmi.io);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(amba_kmi_dev_pm_ops, core::ptr::null_mut(), amba_kmi_resume);
    static const struct amba_id amba_kmi_idtable[] = {
    {
    .id	= 0x00041050,
    .mask	= 0x000fffff,
    },
    { 0, 0 }
    };
    MODULE_DEVICE_TABLE(amba, amba_kmi_idtable);
    static struct amba_driver ambakmi_driver = {
    .drv		= {
    .name	= "kmi-pl050",
    .pm	= pm_sleep_ptr(&amba_kmi_dev_pm_ops),
    },
    .id_table	= amba_kmi_idtable,
    .probe		= amba_kmi_probe,
    .remove		= amba_kmi_remove,
    };
    module_amba_driver(ambakmi_driver);
    MODULE_AUTHOR("Russell King <rmk@arm.linux.org.uk>");
    MODULE_DESCRIPTION("AMBA KMI controller driver");
    MODULE_LICENSE("GPL");
