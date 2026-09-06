//! Automatically rewritten from C to Rust
//! Source: drivers/soc/tegra/fuse/speedo-tegra210.c
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
// Copyright (c) 2013-2015, NVIDIA CORPORATION.  All rights reserved.
//

pub const CPU_PROCESS_CORNERS: c_int = 2;
pub const GPU_PROCESS_CORNERS: c_int = 2;
pub const SOC_PROCESS_CORNERS: c_int = 3;
pub const FUSE_CPU_SPEEDO_0: c_uint = 0x014;
pub const FUSE_CPU_SPEEDO_1: c_uint = 0x02c;
pub const FUSE_CPU_SPEEDO_2: c_uint = 0x030;
pub const FUSE_SOC_SPEEDO_0: c_uint = 0x034;
pub const FUSE_SOC_SPEEDO_1: c_uint = 0x038;
pub const FUSE_SOC_SPEEDO_2: c_uint = 0x03c;
pub const FUSE_CPU_IDDQ: c_uint = 0x018;
pub const FUSE_SOC_IDDQ: c_uint = 0x040;
pub const FUSE_GPU_IDDQ: c_uint = 0x128;
pub const FUSE_FT_REV: c_uint = 0x028;
    enum {
    THRESHOLD_INDEX_0,
    THRESHOLD_INDEX_1,
    THRESHOLD_INDEX_COUNT,
    };
    static const u32 __initconst cpu_process_speedos[][CPU_PROCESS_CORNERS] = {
    { 2119, UINT_MAX },
    { 2119, UINT_MAX },
    };
    static const u32 __initconst gpu_process_speedos[][GPU_PROCESS_CORNERS] = {
    { UINT_MAX, UINT_MAX },
    { UINT_MAX, UINT_MAX },
    };
    static const u32 __initconst soc_process_speedos[][SOC_PROCESS_CORNERS] = {
    { 1950, 2100, UINT_MAX },
    { 1950, 2100, UINT_MAX },
    };
#[no_mangle]
unsafe extern "C" fn get_speedo_revision() -> u8 __init {
    static u8 __init get_speedo_revision(void)
    {
    return tegra_fuse_read_spare(4) << 2 |
    tegra_fuse_read_spare(3) << 1 |
    tegra_fuse_read_spare(2) << 0;
    }
    static void __init rev_sku_to_speedo_ids(struct tegra_sku_info *sku_info,
    u8 speedo_rev, int *threshold)
    {
    let mut sku: c_int = sku_info.sku_id;
// Assign to default
    sku_info.cpu_speedo_id = 0;
    sku_info.soc_speedo_id = 0;
    sku_info.gpu_speedo_id = 0;
// threshold = THRESHOLD_INDEX_0;
    if (sku_info.revision >= TEGRA_REVISION_A02) {
    switch (sku) {
    case 0x00: /* Engineering SKU */
    case 0x01: /* Engineering SKU */
    case 0x13:
    sku_info.cpu_speedo_id = 5;
    sku_info.gpu_speedo_id = 2;
    break;
    case 0x07:
    case 0x17:
    case 0x1F:
    sku_info.cpu_speedo_id = 7;
    sku_info.gpu_speedo_id = 2;
    break;
    case 0x27:
    sku_info.cpu_speedo_id = 1;
    sku_info.gpu_speedo_id = 2;
    break;
    case 0x83:
    sku_info.cpu_speedo_id = 3;
    sku_info.gpu_speedo_id = 3;
    break;
    case 0x87:
    sku_info.cpu_speedo_id = 2;
    sku_info.gpu_speedo_id = 1;
    break;
    case 0x8F:
    sku_info.soc_speedo_id = 2;
    sku_info.cpu_speedo_id = 9;
    sku_info.gpu_speedo_id = 2;
    break;
    default:
    pr_err("Tegra210: unknown revision 2 or newer SKU %#04x\n", sku);
// Using the default for the error case
    break;
    }
    } else if (sku == 0x00 || sku == 0x01 || sku == 0x07 || sku == 0x13 || sku == 0x17) {
    sku_info.gpu_speedo_id = 1;
    } else {
    pr_err("Tegra210: unknown SKU %#04x\n", sku);
    }
    }
#[no_mangle]
unsafe extern "C" fn get_process_id(value: c_int, speedos: *const u32, num: c_uint) -> c_int {
    static int get_process_id(int value, const u32 *speedos, unsigned int num)
    {
    unsigned int i;
    for (i = 0; i < num; i++)
    if (value < speedos[i])
    return i;
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn tegra210_init_speedo_data(sku_info: *mut tegra_sku_info) -> void __init {
    void __init tegra210_init_speedo_data(struct tegra_sku_info *sku_info)
    {
    int cpu_speedo[3], soc_speedo[3];
    unsigned int index;
    u8 speedo_revision;
    BUILD_BUG_ON(ARRAY_SIZE(cpu_process_speedos) !=
    THRESHOLD_INDEX_COUNT);
    BUILD_BUG_ON(ARRAY_SIZE(gpu_process_speedos) !=
    THRESHOLD_INDEX_COUNT);
    BUILD_BUG_ON(ARRAY_SIZE(soc_process_speedos) !=
    THRESHOLD_INDEX_COUNT);
// Read speedo/IDDQ fuses
    cpu_speedo[0] = tegra_fuse_read_early(FUSE_CPU_SPEEDO_0);
    cpu_speedo[1] = tegra_fuse_read_early(FUSE_CPU_SPEEDO_1);
    cpu_speedo[2] = tegra_fuse_read_early(FUSE_CPU_SPEEDO_2);
    soc_speedo[0] = tegra_fuse_read_early(FUSE_SOC_SPEEDO_0);
    soc_speedo[1] = tegra_fuse_read_early(FUSE_SOC_SPEEDO_1);
    soc_speedo[2] = tegra_fuse_read_early(FUSE_SOC_SPEEDO_2);
//
// Determine CPU, GPU and SoC speedo values depending on speedo fusing
// revision. Note that GPU speedo value is fused in CPU_SPEEDO_2.
//
    speedo_revision = get_speedo_revision();
    pr_info("Speedo Revision %u\n", speedo_revision);
    if (speedo_revision >= 3) {
    sku_info.cpu_speedo_value = cpu_speedo[0];
    sku_info.gpu_speedo_value = cpu_speedo[2];
    sku_info.soc_speedo_value = soc_speedo[0];
    } else if (speedo_revision == 2) {
    sku_info.cpu_speedo_value = (-1938 + (1095 * cpu_speedo[0] / 100)) / 10;
    sku_info.gpu_speedo_value = (-1662 + (1082 * cpu_speedo[2] / 100)) / 10;
    sku_info.soc_speedo_value = ( -705 + (1037 * soc_speedo[0] / 100)) / 10;
    } else {
    sku_info.cpu_speedo_value = 2100;
    sku_info.gpu_speedo_value = cpu_speedo[2] - 75;
    sku_info.soc_speedo_value = 1900;
    }
    if ((sku_info.cpu_speedo_value <= 0) ||
    (sku_info.gpu_speedo_value <= 0) ||
    (sku_info.soc_speedo_value <= 0)) {
    WARN(1, "speedo value not fused\n");
    return;
    }
    rev_sku_to_speedo_ids(sku_info, speedo_revision, &index);
    sku_info.gpu_process_id = get_process_id(sku_info.gpu_speedo_value,
    gpu_process_speedos[index],
    GPU_PROCESS_CORNERS);
    sku_info.cpu_process_id = get_process_id(sku_info.cpu_speedo_value,
    cpu_process_speedos[index],
    CPU_PROCESS_CORNERS);
    sku_info.soc_process_id = get_process_id(sku_info.soc_speedo_value,
    soc_process_speedos[index],
    SOC_PROCESS_CORNERS);
    pr_debug("Tegra GPU Speedo ID=%d, Speedo Value=%d\n",
    sku_info.gpu_speedo_id, sku_info.gpu_speedo_value);
    }
