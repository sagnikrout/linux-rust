//! Automatically rewritten from C to Rust
//! Source: drivers/clk/tegra/clk-audio-sync.c
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
// Copyright (c) 2012, NVIDIA CORPORATION.  All rights reserved.
//

    static unsigned long clk_sync_source_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct tegra_clk_sync_source *sync = to_clk_sync_source(hw);
    return sync.rate;
    }
    static int clk_sync_source_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct tegra_clk_sync_source *sync = to_clk_sync_source(hw);
    if (req.rate > sync.max_rate)
    return -EINVAL;
    else
    return 0;
    }
    static int clk_sync_source_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct tegra_clk_sync_source *sync = to_clk_sync_source(hw);
    sync.rate = rate;
    return 0;
    }
    const struct clk_ops tegra_clk_sync_source_ops = {
    .determine_rate = clk_sync_source_determine_rate,
    .set_rate = clk_sync_source_set_rate,
    .recalc_rate = clk_sync_source_recalc_rate,
    };
    struct clk *tegra_clk_register_sync_source(const char *name,
    unsigned long max_rate)
    {
    struct tegra_clk_sync_source *sync;
    struct clk_init_data init;
    struct clk *clk;
    sync = kzalloc_obj(*sync);
    if (!sync) {
    pr_err("%s: could not allocate sync source clk\n", __func__);
    return ERR_PTR(-ENOMEM);
    }
    sync.max_rate = max_rate;
    init.ops = &tegra_clk_sync_source_ops;
    init.name = name;
    init.flags = 0;
    init.parent_names = core::ptr::null_mut();
    init.num_parents = 0;
// Data in .init is copied by clk_register(), so stack variable OK
    sync.hw.init = &init;
    clk = clk_register(core::ptr::null_mut(), &sync.hw);
    if (IS_ERR(clk))
    kfree(sync);
    return clk;
    }
