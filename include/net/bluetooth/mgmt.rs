//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/bluetooth/mgmt.h
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
pub const MGMT_INDEX_NONE: c_uint = 0xFFFF;
pub const MGMT_STATUS_SUCCESS: c_uint = 0x00;
pub const MGMT_STATUS_UNKNOWN_COMMAND: c_uint = 0x01;
pub const MGMT_STATUS_NOT_CONNECTED: c_uint = 0x02;
pub const MGMT_STATUS_FAILED: c_uint = 0x03;
pub const MGMT_STATUS_CONNECT_FAILED: c_uint = 0x04;
pub const MGMT_STATUS_AUTH_FAILED: c_uint = 0x05;
pub const MGMT_STATUS_NOT_PAIRED: c_uint = 0x06;
pub const MGMT_STATUS_NO_RESOURCES: c_uint = 0x07;
pub const MGMT_STATUS_TIMEOUT: c_uint = 0x08;
pub const MGMT_STATUS_ALREADY_CONNECTED: c_uint = 0x09;
pub const MGMT_STATUS_BUSY: c_uint = 0x0a;
pub const MGMT_STATUS_REJECTED: c_uint = 0x0b;
pub const MGMT_STATUS_NOT_SUPPORTED: c_uint = 0x0c;
pub const MGMT_STATUS_INVALID_PARAMS: c_uint = 0x0d;
pub const MGMT_STATUS_DISCONNECTED: c_uint = 0x0e;
pub const MGMT_STATUS_NOT_POWERED: c_uint = 0x0f;
pub const MGMT_STATUS_CANCELLED: c_uint = 0x10;
pub const MGMT_STATUS_INVALID_INDEX: c_uint = 0x11;
pub const MGMT_STATUS_RFKILLED: c_uint = 0x12;
pub const MGMT_STATUS_ALREADY_PAIRED: c_uint = 0x13;
pub const MGMT_STATUS_PERMISSION_DENIED: c_uint = 0x14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_hdr {
    pub opcode: __le16,
    pub index: __le16,
    pub len: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_tlv {
// New members MUST be added within the __struct_group() macro below.
    pub type: __le16,
    pub length: __u8,
    pub value: [__u8; ],
    pub __packed: },
    pub __struct_group()"): "struct member likely outside of,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_addr_info {
    pub bdaddr: bdaddr_t,
    pub type: __u8,
    pub __packed: },
pub const MGMT_ADDR_INFO_SIZE: c_int = 7;
pub const MGMT_OP_READ_VERSION: c_uint = 0x0001;
pub const MGMT_READ_VERSION_SIZE: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_rp_read_version {
    pub version: __u8,
    pub revision: __le16,
    pub __packed: },
pub const MGMT_OP_READ_COMMANDS: c_uint = 0x0002;
pub const MGMT_READ_COMMANDS_SIZE: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_rp_read_commands {
    pub num_commands: __le16,
    pub num_events: __le16,
    pub opcodes: [__le16; ],
    pub __packed: },
pub const MGMT_OP_READ_INDEX_LIST: c_uint = 0x0003;
pub const MGMT_READ_INDEX_LIST_SIZE: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_rp_read_index_list {
    pub num_controllers: __le16,
    pub index: [__le16; ],
    pub __packed: },
// Reserve one extra byte for names in management messages so that they
// are always guaranteed to be nul-terminated

pub const MGMT_OP_READ_INFO: c_uint = 0x0004;
pub const MGMT_READ_INFO_SIZE: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_rp_read_info {
    pub bdaddr: bdaddr_t,
    pub version: __u8,
    pub manufacturer: __le16,
    pub supported_settings: __le32,
    pub current_settings: __le32,
    pub dev_class: [__u8; 3],
    pub name: [__u8; MGMT_MAX_NAME_LENGTH],
    pub short_name: [__u8; MGMT_MAX_SHORT_NAME_LENGTH],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_mode {
    pub val: __u8,
    pub __packed: },
pub const MGMT_SETTING_SIZE: c_int = 1;
pub const MGMT_OP_SET_POWERED: c_uint = 0x0005;
pub const MGMT_OP_SET_DISCOVERABLE: c_uint = 0x0006;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_set_discoverable {
    pub val: __u8,
    pub timeout: __le16,
    pub __packed: },
pub const MGMT_SET_DISCOVERABLE_SIZE: c_int = 3;
pub const MGMT_OP_SET_CONNECTABLE: c_uint = 0x0007;
pub const MGMT_OP_SET_FAST_CONNECTABLE: c_uint = 0x0008;
pub const MGMT_OP_SET_BONDABLE: c_uint = 0x0009;
pub const MGMT_OP_SET_LINK_SECURITY: c_uint = 0x000A;
pub const MGMT_OP_SET_SSP: c_uint = 0x000B;
pub const MGMT_OP_SET_HS: c_uint = 0x000C;
pub const MGMT_OP_SET_LE: c_uint = 0x000D;
pub const MGMT_OP_SET_DEV_CLASS: c_uint = 0x000E;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_set_dev_class {
    pub major: __u8,
    pub minor: __u8,
    pub __packed: },
pub const MGMT_SET_DEV_CLASS_SIZE: c_int = 2;
pub const MGMT_OP_SET_LOCAL_NAME: c_uint = 0x000F;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_set_local_name {
    pub name: [__u8; MGMT_MAX_NAME_LENGTH],
    pub short_name: [__u8; MGMT_MAX_SHORT_NAME_LENGTH],
    pub __packed: },
pub const MGMT_SET_LOCAL_NAME_SIZE: c_int = 260;
pub const MGMT_OP_ADD_UUID: c_uint = 0x0010;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_add_uuid {
    pub uuid: [__u8; 16],
    pub svc_hint: __u8,
    pub __packed: },
pub const MGMT_ADD_UUID_SIZE: c_int = 17;
pub const MGMT_OP_REMOVE_UUID: c_uint = 0x0011;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_remove_uuid {
    pub uuid: [__u8; 16],
    pub __packed: },
pub const MGMT_REMOVE_UUID_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_link_key_info {
    pub addr: mgmt_addr_info,
    pub type: __u8,
    pub val: [__u8; 16],
    pub pin_len: __u8,
    pub __packed: },
pub const MGMT_OP_LOAD_LINK_KEYS: c_uint = 0x0012;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_load_link_keys {
    pub debug_keys: __u8,
    pub key_count: __le16,
    pub keys: [mgmt_link_key_info; ],
    pub __packed: },
pub const MGMT_LOAD_LINK_KEYS_SIZE: c_int = 3;
pub const MGMT_LTK_UNAUTHENTICATED: c_uint = 0x00;
pub const MGMT_LTK_AUTHENTICATED: c_uint = 0x01;
pub const MGMT_LTK_P256_UNAUTH: c_uint = 0x02;
pub const MGMT_LTK_P256_AUTH: c_uint = 0x03;
pub const MGMT_LTK_P256_DEBUG: c_uint = 0x04;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ltk_info {
    pub addr: mgmt_addr_info,
    pub type: __u8,
    pub initiator: __u8,
    pub enc_size: __u8,
    pub ediv: __le16,
    pub rand: __le64,
    pub val: [__u8; 16],
    pub __packed: },
pub const MGMT_OP_LOAD_LONG_TERM_KEYS: c_uint = 0x0013;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_load_long_term_keys {
    pub key_count: __le16,
    pub keys: [mgmt_ltk_info; ],
    pub __packed: },
pub const MGMT_LOAD_LONG_TERM_KEYS_SIZE: c_int = 2;
pub const MGMT_OP_DISCONNECT: c_uint = 0x0014;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_disconnect {
    pub addr: mgmt_addr_info,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_rp_disconnect {
    pub addr: mgmt_addr_info,
    pub __packed: },
pub const MGMT_OP_GET_CONNECTIONS: c_uint = 0x0015;
pub const MGMT_GET_CONNECTIONS_SIZE: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_rp_get_connections {
    pub conn_count: __le16,
    pub addr: [mgmt_addr_info; ],
    pub __packed: },
pub const MGMT_OP_PIN_CODE_REPLY: c_uint = 0x0016;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_pin_code_reply {
    pub addr: mgmt_addr_info,
    pub pin_len: __u8,
    pub pin_code: [__u8; 16],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_rp_pin_code_reply {
    pub addr: mgmt_addr_info,
    pub __packed: },
pub const MGMT_OP_PIN_CODE_NEG_REPLY: c_uint = 0x0017;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_pin_code_neg_reply {
    pub addr: mgmt_addr_info,
    pub __packed: },

pub const MGMT_OP_SET_IO_CAPABILITY: c_uint = 0x0018;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_set_io_capability {
    pub io_capability: __u8,
    pub __packed: },
pub const MGMT_SET_IO_CAPABILITY_SIZE: c_int = 1;
pub const MGMT_OP_PAIR_DEVICE: c_uint = 0x0019;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_pair_device {
    pub addr: mgmt_addr_info,
    pub io_cap: __u8,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_rp_pair_device {
    pub addr: mgmt_addr_info,
    pub __packed: },
pub const MGMT_OP_CANCEL_PAIR_DEVICE: c_uint = 0x001A;

pub const MGMT_OP_UNPAIR_DEVICE: c_uint = 0x001B;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_unpair_device {
    pub addr: mgmt_addr_info,
    pub disconnect: __u8,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_rp_unpair_device {
    pub addr: mgmt_addr_info,
}

pub const MGMT_OP_USER_CONFIRM_REPLY: c_uint = 0x001C;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_user_confirm_reply {
    pub addr: mgmt_addr_info,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_rp_user_confirm_reply {
    pub addr: mgmt_addr_info,
    pub __packed: },
pub const MGMT_OP_USER_CONFIRM_NEG_REPLY: c_uint = 0x001D;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_user_confirm_neg_reply {
    pub addr: mgmt_addr_info,
    pub __packed: },

pub const MGMT_OP_USER_PASSKEY_REPLY: c_uint = 0x001E;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_user_passkey_reply {
    pub addr: mgmt_addr_info,
    pub passkey: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_rp_user_passkey_reply {
    pub addr: mgmt_addr_info,
    pub __packed: },
pub const MGMT_OP_USER_PASSKEY_NEG_REPLY: c_uint = 0x001F;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_user_passkey_neg_reply {
    pub addr: mgmt_addr_info,
    pub __packed: },

pub const MGMT_OP_READ_LOCAL_OOB_DATA: c_uint = 0x0020;
pub const MGMT_READ_LOCAL_OOB_DATA_SIZE: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_rp_read_local_oob_data {
    pub hash192: [__u8; 16],
    pub rand192: [__u8; 16],
    pub hash256: [__u8; 16],
    pub rand256: [__u8; 16],
    pub __packed: },
pub const MGMT_OP_ADD_REMOTE_OOB_DATA: c_uint = 0x0021;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_add_remote_oob_data {
    pub addr: mgmt_addr_info,
    pub hash: [__u8; 16],
    pub rand: [__u8; 16],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_add_remote_oob_ext_data {
    pub addr: mgmt_addr_info,
    pub hash192: [__u8; 16],
    pub rand192: [__u8; 16],
    pub hash256: [__u8; 16],
    pub rand256: [__u8; 16],
    pub __packed: },

pub const MGMT_OP_REMOVE_REMOTE_OOB_DATA: c_uint = 0x0022;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_remove_remote_oob_data {
    pub addr: mgmt_addr_info,
    pub __packed: },

pub const MGMT_OP_START_DISCOVERY: c_uint = 0x0023;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_start_discovery {
    pub type: __u8,
    pub __packed: },
pub const MGMT_START_DISCOVERY_SIZE: c_int = 1;
pub const MGMT_OP_STOP_DISCOVERY: c_uint = 0x0024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_stop_discovery {
    pub type: __u8,
    pub __packed: },
pub const MGMT_STOP_DISCOVERY_SIZE: c_int = 1;
pub const MGMT_OP_CONFIRM_NAME: c_uint = 0x0025;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_confirm_name {
    pub addr: mgmt_addr_info,
    pub name_known: __u8,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_rp_confirm_name {
    pub addr: mgmt_addr_info,
    pub __packed: },
pub const MGMT_OP_BLOCK_DEVICE: c_uint = 0x0026;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_block_device {
    pub addr: mgmt_addr_info,
    pub __packed: },

pub const MGMT_OP_UNBLOCK_DEVICE: c_uint = 0x0027;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_unblock_device {
    pub addr: mgmt_addr_info,
    pub __packed: },

pub const MGMT_OP_SET_DEVICE_ID: c_uint = 0x0028;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_set_device_id {
    pub source: __le16,
    pub vendor: __le16,
    pub product: __le16,
    pub version: __le16,
    pub __packed: },
pub const MGMT_SET_DEVICE_ID_SIZE: c_int = 8;
pub const MGMT_OP_SET_ADVERTISING: c_uint = 0x0029;
pub const MGMT_OP_SET_BREDR: c_uint = 0x002A;
pub const MGMT_OP_SET_STATIC_ADDRESS: c_uint = 0x002B;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_set_static_address {
    pub bdaddr: bdaddr_t,
    pub __packed: },
pub const MGMT_SET_STATIC_ADDRESS_SIZE: c_int = 6;
pub const MGMT_OP_SET_SCAN_PARAMS: c_uint = 0x002C;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_set_scan_params {
    pub interval: __le16,
    pub window: __le16,
    pub __packed: },
pub const MGMT_SET_SCAN_PARAMS_SIZE: c_int = 4;
pub const MGMT_OP_SET_SECURE_CONN: c_uint = 0x002D;
pub const MGMT_OP_SET_DEBUG_KEYS: c_uint = 0x002E;
pub const MGMT_OP_SET_PRIVACY: c_uint = 0x002F;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_set_privacy {
    pub privacy: __u8,
    pub irk: [__u8; 16],
    pub __packed: },
pub const MGMT_SET_PRIVACY_SIZE: c_int = 17;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_irk_info {
    pub addr: mgmt_addr_info,
    pub val: [__u8; 16],
    pub __packed: },
pub const MGMT_OP_LOAD_IRKS: c_uint = 0x0030;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_load_irks {
    pub irk_count: __le16,
    pub irks: [mgmt_irk_info; ],
    pub __packed: },
pub const MGMT_LOAD_IRKS_SIZE: c_int = 2;
pub const MGMT_OP_GET_CONN_INFO: c_uint = 0x0031;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_get_conn_info {
    pub addr: mgmt_addr_info,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_rp_get_conn_info {
    pub addr: mgmt_addr_info,
    pub rssi: __s8,
    pub tx_power: __s8,
    pub max_tx_power: __s8,
    pub __packed: },
pub const MGMT_OP_GET_CLOCK_INFO: c_uint = 0x0032;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_get_clock_info {
    pub addr: mgmt_addr_info,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_rp_get_clock_info {
    pub addr: mgmt_addr_info,
    pub local_clock: __le32,
    pub piconet_clock: __le32,
    pub accuracy: __le16,
    pub __packed: },
pub const MGMT_OP_ADD_DEVICE: c_uint = 0x0033;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_add_device {
    pub addr: mgmt_addr_info,
    pub action: __u8,
    pub __packed: },

pub const MGMT_OP_REMOVE_DEVICE: c_uint = 0x0034;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_remove_device {
    pub addr: mgmt_addr_info,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_conn_param {
    pub addr: mgmt_addr_info,
    pub min_interval: __le16,
    pub max_interval: __le16,
    pub latency: __le16,
    pub timeout: __le16,
    pub __packed: },
pub const MGMT_OP_LOAD_CONN_PARAM: c_uint = 0x0035;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_load_conn_param {
    pub param_count: __le16,
    pub params: [mgmt_conn_param; ],
    pub __packed: },
pub const MGMT_LOAD_CONN_PARAM_SIZE: c_int = 2;
pub const MGMT_OP_READ_UNCONF_INDEX_LIST: c_uint = 0x0036;
pub const MGMT_READ_UNCONF_INDEX_LIST_SIZE: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_rp_read_unconf_index_list {
    pub num_controllers: __le16,
    pub index: [__le16; ],
    pub __packed: },
pub const MGMT_OPTION_EXTERNAL_CONFIG: c_uint = 0x00000001;
pub const MGMT_OPTION_PUBLIC_ADDRESS: c_uint = 0x00000002;
pub const MGMT_OP_READ_CONFIG_INFO: c_uint = 0x0037;
pub const MGMT_READ_CONFIG_INFO_SIZE: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_rp_read_config_info {
    pub manufacturer: __le16,
    pub supported_options: __le32,
    pub missing_options: __le32,
    pub __packed: },
pub const MGMT_OP_SET_EXTERNAL_CONFIG: c_uint = 0x0038;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_set_external_config {
    pub config: __u8,
    pub __packed: },
pub const MGMT_SET_EXTERNAL_CONFIG_SIZE: c_int = 1;
pub const MGMT_OP_SET_PUBLIC_ADDRESS: c_uint = 0x0039;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_set_public_address {
    pub bdaddr: bdaddr_t,
    pub __packed: },
pub const MGMT_SET_PUBLIC_ADDRESS_SIZE: c_int = 6;
pub const MGMT_OP_START_SERVICE_DISCOVERY: c_uint = 0x003A;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_start_service_discovery {
    pub type: __u8,
    pub rssi: __s8,
    pub uuid_count: __le16,
    pub uuids: [__u8; ][16],
    pub __packed: },
pub const MGMT_START_SERVICE_DISCOVERY_SIZE: c_int = 4;
pub const MGMT_OP_READ_LOCAL_OOB_EXT_DATA: c_uint = 0x003B;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_read_local_oob_ext_data {
    pub type: __u8,
    pub __packed: },
pub const MGMT_READ_LOCAL_OOB_EXT_DATA_SIZE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_rp_read_local_oob_ext_data {
    pub type: __u8,
    pub eir_len: __le16,
    pub eir: [__u8; ],
    pub __packed: },
pub const MGMT_OP_READ_EXT_INDEX_LIST: c_uint = 0x003C;
pub const MGMT_READ_EXT_INDEX_LIST_SIZE: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_rp_read_ext_index_list {
    pub num_controllers: __le16,
    pub index: __le16,
    pub type: __u8,
    pub bus: __u8,
    pub entry: [}; ],
    pub __packed: },
pub const MGMT_OP_READ_ADV_FEATURES: c_uint = 0x0003D;
pub const MGMT_READ_ADV_FEATURES_SIZE: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_rp_read_adv_features {
    pub supported_flags: __le32,
    pub max_adv_data_len: __u8,
    pub max_scan_rsp_len: __u8,
    pub max_instances: __u8,
    pub num_instances: __u8,
    pub instance: [__u8; ],
    pub __packed: },
pub const MGMT_OP_ADD_ADVERTISING: c_uint = 0x003E;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_add_advertising {
    pub instance: __u8,
    pub flags: __le32,
    pub duration: __le16,
    pub timeout: __le16,
    pub adv_data_len: __u8,
    pub scan_rsp_len: __u8,
    pub data: [__u8; ],
    pub __packed: },
pub const MGMT_ADD_ADVERTISING_SIZE: c_int = 11;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_rp_add_advertising {
    pub instance: __u8,
    pub __packed: },

pub const MGMT_OP_REMOVE_ADVERTISING: c_uint = 0x003F;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_remove_advertising {
    pub instance: __u8,
    pub __packed: },
pub const MGMT_REMOVE_ADVERTISING_SIZE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_rp_remove_advertising {
    pub instance: __u8,
    pub __packed: },
pub const MGMT_OP_GET_ADV_SIZE_INFO: c_uint = 0x0040;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_get_adv_size_info {
    pub instance: __u8,
    pub flags: __le32,
    pub __packed: },
pub const MGMT_GET_ADV_SIZE_INFO_SIZE: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_rp_get_adv_size_info {
    pub instance: __u8,
    pub flags: __le32,
    pub max_adv_data_len: __u8,
    pub max_scan_rsp_len: __u8,
    pub __packed: },
pub const MGMT_OP_START_LIMITED_DISCOVERY: c_uint = 0x0041;
pub const MGMT_OP_READ_EXT_INFO: c_uint = 0x0042;
pub const MGMT_READ_EXT_INFO_SIZE: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_rp_read_ext_info {
    pub bdaddr: bdaddr_t,
    pub version: __u8,
    pub manufacturer: __le16,
    pub supported_settings: __le32,
    pub current_settings: __le32,
    pub eir_len: __le16,
    pub eir: [__u8; ],
    pub __packed: },
pub const MGMT_OP_SET_APPEARANCE: c_uint = 0x0043;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_set_appearance {
    pub appearance: __le16,
    pub __packed: },
pub const MGMT_SET_APPEARANCE_SIZE: c_int = 2;
pub const MGMT_OP_GET_PHY_CONFIGURATION: c_uint = 0x0044;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_rp_get_phy_configuration {
    pub supported_phys: __le32,
    pub configurable_phys: __le32,
    pub selected_phys: __le32,
    pub __packed: },
pub const MGMT_GET_PHY_CONFIGURATION_SIZE: c_int = 0;

pub const MGMT_OP_SET_PHY_CONFIGURATION: c_uint = 0x0045;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_set_phy_configuration {
    pub selected_phys: __le32,
    pub __packed: },
pub const MGMT_SET_PHY_CONFIGURATION_SIZE: c_int = 4;
pub const MGMT_OP_SET_BLOCKED_KEYS: c_uint = 0x0046;
pub const HCI_BLOCKED_KEY_TYPE_LINKKEY: c_uint = 0x00;
pub const HCI_BLOCKED_KEY_TYPE_LTK: c_uint = 0x01;
pub const HCI_BLOCKED_KEY_TYPE_IRK: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_blocked_key_info {
    pub type: __u8,
    pub val: [__u8; 16],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_set_blocked_keys {
    pub key_count: __le16,
    pub keys: [mgmt_blocked_key_info; ],
    pub __packed: },
pub const MGMT_OP_SET_BLOCKED_KEYS_SIZE: c_int = 2;
pub const MGMT_OP_SET_WIDEBAND_SPEECH: c_uint = 0x0047;
pub const MGMT_CAP_SEC_FLAGS: c_uint = 0x01;
pub const MGMT_CAP_MAX_ENC_KEY_SIZE: c_uint = 0x02;
pub const MGMT_CAP_SMP_MAX_ENC_KEY_SIZE: c_uint = 0x03;
pub const MGMT_CAP_LE_TX_PWR: c_uint = 0x04;
pub const MGMT_OP_READ_CONTROLLER_CAP: c_uint = 0x0048;
pub const MGMT_READ_CONTROLLER_CAP_SIZE: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_rp_read_controller_cap {
    pub cap_len: __le16,
    pub cap: [__u8; ],
    pub __packed: },
pub const MGMT_OP_READ_EXP_FEATURES_INFO: c_uint = 0x0049;
pub const MGMT_READ_EXP_FEATURES_INFO_SIZE: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_rp_read_exp_features_info {
    pub feature_count: __le16,
    pub uuid: [__u8; 16],
    pub flags: __le32,
    pub features: [}; ],
    pub __packed: },
pub const MGMT_OP_SET_EXP_FEATURE: c_uint = 0x004a;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_set_exp_feature {
    pub uuid: [__u8; 16],
    pub param: [__u8; ],
    pub __packed: },
pub const MGMT_SET_EXP_FEATURE_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_rp_set_exp_feature {
    pub uuid: [__u8; 16],
    pub flags: __le32,
    pub __packed: },
pub const MGMT_OP_READ_DEF_SYSTEM_CONFIG: c_uint = 0x004b;
pub const MGMT_READ_DEF_SYSTEM_CONFIG_SIZE: c_int = 0;
pub const MGMT_OP_SET_DEF_SYSTEM_CONFIG: c_uint = 0x004c;
pub const MGMT_SET_DEF_SYSTEM_CONFIG_SIZE: c_int = 0;
pub const MGMT_OP_READ_DEF_RUNTIME_CONFIG: c_uint = 0x004d;
pub const MGMT_READ_DEF_RUNTIME_CONFIG_SIZE: c_int = 0;
pub const MGMT_OP_SET_DEF_RUNTIME_CONFIG: c_uint = 0x004e;
pub const MGMT_SET_DEF_RUNTIME_CONFIG_SIZE: c_int = 0;
pub const MGMT_OP_GET_DEVICE_FLAGS: c_uint = 0x004F;
pub const MGMT_GET_DEVICE_FLAGS_SIZE: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_get_device_flags {
    pub addr: mgmt_addr_info,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_rp_get_device_flags {
    pub addr: mgmt_addr_info,
    pub supported_flags: __le32,
    pub current_flags: __le32,
    pub __packed: },
pub const MGMT_OP_SET_DEVICE_FLAGS: c_uint = 0x0050;
pub const MGMT_SET_DEVICE_FLAGS_SIZE: c_int = 11;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_set_device_flags {
    pub addr: mgmt_addr_info,
    pub current_flags: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_rp_set_device_flags {
    pub addr: mgmt_addr_info,
    pub __packed: },

pub const MGMT_OP_READ_ADV_MONITOR_FEATURES: c_uint = 0x0051;
pub const MGMT_READ_ADV_MONITOR_FEATURES_SIZE: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_rp_read_adv_monitor_features {
    pub supported_features: __le32,
    pub enabled_features: __le32,
    pub max_num_handles: __le16,
    pub max_num_patterns: __u8,
    pub num_handles: __le16,
    pub handles: [__le16; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_adv_pattern {
    pub ad_type: __u8,
    pub offset: __u8,
    pub length: __u8,
    pub value: [__u8; HCI_MAX_AD_LENGTH],
    pub __packed: },
pub const MGMT_OP_ADD_ADV_PATTERNS_MONITOR: c_uint = 0x0052;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_add_adv_patterns_monitor {
    pub pattern_count: __u8,
    pub patterns: [mgmt_adv_pattern; ],
    pub __packed: },
pub const MGMT_ADD_ADV_PATTERNS_MONITOR_SIZE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_rp_add_adv_patterns_monitor {
    pub monitor_handle: __le16,
    pub __packed: },
pub const MGMT_OP_REMOVE_ADV_MONITOR: c_uint = 0x0053;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_remove_adv_monitor {
    pub monitor_handle: __le16,
    pub __packed: },
pub const MGMT_REMOVE_ADV_MONITOR_SIZE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_rp_remove_adv_monitor {
    pub monitor_handle: __le16,
    pub __packed: },
pub const MGMT_OP_ADD_EXT_ADV_PARAMS: c_uint = 0x0054;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_add_ext_adv_params {
    pub instance: __u8,
    pub flags: __le32,
    pub duration: __le16,
    pub timeout: __le16,
    pub min_interval: __le32,
    pub max_interval: __le32,
    pub tx_power: __s8,
    pub __packed: },
pub const MGMT_ADD_EXT_ADV_PARAMS_MIN_SIZE: c_int = 18;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_rp_add_ext_adv_params {
    pub instance: __u8,
    pub tx_power: __s8,
    pub max_adv_data_len: __u8,
    pub max_scan_rsp_len: __u8,
    pub __packed: },
pub const MGMT_OP_ADD_EXT_ADV_DATA: c_uint = 0x0055;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_add_ext_adv_data {
    pub instance: __u8,
    pub adv_data_len: __u8,
    pub scan_rsp_len: __u8,
    pub data: [__u8; ],
    pub __packed: },
pub const MGMT_ADD_EXT_ADV_DATA_SIZE: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_rp_add_ext_adv_data {
    pub instance: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_adv_rssi_thresholds {
    pub high_threshold: __s8,
    pub high_threshold_timeout: __le16,
    pub low_threshold: __s8,
    pub low_threshold_timeout: __le16,
    pub sampling_period: __u8,
    pub __packed: },
pub const MGMT_OP_ADD_ADV_PATTERNS_MONITOR_RSSI: c_uint = 0x0056;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_add_adv_patterns_monitor_rssi {
    pub rssi: mgmt_adv_rssi_thresholds,
    pub pattern_count: __u8,
    pub patterns: [mgmt_adv_pattern; ],
    pub __packed: },
pub const MGMT_ADD_ADV_PATTERNS_MONITOR_RSSI_SIZE: c_int = 8;
pub const MGMT_OP_SET_MESH_RECEIVER: c_uint = 0x0057;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_set_mesh {
    pub enable: __u8,
    pub window: __le16,
    pub period: __le16,
    pub num_ad_types: __u8,
    pub __counted_by(num_ad_types): __u8 ad_types[],
    pub __packed: },
pub const MGMT_SET_MESH_RECEIVER_SIZE: c_int = 6;
pub const MGMT_OP_MESH_READ_FEATURES: c_uint = 0x0058;
pub const MGMT_MESH_READ_FEATURES_SIZE: c_int = 0;
pub const MESH_HANDLES_MAX: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_rp_mesh_read_features {
    pub index: __le16,
    pub max_handles: __u8,
    pub used_handles: __u8,
    pub handles: [__u8; MESH_HANDLES_MAX],
    pub __packed: },
pub const MGMT_OP_MESH_SEND: c_uint = 0x0059;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_mesh_send {
    pub addr: mgmt_addr_info,
    pub instant: __le64,
    pub delay: __le16,
    pub cnt: __u8,
    pub adv_data_len: __u8,
    pub adv_data: [__u8; ],
    pub __packed: },
pub const MGMT_MESH_SEND_SIZE: c_int = 19;
pub const MGMT_OP_MESH_SEND_CANCEL: c_uint = 0x005A;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_mesh_send_cancel {
    pub handle: __u8,
    pub __packed: },
pub const MGMT_MESH_SEND_CANCEL_SIZE: c_int = 1;
pub const MGMT_OP_HCI_CMD_SYNC: c_uint = 0x005B;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_hci_cmd_sync {
    pub opcode: __le16,
    pub event: __u8,
    pub timeout: __u8,
    pub params_len: __le16,
    pub params: [__u8; ],
    pub __packed: },
pub const MGMT_HCI_CMD_SYNC_SIZE: c_int = 6;
pub const MGMT_OP_LOAD_CONN_SUBRATE: c_uint = 0x005C;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_conn_subrate {
    pub addr: mgmt_addr_info,
    pub min_interval: __le16,
    pub max_interval: __le16,
    pub subrate_min: __le16,
    pub subrate_max: __le16,
    pub max_latency: __le16,
    pub cont_num: __le16,
    pub supv_timeout: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cp_load_conn_subrate {
    pub param_count: __le16,
    pub __counted_by_le(param_count): mgmt_conn_subrate params[],
    pub __packed: },
pub const MGMT_LOAD_CONN_SUBRATE_SIZE: c_int = 2;
pub const MGMT_EV_CMD_COMPLETE: c_uint = 0x0001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_cmd_complete {
    pub opcode: __le16,
    pub status: __u8,
    pub data: [__u8; ],
    pub __packed: },
pub const MGMT_EV_CMD_STATUS: c_uint = 0x0002;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_cmd_status {
    pub opcode: __le16,
    pub status: __u8,
    pub __packed: },
pub const MGMT_EV_CONTROLLER_ERROR: c_uint = 0x0003;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_controller_error {
    pub error_code: __u8,
    pub __packed: },
pub const MGMT_EV_INDEX_ADDED: c_uint = 0x0004;
pub const MGMT_EV_INDEX_REMOVED: c_uint = 0x0005;
pub const MGMT_EV_NEW_SETTINGS: c_uint = 0x0006;
pub const MGMT_EV_CLASS_OF_DEV_CHANGED: c_uint = 0x0007;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_class_of_dev_changed {
    pub dev_class: [__u8; 3],
}

pub const MGMT_EV_LOCAL_NAME_CHANGED: c_uint = 0x0008;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_local_name_changed {
    pub name: [__u8; MGMT_MAX_NAME_LENGTH],
    pub short_name: [__u8; MGMT_MAX_SHORT_NAME_LENGTH],
    pub __packed: },
pub const MGMT_EV_NEW_LINK_KEY: c_uint = 0x0009;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_new_link_key {
    pub store_hint: __u8,
    pub key: mgmt_link_key_info,
    pub __packed: },
pub const MGMT_EV_NEW_LONG_TERM_KEY: c_uint = 0x000A;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_new_long_term_key {
    pub store_hint: __u8,
    pub key: mgmt_ltk_info,
    pub __packed: },
pub const MGMT_EV_DEVICE_CONNECTED: c_uint = 0x000B;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_device_connected {
    pub addr: mgmt_addr_info,
    pub flags: __le32,
    pub eir_len: __le16,
    pub eir: [__u8; ],
    pub __packed: },
pub const MGMT_DEV_DISCONN_UNKNOWN: c_uint = 0x00;
pub const MGMT_DEV_DISCONN_TIMEOUT: c_uint = 0x01;
pub const MGMT_DEV_DISCONN_LOCAL_HOST: c_uint = 0x02;
pub const MGMT_DEV_DISCONN_REMOTE: c_uint = 0x03;
pub const MGMT_DEV_DISCONN_AUTH_FAILURE: c_uint = 0x04;
pub const MGMT_DEV_DISCONN_LOCAL_HOST_SUSPEND: c_uint = 0x05;
pub const MGMT_EV_DEVICE_DISCONNECTED: c_uint = 0x000C;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_device_disconnected {
    pub addr: mgmt_addr_info,
    pub reason: __u8,
    pub __packed: },
pub const MGMT_EV_CONNECT_FAILED: c_uint = 0x000D;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_connect_failed {
    pub addr: mgmt_addr_info,
    pub status: __u8,
    pub __packed: },
pub const MGMT_EV_PIN_CODE_REQUEST: c_uint = 0x000E;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_pin_code_request {
    pub addr: mgmt_addr_info,
    pub secure: __u8,
    pub __packed: },
pub const MGMT_EV_USER_CONFIRM_REQUEST: c_uint = 0x000F;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_user_confirm_request {
    pub addr: mgmt_addr_info,
    pub confirm_hint: __u8,
    pub value: __le32,
    pub __packed: },
pub const MGMT_EV_USER_PASSKEY_REQUEST: c_uint = 0x0010;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_user_passkey_request {
    pub addr: mgmt_addr_info,
    pub __packed: },
pub const MGMT_EV_AUTH_FAILED: c_uint = 0x0011;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_auth_failed {
    pub addr: mgmt_addr_info,
    pub status: __u8,
    pub __packed: },

pub const MGMT_EV_DEVICE_FOUND: c_uint = 0x0012;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_device_found {
    pub addr: mgmt_addr_info,
    pub rssi: __s8,
    pub flags: __le32,
    pub eir_len: __le16,
    pub eir: [__u8; ],
    pub __packed: },
pub const MGMT_EV_DISCOVERING: c_uint = 0x0013;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_discovering {
    pub type: __u8,
    pub discovering: __u8,
    pub __packed: },
pub const MGMT_EV_DEVICE_BLOCKED: c_uint = 0x0014;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_device_blocked {
    pub addr: mgmt_addr_info,
    pub __packed: },
pub const MGMT_EV_DEVICE_UNBLOCKED: c_uint = 0x0015;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_device_unblocked {
    pub addr: mgmt_addr_info,
    pub __packed: },
pub const MGMT_EV_DEVICE_UNPAIRED: c_uint = 0x0016;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_device_unpaired {
    pub addr: mgmt_addr_info,
    pub __packed: },
pub const MGMT_EV_PASSKEY_NOTIFY: c_uint = 0x0017;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_passkey_notify {
    pub addr: mgmt_addr_info,
    pub passkey: __le32,
    pub entered: __u8,
    pub __packed: },
pub const MGMT_EV_NEW_IRK: c_uint = 0x0018;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_new_irk {
    pub store_hint: __u8,
    pub rpa: bdaddr_t,
    pub irk: mgmt_irk_info,
    pub __packed: },
pub const MGMT_CSRK_LOCAL_UNAUTHENTICATED: c_uint = 0x00;
pub const MGMT_CSRK_REMOTE_UNAUTHENTICATED: c_uint = 0x01;
pub const MGMT_CSRK_LOCAL_AUTHENTICATED: c_uint = 0x02;
pub const MGMT_CSRK_REMOTE_AUTHENTICATED: c_uint = 0x03;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_csrk_info {
    pub addr: mgmt_addr_info,
    pub type: __u8,
    pub val: [__u8; 16],
    pub __packed: },
pub const MGMT_EV_NEW_CSRK: c_uint = 0x0019;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_new_csrk {
    pub store_hint: __u8,
    pub key: mgmt_csrk_info,
    pub __packed: },
pub const MGMT_EV_DEVICE_ADDED: c_uint = 0x001a;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_device_added {
    pub addr: mgmt_addr_info,
    pub action: __u8,
    pub __packed: },
pub const MGMT_EV_DEVICE_REMOVED: c_uint = 0x001b;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_device_removed {
    pub addr: mgmt_addr_info,
    pub __packed: },
pub const MGMT_EV_NEW_CONN_PARAM: c_uint = 0x001c;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_new_conn_param {
    pub addr: mgmt_addr_info,
    pub store_hint: __u8,
    pub min_interval: __le16,
    pub max_interval: __le16,
    pub latency: __le16,
    pub timeout: __le16,
    pub __packed: },
pub const MGMT_EV_UNCONF_INDEX_ADDED: c_uint = 0x001d;
pub const MGMT_EV_UNCONF_INDEX_REMOVED: c_uint = 0x001e;
pub const MGMT_EV_NEW_CONFIG_OPTIONS: c_uint = 0x001f;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_ext_index {
    pub type: __u8,
    pub bus: __u8,
    pub __packed: },
pub const MGMT_EV_EXT_INDEX_ADDED: c_uint = 0x0020;
pub const MGMT_EV_EXT_INDEX_REMOVED: c_uint = 0x0021;
pub const MGMT_EV_LOCAL_OOB_DATA_UPDATED: c_uint = 0x0022;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_local_oob_data_updated {
    pub type: __u8,
    pub eir_len: __le16,
    pub eir: [__u8; ],
    pub __packed: },
pub const MGMT_EV_ADVERTISING_ADDED: c_uint = 0x0023;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_advertising_added {
    pub instance: __u8,
    pub __packed: },
pub const MGMT_EV_ADVERTISING_REMOVED: c_uint = 0x0024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_advertising_removed {
    pub instance: __u8,
    pub __packed: },
pub const MGMT_EV_EXT_INFO_CHANGED: c_uint = 0x0025;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_ext_info_changed {
    pub eir_len: __le16,
    pub eir: [__u8; ],
    pub __packed: },
pub const MGMT_EV_PHY_CONFIGURATION_CHANGED: c_uint = 0x0026;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_phy_configuration_changed {
    pub selected_phys: __le32,
    pub __packed: },
pub const MGMT_EV_EXP_FEATURE_CHANGED: c_uint = 0x0027;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_exp_feature_changed {
    pub uuid: [__u8; 16],
    pub flags: __le32,
    pub __packed: },
pub const MGMT_EV_DEVICE_FLAGS_CHANGED: c_uint = 0x002a;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_device_flags_changed {
    pub addr: mgmt_addr_info,
    pub supported_flags: __le32,
    pub current_flags: __le32,
    pub __packed: },
pub const MGMT_EV_ADV_MONITOR_ADDED: c_uint = 0x002b;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_adv_monitor_added {
    pub monitor_handle: __le16,
    pub __packed: },
pub const MGMT_EV_ADV_MONITOR_REMOVED: c_uint = 0x002c;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_adv_monitor_removed {
    pub monitor_handle: __le16,
    pub __packed: },
pub const MGMT_EV_CONTROLLER_SUSPEND: c_uint = 0x002d;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_controller_suspend {
    pub suspend_state: __u8,
    pub __packed: },
pub const MGMT_EV_CONTROLLER_RESUME: c_uint = 0x002e;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_controller_resume {
    pub wake_reason: __u8,
    pub addr: mgmt_addr_info,
    pub __packed: },
pub const MGMT_WAKE_REASON_NON_BT_WAKE: c_uint = 0x0;
pub const MGMT_WAKE_REASON_UNEXPECTED: c_uint = 0x1;
pub const MGMT_WAKE_REASON_REMOTE_WAKE: c_uint = 0x2;
pub const MGMT_EV_ADV_MONITOR_DEVICE_FOUND: c_uint = 0x002f;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_adv_monitor_device_found {
    pub monitor_handle: __le16,
    pub addr: mgmt_addr_info,
    pub rssi: __s8,
    pub flags: __le32,
    pub eir_len: __le16,
    pub eir: [__u8; ],
    pub __packed: },
pub const MGMT_EV_ADV_MONITOR_DEVICE_LOST: c_uint = 0x0030;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_adv_monitor_device_lost {
    pub monitor_handle: __le16,
    pub addr: mgmt_addr_info,
    pub __packed: },
pub const MGMT_EV_MESH_DEVICE_FOUND: c_uint = 0x0031;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_mesh_device_found {
    pub addr: mgmt_addr_info,
    pub rssi: __s8,
    pub instant: __le64,
    pub flags: __le32,
    pub eir_len: __le16,
    pub eir: [__u8; ],
    pub __packed: },
pub const MGMT_EV_MESH_PACKET_CMPLT: c_uint = 0x0032;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_mesh_pkt_cmplt {
    pub handle: __u8,
    pub __packed: },
pub const MGMT_EV_CONN_SUBRATE: c_uint = 0x0033;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ev_conn_subrate {
    pub addr: mgmt_addr_info,
    pub status: __u8,
    pub interval: __le16,
    pub subrate: __le16,
    pub latency: __le16,
    pub cont_num: __le16,
    pub supv_timeout: __le16,
    pub __packed: },
