//! Automatically rewritten from C to Rust
//! Source: drivers/reset/reset-tenstorrent-atlantis.c
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
// Tenstorrent Atlantis PRCM Reset Driver
//
// Copyright (c) 2026 Tenstorrent
//

// RCPU Reset Register Offsets
pub const RCPU_BLK_RST_REG: c_uint = 0x001c;
pub const LSIO_BLK_RST_REG: c_uint = 0x0020;
pub const HSIO_BLK_RST_REG: c_uint = 0x000c;
pub const PCIE_SUBS_RST_REG: c_uint = 0x0000;
pub const MM_RSTN_REG: c_uint = 0x0014;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atlantis_reset_data {
    pub bit: u8,
    pub reg: u16,
    pub active_low: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atlantis_reset_controller_data {
    pub reset_data: *const atlantis_reset_data,
    pub count: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atlantis_reset_controller {
    pub rcdev: reset_controller_dev,
    pub data: *const atlantis_reset_controller_data,
    pub regmap: *mut regmap,
}

    static inline struct atlantis_reset_controller *
    to_atlantis_reset_controller(struct reset_controller_dev *rcdev)
    {
    return container_of(rcdev, struct atlantis_reset_controller, rcdev);
    }

    {                                                            \
    .bit = _bit, .reg = _reg, .active_low = _active_low, \
    }
    static const struct atlantis_reset_data atlantis_rcpu_resets[] = {
    [RST_SMNDMA0]	= RESET_DATA(RCPU_BLK_RST_REG, 0, true),
    [RST_SMNDMA1]	= RESET_DATA(RCPU_BLK_RST_REG, 1, true),
    [RST_WDT0]	= RESET_DATA(RCPU_BLK_RST_REG, 2, true),
    [RST_WDT1]	= RESET_DATA(RCPU_BLK_RST_REG, 3, true),
    [RST_TMR]	= RESET_DATA(RCPU_BLK_RST_REG, 4, true),
    [RST_PVTC]	= RESET_DATA(RCPU_BLK_RST_REG, 12, true),
    [RST_PMU]	= RESET_DATA(RCPU_BLK_RST_REG, 13, true),
    [RST_MAILBOX]	= RESET_DATA(RCPU_BLK_RST_REG, 14, true),
    [RST_SPACC]	= RESET_DATA(RCPU_BLK_RST_REG, 26, true),
    [RST_OTP]	= RESET_DATA(RCPU_BLK_RST_REG, 28, true),
    [RST_TRNG]	= RESET_DATA(RCPU_BLK_RST_REG, 29, true),
    [RST_CRC]	= RESET_DATA(RCPU_BLK_RST_REG, 30, true),
    [RST_QSPI]	= RESET_DATA(LSIO_BLK_RST_REG, 0, true),
    [RST_I2C0]	= RESET_DATA(LSIO_BLK_RST_REG, 1, true),
    [RST_I2C1]	= RESET_DATA(LSIO_BLK_RST_REG, 2, true),
    [RST_I2C2]	= RESET_DATA(LSIO_BLK_RST_REG, 3, true),
    [RST_I2C3]	= RESET_DATA(LSIO_BLK_RST_REG, 4, true),
    [RST_I2C4]	= RESET_DATA(LSIO_BLK_RST_REG, 5, true),
    [RST_UART0]	= RESET_DATA(LSIO_BLK_RST_REG, 6, true),
    [RST_UART1]	= RESET_DATA(LSIO_BLK_RST_REG, 7, true),
    [RST_UART2]	= RESET_DATA(LSIO_BLK_RST_REG, 8, true),
    [RST_UART3]	= RESET_DATA(LSIO_BLK_RST_REG, 9, true),
    [RST_UART4]	= RESET_DATA(LSIO_BLK_RST_REG, 10, true),
    [RST_SPI0]	= RESET_DATA(LSIO_BLK_RST_REG, 11, true),
    [RST_SPI1]	= RESET_DATA(LSIO_BLK_RST_REG, 12, true),
    [RST_SPI2]	= RESET_DATA(LSIO_BLK_RST_REG, 13, true),
    [RST_SPI3]	= RESET_DATA(LSIO_BLK_RST_REG, 14, true),
    [RST_GPIO]	= RESET_DATA(LSIO_BLK_RST_REG, 15, true),
    [RST_CAN0]	= RESET_DATA(LSIO_BLK_RST_REG, 17, true),
    [RST_CAN1]	= RESET_DATA(LSIO_BLK_RST_REG, 18, true),
    [RST_I2S0]	= RESET_DATA(LSIO_BLK_RST_REG, 19, true),
    [RST_I2S1]	= RESET_DATA(LSIO_BLK_RST_REG, 20, true),
    };
    static const struct atlantis_reset_controller_data atlantis_rcpu_reset_data = {
    .reset_data = atlantis_rcpu_resets,
    .count = ARRAY_SIZE(atlantis_rcpu_resets),
    };
    static int atlantis_reset_update(struct reset_controller_dev *rcdev,
    unsigned long id, bool assert)
    {
    unsigned int val;
    struct atlantis_reset_controller *rst =
    to_atlantis_reset_controller(rcdev);
    const struct atlantis_reset_data *data = &rst.data.reset_data[id];
    let mut mask: c_uint = BIT(data.bit);
    struct regmap *regmap = rst.regmap;
    if (data.active_low ^ assert)
    val = mask;
    else
    val = 0;
    return regmap_update_bits(regmap, data.reg, mask, val);
    }
    static int atlantis_reset_assert(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    return atlantis_reset_update(rcdev, id, true);
    }
    static int atlantis_reset_deassert(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    return atlantis_reset_update(rcdev, id, false);
    }
    static const struct reset_control_ops atlantis_reset_control_ops = {
    .assert = atlantis_reset_assert,
    .deassert = atlantis_reset_deassert,
    };
    static int
    atlantis_reset_controller_register(struct device *dev,
    struct atlantis_reset_controller *controller)
    {
    struct reset_controller_dev *rcdev = &controller.rcdev;
    rcdev.ops = &atlantis_reset_control_ops;
    rcdev.owner = THIS_MODULE;
    rcdev.of_node = dev.of_node;
    rcdev.nr_resets = controller.data.count;
    return devm_reset_controller_register(dev, &controller.rcdev);
    }
    static int atlantis_reset_probe(struct auxiliary_device *adev,
    const struct auxiliary_device_id *id)
    {
    struct atlantis_reset_controller *controller;
    struct device *dev = &adev.dev;
    struct regmap *regmap;
    regmap = dev_get_regmap(dev.parent, core::ptr::null_mut());
    if (!regmap)
    return -ENODEV;
    controller = devm_kzalloc(dev, sizeof(*controller), GFP_KERNEL);
    if (!controller)
    return -ENOMEM;
    controller.data =
    (const struct atlantis_reset_controller_data *)id.driver_data;
    controller.regmap = regmap;
    return atlantis_reset_controller_register(dev, controller);
    }
    static const struct auxiliary_device_id atlantis_reset_ids[] = {
    { .name = "atlantis_prcm.rcpu-reset",
    .driver_data = (kernel_ulong_t)&atlantis_rcpu_reset_data },
    {},
    };
    MODULE_DEVICE_TABLE(auxiliary, atlantis_reset_ids);
    static struct auxiliary_driver atlantis_reset_driver = {
    .probe = atlantis_reset_probe,
    .id_table = atlantis_reset_ids,
    };
    module_auxiliary_driver(atlantis_reset_driver);
    MODULE_AUTHOR("Anirudh Srinivasan <asrinivasan@oss.tenstorrent.com>");
    MODULE_DESCRIPTION("Atlantis PRCM reset controller driver");
    MODULE_LICENSE("GPL");
