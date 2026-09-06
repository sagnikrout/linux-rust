//! Automatically rewritten from C to Rust
//! Source: drivers/clk/tegra/clk-tegra210-emc.c
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
// Copyright (c) 2015-2020, NVIDIA CORPORATION.  All rights reserved.
//

pub const CLK_SOURCE_EMC: c_uint = 0x19c;

pub const CLK_SRC_PLLM: c_int = 0;
pub const CLK_SRC_PLLC: c_int = 1;
pub const CLK_SRC_PLLP: c_int = 2;
pub const CLK_SRC_CLK_M: c_int = 3;
pub const CLK_SRC_PLLM_UD: c_int = 4;
pub const CLK_SRC_PLLMB_UD: c_int = 5;
pub const CLK_SRC_PLLMB: c_int = 6;
pub const CLK_SRC_PLLP_UD: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra210_clk_emc {
    pub hw: clk_hw,
    pub regs: *mut void __iomem,
    pub provider: *mut tegra210_clk_emc_provider,
    pub parents: [*mut clk; 8],
}

    static inline struct tegra210_clk_emc *
    to_tegra210_clk_emc(struct clk_hw *hw)
    {
    return container_of(hw, struct tegra210_clk_emc, hw);
    }
    static const char *tegra210_clk_emc_parents[] = {
    "pll_m", "pll_c", "pll_p", "clk_m", "pll_m_ud", "pll_mb_ud",
    "pll_mb", "pll_p_ud",
    };
#[no_mangle]
unsafe extern "C" fn tegra210_clk_emc_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 tegra210_clk_emc_get_parent(struct clk_hw *hw)
    {
    struct tegra210_clk_emc *emc = to_tegra210_clk_emc(hw);
    u32 value;
    u8 src;
    value = readl_relaxed(emc.regs + CLK_SOURCE_EMC);
    src = FIELD_GET(CLK_SOURCE_EMC_2X_CLK_SRC, value);
    return src;
    }
    static unsigned long tegra210_clk_emc_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct tegra210_clk_emc *emc = to_tegra210_clk_emc(hw);
    u32 value, div;
//
// CCF assumes that neither the parent nor its rate will change during
// ->set_rate(), so the parent rate passed in here was cached from the
// parent before the ->set_rate() call.
//
// This can lead to wrong results being reported for the EMC clock if
// the parent and/or parent rate have changed as part of the EMC rate
// change sequence. Fix this by overriding the parent clock with what
// we know to be the correct value after the rate change.
//
    parent_rate = clk_hw_get_rate(clk_hw_get_parent(hw));
    value = readl_relaxed(emc.regs + CLK_SOURCE_EMC);
    div = FIELD_GET(CLK_SOURCE_EMC_2X_CLK_DIVISOR, value);
    div += 2;
    return DIV_ROUND_UP(parent_rate * 2, div);
    }
    static int tegra210_clk_emc_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct tegra210_clk_emc *emc = to_tegra210_clk_emc(hw);
    struct tegra210_clk_emc_provider *provider = emc.provider;
    unsigned int i;
    if (!provider || !provider.configs || provider.num_configs == 0) {
    req.rate = clk_hw_get_rate(hw);
    return 0;
    }
    for (i = 0; i < provider.num_configs; i++) {
    if (provider.configs[i].rate >= req.rate) {
    req.rate = provider.configs[i].rate;
    return 0;
    }
    }
    req.rate = provider.configs[i - 1].rate;
    return 0;
    }
    static struct clk *tegra210_clk_emc_find_parent(struct tegra210_clk_emc *emc,
    u8 index)
    {
    struct clk_hw *parent = clk_hw_get_parent_by_index(&emc.hw, index);
    const char *name = clk_hw_get_name(parent);
// XXX implement cache?
    return __clk_lookup(name);
    }
    static int tegra210_clk_emc_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct tegra210_clk_emc *emc = to_tegra210_clk_emc(hw);
    struct tegra210_clk_emc_provider *provider = emc.provider;
    struct tegra210_clk_emc_config *config;
    struct device *dev = provider.dev;
    struct clk_hw *old, *new, *parent;
    u8 old_idx, new_idx, index;
    struct clk *clk;
    unsigned int i;
    int err;
    if (!provider.configs || provider.num_configs == 0)
    return -EINVAL;
    for (i = 0; i < provider.num_configs; i++) {
    if (provider.configs[i].rate >= rate) {
    config = &provider.configs[i];
    break;
    }
    }
    if (i == provider.num_configs)
    config = &provider.configs[i - 1];
    old_idx = tegra210_clk_emc_get_parent(hw);
    new_idx = FIELD_GET(CLK_SOURCE_EMC_2X_CLK_SRC, config.value);
    old = clk_hw_get_parent_by_index(hw, old_idx);
    new = clk_hw_get_parent_by_index(hw, new_idx);
// if the rate has changed...
    if (config.parent_rate != clk_hw_get_rate(old)) {
// ... but the clock source remains the same ...
    if (new_idx == old_idx) {
// ... switch to the alternative clock source.
    switch (new_idx) {
    case CLK_SRC_PLLM:
    new_idx = CLK_SRC_PLLMB;
    break;
    case CLK_SRC_PLLM_UD:
    new_idx = CLK_SRC_PLLMB_UD;
    break;
    case CLK_SRC_PLLMB_UD:
    new_idx = CLK_SRC_PLLM_UD;
    break;
    case CLK_SRC_PLLMB:
    new_idx = CLK_SRC_PLLM;
    break;
    }
//
// This should never happen because we can't deal with
// it.
//
    if (WARN_ON(new_idx == old_idx))
    return -EINVAL;
    new = clk_hw_get_parent_by_index(hw, new_idx);
    }
    index = new_idx;
    parent = new;
    } else {
    index = old_idx;
    parent = old;
    }
    clk = tegra210_clk_emc_find_parent(emc, index);
    if (IS_ERR(clk)) {
    err = PTR_ERR(clk);
    dev_err(dev, "failed to get parent clock for index %u: %d\n",
    index, err);
    return err;
    }
// set the new parent clock to the required rate
    if (clk_get_rate(clk) != config.parent_rate) {
    err = clk_set_rate(clk, config.parent_rate);
    if (err < 0) {
    dev_err(dev, "failed to set rate %lu Hz for %pC: %d\n",
    config.parent_rate, clk, err);
    return err;
    }
    }
// enable the new parent clock
    if (parent != old) {
    err = clk_prepare_enable(clk);
    if (err < 0) {
    dev_err(dev, "failed to enable parent clock %pC: %d\n",
    clk, err);
    return err;
    }
    }
// update the EMC source configuration to reflect the new parent
    config.value &= ~CLK_SOURCE_EMC_2X_CLK_SRC;
    config.value |= FIELD_PREP(CLK_SOURCE_EMC_2X_CLK_SRC, index);
//
// Finally, switch the EMC programming with both old and new parent
// clocks enabled.
//
    err = provider.set_rate(dev, config);
    if (err < 0) {
    dev_err(dev, "failed to set EMC rate to %lu Hz: %d\n", rate,
    err);
//
// If we're unable to switch to the new EMC frequency, we no
// longer need the new parent to be enabled.
//
    if (parent != old)
    clk_disable_unprepare(clk);
    return err;
    }
// reparent to new parent clock and disable the old parent clock
    if (parent != old) {
    clk = tegra210_clk_emc_find_parent(emc, old_idx);
    if (IS_ERR(clk)) {
    err = PTR_ERR(clk);
    dev_err(dev,
    "failed to get parent clock for index %u: %d\n",
    old_idx, err);
    return err;
    }
    clk_hw_reparent(hw, parent);
    clk_disable_unprepare(clk);
    }
    return err;
    }
    static const struct clk_ops tegra210_clk_emc_ops = {
    .get_parent = tegra210_clk_emc_get_parent,
    .recalc_rate = tegra210_clk_emc_recalc_rate,
    .determine_rate = tegra210_clk_emc_determine_rate,
    .set_rate = tegra210_clk_emc_set_rate,
    };
    struct clk *tegra210_clk_register_emc(struct device_node *np,
    void __iomem *regs)
    {
    struct tegra210_clk_emc *emc;
    struct clk_init_data init;
    struct clk *clk;
    emc = kzalloc_obj(*emc);
    if (!emc)
    return ERR_PTR(-ENOMEM);
    emc.regs = regs;
    init.name = "emc";
    init.ops = &tegra210_clk_emc_ops;
    init.flags = CLK_IS_CRITICAL | CLK_GET_RATE_NOCACHE;
    init.parent_names = tegra210_clk_emc_parents;
    init.num_parents = ARRAY_SIZE(tegra210_clk_emc_parents);
    emc.hw.init = &init;
    clk = clk_register(core::ptr::null_mut(), &emc.hw);
    if (IS_ERR(clk)) {
    kfree(emc);
    return clk;
    }
    return clk;
    }
    int tegra210_clk_emc_attach(struct clk *clk,
    struct tegra210_clk_emc_provider *provider)
    {
    struct clk_hw *hw = __clk_get_hw(clk);
    struct tegra210_clk_emc *emc = to_tegra210_clk_emc(hw);
    struct device *dev = provider.dev;
    unsigned int i;
    int err;
    if (!try_module_get(provider.owner))
    return -ENODEV;
    for (i = 0; i < provider.num_configs; i++) {
    struct tegra210_clk_emc_config *config = &provider.configs[i];
    struct clk_hw *parent;
    bool same_freq;
    u8 div, src;
    div = FIELD_GET(CLK_SOURCE_EMC_2X_CLK_DIVISOR, config.value);
    src = FIELD_GET(CLK_SOURCE_EMC_2X_CLK_SRC, config.value);
// do basic sanity checking on the EMC timings
    if (div & 0x1) {
    dev_err(dev, "invalid odd divider %u for rate %lu Hz\n",
    div, config.rate);
    err = -EINVAL;
    goto put;
    }
    same_freq = config.value & CLK_SOURCE_EMC_MC_EMC_SAME_FREQ;
    if (same_freq != config.same_freq) {
    dev_err(dev,
    "ambiguous EMC to MC ratio for rate %lu Hz\n",
    config.rate);
    err = -EINVAL;
    goto put;
    }
    parent = clk_hw_get_parent_by_index(hw, src);
    config.parent = src;
    if (src == CLK_SRC_PLLM || src == CLK_SRC_PLLM_UD) {
    config.parent_rate = config.rate * (1 + div / 2);
    } else {
    let mut rate: c_ulong = config.rate * (1 + div / 2);
    config.parent_rate = clk_hw_get_rate(parent);
    if (config.parent_rate != rate) {
    dev_err(dev,
    "rate %lu Hz does not match input\n",
    config.rate);
    err = -EINVAL;
    goto put;
    }
    }
    }
    emc.provider = provider;
    return 0;
    put:
    module_put(provider.owner);
    return err;
    }
    EXPORT_SYMBOL_GPL(tegra210_clk_emc_attach);
#[no_mangle]
pub unsafe extern "C" fn tegra210_clk_emc_detach(clk: *mut clk) {
    void tegra210_clk_emc_detach(struct clk *clk)
    {
    struct tegra210_clk_emc *emc = to_tegra210_clk_emc(__clk_get_hw(clk));
    module_put(emc.provider.owner);
    emc.provider = core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(tegra210_clk_emc_detach);
