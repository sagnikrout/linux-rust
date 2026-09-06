//! Automatically rewritten from C to Rust
//! Source: drivers/misc/qcom-coincell.c
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
// Copyright (c) 2013, The Linux Foundation. All rights reserved.
// Copyright (c) 2015, Sony Mobile Communications Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_coincell {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub base_addr: u32,
}

pub const QCOM_COINCELL_REG_RSET: c_uint = 0x44;
pub const QCOM_COINCELL_REG_VSET: c_uint = 0x45;
pub const QCOM_COINCELL_REG_ENABLE: c_uint = 0x46;

    static const int qcom_rset_map[] = { 2100, 1700, 1200, 800 };
    static const int qcom_vset_map[] = { 2500, 3200, 3100, 3000 };
// NOTE: for pm8921 and others, voltage of 2500 is 16 (10000b), not 0
// if enable==0, rset and vset are ignored
    static int qcom_coincell_chgr_config(struct qcom_coincell *chgr, int rset,
    int vset, bool enable)
    {
    int i, j, rc;
// if disabling, just do that and skip other operations
    if (!enable)
    return regmap_write(chgr.regmap,
    chgr.base_addr + QCOM_COINCELL_REG_ENABLE, 0);
// find index for current-limiting resistor
    for (i = 0; i < ARRAY_SIZE(qcom_rset_map); i++)
    if (rset == qcom_rset_map[i])
    break;
    if (i >= ARRAY_SIZE(qcom_rset_map)) {
    dev_err(chgr.dev, "invalid rset-ohms value %d\n", rset);
    return -EINVAL;
    }
// find index for charge voltage
    for (j = 0; j < ARRAY_SIZE(qcom_vset_map); j++)
    if (vset == qcom_vset_map[j])
    break;
    if (j >= ARRAY_SIZE(qcom_vset_map)) {
    dev_err(chgr.dev, "invalid vset-millivolts value %d\n", vset);
    return -EINVAL;
    }
    rc = regmap_write(chgr.regmap,
    chgr.base_addr + QCOM_COINCELL_REG_RSET, i);
    if (rc) {
//
// This is mainly to flag a bad base_addr (reg) from dts.
// Other failures writing to the registers should be
// extremely rare, or indicative of problems that
// should be reported elsewhere (eg. spmi failure).
//
    dev_err(chgr.dev, "could not write to RSET register\n");
    return rc;
    }
    rc = regmap_write(chgr.regmap,
    chgr.base_addr + QCOM_COINCELL_REG_VSET, j);
    if (rc)
    return rc;
// set 'enable' register
    return regmap_write(chgr.regmap,
    chgr.base_addr + QCOM_COINCELL_REG_ENABLE,
    QCOM_COINCELL_ENABLE);
    }
#[no_mangle]
unsafe extern "C" fn qcom_coincell_probe(pdev: *mut platform_device) -> c_int {
    static int qcom_coincell_probe(struct platform_device *pdev)
    {
    struct device_node *node = pdev.dev.of_node;
    struct qcom_coincell chgr;
    let mut rset: u32 = 0;
    let mut vset: u32 = 0;
    bool enable;
    int rc;
    chgr.dev = &pdev.dev;
    chgr.regmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    if (!chgr.regmap) {
    dev_err(chgr.dev, "Unable to get regmap\n");
    return -EINVAL;
    }
    rc = of_property_read_u32(node, "reg", &chgr.base_addr);
    if (rc)
    return rc;
    enable = !of_property_read_bool(node, "qcom,charger-disable");
    if (enable) {
    rc = of_property_read_u32(node, "qcom,rset-ohms", &rset);
    if (rc) {
    dev_err(chgr.dev,
    "can't find 'qcom,rset-ohms' in DT block");
    return rc;
    }
    rc = of_property_read_u32(node, "qcom,vset-millivolts", &vset);
    if (rc) {
    dev_err(chgr.dev,
    "can't find 'qcom,vset-millivolts' in DT block");
    return rc;
    }
    }
    return qcom_coincell_chgr_config(&chgr, rset, vset, enable);
    }
    static const struct of_device_id qcom_coincell_match_table[] = {
    { .compatible = "qcom,pm8941-coincell" },
    { }
    };
    MODULE_DEVICE_TABLE(of, qcom_coincell_match_table);
    static struct platform_driver qcom_coincell_driver = {
    .driver	= {
    .name		= "qcom-spmi-coincell",
    .of_match_table	= qcom_coincell_match_table,
    },
    .probe		= qcom_coincell_probe,
    };
    module_platform_driver(qcom_coincell_driver);
    MODULE_DESCRIPTION("Qualcomm PMIC coincell charger driver");
    MODULE_LICENSE("GPL v2");
