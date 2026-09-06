//! Automatically rewritten from C to Rust
//! Source: drivers/input/serio/sa1111ps2.c
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
// linux/drivers/input/serio/sa1111ps2.c
//
// Copyright (C) 2002 Russell King
//

pub const PS2CR: c_uint = 0x0000;
pub const PS2STAT: c_uint = 0x0004;
pub const PS2DATA: c_uint = 0x0008;
pub const PS2CLKDIV: c_uint = 0x000c;
pub const PS2PRECNT: c_uint = 0x0010;
pub const PS2CR_ENA: c_uint = 0x08;
pub const PS2CR_FKD: c_uint = 0x02;
pub const PS2CR_FKC: c_uint = 0x01;
pub const PS2STAT_STP: c_uint = 0x0100;
pub const PS2STAT_TXE: c_uint = 0x0080;
pub const PS2STAT_TXB: c_uint = 0x0040;
pub const PS2STAT_RXF: c_uint = 0x0020;
pub const PS2STAT_RXB: c_uint = 0x0010;
pub const PS2STAT_ENA: c_uint = 0x0008;
pub const PS2STAT_RXP: c_uint = 0x0004;
pub const PS2STAT_KBD: c_uint = 0x0002;
pub const PS2STAT_KBC: c_uint = 0x0001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps2if {
    pub io: *mut serio,
    pub dev: *mut sa1111_dev,
    pub base: *mut void __iomem,
    pub rx_irq: c_int,
    pub tx_irq: c_int,
    pub open: c_uint,
    pub lock: spinlock_t,
    pub head: c_uint,
    pub tail: c_uint,
    pub buf: [c_uchar; 4],
}

//
// Read all bytes waiting in the PS2 port.  There should be
// at the most one, but we loop for safety.  If there was a
// framing error, we have to manually clear the status.
//
#[no_mangle]
unsafe extern "C" fn ps2_rxint(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t ps2_rxint(int irq, void *dev_id)
    {
    struct ps2if *ps2if = dev_id;
    unsigned int scancode, flag, status;
    status = readl_relaxed(ps2if.base + PS2STAT);
    while (status & PS2STAT_RXF) {
    if (status & PS2STAT_STP)
    writel_relaxed(PS2STAT_STP, ps2if.base + PS2STAT);
    flag = (status & PS2STAT_STP ? SERIO_FRAME : 0) |
    (status & PS2STAT_RXP ? 0 : SERIO_PARITY);
    scancode = readl_relaxed(ps2if.base + PS2DATA) & 0xff;
    if (hweight8(scancode) & 1)
    flag ^= SERIO_PARITY;
    serio_interrupt(ps2if.io, scancode, flag);
    status = readl_relaxed(ps2if.base + PS2STAT);
    }
    return IRQ_HANDLED;
    }
//
// Completion of ps2 write
//
#[no_mangle]
unsafe extern "C" fn ps2_txint(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t ps2_txint(int irq, void *dev_id)
    {
    struct ps2if *ps2if = dev_id;
    unsigned int status;
    guard(spinlock)(&ps2if.lock);
    status = readl_relaxed(ps2if.base + PS2STAT);
    if (ps2if.head == ps2if.tail) {
    disable_irq_nosync(irq);
// done
    } else if (status & PS2STAT_TXE) {
    writel_relaxed(ps2if.buf[ps2if.tail], ps2if.base + PS2DATA);
    ps2if.tail = (ps2if.tail + 1) & (sizeof(ps2if.buf) - 1);
    }
    return IRQ_HANDLED;
    }
//
// Write a byte to the PS2 port.  We have to wait for the
// port to indicate that the transmitter is empty.
//
#[no_mangle]
unsafe extern "C" fn ps2_write(io: *mut serio, val: c_uchar) -> c_int {
    static int ps2_write(struct serio *io, unsigned char val)
    {
    struct ps2if *ps2if = io.port_data;
    unsigned int head;
    guard(spinlock_irqsave)(&ps2if.lock);
//
// If the TX register is empty, we can go straight out.
//
    if (readl_relaxed(ps2if.base + PS2STAT) & PS2STAT_TXE) {
    writel_relaxed(val, ps2if.base + PS2DATA);
    } else {
    if (ps2if.head == ps2if.tail)
    enable_irq(ps2if.tx_irq);
    head = (ps2if.head + 1) & (sizeof(ps2if.buf) - 1);
    if (head != ps2if.tail) {
    ps2if.buf[ps2if.head] = val;
    ps2if.head = head;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ps2_open(io: *mut serio) -> c_int {
    static int ps2_open(struct serio *io)
    {
    struct ps2if *ps2if = io.port_data;
    int ret;
    ret = sa1111_enable_device(ps2if.dev);
    if (ret)
    return ret;
    ret = request_irq(ps2if.rx_irq, ps2_rxint, 0,
    SA1111_DRIVER_NAME(ps2if.dev), ps2if);
    if (ret) {
    printk(KERN_ERR "sa1111ps2: could not allocate IRQ%d: %d\n",
    ps2if.rx_irq, ret);
    sa1111_disable_device(ps2if.dev);
    return ret;
    }
    ret = request_irq(ps2if.tx_irq, ps2_txint, 0,
    SA1111_DRIVER_NAME(ps2if.dev), ps2if);
    if (ret) {
    printk(KERN_ERR "sa1111ps2: could not allocate IRQ%d: %d\n",
    ps2if.tx_irq, ret);
    free_irq(ps2if.rx_irq, ps2if);
    sa1111_disable_device(ps2if.dev);
    return ret;
    }
    ps2if.open = 1;
    enable_irq_wake(ps2if.rx_irq);
    writel_relaxed(PS2CR_ENA, ps2if.base + PS2CR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ps2_close(io: *mut serio) {
    static void ps2_close(struct serio *io)
    {
    struct ps2if *ps2if = io.port_data;
    writel_relaxed(0, ps2if.base + PS2CR);
    disable_irq_wake(ps2if.rx_irq);
    ps2if.open = 0;
    free_irq(ps2if.tx_irq, ps2if);
    free_irq(ps2if.rx_irq, ps2if);
    sa1111_disable_device(ps2if.dev);
    }
//
// Clear the input buffer.
//
#[no_mangle]
unsafe extern "C" fn ps2_clear_input(ps2if: *mut ps2if) {
    static void ps2_clear_input(struct ps2if *ps2if)
    {
    let mut maxread: c_int = 100;
    while (maxread--) {
    if ((readl_relaxed(ps2if.base + PS2DATA) & 0xff) == 0xff)
    break;
    }
    }
    static unsigned int ps2_test_one(struct ps2if *ps2if,
    unsigned int mask)
    {
    unsigned int val;
    writel_relaxed(PS2CR_ENA | mask, ps2if.base + PS2CR);
    udelay(10);
    val = readl_relaxed(ps2if.base + PS2STAT);
    return val & (PS2STAT_KBC | PS2STAT_KBD);
    }
//
// Test the keyboard interface.  We basically check to make sure that
// we can drive each line to the keyboard independently of each other.
//
#[no_mangle]
unsafe extern "C" fn ps2_test(ps2if: *mut ps2if) -> c_int {
    static int ps2_test(struct ps2if *ps2if)
    {
    unsigned int stat;
    let mut ret: c_int = 0;
    stat = ps2_test_one(ps2if, PS2CR_FKC);
    if (stat != PS2STAT_KBD) {
    printk("PS/2 interface test failed[1]: %02x\n", stat);
    ret = -ENODEV;
    }
    stat = ps2_test_one(ps2if, 0);
    if (stat != (PS2STAT_KBC | PS2STAT_KBD)) {
    printk("PS/2 interface test failed[2]: %02x\n", stat);
    ret = -ENODEV;
    }
    stat = ps2_test_one(ps2if, PS2CR_FKD);
    if (stat != PS2STAT_KBC) {
    printk("PS/2 interface test failed[3]: %02x\n", stat);
    ret = -ENODEV;
    }
    writel_relaxed(0, ps2if.base + PS2CR);
    return ret;
    }
//
// Add one device to this driver.
//
#[no_mangle]
unsafe extern "C" fn ps2_probe(dev: *mut sa1111_dev) -> c_int {
    static int ps2_probe(struct sa1111_dev *dev)
    {
    struct ps2if *ps2if;
    struct serio *serio;
    int ret;
    ps2if = kzalloc_obj(*ps2if);
    serio = kzalloc_obj(*serio);
    if (!ps2if || !serio) {
    ret = -ENOMEM;
    goto free;
    }
    serio.id.type		= SERIO_8042;
    serio.write		= ps2_write;
    serio.open		= ps2_open;
    serio.close		= ps2_close;
    strscpy(serio.name, dev_name(&dev.dev), sizeof(serio.name));
    strscpy(serio.phys, dev_name(&dev.dev), sizeof(serio.phys));
    serio.port_data	= ps2if;
    serio.dev.parent	= &dev.dev;
    ps2if.io		= serio;
    ps2if.dev		= dev;
    sa1111_set_drvdata(dev, ps2if);
    spin_lock_init(&ps2if.lock);
    ps2if.rx_irq = sa1111_get_irq(dev, 0);
    if (ps2if.rx_irq <= 0) {
    ret = ps2if.rx_irq ? : -ENXIO;
    goto free;
    }
    ps2if.tx_irq = sa1111_get_irq(dev, 1);
    if (ps2if.tx_irq <= 0) {
    ret = ps2if.tx_irq ? : -ENXIO;
    goto free;
    }
//
// Request the physical region for this PS2 port.
//
    if (!request_mem_region(dev.res.start,
    dev.res.end - dev.res.start + 1,
    SA1111_DRIVER_NAME(dev))) {
    ret = -EBUSY;
    goto free;
    }
//
// Our parent device has already mapped the region.
//
    ps2if.base = dev.mapbase;
    sa1111_enable_device(ps2if.dev);
// Incoming clock is 8MHz
    writel_relaxed(0, ps2if.base + PS2CLKDIV);
    writel_relaxed(127, ps2if.base + PS2PRECNT);
//
// Flush any pending input.
//
    ps2_clear_input(ps2if);
//
// Test the keyboard interface.
//
    ret = ps2_test(ps2if);
    if (ret)
    goto out;
//
// Flush any pending input.
//
    ps2_clear_input(ps2if);
    sa1111_disable_device(ps2if.dev);
    serio_register_port(ps2if.io);
    return 0;
    out:
    sa1111_disable_device(ps2if.dev);
    release_mem_region(dev.res.start, resource_size(&dev.res));
    free:
    sa1111_set_drvdata(dev, core::ptr::null_mut());
    kfree(ps2if);
    kfree(serio);
    return ret;
    }
//
// Remove one device from this driver.
//
#[no_mangle]
unsafe extern "C" fn ps2_remove(dev: *mut sa1111_dev) {
    static void ps2_remove(struct sa1111_dev *dev)
    {
    struct ps2if *ps2if = sa1111_get_drvdata(dev);
    serio_unregister_port(ps2if.io);
    release_mem_region(dev.res.start, resource_size(&dev.res));
    sa1111_set_drvdata(dev, core::ptr::null_mut());
    kfree(ps2if);
    }
//
// Our device driver structure
//
    static struct sa1111_driver ps2_driver = {
    .drv = {
    .name	= "sa1111-ps2",
    .owner	= THIS_MODULE,
    },
    .devid		= SA1111_DEVID_PS2,
    .probe		= ps2_probe,
    .remove		= ps2_remove,
    };
#[no_mangle]
unsafe extern "C" fn ps2_init() -> int __init {
    static int __init ps2_init(void)
    {
    return sa1111_driver_register(&ps2_driver);
    }
#[no_mangle]
unsafe extern "C" fn ps2_exit() -> void __exit {
    static void __exit ps2_exit(void)
    {
    sa1111_driver_unregister(&ps2_driver);
    }
    module_init(ps2_init);
    module_exit(ps2_exit);
    MODULE_AUTHOR("Russell King <rmk@arm.linux.org.uk>");
    MODULE_DESCRIPTION("SA1111 PS2 controller driver");
    MODULE_LICENSE("GPL");
