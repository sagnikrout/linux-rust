//! Automatically rewritten from C to Rust
//! Source: drivers/nvmem/meson-mx-efuse.c
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
// Amlogic Meson6, Meson8 and Meson8b eFuse Driver
//
// Copyright (c) 2017 Martin Blumenstingl <martin.blumenstingl@googlemail.com>
//

pub const MESON_MX_EFUSE_CNTL1: c_uint = 0x04;

pub const MESON_MX_EFUSE_CNTL2: c_uint = 0x08;
pub const MESON_MX_EFUSE_CNTL4: c_uint = 0x10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_mx_efuse_platform_data {
    pub name: *const c_char,
    pub word_size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_mx_efuse {
    pub base: *mut void __iomem,
    pub core_clk: *mut clk,
    pub config: nvmem_config,
}

    static void meson_mx_efuse_mask_bits(struct meson_mx_efuse *efuse, u32 reg,
    u32 mask, u32 set)
    {
    u32 data;
    data = readl(efuse.base + reg);
    data &= ~mask;
    data |= (set & mask);
    writel(data, efuse.base + reg);
    }
#[no_mangle]
unsafe extern "C" fn meson_mx_efuse_hw_enable(efuse: *mut meson_mx_efuse) -> c_int {
    static int meson_mx_efuse_hw_enable(struct meson_mx_efuse *efuse)
    {
    int err;
    err = clk_prepare_enable(efuse.core_clk);
    if (err)
    return err;
// power up the efuse
    meson_mx_efuse_mask_bits(efuse, MESON_MX_EFUSE_CNTL1,
    MESON_MX_EFUSE_CNTL1_PD_ENABLE, 0);
    meson_mx_efuse_mask_bits(efuse, MESON_MX_EFUSE_CNTL4,
    MESON_MX_EFUSE_CNTL4_ENCRYPT_ENABLE, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn meson_mx_efuse_hw_disable(efuse: *mut meson_mx_efuse) {
    static void meson_mx_efuse_hw_disable(struct meson_mx_efuse *efuse)
    {
    meson_mx_efuse_mask_bits(efuse, MESON_MX_EFUSE_CNTL1,
    MESON_MX_EFUSE_CNTL1_PD_ENABLE,
    MESON_MX_EFUSE_CNTL1_PD_ENABLE);
    clk_disable_unprepare(efuse.core_clk);
    }
    static int meson_mx_efuse_read_addr(struct meson_mx_efuse *efuse,
    unsigned int addr, u32 *value)
    {
    int err;
    u32 regval;
// write the address to read
    regval = FIELD_PREP(MESON_MX_EFUSE_CNTL1_BYTE_ADDR_MASK, addr);
    meson_mx_efuse_mask_bits(efuse, MESON_MX_EFUSE_CNTL1,
    MESON_MX_EFUSE_CNTL1_BYTE_ADDR_MASK, regval);
// inform the hardware that we changed the address
    meson_mx_efuse_mask_bits(efuse, MESON_MX_EFUSE_CNTL1,
    MESON_MX_EFUSE_CNTL1_BYTE_ADDR_SET,
    MESON_MX_EFUSE_CNTL1_BYTE_ADDR_SET);
    meson_mx_efuse_mask_bits(efuse, MESON_MX_EFUSE_CNTL1,
    MESON_MX_EFUSE_CNTL1_BYTE_ADDR_SET, 0);
// start the read process
    meson_mx_efuse_mask_bits(efuse, MESON_MX_EFUSE_CNTL1,
    MESON_MX_EFUSE_CNTL1_AUTO_RD_START,
    MESON_MX_EFUSE_CNTL1_AUTO_RD_START);
    meson_mx_efuse_mask_bits(efuse, MESON_MX_EFUSE_CNTL1,
    MESON_MX_EFUSE_CNTL1_AUTO_RD_START, 0);
//
// perform a dummy read to ensure that the HW has the RD_BUSY bit set
// when polling for the status below.
//
    readl(efuse.base + MESON_MX_EFUSE_CNTL1);
    err = readl_poll_timeout_atomic(efuse.base + MESON_MX_EFUSE_CNTL1,
    regval,
    (!(regval & MESON_MX_EFUSE_CNTL1_AUTO_RD_BUSY)),
    1, 1000);
    if (err) {
    dev_err(efuse.config.dev,
    "Timeout while reading efuse address %u\n", addr);
    return err;
    }
// value = readl(efuse->base + MESON_MX_EFUSE_CNTL2);
    return 0;
    }
    static int meson_mx_efuse_read(void *context, unsigned int offset,
    void *buf, size_t bytes)
    {
    struct meson_mx_efuse *efuse = context;
    u32 tmp;
    int err, i, addr;
    err = meson_mx_efuse_hw_enable(efuse);
    if (err)
    return err;
    meson_mx_efuse_mask_bits(efuse, MESON_MX_EFUSE_CNTL1,
    MESON_MX_EFUSE_CNTL1_AUTO_RD_ENABLE,
    MESON_MX_EFUSE_CNTL1_AUTO_RD_ENABLE);
    for (i = 0; i < bytes; i += efuse.config.word_size) {
    addr = (offset + i) / efuse.config.word_size;
    err = meson_mx_efuse_read_addr(efuse, addr, &tmp);
    if (err)
    break;
    memcpy(buf + i, &tmp,
    min_t(size_t, bytes - i, efuse.config.word_size));
    }
    meson_mx_efuse_mask_bits(efuse, MESON_MX_EFUSE_CNTL1,
    MESON_MX_EFUSE_CNTL1_AUTO_RD_ENABLE, 0);
    meson_mx_efuse_hw_disable(efuse);
    return err;
    }
    static const struct meson_mx_efuse_platform_data meson6_efuse_data = {
    .name = "meson6-efuse",
    .word_size = 1,
    };
    static const struct meson_mx_efuse_platform_data meson8_efuse_data = {
    .name = "meson8-efuse",
    .word_size = 4,
    };
    static const struct meson_mx_efuse_platform_data meson8b_efuse_data = {
    .name = "meson8b-efuse",
    .word_size = 4,
    };
    static const struct of_device_id meson_mx_efuse_match[] = {
    { .compatible = "amlogic,meson6-efuse", .data = &meson6_efuse_data },
    { .compatible = "amlogic,meson8-efuse", .data = &meson8_efuse_data },
    { .compatible = "amlogic,meson8b-efuse", .data = &meson8b_efuse_data },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, meson_mx_efuse_match);
#[no_mangle]
unsafe extern "C" fn meson_mx_efuse_probe(pdev: *mut platform_device) -> c_int {
    static int meson_mx_efuse_probe(struct platform_device *pdev)
    {
    const struct meson_mx_efuse_platform_data *drvdata;
    struct meson_mx_efuse *efuse;
    struct nvmem_device *nvmem;
    drvdata = of_device_get_match_data(&pdev.dev);
    if (!drvdata)
    return -EINVAL;
    efuse = devm_kzalloc(&pdev.dev, sizeof(*efuse), GFP_KERNEL);
    if (!efuse)
    return -ENOMEM;
    efuse.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(efuse.base))
    return PTR_ERR(efuse.base);
    efuse.config.name = drvdata.name;
    efuse.config.owner = THIS_MODULE;
    efuse.config.dev = &pdev.dev;
    efuse.config.priv = efuse;
    efuse.config.add_legacy_fixed_of_cells = true;
    efuse.config.stride = drvdata.word_size;
    efuse.config.word_size = drvdata.word_size;
    efuse.config.size = SZ_512;
    efuse.config.read_only = true;
    efuse.config.reg_read = meson_mx_efuse_read;
    efuse.core_clk = devm_clk_get(&pdev.dev, "core");
    if (IS_ERR(efuse.core_clk)) {
    dev_err(&pdev.dev, "Failed to get core clock\n");
    return PTR_ERR(efuse.core_clk);
    }
    nvmem = devm_nvmem_register(&pdev.dev, &efuse.config);
    return PTR_ERR_OR_ZERO(nvmem);
    }
    static struct platform_driver meson_mx_efuse_driver = {
    .probe = meson_mx_efuse_probe,
    .driver = {
    .name = "meson-mx-efuse",
    .of_match_table = meson_mx_efuse_match,
    },
    };
    module_platform_driver(meson_mx_efuse_driver);
    MODULE_AUTHOR("Martin Blumenstingl <martin.blumenstingl@googlemail.com>");
    MODULE_DESCRIPTION("Amlogic Meson MX eFuse NVMEM driver");
    MODULE_LICENSE("GPL v2");
