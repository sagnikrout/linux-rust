//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/qcom/lmh.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2021, Linaro Limited. All rights reserved.
//

pub const LMH_NODE_DCVS: c_uint = 0x44435653;
pub const LMH_CLUSTER0_NODE_ID: c_uint = 0x6370302D;
pub const LMH_CLUSTER1_NODE_ID: c_uint = 0x6370312D;
pub const LMH_SUB_FN_THERMAL: c_uint = 0x54484D4C;
pub const LMH_SUB_FN_CRNT: c_uint = 0x43524E54;
pub const LMH_SUB_FN_REL: c_uint = 0x52454C00;
pub const LMH_SUB_FN_BCL: c_uint = 0x42434C00;
pub const LMH_ALGO_MODE_ENABLE: c_uint = 0x454E424C;
pub const LMH_TH_HI_THRESHOLD: c_uint = 0x48494748;
pub const LMH_TH_LOW_THRESHOLD: c_uint = 0x4C4F5700;
pub const LMH_TH_ARM_THRESHOLD: c_uint = 0x41524D00;
pub const LMH_REG_DCVS_INTR_CLR: c_uint = 0x8;
pub const LMH_ENABLE_ALGOS: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lmh_hw_data {
    pub base: *mut void __iomem,
    pub domain: *mut irq_domain,
    pub irq: c_int,
}

#[no_mangle]
unsafe extern "C" fn lmh_handle_irq(hw_irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t lmh_handle_irq(int hw_irq, void *data)
    {
    struct lmh_hw_data *lmh_data = data;
    let mut irq: c_int = irq_find_mapping(lmh_data.domain, 0);
// Call the cpufreq driver to handle the interrupt
    if (irq)
    generic_handle_irq(irq);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn lmh_enable_interrupt(d: *mut irq_data) {
    static void lmh_enable_interrupt(struct irq_data *d)
    {
    struct lmh_hw_data *lmh_data = irq_data_get_irq_chip_data(d);
// Clear the existing interrupt
    writel(0xff, lmh_data.base + LMH_REG_DCVS_INTR_CLR);
    enable_irq(lmh_data.irq);
    }
#[no_mangle]
unsafe extern "C" fn lmh_disable_interrupt(d: *mut irq_data) {
    static void lmh_disable_interrupt(struct irq_data *d)
    {
    struct lmh_hw_data *lmh_data = irq_data_get_irq_chip_data(d);
    disable_irq_nosync(lmh_data.irq);
    }
    static struct irq_chip lmh_irq_chip = {
    .name           = "lmh",
    .irq_enable	= lmh_enable_interrupt,
    .irq_disable	= lmh_disable_interrupt
    };
#[no_mangle]
unsafe extern "C" fn lmh_irq_map(d: *mut irq_domain, irq: c_uint, hw: irq_hw_number_t) -> c_int {
    static int lmh_irq_map(struct irq_domain *d, unsigned int irq, irq_hw_number_t hw)
    {
    struct lmh_hw_data *lmh_data = d.host_data;
    static struct lock_class_key lmh_lock_key;
    static struct lock_class_key lmh_request_key;
//
// This lock class tells lockdep that GPIO irqs are in a different
// category than their parents, so it won't report false recursion.
//
    irq_set_lockdep_class(irq, &lmh_lock_key, &lmh_request_key);
    irq_set_chip_and_handler(irq, &lmh_irq_chip, handle_simple_irq);
    irq_set_chip_data(irq, lmh_data);
    return 0;
    }
    static const struct irq_domain_ops lmh_irq_ops = {
    .map = lmh_irq_map,
    .xlate = irq_domain_xlate_onecell,
    };
#[no_mangle]
unsafe extern "C" fn lmh_probe(pdev: *mut platform_device) -> c_int {
    static int lmh_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct device_node *cpu_node;
    struct lmh_hw_data *lmh_data;
    int temp_low, temp_high, temp_arm, cpu_id, ret;
    unsigned int enable_alg;
    u32 node_id;
    if (!qcom_scm_is_available())
    return -EPROBE_DEFER;
    lmh_data = devm_kzalloc(dev, sizeof(*lmh_data), GFP_KERNEL);
    if (!lmh_data)
    return -ENOMEM;
    lmh_data.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(lmh_data.base))
    return PTR_ERR(lmh_data.base);
    cpu_node = of_parse_phandle(np, "cpus", 0);
    if (!cpu_node)
    return -EINVAL;
    cpu_id = of_cpu_node_to_id(cpu_node);
    of_node_put(cpu_node);
    ret = of_property_read_u32(np, "qcom,lmh-temp-high-millicelsius", &temp_high);
    if (ret) {
    dev_err(dev, "missing qcom,lmh-temp-high-millicelsius property\n");
    return ret;
    }
    ret = of_property_read_u32(np, "qcom,lmh-temp-low-millicelsius", &temp_low);
    if (ret) {
    dev_err(dev, "missing qcom,lmh-temp-low-millicelsius property\n");
    return ret;
    }
    ret = of_property_read_u32(np, "qcom,lmh-temp-arm-millicelsius", &temp_arm);
    if (ret) {
    dev_err(dev, "missing qcom,lmh-temp-arm-millicelsius property\n");
    return ret;
    }
//
// Only sdm845 has lmh hardware currently enabled from hlos. If this is needed
// for other platforms, revisit this to check if the <cpu-id, node-id> should be part
// of a dt match table.
//
    if (cpu_id == 0) {
    node_id = LMH_CLUSTER0_NODE_ID;
    } else if (cpu_id == 4) {
    node_id = LMH_CLUSTER1_NODE_ID;
    } else {
    dev_err(dev, "Wrong CPU id associated with LMh node\n");
    return -EINVAL;
    }
    if (!qcom_scm_lmh_dcvsh_available())
    return -EINVAL;
    enable_alg = (uintptr_t)of_device_get_match_data(dev);
    if (enable_alg) {
    ret = qcom_scm_lmh_dcvsh(LMH_SUB_FN_CRNT, LMH_ALGO_MODE_ENABLE, 1,
    LMH_NODE_DCVS, node_id, 0);
    if (ret)
    dev_err(dev, "Error %d enabling current subfunction\n", ret);
    ret = qcom_scm_lmh_dcvsh(LMH_SUB_FN_REL, LMH_ALGO_MODE_ENABLE, 1,
    LMH_NODE_DCVS, node_id, 0);
    if (ret)
    dev_err(dev, "Error %d enabling reliability subfunction\n", ret);
    ret = qcom_scm_lmh_dcvsh(LMH_SUB_FN_BCL, LMH_ALGO_MODE_ENABLE, 1,
    LMH_NODE_DCVS, node_id, 0);
    if (ret)
    dev_err(dev, "Error %d enabling BCL subfunction\n", ret);
    ret = qcom_scm_lmh_dcvsh(LMH_SUB_FN_THERMAL, LMH_ALGO_MODE_ENABLE, 1,
    LMH_NODE_DCVS, node_id, 0);
    if (ret) {
    dev_err(dev, "Error %d enabling thermal subfunction\n", ret);
    return ret;
    }
    ret = qcom_scm_lmh_profile_change(0x1);
    if (ret) {
    dev_err(dev, "Error %d changing profile\n", ret);
    return ret;
    }
    }
// Set default thermal trips
    ret = qcom_scm_lmh_dcvsh(LMH_SUB_FN_THERMAL, LMH_TH_ARM_THRESHOLD, temp_arm,
    LMH_NODE_DCVS, node_id, 0);
    if (ret) {
    dev_err(dev, "Error setting thermal ARM threshold%d\n", ret);
    return ret;
    }
    ret = qcom_scm_lmh_dcvsh(LMH_SUB_FN_THERMAL, LMH_TH_HI_THRESHOLD, temp_high,
    LMH_NODE_DCVS, node_id, 0);
    if (ret) {
    dev_err(dev, "Error setting thermal HI threshold%d\n", ret);
    return ret;
    }
    ret = qcom_scm_lmh_dcvsh(LMH_SUB_FN_THERMAL, LMH_TH_LOW_THRESHOLD, temp_low,
    LMH_NODE_DCVS, node_id, 0);
    if (ret) {
    dev_err(dev, "Error setting thermal LOW threshold%d\n", ret);
    return ret;
    }
    lmh_data.irq = platform_get_irq(pdev, 0);
    lmh_data.domain = irq_domain_create_linear(dev_fwnode(dev), 1, &lmh_irq_ops, lmh_data);
    if (!lmh_data.domain) {
    dev_err(dev, "Error adding irq_domain\n");
    return -EINVAL;
    }
// Disable the irq and let cpufreq enable it when ready to handle the interrupt
    irq_set_status_flags(lmh_data.irq, IRQ_NOAUTOEN);
    ret = devm_request_irq(dev, lmh_data.irq, lmh_handle_irq,
    IRQF_NO_THREAD | IRQF_NO_SUSPEND,
    "lmh-irq", lmh_data);
    if (ret) {
    irq_domain_remove(lmh_data.domain);
    return ret;
    }
    return 0;
    }
    static const struct of_device_id lmh_table[] = {
    { .compatible = "qcom,sc8180x-lmh", },
    { .compatible = "qcom,sdm845-lmh", .data = (void *)LMH_ENABLE_ALGOS},
    { .compatible = "qcom,sm8150-lmh", },
    {}
    };
    MODULE_DEVICE_TABLE(of, lmh_table);
    static struct platform_driver lmh_driver = {
    .probe = lmh_probe,
    .driver = {
    .name = "qcom-lmh",
    .of_match_table = lmh_table,
    .suppress_bind_attrs = true,
    },
    };
    module_platform_driver(lmh_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("QCOM LMh driver");
