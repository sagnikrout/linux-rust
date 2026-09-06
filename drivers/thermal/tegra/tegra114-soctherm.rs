//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/tegra/tegra114-soctherm.c
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
// Copyright (c) 2024, Svyatoslav Ryhel <clamor95@gmail.com>
//

pub const TEGRA114_THERMTRIP_TSENSE_THRESH_MASK: c_uint = 0xff;

pub const TEGRA114_THRESH_GRAIN: c_int = 1000;
pub const TEGRA114_BPTT: c_int = 8;
    static const struct tegra_tsensor_configuration tegra114_tsensor_config = {
    .tall = 16300,
    .tiddq_en = 1,
    .ten_count = 1,
    .tsample = 163,
    .tsample_ate = 655,
    };
    static const struct tegra_tsensor_group tegra114_tsensor_group_cpu = {
    .id = TEGRA114_SOCTHERM_SENSOR_CPU,
    .name = "cpu",
    .sensor_temp_offset = SENSOR_TEMP1,
    .sensor_temp_mask = SENSOR_TEMP1_CPU_TEMP_MASK,
    .pdiv = 10,
    .pdiv_ate = 10,
    .pdiv_mask = SENSOR_PDIV_CPU_MASK,
    .pllx_hotspot_diff = 6,
    .pllx_hotspot_mask = SENSOR_HOTSPOT_CPU_MASK,
    .thermtrip_any_en_mask = TEGRA114_THERMTRIP_ANY_EN_MASK,
    .thermtrip_enable_mask = TEGRA114_THERMTRIP_CPU_EN_MASK,
    .thermtrip_threshold_mask = TEGRA114_THERMTRIP_CPU_THRESH_MASK,
    .thermctl_isr_mask = THERM_IRQ_CPU_MASK,
    .thermctl_lvl0_offset = THERMCTL_LEVEL0_GROUP_CPU,
    .thermctl_lvl0_up_thresh_mask = TEGRA114_THERMCTL_LVL0_UP_THRESH_MASK,
    .thermctl_lvl0_dn_thresh_mask = TEGRA114_THERMCTL_LVL0_DN_THRESH_MASK,
    };
    static const struct tegra_tsensor_group tegra114_tsensor_group_gpu = {
    .id = TEGRA114_SOCTHERM_SENSOR_GPU,
    .name = "gpu",
    .sensor_temp_offset = SENSOR_TEMP1,
    .sensor_temp_mask = SENSOR_TEMP1_GPU_TEMP_MASK,
    .pdiv = 10,
    .pdiv_ate = 10,
    .pdiv_mask = SENSOR_PDIV_GPU_MASK,
    .pllx_hotspot_diff = 6,
    .pllx_hotspot_mask = SENSOR_HOTSPOT_GPU_MASK,
    .thermtrip_any_en_mask = TEGRA114_THERMTRIP_ANY_EN_MASK,
    .thermtrip_enable_mask = TEGRA114_THERMTRIP_GPU_EN_MASK,
    .thermtrip_threshold_mask = TEGRA114_THERMTRIP_GPUMEM_THRESH_MASK,
    .thermctl_isr_mask = THERM_IRQ_GPU_MASK,
    .thermctl_lvl0_offset = THERMCTL_LEVEL0_GROUP_GPU,
    .thermctl_lvl0_up_thresh_mask = TEGRA114_THERMCTL_LVL0_UP_THRESH_MASK,
    .thermctl_lvl0_dn_thresh_mask = TEGRA114_THERMCTL_LVL0_DN_THRESH_MASK,
    };
    static const struct tegra_tsensor_group tegra114_tsensor_group_pll = {
    .id = TEGRA114_SOCTHERM_SENSOR_PLLX,
    .name = "pll",
    .sensor_temp_offset = SENSOR_TEMP2,
    .sensor_temp_mask = SENSOR_TEMP2_PLLX_TEMP_MASK,
    .pdiv = 10,
    .pdiv_ate = 10,
    .pdiv_mask = SENSOR_PDIV_PLLX_MASK,
    .thermtrip_any_en_mask = TEGRA114_THERMTRIP_ANY_EN_MASK,
    .thermtrip_enable_mask = TEGRA114_THERMTRIP_TSENSE_EN_MASK,
    .thermtrip_threshold_mask = TEGRA114_THERMTRIP_TSENSE_THRESH_MASK,
    .thermctl_isr_mask = THERM_IRQ_TSENSE_MASK,
    .thermctl_lvl0_offset = THERMCTL_LEVEL0_GROUP_TSENSE,
    .thermctl_lvl0_up_thresh_mask = TEGRA114_THERMCTL_LVL0_UP_THRESH_MASK,
    .thermctl_lvl0_dn_thresh_mask = TEGRA114_THERMCTL_LVL0_DN_THRESH_MASK,
    };
    static const struct tegra_tsensor_group tegra114_tsensor_group_mem = {
    .id = TEGRA114_SOCTHERM_SENSOR_MEM,
    .name = "mem",
    .sensor_temp_offset = SENSOR_TEMP2,
    .sensor_temp_mask = SENSOR_TEMP2_MEM_TEMP_MASK,
    .pdiv = 10,
    .pdiv_ate = 10,
    .pdiv_mask = SENSOR_PDIV_MEM_MASK,
    .pllx_hotspot_diff = 0,
    .pllx_hotspot_mask = SENSOR_HOTSPOT_MEM_MASK,
    .thermtrip_any_en_mask = TEGRA114_THERMTRIP_ANY_EN_MASK,
    .thermtrip_enable_mask = TEGRA114_THERMTRIP_MEM_EN_MASK,
    .thermtrip_threshold_mask = TEGRA114_THERMTRIP_GPUMEM_THRESH_MASK,
    .thermctl_isr_mask = THERM_IRQ_MEM_MASK,
    .thermctl_lvl0_offset = THERMCTL_LEVEL0_GROUP_MEM,
    .thermctl_lvl0_up_thresh_mask = TEGRA114_THERMCTL_LVL0_UP_THRESH_MASK,
    .thermctl_lvl0_dn_thresh_mask = TEGRA114_THERMCTL_LVL0_DN_THRESH_MASK,
    };
    static const struct tegra_tsensor_group *tegra114_tsensor_groups[] = {
    &tegra114_tsensor_group_cpu,
    &tegra114_tsensor_group_gpu,
    &tegra114_tsensor_group_pll,
    &tegra114_tsensor_group_mem,
    };
    static const struct tegra_tsensor tegra114_tsensors[] = {
    {
    .name = "cpu0",
    .base = 0xc0,
    .config = &tegra114_tsensor_config,
    .calib_fuse_offset = 0x098,
    .fuse_corr_alpha = 1196400,
    .fuse_corr_beta = -13600000,
    .group = &tegra114_tsensor_group_cpu,
    }, {
    .name = "cpu1",
    .base = 0xe0,
    .config = &tegra114_tsensor_config,
    .calib_fuse_offset = 0x084,
    .fuse_corr_alpha = 1196400,
    .fuse_corr_beta = -13600000,
    .group = &tegra114_tsensor_group_cpu,
    }, {
    .name = "cpu2",
    .base = 0x100,
    .config = &tegra114_tsensor_config,
    .calib_fuse_offset = 0x088,
    .fuse_corr_alpha = 1196400,
    .fuse_corr_beta = -13600000,
    .group = &tegra114_tsensor_group_cpu,
    }, {
    .name = "cpu3",
    .base = 0x120,
    .config = &tegra114_tsensor_config,
    .calib_fuse_offset = 0x12c,
    .fuse_corr_alpha = 1196400,
    .fuse_corr_beta = -13600000,
    .group = &tegra114_tsensor_group_cpu,
    }, {
    .name = "mem0",
    .base = 0x140,
    .config = &tegra114_tsensor_config,
    .calib_fuse_offset = 0x158,
    .fuse_corr_alpha = 1000000,
    .fuse_corr_beta = 0,
    .group = &tegra114_tsensor_group_mem,
    }, {
    .name = "mem1",
    .base = 0x160,
    .config = &tegra114_tsensor_config,
    .calib_fuse_offset = 0x15c,
    .fuse_corr_alpha = 1000000,
    .fuse_corr_beta = 0,
    .group = &tegra114_tsensor_group_mem,
    }, {
    .name = "gpu",
    .base = 0x180,
    .config = &tegra114_tsensor_config,
    .calib_fuse_offset = 0x154,
    .fuse_corr_alpha = 1124500,
    .fuse_corr_beta = -9793100,
    .group = &tegra114_tsensor_group_gpu,
    }, {
    .name = "pllx",
    .base = 0x1a0,
    .config = &tegra114_tsensor_config,
    .calib_fuse_offset = 0x160,
    .fuse_corr_alpha = 1224200,
    .fuse_corr_beta = -14665000,
    .group = &tegra114_tsensor_group_pll,
    },
    };
    static const struct tegra_soctherm_fuse tegra114_soctherm_fuse = {
    .fuse_base_cp_mask = 0x3ff,
    .fuse_base_cp_shift = 0,
    .fuse_shift_cp_mask = 0x3f << 10,
    .fuse_shift_cp_shift = 10,
    .fuse_base_ft_mask = 0x7ff << 16,
    .fuse_base_ft_shift = 16,
    .fuse_shift_ft_mask = 0x1f << 27,
    .fuse_shift_ft_shift = 27,
    .fuse_common_reg = FUSE_VSENSOR_CALIB,
    .fuse_spare_realignment = 0,
    .nominal_calib_ft = 90,
    };
    const struct tegra_soctherm_soc tegra114_soctherm = {
    .tsensors = tegra114_tsensors,
    .num_tsensors = ARRAY_SIZE(tegra114_tsensors),
    .ttgs = tegra114_tsensor_groups,
    .num_ttgs = ARRAY_SIZE(tegra114_tsensor_groups),
    .tfuse = &tegra114_soctherm_fuse,
    .thresh_grain = TEGRA114_THRESH_GRAIN,
    .bptt = TEGRA114_BPTT,
    .use_ccroc = false,
    };
