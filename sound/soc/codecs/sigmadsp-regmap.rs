//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/sigmadsp-regmap.c
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
// Load Analog Devices SigmaStudio firmware files
//
// Copyright 2009-2011 Analog Devices Inc.
//

    static int sigmadsp_write_regmap(void *control_data,
    unsigned int addr, const uint8_t data[], size_t len)
    {
    return regmap_raw_write(control_data, addr,
    data, len);
    }
    static int sigmadsp_read_regmap(void *control_data,
    unsigned int addr, uint8_t data[], size_t len)
    {
    return regmap_raw_read(control_data, addr,
    data, len);
    }
//
// devm_sigmadsp_init_regmap() - Initialize SigmaDSP instance
// @dev: The parent device
// @regmap: Regmap instance to use
// @ops: The sigmadsp_ops to use for this instance
// @firmware_name: Name of the firmware file to load
//
// Allocates a SigmaDSP instance and loads the specified firmware file.
//
// Returns a pointer to a struct sigmadsp on success, or a PTR_ERR() on error.
//
    struct sigmadsp *devm_sigmadsp_init_regmap(struct device *dev,
    struct regmap *regmap, const struct sigmadsp_ops *ops,
    const char *firmware_name)
    {
    struct sigmadsp *sigmadsp;
    sigmadsp = devm_sigmadsp_init(dev, ops, firmware_name);
    if (IS_ERR(sigmadsp))
    return sigmadsp;
    sigmadsp.control_data = regmap;
    sigmadsp.write = sigmadsp_write_regmap;
    sigmadsp.read = sigmadsp_read_regmap;
    return sigmadsp;
    }
    EXPORT_SYMBOL_GPL(devm_sigmadsp_init_regmap);
    MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>");
    MODULE_DESCRIPTION("SigmaDSP regmap firmware loader");
    MODULE_LICENSE("GPL");
