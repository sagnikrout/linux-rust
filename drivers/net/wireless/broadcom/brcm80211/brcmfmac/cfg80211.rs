//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmfmac/cfg80211.h
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
// Copyright (c) 2010 Broadcom Corporation
//
// for brcmu_d11inf

pub const BRCMF_SCAN_IE_LEN_MAX: c_int = 2048;
pub const WL_NUM_SCAN_MAX: c_int = 10;
pub const WL_TLV_INFO_MAX: c_int = 1024;
pub const WL_BSS_INFO_MAX: c_int = 2048;

pub const WL_EXTRA_BUF_MAX: c_int = 2048;

pub const WL_ROAM_DELTA: c_int = 20;
// WME Access Category Indices (ACIs)

pub const EDCF_AC_COUNT: c_int = 4;
pub const MAX_8021D_PRIO: c_int = 8;
pub const EDCF_ACI_MASK: c_uint = 0x60;
pub const EDCF_ACI_SHIFT: c_int = 5;
pub const EDCF_ACM_MASK: c_uint = 0x10;
pub const EDCF_ECWMIN_MASK: c_uint = 0x0f;
pub const EDCF_ECWMAX_SHIFT: c_int = 4;
pub const EDCF_AIFSN_MASK: c_uint = 0x0f;
pub const EDCF_AIFSN_MAX: c_int = 15;
pub const EDCF_ECWMAX_MASK: c_uint = 0xf0;
// Keep BRCMF_ESCAN_BUF_SIZE below 64K (65536). Allocing over 64K can be
// problematic on some systems and should be avoided.
//
pub const BRCMF_ESCAN_BUF_SIZE: c_int = 65000;

pub const WL_ESCAN_ACTION_START: c_int = 1;
pub const WL_ESCAN_ACTION_CONTINUE: c_int = 2;
pub const WL_ESCAN_ACTION_ABORT: c_int = 3;

pub const IE_MAX_LEN: c_int = 512;
// IE TLV processing

// 802.11 Mgmt Packet flags
pub const BRCMF_VNDR_IE_BEACON_FLAG: c_uint = 0x1;
pub const BRCMF_VNDR_IE_PRBRSP_FLAG: c_uint = 0x2;
pub const BRCMF_VNDR_IE_ASSOCRSP_FLAG: c_uint = 0x4;
pub const BRCMF_VNDR_IE_AUTHRSP_FLAG: c_uint = 0x8;
pub const BRCMF_VNDR_IE_PRBREQ_FLAG: c_uint = 0x10;
pub const BRCMF_VNDR_IE_ASSOCREQ_FLAG: c_uint = 0x20;
// vendor IE in IW advertisement protocol ID field
pub const BRCMF_VNDR_IE_IWAPID_FLAG: c_uint = 0x40;
// allow custom IE id
pub const BRCMF_VNDR_IE_CUSTOM_FLAG: c_uint = 0x100;
// P2P Action Frames flags (spec ordered)
pub const BRCMF_VNDR_IE_GONREQ_FLAG: c_uint = 0x001000;
pub const BRCMF_VNDR_IE_GONRSP_FLAG: c_uint = 0x002000;
pub const BRCMF_VNDR_IE_GONCFM_FLAG: c_uint = 0x004000;
pub const BRCMF_VNDR_IE_INVREQ_FLAG: c_uint = 0x008000;
pub const BRCMF_VNDR_IE_INVRSP_FLAG: c_uint = 0x010000;
pub const BRCMF_VNDR_IE_DISREQ_FLAG: c_uint = 0x020000;
pub const BRCMF_VNDR_IE_DISRSP_FLAG: c_uint = 0x040000;
pub const BRCMF_VNDR_IE_PRDREQ_FLAG: c_uint = 0x080000;
pub const BRCMF_VNDR_IE_PRDRSP_FLAG: c_uint = 0x100000;
pub const BRCMF_VNDR_IE_P2PAF_SHIFT: c_int = 12;
pub const BRCMF_MAX_DEFAULT_KEYS: c_int = 6;
// beacon loss timeout defaults
pub const BRCMF_DEFAULT_BCN_TIMEOUT_ROAM_ON: c_int = 2;
pub const BRCMF_DEFAULT_BCN_TIMEOUT_ROAM_OFF: c_int = 4;

//
// enum brcmf_scan_status - scan engine status
//
// @BRCMF_SCAN_STATUS_BUSY: scanning in progress on dongle.
// @BRCMF_SCAN_STATUS_ABORT: scan being aborted on dongle.
// @BRCMF_SCAN_STATUS_SUPPRESS: scanning is suppressed in driver.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum brcmf_scan_status {
    BRCMF_SCAN_STATUS_BUSY,
    BRCMF_SCAN_STATUS_ABORT,
    BRCMF_SCAN_STATUS_SUPPRESS,
}

// dongle configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_cfg80211_conf {
    pub frag_threshold: u32,
    pub rts_threshold: u32,
    pub retry_short: u32,
    pub retry_long: u32,
}

// security information with currently associated ap
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_cfg80211_security {
    pub wpa_versions: u32,
    pub auth_type: u32,
    pub cipher_pairwise: u32,
    pub cipher_group: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum brcmf_profile_fwsup {
    BRCMF_PROFILE_FWSUP_NONE,
    BRCMF_PROFILE_FWSUP_PSK,
    BRCMF_PROFILE_FWSUP_1X,
    BRCMF_PROFILE_FWSUP_SAE
}

//
// enum brcmf_profile_fwauth - firmware authenticator profile
//
// @BRCMF_PROFILE_FWAUTH_NONE: no firmware authenticator
// @BRCMF_PROFILE_FWAUTH_PSK: authenticator for WPA/WPA2-PSK
// @BRCMF_PROFILE_FWAUTH_SAE: authenticator for SAE
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum brcmf_profile_fwauth {
    BRCMF_PROFILE_FWAUTH_NONE,
    BRCMF_PROFILE_FWAUTH_PSK,
    BRCMF_PROFILE_FWAUTH_SAE
}

//
// enum brcmf_mgmt_tx_status - mgmt frame tx status
//
// @BRCMF_MGMT_TX_ACK: mgmt frame acked
// @BRCMF_MGMT_TX_NOACK: mgmt frame not acked
// @BRCMF_MGMT_TX_OFF_CHAN_COMPLETED: off-channel complete
// @BRCMF_MGMT_TX_SEND_FRAME: mgmt frame tx is in progres
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum brcmf_mgmt_tx_status {
    BRCMF_MGMT_TX_ACK,
    BRCMF_MGMT_TX_NOACK,
    BRCMF_MGMT_TX_OFF_CHAN_COMPLETED,
    BRCMF_MGMT_TX_SEND_FRAME
}

//
// struct brcmf_cfg80211_profile - profile information.
//
// @bssid: bssid of joined/joining ibss.
// @sec: security information.
// @key: key information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_cfg80211_profile {
    pub bssid: [u8; ETH_ALEN],
    pub sec: brcmf_cfg80211_security,
    pub key: [brcmf_wsec_key; BRCMF_MAX_DEFAULT_KEYS],
    pub use_fwsup: brcmf_profile_fwsup,
    pub use_fwauth: u16,
    pub is_ft: bool,
}

//
// enum brcmf_vif_status - bit indices for vif status.
//
// @BRCMF_VIF_STATUS_READY: ready for operation.
// @BRCMF_VIF_STATUS_CONNECTING: connect/join in progress.
// @BRCMF_VIF_STATUS_CONNECTED: connected/joined successfully.
// @BRCMF_VIF_STATUS_DISCONNECTING: disconnect/disable in progress.
// @BRCMF_VIF_STATUS_AP_CREATED: AP operation started.
// @BRCMF_VIF_STATUS_EAP_SUCCESS: EAPOL handshake successful.
// @BRCMF_VIF_STATUS_ASSOC_SUCCESS: successful SET_SSID received.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum brcmf_vif_status {
    BRCMF_VIF_STATUS_READY,
    BRCMF_VIF_STATUS_CONNECTING,
    BRCMF_VIF_STATUS_CONNECTED,
    BRCMF_VIF_STATUS_DISCONNECTING,
    BRCMF_VIF_STATUS_AP_CREATED,
    BRCMF_VIF_STATUS_EAP_SUCCESS,
    BRCMF_VIF_STATUS_ASSOC_SUCCESS,
}

//
// struct vif_saved_ie - holds saved IEs for a virtual interface.
//
// @probe_req_ie: IE info for probe request.
// @probe_res_ie: IE info for probe response.
// @beacon_ie: IE info for beacon frame.
// @assoc_req_ie: IE info for association request frame.
// @assoc_res_ie: IE info for association response frame.
// @probe_req_ie_len: IE info length for probe request.
// @probe_res_ie_len: IE info length for probe response.
// @beacon_ie_len: IE info length for beacon frame.
// @assoc_req_ie_len: IE info length for association request frame.
// @assoc_res_ie_len: IE info length for association response frame.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vif_saved_ie {
    pub probe_req_ie: [u8; IE_MAX_LEN],
    pub probe_res_ie: [u8; IE_MAX_LEN],
    pub beacon_ie: [u8; IE_MAX_LEN],
    pub assoc_req_ie: [u8; IE_MAX_LEN],
    pub assoc_res_ie: [u8; IE_MAX_LEN],
    pub probe_req_ie_len: u32,
    pub probe_res_ie_len: u32,
    pub beacon_ie_len: u32,
    pub assoc_req_ie_len: u32,
    pub assoc_res_ie_len: u32,
}

//
// struct brcmf_cfg80211_vif - virtual interface specific information.
//
// @ifp: lower layer interface pointer
// @wdev: wireless device.
// @profile: profile information.
// @sme_state: SME state using enum brcmf_vif_status bits.
// @saved_ie: saved IE info for a vif.
// @list: linked list.
// @mgmt_tx: completion for management frame transmit.
// @mgmt_tx_status: status of last management frame sent to firmware.
// @mgmt_tx_id:
// @mgmt_rx_reg: registered rx mgmt frame types.
// @mbss: Multiple BSS type, set if not first AP (not relevant for P2P).
// @is_11d: beacon contains country IE, enable regulatory 802.11d support
// @cqm_rssi_low: Lower RSSI limit for CQM monitoring
// @cqm_rssi_high: Upper RSSI limit for CQM monitoring
// @cqm_rssi_last: Last RSSI reading for CQM monitoring
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_cfg80211_vif {
    pub ifp: *mut brcmf_if,
    pub wdev: wireless_dev,
    pub profile: brcmf_cfg80211_profile,
    pub sme_state: c_ulong,
    pub saved_ie: vif_saved_ie,
    pub list: list_head,
    pub mgmt_tx: completion,
    pub mgmt_tx_status: c_ulong,
    pub mgmt_tx_id: u32,
    pub mgmt_rx_reg: u16,
    pub mbss: bool,
    pub is_11d: c_int,
    pub cqm_rssi_low: i32,
    pub cqm_rssi_high: i32,
    pub cqm_rssi_last: i32,
}

// association inform
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_cfg80211_connect_info {
    pub req_ie: *mut u8,
    pub req_ie_len: i32,
    pub resp_ie: *mut u8,
    pub resp_ie_len: i32,
}

// assoc ie length
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_cfg80211_assoc_ielen_le {
    pub req_len: __le32,
    pub resp_len: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_cfg80211_edcf_acparam {
    pub ACI: u8,
    pub ECW: u8,
    pub /: *mut *mut u16 TXOP; / stored in network order (ls octet first),
}

// dongle escan state
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wl_escan_state {
    WL_ESCAN_STATE_IDLE,
    WL_ESCAN_STATE_SCANNING
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct escan_info {
    pub escan_state: u32,
    pub escan_buf: *mut u8,
    pub wiphy: *mut wiphy,
    pub ifp: *mut brcmf_if,
    pub request): *mut cfg80211_scan_request,
}

//
// struct brcmf_cfg80211_vif_event - virtual interface event information.
//
// @vif_wq: waitqueue awaiting interface event from firmware.
// @vif_event_lock: protects other members in this structure.
// @vif_complete: completion for net attach.
// @action: either add, change, or delete.
// @vif: virtual interface object related to the event.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_cfg80211_vif_event {
    pub vif_wq: wait_queue_head_t,
    pub vif_event_lock: spinlock_t,
    pub action: u8,
    pub vif: *mut brcmf_cfg80211_vif,
}

//
// struct brcmf_cfg80211_wowl - wowl related information.
//
// @active: set on suspend, cleared on resume.
// @pre_pmmode: firmware PM mode at entering suspend.
// @nd: net dectect data.
// @nd_info: helper struct to pass to cfg80211.
// @nd_data_wait: wait queue to sync net detect data.
// @nd_data_completed: completion for net detect data.
// @nd_enabled: net detect enabled.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_cfg80211_wowl {
    pub active: bool,
    pub pre_pmmode: u32,
    pub nd: *mut cfg80211_wowlan_nd_match,
    pub nd_info: *mut cfg80211_wowlan_nd_info,
    pub nd_data_wait: wait_queue_head_t,
    pub nd_data_completed: bool,
    pub nd_enabled: bool,
}

//
// struct brcmf_cfg80211_info - dongle private data of cfg80211 interface
//
// @wiphy: wiphy object for cfg80211 interface.
// @ops: pointer to copy of ops as registered with wiphy object.
// @conf: dongle configuration.
// @p2p: peer-to-peer specific information.
// @btcoex: Bluetooth coexistence information.
// @scan_request: cfg80211 scan request object.
// @usr_sync: mainly for dongle up/down synchronization.
// @bss_list: bss_list holding scanned ap information.
// @bss_info: bss information for cfg80211 layer.
// @conn_info: association info.
// @pmk_list: wpa2 pmk list.
// @scan_status: scan activity on the dongle.
// @pub: common driver information.
// @channel: current channel.
// @int_escan_map: bucket map for which internal e-scan is done.
// @ibss_starter: indicates this sta is ibss starter.
// @pwr_save: indicate whether dongle to support power save mode.
// @dongle_up: indicate whether dongle up or not.
// @roam_on: on/off switch for dongle self-roaming.
// @scan_tried: indicates if first scan attempted.
// @dcmd_buf: dcmd buffer.
// @extra_buf: mainly to grab assoc information.
// @debugfsdir: debugfs folder for this device.
// @escan_info: escan information.
// @escan_timeout: Timer for catch scan timeout.
// @escan_timeout_work: scan timeout worker.
// @vif_list: linked list of vif instances.
// @vif_cnt: number of vif instances.
// @vif_event: vif event signalling.
// @wowl: wowl related information.
// @pno: information of pno module.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_cfg80211_info {
    pub wiphy: *mut wiphy,
    pub conf: *mut brcmf_cfg80211_conf,
    pub p2p: brcmf_p2p_info,
    pub btcoex: *mut brcmf_btcoex_info,
    pub scan_request: *mut cfg80211_scan_request,
    pub usr_sync: mutex,
    pub bss_info: *mut wl_cfg80211_bss_info,
    pub conn_info: brcmf_cfg80211_connect_info,
    pub pmk_list: brcmf_pmk_list_le,
    pub scan_status: c_ulong,
    pub pub: *mut brcmf_pub,
    pub channel: u32,
    pub int_escan_map: u32,
    pub ibss_starter: bool,
    pub pwr_save: bool,
    pub dongle_up: bool,
    pub scan_tried: bool,
    pub dcmd_buf: *mut u8,
    pub extra_buf: *mut u8,
    pub debugfsdir: *mut dentry,
    pub escan_info: escan_info,
    pub escan_timeout: timer_list,
    pub escan_timeout_work: work_struct,
    pub vif_list: list_head,
    pub vif_event: brcmf_cfg80211_vif_event,
    pub vif_disabled: completion,
    pub d11inf: brcmu_d11inf,
    pub assoclist: brcmf_assoclist_le,
    pub wowl: brcmf_cfg80211_wowl,
    pub pno: *mut brcmf_pno_info,
    pub ac_priority: [u8; MAX_8021D_PRIO],
}

//
// struct brcmf_tlv - tag_ID/length/value_buffer tuple.
//
// @id: tag identifier.
// @len: number of bytes in value buffer.
// @data: value buffer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_tlv {
    pub id: u8,
    pub len: u8,
    pub data: [u8; ],
}

extern "C" {
    pub fn wiphy_to_cfg(_arg: wd->wiphy) -> return;
}
extern "C" {
    pub fn container_of(_arg: wdev, brcmf_cfg80211_vif: struct, _arg: wdev) -> return;
}
extern "C" {
    pub fn wdev_to_cfg(_arg: ndev->ieee80211_ptr) -> return;
}
extern "C" {
    pub fn brcmf_cfg80211_detach(cfg: *mut brcmf_cfg80211_info);
}
extern "C" {
    pub fn brcmf_cfg80211_up(ndev: *mut net_device) -> i32;
}
extern "C" {
    pub fn brcmf_cfg80211_down(ndev: *mut net_device) -> i32;
}
extern "C" {
    pub fn brcmf_free_vif(vif: *mut brcmf_cfg80211_vif);
}
extern "C" {
    pub fn brcmf_vif_clear_mgmt_ies(vif: *mut brcmf_cfg80211_vif) -> i32;
}
extern "C" {
    pub fn brcmf_cfg80211_vif_event_armed(cfg: *mut brcmf_cfg80211_info) -> bool;
}
extern "C" {
    pub fn brcmf_set_mpc(ndev: *mut brcmf_if, mpc: c_int);
}
extern "C" {
    pub fn brcmf_is_apmode_operating(wiphy: *mut wiphy) -> bool;
}
extern "C" {
    pub fn brcmf_abort_scanning(cfg: *mut brcmf_cfg80211_info);
}
extern "C" {
    pub fn brcmf_cfg80211_free_vif(ndev: *mut net_device);
}
extern "C" {
    pub fn brcmf_set_wsec(ifp: *mut brcmf_if, key: *const u8, key_len: u16, flags: u16) -> c_int;
}
