//! Automatically rewritten from C to Rust
//! Source: drivers/soc/bcm/brcmstb/common.c
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
// Copyright © 2014 NVIDIA Corporation
// Copyright © 2015 Broadcom Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmstb_soc_info {
    pub family_id: u32,
    pub product_id: u32,
}

    static struct brcmstb_soc_info *soc_info;
#[no_mangle]
pub unsafe extern "C" fn brcmstb_get_family_id() -> u32 {
    u32 brcmstb_get_family_id(void)
    {
    return soc_info ? soc_info.family_id : 0;
    }
    EXPORT_SYMBOL(brcmstb_get_family_id);
#[no_mangle]
pub unsafe extern "C" fn brcmstb_get_product_id() -> u32 {
    u32 brcmstb_get_product_id(void)
    {
    return soc_info ? soc_info.product_id : 0;
    }
    EXPORT_SYMBOL(brcmstb_get_product_id);
    static const struct of_device_id sun_top_ctrl_match[] = {
    { .compatible = "brcm,bcm7125-sun-top-ctrl", },
    { .compatible = "brcm,bcm7346-sun-top-ctrl", },
    { .compatible = "brcm,bcm7358-sun-top-ctrl", },
    { .compatible = "brcm,bcm7360-sun-top-ctrl", },
    { .compatible = "brcm,bcm7362-sun-top-ctrl", },
    { .compatible = "brcm,bcm7420-sun-top-ctrl", },
    { .compatible = "brcm,bcm7425-sun-top-ctrl", },
    { .compatible = "brcm,bcm7429-sun-top-ctrl", },
    { .compatible = "brcm,bcm7435-sun-top-ctrl", },
    { .compatible = "brcm,brcmstb-sun-top-ctrl", },
    { }
    };
#[no_mangle]
unsafe extern "C" fn brcmstb_soc_device_init() -> int __init {
    static int __init brcmstb_soc_device_init(void)
    {
    struct soc_device_attribute *soc_dev_attr;
    struct device_node *sun_top_ctrl;
    void __iomem *sun_top_ctrl_base;
    struct soc_device *soc_dev;
    let mut ret: c_int = 0;
// We could be on a multi-platform kernel, don't make this fatal but
// bail out early
//
    sun_top_ctrl = of_find_matching_node(core::ptr::null_mut(), sun_top_ctrl_match);
    if (!sun_top_ctrl)
    return 0;
    sun_top_ctrl_base = of_iomap(sun_top_ctrl, 0);
    if (!sun_top_ctrl_base) {
    ret = -ENODEV;
    goto out_put_node;
    }
    soc_info = kzalloc_obj(*soc_info);
    if (!soc_info) {
    ret = -ENOMEM;
    goto out_unmap;
    }
    soc_info.family_id = readl(sun_top_ctrl_base);
    soc_info.product_id = readl(sun_top_ctrl_base + 0x4);
    soc_dev_attr = kzalloc_obj(*soc_dev_attr);
    if (!soc_dev_attr) {
    ret = -ENOMEM;
    goto out_free_info;
    }
    soc_dev_attr.family = kasprintf(GFP_KERNEL, "%x",
    soc_info.family_id >> 28 ?
    soc_info.family_id >> 16 : soc_info.family_id >> 8);
    soc_dev_attr.soc_id = kasprintf(GFP_KERNEL, "%x",
    soc_info.product_id >> 28 ?
    soc_info.product_id >> 16 : soc_info.product_id >> 8);
    soc_dev_attr.revision = kasprintf(GFP_KERNEL, "%c%d",
    ((soc_info.product_id & 0xf0) >> 4) + 'A',
    soc_info.product_id & 0xf);
    soc_dev = soc_device_register(soc_dev_attr);
    if (IS_ERR(soc_dev)) {
    ret = PTR_ERR(soc_dev);
    goto out_free_attr;
    }
    iounmap(sun_top_ctrl_base);
    of_node_put(sun_top_ctrl);
    return 0;
    out_free_attr:
    kfree(soc_dev_attr.revision);
    kfree(soc_dev_attr.soc_id);
    kfree(soc_dev_attr.family);
    kfree(soc_dev_attr);
    out_free_info:
    kfree(soc_info);
    soc_info = core::ptr::null_mut();
    out_unmap:
    iounmap(sun_top_ctrl_base);
    out_put_node:
    of_node_put(sun_top_ctrl);
    return ret;
    }
    early_initcall(brcmstb_soc_device_init);
