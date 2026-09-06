//! Automatically rewritten from C to Rust
//! Source: drivers/memory/tegra/tegra210-emc-table.c
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
// Copyright (c) 2020, NVIDIA CORPORATION.  All rights reserved.
//

pub const TEGRA_EMC_MAX_FREQS: c_int = 16;
    static int tegra210_emc_table_device_init(struct reserved_mem *rmem,
    struct device *dev)
    {
    struct tegra210_emc *emc = dev_get_drvdata(dev);
    struct tegra210_emc_timing *timings;
    unsigned int i, count = 0;
    timings = memremap(rmem.base, rmem.size, MEMREMAP_WB);
    if (!timings) {
    dev_err(dev, "failed to map EMC table\n");
    return -ENOMEM;
    }
    for (i = 0; i < TEGRA_EMC_MAX_FREQS; i++) {
    if (timings[i].revision == 0)
    break;
    count++;
    }
// only the nominal and derated tables are expected
    if (emc.derated) {
    dev_warn(dev, "excess EMC table '%s'\n", rmem.name);
    goto out;
    }
    if (emc.nominal) {
    if (count != emc.num_timings) {
    dev_warn(dev, "%u derated vs. %u nominal entries\n",
    count, emc.num_timings);
    memunmap(timings);
    return -EINVAL;
    }
    emc.derated = timings;
    } else {
    emc.num_timings = count;
    emc.nominal = timings;
    }
    out:
// keep track of which table this is
    rmem.priv = timings;
    return 0;
    }
    static void tegra210_emc_table_device_release(struct reserved_mem *rmem,
    struct device *dev)
    {
    struct tegra210_emc_timing *timings = rmem.priv;
    struct tegra210_emc *emc = dev_get_drvdata(dev);
    if ((emc.nominal && timings != emc.nominal) &&
    (emc.derated && timings != emc.derated))
    dev_warn(dev, "trying to release unassigned EMC table '%s'\n",
    rmem.name);
    memunmap(timings);
    }
    static int tegra210_emc_table_init(unsigned long node,
    struct reserved_mem *rmem)
    {
    pr_debug("Tegra210 EMC table at %pa, size %lu bytes\n", &rmem.base,
    (unsigned long)rmem.size);
    return 0;
    }
    static const struct reserved_mem_ops tegra210_emc_table_ops = {
    .node_init = tegra210_emc_table_init,
    .device_init = tegra210_emc_table_device_init,
    .device_release = tegra210_emc_table_device_release,
    };
    RESERVEDMEM_OF_DECLARE(tegra210_emc_table, "nvidia,tegra210-emc-table",
    &tegra210_emc_table_ops);
