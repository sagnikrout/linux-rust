//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath.h
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
// Copyright (c) 2008-2009 Atheros Communications Inc.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

//
// The key cache is used for h/w cipher state and also for
// tracking station state such as the current tx antenna.
// We also setup a mapping table between key cache slot indices
// and station state to short-circuit node lookups on rx.
// Different parts have different size key caches.  We handle
// up to ATH_KEYMAX entries (could dynamically allocate state).
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_ani {
    pub caldone: bool,
    pub longcal_timer: c_uint,
    pub shortcal_timer: c_uint,
    pub resetcal_timer: c_uint,
    pub checkani_timer: c_uint,
    pub timer: timer_list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_cycle_counters {
    pub cycles: u32,
    pub rx_busy: u32,
    pub rx_frame: u32,
    pub tx_frame: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath_device_state {
    ATH_HW_UNAVAILABLE,
    ATH_HW_INITIALIZED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath_op_flags {
    ATH_OP_INVALID,
    ATH_OP_BEACONS,
    ATH_OP_ANI_RUN,
    ATH_OP_PRIM_STA_VIF,
    ATH_OP_HW_RESET,
    ATH_OP_SCANNING,
    ATH_OP_MULTI_CHANNEL,
    ATH_OP_WOW_ENABLED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath_bus_type {
    ATH_PCI,
    ATH_AHB,
    ATH_USB,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_dmn_pair_mapping {
    pub reg_domain: u16,
    pub reg_5ghz_ctl: u16,
    pub reg_2ghz_ctl: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_regulatory {
    pub alpha2: [c_char; 2],
    pub region: nl80211_dfs_regions,
    pub country_code: u16,
    pub max_power_level: u16,
    pub current_rd: u16,
    pub power_limit: i16,
    pub regpair: *mut reg_dmn_pair_mapping,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath_crypt_caps {
    ATH_CRYPT_CAP_CIPHER_AESCCM		= BIT(0),
    ATH_CRYPT_CAP_MIC_COMBINED		= BIT(1),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_keyval {
    pub kv_type: u8,
    pub kv_pad: u8,
    pub kv_len: u16,
    pub /: *mut *mut u8 kv_val[16]; / TK,
    pub /: *mut *mut u8 kv_mic[8]; / Michael MIC key,
    pub hardware: *mut *mut u8 kv_txmic[8]; / Michael MIC TX key (used only if the,
// supports both MIC keys in the same key cache entry;
// in that case, kv_mic is the RX key)
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath_cipher {
    ATH_CIPHER_WEP = 0,
    ATH_CIPHER_AES_OCB = 1,
    ATH_CIPHER_AES_CCM = 2,
    ATH_CIPHER_CKIP = 3,
    ATH_CIPHER_TKIP = 4,
    ATH_CIPHER_CLR = 5,
    ATH_CIPHER_MIC = 127
}

//
// struct ath_ops - Register read/write operations
//
// @read: Register read
// @multi_read: Multiple register read
// @write: Register write
// @enable_write_buffer: Enable multiple register writes
// @write_flush: flush buffered register writes and disable buffering
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_ops {
    pub reg_offset): *mut *mut *mut unsigned int (read)(void , u32,
    pub count): *mut *mut *mut *mut *mut void (multi_read)(void , u32 addr, u32 val, u16,
    pub reg_offset): *mut *mut *mut void (write)(void , u32 val, u32,
    pub ): *mut *mut void (enable_write_buffer)(void,
    pub ): *mut *mut void (write_flush) (void,
    pub clr): *mut *mut *mut u32 (rmw)(void , u32 reg_offset, u32 set, u32,
    pub ): *mut *mut void (enable_rmw_buffer)(void,
    pub ): *mut *mut void (rmw_flush) (void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_ps_ops {
    pub common): *mut *mut void (wakeup)(struct ath_common,
    pub common): *mut *mut void (restore)(struct ath_common,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_common {
    pub ah: *mut c_void,
    pub priv: *mut c_void,
    pub hw: *mut ieee80211_hw,
    pub debug_mask: c_int,
    pub state: ath_device_state,
    pub op_flags: c_ulong,
    pub ani: ath_ani,
    pub cachelsz: u16,
    pub curaid: u16,
    pub macaddr: [u8; ETH_ALEN],
    pub __aligned(2): u8 curbssid[ETH_ALEN],
    pub bssidmask: [u8; ETH_ALEN],
    pub rx_bufsize: u32,
    pub keymax: u32,
    pub ATH_KEYMAX): DECLARE_BITMAP(keymap,,
    pub ATH_KEYMAX): DECLARE_BITMAP(tkip_keymap,,
    pub ATH_KEYMAX): DECLARE_BITMAP(ccmp_keymap,,
    pub crypt_caps: ath_crypt_caps,
    pub clockrate: c_uint,
    pub cc_lock: spinlock_t,
    pub cc_ani: ath_cycle_counters,
    pub cc_survey: ath_cycle_counters,
    pub regulatory: ath_regulatory,
    pub reg_world_copy: ath_regulatory,
    pub ops: *const ath_ops,
    pub bus_ops: *const ath_bus_ops,
    pub ps_ops: *const ath_ps_ops,
    pub btcoex_enabled: bool,
    pub disable_ani: bool,
    pub bt_ant_diversity: bool,
    pub last_rssi: c_int,
    pub sbands: [ieee80211_supported_band; NUM_NL80211_BANDS],
}

extern "C" {
    pub fn ath_is_mybeacon(common: *mut ath_common, hdr: *mut ieee80211_hdr) -> bool;
}
extern "C" {
    pub fn ath_hw_setbssidmask(common: *mut ath_common);
}
extern "C" {
    pub fn ath_key_delete(common: *mut ath_common, hw_key_idx: u8);
}
extern "C" {
    pub fn ath_hw_keyreset(common: *mut ath_common, entry: u16) -> bool;
}
extern "C" {
    pub fn ath_hw_keysetmac(common: *mut ath_common, entry: u16, mac: *const u8) -> bool;
}
extern "C" {
    pub fn ath_hw_cycle_counters_update(common: *mut ath_common);
}
extern "C" {
    pub fn ath_hw_get_listen_time(common: *mut ath_common) -> i32;
}

//
// enum ath_debug_level - atheros wireless debug level
//
// @ATH_DBG_RESET: reset processing
// @ATH_DBG_QUEUE: hardware queue management
// @ATH_DBG_EEPROM: eeprom processing
// @ATH_DBG_CALIBRATE: periodic calibration
// @ATH_DBG_INTERRUPT: interrupt processing
// @ATH_DBG_REGULATORY: regulatory processing
// @ATH_DBG_ANI: adaptive noise immunitive processing
// @ATH_DBG_XMIT: basic xmit operation
// @ATH_DBG_BEACON: beacon handling
// @ATH_DBG_CONFIG: configuration of the hardware
// @ATH_DBG_FATAL: fatal errors, this is the default, DBG_DEFAULT
// @ATH_DBG_PS: power save processing
// @ATH_DBG_HWTIMER: hardware timer handling
// @ATH_DBG_BTCOEX: bluetooth coexistance
// @ATH_DBG_BSTUCK: stuck beacons
// @ATH_DBG_MCI: Message Coexistence Interface, a private protocol
// used exclusively for WLAN-BT coexistence starting from
// AR9462.
// @ATH_DBG_DFS: radar datection
// @ATH_DBG_WOW: Wake on Wireless
// @ATH_DBG_DYNACK: dynack handling
// @ATH_DBG_SPECTRAL_SCAN: FFT spectral scan
// @ATH_DBG_ANY: enable all debugging
//
// The debug level is used to control the amount and type of debugging output
// we want to see. Each driver has its own method for enabling debugging and
// modifying debug level states -- but this is typically done through a
// module parameter 'debug' along with a respective 'debug' debugfs file
// entry.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ATH_DEBUG {
    ATH_DBG_RESET		= 0x00000001,
    ATH_DBG_QUEUE		= 0x00000002,
    ATH_DBG_EEPROM		= 0x00000004,
    ATH_DBG_CALIBRATE	= 0x00000008,
    ATH_DBG_INTERRUPT	= 0x00000010,
    ATH_DBG_REGULATORY	= 0x00000020,
    ATH_DBG_ANI		= 0x00000040,
    ATH_DBG_XMIT		= 0x00000080,
    ATH_DBG_BEACON		= 0x00000100,
    ATH_DBG_CONFIG		= 0x00000200,
    ATH_DBG_FATAL		= 0x00000400,
    ATH_DBG_PS		= 0x00000800,
    ATH_DBG_BTCOEX		= 0x00001000,
    ATH_DBG_WMI		= 0x00002000,
    ATH_DBG_BSTUCK		= 0x00004000,
    ATH_DBG_MCI		= 0x00008000,
    ATH_DBG_DFS		= 0x00010000,
    ATH_DBG_WOW		= 0x00020000,
    ATH_DBG_CHAN_CTX	= 0x00040000,
    ATH_DBG_DYNACK		= 0x00080000,
    ATH_DBG_SPECTRAL_SCAN	= 0x00100000,
    ATH_DBG_ANY		= 0xffffffff
}

pub const ATH_DBG_MAX_LEN: c_int = 512;

// Returns string describing opmode, or NULL if unknown mode.

