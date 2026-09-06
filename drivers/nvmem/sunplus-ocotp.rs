//! Automatically rewritten from C to Rust
//! Source: drivers/nvmem/sunplus-ocotp.c
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
// The OCOTP driver for Sunplus	SP7021
//
// Copyright (C) 2019 Sunplus Technology Inc., All rights reserved.
//

//
// OTP memory
// Each bank contains 4 words (32 bits).
// Bank 0 starts at offset 0 from the base.
//
pub const OTP_WORDS_PER_BANK: c_int = 4;

pub const QAC628_OTP_NUM_BANKS: c_int = 8;

pub const OTP_READ_TIMEOUT_US: c_int = 200000;
// HB_GPIO
pub const ADDRESS_8_DATA: c_uint = 0x20;
// OTP_RX
pub const OTP_CONTROL_2: c_uint = 0x48;

pub const OTP_STATUS: c_uint = 0x4c;

pub const OTP_READ_ADDRESS: c_uint = 0x50;
    enum base_type {
    HB_GPIO,
    OTPRX,
    BASEMAX,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sp_ocotp_priv {
    pub dev: *mut device,
    pub base: [*mut void __iomem; BASEMAX],
    pub clk: *mut clk,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sp_ocotp_data {
    pub size: c_int,
}

    static const struct sp_ocotp_data sp_otp_v0 = {
    .size = QAC628_OTP_SIZE,
    };
#[no_mangle]
unsafe extern "C" fn sp_otp_read_real(otp: *mut sp_ocotp_priv, addr: c_int, value: *mut c_char) -> c_int {
    static int sp_otp_read_real(struct sp_ocotp_priv *otp, int addr, char *value)
    {
    unsigned int addr_data;
    unsigned int byte_shift;
    unsigned int status;
    int ret;
    addr_data = addr % (OTP_WORD_SIZE * OTP_WORDS_PER_BANK);
    addr_data = addr_data / OTP_WORD_SIZE;
    byte_shift = addr % (OTP_WORD_SIZE * OTP_WORDS_PER_BANK);
    byte_shift = byte_shift % OTP_WORD_SIZE;
    addr = addr / (OTP_WORD_SIZE * OTP_WORDS_PER_BANK);
    addr = addr * OTP_BIT_ADDR_OF_BANK;
    writel(readl(otp.base[OTPRX] + OTP_STATUS) & OTP_READ_DONE_MASK &
    OTP_LOAD_SECURE_DONE_MASK, otp.base[OTPRX] + OTP_STATUS);
    writel(addr, otp.base[OTPRX] + OTP_READ_ADDRESS);
    writel(readl(otp.base[OTPRX] + OTP_CONTROL_2) | OTP_READ,
    otp.base[OTPRX] + OTP_CONTROL_2);
    writel(readl(otp.base[OTPRX] + OTP_CONTROL_2) & SEL_BAK_KEY2_MASK & SW_TRIM_EN_MASK
    & SEL_BAK_KEY_MASK & OTP_LOAD_SECURE_DATA_MASK & OTP_DO_CRC_MASK,
    otp.base[OTPRX] + OTP_CONTROL_2);
    writel((readl(otp.base[OTPRX] + OTP_CONTROL_2) & OTP_RD_PERIOD_MASK) | CPU_CLOCK,
    otp.base[OTPRX] + OTP_CONTROL_2);
    ret = readl_poll_timeout(otp.base[OTPRX] + OTP_STATUS, status,
    status & OTP_READ_DONE, 10, OTP_READ_TIMEOUT_US);
    if (ret < 0)
    return ret;
// value = (readl(otp->base[HB_GPIO] + ADDRESS_8_DATA + addr_data * OTP_WORD_SIZE)
    >> (8 * byte_shift)) & 0xff;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sp_ocotp_read(priv: *mut c_void, offset: c_uint, value: *mut c_void, bytes: usize) -> c_int {
    static int sp_ocotp_read(void *priv, unsigned int offset, void *value, size_t bytes)
    {
    struct sp_ocotp_priv *otp = priv;
    unsigned int addr;
    char *buf = value;
    char val[4];
    int ret;
    ret = clk_enable(otp.clk);
    if (ret)
    return ret;
// buf = 0;
    for (addr = offset; addr < (offset + bytes); addr++) {
    ret = sp_otp_read_real(otp, addr, val);
    if (ret < 0) {
    dev_err(otp.dev, "OTP read fail:%d at %d", ret, addr);
    goto disable_clk;
    }
// buf++ = *val;
    }
    disable_clk:
    clk_disable(otp.clk);
    return ret;
    }
    static struct nvmem_config sp_ocotp_nvmem_config = {
    .name = "sp-ocotp",
    .add_legacy_fixed_of_cells = true,
    .read_only = true,
    .word_size = 1,
    .size = QAC628_OTP_SIZE,
    .stride = 1,
    .reg_read = sp_ocotp_read,
    .owner = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn sp_ocotp_probe(pdev: *mut platform_device) -> c_int {
    static int sp_ocotp_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct nvmem_device *nvmem;
    struct sp_ocotp_priv *otp;
    int ret;
    otp = devm_kzalloc(dev, sizeof(*otp), GFP_KERNEL);
    if (!otp)
    return -ENOMEM;
    otp.dev = dev;
    otp.base[HB_GPIO] = devm_platform_ioremap_resource_byname(pdev, "hb_gpio");
    if (IS_ERR(otp.base[HB_GPIO]))
    return PTR_ERR(otp.base[HB_GPIO]);
    otp.base[OTPRX] = devm_platform_ioremap_resource_byname(pdev, "otprx");
    if (IS_ERR(otp.base[OTPRX]))
    return PTR_ERR(otp.base[OTPRX]);
    otp.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(otp.clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(otp.clk),
    "devm_clk_get fail\n");
    ret = clk_prepare(otp.clk);
    if (ret < 0) {
    dev_err(dev, "failed to prepare clk: %d\n", ret);
    return ret;
    }
    sp_ocotp_nvmem_config.priv = otp;
    sp_ocotp_nvmem_config.dev = dev;
    nvmem = devm_nvmem_register(dev, &sp_ocotp_nvmem_config);
    if (IS_ERR(nvmem)) {
    ret = dev_err_probe(&pdev.dev, PTR_ERR(nvmem),
    "register nvmem device fail\n");
    goto err;
    }
    platform_set_drvdata(pdev, nvmem);
    dev_dbg(dev, "banks:%d x wpb:%d x wsize:%d = %d",
    (int)QAC628_OTP_NUM_BANKS, (int)OTP_WORDS_PER_BANK,
    (int)OTP_WORD_SIZE, (int)QAC628_OTP_SIZE);
    return 0;
    err:
    clk_unprepare(otp.clk);
    return ret;
    }
    static const struct of_device_id sp_ocotp_dt_ids[] = {
    { .compatible = "sunplus,sp7021-ocotp", .data = &sp_otp_v0 },
    { }
    };
    MODULE_DEVICE_TABLE(of, sp_ocotp_dt_ids);
    static struct platform_driver sp_otp_driver = {
    .probe     = sp_ocotp_probe,
    .driver    = {
    .name           = "sunplus,sp7021-ocotp",
    .of_match_table = sp_ocotp_dt_ids,
    }
    };
    module_platform_driver(sp_otp_driver);
    MODULE_AUTHOR("Vincent Shih <vincent.sunplus@gmail.com>");
    MODULE_DESCRIPTION("Sunplus On-Chip OTP driver");
    MODULE_LICENSE("GPL");
