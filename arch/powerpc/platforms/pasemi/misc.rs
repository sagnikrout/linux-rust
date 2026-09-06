//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/pasemi/misc.c
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
// Copyright (C) 2007 PA Semi, Inc
//
// Parts based on arch/powerpc/sysdev/fsl_soc.c:
//
// 2006 (c) MontaVista Software, Inc.
//

// The below is from fsl_soc.c.  It's copied because since there are no
// official bus bindings at this time it doesn't make sense to share across
// the platforms, even though they happen to be common.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_driver_device {
    pub of_device: *mut c_char,
    pub i2c_type: *mut c_char,
}

    static struct i2c_driver_device i2c_devices[] __initdata = {
    {"dallas,ds1338",  "ds1338"},
    };
    static int __init find_i2c_driver(struct device_node *node,
    struct i2c_board_info *info)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(i2c_devices); i++) {
    if (!of_device_is_compatible(node, i2c_devices[i].of_device))
    continue;
    if (strscpy(info.type, i2c_devices[i].i2c_type, I2C_NAME_SIZE) < 0)
    return -ENOMEM;
    return 0;
    }
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn pasemi_register_i2c_devices() -> int __init {
    static int __init pasemi_register_i2c_devices(void)
    {
    struct pci_dev *pdev;
    struct device_node *adap_node;
    struct device_node *node;
    pdev = core::ptr::null_mut();
    while ((pdev = pci_get_device(PCI_VENDOR_ID_PASEMI, 0xa003, pdev))) {
    adap_node = pci_device_to_OF_node(pdev);
    if (!adap_node)
    continue;
    for_each_child_of_node(adap_node, node) {
    let mut info: i2c_board_info = {};
    const u32 *addr;
    int len;
    addr = of_get_property(node, "reg", &len);
    if (!addr || len < sizeof(int) ||
// addr > (1 << 10) - 1) {
    pr_warn("pasemi_register_i2c_devices: invalid i2c device entry\n");
    continue;
    }
    info.irq = irq_of_parse_and_map(node, 0);
    if (!info.irq)
    info.irq = -1;
    if (find_i2c_driver(node, &info) < 0)
    continue;
    info.addr = *addr;
    i2c_register_board_info(PCI_FUNC(pdev.devfn), &info,
    1);
    }
    }
    return 0;
    }
    device_initcall(pasemi_register_i2c_devices);
