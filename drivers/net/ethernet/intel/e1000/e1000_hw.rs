//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/e1000/e1000_hw.h
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
// Copyright(c) 1999 - 2006 Intel Corporation.
// e1000_hw.h
// Structures, enums, and macros for the MAC
//

// Forward declarations of structures used by the shared code
// Enumerated types specific to the e1000 hardware
// Media Access Controllers
// Media Types
// Flow Control Settings
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_shadow_ram {
    pub eeprom_word: u16,
    pub modified: bool,
}

// PCI bus types
// PCI bus speeds
// PCI bus widths
// PHY status info structure and supporting enums
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_phy_info {
    pub cable_length: e1000_cable_length,
    pub extended_10bt_distance: e1000_10bt_ext_dist_enable,
    pub cable_polarity: e1000_rev_polarity,
    pub downshift: e1000_downshift,
    pub polarity_correction: e1000_polarity_reversal,
    pub mdix_mode: e1000_auto_x_mode,
    pub local_rx: e1000_1000t_rx_status,
    pub remote_rx: e1000_1000t_rx_status,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_phy_stats {
    pub idle_errors: u32,
    pub receive_errors: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_eeprom_info {
    pub type: e1000_eeprom_type,
    pub word_size: u16,
    pub opcode_bits: u16,
    pub address_bits: u16,
    pub delay_usec: u16,
    pub page_size: u16,
}

// Flex ASF Information
pub const E1000_HOST_IF_MAX_SIZE: c_int = 2048;
// Error Codes
pub const E1000_SUCCESS: c_int = 0;
pub const E1000_ERR_EEPROM: c_int = 1;
pub const E1000_ERR_PHY: c_int = 2;
pub const E1000_ERR_CONFIG: c_int = 3;
pub const E1000_ERR_PARAM: c_int = 4;
pub const E1000_ERR_MAC_TYPE: c_int = 5;
pub const E1000_ERR_PHY_TYPE: c_int = 6;
pub const E1000_ERR_RESET: c_int = 9;
pub const E1000_ERR_MASTER_REQUESTS_PENDING: c_int = 10;
pub const E1000_ERR_HOST_INTERFACE_COMMAND: c_int = 11;
pub const E1000_BLK_PHY_RESET: c_int = 12;

// Function prototypes
// Initialization
extern "C" {
    pub fn e1000_reset_hw(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000_init_hw(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000_set_mac_type(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000_set_media_type(hw: *mut e1000_hw);
}
// Link Configuration
extern "C" {
    pub fn e1000_setup_link(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000_phy_setup_autoneg(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000_config_collision_dist(hw: *mut e1000_hw);
}
extern "C" {
    pub fn e1000_check_for_link(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000_get_speed_and_duplex(hw: *mut e1000_hw, speed: *mut *mut u16, duplex: *mut *mut u16) -> i32;
}
extern "C" {
    pub fn e1000_force_mac_fc(hw: *mut e1000_hw) -> i32;
}
// PHY
extern "C" {
    pub fn e1000_read_phy_reg(hw: *mut e1000_hw, reg_addr: u32, phy_data: *mut *mut u16) -> i32;
}
extern "C" {
    pub fn e1000_write_phy_reg(hw: *mut e1000_hw, reg_addr: u32, data: u16) -> i32;
}
extern "C" {
    pub fn e1000_phy_hw_reset(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000_phy_reset(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000_phy_get_info(hw: *mut e1000_hw, phy_info: *mut e1000_phy_info) -> i32;
}
extern "C" {
    pub fn e1000_validate_mdi_setting(hw: *mut e1000_hw) -> i32;
}
// EEPROM Functions
extern "C" {
    pub fn e1000_init_eeprom_params(hw: *mut e1000_hw) -> i32;
}
// MNG HOST IF functions
extern "C" {
    pub fn e1000_enable_mng_pass_thru(hw: *mut e1000_hw) -> u32;
}
pub const E1000_MNG_DHCP_TX_PAYLOAD_CMD: c_int = 64;
pub const E1000_HI_MAX_MNG_DATA_LENGTH: c_uint = 0x6F8	/* Host Interface data length */;

pub const E1000_MNG_DHCP_COOKIE_OFFSET: c_uint = 0x6F0	/* Cookie offset */;
pub const E1000_MNG_DHCP_COOKIE_LENGTH: c_uint = 0x10	/* Cookie length */;
pub const E1000_MNG_IAMT_MODE: c_uint = 0x3;
pub const E1000_MNG_ICH_IAMT_MODE: c_uint = 0x2;
pub const E1000_IAMT_SIGNATURE: c_uint = 0x544D4149	/* Intel(R) Active Management Technology signature */;
pub const E1000_MNG_DHCP_COOKIE_STATUS_PARSING_SUPPORT: c_uint = 0x1	/* DHCP parsing enabled */;
pub const E1000_MNG_DHCP_COOKIE_STATUS_VLAN_SUPPORT: c_uint = 0x2	/* DHCP parsing enabled */;
pub const E1000_VFTA_ENTRY_SHIFT: c_uint = 0x5;
pub const E1000_VFTA_ENTRY_MASK: c_uint = 0x7F;
pub const E1000_VFTA_ENTRY_BIT_SHIFT_MASK: c_uint = 0x1F;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_host_mng_command_header {
    pub command_id: u8,
    pub checksum: u8,
    pub reserved1: u16,
    pub reserved2: u16,
    pub command_length: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_host_mng_command_info {
    pub /: *mut *mut e1000_host_mng_command_header command_header; / Command Head/Command Result Head has 4 bytes,
    pub /: *mut *mut u8 command_data[E1000_HI_MAX_MNG_DATA_LENGTH]; / Command data can length 0..0x658,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_host_mng_dhcp_cookie {
    pub signature: u32,
    pub vlan_id: u16,
    pub reserved0: u8,
    pub status: u8,
    pub reserved1: u32,
    pub checksum: u8,
    pub reserved3: u8,
    pub reserved2: u16,
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

extern "C" {
    pub fn e1000_read_eeprom(hw: *mut e1000_hw, reg: u16, words: u16, data: *mut *mut u16) -> i32;
}
extern "C" {
    pub fn e1000_validate_eeprom_checksum(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000_update_eeprom_checksum(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000_write_eeprom(hw: *mut e1000_hw, reg: u16, words: u16, data: *mut *mut u16) -> i32;
}
extern "C" {
    pub fn e1000_read_mac_addr(hw: *mut e1000_hw) -> i32;
}
// Filters (multicast, vlan, receive)
extern "C" {
    pub fn e1000_hash_mc_addr(hw: *mut e1000_hw, mc_addr: *mut *mut u8) -> u32;
}
extern "C" {
    pub fn e1000_rar_set(hw: *mut e1000_hw, mc_addr: *mut *mut u8, rar_index: u32);
}
extern "C" {
    pub fn e1000_write_vfta(hw: *mut e1000_hw, offset: u32, value: u32);
}
// LED functions
extern "C" {
    pub fn e1000_setup_led(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000_cleanup_led(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000_led_on(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000_led_off(hw: *mut e1000_hw) -> i32;
}
// Adaptive IFS Functions
// Everything else
extern "C" {
    pub fn e1000_reset_adaptive(hw: *mut e1000_hw);
}
extern "C" {
    pub fn e1000_update_adaptive(hw: *mut e1000_hw);
}
extern "C" {
    pub fn e1000_get_bus_info(hw: *mut e1000_hw);
}
extern "C" {
    pub fn e1000_pci_set_mwi(hw: *mut e1000_hw);
}
extern "C" {
    pub fn e1000_pci_clear_mwi(hw: *mut e1000_hw);
}
extern "C" {
    pub fn e1000_pcix_set_mmrbc(hw: *mut e1000_hw, mmrbc: c_int);
}
extern "C" {
    pub fn e1000_pcix_get_mmrbc(hw: *mut e1000_hw) -> c_int;
}
// Port I/O is only supported on 82544 and newer
extern "C" {
    pub fn e1000_io_write(hw: *mut e1000_hw, port: c_ulong, value: u32);
}

// PCI Device IDs
pub const E1000_DEV_ID_82542: c_uint = 0x1000;
pub const E1000_DEV_ID_82543GC_FIBER: c_uint = 0x1001;
pub const E1000_DEV_ID_82543GC_COPPER: c_uint = 0x1004;
pub const E1000_DEV_ID_82544EI_COPPER: c_uint = 0x1008;
pub const E1000_DEV_ID_82544EI_FIBER: c_uint = 0x1009;
pub const E1000_DEV_ID_82544GC_COPPER: c_uint = 0x100C;
pub const E1000_DEV_ID_82544GC_LOM: c_uint = 0x100D;
pub const E1000_DEV_ID_82540EM: c_uint = 0x100E;
pub const E1000_DEV_ID_82540EM_LOM: c_uint = 0x1015;
pub const E1000_DEV_ID_82540EP_LOM: c_uint = 0x1016;
pub const E1000_DEV_ID_82540EP: c_uint = 0x1017;
pub const E1000_DEV_ID_82540EP_LP: c_uint = 0x101E;
pub const E1000_DEV_ID_82545EM_COPPER: c_uint = 0x100F;
pub const E1000_DEV_ID_82545EM_FIBER: c_uint = 0x1011;
pub const E1000_DEV_ID_82545GM_COPPER: c_uint = 0x1026;
pub const E1000_DEV_ID_82545GM_FIBER: c_uint = 0x1027;
pub const E1000_DEV_ID_82545GM_SERDES: c_uint = 0x1028;
pub const E1000_DEV_ID_82546EB_COPPER: c_uint = 0x1010;
pub const E1000_DEV_ID_82546EB_FIBER: c_uint = 0x1012;
pub const E1000_DEV_ID_82546EB_QUAD_COPPER: c_uint = 0x101D;
pub const E1000_DEV_ID_82541EI: c_uint = 0x1013;
pub const E1000_DEV_ID_82541EI_MOBILE: c_uint = 0x1018;
pub const E1000_DEV_ID_82541ER_LOM: c_uint = 0x1014;
pub const E1000_DEV_ID_82541ER: c_uint = 0x1078;
pub const E1000_DEV_ID_82547GI: c_uint = 0x1075;
pub const E1000_DEV_ID_82541GI: c_uint = 0x1076;
pub const E1000_DEV_ID_82541GI_MOBILE: c_uint = 0x1077;
pub const E1000_DEV_ID_82541GI_LF: c_uint = 0x107C;
pub const E1000_DEV_ID_82546GB_COPPER: c_uint = 0x1079;
pub const E1000_DEV_ID_82546GB_FIBER: c_uint = 0x107A;
pub const E1000_DEV_ID_82546GB_SERDES: c_uint = 0x107B;
pub const E1000_DEV_ID_82546GB_PCIE: c_uint = 0x108A;
pub const E1000_DEV_ID_82546GB_QUAD_COPPER: c_uint = 0x1099;
pub const E1000_DEV_ID_82547EI: c_uint = 0x1019;
pub const E1000_DEV_ID_82547EI_MOBILE: c_uint = 0x101A;
pub const E1000_DEV_ID_82546GB_QUAD_COPPER_KSP3: c_uint = 0x10B5;
pub const E1000_DEV_ID_INTEL_CE4100_GBE: c_uint = 0x2E6E;
pub const NODE_ADDRESS_SIZE: c_int = 6;
// MAC decode size is 128K - This is the size of BAR0

pub const E1000_82542_2_0_REV_ID: c_int = 2;
pub const E1000_82542_2_1_REV_ID: c_int = 3;
pub const E1000_REVISION_0: c_int = 0;
pub const E1000_REVISION_1: c_int = 1;
pub const E1000_REVISION_2: c_int = 2;
pub const E1000_REVISION_3: c_int = 3;
pub const SPEED_10: c_int = 10;
pub const SPEED_100: c_int = 100;
pub const SPEED_1000: c_int = 1000;
pub const HALF_DUPLEX: c_int = 1;
pub const FULL_DUPLEX: c_int = 2;
// The sizes (in bytes) of a ethernet packet
pub const ENET_HEADER_SIZE: c_int = 14;

pub const ETHERNET_FCS_SIZE: c_int = 4;

pub const MAX_JUMBO_FRAME_SIZE: c_uint = 0x3F00;
// 802.1q VLAN Packet Sizes

// Ethertype field values
pub const ETHERNET_IEEE_VLAN_TYPE: c_uint = 0x8100	/* 802.3ac packet */;
pub const ETHERNET_IP_TYPE: c_uint = 0x0800	/* IP packets */;
pub const ETHERNET_ARP_TYPE: c_uint = 0x0806	/* Address Resolution Protocol (ARP) */;
// Packet Header defines
pub const IP_PROTOCOL_TCP: c_int = 6;
pub const IP_PROTOCOL_UDP: c_uint = 0x11;
// This defines the bits that are set in the Interrupt Mask
// Set/Read Register.  Each bit is documented below:
// o RXDMT0 = Receive Descriptor Minimum Threshold hit (ring 0)
// o RXSEQ  = Receive Sequence Error
//

// This defines the bits that are set in the Interrupt Mask
// Set/Read Register.  Each bit is documented below:
// o RXT0   = Receiver Timer Interrupt (ring 0)
// o TXDW   = Transmit Descriptor Written Back
// o RXDMT0 = Receive Descriptor Minimum Threshold hit (ring 0)
// o RXSEQ  = Receive Sequence Error
// o LSC    = Link Status Change
//

// Number of high/low register pairs in the RAR. The RAR (Receive Address
// Registers) holds the directed and multicast addresses that we monitor. We
// reserve one of these spots for our directed address, allowing us room for
// E1000_RAR_ENTRIES - 1 multicast addresses.
//
pub const E1000_RAR_ENTRIES: c_int = 15;
pub const MIN_NUMBER_OF_DESCRIPTORS: c_int = 8;
pub const MAX_NUMBER_OF_DESCRIPTORS: c_uint = 0xFFF8;
// Receive Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_rx_desc {
    pub /: *mut *mut __le64 buffer_addr; / Address of the descriptor's data buffer,
    pub /: *mut *mut __le16 length; / Length of data DMAed into data buffer,
    pub /: *mut *mut __le16 csum; / Packet checksum,
    pub /: *mut *mut u8 status; / Descriptor status,
    pub /: *mut *mut u8 errors; / Descriptor Errors,
    pub special: __le16,
}

// Receive Descriptor - Extended
#[repr(C)]
#[derive(Copy, Clone)]
pub union e1000_rx_desc_extended {
    pub buffer_addr: __le64,
    pub reserved: __le64,
    pub read: },
    pub /: *mut *mut __le32 mrq; / Multiple Rx Queues,
    pub /: *mut *mut __le32 rss; / RSS Hash,
    pub /: *mut *mut __le16 ip_id; / IP id,
    pub /: *mut *mut __le16 csum; / Packet Checksum,
    pub csum_ip: },
    pub hi_dword: },
    pub lower: },
    pub /: *mut *mut __le32 status_error; / ext status/error,
    pub length: __le16,
    pub /: *mut *mut __le16 vlan; / VLAN tag,
    pub upper: },
    pub /: *mut *mut } wb; / writeback,
}

pub const MAX_PS_BUFFERS: c_int = 4;
// Receive Descriptor - Packet Split
#[repr(C)]
#[derive(Copy, Clone)]
pub union e1000_rx_desc_packet_split {
// one buffer for protocol header(s), three data buffers
    pub buffer_addr: [__le64; MAX_PS_BUFFERS],
    pub read: },
    pub /: *mut *mut __le32 mrq; / Multiple Rx Queues,
    pub /: *mut *mut __le32 rss; / RSS Hash,
    pub /: *mut *mut __le16 ip_id; / IP id,
    pub /: *mut *mut __le16 csum; / Packet Checksum,
    pub csum_ip: },
    pub hi_dword: },
    pub lower: },
    pub /: *mut *mut __le32 status_error; / ext status/error,
    pub /: *mut *mut __le16 length0; / length of buffer 0,
    pub /: *mut *mut __le16 vlan; / VLAN tag,
    pub middle: },
    pub header_status: __le16,
    pub /: *mut *mut __le16 length[3]; / length of buffers 1-3,
    pub upper: },
    pub reserved: __le64,
    pub /: *mut *mut } wb; / writeback,
}

// Receive Descriptor bit definitions
pub const E1000_RXD_STAT_DD: c_uint = 0x01	/* Descriptor Done */;
pub const E1000_RXD_STAT_EOP: c_uint = 0x02	/* End of Packet */;
pub const E1000_RXD_STAT_IXSM: c_uint = 0x04	/* Ignore checksum */;
pub const E1000_RXD_STAT_VP: c_uint = 0x08	/* IEEE VLAN Packet */;
pub const E1000_RXD_STAT_UDPCS: c_uint = 0x10	/* UDP xsum calculated */;
pub const E1000_RXD_STAT_TCPCS: c_uint = 0x20	/* TCP xsum calculated */;
pub const E1000_RXD_STAT_IPCS: c_uint = 0x40	/* IP xsum calculated */;
pub const E1000_RXD_STAT_PIF: c_uint = 0x80	/* passed in-exact filter */;
pub const E1000_RXD_STAT_IPIDV: c_uint = 0x200	/* IP identification valid */;
pub const E1000_RXD_STAT_UDPV: c_uint = 0x400	/* Valid UDP checksum */;
pub const E1000_RXD_STAT_ACK: c_uint = 0x8000	/* ACK Packet indication */;
pub const E1000_RXD_ERR_CE: c_uint = 0x01	/* CRC Error */;
pub const E1000_RXD_ERR_SE: c_uint = 0x02	/* Symbol Error */;
pub const E1000_RXD_ERR_SEQ: c_uint = 0x04	/* Sequence Error */;
pub const E1000_RXD_ERR_CXE: c_uint = 0x10	/* Carrier Extension Error */;
pub const E1000_RXD_ERR_TCPE: c_uint = 0x20	/* TCP/UDP Checksum Error */;
pub const E1000_RXD_ERR_IPE: c_uint = 0x40	/* IP Checksum Error */;
pub const E1000_RXD_ERR_RXE: c_uint = 0x80	/* Rx Data Error */;
pub const E1000_RXD_SPC_VLAN_MASK: c_uint = 0x0FFF	/* VLAN ID is in lower 12 bits */;
pub const E1000_RXD_SPC_PRI_MASK: c_uint = 0xE000	/* Priority is in upper 3 bits */;
pub const E1000_RXD_SPC_PRI_SHIFT: c_int = 13;
pub const E1000_RXD_SPC_CFI_MASK: c_uint = 0x1000	/* CFI is bit 12 */;
pub const E1000_RXD_SPC_CFI_SHIFT: c_int = 12;
pub const E1000_RXDEXT_STATERR_CE: c_uint = 0x01000000;
pub const E1000_RXDEXT_STATERR_SE: c_uint = 0x02000000;
pub const E1000_RXDEXT_STATERR_SEQ: c_uint = 0x04000000;
pub const E1000_RXDEXT_STATERR_CXE: c_uint = 0x10000000;
pub const E1000_RXDEXT_STATERR_TCPE: c_uint = 0x20000000;
pub const E1000_RXDEXT_STATERR_IPE: c_uint = 0x40000000;
pub const E1000_RXDEXT_STATERR_RXE: c_uint = 0x80000000;
pub const E1000_RXDPS_HDRSTAT_HDRSP: c_uint = 0x00008000;
pub const E1000_RXDPS_HDRSTAT_HDRLEN_MASK: c_uint = 0x000003FF;
// mask to determine if packets should be dropped due to frame errors

// Same mask, but for extended and packet split descriptors

// Transmit Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_tx_desc {
    pub /: *mut *mut __le64 buffer_addr; / Address of the descriptor's data buffer,
    pub data: __le32,
    pub /: *mut *mut __le16 length; / Data buffer length,
    pub /: *mut *mut u8 cso; / Checksum offset,
    pub /: *mut *mut u8 cmd; / Descriptor control,
    pub flags: },
    pub lower: },
    pub data: __le32,
    pub /: *mut *mut u8 status; / Descriptor status,
    pub /: *mut *mut u8 css; / Checksum start,
    pub special: __le16,
    pub fields: },
    pub upper: },
}

// Transmit Descriptor bit definitions
pub const E1000_TXD_DTYP_D: c_uint = 0x00100000	/* Data Descriptor */;
pub const E1000_TXD_DTYP_C: c_uint = 0x00000000	/* Context Descriptor */;
pub const E1000_TXD_POPTS_IXSM: c_uint = 0x01	/* Insert IP checksum */;
pub const E1000_TXD_POPTS_TXSM: c_uint = 0x02	/* Insert TCP/UDP checksum */;
pub const E1000_TXD_CMD_EOP: c_uint = 0x01000000	/* End of Packet */;
pub const E1000_TXD_CMD_IFCS: c_uint = 0x02000000	/* Insert FCS (Ethernet CRC) */;
pub const E1000_TXD_CMD_IC: c_uint = 0x04000000	/* Insert Checksum */;
pub const E1000_TXD_CMD_RS: c_uint = 0x08000000	/* Report Status */;
pub const E1000_TXD_CMD_RPS: c_uint = 0x10000000	/* Report Packet Sent */;
pub const E1000_TXD_CMD_DEXT: c_uint = 0x20000000	/* Descriptor extension (0 = legacy) */;
pub const E1000_TXD_CMD_VLE: c_uint = 0x40000000	/* Add VLAN tag */;
pub const E1000_TXD_CMD_IDE: c_uint = 0x80000000	/* Enable Tidv register */;
pub const E1000_TXD_STAT_DD: c_uint = 0x00000001	/* Descriptor Done */;
pub const E1000_TXD_STAT_EC: c_uint = 0x00000002	/* Excess Collisions */;
pub const E1000_TXD_STAT_LC: c_uint = 0x00000004	/* Late Collisions */;
pub const E1000_TXD_STAT_TU: c_uint = 0x00000008	/* Transmit underrun */;
pub const E1000_TXD_CMD_TCP: c_uint = 0x01000000	/* TCP packet */;
pub const E1000_TXD_CMD_IP: c_uint = 0x02000000	/* IP packet */;
pub const E1000_TXD_CMD_TSE: c_uint = 0x04000000	/* TCP Seg enable */;
pub const E1000_TXD_STAT_TC: c_uint = 0x00000004	/* Tx Underrun */;
// Offload Context Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_context_desc {
    pub ip_config: __le32,
    pub /: *mut *mut u8 ipcss; / IP checksum start,
    pub /: *mut *mut u8 ipcso; / IP checksum offset,
    pub /: *mut *mut __le16 ipcse; / IP checksum end,
    pub ip_fields: },
    pub lower_setup: },
    pub tcp_config: __le32,
    pub /: *mut *mut u8 tucss; / TCP checksum start,
    pub /: *mut *mut u8 tucso; / TCP checksum offset,
    pub /: *mut *mut __le16 tucse; / TCP checksum end,
    pub tcp_fields: },
    pub upper_setup: },
    pub /: *mut *mut __le32 cmd_and_length; /,
    pub data: __le32,
    pub /: *mut *mut u8 status; / Descriptor status,
    pub /: *mut *mut u8 hdr_len; / Header length,
    pub /: *mut *mut __le16 mss; / Maximum segment size,
    pub fields: },
    pub tcp_seg_setup: },
}

// Offload data descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_data_desc {
    pub /: *mut *mut __le64 buffer_addr; / Address of the descriptor's buffer address,
    pub data: __le32,
    pub /: *mut *mut __le16 length; / Data buffer length,
    pub /: *mut *mut u8 typ_len_ext; /,
    pub /: *mut *mut u8 cmd; /,
    pub flags: },
    pub lower: },
    pub data: __le32,
    pub /: *mut *mut u8 status; / Descriptor status,
    pub /: *mut *mut u8 popts; / Packet Options,
    pub /: *mut *mut __le16 special; /,
    pub fields: },
    pub upper: },
}

// Filters

// Receive Address Register
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_rar {
    pub /: *mut *mut volatile __le32 low; / receive address low,
    pub /: *mut *mut volatile __le32 high; / receive address high,
}

// Number of entries in the Multicast Table Array (MTA).
pub const E1000_NUM_MTA_REGISTERS: c_int = 128;
// IPv4 Address Table Entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_ipv4_at_entry {
    pub /: *mut *mut volatile u32 ipv4_addr; / IP Address (RW),
    pub reserved: volatile u32,
}

// Four wakeup IP addresses are supported
pub const E1000_WAKEUP_IP_ADDRESS_COUNT_MAX: c_int = 4;

pub const E1000_IP6AT_SIZE: c_int = 1;
// IPv6 Address Table Entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_ipv6_at_entry {
    pub ipv6_addr: [volatile u8; 16],
}

// Flexible Filter Length Table Entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_fflt_entry {
    pub /: *mut *mut volatile u32 length; / Flexible Filter Length (RW),
    pub reserved: volatile u32,
}

// Flexible Filter Mask Table Entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_ffmt_entry {
    pub /: *mut *mut volatile u32 mask; / Flexible Filter Mask (RW),
    pub reserved: volatile u32,
}

// Flexible Filter Value Table Entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_ffvt_entry {
    pub /: *mut *mut volatile u32 value; / Flexible Filter Value (RW),
    pub reserved: volatile u32,
}

// Four Flexible Filters are supported
pub const E1000_FLEXIBLE_FILTER_COUNT_MAX: c_int = 4;
// Each Flexible Filter is at most 128 (0x80) bytes in length
pub const E1000_FLEXIBLE_FILTER_SIZE_MAX: c_int = 128;

pub const E1000_DISABLE_SERDES_LOOPBACK: c_uint = 0x0400;
// Register Set. (82543, 82544)
//
// Registers are defined to be 32 bits and  should be accessed as 32 bit values.
// These registers are physically located on the NIC, but are mapped into the
// host memory address space.
//
// RW - register is both readable and writable
// RO - register is read only
// WO - register is write only
// R/clr - register is read only and is cleared when read
// A - register array
//
pub const E1000_CTRL: c_uint = 0x00000	/* Device Control - RW */;
pub const E1000_CTRL_DUP: c_uint = 0x00004	/* Device Control Duplicate (Shadow) - RW */;
pub const E1000_STATUS: c_uint = 0x00008	/* Device Status - RO */;
pub const E1000_EECD: c_uint = 0x00010	/* EEPROM/Flash Control - RW */;
pub const E1000_EERD: c_uint = 0x00014	/* EEPROM Read - RW */;
pub const E1000_CTRL_EXT: c_uint = 0x00018	/* Extended Device Control - RW */;
pub const E1000_FLA: c_uint = 0x0001C	/* Flash Access - RW */;
pub const E1000_MDIC: c_uint = 0x00020	/* MDI Control - RW */;

pub const E1000_SCTL: c_uint = 0x00024	/* SerDes Control - RW */;
pub const E1000_FEXTNVM: c_uint = 0x00028	/* Future Extended NVM register */;
pub const E1000_FCAL: c_uint = 0x00028	/* Flow Control Address Low - RW */;
pub const E1000_FCAH: c_uint = 0x0002C	/* Flow Control Address High -RW */;
pub const E1000_FCT: c_uint = 0x00030	/* Flow Control Type - RW */;
pub const E1000_VET: c_uint = 0x00038	/* VLAN Ether Type - RW */;
pub const E1000_ICR: c_uint = 0x000C0	/* Interrupt Cause Read - R/clr */;
pub const E1000_ITR: c_uint = 0x000C4	/* Interrupt Throttling Rate - RW */;
pub const E1000_ICS: c_uint = 0x000C8	/* Interrupt Cause Set - WO */;
pub const E1000_IMS: c_uint = 0x000D0	/* Interrupt Mask Set - RW */;
pub const E1000_IMC: c_uint = 0x000D8	/* Interrupt Mask Clear - WO */;
pub const E1000_IAM: c_uint = 0x000E0	/* Interrupt Acknowledge Auto Mask */;
// Auxiliary Control Register. This register is CE4100 specific,
// RMII/RGMII function is switched by this register - RW
// Following are bits definitions of the Auxiliary Control Register
//
pub const E1000_CTL_AUX: c_uint = 0x000E0;
pub const E1000_CTL_AUX_END_SEL_SHIFT: c_int = 10;
pub const E1000_CTL_AUX_ENDIANESS_SHIFT: c_int = 8;
pub const E1000_CTL_AUX_RGMII_RMII_SHIFT: c_int = 0;
// descriptor and packet transfer use CTL_AUX.ENDIANESS

// descriptor use CTL_AUX.ENDIANESS, packet use default

// descriptor use default, packet use CTL_AUX.ENDIANESS

// all use CTL_AUX.ENDIANESS

// LW little endian, Byte big endian

pub const E1000_RCTL: c_uint = 0x00100	/* RX Control - RW */;
pub const E1000_RDTR1: c_uint = 0x02820	/* RX Delay Timer (1) - RW */;
pub const E1000_RDBAL1: c_uint = 0x02900	/* RX Descriptor Base Address Low (1) - RW */;
pub const E1000_RDBAH1: c_uint = 0x02904	/* RX Descriptor Base Address High (1) - RW */;
pub const E1000_RDLEN1: c_uint = 0x02908	/* RX Descriptor Length (1) - RW */;
pub const E1000_RDH1: c_uint = 0x02910	/* RX Descriptor Head (1) - RW */;
pub const E1000_RDT1: c_uint = 0x02918	/* RX Descriptor Tail (1) - RW */;
pub const E1000_FCTTV: c_uint = 0x00170	/* Flow Control Transmit Timer Value - RW */;
pub const E1000_TXCW: c_uint = 0x00178	/* TX Configuration Word - RW */;
pub const E1000_RXCW: c_uint = 0x00180	/* RX Configuration Word - RO */;
pub const E1000_TCTL: c_uint = 0x00400	/* TX Control - RW */;
pub const E1000_TCTL_EXT: c_uint = 0x00404	/* Extended TX Control - RW */;
pub const E1000_TIPG: c_uint = 0x00410	/* TX Inter-packet gap -RW */;
pub const E1000_TBT: c_uint = 0x00448	/* TX Burst Timer - RW */;
pub const E1000_AIT: c_uint = 0x00458	/* Adaptive Interframe Spacing Throttle - RW */;
pub const E1000_LEDCTL: c_uint = 0x00E00	/* LED Control - RW */;
pub const E1000_EXTCNF_CTRL: c_uint = 0x00F00	/* Extended Configuration Control */;
pub const E1000_EXTCNF_SIZE: c_uint = 0x00F08	/* Extended Configuration Size */;
pub const E1000_PHY_CTRL: c_uint = 0x00F10	/* PHY Control Register in CSR */;
pub const FEXTNVM_SW_CONFIG: c_uint = 0x0001;
pub const E1000_PBA: c_uint = 0x01000	/* Packet Buffer Allocation - RW */;
pub const E1000_PBS: c_uint = 0x01008	/* Packet Buffer Size */;
pub const E1000_EEMNGCTL: c_uint = 0x01010	/* MNG EEprom Control */;
pub const E1000_FLASH_UPDATES: c_int = 1000;
pub const E1000_EEARBC: c_uint = 0x01024	/* EEPROM Auto Read Bus Control */;
pub const E1000_FLASHT: c_uint = 0x01028	/* FLASH Timer Register */;
pub const E1000_EEWR: c_uint = 0x0102C	/* EEPROM Write Register - RW */;
pub const E1000_FLSWCTL: c_uint = 0x01030	/* FLASH control register */;
pub const E1000_FLSWDATA: c_uint = 0x01034	/* FLASH data register */;
pub const E1000_FLSWCNT: c_uint = 0x01038	/* FLASH Access Counter */;
pub const E1000_FLOP: c_uint = 0x0103C	/* FLASH Opcode Register */;
pub const E1000_ERT: c_uint = 0x02008	/* Early Rx Threshold - RW */;
pub const E1000_FCRTL: c_uint = 0x02160	/* Flow Control Receive Threshold Low - RW */;
pub const E1000_FCRTH: c_uint = 0x02168	/* Flow Control Receive Threshold High - RW */;
pub const E1000_PSRCTL: c_uint = 0x02170	/* Packet Split Receive Control - RW */;
pub const E1000_RDFH: c_uint = 0x02410  /* RX Data FIFO Head - RW */;
pub const E1000_RDFT: c_uint = 0x02418  /* RX Data FIFO Tail - RW */;
pub const E1000_RDFHS: c_uint = 0x02420  /* RX Data FIFO Head Saved - RW */;
pub const E1000_RDFTS: c_uint = 0x02428  /* RX Data FIFO Tail Saved - RW */;
pub const E1000_RDFPC: c_uint = 0x02430  /* RX Data FIFO Packet Count - RW */;
pub const E1000_RDBAL: c_uint = 0x02800	/* RX Descriptor Base Address Low - RW */;
pub const E1000_RDBAH: c_uint = 0x02804	/* RX Descriptor Base Address High - RW */;
pub const E1000_RDLEN: c_uint = 0x02808	/* RX Descriptor Length - RW */;
pub const E1000_RDH: c_uint = 0x02810	/* RX Descriptor Head - RW */;
pub const E1000_RDT: c_uint = 0x02818	/* RX Descriptor Tail - RW */;
pub const E1000_RDTR: c_uint = 0x02820	/* RX Delay Timer - RW */;

pub const E1000_RXDCTL: c_uint = 0x02828	/* RX Descriptor Control queue 0 - RW */;
pub const E1000_RXDCTL1: c_uint = 0x02928	/* RX Descriptor Control queue 1 - RW */;
pub const E1000_RADV: c_uint = 0x0282C	/* RX Interrupt Absolute Delay Timer - RW */;
pub const E1000_RSRPD: c_uint = 0x02C00	/* RX Small Packet Detect - RW */;
pub const E1000_RAID: c_uint = 0x02C08	/* Receive Ack Interrupt Delay - RW */;
pub const E1000_TXDMAC: c_uint = 0x03000	/* TX DMA Control - RW */;
pub const E1000_KABGTXD: c_uint = 0x03004	/* AFE Band Gap Transmit Ref Data */;
pub const E1000_TDFH: c_uint = 0x03410	/* TX Data FIFO Head - RW */;
pub const E1000_TDFT: c_uint = 0x03418	/* TX Data FIFO Tail - RW */;
pub const E1000_TDFHS: c_uint = 0x03420	/* TX Data FIFO Head Saved - RW */;
pub const E1000_TDFTS: c_uint = 0x03428	/* TX Data FIFO Tail Saved - RW */;
pub const E1000_TDFPC: c_uint = 0x03430	/* TX Data FIFO Packet Count - RW */;
pub const E1000_TDBAL: c_uint = 0x03800	/* TX Descriptor Base Address Low - RW */;
pub const E1000_TDBAH: c_uint = 0x03804	/* TX Descriptor Base Address High - RW */;
pub const E1000_TDLEN: c_uint = 0x03808	/* TX Descriptor Length - RW */;
pub const E1000_TDH: c_uint = 0x03810	/* TX Descriptor Head - RW */;
pub const E1000_TDT: c_uint = 0x03818	/* TX Descripotr Tail - RW */;
pub const E1000_TIDV: c_uint = 0x03820	/* TX Interrupt Delay Value - RW */;
pub const E1000_TXDCTL: c_uint = 0x03828	/* TX Descriptor Control - RW */;
pub const E1000_TADV: c_uint = 0x0382C	/* TX Interrupt Absolute Delay Val - RW */;
pub const E1000_TSPMT: c_uint = 0x03830	/* TCP Segmentation PAD & Min Threshold - RW */;
pub const E1000_TARC0: c_uint = 0x03840	/* TX Arbitration Count (0) */;
pub const E1000_TDBAL1: c_uint = 0x03900	/* TX Desc Base Address Low (1) - RW */;
pub const E1000_TDBAH1: c_uint = 0x03904	/* TX Desc Base Address High (1) - RW */;
pub const E1000_TDLEN1: c_uint = 0x03908	/* TX Desc Length (1) - RW */;
pub const E1000_TDH1: c_uint = 0x03910	/* TX Desc Head (1) - RW */;
pub const E1000_TDT1: c_uint = 0x03918	/* TX Desc Tail (1) - RW */;
pub const E1000_TXDCTL1: c_uint = 0x03928	/* TX Descriptor Control (1) - RW */;
pub const E1000_TARC1: c_uint = 0x03940	/* TX Arbitration Count (1) */;
pub const E1000_CRCERRS: c_uint = 0x04000	/* CRC Error Count - R/clr */;
pub const E1000_ALGNERRC: c_uint = 0x04004	/* Alignment Error Count - R/clr */;
pub const E1000_SYMERRS: c_uint = 0x04008	/* Symbol Error Count - R/clr */;
pub const E1000_RXERRC: c_uint = 0x0400C	/* Receive Error Count - R/clr */;
pub const E1000_MPC: c_uint = 0x04010	/* Missed Packet Count - R/clr */;
pub const E1000_SCC: c_uint = 0x04014	/* Single Collision Count - R/clr */;
pub const E1000_ECOL: c_uint = 0x04018	/* Excessive Collision Count - R/clr */;
pub const E1000_MCC: c_uint = 0x0401C	/* Multiple Collision Count - R/clr */;
pub const E1000_LATECOL: c_uint = 0x04020	/* Late Collision Count - R/clr */;
pub const E1000_COLC: c_uint = 0x04028	/* Collision Count - R/clr */;
pub const E1000_DC: c_uint = 0x04030	/* Defer Count - R/clr */;
pub const E1000_TNCRS: c_uint = 0x04034	/* TX-No CRS - R/clr */;
pub const E1000_SEC: c_uint = 0x04038	/* Sequence Error Count - R/clr */;
pub const E1000_CEXTERR: c_uint = 0x0403C	/* Carrier Extension Error Count - R/clr */;
pub const E1000_RLEC: c_uint = 0x04040	/* Receive Length Error Count - R/clr */;
pub const E1000_XONRXC: c_uint = 0x04048	/* XON RX Count - R/clr */;
pub const E1000_XONTXC: c_uint = 0x0404C	/* XON TX Count - R/clr */;
pub const E1000_XOFFRXC: c_uint = 0x04050	/* XOFF RX Count - R/clr */;
pub const E1000_XOFFTXC: c_uint = 0x04054	/* XOFF TX Count - R/clr */;
pub const E1000_FCRUC: c_uint = 0x04058	/* Flow Control RX Unsupported Count- R/clr */;
pub const E1000_PRC64: c_uint = 0x0405C	/* Packets RX (64 bytes) - R/clr */;
pub const E1000_PRC127: c_uint = 0x04060	/* Packets RX (65-127 bytes) - R/clr */;
pub const E1000_PRC255: c_uint = 0x04064	/* Packets RX (128-255 bytes) - R/clr */;
pub const E1000_PRC511: c_uint = 0x04068	/* Packets RX (255-511 bytes) - R/clr */;
pub const E1000_PRC1023: c_uint = 0x0406C	/* Packets RX (512-1023 bytes) - R/clr */;
pub const E1000_PRC1522: c_uint = 0x04070	/* Packets RX (1024-1522 bytes) - R/clr */;
pub const E1000_GPRC: c_uint = 0x04074	/* Good Packets RX Count - R/clr */;
pub const E1000_BPRC: c_uint = 0x04078	/* Broadcast Packets RX Count - R/clr */;
pub const E1000_MPRC: c_uint = 0x0407C	/* Multicast Packets RX Count - R/clr */;
pub const E1000_GPTC: c_uint = 0x04080	/* Good Packets TX Count - R/clr */;
pub const E1000_GORCL: c_uint = 0x04088	/* Good Octets RX Count Low - R/clr */;
pub const E1000_GORCH: c_uint = 0x0408C	/* Good Octets RX Count High - R/clr */;
pub const E1000_GOTCL: c_uint = 0x04090	/* Good Octets TX Count Low - R/clr */;
pub const E1000_GOTCH: c_uint = 0x04094	/* Good Octets TX Count High - R/clr */;
pub const E1000_RNBC: c_uint = 0x040A0	/* RX No Buffers Count - R/clr */;
pub const E1000_RUC: c_uint = 0x040A4	/* RX Undersize Count - R/clr */;
pub const E1000_RFC: c_uint = 0x040A8	/* RX Fragment Count - R/clr */;
pub const E1000_ROC: c_uint = 0x040AC	/* RX Oversize Count - R/clr */;
pub const E1000_RJC: c_uint = 0x040B0	/* RX Jabber Count - R/clr */;
pub const E1000_MGTPRC: c_uint = 0x040B4	/* Management Packets RX Count - R/clr */;
pub const E1000_MGTPDC: c_uint = 0x040B8	/* Management Packets Dropped Count - R/clr */;
pub const E1000_MGTPTC: c_uint = 0x040BC	/* Management Packets TX Count - R/clr */;
pub const E1000_TORL: c_uint = 0x040C0	/* Total Octets RX Low - R/clr */;
pub const E1000_TORH: c_uint = 0x040C4	/* Total Octets RX High - R/clr */;
pub const E1000_TOTL: c_uint = 0x040C8	/* Total Octets TX Low - R/clr */;
pub const E1000_TOTH: c_uint = 0x040CC	/* Total Octets TX High - R/clr */;
pub const E1000_TPR: c_uint = 0x040D0	/* Total Packets RX - R/clr */;
pub const E1000_TPT: c_uint = 0x040D4	/* Total Packets TX - R/clr */;
pub const E1000_PTC64: c_uint = 0x040D8	/* Packets TX (64 bytes) - R/clr */;
pub const E1000_PTC127: c_uint = 0x040DC	/* Packets TX (65-127 bytes) - R/clr */;
pub const E1000_PTC255: c_uint = 0x040E0	/* Packets TX (128-255 bytes) - R/clr */;
pub const E1000_PTC511: c_uint = 0x040E4	/* Packets TX (256-511 bytes) - R/clr */;
pub const E1000_PTC1023: c_uint = 0x040E8	/* Packets TX (512-1023 bytes) - R/clr */;
pub const E1000_PTC1522: c_uint = 0x040EC	/* Packets TX (1024-1522 Bytes) - R/clr */;
pub const E1000_MPTC: c_uint = 0x040F0	/* Multicast Packets TX Count - R/clr */;
pub const E1000_BPTC: c_uint = 0x040F4	/* Broadcast Packets TX Count - R/clr */;
pub const E1000_TSCTC: c_uint = 0x040F8	/* TCP Segmentation Context TX - R/clr */;
pub const E1000_TSCTFC: c_uint = 0x040FC	/* TCP Segmentation Context TX Fail - R/clr */;
pub const E1000_IAC: c_uint = 0x04100	/* Interrupt Assertion Count */;
pub const E1000_ICRXPTC: c_uint = 0x04104	/* Interrupt Cause Rx Packet Timer Expire Count */;
pub const E1000_ICRXATC: c_uint = 0x04108	/* Interrupt Cause Rx Absolute Timer Expire Count */;
pub const E1000_ICTXPTC: c_uint = 0x0410C	/* Interrupt Cause Tx Packet Timer Expire Count */;
pub const E1000_ICTXATC: c_uint = 0x04110	/* Interrupt Cause Tx Absolute Timer Expire Count */;
pub const E1000_ICTXQEC: c_uint = 0x04118	/* Interrupt Cause Tx Queue Empty Count */;
pub const E1000_ICTXQMTC: c_uint = 0x0411C	/* Interrupt Cause Tx Queue Minimum Threshold Count */;
pub const E1000_ICRXDMTC: c_uint = 0x04120	/* Interrupt Cause Rx Descriptor Minimum Threshold Count */;
pub const E1000_ICRXOC: c_uint = 0x04124	/* Interrupt Cause Receiver Overrun Count */;
pub const E1000_RXCSUM: c_uint = 0x05000	/* RX Checksum Control - RW */;
pub const E1000_RFCTL: c_uint = 0x05008	/* Receive Filter Control */;
pub const E1000_MTA: c_uint = 0x05200	/* Multicast Table Array - RW Array */;
pub const E1000_RA: c_uint = 0x05400	/* Receive Address - RW Array */;
pub const E1000_VFTA: c_uint = 0x05600	/* VLAN Filter Table Array - RW Array */;
pub const E1000_WUC: c_uint = 0x05800	/* Wakeup Control - RW */;
pub const E1000_WUFC: c_uint = 0x05808	/* Wakeup Filter Control - RW */;
pub const E1000_WUS: c_uint = 0x05810	/* Wakeup Status - RO */;
pub const E1000_MANC: c_uint = 0x05820	/* Management Control - RW */;
pub const E1000_IPAV: c_uint = 0x05838	/* IP Address Valid - RW */;
pub const E1000_IP4AT: c_uint = 0x05840	/* IPv4 Address Table - RW Array */;
pub const E1000_IP6AT: c_uint = 0x05880	/* IPv6 Address Table - RW Array */;
pub const E1000_WUPL: c_uint = 0x05900	/* Wakeup Packet Length - RW */;
pub const E1000_WUPM: c_uint = 0x05A00	/* Wakeup Packet Memory - RO A */;
pub const E1000_FFLT: c_uint = 0x05F00	/* Flexible Filter Length Table - RW Array */;
pub const E1000_HOST_IF: c_uint = 0x08800	/* Host Interface */;
pub const E1000_FFMT: c_uint = 0x09000	/* Flexible Filter Mask Table - RW Array */;
pub const E1000_FFVT: c_uint = 0x09800	/* Flexible Filter Value Table - RW Array */;
pub const E1000_KUMCTRLSTA: c_uint = 0x00034	/* MAC-PHY interface - RW */;
pub const E1000_MDPHYA: c_uint = 0x0003C	/* PHY address - RW */;
pub const E1000_MANC2H: c_uint = 0x05860	/* Management Control To Host - RW */;
pub const E1000_SW_FW_SYNC: c_uint = 0x05B5C	/* Software-Firmware Synchronization - RW */;
pub const E1000_GCR: c_uint = 0x05B00	/* PCI-Ex Control */;
pub const E1000_GSCL_1: c_uint = 0x05B10	/* PCI-Ex Statistic Control #1 */;
pub const E1000_GSCL_2: c_uint = 0x05B14	/* PCI-Ex Statistic Control #2 */;
pub const E1000_GSCL_3: c_uint = 0x05B18	/* PCI-Ex Statistic Control #3 */;
pub const E1000_GSCL_4: c_uint = 0x05B1C	/* PCI-Ex Statistic Control #4 */;
pub const E1000_FACTPS: c_uint = 0x05B30	/* Function Active and Power State to MNG */;
pub const E1000_SWSM: c_uint = 0x05B50	/* SW Semaphore */;
pub const E1000_FWSM: c_uint = 0x05B54	/* FW Semaphore */;
pub const E1000_FFLT_DBG: c_uint = 0x05F04	/* Debug Register */;
pub const E1000_HICR: c_uint = 0x08F00	/* Host Interface Control */;
// RSS registers
pub const E1000_CPUVEC: c_uint = 0x02C10	/* CPU Vector Register - RW */;
pub const E1000_MRQC: c_uint = 0x05818	/* Multiple Receive Control - RW */;
pub const E1000_RETA: c_uint = 0x05C00	/* Redirection Table - RW Array */;
pub const E1000_RSSRK: c_uint = 0x05C80	/* RSS Random Key - RW Array */;
pub const E1000_RSSIM: c_uint = 0x05864	/* RSS Interrupt Mask */;
pub const E1000_RSSIR: c_uint = 0x05868	/* RSS Interrupt Request */;
// Register Set (82542)
//
// Some of the 82542 registers are located at different offsets than they are
// in more current versions of the 8254x. Despite the difference in location,
// the registers function in the same manner.
//

pub const E1000_82542_RA: c_uint = 0x00040;

pub const E1000_82542_RDTR: c_uint = 0x00108;

pub const E1000_82542_RDBAL: c_uint = 0x00110;
pub const E1000_82542_RDBAH: c_uint = 0x00114;
pub const E1000_82542_RDLEN: c_uint = 0x00118;
pub const E1000_82542_RDH: c_uint = 0x00120;
pub const E1000_82542_RDT: c_uint = 0x00128;

// RX Control - RW

pub const E1000_82542_RDBAH3: c_uint = 0x02B04	/* RX Desc Base High Queue 3 - RW */;
pub const E1000_82542_RDBAL3: c_uint = 0x02B00	/* RX Desc Low Queue 3 - RW */;
pub const E1000_82542_RDLEN3: c_uint = 0x02B08	/* RX Desc Length Queue 3 - RW */;
pub const E1000_82542_RDH3: c_uint = 0x02B10	/* RX Desc Head Queue 3 - RW */;
pub const E1000_82542_RDT3: c_uint = 0x02B18	/* RX Desc Tail Queue 3 - RW */;
pub const E1000_82542_RDBAL2: c_uint = 0x02A00	/* RX Desc Base Low Queue 2 - RW */;
pub const E1000_82542_RDBAH2: c_uint = 0x02A04	/* RX Desc Base High Queue 2 - RW */;
pub const E1000_82542_RDLEN2: c_uint = 0x02A08	/* RX Desc Length Queue 2 - RW */;
pub const E1000_82542_RDH2: c_uint = 0x02A10	/* RX Desc Head Queue 2 - RW */;
pub const E1000_82542_RDT2: c_uint = 0x02A18	/* RX Desc Tail Queue 2 - RW */;
pub const E1000_82542_RDTR1: c_uint = 0x00130;
pub const E1000_82542_RDBAL1: c_uint = 0x00138;
pub const E1000_82542_RDBAH1: c_uint = 0x0013C;
pub const E1000_82542_RDLEN1: c_uint = 0x00140;
pub const E1000_82542_RDH1: c_uint = 0x00148;
pub const E1000_82542_RDT1: c_uint = 0x00150;
pub const E1000_82542_FCRTH: c_uint = 0x00160;
pub const E1000_82542_FCRTL: c_uint = 0x00168;

pub const E1000_82542_MTA: c_uint = 0x00200;

pub const E1000_82542_TDBAL: c_uint = 0x00420;
pub const E1000_82542_TDBAH: c_uint = 0x00424;
pub const E1000_82542_TDLEN: c_uint = 0x00428;
pub const E1000_82542_TDH: c_uint = 0x00430;
pub const E1000_82542_TDT: c_uint = 0x00438;
pub const E1000_82542_TIDV: c_uint = 0x00440;

pub const E1000_82542_VFTA: c_uint = 0x00600;

pub const E1000_82542_TDFH: c_uint = 0x08010;
pub const E1000_82542_TDFT: c_uint = 0x08018;

// Statistics counters collected by the MAC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_hw_stats {
    pub crcerrs: u64,
    pub algnerrc: u64,
    pub symerrs: u64,
    pub rxerrc: u64,
    pub txerrc: u64,
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
    pub gorcl: u64,
    pub gorch: u64,
    pub gotcl: u64,
    pub gotch: u64,
    pub rnbc: u64,
    pub ruc: u64,
    pub rfc: u64,
    pub roc: u64,
    pub rlerrc: u64,
    pub rjc: u64,
    pub mgprc: u64,
    pub mgpdc: u64,
    pub mgptc: u64,
    pub torl: u64,
    pub torh: u64,
    pub totl: u64,
    pub toth: u64,
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
}

// Structure containing variables used by the shared code (e1000_hw.c)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_hw {
    pub hw_addr: *mut u8 __iomem,
    pub flash_address: *mut u8 __iomem,
    pub ce4100_gbe_mdio_base_virt: *mut void __iomem,
    pub mac_type: e1000_mac_type,
    pub phy_type: e1000_phy_type,
    pub phy_init_script: u32,
    pub media_type: e1000_media_type,
    pub back: *mut c_void,
    pub eeprom_shadow_ram: *mut e1000_shadow_ram,
    pub flash_bank_size: u32,
    pub flash_base_addr: u32,
    pub fc: e1000_fc_type,
    pub bus_speed: e1000_bus_speed,
    pub bus_width: e1000_bus_width,
    pub bus_type: e1000_bus_type,
    pub eeprom: e1000_eeprom_info,
    pub master_slave: e1000_ms_type,
    pub original_master_slave: e1000_ms_type,
    pub ffe_config_state: e1000_ffe_config,
    pub asf_firmware_present: u32,
    pub eeprom_semaphore_present: u32,
    pub io_base: c_ulong,
    pub phy_id: u32,
    pub phy_revision: u32,
    pub phy_addr: u32,
    pub original_fc: u32,
    pub txcw: u32,
    pub autoneg_failed: u32,
    pub max_frame_size: u32,
    pub min_frame_size: u32,
    pub mc_filter_type: u32,
    pub num_mc_addrs: u32,
    pub collision_delta: u32,
    pub tx_packet_delta: u32,
    pub ledctl_default: u32,
    pub ledctl_mode1: u32,
    pub ledctl_mode2: u32,
    pub tx_pkt_filtering: bool,
    pub mng_cookie: e1000_host_mng_dhcp_cookie,
    pub phy_spd_default: u16,
    pub autoneg_advertised: u16,
    pub pci_cmd_word: u16,
    pub fc_high_water: u16,
    pub fc_low_water: u16,
    pub fc_pause_time: u16,
    pub current_ifs_val: u16,
    pub ifs_min_val: u16,
    pub ifs_max_val: u16,
    pub ifs_step_size: u16,
    pub ifs_ratio: u16,
    pub device_id: u16,
    pub vendor_id: u16,
    pub subsystem_id: u16,
    pub subsystem_vendor_id: u16,
    pub revision_id: u8,
    pub autoneg: u8,
    pub mdix: u8,
    pub forced_speed_duplex: u8,
    pub wait_autoneg_complete: u8,
    pub dma_fairness: u8,
    pub mac_addr: [u8; NODE_ADDRESS_SIZE],
    pub perm_mac_addr: [u8; NODE_ADDRESS_SIZE],
    pub disable_polarity_correction: bool,
    pub speed_downgraded: bool,
    pub smart_speed: e1000_smart_speed,
    pub dsp_config_state: e1000_dsp_config,
    pub get_link_status: bool,
    pub serdes_has_link: bool,
    pub tbi_compatibility_en: bool,
    pub tbi_compatibility_on: bool,
    pub laa_is_present: bool,
    pub phy_reset_disable: bool,
    pub initialize_hw_bits_disable: bool,
    pub fc_send_xon: bool,
    pub fc_strict_ieee: bool,
    pub report_tx_early: bool,
    pub adaptive_ifs: bool,
    pub ifs_params_forced: bool,
    pub in_ifs_mode: bool,
    pub mng_reg_access_disabled: bool,
    pub leave_av_bit_off: bool,
    pub bad_tx_carr_stats_fd: bool,
    pub has_smbus: bool,
}

pub const E1000_EEPROM_SWDPIN0: c_uint = 0x0001	/* SWDPIN 0 EEPROM Value */;
pub const E1000_EEPROM_LED_LOGIC: c_uint = 0x0020	/* Led Logic Word */;

// Register Bit Masks
// Device Control
pub const E1000_CTRL_FD: c_uint = 0x00000001	/* Full duplex.0=half; 1=full */;
pub const E1000_CTRL_BEM: c_uint = 0x00000002	/* Endian Mode.0=little,1=big */;
pub const E1000_CTRL_PRIOR: c_uint = 0x00000004	/* Priority on PCI. 0=rx,1=fair */;
pub const E1000_CTRL_GIO_MASTER_DISABLE: c_uint = 0x00000004	/*Blocks new Master requests */;
pub const E1000_CTRL_LRST: c_uint = 0x00000008	/* Link reset. 0=normal,1=reset */;
pub const E1000_CTRL_TME: c_uint = 0x00000010	/* Test mode. 0=normal,1=test */;
pub const E1000_CTRL_SLE: c_uint = 0x00000020	/* Serial Link on 0=dis,1=en */;
pub const E1000_CTRL_ASDE: c_uint = 0x00000020	/* Auto-speed detect enable */;
pub const E1000_CTRL_SLU: c_uint = 0x00000040	/* Set link up (Force Link) */;
pub const E1000_CTRL_ILOS: c_uint = 0x00000080	/* Invert Loss-Of Signal */;
pub const E1000_CTRL_SPD_SEL: c_uint = 0x00000300	/* Speed Select Mask */;
pub const E1000_CTRL_SPD_10: c_uint = 0x00000000	/* Force 10Mb */;
pub const E1000_CTRL_SPD_100: c_uint = 0x00000100	/* Force 100Mb */;
pub const E1000_CTRL_SPD_1000: c_uint = 0x00000200	/* Force 1Gb */;
pub const E1000_CTRL_BEM32: c_uint = 0x00000400	/* Big Endian 32 mode */;
pub const E1000_CTRL_FRCSPD: c_uint = 0x00000800	/* Force Speed */;
pub const E1000_CTRL_FRCDPX: c_uint = 0x00001000	/* Force Duplex */;
pub const E1000_CTRL_D_UD_EN: c_uint = 0x00002000	/* Dock/Undock enable */;
pub const E1000_CTRL_D_UD_POLARITY: c_uint = 0x00004000	/* Defined polarity of Dock/Undock indication in SDP[0] */;
pub const E1000_CTRL_FORCE_PHY_RESET: c_uint = 0x00008000	/* Reset both PHY ports, through PHYRST_N pin */;
pub const E1000_CTRL_EXT_LINK_EN: c_uint = 0x00010000	/* enable link status from external LINK_0 and LINK_1 pins */;
pub const E1000_CTRL_SWDPIN0: c_uint = 0x00040000	/* SWDPIN 0 value */;
pub const E1000_CTRL_SWDPIN1: c_uint = 0x00080000	/* SWDPIN 1 value */;
pub const E1000_CTRL_SWDPIN2: c_uint = 0x00100000	/* SWDPIN 2 value */;
pub const E1000_CTRL_SWDPIN3: c_uint = 0x00200000	/* SWDPIN 3 value */;
pub const E1000_CTRL_SWDPIO0: c_uint = 0x00400000	/* SWDPIN 0 Input or output */;
pub const E1000_CTRL_SWDPIO1: c_uint = 0x00800000	/* SWDPIN 1 input or output */;
pub const E1000_CTRL_SWDPIO2: c_uint = 0x01000000	/* SWDPIN 2 input or output */;
pub const E1000_CTRL_SWDPIO3: c_uint = 0x02000000	/* SWDPIN 3 input or output */;
pub const E1000_CTRL_RST: c_uint = 0x04000000	/* Global reset */;
pub const E1000_CTRL_RFCE: c_uint = 0x08000000	/* Receive Flow Control enable */;
pub const E1000_CTRL_TFCE: c_uint = 0x10000000	/* Transmit flow control enable */;
pub const E1000_CTRL_RTE: c_uint = 0x20000000	/* Routing tag enable */;
pub const E1000_CTRL_VME: c_uint = 0x40000000	/* IEEE VLAN mode enable */;
pub const E1000_CTRL_PHY_RST: c_uint = 0x80000000	/* PHY Reset */;
pub const E1000_CTRL_SW2FW_INT: c_uint = 0x02000000	/* Initiate an interrupt to manageability engine */;
// Device Status
pub const E1000_STATUS_FD: c_uint = 0x00000001	/* Full duplex.0=half,1=full */;
pub const E1000_STATUS_LU: c_uint = 0x00000002	/* Link up.0=no,1=link */;
pub const E1000_STATUS_FUNC_MASK: c_uint = 0x0000000C	/* PCI Function Mask */;
pub const E1000_STATUS_FUNC_SHIFT: c_int = 2;
pub const E1000_STATUS_FUNC_0: c_uint = 0x00000000	/* Function 0 */;
pub const E1000_STATUS_FUNC_1: c_uint = 0x00000004	/* Function 1 */;
pub const E1000_STATUS_TXOFF: c_uint = 0x00000010	/* transmission paused */;
pub const E1000_STATUS_TBIMODE: c_uint = 0x00000020	/* TBI mode */;
pub const E1000_STATUS_SPEED_MASK: c_uint = 0x000000C0;
pub const E1000_STATUS_SPEED_10: c_uint = 0x00000000	/* Speed 10Mb/s */;
pub const E1000_STATUS_SPEED_100: c_uint = 0x00000040	/* Speed 100Mb/s */;
pub const E1000_STATUS_SPEED_1000: c_uint = 0x00000080	/* Speed 1000Mb/s */;
pub const E1000_STATUS_LAN_INIT_DONE: c_uint = 0x00000200	/* Lan Init Completion;
pub const E1000_STATUS_ASDV: c_uint = 0x00000300	/* Auto speed detect value */;
pub const E1000_STATUS_DOCK_CI: c_uint = 0x00000800	/* Change in Dock/Undock state. Clear on write '0'. */;
pub const E1000_STATUS_GIO_MASTER_ENABLE: c_uint = 0x00080000	/* Status of Master requests. */;
pub const E1000_STATUS_MTXCKOK: c_uint = 0x00000400	/* MTX clock running OK */;
pub const E1000_STATUS_PCI66: c_uint = 0x00000800	/* In 66Mhz slot */;
pub const E1000_STATUS_BUS64: c_uint = 0x00001000	/* In 64 bit slot */;
pub const E1000_STATUS_PCIX_MODE: c_uint = 0x00002000	/* PCI-X mode */;
pub const E1000_STATUS_PCIX_SPEED: c_uint = 0x0000C000	/* PCI-X bus speed */;
pub const E1000_STATUS_BMC_SKU_0: c_uint = 0x00100000	/* BMC USB redirect disabled */;
pub const E1000_STATUS_BMC_SKU_1: c_uint = 0x00200000	/* BMC SRAM disabled */;
pub const E1000_STATUS_BMC_SKU_2: c_uint = 0x00400000	/* BMC SDRAM disabled */;
pub const E1000_STATUS_BMC_CRYPTO: c_uint = 0x00800000	/* BMC crypto disabled */;
pub const E1000_STATUS_BMC_LITE: c_uint = 0x01000000	/* BMC external code execution disabled */;
pub const E1000_STATUS_RGMII_ENABLE: c_uint = 0x02000000	/* RGMII disabled */;
pub const E1000_STATUS_FUSE_8: c_uint = 0x04000000;
pub const E1000_STATUS_FUSE_9: c_uint = 0x08000000;
pub const E1000_STATUS_SERDES0_DIS: c_uint = 0x10000000	/* SERDES disabled on port 0 */;
pub const E1000_STATUS_SERDES1_DIS: c_uint = 0x20000000	/* SERDES disabled on port 1 */;
// Constants used to interpret the masked PCI-X bus speed.
pub const E1000_STATUS_PCIX_SPEED_66: c_uint = 0x00000000	/* PCI-X bus speed  50-66 MHz */;
pub const E1000_STATUS_PCIX_SPEED_100: c_uint = 0x00004000	/* PCI-X bus speed  66-100 MHz */;
pub const E1000_STATUS_PCIX_SPEED_133: c_uint = 0x00008000	/* PCI-X bus speed 100-133 MHz */;
// EEPROM/Flash Control
pub const E1000_EECD_SK: c_uint = 0x00000001	/* EEPROM Clock */;
pub const E1000_EECD_CS: c_uint = 0x00000002	/* EEPROM Chip Select */;
pub const E1000_EECD_DI: c_uint = 0x00000004	/* EEPROM Data In */;
pub const E1000_EECD_DO: c_uint = 0x00000008	/* EEPROM Data Out */;
pub const E1000_EECD_FWE_MASK: c_uint = 0x00000030;
pub const E1000_EECD_FWE_DIS: c_uint = 0x00000010	/* Disable FLASH writes */;
pub const E1000_EECD_FWE_EN: c_uint = 0x00000020	/* Enable FLASH writes */;
pub const E1000_EECD_FWE_SHIFT: c_int = 4;
pub const E1000_EECD_REQ: c_uint = 0x00000040	/* EEPROM Access Request */;
pub const E1000_EECD_GNT: c_uint = 0x00000080	/* EEPROM Access Grant */;
pub const E1000_EECD_PRES: c_uint = 0x00000100	/* EEPROM Present */;
pub const E1000_EECD_SIZE: c_uint = 0x00000200	/* EEPROM Size (0=64 word 1=256 word) */;
pub const E1000_EECD_ADDR_BITS: c_uint = 0x00000400	/* EEPROM Addressing bits based on type;
// (0-small, 1-large)
pub const E1000_EECD_TYPE: c_uint = 0x00002000	/* EEPROM Type (1-SPI, 0-Microwire) */;

pub const E1000_EECD_AUTO_RD: c_uint = 0x00000200	/* EEPROM Auto Read done */;
pub const E1000_EECD_SIZE_EX_MASK: c_uint = 0x00007800	/* EEprom Size */;
pub const E1000_EECD_SIZE_EX_SHIFT: c_int = 11;
pub const E1000_EECD_NVADDS: c_uint = 0x00018000	/* NVM Address Size */;
pub const E1000_EECD_SELSHAD: c_uint = 0x00020000	/* Select Shadow RAM */;
pub const E1000_EECD_INITSRAM: c_uint = 0x00040000	/* Initialize Shadow RAM */;
pub const E1000_EECD_FLUPD: c_uint = 0x00080000	/* Update FLASH */;
pub const E1000_EECD_AUPDEN: c_uint = 0x00100000	/* Enable Autonomous FLASH update */;
pub const E1000_EECD_SHADV: c_uint = 0x00200000	/* Shadow RAM Data Valid */;
pub const E1000_EECD_SEC1VAL: c_uint = 0x00400000	/* Sector One Valid */;
pub const E1000_EECD_SECVAL_SHIFT: c_int = 22;
pub const E1000_STM_OPCODE: c_uint = 0xDB00;
pub const E1000_HICR_FW_RESET: c_uint = 0xC0;
pub const E1000_SHADOW_RAM_WORDS: c_int = 2048;
pub const E1000_ICH_NVM_SIG_WORD: c_uint = 0x13;
pub const E1000_ICH_NVM_SIG_MASK: c_uint = 0xC0;
// EEPROM Read
pub const E1000_EERD_START: c_uint = 0x00000001	/* Start Read */;
pub const E1000_EERD_DONE: c_uint = 0x00000010	/* Read Done */;
pub const E1000_EERD_ADDR_SHIFT: c_int = 8;
pub const E1000_EERD_ADDR_MASK: c_uint = 0x0000FF00	/* Read Address */;
pub const E1000_EERD_DATA_SHIFT: c_int = 16;
pub const E1000_EERD_DATA_MASK: c_uint = 0xFFFF0000	/* Read Data */;
// SPI EEPROM Status Register
pub const EEPROM_STATUS_RDY_SPI: c_uint = 0x01;
pub const EEPROM_STATUS_WEN_SPI: c_uint = 0x02;
pub const EEPROM_STATUS_BP0_SPI: c_uint = 0x04;
pub const EEPROM_STATUS_BP1_SPI: c_uint = 0x08;
pub const EEPROM_STATUS_WPEN_SPI: c_uint = 0x80;
// Extended Device Control
pub const E1000_CTRL_EXT_GPI0_EN: c_uint = 0x00000001	/* Maps SDP4 to GPI0 */;
pub const E1000_CTRL_EXT_GPI1_EN: c_uint = 0x00000002	/* Maps SDP5 to GPI1 */;

pub const E1000_CTRL_EXT_GPI2_EN: c_uint = 0x00000004	/* Maps SDP6 to GPI2 */;
pub const E1000_CTRL_EXT_GPI3_EN: c_uint = 0x00000008	/* Maps SDP7 to GPI3 */;
pub const E1000_CTRL_EXT_SDP4_DATA: c_uint = 0x00000010	/* Value of SW Defineable Pin 4 */;
pub const E1000_CTRL_EXT_SDP5_DATA: c_uint = 0x00000020	/* Value of SW Defineable Pin 5 */;

pub const E1000_CTRL_EXT_SDP6_DATA: c_uint = 0x00000040	/* Value of SW Defineable Pin 6 */;
pub const E1000_CTRL_EXT_SDP7_DATA: c_uint = 0x00000080	/* Value of SW Defineable Pin 7 */;
pub const E1000_CTRL_EXT_SDP4_DIR: c_uint = 0x00000100	/* Direction of SDP4 0=in 1=out */;
pub const E1000_CTRL_EXT_SDP5_DIR: c_uint = 0x00000200	/* Direction of SDP5 0=in 1=out */;
pub const E1000_CTRL_EXT_SDP6_DIR: c_uint = 0x00000400	/* Direction of SDP6 0=in 1=out */;
pub const E1000_CTRL_EXT_SDP7_DIR: c_uint = 0x00000800	/* Direction of SDP7 0=in 1=out */;
pub const E1000_CTRL_EXT_ASDCHK: c_uint = 0x00001000	/* Initiate an ASD sequence */;
pub const E1000_CTRL_EXT_EE_RST: c_uint = 0x00002000	/* Reinitialize from EEPROM */;
pub const E1000_CTRL_EXT_IPS: c_uint = 0x00004000	/* Invert Power State */;
pub const E1000_CTRL_EXT_SPD_BYPS: c_uint = 0x00008000	/* Speed Select Bypass */;
pub const E1000_CTRL_EXT_RO_DIS: c_uint = 0x00020000	/* Relaxed Ordering disable */;
pub const E1000_CTRL_EXT_LINK_MODE_MASK: c_uint = 0x00C00000;
pub const E1000_CTRL_EXT_LINK_MODE_GMII: c_uint = 0x00000000;
pub const E1000_CTRL_EXT_LINK_MODE_TBI: c_uint = 0x00C00000;
pub const E1000_CTRL_EXT_LINK_MODE_KMRN: c_uint = 0x00000000;
pub const E1000_CTRL_EXT_LINK_MODE_SERDES: c_uint = 0x00C00000;
pub const E1000_CTRL_EXT_LINK_MODE_SGMII: c_uint = 0x00800000;
pub const E1000_CTRL_EXT_WR_WMARK_MASK: c_uint = 0x03000000;
pub const E1000_CTRL_EXT_WR_WMARK_256: c_uint = 0x00000000;
pub const E1000_CTRL_EXT_WR_WMARK_320: c_uint = 0x01000000;
pub const E1000_CTRL_EXT_WR_WMARK_384: c_uint = 0x02000000;
pub const E1000_CTRL_EXT_WR_WMARK_448: c_uint = 0x03000000;
pub const E1000_CTRL_EXT_DRV_LOAD: c_uint = 0x10000000	/* Driver loaded bit for FW */;
pub const E1000_CTRL_EXT_IAME: c_uint = 0x08000000	/* Interrupt acknowledge Auto-mask */;
pub const E1000_CTRL_EXT_INT_TIMER_CLR: c_uint = 0x20000000	/* Clear Interrupt timers after IMS clear */;
pub const E1000_CRTL_EXT_PB_PAREN: c_uint = 0x01000000	/* packet buffer parity error detection enabled */;
pub const E1000_CTRL_EXT_DF_PAREN: c_uint = 0x02000000	/* descriptor FIFO parity error detection enable */;
pub const E1000_CTRL_EXT_GHOST_PAREN: c_uint = 0x40000000;
// MDI Control
pub const E1000_MDIC_DATA_MASK: c_uint = 0x0000FFFF;
pub const E1000_MDIC_REG_MASK: c_uint = 0x001F0000;
pub const E1000_MDIC_REG_SHIFT: c_int = 16;
pub const E1000_MDIC_PHY_MASK: c_uint = 0x03E00000;
pub const E1000_MDIC_PHY_SHIFT: c_int = 21;
pub const E1000_MDIC_OP_WRITE: c_uint = 0x04000000;
pub const E1000_MDIC_OP_READ: c_uint = 0x08000000;
pub const E1000_MDIC_READY: c_uint = 0x10000000;
pub const E1000_MDIC_INT_EN: c_uint = 0x20000000;
pub const E1000_MDIC_ERROR: c_uint = 0x40000000;
pub const INTEL_CE_GBE_MDIC_OP_WRITE: c_uint = 0x04000000;
pub const INTEL_CE_GBE_MDIC_OP_READ: c_uint = 0x00000000;
pub const INTEL_CE_GBE_MDIC_GO: c_uint = 0x80000000;
pub const INTEL_CE_GBE_MDIC_READ_ERROR: c_uint = 0x80000000;
pub const E1000_KUMCTRLSTA_MASK: c_uint = 0x0000FFFF;
pub const E1000_KUMCTRLSTA_OFFSET: c_uint = 0x001F0000;
pub const E1000_KUMCTRLSTA_OFFSET_SHIFT: c_int = 16;
pub const E1000_KUMCTRLSTA_REN: c_uint = 0x00200000;
pub const E1000_KUMCTRLSTA_OFFSET_FIFO_CTRL: c_uint = 0x00000000;
pub const E1000_KUMCTRLSTA_OFFSET_CTRL: c_uint = 0x00000001;
pub const E1000_KUMCTRLSTA_OFFSET_INB_CTRL: c_uint = 0x00000002;
pub const E1000_KUMCTRLSTA_OFFSET_DIAG: c_uint = 0x00000003;
pub const E1000_KUMCTRLSTA_OFFSET_TIMEOUTS: c_uint = 0x00000004;
pub const E1000_KUMCTRLSTA_OFFSET_INB_PARAM: c_uint = 0x00000009;
pub const E1000_KUMCTRLSTA_OFFSET_HD_CTRL: c_uint = 0x00000010;
pub const E1000_KUMCTRLSTA_OFFSET_M2P_SERDES: c_uint = 0x0000001E;
pub const E1000_KUMCTRLSTA_OFFSET_M2P_MODES: c_uint = 0x0000001F;
// FIFO Control
pub const E1000_KUMCTRLSTA_FIFO_CTRL_RX_BYPASS: c_uint = 0x00000008;
pub const E1000_KUMCTRLSTA_FIFO_CTRL_TX_BYPASS: c_uint = 0x00000800;
// In-Band Control
pub const E1000_KUMCTRLSTA_INB_CTRL_LINK_STATUS_TX_TIMEOUT_DEFAULT: c_uint = 0x00000500;
pub const E1000_KUMCTRLSTA_INB_CTRL_DIS_PADDING: c_uint = 0x00000010;
// Half-Duplex Control
pub const E1000_KUMCTRLSTA_HD_CTRL_10_100_DEFAULT: c_uint = 0x00000004;
pub const E1000_KUMCTRLSTA_HD_CTRL_1000_DEFAULT: c_uint = 0x00000000;
pub const E1000_KUMCTRLSTA_OFFSET_K0S_CTRL: c_uint = 0x0000001E;
pub const E1000_KUMCTRLSTA_DIAG_FELPBK: c_uint = 0x2000;
pub const E1000_KUMCTRLSTA_DIAG_NELPBK: c_uint = 0x1000;
pub const E1000_KUMCTRLSTA_K0S_100_EN: c_uint = 0x2000;
pub const E1000_KUMCTRLSTA_K0S_GBE_EN: c_uint = 0x1000;
pub const E1000_KUMCTRLSTA_K0S_ENTRY_LATENCY_MASK: c_uint = 0x0003;
pub const E1000_KABGTXD_BGSQLBIAS: c_uint = 0x00050000;
pub const E1000_PHY_CTRL_SPD_EN: c_uint = 0x00000001;
pub const E1000_PHY_CTRL_D0A_LPLU: c_uint = 0x00000002;
pub const E1000_PHY_CTRL_NOND0A_LPLU: c_uint = 0x00000004;
pub const E1000_PHY_CTRL_NOND0A_GBE_DISABLE: c_uint = 0x00000008;
pub const E1000_PHY_CTRL_GBE_DISABLE: c_uint = 0x00000040;
pub const E1000_PHY_CTRL_B2B_EN: c_uint = 0x00000080;
// LED Control
pub const E1000_LEDCTL_LED0_MODE_MASK: c_uint = 0x0000000F;
pub const E1000_LEDCTL_LED0_MODE_SHIFT: c_int = 0;
pub const E1000_LEDCTL_LED0_BLINK_RATE: c_uint = 0x0000020;
pub const E1000_LEDCTL_LED0_IVRT: c_uint = 0x00000040;
pub const E1000_LEDCTL_LED0_BLINK: c_uint = 0x00000080;
pub const E1000_LEDCTL_LED1_MODE_MASK: c_uint = 0x00000F00;
pub const E1000_LEDCTL_LED1_MODE_SHIFT: c_int = 8;
pub const E1000_LEDCTL_LED1_BLINK_RATE: c_uint = 0x0002000;
pub const E1000_LEDCTL_LED1_IVRT: c_uint = 0x00004000;
pub const E1000_LEDCTL_LED1_BLINK: c_uint = 0x00008000;
pub const E1000_LEDCTL_LED2_MODE_MASK: c_uint = 0x000F0000;
pub const E1000_LEDCTL_LED2_MODE_SHIFT: c_int = 16;
pub const E1000_LEDCTL_LED2_BLINK_RATE: c_uint = 0x00200000;
pub const E1000_LEDCTL_LED2_IVRT: c_uint = 0x00400000;
pub const E1000_LEDCTL_LED2_BLINK: c_uint = 0x00800000;
pub const E1000_LEDCTL_LED3_MODE_MASK: c_uint = 0x0F000000;
pub const E1000_LEDCTL_LED3_MODE_SHIFT: c_int = 24;
pub const E1000_LEDCTL_LED3_BLINK_RATE: c_uint = 0x20000000;
pub const E1000_LEDCTL_LED3_IVRT: c_uint = 0x40000000;
pub const E1000_LEDCTL_LED3_BLINK: c_uint = 0x80000000;
pub const E1000_LEDCTL_MODE_LINK_10_1000: c_uint = 0x0;
pub const E1000_LEDCTL_MODE_LINK_100_1000: c_uint = 0x1;
pub const E1000_LEDCTL_MODE_LINK_UP: c_uint = 0x2;
pub const E1000_LEDCTL_MODE_ACTIVITY: c_uint = 0x3;
pub const E1000_LEDCTL_MODE_LINK_ACTIVITY: c_uint = 0x4;
pub const E1000_LEDCTL_MODE_LINK_10: c_uint = 0x5;
pub const E1000_LEDCTL_MODE_LINK_100: c_uint = 0x6;
pub const E1000_LEDCTL_MODE_LINK_1000: c_uint = 0x7;
pub const E1000_LEDCTL_MODE_PCIX_MODE: c_uint = 0x8;
pub const E1000_LEDCTL_MODE_FULL_DUPLEX: c_uint = 0x9;
pub const E1000_LEDCTL_MODE_COLLISION: c_uint = 0xA;
pub const E1000_LEDCTL_MODE_BUS_SPEED: c_uint = 0xB;
pub const E1000_LEDCTL_MODE_BUS_SIZE: c_uint = 0xC;
pub const E1000_LEDCTL_MODE_PAUSED: c_uint = 0xD;
pub const E1000_LEDCTL_MODE_LED_ON: c_uint = 0xE;
pub const E1000_LEDCTL_MODE_LED_OFF: c_uint = 0xF;
// Receive Address
pub const E1000_RAH_AV: c_uint = 0x80000000	/* Receive descriptor valid */;
// Interrupt Cause Read
pub const E1000_ICR_TXDW: c_uint = 0x00000001	/* Transmit desc written back */;
pub const E1000_ICR_TXQE: c_uint = 0x00000002	/* Transmit Queue empty */;
pub const E1000_ICR_LSC: c_uint = 0x00000004	/* Link Status Change */;
pub const E1000_ICR_RXSEQ: c_uint = 0x00000008	/* rx sequence error */;
pub const E1000_ICR_RXDMT0: c_uint = 0x00000010	/* rx desc min. threshold (0) */;
pub const E1000_ICR_RXO: c_uint = 0x00000040	/* rx overrun */;
pub const E1000_ICR_RXT0: c_uint = 0x00000080	/* rx timer intr (ring 0) */;
pub const E1000_ICR_MDAC: c_uint = 0x00000200	/* MDIO access complete */;
pub const E1000_ICR_RXCFG: c_uint = 0x00000400	/* RX /c/ ordered set */;
pub const E1000_ICR_GPI_EN0: c_uint = 0x00000800	/* GP Int 0 */;
pub const E1000_ICR_GPI_EN1: c_uint = 0x00001000	/* GP Int 1 */;
pub const E1000_ICR_GPI_EN2: c_uint = 0x00002000	/* GP Int 2 */;
pub const E1000_ICR_GPI_EN3: c_uint = 0x00004000	/* GP Int 3 */;
pub const E1000_ICR_TXD_LOW: c_uint = 0x00008000;
pub const E1000_ICR_SRPD: c_uint = 0x00010000;
pub const E1000_ICR_ACK: c_uint = 0x00020000	/* Receive Ack frame */;
pub const E1000_ICR_MNG: c_uint = 0x00040000	/* Manageability event */;
pub const E1000_ICR_DOCK: c_uint = 0x00080000	/* Dock/Undock */;
pub const E1000_ICR_INT_ASSERTED: c_uint = 0x80000000	/* If this bit asserted, the driver should claim the interrupt */;
pub const E1000_ICR_RXD_FIFO_PAR0: c_uint = 0x00100000	/* queue 0 Rx descriptor FIFO parity error */;
pub const E1000_ICR_TXD_FIFO_PAR0: c_uint = 0x00200000	/* queue 0 Tx descriptor FIFO parity error */;
pub const E1000_ICR_HOST_ARB_PAR: c_uint = 0x00400000	/* host arb read buffer parity error */;
pub const E1000_ICR_PB_PAR: c_uint = 0x00800000	/* packet buffer parity error */;
pub const E1000_ICR_RXD_FIFO_PAR1: c_uint = 0x01000000	/* queue 1 Rx descriptor FIFO parity error */;
pub const E1000_ICR_TXD_FIFO_PAR1: c_uint = 0x02000000	/* queue 1 Tx descriptor FIFO parity error */;
pub const E1000_ICR_ALL_PARITY: c_uint = 0x03F00000	/* all parity error bits */;
pub const E1000_ICR_DSW: c_uint = 0x00000020	/* FW changed the status of DISSW bit in the FWSM */;
pub const E1000_ICR_PHYINT: c_uint = 0x00001000	/* LAN connected device generates an interrupt */;
pub const E1000_ICR_EPRST: c_uint = 0x00100000	/* ME hardware reset occurs */;
// Interrupt Cause Set

// Interrupt Mask Set

// Interrupt Mask Clear

// Receive Control
pub const E1000_RCTL_RST: c_uint = 0x00000001	/* Software reset */;
pub const E1000_RCTL_EN: c_uint = 0x00000002	/* enable */;
pub const E1000_RCTL_SBP: c_uint = 0x00000004	/* store bad packet */;
pub const E1000_RCTL_UPE: c_uint = 0x00000008	/* unicast promiscuous enable */;
pub const E1000_RCTL_MPE: c_uint = 0x00000010	/* multicast promiscuous enab */;
pub const E1000_RCTL_LPE: c_uint = 0x00000020	/* long packet enable */;
pub const E1000_RCTL_LBM_NO: c_uint = 0x00000000	/* no loopback mode */;
pub const E1000_RCTL_LBM_MAC: c_uint = 0x00000040	/* MAC loopback mode */;
pub const E1000_RCTL_LBM_SLP: c_uint = 0x00000080	/* serial link loopback mode */;
pub const E1000_RCTL_LBM_TCVR: c_uint = 0x000000C0	/* tcvr loopback mode */;
pub const E1000_RCTL_DTYP_MASK: c_uint = 0x00000C00	/* Descriptor type mask */;
pub const E1000_RCTL_DTYP_PS: c_uint = 0x00000400	/* Packet Split descriptor */;
pub const E1000_RCTL_RDMTS_HALF: c_uint = 0x00000000	/* rx desc min threshold size */;
pub const E1000_RCTL_RDMTS_QUAT: c_uint = 0x00000100	/* rx desc min threshold size */;
pub const E1000_RCTL_RDMTS_EIGTH: c_uint = 0x00000200	/* rx desc min threshold size */;

pub const E1000_RCTL_MO_0: c_uint = 0x00000000	/* multicast offset 11:0 */;
pub const E1000_RCTL_MO_1: c_uint = 0x00001000	/* multicast offset 12:1 */;
pub const E1000_RCTL_MO_2: c_uint = 0x00002000	/* multicast offset 13:2 */;
pub const E1000_RCTL_MO_3: c_uint = 0x00003000	/* multicast offset 15:4 */;
pub const E1000_RCTL_MDR: c_uint = 0x00004000	/* multicast desc ring 0 */;
pub const E1000_RCTL_BAM: c_uint = 0x00008000	/* broadcast enable */;
// these buffer sizes are valid if E1000_RCTL_BSEX is 0
pub const E1000_RCTL_SZ_2048: c_uint = 0x00000000	/* rx buffer size 2048 */;
pub const E1000_RCTL_SZ_1024: c_uint = 0x00010000	/* rx buffer size 1024 */;
pub const E1000_RCTL_SZ_512: c_uint = 0x00020000	/* rx buffer size 512 */;
pub const E1000_RCTL_SZ_256: c_uint = 0x00030000	/* rx buffer size 256 */;
// these buffer sizes are valid if E1000_RCTL_BSEX is 1
pub const E1000_RCTL_SZ_16384: c_uint = 0x00010000	/* rx buffer size 16384 */;
pub const E1000_RCTL_SZ_8192: c_uint = 0x00020000	/* rx buffer size 8192 */;
pub const E1000_RCTL_SZ_4096: c_uint = 0x00030000	/* rx buffer size 4096 */;
pub const E1000_RCTL_VFE: c_uint = 0x00040000	/* vlan filter enable */;
pub const E1000_RCTL_CFIEN: c_uint = 0x00080000	/* canonical form enable */;
pub const E1000_RCTL_CFI: c_uint = 0x00100000	/* canonical form indicator */;
pub const E1000_RCTL_DPF: c_uint = 0x00400000	/* discard pause frames */;
pub const E1000_RCTL_PMCF: c_uint = 0x00800000	/* pass MAC control frames */;
pub const E1000_RCTL_BSEX: c_uint = 0x02000000	/* Buffer size extension */;
pub const E1000_RCTL_SECRC: c_uint = 0x04000000	/* Strip Ethernet CRC */;
pub const E1000_RCTL_FLXBUF_MASK: c_uint = 0x78000000	/* Flexible buffer size */;

// Use byte values for the following shift parameters
// Usage:
// psrctl |= (((ROUNDUP(value0, 128) >> E1000_PSRCTL_BSIZE0_SHIFT) &
// E1000_PSRCTL_BSIZE0_MASK) |
// ((ROUNDUP(value1, 1024) >> E1000_PSRCTL_BSIZE1_SHIFT) &
// E1000_PSRCTL_BSIZE1_MASK) |
// ((ROUNDUP(value2, 1024) << E1000_PSRCTL_BSIZE2_SHIFT) &
// E1000_PSRCTL_BSIZE2_MASK) |
// ((ROUNDUP(value3, 1024) << E1000_PSRCTL_BSIZE3_SHIFT) |;
// E1000_PSRCTL_BSIZE3_MASK))
// where value0 = [128..16256],  default=256
// value1 = [1024..64512], default=4096
// value2 = [0..64512],    default=4096
// value3 = [0..64512],    default=0
//
pub const E1000_PSRCTL_BSIZE0_MASK: c_uint = 0x0000007F;
pub const E1000_PSRCTL_BSIZE1_MASK: c_uint = 0x00003F00;
pub const E1000_PSRCTL_BSIZE2_MASK: c_uint = 0x003F0000;
pub const E1000_PSRCTL_BSIZE3_MASK: c_uint = 0x3F000000;

// SW_W_SYNC definitions
pub const E1000_SWFW_EEP_SM: c_uint = 0x0001;
pub const E1000_SWFW_PHY0_SM: c_uint = 0x0002;
pub const E1000_SWFW_PHY1_SM: c_uint = 0x0004;
pub const E1000_SWFW_MAC_CSR_SM: c_uint = 0x0008;
// Receive Descriptor
pub const E1000_RDT_DELAY: c_uint = 0x0000ffff	/* Delay timer (1=1024us) */;
pub const E1000_RDT_FPDB: c_uint = 0x80000000	/* Flush descriptor block */;
pub const E1000_RDLEN_LEN: c_uint = 0x0007ff80	/* descriptor length */;
pub const E1000_RDH_RDH: c_uint = 0x0000ffff	/* receive descriptor head */;
pub const E1000_RDT_RDT: c_uint = 0x0000ffff	/* receive descriptor tail */;
// Flow Control
pub const E1000_FCRTH_RTH: c_uint = 0x0000FFF8	/* Mask Bits[15:3] for RTH */;
pub const E1000_FCRTH_XFCE: c_uint = 0x80000000	/* External Flow Control Enable */;
pub const E1000_FCRTL_RTL: c_uint = 0x0000FFF8	/* Mask Bits[15:3] for RTL */;
pub const E1000_FCRTL_XONE: c_uint = 0x80000000	/* Enable XON frame transmission */;
// Header split receive
pub const E1000_RFCTL_ISCSI_DIS: c_uint = 0x00000001;
pub const E1000_RFCTL_ISCSI_DWC_MASK: c_uint = 0x0000003E;
pub const E1000_RFCTL_ISCSI_DWC_SHIFT: c_int = 1;
pub const E1000_RFCTL_NFSW_DIS: c_uint = 0x00000040;
pub const E1000_RFCTL_NFSR_DIS: c_uint = 0x00000080;
pub const E1000_RFCTL_NFS_VER_MASK: c_uint = 0x00000300;
pub const E1000_RFCTL_NFS_VER_SHIFT: c_int = 8;
pub const E1000_RFCTL_IPV6_DIS: c_uint = 0x00000400;
pub const E1000_RFCTL_IPV6_XSUM_DIS: c_uint = 0x00000800;
pub const E1000_RFCTL_ACK_DIS: c_uint = 0x00001000;
pub const E1000_RFCTL_ACKD_DIS: c_uint = 0x00002000;
pub const E1000_RFCTL_IPFRSP_DIS: c_uint = 0x00004000;
pub const E1000_RFCTL_EXTEN: c_uint = 0x00008000;
pub const E1000_RFCTL_IPV6_EX_DIS: c_uint = 0x00010000;
pub const E1000_RFCTL_NEW_IPV6_EXT_DIS: c_uint = 0x00020000;
// Receive Descriptor Control
pub const E1000_RXDCTL_PTHRESH: c_uint = 0x0000003F	/* RXDCTL Prefetch Threshold */;
pub const E1000_RXDCTL_HTHRESH: c_uint = 0x00003F00	/* RXDCTL Host Threshold */;
pub const E1000_RXDCTL_WTHRESH: c_uint = 0x003F0000	/* RXDCTL Writeback Threshold */;
pub const E1000_RXDCTL_GRAN: c_uint = 0x01000000	/* RXDCTL Granularity */;
// Transmit Descriptor Control
pub const E1000_TXDCTL_PTHRESH: c_uint = 0x0000003F	/* TXDCTL Prefetch Threshold */;
pub const E1000_TXDCTL_HTHRESH: c_uint = 0x00003F00	/* TXDCTL Host Threshold */;
pub const E1000_TXDCTL_WTHRESH: c_uint = 0x003F0000	/* TXDCTL Writeback Threshold */;
pub const E1000_TXDCTL_GRAN: c_uint = 0x01000000	/* TXDCTL Granularity */;
pub const E1000_TXDCTL_LWTHRESH: c_uint = 0xFE000000	/* TXDCTL Low Threshold */;
pub const E1000_TXDCTL_FULL_TX_DESC_WB: c_uint = 0x01010000	/* GRAN=1, WTHRESH=1 */;
pub const E1000_TXDCTL_COUNT_DESC: c_uint = 0x00400000	/* Enable the counting of desc.;
// Transmit Configuration Word
pub const E1000_TXCW_FD: c_uint = 0x00000020	/* TXCW full duplex */;
pub const E1000_TXCW_HD: c_uint = 0x00000040	/* TXCW half duplex */;
pub const E1000_TXCW_PAUSE: c_uint = 0x00000080	/* TXCW sym pause request */;
pub const E1000_TXCW_ASM_DIR: c_uint = 0x00000100	/* TXCW astm pause direction */;
pub const E1000_TXCW_PAUSE_MASK: c_uint = 0x00000180	/* TXCW pause request mask */;
pub const E1000_TXCW_RF: c_uint = 0x00003000	/* TXCW remote fault */;
pub const E1000_TXCW_NP: c_uint = 0x00008000	/* TXCW next page */;
pub const E1000_TXCW_CW: c_uint = 0x0000ffff	/* TxConfigWord mask */;
pub const E1000_TXCW_TXC: c_uint = 0x40000000	/* Transmit Config control */;
pub const E1000_TXCW_ANE: c_uint = 0x80000000	/* Auto-neg enable */;
// Receive Configuration Word
pub const E1000_RXCW_CW: c_uint = 0x0000ffff	/* RxConfigWord mask */;
pub const E1000_RXCW_NC: c_uint = 0x04000000	/* Receive config no carrier */;
pub const E1000_RXCW_IV: c_uint = 0x08000000	/* Receive config invalid */;
pub const E1000_RXCW_CC: c_uint = 0x10000000	/* Receive config change */;
pub const E1000_RXCW_C: c_uint = 0x20000000	/* Receive config */;
pub const E1000_RXCW_SYNCH: c_uint = 0x40000000	/* Receive config synch */;
pub const E1000_RXCW_ANC: c_uint = 0x80000000	/* Auto-neg complete */;
// Transmit Control
pub const E1000_TCTL_RST: c_uint = 0x00000001	/* software reset */;
pub const E1000_TCTL_EN: c_uint = 0x00000002	/* enable tx */;
pub const E1000_TCTL_BCE: c_uint = 0x00000004	/* busy check enable */;
pub const E1000_TCTL_PSP: c_uint = 0x00000008	/* pad short packets */;
pub const E1000_TCTL_CT: c_uint = 0x00000ff0	/* collision threshold */;
pub const E1000_TCTL_COLD: c_uint = 0x003ff000	/* collision distance */;
pub const E1000_TCTL_SWXOFF: c_uint = 0x00400000	/* SW Xoff transmission */;
pub const E1000_TCTL_PBE: c_uint = 0x00800000	/* Packet Burst Enable */;
pub const E1000_TCTL_RTLC: c_uint = 0x01000000	/* Re-transmit on late collision */;
pub const E1000_TCTL_NRTU: c_uint = 0x02000000	/* No Re-transmit on underrun */;
pub const E1000_TCTL_MULR: c_uint = 0x10000000	/* Multiple request support */;
// Extended Transmit Control
pub const E1000_TCTL_EXT_BST_MASK: c_uint = 0x000003FF	/* Backoff Slot Time */;
pub const E1000_TCTL_EXT_GCEX_MASK: c_uint = 0x000FFC00	/* Gigabit Carry Extend Padding */;
// Receive Checksum Control
pub const E1000_RXCSUM_PCSS_MASK: c_uint = 0x000000FF	/* Packet Checksum Start */;
pub const E1000_RXCSUM_IPOFL: c_uint = 0x00000100	/* IPv4 checksum offload */;
pub const E1000_RXCSUM_TUOFL: c_uint = 0x00000200	/* TCP / UDP checksum offload */;
pub const E1000_RXCSUM_IPV6OFL: c_uint = 0x00000400	/* IPv6 checksum offload */;
pub const E1000_RXCSUM_IPPCSE: c_uint = 0x00001000	/* IP payload checksum enable */;
pub const E1000_RXCSUM_PCSD: c_uint = 0x00002000	/* packet checksum disabled */;
// Multiple Receive Queue Control
pub const E1000_MRQC_ENABLE_MASK: c_uint = 0x00000003;
pub const E1000_MRQC_ENABLE_RSS_2Q: c_uint = 0x00000001;
pub const E1000_MRQC_ENABLE_RSS_INT: c_uint = 0x00000004;
pub const E1000_MRQC_RSS_FIELD_MASK: c_uint = 0xFFFF0000;
pub const E1000_MRQC_RSS_FIELD_IPV4_TCP: c_uint = 0x00010000;
pub const E1000_MRQC_RSS_FIELD_IPV4: c_uint = 0x00020000;
pub const E1000_MRQC_RSS_FIELD_IPV6_TCP_EX: c_uint = 0x00040000;
pub const E1000_MRQC_RSS_FIELD_IPV6_EX: c_uint = 0x00080000;
pub const E1000_MRQC_RSS_FIELD_IPV6: c_uint = 0x00100000;
pub const E1000_MRQC_RSS_FIELD_IPV6_TCP: c_uint = 0x00200000;
// Definitions for power management and wakeup registers
// Wake Up Control
pub const E1000_WUC_APME: c_uint = 0x00000001	/* APM Enable */;
pub const E1000_WUC_PME_EN: c_uint = 0x00000002	/* PME Enable */;
pub const E1000_WUC_PME_STATUS: c_uint = 0x00000004	/* PME Status */;
pub const E1000_WUC_APMPME: c_uint = 0x00000008	/* Assert PME on APM Wakeup */;
pub const E1000_WUC_SPM: c_uint = 0x80000000	/* Enable SPM */;
// Wake Up Filter Control
pub const E1000_WUFC_LNKC: c_uint = 0x00000001	/* Link Status Change Wakeup Enable */;
pub const E1000_WUFC_MAG: c_uint = 0x00000002	/* Magic Packet Wakeup Enable */;
pub const E1000_WUFC_EX: c_uint = 0x00000004	/* Directed Exact Wakeup Enable */;
pub const E1000_WUFC_MC: c_uint = 0x00000008	/* Directed Multicast Wakeup Enable */;
pub const E1000_WUFC_BC: c_uint = 0x00000010	/* Broadcast Wakeup Enable */;
pub const E1000_WUFC_ARP: c_uint = 0x00000020	/* ARP Request Packet Wakeup Enable */;
pub const E1000_WUFC_IPV4: c_uint = 0x00000040	/* Directed IPv4 Packet Wakeup Enable */;
pub const E1000_WUFC_IPV6: c_uint = 0x00000080	/* Directed IPv6 Packet Wakeup Enable */;
pub const E1000_WUFC_IGNORE_TCO: c_uint = 0x00008000	/* Ignore WakeOn TCO packets */;
pub const E1000_WUFC_FLX0: c_uint = 0x00010000	/* Flexible Filter 0 Enable */;
pub const E1000_WUFC_FLX1: c_uint = 0x00020000	/* Flexible Filter 1 Enable */;
pub const E1000_WUFC_FLX2: c_uint = 0x00040000	/* Flexible Filter 2 Enable */;
pub const E1000_WUFC_FLX3: c_uint = 0x00080000	/* Flexible Filter 3 Enable */;
pub const E1000_WUFC_ALL_FILTERS: c_uint = 0x000F00FF	/* Mask for all wakeup filters */;

pub const E1000_WUFC_FLX_FILTERS: c_uint = 0x000F0000	/* Mask for the 4 flexible filters */;
// Wake Up Status
pub const E1000_WUS_LNKC: c_uint = 0x00000001	/* Link Status Changed */;
pub const E1000_WUS_MAG: c_uint = 0x00000002	/* Magic Packet Received */;
pub const E1000_WUS_EX: c_uint = 0x00000004	/* Directed Exact Received */;
pub const E1000_WUS_MC: c_uint = 0x00000008	/* Directed Multicast Received */;
pub const E1000_WUS_BC: c_uint = 0x00000010	/* Broadcast Received */;
pub const E1000_WUS_ARP: c_uint = 0x00000020	/* ARP Request Packet Received */;
pub const E1000_WUS_IPV4: c_uint = 0x00000040	/* Directed IPv4 Packet Wakeup Received */;
pub const E1000_WUS_IPV6: c_uint = 0x00000080	/* Directed IPv6 Packet Wakeup Received */;
pub const E1000_WUS_FLX0: c_uint = 0x00010000	/* Flexible Filter 0 Match */;
pub const E1000_WUS_FLX1: c_uint = 0x00020000	/* Flexible Filter 1 Match */;
pub const E1000_WUS_FLX2: c_uint = 0x00040000	/* Flexible Filter 2 Match */;
pub const E1000_WUS_FLX3: c_uint = 0x00080000	/* Flexible Filter 3 Match */;
pub const E1000_WUS_FLX_FILTERS: c_uint = 0x000F0000	/* Mask for the 4 flexible filters */;
// Management Control
pub const E1000_MANC_SMBUS_EN: c_uint = 0x00000001	/* SMBus Enabled - RO */;
pub const E1000_MANC_ASF_EN: c_uint = 0x00000002	/* ASF Enabled - RO */;
pub const E1000_MANC_R_ON_FORCE: c_uint = 0x00000004	/* Reset on Force TCO - RO */;
pub const E1000_MANC_RMCP_EN: c_uint = 0x00000100	/* Enable RCMP 026Fh Filtering */;
pub const E1000_MANC_0298_EN: c_uint = 0x00000200	/* Enable RCMP 0298h Filtering */;
pub const E1000_MANC_IPV4_EN: c_uint = 0x00000400	/* Enable IPv4 */;
pub const E1000_MANC_IPV6_EN: c_uint = 0x00000800	/* Enable IPv6 */;
pub const E1000_MANC_SNAP_EN: c_uint = 0x00001000	/* Accept LLC/SNAP */;
pub const E1000_MANC_ARP_EN: c_uint = 0x00002000	/* Enable ARP Request Filtering */;
pub const E1000_MANC_NEIGHBOR_EN: c_uint = 0x00004000	/* Enable Neighbor Discovery;
// Filtering
pub const E1000_MANC_ARP_RES_EN: c_uint = 0x00008000	/* Enable ARP response Filtering */;
pub const E1000_MANC_TCO_RESET: c_uint = 0x00010000	/* TCO Reset Occurred */;
pub const E1000_MANC_RCV_TCO_EN: c_uint = 0x00020000	/* Receive TCO Packets Enabled */;
pub const E1000_MANC_REPORT_STATUS: c_uint = 0x00040000	/* Status Reporting Enabled */;
pub const E1000_MANC_RCV_ALL: c_uint = 0x00080000	/* Receive All Enabled */;
pub const E1000_MANC_BLK_PHY_RST_ON_IDE: c_uint = 0x00040000	/* Block phy resets */;
pub const E1000_MANC_EN_MAC_ADDR_FILTER: c_uint = 0x00100000	/* Enable MAC address;
// filtering
pub const E1000_MANC_EN_MNG2HOST: c_uint = 0x00200000	/* Enable MNG packets to host;
// memory
pub const E1000_MANC_EN_IP_ADDR_FILTER: c_uint = 0x00400000	/* Enable IP address;
// filtering
pub const E1000_MANC_EN_XSUM_FILTER: c_uint = 0x00800000	/* Enable checksum filtering */;
pub const E1000_MANC_BR_EN: c_uint = 0x01000000	/* Enable broadcast filtering */;
pub const E1000_MANC_SMB_REQ: c_uint = 0x01000000	/* SMBus Request */;
pub const E1000_MANC_SMB_GNT: c_uint = 0x02000000	/* SMBus Grant */;
pub const E1000_MANC_SMB_CLK_IN: c_uint = 0x04000000	/* SMBus Clock In */;
pub const E1000_MANC_SMB_DATA_IN: c_uint = 0x08000000	/* SMBus Data In */;
pub const E1000_MANC_SMB_DATA_OUT: c_uint = 0x10000000	/* SMBus Data Out */;
pub const E1000_MANC_SMB_CLK_OUT: c_uint = 0x20000000	/* SMBus Clock Out */;

// SW Semaphore Register
pub const E1000_SWSM_SMBI: c_uint = 0x00000001	/* Driver Semaphore bit */;
pub const E1000_SWSM_SWESMBI: c_uint = 0x00000002	/* FW Semaphore bit */;
pub const E1000_SWSM_WMNG: c_uint = 0x00000004	/* Wake MNG Clock */;
pub const E1000_SWSM_DRV_LOAD: c_uint = 0x00000008	/* Driver Loaded Bit */;
// FW Semaphore Register
pub const E1000_FWSM_MODE_MASK: c_uint = 0x0000000E	/* FW mode */;
pub const E1000_FWSM_MODE_SHIFT: c_int = 1;
pub const E1000_FWSM_FW_VALID: c_uint = 0x00008000	/* FW established a valid mode */;
pub const E1000_FWSM_RSPCIPHY: c_uint = 0x00000040	/* Reset PHY on PCI reset */;
pub const E1000_FWSM_DISSW: c_uint = 0x10000000	/* FW disable SW Write Access */;
pub const E1000_FWSM_SKUSEL_MASK: c_uint = 0x60000000	/* LAN SKU select */;
pub const E1000_FWSM_SKUEL_SHIFT: c_int = 29;
pub const E1000_FWSM_SKUSEL_EMB: c_uint = 0x0	/* Embedded SKU */;
pub const E1000_FWSM_SKUSEL_CONS: c_uint = 0x1	/* Consumer SKU */;
pub const E1000_FWSM_SKUSEL_PERF_100: c_uint = 0x2	/* Perf & Corp 10/100 SKU */;
pub const E1000_FWSM_SKUSEL_PERF_GBE: c_uint = 0x3	/* Perf & Copr GbE SKU */;
// FFLT Debug Register
pub const E1000_FFLT_DBG_INVC: c_uint = 0x00100000	/* Invalid /C/ code handling */;
// Host Interface Control Register
pub const E1000_HICR_EN: c_uint = 0x00000001	/* Enable Bit - RO */;
pub const E1000_HICR_C: c_uint = 0x00000002	/* Driver sets this bit when done;
// to put command in RAM
pub const E1000_HICR_SV: c_uint = 0x00000004	/* Status Validity */;
pub const E1000_HICR_FWR: c_uint = 0x00000080	/* FW reset. Set by the Host */;
// Host Interface Command Interface - Address range 0x8800-0x8EFF

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_host_command_header {
    pub command_id: u8,
    pub command_length: u8,
    pub /: *mut *mut u8 command_options; / I/F bits for command, status for return,
    pub checksum: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_host_command_info {
    pub /: *mut *mut e1000_host_command_header command_header; / Command Head/Command Result Head has 4 bytes,
    pub /: *mut *mut u8 command_data[E1000_HI_MAX_DATA_LENGTH]; / Command data can length 0..252,
}

// Host SMB register #0
pub const E1000_HSMC0R_CLKIN: c_uint = 0x00000001	/* SMB Clock in */;
pub const E1000_HSMC0R_DATAIN: c_uint = 0x00000002	/* SMB Data in */;
pub const E1000_HSMC0R_DATAOUT: c_uint = 0x00000004	/* SMB Data out */;
pub const E1000_HSMC0R_CLKOUT: c_uint = 0x00000008	/* SMB Clock out */;
// Host SMB register #1

// FW Status Register
pub const E1000_FWSTS_FWS_MASK: c_uint = 0x000000FF	/* FW Status */;
// Wake Up Packet Length
pub const E1000_WUPL_LENGTH_MASK: c_uint = 0x0FFF	/* Only the lower 12 bits are valid */;
pub const E1000_MDALIGN: c_int = 4096;
// PCI-Ex registers
// PCI-Ex Control Register
pub const E1000_GCR_RXD_NO_SNOOP: c_uint = 0x00000001;
pub const E1000_GCR_RXDSCW_NO_SNOOP: c_uint = 0x00000002;
pub const E1000_GCR_RXDSCR_NO_SNOOP: c_uint = 0x00000004;
pub const E1000_GCR_TXD_NO_SNOOP: c_uint = 0x00000008;
pub const E1000_GCR_TXDSCW_NO_SNOOP: c_uint = 0x00000010;
pub const E1000_GCR_TXDSCR_NO_SNOOP: c_uint = 0x00000020;

pub const E1000_GCR_L1_ACT_WITHOUT_L0S_RX: c_uint = 0x08000000;
// Function Active and Power State to MNG
pub const E1000_FACTPS_FUNC0_POWER_STATE_MASK: c_uint = 0x00000003;
pub const E1000_FACTPS_LAN0_VALID: c_uint = 0x00000004;
pub const E1000_FACTPS_FUNC0_AUX_EN: c_uint = 0x00000008;
pub const E1000_FACTPS_FUNC1_POWER_STATE_MASK: c_uint = 0x000000C0;
pub const E1000_FACTPS_FUNC1_POWER_STATE_SHIFT: c_int = 6;
pub const E1000_FACTPS_LAN1_VALID: c_uint = 0x00000100;
pub const E1000_FACTPS_FUNC1_AUX_EN: c_uint = 0x00000200;
pub const E1000_FACTPS_FUNC2_POWER_STATE_MASK: c_uint = 0x00003000;
pub const E1000_FACTPS_FUNC2_POWER_STATE_SHIFT: c_int = 12;
pub const E1000_FACTPS_IDE_ENABLE: c_uint = 0x00004000;
pub const E1000_FACTPS_FUNC2_AUX_EN: c_uint = 0x00008000;
pub const E1000_FACTPS_FUNC3_POWER_STATE_MASK: c_uint = 0x000C0000;
pub const E1000_FACTPS_FUNC3_POWER_STATE_SHIFT: c_int = 18;
pub const E1000_FACTPS_SP_ENABLE: c_uint = 0x00100000;
pub const E1000_FACTPS_FUNC3_AUX_EN: c_uint = 0x00200000;
pub const E1000_FACTPS_FUNC4_POWER_STATE_MASK: c_uint = 0x03000000;
pub const E1000_FACTPS_FUNC4_POWER_STATE_SHIFT: c_int = 24;
pub const E1000_FACTPS_IPMI_ENABLE: c_uint = 0x04000000;
pub const E1000_FACTPS_FUNC4_AUX_EN: c_uint = 0x08000000;
pub const E1000_FACTPS_MNGCG: c_uint = 0x20000000;
pub const E1000_FACTPS_LAN_FUNC_SEL: c_uint = 0x40000000;
pub const E1000_FACTPS_PM_STATE_CHANGED: c_uint = 0x80000000;
// PCI-Ex Config Space
pub const PCI_EX_LINK_STATUS: c_uint = 0x12;
pub const PCI_EX_LINK_WIDTH_MASK: c_uint = 0x3F0;
pub const PCI_EX_LINK_WIDTH_SHIFT: c_int = 4;
// EEPROM Commands - Microwire
pub const EEPROM_READ_OPCODE_MICROWIRE: c_uint = 0x6	/* EEPROM read opcode */;
pub const EEPROM_WRITE_OPCODE_MICROWIRE: c_uint = 0x5	/* EEPROM write opcode */;
pub const EEPROM_ERASE_OPCODE_MICROWIRE: c_uint = 0x7	/* EEPROM erase opcode */;
pub const EEPROM_EWEN_OPCODE_MICROWIRE: c_uint = 0x13	/* EEPROM erase/write enable */;
pub const EEPROM_EWDS_OPCODE_MICROWIRE: c_uint = 0x10	/* EEPROM erase/write disable */;
// EEPROM Commands - SPI

pub const EEPROM_READ_OPCODE_SPI: c_uint = 0x03	/* EEPROM read opcode */;
pub const EEPROM_WRITE_OPCODE_SPI: c_uint = 0x02	/* EEPROM write opcode */;
pub const EEPROM_A8_OPCODE_SPI: c_uint = 0x08	/* opcode bit-3 = address bit-8 */;
pub const EEPROM_WREN_OPCODE_SPI: c_uint = 0x06	/* EEPROM set Write Enable latch */;
pub const EEPROM_WRDI_OPCODE_SPI: c_uint = 0x04	/* EEPROM reset Write Enable latch */;
pub const EEPROM_RDSR_OPCODE_SPI: c_uint = 0x05	/* EEPROM read Status register */;
pub const EEPROM_WRSR_OPCODE_SPI: c_uint = 0x01	/* EEPROM write Status register */;
pub const EEPROM_ERASE4K_OPCODE_SPI: c_uint = 0x20	/* EEPROM ERASE 4KB */;
pub const EEPROM_ERASE64K_OPCODE_SPI: c_uint = 0xD8	/* EEPROM ERASE 64KB */;
pub const EEPROM_ERASE256_OPCODE_SPI: c_uint = 0xDB	/* EEPROM ERASE 256B */;
// EEPROM Size definitions
pub const EEPROM_WORD_SIZE_SHIFT: c_int = 6;
pub const EEPROM_SIZE_SHIFT: c_int = 10;
pub const EEPROM_SIZE_MASK: c_uint = 0x1C00;
// EEPROM Word Offsets
pub const EEPROM_COMPAT: c_uint = 0x0003;
pub const EEPROM_ID_LED_SETTINGS: c_uint = 0x0004;
pub const EEPROM_VERSION: c_uint = 0x0005;
pub const EEPROM_SERDES_AMPLITUDE: c_uint = 0x0006	/* For SERDES output amplitude adjustment. */;
pub const EEPROM_PHY_CLASS_WORD: c_uint = 0x0007;
pub const EEPROM_INIT_CONTROL1_REG: c_uint = 0x000A;
pub const EEPROM_INIT_CONTROL2_REG: c_uint = 0x000F;
pub const EEPROM_SWDEF_PINS_CTRL_PORT_1: c_uint = 0x0010;
pub const EEPROM_INIT_CONTROL3_PORT_B: c_uint = 0x0014;
pub const EEPROM_INIT_3GIO_3: c_uint = 0x001A;
pub const EEPROM_SWDEF_PINS_CTRL_PORT_0: c_uint = 0x0020;
pub const EEPROM_INIT_CONTROL3_PORT_A: c_uint = 0x0024;
pub const EEPROM_CFG: c_uint = 0x0012;
pub const EEPROM_FLASH_VERSION: c_uint = 0x0032;
pub const EEPROM_CHECKSUM_REG: c_uint = 0x003F;
pub const E1000_EEPROM_CFG_DONE: c_uint = 0x00040000	/* MNG config cycle done */;
pub const E1000_EEPROM_CFG_DONE_PORT_1: c_uint = 0x00080000	/* ...for second port */;
// Word definitions for ID LED Settings
pub const ID_LED_RESERVED_0000: c_uint = 0x0000;
pub const ID_LED_RESERVED_FFFF: c_uint = 0xFFFF;

pub const ID_LED_DEF1_DEF2: c_uint = 0x1;
pub const ID_LED_DEF1_ON2: c_uint = 0x2;
pub const ID_LED_DEF1_OFF2: c_uint = 0x3;
pub const ID_LED_ON1_DEF2: c_uint = 0x4;
pub const ID_LED_ON1_ON2: c_uint = 0x5;
pub const ID_LED_ON1_OFF2: c_uint = 0x6;
pub const ID_LED_OFF1_DEF2: c_uint = 0x7;
pub const ID_LED_OFF1_ON2: c_uint = 0x8;
pub const ID_LED_OFF1_OFF2: c_uint = 0x9;
pub const IGP_ACTIVITY_LED_MASK: c_uint = 0xFFFFF0FF;
pub const IGP_ACTIVITY_LED_ENABLE: c_uint = 0x0300;
pub const IGP_LED3_MODE: c_uint = 0x07000000;
// Mask bits for SERDES amplitude adjustment in Word 6 of the EEPROM
pub const EEPROM_SERDES_AMPLITUDE_MASK: c_uint = 0x000F;
// Mask bit for PHY class in Word 7 of the EEPROM
pub const EEPROM_PHY_CLASS_A: c_uint = 0x8000;
// Mask bits for fields in Word 0x0a of the EEPROM
pub const EEPROM_WORD0A_ILOS: c_uint = 0x0010;
pub const EEPROM_WORD0A_SWDPIO: c_uint = 0x01E0;
pub const EEPROM_WORD0A_LRST: c_uint = 0x0200;
pub const EEPROM_WORD0A_FD: c_uint = 0x0400;
pub const EEPROM_WORD0A_66MHZ: c_uint = 0x0800;
// Mask bits for fields in Word 0x0f of the EEPROM
pub const EEPROM_WORD0F_PAUSE_MASK: c_uint = 0x3000;
pub const EEPROM_WORD0F_PAUSE: c_uint = 0x1000;
pub const EEPROM_WORD0F_ASM_DIR: c_uint = 0x2000;
pub const EEPROM_WORD0F_ANE: c_uint = 0x0800;
pub const EEPROM_WORD0F_SWPDIO_EXT: c_uint = 0x00F0;
pub const EEPROM_WORD0F_LPLU: c_uint = 0x0001;
// Mask bits for fields in Word 0x10/0x20 of the EEPROM
pub const EEPROM_WORD1020_GIGA_DISABLE: c_uint = 0x0010;
pub const EEPROM_WORD1020_GIGA_DISABLE_NON_D0A: c_uint = 0x0008;
// Mask bits for fields in Word 0x1a of the EEPROM
pub const EEPROM_WORD1A_ASPM_MASK: c_uint = 0x000C;
// For checksumming, the sum of all words in the EEPROM should equal 0xBABA.
pub const EEPROM_SUM: c_uint = 0xBABA;
// EEPROM Map defines (WORD OFFSETS)
pub const EEPROM_NODE_ADDRESS_BYTE_0: c_int = 0;
pub const EEPROM_PBA_BYTE_1: c_int = 8;
pub const EEPROM_RESERVED_WORD: c_uint = 0xFFFF;
// EEPROM Map Sizes (Byte Counts)
pub const PBA_SIZE: c_int = 4;
// Collision related configuration parameters
pub const E1000_COLLISION_THRESHOLD: c_int = 15;
pub const E1000_CT_SHIFT: c_int = 4;
// Collision distance is a 0-based value that applies to
// half-duplex-capable hardware only.
pub const E1000_COLLISION_DISTANCE: c_int = 63;
pub const E1000_COLLISION_DISTANCE_82542: c_int = 64;

pub const E1000_COLD_SHIFT: c_int = 12;
// Number of Transmit and Receive Descriptors must be a multiple of 8
pub const REQ_TX_DESCRIPTOR_MULTIPLE: c_int = 8;
pub const REQ_RX_DESCRIPTOR_MULTIPLE: c_int = 8;
// Default values for the transmit IPG register
pub const DEFAULT_82542_TIPG_IPGT: c_int = 10;
pub const DEFAULT_82543_TIPG_IPGT_FIBER: c_int = 9;
pub const DEFAULT_82543_TIPG_IPGT_COPPER: c_int = 8;
pub const E1000_TIPG_IPGT_MASK: c_uint = 0x000003FF;
pub const E1000_TIPG_IPGR1_MASK: c_uint = 0x000FFC00;
pub const E1000_TIPG_IPGR2_MASK: c_uint = 0x3FF00000;
pub const DEFAULT_82542_TIPG_IPGR1: c_int = 2;
pub const DEFAULT_82543_TIPG_IPGR1: c_int = 8;
pub const E1000_TIPG_IPGR1_SHIFT: c_int = 10;
pub const DEFAULT_82542_TIPG_IPGR2: c_int = 10;
pub const DEFAULT_82543_TIPG_IPGR2: c_int = 6;
pub const E1000_TIPG_IPGR2_SHIFT: c_int = 20;
pub const E1000_TXDMAC_DPP: c_uint = 0x00000001;
// Adaptive IFS defines
pub const TX_THRESHOLD_START: c_int = 8;
pub const TX_THRESHOLD_INCREMENT: c_int = 10;
pub const TX_THRESHOLD_DECREMENT: c_int = 1;
pub const TX_THRESHOLD_STOP: c_int = 190;
pub const TX_THRESHOLD_DISABLE: c_int = 0;
pub const TX_THRESHOLD_TIMER_MS: c_int = 10000;
pub const MIN_NUM_XMITS: c_int = 1000;
pub const IFS_MAX: c_int = 80;
pub const IFS_STEP: c_int = 10;
pub const IFS_MIN: c_int = 40;
pub const IFS_RATIO: c_int = 4;
// Extended Configuration Control and Size
pub const E1000_EXTCNF_CTRL_PCIE_WRITE_ENABLE: c_uint = 0x00000001;
pub const E1000_EXTCNF_CTRL_PHY_WRITE_ENABLE: c_uint = 0x00000002;
pub const E1000_EXTCNF_CTRL_D_UD_ENABLE: c_uint = 0x00000004;
pub const E1000_EXTCNF_CTRL_D_UD_LATENCY: c_uint = 0x00000008;
pub const E1000_EXTCNF_CTRL_D_UD_OWNER: c_uint = 0x00000010;
pub const E1000_EXTCNF_CTRL_MDIO_SW_OWNERSHIP: c_uint = 0x00000020;
pub const E1000_EXTCNF_CTRL_MDIO_HW_OWNERSHIP: c_uint = 0x00000040;
pub const E1000_EXTCNF_CTRL_EXT_CNF_POINTER: c_uint = 0x0FFF0000;
pub const E1000_EXTCNF_SIZE_EXT_PHY_LENGTH: c_uint = 0x000000FF;
pub const E1000_EXTCNF_SIZE_EXT_DOCK_LENGTH: c_uint = 0x0000FF00;
pub const E1000_EXTCNF_SIZE_EXT_PCIE_LENGTH: c_uint = 0x00FF0000;
pub const E1000_EXTCNF_CTRL_LCD_WRITE_ENABLE: c_uint = 0x00000001;
pub const E1000_EXTCNF_CTRL_SWFLAG: c_uint = 0x00000020;
// PBA constants
pub const E1000_PBA_8K: c_uint = 0x0008	/* 8KB, default Rx allocation */;
pub const E1000_PBA_12K: c_uint = 0x000C	/* 12KB, default Rx allocation */;
pub const E1000_PBA_16K: c_uint = 0x0010	/* 16KB, default TX allocation */;
pub const E1000_PBA_20K: c_uint = 0x0014;
pub const E1000_PBA_22K: c_uint = 0x0016;
pub const E1000_PBA_24K: c_uint = 0x0018;
pub const E1000_PBA_30K: c_uint = 0x001E;
pub const E1000_PBA_32K: c_uint = 0x0020;
pub const E1000_PBA_34K: c_uint = 0x0022;
pub const E1000_PBA_38K: c_uint = 0x0026;
pub const E1000_PBA_40K: c_uint = 0x0028;
pub const E1000_PBA_48K: c_uint = 0x0030	/* 48KB, default RX allocation */;

// Flow Control Constants
pub const FLOW_CONTROL_ADDRESS_LOW: c_uint = 0x00C28001;
pub const FLOW_CONTROL_ADDRESS_HIGH: c_uint = 0x00000100;
pub const FLOW_CONTROL_TYPE: c_uint = 0x8808;
// The historical defaults for the flow control values are given below.

// PCIX Config space
pub const PCIX_COMMAND_REGISTER: c_uint = 0xE6;
pub const PCIX_STATUS_REGISTER_LO: c_uint = 0xE8;
pub const PCIX_STATUS_REGISTER_HI: c_uint = 0xEA;
pub const PCIX_COMMAND_MMRBC_MASK: c_uint = 0x000C;
pub const PCIX_COMMAND_MMRBC_SHIFT: c_uint = 0x2;
pub const PCIX_STATUS_HI_MMRBC_MASK: c_uint = 0x0060;
pub const PCIX_STATUS_HI_MMRBC_SHIFT: c_uint = 0x5;
pub const PCIX_STATUS_HI_MMRBC_4K: c_uint = 0x3;
pub const PCIX_STATUS_HI_MMRBC_2K: c_uint = 0x2;
// Number of bits required to shift right the "pause" bits from the
// EEPROM (bits 13:12) to the "pause" (bits 8:7) field in the TXCW register.
//
pub const PAUSE_SHIFT: c_int = 5;
// Number of bits required to shift left the "SWDPIO" bits from the
// EEPROM (bits 8:5) to the "SWDPIO" (bits 25:22) field in the CTRL register.
//
pub const SWDPIO_SHIFT: c_int = 17;
// Number of bits required to shift left the "SWDPIO_EXT" bits from the
// EEPROM word F (bits 7:4) to the bits 11:8 of The Extended CTRL register.
//
pub const SWDPIO__EXT_SHIFT: c_int = 4;
// Number of bits required to shift left the "ILOS" bit from the EEPROM
// (bit 4) to the "ILOS" (bit 7) field in the CTRL register.
//
pub const ILOS_SHIFT: c_int = 3;

// Number of milliseconds we wait for auto-negotiation to complete
pub const LINK_UP_TIMEOUT: c_int = 500;
// Number of milliseconds we wait for Eeprom auto read bit done after MAC reset
pub const AUTO_READ_DONE_TIMEOUT: c_int = 10;
// Number of milliseconds we wait for PHY configuration done after MAC reset
pub const PHY_CFG_TIMEOUT: c_int = 100;

// The carrier extension symbol, as received by the NIC.
pub const CARRIER_EXTENSION: c_uint = 0x0F;
// TBI_ACCEPT macro definition:
//
// This macro requires:
// adapter = a pointer to struct e1000_hw
// status = the 8 bit status field of the RX descriptor with EOP set
// error = the 8 bit error field of the RX descriptor with EOP set
// length = the sum of all the length fields of the RX descriptors that
// make up the current frame
// last_byte = the last byte of the frame DMAed by the hardware
// max_frame_length = the maximum frame length we want to accept.
// min_frame_length = the minimum frame length we want to accept.
//
// This macro is a conditional that should be used in the interrupt
// handler's Rx processing routine when RxErrors have been detected.
//
// Typical use:
// ...
// if (TBI_ACCEPT) {
// accept_frame = true;
// e1000_tbi_adjust_stats(adapter, MacAddress);
// frame_length--;
// } else {
// accept_frame = false;
// }
// ...
//

// Structures, enums, and macros for the PHY
// Bit definitions for the Management Data IO (MDIO) and Management Data
// Clock (MDC) pins in the Device Control Register.
//

// PHY 1000 MII Register/Bit Definitions
// PHY Registers defined by IEEE
pub const PHY_CTRL: c_uint = 0x00	/* Control Register */;
pub const PHY_STATUS: c_uint = 0x01	/* Status Register */;
pub const PHY_ID1: c_uint = 0x02	/* Phy Id Reg (word 1) */;
pub const PHY_ID2: c_uint = 0x03	/* Phy Id Reg (word 2) */;
pub const PHY_AUTONEG_ADV: c_uint = 0x04	/* Autoneg Advertisement */;
pub const PHY_LP_ABILITY: c_uint = 0x05	/* Link Partner Ability (Base Page) */;
pub const PHY_AUTONEG_EXP: c_uint = 0x06	/* Autoneg Expansion Reg */;
pub const PHY_NEXT_PAGE_TX: c_uint = 0x07	/* Next Page TX */;
pub const PHY_LP_NEXT_PAGE: c_uint = 0x08	/* Link Partner Next Page */;
pub const PHY_1000T_CTRL: c_uint = 0x09	/* 1000Base-T Control Reg */;
pub const PHY_1000T_STATUS: c_uint = 0x0A	/* 1000Base-T Status Reg */;
pub const PHY_EXT_STATUS: c_uint = 0x0F	/* Extended Status Reg */;
pub const MAX_PHY_REG_ADDRESS: c_uint = 0x1F	/* 5 bit address bus (0-0x1F) */;
pub const MAX_PHY_MULTI_PAGE_REG: c_uint = 0xF	/* Registers equal on all pages */;
// M88E1000 Specific Registers
pub const M88E1000_PHY_SPEC_CTRL: c_uint = 0x10	/* PHY Specific Control Register */;
pub const M88E1000_PHY_SPEC_STATUS: c_uint = 0x11	/* PHY Specific Status Register */;
pub const M88E1000_INT_ENABLE: c_uint = 0x12	/* Interrupt Enable Register */;
pub const M88E1000_INT_STATUS: c_uint = 0x13	/* Interrupt Status Register */;
pub const M88E1000_EXT_PHY_SPEC_CTRL: c_uint = 0x14	/* Extended PHY Specific Control */;
pub const M88E1000_RX_ERR_CNTR: c_uint = 0x15	/* Receive Error Counter */;
pub const M88E1000_PHY_EXT_CTRL: c_uint = 0x1A	/* PHY extend control register */;
pub const M88E1000_PHY_PAGE_SELECT: c_uint = 0x1D	/* Reg 29 for page number setting */;
pub const M88E1000_PHY_GEN_CONTROL: c_uint = 0x1E	/* Its meaning depends on reg 29 */;
pub const M88E1000_PHY_VCO_REG_BIT8: c_uint = 0x100	/* Bits 8 & 11 are adjusted for */;
pub const M88E1000_PHY_VCO_REG_BIT11: c_uint = 0x800	/* improved BER performance */;
pub const IGP01E1000_IEEE_REGS_PAGE: c_uint = 0x0000;
pub const IGP01E1000_IEEE_RESTART_AUTONEG: c_uint = 0x3300;
pub const IGP01E1000_IEEE_FORCE_GIGA: c_uint = 0x0140;
// IGP01E1000 Specific Registers
pub const IGP01E1000_PHY_PORT_CONFIG: c_uint = 0x10	/* PHY Specific Port Config Register */;
pub const IGP01E1000_PHY_PORT_STATUS: c_uint = 0x11	/* PHY Specific Status Register */;
pub const IGP01E1000_PHY_PORT_CTRL: c_uint = 0x12	/* PHY Specific Control Register */;
pub const IGP01E1000_PHY_LINK_HEALTH: c_uint = 0x13	/* PHY Link Health Register */;
pub const IGP01E1000_GMII_FIFO: c_uint = 0x14	/* GMII FIFO Register */;
pub const IGP01E1000_PHY_CHANNEL_QUALITY: c_uint = 0x15	/* PHY Channel Quality Register */;
pub const IGP02E1000_PHY_POWER_MGMT: c_uint = 0x19;
pub const IGP01E1000_PHY_PAGE_SELECT: c_uint = 0x1F	/* PHY Page Select Core Register */;
// IGP01E1000 AGC Registers - stores the cable length values
pub const IGP01E1000_PHY_AGC_A: c_uint = 0x1172;
pub const IGP01E1000_PHY_AGC_B: c_uint = 0x1272;
pub const IGP01E1000_PHY_AGC_C: c_uint = 0x1472;
pub const IGP01E1000_PHY_AGC_D: c_uint = 0x1872;
// IGP02E1000 AGC Registers for cable length values
pub const IGP02E1000_PHY_AGC_A: c_uint = 0x11B1;
pub const IGP02E1000_PHY_AGC_B: c_uint = 0x12B1;
pub const IGP02E1000_PHY_AGC_C: c_uint = 0x14B1;
pub const IGP02E1000_PHY_AGC_D: c_uint = 0x18B1;
// IGP01E1000 DSP Reset Register
pub const IGP01E1000_PHY_DSP_RESET: c_uint = 0x1F33;
pub const IGP01E1000_PHY_DSP_SET: c_uint = 0x1F71;
pub const IGP01E1000_PHY_DSP_FFE: c_uint = 0x1F35;
pub const IGP01E1000_PHY_CHANNEL_NUM: c_int = 4;
pub const IGP02E1000_PHY_CHANNEL_NUM: c_int = 4;
pub const IGP01E1000_PHY_AGC_PARAM_A: c_uint = 0x1171;
pub const IGP01E1000_PHY_AGC_PARAM_B: c_uint = 0x1271;
pub const IGP01E1000_PHY_AGC_PARAM_C: c_uint = 0x1471;
pub const IGP01E1000_PHY_AGC_PARAM_D: c_uint = 0x1871;
pub const IGP01E1000_PHY_EDAC_MU_INDEX: c_uint = 0xC000;
pub const IGP01E1000_PHY_EDAC_SIGN_EXT_9_BITS: c_uint = 0x8000;
pub const IGP01E1000_PHY_ANALOG_TX_STATE: c_uint = 0x2890;
pub const IGP01E1000_PHY_ANALOG_CLASS_A: c_uint = 0x2000;
pub const IGP01E1000_PHY_FORCE_ANALOG_ENABLE: c_uint = 0x0004;
pub const IGP01E1000_PHY_DSP_FFE_CM_CP: c_uint = 0x0069;
pub const IGP01E1000_PHY_DSP_FFE_DEFAULT: c_uint = 0x002A;
// IGP01E1000 PCS Initialization register - stores the polarity status when
// speed = 1000 Mbps.
pub const IGP01E1000_PHY_PCS_INIT_REG: c_uint = 0x00B4;
pub const IGP01E1000_PHY_PCS_CTRL_REG: c_uint = 0x00B5;
pub const IGP01E1000_ANALOG_REGS_PAGE: c_uint = 0x20C0;
// PHY Control Register
pub const MII_CR_SPEED_SELECT_MSB: c_uint = 0x0040	/* bits 6,13: 10=1000, 01=100, 00=10 */;
pub const MII_CR_COLL_TEST_ENABLE: c_uint = 0x0080	/* Collision test enable */;
pub const MII_CR_FULL_DUPLEX: c_uint = 0x0100	/* FDX =1, half duplex =0 */;
pub const MII_CR_RESTART_AUTO_NEG: c_uint = 0x0200	/* Restart auto negotiation */;
pub const MII_CR_ISOLATE: c_uint = 0x0400	/* Isolate PHY from MII */;
pub const MII_CR_POWER_DOWN: c_uint = 0x0800	/* Power down */;
pub const MII_CR_AUTO_NEG_EN: c_uint = 0x1000	/* Auto Neg Enable */;
pub const MII_CR_SPEED_SELECT_LSB: c_uint = 0x2000	/* bits 6,13: 10=1000, 01=100, 00=10 */;
pub const MII_CR_LOOPBACK: c_uint = 0x4000	/* 0 = normal, 1 = loopback */;
pub const MII_CR_RESET: c_uint = 0x8000	/* 0 = normal, 1 = PHY reset */;
// PHY Status Register
pub const MII_SR_EXTENDED_CAPS: c_uint = 0x0001	/* Extended register capabilities */;
pub const MII_SR_JABBER_DETECT: c_uint = 0x0002	/* Jabber Detected */;
pub const MII_SR_LINK_STATUS: c_uint = 0x0004	/* Link Status 1 = link */;
pub const MII_SR_AUTONEG_CAPS: c_uint = 0x0008	/* Auto Neg Capable */;
pub const MII_SR_REMOTE_FAULT: c_uint = 0x0010	/* Remote Fault Detect */;
pub const MII_SR_AUTONEG_COMPLETE: c_uint = 0x0020	/* Auto Neg Complete */;
pub const MII_SR_PREAMBLE_SUPPRESS: c_uint = 0x0040	/* Preamble may be suppressed */;
pub const MII_SR_EXTENDED_STATUS: c_uint = 0x0100	/* Ext. status info in Reg 0x0F */;
pub const MII_SR_100T2_HD_CAPS: c_uint = 0x0200	/* 100T2 Half Duplex Capable */;
pub const MII_SR_100T2_FD_CAPS: c_uint = 0x0400	/* 100T2 Full Duplex Capable */;
pub const MII_SR_10T_HD_CAPS: c_uint = 0x0800	/* 10T   Half Duplex Capable */;
pub const MII_SR_10T_FD_CAPS: c_uint = 0x1000	/* 10T   Full Duplex Capable */;
pub const MII_SR_100X_HD_CAPS: c_uint = 0x2000	/* 100X  Half Duplex Capable */;
pub const MII_SR_100X_FD_CAPS: c_uint = 0x4000	/* 100X  Full Duplex Capable */;
pub const MII_SR_100T4_CAPS: c_uint = 0x8000	/* 100T4 Capable */;
// Autoneg Advertisement Register
pub const NWAY_AR_SELECTOR_FIELD: c_uint = 0x0001	/* indicates IEEE 802.3 CSMA/CD */;
pub const NWAY_AR_10T_HD_CAPS: c_uint = 0x0020	/* 10T   Half Duplex Capable */;
pub const NWAY_AR_10T_FD_CAPS: c_uint = 0x0040	/* 10T   Full Duplex Capable */;
pub const NWAY_AR_100TX_HD_CAPS: c_uint = 0x0080	/* 100TX Half Duplex Capable */;
pub const NWAY_AR_100TX_FD_CAPS: c_uint = 0x0100	/* 100TX Full Duplex Capable */;
pub const NWAY_AR_100T4_CAPS: c_uint = 0x0200	/* 100T4 Capable */;
pub const NWAY_AR_PAUSE: c_uint = 0x0400	/* Pause operation desired */;
pub const NWAY_AR_ASM_DIR: c_uint = 0x0800	/* Asymmetric Pause Direction bit */;
pub const NWAY_AR_REMOTE_FAULT: c_uint = 0x2000	/* Remote Fault detected */;
pub const NWAY_AR_NEXT_PAGE: c_uint = 0x8000	/* Next Page ability supported */;
// Link Partner Ability Register (Base Page)
pub const NWAY_LPAR_SELECTOR_FIELD: c_uint = 0x0000	/* LP protocol selector field */;
pub const NWAY_LPAR_10T_HD_CAPS: c_uint = 0x0020	/* LP is 10T   Half Duplex Capable */;
pub const NWAY_LPAR_10T_FD_CAPS: c_uint = 0x0040	/* LP is 10T   Full Duplex Capable */;
pub const NWAY_LPAR_100TX_HD_CAPS: c_uint = 0x0080	/* LP is 100TX Half Duplex Capable */;
pub const NWAY_LPAR_100TX_FD_CAPS: c_uint = 0x0100	/* LP is 100TX Full Duplex Capable */;
pub const NWAY_LPAR_100T4_CAPS: c_uint = 0x0200	/* LP is 100T4 Capable */;
pub const NWAY_LPAR_PAUSE: c_uint = 0x0400	/* LP Pause operation desired */;
pub const NWAY_LPAR_ASM_DIR: c_uint = 0x0800	/* LP Asymmetric Pause Direction bit */;
pub const NWAY_LPAR_REMOTE_FAULT: c_uint = 0x2000	/* LP has detected Remote Fault */;
pub const NWAY_LPAR_ACKNOWLEDGE: c_uint = 0x4000	/* LP has rx'd link code word */;
pub const NWAY_LPAR_NEXT_PAGE: c_uint = 0x8000	/* Next Page ability supported */;
// Autoneg Expansion Register
pub const NWAY_ER_LP_NWAY_CAPS: c_uint = 0x0001	/* LP has Auto Neg Capability */;
pub const NWAY_ER_PAGE_RXD: c_uint = 0x0002	/* LP is 10T   Half Duplex Capable */;
pub const NWAY_ER_NEXT_PAGE_CAPS: c_uint = 0x0004	/* LP is 10T   Full Duplex Capable */;
pub const NWAY_ER_LP_NEXT_PAGE_CAPS: c_uint = 0x0008	/* LP is 100TX Half Duplex Capable */;
pub const NWAY_ER_PAR_DETECT_FAULT: c_uint = 0x0010	/* LP is 100TX Full Duplex Capable */;
// Next Page TX Register
pub const NPTX_MSG_CODE_FIELD: c_uint = 0x0001	/* NP msg code or unformatted data */;
pub const NPTX_TOGGLE: c_uint = 0x0800	/* Toggles between exchanges;
// of different NP
//
pub const NPTX_ACKNOWLDGE2: c_uint = 0x1000	/* 1 = will comply with msg;
// 0 = cannot comply with msg
//
pub const NPTX_MSG_PAGE: c_uint = 0x2000	/* formatted(1)/unformatted(0) pg */;
pub const NPTX_NEXT_PAGE: c_uint = 0x8000	/* 1 = addition NP will follow;
// 0 = sending last NP
//
// Link Partner Next Page Register
pub const LP_RNPR_MSG_CODE_FIELD: c_uint = 0x0001	/* NP msg code or unformatted data */;
pub const LP_RNPR_TOGGLE: c_uint = 0x0800	/* Toggles between exchanges;
// of different NP
//
pub const LP_RNPR_ACKNOWLDGE2: c_uint = 0x1000	/* 1 = will comply with msg;
// 0 = cannot comply with msg
//
pub const LP_RNPR_MSG_PAGE: c_uint = 0x2000	/* formatted(1)/unformatted(0) pg */;
pub const LP_RNPR_ACKNOWLDGE: c_uint = 0x4000	/* 1 = ACK / 0 = NO ACK */;
pub const LP_RNPR_NEXT_PAGE: c_uint = 0x8000	/* 1 = addition NP will follow;
// 0 = sending last NP
//
// 1000BASE-T Control Register
pub const CR_1000T_ASYM_PAUSE: c_uint = 0x0080	/* Advertise asymmetric pause bit */;
pub const CR_1000T_HD_CAPS: c_uint = 0x0100	/* Advertise 1000T HD capability */;
pub const CR_1000T_FD_CAPS: c_uint = 0x0200	/* Advertise 1000T FD capability  */;
pub const CR_1000T_REPEATER_DTE: c_uint = 0x0400	/* 1=Repeater/switch device port */;
// 0=DTE device
pub const CR_1000T_MS_VALUE: c_uint = 0x0800	/* 1=Configure PHY as Master */;
// 0=Configure PHY as Slave
pub const CR_1000T_MS_ENABLE: c_uint = 0x1000	/* 1=Master/Slave manual config value */;
// 0=Automatic Master/Slave config
pub const CR_1000T_TEST_MODE_NORMAL: c_uint = 0x0000	/* Normal Operation */;
pub const CR_1000T_TEST_MODE_1: c_uint = 0x2000	/* Transmit Waveform test */;
pub const CR_1000T_TEST_MODE_2: c_uint = 0x4000	/* Master Transmit Jitter test */;
pub const CR_1000T_TEST_MODE_3: c_uint = 0x6000	/* Slave Transmit Jitter test */;
pub const CR_1000T_TEST_MODE_4: c_uint = 0x8000	/* Transmitter Distortion test */;
// 1000BASE-T Status Register
pub const SR_1000T_IDLE_ERROR_CNT: c_uint = 0x00FF	/* Num idle errors since last read */;
pub const SR_1000T_ASYM_PAUSE_DIR: c_uint = 0x0100	/* LP asymmetric pause direction bit */;
pub const SR_1000T_LP_HD_CAPS: c_uint = 0x0400	/* LP is 1000T HD capable */;
pub const SR_1000T_LP_FD_CAPS: c_uint = 0x0800	/* LP is 1000T FD capable */;
pub const SR_1000T_REMOTE_RX_STATUS: c_uint = 0x1000	/* Remote receiver OK */;
pub const SR_1000T_LOCAL_RX_STATUS: c_uint = 0x2000	/* Local receiver OK */;
pub const SR_1000T_MS_CONFIG_RES: c_uint = 0x4000	/* 1=Local TX is Master, 0=Slave */;
pub const SR_1000T_MS_CONFIG_FAULT: c_uint = 0x8000	/* Master/Slave config fault */;
pub const SR_1000T_REMOTE_RX_STATUS_SHIFT: c_int = 12;
pub const SR_1000T_LOCAL_RX_STATUS_SHIFT: c_int = 13;
pub const SR_1000T_PHY_EXCESSIVE_IDLE_ERR_COUNT: c_int = 5;
pub const FFE_IDLE_ERR_COUNT_TIMEOUT_20: c_int = 20;
pub const FFE_IDLE_ERR_COUNT_TIMEOUT_100: c_int = 100;
// Extended Status Register
pub const IEEE_ESR_1000T_HD_CAPS: c_uint = 0x1000	/* 1000T HD capable */;
pub const IEEE_ESR_1000T_FD_CAPS: c_uint = 0x2000	/* 1000T FD capable */;
pub const IEEE_ESR_1000X_HD_CAPS: c_uint = 0x4000	/* 1000X HD capable */;
pub const IEEE_ESR_1000X_FD_CAPS: c_uint = 0x8000	/* 1000X FD capable */;
pub const PHY_TX_POLARITY_MASK: c_uint = 0x0100	/* register 10h bit 8 (polarity bit) */;

pub const AUTO_POLARITY_DISABLE: c_uint = 0x0010	/* register 11h bit 4 */;
// (0=enable, 1=disable)
// M88E1000 PHY Specific Control Register
pub const M88E1000_PSCR_JABBER_DISABLE: c_uint = 0x0001	/* 1=Jabber Function disabled */;
pub const M88E1000_PSCR_POLARITY_REVERSAL: c_uint = 0x0002	/* 1=Polarity Reversal enabled */;
pub const M88E1000_PSCR_SQE_TEST: c_uint = 0x0004	/* 1=SQE Test enabled */;
pub const M88E1000_PSCR_CLK125_DISABLE: c_uint = 0x0010	/* 1=CLK125 low,;
// 0=CLK125 toggling
//
pub const M88E1000_PSCR_MDI_MANUAL_MODE: c_uint = 0x0000	/* MDI Crossover Mode bits 6:5 */;
// Manual MDI configuration
pub const M88E1000_PSCR_MDIX_MANUAL_MODE: c_uint = 0x0020	/* Manual MDIX configuration */;
pub const M88E1000_PSCR_AUTO_X_1000T: c_uint = 0x0040	/* 1000BASE-T: Auto crossover,;
// 100BASE-TX/10BASE-T:
// MDI Mode
//
pub const M88E1000_PSCR_AUTO_X_MODE: c_uint = 0x0060	/* Auto crossover enabled;
// all speeds.
//
pub const M88E1000_PSCR_10BT_EXT_DIST_ENABLE: c_uint = 0x0080;
// 1=Enable Extended 10BASE-T distance
// (Lower 10BASE-T RX Threshold)
// 0=Normal 10BASE-T RX Threshold
pub const M88E1000_PSCR_MII_5BIT_ENABLE: c_uint = 0x0100;
// 1=5-Bit interface in 100BASE-TX
// 0=MII interface in 100BASE-TX
pub const M88E1000_PSCR_SCRAMBLER_DISABLE: c_uint = 0x0200	/* 1=Scrambler disable */;
pub const M88E1000_PSCR_FORCE_LINK_GOOD: c_uint = 0x0400	/* 1=Force link good */;
pub const M88E1000_PSCR_ASSERT_CRS_ON_TX: c_uint = 0x0800	/* 1=Assert CRS on Transmit */;
pub const M88E1000_PSCR_POLARITY_REVERSAL_SHIFT: c_int = 1;
pub const M88E1000_PSCR_AUTO_X_MODE_SHIFT: c_int = 5;
pub const M88E1000_PSCR_10BT_EXT_DIST_ENABLE_SHIFT: c_int = 7;
// M88E1000 PHY Specific Status Register
pub const M88E1000_PSSR_JABBER: c_uint = 0x0001	/* 1=Jabber */;
pub const M88E1000_PSSR_REV_POLARITY: c_uint = 0x0002	/* 1=Polarity reversed */;
pub const M88E1000_PSSR_DOWNSHIFT: c_uint = 0x0020	/* 1=Downshifted */;
pub const M88E1000_PSSR_MDIX: c_uint = 0x0040	/* 1=MDIX; 0=MDI */;
pub const M88E1000_PSSR_CABLE_LENGTH: c_uint = 0x0380	/* 0=<50M;1=50-80M;2=80-110M;;
// 3=110-140M;4=>140M
pub const M88E1000_PSSR_LINK: c_uint = 0x0400	/* 1=Link up, 0=Link down */;
pub const M88E1000_PSSR_SPD_DPLX_RESOLVED: c_uint = 0x0800	/* 1=Speed & Duplex resolved */;
pub const M88E1000_PSSR_PAGE_RCVD: c_uint = 0x1000	/* 1=Page received */;
pub const M88E1000_PSSR_DPLX: c_uint = 0x2000	/* 1=Duplex 0=Half Duplex */;
pub const M88E1000_PSSR_SPEED: c_uint = 0xC000	/* Speed, bits 14:15 */;
pub const M88E1000_PSSR_10MBS: c_uint = 0x0000	/* 00=10Mbs */;
pub const M88E1000_PSSR_100MBS: c_uint = 0x4000	/* 01=100Mbs */;
pub const M88E1000_PSSR_1000MBS: c_uint = 0x8000	/* 10=1000Mbs */;
pub const M88E1000_PSSR_REV_POLARITY_SHIFT: c_int = 1;
pub const M88E1000_PSSR_DOWNSHIFT_SHIFT: c_int = 5;
pub const M88E1000_PSSR_MDIX_SHIFT: c_int = 6;
pub const M88E1000_PSSR_CABLE_LENGTH_SHIFT: c_int = 7;
// M88E1000 Extended PHY Specific Control Register
pub const M88E1000_EPSCR_FIBER_LOOPBACK: c_uint = 0x4000	/* 1=Fiber loopback */;
pub const M88E1000_EPSCR_DOWN_NO_IDLE: c_uint = 0x8000	/* 1=Lost lock detect enabled.;
// Will assert lost lock and bring
// link down if idle not seen
// within 1ms in 1000BASE-T
//
// Number of times we will attempt to autonegotiate before downshifting if we
// are the master
pub const M88E1000_EPSCR_MASTER_DOWNSHIFT_MASK: c_uint = 0x0C00;
pub const M88E1000_EPSCR_MASTER_DOWNSHIFT_1X: c_uint = 0x0000;
pub const M88E1000_EPSCR_MASTER_DOWNSHIFT_2X: c_uint = 0x0400;
pub const M88E1000_EPSCR_MASTER_DOWNSHIFT_3X: c_uint = 0x0800;
pub const M88E1000_EPSCR_MASTER_DOWNSHIFT_4X: c_uint = 0x0C00;
// Number of times we will attempt to autonegotiate before downshifting if we
// are the slave
pub const M88E1000_EPSCR_SLAVE_DOWNSHIFT_MASK: c_uint = 0x0300;
pub const M88E1000_EPSCR_SLAVE_DOWNSHIFT_DIS: c_uint = 0x0000;
pub const M88E1000_EPSCR_SLAVE_DOWNSHIFT_1X: c_uint = 0x0100;
pub const M88E1000_EPSCR_SLAVE_DOWNSHIFT_2X: c_uint = 0x0200;
pub const M88E1000_EPSCR_SLAVE_DOWNSHIFT_3X: c_uint = 0x0300;
pub const M88E1000_EPSCR_TX_CLK_2_5: c_uint = 0x0060	/* 2.5 MHz TX_CLK */;
pub const M88E1000_EPSCR_TX_CLK_25: c_uint = 0x0070	/* 25  MHz TX_CLK */;
pub const M88E1000_EPSCR_TX_CLK_0: c_uint = 0x0000	/* NO  TX_CLK */;
// M88EC018 Rev 2 specific DownShift settings
pub const M88EC018_EPSCR_DOWNSHIFT_COUNTER_MASK: c_uint = 0x0E00;
pub const M88EC018_EPSCR_DOWNSHIFT_COUNTER_1X: c_uint = 0x0000;
pub const M88EC018_EPSCR_DOWNSHIFT_COUNTER_2X: c_uint = 0x0200;
pub const M88EC018_EPSCR_DOWNSHIFT_COUNTER_3X: c_uint = 0x0400;
pub const M88EC018_EPSCR_DOWNSHIFT_COUNTER_4X: c_uint = 0x0600;
pub const M88EC018_EPSCR_DOWNSHIFT_COUNTER_5X: c_uint = 0x0800;
pub const M88EC018_EPSCR_DOWNSHIFT_COUNTER_6X: c_uint = 0x0A00;
pub const M88EC018_EPSCR_DOWNSHIFT_COUNTER_7X: c_uint = 0x0C00;
pub const M88EC018_EPSCR_DOWNSHIFT_COUNTER_8X: c_uint = 0x0E00;
// IGP01E1000 Specific Port Config Register - R/W
pub const IGP01E1000_PSCFR_AUTO_MDIX_PAR_DETECT: c_uint = 0x0010;
pub const IGP01E1000_PSCFR_PRE_EN: c_uint = 0x0020;
pub const IGP01E1000_PSCFR_SMART_SPEED: c_uint = 0x0080;
pub const IGP01E1000_PSCFR_DISABLE_TPLOOPBACK: c_uint = 0x0100;
pub const IGP01E1000_PSCFR_DISABLE_JABBER: c_uint = 0x0400;
pub const IGP01E1000_PSCFR_DISABLE_TRANSMIT: c_uint = 0x2000;
// IGP01E1000 Specific Port Status Register - R/O
pub const IGP01E1000_PSSR_AUTONEG_FAILED: c_uint = 0x0001	/* RO LH SC */;
pub const IGP01E1000_PSSR_POLARITY_REVERSED: c_uint = 0x0002;
pub const IGP01E1000_PSSR_CABLE_LENGTH: c_uint = 0x007C;
pub const IGP01E1000_PSSR_FULL_DUPLEX: c_uint = 0x0200;
pub const IGP01E1000_PSSR_LINK_UP: c_uint = 0x0400;
pub const IGP01E1000_PSSR_MDIX: c_uint = 0x0800;
pub const IGP01E1000_PSSR_SPEED_MASK: c_uint = 0xC000	/* speed bits mask */;
pub const IGP01E1000_PSSR_SPEED_10MBPS: c_uint = 0x4000;
pub const IGP01E1000_PSSR_SPEED_100MBPS: c_uint = 0x8000;
pub const IGP01E1000_PSSR_SPEED_1000MBPS: c_uint = 0xC000;
pub const IGP01E1000_PSSR_CABLE_LENGTH_SHIFT: c_uint = 0x0002	/* shift right 2 */;
pub const IGP01E1000_PSSR_MDIX_SHIFT: c_uint = 0x000B	/* shift right 11 */;
// IGP01E1000 Specific Port Control Register - R/W
pub const IGP01E1000_PSCR_TP_LOOPBACK: c_uint = 0x0010;
pub const IGP01E1000_PSCR_CORRECT_NC_SCMBLR: c_uint = 0x0200;
pub const IGP01E1000_PSCR_TEN_CRS_SELECT: c_uint = 0x0400;
pub const IGP01E1000_PSCR_FLIP_CHIP: c_uint = 0x0800;
pub const IGP01E1000_PSCR_AUTO_MDIX: c_uint = 0x1000;
pub const IGP01E1000_PSCR_FORCE_MDI_MDIX: c_uint = 0x2000	/* 0-MDI, 1-MDIX */;
// IGP01E1000 Specific Port Link Health Register
pub const IGP01E1000_PLHR_SS_DOWNGRADE: c_uint = 0x8000;
pub const IGP01E1000_PLHR_GIG_SCRAMBLER_ERROR: c_uint = 0x4000;
pub const IGP01E1000_PLHR_MASTER_FAULT: c_uint = 0x2000;
pub const IGP01E1000_PLHR_MASTER_RESOLUTION: c_uint = 0x1000;
pub const IGP01E1000_PLHR_GIG_REM_RCVR_NOK: c_uint = 0x0800	/* LH */;
pub const IGP01E1000_PLHR_IDLE_ERROR_CNT_OFLOW: c_uint = 0x0400	/* LH */;
pub const IGP01E1000_PLHR_DATA_ERR_1: c_uint = 0x0200	/* LH */;
pub const IGP01E1000_PLHR_DATA_ERR_0: c_uint = 0x0100;
pub const IGP01E1000_PLHR_AUTONEG_FAULT: c_uint = 0x0040;
pub const IGP01E1000_PLHR_AUTONEG_ACTIVE: c_uint = 0x0010;
pub const IGP01E1000_PLHR_VALID_CHANNEL_D: c_uint = 0x0008;
pub const IGP01E1000_PLHR_VALID_CHANNEL_C: c_uint = 0x0004;
pub const IGP01E1000_PLHR_VALID_CHANNEL_B: c_uint = 0x0002;
pub const IGP01E1000_PLHR_VALID_CHANNEL_A: c_uint = 0x0001;
// IGP01E1000 Channel Quality Register
pub const IGP01E1000_MSE_CHANNEL_D: c_uint = 0x000F;
pub const IGP01E1000_MSE_CHANNEL_C: c_uint = 0x00F0;
pub const IGP01E1000_MSE_CHANNEL_B: c_uint = 0x0F00;
pub const IGP01E1000_MSE_CHANNEL_A: c_uint = 0xF000;
pub const IGP02E1000_PM_SPD: c_uint = 0x0001	/* Smart Power Down */;
pub const IGP02E1000_PM_D3_LPLU: c_uint = 0x0004	/* Enable LPLU in non-D0a modes */;
pub const IGP02E1000_PM_D0_LPLU: c_uint = 0x0002	/* Enable LPLU in D0a mode */;
// IGP01E1000 DSP reset macros
pub const DSP_RESET_ENABLE: c_uint = 0x0;
pub const DSP_RESET_DISABLE: c_uint = 0x2;
pub const E1000_MAX_DSP_RESETS: c_int = 10;
// IGP01E1000 & IGP02E1000 AGC Registers

// IGP02E1000 AGC Register Length 9-bit mask
pub const IGP02E1000_AGC_LENGTH_MASK: c_uint = 0x7F;
// 7 bits (3 Coarse + 4 Fine) --> 128 optional values
pub const IGP01E1000_AGC_LENGTH_TABLE_SIZE: c_int = 128;
pub const IGP02E1000_AGC_LENGTH_TABLE_SIZE: c_int = 113;
// The precision error of the cable length is +/- 10 meters
pub const IGP01E1000_AGC_RANGE: c_int = 10;
pub const IGP02E1000_AGC_RANGE: c_int = 15;
// IGP01E1000 PCS Initialization register
// bits 3:6 in the PCS registers stores the channels polarity
pub const IGP01E1000_PHY_POLARITY_MASK: c_uint = 0x0078;
// IGP01E1000 GMII FIFO Register
pub const IGP01E1000_GMII_FLEX_SPD: c_uint = 0x10	/* Enable flexible speed;
// on Link-Up
pub const IGP01E1000_GMII_SPD: c_uint = 0x20	/* Enable SPD */;
// IGP01E1000 Analog Register
pub const IGP01E1000_ANALOG_SPARE_FUSE_STATUS: c_uint = 0x20D1;
pub const IGP01E1000_ANALOG_FUSE_STATUS: c_uint = 0x20D0;
pub const IGP01E1000_ANALOG_FUSE_CONTROL: c_uint = 0x20DC;
pub const IGP01E1000_ANALOG_FUSE_BYPASS: c_uint = 0x20DE;
pub const IGP01E1000_ANALOG_FUSE_POLY_MASK: c_uint = 0xF000;
pub const IGP01E1000_ANALOG_FUSE_FINE_MASK: c_uint = 0x0F80;
pub const IGP01E1000_ANALOG_FUSE_COARSE_MASK: c_uint = 0x0070;
pub const IGP01E1000_ANALOG_SPARE_FUSE_ENABLED: c_uint = 0x0100;
pub const IGP01E1000_ANALOG_FUSE_ENABLE_SW_CONTROL: c_uint = 0x0002;
pub const IGP01E1000_ANALOG_FUSE_COARSE_THRESH: c_uint = 0x0040;
pub const IGP01E1000_ANALOG_FUSE_COARSE_10: c_uint = 0x0010;
pub const IGP01E1000_ANALOG_FUSE_FINE_1: c_uint = 0x0080;
pub const IGP01E1000_ANALOG_FUSE_FINE_10: c_uint = 0x0500;
// Bit definitions for valid PHY IDs.
// I = Integrated
// E = External
//
pub const M88_VENDOR: c_uint = 0x0141;
pub const M88E1000_E_PHY_ID: c_uint = 0x01410C50;
pub const M88E1000_I_PHY_ID: c_uint = 0x01410C30;
pub const M88E1011_I_PHY_ID: c_uint = 0x01410C20;
pub const IGP01E1000_I_PHY_ID: c_uint = 0x02A80380;

pub const M88E1011_I_REV_4: c_uint = 0x04;
pub const M88E1111_I_PHY_ID: c_uint = 0x01410CC0;
pub const M88E1118_E_PHY_ID: c_uint = 0x01410E40;
pub const L1LXT971A_PHY_ID: c_uint = 0x001378E0;
pub const RTL8211B_PHY_ID: c_uint = 0x001CC910;
pub const RTL8201N_PHY_ID: c_uint = 0x8200;
pub const RTL_PHY_CTRL_FD: c_uint = 0x0100 /* Full duplex.0=half; 1=full */;
pub const RTL_PHY_CTRL_SPD_100: c_uint = 0x200000 /* Force 100Mb */;
// Bits...
// 15-5: page
// 4-0: register offset
//
pub const PHY_PAGE_SHIFT: c_int = 5;

pub const IGP3_KMRN_DIAG_PCS_LOCK_LOSS: c_uint = 0x0002	/* RX PCS is not synced */;

pub const IGP3_VR_CTRL_MODE_SHUT: c_uint = 0x0200	/* Enter powerdown, shutdown VRs */;
pub const IGP3_VR_CTRL_MODE_MASK: c_uint = 0x0300	/* Shutdown VR Mask */;

// Capabilities for SKU Control
pub const IGP3_CAP_INITIATE_TEAM: c_uint = 0x0001	/* Able to initiate a team */;
pub const IGP3_CAP_WFM: c_uint = 0x0002	/* Support WoL and PXE */;
pub const IGP3_CAP_ASF: c_uint = 0x0004	/* Support ASF */;
pub const IGP3_CAP_LPLU: c_uint = 0x0008	/* Support Low Power Link Up */;
pub const IGP3_CAP_DC_AUTO_SPEED: c_uint = 0x0010	/* Support AC/DC Auto Link Speed */;
pub const IGP3_CAP_SPD: c_uint = 0x0020	/* Support Smart Power Down */;
pub const IGP3_CAP_MULT_QUEUE: c_uint = 0x0040	/* Support 2 tx & 2 rx queues */;
pub const IGP3_CAP_RSS: c_uint = 0x0080	/* Support RSS */;
pub const IGP3_CAP_8021PQ: c_uint = 0x0100	/* Support 802.1Q & 802.1p */;
pub const IGP3_CAP_AMT_CB: c_uint = 0x0200	/* Support active manageability and circuit breaker */;
pub const IGP3_PPC_JORDAN_EN: c_uint = 0x0001;
pub const IGP3_PPC_JORDAN_GIGA_SPEED: c_uint = 0x0002;
pub const IGP3_KMRN_PMC_EE_IDLE_LINK_DIS: c_uint = 0x0001;
pub const IGP3_KMRN_PMC_K0S_ENTRY_LATENCY_MASK: c_uint = 0x001E;
pub const IGP3_KMRN_PMC_K0S_MODE1_EN_GIGA: c_uint = 0x0020;
pub const IGP3_KMRN_PMC_K0S_MODE1_EN_100: c_uint = 0x0040;
pub const IGP3E1000_PHY_MISC_CTRL: c_uint = 0x1B	/* Misc. Ctrl register */;
pub const IGP3_PHY_MISC_DUPLEX_MANUAL_SET: c_uint = 0x1000	/* Duplex Manual Set */;

pub const IGP3_KMRN_EC_DIS_INBAND: c_uint = 0x0080;
pub const IGP03E1000_E_PHY_ID: c_uint = 0x02A80390;
pub const IFE_E_PHY_ID: c_uint = 0x02A80330	/* 10/100 PHY */;
pub const IFE_PLUS_E_PHY_ID: c_uint = 0x02A80320;
pub const IFE_C_E_PHY_ID: c_uint = 0x02A80310;
pub const IFE_PHY_EXTENDED_STATUS_CONTROL: c_uint = 0x10	/* 100BaseTx Extended Status, Control and Address */;
pub const IFE_PHY_SPECIAL_CONTROL: c_uint = 0x11	/* 100BaseTx PHY special control register */;
pub const IFE_PHY_RCV_FALSE_CARRIER: c_uint = 0x13	/* 100BaseTx Receive False Carrier Counter */;
pub const IFE_PHY_RCV_DISCONNECT: c_uint = 0x14	/* 100BaseTx Receive Disconnect Counter */;
pub const IFE_PHY_RCV_ERROT_FRAME: c_uint = 0x15	/* 100BaseTx Receive Error Frame Counter */;
pub const IFE_PHY_RCV_SYMBOL_ERR: c_uint = 0x16	/* Receive Symbol Error Counter */;
pub const IFE_PHY_PREM_EOF_ERR: c_uint = 0x17	/* 100BaseTx Receive Premature End Of Frame Error Counter */;
pub const IFE_PHY_RCV_EOF_ERR: c_uint = 0x18	/* 10BaseT Receive End Of Frame Error Counter */;
pub const IFE_PHY_TX_JABBER_DETECT: c_uint = 0x19	/* 10BaseT Transmit Jabber Detect Counter */;
pub const IFE_PHY_EQUALIZER: c_uint = 0x1A	/* PHY Equalizer Control and Status */;
pub const IFE_PHY_SPECIAL_CONTROL_LED: c_uint = 0x1B	/* PHY special control and LED configuration */;
pub const IFE_PHY_MDIX_CONTROL: c_uint = 0x1C	/* MDI/MDI-X Control register */;
pub const IFE_PHY_HWI_CONTROL: c_uint = 0x1D	/* Hardware Integrity Control (HWI) */;
pub const IFE_PESC_REDUCED_POWER_DOWN_DISABLE: c_uint = 0x2000	/* Default 1 = Disable auto reduced power down */;
pub const IFE_PESC_100BTX_POWER_DOWN: c_uint = 0x0400	/* Indicates the power state of 100BASE-TX */;
pub const IFE_PESC_10BTX_POWER_DOWN: c_uint = 0x0200	/* Indicates the power state of 10BASE-T */;
pub const IFE_PESC_POLARITY_REVERSED: c_uint = 0x0100	/* Indicates 10BASE-T polarity */;
pub const IFE_PESC_PHY_ADDR_MASK: c_uint = 0x007C	/* Bit 6:2 for sampled PHY address */;
pub const IFE_PESC_SPEED: c_uint = 0x0002	/* Auto-negotiation speed result 1=100Mbs, 0=10Mbs */;
pub const IFE_PESC_DUPLEX: c_uint = 0x0001	/* Auto-negotiation duplex result 1=Full, 0=Half */;
pub const IFE_PESC_POLARITY_REVERSED_SHIFT: c_int = 8;
pub const IFE_PSC_DISABLE_DYNAMIC_POWER_DOWN: c_uint = 0x0100	/* 1 = Dynamic Power Down disabled */;
pub const IFE_PSC_FORCE_POLARITY: c_uint = 0x0020	/* 1=Reversed Polarity, 0=Normal */;
pub const IFE_PSC_AUTO_POLARITY_DISABLE: c_uint = 0x0010	/* 1=Auto Polarity Disabled, 0=Enabled */;
pub const IFE_PSC_JABBER_FUNC_DISABLE: c_uint = 0x0001	/* 1=Jabber Disabled, 0=Normal Jabber Operation */;
pub const IFE_PSC_FORCE_POLARITY_SHIFT: c_int = 5;
pub const IFE_PSC_AUTO_POLARITY_DISABLE_SHIFT: c_int = 4;
pub const IFE_PMC_AUTO_MDIX: c_uint = 0x0080	/* 1=enable MDI/MDI-X feature, default 0=disabled */;
pub const IFE_PMC_FORCE_MDIX: c_uint = 0x0040	/* 1=force MDIX-X, 0=force MDI */;
pub const IFE_PMC_MDIX_STATUS: c_uint = 0x0020	/* 1=MDI-X, 0=MDI */;
pub const IFE_PMC_AUTO_MDIX_COMPLETE: c_uint = 0x0010	/* Resolution algorithm is completed */;
pub const IFE_PMC_MDIX_MODE_SHIFT: c_int = 6;
pub const IFE_PHC_MDIX_RESET_ALL_MASK: c_uint = 0x0000	/* Disable auto MDI-X */;
pub const IFE_PHC_HWI_ENABLE: c_uint = 0x8000	/* Enable the HWI feature */;
pub const IFE_PHC_ABILITY_CHECK: c_uint = 0x4000	/* 1= Test Passed, 0=failed */;
pub const IFE_PHC_TEST_EXEC: c_uint = 0x2000	/* PHY launch test pulses on the wire */;
pub const IFE_PHC_HIGHZ: c_uint = 0x0200	/* 1 = Open Circuit */;
pub const IFE_PHC_LOWZ: c_uint = 0x0400	/* 1 = Short Circuit */;
pub const IFE_PHC_LOW_HIGH_Z_MASK: c_uint = 0x0600	/* Mask for indication type of problem on the line */;
pub const IFE_PHC_DISTANCE_MASK: c_uint = 0x01FF	/* Mask for distance to the cable problem, in 80cm granularity */;
pub const IFE_PHC_RESET_ALL_MASK: c_uint = 0x0000	/* Disable HWI */;
pub const IFE_PSCL_PROBE_MODE: c_uint = 0x0020	/* LED Probe mode */;
pub const IFE_PSCL_PROBE_LEDS_OFF: c_uint = 0x0006	/* Force LEDs 0 and 2 off */;
pub const IFE_PSCL_PROBE_LEDS_ON: c_uint = 0x0007	/* Force LEDs 0 and 2 on */;

pub const ICH_FLASH_SEG_SIZE_256: c_int = 256;
pub const ICH_FLASH_SEG_SIZE_4K: c_int = 4096;
pub const ICH_FLASH_SEG_SIZE_64K: c_int = 65536;
pub const ICH_CYCLE_READ: c_uint = 0x0;
pub const ICH_CYCLE_RESERVED: c_uint = 0x1;
pub const ICH_CYCLE_WRITE: c_uint = 0x2;
pub const ICH_CYCLE_ERASE: c_uint = 0x3;
pub const ICH_FLASH_GFPREG: c_uint = 0x0000;
pub const ICH_FLASH_HSFSTS: c_uint = 0x0004;
pub const ICH_FLASH_HSFCTL: c_uint = 0x0006;
pub const ICH_FLASH_FADDR: c_uint = 0x0008;
pub const ICH_FLASH_FDATA0: c_uint = 0x0010;
pub const ICH_FLASH_FRACC: c_uint = 0x0050;
pub const ICH_FLASH_FREG0: c_uint = 0x0054;
pub const ICH_FLASH_FREG1: c_uint = 0x0058;
pub const ICH_FLASH_FREG2: c_uint = 0x005C;
pub const ICH_FLASH_FREG3: c_uint = 0x0060;
pub const ICH_FLASH_FPR0: c_uint = 0x0074;
pub const ICH_FLASH_FPR1: c_uint = 0x0078;
pub const ICH_FLASH_SSFSTS: c_uint = 0x0090;
pub const ICH_FLASH_SSFCTL: c_uint = 0x0092;
pub const ICH_FLASH_PREOP: c_uint = 0x0094;
pub const ICH_FLASH_OPTYPE: c_uint = 0x0096;
pub const ICH_FLASH_OPMENU: c_uint = 0x0098;
pub const ICH_FLASH_REG_MAPSIZE: c_uint = 0x00A0;
pub const ICH_FLASH_SECTOR_SIZE: c_int = 4096;
pub const ICH_GFPREG_BASE_MASK: c_uint = 0x1FFF;
pub const ICH_FLASH_LINEAR_ADDR_MASK: c_uint = 0x00FFFFFF;
// Miscellaneous PHY bit definitions.
pub const PHY_PREAMBLE: c_uint = 0xFFFFFFFF;
pub const PHY_SOF: c_uint = 0x01;
pub const PHY_OP_READ: c_uint = 0x02;
pub const PHY_OP_WRITE: c_uint = 0x01;
pub const PHY_TURNAROUND: c_uint = 0x02;
pub const PHY_PREAMBLE_SIZE: c_int = 32;
pub const MII_CR_SPEED_1000: c_uint = 0x0040;
pub const MII_CR_SPEED_100: c_uint = 0x2000;
pub const MII_CR_SPEED_10: c_uint = 0x0000;
pub const E1000_PHY_ADDRESS: c_uint = 0x01;

pub const PHY_REVISION_MASK: c_uint = 0xFFFFFFF0;
pub const DEVICE_SPEED_MASK: c_uint = 0x00000300	/* Device Ctrl Reg Speed Mask */;
pub const REG4_SPEED_MASK: c_uint = 0x01E0;
pub const REG9_SPEED_MASK: c_uint = 0x0300;
pub const ADVERTISE_10_HALF: c_uint = 0x0001;
pub const ADVERTISE_10_FULL: c_uint = 0x0002;
pub const ADVERTISE_100_HALF: c_uint = 0x0004;
pub const ADVERTISE_100_FULL: c_uint = 0x0008;
pub const ADVERTISE_1000_HALF: c_uint = 0x0010;
pub const ADVERTISE_1000_FULL: c_uint = 0x0020;
pub const AUTONEG_ADVERTISE_SPEED_DEFAULT: c_uint = 0x002F	/* Everything but 1000-Half */;
pub const AUTONEG_ADVERTISE_10_100_ALL: c_uint = 0x000F	/* All 10/100 speeds */;
pub const AUTONEG_ADVERTISE_10_ALL: c_uint = 0x0003	/* 10Mbps Full & Half speeds */;
