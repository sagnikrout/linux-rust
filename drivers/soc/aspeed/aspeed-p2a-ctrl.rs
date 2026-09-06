//! Automatically rewritten from C to Rust
//! Source: drivers/soc/aspeed/aspeed-p2a-ctrl.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright 2019 Google Inc
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//
// Provides a simple driver to control the ASPEED P2A interface which allows
// the host to read and write to various regions of the BMC's memory.
//

// SCU2C is a Misc. Control Register.
pub const SCU2C: c_uint = 0x2c;
// SCU180 is the PCIe Configuration Setting Control Register.
pub const SCU180: c_uint = 0x180;
// Bit 1 controls the P2A bridge, while bit 0 controls the entire VGA device
// on the PCI bus.
//

// The ast2400/2500 both have six ranges.
pub const P2A_REGION_COUNT: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct region {
    pub min: u64,
    pub max: u64,
    pub bit: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_p2a_model_data {
// min, max, bit
    pub regions: [region; P2A_REGION_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_p2a_ctrl {
    pub miscdev: miscdevice,
    pub regmap: *mut regmap,
    pub config: *const aspeed_p2a_model_data,
// Access to these needs to be locked, held via probe, mapping ioctl,
// and release, remove.
//
    pub tracking: mutex,
    pub readers: u32,
    pub readerwriters: [u32; P2A_REGION_COUNT],
    pub mem_base: phys_addr_t,
    pub mem_size: resource_size_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_p2a_user {
    pub file: *mut file,
    pub parent: *mut aspeed_p2a_ctrl,
// The entire memory space is opened for reading once the bridge is
// enabled, therefore this needs only to be tracked once per user.
// If any user has it open for read, the bridge must stay enabled.
//
    pub read: u32,
// Each entry of the array corresponds to a P2A Region.  If the user
// opens for read or readwrite, the reference goes up here.  On
// release, this array is walked and references adjusted accordingly.
//
    pub readwrite: [u32; P2A_REGION_COUNT],
}

#[no_mangle]
unsafe extern "C" fn aspeed_p2a_enable_bridge(p2a_ctrl: *mut aspeed_p2a_ctrl) {
    static void aspeed_p2a_enable_bridge(struct aspeed_p2a_ctrl *p2a_ctrl)
    {
    regmap_update_bits(p2a_ctrl.regmap,
    SCU180, SCU180_ENP2A, SCU180_ENP2A);
    }
#[no_mangle]
unsafe extern "C" fn aspeed_p2a_disable_bridge(p2a_ctrl: *mut aspeed_p2a_ctrl) {
    static void aspeed_p2a_disable_bridge(struct aspeed_p2a_ctrl *p2a_ctrl)
    {
    regmap_update_bits(p2a_ctrl.regmap, SCU180, SCU180_ENP2A, 0);
    }
#[no_mangle]
unsafe extern "C" fn aspeed_p2a_mmap(file: *mut file, vma: *mut vm_area_struct) -> c_int {
    static int aspeed_p2a_mmap(struct file *file, struct vm_area_struct *vma)
    {
    unsigned long vsize;
    pgprot_t prot;
    struct aspeed_p2a_user *priv = file.private_data;
    struct aspeed_p2a_ctrl *ctrl = priv.parent;
    if (ctrl.mem_base == 0 && ctrl.mem_size == 0)
    return -EINVAL;
    vsize = vma.vm_end - vma.vm_start;
    prot = vma.vm_page_prot;
    if (vma.vm_pgoff + vma_pages(vma) > ctrl.mem_size >> PAGE_SHIFT)
    return -EINVAL;
// ast2400/2500 AHB accesses are not cache coherent
    prot = pgprot_noncached(prot);
    if (remap_pfn_range(vma, vma.vm_start,
    (ctrl.mem_base >> PAGE_SHIFT) + vma.vm_pgoff,
    vsize, prot))
    return -EAGAIN;
    return 0;
    }
    static bool aspeed_p2a_region_acquire(struct aspeed_p2a_user *priv,
    struct aspeed_p2a_ctrl *ctrl,
    struct aspeed_p2a_ctrl_mapping *map)
    {
    int i;
    u64 base, end;
    let mut matched: bool = false;
    base = map.addr;
    end = map.addr + (map.length - 1);
// If the value is a legal u32, it will find a match.
    for (i = 0; i < P2A_REGION_COUNT; i++) {
    const struct region *curr = &ctrl.config.regions[i];
// If the top of this region is lower than your base, skip it.
//
    if (curr.max < base)
    continue;
// If the bottom of this region is higher than your end, bail.
//
    if (curr.min > end)
    break;
// Lock this and update it, therefore it someone else is
// closing their file out, this'll preserve the increment.
//
    mutex_lock(&ctrl.tracking);
    ctrl.readerwriters[i] += 1;
    mutex_unlock(&ctrl.tracking);
// Track with the user, so when they close their file, we can
// decrement properly.
//
    priv.readwrite[i] += 1;
// Enable the region as read-write.
    regmap_update_bits(ctrl.regmap, SCU2C, curr.bit, 0);
    matched = true;
    }
    return matched;
    }
    static long aspeed_p2a_ioctl(struct file *file, unsigned int cmd,
    unsigned long data)
    {
    struct aspeed_p2a_user *priv = file.private_data;
    struct aspeed_p2a_ctrl *ctrl = priv.parent;
    void __user *arg = (void __user *)data;
    struct aspeed_p2a_ctrl_mapping map;
    if (copy_from_user(&map, arg, sizeof(map)))
    return -EFAULT;
    switch (cmd) {
    case ASPEED_P2A_CTRL_IOCTL_SET_WINDOW:
// If they want a region to be read-only, since the entire
// region is read-only once enabled, we just need to track this
// user wants to read from the bridge, and if it's not enabled.
// Enable it.
//
    if (map.flags == ASPEED_P2A_CTRL_READ_ONLY) {
    mutex_lock(&ctrl.tracking);
    ctrl.readers += 1;
    mutex_unlock(&ctrl.tracking);
// Track with the user, so when they close their file,
// we can decrement properly.
//
    priv.read += 1;
    } else if (map.flags == ASPEED_P2A_CTRL_READWRITE) {
// If we don't acquire any region return error.
    if (!aspeed_p2a_region_acquire(priv, ctrl, &map)) {
    return -EINVAL;
    }
    } else {
// Invalid map flags.
    return -EINVAL;
    }
    aspeed_p2a_enable_bridge(ctrl);
    return 0;
    case ASPEED_P2A_CTRL_IOCTL_GET_MEMORY_CONFIG:
// This is a request for the memory-region and corresponding
// length that is used by the driver for mmap.
//
    map.flags = 0;
    map.addr = ctrl.mem_base;
    map.length = ctrl.mem_size;
    return copy_to_user(arg, &map, sizeof(map)) ? -EFAULT : 0;
    }
    return -EINVAL;
    }
//
// When a user opens this file, we create a structure to track their mappings.
//
// A user can map a region as read-only (bridge enabled), or read-write (bit
// flipped, and bridge enabled).  Either way, this tracking is used, s.t. when
// they release the device references are handled.
//
// The bridge is not enabled until a user calls an ioctl to map a region,
// simply opening the device does not enable it.
//
#[no_mangle]
unsafe extern "C" fn aspeed_p2a_open(inode: *mut inode, file: *mut file) -> c_int {
    static int aspeed_p2a_open(struct inode *inode, struct file *file)
    {
    struct aspeed_p2a_user *priv;
    priv = kmalloc_obj(*priv);
    if (!priv)
    return -ENOMEM;
    priv.file = file;
    priv.read = 0;
    memset(priv.readwrite, 0, sizeof(priv.readwrite));
// The file's private_data is initialized to the p2a_ctrl.
    priv.parent = file.private_data;
// Set the file's private_data to the user's data.
    file.private_data = priv;
    return 0;
    }
//
// This will close the users mappings.  It will go through what they had opened
// for readwrite, and decrement those counts.  If at the end, this is the last
// user, it'll close the bridge.
//
#[no_mangle]
unsafe extern "C" fn aspeed_p2a_release(inode: *mut inode, file: *mut file) -> c_int {
    static int aspeed_p2a_release(struct inode *inode, struct file *file)
    {
    int i;
    let mut bits: u32 = 0;
    let mut open_regions: bool = false;
    struct aspeed_p2a_user *priv = file.private_data;
// Lock others from changing these values until everything is updated
// in one pass.
//
    mutex_lock(&priv.parent.tracking);
    priv.parent.readers -= priv.read;
    for (i = 0; i < P2A_REGION_COUNT; i++) {
    priv.parent.readerwriters[i] -= priv.readwrite[i];
    if (priv.parent.readerwriters[i] > 0)
    open_regions = true;
    else
    bits |= priv.parent.config.regions[i].bit;
    }
// Setting a bit to 1 disables the region, so let's just OR with the
// above to disable any.
//
// Note, if another user is trying to ioctl, they can't grab tracking,
// and therefore can't grab either register mutex.
// If another user is trying to close, they can't grab tracking either.
//
    regmap_update_bits(priv.parent.regmap, SCU2C, bits, bits);
// If parent->readers is zero and open windows is 0, disable the
// bridge.
//
    if (!open_regions && priv.parent.readers == 0)
    aspeed_p2a_disable_bridge(priv.parent);
    mutex_unlock(&priv.parent.tracking);
    kfree(priv);
    return 0;
    }
    static const struct file_operations aspeed_p2a_ctrl_fops = {
    .owner = THIS_MODULE,
    .mmap = aspeed_p2a_mmap,
    .unlocked_ioctl = aspeed_p2a_ioctl,
    .open = aspeed_p2a_open,
    .release = aspeed_p2a_release,
    };
// The regions are controlled by SCU2C
#[no_mangle]
unsafe extern "C" fn aspeed_p2a_disable_all(p2a_ctrl: *mut aspeed_p2a_ctrl) {
    static void aspeed_p2a_disable_all(struct aspeed_p2a_ctrl *p2a_ctrl)
    {
    int i;
    let mut value: u32 = 0;
    for (i = 0; i < P2A_REGION_COUNT; i++)
    value |= p2a_ctrl.config.regions[i].bit;
    regmap_update_bits(p2a_ctrl.regmap, SCU2C, value, value);
// Disable the bridge.
    aspeed_p2a_disable_bridge(p2a_ctrl);
    }
#[no_mangle]
unsafe extern "C" fn aspeed_p2a_ctrl_probe(pdev: *mut platform_device) -> c_int {
    static int aspeed_p2a_ctrl_probe(struct platform_device *pdev)
    {
    struct aspeed_p2a_ctrl *misc_ctrl;
    struct device *dev;
    struct resource resm;
    let mut rc: c_int = 0;
    dev = &pdev.dev;
    misc_ctrl = devm_kzalloc(dev, sizeof(*misc_ctrl), GFP_KERNEL);
    if (!misc_ctrl)
    return -ENOMEM;
    mutex_init(&misc_ctrl.tracking);
// optional.
    rc = of_reserved_mem_region_to_resource(dev.of_node, 0, &resm);
    if (!rc) {
    misc_ctrl.mem_size = resource_size(&resm);
    misc_ctrl.mem_base = resm.start;
    }
    misc_ctrl.regmap = syscon_node_to_regmap(pdev.dev.parent.of_node);
    if (IS_ERR(misc_ctrl.regmap)) {
    dev_err(dev, "Couldn't get regmap\n");
    return -ENODEV;
    }
    misc_ctrl.config = of_device_get_match_data(dev);
    dev_set_drvdata(&pdev.dev, misc_ctrl);
    aspeed_p2a_disable_all(misc_ctrl);
    misc_ctrl.miscdev.minor = MISC_DYNAMIC_MINOR;
    misc_ctrl.miscdev.name = DEVICE_NAME;
    misc_ctrl.miscdev.fops = &aspeed_p2a_ctrl_fops;
    misc_ctrl.miscdev.parent = dev;
    rc = misc_register(&misc_ctrl.miscdev);
    if (rc)
    dev_err(dev, "Unable to register device\n");
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn aspeed_p2a_ctrl_remove(pdev: *mut platform_device) {
    static void aspeed_p2a_ctrl_remove(struct platform_device *pdev)
    {
    struct aspeed_p2a_ctrl *p2a_ctrl = dev_get_drvdata(&pdev.dev);
    misc_deregister(&p2a_ctrl.miscdev);
    }

    static const struct aspeed_p2a_model_data ast2400_model_data = {
    .regions = {
    {0x00000000, 0x17FFFFFF, SCU2C_FLASH},
    {0x18000000, 0x1FFFFFFF, SCU2C_SOC},
    {0x20000000, 0x2FFFFFFF, SCU2C_FLASH},
    {0x30000000, 0x3FFFFFFF, SCU2C_SPI},
    {0x40000000, 0x5FFFFFFF, SCU2C_DRAM},
    {0x60000000, 0xFFFFFFFF, SCU2C_SOC},
    }
    };
    static const struct aspeed_p2a_model_data ast2500_model_data = {
    .regions = {
    {0x00000000, 0x0FFFFFFF, SCU2C_FLASH},
    {0x10000000, 0x1FFFFFFF, SCU2C_SOC},
    {0x20000000, 0x3FFFFFFF, SCU2C_FLASH},
    {0x40000000, 0x5FFFFFFF, SCU2C_SOC},
    {0x60000000, 0x7FFFFFFF, SCU2C_SPI},
    {0x80000000, 0xFFFFFFFF, SCU2C_DRAM},
    }
    };
    static const struct of_device_id aspeed_p2a_ctrl_match[] = {
    { .compatible = "aspeed,ast2400-p2a-ctrl",
    .data = &ast2400_model_data },
    { .compatible = "aspeed,ast2500-p2a-ctrl",
    .data = &ast2500_model_data },
    { },
    };
    MODULE_DEVICE_TABLE(of, aspeed_p2a_ctrl_match);
    static struct platform_driver aspeed_p2a_ctrl_driver = {
    .driver = {
    .name		= DEVICE_NAME,
    .of_match_table = aspeed_p2a_ctrl_match,
    },
    .probe = aspeed_p2a_ctrl_probe,
    .remove = aspeed_p2a_ctrl_remove,
    };
    module_platform_driver(aspeed_p2a_ctrl_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Patrick Venture <venture@google.com>");
    MODULE_DESCRIPTION("Control for aspeed 2400/2500 P2A VGA HOST to BMC mappings");
