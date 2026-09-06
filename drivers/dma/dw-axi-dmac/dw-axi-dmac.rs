//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma/dw-axi-dmac/dw-axi-dmac.h
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
// (C) 2017-2018 Synopsys, Inc. (www.synopsys.com)
//
// Synopsys DesignWare AXI DMA Controller driver.
//
// Author: Eugeniy Paltsev <Eugeniy.Paltsev@synopsys.com>
//

pub const DMAC_MAX_CHANNELS: c_int = 32;
pub const DMAC_MAX_MASTERS: c_int = 2;
pub const DMAC_MAX_BLK_SIZE: c_uint = 0x200000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_axi_dma_hcfg {
    pub nr_channels: u32,
    pub nr_masters: u32,
    pub m_data_width: u32,
    pub block_size: [u32; DMAC_MAX_CHANNELS],
    pub priority: [u32; DMAC_MAX_CHANNELS],
// maximum supported axi burst length
    pub axi_rw_burst_len: u32,
// Register map for DMAX_NUM_CHANNELS <= 8
    pub reg_map_8_channels: bool,
    pub restrict_axi_burst_len: bool,
    pub use_cfg2: bool,
    pub use_handshake_as_channel_number: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct axi_dma_chan {
    pub chip: *mut axi_dma_chip,
    pub chan_regs: *mut void __iomem,
    pub id: u8,
    pub hw_handshake_num: u8,
    pub descs_allocated: core::sync::atomic::AtomicI32,
    pub desc_pool: *mut dma_pool,
    pub vc: virt_dma_chan,
    pub desc: *mut axi_dma_desc,
    pub config: dma_slave_config,
    pub direction: dma_transfer_direction,
    pub cyclic: bool,
// these other elements are all protected by vc.lock
    pub is_paused: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_axi_dma {
    pub dma: dma_device,
    pub hdata: *mut dw_axi_dma_hcfg,
    pub dma_parms: device_dma_parameters,
// channels
    pub chan: *mut axi_dma_chan,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct axi_dma_chip {
    pub dev: *mut device,
    pub irq: [c_int; DMAC_MAX_CHANNELS],
    pub regs: *mut void __iomem,
    pub apb_regs: *mut void __iomem,
    pub core_clk: *mut clk,
    pub cfgr_clk: *mut clk,
    pub dw: *mut dw_axi_dma,
}

// LLI == Linked List Item
#[repr(C)]
#[derive(Copy, Clone)]
pub struct axi_dma_hw_desc {
    pub lli: *mut axi_dma_lli,
    pub llp: dma_addr_t,
    pub len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct axi_dma_desc {
    pub hw_desc: *mut axi_dma_hw_desc,
    pub vd: virt_dma_desc,
    pub chan: *mut axi_dma_chan,
    pub completed_blocks: u32,
    pub length: u32,
    pub period_len: u32,
    pub nr_hw_descs: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct axi_dma_chan_config {
    pub dst_multblk_type: u8,
    pub src_multblk_type: u8,
    pub dst_per: u8,
    pub src_per: u8,
    pub tt_fc: u8,
    pub prior: u8,
    pub hs_sel_dst: u8,
    pub hs_sel_src: u8,
}

extern "C" {
    pub fn container_of(_arg: vd, axi_dma_desc: struct, _arg: vd) -> return;
}
extern "C" {
    pub fn container_of(_arg: vc, axi_dma_chan: struct, _arg: vc) -> return;
}
extern "C" {
    pub fn vc_to_axi_dma_chan(_arg: to_virt_chan(dchan)) -> return;
}
pub const COMMON_REG_LEN: c_uint = 0x100;
pub const CHAN_REG_LEN: c_uint = 0x100;
// Common registers offset
pub const DMAC_ID: c_uint = 0x000 /* R DMAC ID */;
pub const DMAC_COMPVER: c_uint = 0x008 /* R DMAC Component Version */;
pub const DMAC_CFG: c_uint = 0x010 /* R/W DMAC Configuration */;
pub const DMAC_CHEN: c_uint = 0x018 /* R/W DMAC Channel Enable */;
pub const DMAC_CHEN_L: c_uint = 0x018 /* R/W DMAC Channel Enable 00-31 */;
pub const DMAC_CHEN_H: c_uint = 0x01C /* R/W DMAC Channel Enable 32-63 */;
pub const DMAC_CHSUSPREG: c_uint = 0x020 /* R/W DMAC Channel Suspend */;
pub const DMAC_CHABORTREG: c_uint = 0x028 /* R/W DMAC Channel Abort */;
pub const DMAC_INTSTATUS: c_uint = 0x030 /* R DMAC Interrupt Status */;
pub const DMAC_COMMON_INTCLEAR: c_uint = 0x038 /* W DMAC Interrupt Clear */;
pub const DMAC_COMMON_INTSTATUS_ENA: c_uint = 0x040 /* R DMAC Interrupt Status Enable */;
pub const DMAC_COMMON_INTSIGNAL_ENA: c_uint = 0x048 /* R/W DMAC Interrupt Signal Enable */;
pub const DMAC_COMMON_INTSTATUS: c_uint = 0x050 /* R DMAC Interrupt Status */;
pub const DMAC_RESET: c_uint = 0x058 /* R DMAC Reset Register1 */;
// DMA channel registers offset
pub const CH_SAR: c_uint = 0x000 /* R/W Chan Source Address */;
pub const CH_DAR: c_uint = 0x008 /* R/W Chan Destination Address */;
pub const CH_BLOCK_TS: c_uint = 0x010 /* R/W Chan Block Transfer Size */;
pub const CH_CTL: c_uint = 0x018 /* R/W Chan Control */;
pub const CH_CTL_L: c_uint = 0x018 /* R/W Chan Control 00-31 */;
pub const CH_CTL_H: c_uint = 0x01C /* R/W Chan Control 32-63 */;
pub const CH_CFG: c_uint = 0x020 /* R/W Chan Configuration */;
pub const CH_CFG_L: c_uint = 0x020 /* R/W Chan Configuration 00-31 */;
pub const CH_CFG_H: c_uint = 0x024 /* R/W Chan Configuration 32-63 */;
pub const CH_LLP: c_uint = 0x028 /* R/W Chan Linked List Pointer */;
pub const CH_STATUS: c_uint = 0x030 /* R Chan Status */;
pub const CH_SWHSSRC: c_uint = 0x038 /* R/W Chan SW Handshake Source */;
pub const CH_SWHSDST: c_uint = 0x040 /* R/W Chan SW Handshake Destination */;
pub const CH_BLK_TFR_RESUMEREQ: c_uint = 0x048 /* W Chan Block Transfer Resume Req */;
pub const CH_AXI_ID: c_uint = 0x050 /* R/W Chan AXI ID */;
pub const CH_AXI_QOS: c_uint = 0x058 /* R/W Chan AXI QOS */;
pub const CH_SSTAT: c_uint = 0x060 /* R Chan Source Status */;
pub const CH_DSTAT: c_uint = 0x068 /* R Chan Destination Status */;
pub const CH_SSTATAR: c_uint = 0x070 /* R/W Chan Source Status Fetch Addr */;
pub const CH_DSTATAR: c_uint = 0x078 /* R/W Chan Destination Status Fetch Addr */;
pub const CH_INTSTATUS_ENA: c_uint = 0x080 /* R/W Chan Interrupt Status Enable */;
pub const CH_INTSTATUS: c_uint = 0x088 /* R/W Chan Interrupt Status */;
pub const CH_INTSIGNAL_ENA: c_uint = 0x090 /* R/W Chan Interrupt Signal Enable */;
pub const CH_INTCLEAR: c_uint = 0x098 /* W Chan Interrupt Clear */;
// These Apb registers are used by Intel KeemBay SoC
pub const DMAC_APB_CFG: c_uint = 0x000 /* DMAC Apb Configuration Register */;
pub const DMAC_APB_STAT: c_uint = 0x004 /* DMAC Apb Status Register */;
pub const DMAC_APB_DEBUG_STAT_0: c_uint = 0x008 /* DMAC Apb Debug Status Register 0 */;
pub const DMAC_APB_DEBUG_STAT_1: c_uint = 0x00C /* DMAC Apb Debug Status Register 1 */;
pub const DMAC_APB_HW_HS_SEL_0: c_uint = 0x010 /* DMAC Apb HW HS register 0 */;
pub const DMAC_APB_HW_HS_SEL_1: c_uint = 0x014 /* DMAC Apb HW HS register 1 */;
pub const DMAC_APB_LPI: c_uint = 0x018 /* DMAC Apb Low Power Interface Reg */;
pub const DMAC_APB_BYTE_WR_CH_EN: c_uint = 0x01C /* DMAC Apb Byte Write Enable */;
pub const DMAC_APB_HALFWORD_WR_CH_EN: c_uint = 0x020 /* DMAC Halfword write enables */;
pub const UNUSED_CHANNEL: c_uint = 0x3F /* Set unused DMA channel to 0x3F */;
pub const DMA_APB_HS_SEL_BIT_SIZE: c_uint = 0x08 /* HW handshake bits per channel */;
pub const DMA_APB_HS_SEL_MASK: c_uint = 0xFF /* HW handshake select masks */;
pub const MAX_BLOCK_SIZE: c_uint = 0x1000 /* 1024 blocks * 4 bytes data width */;
pub const DMA_REG_MAP_CH_REF: c_uint = 0x08 /* Channel count to choose register map */;
// DMAC_CFG
pub const DMAC_EN_POS: c_int = 0;

pub const INT_EN_POS: c_int = 1;

// DMAC_CHEN
pub const DMAC_CHAN_EN_SHIFT: c_int = 0;
pub const DMAC_CHAN_EN_WE_SHIFT: c_int = 8;
pub const DMAC_CHAN_SUSP_SHIFT: c_int = 16;
pub const DMAC_CHAN_SUSP_WE_SHIFT: c_int = 24;
// DMAC_CHEN2
pub const DMAC_CHAN_EN2_WE_SHIFT: c_int = 16;
// DMAC CHAN BLOCKS
pub const DMAC_CHAN_BLOCK_SHIFT: c_int = 32;
pub const DMAC_CHAN_16: c_int = 16;
// DMAC_CHSUSP
pub const DMAC_CHAN_SUSP2_SHIFT: c_int = 0;
pub const DMAC_CHAN_SUSP2_WE_SHIFT: c_int = 16;
// CH_CTL_H

pub const CH_CTL_H_ARLEN_POS: c_int = 7;

pub const CH_CTL_H_AWLEN_POS: c_int = 16;

// CH_CTL_L

pub const CH_CTL_L_DST_MSIZE_POS: c_int = 18;
pub const CH_CTL_L_SRC_MSIZE_POS: c_int = 14;
pub const CH_CTL_L_DST_WIDTH_POS: c_int = 11;
pub const CH_CTL_L_SRC_WIDTH_POS: c_int = 8;
pub const CH_CTL_L_DST_INC_POS: c_int = 6;
pub const CH_CTL_L_SRC_INC_POS: c_int = 4;

// CH_CFG_H
pub const CH_CFG_H_PRIORITY_POS: c_int = 17;
pub const CH_CFG_H_DST_PER_POS: c_int = 12;
pub const CH_CFG_H_SRC_PER_POS: c_int = 7;
pub const CH_CFG_H_HS_SEL_DST_POS: c_int = 4;
pub const CH_CFG_H_HS_SEL_SRC_POS: c_int = 3;
pub const CH_CFG_H_TT_FC_POS: c_int = 0;
// CH_CFG_L
pub const CH_CFG_L_DST_MULTBLK_TYPE_POS: c_int = 2;
pub const CH_CFG_L_SRC_MULTBLK_TYPE_POS: c_int = 0;
// CH_CFG2
pub const CH_CFG2_L_SRC_PER_POS: c_int = 4;
pub const CH_CFG2_L_DST_PER_POS: c_int = 11;
pub const CH_CFG2_H_TT_FC_POS: c_int = 0;
pub const CH_CFG2_H_HS_SEL_SRC_POS: c_int = 3;
pub const CH_CFG2_H_HS_SEL_DST_POS: c_int = 4;
pub const CH_CFG2_H_PRIORITY_POS: c_int = 20;
//
// DW AXI DMA channel interrupts
//
// @DWAXIDMAC_IRQ_NONE: Bitmask of no one interrupt
// @DWAXIDMAC_IRQ_BLOCK_TRF: Block transfer complete
// @DWAXIDMAC_IRQ_DMA_TRF: Dma transfer complete
// @DWAXIDMAC_IRQ_SRC_TRAN: Source transaction complete
// @DWAXIDMAC_IRQ_DST_TRAN: Destination transaction complete
// @DWAXIDMAC_IRQ_SRC_DEC_ERR: Source decode error
// @DWAXIDMAC_IRQ_DST_DEC_ERR: Destination decode error
// @DWAXIDMAC_IRQ_SRC_SLV_ERR: Source slave error
// @DWAXIDMAC_IRQ_DST_SLV_ERR: Destination slave error
// @DWAXIDMAC_IRQ_LLI_RD_DEC_ERR: LLI read decode error
// @DWAXIDMAC_IRQ_LLI_WR_DEC_ERR: LLI write decode error
// @DWAXIDMAC_IRQ_LLI_RD_SLV_ERR: LLI read slave error
// @DWAXIDMAC_IRQ_LLI_WR_SLV_ERR: LLI write slave error
// @DWAXIDMAC_IRQ_INVALID_ERR: LLI invalid error or Shadow register error
// @DWAXIDMAC_IRQ_MULTIBLKTYPE_ERR: Slave Interface Multiblock type error
// @DWAXIDMAC_IRQ_DEC_ERR: Slave Interface decode error
// @DWAXIDMAC_IRQ_WR2RO_ERR: Slave Interface write to read only error
// @DWAXIDMAC_IRQ_RD2RWO_ERR: Slave Interface read to write only error
// @DWAXIDMAC_IRQ_WRONCHEN_ERR: Slave Interface write to channel error
// @DWAXIDMAC_IRQ_SHADOWREG_ERR: Slave Interface shadow reg error
// @DWAXIDMAC_IRQ_WRONHOLD_ERR: Slave Interface hold error
// @DWAXIDMAC_IRQ_LOCK_CLEARED: Lock Cleared Status
// @DWAXIDMAC_IRQ_SRC_SUSPENDED: Source Suspended Status
// @DWAXIDMAC_IRQ_SUSPENDED: Channel Suspended Status
// @DWAXIDMAC_IRQ_DISABLED: Channel Disabled Status
// @DWAXIDMAC_IRQ_ABORTED: Channel Aborted Status
// @DWAXIDMAC_IRQ_ALL_ERR: Bitmask of all error interrupts
// @DWAXIDMAC_IRQ_ALL: Bitmask of all interrupts
//
