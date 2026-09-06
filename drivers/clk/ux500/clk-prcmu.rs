//! Automatically rewritten from C to Rust
//! Source: drivers/clk/ux500/clk-prcmu.c
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
// PRCMU clock implementation for ux500 platform.
//
// Copyright (C) 2012 ST-Ericsson SA
// Author: Ulf Hansson <ulf.hansson@linaro.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_prcmu {
    pub hw: clk_hw,
    pub cg_sel: u8,
    pub opp_requested: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_prcmu_clkout {
    pub hw: clk_hw,
    pub clkout_id: u8,
    pub source: u8,
    pub divider: u8,
}

// PRCMU clock operations.
#[no_mangle]
unsafe extern "C" fn clk_prcmu_prepare(hw: *mut clk_hw) -> c_int {
    static int clk_prcmu_prepare(struct clk_hw *hw)
    {
    struct clk_prcmu *clk = to_clk_prcmu(hw);
    return db8500_prcmu_request_clock(clk.cg_sel, true);
    }
#[no_mangle]
unsafe extern "C" fn clk_prcmu_unprepare(hw: *mut clk_hw) {
    static void clk_prcmu_unprepare(struct clk_hw *hw)
    {
    struct clk_prcmu *clk = to_clk_prcmu(hw);
    if (db8500_prcmu_request_clock(clk.cg_sel, false))
    pr_err("clk_prcmu: %s failed to disable %s.\n", __func__,
    clk_hw_get_name(hw));
    }
    static unsigned long clk_prcmu_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_prcmu *clk = to_clk_prcmu(hw);
    return prcmu_clock_rate(clk.cg_sel);
    }
    static int clk_prcmu_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct clk_prcmu *clk = to_clk_prcmu(hw);
    req.rate = prcmu_round_clock_rate(clk.cg_sel, req.rate);
    return 0;
    }
    static int clk_prcmu_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct clk_prcmu *clk = to_clk_prcmu(hw);
    return prcmu_set_clock_rate(clk.cg_sel, rate);
    }
#[no_mangle]
unsafe extern "C" fn clk_prcmu_opp_prepare(hw: *mut clk_hw) -> c_int {
    static int clk_prcmu_opp_prepare(struct clk_hw *hw)
    {
    int err;
    struct clk_prcmu *clk = to_clk_prcmu(hw);
    if (!clk.opp_requested) {
    err = prcmu_qos_add_requirement(PRCMU_QOS_APE_OPP,
    (char *)clk_hw_get_name(hw),
    100);
    if (err) {
    pr_err("clk_prcmu: %s fail req APE OPP for %s.\n",
    __func__, clk_hw_get_name(hw));
    return err;
    }
    clk.opp_requested = 1;
    }
    err = db8500_prcmu_request_clock(clk.cg_sel, true);
    if (err) {
    prcmu_qos_remove_requirement(PRCMU_QOS_APE_OPP,
    (char *)clk_hw_get_name(hw));
    clk.opp_requested = 0;
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_prcmu_opp_unprepare(hw: *mut clk_hw) {
    static void clk_prcmu_opp_unprepare(struct clk_hw *hw)
    {
    struct clk_prcmu *clk = to_clk_prcmu(hw);
    if (db8500_prcmu_request_clock(clk.cg_sel, false)) {
    pr_err("clk_prcmu: %s failed to disable %s.\n", __func__,
    clk_hw_get_name(hw));
    return;
    }
    if (clk.opp_requested) {
    prcmu_qos_remove_requirement(PRCMU_QOS_APE_OPP,
    (char *)clk_hw_get_name(hw));
    clk.opp_requested = 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn clk_prcmu_opp_volt_prepare(hw: *mut clk_hw) -> c_int {
    static int clk_prcmu_opp_volt_prepare(struct clk_hw *hw)
    {
    int err;
    struct clk_prcmu *clk = to_clk_prcmu(hw);
    if (!clk.opp_requested) {
    err = db8500_prcmu_request_ape_opp_100_voltage(true);
    if (err) {
    pr_err("clk_prcmu: %s fail req APE OPP VOLT for %s.\n",
    __func__, clk_hw_get_name(hw));
    return err;
    }
    clk.opp_requested = 1;
    }
    err = db8500_prcmu_request_clock(clk.cg_sel, true);
    if (err) {
    db8500_prcmu_request_ape_opp_100_voltage(false);
    clk.opp_requested = 0;
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_prcmu_opp_volt_unprepare(hw: *mut clk_hw) {
    static void clk_prcmu_opp_volt_unprepare(struct clk_hw *hw)
    {
    struct clk_prcmu *clk = to_clk_prcmu(hw);
    if (db8500_prcmu_request_clock(clk.cg_sel, false)) {
    pr_err("clk_prcmu: %s failed to disable %s.\n", __func__,
    clk_hw_get_name(hw));
    return;
    }
    if (clk.opp_requested) {
    db8500_prcmu_request_ape_opp_100_voltage(false);
    clk.opp_requested = 0;
    }
    }
    static const struct clk_ops clk_prcmu_scalable_ops = {
    .prepare = clk_prcmu_prepare,
    .unprepare = clk_prcmu_unprepare,
    .recalc_rate = clk_prcmu_recalc_rate,
    .determine_rate = clk_prcmu_determine_rate,
    .set_rate = clk_prcmu_set_rate,
    };
    static const struct clk_ops clk_prcmu_gate_ops = {
    .prepare = clk_prcmu_prepare,
    .unprepare = clk_prcmu_unprepare,
    .recalc_rate = clk_prcmu_recalc_rate,
    };
    static const struct clk_ops clk_prcmu_scalable_rate_ops = {
    .recalc_rate = clk_prcmu_recalc_rate,
    .determine_rate = clk_prcmu_determine_rate,
    .set_rate = clk_prcmu_set_rate,
    };
    static const struct clk_ops clk_prcmu_rate_ops = {
    .recalc_rate = clk_prcmu_recalc_rate,
    };
    static const struct clk_ops clk_prcmu_opp_gate_ops = {
    .prepare = clk_prcmu_opp_prepare,
    .unprepare = clk_prcmu_opp_unprepare,
    .recalc_rate = clk_prcmu_recalc_rate,
    };
    static const struct clk_ops clk_prcmu_opp_volt_scalable_ops = {
    .prepare = clk_prcmu_opp_volt_prepare,
    .unprepare = clk_prcmu_opp_volt_unprepare,
    .recalc_rate = clk_prcmu_recalc_rate,
    .determine_rate = clk_prcmu_determine_rate,
    .set_rate = clk_prcmu_set_rate,
    };
    static struct clk_hw *clk_reg_prcmu(const char *name,
    const char *parent_name,
    u8 cg_sel,
    unsigned long rate,
    unsigned long flags,
    const struct clk_ops *clk_prcmu_ops)
    {
    struct clk_prcmu *clk;
    struct clk_init_data clk_prcmu_init;
    int ret;
    if (!name) {
    pr_err("clk_prcmu: %s invalid arguments passed\n", __func__);
    return ERR_PTR(-EINVAL);
    }
    clk = kzalloc_obj(*clk);
    if (!clk)
    return ERR_PTR(-ENOMEM);
    clk.cg_sel = cg_sel;
    clk.opp_requested = 0;
// "rate" can be used for changing the initial frequency
    if (rate)
    prcmu_set_clock_rate(cg_sel, rate);
    clk_prcmu_init.name = name;
    clk_prcmu_init.ops = clk_prcmu_ops;
    clk_prcmu_init.flags = flags;
    clk_prcmu_init.parent_names = (parent_name ? &parent_name : core::ptr::null_mut());
    clk_prcmu_init.num_parents = (parent_name ? 1 : 0);
    clk.hw.init = &clk_prcmu_init;
    ret = clk_hw_register(core::ptr::null_mut(), &clk.hw);
    if (ret)
    goto free_clk;
    return &clk.hw;
    free_clk:
    kfree(clk);
    pr_err("clk_prcmu: %s failed to register clk\n", __func__);
    return ERR_PTR(-ENOMEM);
    }
    struct clk_hw *clk_reg_prcmu_scalable(const char *name,
    const char *parent_name,
    u8 cg_sel,
    unsigned long rate,
    unsigned long flags)
    {
    return clk_reg_prcmu(name, parent_name, cg_sel, rate, flags,
    &clk_prcmu_scalable_ops);
    }
    struct clk_hw *clk_reg_prcmu_gate(const char *name,
    const char *parent_name,
    u8 cg_sel,
    unsigned long flags)
    {
    return clk_reg_prcmu(name, parent_name, cg_sel, 0, flags,
    &clk_prcmu_gate_ops);
    }
    struct clk_hw *clk_reg_prcmu_scalable_rate(const char *name,
    const char *parent_name,
    u8 cg_sel,
    unsigned long rate,
    unsigned long flags)
    {
    return clk_reg_prcmu(name, parent_name, cg_sel, rate, flags,
    &clk_prcmu_scalable_rate_ops);
    }
    struct clk_hw *clk_reg_prcmu_rate(const char *name,
    const char *parent_name,
    u8 cg_sel,
    unsigned long flags)
    {
    return clk_reg_prcmu(name, parent_name, cg_sel, 0, flags,
    &clk_prcmu_rate_ops);
    }
    struct clk_hw *clk_reg_prcmu_opp_gate(const char *name,
    const char *parent_name,
    u8 cg_sel,
    unsigned long flags)
    {
    return clk_reg_prcmu(name, parent_name, cg_sel, 0, flags,
    &clk_prcmu_opp_gate_ops);
    }
    struct clk_hw *clk_reg_prcmu_opp_volt_scalable(const char *name,
    const char *parent_name,
    u8 cg_sel,
    unsigned long rate,
    unsigned long flags)
    {
    return clk_reg_prcmu(name, parent_name, cg_sel, rate, flags,
    &clk_prcmu_opp_volt_scalable_ops);
    }
// The clkout (external) clock is special and need special ops
#[no_mangle]
unsafe extern "C" fn clk_prcmu_clkout_prepare(hw: *mut clk_hw) -> c_int {
    static int clk_prcmu_clkout_prepare(struct clk_hw *hw)
    {
    struct clk_prcmu_clkout *clk = to_clk_prcmu_clkout(hw);
    return prcmu_config_clkout(clk.clkout_id, clk.source, clk.divider);
    }
#[no_mangle]
unsafe extern "C" fn clk_prcmu_clkout_unprepare(hw: *mut clk_hw) {
    static void clk_prcmu_clkout_unprepare(struct clk_hw *hw)
    {
    struct clk_prcmu_clkout *clk = to_clk_prcmu_clkout(hw);
    int ret;
// The clkout clock is disabled by dividing by 0
    ret = prcmu_config_clkout(clk.clkout_id, clk.source, 0);
    if (ret)
    pr_err("clk_prcmu: %s failed to disable %s\n", __func__,
    clk_hw_get_name(hw));
    }
    static unsigned long clk_prcmu_clkout_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_prcmu_clkout *clk = to_clk_prcmu_clkout(hw);
    return (parent_rate / clk.divider);
    }
#[no_mangle]
unsafe extern "C" fn clk_prcmu_clkout_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 clk_prcmu_clkout_get_parent(struct clk_hw *hw)
    {
    struct clk_prcmu_clkout *clk = to_clk_prcmu_clkout(hw);
    return clk.source;
    }
#[no_mangle]
unsafe extern "C" fn clk_prcmu_clkout_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    static int clk_prcmu_clkout_set_parent(struct clk_hw *hw, u8 index)
    {
    struct clk_prcmu_clkout *clk = to_clk_prcmu_clkout(hw);
    clk.source = index;
// Make sure the change reaches the hardware immediately
    if (clk_hw_is_prepared(hw))
    return clk_prcmu_clkout_prepare(hw);
    return 0;
    }
    static const struct clk_ops clk_prcmu_clkout_ops = {
    .prepare = clk_prcmu_clkout_prepare,
    .unprepare = clk_prcmu_clkout_unprepare,
    .recalc_rate = clk_prcmu_clkout_recalc_rate,
    .determine_rate = clk_hw_determine_rate_no_reparent,
    .get_parent = clk_prcmu_clkout_get_parent,
    .set_parent = clk_prcmu_clkout_set_parent,
    };
    struct clk_hw *clk_reg_prcmu_clkout(const char *name,
    const char * const *parent_names,
    int num_parents,
    u8 source, u8 divider)
    {
    struct clk_prcmu_clkout *clk;
    struct clk_init_data clk_prcmu_clkout_init;
    u8 clkout_id;
    int ret;
    if (!name) {
    pr_err("clk_prcmu_clkout: %s invalid arguments passed\n", __func__);
    return ERR_PTR(-EINVAL);
    }
    if (!strcmp(name, "clkout1"))
    clkout_id = 0;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(name, _arg: "clkout2")) -> else {
    else if (!strcmp(name, "clkout2"))
    clkout_id = 1;
    else {
    pr_err("clk_prcmu_clkout: %s bad clock name\n", __func__);
    return ERR_PTR(-EINVAL);
    }
    clk = kzalloc_obj(*clk);
    if (!clk)
    return ERR_PTR(-ENOMEM);
    clk.clkout_id = clkout_id;
    clk.source = source;
    clk.divider = divider;
    clk_prcmu_clkout_init.name = name;
    clk_prcmu_clkout_init.ops = &clk_prcmu_clkout_ops;
    clk_prcmu_clkout_init.flags = CLK_GET_RATE_NOCACHE;
    clk_prcmu_clkout_init.parent_names = parent_names;
    clk_prcmu_clkout_init.num_parents = num_parents;
    clk.hw.init = &clk_prcmu_clkout_init;
    ret = clk_hw_register(core::ptr::null_mut(), &clk.hw);
    if (ret)
    goto free_clkout;
    return &clk.hw;
    free_clkout:
    kfree(clk);
    pr_err("clk_prcmu_clkout: %s failed to register clk\n", __func__);
    return ERR_PTR(-ENOMEM);
    }
