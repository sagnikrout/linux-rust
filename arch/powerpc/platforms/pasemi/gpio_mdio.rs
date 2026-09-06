//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/pasemi/gpio_mdio.c
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
// Copyright (C) 2006-2007 PA Semi, Inc
//
// Author: Olof Johansson, PA Semi
//
// Maintained by: Olof Johansson <olof@lixom.net>
//
// Based on drivers/net/fs_enet/mii-bitbang.c.
//

pub const DELAY: c_int = 1;
    static void __iomem *gpio_regs;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_priv {
    pub mdc_pin: c_int,
    pub mdio_pin: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn mdio_lo(bus: *mut mii_bus) {
    static inline void mdio_lo(struct mii_bus *bus)
    {
    out_le32(gpio_regs+0x10, 1 << MDIO_PIN(bus));
    }
#[no_mangle]
pub unsafe extern "C" fn mdio_hi(bus: *mut mii_bus) {
    static inline void mdio_hi(struct mii_bus *bus)
    {
    out_le32(gpio_regs, 1 << MDIO_PIN(bus));
    }
#[no_mangle]
pub unsafe extern "C" fn mdc_lo(bus: *mut mii_bus) {
    static inline void mdc_lo(struct mii_bus *bus)
    {
    out_le32(gpio_regs+0x10, 1 << MDC_PIN(bus));
    }
#[no_mangle]
pub unsafe extern "C" fn mdc_hi(bus: *mut mii_bus) {
    static inline void mdc_hi(struct mii_bus *bus)
    {
    out_le32(gpio_regs, 1 << MDC_PIN(bus));
    }
#[no_mangle]
pub unsafe extern "C" fn mdio_active(bus: *mut mii_bus) {
    static inline void mdio_active(struct mii_bus *bus)
    {
    out_le32(gpio_regs+0x20, (1 << MDC_PIN(bus)) | (1 << MDIO_PIN(bus)));
    }
#[no_mangle]
pub unsafe extern "C" fn mdio_tristate(bus: *mut mii_bus) {
    static inline void mdio_tristate(struct mii_bus *bus)
    {
    out_le32(gpio_regs+0x30, (1 << MDIO_PIN(bus)));
    }
#[no_mangle]
pub unsafe extern "C" fn mdio_read(bus: *mut mii_bus) -> c_int {
    static inline int mdio_read(struct mii_bus *bus)
    {
    return !!(in_le32(gpio_regs+0x40) & (1 << MDIO_PIN(bus)));
    }
#[no_mangle]
unsafe extern "C" fn clock_out(bus: *mut mii_bus, bit: c_int) {
    static void clock_out(struct mii_bus *bus, int bit)
    {
    if (bit)
    mdio_hi(bus);
    else
    mdio_lo(bus);
    udelay(DELAY);
    mdc_hi(bus);
    udelay(DELAY);
    mdc_lo(bus);
    }
// Utility to send the preamble, address, and register (common to read and write).
#[no_mangle]
unsafe extern "C" fn bitbang_pre(bus: *mut mii_bus, read: c_int, addr: u8, reg: u8) {
    static void bitbang_pre(struct mii_bus *bus, int read, u8 addr, u8 reg)
    {
    int i;
// CFE uses a really long preamble (40 bits). We'll do the same.
    mdio_active(bus);
    for (i = 0; i < 40; i++) {
    clock_out(bus, 1);
    }
// send the start bit (01) and the read opcode (10) or write (10)
    clock_out(bus, 0);
    clock_out(bus, 1);
    clock_out(bus, read);
    clock_out(bus, !read);
// send the PHY address
    for (i = 0; i < 5; i++) {
    clock_out(bus, (addr & 0x10) != 0);
    addr <<= 1;
    }
// send the register address
    for (i = 0; i < 5; i++) {
    clock_out(bus, (reg & 0x10) != 0);
    reg <<= 1;
    }
    }
#[no_mangle]
unsafe extern "C" fn gpio_mdio_read(bus: *mut mii_bus, phy_id: c_int, location: c_int) -> c_int {
    static int gpio_mdio_read(struct mii_bus *bus, int phy_id, int location)
    {
    u16 rdreg;
    int ret, i;
    let mut addr: u8 = phy_id & 0xff;
    let mut reg: u8 = location & 0xff;
    bitbang_pre(bus, 1, addr, reg);
// tri-state our MDIO I/O pin so we can read
    mdio_tristate(bus);
    udelay(DELAY);
    mdc_hi(bus);
    udelay(DELAY);
    mdc_lo(bus);
// read 16 bits of register data, MSB first
    rdreg = 0;
    for (i = 0; i < 16; i++) {
    mdc_lo(bus);
    udelay(DELAY);
    mdc_hi(bus);
    udelay(DELAY);
    mdc_lo(bus);
    udelay(DELAY);
    rdreg <<= 1;
    rdreg |= mdio_read(bus);
    }
    mdc_hi(bus);
    udelay(DELAY);
    mdc_lo(bus);
    udelay(DELAY);
    ret = rdreg;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn gpio_mdio_write(bus: *mut mii_bus, phy_id: c_int, location: c_int, val: u16) -> c_int {
    static int gpio_mdio_write(struct mii_bus *bus, int phy_id, int location, u16 val)
    {
    int i;
    let mut addr: u8 = phy_id & 0xff;
    let mut reg: u8 = location & 0xff;
    let mut value: u16 = val & 0xffff;
    bitbang_pre(bus, 0, addr, reg);
// send the turnaround (10)
    mdc_lo(bus);
    mdio_hi(bus);
    udelay(DELAY);
    mdc_hi(bus);
    udelay(DELAY);
    mdc_lo(bus);
    mdio_lo(bus);
    udelay(DELAY);
    mdc_hi(bus);
    udelay(DELAY);
// write 16 bits of register data, MSB first
    for (i = 0; i < 16; i++) {
    mdc_lo(bus);
    if (value & 0x8000)
    mdio_hi(bus);
    else
    mdio_lo(bus);
    udelay(DELAY);
    mdc_hi(bus);
    udelay(DELAY);
    value <<= 1;
    }
//
// Tri-state the MDIO line.
//
    mdio_tristate(bus);
    mdc_lo(bus);
    udelay(DELAY);
    mdc_hi(bus);
    udelay(DELAY);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gpio_mdio_reset(bus: *mut mii_bus) -> c_int {
    static int gpio_mdio_reset(struct mii_bus *bus)
    {
// nothing here - dunno how to reset it
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gpio_mdio_probe(ofdev: *mut platform_device) -> c_int {
    static int gpio_mdio_probe(struct platform_device *ofdev)
    {
    struct device *dev = &ofdev.dev;
    struct device_node *np = ofdev.dev.of_node;
    struct mii_bus *new_bus;
    struct gpio_priv *priv;
    const unsigned int *prop;
    int err;
    err = -ENOMEM;
    priv = kzalloc_obj(struct gpio_priv);
    if (!priv)
    goto out;
    new_bus = mdiobus_alloc();
    if (!new_bus)
    goto out_free_priv;
    new_bus.name = "pasemi gpio mdio bus";
    new_bus.read = &gpio_mdio_read;
    new_bus.write = &gpio_mdio_write;
    new_bus.reset = &gpio_mdio_reset;
    prop = of_get_property(np, "reg", core::ptr::null_mut());
    snprintf(new_bus.id, MII_BUS_ID_SIZE, "%x", *prop);
    new_bus.priv = priv;
    prop = of_get_property(np, "mdc-pin", core::ptr::null_mut());
    priv.mdc_pin = *prop;
    prop = of_get_property(np, "mdio-pin", core::ptr::null_mut());
    priv.mdio_pin = *prop;
    new_bus.parent = dev;
    dev_set_drvdata(dev, new_bus);
    err = of_mdiobus_register(new_bus, np);
    if (err != 0) {
    pr_err("%s: Cannot register as MDIO bus, err %d\n",
    new_bus.name, err);
    goto out_free_irq;
    }
    return 0;
    out_free_irq:
    kfree(new_bus);
    out_free_priv:
    kfree(priv);
    out:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn gpio_mdio_remove(dev: *mut platform_device) {
    static void gpio_mdio_remove(struct platform_device *dev)
    {
    struct mii_bus *bus = dev_get_drvdata(&dev.dev);
    mdiobus_unregister(bus);
    dev_set_drvdata(&dev.dev, core::ptr::null_mut());
    kfree(bus.priv);
    bus.priv = core::ptr::null_mut();
    mdiobus_free(bus);
    }
    static const struct of_device_id gpio_mdio_match[] =
    {
    {
    .compatible      = "gpio-mdio",
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, gpio_mdio_match);
    static struct platform_driver gpio_mdio_driver =
    {
    .probe		= gpio_mdio_probe,
    .remove		= gpio_mdio_remove,
    .driver = {
    .name = "gpio-mdio-bitbang",
    .of_match_table = gpio_mdio_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn gpio_mdio_init() -> int __init {
    static int __init gpio_mdio_init(void)
    {
    struct device_node *np;
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "1682m-gpio");
    if (!np)
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(),
    "pasemi,pwrficient-gpio");
    if (!np)
    return -ENODEV;
    gpio_regs = of_iomap(np, 0);
    of_node_put(np);
    if (!gpio_regs)
    return -ENODEV;
    return platform_driver_register(&gpio_mdio_driver);
    }
    module_init(gpio_mdio_init);
#[no_mangle]
unsafe extern "C" fn gpio_mdio_exit() -> void __exit {
    static void __exit gpio_mdio_exit(void)
    {
    platform_driver_unregister(&gpio_mdio_driver);
    if (gpio_regs)
    iounmap(gpio_regs);
    }
    module_exit(gpio_mdio_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Olof Johansson <olof@lixom.net>");
    MODULE_DESCRIPTION("Driver for MDIO over GPIO on PA Semi PWRficient-based boards");
