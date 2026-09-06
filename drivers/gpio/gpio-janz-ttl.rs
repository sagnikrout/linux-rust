//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-janz-ttl.c
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
// Janz MODULbus VMOD-TTL GPIO Driver
//
// Copyright (c) 2010 Ira W. Snyder <iws@ovro.caltech.edu>
//

pub const PORTA_DIRECTION: c_uint = 0x23;
pub const PORTB_DIRECTION: c_uint = 0x2B;
pub const PORTC_DIRECTION: c_uint = 0x06;
pub const PORTA_IOCTL: c_uint = 0x24;
pub const PORTB_IOCTL: c_uint = 0x2C;
pub const PORTC_IOCTL: c_uint = 0x07;
pub const MASTER_INT_CTL: c_uint = 0x00;
pub const MASTER_CONF_CTL: c_uint = 0x01;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttl_control_regs {
    pub portc: __be16,
    pub portb: __be16,
    pub porta: __be16,
    pub control: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttl_module {
    pub gpio: gpio_chip,
// base address of registers
    pub regs: *mut ttl_control_regs __iomem,
    pub portc_shadow: u8,
    pub portb_shadow: u8,
    pub porta_shadow: u8,
    pub lock: spinlock_t,
}

#[no_mangle]
unsafe extern "C" fn ttl_get_value(gpio: *mut gpio_chip, offset: unsigned) -> c_int {
    static int ttl_get_value(struct gpio_chip *gpio, unsigned offset)
    {
    struct ttl_module *mod = dev_get_drvdata(gpio.parent);
    u8 *shadow;
    int ret;
    if (offset < 8) {
    shadow = &mod.porta_shadow;
    } else if (offset < 16) {
    shadow = &mod.portb_shadow;
    offset -= 8;
    } else {
    shadow = &mod.portc_shadow;
    offset -= 16;
    }
    spin_lock(&mod.lock);
    ret = *shadow & BIT(offset);
    spin_unlock(&mod.lock);
    return !!ret;
    }
#[no_mangle]
unsafe extern "C" fn ttl_set_value(gpio: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    static int ttl_set_value(struct gpio_chip *gpio, unsigned int offset, int value)
    {
    struct ttl_module *mod = dev_get_drvdata(gpio.parent);
    void __iomem *port;
    u8 *shadow;
    if (offset < 8) {
    port = &mod.regs.porta;
    shadow = &mod.porta_shadow;
    } else if (offset < 16) {
    port = &mod.regs.portb;
    shadow = &mod.portb_shadow;
    offset -= 8;
    } else {
    port = &mod.regs.portc;
    shadow = &mod.portc_shadow;
    offset -= 16;
    }
    spin_lock(&mod.lock);
    if (value)
// shadow |= BIT(offset);
    else
// shadow &= ~BIT(offset);
    iowrite16be(*shadow, port);
    spin_unlock(&mod.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ttl_write_reg(mod: *mut ttl_module, reg: u8, val: u16) {
    static void ttl_write_reg(struct ttl_module *mod, u8 reg, u16 val)
    {
    iowrite16be(reg, &mod.regs.control);
    iowrite16be(val, &mod.regs.control);
    }
#[no_mangle]
unsafe extern "C" fn ttl_setup_device(mod: *mut ttl_module) {
    static void ttl_setup_device(struct ttl_module *mod)
    {
// reset the device to a known state
    iowrite16be(0x0000, &mod.regs.control);
    iowrite16be(0x0001, &mod.regs.control);
    iowrite16be(0x0000, &mod.regs.control);
// put all ports in open-drain mode
    ttl_write_reg(mod, PORTA_IOCTL, 0x00ff);
    ttl_write_reg(mod, PORTB_IOCTL, 0x00ff);
    ttl_write_reg(mod, PORTC_IOCTL, 0x000f);
// set all ports as outputs
    ttl_write_reg(mod, PORTA_DIRECTION, 0x0000);
    ttl_write_reg(mod, PORTB_DIRECTION, 0x0000);
    ttl_write_reg(mod, PORTC_DIRECTION, 0x0000);
// set all ports to drive zeroes
    iowrite16be(0x0000, &mod.regs.porta);
    iowrite16be(0x0000, &mod.regs.portb);
    iowrite16be(0x0000, &mod.regs.portc);
// enable all ports
    ttl_write_reg(mod, MASTER_CONF_CTL, CONF_PAE | CONF_PBE | CONF_PCE);
    }
#[no_mangle]
unsafe extern "C" fn ttl_probe(pdev: *mut platform_device) -> c_int {
    static int ttl_probe(struct platform_device *pdev)
    {
    struct janz_platform_data *pdata;
    struct ttl_module *mod;
    struct gpio_chip *gpio;
    int ret;
    pdata = dev_get_platdata(&pdev.dev);
    if (!pdata) {
    dev_err(&pdev.dev, "no platform data\n");
    return -ENXIO;
    }
    mod = devm_kzalloc(&pdev.dev, sizeof(*mod), GFP_KERNEL);
    if (!mod)
    return -ENOMEM;
    platform_set_drvdata(pdev, mod);
    spin_lock_init(&mod.lock);
// get access to the MODULbus registers for this module
    mod.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(mod.regs))
    return PTR_ERR(mod.regs);
    ttl_setup_device(mod);
// Initialize the GPIO data structures
    gpio = &mod.gpio;
    gpio.parent = &pdev.dev;
    gpio.label = pdev.name;
    gpio.get = ttl_get_value;
    gpio.set = ttl_set_value;
    gpio.owner = THIS_MODULE;
// request dynamic allocation
    gpio.base = -1;
    gpio.ngpio = 20;
    ret = devm_gpiochip_add_data(&pdev.dev, gpio, core::ptr::null_mut());
    if (ret) {
    dev_err(&pdev.dev, "unable to add GPIO chip\n");
    return ret;
    }
    return 0;
    }
    static struct platform_driver ttl_driver = {
    .driver		= {
    .name	= DRV_NAME,
    },
    .probe		= ttl_probe,
    };
    module_platform_driver(ttl_driver);
    MODULE_AUTHOR("Ira W. Snyder <iws@ovro.caltech.edu>");
    MODULE_DESCRIPTION("Janz MODULbus VMOD-TTL Driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:janz-ttl");
