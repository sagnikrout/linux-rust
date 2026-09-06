//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath10k/targaddrs.h
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
// Copyright (c) 2005-2011 Atheros Communications Inc.
// Copyright (c) 2011-2016 Qualcomm Atheros, Inc.
//

//
// xxx_HOST_INTEREST_ADDRESS is the address in Target RAM of the
// host_interest structure.  It must match the address of the _host_interest
// symbol (see linker script).
//
// Host Interest is shared between Host and Target in order to coordinate
// between the two, and is intended to remain constant (with additions only
// at the end) across software releases.
//
// All addresses are available here so that it's possible to
// write a single binary that works with all Target Types.
// May be used in assembler code as well as C.
//
pub const QCA988X_HOST_INTEREST_ADDRESS: c_uint = 0x00400800;
pub const HOST_INTEREST_MAX_SIZE: c_uint = 0x200;
//
// These are items that the Host may need to access via BMI or via the
// Diagnostic Window. The position of items in this structure must remain
// constant across firmware revisions! Types for each item must be fixed
// size across target and host platforms. More items may be added at the end.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_interest {
//
// Pointer to application-defined area, if any.
// Set by Target application during startup.
//
    pub /: *mut *mut u32 hi_app_host_interest; / 0x00,
// Pointer to register dump area, valid after Target crash.
    pub /: *mut *mut u32 hi_failure_state; / 0x04,
// Pointer to debug logging header
    pub /: *mut *mut u32 hi_dbglog_hdr; / 0x08,
    pub /: *mut *mut u32 hi_unused0c; / 0x0c,
//
// General-purpose flag bits, similar to SOC_OPTION_* flags.
// Can be used by application rather than by OS.
//
    pub /: *mut *mut u32 hi_option_flag; / 0x10,
//
// Boolean that determines whether or not to
// display messages on the serial port.
//
    pub /: *mut *mut u32 hi_serial_enable; / 0x14,
// Start address of DataSet index, if any
    pub /: *mut *mut u32 hi_dset_list_head; / 0x18,
// Override Target application start address
    pub /: *mut *mut u32 hi_app_start; / 0x1c,
// Clock and voltage tuning
    pub /: *mut *mut u32 hi_skip_clock_init; / 0x20,
    pub /: *mut *mut u32 hi_core_clock_setting; / 0x24,
    pub /: *mut *mut u32 hi_cpu_clock_setting; / 0x28,
    pub /: *mut *mut u32 hi_system_sleep_setting; / 0x2c,
    pub /: *mut *mut u32 hi_xtal_control_setting; / 0x30,
    pub /: *mut *mut u32 hi_pll_ctrl_setting_24ghz; / 0x34,
    pub /: *mut *mut u32 hi_pll_ctrl_setting_5ghz; / 0x38,
    pub /: *mut *mut u32 hi_ref_voltage_trim_setting; / 0x3c,
    pub /: *mut *mut u32 hi_clock_info; / 0x40,
// Host uses BE CPU or not
    pub /: *mut *mut u32 hi_be; / 0x44,
    pub /: *mut *mut *mut *mut u32 hi_stack; / normal stack / / 0x48,
    pub /: *mut *mut *mut *mut u32 hi_err_stack; / error stack / / 0x4c,
    pub /: *mut *mut u32 hi_desired_cpu_speed_hz; / 0x50,
// Pointer to Board Data
    pub /: *mut *mut u32 hi_board_data; / 0x54,
//
// Indication of Board Data state:
// 0: board data is not yet initialized.
// 1: board data is initialized; unknown size
// >1: number of bytes of initialized board data
//
    pub /: *mut *mut u32 hi_board_data_initialized; / 0x58,
    pub /: *mut *mut u32 hi_dset_ram_index_table; / 0x5c,
    pub /: *mut *mut u32 hi_desired_baud_rate; / 0x60,
    pub /: *mut *mut u32 hi_dbglog_config; / 0x64,
    pub /: *mut *mut u32 hi_end_ram_reserve_sz; / 0x68,
    pub /: *mut *mut u32 hi_mbox_io_block_sz; / 0x6c,
    pub /: *mut *mut u32 hi_num_bpatch_streams; / 0x70 -- unused,
    pub /: *mut *mut u32 hi_mbox_isr_yield_limit; / 0x74,
    pub /: *mut *mut u32 hi_refclk_hz; / 0x78,
    pub /: *mut *mut u32 hi_ext_clk_detected; / 0x7c,
    pub /: *mut *mut u32 hi_dbg_uart_txpin; / 0x80,
    pub /: *mut *mut u32 hi_dbg_uart_rxpin; / 0x84,
    pub /: *mut *mut u32 hi_hci_uart_baud; / 0x88,
    pub /: *mut *mut u32 hi_hci_uart_pin_assignments; / 0x8C,
    pub /: *mut *mut u32 hi_hci_uart_baud_scale_val; / 0x90,
    pub /: *mut *mut u32 hi_hci_uart_baud_step_val; / 0x94,
    pub /: *mut *mut u32 hi_allocram_start; / 0x98,
    pub /: *mut *mut u32 hi_allocram_sz; / 0x9c,
    pub /: *mut *mut u32 hi_hci_bridge_flags; / 0xa0,
    pub /: *mut *mut u32 hi_hci_uart_support_pins; / 0xa4,
    pub /: *mut *mut u32 hi_hci_uart_pwr_mgmt_params; / 0xa8,
//
// 0xa8 - [1]: 0 = UART FC active low, 1 = UART FC active high
// [31:16]: wakeup timeout in ms
//
// Pointer to extended board Data
    pub /: *mut *mut u32 hi_board_ext_data; / 0xac,
    pub /: *mut *mut u32 hi_board_ext_data_config; / 0xb0,
//
// Bit [0]  :   valid
// Bit[31:16:   size
//
// hi_reset_flag is used to do some stuff when target reset.
// such as restore app_start after warm reset or
// preserve host Interest area, or preserve ROM data, literals etc.
//
    pub /: *mut *mut u32 hi_reset_flag; / 0xb4,
// indicate hi_reset_flag is valid
    pub /: *mut *mut u32 hi_reset_flag_valid; / 0xb8,
    pub /: *mut *mut u32 hi_hci_uart_pwr_mgmt_params_ext; / 0xbc,
// 0xbc - [31:0]: idle timeout in ms
// ACS flags
    pub /: *mut *mut u32 hi_acs_flags; / 0xc0,
    pub /: *mut *mut u32 hi_console_flags; / 0xc4,
    pub /: *mut *mut u32 hi_nvram_state; / 0xc8,
    pub /: *mut *mut u32 hi_option_flag2; / 0xcc,
// If non-zero, override values sent to Host in WMI_READY event.
    pub /: *mut *mut u32 hi_sw_version_override; / 0xd0,
    pub /: *mut *mut u32 hi_abi_version_override; / 0xd4,
//
// Percentage of high priority RX traffic to total expected RX traffic
// applicable only to ar6004
//
    pub /: *mut *mut u32 hi_hp_rx_traffic_ratio; / 0xd8,
// test applications flags
    pub /: *mut *mut u32 hi_test_apps_related; / 0xdc,
// location of test script
    pub /: *mut *mut u32 hi_ota_testscript; / 0xe0,
// location of CAL data
    pub /: *mut *mut u32 hi_cal_data; / 0xe4,
// Number of packet log buffers
    pub /: *mut *mut u32 hi_pktlog_num_buffers; / 0xe8,
// wow extension configuration
    pub /: *mut *mut u32 hi_wow_ext_config; / 0xec,
    pub /: *mut *mut u32 hi_pwr_save_flags; / 0xf0,
// Spatial Multiplexing Power Save (SMPS) options
    pub /: *mut *mut u32 hi_smps_options; / 0xf4,
// Interconnect-specific state
    pub /: *mut *mut u32 hi_interconnect_state; / 0xf8,
// Coex configuration flags
    pub /: *mut *mut u32 hi_coex_config; / 0xfc,
// Early allocation support
    pub /: *mut *mut u32 hi_early_alloc; / 0x100,
// FW swap field
//
// Bits of this 32bit word will be used to pass specific swap
// instruction to FW
//
// Bit 0 -- AP Nart descriptor no swap. When this bit is set
// FW will not swap TX descriptor. Meaning packets are formed
// on the target processor.
//
// Bit 1 - unused
    pub /: *mut *mut u32 hi_fw_swap; / 0x104,
// global arenas pointer address, used by host driver debug
    pub /: *mut *mut u32 hi_dynamic_mem_arenas_addr; / 0x108,
// allocated bytes of DRAM use by allocated
    pub /: *mut *mut u32 hi_dynamic_mem_allocated; / 0x10C,
// remaining bytes of DRAM
    pub /: *mut *mut u32 hi_dynamic_mem_remaining; / 0x110,
// memory track count, configured by host
    pub /: *mut *mut u32 hi_dynamic_mem_track_max; / 0x114,
// minidump buffer
    pub /: *mut *mut u32 hi_minidump; / 0x118,
// bdata's sig and key addr
    pub /: *mut *mut u32 hi_bd_sig_key; / 0x11c,
    pub __packed: },

// Bits defined in hi_option_flag
// Enable timer workaround
pub const HI_OPTION_TIMER_WAR: c_uint = 0x01;
// Limit BMI command credits
pub const HI_OPTION_BMI_CRED_LIMIT: c_uint = 0x02;
// Relay Dot11 hdr to/from host
pub const HI_OPTION_RELAY_DOT11_HDR: c_uint = 0x04;
// MAC addr method 0-locally administred 1-globally unique addrs
pub const HI_OPTION_MAC_ADDR_METHOD: c_uint = 0x08;
// Firmware Bridging
pub const HI_OPTION_FW_BRIDGE: c_uint = 0x10;
// Enable CPU profiling
pub const HI_OPTION_ENABLE_PROFILE: c_uint = 0x20;
// Disable debug logging
pub const HI_OPTION_DISABLE_DBGLOG: c_uint = 0x40;
// Skip Era Tracking
pub const HI_OPTION_SKIP_ERA_TRACKING: c_uint = 0x80;
// Disable PAPRD (debug)
pub const HI_OPTION_PAPRD_DISABLE: c_uint = 0x100;
pub const HI_OPTION_NUM_DEV_LSB: c_uint = 0x200;
pub const HI_OPTION_NUM_DEV_MSB: c_uint = 0x800;
pub const HI_OPTION_DEV_MODE_LSB: c_uint = 0x1000;
pub const HI_OPTION_DEV_MODE_MSB: c_uint = 0x8000000;
// Disable LowFreq Timer Stabilization
pub const HI_OPTION_NO_LFT_STBL: c_uint = 0x10000000;
// Skip regulatory scan
pub const HI_OPTION_SKIP_REG_SCAN: c_uint = 0x20000000;
//
// Do regulatory scan during init before
// sending WMI ready event to host
//
pub const HI_OPTION_INIT_REG_SCAN: c_uint = 0x40000000;
// REV6: Do not adjust memory map
pub const HI_OPTION_SKIP_MEMMAP: c_uint = 0x80000000;
pub const HI_OPTION_MAC_ADDR_METHOD_SHIFT: c_int = 3;
// 2 bits of hi_option_flag are used to represent 3 modes
pub const HI_OPTION_FW_MODE_IBSS: c_uint = 0x0 /* IBSS Mode */;
pub const HI_OPTION_FW_MODE_BSS_STA: c_uint = 0x1 /* STA Mode */;
pub const HI_OPTION_FW_MODE_AP: c_uint = 0x2 /* AP Mode */;
pub const HI_OPTION_FW_MODE_BT30AMP: c_uint = 0x3 /* BT30 AMP Mode */;
// 2 bits of hi_option flag are usedto represent 4 submodes
pub const HI_OPTION_FW_SUBMODE_NONE: c_uint = 0x0  /* Normal mode */;
pub const HI_OPTION_FW_SUBMODE_P2PDEV: c_uint = 0x1  /* p2p device mode */;
pub const HI_OPTION_FW_SUBMODE_P2PCLIENT: c_uint = 0x2 /* p2p client mode */;
pub const HI_OPTION_FW_SUBMODE_P2PGO: c_uint = 0x3 /* p2p go mode */;
// Num dev Mask
pub const HI_OPTION_NUM_DEV_MASK: c_uint = 0x7;
pub const HI_OPTION_NUM_DEV_SHIFT: c_uint = 0x9;
// firmware bridging
pub const HI_OPTION_FW_BRIDGE_SHIFT: c_uint = 0x04;
//
// Fw Mode/SubMode Mask
// -----------------------------------------------------------------------------
// SUB   |   SUB   |   SUB   |  SUB    |         |         |         |
// MODE[3] | MODE[2] | MODE[1] | MODE[0] | MODE[3] | MODE[2] | MODE[1] | MODE[0]
// (2)   |   (2)   |   (2)   |   (2)   |   (2)   |   (2)   |   (2)   |   (2)
// -----------------------------------------------------------------------------
//
pub const HI_OPTION_FW_MODE_BITS: c_uint = 0x2;
pub const HI_OPTION_FW_MODE_MASK: c_uint = 0x3;
pub const HI_OPTION_FW_MODE_SHIFT: c_uint = 0xC;
pub const HI_OPTION_ALL_FW_MODE_MASK: c_uint = 0xFF;
pub const HI_OPTION_FW_SUBMODE_BITS: c_uint = 0x2;
pub const HI_OPTION_FW_SUBMODE_MASK: c_uint = 0x3;
pub const HI_OPTION_FW_SUBMODE_SHIFT: c_uint = 0x14;
pub const HI_OPTION_ALL_FW_SUBMODE_MASK: c_uint = 0xFF00;
pub const HI_OPTION_ALL_FW_SUBMODE_SHIFT: c_uint = 0x8;
// hi_option_flag2 options
pub const HI_OPTION_OFFLOAD_AMSDU: c_uint = 0x01;
pub const HI_OPTION_DFS_SUPPORT: c_uint = 0x02 /* Enable DFS support */;
pub const HI_OPTION_ENABLE_RFKILL: c_uint = 0x04 /* RFKill Enable Feature*/;
pub const HI_OPTION_RADIO_RETENTION_DISABLE: c_uint = 0x08 /* Disable radio retention */;
pub const HI_OPTION_EARLY_CFG_DONE: c_uint = 0x10 /* Early configuration is complete */;
pub const HI_OPTION_RF_KILL_SHIFT: c_uint = 0x2;
pub const HI_OPTION_RF_KILL_MASK: c_uint = 0x1;
// hi_reset_flag
// preserve App Start address
pub const HI_RESET_FLAG_PRESERVE_APP_START: c_uint = 0x01;
// preserve host interest
pub const HI_RESET_FLAG_PRESERVE_HOST_INTEREST: c_uint = 0x02;
// preserve ROM data
pub const HI_RESET_FLAG_PRESERVE_ROMDATA: c_uint = 0x04;
pub const HI_RESET_FLAG_PRESERVE_NVRAM_STATE: c_uint = 0x08;
pub const HI_RESET_FLAG_PRESERVE_BOOT_INFO: c_uint = 0x10;
pub const HI_RESET_FLAG_WARM_RESET: c_uint = 0x20;
// define hi_fw_swap bits
pub const HI_DESC_IN_FW_BIT: c_uint = 0x01;
// indicate the reset flag is valid
pub const HI_RESET_FLAG_IS_VALID: c_uint = 0x12345678;
// ACS is enabled

// Use physical WWAN device

// Use test VAP

// SDIO/mailbox ACS flag definitions

//
// If both SDIO_CRASH_DUMP_ENHANCEMENT_HOST and SDIO_CRASH_DUMP_ENHANCEMENT_FW
// flags are set, then crashdump upload will be done using the BMI host/target
// communication channel.
//
// HOST to support using BMI dump FW memory when hit assert
pub const HI_OPTION_SDIO_CRASH_DUMP_ENHANCEMENT_HOST: c_uint = 0x400;
// FW to support using BMI dump FW memory when hit assert
pub const HI_OPTION_SDIO_CRASH_DUMP_ENHANCEMENT_FW: c_uint = 0x800;
//
// CONSOLE FLAGS
//
// Bit Range  Meaning
// ---------  --------------------------------
// 2..0     UART ID (0 = Default)
// 3       Baud Select (0 = 9600, 1 = 115200)
// 30..4    Reserved
// 31      Enable Console
//

pub const HI_CONSOLE_FLAGS_UART_SHIFT: c_int = 0;

// SM power save options

//
// WOW Extension configuration
//
// Bit Range  Meaning
// ---------  --------------------------------
// 8..0     Size of each WOW pattern (max 511)
// 15..9    Number of patterns per list (max 127)
// 17..16   Number of lists (max 4)
// 30..18   Reserved
// 31       Enabled
//
// set values (except enable) to zeros for default settings
//

pub const HI_WOW_EXT_NUM_LIST_SHIFT: c_int = 16;

pub const HI_WOW_EXT_NUM_PATTERNS_SHIFT: c_int = 9;

pub const HI_WOW_EXT_PATTERN_SIZE_SHIFT: c_int = 0;

//
// Early allocation configuration
// Support RAM bank configuration before BMI done and this eases the memory
// allocation at very early stage
// Bit Range  Meaning
// ---------  ----------------------------------
// [0:3]      number of bank assigned to be IRAM
// [4:15]     reserved
// [16:31]    magic number
//
// Note:
// 1. target firmware would check magic number and if it's a match, firmware
// would consider the bits[0:15] are valid and base on that to calculate
// the end of DRAM. Early allocation would be located at that area and
// may be reclaimed when necessary
// 2. if no magic number is found, early allocation would happen at "_end"
// symbol of ROM which is located before the app-data and might NOT be
// re-claimable. If this is adopted, link script should keep this in
// mind to avoid data corruption.
//
pub const HI_EARLY_ALLOC_MAGIC: c_uint = 0x6d8a;
pub const HI_EARLY_ALLOC_MAGIC_MASK: c_uint = 0xffff0000;
pub const HI_EARLY_ALLOC_MAGIC_SHIFT: c_int = 16;
pub const HI_EARLY_ALLOC_IRAM_BANKS_MASK: c_uint = 0x0000000f;
pub const HI_EARLY_ALLOC_IRAM_BANKS_SHIFT: c_int = 0;

// power save flag bit definitions
pub const HI_PWR_SAVE_LPL_ENABLED: c_uint = 0x1;
// b1-b3 reserved
// b4-b5 : dev0 LPL type : 0 - none
// 1- Reduce Pwr Search
// 2- Reduce Pwr Listen
//
// b6-b7 : dev1 LPL type and so on for Max 8 devices
pub const HI_PWR_SAVE_LPL_DEV0_LSB: c_int = 4;
pub const HI_PWR_SAVE_LPL_DEV_MASK: c_uint = 0x3;
// power save related utility macros

// Reserve 1024 bytes for extended board data
pub const QCA988X_BOARD_DATA_SZ: c_int = 7168;
pub const QCA988X_BOARD_EXT_DATA_SZ: c_int = 0;
pub const QCA9887_BOARD_DATA_SZ: c_int = 7168;
pub const QCA9887_BOARD_EXT_DATA_SZ: c_int = 0;
pub const QCA6174_BOARD_DATA_SZ: c_int = 8192;
pub const QCA6174_BOARD_EXT_DATA_SZ: c_int = 0;

pub const QCA9377_BOARD_EXT_DATA_SZ: c_int = 0;
pub const QCA99X0_BOARD_DATA_SZ: c_int = 12288;
pub const QCA99X0_BOARD_EXT_DATA_SZ: c_int = 0;
// Dual band extended board data
pub const QCA99X0_EXT_BOARD_DATA_SZ: c_int = 2048;
pub const EXT_BOARD_ADDRESS_OFFSET: c_uint = 0x3000;
pub const QCA4019_BOARD_DATA_SZ: c_int = 12064;
pub const QCA4019_BOARD_EXT_DATA_SZ: c_int = 0;
pub const WCN3990_BOARD_DATA_SZ: c_int = 26328;
pub const WCN3990_BOARD_EXT_DATA_SZ: c_int = 0;
