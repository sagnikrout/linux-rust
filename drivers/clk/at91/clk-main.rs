//! Automatically rewritten from C to Rust
//! Source: drivers/clk/at91/clk-main.c
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

pub const SLOW_CLOCK_FREQ: c_int = 32768;
pub const MAINF_DIV: c_int = 16;

    SLOW_CLOCK_FREQ)

    (AT91_PMC_MOSCEN | \
    AT91_PMC_OSCBYPASS)) ? 1 : 0)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_main_osc {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
    pub pms: at91_clk_pms,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_main_rc_osc {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
    pub frequency: c_ulong,
    pub accuracy: c_ulong,
    pub pms: at91_clk_pms,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_rm9200_main {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_sam9x5_main {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
    pub pms: at91_clk_pms,
    pub parent: u8,
}

#[no_mangle]
pub unsafe extern "C" fn clk_main_osc_ready(regmap: *mut regmap) -> bool {
    static inline bool clk_main_osc_ready(struct regmap *regmap)
    {
    unsigned int status;
    regmap_read(regmap, AT91_PMC_SR, &status);
    return status & AT91_PMC_MOSCS;
    }
#[no_mangle]
unsafe extern "C" fn clk_main_osc_prepare(hw: *mut clk_hw) -> c_int {
    static int clk_main_osc_prepare(struct clk_hw *hw)
    {
    struct clk_main_osc *osc = to_clk_main_osc(hw);
    struct regmap *regmap = osc.regmap;
    u32 tmp;
    regmap_read(regmap, AT91_CKGR_MOR, &tmp);
    tmp &= ~MOR_KEY_MASK;
    if (tmp & AT91_PMC_OSCBYPASS)
    return 0;
    if (!(tmp & AT91_PMC_MOSCEN)) {
    tmp |= AT91_PMC_MOSCEN | AT91_PMC_KEY;
    regmap_write(regmap, AT91_CKGR_MOR, tmp);
    }
    while (!clk_main_osc_ready(regmap))
    cpu_relax();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_main_osc_unprepare(hw: *mut clk_hw) {
    static void clk_main_osc_unprepare(struct clk_hw *hw)
    {
    struct clk_main_osc *osc = to_clk_main_osc(hw);
    struct regmap *regmap = osc.regmap;
    u32 tmp;
    regmap_read(regmap, AT91_CKGR_MOR, &tmp);
    if (tmp & AT91_PMC_OSCBYPASS)
    return;
    if (!(tmp & AT91_PMC_MOSCEN))
    return;
    tmp &= ~(AT91_PMC_KEY | AT91_PMC_MOSCEN);
    regmap_write(regmap, AT91_CKGR_MOR, tmp | AT91_PMC_KEY);
    }
#[no_mangle]
unsafe extern "C" fn clk_main_osc_is_prepared(hw: *mut clk_hw) -> c_int {
    static int clk_main_osc_is_prepared(struct clk_hw *hw)
    {
    struct clk_main_osc *osc = to_clk_main_osc(hw);
    struct regmap *regmap = osc.regmap;
    u32 tmp, status;
    regmap_read(regmap, AT91_CKGR_MOR, &tmp);
    if (tmp & AT91_PMC_OSCBYPASS)
    return 1;
    regmap_read(regmap, AT91_PMC_SR, &status);
    return (status & AT91_PMC_MOSCS) && clk_main_parent_select(tmp);
    }
#[no_mangle]
unsafe extern "C" fn clk_main_osc_save_context(hw: *mut clk_hw) -> c_int {
    static int clk_main_osc_save_context(struct clk_hw *hw)
    {
    struct clk_main_osc *osc = to_clk_main_osc(hw);
    osc.pms.status = clk_main_osc_is_prepared(hw);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_main_osc_restore_context(hw: *mut clk_hw) {
    static void clk_main_osc_restore_context(struct clk_hw *hw)
    {
    struct clk_main_osc *osc = to_clk_main_osc(hw);
    if (osc.pms.status)
    clk_main_osc_prepare(hw);
    }
    static const struct clk_ops main_osc_ops = {
    .prepare = clk_main_osc_prepare,
    .unprepare = clk_main_osc_unprepare,
    .is_prepared = clk_main_osc_is_prepared,
    .save_context = clk_main_osc_save_context,
    .restore_context = clk_main_osc_restore_context,
    };
    struct clk_hw * __init
    at91_clk_register_main_osc(struct regmap *regmap,
    const char *name,
    const char *parent_name,
    struct clk_parent_data *parent_data,
    bool bypass)
    {
    struct clk_main_osc *osc;
    let mut init: clk_init_data = {};
    struct clk_hw *hw;
    int ret;
    if (!name || !(parent_name || parent_data))
    return ERR_PTR(-EINVAL);
    osc = kzalloc_obj(*osc);
    if (!osc)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.ops = &main_osc_ops;
    if (parent_data)
    init.parent_data = (const struct clk_parent_data *)parent_data;
    else
    init.parent_names = &parent_name;
    init.num_parents = 1;
    init.flags = CLK_IGNORE_UNUSED;
    osc.hw.init = &init;
    osc.regmap = regmap;
    if (bypass)
    regmap_update_bits(regmap,
    AT91_CKGR_MOR, MOR_KEY_MASK |
    AT91_PMC_OSCBYPASS,
    AT91_PMC_OSCBYPASS | AT91_PMC_KEY);
    hw = &osc.hw;
    ret = clk_hw_register(core::ptr::null_mut(), &osc.hw);
    if (ret) {
    kfree(osc);
    hw = ERR_PTR(ret);
    }
    return hw;
    }
#[no_mangle]
unsafe extern "C" fn clk_main_rc_osc_ready(regmap: *mut regmap) -> bool {
    static bool clk_main_rc_osc_ready(struct regmap *regmap)
    {
    unsigned int status;
    regmap_read(regmap, AT91_PMC_SR, &status);
    return !!(status & AT91_PMC_MOSCRCS);
    }
#[no_mangle]
unsafe extern "C" fn clk_main_rc_osc_prepare(hw: *mut clk_hw) -> c_int {
    static int clk_main_rc_osc_prepare(struct clk_hw *hw)
    {
    struct clk_main_rc_osc *osc = to_clk_main_rc_osc(hw);
    struct regmap *regmap = osc.regmap;
    unsigned int mor;
    regmap_read(regmap, AT91_CKGR_MOR, &mor);
    if (!(mor & AT91_PMC_MOSCRCEN))
    regmap_update_bits(regmap, AT91_CKGR_MOR,
    MOR_KEY_MASK | AT91_PMC_MOSCRCEN,
    AT91_PMC_MOSCRCEN | AT91_PMC_KEY);
    while (!clk_main_rc_osc_ready(regmap))
    cpu_relax();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_main_rc_osc_unprepare(hw: *mut clk_hw) {
    static void clk_main_rc_osc_unprepare(struct clk_hw *hw)
    {
    struct clk_main_rc_osc *osc = to_clk_main_rc_osc(hw);
    struct regmap *regmap = osc.regmap;
    unsigned int mor;
    regmap_read(regmap, AT91_CKGR_MOR, &mor);
    if (!(mor & AT91_PMC_MOSCRCEN))
    return;
    regmap_update_bits(regmap, AT91_CKGR_MOR,
    MOR_KEY_MASK | AT91_PMC_MOSCRCEN, AT91_PMC_KEY);
    }
#[no_mangle]
unsafe extern "C" fn clk_main_rc_osc_is_prepared(hw: *mut clk_hw) -> c_int {
    static int clk_main_rc_osc_is_prepared(struct clk_hw *hw)
    {
    struct clk_main_rc_osc *osc = to_clk_main_rc_osc(hw);
    struct regmap *regmap = osc.regmap;
    unsigned int mor, status;
    regmap_read(regmap, AT91_CKGR_MOR, &mor);
    regmap_read(regmap, AT91_PMC_SR, &status);
    return (mor & AT91_PMC_MOSCRCEN) && (status & AT91_PMC_MOSCRCS);
    }
    static unsigned long clk_main_rc_osc_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_main_rc_osc *osc = to_clk_main_rc_osc(hw);
    return osc.frequency;
    }
    static unsigned long clk_main_rc_osc_recalc_accuracy(struct clk_hw *hw,
    unsigned long parent_acc)
    {
    struct clk_main_rc_osc *osc = to_clk_main_rc_osc(hw);
    return osc.accuracy;
    }
#[no_mangle]
unsafe extern "C" fn clk_main_rc_osc_save_context(hw: *mut clk_hw) -> c_int {
    static int clk_main_rc_osc_save_context(struct clk_hw *hw)
    {
    struct clk_main_rc_osc *osc = to_clk_main_rc_osc(hw);
    osc.pms.status = clk_main_rc_osc_is_prepared(hw);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_main_rc_osc_restore_context(hw: *mut clk_hw) {
    static void clk_main_rc_osc_restore_context(struct clk_hw *hw)
    {
    struct clk_main_rc_osc *osc = to_clk_main_rc_osc(hw);
    if (osc.pms.status)
    clk_main_rc_osc_prepare(hw);
    }
    static const struct clk_ops main_rc_osc_ops = {
    .prepare = clk_main_rc_osc_prepare,
    .unprepare = clk_main_rc_osc_unprepare,
    .is_prepared = clk_main_rc_osc_is_prepared,
    .recalc_rate = clk_main_rc_osc_recalc_rate,
    .recalc_accuracy = clk_main_rc_osc_recalc_accuracy,
    .save_context = clk_main_rc_osc_save_context,
    .restore_context = clk_main_rc_osc_restore_context,
    };
    struct clk_hw * __init
    at91_clk_register_main_rc_osc(struct regmap *regmap,
    const char *name,
    u32 frequency, u32 accuracy)
    {
    struct clk_main_rc_osc *osc;
    struct clk_init_data init;
    struct clk_hw *hw;
    int ret;
    if (!name || !frequency)
    return ERR_PTR(-EINVAL);
    osc = kzalloc_obj(*osc);
    if (!osc)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.ops = &main_rc_osc_ops;
    init.parent_names = core::ptr::null_mut();
    init.num_parents = 0;
    init.flags = CLK_IGNORE_UNUSED;
    osc.hw.init = &init;
    osc.regmap = regmap;
    osc.frequency = frequency;
    osc.accuracy = accuracy;
    hw = &osc.hw;
    ret = clk_hw_register(core::ptr::null_mut(), hw);
    if (ret) {
    kfree(osc);
    hw = ERR_PTR(ret);
    }
    return hw;
    }
#[no_mangle]
unsafe extern "C" fn clk_main_probe_frequency(regmap: *mut regmap) -> c_int {
    static int clk_main_probe_frequency(struct regmap *regmap)
    {
    unsigned long prep_time, timeout;
    unsigned int mcfr;
    timeout = jiffies + usecs_to_jiffies(MAINFRDY_TIMEOUT);
    do {
    prep_time = jiffies;
    regmap_read(regmap, AT91_CKGR_MCFR, &mcfr);
    if (mcfr & AT91_PMC_MAINRDY)
    return 0;
    if (system_state < SYSTEM_RUNNING)
    udelay(MAINF_LOOP_MIN_WAIT);
    else
    usleep_range(MAINF_LOOP_MIN_WAIT, MAINF_LOOP_MAX_WAIT);
    } while (time_before(prep_time, timeout));
    return -ETIMEDOUT;
    }
    static unsigned long clk_main_recalc_rate(struct regmap *regmap,
    unsigned long parent_rate)
    {
    unsigned int mcfr;
    if (parent_rate)
    return parent_rate;
    pr_warn("Main crystal frequency not set, using approximate value\n");
    regmap_read(regmap, AT91_CKGR_MCFR, &mcfr);
    if (!(mcfr & AT91_PMC_MAINRDY))
    return 0;
    return ((mcfr & AT91_PMC_MAINF) * SLOW_CLOCK_FREQ) / MAINF_DIV;
    }
#[no_mangle]
unsafe extern "C" fn clk_rm9200_main_prepare(hw: *mut clk_hw) -> c_int {
    static int clk_rm9200_main_prepare(struct clk_hw *hw)
    {
    struct clk_rm9200_main *clkmain = to_clk_rm9200_main(hw);
    return clk_main_probe_frequency(clkmain.regmap);
    }
#[no_mangle]
unsafe extern "C" fn clk_rm9200_main_is_prepared(hw: *mut clk_hw) -> c_int {
    static int clk_rm9200_main_is_prepared(struct clk_hw *hw)
    {
    struct clk_rm9200_main *clkmain = to_clk_rm9200_main(hw);
    unsigned int status;
    regmap_read(clkmain.regmap, AT91_CKGR_MCFR, &status);
    return !!(status & AT91_PMC_MAINRDY);
    }
    static unsigned long clk_rm9200_main_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_rm9200_main *clkmain = to_clk_rm9200_main(hw);
    return clk_main_recalc_rate(clkmain.regmap, parent_rate);
    }
    static const struct clk_ops rm9200_main_ops = {
    .prepare = clk_rm9200_main_prepare,
    .is_prepared = clk_rm9200_main_is_prepared,
    .recalc_rate = clk_rm9200_main_recalc_rate,
    };
    struct clk_hw * __init
    at91_clk_register_rm9200_main(struct regmap *regmap,
    const char *name,
    const char *parent_name,
    struct clk_hw *parent_hw)
    {
    struct clk_rm9200_main *clkmain;
    let mut init: clk_init_data = {};
    struct clk_hw *hw;
    int ret;
    if (!name)
    return ERR_PTR(-EINVAL);
    if (!(parent_name || parent_hw))
    return ERR_PTR(-EINVAL);
    clkmain = kzalloc_obj(*clkmain);
    if (!clkmain)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.ops = &rm9200_main_ops;
    if (parent_hw)
    init.parent_hws = (const struct clk_hw **)&parent_hw;
    else
    init.parent_names = &parent_name;
    init.num_parents = 1;
    init.flags = 0;
    clkmain.hw.init = &init;
    clkmain.regmap = regmap;
    hw = &clkmain.hw;
    ret = clk_hw_register(core::ptr::null_mut(), &clkmain.hw);
    if (ret) {
    kfree(clkmain);
    hw = ERR_PTR(ret);
    }
    return hw;
    }
#[no_mangle]
pub unsafe extern "C" fn clk_sam9x5_main_ready(regmap: *mut regmap) -> bool {
    static inline bool clk_sam9x5_main_ready(struct regmap *regmap)
    {
    unsigned int status;
    regmap_read(regmap, AT91_PMC_SR, &status);
    return !!(status & AT91_PMC_MOSCSELS);
    }
#[no_mangle]
unsafe extern "C" fn clk_sam9x5_main_prepare(hw: *mut clk_hw) -> c_int {
    static int clk_sam9x5_main_prepare(struct clk_hw *hw)
    {
    struct clk_sam9x5_main *clkmain = to_clk_sam9x5_main(hw);
    struct regmap *regmap = clkmain.regmap;
    while (!clk_sam9x5_main_ready(regmap))
    cpu_relax();
    return clk_main_probe_frequency(regmap);
    }
#[no_mangle]
unsafe extern "C" fn clk_sam9x5_main_is_prepared(hw: *mut clk_hw) -> c_int {
    static int clk_sam9x5_main_is_prepared(struct clk_hw *hw)
    {
    struct clk_sam9x5_main *clkmain = to_clk_sam9x5_main(hw);
    return clk_sam9x5_main_ready(clkmain.regmap);
    }
    static unsigned long clk_sam9x5_main_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_sam9x5_main *clkmain = to_clk_sam9x5_main(hw);
    return clk_main_recalc_rate(clkmain.regmap, parent_rate);
    }
#[no_mangle]
unsafe extern "C" fn clk_sam9x5_main_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    static int clk_sam9x5_main_set_parent(struct clk_hw *hw, u8 index)
    {
    struct clk_sam9x5_main *clkmain = to_clk_sam9x5_main(hw);
    struct regmap *regmap = clkmain.regmap;
    unsigned int tmp;
    if (index > 1)
    return -EINVAL;
    regmap_read(regmap, AT91_CKGR_MOR, &tmp);
    if (index && !(tmp & AT91_PMC_MOSCSEL))
    tmp = AT91_PMC_MOSCSEL;
#[no_mangle]
pub unsafe extern "C" fn if(AT91_PMC_MOSCSEL): !index && (tmp &) -> else {
    else if (!index && (tmp & AT91_PMC_MOSCSEL))
    tmp = 0;
    else
    return 0;
    regmap_update_bits(regmap, AT91_CKGR_MOR,
    AT91_PMC_MOSCSEL | MOR_KEY_MASK,
    tmp | AT91_PMC_KEY);
    while (!clk_sam9x5_main_ready(regmap))
    cpu_relax();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_sam9x5_main_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 clk_sam9x5_main_get_parent(struct clk_hw *hw)
    {
    struct clk_sam9x5_main *clkmain = to_clk_sam9x5_main(hw);
    unsigned int status;
    regmap_read(clkmain.regmap, AT91_CKGR_MOR, &status);
    return clk_main_parent_select(status);
    }
#[no_mangle]
unsafe extern "C" fn clk_sam9x5_main_save_context(hw: *mut clk_hw) -> c_int {
    static int clk_sam9x5_main_save_context(struct clk_hw *hw)
    {
    struct clk_sam9x5_main *clkmain = to_clk_sam9x5_main(hw);
    clkmain.pms.status = clk_main_rc_osc_is_prepared(&clkmain.hw);
    clkmain.pms.parent = clk_sam9x5_main_get_parent(&clkmain.hw);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_sam9x5_main_restore_context(hw: *mut clk_hw) {
    static void clk_sam9x5_main_restore_context(struct clk_hw *hw)
    {
    struct clk_sam9x5_main *clkmain = to_clk_sam9x5_main(hw);
    int ret;
    ret = clk_sam9x5_main_set_parent(hw, clkmain.pms.parent);
    if (ret)
    return;
    if (clkmain.pms.status)
    clk_sam9x5_main_prepare(hw);
    }
    static const struct clk_ops sam9x5_main_ops = {
    .prepare = clk_sam9x5_main_prepare,
    .is_prepared = clk_sam9x5_main_is_prepared,
    .recalc_rate = clk_sam9x5_main_recalc_rate,
    .determine_rate = clk_hw_determine_rate_no_reparent,
    .set_parent = clk_sam9x5_main_set_parent,
    .get_parent = clk_sam9x5_main_get_parent,
    .save_context = clk_sam9x5_main_save_context,
    .restore_context = clk_sam9x5_main_restore_context,
    };
    struct clk_hw * __init
    at91_clk_register_sam9x5_main(struct regmap *regmap,
    const char *name,
    const char **parent_names,
    struct clk_hw **parent_hws,
    int num_parents)
    {
    struct clk_sam9x5_main *clkmain;
    let mut init: clk_init_data = {};
    unsigned int status;
    struct clk_hw *hw;
    int ret;
    if (!name)
    return ERR_PTR(-EINVAL);
    if (!(parent_hws || parent_names) || !num_parents)
    return ERR_PTR(-EINVAL);
    clkmain = kzalloc_obj(*clkmain);
    if (!clkmain)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.ops = &sam9x5_main_ops;
    if (parent_hws)
    init.parent_hws = (const struct clk_hw **)parent_hws;
    else
    init.parent_names = parent_names;
    init.num_parents = num_parents;
    init.flags = CLK_SET_PARENT_GATE;
    clkmain.hw.init = &init;
    clkmain.regmap = regmap;
    regmap_read(clkmain.regmap, AT91_CKGR_MOR, &status);
    clkmain.parent = clk_main_parent_select(status);
    hw = &clkmain.hw;
    ret = clk_hw_register(core::ptr::null_mut(), &clkmain.hw);
    if (ret) {
    kfree(clkmain);
    hw = ERR_PTR(ret);
    }
    return hw;
    }
