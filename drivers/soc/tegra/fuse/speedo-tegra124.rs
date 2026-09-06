//! Automatically rewritten from C to Rust
//! Source: drivers/soc/tegra/fuse/speedo-tegra124.c
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

pub const CPU_PROCESS_CORNERS: c_int = 2;
pub const GPU_PROCESS_CORNERS: c_int = 2;
pub const SOC_PROCESS_CORNERS: c_int = 2;
pub const FUSE_CPU_SPEEDO_0: c_uint = 0x14;
pub const FUSE_CPU_SPEEDO_1: c_uint = 0x2c;
pub const FUSE_CPU_SPEEDO_2: c_uint = 0x30;
pub const FUSE_SOC_SPEEDO_0: c_uint = 0x34;
pub const FUSE_SOC_SPEEDO_1: c_uint = 0x38;
pub const FUSE_SOC_SPEEDO_2: c_uint = 0x3c;
pub const FUSE_CPU_IDDQ: c_uint = 0x18;
pub const FUSE_SOC_IDDQ: c_uint = 0x40;
pub const FUSE_GPU_IDDQ: c_uint = 0x128;
pub const FUSE_FT_REV: c_uint = 0x28;
    enum {
    THRESHOLD_INDEX_0,
    THRESHOLD_INDEX_1,
    THRESHOLD_INDEX_COUNT,
    };
    static const u32 __initconst cpu_process_speedos[][CPU_PROCESS_CORNERS] = {
    {2190,	UINT_MAX},
    {0,	UINT_MAX},
    };
    static const u32 __initconst gpu_process_speedos[][GPU_PROCESS_CORNERS] = {
    {1965,	UINT_MAX},
    {0,	UINT_MAX},
    };
    static const u32 __initconst soc_process_speedos[][SOC_PROCESS_CORNERS] = {
    {2101,	UINT_MAX},
    {0,	UINT_MAX},
    };
    static void __init rev_sku_to_speedo_ids(struct tegra_sku_info *sku_info,
    int *threshold)
    {
    let mut sku: c_int = sku_info.sku_id;
// Assign to default
    sku_info.cpu_speedo_id = 0;
    sku_info.soc_speedo_id = 0;
    sku_info.gpu_speedo_id = 0;
// threshold = THRESHOLD_INDEX_0;
    switch (sku) {
    case 0x00: /* Eng sku */
    case 0x0F:
    case 0x23:
// Using the default
    break;
    case 0x83:
    sku_info.cpu_speedo_id = 2;
    break;
    case 0x1F:
    case 0x87:
    case 0x27:
    sku_info.cpu_speedo_id = 2;
    sku_info.soc_speedo_id = 0;
    sku_info.gpu_speedo_id = 1;
// threshold = THRESHOLD_INDEX_0;
    break;
    case 0x81:
    case 0x21:
    case 0x07:
    sku_info.cpu_speedo_id = 1;
    sku_info.soc_speedo_id = 1;
    sku_info.gpu_speedo_id = 1;
// threshold = THRESHOLD_INDEX_1;
    break;
    case 0x49:
    case 0x4A:
    case 0x48:
    sku_info.cpu_speedo_id = 4;
    sku_info.soc_speedo_id = 2;
    sku_info.gpu_speedo_id = 3;
// threshold = THRESHOLD_INDEX_1;
    break;
    default:
    pr_err("Tegra Unknown SKU %d\n", sku);
// Using the default for the error case
    break;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn tegra124_init_speedo_data(sku_info: *mut tegra_sku_info) -> void __init {
    void __init tegra124_init_speedo_data(struct tegra_sku_info *sku_info)
    {
    int i, threshold, soc_speedo_0_value;
    BUILD_BUG_ON(ARRAY_SIZE(cpu_process_speedos) !=
    THRESHOLD_INDEX_COUNT);
    BUILD_BUG_ON(ARRAY_SIZE(gpu_process_speedos) !=
    THRESHOLD_INDEX_COUNT);
    BUILD_BUG_ON(ARRAY_SIZE(soc_process_speedos) !=
    THRESHOLD_INDEX_COUNT);
    sku_info.cpu_speedo_value = tegra_fuse_read_early(FUSE_CPU_SPEEDO_0);
    if (sku_info.cpu_speedo_value == 0) {
    pr_warn("Tegra Warning: Speedo value not fused.\n");
    WARN_ON(1);
    return;
    }
// GPU Speedo is stored in CPU_SPEEDO_2
    sku_info.gpu_speedo_value = tegra_fuse_read_early(FUSE_CPU_SPEEDO_2);
    soc_speedo_0_value = tegra_fuse_read_early(FUSE_SOC_SPEEDO_0);
    rev_sku_to_speedo_ids(sku_info, &threshold);
    sku_info.cpu_iddq_value = tegra_fuse_read_early(FUSE_CPU_IDDQ);
    for (i = 0; i < GPU_PROCESS_CORNERS; i++)
    if (sku_info.gpu_speedo_value <
    gpu_process_speedos[threshold][i])
    break;
    sku_info.gpu_process_id = i;
    for (i = 0; i < CPU_PROCESS_CORNERS; i++)
    if (sku_info.cpu_speedo_value <
    cpu_process_speedos[threshold][i])
    break;
    sku_info.cpu_process_id = i;
    for (i = 0; i < SOC_PROCESS_CORNERS; i++)
    if (soc_speedo_0_value <
    soc_process_speedos[threshold][i])
    break;
    sku_info.soc_process_id = i;
    pr_debug("Tegra GPU Speedo ID=%d, Speedo Value=%d\n",
    sku_info.gpu_speedo_id, sku_info.gpu_speedo_value);
    }
