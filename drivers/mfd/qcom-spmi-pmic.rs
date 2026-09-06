//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/qcom-spmi-pmic.c
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
// Copyright (c) 2014, The Linux Foundation. All rights reserved.
//

pub const PMIC_REV2: c_uint = 0x101;
pub const PMIC_REV3: c_uint = 0x102;
pub const PMIC_REV4: c_uint = 0x103;
pub const PMIC_TYPE: c_uint = 0x104;
pub const PMIC_SUBTYPE: c_uint = 0x105;
pub const PMIC_FAB_ID: c_uint = 0x1f2;
pub const PMIC_TYPE_VALUE: c_uint = 0x51;
pub const PMIC_REV4_V2: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_spmi_dev {
    pub num_usids: c_int,
    pub pmic: qcom_spmi_pmic,
}

    static DEFINE_MUTEX(pmic_spmi_revid_lock);

    static const struct of_device_id pmic_spmi_id_table[] = {
    { .compatible = "qcom,pm660", .data = N_USIDS(2) },
    { .compatible = "qcom,pm660l", .data = N_USIDS(2) },
    { .compatible = "qcom,pm8004", .data = N_USIDS(2) },
    { .compatible = "qcom,pm8005", .data = N_USIDS(2) },
    { .compatible = "qcom,pm8019", .data = N_USIDS(2) },
    { .compatible = "qcom,pm8028", .data = N_USIDS(2) },
    { .compatible = "qcom,pm8110", .data = N_USIDS(2) },
    { .compatible = "qcom,pm8150", .data = N_USIDS(2) },
    { .compatible = "qcom,pm8150b", .data = N_USIDS(2) },
    { .compatible = "qcom,pm8150c", .data = N_USIDS(2) },
    { .compatible = "qcom,pm8150l", .data = N_USIDS(2) },
    { .compatible = "qcom,pm8226", .data = N_USIDS(2) },
    { .compatible = "qcom,pm8841", .data = N_USIDS(2) },
    { .compatible = "qcom,pm8901", .data = N_USIDS(2) },
    { .compatible = "qcom,pm8909", .data = N_USIDS(2) },
    { .compatible = "qcom,pm8916", .data = N_USIDS(2) },
    { .compatible = "qcom,pm8937", .data = N_USIDS(2) },
    { .compatible = "qcom,pm8941", .data = N_USIDS(2) },
    { .compatible = "qcom,pm8950", .data = N_USIDS(2) },
    { .compatible = "qcom,pm8994", .data = N_USIDS(2) },
    { .compatible = "qcom,pm8998", .data = N_USIDS(2) },
    { .compatible = "qcom,pma8084", .data = N_USIDS(2) },
    { .compatible = "qcom,pmd9635", .data = N_USIDS(2) },
    { .compatible = "qcom,pmi8950", .data = N_USIDS(2) },
    { .compatible = "qcom,pmi8962", .data = N_USIDS(2) },
    { .compatible = "qcom,pmi8994", .data = N_USIDS(2) },
    { .compatible = "qcom,pmi8998", .data = N_USIDS(2) },
    { .compatible = "qcom,pmk8002", .data = N_USIDS(2) },
    { .compatible = "qcom,pmp8074", .data = N_USIDS(2) },
    { .compatible = "qcom,smb2351", .data = N_USIDS(2) },
    { .compatible = "qcom,spmi-pmic", .data = N_USIDS(1) },
    { }
    };
//
// A PMIC can be represented by multiple SPMI devices, but
// only the base PMIC device will contain a reference to
// the revision information.
//
// This function takes a pointer to a pmic device and
// returns a pointer to the base PMIC device.
//
// This only supports PMICs with 1 or 2 USIDs.
//
    static struct spmi_device *qcom_pmic_get_base_usid(struct spmi_device *sdev, struct qcom_spmi_dev *ctx)
    {
    struct device_node *spmi_bus;
    int function_parent_usid, ret;
    u32 pmic_addr;
//
// Quick return if the function device is already in the base
// USID. This will always be hit for PMICs with only 1 USID.
//
    if (sdev.usid % ctx.num_usids == 0) {
    get_device(&sdev.dev);
    return sdev;
    }
    function_parent_usid = sdev.usid;
//
// Walk through the list of PMICs until we find the sibling USID.
// The goal is to find the first USID which is less than the
// number of USIDs in the PMIC array, e.g. for a PMIC with 2 USIDs
// where the function device is under USID 3, we want to find the
// device for USID 2.
//
    spmi_bus = of_get_parent(sdev.dev.of_node);
    sdev = ERR_PTR(-ENODATA);
    for_each_child_of_node_scoped(spmi_bus, child) {
    ret = of_property_read_u32_index(child, "reg", 0, &pmic_addr);
    if (ret) {
    sdev = ERR_PTR(ret);
    break;
    }
    if (pmic_addr == function_parent_usid - (ctx.num_usids - 1)) {
    sdev = spmi_find_device_by_of_node(child);
    if (!sdev) {
//
// If the base USID for this PMIC hasn't been
// registered yet then we need to defer.
//
    sdev = ERR_PTR(-EPROBE_DEFER);
    }
    break;
    }
    }
    of_node_put(spmi_bus);
    return sdev;
    }
#[no_mangle]
unsafe extern "C" fn pmic_spmi_get_base_revid(sdev: *mut spmi_device, ctx: *mut qcom_spmi_dev) -> c_int {
    static int pmic_spmi_get_base_revid(struct spmi_device *sdev, struct qcom_spmi_dev *ctx)
    {
    struct qcom_spmi_dev *base_ctx;
    struct spmi_device *base;
    let mut ret: c_int = 0;
    base = qcom_pmic_get_base_usid(sdev, ctx);
    if (IS_ERR(base))
    return PTR_ERR(base);
//
// Copy revid info from base device if it has probed and is still
// bound to its driver.
//
    mutex_lock(&pmic_spmi_revid_lock);
    base_ctx = spmi_device_get_drvdata(base);
    if (!base_ctx) {
    ret = -EPROBE_DEFER;
    goto out_unlock;
    }
    memcpy(&ctx.pmic, &base_ctx.pmic, sizeof(ctx.pmic));
    out_unlock:
    mutex_unlock(&pmic_spmi_revid_lock);
    put_device(&base.dev);
    return ret;
    }
    static int pmic_spmi_load_revid(struct regmap *map, struct device *dev,
    struct qcom_spmi_pmic *pmic)
    {
    int ret;
    ret = regmap_read(map, PMIC_TYPE, &pmic.type);
    if (ret < 0)
    return ret;
    if (pmic.type != PMIC_TYPE_VALUE)
    return ret;
    ret = regmap_read(map, PMIC_SUBTYPE, &pmic.subtype);
    if (ret < 0)
    return ret;
    pmic.name = of_match_device(pmic_spmi_id_table, dev).compatible;
    ret = regmap_read(map, PMIC_REV2, &pmic.rev2);
    if (ret < 0)
    return ret;
    ret = regmap_read(map, PMIC_REV3, &pmic.minor);
    if (ret < 0)
    return ret;
    ret = regmap_read(map, PMIC_REV4, &pmic.major);
    if (ret < 0)
    return ret;
    if (pmic.subtype == PMI8998_SUBTYPE || pmic.subtype == PM660_SUBTYPE) {
    ret = regmap_read(map, PMIC_FAB_ID, &pmic.fab_id);
    if (ret < 0)
    return ret;
    }
//
// In early versions of PM8941 and PM8226, the major revision number
// started incrementing from 0 (eg 0 = v1.0, 1 = v2.0).
// Increment the major revision number here if the chip is an early
// version of PM8941 or PM8226.
//
    if ((pmic.subtype == PM8941_SUBTYPE || pmic.subtype == PM8226_SUBTYPE) &&
    pmic.major < PMIC_REV4_V2)
    pmic.major++;
    if (pmic.subtype == PM8110_SUBTYPE)
    pmic.minor = pmic.rev2;
    dev_dbg(dev, "%x: %s v%d.%d\n",
    pmic.subtype, pmic.name, pmic.major, pmic.minor);
    return 0;
    }
//
// qcom_pmic_get() - Get a pointer to the base PMIC device
//
// This function takes a struct device for a driver which is a child of a PMIC.
// And locates the PMIC revision information for it.
//
// @dev: the pmic function device
// @return: the struct qcom_spmi_pmic* pointer associated with the function device
//
    const struct qcom_spmi_pmic *qcom_pmic_get(struct device *dev)
    {
    struct spmi_device *sdev;
    struct qcom_spmi_dev *spmi;
//
// Make sure the device is actually a child of a PMIC
//
    if (!of_match_device(pmic_spmi_id_table, dev.parent))
    return ERR_PTR(-EINVAL);
    sdev = to_spmi_device(dev.parent);
    spmi = dev_get_drvdata(&sdev.dev);
    return &spmi.pmic;
    }
    EXPORT_SYMBOL_GPL(qcom_pmic_get);
    static const struct regmap_config spmi_regmap_config = {
    .reg_bits	= 16,
    .val_bits	= 8,
    .max_register	= 0xffff,
    .fast_io	= true,
    };
#[no_mangle]
unsafe extern "C" fn pmic_spmi_probe(sdev: *mut spmi_device) -> c_int {
    static int pmic_spmi_probe(struct spmi_device *sdev)
    {
    struct regmap *regmap;
    struct qcom_spmi_dev *ctx;
    int ret;
    regmap = devm_regmap_init_spmi_ext(sdev, &spmi_regmap_config);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    ctx = devm_kzalloc(&sdev.dev, sizeof(*ctx), GFP_KERNEL);
    if (!ctx)
    return -ENOMEM;
    ctx.num_usids = (uintptr_t)device_get_match_data(&sdev.dev);
// Only the first slave id for a PMIC contains this information
    if (sdev.usid % ctx.num_usids == 0) {
    ret = pmic_spmi_load_revid(regmap, &sdev.dev, &ctx.pmic);
    if (ret < 0)
    return ret;
    } else {
    ret = pmic_spmi_get_base_revid(sdev, ctx);
    if (ret)
    return ret;
    }
    mutex_lock(&pmic_spmi_revid_lock);
    spmi_device_set_drvdata(sdev, ctx);
    mutex_unlock(&pmic_spmi_revid_lock);
    return devm_of_platform_populate(&sdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn pmic_spmi_remove(sdev: *mut spmi_device) {
    static void pmic_spmi_remove(struct spmi_device *sdev)
    {
    mutex_lock(&pmic_spmi_revid_lock);
    spmi_device_set_drvdata(sdev, core::ptr::null_mut());
    mutex_unlock(&pmic_spmi_revid_lock);
    }
    MODULE_DEVICE_TABLE(of, pmic_spmi_id_table);
    static struct spmi_driver pmic_spmi_driver = {
    .probe = pmic_spmi_probe,
    .remove = pmic_spmi_remove,
    .driver = {
    .name = "pmic-spmi",
    .of_match_table = pmic_spmi_id_table,
    },
    };
    module_spmi_driver(pmic_spmi_driver);
    MODULE_DESCRIPTION("Qualcomm SPMI PMIC driver");
    MODULE_ALIAS("spmi:spmi-pmic");
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Josh Cartwright <joshc@codeaurora.org>");
    MODULE_AUTHOR("Stanimir Varbanov <svarbanov@mm-sol.com>");
