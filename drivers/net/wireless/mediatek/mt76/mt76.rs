//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt76.h
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
//
// Copyright (C) 2016 Felix Fietkau <nbd@nbd.name>
//

pub const MT_MCU_RING_SIZE: c_int = 32;
pub const MT_RX_BUF_SIZE: c_int = 2048;
pub const MT_SKB_HEAD_LEN: c_int = 256;
pub const MT_MAX_NON_AQL_PKT: c_int = 16;
pub const MT_TXQ_FREE_THR: c_int = 32;
pub const MT76_TOKEN_FREE_THR: c_int = 64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_reg_pair {
    pub reg: u32,
    pub value: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt76_bus_type {
    MT76_BUS_MMIO,
    MT76_BUS_USB,
    MT76_BUS_SDIO,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt76_wed_type {
    MT76_WED_Q_TX,
    MT76_WED_Q_TXFREE,
    MT76_WED_Q_RX,
    MT76_WED_RRO_Q_DATA,
    MT76_WED_RRO_Q_MSDU_PG,
    MT76_WED_RRO_Q_IND,
    MT76_WED_RRO_Q_RXDMAD_C,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt76_hwrro_mode {
    MT76_HWRRO_OFF,
    MT76_HWRRO_V3,
    MT76_HWRRO_V3_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_bus_ops {
    pub offset): *mut *mut *mut u32 (rr)(struct mt76_dev dev, u32,
    pub val): *mut *mut *mut void (wr)(struct mt76_dev dev, u32 offset, u32,
    pub val): *mut *mut *mut u32 (rmw)(struct mt76_dev dev, u32 offset, u32 mask, u32,
    pub len): c_int,
    pub len): c_int,
    pub len): *const *const mt76_reg_pair rp, int,
    pub len): *mut *mut mt76_reg_pair rp, int,
    pub type: mt76_bus_type,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt76_txq_id {
    MT_TXQ_VO = IEEE80211_AC_VO,
    MT_TXQ_VI = IEEE80211_AC_VI,
    MT_TXQ_BE = IEEE80211_AC_BE,
    MT_TXQ_BK = IEEE80211_AC_BK,
    MT_TXQ_PSD,
    MT_TXQ_BEACON,
    MT_TXQ_CAB,
    __MT_TXQ_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt76_mcuq_id {
    MT_MCUQ_WM,
    MT_MCUQ_WA,
    MT_MCUQ_FWDL,
    __MT_MCUQ_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt76_rxq_id {
    MT_RXQ_MAIN,
    MT_RXQ_MCU,
    MT_RXQ_MCU_WA,
    MT_RXQ_BAND1,
    MT_RXQ_BAND1_WA,
    MT_RXQ_MAIN_WA,
    MT_RXQ_BAND2,
    MT_RXQ_BAND2_WA,
    MT_RXQ_RRO_BAND0,
    MT_RXQ_RRO_BAND1,
    MT_RXQ_RRO_BAND2,
    MT_RXQ_MSDU_PAGE_BAND0,
    MT_RXQ_MSDU_PAGE_BAND1,
    MT_RXQ_MSDU_PAGE_BAND2,
    MT_RXQ_TXFREE_BAND0,
    MT_RXQ_TXFREE_BAND1,
    MT_RXQ_TXFREE_BAND2,
    MT_RXQ_RRO_IND,
    MT_RXQ_RRO_RXDMAD_C,
    MT_RXQ_NPU0,
    MT_RXQ_NPU1,
    __MT_RXQ_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt76_band_id {
    MT_BAND0,
    MT_BAND1,
    MT_BAND2,
    __MT_MAX_BAND
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt76_cipher_type {
    MT_CIPHER_NONE,
    MT_CIPHER_WEP40,
    MT_CIPHER_TKIP,
    MT_CIPHER_TKIP_NO_MIC,
    MT_CIPHER_AES_CCMP,
    MT_CIPHER_WEP104,
    MT_CIPHER_BIP_CMAC_128,
    MT_CIPHER_WEP128,
    MT_CIPHER_WAPI,
    MT_CIPHER_CCMP_CCX,
    MT_CIPHER_CCMP_256,
    MT_CIPHER_GCMP,
    MT_CIPHER_GCMP_256,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt76_dfs_state {
    MT_DFS_STATE_UNKNOWN,
    MT_DFS_STATE_DISABLED,
    MT_DFS_STATE_CAC,
    MT_DFS_STATE_ACTIVE,
}

pub const MT76_RNR_SCAN_MAX_BSSIDS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_scan_rnr_param {
    pub bssid: [u8; MT76_RNR_SCAN_MAX_BSSIDS][ETH_ALEN],
    pub channel: [u8; MT76_RNR_SCAN_MAX_BSSIDS],
    pub random_mac: [u8; ETH_ALEN],
    pub seq_num: u8,
    pub bssid_num: u8,
    pub sreq_flag: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_queue_buf {
    pub addr: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_tx_info {
    pub buf: [mt76_queue_buf; 32],
    pub skb: *mut sk_buff,
    pub nbuf: c_int,
    pub info: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_queue_entry {
    pub buf: *mut c_void,
    pub skb: *mut sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_queue_regs {
    pub desc_base: u32,
    pub ring_size: u32,
    pub cpu_idx: u32,
    pub dma_idx: u32,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_queue {
    pub regs: *mut mt76_queue_regs __iomem,
    pub lock: spinlock_t,
    pub cleanup_lock: spinlock_t,
    pub entry: *mut mt76_queue_entry,
    pub rro_desc: *mut mt76_rro_desc,
    pub desc: *mut mt76_desc,
    pub first: u16,
    pub head: u16,
    pub tail: u16,
    pub hw_idx: u8,
    pub ep: u8,
    pub ndesc: c_int,
    pub queued: c_int,
    pub buf_size: c_int,
    pub stopped: bool,
    pub blocked: bool,
    pub buf_offset: u8,
    pub flags: u16,
    pub magic_cnt: u8,
    pub emi_cpu_idx: *mut __le16,
    pub wed: *mut mtk_wed_device,
    pub dev: *mut mt76_dev,
    pub wed_regs: u32,
    pub desc_dma: dma_addr_t,
    pub rx_head: *mut sk_buff,
    pub page_pool: *mut page_pool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_mcu_ops {
    pub max_retry: c_uint,
    pub headroom: u32,
    pub tailroom: u32,
    pub wait_resp): int len, bool,
    pub seq): *mut int cmd, int,
    pub seq): *mut int cmd, int,
    pub seq): *mut *mut sk_buff skb, int,
    pub offset): *mut *mut *mut u32 (mcu_rr)(struct mt76_dev dev, u32,
    pub val): *mut *mut *mut void (mcu_wr)(struct mt76_dev dev, u32 offset, u32,
    pub len): *const *const mt76_reg_pair rp, int,
    pub len): *mut *mut mt76_reg_pair rp, int,
    pub dev): *mut *mut int (mcu_restart)(struct mt76_dev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_queue_ops {
    pub budget)): *mut *mut *mut int (poll)(struct napi_struct napi, int,
    pub ring_base): u32,
    pub sta): *mut *mut mt76_wcid wcid, ieee80211_sta,
    pub tx_info): *mut *mut sk_buff skb, u32,
    pub more): *mut *mut *mut int len, u32 info, bool,
    pub qid): *mut *mut *mut void (rx_reset)(struct mt76_dev dev, enum mt76_rxq_id,
    pub flush): bool,
    pub budget)): *mut *mut *mut int (poll)(struct napi_struct napi, int,
    pub q): *mut *mut *mut void (rx_cleanup)(struct mt76_dev dev, struct mt76_queue,
    pub q): *mut *mut *mut void (kick)(struct mt76_dev dev, struct mt76_queue,
    pub reset_idx): bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt76_phy_type {
    MT_PHY_TYPE_CCK,
    MT_PHY_TYPE_OFDM,
    MT_PHY_TYPE_HT,
    MT_PHY_TYPE_HT_GF,
    MT_PHY_TYPE_VHT,
    MT_PHY_TYPE_HE_SU = 8,
    MT_PHY_TYPE_HE_EXT_SU,
    MT_PHY_TYPE_HE_TB,
    MT_PHY_TYPE_HE_MU,
    MT_PHY_TYPE_EHT_SU = 13,
    MT_PHY_TYPE_EHT_TRIG,
    MT_PHY_TYPE_EHT_MU,
    __MT_PHY_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_sta_stats {
    pub tx_mode: [u64; __MT_PHY_TYPE_MAX],
    pub /: *mut *mut u64 tx_bw[5]; / 20, 40, 80, 160, 320,
    pub /: *mut *mut u64 tx_nss[4]; / 1, 2, 3, 4,
    pub /: *mut *mut u64 tx_mcs[16]; / mcs idx,
    pub tx_bytes: u64,
// WED TX
    pub /: *mut *mut u32 tx_packets; / unit: MSDU,
    pub tx_retries: u32,
    pub tx_failed: u32,
// WED RX
    pub rx_bytes: u64,
    pub rx_packets: u32,
    pub rx_errors: u32,
    pub rx_drops: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt76_wcid_flags {
    MT_WCID_FLAG_CHECK_PS,
    MT_WCID_FLAG_PS,
    MT_WCID_FLAG_4ADDR,
    MT_WCID_FLAG_HDR_TRANS,
    MT_WCID_FLAG_TDLS_PEER,
}

pub const MT76_N_WCIDS: c_int = 1088;
pub const MT76_BEACON_MON_MAX_MISS: c_int = 7;
// stored in ieee80211_tx_info::hw_queue

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_wcid {
    pub aggr: [*mut mt76_rx_tid __rcu; IEEE80211_NUM_TIDS],
    pub non_aql_packets: core::sync::atomic::AtomicI32,
    pub flags: c_ulong,
    pub rssi: ewma_signal,
    pub inactive_count: c_int,
    pub rate: rate_info,
    pub ampdu_state: c_ulong,
    pub idx: u16,
    pub hw_key_idx: u8,
    pub hw_key_idx2: u8,
    pub offchannel:1: u8,
    pub sta:1: u8,
    pub sta_disabled:1: u8,
    pub amsdu:1: u8,
    pub phy_idx:2: u8,
    pub link_id:4: u8,
    pub link_valid: bool,
    pub rx_check_pn: u8,
    pub 1][6]: u8 rx_key_pn[IEEE80211_NUM_TIDS +,
    pub cipher: u16,
    pub tx_info: u32,
    pub sw_iv: bool,
    pub tx_list: list_head,
    pub tx_pending: sk_buff_head,
    pub tx_offchannel: sk_buff_head,
    pub list: list_head,
    pub pktid: idr,
    pub stats: mt76_sta_stats,
    pub poll_list: list_head,
    pub def_wcid: *mut mt76_wcid,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_txq {
    pub wcid: u16,
    pub agg_ssn: u16,
    pub send_bar: bool,
    pub aggr: bool,
}

// data0

// data1

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_wed_rro_ind {
    pub data0: __le32,
    pub data1: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_txwi_cache {
    pub list: list_head,
    pub dma_addr: dma_addr_t,
    pub skb: *mut sk_buff,
    pub ptr: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_rx_tid {
    pub rcu_head: rcu_head,
    pub dev: *mut mt76_dev,
    pub lock: spinlock_t,
    pub reorder_work: delayed_work,
    pub id: u16,
    pub head: u16,
    pub size: u16,
    pub nframes: u16,
    pub num: u8,
    pub timer_pending:1: u8 started:1, stopped:1,,
    pub __counted_by(size): *mut *mut sk_buff reorder_buf[],
}

pub const MT_PACKET_ID_NO_ACK: c_int = 0;
pub const MT_PACKET_ID_NO_SKB: c_int = 1;
pub const MT_PACKET_ID_WED: c_int = 2;
pub const MT_PACKET_ID_FIRST: c_int = 3;

// This is timer for when to give up when waiting for TXS callback,
// with starting time being the time at which the DMA_DONE callback
// was seen (so, we know packet was processed then, it should not take
// long after that for firmware to send the TXS callback if it is going
// to do so.)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_tx_cb {
    pub jiffies: c_ulong,
    pub wcid: u16,
    pub pktid: u8,
    pub flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt76_sta_event {
    MT76_STA_EVENT_ASSOC,
    MT76_STA_EVENT_AUTHORIZE,
    MT76_STA_EVENT_DISASSOC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_hw_cap {
    pub has_2ghz: bool,
    pub has_5ghz: bool,
    pub has_6ghz: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_driver_ops {
    pub drv_flags: u32,
    pub survey_flags: u32,
    pub txwi_size: u16,
    pub token_size: u16,
    pub link_data_size: c_uint,
    pub phy): *mut *mut void (update_survey)(struct mt76_phy,
    pub phy): *mut *mut int (set_channel)(struct mt76_phy,
    pub tx_info): *mut mt76_tx_info,
    pub e): *mut mt76_queue_entry,
    pub update): *mut *mut *mut bool (tx_status_data)(struct mt76_dev dev, u8,
    pub len): *mut *mut *mut *mut bool (rx_check)(struct mt76_dev dev, void data, int,
    pub info): *mut *mut sk_buff skb, u32,
    pub q): *mut *mut *mut void (rx_poll_complete)(struct mt76_dev dev, enum mt76_rxq_id,
    pub data): *mut *mut *mut void (rx_rro_ind_process)(struct mt76_dev dev, void,
    pub data): *mut dma_addr_t p, void,
    pub ps): bool,
    pub sta): *mut ieee80211_sta,
    pub ev): *mut *mut ieee80211_sta sta, enum mt76_sta_event,
    pub sta): *mut ieee80211_sta,
    pub mlink): *mut mt76_vif_link,
    pub mlink): *mut mt76_vif_link,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_channel_state {
    pub cc_active: u64,
    pub cc_busy: u64,
    pub cc_rx: u64,
    pub cc_bss_rx: u64,
    pub cc_tx: u64,
    pub noise: i8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_sband {
    pub sband: ieee80211_supported_band,
    pub chan: *mut mt76_channel_state,
}

// addr req mask

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt_vendor_req {
    MT_VEND_DEV_MODE =	0x1,
    MT_VEND_WRITE =		0x2,
    MT_VEND_POWER_ON =	0x4,
    MT_VEND_MULTI_WRITE =	0x6,
    MT_VEND_MULTI_READ =	0x7,
    MT_VEND_READ_EEPROM =	0x9,
    MT_VEND_WRITE_FCE =	0x42,
    MT_VEND_WRITE_CFG =	0x46,
    MT_VEND_READ_CFG =	0x47,
    MT_VEND_READ_EXT =	0x63,
    MT_VEND_WRITE_EXT =	0x66,
    MT_VEND_FEATURE_SET =	0x91,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt76u_in_ep {
    MT_EP_IN_PKT_RX,
    MT_EP_IN_CMD_RESP,
    __MT_EP_IN_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt76u_out_ep {
    MT_EP_OUT_INBAND_CMD,
    MT_EP_OUT_AC_BE,
    MT_EP_OUT_AC_BK,
    MT_EP_OUT_AC_VI,
    MT_EP_OUT_AC_VO,
    MT_EP_OUT_HCCA,
    __MT_EP_OUT_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_mcu {
    pub mutex: mutex,
    pub msg_seq: u32,
    pub timeout: c_int,
    pub res_q: sk_buff_head,
    pub wait: wait_queue_head_t,
}

pub const MT_TX_SG_MAX_SIZE: c_int = 8;
pub const MT_RX_SG_MAX_SIZE: c_int = 4;
pub const MT_NUM_TX_ENTRIES: c_int = 256;
pub const MT_NUM_RX_ENTRIES: c_int = 128;
pub const MCU_RESP_URB_SIZE: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_usb {
    pub usb_ctrl_mtx: mutex,
    pub data: *mut u8,
    pub data_len: u16,
    pub status_worker: mt76_worker,
    pub rx_worker: mt76_worker,
    pub stat_work: work_struct,
    pub out_ep: [u8; __MT_EP_OUT_MAX],
    pub in_ep: [u8; __MT_EP_IN_MAX],
    pub err): *mut *mut *mut void (ctrl_timeout)(struct mt76_dev dev, int,
    pub sg_en: bool,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76u_mcu {
    pub data: *mut u8,
// multiple reads
    pub rp: *mut mt76_reg_pair,
    pub rp_len: c_int,
    pub base: u32,
    pub mcu: },
}

pub const MT76S_XMIT_BUF_SZ: c_uint = 0x3fe00;
pub const MT76S_NUM_TX_ENTRIES: c_int = 256;
pub const MT76S_NUM_RX_ENTRIES: c_int = 512;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_sdio {
    pub txrx_worker: mt76_worker,
    pub status_worker: mt76_worker,
    pub net_worker: mt76_worker,
    pub stat_worker: mt76_worker,
    pub xmit_buf: *mut u8,
    pub xmit_buf_sz: u32,
    pub func: *mut sdio_func,
    pub intr_data: *mut c_void,
    pub hw_ver: u8,
    pub wait: wait_queue_head_t,
    pub pse_mcu_quota_max: c_int,
    pub pse_data_quota: c_int,
    pub ple_data_quota: c_int,
    pub pse_mcu_quota: c_int,
    pub pse_page_size: c_int,
    pub deficit: c_int,
    pub sched: },
    pub intr): *mut *mut *mut int (parse_irq)(struct mt76_dev dev, struct mt76s_intr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_mmio {
    pub regs: *mut void __iomem,
    pub irq_lock: spinlock_t,
    pub irqmask: u32,
    pub wed: mtk_wed_device,
    pub wed_hif2: mtk_wed_device,
    pub wed_reset: completion,
    pub wed_reset_complete: completion,
    pub ppe_dev: *mut airoha_ppe_dev __rcu,
    pub npu: *mut airoha_npu __rcu,
    pub phy_addr: phys_addr_t,
    pub npu_type: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_rx_status {
    pub wcid: *mut mt76_wcid,
    pub wcid_idx: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_freq_range_power {
    pub range: *const cfg80211_sar_freq_ranges,
    pub power: i8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_testmode_ops {
    pub state): *mut *mut *mut int (set_state)(struct mt76_phy phy, enum mt76_testmode_state,
    pub new_state): mt76_testmode_state,
    pub msg): *mut *mut *mut int (dump_stats)(struct mt76_phy phy, struct sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_testmode_data {
    pub state: mt76_testmode_state,
    pub 32)]: u32 param_set[DIV_ROUND_UP(NUM_MT76_TM_ATTRS,,
    pub tx_skb: *mut sk_buff,
    pub tx_count: u32,
    pub tx_mpdu_len: u16,
    pub tx_rate_mode: u8,
    pub tx_rate_idx: u8,
    pub tx_rate_nss: u8,
    pub tx_rate_sgi: u8,
    pub tx_rate_ldpc: u8,
    pub tx_rate_stbc: u8,
    pub tx_ltf: u8,
    pub tx_antenna_mask: u8,
    pub tx_spe_idx: u8,
    pub tx_duty_cycle: u8,
    pub tx_time: u32,
    pub tx_ipg: u32,
    pub freq_offset: u32,
    pub tx_power: [u8; 4],
    pub tx_power_control: u8,
    pub addr: [u8; 3][ETH_ALEN],
    pub tx_pending: u32,
    pub tx_queued: u32,
    pub tx_queued_limit: u16,
    pub tx_done: u32,
    pub packets: [u64; __MT_RXQ_MAX],
    pub fcs_error: [u64; __MT_RXQ_MAX],
    pub rx_stats: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_vif_link {
    pub idx: u8,
    pub link_idx: u8,
    pub omac_idx: u8,
    pub band_idx: u8,
    pub wmm_idx: u8,
    pub scan_seq_num: u8,
    pub cipher: u8,
    pub basic_rates_idx: u8,
    pub mcast_rates_idx: u8,
    pub beacon_rates_idx: u8,
    pub offchannel: bool,
    pub beacon_mon_last: c_ulong,
    pub beacon_mon_interval: u16,
    pub ctx: *mut ieee80211_chanctx_conf,
    pub wcid: *mut mt76_wcid,
    pub mvif: *mut mt76_vif_data,
    pub rcu_head: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_vif_data {
    pub link: [*mut mt76_vif_link __rcu; IEEE80211_MLD_MAX_NUM_LINKS],
    pub offchannel_link: *mut mt76_vif_link __rcu,
    pub roc_phy: *mut mt76_phy,
    pub valid_links: u16,
    pub deflink_id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_phy {
    pub hw: *mut ieee80211_hw,
    pub dev: *mut mt76_dev,
    pub priv: *mut c_void,
    pub state: c_ulong,
    pub num_sta: c_uint,
    pub band_idx: u8,
    pub tx_lock: spinlock_t,
    pub tx_list: list_head,
    pub q_tx: [*mut mt76_queue; __MT_TXQ_MAX],
    pub mgmt_tx_pending: core::sync::atomic::AtomicI32,
    pub chandef: cfg80211_chan_def,
    pub main_chandef: cfg80211_chan_def,
    pub offchannel: bool,
    pub radar_enabled: bool,
    pub no_active_monitor: bool,
    pub roc_work: delayed_work,
    pub roc_vif: *mut ieee80211_vif,
    pub roc_link: *mut mt76_vif_link,
    pub chanctx: *mut mt76_chanctx,
    pub chan_state: *mut mt76_channel_state,
    pub dfs_state: mt76_dfs_state,
    pub survey_time: ktime_t,
    pub aggr_stats: [u32; 32],
    pub cap: mt76_hw_cap,
    pub sband_2g: mt76_sband,
    pub sband_5g: mt76_sband,
    pub sband_6g: mt76_sband,
    pub macaddr: [u8; ETH_ALEN],
    pub txpower_cur: c_int,
    pub antenna_mask: u8,
    pub chainmask: u16,

    pub test: mt76_testmode_data,

    pub mac_work: delayed_work,
    pub mac_work_count: u8,
    pub head: *mut sk_buff,
    pub tail: *mut sk_buff,
    pub seqno: u16,
    pub rx_amsdu: [}; __MT_RXQ_MAX],
    pub frp: *mut mt76_freq_range_power,
    pub cdev: led_classdev,
    pub name: [c_char; 32],
    pub al: bool,
    pub pin: u8,
    pub leds: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_dev {
    pub /: *mut *mut mt76_phy phy; / must be first,
    pub phys: [*mut mt76_phy; __MT_MAX_BAND],
    pub band_phys: [*mut mt76_phy; NUM_NL80211_BANDS],
    pub hw: *mut ieee80211_hw,
    pub wed_lock: spinlock_t,
    pub lock: spinlock_t,
    pub cc_lock: spinlock_t,
    pub cur_cc_bss_rx: u32,
    pub rx_ampdu_status: mt76_rx_status,
    pub rx_ampdu_len: u32,
    pub rx_ampdu_ref: u32,
    pub mutex: mutex,
    pub bus: *const mt76_bus_ops,
    pub drv: *const mt76_driver_ops,
    pub mcu_ops: *const mt76_mcu_ops,
// Optional callback to finalize wiphy state before registration.
    pub dev): *mut *mut int (init_wiphy)(struct mt76_dev,
    pub dev: *mut device,
    pub dma_dev: *mut device,
    pub mcu: mt76_mcu,
    pub napi_dev: *mut net_device,
    pub tx_napi_dev: *mut net_device,
    pub rx_lock: spinlock_t,
    pub napi: [napi_struct; __MT_RXQ_MAX],
    pub rx_skb: [sk_buff_head; __MT_RXQ_MAX],
    pub irq_tasklet: tasklet_struct,
    pub txwi_cache: list_head,
    pub rxwi_cache: list_head,
    pub q_mcu: [*mut mt76_queue; __MT_MCUQ_MAX],
    pub q_rx: [mt76_queue; __MT_RXQ_MAX],
    pub queue_ops: *const mt76_queue_ops,
    pub tx_dma_idx: [c_int; 4],
    pub hwrro_mode: mt76_hwrro_mode,
    pub tx_worker: mt76_worker,
    pub tx_napi: napi_struct,
    pub token_lock: spinlock_t,
    pub token: idr,
    pub wed_token_count: u16,
    pub token_count: u16,
    pub token_start: u16,
    pub token_size: u16,
    pub rx_token_lock: spinlock_t,
    pub rx_token: idr,
    pub rx_token_size: u16,
    pub tx_wait: wait_queue_head_t,
// spinclock used to protect wcid pktid linked list
    pub status_lock: spinlock_t,
    pub 32)]: u32 wcid_mask[DIV_ROUND_UP(MT76_N_WCIDS,,
    pub vif_mask: u64,
    pub global_wcid: mt76_wcid,
    pub wcid: [*mut mt76_wcid __rcu; MT76_N_WCIDS],
    pub wcid_list: list_head,
    pub sta_poll_list: list_head,
    pub sta_poll_lock: spinlock_t,
    pub rev: u32,
    pub pre_tbtt_tasklet: tasklet_struct,
    pub beacon_int: c_int,
    pub beacon_mask: u8,
    pub eeprom: debugfs_blob_wrapper,
    pub otp: debugfs_blob_wrapper,
    pub alpha2: [c_char; 3],
    pub region: nl80211_dfs_regions,
    pub rnr: mt76_scan_rnr_param,
    pub debugfs_reg: u32,
    pub csa_complete: u8,
    pub rxfilter: u32,
    pub scan_work: delayed_work,
    pub scan_lock: spinlock_t,
    pub req: *mut cfg80211_scan_request,
    pub chan: *mut ieee80211_channel,
    pub vif: *mut ieee80211_vif,
    pub mlink: *mut mt76_vif_link,
    pub phy: *mut mt76_phy,
    pub chan_idx: c_int,
    pub beacon_wait: bool,
    pub beacon_received: bool,
    pub scan: },

    pub test_ops: *const mt76_testmode_ops,
    pub name: *const c_char,
    pub offset: u32,
    pub test_mtd: },

    pub wq: *mut workqueue_struct,
    pub mmio: mt76_mmio,
    pub usb: mt76_usb,
    pub sdio: mt76_sdio,
}

// per-phy stats.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_mib_stats {
    pub ack_fail_cnt: u32,
    pub fcs_err_cnt: u32,
    pub rts_cnt: u32,
    pub rts_retries_cnt: u32,
    pub ba_miss_cnt: u32,
    pub tx_bf_cnt: u32,
    pub tx_mu_bf_cnt: u32,
    pub tx_mu_mpdu_cnt: u32,
    pub tx_mu_acked_mpdu_cnt: u32,
    pub tx_su_acked_mpdu_cnt: u32,
    pub tx_bf_ibf_ppdu_cnt: u32,
    pub tx_bf_ebf_ppdu_cnt: u32,
    pub tx_bf_rx_fb_all_cnt: u32,
    pub tx_bf_rx_fb_eht_cnt: u32,
    pub tx_bf_rx_fb_he_cnt: u32,
    pub tx_bf_rx_fb_vht_cnt: u32,
    pub tx_bf_rx_fb_ht_cnt: u32,
    pub /: *mut *mut u32 tx_bf_rx_fb_bw; / value of last sample, not cumulative,
    pub tx_bf_rx_fb_nc_cnt: u32,
    pub tx_bf_rx_fb_nr_cnt: u32,
    pub tx_bf_fb_cpl_cnt: u32,
    pub tx_bf_fb_trig_cnt: u32,
    pub tx_ampdu_cnt: u32,
    pub tx_stop_q_empty_cnt: u32,
    pub tx_mpdu_attempts_cnt: u32,
    pub tx_mpdu_success_cnt: u32,
    pub tx_pkt_ebf_cnt: u32,
    pub tx_pkt_ibf_cnt: u32,
    pub tx_rwp_fail_cnt: u32,
    pub tx_rwp_need_cnt: u32,
// rx stats
    pub rx_fifo_full_cnt: u32,
    pub channel_idle_cnt: u32,
    pub primary_cca_busy_time: u32,
    pub secondary_cca_busy_time: u32,
    pub primary_energy_detect_time: u32,
    pub cck_mdrdy_time: u32,
    pub ofdm_mdrdy_time: u32,
    pub green_mdrdy_time: u32,
    pub rx_vector_mismatch_cnt: u32,
    pub rx_delimiter_fail_cnt: u32,
    pub rx_mrdy_cnt: u32,
    pub rx_len_mismatch_cnt: u32,
    pub rx_mpdu_cnt: u32,
    pub rx_ampdu_cnt: u32,
    pub rx_ampdu_bytes_cnt: u32,
    pub rx_ampdu_valid_subframe_cnt: u32,
    pub rx_ampdu_valid_subframe_bytes_cnt: u32,
    pub rx_pfdrop_cnt: u32,
    pub rx_vec_queue_overflow_drop_cnt: u32,
    pub rx_ba_cnt: u32,
    pub tx_amsdu: [u32; 8],
    pub tx_amsdu_cnt: u32,
// mcu_muru_stats
    pub dl_cck_cnt: u32,
    pub dl_ofdm_cnt: u32,
    pub dl_htmix_cnt: u32,
    pub dl_htgf_cnt: u32,
    pub dl_vht_su_cnt: u32,
    pub dl_vht_2mu_cnt: u32,
    pub dl_vht_3mu_cnt: u32,
    pub dl_vht_4mu_cnt: u32,
    pub dl_he_su_cnt: u32,
    pub dl_he_ext_su_cnt: u32,
    pub dl_he_2ru_cnt: u32,
    pub dl_he_2mu_cnt: u32,
    pub dl_he_3ru_cnt: u32,
    pub dl_he_3mu_cnt: u32,
    pub dl_he_4ru_cnt: u32,
    pub dl_he_4mu_cnt: u32,
    pub dl_he_5to8ru_cnt: u32,
    pub dl_he_9to16ru_cnt: u32,
    pub dl_he_gtr16ru_cnt: u32,
    pub ul_hetrig_su_cnt: u32,
    pub ul_hetrig_2ru_cnt: u32,
    pub ul_hetrig_3ru_cnt: u32,
    pub ul_hetrig_4ru_cnt: u32,
    pub ul_hetrig_5to8ru_cnt: u32,
    pub ul_hetrig_9to16ru_cnt: u32,
    pub ul_hetrig_gtr16ru_cnt: u32,
    pub ul_hetrig_2mu_cnt: u32,
    pub ul_hetrig_3mu_cnt: u32,
    pub ul_hetrig_4mu_cnt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_power_limits {
    pub cck: [i8; 4],
    pub ofdm: [i8; 8],
    pub mcs: [i8; 4][10],
    pub ru: [i8; 7][12],
    pub eht: [i8; 16][16],
    pub cck: [i8; 4],
    pub ofdm: [i8; 4],
    pub ofdm_bf: [i8; 4],
    pub ru: [i8; 7][10],
    pub ru_bf: [i8; 7][10],
    pub path: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_ethtool_worker_info {
    pub data: *mut u64,
    pub idx: c_int,
    pub initial_stat_idx: c_int,
    pub worker_stat_count: c_int,
    pub sta_count: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_chanctx {
    pub phy: *mut mt76_phy,
}

extern "C" {
    pub fn mt76_mmio_init(dev: *mut mt76_dev, regs: *mut void __iomem);
}
extern "C" {
    pub fn mt76_pci_disable_aspm(pdev: *mut pci_dev);
}
extern "C" {
    pub fn mt76_pci_aspm_supported(pdev: *mut pci_dev) -> bool;
}
extern "C" {
    pub fn mt76_wed_release_rx_buf(wed: *mut mtk_wed_device);
}
extern "C" {
    pub fn mt76_wed_offload_disable(wed: *mut mtk_wed_device);
}
extern "C" {
    pub fn mt76_wed_reset_complete(wed: *mut mtk_wed_device);
}
extern "C" {
    pub fn mt76_wed_dma_reset(dev: *mut mt76_dev);
}

extern "C" {
    pub fn mt76_wed_init_rx_buf(wed: *mut mtk_wed_device, size: c_int) -> u32;
}
extern "C" {
    pub fn mt76_wed_offload_enable(wed: *mut mtk_wed_device) -> c_int;
}
extern "C" {
    pub fn mt76_wed_dma_setup(dev: *mut mt76_dev, q: *mut mt76_queue, reset: bool) -> c_int;
}

extern "C" {
    pub fn container_of(_arg: wed, mt76_dev: struct, _arg: mmio.wed_hif2) -> return;
}

extern "C" {
    pub fn container_of(_arg: wed, mt76_dev: struct, _arg: mmio.wed) -> return;
}
extern "C" {
    pub fn rcu_dereference(_arg: dev->wcid[idx]) -> return;
}

extern "C" {
    pub fn mt76_unregister_device(dev: *mut mt76_dev);
}
extern "C" {
    pub fn mt76_free_device(dev: *mut mt76_dev);
}
extern "C" {
    pub fn mt76_reset_device(dev: *mut mt76_dev);
}
extern "C" {
    pub fn mt76_unregister_phy(phy: *mut mt76_phy);
}
extern "C" {
    pub fn mt76_register_debugfs_fops(_arg: &dev->phy, _arg: NULL) -> return;
}
extern "C" {
    pub fn mt76_queues_read(s: *mut seq_file, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn mt76_eeprom_init(dev: *mut mt76_dev, len: c_int) -> c_int;
}
extern "C" {
    pub fn mt76_eeprom_override(phy: *mut mt76_phy) -> c_int;
}
extern "C" {
    pub fn mt76_get_of_data_from_mtd(dev: *mut mt76_dev, eep: *mut c_void, offset: c_int, len: c_int) -> c_int;
}
extern "C" {
    pub fn PTR_ERR(_arg: q) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: q) -> return;
}
// increment with wrap-around
// decrement with wrap-around
extern "C" {
    pub fn mt76_ac_to_hwq(ac: u8) -> u8;
}
extern "C" {
    pub fn container_of(_arg: ptr, ieee80211_txq: struct, _arg: drv_priv) -> return;
}
extern "C" {
    pub fn container_of(_arg: ptr, ieee80211_sta: struct, _arg: drv_priv) -> return;
}
// Alignment concerns

// hw = dev->phys[i]->hw;

extern "C" {
    pub fn mt76_rx(dev: *mut mt76_dev, q: mt76_rxq_id, skb: *mut sk_buff);
}
extern "C" {
    pub fn mt76_wake_tx_queue(hw: *mut ieee80211_hw, txq: *mut ieee80211_txq);
}
extern "C" {
    pub fn mt76_tx_check_agg_ssn(sta: *mut ieee80211_sta, skb: *mut sk_buff);
}
extern "C" {
    pub fn mt76_txq_schedule(phy: *mut mt76_phy, qid: mt76_txq_id);
}
extern "C" {
    pub fn mt76_txq_schedule_all(phy: *mut mt76_phy);
}
extern "C" {
    pub fn mt76_txq_schedule_pending(phy: *mut mt76_phy);
}
extern "C" {
    pub fn mt76_tx_worker_run(dev: *mut mt76_dev);
}
extern "C" {
    pub fn mt76_tx_worker(w: *mut mt76_worker);
}
extern "C" {
    pub fn mt76_has_tx_pending(phy: *mut mt76_phy) -> bool;
}
extern "C" {
    pub fn mt76_update_channel(phy: *mut mt76_phy) -> c_int;
}
extern "C" {
    pub fn mt76_update_survey(phy: *mut mt76_phy);
}
extern "C" {
    pub fn mt76_update_survey_active_time(phy: *mut mt76_phy, time: ktime_t);
}
extern "C" {
    pub fn mt76_rx_signal(chain_mask: u8, chain_signal: *mut i8) -> c_int;
}
extern "C" {
    pub fn mt76_set_stream_caps(phy: *mut mt76_phy, vht: bool);
}
extern "C" {
    pub fn mt76_rx_aggr_stop(dev: *mut mt76_dev, wcid: *mut mt76_wcid, tid: u8);
}
extern "C" {
    pub fn mt76_tx_status_check(dev: *mut mt76_dev, flush: bool);
}
extern "C" {
    pub fn mt76_get_min_avg_rssi(dev: *mut mt76_dev, phy_idx: u8) -> c_int;
}
extern "C" {
    pub fn mt76_get_power_bound(phy: *mut mt76_phy, txpower: i8) -> i8;
}
extern "C" {
    pub fn mt76_csa_check(dev: *mut mt76_dev);
}
extern "C" {
    pub fn mt76_csa_finish(dev: *mut mt76_dev);
}
extern "C" {
    pub fn mt76_set_tim(hw: *mut ieee80211_hw, sta: *mut ieee80211_sta, set: bool) -> c_int;
}
extern "C" {
    pub fn mt76_insert_ccmp_hdr(skb: *mut sk_buff, key_id: u8);
}
extern "C" {
    pub fn mt76_cancel_hw_scan(hw: *mut ieee80211_hw, vif: *mut ieee80211_vif);
}
extern "C" {
    pub fn mt76_scan_rx_beacon(dev: *mut mt76_dev, chan: *mut ieee80211_channel);
}
extern "C" {
    pub fn mt76_rx_beacon(phy: *mut mt76_phy, skb: *mut sk_buff);
}
extern "C" {
    pub fn mt76_beacon_mon_check(phy: *mut mt76_phy);
}
extern "C" {
    pub fn mt76_phy_dfs_state(phy: *mut mt76_phy) -> mt76_dfs_state;
}
extern "C" {
    pub fn mt76_testmode_set_state(phy: *mut mt76_phy, state: mt76_testmode_state) -> c_int;
}
extern "C" {
    pub fn mt76_testmode_alloc_skb(phy: *mut mt76_phy, len: u32) -> c_int;
}

extern "C" {
    pub fn mt76_npu_rx_queue_init(dev: *mut mt76_dev, q: *mut mt76_queue) -> c_int;
}
extern "C" {
    pub fn mt76_npu_fill_rx_queue(dev: *mut mt76_dev, q: *mut mt76_queue) -> c_int;
}
extern "C" {
    pub fn mt76_npu_queue_cleanup(dev: *mut mt76_dev, q: *mut mt76_queue);
}
extern "C" {
    pub fn mt76_npu_disable_irqs(dev: *mut mt76_dev);
}
extern "C" {
    pub fn mt76_npu_init(dev: *mut mt76_dev, phy_addr: phys_addr_t, type: c_int) -> c_int;
}
extern "C" {
    pub fn mt76_npu_deinit(dev: *mut mt76_dev);
}
extern "C" {
    pub fn mt76_npu_queue_setup(dev: *mut mt76_dev, q: *mut mt76_queue);
}
extern "C" {
    pub fn mt76_npu_txdesc_cleanup(q: *mut mt76_queue, index: c_int);
}

extern "C" {
    pub fn mt76_is_mmio(!!rcu_access_pointer(dev->mmio.npu: dev) &&) -> return;
}
extern "C" {
    pub fn mt76_is_mmio(!!rcu_access_pointer(dev->mmio.ppe_dev: dev) &&) -> return;
}

// internal
extern "C" {
    pub fn mt76_put_txwi(dev: *mut mt76_dev, t: *mut mt76_txwi_cache);
}
extern "C" {
    pub fn mt76_put_rxwi(dev: *mut mt76_dev, t: *mut mt76_txwi_cache);
}
extern "C" {
    pub fn mt76_free_pending_rxwi(dev: *mut mt76_dev);
}
extern "C" {
    pub fn mt76_rx_aggr_reorder(skb: *mut sk_buff, frames: *mut sk_buff_head);
}
extern "C" {
    pub fn mt76_testmode_tx_pending(phy: *mut mt76_phy);
}
// chandef = phy->main_chandef;
extern "C" {
    pub fn mt76_scan_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt76_abort_scan(dev: *mut mt76_dev);
}
extern "C" {
    pub fn mt76_roc_complete_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt76_roc_complete(phy: *mut mt76_phy);
}
extern "C" {
    pub fn mt76_abort_roc(phy: *mut mt76_phy);
}
extern "C" {
    pub fn mt76_offchannel_notify(phy: *mut mt76_phy, offchannel: bool);
}
// usb
extern "C" {
    pub fn usb_bulk_msg(_arg: udev, _arg: pipe, _arg: data, _arg: len, _arg: actual_len, _arg: timeout) -> return;
}
extern "C" {
    pub fn mt76_ethtool_page_pool_stats(dev: *mut mt76_dev, data: *mut u64, index: *mut c_int);
}
extern "C" {
    pub fn mt76_skb_adjust_pad(skb: *mut sk_buff, pad: c_int) -> c_int;
}
extern "C" {
    pub fn ___mt76u_rr(dev: *mut mt76_dev, req: u8, req_type: u8, addr: u32) -> u32;
}
extern "C" {
    pub fn mt76u_init(dev: *mut mt76_dev, intf: *mut usb_interface) -> c_int;
}
extern "C" {
    pub fn mt76u_alloc_mcu_queue(dev: *mut mt76_dev) -> c_int;
}
extern "C" {
    pub fn mt76u_alloc_queues(dev: *mut mt76_dev) -> c_int;
}
extern "C" {
    pub fn mt76u_stop_tx(dev: *mut mt76_dev);
}
extern "C" {
    pub fn mt76u_stop_rx(dev: *mut mt76_dev);
}
extern "C" {
    pub fn mt76u_resume_rx(dev: *mut mt76_dev) -> c_int;
}
extern "C" {
    pub fn mt76u_queues_deinit(dev: *mut mt76_dev);
}
extern "C" {
    pub fn mt76s_alloc_rx_queue(dev: *mut mt76_dev, qid: mt76_rxq_id) -> c_int;
}
extern "C" {
    pub fn mt76s_alloc_tx(dev: *mut mt76_dev) -> c_int;
}
extern "C" {
    pub fn mt76s_deinit(dev: *mut mt76_dev);
}
extern "C" {
    pub fn mt76s_sdio_irq(func: *mut sdio_func);
}
extern "C" {
    pub fn mt76s_txrx_worker(sdio: *mut mt76_sdio);
}
extern "C" {
    pub fn mt76s_txqs_empty(dev: *mut mt76_dev) -> bool;
}
extern "C" {
    pub fn mt76s_rr(dev: *mut mt76_dev, offset: u32) -> u32;
}
extern "C" {
    pub fn mt76s_wr(dev: *mut mt76_dev, offset: u32, val: u32);
}
extern "C" {
    pub fn mt76s_rmw(dev: *mut mt76_dev, offset: u32, mask: u32, val: u32) -> u32;
}
extern "C" {
    pub fn mt76s_read_pcr(dev: *mut mt76_dev) -> u32;
}
extern "C" {
    pub fn __mt76_mcu_msg_alloc(_arg: dev, _arg: data, _arg: data_len, _arg: data_len, _arg: GFP_KERNEL) -> return;
}
extern "C" {
    pub fn mt76_mcu_rx_event(dev: *mut mt76_dev, skb: *mut sk_buff);
}
extern "C" {
    pub fn __mt76_mcu_send_firmware(_arg: dev, _arg: cmd, _arg: data, _arg: len, _arg: max_len) -> return;
}
extern "C" {
    pub fn mt76_mcu_send_and_get_msg(_arg: dev, _arg: cmd, _arg: data, _arg: len, _arg: wait_resp, _arg: NULL) -> return;
}
extern "C" {
    pub fn mt76_mcu_skb_send_and_get_msg(_arg: dev, _arg: skb, _arg: cmd, _arg: wait_resp, _arg: NULL) -> return;
}
extern "C" {
    pub fn mt76_set_irq_mask(dev: *mut mt76_dev, addr: u32, clear: u32, set: u32);
}
extern "C" {
    pub fn mt76_token_consume(dev: *mut mt76_dev, ptxwi: *mut mt76_txwi_cache) -> c_int;
}
extern "C" {
    pub fn __mt76_set_tx_blocked(dev: *mut mt76_dev, blocked: bool);
}
extern "C" {
    pub fn mt76_create_page_pool(dev: *mut mt76_dev, q: *mut mt76_queue) -> c_int;
}
extern "C" {
    pub fn mt76_wcid_init(wcid: *mut mt76_wcid, band_idx: u8);
}
extern "C" {
    pub fn mt76_wcid_cleanup(dev: *mut mt76_dev, wcid: *mut mt76_wcid);
}
extern "C" {
    pub fn mt76_wcid_add_poll(dev: *mut mt76_dev, wcid: *mut mt76_wcid);
}
extern "C" {
    pub fn mt76_vif_cleanup(dev: *mut mt76_dev, vif: *mut ieee80211_vif);
}
extern "C" {
    pub fn mt76_select_links(vif: *mut ieee80211_vif, max_active_links: c_int) -> u16;
}
extern "C" {
    pub fn mt76_dereference(_arg: mvif->link[link_id], _arg: dev) -> return;
}
extern "C" {
    pub fn mt76_dereference(_arg: mvif->link[link_conf->link_id], _arg: dev) -> return;
}
