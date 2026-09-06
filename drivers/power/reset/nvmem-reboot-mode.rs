//! Automatically rewritten from C to Rust
//! Source: drivers/power/reset/nvmem-reboot-mode.c
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
// Copyright (c) Vaisala Oyj. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmem_reboot_mode {
    pub reboot: reboot_mode_driver,
    pub cell: *mut nvmem_cell,
}

    static int nvmem_reboot_mode_write(struct reboot_mode_driver *reboot,
    unsigned int magic)
    {
    struct nvmem_reboot_mode *nvmem_rbm;
    size_t buf_len;
    void *buf;
    int ret;
    nvmem_rbm = container_of(reboot, struct nvmem_reboot_mode, reboot);
    buf = nvmem_cell_read(nvmem_rbm.cell, &buf_len);
    if (IS_ERR(buf))
    return PTR_ERR(buf);
    kfree(buf);
    if (buf_len > sizeof(magic))
    return -EINVAL;
    ret = nvmem_cell_write(nvmem_rbm.cell, &magic, buf_len);
    if (ret < 0)
    dev_err(reboot.dev, "update reboot mode bits failed\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn nvmem_reboot_mode_probe(pdev: *mut platform_device) -> c_int {
    static int nvmem_reboot_mode_probe(struct platform_device *pdev)
    {
    int ret;
    struct nvmem_reboot_mode *nvmem_rbm;
    nvmem_rbm = devm_kzalloc(&pdev.dev, sizeof(*nvmem_rbm), GFP_KERNEL);
    if (!nvmem_rbm)
    return -ENOMEM;
    nvmem_rbm.reboot.dev = &pdev.dev;
    nvmem_rbm.reboot.write = nvmem_reboot_mode_write;
    nvmem_rbm.cell = devm_nvmem_cell_get(&pdev.dev, "reboot-mode");
    if (IS_ERR(nvmem_rbm.cell)) {
    return dev_err_probe(&pdev.dev, PTR_ERR(nvmem_rbm.cell),
    "failed to get the nvmem cell reboot-mode\n");
    }
    ret = devm_reboot_mode_register(&pdev.dev, &nvmem_rbm.reboot);
    if (ret)
    dev_err(&pdev.dev, "can't register reboot mode\n");
    return ret;
    }
    static const struct of_device_id nvmem_reboot_mode_of_match[] = {
    { .compatible = "nvmem-reboot-mode" },
    {}
    };
    MODULE_DEVICE_TABLE(of, nvmem_reboot_mode_of_match);
    static struct platform_driver nvmem_reboot_mode_driver = {
    .probe = nvmem_reboot_mode_probe,
    .driver = {
    .name = "nvmem-reboot-mode",
    .of_match_table = nvmem_reboot_mode_of_match,
    },
    };
    module_platform_driver(nvmem_reboot_mode_driver);
    MODULE_AUTHOR("Nandor Han <nandor.han@vaisala.com>");
    MODULE_DESCRIPTION("NVMEM reboot mode driver");
    MODULE_LICENSE("GPL");
