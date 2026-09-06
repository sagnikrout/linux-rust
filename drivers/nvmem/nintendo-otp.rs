//! Automatically rewritten from C to Rust
//! Source: drivers/nvmem/nintendo-otp.c
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
// Nintendo Wii and Wii U OTP driver
//
// This is a driver exposing the OTP of a Nintendo Wii or Wii U console.
//
// This memory contains common and per-console keys, signatures and
// related data required to access peripherals.
//
// Based on reversed documentation from https://wiiubrew.org/wiki/Hardware/OTP
//
// Copyright (C) 2021 Emmanuel Gil Peyrot <linkmauve@linkmauve.fr>
//

pub const HW_OTPCMD: c_int = 0;
pub const HW_OTPDATA: c_int = 4;
pub const OTP_READ: c_uint = 0x80000000;
pub const BANK_SIZE: c_int = 128;
pub const WORD_SIZE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nintendo_otp_priv {
    pub regs: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nintendo_otp_devtype_data {
    pub name: *const c_char,
    pub num_banks: c_uint,
}

    static const struct nintendo_otp_devtype_data hollywood_otp_data = {
    .name = "wii-otp",
    .num_banks = 1,
    };
    static const struct nintendo_otp_devtype_data latte_otp_data = {
    .name = "wiiu-otp",
    .num_banks = 8,
    };
    static int nintendo_otp_reg_read(void *context,
    unsigned int reg, void *_val, size_t bytes)
    {
    struct nintendo_otp_priv *priv = context;
    u32 *val = _val;
    let mut words: c_int = bytes / WORD_SIZE;
    u32 bank, addr;
    while (words--) {
    bank = (reg / BANK_SIZE) << 8;
    addr = (reg / WORD_SIZE) % (BANK_SIZE / WORD_SIZE);
    iowrite32be(OTP_READ | bank | addr, priv.regs + HW_OTPCMD);
// val++ = ioread32be(priv->regs + HW_OTPDATA);
    reg += WORD_SIZE;
    }
    return 0;
    }
    static const struct of_device_id nintendo_otp_of_table[] = {
    { .compatible = "nintendo,hollywood-otp", .data = &hollywood_otp_data },
    { .compatible = "nintendo,latte-otp", .data = &latte_otp_data },
    {/* sentinel */},
    };
    MODULE_DEVICE_TABLE(of, nintendo_otp_of_table);
#[no_mangle]
unsafe extern "C" fn nintendo_otp_probe(pdev: *mut platform_device) -> c_int {
    static int nintendo_otp_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    const struct nintendo_otp_devtype_data *data;
    struct nvmem_device *nvmem;
    struct nintendo_otp_priv *priv;
    struct nvmem_config config = {
    .stride = WORD_SIZE,
    .word_size = WORD_SIZE,
    .reg_read = nintendo_otp_reg_read,
    .read_only = true,
    .root_only = true,
    };
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.regs))
    return PTR_ERR(priv.regs);
    data = of_device_get_match_data(dev);
    if (data) {
    config.name = data.name;
    config.size = data.num_banks * BANK_SIZE;
    }
    config.dev = dev;
    config.priv = priv;
    nvmem = devm_nvmem_register(dev, &config);
    return PTR_ERR_OR_ZERO(nvmem);
    }
    static struct platform_driver nintendo_otp_driver = {
    .probe = nintendo_otp_probe,
    .driver = {
    .name = "nintendo-otp",
    .of_match_table = nintendo_otp_of_table,
    },
    };
    module_platform_driver(nintendo_otp_driver);
    MODULE_AUTHOR("Emmanuel Gil Peyrot <linkmauve@linkmauve.fr>");
    MODULE_DESCRIPTION("Nintendo Wii and Wii U OTP driver");
    MODULE_LICENSE("GPL v2");
