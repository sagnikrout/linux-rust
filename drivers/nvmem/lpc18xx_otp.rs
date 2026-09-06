//! Automatically rewritten from C to Rust
//! Source: drivers/nvmem/lpc18xx_otp.c
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
// NXP LPC18xx/43xx OTP memory NVMEM driver
//
// Copyright (c) 2016 Joachim Eastwood <manabian@gmail.com>
//
// Based on the imx ocotp driver,
// Copyright (c) 2015 Pengutronix, Philipp Zabel <p.zabel@pengutronix.de>
//
// TODO: add support for writing OTP register via API in boot ROM.
//

//
// LPC18xx OTP memory contains 4 banks with 4 32-bit words. Bank 0 starts
// at offset 0 from the base.
//
// Bank 0 contains the part ID for Flashless devices and is reserved for
// devices with Flash.
// Bank 1/2 is generale purpose or AES key storage for secure devices.
// Bank 3 contains control data, USB ID and generale purpose words.
//
pub const LPC18XX_OTP_NUM_BANKS: c_int = 4;
pub const LPC18XX_OTP_WORDS_PER_BANK: c_int = 4;

    LPC18XX_OTP_WORDS_PER_BANK * \
    LPC18XX_OTP_WORD_SIZE)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpc18xx_otp {
    pub base: *mut void __iomem,
}

    static int lpc18xx_otp_read(void *context, unsigned int offset,
    void *val, size_t bytes)
    {
    struct lpc18xx_otp *otp = context;
    let mut count: c_uint = bytes >> 2;
    let mut index: u32 = offset >> 2;
    u32 *buf = val;
    int i;
    if (count > (LPC18XX_OTP_SIZE - index))
    count = LPC18XX_OTP_SIZE - index;
    for (i = index; i < (index + count); i++)
// buf++ = readl(otp->base + i * LPC18XX_OTP_WORD_SIZE);
    return 0;
    }
    static struct nvmem_config lpc18xx_otp_nvmem_config = {
    .name = "lpc18xx-otp",
    .read_only = true,
    .word_size = LPC18XX_OTP_WORD_SIZE,
    .stride = LPC18XX_OTP_WORD_SIZE,
    .reg_read = lpc18xx_otp_read,
    };
#[no_mangle]
unsafe extern "C" fn lpc18xx_otp_probe(pdev: *mut platform_device) -> c_int {
    static int lpc18xx_otp_probe(struct platform_device *pdev)
    {
    struct nvmem_device *nvmem;
    struct lpc18xx_otp *otp;
    otp = devm_kzalloc(&pdev.dev, sizeof(*otp), GFP_KERNEL);
    if (!otp)
    return -ENOMEM;
    otp.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(otp.base))
    return PTR_ERR(otp.base);
    lpc18xx_otp_nvmem_config.size = LPC18XX_OTP_SIZE;
    lpc18xx_otp_nvmem_config.dev = &pdev.dev;
    lpc18xx_otp_nvmem_config.priv = otp;
    nvmem = devm_nvmem_register(&pdev.dev, &lpc18xx_otp_nvmem_config);
    return PTR_ERR_OR_ZERO(nvmem);
    }
    static const struct of_device_id lpc18xx_otp_dt_ids[] = {
    { .compatible = "nxp,lpc1850-otp" },
    { },
    };
    MODULE_DEVICE_TABLE(of, lpc18xx_otp_dt_ids);
    static struct platform_driver lpc18xx_otp_driver = {
    .probe	= lpc18xx_otp_probe,
    .driver = {
    .name	= "lpc18xx_otp",
    .of_match_table = lpc18xx_otp_dt_ids,
    },
    };
    module_platform_driver(lpc18xx_otp_driver);
    MODULE_AUTHOR("Joachim Eastwoood <manabian@gmail.com>");
    MODULE_DESCRIPTION("NXP LPC18xx OTP NVMEM driver");
    MODULE_LICENSE("GPL v2");
