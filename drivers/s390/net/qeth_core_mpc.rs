//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/net/qeth_core_mpc.h
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
//
// Copyright IBM Corp. 2007
// Author(s): Frank Pavlic <fpavlic@de.ibm.com>,
// Thomas Spatzier <tspat@de.ibm.com>,
// Frank Blaschka <frank.blaschka@de.ibm.com>
//

pub const IPA_PDU_HEADER_SIZE: c_uint = 0x40;

pub const QETH_SEQ_NO_LENGTH: c_int = 4;
pub const QETH_MPC_TOKEN_LENGTH: c_int = 4;
pub const QETH_MCL_LENGTH: c_int = 4;

//
// IP Assist related definitions
//
pub const IPA_CMD_INITIATOR_HOST: c_uint = 0x00;
pub const IPA_CMD_INITIATOR_OSA: c_uint = 0x01;
pub const IPA_CMD_PRIM_VERSION_NO: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_ipa_caps {
    pub supported: u32,
    pub enabled: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_card_types {
    QETH_CARD_TYPE_OSD     = 1,
    QETH_CARD_TYPE_IQD     = 5,
    QETH_CARD_TYPE_OSM     = 3,
    QETH_CARD_TYPE_OSX     = 2,
}

pub const QETH_MPC_DIFINFO_LEN_INDICATES_LINK_TYPE: c_uint = 0x18;
// only the first two bytes are looked at in qeth_get_cardname_short
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_link_types {
    QETH_LINK_TYPE_FAST_ETH     = 0x01,
    QETH_LINK_TYPE_HSTR         = 0x02,
    QETH_LINK_TYPE_GBIT_ETH     = 0x03,
    QETH_LINK_TYPE_10GBIT_ETH   = 0x10,
    QETH_LINK_TYPE_25GBIT_ETH   = 0x12,
    QETH_LINK_TYPE_LANE_ETH100  = 0x81,
    QETH_LINK_TYPE_LANE_TR      = 0x82,
    QETH_LINK_TYPE_LANE_ETH1000 = 0x83,
    QETH_LINK_TYPE_LANE         = 0x88,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_routing_types {
// TODO: set to bit flag used in IPA Command
    NO_ROUTER		= 0,
    PRIMARY_ROUTER		= 1,
    SECONDARY_ROUTER	= 2,
    MULTICAST_ROUTER	= 3,
    PRIMARY_CONNECTOR	= 4,
    SECONDARY_CONNECTOR	= 5,
}

// IPA Commands
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_ipa_cmds {
    IPA_CMD_STARTLAN		= 0x01,
    IPA_CMD_STOPLAN			= 0x02,
    IPA_CMD_SETVMAC			= 0x21,
    IPA_CMD_DELVMAC			= 0x22,
    IPA_CMD_SETGMAC			= 0x23,
    IPA_CMD_DELGMAC			= 0x24,
    IPA_CMD_SETVLAN			= 0x25,
    IPA_CMD_DELVLAN			= 0x26,
    IPA_CMD_VNICC			= 0x2a,
    IPA_CMD_SETBRIDGEPORT_OSA	= 0x2b,
    IPA_CMD_SETIP			= 0xb1,
    IPA_CMD_QIPASSIST		= 0xb2,
    IPA_CMD_SETASSPARMS		= 0xb3,
    IPA_CMD_SETIPM			= 0xb4,
    IPA_CMD_DELIPM			= 0xb5,
    IPA_CMD_SETRTG			= 0xb6,
    IPA_CMD_DELIP			= 0xb7,
    IPA_CMD_SETADAPTERPARMS		= 0xb8,
    IPA_CMD_SET_DIAG_ASS		= 0xb9,
    IPA_CMD_SETBRIDGEPORT_IQD	= 0xbe,
    IPA_CMD_CREATE_ADDR		= 0xc3,
    IPA_CMD_DESTROY_ADDR		= 0xc4,
    IPA_CMD_REGISTER_LOCAL_ADDR	= 0xd1,
    IPA_CMD_UNREGISTER_LOCAL_ADDR	= 0xd2,
    IPA_CMD_ADDRESS_CHANGE_NOTIF	= 0xd3,
    IPA_CMD_UNKNOWN			= 0x00
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_ip_ass_cmds {
    IPA_CMD_ASS_START	= 0x0001,
    IPA_CMD_ASS_STOP	= 0x0002,
    IPA_CMD_ASS_CONFIGURE	= 0x0003,
    IPA_CMD_ASS_ENABLE	= 0x0004,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_arp_process_subcmds {
    IPA_CMD_ASS_ARP_SET_NO_ENTRIES	= 0x0003,
    IPA_CMD_ASS_ARP_QUERY_CACHE	= 0x0004,
    IPA_CMD_ASS_ARP_ADD_ENTRY	= 0x0005,
    IPA_CMD_ASS_ARP_REMOVE_ENTRY	= 0x0006,
    IPA_CMD_ASS_ARP_FLUSH_CACHE	= 0x0007,
    IPA_CMD_ASS_ARP_QUERY_INFO	= 0x0104,
    IPA_CMD_ASS_ARP_QUERY_STATS	= 0x0204,
}

// Return Codes for IPA Commands
// according to OSA card Specs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_ipa_return_codes {
    IPA_RC_SUCCESS			= 0x0000,
    IPA_RC_NOTSUPP			= 0x0001,
    IPA_RC_IP_TABLE_FULL		= 0x0002,
    IPA_RC_INVALID_SUBCMD		= 0x0002,
    IPA_RC_UNKNOWN_ERROR		= 0x0003,
    IPA_RC_HARDWARE_AUTH_ERROR	= 0x0003,
    IPA_RC_UNSUPPORTED_COMMAND	= 0x0004,
    IPA_RC_TRACE_ALREADY_ACTIVE	= 0x0005,
    IPA_RC_VNICC_OOSEQ		= 0x0005,
    IPA_RC_INVALID_FORMAT		= 0x0006,
    IPA_RC_DUP_IPV6_REMOTE		= 0x0008,
    IPA_RC_SBP_IQD_NOT_CONFIGURED	= 0x000C,
    IPA_RC_DUP_IPV6_HOME		= 0x0010,
    IPA_RC_SBP_IQD_OS_MISMATCH	= 0x0010,
    IPA_RC_UNREGISTERED_ADDR	= 0x0011,
    IPA_RC_NO_ID_AVAILABLE		= 0x0012,
    IPA_RC_ID_NOT_FOUND		= 0x0013,
    IPA_RC_SBP_IQD_ANO_DEV_PRIMARY	= 0x0014,
    IPA_RC_SBP_IQD_CURRENT_SECOND	= 0x0018,
    IPA_RC_SBP_IQD_LIMIT_SECOND	= 0x001C,
    IPA_RC_INVALID_IP_VERSION	= 0x0020,
    IPA_RC_SBP_IQD_NOT_AUTHD_BY_ZMAN = 0x0020,
    IPA_RC_SBP_IQD_CURRENT_PRIMARY	= 0x0024,
    IPA_RC_LAN_FRAME_MISMATCH	= 0x0040,
    IPA_RC_SBP_IQD_NO_QDIO_QUEUES	= 0x00EB,
    IPA_RC_L2_UNSUPPORTED_CMD	= 0x2003,
    IPA_RC_L2_DUP_MAC		= 0x2005,
    IPA_RC_L2_ADDR_TABLE_FULL	= 0x2006,
    IPA_RC_L2_DUP_LAYER3_MAC	= 0x200a,
    IPA_RC_L2_GMAC_NOT_FOUND	= 0x200b,
    IPA_RC_L2_MAC_NOT_AUTH_BY_HYP	= 0x200c,
    IPA_RC_L2_MAC_NOT_AUTH_BY_ADP	= 0x200d,
    IPA_RC_L2_MAC_NOT_FOUND		= 0x2010,
    IPA_RC_L2_INVALID_VLAN_ID	= 0x2015,
    IPA_RC_L2_DUP_VLAN_ID		= 0x2016,
    IPA_RC_L2_VLAN_ID_NOT_FOUND	= 0x2017,
    IPA_RC_L2_VLAN_ID_NOT_ALLOWED	= 0x2050,
    IPA_RC_VNICC_VNICBP		= 0x20B0,
    IPA_RC_SBP_OSA_NOT_CONFIGURED	= 0x2B0C,
    IPA_RC_SBP_OSA_OS_MISMATCH	= 0x2B10,
    IPA_RC_SBP_OSA_ANO_DEV_PRIMARY	= 0x2B14,
    IPA_RC_SBP_OSA_CURRENT_SECOND	= 0x2B18,
    IPA_RC_SBP_OSA_LIMIT_SECOND	= 0x2B1C,
    IPA_RC_SBP_OSA_NOT_AUTHD_BY_ZMAN = 0x2B20,
    IPA_RC_SBP_OSA_CURRENT_PRIMARY	= 0x2B24,
    IPA_RC_SBP_OSA_NO_QDIO_QUEUES	= 0x2BEB,
    IPA_RC_DATA_MISMATCH		= 0xe001,
    IPA_RC_INVALID_MTU_SIZE		= 0xe002,
    IPA_RC_INVALID_LANTYPE		= 0xe003,
    IPA_RC_INVALID_LANNUM		= 0xe004,
    IPA_RC_DUPLICATE_IP_ADDRESS	= 0xe005,
    IPA_RC_IP_ADDR_TABLE_FULL	= 0xe006,
    IPA_RC_LAN_PORT_STATE_ERROR	= 0xe007,
    IPA_RC_SETIP_NO_STARTLAN	= 0xe008,
    IPA_RC_SETIP_ALREADY_RECEIVED	= 0xe009,
    IPA_RC_IP_ADDR_ALREADY_USED	= 0xe00a,
    IPA_RC_MC_ADDR_NOT_FOUND	= 0xe00b,
    IPA_RC_SETIP_INVALID_VERSION	= 0xe00d,
    IPA_RC_UNSUPPORTED_SUBCMD	= 0xe00e,
    IPA_RC_ARP_ASSIST_NO_ENABLE	= 0xe00f,
    IPA_RC_PRIMARY_ALREADY_DEFINED	= 0xe010,
    IPA_RC_SECOND_ALREADY_DEFINED	= 0xe011,
    IPA_RC_INVALID_SETRTG_INDICATOR	= 0xe012,
    IPA_RC_MC_ADDR_ALREADY_DEFINED	= 0xe013,
    IPA_RC_LAN_OFFLINE		= 0xe080,
    IPA_RC_VEPA_TO_VEB_TRANSITION	= 0xe090,
    IPA_RC_INVALID_IP_VERSION2	= 0xf001,
    IPA_RC_FFFF			= 0xffff
}

// IPA function flags; each flag marks availability of respective function
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_ipa_funcs {
    IPA_ARP_PROCESSING      = 0x00000001L,
    IPA_INBOUND_CHECKSUM    = 0x00000002L,
    IPA_OUTBOUND_CHECKSUM   = 0x00000004L,
// RESERVED		= 0x00000008L,
    IPA_FILTERING           = 0x00000010L,
    IPA_IPV6                = 0x00000020L,
    IPA_MULTICASTING        = 0x00000040L,
    IPA_IP_REASSEMBLY       = 0x00000080L,
    IPA_QUERY_ARP_COUNTERS  = 0x00000100L,
    IPA_QUERY_ARP_ADDR_INFO = 0x00000200L,
    IPA_SETADAPTERPARMS     = 0x00000400L,
    IPA_VLAN_PRIO           = 0x00000800L,
    IPA_PASSTHRU            = 0x00001000L,
    IPA_FLUSH_ARP_SUPPORT   = 0x00002000L,
    IPA_FULL_VLAN           = 0x00004000L,
    IPA_INBOUND_PASSTHRU    = 0x00008000L,
    IPA_SOURCE_MAC          = 0x00010000L,
    IPA_OSA_MC_ROUTER       = 0x00020000L,
    IPA_QUERY_ARP_ASSIST	= 0x00040000L,
    IPA_INBOUND_TSO         = 0x00080000L,
    IPA_OUTBOUND_TSO        = 0x00100000L,
    IPA_INBOUND_CHECKSUM_V6 = 0x00400000L,
    IPA_OUTBOUND_CHECKSUM_V6 = 0x00800000L,
}

// SETIP/DELIP IPA Command:
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_ipa_setdelip_flags {
    QETH_IPA_SETDELIP_DEFAULT          = 0x00L, /* default */
    QETH_IPA_SETIP_VIPA_FLAG           = 0x01L, /* no grat. ARP */
    QETH_IPA_SETIP_TAKEOVER_FLAG       = 0x02L, /* nofail on grat. ARP */
    QETH_IPA_DELIP_ADDR_2_B_TAKEN_OVER = 0x20L,
    QETH_IPA_DELIP_VIPA_FLAG           = 0x40L,
    QETH_IPA_DELIP_ADDR_NEEDS_SETIP    = 0x80L,
}

// SETADAPTER IPA Command:
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_ipa_setadp_cmd {
    IPA_SETADP_QUERY_COMMANDS_SUPPORTED	= 0x00000001L,
    IPA_SETADP_ALTER_MAC_ADDRESS		= 0x00000002L,
    IPA_SETADP_ADD_DELETE_GROUP_ADDRESS	= 0x00000004L,
    IPA_SETADP_ADD_DELETE_FUNCTIONAL_ADDR	= 0x00000008L,
    IPA_SETADP_SET_ADDRESSING_MODE		= 0x00000010L,
    IPA_SETADP_SET_CONFIG_PARMS		= 0x00000020L,
    IPA_SETADP_SET_CONFIG_PARMS_EXTENDED	= 0x00000040L,
    IPA_SETADP_SET_BROADCAST_MODE		= 0x00000080L,
    IPA_SETADP_SEND_OSA_MESSAGE		= 0x00000100L,
    IPA_SETADP_SET_SNMP_CONTROL		= 0x00000200L,
    IPA_SETADP_QUERY_CARD_INFO		= 0x00000400L,
    IPA_SETADP_SET_PROMISC_MODE		= 0x00000800L,
    IPA_SETADP_SET_DIAG_ASSIST		= 0x00002000L,
    IPA_SETADP_SET_ACCESS_CONTROL		= 0x00010000L,
    IPA_SETADP_QUERY_OAT			= 0x00080000L,
    IPA_SETADP_QUERY_SWITCH_ATTRIBUTES	= 0x00100000L,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_ipa_mac_ops {
    CHANGE_ADDR_READ_MAC		= 0,
    CHANGE_ADDR_REPLACE_MAC		= 1,
    CHANGE_ADDR_ADD_MAC		= 2,
    CHANGE_ADDR_DEL_MAC		= 4,
    CHANGE_ADDR_RESET_MAC		= 8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_ipa_addr_ops {
    CHANGE_ADDR_READ_ADDR		= 0,
    CHANGE_ADDR_ADD_ADDR		= 1,
    CHANGE_ADDR_DEL_ADDR		= 2,
    CHANGE_ADDR_FLUSH_ADDR_TABLE	= 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_ipa_promisc_modes {
    SET_PROMISC_MODE_OFF		= 0,
    SET_PROMISC_MODE_ON		= 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_ipa_isolation_modes {
    ISOLATION_MODE_NONE		= 0x00000000L,
    ISOLATION_MODE_FWD		= 0x00000001L,
    ISOLATION_MODE_DROP		= 0x00000002L,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_ipa_set_access_mode_rc {
    SET_ACCESS_CTRL_RC_SUCCESS		= 0x0000,
    SET_ACCESS_CTRL_RC_NOT_SUPPORTED	= 0x0004,
    SET_ACCESS_CTRL_RC_ALREADY_NOT_ISOLATED	= 0x0008,
    SET_ACCESS_CTRL_RC_ALREADY_ISOLATED	= 0x0010,
    SET_ACCESS_CTRL_RC_NONE_SHARED_ADAPTER	= 0x0014,
    SET_ACCESS_CTRL_RC_ACTIVE_CHECKSUM_OFF	= 0x0018,
    SET_ACCESS_CTRL_RC_REFLREL_UNSUPPORTED	= 0x0022,
    SET_ACCESS_CTRL_RC_REFLREL_FAILED	= 0x0024,
    SET_ACCESS_CTRL_RC_REFLREL_DEACT_FAILED	= 0x0028,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_card_info_card_type {
    CARD_INFO_TYPE_1G_COPPER_A	= 0x61,
    CARD_INFO_TYPE_1G_FIBRE_A	= 0x71,
    CARD_INFO_TYPE_10G_FIBRE_A	= 0x91,
    CARD_INFO_TYPE_1G_COPPER_B	= 0xb1,
    CARD_INFO_TYPE_1G_FIBRE_B	= 0xa1,
    CARD_INFO_TYPE_10G_FIBRE_B	= 0xc1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_card_info_port_mode {
    CARD_INFO_PORTM_HALFDUPLEX	= 0x0002,
    CARD_INFO_PORTM_FULLDUPLEX	= 0x0003,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_card_info_port_speed {
    CARD_INFO_PORTS_10M		= 0x00000005,
    CARD_INFO_PORTS_100M		= 0x00000006,
    CARD_INFO_PORTS_1G		= 0x00000007,
    CARD_INFO_PORTS_10G		= 0x00000008,
    CARD_INFO_PORTS_25G		= 0x0000000A,
}

// (SET)DELIP(M) IPA stuff
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_ipacmd_setdelip4 {
    pub addr: __be32,
    pub mask: __be32,
    pub flags: __u32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_ipacmd_setdelip6 {
    pub addr: in6_addr,
    pub prefix: in6_addr,
    pub flags: __u32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_ipacmd_setdelipm {
    pub mac: [__u8; 6],
    pub padding: [__u8; 2],
    pub ip: in6_addr,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_ipacmd_layer2setdelmac {
    pub mac_length: __u32,
    pub mac: [__u8; 6],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_ipacmd_layer2setdelvlan {
    pub vlan_id: __u16,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_ipacmd_setassparms_hdr {
    pub length: __u16,
    pub command_code: __u16,
    pub return_code: __u16,
    pub number_of_replies: __u8,
    pub seq_no: __u8,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_arp_query_data {
    pub request_bits: __u16,
    pub reply_bits: __u16,
    pub no_entries: __u32,
    pub /: *mut *mut char data; / only for replies,
    pub __attribute__((packed)): },
// used as parameter for arp_query reply
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_arp_query_info {
    pub udata_len: __u32,
    pub mask_bits: __u16,
    pub udata_offset: __u32,
    pub no_entries: __u32,
    pub udata: *mut c_char,
}

// IPA set assist segmentation bit definitions for receive and
// transmit checksum offloading.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_ipa_checksum_bits {
    QETH_IPA_CHECKSUM_IP_HDR	= 0x0002,
    QETH_IPA_CHECKSUM_UDP		= 0x0008,
    QETH_IPA_CHECKSUM_TCP		= 0x0010,
    QETH_IPA_CHECKSUM_LP2LP		= 0x0020
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_ipa_large_send_caps {
    QETH_IPA_LARGE_SEND_TCP		= 0x00000001,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_tso_start_data {
    pub mss: u32,
    pub supported: u32,
}

// SETASSPARMS IPA Command:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_ipacmd_setassparms {
    pub assist_no: u32,
    pub hdr: qeth_ipacmd_setassparms_hdr,
    pub flags_32bit: __u32,
    pub caps: qeth_ipa_caps,
    pub arp_entry: qeth_arp_cache_entry,
    pub query_arp: qeth_arp_query_data,
    pub tso: qeth_tso_start_data,
    pub data: },
// C attribute field omitted

// SETRTG IPA Command:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_set_routing {
    pub type: __u8,
}

// SETADAPTERPARMS IPA Command:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_query_cmds_supp {
    pub no_lantypes_supp: __u32,
    pub lan_type: __u8,
    pub reserved1: [__u8; 3],
    pub supported_cmds: __u32,
    pub reserved2: [__u8; 8],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_change_addr {
    pub cmd: u32,
    pub addr_size: u32,
    pub no_macs: u32,
    pub addr: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_snmp_cmd {
    pub token: [__u8; 16],
    pub request: __u32,
    pub interface: __u32,
    pub returncode: __u32,
    pub firmwarelevel: __u32,
    pub seqno: __u32,
    pub data: __u8,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_snmp_ureq_hdr {
    pub data_len: __u32,
    pub req_len: __u32,
    pub reserved1: __u32,
    pub reserved2: __u32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_snmp_ureq {
    pub hdr: qeth_snmp_ureq_hdr,
    pub cmd: qeth_snmp_cmd,
    pub __attribute__((packed)): },
// SET_ACCESS_CONTROL: same format for request and reply
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_set_access_ctrl {
    pub subcmd_code: __u32,
    pub reserved: [__u8; 8],
    pub __attribute__((packed)): },
pub const QETH_QOAT_PHYS_SPEED_UNKNOWN: c_uint = 0x00;
pub const QETH_QOAT_PHYS_SPEED_10M_HALF: c_uint = 0x01;
pub const QETH_QOAT_PHYS_SPEED_10M_FULL: c_uint = 0x02;
pub const QETH_QOAT_PHYS_SPEED_100M_HALF: c_uint = 0x03;
pub const QETH_QOAT_PHYS_SPEED_100M_FULL: c_uint = 0x04;
pub const QETH_QOAT_PHYS_SPEED_1000M_HALF: c_uint = 0x05;
pub const QETH_QOAT_PHYS_SPEED_1000M_FULL: c_uint = 0x06;
// n/a						0x07
pub const QETH_QOAT_PHYS_SPEED_10G_FULL: c_uint = 0x08;
// n/a						0x09
pub const QETH_QOAT_PHYS_SPEED_25G_FULL: c_uint = 0x0A;
pub const QETH_QOAT_PHYS_MEDIA_COPPER: c_uint = 0x01;
pub const QETH_QOAT_PHYS_MEDIA_FIBRE_SHORT: c_uint = 0x02;
pub const QETH_QOAT_PHYS_MEDIA_FIBRE_LONG: c_uint = 0x04;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_query_oat_physical_if {
    pub res_head: [u8; 33],
    pub speed_duplex: u8,
    pub media_type: u8,
    pub res_tail: [u8; 29],
}

pub const QETH_QOAT_REPLY_TYPE_PHYS_IF: c_uint = 0x0004;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_query_oat_reply {
    pub type: u16,
    pub length: u16,
    pub version: u16,
    pub res: [u8; 10],
    pub phys_if: qeth_query_oat_physical_if,
}

pub const QETH_QOAT_SCOPE_INTERFACE: c_uint = 0x00000001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_query_oat {
    pub subcmd_code: u32,
    pub reserved: [u8; 12],
    pub reply: [qeth_query_oat_reply; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_qoat_priv {
    pub buffer_len: __u32,
    pub response_len: __u32,
    pub buffer: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_query_card_info {
    pub card_type: __u8,
    pub reserved1: __u8,
    pub port_mode: __u16,
    pub port_speed: __u32,
    pub reserved2: __u32,
}

pub const QETH_SWITCH_FORW_802_1: c_uint = 0x00000001;
pub const QETH_SWITCH_FORW_REFL_RELAY: c_uint = 0x00000002;
pub const QETH_SWITCH_CAP_RTE: c_uint = 0x00000004;
pub const QETH_SWITCH_CAP_ECP: c_uint = 0x00000008;
pub const QETH_SWITCH_CAP_VDP: c_uint = 0x00000010;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_query_switch_attributes {
    pub version: __u8,
    pub reserved1: __u8,
    pub reserved2: __u16,
    pub capabilities: __u32,
    pub settings: __u32,
    pub reserved3: [__u8; 8],
}

pub const QETH_SETADP_FLAGS_VIRTUAL_MAC: c_uint = 0x80	/* for CHANGE_ADDR_READ_MAC */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_ipacmd_setadpparms_hdr {
    pub cmdlength: u16,
    pub reserved2: u16,
    pub command_code: u32,
    pub return_code: u16,
    pub used_total: u8,
    pub seq_no: u8,
    pub flags: u8,
    pub reserved3: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_ipacmd_setadpparms {
    pub hw_cmds: qeth_ipa_caps,
    pub hdr: qeth_ipacmd_setadpparms_hdr,
    pub query_cmds_supp: qeth_query_cmds_supp,
    pub change_addr: qeth_change_addr,
    pub snmp: qeth_snmp_cmd,
    pub set_access_ctrl: qeth_set_access_ctrl,
    pub query_oat: qeth_query_oat,
    pub card_info: qeth_query_card_info,
    pub query_switch_attributes: qeth_query_switch_attributes,
    pub mode: __u32,
    pub data: },
// C attribute field omitted

// CREATE_ADDR IPA Command:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_create_destroy_address {
    pub mac_addr: [u8; ETH_ALEN],
    pub uid: u16,
}

// SET DIAGNOSTIC ASSIST IPA Command:
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_diags_cmds {
    QETH_DIAGS_CMD_QUERY	= 0x0001,
    QETH_DIAGS_CMD_TRAP	= 0x0002,
    QETH_DIAGS_CMD_TRACE	= 0x0004,
    QETH_DIAGS_CMD_NOLOG	= 0x0008,
    QETH_DIAGS_CMD_DUMP	= 0x0010,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_diags_trace_types {
    QETH_DIAGS_TYPE_HIPERSOCKET	= 0x02,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_diags_trace_cmds {
    QETH_DIAGS_CMD_TRACE_ENABLE	= 0x0001,
    QETH_DIAGS_CMD_TRACE_DISABLE	= 0x0002,
    QETH_DIAGS_CMD_TRACE_MODIFY	= 0x0004,
    QETH_DIAGS_CMD_TRACE_REPLACE	= 0x0008,
    QETH_DIAGS_CMD_TRACE_QUERY	= 0x0010,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_diags_trap_action {
    QETH_DIAGS_TRAP_ARM	= 0x01,
    QETH_DIAGS_TRAP_DISARM	= 0x02,
    QETH_DIAGS_TRAP_CAPTURE = 0x04,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_ipacmd_diagass {
    pub host_tod2: __u32,
    pub subcmd_len: __u16,
    pub subcmd: __u32,
    pub type: __u8,
    pub action: __u8,
    pub options: __u16,
    pub ext: __u32,
    pub cdata: [__u8; 64],
// C attribute field omitted

// VNIC Characteristics IPA Command:
// IPA commands/sub commands for VNICC
pub const IPA_VNICC_QUERY_CHARS: c_uint = 0x00000000L;
pub const IPA_VNICC_QUERY_CMDS: c_uint = 0x00000001L;
pub const IPA_VNICC_ENABLE: c_uint = 0x00000002L;
pub const IPA_VNICC_DISABLE: c_uint = 0x00000004L;
pub const IPA_VNICC_SET_TIMEOUT: c_uint = 0x00000008L;
pub const IPA_VNICC_GET_TIMEOUT: c_uint = 0x00000010L;
// VNICC flags
pub const QETH_VNICC_FLOODING: c_uint = 0x80000000;
pub const QETH_VNICC_MCAST_FLOODING: c_uint = 0x40000000;
pub const QETH_VNICC_LEARNING: c_uint = 0x20000000;
pub const QETH_VNICC_TAKEOVER_SETVMAC: c_uint = 0x10000000;
pub const QETH_VNICC_TAKEOVER_LEARNING: c_uint = 0x08000000;
pub const QETH_VNICC_BRIDGE_INVISIBLE: c_uint = 0x04000000;
pub const QETH_VNICC_RX_BCAST: c_uint = 0x02000000;
// VNICC default values
pub const QETH_VNICC_ALL: c_uint = 0xff000000;

// default VNICC timeout in seconds
pub const QETH_VNICC_DEFAULT_TIMEOUT: c_int = 600;
// VNICC header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_ipacmd_vnicc_hdr {
    pub data_length: u16,
    pub reserved: u16,
    pub sub_command: u32,
}

// query supported commands for VNIC characteristic
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_vnicc_query_cmds {
    pub vnic_char: u32,
    pub sup_cmds: u32,
}

// enable/disable VNIC characteristic
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_vnicc_set_char {
    pub vnic_char: u32,
}

// get/set timeout for VNIC characteristic
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_vnicc_getset_timeout {
    pub vnic_char: u32,
    pub timeout: u32,
}

// complete VNICC IPA command message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_ipacmd_vnicc {
    pub vnicc_cmds: qeth_ipa_caps,
    pub hdr: qeth_ipacmd_vnicc_hdr,
    pub query_cmds: qeth_vnicc_query_cmds,
    pub set_char: qeth_vnicc_set_char,
    pub getset_timeout: qeth_vnicc_getset_timeout,
    pub data: },
}

// SETBRIDGEPORT IPA Command:
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_ipa_sbp_cmd {
    IPA_SBP_QUERY_COMMANDS_SUPPORTED	= 0x00000000L,
    IPA_SBP_RESET_BRIDGE_PORT_ROLE		= 0x00000001L,
    IPA_SBP_SET_PRIMARY_BRIDGE_PORT		= 0x00000002L,
    IPA_SBP_SET_SECONDARY_BRIDGE_PORT	= 0x00000004L,
    IPA_SBP_QUERY_BRIDGE_PORTS		= 0x00000008L,
    IPA_SBP_BRIDGE_PORT_STATE_CHANGE	= 0x00000010L,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_if_token {
    pub devnum: __u16,
    pub cssid: __u8,
    pub iid: __u8,
    pub ssid: __u8,
    pub chpid: __u8,
    pub chid: __u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_addr_lnid {
    pub mac: [__u8; 6],
    pub lnid: __u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_ipacmd_sbp_hdr {
    pub cmdlength: __u16,
    pub reserved1: __u16,
    pub command_code: __u32,
    pub return_code: __u16,
    pub used_total: __u8,
    pub seq_no: __u8,
    pub reserved2: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_sbp_query_cmds_supp {
    pub supported_cmds: __u32,
    pub reserved: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_sbp_set_primary {
    pub token: net_if_token,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_sbp_port_entry {
    pub role: __u8,
    pub state: __u8,
    pub reserved1: __u8,
    pub reserved2: __u8,
    pub token: net_if_token,
    pub __packed: },
// For IPA_SBP_QUERY_BRIDGE_PORTS, IPA_SBP_BRIDGE_PORT_STATE_CHANGE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_sbp_port_data {
    pub primary_bp_supported: __u8,
    pub secondary_bp_supported: __u8,
    pub num_entries: __u8,
    pub entry_length: __u8,
    pub entry: [qeth_sbp_port_entry; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_ipacmd_setbridgeport {
    pub sbp_cmds: qeth_ipa_caps,
    pub hdr: qeth_ipacmd_sbp_hdr,
    pub query_cmds_supp: qeth_sbp_query_cmds_supp,
    pub set_primary: qeth_sbp_set_primary,
    pub port_data: qeth_sbp_port_data,
    pub data: },
    pub __packed: },

// ADDRESS_CHANGE_NOTIFICATION adapter-initiated "command"
// Bitmask for entry->change_code. Both bits may be raised.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_ipa_addr_change_code {
    IPA_ADDR_CHANGE_CODE_VLANID		= 0x01,
    IPA_ADDR_CHANGE_CODE_MACADDR		= 0x02,
    IPA_ADDR_CHANGE_CODE_REMOVAL		= 0x80,	/* else addition */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_ipacmd_addr_change_entry {
    pub token: net_if_token,
    pub addr_lnid: mac_addr_lnid,
    pub change_code: __u8,
    pub reserved1: __u8,
    pub reserved2: __u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_ipacmd_addr_change {
    pub lost_event_mask: __u8,
    pub reserved: __u8,
    pub num_entries: __u16,
    pub entry: [qeth_ipacmd_addr_change_entry; ],
    pub __packed: },
// [UN]REGISTER_LOCAL_ADDRESS notifications
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_ipacmd_local_addr4 {
    pub addr: __be32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_ipacmd_local_addrs4 {
    pub count: u32,
    pub addr_length: u32,
    pub addrs: [qeth_ipacmd_local_addr4; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_ipacmd_local_addr6 {
    pub addr: in6_addr,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_ipacmd_local_addrs6 {
    pub count: u32,
    pub addr_length: u32,
    pub addrs: [qeth_ipacmd_local_addr6; ],
}

// Header for each IPA command
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_ipacmd_hdr {
    pub command: __u8,
    pub initiator: __u8,
    pub seqno: __u16,
    pub return_code: __u16,
    pub adapter_type: __u8,
    pub rel_adapter_no: __u8,
    pub prim_version_no: __u8,
    pub param_count: __u8,
    pub prot_version: __u16,
    pub assists: qeth_ipa_caps,
// C attribute field omitted
// The IPA command itself
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_ipa_cmd {
    pub hdr: qeth_ipacmd_hdr,
    pub setdelip4: qeth_ipacmd_setdelip4,
    pub setdelip6: qeth_ipacmd_setdelip6,
    pub setdelipm: qeth_ipacmd_setdelipm,
    pub setassparms: qeth_ipacmd_setassparms,
    pub setdelmac: qeth_ipacmd_layer2setdelmac,
    pub setdelvlan: qeth_ipacmd_layer2setdelvlan,
    pub create_destroy_addr: qeth_create_destroy_address,
    pub setadapterparms: qeth_ipacmd_setadpparms,
    pub setrtg: qeth_set_routing,
    pub diagass: qeth_ipacmd_diagass,
    pub sbp: qeth_ipacmd_setbridgeport,
    pub addrchange: qeth_ipacmd_addr_change,
    pub vnicc: qeth_ipacmd_vnicc,
    pub local_addrs4: qeth_ipacmd_local_addrs4,
    pub local_addrs6: qeth_ipacmd_local_addrs6,
    pub data: },
// C attribute field omitted

//
// special command for ARP processing.
// this is not included in setassparms command before, because we get
// problem with the size of struct qeth_ipacmd_setassparms otherwise
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_ipa_arp_return_codes {
    QETH_IPA_ARP_RC_SUCCESS      = 0x0000,
    QETH_IPA_ARP_RC_FAILED       = 0x0001,
    QETH_IPA_ARP_RC_NOTSUPP      = 0x0002,
    QETH_IPA_ARP_RC_OUT_OF_RANGE = 0x0003,
    QETH_IPA_ARP_RC_Q_NOTSUPP    = 0x0004,
    QETH_IPA_ARP_RC_Q_NO_DATA    = 0x0008,
}

    pub rc): qeth_ipa_return_codes,
    pub cmd): *const *const char qeth_get_ipa_cmd_name(enum qeth_ipa_cmds,
// Helper functions

//
// END OF   IP Assist related definitions
//
    pub CM_ENABLE: [extern unsigned char; ],
pub const CM_ENABLE_SIZE: c_uint = 0x63;
    pub CM_SETUP: [extern unsigned char; ],
pub const CM_SETUP_SIZE: c_uint = 0x64;
    pub ULP_ENABLE: [extern unsigned char; ],
pub const ULP_ENABLE_SIZE: c_uint = 0x6b;

pub const QETH_MPC_PROT_L2: c_uint = 0x08;
pub const QETH_MPC_PROT_L3: c_uint = 0x03;
    pub ULP_SETUP: [extern unsigned char; ],
pub const ULP_SETUP_SIZE: c_uint = 0x6c;
    pub DM_ACT: [extern unsigned char; ],
pub const DM_ACT_SIZE: c_uint = 0x55;
    pub IDX_ACTIVATE_READ: [extern unsigned char; ],
    pub IDX_ACTIVATE_WRITE: [extern unsigned char; ],
pub const IDX_ACTIVATE_SIZE: c_uint = 0x22;

pub const QETH_IDX_ACT_INVAL_FRAME: c_uint = 0x40;

pub const QETH_IDX_ACT_ERR_EXCL: c_uint = 0x19;
pub const QETH_IDX_ACT_ERR_AUTH: c_uint = 0x1E;
pub const QETH_IDX_ACT_ERR_AUTH_USER: c_uint = 0x20;
pub const QETH_IDX_TERMINATE: c_uint = 0xc0;
pub const QETH_IDX_TERMINATE_MASK: c_uint = 0xc0;
pub const QETH_IDX_TERM_BAD_TRANSPORT: c_uint = 0x41;
pub const QETH_IDX_TERM_BAD_TRANSPORT_VM: c_uint = 0xf6;

// (buffer + *(buffer + 0x0b) + 0x11) + 0x07))

