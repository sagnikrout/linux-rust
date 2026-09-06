//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/pasemi_dma.h
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
// Copyright (C) 2006-2008 PA Semi, Inc
//
// Hardware register layout and descriptor formats for the on-board
// DMA engine on PA Semi PWRficient. Used by ethernet, function and security
// drivers.
//
// status register layout in IOB region, at 0xfb800000
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pasdma_status {
    pub /: *mut *mut u64 rx_sta[64]; / RX channel status,
    pub /: *mut *mut u64 tx_sta[20]; / TX channel status,
}

// All these registers live in the PCI configuration space for the DMA PCI
// device. Use the normal PCI config access functions for them.
//
pub const PAS_DMA_CAP_TXCH_TCHN_M: c_uint = 0x00ff0000 /* # of TX channels */;
pub const PAS_DMA_CAP_TXCH_TCHN_S: c_int = 16;
pub const PAS_DMA_CAP_RXCH_RCHN_M: c_uint = 0x00ff0000 /* # of RX channels */;
pub const PAS_DMA_CAP_RXCH_RCHN_S: c_int = 16;
pub const PAS_DMA_CAP_IFI_IOFF_M: c_uint = 0xff000000 /* Cfg reg for intf pointers */;
pub const PAS_DMA_CAP_IFI_IOFF_S: c_int = 24;
pub const PAS_DMA_CAP_IFI_NIN_M: c_uint = 0x00ff0000 /* # of interfaces */;
pub const PAS_DMA_CAP_IFI_NIN_S: c_int = 16;
pub const PAS_DMA_COM_TXCMD_EN: c_uint = 0x00000001 /* enable */;
pub const PAS_DMA_COM_TXSTA_ACT: c_uint = 0x00000001 /* active */;
pub const PAS_DMA_COM_RXCMD_EN: c_uint = 0x00000001 /* enable */;
pub const PAS_DMA_COM_RXSTA_ACT: c_uint = 0x00000001 /* active */;
// Per-interface and per-channel registers
pub const _PAS_DMA_RXINT_STRIDE: c_uint = 0x20;

pub const PAS_DMA_RXINT_RCMDSTA_EN: c_uint = 0x00000001;
pub const PAS_DMA_RXINT_RCMDSTA_ST: c_uint = 0x00000002;
pub const PAS_DMA_RXINT_RCMDSTA_MBT: c_uint = 0x00000008;
pub const PAS_DMA_RXINT_RCMDSTA_MDR: c_uint = 0x00000010;
pub const PAS_DMA_RXINT_RCMDSTA_MOO: c_uint = 0x00000020;
pub const PAS_DMA_RXINT_RCMDSTA_MBP: c_uint = 0x00000040;
pub const PAS_DMA_RXINT_RCMDSTA_BT: c_uint = 0x00000800;
pub const PAS_DMA_RXINT_RCMDSTA_DR: c_uint = 0x00001000;
pub const PAS_DMA_RXINT_RCMDSTA_OO: c_uint = 0x00002000;
pub const PAS_DMA_RXINT_RCMDSTA_BP: c_uint = 0x00004000;
pub const PAS_DMA_RXINT_RCMDSTA_TB: c_uint = 0x00008000;
pub const PAS_DMA_RXINT_RCMDSTA_ACT: c_uint = 0x00010000;
pub const PAS_DMA_RXINT_RCMDSTA_DROPS_M: c_uint = 0xfffe0000;
pub const PAS_DMA_RXINT_RCMDSTA_DROPS_S: c_int = 17;

pub const PAS_DMA_RXINT_CFG_RBP: c_uint = 0x80000000;
pub const PAS_DMA_RXINT_CFG_ITRR: c_uint = 0x40000000;
pub const PAS_DMA_RXINT_CFG_DHL_M: c_uint = 0x07000000;
pub const PAS_DMA_RXINT_CFG_DHL_S: c_int = 24;

pub const PAS_DMA_RXINT_CFG_ITR: c_uint = 0x00400000;
pub const PAS_DMA_RXINT_CFG_LW: c_uint = 0x00200000;
pub const PAS_DMA_RXINT_CFG_L2: c_uint = 0x00100000;
pub const PAS_DMA_RXINT_CFG_HEN: c_uint = 0x00080000;
pub const PAS_DMA_RXINT_CFG_WIF: c_uint = 0x00000002;
pub const PAS_DMA_RXINT_CFG_WIL: c_uint = 0x00000001;

pub const PAS_DMA_RXINT_INCR_INCR_M: c_uint = 0x0000ffff;
pub const PAS_DMA_RXINT_INCR_INCR_S: c_int = 0;

pub const PAS_DMA_RXINT_BASEU_SIZ_M: c_uint = 0x3fff0000	/* # of cache lines worth of buffer ring */;

pub const _PAS_DMA_TXCHAN_STRIDE: c_uint = 0x20    /* Size per channel		*/;
pub const _PAS_DMA_TXCHAN_TCMDSTA: c_uint = 0x300	/* Command / Status		*/;
pub const _PAS_DMA_TXCHAN_CFG: c_uint = 0x304	/* Configuration		*/;
pub const _PAS_DMA_TXCHAN_DSCRBU: c_uint = 0x308	/* Descriptor BU Allocation	*/;
pub const _PAS_DMA_TXCHAN_INCR: c_uint = 0x310	/* Descriptor increment		*/;
pub const _PAS_DMA_TXCHAN_CNT: c_uint = 0x314	/* Descriptor count/offset	*/;
pub const _PAS_DMA_TXCHAN_BASEL: c_uint = 0x318	/* Descriptor ring base (low)	*/;
pub const _PAS_DMA_TXCHAN_BASEU: c_uint = 0x31c	/*			(high)	*/;

pub const PAS_DMA_TXCHAN_TCMDSTA_EN: c_uint = 0x00000001	/* Enabled */;
pub const PAS_DMA_TXCHAN_TCMDSTA_ST: c_uint = 0x00000002	/* Stop interface */;
pub const PAS_DMA_TXCHAN_TCMDSTA_ACT: c_uint = 0x00010000	/* Active */;
pub const PAS_DMA_TXCHAN_TCMDSTA_SZ: c_uint = 0x00000800;
pub const PAS_DMA_TXCHAN_TCMDSTA_DB: c_uint = 0x00000400;
pub const PAS_DMA_TXCHAN_TCMDSTA_DE: c_uint = 0x00000200;
pub const PAS_DMA_TXCHAN_TCMDSTA_DA: c_uint = 0x00000100;

pub const PAS_DMA_TXCHAN_CFG_TY_IFACE: c_uint = 0x00000000	/* Type = interface */;
pub const PAS_DMA_TXCHAN_CFG_TY_COPY: c_uint = 0x00000001	/* Type = copy only */;
pub const PAS_DMA_TXCHAN_CFG_TY_FUNC: c_uint = 0x00000002	/* Type = function */;
pub const PAS_DMA_TXCHAN_CFG_TY_XOR: c_uint = 0x00000003	/* Type = xor only */;
pub const PAS_DMA_TXCHAN_CFG_TATTR_M: c_uint = 0x0000003c;
pub const PAS_DMA_TXCHAN_CFG_TATTR_S: c_int = 2;

pub const PAS_DMA_TXCHAN_CFG_LPDQ: c_uint = 0x00000800;
pub const PAS_DMA_TXCHAN_CFG_LPSQ: c_uint = 0x00000400;
pub const PAS_DMA_TXCHAN_CFG_WT_M: c_uint = 0x000003c0;
pub const PAS_DMA_TXCHAN_CFG_WT_S: c_int = 6;

pub const PAS_DMA_TXCHAN_CFG_TRD: c_uint = 0x00010000	/* translate data */;
pub const PAS_DMA_TXCHAN_CFG_TRR: c_uint = 0x00008000	/* translate rings */;
pub const PAS_DMA_TXCHAN_CFG_UP: c_uint = 0x00004000	/* update tx descr when sent */;
pub const PAS_DMA_TXCHAN_CFG_CL: c_uint = 0x00002000	/* Clean last line */;
pub const PAS_DMA_TXCHAN_CFG_CF: c_uint = 0x00001000	/* Clean first line */;

pub const PAS_DMA_TXCHAN_BASEL_BRBL_M: c_uint = 0xffffffc0;
pub const PAS_DMA_TXCHAN_BASEL_BRBL_S: c_int = 0;

pub const PAS_DMA_TXCHAN_BASEU_BRBH_M: c_uint = 0x00000fff;
pub const PAS_DMA_TXCHAN_BASEU_BRBH_S: c_int = 0;

// # of cache lines worth of buffer ring
pub const PAS_DMA_TXCHAN_BASEU_SIZ_M: c_uint = 0x3fff0000;

pub const _PAS_DMA_RXCHAN_STRIDE: c_uint = 0x20    /* Size per channel		*/;
pub const _PAS_DMA_RXCHAN_CCMDSTA: c_uint = 0x800	/* Command / Status		*/;
pub const _PAS_DMA_RXCHAN_CFG: c_uint = 0x804	/* Configuration		*/;
pub const _PAS_DMA_RXCHAN_INCR: c_uint = 0x810	/* Descriptor increment		*/;
pub const _PAS_DMA_RXCHAN_CNT: c_uint = 0x814	/* Descriptor count/offset	*/;
pub const _PAS_DMA_RXCHAN_BASEL: c_uint = 0x818	/* Descriptor ring base (low)	*/;
pub const _PAS_DMA_RXCHAN_BASEU: c_uint = 0x81c	/*			(high)	*/;

pub const PAS_DMA_RXCHAN_CCMDSTA_EN: c_uint = 0x00000001	/* Enabled */;
pub const PAS_DMA_RXCHAN_CCMDSTA_ST: c_uint = 0x00000002	/* Stop interface */;
pub const PAS_DMA_RXCHAN_CCMDSTA_ACT: c_uint = 0x00010000	/* Active */;
pub const PAS_DMA_RXCHAN_CCMDSTA_DU: c_uint = 0x00020000;
pub const PAS_DMA_RXCHAN_CCMDSTA_OD: c_uint = 0x00002000;
pub const PAS_DMA_RXCHAN_CCMDSTA_FD: c_uint = 0x00001000;
pub const PAS_DMA_RXCHAN_CCMDSTA_DT: c_uint = 0x00000800;

pub const PAS_DMA_RXCHAN_CFG_CTR: c_uint = 0x00000400;
pub const PAS_DMA_RXCHAN_CFG_HBU_M: c_uint = 0x00000380;
pub const PAS_DMA_RXCHAN_CFG_HBU_S: c_int = 7;

pub const PAS_DMA_RXCHAN_BASEL_BRBL_M: c_uint = 0xffffffc0;
pub const PAS_DMA_RXCHAN_BASEL_BRBL_S: c_int = 0;

pub const PAS_DMA_RXCHAN_BASEU_BRBH_M: c_uint = 0x00000fff;
pub const PAS_DMA_RXCHAN_BASEU_BRBH_S: c_int = 0;

// # of cache lines worth of buffer ring
pub const PAS_DMA_RXCHAN_BASEU_SIZ_M: c_uint = 0x3fff0000;

pub const PAS_STATUS_PCNT_M: c_uint = 0x000000000000ffffull;
pub const PAS_STATUS_PCNT_S: c_int = 0;
pub const PAS_STATUS_DCNT_M: c_uint = 0x00000000ffff0000ull;
pub const PAS_STATUS_DCNT_S: c_int = 16;
pub const PAS_STATUS_BPCNT_M: c_uint = 0x0000ffff00000000ull;
pub const PAS_STATUS_BPCNT_S: c_int = 32;
pub const PAS_STATUS_CAUSE_M: c_uint = 0xf000000000000000ull;
pub const PAS_STATUS_TIMER: c_uint = 0x1000000000000000ull;
pub const PAS_STATUS_ERROR: c_uint = 0x2000000000000000ull;
pub const PAS_STATUS_SOFT: c_uint = 0x4000000000000000ull;
pub const PAS_STATUS_INT: c_uint = 0x8000000000000000ull;
pub const PAS_IOB_COM_PKTHDRCNT: c_uint = 0x120;
pub const PAS_IOB_COM_PKTHDRCNT_PKTHDR1_M: c_uint = 0x0fff0000;
pub const PAS_IOB_COM_PKTHDRCNT_PKTHDR1_S: c_int = 16;
pub const PAS_IOB_COM_PKTHDRCNT_PKTHDR0_M: c_uint = 0x00000fff;
pub const PAS_IOB_COM_PKTHDRCNT_PKTHDR0_S: c_int = 0;

pub const PAS_IOB_DMA_RXCH_CFG_CNTTH_M: c_uint = 0x00000fff;
pub const PAS_IOB_DMA_RXCH_CFG_CNTTH_S: c_int = 0;

pub const PAS_IOB_DMA_TXCH_CFG_CNTTH_M: c_uint = 0x00000fff;
pub const PAS_IOB_DMA_TXCH_CFG_CNTTH_S: c_int = 0;

pub const PAS_IOB_DMA_RXCH_STAT_INTGEN: c_uint = 0x00001000;
pub const PAS_IOB_DMA_RXCH_STAT_CNTDEL_M: c_uint = 0x00000fff;
pub const PAS_IOB_DMA_RXCH_STAT_CNTDEL_S: c_int = 0;

pub const PAS_IOB_DMA_TXCH_STAT_INTGEN: c_uint = 0x00001000;
pub const PAS_IOB_DMA_TXCH_STAT_CNTDEL_M: c_uint = 0x00000fff;
pub const PAS_IOB_DMA_TXCH_STAT_CNTDEL_S: c_int = 0;

pub const PAS_IOB_DMA_RXCH_RESET_PCNT_M: c_uint = 0xffff0000;
pub const PAS_IOB_DMA_RXCH_RESET_PCNT_S: c_int = 16;

pub const PAS_IOB_DMA_RXCH_RESET_PCNTRST: c_uint = 0x00000020;
pub const PAS_IOB_DMA_RXCH_RESET_DCNTRST: c_uint = 0x00000010;
pub const PAS_IOB_DMA_RXCH_RESET_TINTC: c_uint = 0x00000008;
pub const PAS_IOB_DMA_RXCH_RESET_DINTC: c_uint = 0x00000004;
pub const PAS_IOB_DMA_RXCH_RESET_SINTC: c_uint = 0x00000002;
pub const PAS_IOB_DMA_RXCH_RESET_PINTC: c_uint = 0x00000001;

pub const PAS_IOB_DMA_TXCH_RESET_PCNT_M: c_uint = 0xffff0000;
pub const PAS_IOB_DMA_TXCH_RESET_PCNT_S: c_int = 16;

pub const PAS_IOB_DMA_TXCH_RESET_PCNTRST: c_uint = 0x00000020;
pub const PAS_IOB_DMA_TXCH_RESET_DCNTRST: c_uint = 0x00000010;
pub const PAS_IOB_DMA_TXCH_RESET_TINTC: c_uint = 0x00000008;
pub const PAS_IOB_DMA_TXCH_RESET_DINTC: c_uint = 0x00000004;
pub const PAS_IOB_DMA_TXCH_RESET_SINTC: c_uint = 0x00000002;
pub const PAS_IOB_DMA_TXCH_RESET_PINTC: c_uint = 0x00000001;
pub const PAS_IOB_DMA_COM_TIMEOUTCFG: c_uint = 0x1700;
pub const PAS_IOB_DMA_COM_TIMEOUTCFG_TCNT_M: c_uint = 0x00ffffff;
pub const PAS_IOB_DMA_COM_TIMEOUTCFG_TCNT_S: c_int = 0;

// Transmit descriptor fields
pub const XCT_MACTX_T: c_uint = 0x8000000000000000ull;
pub const XCT_MACTX_ST: c_uint = 0x4000000000000000ull;
pub const XCT_MACTX_NORES: c_uint = 0x0000000000000000ull;
pub const XCT_MACTX_8BRES: c_uint = 0x1000000000000000ull;
pub const XCT_MACTX_24BRES: c_uint = 0x2000000000000000ull;
pub const XCT_MACTX_40BRES: c_uint = 0x3000000000000000ull;
pub const XCT_MACTX_I: c_uint = 0x0800000000000000ull;
pub const XCT_MACTX_O: c_uint = 0x0400000000000000ull;
pub const XCT_MACTX_E: c_uint = 0x0200000000000000ull;
pub const XCT_MACTX_VLAN_M: c_uint = 0x0180000000000000ull;
pub const XCT_MACTX_VLAN_NOP: c_uint = 0x0000000000000000ull;
pub const XCT_MACTX_VLAN_REMOVE: c_uint = 0x0080000000000000ull;
pub const XCT_MACTX_VLAN_INSERT: c_uint = 0x0100000000000000ull;
pub const XCT_MACTX_VLAN_REPLACE: c_uint = 0x0180000000000000ull;
pub const XCT_MACTX_CRC_M: c_uint = 0x0060000000000000ull;
pub const XCT_MACTX_CRC_NOP: c_uint = 0x0000000000000000ull;
pub const XCT_MACTX_CRC_INSERT: c_uint = 0x0020000000000000ull;
pub const XCT_MACTX_CRC_PAD: c_uint = 0x0040000000000000ull;
pub const XCT_MACTX_CRC_REPLACE: c_uint = 0x0060000000000000ull;
pub const XCT_MACTX_SS: c_uint = 0x0010000000000000ull;
pub const XCT_MACTX_LLEN_M: c_uint = 0x00007fff00000000ull;

pub const XCT_MACTX_IPH_M: c_uint = 0x00000000f8000000ull;

pub const XCT_MACTX_IPO_M: c_uint = 0x0000000007c00000ull;

pub const XCT_MACTX_CSUM_M: c_uint = 0x0000000000000060ull;
pub const XCT_MACTX_CSUM_NOP: c_uint = 0x0000000000000000ull;
pub const XCT_MACTX_CSUM_TCP: c_uint = 0x0000000000000040ull;
pub const XCT_MACTX_CSUM_UDP: c_uint = 0x0000000000000060ull;
pub const XCT_MACTX_V6: c_uint = 0x0000000000000010ull;
pub const XCT_MACTX_C: c_uint = 0x0000000000000004ull;
pub const XCT_MACTX_AL2: c_uint = 0x0000000000000002ull;
// Receive descriptor fields
pub const XCT_MACRX_T: c_uint = 0x8000000000000000ull;
pub const XCT_MACRX_ST: c_uint = 0x4000000000000000ull;
pub const XCT_MACRX_RR_M: c_uint = 0x3000000000000000ull;
pub const XCT_MACRX_RR_NORES: c_uint = 0x0000000000000000ull;
pub const XCT_MACRX_RR_8BRES: c_uint = 0x1000000000000000ull;
pub const XCT_MACRX_O: c_uint = 0x0400000000000000ull;
pub const XCT_MACRX_E: c_uint = 0x0200000000000000ull;
pub const XCT_MACRX_FF: c_uint = 0x0100000000000000ull;
pub const XCT_MACRX_PF: c_uint = 0x0080000000000000ull;
pub const XCT_MACRX_OB: c_uint = 0x0040000000000000ull;
pub const XCT_MACRX_OD: c_uint = 0x0020000000000000ull;
pub const XCT_MACRX_FS: c_uint = 0x0010000000000000ull;
pub const XCT_MACRX_NB_M: c_uint = 0x000fc00000000000ull;

pub const XCT_MACRX_LLEN_M: c_uint = 0x00003fff00000000ull;

pub const XCT_MACRX_CRC: c_uint = 0x0000000080000000ull;
pub const XCT_MACRX_LEN_M: c_uint = 0x0000000060000000ull;
pub const XCT_MACRX_LEN_TOOSHORT: c_uint = 0x0000000020000000ull;
pub const XCT_MACRX_LEN_BELOWMIN: c_uint = 0x0000000040000000ull;
pub const XCT_MACRX_LEN_TRUNC: c_uint = 0x0000000060000000ull;
pub const XCT_MACRX_CAST_M: c_uint = 0x0000000018000000ull;
pub const XCT_MACRX_CAST_UNI: c_uint = 0x0000000000000000ull;
pub const XCT_MACRX_CAST_MULTI: c_uint = 0x0000000008000000ull;
pub const XCT_MACRX_CAST_BROAD: c_uint = 0x0000000010000000ull;
pub const XCT_MACRX_CAST_PAUSE: c_uint = 0x0000000018000000ull;
pub const XCT_MACRX_VLC_M: c_uint = 0x0000000006000000ull;
pub const XCT_MACRX_FM: c_uint = 0x0000000001000000ull;
pub const XCT_MACRX_HTY_M: c_uint = 0x0000000000c00000ull;
pub const XCT_MACRX_HTY_IPV4_OK: c_uint = 0x0000000000000000ull;
pub const XCT_MACRX_HTY_IPV6: c_uint = 0x0000000000400000ull;
pub const XCT_MACRX_HTY_IPV4_BAD: c_uint = 0x0000000000800000ull;
pub const XCT_MACRX_HTY_NONIP: c_uint = 0x0000000000c00000ull;
pub const XCT_MACRX_IPP_M: c_uint = 0x00000000003f0000ull;
pub const XCT_MACRX_IPP_S: c_int = 16;
pub const XCT_MACRX_CSUM_M: c_uint = 0x000000000000ffffull;
pub const XCT_MACRX_CSUM_S: c_int = 0;
pub const XCT_PTR_T: c_uint = 0x8000000000000000ull;
pub const XCT_PTR_LEN_M: c_uint = 0x7ffff00000000000ull;
pub const XCT_PTR_LEN_S: c_int = 44;

pub const XCT_PTR_ADDR_M: c_uint = 0x00000fffffffffffull;
pub const XCT_PTR_ADDR_S: c_int = 0;

// Receive interface 8byte result fields
pub const XCT_RXRES_8B_L4O_M: c_uint = 0xff00000000000000ull;
pub const XCT_RXRES_8B_L4O_S: c_int = 56;
pub const XCT_RXRES_8B_RULE_M: c_uint = 0x00ffff0000000000ull;
pub const XCT_RXRES_8B_RULE_S: c_int = 40;
pub const XCT_RXRES_8B_EVAL_M: c_uint = 0x000000ffff000000ull;
pub const XCT_RXRES_8B_EVAL_S: c_int = 24;
pub const XCT_RXRES_8B_HTYPE_M: c_uint = 0x0000000000f00000ull;
pub const XCT_RXRES_8B_HASH_M: c_uint = 0x00000000000fffffull;
pub const XCT_RXRES_8B_HASH_S: c_int = 0;
// Receive interface buffer fields
pub const XCT_RXB_LEN_M: c_uint = 0x0ffff00000000000ull;
pub const XCT_RXB_LEN_S: c_int = 44;

pub const XCT_RXB_ADDR_M: c_uint = 0x00000fffffffffffull;
pub const XCT_RXB_ADDR_S: c_int = 0;

// Copy descriptor fields
pub const XCT_COPY_T: c_uint = 0x8000000000000000ull;
pub const XCT_COPY_ST: c_uint = 0x4000000000000000ull;
pub const XCT_COPY_RR_M: c_uint = 0x3000000000000000ull;
pub const XCT_COPY_RR_NORES: c_uint = 0x0000000000000000ull;
pub const XCT_COPY_RR_8BRES: c_uint = 0x1000000000000000ull;
pub const XCT_COPY_RR_24BRES: c_uint = 0x2000000000000000ull;
pub const XCT_COPY_RR_40BRES: c_uint = 0x3000000000000000ull;
pub const XCT_COPY_I: c_uint = 0x0800000000000000ull;
pub const XCT_COPY_O: c_uint = 0x0400000000000000ull;
pub const XCT_COPY_E: c_uint = 0x0200000000000000ull;
pub const XCT_COPY_STY_ZERO: c_uint = 0x01c0000000000000ull;
pub const XCT_COPY_DTY_PREF: c_uint = 0x0038000000000000ull;
pub const XCT_COPY_LLEN_M: c_uint = 0x0007ffff00000000ull;
pub const XCT_COPY_LLEN_S: c_int = 32;

pub const XCT_COPY_SE: c_uint = 0x0000000000000001ull;
// Function descriptor fields
pub const XCT_FUN_T: c_uint = 0x8000000000000000ull;
pub const XCT_FUN_ST: c_uint = 0x4000000000000000ull;
pub const XCT_FUN_RR_M: c_uint = 0x3000000000000000ull;
pub const XCT_FUN_RR_NORES: c_uint = 0x0000000000000000ull;
pub const XCT_FUN_RR_8BRES: c_uint = 0x1000000000000000ull;
pub const XCT_FUN_RR_24BRES: c_uint = 0x2000000000000000ull;
pub const XCT_FUN_RR_40BRES: c_uint = 0x3000000000000000ull;
pub const XCT_FUN_I: c_uint = 0x0800000000000000ull;
pub const XCT_FUN_O: c_uint = 0x0400000000000000ull;
pub const XCT_FUN_E: c_uint = 0x0200000000000000ull;
pub const XCT_FUN_FUN_M: c_uint = 0x01c0000000000000ull;
pub const XCT_FUN_FUN_S: c_int = 54;

pub const XCT_FUN_CRM_M: c_uint = 0x0038000000000000ull;
pub const XCT_FUN_CRM_NOP: c_uint = 0x0000000000000000ull;
pub const XCT_FUN_CRM_SIG: c_uint = 0x0008000000000000ull;
pub const XCT_FUN_LLEN_M: c_uint = 0x0007ffff00000000ull;
pub const XCT_FUN_LLEN_S: c_int = 32;

pub const XCT_FUN_SHL_M: c_uint = 0x00000000f8000000ull;
pub const XCT_FUN_SHL_S: c_int = 27;

pub const XCT_FUN_CHL_M: c_uint = 0x0000000007c00000ull;
pub const XCT_FUN_HSZ_M: c_uint = 0x00000000003c0000ull;
pub const XCT_FUN_ALG_M: c_uint = 0x0000000000038000ull;
pub const XCT_FUN_HP: c_uint = 0x0000000000004000ull;
pub const XCT_FUN_BCM_M: c_uint = 0x0000000000003800ull;
pub const XCT_FUN_BCP_M: c_uint = 0x0000000000000600ull;
pub const XCT_FUN_SIG_M: c_uint = 0x00000000000001f0ull;
pub const XCT_FUN_SIG_TCP4: c_uint = 0x0000000000000140ull;
pub const XCT_FUN_SIG_TCP6: c_uint = 0x0000000000000150ull;
pub const XCT_FUN_SIG_UDP4: c_uint = 0x0000000000000160ull;
pub const XCT_FUN_SIG_UDP6: c_uint = 0x0000000000000170ull;
pub const XCT_FUN_A: c_uint = 0x0000000000000008ull;
pub const XCT_FUN_C: c_uint = 0x0000000000000004ull;
pub const XCT_FUN_AL2: c_uint = 0x0000000000000002ull;
pub const XCT_FUN_SE: c_uint = 0x0000000000000001ull;
// Function descriptor 8byte result fields
pub const XCT_FUNRES_8B_CS_M: c_uint = 0x0000ffff00000000ull;
pub const XCT_FUNRES_8B_CS_S: c_int = 32;
pub const XCT_FUNRES_8B_CRC_M: c_uint = 0x00000000ffffffffull;
pub const XCT_FUNRES_8B_CRC_S: c_int = 0;
// Control descriptor fields
pub const CTRL_CMD_T: c_uint = 0x8000000000000000ull;
pub const CTRL_CMD_META_EVT: c_uint = 0x2000000000000000ull;
pub const CTRL_CMD_O: c_uint = 0x0400000000000000ull;
pub const CTRL_CMD_ETYPE_M: c_uint = 0x0038000000000000ull;
pub const CTRL_CMD_ETYPE_EXT: c_uint = 0x0000000000000000ull;
pub const CTRL_CMD_ETYPE_WSET: c_uint = 0x0020000000000000ull;
pub const CTRL_CMD_ETYPE_WCLR: c_uint = 0x0028000000000000ull;
pub const CTRL_CMD_ETYPE_SET: c_uint = 0x0030000000000000ull;
pub const CTRL_CMD_ETYPE_CLR: c_uint = 0x0038000000000000ull;
pub const CTRL_CMD_REG_M: c_uint = 0x000000000000007full;
pub const CTRL_CMD_REG_S: c_int = 0;

// Prototypes for the shared DMA functions in the platform code.
// DMA TX Channel type. Right now only limitations used are event types 0/1,
// for event-triggered DMA transactions.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pasemi_dmachan_type {
    RXCHAN = 0,		/* Any RX chan */
    TXCHAN = 1,		/* Any TX chan */
    TXCHAN_EVT0 = 0x1001,	/* TX chan in event class 0 (chan 0-9) */
    TXCHAN_EVT1 = 0x2001,	/* TX chan in event class 1 (chan 10-19) */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pasemi_dmachan {
    pub /: *mut *mut int chno; / Channel number,
    pub /: *mut *mut pasemi_dmachan_type chan_type; / TX / RX,
    pub /: *mut *mut *mut u64 status; / Ptr to cacheable status,
    pub /: *mut *mut int irq; / IRQ used by channel,
    pub /: *mut *mut unsigned int ring_size; / size of allocated ring,
    pub /: *mut *mut dma_addr_t ring_dma; / DMA address for ring,
    pub /: *mut *mut *mut u64 ring_virt; / Virt address for ring,
    pub /: *mut *mut *mut void priv; / Ptr to start of client struct,
}

// Read/write the different registers in the I/O Bridge, Ethernet
// and DMA Controller
//
extern "C" {
    pub fn pasemi_read_iob_reg(reg: c_uint) -> c_uint;
}
extern "C" {
    pub fn pasemi_write_iob_reg(reg: c_uint, val: c_uint);
}
extern "C" {
    pub fn pasemi_read_mac_reg(intf: c_int, reg: c_uint) -> c_uint;
}
extern "C" {
    pub fn pasemi_write_mac_reg(intf: c_int, reg: c_uint, val: c_uint);
}
extern "C" {
    pub fn pasemi_read_dma_reg(reg: c_uint) -> c_uint;
}
extern "C" {
    pub fn pasemi_write_dma_reg(reg: c_uint, val: c_uint);
}
// Channel management routines
extern "C" {
    pub fn pasemi_dma_free_chan(chan: *mut pasemi_dmachan);
}
extern "C" {
    pub fn pasemi_dma_stop_chan(chan: *const pasemi_dmachan) -> c_int;
}
// Common routines to allocate rings and buffers
extern "C" {
    pub fn pasemi_dma_alloc_ring(chan: *mut pasemi_dmachan, ring_size: c_int) -> c_int;
}
extern "C" {
    pub fn pasemi_dma_free_ring(chan: *mut pasemi_dmachan);
}
// Routines to allocate flags (events) for channel synchronization
extern "C" {
    pub fn pasemi_dma_alloc_flag() -> c_int;
}
extern "C" {
    pub fn pasemi_dma_free_flag(flag: c_int);
}
extern "C" {
    pub fn pasemi_dma_set_flag(flag: c_int);
}
extern "C" {
    pub fn pasemi_dma_clear_flag(flag: c_int);
}
// Routines to allocate function engines
extern "C" {
    pub fn pasemi_dma_alloc_fun() -> c_int;
}
extern "C" {
    pub fn pasemi_dma_free_fun(fun: c_int);
}
// Initialize the library, must be called before any other functions
extern "C" {
    pub fn pasemi_dma_init() -> c_int;
}
