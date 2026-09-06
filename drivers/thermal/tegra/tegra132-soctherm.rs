//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/tegra/tegra132-soctherm.c
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
// Copyright (c) 2014-2018, NVIDIA CORPORATION.  All rights reserved.
//
// This software is licensed under the terms of the GNU General Public
// License version 2, as published by the Free Software Foundation, and
// may be copied, distributed, and modified under those terms.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//

pub const TEGRA132_THERMTRIP_TSENSE_THRESH_MASK: c_uint = 0xff;

pub const TEGRA132_THRESH_GRAIN: c_int = 1000;
pub const TEGRA132_BPTT: c_int = 8;
    static const struct tegra_tsensor_configuration tegra132_tsensor_config = {
    .tall = 16300,
    .tiddq_en = 1,
    .ten_count = 1,
    .tsample = 120,
    .tsample_ate = 480,
    };
    static const struct tegra_tsensor_group tegra132_tsensor_group_cpu = {
    .id = TEGRA124_SOCTHERM_SENSOR_CPU,
    .name = "cpu",
    .sensor_temp_offset = SENSOR_TEMP1,
    .sensor_temp_mask = SENSOR_TEMP1_CPU_TEMP_MASK,
    .pdiv = 8,
    .pdiv_ate = 8,
    .pdiv_mask = SENSOR_PDIV_CPU_MASK,
    .pllx_hotspot_diff = 10,
    .pllx_hotspot_mask = SENSOR_HOTSPOT_CPU_MASK,
    .thermtrip_any_en_mask = TEGRA132_THERMTRIP_ANY_EN_MASK,
    .thermtrip_enable_mask = TEGRA132_THERMTRIP_CPU_EN_MASK,
    .thermtrip_threshold_mask = TEGRA132_THERMTRIP_CPU_THRESH_MASK,
    .thermctl_isr_mask = THERM_IRQ_CPU_MASK,
    .thermctl_lvl0_offset = THERMCTL_LEVEL0_GROUP_CPU,
    .thermctl_lvl0_up_thresh_mask = TEGRA132_THERMCTL_LVL0_UP_THRESH_MASK,
    .thermctl_lvl0_dn_thresh_mask = TEGRA132_THERMCTL_LVL0_DN_THRESH_MASK,
    };
    static const struct tegra_tsensor_group tegra132_tsensor_group_gpu = {
    .id = TEGRA124_SOCTHERM_SENSOR_GPU,
    .name = "gpu",
    .sensor_temp_offset = SENSOR_TEMP1,
    .sensor_temp_mask = SENSOR_TEMP1_GPU_TEMP_MASK,
    .pdiv = 8,
    .pdiv_ate = 8,
    .pdiv_mask = SENSOR_PDIV_GPU_MASK,
    .pllx_hotspot_diff = 5,
    .pllx_hotspot_mask = SENSOR_HOTSPOT_GPU_MASK,
    .thermtrip_any_en_mask = TEGRA132_THERMTRIP_ANY_EN_MASK,
    .thermtrip_enable_mask = TEGRA132_THERMTRIP_GPU_EN_MASK,
    .thermtrip_threshold_mask = TEGRA132_THERMTRIP_GPUMEM_THRESH_MASK,
    .thermctl_isr_mask = THERM_IRQ_GPU_MASK,
    .thermctl_lvl0_offset = THERMCTL_LEVEL0_GROUP_GPU,
    .thermctl_lvl0_up_thresh_mask = TEGRA132_THERMCTL_LVL0_UP_THRESH_MASK,
    .thermctl_lvl0_dn_thresh_mask = TEGRA132_THERMCTL_LVL0_DN_THRESH_MASK,
    };
    static const struct tegra_tsensor_group tegra132_tsensor_group_pll = {
    .id = TEGRA124_SOCTHERM_SENSOR_PLLX,
    .name = "pll",
    .sensor_temp_offset = SENSOR_TEMP2,
    .sensor_temp_mask = SENSOR_TEMP2_PLLX_TEMP_MASK,
    .pdiv = 8,
    .pdiv_ate = 8,
    .pdiv_mask = SENSOR_PDIV_PLLX_MASK,
    .thermtrip_any_en_mask = TEGRA132_THERMTRIP_ANY_EN_MASK,
    .thermtrip_enable_mask = TEGRA132_THERMTRIP_TSENSE_EN_MASK,
    .thermtrip_threshold_mask = TEGRA132_THERMTRIP_TSENSE_THRESH_MASK,
    .thermctl_isr_mask = THERM_IRQ_TSENSE_MASK,
    .thermctl_lvl0_offset = THERMCTL_LEVEL0_GROUP_TSENSE,
    .thermctl_lvl0_up_thresh_mask = TEGRA132_THERMCTL_LVL0_UP_THRESH_MASK,
    .thermctl_lvl0_dn_thresh_mask = TEGRA132_THERMCTL_LVL0_DN_THRESH_MASK,
    };
    static const struct tegra_tsensor_group tegra132_tsensor_group_mem = {
    .id = TEGRA124_SOCTHERM_SENSOR_MEM,
    .name = "mem",
    .sensor_temp_offset = SENSOR_TEMP2,
    .sensor_temp_mask = SENSOR_TEMP2_MEM_TEMP_MASK,
    .pdiv = 8,
    .pdiv_ate = 8,
    .pdiv_mask = SENSOR_PDIV_MEM_MASK,
    .pllx_hotspot_diff = 0,
    .pllx_hotspot_mask = SENSOR_HOTSPOT_MEM_MASK,
    .thermtrip_any_en_mask = TEGRA132_THERMTRIP_ANY_EN_MASK,
    .thermtrip_enable_mask = TEGRA132_THERMTRIP_MEM_EN_MASK,
    .thermtrip_threshold_mask = TEGRA132_THERMTRIP_GPUMEM_THRESH_MASK,
    .thermctl_isr_mask = THERM_IRQ_MEM_MASK,
    .thermctl_lvl0_offset = THERMCTL_LEVEL0_GROUP_MEM,
    .thermctl_lvl0_up_thresh_mask = TEGRA132_THERMCTL_LVL0_UP_THRESH_MASK,
    .thermctl_lvl0_dn_thresh_mask = TEGRA132_THERMCTL_LVL0_DN_THRESH_MASK,
    };
    static const struct tegra_tsensor_group *tegra132_tsensor_groups[] = {
    &tegra132_tsensor_group_cpu,
    &tegra132_tsensor_group_gpu,
    &tegra132_tsensor_group_pll,
    &tegra132_tsensor_group_mem,
    };
    static struct tegra_tsensor tegra132_tsensors[] = {
    {
    .name = "cpu0",
    .base = 0xc0,
    .config = &tegra132_tsensor_config,
    .calib_fuse_offset = 0x098,
    .fuse_corr_alpha = 1126600,
    .fuse_corr_beta = -9433500,
    .group = &tegra132_tsensor_group_cpu,
    }, {
    .name = "cpu1",
    .base = 0xe0,
    .config = &tegra132_tsensor_config,
    .calib_fuse_offset = 0x084,
    .fuse_corr_alpha = 1110800,
    .fuse_corr_beta = -7383000,
    .group = &tegra132_tsensor_group_cpu,
    }, {
    .name = "cpu2",
    .base = 0x100,
    .config = &tegra132_tsensor_config,
    .calib_fuse_offset = 0x088,
    .fuse_corr_alpha = 1113800,
    .fuse_corr_beta = -6215200,
    .group = &tegra132_tsensor_group_cpu,
    }, {
    .name = "cpu3",
    .base = 0x120,
    .config = &tegra132_tsensor_config,
    .calib_fuse_offset = 0x12c,
    .fuse_corr_alpha = 1129600,
    .fuse_corr_beta = -8196100,
    .group = &tegra132_tsensor_group_cpu,
    }, {
    .name = "mem0",
    .base = 0x140,
    .config = &tegra132_tsensor_config,
    .calib_fuse_offset = 0x158,
    .fuse_corr_alpha = 1132900,
    .fuse_corr_beta = -6755300,
    .group = &tegra132_tsensor_group_mem,
    }, {
    .name = "mem1",
    .base = 0x160,
    .config = &tegra132_tsensor_config,
    .calib_fuse_offset = 0x15c,
    .fuse_corr_alpha = 1142300,
    .fuse_corr_beta = -7374200,
    .group = &tegra132_tsensor_group_mem,
    }, {
    .name = "gpu",
    .base = 0x180,
    .config = &tegra132_tsensor_config,
    .calib_fuse_offset = 0x154,
    .fuse_corr_alpha = 1125100,
    .fuse_corr_beta = -6350400,
    .group = &tegra132_tsensor_group_gpu,
    }, {
    .name = "pllx",
    .base = 0x1a0,
    .config = &tegra132_tsensor_config,
    .calib_fuse_offset = 0x160,
    .fuse_corr_alpha = 1118100,
    .fuse_corr_beta = -8208800,
    .group = &tegra132_tsensor_group_pll,
    },
    };
//
// Mask/shift bits in FUSE_TSENSOR_COMMON and
// FUSE_TSENSOR_COMMON, which are described in
// tegra_soctherm_fuse.c
//
    static const struct tegra_soctherm_fuse tegra132_soctherm_fuse = {
    .fuse_base_cp_mask = 0x3ff,
    .fuse_base_cp_shift = 0,
    .fuse_shift_cp_mask = 0x3f,
    .fuse_shift_cp_shift = 0,
    .fuse_base_ft_mask = 0x7ff << 10,
    .fuse_base_ft_shift = 10,
    .fuse_shift_ft_mask = 0x1f << 21,
    .fuse_shift_ft_shift = 21,
    .fuse_common_reg = FUSE_TSENSOR_COMMON,
    .fuse_spare_realignment = 0x1fc,
    .nominal_calib_ft = 105,
    };
    const struct tegra_soctherm_soc tegra132_soctherm = {
    .tsensors = tegra132_tsensors,
    .num_tsensors = ARRAY_SIZE(tegra132_tsensors),
    .ttgs = tegra132_tsensor_groups,
    .num_ttgs = ARRAY_SIZE(tegra132_tsensor_groups),
    .tfuse = &tegra132_soctherm_fuse,
    .thresh_grain = TEGRA132_THRESH_GRAIN,
    .bptt = TEGRA132_BPTT,
    .use_ccroc = true,
    };
