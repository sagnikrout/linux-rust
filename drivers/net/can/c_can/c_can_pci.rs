//! Automatically rewritten from C to Rust
//! Source: drivers/net/can/c_can/c_can_pci.c
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
// PCI bus driver for Bosch C_CAN/D_CAN controller
//
// Copyright (C) 2012 Federico Vaga <federico.vaga@gmail.com>
//
// Borrowed from c_can_platform.c
//
// This file is licensed under the terms of the GNU General Public
// License version 2. This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//

pub const PCI_DEVICE_ID_PCH_CAN: c_uint = 0x8818;
pub const PCH_PCI_SOFT_RESET: c_uint = 0x01fc;
    enum c_can_pci_reg_align {
    C_CAN_REG_ALIGN_16,
    C_CAN_REG_ALIGN_32,
    C_CAN_REG_32,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c_can_pci_data {
// Specify if is C_CAN or D_CAN
    pub type: enum c_can_dev_id,
// Number of message objects
    pub msg_obj_num: c_uint,
// Set the register alignment in the memory
    pub reg_align: enum c_can_pci_reg_align,
// Set the frequency
    pub freq: c_uint,
// PCI bar number
    pub bar: c_int,
// Callback for reset
    pub enable): *const *const *const void (init)(struct c_can_priv priv, bool,
}

// 16-bit c_can registers can be arranged differently in the memory
// architecture of different implementations. For example: 16-bit
// registers can be aligned to a 16-bit boundary or 32-bit boundary etc.
// Handle the same by providing a common read/write interface.
//
    static u16 c_can_pci_read_reg_aligned_to_16bit(const struct c_can_priv *priv,
    enum reg index)
    {
    return readw(priv.base + priv.regs[index]);
    }
    static void c_can_pci_write_reg_aligned_to_16bit(const struct c_can_priv *priv,
    enum reg index, u16 val)
    {
    writew(val, priv.base + priv.regs[index]);
    }
    static u16 c_can_pci_read_reg_aligned_to_32bit(const struct c_can_priv *priv,
    enum reg index)
    {
    return readw(priv.base + 2 * priv.regs[index]);
    }
    static void c_can_pci_write_reg_aligned_to_32bit(const struct c_can_priv *priv,
    enum reg index, u16 val)
    {
    writew(val, priv.base + 2 * priv.regs[index]);
    }
    static u16 c_can_pci_read_reg_32bit(const struct c_can_priv *priv,
    enum reg index)
    {
    return (u16)ioread32(priv.base + 2 * priv.regs[index]);
    }
    static void c_can_pci_write_reg_32bit(const struct c_can_priv *priv,
    enum reg index, u16 val)
    {
    iowrite32((u32)val, priv.base + 2 * priv.regs[index]);
    }
#[no_mangle]
unsafe extern "C" fn c_can_pci_read_reg32(priv: *const c_can_priv, index: enum reg) -> u32 {
    static u32 c_can_pci_read_reg32(const struct c_can_priv *priv, enum reg index)
    {
    u32 val;
    val = priv.read_reg(priv, index);
    val |= ((u32)priv.read_reg(priv, index + 1)) << 16;
    return val;
    }
    static void c_can_pci_write_reg32(const struct c_can_priv *priv, enum reg index,
    u32 val)
    {
    priv.write_reg(priv, index + 1, val >> 16);
    priv.write_reg(priv, index, val);
    }
#[no_mangle]
unsafe extern "C" fn c_can_pci_reset_pch(priv: *const c_can_priv, enable: bool) {
    static void c_can_pci_reset_pch(const struct c_can_priv *priv, bool enable)
    {
    if (enable) {
    u32 __iomem *addr = priv.base + PCH_PCI_SOFT_RESET;
// write to sw reset register
    iowrite32(1, addr);
    iowrite32(0, addr);
    }
    }
    static int c_can_pci_probe(struct pci_dev *pdev,
    const struct pci_device_id *ent)
    {
    struct c_can_pci_data *c_can_pci_data = (void *)ent.driver_data;
    struct c_can_priv *priv;
    struct net_device *dev;
    void __iomem *addr;
    int ret;
    ret = pci_enable_device(pdev);
    if (ret) {
    dev_err(&pdev.dev, "pci_enable_device FAILED\n");
    goto out;
    }
    ret = pci_request_regions(pdev, KBUILD_MODNAME);
    if (ret) {
    dev_err(&pdev.dev, "pci_request_regions FAILED\n");
    goto out_disable_device;
    }
    ret = pci_enable_msi(pdev);
    if (!ret) {
    dev_info(&pdev.dev, "MSI enabled\n");
    pci_set_master(pdev);
    }
    addr = pci_iomap(pdev, c_can_pci_data.bar,
    pci_resource_len(pdev, c_can_pci_data.bar));
    if (!addr) {
    dev_err(&pdev.dev,
    "device has no PCI memory resources, failing adapter\n");
    ret = -ENOMEM;
    goto out_release_regions;
    }
// allocate the c_can device
    dev = alloc_c_can_dev(c_can_pci_data.msg_obj_num);
    if (!dev) {
    ret = -ENOMEM;
    goto out_iounmap;
    }
    priv = netdev_priv(dev);
    pci_set_drvdata(pdev, dev);
    SET_NETDEV_DEV(dev, &pdev.dev);
    dev.irq = pdev.irq;
    priv.base = addr;
    priv.device = &pdev.dev;
    if (!c_can_pci_data.freq) {
    dev_err(&pdev.dev, "no clock frequency defined\n");
    ret = -ENODEV;
    goto out_free_c_can;
    } else {
    priv.can.clock.freq = c_can_pci_data.freq;
    }
// Configure CAN type
    switch (c_can_pci_data.type) {
    case BOSCH_C_CAN:
    priv.regs = reg_map_c_can;
    break;
    case BOSCH_D_CAN:
    priv.regs = reg_map_d_can;
    break;
    default:
    ret = -EINVAL;
    goto out_free_c_can;
    }
    priv.type = c_can_pci_data.type;
// Configure access to registers
    switch (c_can_pci_data.reg_align) {
    case C_CAN_REG_ALIGN_32:
    priv.read_reg = c_can_pci_read_reg_aligned_to_32bit;
    priv.write_reg = c_can_pci_write_reg_aligned_to_32bit;
    break;
    case C_CAN_REG_ALIGN_16:
    priv.read_reg = c_can_pci_read_reg_aligned_to_16bit;
    priv.write_reg = c_can_pci_write_reg_aligned_to_16bit;
    break;
    case C_CAN_REG_32:
    priv.read_reg = c_can_pci_read_reg_32bit;
    priv.write_reg = c_can_pci_write_reg_32bit;
    break;
    default:
    ret = -EINVAL;
    goto out_free_c_can;
    }
    priv.read_reg32 = c_can_pci_read_reg32;
    priv.write_reg32 = c_can_pci_write_reg32;
    priv.raminit = c_can_pci_data.init;
    ret = register_c_can_dev(dev);
    if (ret) {
    dev_err(&pdev.dev, "registering %s failed (err=%d)\n",
    KBUILD_MODNAME, ret);
    goto out_free_c_can;
    }
    dev_dbg(&pdev.dev, "%s device registered (regs=%p, irq=%d)\n",
    KBUILD_MODNAME, priv.regs, dev.irq);
    return 0;
    out_free_c_can:
    free_c_can_dev(dev);
    out_iounmap:
    pci_iounmap(pdev, addr);
    out_release_regions:
    pci_disable_msi(pdev);
    pci_release_regions(pdev);
    out_disable_device:
    pci_disable_device(pdev);
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn c_can_pci_remove(pdev: *mut pci_dev) {
    static void c_can_pci_remove(struct pci_dev *pdev)
    {
    struct net_device *dev = pci_get_drvdata(pdev);
    struct c_can_priv *priv = netdev_priv(dev);
    void __iomem *addr = priv.base;
    unregister_c_can_dev(dev);
    free_c_can_dev(dev);
    pci_iounmap(pdev, addr);
    pci_disable_msi(pdev);
    pci_release_regions(pdev);
    pci_disable_device(pdev);
    }
    static const struct c_can_pci_data c_can_sta2x11 = {
    .type = BOSCH_C_CAN,
    .msg_obj_num = 32,
    .reg_align = C_CAN_REG_ALIGN_32,
    .freq = 52000000, /* 52 Mhz */
    .bar = 0,
    };
    static const struct c_can_pci_data c_can_pch = {
    .type = BOSCH_C_CAN,
    .msg_obj_num = 32,
    .reg_align = C_CAN_REG_32,
    .freq = 50000000, /* 50 MHz */
    .init = c_can_pci_reset_pch,
    .bar = 1,
    };

    PCI_DEVICE(_vend, _dev),			\
    .driver_data = (unsigned long)&(_driverdata),	\
    }
    static const struct pci_device_id c_can_pci_tbl[] = {
    C_CAN_ID(PCI_VENDOR_ID_STMICRO, PCI_DEVICE_ID_STMICRO_CAN,
    c_can_sta2x11),
    C_CAN_ID(PCI_VENDOR_ID_INTEL, PCI_DEVICE_ID_PCH_CAN,
    c_can_pch),
    {},
    };
    static struct pci_driver c_can_pci_driver = {
    .name = KBUILD_MODNAME,
    .id_table = c_can_pci_tbl,
    .probe = c_can_pci_probe,
    .remove = c_can_pci_remove,
    };
    module_pci_driver(c_can_pci_driver);
    MODULE_AUTHOR("Federico Vaga <federico.vaga@gmail.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("PCI CAN bus driver for Bosch C_CAN/D_CAN controller");
    MODULE_DEVICE_TABLE(pci, c_can_pci_tbl);
