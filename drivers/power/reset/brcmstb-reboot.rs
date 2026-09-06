//! Automatically rewritten from C to Rust
//! Source: drivers/power/reset/brcmstb-reboot.c
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
// Copyright (C) 2013 Broadcom Corporation

    static struct regmap *regmap;
    static u32 rst_src_en;
    static u32 sw_mstr_rst;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reset_reg_mask {
    pub rst_src_en_mask: u32,
    pub sw_mstr_rst_mask: u32,
}

    static const struct reset_reg_mask *reset_masks;
#[no_mangle]
unsafe extern "C" fn brcmstb_restart_handler(data: *mut sys_off_data) -> c_int {
    static int brcmstb_restart_handler(struct sys_off_data *data)
    {
    int rc;
    u32 tmp;
    rc = regmap_write(regmap, rst_src_en, reset_masks.rst_src_en_mask);
    if (rc) {
    pr_err("failed to write rst_src_en (%d)\n", rc);
    return NOTIFY_DONE;
    }
    rc = regmap_read(regmap, rst_src_en, &tmp);
    if (rc) {
    pr_err("failed to read rst_src_en (%d)\n", rc);
    return NOTIFY_DONE;
    }
    rc = regmap_write(regmap, sw_mstr_rst, reset_masks.sw_mstr_rst_mask);
    if (rc) {
    pr_err("failed to write sw_mstr_rst (%d)\n", rc);
    return NOTIFY_DONE;
    }
    rc = regmap_read(regmap, sw_mstr_rst, &tmp);
    if (rc) {
    pr_err("failed to read sw_mstr_rst (%d)\n", rc);
    return NOTIFY_DONE;
    }
    return NOTIFY_DONE;
    }
    static const struct reset_reg_mask reset_bits_40nm = {
    .rst_src_en_mask = BIT(0),
    .sw_mstr_rst_mask = BIT(0),
    };
    static const struct reset_reg_mask reset_bits_65nm = {
    .rst_src_en_mask = BIT(3),
    .sw_mstr_rst_mask = BIT(31),
    };
#[no_mangle]
unsafe extern "C" fn brcmstb_reboot_probe(pdev: *mut platform_device) -> c_int {
    static int brcmstb_reboot_probe(struct platform_device *pdev)
    {
    int rc;
    struct device_node *np = pdev.dev.of_node;
    unsigned int args[2];
    reset_masks = device_get_match_data(&pdev.dev);
    if (!reset_masks) {
    pr_err("failed to get match data\n");
    return -EINVAL;
    }
    regmap = syscon_regmap_lookup_by_phandle_args(np, "syscon", ARRAY_SIZE(args), args);
    if (IS_ERR(regmap)) {
    pr_err("failed to get syscon phandle\n");
    return -EINVAL;
    }
    rst_src_en = args[0];
    sw_mstr_rst = args[1];
    rc = devm_register_sys_off_handler(&pdev.dev, SYS_OFF_MODE_RESTART,
    128, brcmstb_restart_handler, core::ptr::null_mut());
    if (rc)
    dev_err(&pdev.dev,
    "cannot register restart handler (err=%d)\n", rc);
    return rc;
    }
    static const struct of_device_id of_match[] = {
    { .compatible = "brcm,brcmstb-reboot", .data = &reset_bits_40nm },
    { .compatible = "brcm,bcm7038-reboot", .data = &reset_bits_65nm },
    {},
    };
    static struct platform_driver brcmstb_reboot_driver = {
    .probe = brcmstb_reboot_probe,
    .driver = {
    .name = "brcmstb-reboot",
    .of_match_table = of_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn brcmstb_reboot_init() -> int __init {
    static int __init brcmstb_reboot_init(void)
    {
    return platform_driver_register(&brcmstb_reboot_driver);
    }
    subsys_initcall(brcmstb_reboot_init);
