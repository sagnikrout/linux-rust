//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_ptp_consts.h
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


// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018-2021, Intel Corporation.
// Constant definitions related to the hardware clock used for PTP 1588
// features and functionality.
//
// Constants defined for the PTP 1588 clock hardware.
// Dynamic bitslip 0 equals to 10
// struct ice_time_ref_info_e82x
//
// E82X hardware can use different sources as the reference for the PTP
// hardware clock. Each clock has different characteristics such as a slightly
// different frequency, etc.
//
// This lookup table defines several constants that depend on the current time
// reference. See the struct ice_time_ref_info_e82x for information about the
// meaning of each constant.
//
// ICE_TSPLL_FREQ_25_000 -> 25 MHz
// pll_freq
// nominal_incval
// ICE_TSPLL_FREQ_122_880 -> 122.88 MHz
// pll_freq
// nominal_incval
// ICE_TSPLL_FREQ_125_000 -> 125 MHz
// pll_freq
// nominal_incval
// ICE_TSPLL_FREQ_153_600 -> 153.6 MHz
// pll_freq
// nominal_incval
// ICE_TSPLL_FREQ_156_250 -> 156.25 MHz
// pll_freq
// nominal_incval
// ICE_TSPLL_FREQ_245_760 -> 245.76 MHz
// pll_freq
// nominal_incval
// struct ice_vernier_info_e82x
//
// E822 hardware calibrates the delay of the timestamp indication from the
// actual packet transmission or reception during the initialization of the
// PHY. To do this, the hardware mechanism uses some conversions between the
// various clocks within the PHY block. This table defines constants used to
// calculate the correct conversion ratios in the PHY registers.
//
// Many of the values relate to the PAR/PCS clock conversion registers. For
// these registers, a value of 0 means that the associated register is not
// used by this link speed, and that the register should be cleared by writing
// 0. Other values specify the clock frequency in Hz.
//
// ICE_PTP_LNK_SPD_1G
// tx_par_clk
// rx_par_clk
// tx_pcs_clk
// rx_pcs_clk
// tx_desk_rsgb_par
// rx_desk_rsgb_par
// tx_desk_rsgb_pcs
// rx_desk_rsgb_pcs
// tx_fixed_delay
// pmd_adj_divisor
// rx_fixed_delay
// ICE_PTP_LNK_SPD_10G
// tx_par_clk
// rx_par_clk
// tx_pcs_clk
// rx_pcs_clk
// tx_desk_rsgb_par
// rx_desk_rsgb_par
// tx_desk_rsgb_pcs
// rx_desk_rsgb_pcs
// tx_fixed_delay
// pmd_adj_divisor
// rx_fixed_delay
// ICE_PTP_LNK_SPD_25G
// tx_par_clk
// rx_par_clk
// tx_pcs_clk
// rx_pcs_clk
// tx_desk_rsgb_par
// rx_desk_rsgb_par
// tx_desk_rsgb_pcs
// rx_desk_rsgb_pcs
// tx_fixed_delay
// pmd_adj_divisor
// rx_fixed_delay
// ICE_PTP_LNK_SPD_25G_RS
// tx_par_clk
// rx_par_clk
// tx_pcs_clk
// rx_pcs_clk
// tx_desk_rsgb_par
// rx_desk_rsgb_par
// tx_desk_rsgb_pcs
// rx_desk_rsgb_pcs
// tx_fixed_delay
// pmd_adj_divisor
// rx_fixed_delay
// ICE_PTP_LNK_SPD_40G
// tx_par_clk
// rx_par_clk
// tx_pcs_clk
// rx_pcs_clk
// tx_desk_rsgb_par
// rx_desk_rsgb_par
// tx_desk_rsgb_pcs
// rx_desk_rsgb_pcs
// tx_fixed_delay
// pmd_adj_divisor
// rx_fixed_delay
// ICE_PTP_LNK_SPD_50G
// tx_par_clk
// rx_par_clk
// tx_pcs_clk
// rx_pcs_clk
// tx_desk_rsgb_par
// rx_desk_rsgb_par
// tx_desk_rsgb_pcs
// rx_desk_rsgb_pcs
// tx_fixed_delay
// pmd_adj_divisor
// rx_fixed_delay
// ICE_PTP_LNK_SPD_50G_RS
// tx_par_clk
// rx_par_clk
// tx_pcs_clk
// rx_pcs_clk
// tx_desk_rsgb_par
// rx_desk_rsgb_par
// tx_desk_rsgb_pcs
// rx_desk_rsgb_pcs
// tx_fixed_delay
// pmd_adj_divisor
// rx_fixed_delay
// ICE_PTP_LNK_SPD_100G_RS
// tx_par_clk
// rx_par_clk
// tx_pcs_clk
// rx_pcs_clk
// tx_desk_rsgb_par
// rx_desk_rsgb_par
// tx_desk_rsgb_pcs
// rx_desk_rsgb_pcs
// tx_fixed_delay
// pmd_adj_divisor
// rx_fixed_delay
