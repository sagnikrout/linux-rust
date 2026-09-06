//! Automatically rewritten from C to Rust
//! Source: drivers/nvmem/max77759-nvmem.c
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
// Copyright 2020 Google Inc
// Copyright 2025 Linaro Ltd.
//
// NVMEM driver for Maxim MAX77759

pub const MAX77759_NVMEM_OPCODE_HEADER_LEN: c_int = 3;
//
// NVMEM commands have a three byte header (which becomes part of the command),
// so we need to subtract that.
//

    - MAX77759_NVMEM_OPCODE_HEADER_LEN)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max77759_nvmem {
    pub dev: *mut device,
    pub max77759: *mut max77759,
}

    static int max77759_nvmem_reg_read(void *priv, unsigned int offset,
    void *val, size_t bytes)
    {
    struct max77759_nvmem *nvmem = priv;
    DEFINE_FLEX(struct max77759_maxq_command, cmd, cmd, length,
    MAX77759_NVMEM_OPCODE_HEADER_LEN);
    DEFINE_FLEX(struct max77759_maxq_response, rsp, rsp, length,
    MAX77759_MAXQ_OPCODE_MAXLENGTH);
    int ret;
    cmd.cmd[0] = MAX77759_MAXQ_OPCODE_USER_SPACE_READ;
    cmd.cmd[1] = offset;
    cmd.cmd[2] = bytes;
    rsp.length = bytes + MAX77759_NVMEM_OPCODE_HEADER_LEN;
    ret = max77759_maxq_command(nvmem.max77759, cmd, rsp);
    if (ret < 0)
    return ret;
    if (memcmp(cmd.cmd, rsp.rsp, MAX77759_NVMEM_OPCODE_HEADER_LEN)) {
    dev_warn(nvmem.dev, "protocol error (read)\n");
    return -EIO;
    }
    memcpy(val, &rsp.rsp[MAX77759_NVMEM_OPCODE_HEADER_LEN], bytes);
    return 0;
    }
    static int max77759_nvmem_reg_write(void *priv, unsigned int offset,
    void *val, size_t bytes)
    {
    struct max77759_nvmem *nvmem = priv;
    DEFINE_FLEX(struct max77759_maxq_command, cmd, cmd, length,
    MAX77759_MAXQ_OPCODE_MAXLENGTH);
    DEFINE_FLEX(struct max77759_maxq_response, rsp, rsp, length,
    MAX77759_MAXQ_OPCODE_MAXLENGTH);
    int ret;
    cmd.cmd[0] = MAX77759_MAXQ_OPCODE_USER_SPACE_WRITE;
    cmd.cmd[1] = offset;
    cmd.cmd[2] = bytes;
    memcpy(&cmd.cmd[MAX77759_NVMEM_OPCODE_HEADER_LEN], val, bytes);
    cmd.length = bytes + MAX77759_NVMEM_OPCODE_HEADER_LEN;
    rsp.length = cmd.length;
    ret = max77759_maxq_command(nvmem.max77759, cmd, rsp);
    if (ret < 0)
    return ret;
    if (memcmp(cmd.cmd, rsp.rsp, cmd.length)) {
    dev_warn(nvmem.dev, "protocol error (write)\n");
    return -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max77759_nvmem_probe(pdev: *mut platform_device) -> c_int {
    static int max77759_nvmem_probe(struct platform_device *pdev)
    {
    struct nvmem_config config = {
    .dev = &pdev.dev,
    .name = dev_name(&pdev.dev),
    .id = NVMEM_DEVID_NONE,
    .type = NVMEM_TYPE_EEPROM,
    .ignore_wp = true,
    .size = MAX77759_NVMEM_SIZE,
    .word_size = sizeof(u8),
    .stride = sizeof(u8),
    .reg_read = max77759_nvmem_reg_read,
    .reg_write = max77759_nvmem_reg_write,
    };
    struct max77759_nvmem *nvmem;
    nvmem = devm_kzalloc(&pdev.dev, sizeof(*nvmem), GFP_KERNEL);
    if (!nvmem)
    return -ENOMEM;
    nvmem.dev = &pdev.dev;
    nvmem.max77759 = dev_get_drvdata(pdev.dev.parent);
    config.priv = nvmem;
    return PTR_ERR_OR_ZERO(devm_nvmem_register(config.dev, &config));
    }
    static const struct of_device_id max77759_nvmem_of_id[] = {
    { .compatible = "maxim,max77759-nvmem", },
    { }
    };
    MODULE_DEVICE_TABLE(of, max77759_nvmem_of_id);
    static const struct platform_device_id max77759_nvmem_platform_id[] = {
    { "max77759-nvmem", },
    { }
    };
    MODULE_DEVICE_TABLE(platform, max77759_nvmem_platform_id);
    static struct platform_driver max77759_nvmem_driver = {
    .driver = {
    .name = "max77759-nvmem",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table = max77759_nvmem_of_id,
    },
    .probe = max77759_nvmem_probe,
    .id_table = max77759_nvmem_platform_id,
    };
    module_platform_driver(max77759_nvmem_driver);
    MODULE_AUTHOR("André Draszik <andre.draszik@linaro.org>");
    MODULE_DESCRIPTION("NVMEM driver for Maxim MAX77759");
    MODULE_LICENSE("GPL");
