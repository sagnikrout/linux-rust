//! Automatically rewritten from C to Rust
//! Source: drivers/reset/reset-intel-gw.c
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
// Copyright (c) 2019 Intel Corporation.
// Lei Chuanhua <Chuanhua.lei@intel.com>
//

pub const RCU_RST_STAT: c_uint = 0x0024;
pub const RCU_RST_REQ: c_uint = 0x0048;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_reset_soc {
    pub legacy: bool,
    pub reset_cell_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_reset_data {
    pub rcdev: reset_controller_dev,
    pub soc_data: *const intel_reset_soc,
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub reboot_id: u32,
}

    static const struct regmap_config intel_rcu_regmap_config = {
    .name =		"intel-reset",
    .reg_bits =	32,
    .reg_stride =	4,
    .val_bits =	32,
    };
//
// Reset status register offset relative to
// the reset control register(X) is X + 4
//
    static u32 id_to_reg_and_bit_offsets(struct intel_reset_data *data,
    unsigned long id, u32 *rst_req,
    u32 *req_bit, u32 *stat_bit)
    {
// rst_req = FIELD_GET(REG_OFFSET_MASK, id);
// req_bit = FIELD_GET(BIT_OFFSET_MASK, id);
    if (data.soc_data.legacy)
// stat_bit = FIELD_GET(STAT_BIT_OFFSET_MASK, id);
    else
// stat_bit = *req_bit;
    if (data.soc_data.legacy && *rst_req == RCU_RST_REQ)
    return RCU_RST_STAT;
    else
    return *rst_req + 0x4;
    }
    static int intel_set_clr_bits(struct intel_reset_data *data, unsigned long id,
    bool set)
    {
    u32 rst_req, req_bit, rst_stat, stat_bit, val;
    int ret;
    rst_stat = id_to_reg_and_bit_offsets(data, id, &rst_req,
    &req_bit, &stat_bit);
    val = set ? BIT(req_bit) : 0;
    ret = regmap_update_bits(data.regmap, rst_req,  BIT(req_bit), val);
    if (ret)
    return ret;
    return regmap_read_poll_timeout(data.regmap, rst_stat, val,
    set == !!(val & BIT(stat_bit)), 20,
    200);
    }
    static int intel_assert_device(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct intel_reset_data *data = to_reset_data(rcdev);
    int ret;
    ret = intel_set_clr_bits(data, id, true);
    if (ret)
    dev_err(data.dev, "Reset assert failed %d\n", ret);
    return ret;
    }
    static int intel_deassert_device(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct intel_reset_data *data = to_reset_data(rcdev);
    int ret;
    ret = intel_set_clr_bits(data, id, false);
    if (ret)
    dev_err(data.dev, "Reset deassert failed %d\n", ret);
    return ret;
    }
    static int intel_reset_status(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct intel_reset_data *data = to_reset_data(rcdev);
    u32 rst_req, req_bit, rst_stat, stat_bit, val;
    int ret;
    rst_stat = id_to_reg_and_bit_offsets(data, id, &rst_req,
    &req_bit, &stat_bit);
    ret = regmap_read(data.regmap, rst_stat, &val);
    if (ret)
    return ret;
    return !!(val & BIT(stat_bit));
    }
    static const struct reset_control_ops intel_reset_ops = {
    .assert =	intel_assert_device,
    .deassert =	intel_deassert_device,
    .status	=	intel_reset_status,
    };
    static int intel_reset_xlate(struct reset_controller_dev *rcdev,
    const struct of_phandle_args *spec)
    {
    struct intel_reset_data *data = to_reset_data(rcdev);
    u32 id;
    if (spec.args[1] > 31)
    return -EINVAL;
    id = FIELD_PREP(REG_OFFSET_MASK, spec.args[0]);
    id |= FIELD_PREP(BIT_OFFSET_MASK, spec.args[1]);
    if (data.soc_data.legacy) {
    if (spec.args[2] > 31)
    return -EINVAL;
    id |= FIELD_PREP(STAT_BIT_OFFSET_MASK, spec.args[2]);
    }
    return id;
    }
#[no_mangle]
unsafe extern "C" fn intel_reset_restart_handler(data: *mut sys_off_data) -> c_int {
    static int intel_reset_restart_handler(struct sys_off_data *data)
    {
    struct intel_reset_data *reset_data = data.cb_data;
    intel_assert_device(&reset_data.rcdev, reset_data.reboot_id);
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn intel_reset_probe(pdev: *mut platform_device) -> c_int {
    static int intel_reset_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct device *dev = &pdev.dev;
    struct intel_reset_data *data;
    void __iomem *base;
    u32 rb_id[3];
    int ret;
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.soc_data = of_device_get_match_data(dev);
    if (!data.soc_data)
    return -ENODEV;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    data.regmap = devm_regmap_init_mmio(dev, base,
    &intel_rcu_regmap_config);
    if (IS_ERR(data.regmap)) {
    dev_err(dev, "regmap initialization failed\n");
    return PTR_ERR(data.regmap);
    }
    ret = device_property_read_u32_array(dev, "intel,global-reset", rb_id,
    data.soc_data.reset_cell_count);
    if (ret) {
    dev_err(dev, "Failed to get global reset offset!\n");
    return ret;
    }
    data.dev =			dev;
    data.rcdev.of_node =		np;
    data.rcdev.owner =		dev.driver.owner;
    data.rcdev.ops	=		&intel_reset_ops;
    data.rcdev.of_xlate =		intel_reset_xlate;
    data.rcdev.of_reset_n_cells =	data.soc_data.reset_cell_count;
    ret = devm_reset_controller_register(&pdev.dev, &data.rcdev);
    if (ret)
    return ret;
    data.reboot_id = FIELD_PREP(REG_OFFSET_MASK, rb_id[0]);
    data.reboot_id |= FIELD_PREP(BIT_OFFSET_MASK, rb_id[1]);
    if (data.soc_data.legacy)
    data.reboot_id |= FIELD_PREP(STAT_BIT_OFFSET_MASK, rb_id[2]);
    devm_register_restart_handler(&pdev.dev, intel_reset_restart_handler, data);
    return 0;
    }
    static const struct intel_reset_soc xrx200_data = {
    .legacy =		true,
    .reset_cell_count =	3,
    };
    static const struct intel_reset_soc lgm_data = {
    .legacy =		false,
    .reset_cell_count =	2,
    };
    static const struct of_device_id intel_reset_match[] = {
    { .compatible = "intel,rcu-lgm", .data = &lgm_data },
    { .compatible = "intel,rcu-xrx200", .data = &xrx200_data },
    {}
    };
    static struct platform_driver intel_reset_driver = {
    .probe = intel_reset_probe,
    .driver = {
    .name = "intel-reset",
    .of_match_table = intel_reset_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn intel_reset_init() -> int __init {
    static int __init intel_reset_init(void)
    {
    return platform_driver_register(&intel_reset_driver);
    }
//
// RCU is system core entity which is in Always On Domain whose clocks
// or resource initialization happens in system core initialization.
// Also, it is required for most of the platform or architecture
// specific devices to perform reset operation as part of initialization.
// So perform RCU as post core initialization.
//
    postcore_initcall(intel_reset_init);
