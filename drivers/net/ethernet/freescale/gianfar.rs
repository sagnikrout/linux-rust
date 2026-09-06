//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/freescale/gianfar.h
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
// drivers/net/ethernet/freescale/gianfar.h
//
// Gianfar Ethernet Driver
// Driver for FEC on MPC8540 and TSEC on MPC8540/MPC8560
// Based on 8260_io/fcc_enet.c
//
// Author: Andy Fleming
// Maintainer: Kumar Gala
// Modifier: Sandeep Gopalpet <sandeep.kumar@freescale.com>
//
// Copyright 2002-2009, 2011-2013 Freescale Semiconductor, Inc.
//
// Still left to do:
// -Add support for module parameters
// -Add patch for ethtool phys id
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_flow_spec_container {
    pub fs: ethtool_rx_flow_spec,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_rx_list {
    pub list: list_head,
    pub count: c_uint,
}

// Length for FCB
pub const GMAC_FCB_LEN: c_int = 8;
// Length for TxPAL
pub const GMAC_TXPAL_LEN: c_int = 16;
// Default padding amount
pub const DEFAULT_PADDING: c_int = 2;
// Number of bytes to align the rx bufs to
pub const RXBUF_ALIGNMENT: c_int = 64;

// MAXIMUM NUMBER OF QUEUES SUPPORTED
pub const MAX_TX_QS: c_uint = 0x8;
pub const MAX_RX_QS: c_uint = 0x8;
// MAXIMUM NUMBER OF GROUPS SUPPORTED
pub const MAXGROUPS: c_uint = 0x2;
// These need to be powers of 2 for this driver
pub const DEFAULT_TX_RING_SIZE: c_int = 256;
pub const DEFAULT_RX_RING_SIZE: c_int = 256;
pub const GFAR_RX_BUFF_ALLOC: c_int = 16;
pub const GFAR_RX_MAX_RING_SIZE: c_int = 256;
pub const GFAR_TX_MAX_RING_SIZE: c_int = 256;
pub const FBTHR_SHIFT: c_int = 24;
pub const DEFAULT_RX_LFC_THR: c_int = 16;
pub const DEFAULT_LFC_PTVVAL: c_int = 4;
pub const GFAR_RXB_TRUESIZE: c_int = 2048;

pub const GFAR_JUMBO_FRAME_SIZE: c_int = 9600;
pub const DEFAULT_FIFO_TX_THR: c_uint = 0x100;
pub const DEFAULT_FIFO_TX_STARVE: c_uint = 0x40;
pub const DEFAULT_FIFO_TX_STARVE_OFF: c_uint = 0x80;
// The number of Exact Match registers
pub const GFAR_EM_NUM: c_int = 15;
// Latency of interface clock in nanoseconds
// Interface clock latency , in this case, means the
// time described by a value of 1 in the interrupt
// coalescing registers' time fields.  Since those fields
// refer to the time it takes for 64 clocks to pass, the
// latencies are as such:
// GBIT = 125MHz => 8ns/clock => 8*64 ns / tick
// 100 = 25 MHz => 40ns/clock => 40*64 ns / tick
// 10 = 2.5 MHz => 400ns/clock => 400*64 ns / tick
//
pub const GFAR_GBIT_TIME: c_int = 512;
pub const GFAR_100_TIME: c_int = 2560;
pub const GFAR_10_TIME: c_int = 25600;
pub const DEFAULT_TX_COALESCE: c_int = 1;
pub const DEFAULT_TXCOUNT: c_int = 16;
pub const DEFAULT_TXTIME: c_int = 21;
pub const DEFAULT_RXTIME: c_int = 21;
pub const DEFAULT_RX_COALESCE: c_int = 0;
pub const DEFAULT_RXCOUNT: c_int = 0;
// TBI register addresses
pub const MII_TBICON: c_uint = 0x11;
// TBICON register bit fields
pub const TBICON_CLK_SELECT: c_uint = 0x0020;
// MAC register bits
pub const MACCFG1_SOFT_RESET: c_uint = 0x80000000;
pub const MACCFG1_RESET_RX_MC: c_uint = 0x00080000;
pub const MACCFG1_RESET_TX_MC: c_uint = 0x00040000;
pub const MACCFG1_RESET_RX_FUN: c_uint = 0x00020000;
pub const MACCFG1_RESET_TX_FUN: c_uint = 0x00010000;
pub const MACCFG1_LOOPBACK: c_uint = 0x00000100;
pub const MACCFG1_RX_FLOW: c_uint = 0x00000020;
pub const MACCFG1_TX_FLOW: c_uint = 0x00000010;
pub const MACCFG1_SYNCD_RX_EN: c_uint = 0x00000008;
pub const MACCFG1_RX_EN: c_uint = 0x00000004;
pub const MACCFG1_SYNCD_TX_EN: c_uint = 0x00000002;
pub const MACCFG1_TX_EN: c_uint = 0x00000001;
pub const MACCFG2_INIT_SETTINGS: c_uint = 0x00007205;
pub const MACCFG2_FULL_DUPLEX: c_uint = 0x00000001;
pub const MACCFG2_IF: c_uint = 0x00000300;
pub const MACCFG2_MII: c_uint = 0x00000100;
pub const MACCFG2_GMII: c_uint = 0x00000200;
pub const MACCFG2_HUGEFRAME: c_uint = 0x00000020;
pub const MACCFG2_LENGTHCHECK: c_uint = 0x00000010;
pub const MACCFG2_MPEN: c_uint = 0x00000008;
pub const ECNTRL_FIFM: c_uint = 0x00008000;
pub const ECNTRL_INIT_SETTINGS: c_uint = 0x00001000;
pub const ECNTRL_TBI_MODE: c_uint = 0x00000020;
pub const ECNTRL_REDUCED_MODE: c_uint = 0x00000010;
pub const ECNTRL_R100: c_uint = 0x00000008;
pub const ECNTRL_REDUCED_MII_MODE: c_uint = 0x00000004;
pub const ECNTRL_SGMII_MODE: c_uint = 0x00000002;
pub const MINFLR_INIT_SETTINGS: c_uint = 0x00000040;
// Tqueue control
pub const TQUEUE_EN0: c_uint = 0x00008000;
pub const TQUEUE_EN1: c_uint = 0x00004000;
pub const TQUEUE_EN2: c_uint = 0x00002000;
pub const TQUEUE_EN3: c_uint = 0x00001000;
pub const TQUEUE_EN4: c_uint = 0x00000800;
pub const TQUEUE_EN5: c_uint = 0x00000400;
pub const TQUEUE_EN6: c_uint = 0x00000200;
pub const TQUEUE_EN7: c_uint = 0x00000100;
pub const TQUEUE_EN_ALL: c_uint = 0x0000FF00;
pub const TR03WT_WT0_MASK: c_uint = 0xFF000000;
pub const TR03WT_WT1_MASK: c_uint = 0x00FF0000;
pub const TR03WT_WT2_MASK: c_uint = 0x0000FF00;
pub const TR03WT_WT3_MASK: c_uint = 0x000000FF;
pub const TR47WT_WT4_MASK: c_uint = 0xFF000000;
pub const TR47WT_WT5_MASK: c_uint = 0x00FF0000;
pub const TR47WT_WT6_MASK: c_uint = 0x0000FF00;
pub const TR47WT_WT7_MASK: c_uint = 0x000000FF;
// Rqueue control
pub const RQUEUE_EX0: c_uint = 0x00800000;
pub const RQUEUE_EX1: c_uint = 0x00400000;
pub const RQUEUE_EX2: c_uint = 0x00200000;
pub const RQUEUE_EX3: c_uint = 0x00100000;
pub const RQUEUE_EX4: c_uint = 0x00080000;
pub const RQUEUE_EX5: c_uint = 0x00040000;
pub const RQUEUE_EX6: c_uint = 0x00020000;
pub const RQUEUE_EX7: c_uint = 0x00010000;
pub const RQUEUE_EX_ALL: c_uint = 0x00FF0000;
pub const RQUEUE_EN0: c_uint = 0x00000080;
pub const RQUEUE_EN1: c_uint = 0x00000040;
pub const RQUEUE_EN2: c_uint = 0x00000020;
pub const RQUEUE_EN3: c_uint = 0x00000010;
pub const RQUEUE_EN4: c_uint = 0x00000008;
pub const RQUEUE_EN5: c_uint = 0x00000004;
pub const RQUEUE_EN6: c_uint = 0x00000002;
pub const RQUEUE_EN7: c_uint = 0x00000001;
pub const RQUEUE_EN_ALL: c_uint = 0x000000FF;
// Init to do tx snooping for buffers and descriptors
pub const DMACTRL_INIT_SETTINGS: c_uint = 0x000000c3;
pub const DMACTRL_GRS: c_uint = 0x00000010;
pub const DMACTRL_GTS: c_uint = 0x00000008;
pub const TSTAT_CLEAR_THALT_ALL: c_uint = 0xFF000000;
pub const TSTAT_CLEAR_THALT: c_uint = 0x80000000;
pub const TSTAT_CLEAR_THALT0: c_uint = 0x80000000;
pub const TSTAT_CLEAR_THALT1: c_uint = 0x40000000;
pub const TSTAT_CLEAR_THALT2: c_uint = 0x20000000;
pub const TSTAT_CLEAR_THALT3: c_uint = 0x10000000;
pub const TSTAT_CLEAR_THALT4: c_uint = 0x08000000;
pub const TSTAT_CLEAR_THALT5: c_uint = 0x04000000;
pub const TSTAT_CLEAR_THALT6: c_uint = 0x02000000;
pub const TSTAT_CLEAR_THALT7: c_uint = 0x01000000;
// Interrupt coalescing macros
pub const IC_ICEN: c_uint = 0x80000000;
pub const IC_ICFT_MASK: c_uint = 0x1fe00000;
pub const IC_ICFT_SHIFT: c_int = 21;

pub const IC_ICTT_MASK: c_uint = 0x0000ffff;

pub const RCTRL_TS_ENABLE: c_uint = 0x01000000;
pub const RCTRL_PAL_MASK: c_uint = 0x001f0000;
pub const RCTRL_LFC: c_uint = 0x00004000;
pub const RCTRL_VLEX: c_uint = 0x00002000;
pub const RCTRL_FILREN: c_uint = 0x00001000;
pub const RCTRL_GHTX: c_uint = 0x00000400;
pub const RCTRL_IPCSEN: c_uint = 0x00000200;
pub const RCTRL_TUCSEN: c_uint = 0x00000100;
pub const RCTRL_PRSDEP_MASK: c_uint = 0x000000c0;
pub const RCTRL_PRSDEP_INIT: c_uint = 0x000000c0;
pub const RCTRL_PRSFM: c_uint = 0x00000020;
pub const RCTRL_PROM: c_uint = 0x00000008;
pub const RCTRL_EMEN: c_uint = 0x00000002;

pub const RSTAT_CLEAR_RHALT: c_uint = 0x00800000;
pub const RSTAT_CLEAR_RXF0: c_uint = 0x00000080;
pub const RSTAT_RXF_MASK: c_uint = 0x000000ff;
pub const TCTRL_IPCSEN: c_uint = 0x00004000;
pub const TCTRL_TUCSEN: c_uint = 0x00002000;
pub const TCTRL_VLINS: c_uint = 0x00001000;
pub const TCTRL_THDF: c_uint = 0x00000800;
pub const TCTRL_RFCPAUSE: c_uint = 0x00000010;
pub const TCTRL_TFCPAUSE: c_uint = 0x00000008;
pub const TCTRL_TXSCHED_MASK: c_uint = 0x00000006;
pub const TCTRL_TXSCHED_INIT: c_uint = 0x00000000;
// priority scheduling
pub const TCTRL_TXSCHED_PRIO: c_uint = 0x00000002;
// weighted round-robin scheduling (WRRS)
pub const TCTRL_TXSCHED_WRRS: c_uint = 0x00000004;
// default WRRS weight and policy setting,
// tailored to the tr03wt and tr47wt registers:
// equal weight for all Tx Qs, measured in 64byte units
//
pub const DEFAULT_WRRS_WEIGHT: c_uint = 0x18181818;

pub const IEVENT_INIT_CLEAR: c_uint = 0xffffffff;
pub const IEVENT_BABR: c_uint = 0x80000000;
pub const IEVENT_RXC: c_uint = 0x40000000;
pub const IEVENT_BSY: c_uint = 0x20000000;
pub const IEVENT_EBERR: c_uint = 0x10000000;
pub const IEVENT_MSRO: c_uint = 0x04000000;
pub const IEVENT_GTSC: c_uint = 0x02000000;
pub const IEVENT_BABT: c_uint = 0x01000000;
pub const IEVENT_TXC: c_uint = 0x00800000;
pub const IEVENT_TXE: c_uint = 0x00400000;
pub const IEVENT_TXB: c_uint = 0x00200000;
pub const IEVENT_TXF: c_uint = 0x00100000;
pub const IEVENT_LC: c_uint = 0x00040000;
pub const IEVENT_CRL: c_uint = 0x00020000;
pub const IEVENT_XFUN: c_uint = 0x00010000;
pub const IEVENT_RXB0: c_uint = 0x00008000;
pub const IEVENT_MAG: c_uint = 0x00000800;
pub const IEVENT_GRSC: c_uint = 0x00000100;
pub const IEVENT_RXF0: c_uint = 0x00000080;
pub const IEVENT_FGPI: c_uint = 0x00000010;
pub const IEVENT_FIR: c_uint = 0x00000008;
pub const IEVENT_FIQ: c_uint = 0x00000004;
pub const IEVENT_DPE: c_uint = 0x00000002;
pub const IEVENT_PERR: c_uint = 0x00000001;

pub const IMASK_INIT_CLEAR: c_uint = 0x00000000;
pub const IMASK_BABR: c_uint = 0x80000000;
pub const IMASK_RXC: c_uint = 0x40000000;
pub const IMASK_BSY: c_uint = 0x20000000;
pub const IMASK_EBERR: c_uint = 0x10000000;
pub const IMASK_MSRO: c_uint = 0x04000000;
pub const IMASK_GTSC: c_uint = 0x02000000;
pub const IMASK_BABT: c_uint = 0x01000000;
pub const IMASK_TXC: c_uint = 0x00800000;
pub const IMASK_TXEEN: c_uint = 0x00400000;
pub const IMASK_TXBEN: c_uint = 0x00200000;
pub const IMASK_TXFEN: c_uint = 0x00100000;
pub const IMASK_LC: c_uint = 0x00040000;
pub const IMASK_CRL: c_uint = 0x00020000;
pub const IMASK_XFUN: c_uint = 0x00010000;
pub const IMASK_RXB0: c_uint = 0x00008000;
pub const IMASK_MAG: c_uint = 0x00000800;
pub const IMASK_GRSC: c_uint = 0x00000100;
pub const IMASK_RXFEN0: c_uint = 0x00000080;
pub const IMASK_FGPI: c_uint = 0x00000010;
pub const IMASK_FIR: c_uint = 0x00000008;
pub const IMASK_FIQ: c_uint = 0x00000004;
pub const IMASK_DPE: c_uint = 0x00000002;
pub const IMASK_PERR: c_uint = 0x00000001;

// Attribute fields
// This enables rx snooping for buffers and descriptors
pub const ATTR_BDSTASH: c_uint = 0x00000800;
pub const ATTR_BUFSTASH: c_uint = 0x00004000;
pub const ATTR_SNOOPING: c_uint = 0x000000c0;

pub const ATTRELI_INIT_SETTINGS: c_uint = 0x0;
pub const ATTRELI_EL_MASK: c_uint = 0x3fff0000;

pub const ATTRELI_EI_MASK: c_uint = 0x00003fff;

pub const BD_LENGTH_MASK: c_uint = 0x0000ffff;
pub const FPR_FILER_MASK: c_uint = 0xFFFFFFFF;
pub const MAX_FILER_IDX: c_uint = 0xFF;
// This default RIR value directly corresponds
// to the 3-bit hash value generated
pub const DEFAULT_8RXQ_RIR0: c_uint = 0x05397700;
// Map even hash values to Q0, and odd ones to Q1
pub const DEFAULT_2RXQ_RIR0: c_uint = 0x04104100;
// RQFCR register bits
pub const RQFCR_GPI: c_uint = 0x80000000;
pub const RQFCR_HASHTBL_Q: c_uint = 0x00000000;
pub const RQFCR_HASHTBL_0: c_uint = 0x00020000;
pub const RQFCR_HASHTBL_1: c_uint = 0x00040000;
pub const RQFCR_HASHTBL_2: c_uint = 0x00060000;
pub const RQFCR_HASHTBL_3: c_uint = 0x00080000;
pub const RQFCR_HASH: c_uint = 0x00010000;
pub const RQFCR_QUEUE: c_uint = 0x0000FC00;
pub const RQFCR_CLE: c_uint = 0x00000200;
pub const RQFCR_RJE: c_uint = 0x00000100;
pub const RQFCR_AND: c_uint = 0x00000080;
pub const RQFCR_CMP_EXACT: c_uint = 0x00000000;
pub const RQFCR_CMP_MATCH: c_uint = 0x00000020;
pub const RQFCR_CMP_NOEXACT: c_uint = 0x00000040;
pub const RQFCR_CMP_NOMATCH: c_uint = 0x00000060;
// RQFCR PID values
pub const RQFCR_PID_MASK: c_uint = 0x00000000;
pub const RQFCR_PID_PARSE: c_uint = 0x00000001;
pub const RQFCR_PID_ARB: c_uint = 0x00000002;
pub const RQFCR_PID_DAH: c_uint = 0x00000003;
pub const RQFCR_PID_DAL: c_uint = 0x00000004;
pub const RQFCR_PID_SAH: c_uint = 0x00000005;
pub const RQFCR_PID_SAL: c_uint = 0x00000006;
pub const RQFCR_PID_ETY: c_uint = 0x00000007;
pub const RQFCR_PID_VID: c_uint = 0x00000008;
pub const RQFCR_PID_PRI: c_uint = 0x00000009;
pub const RQFCR_PID_TOS: c_uint = 0x0000000A;
pub const RQFCR_PID_L4P: c_uint = 0x0000000B;
pub const RQFCR_PID_DIA: c_uint = 0x0000000C;
pub const RQFCR_PID_SIA: c_uint = 0x0000000D;
pub const RQFCR_PID_DPT: c_uint = 0x0000000E;
pub const RQFCR_PID_SPT: c_uint = 0x0000000F;
// RQFPR when PID is 0x0001
pub const RQFPR_HDR_GE_512: c_uint = 0x00200000;
pub const RQFPR_LERR: c_uint = 0x00100000;
pub const RQFPR_RAR: c_uint = 0x00080000;
pub const RQFPR_RARQ: c_uint = 0x00040000;
pub const RQFPR_AR: c_uint = 0x00020000;
pub const RQFPR_ARQ: c_uint = 0x00010000;
pub const RQFPR_EBC: c_uint = 0x00008000;
pub const RQFPR_VLN: c_uint = 0x00004000;
pub const RQFPR_CFI: c_uint = 0x00002000;
pub const RQFPR_JUM: c_uint = 0x00001000;
pub const RQFPR_IPF: c_uint = 0x00000800;
pub const RQFPR_FIF: c_uint = 0x00000400;
pub const RQFPR_IPV4: c_uint = 0x00000200;
pub const RQFPR_IPV6: c_uint = 0x00000100;
pub const RQFPR_ICC: c_uint = 0x00000080;
pub const RQFPR_ICV: c_uint = 0x00000040;
pub const RQFPR_TCP: c_uint = 0x00000020;
pub const RQFPR_UDP: c_uint = 0x00000010;
pub const RQFPR_TUC: c_uint = 0x00000008;
pub const RQFPR_TUV: c_uint = 0x00000004;
pub const RQFPR_PER: c_uint = 0x00000002;
pub const RQFPR_EER: c_uint = 0x00000001;
// CAR1 bits
pub const CAR1_C164: c_uint = 0x80000000;
pub const CAR1_C1127: c_uint = 0x40000000;
pub const CAR1_C1255: c_uint = 0x20000000;
pub const CAR1_C1511: c_uint = 0x10000000;
pub const CAR1_C11K: c_uint = 0x08000000;
pub const CAR1_C1MAX: c_uint = 0x04000000;
pub const CAR1_C1MGV: c_uint = 0x02000000;
pub const CAR1_C1REJ: c_uint = 0x00020000;
pub const CAR1_C1RBY: c_uint = 0x00010000;
pub const CAR1_C1RPK: c_uint = 0x00008000;
pub const CAR1_C1RFC: c_uint = 0x00004000;
pub const CAR1_C1RMC: c_uint = 0x00002000;
pub const CAR1_C1RBC: c_uint = 0x00001000;
pub const CAR1_C1RXC: c_uint = 0x00000800;
pub const CAR1_C1RXP: c_uint = 0x00000400;
pub const CAR1_C1RXU: c_uint = 0x00000200;
pub const CAR1_C1RAL: c_uint = 0x00000100;
pub const CAR1_C1RFL: c_uint = 0x00000080;
pub const CAR1_C1RCD: c_uint = 0x00000040;
pub const CAR1_C1RCS: c_uint = 0x00000020;
pub const CAR1_C1RUN: c_uint = 0x00000010;
pub const CAR1_C1ROV: c_uint = 0x00000008;
pub const CAR1_C1RFR: c_uint = 0x00000004;
pub const CAR1_C1RJB: c_uint = 0x00000002;
pub const CAR1_C1RDR: c_uint = 0x00000001;
// CAM1 bits
pub const CAM1_M164: c_uint = 0x80000000;
pub const CAM1_M1127: c_uint = 0x40000000;
pub const CAM1_M1255: c_uint = 0x20000000;
pub const CAM1_M1511: c_uint = 0x10000000;
pub const CAM1_M11K: c_uint = 0x08000000;
pub const CAM1_M1MAX: c_uint = 0x04000000;
pub const CAM1_M1MGV: c_uint = 0x02000000;
pub const CAM1_M1REJ: c_uint = 0x00020000;
pub const CAM1_M1RBY: c_uint = 0x00010000;
pub const CAM1_M1RPK: c_uint = 0x00008000;
pub const CAM1_M1RFC: c_uint = 0x00004000;
pub const CAM1_M1RMC: c_uint = 0x00002000;
pub const CAM1_M1RBC: c_uint = 0x00001000;
pub const CAM1_M1RXC: c_uint = 0x00000800;
pub const CAM1_M1RXP: c_uint = 0x00000400;
pub const CAM1_M1RXU: c_uint = 0x00000200;
pub const CAM1_M1RAL: c_uint = 0x00000100;
pub const CAM1_M1RFL: c_uint = 0x00000080;
pub const CAM1_M1RCD: c_uint = 0x00000040;
pub const CAM1_M1RCS: c_uint = 0x00000020;
pub const CAM1_M1RUN: c_uint = 0x00000010;
pub const CAM1_M1ROV: c_uint = 0x00000008;
pub const CAM1_M1RFR: c_uint = 0x00000004;
pub const CAM1_M1RJB: c_uint = 0x00000002;
pub const CAM1_M1RDR: c_uint = 0x00000001;
// TxBD status field bits
pub const TXBD_READY: c_uint = 0x8000;
pub const TXBD_PADCRC: c_uint = 0x4000;
pub const TXBD_WRAP: c_uint = 0x2000;
pub const TXBD_INTERRUPT: c_uint = 0x1000;
pub const TXBD_LAST: c_uint = 0x0800;
pub const TXBD_CRC: c_uint = 0x0400;
pub const TXBD_DEF: c_uint = 0x0200;
pub const TXBD_HUGEFRAME: c_uint = 0x0080;
pub const TXBD_LATECOLLISION: c_uint = 0x0080;
pub const TXBD_RETRYLIMIT: c_uint = 0x0040;
pub const TXBD_RETRYCOUNTMASK: c_uint = 0x003c;
pub const TXBD_UNDERRUN: c_uint = 0x0002;
pub const TXBD_TOE: c_uint = 0x0002;
// Tx FCB param bits
pub const TXFCB_VLN: c_uint = 0x80;
pub const TXFCB_IP: c_uint = 0x40;
pub const TXFCB_IP6: c_uint = 0x20;
pub const TXFCB_TUP: c_uint = 0x10;
pub const TXFCB_UDP: c_uint = 0x08;
pub const TXFCB_CIP: c_uint = 0x04;
pub const TXFCB_CTU: c_uint = 0x02;
pub const TXFCB_NPH: c_uint = 0x01;

// RxBD status field bits
pub const RXBD_EMPTY: c_uint = 0x8000;
pub const RXBD_RO1: c_uint = 0x4000;
pub const RXBD_WRAP: c_uint = 0x2000;
pub const RXBD_INTERRUPT: c_uint = 0x1000;
pub const RXBD_LAST: c_uint = 0x0800;
pub const RXBD_FIRST: c_uint = 0x0400;
pub const RXBD_MISS: c_uint = 0x0100;
pub const RXBD_BROADCAST: c_uint = 0x0080;
pub const RXBD_MULTICAST: c_uint = 0x0040;
pub const RXBD_LARGE: c_uint = 0x0020;
pub const RXBD_NONOCTET: c_uint = 0x0010;
pub const RXBD_SHORT: c_uint = 0x0008;
pub const RXBD_CRCERR: c_uint = 0x0004;
pub const RXBD_OVERRUN: c_uint = 0x0002;
pub const RXBD_TRUNCATED: c_uint = 0x0001;
pub const RXBD_STATS: c_uint = 0x01ff;

// Rx FCB status field bits
pub const RXFCB_VLN: c_uint = 0x8000;
pub const RXFCB_IP: c_uint = 0x4000;
pub const RXFCB_IP6: c_uint = 0x2000;
pub const RXFCB_TUP: c_uint = 0x1000;
pub const RXFCB_CIP: c_uint = 0x0800;
pub const RXFCB_CTU: c_uint = 0x0400;
pub const RXFCB_EIP: c_uint = 0x0200;
pub const RXFCB_ETU: c_uint = 0x0100;
pub const RXFCB_CSUM_MASK: c_uint = 0x0f00;
pub const RXFCB_PERR_MASK: c_uint = 0x000c;
pub const RXFCB_PERR_BADL3: c_uint = 0x0008;

pub const GFAR_WOL_MAGIC: c_uint = 0x00000001;
pub const GFAR_WOL_FILER_UCAST: c_uint = 0x00000002;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct txfcb {
    pub flags: u8,
    pub /: *mut *mut u8 ptp; / Flag to enable tx timestamping,
    pub /: *mut *mut u8 l4os; / Level 4 Header Offset,
    pub /: *mut *mut u8 l3os; / Level 3 Header Offset,
    pub /: *mut *mut __be16 phcs; / Pseudo-header Checksum,
    pub /: *mut *mut __be16 vlctl; / VLAN control word,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxfcb {
    pub flags: __be16,
    pub /: *mut *mut u8 rq; / Receive Queue index,
    pub /: *mut *mut u8 pro; / Layer 4 Protocol,
    pub reserved: u16,
    pub /: *mut *mut __be16 vlctl; / VLAN control word,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gianfar_skb_cb {
    pub /: *mut *mut unsigned int bytes_sent; / bytes-on-wire (i.e. no FCB),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmon_overflow {
// lock for synchronization of the rdrp field of this struct, and
// CAR1/CAR2 registers
//
    pub lock: spinlock_t,
    pub imask: u32,
    pub rdrp: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfar_extra_stats {
    pub rx_alloc_err: core::sync::atomic::AtomicI64,
    pub rx_large: core::sync::atomic::AtomicI64,
    pub rx_short: core::sync::atomic::AtomicI64,
    pub rx_nonoctet: core::sync::atomic::AtomicI64,
    pub rx_crcerr: core::sync::atomic::AtomicI64,
    pub rx_overrun: core::sync::atomic::AtomicI64,
    pub rx_bsy: core::sync::atomic::AtomicI64,
    pub rx_babr: core::sync::atomic::AtomicI64,
    pub rx_trunc: core::sync::atomic::AtomicI64,
    pub eberr: core::sync::atomic::AtomicI64,
    pub tx_babt: core::sync::atomic::AtomicI64,
    pub tx_underrun: core::sync::atomic::AtomicI64,
    pub tx_timeout: core::sync::atomic::AtomicI64,
}

// Number of stats exported via ethtool

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfar {
    pub /: *mut *mut u32 tsec_id; / 0x.000 - Controller ID register,
    pub /: *mut *mut u32 tsec_id2; / 0x.004 - Controller ID2 register,
    pub res1: [u8; 8],
    pub /: *mut *mut u32 ievent; / 0x.010 - Interrupt Event Register,
    pub /: *mut *mut u32 imask; / 0x.014 - Interrupt Mask Register,
    pub /: *mut *mut u32 edis; / 0x.018 - Error Disabled Register,
    pub /: *mut *mut u32 emapg; / 0x.01c - Group Error mapping register,
    pub /: *mut *mut u32 ecntrl; / 0x.020 - Ethernet Control Register,
    pub /: *mut *mut u32 minflr; / 0x.024 - Minimum Frame Length Register,
    pub /: *mut *mut u32 ptv; / 0x.028 - Pause Time Value Register,
    pub /: *mut *mut u32 dmactrl; / 0x.02c - DMA Control Register,
    pub /: *mut *mut u32 tbipa; / 0x.030 - TBI PHY Address Register,
    pub res2: [u8; 28],
    pub threshold: *mut *mut u32 fifo_rx_pause; / 0x.050 - FIFO receive pause start,
    pub shutoff: *mut *mut u32 fifo_rx_pause_shutoff; / x.054 - FIFO receive starve,
    pub threshold: *mut *mut u32 fifo_rx_alarm; / 0x.058 - FIFO receive alarm start,
    pub starve: *mut *mut u32 fifo_rx_alarm_shutoff; /0x.05c - FIFO receive alarm,
    pub res3: [u8; 44],
    pub /: *mut *mut u32 fifo_tx_thr; / 0x.08c - FIFO transmit threshold register,
    pub res4: [u8; 8],
    pub /: *mut *mut u32 fifo_tx_starve; / 0x.098 - FIFO transmit starve register,
    pub /: *mut *mut u32 fifo_tx_starve_shutoff; / 0x.09c - FIFO transmit starve shutoff register,
    pub res5: [u8; 96],
    pub /: *mut *mut u32 tctrl; / 0x.100 - Transmit Control Register,
    pub /: *mut *mut u32 tstat; / 0x.104 - Transmit Status Register,
    pub /: *mut *mut u32 dfvlan; / 0x.108 - Default VLAN Control word,
    pub /: *mut *mut u32 tbdlen; / 0x.10c - Transmit Buffer Descriptor Data Length Register,
    pub /: *mut *mut u32 txic; / 0x.110 - Transmit Interrupt Coalescing Configuration Register,
    pub /: *mut *mut u32 tqueue; / 0x.114 - Transmit queue control register,
    pub res7: [u8; 40],
    pub /: *mut *mut u32 tr03wt; / 0x.140 - TxBD Rings 0-3 round-robin weightings,
    pub /: *mut *mut u32 tr47wt; / 0x.144 - TxBD Rings 4-7 round-robin weightings,
    pub res8: [u8; 52],
    pub /: *mut *mut u32 tbdbph; / 0x.17c - Tx data buffer pointer high,
    pub res9a: [u8; 4],
    pub /: *mut *mut u32 tbptr0; / 0x.184 - TxBD Pointer for ring 0,
    pub res9b: [u8; 4],
    pub /: *mut *mut u32 tbptr1; / 0x.18c - TxBD Pointer for ring 1,
    pub res9c: [u8; 4],
    pub /: *mut *mut u32 tbptr2; / 0x.194 - TxBD Pointer for ring 2,
    pub res9d: [u8; 4],
    pub /: *mut *mut u32 tbptr3; / 0x.19c - TxBD Pointer for ring 3,
    pub res9e: [u8; 4],
    pub /: *mut *mut u32 tbptr4; / 0x.1a4 - TxBD Pointer for ring 4,
    pub res9f: [u8; 4],
    pub /: *mut *mut u32 tbptr5; / 0x.1ac - TxBD Pointer for ring 5,
    pub res9g: [u8; 4],
    pub /: *mut *mut u32 tbptr6; / 0x.1b4 - TxBD Pointer for ring 6,
    pub res9h: [u8; 4],
    pub /: *mut *mut u32 tbptr7; / 0x.1bc - TxBD Pointer for ring 7,
    pub res9: [u8; 64],
    pub /: *mut *mut u32 tbaseh; / 0x.200 - TxBD base address high,
    pub /: *mut *mut u32 tbase0; / 0x.204 - TxBD Base Address of ring 0,
    pub res10a: [u8; 4],
    pub /: *mut *mut u32 tbase1; / 0x.20c - TxBD Base Address of ring 1,
    pub res10b: [u8; 4],
    pub /: *mut *mut u32 tbase2; / 0x.214 - TxBD Base Address of ring 2,
    pub res10c: [u8; 4],
    pub /: *mut *mut u32 tbase3; / 0x.21c - TxBD Base Address of ring 3,
    pub res10d: [u8; 4],
    pub /: *mut *mut u32 tbase4; / 0x.224 - TxBD Base Address of ring 4,
    pub res10e: [u8; 4],
    pub /: *mut *mut u32 tbase5; / 0x.22c - TxBD Base Address of ring 5,
    pub res10f: [u8; 4],
    pub /: *mut *mut u32 tbase6; / 0x.234 - TxBD Base Address of ring 6,
    pub res10g: [u8; 4],
    pub /: *mut *mut u32 tbase7; / 0x.23c - TxBD Base Address of ring 7,
    pub res10: [u8; 192],
    pub /: *mut *mut u32 rctrl; / 0x.300 - Receive Control Register,
    pub /: *mut *mut u32 rstat; / 0x.304 - Receive Status Register,
    pub res12: [u8; 8],
    pub /: *mut *mut u32 rxic; / 0x.310 - Receive Interrupt Coalescing Configuration Register,
    pub /: *mut *mut u32 rqueue; / 0x.314 - Receive queue control register,
    pub /: *mut *mut u32 rir0; / 0x.318 - Ring mapping register 0,
    pub /: *mut *mut u32 rir1; / 0x.31c - Ring mapping register 1,
    pub /: *mut *mut u32 rir2; / 0x.320 - Ring mapping register 2,
    pub /: *mut *mut u32 rir3; / 0x.324 - Ring mapping register 3,
    pub res13: [u8; 8],
    pub /: *mut *mut u32 rbifx; / 0x.330 - Receive bit field extract control register,
    pub /: *mut *mut u32 rqfar; / 0x.334 - Receive queue filing table address register,
    pub /: *mut *mut u32 rqfcr; / 0x.338 - Receive queue filing table control register,
    pub /: *mut *mut u32 rqfpr; / 0x.33c - Receive queue filing table property register,
    pub /: *mut *mut u32 mrblr; / 0x.340 - Maximum Receive Buffer Length Register,
    pub res14: [u8; 56],
    pub /: *mut *mut u32 rbdbph; / 0x.37c - Rx data buffer pointer high,
    pub res15a: [u8; 4],
    pub /: *mut *mut u32 rbptr0; / 0x.384 - RxBD pointer for ring 0,
    pub res15b: [u8; 4],
    pub /: *mut *mut u32 rbptr1; / 0x.38c - RxBD pointer for ring 1,
    pub res15c: [u8; 4],
    pub /: *mut *mut u32 rbptr2; / 0x.394 - RxBD pointer for ring 2,
    pub res15d: [u8; 4],
    pub /: *mut *mut u32 rbptr3; / 0x.39c - RxBD pointer for ring 3,
    pub res15e: [u8; 4],
    pub /: *mut *mut u32 rbptr4; / 0x.3a4 - RxBD pointer for ring 4,
    pub res15f: [u8; 4],
    pub /: *mut *mut u32 rbptr5; / 0x.3ac - RxBD pointer for ring 5,
    pub res15g: [u8; 4],
    pub /: *mut *mut u32 rbptr6; / 0x.3b4 - RxBD pointer for ring 6,
    pub res15h: [u8; 4],
    pub /: *mut *mut u32 rbptr7; / 0x.3bc - RxBD pointer for ring 7,
    pub res16: [u8; 64],
    pub /: *mut *mut u32 rbaseh; / 0x.400 - RxBD base address high,
    pub /: *mut *mut u32 rbase0; / 0x.404 - RxBD base address of ring 0,
    pub res17a: [u8; 4],
    pub /: *mut *mut u32 rbase1; / 0x.40c - RxBD base address of ring 1,
    pub res17b: [u8; 4],
    pub /: *mut *mut u32 rbase2; / 0x.414 - RxBD base address of ring 2,
    pub res17c: [u8; 4],
    pub /: *mut *mut u32 rbase3; / 0x.41c - RxBD base address of ring 3,
    pub res17d: [u8; 4],
    pub /: *mut *mut u32 rbase4; / 0x.424 - RxBD base address of ring 4,
    pub res17e: [u8; 4],
    pub /: *mut *mut u32 rbase5; / 0x.42c - RxBD base address of ring 5,
    pub res17f: [u8; 4],
    pub /: *mut *mut u32 rbase6; / 0x.434 - RxBD base address of ring 6,
    pub res17g: [u8; 4],
    pub /: *mut *mut u32 rbase7; / 0x.43c - RxBD base address of ring 7,
    pub res17: [u8; 192],
    pub /: *mut *mut u32 maccfg1; / 0x.500 - MAC Configuration 1 Register,
    pub /: *mut *mut u32 maccfg2; / 0x.504 - MAC Configuration 2 Register,
    pub /: *mut *mut u32 ipgifg; / 0x.508 - Inter Packet Gap/Inter Frame Gap Register,
    pub /: *mut *mut u32 hafdup; / 0x.50c - Half Duplex Register,
    pub /: *mut *mut u32 maxfrm; / 0x.510 - Maximum Frame Length Register,
    pub res18: [u8; 12],
    pub /: *mut *mut u8 gfar_mii_regs[24]; / See gianfar_phy.h,
    pub /: *mut *mut u32 ifctrl; / 0x.538 - Interface control register,
    pub /: *mut *mut u32 ifstat; / 0x.53c - Interface Status Register,
    pub /: *mut *mut u32 macstnaddr1; / 0x.540 - Station Address Part 1 Register,
    pub /: *mut *mut u32 macstnaddr2; / 0x.544 - Station Address Part 2 Register,
    pub /: *mut *mut u32 mac01addr1; / 0x.548 - MAC exact match address 1, part 1,
    pub /: *mut *mut u32 mac01addr2; / 0x.54c - MAC exact match address 1, part 2,
    pub /: *mut *mut u32 mac02addr1; / 0x.550 - MAC exact match address 2, part 1,
    pub /: *mut *mut u32 mac02addr2; / 0x.554 - MAC exact match address 2, part 2,
    pub /: *mut *mut u32 mac03addr1; / 0x.558 - MAC exact match address 3, part 1,
    pub /: *mut *mut u32 mac03addr2; / 0x.55c - MAC exact match address 3, part 2,
    pub /: *mut *mut u32 mac04addr1; / 0x.560 - MAC exact match address 4, part 1,
    pub /: *mut *mut u32 mac04addr2; / 0x.564 - MAC exact match address 4, part 2,
    pub /: *mut *mut u32 mac05addr1; / 0x.568 - MAC exact match address 5, part 1,
    pub /: *mut *mut u32 mac05addr2; / 0x.56c - MAC exact match address 5, part 2,
    pub /: *mut *mut u32 mac06addr1; / 0x.570 - MAC exact match address 6, part 1,
    pub /: *mut *mut u32 mac06addr2; / 0x.574 - MAC exact match address 6, part 2,
    pub /: *mut *mut u32 mac07addr1; / 0x.578 - MAC exact match address 7, part 1,
    pub /: *mut *mut u32 mac07addr2; / 0x.57c - MAC exact match address 7, part 2,
    pub /: *mut *mut u32 mac08addr1; / 0x.580 - MAC exact match address 8, part 1,
    pub /: *mut *mut u32 mac08addr2; / 0x.584 - MAC exact match address 8, part 2,
    pub /: *mut *mut u32 mac09addr1; / 0x.588 - MAC exact match address 9, part 1,
    pub /: *mut *mut u32 mac09addr2; / 0x.58c - MAC exact match address 9, part 2,
    pub 1*/: *mut *mut u32 mac10addr1; / 0x.590 - MAC exact match address 10, part,
    pub 2*/: *mut *mut u32 mac10addr2; / 0x.594 - MAC exact match address 10, part,
    pub 1*/: *mut *mut u32 mac11addr1; / 0x.598 - MAC exact match address 11, part,
    pub 2*/: *mut *mut u32 mac11addr2; / 0x.59c - MAC exact match address 11, part,
    pub 1*/: *mut *mut u32 mac12addr1; / 0x.5a0 - MAC exact match address 12, part,
    pub 2*/: *mut *mut u32 mac12addr2; / 0x.5a4 - MAC exact match address 12, part,
    pub 1*/: *mut *mut u32 mac13addr1; / 0x.5a8 - MAC exact match address 13, part,
    pub 2*/: *mut *mut u32 mac13addr2; / 0x.5ac - MAC exact match address 13, part,
    pub 1*/: *mut *mut u32 mac14addr1; / 0x.5b0 - MAC exact match address 14, part,
    pub 2*/: *mut *mut u32 mac14addr2; / 0x.5b4 - MAC exact match address 14, part,
    pub 1*/: *mut *mut u32 mac15addr1; / 0x.5b8 - MAC exact match address 15, part,
    pub 2*/: *mut *mut u32 mac15addr2; / 0x.5bc - MAC exact match address 15, part,
    pub res20: [u8; 192],
    pub /: *mut *mut rmon_mib rmon; / 0x.680-0x.73c,
    pub /: *mut *mut u32 rrej; / 0x.740 - Receive filer rejected packet counter,
    pub res21: [u8; 188],
    pub 0*/: *mut *mut u32 igaddr0; / 0x.800 - Indivdual/Group address register,
    pub 1*/: *mut *mut u32 igaddr1; / 0x.804 - Indivdual/Group address register,
    pub 2*/: *mut *mut u32 igaddr2; / 0x.808 - Indivdual/Group address register,
    pub 3*/: *mut *mut u32 igaddr3; / 0x.80c - Indivdual/Group address register,
    pub 4*/: *mut *mut u32 igaddr4; / 0x.810 - Indivdual/Group address register,
    pub 5*/: *mut *mut u32 igaddr5; / 0x.814 - Indivdual/Group address register,
    pub 6*/: *mut *mut u32 igaddr6; / 0x.818 - Indivdual/Group address register,
    pub 7*/: *mut *mut u32 igaddr7; / 0x.81c - Indivdual/Group address register,
    pub res22: [u8; 96],
    pub /: *mut *mut u32 gaddr0; / 0x.880 - Group address register 0,
    pub /: *mut *mut u32 gaddr1; / 0x.884 - Group address register 1,
    pub /: *mut *mut u32 gaddr2; / 0x.888 - Group address register 2,
    pub /: *mut *mut u32 gaddr3; / 0x.88c - Group address register 3,
    pub /: *mut *mut u32 gaddr4; / 0x.890 - Group address register 4,
    pub /: *mut *mut u32 gaddr5; / 0x.894 - Group address register 5,
    pub /: *mut *mut u32 gaddr6; / 0x.898 - Group address register 6,
    pub /: *mut *mut u32 gaddr7; / 0x.89c - Group address register 7,
    pub res23a: [u8; 352],
    pub /: *mut *mut u32 fifocfg; / 0x.a00 - FIFO interface config register,
    pub res23b: [u8; 252],
    pub res23c: [u8; 248],
    pub /: *mut *mut u32 attr; / 0x.bf8 - Attributes Register,
    pub /: *mut *mut u32 attreli; / 0x.bfc - Attributes Extract Length and Extract Index Register,
    pub /: *mut *mut u32 rqprm0; / 0x.c00 - Receive queue parameters register 0,
    pub /: *mut *mut u32 rqprm1; / 0x.c04 - Receive queue parameters register 1,
    pub /: *mut *mut u32 rqprm2; / 0x.c08 - Receive queue parameters register 2,
    pub /: *mut *mut u32 rqprm3; / 0x.c0c - Receive queue parameters register 3,
    pub /: *mut *mut u32 rqprm4; / 0x.c10 - Receive queue parameters register 4,
    pub /: *mut *mut u32 rqprm5; / 0x.c14 - Receive queue parameters register 5,
    pub /: *mut *mut u32 rqprm6; / 0x.c18 - Receive queue parameters register 6,
    pub /: *mut *mut u32 rqprm7; / 0x.c1c - Receive queue parameters register 7,
    pub res24: [u8; 36],
    pub /: *mut *mut u32 rfbptr0; / 0x.c44 - Last free RxBD pointer for ring 0,
    pub res24a: [u8; 4],
    pub /: *mut *mut u32 rfbptr1; / 0x.c4c - Last free RxBD pointer for ring 1,
    pub res24b: [u8; 4],
    pub /: *mut *mut u32 rfbptr2; / 0x.c54 - Last free RxBD pointer for ring 2,
    pub res24c: [u8; 4],
    pub /: *mut *mut u32 rfbptr3; / 0x.c5c - Last free RxBD pointer for ring 3,
    pub res24d: [u8; 4],
    pub /: *mut *mut u32 rfbptr4; / 0x.c64 - Last free RxBD pointer for ring 4,
    pub res24e: [u8; 4],
    pub /: *mut *mut u32 rfbptr5; / 0x.c6c - Last free RxBD pointer for ring 5,
    pub res24f: [u8; 4],
    pub /: *mut *mut u32 rfbptr6; / 0x.c74 - Last free RxBD pointer for ring 6,
    pub res24g: [u8; 4],
    pub /: *mut *mut u32 rfbptr7; / 0x.c7c - Last free RxBD pointer for ring 7,
    pub res24h: [u8; 4],
    pub res24x: [u8; 556],
    pub /: *mut *mut u32 isrg0; / 0x.eb0 - Interrupt steering group 0 register,
    pub /: *mut *mut u32 isrg1; / 0x.eb4 - Interrupt steering group 1 register,
    pub /: *mut *mut u32 isrg2; / 0x.eb8 - Interrupt steering group 2 register,
    pub /: *mut *mut u32 isrg3; / 0x.ebc - Interrupt steering group 3 register,
    pub res25: [u8; 16],
    pub /: *mut *mut u32 rxic0; / 0x.ed0 - Ring 0 Rx interrupt coalescing,
    pub /: *mut *mut u32 rxic1; / 0x.ed4 - Ring 1 Rx interrupt coalescing,
    pub /: *mut *mut u32 rxic2; / 0x.ed8 - Ring 2 Rx interrupt coalescing,
    pub /: *mut *mut u32 rxic3; / 0x.edc - Ring 3 Rx interrupt coalescing,
    pub /: *mut *mut u32 rxic4; / 0x.ee0 - Ring 4 Rx interrupt coalescing,
    pub /: *mut *mut u32 rxic5; / 0x.ee4 - Ring 5 Rx interrupt coalescing,
    pub /: *mut *mut u32 rxic6; / 0x.ee8 - Ring 6 Rx interrupt coalescing,
    pub /: *mut *mut u32 rxic7; / 0x.eec - Ring 7 Rx interrupt coalescing,
    pub res26: [u8; 32],
    pub /: *mut *mut u32 txic0; / 0x.f10 - Ring 0 Tx interrupt coalescing,
    pub /: *mut *mut u32 txic1; / 0x.f14 - Ring 1 Tx interrupt coalescing,
    pub /: *mut *mut u32 txic2; / 0x.f18 - Ring 2 Tx interrupt coalescing,
    pub /: *mut *mut u32 txic3; / 0x.f1c - Ring 3 Tx interrupt coalescing,
    pub /: *mut *mut u32 txic4; / 0x.f20 - Ring 4 Tx interrupt coalescing,
    pub /: *mut *mut u32 txic5; / 0x.f24 - Ring 5 Tx interrupt coalescing,
    pub /: *mut *mut u32 txic6; / 0x.f28 - Ring 6 Tx interrupt coalescing,
    pub /: *mut *mut u32 txic7; / 0x.f2c - Ring 7 Tx interrupt coalescing,
    pub res27: [u8; 208],
}

// Flags related to gianfar device features
pub const FSL_GIANFAR_DEV_HAS_GIGABIT: c_uint = 0x00000001;
pub const FSL_GIANFAR_DEV_HAS_COALESCE: c_uint = 0x00000002;
pub const FSL_GIANFAR_DEV_HAS_RMON: c_uint = 0x00000004;
pub const FSL_GIANFAR_DEV_HAS_MULTI_INTR: c_uint = 0x00000008;
pub const FSL_GIANFAR_DEV_HAS_CSUM: c_uint = 0x00000010;
pub const FSL_GIANFAR_DEV_HAS_VLAN: c_uint = 0x00000020;
pub const FSL_GIANFAR_DEV_HAS_EXTENDED_HASH: c_uint = 0x00000040;
pub const FSL_GIANFAR_DEV_HAS_MAGIC_PACKET: c_uint = 0x00000100;
pub const FSL_GIANFAR_DEV_HAS_BD_STASHING: c_uint = 0x00000200;
pub const FSL_GIANFAR_DEV_HAS_BUF_STASHING: c_uint = 0x00000400;
pub const FSL_GIANFAR_DEV_HAS_TIMER: c_uint = 0x00000800;
pub const FSL_GIANFAR_DEV_HAS_WAKE_ON_FILER: c_uint = 0x00001000;
pub const FSL_GIANFAR_DEV_HAS_RX_FILER: c_uint = 0x00002000;

pub const DEFAULT_MAPPING: c_uint = 0xAA;

pub const DEFAULT_MAPPING: c_uint = 0xFF;

pub const ISRG_RR0: c_uint = 0x80000000;
pub const ISRG_TR0: c_uint = 0x00800000;
// The same driver can operate in two modes
// SQ_SG_MODE: Single Queue Single Group Mode
// (Backward compatible mode)
// MQ_MG_MODE: Multi Queue Multi Group mode
//
// Per TX queue stats
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_q_stats {
    pub tx_packets: u64,
    pub tx_bytes: u64,
}

//
// struct gfar_priv_tx_q - per tx queue structure
// @txlock: per queue tx spin lock
// @tx_skbuff:skb pointers
// @skb_curtx: to be used skb pointer
// @skb_dirtytx:the last used skb pointer
// @stats: bytes/packets stats
// @qindex: index of this queue
// @dev: back pointer to the dev structure
// @grp: back pointer to the group to which this queue belongs
// @tx_bd_base: First tx buffer descriptor
// @cur_tx: Next free ring entry
// @dirty_tx: First buffer in line to be transmitted
// @tx_ring_size: Tx ring size
// @num_txbdfree: number of free TxBds
// @txcoalescing: enable/disable tx coalescing
// @txic: transmit interrupt coalescing value
// @txcount: coalescing value if based on tx frame count
// @txtime: coalescing value if based on time
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfar_priv_tx_q {
// cacheline 1
// C attribute field omitted
    pub tx_bd_base: *mut txbd8,
    pub cur_tx: *mut txbd8,
    pub num_txbdfree: c_uint,
    pub skb_curtx: c_ushort,
    pub tx_ring_size: c_ushort,
    pub stats: tx_q_stats,
    pub grp: *mut gfar_priv_grp,
// cacheline 2
    pub dev: *mut net_device,
    pub tx_skbuff: *mut sk_buff,
    pub dirty_tx: *mut txbd8,
    pub skb_dirtytx: c_ushort,
    pub qindex: c_ushort,
// Configuration info for the coalescing features
    pub txcoalescing: c_uint,
    pub txic: c_ulong,
    pub tx_bd_dma_base: dma_addr_t,
}

//
// Per RX queue stats
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_q_stats {
    pub rx_packets: u64,
    pub rx_bytes: u64,
    pub rx_dropped: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfar_rx_buff {
    pub dma: dma_addr_t,
    pub page: *mut page,
    pub page_offset: c_uint,
}

//
// struct gfar_priv_rx_q - per rx queue structure
// @rx_buff: Array of buffer info metadata structs
// @rx_bd_base: First rx buffer descriptor
// @next_to_use: index of the next buffer to be alloc'd
// @next_to_clean: index of the next buffer to be cleaned
// @qindex: index of this queue
// @ndev: back pointer to net_device
// @rx_ring_size: Rx ring size
// @rxcoalescing: enable/disable rx-coalescing
// @rxic: receive interrupt coalescing vlaue
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfar_priv_rx_q {
    pub __aligned(SMP_CACHE_BYTES): *mut *mut gfar_rx_buff rx_buff,
    pub rx_bd_base: *mut rxbd8,
    pub ndev: *mut net_device,
    pub dev: *mut device,
    pub rx_ring_size: u16,
    pub qindex: u16,
    pub grp: *mut gfar_priv_grp,
    pub next_to_clean: u16,
    pub next_to_use: u16,
    pub next_to_alloc: u16,
    pub skb: *mut sk_buff,
    pub stats: rx_q_stats,
    pub rfbptr: *mut u32 __iomem,
    pub rxcoalescing: c_uchar,
    pub rxic: c_ulong,
    pub rx_bd_dma_base: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gfar_irqinfo_id {
    GFAR_TX = 0,
    GFAR_RX = 1,
    GFAR_ER = 2,
    GFAR_NUM_IRQS = 3
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfar_irqinfo {
    pub irq: c_uint,
    pub name: [c_char; GFAR_INT_NAME_MAX],
}

//
// struct gfar_priv_grp - per group structure
// @napi: the napi poll function
// @priv: back pointer to the priv structure
// @regs: the ioremapped register space for this group
// @irqinfo: TX/RX/ER irq data for this group
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfar_priv_grp {
    pub __aligned(SMP_CACHE_BYTES): spinlock_t grplock,
    pub napi_rx: napi_struct,
    pub napi_tx: napi_struct,
    pub regs: *mut gfar __iomem,
    pub tx_queue: *mut gfar_priv_tx_q,
    pub rx_queue: *mut gfar_priv_rx_q,
    pub tstat: c_uint,
    pub rstat: c_uint,
    pub priv: *mut gfar_private,
    pub num_tx_queues: c_ulong,
    pub tx_bit_map: c_ulong,
    pub num_rx_queues: c_ulong,
    pub rx_bit_map: c_ulong,
    pub irqinfo: [*mut gfar_irqinfo; GFAR_NUM_IRQS],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gfar_errata {
    GFAR_ERRATA_74		= 0x01,
    GFAR_ERRATA_76		= 0x02,
    GFAR_ERRATA_A002	= 0x04,
    GFAR_ERRATA_12		= 0x08, /* a.k.a errata eTSEC49 */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gfar_dev_state {
    GFAR_DOWN = 1,
    GFAR_RESETTING
}

// Struct stolen almost completely (and shamelessly) from the FCC enet source
// (Ok, that's not so true anymore, but there is a family resemblance)
// The GFAR buffer descriptors track the ring buffers.  The rx_bd_base
// and tx_bd_base always point to the currently available buffer.
// The dirty_tx tracks the current buffer that is being sent by the
// controller.  The cur_tx and dirty_tx are equal under both completely
// empty and completely full conditions.  The empty/ready indicator in
// the buffer descriptor determines the actual condition.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfar_private {
    pub dev: *mut device,
    pub ndev: *mut net_device,
    pub errata: gfar_errata,
    pub uses_rxfcb: u16,
    pub padding: u16,
    pub device_flags: u32,
// HW time stamping enabled flag
    pub hwts_rx_en: c_int,
    pub hwts_tx_en: c_int,
    pub tx_queue: [*mut gfar_priv_tx_q; MAX_TX_QS],
    pub rx_queue: [*mut gfar_priv_rx_q; MAX_RX_QS],
    pub gfargrp: [gfar_priv_grp; MAXGROUPS],
    pub state: c_ulong,
    pub mode: c_ushort,
    pub num_tx_queues: c_uint,
    pub num_rx_queues: c_uint,
    pub num_grps: c_uint,
    pub tx_actual_en: c_int,
// Network Statistics
    pub extra_stats: gfar_extra_stats,
    pub rmon_overflow: rmon_overflow,
// PHY stuff
    pub interface: phy_interface_t,
    pub phy_node: *mut device_node,
    pub tbi_node: *mut device_node,
    pub mii_bus: *mut mii_bus,
    pub oldspeed: c_int,
    pub oldduplex: c_int,
    pub oldlink: c_int,
    pub msg_enable: u32,
    pub reset_task: work_struct,
    pub ofdev: *mut platform_device,
// Enable priorty based Tx scheduling in Hw
// Flow control flags
// The total tx and rx ring size for the enabled queues
    pub total_tx_ring_size: c_uint,
    pub total_rx_ring_size: c_uint,
    pub rqueue: u32,
    pub tqueue: u32,
// RX per device parameters
    pub rx_stash_size: c_uint,
    pub rx_stash_index: c_uint,
    pub cur_filer_idx: u32,
// RX queue filer rule set
    pub rx_list: ethtool_rx_list,
    pub rx_queue_access: mutex,
// Hash registers and their width
    pub hash_regs: [*mut u32 __iomem; 16],
    pub hash_width: c_int,
// wake-on-lan settings
    pub wol_opts: u16,
    pub wol_supported: u16,
// Filer table
    pub 1]: unsigned int ftp_rqfpr[MAX_FILER_IDX +,
    pub 1]: unsigned int ftp_rqfcr[MAX_FILER_IDX +,
}

// fcr = gfar_read(&regs->rqfcr);
// fpr = gfar_read(&regs->rqfpr);

// The powerpc-specific eieio() is used, as wmb() has too strong
// semantics (it requires synchronization between cacheable and
// uncacheable mappings, which eieio() doesn't provide and which we
// don't need), thus requiring a more expensive sync instruction.  At
// some point, the set of architecture-independent barrier functions
// should be expanded to include weaker barriers.
//

extern "C" {
    pub fn startup_gfar(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn stop_gfar(dev: *mut net_device);
}
extern "C" {
    pub fn gfar_mac_reset(priv: *mut gfar_private);
}
extern "C" {
    pub fn gfar_set_features(dev: *mut net_device, features: netdev_features_t) -> c_int;
}

pub const RQFCR_PID_PRI_MASK: c_uint = 0xFFFFFFF8;
pub const RQFCR_PID_L4P_MASK: c_uint = 0xFFFFFF00;
pub const RQFCR_PID_VID_MASK: c_uint = 0xFFFFF000;
pub const RQFCR_PID_PORT_MASK: c_uint = 0xFFFF0000;
pub const RQFCR_PID_MAC_MASK: c_uint = 0xFF000000;
// Represents a receive filer table entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfar_filer_entry {
    pub ctrl: u32,
    pub prop: u32,
}

// The 20 additional entries are a shadow for one extra element
#[repr(C)]
#[derive(Copy, Clone)]
pub struct filer_table {
    pub index: u32,
    pub 20]: gfar_filer_entry fe[MAX_FILER_CACHE_IDX +,
}
