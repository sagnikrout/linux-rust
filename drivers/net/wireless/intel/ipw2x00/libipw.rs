//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/ipw2x00/libipw.h
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
// Merged with mainline ieee80211.h in Aug 2004.  Original ieee802_11
// remains copyright by the original authors
//
// Portions of the merged code are based on Host AP (software wireless
// LAN access point) driver for Intersil Prism2/2.5/3.
//
// Copyright (c) 2001-2002, SSH Communications Security Corp and Jouni Malinen
// <j@w1.fi>
// Copyright (c) 2002-2003, Jouni Malinen <j@w1.fi>
//
// Adaption to a generic IEEE 802.11 stack by James Ketrenos
// <jketreno@linux.intel.com>
// Copyright (c) 2004-2005, Intel Corporation
//
// API Version History
// 1.0.x -- Initial version
// 1.1.x -- Added radiotap, QoS, TIM, libipw_geo APIs,
// various structure changes, and crypto API init method
//

pub const LIBIPW_DATA_LEN: c_int = 2304;
// Maximum size for the MA-UNITDATA primitive, 802.11 standard section
pub const LIBIPW_1ADDR_LEN: c_int = 10;
pub const LIBIPW_2ADDR_LEN: c_int = 16;
pub const LIBIPW_3ADDR_LEN: c_int = 24;
pub const LIBIPW_4ADDR_LEN: c_int = 30;
pub const LIBIPW_FCS_LEN: c_int = 4;

// QOS control
pub const LIBIPW_QCTL_TID: c_uint = 0x000F;
// debug macros

//
// To use the debug system:
//
// If you are defining a new debug classification, simply add it to the #define
// list here in the form of:
//
// #define LIBIPW_DL_xxxx VALUE
//
// shifting value to the left one bit from the previous entry.  xxxx should be
// the name of the classification (for example, WEP)
//
// You then need to either add a LIBIPW_xxxx_DEBUG() macro definition for your
// classification, or use LIBIPW_DEBUG(LIBIPW_DL_xxxx, ...) whenever you want
// to send output to that classification.
//
// To add your debug level to the list of levels seen when you perform
//
// % cat /proc/net/ieee80211/debug_level
//
// you simply need to add your entry to the libipw_debug_level array.
//
// If you do not see debug_level in /proc/net/ieee80211 then you do not have
// CONFIG_LIBIPW_DEBUG defined in your kernel configuration
//

pub const ETH_P_PREAUTH: c_uint = 0x88C7	/* IEEE 802.11i pre-authentication */;

// IEEE 802.11 defines
pub const P80211_OUI_LEN: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_snap_hdr {
    pub /: *mut *mut u8 dsap; / always 0xAA,
    pub /: *mut *mut u8 ssap; / always 0xAA,
    pub /: *mut *mut u8 ctrl; / always 0x03,
    pub /: *mut *mut u8 oui[P80211_OUI_LEN]; / organizational universal id,
    pub __packed: },

pub const LIBIPW_STATMASK_WEMASK: c_uint = 0x7;

pub const LIBIPW_CCK_RATE_1MB: c_uint = 0x02;
pub const LIBIPW_CCK_RATE_2MB: c_uint = 0x04;
pub const LIBIPW_CCK_RATE_5MB: c_uint = 0x0B;
pub const LIBIPW_CCK_RATE_11MB: c_uint = 0x16;
pub const LIBIPW_OFDM_RATE_6MB: c_uint = 0x0C;
pub const LIBIPW_OFDM_RATE_9MB: c_uint = 0x12;
pub const LIBIPW_OFDM_RATE_12MB: c_uint = 0x18;
pub const LIBIPW_OFDM_RATE_18MB: c_uint = 0x24;
pub const LIBIPW_OFDM_RATE_24MB: c_uint = 0x30;
pub const LIBIPW_OFDM_RATE_36MB: c_uint = 0x48;
pub const LIBIPW_OFDM_RATE_48MB: c_uint = 0x60;
pub const LIBIPW_OFDM_RATE_54MB: c_uint = 0x6C;
pub const LIBIPW_BASIC_RATE_MASK: c_uint = 0x80;

pub const LIBIPW_CCK_RATES_MASK: c_uint = 0x0000000F;

pub const LIBIPW_OFDM_RATES_MASK: c_uint = 0x00000FF0;

pub const LIBIPW_NUM_OFDM_RATES: c_int = 8;
pub const LIBIPW_NUM_CCK_RATES: c_int = 4;
pub const LIBIPW_OFDM_SHIFT_MASK_A: c_int = 4;
// NOTE: This data is for statistical purposes; not all hardware provides this
// information for frames received.
// For libipw_rx_mgt, you need to set at least the 'len' parameter.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_rx_stats {
    pub mac_time: u32,
    pub rssi: i8,
    pub signal: u8,
    pub noise: u8,
    pub /: *mut *mut u16 rate; / in 100 kbps,
    pub received_channel: u8,
    pub control: u8,
    pub mask: u8,
    pub freq: u8,
    pub len: u16,
    pub tsf: u64,
    pub beacon_time: u32,
}

// IEEE 802.11 requires that STA supports concurrent reception of at least
// three fragmented frames. This define can be increased to support more
// concurrent frames, but it should be noted that each entry can consume about
// 2 kB of RAM and increasing cache size will slow down frame reassembly.
pub const LIBIPW_FRAG_CACHE_LEN: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_frag_entry {
    pub first_frag_time: c_ulong,
    pub seq: c_uint,
    pub last_frag: c_uint,
    pub skb: *mut sk_buff,
    pub src_addr: [u8; ETH_ALEN],
    pub dst_addr: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_stats {
    pub tx_unicast_frames: c_uint,
    pub tx_multicast_frames: c_uint,
    pub tx_fragments: c_uint,
    pub tx_unicast_octets: c_uint,
    pub tx_multicast_octets: c_uint,
    pub tx_deferred_transmissions: c_uint,
    pub tx_single_retry_frames: c_uint,
    pub tx_multiple_retry_frames: c_uint,
    pub tx_retry_limit_exceeded: c_uint,
    pub tx_discards: c_uint,
    pub rx_unicast_frames: c_uint,
    pub rx_multicast_frames: c_uint,
    pub rx_fragments: c_uint,
    pub rx_unicast_octets: c_uint,
    pub rx_multicast_octets: c_uint,
    pub rx_fcs_errors: c_uint,
    pub rx_discards_no_buffer: c_uint,
    pub tx_discards_wrong_sa: c_uint,
    pub rx_discards_undecryptable: c_uint,
    pub rx_message_in_msg_fragments: c_uint,
    pub rx_message_in_bad_msg_fragments: c_uint,
}

pub const SEC_ALG_NONE: c_int = 0;
pub const SEC_ALG_WEP: c_int = 1;
pub const SEC_ALG_TKIP: c_int = 2;
pub const SEC_ALG_CCMP: c_int = 3;
pub const WEP_KEYS: c_int = 4;
pub const WEP_KEY_LEN: c_int = 13;
pub const SCM_KEY_LEN: c_int = 32;
pub const SCM_TEMPORAL_KEY_LENGTH: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_security {
    pub encrypt:1: u16 active_key:2, enabled:1, unicast_uses_group:1,,
    pub auth_mode: u8,
    pub encode_alg: [u8; WEP_KEYS],
    pub key_sizes: [u8; WEP_KEYS],
    pub keys: [u8; WEP_KEYS][SCM_KEY_LEN],
    pub level: u8,
    pub flags: u16,
    pub __packed: },
//
pub const BEACON_PROBE_SSID_ID_POSITION: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_hdr_1addr {
    pub frame_ctl: __le16,
    pub duration_id: __le16,
    pub addr1: [u8; ETH_ALEN],
    pub payload: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_hdr_2addr {
    pub frame_ctl: __le16,
    pub duration_id: __le16,
    pub addr1: [u8; ETH_ALEN],
    pub addr2: [u8; ETH_ALEN],
    pub payload: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_hdr_3addr {
// New members MUST be added within the __struct_group() macro below.
    pub frame_ctl: __le16,
    pub duration_id: __le16,
    pub addr1: [u8; ETH_ALEN],
    pub addr2: [u8; ETH_ALEN],
    pub addr3: [u8; ETH_ALEN],
    pub seq_ctl: __le16,
    pub payload: [u8; ],
    pub __packed: },
    pub __struct_group()"): "struct member likely outside of,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_hdr_4addr {
    pub frame_ctl: __le16,
    pub duration_id: __le16,
    pub addr1: [u8; ETH_ALEN],
    pub addr2: [u8; ETH_ALEN],
    pub addr3: [u8; ETH_ALEN],
    pub seq_ctl: __le16,
    pub addr4: [u8; ETH_ALEN],
    pub payload: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_hdr_3addrqos {
    pub frame_ctl: __le16,
    pub duration_id: __le16,
    pub addr1: [u8; ETH_ALEN],
    pub addr2: [u8; ETH_ALEN],
    pub addr3: [u8; ETH_ALEN],
    pub seq_ctl: __le16,
    pub payload: [u8; 0],
    pub qos_ctl: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_info_element {
    pub id: u8,
    pub len: u8,
    pub data: [u8; ],
    pub __packed: },
//
// These are the data types that can make up management packets
//
    pub auth_algorithm: u16,
    pub auth_sequence: u16,
    pub beacon_interval: u16,
    pub capability: u16,
    pub current_ap: [u8; ETH_ALEN],
    pub listen_interval: u16,
    pub reserved:2: u16 association_id:14,,
    pub __packed: },
    pub time_stamp: [u32; 2],
    pub reason: u16,
    pub status: u16,
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_auth {
    pub header: libipw_hdr_3addr_hdr,
    pub algorithm: __le16,
    pub transaction: __le16,
    pub status: __le16,
// challenge
    pub variable: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_channel_switch {
    pub id: u8,
    pub len: u8,
    pub mode: u8,
    pub channel: u8,
    pub count: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_action {
    pub header: libipw_hdr_3addr_hdr,
    pub category: u8,
    pub action: u8,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_action_exchange {
    pub token: u8,
    pub exchange: },
    pub channel_switch: libipw_channel_switch,
    pub format: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_disassoc {
    pub header: libipw_hdr_3addr_hdr,
    pub reason: __le16,
    pub __packed: },
// Alias deauth for disassoc

#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_probe_request {
    pub header: libipw_hdr_3addr_hdr,
// SSID, supported rates
    pub variable: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_probe_response {
    pub header: libipw_hdr_3addr_hdr,
    pub time_stamp: [__le32; 2],
    pub beacon_interval: __le16,
    pub capability: __le16,
// SSID, supported rates, FH params, DS params,
// CF params, IBSS params, TIM (if beacon), RSN
    pub variable: [u8; ],
    pub __packed: },
// Alias beacon for probe_response

#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_reassoc_request {
    pub header: libipw_hdr_3addr_hdr,
    pub capability: __le16,
    pub listen_interval: __le16,
    pub current_ap: [u8; ETH_ALEN],
    pub variable: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_assoc_response {
    pub header: libipw_hdr_3addr_hdr,
    pub capability: __le16,
    pub status: __le16,
    pub aid: __le16,
// supported rates
    pub variable: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_txb {
    pub nr_frags: u8,
    pub encrypted: u8,
    pub rts_included: u8,
    pub reserved: u8,
    pub frag_size: u16,
    pub payload_size: u16,
    pub __counted_by(nr_frags): *mut *mut sk_buff fragments[],
}

// SWEEP TABLE ENTRIES NUMBER
pub const MAX_SWEEP_TAB_ENTRIES: c_int = 42;
pub const MAX_SWEEP_TAB_ENTRIES_PER_PACKET: c_int = 7;
// MAX_RATES_LENGTH needs to be 12.  The spec says 8, and many APs
// only use 8, and then use extended rates for the remaining supported
// rates.  Other APs, however, stick all of their supported rates on the
// main rates information element...

pub const MAX_NETWORK_COUNT: c_int = 128;

pub const MAX_WPA_IE_LEN: c_int = 64;

// QoS structure

// 802.11h

pub const QOS_QUEUE_NUM: c_int = 4;
pub const QOS_OUI_LEN: c_int = 3;
pub const QOS_OUI_TYPE: c_int = 2;
pub const QOS_ELEMENT_ID: c_int = 221;
pub const QOS_OUI_INFO_SUB_TYPE: c_int = 0;
pub const QOS_OUI_PARAM_SUB_TYPE: c_int = 1;
pub const QOS_VERSION_1: c_int = 1;
pub const QOS_AIFSN_MIN_VALUE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_qos_information_element {
    pub elementID: u8,
    pub length: u8,
    pub qui: [u8; QOS_OUI_LEN],
    pub qui_type: u8,
    pub qui_subtype: u8,
    pub version: u8,
    pub ac_info: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_qos_ac_parameter {
    pub aci_aifsn: u8,
    pub ecw_min_max: u8,
    pub tx_op_limit: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_qos_parameter_info {
    pub info_element: libipw_qos_information_element,
    pub reserved: u8,
    pub ac_params_record: [libipw_qos_ac_parameter; QOS_QUEUE_NUM],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_qos_parameters {
    pub cw_min: [__le16; QOS_QUEUE_NUM],
    pub cw_max: [__le16; QOS_QUEUE_NUM],
    pub aifs: [u8; QOS_QUEUE_NUM],
    pub flag: [u8; QOS_QUEUE_NUM],
    pub tx_op_limit: [__le16; QOS_QUEUE_NUM],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_qos_data {
    pub parameters: libipw_qos_parameters,
    pub active: c_int,
    pub supported: c_int,
    pub param_count: u8,
    pub old_param_count: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_tim_parameters {
    pub tim_count: u8,
    pub tim_period: u8,
    pub __packed: },
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_tpc_report {
    pub transmit_power: u8,
    pub link_margin: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_channel_map {
    pub channel: u8,
    pub map: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_csa {
    pub mode: u8,
    pub channel: u8,
    pub count: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_quiet {
    pub count: u8,
    pub period: u8,
    pub duration: u8,
    pub offset: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_network {
// These entries are used to identify a unique network
    pub bssid: [u8; ETH_ALEN],
    pub channel: u8,
// Ensure null-terminated for any debug msgs
    pub 1]: u8 ssid[IW_ESSID_MAX_SIZE +,
    pub ssid_len: u8,
    pub qos_data: libipw_qos_data,
// These are network statistics
    pub stats: libipw_rx_stats,
    pub capability: u16,
    pub rates: [u8; MAX_RATES_LENGTH],
    pub rates_len: u8,
    pub rates_ex: [u8; MAX_RATES_EX_LENGTH],
    pub rates_ex_len: u8,
    pub last_scanned: c_ulong,
    pub mode: u8,
    pub flags: u32,
    pub last_associate: u32,
    pub time_stamp: [u32; 2],
    pub beacon_interval: u16,
    pub listen_interval: u16,
    pub atim_window: u16,
    pub erp_value: u8,
    pub wpa_ie: [u8; MAX_WPA_IE_LEN],
    pub wpa_ie_len: usize,
    pub rsn_ie: [u8; MAX_WPA_IE_LEN],
    pub rsn_ie_len: usize,
    pub tim: libipw_tim_parameters,
// 802.11h info
// Power Constraint - mandatory if spctrm mgmt required
    pub power_constraint: u8,
// TPC Report - mandatory if spctrm mgmt required
    pub tpc_report: libipw_tpc_report,
// Channel Switch Announcement - optional if spctrm mgmt required
    pub csa: libipw_csa,
// Quiet - optional if spctrm mgmt required
    pub quiet: libipw_quiet,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum libipw_state {
    LIBIPW_UNINITIALIZED = 0,
    LIBIPW_INITIALIZED,
    LIBIPW_ASSOCIATING,
    LIBIPW_ASSOCIATED,
    LIBIPW_AUTHENTICATING,
    LIBIPW_AUTHENTICATED,
    LIBIPW_SHUTDOWN
}

pub const DEFAULT_FTS: c_int = 2346;

pub const LIBIPW_24GHZ_MIN_CHANNEL: c_int = 1;
pub const LIBIPW_24GHZ_MAX_CHANNEL: c_int = 14;

pub const LIBIPW_52GHZ_MIN_CHANNEL: c_int = 34;
pub const LIBIPW_52GHZ_MAX_CHANNEL: c_int = 165;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_channel {
    pub /: *mut *mut u32 freq; / in MHz,
    pub channel: u8,
    pub flags: u8,
    pub /: *mut *mut u8 max_power; / in dBm,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_geo {
    pub name: [u8; 4],
    pub bg_channels: u8,
    pub a_channels: u8,
    pub bg: [libipw_channel; LIBIPW_24GHZ_CHANNELS],
    pub a: [libipw_channel; LIBIPW_52GHZ_CHANNELS],
}

pub const NUM_WEP_KEYS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_crypto_ops {
    pub name: *const c_char,
    pub list: list_head,
// init new crypto context (e.g., allocate private data space,
// select IV, etc.); returns NULL on failure or pointer to allocated
// private data on success
    pub keyidx): *mut *mut *mut void (init) (int,
// deinitialize crypto context and free allocated private data
    pub priv): *mut *mut void (deinit) (void,
// encrypt/decrypt return < 0 on error or >= 0 on success. The return
// value from decrypt_mpdu is passed as the keyidx value for
// decrypt_msdu. skb must have enough head and tail room for the
// encryption; if not, error will be returned; these functions are
// called for all MPDUs (i.e., fragments).
//
    pub priv): *mut *mut *mut int (encrypt_mpdu) (struct sk_buff  skb, int hdr_len, void,
    pub priv): *mut *mut *mut int (decrypt_mpdu) (struct sk_buff  skb, int hdr_len, void,
// These functions are called for full MSDUs, i.e. full frames.
// These can be NULL if full MSDU operations are not needed.
    pub priv): *mut *mut *mut int (encrypt_msdu) (struct sk_buff  skb, int hdr_len, void,
    pub priv): *mut c_void,
    pub priv): *mut *mut *mut *mut int (set_key) (void key, int len, u8  seq, void,
    pub priv): *mut *mut *mut *mut int (get_key) (void key, int len, u8  seq, void,
// procfs handler for printing out key information and possible
// statistics
    pub priv): *mut *mut *mut void (print_stats) (struct seq_file m, void,
// Crypto specific flag get/set for configuration settings
    pub priv): *mut *mut unsigned long (get_flags) (void,
    pub priv): *mut *mut unsigned long (set_flags) (unsigned long flags, void,
// maximum number of bytes added by encryption; encrypt buf is
// allocated with extra_prefix_len bytes, copy of in_buf, and
// extra_postfix_len; encrypt need not use all this space, but
// the result must start at the beginning of the buffer and correct
// length must be returned
    pub extra_mpdu_postfix_len: int extra_mpdu_prefix_len,,
    pub extra_msdu_postfix_len: int extra_msdu_prefix_len,,
    pub owner: *mut module,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_crypt_info {
    pub name: *mut c_char,
// Most clients will already have a lock,
    pub lock: *mut spinlock_t,
    pub crypt: [*mut libipw_crypt_data; NUM_WEP_KEYS],
    pub /: *mut *mut int tx_keyidx; / default TX key index (crypt[tx_keyidx]),
    pub crypt_deinit_list: list_head,
    pub crypt_deinit_timer: timer_list,
    pub crypt_quiesced: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_device {
    pub dev: *mut net_device,
    pub wdev: wireless_dev,
    pub sec: libipw_security,
// Bookkeeping structures
    pub ieee_stats: libipw_stats,
    pub geo: libipw_geo,
    pub bg_band: ieee80211_supported_band,
    pub a_band: ieee80211_supported_band,
// Probe / Beacon management
    pub network_free_list: list_head,
    pub network_list: list_head,
    pub networks: [*mut libipw_network; MAX_NETWORK_COUNT],
    pub scans: c_int,
    pub scan_age: c_int,
    pub /: *mut *mut *mut int iw_mode; / operating mode (IW_MODE_),
    pub /: *mut *mut iw_spy_data spy_data; / iwspy support,
    pub spy_enabled: bool,
    pub lock: spinlock_t,
    pub front: *mut *mut int tx_headroom; / Set to size of any additional room needed at,
// of allocated Tx SKBs
    pub config: u32,
// WEP and other encryption related settings at the device level
    pub /: *mut *mut int open_wep; / Set to 1 to allow unencrypted frames,
// If the host performs {en,de}cryption, then set to 1
    pub host_encrypt: c_int,
    pub host_encrypt_msdu: c_int,
    pub host_decrypt: c_int,
// host performs multicast decryption
    pub host_mc_decrypt: c_int,
// host should strip IV and ICV from protected frames
// meaningful only when hardware decryption is being used
    pub host_strip_iv_icv: c_int,
    pub host_open_frag: c_int,
    pub /: *mut *mut int ieee802_1x; / is IEEE 802.1X used,
// WPA data
    pub wpa_enabled: c_int,
    pub drop_unencrypted: c_int,
    pub privacy_invoked: c_int,
    pub wpa_ie_len: usize,
    pub wpa_ie: *mut u8,
    pub crypt_info: libipw_crypt_info,
    pub even: *mut *mut int bcrx_sta_key; / use individual keys to override default keys,
// with RX of broad/multicast frames
// Fragmentation structures
    pub frag_cache: [libipw_frag_entry; LIBIPW_FRAG_CACHE_LEN],
    pub frag_next_idx: c_uint,
    pub /: *mut *mut u16 fts; / Fragmentation Threshold,
    pub /: *mut *mut u16 rts; / RTS threshold,
// Association info
    pub bssid: [u8; ETH_ALEN],
    pub state: libipw_state,
    pub /: *mut *mut int mode; / A, B, G,
    pub /: *mut *mut int modulation; / CCK, OFDM,
    pub /: *mut *mut int freq_band; / 2.4Ghz, 5.2Ghz, Mixed,
    pub /: *mut *mut int abg_true; / ABG flag,
    pub perfect_rssi: c_int,
    pub worst_rssi: c_int,
    pub /: *mut *mut u16 prev_seq_ctl; / used to drop duplicate frames,
// Callback functions
    pub sec): *mut *mut libipw_security,
    pub pri): *mut *mut net_device  dev, int,
    pub pri): *mut *mut *mut int (is_queue_full) (struct net_device  dev, int,
    pub type): *mut *mut libipw_network  network, u16,
    pub skb): *mut *mut *mut int (is_qos_active) (struct net_device dev, struct sk_buff,
// Typical STA methods
    pub auth): *mut *mut libipw_auth,
    pub auth): *mut *mut libipw_deauth,
    pub stats): *mut *mut libipw_rx_stats,
    pub assoc): *mut *mut libipw_disassoc,
    pub network): *mut *mut libipw_network,
    pub network): *mut *mut libipw_network,
    pub stats): *mut *mut libipw_rx_stats,
    pub network): *mut *mut libipw_network,
// Typical AP methods
    pub dev): *mut *mut *mut int (handle_assoc_request) (struct net_device,
    pub req): *mut *mut libipw_reassoc_request,
// This must be the last item so that it points to the data
// allocated beyond this structure by alloc_libipw
    pub priv: [u8; ],
}

//
// It is possible for both access points and our device to support
// combinations of modes, so as long as there is one valid combination
// of ap/device supported modes, then return success
//
// libipw.c
extern "C" {
    pub fn free_libipw(dev: *mut net_device, monitor: c_int);
}
extern "C" {
    pub fn libipw_networks_age(ieee: *mut libipw_device, age_secs: c_ulong);
}
extern "C" {
    pub fn libipw_set_encryption(ieee: *mut libipw_device) -> c_int;
}
// libipw_tx.c
extern "C" {
    pub fn libipw_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t;
}
extern "C" {
    pub fn libipw_txb_free(: *mut libipw_txb);
}
// libipw_rx.c
// make sure to set stats->len
// libipw_geo.c
extern "C" {
    pub fn libipw_set_geo(ieee: *mut libipw_device, geo: *const libipw_geo);
}
extern "C" {
    pub fn libipw_is_valid_channel(ieee: *mut libipw_device, channel: u8) -> c_int;
}
extern "C" {
    pub fn libipw_channel_to_index(ieee: *mut libipw_device, channel: u8) -> c_int;
}
extern "C" {
    pub fn libipw_freq_to_channel(ieee: *mut libipw_device, freq: u32) -> u8;
}
extern "C" {
    pub fn libipw_get_channel_flags(ieee: *mut libipw_device, channel: u8) -> u8;
}
extern "C" {
    pub fn libipw_channel_to_freq(ieee: *mut libipw_device, channel: u8) -> u32;
}
// libipw_wx.c
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_crypt_data {
    pub /: *mut *mut list_head list; / delayed deletion list,
    pub ops: *const libipw_crypto_ops,
    pub priv: *mut c_void,
    pub refcnt: core::sync::atomic::AtomicI32,
}

extern "C" {
    pub fn libipw_crypt_info_free(info: *mut libipw_crypt_info);
}
extern "C" {
    pub fn libipw_register_crypto_ops(ops: *const libipw_crypto_ops) -> c_int;
}
extern "C" {
    pub fn libipw_unregister_crypto_ops(ops: *const libipw_crypto_ops) -> c_int;
}
// must be called in the listed order
extern "C" {
    pub fn libipw_crypto_init() -> c_int;
}
extern "C" {
    pub fn libipw_crypto_ccmp_init() -> c_int;
}
extern "C" {
    pub fn libipw_crypto_tkip_init() -> c_int;
}
extern "C" {
    pub fn libipw_crypto_wep_init() -> c_int;
}
extern "C" {
    pub fn libipw_crypto_wep_exit();
}
extern "C" {
    pub fn libipw_crypto_tkip_exit();
}
extern "C" {
    pub fn libipw_crypto_ccmp_exit();
}
extern "C" {
    pub fn libipw_crypto_exit();
}
