//! Automatically rewritten from C to Rust
//! Source: drivers/clk/at91/clk-master.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2013 Boris BREZILLON <b.brezillon@overkiz.com>
//

pub const MASTER_PRES_MASK: c_uint = 0x7;

pub const MASTER_DIV_SHIFT: c_int = 8;
pub const MASTER_DIV_MASK: c_uint = 0x7;

pub const MASTER_MAX_ID: c_int = 9;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_master {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
    pub lock: *mut spinlock_t,
    pub layout: *const clk_master_layout,
    pub characteristics: *const clk_master_characteristics,
    pub pms: at91_clk_pms,
    pub mux_table: *mut u32,
    pub mckr: u32,
    pub chg_pid: c_int,
    pub id: u8,
    pub parent: u8,
    pub div: u8,
    pub safe_div: u32,
}

// MCK div reference to be used by notifier.
    static struct clk_master *master_div;
#[no_mangle]
pub unsafe extern "C" fn clk_master_ready(master: *mut clk_master) -> bool {
    static inline bool clk_master_ready(struct clk_master *master)
    {
    let mut bit: c_uint = master.id ? AT91_PMC_MCKXRDY : AT91_PMC_MCKRDY;
    unsigned int status;
    regmap_read(master.regmap, AT91_PMC_SR, &status);
    return !!(status & bit);
    }
#[no_mangle]
unsafe extern "C" fn clk_master_prepare(hw: *mut clk_hw) -> c_int {
    static int clk_master_prepare(struct clk_hw *hw)
    {
    struct clk_master *master = to_clk_master(hw);
    unsigned long flags;
    spin_lock_irqsave(master.lock, flags);
    while (!clk_master_ready(master))
    cpu_relax();
    spin_unlock_irqrestore(master.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_master_is_prepared(hw: *mut clk_hw) -> c_int {
    static int clk_master_is_prepared(struct clk_hw *hw)
    {
    struct clk_master *master = to_clk_master(hw);
    unsigned long flags;
    bool status;
    spin_lock_irqsave(master.lock, flags);
    status = clk_master_ready(master);
    spin_unlock_irqrestore(master.lock, flags);
    return status;
    }
    static unsigned long clk_master_div_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    u8 div;
    unsigned long flags, rate = parent_rate;
    struct clk_master *master = to_clk_master(hw);
    const struct clk_master_layout *layout = master.layout;
    const struct clk_master_characteristics *characteristics =
    master.characteristics;
    unsigned int mckr;
    spin_lock_irqsave(master.lock, flags);
    regmap_read(master.regmap, master.layout.offset, &mckr);
    spin_unlock_irqrestore(master.lock, flags);
    mckr &= layout.mask;
    div = (mckr >> MASTER_DIV_SHIFT) & MASTER_DIV_MASK;
    rate /= characteristics.divisors[div];
    if (rate < characteristics.output.min)
    pr_warn("master clk div is underclocked");
#[no_mangle]
pub unsafe extern "C" fn if(characteristics->output.max: rate >) -> else {
    else if (rate > characteristics.output.max)
    pr_warn("master clk div is overclocked");
    return rate;
    }
#[no_mangle]
unsafe extern "C" fn clk_master_div_save_context(hw: *mut clk_hw) -> c_int {
    static int clk_master_div_save_context(struct clk_hw *hw)
    {
    struct clk_master *master = to_clk_master(hw);
    struct clk_hw *parent_hw = clk_hw_get_parent(hw);
    unsigned long flags;
    unsigned int mckr, div;
    spin_lock_irqsave(master.lock, flags);
    regmap_read(master.regmap, master.layout.offset, &mckr);
    spin_unlock_irqrestore(master.lock, flags);
    mckr &= master.layout.mask;
    div = (mckr >> MASTER_DIV_SHIFT) & MASTER_DIV_MASK;
    div = master.characteristics.divisors[div];
    master.pms.parent_rate = clk_hw_get_rate(parent_hw);
    master.pms.rate = DIV_ROUND_CLOSEST(master.pms.parent_rate, div);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_master_div_restore_context(hw: *mut clk_hw) {
    static void clk_master_div_restore_context(struct clk_hw *hw)
    {
    struct clk_master *master = to_clk_master(hw);
    unsigned long flags;
    unsigned int mckr;
    u8 div;
    spin_lock_irqsave(master.lock, flags);
    regmap_read(master.regmap, master.layout.offset, &mckr);
    spin_unlock_irqrestore(master.lock, flags);
    mckr &= master.layout.mask;
    div = (mckr >> MASTER_DIV_SHIFT) & MASTER_DIV_MASK;
    div = master.characteristics.divisors[div];
    if (div != DIV_ROUND_CLOSEST(master.pms.parent_rate, master.pms.rate))
    pr_warn("MCKR DIV not configured properly by firmware!\n");
    }
    static const struct clk_ops master_div_ops = {
    .prepare = clk_master_prepare,
    .is_prepared = clk_master_is_prepared,
    .recalc_rate = clk_master_div_recalc_rate,
    .save_context = clk_master_div_save_context,
    .restore_context = clk_master_div_restore_context,
    };
// This function must be called with lock acquired.
    static int clk_master_div_set(struct clk_master *master,
    unsigned long parent_rate, int div)
    {
    const struct clk_master_characteristics *characteristics =
    master.characteristics;
    let mut rate: c_ulong = parent_rate;
    let mut max_div: c_uint = 0, div_index = 0, max_div_index = 0;
    unsigned int i, mckr, tmp;
    int ret;
    for (i = 0; i < ARRAY_SIZE(characteristics.divisors); i++) {
    if (!characteristics.divisors[i])
    break;
    if (div == characteristics.divisors[i])
    div_index = i;
    if (max_div < characteristics.divisors[i]) {
    max_div = characteristics.divisors[i];
    max_div_index = i;
    }
    }
    if (div > max_div)
    div_index = max_div_index;
    ret = regmap_read(master.regmap, master.layout.offset, &mckr);
    if (ret)
    return ret;
    mckr &= master.layout.mask;
    tmp = (mckr >> MASTER_DIV_SHIFT) & MASTER_DIV_MASK;
    if (tmp == div_index)
    return 0;
    rate /= characteristics.divisors[div_index];
    if (rate < characteristics.output.min)
    pr_warn("master clk div is underclocked");
#[no_mangle]
pub unsafe extern "C" fn if(characteristics->output.max: rate >) -> else {
    else if (rate > characteristics.output.max)
    pr_warn("master clk div is overclocked");
    mckr &= ~(MASTER_DIV_MASK << MASTER_DIV_SHIFT);
    mckr |= (div_index << MASTER_DIV_SHIFT);
    ret = regmap_write(master.regmap, master.layout.offset, mckr);
    if (ret)
    return ret;
    while (!clk_master_ready(master))
    cpu_relax();
    master.div = characteristics.divisors[div_index];
    return 0;
    }
    static unsigned long clk_master_div_recalc_rate_chg(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_master *master = to_clk_master(hw);
    return DIV_ROUND_CLOSEST_ULL(parent_rate, master.div);
    }
#[no_mangle]
unsafe extern "C" fn clk_master_div_restore_context_chg(hw: *mut clk_hw) {
    static void clk_master_div_restore_context_chg(struct clk_hw *hw)
    {
    struct clk_master *master = to_clk_master(hw);
    unsigned long flags;
    int ret;
    spin_lock_irqsave(master.lock, flags);
    ret = clk_master_div_set(master, master.pms.parent_rate,
    DIV_ROUND_CLOSEST(master.pms.parent_rate,
    master.pms.rate));
    spin_unlock_irqrestore(master.lock, flags);
    if (ret)
    pr_warn("Failed to restore MCK DIV clock\n");
    }
    static const struct clk_ops master_div_ops_chg = {
    .prepare = clk_master_prepare,
    .is_prepared = clk_master_is_prepared,
    .recalc_rate = clk_master_div_recalc_rate_chg,
    .save_context = clk_master_div_save_context,
    .restore_context = clk_master_div_restore_context_chg,
    };
    static int clk_master_div_notifier_fn(struct notifier_block *notifier,
    unsigned long code, void *data)
    {
    const struct clk_master_characteristics *characteristics =
    master_div.characteristics;
    struct clk_notifier_data *cnd = data;
    unsigned long flags, new_parent_rate, new_rate;
    unsigned int mckr, div, new_div = 0;
    int ret, i;
    long tmp_diff;
    let mut best_diff: c_long = -1;
    spin_lock_irqsave(master_div.lock, flags);
    switch (code) {
    case PRE_RATE_CHANGE:
//
// We want to avoid any overclocking of MCK DIV domain. To do
// this we set a safe divider (the underclocking is not of
// interest as we can go as low as 32KHz). The relation
// b/w this clock and its parents are as follows:
//
// FRAC PLL -> DIV PLL -> MCK DIV
//
// With the proper safe divider we should be good even with FRAC
// PLL at its maximum value.
//
    ret = regmap_read(master_div.regmap, master_div.layout.offset,
    &mckr);
    if (ret) {
    ret = NOTIFY_STOP_MASK;
    goto unlock;
    }
    mckr &= master_div.layout.mask;
    div = (mckr >> MASTER_DIV_SHIFT) & MASTER_DIV_MASK;
// Switch to safe divider.
    clk_master_div_set(master_div,
    cnd.old_rate * characteristics.divisors[div],
    master_div.safe_div);
    break;
    case POST_RATE_CHANGE:
//
// At this point we want to restore MCK DIV domain to its maximum
// allowed rate.
//
    ret = regmap_read(master_div.regmap, master_div.layout.offset,
    &mckr);
    if (ret) {
    ret = NOTIFY_STOP_MASK;
    goto unlock;
    }
    mckr &= master_div.layout.mask;
    div = (mckr >> MASTER_DIV_SHIFT) & MASTER_DIV_MASK;
    new_parent_rate = cnd.new_rate * characteristics.divisors[div];
    for (i = 0; i < ARRAY_SIZE(characteristics.divisors); i++) {
    if (!characteristics.divisors[i])
    break;
    new_rate = DIV_ROUND_CLOSEST_ULL(new_parent_rate,
    characteristics.divisors[i]);
    tmp_diff = characteristics.output.max - new_rate;
    if (tmp_diff < 0)
    continue;
    if (best_diff < 0 || best_diff > tmp_diff) {
    new_div = characteristics.divisors[i];
    best_diff = tmp_diff;
    }
    if (!tmp_diff)
    break;
    }
    if (!new_div) {
    ret = NOTIFY_STOP_MASK;
    goto unlock;
    }
// Update the div to preserve MCK DIV clock rate.
    clk_master_div_set(master_div, new_parent_rate,
    new_div);
    ret = NOTIFY_OK;
    break;
    default:
    ret = NOTIFY_DONE;
    break;
    }
    unlock:
    spin_unlock_irqrestore(master_div.lock, flags);
    return ret;
    }
    static struct notifier_block clk_master_div_notifier = {
    .notifier_call = clk_master_div_notifier_fn,
    };
    static void clk_sama7g5_master_best_diff(struct clk_rate_request *req,
    struct clk_hw *parent,
    unsigned long parent_rate,
    long *best_rate,
    long *best_diff,
    u32 div)
    {
    unsigned long tmp_rate, tmp_diff;
    if (div == MASTER_PRES_MAX)
    tmp_rate = parent_rate / 3;
    else
    tmp_rate = parent_rate >> div;
    tmp_diff = abs(req.rate - tmp_rate);
    if (*best_diff < 0 || *best_diff >= tmp_diff) {
// best_rate = tmp_rate;
// best_diff = tmp_diff;
    req.best_parent_rate = parent_rate;
    req.best_parent_hw = parent;
    }
    }
    static unsigned long clk_master_pres_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_master *master = to_clk_master(hw);
    const struct clk_master_characteristics *characteristics =
    master.characteristics;
    unsigned long flags;
    unsigned int val, pres;
    spin_lock_irqsave(master.lock, flags);
    regmap_read(master.regmap, master.layout.offset, &val);
    spin_unlock_irqrestore(master.lock, flags);
    val &= master.layout.mask;
    pres = (val >> master.layout.pres_shift) & MASTER_PRES_MASK;
    if (pres == MASTER_PRES_MAX && characteristics.have_div3_pres)
    pres = 3;
    else
    pres = (1 << pres);
    return DIV_ROUND_CLOSEST_ULL(parent_rate, pres);
    }
#[no_mangle]
unsafe extern "C" fn clk_master_pres_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 clk_master_pres_get_parent(struct clk_hw *hw)
    {
    struct clk_master *master = to_clk_master(hw);
    unsigned long flags;
    unsigned int mckr;
    spin_lock_irqsave(master.lock, flags);
    regmap_read(master.regmap, master.layout.offset, &mckr);
    spin_unlock_irqrestore(master.lock, flags);
    mckr &= master.layout.mask;
    return mckr & AT91_PMC_CSS;
    }
#[no_mangle]
unsafe extern "C" fn clk_master_pres_save_context(hw: *mut clk_hw) -> c_int {
    static int clk_master_pres_save_context(struct clk_hw *hw)
    {
    struct clk_master *master = to_clk_master(hw);
    struct clk_hw *parent_hw = clk_hw_get_parent(hw);
    unsigned long flags;
    unsigned int val, pres;
    spin_lock_irqsave(master.lock, flags);
    regmap_read(master.regmap, master.layout.offset, &val);
    spin_unlock_irqrestore(master.lock, flags);
    val &= master.layout.mask;
    pres = (val >> master.layout.pres_shift) & MASTER_PRES_MASK;
    if (pres == MASTER_PRES_MAX && master.characteristics.have_div3_pres)
    pres = 3;
    else
    pres = (1 << pres);
    master.pms.parent = val & AT91_PMC_CSS;
    master.pms.parent_rate = clk_hw_get_rate(parent_hw);
    master.pms.rate = DIV_ROUND_CLOSEST_ULL(master.pms.parent_rate, pres);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_master_pres_restore_context(hw: *mut clk_hw) {
    static void clk_master_pres_restore_context(struct clk_hw *hw)
    {
    struct clk_master *master = to_clk_master(hw);
    unsigned long flags;
    unsigned int val, pres;
    spin_lock_irqsave(master.lock, flags);
    regmap_read(master.regmap, master.layout.offset, &val);
    spin_unlock_irqrestore(master.lock, flags);
    val &= master.layout.mask;
    pres = (val >> master.layout.pres_shift) & MASTER_PRES_MASK;
    if (pres == MASTER_PRES_MAX && master.characteristics.have_div3_pres)
    pres = 3;
    else
    pres = (1 << pres);
    if (master.pms.rate !=
    DIV_ROUND_CLOSEST_ULL(master.pms.parent_rate, pres) ||
    (master.pms.parent != (val & AT91_PMC_CSS)))
    pr_warn("MCKR PRES was not configured properly by firmware!\n");
    }
    static const struct clk_ops master_pres_ops = {
    .prepare = clk_master_prepare,
    .is_prepared = clk_master_is_prepared,
    .recalc_rate = clk_master_pres_recalc_rate,
    .get_parent = clk_master_pres_get_parent,
    .save_context = clk_master_pres_save_context,
    .restore_context = clk_master_pres_restore_context,
    };
    static struct clk_hw * __init
    at91_clk_register_master_internal(struct regmap *regmap,
    const char *name, int num_parents,
    const char **parent_names,
    struct clk_hw **parent_hws,
    const struct clk_master_layout *layout,
    const struct clk_master_characteristics *characteristics,
    const struct clk_ops *ops, spinlock_t *lock, u32 flags)
    {
    struct clk_master *master;
    let mut init: clk_init_data = {};
    struct clk_hw *hw;
    unsigned int mckr;
    unsigned long irqflags;
    int ret;
    if (!name || !num_parents || !(parent_names || parent_hws) || !lock)
    return ERR_PTR(-EINVAL);
    master = kzalloc_obj(*master);
    if (!master)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.ops = ops;
    if (parent_hws)
    init.parent_hws = (const struct clk_hw **)parent_hws;
    else
    init.parent_names = parent_names;
    init.num_parents = num_parents;
    init.flags = flags;
    master.hw.init = &init;
    master.layout = layout;
    master.characteristics = characteristics;
    master.regmap = regmap;
    master.lock = lock;
    if (ops == &master_div_ops_chg) {
    spin_lock_irqsave(master.lock, irqflags);
    regmap_read(master.regmap, master.layout.offset, &mckr);
    spin_unlock_irqrestore(master.lock, irqflags);
    mckr &= layout.mask;
    mckr = (mckr >> MASTER_DIV_SHIFT) & MASTER_DIV_MASK;
    master.div = characteristics.divisors[mckr];
    }
    hw = &master.hw;
    ret = clk_hw_register(core::ptr::null_mut(), &master.hw);
    if (ret) {
    kfree(master);
    hw = ERR_PTR(ret);
    }
    return hw;
    }
    struct clk_hw * __init
    at91_clk_register_master_pres(struct regmap *regmap,
    const char *name, int num_parents,
    const char **parent_names,
    struct clk_hw **parent_hws,
    const struct clk_master_layout *layout,
    const struct clk_master_characteristics *characteristics,
    spinlock_t *lock)
    {
    return at91_clk_register_master_internal(regmap, name, num_parents,
    parent_names, parent_hws, layout,
    characteristics,
    &master_pres_ops,
    lock, CLK_SET_RATE_GATE);
    }
    struct clk_hw * __init
    at91_clk_register_master_div(struct regmap *regmap,
    const char *name, const char *parent_name,
    struct clk_hw *parent_hw, const struct clk_master_layout *layout,
    const struct clk_master_characteristics *characteristics,
    spinlock_t *lock, u32 flags, u32 safe_div)
    {
    const struct clk_ops *ops;
    struct clk_hw *hw;
    if (flags & CLK_SET_RATE_GATE)
    ops = &master_div_ops;
    else
    ops = &master_div_ops_chg;
    hw = at91_clk_register_master_internal(regmap, name, 1,
    parent_name ? &parent_name : core::ptr::null_mut(),
    parent_hw ? &parent_hw : core::ptr::null_mut(), layout,
    characteristics, ops,
    lock, flags);
    if (!IS_ERR(hw) && safe_div) {
    master_div = to_clk_master(hw);
    master_div.safe_div = safe_div;
    clk_notifier_register(hw.clk,
    &clk_master_div_notifier);
    }
    return hw;
    }
    static unsigned long
    clk_sama7g5_master_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_master *master = to_clk_master(hw);
    if (master.div == MASTER_PRES_MAX)
    return DIV_ROUND_CLOSEST_ULL(parent_rate, 3);
    return DIV_ROUND_CLOSEST_ULL(parent_rate, (1 << master.div));
    }
    static int clk_sama7g5_master_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct clk_master *master = to_clk_master(hw);
    struct clk_hw *parent;
    let mut best_rate: c_long = LONG_MIN, best_diff = LONG_MIN;
    unsigned long parent_rate;
    unsigned int div, i;
// First: check the dividers of MCR.
    for (i = 0; i < clk_hw_get_num_parents(hw); i++) {
    parent = clk_hw_get_parent_by_index(hw, i);
    if (!parent)
    continue;
    parent_rate = clk_hw_get_rate(parent);
    if (!parent_rate)
    continue;
    for (div = 0; div < MASTER_PRES_MAX + 1; div++) {
    clk_sama7g5_master_best_diff(req, parent, parent_rate,
    &best_rate, &best_diff,
    div);
    if (!best_diff)
    break;
    }
    if (!best_diff)
    break;
    }
// Second: try to request rate form changeable parent.
    if (master.chg_pid < 0)
    goto end;
    parent = clk_hw_get_parent_by_index(hw, master.chg_pid);
    if (!parent)
    goto end;
    for (div = 0; div < MASTER_PRES_MAX + 1; div++) {
    struct clk_rate_request req_parent;
    unsigned long req_rate;
    if (div == MASTER_PRES_MAX)
    req_rate = req.rate * 3;
    else
    req_rate = req.rate << div;
    clk_hw_forward_rate_request(hw, req, parent, &req_parent, req_rate);
    if (__clk_determine_rate(parent, &req_parent))
    continue;
    clk_sama7g5_master_best_diff(req, parent, req_parent.rate,
    &best_rate, &best_diff, div);
    if (!best_diff)
    break;
    }
    end:
    pr_debug("MCK: %s, best_rate = %ld, parent clk: %s @ %ld\n",
    __func__, best_rate,
    __clk_get_name((req.best_parent_hw).clk),
    req.best_parent_rate);
    if (best_rate < 0)
    return -EINVAL;
    req.rate = best_rate;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_sama7g5_master_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 clk_sama7g5_master_get_parent(struct clk_hw *hw)
    {
    struct clk_master *master = to_clk_master(hw);
    unsigned long flags;
    u8 index;
    spin_lock_irqsave(master.lock, flags);
    index = clk_mux_val_to_index(&master.hw, master.mux_table, 0,
    master.parent);
    spin_unlock_irqrestore(master.lock, flags);
    return index;
    }
#[no_mangle]
unsafe extern "C" fn clk_sama7g5_master_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    static int clk_sama7g5_master_set_parent(struct clk_hw *hw, u8 index)
    {
    struct clk_master *master = to_clk_master(hw);
    unsigned long flags;
    if (index >= clk_hw_get_num_parents(hw))
    return -EINVAL;
    spin_lock_irqsave(master.lock, flags);
    master.parent = clk_mux_index_to_val(master.mux_table, 0, index);
    spin_unlock_irqrestore(master.lock, flags);
    return 0;
    }
    static void clk_sama7g5_master_set(struct clk_master *master,
    unsigned int status)
    {
    unsigned long flags;
    unsigned int val, cparent;
    let mut enable: c_uint = status ? AT91_PMC_MCR_V2_EN : 0;
    let mut parent: c_uint = master.parent << PMC_MCR_CSS_SHIFT;
    let mut div: c_uint = master.div << MASTER_DIV_SHIFT;
    spin_lock_irqsave(master.lock, flags);
    regmap_write(master.regmap, AT91_PMC_MCR_V2,
    AT91_PMC_MCR_V2_ID(master.id));
    regmap_read(master.regmap, AT91_PMC_MCR_V2, &val);
    regmap_update_bits(master.regmap, AT91_PMC_MCR_V2,
    enable | AT91_PMC_MCR_V2_CSS | AT91_PMC_MCR_V2_DIV |
    AT91_PMC_MCR_V2_CMD | AT91_PMC_MCR_V2_ID_MSK,
    enable | parent | div | AT91_PMC_MCR_V2_CMD |
    AT91_PMC_MCR_V2_ID(master.id));
    cparent = (val & AT91_PMC_MCR_V2_CSS) >> PMC_MCR_CSS_SHIFT;
// Wait here only if parent is being changed.
    while ((cparent != master.parent) && !clk_master_ready(master))
    cpu_relax();
    spin_unlock_irqrestore(master.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn clk_sama7g5_master_enable(hw: *mut clk_hw) -> c_int {
    static int clk_sama7g5_master_enable(struct clk_hw *hw)
    {
    struct clk_master *master = to_clk_master(hw);
    clk_sama7g5_master_set(master, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_sama7g5_master_disable(hw: *mut clk_hw) {
    static void clk_sama7g5_master_disable(struct clk_hw *hw)
    {
    struct clk_master *master = to_clk_master(hw);
    unsigned long flags;
    spin_lock_irqsave(master.lock, flags);
    regmap_write(master.regmap, AT91_PMC_MCR_V2, master.id);
    regmap_update_bits(master.regmap, AT91_PMC_MCR_V2,
    AT91_PMC_MCR_V2_EN | AT91_PMC_MCR_V2_CMD |
    AT91_PMC_MCR_V2_ID_MSK,
    AT91_PMC_MCR_V2_CMD |
    AT91_PMC_MCR_V2_ID(master.id));
    spin_unlock_irqrestore(master.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn clk_sama7g5_master_is_enabled(hw: *mut clk_hw) -> c_int {
    static int clk_sama7g5_master_is_enabled(struct clk_hw *hw)
    {
    struct clk_master *master = to_clk_master(hw);
    unsigned long flags;
    unsigned int val;
    spin_lock_irqsave(master.lock, flags);
    regmap_write(master.regmap, AT91_PMC_MCR_V2, master.id);
    regmap_read(master.regmap, AT91_PMC_MCR_V2, &val);
    spin_unlock_irqrestore(master.lock, flags);
    return !!(val & AT91_PMC_MCR_V2_EN);
    }
    static int clk_sama7g5_master_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct clk_master *master = to_clk_master(hw);
    unsigned long div, flags;
    div = DIV_ROUND_CLOSEST(parent_rate, rate);
    if ((div > (1 << (MASTER_PRES_MAX - 1))) || (div & (div - 1)))
    return -EINVAL;
    if (div == 3)
    div = MASTER_PRES_MAX;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: div) -> else {
    else if (div)
    div = ffs(div) - 1;
    spin_lock_irqsave(master.lock, flags);
    master.div = div;
    spin_unlock_irqrestore(master.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_sama7g5_master_save_context(hw: *mut clk_hw) -> c_int {
    static int clk_sama7g5_master_save_context(struct clk_hw *hw)
    {
    struct clk_master *master = to_clk_master(hw);
    master.pms.status = clk_sama7g5_master_is_enabled(hw);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_sama7g5_master_restore_context(hw: *mut clk_hw) {
    static void clk_sama7g5_master_restore_context(struct clk_hw *hw)
    {
    struct clk_master *master = to_clk_master(hw);
    if (master.pms.status)
    clk_sama7g5_master_set(master, master.pms.status);
    }
    static const struct clk_ops sama7g5_master_ops = {
    .enable = clk_sama7g5_master_enable,
    .disable = clk_sama7g5_master_disable,
    .is_enabled = clk_sama7g5_master_is_enabled,
    .recalc_rate = clk_sama7g5_master_recalc_rate,
    .determine_rate = clk_sama7g5_master_determine_rate,
    .set_rate = clk_sama7g5_master_set_rate,
    .get_parent = clk_sama7g5_master_get_parent,
    .set_parent = clk_sama7g5_master_set_parent,
    .save_context = clk_sama7g5_master_save_context,
    .restore_context = clk_sama7g5_master_restore_context,
    };
    struct clk_hw * __init
    at91_clk_sama7g5_register_master(struct regmap *regmap,
    const char *name, int num_parents,
    const char **parent_names,
    struct clk_hw **parent_hws,
    u32 *mux_table,
    spinlock_t *lock, u8 id,
    bool critical, int chg_pid)
    {
    struct clk_master *master;
    struct clk_hw *hw;
    let mut init: clk_init_data = {};
    unsigned long flags;
    unsigned int val;
    int ret;
    if (!name || !num_parents || !(parent_names || parent_hws) || !mux_table ||
    !lock || id > MASTER_MAX_ID)
    return ERR_PTR(-EINVAL);
    master = kzalloc_obj(*master);
    if (!master)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.ops = &sama7g5_master_ops;
    if (parent_hws)
    init.parent_hws = (const struct clk_hw **)parent_hws;
    else
    init.parent_names = parent_names;
    init.num_parents = num_parents;
    init.flags = CLK_SET_RATE_GATE | CLK_SET_PARENT_GATE;
    if (chg_pid >= 0)
    init.flags |= CLK_SET_RATE_PARENT;
    if (critical)
    init.flags |= CLK_IS_CRITICAL;
    master.hw.init = &init;
    master.regmap = regmap;
    master.id = id;
    master.chg_pid = chg_pid;
    master.lock = lock;
    master.mux_table = mux_table;
    spin_lock_irqsave(master.lock, flags);
    regmap_write(master.regmap, AT91_PMC_MCR_V2, master.id);
    regmap_read(master.regmap, AT91_PMC_MCR_V2, &val);
    master.parent = (val & AT91_PMC_MCR_V2_CSS) >> PMC_MCR_CSS_SHIFT;
    master.div = (val & AT91_PMC_MCR_V2_DIV) >> MASTER_DIV_SHIFT;
    spin_unlock_irqrestore(master.lock, flags);
    hw = &master.hw;
    ret = clk_hw_register(core::ptr::null_mut(), &master.hw);
    if (ret) {
    kfree(master);
    hw = ERR_PTR(ret);
    }
    return hw;
    }
    const struct clk_master_layout at91rm9200_master_layout = {
    .mask = 0x31F,
    .pres_shift = 2,
    .offset = AT91_PMC_MCKR,
    };
    const struct clk_master_layout at91sam9x5_master_layout = {
    .mask = 0x373,
    .pres_shift = 4,
    .offset = AT91_PMC_MCKR,
    };
