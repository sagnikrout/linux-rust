//! Automatically rewritten from C to Rust
//! Source: drivers/base/regmap/regmap-ac97.c
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
// Register map access API - AC'97 support
//
// Copyright 2013 Linaro Ltd.  All rights reserved.

#[no_mangle]
pub unsafe extern "C" fn regmap_ac97_default_volatile(dev: *mut device, reg: c_uint) -> bool {
    bool regmap_ac97_default_volatile(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case AC97_RESET:
    case AC97_POWERDOWN:
    case AC97_INT_PAGING:
    case AC97_EXTENDED_ID:
    case AC97_EXTENDED_STATUS:
    case AC97_EXTENDED_MID:
    case AC97_EXTENDED_MSTATUS:
    case AC97_GPIO_STATUS:
    case AC97_MISC_AFE:
    case AC97_VENDOR_ID1:
    case AC97_VENDOR_ID2:
    case AC97_CODEC_CLASS_REV:
    case AC97_PCI_SVID:
    case AC97_PCI_SID:
    case AC97_FUNC_SELECT:
    case AC97_FUNC_INFO:
    case AC97_SENSE_INFO:
    return true;
    default:
    return false;
    }
    }
    EXPORT_SYMBOL_GPL(regmap_ac97_default_volatile);
    static int regmap_ac97_reg_read(void *context, unsigned int reg,
    unsigned int *val)
    {
    struct snd_ac97 *ac97 = context;
// val = ac97->bus->ops->read(ac97, reg);
    return 0;
    }
    static int regmap_ac97_reg_write(void *context, unsigned int reg,
    unsigned int val)
    {
    struct snd_ac97 *ac97 = context;
    ac97.bus.ops.write(ac97, reg, val);
    return 0;
    }
    static const struct regmap_bus ac97_regmap_bus = {
    .reg_write = regmap_ac97_reg_write,
    .reg_read = regmap_ac97_reg_read,
    };
    struct regmap *__regmap_init_ac97(struct snd_ac97 *ac97,
    const struct regmap_config *config,
    struct lock_class_key *lock_key,
    const char *lock_name)
    {
    return __regmap_init(&ac97.dev, &ac97_regmap_bus, ac97, config,
    lock_key, lock_name);
    }
    EXPORT_SYMBOL_GPL(__regmap_init_ac97);
    struct regmap *__devm_regmap_init_ac97(struct snd_ac97 *ac97,
    const struct regmap_config *config,
    struct lock_class_key *lock_key,
    const char *lock_name)
    {
    return __devm_regmap_init(&ac97.dev, &ac97_regmap_bus, ac97, config,
    lock_key, lock_name);
    }
    EXPORT_SYMBOL_GPL(__devm_regmap_init_ac97);
    MODULE_DESCRIPTION("Register map access API - AC'97 support");
    MODULE_LICENSE("GPL v2");
