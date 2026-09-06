//! Automatically rewritten from C to Rust
//! Source: drivers/media/v4l2-core/v4l2-cci.c
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
// MIPI Camera Control Interface (CCI) register access helpers.
//
// Copyright (C) 2023 Hans de Goede <hansg@kernel.org>
//

#[no_mangle]
pub unsafe extern "C" fn cci_read(map: *mut regmap, reg: u32, val: *mut u64, err: *mut c_int) -> c_int {
    int cci_read(struct regmap *map, u32 reg, u64 *val, int *err)
    {
    bool little_endian;
    unsigned int len;
    u8 buf[8];
    int ret;
//
// TODO: Fix smatch. Assign *val to 0 here in order to avoid
// failing a smatch check on caller when the caller proceeds to
// read *val without initialising it on caller's side. *val is set
// to a valid value whenever this function returns 0 but smatch
// can't figure that out currently.
//
// val = 0;
    if (err && *err)
    return *err;
    little_endian = reg & CCI_REG_LE;
    len = CCI_REG_WIDTH_BYTES(reg);
    reg = CCI_REG_ADDR(reg);
    ret = regmap_bulk_read(map, reg, buf, len);
    if (ret) {
    dev_err(regmap_get_device(map), "Error reading reg 0x%04x: %d\n",
    reg, ret);
    goto out;
    }
    switch (len) {
    case 1:
// val = buf[0];
    break;
    case 2:
    if (little_endian)
// val = get_unaligned_le16(buf);
    else
// val = get_unaligned_be16(buf);
    break;
    case 3:
    if (little_endian)
// val = get_unaligned_le24(buf);
    else
// val = get_unaligned_be24(buf);
    break;
    case 4:
    if (little_endian)
// val = get_unaligned_le32(buf);
    else
// val = get_unaligned_be32(buf);
    break;
    case 8:
    if (little_endian)
// val = get_unaligned_le64(buf);
    else
// val = get_unaligned_be64(buf);
    break;
    default:
    dev_err(regmap_get_device(map), "Error invalid reg-width %u for reg 0x%04x\n",
    len, reg);
    ret = -EINVAL;
    break;
    }
    out:
    if (ret && err)
// err = ret;
    return ret;
    }
    EXPORT_SYMBOL_GPL(cci_read);
#[no_mangle]
pub unsafe extern "C" fn cci_write(map: *mut regmap, reg: u32, val: u64, err: *mut c_int) -> c_int {
    int cci_write(struct regmap *map, u32 reg, u64 val, int *err)
    {
    bool little_endian;
    unsigned int len;
    u8 buf[8];
    int ret;
    if (err && *err)
    return *err;
    little_endian = reg & CCI_REG_LE;
    len = CCI_REG_WIDTH_BYTES(reg);
    reg = CCI_REG_ADDR(reg);
    switch (len) {
    case 1:
    buf[0] = val;
    break;
    case 2:
    if (little_endian)
    put_unaligned_le16(val, buf);
    else
    put_unaligned_be16(val, buf);
    break;
    case 3:
    if (little_endian)
    put_unaligned_le24(val, buf);
    else
    put_unaligned_be24(val, buf);
    break;
    case 4:
    if (little_endian)
    put_unaligned_le32(val, buf);
    else
    put_unaligned_be32(val, buf);
    break;
    case 8:
    if (little_endian)
    put_unaligned_le64(val, buf);
    else
    put_unaligned_be64(val, buf);
    break;
    default:
    dev_err(regmap_get_device(map), "Error invalid reg-width %u for reg 0x%04x\n",
    len, reg);
    ret = -EINVAL;
    goto out;
    }
    ret = regmap_bulk_write(map, reg, buf, len);
    if (ret)
    dev_err(regmap_get_device(map), "Error writing reg 0x%04x: %d\n",
    reg, ret);
    out:
    if (ret && err)
// err = ret;
    return ret;
    }
    EXPORT_SYMBOL_GPL(cci_write);
#[no_mangle]
pub unsafe extern "C" fn cci_update_bits(map: *mut regmap, reg: u32, mask: u64, val: u64, err: *mut c_int) -> c_int {
    int cci_update_bits(struct regmap *map, u32 reg, u64 mask, u64 val, int *err)
    {
    u64 readval;
    int ret;
    ret = cci_read(map, reg, &readval, err);
    if (ret)
    return ret;
    val = (readval & ~mask) | (val & mask);
    return cci_write(map, reg, val, err);
    }
    EXPORT_SYMBOL_GPL(cci_update_bits);
    int cci_multi_reg_write(struct regmap *map, const struct cci_reg_sequence *regs,
    unsigned int num_regs, int *err)
    {
    unsigned int i;
    int ret;
    for (i = 0; i < num_regs; i++) {
    ret = cci_write(map, regs[i].reg, regs[i].val, err);
    if (ret)
    return ret;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(cci_multi_reg_write);

    struct regmap *devm_cci_regmap_init_i2c(struct i2c_client *client,
    int reg_addr_bits)
    {
    struct regmap_config config = {
    .reg_bits = reg_addr_bits,
    .val_bits = 8,
    .reg_format_endian = REGMAP_ENDIAN_BIG,
    .disable_locking = true,
    };
    return devm_regmap_init_i2c(client, &config);
    }
    EXPORT_SYMBOL_GPL(devm_cci_regmap_init_i2c);

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Hans de Goede <hansg@kernel.org>");
    MODULE_DESCRIPTION("MIPI Camera Control Interface (CCI) support");
