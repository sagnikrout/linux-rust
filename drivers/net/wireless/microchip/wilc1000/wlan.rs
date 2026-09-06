//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/microchip/wilc1000/wlan.h
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
// Copyright (c) 2012 - 2018 Microchip Technology Inc., and its subsidiaries.
// All rights reserved.
//

//
// Mac eth header length
//

pub const SUB_MSDU_HEADER_LENGTH: c_int = 14;
pub const SNAP_HDR_LEN: c_int = 8;
pub const ETHERNET_HDR_LEN: c_int = 14;
pub const WORD_ALIGNMENT_PAD: c_int = 0;

pub const HOST_HDR_OFFSET: c_int = 4;
pub const ETHERNET_HDR_LEN: c_int = 14;
pub const IP_HDR_LEN: c_int = 20;

pub const UDP_HDR_LEN: c_int = 8;

//
// Register Defines
//
pub const WILC_PERIPH_REG_BASE: c_uint = 0x1000;
pub const WILC_CHANGING_VIR_IF: c_uint = 0x108c;

pub const WILC_RF_REVISION_ID: c_uint = 0x13f4;
pub const WILC_VMM_TBL_SIZE: c_int = 64;
pub const WILC_VMM_TX_TBL_BASE: c_uint = 0x150400;
pub const WILC_VMM_RX_TBL_BASE: c_uint = 0x150500;
pub const WILC_VMM_BASE: c_uint = 0x150000;

pub const WILC_SPI_REG_BASE: c_uint = 0xe800;

pub const WILC_SPI_WAKEUP_REG: c_uint = 0x1;

// WILC1000 specific
pub const WILC1000_SPI_CLK_STATUS_REG: c_uint = 0x0f;

// WILC3000 specific
pub const WILC3000_SPI_CLK_STATUS_REG: c_uint = 0x13;

pub const WILC_SPI_HOST_TO_FW_REG: c_uint = 0x0b;

pub const WILC_SPI_FW_TO_HOST_REG: c_uint = 0x10;

pub const WILC_SPI_CLOCKLESS_ADDR_LIMIT: c_uint = 0x30;
// Functions IO enables bits

// Function/Interrupt enables bits

// Abort CCCR register bits

// Vendor specific CCCR registers
pub const WILC_SDIO_WAKEUP_REG: c_uint = 0xf0;

// WILC1000
pub const WILC1000_SDIO_CLK_STATUS_REG: c_uint = 0xf1;

pub const WILC1000_SDIO_IRQ_FLAG_REG: c_uint = 0xf7;
pub const WILC1000_SDIO_IRQ_CLEAR_FLAG_REG: c_uint = 0xf8;
// WILC3000 specific
pub const WILC3000_SDIO_CLK_STATUS_REG: c_uint = 0xf0 /* clk & wakeup are on same reg */;

pub const WILC3000_SDIO_VMM_TBL_CTRL_REG: c_uint = 0xf1;
pub const WILC3000_SDIO_IRQ_FLAG_REG: c_uint = 0xfe;
// Common vendor specific CCCR register
pub const WILC_SDIO_INTERRUPT_DATA_SZ_REG: c_uint = 0xf2 /* Read size (2 bytes) */;
pub const WILC_SDIO_VMM_TBL_CTRL_REG: c_uint = 0xf6;
pub const WILC_SDIO_HOST_TO_FW_REG: c_uint = 0xfa;

pub const WILC_SDIO_FW_TO_HOST_REG: c_uint = 0xfc;

// Function 1 specific FBR register
pub const WILC_SDIO_FBR_CSA_REG: c_uint = 0x10C /* CSA pointer (3 bytes) */;
pub const WILC_SDIO_FBR_DATA_REG: c_uint = 0x10F;
pub const WILC_SDIO_F1_DATA_REG: c_uint = 0x0;
pub const WILC_SDIO_EXT_IRQ_FLAG_REG: c_uint = 0x4;
pub const WILC_AHB_DATA_MEM_BASE: c_uint = 0x30000;
pub const WILC_AHB_SHARE_MEM_BASE: c_uint = 0xd0000;

pub const WILC_VMM_TBL_RX_SHADOW_SIZE: c_int = 256;
pub const WILC_FW_HOST_COMM: c_uint = 0x13c0;
pub const WILC_GP_REG_0: c_uint = 0x149c;
pub const WILC_GP_REG_1: c_uint = 0x14a0;
pub const GLOBAL_MODE_CONTROL: c_uint = 0x1614;
pub const PWR_SEQ_MISC_CTRL: c_uint = 0x3008;

pub const WILC_CORTUS_INTERRUPT_BASE: c_uint = 0x10A8;

// tx control register 1 to 4 for RX
pub const WILC_REG_4_TO_1_RX: c_uint = 0x1e1c;
// tx control register 1 to 4 for TX Bank_0
pub const WILC_REG_4_TO_1_TX_BANK0: c_uint = 0x1e9c;
pub const WILC_CORTUS_RESET_MUX_SEL: c_uint = 0x1118;
pub const WILC_CORTUS_BOOT_REGISTER: c_uint = 0xc0000;
pub const WILC3000_BOOTROM_STATUS: c_uint = 0x207ac;
pub const WILC3000_CORTUS_BOOT_REGISTER_2: c_uint = 0x4f0000;
pub const WILC3000_CHIP_ID: c_uint = 0x3b0000;
pub const WILC_CORTUS_BOOT_FROM_IRAM: c_uint = 0x71;
pub const WILC_1000_BASE_ID: c_uint = 0x100000;
pub const WILC_1000_BASE_ID_2A: c_uint = 0x1002A0;

pub const WILC_1000_BASE_ID_2B: c_uint = 0x1002B0;

pub const WILC_3000_BASE_ID: c_uint = 0x300000;

//
// Wlan Defines
//
pub const WILC_CFG_PKT: c_int = 1;
pub const WILC_NET_PKT: c_int = 0;
pub const WILC_MGMT_PKT: c_int = 2;
pub const WILC_CFG_SET: c_int = 1;
pub const WILC_CFG_QUERY: c_int = 0;
pub const WILC_CFG_RSP: c_int = 1;
pub const WILC_CFG_RSP_STATUS: c_int = 2;
pub const WILC_CFG_RSP_SCAN: c_int = 3;

pub const NQUEUES: c_int = 4;
pub const AC_BUFFER_SIZE: c_int = 1000;

//
// E0 and later Interrupt flags.
//
// E0 and later Interrupt flags.
// IRQ Status word
// 15:0 = DMA count in words.
// 16: INT0 flag
// 17: INT1 flag
// 18: INT2 flag
// 19: INT3 flag
// 20: INT4 flag
// 21: INT5 flag
//
pub const IRG_FLAGS_OFFSET: c_int = 16;

pub const MAX_NUM_INT: c_int = 5;

//
// E0 and later Interrupt flags.
// IRQ Clear word
// 0: Clear INT0
// 1: Clear INT1
// 2: Clear INT2
// 3: Clear INT3
// 4: Clear INT4
// 5: Clear INT5
// 6: Select VMM table 1
// 7: Select VMM table 2
// 8: Enable VMM
//

pub const NUM_INT_EXT: c_int = 1;

// time for expiring the completion of cfg packets

pub const IS_MANAGMEMENT: c_uint = 0x100;
pub const IS_MANAGMEMENT_CALLBACK: c_uint = 0x080;
pub const IS_MGMT_STATUS_SUCCES: c_uint = 0x040;
pub const IS_MGMT_AUTH_PKT: c_uint = 0x010;

pub const WILC_VMM_ENTRY_FULL_RETRY: c_int = 1;
//
// Tx/Rx Queue Structure
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ip_pkt_priority {
    AC_VO_Q = 0,
    AC_VI_Q = 1,
    AC_BE_Q = 2,
    AC_BK_Q = 3
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct txq_entry_t {
    pub list: list_head,
    pub type: c_int,
    pub q_num: u8,
    pub ack_idx: c_int,
    pub buffer: *mut u8,
    pub buffer_size: c_int,
    pub priv: *mut c_void,
    pub status: c_int,
    pub vif: *mut wilc_vif,
    pub status): *mut *mut *mut void (tx_complete_func)(void priv, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct txq_fw_recv_queue_stat {
    pub acm: u8,
    pub count: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct txq_handle {
    pub txq_head: txq_entry_t,
    pub count: u16,
    pub fw: txq_fw_recv_queue_stat,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxq_entry_t {
    pub list: list_head,
    pub buffer: *mut u8,
    pub buffer_size: c_int,
}

//
// Host IF Structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilc_hif_func {
    pub resume): *mut *mut *mut int (hif_init)(struct wilc wilc, bool,
    pub wilc): *mut *mut int (hif_deinit)(struct wilc,
    pub data): *mut *mut *mut int (hif_read_reg)(struct wilc wilc, u32 addr, u32,
    pub data): *mut *mut *mut int (hif_write_reg)(struct wilc wilc, u32 addr, u32,
    pub size): *mut *mut *mut *mut int (hif_block_rx)(struct wilc wilc, u32 addr, u8 buf, u32,
    pub size): *mut *mut *mut *mut int (hif_block_tx)(struct wilc wilc, u32 addr, u8 buf, u32,
    pub int_status): *mut *mut *mut int (hif_read_int)(struct wilc wilc, u32,
    pub val): *mut *mut *mut int (hif_clear_int_ext)(struct wilc wilc, u32,
    pub size): *mut *mut *mut int (hif_read_size)(struct wilc wilc, u32,
    pub size): *mut *mut *mut *mut int (hif_block_tx_ext)(struct wilc wilc, u32 addr, u8 buf, u32,
    pub size): *mut *mut *mut *mut int (hif_block_rx_ext)(struct wilc wilc, u32 addr, u8 buf, u32,
    pub nint): *mut *mut *mut int (hif_sync_ext)(struct wilc wilc, int,
    pub nic): *mut *mut int (enable_interrupt)(struct wilc,
    pub nic): *mut *mut void (disable_interrupt)(struct wilc,
    pub wilc): *mut *mut int (hif_reset)(struct wilc,
    pub wilc): *mut *mut bool (hif_is_init)(struct wilc,
}

pub const WILC_MAX_CFG_FRAME_SIZE: c_int = 1468;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_complete_data {
    pub size: c_int,
    pub buff: *mut c_void,
    pub skb: *mut sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilc_cfg_cmd_hdr {
    pub cmd_type: u8,
    pub seq_no: u8,
    pub total_len: __le16,
    pub driver_handler: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilc_cfg_frame {
    pub hdr: wilc_cfg_cmd_hdr,
    pub frame: [u8; WILC_MAX_CFG_FRAME_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilc_cfg_rsp {
    pub type: u8,
    pub seq_no: u8,
}

extern "C" {
    pub fn wilc_wlan_start(wilc: *mut wilc) -> c_int;
}
extern "C" {
    pub fn wilc_wlan_stop(wilc: *mut wilc, vif: *mut wilc_vif) -> c_int;
}
extern "C" {
    pub fn wilc_wlan_handle_txq(wl: *mut wilc, txq_count: *mut u32) -> c_int;
}
extern "C" {
    pub fn wilc_handle_isr(wilc: *mut wilc);
}
extern "C" {
    pub fn wilc_wlan_cleanup(dev: *mut net_device);
}
extern "C" {
    pub fn wilc_enable_tcp_ack_filter(vif: *mut wilc_vif, value: bool);
}
extern "C" {
    pub fn wilc_wlan_get_num_conn_ifcs(wilc: *mut wilc) -> c_int;
}
extern "C" {
    pub fn wilc_mac_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t;
}
extern "C" {
    pub fn wilc_wfi_p2p_rx(vif: *mut wilc_vif, buff: *mut u8, size: u32);
}
extern "C" {
    pub fn wilc_wfi_mgmt_frame_rx(vif: *mut wilc_vif, buff: *mut u8, size: u32) -> bool;
}
extern "C" {
    pub fn host_wakeup_notify(wilc: *mut wilc) -> c_int;
}
extern "C" {
    pub fn host_sleep_notify(wilc: *mut wilc) -> c_int;
}
extern "C" {
    pub fn wilc_wlan_init(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn wilc_get_chipid(wilc: *mut wilc) -> c_int;
}
extern "C" {
    pub fn wilc_load_mac_from_nv(wilc: *mut wilc) -> c_int;
}
