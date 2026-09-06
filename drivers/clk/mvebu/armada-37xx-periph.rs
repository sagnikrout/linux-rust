//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mvebu/armada-37xx-periph.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Marvell Armada 37xx SoC Peripheral clocks
//
// Copyright (C) 2016 Marvell
//
// Gregory CLEMENT <gregory.clement@free-electrons.com>
//
// Most of the peripheral clocks can be modelled like this:
// _____    _______    _______
// TBG-A-P  --|     |  |       |  |       |   ______
// TBG-B-P  --| Mux |--| /div1 |--| /div2 |--| Gate |--> perip_clk
// TBG-A-S  --|     |  |       |  |       |  |______|
// TBG-B-S  --|_____|  |_______|  |_______|
//
// However some clocks may use only one or two block or and use the
// xtal clock as parent.
//

pub const TBG_SEL: c_uint = 0x0;
pub const DIV_SEL0: c_uint = 0x4;
pub const DIV_SEL1: c_uint = 0x8;
pub const DIV_SEL2: c_uint = 0xC;
pub const CLK_SEL: c_uint = 0x10;
pub const CLK_DIS: c_uint = 0x14;
pub const ARMADA_37XX_DVFS_LOAD_1: c_int = 1;
pub const LOAD_LEVEL_NR: c_int = 4;
pub const ARMADA_37XX_NB_L0L1: c_uint = 0x18;
pub const ARMADA_37XX_NB_L2L3: c_uint = 0x1C;
pub const ARMADA_37XX_NB_TBG_DIV_OFF: c_int = 13;
pub const ARMADA_37XX_NB_TBG_DIV_MASK: c_uint = 0x7;
pub const ARMADA_37XX_NB_CLK_SEL_OFF: c_int = 11;
pub const ARMADA_37XX_NB_CLK_SEL_MASK: c_uint = 0x1;
pub const ARMADA_37XX_NB_TBG_SEL_OFF: c_int = 9;
pub const ARMADA_37XX_NB_TBG_SEL_MASK: c_uint = 0x3;
pub const ARMADA_37XX_NB_CONFIG_SHIFT: c_int = 16;
pub const ARMADA_37XX_NB_DYN_MOD: c_uint = 0x24;
pub const ARMADA_37XX_NB_DFS_EN: c_int = 31;
pub const ARMADA_37XX_NB_CPU_LOAD: c_uint = 0x30;
pub const ARMADA_37XX_NB_CPU_LOAD_MASK: c_uint = 0x3;
pub const ARMADA_37XX_DVFS_LOAD_0: c_int = 0;
pub const ARMADA_37XX_DVFS_LOAD_1: c_int = 1;
pub const ARMADA_37XX_DVFS_LOAD_2: c_int = 2;
pub const ARMADA_37XX_DVFS_LOAD_3: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_periph_driver_data {
    pub hw_data: *mut clk_hw_onecell_data,
    pub lock: spinlock_t,
    pub reg: *mut void __iomem,
// Storage registers for suspend/resume operations
    pub tbg_sel: u32,
    pub div_sel0: u32,
    pub div_sel1: u32,
    pub div_sel2: u32,
    pub clk_sel: u32,
    pub clk_dis: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_double_div {
    pub hw: clk_hw,
    pub reg1: *mut void __iomem,
    pub shift1: u8,
    pub reg2: *mut void __iomem,
    pub shift2: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_pm_cpu {
    pub hw: clk_hw,
    pub reg_mux: *mut void __iomem,
    pub shift_mux: u8,
    pub mask_mux: u32,
    pub reg_div: *mut void __iomem,
    pub shift_div: u8,
    pub nb_pm_base: *mut regmap,
    pub l1_expiration: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_periph_data {
    pub name: *const c_char,
    pub parent_names: *const *const c_char,
    pub num_parents: c_int,
    pub mux_hw: *mut clk_hw,
    pub rate_hw: *mut clk_hw,
    pub gate_hw: *mut clk_hw,
    pub muxrate_hw: *mut clk_hw,
    pub is_double_div: bool,
}

    static const struct clk_div_table clk_table6[] = {
    { .val = 1, .div = 1, },
    { .val = 2, .div = 2, },
    { .val = 3, .div = 3, },
    { .val = 4, .div = 4, },
    { .val = 5, .div = 5, },
    { .val = 6, .div = 6, },
    { .val = 0, .div = 0, }, /* last entry */
    };
    static const struct clk_div_table clk_table1[] = {
    { .val = 0, .div = 1, },
    { .val = 1, .div = 2, },
    { .val = 0, .div = 0, }, /* last entry */
    };
    static const struct clk_div_table clk_table2[] = {
    { .val = 0, .div = 2, },
    { .val = 1, .div = 4, },
    { .val = 0, .div = 0, }, /* last entry */
    };
    static const struct clk_ops clk_double_div_ops;
    static const struct clk_ops clk_pm_cpu_ops;

    struct clk_gate gate_##_name = {		\
    .reg = __reg(CLK_DIS),			\
    .bit_idx = _bit,			\
    .hw.init = &(struct clk_init_data){	\
    .ops =  &clk_gate_ops,		\
    }					\
    };

    struct clk_mux mux_##_name = {			\
    .reg = __reg(TBG_SEL),			\
    .shift = _shift,			\
    .mask = 3,				\
    .hw.init = &(struct clk_init_data){	\
    .ops =  &clk_mux_ro_ops,	\
    }					\
    };

    struct clk_double_div rate_##_name = {		\
    .reg1 = __reg(_reg1),			\
    .reg2 = __reg(_reg2),			\
    .shift1 = _shift1,			\
    .shift2 = _shift2,			\
    .hw.init = &(struct clk_init_data){	\
    .ops =  &clk_double_div_ops,	\
    }					\
    };

    struct clk_divider rate_##_name = {		\
    .reg = __reg(_reg),			\
    .table = _table,			\
    .shift = _shift,			\
    .hw.init = &(struct clk_init_data){	\
    .ops =  &clk_divider_ro_ops,	\
    }					\
    };

    struct clk_pm_cpu muxrate_##_name = {		\
    .reg_mux = __reg(TBG_SEL),		\
    .mask_mux = 3,				\
    .shift_mux = _shift1,			\
    .reg_div = __reg(_reg),			\
    .shift_div = _shift2,			\
    .hw.init = &(struct clk_init_data){	\
    .ops =  &clk_pm_cpu_ops,	\
    }					\
    };

    static PERIPH_GATE(_name, _bit);			    \
    static PERIPH_MUX(_name, _shift);			    \
    static PERIPH_DOUBLEDIV(_name, _reg1, _reg2, _shift1, _shift2);

    static PERIPH_GATE(_name, _bit);			    \
    static PERIPH_MUX(_name, _shift);			    \
    static PERIPH_DIV(_name, _reg, _shift1, _table);

    static PERIPH_GATE(_name, _bit);			\
    static PERIPH_DIV(_name, _reg, _shift, _table);

    static PERIPH_MUX(_name, _shift);			    \
    static PERIPH_DOUBLEDIV(_name, _reg1, _reg2, _shift1, _shift2);

    { .name = #_name,				\
    .parent_names = (const char *[]){ "TBG-A-P",	\
    "TBG-B-P", "TBG-A-S", "TBG-B-S"},		\
    .num_parents = 4,				\
    .mux_hw = &mux_##_name.hw,			\
    .gate_hw = &gate_##_name.hw,			\
    .rate_hw = &rate_##_name.hw,			\
    }

    { .name = #_name,				\
    .parent_names = (const char *[]){ "TBG-A-P",	\
    "TBG-B-P", "TBG-A-S", "TBG-B-S"},		\
    .num_parents = 4,				\
    .mux_hw = &mux_##_name.hw,			\
    .gate_hw = &gate_##_name.hw,			\
    .rate_hw = &rate_##_name.hw,			\
    .is_double_div = true,			\
    }

    { .name = #_name,					\
    .parent_names = (const char *[]){ _parent_name},	\
    .num_parents = 1,					\
    .gate_hw = &gate_##_name.hw,				\
    }

    { .name = #_name,					\
    .parent_names = (const char *[]){ _parent_name},	\
    .num_parents = 1,					\
    .gate_hw = &gate_##_name.hw,				\
    .rate_hw = &rate_##_name.hw,				\
    }

    { .name = #_name,				\
    .parent_names = (const char *[]){ "TBG-A-P",	\
    "TBG-B-P", "TBG-A-S", "TBG-B-S"},		\
    .num_parents = 4,				\
    .muxrate_hw = &muxrate_##_name.hw,		\
    }

    { .name = #_name,				\
    .parent_names = (const char *[]){ "TBG-A-P",	\
    "TBG-B-P", "TBG-A-S", "TBG-B-S"},		\
    .num_parents = 4,				\
    .mux_hw = &mux_##_name.hw,			\
    .rate_hw = &rate_##_name.hw,			\
    .is_double_div = true,			\
    }
// NB periph clocks
    PERIPH_CLK_FULL_DD(mmc, 2, 0, DIV_SEL2, DIV_SEL2, 16, 13);
    PERIPH_CLK_FULL_DD(sata_host, 3, 2, DIV_SEL2, DIV_SEL2, 10, 7);
    PERIPH_CLK_FULL_DD(sec_at, 6, 4, DIV_SEL1, DIV_SEL1, 3, 0);
    PERIPH_CLK_FULL_DD(sec_dap, 7, 6, DIV_SEL1, DIV_SEL1, 9, 6);
    PERIPH_CLK_FULL_DD(tscem, 8, 8, DIV_SEL1, DIV_SEL1, 15, 12);
    PERIPH_CLK_FULL(tscem_tmx, 10, 10, DIV_SEL1, 18, clk_table6);
    static PERIPH_GATE(avs, 11);
    PERIPH_CLK_FULL_DD(pwm, 13, 14, DIV_SEL0, DIV_SEL0, 3, 0);
    PERIPH_CLK_FULL_DD(sqf, 12, 12, DIV_SEL1, DIV_SEL1, 27, 24);
    static PERIPH_GATE(i2c_2, 16);
    static PERIPH_GATE(i2c_1, 17);
    PERIPH_CLK_GATE_DIV(ddr_phy, 19, DIV_SEL0, 18, clk_table2);
    PERIPH_CLK_FULL_DD(ddr_fclk, 21, 16, DIV_SEL0, DIV_SEL0, 15, 12);
    PERIPH_CLK_FULL(trace, 22, 18, DIV_SEL0, 20, clk_table6);
    PERIPH_CLK_FULL(counter, 23, 20, DIV_SEL0, 23, clk_table6);
    PERIPH_CLK_FULL_DD(eip97, 24, 24, DIV_SEL2, DIV_SEL2, 22, 19);
    static PERIPH_PM_CPU(cpu, 22, DIV_SEL0, 28);
    static struct clk_periph_data data_nb[] = {
    REF_CLK_FULL_DD(mmc),
    REF_CLK_FULL_DD(sata_host),
    REF_CLK_FULL_DD(sec_at),
    REF_CLK_FULL_DD(sec_dap),
    REF_CLK_FULL_DD(tscem),
    REF_CLK_FULL(tscem_tmx),
    REF_CLK_GATE(avs, "xtal"),
    REF_CLK_FULL_DD(sqf),
    REF_CLK_FULL_DD(pwm),
    REF_CLK_GATE(i2c_2, "xtal"),
    REF_CLK_GATE(i2c_1, "xtal"),
    REF_CLK_GATE_DIV(ddr_phy, "TBG-A-S"),
    REF_CLK_FULL_DD(ddr_fclk),
    REF_CLK_FULL(trace),
    REF_CLK_FULL(counter),
    REF_CLK_FULL_DD(eip97),
    REF_CLK_PM_CPU(cpu),
    { },
    };
// SB periph clocks
    PERIPH_CLK_MUX_DD(gbe_50, 6, DIV_SEL2, DIV_SEL2, 6, 9);
    PERIPH_CLK_MUX_DD(gbe_core, 8, DIV_SEL1, DIV_SEL1, 18, 21);
    PERIPH_CLK_MUX_DD(gbe_125, 10, DIV_SEL1, DIV_SEL1, 6, 9);
    static PERIPH_GATE(gbe1_50, 0);
    static PERIPH_GATE(gbe0_50, 1);
    static PERIPH_GATE(gbe1_125, 2);
    static PERIPH_GATE(gbe0_125, 3);
    PERIPH_CLK_GATE_DIV(gbe1_core, 4, DIV_SEL1, 13, clk_table1);
    PERIPH_CLK_GATE_DIV(gbe0_core, 5, DIV_SEL1, 14, clk_table1);
    PERIPH_CLK_GATE_DIV(gbe_bm, 12, DIV_SEL1, 0, clk_table1);
    PERIPH_CLK_FULL_DD(sdio, 11, 14, DIV_SEL0, DIV_SEL0, 3, 6);
    PERIPH_CLK_FULL_DD(usb32_usb2_sys, 16, 16, DIV_SEL0, DIV_SEL0, 9, 12);
    PERIPH_CLK_FULL_DD(usb32_ss_sys, 17, 18, DIV_SEL0, DIV_SEL0, 15, 18);
    static PERIPH_GATE(pcie, 14);
    static struct clk_periph_data data_sb[] = {
    REF_CLK_MUX_DD(gbe_50),
    REF_CLK_MUX_DD(gbe_core),
    REF_CLK_MUX_DD(gbe_125),
    REF_CLK_GATE(gbe1_50, "gbe_50"),
    REF_CLK_GATE(gbe0_50, "gbe_50"),
    REF_CLK_GATE(gbe1_125, "gbe_125"),
    REF_CLK_GATE(gbe0_125, "gbe_125"),
    REF_CLK_GATE_DIV(gbe1_core, "gbe_core"),
    REF_CLK_GATE_DIV(gbe0_core, "gbe_core"),
    REF_CLK_GATE_DIV(gbe_bm, "gbe_core"),
    REF_CLK_FULL_DD(sdio),
    REF_CLK_FULL_DD(usb32_usb2_sys),
    REF_CLK_FULL_DD(usb32_ss_sys),
    REF_CLK_GATE(pcie, "gbe_core"),
    { },
    };
#[no_mangle]
unsafe extern "C" fn get_div(reg: *mut void __iomem, shift: c_int) -> c_uint {
    static unsigned int get_div(void __iomem *reg, int shift)
    {
    u32 val;
    val = (readl(reg) >> shift) & 0x7;
    if (val > 6)
    return 0;
    return val;
    }
    static unsigned long clk_double_div_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_double_div *double_div = to_clk_double_div(hw);
    unsigned int div;
    div = get_div(double_div.reg1, double_div.shift1);
    div *= get_div(double_div.reg2, double_div.shift2);
    return DIV_ROUND_UP_ULL((u64)parent_rate, div);
    }
    static const struct clk_ops clk_double_div_ops = {
    .recalc_rate = clk_double_div_recalc_rate,
    };
    static void armada_3700_pm_dvfs_update_regs(unsigned int load_level,
    unsigned int *reg,
    unsigned int *offset)
    {
    if (load_level <= ARMADA_37XX_DVFS_LOAD_1)
// reg = ARMADA_37XX_NB_L0L1;
    else
// reg = ARMADA_37XX_NB_L2L3;
    if (load_level == ARMADA_37XX_DVFS_LOAD_0 ||
    load_level ==  ARMADA_37XX_DVFS_LOAD_2)
// offset += ARMADA_37XX_NB_CONFIG_SHIFT;
    }
#[no_mangle]
unsafe extern "C" fn armada_3700_pm_dvfs_is_enabled(base: *mut regmap) -> bool {
    static bool armada_3700_pm_dvfs_is_enabled(struct regmap *base)
    {
    unsigned int val, reg = ARMADA_37XX_NB_DYN_MOD;
    if (IS_ERR(base))
    return false;
    regmap_read(base, reg, &val);
    return !!(val & BIT(ARMADA_37XX_NB_DFS_EN));
    }
#[no_mangle]
unsafe extern "C" fn armada_3700_pm_dvfs_get_cpu_div(base: *mut regmap) -> c_uint {
    static unsigned int armada_3700_pm_dvfs_get_cpu_div(struct regmap *base)
    {
    let mut reg: c_uint = ARMADA_37XX_NB_CPU_LOAD;
    let mut offset: c_uint = ARMADA_37XX_NB_TBG_DIV_OFF;
    unsigned int load_level, div;
//
// This function is always called after the function
// armada_3700_pm_dvfs_is_enabled, so no need to check again
// if the base is valid.
//
    regmap_read(base, reg, &load_level);
//
// The register and the offset inside this register accessed to
// read the current divider depend on the load level
//
    load_level &= ARMADA_37XX_NB_CPU_LOAD_MASK;
    armada_3700_pm_dvfs_update_regs(load_level, &reg, &offset);
    regmap_read(base, reg, &div);
    return (div >> offset) & ARMADA_37XX_NB_TBG_DIV_MASK;
    }
#[no_mangle]
unsafe extern "C" fn armada_3700_pm_dvfs_get_cpu_parent(base: *mut regmap) -> c_uint {
    static unsigned int armada_3700_pm_dvfs_get_cpu_parent(struct regmap *base)
    {
    let mut reg: c_uint = ARMADA_37XX_NB_CPU_LOAD;
    let mut offset: c_uint = ARMADA_37XX_NB_TBG_SEL_OFF;
    unsigned int load_level, sel;
//
// This function is always called after the function
// armada_3700_pm_dvfs_is_enabled, so no need to check again
// if the base is valid
//
    regmap_read(base, reg, &load_level);
//
// The register and the offset inside this register accessed to
// read the current divider depend on the load level
//
    load_level &= ARMADA_37XX_NB_CPU_LOAD_MASK;
    armada_3700_pm_dvfs_update_regs(load_level, &reg, &offset);
    regmap_read(base, reg, &sel);
    return (sel >> offset) & ARMADA_37XX_NB_TBG_SEL_MASK;
    }
#[no_mangle]
unsafe extern "C" fn clk_pm_cpu_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 clk_pm_cpu_get_parent(struct clk_hw *hw)
    {
    struct clk_pm_cpu *pm_cpu = to_clk_pm_cpu(hw);
    u32 val;
    if (armada_3700_pm_dvfs_is_enabled(pm_cpu.nb_pm_base)) {
    val = armada_3700_pm_dvfs_get_cpu_parent(pm_cpu.nb_pm_base);
    } else {
    val = readl(pm_cpu.reg_mux) >> pm_cpu.shift_mux;
    val &= pm_cpu.mask_mux;
    }
    return val;
    }
    static unsigned long clk_pm_cpu_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_pm_cpu *pm_cpu = to_clk_pm_cpu(hw);
    unsigned int div;
    if (armada_3700_pm_dvfs_is_enabled(pm_cpu.nb_pm_base))
    div = armada_3700_pm_dvfs_get_cpu_div(pm_cpu.nb_pm_base);
    else
    div = get_div(pm_cpu.reg_div, pm_cpu.shift_div);
    return DIV_ROUND_UP_ULL((u64)parent_rate, div);
    }
    static int clk_pm_cpu_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct clk_pm_cpu *pm_cpu = to_clk_pm_cpu(hw);
    struct regmap *base = pm_cpu.nb_pm_base;
    let mut div: c_uint = req.best_parent_rate / req.rate;
    unsigned int load_level;
// only available when DVFS is enabled
    if (!armada_3700_pm_dvfs_is_enabled(base))
    return -EINVAL;
    for (load_level = 0; load_level < LOAD_LEVEL_NR; load_level++) {
    unsigned int reg, val, offset = ARMADA_37XX_NB_TBG_DIV_OFF;
    armada_3700_pm_dvfs_update_regs(load_level, &reg, &offset);
    regmap_read(base, reg, &val);
    val >>= offset;
    val &= ARMADA_37XX_NB_TBG_DIV_MASK;
    if (val == div) {
//
// We found a load level matching the target
// divider, switch to this load level and
// return.
//
    req.rate = req.best_parent_rate / div;
    return 0;
    }
    }
// We didn't find any valid divider
    return -EINVAL;
    }
//
// Workaround when base CPU frequnecy is 1000 or 1200 MHz
//
// Switching the CPU from the L2 or L3 frequencies (250/300 or 200 MHz
// respectively) to L0 frequency (1/1.2 GHz) requires a significant
// amount of time to let VDD stabilize to the appropriate
// voltage. This amount of time is large enough that it cannot be
// covered by the hardware countdown register. Due to this, the CPU
// might start operating at L0 before the voltage is stabilized,
// leading to CPU stalls.
//
// To work around this problem, we prevent switching directly from the
// L2/L3 frequencies to the L0 frequency, and instead switch to the L1
// frequency in-between. The sequence therefore becomes:
// 1. First switch from L2/L3 (200/250/300 MHz) to L1 (500/600 MHz)
// 2. Sleep 20ms for stabling VDD voltage
// 3. Then switch from L1 (500/600 MHz) to L0 (1000/1200 MHz).
//
    static void clk_pm_cpu_set_rate_wa(struct clk_pm_cpu *pm_cpu,
    unsigned int new_level, unsigned long rate,
    struct regmap *base)
    {
    unsigned int cur_level;
    regmap_read(base, ARMADA_37XX_NB_CPU_LOAD, &cur_level);
    cur_level &= ARMADA_37XX_NB_CPU_LOAD_MASK;
    if (cur_level == new_level)
    return;
//
// System wants to go to L1 on its own. If we are going from L2/L3,
// remember when 20ms will expire. If from L0, set the value so that
// next switch to L0 won't have to wait.
//
    if (new_level == ARMADA_37XX_DVFS_LOAD_1) {
    if (cur_level == ARMADA_37XX_DVFS_LOAD_0)
    pm_cpu.l1_expiration = jiffies;
    else
    pm_cpu.l1_expiration = jiffies + msecs_to_jiffies(20);
    return;
    }
//
// If we are setting to L2/L3, just invalidate L1 expiration time,
// sleeping is not needed.
//
    if (rate < 1000*1000*1000)
    goto invalidate_l1_exp;
//
// We are going to L0 with rate >= 1GHz. Check whether we have been at
// L1 for long enough time. If not, go to L1 for 20ms.
//
    if (pm_cpu.l1_expiration && time_is_before_eq_jiffies(pm_cpu.l1_expiration))
    goto invalidate_l1_exp;
    regmap_update_bits(base, ARMADA_37XX_NB_CPU_LOAD,
    ARMADA_37XX_NB_CPU_LOAD_MASK,
    ARMADA_37XX_DVFS_LOAD_1);
    msleep(20);
    invalidate_l1_exp:
    pm_cpu.l1_expiration = 0;
    }
    static int clk_pm_cpu_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct clk_pm_cpu *pm_cpu = to_clk_pm_cpu(hw);
    struct regmap *base = pm_cpu.nb_pm_base;
    let mut div: c_uint = parent_rate / rate;
    unsigned int load_level;
// only available when DVFS is enabled
    if (!armada_3700_pm_dvfs_is_enabled(base))
    return -EINVAL;
    for (load_level = 0; load_level < LOAD_LEVEL_NR; load_level++) {
    unsigned int reg, mask, val,
    offset = ARMADA_37XX_NB_TBG_DIV_OFF;
    armada_3700_pm_dvfs_update_regs(load_level, &reg, &offset);
    regmap_read(base, reg, &val);
    val >>= offset;
    val &= ARMADA_37XX_NB_TBG_DIV_MASK;
    if (val == div) {
//
// We found a load level matching the target
// divider, switch to this load level and
// return.
//
    reg = ARMADA_37XX_NB_CPU_LOAD;
    mask = ARMADA_37XX_NB_CPU_LOAD_MASK;
// Apply workaround when base CPU frequency is 1000 or 1200 MHz
    if (parent_rate >= 1000*1000*1000)
    clk_pm_cpu_set_rate_wa(pm_cpu, load_level, rate, base);
    regmap_update_bits(base, reg, mask, load_level);
    return rate;
    }
    }
// We didn't find any valid divider
    return -EINVAL;
    }
    static const struct clk_ops clk_pm_cpu_ops = {
    .get_parent = clk_pm_cpu_get_parent,
    .determine_rate = clk_pm_cpu_determine_rate,
    .set_rate = clk_pm_cpu_set_rate,
    .recalc_rate = clk_pm_cpu_recalc_rate,
    };
    static const struct of_device_id armada_3700_periph_clock_of_match[] = {
    { .compatible = "marvell,armada-3700-periph-clock-nb",
    .data = data_nb, },
    { .compatible = "marvell,armada-3700-periph-clock-sb",
    .data = data_sb, },
    { }
    };
    static int armada_3700_add_composite_clk(const struct clk_periph_data *data,
    void __iomem *reg, spinlock_t *lock,
    struct device *dev, struct clk_hw **hw)
    {
    const struct clk_ops *mux_ops = core::ptr::null_mut(), *gate_ops = core::ptr::null_mut(),
// rate_ops = NULL;
    struct clk_hw *mux_hw = core::ptr::null_mut(), *gate_hw = core::ptr::null_mut(), *rate_hw = core::ptr::null_mut();
    if (data.mux_hw) {
    struct clk_mux *mux;
    mux_hw = data.mux_hw;
    mux = to_clk_mux(mux_hw);
    mux.lock = lock;
    mux_ops = mux_hw.init.ops;
    mux.reg = reg + (u64)mux.reg;
    }
    if (data.gate_hw) {
    struct clk_gate *gate;
    gate_hw = data.gate_hw;
    gate = to_clk_gate(gate_hw);
    gate.lock = lock;
    gate_ops = gate_hw.init.ops;
    gate.reg = reg + (u64)gate.reg;
    gate.flags = CLK_GATE_SET_TO_DISABLE;
    }
    if (data.rate_hw) {
    rate_hw = data.rate_hw;
    rate_ops = rate_hw.init.ops;
    if (data.is_double_div) {
    struct clk_double_div *rate;
    rate =  to_clk_double_div(rate_hw);
    rate.reg1 = reg + (u64)rate.reg1;
    rate.reg2 = reg + (u64)rate.reg2;
    } else {
    struct clk_divider *rate = to_clk_divider(rate_hw);
    const struct clk_div_table *clkt;
    let mut table_size: c_int = 0;
    rate.reg = reg + (u64)rate.reg;
    for (clkt = rate.table; clkt.div; clkt++)
    table_size++;
    rate.width = order_base_2(table_size);
    rate.lock = lock;
    }
    }
    if (data.muxrate_hw) {
    struct clk_pm_cpu *pmcpu_clk;
    struct clk_hw *muxrate_hw = data.muxrate_hw;
    struct regmap *map;
    pmcpu_clk =  to_clk_pm_cpu(muxrate_hw);
    pmcpu_clk.reg_mux = reg + (u64)pmcpu_clk.reg_mux;
    pmcpu_clk.reg_div = reg + (u64)pmcpu_clk.reg_div;
    mux_hw = muxrate_hw;
    rate_hw = muxrate_hw;
    mux_ops = muxrate_hw.init.ops;
    rate_ops = muxrate_hw.init.ops;
    map = syscon_regmap_lookup_by_compatible(
    "marvell,armada-3700-nb-pm");
    pmcpu_clk.nb_pm_base = map;
    }
// hw = clk_hw_register_composite(dev, data->name, data->parent_names,
    data.num_parents, mux_hw,
    mux_ops, rate_hw, rate_ops,
    gate_hw, gate_ops, CLK_IGNORE_UNUSED);
    return PTR_ERR_OR_ZERO(*hw);
    }
#[no_mangle]
unsafe extern "C" fn armada_3700_periph_clock_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused armada_3700_periph_clock_suspend(struct device *dev)
    {
    struct clk_periph_driver_data *data = dev_get_drvdata(dev);
    data.tbg_sel = readl(data.reg + TBG_SEL);
    data.div_sel0 = readl(data.reg + DIV_SEL0);
    data.div_sel1 = readl(data.reg + DIV_SEL1);
    data.div_sel2 = readl(data.reg + DIV_SEL2);
    data.clk_sel = readl(data.reg + CLK_SEL);
    data.clk_dis = readl(data.reg + CLK_DIS);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn armada_3700_periph_clock_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused armada_3700_periph_clock_resume(struct device *dev)
    {
    struct clk_periph_driver_data *data = dev_get_drvdata(dev);
// Follow the same order than what the Cortex-M3 does (ATF code)
    writel(data.clk_dis, data.reg + CLK_DIS);
    writel(data.div_sel0, data.reg + DIV_SEL0);
    writel(data.div_sel1, data.reg + DIV_SEL1);
    writel(data.div_sel2, data.reg + DIV_SEL2);
    writel(data.tbg_sel, data.reg + TBG_SEL);
    writel(data.clk_sel, data.reg + CLK_SEL);
    return 0;
    }
    static const struct dev_pm_ops armada_3700_periph_clock_pm_ops = {
    SET_NOIRQ_SYSTEM_SLEEP_PM_OPS(armada_3700_periph_clock_suspend,
    armada_3700_periph_clock_resume)
    };
#[no_mangle]
unsafe extern "C" fn armada_3700_periph_clock_probe(pdev: *mut platform_device) -> c_int {
    static int armada_3700_periph_clock_probe(struct platform_device *pdev)
    {
    struct clk_periph_driver_data *driver_data;
    struct device_node *np = pdev.dev.of_node;
    const struct clk_periph_data *data;
    struct device *dev = &pdev.dev;
    let mut num_periph: c_int = 0, i, ret;
    data = of_device_get_match_data(dev);
    if (!data)
    return -ENODEV;
    while (data[num_periph].name)
    num_periph++;
    driver_data = devm_kzalloc(dev, sizeof(*driver_data), GFP_KERNEL);
    if (!driver_data)
    return -ENOMEM;
    driver_data.hw_data = devm_kzalloc(dev,
    struct_size(driver_data.hw_data,
    hws, num_periph),
    GFP_KERNEL);
    if (!driver_data.hw_data)
    return -ENOMEM;
    driver_data.hw_data.num = num_periph;
    driver_data.reg = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(driver_data.reg))
    return PTR_ERR(driver_data.reg);
    spin_lock_init(&driver_data.lock);
    for (i = 0; i < num_periph; i++) {
    struct clk_hw **hw = &driver_data.hw_data.hws[i];
    if (armada_3700_add_composite_clk(&data[i], driver_data.reg,
    &driver_data.lock, dev, hw))
    dev_err(dev, "Can't register periph clock %s\n",
    data[i].name);
    }
    ret = of_clk_add_hw_provider(np, of_clk_hw_onecell_get,
    driver_data.hw_data);
    if (ret) {
    for (i = 0; i < num_periph; i++)
    clk_hw_unregister(driver_data.hw_data.hws[i]);
    return ret;
    }
    platform_set_drvdata(pdev, driver_data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn armada_3700_periph_clock_remove(pdev: *mut platform_device) {
    static void armada_3700_periph_clock_remove(struct platform_device *pdev)
    {
    struct clk_periph_driver_data *data = platform_get_drvdata(pdev);
    struct clk_hw_onecell_data *hw_data = data.hw_data;
    int i;
    of_clk_del_provider(pdev.dev.of_node);
    for (i = 0; i < hw_data.num; i++)
    clk_hw_unregister(hw_data.hws[i]);
    }
    static struct platform_driver armada_3700_periph_clock_driver = {
    .probe = armada_3700_periph_clock_probe,
    .remove = armada_3700_periph_clock_remove,
    .driver		= {
    .name	= "marvell-armada-3700-periph-clock",
    .of_match_table = armada_3700_periph_clock_of_match,
    .pm	= &armada_3700_periph_clock_pm_ops,
    },
    };
    builtin_platform_driver(armada_3700_periph_clock_driver);
