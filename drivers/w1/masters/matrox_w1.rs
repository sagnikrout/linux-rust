//! Automatically rewritten from C to Rust
//! Source: drivers/w1/masters/matrox_w1.c
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
// matrox_w1.c
//
// Copyright (c) 2004 Evgeniy Polyakov <zbr@ioremap.net>
//

//
// Matrox G400 DDC registers.
//

pub const MATROX_BASE: c_uint = 0x3C00;
pub const MATROX_STATUS: c_uint = 0x1e14;
pub const MATROX_PORT_INDEX_OFFSET: c_uint = 0x00;
pub const MATROX_PORT_DATA_OFFSET: c_uint = 0x0A;
pub const MATROX_GET_CONTROL: c_uint = 0x2A;
pub const MATROX_GET_DATA: c_uint = 0x2B;
pub const MATROX_CURSOR_CTL: c_uint = 0x06;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct matrox_device {
    pub base_addr: *mut void __iomem,
    pub port_index: *mut void __iomem,
    pub port_data: *mut void __iomem,
    pub data_mask: u8,
    pub phys_addr: c_ulong,
    pub virt_addr: *mut void __iomem,
    pub bus_master: *mut w1_bus_master,
}

//
// These functions read and write DDC Data bit.
//
// Using tristate pins, since i can't find any open-drain pin in whole motherboard.
// Unfortunately we can't connect to Intel's 82801xx IO controller
// since we don't know motherboard schema, which has pretty unused(may be not) GPIO.
//
// I've heard that PIIX also has open drain pin.
//
// Port mapping.
//
#[no_mangle]
pub unsafe extern "C" fn matrox_w1_read_reg(dev: *mut matrox_device, reg: u8) -> u8 {
    static inline u8 matrox_w1_read_reg(struct matrox_device *dev, u8 reg)
    {
    u8 ret;
    writeb(reg, dev.port_index);
    ret = readb(dev.port_data);
    barrier();
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn matrox_w1_write_reg(dev: *mut matrox_device, reg: u8, val: u8) {
    static inline void matrox_w1_write_reg(struct matrox_device *dev, u8 reg, u8 val)
    {
    writeb(reg, dev.port_index);
    writeb(val, dev.port_data);
    wmb();
    }
#[no_mangle]
unsafe extern "C" fn matrox_w1_write_ddc_bit(data: *mut c_void, bit: u8) {
    static void matrox_w1_write_ddc_bit(void *data, u8 bit)
    {
    u8 ret;
    struct matrox_device *dev = data;
    if (bit)
    bit = 0;
    else
    bit = dev.data_mask;
    ret = matrox_w1_read_reg(dev, MATROX_GET_CONTROL);
    matrox_w1_write_reg(dev, MATROX_GET_CONTROL, ((ret & ~dev.data_mask) | bit));
    matrox_w1_write_reg(dev, MATROX_GET_DATA, 0x00);
    }
#[no_mangle]
unsafe extern "C" fn matrox_w1_read_ddc_bit(data: *mut c_void) -> u8 {
    static u8 matrox_w1_read_ddc_bit(void *data)
    {
    u8 ret;
    struct matrox_device *dev = data;
    ret = matrox_w1_read_reg(dev, MATROX_GET_DATA);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn matrox_w1_hw_init(dev: *mut matrox_device) {
    static void matrox_w1_hw_init(struct matrox_device *dev)
    {
    matrox_w1_write_reg(dev, MATROX_GET_DATA, 0xFF);
    matrox_w1_write_reg(dev, MATROX_GET_CONTROL, 0x00);
    }
#[no_mangle]
unsafe extern "C" fn matrox_w1_probe(pdev: *mut pci_dev, ent: *const pci_device_id) -> c_int {
    static int matrox_w1_probe(struct pci_dev *pdev, const struct pci_device_id *ent)
    {
    struct matrox_device *dev;
    int err;
    if (pdev.vendor != PCI_VENDOR_ID_MATROX || pdev.device != PCI_DEVICE_ID_MATROX_G400)
    return -ENODEV;
    dev = kzalloc(sizeof(struct matrox_device) +
    sizeof(struct w1_bus_master), GFP_KERNEL);
    if (!dev)
    return -ENOMEM;
    dev.bus_master = (struct w1_bus_master *)(dev + 1);
//
// True for G400, for some other we need resource 0, see drivers/video/matrox/matroxfb_base.c
//
    dev.phys_addr = pci_resource_start(pdev, 1);
    dev.virt_addr = ioremap(dev.phys_addr, 16384);
    if (!dev.virt_addr) {
    dev_err(&pdev.dev, "%s: failed to ioremap(0x%lx, %d).\n",
    __func__, dev.phys_addr, 16384);
    err = -EIO;
    goto err_out_free_device;
    }
    dev.base_addr = dev.virt_addr + MATROX_BASE;
    dev.port_index = dev.base_addr + MATROX_PORT_INDEX_OFFSET;
    dev.port_data = dev.base_addr + MATROX_PORT_DATA_OFFSET;
    dev.data_mask = (MATROX_G400_DDC_DATA);
    matrox_w1_hw_init(dev);
    dev.bus_master.data = dev;
    dev.bus_master.read_bit = &matrox_w1_read_ddc_bit;
    dev.bus_master.write_bit = &matrox_w1_write_ddc_bit;
    err = w1_add_master_device(dev.bus_master);
    if (err)
    goto err_out_free_device;
    pci_set_drvdata(pdev, dev);
    dev_info(&pdev.dev, "Matrox G400 GPIO transport layer for 1-wire.\n");
    return 0;
    err_out_free_device:
    if (dev.virt_addr)
    iounmap(dev.virt_addr);
    kfree(dev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn matrox_w1_remove(pdev: *mut pci_dev) {
    static void matrox_w1_remove(struct pci_dev *pdev)
    {
    struct matrox_device *dev = pci_get_drvdata(pdev);
    w1_remove_master_device(dev.bus_master);
    iounmap(dev.virt_addr);
    kfree(dev);
    }
    static struct pci_device_id matrox_w1_tbl[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_MATROX, PCI_DEVICE_ID_MATROX_G400) },
    { },
    };
    MODULE_DEVICE_TABLE(pci, matrox_w1_tbl);
    static struct pci_driver matrox_w1_pci_driver = {
    .name = "matrox_w1",
    .id_table = matrox_w1_tbl,
    .probe = matrox_w1_probe,
    .remove = matrox_w1_remove,
    };
    module_pci_driver(matrox_w1_pci_driver);
    MODULE_AUTHOR("Evgeniy Polyakov <zbr@ioremap.net>");
    MODULE_DESCRIPTION("Driver for transport(Dallas 1-wire protocol) over VGA DDC(matrox gpio).");
    MODULE_LICENSE("GPL");
