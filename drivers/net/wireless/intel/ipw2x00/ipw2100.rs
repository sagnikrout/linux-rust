//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/ipw2x00/ipw2100.h
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

pub const IPW_DL_UNINIT: c_uint = 0x80000000;
pub const IPW_DL_NONE: c_uint = 0x00000000;
pub const IPW_DL_ALL: c_uint = 0x7FFFFFFF;
//
// To use the debug system;
//
// If you are defining a new debug classification, simply add it to the #define
// list here in the form of:
//
// #define IPW_DL_xxxx VALUE
//
// shifting value to the left one bit from the previous entry.  xxxx should be
// the name of the classification (for example, WEP)
//
// You then need to either add a IPW2100_xxxx_DEBUG() macro definition for your
// classification, or use IPW_DEBUG(IPW_DL_xxxx, ...) whenever you want
// to send output to that classification.
//
// To add your debug level to the list of levels seen when you perform
//
// % cat /proc/net/ipw2100/debug_level
//
// you simply need to add your entry to the ipw2100_debug_levels array.
//
// If you do not see debug_level in /proc/net/ipw2100 then you do not have
// CONFIG_IPW2100_DEBUG defined in your kernel configuration
//

pub const NUMBER_OF_BD_PER_COMMAND_PACKET: c_int = 1;
pub const NUMBER_OF_BD_PER_DATA_PACKET: c_int = 2;
pub const IPW_MAX_BDS: c_int = 6;
pub const NUMBER_OF_OVERHEAD_BDS_PER_PACKETR: c_int = 2;
pub const NUMBER_OF_BDS_TO_LEAVE_FOR_COMMANDS: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bd_status {
    pub reserved:4: u8 nlf:1, txType:2, intEnabled:1,,
    pub fields: },
    pub field: u8,
    pub info: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw2100_bd {
    pub host_addr: u32,
    pub buf_length: u32,
    pub status: bd_status,
// number of fragments for frame (should be set only for
// 1st TBD)
    pub num_fragments: u8,
    pub reserved: [u8; 6],
    pub __packed: },

pub const IPW_BD_STATUS_TX_FRAME_802_3: c_uint = 0x00;
pub const IPW_BD_STATUS_TX_FRAME_NOT_LAST_FRAGMENT: c_uint = 0x01;
pub const IPW_BD_STATUS_TX_FRAME_COMMAND: c_uint = 0x02;
pub const IPW_BD_STATUS_TX_FRAME_802_11: c_uint = 0x04;
pub const IPW_BD_STATUS_TX_INTERRUPT_ENABLE: c_uint = 0x08;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw2100_bd_queue {
// driver (virtual) pointer to queue
    pub drv: *mut ipw2100_bd,
// firmware (physical) pointer to queue
    pub nic: dma_addr_t,
// Length of phy memory allocated for BDs
    pub size: u32,
// Number of BDs in queue (and in array)
    pub entries: u32,
// Number of available BDs (invalid for NIC BDs)
    pub available: u32,
// Offset of oldest used BD in array (next one to
// check for completion)
    pub oldest: u32,
// Offset of next available (unused) BD
    pub next: u32,
}

pub const RX_QUEUE_LENGTH: c_int = 256;
pub const TX_QUEUE_LENGTH: c_int = 256;
pub const HW_QUEUE_LENGTH: c_int = 256;

pub const STATUS_TYPE_MASK: c_uint = 0x0000000f;
pub const COMMAND_STATUS_VAL: c_int = 0;
pub const STATUS_CHANGE_VAL: c_int = 1;
pub const P80211_DATA_VAL: c_int = 2;
pub const P8023_DATA_VAL: c_int = 3;
pub const HOST_NOTIFICATION_VAL: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw2100_status {
    pub frame_size: u32,
    pub status_fields: u16,
    pub flags: u8,

    pub rssi: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw2100_status_queue {
// driver (virtual) pointer to queue
    pub drv: *mut ipw2100_status,
// firmware (physical) pointer to queue
    pub nic: dma_addr_t,
// Length of phy memory allocated for BDs
    pub size: u32,
}

pub const HOST_COMMAND_PARAMS_REG_LEN: c_int = 100;
pub const CMD_STATUS_PARAMS_REG_LEN: c_int = 3;
pub const IPW_WPA_CAPABILITIES: c_uint = 0x1;
pub const IPW_WPA_LISTENINTERVAL: c_uint = 0x2;
pub const IPW_WPA_AP_ADDRESS: c_uint = 0x4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw2100_wpa_assoc_frame {
    pub fixed_ie_mask: u16,
    pub capab_info: u16,
    pub listen_interval: u16,
    pub current_ap: [u8; ETH_ALEN],
    pub fixed_ies: },
    pub var_ie_len: u32,
    pub var_ie: [u8; IPW_MAX_VAR_IE_LEN],
}

pub const IPW_BSS: c_int = 1;
pub const IPW_MONITOR: c_int = 2;
pub const IPW_IBSS: c_int = 3;
//
// @struct _tx_cmd - HWCommand
// @brief H/W command structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw2100_cmd_header {
    pub host_command_reg: u32,
    pub host_command_reg1: u32,
    pub sequence: u32,
    pub host_command_len_reg: u32,
    pub host_command_params_reg: [u32; HOST_COMMAND_PARAMS_REG_LEN],
    pub cmd_status_reg: u32,
    pub cmd_status_params_reg: [u32; CMD_STATUS_PARAMS_REG_LEN],
    pub rxq_base_ptr: u32,
    pub rxq_next_ptr: u32,
    pub rxq_host_ptr: u32,
    pub txq_base_ptr: u32,
    pub txq_next_ptr: u32,
    pub txq_host_ptr: u32,
    pub tx_status_reg: u32,
    pub reserved: u32,
    pub status_change_reg: u32,
    pub reserved1: [u32; 3],
    pub ordinal1_ptr: *mut u32,
    pub ordinal2_ptr: *mut u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw2100_data_header {
    pub host_command_reg: u32,
    pub host_command_reg1: u32,
    pub driver: u8 encrypted; // BOOLEAN in win! TRUE if frame is enc by,
    pub NIC: u8 needs_encryption; // BOOLEAN in win! TRUE if frma need to be enc in,
    pub key: u8 wep_index; // 0 no key, 1-4 key index, 0xff immediate,
    pub IV: u8 key_size; // 0 no imm key, 0x5 64bit encr, 0xd 128bit encr, 0x10 128bit encr and 128bit,
    pub key: [u8; 16],
    pub reserved: u8 reserved[10]; // f/w,
    pub src_addr: [u8; ETH_ALEN],
    pub dst_addr: [u8; ETH_ALEN],
    pub fragment_size: u16,
    pub __packed: },
// Host command data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_command {
    pub ID: u32 host_command; // COMMAND,
    pub ID: u32 host_command1; // COMMAND,
    pub (ID): u32 host_command_sequence; // UNIQUE COMMAND NUMBER,
    pub LENGTH: u32 host_command_length; //,
    pub PARAMETERS: u32 host_command_parameters[HOST_COMMAND_PARAMS_REG_LEN]; // COMMAND,
    pub __packed: },
    pub ipw2100_reset_event: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw2100_tx_packet {
    pub type: c_int,
    pub index: c_int,
    pub cmd: *mut ipw2100_cmd_header,
    pub cmd_phys: dma_addr_t,
    pub c_struct: },
    pub data: *mut ipw2100_data_header,
    pub data_phys: dma_addr_t,
    pub txb: *mut libipw_txb,
    pub d_struct: },
    pub info: },
    pub jiffy_start: c_int,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw2100_rx_packet {
    pub rxp: *mut ipw2100_rx,
    pub dma_addr: dma_addr_t,
    pub jiffy_start: c_int,
    pub skb: *mut sk_buff,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw2100_ordinals {
    pub table1_addr: u32,
    pub table2_addr: u32,
    pub table1_size: u32,
    pub table2_size: u32,
}

// Host Notification header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw2100_notification {
    pub /: *mut *mut u32 hnhdr_subtype; / type of host notification,
    pub data: *mut *mut u32 hnhdr_size; / size in bytes of,
    pub __packed: },
pub const MAX_KEY_SIZE: c_int = 16;
pub const MAX_KEYS: c_int = 8;

pub const IPW_AUTH_OPEN: c_int = 0;
pub const IPW_AUTH_SHARED: c_int = 1;
pub const IPW_AUTH_LEAP: c_int = 2;
pub const IPW_AUTH_LEAP_CISCO_ID: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct statistic {
    pub value: c_int,
    pub hi: c_int,
    pub lo: c_int,
}

pub const IPW2100_ERROR_QUEUE: c_int = 5;
// Power management code: enable or disable?

// Internal NIC states

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw2100_priv {
    pub ioaddr: *mut void __iomem,
    pub /: *mut *mut int stop_hang_check; / Set 1 when shutting down to kill hang_check,
    pub /: *mut *mut int stop_rf_kill; / Set 1 when shutting down to kill rf_kill,
    pub ieee: *mut libipw_device,
    pub status: c_ulong,
    pub config: c_ulong,
    pub capability: c_ulong,
// Statistics
    pub resets: c_int,
    pub reset_backoff: time64_t,
// Context
    pub essid: [u8; IW_ESSID_MAX_SIZE],
    pub essid_len: u8,
    pub bssid: [u8; ETH_ALEN],
    pub channel: u8,
    pub last_mode: c_int,
    pub connect_start: time64_t,
    pub last_reset: time64_t,
    pub channel_mask: u32,
    pub fatal_error: u32,
    pub fatal_errors: [u32; IPW2100_ERROR_QUEUE],
    pub fatal_index: u32,
    pub eeprom_version: c_int,
    pub firmware_version: c_int,
    pub hw_features: c_ulong,
    pub hangs: c_int,
    pub last_rtc: u32,
    pub /: *mut *mut int dump_raw; / 1 to dump raw bytes in /sys/.../memory,
    pub snapshot: [*mut u8; 0x30],
    pub mandatory_bssid_mac: [u8; ETH_ALEN],
    pub mac_addr: [u8; ETH_ALEN],
    pub power_mode: c_int,
    pub messages_sent: c_int,
    pub short_retry_limit: c_int,
    pub long_retry_limit: c_int,
    pub rts_threshold: u32,
    pub frag_threshold: u32,
    pub in_isr: c_int,
    pub tx_rates: u32,
    pub tx_power: c_int,
    pub beacon_interval: u32,
    pub 1]: char nick[IW_ESSID_MAX_SIZE +,
    pub status_queue: ipw2100_status_queue,
    pub txq_stat: statistic,
    pub rxq_stat: statistic,
    pub rx_queue: ipw2100_bd_queue,
    pub tx_queue: ipw2100_bd_queue,
    pub rx_buffers: *mut ipw2100_rx_packet,
    pub fw_pend_stat: statistic,
    pub fw_pend_list: list_head,
    pub msg_free_stat: statistic,
    pub msg_pend_stat: statistic,
    pub msg_free_list: list_head,
    pub msg_pend_list: list_head,
    pub msg_buffers: *mut ipw2100_tx_packet,
    pub tx_free_stat: statistic,
    pub tx_pend_stat: statistic,
    pub tx_free_list: list_head,
    pub tx_pend_list: list_head,
    pub tx_buffers: *mut ipw2100_tx_packet,
    pub ordinals: ipw2100_ordinals,
    pub pci_dev: *mut pci_dev,
    pub dir_dev: *mut proc_dir_entry,
    pub net_dev: *mut net_device,
    pub wstats: iw_statistics,
    pub irq_tasklet: tasklet_struct,
    pub reset_work: delayed_work,
    pub security_work: delayed_work,
    pub wx_event_work: delayed_work,
    pub hang_check: delayed_work,
    pub rf_kill: delayed_work,
    pub scan_event: delayed_work,
    pub user_requested_scan: c_int,
// Track time in suspend, using CLOCK_BOOTTIME
    pub suspend_at: time64_t,
    pub suspend_time: time64_t,
    pub interrupts: u32,
    pub tx_interrupts: c_int,
    pub rx_interrupts: c_int,
    pub inta_other: c_int,
    pub low_lock: spinlock_t,
    pub action_mutex: mutex,
    pub adapter_mutex: mutex,
    pub wait_command_queue: wait_queue_head_t,
}

//
// Host Command -> From Driver to FW
//
// Host command identifiers
//
pub const HOST_COMPLETE: c_int = 2;
pub const SYSTEM_CONFIG: c_int = 6;
pub const SSID: c_int = 8;
pub const MANDATORY_BSSID: c_int = 9;
pub const AUTHENTICATION_TYPE: c_int = 10;
pub const ADAPTER_ADDRESS: c_int = 11;
pub const PORT_TYPE: c_int = 12;
pub const INTERNATIONAL_MODE: c_int = 13;
pub const CHANNEL: c_int = 14;
pub const RTS_THRESHOLD: c_int = 15;
pub const FRAG_THRESHOLD: c_int = 16;
pub const POWER_MODE: c_int = 17;
pub const TX_RATES: c_int = 18;
pub const BASIC_TX_RATES: c_int = 19;
pub const WEP_KEY_INFO: c_int = 20;
pub const WEP_KEY_INDEX: c_int = 25;
pub const WEP_FLAGS: c_int = 26;
pub const ADD_MULTICAST: c_int = 27;
pub const CLEAR_ALL_MULTICAST: c_int = 28;
pub const BEACON_INTERVAL: c_int = 29;
pub const ATIM_WINDOW: c_int = 30;
pub const CLEAR_STATISTICS: c_int = 31;
pub const SEND: c_int = 33;
pub const TX_POWER_INDEX: c_int = 36;
pub const BROADCAST_SCAN: c_int = 43;
pub const CARD_DISABLE: c_int = 44;
pub const PREFERRED_BSSID: c_int = 45;
pub const SET_SCAN_OPTIONS: c_int = 46;
pub const SCAN_DWELL_TIME: c_int = 47;
pub const SWEEP_TABLE: c_int = 48;
pub const AP_OR_STATION_TABLE: c_int = 49;
pub const GROUP_ORDINALS: c_int = 50;
pub const SHORT_RETRY_LIMIT: c_int = 51;
pub const LONG_RETRY_LIMIT: c_int = 52;
pub const HOST_PRE_POWER_DOWN: c_int = 58;
pub const CARD_DISABLE_PHY_OFF: c_int = 61;
pub const MSDU_TX_RATES: c_int = 62;
// Rogue AP Detection
pub const SET_STATION_STAT_BITS: c_int = 64;
pub const CLEAR_STATIONS_STAT_BITS: c_int = 65;

pub const SET_SECURITY_INFORMATION: c_int = 67;
pub const DISASSOCIATION_BSSID: c_int = 68;
pub const SET_WPA_IE: c_int = 69;
// system configuration bit mask:
pub const IPW_CFG_MONITOR: c_uint = 0x00004;
pub const IPW_CFG_PREAMBLE_AUTO: c_uint = 0x00010;
pub const IPW_CFG_IBSS_AUTO_START: c_uint = 0x00020;
pub const IPW_CFG_LOOPBACK: c_uint = 0x00100;
pub const IPW_CFG_ANSWER_BCSSID_PROBE: c_uint = 0x00800;
pub const IPW_CFG_BT_SIDEBAND_SIGNAL: c_uint = 0x02000;
pub const IPW_CFG_802_1x_ENABLE: c_uint = 0x04000;
pub const IPW_CFG_BSS_MASK: c_uint = 0x08000;
pub const IPW_CFG_IBSS_MASK: c_uint = 0x10000;

// RESERVED (1<<2)

pub const IPW_NIC_FATAL_ERROR: c_uint = 0x2A7F0;

pub const IPW_MEM_SRAM_HOST_SHARED_LOWER_BOUND: c_uint = 0x200;

pub const IPW_BIT_GPIO_GPIO1_MASK: c_uint = 0x0000000C;
pub const IPW_BIT_GPIO_GPIO3_MASK: c_uint = 0x000000C0;
pub const IPW_BIT_GPIO_GPIO1_ENABLE: c_uint = 0x00000008;
pub const IPW_BIT_GPIO_RF_KILL: c_uint = 0x00010000;
pub const IPW_BIT_GPIO_LED_OFF: c_uint = 0x00002000	// Bit 13 = 1;
pub const IPW_REG_DOMAIN_0_OFFSET: c_uint = 0x0000;

pub const IPW_REG_INDIRECT_ADDR_MASK: c_uint = 0x00FFFFFC;
pub const IPW_INTERRUPT_MASK: c_uint = 0xC1010013;
pub const IPW2100_CONTROL_REG: c_uint = 0x220000;
pub const IPW2100_CONTROL_PHY_OFF: c_uint = 0x8;
pub const IPW2100_COMMAND: c_uint = 0x00300004;
pub const IPW2100_COMMAND_PHY_ON: c_uint = 0x0;
pub const IPW2100_COMMAND_PHY_OFF: c_uint = 0x1;
// in DEBUG_AREA, values of memory always 0xd55555d5

pub const IPW_DATA_DOA_DEBUG_VALUE: c_uint = 0xd55555d5;
pub const IPW_INTERNAL_REGISTER_HALT_AND_RESET: c_uint = 0x003000e0;

// BD ring queue read/write difference
pub const IPW_BD_QUEUE_W_R_MIN_SPARE: c_int = 2;
pub const IPW_CACHE_LINE_LENGTH_DEFAULT: c_uint = 0x80;

pub const IPW_MAX_802_11_PAYLOAD_LENGTH: c_int = 2312;
pub const IPW_MAX_ACCEPTABLE_TX_FRAME_LENGTH: c_int = 1536;
pub const IPW_MIN_ACCEPTABLE_RX_FRAME_LENGTH: c_int = 60;

pub const IPW_802_11_FCS_LENGTH: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw2100_rx {
    pub payload: [c_uchar; IPW_RX_NIC_BUFFER_LENGTH],
    pub header: libipw_hdr_4addr,
    pub status: u32,
    pub notification: ipw2100_notification,
    pub command: ipw2100_cmd_header,
    pub rx_data: },
    pub __packed: },
// Bit 0-7 are for 802.11b tx rates - .  Bit 5-7 are reserved
pub const TX_RATE_1_MBIT: c_uint = 0x0001;
pub const TX_RATE_2_MBIT: c_uint = 0x0002;
pub const TX_RATE_5_5_MBIT: c_uint = 0x0004;
pub const TX_RATE_11_MBIT: c_uint = 0x0008;
pub const TX_RATE_MASK: c_uint = 0x000F;
pub const DEFAULT_TX_RATES: c_uint = 0x000F;
pub const IPW_POWER_MODE_CAM: c_uint = 0x00	//(always on);
pub const IPW_POWER_INDEX_1: c_uint = 0x01;
pub const IPW_POWER_INDEX_2: c_uint = 0x02;
pub const IPW_POWER_INDEX_3: c_uint = 0x03;
pub const IPW_POWER_INDEX_4: c_uint = 0x04;
pub const IPW_POWER_INDEX_5: c_uint = 0x05;
pub const IPW_POWER_AUTO: c_uint = 0x06;
pub const IPW_POWER_MASK: c_uint = 0x0F;
pub const IPW_POWER_ENABLED: c_uint = 0x10;

pub const IPW_TX_POWER_AUTO: c_int = 0;
pub const IPW_TX_POWER_ENHANCED: c_int = 1;
pub const IPW_TX_POWER_DEFAULT: c_int = 32;
pub const IPW_TX_POWER_MIN: c_int = 0;
pub const IPW_TX_POWER_MAX: c_int = 16;

pub const IPW_TX_POWER_MAX_DBM: c_int = 16;
pub const FW_SCAN_DONOT_ASSOCIATE: c_uint = 0x0001	// Dont Attempt to Associate after Scan;
pub const FW_SCAN_PASSIVE: c_uint = 0x0008	// Force PASSSIVE Scan;
pub const REG_MIN_CHANNEL: c_int = 0;
pub const REG_MAX_CHANNEL: c_int = 14;
pub const REG_CHANNEL_MASK: c_uint = 0x00003FFF;
pub const IPW_IBSS_11B_DEFAULT_MASK: c_uint = 0x87ff;

pub const HOST_COMMAND_WAIT: c_int = 0;
pub const HOST_COMMAND_NO_WAIT: c_int = 1;
pub const LOCK_NONE: c_int = 0;
pub const LOCK_DRIVER: c_int = 1;
pub const LOCK_FW: c_int = 2;
pub const TYPE_SWEEP_ORD: c_uint = 0x000D;
pub const TYPE_IBSS_STTN_ORD: c_uint = 0x000E;
pub const TYPE_BSS_AP_ORD: c_uint = 0x000F;
pub const TYPE_RAW_BEACON_ENTRY: c_uint = 0x0010;
pub const TYPE_CALIBRATION_DATA: c_uint = 0x0011;
pub const TYPE_ROGUE_AP_DATA: c_uint = 0x0012;
pub const TYPE_ASSOCIATION_REQUEST: c_uint = 0x0013;
pub const TYPE_REASSOCIATION_REQUEST: c_uint = 0x0014;
pub const HW_FEATURE_RFKILL: c_uint = 0x0001;
pub const RF_KILLSWITCH_OFF: c_int = 1;
pub const RF_KILLSWITCH_ON: c_int = 0;
pub const IPW_COMMAND_POOL_SIZE: c_int = 40;
pub const IPW_START_ORD_TAB_1: c_int = 1;
pub const IPW_START_ORD_TAB_2: c_int = 1000;

pub const BSS_ID_LENGTH: c_int = 6;
// Fixed size data: Ordinal Table 1
// Transmit statistics
// Receive statistics
// PSP Statistics
// Association and roaming
// AP table entry. set to 0 if not associated
// hops or no prob_ responses in last 3 minutes
// load at the AP
// eligible group
// Other statistics
// IPW_ORD_COUNTRY_CHANNELS:
// For 11b the lower 2-byte are used for channels from 1-14
// and the higher 2-byte are not used.
    pub ORDINALTABLE1: },
// ordinal table 2
// Variable length data:
pub const IPW_FIRST_VARIABLE_LENGTH_ORDINAL: c_int = 1001;
    pub "0.08.011"): IPW_ORD_STAT_FW_VER_NUM = 1012, // 14 bytes: fw version ID string as in (a.bb.ccc;,
    pub 2002"): IPW_ORD_STAT_FW_DATE = 1013, // 14 bytes: fw date string (mmm dd yyyy; "Mar 13,
    pub FW: } ORDINALTABLE2; // NS - means Not Supported by,
pub const IPW_LAST_VARIABLE_LENGTH_ORDINAL: c_int = 1018;

pub const IPW_HOST_FW_SHARED_AREA0: c_uint = 0x0002f200;
pub const IPW_HOST_FW_SHARED_AREA0_END: c_uint = 0x0002f510	// 0x310 bytes;
pub const IPW_HOST_FW_SHARED_AREA1: c_uint = 0x0002f610;
pub const IPW_HOST_FW_SHARED_AREA1_END: c_uint = 0x0002f630	// 0x20 bytes;
pub const IPW_HOST_FW_SHARED_AREA2: c_uint = 0x0002fa00;
pub const IPW_HOST_FW_SHARED_AREA2_END: c_uint = 0x0002fa20	// 0x20 bytes;
pub const IPW_HOST_FW_SHARED_AREA3: c_uint = 0x0002fc00;
pub const IPW_HOST_FW_SHARED_AREA3_END: c_uint = 0x0002fc10	// 0x10 bytes;
pub const IPW_HOST_FW_INTERRUPT_AREA: c_uint = 0x0002ff80;
pub const IPW_HOST_FW_INTERRUPT_AREA_END: c_uint = 0x00030000	// 0x80 bytes;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw2100_fw_chunk {
    pub buf: *mut c_uchar,
    pub len: c_long,
    pub pos: c_long,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw2100_fw_chunk_set {
    pub data: *const c_void,
    pub size: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw2100_fw {
    pub version: c_int,
    pub fw: ipw2100_fw_chunk_set,
    pub uc: ipw2100_fw_chunk_set,
    pub fw_entry: *const firmware,
}

pub const MAX_FW_VERSION_LEN: c_int = 14;
