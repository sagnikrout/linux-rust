//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/quantenna/qtnfmac/qlink.h
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright (c) 2015-2016 Quantenna Communications. All rights reserved.

pub const QLINK_PROTO_VER_MAJOR_M: c_uint = 0xFFFF;
pub const QLINK_PROTO_VER_MAJOR_S: c_int = 16;
pub const QLINK_PROTO_VER_MINOR_M: c_uint = 0xFFFF;

pub const QLINK_PROTO_VER_MAJOR: c_int = 18;
pub const QLINK_PROTO_VER_MINOR: c_int = 1;

pub const QLINK_ALIGN: c_int = 4;
pub const QLINK_MACID_RSVD: c_uint = 0xFF;
pub const QLINK_VIFID_RSVD: c_uint = 0xFF;
// Common QLINK protocol messages definitions.
//
// enum qlink_msg_type - QLINK message types
//
// Used to distinguish between message types of QLINK protocol.
//
// @QLINK_MSG_TYPE_CMD: Message is carrying data of a command sent from
// driver to wireless hardware.
// @QLINK_MSG_TYPE_CMDRSP: Message is carrying data of a response to a command.
// Sent from wireless HW to driver in reply to previously issued command.
// @QLINK_MSG_TYPE_EVENT: Data for an event originated in wireless hardware and
// sent asynchronously to driver.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlink_msg_type {
    QLINK_MSG_TYPE_CMD	= 1,
    QLINK_MSG_TYPE_CMDRSP	= 2,
    QLINK_MSG_TYPE_EVENT	= 3
}

//
// struct qlink_msg_header - common QLINK protocol message header
//
// Portion of QLINK protocol header common for all message types.
//
// @type: Message type, one of &enum qlink_msg_type.
// @len: Total length of message including all headers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_msg_header {
    pub type: __le16,
    pub len: __le16,
    pub __packed: },
// Generic definitions of data and information carried in QLINK messages
//
// enum qlink_hw_capab - device capabilities.
//
// @QLINK_HW_CAPAB_REG_UPDATE: device can update it's regulatory region.
// @QLINK_HW_CAPAB_STA_INACT_TIMEOUT: device implements a logic to kick-out
// associated STAs due to inactivity. Inactivity timeout period is taken
// from QLINK_CMD_START_AP parameters.
// @QLINK_HW_CAPAB_DFS_OFFLOAD: device implements DFS offload functionality
// @QLINK_HW_CAPAB_SCAN_RANDOM_MAC_ADDR: device supports MAC Address
// Randomization in probe requests.
// @QLINK_HW_CAPAB_OBSS_SCAN: device can perform OBSS scanning.
// @QLINK_HW_CAPAB_HW_BRIDGE: device has hardware switch capabilities.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlink_hw_capab {
    QLINK_HW_CAPAB_REG_UPDATE = 0,
    QLINK_HW_CAPAB_STA_INACT_TIMEOUT,
    QLINK_HW_CAPAB_DFS_OFFLOAD,
    QLINK_HW_CAPAB_SCAN_RANDOM_MAC_ADDR,
    QLINK_HW_CAPAB_PWR_MGMT,
    QLINK_HW_CAPAB_OBSS_SCAN,
    QLINK_HW_CAPAB_SCAN_DWELL,
    QLINK_HW_CAPAB_SAE,
    QLINK_HW_CAPAB_HW_BRIDGE,
    QLINK_HW_CAPAB_NUM
}

//
// enum qlink_driver_capab - host driver capabilities.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlink_driver_capab {
    QLINK_DRV_CAPAB_NUM = 0
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlink_iface_type {
    QLINK_IFTYPE_AP		= 1,
    QLINK_IFTYPE_STATION	= 2,
    QLINK_IFTYPE_ADHOC	= 3,
    QLINK_IFTYPE_MONITOR	= 4,
    QLINK_IFTYPE_WDS	= 5,
    QLINK_IFTYPE_AP_VLAN	= 6,
}

//
// struct qlink_intf_info - information on virtual interface.
//
// Data describing a single virtual interface.
//
// @if_type: Mode of interface operation, one of &enum qlink_iface_type
// @vlanid: VLAN ID for AP_VLAN interface type
// @mac_addr: MAC address of virtual interface.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_intf_info {
    pub if_type: __le16,
    pub vlanid: __le16,
    pub mac_addr: [u8; ETH_ALEN],
    pub use4addr: u8,
    pub rsvd: [u8; 1],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlink_sta_flags {
    QLINK_STA_FLAG_INVALID		= 0,
    QLINK_STA_FLAG_AUTHORIZED		= BIT(0),
    QLINK_STA_FLAG_SHORT_PREAMBLE	= BIT(1),
    QLINK_STA_FLAG_WME			= BIT(2),
    QLINK_STA_FLAG_MFP			= BIT(3),
    QLINK_STA_FLAG_AUTHENTICATED		= BIT(4),
    QLINK_STA_FLAG_TDLS_PEER		= BIT(5),
    QLINK_STA_FLAG_ASSOCIATED		= BIT(6),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlink_channel_width {
    QLINK_CHAN_WIDTH_5 = 0,
    QLINK_CHAN_WIDTH_10,
    QLINK_CHAN_WIDTH_20_NOHT,
    QLINK_CHAN_WIDTH_20,
    QLINK_CHAN_WIDTH_40,
    QLINK_CHAN_WIDTH_80,
    QLINK_CHAN_WIDTH_80P80,
    QLINK_CHAN_WIDTH_160,
}

//
// struct qlink_channel - qlink control channel definition
//
// @hw_value: hardware-specific value for the channel
// @center_freq: center frequency in MHz
// @flags: channel flags from &enum qlink_channel_flags
// @band: band this channel belongs to
// @max_antenna_gain: maximum antenna gain in dBi
// @max_power: maximum transmission power (in dBm)
// @max_reg_power: maximum regulatory transmission power (in dBm)
// @dfs_state: current state of this channel.
// Only relevant if radar is required on this channel.
// @beacon_found: helper to regulatory code to indicate when a beacon
// has been found on this channel. Use regulatory_hint_found_beacon()
// to enable this, this is useful only on 5 GHz band.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_channel {
    pub hw_value: __le16,
    pub center_freq: __le16,
    pub flags: __le32,
    pub band: u8,
    pub max_antenna_gain: u8,
    pub max_power: u8,
    pub max_reg_power: u8,
    pub dfs_cac_ms: __le32,
    pub dfs_state: u8,
    pub beacon_found: u8,
    pub rsvd: [u8; 2],
    pub __packed: },
//
// struct qlink_chandef - qlink channel definition
//
// @chan: primary channel definition
// @center_freq1: center frequency of first segment
// @center_freq2: center frequency of second segment (80+80 only)
// @width: channel width, one of @enum qlink_channel_width
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_chandef {
    pub chan: qlink_channel,
    pub center_freq1: __le16,
    pub center_freq2: __le16,
    pub width: u8,
    pub rsvd: [u8; 3],
    pub __packed: },
pub const QLINK_MAX_NR_CIPHER_SUITES: c_int = 5;
pub const QLINK_MAX_NR_AKM_SUITES: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_auth_encr {
    pub wpa_versions: __le32,
    pub cipher_group: __le32,
    pub n_ciphers_pairwise: __le32,
    pub ciphers_pairwise: [__le32; QLINK_MAX_NR_CIPHER_SUITES],
    pub n_akm_suites: __le32,
    pub akm_suites: [__le32; QLINK_MAX_NR_AKM_SUITES],
    pub control_port_ethertype: __le16,
    pub auth_type: u8,
    pub privacy: u8,
    pub control_port: u8,
    pub control_port_no_encrypt: u8,
    pub rsvd: [u8; 2],
    pub __packed: },
//
// struct qlink_sta_info_state - station flags mask/value
//
// @mask: STA flags mask, bitmap of &enum qlink_sta_flags
// @value: STA flags values, bitmap of &enum qlink_sta_flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_sta_info_state {
    pub mask: __le32,
    pub value: __le32,
    pub __packed: },
//
// enum qlink_sr_ctrl_flags - control flags for spatial reuse parameter set
//
// @QLINK_SR_PSR_DISALLOWED: indicates whether or not PSR-based spatial reuse
// transmissions are allowed for STAs associated with the AP
// @QLINK_SR_NON_SRG_OBSS_PD_SR_DISALLOWED: indicates whether or not
// Non-SRG OBSS PD spatial reuse transmissions are allowed for STAs associated
// with the AP
// @NON_SRG_OFFSET_PRESENT: indicates whether or not Non-SRG OBSS PD Max offset
// field is valid in the element
// @QLINK_SR_SRG_INFORMATION_PRESENT: indicates whether or not SRG OBSS PD
// Min/Max offset fields ore valid in the element
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlink_sr_ctrl_flags {
    QLINK_SR_PSR_DISALLOWED                = BIT(0),
    QLINK_SR_NON_SRG_OBSS_PD_SR_DISALLOWED = BIT(1),
    QLINK_SR_NON_SRG_OFFSET_PRESENT        = BIT(2),
    QLINK_SR_SRG_INFORMATION_PRESENT       = BIT(3),
}

//
// struct qlink_sr_params - spatial reuse parameters
//
// @sr_control: spatial reuse control field; flags contained in this field are
// defined in @qlink_sr_ctrl_flags
// @non_srg_obss_pd_max: added to -82 dBm to generate the value of the
// Non-SRG OBSS PD Max parameter
// @srg_obss_pd_min_offset: added to -82 dBm to generate the value of the
// SRG OBSS PD Min parameter
// @srg_obss_pd_max_offset: added to -82 dBm to generate the value of the
// SRG PBSS PD Max parameter
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_sr_params {
    pub sr_control: u8,
    pub non_srg_obss_pd_max: u8,
    pub srg_obss_pd_min_offset: u8,
    pub srg_obss_pd_max_offset: u8,
    pub __packed: },
// QLINK Command messages related definitions
//
// enum qlink_cmd_type - list of supported commands
//
// Commands are QLINK messages of type @QLINK_MSG_TYPE_CMD, sent by driver to
// wireless network device for processing. Device is expected to send back a
// reply message of type &QLINK_MSG_TYPE_CMDRSP, containing at least command
// execution status (one of &enum qlink_cmd_result). Reply message
// may also contain data payload specific to the command type.
//
// @QLINK_CMD_SEND_FRAME: send specified frame over the air; firmware will
// encapsulate 802.3 packet into 802.11 frame automatically.
// @QLINK_CMD_BAND_INFO_GET: for the specified MAC and specified band, get
// the band's description including number of operational channels and
// info on each channel, HT/VHT capabilities, supported rates etc.
// This command is generic to a specified MAC, interface index must be set
// to QLINK_VIFID_RSVD in command header.
// @QLINK_CMD_REG_NOTIFY: notify device about regulatory domain change. This
// command is supported only if device reports QLINK_HW_SUPPORTS_REG_UPDATE
// capability.
// @QLINK_CMD_START_CAC: start radar detection procedure on a specified channel.
// @QLINK_CMD_TXPWR: get or set current channel transmit power for
// the specified MAC.
// @QLINK_CMD_NDEV_EVENT: signalizes changes made with a corresponding network
// device.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlink_cmd_type {
    QLINK_CMD_FW_INIT		= 0x0001,
    QLINK_CMD_FW_DEINIT		= 0x0002,
    QLINK_CMD_REGISTER_MGMT		= 0x0003,
    QLINK_CMD_SEND_FRAME		= 0x0004,
    QLINK_CMD_MGMT_SET_APPIE	= 0x0005,
    QLINK_CMD_PHY_PARAMS_SET	= 0x0012,
    QLINK_CMD_GET_HW_INFO		= 0x0013,
    QLINK_CMD_MAC_INFO		= 0x0014,
    QLINK_CMD_ADD_INTF		= 0x0015,
    QLINK_CMD_DEL_INTF		= 0x0016,
    QLINK_CMD_CHANGE_INTF		= 0x0017,
    QLINK_CMD_UPDOWN_INTF		= 0x0018,
    QLINK_CMD_REG_NOTIFY		= 0x0019,
    QLINK_CMD_BAND_INFO_GET		= 0x001A,
    QLINK_CMD_CHAN_SWITCH		= 0x001B,
    QLINK_CMD_CHAN_GET		= 0x001C,
    QLINK_CMD_START_CAC		= 0x001D,
    QLINK_CMD_START_AP		= 0x0021,
    QLINK_CMD_STOP_AP		= 0x0022,
    QLINK_CMD_SET_MAC_ACL		= 0x0023,
    QLINK_CMD_GET_STA_INFO		= 0x0030,
    QLINK_CMD_ADD_KEY		= 0x0040,
    QLINK_CMD_DEL_KEY		= 0x0041,
    QLINK_CMD_SET_DEFAULT_KEY	= 0x0042,
    QLINK_CMD_SET_DEFAULT_MGMT_KEY	= 0x0043,
    QLINK_CMD_CHANGE_STA		= 0x0051,
    QLINK_CMD_DEL_STA		= 0x0052,
    QLINK_CMD_SCAN			= 0x0053,
    QLINK_CMD_CHAN_STATS		= 0x0054,
    QLINK_CMD_NDEV_EVENT		= 0x0055,
    QLINK_CMD_CONNECT		= 0x0060,
    QLINK_CMD_DISCONNECT		= 0x0061,
    QLINK_CMD_PM_SET		= 0x0062,
    QLINK_CMD_WOWLAN_SET		= 0x0063,
    QLINK_CMD_EXTERNAL_AUTH		= 0x0066,
    QLINK_CMD_TXPWR			= 0x0067,
    QLINK_CMD_UPDATE_OWE		= 0x0068,
}

//
// struct qlink_cmd - QLINK command message header
//
// Header used for QLINK messages of QLINK_MSG_TYPE_CMD type.
//
// @mhdr: Common QLINK message header.
// @cmd_id: command id, one of &enum qlink_cmd_type.
// @seq_num: sequence number of command message, used for matching with
// response message.
// @macid: index of physical radio device the command is destined to or
// QLINK_MACID_RSVD if not applicable.
// @vifid: index of virtual wireless interface on specified @macid the command
// is destined to or QLINK_VIFID_RSVD if not applicable.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_cmd {
    pub mhdr: qlink_msg_header,
    pub cmd_id: __le16,
    pub seq_num: __le16,
    pub macid: u8,
    pub vifid: u8,
    pub rsvd: [u8; 2],
    pub __packed: },
//
// struct qlink_cmd_init_fw - data for QLINK_CMD_FW_INIT
//
// Initialize firmware based on specified host configuration. This is the first
// command sent to wifi card and it's fixed part should never be changed, any
// additions must be done by appending TLVs.
// If wifi card can not operate with a specified parameters it will return
// error.
//
// @qlink_proto_ver: QLINK protocol version used by host driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_cmd_init_fw {
    pub chdr: qlink_cmd,
    pub qlink_proto_ver: __le32,
    pub var_info: [u8; ],
    pub __packed: },
//
// struct qlink_cmd_manage_intf - interface management command
//
// Data for interface management commands QLINK_CMD_ADD_INTF, QLINK_CMD_DEL_INTF
// and QLINK_CMD_CHANGE_INTF.
//
// @intf_info: interface description.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_cmd_manage_intf {
    pub chdr: qlink_cmd,
    pub intf_info: qlink_intf_info,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlink_mgmt_frame_type {
    QLINK_MGMT_FRAME_ASSOC_REQ	= 0x00,
    QLINK_MGMT_FRAME_ASSOC_RESP	= 0x01,
    QLINK_MGMT_FRAME_REASSOC_REQ	= 0x02,
    QLINK_MGMT_FRAME_REASSOC_RESP	= 0x03,
    QLINK_MGMT_FRAME_PROBE_REQ	= 0x04,
    QLINK_MGMT_FRAME_PROBE_RESP	= 0x05,
    QLINK_MGMT_FRAME_BEACON		= 0x06,
    QLINK_MGMT_FRAME_ATIM		= 0x07,
    QLINK_MGMT_FRAME_DISASSOC	= 0x08,
    QLINK_MGMT_FRAME_AUTH		= 0x09,
    QLINK_MGMT_FRAME_DEAUTH		= 0x0A,
    QLINK_MGMT_FRAME_ACTION		= 0x0B,

    QLINK_MGMT_FRAME_TYPE_COUNT
}

//
// struct qlink_cmd_mgmt_frame_register - data for QLINK_CMD_REGISTER_MGMT
//
// @frame_type: MGMT frame type the registration request describes, one of
// &enum qlink_mgmt_frame_type.
// @do_register: 0 - unregister, otherwise register for reception of specified
// MGMT frame type.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_cmd_mgmt_frame_register {
    pub chdr: qlink_cmd,
    pub frame_type: __le16,
    pub do_register: u8,
    pub rsvd: [u8; 1],
    pub __packed: },
//
// @QLINK_FRAME_TX_FLAG_8023: frame has a 802.3 header; if not set, frame
// is a 802.11 encapsulated.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlink_frame_tx_flags {
    QLINK_FRAME_TX_FLAG_OFFCHAN	= BIT(0),
    QLINK_FRAME_TX_FLAG_NO_CCK	= BIT(1),
    QLINK_FRAME_TX_FLAG_ACK_NOWAIT	= BIT(2),
    QLINK_FRAME_TX_FLAG_8023	= BIT(3),
}

//
// struct qlink_cmd_frame_tx - data for QLINK_CMD_SEND_FRAME command
//
// @cookie: opaque request identifier.
// @freq: Frequency to use for frame transmission.
// @flags: Transmission flags, one of &enum qlink_frame_tx_flags.
// @frame_data: frame to transmit.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_cmd_frame_tx {
    pub chdr: qlink_cmd,
    pub cookie: __le32,
    pub freq: __le16,
    pub flags: __le16,
    pub frame_data: [u8; ],
    pub __packed: },
//
// struct qlink_cmd_get_sta_info - data for QLINK_CMD_GET_STA_INFO command
//
// @sta_addr: MAC address of the STA statistics is requested for.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_cmd_get_sta_info {
    pub chdr: qlink_cmd,
    pub sta_addr: [u8; ETH_ALEN],
    pub rsvd: [u8; 2],
    pub __packed: },
//
// struct qlink_cmd_add_key - data for QLINK_CMD_ADD_KEY command.
//
// @key_index: index of the key being installed.
// @pairwise: whether to use pairwise key.
// @addr: MAC address of a STA key is being installed to.
// @cipher: cipher suite.
// @vlanid: VLAN ID for AP_VLAN interface type
// @key_data: key data itself.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_cmd_add_key {
    pub chdr: qlink_cmd,
    pub key_index: u8,
    pub pairwise: u8,
    pub addr: [u8; ETH_ALEN],
    pub cipher: __le32,
    pub vlanid: __le16,
    pub rsvd: [u8; 2],
    pub key_data: [u8; ],
    pub __packed: },
//
// struct qlink_cmd_del_key_req - data for QLINK_CMD_DEL_KEY command
//
// @key_index: index of the key being removed.
// @pairwise: whether to use pairwise key.
// @addr: MAC address of a STA for which a key is removed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_cmd_del_key {
    pub chdr: qlink_cmd,
    pub key_index: u8,
    pub pairwise: u8,
    pub addr: [u8; ETH_ALEN],
    pub __packed: },
//
// struct qlink_cmd_set_def_key - data for QLINK_CMD_SET_DEFAULT_KEY command
//
// @key_index: index of the key to be set as default one.
// @unicast: key is unicast.
// @multicast: key is multicast.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_cmd_set_def_key {
    pub chdr: qlink_cmd,
    pub key_index: u8,
    pub unicast: u8,
    pub multicast: u8,
    pub rsvd: [u8; 1],
    pub __packed: },
//
// struct qlink_cmd_set_def_mgmt_key - data for QLINK_CMD_SET_DEFAULT_MGMT_KEY
//
// @key_index: index of the key to be set as default MGMT key.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_cmd_set_def_mgmt_key {
    pub chdr: qlink_cmd,
    pub key_index: u8,
    pub rsvd: [u8; 3],
    pub __packed: },
//
// struct qlink_cmd_change_sta - data for QLINK_CMD_CHANGE_STA command
//
// @flag_update: STA flags to update
// @if_type: Mode of interface operation, one of &enum qlink_iface_type
// @vlanid: VLAN ID to assign to specific STA
// @sta_addr: address of the STA for which parameters are set.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_cmd_change_sta {
    pub chdr: qlink_cmd,
    pub flag_update: qlink_sta_info_state,
    pub if_type: __le16,
    pub vlanid: __le16,
    pub sta_addr: [u8; ETH_ALEN],
    pub rsvd: [u8; 2],
    pub __packed: },
//
// struct qlink_cmd_del_sta - data for QLINK_CMD_DEL_STA command.
//
// See &struct station_del_parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_cmd_del_sta {
    pub chdr: qlink_cmd,
    pub reason_code: __le16,
    pub sta_addr: [u8; ETH_ALEN],
    pub subtype: u8,
    pub rsvd: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlink_sta_connect_flags {
    QLINK_STA_CONNECT_DISABLE_HT	= BIT(0),
    QLINK_STA_CONNECT_DISABLE_VHT	= BIT(1),
    QLINK_STA_CONNECT_USE_RRM	= BIT(2),
}

//
// struct qlink_cmd_connect - data for QLINK_CMD_CONNECT command
//
// @bssid: BSSID of the BSS to connect to.
// @bssid_hint: recommended AP BSSID for initial connection to the BSS or
// 00:00:00:00:00:00 if not specified.
// @prev_bssid: previous BSSID, if specified (not 00:00:00:00:00:00) indicates
// a request to reassociate.
// @bg_scan_period: period of background scan.
// @flags: one of &enum qlink_sta_connect_flags.
// @ht_capa: HT Capabilities overrides.
// @ht_capa_mask: The bits of ht_capa which are to be used.
// @vht_capa: VHT Capability overrides
// @vht_capa_mask: The bits of vht_capa which are to be used.
// @aen: authentication information.
// @mfp: whether to use management frame protection.
// @payload: variable portion of connection request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_cmd_connect {
    pub chdr: qlink_cmd,
    pub bssid: [u8; ETH_ALEN],
    pub bssid_hint: [u8; ETH_ALEN],
    pub prev_bssid: [u8; ETH_ALEN],
    pub bg_scan_period: __le16,
    pub flags: __le32,
    pub ht_capa: ieee80211_ht_cap,
    pub ht_capa_mask: ieee80211_ht_cap,
    pub vht_capa: ieee80211_vht_cap,
    pub vht_capa_mask: ieee80211_vht_cap,
    pub aen: qlink_auth_encr,
    pub mfp: u8,
    pub pbss: u8,
    pub rsvd: [u8; 2],
    pub payload: [u8; ],
    pub __packed: },
//
// struct qlink_cmd_external_auth - data for QLINK_CMD_EXTERNAL_AUTH command
//
// @bssid: BSSID of the BSS to connect to
// @status: authentication status code
// @payload: variable portion of connection request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_cmd_external_auth {
    pub chdr: qlink_cmd,
    pub peer: [u8; ETH_ALEN],
    pub status: __le16,
    pub payload: [u8; ],
    pub __packed: },
//
// struct qlink_cmd_disconnect - data for QLINK_CMD_DISCONNECT command
//
// @reason: code of the reason of disconnect, see &enum ieee80211_reasoncode.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_cmd_disconnect {
    pub chdr: qlink_cmd,
    pub reason: __le16,
    pub rsvd: [u8; 2],
    pub __packed: },
//
// struct qlink_cmd_updown - data for QLINK_CMD_UPDOWN_INTF command
//
// @if_up: bring specified interface DOWN (if_up==0) or UP (otherwise).
// Interface is specified in common command header @chdr.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_cmd_updown {
    pub chdr: qlink_cmd,
    pub if_up: u8,
    pub rsvd: [u8; 3],
    pub __packed: },
//
// enum qlink_band - a list of frequency bands
//
// @QLINK_BAND_2GHZ: 2.4GHz band
// @QLINK_BAND_5GHZ: 5GHz band
// @QLINK_BAND_60GHZ: 60GHz band
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlink_band {
    QLINK_BAND_2GHZ = BIT(0),
    QLINK_BAND_5GHZ = BIT(1),
    QLINK_BAND_60GHZ = BIT(2),
}

//
// struct qlink_cmd_band_info_get - data for QLINK_CMD_BAND_INFO_GET command
//
// @band: a PHY band for which information is queried, one of @enum qlink_band
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_cmd_band_info_get {
    pub chdr: qlink_cmd,
    pub band: u8,
    pub rsvd: [u8; 3],
    pub __packed: },
//
// struct qlink_cmd_get_chan_stats - data for QLINK_CMD_CHAN_STATS command
//
// @channel_freq: channel center frequency
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_cmd_get_chan_stats {
    pub chdr: qlink_cmd,
    pub channel_freq: __le32,
    pub __packed: },
//
// enum qlink_reg_initiator - Indicates the initiator of a reg domain request
//
// See &enum nl80211_reg_initiator for more info.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlink_reg_initiator {
    QLINK_REGDOM_SET_BY_CORE,
    QLINK_REGDOM_SET_BY_USER,
    QLINK_REGDOM_SET_BY_DRIVER,
    QLINK_REGDOM_SET_BY_COUNTRY_IE,
}

//
// enum qlink_user_reg_hint_type - type of user regulatory hint
//
// See &enum nl80211_user_reg_hint_type for more info.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlink_user_reg_hint_type {
    QLINK_USER_REG_HINT_USER	= 0,
    QLINK_USER_REG_HINT_CELL_BASE	= 1,
    QLINK_USER_REG_HINT_INDOOR	= 2,
}

//
// struct qlink_cmd_reg_notify - data for QLINK_CMD_REG_NOTIFY command
//
// @alpha2: the ISO / IEC 3166 alpha2 country code.
// @initiator: which entity sent the request, one of &enum qlink_reg_initiator.
// @user_reg_hint_type: type of hint for QLINK_REGDOM_SET_BY_USER request, one
// of &enum qlink_user_reg_hint_type.
// @num_channels: number of &struct qlink_tlv_channel in a variable portion of a
// payload.
// @dfs_region: one of &enum qlink_dfs_regions.
// @slave_radar: whether slave device should enable radar detection.
// @dfs_offload: enable or disable DFS offload to firmware.
// @info: variable portion of regulatory notifier callback.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_cmd_reg_notify {
    pub chdr: qlink_cmd,
    pub alpha2: [u8; 2],
    pub initiator: u8,
    pub user_reg_hint_type: u8,
    pub num_channels: u8,
    pub dfs_region: u8,
    pub slave_radar: u8,
    pub dfs_offload: u8,
    pub info: [u8; ],
    pub __packed: },
//
// enum qlink_chan_sw_flags - channel switch control flags
//
// @QLINK_CHAN_SW_RADAR_REQUIRED: whether radar detection is required on a new
// channel.
// @QLINK_CHAN_SW_BLOCK_TX: whether transmissions should be blocked while
// changing a channel.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlink_chan_sw_flags {
    QLINK_CHAN_SW_RADAR_REQUIRED = BIT(0),
    QLINK_CHAN_SW_BLOCK_TX = BIT(1),
}

//
// struct qlink_cmd_chan_switch - data for QLINK_CMD_CHAN_SWITCH command
//
// @channel: channel to switch to.
// @flags: flags to control channel switch, bitmap of &enum qlink_chan_sw_flags.
// @beacon_count: number of beacons until switch
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_cmd_chan_switch {
    pub chdr: qlink_cmd,
    pub channel: qlink_chandef,
    pub flags: __le64,
    pub n_counter_offsets_beacon: __le32,
    pub n_counter_offsets_presp: __le32,
    pub beacon_count: u8,
    pub rsvd: [u8; 3],
    pub __packed: },
//
// enum qlink_hidden_ssid - values for %NL80211_ATTR_HIDDEN_SSID
//
// Refer to &enum nl80211_hidden_ssid
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlink_hidden_ssid {
    QLINK_HIDDEN_SSID_NOT_IN_USE,
    QLINK_HIDDEN_SSID_ZERO_LEN,
    QLINK_HIDDEN_SSID_ZERO_CONTENTS
}

//
// struct qlink_cmd_start_ap - data for QLINK_CMD_START_AP command
//
// @beacon_interval: beacon interval
// @inactivity_timeout: station's inactivity period in seconds
// @dtim_period: DTIM period
// @hidden_ssid: whether to hide the SSID, one of &enum qlink_hidden_ssid
// @smps_mode: SMPS mode
// @ht_required: stations must support HT
// @vht_required: stations must support VHT
// @aen: encryption info
// @sr_params: spatial reuse parameters
// @twt_responder: enable Target Wake Time
// @info: variable configurations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_cmd_start_ap {
    pub chdr: qlink_cmd,
    pub beacon_interval: __le16,
    pub inactivity_timeout: __le16,
    pub dtim_period: u8,
    pub hidden_ssid: u8,
    pub smps_mode: u8,
    pub p2p_ctwindow: u8,
    pub p2p_opp_ps: u8,
    pub pbss: u8,
    pub ht_required: u8,
    pub vht_required: u8,
    pub aen: qlink_auth_encr,
    pub sr_params: qlink_sr_params,
    pub twt_responder: u8,
    pub rsvd: [u8; 3],
    pub info: [u8; ],
    pub __packed: },
//
// struct qlink_cmd_start_cac - data for QLINK_CMD_START_CAC command
//
// @chan: a channel to start a radar detection procedure on.
// @cac_time_ms: CAC time.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_cmd_start_cac {
    pub chdr: qlink_cmd,
    pub chan: qlink_chandef,
    pub cac_time_ms: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlink_acl_policy {
    QLINK_ACL_POLICY_ACCEPT_UNLESS_LISTED,
    QLINK_ACL_POLICY_DENY_UNLESS_LISTED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_mac_address {
    pub addr: [u8; ETH_ALEN],
    pub __packed: },
//
// struct qlink_acl_data - ACL data
//
// @policy: filter policy, one of &enum qlink_acl_policy.
// @num_entries: number of MAC addresses in array.
// @mac_address: MAC addresses array.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_acl_data {
    pub policy: __le32,
    pub num_entries: __le32,
    pub mac_addrs: [qlink_mac_address; ],
    pub __packed: },
//
// enum qlink_pm_mode - Power Management mode
//
// @QLINK_PM_OFF: normal mode, no power saving enabled
// @QLINK_PM_AUTO_STANDBY: enable auto power save mode
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlink_pm_mode {
    QLINK_PM_OFF		= 0,
    QLINK_PM_AUTO_STANDBY	= 1,
}

//
// struct qlink_cmd_pm_set - data for QLINK_CMD_PM_SET command
//
// @pm_standby timer: period of network inactivity in seconds before
// putting a radio in power save mode
// @pm_mode: power management mode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_cmd_pm_set {
    pub chdr: qlink_cmd,
    pub pm_standby_timer: __le32,
    pub pm_mode: u8,
    pub rsvd: [u8; 3],
    pub __packed: },
//
// enum qlink_txpwr_op - transmit power operation type
// @QLINK_TXPWR_SET: set tx power
// @QLINK_TXPWR_GET: get current tx power setting
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlink_txpwr_op {
    QLINK_TXPWR_SET,
    QLINK_TXPWR_GET
}

//
// struct qlink_cmd_txpwr - get or set current transmit power
//
// @txpwr: new transmit power setting, in mBm
// @txpwr_setting: transmit power setting type, one of
// &enum nl80211_tx_power_setting
// @op_type: type of operation, one of &enum qlink_txpwr_op
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_cmd_txpwr {
    pub chdr: qlink_cmd,
    pub txpwr: __le32,
    pub txpwr_setting: u8,
    pub op_type: u8,
    pub rsvd: [u8; 2],
    pub __packed: },
//
// enum qlink_wowlan_trigger
//
// @QLINK_WOWLAN_TRIG_DISCONNECT: wakeup on disconnect
// @QLINK_WOWLAN_TRIG_MAGIC_PKT: wakeup on magic packet
// @QLINK_WOWLAN_TRIG_PATTERN_PKT: wakeup on user-defined packet
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlink_wowlan_trigger {
    QLINK_WOWLAN_TRIG_DISCONNECT	= BIT(0),
    QLINK_WOWLAN_TRIG_MAGIC_PKT	= BIT(1),
    QLINK_WOWLAN_TRIG_PATTERN_PKT	= BIT(2),
}

//
// struct qlink_cmd_wowlan_set - data for QLINK_CMD_WOWLAN_SET command
//
// @triggers: requested bitmask of WoWLAN triggers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_cmd_wowlan_set {
    pub chdr: qlink_cmd,
    pub triggers: __le32,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlink_ndev_event_type {
    QLINK_NDEV_EVENT_CHANGEUPPER,
}

//
// struct qlink_cmd_ndev_event - data for QLINK_CMD_NDEV_EVENT command
//
// @event: type of event, one of &enum qlink_ndev_event_type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_cmd_ndev_event {
    pub chdr: qlink_cmd,
    pub event: __le16,
    pub rsvd: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlink_ndev_upper_type {
    QLINK_NDEV_UPPER_TYPE_NONE,
    QLINK_NDEV_UPPER_TYPE_BRIDGE,
}

//
// struct qlink_cmd_ndev_changeupper - data for QLINK_NDEV_EVENT_CHANGEUPPER
//
// @br_domain: layer 2 broadcast domain ID that ndev is a member of
// @upper_type: type of upper device, one of &enum qlink_ndev_upper_type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_cmd_ndev_changeupper {
    pub nehdr: qlink_cmd_ndev_event,
    pub flags: __le64,
    pub br_domain: __le32,
    pub netspace_id: __le32,
    pub vlanid: __le16,
    pub upper_type: u8,
    pub rsvd: [u8; 1],
    pub __packed: },
//
// enum qlink_scan_flags -  scan request control flags
//
// Scan flags are used to control QLINK_CMD_SCAN behavior.
//
// @QLINK_SCAN_FLAG_FLUSH: flush cache before scanning.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlink_scan_flags {
    QLINK_SCAN_FLAG_FLUSH = BIT(0),
    QLINK_SCAN_FLAG_DURATION_MANDATORY = BIT(1),
}

//
// struct qlink_cmd_scan - data for QLINK_CMD_SCAN command
//
// @flags: scan flags, a bitmap of &enum qlink_scan_flags.
// @n_ssids: number of WLAN_EID_SSID TLVs expected in variable portion of the
// command.
// @n_channels: number of QTN_TLV_ID_CHANNEL TLVs expected in variable payload.
// @active_dwell: time spent on a single channel for an active scan.
// @passive_dwell: time spent on a single channel for a passive scan.
// @sample_duration: total duration of sampling a single channel during a scan
// including off-channel dwell time and operating channel time.
// @bssid: specific BSSID to scan for or a broadcast BSSID.
// @scan_width: channel width to use, one of &enum qlink_channel_width.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_cmd_scan {
    pub chdr: qlink_cmd,
    pub flags: __le64,
    pub n_ssids: __le16,
    pub n_channels: __le16,
    pub active_dwell: __le16,
    pub passive_dwell: __le16,
    pub sample_duration: __le16,
    pub bssid: [u8; ETH_ALEN],
    pub scan_width: u8,
    pub rsvd: [u8; 3],
    pub var_info: [u8; ],
    pub __packed: },
//
// struct qlink_cmd_update_owe - data for QLINK_CMD_UPDATE_OWE_INFO command
//
// @peer: MAC of the peer device for which OWE processing has been completed
// @status: OWE external processing status code
// @ies: IEs for the peer constructed by the user space
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_cmd_update_owe {
    pub chdr: qlink_cmd,
    pub peer: [u8; ETH_ALEN],
    pub status: __le16,
    pub ies: [u8; ],
    pub __packed: },
// QLINK Command Responses messages related definitions
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlink_cmd_result {
    QLINK_CMD_RESULT_OK = 0,
    QLINK_CMD_RESULT_INVALID,
    QLINK_CMD_RESULT_ENOTSUPP,
    QLINK_CMD_RESULT_ENOTFOUND,
    QLINK_CMD_RESULT_EALREADY,
    QLINK_CMD_RESULT_EADDRINUSE,
    QLINK_CMD_RESULT_EADDRNOTAVAIL,
    QLINK_CMD_RESULT_EBUSY,
}

//
// struct qlink_resp - QLINK command response message header
//
// Header used for QLINK messages of QLINK_MSG_TYPE_CMDRSP type.
//
// @mhdr: see &struct qlink_msg_header.
// @cmd_id: command ID the response corresponds to, one of &enum qlink_cmd_type.
// @seq_num: sequence number of command message, used for matching with
// response message.
// @result: result of the command execution, one of &enum qlink_cmd_result.
// @macid: index of physical radio device the response is sent from or
// QLINK_MACID_RSVD if not applicable.
// @vifid: index of virtual wireless interface on specified @macid the response
// is sent from or QLINK_VIFID_RSVD if not applicable.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_resp {
    pub mhdr: qlink_msg_header,
    pub cmd_id: __le16,
    pub seq_num: __le16,
    pub result: __le16,
    pub macid: u8,
    pub vifid: u8,
    pub __packed: },
//
// struct qlink_resp_init_fw - response for QLINK_CMD_FW_INIT
//
// @qlink_proto_ver: QLINK protocol version used by wifi card firmware.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_resp_init_fw {
    pub rhdr: qlink_resp,
    pub qlink_proto_ver: __le32,
    pub __packed: },
//
// enum qlink_dfs_regions - regulatory DFS regions
//
// Corresponds to &enum nl80211_dfs_regions.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlink_dfs_regions {
    QLINK_DFS_UNSET	= 0,
    QLINK_DFS_FCC	= 1,
    QLINK_DFS_ETSI	= 2,
    QLINK_DFS_JP	= 3,
}

//
// struct qlink_resp_get_mac_info - response for QLINK_CMD_MAC_INFO command
//
// Data describing specific physical device providing wireless MAC
// functionality.
//
// @dev_mac: MAC address of physical WMAC device (used for first BSS on
// specified WMAC).
// @num_tx_chain: Number of transmit chains used by WMAC.
// @num_rx_chain: Number of receive chains used by WMAC.
// @vht_cap_mod_mask: mask specifying which VHT capabilities can be altered.
// @ht_cap_mod_mask: mask specifying which HT capabilities can be altered.
// @max_scan_ssids: maximum number of SSIDs the device can scan for in any scan.
// @bands_cap: wireless bands WMAC can operate in, bitmap of &enum qlink_band.
// @max_ap_assoc_sta: Maximum number of associations supported by WMAC.
// @radar_detect_widths: bitmask of channels BW for which WMAC can detect radar.
// @alpha2: country code ID firmware is configured to.
// @n_reg_rules: number of regulatory rules TLVs in variable portion of the
// message.
// @dfs_region: regulatory DFS region, one of &enum qlink_dfs_regions.
// @var_info: variable-length WMAC info data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_resp_get_mac_info {
    pub rhdr: qlink_resp,
    pub dev_mac: [u8; ETH_ALEN],
    pub num_tx_chain: u8,
    pub num_rx_chain: u8,
    pub vht_cap_mod_mask: ieee80211_vht_cap,
    pub ht_cap_mod_mask: ieee80211_ht_cap,
    pub max_ap_assoc_sta: __le16,
    pub hw_version: __le32,
    pub probe_resp_offload: __le32,
    pub bss_select_support: __le32,
    pub n_addresses: __le16,
    pub radar_detect_widths: __le16,
    pub max_remain_on_channel_duration: __le16,
    pub max_acl_mac_addrs: __le16,
    pub frag_threshold: __le32,
    pub rts_threshold: __le32,
    pub retry_short: u8,
    pub retry_long: u8,
    pub coverage_class: u8,
    pub max_scan_ssids: u8,
    pub max_sched_scan_reqs: u8,
    pub max_sched_scan_ssids: u8,
    pub max_match_sets: u8,
    pub max_adj_channel_rssi_comp: u8,
    pub max_scan_ie_len: __le16,
    pub max_sched_scan_ie_len: __le16,
    pub max_sched_scan_plans: __le32,
    pub max_sched_scan_plan_interval: __le32,
    pub max_sched_scan_plan_iterations: __le32,
    pub n_cipher_suites: u8,
    pub n_akm_suites: u8,
    pub max_num_pmkids: u8,
    pub num_iftype_ext_capab: u8,
    pub extended_capabilities_len: u8,
    pub max_data_retry_count: u8,
    pub n_iface_combinations: u8,
    pub max_num_csa_counters: u8,
    pub bands_cap: u8,
    pub alpha2: [u8; 2],
    pub n_reg_rules: u8,
    pub dfs_region: u8,
    pub rsvd: [u8; 3],
    pub var_info: [u8; ],
    pub __packed: },
//
// struct qlink_resp_get_hw_info - response for QLINK_CMD_GET_HW_INFO command
//
// Description of wireless hardware capabilities and features.
//
// @fw_ver: wireless hardware firmware version.
// @num_mac: Number of separate physical radio devices provided by hardware.
// @mac_bitmap: Bitmap of MAC IDs that are active and can be used in firmware.
// @total_tx_chains: total number of transmit chains used by device.
// @total_rx_chains: total number of receive chains.
// @info: variable-length HW info.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_resp_get_hw_info {
    pub rhdr: qlink_resp,
    pub fw_ver: __le32,
    pub bld_tmstamp: __le32,
    pub plat_id: __le32,
    pub hw_ver: __le32,
    pub num_mac: u8,
    pub mac_bitmap: u8,
    pub total_tx_chain: u8,
    pub total_rx_chain: u8,
    pub info: [u8; ],
    pub __packed: },
//
// struct qlink_resp_manage_intf - response for interface management commands
//
// Response data for QLINK_CMD_ADD_INTF and QLINK_CMD_CHANGE_INTF commands.
//
// @rhdr: Common Command Response message header.
// @intf_info: interface description.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_resp_manage_intf {
    pub rhdr: qlink_resp,
    pub intf_info: qlink_intf_info,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlink_sta_info_rate_flags {
    QLINK_STA_INFO_RATE_FLAG_HT_MCS		= BIT(0),
    QLINK_STA_INFO_RATE_FLAG_VHT_MCS	= BIT(1),
    QLINK_STA_INFO_RATE_FLAG_SHORT_GI	= BIT(2),
    QLINK_STA_INFO_RATE_FLAG_60G		= BIT(3),
    QLINK_STA_INFO_RATE_FLAG_HE_MCS		= BIT(4),
}

//
// struct qlink_resp_get_sta_info - response for QLINK_CMD_GET_STA_INFO command
//
// Response data containing statistics for specified STA.
//
// @sta_addr: MAC address of STA the response carries statistic for.
// @info: variable statistics for specified STA.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_resp_get_sta_info {
    pub rhdr: qlink_resp,
    pub sta_addr: [u8; ETH_ALEN],
    pub rsvd: [u8; 2],
    pub info: [u8; ],
    pub __packed: },
//
// struct qlink_resp_band_info_get - response for QLINK_CMD_BAND_INFO_GET cmd
//
// @band: frequency band that the response describes, one of @enum qlink_band.
// @num_chans: total number of channels info TLVs contained in reply.
// @num_bitrates: total number of bitrate TLVs contained in reply.
// @info: variable-length info portion.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_resp_band_info_get {
    pub rhdr: qlink_resp,
    pub band: u8,
    pub num_chans: u8,
    pub num_bitrates: u8,
    pub rsvd: [u8; 1],
    pub info: [u8; ],
    pub __packed: },
//
// struct qlink_resp_get_chan_stats - response for QLINK_CMD_CHAN_STATS cmd
//
// @chan_freq: center frequency for a channel the report is sent for.
// @info: variable-length channel info.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_resp_get_chan_stats {
    pub rhdr: qlink_resp,
    pub chan_freq: __le32,
    pub info: [u8; ],
    pub __packed: },
//
// struct qlink_resp_channel_get - response for QLINK_CMD_CHAN_GET command
//
// @chan: definition of current operating channel.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_resp_channel_get {
    pub rhdr: qlink_resp,
    pub chan: qlink_chandef,
    pub __packed: },
//
// struct qlink_resp_txpwr - response for QLINK_CMD_TXPWR command
//
// This response is intended for QLINK_TXPWR_GET operation and does not
// contain any meaningful information in case of QLINK_TXPWR_SET operation.
//
// @txpwr: current transmit power setting, in mBm
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_resp_txpwr {
    pub rhdr: qlink_resp,
    pub txpwr: __le32,
    pub __packed: },
// QLINK Events messages related definitions
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlink_event_type {
    QLINK_EVENT_STA_ASSOCIATED	= 0x0021,
    QLINK_EVENT_STA_DEAUTH		= 0x0022,
    QLINK_EVENT_MGMT_RECEIVED	= 0x0023,
    QLINK_EVENT_SCAN_RESULTS	= 0x0024,
    QLINK_EVENT_SCAN_COMPLETE	= 0x0025,
    QLINK_EVENT_BSS_JOIN		= 0x0026,
    QLINK_EVENT_BSS_LEAVE		= 0x0027,
    QLINK_EVENT_FREQ_CHANGE		= 0x0028,
    QLINK_EVENT_RADAR		= 0x0029,
    QLINK_EVENT_EXTERNAL_AUTH	= 0x0030,
    QLINK_EVENT_MIC_FAILURE		= 0x0031,
    QLINK_EVENT_UPDATE_OWE		= 0x0032,
}

//
// struct qlink_event - QLINK event message header
//
// Header used for QLINK messages of QLINK_MSG_TYPE_EVENT type.
//
// @mhdr: Common QLINK message header.
// @event_id: Specifies specific event ID, one of &enum qlink_event_type.
// @macid: index of physical radio device the event was generated on or
// QLINK_MACID_RSVD if not applicable.
// @vifid: index of virtual wireless interface on specified @macid the event
// was generated on or QLINK_VIFID_RSVD if not applicable.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_event {
    pub mhdr: qlink_msg_header,
    pub event_id: __le16,
    pub macid: u8,
    pub vifid: u8,
    pub __packed: },
//
// struct qlink_event_sta_assoc - data for QLINK_EVENT_STA_ASSOCIATED event
//
// @sta_addr: Address of a STA for which new association event was generated
// @frame_control: control bits from 802.11 ASSOC_REQUEST header.
// @payload: IEs from association request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_event_sta_assoc {
    pub ehdr: qlink_event,
    pub sta_addr: [u8; ETH_ALEN],
    pub frame_control: __le16,
    pub ies: [u8; ],
    pub __packed: },
//
// struct qlink_event_sta_deauth - data for QLINK_EVENT_STA_DEAUTH event
//
// @sta_addr: Address of a deauthenticated STA.
// @reason: reason for deauthentication.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_event_sta_deauth {
    pub ehdr: qlink_event,
    pub sta_addr: [u8; ETH_ALEN],
    pub reason: __le16,
    pub __packed: },
//
// struct qlink_event_bss_join - data for QLINK_EVENT_BSS_JOIN event
//
// @chan: new operating channel definition
// @bssid: BSSID of a BSS which interface tried to joined.
// @status: status of joining attempt, see &enum ieee80211_statuscode.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_event_bss_join {
    pub ehdr: qlink_event,
    pub chan: qlink_chandef,
    pub bssid: [u8; ETH_ALEN],
    pub status: __le16,
    pub ies: [u8; ],
    pub __packed: },
//
// struct qlink_event_bss_leave - data for QLINK_EVENT_BSS_LEAVE event
//
// @reason: reason of disconnecting from BSS.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_event_bss_leave {
    pub ehdr: qlink_event,
    pub reason: __le16,
    pub rsvd: [u8; 2],
    pub __packed: },
//
// struct qlink_event_freq_change - data for QLINK_EVENT_FREQ_CHANGE event
//
// @chan: new operating channel definition
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_event_freq_change {
    pub ehdr: qlink_event,
    pub chan: qlink_chandef,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlink_rxmgmt_flags {
    QLINK_RXMGMT_FLAG_ANSWERED = 1 << 0,
}

//
// struct qlink_event_rxmgmt - data for QLINK_EVENT_MGMT_RECEIVED event
//
// @freq: Frequency on which the frame was received in MHz.
// @flags: bitmap of &enum qlink_rxmgmt_flags.
// @sig_dbm: signal strength in dBm.
// @frame_data: data of Rx'd frame itself.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_event_rxmgmt {
    pub ehdr: qlink_event,
    pub freq: __le32,
    pub flags: __le32,
    pub sig_dbm: i8,
    pub rsvd: [u8; 3],
    pub frame_data: [u8; ],
    pub __packed: },
//
// struct qlink_event_scan_result - data for QLINK_EVENT_SCAN_RESULTS event
//
// @tsf: TSF timestamp indicating when scan results were generated.
// @freq: Center frequency of the channel where BSS for which the scan result
// event was generated was discovered.
// @capab: capabilities field.
// @bintval: beacon interval announced by discovered BSS.
// @sig_dbm: signal strength in dBm.
// @bssid: BSSID announced by discovered BSS.
// @ssid_len: length of SSID announced by BSS.
// @ssid: SSID announced by discovered BSS.
// @payload: IEs that are announced by discovered BSS in its MGMt frames.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_event_scan_result {
    pub ehdr: qlink_event,
    pub tsf: __le64,
    pub freq: __le16,
    pub capab: __le16,
    pub bintval: __le16,
    pub sig_dbm: i8,
    pub ssid_len: u8,
    pub ssid: [u8; IEEE80211_MAX_SSID_LEN],
    pub bssid: [u8; ETH_ALEN],
    pub rsvd: [u8; 2],
    pub payload: [u8; ],
    pub __packed: },
//
// enum qlink_scan_complete_flags - indicates result of scan request.
//
// @QLINK_SCAN_NONE: Scan request was processed.
// @QLINK_SCAN_ABORTED: Scan was aborted.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlink_scan_complete_flags {
    QLINK_SCAN_NONE		= 0,
    QLINK_SCAN_ABORTED	= BIT(0),
}

//
// struct qlink_event_scan_complete - data for QLINK_EVENT_SCAN_COMPLETE event
//
// @flags: flags indicating the status of pending scan request,
// see &enum qlink_scan_complete_flags.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_event_scan_complete {
    pub ehdr: qlink_event,
    pub flags: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlink_radar_event {
    QLINK_RADAR_DETECTED,
    QLINK_RADAR_CAC_FINISHED,
    QLINK_RADAR_CAC_ABORTED,
    QLINK_RADAR_NOP_FINISHED,
    QLINK_RADAR_PRE_CAC_EXPIRED,
    QLINK_RADAR_CAC_STARTED,
}

//
// struct qlink_event_radar - data for QLINK_EVENT_RADAR event
//
// @chan: channel on which radar event happened.
// @event: radar event type, one of &enum qlink_radar_event.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_event_radar {
    pub ehdr: qlink_event,
    pub chan: qlink_chandef,
    pub event: u8,
    pub rsvd: [u8; 3],
    pub __packed: },
//
// struct qlink_event_external_auth - data for QLINK_EVENT_EXTERNAL_AUTH event
//
// @ssid: SSID announced by BSS
// @ssid_len: SSID length
// @bssid: BSSID of the BSS to connect to
// @akm_suite: AKM suite for external authentication
// @action: action type/trigger for external authentication
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_event_external_auth {
    pub ehdr: qlink_event,
    pub akm_suite: __le32,
    pub ssid: [u8; IEEE80211_MAX_SSID_LEN],
    pub bssid: [u8; ETH_ALEN],
    pub ssid_len: u8,
    pub action: u8,
    pub __packed: },
//
// struct qlink_event_mic_failure - data for QLINK_EVENT_MIC_FAILURE event
//
// @src: source MAC address of the frame
// @key_index: index of the key being reported
// @pairwise: whether the key is pairwise or group
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_event_mic_failure {
    pub ehdr: qlink_event,
    pub src: [u8; ETH_ALEN],
    pub key_index: u8,
    pub pairwise: u8,
    pub __packed: },
//
// struct qlink_event_update_owe - data for QLINK_EVENT_UPDATE_OWE event
//
// @peer: MAC addr of the peer device for which OWE processing needs to be done
// @ies: IEs from the peer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_event_update_owe {
    pub ehdr: qlink_event,
    pub peer: [u8; ETH_ALEN],
    pub rsvd: [u8; 2],
    pub ies: [u8; ],
    pub __packed: },
// QLINK TLVs (Type-Length Values) definitions
//
// enum qlink_tlv_id - list of TLVs that Qlink messages can carry
//
// @QTN_TLV_ID_BITMAP: a data representing a bitmap that is used together with
// other TLVs:
// &enum qlink_sta_info used to indicate which statistic carried in
// QTN_TLV_ID_STA_STATS is valid.
// &enum qlink_hw_capab listing wireless card capabilities.
// &enum qlink_driver_capab listing driver/host system capabilities.
// &enum qlink_chan_stat used to indicate which statistic carried in
// QTN_TLV_ID_CHANNEL_STATS is valid.
// @QTN_TLV_ID_STA_STATS: per-STA statistics as defined by
// &struct qlink_sta_stats. Valid values are marked as such in a bitmap
// carried by QTN_TLV_ID_BITMAP.
// @QTN_TLV_ID_IFTYPE_DATA: supported band data.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlink_tlv_id {
    QTN_TLV_ID_FRAG_THRESH		= 0x0201,
    QTN_TLV_ID_RTS_THRESH		= 0x0202,
    QTN_TLV_ID_SRETRY_LIMIT		= 0x0203,
    QTN_TLV_ID_LRETRY_LIMIT		= 0x0204,
    QTN_TLV_ID_REG_RULE		= 0x0207,
    QTN_TLV_ID_CHANNEL		= 0x020F,
    QTN_TLV_ID_CHANDEF		= 0x0210,
    QTN_TLV_ID_BITMAP		= 0x0211,
    QTN_TLV_ID_STA_STATS		= 0x0212,
    QTN_TLV_ID_COVERAGE_CLASS	= 0x0213,
    QTN_TLV_ID_IFACE_LIMIT		= 0x0214,
    QTN_TLV_ID_CHANNEL_STATS	= 0x0216,
    QTN_TLV_ID_KEY			= 0x0302,
    QTN_TLV_ID_SEQ			= 0x0303,
    QTN_TLV_ID_IE_SET		= 0x0305,
    QTN_TLV_ID_EXT_CAPABILITY_MASK	= 0x0306,
    QTN_TLV_ID_ACL_DATA		= 0x0307,
    QTN_TLV_ID_BUILD_NAME		= 0x0401,
    QTN_TLV_ID_BUILD_REV		= 0x0402,
    QTN_TLV_ID_BUILD_TYPE		= 0x0403,
    QTN_TLV_ID_BUILD_LABEL		= 0x0404,
    QTN_TLV_ID_HW_ID		= 0x0405,
    QTN_TLV_ID_CALIBRATION_VER	= 0x0406,
    QTN_TLV_ID_UBOOT_VER		= 0x0407,
    QTN_TLV_ID_RANDOM_MAC_ADDR	= 0x0408,
    QTN_TLV_ID_WOWLAN_CAPAB		= 0x0410,
    QTN_TLV_ID_WOWLAN_PATTERN	= 0x0411,
    QTN_TLV_ID_IFTYPE_DATA		= 0x0418,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_tlv_hdr {
// New members MUST be added within the struct_group() macro below.
    pub type: __le16,
    pub len: __le16,
    pub val: [u8; ],
    pub __packed: },
    pub __struct_group()"): "struct member likely outside of,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_iface_limit {
    pub max_num: __le16,
    pub type: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_iface_limit_record {
    pub max_interfaces: __le16,
    pub num_different_channels: u8,
    pub n_limits: u8,
    pub limits: [qlink_iface_limit; ],
    pub __packed: },
pub const QLINK_RSSI_OFFSET: c_int = 120;
//
// enum qlink_reg_rule_flags - regulatory rule flags
//
// See description of &enum nl80211_reg_rule_flags
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlink_reg_rule_flags {
    QLINK_RRF_NO_OFDM	= BIT(0),
    QLINK_RRF_NO_CCK	= BIT(1),
    QLINK_RRF_NO_INDOOR	= BIT(2),
    QLINK_RRF_NO_OUTDOOR	= BIT(3),
    QLINK_RRF_DFS		= BIT(4),
    QLINK_RRF_PTP_ONLY	= BIT(5),
    QLINK_RRF_PTMP_ONLY	= BIT(6),
    QLINK_RRF_NO_IR		= BIT(7),
    QLINK_RRF_AUTO_BW	= BIT(8),
    QLINK_RRF_IR_CONCURRENT	= BIT(9),
    QLINK_RRF_NO_HT40MINUS	= BIT(10),
    QLINK_RRF_NO_HT40PLUS	= BIT(11),
    QLINK_RRF_NO_80MHZ	= BIT(12),
    QLINK_RRF_NO_160MHZ	= BIT(13),
}

//
// struct qlink_tlv_reg_rule - data for QTN_TLV_ID_REG_RULE TLV
//
// Regulatory rule description.
//
// @start_freq_khz: start frequency of the range the rule is attributed to.
// @end_freq_khz: end frequency of the range the rule is attributed to.
// @max_bandwidth_khz: max bandwidth that channels in specified range can be
// configured to.
// @max_antenna_gain: max antenna gain that can be used in the specified
// frequency range, dBi.
// @max_eirp: maximum EIRP.
// @flags: regulatory rule flags in &enum qlink_reg_rule_flags.
// @dfs_cac_ms: DFS CAC period.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_tlv_reg_rule {
    pub hdr: qlink_tlv_hdr_fixed,
    pub start_freq_khz: __le32,
    pub end_freq_khz: __le32,
    pub max_bandwidth_khz: __le32,
    pub max_antenna_gain: __le32,
    pub max_eirp: __le32,
    pub flags: __le32,
    pub dfs_cac_ms: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlink_channel_flags {
    QLINK_CHAN_DISABLED		= BIT(0),
    QLINK_CHAN_NO_IR		= BIT(1),
    QLINK_CHAN_RADAR		= BIT(3),
    QLINK_CHAN_NO_HT40PLUS		= BIT(4),
    QLINK_CHAN_NO_HT40MINUS		= BIT(5),
    QLINK_CHAN_NO_OFDM		= BIT(6),
    QLINK_CHAN_NO_80MHZ		= BIT(7),
    QLINK_CHAN_NO_160MHZ		= BIT(8),
    QLINK_CHAN_INDOOR_ONLY		= BIT(9),
    QLINK_CHAN_IR_CONCURRENT	= BIT(10),
    QLINK_CHAN_NO_20MHZ		= BIT(11),
    QLINK_CHAN_NO_10MHZ		= BIT(12),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlink_dfs_state {
    QLINK_DFS_USABLE,
    QLINK_DFS_UNAVAILABLE,
    QLINK_DFS_AVAILABLE,
}

//
// struct qlink_tlv_channel - data for QTN_TLV_ID_CHANNEL TLV
//
// Channel settings.
//
// @channel: ieee80211 channel settings.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_tlv_channel {
    pub hdr: qlink_tlv_hdr_fixed,
    pub chan: qlink_channel,
    pub __packed: },
//
// struct qlink_tlv_chandef - data for QTN_TLV_ID_CHANDEF TLV
//
// Channel definition.
//
// @chan: channel definition data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_tlv_chandef {
    pub hdr: qlink_tlv_hdr_fixed,
    pub chdef: qlink_chandef,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlink_ie_set_type {
    QLINK_IE_SET_UNKNOWN,
    QLINK_IE_SET_ASSOC_REQ,
    QLINK_IE_SET_ASSOC_RESP,
    QLINK_IE_SET_PROBE_REQ,
    QLINK_IE_SET_SCAN,
    QLINK_IE_SET_BEACON_HEAD,
    QLINK_IE_SET_BEACON_TAIL,
    QLINK_IE_SET_BEACON_IES,
    QLINK_IE_SET_PROBE_RESP,
    QLINK_IE_SET_PROBE_RESP_IES,
}

//
// struct qlink_tlv_ie_set - data for QTN_TLV_ID_IE_SET
//
// @type: type of MGMT frame IEs belong to, one of &enum qlink_ie_set_type.
// @flags: for future use.
// @ie_data: IEs data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_tlv_ie_set {
    pub hdr: qlink_tlv_hdr_fixed,
    pub type: u8,
    pub flags: u8,
    pub rsvd: [u8; 2],
    pub ie_data: [u8; ],
    pub __packed: },
//
// struct qlink_tlv_ext_ie - extension IE
//
// @eid_ext: element ID extension, one of &enum ieee80211_eid_ext.
// @ie_data: IEs data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_tlv_ext_ie {
    pub hdr: qlink_tlv_hdr_fixed,
    pub eid_ext: u8,
    pub rsvd: [u8; 3],
    pub ie_data: [u8; ],
    pub __packed: },
pub const IEEE80211_HE_PPE_THRES_MAX_LEN: c_int = 25;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_sband_iftype_data {
    pub types_mask: __le16,
    pub he_cap_elem: ieee80211_he_cap_elem,
    pub he_mcs_nss_supp: ieee80211_he_mcs_nss_supp,
    pub ppe_thres: [u8; IEEE80211_HE_PPE_THRES_MAX_LEN],
    pub __packed: },
//
// struct qlink_tlv_iftype_data - data for QTN_TLV_ID_IFTYPE_DATA
//
// @n_iftype_data: number of entries in iftype_data.
// @iftype_data: interface type data entries.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_tlv_iftype_data {
    pub hdr: qlink_tlv_hdr_fixed,
    pub n_iftype_data: u8,
    pub rsvd: [u8; 3],
    pub iftype_data: [qlink_sband_iftype_data; ],
    pub __packed: },
//
// enum qlink_chan_stat - channel statistics bitmap
//
// Used to indicate which statistics values in &struct qlink_chan_stats
// are valid. Individual values are used to fill a bitmap carried in a
// payload of QTN_TLV_ID_BITMAP.
//
// @QLINK_CHAN_STAT_TIME_ON: time_on value is valid.
// @QLINK_CHAN_STAT_TIME_TX: time_tx value is valid.
// @QLINK_CHAN_STAT_TIME_RX: time_rx value is valid.
// @QLINK_CHAN_STAT_CCA_BUSY: cca_busy value is valid.
// @QLINK_CHAN_STAT_CCA_BUSY_EXT: cca_busy_ext value is valid.
// @QLINK_CHAN_STAT_TIME_SCAN: time_scan value is valid.
// @QLINK_CHAN_STAT_CHAN_NOISE: chan_noise value is valid.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlink_chan_stat {
    QLINK_CHAN_STAT_TIME_ON,
    QLINK_CHAN_STAT_TIME_TX,
    QLINK_CHAN_STAT_TIME_RX,
    QLINK_CHAN_STAT_CCA_BUSY,
    QLINK_CHAN_STAT_CCA_BUSY_EXT,
    QLINK_CHAN_STAT_TIME_SCAN,
    QLINK_CHAN_STAT_CHAN_NOISE,
    QLINK_CHAN_STAT_NUM,
}

//
// struct qlink_chan_stats - data for QTN_TLV_ID_CHANNEL_STATS
//
// Carries a per-channel statistics. Not all fields may be filled with
// valid values. Valid fields should be indicated as such using a bitmap of
// &enum qlink_chan_stat. Bitmap is carried separately in a payload of
// QTN_TLV_ID_BITMAP.
//
// @time_on: amount of time radio operated on that channel.
// @time_tx: amount of time radio spent transmitting on the channel.
// @time_rx: amount of time radio spent receiving on the channel.
// @cca_busy: amount of time the primary channel was busy.
// @cca_busy_ext: amount of time the secondary channel was busy.
// @time_scan: amount of radio spent scanning on the channel.
// @chan_noise: channel noise.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_chan_stats {
    pub time_on: __le64,
    pub time_tx: __le64,
    pub time_rx: __le64,
    pub cca_busy: __le64,
    pub cca_busy_ext: __le64,
    pub time_scan: __le64,
    pub chan_noise: i8,
    pub rsvd: [u8; 3],
    pub __packed: },
//
// enum qlink_sta_info - station information bitmap
//
// Used to indicate which statistics values in &struct qlink_sta_stats
// are valid. Individual values are used to fill a bitmap carried in a
// payload of QTN_TLV_ID_BITMAP.
//
// @QLINK_STA_INFO_CONNECTED_TIME: connected_time value is valid.
// @QLINK_STA_INFO_INACTIVE_TIME: inactive_time value is valid.
// @QLINK_STA_INFO_RX_BYTES: lower 32 bits of rx_bytes value are valid.
// @QLINK_STA_INFO_TX_BYTES: lower 32 bits of tx_bytes value are valid.
// @QLINK_STA_INFO_RX_BYTES64: rx_bytes value is valid.
// @QLINK_STA_INFO_TX_BYTES64: tx_bytes value is valid.
// @QLINK_STA_INFO_RX_DROP_MISC: rx_dropped_misc value is valid.
// @QLINK_STA_INFO_BEACON_RX: rx_beacon value is valid.
// @QLINK_STA_INFO_SIGNAL: signal value is valid.
// @QLINK_STA_INFO_SIGNAL_AVG: signal_avg value is valid.
// @QLINK_STA_INFO_RX_BITRATE: rxrate value is valid.
// @QLINK_STA_INFO_TX_BITRATE: txrate value is valid.
// @QLINK_STA_INFO_RX_PACKETS: rx_packets value is valid.
// @QLINK_STA_INFO_TX_PACKETS: tx_packets value is valid.
// @QLINK_STA_INFO_TX_RETRIES: tx_retries value is valid.
// @QLINK_STA_INFO_TX_FAILED: tx_failed value is valid.
// @QLINK_STA_INFO_STA_FLAGS: sta_flags value is valid.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlink_sta_info {
    QLINK_STA_INFO_CONNECTED_TIME,
    QLINK_STA_INFO_INACTIVE_TIME,
    QLINK_STA_INFO_RX_BYTES,
    QLINK_STA_INFO_TX_BYTES,
    QLINK_STA_INFO_RX_BYTES64,
    QLINK_STA_INFO_TX_BYTES64,
    QLINK_STA_INFO_RX_DROP_MISC,
    QLINK_STA_INFO_BEACON_RX,
    QLINK_STA_INFO_SIGNAL,
    QLINK_STA_INFO_SIGNAL_AVG,
    QLINK_STA_INFO_RX_BITRATE,
    QLINK_STA_INFO_TX_BITRATE,
    QLINK_STA_INFO_RX_PACKETS,
    QLINK_STA_INFO_TX_PACKETS,
    QLINK_STA_INFO_TX_RETRIES,
    QLINK_STA_INFO_TX_FAILED,
    QLINK_STA_INFO_STA_FLAGS,
    QLINK_STA_INFO_NUM,
}

//
// struct qlink_sta_info_rate - STA rate statistics
//
// @rate: data rate in Mbps.
// @flags: bitmap of &enum qlink_sta_info_rate_flags.
// @mcs: 802.11-defined MCS index.
// nss: Number of Spatial Streams.
// @bw: bandwidth, one of &enum qlink_channel_width.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_sta_info_rate {
    pub rate: __le16,
    pub flags: u8,
    pub mcs: u8,
    pub nss: u8,
    pub bw: u8,
    pub __packed: },
//
// struct qlink_sta_stats - data for QTN_TLV_ID_STA_STATS
//
// Carries statistics of a STA. Not all fields may be filled with
// valid values. Valid fields should be indicated as such using a bitmap of
// &enum qlink_sta_info. Bitmap is carried separately in a payload of
// QTN_TLV_ID_BITMAP.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_sta_stats {
    pub rx_bytes: __le64,
    pub tx_bytes: __le64,
    pub rx_beacon: __le64,
    pub rx_duration: __le64,
    pub t_offset: __le64,
    pub connected_time: __le32,
    pub inactive_time: __le32,
    pub rx_packets: __le32,
    pub tx_packets: __le32,
    pub tx_retries: __le32,
    pub tx_failed: __le32,
    pub rx_dropped_misc: __le32,
    pub beacon_loss_count: __le32,
    pub expected_throughput: __le32,
    pub sta_flags: qlink_sta_info_state,
    pub txrate: qlink_sta_info_rate,
    pub rxrate: qlink_sta_info_rate,
    pub llid: __le16,
    pub plid: __le16,
    pub local_pm: u8,
    pub peer_pm: u8,
    pub nonpeer_pm: u8,
    pub rx_beacon_signal_avg: u8,
    pub plink_state: u8,
    pub signal: u8,
    pub signal_avg: u8,
    pub rsvd: [u8; 1],
}

//
// struct qlink_random_mac_addr - data for QTN_TLV_ID_RANDOM_MAC_ADDR TLV
//
// Specifies MAC address mask/value for generation random MAC address
// during scan.
//
// @mac_addr: MAC address used with randomisation
// @mac_addr_mask: MAC address mask used with randomisation, bits that
// are 0 in the mask should be randomised, bits that are 1 should
// be taken from the @mac_addr
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_random_mac_addr {
    pub mac_addr: [u8; ETH_ALEN],
    pub mac_addr_mask: [u8; ETH_ALEN],
    pub __packed: },
//
// struct qlink_wowlan_capab_data - data for QTN_TLV_ID_WOWLAN_CAPAB TLV
//
// WoWLAN capabilities supported by cards.
//
// @version: version of WoWLAN data structure, to ensure backward
// compatibility for firmwares with limited WoWLAN support
// @len: Total length of WoWLAN data
// @data: supported WoWLAN features
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_wowlan_capab_data {
    pub version: __le16,
    pub len: __le16,
    pub data: [u8; ],
    pub __packed: },
//
// struct qlink_wowlan_support - supported WoWLAN capabilities
//
// @n_patterns: number of supported wakeup patterns
// @pattern_max_len: maximum length of each pattern
// @pattern_min_len: minimum length of each pattern
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink_wowlan_support {
    pub n_patterns: __le32,
    pub pattern_max_len: __le32,
    pub pattern_min_len: __le32,
    pub __packed: },
