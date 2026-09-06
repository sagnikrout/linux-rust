//! Automatically rewritten from C to Rust
//! Source: drivers/net/dsa/b53/b53_mdio.c
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


//
// B53 register access through MII registers
//
// Copyright (C) 2011-2013 Jonas Gorski <jogo@openwrt.org>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

// MII registers
pub const REG_MII_PAGE: c_uint = 0x10    /* MII Page register */;
pub const REG_MII_ADDR: c_uint = 0x11    /* MII Address register */;
pub const REG_MII_DATA0: c_uint = 0x18    /* MII Data register 0 */;
pub const REG_MII_DATA1: c_uint = 0x19    /* MII Data register 1 */;
pub const REG_MII_DATA2: c_uint = 0x1a    /* MII Data register 2 */;
pub const REG_MII_DATA3: c_uint = 0x1b    /* MII Data register 3 */;

#[no_mangle]
unsafe extern "C" fn b53_mdio_op(dev: *mut b53_device, page: u8, reg: u8, op: u16) -> c_int {
    static int b53_mdio_op(struct b53_device *dev, u8 page, u8 reg, u16 op)
    {
    int i;
    u16 v;
    int ret;
    struct mii_bus *bus = dev.priv;
    if (dev.current_page != page) {
// set page number
    v = (page << 8) | REG_MII_PAGE_ENABLE;
    ret = mdiobus_write_nested(bus, BRCM_PSEUDO_PHY_ADDR,
    REG_MII_PAGE, v);
    if (ret)
    return ret;
    dev.current_page = page;
    }
// set register address
    v = (reg << 8) | op;
    ret = mdiobus_write_nested(bus, BRCM_PSEUDO_PHY_ADDR, REG_MII_ADDR, v);
    if (ret)
    return ret;
// check if operation completed
    for (i = 0; i < 5; ++i) {
    v = mdiobus_read_nested(bus, BRCM_PSEUDO_PHY_ADDR,
    REG_MII_ADDR);
    if (!(v & (REG_MII_ADDR_WRITE | REG_MII_ADDR_READ)))
    break;
    usleep_range(10, 100);
    }
    if (WARN_ON(i == 5))
    return -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn b53_mdio_read8(dev: *mut b53_device, page: u8, reg: u8, val: *mut u8) -> c_int {
    static int b53_mdio_read8(struct b53_device *dev, u8 page, u8 reg, u8 *val)
    {
    struct mii_bus *bus = dev.priv;
    int ret;
    ret = b53_mdio_op(dev, page, reg, REG_MII_ADDR_READ);
    if (ret)
    return ret;
// val = mdiobus_read_nested(bus, BRCM_PSEUDO_PHY_ADDR,
    REG_MII_DATA0) & 0xff;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn b53_mdio_read16(dev: *mut b53_device, page: u8, reg: u8, val: *mut u16) -> c_int {
    static int b53_mdio_read16(struct b53_device *dev, u8 page, u8 reg, u16 *val)
    {
    struct mii_bus *bus = dev.priv;
    int ret;
    ret = b53_mdio_op(dev, page, reg, REG_MII_ADDR_READ);
    if (ret)
    return ret;
// val = mdiobus_read_nested(bus, BRCM_PSEUDO_PHY_ADDR, REG_MII_DATA0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn b53_mdio_read32(dev: *mut b53_device, page: u8, reg: u8, val: *mut u32) -> c_int {
    static int b53_mdio_read32(struct b53_device *dev, u8 page, u8 reg, u32 *val)
    {
    struct mii_bus *bus = dev.priv;
    int ret;
    ret = b53_mdio_op(dev, page, reg, REG_MII_ADDR_READ);
    if (ret)
    return ret;
// val = mdiobus_read_nested(bus, BRCM_PSEUDO_PHY_ADDR, REG_MII_DATA0);
// val |= mdiobus_read_nested(bus, BRCM_PSEUDO_PHY_ADDR,
    REG_MII_DATA1) << 16;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn b53_mdio_read48(dev: *mut b53_device, page: u8, reg: u8, val: *mut u64) -> c_int {
    static int b53_mdio_read48(struct b53_device *dev, u8 page, u8 reg, u64 *val)
    {
    struct mii_bus *bus = dev.priv;
    let mut temp: u64 = 0;
    int i;
    int ret;
    ret = b53_mdio_op(dev, page, reg, REG_MII_ADDR_READ);
    if (ret)
    return ret;
    for (i = 2; i >= 0; i--) {
    temp <<= 16;
    temp |= mdiobus_read_nested(bus, BRCM_PSEUDO_PHY_ADDR,
    REG_MII_DATA0 + i);
    }
// val = temp;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn b53_mdio_read64(dev: *mut b53_device, page: u8, reg: u8, val: *mut u64) -> c_int {
    static int b53_mdio_read64(struct b53_device *dev, u8 page, u8 reg, u64 *val)
    {
    struct mii_bus *bus = dev.priv;
    let mut temp: u64 = 0;
    int i;
    int ret;
    ret = b53_mdio_op(dev, page, reg, REG_MII_ADDR_READ);
    if (ret)
    return ret;
    for (i = 3; i >= 0; i--) {
    temp <<= 16;
    temp |= mdiobus_read_nested(bus, BRCM_PSEUDO_PHY_ADDR,
    REG_MII_DATA0 + i);
    }
// val = temp;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn b53_mdio_write8(dev: *mut b53_device, page: u8, reg: u8, value: u8) -> c_int {
    static int b53_mdio_write8(struct b53_device *dev, u8 page, u8 reg, u8 value)
    {
    struct mii_bus *bus = dev.priv;
    int ret;
    ret = mdiobus_write_nested(bus, BRCM_PSEUDO_PHY_ADDR,
    REG_MII_DATA0, value);
    if (ret)
    return ret;
    return b53_mdio_op(dev, page, reg, REG_MII_ADDR_WRITE);
    }
    static int b53_mdio_write16(struct b53_device *dev, u8 page, u8 reg,
    u16 value)
    {
    struct mii_bus *bus = dev.priv;
    int ret;
    ret = mdiobus_write_nested(bus, BRCM_PSEUDO_PHY_ADDR,
    REG_MII_DATA0, value);
    if (ret)
    return ret;
    return b53_mdio_op(dev, page, reg, REG_MII_ADDR_WRITE);
    }
    static int b53_mdio_write32(struct b53_device *dev, u8 page, u8 reg,
    u32 value)
    {
    struct mii_bus *bus = dev.priv;
    unsigned int i;
    let mut temp: u32 = value;
    for (i = 0; i < 2; i++) {
    int ret = mdiobus_write_nested(bus, BRCM_PSEUDO_PHY_ADDR,
    REG_MII_DATA0 + i,
    temp & 0xffff);
    if (ret)
    return ret;
    temp >>= 16;
    }
    return b53_mdio_op(dev, page, reg, REG_MII_ADDR_WRITE);
    }
    static int b53_mdio_write48(struct b53_device *dev, u8 page, u8 reg,
    u64 value)
    {
    struct mii_bus *bus = dev.priv;
    unsigned int i;
    let mut temp: u64 = value;
    for (i = 0; i < 3; i++) {
    int ret = mdiobus_write_nested(bus, BRCM_PSEUDO_PHY_ADDR,
    REG_MII_DATA0 + i,
    temp & 0xffff);
    if (ret)
    return ret;
    temp >>= 16;
    }
    return b53_mdio_op(dev, page, reg, REG_MII_ADDR_WRITE);
    }
    static int b53_mdio_write64(struct b53_device *dev, u8 page, u8 reg,
    u64 value)
    {
    struct mii_bus *bus = dev.priv;
    unsigned int i;
    let mut temp: u64 = value;
    for (i = 0; i < 4; i++) {
    int ret = mdiobus_write_nested(bus, BRCM_PSEUDO_PHY_ADDR,
    REG_MII_DATA0 + i,
    temp & 0xffff);
    if (ret)
    return ret;
    temp >>= 16;
    }
    return b53_mdio_op(dev, page, reg, REG_MII_ADDR_WRITE);
    }
    static int b53_mdio_phy_read16(struct b53_device *dev, int addr, int reg,
    u16 *value)
    {
    struct mii_bus *bus = dev.priv;
// value = mdiobus_read_nested(bus, addr, reg);
    return 0;
    }
    static int b53_mdio_phy_write16(struct b53_device *dev, int addr, int reg,
    u16 value)
    {
    struct mii_bus *bus = dev.bus;
    return mdiobus_write_nested(bus, addr, reg, value);
    }
    static const struct b53_io_ops b53_mdio_ops = {
    .read8 = b53_mdio_read8,
    .read16 = b53_mdio_read16,
    .read32 = b53_mdio_read32,
    .read48 = b53_mdio_read48,
    .read64 = b53_mdio_read64,
    .write8 = b53_mdio_write8,
    .write16 = b53_mdio_write16,
    .write32 = b53_mdio_write32,
    .write48 = b53_mdio_write48,
    .write64 = b53_mdio_write64,
    .phy_read16 = b53_mdio_phy_read16,
    .phy_write16 = b53_mdio_phy_write16,
    };
pub const B53_BRCM_OUI_1: c_uint = 0x0143bc00;
pub const B53_BRCM_OUI_2: c_uint = 0x03625c00;
pub const B53_BRCM_OUI_3: c_uint = 0x00406000;
pub const B53_BRCM_OUI_4: c_uint = 0x01410c00;
pub const B53_BRCM_OUI_5: c_uint = 0xae025000;
#[no_mangle]
unsafe extern "C" fn b53_mdio_probe(mdiodev: *mut mdio_device) -> c_int {
    static int b53_mdio_probe(struct mdio_device *mdiodev)
    {
    struct b53_device *dev;
    u32 phy_id;
    int ret;
// allow the generic PHY driver to take over the non-management MDIO
// addresses
//
    if (mdiodev.addr != BRCM_PSEUDO_PHY_ADDR && mdiodev.addr != 0) {
    dev_err(&mdiodev.dev, "leaving address %d to PHY\n",
    mdiodev.addr);
    return -ENODEV;
    }
// read the first port's id
    phy_id = mdiobus_read(mdiodev.bus, 0, 2) << 16;
    phy_id |= mdiobus_read(mdiodev.bus, 0, 3);
// BCM5325, BCM539x (OUI_1)
// BCM53125, BCM53128 (OUI_2)
// BCM5365 (OUI_3)
//
    if ((phy_id & 0xfffffc00) != B53_BRCM_OUI_1 &&
    (phy_id & 0xfffffc00) != B53_BRCM_OUI_2 &&
    (phy_id & 0xfffffc00) != B53_BRCM_OUI_3 &&
    (phy_id & 0xfffffc00) != B53_BRCM_OUI_4 &&
    (phy_id & 0xfffffc00) != B53_BRCM_OUI_5) {
    dev_err(&mdiodev.dev, "Unsupported device: 0x%08x\n", phy_id);
    return -ENODEV;
    }
// First probe will come from SWITCH_MDIO controller on the 7445D0
// switch, which will conflict with the 7445 integrated switch
// pseudo-phy (we end-up programming both). In that case, we return
// -EPROBE_DEFER for the first time we get here, and wait until we come
// back with the slave MDIO bus which has the correct indirection
// layer setup
//
    if (of_machine_is_compatible("brcm,bcm7445d0") &&
    strcmp(mdiodev.bus.name, "sf2 user mii"))
    return -EPROBE_DEFER;
    dev = b53_switch_alloc(&mdiodev.dev, &b53_mdio_ops, mdiodev.bus);
    if (!dev)
    return -ENOMEM;
// we don't use page 0xff, so force a page set
    dev.current_page = 0xff;
    dev.bus = mdiodev.bus;
    dev_set_drvdata(&mdiodev.dev, dev);
    ret = b53_switch_register(dev);
    if (ret)
    return dev_err_probe(&mdiodev.dev, ret,
    "failed to register switch\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn b53_mdio_remove(mdiodev: *mut mdio_device) {
    static void b53_mdio_remove(struct mdio_device *mdiodev)
    {
    struct b53_device *dev = dev_get_drvdata(&mdiodev.dev);
    if (!dev)
    return;
    b53_switch_remove(dev);
    }
#[no_mangle]
unsafe extern "C" fn b53_mdio_shutdown(mdiodev: *mut mdio_device) {
    static void b53_mdio_shutdown(struct mdio_device *mdiodev)
    {
    struct b53_device *dev = dev_get_drvdata(&mdiodev.dev);
    if (!dev)
    return;
    b53_switch_shutdown(dev);
    dev_set_drvdata(&mdiodev.dev, core::ptr::null_mut());
    }
    static const struct of_device_id b53_of_match[] = {
    { .compatible = "brcm,bcm5325" },
    { .compatible = "brcm,bcm53101" },
    { .compatible = "brcm,bcm53115" },
    { .compatible = "brcm,bcm53125" },
    { .compatible = "brcm,bcm53128" },
    { .compatible = "brcm,bcm53134" },
    { .compatible = "brcm,bcm5365" },
    { .compatible = "brcm,bcm5389" },
    { .compatible = "brcm,bcm5395" },
    { .compatible = "brcm,bcm5397" },
    { .compatible = "brcm,bcm5398" },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, b53_of_match);
    static struct mdio_driver b53_mdio_driver = {
    .probe	= b53_mdio_probe,
    .remove	= b53_mdio_remove,
    .shutdown = b53_mdio_shutdown,
    .mdiodrv.driver = {
    .name = "bcm53xx",
    .of_match_table = b53_of_match,
    },
    };
    mdio_module_driver(b53_mdio_driver);
    MODULE_DESCRIPTION("B53 MDIO access driver");
    MODULE_LICENSE("Dual BSD/GPL");
