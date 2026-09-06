//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath6kl/target.h
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
// Copyright (c) 2004-2010 Atheros Communications Inc.
// Copyright (c) 2011 Qualcomm Atheros, Inc.
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
pub const AR6003_BOARD_DATA_SZ: c_int = 1024;
pub const AR6003_BOARD_EXT_DATA_SZ: c_int = 768;
pub const AR6003_BOARD_EXT_DATA_SZ_V2: c_int = 1024;
pub const AR6004_BOARD_DATA_SZ: c_int = 6144;
pub const AR6004_BOARD_EXT_DATA_SZ: c_int = 0;
pub const RESET_CONTROL_ADDRESS: c_uint = 0x00004000;
pub const RESET_CONTROL_COLD_RST: c_uint = 0x00000100;
pub const RESET_CONTROL_MBOX_RST: c_uint = 0x00000004;
pub const CPU_CLOCK_STANDARD_S: c_int = 0;
pub const CPU_CLOCK_STANDARD: c_uint = 0x00000003;
pub const CPU_CLOCK_ADDRESS: c_uint = 0x00000020;
pub const CLOCK_CONTROL_ADDRESS: c_uint = 0x00000028;
pub const CLOCK_CONTROL_LF_CLK32_S: c_int = 2;
pub const CLOCK_CONTROL_LF_CLK32: c_uint = 0x00000004;
pub const SYSTEM_SLEEP_ADDRESS: c_uint = 0x000000c4;
pub const SYSTEM_SLEEP_DISABLE_S: c_int = 0;
pub const SYSTEM_SLEEP_DISABLE: c_uint = 0x00000001;
pub const LPO_CAL_ADDRESS: c_uint = 0x000000e0;
pub const LPO_CAL_ENABLE_S: c_int = 20;
pub const LPO_CAL_ENABLE: c_uint = 0x00100000;
pub const GPIO_PIN9_ADDRESS: c_uint = 0x0000004c;
pub const GPIO_PIN10_ADDRESS: c_uint = 0x00000050;
pub const GPIO_PIN11_ADDRESS: c_uint = 0x00000054;
pub const GPIO_PIN12_ADDRESS: c_uint = 0x00000058;
pub const GPIO_PIN13_ADDRESS: c_uint = 0x0000005c;
pub const HOST_INT_STATUS_ADDRESS: c_uint = 0x00000400;
pub const HOST_INT_STATUS_ERROR_S: c_int = 7;
pub const HOST_INT_STATUS_ERROR: c_uint = 0x00000080;
pub const HOST_INT_STATUS_CPU_S: c_int = 6;
pub const HOST_INT_STATUS_CPU: c_uint = 0x00000040;
pub const HOST_INT_STATUS_COUNTER_S: c_int = 4;
pub const HOST_INT_STATUS_COUNTER: c_uint = 0x00000010;
pub const CPU_INT_STATUS_ADDRESS: c_uint = 0x00000401;
pub const ERROR_INT_STATUS_ADDRESS: c_uint = 0x00000402;
pub const ERROR_INT_STATUS_WAKEUP_S: c_int = 2;
pub const ERROR_INT_STATUS_WAKEUP: c_uint = 0x00000004;
pub const ERROR_INT_STATUS_RX_UNDERFLOW_S: c_int = 1;
pub const ERROR_INT_STATUS_RX_UNDERFLOW: c_uint = 0x00000002;
pub const ERROR_INT_STATUS_TX_OVERFLOW_S: c_int = 0;
pub const ERROR_INT_STATUS_TX_OVERFLOW: c_uint = 0x00000001;
pub const COUNTER_INT_STATUS_ADDRESS: c_uint = 0x00000403;
pub const COUNTER_INT_STATUS_COUNTER_S: c_int = 0;
pub const COUNTER_INT_STATUS_COUNTER: c_uint = 0x000000ff;
pub const RX_LOOKAHEAD_VALID_ADDRESS: c_uint = 0x00000405;
pub const INT_STATUS_ENABLE_ADDRESS: c_uint = 0x00000418;
pub const INT_STATUS_ENABLE_ERROR_S: c_int = 7;
pub const INT_STATUS_ENABLE_ERROR: c_uint = 0x00000080;
pub const INT_STATUS_ENABLE_CPU_S: c_int = 6;
pub const INT_STATUS_ENABLE_CPU: c_uint = 0x00000040;
pub const INT_STATUS_ENABLE_INT_S: c_int = 5;
pub const INT_STATUS_ENABLE_INT: c_uint = 0x00000020;
pub const INT_STATUS_ENABLE_COUNTER_S: c_int = 4;
pub const INT_STATUS_ENABLE_COUNTER: c_uint = 0x00000010;
pub const INT_STATUS_ENABLE_MBOX_DATA_S: c_int = 0;
pub const INT_STATUS_ENABLE_MBOX_DATA: c_uint = 0x0000000f;
pub const CPU_INT_STATUS_ENABLE_ADDRESS: c_uint = 0x00000419;
pub const CPU_INT_STATUS_ENABLE_BIT_S: c_int = 0;
pub const CPU_INT_STATUS_ENABLE_BIT: c_uint = 0x000000ff;
pub const ERROR_STATUS_ENABLE_ADDRESS: c_uint = 0x0000041a;
pub const ERROR_STATUS_ENABLE_RX_UNDERFLOW_S: c_int = 1;
pub const ERROR_STATUS_ENABLE_RX_UNDERFLOW: c_uint = 0x00000002;
pub const ERROR_STATUS_ENABLE_TX_OVERFLOW_S: c_int = 0;
pub const ERROR_STATUS_ENABLE_TX_OVERFLOW: c_uint = 0x00000001;
pub const COUNTER_INT_STATUS_ENABLE_ADDRESS: c_uint = 0x0000041b;
pub const COUNTER_INT_STATUS_ENABLE_BIT_S: c_int = 0;
pub const COUNTER_INT_STATUS_ENABLE_BIT: c_uint = 0x000000ff;
pub const COUNT_ADDRESS: c_uint = 0x00000420;
pub const COUNT_DEC_ADDRESS: c_uint = 0x00000440;
pub const WINDOW_DATA_ADDRESS: c_uint = 0x00000474;
pub const WINDOW_WRITE_ADDR_ADDRESS: c_uint = 0x00000478;
pub const WINDOW_READ_ADDR_ADDRESS: c_uint = 0x0000047c;
pub const CPU_DBG_SEL_ADDRESS: c_uint = 0x00000483;
pub const CPU_DBG_ADDRESS: c_uint = 0x00000484;
pub const LOCAL_SCRATCH_ADDRESS: c_uint = 0x000000c0;
pub const ATH6KL_OPTION_SLEEP_DISABLE: c_uint = 0x08;
pub const RTC_BASE_ADDRESS: c_uint = 0x00004000;
pub const GPIO_BASE_ADDRESS: c_uint = 0x00014000;
pub const MBOX_BASE_ADDRESS: c_uint = 0x00018000;
pub const ANALOG_INTF_BASE_ADDRESS: c_uint = 0x0001c000;
// real name of the register is unknown

//
// xxx_HOST_INTEREST_ADDRESS is the address in Target RAM of the
// host_interest structure.
//
// Host Interest is shared between Host and Target in order to coordinate
// between the two, and is intended to remain constant (with additions only
// at the end).
//
pub const ATH6KL_AR6003_HI_START_ADDR: c_uint = 0x00540600;
pub const ATH6KL_AR6004_HI_START_ADDR: c_uint = 0x00400800;
//
// These are items that the Host may need to access
// via BMI or via the Diagnostic Window. The position
// of items in this structure must remain constant.
// across firmware revisions!
//
// Types for each item must be fixed size across target and host platforms.
// The structure is used only to calculate offset for each register with
// HI_ITEM() macro, no values are stored to it.
//
// More items may be added at the end.
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
    pub /: *mut *mut u32 hi_unused1; / 0x0c,
//
// General-purpose flag bits, similar to ATH6KL_OPTION_* flags.
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
//
// Flash configuration overrides, used only
// when firmware is not executing from flash.
// (When using flash, modify the global variables
// with equivalent names.)
//
    pub /: *mut *mut u32 hi_bank0_addr_value; / 0x44,
    pub /: *mut *mut u32 hi_bank0_read_value; / 0x48,
    pub /: *mut *mut u32 hi_bank0_write_value; / 0x4c,
    pub /: *mut *mut u32 hi_bank0_config_value; / 0x50,
// Pointer to Board Data
    pub /: *mut *mut u32 hi_board_data; / 0x54,
    pub /: *mut *mut u32 hi_board_data_initialized; / 0x58,
    pub /: *mut *mut u32 hi_dset_ram_index_tbl; / 0x5c,
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
//
// NOTE: byte [0] = tx pin, [1] = rx pin, [2] = rts pin, [3] = cts
// pin
//
    pub /: *mut *mut u32 hi_hci_uart_baud_scale_val; / 0x90,
    pub /: *mut *mut u32 hi_hci_uart_baud_step_val; / 0x94,
    pub /: *mut *mut u32 hi_allocram_start; / 0x98,
    pub /: *mut *mut u32 hi_allocram_sz; / 0x9c,
    pub /: *mut *mut u32 hi_hci_bridge_flags; / 0xa0,
    pub /: *mut *mut u32 hi_hci_uart_support_pins; / 0xa4,
//
// NOTE: byte [0] = RESET pin (bit 7 is polarity),
// bytes[1]..bytes[3] are for future use
//
    pub /: *mut *mut u32 hi_hci_uart_pwr_mgmt_params; / 0xa8,
//
// 0xa8   - [1]: 0 = UART FC active low, 1 = UART FC active high
// [31:16]: wakeup timeout in ms
//
// Pointer to extended board data
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
//
// 0xbc - [31:0]: idle timeout in ms
//
// ACS flags
    pub /: *mut *mut u32 hi_acs_flags; / 0xc0,
    pub /: *mut *mut u32 hi_console_flags; / 0xc4,
    pub /: *mut *mut u32 hi_nvram_state; / 0xc8,
    pub /: *mut *mut u32 hi_option_flag2; / 0xcc,
// If non-zero, override values sent to Host in WMI_READY event.
    pub /: *mut *mut u32 hi_sw_version_override; / 0xd0,
    pub /: *mut *mut u32 hi_abi_version_override; / 0xd4,
//
// Percentage of high priority RX traffic to total expected RX traffic -
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
    pub __packed: },

pub const HI_OPTION_MAC_ADDR_METHOD_SHIFT: c_int = 3;
pub const HI_OPTION_FW_MODE_IBSS: c_uint = 0x0;
pub const HI_OPTION_FW_MODE_BSS_STA: c_uint = 0x1;
pub const HI_OPTION_FW_MODE_AP: c_uint = 0x2;
pub const HI_OPTION_FW_SUBMODE_NONE: c_uint = 0x0;
pub const HI_OPTION_FW_SUBMODE_P2PDEV: c_uint = 0x1;
pub const HI_OPTION_FW_SUBMODE_P2PCLIENT: c_uint = 0x2;
pub const HI_OPTION_FW_SUBMODE_P2PGO: c_uint = 0x3;
pub const HI_OPTION_NUM_DEV_SHIFT: c_uint = 0x9;
pub const HI_OPTION_FW_BRIDGE_SHIFT: c_uint = 0x04;
// Fw Mode/SubMode Mask
//
pub const HI_OPTION_FW_MODE_BITS: c_uint = 0x2;
pub const HI_OPTION_FW_MODE_SHIFT: c_uint = 0xC;
pub const HI_OPTION_FW_SUBMODE_BITS: c_uint = 0x2;
pub const HI_OPTION_FW_SUBMODE_SHIFT: c_uint = 0x14;
// Convert a Target virtual address into a Target physical address

pub const ATH6KL_FWLOG_PAYLOAD_SIZE: c_int = 1500;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath6kl_dbglog_buf {
    pub next: __le32,
    pub buffer_addr: __le32,
    pub bufsize: __le32,
    pub length: __le32,
    pub count: __le32,
    pub free: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath6kl_dbglog_hdr {
    pub dbuf_addr: __le32,
    pub dropped: __le32,
    pub __packed: },
