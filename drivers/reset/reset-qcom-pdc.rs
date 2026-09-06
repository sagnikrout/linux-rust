//! Automatically rewritten from C to Rust
//! Source: drivers/reset/reset-qcom-pdc.c
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
// Copyright (C) 2018 The Linux Foundation. All rights reserved.
//

pub const RPMH_SDM845_PDC_SYNC_RESET: c_uint = 0x100;
pub const RPMH_SC7280_PDC_SYNC_RESET: c_uint = 0x1000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_pdc_reset_map {
    pub bit: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_pdc_reset_desc {
    pub resets: *const qcom_pdc_reset_map,
    pub num_resets: usize,
    pub offset: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_pdc_reset_data {
    pub rcdev: reset_controller_dev,
    pub regmap: *mut regmap,
    pub desc: *const qcom_pdc_reset_desc,
}

    static const struct regmap_config pdc_regmap_config = {
    .name		= "pdc-reset",
    .reg_bits	= 32,
    .reg_stride	= 4,
    .val_bits	= 32,
    .max_register	= 0x20000,
    };
    static const struct qcom_pdc_reset_map sdm845_pdc_resets[] = {
    [PDC_APPS_SYNC_RESET] = {0},
    [PDC_SP_SYNC_RESET] = {1},
    [PDC_AUDIO_SYNC_RESET] = {2},
    [PDC_SENSORS_SYNC_RESET] = {3},
    [PDC_AOP_SYNC_RESET] = {4},
    [PDC_DEBUG_SYNC_RESET] = {5},
    [PDC_GPU_SYNC_RESET] = {6},
    [PDC_DISPLAY_SYNC_RESET] = {7},
    [PDC_COMPUTE_SYNC_RESET] = {8},
    [PDC_MODEM_SYNC_RESET] = {9},
    };
    static const struct qcom_pdc_reset_desc sdm845_pdc_reset_desc = {
    .resets = sdm845_pdc_resets,
    .num_resets = ARRAY_SIZE(sdm845_pdc_resets),
    .offset = RPMH_SDM845_PDC_SYNC_RESET,
    };
    static const struct qcom_pdc_reset_map sc7280_pdc_resets[] = {
    [PDC_APPS_SYNC_RESET] = {0},
    [PDC_SP_SYNC_RESET] = {1},
    [PDC_AUDIO_SYNC_RESET] = {2},
    [PDC_SENSORS_SYNC_RESET] = {3},
    [PDC_AOP_SYNC_RESET] = {4},
    [PDC_DEBUG_SYNC_RESET] = {5},
    [PDC_GPU_SYNC_RESET] = {6},
    [PDC_DISPLAY_SYNC_RESET] = {7},
    [PDC_COMPUTE_SYNC_RESET] = {8},
    [PDC_MODEM_SYNC_RESET] = {9},
    [PDC_WLAN_RF_SYNC_RESET] = {10},
    [PDC_WPSS_SYNC_RESET] = {11},
    };
    static const struct qcom_pdc_reset_desc sc7280_pdc_reset_desc = {
    .resets = sc7280_pdc_resets,
    .num_resets = ARRAY_SIZE(sc7280_pdc_resets),
    .offset = RPMH_SC7280_PDC_SYNC_RESET,
    };
    static inline struct qcom_pdc_reset_data *to_qcom_pdc_reset_data(
    struct reset_controller_dev *rcdev)
    {
    return container_of(rcdev, struct qcom_pdc_reset_data, rcdev);
    }
    static int qcom_pdc_control_assert(struct reset_controller_dev *rcdev,
    unsigned long idx)
    {
    struct qcom_pdc_reset_data *data = to_qcom_pdc_reset_data(rcdev);
    let mut mask: u32 = BIT(data.desc.resets[idx].bit);
    return regmap_update_bits(data.regmap, data.desc.offset, mask, mask);
    }
    static int qcom_pdc_control_deassert(struct reset_controller_dev *rcdev,
    unsigned long idx)
    {
    struct qcom_pdc_reset_data *data = to_qcom_pdc_reset_data(rcdev);
    let mut mask: u32 = BIT(data.desc.resets[idx].bit);
    return regmap_update_bits(data.regmap, data.desc.offset, mask, 0);
    }
    static const struct reset_control_ops qcom_pdc_reset_ops = {
    .assert = qcom_pdc_control_assert,
    .deassert = qcom_pdc_control_deassert,
    };
#[no_mangle]
unsafe extern "C" fn qcom_pdc_reset_probe(pdev: *mut platform_device) -> c_int {
    static int qcom_pdc_reset_probe(struct platform_device *pdev)
    {
    const struct qcom_pdc_reset_desc *desc;
    struct qcom_pdc_reset_data *data;
    struct device *dev = &pdev.dev;
    void __iomem *base;
    desc = device_get_match_data(&pdev.dev);
    if (!desc)
    return -EINVAL;
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.desc = desc;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    data.regmap = devm_regmap_init_mmio(dev, base, &pdc_regmap_config);
    if (IS_ERR(data.regmap)) {
    dev_err(dev, "Unable to initialize regmap\n");
    return PTR_ERR(data.regmap);
    }
    data.rcdev.owner = THIS_MODULE;
    data.rcdev.ops = &qcom_pdc_reset_ops;
    data.rcdev.nr_resets = desc.num_resets;
    data.rcdev.of_node = dev.of_node;
    return devm_reset_controller_register(dev, &data.rcdev);
    }
    static const struct of_device_id qcom_pdc_reset_of_match[] = {
    { .compatible = "qcom,sc7280-pdc-global", .data = &sc7280_pdc_reset_desc },
    { .compatible = "qcom,sdm845-pdc-global", .data = &sdm845_pdc_reset_desc },
    {}
    };
    MODULE_DEVICE_TABLE(of, qcom_pdc_reset_of_match);
    static struct platform_driver qcom_pdc_reset_driver = {
    .probe = qcom_pdc_reset_probe,
    .driver = {
    .name = "qcom_pdc_reset",
    .of_match_table = qcom_pdc_reset_of_match,
    },
    };
    module_platform_driver(qcom_pdc_reset_driver);
    MODULE_DESCRIPTION("Qualcomm PDC Reset Driver");
    MODULE_LICENSE("GPL v2");
