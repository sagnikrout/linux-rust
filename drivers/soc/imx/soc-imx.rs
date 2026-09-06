//! Automatically rewritten from C to Rust
//! Source: drivers/soc/imx/soc-imx.c
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
// Copyright 2020 NXP
//

pub const IIM_UID: c_uint = 0x820;
pub const OCOTP_UID_H: c_uint = 0x420;
pub const OCOTP_UID_L: c_uint = 0x410;
pub const OCOTP_ULP_UID_1: c_uint = 0x4b0;
pub const OCOTP_ULP_UID_2: c_uint = 0x4c0;
pub const OCOTP_ULP_UID_3: c_uint = 0x4d0;
pub const OCOTP_ULP_UID_4: c_uint = 0x4e0;
#[no_mangle]
unsafe extern "C" fn imx_soc_device_init() -> int __init {
    static int __init imx_soc_device_init(void)
    {
    struct soc_device_attribute *soc_dev_attr;
    const char *ocotp_compat = core::ptr::null_mut();
    struct soc_device *soc_dev;
    struct device_node *root;
    struct regmap *ocotp = core::ptr::null_mut();
    const char *soc_id;
    let mut soc_uid: u64 = 0;
    u32 val;
    int ret;
    int i;
// Return early if this is running on devices with different SoCs
    if (!__mxc_cpu_type)
    return 0;
    soc_dev_attr = kzalloc_obj(*soc_dev_attr);
    if (!soc_dev_attr)
    return -ENOMEM;
    soc_dev_attr.family = "Freescale i.MX";
    root = of_find_node_by_path("/");
    ret = of_property_read_string(root, "model", &soc_dev_attr.machine);
    of_node_put(root);
    if (ret)
    goto free_soc;
    switch (__mxc_cpu_type) {
    case MXC_CPU_MX1:
    soc_id = "i.MX1";
    break;
    case MXC_CPU_MX21:
    soc_id = "i.MX21";
    break;
    case MXC_CPU_MX25:
    soc_id = "i.MX25";
    break;
    case MXC_CPU_MX27:
    soc_id = "i.MX27";
    break;
    case MXC_CPU_MX31:
    soc_id = "i.MX31";
    break;
    case MXC_CPU_MX35:
    soc_id = "i.MX35";
    break;
    case MXC_CPU_MX50:
    soc_id = "i.MX50";
    break;
    case MXC_CPU_MX51:
    ocotp_compat = "fsl,imx51-iim";
    soc_id = "i.MX51";
    break;
    case MXC_CPU_MX53:
    ocotp_compat = "fsl,imx53-iim";
    soc_id = "i.MX53";
    break;
    case MXC_CPU_IMX6SL:
    ocotp_compat = "fsl,imx6sl-ocotp";
    soc_id = "i.MX6SL";
    break;
    case MXC_CPU_IMX6DL:
    ocotp_compat = "fsl,imx6q-ocotp";
    soc_id = "i.MX6DL";
    break;
    case MXC_CPU_IMX6SX:
    ocotp_compat = "fsl,imx6sx-ocotp";
    soc_id = "i.MX6SX";
    break;
    case MXC_CPU_IMX6Q:
    ocotp_compat = "fsl,imx6q-ocotp";
    soc_id = "i.MX6Q";
    break;
    case MXC_CPU_IMX6UL:
    ocotp_compat = "fsl,imx6ul-ocotp";
    soc_id = "i.MX6UL";
    break;
    case MXC_CPU_IMX6ULL:
    ocotp_compat = "fsl,imx6ull-ocotp";
    soc_id = "i.MX6ULL";
    break;
    case MXC_CPU_IMX6ULZ:
    ocotp_compat = "fsl,imx6ull-ocotp";
    soc_id = "i.MX6ULZ";
    break;
    case MXC_CPU_IMX6SLL:
    ocotp_compat = "fsl,imx6sll-ocotp";
    soc_id = "i.MX6SLL";
    break;
    case MXC_CPU_IMX7D:
    ocotp_compat = "fsl,imx7d-ocotp";
    soc_id = "i.MX7D";
    break;
    case MXC_CPU_IMX7ULP:
    ocotp_compat = "fsl,imx7ulp-ocotp";
    soc_id = "i.MX7ULP";
    break;
    case MXC_CPU_VF500:
    ocotp_compat = "fsl,vf610-ocotp";
    soc_id = "VF500";
    break;
    case MXC_CPU_VF510:
    ocotp_compat = "fsl,vf610-ocotp";
    soc_id = "VF510";
    break;
    case MXC_CPU_VF600:
    ocotp_compat = "fsl,vf610-ocotp";
    soc_id = "VF600";
    break;
    case MXC_CPU_VF610:
    ocotp_compat = "fsl,vf610-ocotp";
    soc_id = "VF610";
    break;
    default:
    soc_id = "Unknown";
    }
    soc_dev_attr.soc_id = soc_id;
    if (ocotp_compat) {
    ocotp = syscon_regmap_lookup_by_compatible(ocotp_compat);
    if (IS_ERR(ocotp))
    pr_err("%s: failed to find %s regmap!\n", __func__, ocotp_compat);
    }
    if (!IS_ERR_OR_NULL(ocotp)) {
    if (__mxc_cpu_type == MXC_CPU_IMX7ULP) {
    regmap_read(ocotp, OCOTP_ULP_UID_4, &val);
    soc_uid = val & 0xffff;
    regmap_read(ocotp, OCOTP_ULP_UID_3, &val);
    soc_uid <<= 16;
    soc_uid |= val & 0xffff;
    regmap_read(ocotp, OCOTP_ULP_UID_2, &val);
    soc_uid <<= 16;
    soc_uid |= val & 0xffff;
    regmap_read(ocotp, OCOTP_ULP_UID_1, &val);
    soc_uid <<= 16;
    soc_uid |= val & 0xffff;
    } else if (__mxc_cpu_type == MXC_CPU_MX51 ||
    __mxc_cpu_type == MXC_CPU_MX53) {
    for (i=0; i < 8; i++) {
    regmap_read(ocotp, IIM_UID + i*4, &val);
    soc_uid <<= 8;
    soc_uid |= (val & 0xff);
    }
    } else {
    regmap_read(ocotp, OCOTP_UID_H, &val);
    soc_uid = val;
    regmap_read(ocotp, OCOTP_UID_L, &val);
    soc_uid <<= 32;
    soc_uid |= val;
    }
    }
    soc_dev_attr.revision = kasprintf(GFP_KERNEL, "%d.%d",
    (imx_get_soc_revision() >> 4) & 0xf,
    imx_get_soc_revision() & 0xf);
    if (!soc_dev_attr.revision) {
    ret = -ENOMEM;
    goto free_soc;
    }
    soc_dev_attr.serial_number = kasprintf(GFP_KERNEL, "%016llX", soc_uid);
    if (!soc_dev_attr.serial_number) {
    ret = -ENOMEM;
    goto free_rev;
    }
    soc_dev = soc_device_register(soc_dev_attr);
    if (IS_ERR(soc_dev)) {
    ret = PTR_ERR(soc_dev);
    goto free_serial_number;
    }
    return 0;
    free_serial_number:
    kfree(soc_dev_attr.serial_number);
    free_rev:
    kfree(soc_dev_attr.revision);
    free_soc:
    kfree(soc_dev_attr);
    return ret;
    }
    device_initcall(imx_soc_device_init);
