//! Automatically rewritten from C to Rust
//! Source: drivers/reset/reset-raspberrypi.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Raspberry Pi 4 firmware reset driver
//
// Copyright (C) 2020 Nicolas Saenz Julienne <nsaenzjulienne@suse.de>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpi_reset {
    pub rcdev: reset_controller_dev,
    pub fw: *mut rpi_firmware,
}

    static inline struct rpi_reset *to_rpi(struct reset_controller_dev *rcdev)
    {
    return container_of(rcdev, struct rpi_reset, rcdev);
    }
#[no_mangle]
unsafe extern "C" fn rpi_reset_reset(rcdev: *mut reset_controller_dev, id: c_ulong) -> c_int {
    static int rpi_reset_reset(struct reset_controller_dev *rcdev, unsigned long id)
    {
    struct rpi_reset *priv = to_rpi(rcdev);
    u32 dev_addr;
    int ret;
    switch (id) {
    case RASPBERRYPI_FIRMWARE_RESET_ID_USB:
//
// The Raspberry Pi 4 gets its USB functionality from VL805, a
// PCIe chip that implements xHCI. After a PCI reset, VL805's
// firmware may either be loaded directly from an EEPROM or, if
// not present, by the SoC's co-processor, VideoCore. rpi's
// VideoCore OS contains both the non public firmware load
// logic and the VL805 firmware blob. This triggers the
// aforementioned process.
//
// The pci device address is expected is expected by the
// firmware encoded like this:
//
// PCI_BUS << 20 | PCI_SLOT << 15 | PCI_FUNC << 12
//
// But since rpi's PCIe is hardwired, we know the address in
// advance.
//
    dev_addr = 0x100000;
    ret = rpi_firmware_property(priv.fw, RPI_FIRMWARE_NOTIFY_XHCI_RESET,
    &dev_addr, sizeof(dev_addr));
    if (ret)
    return ret;
// Wait for vl805 to startup
    usleep_range(200, 1000);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static const struct reset_control_ops rpi_reset_ops = {
    .reset	= rpi_reset_reset,
    };
#[no_mangle]
unsafe extern "C" fn rpi_reset_probe(pdev: *mut platform_device) -> c_int {
    static int rpi_reset_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct rpi_firmware *fw;
    struct device_node *np;
    struct rpi_reset *priv;
    np = of_get_parent(dev.of_node);
    if (!np) {
    dev_err(dev, "Missing firmware node\n");
    return -ENOENT;
    }
    fw = devm_rpi_firmware_get(&pdev.dev, np);
    of_node_put(np);
    if (!fw)
    return -EPROBE_DEFER;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    dev_set_drvdata(dev, priv);
    priv.fw = fw;
    priv.rcdev.owner = THIS_MODULE;
    priv.rcdev.nr_resets = RASPBERRYPI_FIRMWARE_RESET_NUM_IDS;
    priv.rcdev.ops = &rpi_reset_ops;
    priv.rcdev.of_node = dev.of_node;
    return devm_reset_controller_register(dev, &priv.rcdev);
    }
    static const struct of_device_id rpi_reset_of_match[] = {
    { .compatible = "raspberrypi,firmware-reset" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, rpi_reset_of_match);
    static struct platform_driver rpi_reset_driver = {
    .probe	= rpi_reset_probe,
    .driver	= {
    .name = "raspberrypi-reset",
    .of_match_table = rpi_reset_of_match,
    },
    };
    module_platform_driver(rpi_reset_driver);
    MODULE_AUTHOR("Nicolas Saenz Julienne <nsaenzjulienne@suse.de>");
    MODULE_DESCRIPTION("Raspberry Pi 4 firmware reset driver");
    MODULE_LICENSE("GPL");
