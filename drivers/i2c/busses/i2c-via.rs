//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-via.c
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
    i2c Support for Via Technologies 82C586B South Bridge
    Copyright (c) 1998, 1999 Kyösti Mälkki <kmalkki@cc.hut.fi>
//

// Power management registers
pub const PM_CFG_REVID: c_uint = 0x08	/* silicon revision code */;
pub const PM_CFG_IOBASE0: c_uint = 0x20;
pub const PM_CFG_IOBASE1: c_uint = 0x48;

pub const I2C_SCL: c_uint = 0x02	/* clock bit in DIR/OUT/IN register */;
pub const I2C_SDA: c_uint = 0x04;
// io-region reservation
pub const IOSPACE: c_uint = 0x06;
    static struct pci_driver vt586b_driver;
    static u16 pm_io_base;
//
    It does not appear from the datasheet that the GPIO pins are
    open drain. So a we set a low value by setting the direction to
    output and a high value by setting the direction to input and
    relying on the required I2C pullup. The data value is initialized
    to 0 in via_init() and never changed.
//
#[no_mangle]
unsafe extern "C" fn bit_via_setscl(data: *mut c_void, state: c_int) {
    static void bit_via_setscl(void *data, int state)
    {
    outb(state ? inb(I2C_DIR) & ~I2C_SCL : inb(I2C_DIR) | I2C_SCL, I2C_DIR);
    }
#[no_mangle]
unsafe extern "C" fn bit_via_setsda(data: *mut c_void, state: c_int) {
    static void bit_via_setsda(void *data, int state)
    {
    outb(state ? inb(I2C_DIR) & ~I2C_SDA : inb(I2C_DIR) | I2C_SDA, I2C_DIR);
    }
#[no_mangle]
unsafe extern "C" fn bit_via_getscl(data: *mut c_void) -> c_int {
    static int bit_via_getscl(void *data)
    {
    return (0 != (inb(I2C_IN) & I2C_SCL));
    }
#[no_mangle]
unsafe extern "C" fn bit_via_getsda(data: *mut c_void) -> c_int {
    static int bit_via_getsda(void *data)
    {
    return (0 != (inb(I2C_IN) & I2C_SDA));
    }
    static struct i2c_algo_bit_data bit_data = {
    .setsda		= bit_via_setsda,
    .setscl		= bit_via_setscl,
    .getsda		= bit_via_getsda,
    .getscl		= bit_via_getscl,
    .udelay		= 5,
    .timeout	= HZ
    };
    static struct i2c_adapter vt586b_adapter = {
    .owner		= THIS_MODULE,
    .class          = I2C_CLASS_HWMON,
    .name		= "VIA i2c",
    .algo_data	= &bit_data,
    };
    static const struct pci_device_id vt586b_ids[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_VIA, PCI_DEVICE_ID_VIA_82C586_3) },
    { 0, }
    };
    MODULE_DEVICE_TABLE (pci, vt586b_ids);
#[no_mangle]
unsafe extern "C" fn vt586b_probe(dev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int vt586b_probe(struct pci_dev *dev, const struct pci_device_id *id)
    {
    u16 base;
    u8 rev;
    int res;
    if (pm_io_base)
    return dev_err_probe(&dev.dev, -ENODEV,
    "Will only support one host\n");
    pci_read_config_byte(dev, PM_CFG_REVID, &rev);
    switch (rev) {
    case 0x00:
    base = PM_CFG_IOBASE0;
    break;
    case 0x01:
    case 0x10:
    base = PM_CFG_IOBASE1;
    break;
    default:
    base = PM_CFG_IOBASE1;
// later revision
    }
    pci_read_config_word(dev, base, &pm_io_base);
    pm_io_base &= (0xff << 8);
    if (!request_region(I2C_DIR, IOSPACE, vt586b_driver.name))
    return dev_err_probe(&dev.dev, -ENODEV,
    "IO 0x%x-0x%x already in use\n",
    I2C_DIR, I2C_DIR + IOSPACE);
    outb(inb(I2C_DIR) & ~(I2C_SDA | I2C_SCL), I2C_DIR);
    outb(inb(I2C_OUT) & ~(I2C_SDA | I2C_SCL), I2C_OUT);
// set up the sysfs linkage to our parent device
    vt586b_adapter.dev.parent = &dev.dev;
    res = i2c_bit_add_bus(&vt586b_adapter);
    if ( res < 0 ) {
    release_region(I2C_DIR, IOSPACE);
    pm_io_base = 0;
    return res;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vt586b_remove(dev: *mut pci_dev) {
    static void vt586b_remove(struct pci_dev *dev)
    {
    i2c_del_adapter(&vt586b_adapter);
    release_region(I2C_DIR, IOSPACE);
    pm_io_base = 0;
    }
    static struct pci_driver vt586b_driver = {
    .name		= "vt586b_smbus",
    .id_table	= vt586b_ids,
    .probe		= vt586b_probe,
    .remove		= vt586b_remove,
    };
    module_pci_driver(vt586b_driver);
    MODULE_AUTHOR("Kyösti Mälkki <kmalkki@cc.hut.fi>");
    MODULE_DESCRIPTION("i2c for Via vt82c586b southbridge");
    MODULE_LICENSE("GPL");
