//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/iwl-config.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (C) 2005-2014, 2018-2021 Intel Corporation
// Copyright (C) 2016-2017 Intel Deutschland GmbH
// Copyright (C) 2018-2026 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_device_family {
    IWL_DEVICE_FAMILY_UNDEFINED,
    IWL_DEVICE_FAMILY_1000,
    IWL_DEVICE_FAMILY_100,
    IWL_DEVICE_FAMILY_2000,
    IWL_DEVICE_FAMILY_2030,
    IWL_DEVICE_FAMILY_105,
    IWL_DEVICE_FAMILY_135,
    IWL_DEVICE_FAMILY_5000,
    IWL_DEVICE_FAMILY_5150,
    IWL_DEVICE_FAMILY_6000,
    IWL_DEVICE_FAMILY_6000i,
    IWL_DEVICE_FAMILY_6005,
    IWL_DEVICE_FAMILY_6030,
    IWL_DEVICE_FAMILY_6050,
    IWL_DEVICE_FAMILY_6150,
    IWL_DEVICE_FAMILY_7000,
    IWL_DEVICE_FAMILY_8000,
    IWL_DEVICE_FAMILY_9000,
    IWL_DEVICE_FAMILY_22000,
    IWL_DEVICE_FAMILY_AX210,
    IWL_DEVICE_FAMILY_BZ,
    IWL_DEVICE_FAMILY_SC,
    IWL_DEVICE_FAMILY_DR,
}

//
// LED mode
// IWL_LED_DEFAULT:  use device default
// IWL_LED_RF_STATE: turn LED on/off based on RF state
// LED ON  = RF ON
// LED OFF = RF OFF
// IWL_LED_BLINK:    adjust led blink rate based on blink table
// IWL_LED_DISABLE:	led disabled
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_led_mode {
    IWL_LED_DEFAULT,
    IWL_LED_RF_STATE,
    IWL_LED_BLINK,
    IWL_LED_DISABLE,
}

//
// enum iwl_nvm_type - nvm formats
// @IWL_NVM: the regular format
// @IWL_NVM_EXT: extended NVM format
// @IWL_NVM_SDP: NVM format used by 3168 series
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_nvm_type {
    IWL_NVM,
    IWL_NVM_EXT,
    IWL_NVM_SDP,
}

//
// This is the threshold value of plcp error rate per 100mSecs.  It is
// used to set and check for the validity of plcp_delta.
//
pub const IWL_MAX_PLCP_ERR_THRESHOLD_MIN: c_int = 1;
pub const IWL_MAX_PLCP_ERR_THRESHOLD_DEF: c_int = 50;
pub const IWL_MAX_PLCP_ERR_LONG_THRESHOLD_DEF: c_int = 100;
pub const IWL_MAX_PLCP_ERR_EXT_LONG_THRESHOLD_DEF: c_int = 200;
pub const IWL_MAX_PLCP_ERR_THRESHOLD_MAX: c_int = 255;
pub const IWL_MAX_PLCP_ERR_THRESHOLD_DISABLE: c_int = 0;
// TX queue watchdog timeouts in mSecs
pub const IWL_WATCHDOG_DISABLED: c_int = 0;
pub const IWL_DEF_WD_TIMEOUT: c_int = 2500;
pub const IWL_LONG_WD_TIMEOUT: c_int = 10000;
pub const IWL_DEFAULT_MAX_TX_POWER: c_int = 22;

// Antenna presence definitions
pub const ANT_NONE: c_uint = 0x0;
pub const ANT_INVALID: c_uint = 0xff;

//
// struct iwl_fw_mon_reg - FW monitor register info
// @addr: register address
// @mask: register mask
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_fw_mon_reg {
    pub addr: u32,
    pub mask: u32,
}

//
// struct iwl_fw_mon_regs - FW monitor registers
// @write_ptr: write pointer register
// @cycle_cnt: cycle count register
// @cur_frag: current fragment in use
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_fw_mon_regs {
    pub write_ptr: iwl_fw_mon_reg,
    pub cycle_cnt: iwl_fw_mon_reg,
    pub cur_frag: iwl_fw_mon_reg,
}

//
// struct iwl_family_base_params - base parameters for an entire family
// @max_ll_items: max number of OTP blocks
// @shadow_ram_support: shadow support for OTP memory
// @led_compensation: compensate on the led on/off time per HW according
// to the deviation to achieve the desired led frequency.
// The detail algorithm is described in iwl-led.c
// @wd_timeout: TX queues watchdog timeout
// @max_event_log_size: size of event log buffer size for ucode event logging
// @shadow_reg_enable: HW shadow register support
// @apmg_not_supported: there's no APMG
// @apmg_wake_up_wa: should the MAC access REQ be asserted when a command
// is in flight. This is due to a HW bug in 7260, 3160 and 7265.
// @scd_chain_ext_wa: should the chain extension feature in SCD be disabled.
// @max_tfd_queue_size: max number of entries in tfd queue.
// @eeprom_size: EEPROM size
// @num_of_queues: number of HW TX queues supported
// @pcie_l1_allowed: PCIe L1 state is allowed
// @pll_cfg: PLL configuration needed
// @nvm_hw_section_num: the ID of the HW NVM section
// @features: hw features, any combination of feature_passlist
// @smem_offset: offset from which the SMEM begins
// @smem_len: the length of SMEM
// @mac_addr_from_csr: read HW address from CSR registers at this offset
// @d3_debug_data_base_addr: base address where D3 debug data is stored
// @d3_debug_data_length: length of the D3 debug data
// @min_ba_txq_size: minimum number of slots required in a TX queue used
// for aggregation
// @min_txq_size: minimum number of slots required in a TX queue
// @gp2_reg_addr: GP2 (timer) register address
// @mon_dbgi_regs: monitor DBGI registers
// @mon_dram_regs: monitor DRAM registers
// @mon_smem_regs: monitor SMEM registers
// @ucode_api_max: Highest version of uCode API supported by driver.
// @ucode_api_min: Lowest version of uCode API supported by driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_family_base_params {
    pub wd_timeout: c_uint,
    pub eeprom_size: u16,
    pub max_event_log_size: u16,
    pub /: *mut *mut u16 num_of_queues; / def: HW dependent,
    pub /: *mut *mut u32 max_tfd_queue_size; / def: HW dependent,
    pub max_ll_items: u8,
    pub led_compensation: u8,
    pub ucode_api_max: u16,
    pub ucode_api_min: u16,
    pub mac_addr_from_csr:10: u32,
    pub nvm_hw_section_num: u8,
    pub features: netdev_features_t,
    pub smem_offset: u32,
    pub smem_len: u32,
    pub d3_debug_data_base_addr: u32,
    pub d3_debug_data_length: u32,
    pub min_txq_size: u32,
    pub gp2_reg_addr: u32,
    pub min_ba_txq_size: u32,
    pub mon_dram_regs: iwl_fw_mon_regs,
    pub mon_smem_regs: iwl_fw_mon_regs,
    pub mon_dbgi_regs: iwl_fw_mon_regs,
}

//
// FW is released as "core N release", and we used to have a
// gap of 3 between the API version and core number. Now the
// reported API version will be 1000 + core and we encode it
// in the filename as "c<core>".
//
pub const API_IS_CORE_START: c_int = 1000;
pub const API_TO_CORE_OFFS: c_int = 3;

//
// @stbc: support Tx STBC and 1*SS Rx STBC
// @ldpc: support Tx/Rx with LDPC
// @use_rts_for_aggregation: use rts/cts protection for HT traffic
// @ht40_bands: bitmap of bands (using %NL80211_BAND_*) that support HT40
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_ht_params {
    pub ht40_bands: u8,
}

//
// Tx-backoff threshold
// @temperature: The threshold in Celsius
// @backoff: The tx-backoff in uSec
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tt_tx_backoff {
    pub temperature: i32,
    pub backoff: u32,
}

pub const TT_TX_BACKOFF_SIZE: c_int = 6;
//
// struct iwl_tt_params - thermal throttling parameters
// @ct_kill_entry: CT Kill entry threshold
// @ct_kill_exit: CT Kill exit threshold
// @ct_kill_duration: The time  intervals (in uSec) in which the driver needs
// to checks whether to exit CT Kill.
// @dynamic_smps_entry: Dynamic SMPS entry threshold
// @dynamic_smps_exit: Dynamic SMPS exit threshold
// @tx_protection_entry: TX protection entry threshold
// @tx_protection_exit: TX protection exit threshold
// @tx_backoff: Array of thresholds for tx-backoff , in ascending order.
// @support_ct_kill: Support CT Kill?
// @support_dynamic_smps: Support dynamic SMPS?
// @support_tx_protection: Support tx protection?
// @support_tx_backoff: Support tx-backoff?
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tt_params {
    pub ct_kill_entry: u32,
    pub ct_kill_exit: u32,
    pub ct_kill_duration: u32,
    pub dynamic_smps_entry: u32,
    pub dynamic_smps_exit: u32,
    pub tx_protection_entry: u32,
    pub tx_protection_exit: u32,
    pub tx_backoff: [iwl_tt_tx_backoff; TT_TX_BACKOFF_SIZE],
}

//
// information on how to parse the EEPROM
//
pub const EEPROM_REG_BAND_1_CHANNELS: c_uint = 0x08;
pub const EEPROM_REG_BAND_2_CHANNELS: c_uint = 0x26;
pub const EEPROM_REG_BAND_3_CHANNELS: c_uint = 0x42;
pub const EEPROM_REG_BAND_4_CHANNELS: c_uint = 0x5C;
pub const EEPROM_REG_BAND_5_CHANNELS: c_uint = 0x74;
pub const EEPROM_REG_BAND_24_HT40_CHANNELS: c_uint = 0x82;
pub const EEPROM_REG_BAND_52_HT40_CHANNELS: c_uint = 0x92;
pub const EEPROM_6000_REG_BAND_24_HT40_CHANNELS: c_uint = 0x80;
pub const EEPROM_REGULATORY_BAND_NO_HT40: c_int = 0;
// lower blocks contain EEPROM image and calibration data

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_eeprom_params {
    pub regulatory_bands: [u8; 7],
    pub enhanced_txpower: bool,
}

// Tx-backoff power threshold
// @pwr: The power limit in mw
// @backoff: The tx-backoff in uSec
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_pwr_tx_backoff {
    pub pwr: u32,
    pub backoff: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_mac_cfg_ltr_delay {
    IWL_CFG_TRANS_LTR_DELAY_NONE	= 0,
    IWL_CFG_TRANS_LTR_DELAY_200US	= 1,
    IWL_CFG_TRANS_LTR_DELAY_2500US	= 2,
    IWL_CFG_TRANS_LTR_DELAY_1820US	= 3,
}

//
// struct iwl_mac_cfg - information about the MAC-specific device part
//
// These values are specific to the device ID and do not change when
// multiple configs are used for a single device ID.  They values are
// used, among other things, to boot the NIC so that the HW REV or
// RFID can be read before deciding the remaining parameters to use.
//
// @base: pointer to basic parameters
// @device_family: the device family
// @umac_prph_offset: offset to add to UMAC periphery address
// @xtal_latency: power up latency to get the xtal stabilized
// @extra_phy_cfg_flags: extra configuration flags to pass to the PHY
// @gen2: 22000 and on transport operation
// @mq_rx_supported: multi-queue rx support
// @integrated: discrete or integrated
// @low_latency_xtal: use the low latency xtal if supported
// @bisr_workaround: BISR hardware workaround (for 22260 series devices)
// @ltr_delay: LTR delay parameter, &enum iwl_mac_cfg_ltr_delay.
// @imr_enabled: use the IMR if supported.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mac_cfg {
    pub base: *const iwl_family_base_params,
    pub device_family: iwl_device_family,
    pub umac_prph_offset: u32,
    pub xtal_latency: u32,
    pub extra_phy_cfg_flags: u32,
}

//
// These sizes were picked according to 8 MSDUs inside 64/256/512 A-MSDUs
// in an A-MPDU, with additional overhead to account for processing time.
// They will be doubled for MACs starting from So/Ty that don't support
// putting multiple frames into a single buffer.
//

//
// struct iwl_rf_cfg - RF/CRF configuration data
// @fw_name_pre: Firmware filename prefix. The api version and extension
// (.ucode) will be added to filename before loading from disk. The
// filename is constructed as <fw_name_pre>-<api>.ucode.
// name will be generated dynamically
// @ucode_api_max: Highest version of uCode API supported by driver.
// @ucode_api_min: Lowest version of uCode API supported by driver.
// @max_inst_size: The maximal length of the fw inst section (only DVM)
// @max_data_size: The maximal length of the fw data section (only DVM)
// @valid_tx_ant: valid transmit antenna
// @valid_rx_ant: valid receive antenna
// @non_shared_ant: the antenna that is for WiFi only
// @nvm_ver: NVM version
// @nvm_calib_ver: NVM calibration version
// @bw_limit: bandwidth limit for this device, if non-zero
// @ht_params: point to ht parameters
// @eeprom_params: EEPROM parameters (old devices)
// @thermal_params: Thermal throttling parameters
// @lp_xtal_workaround: low-power crystal workaround needed
// @led_mode: 0=blinking, 1=On(RF On)/Off(RF Off)
// @rx_with_siso_diversity: 1x1 device with rx antenna diversity
// @tx_with_siso_diversity: 1x1 device with tx antenna diversity
// @internal_wimax_coex: internal wifi/wimax combo device
// @host_interrupt_operation_mode: device needs host interrupt operation
// mode set
// @pwr_tx_backoffs: translation table between power limits and backoffs
// @dccm_offset: offset from which DCCM begins
// @dccm_len: length of DCCM (including runtime stack CCM)
// @dccm2_offset: offset from which the second DCCM begins
// @dccm2_len: length of the second DCCM
// @vht_mu_mimo_supported: VHT MU-MIMO support
// @nvm_type: see &enum iwl_nvm_type
// @uhb_supported: ultra high band channels supported
// @unii9_supported: UNII-9 channels supported
// @eht_supported: EHT supported
// @uhr_supported: UHR supported
// @num_rbds: number of receive buffer descriptors to use
// (only used for multi-queue capable devices)
//
// We enable the driver to be backward compatible wrt. hardware features.
// API differences in uCode shouldn't be handled here but through TLVs
// and/or the uCode API version instead.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_rf_cfg {
// params specific to an individual device within a device family
    pub fw_name_pre: *const c_char,
// params likely to change within a device family
    pub ht_params: iwl_ht_params,
    pub eeprom_params: *const iwl_eeprom_params,
    pub pwr_tx_backoffs: *const iwl_pwr_tx_backoff,
    pub thermal_params: *const iwl_tt_params,
    pub led_mode: iwl_led_mode,
    pub nvm_type: iwl_nvm_type,
    pub max_data_size: u32,
    pub max_inst_size: u32,
    pub dccm_offset: u32,
    pub dccm_len: u32,
    pub dccm2_offset: u32,
    pub dccm2_len: u32,
    pub nvm_ver: u16,
    pub nvm_calib_ver: u16,
    pub bw_limit: u16,
    pub valid_tx_ant: u8,
    pub valid_rx_ant: u8,
    pub non_shared_ant: u8,
    pub ucode_api_max: u16,
    pub ucode_api_min: u16,
    pub num_rbds: u16,
}

pub const IWL_CFG_MAC_TYPE_PU: c_uint = 0x31;
pub const IWL_CFG_MAC_TYPE_TH: c_uint = 0x32;
pub const IWL_CFG_MAC_TYPE_QU: c_uint = 0x33;
pub const IWL_CFG_MAC_TYPE_CC: c_uint = 0x34;
pub const IWL_CFG_MAC_TYPE_QUZ: c_uint = 0x35;
pub const IWL_CFG_MAC_TYPE_SO: c_uint = 0x37;
pub const IWL_CFG_MAC_TYPE_TY: c_uint = 0x42;
pub const IWL_CFG_MAC_TYPE_SOF: c_uint = 0x43;
pub const IWL_CFG_MAC_TYPE_MA: c_uint = 0x44;
pub const IWL_CFG_MAC_TYPE_BZ: c_uint = 0x46;
pub const IWL_CFG_MAC_TYPE_GL: c_uint = 0x47;
pub const IWL_CFG_MAC_TYPE_SC: c_uint = 0x48;
pub const IWL_CFG_MAC_TYPE_SC2: c_uint = 0x49;
pub const IWL_CFG_MAC_TYPE_SC2F: c_uint = 0x4A;
pub const IWL_CFG_MAC_TYPE_BZ_W: c_uint = 0x4B;
pub const IWL_CFG_MAC_TYPE_BR: c_uint = 0x4C;
pub const IWL_CFG_MAC_TYPE_DR: c_uint = 0x4D;
pub const IWL_CFG_RF_TYPE_JF2: c_uint = 0x105;
pub const IWL_CFG_RF_TYPE_JF1: c_uint = 0x108;
pub const IWL_CFG_RF_TYPE_HR2: c_uint = 0x10A;
pub const IWL_CFG_RF_TYPE_HR1: c_uint = 0x10C;
pub const IWL_CFG_RF_TYPE_GF: c_uint = 0x10D;
pub const IWL_CFG_RF_TYPE_FM: c_uint = 0x112;
pub const IWL_CFG_RF_TYPE_WH: c_uint = 0x113;
pub const IWL_CFG_RF_TYPE_PE: c_uint = 0x114;
pub const IWL_CFG_RF_ID_TH: c_uint = 0x1;
pub const IWL_CFG_RF_ID_TH1: c_uint = 0x1;
pub const IWL_CFG_RF_ID_JF: c_uint = 0x3;
pub const IWL_CFG_RF_ID_JF1: c_uint = 0x6;
pub const IWL_CFG_RF_ID_JF1_DIV: c_uint = 0xA;
pub const IWL_CFG_RF_ID_HR: c_uint = 0x7;
pub const IWL_CFG_RF_ID_HR1: c_uint = 0x4;
pub const IWL_CFG_CORES_BT: c_uint = 0x0;
pub const IWL_CFG_CORES_BT_GNSS: c_uint = 0x5;
pub const IWL_CFG_NO_CDB: c_uint = 0x0;
pub const IWL_CFG_CDB: c_uint = 0x1;
pub const IWL_CFG_NO_JACKET: c_uint = 0x0;
pub const IWL_CFG_IS_JACKET: c_uint = 0x1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_dev_info {
    pub cfg: *const iwl_rf_cfg,
    pub name: *const c_char,
    pub device: u16,
    pub subdevice: u16,
}

//
// This list declares the config structures for all devices.
//

