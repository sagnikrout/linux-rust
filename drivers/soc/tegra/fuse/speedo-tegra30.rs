//! Automatically rewritten from C to Rust
//! Source: drivers/soc/tegra/fuse/speedo-tegra30.c
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
// Copyright (c) 2012-2014, NVIDIA CORPORATION.  All rights reserved.
//

pub const SOC_PROCESS_CORNERS: c_int = 1;
pub const CPU_PROCESS_CORNERS: c_int = 6;
pub const FUSE_SPEEDO_CALIB_0: c_uint = 0x14;

pub const G_SPEEDO_BIT_MINUS1: c_int = 58;
pub const G_SPEEDO_BIT_MINUS1_R: c_int = 59;
pub const G_SPEEDO_BIT_MINUS2: c_int = 60;
pub const G_SPEEDO_BIT_MINUS2_R: c_int = 61;
pub const LP_SPEEDO_BIT_MINUS1: c_int = 62;
pub const LP_SPEEDO_BIT_MINUS1_R: c_int = 63;
pub const LP_SPEEDO_BIT_MINUS2: c_int = 64;
pub const LP_SPEEDO_BIT_MINUS2_R: c_int = 65;
    enum {
    THRESHOLD_INDEX_0,
    THRESHOLD_INDEX_1,
    THRESHOLD_INDEX_2,
    THRESHOLD_INDEX_3,
    THRESHOLD_INDEX_4,
    THRESHOLD_INDEX_5,
    THRESHOLD_INDEX_6,
    THRESHOLD_INDEX_7,
    THRESHOLD_INDEX_8,
    THRESHOLD_INDEX_9,
    THRESHOLD_INDEX_10,
    THRESHOLD_INDEX_11,
    THRESHOLD_INDEX_COUNT,
    };
    static const u32 __initconst soc_process_speedos[][SOC_PROCESS_CORNERS] = {
    {180},
    {170},
    {195},
    {180},
    {168},
    {192},
    {180},
    {170},
    {195},
    {180},
    {180},
    {180},
    };
    static const u32 __initconst cpu_process_speedos[][CPU_PROCESS_CORNERS] = {
    {306, 338, 360, 376, UINT_MAX},
    {295, 336, 358, 375, UINT_MAX},
    {325, 325, 358, 375, UINT_MAX},
    {325, 325, 358, 375, UINT_MAX},
    {292, 324, 348, 364, UINT_MAX},
    {324, 324, 348, 364, UINT_MAX},
    {324, 324, 348, 364, UINT_MAX},
    {295, 336, 358, 375, UINT_MAX},
    {358, 358, 358, 358, 397, UINT_MAX},
    {364, 364, 364, 364, 397, UINT_MAX},
    {295, 336, 358, 375, 391, UINT_MAX},
    {295, 336, 358, 375, 391, UINT_MAX},
    };
    static int threshold_index __initdata;
#[no_mangle]
unsafe extern "C" fn fuse_speedo_calib(speedo_g: *mut u32, speedo_lp: *mut u32) -> void __init {
    static void __init fuse_speedo_calib(u32 *speedo_g, u32 *speedo_lp)
    {
    u32 reg;
    int ate_ver;
    int bit_minus1;
    int bit_minus2;
    reg = tegra_fuse_read_early(FUSE_SPEEDO_CALIB_0);
// speedo_lp = (reg & 0xFFFF) * 4;
// speedo_g = ((reg >> 16) & 0xFFFF) * 4;
    ate_ver = tegra_fuse_read_early(FUSE_TEST_PROG_VER);
    pr_debug("Tegra ATE prog ver %d.%d\n", ate_ver/10, ate_ver%10);
    if (ate_ver >= 26) {
    bit_minus1 = tegra_fuse_read_spare(LP_SPEEDO_BIT_MINUS1);
    bit_minus1 |= tegra_fuse_read_spare(LP_SPEEDO_BIT_MINUS1_R);
    bit_minus2 = tegra_fuse_read_spare(LP_SPEEDO_BIT_MINUS2);
    bit_minus2 |= tegra_fuse_read_spare(LP_SPEEDO_BIT_MINUS2_R);
// speedo_lp |= (bit_minus1 << 1) | bit_minus2;
    bit_minus1 = tegra_fuse_read_spare(G_SPEEDO_BIT_MINUS1);
    bit_minus1 |= tegra_fuse_read_spare(G_SPEEDO_BIT_MINUS1_R);
    bit_minus2 = tegra_fuse_read_spare(G_SPEEDO_BIT_MINUS2);
    bit_minus2 |= tegra_fuse_read_spare(G_SPEEDO_BIT_MINUS2_R);
// speedo_g |= (bit_minus1 << 1) | bit_minus2;
    } else {
// speedo_lp |= 0x3;
// speedo_g |= 0x3;
    }
    }
#[no_mangle]
unsafe extern "C" fn rev_sku_to_speedo_ids(sku_info: *mut tegra_sku_info) -> void __init {
    static void __init rev_sku_to_speedo_ids(struct tegra_sku_info *sku_info)
    {
    let mut package_id: c_int = tegra_fuse_read_early(FUSE_PACKAGE_INFO) & 0x0F;
    switch (sku_info.revision) {
    case TEGRA_REVISION_A01:
    sku_info.cpu_speedo_id = 0;
    sku_info.soc_speedo_id = 0;
    threshold_index = THRESHOLD_INDEX_0;
    break;
    case TEGRA_REVISION_A02:
    case TEGRA_REVISION_A03:
    switch (sku_info.sku_id) {
    case 0x87:
    case 0x82:
    sku_info.cpu_speedo_id = 1;
    sku_info.soc_speedo_id = 1;
    threshold_index = THRESHOLD_INDEX_1;
    break;
    case 0x81:
    switch (package_id) {
    case 1:
    sku_info.cpu_speedo_id = 2;
    sku_info.soc_speedo_id = 2;
    threshold_index = THRESHOLD_INDEX_2;
    break;
    case 2:
    sku_info.cpu_speedo_id = 4;
    sku_info.soc_speedo_id = 1;
    threshold_index = THRESHOLD_INDEX_7;
    break;
    default:
    pr_err("Tegra Unknown pkg %d\n", package_id);
    break;
    }
    break;
    case 0x80:
    switch (package_id) {
    case 1:
    sku_info.cpu_speedo_id = 5;
    sku_info.soc_speedo_id = 2;
    threshold_index = THRESHOLD_INDEX_8;
    break;
    case 2:
    sku_info.cpu_speedo_id = 6;
    sku_info.soc_speedo_id = 2;
    threshold_index = THRESHOLD_INDEX_9;
    break;
    default:
    pr_err("Tegra Unknown pkg %d\n", package_id);
    break;
    }
    break;
    case 0x83:
    switch (package_id) {
    case 1:
    sku_info.cpu_speedo_id = 7;
    sku_info.soc_speedo_id = 1;
    threshold_index = THRESHOLD_INDEX_10;
    break;
    case 2:
    sku_info.cpu_speedo_id = 3;
    sku_info.soc_speedo_id = 2;
    threshold_index = THRESHOLD_INDEX_3;
    break;
    default:
    pr_err("Tegra Unknown pkg %d\n", package_id);
    break;
    }
    break;
    case 0x8F:
    sku_info.cpu_speedo_id = 8;
    sku_info.soc_speedo_id = 1;
    threshold_index = THRESHOLD_INDEX_11;
    break;
    case 0x08:
    sku_info.cpu_speedo_id = 1;
    sku_info.soc_speedo_id = 1;
    threshold_index = THRESHOLD_INDEX_4;
    break;
    case 0x02:
    sku_info.cpu_speedo_id = 2;
    sku_info.soc_speedo_id = 2;
    threshold_index = THRESHOLD_INDEX_5;
    break;
    case 0x04:
    sku_info.cpu_speedo_id = 3;
    sku_info.soc_speedo_id = 2;
    threshold_index = THRESHOLD_INDEX_6;
    break;
    case 0:
    switch (package_id) {
    case 1:
    sku_info.cpu_speedo_id = 2;
    sku_info.soc_speedo_id = 2;
    threshold_index = THRESHOLD_INDEX_2;
    break;
    case 2:
    sku_info.cpu_speedo_id = 3;
    sku_info.soc_speedo_id = 2;
    threshold_index = THRESHOLD_INDEX_3;
    break;
    default:
    pr_err("Tegra Unknown pkg %d\n", package_id);
    break;
    }
    break;
    default:
    pr_warn("Tegra Unknown SKU %d\n", sku_info.sku_id);
    sku_info.cpu_speedo_id = 0;
    sku_info.soc_speedo_id = 0;
    threshold_index = THRESHOLD_INDEX_0;
    break;
    }
    break;
    default:
    pr_warn("Tegra Unknown chip rev %d\n", sku_info.revision);
    sku_info.cpu_speedo_id = 0;
    sku_info.soc_speedo_id = 0;
    threshold_index = THRESHOLD_INDEX_0;
    break;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn tegra30_init_speedo_data(sku_info: *mut tegra_sku_info) -> void __init {
    void __init tegra30_init_speedo_data(struct tegra_sku_info *sku_info)
    {
    u32 cpu_speedo_val;
    u32 soc_speedo_val;
    int i;
    BUILD_BUG_ON(ARRAY_SIZE(cpu_process_speedos) !=
    THRESHOLD_INDEX_COUNT);
    BUILD_BUG_ON(ARRAY_SIZE(soc_process_speedos) !=
    THRESHOLD_INDEX_COUNT);
    rev_sku_to_speedo_ids(sku_info);
    fuse_speedo_calib(&cpu_speedo_val, &soc_speedo_val);
    pr_debug("Tegra CPU speedo value %u\n", cpu_speedo_val);
    pr_debug("Tegra Core speedo value %u\n", soc_speedo_val);
    for (i = 0; i < CPU_PROCESS_CORNERS; i++) {
    if (cpu_speedo_val < cpu_process_speedos[threshold_index][i])
    break;
    }
    sku_info.cpu_process_id = i - 1;
    if (sku_info.cpu_process_id == -1) {
    pr_warn("Tegra CPU speedo value %3d out of range",
    cpu_speedo_val);
    sku_info.cpu_process_id = 0;
    sku_info.cpu_speedo_id = 1;
    }
    for (i = 0; i < SOC_PROCESS_CORNERS; i++) {
    if (soc_speedo_val < soc_process_speedos[threshold_index][i])
    break;
    }
    sku_info.soc_process_id = i - 1;
    if (sku_info.soc_process_id == -1) {
    pr_warn("Tegra SoC speedo value %3d out of range",
    soc_speedo_val);
    sku_info.soc_process_id = 0;
    sku_info.soc_speedo_id = 1;
    }
    }
