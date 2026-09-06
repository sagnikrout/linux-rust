//! Automatically rewritten from C to Rust
//! Source: drivers/soc/tegra/fuse/speedo-tegra20.c
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

pub const CPU_SPEEDO_LSBIT: c_int = 20;
pub const CPU_SPEEDO_MSBIT: c_int = 29;
pub const CPU_SPEEDO_REDUND_LSBIT: c_int = 30;
pub const CPU_SPEEDO_REDUND_MSBIT: c_int = 39;

pub const SOC_SPEEDO_LSBIT: c_int = 40;
pub const SOC_SPEEDO_MSBIT: c_int = 47;
pub const SOC_SPEEDO_REDUND_LSBIT: c_int = 48;
pub const SOC_SPEEDO_REDUND_MSBIT: c_int = 55;

pub const SPEEDO_MULT: c_int = 4;
pub const PROCESS_CORNERS_NUM: c_int = 4;

    (((sku) != 20) && ((sku) != 23) && ((sku) != 24) && \
    ((sku) != 27) && ((sku) != 28))
    enum {
    SPEEDO_ID_0,
    SPEEDO_ID_1,
    SPEEDO_ID_2,
    SPEEDO_ID_COUNT,
    };
    static const u32 __initconst cpu_process_speedos[][PROCESS_CORNERS_NUM] = {
    {315, 366, 420, UINT_MAX},
    {303, 368, 419, UINT_MAX},
    {316, 331, 383, UINT_MAX},
    };
    static const u32 __initconst soc_process_speedos[][PROCESS_CORNERS_NUM] = {
    {165, 195, 224, UINT_MAX},
    {165, 195, 224, UINT_MAX},
    {165, 195, 224, UINT_MAX},
    };
#[no_mangle]
pub unsafe extern "C" fn tegra20_init_speedo_data(sku_info: *mut tegra_sku_info) -> void __init {
    void __init tegra20_init_speedo_data(struct tegra_sku_info *sku_info)
    {
    u32 reg;
    u32 val;
    int i;
    BUILD_BUG_ON(ARRAY_SIZE(cpu_process_speedos) != SPEEDO_ID_COUNT);
    BUILD_BUG_ON(ARRAY_SIZE(soc_process_speedos) != SPEEDO_ID_COUNT);
    if (SPEEDO_ID_SELECT_0(sku_info.revision))
    sku_info.soc_speedo_id = SPEEDO_ID_0;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: SPEEDO_ID_SELECT_1(sku_info->sku_id)) -> else {
    else if (SPEEDO_ID_SELECT_1(sku_info.sku_id))
    sku_info.soc_speedo_id = SPEEDO_ID_1;
    else
    sku_info.soc_speedo_id = SPEEDO_ID_2;
    val = 0;
    for (i = CPU_SPEEDO_MSBIT; i >= CPU_SPEEDO_LSBIT; i--) {
    reg = tegra_fuse_read_spare(i) |
    tegra_fuse_read_spare(i + CPU_SPEEDO_REDUND_OFFS);
    val = (val << 1) | (reg & 0x1);
    }
    val = val * SPEEDO_MULT;
    pr_debug("Tegra CPU speedo value %u\n", val);
    for (i = 0; i < (PROCESS_CORNERS_NUM - 1); i++) {
    if (val <= cpu_process_speedos[sku_info.soc_speedo_id][i])
    break;
    }
    sku_info.cpu_process_id = i;
    val = 0;
    for (i = SOC_SPEEDO_MSBIT; i >= SOC_SPEEDO_LSBIT; i--) {
    reg = tegra_fuse_read_spare(i) |
    tegra_fuse_read_spare(i + SOC_SPEEDO_REDUND_OFFS);
    val = (val << 1) | (reg & 0x1);
    }
    val = val * SPEEDO_MULT;
    pr_debug("Core speedo value %u\n", val);
    for (i = 0; i < (PROCESS_CORNERS_NUM - 1); i++) {
    if (val <= soc_process_speedos[sku_info.soc_speedo_id][i])
    break;
    }
    sku_info.soc_process_id = i;
    }
