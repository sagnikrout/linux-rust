//! Automatically rewritten from C to Rust
//! Source: drivers/power/reset/syscon-reboot-mode.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (c) 2016, Fuzhou Rockchip Electronics Co., Ltd
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct syscon_reboot_mode {
    pub map: *mut regmap,
    pub reboot: reboot_mode_driver,
    pub offset: u32,
    pub mask: u32,
}

    static int syscon_reboot_mode_write(struct reboot_mode_driver *reboot,
    unsigned int magic)
    {
    struct syscon_reboot_mode *syscon_rbm;
    int ret;
    syscon_rbm = container_of(reboot, struct syscon_reboot_mode, reboot);
    ret = regmap_update_bits(syscon_rbm.map, syscon_rbm.offset,
    syscon_rbm.mask, magic);
    if (ret < 0)
    dev_err(reboot.dev, "update reboot mode bits failed\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn syscon_reboot_mode_probe(pdev: *mut platform_device) -> c_int {
    static int syscon_reboot_mode_probe(struct platform_device *pdev)
    {
    int ret;
    struct syscon_reboot_mode *syscon_rbm;
    syscon_rbm = devm_kzalloc(&pdev.dev, sizeof(*syscon_rbm), GFP_KERNEL);
    if (!syscon_rbm)
    return -ENOMEM;
    syscon_rbm.reboot.dev = &pdev.dev;
    syscon_rbm.reboot.write = syscon_reboot_mode_write;
    syscon_rbm.mask = 0xffffffff;
    syscon_rbm.map = syscon_node_to_regmap(pdev.dev.parent.of_node);
    if (IS_ERR(syscon_rbm.map))
    return PTR_ERR(syscon_rbm.map);
    if (of_property_read_u32(pdev.dev.of_node, "offset",
    &syscon_rbm.offset))
    return -EINVAL;
    of_property_read_u32(pdev.dev.of_node, "mask", &syscon_rbm.mask);
    ret = devm_reboot_mode_register(&pdev.dev, &syscon_rbm.reboot);
    if (ret)
    dev_err(&pdev.dev, "can't register reboot mode\n");
    return ret;
    }
    static const struct of_device_id syscon_reboot_mode_of_match[] = {
    { .compatible = "syscon-reboot-mode" },
    {}
    };
    MODULE_DEVICE_TABLE(of, syscon_reboot_mode_of_match);
    static struct platform_driver syscon_reboot_mode_driver = {
    .probe = syscon_reboot_mode_probe,
    .driver = {
    .name = "syscon-reboot-mode",
    .of_match_table = syscon_reboot_mode_of_match,
    },
    };
    module_platform_driver(syscon_reboot_mode_driver);
    MODULE_AUTHOR("Andy Yan <andy.yan@rock-chips.com");
    MODULE_DESCRIPTION("SYSCON reboot mode driver");
    MODULE_LICENSE("GPL v2");
