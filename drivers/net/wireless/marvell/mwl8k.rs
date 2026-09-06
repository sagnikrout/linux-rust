//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/marvell/mwl8k.c
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
// drivers/net/wireless/mwl8k.c
// Driver for Marvell TOPDOG 802.11 Wireless cards
//
// Copyright (C) 2008, 2009, 2010 Marvell Semiconductor Inc.
//
// This file is licensed under the terms of the GNU General Public
// License version 2.  This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//

// Module parameters
    static bool ap_mode_default;
    module_param(ap_mode_default, bool, 0);
    MODULE_PARM_DESC(ap_mode_default,
    "Set to 1 to make ap mode the default instead of sta mode");
// Register definitions
pub const MWL8K_HIU_GEN_PTR: c_uint = 0x00000c10;
pub const MWL8K_MODE_STA: c_uint = 0x0000005a;
pub const MWL8K_MODE_AP: c_uint = 0x000000a5;
pub const MWL8K_HIU_INT_CODE: c_uint = 0x00000c14;
pub const MWL8K_FWSTA_READY: c_uint = 0xf0f1f2f4;
pub const MWL8K_FWAP_READY: c_uint = 0xf1f2f4a5;
pub const MWL8K_INT_CODE_CMD_FINISHED: c_uint = 0x00000005;
pub const MWL8K_HIU_SCRATCH: c_uint = 0x00000c40;
// Host->device communications
pub const MWL8K_HIU_H2A_INTERRUPT_EVENTS: c_uint = 0x00000c18;
pub const MWL8K_HIU_H2A_INTERRUPT_STATUS: c_uint = 0x00000c1c;
pub const MWL8K_HIU_H2A_INTERRUPT_MASK: c_uint = 0x00000c20;
pub const MWL8K_HIU_H2A_INTERRUPT_CLEAR_SEL: c_uint = 0x00000c24;
pub const MWL8K_HIU_H2A_INTERRUPT_STATUS_MASK: c_uint = 0x00000c28;

// Device->host communications
pub const MWL8K_HIU_A2H_INTERRUPT_EVENTS: c_uint = 0x00000c2c;
pub const MWL8K_HIU_A2H_INTERRUPT_STATUS: c_uint = 0x00000c30;
pub const MWL8K_HIU_A2H_INTERRUPT_MASK: c_uint = 0x00000c34;
pub const MWL8K_HIU_A2H_INTERRUPT_CLEAR_SEL: c_uint = 0x00000c38;
pub const MWL8K_HIU_A2H_INTERRUPT_STATUS_MASK: c_uint = 0x00000c3c;

// HW micro second timer register
// located at offset 0xA600. This
// will be used to timestamp tx
// packets.
//
pub const MWL8K_HW_TIMER_REGISTER: c_uint = 0x0000a600;
pub const BBU_RXRDY_CNT_REG: c_uint = 0x0000a860;
pub const NOK_CCA_CNT_REG: c_uint = 0x0000a6a0;
pub const BBU_AVG_NOISE_VAL: c_uint = 0x67;

    MWL8K_A2H_INT_CHNL_SWITCHED | \
    MWL8K_A2H_INT_QUEUE_EMPTY | \
    MWL8K_A2H_INT_RADAR_DETECT | \
    MWL8K_A2H_INT_RADIO_ON | \
    MWL8K_A2H_INT_RADIO_OFF | \
    MWL8K_A2H_INT_MAC_EVENT | \
    MWL8K_A2H_INT_OPC_DONE | \
    MWL8K_A2H_INT_RX_READY | \
    MWL8K_A2H_INT_TX_DONE | \
    MWL8K_A2H_INT_BA_WATCHDOG)
pub const MWL8K_RX_QUEUES: c_int = 1;
pub const MWL8K_TX_WMM_QUEUES: c_int = 4;
pub const MWL8K_MAX_AMPDU_QUEUES: c_int = 8;

// txpriorities are mapped with hw queues.
// Each hw queue has a txpriority.
//
pub const TOTAL_HW_TX_QUEUES: c_int = 8;
// Each HW queue can have one AMPDU stream.
// But, because one of the hw queue is reserved,
// maximum AMPDU queues that can be created are
// one short of total tx queues.
//

pub const MWL8K_NUM_CHANS: c_int = 18;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxd_ops {
    pub rxd_size: c_int,
    pub next_dma_addr): *mut *mut *mut void (rxd_init)(void rxd, dma_addr_t,
    pub len): *mut *mut *mut void (rxd_refill)(void rxd, dma_addr_t addr, int,
    int (*rxd_process)(void *rxd, struct ieee80211_rx_status *status,
    pub noise): *mut *mut __le16 qos, s8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_device_info {
    pub part_name: *mut c_char,
    pub helper_image: *mut c_char,
    pub fw_image_sta: *mut c_char,
    pub fw_image_ap: *mut c_char,
    pub ap_rxd_ops: *mut rxd_ops,
    pub fw_api_ap: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_rx_queue {
    pub rxd_count: c_int,
// hw receives here
    pub head: c_int,
// refill descs here
    pub tail: c_int,
    pub rxd: *mut c_void,
    pub rxd_dma: dma_addr_t,
    struct {
    pub skb: *mut sk_buff,
    pub buf: *mut },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_tx_queue {
// hw transmits here
    pub head: c_int,
// sw appends here
    pub tail: c_int,
    pub len: c_uint,
    pub txd: *mut mwl8k_tx_desc,
    pub txd_dma: dma_addr_t,
    pub skb: *mut sk_buff,
}

    enum {
    AMPDU_NO_STREAM,
    AMPDU_STREAM_NEW,
    AMPDU_STREAM_IN_PROGRESS,
    AMPDU_STREAM_ACTIVE,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_ampdu_stream {
    pub sta: *mut ieee80211_sta,
    pub tid: u8,
    pub state: u8,
    pub idx: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_priv {
    pub hw: *mut ieee80211_hw,
    pub pdev: *mut pci_dev,
    pub irq: c_int,
    pub device_info: *mut mwl8k_device_info,
    pub sram: *mut void __iomem,
    pub regs: *mut void __iomem,
// firmware
    pub fw_helper: *const firmware,
    pub fw_ucode: *const firmware,
// hardware/firmware parameters
    pub ap_fw: bool,
    pub rxd_ops: *mut rxd_ops,
    pub band_24: ieee80211_supported_band,
    pub channels_24: [ieee80211_channel; 14],
    pub rates_24: [ieee80211_rate; 13],
    pub band_50: ieee80211_supported_band,
    pub channels_50: [ieee80211_channel; 9],
    pub rates_50: [ieee80211_rate; 8],
    pub ap_macids_supported: u32,
    pub sta_macids_supported: u32,
// Ampdu stream information
    pub num_ampdu_queues: u8,
    pub stream_lock: spinlock_t,
    pub ampdu: [mwl8k_ampdu_stream; MWL8K_MAX_AMPDU_QUEUES],
    pub watchdog_ba_handle: work_struct,
// firmware access
    pub fw_mutex: mutex,
    pub fw_mutex_owner: *mut task_struct,
    pub hw_restart_owner: *mut task_struct,
    pub fw_mutex_depth: c_int,
    pub hostcmd_wait: *mut completion,
    pub watchdog_event_pending: core::sync::atomic::AtomicI32,
// lock held over TX and TX reap
    pub tx_lock: spinlock_t,
// TX quiesce completion, protected by fw_mutex and tx_lock
    pub tx_wait: *mut completion,
// List of interfaces.
    pub macids_used: u32,
    pub vif_list: list_head,
// power management status cookie from firmware
    pub cookie: *mut u32,
    pub cookie_dma: dma_addr_t,
    pub num_mcaddrs: u16,
    pub hw_rev: u8,
    pub fw_rev: u32,
    pub caps: u32,
//
// Running count of TX packets in flight, to avoid
// iterating over the transmit rings each time.
//
    pub pending_tx_pkts: c_int,
    pub rxq: [mwl8k_rx_queue; MWL8K_RX_QUEUES],
    pub txq: [mwl8k_tx_queue; MWL8K_MAX_TX_QUEUES],
    pub txq_offset: [u32; MWL8K_MAX_TX_QUEUES],
    pub radio_on: bool,
    pub radio_short_preamble: bool,
    pub sniffer_enabled: bool,
    pub wmm_enabled: bool,
// XXX need to convert this to handle multiple interfaces
    pub capture_beacon: bool,
    pub capture_bssid: [u8; ETH_ALEN],
    pub beacon_skb: *mut sk_buff,
//
// This FJ worker has to be global as it is scheduled from the
// RX handler.  At this point we don't know which interface it
// belongs to until the list of bssids waiting to complete join
// is checked.
//
    pub finalize_join_worker: work_struct,
// Tasklet to perform TX reclaim.
    pub poll_tx_task: tasklet_struct,
// Tasklet to perform RX.
    pub poll_rx_task: tasklet_struct,
// Most recently reported noise in dBm
    pub noise: i8,
//
// preserve the queue configurations so they can be restored if/when
// the firmware image is swapped.
//
    pub wmm_params: [ieee80211_tx_queue_params; MWL8K_TX_WMM_QUEUES],
// To perform the task of reloading the firmware
    pub fw_reload: work_struct,
    pub hw_restart_in_progress: bool,
// async firmware loading state
    pub fw_state: unsigned,
    pub fw_pref: *mut c_char,
    pub fw_alt: *mut c_char,
    pub is_8764: bool,
    pub firmware_loading_complete: completion,
// bitmap of running BSSes
    pub running_bsses: u32,
// ACS related
    pub sw_scan_start: bool,
    pub acs_chan: *mut ieee80211_channel,
    pub channel_time: c_ulong,
    pub survey: [survey_info; MWL8K_NUM_CHANS],
}

pub const MAX_WEP_KEY_LEN: c_int = 13;
pub const NUM_WEP_KEYS: c_int = 4;
// Per interface specific private data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_vif {
    pub list: list_head,
    pub vif: *mut ieee80211_vif,
// Firmware macid for this vif.
    pub macid: c_int,
// Non AMPDU sequence number assigned by driver.
    pub seqno: u16,
// Saved WEP keys
    struct {
    pub enabled: u8,
    pub MAX_WEP_KEY_LEN]: u8 key[sizeof(struct ieee80211_key_conf) +,
    pub wep_key_conf: [}; NUM_WEP_KEYS],
// BSSID
    pub bssid: [u8; ETH_ALEN],
// A flag to indicate is HW crypto is enabled for this bssid
    pub is_hw_crypto_enabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_traffic_info {
    pub start_time: u32,
    pub pkts: u32,
}

pub const MWL8K_MAX_TID: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_sta {
// Index into station database. Returned by UPDATE_STADB.
    pub peer_id: u8,
    pub is_ampdu_allowed: u8,
    pub tx_stats: [tx_traffic_info; MWL8K_MAX_TID],
}

    static const struct ieee80211_channel mwl8k_channels_24[] = {
    { .band = NL80211_BAND_2GHZ, .center_freq = 2412, .hw_value = 1, },
    { .band = NL80211_BAND_2GHZ, .center_freq = 2417, .hw_value = 2, },
    { .band = NL80211_BAND_2GHZ, .center_freq = 2422, .hw_value = 3, },
    { .band = NL80211_BAND_2GHZ, .center_freq = 2427, .hw_value = 4, },
    { .band = NL80211_BAND_2GHZ, .center_freq = 2432, .hw_value = 5, },
    { .band = NL80211_BAND_2GHZ, .center_freq = 2437, .hw_value = 6, },
    { .band = NL80211_BAND_2GHZ, .center_freq = 2442, .hw_value = 7, },
    { .band = NL80211_BAND_2GHZ, .center_freq = 2447, .hw_value = 8, },
    { .band = NL80211_BAND_2GHZ, .center_freq = 2452, .hw_value = 9, },
    { .band = NL80211_BAND_2GHZ, .center_freq = 2457, .hw_value = 10, },
    { .band = NL80211_BAND_2GHZ, .center_freq = 2462, .hw_value = 11, },
    { .band = NL80211_BAND_2GHZ, .center_freq = 2467, .hw_value = 12, },
    { .band = NL80211_BAND_2GHZ, .center_freq = 2472, .hw_value = 13, },
    { .band = NL80211_BAND_2GHZ, .center_freq = 2484, .hw_value = 14, },
    };
    static const struct ieee80211_rate mwl8k_rates_24[] = {
    { .bitrate = 10, .hw_value = 2, },
    { .bitrate = 20, .hw_value = 4, },
    { .bitrate = 55, .hw_value = 11, },
    { .bitrate = 110, .hw_value = 22, },
    { .bitrate = 220, .hw_value = 44, },
    { .bitrate = 60, .hw_value = 12, },
    { .bitrate = 90, .hw_value = 18, },
    { .bitrate = 120, .hw_value = 24, },
    { .bitrate = 180, .hw_value = 36, },
    { .bitrate = 240, .hw_value = 48, },
    { .bitrate = 360, .hw_value = 72, },
    { .bitrate = 480, .hw_value = 96, },
    { .bitrate = 540, .hw_value = 108, },
    };
    static const struct ieee80211_channel mwl8k_channels_50[] = {
    { .band = NL80211_BAND_5GHZ, .center_freq = 5180, .hw_value = 36, },
    { .band = NL80211_BAND_5GHZ, .center_freq = 5200, .hw_value = 40, },
    { .band = NL80211_BAND_5GHZ, .center_freq = 5220, .hw_value = 44, },
    { .band = NL80211_BAND_5GHZ, .center_freq = 5240, .hw_value = 48, },
    { .band = NL80211_BAND_5GHZ, .center_freq = 5745, .hw_value = 149, },
    { .band = NL80211_BAND_5GHZ, .center_freq = 5765, .hw_value = 153, },
    { .band = NL80211_BAND_5GHZ, .center_freq = 5785, .hw_value = 157, },
    { .band = NL80211_BAND_5GHZ, .center_freq = 5805, .hw_value = 161, },
    { .band = NL80211_BAND_5GHZ, .center_freq = 5825, .hw_value = 165, },
    };
    static const struct ieee80211_rate mwl8k_rates_50[] = {
    { .bitrate = 60, .hw_value = 12, },
    { .bitrate = 90, .hw_value = 18, },
    { .bitrate = 120, .hw_value = 24, },
    { .bitrate = 180, .hw_value = 36, },
    { .bitrate = 240, .hw_value = 48, },
    { .bitrate = 360, .hw_value = 72, },
    { .bitrate = 480, .hw_value = 96, },
    { .bitrate = 540, .hw_value = 108, },
    };
// Set or get info from Firmware
pub const MWL8K_CMD_GET: c_uint = 0x0000;
pub const MWL8K_CMD_SET: c_uint = 0x0001;
pub const MWL8K_CMD_SET_LIST: c_uint = 0x0002;
// Firmware command codes
pub const MWL8K_CMD_CODE_DNLD: c_uint = 0x0001;
pub const MWL8K_CMD_GET_HW_SPEC: c_uint = 0x0003;
pub const MWL8K_CMD_SET_HW_SPEC: c_uint = 0x0004;
pub const MWL8K_CMD_MAC_MULTICAST_ADR: c_uint = 0x0010;
pub const MWL8K_CMD_GET_STAT: c_uint = 0x0014;
pub const MWL8K_CMD_BBP_REG_ACCESS: c_uint = 0x001a;
pub const MWL8K_CMD_RADIO_CONTROL: c_uint = 0x001c;
pub const MWL8K_CMD_RF_TX_POWER: c_uint = 0x001e;
pub const MWL8K_CMD_TX_POWER: c_uint = 0x001f;
pub const MWL8K_CMD_RF_ANTENNA: c_uint = 0x0020;
pub const MWL8K_CMD_SET_BEACON: c_uint = 0x0100		/* per-vif */;
pub const MWL8K_CMD_SET_PRE_SCAN: c_uint = 0x0107;
pub const MWL8K_CMD_SET_POST_SCAN: c_uint = 0x0108;
pub const MWL8K_CMD_SET_RF_CHANNEL: c_uint = 0x010a;
pub const MWL8K_CMD_SET_AID: c_uint = 0x010d;
pub const MWL8K_CMD_SET_RATE: c_uint = 0x0110;
pub const MWL8K_CMD_SET_FINALIZE_JOIN: c_uint = 0x0111;
pub const MWL8K_CMD_RTS_THRESHOLD: c_uint = 0x0113;
pub const MWL8K_CMD_SET_SLOT: c_uint = 0x0114;
pub const MWL8K_CMD_SET_EDCA_PARAMS: c_uint = 0x0115;
pub const MWL8K_CMD_SET_WMM_MODE: c_uint = 0x0123;
pub const MWL8K_CMD_MIMO_CONFIG: c_uint = 0x0125;
pub const MWL8K_CMD_USE_FIXED_RATE: c_uint = 0x0126;
pub const MWL8K_CMD_ENABLE_SNIFFER: c_uint = 0x0150;
pub const MWL8K_CMD_SET_MAC_ADDR: c_uint = 0x0202		/* per-vif */;
pub const MWL8K_CMD_SET_RATEADAPT_MODE: c_uint = 0x0203;
pub const MWL8K_CMD_GET_WATCHDOG_BITMAP: c_uint = 0x0205;
pub const MWL8K_CMD_DEL_MAC_ADDR: c_uint = 0x0206		/* per-vif */;
pub const MWL8K_CMD_BSS_START: c_uint = 0x1100		/* per-vif */;
pub const MWL8K_CMD_SET_NEW_STN: c_uint = 0x1111		/* per-vif */;
pub const MWL8K_CMD_UPDATE_ENCRYPTION: c_uint = 0x1122		/* per-vif */;
pub const MWL8K_CMD_UPDATE_STADB: c_uint = 0x1123;
pub const MWL8K_CMD_BASTREAM: c_uint = 0x1125;

    (ARRAY_SIZE(mwl8k_rates_24) - ARRAY_SIZE(mwl8k_rates_50))
    static const char *mwl8k_cmd_name(__le16 cmd, char *buf, int bufsize)
    {
    let mut command: u16 = le16_to_cpu(cmd);

    snprintf(buf, bufsize, "%s", #x);\
    return buf;\
    } while (0)
    switch (command & ~0x8000) {
    MWL8K_CMDNAME(CODE_DNLD);
    MWL8K_CMDNAME(GET_HW_SPEC);
    MWL8K_CMDNAME(SET_HW_SPEC);
    MWL8K_CMDNAME(MAC_MULTICAST_ADR);
    MWL8K_CMDNAME(GET_STAT);
    MWL8K_CMDNAME(RADIO_CONTROL);
    MWL8K_CMDNAME(RF_TX_POWER);
    MWL8K_CMDNAME(TX_POWER);
    MWL8K_CMDNAME(RF_ANTENNA);
    MWL8K_CMDNAME(SET_BEACON);
    MWL8K_CMDNAME(SET_PRE_SCAN);
    MWL8K_CMDNAME(SET_POST_SCAN);
    MWL8K_CMDNAME(SET_RF_CHANNEL);
    MWL8K_CMDNAME(SET_AID);
    MWL8K_CMDNAME(SET_RATE);
    MWL8K_CMDNAME(SET_FINALIZE_JOIN);
    MWL8K_CMDNAME(RTS_THRESHOLD);
    MWL8K_CMDNAME(SET_SLOT);
    MWL8K_CMDNAME(SET_EDCA_PARAMS);
    MWL8K_CMDNAME(SET_WMM_MODE);
    MWL8K_CMDNAME(MIMO_CONFIG);
    MWL8K_CMDNAME(USE_FIXED_RATE);
    MWL8K_CMDNAME(ENABLE_SNIFFER);
    MWL8K_CMDNAME(SET_MAC_ADDR);
    MWL8K_CMDNAME(SET_RATEADAPT_MODE);
    MWL8K_CMDNAME(BSS_START);
    MWL8K_CMDNAME(SET_NEW_STN);
    MWL8K_CMDNAME(UPDATE_ENCRYPTION);
    MWL8K_CMDNAME(UPDATE_STADB);
    MWL8K_CMDNAME(BASTREAM);
    MWL8K_CMDNAME(GET_WATCHDOG_BITMAP);
    default:
    snprintf(buf, bufsize, "0x%x", cmd);
    }

    return buf;
    }
// Hardware and firmware reset
#[no_mangle]
unsafe extern "C" fn mwl8k_hw_reset(priv: *mut mwl8k_priv) {
    static void mwl8k_hw_reset(struct mwl8k_priv *priv)
    {
    iowrite32(MWL8K_H2A_INT_RESET,
    priv.regs + MWL8K_HIU_H2A_INTERRUPT_EVENTS);
    iowrite32(MWL8K_H2A_INT_RESET,
    priv.regs + MWL8K_HIU_H2A_INTERRUPT_EVENTS);
    msleep(20);
    }
// Release fw image
#[no_mangle]
unsafe extern "C" fn mwl8k_release_fw(fw: *const firmware) {
    static void mwl8k_release_fw(const struct firmware **fw)
    {
    if (*fw == core::ptr::null_mut())
    return;
    release_firmware(*fw);
// fw = NULL;
    }
#[no_mangle]
unsafe extern "C" fn mwl8k_release_firmware(priv: *mut mwl8k_priv) {
    static void mwl8k_release_firmware(struct mwl8k_priv *priv)
    {
    mwl8k_release_fw(&priv.fw_ucode);
    mwl8k_release_fw(&priv.fw_helper);
    }
// states for asynchronous f/w loading
    static void mwl8k_fw_state_machine(const struct firmware *fw, void *context);
    enum {
    FW_STATE_INIT = 0,
    FW_STATE_LOADING_PREF,
    FW_STATE_LOADING_ALT,
    FW_STATE_ERROR,
    };
// Request fw image
    static int mwl8k_request_fw(struct mwl8k_priv *priv,
    const char *fname, const struct firmware **fw,
    bool nowait)
    {
// release current image
    if (*fw != core::ptr::null_mut())
    mwl8k_release_fw(fw);
    if (nowait)
    return request_firmware_nowait(THIS_MODULE, 1, fname,
    &priv.pdev.dev, GFP_KERNEL,
    priv, mwl8k_fw_state_machine);
    else
    return request_firmware(fw, fname, &priv.pdev.dev);
    }
    static int mwl8k_request_firmware(struct mwl8k_priv *priv, char *fw_image,
    bool nowait)
    {
    struct mwl8k_device_info *di = priv.device_info;
    int rc;
    if (di.helper_image != core::ptr::null_mut()) {
    if (nowait)
    rc = mwl8k_request_fw(priv, di.helper_image,
    &priv.fw_helper, true);
    else
    rc = mwl8k_request_fw(priv, di.helper_image,
    &priv.fw_helper, false);
    if (rc)
    printk(KERN_ERR "%s: Error requesting helper fw %s\n",
    pci_name(priv.pdev), di.helper_image);
    if (rc || nowait)
    return rc;
    }
    if (nowait) {
//
// if we get here, no helper image is needed.  Skip the
// FW_STATE_INIT state.
//
    priv.fw_state = FW_STATE_LOADING_PREF;
    rc = mwl8k_request_fw(priv, fw_image,
    &priv.fw_ucode,
    true);
    } else
    rc = mwl8k_request_fw(priv, fw_image,
    &priv.fw_ucode, false);
    if (rc) {
    printk(KERN_ERR "%s: Error requesting firmware file %s\n",
    pci_name(priv.pdev), fw_image);
    mwl8k_release_fw(&priv.fw_helper);
    return rc;
    }
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_cmd_pkt {
// New members MUST be added within the __struct_group() macro below.
    __struct_group(mwl8k_cmd_pkt_hdr, hdr, __packed,
    pub code: __le16,
    pub length: __le16,
    pub seq_num: __u8,
    pub macid: __u8,
    pub result: __le16,
    pub payload: [c_char; ],
    pub __packed: },
    static_assert(offsetof(struct mwl8k_cmd_pkt, payload) == sizeof(struct mwl8k_cmd_pkt_hdr),
    pub __struct_group()"): "struct member likely outside of,
//
// Firmware loading.
//
    static int
    mwl8k_send_fw_load_cmd(struct mwl8k_priv *priv, void *data, int length)
    {
    pub priv->regs: *mut *mut void __iomem regs =,
    pub dma_addr: dma_addr_t,
    pub loops: c_int,
    dma_addr = dma_map_single(&priv.pdev.dev, data, length,
    if (dma_mapping_error(&priv.pdev.dev, dma_addr))
    pub -ENOMEM: return,
    pub MWL8K_HIU_GEN_PTR): iowrite32(dma_addr, regs +,
    pub MWL8K_HIU_INT_CODE): iowrite32(0, regs +,
    iowrite32(MWL8K_H2A_INT_DOORBELL,
    pub MWL8K_HIU_H2A_INTERRUPT_EVENTS): regs +,
    iowrite32(MWL8K_H2A_INT_DUMMY,
    pub MWL8K_HIU_H2A_INTERRUPT_EVENTS): regs +,
    pub 1000: loops =,
    do {
    pub int_code: u32,
    if (priv.is_8764) {
    int_code = ioread32(regs +
    if (int_code == 0)
    } else {
    pub MWL8K_HIU_INT_CODE): int_code = ioread32(regs +,
    if (int_code == MWL8K_INT_CODE_CMD_FINISHED) {
    pub MWL8K_HIU_INT_CODE): iowrite32(0, regs +,
    }
    }
    pub (--loops): } while,
    pub DMA_TO_DEVICE): dma_unmap_single(&priv->pdev->dev, dma_addr, length,,
    pub -ETIMEDOUT: return loops ? 0 :,
    }
    static int mwl8k_load_fw_image(struct mwl8k_priv *priv,
    const u8 *data, size_t length)
    {
    pub cmd: *mut mwl8k_cmd_pkt,
    pub done: c_int,
    pub 0: int rc =,
    pub GFP_KERNEL): *mut *mut cmd = kmalloc(sizeof(cmd) + 256,,
    if (cmd == core::ptr::null_mut())
    pub -ENOMEM: return,
    pub cpu_to_le16(MWL8K_CMD_CODE_DNLD): cmd->code =,
    pub 0: cmd->seq_num =,
    pub 0: cmd->macid =,
    pub 0: cmd->result =,
    pub 0: done =,
    while (length) {
    pub length: int block_size = length > 256 ? 256 :,
    pub block_size): memcpy(cmd->payload, data + done,,
    pub cpu_to_le16(block_size): cmd->length =,
    rc = mwl8k_send_fw_load_cmd(priv, cmd,
    pub block_size): *mut *mut sizeof(cmd) +,
    if (rc)
    pub block_size: done +=,
    pub block_size: length -=,
    }
    if (!rc) {
    pub 0: cmd->length =,
    pub sizeof(*cmd)): *mut rc = mwl8k_send_fw_load_cmd(priv, cmd,,
    }
    pub rc: return,
    }
    static int mwl8k_feed_fw_image(struct mwl8k_priv *priv,
    const u8 *data, size_t length)
    {
    pub buffer: *mut c_uchar,
    pub 0: int may_continue, rc =,
    pub prev_block_size: u32 done,,
    pub GFP_KERNEL): buffer = kmalloc(1024,,
    if (buffer == core::ptr::null_mut())
    pub -ENOMEM: return,
    pub 0: done =,
    pub 0: prev_block_size =,
    pub 1000: may_continue =,
    while (may_continue > 0) {
    pub block_size: u32,
    pub MWL8K_HIU_SCRATCH): block_size = ioread32(priv->regs +,
    if (block_size & 1) {
    pub ~1: block_size &=,
    } else {
    pub prev_block_size: done +=,
    pub prev_block_size: length -=,
    }
    if (block_size > 1024 || block_size > length) {
    pub -EOVERFLOW: rc =,
    }
    if (length == 0) {
    pub 0: rc =,
    }
    if (block_size == 0) {
    pub -EPROTO: rc =,
    }
    pub block_size: prev_block_size =,
    pub block_size): memcpy(buffer, data + done,,
    pub block_size): rc = mwl8k_send_fw_load_cmd(priv, buffer,,
    if (rc)
    }
    if (!rc && length != 0)
    pub -EREMOTEIO: rc =,
    pub rc: return,
    }
#[no_mangle]
unsafe extern "C" fn mwl8k_load_firmware(hw: *mut ieee80211_hw) -> c_int {
    static int mwl8k_load_firmware(struct ieee80211_hw *hw)
    {
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    pub priv->fw_ucode: *const *const firmware fw =,
    pub rc: c_int,
    pub loops: c_int,
    if (!memcmp(fw.data, "\x01\x00\x00\x00", 4) && !priv.is_8764) {
    pub priv->fw_helper: *const *const firmware helper =,
    if (helper == core::ptr::null_mut()) {
    printk(KERN_ERR "%s: helper image needed but none "
    pub pci_name(priv->pdev)): "given\n",,
    pub -EINVAL: return,
    }
    pub helper->size): rc = mwl8k_load_fw_image(priv, helper->data,,
    if (rc) {
    printk(KERN_ERR "%s: unable to load firmware "
    pub pci_name(priv->pdev)): "helper image\n",,
    pub rc: return,
    }
    pub fw->size): rc = mwl8k_feed_fw_image(priv, fw->data,,
    } else {
    if (priv.is_8764)
    pub fw->size): rc = mwl8k_feed_fw_image(priv, fw->data,,
    else
    pub fw->size): rc = mwl8k_load_fw_image(priv, fw->data,,
    }
    if (rc) {
    printk(KERN_ERR "%s: unable to load firmware image\n",
    pub rc: return,
    }
    pub MWL8K_HIU_GEN_PTR): iowrite32(MWL8K_MODE_STA, priv->regs +,
    pub 500000: loops =,
    do {
    pub ready_code: u32,
    pub MWL8K_HIU_INT_CODE): ready_code = ioread32(priv->regs +,
    if (ready_code == MWL8K_FWAP_READY) {
    pub true: priv->ap_fw =,
    } else if (ready_code == MWL8K_FWSTA_READY) {
    pub false: priv->ap_fw =,
    }
    pub (--loops): } while,
    pub -ETIMEDOUT: return loops ? 0 :,
    }
// DMA header used by firmware and hardware.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_dma_data {
    pub fwlen: __le16,
    pub wh: ieee80211_hdr,
    pub data: [c_char; ],
    pub __aligned(2): } __packed,
// Routines to add/remove DMA header from skb.
#[no_mangle]
pub unsafe extern "C" fn mwl8k_remove_dma_header(skb: *mut sk_buff, qos: __le16) {
    static inline void mwl8k_remove_dma_header(struct sk_buff *skb, __le16 qos)
    {
    pub tr: *mut mwl8k_dma_data,
    pub hdrlen: c_int,
    pub )skb->data: *mut tr = (struct mwl8k_dma_data,
    pub ieee80211_hdrlen(tr->wh.frame_control): hdrlen =,
    if (hdrlen != sizeof(tr.wh)) {
    if (ieee80211_is_data_qos(tr.wh.frame_control)) {
    pub 2): memmove(tr->data - hdrlen, &tr->wh, hdrlen -,
// ((__le16 *)(tr->data - 2)) = qos;
    } else {
    pub hdrlen): memmove(tr->data - hdrlen, &tr->wh,,
    }
    }
    if (hdrlen != sizeof(*tr))
    pub hdrlen): *mut *mut skb_pull(skb, sizeof(tr) -,
    }
pub const REDUCED_TX_HEADROOM: c_int = 8;
    static void
    mwl8k_add_dma_header(struct mwl8k_priv *priv, struct sk_buff *skb,
    int head_pad, int tail_pad)
    {
    pub wh: *mut ieee80211_hdr,
    pub hdrlen: c_int,
    pub reqd_hdrlen: c_int,
    pub tr: *mut mwl8k_dma_data,
//
// Add a firmware DMA header; the firmware requires that we
// present a 2-byte payload length followed by a 4-address
// header (without QoS field), followed (optionally) by any
// WEP/ExtIV header (but only filled in for CCMP).
//
    pub )skb->data: *mut wh = (struct ieee80211_hdr,
    pub ieee80211_hdrlen(wh->frame_control): hdrlen =,
//
// Check if skb_resize is required because of
// tx_headroom adjustment.
//
    if (priv.ap_fw && (hdrlen < (sizeof(struct ieee80211_cts)
    + REDUCED_TX_HEADROOM))) {
    if (pskb_expand_head(skb, REDUCED_TX_HEADROOM, 0, GFP_ATOMIC)) {
    wiphy_err(priv.hw.wiphy,
    pub buffer\n"): "Failed to reallocate TX,
    }
    pub REDUCED_TX_HEADROOM: skb->truesize +=,
    }
    pub head_pad: *mut *mut reqd_hdrlen = sizeof(tr) +,
    if (hdrlen != reqd_hdrlen)
    pub hdrlen): skb_push(skb, reqd_hdrlen -,
    if (ieee80211_is_data_qos(wh.frame_control))
    pub IEEE80211_QOS_CTL_LEN: hdrlen -=,
    pub )skb->data: *mut tr = (struct mwl8k_dma_data,
    if (wh != &tr.wh)
    pub hdrlen): memmove(&tr->wh, wh,,
    if (hdrlen != sizeof(tr.wh))
    pub hdrlen): *mut *mut memset(((void )&tr->wh) + hdrlen, 0, sizeof(tr->wh) -,
//
// Firmware length is the length of the fully formed "802.11
// payload".  That is, everything except for the 802.11 header.
// This includes all crypto material including the MIC.
//
    pub tail_pad): *mut *mut tr->fwlen = cpu_to_le16(skb->len - sizeof(tr) +,
    }
    static void mwl8k_encapsulate_tx_frame(struct mwl8k_priv *priv,
    struct sk_buff *skb)
    {
    pub wh: *mut ieee80211_hdr,
    pub tx_info: *mut ieee80211_tx_info,
    pub key_conf: *mut ieee80211_key_conf,
    pub data_pad: c_int,
    pub 0: int head_pad =,
    pub )skb->data: *mut wh = (struct ieee80211_hdr,
    pub IEEE80211_SKB_CB(skb): tx_info =,
    pub NULL: key_conf =,
    if (ieee80211_is_data(wh.frame_control))
    pub tx_info->control.hw_key: key_conf =,
//
// Make sure the packet header is in the DMA header format (4-address
// without QoS), and add head & tail padding when HW crypto is enabled.
//
// We have the following trailer padding requirements:
// - WEP: 4 trailer bytes (ICV)
// - TKIP: 12 trailer bytes (8 MIC + 4 ICV)
// - CCMP: 8 trailer bytes (MIC)
//
    pub 0: data_pad =,
    if (key_conf != core::ptr::null_mut()) {
    pub key_conf->iv_len: head_pad =,
    switch (key_conf.cipher) {
    case WLAN_CIPHER_SUITE_WEP40:
    case WLAN_CIPHER_SUITE_WEP104:
    pub 4: data_pad =,
    case WLAN_CIPHER_SUITE_TKIP:
    pub 12: data_pad =,
    case WLAN_CIPHER_SUITE_CCMP:
    pub 8: data_pad =,
    }
    }
    pub data_pad): mwl8k_add_dma_header(priv, skb, head_pad,,
    }
//
// Packet reception for 88w8366/88w8764 AP firmware.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_rxd_ap {
    pub pkt_len: __le16,
    pub sq2: __u8,
    pub rate: __u8,
    pub pkt_phys_addr: __le32,
    pub next_rxd_phys_addr: __le32,
    pub qos_control: __le16,
    pub htsig2: __le16,
    pub hw_rssi_info: __le32,
    pub hw_noise_floor_info: __le32,
    pub noise_floor: __u8,
    pub pad0: [__u8; 3],
    pub rssi: __u8,
    pub rx_status: __u8,
    pub channel: __u8,
    pub rx_ctrl: __u8,
    pub __packed: },
pub const MWL8K_AP_RATE_INFO_MCS_FORMAT: c_uint = 0x80;
pub const MWL8K_AP_RATE_INFO_40MHZ: c_uint = 0x40;

pub const MWL8K_AP_RX_CTRL_OWNED_BY_HOST: c_uint = 0x80;
// 8366/8764 AP rx_status bits
pub const MWL8K_AP_RXSTAT_DECRYPT_ERR_MASK: c_uint = 0x80;
pub const MWL8K_AP_RXSTAT_GENERAL_DECRYPT_ERR: c_uint = 0xFF;
pub const MWL8K_AP_RXSTAT_TKIP_DECRYPT_MIC_ERR: c_uint = 0x02;
pub const MWL8K_AP_RXSTAT_WEP_DECRYPT_ICV_ERR: c_uint = 0x04;
pub const MWL8K_AP_RXSTAT_TKIP_DECRYPT_ICV_ERR: c_uint = 0x08;
#[no_mangle]
unsafe extern "C" fn mwl8k_rxd_ap_init(_rxd: *mut c_void, next_dma_addr: dma_addr_t) {
    static void mwl8k_rxd_ap_init(void *_rxd, dma_addr_t next_dma_addr)
    {
    pub _rxd: *mut *mut mwl8k_rxd_ap rxd =,
    pub cpu_to_le32(next_dma_addr): rxd->next_rxd_phys_addr =,
    pub MWL8K_AP_RX_CTRL_OWNED_BY_HOST: rxd->rx_ctrl =,
    }
#[no_mangle]
unsafe extern "C" fn mwl8k_rxd_ap_refill(_rxd: *mut c_void, addr: dma_addr_t, len: c_int) {
    static void mwl8k_rxd_ap_refill(void *_rxd, dma_addr_t addr, int len)
    {
    pub _rxd: *mut *mut mwl8k_rxd_ap rxd =,
    pub cpu_to_le16(len): rxd->pkt_len =,
    pub cpu_to_le32(addr): rxd->pkt_phys_addr =,
    pub 0: rxd->rx_ctrl =,
    }
    static int
    mwl8k_rxd_ap_process(void *_rxd, struct ieee80211_rx_status *status,
    __le16 *qos, s8 *noise)
    {
    pub _rxd: *mut *mut mwl8k_rxd_ap rxd =,
    if (!(rxd.rx_ctrl & MWL8K_AP_RX_CTRL_OWNED_BY_HOST))
    pub -1: return,
    pub sizeof(*status)): *mut memset(status, 0,,
    pub -rxd->rssi: status->signal =,
// noise = -rxd->noise_floor;
    if (rxd.rate & MWL8K_AP_RATE_INFO_MCS_FORMAT) {
    pub RX_ENC_HT: status->encoding =,
    if (rxd.rate & MWL8K_AP_RATE_INFO_40MHZ)
    pub RATE_INFO_BW_40: status->bw =,
    pub MWL8K_AP_RATE_INFO_RATEID(rxd->rate): status->rate_idx =,
    } else {
    pub i: c_int,
    pub {: for (i = 0; i < ARRAY_SIZE(mwl8k_rates_24); i++),
    if (mwl8k_rates_24[i].hw_value == rxd.rate) {
    pub i: status->rate_idx =,
    }
    }
    }
    if (rxd.channel > 14) {
    pub NL80211_BAND_5GHZ: status->band =,
    if (!(status.encoding == RX_ENC_HT) &&
    status.rate_idx >= MWL8K_LEGACY_5G_RATE_OFFSET)
    pub MWL8K_LEGACY_5G_RATE_OFFSET: status->rate_idx -=,
    } else {
    pub NL80211_BAND_2GHZ: status->band =,
    }
    status.freq = ieee80211_channel_to_frequency(rxd.channel,
// qos = rxd->qos_control;
    if ((rxd.rx_status != MWL8K_AP_RXSTAT_GENERAL_DECRYPT_ERR) &&
    (rxd.rx_status & MWL8K_AP_RXSTAT_DECRYPT_ERR_MASK) &&
    (rxd.rx_status & MWL8K_AP_RXSTAT_TKIP_DECRYPT_MIC_ERR))
    pub RX_FLAG_MMIC_ERROR: status->flag |=,
    pub le16_to_cpu(rxd->pkt_len): return,
    }
    static struct rxd_ops rxd_ap_ops = {
    .rxd_size	= sizeof(struct mwl8k_rxd_ap),
    .rxd_init	= mwl8k_rxd_ap_init,
    .rxd_refill	= mwl8k_rxd_ap_refill,
    .rxd_process	= mwl8k_rxd_ap_process,
}

//
// Packet reception for STA firmware.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_rxd_sta {
    pub pkt_len: __le16,
    pub link_quality: __u8,
    pub noise_level: __u8,
    pub pkt_phys_addr: __le32,
    pub next_rxd_phys_addr: __le32,
    pub qos_control: __le16,
    pub rate_info: __le16,
    pub pad0: [__le32; 4],
    pub rssi: __u8,
    pub channel: __u8,
    pub pad1: __le16,
    pub rx_ctrl: __u8,
    pub rx_status: __u8,
    pub pad2: [__u8; 2],
    pub __packed: },
pub const MWL8K_STA_RATE_INFO_SHORTPRE: c_uint = 0x8000;

pub const MWL8K_STA_RATE_INFO_40MHZ: c_uint = 0x0004;
pub const MWL8K_STA_RATE_INFO_SHORTGI: c_uint = 0x0002;
pub const MWL8K_STA_RATE_INFO_MCS_FORMAT: c_uint = 0x0001;
pub const MWL8K_STA_RX_CTRL_OWNED_BY_HOST: c_uint = 0x02;
pub const MWL8K_STA_RX_CTRL_DECRYPT_ERROR: c_uint = 0x04;
// ICV=0 or MIC=1
pub const MWL8K_STA_RX_CTRL_DEC_ERR_TYPE: c_uint = 0x08;
// Key is uploaded only in failure case
pub const MWL8K_STA_RX_CTRL_KEY_INDEX: c_uint = 0x30;
#[no_mangle]
unsafe extern "C" fn mwl8k_rxd_sta_init(_rxd: *mut c_void, next_dma_addr: dma_addr_t) {
    static void mwl8k_rxd_sta_init(void *_rxd, dma_addr_t next_dma_addr)
    {
    pub _rxd: *mut *mut mwl8k_rxd_sta rxd =,
    pub cpu_to_le32(next_dma_addr): rxd->next_rxd_phys_addr =,
    pub MWL8K_STA_RX_CTRL_OWNED_BY_HOST: rxd->rx_ctrl =,
    }
#[no_mangle]
unsafe extern "C" fn mwl8k_rxd_sta_refill(_rxd: *mut c_void, addr: dma_addr_t, len: c_int) {
    static void mwl8k_rxd_sta_refill(void *_rxd, dma_addr_t addr, int len)
    {
    pub _rxd: *mut *mut mwl8k_rxd_sta rxd =,
    pub cpu_to_le16(len): rxd->pkt_len =,
    pub cpu_to_le32(addr): rxd->pkt_phys_addr =,
    pub 0: rxd->rx_ctrl =,
    }
    static int
    mwl8k_rxd_sta_process(void *_rxd, struct ieee80211_rx_status *status,
    __le16 *qos, s8 *noise)
    {
    pub _rxd: *mut *mut mwl8k_rxd_sta rxd =,
    pub rate_info: u16,
    if (!(rxd.rx_ctrl & MWL8K_STA_RX_CTRL_OWNED_BY_HOST))
    pub -1: return,
    pub le16_to_cpu(rxd->rate_info): rate_info =,
    pub sizeof(*status)): *mut memset(status, 0,,
    pub -rxd->rssi: status->signal =,
// noise = -rxd->noise_level;
    pub MWL8K_STA_RATE_INFO_ANTSELECT(rate_info): status->antenna =,
    pub MWL8K_STA_RATE_INFO_RATEID(rate_info): status->rate_idx =,
    if (rate_info & MWL8K_STA_RATE_INFO_SHORTPRE)
    pub RX_ENC_FLAG_SHORTPRE: status->enc_flags |=,
    if (rate_info & MWL8K_STA_RATE_INFO_40MHZ)
    pub RATE_INFO_BW_40: status->bw =,
    if (rate_info & MWL8K_STA_RATE_INFO_SHORTGI)
    pub RX_ENC_FLAG_SHORT_GI: status->enc_flags |=,
    if (rate_info & MWL8K_STA_RATE_INFO_MCS_FORMAT)
    pub RX_ENC_HT: status->encoding =,
    if (rxd.channel > 14) {
    pub NL80211_BAND_5GHZ: status->band =,
    if (!(status.encoding == RX_ENC_HT) &&
    status.rate_idx >= MWL8K_LEGACY_5G_RATE_OFFSET)
    pub MWL8K_LEGACY_5G_RATE_OFFSET: status->rate_idx -=,
    } else {
    pub NL80211_BAND_2GHZ: status->band =,
    }
    status.freq = ieee80211_channel_to_frequency(rxd.channel,
// qos = rxd->qos_control;
    if ((rxd.rx_ctrl & MWL8K_STA_RX_CTRL_DECRYPT_ERROR) &&
    (rxd.rx_ctrl & MWL8K_STA_RX_CTRL_DEC_ERR_TYPE))
    pub RX_FLAG_MMIC_ERROR: status->flag |=,
    pub le16_to_cpu(rxd->pkt_len): return,
    }
    static struct rxd_ops rxd_sta_ops = {
    .rxd_size	= sizeof(struct mwl8k_rxd_sta),
    .rxd_init	= mwl8k_rxd_sta_init,
    .rxd_refill	= mwl8k_rxd_sta_refill,
    .rxd_process	= mwl8k_rxd_sta_process,
}

pub const MWL8K_RX_DESCS: c_int = 256;
pub const MWL8K_RX_MAXSZ: c_int = 3800;
#[no_mangle]
unsafe extern "C" fn mwl8k_rxq_init(hw: *mut ieee80211_hw, index: c_int) -> c_int {
    static int mwl8k_rxq_init(struct ieee80211_hw *hw, int index)
    {
    struct mwl8k_priv *priv = hw.priv;
    struct mwl8k_rx_queue *rxq = priv.rxq + index;
    int size;
    int i;
    rxq.rxd_count = 0;
    rxq.head = 0;
    rxq.tail = 0;
    size = MWL8K_RX_DESCS * priv.rxd_ops.rxd_size;
    rxq.rxd = dma_alloc_coherent(&priv.pdev.dev, size, &rxq.rxd_dma,
    GFP_KERNEL);
    if (rxq.rxd == core::ptr::null_mut()) {
    wiphy_err(hw.wiphy, "failed to alloc RX descriptors\n");
    return -ENOMEM;
    }
    rxq.buf = kzalloc_objs(*rxq.buf, MWL8K_RX_DESCS);
    if (rxq.buf == core::ptr::null_mut()) {
    dma_free_coherent(&priv.pdev.dev, size, rxq.rxd,
    rxq.rxd_dma);
    return -ENOMEM;
    }
    for (i = 0; i < MWL8K_RX_DESCS; i++) {
    int desc_size;
    void *rxd;
    int nexti;
    dma_addr_t next_dma_addr;
    desc_size = priv.rxd_ops.rxd_size;
    rxd = rxq.rxd + (i * priv.rxd_ops.rxd_size);
    nexti = i + 1;
    if (nexti == MWL8K_RX_DESCS)
    nexti = 0;
    next_dma_addr = rxq.rxd_dma + (nexti * desc_size);
    priv.rxd_ops.rxd_init(rxd, next_dma_addr);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rxq_refill(hw: *mut ieee80211_hw, index: c_int, limit: c_int) -> c_int {
    static int rxq_refill(struct ieee80211_hw *hw, int index, int limit)
    {
    struct mwl8k_priv *priv = hw.priv;
    struct mwl8k_rx_queue *rxq = priv.rxq + index;
    let mut refilled: c_int = 0;
    while (rxq.rxd_count < MWL8K_RX_DESCS && limit--) {
    struct sk_buff *skb;
    dma_addr_t addr;
    int rx;
    void *rxd;
    skb = dev_alloc_skb(MWL8K_RX_MAXSZ);
    if (skb == core::ptr::null_mut())
    break;
    addr = dma_map_single(&priv.pdev.dev, skb.data,
    MWL8K_RX_MAXSZ, DMA_FROM_DEVICE);
    if (dma_mapping_error(&priv.pdev.dev, addr)) {
    kfree_skb(skb);
    break;
    }
    rxq.rxd_count++;
    rx = rxq.tail++;
    if (rxq.tail == MWL8K_RX_DESCS)
    rxq.tail = 0;
    rxq.buf[rx].skb = skb;
    dma_unmap_addr_set(&rxq.buf[rx], dma, addr);
    rxd = rxq.rxd + (rx * priv.rxd_ops.rxd_size);
    priv.rxd_ops.rxd_refill(rxd, addr, MWL8K_RX_MAXSZ);
    refilled++;
    }
    return refilled;
    }
// Must be called only when the card's reception is completely halted
#[no_mangle]
unsafe extern "C" fn mwl8k_rxq_deinit(hw: *mut ieee80211_hw, index: c_int) {
    static void mwl8k_rxq_deinit(struct ieee80211_hw *hw, int index)
    {
    struct mwl8k_priv *priv = hw.priv;
    struct mwl8k_rx_queue *rxq = priv.rxq + index;
    int i;
    if (rxq.rxd == core::ptr::null_mut())
    return;
    for (i = 0; i < MWL8K_RX_DESCS; i++) {
    if (rxq.buf[i].skb != core::ptr::null_mut()) {
    dma_unmap_single(&priv.pdev.dev,
    dma_unmap_addr(&rxq.buf[i], dma),
    MWL8K_RX_MAXSZ, DMA_FROM_DEVICE);
    dma_unmap_addr_set(&rxq.buf[i], dma, 0);
    kfree_skb(rxq.buf[i].skb);
    rxq.buf[i].skb = core::ptr::null_mut();
    }
    }
    kfree(rxq.buf);
    rxq.buf = core::ptr::null_mut();
    dma_free_coherent(&priv.pdev.dev,
    MWL8K_RX_DESCS * priv.rxd_ops.rxd_size, rxq.rxd,
    rxq.rxd_dma);
    rxq.rxd = core::ptr::null_mut();
    }
//
// Scan a list of BSSIDs to process for finalize join.
// Allows for extension to process multiple BSSIDs.
//
    static inline int
    mwl8k_capture_bssid(struct mwl8k_priv *priv, struct ieee80211_hdr *wh)
    {
    return priv.capture_beacon &&
    ieee80211_is_beacon(wh.frame_control) &&
    ether_addr_equal_64bits(wh.addr3, priv.capture_bssid);
    }
    static inline void mwl8k_save_beacon(struct ieee80211_hw *hw,
    struct sk_buff *skb)
    {
    struct mwl8k_priv *priv = hw.priv;
    priv.capture_beacon = false;
    eth_zero_addr(priv.capture_bssid);
//
// Use GFP_ATOMIC as rxq_process is called from
// the primary interrupt handler, memory allocation call
// must not sleep.
//
    priv.beacon_skb = skb_copy(skb, GFP_ATOMIC);
    if (priv.beacon_skb != core::ptr::null_mut())
    ieee80211_queue_work(hw, &priv.finalize_join_worker);
    }
    static inline struct mwl8k_vif *mwl8k_find_vif_bss(struct list_head *vif_list,
    u8 *bssid)
    {
    struct mwl8k_vif *mwl8k_vif;
    list_for_each_entry(mwl8k_vif,
    vif_list, list) {
    if (memcmp(bssid, mwl8k_vif.bssid,
    ETH_ALEN) == 0)
    return mwl8k_vif;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn rxq_process(hw: *mut ieee80211_hw, index: c_int, limit: c_int) -> c_int {
    static int rxq_process(struct ieee80211_hw *hw, int index, int limit)
    {
    struct mwl8k_priv *priv = hw.priv;
    struct mwl8k_vif *mwl8k_vif = core::ptr::null_mut();
    struct mwl8k_rx_queue *rxq = priv.rxq + index;
    int processed;
    processed = 0;
    while (rxq.rxd_count && limit--) {
    struct sk_buff *skb;
    void *rxd;
    int pkt_len;
    struct ieee80211_rx_status status;
    struct ieee80211_hdr *wh;
    __le16 qos;
    skb = rxq.buf[rxq.head].skb;
    if (skb == core::ptr::null_mut())
    break;
    rxd = rxq.rxd + (rxq.head * priv.rxd_ops.rxd_size);
    pkt_len = priv.rxd_ops.rxd_process(rxd, &status, &qos,
    &priv.noise);
    if (pkt_len < 0)
    break;
    rxq.buf[rxq.head].skb = core::ptr::null_mut();
    dma_unmap_single(&priv.pdev.dev,
    dma_unmap_addr(&rxq.buf[rxq.head], dma),
    MWL8K_RX_MAXSZ, DMA_FROM_DEVICE);
    dma_unmap_addr_set(&rxq.buf[rxq.head], dma, 0);
    rxq.head++;
    if (rxq.head == MWL8K_RX_DESCS)
    rxq.head = 0;
    rxq.rxd_count--;
    wh = &((struct mwl8k_dma_data *)skb.data).wh;
//
// Check for a pending join operation.  Save a
// copy of the beacon and schedule a tasklet to
// send a FINALIZE_JOIN command to the firmware.
//
    if (mwl8k_capture_bssid(priv, (void *)skb.data))
    mwl8k_save_beacon(hw, skb);
    if (ieee80211_has_protected(wh.frame_control)) {
// Check if hw crypto has been enabled for
// this bss. If yes, set the status flags
// accordingly
//
    mwl8k_vif = mwl8k_find_vif_bss(&priv.vif_list,
    wh.addr1);
    if (mwl8k_vif != core::ptr::null_mut() &&
    mwl8k_vif.is_hw_crypto_enabled) {
//
// When MMIC ERROR is encountered
// by the firmware, payload is
// dropped and only 32 bytes of
// mwl8k Firmware header is sent
// to the host.
//
// We need to add four bytes of
// key information.  In it
// MAC80211 expects keyidx set to
// 0 for triggering Counter
// Measure of MMIC failure.
//
    if (status.flag & RX_FLAG_MMIC_ERROR) {
    struct mwl8k_dma_data *tr;
    tr = (struct mwl8k_dma_data *)skb.data;
    memset((void *)&(tr.data), 0, 4);
    pkt_len += 4;
    }
    if (!ieee80211_is_auth(wh.frame_control))
    status.flag |= RX_FLAG_IV_STRIPPED |
    RX_FLAG_DECRYPTED |
    RX_FLAG_MMIC_STRIPPED;
    }
    }
    skb_put(skb, pkt_len);
    mwl8k_remove_dma_header(skb, qos);
    memcpy(IEEE80211_SKB_RXCB(skb), &status, sizeof(status));
    ieee80211_rx_irqsafe(hw, skb);
    processed++;
    }
    return processed;
    }
//
// Packet transmission.
//
pub const MWL8K_TXD_STATUS_OK: c_uint = 0x00000001;
pub const MWL8K_TXD_STATUS_OK_RETRY: c_uint = 0x00000002;
pub const MWL8K_TXD_STATUS_OK_MORE_RETRY: c_uint = 0x00000004;
pub const MWL8K_TXD_STATUS_MULTICAST_TX: c_uint = 0x00000008;
pub const MWL8K_TXD_STATUS_FW_OWNED: c_uint = 0x80000000;
pub const MWL8K_QOS_QLEN_UNSPEC: c_uint = 0xff00;
pub const MWL8K_QOS_ACK_POLICY_MASK: c_uint = 0x0060;
pub const MWL8K_QOS_ACK_POLICY_NORMAL: c_uint = 0x0000;
pub const MWL8K_QOS_ACK_POLICY_BLOCKACK: c_uint = 0x0060;
pub const MWL8K_QOS_EOSP: c_uint = 0x0010;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_tx_desc {
    pub status: __le32,
    pub data_rate: __u8,
    pub tx_priority: __u8,
    pub qos_control: __le16,
    pub pkt_phys_addr: __le32,
    pub pkt_len: __le16,
    pub dest_MAC_addr: [__u8; ETH_ALEN],
    pub next_txd_phys_addr: __le32,
    pub timestamp: __le32,
    pub rate_info: __le16,
    pub peer_id: __u8,
    pub tx_frag_cnt: __u8,
    pub __packed: },
pub const MWL8K_TX_DESCS: c_int = 128;
#[no_mangle]
unsafe extern "C" fn mwl8k_txq_init(hw: *mut ieee80211_hw, index: c_int) -> c_int {
    static int mwl8k_txq_init(struct ieee80211_hw *hw, int index)
    {
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    pub index: *mut *mut mwl8k_tx_queue txq = priv->txq +,
    pub size: c_int,
    pub i: c_int,
    pub 0: txq->len =,
    pub 0: txq->head =,
    pub 0: txq->tail =,
    pub mwl8k_tx_desc): *mut *mut size = MWL8K_TX_DESCS  sizeof(struct,
    txq.txd = dma_alloc_coherent(&priv.pdev.dev, size, &txq.txd_dma,
    if (txq.txd == core::ptr::null_mut()) {
    pub descriptors\n"): wiphy_err(hw->wiphy, "failed to alloc TX,
    pub -ENOMEM: return,
    }
    pub MWL8K_TX_DESCS): *mut *mut txq->skb = kzalloc_objs(txq->skb,,
    if (txq.skb == core::ptr::null_mut()) {
    dma_free_coherent(&priv.pdev.dev, size, txq.txd,
    pub NULL: txq->txd =,
    pub -ENOMEM: return,
    }
    pub {: for (i = 0; i < MWL8K_TX_DESCS; i++),
    pub tx_desc: *mut mwl8k_tx_desc,
    pub nexti: c_int,
    pub i: tx_desc = txq->txd +,
    pub MWL8K_TX_DESCS: nexti = (i + 1) %,
    pub 0: tx_desc->status =,
    tx_desc.next_txd_phys_addr =
    pub sizeof(*tx_desc)): *mut *mut cpu_to_le32(txq->txd_dma + nexti,
    }
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn mwl8k_tx_start(priv: *mut mwl8k_priv) {
    static inline void mwl8k_tx_start(struct mwl8k_priv *priv)
    {
    iowrite32(MWL8K_H2A_INT_PPA_READY,
    pub MWL8K_HIU_H2A_INTERRUPT_EVENTS): priv->regs +,
    iowrite32(MWL8K_H2A_INT_DUMMY,
    pub MWL8K_HIU_H2A_INTERRUPT_EVENTS): priv->regs +,
    pub MWL8K_HIU_INT_CODE): ioread32(priv->regs +,
    }
#[no_mangle]
unsafe extern "C" fn mwl8k_dump_tx_rings(hw: *mut ieee80211_hw) {
    static void mwl8k_dump_tx_rings(struct ieee80211_hw *hw)
    {
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    pub i: c_int,
    pub {: for (i = 0; i < mwl8k_tx_queues(priv); i++),
    pub i: *mut *mut mwl8k_tx_queue txq = priv->txq +,
    pub 0: int fw_owned =,
    pub 0: int drv_owned =,
    pub 0: int unused =,
    pub desc: c_int,
    pub {: for (desc = 0; desc < MWL8K_TX_DESCS; desc++),
    pub desc: *mut *mut mwl8k_tx_desc tx_desc = txq->txd +,
    pub status: u32,
    pub le32_to_cpu(tx_desc->status): status =,
    if (status & MWL8K_TXD_STATUS_FW_OWNED)
    else
    if (tx_desc.pkt_len == 0)
    }
    wiphy_err(hw.wiphy,
    "txq[%d] len=%d head=%d tail=%d "
    "fw_owned=%d drv_owned=%d unused=%d\n",
    i,
    txq.len, txq.head, txq.tail,
    pub unused): fw_owned, drv_owned,,
    }
    }
//
// Must be called with priv->fw_mutex held and tx queues stopped.
//
pub const MWL8K_TX_WAIT_TIMEOUT_MS: c_int = 5000;
#[no_mangle]
unsafe extern "C" fn mwl8k_tx_wait_empty(hw: *mut ieee80211_hw) -> c_int {
    static int mwl8k_tx_wait_empty(struct ieee80211_hw *hw)
    {
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    pub retry: c_int,
    pub rc: c_int,
// Since fw restart is in progress, allow only the firmware
// commands from the restart code and block the other
// commands since they are going to fail in any case since
// the firmware has crashed
//
    if (priv.hw_restart_in_progress) {
    if (priv.hw_restart_owner == current)
    pub 0: return,
    else
    pub -EBUSY: return,
    }
    if (atomic_read(&priv.watchdog_event_pending))
    pub 0: return,
//
// The TX queues are stopped at this point, so this test
// doesn't need to take ->tx_lock.
//
    if (!priv.pending_tx_pkts)
    pub 0: return,
    pub 1: retry =,
    pub 0: rc =,
    pub &tx_wait: priv->tx_wait =,
    while (!rc) {
    pub oldcount: c_int,
    pub timeout: c_ulong,
    pub priv->pending_tx_pkts: oldcount =,
    timeout = wait_for_completion_timeout(&tx_wait,
    if (atomic_read(&priv.watchdog_event_pending)) {
    pub NULL: priv->tx_wait =,
    pub 0: return,
    }
    if (timeout || !priv.pending_tx_pkts) {
    if (retry)
    pub drained\n"): wiphy_notice(hw->wiphy, "tx rings,
    }
    if (retry) {
    pub 0: retry =,
    }
    if (priv.pending_tx_pkts < oldcount) {
    wiphy_notice(hw.wiphy,
    "waiting for tx rings to drain (%d . %d pkts)\n",
    pub priv->pending_tx_pkts): oldcount,,
    pub 1: retry =,
    }
    pub NULL: priv->tx_wait =,
    wiphy_err(hw.wiphy, "tx rings stuck for %d ms\n",
    pub true: priv->hw_restart_in_progress =,
    pub &priv->fw_reload): ieee80211_queue_work(hw,,
    pub -ETIMEDOUT: rc =,
    }
    pub NULL: priv->tx_wait =,
    pub rc: return,
    }

    ((status) & (MWL8K_TXD_STATUS_OK |			\
    MWL8K_TXD_STATUS_OK_RETRY |		\
    MWL8K_TXD_STATUS_OK_MORE_RETRY))
#[no_mangle]
unsafe extern "C" fn mwl8k_tid_queue_mapping(tid: u8) -> c_int {
    static int mwl8k_tid_queue_mapping(u8 tid)
    {
    pub 7): BUG_ON(tid >,
    switch (tid) {
    case 0:
    case 3:
    pub IEEE80211_AC_BE: return,
    case 1:
    case 2:
    pub IEEE80211_AC_BK: return,
    case 4:
    case 5:
    pub IEEE80211_AC_VI: return,
    case 6:
    case 7:
    pub IEEE80211_AC_VO: return,
    default:
    pub -1: return,
    }
    }
// The firmware will fill in the rate information
// for each packet that gets queued in the hardware
// and these macros will interpret that info.
//

    static int
    mwl8k_txq_reclaim(struct ieee80211_hw *hw, int index, int limit, int force)
    {
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    pub index: *mut *mut mwl8k_tx_queue txq = priv->txq +,
    pub processed: c_int,
    pub 0: processed =,
    while (txq.len > 0 && limit--) {
    pub tx: c_int,
    pub tx_desc: *mut mwl8k_tx_desc,
    pub addr: c_ulong,
    pub size: c_int,
    pub skb: *mut sk_buff,
    pub info: *mut ieee80211_tx_info,
    pub status: u32,
    pub sta: *mut ieee80211_sta,
    pub NULL: *mut *mut mwl8k_sta sta_info =,
    pub rate_info: u16,
    pub wh: *mut ieee80211_hdr,
    pub txq->head: tx =,
    pub tx: tx_desc = txq->txd +,
    pub le32_to_cpu(tx_desc->status): status =,
    if (status & MWL8K_TXD_STATUS_FW_OWNED) {
    if (!force)
    tx_desc.status &=
    }
    pub MWL8K_TX_DESCS: txq->head = (tx + 1) %,
    pub 0): BUG_ON(txq->len ==,
    pub le32_to_cpu(tx_desc->pkt_phys_addr): addr =,
    pub le16_to_cpu(tx_desc->pkt_len): size =,
    pub txq->skb[tx]: skb =,
    pub NULL: txq->skb[tx] =,
    pub NULL): BUG_ON(skb ==,
    pub DMA_TO_DEVICE): dma_unmap_single(&priv->pdev->dev, addr, size,,
    pub tx_desc->qos_control): mwl8k_remove_dma_header(skb,,
    pub skb->data: *mut *mut wh = (struct ieee80211_hdr ),
// Mark descriptor as unused
    pub 0: tx_desc->pkt_phys_addr =,
    pub 0: tx_desc->pkt_len =,
    pub IEEE80211_SKB_CB(skb): info =,
    if (ieee80211_is_data(wh.frame_control)) {
    sta = ieee80211_find_sta_by_ifaddr(hw, wh.addr1,
    if (sta) {
    pub MWL8K_STA(sta): sta_info =,
    pub NULL): BUG_ON(sta_info ==,
    pub le16_to_cpu(tx_desc->rate_info): rate_info =,
// If rate is < 6.5 Mpbs for an ht station
// do not form an ampdu. If the station is a
// legacy station (format = 0), do not form an
// ampdu
//
    if (RI_RATE_ID_MCS(rate_info) < 1 ||
    RI_FORMAT(rate_info) == 0) {
    pub false: sta_info->is_ampdu_allowed =,
    } else {
    pub true: sta_info->is_ampdu_allowed =,
    }
    }
    }
// Rate control is happening in the firmware.
// Ensure no tx rate is being reported.
//
    pub -1: info->status.rates[0].idx =,
    pub 1: info->status.rates[0].count =,
    if (MWL8K_TXD_SUCCESS(status))
    pub IEEE80211_TX_STAT_ACK: info->flags |=,
    pub skb): ieee80211_tx_status_irqsafe(hw,,
    }
    pub processed: return,
    }
// must be called only when the card's transmit is completely halted
#[no_mangle]
unsafe extern "C" fn mwl8k_txq_deinit(hw: *mut ieee80211_hw, index: c_int) {
    static void mwl8k_txq_deinit(struct ieee80211_hw *hw, int index)
    {
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    pub index: *mut *mut mwl8k_tx_queue txq = priv->txq +,
    if (txq.txd == core::ptr::null_mut())
    pub 1): mwl8k_txq_reclaim(hw, index, INT_MAX,,
    pub NULL: txq->skb =,
    dma_free_coherent(&priv.pdev.dev,
    MWL8K_TX_DESCS * sizeof(struct mwl8k_tx_desc),
    pub txq->txd_dma): txq->txd,,
    pub NULL: txq->txd =,
    }
// caller must hold priv->stream_lock when calling the stream functions
    static struct mwl8k_ampdu_stream *
    mwl8k_add_stream(struct ieee80211_hw *hw, struct ieee80211_sta *sta, u8 tid)
    {
    pub stream: *mut mwl8k_ampdu_stream,
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    pub i: c_int,
    pub {: for (i = 0; i < MWL8K_NUM_AMPDU_STREAMS; i++),
    pub &priv->ampdu[i]: stream =,
    if (stream.state == AMPDU_NO_STREAM) {
    pub sta: stream->sta =,
    pub AMPDU_STREAM_NEW: stream->state =,
    pub tid: stream->tid =,
    pub i: stream->idx =,
    wiphy_debug(hw.wiphy, "Added a new stream for %pM %d",
    pub tid): sta->addr,,
    pub stream: return,
    }
    }
    pub NULL: return,
    }
    static int
    mwl8k_start_stream(struct ieee80211_hw *hw, struct mwl8k_ampdu_stream *stream)
    {
    pub ret: c_int,
// if the stream has already been started, don't start it again
    if (stream.state != AMPDU_STREAM_NEW)
    pub 0: return,
    pub 0): ret = ieee80211_start_tx_ba_session(stream->sta, stream->tid,,
    if (ret)
    wiphy_debug(hw.wiphy, "Failed to start stream for %pM %d: "
    pub ret): "%d\n", stream->sta->addr, stream->tid,,
    else
    wiphy_debug(hw.wiphy, "Started stream for %pM %d\n",
    pub stream->tid): stream->sta->addr,,
    pub ret: return,
    }
    static void
    mwl8k_remove_stream(struct ieee80211_hw *hw, struct mwl8k_ampdu_stream *stream)
    {
    wiphy_debug(hw.wiphy, "Remove stream for %pM %d\n", stream.sta.addr,
    pub sizeof(*stream)): *mut memset(stream, 0,,
    }
    static struct mwl8k_ampdu_stream *
    mwl8k_lookup_stream(struct ieee80211_hw *hw, u8 *addr, u8 tid)
    {
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    pub i: c_int,
    pub {: for (i = 0; i < MWL8K_NUM_AMPDU_STREAMS; i++),
    pub stream: *mut mwl8k_ampdu_stream,
    pub &priv->ampdu[i]: stream =,
    if (stream.state == AMPDU_NO_STREAM)
    if (!memcmp(stream.sta.addr, addr, ETH_ALEN) &&
    stream.tid == tid)
    pub stream: return,
    }
    pub NULL: return,
    }
pub const MWL8K_AMPDU_PACKET_THRESHOLD: c_int = 64;
#[no_mangle]
pub unsafe extern "C" fn mwl8k_ampdu_allowed(sta: *mut ieee80211_sta, tid: u8) -> bool {
    static inline bool mwl8k_ampdu_allowed(struct ieee80211_sta *sta, u8 tid)
    {
    pub MWL8K_STA(sta): *mut *mut mwl8k_sta sta_info =,
    pub tx_stats: *mut tx_traffic_info,
    pub MWL8K_MAX_TID): BUG_ON(tid >=,
    pub &sta_info->tx_stats[tid]: tx_stats =,
    return sta_info.is_ampdu_allowed &&
    pub MWL8K_AMPDU_PACKET_THRESHOLD: tx_stats->pkts >,
    }
#[no_mangle]
pub unsafe extern "C" fn mwl8k_tx_count_packet(sta: *mut ieee80211_sta, tid: u8) {
    static inline void mwl8k_tx_count_packet(struct ieee80211_sta *sta, u8 tid)
    {
    pub MWL8K_STA(sta): *mut *mut mwl8k_sta sta_info =,
    pub tx_stats: *mut tx_traffic_info,
    pub MWL8K_MAX_TID): BUG_ON(tid >=,
    pub &sta_info->tx_stats[tid]: tx_stats =,
    if (tx_stats.start_time == 0)
    pub jiffies: tx_stats->start_time =,
// reset the packet count after each second elapses.  If the number of
// packets ever exceeds the ampdu_min_traffic threshold, we will allow
// an ampdu stream to be started.
//
    if (time_after(jiffies, (unsigned long)tx_stats.start_time + HZ)) {
    pub 0: tx_stats->pkts =,
    pub 0: tx_stats->start_time =,
    } else
    }
// The hardware ampdu queues start from 5.
// txpriorities for ampdu queues are
// 5 6 7 0 1 2 3 4 ie., queue 5 is highest
// and queue 3 is lowest (queue 4 is reserved)
//
pub const BA_QUEUE: c_int = 5;
    static void
    mwl8k_txq_xmit(struct ieee80211_hw *hw,
    int index,
    struct ieee80211_sta *sta,
    struct sk_buff *skb)
    {
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    pub tx_info: *mut ieee80211_tx_info,
    pub mwl8k_vif: *mut mwl8k_vif,
    pub wh: *mut ieee80211_hdr,
    pub txq: *mut mwl8k_tx_queue,
    pub tx: *mut mwl8k_tx_desc,
    pub dma: dma_addr_t,
    pub txstatus: u32,
    pub txdatarate: u8,
    pub qos: u16,
    pub txpriority: c_int,
    pub 0: u8 tid =,
    pub NULL: *mut *mut mwl8k_ampdu_stream stream =,
    pub false: bool start_ba_session =,
    pub false: bool mgmtframe =,
    pub )skb->data: *mut *mut ieee80211_mgmt mgmt = (ieee80211_mgmt,
    pub false: bool eapol_frame =,
    pub )skb->data: *mut wh = (struct ieee80211_hdr,
    if (ieee80211_is_data_qos(wh.frame_control))
    pub )ieee80211_get_qos_ctl(wh))): *mut *mut qos = le16_to_cpu(((__le16,
    else
    pub 0: qos =,
    if (skb.protocol == cpu_to_be16(ETH_P_PAE))
    pub true: eapol_frame =,
    if (ieee80211_is_mgmt(wh.frame_control))
    pub true: mgmtframe =,
    if (priv.ap_fw)
    pub skb): mwl8k_encapsulate_tx_frame(priv,,
    else
    pub 0): mwl8k_add_dma_header(priv, skb, 0,,
    pub )skb->data)->wh: *mut wh = &((struct mwl8k_dma_data,
    pub IEEE80211_SKB_CB(skb): tx_info =,
    pub MWL8K_VIF(tx_info->control.vif): mwl8k_vif =,
    if (tx_info.flags & IEEE80211_TX_CTL_ASSIGN_SEQ) {
    pub cpu_to_le16(IEEE80211_SCTL_FRAG): wh->seq_ctrl &=,
    pub cpu_to_le16(mwl8k_vif->seqno): wh->seq_ctrl |=,
    pub 0x10: mwl8k_vif->seqno +=,
    }
// Setup firmware control bit fields for each frame type.
    pub 0: txstatus =,
    pub 0: txdatarate =,
    if (ieee80211_is_mgmt(wh.frame_control) ||
    ieee80211_is_ctl(wh.frame_control)) {
    pub 0: txdatarate =,
    pub MWL8K_QOS_EOSP: qos |= MWL8K_QOS_QLEN_UNSPEC |,
    } else if (ieee80211_is_data(wh.frame_control)) {
    pub 1: txdatarate =,
    if (is_multicast_ether_addr(wh.addr1))
    pub MWL8K_TXD_STATUS_MULTICAST_TX: txstatus |=,
    pub ~MWL8K_QOS_ACK_POLICY_MASK: qos &=,
    if (tx_info.flags & IEEE80211_TX_CTL_AMPDU)
    pub MWL8K_QOS_ACK_POLICY_BLOCKACK: qos |=,
    else
    pub MWL8K_QOS_ACK_POLICY_NORMAL: qos |=,
    }
// Queue ADDBA request in the respective data queue.  While setting up
// the ampdu stream, mac80211 queues further packets for that
// particular ra/tid pair.  However, packets piled up in the hardware
// for that ra/tid pair will still go out. ADDBA request and the
// related data packets going out from different queues asynchronously
// will cause a shift in the receiver window which might result in
// ampdu packets getting dropped at the receiver after the stream has
// been setup.
//
    if (unlikely(ieee80211_is_action(wh.frame_control) &&
    mgmt.u.action.category == WLAN_CATEGORY_BACK &&
    mgmt.u.action.action_code == WLAN_ACTION_ADDBA_REQ &&
    priv.ap_fw)) {
    pub le16_to_cpu(mgmt->u.action.addba_req.capab): u16 capab =,
    pub 2: tid = (capab & IEEE80211_ADDBA_PARAM_TID_MASK) >>,
    pub mwl8k_tid_queue_mapping(tid): index =,
    }
    pub index: txpriority =,
    if (priv.ap_fw && sta && sta.deflink.ht_cap.ht_supported && !eapol_frame &&
    ieee80211_is_data_qos(wh.frame_control)) {
    pub 0xf: tid = qos &,
    pub tid): mwl8k_tx_count_packet(sta,,
    pub tid): stream = mwl8k_lookup_stream(hw, sta->addr,,
    if (stream != core::ptr::null_mut()) {
    if (stream.state == AMPDU_STREAM_ACTIVE) {
    pub MWL8K_QOS_ACK_POLICY_BLOCKACK)): WARN_ON(!(qos &,
    txpriority = (BA_QUEUE + stream.idx) %
    if (stream.idx <= 1)
    index = stream.idx +
    } else if (stream.state == AMPDU_STREAM_NEW) {
// We get here if the driver sends us packets
// after we've initiated a stream, but before
// our ampdu_action routine has been called
// with IEEE80211_AMPDU_TX_START to get the SSN
// for the ADDBA request.  So this packet can
// go out with no risk of sequence number
// mismatch.  No special handling is required.
//
    } else {
// Drop packets that would go out after the
// ADDBA request was sent but before the ADDBA
// response is received.  If we don't do this,
// the recipient would probably receive it
// after the ADDBA request with SSN 0.  This
// will cause the recipient's BA receive window
// to shift, which would cause the subsequent
// packets in the BA stream to be discarded.
// mac80211 queues our packets for us in this
// case, so this is really just a safety check.
//
    wiphy_warn(hw.wiphy,
    "Cannot send packet while ADDBA "
    pub underway.\n"): "dialog is,
    }
    } else {
// Defer calling mwl8k_start_stream so that the current
// skb can go out before the ADDBA request.  This
// prevents sequence number mismatch at the recepient
// as described above.
//
    if (mwl8k_ampdu_allowed(sta, tid)) {
    pub tid): stream = mwl8k_add_stream(hw, sta,,
    if (stream != core::ptr::null_mut())
    pub true: start_ba_session =,
    }
    }
    } else {
    pub ~MWL8K_QOS_ACK_POLICY_MASK: qos &=,
    pub MWL8K_QOS_ACK_POLICY_NORMAL: qos |=,
    }
    dma = dma_map_single(&priv.pdev.dev, skb.data, skb.len,
    if (dma_mapping_error(&priv.pdev.dev, dma)) {
    wiphy_debug(hw.wiphy,
    pub frame.\n"): "failed to dma map skb, dropping TX,
    if (start_ba_session) {
    pub stream): mwl8k_remove_stream(hw,,
    }
    }
    pub index: txq = priv->txq +,
// Mgmt frames that go out frequently are probe
// responses. Other mgmt frames got out relatively
// infrequently. Hence reserve 2 buffers so that
// other mgmt frames do not get dropped due to an
// already queued probe response in one of the
// reserved buffers.
//
    if (txq.len >= MWL8K_TX_DESCS - 2) {
    if (!mgmtframe || txq.len == MWL8K_TX_DESCS) {
    if (start_ba_session) {
    pub stream): mwl8k_remove_stream(hw,,
    }
    dma_unmap_single(&priv.pdev.dev, dma, skb.len,
    }
    }
    pub NULL): BUG_ON(txq->skb[txq->tail] !=,
    pub skb: txq->skb[txq->tail] =,
    pub txq->tail: tx = txq->txd +,
    pub txdatarate: tx->data_rate =,
    pub txpriority: tx->tx_priority =,
    pub cpu_to_le16(qos): tx->qos_control =,
    pub cpu_to_le32(dma): tx->pkt_phys_addr =,
    pub cpu_to_le16(skb->len): tx->pkt_len =,
    pub 0: tx->rate_info =,
    if (!priv.ap_fw && sta != core::ptr::null_mut())
    pub MWL8K_STA(sta)->peer_id: tx->peer_id =,
    else
    pub 0: tx->peer_id =,
    if (priv.ap_fw && ieee80211_is_data(wh.frame_control) && !eapol_frame)
    tx.timestamp = cpu_to_le32(ioread32(priv.regs +
    else
    pub 0: tx->timestamp =,
    pub txstatus): tx->status = cpu_to_le32(MWL8K_TXD_STATUS_FW_OWNED |,
    if (txq.tail == MWL8K_TX_DESCS)
    pub 0: txq->tail =,
// Initiate the ampdu session here
    if (start_ba_session) {
    if (mwl8k_start_stream(hw, stream))
    pub stream): mwl8k_remove_stream(hw,,
    }
    }
//
// Firmware access.
//
// We have the following requirements for issuing firmware commands:
// - Some commands require that the packet transmit path is idle when
// the command is issued.  (For simplicity, we'll just quiesce the
// transmit path for every command.)
// - There are certain sequences of commands that need to be issued to
// the hardware sequentially, with no other intervening commands.
//
// This leads to an implementation of a "firmware lock" as a mutex that
// can be taken recursively, and which is taken by both the low-level
// command submission function (mwl8k_post_cmd) as well as any users of
// that function that require issuing of an atomic sequence of commands,
// and quiesces the transmit path whenever it's taken.
//
#[no_mangle]
unsafe extern "C" fn mwl8k_fw_lock(hw: *mut ieee80211_hw) -> c_int {
    static int mwl8k_fw_lock(struct ieee80211_hw *hw)
    {
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    if (priv.fw_mutex_owner != current) {
    pub rc: c_int,
    pub mwl8k_tx_wait_empty(hw): rc =,
    if (rc) {
    if (!priv.hw_restart_in_progress)
    pub rc: return,
    }
    pub current: priv->fw_mutex_owner =,
    }
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn mwl8k_fw_unlock(hw: *mut ieee80211_hw) {
    static void mwl8k_fw_unlock(struct ieee80211_hw *hw)
    {
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    if (!--priv.fw_mutex_depth) {
    if (!priv.hw_restart_in_progress)
    pub NULL: priv->fw_mutex_owner =,
    }
    }
    static void mwl8k_enable_bsses(struct ieee80211_hw *hw, bool enable,
    pub bitmap): u32,
//
// Command processing.
//
// Timeout firmware commands after 10s
pub const MWL8K_CMD_TIMEOUT_MS: c_int = 10000;
#[no_mangle]
unsafe extern "C" fn mwl8k_post_cmd(hw: *mut ieee80211_hw, cmd: *mut mwl8k_cmd_pkt_hdr) -> c_int {
    static int mwl8k_post_cmd(struct ieee80211_hw *hw, struct mwl8k_cmd_pkt_hdr *cmd)
    {
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    pub priv->regs: *mut *mut void __iomem regs =,
    pub dma_addr: dma_addr_t,
    pub dma_size: c_uint,
    pub rc: c_int,
    pub 0: unsigned long time_left =,
    pub buf: [u8; 32],
    pub 0: u32 bitmap =,
    wiphy_dbg(hw.wiphy, "Posting %s [%d]\n",
    pub cmd->macid): mwl8k_cmd_name(cmd->code, buf, sizeof(buf)),,
// Before posting firmware commands that could change the hardware
// characteristics, make sure that all BSSes are stopped temporary.
// Enable these stopped BSSes after completion of the commands
//
    pub mwl8k_fw_lock(hw): rc =,
    if (rc)
    pub rc: return,
    if (priv.ap_fw && priv.running_bsses) {
    switch (le16_to_cpu(cmd.code)) {
    case MWL8K_CMD_SET_RF_CHANNEL:
    case MWL8K_CMD_RADIO_CONTROL:
    case MWL8K_CMD_RF_TX_POWER:
    case MWL8K_CMD_TX_POWER:
    case MWL8K_CMD_RF_ANTENNA:
    case MWL8K_CMD_RTS_THRESHOLD:
    case MWL8K_CMD_MIMO_CONFIG:
    pub priv->running_bsses: bitmap =,
    pub bitmap): mwl8k_enable_bsses(hw, false,,
    }
    }
    pub 0xffff: cmd->result = ( __le16),
    pub le16_to_cpu(cmd->length): dma_size =,
    dma_addr = dma_map_single(&priv.pdev.dev, cmd, dma_size,
    if (dma_mapping_error(&priv.pdev.dev, dma_addr)) {
    pub -ENOMEM: rc =,
    pub exit: goto,
    }
    pub &cmd_wait: priv->hostcmd_wait =,
    pub MWL8K_HIU_GEN_PTR): iowrite32(dma_addr, regs +,
    iowrite32(MWL8K_H2A_INT_DOORBELL,
    pub MWL8K_HIU_H2A_INTERRUPT_EVENTS): regs +,
    iowrite32(MWL8K_H2A_INT_DUMMY,
    pub MWL8K_HIU_H2A_INTERRUPT_EVENTS): regs +,
    time_left = wait_for_completion_timeout(&cmd_wait,
    pub NULL: priv->hostcmd_wait =,
    dma_unmap_single(&priv.pdev.dev, dma_addr, dma_size,
    if (!time_left) {
    wiphy_err(hw.wiphy, "Command %s timeout after %u ms\n",
    mwl8k_cmd_name(cmd.code, buf, sizeof(buf)),
    pub -ETIMEDOUT: rc =,
    } else {
    pub ms: c_int,
    pub jiffies_to_msecs(time_left): ms = MWL8K_CMD_TIMEOUT_MS -,
    pub 0: rc = cmd->result ? -EINVAL :,
    if (rc)
    wiphy_err(hw.wiphy, "Command %s error 0x%x\n",
    mwl8k_cmd_name(cmd.code, buf, sizeof(buf)),
#[no_mangle]
pub unsafe extern "C" fn if(2000: ms >) -> else {
    else if (ms > 2000)
    wiphy_notice(hw.wiphy, "Command %s took %d ms\n",
    mwl8k_cmd_name(cmd.code,
    buf, sizeof(buf)),
    }
    exit:
    if (bitmap)
    pub bitmap): mwl8k_enable_bsses(hw, true,,
    pub rc: return,
    }
    static int mwl8k_post_pervif_cmd(struct ieee80211_hw *hw,
    struct ieee80211_vif *vif,
    struct mwl8k_cmd_pkt_hdr *cmd)
    {
    if (vif != core::ptr::null_mut())
    pub MWL8K_VIF(vif)->macid: cmd->macid =,
    pub cmd): return mwl8k_post_cmd(hw,,
    }
//
// Setup code shared between STA and AP firmware images.
//
#[no_mangle]
unsafe extern "C" fn mwl8k_setup_2ghz_band(hw: *mut ieee80211_hw) {
    static void mwl8k_setup_2ghz_band(struct ieee80211_hw *hw)
    {
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    pub sizeof(mwl8k_channels_24)): BUILD_BUG_ON(sizeof(priv->channels_24) !=,
    pub sizeof(mwl8k_channels_24)): memcpy(priv->channels_24, mwl8k_channels_24,,
    pub sizeof(mwl8k_rates_24)): BUILD_BUG_ON(sizeof(priv->rates_24) !=,
    pub sizeof(mwl8k_rates_24)): memcpy(priv->rates_24, mwl8k_rates_24,,
    pub NL80211_BAND_2GHZ: priv->band_24.band =,
    pub priv->channels_24: priv->band_24.channels =,
    pub ARRAY_SIZE(mwl8k_channels_24): priv->band_24.n_channels =,
    pub priv->rates_24: priv->band_24.bitrates =,
    pub ARRAY_SIZE(mwl8k_rates_24): priv->band_24.n_bitrates =,
    pub &priv->band_24: hw->wiphy->bands[NL80211_BAND_2GHZ] =,
    }
#[no_mangle]
unsafe extern "C" fn mwl8k_setup_5ghz_band(hw: *mut ieee80211_hw) {
    static void mwl8k_setup_5ghz_band(struct ieee80211_hw *hw)
    {
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    pub sizeof(mwl8k_channels_50)): BUILD_BUG_ON(sizeof(priv->channels_50) !=,
    pub sizeof(mwl8k_channels_50)): memcpy(priv->channels_50, mwl8k_channels_50,,
    pub sizeof(mwl8k_rates_50)): BUILD_BUG_ON(sizeof(priv->rates_50) !=,
    pub sizeof(mwl8k_rates_50)): memcpy(priv->rates_50, mwl8k_rates_50,,
    pub NL80211_BAND_5GHZ: priv->band_50.band =,
    pub priv->channels_50: priv->band_50.channels =,
    pub ARRAY_SIZE(mwl8k_channels_50): priv->band_50.n_channels =,
    pub priv->rates_50: priv->band_50.bitrates =,
    pub ARRAY_SIZE(mwl8k_rates_50): priv->band_50.n_bitrates =,
    pub &priv->band_50: hw->wiphy->bands[NL80211_BAND_5GHZ] =,
    }
//
// CMD_GET_HW_SPEC (STA version).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_cmd_get_hw_spec_sta {
    pub header: mwl8k_cmd_pkt_hdr,
    pub hw_rev: __u8,
    pub host_interface: __u8,
    pub num_mcaddrs: __le16,
    pub perm_addr: [__u8; ETH_ALEN],
    pub region_code: __le16,
    pub fw_rev: __le32,
    pub ps_cookie: __le32,
    pub caps: __le32,
    pub mcs_bitmap: [__u8; 16],
    pub rx_queue_ptr: __le32,
    pub num_tx_queues: __le32,
    pub tx_queue_ptrs: [__le32; MWL8K_TX_WMM_QUEUES],
    pub caps2: __le32,
    pub num_tx_desc_per_queue: __le32,
    pub total_rxd: __le32,
    pub __packed: },
pub const MWL8K_CAP_MAX_AMSDU: c_uint = 0x20000000;
pub const MWL8K_CAP_GREENFIELD: c_uint = 0x08000000;
pub const MWL8K_CAP_AMPDU: c_uint = 0x04000000;
pub const MWL8K_CAP_RX_STBC: c_uint = 0x01000000;
pub const MWL8K_CAP_TX_STBC: c_uint = 0x00800000;
pub const MWL8K_CAP_SHORTGI_40MHZ: c_uint = 0x00400000;
pub const MWL8K_CAP_SHORTGI_20MHZ: c_uint = 0x00200000;
pub const MWL8K_CAP_RX_ANTENNA_MASK: c_uint = 0x000e0000;
pub const MWL8K_CAP_TX_ANTENNA_MASK: c_uint = 0x0001c000;
pub const MWL8K_CAP_DELAY_BA: c_uint = 0x00003000;
pub const MWL8K_CAP_MIMO: c_uint = 0x00000200;
pub const MWL8K_CAP_40MHZ: c_uint = 0x00000100;
pub const MWL8K_CAP_BAND_MASK: c_uint = 0x00000007;
pub const MWL8K_CAP_5GHZ: c_uint = 0x00000004;
pub const MWL8K_CAP_2GHZ4: c_uint = 0x00000001;
    static void
    mwl8k_set_ht_caps(struct ieee80211_hw *hw,
    struct ieee80211_supported_band *band, u32 cap)
    {
    pub rx_streams: c_int,
    pub tx_streams: c_int,
    pub 1: band->ht_cap.ht_supported =,
    if (cap & MWL8K_CAP_MAX_AMSDU)
    pub IEEE80211_HT_CAP_MAX_AMSDU: band->ht_cap.cap |=,
    if (cap & MWL8K_CAP_GREENFIELD)
    pub IEEE80211_HT_CAP_GRN_FLD: band->ht_cap.cap |=,
    if (cap & MWL8K_CAP_AMPDU) {
    pub AMPDU_AGGREGATION): ieee80211_hw_set(hw,,
    pub IEEE80211_HT_MAX_AMPDU_64K: band->ht_cap.ampdu_factor =,
    pub IEEE80211_HT_MPDU_DENSITY_NONE: band->ht_cap.ampdu_density =,
    }
    if (cap & MWL8K_CAP_RX_STBC)
    pub IEEE80211_HT_CAP_RX_STBC: band->ht_cap.cap |=,
    if (cap & MWL8K_CAP_TX_STBC)
    pub IEEE80211_HT_CAP_TX_STBC: band->ht_cap.cap |=,
    if (cap & MWL8K_CAP_SHORTGI_40MHZ)
    pub IEEE80211_HT_CAP_SGI_40: band->ht_cap.cap |=,
    if (cap & MWL8K_CAP_SHORTGI_20MHZ)
    pub IEEE80211_HT_CAP_SGI_20: band->ht_cap.cap |=,
    if (cap & MWL8K_CAP_DELAY_BA)
    pub IEEE80211_HT_CAP_DELAY_BA: band->ht_cap.cap |=,
    if (cap & MWL8K_CAP_40MHZ)
    pub IEEE80211_HT_CAP_SUP_WIDTH_20_40: band->ht_cap.cap |=,
    pub MWL8K_CAP_RX_ANTENNA_MASK): rx_streams = hweight32(cap &,
    pub MWL8K_CAP_TX_ANTENNA_MASK): tx_streams = hweight32(cap &,
    pub 0xff: band->ht_cap.mcs.rx_mask[0] =,
    if (rx_streams >= 2)
    pub 0xff: band->ht_cap.mcs.rx_mask[1] =,
    if (rx_streams >= 3)
    pub 0xff: band->ht_cap.mcs.rx_mask[2] =,
    pub 0x01: band->ht_cap.mcs.rx_mask[4] =,
    pub IEEE80211_HT_MCS_TX_DEFINED: band->ht_cap.mcs.tx_params =,
    if (rx_streams != tx_streams) {
    pub IEEE80211_HT_MCS_TX_RX_DIFF: band->ht_cap.mcs.tx_params |=,
    band.ht_cap.mcs.tx_params |= (tx_streams - 1) <<
    }
    }
    static void
    mwl8k_set_caps(struct ieee80211_hw *hw, u32 caps)
    {
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    if (priv.caps)
    if ((caps & MWL8K_CAP_2GHZ4) || !(caps & MWL8K_CAP_BAND_MASK)) {
    if (caps & MWL8K_CAP_MIMO)
    pub caps): mwl8k_set_ht_caps(hw, &priv->band_24,,
    }
    if (caps & MWL8K_CAP_5GHZ) {
    if (caps & MWL8K_CAP_MIMO)
    pub caps): mwl8k_set_ht_caps(hw, &priv->band_50,,
    }
    pub caps: priv->caps =,
    }
#[no_mangle]
unsafe extern "C" fn mwl8k_cmd_get_hw_spec_sta(hw: *mut ieee80211_hw) -> c_int {
    static int mwl8k_cmd_get_hw_spec_sta(struct ieee80211_hw *hw)
    {
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    pub cmd: *mut mwl8k_cmd_get_hw_spec_sta,
    pub rc: c_int,
    pub i: c_int,
    pub kzalloc_obj(*cmd): *mut cmd =,
    if (cmd == core::ptr::null_mut())
    pub -ENOMEM: return,
    pub cpu_to_le16(MWL8K_CMD_GET_HW_SPEC): cmd->header.code =,
    pub cpu_to_le16(sizeof(*cmd)): *mut cmd->header.length =,
    pub sizeof(cmd->perm_addr)): memset(cmd->perm_addr, 0xff,,
    pub cpu_to_le32(priv->cookie_dma): cmd->ps_cookie =,
    pub cpu_to_le32(priv->rxq[0].rxd_dma): cmd->rx_queue_ptr =,
    pub cpu_to_le32(mwl8k_tx_queues(priv)): cmd->num_tx_queues =,
    pub i++): for (i = 0; i < mwl8k_tx_queues(priv);,
    pub cpu_to_le32(priv->txq[i].txd_dma): cmd->tx_queue_ptrs[i] =,
    pub cpu_to_le32(MWL8K_TX_DESCS): cmd->num_tx_desc_per_queue =,
    pub cpu_to_le32(MWL8K_RX_DESCS): cmd->total_rxd =,
    pub &cmd->header): rc = mwl8k_post_cmd(hw,,
    if (!rc) {
    pub cmd->perm_addr): SET_IEEE80211_PERM_ADDR(hw,,
    pub le16_to_cpu(cmd->num_mcaddrs): priv->num_mcaddrs =,
    pub le32_to_cpu(cmd->fw_rev): priv->fw_rev =,
    pub cmd->hw_rev: priv->hw_rev =,
    pub le32_to_cpu(cmd->caps)): mwl8k_set_caps(hw,,
    pub 0x00000000: priv->ap_macids_supported =,
    pub 0x00000001: priv->sta_macids_supported =,
    }
    pub rc: return,
    }
//
// CMD_GET_HW_SPEC (AP version).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_cmd_get_hw_spec_ap {
    pub header: mwl8k_cmd_pkt_hdr,
    pub hw_rev: __u8,
    pub host_interface: __u8,
    pub num_wcb: __le16,
    pub num_mcaddrs: __le16,
    pub perm_addr: [__u8; ETH_ALEN],
    pub region_code: __le16,
    pub num_antenna: __le16,
    pub fw_rev: __le32,
    pub wcbbase0: __le32,
    pub rxwrptr: __le32,
    pub rxrdptr: __le32,
    pub ps_cookie: __le32,
    pub wcbbase1: __le32,
    pub wcbbase2: __le32,
    pub wcbbase3: __le32,
    pub fw_api_version: __le32,
    pub caps: __le32,
    pub num_of_ampdu_queues: __le32,
    pub wcbbase_ampdu: [__le32; MWL8K_MAX_AMPDU_QUEUES],
    pub __packed: },
#[no_mangle]
unsafe extern "C" fn mwl8k_cmd_get_hw_spec_ap(hw: *mut ieee80211_hw) -> c_int {
    static int mwl8k_cmd_get_hw_spec_ap(struct ieee80211_hw *hw)
    {
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    pub cmd: *mut mwl8k_cmd_get_hw_spec_ap,
    pub i: int rc,,
    pub api_version: u32,
    pub kzalloc_obj(*cmd): *mut cmd =,
    if (cmd == core::ptr::null_mut())
    pub -ENOMEM: return,
    pub cpu_to_le16(MWL8K_CMD_GET_HW_SPEC): cmd->header.code =,
    pub cpu_to_le16(sizeof(*cmd)): *mut cmd->header.length =,
    pub sizeof(cmd->perm_addr)): memset(cmd->perm_addr, 0xff,,
    pub cpu_to_le32(priv->cookie_dma): cmd->ps_cookie =,
    pub &cmd->header): rc = mwl8k_post_cmd(hw,,
    if (!rc) {
    pub off: c_int,
    pub le32_to_cpu(cmd->fw_api_version): api_version =,
    if (priv.device_info.fw_api_ap != api_version) {
    printk(KERN_ERR "%s: Unsupported fw API version for %s."
    "  Expected %d got %d.\n", MWL8K_NAME,
    priv.device_info.part_name,
    priv.device_info.fw_api_ap,
    pub -EINVAL: rc =,
    pub done: goto,
    }
    pub cmd->perm_addr): SET_IEEE80211_PERM_ADDR(hw,,
    pub le16_to_cpu(cmd->num_mcaddrs): priv->num_mcaddrs =,
    pub le32_to_cpu(cmd->fw_rev): priv->fw_rev =,
    pub cmd->hw_rev: priv->hw_rev =,
    pub le32_to_cpu(cmd->caps)): mwl8k_set_caps(hw,,
    pub 0x000000ff: priv->ap_macids_supported =,
    pub 0x00000100: priv->sta_macids_supported =,
    pub le32_to_cpu(cmd->num_of_ampdu_queues): priv->num_ampdu_queues =,
    if (priv.num_ampdu_queues > MWL8K_MAX_AMPDU_QUEUES) {
    wiphy_warn(hw.wiphy, "fw reported %d ampdu queues"
    " but we only support %d.\n",
    priv.num_ampdu_queues,
    pub MWL8K_MAX_AMPDU_QUEUES: priv->num_ampdu_queues =,
    }
    pub 0xffff: off = le32_to_cpu(cmd->rxwrptr) &,
    pub off): iowrite32(priv->rxq[0].rxd_dma, priv->sram +,
    pub 0xffff: off = le32_to_cpu(cmd->rxrdptr) &,
    pub off): iowrite32(priv->rxq[0].rxd_dma, priv->sram +,
    pub 0xffff: priv->txq_offset[0] = le32_to_cpu(cmd->wcbbase0) &,
    pub 0xffff: priv->txq_offset[1] = le32_to_cpu(cmd->wcbbase1) &,
    pub 0xffff: priv->txq_offset[2] = le32_to_cpu(cmd->wcbbase2) &,
    pub 0xffff: priv->txq_offset[3] = le32_to_cpu(cmd->wcbbase3) &,
    pub i++): for (i = 0; i < priv->num_ampdu_queues;,
    priv.txq_offset[i + MWL8K_TX_WMM_QUEUES] =
    pub 0xffff: le32_to_cpu(cmd->wcbbase_ampdu[i]) &,
    }
    done:
    pub rc: return,
    }
//
// CMD_SET_HW_SPEC.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_cmd_set_hw_spec {
    pub header: mwl8k_cmd_pkt_hdr,
    pub hw_rev: __u8,
    pub host_interface: __u8,
    pub num_mcaddrs: __le16,
    pub perm_addr: [__u8; ETH_ALEN],
    pub region_code: __le16,
    pub fw_rev: __le32,
    pub ps_cookie: __le32,
    pub caps: __le32,
    pub rx_queue_ptr: __le32,
    pub num_tx_queues: __le32,
    pub tx_queue_ptrs: [__le32; MWL8K_MAX_TX_QUEUES],
    pub flags: __le32,
    pub num_tx_desc_per_queue: __le32,
    pub total_rxd: __le32,
    pub __packed: },
// If enabled, MWL8K_SET_HW_SPEC_FLAG_ENABLE_LIFE_TIME_EXPIRY will cause
// packets to expire 500 ms after the timestamp in the tx descriptor.  That is,
// the packets that are queued for more than 500ms, will be dropped in the
// hardware. This helps minimizing the issues caused due to head-of-line
// blocking where a slow client can hog the bandwidth and affect traffic to a
// faster client.
//
pub const MWL8K_SET_HW_SPEC_FLAG_ENABLE_LIFE_TIME_EXPIRY: c_uint = 0x00000400;
pub const MWL8K_SET_HW_SPEC_FLAG_GENERATE_CCMP_HDR: c_uint = 0x00000200;
pub const MWL8K_SET_HW_SPEC_FLAG_HOST_DECR_MGMT: c_uint = 0x00000080;
pub const MWL8K_SET_HW_SPEC_FLAG_HOSTFORM_PROBERESP: c_uint = 0x00000020;
pub const MWL8K_SET_HW_SPEC_FLAG_HOSTFORM_BEACON: c_uint = 0x00000010;
#[no_mangle]
unsafe extern "C" fn mwl8k_cmd_set_hw_spec(hw: *mut ieee80211_hw) -> c_int {
    static int mwl8k_cmd_set_hw_spec(struct ieee80211_hw *hw)
    {
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    pub cmd: *mut mwl8k_cmd_set_hw_spec,
    pub rc: c_int,
    pub i: c_int,
    pub kzalloc_obj(*cmd): *mut cmd =,
    if (cmd == core::ptr::null_mut())
    pub -ENOMEM: return,
    pub cpu_to_le16(MWL8K_CMD_SET_HW_SPEC): cmd->header.code =,
    pub cpu_to_le16(sizeof(*cmd)): *mut cmd->header.length =,
    pub cpu_to_le32(priv->cookie_dma): cmd->ps_cookie =,
    pub cpu_to_le32(priv->rxq[0].rxd_dma): cmd->rx_queue_ptr =,
    pub cpu_to_le32(mwl8k_tx_queues(priv)): cmd->num_tx_queues =,
//
// Mac80211 stack has Q0 as highest priority and Q3 as lowest in
// that order. Firmware has Q3 as highest priority and Q0 as lowest
// in that order. Map Q3 of mac80211 to Q0 of firmware so that the
// priority is interpreted the right way in firmware.
//
    pub {: for (i = 0; i < mwl8k_tx_queues(priv); i++),
    pub i: int j = mwl8k_tx_queues(priv) - 1 -,
    pub cpu_to_le32(priv->txq[j].txd_dma): cmd->tx_queue_ptrs[i] =,
    }
    cmd.flags = cpu_to_le32(MWL8K_SET_HW_SPEC_FLAG_HOST_DECR_MGMT |
    MWL8K_SET_HW_SPEC_FLAG_HOSTFORM_PROBERESP |
    MWL8K_SET_HW_SPEC_FLAG_HOSTFORM_BEACON |
    MWL8K_SET_HW_SPEC_FLAG_ENABLE_LIFE_TIME_EXPIRY |
    pub cpu_to_le32(MWL8K_TX_DESCS): cmd->num_tx_desc_per_queue =,
    pub cpu_to_le32(MWL8K_RX_DESCS): cmd->total_rxd =,
    pub &cmd->header): rc = mwl8k_post_cmd(hw,,
    pub rc: return,
    }
//
// CMD_MAC_MULTICAST_ADR.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_cmd_mac_multicast_adr {
    pub header: mwl8k_cmd_pkt_hdr,
    pub action: __le16,
    pub numaddr: __le16,
    pub addr: [__u8; ][ETH_ALEN],
}

pub const MWL8K_ENABLE_RX_DIRECTED: c_uint = 0x0001;
pub const MWL8K_ENABLE_RX_MULTICAST: c_uint = 0x0002;
pub const MWL8K_ENABLE_RX_ALL_MULTICAST: c_uint = 0x0004;
pub const MWL8K_ENABLE_RX_BROADCAST: c_uint = 0x0008;
    static struct mwl8k_cmd_pkt_hdr *
    __mwl8k_cmd_mac_multicast_adr(struct ieee80211_hw *hw, int allmulti,
    struct netdev_hw_addr_list *mc_list)
    {
    struct mwl8k_priv *priv = hw.priv;
    struct mwl8k_cmd_mac_multicast_adr *cmd;
    int size;
    let mut mc_count: c_int = 0;
    if (mc_list)
    mc_count = netdev_hw_addr_list_count(mc_list);
    if (allmulti || mc_count > priv.num_mcaddrs) {
    allmulti = 1;
    mc_count = 0;
    }
    size = sizeof(*cmd) + mc_count * ETH_ALEN;
    cmd = kzalloc(size, GFP_ATOMIC);
    if (cmd == core::ptr::null_mut())
    return core::ptr::null_mut();
    cmd.header.code = cpu_to_le16(MWL8K_CMD_MAC_MULTICAST_ADR);
    cmd.header.length = cpu_to_le16(size);
    cmd.action = cpu_to_le16(MWL8K_ENABLE_RX_DIRECTED |
    MWL8K_ENABLE_RX_BROADCAST);
    if (allmulti) {
    cmd.action |= cpu_to_le16(MWL8K_ENABLE_RX_ALL_MULTICAST);
    } else if (mc_count) {
    struct netdev_hw_addr *ha;
    let mut i: c_int = 0;
    cmd.action |= cpu_to_le16(MWL8K_ENABLE_RX_MULTICAST);
    cmd.numaddr = cpu_to_le16(mc_count);
    netdev_hw_addr_list_for_each(ha, mc_list) {
    memcpy(cmd.addr[i++], ha.addr, ETH_ALEN);
    }
    }
    return &cmd.header;
    }
//
// CMD_GET_STAT.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_cmd_get_stat {
    pub header: mwl8k_cmd_pkt_hdr,
    pub stats: [__le32; 64],
    pub __packed: },
pub const MWL8K_STAT_ACK_FAILURE: c_int = 9;
pub const MWL8K_STAT_RTS_FAILURE: c_int = 12;
pub const MWL8K_STAT_FCS_ERROR: c_int = 24;
pub const MWL8K_STAT_RTS_SUCCESS: c_int = 11;
    static int mwl8k_cmd_get_stat(struct ieee80211_hw *hw,
    struct ieee80211_low_level_stats *stats)
    {
    pub cmd: *mut mwl8k_cmd_get_stat,
    pub rc: c_int,
    pub kzalloc_obj(*cmd): *mut cmd =,
    if (cmd == core::ptr::null_mut())
    pub -ENOMEM: return,
    pub cpu_to_le16(MWL8K_CMD_GET_STAT): cmd->header.code =,
    pub cpu_to_le16(sizeof(*cmd)): *mut cmd->header.length =,
    pub &cmd->header): rc = mwl8k_post_cmd(hw,,
    if (!rc) {
    stats.dot11ACKFailureCount =
    stats.dot11RTSFailureCount =
    stats.dot11FCSErrorCount =
    stats.dot11RTSSuccessCount =
    }
    pub rc: return,
    }
//
// CMD_RADIO_CONTROL.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_cmd_radio_control {
    pub header: mwl8k_cmd_pkt_hdr,
    pub action: __le16,
    pub control: __le16,
    pub radio_on: __le16,
    pub __packed: },
    static int
    mwl8k_cmd_radio_control(struct ieee80211_hw *hw, bool enable, bool force)
    {
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    pub cmd: *mut mwl8k_cmd_radio_control,
    pub rc: c_int,
    if (enable == priv.radio_on && !force)
    pub 0: return,
    pub kzalloc_obj(*cmd): *mut cmd =,
    if (cmd == core::ptr::null_mut())
    pub -ENOMEM: return,
    pub cpu_to_le16(MWL8K_CMD_RADIO_CONTROL): cmd->header.code =,
    pub cpu_to_le16(sizeof(*cmd)): *mut cmd->header.length =,
    pub cpu_to_le16(MWL8K_CMD_SET): cmd->action =,
    pub 1): cmd->control = cpu_to_le16(priv->radio_short_preamble ? 3 :,
    pub 0x0000): cmd->radio_on = cpu_to_le16(enable ? 0x0001 :,
    pub &cmd->header): rc = mwl8k_post_cmd(hw,,
    if (!rc)
    pub enable: priv->radio_on =,
    pub rc: return,
    }
#[no_mangle]
unsafe extern "C" fn mwl8k_cmd_radio_disable(hw: *mut ieee80211_hw) -> c_int {
    static int mwl8k_cmd_radio_disable(struct ieee80211_hw *hw)
    {
    pub 0): return mwl8k_cmd_radio_control(hw, 0,,
    }
#[no_mangle]
unsafe extern "C" fn mwl8k_cmd_radio_enable(hw: *mut ieee80211_hw) -> c_int {
    static int mwl8k_cmd_radio_enable(struct ieee80211_hw *hw)
    {
    pub 0): return mwl8k_cmd_radio_control(hw, 1,,
    }
    static int
    mwl8k_set_radio_preamble(struct ieee80211_hw *hw, bool short_preamble)
    {
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    pub short_preamble: priv->radio_short_preamble =,
    pub 1): return mwl8k_cmd_radio_control(hw, 1,,
    }
//
// CMD_RF_TX_POWER.
//
pub const MWL8K_RF_TX_POWER_LEVEL_TOTAL: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_cmd_rf_tx_power {
    pub header: mwl8k_cmd_pkt_hdr,
    pub action: __le16,
    pub support_level: __le16,
    pub current_level: __le16,
    pub reserved: __le16,
    pub power_level_list: [__le16; MWL8K_RF_TX_POWER_LEVEL_TOTAL],
    pub __packed: },
#[no_mangle]
unsafe extern "C" fn mwl8k_cmd_rf_tx_power(hw: *mut ieee80211_hw, dBm: c_int) -> c_int {
    static int mwl8k_cmd_rf_tx_power(struct ieee80211_hw *hw, int dBm)
    {
    pub cmd: *mut mwl8k_cmd_rf_tx_power,
    pub rc: c_int,
    pub kzalloc_obj(*cmd): *mut cmd =,
    if (cmd == core::ptr::null_mut())
    pub -ENOMEM: return,
    pub cpu_to_le16(MWL8K_CMD_RF_TX_POWER): cmd->header.code =,
    pub cpu_to_le16(sizeof(*cmd)): *mut cmd->header.length =,
    pub cpu_to_le16(MWL8K_CMD_SET): cmd->action =,
    pub cpu_to_le16(dBm): cmd->support_level =,
    pub &cmd->header): rc = mwl8k_post_cmd(hw,,
    pub rc: return,
    }
//
// CMD_TX_POWER.
//
pub const MWL8K_TX_POWER_LEVEL_TOTAL: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_cmd_tx_power {
    pub header: mwl8k_cmd_pkt_hdr,
    pub action: __le16,
    pub band: __le16,
    pub channel: __le16,
    pub bw: __le16,
    pub sub_ch: __le16,
    pub power_level_list: [__le16; MWL8K_TX_POWER_LEVEL_TOTAL],
    pub __packed: },
    static int mwl8k_cmd_tx_power(struct ieee80211_hw *hw,
    struct ieee80211_conf *conf,
    unsigned short pwr)
    {
    pub conf->chandef.chan: *mut *mut ieee80211_channel channel =,
    enum nl80211_channel_type channel_type =
    pub cmd: *mut mwl8k_cmd_tx_power,
    pub rc: c_int,
    pub i: c_int,
    pub kzalloc_obj(*cmd): *mut cmd =,
    if (cmd == core::ptr::null_mut())
    pub -ENOMEM: return,
    pub cpu_to_le16(MWL8K_CMD_TX_POWER): cmd->header.code =,
    pub cpu_to_le16(sizeof(*cmd)): *mut cmd->header.length =,
    pub cpu_to_le16(MWL8K_CMD_SET_LIST): cmd->action =,
    if (channel.band == NL80211_BAND_2GHZ)
    pub cpu_to_le16(0x1): cmd->band =,
#[no_mangle]
pub unsafe extern "C" fn if(NL80211_BAND_5GHZ: channel->band ==) -> else {
    else if (channel.band == NL80211_BAND_5GHZ)
    pub cpu_to_le16(0x4): cmd->band =,
    pub cpu_to_le16(channel->hw_value): cmd->channel =,
    if (channel_type == NL80211_CHAN_NO_HT ||
    channel_type == NL80211_CHAN_HT20) {
    pub cpu_to_le16(0x2): cmd->bw =,
    } else {
    pub cpu_to_le16(0x4): cmd->bw =,
    if (channel_type == NL80211_CHAN_HT40MINUS)
    pub cpu_to_le16(0x3): cmd->sub_ch =,
#[no_mangle]
pub unsafe extern "C" fn if(NL80211_CHAN_HT40PLUS: channel_type ==) -> else {
    else if (channel_type == NL80211_CHAN_HT40PLUS)
    pub cpu_to_le16(0x1): cmd->sub_ch =,
    }
    pub i++): for (i = 0; i < MWL8K_TX_POWER_LEVEL_TOTAL;,
    pub cpu_to_le16(pwr): cmd->power_level_list[i] =,
    pub &cmd->header): rc = mwl8k_post_cmd(hw,,
    pub rc: return,
    }
//
// CMD_RF_ANTENNA.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_cmd_rf_antenna {
    pub header: mwl8k_cmd_pkt_hdr,
    pub antenna: __le16,
    pub mode: __le16,
    pub __packed: },
pub const MWL8K_RF_ANTENNA_RX: c_int = 1;
pub const MWL8K_RF_ANTENNA_TX: c_int = 2;
    static int
    mwl8k_cmd_rf_antenna(struct ieee80211_hw *hw, int antenna, int mask)
    {
    pub cmd: *mut mwl8k_cmd_rf_antenna,
    pub rc: c_int,
    pub kzalloc_obj(*cmd): *mut cmd =,
    if (cmd == core::ptr::null_mut())
    pub -ENOMEM: return,
    pub cpu_to_le16(MWL8K_CMD_RF_ANTENNA): cmd->header.code =,
    pub cpu_to_le16(sizeof(*cmd)): *mut cmd->header.length =,
    pub cpu_to_le16(antenna): cmd->antenna =,
    pub cpu_to_le16(mask): cmd->mode =,
    pub &cmd->header): rc = mwl8k_post_cmd(hw,,
    pub rc: return,
    }
//
// CMD_SET_BEACON.
//
#[no_mangle]
unsafe extern "C" fn mwl8k_beacon_has_ds_params(buf: *const u8, len: c_int) -> bool {
    static bool mwl8k_beacon_has_ds_params(const u8 *buf, int len)
    {
    pub )buf: *const *const ieee80211_mgmt mgmt = (void,
    pub ies_len: c_int,
    if (len <= offsetof(struct ieee80211_mgmt, u.beacon.variable))
    pub false: return,
    pub u.beacon.variable): ies_len = len - offsetof(struct ieee80211_mgmt,,
    return cfg80211_find_ie(WLAN_EID_DS_PARAMS, mgmt.u.beacon.variable,
    pub NULL: ies_len) !=,
    }
    static void mwl8k_beacon_copy_inject_ds_params(struct ieee80211_hw *hw,
    u8 *buf_dst, const u8 *buf_src,
    int src_len)
    {
    pub )buf_src: *const *const ieee80211_mgmt mgmt = (void,
    static const u8 before_ds_params[] = {
    WLAN_EID_SSID,
    WLAN_EID_SUPP_RATES,
}

    const u8 *ies;
    int hdr_len, left, offs, pos;
    ies = mgmt.u.beacon.variable;
    hdr_len = offsetof(struct ieee80211_mgmt, u.beacon.variable);
    offs = ieee80211_ie_split(ies, src_len - hdr_len, before_ds_params,
    ARRAY_SIZE(before_ds_params), 0);
    pos = hdr_len + offs;
    left = src_len - pos;
    memcpy(buf_dst, buf_src, pos);
// Inject a DSSS Parameter Set after SSID + Supp Rates
    buf_dst[pos + 0] = WLAN_EID_DS_PARAMS;
    buf_dst[pos + 1] = 1;
    buf_dst[pos + 2] = hw.conf.chandef.chan.hw_value;
    memcpy(buf_dst + pos + 3, buf_src + pos, left);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_cmd_set_beacon {
    pub header: mwl8k_cmd_pkt_hdr,
    pub beacon_len: __le16,
    pub beacon: [__u8; ],
}

    static int mwl8k_cmd_set_beacon(struct ieee80211_hw *hw,
    struct ieee80211_vif *vif, u8 *beacon, int len)
    {
    let mut ds_params_present: bool = mwl8k_beacon_has_ds_params(beacon, len);
    struct mwl8k_cmd_set_beacon *cmd;
    int rc, final_len = len;
    if (!ds_params_present) {
//
// mwl8k firmware requires a DS Params IE with the current
// channel in AP beacons. If mac80211/hostapd does not
// include it, inject one here. IE ID + length + channel
// number = 3 bytes.
//
    final_len += 3;
    }
    cmd = kzalloc(sizeof(*cmd) + final_len, GFP_KERNEL);
    if (cmd == core::ptr::null_mut())
    return -ENOMEM;
    cmd.header.code = cpu_to_le16(MWL8K_CMD_SET_BEACON);
    cmd.header.length = cpu_to_le16(sizeof(*cmd) + final_len);
    cmd.beacon_len = cpu_to_le16(final_len);
    if (ds_params_present)
    memcpy(cmd.beacon, beacon, len);
    else
    mwl8k_beacon_copy_inject_ds_params(hw, cmd.beacon, beacon,
    len);
    rc = mwl8k_post_pervif_cmd(hw, vif, &cmd.header);
    kfree(cmd);
    return rc;
    }
//
// CMD_SET_PRE_SCAN.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_cmd_set_pre_scan {
    pub header: mwl8k_cmd_pkt_hdr,
    pub __packed: },
#[no_mangle]
unsafe extern "C" fn mwl8k_cmd_set_pre_scan(hw: *mut ieee80211_hw) -> c_int {
    static int mwl8k_cmd_set_pre_scan(struct ieee80211_hw *hw)
    {
    pub cmd: *mut mwl8k_cmd_set_pre_scan,
    pub rc: c_int,
    pub kzalloc_obj(*cmd): *mut cmd =,
    if (cmd == core::ptr::null_mut())
    pub -ENOMEM: return,
    pub cpu_to_le16(MWL8K_CMD_SET_PRE_SCAN): cmd->header.code =,
    pub cpu_to_le16(sizeof(*cmd)): *mut cmd->header.length =,
    pub &cmd->header): rc = mwl8k_post_cmd(hw,,
    pub rc: return,
    }
//
// CMD_BBP_REG_ACCESS.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_cmd_bbp_reg_access {
    pub header: mwl8k_cmd_pkt_hdr,
    pub action: __le16,
    pub offset: __le16,
    pub value: u8,
    pub rsrv: [u8; 3],
    pub __packed: },
    static int
    mwl8k_cmd_bbp_reg_access(struct ieee80211_hw *hw,
    u16 action,
    u16 offset,
    u8 *value)
    {
    pub cmd: *mut mwl8k_cmd_bbp_reg_access,
    pub rc: c_int,
    pub kzalloc_obj(*cmd): *mut cmd =,
    if (cmd == core::ptr::null_mut())
    pub -ENOMEM: return,
    pub cpu_to_le16(MWL8K_CMD_BBP_REG_ACCESS): cmd->header.code =,
    pub cpu_to_le16(sizeof(*cmd)): *mut cmd->header.length =,
    pub cpu_to_le16(action): cmd->action =,
    pub cpu_to_le16(offset): cmd->offset =,
    pub &cmd->header): rc = mwl8k_post_cmd(hw,,
    if (!rc)
// value = cmd->value;
    else
// value = 0;
    pub rc: return,
    }
//
// CMD_SET_POST_SCAN.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_cmd_set_post_scan {
    pub header: mwl8k_cmd_pkt_hdr,
    pub isibss: __le32,
    pub bssid: [__u8; ETH_ALEN],
    pub __packed: },
    static int
    mwl8k_cmd_set_post_scan(struct ieee80211_hw *hw, const __u8 *mac)
    {
    pub cmd: *mut mwl8k_cmd_set_post_scan,
    pub rc: c_int,
    pub kzalloc_obj(*cmd): *mut cmd =,
    if (cmd == core::ptr::null_mut())
    pub -ENOMEM: return,
    pub cpu_to_le16(MWL8K_CMD_SET_POST_SCAN): cmd->header.code =,
    pub cpu_to_le16(sizeof(*cmd)): *mut cmd->header.length =,
    pub 0: cmd->isibss =,
    pub ETH_ALEN): memcpy(cmd->bssid, mac,,
    pub &cmd->header): rc = mwl8k_post_cmd(hw,,
    pub rc: return,
    }
#[no_mangle]
unsafe extern "C" fn freq_to_idx(priv: *mut mwl8k_priv, freq: c_int) -> c_int {
    static int freq_to_idx(struct mwl8k_priv *priv, int freq)
    {
    pub sband: *mut ieee80211_supported_band,
    pub 0: int band, ch, idx =,
    pub {: for (band = NL80211_BAND_2GHZ; band < NUM_NL80211_BANDS; band++),
    pub priv->hw->wiphy->bands[band]: sband =,
    if (!sband)
    pub idx++): for (ch = 0; ch < sband->n_channels; ch++,,
    if (sband.channels[ch].center_freq == freq)
    pub exit: goto,
    }
    exit:
    pub idx: return,
    }
    static void mwl8k_update_survey(struct mwl8k_priv *priv,
    struct ieee80211_channel *channel)
    {
    pub rx_rdy: u32 cca_cnt,,
    pub idx: s8 nf = 0,,
    pub survey: *mut survey_info,
    pub priv->acs_chan->center_freq): idx = freq_to_idx(priv,,
    if (idx >= MWL8K_NUM_CHANS) {
    pub survey\n"): wiphy_err(priv->hw->wiphy, "Failed to update,
    }
    pub &priv->survey[idx]: survey =,
    pub NOK_CCA_CNT_REG): cca_cnt = ioread32(priv->regs +,
    pub /: *mut *mut cca_cnt /= 1000; / uSecs to mSecs,
    pub cca_cnt: survey->time_busy = (u64),
    pub BBU_RXRDY_CNT_REG): rx_rdy = ioread32(priv->regs +,
    pub /: *mut *mut rx_rdy /= 1000; / uSecs to mSecs,
    pub rx_rdy: survey->time_rx = (u64),
    pub priv->channel_time: priv->channel_time = jiffies -,
    pub jiffies_to_msecs(priv->channel_time): survey->time =,
    pub channel: survey->channel =,
    pub &nf): mwl8k_cmd_bbp_reg_access(priv->hw, 0, BBU_AVG_NOISE_VAL,,
// Make sure sign is negative else ACS  at hostapd fails
    pub -1: *mut *mut survey->noise = nf,
    survey.filled = SURVEY_INFO_NOISE_DBM |
    SURVEY_INFO_TIME |
    SURVEY_INFO_TIME_BUSY |
    }
//
// CMD_SET_RF_CHANNEL.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_cmd_set_rf_channel {
    pub header: mwl8k_cmd_pkt_hdr,
    pub action: __le16,
    pub current_channel: __u8,
    pub channel_flags: __le32,
    pub __packed: },
    static int mwl8k_cmd_set_rf_channel(struct ieee80211_hw *hw,
    struct ieee80211_conf *conf)
    {
    pub conf->chandef.chan: *mut *mut ieee80211_channel channel =,
    enum nl80211_channel_type channel_type =
    pub cmd: *mut mwl8k_cmd_set_rf_channel,
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    pub rc: c_int,
    pub kzalloc_obj(*cmd): *mut cmd =,
    if (cmd == core::ptr::null_mut())
    pub -ENOMEM: return,
    pub cpu_to_le16(MWL8K_CMD_SET_RF_CHANNEL): cmd->header.code =,
    pub cpu_to_le16(sizeof(*cmd)): *mut cmd->header.length =,
    pub cpu_to_le16(MWL8K_CMD_SET): cmd->action =,
    pub channel->hw_value: cmd->current_channel =,
    if (channel.band == NL80211_BAND_2GHZ)
    pub cpu_to_le32(0x00000001): cmd->channel_flags |=,
#[no_mangle]
pub unsafe extern "C" fn if(NL80211_BAND_5GHZ: channel->band ==) -> else {
    else if (channel.band == NL80211_BAND_5GHZ)
    pub cpu_to_le32(0x00000004): cmd->channel_flags |=,
    if (!priv.sw_scan_start) {
    if (channel_type == NL80211_CHAN_NO_HT ||
    channel_type == NL80211_CHAN_HT20)
    pub cpu_to_le32(0x00000080): cmd->channel_flags |=,
#[no_mangle]
pub unsafe extern "C" fn if(NL80211_CHAN_HT40MINUS: channel_type ==) -> else {
    else if (channel_type == NL80211_CHAN_HT40MINUS)
    pub cpu_to_le32(0x000001900): cmd->channel_flags |=,
#[no_mangle]
pub unsafe extern "C" fn if(NL80211_CHAN_HT40PLUS: channel_type ==) -> else {
    else if (channel_type == NL80211_CHAN_HT40PLUS)
    pub cpu_to_le32(0x000000900): cmd->channel_flags |=,
    } else {
    pub cpu_to_le32(0x00000080): cmd->channel_flags |=,
    }
    if (priv.sw_scan_start) {
// Store current channel stats
// before switching to newer one.
// This will be processed only for AP fw.
//
    if (priv.channel_time != 0)
    pub priv->acs_chan): mwl8k_update_survey(priv,,
    pub jiffies: priv->channel_time =,
    pub channel: priv->acs_chan =,
    }
    pub &cmd->header): rc = mwl8k_post_cmd(hw,,
    pub rc: return,
    }
//
// CMD_SET_AID.
//
pub const MWL8K_FRAME_PROT_DISABLED: c_uint = 0x00;
pub const MWL8K_FRAME_PROT_11G: c_uint = 0x07;
pub const MWL8K_FRAME_PROT_11N_HT_40MHZ_ONLY: c_uint = 0x02;
pub const MWL8K_FRAME_PROT_11N_HT_ALL: c_uint = 0x06;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_cmd_update_set_aid {
    pub header: mwl8k_cmd_pkt_hdr,
    pub aid: __le16,
// AP's MAC address (BSSID)
    pub bssid: [__u8; ETH_ALEN],
    pub protection_mode: __le16,
    pub supp_rates: [__u8; 14],
    pub __packed: },
#[no_mangle]
unsafe extern "C" fn legacy_rate_mask_to_array(rates: *mut u8, mask: u32) {
    static void legacy_rate_mask_to_array(u8 *rates, u32 mask)
    {
    pub i: c_int,
    pub j: c_int,
//
// Clear nonstandard rate 4.
//
    pub 0x1fef: mask &=,
    pub {: for (i = 0, j = 0; i < 13; i++),
    if (mask & (1 << i))
    pub mwl8k_rates_24[i].hw_value: rates[j++] =,
    }
    }
    static int
    mwl8k_cmd_set_aid(struct ieee80211_hw *hw,
    struct ieee80211_vif *vif, u32 legacy_rate_mask)
    {
    pub cmd: *mut mwl8k_cmd_update_set_aid,
    pub prot_mode: u16,
    pub rc: c_int,
    pub kzalloc_obj(*cmd): *mut cmd =,
    if (cmd == core::ptr::null_mut())
    pub -ENOMEM: return,
    pub cpu_to_le16(MWL8K_CMD_SET_AID): cmd->header.code =,
    pub cpu_to_le16(sizeof(*cmd)): *mut cmd->header.length =,
    pub cpu_to_le16(vif->cfg.aid): cmd->aid =,
    pub ETH_ALEN): memcpy(cmd->bssid, vif->bss_conf.bssid,,
    if (vif.bss_conf.use_cts_prot) {
    pub MWL8K_FRAME_PROT_11G: prot_mode =,
    } else {
    switch (vif.bss_conf.ht_operation_mode &
    IEEE80211_HT_OP_MODE_PROTECTION) {
    case IEEE80211_HT_OP_MODE_PROTECTION_20MHZ:
    pub MWL8K_FRAME_PROT_11N_HT_40MHZ_ONLY: prot_mode =,
    case IEEE80211_HT_OP_MODE_PROTECTION_NONHT_MIXED:
    pub MWL8K_FRAME_PROT_11N_HT_ALL: prot_mode =,
    default:
    pub MWL8K_FRAME_PROT_DISABLED: prot_mode =,
    }
    }
    pub cpu_to_le16(prot_mode): cmd->protection_mode =,
    pub legacy_rate_mask): legacy_rate_mask_to_array(cmd->supp_rates,,
    pub &cmd->header): rc = mwl8k_post_cmd(hw,,
    pub rc: return,
    }
//
// CMD_SET_RATE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_cmd_set_rate {
    pub header: mwl8k_cmd_pkt_hdr,
    pub legacy_rates: [__u8; 14],
// Bitmap for supported MCS codes.
    pub mcs_set: [__u8; 16],
    pub reserved: [__u8; 16],
    pub __packed: },
    static int
    mwl8k_cmd_set_rate(struct ieee80211_hw *hw, struct ieee80211_vif *vif,
    u32 legacy_rate_mask, u8 *mcs_rates)
    {
    pub cmd: *mut mwl8k_cmd_set_rate,
    pub rc: c_int,
    pub kzalloc_obj(*cmd): *mut cmd =,
    if (cmd == core::ptr::null_mut())
    pub -ENOMEM: return,
    pub cpu_to_le16(MWL8K_CMD_SET_RATE): cmd->header.code =,
    pub cpu_to_le16(sizeof(*cmd)): *mut cmd->header.length =,
    pub legacy_rate_mask): legacy_rate_mask_to_array(cmd->legacy_rates,,
    pub 16): memcpy(cmd->mcs_set, mcs_rates,,
    pub &cmd->header): rc = mwl8k_post_cmd(hw,,
    pub rc: return,
    }
//
// CMD_FINALIZE_JOIN.
//
pub const MWL8K_FJ_BEACON_MAXLEN: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_cmd_finalize_join {
    pub header: mwl8k_cmd_pkt_hdr,
    pub /: *mut *mut __le32 sleep_interval; / Number of beacon periods to sleep,
    pub beacon_data: [__u8; MWL8K_FJ_BEACON_MAXLEN],
    pub __packed: },
    static int mwl8k_cmd_finalize_join(struct ieee80211_hw *hw, void *frame,
    int framelen, int dtim)
    {
    pub cmd: *mut mwl8k_cmd_finalize_join,
    pub frame: *mut *mut ieee80211_mgmt payload =,
    pub payload_len: c_int,
    pub rc: c_int,
    pub kzalloc_obj(*cmd): *mut cmd =,
    if (cmd == core::ptr::null_mut())
    pub -ENOMEM: return,
    pub cpu_to_le16(MWL8K_CMD_SET_FINALIZE_JOIN): cmd->header.code =,
    pub cpu_to_le16(sizeof(*cmd)): *mut cmd->header.length =,
    pub 1): cmd->sleep_interval = cpu_to_le32(dtim ? dtim :,
    pub ieee80211_hdrlen(payload->frame_control): payload_len = framelen -,
    if (payload_len < 0)
    pub 0: payload_len =,
#[no_mangle]
pub unsafe extern "C" fn if(MWL8K_FJ_BEACON_MAXLEN: payload_len >) -> else {
    else if (payload_len > MWL8K_FJ_BEACON_MAXLEN)
    pub MWL8K_FJ_BEACON_MAXLEN: payload_len =,
    pub payload_len): memcpy(cmd->beacon_data, &payload->u.beacon,,
    pub &cmd->header): rc = mwl8k_post_cmd(hw,,
    pub rc: return,
    }
//
// CMD_SET_RTS_THRESHOLD.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_cmd_set_rts_threshold {
    pub header: mwl8k_cmd_pkt_hdr,
    pub action: __le16,
    pub threshold: __le16,
    pub __packed: },
    static int
    mwl8k_cmd_set_rts_threshold(struct ieee80211_hw *hw, int radio_idx,
    int rts_thresh)
    {
    pub cmd: *mut mwl8k_cmd_set_rts_threshold,
    pub rc: c_int,
    pub kzalloc_obj(*cmd): *mut cmd =,
    if (cmd == core::ptr::null_mut())
    pub -ENOMEM: return,
    pub cpu_to_le16(MWL8K_CMD_RTS_THRESHOLD): cmd->header.code =,
    pub cpu_to_le16(sizeof(*cmd)): *mut cmd->header.length =,
    pub cpu_to_le16(MWL8K_CMD_SET): cmd->action =,
    pub cpu_to_le16(rts_thresh): cmd->threshold =,
    pub &cmd->header): rc = mwl8k_post_cmd(hw,,
    pub rc: return,
    }
//
// CMD_SET_SLOT.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_cmd_set_slot {
    pub header: mwl8k_cmd_pkt_hdr,
    pub action: __le16,
    pub short_slot: __u8,
    pub __packed: },
#[no_mangle]
unsafe extern "C" fn mwl8k_cmd_set_slot(hw: *mut ieee80211_hw, short_slot_time: bool) -> c_int {
    static int mwl8k_cmd_set_slot(struct ieee80211_hw *hw, bool short_slot_time)
    {
    pub cmd: *mut mwl8k_cmd_set_slot,
    pub rc: c_int,
    pub kzalloc_obj(*cmd): *mut cmd =,
    if (cmd == core::ptr::null_mut())
    pub -ENOMEM: return,
    pub cpu_to_le16(MWL8K_CMD_SET_SLOT): cmd->header.code =,
    pub cpu_to_le16(sizeof(*cmd)): *mut cmd->header.length =,
    pub cpu_to_le16(MWL8K_CMD_SET): cmd->action =,
    pub short_slot_time: cmd->short_slot =,
    pub &cmd->header): rc = mwl8k_post_cmd(hw,,
    pub rc: return,
    }
//
// CMD_SET_EDCA_PARAMS.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_cmd_set_edca_params {
    pub header: mwl8k_cmd_pkt_hdr,
// See MWL8K_SET_EDCA_XXX below
    pub action: __le16,
// TX opportunity in units of 32 us
    pub txop: __le16,
    union {
    struct {
// Log exponent of max contention period: 0...15
    pub log_cw_max: __le32,
// Log exponent of min contention period: 0...15
    pub log_cw_min: __le32,
// Adaptive interframe spacing in units of 32us
    pub aifs: __u8,
// TX queue to configure
    pub txq: __u8,
    pub ap: },
    struct {
// Log exponent of max contention period: 0...15
    pub log_cw_max: __u8,
// Log exponent of min contention period: 0...15
    pub log_cw_min: __u8,
// Adaptive interframe spacing in units of 32us
    pub aifs: __u8,
// TX queue to configure
    pub txq: __u8,
    pub sta: },
}

    } __packed;
pub const MWL8K_SET_EDCA_CW: c_uint = 0x01;
pub const MWL8K_SET_EDCA_TXOP: c_uint = 0x02;
pub const MWL8K_SET_EDCA_AIFS: c_uint = 0x04;

    MWL8K_SET_EDCA_TXOP | \
    MWL8K_SET_EDCA_AIFS)
    static int
    mwl8k_cmd_set_edca_params(struct ieee80211_hw *hw, __u8 qnum,
    __u16 cw_min, __u16 cw_max,
    __u8 aifs, __u16 txop)
    {
    struct mwl8k_priv *priv = hw.priv;
    struct mwl8k_cmd_set_edca_params *cmd;
    int rc;
    cmd = kzalloc_obj(*cmd);
    if (cmd == core::ptr::null_mut())
    return -ENOMEM;
    cmd.header.code = cpu_to_le16(MWL8K_CMD_SET_EDCA_PARAMS);
    cmd.header.length = cpu_to_le16(sizeof(*cmd));
    cmd.action = cpu_to_le16(MWL8K_SET_EDCA_ALL);
    cmd.txop = cpu_to_le16(txop);
    if (priv.ap_fw) {
    cmd.ap.log_cw_max = cpu_to_le32(ilog2(cw_max + 1));
    cmd.ap.log_cw_min = cpu_to_le32(ilog2(cw_min + 1));
    cmd.ap.aifs = aifs;
    cmd.ap.txq = qnum;
    } else {
    cmd.sta.log_cw_max = (u8)ilog2(cw_max + 1);
    cmd.sta.log_cw_min = (u8)ilog2(cw_min + 1);
    cmd.sta.aifs = aifs;
    cmd.sta.txq = qnum;
    }
    rc = mwl8k_post_cmd(hw, &cmd.header);
    kfree(cmd);
    return rc;
    }
//
// CMD_SET_WMM_MODE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_cmd_set_wmm_mode {
    pub header: mwl8k_cmd_pkt_hdr,
    pub action: __le16,
    pub __packed: },
#[no_mangle]
unsafe extern "C" fn mwl8k_cmd_set_wmm_mode(hw: *mut ieee80211_hw, enable: bool) -> c_int {
    static int mwl8k_cmd_set_wmm_mode(struct ieee80211_hw *hw, bool enable)
    {
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    pub cmd: *mut mwl8k_cmd_set_wmm_mode,
    pub rc: c_int,
    pub kzalloc_obj(*cmd): *mut cmd =,
    if (cmd == core::ptr::null_mut())
    pub -ENOMEM: return,
    pub cpu_to_le16(MWL8K_CMD_SET_WMM_MODE): cmd->header.code =,
    pub cpu_to_le16(sizeof(*cmd)): *mut cmd->header.length =,
    pub cpu_to_le16(!!enable): cmd->action =,
    pub &cmd->header): rc = mwl8k_post_cmd(hw,,
    if (!rc)
    pub enable: priv->wmm_enabled =,
    pub rc: return,
    }
//
// CMD_MIMO_CONFIG.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_cmd_mimo_config {
    pub header: mwl8k_cmd_pkt_hdr,
    pub action: __le32,
    pub rx_antenna_map: __u8,
    pub tx_antenna_map: __u8,
    pub __packed: },
#[no_mangle]
unsafe extern "C" fn mwl8k_cmd_mimo_config(hw: *mut ieee80211_hw, rx: __u8, tx: __u8) -> c_int {
    static int mwl8k_cmd_mimo_config(struct ieee80211_hw *hw, __u8 rx, __u8 tx)
    {
    pub cmd: *mut mwl8k_cmd_mimo_config,
    pub rc: c_int,
    pub kzalloc_obj(*cmd): *mut cmd =,
    if (cmd == core::ptr::null_mut())
    pub -ENOMEM: return,
    pub cpu_to_le16(MWL8K_CMD_MIMO_CONFIG): cmd->header.code =,
    pub cpu_to_le16(sizeof(*cmd)): *mut cmd->header.length =,
    pub cpu_to_le32((u32)MWL8K_CMD_SET): cmd->action =,
    pub rx: cmd->rx_antenna_map =,
    pub tx: cmd->tx_antenna_map =,
    pub &cmd->header): rc = mwl8k_post_cmd(hw,,
    pub rc: return,
    }
//
// CMD_USE_FIXED_RATE (STA version).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_cmd_use_fixed_rate_sta {
    pub header: mwl8k_cmd_pkt_hdr,
    pub action: __le32,
    pub allow_rate_drop: __le32,
    pub num_rates: __le32,
    struct {
    pub is_ht_rate: __le32,
    pub enable_retry: __le32,
    pub rate: __le32,
    pub retry_count: __le32,
    pub rate_entry: [}; 8],
    pub rate_type: __le32,
    pub reserved1: __le32,
    pub reserved2: __le32,
    pub __packed: },
pub const MWL8K_USE_AUTO_RATE: c_uint = 0x0002;
pub const MWL8K_UCAST_RATE: c_int = 0;
#[no_mangle]
unsafe extern "C" fn mwl8k_cmd_use_fixed_rate_sta(hw: *mut ieee80211_hw) -> c_int {
    static int mwl8k_cmd_use_fixed_rate_sta(struct ieee80211_hw *hw)
    {
    pub cmd: *mut mwl8k_cmd_use_fixed_rate_sta,
    pub rc: c_int,
    pub kzalloc_obj(*cmd): *mut cmd =,
    if (cmd == core::ptr::null_mut())
    pub -ENOMEM: return,
    pub cpu_to_le16(MWL8K_CMD_USE_FIXED_RATE): cmd->header.code =,
    pub cpu_to_le16(sizeof(*cmd)): *mut cmd->header.length =,
    pub cpu_to_le32(MWL8K_USE_AUTO_RATE): cmd->action =,
    pub cpu_to_le32(MWL8K_UCAST_RATE): cmd->rate_type =,
    pub &cmd->header): rc = mwl8k_post_cmd(hw,,
    pub rc: return,
    }
//
// CMD_USE_FIXED_RATE (AP version).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_cmd_use_fixed_rate_ap {
    pub header: mwl8k_cmd_pkt_hdr,
    pub action: __le32,
    pub allow_rate_drop: __le32,
    pub num_rates: __le32,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_rate_entry_ap {
    pub is_ht_rate: __le32,
    pub enable_retry: __le32,
    pub rate: __le32,
    pub retry_count: __le32,
    pub rate_entry: [}; 4],
    pub multicast_rate: u8,
    pub multicast_rate_type: u8,
    pub management_rate: u8,
    pub __packed: },
    static int
    mwl8k_cmd_use_fixed_rate_ap(struct ieee80211_hw *hw, int mcast, int mgmt)
    {
    pub cmd: *mut mwl8k_cmd_use_fixed_rate_ap,
    pub rc: c_int,
    pub kzalloc_obj(*cmd): *mut cmd =,
    if (cmd == core::ptr::null_mut())
    pub -ENOMEM: return,
    pub cpu_to_le16(MWL8K_CMD_USE_FIXED_RATE): cmd->header.code =,
    pub cpu_to_le16(sizeof(*cmd)): *mut cmd->header.length =,
    pub cpu_to_le32(MWL8K_USE_AUTO_RATE): cmd->action =,
    pub mcast: cmd->multicast_rate =,
    pub mgmt: cmd->management_rate =,
    pub &cmd->header): rc = mwl8k_post_cmd(hw,,
    pub rc: return,
    }
//
// CMD_ENABLE_SNIFFER.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_cmd_enable_sniffer {
    pub header: mwl8k_cmd_pkt_hdr,
    pub action: __le32,
    pub __packed: },
#[no_mangle]
unsafe extern "C" fn mwl8k_cmd_enable_sniffer(hw: *mut ieee80211_hw, enable: bool) -> c_int {
    static int mwl8k_cmd_enable_sniffer(struct ieee80211_hw *hw, bool enable)
    {
    pub cmd: *mut mwl8k_cmd_enable_sniffer,
    pub rc: c_int,
    pub kzalloc_obj(*cmd): *mut cmd =,
    if (cmd == core::ptr::null_mut())
    pub -ENOMEM: return,
    pub cpu_to_le16(MWL8K_CMD_ENABLE_SNIFFER): cmd->header.code =,
    pub cpu_to_le16(sizeof(*cmd)): *mut cmd->header.length =,
    pub cpu_to_le32(!!enable): cmd->action =,
    pub &cmd->header): rc = mwl8k_post_cmd(hw,,
    pub rc: return,
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_cmd_update_mac_addr {
    pub header: mwl8k_cmd_pkt_hdr,
    union {
    struct {
    pub mac_type: __le16,
    pub mac_addr: [__u8; ETH_ALEN],
    pub mbss: },
    pub mac_addr: [__u8; ETH_ALEN],
}

    } __packed;
pub const MWL8K_MAC_TYPE_PRIMARY_CLIENT: c_int = 0;
pub const MWL8K_MAC_TYPE_SECONDARY_CLIENT: c_int = 1;
pub const MWL8K_MAC_TYPE_PRIMARY_AP: c_int = 2;
pub const MWL8K_MAC_TYPE_SECONDARY_AP: c_int = 3;
    static int mwl8k_cmd_update_mac_addr(struct ieee80211_hw *hw,
    struct ieee80211_vif *vif, u8 *mac, bool set)
    {
    struct mwl8k_priv *priv = hw.priv;
    struct mwl8k_vif *mwl8k_vif = MWL8K_VIF(vif);
    struct mwl8k_cmd_update_mac_addr *cmd;
    int mac_type;
    int rc;
    mac_type = MWL8K_MAC_TYPE_PRIMARY_AP;
    if (vif != core::ptr::null_mut() && vif.type == NL80211_IFTYPE_STATION) {
    if (mwl8k_vif.macid + 1 == ffs(priv.sta_macids_supported))
    if (priv.ap_fw)
    mac_type = MWL8K_MAC_TYPE_SECONDARY_CLIENT;
    else
    mac_type = MWL8K_MAC_TYPE_PRIMARY_CLIENT;
    else
    mac_type = MWL8K_MAC_TYPE_SECONDARY_CLIENT;
    } else if (vif != core::ptr::null_mut() && vif.type == NL80211_IFTYPE_AP) {
    if (mwl8k_vif.macid + 1 == ffs(priv.ap_macids_supported))
    mac_type = MWL8K_MAC_TYPE_PRIMARY_AP;
    else
    mac_type = MWL8K_MAC_TYPE_SECONDARY_AP;
    }
    cmd = kzalloc_obj(*cmd);
    if (cmd == core::ptr::null_mut())
    return -ENOMEM;
    if (set)
    cmd.header.code = cpu_to_le16(MWL8K_CMD_SET_MAC_ADDR);
    else
    cmd.header.code = cpu_to_le16(MWL8K_CMD_DEL_MAC_ADDR);
    cmd.header.length = cpu_to_le16(sizeof(*cmd));
    if (priv.ap_fw) {
    cmd.mbss.mac_type = cpu_to_le16(mac_type);
    memcpy(cmd.mbss.mac_addr, mac, ETH_ALEN);
    } else {
    memcpy(cmd.mac_addr, mac, ETH_ALEN);
    }
    rc = mwl8k_post_pervif_cmd(hw, vif, &cmd.header);
    kfree(cmd);
    return rc;
    }
//
// MWL8K_CMD_SET_MAC_ADDR.
//
    static inline int mwl8k_cmd_set_mac_addr(struct ieee80211_hw *hw,
    struct ieee80211_vif *vif, u8 *mac)
    {
    return mwl8k_cmd_update_mac_addr(hw, vif, mac, true);
    }
//
// MWL8K_CMD_DEL_MAC_ADDR.
//
    static inline int mwl8k_cmd_del_mac_addr(struct ieee80211_hw *hw,
    struct ieee80211_vif *vif, u8 *mac)
    {
    return mwl8k_cmd_update_mac_addr(hw, vif, mac, false);
    }
//
// CMD_SET_RATEADAPT_MODE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_cmd_set_rate_adapt_mode {
    pub header: mwl8k_cmd_pkt_hdr,
    pub action: __le16,
    pub mode: __le16,
    pub __packed: },
#[no_mangle]
unsafe extern "C" fn mwl8k_cmd_set_rateadapt_mode(hw: *mut ieee80211_hw, mode: __u16) -> c_int {
    static int mwl8k_cmd_set_rateadapt_mode(struct ieee80211_hw *hw, __u16 mode)
    {
    pub cmd: *mut mwl8k_cmd_set_rate_adapt_mode,
    pub rc: c_int,
    pub kzalloc_obj(*cmd): *mut cmd =,
    if (cmd == core::ptr::null_mut())
    pub -ENOMEM: return,
    pub cpu_to_le16(MWL8K_CMD_SET_RATEADAPT_MODE): cmd->header.code =,
    pub cpu_to_le16(sizeof(*cmd)): *mut cmd->header.length =,
    pub cpu_to_le16(MWL8K_CMD_SET): cmd->action =,
    pub cpu_to_le16(mode): cmd->mode =,
    pub &cmd->header): rc = mwl8k_post_cmd(hw,,
    pub rc: return,
    }
//
// CMD_GET_WATCHDOG_BITMAP.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_cmd_get_watchdog_bitmap {
    pub header: mwl8k_cmd_pkt_hdr,
    pub bitmap: u8,
    pub __packed: },
#[no_mangle]
unsafe extern "C" fn mwl8k_cmd_get_watchdog_bitmap(hw: *mut ieee80211_hw, bitmap: *mut u8) -> c_int {
    static int mwl8k_cmd_get_watchdog_bitmap(struct ieee80211_hw *hw, u8 *bitmap)
    {
    pub cmd: *mut mwl8k_cmd_get_watchdog_bitmap,
    pub rc: c_int,
    pub kzalloc_obj(*cmd): *mut cmd =,
    if (cmd == core::ptr::null_mut())
    pub -ENOMEM: return,
    pub cpu_to_le16(MWL8K_CMD_GET_WATCHDOG_BITMAP): cmd->header.code =,
    pub cpu_to_le16(sizeof(*cmd)): *mut cmd->header.length =,
    pub &cmd->header): rc = mwl8k_post_cmd(hw,,
    if (!rc)
// bitmap = cmd->bitmap;
    pub rc: return,
    }
pub const MWL8K_WMM_QUEUE_NUMBER: c_int = 3;
    static void mwl8k_destroy_ba(struct ieee80211_hw *hw,
    pub idx): u8,
#[no_mangle]
unsafe extern "C" fn mwl8k_watchdog_ba_events(work: *mut work_struct) {
    static void mwl8k_watchdog_ba_events(struct work_struct *work)
    {
    pub rc: c_int,
    pub stream_index: u8 bitmap = 0,,
    pub streams: *mut mwl8k_ampdu_stream,
    struct mwl8k_priv *priv =
    pub watchdog_ba_handle): container_of(work, struct mwl8k_priv,,
    pub priv->hw: *mut *mut ieee80211_hw hw =,
    pub i: c_int,
    pub 0: u32 status =,
    pub &bitmap): rc = mwl8k_cmd_get_watchdog_bitmap(priv->hw,,
    if (rc)
    pub done: goto,
// the bitmap is the hw queue number.  Map it to the ampdu queue.
    pub {: for (i = 0; i < TOTAL_HW_TX_QUEUES; i++),
    if (bitmap & (1 << i)) {
    stream_index = (i + MWL8K_WMM_QUEUE_NUMBER) %
    pub &priv->ampdu[stream_index]: streams =,
    if (streams.state == AMPDU_STREAM_ACTIVE) {
    ieee80211_stop_tx_ba_session(streams.sta,
    pub stream_index): mwl8k_destroy_ba(hw,,
    }
    }
    }
    done:
    pub MWL8K_HIU_A2H_INTERRUPT_STATUS_MASK): status = ioread32(priv->regs +,
    iowrite32((status | MWL8K_A2H_INT_BA_WATCHDOG),
    pub MWL8K_HIU_A2H_INTERRUPT_STATUS_MASK): priv->regs +,
    }
//
// CMD_BSS_START.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_cmd_bss_start {
    pub header: mwl8k_cmd_pkt_hdr,
    pub enable: __le32,
    pub __packed: },
    static int mwl8k_cmd_bss_start(struct ieee80211_hw *hw,
    struct ieee80211_vif *vif, int enable)
    {
    pub cmd: *mut mwl8k_cmd_bss_start,
    pub MWL8K_VIF(vif): *mut *mut mwl8k_vif mwl8k_vif =,
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    pub rc: c_int,
    if (enable && (priv.running_bsses & (1 << mwl8k_vif.macid)))
    pub 0: return,
    if (!enable && !(priv.running_bsses & (1 << mwl8k_vif.macid)))
    pub 0: return,
    pub kzalloc_obj(*cmd): *mut cmd =,
    if (cmd == core::ptr::null_mut())
    pub -ENOMEM: return,
    pub cpu_to_le16(MWL8K_CMD_BSS_START): cmd->header.code =,
    pub cpu_to_le16(sizeof(*cmd)): *mut cmd->header.length =,
    pub cpu_to_le32(enable): cmd->enable =,
    pub &cmd->header): rc = mwl8k_post_pervif_cmd(hw, vif,,
    if (!rc) {
    if (enable)
    pub mwl8k_vif->macid): priv->running_bsses |= (1 <<,
    else
    pub mwl8k_vif->macid): priv->running_bsses &= ~(1 <<,
    }
    pub rc: return,
    }
#[no_mangle]
unsafe extern "C" fn mwl8k_enable_bsses(hw: *mut ieee80211_hw, enable: bool, bitmap: u32) {
    static void mwl8k_enable_bsses(struct ieee80211_hw *hw, bool enable, u32 bitmap)
    {
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    pub tmp_vif: *mut *mut mwl8k_vif mwl8k_vif,,
    pub vif: *mut ieee80211_vif,
    list_for_each_entry_safe(mwl8k_vif, tmp_vif, &priv.vif_list, list) {
    pub mwl8k_vif->vif: vif =,
    if (!(bitmap & (1 << mwl8k_vif.macid)))
    if (vif.type == NL80211_IFTYPE_AP)
    pub enable): mwl8k_cmd_bss_start(hw, vif,,
    }
    }
//
// CMD_BASTREAM.
//
// UPSTREAM is tx direction
//
pub const BASTREAM_FLAG_DIRECTION_UPSTREAM: c_uint = 0x00;
pub const BASTREAM_FLAG_IMMEDIATE_TYPE: c_uint = 0x01;
    enum ba_stream_action_type {
    MWL8K_BA_CREATE,
    MWL8K_BA_UPDATE,
    MWL8K_BA_DESTROY,
    MWL8K_BA_FLUSH,
    MWL8K_BA_CHECK,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_create_ba_stream {
    pub flags: __le32,
    pub idle_thrs: __le32,
    pub bar_thrs: __le32,
    pub window_size: __le32,
    pub peer_mac_addr: [u8; 6],
    pub dialog_token: u8,
    pub tid: u8,
    pub queue_id: u8,
    pub param_info: u8,
    pub ba_context: __le32,
    pub reset_seq_no_flag: u8,
    pub curr_seq_no: __le16,
    pub sta_src_mac_addr: [u8; 6],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_destroy_ba_stream {
    pub flags: __le32,
    pub ba_context: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_cmd_bastream {
    pub header: mwl8k_cmd_pkt_hdr,
    pub action: __le32,
    union {
    pub create_params: mwl8k_create_ba_stream,
    pub destroy_params: mwl8k_destroy_ba_stream,
}

    } __packed;
    static int
    mwl8k_check_ba(struct ieee80211_hw *hw, struct mwl8k_ampdu_stream *stream,
    struct ieee80211_vif *vif)
    {
    struct mwl8k_cmd_bastream *cmd;
    int rc;
    cmd = kzalloc_obj(*cmd);
    if (cmd == core::ptr::null_mut())
    return -ENOMEM;
    cmd.header.code = cpu_to_le16(MWL8K_CMD_BASTREAM);
    cmd.header.length = cpu_to_le16(sizeof(*cmd));
    cmd.action = cpu_to_le32(MWL8K_BA_CHECK);
    cmd.create_params.queue_id = stream.idx;
    memcpy(&cmd.create_params.peer_mac_addr[0], stream.sta.addr,
    ETH_ALEN);
    cmd.create_params.tid = stream.tid;
    cmd.create_params.flags =
    cpu_to_le32(BASTREAM_FLAG_IMMEDIATE_TYPE) |
    cpu_to_le32(BASTREAM_FLAG_DIRECTION_UPSTREAM);
    rc = mwl8k_post_pervif_cmd(hw, vif, &cmd.header);
    kfree(cmd);
    return rc;
    }
    static int
    mwl8k_create_ba(struct ieee80211_hw *hw, struct mwl8k_ampdu_stream *stream,
    u8 buf_size, struct ieee80211_vif *vif)
    {
    struct mwl8k_cmd_bastream *cmd;
    int rc;
    cmd = kzalloc_obj(*cmd);
    if (cmd == core::ptr::null_mut())
    return -ENOMEM;
    cmd.header.code = cpu_to_le16(MWL8K_CMD_BASTREAM);
    cmd.header.length = cpu_to_le16(sizeof(*cmd));
    cmd.action = cpu_to_le32(MWL8K_BA_CREATE);
    cmd.create_params.bar_thrs = cpu_to_le32((u32)buf_size);
    cmd.create_params.window_size = cpu_to_le32((u32)buf_size);
    cmd.create_params.queue_id = stream.idx;
    memcpy(cmd.create_params.peer_mac_addr, stream.sta.addr, ETH_ALEN);
    cmd.create_params.tid = stream.tid;
    cmd.create_params.curr_seq_no = cpu_to_le16(0);
    cmd.create_params.reset_seq_no_flag = 1;
    cmd.create_params.param_info =
    (stream.sta.deflink.ht_cap.ampdu_factor &
    IEEE80211_HT_AMPDU_PARM_FACTOR) |
    ((stream.sta.deflink.ht_cap.ampdu_density << 2) &
    IEEE80211_HT_AMPDU_PARM_DENSITY);
    cmd.create_params.flags =
    cpu_to_le32(BASTREAM_FLAG_IMMEDIATE_TYPE |
    BASTREAM_FLAG_DIRECTION_UPSTREAM);
    rc = mwl8k_post_pervif_cmd(hw, vif, &cmd.header);
    wiphy_debug(hw.wiphy, "Created a BA stream for %pM : tid %d\n",
    stream.sta.addr, stream.tid);
    kfree(cmd);
    return rc;
    }
    static void mwl8k_destroy_ba(struct ieee80211_hw *hw,
    u8 idx)
    {
    struct mwl8k_cmd_bastream *cmd;
    cmd = kzalloc_obj(*cmd);
    if (cmd == core::ptr::null_mut())
    return;
    cmd.header.code = cpu_to_le16(MWL8K_CMD_BASTREAM);
    cmd.header.length = cpu_to_le16(sizeof(*cmd));
    cmd.action = cpu_to_le32(MWL8K_BA_DESTROY);
    cmd.destroy_params.ba_context = cpu_to_le32(idx);
    mwl8k_post_cmd(hw, &cmd.header);
    wiphy_debug(hw.wiphy, "Deleted BA stream index %d\n", idx);
    kfree(cmd);
    }
//
// CMD_SET_NEW_STN.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_cmd_set_new_stn {
    pub header: mwl8k_cmd_pkt_hdr,
    pub aid: __le16,
    pub mac_addr: [__u8; 6],
    pub stn_id: __le16,
    pub action: __le16,
    pub rsvd: __le16,
    pub legacy_rates: __le32,
    pub ht_rates: [__u8; 4],
    pub cap_info: __le16,
    pub ht_capabilities_info: __le16,
    pub mac_ht_param_info: __u8,
    pub rev: __u8,
    pub control_channel: __u8,
    pub add_channel: __u8,
    pub op_mode: __le16,
    pub stbc: __le16,
    pub add_qos_info: __u8,
    pub is_qos_sta: __u8,
    pub fw_sta_ptr: __le32,
    pub __packed: },
pub const MWL8K_STA_ACTION_ADD: c_int = 0;
pub const MWL8K_STA_ACTION_REMOVE: c_int = 2;
    static int mwl8k_cmd_set_new_stn_add(struct ieee80211_hw *hw,
    struct ieee80211_vif *vif,
    struct ieee80211_sta *sta)
    {
    pub cmd: *mut mwl8k_cmd_set_new_stn,
    pub rates: u32,
    pub rc: c_int,
    pub kzalloc_obj(*cmd): *mut cmd =,
    if (cmd == core::ptr::null_mut())
    pub -ENOMEM: return,
    pub cpu_to_le16(MWL8K_CMD_SET_NEW_STN): cmd->header.code =,
    pub cpu_to_le16(sizeof(*cmd)): *mut cmd->header.length =,
    pub cpu_to_le16(sta->aid): cmd->aid =,
    pub ETH_ALEN): memcpy(cmd->mac_addr, sta->addr,,
    pub cpu_to_le16(sta->aid): cmd->stn_id =,
    pub cpu_to_le16(MWL8K_STA_ACTION_ADD): cmd->action =,
    if (hw.conf.chandef.chan.band == NL80211_BAND_2GHZ)
    pub sta->deflink.supp_rates[NL80211_BAND_2GHZ]: rates =,
    else
    pub 5: rates = sta->deflink.supp_rates[NL80211_BAND_5GHZ] <<,
    pub cpu_to_le32(rates): cmd->legacy_rates =,
    if (sta.deflink.ht_cap.ht_supported) {
    pub sta->deflink.ht_cap.mcs.rx_mask[0]: cmd->ht_rates[0] =,
    pub sta->deflink.ht_cap.mcs.rx_mask[1]: cmd->ht_rates[1] =,
    pub sta->deflink.ht_cap.mcs.rx_mask[2]: cmd->ht_rates[2] =,
    pub sta->deflink.ht_cap.mcs.rx_mask[3]: cmd->ht_rates[3] =,
    pub cpu_to_le16(sta->deflink.ht_cap.cap): cmd->ht_capabilities_info =,
    cmd.mac_ht_param_info = (sta.deflink.ht_cap.ampdu_factor & 3) |
    pub 2): ((sta->deflink.ht_cap.ampdu_density & 7) <<,
    pub 1: cmd->is_qos_sta =,
    }
    pub &cmd->header): rc = mwl8k_post_pervif_cmd(hw, vif,,
    pub rc: return,
    }
    static int mwl8k_cmd_set_new_stn_add_self(struct ieee80211_hw *hw,
    struct ieee80211_vif *vif)
    {
    pub cmd: *mut mwl8k_cmd_set_new_stn,
    pub rc: c_int,
    pub kzalloc_obj(*cmd): *mut cmd =,
    if (cmd == core::ptr::null_mut())
    pub -ENOMEM: return,
    pub cpu_to_le16(MWL8K_CMD_SET_NEW_STN): cmd->header.code =,
    pub cpu_to_le16(sizeof(*cmd)): *mut cmd->header.length =,
    pub ETH_ALEN): memcpy(cmd->mac_addr, vif->addr,,
    pub &cmd->header): rc = mwl8k_post_pervif_cmd(hw, vif,,
    pub rc: return,
    }
    static int mwl8k_cmd_set_new_stn_del(struct ieee80211_hw *hw,
    struct ieee80211_vif *vif, u8 *addr)
    {
    pub cmd: *mut mwl8k_cmd_set_new_stn,
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    pub i: int rc,,
    pub idx: u8,
// Destroy any active ampdu streams for this sta
    pub {: for (i = 0; i < MWL8K_NUM_AMPDU_STREAMS; i++),
    pub s: *mut mwl8k_ampdu_stream,
    pub &priv->ampdu[i]: s =,
    if (s.state != AMPDU_NO_STREAM) {
    if (memcmp(s.sta.addr, addr, ETH_ALEN) == 0) {
    if (s.state == AMPDU_STREAM_ACTIVE) {
    pub s->idx: idx =,
    pub idx): mwl8k_destroy_ba(hw,,
    } else if (s.state == AMPDU_STREAM_NEW) {
    pub s): mwl8k_remove_stream(hw,,
    }
    }
    }
    }
    pub kzalloc_obj(*cmd): *mut cmd =,
    if (cmd == core::ptr::null_mut())
    pub -ENOMEM: return,
    pub cpu_to_le16(MWL8K_CMD_SET_NEW_STN): cmd->header.code =,
    pub cpu_to_le16(sizeof(*cmd)): *mut cmd->header.length =,
    pub ETH_ALEN): memcpy(cmd->mac_addr, addr,,
    pub cpu_to_le16(MWL8K_STA_ACTION_REMOVE): cmd->action =,
    pub &cmd->header): rc = mwl8k_post_pervif_cmd(hw, vif,,
    pub rc: return,
    }
//
// CMD_UPDATE_ENCRYPTION.
//
pub const MAX_ENCR_KEY_LENGTH: c_int = 16;
pub const MIC_KEY_LENGTH: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_cmd_update_encryption {
    pub header: mwl8k_cmd_pkt_hdr,
    pub action: __le32,
    pub reserved: __le32,
    pub mac_addr: [__u8; 6],
    pub encr_type: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_cmd_set_key {
    pub header: mwl8k_cmd_pkt_hdr,
    pub action: __le32,
    pub reserved: __le32,
    pub length: __le16,
    pub key_type_id: __le16,
    pub key_info: __le32,
    pub key_id: __le32,
    pub key_len: __le16,
    struct {
    pub key_material: [__u8; MAX_ENCR_KEY_LENGTH],
    pub tkip_tx_mic_key: [__u8; MIC_KEY_LENGTH],
    pub tkip_rx_mic_key: [__u8; MIC_KEY_LENGTH],
    pub tkip: },
    pub tkip_rsc_low: __le16,
    pub tkip_rsc_high: __le32,
    pub tkip_tsc_low: __le16,
    pub tkip_tsc_high: __le32,
    pub mac_addr: [__u8; 6],
    pub __packed: },
    enum {
    MWL8K_ENCR_ENABLE,
    MWL8K_ENCR_SET_KEY,
    MWL8K_ENCR_REMOVE_KEY,
    MWL8K_ENCR_SET_GROUP_KEY,
}

pub const MWL8K_UPDATE_ENCRYPTION_TYPE_WEP: c_int = 0;
pub const MWL8K_UPDATE_ENCRYPTION_TYPE_DISABLE: c_int = 1;
pub const MWL8K_UPDATE_ENCRYPTION_TYPE_TKIP: c_int = 4;
pub const MWL8K_UPDATE_ENCRYPTION_TYPE_MIXED: c_int = 7;
pub const MWL8K_UPDATE_ENCRYPTION_TYPE_AES: c_int = 8;
    enum {
    MWL8K_ALG_WEP,
    MWL8K_ALG_TKIP,
    MWL8K_ALG_CCMP,
    };
pub const MWL8K_KEY_FLAG_TXGROUPKEY: c_uint = 0x00000004;
pub const MWL8K_KEY_FLAG_PAIRWISE: c_uint = 0x00000008;
pub const MWL8K_KEY_FLAG_TSC_VALID: c_uint = 0x00000040;
pub const MWL8K_KEY_FLAG_WEP_TXKEY: c_uint = 0x01000000;
pub const MWL8K_KEY_FLAG_MICKEY_VALID: c_uint = 0x02000000;
    static int mwl8k_cmd_update_encryption_enable(struct ieee80211_hw *hw,
    struct ieee80211_vif *vif,
    u8 *addr,
    u8 encr_type)
    {
    struct mwl8k_cmd_update_encryption *cmd;
    int rc;
    cmd = kzalloc_obj(*cmd);
    if (cmd == core::ptr::null_mut())
    return -ENOMEM;
    cmd.header.code = cpu_to_le16(MWL8K_CMD_UPDATE_ENCRYPTION);
    cmd.header.length = cpu_to_le16(sizeof(*cmd));
    cmd.action = cpu_to_le32(MWL8K_ENCR_ENABLE);
    memcpy(cmd.mac_addr, addr, ETH_ALEN);
    cmd.encr_type = encr_type;
    rc = mwl8k_post_pervif_cmd(hw, vif, &cmd.header);
    kfree(cmd);
    return rc;
    }
    static int mwl8k_encryption_set_cmd_info(struct mwl8k_cmd_set_key *cmd,
    u8 *addr,
    struct ieee80211_key_conf *key)
    {
    cmd.header.code = cpu_to_le16(MWL8K_CMD_UPDATE_ENCRYPTION);
    cmd.header.length = cpu_to_le16(sizeof(*cmd));
    cmd.length = cpu_to_le16(sizeof(*cmd) -
    offsetof(struct mwl8k_cmd_set_key, length));
    cmd.key_id = cpu_to_le32(key.keyidx);
    cmd.key_len = cpu_to_le16(key.keylen);
    memcpy(cmd.mac_addr, addr, ETH_ALEN);
    switch (key.cipher) {
    case WLAN_CIPHER_SUITE_WEP40:
    case WLAN_CIPHER_SUITE_WEP104:
    cmd.key_type_id = cpu_to_le16(MWL8K_ALG_WEP);
    if (key.keyidx == 0)
    cmd.key_info =	cpu_to_le32(MWL8K_KEY_FLAG_WEP_TXKEY);
    break;
    case WLAN_CIPHER_SUITE_TKIP:
    cmd.key_type_id = cpu_to_le16(MWL8K_ALG_TKIP);
    cmd.key_info =	(key.flags & IEEE80211_KEY_FLAG_PAIRWISE)
    ? cpu_to_le32(MWL8K_KEY_FLAG_PAIRWISE)
    : cpu_to_le32(MWL8K_KEY_FLAG_TXGROUPKEY);
    cmd.key_info |= cpu_to_le32(MWL8K_KEY_FLAG_MICKEY_VALID
    | MWL8K_KEY_FLAG_TSC_VALID);
    break;
    case WLAN_CIPHER_SUITE_CCMP:
    cmd.key_type_id = cpu_to_le16(MWL8K_ALG_CCMP);
    cmd.key_info =	(key.flags & IEEE80211_KEY_FLAG_PAIRWISE)
    ? cpu_to_le32(MWL8K_KEY_FLAG_PAIRWISE)
    : cpu_to_le32(MWL8K_KEY_FLAG_TXGROUPKEY);
    break;
    default:
    return -ENOTSUPP;
    }
    return 0;
    }
    static int mwl8k_cmd_encryption_set_key(struct ieee80211_hw *hw,
    struct ieee80211_vif *vif,
    u8 *addr,
    struct ieee80211_key_conf *key)
    {
    struct mwl8k_cmd_set_key *cmd;
    int rc;
    int keymlen;
    u32 action;
    u8 idx;
    struct mwl8k_vif *mwl8k_vif = MWL8K_VIF(vif);
    cmd = kzalloc_obj(*cmd);
    if (cmd == core::ptr::null_mut())
    return -ENOMEM;
    rc = mwl8k_encryption_set_cmd_info(cmd, addr, key);
    if (rc < 0)
    goto done;
    idx = key.keyidx;
    if (key.flags & IEEE80211_KEY_FLAG_PAIRWISE)
    action = MWL8K_ENCR_SET_KEY;
    else
    action = MWL8K_ENCR_SET_GROUP_KEY;
    switch (key.cipher) {
    case WLAN_CIPHER_SUITE_WEP40:
    case WLAN_CIPHER_SUITE_WEP104:
    if (!mwl8k_vif.wep_key_conf[idx].enabled) {
    memcpy(mwl8k_vif.wep_key_conf[idx].key, key,
    sizeof(*key) + key.keylen);
    mwl8k_vif.wep_key_conf[idx].enabled = 1;
    }
    keymlen = key.keylen;
    action = MWL8K_ENCR_SET_KEY;
    break;
    case WLAN_CIPHER_SUITE_TKIP:
    keymlen = MAX_ENCR_KEY_LENGTH + 2 * MIC_KEY_LENGTH;
    break;
    case WLAN_CIPHER_SUITE_CCMP:
    keymlen = key.keylen;
    break;
    default:
    rc = -ENOTSUPP;
    goto done;
    }
    memcpy(&cmd.tkip, key.key, keymlen);
    cmd.action = cpu_to_le32(action);
    rc = mwl8k_post_pervif_cmd(hw, vif, &cmd.header);
    done:
    kfree(cmd);
    return rc;
    }
    static int mwl8k_cmd_encryption_remove_key(struct ieee80211_hw *hw,
    struct ieee80211_vif *vif,
    u8 *addr,
    struct ieee80211_key_conf *key)
    {
    struct mwl8k_cmd_set_key *cmd;
    int rc;
    struct mwl8k_vif *mwl8k_vif = MWL8K_VIF(vif);
    cmd = kzalloc_obj(*cmd);
    if (cmd == core::ptr::null_mut())
    return -ENOMEM;
    rc = mwl8k_encryption_set_cmd_info(cmd, addr, key);
    if (rc < 0)
    goto done;
    if (key.cipher == WLAN_CIPHER_SUITE_WEP40 ||
    key.cipher == WLAN_CIPHER_SUITE_WEP104)
    mwl8k_vif.wep_key_conf[key.keyidx].enabled = 0;
    cmd.action = cpu_to_le32(MWL8K_ENCR_REMOVE_KEY);
    rc = mwl8k_post_pervif_cmd(hw, vif, &cmd.header);
    done:
    kfree(cmd);
    return rc;
    }
    static int mwl8k_set_key(struct ieee80211_hw *hw,
    enum set_key_cmd cmd_param,
    struct ieee80211_vif *vif,
    struct ieee80211_sta *sta,
    struct ieee80211_key_conf *key)
    {
    let mut rc: c_int = 0;
    u8 encr_type;
    u8 *addr;
    struct mwl8k_vif *mwl8k_vif = MWL8K_VIF(vif);
    struct mwl8k_priv *priv = hw.priv;
    if (vif.type == NL80211_IFTYPE_STATION && !priv.ap_fw)
    return -EOPNOTSUPP;
    if (sta == core::ptr::null_mut())
    addr = vif.addr;
    else
    addr = sta.addr;
    if (cmd_param == SET_KEY) {
    rc = mwl8k_cmd_encryption_set_key(hw, vif, addr, key);
    if (rc)
    goto out;
    if ((key.cipher == WLAN_CIPHER_SUITE_WEP40)
    || (key.cipher == WLAN_CIPHER_SUITE_WEP104))
    encr_type = MWL8K_UPDATE_ENCRYPTION_TYPE_WEP;
    else
    encr_type = MWL8K_UPDATE_ENCRYPTION_TYPE_MIXED;
    rc = mwl8k_cmd_update_encryption_enable(hw, vif, addr,
    encr_type);
    if (rc)
    goto out;
    mwl8k_vif.is_hw_crypto_enabled = true;
    } else {
    rc = mwl8k_cmd_encryption_remove_key(hw, vif, addr, key);
    if (rc)
    goto out;
    }
    out:
    return rc;
    }
//
// CMD_UPDATE_STADB.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ewc_ht_info {
    pub control1: __le16,
    pub control2: __le16,
    pub control3: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct peer_capability_info {
// Peer type - AP vs. STA.
    pub peer_type: __u8,
// Basic 802.11 capabilities from assoc resp.
    pub basic_caps: __le16,
// Set if peer supports 802.11n high throughput (HT).
    pub ht_support: __u8,
// Valid if HT is supported.
    pub ht_caps: __le16,
    pub extended_ht_caps: __u8,
    pub ewc_info: ewc_ht_info,
// Legacy rate table. Intersection of our rates and peer rates.
    pub legacy_rates: [__u8; 12],
// HT rate table. Intersection of our rates and peer rates.
    pub ht_rates: [__u8; 16],
    pub pad: [__u8; 16],
// If set, interoperability mode, no proprietary extensions.
    pub interop: __u8,
    pub pad2: __u8,
    pub station_id: __u8,
    pub amsdu_enabled: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwl8k_cmd_update_stadb {
    pub header: mwl8k_cmd_pkt_hdr,
// See STADB_ACTION_TYPE
    pub action: __le32,
// Peer MAC address
    pub peer_addr: [__u8; ETH_ALEN],
    pub reserved: __le32,
// Peer info - valid during add/update.
    pub peer_info: peer_capability_info,
    pub __packed: },
pub const MWL8K_STA_DB_MODIFY_ENTRY: c_int = 1;
pub const MWL8K_STA_DB_DEL_ENTRY: c_int = 2;
// Peer Entry flags - used to define the type of the peer node
pub const MWL8K_PEER_TYPE_ACCESSPOINT: c_int = 2;
    static int mwl8k_cmd_update_stadb_add(struct ieee80211_hw *hw,
    struct ieee80211_vif *vif,
    struct ieee80211_sta *sta)
    {
    pub cmd: *mut mwl8k_cmd_update_stadb,
    pub p: *mut peer_capability_info,
    pub rates: u32,
    pub rc: c_int,
    pub kzalloc_obj(*cmd): *mut cmd =,
    if (cmd == core::ptr::null_mut())
    pub -ENOMEM: return,
    pub cpu_to_le16(MWL8K_CMD_UPDATE_STADB): cmd->header.code =,
    pub cpu_to_le16(sizeof(*cmd)): *mut cmd->header.length =,
    pub cpu_to_le32(MWL8K_STA_DB_MODIFY_ENTRY): cmd->action =,
    pub ETH_ALEN): memcpy(cmd->peer_addr, sta->addr,,
    pub &cmd->peer_info: p =,
    pub MWL8K_PEER_TYPE_ACCESSPOINT: p->peer_type =,
    pub cpu_to_le16(vif->bss_conf.assoc_capability): p->basic_caps =,
    pub sta->deflink.ht_cap.ht_supported: p->ht_support =,
    pub cpu_to_le16(sta->deflink.ht_cap.cap): p->ht_caps =,
    p.extended_ht_caps = (sta.deflink.ht_cap.ampdu_factor & 3) |
    pub 2): ((sta->deflink.ht_cap.ampdu_density & 7) <<,
    if (hw.conf.chandef.chan.band == NL80211_BAND_2GHZ)
    pub sta->deflink.supp_rates[NL80211_BAND_2GHZ]: rates =,
    else
    pub 5: rates = sta->deflink.supp_rates[NL80211_BAND_5GHZ] <<,
    pub rates): legacy_rate_mask_to_array(p->legacy_rates,,
    pub 16): memcpy(p->ht_rates, &sta->deflink.ht_cap.mcs,,
    pub 1: p->interop =,
    pub 0: p->amsdu_enabled =,
    pub &cmd->header): rc = mwl8k_post_cmd(hw,,
    if (!rc)
    pub p->station_id: rc =,
    pub rc: return,
    }
    static int mwl8k_cmd_update_stadb_del(struct ieee80211_hw *hw,
    struct ieee80211_vif *vif, u8 *addr)
    {
    pub cmd: *mut mwl8k_cmd_update_stadb,
    pub rc: c_int,
    pub kzalloc_obj(*cmd): *mut cmd =,
    if (cmd == core::ptr::null_mut())
    pub -ENOMEM: return,
    pub cpu_to_le16(MWL8K_CMD_UPDATE_STADB): cmd->header.code =,
    pub cpu_to_le16(sizeof(*cmd)): *mut cmd->header.length =,
    pub cpu_to_le32(MWL8K_STA_DB_DEL_ENTRY): cmd->action =,
    pub ETH_ALEN): memcpy(cmd->peer_addr, addr,,
    pub &cmd->header): rc = mwl8k_post_cmd(hw,,
    pub rc: return,
    }
//
// Interrupt handling.
//
#[no_mangle]
unsafe extern "C" fn mwl8k_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t mwl8k_interrupt(int irq, void *dev_id)
    {
    pub dev_id: *mut *mut ieee80211_hw hw =,
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    pub status: u32,
    pub MWL8K_HIU_A2H_INTERRUPT_STATUS): status = ioread32(priv->regs +,
    if (!status)
    pub IRQ_NONE: return,
    if (status & MWL8K_A2H_INT_TX_DONE) {
    pub ~MWL8K_A2H_INT_TX_DONE: status &=,
    }
    if (status & MWL8K_A2H_INT_RX_READY) {
    pub ~MWL8K_A2H_INT_RX_READY: status &=,
    }
    if (status & MWL8K_A2H_INT_BA_WATCHDOG) {
    iowrite32(~MWL8K_A2H_INT_BA_WATCHDOG,
    pub MWL8K_HIU_A2H_INTERRUPT_STATUS_MASK): priv->regs +,
    pub ~MWL8K_A2H_INT_BA_WATCHDOG: status &=,
    pub &priv->watchdog_ba_handle): ieee80211_queue_work(hw,,
    }
    if (status)
    pub MWL8K_HIU_A2H_INTERRUPT_STATUS): iowrite32(~status, priv->regs +,
    if (status & MWL8K_A2H_INT_OPC_DONE) {
    if (priv.hostcmd_wait != core::ptr::null_mut())
    }
    if (status & MWL8K_A2H_INT_QUEUE_EMPTY) {
    if (!mutex_is_locked(&priv.fw_mutex) &&
    priv.radio_on && priv.pending_tx_pkts)
    }
    pub IRQ_HANDLED: return,
    }
#[no_mangle]
unsafe extern "C" fn mwl8k_tx_poll(t: *mut tasklet_struct) {
    static void mwl8k_tx_poll(struct tasklet_struct *t)
    {
    pub poll_tx_task): *mut *mut mwl8k_priv priv = from_tasklet(priv, t,,
    pub pci_get_drvdata(priv->pdev): *mut *mut ieee80211_hw hw =,
    pub limit: c_int,
    pub i: c_int,
    pub 32: limit =,
    pub i++): for (i = 0; i < mwl8k_tx_queues(priv);,
    pub 0): limit -= mwl8k_txq_reclaim(hw, i, limit,,
    if (!priv.pending_tx_pkts && priv.tx_wait != core::ptr::null_mut()) {
    pub NULL: priv->tx_wait =,
    }
    if (limit) {
    writel(~MWL8K_A2H_INT_TX_DONE,
    pub MWL8K_HIU_A2H_INTERRUPT_STATUS): priv->regs +,
    } else {
    }
    }
#[no_mangle]
unsafe extern "C" fn mwl8k_rx_poll(t: *mut tasklet_struct) {
    static void mwl8k_rx_poll(struct tasklet_struct *t)
    {
    pub poll_rx_task): *mut *mut mwl8k_priv priv = from_tasklet(priv, t,,
    pub pci_get_drvdata(priv->pdev): *mut *mut ieee80211_hw hw =,
    pub limit: c_int,
    pub 32: limit =,
    pub limit): limit -= rxq_process(hw, 0,,
    pub limit): limit -= rxq_refill(hw, 0,,
    if (limit) {
    writel(~MWL8K_A2H_INT_RX_READY,
    pub MWL8K_HIU_A2H_INTERRUPT_STATUS): priv->regs +,
    } else {
    }
    }
//
// Core driver operations.
//
    static void mwl8k_tx(struct ieee80211_hw *hw,
    struct ieee80211_tx_control *control,
    struct sk_buff *skb)
    {
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    pub skb_get_queue_mapping(skb): int index =,
    if (!priv.radio_on) {
    wiphy_debug(hw.wiphy,
    pub disabled\n"): "dropped TX frame since radio,
    }
    pub skb): mwl8k_txq_xmit(hw, index, control->sta,,
    }
#[no_mangle]
unsafe extern "C" fn mwl8k_start(hw: *mut ieee80211_hw) -> c_int {
    static int mwl8k_start(struct ieee80211_hw *hw)
    {
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    pub rc: c_int,
    rc = request_irq(priv.pdev.irq, mwl8k_interrupt,
    pub hw): IRQF_SHARED, MWL8K_NAME,,
    if (rc) {
    pub -1: priv->irq =,
    pub handler\n"): wiphy_err(hw->wiphy, "failed to register IRQ,
    pub -EIO: return,
    }
    pub priv->pdev->irq: priv->irq =,
// Enable TX reclaim and RX tasklets.
// Enable interrupts
    pub MWL8K_HIU_A2H_INTERRUPT_MASK): iowrite32(MWL8K_A2H_EVENTS, priv->regs +,
    iowrite32(MWL8K_A2H_EVENTS,
    pub MWL8K_HIU_A2H_INTERRUPT_STATUS_MASK): priv->regs +,
    pub mwl8k_fw_lock(hw): rc =,
    if (!rc) {
    pub mwl8k_cmd_radio_enable(hw): rc =,
    if (!priv.ap_fw) {
    if (!rc)
    pub 0): rc = mwl8k_cmd_enable_sniffer(hw,,
    if (!rc)
    pub mwl8k_cmd_set_pre_scan(hw): rc =,
    if (!rc)
    rc = mwl8k_cmd_set_post_scan(hw,
    }
    if (!rc)
    pub 0): rc = mwl8k_cmd_set_rateadapt_mode(hw,,
    if (!rc)
    pub 0): rc = mwl8k_cmd_set_wmm_mode(hw,,
    }
    if (rc) {
    pub MWL8K_HIU_A2H_INTERRUPT_MASK): iowrite32(0, priv->regs +,
    pub hw): free_irq(priv->pdev->irq,,
    pub -1: priv->irq =,
    } else {
    }
    pub rc: return,
    }
#[no_mangle]
unsafe extern "C" fn mwl8k_stop(hw: *mut ieee80211_hw, suspend: bool) {
    static void mwl8k_stop(struct ieee80211_hw *hw, bool suspend)
    {
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    pub i: c_int,
    if (!priv.hw_restart_in_progress)
// Disable interrupts
    pub MWL8K_HIU_A2H_INTERRUPT_MASK): iowrite32(0, priv->regs +,
    if (priv.irq != -1) {
    pub hw): free_irq(priv->pdev->irq,,
    pub -1: priv->irq =,
    }
// Stop finalize join worker
    if (priv.beacon_skb != core::ptr::null_mut())
// Stop TX reclaim and RX tasklets.
// Return all skbs to mac80211
    pub i++): for (i = 0; i < mwl8k_tx_queues(priv);,
    pub 1): mwl8k_txq_reclaim(hw, i, INT_MAX,,
    }
    pub fw_image): *mut *mut static int mwl8k_reload_firmware(struct ieee80211_hw hw, char,
    static int mwl8k_add_interface(struct ieee80211_hw *hw,
    struct ieee80211_vif *vif)
    {
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    pub mwl8k_vif: *mut mwl8k_vif,
    pub macids_supported: u32,
    pub rc: int macid,,
    pub di: *mut mwl8k_device_info,
//
// Reject interface creation if sniffer mode is active, as
// STA operation is mutually exclusive with hardware sniffer
// mode.  (Sniffer mode is only used on STA firmware.)
//
    if (priv.sniffer_enabled) {
    wiphy_info(hw.wiphy,
    pub enabled\n"): "unable to create STA interface because sniffer mode is,
    pub -EINVAL: return,
    }
    pub priv->device_info: di =,
    switch (vif.type) {
    case NL80211_IFTYPE_AP:
    if (!priv.ap_fw && di.fw_image_ap) {
// we must load the ap fw to meet this request
    if (!list_empty(&priv.vif_list))
    pub -EBUSY: return,
    pub di->fw_image_ap): rc = mwl8k_reload_firmware(hw,,
    if (rc)
    pub rc: return,
    }
    pub priv->ap_macids_supported: macids_supported =,
    case NL80211_IFTYPE_STATION:
    if (priv.ap_fw && di.fw_image_sta) {
    if (!list_empty(&priv.vif_list)) {
    wiphy_warn(hw.wiphy, "AP interface is running.\n"
    pub WDS"): "Adding STA interface for,
    } else {
// we must load the sta fw to
// meet this request.
//
    rc = mwl8k_reload_firmware(hw,
    if (rc)
    pub rc: return,
    }
    }
    pub priv->sta_macids_supported: macids_supported =,
    default:
    pub -EINVAL: return,
    }
    pub ~priv->macids_used): macid = ffs(macids_supported &,
    if (!macid--)
    pub -EBUSY: return,
// Setup driver private area.
    pub MWL8K_VIF(vif): mwl8k_vif =,
    pub sizeof(*mwl8k_vif)): *mut memset(mwl8k_vif, 0,,
    pub vif: mwl8k_vif->vif =,
    pub macid: mwl8k_vif->macid =,
    pub 0: mwl8k_vif->seqno =,
    pub ETH_ALEN): memcpy(mwl8k_vif->bssid, vif->addr,,
    pub false: mwl8k_vif->is_hw_crypto_enabled =,
// Set the mac address.
    pub vif->addr): mwl8k_cmd_set_mac_addr(hw, vif,,
    if (vif.type == NL80211_IFTYPE_AP)
    pub vif): mwl8k_cmd_set_new_stn_add_self(hw,,
    pub mwl8k_vif->macid: priv->macids_used |= 1 <<,
    pub &priv->vif_list): list_add_tail(&mwl8k_vif->list,,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn mwl8k_remove_vif(priv: *mut mwl8k_priv, vif: *mut mwl8k_vif) {
    static void mwl8k_remove_vif(struct mwl8k_priv *priv, struct mwl8k_vif *vif)
    {
// Has ieee80211_restart_hw re-added the removed interfaces?
    if (!priv.macids_used)
    pub vif->macid): priv->macids_used &= ~(1 <<,
    }
    static void mwl8k_remove_interface(struct ieee80211_hw *hw,
    struct ieee80211_vif *vif)
    {
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    pub MWL8K_VIF(vif): *mut *mut mwl8k_vif mwl8k_vif =,
    if (vif.type == NL80211_IFTYPE_AP)
    pub vif->addr): mwl8k_cmd_set_new_stn_del(hw, vif,,
    pub vif->addr): mwl8k_cmd_del_mac_addr(hw, vif,,
    pub mwl8k_vif): mwl8k_remove_vif(priv,,
    }
#[no_mangle]
unsafe extern "C" fn mwl8k_hw_restart_work(work: *mut work_struct) {
    static void mwl8k_hw_restart_work(struct work_struct *work)
    {
    struct mwl8k_priv *priv =
    pub fw_reload): container_of(work, struct mwl8k_priv,,
    pub priv->hw: *mut *mut ieee80211_hw hw =,
    pub di: *mut mwl8k_device_info,
    pub rc: c_int,
// If some command is waiting for a response, clear it
    if (priv.hostcmd_wait != core::ptr::null_mut()) {
    pub NULL: priv->hostcmd_wait =,
    }
    pub current: priv->hw_restart_owner =,
    pub priv->device_info: di =,
    if (priv.ap_fw)
    pub di->fw_image_ap): rc = mwl8k_reload_firmware(hw,,
    else
    pub di->fw_image_sta): rc = mwl8k_reload_firmware(hw,,
    if (rc)
    pub fail: goto,
    pub NULL: priv->hw_restart_owner =,
    pub false: priv->hw_restart_in_progress =,
//
// This unlock will wake up the queues and
// also opens the command path for other
// commands
//
    pub successfully\n"): wiphy_err(hw->wiphy, "Firmware restarted,
    fail:
    pub failed\n"): wiphy_err(hw->wiphy, "Firmware restart,
    }
#[no_mangle]
unsafe extern "C" fn mwl8k_config(hw: *mut ieee80211_hw, radio_idx: c_int, changed: u32) -> c_int {
    static int mwl8k_config(struct ieee80211_hw *hw, int radio_idx, u32 changed)
    {
    pub &hw->conf: *mut *mut ieee80211_conf conf =,
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    pub rc: c_int,
    pub mwl8k_fw_lock(hw): rc =,
    if (rc)
    pub rc: return,
    if (conf.flags & IEEE80211_CONF_IDLE)
    pub mwl8k_cmd_radio_disable(hw): rc =,
    else
    pub mwl8k_cmd_radio_enable(hw): rc =,
    if (rc)
    pub out: goto,
    if (changed & IEEE80211_CONF_CHANGE_CHANNEL) {
    pub conf): rc = mwl8k_cmd_set_rf_channel(hw,,
    if (rc)
    pub out: goto,
    }
    if (conf.power_level > 18)
    pub 18: conf->power_level =,
    if (priv.ap_fw) {
    if (conf.flags & IEEE80211_CONF_CHANGE_POWER) {
    pub conf->power_level): rc = mwl8k_cmd_tx_power(hw, conf,,
    if (rc)
    pub out: goto,
    }
    } else {
    pub conf->power_level): rc = mwl8k_cmd_rf_tx_power(hw,,
    if (rc)
    pub out: goto,
    pub 0x7): rc = mwl8k_cmd_mimo_config(hw, 0x7,,
    }
    out:
    pub rc: return,
    }
    static void
    mwl8k_bss_info_changed_sta(struct ieee80211_hw *hw, struct ieee80211_vif *vif,
    struct ieee80211_bss_conf *info, u32 changed)
    {
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    pub 0: u32 ap_legacy_rates =,
    pub ap_mcs_rates: [u8; 16],
    pub rc: c_int,
    if (mwl8k_fw_lock(hw))
//
// No need to capture a beacon if we're no longer associated.
//
    if ((changed & BSS_CHANGED_ASSOC) && !vif.cfg.assoc)
    pub false: priv->capture_beacon =,
//
// Get the AP's legacy and MCS rates.
//
    if (vif.cfg.assoc) {
    pub ap: *mut ieee80211_sta,
    pub vif->bss_conf.bssid): ap = ieee80211_find_sta(vif,,
    if (ap == core::ptr::null_mut()) {
    pub out: goto,
    }
    if (hw.conf.chandef.chan.band == NL80211_BAND_2GHZ) {
    pub ap->deflink.supp_rates[NL80211_BAND_2GHZ]: ap_legacy_rates =,
    } else {
    ap_legacy_rates =
    pub 5: ap->deflink.supp_rates[NL80211_BAND_5GHZ] <<,
    }
    pub 16): memcpy(ap_mcs_rates, &ap->deflink.ht_cap.mcs,,
    if (changed & BSS_CHANGED_ASSOC) {
    if (!priv.ap_fw) {
    rc = mwl8k_cmd_set_rate(hw, vif,
    ap_legacy_rates,
    if (rc)
    pub out: goto,
    pub mwl8k_cmd_use_fixed_rate_sta(hw): rc =,
    if (rc)
    pub out: goto,
    } else {
    pub idx: c_int,
    pub rate: c_int,
// Use AP firmware specific rate command.
//
    pub ffs(vif->bss_conf.basic_rates): idx =,
    if (idx)
    if (hw.conf.chandef.chan.band ==
    NL80211_BAND_2GHZ)
    pub mwl8k_rates_24[idx].hw_value: rate =,
    else
    pub mwl8k_rates_50[idx].hw_value: rate =,
    pub rate): mwl8k_cmd_use_fixed_rate_ap(hw, rate,,
    }
    }
    }
    if (changed & BSS_CHANGED_ERP_PREAMBLE) {
    rc = mwl8k_set_radio_preamble(hw,
    if (rc)
    pub out: goto,
    }
    if ((changed & BSS_CHANGED_ERP_SLOT) && !priv.ap_fw)  {
    pub vif->bss_conf.use_short_slot): rc = mwl8k_cmd_set_slot(hw,,
    if (rc)
    pub out: goto,
    }
    if (vif.cfg.assoc && !priv.ap_fw &&
    (changed & (BSS_CHANGED_ASSOC | BSS_CHANGED_ERP_CTS_PROT |
    BSS_CHANGED_HT))) {
    pub ap_legacy_rates): rc = mwl8k_cmd_set_aid(hw, vif,,
    if (rc)
    pub out: goto,
    }
    if (vif.cfg.assoc &&
    (changed & (BSS_CHANGED_ASSOC | BSS_CHANGED_BEACON_INT))) {
//
// Finalize the join.  Tell rx handler to process
// next beacon from our BSSID.
//
    pub ETH_ALEN): memcpy(priv->capture_bssid, vif->bss_conf.bssid,,
    pub true: priv->capture_beacon =,
    }
    out:
    }
    static void
    mwl8k_bss_info_changed_ap(struct ieee80211_hw *hw, struct ieee80211_vif *vif,
    struct ieee80211_bss_conf *info, u32 changed)
    {
    pub rc: c_int,
    if (mwl8k_fw_lock(hw))
    if (changed & BSS_CHANGED_ERP_PREAMBLE) {
    rc = mwl8k_set_radio_preamble(hw,
    if (rc)
    pub out: goto,
    }
    if (changed & BSS_CHANGED_BASIC_RATES) {
    pub idx: c_int,
    pub rate: c_int,
//
// Use lowest supported basic rate for multicasts
// and management frames (such as probe responses --
// beacons will always go out at 1 Mb/s).
//
    pub ffs(vif->bss_conf.basic_rates): idx =,
    if (idx)
    if (hw.conf.chandef.chan.band == NL80211_BAND_2GHZ)
    pub mwl8k_rates_24[idx].hw_value: rate =,
    else
    pub mwl8k_rates_50[idx].hw_value: rate =,
    pub rate): mwl8k_cmd_use_fixed_rate_ap(hw, rate,,
    }
    if (changed & (BSS_CHANGED_BEACON_INT | BSS_CHANGED_BEACON)) {
    pub skb: *mut sk_buff,
    pub 0): skb = ieee80211_beacon_get(hw, vif,,
    if (skb != core::ptr::null_mut()) {
    pub skb->len): mwl8k_cmd_set_beacon(hw, vif, skb->data,,
    }
    }
    if (changed & BSS_CHANGED_BEACON_ENABLED)
    pub info->enable_beacon): mwl8k_cmd_bss_start(hw, vif,,
    out:
    }
    static void
    mwl8k_bss_info_changed(struct ieee80211_hw *hw, struct ieee80211_vif *vif,
    struct ieee80211_bss_conf *info, u64 changed)
    {
    if (vif.type == NL80211_IFTYPE_STATION)
    pub changed): mwl8k_bss_info_changed_sta(hw, vif, info,,
    if (vif.type == NL80211_IFTYPE_AP)
    pub changed): mwl8k_bss_info_changed_ap(hw, vif, info,,
    }
    static u64 mwl8k_prepare_multicast(struct ieee80211_hw *hw,
    struct netdev_hw_addr_list *mc_list)
    {
    pub cmd: *mut mwl8k_cmd_pkt_hdr,
//
// Synthesize and return a command packet that programs the
// hardware multicast address filter.  At this point we don't
// know whether FIF_ALLMULTI is being requested, but if it is,
// we'll end up throwing this packet away and creating a new
// one in mwl8k_configure_filter().
//
    pub mc_list): cmd = __mwl8k_cmd_mac_multicast_adr(hw, 0,,
    pub long)cmd: return (unsigned,
    }
    static int
    mwl8k_configure_filter_sniffer(struct ieee80211_hw *hw,
    unsigned int changed_flags,
    unsigned int *total_flags)
    {
    pub hw->priv: *mut *mut mwl8k_priv priv =,
//
// Hardware sniffer mode is mutually exclusive with STA
// operation, so refuse to enable sniffer mode if a STA
// interface is active.
//
    if (!list_empty(&priv.vif_list)) {
    if (net_ratelimit())
    wiphy_info(hw.wiphy,
    pub active\n"): "not enabling sniffer mode because STA interface is,
    pub 0: return,
    }
    if (!priv.sniffer_enabled) {
    if (mwl8k_cmd_enable_sniffer(hw, 1))
    pub 0: return,
    pub true: priv->sniffer_enabled =,
    }
// total_flags &=	FIF_ALLMULTI |
    FIF_BCN_PRBRESP_PROMISC | FIF_CONTROL |
    pub 1: return,
    }
    static struct mwl8k_vif *mwl8k_first_vif(struct mwl8k_priv *priv)
    {
    if (!list_empty(&priv.vif_list))
    pub list): return list_entry(priv->vif_list.next, struct mwl8k_vif,,
    pub NULL: return,
    }
    static void mwl8k_configure_filter(struct ieee80211_hw *hw,
    unsigned int changed_flags,
    unsigned int *total_flags,
    u64 multicast)
    {
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    pub long)multicast: *mut *mut *mut mwl8k_cmd_pkt_hdr cmd = (void )(unsigned,
//
// AP firmware doesn't allow fine-grained control over
// the receive filter.
//
    if (priv.ap_fw) {
// total_flags &= FIF_ALLMULTI | FIF_BCN_PRBRESP_PROMISC;
    }
//
// Enable hardware sniffer mode if FIF_CONTROL or
// FIF_OTHER_BSS is requested.
//
    if (*total_flags & (FIF_CONTROL | FIF_OTHER_BSS) &&
    mwl8k_configure_filter_sniffer(hw, changed_flags, total_flags)) {
    }
// Clear unsupported feature flags
// total_flags &= FIF_ALLMULTI | FIF_BCN_PRBRESP_PROMISC;
    if (mwl8k_fw_lock(hw)) {
    }
    if (priv.sniffer_enabled) {
    pub 0): mwl8k_cmd_enable_sniffer(hw,,
    pub false: priv->sniffer_enabled =,
    }
    if (changed_flags & FIF_BCN_PRBRESP_PROMISC) {
    if (*total_flags & FIF_BCN_PRBRESP_PROMISC) {
//
// Disable the BSS filter.
//
    } else {
    pub mwl8k_vif: *mut mwl8k_vif,
    pub bssid: *const u8,
//
// Enable the BSS filter.
//
// If there is an active STA interface, use that
// interface's BSSID, otherwise use a dummy one
// (where the OUI part needs to be nonzero for
// the BSSID to be accepted by POST_SCAN).
//
    pub mwl8k_first_vif(priv): mwl8k_vif =,
    if (mwl8k_vif != core::ptr::null_mut())
    pub mwl8k_vif->vif->bss_conf.bssid: bssid =,
    else
    pub "\x01\x00\x00\x00\x00\x00": bssid =,
    pub bssid): mwl8k_cmd_set_post_scan(hw,,
    }
    }
//
// If FIF_ALLMULTI is being requested, throw away the command
// packet that ->prepare_multicast() built and replace it with
// a command packet that enables reception of all multicast
// packets.
//
    if (*total_flags & FIF_ALLMULTI) {
    pub NULL): cmd = __mwl8k_cmd_mac_multicast_adr(hw, 1,,
    }
    if (cmd != core::ptr::null_mut()) {
    pub cmd): mwl8k_post_cmd(hw,,
    }
    }
    static int mwl8k_set_rts_threshold(struct ieee80211_hw *hw, int radio_idx,
    u32 value)
    {
    pub value): return mwl8k_cmd_set_rts_threshold(hw, radio_idx,,
    }
    static int mwl8k_sta_remove(struct ieee80211_hw *hw,
    struct ieee80211_vif *vif,
    struct ieee80211_sta *sta)
    {
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    if (priv.ap_fw)
    pub sta->addr): return mwl8k_cmd_set_new_stn_del(hw, vif,,
    else
    pub sta->addr): return mwl8k_cmd_update_stadb_del(hw, vif,,
    }
    static int mwl8k_sta_add(struct ieee80211_hw *hw,
    struct ieee80211_vif *vif,
    struct ieee80211_sta *sta)
    {
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    pub ret: c_int,
    pub i: c_int,
    pub MWL8K_VIF(vif): *mut *mut mwl8k_vif mwl8k_vif =,
    pub key: *mut ieee80211_key_conf,
    if (!priv.ap_fw) {
    pub sta): ret = mwl8k_cmd_update_stadb_add(hw, vif,,
    if (ret >= 0) {
    pub ret: MWL8K_STA(sta)->peer_id =,
    if (sta.deflink.ht_cap.ht_supported)
    pub true: MWL8K_STA(sta)->is_ampdu_allowed =,
    pub 0: ret =,
    }
    } else {
    pub sta): ret = mwl8k_cmd_set_new_stn_add(hw, vif,,
    }
    pub {: for (i = 0; i < NUM_WEP_KEYS; i++),
    pub IEEE80211_KEY_CONF(mwl8k_vif->wep_key_conf[i].key): key =,
    if (mwl8k_vif.wep_key_conf[i].enabled)
    pub key): mwl8k_set_key(hw, SET_KEY, vif, sta,,
    }
    pub ret: return,
    }
    static int mwl8k_conf_tx(struct ieee80211_hw *hw,
    struct ieee80211_vif *vif,
    unsigned int link_id, u16 queue,
    const struct ieee80211_tx_queue_params *params)
    {
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    pub rc: c_int,
    pub mwl8k_fw_lock(hw): rc =,
    if (!rc) {
    pub 1): BUG_ON(queue > MWL8K_TX_WMM_QUEUES -,
    pub sizeof(*params)): *mut memcpy(&priv->wmm_params[queue], params,,
    if (!priv.wmm_enabled)
    pub 1): rc = mwl8k_cmd_set_wmm_mode(hw,,
    if (!rc) {
    pub queue: int q = MWL8K_TX_WMM_QUEUES - 1 -,
    rc = mwl8k_cmd_set_edca_params(hw, q,
    params.cw_min,
    params.cw_max,
    params.aifs,
    }
    }
    pub rc: return,
    }
    static int mwl8k_get_stats(struct ieee80211_hw *hw,
    struct ieee80211_low_level_stats *stats)
    {
    pub stats): return mwl8k_cmd_get_stat(hw,,
    }
    static int mwl8k_get_survey(struct ieee80211_hw *hw, int idx,
    struct survey_info *survey)
    {
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    pub &hw->conf: *mut *mut ieee80211_conf conf =,
    pub sband: *mut ieee80211_supported_band,
    if (priv.ap_fw) {
    pub hw->wiphy->bands[NL80211_BAND_2GHZ]: sband =,
    if (sband && idx >= sband.n_channels) {
    pub sband->n_channels: idx -=,
    pub NULL: sband =,
    }
    if (!sband)
    pub hw->wiphy->bands[NL80211_BAND_5GHZ]: sband =,
    if (!sband || idx >= sband.n_channels)
    pub -ENOENT: return,
    pub sizeof(*survey)): *mut memcpy(survey, &priv->survey[idx],,
    pub &sband->channels[idx]: survey->channel =,
    pub 0: return,
    }
    if (idx != 0)
    pub -ENOENT: return,
    pub conf->chandef.chan: survey->channel =,
    pub SURVEY_INFO_NOISE_DBM: survey->filled =,
    pub priv->noise: survey->noise =,
    pub 0: return,
    }
pub const MAX_AMPDU_ATTEMPTS: c_int = 5;
    static int
    mwl8k_ampdu_action(struct ieee80211_hw *hw, struct ieee80211_vif *vif,
    struct ieee80211_ampdu_params *params)
    {
    pub params->sta: *mut *mut ieee80211_sta sta =,
    pub params->action: enum ieee80211_ampdu_mlme_action action =,
    pub params->tid: u16 tid =,
    pub &params->ssn: *mut *mut u16 ssn =,
    pub params->buf_size: u8 buf_size =,
    pub 0: int i, rc =,
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    pub stream: *mut mwl8k_ampdu_stream,
    pub idx: *mut *mut u8 addr = sta->addr,,
    pub MWL8K_STA(sta): *mut *mut mwl8k_sta sta_info =,
    if (!ieee80211_hw_check(hw, AMPDU_AGGREGATION))
    pub -ENOTSUPP: return,
    pub tid): stream = mwl8k_lookup_stream(hw, addr,,
    switch (action) {
    case IEEE80211_AMPDU_RX_START:
    case IEEE80211_AMPDU_RX_STOP:
    case IEEE80211_AMPDU_TX_START:
// By the time we get here the hw queues may contain outgoing
// packets for this RA/TID that are not part of this BA
// session.  The hw will assign sequence numbers to these
// packets as they go out.  So if we query the hw for its next
// sequence number and use that for the SSN here, it may end up
// being wrong, which will lead to sequence number mismatch at
// the recipient.  To avoid this, we reset the sequence number
// to O for the first MPDU in this BA stream.
//
// ssn = 0;
    if (stream == core::ptr::null_mut()) {
// This means that somebody outside this driver called
// ieee80211_start_tx_ba_session.  This is unexpected
// because we do our own rate control.  Just warn and
// move on.
//
    wiphy_warn(hw.wiphy, "Unexpected call to %s.  "
    pub __func__): "Proceeding anyway.\n",,
    pub tid): stream = mwl8k_add_stream(hw, sta,,
    }
    if (stream == core::ptr::null_mut()) {
    pub streams\n"): wiphy_debug(hw->wiphy, "no free AMPDU,
    pub -EBUSY: rc =,
    }
    pub AMPDU_STREAM_IN_PROGRESS: stream->state =,
// Release the lock before we do the time consuming stuff
    pub {: for (i = 0; i < MAX_AMPDU_ATTEMPTS; i++),
// Check if link is still valid
    if (!sta_info.is_ampdu_allowed) {
    pub stream): mwl8k_remove_stream(hw,,
    pub -EBUSY: return,
    }
    pub vif): rc = mwl8k_check_ba(hw, stream,,
// If HW restart is in progress mwl8k_post_cmd will
// return -EBUSY. Avoid retrying mwl8k_check_ba in
// such cases
//
    if (!rc || rc == -EBUSY)
//
// HW queues take time to be flushed, give them
// sufficient time
//
    }
    if (rc) {
    wiphy_err(hw.wiphy, "Stream for tid %d busy after %d"
    pub MAX_AMPDU_ATTEMPTS): " attempts\n", tid,,
    pub stream): mwl8k_remove_stream(hw,,
    pub -EBUSY: rc =,
    }
    pub IEEE80211_AMPDU_TX_START_IMMEDIATE: rc =,
    case IEEE80211_AMPDU_TX_STOP_CONT:
    case IEEE80211_AMPDU_TX_STOP_FLUSH:
    case IEEE80211_AMPDU_TX_STOP_FLUSH_CONT:
    if (stream) {
    if (stream.state == AMPDU_STREAM_ACTIVE) {
    pub stream->idx: idx =,
    pub idx): mwl8k_destroy_ba(hw,,
    }
    pub stream): mwl8k_remove_stream(hw,,
    }
    pub tid): ieee80211_stop_tx_ba_cb_irqsafe(vif, addr,,
    case IEEE80211_AMPDU_TX_OPERATIONAL:
    pub NULL): BUG_ON(stream ==,
    pub AMPDU_STREAM_IN_PROGRESS): BUG_ON(stream->state !=,
    pub vif): rc = mwl8k_create_ba(hw, stream, buf_size,,
    if (!rc)
    pub AMPDU_STREAM_ACTIVE: stream->state =,
    else {
    pub stream->idx: idx =,
    pub idx): mwl8k_destroy_ba(hw,,
    wiphy_debug(hw.wiphy,
    "Failed adding stream for sta %pM tid %d\n",
    pub tid): addr,,
    pub stream): mwl8k_remove_stream(hw,,
    }
    default:
    pub -ENOTSUPP: rc =,
    }
    pub rc: return,
    }
    static void mwl8k_sw_scan_start(struct ieee80211_hw *hw,
    struct ieee80211_vif *vif,
    const u8 *mac_addr)
    {
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    pub tmp: u8,
    if (!priv.ap_fw)
// clear all stats
    pub 0: priv->channel_time =,
    pub BBU_RXRDY_CNT_REG): ioread32(priv->regs +,
    pub NOK_CCA_CNT_REG): ioread32(priv->regs +,
    pub &tmp): mwl8k_cmd_bbp_reg_access(priv->hw, 0, BBU_AVG_NOISE_VAL,,
    pub true: priv->sw_scan_start =,
    }
    static void mwl8k_sw_scan_complete(struct ieee80211_hw *hw,
    struct ieee80211_vif *vif)
    {
    pub hw->priv: *mut *mut mwl8k_priv priv =,
    pub tmp: u8,
    if (!priv.ap_fw)
    pub false: priv->sw_scan_start =,
// clear all stats
    pub 0: priv->channel_time =,
    pub BBU_RXRDY_CNT_REG): ioread32(priv->regs +,
    pub NOK_CCA_CNT_REG): ioread32(priv->regs +,
    pub &tmp): mwl8k_cmd_bbp_reg_access(priv->hw, 0, BBU_AVG_NOISE_VAL,,
    }
    static const struct ieee80211_ops mwl8k_ops = {
    .add_chanctx = ieee80211_emulate_add_chanctx,
    .remove_chanctx = ieee80211_emulate_remove_chanctx,
    .change_chanctx = ieee80211_emulate_change_chanctx,
    .switch_vif_chanctx = ieee80211_emulate_switch_vif_chanctx,
    .tx			= mwl8k_tx,
    .wake_tx_queue		= ieee80211_handle_wake_tx_queue,
    .start			= mwl8k_start,
    .stop			= mwl8k_stop,
    .add_interface		= mwl8k_add_interface,
    .remove_interface	= mwl8k_remove_interface,
    .config			= mwl8k_config,
    .bss_info_changed	= mwl8k_bss_info_changed,
    .prepare_multicast	= mwl8k_prepare_multicast,
    .configure_filter	= mwl8k_configure_filter,
    .set_key                = mwl8k_set_key,
    .set_rts_threshold	= mwl8k_set_rts_threshold,
    .sta_add		= mwl8k_sta_add,
    .sta_remove		= mwl8k_sta_remove,
    .conf_tx		= mwl8k_conf_tx,
    .get_stats		= mwl8k_get_stats,
    .get_survey		= mwl8k_get_survey,
    .ampdu_action		= mwl8k_ampdu_action,
    .sw_scan_start		= mwl8k_sw_scan_start,
    .sw_scan_complete	= mwl8k_sw_scan_complete,
}

#[no_mangle]
unsafe extern "C" fn mwl8k_finalize_join_worker(work: *mut work_struct) {
    static void mwl8k_finalize_join_worker(struct work_struct *work)
    {
    struct mwl8k_priv *priv =
    container_of(work, struct mwl8k_priv, finalize_join_worker);
    struct sk_buff *skb = priv.beacon_skb;
    struct ieee80211_mgmt *mgmt = (void *)skb.data;
    let mut len: c_int = skb.len - offsetof(struct ieee80211_mgmt, u.beacon.variable);
    const u8 *tim = cfg80211_find_ie(WLAN_EID_TIM,
    mgmt.u.beacon.variable, len);
    let mut dtim_period: c_int = 1;
    if (tim && tim[1] >= 2)
    dtim_period = tim[3];
    mwl8k_cmd_finalize_join(priv.hw, skb.data, skb.len, dtim_period);
    dev_kfree_skb(skb);
    priv.beacon_skb = core::ptr::null_mut();
    }
    enum {
    MWL8363 = 0,
    MWL8687,
    MWL8366,
    MWL8764,
    };
pub const MWL8K_8366_AP_FW_API: c_int = 3;

pub const MWL8K_8764_AP_FW_API: c_int = 1;

    static struct mwl8k_device_info mwl8k_info_tbl[] = {
    [MWL8363] = {
    .part_name	= "88w8363",
    .helper_image	= "mwl8k/helper_8363.fw",
    .fw_image_sta	= "mwl8k/fmimage_8363.fw",
    },
    [MWL8687] = {
    .part_name	= "88w8687",
    .helper_image	= "mwl8k/helper_8687.fw",
    .fw_image_sta	= "mwl8k/fmimage_8687.fw",
    },
    [MWL8366] = {
    .part_name	= "88w8366",
    .helper_image	= "mwl8k/helper_8366.fw",
    .fw_image_sta	= "mwl8k/fmimage_8366.fw",
    .fw_image_ap	= MWL8K_8366_AP_FW(MWL8K_8366_AP_FW_API),
    .fw_api_ap	= MWL8K_8366_AP_FW_API,
    .ap_rxd_ops	= &rxd_ap_ops,
    },
    [MWL8764] = {
    .part_name	= "88w8764",
    .fw_image_ap	= MWL8K_8764_AP_FW(MWL8K_8764_AP_FW_API),
    .fw_api_ap	= MWL8K_8764_AP_FW_API,
    .ap_rxd_ops	= &rxd_ap_ops,
    },
    };
    MODULE_FIRMWARE("mwl8k/helper_8363.fw");
    MODULE_FIRMWARE("mwl8k/fmimage_8363.fw");
    MODULE_FIRMWARE("mwl8k/helper_8687.fw");
    MODULE_FIRMWARE("mwl8k/fmimage_8687.fw");
    MODULE_FIRMWARE("mwl8k/helper_8366.fw");
    MODULE_FIRMWARE("mwl8k/fmimage_8366.fw");
    MODULE_FIRMWARE(MWL8K_8366_AP_FW(MWL8K_8366_AP_FW_API));
    static const struct pci_device_id mwl8k_pci_id_table[] = {
    { PCI_VDEVICE(MARVELL, 0x2a0a), .driver_data = MWL8363, },
    { PCI_VDEVICE(MARVELL, 0x2a0c), .driver_data = MWL8363, },
    { PCI_VDEVICE(MARVELL, 0x2a24), .driver_data = MWL8363, },
    { PCI_VDEVICE(MARVELL, 0x2a2b), .driver_data = MWL8687, },
    { PCI_VDEVICE(MARVELL, 0x2a30), .driver_data = MWL8687, },
    { PCI_VDEVICE(MARVELL, 0x2a40), .driver_data = MWL8366, },
    { PCI_VDEVICE(MARVELL, 0x2a41), .driver_data = MWL8366, },
    { PCI_VDEVICE(MARVELL, 0x2a42), .driver_data = MWL8366, },
    { PCI_VDEVICE(MARVELL, 0x2a43), .driver_data = MWL8366, },
    { PCI_VDEVICE(MARVELL, 0x2b36), .driver_data = MWL8764, },
    { },
    };
    MODULE_DEVICE_TABLE(pci, mwl8k_pci_id_table);
#[no_mangle]
unsafe extern "C" fn mwl8k_request_alt_fw(priv: *mut mwl8k_priv) -> c_int {
    static int mwl8k_request_alt_fw(struct mwl8k_priv *priv)
    {
    int rc;
    printk(KERN_ERR "%s: Error requesting preferred fw %s.\n"
    "Trying alternative firmware %s\n", pci_name(priv.pdev),
    priv.fw_pref, priv.fw_alt);
    rc = mwl8k_request_fw(priv, priv.fw_alt, &priv.fw_ucode, true);
    if (rc) {
    printk(KERN_ERR "%s: Error requesting alt fw %s\n",
    pci_name(priv.pdev), priv.fw_alt);
    return rc;
    }
    return 0;
    }
    static int mwl8k_firmware_load_success(struct mwl8k_priv *priv);
#[no_mangle]
unsafe extern "C" fn mwl8k_fw_state_machine(fw: *const firmware, context: *mut c_void) {
    static void mwl8k_fw_state_machine(const struct firmware *fw, void *context)
    {
    struct mwl8k_priv *priv = context;
    struct mwl8k_device_info *di = priv.device_info;
    int rc;
    switch (priv.fw_state) {
    case FW_STATE_INIT:
    if (!fw) {
    printk(KERN_ERR "%s: Error requesting helper fw %s\n",
    pci_name(priv.pdev), di.helper_image);
    goto fail;
    }
    priv.fw_helper = fw;
    rc = mwl8k_request_fw(priv, priv.fw_pref, &priv.fw_ucode,
    true);
    if (rc && priv.fw_alt) {
    rc = mwl8k_request_alt_fw(priv);
    if (rc)
    goto fail;
    priv.fw_state = FW_STATE_LOADING_ALT;
    } else if (rc)
    goto fail;
    else
    priv.fw_state = FW_STATE_LOADING_PREF;
    break;
    case FW_STATE_LOADING_PREF:
    if (!fw) {
    if (priv.fw_alt) {
    rc = mwl8k_request_alt_fw(priv);
    if (rc)
    goto fail;
    priv.fw_state = FW_STATE_LOADING_ALT;
    } else
    goto fail;
    } else {
    priv.fw_ucode = fw;
    rc = mwl8k_firmware_load_success(priv);
    if (rc)
    goto fail;
    else
    complete(&priv.firmware_loading_complete);
    }
    break;
    case FW_STATE_LOADING_ALT:
    if (!fw) {
    printk(KERN_ERR "%s: Error requesting alt fw %s\n",
    pci_name(priv.pdev), di.helper_image);
    goto fail;
    }
    priv.fw_ucode = fw;
    rc = mwl8k_firmware_load_success(priv);
    if (rc)
    goto fail;
    else
    complete(&priv.firmware_loading_complete);
    break;
    default:
    printk(KERN_ERR "%s: Unexpected firmware loading state: %d\n",
    MWL8K_NAME, priv.fw_state);
    BUG_ON(1);
    }
    return;
    fail:
    priv.fw_state = FW_STATE_ERROR;
    complete(&priv.firmware_loading_complete);
    mwl8k_release_firmware(priv);
    device_release_driver(&priv.pdev.dev);
    }
pub const MAX_RESTART_ATTEMPTS: c_int = 1;
    static int mwl8k_init_firmware(struct ieee80211_hw *hw, char *fw_image,
    bool nowait)
    {
    struct mwl8k_priv *priv = hw.priv;
    int rc;
    let mut count: c_int = MAX_RESTART_ATTEMPTS;
    retry:
// Reset firmware and hardware
    mwl8k_hw_reset(priv);
// Ask userland hotplug daemon for the device firmware
    rc = mwl8k_request_firmware(priv, fw_image, nowait);
    if (rc) {
    wiphy_err(hw.wiphy, "Firmware files not found\n");
    return rc;
    }
    if (nowait)
    return rc;
// Load firmware into hardware
    rc = mwl8k_load_firmware(hw);
    if (rc)
    wiphy_err(hw.wiphy, "Cannot start firmware\n");
// Reclaim memory once firmware is successfully loaded
    mwl8k_release_firmware(priv);
    if (rc && count) {
// FW did not start successfully;
// lets try one more time
//
    count--;
    wiphy_err(hw.wiphy, "Trying to reload the firmware again\n");
    msleep(20);
    goto retry;
    }
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn mwl8k_init_txqs(hw: *mut ieee80211_hw) -> c_int {
    static int mwl8k_init_txqs(struct ieee80211_hw *hw)
    {
    struct mwl8k_priv *priv = hw.priv;
    let mut rc: c_int = 0;
    int i;
    for (i = 0; i < mwl8k_tx_queues(priv); i++) {
    rc = mwl8k_txq_init(hw, i);
    if (rc)
    break;
    if (priv.ap_fw)
    iowrite32(priv.txq[i].txd_dma,
    priv.sram + priv.txq_offset[i]);
    }
    return rc;
    }
// initialize hw after successfully loading a firmware image
#[no_mangle]
unsafe extern "C" fn mwl8k_probe_hw(hw: *mut ieee80211_hw) -> c_int {
    static int mwl8k_probe_hw(struct ieee80211_hw *hw)
    {
    struct mwl8k_priv *priv = hw.priv;
    let mut rc: c_int = 0;
    int i;
    if (priv.ap_fw) {
    priv.rxd_ops = priv.device_info.ap_rxd_ops;
    if (priv.rxd_ops == core::ptr::null_mut()) {
    wiphy_err(hw.wiphy,
    "Driver does not have AP firmware image support for this hardware\n");
    rc = -ENOENT;
    goto err_stop_firmware;
    }
    } else {
    priv.rxd_ops = &rxd_sta_ops;
    }
    priv.sniffer_enabled = false;
    priv.wmm_enabled = false;
    priv.pending_tx_pkts = 0;
    atomic_set(&priv.watchdog_event_pending, 0);
    rc = mwl8k_rxq_init(hw, 0);
    if (rc)
    goto err_stop_firmware;
    rxq_refill(hw, 0, INT_MAX);
// For the sta firmware, we need to know the dma addresses of tx queues
// before sending MWL8K_CMD_GET_HW_SPEC.  So we must initialize them
// prior to issuing this command.  But for the AP case, we learn the
// total number of queues from the result CMD_GET_HW_SPEC, so for this
// case we must initialize the tx queues after.
//
    priv.num_ampdu_queues = 0;
    if (!priv.ap_fw) {
    rc = mwl8k_init_txqs(hw);
    if (rc)
    goto err_free_queues;
    }
    iowrite32(0, priv.regs + MWL8K_HIU_A2H_INTERRUPT_STATUS);
    iowrite32(0, priv.regs + MWL8K_HIU_A2H_INTERRUPT_MASK);
    iowrite32(MWL8K_A2H_INT_TX_DONE|MWL8K_A2H_INT_RX_READY|
    MWL8K_A2H_INT_BA_WATCHDOG,
    priv.regs + MWL8K_HIU_A2H_INTERRUPT_CLEAR_SEL);
    iowrite32(MWL8K_A2H_INT_OPC_DONE,
    priv.regs + MWL8K_HIU_A2H_INTERRUPT_STATUS_MASK);
    rc = request_irq(priv.pdev.irq, mwl8k_interrupt,
    IRQF_SHARED, MWL8K_NAME, hw);
    if (rc) {
    wiphy_err(hw.wiphy, "failed to register IRQ handler\n");
    goto err_free_queues;
    }
//
// When hw restart is requested,
// mac80211 will take care of clearing
// the ampdu streams, so do not clear
// the ampdu state here
//
    if (!priv.hw_restart_in_progress)
    memset(priv.ampdu, 0, sizeof(priv.ampdu));
//
// Temporarily enable interrupts.  Initial firmware host
// commands use interrupts and avoid polling.  Disable
// interrupts when done.
//
    iowrite32(MWL8K_A2H_EVENTS, priv.regs + MWL8K_HIU_A2H_INTERRUPT_MASK);
// Get config data, mac addrs etc
    if (priv.ap_fw) {
    rc = mwl8k_cmd_get_hw_spec_ap(hw);
    if (!rc)
    rc = mwl8k_init_txqs(hw);
    if (!rc)
    rc = mwl8k_cmd_set_hw_spec(hw);
    } else {
    rc = mwl8k_cmd_get_hw_spec_sta(hw);
    }
    if (rc) {
    wiphy_err(hw.wiphy, "Cannot initialise firmware\n");
    goto err_free_irq;
    }
// Turn radio off
    rc = mwl8k_cmd_radio_disable(hw);
    if (rc) {
    wiphy_err(hw.wiphy, "Cannot disable\n");
    goto err_free_irq;
    }
// Clear MAC address
    rc = mwl8k_cmd_set_mac_addr(hw, core::ptr::null_mut(), "\x00\x00\x00\x00\x00\x00");
    if (rc) {
    wiphy_err(hw.wiphy, "Cannot clear MAC address\n");
    goto err_free_irq;
    }
// Configure Antennas
    rc = mwl8k_cmd_rf_antenna(hw, MWL8K_RF_ANTENNA_RX, 0x3);
    if (rc)
    wiphy_warn(hw.wiphy, "failed to set # of RX antennas");
    rc = mwl8k_cmd_rf_antenna(hw, MWL8K_RF_ANTENNA_TX, 0x7);
    if (rc)
    wiphy_warn(hw.wiphy, "failed to set # of TX antennas");
// Disable interrupts
    iowrite32(0, priv.regs + MWL8K_HIU_A2H_INTERRUPT_MASK);
    free_irq(priv.pdev.irq, hw);
    wiphy_info(hw.wiphy, "%s v%d, %pm, %s firmware %u.%u.%u.%u\n",
    priv.device_info.part_name,
    priv.hw_rev, hw.wiphy.perm_addr,
    priv.ap_fw ? "AP" : "STA",
    (priv.fw_rev >> 24) & 0xff, (priv.fw_rev >> 16) & 0xff,
    (priv.fw_rev >> 8) & 0xff, priv.fw_rev & 0xff);
    return 0;
    err_free_irq:
    iowrite32(0, priv.regs + MWL8K_HIU_A2H_INTERRUPT_MASK);
    free_irq(priv.pdev.irq, hw);
    err_free_queues:
    for (i = 0; i < mwl8k_tx_queues(priv); i++)
    mwl8k_txq_deinit(hw, i);
    mwl8k_rxq_deinit(hw, 0);
    err_stop_firmware:
    mwl8k_hw_reset(priv);
    return rc;
    }
//
// invoke mwl8k_reload_firmware to change the firmware image after the device
// has already been registered
//
#[no_mangle]
unsafe extern "C" fn mwl8k_reload_firmware(hw: *mut ieee80211_hw, fw_image: *mut c_char) -> c_int {
    static int mwl8k_reload_firmware(struct ieee80211_hw *hw, char *fw_image)
    {
    int i, rc = 0;
    struct mwl8k_priv *priv = hw.priv;
    struct mwl8k_vif *vif, *tmp_vif;
    mwl8k_stop(hw, false);
    mwl8k_rxq_deinit(hw, 0);
//
// All the existing interfaces are re-added by the ieee80211_reconfig;
// which means driver should remove existing interfaces before calling
// ieee80211_restart_hw
//
    if (priv.hw_restart_in_progress)
    list_for_each_entry_safe(vif, tmp_vif, &priv.vif_list, list)
    mwl8k_remove_vif(priv, vif);
    for (i = 0; i < mwl8k_tx_queues(priv); i++)
    mwl8k_txq_deinit(hw, i);
    rc = mwl8k_init_firmware(hw, fw_image, false);
    if (rc)
    goto fail;
    rc = mwl8k_probe_hw(hw);
    if (rc)
    goto fail;
    if (priv.hw_restart_in_progress)
    return rc;
    rc = mwl8k_start(hw);
    if (rc)
    goto fail;
    rc = mwl8k_config(hw, -1, ~0);
    if (rc)
    goto fail;
    for (i = 0; i < MWL8K_TX_WMM_QUEUES; i++) {
    rc = mwl8k_conf_tx(hw, core::ptr::null_mut(), 0, i, &priv.wmm_params[i]);
    if (rc)
    goto fail;
    }
    return rc;
    fail:
    printk(KERN_WARNING "mwl8k: Failed to reload firmware image.\n");
    return rc;
    }
    static const struct ieee80211_iface_limit ap_if_limits[] = {
    { .max = 8,	.types = BIT(NL80211_IFTYPE_AP) },
    { .max = 1,	.types = BIT(NL80211_IFTYPE_STATION) },
    };
    static const struct ieee80211_iface_combination ap_if_comb = {
    .limits = ap_if_limits,
    .n_limits = ARRAY_SIZE(ap_if_limits),
    .max_interfaces = 8,
    .num_different_channels = 1,
    };
#[no_mangle]
unsafe extern "C" fn mwl8k_firmware_load_success(priv: *mut mwl8k_priv) -> c_int {
    static int mwl8k_firmware_load_success(struct mwl8k_priv *priv)
    {
    struct ieee80211_hw *hw = priv.hw;
    int i, rc;
    rc = mwl8k_load_firmware(hw);
    mwl8k_release_firmware(priv);
    if (rc) {
    wiphy_err(hw.wiphy, "Cannot start firmware\n");
    return rc;
    }
//
// Extra headroom is the size of the required DMA header
// minus the size of the smallest 802.11 frame (CTS frame).
//
    hw.extra_tx_headroom =
    sizeof(struct mwl8k_dma_data) - sizeof(struct ieee80211_cts);
    hw.extra_tx_headroom -= priv.ap_fw ? REDUCED_TX_HEADROOM : 0;
    hw.queues = MWL8K_TX_WMM_QUEUES;
// Set rssi values to dBm
    ieee80211_hw_set(hw, SIGNAL_DBM);
    ieee80211_hw_set(hw, HAS_RATE_CONTROL);
//
// Ask mac80211 to not to trigger PS mode
// based on PM bit of incoming frames.
//
    if (priv.ap_fw)
    ieee80211_hw_set(hw, AP_LINK_PS);
    hw.vif_data_size = sizeof(struct mwl8k_vif);
    hw.sta_data_size = sizeof(struct mwl8k_sta);
    priv.macids_used = 0;
    INIT_LIST_HEAD(&priv.vif_list);
// Set default radio state and preamble
    priv.radio_on = false;
    priv.radio_short_preamble = false;
// Finalize join worker
    INIT_WORK(&priv.finalize_join_worker, mwl8k_finalize_join_worker);
// Handle watchdog ba events
    INIT_WORK(&priv.watchdog_ba_handle, mwl8k_watchdog_ba_events);
// To reload the firmware if it crashes
    INIT_WORK(&priv.fw_reload, mwl8k_hw_restart_work);
// TX reclaim and RX tasklets.
    tasklet_setup(&priv.poll_tx_task, mwl8k_tx_poll);
    tasklet_disable(&priv.poll_tx_task);
    tasklet_setup(&priv.poll_rx_task, mwl8k_rx_poll);
    tasklet_disable(&priv.poll_rx_task);
// Power management cookie
    priv.cookie = dma_alloc_coherent(&priv.pdev.dev, 4,
    &priv.cookie_dma, GFP_KERNEL);
    if (priv.cookie == core::ptr::null_mut())
    return -ENOMEM;
    mutex_init(&priv.fw_mutex);
    priv.fw_mutex_owner = core::ptr::null_mut();
    priv.fw_mutex_depth = 0;
    priv.hostcmd_wait = core::ptr::null_mut();
    spin_lock_init(&priv.tx_lock);
    spin_lock_init(&priv.stream_lock);
    priv.tx_wait = core::ptr::null_mut();
    rc = mwl8k_probe_hw(hw);
    if (rc)
    goto err_free_cookie;
    hw.wiphy.interface_modes = 0;
    if (priv.ap_macids_supported || priv.device_info.fw_image_ap) {
    hw.wiphy.interface_modes |= BIT(NL80211_IFTYPE_AP);
    hw.wiphy.interface_modes |= BIT(NL80211_IFTYPE_STATION);
    hw.wiphy.iface_combinations = &ap_if_comb;
    hw.wiphy.n_iface_combinations = 1;
    }
    if (priv.sta_macids_supported || priv.device_info.fw_image_sta)
    hw.wiphy.interface_modes |= BIT(NL80211_IFTYPE_STATION);
    wiphy_ext_feature_set(hw.wiphy, NL80211_EXT_FEATURE_CQM_RSSI_LIST);
    rc = ieee80211_register_hw(hw);
    if (rc) {
    wiphy_err(hw.wiphy, "Cannot register device\n");
    goto err_unprobe_hw;
    }
    return 0;
    err_unprobe_hw:
    for (i = 0; i < mwl8k_tx_queues(priv); i++)
    mwl8k_txq_deinit(hw, i);
    mwl8k_rxq_deinit(hw, 0);
    err_free_cookie:
    if (priv.cookie != core::ptr::null_mut())
    dma_free_coherent(&priv.pdev.dev, 4, priv.cookie,
    priv.cookie_dma);
    return rc;
    }
    static int mwl8k_probe(struct pci_dev *pdev,
    const struct pci_device_id *id)
    {
    static int printed_version;
    struct ieee80211_hw *hw;
    struct mwl8k_priv *priv;
    struct mwl8k_device_info *di;
    int rc;
    if (!printed_version) {
    printk(KERN_INFO "%s version %s\n", MWL8K_DESC, MWL8K_VERSION);
    printed_version = 1;
    }
    rc = pci_enable_device(pdev);
    if (rc) {
    printk(KERN_ERR "%s: Cannot enable new PCI device\n",
    MWL8K_NAME);
    return rc;
    }
    rc = pci_request_regions(pdev, MWL8K_NAME);
    if (rc) {
    printk(KERN_ERR "%s: Cannot obtain PCI resources\n",
    MWL8K_NAME);
    goto err_disable_device;
    }
    pci_set_master(pdev);
    hw = ieee80211_alloc_hw(sizeof(*priv), &mwl8k_ops);
    if (hw == core::ptr::null_mut()) {
    printk(KERN_ERR "%s: ieee80211 alloc failed\n", MWL8K_NAME);
    rc = -ENOMEM;
    goto err_free_reg;
    }
    SET_IEEE80211_DEV(hw, &pdev.dev);
    pci_set_drvdata(pdev, hw);
    priv = hw.priv;
    priv.hw = hw;
    priv.pdev = pdev;
    priv.device_info = &mwl8k_info_tbl[id.driver_data];
    if (id.driver_data == MWL8764)
    priv.is_8764 = true;
    priv.sram = pci_iomap(pdev, 0, 0x10000);
    if (priv.sram == core::ptr::null_mut()) {
    wiphy_err(hw.wiphy, "Cannot map device SRAM\n");
    rc = -EIO;
    goto err_iounmap;
    }
//
// If BAR0 is a 32 bit BAR, the register BAR will be BAR1.
// If BAR0 is a 64 bit BAR, the register BAR will be BAR2.
//
    priv.regs = pci_iomap(pdev, 1, 0x10000);
    if (priv.regs == core::ptr::null_mut()) {
    priv.regs = pci_iomap(pdev, 2, 0x10000);
    if (priv.regs == core::ptr::null_mut()) {
    wiphy_err(hw.wiphy, "Cannot map device registers\n");
    rc = -EIO;
    goto err_iounmap;
    }
    }
//
// Choose the initial fw image depending on user input.  If a second
// image is available, make it the alternative image that will be
// loaded if the first one fails.
//
    init_completion(&priv.firmware_loading_complete);
    di = priv.device_info;
    if (ap_mode_default && di.fw_image_ap) {
    priv.fw_pref = di.fw_image_ap;
    priv.fw_alt = di.fw_image_sta;
    } else if (!ap_mode_default && di.fw_image_sta) {
    priv.fw_pref = di.fw_image_sta;
    priv.fw_alt = di.fw_image_ap;
    } else if (ap_mode_default && !di.fw_image_ap && di.fw_image_sta) {
    printk(KERN_WARNING "AP fw is unavailable.  Using STA fw.");
    priv.fw_pref = di.fw_image_sta;
    } else if (!ap_mode_default && !di.fw_image_sta && di.fw_image_ap) {
    printk(KERN_WARNING "STA fw is unavailable.  Using AP fw.");
    priv.fw_pref = di.fw_image_ap;
    }
    rc = mwl8k_init_firmware(hw, priv.fw_pref, true);
    if (rc)
    goto err_stop_firmware;
    priv.hw_restart_in_progress = false;
    priv.running_bsses = 0;
    return rc;
    err_stop_firmware:
    mwl8k_hw_reset(priv);
    err_iounmap:
    if (priv.regs != core::ptr::null_mut())
    pci_iounmap(pdev, priv.regs);
    if (priv.sram != core::ptr::null_mut())
    pci_iounmap(pdev, priv.sram);
    ieee80211_free_hw(hw);
    err_free_reg:
    pci_release_regions(pdev);
    err_disable_device:
    pci_disable_device(pdev);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn mwl8k_remove(pdev: *mut pci_dev) {
    static void mwl8k_remove(struct pci_dev *pdev)
    {
    struct ieee80211_hw *hw = pci_get_drvdata(pdev);
    struct mwl8k_priv *priv;
    int i;
    if (hw == core::ptr::null_mut())
    return;
    priv = hw.priv;
    wait_for_completion(&priv.firmware_loading_complete);
    if (priv.fw_state == FW_STATE_ERROR) {
    mwl8k_hw_reset(priv);
    goto unmap;
    }
    ieee80211_stop_queues(hw);
    ieee80211_unregister_hw(hw);
// Remove TX reclaim and RX tasklets.
    tasklet_kill(&priv.poll_tx_task);
    tasklet_kill(&priv.poll_rx_task);
// Stop hardware
    mwl8k_hw_reset(priv);
// Return all skbs to mac80211
    for (i = 0; i < mwl8k_tx_queues(priv); i++)
    mwl8k_txq_reclaim(hw, i, INT_MAX, 1);
    for (i = 0; i < mwl8k_tx_queues(priv); i++)
    mwl8k_txq_deinit(hw, i);
    mwl8k_rxq_deinit(hw, 0);
    dma_free_coherent(&priv.pdev.dev, 4, priv.cookie, priv.cookie_dma);
    unmap:
    pci_iounmap(pdev, priv.regs);
    pci_iounmap(pdev, priv.sram);
    ieee80211_free_hw(hw);
    pci_release_regions(pdev);
    pci_disable_device(pdev);
    }
    static struct pci_driver mwl8k_driver = {
    .name		= MWL8K_NAME,
    .id_table	= mwl8k_pci_id_table,
    .probe		= mwl8k_probe,
    .remove		= mwl8k_remove,
    };
    module_pci_driver(mwl8k_driver);
    MODULE_DESCRIPTION(MWL8K_DESC);
    MODULE_VERSION(MWL8K_VERSION);
    MODULE_AUTHOR("Lennert Buytenhek <buytenh@marvell.com>");
    MODULE_LICENSE("GPL");
