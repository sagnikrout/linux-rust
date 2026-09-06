//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mediatek/mtk_wed_regs.h
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
// Copyright (C) 2020 Felix Fietkau <nbd@nbd.name>

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_wdma_desc {
    pub buf0: __le32,
    pub ctrl: __le32,
    pub buf1: __le32,
    pub info: __le32,
    pub __aligned(4): } __packed,
pub const MTK_WED_REV_ID: c_uint = 0x004;
pub const MTK_WED_RESET: c_uint = 0x008;

pub const MTK_WED_CTRL: c_uint = 0x00c;

pub const MTK_WED_EXT_INT_STATUS: c_uint = 0x020;

pub const MTK_WED_EXT_INT_MASK: c_uint = 0x028;
pub const MTK_WED_EXT_INT_MASK1: c_uint = 0x02c;
pub const MTK_WED_EXT_INT_MASK2: c_uint = 0x030;
pub const MTK_WED_EXT_INT_MASK3: c_uint = 0x034;
pub const MTK_WED_STATUS: c_uint = 0x060;

pub const MTK_WED_WPDMA_STATUS: c_uint = 0x068;

pub const MTK_WED_TX_BM_CTRL: c_uint = 0x080;

pub const MTK_WED_TX_BM_BASE: c_uint = 0x084;
pub const MTK_WED_TX_BM_INIT_PTR: c_uint = 0x088;

pub const MTK_WED_TX_BM_BUF_LEN: c_uint = 0x08c;
pub const MTK_WED_TX_BM_INTF: c_uint = 0x09c;

pub const MTK_WED_TX_BM_DYN_THR: c_uint = 0x0a0;

pub const MTK_WED_TX_TKID_CTRL: c_uint = 0x0c0;

pub const MTK_WED_TX_TKID_INTF: c_uint = 0x0dc;

pub const MTK_WED_TX_TKID_DYN_THR: c_uint = 0x0e0;

pub const MTK_WED_TXP_DW0: c_uint = 0x120;
pub const MTK_WED_TXP_DW1: c_uint = 0x124;

pub const MTK_WED_TXDP_CTRL: c_uint = 0x130;

pub const MTK_WED_RX_BM_TKID_MIB: c_uint = 0x1cc;
pub const MTK_WED_INT_STATUS: c_uint = 0x200;
pub const MTK_WED_INT_MASK: c_uint = 0x204;
pub const MTK_WED_GLO_CFG: c_uint = 0x208;

pub const MTK_WED_RESET_IDX: c_uint = 0x20c;

pub const MTK_WED_SCR0: c_uint = 0x3c0;
pub const MTK_WED_RX1_CTRL2: c_uint = 0x418;
pub const MTK_WED_WPDMA_INT_TRIGGER: c_uint = 0x504;

pub const MTK_WED_WPDMA_GLO_CFG: c_uint = 0x508;

// CONFIG_MEDIATEK_NETSYS_V2

pub const MTK_WED_WPDMA_RESET_IDX: c_uint = 0x50c;

pub const MTK_WED_WPDMA_CTRL: c_uint = 0x518;

pub const MTK_WED_WPDMA_INT_CTRL: c_uint = 0x520;

pub const MTK_WED_WPDMA_INT_MASK: c_uint = 0x524;
pub const MTK_WED_WPDMA_INT_CTRL_TX: c_uint = 0x530;

pub const MTK_WED_WPDMA_INT_CTRL_RX: c_uint = 0x534;

pub const MTK_WED_WPDMA_INT_CTRL_TX_FREE: c_uint = 0x538;

pub const MTK_WED_PCIE_CFG_BASE: c_uint = 0x560;
pub const MTK_WED_PCIE_CFG_BASE: c_uint = 0x560;
pub const MTK_WED_PCIE_CFG_INTM: c_uint = 0x564;
pub const MTK_WED_PCIE_CFG_MSIS: c_uint = 0x568;
pub const MTK_WED_PCIE_INT_TRIGGER: c_uint = 0x570;

pub const MTK_WED_PCIE_INT_CTRL: c_uint = 0x57c;

pub const MTK_WED_WPDMA_CFG_BASE: c_uint = 0x580;
pub const MTK_WED_WPDMA_CFG_INT_MASK: c_uint = 0x584;
pub const MTK_WED_WPDMA_CFG_TX: c_uint = 0x588;
pub const MTK_WED_WPDMA_CFG_TX_FREE: c_uint = 0x58c;

pub const MTK_WED_WPDMA_RX_D_GLO_CFG: c_uint = 0x75c;

pub const MTK_WED_WPDMA_RX_D_RST_IDX: c_uint = 0x760;

pub const MTK_WED_WPDMA_RX_GLO_CFG: c_uint = 0x76c;

pub const MTK_WED_WPDMA_RX_D_COHERENT_MIB: c_uint = 0x78c;
pub const MTK_WED_WPDMA_RX_D_PREF_CFG: c_uint = 0x7b4;

pub const MTK_WED_WPDMA_RX_D_PREF_RX0_SIDX: c_uint = 0x7b8;

pub const MTK_WED_WPDMA_RX_D_PREF_RX1_SIDX: c_uint = 0x7bc;
pub const MTK_WED_WPDMA_RX_D_PREF_FIFO_CFG: c_uint = 0x7c0;

pub const MTK_WED_WDMA_RING_TX: c_uint = 0x800;
pub const MTK_WED_WDMA_TX_MIB: c_uint = 0x810;

pub const MTK_WED_WDMA_RX_PREF_CFG: c_uint = 0x950;

pub const MTK_WED_WDMA_RX_PREF_FIFO_CFG: c_uint = 0x95C;

pub const MTK_WED_WDMA_GLO_CFG: c_uint = 0xa04;

pub const MTK_WED_WDMA_RESET_IDX: c_uint = 0xa08;

pub const MTK_WED_WDMA_INT_CLR: c_uint = 0xa24;

pub const MTK_WED_WDMA_INT_TRIGGER: c_uint = 0xa28;

pub const MTK_WED_WDMA_INT_CTRL: c_uint = 0xa2c;

pub const MTK_WED_WDMA_CFG_BASE: c_uint = 0xaa0;
pub const MTK_WED_WDMA_OFFSET0: c_uint = 0xaa4;
pub const MTK_WED_WDMA_OFFSET1: c_uint = 0xaa8;

pub const MTK_WED_RX_BM_RX_DMAD: c_uint = 0xd80;

pub const MTK_WED_RX_BM_BASE: c_uint = 0xd84;
pub const MTK_WED_RX_BM_INIT_PTR: c_uint = 0xd88;

pub const MTK_WED_RX_PTR: c_uint = 0xd8c;
pub const MTK_WED_RX_BM_DYN_ALLOC_TH: c_uint = 0xdb4;

pub const MTK_WED_RING_OFS_BASE: c_uint = 0x00;
pub const MTK_WED_RING_OFS_COUNT: c_uint = 0x04;
pub const MTK_WED_RING_OFS_CPU_IDX: c_uint = 0x08;
pub const MTK_WED_RING_OFS_DMA_IDX: c_uint = 0x0c;

pub const MTK_WDMA_GLO_CFG: c_uint = 0x204;

pub const MTK_WDMA_RESET_IDX: c_uint = 0x208;

pub const MTK_WDMA_INT_STATUS: c_uint = 0x220;
pub const MTK_WDMA_INT_MASK: c_uint = 0x228;

pub const MTK_WDMA_XDMA_TX_FIFO_CFG: c_uint = 0x238;

pub const MTK_WDMA_XDMA_RX_FIFO_CFG: c_uint = 0x23c;

pub const MTK_WDMA_INT_GRP1: c_uint = 0x250;
pub const MTK_WDMA_INT_GRP2: c_uint = 0x254;
pub const MTK_WDMA_PREF_TX_CFG: c_uint = 0x2d0;

pub const MTK_WDMA_PREF_RX_CFG: c_uint = 0x2dc;

pub const MTK_WDMA_PREF_RX_FIFO_CFG: c_uint = 0x2e0;

pub const MTK_WDMA_PREF_TX_FIFO_CFG: c_uint = 0x2d4;

pub const MTK_WDMA_PREF_SIDX_CFG: c_uint = 0x2e4;

pub const MTK_WDMA_WRBK_TX_CFG: c_uint = 0x300;

pub const MTK_WDMA_WRBK_RX_CFG: c_uint = 0x344;

pub const MTK_WDMA_WRBK_SIDX_CFG: c_uint = 0x388;

// DMA channel mapping
pub const HIFSYS_DMA_AG_MAP: c_uint = 0x008;
pub const MTK_WED_RTQM_GLO_CFG: c_uint = 0xb00;

pub const MTK_WED_RTQM_RST: c_uint = 0xb04;
pub const MTK_WED_RTQM_IGRS0_I2HW_DMAD_CNT: c_uint = 0xb1c;

pub const MTK_WED_RTQM_IGRS0_I2HW_PKT_CNT: c_uint = 0xb28;

pub const MTK_WED_RTQM_IGRS0_FDROP_CNT: c_uint = 0xb34;
pub const MTK_WED_RTQM_IGRS1_I2HW_DMAD_CNT: c_uint = 0xb44;

pub const MTK_WED_RTQM_IGRS1_I2HW_PKT_CNT: c_uint = 0xb50;

pub const MTK_WED_RTQM_IGRS1_FDROP_CNT: c_uint = 0xb5c;
pub const MTK_WED_RTQM_IGRS2_I2HW_DMAD_CNT: c_uint = 0xb6c;

pub const MTK_WED_RTQM_IGRS2_I2HW_PKT_CNT: c_uint = 0xb78;

pub const MTK_WED_RTQM_IGRS2_FDROP_CNT: c_uint = 0xb84;
pub const MTK_WED_RTQM_IGRS3_I2HW_DMAD_CNT: c_uint = 0xb94;

pub const MTK_WED_RTQM_IGRS3_I2HW_PKT_CNT: c_uint = 0xba0;

pub const MTK_WED_RTQM_IGRS3_FDROP_CNT: c_uint = 0xbac;

pub const MTK_WED_RTQM_Q2N_MIB: c_uint = 0xb80;

pub const MTK_WED_RTQM_Q2B_MIB: c_uint = 0xb8c;
pub const MTK_WED_RTQM_PFDBK_MIB: c_uint = 0xb90;
pub const MTK_WED_RTQM_ENQ_CFG0: c_uint = 0xbb8;

pub const MTK_WED_RTQM_FDROP_MIB: c_uint = 0xb84;
pub const MTK_WED_RTQM_ENQ_I2Q_DMAD_CNT: c_uint = 0xbbc;
pub const MTK_WED_RTQM_ENQ_I2N_DMAD_CNT: c_uint = 0xbc0;
pub const MTK_WED_RTQM_ENQ_I2Q_PKT_CNT: c_uint = 0xbc4;
pub const MTK_WED_RTQM_ENQ_I2N_PKT_CNT: c_uint = 0xbc8;
pub const MTK_WED_RTQM_ENQ_USED_ENTRY_CNT: c_uint = 0xbcc;
pub const MTK_WED_RTQM_ENQ_ERR_CNT: c_uint = 0xbd0;
pub const MTK_WED_RTQM_DEQ_DMAD_CNT: c_uint = 0xbd8;
pub const MTK_WED_RTQM_DEQ_Q2I_DMAD_CNT: c_uint = 0xbdc;
pub const MTK_WED_RTQM_DEQ_PKT_CNT: c_uint = 0xbe0;
pub const MTK_WED_RTQM_DEQ_Q2I_PKT_CNT: c_uint = 0xbe4;
pub const MTK_WED_RTQM_DEQ_USED_PFDBK_CNT: c_uint = 0xbe8;
pub const MTK_WED_RTQM_DEQ_ERR_CNT: c_uint = 0xbec;
pub const MTK_WED_RROQM_GLO_CFG: c_uint = 0xc04;
pub const MTK_WED_RROQM_RST_IDX: c_uint = 0xc08;

pub const MTK_WED_RROQM_MIOD_CTRL0: c_uint = 0xc40;
pub const MTK_WED_RROQM_MIOD_CTRL1: c_uint = 0xc44;

pub const MTK_WED_RROQM_MIOD_CTRL2: c_uint = 0xc48;
pub const MTK_WED_RROQM_MIOD_CTRL3: c_uint = 0xc4c;
pub const MTK_WED_RROQM_FDBK_CTRL0: c_uint = 0xc50;
pub const MTK_WED_RROQM_FDBK_CTRL1: c_uint = 0xc54;

pub const MTK_WED_RROQM_FDBK_CTRL2: c_uint = 0xc58;
pub const MTK_WED_RROQ_BASE_L: c_uint = 0xc80;
pub const MTK_WED_RROQ_BASE_H: c_uint = 0xc84;
pub const MTK_WED_RROQM_MIOD_CFG: c_uint = 0xc8c;

pub const MTK_WED_RROQM_MID_MIB: c_uint = 0xcc0;
pub const MTK_WED_RROQM_MOD_MIB: c_uint = 0xcc4;
pub const MTK_WED_RROQM_MOD_COHERENT_MIB: c_uint = 0xcc8;
pub const MTK_WED_RROQM_FDBK_MIB: c_uint = 0xcd0;
pub const MTK_WED_RROQM_FDBK_COHERENT_MIB: c_uint = 0xcd4;
pub const MTK_WED_RROQM_FDBK_IND_MIB: c_uint = 0xce0;
pub const MTK_WED_RROQM_FDBK_ENQ_MIB: c_uint = 0xce4;
pub const MTK_WED_RROQM_FDBK_ANC_MIB: c_uint = 0xce8;
pub const MTK_WED_RROQM_FDBK_ANC2H_MIB: c_uint = 0xcec;
pub const MTK_WED_RX_BM_RX_DMAD: c_uint = 0xd80;
pub const MTK_WED_RX_BM_BASE: c_uint = 0xd84;
pub const MTK_WED_RX_BM_INIT_PTR: c_uint = 0xd88;
pub const MTK_WED_RX_BM_PTR: c_uint = 0xd8c;

pub const MTK_WED_RX_BM_BLEN: c_uint = 0xd90;
pub const MTK_WED_RX_BM_STS: c_uint = 0xd94;
pub const MTK_WED_RX_BM_INTF2: c_uint = 0xd98;
pub const MTK_WED_RX_BM_INTF: c_uint = 0xd9c;
pub const MTK_WED_RX_BM_ERR_STS: c_uint = 0xda8;
pub const MTK_RRO_IND_CMD_SIGNATURE: c_uint = 0xe00;

pub const MTK_WED_IND_CMD_RX_CTRL0: c_uint = 0xe04;

pub const MTK_WED_IND_CMD_RX_CTRL1: c_uint = 0xe08;
pub const MTK_WED_IND_CMD_RX_CTRL2: c_uint = 0xe0c;

pub const MTK_WED_RRO_CFG0: c_uint = 0xe10;
pub const MTK_WED_RRO_CFG1: c_uint = 0xe14;

pub const MTK_WED_ADDR_ELEM_CFG0: c_uint = 0xe18;
pub const MTK_WED_ADDR_ELEM_CFG1: c_uint = 0xe1c;

pub const MTK_WED_ADDR_ELEM_TBL_CFG: c_uint = 0xe20;

pub const MTK_WED_RADDR_ELEM_TBL_WDATA: c_uint = 0xe24;
pub const MTK_WED_RADDR_ELEM_TBL_RDATA: c_uint = 0xe28;
pub const MTK_WED_PN_CHECK_CFG: c_uint = 0xe30;

pub const MTK_WED_PN_CHECK_WDATA_M: c_uint = 0xe38;

pub const MTK_WED_RRO_MSDU_PG_RING2_CFG: c_uint = 0xe58;

pub const MTK_WED_RRO_PG_BM_RX_DMAM: c_uint = 0xeb0;

pub const MTK_WED_RRO_PG_BM_BASE: c_uint = 0xeb4;
pub const MTK_WED_RRO_PG_BM_INIT_PTR: c_uint = 0xeb8;

pub const MTK_WED_WPDMA_INT_CTRL_RRO_RX: c_uint = 0xeec;

pub const MTK_WED_WPDMA_INT_CTRL_RRO_MSDU_PG: c_uint = 0xef4;

pub const MTK_WED_RRO_RX_HW_STS: c_uint = 0xf00;

pub const MTK_WED_RX_IND_CMD_CNT0: c_uint = 0xf20;

pub const MTK_WED_RX_PN_CHK_CNT: c_uint = 0xf70;

pub const MTK_WED_WOCPU_VIEW_MIOD_BASE: c_uint = 0x8000;
pub const MTK_WED_PCIE_INT_MASK: c_uint = 0x0;
pub const MTK_WED_AMSDU_FIFO: c_uint = 0x1800;

pub const MTK_WED_AMSDU_STA_INFO: c_uint = 0x01810;

pub const MTK_WED_AMSDU_STA_INFO_INIT: c_uint = 0x01814;

pub const MTK_WED_AMSDU_PSE: c_uint = 0x1910;

pub const MTK_WED_AMSDU_HIFTXD_CFG: c_uint = 0x1968;

pub const MTK_WED_MON_AMSDU_FIFO_DMAD: c_uint = 0x1a34;

pub const MTK_WED_MON_AMSDU_QMEM_STS1: c_uint = 0x1e04;

pub const MTK_WED_PCIE_BASE: c_uint = 0x11280000;
pub const MTK_WED_PCIE_BASE0: c_uint = 0x11300000;
pub const MTK_WED_PCIE_BASE1: c_uint = 0x11310000;
pub const MTK_WED_PCIE_BASE2: c_uint = 0x11290000;
