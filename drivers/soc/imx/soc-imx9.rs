//! Automatically rewritten from C to Rust
//! Source: drivers/soc/imx/soc-imx9.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright 2024 NXP
//

pub const IMX_SIP_GET_SOC_INFO: c_uint = 0xc2000006;

#[no_mangle]
unsafe extern "C" fn imx9_soc_probe(pdev: *mut platform_device) -> c_int {
    static int imx9_soc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct soc_device_attribute *attr;
    struct arm_smccc_res res;
    struct soc_device *sdev;
    u32 soc_id, rev_major, rev_minor;
    u64 uid127_64, uid63_0;
    int err;
    attr = devm_kzalloc(dev, sizeof(*attr), GFP_KERNEL);
    if (!attr)
    return -ENOMEM;
    err = soc_attr_read_machine(attr);
    if (err)
    return dev_err_probe(dev, err, "%s: missing model property\n", __func__);
    attr.family = devm_kasprintf(dev, GFP_KERNEL, "Freescale i.MX");
    if (!attr.family)
    return -ENOMEM;
//
// Retrieve the soc id, rev & uid info:
// res.a1[31:16]: soc revision;
// res.a1[15:0]: soc id;
// res.a2: uid[127:64];
// res.a3: uid[63:0];
//
    arm_smccc_smc(IMX_SIP_GET_SOC_INFO, 0, 0, 0, 0, 0, 0, 0, &res);
    if (res.a0 != SMCCC_RET_SUCCESS)
    return dev_err_probe(dev, -EINVAL, "%s: SMC failed: 0x%lx\n", __func__, res.a0);
    soc_id = SOC_ID(res.a1);
    rev_major = SOC_REV_MAJOR(res.a1);
    rev_minor = SOC_REV_MINOR(res.a1);
    attr.soc_id = devm_kasprintf(dev, GFP_KERNEL, "i.MX%2x", soc_id);
    if (!attr.soc_id)
    return -ENOMEM;
    attr.revision = devm_kasprintf(dev, GFP_KERNEL, "%d.%d", rev_major, rev_minor);
    if (!attr.revision)
    return -ENOMEM;
    uid127_64 = res.a2;
    uid63_0 = res.a3;
    attr.serial_number = devm_kasprintf(dev, GFP_KERNEL, "%016llx%016llx", uid127_64, uid63_0);
    if (!attr.serial_number)
    return -ENOMEM;
    sdev = soc_device_register(attr);
    if (IS_ERR(sdev))
    return dev_err_probe(dev, PTR_ERR(sdev),
    "%s failed to register SoC as a device\n", __func__);
    return 0;
    }
    static __maybe_unused const struct of_device_id imx9_soc_match[] = {
    { .compatible = "fsl,imx93", },
    { .compatible = "fsl,imx94", },
    { .compatible = "fsl,imx95", },
    { .compatible = "fsl,imx952", },
    { }
    };

    static struct platform_driver imx9_soc_driver = {
    .probe = imx9_soc_probe,
    .driver = {
    .name = IMX_SOC_DRIVER,
    },
    };
#[no_mangle]
unsafe extern "C" fn imx9_soc_init() -> int __init {
    static int __init imx9_soc_init(void)
    {
    int ret;
    struct platform_device *pdev;
// No match means it is not an i.MX 9 series SoC, do nothing.
    if (!of_machine_device_match(imx9_soc_match))
    return 0;
    ret = platform_driver_register(&imx9_soc_driver);
    if (ret) {
    pr_err("failed to register imx9_soc platform driver: %d\n", ret);
    return ret;
    }
    pdev = platform_device_register_simple(IMX_SOC_DRIVER, -1, core::ptr::null_mut(), 0);
    if (IS_ERR(pdev)) {
    pr_err("failed to register imx9_soc platform device: %ld\n", PTR_ERR(pdev));
    platform_driver_unregister(&imx9_soc_driver);
    return PTR_ERR(pdev);
    }
    return 0;
    }
    device_initcall(imx9_soc_init);
    MODULE_AUTHOR("NXP");
    MODULE_DESCRIPTION("NXP i.MX9 SoC");
    MODULE_LICENSE("GPL");
