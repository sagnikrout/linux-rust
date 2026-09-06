//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/wcn36xx/wcn36xx.h
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


//
// Copyright (c) 2013 Eugene Krasnikov <k.eugene.e@gmail.com>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
// OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

pub const WCN36XX_AGGR_BUFFER_SIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcn36xx_debug_mask {
    WCN36XX_DBG_DXE		= 0x00000001,
    WCN36XX_DBG_DXE_DUMP	= 0x00000002,
    WCN36XX_DBG_SMD		= 0x00000004,
    WCN36XX_DBG_SMD_DUMP	= 0x00000008,
    WCN36XX_DBG_RX		= 0x00000010,
    WCN36XX_DBG_RX_DUMP	= 0x00000020,
    WCN36XX_DBG_TX		= 0x00000040,
    WCN36XX_DBG_TX_DUMP	= 0x00000080,
    WCN36XX_DBG_HAL		= 0x00000100,
    WCN36XX_DBG_HAL_DUMP	= 0x00000200,
    WCN36XX_DBG_MAC		= 0x00000400,
    WCN36XX_DBG_BEACON	= 0x00000800,
    WCN36XX_DBG_BEACON_DUMP	= 0x00001000,
    WCN36XX_DBG_PMC		= 0x00002000,
    WCN36XX_DBG_PMC_DUMP	= 0x00004000,
    WCN36XX_DBG_TESTMODE		= 0x00008000,
    WCN36XX_DBG_TESTMODE_DUMP	= 0x00010000,
    WCN36XX_DBG_ANY		= 0xffffffff,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcn36xx_ampdu_state {
    WCN36XX_AMPDU_NONE,
    WCN36XX_AMPDU_INIT,
    WCN36XX_AMPDU_START,
    WCN36XX_AMPDU_OPERATIONAL,
}

pub const HW_VALUE_PHY_SHIFT: c_int = 8;

// Macro flag: #define WCN36XX_HW_CHANNEL(__wcn)\

pub const RF_UNKNOWN: c_uint = 0x0000;
pub const RF_IRIS_WCN3620: c_uint = 0x3620;
pub const RF_IRIS_WCN3660: c_uint = 0x3660;
pub const RF_IRIS_WCN3680: c_uint = 0x3680;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv_data {
    pub is_valid: c_int,
    pub table: u8,
}

//
// struct wcn36xx_vif - holds VIF related fields
//
// @bss_index: bss_index is initially set to 0xFF. bss_index is received from
// HW after first config_bss call and must be used in delete_bss and
// enter/exit_bmps.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_vif {
    pub list: list_head,
    pub dtim_period: u8,
    pub encrypt_type: ani_ed_type,
    pub is_joining: bool,
    pub sta_assoc: bool,
    pub ssid: wcn36xx_hal_mac_ssid,
    pub bss_type: wcn36xx_hal_bss_type,
// Power management
    pub pw_state: wcn36xx_power_state,
    pub bss_index: u8,
// Returned from WCN36XX_HAL_ADD_STA_SELF_RSP
    pub self_sta_index: u8,
    pub self_dpu_desc_index: u8,
    pub self_ucast_dpu_sign: u8,

// IPv6 addresses for WoWLAN
    pub target_ipv6_addrs: [in6_addr; WCN36XX_HAL_IPV6_OFFLOAD_ADDR_MAX],
    pub tentative_addrs: [c_ulong; BITS_TO_LONGS(WCN36XX_HAL_IPV6_OFFLOAD_ADDR_MAX)],
    pub num_target_ipv6_addrs: c_int,

// WoWLAN GTK rekey data
    pub kek: [u8 kck[NL80211_KCK_LEN],; NL80211_KEK_LEN],
    pub replay_ctr: __le64,
    pub valid: bool,
    pub rekey_data: },
    pub sta_list: list_head,
    pub bmps_fail_ct: c_int,
}

//
// struct wcn36xx_sta - holds STA related fields
//
// @tid: traffic ID that is used during AMPDU and in TX BD.
// @sta_index: STA index is returned from HW after config_sta call and is
// used in both SMD channel and TX BD.
// @dpu_desc_index: DPU descriptor index is returned from HW after config_sta
// call and is used in TX BD.
// @bss_sta_index: STA index is returned from HW after config_bss call and is
// used in both SMD channel and TX BD. See table below when it is used.
// @bss_dpu_desc_index: DPU descriptor index is returned from HW after
// config_bss call and is used in TX BD.
// ______________________________________________
// |		  |	STA	|	AP	|
// |______________|_____________|_______________|
// |    TX BD     |bss_sta_index|   sta_index   |
// |______________|_____________|_______________|
// |all SMD calls |bss_sta_index|   sta_index	|
// |______________|_____________|_______________|
// |smd_delete_sta|  sta_index  |   sta_index	|
// |______________|_____________|_______________|
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_sta {
    pub list: list_head,
    pub vif: *mut wcn36xx_vif,
    pub aid: u16,
    pub tid: u16,
    pub sta_index: u8,
    pub dpu_desc_index: u8,
    pub ucast_dpu_sign: u8,
    pub bss_sta_index: u8,
    pub bss_dpu_desc_index: u8,
    pub is_data_encrypted: bool,
// Rates
    pub supported_rates: wcn36xx_hal_supported_rates_v1,
    pub /: *mut *mut spinlock_t ampdu_lock; / protects next two fields,
    pub ampdu_state: [wcn36xx_ampdu_state; 16],
    pub non_agg_frame_ct: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_chan_survey {
    pub rssi: i8,
    pub snr: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx {
    pub hw: *mut ieee80211_hw,
    pub dev: *mut device,
    pub vif_list: list_head,
    pub nv_file: *const c_char,
    pub nv: *const firmware,
    pub fw_revision: u8,
    pub fw_version: u8,
    pub fw_minor: u8,
    pub fw_major: u8,
    pub fw_feat_caps: [u32; WCN36XX_HAL_CAPS_SIZE],
    pub is_pronto: bool,
    pub is_pronto_v3: bool,
// extra byte for the NULL termination
    pub 1]: u8 crm_version[WCN36XX_HAL_VERSION_LENGTH +,
    pub 1]: u8 wlan_version[WCN36XX_HAL_VERSION_LENGTH +,
    pub first_boot: bool,
// IRQs
    pub tx_irq: c_int,
    pub rx_irq: c_int,
    pub ccu_base: *mut void __iomem,
    pub dxe_base: *mut void __iomem,
    pub smd_channel: *mut rpmsg_endpoint,
    pub tx_enable_state: *mut qcom_smem_state,
    pub tx_enable_state_bit: unsigned,
    pub tx_rings_empty_state: *mut qcom_smem_state,
    pub tx_rings_empty_state_bit: unsigned,
// prevents concurrent FW reconfiguration
    pub conf_mutex: mutex,
//
// smd_buf must be protected with smd_mutex to garantee
// that all messages are sent one after another
//
    pub hal_buf: *mut u8,
    pub hal_rsp_len: usize,
    pub hal_mutex: mutex,
    pub hal_rsp_compl: completion,
    pub hal_ind_wq: *mut workqueue_struct,
    pub hal_ind_work: work_struct,
    pub hal_ind_lock: spinlock_t,
    pub hal_ind_queue: list_head,
    pub scan_req: *mut cfg80211_scan_request,
    pub sw_scan: bool,
    pub sw_scan_opchannel: u8,
    pub sw_scan_init: bool,
    pub sw_scan_channel: u8,
    pub sw_scan_vif: *mut ieee80211_vif,
    pub scan_lock: mutex,
    pub scan_aborted: bool,
// DXE channels
    pub /: *mut *mut wcn36xx_dxe_ch dxe_tx_l_ch; / TX low,
    pub /: *mut *mut wcn36xx_dxe_ch dxe_tx_h_ch; / TX high,
    pub /: *mut *mut wcn36xx_dxe_ch dxe_rx_l_ch; / RX low,
    pub /: *mut *mut wcn36xx_dxe_ch dxe_rx_h_ch; / RX high,
// For synchronization of DXE resources from BH, IRQ and WQ contexts
    pub dxe_lock: spinlock_t,
    pub queues_stopped: bool,
// Memory pools
    pub mgmt_mem_pool: wcn36xx_dxe_mem_pool,
    pub data_mem_pool: wcn36xx_dxe_mem_pool,
    pub tx_ack_skb: *mut sk_buff,
    pub tx_ack_timer: timer_list,
// For A-MSDU re-aggregation
    pub amsdu: sk_buff_head,
// RF module
    pub rf_id: unsigned,

// Debug file system entry
    pub dfs: wcn36xx_dfs_entry,

    pub band: *mut ieee80211_supported_band,
    pub channel: *mut ieee80211_channel,
    pub /: *mut *mut spinlock_t survey_lock; / protects chan_survey,
    pub chan_survey: [wcn36xx_chan_survey; ],
}

extern "C" {
    pub fn wcn36xx_set_default_rates(rates: *mut wcn36xx_hal_supported_rates);
}
extern "C" {
    pub fn wcn36xx_set_default_rates_v1(rates: *mut wcn36xx_hal_supported_rates_v1);
}
extern "C" {
    pub fn container_of()sta_priv: *mut (void, ieee80211_sta: struct, _arg: drv_priv) -> return;
}
extern "C" {
    pub fn container_of(vif_priv: *mut *mut (void ), ieee80211_vif: struct, _arg: drv_priv) -> return;
}
