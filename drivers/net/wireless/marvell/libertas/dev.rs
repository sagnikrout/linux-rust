//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/marvell/libertas/dev.h
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
// This file contains definitions and data structures specific
// to Marvell 802.11 NIC. It contains the Device Information
// structure struct lbs_private..
//

// sleep_params
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sleep_params {
    pub sp_error: u16,
    pub sp_offset: u16,
    pub sp_stabletime: u16,
    pub sp_calcontrol: u8,
    pub sp_extsleepclk: u8,
    pub sp_reserved: u16,
}

// Mesh statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lbs_mesh_stats {
    pub /: *mut *mut u32 fwd_bcast_cnt; / Fwd: Broadcast counter,
    pub /: *mut *mut u32 fwd_unicast_cnt; / Fwd: Unicast counter,
    pub /: *mut *mut u32 fwd_drop_ttl; / Fwd: TTL zero,
    pub /: *mut *mut u32 fwd_drop_rbt; / Fwd: Recently Broadcasted,
    pub /: *mut *mut u32 fwd_drop_noroute; / Fwd: No route to Destination,
    pub /: *mut *mut u32 fwd_drop_nobuf; / Fwd: Run out of internal buffers,
    pub /: *mut *mut u32 drop_blind; / Rx: Dropped by blinding table,
    pub /: *mut *mut u32 tx_failed_cnt; / Tx: Failed transmissions,
}

// Private structure for the MV device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lbs_private {
// Basic networking
    pub dev: *mut net_device,
    pub connect_status: u32,
    pub mcast_work: work_struct,
    pub nr_of_multicastmacaddr: u32,
    pub multicastlist: [u8; MRVDRV_MAX_MULTICAST_LIST_SIZE][ETH_ALEN],
// CFG80211
    pub wdev: *mut wireless_dev,
    pub wiphy_registered: bool,
    pub scan_req: *mut cfg80211_scan_request,
    pub assoc_bss: [u8; ETH_ALEN],
    pub country_code: [u8; IEEE80211_COUNTRY_STRING_LEN],
    pub disassoc_reason: u8,
// Mesh
    pub /: *mut *mut *mut net_device mesh_dev; / Virtual device,

    pub mstats: lbs_mesh_stats,
    pub mesh_tlv: u16,
    pub mesh_channel: u8,

// Debugfs
    pub debugfs_dir: *mut dentry,
    pub debugfs_debug: *mut dentry,
    pub debugfs_files: [*mut dentry; 6],
    pub events_dir: *mut dentry,
    pub debugfs_events_files: [*mut dentry; 6],
    pub regs_dir: *mut dentry,
    pub debugfs_regs_files: [*mut dentry; 6],
// Hardware debugging
    pub mac_offset: u32,
    pub bbp_offset: u32,
    pub rf_offset: u32,
// Power management
    pub psmode: u16,
    pub psstate: u32,
    pub needtowakeup: u8,
// Deep sleep
    pub is_deep_sleep: c_int,
    pub deep_sleep_required: c_int,
    pub is_activity_detected: c_int,
    pub ds_awake_q: wait_queue_head_t,
// Host sleep
    pub is_host_sleep_configured: c_int,
    pub is_host_sleep_activated: c_int,
    pub host_sleep_q: wait_queue_head_t,
// Hardware access
    pub card: *mut c_void,
    pub iface_running: bool,
    pub /: *mut *mut u8 is_polling; / host has to poll the card irq,
    pub fw_ready: u8,
    pub surpriseremoved: u8,
    pub setup_fw_on_resume: u8,
    pub power_up_on_resume: u8,
    pub nb): *mut *mut *mut *mut int (hw_host_to_card) (struct lbs_private priv, u8 type, u8 payload, u16,
    pub priv): *mut *mut void (reset_card) (struct lbs_private,
    pub priv): *mut *mut int (power_save) (struct lbs_private,
    pub priv): *mut *mut int (power_restore) (struct lbs_private,
    pub priv): *mut *mut int (enter_deep_sleep) (struct lbs_private,
    pub priv): *mut *mut int (exit_deep_sleep) (struct lbs_private,
    pub priv): *mut *mut int (reset_deep_sleep_wakeup) (struct lbs_private,
// Adapter info (from EEPROM)
    pub fwrelease: u32,
    pub fwcapinfo: u32,
    pub regioncode: u16,
    pub current_addr: [u8; ETH_ALEN],
    pub copied_hwaddr: u8,
// Command download
    pub dnld_sent: u8,
// bit0 1/0=data_sent/data_tx_done,
    pub seqnum: u16,
    pub cmd_array: *mut cmd_ctrl_node,
    pub cur_cmd: *mut cmd_ctrl_node,
    pub /: *mut *mut list_head cmdfreeq; / free command buffers,
    pub /: *mut *mut list_head cmdpendingq; / pending command buffers,
    pub command_timer: timer_list,
    pub cmd_timed_out: c_int,
// Command responses sent from the hardware to the driver
    pub resp_idx: u8,
    pub resp_buf: [u8; 2][LBS_UPLD_SIZE],
    pub resp_len: [u32; 2],
// Events sent from hardware to driver
    pub event_fifo: kfifo,
// thread to service interrupts
    pub main_thread: *mut task_struct,
    pub waitq: wait_queue_head_t,
    pub work_thread: *mut workqueue_struct,
// Encryption stuff
    pub authtype_auto: u8,
    pub wep_tx_key: u8,
    pub wep_key: [u8; 4][WLAN_KEY_LEN_WEP104],
    pub wep_key_len: [u8; 4],
// Wake On LAN
    pub wol_criteria: u32,
    pub wol_gpio: u8,
    pub wol_gap: u8,
    pub ehs_remove_supported: bool,
// Transmitting
    pub /: *mut *mut int tx_pending_len; / -1 while building packet,
    pub tx_pending_buf: [u8; LBS_UPLD_SIZE],
// protected by hard_start_xmit serialization
    pub txretrycount: u8,
    pub currenttxskb: *mut sk_buff,
    pub tx_lockup_timer: timer_list,
// Locks
    pub lock: mutex,
    pub driver_lock: spinlock_t,
// NIC/link operation characteristics
    pub mac_control: u16,
    pub radio_on: u8,
    pub cur_rate: u8,
    pub channel: u8,
    pub txpower_cur: i16,
    pub txpower_min: i16,
    pub txpower_max: i16,
// Scanning
    pub scan_work: delayed_work,
    pub scan_channel: c_int,
// Queue of things waiting for scan completion
    pub scan_q: wait_queue_head_t,
// Whether the scan was initiated internally and not by cfg80211
    pub internal_scan: bool,
// Firmware load
    pub fw_model: u32,
    pub fw_waitq: wait_queue_head_t,
    pub fw_device: *mut device,
    pub helper_fw: *const firmware,
    pub fw_table: *const lbs_fw_table,
    pub fw_iter: *const lbs_fw_table,
    pub fw_callback: lbs_fw_cb,
}

// Check if there is an interface active.
