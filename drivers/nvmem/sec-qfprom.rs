//! Automatically rewritten from C to Rust
//! Source: drivers/nvmem/sec-qfprom.c
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
// Copyright (c) 2023, Qualcomm Innovation Center, Inc. All rights reserved.
//

//
// struct sec_qfprom - structure holding secure qfprom attributes
//
// @base: starting physical address for secure qfprom corrected address space.
// @dev: qfprom device structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_qfprom {
    pub base: phys_addr_t,
    pub dev: *mut device,
}

#[no_mangle]
unsafe extern "C" fn sec_qfprom_reg_read(context: *mut c_void, reg: c_uint, _val: *mut c_void, bytes: usize) -> c_int {
    static int sec_qfprom_reg_read(void *context, unsigned int reg, void *_val, size_t bytes)
    {
    struct sec_qfprom *priv = context;
    unsigned int i;
    u8 *val = _val;
    u32 read_val;
    u8 *tmp;
    for (i = 0; i < bytes; i++, reg++) {
    if (i == 0 || reg % 4 == 0) {
    if (qcom_scm_io_readl(priv.base + (reg & ~3), &read_val)) {
    dev_err(priv.dev, "Couldn't access fuse register\n");
    return -EINVAL;
    }
    tmp = (u8 *)&read_val;
    }
    val[i] = tmp[reg & 3];
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sec_qfprom_probe(pdev: *mut platform_device) -> c_int {
    static int sec_qfprom_probe(struct platform_device *pdev)
    {
    struct nvmem_config econfig = {
    .name = "sec-qfprom",
    .add_legacy_fixed_of_cells = true,
    .stride = 1,
    .word_size = 1,
    .id = NVMEM_DEVID_AUTO,
    .reg_read = sec_qfprom_reg_read,
    };
    struct device *dev = &pdev.dev;
    struct nvmem_device *nvmem;
    struct sec_qfprom *priv;
    struct resource *res;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res)
    return -EINVAL;
    priv.base = res.start;
    econfig.size = resource_size(res);
    econfig.dev = dev;
    econfig.priv = priv;
    priv.dev = dev;
    nvmem = devm_nvmem_register(dev, &econfig);
    return PTR_ERR_OR_ZERO(nvmem);
    }
    static const struct of_device_id sec_qfprom_of_match[] = {
    { .compatible = "qcom,sec-qfprom" },
    {/* sentinel */},
    };
    MODULE_DEVICE_TABLE(of, sec_qfprom_of_match);
    static struct platform_driver qfprom_driver = {
    .probe = sec_qfprom_probe,
    .driver = {
    .name = "qcom_sec_qfprom",
    .of_match_table = sec_qfprom_of_match,
    },
    };
    module_platform_driver(qfprom_driver);
    MODULE_DESCRIPTION("Qualcomm Secure QFPROM driver");
    MODULE_LICENSE("GPL");
