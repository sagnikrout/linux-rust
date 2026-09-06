//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/morsemicro/mm81x/command_defs.h
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
// Copyright (c) 2017-2026 Morse Micro
//

pub const HOST_CMD_SEMVER_MAJOR: c_int = 56;
pub const HOST_CMD_SEMVER_MINOR: c_int = 17;
pub const HOST_CMD_SEMVER_PATCH: c_int = 0;

pub const HOST_CMD_SSID_MAX_LEN: c_int = 32;
pub const HOST_CMD_MAC_ADDR_LEN: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum host_cmd_id {
    HOST_CMD_ID_SET_CHANNEL = 0x0001,
    HOST_CMD_ID_GET_CHANNEL = 0x001D,
    HOST_CMD_ID_GET_CHANNEL_FULL = 0x0013,
    HOST_CMD_ID_GET_CHANNEL_DTIM = 0x001C,
    HOST_CMD_ID_GET_VERSION = 0x0002,
    HOST_CMD_ID_SET_TXPOWER = 0x0003,
    HOST_CMD_ID_GET_MAX_TXPOWER = 0x0024,
    HOST_CMD_ID_ADD_INTERFACE = 0x0004,
    HOST_CMD_ID_REMOVE_INTERFACE = 0x0005,
    HOST_CMD_ID_BSS_CONFIG = 0x0006,
    HOST_CMD_ID_SCAN_CONFIG = 0x0010,
    HOST_CMD_ID_SET_QOS_PARAMS = 0x0011,
    HOST_CMD_ID_GET_QOS_PARAMS = 0x0012,
    HOST_CMD_ID_SET_STA_STATE = 0x0014,
    HOST_CMD_ID_SET_BSS_COLOR = 0x0015,
    HOST_CMD_ID_CONFIG_PS = 0x0016,
    HOST_CMD_ID_HEALTH_CHECK = 0x0019,
    HOST_CMD_ID_CTS_SELF_PS = 0x001A,
    HOST_CMD_ID_DTIM_CHANNEL_ENABLE = 0x001B,
    HOST_CMD_ID_ARP_OFFLOAD = 0x0020,
    HOST_CMD_ID_SET_LONG_SLEEP_CONFIG = 0x0021,
    HOST_CMD_ID_SET_DUTY_CYCLE = 0x0022,
    HOST_CMD_ID_GET_DUTY_CYCLE = 0x0023,
    HOST_CMD_ID_GET_CAPABILITIES = 0x0025,
    HOST_CMD_ID_TWT_AGREEMENT_INSTALL = 0x0026,
    HOST_CMD_ID_TWT_AGREEMENT_VALIDATE = 0x0036,
    HOST_CMD_ID_TWT_AGREEMENT_REMOVE = 0x0027,
    HOST_CMD_ID_GET_TSF = 0x0028,
    HOST_CMD_ID_MAC_ADDR = 0x0029,
    HOST_CMD_ID_MPSW_CONFIG = 0x0030,
    HOST_CMD_ID_INSTALL_KEY = 0x000A,
    HOST_CMD_ID_DISABLE_KEY = 0x000B,
    HOST_CMD_ID_DHCP_OFFLOAD = 0x0032,
    HOST_CMD_ID_SET_KEEP_ALIVE_OFFLOAD = 0x0033,
    HOST_CMD_ID_UPDATE_OUI_FILTER = 0x0034,
    HOST_CMD_ID_IBSS_CONFIG = 0x0035,
    HOST_CMD_ID_OCS = 0x0038,
    HOST_CMD_ID_MESH_CONFIG = 0x0039,
    HOST_CMD_ID_SET_OFFSET_TSF = 0x003A,
    HOST_CMD_ID_GET_CHANNEL_USAGE = 0x003B,
    HOST_CMD_ID_MCAST_FILTER = 0x003C,
    HOST_CMD_ID_BSS_BEACON_CONFIG = 0x003D,
    HOST_CMD_ID_UAPSD_CONFIG = 0x0040,
    HOST_CMD_ID_PAGE_SLICING_CONFIG = 0x0043,
    HOST_CMD_ID_HW_SCAN = 0x0044,
    HOST_CMD_ID_SET_WHITELIST = 0x0045,
    HOST_CMD_ID_ARP_PERIODIC_REFRESH = 0x0046,
    HOST_CMD_ID_SET_TCP_KEEPALIVE = 0x0047,
    HOST_CMD_ID_FORCE_POWER_MODE = 0x0048,
    HOST_CMD_ID_LI_SLEEP = 0x0049,
    HOST_CMD_ID_GET_DISABLED_CHANNELS = 0x004A,
    HOST_CMD_ID_SET_CQM_RSSI = 0x004F,
    HOST_CMD_ID_GET_APF_CAPABILITIES = 0x0050,
    HOST_CMD_ID_READ_WRITE_APF = 0x0051,
    HOST_CMD_ID_BSSID_SET = 0x0052,
    HOST_CMD_ID_BEACON_OFFLOAD = 0x0053,
    HOST_CMD_ID_PROBE_RESPONSE_OFFLOAD = 0x0054,
    HOST_CMD_ID_HOST_STATS_LOG = 0x2007,
    HOST_CMD_ID_HOST_STATS_RESET = 0x2008,
    HOST_CMD_ID_MAC_STATS_LOG = 0x200C,
    HOST_CMD_ID_MAC_STATS_RESET = 0x200D,
    HOST_CMD_ID_UPHY_STATS_LOG = 0x200E,
    HOST_CMD_ID_UPHY_STATS_RESET = 0x200F,
    HOST_CMD_ID_SET_STA_TYPE = 0xA000,
    HOST_CMD_ID_SET_ENC_MODE = 0xA001,
    HOST_CMD_ID_TEST_BA = 0xA002,
    HOST_CMD_ID_SET_LISTEN_INTERVAL = 0xA003,
    HOST_CMD_ID_SET_AMPDU = 0xA004,
    HOST_CMD_ID_COREDUMP = 0xA006,
    HOST_CMD_ID_SET_S1G_OP_CLASS = 0xA007,
    HOST_CMD_ID_SEND_WAKE_ACTION_FRAME = 0xA008,
    HOST_CMD_ID_VENDOR_IE_CONFIG = 0xA009,
    HOST_CMD_ID_SET_TWT_CONF = 0xA010,
    HOST_CMD_ID_GET_AVAILABLE_CHANNELS = 0xA011,
    HOST_CMD_ID_SET_ECSA_S1G_INFO = 0xA012,
    HOST_CMD_ID_GET_HW_VERSION = 0xA013,
    HOST_CMD_ID_CAC = 0xA014,
    HOST_CMD_ID_DRIVER_SET_DUTY_CYCLE = 0xA015,
    HOST_CMD_ID_OCS_DRIVER = 0xA017,
    HOST_CMD_ID_MBSSID = 0xA016,
    HOST_CMD_ID_SET_MESH_CONFIG = 0xA018,
    HOST_CMD_ID_SET_MCBA_CONF = 0xA019,
    HOST_CMD_ID_DYNAMIC_PEERING_CONFIG = 0xA020,
    HOST_CMD_ID_CONFIG_RAW = 0xA021,
    HOST_CMD_ID_CONFIG_BSS_STATS = 0xA022,
    HOST_CMD_ID_GET_RSSI = 0x1002,
    HOST_CMD_ID_SET_IFS = 0x1003,
    HOST_CMD_ID_SET_FEM_SETTINGS = 0x1005,
    HOST_CMD_ID_SET_TXOP = 0x1008,
    HOST_CMD_ID_SET_CONTROL_RESPONSE = 0x1009,
    HOST_CMD_ID_SET_PERIODIC_CAL = 0x100A,
    HOST_CMD_ID_SET_BCN_RSSI_THRESHOLD = 0x100B,
    HOST_CMD_ID_SET_TX_PKT_LIFETIME_USECS = 0x100C,
    HOST_CMD_ID_SET_PHYSM_WATCHDOG = 0x100D,
    HOST_CMD_ID_TX_POLAR = 0x100E,
    HOST_CMD_ID_EVT_STA_STATE = 0x4001,
    HOST_CMD_ID_EVT_BEACON_LOSS = 0x4002,
    HOST_CMD_ID_EVT_SIG_FIELD_ERROR = 0x4003,
    HOST_CMD_ID_EVT_UMAC_TRAFFIC_CONTROL = 0x4004,
    HOST_CMD_ID_EVT_DHCP_LEASE_UPDATE = 0x4005,
    HOST_CMD_ID_EVT_OCS_DONE = 0x4006,
    HOST_CMD_ID_EVT_HW_SCAN_DONE = 0x4011,
    HOST_CMD_ID_EVT_CHANNEL_USAGE = 0x4012,
    HOST_CMD_ID_EVT_CONNECTION_LOSS = 0x4013,
    HOST_CMD_ID_EVT_SCHED_SCAN_RESULTS = 0x4014,
    HOST_CMD_ID_EVT_CQM_RSSI_NOTIFY = 0x4015,
    HOST_CMD_ID_EVT_SCAN_DONE = 0x4007,
    HOST_CMD_ID_EVT_SCAN_RESULT = 0x4008,
    HOST_CMD_ID_EVT_CONNECTED = 0x4009,
    HOST_CMD_ID_EVT_DISCONNECTED = 0x4010,
    HOST_CMD_ID_EVT_BEACON_FILTER_MATCH = 0x4016,
    HOST_CMD_ID_SET_CAPABILITIES = 0x8118,
    HOST_CMD_ID_SET_TRANSMISSION_RATE = 0x8009,
    HOST_CMD_ID_FORCE_ASSERT = 0x800E,
    HOST_CMD_ID_GET_SET_GENERIC_PARAM = 0x003E,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_mac_addr {
    pub octet: [u8; HOST_CMD_MAC_ADDR_LEN],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum host_cmd_ocs_subcmd {
    HOST_CMD_OCS_SUBCMD_CONFIG = 1,
    HOST_CMD_OCS_SUBCMD_STATUS = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum host_cmd_headless_cfg_option {
    HOST_CMD_HEADLESS_CFG_OPTION_KEEP_IFACES = BIT(0),
    HOST_CMD_HEADLESS_CFG_OPTION_BUFFER_RX = BIT(1),
    HOST_CMD_HEADLESS_CFG_OPTION_NOTIFY_ON_ANY_RX = BIT(2),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_header {
    pub flags: __le16,
    pub message_id: __le16,
    pub len: __le16,
    pub host_id: __le16,
    pub vif_id: __le16,
    pub pad: __le16,
}

pub const HOST_CMD_CHANNEL_BW_NOT_SET: c_uint = 0xFF;
pub const HOST_CMD_CHANNEL_IDX_NOT_SET: c_uint = 0xFF;
pub const HOST_CMD_CHANNEL_FREQ_NOT_SET: c_uint = 0xFFFFFFFF;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum host_cmd_dot11_proto_mode {
    HOST_CMD_DOT11_PROTO_MODE_AH = 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_set_channel {
    pub hdr: host_cmd_header,
    pub op_chan_freq_hz: __le32,
    pub op_bw_mhz: u8,
    pub pri_bw_mhz: u8,
    pub pri_1mhz_chan_idx: u8,
    pub dot11_mode: u8,
    pub __deprecated_reg_tx_power_set: u8,
    pub is_off_channel: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_set_channel {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub power_qdbm: __sle32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_get_channel {
    pub hdr: host_cmd_header,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_get_channel {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub op_chan_freq_hz: __le32,
    pub op_chan_bw_mhz: u8,
    pub pri_chan_bw_mhz: u8,
    pub pri_1mhz_chan_idx: u8,
    pub __packed: },
pub const HOST_CMD_MAX_VERSION_LEN: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_get_version {
    pub hdr: host_cmd_header,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_get_version {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub length: __sle32,
    pub version: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_set_txpower {
    pub hdr: host_cmd_header,
    pub power_qdbm: __sle32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_set_txpower {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub power_qdbm: __sle32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_get_max_txpower {
    pub hdr: host_cmd_header,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_get_max_txpower {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub power_qdbm: __sle32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum host_cmd_interface_type {
    HOST_CMD_INTERFACE_TYPE_INVALID = 0,
    HOST_CMD_INTERFACE_TYPE_STA = 1,
    HOST_CMD_INTERFACE_TYPE_AP = 2,
    HOST_CMD_INTERFACE_TYPE_MON = 3,
    HOST_CMD_INTERFACE_TYPE_ADHOC = 4,
    HOST_CMD_INTERFACE_TYPE_MESH = 5,
    HOST_CMD_INTERFACE_TYPE_LAST = HOST_CMD_INTERFACE_TYPE_MESH,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_add_interface {
    pub hdr: host_cmd_header,
    pub addr: host_cmd_mac_addr,
    pub interface_type: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_add_interface {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_remove_interface {
    pub hdr: host_cmd_header,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_remove_interface {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_bss_config {
    pub hdr: host_cmd_header,
    pub beacon_interval_tu: __le16,
    pub dtim_period: __le16,
    pub __padding: [u8; 2],
    pub cssid: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_bss_config {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_scan_config {
    pub hdr: host_cmd_header,
    pub enabled: u8,
    pub is_survey: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_scan_config {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_set_qos_params {
    pub hdr: host_cmd_header,
    pub uapsd: u8,
    pub queue_idx: u8,
    pub aifs_slot_count: u8,
    pub contention_window_min: __le16,
    pub contention_window_max: __le16,
    pub max_txop_usec: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_set_qos_params {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_get_qos_params {
    pub hdr: host_cmd_header,
    pub queue_idx: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_get_qos_params {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub aifs_slot_count: u8,
    pub contention_window_min: __le16,
    pub contention_window_max: __le16,
    pub max_txop_usec: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_set_sta_state {
    pub hdr: host_cmd_header,
    pub sta_addr: [u8; HOST_CMD_MAC_ADDR_LEN],
    pub aid: __le16,
    pub state: __le16,
    pub uapsd_queues: u8,
    pub flags: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_set_sta_state {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_set_bss_color {
    pub hdr: host_cmd_header,
    pub bss_color: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_set_bss_color {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_config_ps {
    pub hdr: host_cmd_header,
    pub enabled: u8,
    pub dynamic_ps_offload: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_config_ps {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_health_check {
    pub hdr: host_cmd_header,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_health_check {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_cts_self_ps {
    pub hdr: host_cmd_header,
    pub enable: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_cts_self_ps {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_dtim_channel_enable {
    pub hdr: host_cmd_header,
    pub enable: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_dtim_channel_enable {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub __packed: },
pub const HOST_CMD_ARP_OFFLOAD_MAX_IP_ADDRESSES: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_arp_offload {
    pub hdr: host_cmd_header,
    pub ip_table: [__be32; HOST_CMD_ARP_OFFLOAD_MAX_IP_ADDRESSES],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_arp_offload {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_set_long_sleep_config {
    pub hdr: host_cmd_header,
    pub enabled: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_set_long_sleep_config {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum host_cmd_duty_cycle_mode {
    HOST_CMD_DUTY_CYCLE_MODE_SPREAD = 0,
    HOST_CMD_DUTY_CYCLE_MODE_BURST = 1,
    HOST_CMD_DUTY_CYCLE_MODE_LAST = HOST_CMD_DUTY_CYCLE_MODE_BURST,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_duty_cycle_configuration {
    pub omit_control_responses: u8,
    pub duty_cycle: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_duty_cycle_set_configuration_ext {
    pub burst_record_unit_us: __le32,
    pub mode: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_duty_cycle_configuration_ext {
    pub airtime_remaining_us: __le32,
    pub burst_window_duration_us: __le32,
    pub set: host_cmd_duty_cycle_set_configuration_ext,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_set_duty_cycle {
    pub hdr: host_cmd_header,
    pub config: host_cmd_duty_cycle_configuration,
    pub set_cfgs: u8,
    pub config_ext: host_cmd_duty_cycle_set_configuration_ext,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_get_duty_cycle {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub config: host_cmd_duty_cycle_configuration,
    pub config_ext: host_cmd_duty_cycle_configuration_ext,
    pub __packed: },

pub const HOST_CMD_S1G_CAPABILITY_FLAGS_WIDTH: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_mm_capabilities {
    pub flags: [__le32; HOST_CMD_S1G_CAPABILITY_FLAGS_WIDTH],
    pub ampdu_mss: u8,
    pub beamformee_sts_capability: u8,
    pub number_sounding_dimensions: u8,
    pub maximum_ampdu_length_exponent: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_get_capabilities {
    pub hdr: host_cmd_header,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_get_capabilities {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub capabilities: host_cmd_mm_capabilities,
    pub morse_mmss_offset: u8,
    pub __packed: },
pub const HOST_CMD_DOT11_TWT_AGREEMENT_MAX_LEN: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_twt_agreement_install {
    pub hdr: host_cmd_header,
    pub flow_id: u8,
    pub agreement_len: u8,
    pub agreement: [u8; HOST_CMD_DOT11_TWT_AGREEMENT_MAX_LEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_twt_agreement_install {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_twt_agreement_validate {
    pub hdr: host_cmd_header,
    pub flow_id: u8,
    pub agreement_len: u8,
    pub agreement: [u8; HOST_CMD_DOT11_TWT_AGREEMENT_MAX_LEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_twt_agreement_validate {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_twt_agreement_remove {
    pub hdr: host_cmd_header,
    pub flow_id: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_get_tsf {
    pub hdr: host_cmd_header,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_get_tsf {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub now_tsf: __le64,
    pub now_chip_ts: __le64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_mac_addr {
    pub hdr: host_cmd_header,
    pub write: u8,
    pub octet: [u8; HOST_CMD_MAC_ADDR_LEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_mac_addr {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub octet: [u8; HOST_CMD_MAC_ADDR_LEN],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_mpsw_configuration {
    pub airtime_max_us: __le32,
    pub airtime_min_us: __le32,
    pub packet_space_window_length_us: __le32,
    pub enable: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_mpsw_config {
    pub hdr: host_cmd_header,
    pub config: host_cmd_mpsw_configuration,
    pub set_cfgs: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_mpsw_config {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub config: host_cmd_mpsw_configuration,
    pub __packed: },
pub const HOST_CMD_MAX_KEY_LEN: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum host_cmd_key_cipher {
    HOST_CMD_KEY_CIPHER_INVALID = 0,
    HOST_CMD_KEY_CIPHER_AES_CCM = 1,
    HOST_CMD_KEY_CIPHER_AES_GCM = 2,
    HOST_CMD_KEY_CIPHER_AES_CMAC = 3,
    HOST_CMD_KEY_CIPHER_AES_GMAC = 4,
    HOST_CMD_KEY_CIPHER_LAST = HOST_CMD_KEY_CIPHER_AES_GMAC,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum host_cmd_aes_key_len {
    HOST_CMD_AES_KEY_LEN_INVALID = 0,
    HOST_CMD_AES_KEY_LEN_LENGTH_128 = 1,
    HOST_CMD_AES_KEY_LEN_LENGTH_256 = 2,
    HOST_CMD_AES_KEY_LEN_LENGTH_LAST = HOST_CMD_AES_KEY_LEN_LENGTH_256,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum host_cmd_temporal_key_type {
    HOST_CMD_TEMPORAL_KEY_TYPE_INVALID = 0,
    HOST_CMD_TEMPORAL_KEY_TYPE_GTK = 1,
    HOST_CMD_TEMPORAL_KEY_TYPE_PTK = 2,
    HOST_CMD_TEMPORAL_KEY_TYPE_IGTK = 3,
    HOST_CMD_TEMPORAL_KEY_TYPE_LAST = HOST_CMD_TEMPORAL_KEY_TYPE_IGTK,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_install_key {
    pub hdr: host_cmd_header,
    pub pn: __le64,
    pub aid: __le32,
    pub key_idx: u8,
    pub cipher: u8,
    pub key_length: u8,
    pub key_type: u8,
    pub __padding: [u8; 2],
    pub key: [u8; HOST_CMD_MAX_KEY_LEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_install_key {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub key_idx: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_disable_key {
    pub hdr: host_cmd_header,
    pub key_type: __le32,
    pub aid: __le32,
    pub key_idx: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_disable_key {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum host_cmd_dhcp_opcode {
    HOST_CMD_DHCP_OPCODE_ENABLE = 0,
    HOST_CMD_DHCP_OPCODE_DO_DISCOVERY = 1,
    HOST_CMD_DHCP_OPCODE_GET_LEASE = 2,
    HOST_CMD_DHCP_OPCODE_CLEAR_LEASE = 3,
    HOST_CMD_DHCP_OPCODE_RENEW_LEASE = 4,
    HOST_CMD_DHCP_OPCODE_REBIND_LEASE = 5,
    HOST_CMD_DHCP_OPCODE_SEND_LEASE_UPDATE = 6,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum host_cmd_dhcp_retcode {
    HOST_CMD_DHCP_RETCODE_SUCCESS = 0,
    HOST_CMD_DHCP_RETCODE_NOT_ENABLED = 1,
    HOST_CMD_DHCP_RETCODE_ALREADY_ENABLED = 2,
    HOST_CMD_DHCP_RETCODE_NO_LEASE = 3,
    HOST_CMD_DHCP_RETCODE_HAVE_LEASE = 4,
    HOST_CMD_DHCP_RETCODE_BUSY = 5,
    HOST_CMD_DHCP_RETCODE_BAD_VIF = 6,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_dhcp_offload {
    pub hdr: host_cmd_header,
    pub opcode: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_dhcp_offload {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub retcode: __le32,
    pub my_ip: __le32,
    pub netmask: __le32,
    pub router: __le32,
    pub dns: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_set_keep_alive_offload {
    pub hdr: host_cmd_header,
    pub bss_max_idle_period: __le16,
    pub interpret_as_11ah: u8,
    pub __packed: },
pub const HOST_CMD_MAX_OUI_FILTERS: c_int = 5;
pub const HOST_CMD_OUI_SIZE: c_int = 3;
pub const HOST_CMD_MAX_OUI_FILTER_ARRAY_SIZE: c_int = 15;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_update_oui_filter {
    pub hdr: host_cmd_header,
    pub n_ouis: u8,
    pub ouis: [u8; HOST_CMD_MAX_OUI_FILTERS][HOST_CMD_OUI_SIZE],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum host_cmd_ibss_config_opcode {
    HOST_CMD_IBSS_CONFIG_OPCODE_CREATE = 0,
    HOST_CMD_IBSS_CONFIG_OPCODE_JOIN = 1,
    HOST_CMD_IBSS_CONFIG_OPCODE_STOP = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_ibss_config {
    pub hdr: host_cmd_header,
    pub ibss_bssid: [u8; HOST_CMD_MAC_ADDR_LEN],
    pub ibss_cfg_opcode: u8,
    pub ibss_probe_filtering: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum host_cmd_ocs_type {
    HOST_CMD_OCS_TYPE_QNULL = 0,
    HOST_CMD_OCS_TYPE_RAW = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ocs_config_req {
    pub op_channel_freq_hz: __le32,
    pub op_channel_bw_mhz: u8,
    pub pri_channel_bw_mhz: u8,
    pub pri_1mhz_channel_index: u8,
    pub aid: __le16,
    pub type: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ocs_status_resp {
    pub running: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_ocs {
    pub hdr: host_cmd_header,
    pub subcmd: __le32,
    pub opaque: [u8; 0],
    pub config: host_cmd_ocs_config_req,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_ocs {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub subcmd: __le32,
    pub opaque: [u8; 0],
    pub ocs_status: host_cmd_ocs_status_resp,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum host_cmd_mesh_config_opcode {
    HOST_CMD_MESH_CONFIG_OPCODE_START = 0,
    HOST_CMD_MESH_CONFIG_OPCODE_STOP = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_mesh_config {
    pub hdr: host_cmd_header,
    pub mesh_cfg_opcode: u8,
    pub enable_beaconing: u8,
    pub mbca_config: u8,
    pub min_beacon_gap_ms: u8,
    pub mbss_start_scan_duration_ms: __le16,
    pub tbtt_adj_timer_interval_ms: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_set_offset_tsf {
    pub hdr: host_cmd_header,
    pub offset_tsf: __sle64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_get_channel_usage {
    pub hdr: host_cmd_header,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_get_channel_usage {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub time_listen: __le64,
    pub busy_time: __le64,
    pub freq_hz: __le32,
    pub noise: i8,
    pub bw_mhz: u8,
    pub __packed: },
pub const HOST_CMD_MAX_MCAST_FILTERS: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_mcast_filter {
    pub hdr: host_cmd_header,
    pub count: u8,
    pub hw_addr: [__le32; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_bss_beacon_config {
    pub hdr: host_cmd_header,
    pub enable: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_bss_beacon_config {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub interface_id: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_uapsd_config {
    pub hdr: host_cmd_header,
    pub auto_trigger_enabled: u8,
    pub auto_trigger_timeout: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_uapsd_config {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub auto_trigger_enabled: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_page_slicing_config {
    pub hdr: host_cmd_header,
    pub enable: u8,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum host_cmd_hw_scan_tlv_tag {
    HOST_CMD_HW_SCAN_TLV_TAG_PAD = 0,
    HOST_CMD_HW_SCAN_TLV_TAG_PROBE_REQ = 1,
    HOST_CMD_HW_SCAN_TLV_TAG_CHAN_LIST = 2,
    HOST_CMD_HW_SCAN_TLV_TAG_POWER_LIST = 3,
    HOST_CMD_HW_SCAN_TLV_TAG_DWELL_ON_HOME = 4,
    HOST_CMD_HW_SCAN_TLV_TAG_SCHED = 5,
    HOST_CMD_HW_SCAN_TLV_TAG_FILTER = 6,
    HOST_CMD_HW_SCAN_TLV_TAG_SCHED_PARAMS = 7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_hw_scan_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub value: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_hw_scan {
    pub hdr: host_cmd_header,
    pub flags: __le32,
    pub dwell_time_ms: __le32,
    pub variable: [u8; ],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_set_whitelist {
    pub hdr: host_cmd_header,
    pub flags: u8,
    pub ip_protocol: u8,
    pub llc_protocol: __be16,
    pub src_ip: __be32,
    pub dest_ip: __be32,
    pub netmask: __be32,
    pub src_port: __be16,
    pub dest_port: __be16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_arp_periodic_params {
    pub refresh_period_s: __le32,
    pub destination_ip: __le32,
    pub send_as_garp: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_arp_periodic_refresh {
    pub hdr: host_cmd_header,
    pub config: host_cmd_arp_periodic_params,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_set_tcp_keepalive {
    pub hdr: host_cmd_header,
    pub enabled: u8,
    pub retry_count: u8,
    pub retry_interval_s: u8,
    pub set_cfgs: u8,
    pub src_ip: __be32,
    pub dest_ip: __be32,
    pub src_port: __be16,
    pub dest_port: __be16,
    pub period_s: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum host_cmd_power_mode {
    HOST_CMD_POWER_MODE_SNOOZE = 0,
    HOST_CMD_POWER_MODE_DEEP_SLEEP = 1,
    HOST_CMD_POWER_MODE_HIBERNATE = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_force_power_mode {
    pub hdr: host_cmd_header,
    pub mode: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_li_sleep {
    pub hdr: host_cmd_header,
    pub listen_interval: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_disabled_channel_entry {
    pub freq_100khz: __le16,
    pub bw_mhz: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_get_disabled_channels {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub n_channels: __le32,
    pub channels: [host_cmd_disabled_channel_entry; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_set_cqm_rssi {
    pub hdr: host_cmd_header,
    pub threshold: __sle32,
    pub hysteresis: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_get_apf_capabilities {
    pub hdr: host_cmd_header,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_get_apf_capabilities {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub max_length: __le32,
    pub version: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_read_write_apf {
    pub hdr: host_cmd_header,
    pub offset: __le32,
    pub program_length: __le16,
    pub write: u8,
    pub program: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_read_write_apf {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub program_length: __le16,
    pub program: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_bssid_set {
    pub hdr: host_cmd_header,
    pub bssid: host_cmd_mac_addr,
    pub __packed: },

pub const HOST_CMD_BEACON_OFFLOAD_CSSID_LEN: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum host_cmd_beacon_offload_tlv_tag {
    HOST_CMD_BEACON_OFFLOAD_TLV_TAG_DTIM_CNT = 0,
    HOST_CMD_BEACON_OFFLOAD_TLV_TAG_FRAME_CTRL = 1,
    HOST_CMD_BEACON_OFFLOAD_TLV_TAG_CHANGE_SEQ = 2,
    HOST_CMD_BEACON_OFFLOAD_TLV_TAG_CSSID = 3,
    HOST_CMD_BEACON_OFFLOAD_TLV_TAG_IES = 4,
    HOST_CMD_BEACON_OFFLOAD_TLV_TAG_TX_INFO = 5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_beacon_offload_tlv_hdr {
    pub tag: __le16,
    pub len: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_beacon_offload_tlv_generic {
    pub hdr: host_cmd_beacon_offload_tlv_hdr,
    pub value: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_beacon_offload_tlv_dtim_cnt {
    pub hdr: host_cmd_beacon_offload_tlv_hdr,
    pub dtim_cnt: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_beacon_offload_tlv_frame_ctrl {
    pub hdr: host_cmd_beacon_offload_tlv_hdr,
    pub frame_ctrl: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_beacon_offload_tlv_change_seq {
    pub hdr: host_cmd_beacon_offload_tlv_hdr,
    pub change_seq: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_beacon_offload_tlv_tx_info {
    pub hdr: host_cmd_beacon_offload_tlv_hdr,
    pub bw_mhz: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_beacon_offload_tlv_cssid {
    pub hdr: host_cmd_beacon_offload_tlv_hdr,
    pub cssid: [u8; HOST_CMD_BEACON_OFFLOAD_CSSID_LEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_beacon_offload_tlv_ies {
    pub hdr: host_cmd_beacon_offload_tlv_hdr,
    pub buf: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_beacon_offload {
    pub hdr: host_cmd_header,
    pub flags: __le32,
    pub variable: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_beacon_offload {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub dtim_count: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_probe_response_offload {
    pub hdr: host_cmd_header,
    pub enable: u8,
    pub probe_resp_len: __le16,
    pub probe_resp_buf: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_probe_response_offload {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_set_sta_type {
    pub hdr: host_cmd_header,
    pub sta_type: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_set_enc_mode {
    pub hdr: host_cmd_header,
    pub enc_mode: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_test_ba {
    pub hdr: host_cmd_header,
    pub addr: [u8; HOST_CMD_MAC_ADDR_LEN],
    pub start: u8,
    pub tx: u8,
    pub tid: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_set_listen_interval {
    pub hdr: host_cmd_header,
    pub listen_interval: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_set_ampdu {
    pub hdr: host_cmd_header,
    pub ampdu_enabled: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_set_s1g_op_class {
    pub hdr: host_cmd_header,
    pub opclass: u8,
    pub prim_opclass: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_send_wake_action_frame {
    pub hdr: host_cmd_header,
    pub dest_addr: [u8; HOST_CMD_MAC_ADDR_LEN],
    pub payload_size: __le32,
    pub payload: [u8; ],
    pub __packed: },
pub const HOST_CMD_MAX_VENDOR_IE_LENGTH: c_int = 255;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum host_cmd_vendor_ie_op {
    HOST_CMD_VENDOR_IE_OP_ADD_ELEMENT = 0,
    HOST_CMD_VENDOR_IE_OP_CLEAR_ELEMENTS = 1,
    HOST_CMD_VENDOR_IE_OP_ADD_FILTER = 2,
    HOST_CMD_VENDOR_IE_OP_CLEAR_FILTERS = 3,
    HOST_CMD_VENDOR_IE_OP_INVALID = U16_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_vendor_ie_config {
    pub hdr: host_cmd_header,
    pub opcode: __le16,
    pub mgmt_type_mask: __le16,
    pub data: [u8; HOST_CMD_MAX_VENDOR_IE_LENGTH],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_vendor_ie_config {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum host_cmd_twt_conf_op {
    HOST_CMD_TWT_CONF_OP_CONFIGURE = 0,
    HOST_CMD_TWT_CONF_OP_FORCE_INSTALL_AGREEMENT = 1,
    HOST_CMD_TWT_CONF_OP_REMOVE_AGREEMENT = 2,
    HOST_CMD_TWT_CONF_OP_CONFIGURE_EXPLICIT = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_explicit_twt_wake_interval {
    pub wake_interval_mantissa: __le16,
    pub wake_interval_exponent: u8,
    pub __padding: [u8; 5],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union host_cmd_wake_interval {
    pub wake_interval_us: __le64,
    pub explicit_twt: host_cmd_explicit_twt_wake_interval,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_set_twt_conf {
    pub hdr: host_cmd_header,
    pub opcode: u8,
    pub flow_id: u8,
    pub target_wake_time: __le64,
    pub wake_interval: host_cmd_wake_interval,
    pub wake_duration_us: __le32,
    pub twt_setup_command: u8,
    pub __padding: [u8; 3],
    pub __packed: },
pub const HOST_CMD_MAX_AVAILABLE_CHANNELS: c_int = 255;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_channel_info {
    pub frequency_khz: __le32,
    pub channel_5g: u8,
    pub channel_s1g: u8,
    pub bandwidth_mhz: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_get_available_channels {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub num_channels: __le32,
    pub channels: [host_cmd_channel_info; HOST_CMD_MAX_AVAILABLE_CHANNELS],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_set_ecsa_s1g_info {
    pub hdr: host_cmd_header,
    pub operating_channel_freq_hz: __le32,
    pub opclass: u8,
    pub primary_channel_bw_mhz: u8,
    pub prim_1mhz_ch_idx: u8,
    pub operating_channel_bw_mhz: u8,
    pub prim_opclass: u8,
    pub s1g_cap0: u8,
    pub s1g_cap1: u8,
    pub s1g_cap2: u8,
    pub s1g_cap3: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_get_hw_version {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub hw_version: [u8; 64],
    pub __packed: },
pub const HOST_CMD_CAC_CFG_CHANGE_RULE_MAX: c_int = 8;
pub const HOST_CMD_CAC_CFG_ARFS_MAX: c_int = 99;
pub const HOST_CMD_CAC_CFG_CHANGE_MAX: c_int = 99;
pub const HOST_CMD_CAC_CFG_CHANGE_STEP: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum host_cmd_cac_op {
    HOST_CMD_CAC_OP_DISABLE = 0,
    HOST_CMD_CAC_OP_ENABLE = 1,
    HOST_CMD_CAC_OP_CFG_GET = 2,
    HOST_CMD_CAC_OP_CFG_SET = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_cac_change_rule {
    pub arfs: __le16,
    pub threshold_change: __sle16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_cac {
    pub hdr: host_cmd_header,
    pub opcode: u8,
    pub rule_tot: u8,
    pub rule: [host_cmd_cac_change_rule; HOST_CMD_CAC_CFG_CHANGE_RULE_MAX],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_cac {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub rule_tot: u8,
    pub rule: [host_cmd_cac_change_rule; HOST_CMD_CAC_CFG_CHANGE_RULE_MAX],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ocs_driver_req {
    pub op_channel_freq_hz: __le32,
    pub op_channel_bw_mhz: u8,
    pub pri_channel_bw_mhz: u8,
    pub pri_1mhz_channel_index: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ocs_driver_resp {
    pub running: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_ocs_driver {
    pub hdr: host_cmd_header,
    pub subcmd: __le32,
    pub opaque: [u8; 0],
    pub config: host_cmd_ocs_driver_req,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_ocs_driver {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub subcmd: __le32,
    pub opaque: [u8; 0],
    pub ocs_status: host_cmd_ocs_driver_resp,
}

pub const HOST_CMD_IFNAMSIZ: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_mbssid {
    pub hdr: host_cmd_header,
    pub max_bssid_indicator: u8,
    pub transmitter_iface: [i8; HOST_CMD_IFNAMSIZ],
    pub __packed: },
pub const HOST_CMD_MESH_ID_LEN_MAX: c_int = 32;
pub const HOST_CMD_MESH_BEACONLESS_MODE_DISABLE: c_int = 0;
pub const HOST_CMD_MESH_BEACONLESS_MODE_ENABLE: c_int = 1;
pub const HOST_CMD_MESH_PEER_LINKS_MIN: c_int = 0;
pub const HOST_CMD_MESH_PEER_LINKS_MAX: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_set_mesh_config {
    pub hdr: host_cmd_header,
    pub mesh_id_len: u8,
    pub mesh_id: [u8; HOST_CMD_MESH_ID_LEN_MAX],
    pub mesh_beaconless_mode: u8,
    pub max_plinks: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_set_mcba_conf {
    pub hdr: host_cmd_header,
    pub mbca_config: u8,
    pub beacon_timing_report_interval: u8,
    pub min_beacon_gap_ms: u8,
    pub mbss_start_scan_duration_ms: __le16,
    pub tbtt_adj_interval_ms: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_dynamic_peering_config {
    pub hdr: host_cmd_header,
    pub enabled: u8,
    pub rssi_margin: u8,
    pub blacklist_timeout: __le32,
    pub __packed: },

pub const HOST_CMD_RAW_RESERVED_AID_DCS: c_int = 2008;
pub const HOST_CMD_RAW_RESERVED_AID_DOWNLINK: c_int = 2009;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum host_cmd_raw_tlv_tag {
    HOST_CMD_RAW_TLV_TAG_SLOT_DEF = 0,
    HOST_CMD_RAW_TLV_TAG_GROUP = 1,
    HOST_CMD_RAW_TLV_TAG_START_TIME = 2,
    HOST_CMD_RAW_TLV_TAG_PRAW = 3,
    HOST_CMD_RAW_TLV_TAG_BCN_SPREAD = 4,
    HOST_CMD_RAW_TLV_TAG_DYN_GLOBAL = 5,
    HOST_CMD_RAW_TLV_TAG_DYN_CONFIG = 6,
    HOST_CMD_RAW_TLV_TAG_LAST = 7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_raw_tlv_slot_def {
    pub tag: u8,
    pub raw_duration_us: __le32,
    pub num_slots: u8,
    pub cross_slot_bleed: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_raw_tlv_group {
    pub tag: u8,
    pub aid_start: __le16,
    pub aid_end: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_raw_tlv_start_time {
    pub tag: u8,
    pub start_time_us: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_raw_tlv_praw {
    pub tag: u8,
    pub periodicity: u8,
    pub validity: u8,
    pub start_offset: u8,
    pub refresh_on_expiry: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_raw_tlv_bcn_spread {
    pub tag: u8,
    pub max_spread: __le16,
    pub nominal_sta_per_bcn: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_raw_tlv_dyn_global {
    pub tag: u8,
    pub num_configs: __le16,
    pub num_bcn_indexes: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_raw_tlv_dyn_config {
    pub tag: u8,
    pub id: __le16,
    pub index: __le16,
    pub len: __le16,
    pub variable: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union host_cmd_raw_tlvs {
    pub tag: u8,
    pub slot_def: host_cmd_raw_tlv_slot_def,
    pub group: host_cmd_raw_tlv_group,
    pub start_time: host_cmd_raw_tlv_start_time,
    pub praw: host_cmd_raw_tlv_praw,
    pub bcn_spread: host_cmd_raw_tlv_bcn_spread,
    pub dyn_global: host_cmd_raw_tlv_dyn_global,
    pub dyn_config: host_cmd_raw_tlv_dyn_config,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_config_raw {
    pub hdr: host_cmd_header,
    pub flags: __le32,
    pub id: __le16,
    pub variable: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_config_bss_stats {
    pub hdr: host_cmd_header,
    pub enable: u8,
    pub monitor_window_ms: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_get_rssi {
    pub hdr: host_cmd_header,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_get_rssi {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub rssi0: __sle32,
    pub rssi1: __sle32,
    pub rssi2: __sle32,
    pub rssi3: __sle32,
    pub rssi4: __sle32,
    pub rssi5: __sle32,
    pub rssi6: __sle32,
    pub rssi7: __sle32,
    pub __packed: },
pub const HOST_CMD_SET_IFS_MIN_USECS: c_int = 160;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_set_ifs {
    pub hdr: host_cmd_header,
    pub period_usecs: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_set_ifs {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_set_fem_settings {
    pub hdr: host_cmd_header,
    pub tx_antenna: __le32,
    pub rx_antenna: __le32,
    pub lna_enabled: __le32,
    pub pa_enabled: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_set_fem_settings {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_set_txop {
    pub hdr: host_cmd_header,
    pub min_packet_count: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_set_txop {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_set_control_response {
    pub hdr: host_cmd_header,
    pub direction: u8,
    pub control_response_1mhz_en: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_set_control_response {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_set_periodic_cal {
    pub hdr: host_cmd_header,
    pub periodic_cal_en_mask: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_set_periodic_cal {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_set_bcn_rssi_threshold {
    pub hdr: host_cmd_header,
    pub threshold_db: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_set_bcn_rssi_threshold {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_set_tx_pkt_lifetime_usecs {
    pub hdr: host_cmd_header,
    pub lifetime_usecs: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_set_tx_pkt_lifetime_usecs {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_set_physm_watchdog {
    pub hdr: host_cmd_header,
    pub physm_watchdog_en: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_tx_polar {
    pub hdr: host_cmd_header,
    pub enable: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_evt_sta_state {
    pub hdr: host_cmd_header,
    pub sta_addr: [u8; HOST_CMD_MAC_ADDR_LEN],
    pub aid: __le16,
    pub state: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_evt_beacon_loss {
    pub hdr: host_cmd_header,
    pub num_bcns: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_evt_sig_field_error {
    pub hdr: host_cmd_header,
    pub start_timestamp: __le64,
    pub end_timestamp: __le64,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_evt_umac_traffic_control {
    pub hdr: host_cmd_header,
    pub pause_data_traffic: u8,
    pub sources: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_evt_dhcp_lease_update {
    pub hdr: host_cmd_header,
    pub my_ip: __le32,
    pub netmask: __le32,
    pub router: __le32,
    pub dns: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_evt_ocs_done {
    pub hdr: host_cmd_header,
    pub time_listen: __le64,
    pub time_rx: __le64,
    pub noise: i8,
    pub metric: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_evt_hw_scan_done {
    pub hdr: host_cmd_header,
    pub aborted: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_evt_channel_usage {
    pub hdr: host_cmd_header,
    pub time_listen: __le64,
    pub busy_time: __le64,
    pub freq_hz: __le32,
    pub noise: u8,
    pub bw_mhz: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum host_cmd_connection_loss_reason {
    HOST_CMD_CONNECTION_LOSS_REASON_TSF_RESET = 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_evt_connection_loss {
    pub hdr: host_cmd_header,
    pub reason: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_evt_sched_scan_results {
    pub hdr: host_cmd_header,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum host_cmd_cqm_rssi_threshold_event {
    HOST_CMD_CQM_RSSI_THRESHOLD_EVENT_LOW = 0,
    HOST_CMD_CQM_RSSI_THRESHOLD_EVENT_HIGH = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_evt_cqm_rssi_notify {
    pub hdr: host_cmd_header,
    pub rssi: __sle16,
    pub event: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_evt_scan_done {
    pub hdr: host_cmd_header,
    pub aborted: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum host_cmd_scan_result_frame {
    HOST_CMD_SCAN_RESULT_FRAME_UNKNOWN = 0,
    HOST_CMD_SCAN_RESULT_FRAME_BEACON = 1,
    HOST_CMD_SCAN_RESULT_FRAME_PROBE_RESPONSE = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_evt_scan_result {
    pub hdr: host_cmd_header,
    pub channel_freq_hz: __le32,
    pub bw_mhz: u8,
    pub frame_type: u8,
    pub rssi: __sle16,
    pub bssid: [u8; HOST_CMD_MAC_ADDR_LEN],
    pub beacon_interval: __le16,
    pub capability_info: __le16,
    pub tsf: __le64,
    pub ies_len: __le16,
    pub ies: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_evt_connected {
    pub hdr: host_cmd_header,
    pub bssid: [u8; HOST_CMD_MAC_ADDR_LEN],
    pub rssi: __sle16,
    pub padding_0: [u8; 8],
    pub assoc_resp_ies_len: __le16,
    pub assoc_resp_ies: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_evt_beacon_filter_match {
    pub hdr: host_cmd_header,
    pub padding_0: [u8; 4],
    pub ies_len: __le32,
    pub ies: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_set_capabilities {
    pub hdr: host_cmd_header,
    pub capabilities: host_cmd_mm_capabilities,
    pub set_caps: u8,
    pub morse_mmss_offset: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_set_capabilities {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_set_transmission_rate {
    pub hdr: host_cmd_header,
    pub mcs_index: __sle32,
    pub bandwidth_mhz: __sle32,
    pub tx_80211ah_format: __sle32,
    pub use_traveling_pilots: i8,
    pub use_sgi: i8,
    pub enabled: u8,
    pub nss_idx: i8,
    pub use_ldpc: i8,
    pub use_stbc: i8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_set_transmission_rate {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum host_cmd_hart_id {
    HOST_CMD_HART_ID_HOST = 0,
    HOST_CMD_HART_ID_MAC = 1,
    HOST_CMD_HART_ID_UPHY = 2,
    HOST_CMD_HART_ID_LPHY = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_force_assert {
    pub hdr: host_cmd_header,
    pub hart_id: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum host_cmd_param_action {
    HOST_CMD_PARAM_ACTION_SET = 0,
    HOST_CMD_PARAM_ACTION_GET = 1,
    HOST_CMD_PARAM_ACTION_LAST = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum host_cmd_slow_clock_mode {
    HOST_CMD_SLOW_CLOCK_MODE_AUTO = 0,
    HOST_CMD_SLOW_CLOCK_MODE_INTERNAL = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum host_cmd_param_id {
    HOST_CMD_PARAM_ID_MAX_TRAFFIC_DELIVERY_WAIT_US = 0,
    HOST_CMD_PARAM_ID_EXTRA_ACK_TIMEOUT_ADJUST_US = 1,
    HOST_CMD_PARAM_ID_TX_STATUS_FLUSH_WATERMARK = 2,
    HOST_CMD_PARAM_ID_TX_STATUS_FLUSH_MIN_AMPDU_SIZE = 3,
    HOST_CMD_PARAM_ID_POWERSAVE_TYPE = 4,
    HOST_CMD_PARAM_ID_SNOOZE_DURATION_ADJUST_US = 5,
    HOST_CMD_PARAM_ID_TX_BLOCK = 6,
    HOST_CMD_PARAM_ID_FORCED_SNOOZE_PERIOD_US = 7,
    HOST_CMD_PARAM_ID_WAKE_ACTION_GPIO = 8,
    HOST_CMD_PARAM_ID_WAKE_ACTION_GPIO_PULSE_MS = 9,
    HOST_CMD_PARAM_ID_CONNECTION_MONITOR_GPIO = 10,
    HOST_CMD_PARAM_ID_INPUT_TRIGGER_GPIO = 11,
    HOST_CMD_PARAM_ID_INPUT_TRIGGER_MODE = 12,
    HOST_CMD_PARAM_ID_COUNTRY = 13,
    HOST_CMD_PARAM_ID_RTS_THRESHOLD = 14,
    HOST_CMD_PARAM_ID_HOST_TX_BLOCK = 15,
    HOST_CMD_PARAM_ID_MEM_RETENTION_CODE = 16,
    HOST_CMD_PARAM_ID_NON_TIM_MODE = 17,
    HOST_CMD_PARAM_ID_DYNAMIC_PS_TIMEOUT_MS = 18,
    HOST_CMD_PARAM_ID_HOME_CHANNEL_DWELL_MS = 19,
    HOST_CMD_PARAM_ID_SLOW_CLOCK_MODE = 20,
    HOST_CMD_PARAM_ID_FRAGMENT_THRESHOLD = 21,
    HOST_CMD_PARAM_ID_BEACON_LOSS_COUNT = 22,
    HOST_CMD_PARAM_ID_AP_POWER_SAVE = 23,
    HOST_CMD_PARAM_ID_BEACON_OFFLOAD = 24,
    HOST_CMD_PARAM_ID_PROBE_RESP_OFFLOAD = 25,
    HOST_CMD_PARAM_ID_BSS_MAX_AWAY_DURATION = 26,
    HOST_CMD_PARAM_ID_DEFAULT_ACTIVE_SCAN_DWELL_MS = 27,
    HOST_CMD_PARAM_ID_CTS_TO_SELF = 28,
    HOST_CMD_PARAM_ID_CHANNELIZATION = 29,
    HOST_CMD_PARAM_ID_LAST = 30,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req_get_set_generic_param {
    pub hdr: host_cmd_header,
    pub param_id: __le32,
    pub action: __le32,
    pub flags: __le32,
    pub value: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp_get_set_generic_param {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub flags: __le32,
    pub value: __le32,
    pub __packed: },
