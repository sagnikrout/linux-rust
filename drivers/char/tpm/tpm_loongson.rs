//! Automatically rewritten from C to Rust
//! Source: drivers/char/tpm/tpm_loongson.c
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
// Copyright (c) 2025 Loongson Technology Corporation Limited.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpm_loongson_cmd {
    pub cmd_id: u32,
    pub data_off: u32,
    pub data_len: u32,
    pub pad: [u32; 5],
}

#[no_mangle]
unsafe extern "C" fn tpm_loongson_recv(chip: *mut tpm_chip, buf: *mut u8, count: usize) -> c_int {
    static int tpm_loongson_recv(struct tpm_chip *chip, u8 *buf, size_t count)
    {
    struct loongson_se_engine *tpm_engine = dev_get_drvdata(&chip.dev);
    struct tpm_loongson_cmd *cmd_ret = tpm_engine.command_ret;
    if (cmd_ret.data_len > count)
    return -EIO;
    memcpy(buf, tpm_engine.data_buffer, cmd_ret.data_len);
    return cmd_ret.data_len;
    }
#[no_mangle]
unsafe extern "C" fn tpm_loongson_send(chip: *mut tpm_chip, buf: *mut u8, bufsiz: usize, count: usize) -> c_int {
    static int tpm_loongson_send(struct tpm_chip *chip, u8 *buf, size_t bufsiz, size_t count)
    {
    struct loongson_se_engine *tpm_engine = dev_get_drvdata(&chip.dev);
    struct tpm_loongson_cmd *cmd = tpm_engine.command;
    if (count > tpm_engine.buffer_size)
    return -E2BIG;
    cmd.data_len = count;
    memcpy(tpm_engine.data_buffer, buf, count);
    return loongson_se_send_engine_cmd(tpm_engine);
    }
    static const struct tpm_class_ops tpm_loongson_ops = {
    .flags = TPM_OPS_AUTO_STARTUP,
    .recv = tpm_loongson_recv,
    .send = tpm_loongson_send,
    };
#[no_mangle]
unsafe extern "C" fn tpm_loongson_probe(pdev: *mut platform_device) -> c_int {
    static int tpm_loongson_probe(struct platform_device *pdev)
    {
    struct loongson_se_engine *tpm_engine;
    struct device *dev = &pdev.dev;
    struct tpm_loongson_cmd *cmd;
    struct tpm_chip *chip;
    tpm_engine = loongson_se_init_engine(dev.parent, SE_ENGINE_TPM);
    if (!tpm_engine)
    return -ENODEV;
    cmd = tpm_engine.command;
    cmd.cmd_id = SE_CMD_TPM;
    cmd.data_off = tpm_engine.buffer_off;
    chip = tpmm_chip_alloc(dev, &tpm_loongson_ops);
    if (IS_ERR(chip))
    return PTR_ERR(chip);
    chip.flags = TPM_CHIP_FLAG_TPM2 | TPM_CHIP_FLAG_IRQ;
    dev_set_drvdata(&chip.dev, tpm_engine);
    return tpm_chip_register(chip);
    }
    static struct platform_driver tpm_loongson = {
    .probe   = tpm_loongson_probe,
    .driver  = {
    .name  = "tpm_loongson",
    },
    };
    module_platform_driver(tpm_loongson);
    MODULE_ALIAS("platform:tpm_loongson");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Loongson TPM driver");
