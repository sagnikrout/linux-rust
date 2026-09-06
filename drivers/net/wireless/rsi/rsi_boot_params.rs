//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/rsi/rsi_boot_params.h
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


//
// Copyright (c) 2014 Redpine Signals Inc.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

pub const TA_PLL_M_VAL_20: c_int = 9;
pub const TA_PLL_N_VAL_20: c_int = 0;
pub const TA_PLL_P_VAL_20: c_int = 4;
pub const PLL960_M_VAL_20: c_uint = 0x14;
pub const PLL960_N_VAL_20: c_int = 0;
pub const PLL960_P_VAL_20: c_int = 5;
pub const UMAC_CLK_40MHZ: c_int = 80;
pub const TA_PLL_M_VAL_40: c_int = 9;
pub const TA_PLL_N_VAL_40: c_int = 0;
pub const TA_PLL_P_VAL_40: c_int = 4;
pub const PLL960_M_VAL_40: c_uint = 0x14;
pub const PLL960_N_VAL_40: c_int = 0;
pub const PLL960_P_VAL_40: c_int = 5;

// structure to store configs related to TAPLL programming
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tapll_info {
    pub pll_reg_1: __le16,
    pub pll_reg_2: __le16,
    pub __packed: },
// structure to store configs related to PLL960 programming
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pll960_info {
    pub pll_reg_1: __le16,
    pub pll_reg_2: __le16,
    pub pll_reg_3: __le16,
    pub __packed: },
// structure to store configs related to AFEPLL programming
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afepll_info {
    pub pll_reg: __le16,
    pub __packed: },
// structure to store configs related to pll configs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pll_config {
    pub tapll_info_g: tapll_info,
    pub pll960_info_g: pll960_info,
    pub afepll_info_g: afepll_info,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pll_config_9116 {
    pub pll_ctrl_set_reg: __le16,
    pub pll_ctrl_clr_reg: __le16,
    pub pll_modem_conig_reg: __le16,
    pub soc_clk_config_reg: __le16,
    pub adc_dac_strm1_config_reg: __le16,
    pub adc_dac_strm2_config_reg: __le16,
    pub __packed: },
// structure to store configs related to UMAC clk programming
#[repr(C)]
#[derive(Copy, Clone)]
pub struct switch_clk {
    pub switch_clk_info: __le16,
// If switch_bbp_lmac_clk_reg is set then this value will be programmed
// into reg
//
    pub bbp_lmac_clk_reg_val: __le16,
// if switch_umac_clk is set then this value will be programmed
    pub umac_clock_reg_config: __le16,
// if switch_qspi_clk is set then this value will be programmed
    pub qspi_uart_clock_reg_config: __le16,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct switch_clk_9116 {
    pub switch_clk_info: __le32,
    pub tass_clock_reg: __le32,
    pub wlan_bbp_lmac_clk_reg_val: __le32,
    pub zbbt_bbp_lmac_clk_reg_val: __le32,
    pub bbp_lmac_clk_en_val: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_clk_info {
    pub pll_config_g: pll_config,
    pub switch_clk_g: switch_clk,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_clk_info_9116 {
    pub pll_config_9116_g: pll_config_9116,
    pub switch_clk_9116_g: switch_clk_9116,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bootup_params {
    pub magic_number: __le16,
    pub crystal_good_time: __le16,
    pub valid: __le32,
    pub reserved_for_valids: __le32,
    pub bootup_mode_info: __le16,
// configuration used for digital loop back
    pub digital_loop_back_params: __le16,
    pub rtls_timestamp_en: __le16,
    pub host_spi_intr_cfg: __le16,
    pub device_clk_info: [device_clk_info; 3],
// ulp buckboost wait time
    pub buckboost_wakeup_cnt: __le32,
// pmu wakeup wait time & WDT EN info
    pub pmu_wakeup_wait: __le16,
    pub shutdown_wait_time: u8,
// Sleep clock source selection
    pub pmu_slp_clkout_sel: u8,
// WDT programming values
    pub wdt_prog_value: __le32,
// WDT soc reset delay
    pub wdt_soc_rst_delay: __le32,
// dcdc modes configs
    pub dcdc_operation_mode: __le32,
    pub soc_reset_wait_cnt: __le32,
    pub waiting_time_at_fresh_sleep: __le32,
    pub max_threshold_to_avoid_sleep: __le32,
    pub beacon_resedue_alg_en: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bootup_params_9116 {
    pub magic_number: __le16,
pub const LOADED_TOKEN: c_uint = 0x5AA5   /* Bootup params are installed by host;
// or OTP/FLASH (Bootloader)
//
pub const ROM_TOKEN: c_uint = 0x55AA   /* Bootup params are taken from ROM;
// itself in MCU mode.
//
    pub crystal_good_time: __le16,
    pub valid: __le32,
    pub reserved_for_valids: __le32,
    pub bootup_mode_info: __le16,

    pub digital_loop_back_params: __le16,
    pub rtls_timestamp_en: __le16,
    pub host_spi_intr_cfg: __le16,
    pub device_clk_info_9116: [device_clk_info_9116; 1],
    pub buckboost_wakeup_cnt: __le32,
    pub pmu_wakeup_wait: __le16,
    pub shutdown_wait_time: u8,
    pub pmu_slp_clkout_sel: u8,
    pub wdt_prog_value: __le32,
    pub wdt_soc_rst_delay: __le32,
    pub dcdc_operation_mode: __le32,
    pub soc_reset_wait_cnt: __le32,
    pub waiting_time_at_fresh_sleep: __le32,
    pub max_threshold_to_avoid_sleep: __le32,
    pub beacon_resedue_alg_en: u8,
    pub __packed: },
