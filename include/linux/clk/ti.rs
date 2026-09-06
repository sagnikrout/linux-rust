//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/clk/ti.h
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
// TI clock drivers support
//
// Copyright (C) 2013 Texas Instruments, Inc.
//

//
// struct clk_omap_reg - OMAP register declaration
// @offset: offset from the master IP module base address
// @bit: register bit offset
// @index: index of the master IP module
// @flags: flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_omap_reg {
    pub ptr: *mut void __iomem,
    pub offset: u16,
    pub bit: u8,
    pub index: u8,
    pub flags: u8,
}

//
// struct dpll_data - DPLL registers and integration data
// @mult_div1_reg: register containing the DPLL M and N bitfields
// @mult_mask: mask of the DPLL M bitfield in @mult_div1_reg
// @div1_mask: mask of the DPLL N bitfield in @mult_div1_reg
// @clk_bypass: struct clk_hw pointer to the clock's bypass clock input
// @clk_ref: struct clk_hw pointer to the clock's reference clock input
// @control_reg: register containing the DPLL mode bitfield
// @enable_mask: mask of the DPLL mode bitfield in @control_reg
// @last_rounded_rate: cache of the last rate result of omap2_dpll_determine_rate()
// @last_rounded_m: cache of the last M result of omap2_dpll_determine_rate()
// @last_rounded_m4xen: cache of the last M4X result of
// omap4_dpll_regm4xen_determine_rate()
// @last_rounded_lpmode: cache of the last lpmode result of
// omap4_dpll_lpmode_recalc()
// @max_multiplier: maximum valid non-bypass multiplier value (actual)
// @last_rounded_n: cache of the last N result of omap2_dpll_determine_rate()
// @min_divider: minimum valid non-bypass divider value (actual)
// @max_divider: maximum valid non-bypass divider value (actual)
// @max_rate: maximum clock rate for the DPLL
// @modes: possible values of @enable_mask
// @autoidle_reg: register containing the DPLL autoidle mode bitfield
// @idlest_reg: register containing the DPLL idle status bitfield
// @autoidle_mask: mask of the DPLL autoidle mode bitfield in @autoidle_reg
// @freqsel_mask: mask of the DPLL jitter correction bitfield in @control_reg
// @dcc_mask: mask of the DPLL DCC correction bitfield @mult_div1_reg
// @dcc_rate: rate atleast which DCC @dcc_mask must be set
// @idlest_mask: mask of the DPLL idle status bitfield in @idlest_reg
// @lpmode_mask: mask of the DPLL low-power mode bitfield in @control_reg
// @m4xen_mask: mask of the DPLL M4X multiplier bitfield in @control_reg
// @auto_recal_bit: bitshift of the driftguard enable bit in @control_reg
// @recal_en_bit: bitshift of the PRM_IRQENABLE_* bit for recalibration IRQs
// @recal_st_bit: bitshift of the PRM_IRQSTATUS_* bit for recalibration IRQs
// @ssc_deltam_reg: register containing the DPLL SSC frequency spreading
// @ssc_modfreq_reg: register containing the DPLL SSC modulation frequency
// @ssc_modfreq_mant_mask: mask of the mantissa component in @ssc_modfreq_reg
// @ssc_modfreq_exp_mask: mask of the exponent component in @ssc_modfreq_reg
// @ssc_enable_mask: mask of the DPLL SSC enable bit in @control_reg
// @ssc_downspread_mask: mask of the DPLL SSC low frequency only bit in
// @control_reg
// @ssc_modfreq: the DPLL SSC frequency modulation in kHz
// @ssc_deltam: the DPLL SSC frequency spreading in permille (10th of percent)
// @ssc_downspread: require the only low frequency spread of the DPLL in SSC
// mode
// @flags: DPLL type/features (see below)
//
// Possible values for @flags:
// DPLL_J_TYPE: "J-type DPLL" (only some 36xx, 4xxx DPLLs)
//
// @freqsel_mask is only used on the OMAP34xx family and AM35xx.
//
// XXX Some DPLLs have multiple bypass inputs, so it's not technically
// correct to only have one @clk_bypass pointer.
//
// XXX The runtime-variable fields (@last_rounded_rate, @last_rounded_m,
// @last_rounded_n) should be separated from the runtime-fixed fields
// and placed into a different structure, so that the runtime-fixed data
// can be placed into read-only space.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpll_data {
    pub mult_div1_reg: clk_omap_reg,
    pub mult_mask: u32,
    pub div1_mask: u32,
    pub clk_bypass: *mut clk_hw,
    pub clk_ref: *mut clk_hw,
    pub control_reg: clk_omap_reg,
    pub enable_mask: u32,
    pub last_rounded_rate: c_ulong,
    pub last_rounded_m: u16,
    pub last_rounded_m4xen: u8,
    pub last_rounded_lpmode: u8,
    pub max_multiplier: u16,
    pub last_rounded_n: u8,
    pub min_divider: u8,
    pub max_divider: u16,
    pub max_rate: c_ulong,
    pub modes: u8,
    pub autoidle_reg: clk_omap_reg,
    pub idlest_reg: clk_omap_reg,
    pub autoidle_mask: u32,
    pub freqsel_mask: u32,
    pub idlest_mask: u32,
    pub dco_mask: u32,
    pub sddiv_mask: u32,
    pub dcc_mask: u32,
    pub dcc_rate: c_ulong,
    pub lpmode_mask: u32,
    pub m4xen_mask: u32,
    pub auto_recal_bit: u8,
    pub recal_en_bit: u8,
    pub recal_st_bit: u8,
    pub ssc_deltam_reg: clk_omap_reg,
    pub ssc_modfreq_reg: clk_omap_reg,
    pub ssc_deltam_int_mask: u32,
    pub ssc_deltam_frac_mask: u32,
    pub ssc_modfreq_mant_mask: u32,
    pub ssc_modfreq_exp_mask: u32,
    pub ssc_enable_mask: u32,
    pub ssc_downspread_mask: u32,
    pub ssc_modfreq: u32,
    pub ssc_deltam: u32,
    pub ssc_downspread: bool,
    pub flags: u8,
}

//
// struct clk_hw_omap_ops - OMAP clk ops
// @find_idlest: find idlest register information for a clock
// @find_companion: find companion clock register information for a clock,
// basically converts CM_ICLKEN* <-> CM_FCLKEN
// @allow_idle: enables autoidle hardware functionality for a clock
// @deny_idle: prevent autoidle hardware functionality for a clock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_hw_omap_ops {
    pub idlest_val): *mut *mut u8 idlest_bit, u8,
    pub other_bit): *mut u8,
    pub oclk): *mut *mut void (allow_idle)(struct clk_hw_omap,
    pub oclk): *mut *mut void (deny_idle)(struct clk_hw_omap,
}

//
// struct clk_hw_omap - OMAP struct clk
// @node: list_head connecting this clock into the full clock list
// @enable_reg: register to write to enable the clock (see @enable_bit)
// @enable_bit: bitshift to write to enable/disable the clock (see @enable_reg)
// @flags: see "struct clk.flags possibilities" above
// @clksel_reg: for clksel clks, register va containing src/divisor select
// @dpll_data: for DPLLs, pointer to struct dpll_data for this clock
// @clkdm_name: clockdomain name that this clock is contained in
// @clkdm: pointer to struct clockdomain, resolved from @clkdm_name at runtime
// @ops: clock ops for this clock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_hw_omap {
    pub hw: clk_hw,
    pub node: list_head,
    pub fixed_rate: c_ulong,
    pub fixed_div: u8,
    pub enable_reg: clk_omap_reg,
    pub enable_bit: u8,
    pub flags: c_ulong,
    pub clksel_reg: clk_omap_reg,
    pub dpll_data: *mut dpll_data,
    pub clkdm_name: *const c_char,
    pub clkdm: *mut clockdomain,
    pub ops: *const clk_hw_omap_ops,
    pub context: u32,
    pub autoidle_count: c_int,
}

//
// struct clk_hw_omap.flags possibilities
//
// XXX document the rest of the clock flags here
//
// ENABLE_REG_32BIT: (OMAP1 only) clock control register must be accessed
// with 32bit ops, by default OMAP1 uses 16bit ops.
// CLOCK_IDLE_CONTROL: (OMAP1 only) clock has autoidle support.
// CLOCK_NO_IDLE_PARENT: (OMAP1 only) when clock is enabled, its parent
// clock is put to no-idle mode.
// ENABLE_ON_INIT: Clock is enabled on init.
// INVERT_ENABLE: By default, clock enable bit behavior is '1' enable, '0'
// disable. This inverts the behavior making '0' enable and '1' disable.
// CLOCK_CLKOUTX2: (OMAP4 only) DPLL CLKOUT and CLKOUTX2 GATE_CTRL
// bits share the same register.  This flag allows the
// omap4_dpllmx*() code to determine which GATE_CTRL bit field
// should be used.  This is a temporary solution - a better approach
// would be to associate clock type-specific data with the clock,
// similar to the struct dpll_data approach.
//

// CM_CLKEN_PLL*.EN* bit values - not all are available for every DPLL
pub const DPLL_LOW_POWER_STOP: c_uint = 0x1;
pub const DPLL_LOW_POWER_BYPASS: c_uint = 0x5;
pub const DPLL_LOCKED: c_uint = 0x7;
// DPLL Type and DCO Selection Flags
pub const DPLL_J_TYPE: c_uint = 0x1;
// Static memmap indices
//
// struct ti_clk_ll_ops - low-level ops for clocks
// @clk_readl: pointer to register read function
// @clk_writel: pointer to register write function
// @clk_rmw: pointer to register read-modify-write function
// @clkdm_clk_enable: pointer to clockdomain enable function
// @clkdm_clk_disable: pointer to clockdomain disable function
// @clkdm_lookup: pointer to clockdomain lookup function
// @cm_wait_module_ready: pointer to CM module wait ready function
// @cm_split_idlest_reg: pointer to CM module function to split idlest reg
//
// Low-level ops are generally used by the basic clock types (clk-gate,
// clk-mux, clk-divider etc.) to provide support for various low-level
// hadrware interfaces (direct MMIO, regmap etc.), and is initialized
// by board code. Low-level ops also contain some other platform specific
// operations not provided directly by clock drivers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_clk_ll_ops {
    pub reg): *const *const u32 (clk_readl)(struct clk_omap_reg,
    pub reg): *const *const void (clk_writel)(u32 val, struct clk_omap_reg,
    pub reg): *const *const void (clk_rmw)(u32 val, u32 mask, struct clk_omap_reg,
    pub clk): *mut *mut *mut int (clkdm_clk_enable)(struct clockdomain clkdm, struct clk,
    pub clk): *mut clk,
    pub name): *const *const *const clockdomain  (clkdm_lookup)(char,
    pub idlest_shift): u8,
    pub idlest_reg_id): *mut *mut s16 prcm_inst, u8,
}

extern "C" {
    pub fn omap2_clk_is_hw_omap(hw: *mut clk_hw) -> bool;
}
extern "C" {
    pub fn omap2_clk_disable_autoidle_all() -> c_int;
}
extern "C" {
    pub fn omap2_clk_enable_autoidle_all() -> c_int;
}
extern "C" {
    pub fn omap2_clk_allow_idle(clk: *mut clk) -> c_int;
}
extern "C" {
    pub fn omap2_clk_deny_idle(clk: *mut clk) -> c_int;
}
extern "C" {
    pub fn omap2xxx_clkt_dpllcore_init(hw: *mut clk_hw);
}
extern "C" {
    pub fn omap2xxx_clkt_vps_init();
}
extern "C" {
    pub fn omap2_get_dpll_rate(clk: *mut clk_hw_omap) -> c_ulong;
}
extern "C" {
    pub fn ti_dt_clk_init_retry_clks();
}
extern "C" {
    pub fn ti_dt_clockdomains_setup();
}
extern "C" {
    pub fn ti_clk_setup_ll_ops(ops: *mut ti_clk_ll_ops) -> c_int;
}
extern "C" {
    pub fn omap2_clk_legacy_provider_init(index: c_int, mem: *mut void __iomem);
}
extern "C" {
    pub fn omap3430_dt_clk_init() -> c_int;
}
extern "C" {
    pub fn omap3630_dt_clk_init() -> c_int;
}
extern "C" {
    pub fn am35xx_dt_clk_init() -> c_int;
}
extern "C" {
    pub fn dm814x_dt_clk_init() -> c_int;
}
extern "C" {
    pub fn dm816x_dt_clk_init() -> c_int;
}
extern "C" {
    pub fn omap4xxx_dt_clk_init() -> c_int;
}
extern "C" {
    pub fn omap5xxx_dt_clk_init() -> c_int;
}
extern "C" {
    pub fn dra7xx_dt_clk_init() -> c_int;
}
extern "C" {
    pub fn am33xx_dt_clk_init() -> c_int;
}
extern "C" {
    pub fn am43xx_dt_clk_init() -> c_int;
}
extern "C" {
    pub fn omap2420_dt_clk_init() -> c_int;
}
extern "C" {
    pub fn omap2430_dt_clk_init() -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_clk_features {
    pub flags: u32,
    pub fint_min: c_long,
    pub fint_max: c_long,
    pub fint_band1_max: c_long,
    pub fint_band2_min: c_long,
    pub dpll_bypass_vals: u8,
    pub cm_idlest_val: u8,
}

extern "C" {
    pub fn ti_clk_setup_features(features: *mut ti_clk_features);
}
extern "C" {
    pub fn ti_clk_is_in_standby(clk: *mut clk) -> bool;
}
extern "C" {
    pub fn omap3_noncore_dpll_save_context(hw: *mut clk_hw) -> c_int;
}
extern "C" {
    pub fn omap3_noncore_dpll_restore_context(hw: *mut clk_hw);
}
extern "C" {
    pub fn omap3_core_dpll_save_context(hw: *mut clk_hw) -> c_int;
}
extern "C" {
    pub fn omap3_core_dpll_restore_context(hw: *mut clk_hw);
}

extern "C" {
    pub fn omap3430_clk_legacy_init() -> c_int;
}
extern "C" {
    pub fn omap3430es1_clk_legacy_init() -> c_int;
}
extern "C" {
    pub fn omap36xx_clk_legacy_init() -> c_int;
}
extern "C" {
    pub fn am35xx_clk_legacy_init() -> c_int;
}

