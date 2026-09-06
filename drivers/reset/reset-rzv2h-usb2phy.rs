//! Automatically rewritten from C to Rust
//! Source: drivers/reset/reset-rzv2h-usb2phy.c
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
// Renesas RZ/V2H(P) USB2PHY Port reset control driver
//
// Copyright (C) 2025 Renesas Electronics Corporation
//

    static DEFINE_IDA(auxiliary_ids);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzv2h_usb2phy_reset_of_data {
    pub init_seq: *const reg_sequence,
    pub init_nseq: c_uint,
    pub assert_seq: *const reg_sequence,
    pub assert_nseq: c_uint,
    pub deassert_seq: *const reg_sequence,
    pub deassert_nseq: c_uint,
    pub reset_reg: u16,
    pub reset_status_bits: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzv2h_usb2phy_reset_priv {
    pub data: *const rzv2h_usb2phy_reset_of_data,
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub rcdev: reset_controller_dev,
}

    static inline struct rzv2h_usb2phy_reset_priv
// rzv2h_usbphy_rcdev_to_priv(struct reset_controller_dev *rcdev)
    {
    return container_of(rcdev, struct rzv2h_usb2phy_reset_priv, rcdev);
    }
    static int rzv2h_usbphy_reset_assert(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct rzv2h_usb2phy_reset_priv *priv = rzv2h_usbphy_rcdev_to_priv(rcdev);
    return regmap_multi_reg_write(priv.regmap, priv.data.assert_seq,
    priv.data.assert_nseq);
    }
    static int rzv2h_usbphy_reset_deassert(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct rzv2h_usb2phy_reset_priv *priv = rzv2h_usbphy_rcdev_to_priv(rcdev);
    return regmap_multi_reg_write(priv.regmap, priv.data.deassert_seq,
    priv.data.deassert_nseq);
    }
    static int rzv2h_usbphy_reset_status(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct rzv2h_usb2phy_reset_priv *priv = rzv2h_usbphy_rcdev_to_priv(rcdev);
    u32 reg;
    regmap_read(priv.regmap, priv.data.reset_reg, &reg);
    return (reg & priv.data.reset_status_bits) == priv.data.reset_status_bits;
    }
    static const struct reset_control_ops rzv2h_usbphy_reset_ops = {
    .assert = rzv2h_usbphy_reset_assert,
    .deassert = rzv2h_usbphy_reset_deassert,
    .status = rzv2h_usbphy_reset_status,
    };
    static int rzv2h_usb2phy_reset_of_xlate(struct reset_controller_dev *rcdev,
    const struct of_phandle_args *reset_spec)
    {
// No special handling needed, we have only one reset line per device
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rzv2h_usb2phy_reset_ida_free(data: *mut c_void) {
    static void rzv2h_usb2phy_reset_ida_free(void *data)
    {
    struct auxiliary_device *adev = data;
    ida_free(&auxiliary_ids, adev.id);
    }
    static int rzv2h_usb2phy_reset_mux_register(struct device *dev,
    const char *mux_name)
    {
    struct auxiliary_device *adev;
    int id;
    id = ida_alloc(&auxiliary_ids, GFP_KERNEL);
    if (id < 0)
    return id;
    adev = __devm_auxiliary_device_create(dev, dev.driver.name,
    mux_name, core::ptr::null_mut(), id);
    if (!adev) {
    ida_free(&auxiliary_ids, id);
    return -ENOMEM;
    }
    return devm_add_action_or_reset(dev, rzv2h_usb2phy_reset_ida_free, adev);
    }
    static const struct regmap_config rzv2h_usb2phy_reset_regconf = {
    .reg_bits = 32,
    .val_bits = 32,
    .reg_stride = 4,
    .can_sleep = true,
    };
#[no_mangle]
unsafe extern "C" fn rzv2h_usb2phy_reset_pm_runtime_put(data: *mut c_void) {
    static void rzv2h_usb2phy_reset_pm_runtime_put(void *data)
    {
    pm_runtime_put(data);
    }
#[no_mangle]
unsafe extern "C" fn rzv2h_usb2phy_reset_probe(pdev: *mut platform_device) -> c_int {
    static int rzv2h_usb2phy_reset_probe(struct platform_device *pdev)
    {
    const struct rzv2h_usb2phy_reset_of_data *data;
    struct rzv2h_usb2phy_reset_priv *priv;
    struct device *dev = &pdev.dev;
    struct reset_control *rstc;
    void __iomem *base;
    int error;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    data = of_device_get_match_data(dev);
    priv.data = data;
    priv.dev = dev;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    priv.regmap = devm_regmap_init_mmio(dev, base, &rzv2h_usb2phy_reset_regconf);
    if (IS_ERR(priv.regmap))
    return PTR_ERR(priv.regmap);
    rstc = devm_reset_control_get_shared_deasserted(dev, core::ptr::null_mut());
    if (IS_ERR(rstc))
    return dev_err_probe(dev, PTR_ERR(rstc),
    "failed to get deasserted reset\n");
    error = devm_pm_runtime_enable(dev);
    if (error)
    return dev_err_probe(dev, error, "Failed to enable pm_runtime\n");
    error = pm_runtime_resume_and_get(dev);
    if (error)
    return dev_err_probe(dev, error, "pm_runtime_resume_and_get failed\n");
    error = devm_add_action_or_reset(dev, rzv2h_usb2phy_reset_pm_runtime_put,
    dev);
    if (error)
    return dev_err_probe(dev, error, "unable to register cleanup action\n");
    error = regmap_multi_reg_write(priv.regmap, data.init_seq, data.init_nseq);
    if (error)
    return dev_err_probe(dev, error, "failed to initialize PHY registers\n");
    priv.rcdev.ops = &rzv2h_usbphy_reset_ops;
    priv.rcdev.of_reset_n_cells = 0;
    priv.rcdev.nr_resets = 1;
    priv.rcdev.of_xlate = rzv2h_usb2phy_reset_of_xlate;
    priv.rcdev.of_node = dev.of_node;
    priv.rcdev.dev = dev;
    error = devm_reset_controller_register(dev, &priv.rcdev);
    if (error)
    return dev_err_probe(dev, error, "could not register reset controller\n");
    error = rzv2h_usb2phy_reset_mux_register(dev, "vbenctl");
    if (error)
    return dev_err_probe(dev, error, "could not register aux mux\n");
    return 0;
    }
//
// initialization values required to prepare the PHY to receive
// assert and deassert requests.
//
    static const struct reg_sequence rzv2h_init_seq[] = {
    { .reg = 0xc10, .def = 0x67c },
    { .reg = 0xc14, .def = 0x01f },
    { .reg = 0x600, .def = 0x909 },
    };
    static const struct reg_sequence rzv2h_assert_seq[] = {
    { .reg = 0xb04, .def = 0x303 },
    { .reg = 0x000, .def = 0x206, .delay_us = 11 },
    };
    static const struct reg_sequence rzv2h_deassert_seq[] = {
    { .reg = 0x000, .def = 0x200 },
    { .reg = 0xb04, .def = 0x003 },
    { .reg = 0x000, .def = 0x000 },
    };
    static const struct rzv2h_usb2phy_reset_of_data rzv2h_reset_of_data = {
    .init_seq = rzv2h_init_seq,
    .init_nseq = ARRAY_SIZE(rzv2h_init_seq),
    .assert_seq = rzv2h_assert_seq,
    .assert_nseq = ARRAY_SIZE(rzv2h_assert_seq),
    .deassert_seq = rzv2h_deassert_seq,
    .deassert_nseq = ARRAY_SIZE(rzv2h_deassert_seq),
    .reset_reg = 0,
    .reset_status_bits = BIT(2),
    };
    static const struct of_device_id rzv2h_usb2phy_reset_of_match[] = {
    { .compatible = "renesas,r9a09g057-usb2phy-reset", .data = &rzv2h_reset_of_data },
    { /* Sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, rzv2h_usb2phy_reset_of_match);
    static struct platform_driver rzv2h_usb2phy_reset_driver = {
    .driver = {
    .name		= "rzv2h_usb2phy_reset",
    .of_match_table	= rzv2h_usb2phy_reset_of_match,
    },
    .probe = rzv2h_usb2phy_reset_probe,
    };
    module_platform_driver(rzv2h_usb2phy_reset_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Lad Prabhakar <prabhakar.mahadev-lad.rj@bp.renesas.com>");
    MODULE_DESCRIPTION("Renesas RZ/V2H(P) USB2PHY Control");
