//! Automatically rewritten from C to Rust
//! Source: drivers/net/dsa/lan9303_mdio.c
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
// Copyright (C) 2017 Pengutronix, Juergen Borleis <kernel@pengutronix.de>
//
// Partially based on a patch from
// Copyright (c) 2014 Stefan Roese <sr@denx.de>
//

// Generate phy-addr and -reg from the input address

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan9303_mdio {
    pub device: *mut mdio_device,
    pub chip: lan9303,
}

#[no_mangle]
unsafe extern "C" fn lan9303_mdio_real_write(mdio: *mut mdio_device, reg: c_int, val: u16) {
    static void lan9303_mdio_real_write(struct mdio_device *mdio, int reg, u16 val)
    {
    mdio.bus.write(mdio.bus, PHY_ADDR(reg), PHY_REG(reg), val);
    }
#[no_mangle]
unsafe extern "C" fn lan9303_mdio_write(ctx: *mut c_void, reg: u32, val: u32) -> c_int {
    static int lan9303_mdio_write(void *ctx, uint32_t reg, uint32_t val)
    {
    struct lan9303_mdio *sw_dev = (struct lan9303_mdio *)ctx;
    reg <<= 2; /* reg num to offset */
    mutex_lock_nested(&sw_dev.device.bus.mdio_lock, MDIO_MUTEX_NESTED);
    lan9303_mdio_real_write(sw_dev.device, reg, val & 0xffff);
    lan9303_mdio_real_write(sw_dev.device, reg + 2, (val >> 16) & 0xffff);
    mutex_unlock(&sw_dev.device.bus.mdio_lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lan9303_mdio_real_read(mdio: *mut mdio_device, reg: c_int) -> u16 {
    static u16 lan9303_mdio_real_read(struct mdio_device *mdio, int reg)
    {
    return mdio.bus.read(mdio.bus, PHY_ADDR(reg), PHY_REG(reg));
    }
#[no_mangle]
unsafe extern "C" fn lan9303_mdio_read(ctx: *mut c_void, reg: u32, val: *mut u32) -> c_int {
    static int lan9303_mdio_read(void *ctx, uint32_t reg, uint32_t *val)
    {
    struct lan9303_mdio *sw_dev = (struct lan9303_mdio *)ctx;
    reg <<= 2; /* reg num to offset */
    mutex_lock_nested(&sw_dev.device.bus.mdio_lock, MDIO_MUTEX_NESTED);
// val = lan9303_mdio_real_read(sw_dev->device, reg);
// val |= (lan9303_mdio_real_read(sw_dev->device, reg + 2) << 16);
    mutex_unlock(&sw_dev.device.bus.mdio_lock);
    return 0;
    }
    static int lan9303_mdio_phy_write(struct lan9303 *chip, int addr, int reg,
    u16 val)
    {
    struct lan9303_mdio *sw_dev = dev_get_drvdata(chip.dev);
    return mdiobus_write_nested(sw_dev.device.bus, addr, reg, val);
    }
#[no_mangle]
unsafe extern "C" fn lan9303_mdio_phy_read(chip: *mut lan9303, addr: c_int, reg: c_int) -> c_int {
    static int lan9303_mdio_phy_read(struct lan9303 *chip, int addr, int reg)
    {
    struct lan9303_mdio *sw_dev = dev_get_drvdata(chip.dev);
    return mdiobus_read_nested(sw_dev.device.bus, addr, reg);
    }
    static const struct lan9303_phy_ops lan9303_mdio_phy_ops = {
    .phy_read = lan9303_mdio_phy_read,
    .phy_write = lan9303_mdio_phy_write,
    };
    static const struct regmap_config lan9303_mdio_regmap_config = {
    .reg_bits = 8,
    .val_bits = 32,
    .reg_stride = 1,
    .can_multi_write = true,
    .max_register = 0x0ff, /* address bits 0..1 are not used */
    .reg_format_endian = REGMAP_ENDIAN_LITTLE,
    .volatile_table = &lan9303_register_set,
    .wr_table = &lan9303_register_set,
    .rd_table = &lan9303_register_set,
    .reg_read = lan9303_mdio_read,
    .reg_write = lan9303_mdio_write,
    .cache_type = REGCACHE_NONE,
    };
#[no_mangle]
unsafe extern "C" fn lan9303_mdio_probe(mdiodev: *mut mdio_device) -> c_int {
    static int lan9303_mdio_probe(struct mdio_device *mdiodev)
    {
    struct lan9303_mdio *sw_dev;
    int ret;
    sw_dev = devm_kzalloc(&mdiodev.dev, sizeof(struct lan9303_mdio),
    GFP_KERNEL);
    if (!sw_dev)
    return -ENOMEM;
    sw_dev.chip.regmap = devm_regmap_init(&mdiodev.dev, core::ptr::null_mut(), sw_dev,
    &lan9303_mdio_regmap_config);
    if (IS_ERR(sw_dev.chip.regmap)) {
    ret = PTR_ERR(sw_dev.chip.regmap);
    dev_err(&mdiodev.dev, "regmap init failed: %d\n", ret);
    return ret;
    }
// link forward and backward
    sw_dev.device = mdiodev;
    dev_set_drvdata(&mdiodev.dev, sw_dev);
    sw_dev.chip.dev = &mdiodev.dev;
    sw_dev.chip.ops = &lan9303_mdio_phy_ops;
    ret = lan9303_probe(&sw_dev.chip, mdiodev.dev.of_node);
    if (ret != 0)
    return ret;
    dev_info(&mdiodev.dev, "LAN9303 MDIO driver loaded successfully\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lan9303_mdio_remove(mdiodev: *mut mdio_device) {
    static void lan9303_mdio_remove(struct mdio_device *mdiodev)
    {
    struct lan9303_mdio *sw_dev = dev_get_drvdata(&mdiodev.dev);
    if (!sw_dev)
    return;
    lan9303_remove(&sw_dev.chip);
    }
#[no_mangle]
unsafe extern "C" fn lan9303_mdio_shutdown(mdiodev: *mut mdio_device) {
    static void lan9303_mdio_shutdown(struct mdio_device *mdiodev)
    {
    struct lan9303_mdio *sw_dev = dev_get_drvdata(&mdiodev.dev);
    if (!sw_dev)
    return;
    lan9303_shutdown(&sw_dev.chip);
    dev_set_drvdata(&mdiodev.dev, core::ptr::null_mut());
    }
// -------------------------------------------------------------------------
    static const struct of_device_id lan9303_mdio_of_match[] = {
    { .compatible = "smsc,lan9303-mdio" },
    { .compatible = "microchip,lan9354-mdio" },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, lan9303_mdio_of_match);
    static struct mdio_driver lan9303_mdio_driver = {
    .mdiodrv.driver = {
    .name = "LAN9303_MDIO",
    .of_match_table = lan9303_mdio_of_match,
    },
    .probe  = lan9303_mdio_probe,
    .remove = lan9303_mdio_remove,
    .shutdown = lan9303_mdio_shutdown,
    };
    mdio_module_driver(lan9303_mdio_driver);
    MODULE_AUTHOR("Stefan Roese <sr@denx.de>, Juergen Borleis <kernel@pengutronix.de>");
    MODULE_DESCRIPTION("Driver for SMSC/Microchip LAN9303 three port ethernet switch in MDIO managed mode");
    MODULE_LICENSE("GPL v2");
