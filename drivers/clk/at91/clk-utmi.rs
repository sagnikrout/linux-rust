//! Automatically rewritten from C to Rust
//! Source: drivers/clk/at91/clk-utmi.c
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

//
// The purpose of this clock is to generate a 480 MHz signal. A different
// rate can't be configured.
//
pub const UTMI_RATE: c_int = 480000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_utmi {
    pub hw: clk_hw,
    pub regmap_pmc: *mut regmap,
    pub regmap_sfr: *mut regmap,
    pub pms: at91_clk_pms,
}

#[no_mangle]
pub unsafe extern "C" fn clk_utmi_ready(regmap: *mut regmap) -> bool {
    static inline bool clk_utmi_ready(struct regmap *regmap)
    {
    unsigned int status;
    regmap_read(regmap, AT91_PMC_SR, &status);
    return status & AT91_PMC_LOCKU;
    }
#[no_mangle]
unsafe extern "C" fn clk_utmi_prepare(hw: *mut clk_hw) -> c_int {
    static int clk_utmi_prepare(struct clk_hw *hw)
    {
    struct clk_hw *hw_parent;
    struct clk_utmi *utmi = to_clk_utmi(hw);
    unsigned int uckr = AT91_PMC_UPLLEN | AT91_PMC_UPLLCOUNT |
    AT91_PMC_BIASEN;
    unsigned int utmi_ref_clk_freq;
    unsigned long parent_rate;
//
// If mainck rate is different from 12 MHz, we have to configure the
// FREQ field of the SFR_UTMICKTRIM register to generate properly
// the utmi clock.
//
    hw_parent = clk_hw_get_parent(hw);
    parent_rate = clk_hw_get_rate(hw_parent);
    switch (parent_rate) {
    case 12000000:
    utmi_ref_clk_freq = 0;
    break;
    case 16000000:
    utmi_ref_clk_freq = 1;
    break;
    case 24000000:
    utmi_ref_clk_freq = 2;
    break;
//
// Not supported on SAMA5D2 but it's not an issue since MAINCK
// maximum value is 24 MHz.
//
    case 48000000:
    utmi_ref_clk_freq = 3;
    break;
    default:
    pr_err("UTMICK: unsupported mainck rate\n");
    return -EINVAL;
    }
    if (utmi.regmap_sfr) {
    regmap_update_bits(utmi.regmap_sfr, AT91_SFR_UTMICKTRIM,
    AT91_UTMICKTRIM_FREQ, utmi_ref_clk_freq);
    } else if (utmi_ref_clk_freq) {
    pr_err("UTMICK: sfr node required\n");
    return -EINVAL;
    }
    regmap_update_bits(utmi.regmap_pmc, AT91_CKGR_UCKR, uckr, uckr);
    while (!clk_utmi_ready(utmi.regmap_pmc))
    cpu_relax();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_utmi_is_prepared(hw: *mut clk_hw) -> c_int {
    static int clk_utmi_is_prepared(struct clk_hw *hw)
    {
    struct clk_utmi *utmi = to_clk_utmi(hw);
    return clk_utmi_ready(utmi.regmap_pmc);
    }
#[no_mangle]
unsafe extern "C" fn clk_utmi_unprepare(hw: *mut clk_hw) {
    static void clk_utmi_unprepare(struct clk_hw *hw)
    {
    struct clk_utmi *utmi = to_clk_utmi(hw);
    regmap_update_bits(utmi.regmap_pmc, AT91_CKGR_UCKR,
    AT91_PMC_UPLLEN, 0);
    }
    static unsigned long clk_utmi_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
// UTMI clk rate is fixed.
    return UTMI_RATE;
    }
#[no_mangle]
unsafe extern "C" fn clk_utmi_save_context(hw: *mut clk_hw) -> c_int {
    static int clk_utmi_save_context(struct clk_hw *hw)
    {
    struct clk_utmi *utmi = to_clk_utmi(hw);
    utmi.pms.status = clk_utmi_is_prepared(hw);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_utmi_restore_context(hw: *mut clk_hw) {
    static void clk_utmi_restore_context(struct clk_hw *hw)
    {
    struct clk_utmi *utmi = to_clk_utmi(hw);
    if (utmi.pms.status)
    clk_utmi_prepare(hw);
    }
    static const struct clk_ops utmi_ops = {
    .prepare = clk_utmi_prepare,
    .unprepare = clk_utmi_unprepare,
    .is_prepared = clk_utmi_is_prepared,
    .recalc_rate = clk_utmi_recalc_rate,
    .save_context = clk_utmi_save_context,
    .restore_context = clk_utmi_restore_context,
    };
    static struct clk_hw * __init
    at91_clk_register_utmi_internal(struct regmap *regmap_pmc,
    struct regmap *regmap_sfr,
    const char *name, const char *parent_name,
    struct clk_hw *parent_hw,
    const struct clk_ops *ops, unsigned long flags)
    {
    struct clk_utmi *utmi;
    struct clk_hw *hw;
    let mut init: clk_init_data = {};
    int ret;
    if (!(parent_name || parent_hw))
    return ERR_PTR(-EINVAL);
    utmi = kzalloc_obj(*utmi);
    if (!utmi)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.ops = ops;
    if (parent_hw)
    init.parent_hws = (const struct clk_hw **)&parent_hw;
    else
    init.parent_names = &parent_name;
    init.num_parents = 1;
    init.flags = flags;
    utmi.hw.init = &init;
    utmi.regmap_pmc = regmap_pmc;
    utmi.regmap_sfr = regmap_sfr;
    hw = &utmi.hw;
    ret = clk_hw_register(core::ptr::null_mut(), &utmi.hw);
    if (ret) {
    kfree(utmi);
    hw = ERR_PTR(ret);
    }
    return hw;
    }
    struct clk_hw * __init
    at91_clk_register_utmi(struct regmap *regmap_pmc, struct regmap *regmap_sfr,
    const char *name, const char *parent_name,
    struct clk_hw *parent_hw)
    {
    return at91_clk_register_utmi_internal(regmap_pmc, regmap_sfr, name,
    parent_name, parent_hw, &utmi_ops, CLK_SET_RATE_GATE);
    }
#[no_mangle]
unsafe extern "C" fn clk_utmi_sama7g5_prepare(hw: *mut clk_hw) -> c_int {
    static int clk_utmi_sama7g5_prepare(struct clk_hw *hw)
    {
    struct clk_utmi *utmi = to_clk_utmi(hw);
    struct clk_hw *hw_parent;
    unsigned long parent_rate;
    unsigned int val;
    hw_parent = clk_hw_get_parent(hw);
    parent_rate = clk_hw_get_rate(hw_parent);
    switch (parent_rate) {
    case 16000000:
    val = 0;
    break;
    case 20000000:
    val = 2;
    break;
    case 24000000:
    val = 3;
    break;
    case 32000000:
    val = 5;
    break;
    default:
    pr_err("UTMICK: unsupported main_xtal rate\n");
    return -EINVAL;
    }
    regmap_write(utmi.regmap_pmc, AT91_PMC_XTALF, val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_utmi_sama7g5_is_prepared(hw: *mut clk_hw) -> c_int {
    static int clk_utmi_sama7g5_is_prepared(struct clk_hw *hw)
    {
    struct clk_utmi *utmi = to_clk_utmi(hw);
    struct clk_hw *hw_parent;
    unsigned long parent_rate;
    unsigned int val;
    hw_parent = clk_hw_get_parent(hw);
    parent_rate = clk_hw_get_rate(hw_parent);
    regmap_read(utmi.regmap_pmc, AT91_PMC_XTALF, &val);
    switch (val & 0x7) {
    case 0:
    if (parent_rate == 16000000)
    return 1;
    break;
    case 2:
    if (parent_rate == 20000000)
    return 1;
    break;
    case 3:
    if (parent_rate == 24000000)
    return 1;
    break;
    case 5:
    if (parent_rate == 32000000)
    return 1;
    break;
    default:
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_utmi_sama7g5_save_context(hw: *mut clk_hw) -> c_int {
    static int clk_utmi_sama7g5_save_context(struct clk_hw *hw)
    {
    struct clk_utmi *utmi = to_clk_utmi(hw);
    utmi.pms.status = clk_utmi_sama7g5_is_prepared(hw);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_utmi_sama7g5_restore_context(hw: *mut clk_hw) {
    static void clk_utmi_sama7g5_restore_context(struct clk_hw *hw)
    {
    struct clk_utmi *utmi = to_clk_utmi(hw);
    if (utmi.pms.status)
    clk_utmi_sama7g5_prepare(hw);
    }
    static const struct clk_ops sama7g5_utmi_ops = {
    .prepare = clk_utmi_sama7g5_prepare,
    .is_prepared = clk_utmi_sama7g5_is_prepared,
    .recalc_rate = clk_utmi_recalc_rate,
    .save_context = clk_utmi_sama7g5_save_context,
    .restore_context = clk_utmi_sama7g5_restore_context,
    };
    struct clk_hw * __init
    at91_clk_sama7g5_register_utmi(struct regmap *regmap_pmc, const char *name,
    const char *parent_name, struct clk_hw *parent_hw)
    {
    return at91_clk_register_utmi_internal(regmap_pmc, core::ptr::null_mut(), name,
    parent_name, parent_hw, &sama7g5_utmi_ops, 0);
    }
