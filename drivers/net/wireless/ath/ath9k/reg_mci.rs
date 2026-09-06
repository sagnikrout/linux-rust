//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath9k/reg_mci.h
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
// Copyright (c) 2015 Qualcomm Atheros Inc.
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
pub const AR_MCI_COMMAND0: c_uint = 0x1800;
pub const AR_MCI_COMMAND0_HEADER: c_uint = 0xFF;
pub const AR_MCI_COMMAND0_HEADER_S: c_int = 0;
pub const AR_MCI_COMMAND0_LEN: c_uint = 0x1f00;
pub const AR_MCI_COMMAND0_LEN_S: c_int = 8;
pub const AR_MCI_COMMAND0_DISABLE_TIMESTAMP: c_uint = 0x2000;
pub const AR_MCI_COMMAND0_DISABLE_TIMESTAMP_S: c_int = 13;
pub const AR_MCI_COMMAND1: c_uint = 0x1804;
pub const AR_MCI_COMMAND2: c_uint = 0x1808;
pub const AR_MCI_COMMAND2_RESET_TX: c_uint = 0x01;
pub const AR_MCI_COMMAND2_RESET_TX_S: c_int = 0;
pub const AR_MCI_COMMAND2_RESET_RX: c_uint = 0x02;
pub const AR_MCI_COMMAND2_RESET_RX_S: c_int = 1;
pub const AR_MCI_COMMAND2_RESET_RX_NUM_CYCLES: c_uint = 0x3FC;
pub const AR_MCI_COMMAND2_RESET_RX_NUM_CYCLES_S: c_int = 2;
pub const AR_MCI_COMMAND2_RESET_REQ_WAKEUP: c_uint = 0x400;
pub const AR_MCI_COMMAND2_RESET_REQ_WAKEUP_S: c_int = 10;
pub const AR_MCI_RX_CTRL: c_uint = 0x180c;
pub const AR_MCI_TX_CTRL: c_uint = 0x1810;
//
// 0 = no division,
// 1 = divide by 2,
// 2 = divide by 4,
// 3 = divide by 8
//
pub const AR_MCI_TX_CTRL_CLK_DIV: c_uint = 0x03;
pub const AR_MCI_TX_CTRL_CLK_DIV_S: c_int = 0;
pub const AR_MCI_TX_CTRL_DISABLE_LNA_UPDATE: c_uint = 0x04;
pub const AR_MCI_TX_CTRL_DISABLE_LNA_UPDATE_S: c_int = 2;
pub const AR_MCI_TX_CTRL_GAIN_UPDATE_FREQ: c_uint = 0xFFFFF8;
pub const AR_MCI_TX_CTRL_GAIN_UPDATE_FREQ_S: c_int = 3;
pub const AR_MCI_TX_CTRL_GAIN_UPDATE_NUM: c_uint = 0xF000000;
pub const AR_MCI_TX_CTRL_GAIN_UPDATE_NUM_S: c_int = 24;
pub const AR_MCI_MSG_ATTRIBUTES_TABLE: c_uint = 0x1814;
pub const AR_MCI_MSG_ATTRIBUTES_TABLE_CHECKSUM: c_uint = 0xFFFF;
pub const AR_MCI_MSG_ATTRIBUTES_TABLE_CHECKSUM_S: c_int = 0;
pub const AR_MCI_MSG_ATTRIBUTES_TABLE_INVALID_HDR: c_uint = 0xFFFF0000;
pub const AR_MCI_MSG_ATTRIBUTES_TABLE_INVALID_HDR_S: c_int = 16;
pub const AR_MCI_SCHD_TABLE_0: c_uint = 0x1818;
pub const AR_MCI_SCHD_TABLE_1: c_uint = 0x181c;
pub const AR_MCI_GPM_0: c_uint = 0x1820;
pub const AR_MCI_GPM_1: c_uint = 0x1824;
pub const AR_MCI_GPM_WRITE_PTR: c_uint = 0xFFFF0000;
pub const AR_MCI_GPM_WRITE_PTR_S: c_int = 16;
pub const AR_MCI_GPM_BUF_LEN: c_uint = 0x0000FFFF;
pub const AR_MCI_GPM_BUF_LEN_S: c_int = 0;
pub const AR_MCI_INTERRUPT_RAW: c_uint = 0x1828;
pub const AR_MCI_INTERRUPT_EN: c_uint = 0x182c;
pub const AR_MCI_INTERRUPT_SW_MSG_DONE: c_uint = 0x00000001;
pub const AR_MCI_INTERRUPT_SW_MSG_DONE_S: c_int = 0;
pub const AR_MCI_INTERRUPT_CPU_INT_MSG: c_uint = 0x00000002;
pub const AR_MCI_INTERRUPT_CPU_INT_MSG_S: c_int = 1;
pub const AR_MCI_INTERRUPT_RX_CKSUM_FAIL: c_uint = 0x00000004;
pub const AR_MCI_INTERRUPT_RX_CKSUM_FAIL_S: c_int = 2;
pub const AR_MCI_INTERRUPT_RX_INVALID_HDR: c_uint = 0x00000008;
pub const AR_MCI_INTERRUPT_RX_INVALID_HDR_S: c_int = 3;
pub const AR_MCI_INTERRUPT_RX_HW_MSG_FAIL: c_uint = 0x00000010;
pub const AR_MCI_INTERRUPT_RX_HW_MSG_FAIL_S: c_int = 4;
pub const AR_MCI_INTERRUPT_RX_SW_MSG_FAIL: c_uint = 0x00000020;
pub const AR_MCI_INTERRUPT_RX_SW_MSG_FAIL_S: c_int = 5;
pub const AR_MCI_INTERRUPT_TX_HW_MSG_FAIL: c_uint = 0x00000080;
pub const AR_MCI_INTERRUPT_TX_HW_MSG_FAIL_S: c_int = 7;
pub const AR_MCI_INTERRUPT_TX_SW_MSG_FAIL: c_uint = 0x00000100;
pub const AR_MCI_INTERRUPT_TX_SW_MSG_FAIL_S: c_int = 8;
pub const AR_MCI_INTERRUPT_RX_MSG: c_uint = 0x00000200;
pub const AR_MCI_INTERRUPT_RX_MSG_S: c_int = 9;
pub const AR_MCI_INTERRUPT_REMOTE_SLEEP_UPDATE: c_uint = 0x00000400;
pub const AR_MCI_INTERRUPT_REMOTE_SLEEP_UPDATE_S: c_int = 10;
pub const AR_MCI_INTERRUPT_BT_PRI: c_uint = 0x07fff800;
pub const AR_MCI_INTERRUPT_BT_PRI_S: c_int = 11;
pub const AR_MCI_INTERRUPT_BT_PRI_THRESH: c_uint = 0x08000000;
pub const AR_MCI_INTERRUPT_BT_PRI_THRESH_S: c_int = 27;
pub const AR_MCI_INTERRUPT_BT_FREQ: c_uint = 0x10000000;
pub const AR_MCI_INTERRUPT_BT_FREQ_S: c_int = 28;
pub const AR_MCI_INTERRUPT_BT_STOMP: c_uint = 0x20000000;
pub const AR_MCI_INTERRUPT_BT_STOMP_S: c_int = 29;
pub const AR_MCI_INTERRUPT_BB_AIC_IRQ: c_uint = 0x40000000;
pub const AR_MCI_INTERRUPT_BB_AIC_IRQ_S: c_int = 30;
pub const AR_MCI_INTERRUPT_CONT_INFO_TIMEOUT: c_uint = 0x80000000;
pub const AR_MCI_INTERRUPT_CONT_INFO_TIMEOUT_S: c_int = 31;
pub const AR_MCI_REMOTE_CPU_INT: c_uint = 0x1830;
pub const AR_MCI_REMOTE_CPU_INT_EN: c_uint = 0x1834;
pub const AR_MCI_INTERRUPT_RX_MSG_RAW: c_uint = 0x1838;
pub const AR_MCI_INTERRUPT_RX_MSG_EN: c_uint = 0x183c;
pub const AR_MCI_INTERRUPT_RX_MSG_REMOTE_RESET: c_uint = 0x00000001;
pub const AR_MCI_INTERRUPT_RX_MSG_REMOTE_RESET_S: c_int = 0;
pub const AR_MCI_INTERRUPT_RX_MSG_LNA_CONTROL: c_uint = 0x00000002;
pub const AR_MCI_INTERRUPT_RX_MSG_LNA_CONTROL_S: c_int = 1;
pub const AR_MCI_INTERRUPT_RX_MSG_CONT_NACK: c_uint = 0x00000004;
pub const AR_MCI_INTERRUPT_RX_MSG_CONT_NACK_S: c_int = 2;
pub const AR_MCI_INTERRUPT_RX_MSG_CONT_INFO: c_uint = 0x00000008;
pub const AR_MCI_INTERRUPT_RX_MSG_CONT_INFO_S: c_int = 3;
pub const AR_MCI_INTERRUPT_RX_MSG_CONT_RST: c_uint = 0x00000010;
pub const AR_MCI_INTERRUPT_RX_MSG_CONT_RST_S: c_int = 4;
pub const AR_MCI_INTERRUPT_RX_MSG_SCHD_INFO: c_uint = 0x00000020;
pub const AR_MCI_INTERRUPT_RX_MSG_SCHD_INFO_S: c_int = 5;
pub const AR_MCI_INTERRUPT_RX_MSG_CPU_INT: c_uint = 0x00000040;
pub const AR_MCI_INTERRUPT_RX_MSG_CPU_INT_S: c_int = 6;
pub const AR_MCI_INTERRUPT_RX_MSG_GPM: c_uint = 0x00000100;
pub const AR_MCI_INTERRUPT_RX_MSG_GPM_S: c_int = 8;
pub const AR_MCI_INTERRUPT_RX_MSG_LNA_INFO: c_uint = 0x00000200;
pub const AR_MCI_INTERRUPT_RX_MSG_LNA_INFO_S: c_int = 9;
pub const AR_MCI_INTERRUPT_RX_MSG_SYS_SLEEPING: c_uint = 0x00000400;
pub const AR_MCI_INTERRUPT_RX_MSG_SYS_SLEEPING_S: c_int = 10;
pub const AR_MCI_INTERRUPT_RX_MSG_SYS_WAKING: c_uint = 0x00000800;
pub const AR_MCI_INTERRUPT_RX_MSG_SYS_WAKING_S: c_int = 11;
pub const AR_MCI_INTERRUPT_RX_MSG_REQ_WAKE: c_uint = 0x00001000;
pub const AR_MCI_INTERRUPT_RX_MSG_REQ_WAKE_S: c_int = 12;
pub const AR_MCI_CPU_INT: c_uint = 0x1840;
pub const AR_MCI_RX_STATUS: c_uint = 0x1844;
pub const AR_MCI_RX_LAST_SCHD_MSG_INDEX: c_uint = 0x00000F00;
pub const AR_MCI_RX_LAST_SCHD_MSG_INDEX_S: c_int = 8;
pub const AR_MCI_RX_REMOTE_SLEEP: c_uint = 0x00001000;
pub const AR_MCI_RX_REMOTE_SLEEP_S: c_int = 12;
pub const AR_MCI_RX_MCI_CLK_REQ: c_uint = 0x00002000;
pub const AR_MCI_RX_MCI_CLK_REQ_S: c_int = 13;
pub const AR_MCI_CONT_STATUS: c_uint = 0x1848;
pub const AR_MCI_CONT_RSSI_POWER: c_uint = 0x000000FF;
pub const AR_MCI_CONT_RSSI_POWER_S: c_int = 0;
pub const AR_MCI_CONT_PRIORITY: c_uint = 0x0000FF00;
pub const AR_MCI_CONT_PRIORITY_S: c_int = 8;
pub const AR_MCI_CONT_TXRX: c_uint = 0x00010000;
pub const AR_MCI_CONT_TXRX_S: c_int = 16;
pub const AR_MCI_BT_PRI0: c_uint = 0x184c;
pub const AR_MCI_BT_PRI1: c_uint = 0x1850;
pub const AR_MCI_BT_PRI2: c_uint = 0x1854;
pub const AR_MCI_BT_PRI3: c_uint = 0x1858;
pub const AR_MCI_BT_PRI: c_uint = 0x185c;
pub const AR_MCI_WL_FREQ0: c_uint = 0x1860;
pub const AR_MCI_WL_FREQ1: c_uint = 0x1864;
pub const AR_MCI_WL_FREQ2: c_uint = 0x1868;
pub const AR_MCI_GAIN: c_uint = 0x186c;
pub const AR_MCI_WBTIMER1: c_uint = 0x1870;
pub const AR_MCI_WBTIMER2: c_uint = 0x1874;
pub const AR_MCI_WBTIMER3: c_uint = 0x1878;
pub const AR_MCI_WBTIMER4: c_uint = 0x187c;
pub const AR_MCI_MAXGAIN: c_uint = 0x1880;
pub const AR_MCI_HW_SCHD_TBL_CTL: c_uint = 0x1884;
pub const AR_MCI_HW_SCHD_TBL_D0: c_uint = 0x1888;
pub const AR_MCI_HW_SCHD_TBL_D1: c_uint = 0x188c;
pub const AR_MCI_HW_SCHD_TBL_D2: c_uint = 0x1890;
pub const AR_MCI_HW_SCHD_TBL_D3: c_uint = 0x1894;
pub const AR_MCI_TX_PAYLOAD0: c_uint = 0x1898;
pub const AR_MCI_TX_PAYLOAD1: c_uint = 0x189c;
pub const AR_MCI_TX_PAYLOAD2: c_uint = 0x18a0;
pub const AR_MCI_TX_PAYLOAD3: c_uint = 0x18a4;
pub const AR_BTCOEX_WBTIMER: c_uint = 0x18a8;
pub const AR_BTCOEX_CTRL: c_uint = 0x18ac;
pub const AR_BTCOEX_CTRL_AR9462_MODE: c_uint = 0x00000001;
pub const AR_BTCOEX_CTRL_AR9462_MODE_S: c_int = 0;
pub const AR_BTCOEX_CTRL_WBTIMER_EN: c_uint = 0x00000002;
pub const AR_BTCOEX_CTRL_WBTIMER_EN_S: c_int = 1;
pub const AR_BTCOEX_CTRL_MCI_MODE_EN: c_uint = 0x00000004;
pub const AR_BTCOEX_CTRL_MCI_MODE_EN_S: c_int = 2;
pub const AR_BTCOEX_CTRL_LNA_SHARED: c_uint = 0x00000008;
pub const AR_BTCOEX_CTRL_LNA_SHARED_S: c_int = 3;
pub const AR_BTCOEX_CTRL_PA_SHARED: c_uint = 0x00000010;
pub const AR_BTCOEX_CTRL_PA_SHARED_S: c_int = 4;
pub const AR_BTCOEX_CTRL_ONE_STEP_LOOK_AHEAD_EN: c_uint = 0x00000020;
pub const AR_BTCOEX_CTRL_ONE_STEP_LOOK_AHEAD_EN_S: c_int = 5;
pub const AR_BTCOEX_CTRL_TIME_TO_NEXT_BT_THRESH_EN: c_uint = 0x00000040;
pub const AR_BTCOEX_CTRL_TIME_TO_NEXT_BT_THRESH_EN_S: c_int = 6;
pub const AR_BTCOEX_CTRL_NUM_ANTENNAS: c_uint = 0x00000180;
pub const AR_BTCOEX_CTRL_NUM_ANTENNAS_S: c_int = 7;
pub const AR_BTCOEX_CTRL_RX_CHAIN_MASK: c_uint = 0x00000E00;
pub const AR_BTCOEX_CTRL_RX_CHAIN_MASK_S: c_int = 9;
pub const AR_BTCOEX_CTRL_AGGR_THRESH: c_uint = 0x00007000;
pub const AR_BTCOEX_CTRL_AGGR_THRESH_S: c_int = 12;
pub const AR_BTCOEX_CTRL_1_CHAIN_BCN: c_uint = 0x00080000;
pub const AR_BTCOEX_CTRL_1_CHAIN_BCN_S: c_int = 19;
pub const AR_BTCOEX_CTRL_1_CHAIN_ACK: c_uint = 0x00100000;
pub const AR_BTCOEX_CTRL_1_CHAIN_ACK_S: c_int = 20;
pub const AR_BTCOEX_CTRL_WAIT_BA_MARGIN: c_uint = 0x1FE00000;
pub const AR_BTCOEX_CTRL_WAIT_BA_MARGIN_S: c_int = 28;
pub const AR_BTCOEX_CTRL_REDUCE_TXPWR: c_uint = 0x20000000;
pub const AR_BTCOEX_CTRL_REDUCE_TXPWR_S: c_int = 29;
pub const AR_BTCOEX_CTRL_SPDT_ENABLE_10: c_uint = 0x40000000;
pub const AR_BTCOEX_CTRL_SPDT_ENABLE_10_S: c_int = 30;
pub const AR_BTCOEX_CTRL_SPDT_POLARITY: c_uint = 0x80000000;
pub const AR_BTCOEX_CTRL_SPDT_POLARITY_S: c_int = 31;
pub const AR_BTCOEX_WL_WEIGHTS0: c_uint = 0x18b0;
pub const AR_BTCOEX_WL_WEIGHTS1: c_uint = 0x18b4;
pub const AR_BTCOEX_WL_WEIGHTS2: c_uint = 0x18b8;
pub const AR_BTCOEX_WL_WEIGHTS3: c_uint = 0x18bc;

pub const AR_BTCOEX_WL_LNA: c_uint = 0x1940;
pub const AR_BTCOEX_RFGAIN_CTRL: c_uint = 0x1944;
pub const AR_BTCOEX_WL_LNA_TIMEOUT: c_uint = 0x003FFFFF;
pub const AR_BTCOEX_WL_LNA_TIMEOUT_S: c_int = 0;
pub const AR_BTCOEX_CTRL2: c_uint = 0x1948;
pub const AR_BTCOEX_CTRL2_TXPWR_THRESH: c_uint = 0x0007F800;
pub const AR_BTCOEX_CTRL2_TXPWR_THRESH_S: c_int = 11;
pub const AR_BTCOEX_CTRL2_TX_CHAIN_MASK: c_uint = 0x00380000;
pub const AR_BTCOEX_CTRL2_TX_CHAIN_MASK_S: c_int = 19;
pub const AR_BTCOEX_CTRL2_RX_DEWEIGHT: c_uint = 0x00400000;
pub const AR_BTCOEX_CTRL2_RX_DEWEIGHT_S: c_int = 22;
pub const AR_BTCOEX_CTRL2_GPIO_OBS_SEL: c_uint = 0x00800000;
pub const AR_BTCOEX_CTRL2_GPIO_OBS_SEL_S: c_int = 23;
pub const AR_BTCOEX_CTRL2_MAC_BB_OBS_SEL: c_uint = 0x01000000;
pub const AR_BTCOEX_CTRL2_MAC_BB_OBS_SEL_S: c_int = 24;
pub const AR_BTCOEX_CTRL2_DESC_BASED_TXPWR_ENABLE: c_uint = 0x02000000;
pub const AR_BTCOEX_CTRL2_DESC_BASED_TXPWR_ENABLE_S: c_int = 25;
pub const AR_BTCOEX_CTRL_SPDT_ENABLE: c_uint = 0x00000001;
pub const AR_BTCOEX_CTRL_SPDT_ENABLE_S: c_int = 0;
pub const AR_BTCOEX_CTRL_BT_OWN_SPDT_CTRL: c_uint = 0x00000002;
pub const AR_BTCOEX_CTRL_BT_OWN_SPDT_CTRL_S: c_int = 1;
pub const AR_BTCOEX_CTRL_USE_LATCHED_BT_ANT: c_uint = 0x00000004;
pub const AR_BTCOEX_CTRL_USE_LATCHED_BT_ANT_S: c_int = 2;
pub const AR_GLB_WLAN_UART_INTF_EN: c_uint = 0x00020000;
pub const AR_GLB_WLAN_UART_INTF_EN_S: c_int = 17;
pub const AR_GLB_DS_JTAG_DISABLE: c_uint = 0x00040000;
pub const AR_GLB_DS_JTAG_DISABLE_S: c_int = 18;
pub const AR_BTCOEX_RC: c_uint = 0x194c;

pub const AR_BTCOEX_DBG: c_uint = 0x1a50;
pub const AR_MCI_LAST_HW_MSG_HDR: c_uint = 0x1a54;
pub const AR_MCI_LAST_HW_MSG_BDY: c_uint = 0x1a58;
pub const AR_MCI_SCHD_TABLE_2: c_uint = 0x1a5c;
pub const AR_MCI_SCHD_TABLE_2_MEM_BASED: c_uint = 0x00000001;
pub const AR_MCI_SCHD_TABLE_2_MEM_BASED_S: c_int = 0;
pub const AR_MCI_SCHD_TABLE_2_HW_BASED: c_uint = 0x00000002;
pub const AR_MCI_SCHD_TABLE_2_HW_BASED_S: c_int = 1;
pub const AR_BTCOEX_CTRL3: c_uint = 0x1a60;
pub const AR_BTCOEX_CTRL3_CONT_INFO_TIMEOUT: c_uint = 0x00000fff;
pub const AR_BTCOEX_CTRL3_CONT_INFO_TIMEOUT_S: c_int = 0;
pub const AR_GLB_SWREG_DISCONT_MODE: c_uint = 0x2002c;
pub const AR_GLB_SWREG_DISCONT_EN_BT_WLAN: c_uint = 0x3;
pub const AR_MCI_MISC: c_uint = 0x1a74;
pub const AR_MCI_MISC_HW_FIX_EN: c_uint = 0x00000001;
pub const AR_MCI_MISC_HW_FIX_EN_S: c_int = 0;
pub const AR_MCI_DBG_CNT_CTRL: c_uint = 0x1a78;
pub const AR_MCI_DBG_CNT_CTRL_ENABLE: c_uint = 0x00000001;
pub const AR_MCI_DBG_CNT_CTRL_ENABLE_S: c_int = 0;
pub const AR_MCI_DBG_CNT_CTRL_BT_LINKID: c_uint = 0x000007f8;
pub const AR_MCI_DBG_CNT_CTRL_BT_LINKID_S: c_int = 3;
pub const MCI_STAT_ALL_BT_LINKID: c_uint = 0xffff;

