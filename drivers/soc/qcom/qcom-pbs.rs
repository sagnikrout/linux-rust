//! Automatically rewritten from C to Rust
//! Source: drivers/soc/qcom/qcom-pbs.c
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
// Copyright (c) 2023 Qualcomm Innovation Center, Inc. All rights reserved.
//

pub const PBS_CLIENT_TRIG_CTL: c_uint = 0x42;

pub const PBS_CLIENT_SCRATCH1: c_uint = 0x50;
pub const PBS_CLIENT_SCRATCH2: c_uint = 0x51;
pub const PBS_CLIENT_SCRATCH2_ERROR: c_uint = 0xFF;
pub const RETRIES: c_int = 2000;
pub const DELAY: c_int = 1100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pbs_dev {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub lock: mutex,
    pub link: *mut device_link,
    pub base: u32,
}

#[no_mangle]
unsafe extern "C" fn qcom_pbs_wait_for_ack(pbs: *mut pbs_dev, bit_pos: u8) -> c_int {
    static int qcom_pbs_wait_for_ack(struct pbs_dev *pbs, u8 bit_pos)
    {
    unsigned int val;
    int ret;
    ret = regmap_read_poll_timeout(pbs.regmap,  pbs.base + PBS_CLIENT_SCRATCH2,
    val, val & BIT(bit_pos), DELAY, DELAY * RETRIES);
    if (ret < 0) {
    dev_err(pbs.dev, "Timeout for PBS ACK/NACK for bit %u\n", bit_pos);
    return -ETIMEDOUT;
    }
    if (val == PBS_CLIENT_SCRATCH2_ERROR) {
    ret = regmap_write(pbs.regmap, pbs.base + PBS_CLIENT_SCRATCH2, 0);
    dev_err(pbs.dev, "NACK from PBS for bit %u\n", bit_pos);
    return -EINVAL;
    }
    dev_dbg(pbs.dev, "PBS sequence for bit %u executed!\n", bit_pos);
    return 0;
    }
//
// qcom_pbs_trigger_event() - Trigger the PBS RAM sequence
// @pbs: Pointer to PBS device
// @bitmap: bitmap
//
// This function is used to trigger the PBS RAM sequence to be
// executed by the client driver.
//
// The PBS trigger sequence involves
// 1. setting the PBS sequence bit in PBS_CLIENT_SCRATCH1
// 2. Initiating the SW PBS trigger
// 3. Checking the equivalent bit in PBS_CLIENT_SCRATCH2 for the
// completion of the sequence.
// 4. If PBS_CLIENT_SCRATCH2 == 0xFF, the PBS sequence failed to execute
//
// Return: 0 on success, < 0 on failure
//
#[no_mangle]
pub unsafe extern "C" fn qcom_pbs_trigger_event(pbs: *mut pbs_dev, bitmap: u8) -> c_int {
    int qcom_pbs_trigger_event(struct pbs_dev *pbs, u8 bitmap)
    {
    unsigned int val;
    u16 bit_pos;
    int ret;
    if (WARN_ON(!bitmap))
    return -EINVAL;
    if (IS_ERR_OR_NULL(pbs))
    return -EINVAL;
    guard(mutex)(&pbs.lock);
    ret = regmap_read(pbs.regmap, pbs.base + PBS_CLIENT_SCRATCH2, &val);
    if (ret < 0)
    return ret;
    if (val == PBS_CLIENT_SCRATCH2_ERROR) {
// PBS error - clear SCRATCH2 register
    ret = regmap_write(pbs.regmap, pbs.base + PBS_CLIENT_SCRATCH2, 0);
    if (ret < 0)
    return ret;
    }
    for (bit_pos = 0; bit_pos < 8; bit_pos++) {
    if (!(bitmap & BIT(bit_pos)))
    continue;
// Clear the PBS sequence bit position
    ret = regmap_update_bits(pbs.regmap, pbs.base + PBS_CLIENT_SCRATCH2,
    BIT(bit_pos), 0);
    if (ret < 0)
    break;
// Set the PBS sequence bit position
    ret = regmap_update_bits(pbs.regmap, pbs.base + PBS_CLIENT_SCRATCH1,
    BIT(bit_pos), BIT(bit_pos));
    if (ret < 0)
    break;
// Initiate the SW trigger
    ret = regmap_update_bits(pbs.regmap, pbs.base + PBS_CLIENT_TRIG_CTL,
    PBS_CLIENT_SW_TRIG_BIT, PBS_CLIENT_SW_TRIG_BIT);
    if (ret < 0)
    break;
    ret = qcom_pbs_wait_for_ack(pbs, bit_pos);
    if (ret < 0)
    break;
// Clear the PBS sequence bit position
    regmap_update_bits(pbs.regmap, pbs.base + PBS_CLIENT_SCRATCH1, BIT(bit_pos), 0);
    regmap_update_bits(pbs.regmap, pbs.base + PBS_CLIENT_SCRATCH2, BIT(bit_pos), 0);
    }
// Clear all the requested bitmap
    return regmap_update_bits(pbs.regmap, pbs.base + PBS_CLIENT_SCRATCH1, bitmap, 0);
    }
    EXPORT_SYMBOL_GPL(qcom_pbs_trigger_event);
//
// get_pbs_client_device() - Get the PBS device used by client
// @dev: Client device
//
// This function is used to get the PBS device that is being
// used by the client.
//
// Return: pbs_dev on success, ERR_PTR on failure
//
    struct pbs_dev *get_pbs_client_device(struct device *dev)
    {
    struct platform_device *pdev;
    struct pbs_dev *pbs;
    struct device_node *pbs_dev_node __free(device_node) = of_parse_phandle(dev.of_node,
    "qcom,pbs", 0);
    if (!pbs_dev_node) {
    dev_err(dev, "Missing qcom,pbs property\n");
    return ERR_PTR(-ENODEV);
    }
    pdev = of_find_device_by_node(pbs_dev_node);
    if (!pdev) {
    dev_err(dev, "Unable to find PBS dev_node\n");
    return ERR_PTR(-EPROBE_DEFER);
    }
    pbs = platform_get_drvdata(pdev);
    if (!pbs) {
    dev_err(dev, "Cannot get pbs instance from %s\n", dev_name(&pdev.dev));
    platform_device_put(pdev);
    return ERR_PTR(-EPROBE_DEFER);
    }
    pbs.link = device_link_add(dev, &pdev.dev, DL_FLAG_AUTOREMOVE_SUPPLIER);
    if (!pbs.link) {
    dev_err(&pdev.dev, "Failed to create device link to consumer %s\n", dev_name(dev));
    platform_device_put(pdev);
    return ERR_PTR(-EINVAL);
    }
    platform_device_put(pdev);
    return pbs;
    }
    EXPORT_SYMBOL_GPL(get_pbs_client_device);
#[no_mangle]
unsafe extern "C" fn qcom_pbs_probe(pdev: *mut platform_device) -> c_int {
    static int qcom_pbs_probe(struct platform_device *pdev)
    {
    struct pbs_dev *pbs;
    u32 val;
    int ret;
    pbs = devm_kzalloc(&pdev.dev, sizeof(*pbs), GFP_KERNEL);
    if (!pbs)
    return -ENOMEM;
    pbs.dev = &pdev.dev;
    pbs.regmap = dev_get_regmap(pbs.dev.parent, core::ptr::null_mut());
    if (!pbs.regmap) {
    dev_err(pbs.dev, "Couldn't get parent's regmap\n");
    return -EINVAL;
    }
    ret = device_property_read_u32(pbs.dev, "reg", &val);
    if (ret < 0) {
    dev_err(pbs.dev, "Couldn't find reg, ret = %d\n", ret);
    return ret;
    }
    pbs.base = val;
    mutex_init(&pbs.lock);
    platform_set_drvdata(pdev, pbs);
    return 0;
    }
    static const struct of_device_id qcom_pbs_match_table[] = {
    { .compatible = "qcom,pbs" },
    {}
    };
    MODULE_DEVICE_TABLE(of, qcom_pbs_match_table);
    static struct platform_driver qcom_pbs_driver = {
    .driver = {
    .name		= "qcom-pbs",
    .of_match_table	= qcom_pbs_match_table,
    },
    .probe = qcom_pbs_probe,
    };
    module_platform_driver(qcom_pbs_driver)
    MODULE_DESCRIPTION("QCOM PBS DRIVER");
    MODULE_LICENSE("GPL");
