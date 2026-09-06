//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_ptp_hw.h
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
// Copyright (C) 2021, Intel Corporation.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_ptp_tmr_cmd {
    ICE_PTP_INIT_TIME,
    ICE_PTP_INIT_INCVAL,
    ICE_PTP_ADJ_TIME,
    ICE_PTP_ADJ_TIME_AT_TIME,
    ICE_PTP_READ_TIME,
    ICE_PTP_NOP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_ptp_serdes {
    ICE_PTP_SERDES_1G,
    ICE_PTP_SERDES_10G,
    ICE_PTP_SERDES_25G,
    ICE_PTP_SERDES_40G,
    ICE_PTP_SERDES_50G,
    ICE_PTP_SERDES_100G
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_ptp_link_spd {
    ICE_PTP_LNK_SPD_1G,
    ICE_PTP_LNK_SPD_10G,
    ICE_PTP_LNK_SPD_25G,
    ICE_PTP_LNK_SPD_25G_RS,
    ICE_PTP_LNK_SPD_40G,
    ICE_PTP_LNK_SPD_50G,
    ICE_PTP_LNK_SPD_50G_RS,
    ICE_PTP_LNK_SPD_100G_RS,
    NUM_ICE_PTP_LNK_SPD /* Must be last */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_ptp_fec_mode {
    ICE_PTP_FEC_MODE_NONE,
    ICE_PTP_FEC_MODE_CLAUSE74,
    ICE_PTP_FEC_MODE_RS_FEC
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eth56g_res_type {
    ETH56G_PHY_REG_PTP,
    ETH56G_PHY_MEM_PTP,
    ETH56G_PHY_REG_XPCS,
    ETH56G_PHY_REG_MAC,
    ETH56G_PHY_REG_GPCS,
    NUM_ETH56G_PHY_RES
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_eth56g_link_spd {
    ICE_ETH56G_LNK_SPD_1G,
    ICE_ETH56G_LNK_SPD_2_5G,
    ICE_ETH56G_LNK_SPD_10G,
    ICE_ETH56G_LNK_SPD_25G,
    ICE_ETH56G_LNK_SPD_40G,
    ICE_ETH56G_LNK_SPD_50G,
    ICE_ETH56G_LNK_SPD_50G2,
    ICE_ETH56G_LNK_SPD_100G,
    ICE_ETH56G_LNK_SPD_100G2,
    NUM_ICE_ETH56G_LNK_SPD /* Must be last */
}

//
// struct ice_phy_reg_info_eth56g - ETH56G PHY register parameters
// @base_addr: base address for each PHY block
// @step: step between PHY lanes
//
// Characteristic information for the various PHY register parameters in the
// ETH56G devices
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_phy_reg_info_eth56g {
    pub base_addr: u32,
    pub step: u32,
}

//
// struct ice_time_ref_info_e82x
// @pll_freq: Frequency of PLL that drives timer ticks in Hz
// @nominal_incval: increment to generate nanoseconds in GLTSYN_TIME_L
//
// Characteristic information for the various TIME_REF sources possible in the
// E822 devices
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_time_ref_info_e82x {
    pub pll_freq: u64,
    pub nominal_incval: u64,
}

//
// struct ice_vernier_info_e82x
// @tx_par_clk: Frequency used to calculate P_REG_PAR_TX_TUS
// @rx_par_clk: Frequency used to calculate P_REG_PAR_RX_TUS
// @tx_pcs_clk: Frequency used to calculate P_REG_PCS_TX_TUS
// @rx_pcs_clk: Frequency used to calculate P_REG_PCS_RX_TUS
// @tx_desk_rsgb_par: Frequency used to calculate P_REG_DESK_PAR_TX_TUS
// @rx_desk_rsgb_par: Frequency used to calculate P_REG_DESK_PAR_RX_TUS
// @tx_desk_rsgb_pcs: Frequency used to calculate P_REG_DESK_PCS_TX_TUS
// @rx_desk_rsgb_pcs: Frequency used to calculate P_REG_DESK_PCS_RX_TUS
// @tx_fixed_delay: Fixed Tx latency measured in 1/100th nanoseconds
// @pmd_adj_divisor: Divisor used to calculate PDM alignment adjustment
// @rx_fixed_delay: Fixed Rx latency measured in 1/100th nanoseconds
//
// Table of constants used during as part of the Vernier calibration of the Tx
// and Rx timestamps. This includes frequency values used to compute TUs per
// PAR/PCS clock cycle, and static delay values measured during hardware
// design.
//
// Note that some values are not used for all link speeds, and the
// P_REG_DESK_PAR* registers may represent different clock markers at
// different link speeds, either the deskew marker for multi-lane link speeds
// or the Reed Solomon gearbox marker for RS-FEC.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_vernier_info_e82x {
    pub tx_par_clk: u32,
    pub rx_par_clk: u32,
    pub tx_pcs_clk: u32,
    pub rx_pcs_clk: u32,
    pub tx_desk_rsgb_par: u32,
    pub rx_desk_rsgb_par: u32,
    pub tx_desk_rsgb_pcs: u32,
    pub rx_desk_rsgb_pcs: u32,
    pub tx_fixed_delay: u32,
    pub pmd_adj_divisor: u32,
    pub rx_fixed_delay: u32,
}

pub const ICE_ETH56G_MAC_CFG_FRAC_W: c_int = 9;
//
// struct ice_eth56g_mac_reg_cfg - MAC config values for specific PTP registers
// @tx_mode: Tx timestamp compensation mode
// @tx_mk_dly: Tx timestamp marker start strobe delay
// @tx_cw_dly: Tx timestamp codeword start strobe delay
// @rx_mode: Rx timestamp compensation mode
// @rx_mk_dly: Rx timestamp marker start strobe delay
// @rx_cw_dly: Rx timestamp codeword start strobe delay
// @blks_per_clk: number of blocks transferred per clock cycle
// @blktime: block time, fixed point
// @mktime: marker time, fixed point
// @tx_offset: total Tx offset, fixed point
// @rx_offset: total Rx offset, contains value for bitslip/deskew, fixed point
//
// All fixed point registers except Rx offset are 23 bit unsigned ints with
// a 9 bit fractional.
// Rx offset is 11 bit unsigned int with a 9 bit fractional.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_eth56g_mac_reg_cfg {
    pub def: u8,
    pub rs: u8,
    pub tx_mode: },
    pub tx_mk_dly: u8,
    pub def: u8,
    pub onestep: u8,
    pub tx_cw_dly: },
    pub def: u8,
    pub rs: u8,
    pub rx_mode: },
    pub def: u8,
    pub rs: u8,
    pub rx_mk_dly: },
    pub def: u8,
    pub rs: u8,
    pub rx_cw_dly: },
    pub blks_per_clk: u8,
    pub blktime: u16,
    pub mktime: u16,
    pub serdes: u32,
    pub no_fec: u32,
    pub fc: u32,
    pub rs: u32,
    pub sfd: u32,
    pub onestep: u32,
    pub tx_offset: },
    pub serdes: u32,
    pub no_fec: u32,
    pub fc: u32,
    pub rs: u32,
    pub sfd: u32,
    pub bs_ds: u32,
    pub rx_offset: },
}

pub const E810C_QSFP_C827_0_HANDLE: c_int = 2;
pub const E810C_QSFP_C827_1_HANDLE: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_e810_c827_idx {
    C827_0,
    C827_1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_phy_rclk_pins {
    ICE_RCLKA_PIN = 0,		/* SCL pin */
    ICE_RCLKB_PIN,			/* SDA pin */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_zl_cgu_in_pins {
    ZL_REF0P = 0,
    ZL_REF0N,
    ZL_REF1P,
    ZL_REF1N,
    ZL_REF2P,
    ZL_REF2N,
    ZL_REF3P,
    ZL_REF3N,
    ZL_REF4P,
    ZL_REF4N,
    NUM_ZL_CGU_INPUT_PINS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_zl_cgu_out_pins {
    ZL_OUT0 = 0,
    ZL_OUT1,
    ZL_OUT2,
    ZL_OUT3,
    ZL_OUT4,
    ZL_OUT5,
    ZL_OUT6,
    NUM_ZL_CGU_OUTPUT_PINS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_si_cgu_in_pins {
    SI_REF0P = 0,
    SI_REF0N,
    SI_REF1P,
    SI_REF1N,
    SI_REF2P,
    SI_REF2N,
    SI_REF3,
    SI_REF4,
    NUM_SI_CGU_INPUT_PINS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_si_cgu_out_pins {
    SI_OUT0 = 0,
    SI_OUT1,
    SI_OUT2,
    SI_OUT3,
    SI_OUT4,
    NUM_SI_CGU_OUTPUT_PINS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_cgu_pin_desc {
    pub name: *const c_char,
    pub index: u8,
    pub type: dpll_pin_type,
    pub freq_supp_num: u32,
    pub freq_supp: *mut dpll_pin_frequency,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_e825c_ref_clk {
    ICE_REF_CLK_ENET,
    ICE_REF_CLK_SYNCE,
    ICE_REF_CLK_EREF0,
    ICE_REF_CLK_MAX,
}

pub const E810C_QSFP_C827_0_HANDLE: c_int = 2;
pub const E810C_QSFP_C827_1_HANDLE: c_int = 3;
// Table of constants related to possible ETH56G PHY resources
// Table of constants related to possible TIME_REF sources
// Table of constants for Vernier calibration on E822
// Increment value to generate nanoseconds in the GLTSYN_TIME_L register for
// the E810 devices. Based off of a PLL with an 812.5 MHz frequency.
//
pub const ICE_E810_PLL_FREQ: c_int = 812500000;
pub const ICE_PTP_NOMINAL_INCVAL_E810: c_uint = 0x13b13b13bULL;
pub const ICE_E810_E830_SYNC_DELAY: c_int = 0;
// Device agnostic functions
extern "C" {
    pub fn ice_get_ptp_src_clock_index(hw: *mut ice_hw) -> u8;
}
extern "C" {
    pub fn ice_ptp_lock(hw: *mut ice_hw) -> bool;
}
extern "C" {
    pub fn ice_ptp_unlock(hw: *mut ice_hw);
}
extern "C" {
    pub fn ice_ptp_src_cmd(hw: *mut ice_hw, cmd: ice_ptp_tmr_cmd);
}
extern "C" {
    pub fn ice_ptp_init_time(hw: *mut ice_hw, time: u64) -> c_int;
}
extern "C" {
    pub fn ice_ptp_write_incval(hw: *mut ice_hw, incval: u64) -> c_int;
}
extern "C" {
    pub fn ice_ptp_write_incval_locked(hw: *mut ice_hw, incval: u64) -> c_int;
}
extern "C" {
    pub fn ice_ptp_adj_clock(hw: *mut ice_hw, adj: i32) -> c_int;
}
extern "C" {
    pub fn ice_ptp_clear_phy_offset_ready_e82x(hw: *mut ice_hw) -> c_int;
}
extern "C" {
    pub fn ice_read_phy_tstamp(hw: *mut ice_hw, block: u8, idx: u8, tstamp: *mut u64) -> c_int;
}
extern "C" {
    pub fn ice_clear_phy_tstamp(hw: *mut ice_hw, block: u8, idx: u8) -> c_int;
}
extern "C" {
    pub fn ice_ptp_reset_ts_memory(hw: *mut ice_hw);
}
extern "C" {
    pub fn ice_ptp_init_phc(hw: *mut ice_hw) -> c_int;
}
extern "C" {
    pub fn ice_ptp_init_hw(hw: *mut ice_hw);
}
extern "C" {
    pub fn ice_get_phy_tx_tstamp_ready(hw: *mut ice_hw, block: u8, tstamp_ready: *mut u64) -> c_int;
}
extern "C" {
    pub fn ice_check_phy_tx_tstamp_ready(hw: *mut ice_hw) -> c_int;
}
// E822 family functions
extern "C" {
    pub fn ice_read_quad_reg_e82x(hw: *mut ice_hw, quad: u8, offset: u16, val: *mut u32) -> c_int;
}
extern "C" {
    pub fn ice_write_quad_reg_e82x(hw: *mut ice_hw, quad: u8, offset: u16, val: u32) -> c_int;
}
extern "C" {
    pub fn ice_ptp_reset_ts_memory_quad_e82x(hw: *mut ice_hw, quad: u8);
}
//
// ice_e82x_time_ref - Get the current TIME_REF from capabilities
// @hw: pointer to the HW structure
//
// Returns the current TIME_REF from the capabilities structure.
//
// ice_set_e82x_time_ref - Set new TIME_REF
// @hw: pointer to the HW structure
// @time_ref: new TIME_REF to set
//
// Update the TIME_REF in the capabilities structure in response to some
// change, such as an update to the CGU registers.
//
// E822 Vernier calibration functions
extern "C" {
    pub fn ice_stop_phy_timer_e82x(hw: *mut ice_hw, port: u8, soft_reset: bool) -> c_int;
}
extern "C" {
    pub fn ice_start_phy_timer_e82x(hw: *mut ice_hw, port: u8) -> c_int;
}
extern "C" {
    pub fn ice_phy_cfg_tx_offset_e82x(hw: *mut ice_hw, port: u8) -> c_int;
}
extern "C" {
    pub fn ice_phy_cfg_rx_offset_e82x(hw: *mut ice_hw, port: u8) -> c_int;
}
extern "C" {
    pub fn ice_phy_cfg_intr_e82x(hw: *mut ice_hw, quad: u8, ena: bool, threshold: u8) -> c_int;
}
// E810 family functions
extern "C" {
    pub fn ice_read_sma_ctrl(hw: *mut ice_hw, data: *mut u8) -> c_int;
}
extern "C" {
    pub fn ice_write_sma_ctrl(hw: *mut ice_hw, data: u8) -> c_int;
}
extern "C" {
    pub fn ice_ptp_read_sdp_ac(hw: *mut ice_hw, entries: *mut __le16, num_entries: *mut c_uint) -> c_int;
}
extern "C" {
    pub fn ice_cgu_get_num_pins(hw: *mut ice_hw, input: bool) -> c_int;
}
extern "C" {
    pub fn ice_cgu_get_pin_type(hw: *mut ice_hw, pin: u8, input: bool) -> dpll_pin_type;
}
extern "C" {
    pub fn ice_get_cgu_rclk_pin_info(hw: *mut ice_hw, base_idx: *mut u8, pin_num: *mut u8) -> c_int;
}
// ETH56G family functions
extern "C" {
    pub fn ice_ptp_read_tx_hwtstamp_status_eth56g(hw: *mut ice_hw, ts_status: *mut u32) -> c_int;
}
extern "C" {
    pub fn ice_stop_phy_timer_eth56g(hw: *mut ice_hw, port: u8, soft_reset: bool) -> c_int;
}
extern "C" {
    pub fn ice_start_phy_timer_eth56g(hw: *mut ice_hw, port: u8) -> c_int;
}
extern "C" {
    pub fn ice_phy_cfg_intr_eth56g(hw: *mut ice_hw, port: u8, ena: bool, threshold: u8) -> c_int;
}
extern "C" {
    pub fn ice_phy_cfg_ptp_1step_eth56g(hw: *mut ice_hw, port: u8) -> c_int;
}
extern "C" {
    pub fn ice_ptp_phy_soft_reset_eth56g(hw: *mut ice_hw, port: u8) -> c_int;
}
pub const ICE_ETH56G_NOMINAL_INCVAL: c_uint = 0x140000000ULL;
pub const ICE_ETH56G_NOMINAL_PCS_REF_TUS: c_uint = 0x100000000ULL;
pub const ICE_ETH56G_NOMINAL_PCS_REF_INC: c_uint = 0x300000000ULL;
pub const ICE_ETH56G_NOMINAL_THRESH4: c_uint = 0x7777;
pub const ICE_ETH56G_NOMINAL_TX_THRESH: c_uint = 0x6;
//
// ice_get_base_incval - Get base clock increment value
// @hw: pointer to the HW struct
//
// Return: base clock increment value for supported PHYs, 0 otherwise
//
extern "C" {
    pub fn ice_e82x_nominal_incval(_arg: ice_e82x_time_ref(hw)) -> return;
}
pub const PFTSYN_SEM_BYTES: c_int = 4;
pub const ICE_PTP_CLOCK_INDEX_0: c_uint = 0x00;
pub const ICE_PTP_CLOCK_INDEX_1: c_uint = 0x01;
// PHY timer commands
pub const SEL_CPK_SRC: c_int = 8;
pub const SEL_PHY_SRC: c_int = 3;
// Time Sync command Definitions

// PHY port Time Sync command definitions

pub const TS_CMD_MASK_E810: c_uint = 0xFF;
pub const TS_CMD_MASK: c_uint = 0xF;
pub const SYNC_EXEC_CMD: c_uint = 0x3;

// Macros to derive port low and high addresses on both quads

// PHY QUAD register base addresses
pub const Q_0_BASE: c_uint = 0x94000;
pub const Q_1_BASE: c_uint = 0x114000;
// Timestamp memory reset registers
pub const Q_REG_TS_CTRL: c_uint = 0x618;
pub const Q_REG_TS_CTRL_S: c_int = 0;

// Timestamp availability status registers
pub const Q_REG_TX_MEMORY_STATUS_L: c_uint = 0xCF0;
pub const Q_REG_TX_MEMORY_STATUS_U: c_uint = 0xCF4;
// Tx FIFO status registers
pub const Q_REG_FIFO23_STATUS: c_uint = 0xCF8;
pub const Q_REG_FIFO01_STATUS: c_uint = 0xCFC;
pub const Q_REG_FIFO02_S: c_int = 0;

pub const Q_REG_FIFO13_S: c_int = 10;

// Interrupt control Config registers
pub const Q_REG_TX_MEM_GBL_CFG: c_uint = 0xC08;
pub const Q_REG_TX_MEM_GBL_CFG_LANE_TYPE_S: c_int = 0;

// Tx Timestamp data registers
pub const Q_REG_TX_MEMORY_BANK_START: c_uint = 0xA00;
// PHY port register base addresses
pub const P_0_BASE: c_uint = 0x80000;
pub const P_4_BASE: c_uint = 0x106000;
// Timestamp init registers
pub const P_REG_RX_TIMER_INC_PRE_L: c_uint = 0x46C;
pub const P_REG_RX_TIMER_INC_PRE_U: c_uint = 0x470;
pub const P_REG_TX_TIMER_INC_PRE_L: c_uint = 0x44C;
pub const P_REG_TX_TIMER_INC_PRE_U: c_uint = 0x450;
// Timestamp match and adjust target registers
pub const P_REG_RX_TIMER_CNT_ADJ_L: c_uint = 0x474;
pub const P_REG_RX_TIMER_CNT_ADJ_U: c_uint = 0x478;
pub const P_REG_TX_TIMER_CNT_ADJ_L: c_uint = 0x454;
pub const P_REG_TX_TIMER_CNT_ADJ_U: c_uint = 0x458;
// Timestamp capture registers
pub const P_REG_RX_CAPTURE_L: c_uint = 0x4D8;
pub const P_REG_RX_CAPTURE_U: c_uint = 0x4DC;
pub const P_REG_TX_CAPTURE_L: c_uint = 0x4B4;
pub const P_REG_TX_CAPTURE_U: c_uint = 0x4B8;
// Timestamp PHY incval registers
pub const P_REG_TIMETUS_L: c_uint = 0x410;
pub const P_REG_TIMETUS_U: c_uint = 0x414;

pub const P_REG_40B_HIGH_S: c_int = 8;
// PHY window length registers
pub const P_REG_WL: c_uint = 0x40C;
pub const PTP_VERNIER_WL: c_uint = 0x111ed;
// PHY start registers
pub const P_REG_PS: c_uint = 0x408;
pub const P_REG_PS_START_S: c_int = 0;

pub const P_REG_PS_BYPASS_MODE_S: c_int = 1;

pub const P_REG_PS_ENA_CLK_S: c_int = 2;

pub const P_REG_PS_LOAD_OFFSET_S: c_int = 3;

pub const P_REG_PS_SFT_RESET_S: c_int = 11;

// PHY offset valid registers
pub const P_REG_TX_OV_STATUS: c_uint = 0x4D4;
pub const P_REG_TX_OV_STATUS_OV_S: c_int = 0;

pub const P_REG_RX_OV_STATUS: c_uint = 0x4F8;
pub const P_REG_RX_OV_STATUS_OV_S: c_int = 0;

// PHY offset ready registers
pub const P_REG_TX_OR: c_uint = 0x45C;
pub const P_REG_RX_OR: c_uint = 0x47C;
// PHY total offset registers
pub const P_REG_TOTAL_RX_OFFSET_L: c_uint = 0x460;
pub const P_REG_TOTAL_RX_OFFSET_U: c_uint = 0x464;
pub const P_REG_TOTAL_TX_OFFSET_L: c_uint = 0x440;
pub const P_REG_TOTAL_TX_OFFSET_U: c_uint = 0x444;
// Timestamp PAR/PCS registers
pub const P_REG_UIX66_10G_40G_L: c_uint = 0x480;
pub const P_REG_UIX66_10G_40G_U: c_uint = 0x484;
pub const P_REG_UIX66_25G_100G_L: c_uint = 0x488;
pub const P_REG_UIX66_25G_100G_U: c_uint = 0x48C;
pub const P_REG_DESK_PAR_RX_TUS_L: c_uint = 0x490;
pub const P_REG_DESK_PAR_RX_TUS_U: c_uint = 0x494;
pub const P_REG_DESK_PAR_TX_TUS_L: c_uint = 0x498;
pub const P_REG_DESK_PAR_TX_TUS_U: c_uint = 0x49C;
pub const P_REG_DESK_PCS_RX_TUS_L: c_uint = 0x4A0;
pub const P_REG_DESK_PCS_RX_TUS_U: c_uint = 0x4A4;
pub const P_REG_DESK_PCS_TX_TUS_L: c_uint = 0x4A8;
pub const P_REG_DESK_PCS_TX_TUS_U: c_uint = 0x4AC;
pub const P_REG_PAR_RX_TUS_L: c_uint = 0x420;
pub const P_REG_PAR_RX_TUS_U: c_uint = 0x424;
pub const P_REG_PAR_TX_TUS_L: c_uint = 0x428;
pub const P_REG_PAR_TX_TUS_U: c_uint = 0x42C;
pub const P_REG_PCS_RX_TUS_L: c_uint = 0x430;
pub const P_REG_PCS_RX_TUS_U: c_uint = 0x434;
pub const P_REG_PCS_TX_TUS_L: c_uint = 0x438;
pub const P_REG_PCS_TX_TUS_U: c_uint = 0x43C;
pub const P_REG_PAR_RX_TIME_L: c_uint = 0x4F0;
pub const P_REG_PAR_RX_TIME_U: c_uint = 0x4F4;
pub const P_REG_PAR_TX_TIME_L: c_uint = 0x4CC;
pub const P_REG_PAR_TX_TIME_U: c_uint = 0x4D0;
pub const P_REG_PAR_PCS_RX_OFFSET_L: c_uint = 0x4E8;
pub const P_REG_PAR_PCS_RX_OFFSET_U: c_uint = 0x4EC;
pub const P_REG_PAR_PCS_TX_OFFSET_L: c_uint = 0x4C4;
pub const P_REG_PAR_PCS_TX_OFFSET_U: c_uint = 0x4C8;
pub const P_REG_LINK_SPEED: c_uint = 0x4FC;
pub const P_REG_LINK_SPEED_SERDES_S: c_int = 0;

pub const P_REG_LINK_SPEED_FEC_MODE_S: c_int = 3;

// PHY timestamp related registers
pub const P_REG_PMD_ALIGNMENT: c_uint = 0x0FC;
pub const P_REG_RX_80_TO_160_CNT: c_uint = 0x6FC;
pub const P_REG_RX_80_TO_160_CNT_RXCYC_S: c_int = 0;

pub const P_REG_RX_40_TO_160_CNT: c_uint = 0x8FC;
pub const P_REG_RX_40_TO_160_CNT_RXCYC_S: c_int = 0;

// Rx FIFO status registers
pub const P_REG_RX_OV_FS: c_uint = 0x4F8;
pub const P_REG_RX_OV_FS_FIFO_STATUS_S: c_int = 2;

// Timestamp command registers
pub const P_REG_TX_TMR_CMD: c_uint = 0x448;
pub const P_REG_RX_TMR_CMD: c_uint = 0x468;
// E810 timesync enable register

// E810 shadow init time registers

// E810 shadow time adjust registers

// E810 timer command register
pub const E810_ETH_GLTSYN_CMD: c_uint = 0x03000344;
// E830 timer command register
pub const E830_ETH_GLTSYN_CMD: c_uint = 0x00088814;
// E810 PHC time register

// Source timer incval macros
pub const INCVAL_HIGH_M: c_uint = 0xFF;
// PHY 40b registers macros

pub const TS_LOW_M: c_uint = 0xFFFFFFFF;
pub const TS_HIGH_M: c_uint = 0xFF;
pub const TS_HIGH_S: c_int = 32;
pub const BYTES_PER_IDX_ADDR_L_U: c_int = 8;
pub const BYTES_PER_IDX_ADDR_L: c_int = 4;
// Tx timestamp low latency read definitions
pub const REG_LL_PROXY_H_TIMEOUT_US: c_int = 2000;

pub const REG_LL_PROXY_H_PHY_TMR_CMD_ADJ: c_uint = 0x1;
pub const REG_LL_PROXY_H_PHY_TMR_CMD_FREQ: c_uint = 0x2;

// Internal PHY timestamp address

// External PHY timestamp address

pub const LOW_TX_MEMORY_BANK_START: c_uint = 0x03090000;
pub const HIGH_TX_MEMORY_BANK_START: c_uint = 0x03090004;
// SMA controller pin control

pub const ICE_SMA_MIN_BIT: c_int = 3;
pub const ICE_SMA_MAX_BIT: c_int = 7;
pub const ICE_PCA9575_P1_OFFSET: c_int = 8;
// PCA9575 IO controller registers
pub const ICE_PCA9575_P0_IN: c_uint = 0x0;
// PCA9575 IO controller pin control

// ETH56G PHY register addresses
pub const PHY_REG_GLOBAL: c_uint = 0x0;

// Timestamp PHY incval registers
pub const PHY_REG_TIMETUS_L: c_uint = 0x8;
pub const PHY_REG_TIMETUS_U: c_uint = 0xC;
// Timestamp PCS registers
pub const PHY_PCS_REF_TUS_L: c_uint = 0x18;
pub const PHY_PCS_REF_TUS_U: c_uint = 0x1C;
// Timestamp PCS ref incval registers
pub const PHY_PCS_REF_INC_L: c_uint = 0x20;
pub const PHY_PCS_REF_INC_U: c_uint = 0x24;
// Timestamp init registers
pub const PHY_REG_RX_TIMER_INC_PRE_L: c_uint = 0x64;
pub const PHY_REG_RX_TIMER_INC_PRE_U: c_uint = 0x68;
pub const PHY_REG_TX_TIMER_INC_PRE_L: c_uint = 0x44;
pub const PHY_REG_TX_TIMER_INC_PRE_U: c_uint = 0x48;
// Timestamp match and adjust target registers
pub const PHY_REG_RX_TIMER_CNT_ADJ_L: c_uint = 0x6C;
pub const PHY_REG_RX_TIMER_CNT_ADJ_U: c_uint = 0x70;
pub const PHY_REG_TX_TIMER_CNT_ADJ_L: c_uint = 0x4C;
pub const PHY_REG_TX_TIMER_CNT_ADJ_U: c_uint = 0x50;
// Timestamp command registers
pub const PHY_REG_TX_TMR_CMD: c_uint = 0x40;
pub const PHY_REG_RX_TMR_CMD: c_uint = 0x60;
// Phy offset ready registers
pub const PHY_REG_TX_OFFSET_READY: c_uint = 0x54;
pub const PHY_REG_RX_OFFSET_READY: c_uint = 0x74;
// Phy total offset registers
pub const PHY_REG_TOTAL_TX_OFFSET_L: c_uint = 0x38;
pub const PHY_REG_TOTAL_TX_OFFSET_U: c_uint = 0x3C;
pub const PHY_REG_TOTAL_RX_OFFSET_L: c_uint = 0x58;
pub const PHY_REG_TOTAL_RX_OFFSET_U: c_uint = 0x5C;
// Timestamp capture registers
pub const PHY_REG_TX_CAPTURE_L: c_uint = 0x78;
pub const PHY_REG_TX_CAPTURE_U: c_uint = 0x7C;
pub const PHY_REG_RX_CAPTURE_L: c_uint = 0x8C;
pub const PHY_REG_RX_CAPTURE_U: c_uint = 0x90;
// Memory status registers
pub const PHY_REG_TX_MEMORY_STATUS_L: c_uint = 0x80;
pub const PHY_REG_TX_MEMORY_STATUS_U: c_uint = 0x84;
// Interrupt config register
pub const PHY_REG_TS_INT_CONFIG: c_uint = 0x88;
// XIF mode config register
pub const PHY_MAC_XIF_MODE: c_uint = 0x24;

// Macros to derive offsets for TimeStampLow and TimeStampHigh

pub const PHY_REG_DESKEW_0: c_uint = 0x94;

pub const PHY_REG_DESKEW_0_RLEVEL_FRAC_W: c_int = 3;

pub const PHY_REVISION_ETH56G: c_uint = 0x10200;
pub const PHY_VENDOR_TXLANE_THRESH: c_uint = 0x2000C;
pub const PHY_MAC_TSU_CONFIG: c_uint = 0x40;

pub const PHY_MAC_RX_MODULO: c_uint = 0x44;
pub const PHY_MAC_RX_OFFSET: c_uint = 0x48;

pub const PHY_MAC_TX_MODULO: c_uint = 0x4C;
pub const PHY_MAC_BLOCKTIME: c_uint = 0x50;
pub const PHY_MAC_MARKERTIME: c_uint = 0x54;
pub const PHY_MAC_TX_OFFSET: c_uint = 0x58;
pub const PHY_GPCS_BITSLIP: c_uint = 0x5C;
pub const PHY_PTP_INT_STATUS: c_uint = 0x7FD140;
// ETH56G registers shared per quad
// GPCS config register
pub const PHY_GPCS_CONFIG_REG0: c_uint = 0x268;

// 1-step PTP config
pub const PHY_PTP_1STEP_CONFIG: c_uint = 0x270;

pub const REF_SEL_NT_ENET: c_int = 0;
pub const REF_SEL_NT_EREF0: c_int = 1;
pub const REF_SEL_NT_SYNCE: c_int = 2;
