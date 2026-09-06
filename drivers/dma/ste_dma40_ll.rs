//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma/ste_dma40_ll.h
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
// Copyright (C) ST-Ericsson SA 2007-2010
// Author: Per Friden <per.friden@stericsson.com> for ST-Ericsson SA
// Author: Jonas Aaberg <jonas.aberg@stericsson.com> for ST-Ericsson SA
//
pub const D40_DREG_PCBASE: c_uint = 0x400;

pub const D40_LCPA_CHAN_SIZE: c_int = 32;
pub const D40_LCPA_CHAN_DST_DELTA: c_int = 16;

pub const D40_GROUP_SIZE: c_int = 8;

// Most bits of the CFG register are the same in log as in phy mode
pub const D40_SREG_CFG_MST_POS: c_int = 15;
pub const D40_SREG_CFG_TIM_POS: c_int = 14;
pub const D40_SREG_CFG_EIM_POS: c_int = 13;
pub const D40_SREG_CFG_LOG_INCR_POS: c_int = 12;
pub const D40_SREG_CFG_PHY_PEN_POS: c_int = 12;
pub const D40_SREG_CFG_PSIZE_POS: c_int = 10;
pub const D40_SREG_CFG_ESIZE_POS: c_int = 8;
pub const D40_SREG_CFG_PRI_POS: c_int = 7;
pub const D40_SREG_CFG_LBE_POS: c_int = 6;
pub const D40_SREG_CFG_LOG_GIM_POS: c_int = 5;
pub const D40_SREG_CFG_LOG_MFU_POS: c_int = 4;
pub const D40_SREG_CFG_PHY_TM_POS: c_int = 4;
pub const D40_SREG_CFG_PHY_EVTL_POS: c_int = 0;
// Standard channel parameters - basic mode (element register)
pub const D40_SREG_ELEM_PHY_ECNT_POS: c_int = 16;
pub const D40_SREG_ELEM_PHY_EIDX_POS: c_int = 0;

// Standard channel parameters - basic mode (Link register)
pub const D40_SREG_LNK_PHY_TCP_POS: c_int = 0;
pub const D40_SREG_LNK_PHY_LMP_POS: c_int = 1;
pub const D40_SREG_LNK_PHY_PRE_POS: c_int = 2;
//
// Source  destination link address. Contains the
// 29-bit byte word aligned address of the reload area.
//
pub const D40_SREG_LNK_PHYS_LNK_MASK: c_uint = 0xFFFFFFF8UL;
// Standard basic channel logical mode
// Element register
pub const D40_SREG_ELEM_LOG_ECNT_POS: c_int = 16;
pub const D40_SREG_ELEM_LOG_LIDX_POS: c_int = 8;
pub const D40_SREG_ELEM_LOG_LOS_POS: c_int = 1;
pub const D40_SREG_ELEM_LOG_TCP_POS: c_int = 0;

// Link register

// Standard basic channel logical params in memory
// LCSP0
pub const D40_MEM_LCSP0_ECNT_POS: c_int = 16;
pub const D40_MEM_LCSP0_SPTR_POS: c_int = 0;

// LCSP1
pub const D40_MEM_LCSP1_SPTR_POS: c_int = 16;
pub const D40_MEM_LCSP1_SCFG_MST_POS: c_int = 15;
pub const D40_MEM_LCSP1_SCFG_TIM_POS: c_int = 14;
pub const D40_MEM_LCSP1_SCFG_EIM_POS: c_int = 13;
pub const D40_MEM_LCSP1_SCFG_INCR_POS: c_int = 12;
pub const D40_MEM_LCSP1_SCFG_PSIZE_POS: c_int = 10;
pub const D40_MEM_LCSP1_SCFG_ESIZE_POS: c_int = 8;
pub const D40_MEM_LCSP1_SLOS_POS: c_int = 1;
pub const D40_MEM_LCSP1_STCP_POS: c_int = 0;

// LCSP2
pub const D40_MEM_LCSP2_ECNT_POS: c_int = 16;

// LCSP3
pub const D40_MEM_LCSP3_DCFG_MST_POS: c_int = 15;
pub const D40_MEM_LCSP3_DCFG_TIM_POS: c_int = 14;
pub const D40_MEM_LCSP3_DCFG_EIM_POS: c_int = 13;
pub const D40_MEM_LCSP3_DCFG_INCR_POS: c_int = 12;
pub const D40_MEM_LCSP3_DCFG_PSIZE_POS: c_int = 10;
pub const D40_MEM_LCSP3_DCFG_ESIZE_POS: c_int = 8;
pub const D40_MEM_LCSP3_DLOS_POS: c_int = 1;
pub const D40_MEM_LCSP3_DTCP_POS: c_int = 0;

// Standard channel parameter register offsets
pub const D40_CHAN_REG_SSCFG: c_uint = 0x00;
pub const D40_CHAN_REG_SSELT: c_uint = 0x04;
pub const D40_CHAN_REG_SSPTR: c_uint = 0x08;
pub const D40_CHAN_REG_SSLNK: c_uint = 0x0C;
pub const D40_CHAN_REG_SDCFG: c_uint = 0x10;
pub const D40_CHAN_REG_SDELT: c_uint = 0x14;
pub const D40_CHAN_REG_SDPTR: c_uint = 0x18;
pub const D40_CHAN_REG_SDLNK: c_uint = 0x1C;
// DMA Register Offsets
pub const D40_DREG_GCC: c_uint = 0x000;
pub const D40_DREG_GCC_ENA: c_uint = 0x1;
// This assumes that there are only 4 event groups
pub const D40_DREG_GCC_ENABLE_ALL: c_uint = 0x3ff01;
pub const D40_DREG_GCC_EVTGRP_POS: c_int = 8;
pub const D40_DREG_GCC_SRC: c_int = 0;
pub const D40_DREG_GCC_DST: c_int = 1;

pub const D40_DREG_PRTYP: c_uint = 0x004;
pub const D40_DREG_PRSME: c_uint = 0x008;
pub const D40_DREG_PRSMO: c_uint = 0x00C;
pub const D40_DREG_PRMSE: c_uint = 0x010;
pub const D40_DREG_PRMSO: c_uint = 0x014;
pub const D40_DREG_PRMOE: c_uint = 0x018;
pub const D40_DREG_PRMOO: c_uint = 0x01C;
pub const D40_DREG_PRMO_PCHAN_BASIC: c_uint = 0x1;
pub const D40_DREG_PRMO_PCHAN_MODULO: c_uint = 0x2;
pub const D40_DREG_PRMO_PCHAN_DOUBLE_DST: c_uint = 0x3;
pub const D40_DREG_PRMO_LCHAN_SRC_PHY_DST_LOG: c_uint = 0x1;
pub const D40_DREG_PRMO_LCHAN_SRC_LOG_DST_PHY: c_uint = 0x2;
pub const D40_DREG_PRMO_LCHAN_SRC_LOG_DST_LOG: c_uint = 0x3;
pub const D40_DREG_LCPA: c_uint = 0x020;
pub const D40_DREG_LCLA: c_uint = 0x024;
pub const D40_DREG_SSEG1: c_uint = 0x030;
pub const D40_DREG_SSEG2: c_uint = 0x034;
pub const D40_DREG_SSEG3: c_uint = 0x038;
pub const D40_DREG_SSEG4: c_uint = 0x03C;
pub const D40_DREG_SCEG1: c_uint = 0x040;
pub const D40_DREG_SCEG2: c_uint = 0x044;
pub const D40_DREG_SCEG3: c_uint = 0x048;
pub const D40_DREG_SCEG4: c_uint = 0x04C;
pub const D40_DREG_ACTIVE: c_uint = 0x050;
pub const D40_DREG_ACTIVO: c_uint = 0x054;
pub const D40_DREG_CIDMOD: c_uint = 0x058;
pub const D40_DREG_TCIDV: c_uint = 0x05C;
pub const D40_DREG_PCMIS: c_uint = 0x060;
pub const D40_DREG_PCICR: c_uint = 0x064;
pub const D40_DREG_PCTIS: c_uint = 0x068;
pub const D40_DREG_PCEIS: c_uint = 0x06C;
pub const D40_DREG_SPCMIS: c_uint = 0x070;
pub const D40_DREG_SPCICR: c_uint = 0x074;
pub const D40_DREG_SPCTIS: c_uint = 0x078;
pub const D40_DREG_SPCEIS: c_uint = 0x07C;
pub const D40_DREG_LCMIS0: c_uint = 0x080;
pub const D40_DREG_LCMIS1: c_uint = 0x084;
pub const D40_DREG_LCMIS2: c_uint = 0x088;
pub const D40_DREG_LCMIS3: c_uint = 0x08C;
pub const D40_DREG_LCICR0: c_uint = 0x090;
pub const D40_DREG_LCICR1: c_uint = 0x094;
pub const D40_DREG_LCICR2: c_uint = 0x098;
pub const D40_DREG_LCICR3: c_uint = 0x09C;
pub const D40_DREG_LCTIS0: c_uint = 0x0A0;
pub const D40_DREG_LCTIS1: c_uint = 0x0A4;
pub const D40_DREG_LCTIS2: c_uint = 0x0A8;
pub const D40_DREG_LCTIS3: c_uint = 0x0AC;
pub const D40_DREG_LCEIS0: c_uint = 0x0B0;
pub const D40_DREG_LCEIS1: c_uint = 0x0B4;
pub const D40_DREG_LCEIS2: c_uint = 0x0B8;
pub const D40_DREG_LCEIS3: c_uint = 0x0BC;
pub const D40_DREG_SLCMIS1: c_uint = 0x0C0;
pub const D40_DREG_SLCMIS2: c_uint = 0x0C4;
pub const D40_DREG_SLCMIS3: c_uint = 0x0C8;
pub const D40_DREG_SLCMIS4: c_uint = 0x0CC;
pub const D40_DREG_SLCICR1: c_uint = 0x0D0;
pub const D40_DREG_SLCICR2: c_uint = 0x0D4;
pub const D40_DREG_SLCICR3: c_uint = 0x0D8;
pub const D40_DREG_SLCICR4: c_uint = 0x0DC;
pub const D40_DREG_SLCTIS1: c_uint = 0x0E0;
pub const D40_DREG_SLCTIS2: c_uint = 0x0E4;
pub const D40_DREG_SLCTIS3: c_uint = 0x0E8;
pub const D40_DREG_SLCTIS4: c_uint = 0x0EC;
pub const D40_DREG_SLCEIS1: c_uint = 0x0F0;
pub const D40_DREG_SLCEIS2: c_uint = 0x0F4;
pub const D40_DREG_SLCEIS3: c_uint = 0x0F8;
pub const D40_DREG_SLCEIS4: c_uint = 0x0FC;
pub const D40_DREG_FSESS1: c_uint = 0x100;
pub const D40_DREG_FSESS2: c_uint = 0x104;
pub const D40_DREG_FSEBS1: c_uint = 0x108;
pub const D40_DREG_FSEBS2: c_uint = 0x10C;
pub const D40_DREG_PSEG1: c_uint = 0x110;
pub const D40_DREG_PSEG2: c_uint = 0x114;
pub const D40_DREG_PSEG3: c_uint = 0x118;
pub const D40_DREG_PSEG4: c_uint = 0x11C;
pub const D40_DREG_PCEG1: c_uint = 0x120;
pub const D40_DREG_PCEG2: c_uint = 0x124;
pub const D40_DREG_PCEG3: c_uint = 0x128;
pub const D40_DREG_PCEG4: c_uint = 0x12C;
pub const D40_DREG_RSEG1: c_uint = 0x130;
pub const D40_DREG_RSEG2: c_uint = 0x134;
pub const D40_DREG_RSEG3: c_uint = 0x138;
pub const D40_DREG_RSEG4: c_uint = 0x13C;
pub const D40_DREG_RCEG1: c_uint = 0x140;
pub const D40_DREG_RCEG2: c_uint = 0x144;
pub const D40_DREG_RCEG3: c_uint = 0x148;
pub const D40_DREG_RCEG4: c_uint = 0x14C;
pub const D40_DREG_PREFOT: c_uint = 0x15C;
pub const D40_DREG_EXTCFG: c_uint = 0x160;
pub const D40_DREG_CPSEG1: c_uint = 0x200;
pub const D40_DREG_CPSEG2: c_uint = 0x204;
pub const D40_DREG_CPSEG3: c_uint = 0x208;
pub const D40_DREG_CPSEG4: c_uint = 0x20C;
pub const D40_DREG_CPSEG5: c_uint = 0x210;
pub const D40_DREG_CPCEG1: c_uint = 0x220;
pub const D40_DREG_CPCEG2: c_uint = 0x224;
pub const D40_DREG_CPCEG3: c_uint = 0x228;
pub const D40_DREG_CPCEG4: c_uint = 0x22C;
pub const D40_DREG_CPCEG5: c_uint = 0x230;
pub const D40_DREG_CRSEG1: c_uint = 0x240;
pub const D40_DREG_CRSEG2: c_uint = 0x244;
pub const D40_DREG_CRSEG3: c_uint = 0x248;
pub const D40_DREG_CRSEG4: c_uint = 0x24C;
pub const D40_DREG_CRSEG5: c_uint = 0x250;
pub const D40_DREG_CRCEG1: c_uint = 0x260;
pub const D40_DREG_CRCEG2: c_uint = 0x264;
pub const D40_DREG_CRCEG3: c_uint = 0x268;
pub const D40_DREG_CRCEG4: c_uint = 0x26C;
pub const D40_DREG_CRCEG5: c_uint = 0x270;
pub const D40_DREG_CFSESS1: c_uint = 0x280;
pub const D40_DREG_CFSESS2: c_uint = 0x284;
pub const D40_DREG_CFSESS3: c_uint = 0x288;
pub const D40_DREG_CFSEBS1: c_uint = 0x290;
pub const D40_DREG_CFSEBS2: c_uint = 0x294;
pub const D40_DREG_CFSEBS3: c_uint = 0x298;
pub const D40_DREG_CLCMIS1: c_uint = 0x300;
pub const D40_DREG_CLCMIS2: c_uint = 0x304;
pub const D40_DREG_CLCMIS3: c_uint = 0x308;
pub const D40_DREG_CLCMIS4: c_uint = 0x30C;
pub const D40_DREG_CLCMIS5: c_uint = 0x310;
pub const D40_DREG_CLCICR1: c_uint = 0x320;
pub const D40_DREG_CLCICR2: c_uint = 0x324;
pub const D40_DREG_CLCICR3: c_uint = 0x328;
pub const D40_DREG_CLCICR4: c_uint = 0x32C;
pub const D40_DREG_CLCICR5: c_uint = 0x330;
pub const D40_DREG_CLCTIS1: c_uint = 0x340;
pub const D40_DREG_CLCTIS2: c_uint = 0x344;
pub const D40_DREG_CLCTIS3: c_uint = 0x348;
pub const D40_DREG_CLCTIS4: c_uint = 0x34C;
pub const D40_DREG_CLCTIS5: c_uint = 0x350;
pub const D40_DREG_CLCEIS1: c_uint = 0x360;
pub const D40_DREG_CLCEIS2: c_uint = 0x364;
pub const D40_DREG_CLCEIS3: c_uint = 0x368;
pub const D40_DREG_CLCEIS4: c_uint = 0x36C;
pub const D40_DREG_CLCEIS5: c_uint = 0x370;
pub const D40_DREG_CPCMIS: c_uint = 0x380;
pub const D40_DREG_CPCICR: c_uint = 0x384;
pub const D40_DREG_CPCTIS: c_uint = 0x388;
pub const D40_DREG_CPCEIS: c_uint = 0x38C;
pub const D40_DREG_SCCIDA1: c_uint = 0xE80;
pub const D40_DREG_SCCIDA2: c_uint = 0xE90;
pub const D40_DREG_SCCIDA3: c_uint = 0xEA0;
pub const D40_DREG_SCCIDA4: c_uint = 0xEB0;
pub const D40_DREG_SCCIDA5: c_uint = 0xEC0;
pub const D40_DREG_SCCIDB1: c_uint = 0xE84;
pub const D40_DREG_SCCIDB2: c_uint = 0xE94;
pub const D40_DREG_SCCIDB3: c_uint = 0xEA4;
pub const D40_DREG_SCCIDB4: c_uint = 0xEB4;
pub const D40_DREG_SCCIDB5: c_uint = 0xEC4;
pub const D40_DREG_PRSCCIDA: c_uint = 0xF80;
pub const D40_DREG_PRSCCIDB: c_uint = 0xF84;
pub const D40_DREG_STFU: c_uint = 0xFC8;
pub const D40_DREG_ICFG: c_uint = 0xFCC;
pub const D40_DREG_PERIPHID0: c_uint = 0xFE0;
pub const D40_DREG_PERIPHID1: c_uint = 0xFE4;
pub const D40_DREG_PERIPHID2: c_uint = 0xFE8;
pub const D40_DREG_PERIPHID3: c_uint = 0xFEC;
pub const D40_DREG_CELLID0: c_uint = 0xFF0;
pub const D40_DREG_CELLID1: c_uint = 0xFF4;
pub const D40_DREG_CELLID2: c_uint = 0xFF8;
pub const D40_DREG_CELLID3: c_uint = 0xFFC;
// LLI related structures
//
// struct d40_phy_lli - The basic configuration register for each physical
// channel.
//
// @reg_cfg: The configuration register.
// @reg_elt: The element register.
// @reg_ptr: The pointer register.
// @reg_lnk: The link register.
//
// These registers are set up for both physical and logical transfers
// Note that the bit in each register means differently in logical and
// physical(standard) mode.
//
// This struct must be 16 bytes aligned, and only contain physical registers
// since it will be directly accessed by the DMA.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct d40_phy_lli {
    pub reg_cfg: u32,
    pub reg_elt: u32,
    pub reg_ptr: u32,
    pub reg_lnk: u32,
}

//
// struct d40_phy_lli_bidir - struct for a transfer.
//
// @src: Register settings for src channel.
// @dst: Register settings for dst channel.
//
// All DMA transfers have a source and a destination.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct d40_phy_lli_bidir {
    pub src: *mut d40_phy_lli,
    pub dst: *mut d40_phy_lli,
}

//
// struct d40_log_lli - logical lli configuration
//
// @lcsp02: Either maps to register lcsp0 if src or lcsp2 if dst.
// @lcsp13: Either maps to register lcsp1 if src or lcsp3 if dst.
//
// This struct must be 8 bytes aligned since it will be accessed directly by
// the DMA. Never add any none hw mapped registers to this struct.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct d40_log_lli {
    pub lcsp02: u32,
    pub lcsp13: u32,
}

//
// struct d40_log_lli_bidir - For both src and dst
//
// @src: pointer to src lli configuration.
// @dst: pointer to dst lli configuration.
//
// You always have a src and a dst when doing DMA transfers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct d40_log_lli_bidir {
    pub src: *mut d40_log_lli,
    pub dst: *mut d40_log_lli,
}

//
// struct d40_log_lli_full - LCPA layout
//
// @lcsp0: Logical Channel Standard Param 0 - Src.
// @lcsp1: Logical Channel Standard Param 1 - Src.
// @lcsp2: Logical Channel Standard Param 2 - Dst.
// @lcsp3: Logical Channel Standard Param 3 - Dst.
//
// This struct maps to LCPA physical memory layout. Must map to
// the hw.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct d40_log_lli_full {
    pub lcsp0: u32,
    pub lcsp1: u32,
    pub lcsp2: u32,
    pub lcsp3: u32,
}

//
// struct d40_def_lcsp - Default LCSP1 and LCSP3 settings
//
// @lcsp3: The default configuration for dst.
// @lcsp1: The default configuration for src.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct d40_def_lcsp {
    pub lcsp3: u32,
    pub lcsp1: u32,
}

// Physical channels
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum d40_lli_flags {
    LLI_ADDR_INC	= 1 << 0,
    LLI_TERM_INT	= 1 << 1,
    LLI_CYCLIC	= 1 << 2,
    LLI_LAST_LINK	= 1 << 3,
}

// Logical channels
