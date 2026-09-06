//! Automatically rewritten from C to Rust
//! Source: drivers/reset/reset-qcom-aoss.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_aoss_reset_map {
    pub reg: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_aoss_desc {
    pub resets: *const qcom_aoss_reset_map,
    pub num_resets: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_aoss_reset_data {
    pub rcdev: reset_controller_dev,
    pub base: *mut void __iomem,
    pub desc: *const qcom_aoss_desc,
}

    static const struct qcom_aoss_reset_map sdm845_aoss_resets[] = {
    [AOSS_CC_MSS_RESTART] = {0x10000},
    [AOSS_CC_CAMSS_RESTART] = {0x11000},
    [AOSS_CC_VENUS_RESTART] = {0x12000},
    [AOSS_CC_GPU_RESTART] = {0x13000},
    [AOSS_CC_DISPSS_RESTART] = {0x14000},
    [AOSS_CC_WCSS_RESTART] = {0x20000},
    [AOSS_CC_LPASS_RESTART] = {0x30000},
    };
    static const struct qcom_aoss_desc sdm845_aoss_desc = {
    .resets = sdm845_aoss_resets,
    .num_resets = ARRAY_SIZE(sdm845_aoss_resets),
    };
    static inline struct qcom_aoss_reset_data *to_qcom_aoss_reset_data(
    struct reset_controller_dev *rcdev)
    {
    return container_of(rcdev, struct qcom_aoss_reset_data, rcdev);
    }
    static int qcom_aoss_control_assert(struct reset_controller_dev *rcdev,
    unsigned long idx)
    {
    struct qcom_aoss_reset_data *data = to_qcom_aoss_reset_data(rcdev);
    const struct qcom_aoss_reset_map *map = &data.desc.resets[idx];
    writel(1, data.base + map.reg);
// Wait 6 32kHz sleep cycles for reset
    usleep_range(200, 300);
    return 0;
    }
    static int qcom_aoss_control_deassert(struct reset_controller_dev *rcdev,
    unsigned long idx)
    {
    struct qcom_aoss_reset_data *data = to_qcom_aoss_reset_data(rcdev);
    const struct qcom_aoss_reset_map *map = &data.desc.resets[idx];
    writel(0, data.base + map.reg);
// Wait 6 32kHz sleep cycles for reset
    usleep_range(200, 300);
    return 0;
    }
    static int qcom_aoss_control_reset(struct reset_controller_dev *rcdev,
    unsigned long idx)
    {
    qcom_aoss_control_assert(rcdev, idx);
    return qcom_aoss_control_deassert(rcdev, idx);
    }
    static const struct reset_control_ops qcom_aoss_reset_ops = {
    .reset = qcom_aoss_control_reset,
    .assert = qcom_aoss_control_assert,
    .deassert = qcom_aoss_control_deassert,
    };
#[no_mangle]
unsafe extern "C" fn qcom_aoss_reset_probe(pdev: *mut platform_device) -> c_int {
    static int qcom_aoss_reset_probe(struct platform_device *pdev)
    {
    struct qcom_aoss_reset_data *data;
    struct device *dev = &pdev.dev;
    const struct qcom_aoss_desc *desc;
    desc = of_device_get_match_data(dev);
    if (!desc)
    return -EINVAL;
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.desc = desc;
    data.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(data.base))
    return PTR_ERR(data.base);
    data.rcdev.owner = THIS_MODULE;
    data.rcdev.ops = &qcom_aoss_reset_ops;
    data.rcdev.nr_resets = desc.num_resets;
    data.rcdev.of_node = dev.of_node;
    return devm_reset_controller_register(dev, &data.rcdev);
    }
    static const struct of_device_id qcom_aoss_reset_of_match[] = {
    { .compatible = "qcom,sdm845-aoss-cc", .data = &sdm845_aoss_desc },
    {}
    };
    MODULE_DEVICE_TABLE(of, qcom_aoss_reset_of_match);
    static struct platform_driver qcom_aoss_reset_driver = {
    .probe = qcom_aoss_reset_probe,
    .driver  = {
    .name = "qcom_aoss_reset",
    .of_match_table = qcom_aoss_reset_of_match,
    },
    };
    module_platform_driver(qcom_aoss_reset_driver);
    MODULE_DESCRIPTION("Qualcomm AOSS Reset Driver");
    MODULE_LICENSE("GPL v2");
