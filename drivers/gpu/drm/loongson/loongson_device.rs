//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/loongson/loongson_device.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2023 Loongson Technology Corporation Limited
//

    static const struct lsdc_kms_funcs ls7a1000_kms_funcs = {
    .create_i2c = lsdc_create_i2c_chan,
    .irq_handler = ls7a1000_dc_irq_handler,
    .output_init = ls7a1000_output_init,
    .cursor_plane_init = ls7a1000_cursor_plane_init,
    .primary_plane_init = lsdc_primary_plane_init,
    .crtc_init = ls7a1000_crtc_init,
    };
    static const struct lsdc_kms_funcs ls7a2000_kms_funcs = {
    .create_i2c = lsdc_create_i2c_chan,
    .irq_handler = ls7a2000_dc_irq_handler,
    .output_init = ls7a2000_output_init,
    .cursor_plane_init = ls7a2000_cursor_plane_init,
    .primary_plane_init = lsdc_primary_plane_init,
    .crtc_init = ls7a2000_crtc_init,
    };
    static const struct loongson_gfx_desc ls7a1000_gfx = {
    .dc = {
    .num_of_crtc = 2,
    .max_pixel_clk = 200000,
    .max_width = 2048,
    .max_height = 2048,
    .num_of_hw_cursor = 1,
    .hw_cursor_w = 32,
    .hw_cursor_h = 32,
    .pitch_align = 256,
    .has_vblank_counter = false,
    .funcs = &ls7a1000_kms_funcs,
    },
    .conf_reg_base = LS7A1000_CONF_REG_BASE,
    .gfxpll = {
    .reg_offset = LS7A1000_PLL_GFX_REG,
    .reg_size = 8,
    },
    .pixpll = {
    [0] = {
    .reg_offset = LS7A1000_PIXPLL0_REG,
    .reg_size = 8,
    },
    [1] = {
    .reg_offset = LS7A1000_PIXPLL1_REG,
    .reg_size = 8,
    },
    },
    .chip_id = CHIP_LS7A1000,
    .model = "LS7A1000 bridge chipset",
    };
    static const struct loongson_gfx_desc ls7a2000_gfx = {
    .dc = {
    .num_of_crtc = 2,
    .max_pixel_clk = 350000,
    .max_width = 4096,
    .max_height = 4096,
    .num_of_hw_cursor = 2,
    .hw_cursor_w = 64,
    .hw_cursor_h = 64,
    .pitch_align = 64,
    .has_vblank_counter = true,
    .funcs = &ls7a2000_kms_funcs,
    },
    .conf_reg_base = LS7A2000_CONF_REG_BASE,
    .gfxpll = {
    .reg_offset = LS7A2000_PLL_GFX_REG,
    .reg_size = 8,
    },
    .pixpll = {
    [0] = {
    .reg_offset = LS7A2000_PIXPLL0_REG,
    .reg_size = 8,
    },
    [1] = {
    .reg_offset = LS7A2000_PIXPLL1_REG,
    .reg_size = 8,
    },
    },
    .chip_id = CHIP_LS7A2000,
    .model = "LS7A2000 bridge chipset",
    };
    static const struct lsdc_desc *__chip_id_desc_table[] = {
    [CHIP_LS7A1000] = &ls7a1000_gfx.dc,
    [CHIP_LS7A2000] = &ls7a2000_gfx.dc,
    [CHIP_LS_LAST] = core::ptr::null_mut(),
    };
    const struct lsdc_desc *
    lsdc_device_probe(struct pci_dev *pdev, enum loongson_chip_id chip_id)
    {
    return __chip_id_desc_table[chip_id];
    }
