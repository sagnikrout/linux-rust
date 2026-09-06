//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/moxa/moxart_ether.h
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


// MOXA ART Ethernet (RTL8201CP) driver.
//
// Copyright (C) 2013 Jonas Jensen
//
// Jonas Jensen <jonas.jensen@gmail.com>
//
// Based on code from
// Moxa Technology Co., Ltd. <www.moxa.com>
//
// This file is licensed under the terms of the GNU General Public
// License version 2.  This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//
pub const TX_REG_OFFSET_DESC0: c_int = 0;
pub const TX_REG_OFFSET_DESC1: c_int = 4;
pub const TX_REG_OFFSET_DESC2: c_int = 8;
pub const TX_REG_DESC_SIZE: c_int = 16;
pub const RX_REG_OFFSET_DESC0: c_int = 0;
pub const RX_REG_OFFSET_DESC1: c_int = 4;
pub const RX_REG_OFFSET_DESC2: c_int = 8;
pub const RX_REG_DESC_SIZE: c_int = 16;
pub const TX_DESC0_PKT_LATE_COL: c_uint = 0x1		/* abort, late collision */;
pub const TX_DESC0_RX_PKT_EXS_COL: c_uint = 0x2		/* abort, >16 collisions */;
pub const TX_DESC0_DMA_OWN: c_uint = 0x80000000	/* owned by controller */;
pub const TX_DESC1_BUF_SIZE_MASK: c_uint = 0x7ff;
pub const TX_DESC1_LTS: c_uint = 0x8000000	/* last TX packet */;
pub const TX_DESC1_FTS: c_uint = 0x10000000	/* first TX packet */;
pub const TX_DESC1_FIFO_COMPLETE: c_uint = 0x20000000;
pub const TX_DESC1_INTR_COMPLETE: c_uint = 0x40000000;
pub const TX_DESC1_END: c_uint = 0x80000000;
pub const TX_DESC2_ADDRESS_PHYS: c_int = 0;
pub const TX_DESC2_ADDRESS_VIRT: c_int = 4;
pub const RX_DESC0_FRAME_LEN: c_int = 0;
pub const RX_DESC0_FRAME_LEN_MASK: c_uint = 0x7FF;
pub const RX_DESC0_MULTICAST: c_uint = 0x10000;
pub const RX_DESC0_BROADCAST: c_uint = 0x20000;
pub const RX_DESC0_ERR: c_uint = 0x40000;
pub const RX_DESC0_CRC_ERR: c_uint = 0x80000;
pub const RX_DESC0_FTL: c_uint = 0x100000;
pub const RX_DESC0_RUNT: c_uint = 0x200000	/* packet less than 64 bytes */;
pub const RX_DESC0_ODD_NB: c_uint = 0x400000	/* receive odd nibbles */;
pub const RX_DESC0_LRS: c_uint = 0x10000000	/* last receive segment */;
pub const RX_DESC0_FRS: c_uint = 0x20000000	/* first receive segment */;
pub const RX_DESC0_DMA_OWN: c_uint = 0x80000000;
pub const RX_DESC1_BUF_SIZE_MASK: c_uint = 0x7FF;
pub const RX_DESC1_END: c_uint = 0x80000000;
pub const RX_DESC2_ADDRESS_PHYS: c_int = 0;
pub const RX_DESC2_ADDRESS_VIRT: c_int = 4;
pub const TX_DESC_NUM: c_int = 64;

pub const TX_BUF_SIZE: c_int = 1600;

pub const TX_WAKE_THRESHOLD: c_int = 16;
pub const RX_DESC_NUM: c_int = 64;

pub const RX_BUF_SIZE: c_int = 1600;

pub const REG_INTERRUPT_STATUS: c_int = 0;
pub const REG_INTERRUPT_MASK: c_int = 4;
pub const REG_MAC_MS_ADDRESS: c_int = 8;
pub const REG_MAC_LS_ADDRESS: c_int = 12;
pub const REG_MCAST_HASH_TABLE0: c_int = 16;
pub const REG_MCAST_HASH_TABLE1: c_int = 20;
pub const REG_TX_POLL_DEMAND: c_int = 24;
pub const REG_RX_POLL_DEMAND: c_int = 28;
pub const REG_TXR_BASE_ADDRESS: c_int = 32;
pub const REG_RXR_BASE_ADDRESS: c_int = 36;
pub const REG_INT_TIMER_CTRL: c_int = 40;
pub const REG_APOLL_TIMER_CTRL: c_int = 44;
pub const REG_DMA_BLEN_CTRL: c_int = 48;
pub const REG_RESERVED1: c_int = 52;
pub const REG_MAC_CTRL: c_int = 136;
pub const REG_MAC_STATUS: c_int = 140;
pub const REG_PHY_CTRL: c_int = 144;
pub const REG_PHY_WRITE_DATA: c_int = 148;
pub const REG_FLOW_CTRL: c_int = 152;
pub const REG_BACK_PRESSURE: c_int = 156;
pub const REG_RESERVED2: c_int = 160;
pub const REG_TEST_SEED: c_int = 196;
pub const REG_DMA_FIFO_STATE: c_int = 200;
pub const REG_TEST_MODE: c_int = 204;
pub const REG_RESERVED3: c_int = 208;
pub const REG_TX_COL_COUNTER: c_int = 212;
pub const REG_RPF_AEP_COUNTER: c_int = 216;
pub const REG_XM_PG_COUNTER: c_int = 220;
pub const REG_RUNT_TLC_COUNTER: c_int = 224;
pub const REG_CRC_FTL_COUNTER: c_int = 228;
pub const REG_RLC_RCC_COUNTER: c_int = 232;
pub const REG_BROC_COUNTER: c_int = 236;
pub const REG_MULCA_COUNTER: c_int = 240;
pub const REG_RP_COUNTER: c_int = 244;
pub const REG_XP_COUNTER: c_int = 248;
pub const REG_PHY_CTRL_OFFSET: c_uint = 0x0;
pub const REG_PHY_STATUS: c_uint = 0x1;
pub const REG_PHY_ID1: c_uint = 0x2;
pub const REG_PHY_ID2: c_uint = 0x3;
pub const REG_PHY_ANA: c_uint = 0x4;
pub const REG_PHY_ANLPAR: c_uint = 0x5;
pub const REG_PHY_ANE: c_uint = 0x6;
pub const REG_PHY_ECTRL1: c_uint = 0x10;
pub const REG_PHY_QPDS: c_uint = 0x11;
pub const REG_PHY_10BOP: c_uint = 0x12;
pub const REG_PHY_ECTRL2: c_uint = 0x13;
pub const REG_PHY_FTMAC100_WRITE: c_uint = 0x8000000;
pub const REG_PHY_FTMAC100_READ: c_uint = 0x4000000;
// REG_INTERRUPT_STATUS

// REG_INTERRUPT_MASK

// REG_MAC_MS_ADDRESS
pub const MAC_MADR_MASK: c_uint = 0xffff	/* 2 MSB MAC address */;
// REG_INT_TIMER_CTRL

pub const TXINT_THR_MASK: c_uint = 0x7000;
pub const TXINT_CNT_MASK: c_uint = 0xf00;

pub const RXINT_THR_MASK: c_uint = 0x70;
pub const RXINT_CNT_MASK: c_uint = 0xF;
// REG_APOLL_TIMER_CTRL

pub const TXPOLL_CNT_MASK: c_uint = 0xf00;
pub const TXPOLL_CNT_SHIFT_BIT: c_int = 8;

pub const RXPOLL_CNT_MASK: c_uint = 0xF;
pub const RXPOLL_CNT_SHIFT_BIT: c_int = 0;
// REG_DMA_BLEN_CTRL

pub const RXFIFO_HTHR_MASK: c_uint = 0x1c0;
pub const RXFIFO_LTHR_MASK: c_uint = 0x38;

// REG_MAC_CTRL

// REG_MAC_STATUS

// REG_PHY_CTRL

pub const REGAD_MASK: c_uint = 0x3e00000;
pub const PHYAD_MASK: c_uint = 0x1f0000;
pub const MIIRDATA_MASK: c_uint = 0xffff;
// REG_PHY_WRITE_DATA
pub const MIIWDATA_MASK: c_uint = 0xffff;
// REG_FLOW_CTRL
pub const PAUSE_TIME_MASK: c_uint = 0xffff0000;
pub const FC_HIGH_MASK: c_uint = 0xf000;
pub const FC_LOW_MASK: c_uint = 0xf00;

// REG_BACK_PRESSURE
pub const BACKP_LOW_MASK: c_uint = 0xf00;
pub const BACKP_JAM_LEN_MASK: c_uint = 0xf0;

// REG_TEST_SEED
pub const TEST_SEED_MASK: c_uint = 0x3fff;
// REG_DMA_FIFO_STATE

pub const TX_DMA2_SM_MASK: c_uint = 0x7000;
pub const TX_DMA1_SM_MASK: c_uint = 0xf00;
pub const RX_DMA2_SM_MASK: c_uint = 0x70;
pub const RX_DMA1_SM_MASK: c_uint = 0xF;
// REG_TEST_MODE

pub const TEST_TIME_MASK: c_uint = 0xffc00;
pub const TEST_EXCEL_MASK: c_uint = 0x3e0;
// REG_TX_COL_COUNTER
pub const TX_MCOL_MASK: c_uint = 0xffff0000;
pub const TX_MCOL_SHIFT_BIT: c_int = 16;
pub const TX_SCOL_MASK: c_uint = 0xffff;
pub const TX_SCOL_SHIFT_BIT: c_int = 0;
// REG_RPF_AEP_COUNTER
pub const RPF_MASK: c_uint = 0xffff0000;
pub const RPF_SHIFT_BIT: c_int = 16;
pub const AEP_MASK: c_uint = 0xffff;
pub const AEP_SHIFT_BIT: c_int = 0;
// REG_XM_PG_COUNTER
pub const XM_MASK: c_uint = 0xffff0000;
pub const XM_SHIFT_BIT: c_int = 16;
pub const PG_MASK: c_uint = 0xffff;
pub const PG_SHIFT_BIT: c_int = 0;
// REG_RUNT_TLC_COUNTER
pub const RUNT_CNT_MASK: c_uint = 0xffff0000;
pub const RUNT_CNT_SHIFT_BIT: c_int = 16;
pub const TLCC_MASK: c_uint = 0xffff;
pub const TLCC_SHIFT_BIT: c_int = 0;
// REG_CRC_FTL_COUNTER
pub const CRCER_CNT_MASK: c_uint = 0xffff0000;
pub const CRCER_CNT_SHIFT_BIT: c_int = 16;
pub const FTL_CNT_MASK: c_uint = 0xffff;
pub const FTL_CNT_SHIFT_BIT: c_int = 0;
// REG_RLC_RCC_COUNTER
pub const RLC_MASK: c_uint = 0xffff0000;
pub const RLC_SHIFT_BIT: c_int = 16;
pub const RCC_MASK: c_uint = 0xffff;
pub const RCC_SHIFT_BIT: c_int = 0;
// REG_PHY_STATUS
pub const AN_COMPLETE: c_uint = 0x20;
pub const LINK_STATUS: c_uint = 0x4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct moxart_mac_priv_t {
    pub pdev: *mut platform_device,
    pub base: *mut void __iomem,
    pub reg_maccr: c_uint,
    pub reg_imr: c_uint,
    pub napi: napi_struct,
    pub ndev: *mut net_device,
    pub rx_base: dma_addr_t,
    pub rx_mapping: [dma_addr_t; RX_DESC_NUM],
    pub rx_desc_base: *mut c_void,
    pub rx_buf_base: *mut c_uchar,
    pub rx_buf: [*mut c_uchar; RX_DESC_NUM],
    pub rx_head: c_uint,
    pub rx_buf_size: c_uint,
    pub tx_base: dma_addr_t,
    pub tx_mapping: [dma_addr_t; TX_DESC_NUM],
    pub tx_desc_base: *mut c_void,
    pub tx_buf_base: *mut c_uchar,
    pub tx_buf: [*mut c_uchar; RX_DESC_NUM],
    pub tx_head: c_uint,
    pub tx_buf_size: c_uint,
    pub txlock: spinlock_t,
    pub tx_len: [c_uint; TX_DESC_NUM],
    pub tx_skb: [*mut sk_buff; TX_DESC_NUM],
    pub tx_tail: c_uint,
}

