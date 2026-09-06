//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mmp/clk-mix.c
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
// mmp mix(div and mux) clock operation source file
//
// Copyright (C) 2014 Marvell
// Chao Xie <chao.xie@marvell.com>
//

//
// The mix clock is a clock combined mux and div type clock.
// Because the div field and mux field need to be set at same
// time, we can not divide it into 2 types of clock
//

#[no_mangle]
unsafe extern "C" fn _get_maxdiv(mix: *mut mmp_clk_mix) -> c_uint {
    static unsigned int _get_maxdiv(struct mmp_clk_mix *mix)
    {
    let mut div_mask: c_uint = (1 << mix.reg_info.width_div) - 1;
    let mut maxdiv: c_uint = 0;
    struct clk_div_table *clkt;
    if (mix.div_flags & CLK_DIVIDER_ONE_BASED)
    return div_mask;
    if (mix.div_flags & CLK_DIVIDER_POWER_OF_TWO)
    return 1 << div_mask;
    if (mix.div_table) {
    for (clkt = mix.div_table; clkt.div; clkt++)
    if (clkt.div > maxdiv)
    maxdiv = clkt.div;
    return maxdiv;
    }
    return div_mask + 1;
    }
#[no_mangle]
unsafe extern "C" fn _get_div(mix: *mut mmp_clk_mix, val: c_uint) -> c_uint {
    static unsigned int _get_div(struct mmp_clk_mix *mix, unsigned int val)
    {
    struct clk_div_table *clkt;
    if (mix.div_flags & CLK_DIVIDER_ONE_BASED)
    return val;
    if (mix.div_flags & CLK_DIVIDER_POWER_OF_TWO)
    return 1 << val;
    if (mix.div_table) {
    for (clkt = mix.div_table; clkt.div; clkt++)
    if (clkt.val == val)
    return clkt.div;
    if (clkt.div == 0)
    return 0;
    }
    return val + 1;
    }
#[no_mangle]
unsafe extern "C" fn _get_mux(mix: *mut mmp_clk_mix, val: c_uint) -> c_uint {
    static unsigned int _get_mux(struct mmp_clk_mix *mix, unsigned int val)
    {
    let mut num_parents: c_int = clk_hw_get_num_parents(&mix.hw);
    int i;
    if (mix.mux_flags & CLK_MUX_INDEX_BIT)
    return ffs(val) - 1;
    if (mix.mux_flags & CLK_MUX_INDEX_ONE)
    return val - 1;
    if (mix.mux_table) {
    for (i = 0; i < num_parents; i++)
    if (mix.mux_table[i] == val)
    return i;
    if (i == num_parents)
    return 0;
    }
    return val;
    }
#[no_mangle]
unsafe extern "C" fn _get_div_val(mix: *mut mmp_clk_mix, div: c_uint) -> c_uint {
    static unsigned int _get_div_val(struct mmp_clk_mix *mix, unsigned int div)
    {
    struct clk_div_table *clkt;
    if (mix.div_flags & CLK_DIVIDER_ONE_BASED)
    return div;
    if (mix.div_flags & CLK_DIVIDER_POWER_OF_TWO)
    return __ffs(div);
    if (mix.div_table) {
    for (clkt = mix.div_table; clkt.div; clkt++)
    if (clkt.div == div)
    return clkt.val;
    if (clkt.div == 0)
    return 0;
    }
    return div - 1;
    }
#[no_mangle]
unsafe extern "C" fn _get_mux_val(mix: *mut mmp_clk_mix, mux: c_uint) -> c_uint {
    static unsigned int _get_mux_val(struct mmp_clk_mix *mix, unsigned int mux)
    {
    if (mix.mux_table)
    return mix.mux_table[mux];
    return mux;
    }
    static void _filter_clk_table(struct mmp_clk_mix *mix,
    struct mmp_clk_mix_clk_table *table,
    unsigned int table_size)
    {
    int i;
    struct mmp_clk_mix_clk_table *item;
    struct clk_hw *parent, *hw;
    unsigned long parent_rate;
    hw = &mix.hw;
    for (i = 0; i < table_size; i++) {
    item = &table[i];
    parent = clk_hw_get_parent_by_index(hw, item.parent_index);
    parent_rate = clk_hw_get_rate(parent);
    if (parent_rate % item.rate) {
    item.valid = 0;
    } else {
    item.divisor = parent_rate / item.rate;
    item.valid = 1;
    }
    }
    }
    static int _set_rate(struct mmp_clk_mix *mix, u32 mux_val, u32 div_val,
    unsigned int change_mux, unsigned int change_div)
    {
    struct mmp_clk_mix_reg_info *ri = &mix.reg_info;
    u8 width, shift;
    u32 mux_div, fc_req;
    int ret, timeout = 50;
    let mut flags: c_ulong = 0;
    if (!change_mux && !change_div)
    return -EINVAL;
    if (mix.lock)
    spin_lock_irqsave(mix.lock, flags);
    if (mix.type == MMP_CLK_MIX_TYPE_V1
    || mix.type == MMP_CLK_MIX_TYPE_V2)
    mux_div = readl(ri.reg_clk_ctrl);
    else
    mux_div = readl(ri.reg_clk_sel);
    if (change_div) {
    width = ri.width_div;
    shift = ri.shift_div;
    mux_div &= ~MMP_CLK_BITS_MASK(width, shift);
    mux_div |= MMP_CLK_BITS_SET_VAL(div_val, width, shift);
    }
    if (change_mux) {
    width = ri.width_mux;
    shift = ri.shift_mux;
    mux_div &= ~MMP_CLK_BITS_MASK(width, shift);
    mux_div |= MMP_CLK_BITS_SET_VAL(mux_val, width, shift);
    }
    if (mix.type == MMP_CLK_MIX_TYPE_V1) {
    writel(mux_div, ri.reg_clk_ctrl);
    } else if (mix.type == MMP_CLK_MIX_TYPE_V2) {
    mux_div |= (1 << ri.bit_fc);
    writel(mux_div, ri.reg_clk_ctrl);
    do {
    fc_req = readl(ri.reg_clk_ctrl);
    timeout--;
    if (!(fc_req & (1 << ri.bit_fc)))
    break;
    } while (timeout);
    if (timeout == 0) {
    pr_err("%s:%s cannot do frequency change\n",
    __func__, clk_hw_get_name(&mix.hw));
    ret = -EBUSY;
    goto error;
    }
    } else {
    fc_req = readl(ri.reg_clk_ctrl);
    fc_req |= 1 << ri.bit_fc;
    writel(fc_req, ri.reg_clk_ctrl);
    writel(mux_div, ri.reg_clk_sel);
    fc_req &= ~(1 << ri.bit_fc);
    }
    ret = 0;
    error:
    if (mix.lock)
    spin_unlock_irqrestore(mix.lock, flags);
    return ret;
    }
    static int mmp_clk_mix_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct mmp_clk_mix *mix = to_clk_mix(hw);
    struct mmp_clk_mix_clk_table *item;
    struct clk_hw *parent, *parent_best;
    unsigned long parent_rate, mix_rate, mix_rate_best, parent_rate_best;
    unsigned long gap, gap_best;
    u32 div_val_max;
    unsigned int div;
    int i, j;
    mix_rate_best = 0;
    parent_rate_best = 0;
    gap_best = ULONG_MAX;
    parent_best = core::ptr::null_mut();
    if (mix.table) {
    for (i = 0; i < mix.table_size; i++) {
    item = &mix.table[i];
    if (item.valid == 0)
    continue;
    parent = clk_hw_get_parent_by_index(hw,
    item.parent_index);
    parent_rate = clk_hw_get_rate(parent);
    mix_rate = parent_rate / item.divisor;
    gap = abs(mix_rate - req.rate);
    if (!parent_best || gap < gap_best) {
    parent_best = parent;
    parent_rate_best = parent_rate;
    mix_rate_best = mix_rate;
    gap_best = gap;
    if (gap_best == 0)
    goto found;
    }
    }
    } else {
    for (i = 0; i < clk_hw_get_num_parents(hw); i++) {
    parent = clk_hw_get_parent_by_index(hw, i);
    parent_rate = clk_hw_get_rate(parent);
    div_val_max = _get_maxdiv(mix);
    for (j = 0; j < div_val_max; j++) {
    div = _get_div(mix, j);
    mix_rate = parent_rate / div;
    gap = abs(mix_rate - req.rate);
    if (!parent_best || gap < gap_best) {
    parent_best = parent;
    parent_rate_best = parent_rate;
    mix_rate_best = mix_rate;
    gap_best = gap;
    if (gap_best == 0)
    goto found;
    }
    }
    }
    }
    found:
    if (!parent_best)
    return -EINVAL;
    req.best_parent_rate = parent_rate_best;
    req.best_parent_hw = parent_best;
    req.rate = mix_rate_best;
    return 0;
    }
    static int mmp_clk_mix_set_rate_and_parent(struct clk_hw *hw,
    unsigned long rate,
    unsigned long parent_rate,
    u8 index)
    {
    struct mmp_clk_mix *mix = to_clk_mix(hw);
    unsigned int div;
    u32 div_val, mux_val;
    div = parent_rate / rate;
    div_val = _get_div_val(mix, div);
    mux_val = _get_mux_val(mix, index);
    return _set_rate(mix, mux_val, div_val, 1, 1);
    }
#[no_mangle]
unsafe extern "C" fn mmp_clk_mix_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 mmp_clk_mix_get_parent(struct clk_hw *hw)
    {
    struct mmp_clk_mix *mix = to_clk_mix(hw);
    struct mmp_clk_mix_reg_info *ri = &mix.reg_info;
    let mut flags: c_ulong = 0;
    let mut mux_div: u32 = 0;
    u8 width, shift;
    u32 mux_val;
    if (mix.lock)
    spin_lock_irqsave(mix.lock, flags);
    if (mix.type == MMP_CLK_MIX_TYPE_V1
    || mix.type == MMP_CLK_MIX_TYPE_V2)
    mux_div = readl(ri.reg_clk_ctrl);
    else
    mux_div = readl(ri.reg_clk_sel);
    if (mix.lock)
    spin_unlock_irqrestore(mix.lock, flags);
    width = mix.reg_info.width_mux;
    shift = mix.reg_info.shift_mux;
    mux_val = MMP_CLK_BITS_GET_VAL(mux_div, width, shift);
    return _get_mux(mix, mux_val);
    }
    static unsigned long mmp_clk_mix_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct mmp_clk_mix *mix = to_clk_mix(hw);
    struct mmp_clk_mix_reg_info *ri = &mix.reg_info;
    let mut flags: c_ulong = 0;
    let mut mux_div: u32 = 0;
    u8 width, shift;
    unsigned int div;
    if (mix.lock)
    spin_lock_irqsave(mix.lock, flags);
    if (mix.type == MMP_CLK_MIX_TYPE_V1
    || mix.type == MMP_CLK_MIX_TYPE_V2)
    mux_div = readl(ri.reg_clk_ctrl);
    else
    mux_div = readl(ri.reg_clk_sel);
    if (mix.lock)
    spin_unlock_irqrestore(mix.lock, flags);
    width = mix.reg_info.width_div;
    shift = mix.reg_info.shift_div;
    div = _get_div(mix, MMP_CLK_BITS_GET_VAL(mux_div, width, shift));
    return parent_rate / div;
    }
#[no_mangle]
unsafe extern "C" fn mmp_clk_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    static int mmp_clk_set_parent(struct clk_hw *hw, u8 index)
    {
    struct mmp_clk_mix *mix = to_clk_mix(hw);
    struct mmp_clk_mix_clk_table *item;
    int i;
    u32 div_val, mux_val;
    if (mix.table) {
    for (i = 0; i < mix.table_size; i++) {
    item = &mix.table[i];
    if (item.valid == 0)
    continue;
    if (item.parent_index == index)
    break;
    }
    if (i < mix.table_size) {
    div_val = _get_div_val(mix, item.divisor);
    mux_val = _get_mux_val(mix, item.parent_index);
    } else
    return -EINVAL;
    } else {
    mux_val = _get_mux_val(mix, index);
    div_val = 0;
    }
    return _set_rate(mix, mux_val, div_val, 1, div_val ? 1 : 0);
    }
    static int mmp_clk_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long best_parent_rate)
    {
    struct mmp_clk_mix *mix = to_clk_mix(hw);
    struct mmp_clk_mix_clk_table *item;
    unsigned long parent_rate;
    unsigned int best_divisor;
    struct clk_hw *parent;
    int i;
    best_divisor = best_parent_rate / rate;
    if (mix.table) {
    for (i = 0; i < mix.table_size; i++) {
    item = &mix.table[i];
    if (item.valid == 0)
    continue;
    parent = clk_hw_get_parent_by_index(hw,
    item.parent_index);
    parent_rate = clk_hw_get_rate(parent);
    if (parent_rate == best_parent_rate
    && item.divisor == best_divisor)
    break;
    }
    if (i < mix.table_size)
    return _set_rate(mix,
    _get_mux_val(mix, item.parent_index),
    _get_div_val(mix, item.divisor),
    1, 1);
    else
    return -EINVAL;
    } else {
    for (i = 0; i < clk_hw_get_num_parents(hw); i++) {
    parent = clk_hw_get_parent_by_index(hw, i);
    parent_rate = clk_hw_get_rate(parent);
    if (parent_rate == best_parent_rate)
    break;
    }
    if (i < clk_hw_get_num_parents(hw))
    return _set_rate(mix, _get_mux_val(mix, i),
    _get_div_val(mix, best_divisor), 1, 1);
    else
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn mmp_clk_mix_init(hw: *mut clk_hw) -> c_int {
    static int mmp_clk_mix_init(struct clk_hw *hw)
    {
    struct mmp_clk_mix *mix = to_clk_mix(hw);
    if (mix.table)
    _filter_clk_table(mix, mix.table, mix.table_size);
    return 0;
    }
    const struct clk_ops mmp_clk_mix_ops = {
    .determine_rate = mmp_clk_mix_determine_rate,
    .set_rate_and_parent = mmp_clk_mix_set_rate_and_parent,
    .set_rate = mmp_clk_set_rate,
    .set_parent = mmp_clk_set_parent,
    .get_parent = mmp_clk_mix_get_parent,
    .recalc_rate = mmp_clk_mix_recalc_rate,
    .init = mmp_clk_mix_init,
    };
    struct clk *mmp_clk_register_mix(struct device *dev,
    const char *name,
    const char * const *parent_names,
    u8 num_parents,
    unsigned long flags,
    struct mmp_clk_mix_config *config,
    spinlock_t *lock)
    {
    struct mmp_clk_mix *mix;
    struct clk *clk;
    struct clk_init_data init;
    mix = kzalloc_obj(*mix);
    if (!mix)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.flags = flags | CLK_GET_RATE_NOCACHE;
    init.parent_names = parent_names;
    init.num_parents = num_parents;
    init.ops = &mmp_clk_mix_ops;
    memcpy(&mix.reg_info, &config.reg_info, sizeof(config.reg_info));
    if (config.table) {
    mix.table = kmemdup_array(config.table, config.table_size,
    sizeof(*mix.table), GFP_KERNEL);
    if (!mix.table)
    goto free_mix;
    mix.table_size = config.table_size;
    }
    if (config.mux_table) {
    mix.mux_table = kmemdup_array(config.mux_table, num_parents,
    sizeof(*mix.mux_table), GFP_KERNEL);
    if (!mix.mux_table) {
    kfree(mix.table);
    goto free_mix;
    }
    }
    mix.div_flags = config.div_flags;
    mix.mux_flags = config.mux_flags;
    mix.lock = lock;
    mix.hw.init = &init;
    if (config.reg_info.bit_fc >= 32)
    mix.type = MMP_CLK_MIX_TYPE_V1;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: config->reg_info.reg_clk_sel) -> else {
    else if (config.reg_info.reg_clk_sel)
    mix.type = MMP_CLK_MIX_TYPE_V3;
    else
    mix.type = MMP_CLK_MIX_TYPE_V2;
    clk = clk_register(dev, &mix.hw);
    if (IS_ERR(clk)) {
    kfree(mix.mux_table);
    kfree(mix.table);
    kfree(mix);
    }
    return clk;
    free_mix:
    kfree(mix);
    return ERR_PTR(-ENOMEM);
    }
