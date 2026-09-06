//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/marvell/mwifiex/sdio.h
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
// NXP Wireless LAN device driver: SDIO specific definitions
//
// Copyright 2011-2020 NXP
//

pub const BLOCK_MODE: c_int = 1;
pub const BYTE_MODE: c_int = 0;
pub const MWIFIEX_SDIO_IO_PORT_MASK: c_uint = 0xfffff;
pub const MWIFIEX_SDIO_BYTE_MODE_MASK: c_uint = 0x80000000;
pub const MWIFIEX_MAX_FUNC2_REG_NUM: c_int = 13;
pub const MWIFIEX_SDIO_SCRATCH_SIZE: c_int = 10;
pub const SDIO_MPA_ADDR_BASE: c_uint = 0x1000;
pub const CTRL_PORT: c_int = 0;
pub const CTRL_PORT_MASK: c_uint = 0x0001;

pub const REG_PORT: c_int = 0;
pub const MEM_PORT: c_uint = 0x10000;

pub const CMD_PORT_SLCT: c_uint = 0x8000;

// we leave one block of 256 bytes for DMA alignment

// Misc. Config Register : Auto Re-enable interrupts

// Host Control Registers : Configuration
pub const CONFIGURATION_REG: c_uint = 0x00;
// Host Control Registers : Host power up

// Host Control Registers : Upload host interrupt mask

// Host Control Registers : Download host interrupt mask

// Host Control Registers : Upload host interrupt status

// Host Control Registers : Download host interrupt status

// Host Control Registers : Host interrupt status
pub const CARD_INT_STATUS_REG: c_uint = 0x28;
// Card Control Registers : Card I/O ready

// Card Control Registers : Download card ready

// Max retry number of CMD53 write
pub const MAX_WRITE_IOMEM_RETRY: c_int = 2;
// SDIO Tx aggregation in progress ?

// SDIO Tx aggregation buffer room for next packet ?

// Copy current packet (SDIO Tx aggregation buffer) to SDIO buffer

// SDIO Tx aggregation limit ?

// Reset SDIO Tx aggregation buffer parameters

// SDIO Rx aggregation limit ?

// SDIO Rx aggregation in progress ?

// SDIO Rx aggregation buffer room for next packet ?

// Reset SDIO Rx aggregation buffer parameters

// data structure for SDIO MPA TX
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_sdio_mpa_tx {
// multiport tx aggregation buffer pointer
    pub buf: *mut u8,
    pub buf_len: u32,
    pub pkt_cnt: u32,
    pub ports: u32,
    pub start_port: u16,
    pub enabled: u8,
    pub buf_size: u32,
    pub pkt_aggr_limit: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_sdio_mpa_rx {
    pub buf: *mut u8,
    pub buf_len: u32,
    pub pkt_cnt: u32,
    pub ports: u32,
    pub start_port: u16,
    pub len_arr: *mut u32,
    pub enabled: u8,
    pub buf_size: u32,
    pub pkt_aggr_limit: u32,
}

extern "C" {
    pub fn mwifiex_bus_register() -> c_int;
}
extern "C" {
    pub fn mwifiex_bus_unregister();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_sdio_card_reg {
    pub start_rd_port: u8,
    pub start_wr_port: u8,
    pub base_0_reg: u8,
    pub base_1_reg: u8,
    pub poll_reg: u8,
    pub host_int_enable: u8,
    pub host_int_rsr_reg: u8,
    pub host_int_status_reg: u8,
    pub host_int_mask_reg: u8,
    pub host_strap_reg: u8,
    pub host_strap_mask: u8,
    pub host_strap_value: u8,
    pub status_reg_0: u8,
    pub status_reg_1: u8,
    pub sdio_int_mask: u8,
    pub data_port_mask: u32,
    pub io_port_0_reg: u8,
    pub io_port_1_reg: u8,
    pub io_port_2_reg: u8,
    pub max_mp_regs: u8,
    pub rd_bitmap_l: u8,
    pub rd_bitmap_u: u8,
    pub rd_bitmap_1l: u8,
    pub rd_bitmap_1u: u8,
    pub wr_bitmap_l: u8,
    pub wr_bitmap_u: u8,
    pub wr_bitmap_1l: u8,
    pub wr_bitmap_1u: u8,
    pub rd_len_p0_l: u8,
    pub rd_len_p0_u: u8,
    pub card_misc_cfg_reg: u8,
    pub card_cfg_2_1_reg: u8,
    pub cmd_rd_len_0: u8,
    pub cmd_rd_len_1: u8,
    pub cmd_rd_len_2: u8,
    pub cmd_rd_len_3: u8,
    pub cmd_cfg_0: u8,
    pub cmd_cfg_1: u8,
    pub cmd_cfg_2: u8,
    pub cmd_cfg_3: u8,
    pub fw_dump_host_ready: u8,
    pub fw_dump_ctrl: u8,
    pub fw_dump_start: u8,
    pub fw_dump_end: u8,
    pub func1_dump_reg_start: u8,
    pub func1_dump_reg_end: u8,
    pub func1_scratch_reg: u8,
    pub func1_spec_reg_num: u8,
    pub func1_spec_reg_table: [u8; MWIFIEX_MAX_FUNC2_REG_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdio_mmc_card {
    pub func: *mut sdio_func,
    pub adapter: *mut mwifiex_adapter,
    pub fw_done: completion,
    pub firmware: *const c_char,
    pub firmware_sdiouart: *const c_char,
    pub reg: *const mwifiex_sdio_card_reg,
    pub max_ports: u8,
    pub mp_agg_pkt_limit: u8,
    pub tx_buf_size: u16,
    pub mp_tx_agg_buf_size: u32,
    pub mp_rx_agg_buf_size: u32,
    pub mp_rd_bitmap: u32,
    pub mp_wr_bitmap: u32,
    pub mp_end_port: u16,
    pub mp_data_port_mask: u32,
    pub curr_rd_port: u8,
    pub curr_wr_port: u8,
    pub mp_regs: *mut u8,
    pub supports_sdio_new_mode: bool,
    pub has_control_mask: bool,
    pub can_dump_fw: bool,
    pub fw_dump_enh: bool,
    pub can_auto_tdls: bool,
    pub can_ext_scan: bool,
    pub fw_ready_extra_delay: bool,
    pub host_mlme: bool,
    pub mpa_tx: mwifiex_sdio_mpa_tx,
    pub mpa_rx: mwifiex_sdio_mpa_rx,
    pub work: work_struct,
    pub work_flags: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_sdio_device {
    pub firmware: *const c_char,
    pub firmware_sdiouart: *const c_char,
    pub reg: *const mwifiex_sdio_card_reg,
    pub max_ports: u8,
    pub mp_agg_pkt_limit: u8,
    pub tx_buf_size: u16,
    pub mp_tx_agg_buf_size: u32,
    pub mp_rx_agg_buf_size: u32,
    pub supports_sdio_new_mode: bool,
    pub has_control_mask: bool,
    pub can_dump_fw: bool,
    pub fw_dump_enh: bool,
    pub can_auto_tdls: bool,
    pub can_ext_scan: bool,
    pub fw_ready_extra_delay: bool,
    pub host_mlme: bool,
}

//
// .cmdrsp_complete handler
//
// .event_complete handler
//
// Prepare to copy current packet from card to SDIO Rx aggregation buffer
