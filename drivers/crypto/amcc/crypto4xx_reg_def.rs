//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/amcc/crypto4xx_reg_def.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// AMCC SoC PPC4xx Crypto Driver
//
// Copyright (c) 2008 Applied Micro Circuits Corporation.
// All rights reserved. James Hsiao <jhsiao@amcc.com>
//
// This filr defines the register set for Security Subsystem
//
// CRYPTO4XX Register offset
pub const CRYPTO4XX_DESCRIPTOR: c_uint = 0x00000000;
pub const CRYPTO4XX_CTRL_STAT: c_uint = 0x00000000;
pub const CRYPTO4XX_SOURCE: c_uint = 0x00000004;
pub const CRYPTO4XX_DEST: c_uint = 0x00000008;
pub const CRYPTO4XX_SA: c_uint = 0x0000000C;
pub const CRYPTO4XX_SA_LENGTH: c_uint = 0x00000010;
pub const CRYPTO4XX_LENGTH: c_uint = 0x00000014;
pub const CRYPTO4XX_PE_DMA_CFG: c_uint = 0x00000040;
pub const CRYPTO4XX_PE_DMA_STAT: c_uint = 0x00000044;
pub const CRYPTO4XX_PDR_BASE: c_uint = 0x00000048;
pub const CRYPTO4XX_RDR_BASE: c_uint = 0x0000004c;
pub const CRYPTO4XX_RING_SIZE: c_uint = 0x00000050;
pub const CRYPTO4XX_RING_CTRL: c_uint = 0x00000054;
pub const CRYPTO4XX_INT_RING_STAT: c_uint = 0x00000058;
pub const CRYPTO4XX_EXT_RING_STAT: c_uint = 0x0000005c;
pub const CRYPTO4XX_IO_THRESHOLD: c_uint = 0x00000060;
pub const CRYPTO4XX_GATH_RING_BASE: c_uint = 0x00000064;
pub const CRYPTO4XX_SCAT_RING_BASE: c_uint = 0x00000068;
pub const CRYPTO4XX_PART_RING_SIZE: c_uint = 0x0000006c;
pub const CRYPTO4XX_PART_RING_CFG: c_uint = 0x00000070;
pub const CRYPTO4XX_PDR_BASE_UADDR: c_uint = 0x00000080;
pub const CRYPTO4XX_RDR_BASE_UADDR: c_uint = 0x00000084;
pub const CRYPTO4XX_PKT_SRC_UADDR: c_uint = 0x00000088;
pub const CRYPTO4XX_PKT_DEST_UADDR: c_uint = 0x0000008c;
pub const CRYPTO4XX_SA_UADDR: c_uint = 0x00000090;
pub const CRYPTO4XX_GATH_RING_BASE_UADDR: c_uint = 0x000000A0;
pub const CRYPTO4XX_SCAT_RING_BASE_UADDR: c_uint = 0x000000A4;
pub const CRYPTO4XX_SEQ_RD: c_uint = 0x00000408;
pub const CRYPTO4XX_SEQ_MASK_RD: c_uint = 0x0000040C;
pub const CRYPTO4XX_SA_CMD_0: c_uint = 0x00010600;
pub const CRYPTO4XX_SA_CMD_1: c_uint = 0x00010604;
pub const CRYPTO4XX_STATE_PTR: c_uint = 0x000106dc;
pub const CRYPTO4XX_STATE_IV: c_uint = 0x00010700;
pub const CRYPTO4XX_STATE_HASH_BYTE_CNT_0: c_uint = 0x00010710;
pub const CRYPTO4XX_STATE_HASH_BYTE_CNT_1: c_uint = 0x00010714;
pub const CRYPTO4XX_STATE_IDIGEST_0: c_uint = 0x00010718;
pub const CRYPTO4XX_STATE_IDIGEST_1: c_uint = 0x0001071c;
pub const CRYPTO4XX_DATA_IN: c_uint = 0x00018000;
pub const CRYPTO4XX_DATA_OUT: c_uint = 0x0001c000;
pub const CRYPTO4XX_INT_UNMASK_STAT: c_uint = 0x000500a0;
pub const CRYPTO4XX_INT_MASK_STAT: c_uint = 0x000500a4;
pub const CRYPTO4XX_INT_CLR: c_uint = 0x000500a4;
pub const CRYPTO4XX_INT_EN: c_uint = 0x000500a8;
pub const CRYPTO4XX_INT_PKA: c_uint = 0x00000002;
pub const CRYPTO4XX_INT_PDR_DONE: c_uint = 0x00008000;
pub const CRYPTO4XX_INT_MA_WR_ERR: c_uint = 0x00020000;
pub const CRYPTO4XX_INT_MA_RD_ERR: c_uint = 0x00010000;
pub const CRYPTO4XX_INT_PE_ERR: c_uint = 0x00000200;
pub const CRYPTO4XX_INT_USER_DMA_ERR: c_uint = 0x00000040;
pub const CRYPTO4XX_INT_SLAVE_ERR: c_uint = 0x00000010;
pub const CRYPTO4XX_INT_MASTER_ERR: c_uint = 0x00000008;
pub const CRYPTO4XX_INT_ERROR: c_uint = 0x00030258;
pub const CRYPTO4XX_INT_CFG: c_uint = 0x000500ac;
pub const CRYPTO4XX_INT_DESCR_RD: c_uint = 0x000500b0;
pub const CRYPTO4XX_INT_DESCR_CNT: c_uint = 0x000500b4;
pub const CRYPTO4XX_INT_TIMEOUT_CNT: c_uint = 0x000500b8;
pub const CRYPTO4XX_DEVICE_CTRL: c_uint = 0x00060080;
pub const CRYPTO4XX_DEVICE_ID: c_uint = 0x00060084;
pub const CRYPTO4XX_DEVICE_INFO: c_uint = 0x00060088;
pub const CRYPTO4XX_DMA_USER_SRC: c_uint = 0x00060094;
pub const CRYPTO4XX_DMA_USER_DEST: c_uint = 0x00060098;
pub const CRYPTO4XX_DMA_USER_CMD: c_uint = 0x0006009C;
pub const CRYPTO4XX_DMA_CFG: c_uint = 0x000600d4;
pub const CRYPTO4XX_BYTE_ORDER_CFG: c_uint = 0x000600d8;
pub const CRYPTO4XX_ENDIAN_CFG: c_uint = 0x000600d8;
pub const CRYPTO4XX_PRNG_CTRL: c_uint = 0x00070004;
pub const CRYPTO4XX_PRNG_SEED_L: c_uint = 0x00070008;
pub const CRYPTO4XX_PRNG_SEED_H: c_uint = 0x0007000c;
//
// Initialize CRYPTO ENGINE registers, and memory bases.
//
pub const PPC4XX_PDR_POLL: c_uint = 0x3ff;
pub const PPC4XX_OUTPUT_THRESHOLD: c_int = 2;
pub const PPC4XX_INPUT_THRESHOLD: c_int = 2;
pub const PPC4XX_PD_SIZE: c_int = 6;
pub const PPC4XX_CTX_DONE_INT: c_uint = 0x2000;
pub const PPC4XX_PD_DONE_INT: c_uint = 0x8000;
pub const PPC4XX_TMO_ERR_INT: c_uint = 0x40000;
pub const PPC4XX_BYTE_ORDER: c_uint = 0x22222;
pub const PPC4XX_INTERRUPT_CLR: c_uint = 0x3ffff;
pub const PPC4XX_PRNG_CTRL_AUTO_EN: c_uint = 0x3;
pub const PPC4XX_DC_3DES_EN: c_int = 1;
pub const PPC4XX_TRNG_EN: c_uint = 0x00020000;
pub const PPC4XX_INT_DESCR_CNT: c_int = 7;
pub const PPC4XX_INT_TIMEOUT_CNT: c_int = 0;
pub const PPC4XX_INT_TIMEOUT_CNT_REVB: c_uint = 0x3FF;
pub const PPC4XX_INT_CFG: c_int = 1;
//
// all follow define are ad hoc
//
pub const PPC4XX_RING_RETRY: c_int = 100;
pub const PPC4XX_RING_POLL: c_int = 100;

//
// Generic Security Association (SA) with all possible fields. These will
// never likely used except for reference purpose. These structure format
// can be not changed as the hardware expects them to be layout as defined.
// Field can be removed or reduced but ordering can not be changed.
//
pub const CRYPTO4XX_DMA_CFG_OFFSET: c_uint = 0x40;
#[repr(C)]
#[derive(Copy, Clone)]
pub union ce_pe_dma_cfg {
    pub rsv:7: u32,
    pub dir_host:1: u32,
    pub rsv1:2: u32,
    pub bo_td_en:1: u32,
    pub dis_pdr_upd:1: u32,
    pub bo_sgpd_en:1: u32,
    pub bo_data_en:1: u32,
    pub bo_sa_en:1: u32,
    pub bo_pd_en:1: u32,
    pub rsv2:4: u32,
    pub dynamic_sa_en:1: u32,
    pub pdr_mode:2: u32,
    pub pe_mode:1: u32,
    pub rsv3:5: u32,
    pub reset_sg:1: u32,
    pub reset_pdr:1: u32,
    pub reset_pe:1: u32,
    pub bf: },
    pub w: u32,
    pub __attribute__((packed)): },
pub const CRYPTO4XX_PDR_BASE_OFFSET: c_uint = 0x48;
pub const CRYPTO4XX_RDR_BASE_OFFSET: c_uint = 0x4c;
pub const CRYPTO4XX_RING_SIZE_OFFSET: c_uint = 0x50;
#[repr(C)]
#[derive(Copy, Clone)]
pub union ce_ring_size {
    pub ring_offset:16: u32,
    pub rsv:6: u32,
    pub ring_size:10: u32,
    pub bf: },
    pub w: u32,
    pub __attribute__((packed)): },
pub const CRYPTO4XX_RING_CONTROL_OFFSET: c_uint = 0x54;
#[repr(C)]
#[derive(Copy, Clone)]
pub union ce_ring_control {
    pub continuous:1: u32,
    pub rsv:5: u32,
    pub ring_retry_divisor:10: u32,
    pub rsv1:4: u32,
    pub ring_poll_divisor:10: u32,
    pub bf: },
    pub w: u32,
    pub __attribute__((packed)): },
pub const CRYPTO4XX_IO_THRESHOLD_OFFSET: c_uint = 0x60;
#[repr(C)]
#[derive(Copy, Clone)]
pub union ce_io_threshold {
    pub rsv:6: u32,
    pub output_threshold:10: u32,
    pub rsv1:6: u32,
    pub input_threshold:10: u32,
    pub bf: },
    pub w: u32,
    pub __attribute__((packed)): },
pub const CRYPTO4XX_GATHER_RING_BASE_OFFSET: c_uint = 0x64;
pub const CRYPTO4XX_SCATTER_RING_BASE_OFFSET: c_uint = 0x68;
#[repr(C)]
#[derive(Copy, Clone)]
pub union ce_part_ring_size {
    pub sdr_size:16: u32,
    pub gdr_size:16: u32,
    pub bf: },
    pub w: u32,
    pub __attribute__((packed)): },
pub const MAX_BURST_SIZE_32: c_int = 0;
pub const MAX_BURST_SIZE_64: c_int = 1;
pub const MAX_BURST_SIZE_128: c_int = 2;
pub const MAX_BURST_SIZE_256: c_int = 3;
// gather descriptor control length
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gd_ctl_len {
    pub len:16: u32,
    pub rsv:14: u32,
    pub done:1: u32,
    pub ready:1: u32,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ce_gd {
    pub ptr: u32,
    pub ctl_len: gd_ctl_len,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sd_ctl {
    pub ctl:30: u32,
    pub done:1: u32,
    pub rdy:1: u32,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ce_sd {
    pub ptr: u32,
    pub ctl: sd_ctl,
    pub __attribute__((packed)): },
pub const PD_PAD_CTL_32: c_uint = 0x10;
pub const PD_PAD_CTL_64: c_uint = 0x20;
pub const PD_PAD_CTL_128: c_uint = 0x40;
pub const PD_PAD_CTL_256: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone)]
pub union ce_pd_ctl {
    pub pd_pad_ctl:8: u32,
    pub status:8: u32,
    pub next_hdr:8: u32,
    pub rsv:2: u32,
    pub cached_sa:1: u32,
    pub hash_final:1: u32,
    pub init_arc4:1: u32,
    pub rsv1:1: u32,
    pub pe_done:1: u32,
    pub host_ready:1: u32,
    pub bf: },
    pub w: u32,
    pub __attribute__((packed)): },

#[repr(C)]
#[derive(Copy, Clone)]
pub union ce_pd_ctl_len {
    pub bypass:8: u32,
    pub pe_done:1: u32,
    pub host_ready:1: u32,
    pub rsv:2: u32,
    pub pkt_len:20: u32,
    pub bf: },
    pub w: u32,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ce_pd {
    pub pd_ctl: ce_pd_ctl,
    pub src: u32,
    pub dest: u32,
    pub /: *mut *mut u32 sa; / get from ctx->sa_dma_addr,
    pub /: *mut *mut u32 sa_len; / only if dynamic sa is used,
    pub pd_ctl_len: ce_pd_ctl_len,
    pub __attribute__((packed)): },
