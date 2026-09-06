//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/igb/e1000_hw.h
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
// Copyright(c) 2007 - 2018 Intel Corporation.

pub const E1000_DEV_ID_82576: c_uint = 0x10C9;
pub const E1000_DEV_ID_82576_FIBER: c_uint = 0x10E6;
pub const E1000_DEV_ID_82576_SERDES: c_uint = 0x10E7;
pub const E1000_DEV_ID_82576_QUAD_COPPER: c_uint = 0x10E8;
pub const E1000_DEV_ID_82576_QUAD_COPPER_ET2: c_uint = 0x1526;
pub const E1000_DEV_ID_82576_NS: c_uint = 0x150A;
pub const E1000_DEV_ID_82576_NS_SERDES: c_uint = 0x1518;
pub const E1000_DEV_ID_82576_SERDES_QUAD: c_uint = 0x150D;
pub const E1000_DEV_ID_82575EB_COPPER: c_uint = 0x10A7;
pub const E1000_DEV_ID_82575EB_FIBER_SERDES: c_uint = 0x10A9;
pub const E1000_DEV_ID_82575GB_QUAD_COPPER: c_uint = 0x10D6;
pub const E1000_DEV_ID_82580_COPPER: c_uint = 0x150E;
pub const E1000_DEV_ID_82580_FIBER: c_uint = 0x150F;
pub const E1000_DEV_ID_82580_SERDES: c_uint = 0x1510;
pub const E1000_DEV_ID_82580_SGMII: c_uint = 0x1511;
pub const E1000_DEV_ID_82580_COPPER_DUAL: c_uint = 0x1516;
pub const E1000_DEV_ID_82580_QUAD_FIBER: c_uint = 0x1527;
pub const E1000_DEV_ID_DH89XXCC_SGMII: c_uint = 0x0438;
pub const E1000_DEV_ID_DH89XXCC_SERDES: c_uint = 0x043A;
pub const E1000_DEV_ID_DH89XXCC_BACKPLANE: c_uint = 0x043C;
pub const E1000_DEV_ID_DH89XXCC_SFP: c_uint = 0x0440;
pub const E1000_DEV_ID_I350_COPPER: c_uint = 0x1521;
pub const E1000_DEV_ID_I350_FIBER: c_uint = 0x1522;
pub const E1000_DEV_ID_I350_SERDES: c_uint = 0x1523;
pub const E1000_DEV_ID_I350_SGMII: c_uint = 0x1524;
pub const E1000_DEV_ID_I210_COPPER: c_uint = 0x1533;
pub const E1000_DEV_ID_I210_FIBER: c_uint = 0x1536;
pub const E1000_DEV_ID_I210_SERDES: c_uint = 0x1537;
pub const E1000_DEV_ID_I210_SGMII: c_uint = 0x1538;
pub const E1000_DEV_ID_I210_COPPER_FLASHLESS: c_uint = 0x157B;
pub const E1000_DEV_ID_I210_SERDES_FLASHLESS: c_uint = 0x157C;
pub const E1000_DEV_ID_I211_COPPER: c_uint = 0x1539;
pub const E1000_DEV_ID_I354_BACKPLANE_1GBPS: c_uint = 0x1F40;
pub const E1000_DEV_ID_I354_SGMII: c_uint = 0x1F41;
pub const E1000_DEV_ID_I354_BACKPLANE_2_5GBPS: c_uint = 0x1F45;
pub const E1000_REVISION_2: c_int = 2;
pub const E1000_REVISION_4: c_int = 4;
pub const E1000_FUNC_0: c_int = 0;
pub const E1000_FUNC_1: c_int = 1;
pub const E1000_FUNC_2: c_int = 2;
pub const E1000_FUNC_3: c_int = 3;
pub const E1000_ALT_MAC_ADDRESS_OFFSET_LAN0: c_int = 0;
pub const E1000_ALT_MAC_ADDRESS_OFFSET_LAN1: c_int = 3;
pub const E1000_ALT_MAC_ADDRESS_OFFSET_LAN2: c_int = 6;
pub const E1000_ALT_MAC_ADDRESS_OFFSET_LAN3: c_int = 9;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e1000_mac_type {
    e1000_undefined = 0,
    e1000_82575,
    e1000_82576,
    e1000_82580,
    e1000_i350,
    e1000_i354,
    e1000_i210,
    e1000_i211,
    e1000_num_macs  /* List is 1-based, so subtract 1 for true count. */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e1000_media_type {
    e1000_media_type_unknown = 0,
    e1000_media_type_copper = 1,
    e1000_media_type_fiber = 2,
    e1000_media_type_internal_serdes = 3,
    e1000_num_media_types
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e1000_nvm_type {
    e1000_nvm_unknown = 0,
    e1000_nvm_none,
    e1000_nvm_eeprom_spi,
    e1000_nvm_flash_hw,
    e1000_nvm_invm,
    e1000_nvm_flash_sw
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e1000_nvm_override {
    e1000_nvm_override_none = 0,
    e1000_nvm_override_spi_small,
    e1000_nvm_override_spi_large,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e1000_phy_type {
    e1000_phy_unknown = 0,
    e1000_phy_none,
    e1000_phy_m88,
    e1000_phy_igp,
    e1000_phy_igp_2,
    e1000_phy_gg82563,
    e1000_phy_igp_3,
    e1000_phy_ife,
    e1000_phy_82580,
    e1000_phy_i210,
    e1000_phy_bcm54616,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e1000_bus_type {
    e1000_bus_type_unknown = 0,
    e1000_bus_type_pci,
    e1000_bus_type_pcix,
    e1000_bus_type_pci_express,
    e1000_bus_type_reserved
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e1000_bus_speed {
    e1000_bus_speed_unknown = 0,
    e1000_bus_speed_33,
    e1000_bus_speed_66,
    e1000_bus_speed_100,
    e1000_bus_speed_120,
    e1000_bus_speed_133,
    e1000_bus_speed_2500,
    e1000_bus_speed_5000,
    e1000_bus_speed_reserved
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e1000_bus_width {
    e1000_bus_width_unknown = 0,
    e1000_bus_width_pcie_x1,
    e1000_bus_width_pcie_x2,
    e1000_bus_width_pcie_x4 = 4,
    e1000_bus_width_pcie_x8 = 8,
    e1000_bus_width_32,
    e1000_bus_width_64,
    e1000_bus_width_reserved
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e1000_1000t_rx_status {
    e1000_1000t_rx_status_not_ok = 0,
    e1000_1000t_rx_status_ok,
    e1000_1000t_rx_status_undefined = 0xFF
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e1000_rev_polarity {
    e1000_rev_polarity_normal = 0,
    e1000_rev_polarity_reversed,
    e1000_rev_polarity_undefined = 0xFF
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e1000_fc_mode {
    e1000_fc_none = 0,
    e1000_fc_rx_pause,
    e1000_fc_tx_pause,
    e1000_fc_full,
    e1000_fc_default = 0xFF
}

// Statistics counters collected by the MAC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_hw_stats {
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
    pub icrxptc: u64,
    pub icrxatc: u64,
    pub ictxptc: u64,
    pub ictxatc: u64,
    pub ictxqec: u64,
    pub ictxqmtc: u64,
    pub icrxdmtc: u64,
    pub icrxoc: u64,
    pub cbtmpc: u64,
    pub htdpmc: u64,
    pub cbrdpc: u64,
    pub cbrmpc: u64,
    pub rpthc: u64,
    pub hgptc: u64,
    pub htcbdpc: u64,
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
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_host_mng_dhcp_cookie {
    pub signature: u32,
    pub status: u8,
    pub reserved0: u8,
    pub vlan_id: u16,
    pub reserved1: u32,
    pub reserved2: u16,
    pub reserved3: u8,
    pub checksum: u8,
}

// Host Interface "Rev 1"
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_host_command_header {
    pub command_id: u8,
    pub command_length: u8,
    pub command_options: u8,
    pub checksum: u8,
}

pub const E1000_HI_MAX_DATA_LENGTH: c_int = 252;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_host_command_info {
    pub command_header: e1000_host_command_header,
    pub command_data: [u8; E1000_HI_MAX_DATA_LENGTH],
}

// Host Interface "Rev 2"
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_host_mng_command_header {
    pub command_id: u8,
    pub checksum: u8,
    pub reserved1: u16,
    pub reserved2: u16,
    pub command_length: u16,
}

pub const E1000_HI_MAX_MNG_DATA_LENGTH: c_uint = 0x6F8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_host_mng_command_info {
    pub command_header: e1000_host_mng_command_header,
    pub command_data: [u8; E1000_HI_MAX_MNG_DATA_LENGTH],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_mac_operations {
    pub ): *mut *mut s32 (check_for_link)(struct e1000_hw,
    pub ): *mut *mut s32 (reset_hw)(struct e1000_hw,
    pub ): *mut *mut s32 (init_hw)(struct e1000_hw,
    pub ): *mut *mut bool (check_mng_mode)(struct e1000_hw,
    pub ): *mut *mut s32 (setup_physical_interface)(struct e1000_hw,
    pub u32): *mut *mut *mut *mut void (rar_set)(struct e1000_hw , u8 ,,
    pub ): *mut *mut s32 (read_mac_addr)(struct e1000_hw,
    pub ): *mut *mut *mut *mut s32 (get_speed_and_duplex)(struct e1000_hw , u16 , u16,
    pub u16): *mut *mut *mut s32 (acquire_swfw_sync)(struct e1000_hw ,,
    pub u16): *mut *mut *mut void (release_swfw_sync)(struct e1000_hw ,,

    pub ): *mut *mut s32 (get_thermal_sensor_data)(struct e1000_hw,
    pub ): *mut *mut s32 (init_thermal_sensor_thresh)(struct e1000_hw,

    pub u32): *mut *mut *mut void (write_vfta)(struct e1000_hw , u32,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_phy_operations {
    pub ): *mut *mut s32 (acquire)(struct e1000_hw,
    pub ): *mut *mut s32 (check_polarity)(struct e1000_hw,
    pub ): *mut *mut s32 (check_reset_block)(struct e1000_hw,
    pub ): *mut *mut s32 (force_speed_duplex)(struct e1000_hw,
    pub hw): *mut *mut s32 (get_cfg_done)(struct e1000_hw,
    pub ): *mut *mut s32 (get_cable_length)(struct e1000_hw,
    pub ): *mut *mut s32 (get_phy_info)(struct e1000_hw,
    pub ): *mut *mut *mut s32 (read_reg)(struct e1000_hw , u32, u16,
    pub ): *mut *mut void (release)(struct e1000_hw,
    pub ): *mut *mut s32 (reset)(struct e1000_hw,
    pub bool): *mut *mut *mut s32 (set_d0_lplu_state)(struct e1000_hw ,,
    pub bool): *mut *mut *mut s32 (set_d3_lplu_state)(struct e1000_hw ,,
    pub u16): *mut *mut *mut s32 (write_reg)(struct e1000_hw , u32,,
    pub ): *mut *mut *mut s32 (read_i2c_byte)(struct e1000_hw , u8, u8, u8,
    pub u8): *mut *mut *mut s32 (write_i2c_byte)(struct e1000_hw , u8, u8,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_nvm_operations {
    pub ): *mut *mut s32 (acquire)(struct e1000_hw,
    pub ): *mut *mut *mut s32 (read)(struct e1000_hw , u16, u16, u16,
    pub ): *mut *mut void (release)(struct e1000_hw,
    pub ): *mut *mut *mut s32 (write)(struct e1000_hw , u16, u16, u16,
    pub ): *mut *mut s32 (update)(struct e1000_hw,
    pub ): *mut *mut s32 (validate)(struct e1000_hw,
    pub ): *mut *mut *mut s32 (valid_led_default)(struct e1000_hw , u16,
}

pub const E1000_MAX_SENSORS: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_thermal_diode_data {
    pub location: u8,
    pub temp: u8,
    pub caution_thresh: u8,
    pub max_op_thresh: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_thermal_sensor_data {
    pub sensor: [e1000_thermal_diode_data; E1000_MAX_SENSORS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_info {
    pub ): *mut *mut s32 (get_invariants)(struct e1000_hw,
    pub mac_ops: *mut e1000_mac_operations,
    pub phy_ops: *const e1000_phy_operations,
    pub nvm_ops: *mut e1000_nvm_operations,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_mac_info {
    pub ops: e1000_mac_operations,
    pub addr: [u8; 6],
    pub perm_addr: [u8; 6],
    pub type: e1000_mac_type,
    pub ledctl_default: u32,
    pub ledctl_mode1: u32,
    pub ledctl_mode2: u32,
    pub mc_filter_type: u32,
    pub txcw: u32,
    pub mta_reg_count: u16,
    pub uta_reg_count: u16,
// Maximum size of the MTA register table in all supported adapters
pub const MAX_MTA_REG: c_int = 128;
    pub mta_shadow: [u32; MAX_MTA_REG],
    pub rar_entry_count: u16,
    pub forced_speed_duplex: u8,
    pub adaptive_ifs: bool,
    pub arc_subsystem_valid: bool,
    pub asf_firmware_present: bool,
    pub autoneg: bool,
    pub autoneg_failed: bool,
    pub disable_hw_init_bits: bool,
    pub get_link_status: bool,
    pub ifs_params_forced: bool,
    pub in_ifs_mode: bool,
    pub report_tx_early: bool,
    pub serdes_has_link: bool,
    pub tx_pkt_filtering: bool,
    pub thermal_sensor_data: e1000_thermal_sensor_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_phy_info {
    pub ops: e1000_phy_operations,
    pub type: e1000_phy_type,
    pub local_rx: e1000_1000t_rx_status,
    pub remote_rx: e1000_1000t_rx_status,
    pub ms_type: e1000_ms_type,
    pub original_ms_type: e1000_ms_type,
    pub cable_polarity: e1000_rev_polarity,
    pub smart_speed: e1000_smart_speed,
    pub addr: u32,
    pub id: u32,
    pub /: *mut *mut u32 reset_delay_us; / in usec,
    pub revision: u32,
    pub media_type: e1000_media_type,
    pub autoneg_advertised: u16,
    pub autoneg_mask: u16,
    pub cable_length: u16,
    pub max_cable_length: u16,
    pub min_cable_length: u16,
    pub pair_length: [u16; 4],
    pub mdix: u8,
    pub disable_polarity_correction: bool,
    pub is_mdix: bool,
    pub polarity_correction: bool,
    pub reset_disable: bool,
    pub speed_downgraded: bool,
    pub autoneg_wait_to_complete: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_nvm_info {
    pub ops: e1000_nvm_operations,
    pub type: e1000_nvm_type,
    pub override: e1000_nvm_override,
    pub flash_bank_size: u32,
    pub flash_base_addr: u32,
    pub word_size: u16,
    pub delay_usec: u16,
    pub address_bits: u16,
    pub opcode_bits: u16,
    pub page_size: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_bus_info {
    pub type: e1000_bus_type,
    pub speed: e1000_bus_speed,
    pub width: e1000_bus_width,
    pub snoop: u32,
    pub func: u16,
    pub pci_cmd_word: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_fc_info {
    pub /: *mut *mut u32 high_water; / Flow control high-water mark,
    pub /: *mut *mut u32 low_water; / Flow control low-water mark,
    pub /: *mut *mut u16 pause_time; / Flow control pause timer,
    pub /: *mut *mut bool send_xon; / Flow control send XON,
    pub /: *mut *mut bool strict_ieee; / Strict IEEE mode,
    pub /: *mut *mut e1000_fc_mode current_mode; / Type of flow control,
    pub requested_mode: e1000_fc_mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_mbx_operations {
    pub hw): *mut *mut s32 (init_params)(struct e1000_hw,
    pub unlock): bool,
    pub mbx_id): *mut *mut *mut *mut s32 (write)(struct e1000_hw hw, u32 msg, u16 size, u16,
    pub mbx_id): *mut *mut *mut *mut s32 (read_posted)(struct e1000_hw hw, u32 msg, u16 size, u16,
    pub mbx_id): u16,
    pub mbx_id): *mut *mut *mut s32 (check_for_msg)(struct e1000_hw hw, u16,
    pub mbx_id): *mut *mut *mut s32 (check_for_ack)(struct e1000_hw hw, u16,
    pub mbx_id): *mut *mut *mut s32 (check_for_rst)(struct e1000_hw hw, u16,
    pub mbx_id): *mut *mut *mut s32 (unlock)(struct e1000_hw hw, u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_mbx_stats {
    pub msgs_tx: u32,
    pub msgs_rx: u32,
    pub acks: u32,
    pub reqs: u32,
    pub rsts: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_mbx_info {
    pub ops: e1000_mbx_operations,
    pub stats: e1000_mbx_stats,
    pub timeout: u32,
    pub usec_delay: u32,
    pub size: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_dev_spec_82575 {
    pub sgmii_active: bool,
    pub global_device_reset: bool,
    pub eee_disable: bool,
    pub clear_semaphore_once: bool,
    pub eth_flags: e1000_sfp_flags,
    pub module_plugged: bool,
    pub media_port: u8,
    pub media_changed: bool,
    pub mas_capable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_hw {
    pub back: *mut c_void,
    pub hw_addr: *mut u8 __iomem,
    pub flash_address: *mut u8 __iomem,
    pub io_base: c_ulong,
    pub mac: e1000_mac_info,
    pub fc: e1000_fc_info,
    pub phy: e1000_phy_info,
    pub nvm: e1000_nvm_info,
    pub bus: e1000_bus_info,
    pub mbx: e1000_mbx_info,
    pub mng_cookie: e1000_host_mng_dhcp_cookie,
    pub _82575: e1000_dev_spec_82575,
    pub dev_spec: },
    pub device_id: u16,
    pub subsystem_vendor_id: u16,
    pub subsystem_device_id: u16,
    pub vendor_id: u16,
    pub revision_id: u8,
}

// These functions must be implemented by drivers
extern "C" {
    pub fn igb_read_pcie_cap_reg(hw: *mut e1000_hw, reg: u32, value: *mut u16) -> i32;
}
extern "C" {
    pub fn igb_write_pcie_cap_reg(hw: *mut e1000_hw, reg: u32, value: *mut u16) -> i32;
}
extern "C" {
    pub fn igb_read_pci_cfg(hw: *mut e1000_hw, reg: u32, value: *mut u16);
}
extern "C" {
    pub fn igb_write_pci_cfg(hw: *mut e1000_hw, reg: u32, value: *mut u16);
}
