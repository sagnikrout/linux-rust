//! Automatically rewritten from C to Rust
//! Source: drivers/nvmem/airoha-smc-efuses.c
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
// Author: Christian Marangi <ansuelsmth@gmail.com>
//

pub const AIROHA_SMC_EFUSE_FID: c_uint = 0x82000001;
pub const AIROHA_SMC_EFUSE_SUB_ID_READ: c_uint = 0x44414552;
pub const AIROHA_EFUSE_CELLS: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_efuse_bank_priv {
    pub bank_index: u32,
}

    static int airoha_efuse_read(void *context, unsigned int offset,
    void *val, size_t bytes)
    {
    struct regmap *regmap = context;
    return regmap_bulk_read(regmap, offset,
    val, bytes / sizeof(u32));
    }
    static int airoha_efuse_reg_read(void *context, unsigned int offset,
    unsigned int *val)
    {
    struct airoha_efuse_bank_priv *priv = context;
    struct arm_smccc_res res;
    arm_smccc_1_1_invoke(AIROHA_SMC_EFUSE_FID,
    AIROHA_SMC_EFUSE_SUB_ID_READ,
    priv.bank_index, offset, 0, 0, 0, 0, &res);
// check if SMC reported an error
    if (res.a0)
    return -EIO;
// val = res.a1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn airoha_efuse_probe(pdev: *mut platform_device) -> c_int {
    static int airoha_efuse_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    int ret;
    for_each_child_of_node_scoped(dev.of_node, child) {
    struct nvmem_config nvmem_config = {
    .size = AIROHA_EFUSE_CELLS * sizeof(u32),
    .stride = sizeof(u32),
    .word_size = sizeof(u32),
    .reg_read = airoha_efuse_read,
    };
    struct regmap_config regmap_config = {
    .reg_read = airoha_efuse_reg_read,
    .reg_bits = 32,
    .val_bits = 32,
    .reg_stride = 4,
    };
    struct airoha_efuse_bank_priv *priv;
    struct nvmem_device *nvmem;
    struct regmap *regmap;
    const char *name;
    u32 bank;
    ret = of_property_read_u32(child, "reg", &bank);
    if (ret)
    return ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    name = devm_kasprintf(dev, GFP_KERNEL, "airoha-efuse-%u",
    bank);
    if (!name)
    return -ENOMEM;
    priv.bank_index = bank;
    regmap_config.name = name;
    regmap = devm_regmap_init(dev, core::ptr::null_mut(), priv,
    &regmap_config);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    nvmem_config.name = name;
    nvmem_config.priv = regmap;
    nvmem_config.dev = dev;
    nvmem_config.id = bank;
    nvmem_config.of_node = child;
    nvmem = devm_nvmem_register(dev, &nvmem_config);
    if (IS_ERR(nvmem))
    return PTR_ERR(nvmem);
    }
    return 0;
    }
    static const struct of_device_id airoha_efuse_of_match[] = {
    { .compatible = "airoha,an7581-efuses", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, airoha_efuse_of_match);
    static struct platform_driver airoha_efuse_driver = {
    .probe = airoha_efuse_probe,
    .driver = {
    .name = "airoha-efuse",
    .of_match_table = airoha_efuse_of_match,
    },
    };
    module_platform_driver(airoha_efuse_driver);
    MODULE_AUTHOR("Christian Marangi <ansuelsmth@gmail.com>");
    MODULE_DESCRIPTION("Driver for Airoha SMC eFUSEs");
    MODULE_LICENSE("GPL");
