//! Automatically rewritten from C to Rust
//! Source: drivers/clk/ti/clk-3xxx.c
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
// OMAP3 Clock init
//
// Copyright (C) 2013 Texas Instruments, Inc
// Tero Kristo (t-kristo@ti.com)
//

pub const OMAP3430ES2_ST_DSS_IDLE_SHIFT: c_int = 1;
pub const OMAP3430ES2_ST_HSOTGUSB_IDLE_SHIFT: c_int = 5;
pub const OMAP3430ES2_ST_SSI_IDLE_SHIFT: c_int = 8;
pub const OMAP34XX_CM_IDLEST_VAL: c_int = 1;
//
// In AM35xx IPSS, the {ICK,FCK} enable bits for modules are exported
// in the same register at a bit offset of 0x8. The EN_ACK for ICK is
// at an offset of 4 from ICK enable bit.
//
pub const AM35XX_IPSS_ICK_MASK: c_uint = 0xF;
pub const AM35XX_IPSS_ICK_EN_ACK_OFFSET: c_uint = 0x4;
pub const AM35XX_IPSS_ICK_FCK_OFFSET: c_uint = 0x8;
pub const AM35XX_IPSS_CLK_IDLEST_VAL: c_int = 0;
pub const AM35XX_ST_IPSS_SHIFT: c_int = 5;
//
// omap3430es2_clk_ssi_find_idlest - return CM_IDLEST info for SSI
// @clk: struct clk * being enabled
// @idlest_reg: void __iomem ** to store CM_IDLEST reg address into
// @idlest_bit: pointer to a u8 to store the CM_IDLEST bit shift into
// @idlest_val: pointer to a u8 to store the CM_IDLEST indicator
//
// The OMAP3430ES2 SSI target CM_IDLEST bit is at a different shift
// from the CM_{I,F}CLKEN bit.  Pass back the correct info via
// @idlest_reg and @idlest_bit.  No return value.
//
    static void omap3430es2_clk_ssi_find_idlest(struct clk_hw_omap *clk,
    struct clk_omap_reg *idlest_reg,
    u8 *idlest_bit,
    u8 *idlest_val)
    {
    memcpy(idlest_reg, &clk.enable_reg, sizeof(*idlest_reg));
    idlest_reg.offset &= ~0xf0;
    idlest_reg.offset |= 0x20;
// idlest_bit = OMAP3430ES2_ST_SSI_IDLE_SHIFT;
// idlest_val = OMAP34XX_CM_IDLEST_VAL;
    }
    const struct clk_hw_omap_ops clkhwops_omap3430es2_iclk_ssi_wait = {
    .allow_idle	= omap2_clkt_iclk_allow_idle,
    .deny_idle	= omap2_clkt_iclk_deny_idle,
    .find_idlest	= omap3430es2_clk_ssi_find_idlest,
    .find_companion	= omap2_clk_dflt_find_companion,
    };
//
// omap3430es2_clk_dss_usbhost_find_idlest - CM_IDLEST info for DSS, USBHOST
// @clk: struct clk * being enabled
// @idlest_reg: void __iomem ** to store CM_IDLEST reg address into
// @idlest_bit: pointer to a u8 to store the CM_IDLEST bit shift into
// @idlest_val: pointer to a u8 to store the CM_IDLEST indicator
//
// Some OMAP modules on OMAP3 ES2+ chips have both initiator and
// target IDLEST bits.  For our purposes, we are concerned with the
// target IDLEST bits, which exist at a different bit position than
// the *CLKEN bit position for these modules (DSS and USBHOST) (The
// default find_idlest code assumes that they are at the same
// position.)  No return value.
//
    static void
    omap3430es2_clk_dss_usbhost_find_idlest(struct clk_hw_omap *clk,
    struct clk_omap_reg *idlest_reg,
    u8 *idlest_bit, u8 *idlest_val)
    {
    memcpy(idlest_reg, &clk.enable_reg, sizeof(*idlest_reg));
    idlest_reg.offset &= ~0xf0;
    idlest_reg.offset |= 0x20;
// USBHOST_IDLE has same shift
// idlest_bit = OMAP3430ES2_ST_DSS_IDLE_SHIFT;
// idlest_val = OMAP34XX_CM_IDLEST_VAL;
    }
    const struct clk_hw_omap_ops clkhwops_omap3430es2_dss_usbhost_wait = {
    .find_idlest	= omap3430es2_clk_dss_usbhost_find_idlest,
    .find_companion	= omap2_clk_dflt_find_companion,
    };
    const struct clk_hw_omap_ops clkhwops_omap3430es2_iclk_dss_usbhost_wait = {
    .allow_idle	= omap2_clkt_iclk_allow_idle,
    .deny_idle	= omap2_clkt_iclk_deny_idle,
    .find_idlest	= omap3430es2_clk_dss_usbhost_find_idlest,
    .find_companion	= omap2_clk_dflt_find_companion,
    };
//
// omap3430es2_clk_hsotgusb_find_idlest - return CM_IDLEST info for HSOTGUSB
// @clk: struct clk * being enabled
// @idlest_reg: void __iomem ** to store CM_IDLEST reg address into
// @idlest_bit: pointer to a u8 to store the CM_IDLEST bit shift into
// @idlest_val: pointer to a u8 to store the CM_IDLEST indicator
//
// The OMAP3430ES2 HSOTGUSB target CM_IDLEST bit is at a different
// shift from the CM_{I,F}CLKEN bit.  Pass back the correct info via
// @idlest_reg and @idlest_bit.  No return value.
//
    static void
    omap3430es2_clk_hsotgusb_find_idlest(struct clk_hw_omap *clk,
    struct clk_omap_reg *idlest_reg,
    u8 *idlest_bit,
    u8 *idlest_val)
    {
    memcpy(idlest_reg, &clk.enable_reg, sizeof(*idlest_reg));
    idlest_reg.offset &= ~0xf0;
    idlest_reg.offset |= 0x20;
// idlest_bit = OMAP3430ES2_ST_HSOTGUSB_IDLE_SHIFT;
// idlest_val = OMAP34XX_CM_IDLEST_VAL;
    }
    const struct clk_hw_omap_ops clkhwops_omap3430es2_iclk_hsotgusb_wait = {
    .allow_idle	= omap2_clkt_iclk_allow_idle,
    .deny_idle	= omap2_clkt_iclk_deny_idle,
    .find_idlest	= omap3430es2_clk_hsotgusb_find_idlest,
    .find_companion	= omap2_clk_dflt_find_companion,
    };
//
// am35xx_clk_find_idlest - return clock ACK info for AM35XX IPSS
// @clk: struct clk * being enabled
// @idlest_reg: void __iomem ** to store CM_IDLEST reg address into
// @idlest_bit: pointer to a u8 to store the CM_IDLEST bit shift into
// @idlest_val: pointer to a u8 to store the CM_IDLEST indicator
//
// The interface clocks on AM35xx IPSS reflects the clock idle status
// in the enable register itsel at a bit offset of 4 from the enable
// bit. A value of 1 indicates that clock is enabled.
//
    static void am35xx_clk_find_idlest(struct clk_hw_omap *clk,
    struct clk_omap_reg *idlest_reg,
    u8 *idlest_bit,
    u8 *idlest_val)
    {
    memcpy(idlest_reg, &clk.enable_reg, sizeof(*idlest_reg));
// idlest_bit = clk->enable_bit + AM35XX_IPSS_ICK_EN_ACK_OFFSET;
// idlest_val = AM35XX_IPSS_CLK_IDLEST_VAL;
    }
//
// am35xx_clk_find_companion - find companion clock to @clk
// @clk: struct clk * to find the companion clock of
// @other_reg: void __iomem ** to return the companion clock CM_*CLKEN va in
// @other_bit: u8 ** to return the companion clock bit shift in
//
// Some clocks don't have companion clocks.  For example, modules with
// only an interface clock (such as HECC) don't have a companion
// clock.  Right now, this code relies on the hardware exporting a bit
// in the correct companion register that indicates that the
// nonexistent 'companion clock' is active.  Future patches will
// associate this type of code with per-module data structures to
// avoid this issue, and remove the casts.  No return value.
//
    static void am35xx_clk_find_companion(struct clk_hw_omap *clk,
    struct clk_omap_reg *other_reg,
    u8 *other_bit)
    {
    memcpy(other_reg, &clk.enable_reg, sizeof(*other_reg));
    if (clk.enable_bit & AM35XX_IPSS_ICK_MASK)
// other_bit = clk->enable_bit + AM35XX_IPSS_ICK_FCK_OFFSET;
    else
// other_bit = clk->enable_bit - AM35XX_IPSS_ICK_FCK_OFFSET;
    }
    const struct clk_hw_omap_ops clkhwops_am35xx_ipss_module_wait = {
    .find_idlest	= am35xx_clk_find_idlest,
    .find_companion	= am35xx_clk_find_companion,
    };
//
// am35xx_clk_ipss_find_idlest - return CM_IDLEST info for IPSS
// @clk: struct clk * being enabled
// @idlest_reg: void __iomem ** to store CM_IDLEST reg address into
// @idlest_bit: pointer to a u8 to store the CM_IDLEST bit shift into
// @idlest_val: pointer to a u8 to store the CM_IDLEST indicator
//
// The IPSS target CM_IDLEST bit is at a different shift from the
// CM_{I,F}CLKEN bit.  Pass back the correct info via @idlest_reg
// and @idlest_bit.  No return value.
//
    static void am35xx_clk_ipss_find_idlest(struct clk_hw_omap *clk,
    struct clk_omap_reg *idlest_reg,
    u8 *idlest_bit,
    u8 *idlest_val)
    {
    memcpy(idlest_reg, &clk.enable_reg, sizeof(*idlest_reg));
    idlest_reg.offset &= ~0xf0;
    idlest_reg.offset |= 0x20;
// idlest_bit = AM35XX_ST_IPSS_SHIFT;
// idlest_val = OMAP34XX_CM_IDLEST_VAL;
    }
    const struct clk_hw_omap_ops clkhwops_am35xx_ipss_wait = {
    .allow_idle	= omap2_clkt_iclk_allow_idle,
    .deny_idle	= omap2_clkt_iclk_deny_idle,
    .find_idlest	= am35xx_clk_ipss_find_idlest,
    .find_companion	= omap2_clk_dflt_find_companion,
    };
    static struct ti_dt_clk omap3xxx_clks[] = {
    DT_CLK(core::ptr::null_mut(), "timer_32k_ck", "omap_32k_fck"),
    DT_CLK(core::ptr::null_mut(), "timer_sys_ck", "sys_ck"),
    { .node_name = core::ptr::null_mut() },
    };
    static struct ti_dt_clk omap36xx_omap3430es2plus_clks[] = {
    DT_CLK(core::ptr::null_mut(), "ssi_ssr_fck", "ssi_ssr_fck_3430es2"),
    DT_CLK(core::ptr::null_mut(), "ssi_sst_fck", "ssi_sst_fck_3430es2"),
    DT_CLK(core::ptr::null_mut(), "hsotgusb_ick", "hsotgusb_ick_3430es2"),
    DT_CLK(core::ptr::null_mut(), "ssi_ick", "ssi_ick_3430es2"),
    { .node_name = core::ptr::null_mut() },
    };
    static struct ti_dt_clk omap3430es1_clks[] = {
    DT_CLK(core::ptr::null_mut(), "ssi_ssr_fck", "ssi_ssr_fck_3430es1"),
    DT_CLK(core::ptr::null_mut(), "ssi_sst_fck", "ssi_sst_fck_3430es1"),
    DT_CLK(core::ptr::null_mut(), "hsotgusb_ick", "hsotgusb_ick_3430es1"),
    DT_CLK(core::ptr::null_mut(), "ssi_ick", "ssi_ick_3430es1"),
    DT_CLK(core::ptr::null_mut(), "dss1_alwon_fck", "dss1_alwon_fck_3430es1"),
    DT_CLK(core::ptr::null_mut(), "dss_ick", "dss_ick_3430es1"),
    { .node_name = core::ptr::null_mut() },
    };
    static struct ti_dt_clk omap36xx_am35xx_omap3430es2plus_clks[] = {
    DT_CLK(core::ptr::null_mut(), "dss1_alwon_fck", "dss1_alwon_fck_3430es2"),
    DT_CLK(core::ptr::null_mut(), "dss_ick", "dss_ick_3430es2"),
    { .node_name = core::ptr::null_mut() },
    };
    static struct ti_dt_clk am35xx_clks[] = {
    DT_CLK(core::ptr::null_mut(), "hsotgusb_ick", "hsotgusb_ick_am35xx"),
    DT_CLK(core::ptr::null_mut(), "hsotgusb_fck", "hsotgusb_fck_am35xx"),
    DT_CLK(core::ptr::null_mut(), "uart4_ick", "uart4_ick_am35xx"),
    DT_CLK(core::ptr::null_mut(), "uart4_fck", "uart4_fck_am35xx"),
    { .node_name = core::ptr::null_mut() },
    };
    static const char *enable_init_clks[] = {
    "sdrc_ick",
    "gpmc_fck",
    "omapctrl_ick",
    };
    enum {
    OMAP3_SOC_AM35XX,
    OMAP3_SOC_OMAP3430_ES1,
    OMAP3_SOC_OMAP3430_ES2_PLUS,
    OMAP3_SOC_OMAP3630,
    };
//
// omap3_clk_lock_dpll5 - locks DPLL5
//
// Locks DPLL5 to a pre-defined frequency. This is required for proper
// operation of USB.
//
#[no_mangle]
pub unsafe extern "C" fn omap3_clk_lock_dpll5() -> void __init {
    void __init omap3_clk_lock_dpll5(void)
    {
    struct clk *dpll5_clk;
    struct clk *dpll5_m2_clk;
//
// Errata sprz319f advisory 2.1 documents a USB host clock drift issue
// that can be worked around using specially crafted dpll5 settings
// with a dpll5_m2 divider set to 8. Set the dpll5 rate to 8x the USB
// host clock rate, its .set_rate handler() will detect that frequency
// and use the errata settings.
//
    dpll5_clk = clk_get(core::ptr::null_mut(), "dpll5_ck");
    clk_set_rate(dpll5_clk, OMAP3_DPLL5_FREQ_FOR_USBHOST * 8);
    clk_prepare_enable(dpll5_clk);
// Program dpll5_m2_clk divider
    dpll5_m2_clk = clk_get(core::ptr::null_mut(), "dpll5_m2_ck");
    clk_prepare_enable(dpll5_m2_clk);
    clk_set_rate(dpll5_m2_clk, OMAP3_DPLL5_FREQ_FOR_USBHOST);
    clk_disable_unprepare(dpll5_m2_clk);
    clk_disable_unprepare(dpll5_clk);
    }
#[no_mangle]
unsafe extern "C" fn omap3xxx_dt_clk_init(soc_type: c_int) -> int __init {
    static int __init omap3xxx_dt_clk_init(int soc_type)
    {
    if (soc_type == OMAP3_SOC_AM35XX || soc_type == OMAP3_SOC_OMAP3630 ||
    soc_type == OMAP3_SOC_OMAP3430_ES1 ||
    soc_type == OMAP3_SOC_OMAP3430_ES2_PLUS)
    ti_dt_clocks_register(omap3xxx_clks);
    if (soc_type == OMAP3_SOC_AM35XX)
    ti_dt_clocks_register(am35xx_clks);
    if (soc_type == OMAP3_SOC_OMAP3630 || soc_type == OMAP3_SOC_AM35XX ||
    soc_type == OMAP3_SOC_OMAP3430_ES2_PLUS)
    ti_dt_clocks_register(omap36xx_am35xx_omap3430es2plus_clks);
    if (soc_type == OMAP3_SOC_OMAP3430_ES1)
    ti_dt_clocks_register(omap3430es1_clks);
    if (soc_type == OMAP3_SOC_OMAP3430_ES2_PLUS ||
    soc_type == OMAP3_SOC_OMAP3630)
    ti_dt_clocks_register(omap36xx_omap3430es2plus_clks);
    omap2_clk_disable_autoidle_all();
    ti_clk_add_aliases();
    omap2_clk_enable_init_clocks(enable_init_clks,
    ARRAY_SIZE(enable_init_clks));
    pr_info("Clocking rate (Crystal/Core/MPU): %ld.%01ld/%ld/%ld MHz\n",
    (clk_get_rate(clk_get_sys(core::ptr::null_mut(), "osc_sys_ck")) / 1000000),
    (clk_get_rate(clk_get_sys(core::ptr::null_mut(), "osc_sys_ck")) / 100000) % 10,
    (clk_get_rate(clk_get_sys(core::ptr::null_mut(), "core_ck")) / 1000000),
    (clk_get_rate(clk_get_sys(core::ptr::null_mut(), "arm_fck")) / 1000000));
    if (soc_type != OMAP3_SOC_OMAP3430_ES1)
    omap3_clk_lock_dpll5();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn omap3430_dt_clk_init() -> int __init {
    int __init omap3430_dt_clk_init(void)
    {
    return omap3xxx_dt_clk_init(OMAP3_SOC_OMAP3430_ES2_PLUS);
    }
#[no_mangle]
pub unsafe extern "C" fn omap3630_dt_clk_init() -> int __init {
    int __init omap3630_dt_clk_init(void)
    {
    return omap3xxx_dt_clk_init(OMAP3_SOC_OMAP3630);
    }
#[no_mangle]
pub unsafe extern "C" fn am35xx_dt_clk_init() -> int __init {
    int __init am35xx_dt_clk_init(void)
    {
    return omap3xxx_dt_clk_init(OMAP3_SOC_AM35XX);
    }
