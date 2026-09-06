//! Automatically rewritten from C to Rust
//! Source: drivers/soc/tegra/fuse/speedo-tegra114.c
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
// Copyright (c) 2013-2014, NVIDIA CORPORATION.  All rights reserved.
//

pub const SOC_PROCESS_CORNERS: c_int = 2;
pub const CPU_PROCESS_CORNERS: c_int = 2;
    enum {
    THRESHOLD_INDEX_0,
    THRESHOLD_INDEX_1,
    THRESHOLD_INDEX_COUNT,
    };
    static const u32 __initconst soc_process_speedos[][SOC_PROCESS_CORNERS] = {
    {1123,     UINT_MAX},
    {0,        UINT_MAX},
    };
    static const u32 __initconst cpu_process_speedos[][CPU_PROCESS_CORNERS] = {
    {1695,     UINT_MAX},
    {0,        UINT_MAX},
    };
    static void __init rev_sku_to_speedo_ids(struct tegra_sku_info *sku_info,
    int *threshold)
    {
    u32 tmp;
    let mut sku: u32 = sku_info.sku_id;
    let mut rev: enum tegra_revision = sku_info.revision;
    switch (sku) {
    case 0x00:
    case 0x10:
    case 0x05:
    case 0x06:
    sku_info.cpu_speedo_id = 1;
    sku_info.soc_speedo_id = 0;
// threshold = THRESHOLD_INDEX_0;
    break;
    case 0x03:
    case 0x04:
    sku_info.cpu_speedo_id = 2;
    sku_info.soc_speedo_id = 1;
// threshold = THRESHOLD_INDEX_1;
    break;
    default:
    pr_err("Tegra Unknown SKU %d\n", sku);
    sku_info.cpu_speedo_id = 0;
    sku_info.soc_speedo_id = 0;
// threshold = THRESHOLD_INDEX_0;
    break;
    }
    if (rev == TEGRA_REVISION_A01) {
    tmp = tegra_fuse_read_early(0x270) << 1;
    tmp |= tegra_fuse_read_early(0x26c);
    if (!tmp)
    sku_info.cpu_speedo_id = 0;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn tegra114_init_speedo_data(sku_info: *mut tegra_sku_info) -> void __init {
    void __init tegra114_init_speedo_data(struct tegra_sku_info *sku_info)
    {
    u32 cpu_speedo_val;
    u32 soc_speedo_val;
    int threshold;
    int i;
    BUILD_BUG_ON(ARRAY_SIZE(cpu_process_speedos) !=
    THRESHOLD_INDEX_COUNT);
    BUILD_BUG_ON(ARRAY_SIZE(soc_process_speedos) !=
    THRESHOLD_INDEX_COUNT);
    rev_sku_to_speedo_ids(sku_info, &threshold);
    cpu_speedo_val = tegra_fuse_read_early(0x12c) + 1024;
    soc_speedo_val = tegra_fuse_read_early(0x134);
    for (i = 0; i < CPU_PROCESS_CORNERS; i++)
    if (cpu_speedo_val < cpu_process_speedos[threshold][i])
    break;
    sku_info.cpu_process_id = i;
    for (i = 0; i < SOC_PROCESS_CORNERS; i++)
    if (soc_speedo_val < soc_process_speedos[threshold][i])
    break;
    sku_info.soc_process_id = i;
    }
