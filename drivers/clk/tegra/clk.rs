//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/tegra/clk.h
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

pub const CLK_OUT_ENB_L: c_uint = 0x010;
pub const CLK_OUT_ENB_H: c_uint = 0x014;
pub const CLK_OUT_ENB_U: c_uint = 0x018;
pub const CLK_OUT_ENB_V: c_uint = 0x360;
pub const CLK_OUT_ENB_W: c_uint = 0x364;
pub const CLK_OUT_ENB_X: c_uint = 0x280;
pub const CLK_OUT_ENB_Y: c_uint = 0x298;

pub const CLK_OUT_ENB_SET_L: c_uint = 0x320;
pub const CLK_OUT_ENB_CLR_L: c_uint = 0x324;
pub const CLK_OUT_ENB_SET_H: c_uint = 0x328;
pub const CLK_OUT_ENB_CLR_H: c_uint = 0x32c;
pub const CLK_OUT_ENB_SET_U: c_uint = 0x330;
pub const CLK_OUT_ENB_CLR_U: c_uint = 0x334;
pub const CLK_OUT_ENB_SET_V: c_uint = 0x440;
pub const CLK_OUT_ENB_CLR_V: c_uint = 0x444;
pub const CLK_OUT_ENB_SET_W: c_uint = 0x448;
pub const CLK_OUT_ENB_CLR_W: c_uint = 0x44c;
pub const CLK_OUT_ENB_SET_X: c_uint = 0x284;
pub const CLK_OUT_ENB_CLR_X: c_uint = 0x288;
pub const CLK_OUT_ENB_SET_Y: c_uint = 0x29c;
pub const CLK_OUT_ENB_CLR_Y: c_uint = 0x2a0;
pub const RST_DEVICES_L: c_uint = 0x004;
pub const RST_DEVICES_H: c_uint = 0x008;
pub const RST_DEVICES_U: c_uint = 0x00C;
pub const RST_DEVICES_V: c_uint = 0x358;
pub const RST_DEVICES_W: c_uint = 0x35C;
pub const RST_DEVICES_X: c_uint = 0x28C;
pub const RST_DEVICES_Y: c_uint = 0x2a4;
pub const RST_DEVICES_SET_L: c_uint = 0x300;
pub const RST_DEVICES_CLR_L: c_uint = 0x304;
pub const RST_DEVICES_SET_H: c_uint = 0x308;
pub const RST_DEVICES_CLR_H: c_uint = 0x30c;
pub const RST_DEVICES_SET_U: c_uint = 0x310;
pub const RST_DEVICES_CLR_U: c_uint = 0x314;
pub const RST_DEVICES_SET_V: c_uint = 0x430;
pub const RST_DEVICES_CLR_V: c_uint = 0x434;
pub const RST_DEVICES_SET_W: c_uint = 0x438;
pub const RST_DEVICES_CLR_W: c_uint = 0x43c;
pub const RST_DEVICES_SET_X: c_uint = 0x290;
pub const RST_DEVICES_CLR_X: c_uint = 0x294;
pub const RST_DEVICES_SET_Y: c_uint = 0x2a8;
pub const RST_DEVICES_CLR_Y: c_uint = 0x2ac;
//
// Tegra CLK_OUT_ENB registers have some undefined bits which are not used and
// any accidental write of 1 to these bits can cause PSLVERR.
// So below are the valid mask defines for each CLK_OUT_ENB register used to
// turn ON only the valid clocks.
//
pub const TEGRA210_CLK_ENB_VLD_MSK_L: c_uint = 0xdcd7dff9;
pub const TEGRA210_CLK_ENB_VLD_MSK_H: c_uint = 0x87d1f3e7;
pub const TEGRA210_CLK_ENB_VLD_MSK_U: c_uint = 0xf3fed3fa;
pub const TEGRA210_CLK_ENB_VLD_MSK_V: c_uint = 0xffc18cfb;
pub const TEGRA210_CLK_ENB_VLD_MSK_W: c_uint = 0x793fb7ff;
pub const TEGRA210_CLK_ENB_VLD_MSK_X: c_uint = 0x3fe66fff;
pub const TEGRA210_CLK_ENB_VLD_MSK_Y: c_uint = 0xfc1fc7ff;
//
// struct tegra_clk_sync_source - external clock source from codec
//
// @hw: handle between common and hardware-specific interfaces
// @rate: input frequency from source
// @max_rate: max rate allowed
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_clk_sync_source {
    pub hw: clk_hw,
    pub rate: c_ulong,
    pub max_rate: c_ulong,
}

//
// struct tegra_clk_frac_div - fractional divider clock
//
// @hw:		handle between common and hardware-specific interfaces
// @reg:	register containing divider
// @flags:	hardware-specific flags
// @shift:	shift to the divider bit field
// @width:	width of the divider bit field
// @frac_width:	width of the fractional bit field
// @lock:	register lock
//
// Flags:
// TEGRA_DIVIDER_ROUND_UP - This flags indicates to round up the divider value.
// TEGRA_DIVIDER_FIXED - Fixed rate PLL dividers has addition override bit, this
// flag indicates that this divider is for fixed rate PLL.
// TEGRA_DIVIDER_INT - Some modules can not cope with the duty cycle when
// fraction bit is set. This flags indicates to calculate divider for which
// fracton bit will be zero.
// TEGRA_DIVIDER_UART - UART module divider has additional enable bit which is
// set when divider value is not 0. This flags indicates that the divider
// is for UART module.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_clk_frac_div {
    pub hw: clk_hw,
    pub reg: *mut void __iomem,
    pub flags: u8,
    pub shift: u8,
    pub width: u8,
    pub frac_width: u8,
    pub lock: *mut spinlock_t,
}

//
// Tegra PLL:
//
// In general, there are 3 requirements for each PLL
// that SW needs to be comply with.
// (1) Input frequency range (REF).
// (2) Comparison frequency range (CF). CF = REF/DIVM.
// (3) VCO frequency range (VCO).  VCO = CF * DIVN.
//
// The final PLL output frequency (FO) = VCO >> DIVP.
//
// struct tegra_clk_pll_freq_table - PLL frequecy table
//
// @input_rate:		input rate from source
// @output_rate:	output rate from PLL for the input rate
// @n:			feedback divider
// @m:			input divider
// @p:			post divider
// @cpcon:		charge pump current
// @sdm_data:		fraction divider setting (0 = disabled)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_clk_pll_freq_table {
    pub input_rate: c_ulong,
    pub output_rate: c_ulong,
    pub n: u32,
    pub m: u32,
    pub p: u8,
    pub cpcon: u8,
    pub sdm_data: u16,
}

//
// struct pdiv_map - map post divider to hw value
//
// @pdiv:		post divider
// @hw_val:		value to be written to the PLL hw
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pdiv_map {
    pub pdiv: u8,
    pub hw_val: u8,
}

//
// struct div_nmp - offset and width of m,n and p fields
//
// @divn_shift:	shift to the feedback divider bit field
// @divn_width:	width of the feedback divider bit field
// @divm_shift:	shift to the input divider bit field
// @divm_width:	width of the input divider bit field
// @divp_shift:	shift to the post divider bit field
// @divp_width:	width of the post divider bit field
// @override_divn_shift: shift to the feedback divider bitfield in override reg
// @override_divm_shift: shift to the input divider bitfield in override reg
// @override_divp_shift: shift to the post divider bitfield in override reg
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct div_nmp {
    pub divn_shift: u8,
    pub divn_width: u8,
    pub divm_shift: u8,
    pub divm_width: u8,
    pub divp_shift: u8,
    pub divp_width: u8,
    pub override_divn_shift: u8,
    pub override_divm_shift: u8,
    pub override_divp_shift: u8,
}

pub const MAX_PLL_MISC_REG_COUNT: c_int = 6;
//
// struct tegra_clk_pll_params - PLL parameters
//
// @input_min:			Minimum input frequency
// @input_max:			Maximum input frequency
// @cf_min:			Minimum comparison frequency
// @cf_max:			Maximum comparison frequency
// @vco_min:			Minimum VCO frequency
// @vco_max:			Maximum VCO frequency
// @base_reg:			PLL base reg offset
// @misc_reg:			PLL misc reg offset
// @lock_reg:			PLL lock reg offset
// @lock_mask:			Bitmask for PLL lock status
// @lock_enable_bit_idx:	Bit index to enable PLL lock
// @iddq_reg:			PLL IDDQ register offset
// @iddq_bit_idx:		Bit index to enable PLL IDDQ
// @reset_reg:			Register offset of where RESET bit is
// @reset_bit_idx:		Shift of reset bit in reset_reg
// @sdm_din_reg:		Register offset where SDM settings are
// @sdm_din_mask:		Mask of SDM divider bits
// @sdm_ctrl_reg:		Register offset where SDM enable is
// @sdm_ctrl_en_mask:		Mask of SDM enable bit
// @ssc_ctrl_reg:		Register offset where SSC settings are
// @ssc_ctrl_en_mask:		Mask of SSC enable bit
// @aux_reg:			AUX register offset
// @dyn_ramp_reg:		Dynamic ramp control register offset
// @ext_misc_reg:		Miscellaneous control register offsets
// @pmc_divnm_reg:		n, m divider PMC override register offset (PLLM)
// @pmc_divp_reg:		p divider PMC override register offset (PLLM)
// @flags:			PLL flags
// @stepa_shift:		Dynamic ramp step A field shift
// @stepb_shift:		Dynamic ramp step B field shift
// @lock_delay:			Delay in us if PLL lock is not used
// @max_p:			maximum value for the p divider
// @defaults_set:		Boolean signaling all reg defaults for PLL set.
// @pdiv_tohw:			mapping of p divider to register values
// @div_nmp:			offsets and widths on n, m and p fields
// @freq_table:			array of frequencies supported by PLL
// @fixed_rate:			PLL rate if it is fixed
// @mdiv_default:		Default value for fixed mdiv for this PLL
// @round_p_to_pdiv:		Callback used to round p to the closed pdiv
// @set_gain:			Callback to adjust N div for SDM enabled
// PLL's based on fractional divider value.
// @calc_rate:			Callback used to change how out of table
// rates (dividers and multipler) are calculated.
// @adjust_vco:			Callback to adjust the programming range of the
// divider range (if SDM is present)
// @set_defaults:		Callback which will try to initialize PLL
// registers to sane default values. This is first
// tried during PLL registration, but if the PLL
// is already enabled, it will be done the first
// time the rate is changed while the PLL is
// disabled.
// @dyn_ramp:			Callback which can be used to define a custom
// dynamic ramp function for a given PLL.
// @pre_rate_change:		Callback which is invoked just before changing
// PLL's rate.
// @post_rate_change:		Callback which is invoked right after changing
// PLL's rate.
//
// Flags:
// TEGRA_PLL_USE_LOCK - This flag indicated to use lock bits for
// PLL locking. If not set it will use lock_delay value to wait.
// TEGRA_PLL_HAS_CPCON - This flag indicates that CPCON value needs
// to be programmed to change output frequency of the PLL.
// TEGRA_PLL_SET_LFCON - This flag indicates that LFCON value needs
// to be programmed to change output frequency of the PLL.
// TEGRA_PLL_SET_DCCON - This flag indicates that DCCON value needs
// to be programmed to change output frequency of the PLL.
// TEGRA_PLLU - PLLU has inverted post divider. This flags indicated
// that it is PLLU and invert post divider value.
// TEGRA_PLLM - PLLM has additional override settings in PMC. This
// flag indicates that it is PLLM and use override settings.
// TEGRA_PLL_FIXED - We are not supposed to change output frequency
// of some plls.
// TEGRA_PLLE_CONFIGURE - Configure PLLE when enabling.
// TEGRA_PLL_LOCK_MISC - Lock bit is in the misc register instead of the
// base register.
// TEGRA_PLL_BYPASS - PLL has bypass bit
// TEGRA_PLL_HAS_LOCK_ENABLE - PLL has bit to enable lock monitoring
// TEGRA_MDIV_NEW - Switch to new method for calculating fixed mdiv
// it may be more accurate (especially if SDM present)
// TEGRA_PLLMB - PLLMB has should be treated similar to PLLM. This
// flag indicated that it is PLLMB.
// TEGRA_PLL_VCO_OUT - Used to indicate that the PLL has a VCO output
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_clk_pll_params {
    pub input_min: c_ulong,
    pub input_max: c_ulong,
    pub cf_min: c_ulong,
    pub cf_max: c_ulong,
    pub vco_min: c_ulong,
    pub vco_max: c_ulong,
    pub base_reg: u32,
    pub misc_reg: u32,
    pub lock_reg: u32,
    pub lock_mask: u32,
    pub lock_enable_bit_idx: u32,
    pub iddq_reg: u32,
    pub iddq_bit_idx: u32,
    pub reset_reg: u32,
    pub reset_bit_idx: u32,
    pub sdm_din_reg: u32,
    pub sdm_din_mask: u32,
    pub sdm_ctrl_reg: u32,
    pub sdm_ctrl_en_mask: u32,
    pub ssc_ctrl_reg: u32,
    pub ssc_ctrl_en_mask: u32,
    pub aux_reg: u32,
    pub dyn_ramp_reg: u32,
    pub ext_misc_reg: [u32; MAX_PLL_MISC_REG_COUNT],
    pub pmc_divnm_reg: u32,
    pub pmc_divp_reg: u32,
    pub flags: u32,
    pub stepa_shift: c_int,
    pub stepb_shift: c_int,
    pub lock_delay: c_int,
    pub max_p: c_int,
    pub defaults_set: bool,
    pub pdiv_tohw: *const pdiv_map,
    pub div_nmp: *mut div_nmp,
    pub freq_table: *mut tegra_clk_pll_freq_table,
    pub fixed_rate: c_ulong,
    pub mdiv_default: u16,
    pub pdiv): *mut *mut u32 (round_p_to_pdiv)(u32 p, u32,
    pub cfg): *mut *mut void (set_gain)(struct tegra_clk_pll_freq_table,
    pub parent_rate): unsigned long rate, unsigned long,
    pub parent_rate): c_ulong,
    pub pll): *mut *mut void (set_defaults)(struct tegra_clk_pll,
    pub cfg): *mut tegra_clk_pll_freq_table,
    pub (*pre_rate_change)(void): *mut c_int,
    pub (*post_rate_change)(void): *mut c_void,
}

//
// struct tegra_clk_pll - Tegra PLL clock
//
// @hw:		handle between common and hardware-specifix interfaces
// @clk_base:	address of CAR controller
// @pmc:	address of PMC, required to read override bits
// @lock:	register lock
// @params:	PLL parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_clk_pll {
    pub hw: clk_hw,
    pub clk_base: *mut void __iomem,
    pub pmc: *mut void __iomem,
    pub lock: *mut spinlock_t,
    pub params: *mut tegra_clk_pll_params,
}

//
// struct tegra_audio_clk_info - Tegra Audio Clk Information
//
// @name:	name for the audio pll
// @pll_params:	pll_params for audio pll
// @clk_id:	clk_ids for the audio pll
// @parent:	name of the parent of the audio pll
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_audio_clk_info {
    pub name: *mut c_char,
    pub pll_params: *mut tegra_clk_pll_params,
    pub clk_id: c_int,
    pub parent: *mut c_char,
}

//
// struct tegra_clk_pll_out - PLL divider down clock
//
// @hw:			handle between common and hardware-specific interfaces
// @reg:		register containing the PLL divider
// @enb_bit_idx:	bit to enable/disable PLL divider
// @rst_bit_idx:	bit to reset PLL divider
// @lock:		register lock
// @flags:		hardware-specific flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_clk_pll_out {
    pub hw: clk_hw,
    pub reg: *mut void __iomem,
    pub enb_bit_idx: u8,
    pub rst_bit_idx: u8,
    pub lock: *mut spinlock_t,
    pub flags: u8,
}

//
// struct tegra_clk_periph_regs -  Registers controlling peripheral clock
//
// @enb_reg:		read the enable status
// @enb_set_reg:	write 1 to enable clock
// @enb_clr_reg:	write 1 to disable clock
// @rst_reg:		read the reset status
// @rst_set_reg:	write 1 to assert the reset of peripheral
// @rst_clr_reg:	write 1 to deassert the reset of peripheral
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_clk_periph_regs {
    pub enb_reg: u32,
    pub enb_set_reg: u32,
    pub enb_clr_reg: u32,
    pub rst_reg: u32,
    pub rst_set_reg: u32,
    pub rst_clr_reg: u32,
}

//
// struct tegra_clk_periph_gate - peripheral gate clock
//
// @magic:		magic number to validate type
// @hw:			handle between common and hardware-specific interfaces
// @clk_base:		address of CAR controller
// @regs:		Registers to control the peripheral
// @flags:		hardware-specific flags
// @clk_num:		Clock number
// @enable_refcnt:	array to maintain reference count of the clock
//
// Flags:
// TEGRA_PERIPH_NO_RESET - This flag indicates that reset is not allowed
// for this module.
// TEGRA_PERIPH_ON_APB - If peripheral is in the APB bus then read the
// bus to flush the write operation in apb bus. This flag indicates
// that this peripheral is in apb bus.
// TEGRA_PERIPH_WAR_1005168 - Apply workaround for Tegra114 MSENC bug
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_clk_periph_gate {
    pub magic: u32,
    pub hw: clk_hw,
    pub clk_base: *mut void __iomem,
    pub flags: u8,
    pub clk_num: c_int,
    pub enable_refcnt: *mut c_int,
    pub regs: *const tegra_clk_periph_regs,
}

pub const TEGRA_CLK_PERIPH_GATE_MAGIC: c_uint = 0x17760309;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_clk_periph_fixed {
    pub hw: clk_hw,
    pub base: *mut void __iomem,
    pub regs: *const tegra_clk_periph_regs,
    pub mul: c_uint,
    pub div: c_uint,
    pub num: c_uint,
}

//
// struct clk-periph - peripheral clock
//
// @magic:	magic number to validate type
// @hw:		handle between common and hardware-specific interfaces
// @mux:	mux clock
// @divider:	divider clock
// @gate:	gate clock
// @mux_ops:	mux clock ops
// @div_ops:	divider clock ops
// @gate_ops:	gate clock ops
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_clk_periph {
    pub magic: u32,
    pub hw: clk_hw,
    pub mux: clk_mux,
    pub divider: tegra_clk_frac_div,
    pub gate: tegra_clk_periph_gate,
    pub mux_ops: *const clk_ops,
    pub div_ops: *const clk_ops,
    pub gate_ops: *const clk_ops,
}

pub const TEGRA_CLK_PERIPH_MAGIC: c_uint = 0x18221223;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_periph_init_data {
    pub name: *const c_char,
    pub clk_id: c_int,
    pub parent_names: *const *const c_char,
    pub parent_name: *const c_char,
    pub p: },
    pub num_parents: c_int,
    pub periph: tegra_clk_periph,
    pub offset: u32,
    pub con_id: *const c_char,
    pub dev_id: *const c_char,
    pub flags: c_ulong,
}

//
// struct clk_super_mux - super clock
//
// @hw:		handle between common and hardware-specific interfaces
// @reg:	register controlling multiplexer
// @width:	width of the multiplexer bit field
// @flags:	hardware-specific flags
// @div2_index:	bit controlling divide-by-2
// @pllx_index:	PLLX index in the parent list
// @lock:	register lock
//
// Flags:
// TEGRA_DIVIDER_2 - LP cluster has additional divider. This flag indicates
// that this is LP cluster clock.
// TEGRA210_CPU_CLK - This flag is used to identify CPU cluster for gen5
// super mux parent using PLLP branches. To use PLLP branches to CPU, need
// to configure additional bit PLLP_OUT_CPU in the clock registers.
// TEGRA20_SUPER_CLK - Tegra20 doesn't have a dedicated divider for Super
// clocks, it only has a clock-skipper.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_clk_super_mux {
    pub hw: clk_hw,
    pub reg: *mut void __iomem,
    pub frac_div: tegra_clk_frac_div,
    pub div_ops: *const clk_ops,
    pub width: u8,
    pub flags: u8,
    pub div2_index: u8,
    pub pllx_index: u8,
    pub lock: *mut spinlock_t,
}

extern "C" {
    pub fn tegra_cclk_pre_pllx_rate_change() -> c_int;
}
extern "C" {
    pub fn tegra_cclk_post_pllx_rate_change();
}
//
// struct tegra_sdmmc_mux - switch divider with Low Jitter inputs for SDMMC
//
// @hw:		handle between common and hardware-specific interfaces
// @reg:	register controlling mux and divider
// @flags:	hardware-specific flags
// @lock:	optional register lock
// @gate:	gate clock
// @gate_ops:	gate clock ops
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_sdmmc_mux {
    pub hw: clk_hw,
    pub reg: *mut void __iomem,
    pub lock: *mut spinlock_t,
    pub gate_ops: *const clk_ops,
    pub gate: tegra_clk_periph_gate,
    pub div_flags: u8,
}

//
// struct clk_init_table - clock initialization table
// @clk_id:	clock id as mentioned in device tree bindings
// @parent_id:	parent clock id as mentioned in device tree bindings
// @rate:	rate to set
// @state:	enable/disable
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_clk_init_table {
    pub clk_id: c_uint,
    pub parent_id: c_uint,
    pub rate: c_ulong,
    pub state: c_int,
}

//
// struct clk_duplicate - duplicate clocks
// @clk_id:	clock id as mentioned in device tree bindings
// @lookup:	duplicate lookup entry for the clock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_clk_duplicate {
    pub clk_id: c_int,
    pub lookup: clk_lookup,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_clk {
    pub dt_id: c_int,
    pub present: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_devclk {
    pub dt_id: c_int,
    pub dev_id: *mut c_char,
    pub con_id: *mut c_char,
}

extern "C" {
    pub fn tegra_add_of_provider(np: *mut device_node, clk_src_onecell_get: *mut c_void);
}
extern "C" {
    pub fn tegra_register_devclks(dev_clks: *mut tegra_devclk, num: c_int);
}
extern "C" {
    pub fn tegra_fixed_clk_init(tegra_clks: *mut tegra_clk);
}

extern "C" {
    pub fn tegra124_clk_emc_driver_available(emc_hw: *mut clk_hw) -> bool;
}

extern "C" {
    pub fn tegra114_clock_tune_cpu_trimmers_high();
}
extern "C" {
    pub fn tegra114_clock_tune_cpu_trimmers_low();
}
extern "C" {
    pub fn tegra114_clock_tune_cpu_trimmers_init();
}
extern "C" {
    pub fn void(_arg: *mut tegra_clk_apply_init_table_func)(void) -> typedef;
}
extern "C" {
    pub fn tegra_pll_wait_for_lock(pll: *mut tegra_clk_pll) -> c_int;
}
extern "C" {
    pub fn tegra_pll_get_fixed_mdiv(hw: *mut clk_hw, input_rate: c_ulong) -> u16;
}
extern "C" {
    pub fn tegra_pll_p_div_to_hw(pll: *mut tegra_clk_pll, p_div: u8) -> c_int;
}
extern "C" {
    pub fn tegra_clk_osc_resume(clk_base: *mut void __iomem);
}
extern "C" {
    pub fn tegra_clk_set_pllp_out_cpu(enable: bool);
}
extern "C" {
    pub fn tegra_clk_periph_suspend();
}
extern "C" {
    pub fn tegra_clk_periph_resume();
}
// Combined read fence with delay

extern "C" {
    pub fn tegra20_clk_emc_driver_available(emc_hw: *mut clk_hw) -> bool;
}
