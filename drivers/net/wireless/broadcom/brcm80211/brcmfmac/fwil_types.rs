//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmfmac/fwil_types.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2012 Broadcom Corporation
//

pub const BRCMF_FIL_ACTION_FRAME_SIZE: c_int = 1800;
// ARP Offload feature flags for arp_ol iovar
pub const BRCMF_ARP_OL_AGENT: c_uint = 0x00000001;
pub const BRCMF_ARP_OL_SNOOP: c_uint = 0x00000002;
pub const BRCMF_ARP_OL_HOST_AUTO_REPLY: c_uint = 0x00000004;
pub const BRCMF_ARP_OL_PEER_AUTO_REPLY: c_uint = 0x00000008;

pub const BRCMF_BSS_RSSI_ON_CHANNEL: c_uint = 0x0004;
pub const BRCMF_STA_BRCM: c_uint = 0x00000001	/* Running a Broadcom driver */;
pub const BRCMF_STA_WME: c_uint = 0x00000002	/* WMM association */;
pub const BRCMF_STA_NONERP: c_uint = 0x00000004	/* No ERP */;
pub const BRCMF_STA_AUTHE: c_uint = 0x00000008	/* Authenticated */;
pub const BRCMF_STA_ASSOC: c_uint = 0x00000010	/* Associated */;
pub const BRCMF_STA_AUTHO: c_uint = 0x00000020	/* Authorized */;
pub const BRCMF_STA_WDS: c_uint = 0x00000040	/* Wireless Distribution System */;
pub const BRCMF_STA_WDS_LINKUP: c_uint = 0x00000080	/* WDS traffic/probes flowing properly */;
pub const BRCMF_STA_PS: c_uint = 0x00000100	/* STA is in power save mode from AP's viewpoint */;
pub const BRCMF_STA_APSD_BE: c_uint = 0x00000200	/* APSD delv/trigger for AC_BE is default enabled */;
pub const BRCMF_STA_APSD_BK: c_uint = 0x00000400	/* APSD delv/trigger for AC_BK is default enabled */;
pub const BRCMF_STA_APSD_VI: c_uint = 0x00000800	/* APSD delv/trigger for AC_VI is default enabled */;
pub const BRCMF_STA_APSD_VO: c_uint = 0x00001000	/* APSD delv/trigger for AC_VO is default enabled */;
pub const BRCMF_STA_N_CAP: c_uint = 0x00002000	/* STA 802.11n capable */;
pub const BRCMF_STA_SCBSTATS: c_uint = 0x00004000	/* Per STA debug stats */;
pub const BRCMF_STA_AMPDU_CAP: c_uint = 0x00008000	/* STA AMPDU capable */;
pub const BRCMF_STA_AMSDU_CAP: c_uint = 0x00010000	/* STA AMSDU capable */;
pub const BRCMF_STA_MIMO_PS: c_uint = 0x00020000	/* mimo ps mode is enabled */;
pub const BRCMF_STA_MIMO_RTS: c_uint = 0x00040000	/* send rts in mimo ps mode */;
pub const BRCMF_STA_RIFS_CAP: c_uint = 0x00080000	/* rifs enabled */;
pub const BRCMF_STA_VHT_CAP: c_uint = 0x00100000	/* STA VHT(11ac) capable */;
pub const BRCMF_STA_WPS: c_uint = 0x00200000	/* WPS state */;
pub const BRCMF_STA_DWDS_CAP: c_uint = 0x01000000	/* DWDS CAP */;
pub const BRCMF_STA_DWDS: c_uint = 0x02000000	/* DWDS active */;
// size of brcmf_scan_params not including variable length array
pub const BRCMF_SCAN_PARAMS_FIXED_SIZE: c_int = 64;
pub const BRCMF_SCAN_PARAMS_V2_FIXED_SIZE: c_int = 72;
// version of brcmf_scan_params structure
pub const BRCMF_SCAN_PARAMS_VERSION_V2: c_int = 2;
// masks for channel and ssid count
pub const BRCMF_SCAN_PARAMS_COUNT_MASK: c_uint = 0x0000ffff;
pub const BRCMF_SCAN_PARAMS_NSSID_SHIFT: c_int = 16;
// scan type definitions
pub const BRCMF_SCANTYPE_DEFAULT: c_uint = 0xFF;
pub const BRCMF_SCANTYPE_ACTIVE: c_int = 0;
pub const BRCMF_SCANTYPE_PASSIVE: c_int = 1;
pub const BRCMF_WSEC_MAX_PSK_LEN: c_int = 32;

pub const BRCMF_WSEC_MAX_SAE_PASSWORD_LEN: c_int = 128;
// primary (ie tx) key

pub const DOT11_BSSTYPE_ANY: c_int = 2;
pub const BRCMF_ESCAN_REQ_VERSION: c_int = 1;
pub const BRCMF_ESCAN_REQ_VERSION_V2: c_int = 2;

// OBSS Coex Auto/On/Off

pub const BRCMF_OBSS_COEX_OFF: c_int = 0;
pub const BRCMF_OBSS_COEX_ON: c_int = 1;
// WOWL bits
// Wakeup on Magic packet:

// Wakeup on Netpattern

// Wakeup on loss-of-link due to Disassoc/Deauth:

// Wakeup on retrograde TSF:

// Wakeup on loss of beacon:

// Wakeup after test:

// Wakeup after PTK refresh:

// Wakeup after receipt of EAP-Identity Req:

// Wakeind via PME(0) or GPIO(1):

// need tkip phase 1 key to be updated by the driver:

// enable wakeup if GTK fails:

// support extended magic packets:

// support ARP/NS/keepalive offloading:

// read protocol version for EAPOL frames:

// If the bit is set, use key rotaton:

// If the bit is set, frm received was bcast frame:

// If the bit is set, scan offload is enabled:

// Wakeup on tcpkeep alive timeout:

// Wakeup on mDNS Conflict Resolution:

// Wakeup on mDNS Service Connect:

// tcp keepalive got data:

// Firmware died in wowl mode:

// Enable detection of radio button changes:

// Offloads detected MIC failure(s):

// Wakeup in Unassociated state (Net/Magic Pattern):

// Wakeup if received matched secured pattern:

// Wakeup on finding preferred network

// Wakeup on receiving pairwise key EAP packets:

// Link Down indication in WoWL mode:

pub const BRCMF_WOWL_MAXPATTERNS: c_int = 16;
pub const BRCMF_WOWL_MAXPATTERNSIZE: c_int = 128;
pub const BRCMF_COUNTRY_BUF_SZ: c_int = 4;
pub const BRCMF_ANT_MAX: c_int = 4;
pub const BRCMF_MAX_ASSOCLIST: c_int = 128;

pub const BRCMF_NUMCHANNELS: c_int = 64;
pub const BRCMF_PFN_MACADDR_CFG_VER: c_int = 1;

pub const BRCMF_MCSSET_LEN: c_int = 16;
pub const BRCMF_RSN_KCK_LENGTH: c_int = 16;
pub const BRCMF_RSN_KEK_LENGTH: c_int = 16;
pub const BRCMF_RSN_REPLAY_LEN: c_int = 8;
pub const BRCMF_MFP_NONE: c_int = 0;
pub const BRCMF_MFP_CAPABLE: c_int = 1;
pub const BRCMF_MFP_REQUIRED: c_int = 2;
pub const BRCMF_VHT_CAP_MCS_MAP_NSS_MAX: c_int = 8;
pub const BRCMF_HE_CAP_MCS_MAP_NSS_MAX: c_int = 8;
pub const BRCMF_PMKSA_VER_2: c_int = 2;
pub const BRCMF_PMKSA_VER_3: c_int = 3;
pub const BRCMF_PMKSA_NO_EXPIRY: c_uint = 0xffffffff;
// MAX_CHUNK_LEN is the maximum length for data passing to firmware in each
// ioctl. It is relatively small because firmware has small maximum size input
// playload restriction for ioctls.
//
pub const MAX_CHUNK_LEN: c_int = 1400;

pub const DLOAD_FLAG_VER_MASK: c_uint = 0xf000	/* Downloader version mask */;

pub const DL_BEGIN: c_uint = 0x0002;
pub const DL_END: c_uint = 0x0004;
pub const DL_TYPE_CLM: c_int = 2;
// join preference types for join_pref iovar
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum brcmf_join_pref_types {
    BRCMF_JOIN_PREF_RSSI = 1,
    BRCMF_JOIN_PREF_WPA,
    BRCMF_JOIN_PREF_BAND,
    BRCMF_JOIN_PREF_RSSI_DELTA,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum brcmf_fil_p2p_if_types {
    BRCMF_FIL_P2P_IF_CLIENT,
    BRCMF_FIL_P2P_IF_GO,
    BRCMF_FIL_P2P_IF_DYNBCN_GO,
    BRCMF_FIL_P2P_IF_DEV,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum brcmf_wowl_pattern_type {
    BRCMF_WOWL_PATTERN_TYPE_BITMAP = 0,
    BRCMF_WOWL_PATTERN_TYPE_ARP,
    BRCMF_WOWL_PATTERN_TYPE_NA
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_fil_p2p_if_le {
    pub addr: [u8; ETH_ALEN],
    pub type: __le16,
    pub chspec: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_fil_chan_info_le {
    pub hw_channel: __le32,
    pub target_channel: __le32,
    pub scan_channel: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_fil_action_frame_le {
    pub da: [u8; ETH_ALEN],
    pub len: __le16,
    pub packet_id: __le32,
    pub data: [u8; BRCMF_FIL_ACTION_FRAME_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_fil_af_params_le {
    pub channel: __le32,
    pub dwell_time: __le32,
    pub bssid: [u8; ETH_ALEN],
    pub pad: [u8; 2],
    pub action_frame: brcmf_fil_action_frame_le,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_fil_bss_enable_le {
    pub bsscfgidx: __le32,
    pub enable: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_fil_bwcap_le {
    pub band: __le32,
    pub bw_cap: __le32,
}

//
// struct tdls_iovar - common structure for tdls iovars.
//
// @ea: ether address of peer station.
// @mode: mode value depending on specific tdls iovar.
// @chanspec: channel specification.
// @pad: unused (for future use).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_tdls_iovar_le {
    pub /: *mut *mut u8 ea[ETH_ALEN]; / Station address,
    pub /: *mut *mut u8 mode; / mode: depends on iovar,
    pub chanspec: __le16,
    pub /: *mut *mut __le32 pad; / future,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum brcmf_tdls_manual_ep_ops {
    BRCMF_TDLS_MANUAL_EP_CREATE = 1,
    BRCMF_TDLS_MANUAL_EP_DELETE = 3,
    BRCMF_TDLS_MANUAL_EP_DISCOVERY = 6
}

// Pattern matching filter. Specifies an offset within received packets to
// start matching, the pattern to match, the size of the pattern, and a bitmask
// that indicates which bits within the pattern should be matched.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_pkt_filter_pattern_le {
//
// Offset within received packet to start pattern matching.
// Offset '0' is the first byte of the ethernet header.
//
    pub offset: __le32,
// Size of the pattern.  Bitmask must be the same size.
    pub size_bytes: __le32,
//
// Variable length mask and pattern data. mask starts at offset 0.
// Pattern immediately follows mask.
//
    pub mask_and_pattern: [u8; 1],
}

// IOVAR "pkt_filter_add" parameter. Used to install packet filters.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_pkt_filter_le {
    pub /: *mut *mut __le32 id; / Unique filter id, specified by app.,
    pub /: *mut *mut __le32 type; / Filter type (WL_PKT_FILTER_TYPE_xxx).,
    pub /: *mut *mut __le32 negate_match; / Negate the result of filter matches,
    pub /: *mut *mut brcmf_pkt_filter_pattern_le pattern; / Filter pattern,
    pub u: },
}

// IOVAR "pkt_filter_enable" parameter.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_pkt_filter_enable_le {
    pub /: *mut *mut __le32 id; / Unique filter id,
    pub /: *mut *mut __le32 enable; / Enable/disable bool,
}

// BSS info structure
// Applications MUST CHECK ie_offset field and length field to access IEs and
// next bss_info structure in a vector (in struct brcmf_scan_results)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_bss_info_le {
    pub /: *mut *mut __le32 version; / version field,
    pub record,: *mut *mut __le32 length; / byte length of data in this,
// starting at version and including IEs
//
    pub BSSID: [u8; ETH_ALEN],
    pub /: *mut *mut __le16 beacon_period; / units are Kusec,
    pub /: *mut *mut __le16 capability; / Capability information,
    pub SSID_len: u8,
    pub SSID: [u8; 32],
    pub /: *mut *mut __le32 count; / # rates in this set,
    pub /: *mut *mut u8 rates[16]; / rates in 500kbps units w/hi bit set if basic,
    pub /: *mut *mut } rateset; / supported rates,
    pub /: *mut *mut __le16 chanspec; / chanspec for bss,
    pub /: *mut *mut __le16 atim_window; / units are Kusec,
    pub /: *mut *mut u8 dtim_period; / DTIM period,
    pub /: *mut *mut __le16 RSSI; / receive signal strength (in dBm),
    pub /: *mut *mut s8 phy_noise; / noise (in dBm),
    pub /: *mut *mut u8 n_cap; / BSS is 802.11N Capable,
// 802.11N BSS Capabilities (based on HT_CAP_*):
    pub nbss_cap: __le32,
    pub /: *mut *mut u8 ctl_ch; / 802.11N BSS control channel number,
    pub /: *mut *mut __le32 reserved32[1]; / Reserved for expansion of BSS properties,
    pub /: *mut *mut u8 flags; / flags,
    pub /: *mut *mut u8 reserved[3]; / Reserved for expansion of BSS properties,
    pub /: *mut *mut u8 basic_mcs[BRCMF_MCSSET_LEN]; / 802.11N BSS required MCS set,
    pub /: *mut *mut __le16 ie_offset; / offset at which IEs start, from beginning,
    pub /: *mut *mut __le32 ie_length; / byte length of Information Elements,
    pub /: *mut *mut __le16 SNR; / average SNR of during frame reception,
// Add new fields here
// variable length Information Elements
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcm_rateset_le {
// # rates in this set
    pub count: __le32,
// rates in 500kbps units w/hi bit set if basic
    pub rates: [u8; BRCMF_MAXRATES_IN_SET],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_ssid_le {
    pub SSID_len: __le32,
    pub SSID: [c_uchar; IEEE80211_MAX_SSID_LEN],
}

// Alternate SSID structure used in some places...
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_ssid8_le {
    pub SSID_len: u8,
    pub SSID: [c_uchar; IEEE80211_MAX_SSID_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_scan_params_le {
    pub /: *mut *mut brcmf_ssid_le ssid_le; / default: {0, ""},
    pub /: *mut *mut u8 bssid[ETH_ALEN]; / default: bcast,
    pub any,: *mut *mut s8 bss_type; / default:,
// DOT11_BSSTYPE_ANY/INFRASTRUCTURE/INDEPENDENT
//
    pub /: *mut *mut u8 scan_type; / flags, 0 use default,
    pub /: *mut *mut __le32 nprobes; / -1 use default, number of probes per channel,
    pub for: *mut *mut __le32 active_time; / -1 use default, dwell time per channel,
// active scanning
//
    pub channel: *mut *mut __le32 passive_time; / -1 use default, dwell time per,
// for passive scanning
//
    pub the: *mut *mut __le32 home_time; / -1 use default, dwell time for,
// home channel between channel scans
//
    pub follow: *mut *mut __le32 channel_num; / count of channels and ssids that,
//
// low half is count of channels in
// channel_list, 0 means default (use all
// available channels)
//
// high half is entries in struct brcmf_ssid
// array that follows channel_list, aligned for
// s32 (4 bytes) meaning an odd channel count
// implies a 2-byte pad between end of
// channel_list and first ssid
//
// if ssid count is zero, single ssid in the
// fixed parameter portion is assumed, otherwise
// ssid in the fixed portion is ignored
//
    pub abort: *mut *mut __le16 padding; / Reserve space for at least 1 entry for,
// which uses an on stack brcmf_scan_params_le
//
    pub /: *mut *mut DECLARE_FLEX_ARRAY(__le16, channel_list); / chanspecs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_scan_params_v2_le {
    pub /: *mut *mut __le16 version; / structure version,
    pub /: *mut *mut __le16 length; / structure length,
    pub /: *mut *mut brcmf_ssid_le ssid_le; / default: {0, ""},
    pub /: *mut *mut u8 bssid[ETH_ALEN]; / default: bcast,
    pub any,: *mut *mut s8 bss_type; / default:,
// DOT11_BSSTYPE_ANY/INFRASTRUCTURE/INDEPENDENT
//
    pub pad: u8,
    pub /: *mut *mut __le32 scan_type; / flags, 0 use default,
    pub /: *mut *mut __le32 nprobes; / -1 use default, number of probes per channel,
    pub for: *mut *mut __le32 active_time; / -1 use default, dwell time per channel,
// active scanning
//
    pub channel: *mut *mut __le32 passive_time; / -1 use default, dwell time per,
// for passive scanning
//
    pub the: *mut *mut __le32 home_time; / -1 use default, dwell time for,
// home channel between channel scans
//
    pub follow: *mut *mut __le32 channel_num; / count of channels and ssids that,
//
// low half is count of channels in
// channel_list, 0 means default (use all
// available channels)
//
// high half is entries in struct brcmf_ssid
// array that follows channel_list, aligned for
// s32 (4 bytes) meaning an odd channel count
// implies a 2-byte pad between end of
// channel_list and first ssid
//
// if ssid count is zero, single ssid in the
// fixed parameter portion is assumed, otherwise
// ssid in the fixed portion is ignored
//
    pub abort: *mut *mut __le16 padding; / Reserve space for at least 1 entry for,
// which uses an on stack brcmf_scan_params_v2_le
//
    pub /: *mut *mut DECLARE_FLEX_ARRAY(__le16, channel_list); / chanspecs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_scan_results {
    pub buflen: u32,
    pub version: u32,
    pub count: u32,
    pub bss_info_le: [brcmf_bss_info_le; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_escan_params_le {
    pub version: __le32,
    pub action: __le16,
    pub sync_id: __le16,
    pub params_le: brcmf_scan_params_le,
    pub params_v2_le: brcmf_scan_params_v2_le,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_escan_result_le {
    pub buflen: __le32,
    pub version: __le32,
    pub sync_id: __le16,
    pub bss_count: __le16,
    pub bss_info_le: brcmf_bss_info_le,
}

// used for association with a specific BSSID and chanspec list
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_assoc_params_le {
// 00:00:00:00:00:00: broadcast scan
    pub bssid: [u8; ETH_ALEN],
// 0: all available channels, otherwise count of chanspecs in
// chanspec_list
    pub chanspec_num: __le32,
// list of chanspecs
    pub chanspec_list: [__le16; 1],
}

//
// struct join_pref params - parameters for preferred join selection.
//
// @type: preference type (see enum brcmf_join_pref_types).
// @len: length of bytes following (currently always 2).
// @rssi_gain: signal gain for selection (only when @type is RSSI_DELTA).
// @band: band to which selection preference applies.
// This is used if @type is BAND or RSSI_DELTA.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_join_pref_params {
    pub type: u8,
    pub len: u8,
    pub rssi_gain: u8,
    pub band: u8,
}

// used for join with or without a specific bssid and channel list
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_join_params {
    pub ssid_le: brcmf_ssid_le,
    pub params_le: brcmf_assoc_params_le,
}

// scan params for extended join
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_join_scan_params_le {
    pub /: *mut *mut u8 scan_type; / 0 use default, active or passive scan,
    pub /: *mut *mut __le32 nprobes; / -1 use default, nr of probes per channel,
    pub for: *mut *mut __le32 active_time; / -1 use default, dwell time per channel,
// active scanning
//
    pub channel: *mut *mut __le32 passive_time; / -1 use default, dwell time per,
// for passive scanning
//
    pub home: *mut *mut __le32 home_time; / -1 use default, dwell time for the,
// channel between channel scans
//
}

// extended join params
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_ext_join_params_le {
    pub /: *mut *mut brcmf_ssid_le ssid_le; / {0, ""}: wildcard scan,
    pub scan_le: brcmf_join_scan_params_le,
    pub assoc_le: brcmf_assoc_params_le,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_wsec_key {
    pub /: *mut *mut u32 index; / key index,
    pub /: *mut *mut u32 len; / key length,
    pub /: *mut *mut u8 data[WLAN_MAX_KEY_LEN]; / key data,
    pub pad_1: [u32; 18],
    pub /: *mut *mut u32 algo; / CRYPTO_ALGO_AES_CCM, CRYPTO_ALGO_WEP128, etc,
    pub /: *mut *mut u32 flags; / misc flags,
    pub pad_2: [u32; 3],
    pub /: *mut *mut u32 iv_initialized; / has IV been initialized already?,
    pub pad_3: u32,
// Rx IV
    pub /: *mut *mut u32 hi; / upper 32 bits of IV,
    pub /: *mut *mut u16 lo; / lower 16 bits of IV,
    pub rxiv: },
    pub pad_4: [u32; 2],
    pub /: *mut *mut u8 ea[ETH_ALEN]; / per station,
}

//
// dongle requires same struct as above but with fields in little endian order
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_wsec_key_le {
    pub /: *mut *mut __le32 index; / key index,
    pub /: *mut *mut __le32 len; / key length,
    pub /: *mut *mut u8 data[WLAN_MAX_KEY_LEN]; / key data,
    pub pad_1: [__le32; 18],
    pub /: *mut *mut __le32 algo; / CRYPTO_ALGO_AES_CCM, CRYPTO_ALGO_WEP128, etc,
    pub /: *mut *mut __le32 flags; / misc flags,
    pub pad_2: [__le32; 3],
    pub /: *mut *mut __le32 iv_initialized; / has IV been initialized already?,
    pub pad_3: __le32,
// Rx IV
    pub /: *mut *mut __le32 hi; / upper 32 bits of IV,
    pub /: *mut *mut __le16 lo; / lower 16 bits of IV,
    pub rxiv: },
    pub pad_4: [__le32; 2],
    pub /: *mut *mut u8 ea[ETH_ALEN]; / per station,
}

//
// struct brcmf_wsec_pmk_le - firmware pmk material.
//
// @key_len: number of octets in key material.
// @flags: key handling qualifiers.
// @key: PMK key material.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_wsec_pmk_le {
    pub key_len: __le16,
    pub flags: __le16,
    pub key: [u8; BRCMF_WSEC_MAX_SAE_PASSWORD_LEN],
}

//
// struct brcmf_wsec_sae_pwd_le - firmware SAE password material.
//
// @key_len: number of octets in key materials.
// @key: SAE password material.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_wsec_sae_pwd_le {
    pub key_len: __le16,
    pub key: [u8; BRCMF_WSEC_MAX_SAE_PASSWORD_LEN],
}

// Used to get specific STA parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_scb_val_le {
    pub val: __le32,
    pub ea: [u8; ETH_ALEN],
}

// channel encoding
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_channel_info_le {
    pub hw_channel: __le32,
    pub target_channel: __le32,
    pub scan_channel: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_sta_info_le {
    pub /: *mut *mut __le16 ver; / version of this struct,
    pub /: *mut *mut __le16 len; / length in bytes of this structure,
    pub /: *mut *mut __le16 cap; / sta's advertised capabilities,
    pub /: *mut *mut __le32 flags; / flags defined below,
    pub /: *mut *mut __le32 idle; / time since data pkt rx'd from sta,
    pub /: *mut *mut u8 ea[ETH_ALEN]; / Station address,
    pub /: *mut *mut __le32 count; / # rates in this set,
    pub /: *mut *mut u8 rates[BRCMF_MAXRATES_IN_SET]; / rates in 500kbps units,
// w/hi bit set if basic
    pub /: *mut *mut __le32 in; / seconds elapsed since associated,
    pub /: *mut *mut __le32 listen_interval_inms; / Min Listen interval in ms for STA,
// Fields valid for ver >= 3
    pub /: *mut *mut __le32 tx_pkts; / # of packets transmitted,
    pub /: *mut *mut __le32 tx_failures; / # of packets failed,
    pub /: *mut *mut __le32 rx_ucast_pkts; / # of unicast packets received,
    pub /: *mut *mut __le32 rx_mcast_pkts; / # of multicast packets received,
    pub /: *mut *mut __le32 tx_rate; / Rate of last successful tx frame,
    pub /: *mut *mut __le32 rx_rate; / Rate of last successful rx frame,
    pub /: *mut *mut __le32 rx_decrypt_succeeds; / # of packet decrypted successfully,
    pub /: *mut *mut __le32 rx_decrypt_failures; / # of packet decrypted failed,
// Fields valid for ver >= 4
    pub /: *mut *mut __le32 tx_tot_pkts; / # of tx pkts (ucast + mcast),
    pub /: *mut *mut __le32 rx_tot_pkts; / # of data packets recvd (uni + mcast),
    pub /: *mut *mut __le32 tx_mcast_pkts; / # of mcast pkts txed,
    pub /: *mut *mut __le64 tx_tot_bytes; / data bytes txed (ucast + mcast),
    pub /: *mut *mut __le64 rx_tot_bytes; / data bytes recvd (ucast + mcast),
    pub /: *mut *mut __le64 tx_ucast_bytes; / data bytes txed (ucast),
    pub /: *mut *mut __le64 tx_mcast_bytes; / # data bytes txed (mcast),
    pub /: *mut *mut __le64 rx_ucast_bytes; / data bytes recvd (ucast),
    pub /: *mut *mut __le64 rx_mcast_bytes; / data bytes recvd (mcast),
    pub /: *mut *mut s8 rssi[BRCMF_ANT_MAX]; / per antenna rssi,
    pub /: *mut *mut s8 nf[BRCMF_ANT_MAX]; / per antenna noise floor,
    pub /: *mut *mut __le16 aid; / association ID,
    pub /: *mut *mut __le16 ht_capabilities; / advertised ht caps,
    pub /: *mut *mut __le16 vht_flags; / converted vht flags,
    pub was: *mut *mut __le32 tx_pkts_retry_cnt; / # of frames where a retry,
// exhausted.
//
    pub retry: *mut *mut __le32 tx_pkts_retry_exhausted; / # of user frames where a,
// was exhausted
//
    pub last: *mut *mut s8 rx_lastpkt_rssi[BRCMF_ANT_MAX]; / Per antenna RSSI of,
// received data frame.
//
// TX WLAN retry/failure statistics:
// Separated for host requested frames and locally generated frames.
// Include unicast frame only where the retries/failures can be counted.
//
    pub /: *mut *mut __le32 tx_pkts_total; / # user frames sent successfully,
    pub /: *mut *mut __le32 tx_pkts_retries; / # user frames retries,
    pub /: *mut *mut __le32 tx_pkts_fw_total; / # FW generated sent successfully,
    pub /: *mut *mut __le32 tx_pkts_fw_retries; / # retries for FW generated frames,
    pub retry: *mut *mut __le32 tx_pkts_fw_retry_exhausted; / # FW generated where a,
// was exhausted
//
    pub /: *mut *mut __le32 rx_pkts_retried; / # rx with retry bit set,
    pub /: *mut *mut __le32 tx_rate_fallback; / lowest fallback TX rate,
    pub /: *mut *mut __le32 count; / # rates in this set,
    pub /: *mut *mut u8 rates[BRCMF_MAXRATES_IN_SET]; / rates in 500kbps units w/hi bit set if basic,
    pub /: *mut *mut u8 mcs[BRCMF_MCSSET_LEN]; / supported mcs index bit map,
    pub /: *mut *mut __le16 vht_mcs[BRCMF_VHT_CAP_MCS_MAP_NSS_MAX]; / supported mcs index bit map per nss,
    pub rateset_adv: },
    pub v5: },
    pub /: *mut *mut __le32 rx_dur_total; / total user RX duration (estimated),
    pub /: *mut *mut *mut __le16 chanspec; / chanspec this sta is on,
    pub pad_1: __le16,
    pub /: *mut *mut __le16 version; / version,
    pub /: *mut *mut __le16 len; / length,
    pub /: *mut *mut __le32 count; / # rates in this set,
    pub /: *mut *mut u8 rates[BRCMF_MAXRATES_IN_SET]; / rates in 500kbps units w/hi bit set if basic,
    pub /: *mut *mut u8 mcs[BRCMF_MCSSET_LEN]; / supported mcs index bit map,
    pub /: *mut *mut __le16 vht_mcs[BRCMF_VHT_CAP_MCS_MAP_NSS_MAX]; / supported mcs index bit map per nss,
    pub /: *mut *mut __le16 he_mcs[BRCMF_HE_CAP_MCS_MAP_NSS_MAX]; / supported he mcs index bit map per nss,
    pub /: *mut *mut } rateset_adv; / rateset along with mcs index bitmap,
    pub /: *mut *mut __le16 wpauth; / authentication type,
    pub /: *mut *mut u8 algo; / crypto algorithm,
    pub pad_2: u8,
    pub /: *mut *mut __le32 tx_rspec; / Rate of last successful tx frame,
    pub /: *mut *mut __le32 rx_rspec; / Rate of last successful rx frame,
    pub /: *mut *mut __le32 wnm_cap; / wnm capabilities,
    pub v7: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_chanspec_list {
    pub /: *mut *mut __le32 count; / # of entries,
    pub /: *mut *mut __le32 element[]; / variable length uint32 list,
}

//
// WLC_E_PROBRESP_MSG
// WLC_E_P2P_PROBREQ_MSG
// WLC_E_ACTION_FRAME_RX
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_rx_mgmt_data {
    pub version: __be16,
    pub chanspec: __be16,
    pub rssi: __be32,
    pub mactime: __be32,
    pub rate: __be32,
}

//
// struct brcmf_fil_wowl_pattern_le - wowl pattern configuration struct.
//
// @cmd: "add", "del" or "clr".
// @masksize: Size of the mask in #of bytes
// @offset: Pattern byte offset in packet
// @patternoffset: Offset of start of pattern. Starting from field masksize.
// @patternsize: Size of the pattern itself in #of bytes
// @id: id
// @reasonsize: Size of the wakeup reason code
// @type: Type of pattern (enum brcmf_wowl_pattern_type)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_fil_wowl_pattern_le {
    pub cmd: [u8; 4],
    pub masksize: __le32,
    pub offset: __le32,
    pub patternoffset: __le32,
    pub patternsize: __le32,
    pub id: __le32,
    pub reasonsize: __le32,
    pub type: __le32,
// u8 mask[] - Mask follows the structure above
// u8 pattern[] - Pattern follows the mask is at 'patternoffset'
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_mbss_ssid_le {
    pub bsscfgidx: __le32,
    pub SSID_len: __le32,
    pub SSID: [c_uchar; 32],
}

//
// struct brcmf_fil_country_le - country configuration structure.
//
// @country_abbrev: null-terminated country code used in the country IE.
// @rev: revision specifier for ccode. on set, -1 indicates unspecified.
// @ccode: null-terminated built-in country code.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_fil_country_le {
    pub country_abbrev: [c_char; BRCMF_COUNTRY_BUF_SZ],
    pub rev: __le32,
    pub ccode: [c_char; BRCMF_COUNTRY_BUF_SZ],
}

//
// struct brcmf_rev_info_le - device revision info.
//
// @vendorid: PCI vendor id.
// @deviceid: device id of chip.
// @radiorev: radio revision.
// @chiprev: chip revision.
// @corerev: core revision.
// @boardid: board identifier (usu. PCI sub-device id).
// @boardvendor: board vendor (usu. PCI sub-vendor id).
// @boardrev: board revision.
// @driverrev: driver version.
// @ucoderev: microcode version.
// @bus: bus type.
// @chipnum: chip number.
// @phytype: phy type.
// @phyrev: phy revision.
// @anarev: anacore rev.
// @chippkg: chip package info.
// @nvramrev: nvram revision number.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_rev_info_le {
    pub vendorid: __le32,
    pub deviceid: __le32,
    pub radiorev: __le32,
    pub chiprev: __le32,
    pub corerev: __le32,
    pub boardid: __le32,
    pub boardvendor: __le32,
    pub boardrev: __le32,
    pub driverrev: __le32,
    pub ucoderev: __le32,
    pub bus: __le32,
    pub chipnum: __le32,
    pub phytype: __le32,
    pub phyrev: __le32,
    pub anarev: __le32,
    pub chippkg: __le32,
    pub nvramrev: __le32,
}

//
// struct brcmf_wlc_version_le - firmware revision info.
//
// @version: structure version.
// @length: structure length.
// @epi_ver_major: EPI major version
// @epi_ver_minor: EPI minor version
// @epi_ver_rc: EPI rc version
// @epi_ver_incr: EPI increment version
// @wlc_ver_major: WLC major version
// @wlc_ver_minor: WLC minor version
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_wlc_version_le {
    pub version: __le16,
    pub length: __le16,
    pub epi_ver_major: __le16,
    pub epi_ver_minor: __le16,
    pub epi_ver_rc: __le16,
    pub epi_ver_incr: __le16,
    pub wlc_ver_major: __le16,
    pub wlc_ver_minor: __le16,
}

//
// struct brcmf_assoclist_le - request assoc list.
//
// @count: indicates number of stations.
// @mac: MAC addresses of stations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_assoclist_le {
    pub count: __le32,
    pub mac: [u8; BRCMF_MAX_ASSOCLIST][ETH_ALEN],
}

//
// struct brcmf_rssi_be - RSSI threshold event format
//
// @rssi: receive signal strength (in dBm)
// @snr: signal-noise ratio
// @noise: noise (in dBm)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_rssi_be {
    pub rssi: __be32,
    pub snr: __be32,
    pub noise: __be32,
}

pub const BRCMF_MAX_RSSI_LEVELS: c_int = 8;
//
// struct brcm_rssi_event_le - rssi_event IOVAR format
//
// @rate_limit_msec: RSSI event rate limit
// @rssi_level_num: number of supplied RSSI levels
// @rssi_levels: RSSI levels in ascending order
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_rssi_event_le {
    pub rate_limit_msec: __le32,
    pub rssi_level_num: i8,
    pub rssi_levels: [i8; BRCMF_MAX_RSSI_LEVELS],
}

//
// struct brcmf_wowl_wakeind_le - Wakeup indicators
// Note: note both fields contain same information.
//
// @pci_wakeind: Whether PCI PMECSR PMEStatus bit was set.
// @ucode_wakeind: What wakeup-event indication was set by ucode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_wowl_wakeind_le {
    pub pci_wakeind: __le32,
    pub ucode_wakeind: __le32,
}

//
// struct brcmf_pmksa - PMK Security Association
//
// @bssid: The AP's BSSID.
// @pmkid: he PMK material itself.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_pmksa {
    pub bssid: [u8; ETH_ALEN],
    pub pmkid: [u8; WLAN_PMKID_LEN],
}

//
// struct brcmf_pmksa_v2 - PMK Security Association
//
// @length: Length of the structure.
// @bssid: The AP's BSSID.
// @pmkid: The PMK ID.
// @pmk: PMK material for FILS key derivation.
// @pmk_len: Length of PMK data.
// @ssid: The AP's SSID.
// @fils_cache_id: FILS cache identifier
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_pmksa_v2 {
    pub length: __le16,
    pub bssid: [u8; ETH_ALEN],
    pub pmkid: [u8; WLAN_PMKID_LEN],
    pub pmk: [u8; WLAN_PMK_LEN_SUITE_B_192],
    pub pmk_len: __le16,
    pub ssid: brcmf_ssid8_le,
    pub fils_cache_id: u16,
}

//
// struct brcmf_pmksa_v3 - PMK Security Association
//
// @bssid: The AP's BSSID.
// @pmkid: The PMK ID.
// @pmkid_len: The length of the PMK ID.
// @pmk: PMK material for FILS key derivation.
// @pmk_len: Length of PMK data.
// @fils_cache_id: FILS cache identifier
// @ssid: The AP's SSID.
// @time_left: Remaining time until expiry. 0 = expired, ~0 = no expiry.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_pmksa_v3 {
    pub bssid: [u8; ETH_ALEN],
    pub pmkid: [u8; WLAN_PMKID_LEN],
    pub pmkid_len: u8,
    pub pmk: [u8; WLAN_PMK_LEN_SUITE_B_192],
    pub pmk_len: u8,
    pub fils_cache_id: __le16,
    pub pad: u8,
    pub ssid: brcmf_ssid8_le,
    pub time_left: __le32,
}

//
// struct brcmf_pmk_list_le - List of pmksa's.
//
// @npmk: Number of pmksa's.
// @pmk: PMK SA information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_pmk_list_le {
    pub npmk: __le32,
    pub pmk: [brcmf_pmksa; BRCMF_MAXPMKID],
}

//
// struct brcmf_pmk_list_v2_le - List of pmksa's.
//
// @version: Request version.
// @length: Length of this structure.
// @pmk: PMK SA information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_pmk_list_v2_le {
    pub version: __le16,
    pub length: __le16,
    pub pmk: [brcmf_pmksa_v2; BRCMF_MAXPMKID],
}

//
// struct brcmf_pmk_op_v3_le - Operation on PMKSA list.
//
// @version: Request version.
// @length: Length of this structure.
// @pmk: PMK SA information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_pmk_op_v3_le {
    pub version: __le16,
    pub length: __le16,
    pub count: __le16,
    pub pad: __le16,
    pub pmk: [brcmf_pmksa_v3; BRCMF_MAXPMKID],
}

//
// struct brcmf_pno_param_le - PNO scan configuration parameters
//
// @version: PNO parameters version.
// @scan_freq: scan frequency.
// @lost_network_timeout: #sec. to declare discovered network as lost.
// @flags: Bit field to control features of PFN such as sort criteria auto
// enable switch and background scan.
// @rssi_margin: Margin to avoid jitter for choosing a PFN based on RSSI sort
// criteria.
// @bestn: number of best networks in each scan.
// @mscan: number of scans recorded.
// @repeat: minimum number of scan intervals before scan frequency changes
// in adaptive scan.
// @exp: exponent of 2 for maximum scan interval.
// @slow_freq: slow scan period.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_pno_param_le {
    pub version: __le32,
    pub scan_freq: __le32,
    pub lost_network_timeout: __le32,
    pub flags: __le16,
    pub rssi_margin: __le16,
    pub bestn: u8,
    pub mscan: u8,
    pub repeat: u8,
    pub exp: u8,
    pub slow_freq: __le32,
}

//
// struct brcmf_pno_config_le - PNO channel configuration.
//
// @reporttype: determines what is reported.
// @channel_num: number of channels specified in @channel_list.
// @channel_list: channels to use in PNO scan.
// @flags: reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_pno_config_le {
    pub reporttype: __le32,
    pub channel_num: __le32,
    pub channel_list: [__le16; BRCMF_NUMCHANNELS],
    pub flags: __le32,
}

//
// struct brcmf_pno_net_param_le - scan parameters per preferred network.
//
// @ssid: ssid name and its length.
// @flags: bit2: hidden.
// @infra: BSS vs IBSS.
// @auth: Open vs Closed.
// @wpa_auth: WPA type.
// @wsec: wsec value.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_pno_net_param_le {
    pub ssid: brcmf_ssid_le,
    pub flags: __le32,
    pub infra: __le32,
    pub auth: __le32,
    pub wpa_auth: __le32,
    pub wsec: __le32,
}

//
// struct brcmf_pno_net_info_le - information per found network.
//
// @bssid: BSS network identifier.
// @channel: channel number only.
// @SSID_len: length of ssid.
// @SSID: ssid characters.
// @RSSI: receive signal strength (in dBm).
// @timestamp: age in seconds.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_pno_net_info_le {
    pub bssid: [u8; ETH_ALEN],
    pub channel: u8,
    pub SSID_len: u8,
    pub SSID: [u8; 32],
    pub RSSI: __le16,
    pub timestamp: __le16,
}

//
// struct brcmf_pno_scanresults_le - result returned in PNO NET FOUND event.
//
// @version: PNO version identifier.
// @status: indicates completion status of PNO scan.
// @count: amount of brcmf_pno_net_info_le entries appended.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_pno_scanresults_le {
    pub version: __le32,
    pub status: __le32,
    pub count: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_pno_scanresults_v2_le {
    pub version: __le32,
    pub status: __le32,
    pub count: __le32,
    pub scan_ch_bucket: __le32,
}

//
// struct brcmf_pno_macaddr_le - to configure PNO macaddr randomization.
//
// @version: PNO version identifier.
// @flags: Flags defining how mac addrss should be used.
// @mac: MAC address.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_pno_macaddr_le {
    pub version: u8,
    pub flags: u8,
    pub mac: [u8; ETH_ALEN],
}

//
// struct brcmf_dload_data_le - data passing to firmware for downloading
// @flag: flags related to download data.
// @dload_type: type of download data.
// @len: length in bytes of download data.
// @crc: crc of download data.
// @data: download data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_dload_data_le {
    pub flag: __le16,
    pub dload_type: __le16,
    pub len: __le32,
    pub crc: __le32,
    pub data: [u8; ],
}

//
// struct brcmf_pno_bssid_le - bssid configuration for PNO scan.
//
// @bssid: BSS network identifier.
// @flags: flags for this BSSID.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_pno_bssid_le {
    pub bssid: [u8; ETH_ALEN],
    pub flags: __le16,
}

//
// struct brcmf_pktcnt_le - packet counters.
//
// @rx_good_pkt: packets (MSDUs & MMPDUs) received from this station
// @rx_bad_pkt: failed rx packets
// @tx_good_pkt: packets (MSDUs & MMPDUs) transmitted to this station
// @tx_bad_pkt: failed tx packets
// @rx_ocast_good_pkt: unicast packets destined for others
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_pktcnt_le {
    pub rx_good_pkt: __le32,
    pub rx_bad_pkt: __le32,
    pub tx_good_pkt: __le32,
    pub tx_bad_pkt: __le32,
    pub rx_ocast_good_pkt: __le32,
}

//
// struct brcmf_gtk_keyinfo_le - GTP rekey data
//
// @kck: key confirmation key.
// @kek: key encryption key.
// @replay_counter: replay counter.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_gtk_keyinfo_le {
    pub kck: [u8; BRCMF_RSN_KCK_LENGTH],
    pub kek: [u8; BRCMF_RSN_KEK_LENGTH],
    pub replay_counter: [u8; BRCMF_RSN_REPLAY_LEN],
}

//
// struct brcmf_gscan_bucket_config - configuration data for channel bucket.
//
// @bucket_end_index: last channel index in @channel_list in
// @struct brcmf_pno_config_le.
// @bucket_freq_multiple: scan interval expressed in N * @scan_freq.
// @flag: channel bucket report flags.
// @reserved: for future use.
// @repeat: number of scan at interval for exponential scan.
// @max_freq_multiple: maximum scan interval for exponential scan.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_gscan_bucket_config {
    pub bucket_end_index: u8,
    pub bucket_freq_multiple: u8,
    pub flag: u8,
    pub reserved: u8,
    pub repeat: __le16,
    pub max_freq_multiple: __le16,
}

// version supported which must match firmware
pub const BRCMF_GSCAN_CFG_VERSION: c_int = 2;
//
// enum brcmf_gscan_cfg_flags - bit values for gscan flags.
//
// @BRCMF_GSCAN_CFG_FLAGS_ALL_RESULTS: send probe responses/beacons to host.
// @BRCMF_GSCAN_CFG_ALL_BUCKETS_IN_1ST_SCAN: all buckets will be included in
// first scan cycle.
// @BRCMF_GSCAN_CFG_FLAGS_CHANGE_ONLY: indicated only flags member is changed.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum brcmf_gscan_cfg_flags {
    BRCMF_GSCAN_CFG_FLAGS_ALL_RESULTS = BIT(0),
    BRCMF_GSCAN_CFG_ALL_BUCKETS_IN_1ST_SCAN = BIT(3),
    BRCMF_GSCAN_CFG_FLAGS_CHANGE_ONLY = BIT(7),
}

//
// struct brcmf_gscan_config - configuration data for gscan.
//
// @version: version of the api to match firmware.
// @flags: flags according %enum brcmf_gscan_cfg_flags.
// @buffer_threshold: percentage threshold of buffer to generate an event.
// @swc_nbssid_threshold: number of BSSIDs with significant change that
// will generate an event.
// @swc_rssi_window_size: size of rssi cache buffer (max=8).
// @count_of_channel_buckets: number of array members in @bucket.
// @retry_threshold: !unknown!
// @lost_ap_window: !unknown!
// @bucket: array of channel buckets.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_gscan_config {
    pub version: __le16,
    pub flags: u8,
    pub buffer_threshold: u8,
    pub swc_nbssid_threshold: u8,
    pub swc_rssi_window_size: u8,
    pub count_of_channel_buckets: u8,
    pub retry_threshold: u8,
    pub lost_ap_window: __le16,
    pub __counted_by(count_of_channel_buckets): brcmf_gscan_bucket_config bucket[],
}

//
// struct brcmf_mkeep_alive_pkt_le - configuration data for keep-alive frame.
//
// @version: version for mkeep_alive
// @length: length of fixed parameters in the structure.
// @period_msec: keep-alive period in milliseconds.
// @len_bytes: size of the data.
// @keep_alive_id: ID  (0 - 3).
// @data: keep-alive frame data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_mkeep_alive_pkt_le {
    pub version: __le16,
    pub length: __le16,
    pub period_msec: __le32,
    pub len_bytes: __le16,
    pub keep_alive_id: u8,
    pub data: [u8; ],
    pub __packed: },
