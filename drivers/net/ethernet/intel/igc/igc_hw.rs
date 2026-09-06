//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/igc/igc_hw.h
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
// Copyright (c)  2018 Intel Corporation

pub const IGC_DEV_ID_I225_LM: c_uint = 0x15F2;
pub const IGC_DEV_ID_I225_V: c_uint = 0x15F3;
pub const IGC_DEV_ID_I225_I: c_uint = 0x15F8;
pub const IGC_DEV_ID_I220_V: c_uint = 0x15F7;
pub const IGC_DEV_ID_I225_K: c_uint = 0x3100;
pub const IGC_DEV_ID_I225_K2: c_uint = 0x3101;
pub const IGC_DEV_ID_I226_K: c_uint = 0x3102;
pub const IGC_DEV_ID_I225_LMVP: c_uint = 0x5502;
pub const IGC_DEV_ID_I226_LMVP: c_uint = 0x5503;
pub const IGC_DEV_ID_I225_IT: c_uint = 0x0D9F;
pub const IGC_DEV_ID_I226_LM: c_uint = 0x125B;
pub const IGC_DEV_ID_I226_V: c_uint = 0x125C;
pub const IGC_DEV_ID_I226_IT: c_uint = 0x125D;
pub const IGC_DEV_ID_I221_V: c_uint = 0x125E;
pub const IGC_DEV_ID_I226_BLANK_NVM: c_uint = 0x125F;
pub const IGC_DEV_ID_I225_BLANK_NVM: c_uint = 0x15FD;
// Function pointers for the MAC.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct igc_mac_operations {
    pub hw): *mut *mut s32 (check_for_link)(struct igc_hw,
    pub hw): *mut *mut s32 (reset_hw)(struct igc_hw,
    pub hw): *mut *mut s32 (init_hw)(struct igc_hw,
    pub hw): *mut *mut s32 (setup_physical_interface)(struct igc_hw,
    pub index): *mut *mut *mut *mut void (rar_set)(struct igc_hw hw, u8 address, u32,
    pub hw): *mut *mut s32 (read_mac_addr)(struct igc_hw,
    pub duplex): *mut u16,
    pub mask): *mut *mut *mut s32 (acquire_swfw_sync)(struct igc_hw hw, u16,
    pub mask): *mut *mut *mut void (release_swfw_sync)(struct igc_hw hw, u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum igc_mac_type {
    igc_undefined = 0,
    igc_i225,
    igc_num_macs  /* List is 1-based, so subtract 1 for true count. */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum igc_media_type {
    igc_media_type_unknown = 0,
    igc_media_type_copper = 1,
    igc_num_media_types
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum igc_nvm_type {
    igc_nvm_unknown = 0,
    igc_nvm_eeprom_spi,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igc_info {
    pub hw): *mut *mut s32 (get_invariants)(struct igc_hw,
    pub mac_ops: *mut igc_mac_operations,
    pub phy_ops: *const igc_phy_operations,
    pub nvm_ops: *mut igc_nvm_operations,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum igc_forced_speed_duplex {
    IGC_FORCED_10H,
    IGC_FORCED_10F,
    IGC_FORCED_100H,
    IGC_FORCED_100F,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igc_mac_info {
    pub ops: igc_mac_operations,
    pub addr: [u8; ETH_ALEN],
    pub perm_addr: [u8; ETH_ALEN],
    pub type: igc_mac_type,
    pub mc_filter_type: u32,
    pub mta_reg_count: u16,
    pub uta_reg_count: u16,
    pub mta_shadow: [u32; MAX_MTA_REG],
    pub rar_entry_count: u16,
    pub asf_firmware_present: bool,
    pub arc_subsystem_valid: bool,
    pub get_link_status: bool,
    pub autoneg_enabled: bool,
    pub forced_speed_duplex: igc_forced_speed_duplex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igc_nvm_operations {
    pub hw): *mut *mut s32 (acquire)(struct igc_hw,
    pub data): *mut *mut *mut s32 (read)(struct igc_hw hw, u16 offset, u16 i, u16,
    pub hw): *mut *mut void (release)(struct igc_hw,
    pub data): *mut *mut *mut s32 (write)(struct igc_hw hw, u16 offset, u16 i, u16,
    pub hw): *mut *mut s32 (update)(struct igc_hw,
    pub hw): *mut *mut s32 (validate)(struct igc_hw,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igc_phy_operations {
    pub hw): *mut *mut s32 (acquire)(struct igc_hw,
    pub hw): *mut *mut s32 (check_reset_block)(struct igc_hw,
    pub hw): *mut *mut s32 (force_speed_duplex)(struct igc_hw,
    pub hw): *mut *mut s32 (get_phy_info)(struct igc_hw,
    pub data): *mut *mut *mut s32 (read_reg)(struct igc_hw hw, u32 address, u16,
    pub hw): *mut *mut void (release)(struct igc_hw,
    pub hw): *mut *mut s32 (reset)(struct igc_hw,
    pub data): *mut *mut *mut s32 (write_reg)(struct igc_hw hw, u32 address, u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igc_nvm_info {
    pub ops: igc_nvm_operations,
    pub type: igc_nvm_type,
    pub word_size: u16,
    pub delay_usec: u16,
    pub address_bits: u16,
    pub opcode_bits: u16,
    pub page_size: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igc_phy_info {
    pub ops: igc_phy_operations,
    pub addr: u32,
    pub id: u32,
    pub /: *mut *mut u32 reset_delay_us; / in usec,
    pub revision: u32,
    pub media_type: igc_media_type,
    pub autoneg_advertised: u16,
    pub autoneg_mask: u16,
    pub mdix: u8,
    pub is_mdix: bool,
    pub speed_downgraded: bool,
    pub autoneg_wait_to_complete: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igc_bus_info {
    pub func: u16,
    pub pci_cmd_word: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum igc_fc_mode {
    igc_fc_none = 0,
    igc_fc_rx_pause,
    igc_fc_tx_pause,
    igc_fc_full,
    igc_fc_default = 0xFF
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igc_fc_info {
    pub /: *mut *mut u32 high_water; / Flow control high-water mark,
    pub /: *mut *mut u32 low_water; / Flow control low-water mark,
    pub /: *mut *mut u16 pause_time; / Flow control pause timer,
    pub /: *mut *mut bool send_xon; / Flow control send XON,
    pub /: *mut *mut bool strict_ieee; / Strict IEEE mode,
    pub /: *mut *mut igc_fc_mode current_mode; / Type of flow control,
    pub requested_mode: igc_fc_mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igc_dev_spec_base {
    pub clear_semaphore_once: bool,
    pub eee_enable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igc_hw {
    pub back: *mut c_void,
    pub hw_addr: *mut u8 __iomem,
    pub io_base: c_ulong,
    pub mac: igc_mac_info,
    pub fc: igc_fc_info,
    pub nvm: igc_nvm_info,
    pub phy: igc_phy_info,
    pub bus: igc_bus_info,
    pub _base: igc_dev_spec_base,
    pub dev_spec: },
    pub device_id: u16,
    pub subsystem_vendor_id: u16,
    pub subsystem_device_id: u16,
    pub vendor_id: u16,
    pub revision_id: u8,
}

// Statistics counters collected by the MAC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct igc_hw_stats {
    pub crcerrs: u64,
    pub algnerrc: u64,
    pub symerrs: u64,
    pub rxerrc: u64,
    pub mpc: u64,
    pub scc: u64,
    pub ecol: u64,
    pub mcc: u64,
    pub latecol: u64,
    pub colc: u64,
    pub dc: u64,
    pub tncrs: u64,
    pub sec: u64,
    pub cexterr: u64,
    pub rlec: u64,
    pub xonrxc: u64,
    pub xontxc: u64,
    pub xoffrxc: u64,
    pub xofftxc: u64,
    pub fcruc: u64,
    pub prc64: u64,
    pub prc127: u64,
    pub prc255: u64,
    pub prc511: u64,
    pub prc1023: u64,
    pub prc1522: u64,
    pub tlpic: u64,
    pub rlpic: u64,
    pub gprc: u64,
    pub bprc: u64,
    pub mprc: u64,
    pub gptc: u64,
    pub gorc: u64,
    pub gotc: u64,
    pub rnbc: u64,
    pub ruc: u64,
    pub rfc: u64,
    pub roc: u64,
    pub rjc: u64,
    pub mgprc: u64,
    pub mgpdc: u64,
    pub mgptc: u64,
    pub tor: u64,
    pub tot: u64,
    pub tpr: u64,
    pub tpt: u64,
    pub ptc64: u64,
    pub ptc127: u64,
    pub ptc255: u64,
    pub ptc511: u64,
    pub ptc1023: u64,
    pub ptc1522: u64,
    pub mptc: u64,
    pub bptc: u64,
    pub tsctc: u64,
    pub tsctfc: u64,
    pub iac: u64,
    pub htdpmc: u64,
    pub rpthc: u64,
    pub hgptc: u64,
    pub hgorc: u64,
    pub hgotc: u64,
    pub lenerrs: u64,
    pub scvpc: u64,
    pub hrmpc: u64,
    pub doosync: u64,
    pub o2bgptc: u64,
    pub o2bspc: u64,
    pub b2ospc: u64,
    pub b2ogprc: u64,
    pub txdrop: u64,
}

