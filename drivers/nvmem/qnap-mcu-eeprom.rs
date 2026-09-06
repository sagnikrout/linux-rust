//! Automatically rewritten from C to Rust
//! Source: drivers/nvmem/qnap-mcu-eeprom.c
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
// ee1004 - driver for DDR4 SPD EEPROMs
//
// Copyright (C) 2017-2019 Jean Delvare
//
// Based on the at24 driver:
// Copyright (C) 2005-2007 David Brownell
// Copyright (C) 2008 Wolfram Sang, Pengutronix
//

// Determined by trial and error until read anomalies appeared
pub const QNAP_MCU_EEPROM_SIZE: c_int = 256;
pub const QNAP_MCU_EEPROM_BLOCK_SIZE: c_int = 32;
    static int qnap_mcu_eeprom_read_block(struct qnap_mcu *mcu, unsigned int offset,
    void *val, size_t bytes)
    {
    const u8 cmd[] = { 0xf7, 0xa1, offset, bytes };
    u8 *reply;
    let mut ret: c_int = 0;
    reply = kzalloc(bytes + sizeof(cmd), GFP_KERNEL);
    if (!reply)
    return -ENOMEM;
    ret = qnap_mcu_exec(mcu, cmd, sizeof(cmd), reply, bytes + sizeof(cmd));
    if (ret)
    goto out;
// First bytes must mirror the sent command
    if (memcmp(cmd, reply, sizeof(cmd))) {
    ret = -EIO;
    goto out;
    }
    memcpy(val, reply + sizeof(cmd), bytes);
    out:
    kfree(reply);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qnap_mcu_eeprom_read(priv: *mut c_void, offset: c_uint, val: *mut c_void, bytes: usize) -> c_int {
    static int qnap_mcu_eeprom_read(void *priv, unsigned int offset, void *val, size_t bytes)
    {
    struct qnap_mcu *mcu = priv;
    let mut pos: c_int = 0, ret;
    u8 *buf = val;
    if (unlikely(!bytes))
    return 0;
    while (bytes > 0) {
    size_t to_read = (bytes > QNAP_MCU_EEPROM_BLOCK_SIZE) ?
    QNAP_MCU_EEPROM_BLOCK_SIZE : bytes;
    ret = qnap_mcu_eeprom_read_block(mcu, offset + pos, &buf[pos], to_read);
    if (ret < 0)
    return ret;
    pos += to_read;
    bytes -= to_read;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qnap_mcu_eeprom_probe(pdev: *mut platform_device) -> c_int {
    static int qnap_mcu_eeprom_probe(struct platform_device *pdev)
    {
    struct qnap_mcu *mcu = dev_get_drvdata(pdev.dev.parent);
    let mut nvcfg: nvmem_config = {};
    struct nvmem_device *ndev;
    nvcfg.dev = &pdev.dev;
    nvcfg.of_node = pdev.dev.parent.of_node;
    nvcfg.name = dev_name(&pdev.dev);
    nvcfg.id = NVMEM_DEVID_NONE;
    nvcfg.owner = THIS_MODULE;
    nvcfg.type = NVMEM_TYPE_EEPROM;
    nvcfg.read_only = true;
    nvcfg.root_only = false;
    nvcfg.reg_read = qnap_mcu_eeprom_read;
    nvcfg.size = QNAP_MCU_EEPROM_SIZE;
    nvcfg.word_size = 1;
    nvcfg.stride = 1;
    nvcfg.priv = mcu;
    ndev = devm_nvmem_register(&pdev.dev, &nvcfg);
    if (IS_ERR(ndev))
    return PTR_ERR(ndev);
    return 0;
    }
    static struct platform_driver qnap_mcu_eeprom_driver = {
    .probe = qnap_mcu_eeprom_probe,
    .driver = {
    .name = "qnap-mcu-eeprom",
    },
    };
    module_platform_driver(qnap_mcu_eeprom_driver);
    MODULE_AUTHOR("Heiko Stuebner <heiko@sntech.de>");
    MODULE_DESCRIPTION("QNAP MCU EEPROM driver");
    MODULE_LICENSE("GPL");
