//! Automatically rewritten from C to Rust
//! Source: drivers/reset/amlogic/reset-meson-aux.c
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Amlogic Meson Reset Auxiliary driver
//
// Copyright (c) 2024 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>
//

    static const struct meson_reset_param meson_a1_audio_param = {
    .reset_ops	= &meson_reset_toggle_ops,
    .reset_num	= 32,
    .level_offset	= 0x28,
    };
    static const struct meson_reset_param meson_a1_audio_vad_param = {
    .reset_ops	= &meson_reset_toggle_ops,
    .reset_num	= 6,
    .level_offset	= 0x8,
    };
    static const struct meson_reset_param meson_g12a_audio_param = {
    .reset_ops	= &meson_reset_toggle_ops,
    .reset_num	= 26,
    .level_offset	= 0x24,
    };
    static const struct meson_reset_param meson_sm1_audio_param = {
    .reset_ops	= &meson_reset_toggle_ops,
    .reset_num	= 39,
    .level_offset	= 0x28,
    };
    static const struct auxiliary_device_id meson_reset_aux_ids[] = {
    {
    .name = "a1-audio-clkc.rst-a1",
    .driver_data = (kernel_ulong_t)&meson_a1_audio_param,
    }, {
    .name = "a1-audio-clkc.rst-a1-vad",
    .driver_data = (kernel_ulong_t)&meson_a1_audio_vad_param,
    }, {
    .name = "axg-audio-clkc.rst-g12a",
    .driver_data = (kernel_ulong_t)&meson_g12a_audio_param,
    }, {
    .name = "axg-audio-clkc.rst-sm1",
    .driver_data = (kernel_ulong_t)&meson_sm1_audio_param,
    }, {}
    };
    MODULE_DEVICE_TABLE(auxiliary, meson_reset_aux_ids);
    static int meson_reset_aux_probe(struct auxiliary_device *adev,
    const struct auxiliary_device_id *id)
    {
    const struct meson_reset_param *param =
    (const struct meson_reset_param *)(id.driver_data);
    struct regmap *map;
    map = dev_get_regmap(adev.dev.parent, core::ptr::null_mut());
    if (!map)
    return -EINVAL;
    return meson_reset_controller_register(&adev.dev, map, param);
    }
    static struct auxiliary_driver meson_reset_aux_driver = {
    .probe		= meson_reset_aux_probe,
    .id_table	= meson_reset_aux_ids,
    };
    module_auxiliary_driver(meson_reset_aux_driver);
    MODULE_DESCRIPTION("Amlogic Meson Reset Auxiliary driver");
    MODULE_AUTHOR("Jerome Brunet <jbrunet@baylibre.com>");
    MODULE_LICENSE("Dual BSD/GPL");
    MODULE_IMPORT_NS("MESON_RESET");
