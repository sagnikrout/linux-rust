//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sun/cassini.h
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


// SPDX-License-Identifier: GPL-2.0+
// $Id: cassini.h,v 1.16 2004/08/17 21:15:16 zaumen Exp $
// cassini.h: Definitions for Sun Microsystems Cassini(+) ethernet driver.
//
// Copyright (C) 2004 Sun Microsystems Inc.
// Copyright (c) 2003 Adrian Sun (asun@darksunrising.com)
//
// vendor id: 0x108E (Sun Microsystems, Inc.)
// device id: 0xabba (Cassini)
// revision ids: 0x01 = Cassini
// 0x02 = Cassini rev 2
// 0x10 = Cassini+
// 0x11 = Cassini+ 0.2u
//
// vendor id: 0x100b (National Semiconductor)
// device id: 0x0035 (DP83065/Saturn)
// revision ids: 0x30 = Saturn B2
//
// rings are all offset from 0.
//
// there are two clock domains:
// PCI:  33/66MHz clock
// chip: 125MHz clock
//
// cassini register map: 2M memory mapped in 32-bit memory space accessible as
// 32-bit words. there is no i/o port access. REG_ addresses are
// shared between cassini and cassini+. REG_PLUS_ addresses only
// appear in cassini+. REG_MINUS_ addresses only appear in cassini.
//
pub const CAS_ID_REV2: c_uint = 0x02;
pub const CAS_ID_REVPLUS: c_uint = 0x10;
pub const CAS_ID_REVPLUS02u: c_uint = 0x11;
pub const CAS_ID_REVSATURNB2: c_uint = 0x30;
// global resources
// this register sets the weights for the weighted round robin arbiter. e.g.,
// if rx weight == 1 and tx weight == 0, rx == 2x tx transfer credit
// for its next turn to access the pci bus.
// map: 0x0 = x1, 0x1 = x2, 0x2 = x4, 0x3 = x8
// DEFAULT: 0x0, SIZE: 5 bits
//
pub const REG_CAWR: c_uint = 0x0004  /* core arbitration weight */;
pub const CAWR_RX_DMA_WEIGHT_SHIFT: c_int = 0;
pub const CAWR_RX_DMA_WEIGHT_MASK: c_uint = 0x03    /* [0:1] */;
pub const CAWR_TX_DMA_WEIGHT_SHIFT: c_int = 2;
pub const CAWR_TX_DMA_WEIGHT_MASK: c_uint = 0x0C    /* [3:2] */;
pub const CAWR_RR_DIS: c_uint = 0x10    /* [4] */;
// if enabled, BIM can send bursts across PCI bus > cacheline size. burst
// sizes determined by length of packet or descriptor transfer and the
// max length allowed by the target.
// DEFAULT: 0x0, SIZE: 1 bit
//
pub const REG_INF_BURST: c_uint = 0x0008  /* infinite burst enable reg */;
pub const INF_BURST_EN: c_uint = 0x1     /* enable */;
// top level interrupts [0-9] are auto-cleared to 0 when the status
// register is read. second level interrupts [13 - 18] are cleared at
// the source. tx completion register 3 is replicated in [19 - 31]
// DEFAULT: 0x00000000, SIZE: 29 bits
//
pub const REG_INTR_STATUS: c_uint = 0x000C  /* interrupt status register */;
pub const INTR_TX_INTME: c_uint = 0x00000001  /* frame w/ INT ME desc bit set;
pub const INTR_TX_ALL: c_uint = 0x00000002  /* all xmit frames xferred into;
pub const INTR_TX_DONE: c_uint = 0x00000004  /* any frame xferred into tx;
pub const INTR_TX_TAG_ERROR: c_uint = 0x00000008  /* TX FIFO tag framing;
pub const INTR_RX_DONE: c_uint = 0x00000010  /* at least 1 frame xferred;
pub const INTR_RX_BUF_UNAVAIL: c_uint = 0x00000020  /* no more receive buffers.;
pub const INTR_RX_TAG_ERROR: c_uint = 0x00000040  /* RX FIFO tag framing;
pub const INTR_RX_COMP_FULL: c_uint = 0x00000080  /* no more room in completion;
pub const INTR_RX_BUF_AE: c_uint = 0x00000100  /* less than the;
pub const INTR_RX_COMP_AF: c_uint = 0x00000200  /* less than the;
pub const INTR_RX_LEN_MISMATCH: c_uint = 0x00000400  /* len field from MAC !=;
pub const INTR_SUMMARY: c_uint = 0x00001000  /* summary interrupt bit. this;
pub const INTR_PCS_STATUS: c_uint = 0x00002000  /* PCS interrupt status register */;
pub const INTR_TX_MAC_STATUS: c_uint = 0x00004000  /* TX MAC status register has at;
pub const INTR_RX_MAC_STATUS: c_uint = 0x00008000  /* RX MAC status register has at;
pub const INTR_MAC_CTRL_STATUS: c_uint = 0x00010000  /* MAC control status register has;
pub const INTR_MIF_STATUS: c_uint = 0x00020000  /* MIF status register has at least;
pub const INTR_PCI_ERROR_STATUS: c_uint = 0x00040000  /* PCI error status register in the;
pub const INTR_TX_COMP_3_MASK: c_uint = 0xFFF80000  /* mask for TX completion;
pub const INTR_TX_COMP_3_SHIFT: c_int = 19;

// determines which status events will cause an interrupt. layout same
// as REG_INTR_STATUS.
// DEFAULT: 0xFFFFFFFF, SIZE: 16 bits
//
pub const REG_INTR_MASK: c_uint = 0x0010  /* Interrupt mask */;
// top level interrupt bits that are cleared during read of REG_INTR_STATUS_ALIAS.
// useful when driver is polling for interrupts. layout same as REG_INTR_MASK.
// DEFAULT: 0x00000000, SIZE: 12 bits
//
pub const REG_ALIAS_CLEAR: c_uint = 0x0014  /* alias clear mask;
// same as REG_INTR_STATUS except that only bits cleared are those selected by
// REG_ALIAS_CLEAR
// DEFAULT: 0x00000000, SIZE: 29 bits
//
pub const REG_INTR_STATUS_ALIAS: c_uint = 0x001C  /* interrupt status alias;
// DEFAULT: 0x0, SIZE: 3 bits
pub const REG_PCI_ERR_STATUS: c_uint = 0x1000  /* PCI error status */;
pub const PCI_ERR_BADACK: c_uint = 0x01    /* reserved in Cassini+.;
pub const PCI_ERR_DTRTO: c_uint = 0x02    /* delayed xaction timeout. set if;
pub const PCI_ERR_OTHER: c_uint = 0x04    /* other PCI errors */;
pub const PCI_ERR_BIM_DMA_WRITE: c_uint = 0x08    /* BIM received 0 count DMA write req.;
pub const PCI_ERR_BIM_DMA_READ: c_uint = 0x10    /* BIM received 0 count DMA read req.;
pub const PCI_ERR_BIM_DMA_TIMEOUT: c_uint = 0x20    /* BIM received 255 retries during;
// mask for PCI status events that will set PCI_ERR_STATUS. if cleared, event
// causes an interrupt to be generated.
// DEFAULT: 0x7, SIZE: 3 bits
//
pub const REG_PCI_ERR_STATUS_MASK: c_uint = 0x1004  /* PCI Error status mask */;
// used to configure PCI related parameters that are not in PCI config space.
// DEFAULT: 0bxx000, SIZE: 5 bits
//
pub const REG_BIM_CFG: c_uint = 0x1008  /* BIM Configuration */;
pub const BIM_CFG_RESERVED0: c_uint = 0x001   /* reserved */;
pub const BIM_CFG_RESERVED1: c_uint = 0x002   /* reserved */;
pub const BIM_CFG_64BIT_DISABLE: c_uint = 0x004   /* disable 64-bit mode */;
pub const BIM_CFG_66MHZ: c_uint = 0x008   /* (ro) 1 = 66MHz, 0 = < 66MHz */;
pub const BIM_CFG_32BIT: c_uint = 0x010   /* (ro) 1 = 32-bit slot, 0 = 64-bit */;
pub const BIM_CFG_DPAR_INTR_ENABLE: c_uint = 0x020   /* detected parity err enable */;
pub const BIM_CFG_RMA_INTR_ENABLE: c_uint = 0x040   /* master abort intr enable */;
pub const BIM_CFG_RTA_INTR_ENABLE: c_uint = 0x080   /* target abort intr enable */;
pub const BIM_CFG_RESERVED2: c_uint = 0x100   /* reserved */;
pub const BIM_CFG_BIM_DISABLE: c_uint = 0x200   /* stop BIM DMA. use before global;
pub const BIM_CFG_BIM_STATUS: c_uint = 0x400   /* (ro) 1 = BIM DMA suspended.;
pub const BIM_CFG_PERROR_BLOCK: c_uint = 0x800  /* block PERR# to pci bus. def: 0.;
// DEFAULT: 0x00000000, SIZE: 32 bits
pub const REG_BIM_DIAG: c_uint = 0x100C  /* BIM Diagnostic */;
pub const BIM_DIAG_MSTR_SM_MASK: c_uint = 0x3FFFFF00 /* PCI master controller state;
pub const BIM_DIAG_BRST_SM_MASK: c_uint = 0x7F    /* PCI burst controller state;
// writing to SW_RESET_TX and SW_RESET_RX will issue a global
// reset. poll until TX and RX read back as 0's for completion.
//
pub const REG_SW_RESET: c_uint = 0x1010  /* Software reset */;
pub const SW_RESET_TX: c_uint = 0x00000001  /* reset TX DMA engine. poll until;
pub const SW_RESET_RX: c_uint = 0x00000002  /* reset RX DMA engine. poll until;
pub const SW_RESET_RSTOUT: c_uint = 0x00000004  /* force RSTOUT# pin active (low).;
pub const SW_RESET_BLOCK_PCS_SLINK: c_uint = 0x00000008  /* if a global reset is done with;
pub const SW_RESET_BREQ_SM_MASK: c_uint = 0x00007F00  /* breq state machine [6:0] */;
pub const SW_RESET_PCIARB_SM_MASK: c_uint = 0x00070000  /* pci arbitration state bits:;
pub const SW_RESET_RDPCI_SM_MASK: c_uint = 0x00300000  /* read pci state bits:;
pub const SW_RESET_RDARB_SM_MASK: c_uint = 0x00C00000  /* read arbitration state bits:;
pub const SW_RESET_WRPCI_SM_MASK: c_uint = 0x06000000  /* write pci state bits;
pub const SW_RESET_WRARB_SM_MASK: c_uint = 0x38000000  /* write arbitration state bits:;
// Cassini only. 64-bit register used to check PCI datapath. when read,
// value written has both lower and upper 32-bit halves rotated to the right
// one bit position. e.g., FFFFFFFF FFFFFFFF -> 7FFFFFFF 7FFFFFFF
//
pub const REG_MINUS_BIM_DATAPATH_TEST: c_uint = 0x1018  /* Cassini: BIM datapath test;
// output enables are provided for each device's chip select and for the rest
// of the outputs from cassini to its local bus devices. two sw programmable
// bits are connected to general purpose control/status bits.
// DEFAULT: 0x7
//
pub const REG_BIM_LOCAL_DEV_EN: c_uint = 0x1020  /* BIM local device;
pub const BIM_LOCAL_DEV_PAD: c_uint = 0x01    /* address bus, RW signal, and;
pub const BIM_LOCAL_DEV_PROM: c_uint = 0x02    /* PROM chip select */;
pub const BIM_LOCAL_DEV_EXT: c_uint = 0x04    /* secondary local bus device chip;
pub const BIM_LOCAL_DEV_SOFT_0: c_uint = 0x08    /* sw programmable ctrl bit 0 */;
pub const BIM_LOCAL_DEV_SOFT_1: c_uint = 0x10    /* sw programmable ctrl bit 1 */;
pub const BIM_LOCAL_DEV_HW_RESET: c_uint = 0x20    /* internal hw reset. Cassini+ only. */;
// access 24 entry BIM read and write buffers. put address in REG_BIM_BUFFER_ADDR
// and read/write from/to it REG_BIM_BUFFER_DATA_LOW and _DATA_HI.
// _DATA_HI should be the last access of the sequence.
// DEFAULT: undefined
//
pub const REG_BIM_BUFFER_ADDR: c_uint = 0x1024  /* BIM buffer address. for;
pub const BIM_BUFFER_ADDR_MASK: c_uint = 0x3F    /* index (0 - 23) of buffer  */;
pub const BIM_BUFFER_WR_SELECT: c_uint = 0x40    /* write buffer access = 1;
// DEFAULT: undefined
pub const REG_BIM_BUFFER_DATA_LOW: c_uint = 0x1028  /* BIM buffer data low */;
pub const REG_BIM_BUFFER_DATA_HI: c_uint = 0x102C  /* BIM buffer data high */;
// set BIM_RAM_BIST_START to start built-in self test for BIM read buffer.
// bit auto-clears when done with status read from _SUMMARY and _PASS bits.
//
pub const REG_BIM_RAM_BIST: c_uint = 0x102C  /* BIM RAM (read buffer) BIST;
pub const BIM_RAM_BIST_RD_START: c_uint = 0x01    /* start BIST for BIM read buffer */;
pub const BIM_RAM_BIST_WR_START: c_uint = 0x02    /* start BIST for BIM write buffer.;
pub const BIM_RAM_BIST_RD_PASS: c_uint = 0x04    /* summary BIST pass status for read;
pub const BIM_RAM_BIST_WR_PASS: c_uint = 0x08    /* summary BIST pass status for write;
pub const BIM_RAM_BIST_RD_LOW_PASS: c_uint = 0x10    /* read low bank passes BIST */;
pub const BIM_RAM_BIST_RD_HI_PASS: c_uint = 0x20    /* read high bank passes BIST */;
pub const BIM_RAM_BIST_WR_LOW_PASS: c_uint = 0x40    /* write low bank passes BIST.;
pub const BIM_RAM_BIST_WR_HI_PASS: c_uint = 0x80    /* write high bank passes BIST.;
// ASUN: i'm not sure what this does as it's not in the spec.
// DEFAULT: 0xFC
//
pub const REG_BIM_DIAG_MUX: c_uint = 0x1030  /* BIM diagnostic probe mux;
// enable probe monitoring mode and select data appearing on the P_A* bus. bit
// values for _SEL_HI_MASK and _SEL_LOW_MASK:
// 0x0: internal probe[7:0] (pci arb state, wtc empty w, wtc full w, wtc empty w,
// wtc empty r, post pci)
// 0x1: internal probe[15:8] (pci wbuf comp, pci wpkt comp, pci rbuf comp,
// pci rpkt comp, txdma wr req, txdma wr ack,
// txdma wr rdy, txdma wr xfr done)
// 0x2: internal probe[23:16] (txdma rd req, txdma rd ack, txdma rd rdy, rxdma rd,
// rd arb state, rd pci state)
// 0x3: internal probe[31:24] (rxdma req, rxdma ack, rxdma rdy, wrarb state,
// wrpci state)
// 0x4: pci io probe[7:0]     0x5: pci io probe[15:8]
// 0x6: pci io probe[23:16]   0x7: pci io probe[31:24]
// 0x8: pci io probe[39:32]   0x9: pci io probe[47:40]
// 0xa: pci io probe[55:48]   0xb: pci io probe[63:56]
// the following are not available in Cassini:
// 0xc: rx probe[7:0]         0xd: tx probe[7:0]
// 0xe: hp probe[7:0] 	      0xf: mac probe[7:0]
//
pub const REG_PLUS_PROBE_MUX_SELECT: c_uint = 0x1034 /* Cassini+: PROBE MUX SELECT */;
pub const PROBE_MUX_EN: c_uint = 0x80000000 /* allow probe signals to be;
pub const PROBE_MUX_SUB_MUX_MASK: c_uint = 0x0000FF00 /* select sub module probe signals:;
pub const PROBE_MUX_SEL_HI_MASK: c_uint = 0x000000F0 /* select which module to appear;
pub const PROBE_MUX_SEL_LOW_MASK: c_uint = 0x0000000F /* select which module to appear;
// values mean the same thing as REG_INTR_MASK excep that it's for INTB.
pub const REG_PLUS_INTR_MASK_1: c_uint = 0x1038 /* Cassini+: interrupt mask;

// bits correspond to both _MASK and _STATUS registers. _ALT corresponds to
// all of the alternate (2-4) INTR registers while _1 corresponds to only
// _MASK_1 and _STATUS_1 registers.
// DEFAULT: 0x7 for MASK registers, 0x0 for ALIAS_CLEAR registers
//
pub const INTR_RX_DONE_ALT: c_uint = 0x01;
pub const INTR_RX_COMP_FULL_ALT: c_uint = 0x02;
pub const INTR_RX_COMP_AF_ALT: c_uint = 0x04;
pub const INTR_RX_BUF_UNAVAIL_1: c_uint = 0x08;
pub const INTR_RX_BUF_AE_1: c_uint = 0x10 /* almost empty */;
pub const INTRN_MASK_RX_EN: c_uint = 0x80;

pub const REG_PLUS_INTR_STATUS_1: c_uint = 0x103C /* Cassini+: interrupt status;

pub const INTR_STATUS_ALT_INTX_EN: c_uint = 0x80   /* generate INTX when one of the;
pub const REG_PLUS_ALIAS_CLEAR_1: c_uint = 0x1040 /* Cassini+: alias clear mask;

pub const REG_PLUS_INTR_STATUS_ALIAS_1: c_uint = 0x1044 /* Cassini+: interrupt status;

pub const REG_SATURN_PCFG: c_uint = 0x106c /* pin configuration register for;
pub const SATURN_PCFG_TLA: c_uint = 0x00000001 /* 1 = phy actled */;
pub const SATURN_PCFG_FLA: c_uint = 0x00000002 /* 1 = phy link10led */;
pub const SATURN_PCFG_CLA: c_uint = 0x00000004 /* 1 = phy link100led */;
pub const SATURN_PCFG_LLA: c_uint = 0x00000008 /* 1 = phy link1000led */;
pub const SATURN_PCFG_RLA: c_uint = 0x00000010 /* 1 = phy duplexled */;
pub const SATURN_PCFG_PDS: c_uint = 0x00000020 /* phy debug mode.;
pub const SATURN_PCFG_MTP: c_uint = 0x00000080 /* test point select */;
pub const SATURN_PCFG_GMO: c_uint = 0x00000100 /* GMII observe. 1 =;
pub const SATURN_PCFG_FSI: c_uint = 0x00000200 /* 1 = freeze serdes/gmii. all;
pub const SATURN_PCFG_LAD: c_uint = 0x00000800 /* 0 = mac core led ctrl;
// transmit dma registers
pub const MAX_TX_RINGS_SHIFT: c_int = 2;

// TX configuration.
// descr ring sizes size = 32 * (1 << n), n < 9. e.g., 0x8 = 8k. default: 0x8
// DEFAULT: 0x3F000001
//
pub const REG_TX_CFG: c_uint = 0x2004  /* TX config */;
pub const TX_CFG_DMA_EN: c_uint = 0x00000001  /* enable TX DMA. if cleared, DMA;
pub const TX_CFG_FIFO_PIO_SEL: c_uint = 0x00000002  /* TX DMA FIFO can be;
pub const TX_CFG_DESC_RING0_MASK: c_uint = 0x0000003C  /* # desc entries in;
pub const TX_CFG_DESC_RING0_SHIFT: c_int = 2;

pub const TX_CFG_PACED_MODE: c_uint = 0x00100000  /* TX_ALL only set after;
pub const TX_CFG_DMA_RDPIPE_DIS: c_uint = 0x01000000  /* always set to 1 */;
pub const TX_CFG_COMPWB_Q1: c_uint = 0x02000000  /* completion writeback happens at;
pub const TX_CFG_COMPWB_Q2: c_uint = 0x04000000  /* completion writeback happens at;
pub const TX_CFG_COMPWB_Q3: c_uint = 0x08000000  /* completion writeback happens at;
pub const TX_CFG_COMPWB_Q4: c_uint = 0x10000000  /* completion writeback happens at;
pub const TX_CFG_INTR_COMPWB_DIS: c_uint = 0x20000000  /* disable pre-interrupt completion;
pub const TX_CFG_CTX_SEL_MASK: c_uint = 0xC0000000  /* selects tx test port;
pub const TX_CFG_CTX_SEL_SHIFT: c_int = 30;
// 11-bit counters that point to next location in FIFO to be loaded/retrieved.
// used for diagnostics only.
//
pub const REG_TX_FIFO_WRITE_PTR: c_uint = 0x2014  /* TX FIFO write pointer */;
pub const REG_TX_FIFO_SHADOW_WRITE_PTR: c_uint = 0x2018  /* TX FIFO shadow write;
pub const REG_TX_FIFO_READ_PTR: c_uint = 0x201C  /* TX FIFO read pointer */;
pub const REG_TX_FIFO_SHADOW_READ_PTR: c_uint = 0x2020  /* TX FIFO shadow read;
// (ro) 11-bit up/down counter w/ # of frames currently in TX FIFO
pub const REG_TX_FIFO_PKT_CNT: c_uint = 0x2024  /* TX FIFO packet counter */;
// current state of all state machines in TX
pub const REG_TX_SM_1: c_uint = 0x2028  /* TX state machine reg #1 */;
pub const TX_SM_1_CHAIN_MASK: c_uint = 0x000003FF   /* chaining state machine */;
pub const TX_SM_1_CSUM_MASK: c_uint = 0x00000C00   /* checksum state machine */;
pub const TX_SM_1_FIFO_LOAD_MASK: c_uint = 0x0003F000   /* FIFO load state machine.;
pub const TX_SM_1_FIFO_UNLOAD_MASK: c_uint = 0x003C0000   /* FIFO unload state machine */;
pub const TX_SM_1_CACHE_MASK: c_uint = 0x03C00000   /* desc. prefetch cache controller;
pub const TX_SM_1_CBQ_ARB_MASK: c_uint = 0xF8000000   /* CBQ arbiter state machine */;
pub const REG_TX_SM_2: c_uint = 0x202C  /* TX state machine reg #2 */;
pub const TX_SM_2_COMP_WB_MASK: c_uint = 0x07    /* completion writeback sm */;
pub const TX_SM_2_SUB_LOAD_MASK: c_uint = 0x38    /* sub load state machine */;
pub const TX_SM_2_KICK_MASK: c_uint = 0xC0    /* kick state machine */;
// 64-bit pointer to the transmit data buffer. only the 50 LSB are incremented
// while the upper 23 bits are taken from the TX descriptor
//
pub const REG_TX_DATA_PTR_LOW: c_uint = 0x2030  /* TX data pointer low */;
pub const REG_TX_DATA_PTR_HI: c_uint = 0x2034  /* TX data pointer high */;
// 13 bit registers written by driver w/ descriptor value that follows
// last valid xmit descriptor. kick # and complete # values are used by
// the xmit dma engine to control tx descr fetching. if > 1 valid
// tx descr is available within the cache line being read, cassini will
// internally cache up to 4 of them. 0 on reset. _KICK = rw, _COMP = ro.
//
pub const REG_TX_KICK0: c_uint = 0x2038  /* TX kick reg #1 */;

pub const REG_TX_COMP0: c_uint = 0x2048  /* TX completion reg #1 */;

// values of TX_COMPLETE_1-4 are written. each completion register
// is 2bytes in size and contiguous. 8B allocation w/ 8B alignment.
// NOTE: completion reg values are only written back prior to TX_INTME and
// TX_ALL interrupts. at all other times, the most up-to-date index values
// should be obtained from the REG_TX_COMPLETE_# registers.
// here's the layout:
// offset from base addr      completion # byte
// 0                TX_COMPLETE_1_MSB
// 1                TX_COMPLETE_1_LSB
// 2                TX_COMPLETE_2_MSB
// 3                TX_COMPLETE_2_LSB
// 4                TX_COMPLETE_3_MSB
// 5                TX_COMPLETE_3_LSB
// 6                TX_COMPLETE_4_MSB
// 7                TX_COMPLETE_4_LSB
//
pub const TX_COMPWB_SIZE: c_int = 8;
pub const REG_TX_COMPWB_DB_LOW: c_uint = 0x2058  /* TX completion write back;
pub const REG_TX_COMPWB_DB_HI: c_uint = 0x205C  /* TX completion write back;
pub const TX_COMPWB_MSB_MASK: c_uint = 0x00000000000000FFULL;
pub const TX_COMPWB_MSB_SHIFT: c_int = 0;
pub const TX_COMPWB_LSB_MASK: c_uint = 0x000000000000FF00ULL;
pub const TX_COMPWB_LSB_SHIFT: c_int = 8;

// 53 MSB used as base address. 11 LSB assumed to be 0. TX desc pointer must
// be 2KB-aligned.
pub const REG_TX_DB0_LOW: c_uint = 0x2060  /* TX descriptor base low #1 */;
pub const REG_TX_DB0_HI: c_uint = 0x2064  /* TX descriptor base hi #1 */;

// 16-bit registers hold weights for the weighted round-robin of the
// four CBQ TX descr rings. weights correspond to # bytes xferred from
// host to TXFIFO in a round of WRR arbitration. can be set
// dynamically with new weights set upon completion of the current
// packet transfer from host memory to TXFIFO. a dummy write to any of
// these registers causes a queue1 pre-emption with all historical bw
// deficit data reset to 0 (useful when congestion requires a
// pre-emption/re-allocation of network bandwidth
//
pub const REG_TX_MAXBURST_0: c_uint = 0x2080  /* TX MaxBurst #1 */;
pub const REG_TX_MAXBURST_1: c_uint = 0x2084  /* TX MaxBurst #2 */;
pub const REG_TX_MAXBURST_2: c_uint = 0x2088  /* TX MaxBurst #3 */;
pub const REG_TX_MAXBURST_3: c_uint = 0x208C  /* TX MaxBurst #4 */;
// diagnostics access to any TX FIFO location. every access is 65
// bits.  _DATA_LOW = 32 LSB, _DATA_HI_T1/T0 = 32 MSB. _TAG = tag bit.
// writing _DATA_HI_T0 sets tag bit low, writing _DATA_HI_T1 sets tag
// bit high.  TX_FIFO_PIO_SEL must be set for TX FIFO PIO access. if
// TX FIFO data integrity is desired, TX DMA should be
// disabled. _DATA_HI_Tx should be the last access of the sequence.
//
pub const REG_TX_FIFO_ADDR: c_uint = 0x2104  /* TX FIFO address */;
pub const REG_TX_FIFO_TAG: c_uint = 0x2108  /* TX FIFO tag */;
pub const REG_TX_FIFO_DATA_LOW: c_uint = 0x210C  /* TX FIFO data low */;
pub const REG_TX_FIFO_DATA_HI_T1: c_uint = 0x2110  /* TX FIFO data high t1 */;
pub const REG_TX_FIFO_DATA_HI_T0: c_uint = 0x2114  /* TX FIFO data high t0 */;
pub const REG_TX_FIFO_SIZE: c_uint = 0x2118  /* (ro) TX FIFO size = 0x090 = 9KB */;
// 9-bit register controls BIST of TX FIFO. bit set indicates that the BIST
// passed for the specified memory
//
pub const REG_TX_RAMBIST: c_uint = 0x211C /* TX RAMBIST control/status */;
pub const TX_RAMBIST_STATE: c_uint = 0x01C0 /* progress state of RAMBIST;
pub const TX_RAMBIST_RAM33A_PASS: c_uint = 0x0020 /* RAM33A passed */;
pub const TX_RAMBIST_RAM32A_PASS: c_uint = 0x0010 /* RAM32A passed */;
pub const TX_RAMBIST_RAM33B_PASS: c_uint = 0x0008 /* RAM33B passed */;
pub const TX_RAMBIST_RAM32B_PASS: c_uint = 0x0004 /* RAM32B passed */;
pub const TX_RAMBIST_SUMMARY: c_uint = 0x0002 /* all RAM passed */;
pub const TX_RAMBIST_START: c_uint = 0x0001 /* write 1 to start BIST. self;
// receive dma registers
pub const MAX_RX_DESC_RINGS: c_int = 2;
pub const MAX_RX_COMP_RINGS: c_int = 4;
// receive DMA channel configuration. default: 0x80910
// free ring size       = (1 << n)*32  -> [32 - 8k]
// completion ring size = (1 << n)*128 -> [128 - 32k], n < 9
// DEFAULT: 0x80910
//
pub const REG_RX_CFG: c_uint = 0x4000  /* RX config */;
pub const RX_CFG_DMA_EN: c_uint = 0x00000001 /* enable RX DMA. 0 stops;
pub const RX_CFG_DESC_RING_MASK: c_uint = 0x0000001E /* # desc entries in RX;
pub const RX_CFG_DESC_RING_SHIFT: c_int = 1;
pub const RX_CFG_COMP_RING_MASK: c_uint = 0x000001E0 /* # desc entries in RX complete;
pub const RX_CFG_COMP_RING_SHIFT: c_int = 5;
pub const RX_CFG_BATCH_DIS: c_uint = 0x00000200 /* disable receive desc;
pub const RX_CFG_SWIVEL_MASK: c_uint = 0x00001C00 /* byte offset of the 1st;
pub const RX_CFG_SWIVEL_SHIFT: c_int = 10;
// cassini+ only
pub const RX_CFG_DESC_RING1_MASK: c_uint = 0x000F0000 /* # of desc entries in;
pub const RX_CFG_DESC_RING1_SHIFT: c_int = 16;
// the page size register allows cassini chips to do the following with
// received data:
// [--------------------------------------------------------------] page
// [off][buf1][pad][off][buf2][pad][off][buf3][pad][off][buf4][pad]
// |--------------| = PAGE_SIZE_BUFFER_STRIDE
// page = PAGE_SIZE
// offset = PAGE_SIZE_MTU_OFF
// for the above example, MTU_BUFFER_COUNT = 4.
// NOTE: as is apparent, you need to ensure that the following holds:
// MTU_BUFFER_COUNT <= PAGE_SIZE/PAGE_SIZE_BUFFER_STRIDE
// DEFAULT: 0x48002002 (8k pages)
//
pub const REG_RX_PAGE_SIZE: c_uint = 0x4004  /* RX page size */;
pub const RX_PAGE_SIZE_MASK: c_uint = 0x00000003 /* size of pages pointed to;
pub const RX_PAGE_SIZE_SHIFT: c_int = 0;
pub const RX_PAGE_SIZE_MTU_COUNT_MASK: c_uint = 0x00007800 /* # of MTU buffers the hw;
pub const RX_PAGE_SIZE_MTU_COUNT_SHIFT: c_int = 11;
pub const RX_PAGE_SIZE_MTU_STRIDE_MASK: c_uint = 0x18000000 /* # of bytes that separate;
pub const RX_PAGE_SIZE_MTU_STRIDE_SHIFT: c_int = 27;
pub const RX_PAGE_SIZE_MTU_OFF_MASK: c_uint = 0xC0000000 /* offset in each page that;
pub const RX_PAGE_SIZE_MTU_OFF_SHIFT: c_int = 30;
// 11-bit counter points to next location in RX FIFO to be loaded/read.
// shadow write pointers enable retries in case of early receive aborts.
// DEFAULT: 0x0. generated on 64-bit boundaries.
//
pub const REG_RX_FIFO_WRITE_PTR: c_uint = 0x4008  /* RX FIFO write pointer */;
pub const REG_RX_FIFO_READ_PTR: c_uint = 0x400C  /* RX FIFO read pointer */;
pub const REG_RX_IPP_FIFO_SHADOW_WRITE_PTR: c_uint = 0x4010  /* RX IPP FIFO shadow write;
pub const REG_RX_IPP_FIFO_SHADOW_READ_PTR: c_uint = 0x4014  /* RX IPP FIFO shadow read;
pub const REG_RX_IPP_FIFO_READ_PTR: c_uint = 0x400C  /* RX IPP FIFO read;
// current state of RX DMA state engines + other info
// DEFAULT: 0x0
//
pub const REG_RX_DEBUG: c_uint = 0x401C  /* RX debug */;
pub const RX_DEBUG_LOAD_STATE_MASK: c_uint = 0x0000000F /* load state machine w/ MAC:;
pub const RX_DEBUG_LM_STATE_MASK: c_uint = 0x00000070 /* load state machine w/ HP and;
pub const RX_DEBUG_FC_STATE_MASK: c_uint = 0x000000180 /* flow control state machine;
pub const RX_DEBUG_DATA_STATE_MASK: c_uint = 0x000001E00 /* unload data state machine;
pub const RX_DEBUG_DESC_STATE_MASK: c_uint = 0x0001E000 /* unload desc state machine;
pub const RX_DEBUG_INTR_READ_PTR_MASK: c_uint = 0x30000000 /* interrupt read ptr of the;
pub const RX_DEBUG_INTR_WRITE_PTR_MASK: c_uint = 0xC0000000 /* interrupt write pointer;
// flow control frames are emitted using two PAUSE thresholds:
// XOFF PAUSE uses pause time value pre-programmed in the Send PAUSE MAC reg
// XON PAUSE uses a pause time of 0. granularity of threshold is 64bytes.
// PAUSE thresholds defined in terms of FIFO occupancy and may be translated
// into FIFO vacancy using RX_FIFO_SIZE. setting ON will trigger XON frames
// when FIFO reaches 0. OFF threshold should not be > size of RX FIFO. max
// value is 0x6F.
// DEFAULT: 0x00078
//
pub const REG_RX_PAUSE_THRESH: c_uint = 0x4020  /* RX pause thresholds */;
pub const RX_PAUSE_THRESH_QUANTUM: c_int = 64;
pub const RX_PAUSE_THRESH_OFF_MASK: c_uint = 0x000001FF /* XOFF PAUSE emitted when;
pub const RX_PAUSE_THRESH_OFF_SHIFT: c_int = 0;
pub const RX_PAUSE_THRESH_ON_MASK: c_uint = 0x001FF000 /* XON PAUSE emitted after;
pub const RX_PAUSE_THRESH_ON_SHIFT: c_int = 12;
// 13-bit register used to control RX desc fetching and intr generation. if 4+
// valid RX descriptors are available, Cassini will read 4 at a time.
// writing N means that all desc up to *but* excluding N are available. N must
// be a multiple of 4 (N % 4 = 0). first desc should be cache-line aligned.
// DEFAULT: 0 on reset
//
pub const REG_RX_KICK: c_uint = 0x4024  /* RX kick reg */;
// 8KB aligned 64-bit pointer to the base of the RX free/completion rings.
// lower 13 bits of the low register are hard-wired to 0.
//
pub const REG_RX_DB_LOW: c_uint = 0x4028  /* RX descriptor ring;
pub const REG_RX_DB_HI: c_uint = 0x402C  /* RX descriptor ring;
pub const REG_RX_CB_LOW: c_uint = 0x4030  /* RX completion ring;
pub const REG_RX_CB_HI: c_uint = 0x4034  /* RX completion ring;
// 13-bit register indicate desc used by cassini for receive frames. used
// for diagnostic purposes.
// DEFAULT: 0 on reset
//
pub const REG_RX_COMP: c_uint = 0x4038  /* (ro) RX completion */;
// HEAD and TAIL are used to control RX desc posting and interrupt
// generation.  hw moves the head register to pass ownership to sw. sw
// moves the tail register to pass ownership back to hw. to give all
// entries to hw, set TAIL = HEAD.  if HEAD and TAIL indicate that no
// more entries are available, DMA will pause and an interrupt will be
// generated to indicate no more entries are available.  sw can use
// this interrupt to reduce the # of times it must update the
// completion tail register.
// DEFAULT: 0 on reset
//
pub const REG_RX_COMP_HEAD: c_uint = 0x403C  /* RX completion head */;
pub const REG_RX_COMP_TAIL: c_uint = 0x4040  /* RX completion tail */;
// values used for receive interrupt blanking. loaded each time the ISR is read
// DEFAULT: 0x00000000
//
pub const REG_RX_BLANK: c_uint = 0x4044  /* RX blanking register;
pub const RX_BLANK_INTR_PKT_MASK: c_uint = 0x000001FF /* RX_DONE intr asserted if;
pub const RX_BLANK_INTR_PKT_SHIFT: c_int = 0;
pub const RX_BLANK_INTR_TIME_MASK: c_uint = 0x3FFFF000 /* RX_DONE interrupt asserted;
pub const RX_BLANK_INTR_TIME_SHIFT: c_int = 12;
// values used for interrupt generation based on threshold values of how
// many free desc and completion entries are available for hw use.
// DEFAULT: 0x00000000
//
pub const REG_RX_AE_THRESH: c_uint = 0x4048  /* RX almost empty;
pub const RX_AE_THRESH_FREE_MASK: c_uint = 0x00001FFF /* RX_BUF_AE will be;

pub const RX_AE_THRESH_FREE_SHIFT: c_int = 0;
pub const RX_AE_THRESH_COMP_MASK: c_uint = 0x0FFFE000 /* RX_COMP_AE will be;

pub const RX_AE_THRESH_COMP_SHIFT: c_int = 13;
// probabilities for random early drop (RED) thresholds on a FIFO threshold
// basis. probability should increase when the FIFO level increases. control
// packets are never dropped and not counted in stats. probability programmed
// on a 12.5% granularity. e.g., 0x1 = 1/8 packets dropped.
// DEFAULT: 0x00000000
//
pub const REG_RX_RED: c_uint = 0x404C  /* RX random early detect enable */;
pub const RX_RED_4K_6K_FIFO_MASK: c_uint = 0x000000FF /*  4KB < FIFO thresh < 6KB */;
pub const RX_RED_6K_8K_FIFO_MASK: c_uint = 0x0000FF00 /*  6KB < FIFO thresh < 8KB */;
pub const RX_RED_8K_10K_FIFO_MASK: c_uint = 0x00FF0000 /*  8KB < FIFO thresh < 10KB */;
pub const RX_RED_10K_12K_FIFO_MASK: c_uint = 0xFF000000 /* 10KB < FIFO thresh < 12KB */;
// FIFO fullness levels for RX FIFO, RX control FIFO, and RX IPP FIFO.
// RX control FIFO = # of packets in RX FIFO.
// DEFAULT: 0x0
//
pub const REG_RX_FIFO_FULLNESS: c_uint = 0x4050  /* (ro) RX FIFO fullness */;
pub const RX_FIFO_FULLNESS_RX_FIFO_MASK: c_uint = 0x3FF80000 /* level w/ 8B granularity */;
pub const RX_FIFO_FULLNESS_IPP_FIFO_MASK: c_uint = 0x0007FF00 /* level w/ 8B granularity */;
pub const RX_FIFO_FULLNESS_RX_PKT_MASK: c_uint = 0x000000FF /* # packets in RX FIFO */;
pub const REG_RX_IPP_PACKET_COUNT: c_uint = 0x4054  /* RX IPP packet counter */;
pub const REG_RX_WORK_DMA_PTR_LOW: c_uint = 0x4058  /* RX working DMA ptr low */;
pub const REG_RX_WORK_DMA_PTR_HI: c_uint = 0x405C  /* RX working DMA ptr;
// BIST testing ro RX FIFO, RX control FIFO, and RX IPP FIFO. only RX BIST
// START/COMPLETE is writeable. START will clear when the BIST has completed
// checking all 17 RAMS.
// DEFAULT: 0bxxxx xxxxx xxxx xxxx xxxx x000 0000 0000 00x0
//
pub const REG_RX_BIST: c_uint = 0x4060  /* (ro) RX BIST */;
pub const RX_BIST_32A_PASS: c_uint = 0x80000000 /* RX FIFO 32A passed */;
pub const RX_BIST_33A_PASS: c_uint = 0x40000000 /* RX FIFO 33A passed */;
pub const RX_BIST_32B_PASS: c_uint = 0x20000000 /* RX FIFO 32B passed */;
pub const RX_BIST_33B_PASS: c_uint = 0x10000000 /* RX FIFO 33B passed */;
pub const RX_BIST_32C_PASS: c_uint = 0x08000000 /* RX FIFO 32C passed */;
pub const RX_BIST_33C_PASS: c_uint = 0x04000000 /* RX FIFO 33C passed */;
pub const RX_BIST_IPP_32A_PASS: c_uint = 0x02000000 /* RX IPP FIFO 33B passed */;
pub const RX_BIST_IPP_33A_PASS: c_uint = 0x01000000 /* RX IPP FIFO 33A passed */;
pub const RX_BIST_IPP_32B_PASS: c_uint = 0x00800000 /* RX IPP FIFO 32B passed */;
pub const RX_BIST_IPP_33B_PASS: c_uint = 0x00400000 /* RX IPP FIFO 33B passed */;
pub const RX_BIST_IPP_32C_PASS: c_uint = 0x00200000 /* RX IPP FIFO 32C passed */;
pub const RX_BIST_IPP_33C_PASS: c_uint = 0x00100000 /* RX IPP FIFO 33C passed */;
pub const RX_BIST_CTRL_32_PASS: c_uint = 0x00800000 /* RX CTRL FIFO 32 passed */;
pub const RX_BIST_CTRL_33_PASS: c_uint = 0x00400000 /* RX CTRL FIFO 33 passed */;
pub const RX_BIST_REAS_26A_PASS: c_uint = 0x00200000 /* RX Reas 26A passed */;
pub const RX_BIST_REAS_26B_PASS: c_uint = 0x00100000 /* RX Reas 26B passed */;
pub const RX_BIST_REAS_27_PASS: c_uint = 0x00080000 /* RX Reas 27 passed */;
pub const RX_BIST_STATE_MASK: c_uint = 0x00078000 /* BIST state machine */;
pub const RX_BIST_SUMMARY: c_uint = 0x00000002 /* when BIST complete,;
pub const RX_BIST_START: c_uint = 0x00000001 /* write 1 to start;
// next location in RX CTRL FIFO that will be loaded w/ data from RX IPP/read
// from to retrieve packet control info.
// DEFAULT: 0
//
pub const REG_RX_CTRL_FIFO_WRITE_PTR: c_uint = 0x4064  /* (ro) RX control FIFO;
pub const REG_RX_CTRL_FIFO_READ_PTR: c_uint = 0x4068  /* (ro) RX control FIFO read;
// receive interrupt blanking. loaded each time interrupt alias register is
// read.
// DEFAULT: 0x0
//
pub const REG_RX_BLANK_ALIAS_READ: c_uint = 0x406C  /* RX blanking register for;
pub const RX_BAR_INTR_PACKET_MASK: c_uint = 0x000001FF /* assert RX_DONE if #;
pub const RX_BAR_INTR_TIME_MASK: c_uint = 0x3FFFF000 /* assert RX_DONE if #;
// diagnostic access to RX FIFO. 32 LSB accessed via DATA_LOW. 32 MSB accessed
// via DATA_HI_T0 or DATA_HI_T1. TAG reads the tag bit. writing HI_T0
// will unset the tag bit while writing HI_T1 will set the tag bit. to reset
// to normal operation after diagnostics, write to address location 0x0.
// RX_DMA_EN bit must be set to 0x0 for RX FIFO PIO access. DATA_HI should
// be the last write access of a write sequence.
// DEFAULT: undefined
//
pub const REG_RX_FIFO_ADDR: c_uint = 0x4080  /* RX FIFO address */;
pub const REG_RX_FIFO_TAG: c_uint = 0x4084  /* RX FIFO tag */;
pub const REG_RX_FIFO_DATA_LOW: c_uint = 0x4088  /* RX FIFO data low */;
pub const REG_RX_FIFO_DATA_HI_T0: c_uint = 0x408C  /* RX FIFO data high T0 */;
pub const REG_RX_FIFO_DATA_HI_T1: c_uint = 0x4090  /* RX FIFO data high T1 */;
// diagnostic assess to RX CTRL FIFO. 8-bit FIFO_ADDR holds address of
// 81 bit control entry and 6 bit flow id. LOW and MID are both 32-bit
// accesses. HI is 7-bits with 6-bit flow id and 1 bit control
// word. RX_DMA_EN must be 0 for RX CTRL FIFO PIO access. DATA_HI
// should be last write access of the write sequence.
// DEFAULT: undefined
//
pub const REG_RX_CTRL_FIFO_ADDR: c_uint = 0x4094  /* RX Control FIFO and;
pub const REG_RX_CTRL_FIFO_DATA_LOW: c_uint = 0x4098  /* RX Control FIFO data;
pub const REG_RX_CTRL_FIFO_DATA_MID: c_uint = 0x409C  /* RX Control FIFO data;
pub const REG_RX_CTRL_FIFO_DATA_HI: c_uint = 0x4100  /* RX Control FIFO data;
pub const RX_CTRL_FIFO_DATA_HI_CTRL: c_uint = 0x0001  /* upper bit of ctrl word */;
pub const RX_CTRL_FIFO_DATA_HI_FLOW_MASK: c_uint = 0x007E  /* flow id */;
// diagnostic access to RX IPP FIFO. same semantics as RX_FIFO.
// DEFAULT: undefined
//
pub const REG_RX_IPP_FIFO_ADDR: c_uint = 0x4104  /* RX IPP FIFO address */;
pub const REG_RX_IPP_FIFO_TAG: c_uint = 0x4108  /* RX IPP FIFO tag */;
pub const REG_RX_IPP_FIFO_DATA_LOW: c_uint = 0x410C  /* RX IPP FIFO data low */;
pub const REG_RX_IPP_FIFO_DATA_HI_T0: c_uint = 0x4110  /* RX IPP FIFO data high;
pub const REG_RX_IPP_FIFO_DATA_HI_T1: c_uint = 0x4114  /* RX IPP FIFO data high;
// 64-bit pointer to receive data buffer in host memory used for headers and
// small packets. MSB in high register. loaded by DMA state machine and
// increments as DMA writes receive data. only 50 LSB are incremented. top
// 13 bits taken from RX descriptor.
// DEFAULT: undefined
//
pub const REG_RX_HEADER_PAGE_PTR_LOW: c_uint = 0x4118  /* (ro) RX header page ptr;
pub const REG_RX_HEADER_PAGE_PTR_HI: c_uint = 0x411C  /* (ro) RX header page ptr;
pub const REG_RX_MTU_PAGE_PTR_LOW: c_uint = 0x4120  /* (ro) RX MTU page pointer;
pub const REG_RX_MTU_PAGE_PTR_HI: c_uint = 0x4124  /* (ro) RX MTU page pointer;
// PIO diagnostic access to RX reassembly DMA Table RAM. 6-bit register holds
// one of 64 79-bit locations in the RX Reassembly DMA table and the addr of
// one of the 64 byte locations in the Batching table. LOW holds 32 LSB.
// MID holds the next 32 LSB. HIGH holds the 15 MSB. RX_DMA_EN must be set
// to 0 for PIO access. DATA_HIGH should be last write of write sequence.
// layout:
// reassmbl ptr [78:15] | reassmbl index [14:1] | reassmbl entry valid [0]
// DEFAULT: undefined
//
pub const REG_RX_TABLE_ADDR: c_uint = 0x4128  /* RX reassembly DMA table;
pub const RX_TABLE_ADDR_MASK: c_uint = 0x0000003F /* address mask */;
pub const REG_RX_TABLE_DATA_LOW: c_uint = 0x412C  /* RX reassembly DMA table;
pub const REG_RX_TABLE_DATA_MID: c_uint = 0x4130  /* RX reassembly DMA table;
pub const REG_RX_TABLE_DATA_HI: c_uint = 0x4134  /* RX reassembly DMA table;
// cassini+ only
// 8KB aligned 64-bit pointer to base of RX rings. lower 13 bits hardwired to
// 0. same semantics as primary desc/complete rings.
//
pub const REG_PLUS_RX_DB1_LOW: c_uint = 0x4200  /* RX descriptor ring;
pub const REG_PLUS_RX_DB1_HI: c_uint = 0x4204  /* RX descriptor ring;
pub const REG_PLUS_RX_CB1_LOW: c_uint = 0x4208  /* RX completion ring;
pub const REG_PLUS_RX_CB1_HI: c_uint = 0x420C  /* RX completion ring;

pub const REG_PLUS_RX_KICK1: c_uint = 0x4220  /* RX Kick 2 register */;
pub const REG_PLUS_RX_COMP1: c_uint = 0x4224  /* (ro) RX completion 2;
pub const REG_PLUS_RX_COMP1_HEAD: c_uint = 0x4228  /* (ro) RX completion 2;
pub const REG_PLUS_RX_COMP1_TAIL: c_uint = 0x422C  /* RX completion 2;

pub const REG_PLUS_RX_AE1_THRESH: c_uint = 0x4240  /* RX almost empty 2;

// header parser registers
// RX parser configuration register.
// DEFAULT: 0x1651004
//
pub const REG_HP_CFG: c_uint = 0x4140  /* header parser;
pub const HP_CFG_PARSE_EN: c_uint = 0x00000001 /* enab header parsing */;
pub const HP_CFG_NUM_CPU_MASK: c_uint = 0x000000FC /* # processors;
pub const HP_CFG_NUM_CPU_SHIFT: c_int = 2;
pub const HP_CFG_SYN_INC_MASK: c_uint = 0x00000100 /* SYN bit won't increment;
pub const HP_CFG_TCP_THRESH_MASK: c_uint = 0x000FFE00 /* # bytes of TCP data;
pub const HP_CFG_TCP_THRESH_SHIFT: c_int = 9;
// access to RX Instruction RAM. 5-bit register/counter holds addr
// of 39 bit entry to be read/written. 32 LSB in _DATA_LOW. 7 MSB in _DATA_HI.
// RX_DMA_EN must be 0 for RX instr PIO access. DATA_HI should be last access
// of sequence.
// DEFAULT: undefined
//
pub const REG_HP_INSTR_RAM_ADDR: c_uint = 0x4144  /* HP instruction RAM;
pub const HP_INSTR_RAM_ADDR_MASK: c_uint = 0x01F   /* 5-bit mask */;
pub const REG_HP_INSTR_RAM_DATA_LOW: c_uint = 0x4148  /* HP instruction RAM;
pub const HP_INSTR_RAM_LOW_OUTMASK_MASK: c_uint = 0x0000FFFF;
pub const HP_INSTR_RAM_LOW_OUTMASK_SHIFT: c_int = 0;
pub const HP_INSTR_RAM_LOW_OUTSHIFT_MASK: c_uint = 0x000F0000;
pub const HP_INSTR_RAM_LOW_OUTSHIFT_SHIFT: c_int = 16;
pub const HP_INSTR_RAM_LOW_OUTEN_MASK: c_uint = 0x00300000;
pub const HP_INSTR_RAM_LOW_OUTEN_SHIFT: c_int = 20;
pub const HP_INSTR_RAM_LOW_OUTARG_MASK: c_uint = 0xFFC00000;
pub const HP_INSTR_RAM_LOW_OUTARG_SHIFT: c_int = 22;
pub const REG_HP_INSTR_RAM_DATA_MID: c_uint = 0x414C  /* HP instruction RAM;
pub const HP_INSTR_RAM_MID_OUTARG_MASK: c_uint = 0x00000003;
pub const HP_INSTR_RAM_MID_OUTARG_SHIFT: c_int = 0;
pub const HP_INSTR_RAM_MID_OUTOP_MASK: c_uint = 0x0000003C;
pub const HP_INSTR_RAM_MID_OUTOP_SHIFT: c_int = 2;
pub const HP_INSTR_RAM_MID_FNEXT_MASK: c_uint = 0x000007C0;
pub const HP_INSTR_RAM_MID_FNEXT_SHIFT: c_int = 6;
pub const HP_INSTR_RAM_MID_FOFF_MASK: c_uint = 0x0003F800;
pub const HP_INSTR_RAM_MID_FOFF_SHIFT: c_int = 11;
pub const HP_INSTR_RAM_MID_SNEXT_MASK: c_uint = 0x007C0000;
pub const HP_INSTR_RAM_MID_SNEXT_SHIFT: c_int = 18;
pub const HP_INSTR_RAM_MID_SOFF_MASK: c_uint = 0x3F800000;
pub const HP_INSTR_RAM_MID_SOFF_SHIFT: c_int = 23;
pub const HP_INSTR_RAM_MID_OP_MASK: c_uint = 0xC0000000;
pub const HP_INSTR_RAM_MID_OP_SHIFT: c_int = 30;
pub const REG_HP_INSTR_RAM_DATA_HI: c_uint = 0x4150  /* HP instruction RAM;
pub const HP_INSTR_RAM_HI_VAL_MASK: c_uint = 0x0000FFFF;
pub const HP_INSTR_RAM_HI_VAL_SHIFT: c_int = 0;
pub const HP_INSTR_RAM_HI_MASK_MASK: c_uint = 0xFFFF0000;
pub const HP_INSTR_RAM_HI_MASK_SHIFT: c_int = 16;
// PIO access into RX Header parser data RAM and flow database.
// 11-bit register. Data fills the LSB portion of bus if less than 32 bits.
// DATA_RAM: write RAM_FDB_DATA with index to access DATA_RAM.
// RAM bytes = 4*(x - 1) + [3:0]. e.g., 0 -> [3:0], 31 -> [123:120]
// FLOWDB: write DATA_RAM_FDB register and then read/write FDB1-12 to access
// flow database.
// RX_DMA_EN must be 0 for RX parser RAM PIO access. RX Parser RAM data reg
// should be the last write access of the write sequence.
// DEFAULT: undefined
//
pub const REG_HP_DATA_RAM_FDB_ADDR: c_uint = 0x4154  /* HP data and FDB;
pub const HP_DATA_RAM_FDB_DATA_MASK: c_uint = 0x001F  /* select 1 of 86 byte;
pub const HP_DATA_RAM_FDB_FDB_MASK: c_uint = 0x3F00  /* 1 of 64 353-bit locations;
pub const REG_HP_DATA_RAM_DATA: c_uint = 0x4158  /* HP data RAM data */;
// HP flow database registers: 1 - 12, 0x415C - 0x4188, 4 8-bit bytes
// FLOW_DB(1) = IP_SA[127:96], FLOW_DB(2) = IP_SA[95:64]
// FLOW_DB(3) = IP_SA[63:32],  FLOW_DB(4) = IP_SA[31:0]
// FLOW_DB(5) = IP_DA[127:96], FLOW_DB(6) = IP_DA[95:64]
// FLOW_DB(7) = IP_DA[63:32],  FLOW_DB(8) = IP_DA[31:0]
// FLOW_DB(9) = {TCP_SP[15:0],TCP_DP[15:0]}
// FLOW_DB(10) = bit 0 has value for flow valid
// FLOW_DB(11) = TCP_SEQ[63:32], FLOW_DB(12) = TCP_SEQ[31:0]
//
pub const REG_HP_FLOW_DB0: c_uint = 0x415C  /* HP flow database 1 reg */;

// diagnostics for RX Header Parser block.
// ASUN: the header parser state machine register is used for diagnostics
// purposes. however, the spec doesn't have any details on it.
//
pub const REG_HP_STATE_MACHINE: c_uint = 0x418C  /* (ro) HP state machine */;
pub const REG_HP_STATUS0: c_uint = 0x4190  /* (ro) HP status 1 */;
pub const HP_STATUS0_SAP_MASK: c_uint = 0xFFFF0000 /* SAP */;
pub const HP_STATUS0_L3_OFF_MASK: c_uint = 0x0000FE00 /* L3 offset */;
pub const HP_STATUS0_LB_CPUNUM_MASK: c_uint = 0x000001F8 /* load balancing CPU;
pub const HP_STATUS0_HRP_OPCODE_MASK: c_uint = 0x00000007 /* HRP opcode */;
pub const REG_HP_STATUS1: c_uint = 0x4194  /* (ro) HP status 2 */;
pub const HP_STATUS1_ACCUR2_MASK: c_uint = 0xE0000000 /* accu R2[6:4] */;
pub const HP_STATUS1_FLOWID_MASK: c_uint = 0x1F800000 /* flow id */;
pub const HP_STATUS1_TCP_OFF_MASK: c_uint = 0x007F0000 /* tcp payload offset */;
pub const HP_STATUS1_TCP_SIZE_MASK: c_uint = 0x0000FFFF /* tcp payload size */;
pub const REG_HP_STATUS2: c_uint = 0x4198  /* (ro) HP status 3 */;
pub const HP_STATUS2_ACCUR2_MASK: c_uint = 0xF0000000 /* accu R2[3:0] */;
pub const HP_STATUS2_CSUM_OFF_MASK: c_uint = 0x07F00000 /* checksum start;
pub const HP_STATUS2_ACCUR1_MASK: c_uint = 0x000FE000 /* accu R1 */;
pub const HP_STATUS2_FORCE_DROP: c_uint = 0x00001000 /* force drop */;
pub const HP_STATUS2_BWO_REASSM: c_uint = 0x00000800 /* batching w/o;
pub const HP_STATUS2_JH_SPLIT_EN: c_uint = 0x00000400 /* jumbo header split;
pub const HP_STATUS2_FORCE_TCP_NOCHECK: c_uint = 0x00000200 /* force tcp no payload;
pub const HP_STATUS2_DATA_MASK_ZERO: c_uint = 0x00000100 /* mask of data length;
pub const HP_STATUS2_FORCE_TCP_CHECK: c_uint = 0x00000080 /* force tcp payload;
pub const HP_STATUS2_MASK_TCP_THRESH: c_uint = 0x00000040 /* mask of payload;
pub const HP_STATUS2_NO_ASSIST: c_uint = 0x00000020 /* no assist */;
pub const HP_STATUS2_CTRL_PACKET_FLAG: c_uint = 0x00000010 /* control packet flag */;
pub const HP_STATUS2_TCP_FLAG_CHECK: c_uint = 0x00000008 /* tcp flag check */;
pub const HP_STATUS2_SYN_FLAG: c_uint = 0x00000004 /* syn flag */;
pub const HP_STATUS2_TCP_CHECK: c_uint = 0x00000002 /* tcp payload chk */;
pub const HP_STATUS2_TCP_NOCHECK: c_uint = 0x00000001 /* tcp no payload chk */;
// BIST for header parser(HP) and flow database memories (FDBM). set _START
// to start BIST. controller clears _START on completion. _START can also
// be cleared to force termination of BIST. a bit set indicates that that
// memory passed its BIST.
//
pub const REG_HP_RAM_BIST: c_uint = 0x419C  /* HP RAM BIST reg */;
pub const HP_RAM_BIST_HP_DATA_PASS: c_uint = 0x80000000 /* HP data ram */;
pub const HP_RAM_BIST_HP_INSTR0_PASS: c_uint = 0x40000000 /* HP instr ram 0 */;
pub const HP_RAM_BIST_HP_INSTR1_PASS: c_uint = 0x20000000 /* HP instr ram 1 */;
pub const HP_RAM_BIST_HP_INSTR2_PASS: c_uint = 0x10000000 /* HP instr ram 2 */;
pub const HP_RAM_BIST_FDBM_AGE0_PASS: c_uint = 0x08000000 /* FDBM aging RAM0 */;
pub const HP_RAM_BIST_FDBM_AGE1_PASS: c_uint = 0x04000000 /* FDBM aging RAM1 */;
pub const HP_RAM_BIST_FDBM_FLOWID00_PASS: c_uint = 0x02000000 /* FDBM flowid RAM0;
pub const HP_RAM_BIST_FDBM_FLOWID10_PASS: c_uint = 0x01000000 /* FDBM flowid RAM1;
pub const HP_RAM_BIST_FDBM_FLOWID20_PASS: c_uint = 0x00800000 /* FDBM flowid RAM2;
pub const HP_RAM_BIST_FDBM_FLOWID30_PASS: c_uint = 0x00400000 /* FDBM flowid RAM3;
pub const HP_RAM_BIST_FDBM_FLOWID01_PASS: c_uint = 0x00200000 /* FDBM flowid RAM0;
pub const HP_RAM_BIST_FDBM_FLOWID11_PASS: c_uint = 0x00100000 /* FDBM flowid RAM1;
pub const HP_RAM_BIST_FDBM_FLOWID21_PASS: c_uint = 0x00080000 /* FDBM flowid RAM2;
pub const HP_RAM_BIST_FDBM_FLOWID31_PASS: c_uint = 0x00040000 /* FDBM flowid RAM3;
pub const HP_RAM_BIST_FDBM_TCPSEQ_PASS: c_uint = 0x00020000 /* FDBM tcp sequence;
pub const HP_RAM_BIST_SUMMARY: c_uint = 0x00000002 /* all BIST tests */;
pub const HP_RAM_BIST_START: c_uint = 0x00000001 /* start/stop BIST */;
// MAC registers.
// reset bits are set using a PIO write and self-cleared after the command
// execution has completed.
//
pub const REG_MAC_TX_RESET: c_uint = 0x6000  /* TX MAC software reset;
pub const REG_MAC_RX_RESET: c_uint = 0x6004  /* RX MAC software reset;
// execute a pause flow control frame transmission
pub const REG_MAC_SEND_PAUSE: c_uint = 0x6008  /* send pause command reg */;
pub const MAC_SEND_PAUSE_TIME_MASK: c_uint = 0x0000FFFF /* value of pause time;
pub const MAC_SEND_PAUSE_SEND: c_uint = 0x00010000 /* send pause flow ctrl;
// bit set indicates that event occurred. auto-cleared when status register
// is read and have corresponding mask bits in mask register. events will
// trigger an interrupt if the corresponding mask bit is 0.
// status register default: 0x00000000
// mask register default = 0xFFFFFFFF on reset
//
pub const REG_MAC_TX_STATUS: c_uint = 0x6010  /* TX MAC status reg */;
pub const MAC_TX_FRAME_XMIT: c_uint = 0x0001  /* successful frame;
pub const MAC_TX_UNDERRUN: c_uint = 0x0002  /* terminated frame;
pub const MAC_TX_MAX_PACKET_ERR: c_uint = 0x0004  /* frame exceeds max allowed;
pub const MAC_TX_COLL_NORMAL: c_uint = 0x0008  /* rollover of the normal;
pub const MAC_TX_COLL_EXCESS: c_uint = 0x0010  /* rollover of the excessive;
pub const MAC_TX_COLL_LATE: c_uint = 0x0020  /* rollover of the late;
pub const MAC_TX_COLL_FIRST: c_uint = 0x0040  /* rollover of the first;
pub const MAC_TX_DEFER_TIMER: c_uint = 0x0080  /* rollover of the defer;
pub const MAC_TX_PEAK_ATTEMPTS: c_uint = 0x0100  /* rollover of the peak;
pub const REG_MAC_RX_STATUS: c_uint = 0x6014  /* RX MAC status reg */;
pub const MAC_RX_FRAME_RECV: c_uint = 0x0001  /* successful receipt of;
pub const MAC_RX_OVERFLOW: c_uint = 0x0002  /* dropped frame due to;
pub const MAC_RX_FRAME_COUNT: c_uint = 0x0004  /* rollover of receive frame;
pub const MAC_RX_ALIGN_ERR: c_uint = 0x0008  /* rollover of alignment;
pub const MAC_RX_CRC_ERR: c_uint = 0x0010  /* rollover of crc error;
pub const MAC_RX_LEN_ERR: c_uint = 0x0020  /* rollover of length;
pub const MAC_RX_VIOL_ERR: c_uint = 0x0040  /* rollover of code;
// DEFAULT: 0xXXXX0000 on reset
pub const REG_MAC_CTRL_STATUS: c_uint = 0x6018  /* MAC control status reg */;
pub const MAC_CTRL_PAUSE_RECEIVED: c_uint = 0x00000001  /* successful;
pub const MAC_CTRL_PAUSE_STATE: c_uint = 0x00000002  /* MAC has made a;
pub const MAC_CTRL_NOPAUSE_STATE: c_uint = 0x00000004  /* MAC has made a;
pub const MAC_CTRL_PAUSE_TIME_MASK: c_uint = 0xFFFF0000  /* value of pause time;
// layout identical to TX MAC[8:0]
pub const REG_MAC_TX_MASK: c_uint = 0x6020  /* TX MAC mask reg */;
// layout identical to RX MAC[6:0]
pub const REG_MAC_RX_MASK: c_uint = 0x6024  /* RX MAC mask reg */;
// layout identical to CTRL MAC[2:0]
pub const REG_MAC_CTRL_MASK: c_uint = 0x6028  /* MAC control mask reg */;
// to ensure proper operation, CFG_EN must be cleared to 0 and a delay
// imposed before writes to other bits in the TX_MAC_CFG register or any of
// the MAC parameters is performed. delay dependent upon time required to
// transmit a maximum size frame (= MAC_FRAMESIZE_MAX*8/Mbps). e.g.,
// the delay for a 1518-byte frame on a 100Mbps network is 125us.
// alternatively, just poll TX_CFG_EN until it reads back as 0.
// NOTE: on half-duplex 1Gbps, TX_CFG_CARRIER_EXTEND and
// RX_CFG_CARRIER_EXTEND should be set and the SLOT_TIME register should
// be 0x200 (slot time of 512 bytes)
//
pub const REG_MAC_TX_CFG: c_uint = 0x6030  /* TX MAC config reg */;
pub const MAC_TX_CFG_EN: c_uint = 0x0001  /* enable TX MAC. 0 will;
pub const MAC_TX_CFG_IGNORE_CARRIER: c_uint = 0x0002  /* disable CSMA/CD deferral;
pub const MAC_TX_CFG_IGNORE_COLL: c_uint = 0x0004  /* disable CSMA/CD backoff;
pub const MAC_TX_CFG_IPG_EN: c_uint = 0x0008  /* enable extension of the;
pub const MAC_TX_CFG_NEVER_GIVE_UP_EN: c_uint = 0x0010  /* TX MAC will not easily;
pub const MAC_TX_CFG_NEVER_GIVE_UP_LIM: c_uint = 0x0020  /* when set, TX MAC will;
pub const MAC_TX_CFG_NO_BACKOFF: c_uint = 0x0040  /* modify CSMA/CD to disable;
pub const MAC_TX_CFG_SLOW_DOWN: c_uint = 0x0080  /* modify CSMA/CD so that;
pub const MAC_TX_CFG_NO_FCS: c_uint = 0x0100  /* TX MAC will not generate;
pub const MAC_TX_CFG_CARRIER_EXTEND: c_uint = 0x0200  /* enables xmit part of the;
// when CRC is not stripped, reassembly packets will not contain the CRC.
// these will be stripped by HRP because it reassembles layer 4 data, and the
// CRC is layer 2. however, non-reassembly packets will still contain the CRC
// when passed to the host. to ensure proper operation, need to wait 3.2ms
// after clearing RX_CFG_EN before writing to any other RX MAC registers
// or other MAC parameters. alternatively, poll RX_CFG_EN until it clears
// to 0. Similarly, HASH_FILTER_EN and ADDR_FILTER_EN have the same
// restrictions as CFG_EN.
//
pub const REG_MAC_RX_CFG: c_uint = 0x6034  /* RX MAC config reg */;
pub const MAC_RX_CFG_EN: c_uint = 0x0001  /* enable RX MAC */;
pub const MAC_RX_CFG_STRIP_PAD: c_uint = 0x0002  /* always program to 0.;
pub const MAC_RX_CFG_STRIP_FCS: c_uint = 0x0004  /* RX MAC will strip the;
pub const MAC_RX_CFG_PROMISC_EN: c_uint = 0x0008  /* promiscuous mode */;
pub const MAC_RX_CFG_PROMISC_GROUP_EN: c_uint = 0x0010  /* accept all valid;
pub const MAC_RX_CFG_HASH_FILTER_EN: c_uint = 0x0020  /* use hash table to filter;
pub const MAC_RX_CFG_ADDR_FILTER_EN: c_uint = 0x0040  /* cause RX MAC to use;
pub const MAC_RX_CFG_DISABLE_DISCARD: c_uint = 0x0080  /* pass errored frames to;
pub const MAC_RX_CFG_CARRIER_EXTEND: c_uint = 0x0100  /* enable reception of;
// DEFAULT: 0x0
pub const REG_MAC_CTRL_CFG: c_uint = 0x6038  /* MAC control config reg */;
pub const MAC_CTRL_CFG_SEND_PAUSE_EN: c_uint = 0x0001  /* respond to requests for;
pub const MAC_CTRL_CFG_RECV_PAUSE_EN: c_uint = 0x0002  /* respond to received;
pub const MAC_CTRL_CFG_PASS_CTRL: c_uint = 0x0004  /* pass valid MAC ctrl;
// to ensure proper operation, a global initialization sequence should be
// performed when a loopback config is entered or exited. if programmed after
// a hw or global sw reset, RX/TX MAC software reset and initialization
// should be done to ensure stable clocking.
// DEFAULT: 0x0
//
pub const REG_MAC_XIF_CFG: c_uint = 0x603C  /* XIF config reg */;
pub const MAC_XIF_TX_MII_OUTPUT_EN: c_uint = 0x0001  /* enable output drivers;
pub const MAC_XIF_MII_INT_LOOPBACK: c_uint = 0x0002  /* loopback GMII xmit data;
pub const MAC_XIF_DISABLE_ECHO: c_uint = 0x0004  /* disables receive data;
pub const MAC_XIF_GMII_MODE: c_uint = 0x0008  /* MAC operates with GMII;
pub const MAC_XIF_MII_BUFFER_OUTPUT_EN: c_uint = 0x0010  /* MII_BUF_EN pin. enable;
pub const MAC_XIF_LINK_LED: c_uint = 0x0020  /* LINKLED# active (low) */;
pub const MAC_XIF_FDPLX_LED: c_uint = 0x0040  /* FDPLXLED# active (low) */;
pub const REG_MAC_IPG0: c_uint = 0x6040  /* inter-packet gap0 reg.;
pub const REG_MAC_IPG1: c_uint = 0x6044  /* inter-packet gap1 reg;
pub const REG_MAC_IPG2: c_uint = 0x6048  /* inter-packet gap2 reg;
pub const REG_MAC_SLOT_TIME: c_uint = 0x604C  /* slot time reg;
pub const REG_MAC_FRAMESIZE_MIN: c_uint = 0x6050  /* min frame size reg;
// FRAMESIZE_MAX holds both the max frame size as well as the max burst size.
// recommended value:  0x2000.05EE
//
pub const REG_MAC_FRAMESIZE_MAX: c_uint = 0x6054  /* max frame size reg */;
pub const MAC_FRAMESIZE_MAX_BURST_MASK: c_uint = 0x3FFF0000 /* max burst size */;
pub const MAC_FRAMESIZE_MAX_BURST_SHIFT: c_int = 16;
pub const MAC_FRAMESIZE_MAX_FRAME_MASK: c_uint = 0x00007FFF /* max frame size */;
pub const MAC_FRAMESIZE_MAX_FRAME_SHIFT: c_int = 0;
pub const REG_MAC_PA_SIZE: c_uint = 0x6058  /* PA size reg. number of;
pub const REG_MAC_JAM_SIZE: c_uint = 0x605C  /* jam size reg. duration;
pub const REG_MAC_ATTEMPT_LIMIT: c_uint = 0x6060  /* attempt limit reg. #;
pub const REG_MAC_CTRL_TYPE: c_uint = 0x6064  /* MAC control type reg.;
// mac address registers: 0 - 44, 0x6080 - 0x6130, 4 8-bit bytes.
// register           contains                   comparison
// 0        16 MSB of primary MAC addr        [47:32] of DA field
// 1        16 middle bits ""                 [31:16] of DA field
// 2        16 LSB ""                         [15:0] of DA field
// 3*x      16MSB of alt MAC addr 1-15        [47:32] of DA field
// 4*x      16 middle bits ""                 [31:16]
// 5*x      16 LSB ""                         [15:0]
// 42       16 MSB of MAC CTRL addr           [47:32] of DA.
// 43       16 middle bits ""                 [31:16]
// 44       16 LSB ""                         [15:0]
// MAC CTRL addr must be the reserved multicast addr for MAC CTRL frames.
// if there is a match, MAC will set the bit for alternative address
// filter pass [15]
// here is the map of registers given MAC address notation: a:b:c:d:e:f
// ab             cd             ef
// primary addr     reg 2          reg 1          reg 0
// alt addr 1       reg 5          reg 4          reg 3
// alt addr x       reg 5*x        reg 4*x        reg 3*x
// ctrl addr        reg 44         reg 43         reg 42
//
pub const REG_MAC_ADDR0: c_uint = 0x6080  /* MAC address 0 reg */;

pub const REG_MAC_ADDR_FILTER0: c_uint = 0x614C  /* address filter 0 reg;
pub const REG_MAC_ADDR_FILTER1: c_uint = 0x6150  /* address filter 1 reg;
pub const REG_MAC_ADDR_FILTER2: c_uint = 0x6154  /* address filter 2 reg;
pub const REG_MAC_ADDR_FILTER2_1_MASK: c_uint = 0x6158  /* address filter 2 and 1;
pub const REG_MAC_ADDR_FILTER0_MASK: c_uint = 0x615C  /* address filter 0 mask;
// hash table registers: 0 - 15, 0x6160 - 0x619C, 4 8-bit bytes
// 16-bit registers contain bits of the hash table.
// reg x  -> [16*(15 - x) + 15 : 16*(15 - x)].
// e.g., 15 -> [15:0], 0 -> [255:240]
//
pub const REG_MAC_HASH_TABLE0: c_uint = 0x6160  /* hash table 0 reg */;

// statistics registers. these registers generate an interrupt on
// overflow. recommended initialization: 0x0000. most are 16-bits except
// for PEAK_ATTEMPTS register which is 8 bits.
//
pub const REG_MAC_COLL_NORMAL: c_uint = 0x61A0 /* normal collision;
pub const REG_MAC_COLL_FIRST: c_uint = 0x61A4 /* first attempt;
pub const REG_MAC_COLL_EXCESS: c_uint = 0x61A8 /* excessive collision;
pub const REG_MAC_COLL_LATE: c_uint = 0x61AC /* late collision counter */;
pub const REG_MAC_TIMER_DEFER: c_uint = 0x61B0 /* defer timer. time base;
pub const REG_MAC_ATTEMPTS_PEAK: c_uint = 0x61B4 /* peak attempts reg */;
pub const REG_MAC_RECV_FRAME: c_uint = 0x61B8 /* receive frame counter */;
pub const REG_MAC_LEN_ERR: c_uint = 0x61BC /* length error counter */;
pub const REG_MAC_ALIGN_ERR: c_uint = 0x61C0 /* alignment error counter */;
pub const REG_MAC_FCS_ERR: c_uint = 0x61C4 /* FCS error counter */;
pub const REG_MAC_RX_CODE_ERR: c_uint = 0x61C8 /* RX code violation;
// misc registers
pub const REG_MAC_RANDOM_SEED: c_uint = 0x61CC /* random number seed reg.;
// ASUN: there's a PAUSE_TIMER (ro) described, but it's not in the address
// map
//
// 27-bit register has the current state for key state machines in the MAC
pub const REG_MAC_STATE_MACHINE: c_uint = 0x61D0 /* (ro) state machine reg */;
pub const MAC_SM_RLM_MASK: c_uint = 0x07800000;
pub const MAC_SM_RLM_SHIFT: c_int = 23;
pub const MAC_SM_RX_FC_MASK: c_uint = 0x00700000;
pub const MAC_SM_RX_FC_SHIFT: c_int = 20;
pub const MAC_SM_TLM_MASK: c_uint = 0x000F0000;
pub const MAC_SM_TLM_SHIFT: c_int = 16;
pub const MAC_SM_ENCAP_SM_MASK: c_uint = 0x0000F000;
pub const MAC_SM_ENCAP_SM_SHIFT: c_int = 12;
pub const MAC_SM_TX_REQ_MASK: c_uint = 0x00000C00;
pub const MAC_SM_TX_REQ_SHIFT: c_int = 10;
pub const MAC_SM_TX_FC_MASK: c_uint = 0x000003C0;
pub const MAC_SM_TX_FC_SHIFT: c_int = 6;
pub const MAC_SM_FIFO_WRITE_SEL_MASK: c_uint = 0x00000038;
pub const MAC_SM_FIFO_WRITE_SEL_SHIFT: c_int = 3;
pub const MAC_SM_TX_FIFO_EMPTY_MASK: c_uint = 0x00000007;
pub const MAC_SM_TX_FIFO_EMPTY_SHIFT: c_int = 0;
// MIF registers. the MIF can be programmed in either bit-bang or
// frame mode.
//
pub const REG_MIF_BIT_BANG_CLOCK: c_uint = 0x6200 /* MIF bit-bang clock.;
pub const REG_MIF_BIT_BANG_DATA: c_uint = 0x6204 /* MIF bit-bang data. 1-bit;
pub const REG_MIF_BIT_BANG_OUTPUT_EN: c_uint = 0x6208 /* MIF bit-bang output;
// 32-bit register serves as an instruction register when the MIF is
// programmed in frame mode. load this register w/ a valid instruction
// (as per IEEE 802.3u MII spec). poll this register to check for instruction
// execution completion. during a read operation, this register will also
// contain the 16-bit data returned by the transceiver. unless specified
// otherwise, fields are considered "don't care" when polling for
// completion.
//
pub const REG_MIF_FRAME: c_uint = 0x620C /* MIF frame/output reg */;
pub const MIF_FRAME_START_MASK: c_uint = 0xC0000000 /* start of frame.;
pub const MIF_FRAME_ST: c_uint = 0x40000000 /* STart of frame */;
pub const MIF_FRAME_OPCODE_MASK: c_uint = 0x30000000 /* opcode. 01 for a;
pub const MIF_FRAME_OP_READ: c_uint = 0x20000000 /* read OPcode */;
pub const MIF_FRAME_OP_WRITE: c_uint = 0x10000000 /* write OPcode */;
pub const MIF_FRAME_PHY_ADDR_MASK: c_uint = 0x0F800000 /* phy address. when;
pub const MIF_FRAME_PHY_ADDR_SHIFT: c_int = 23;
pub const MIF_FRAME_REG_ADDR_MASK: c_uint = 0x007C0000 /* register address.;
pub const MIF_FRAME_REG_ADDR_SHIFT: c_int = 18;
pub const MIF_FRAME_TURN_AROUND_MSB: c_uint = 0x00020000 /* turn around, MSB.;
pub const MIF_FRAME_TURN_AROUND_LSB: c_uint = 0x00010000 /* turn around, LSB.;
pub const MIF_FRAME_DATA_MASK: c_uint = 0x0000FFFF /* instruction payload;
pub const REG_MIF_CFG: c_uint = 0x6210 /* MIF config reg */;
pub const MIF_CFG_PHY_SELECT: c_uint = 0x0001 /* 1 -> select MDIO_1;
pub const MIF_CFG_POLL_EN: c_uint = 0x0002 /* enable polling;
pub const MIF_CFG_BB_MODE: c_uint = 0x0004 /* 1 -> bit-bang mode;
pub const MIF_CFG_POLL_REG_MASK: c_uint = 0x00F8 /* register address to be;
pub const MIF_CFG_POLL_REG_SHIFT: c_int = 3;
pub const MIF_CFG_MDIO_0: c_uint = 0x0100 /* (ro) dual purpose.;
pub const MIF_CFG_MDIO_1: c_uint = 0x0200 /* (ro) dual purpose.;
pub const MIF_CFG_POLL_PHY_MASK: c_uint = 0x7C00 /* transceiver address to;
pub const MIF_CFG_POLL_PHY_SHIFT: c_int = 10;
// 16-bit register used to determine which bits in the POLL_STATUS portion of
// the MIF_STATUS register will cause an interrupt. if a mask bit is 0,
// corresponding bit of the POLL_STATUS will generate a MIF interrupt when
// set. DEFAULT: 0xFFFF
//
pub const REG_MIF_MASK: c_uint = 0x6214 /* MIF mask reg */;
// 32-bit register used when in poll mode. auto-cleared after being read
pub const REG_MIF_STATUS: c_uint = 0x6218 /* MIF status reg */;
pub const MIF_STATUS_POLL_DATA_MASK: c_uint = 0xFFFF0000 /* poll data contains;
pub const MIF_STATUS_POLL_DATA_SHIFT: c_int = 16;
pub const MIF_STATUS_POLL_STATUS_MASK: c_uint = 0x0000FFFF /* poll status indicates;
pub const MIF_STATUS_POLL_STATUS_SHIFT: c_int = 0;
// 7-bit register has current state for all state machines in the MIF
pub const REG_MIF_STATE_MACHINE: c_uint = 0x621C /* MIF state machine reg */;
pub const MIF_SM_CONTROL_MASK: c_uint = 0x07   /* control state machine;
pub const MIF_SM_EXECUTION_MASK: c_uint = 0x60   /* execution state machine;
// PCS/Serialink. the following registers are equivalent to the standard
// MII management registers except that they're directly mapped in
// Cassini's register space.
//
// the auto-negotiation enable bit should be programmed the same at
// the link partner as in the local device to enable auto-negotiation to
// complete. when that bit is reprogrammed, auto-neg/manual config is
// restarted automatically.
// DEFAULT: 0x1040
//
pub const REG_PCS_MII_CTRL: c_uint = 0x9000 /* PCS MII control reg */;
pub const PCS_MII_CTRL_1000_SEL: c_uint = 0x0040 /* reads 1. ignored on;
pub const PCS_MII_CTRL_COLLISION_TEST: c_uint = 0x0080 /* COL signal at the PCS;
pub const PCS_MII_CTRL_DUPLEX: c_uint = 0x0100 /* forced 0x0. PCS;
pub const PCS_MII_RESTART_AUTONEG: c_uint = 0x0200 /* self clearing.;
pub const PCS_MII_ISOLATE: c_uint = 0x0400 /* read as 0. ignored;
pub const PCS_MII_POWER_DOWN: c_uint = 0x0800 /* read as 0. ignored;
pub const PCS_MII_AUTONEG_EN: c_uint = 0x1000 /* default 1. PCS goes;
pub const PCS_MII_10_100_SEL: c_uint = 0x2000 /* read as 0. ignored on;
pub const PCS_MII_RESET: c_uint = 0x8000 /* reset PCS. self-clears;
// DEFAULT: 0x0108
pub const REG_PCS_MII_STATUS: c_uint = 0x9004 /* PCS MII status reg */;
pub const PCS_MII_STATUS_EXTEND_CAP: c_uint = 0x0001 /* reads 0 */;
pub const PCS_MII_STATUS_JABBER_DETECT: c_uint = 0x0002 /* reads 0 */;
pub const PCS_MII_STATUS_LINK_STATUS: c_uint = 0x0004 /* 1 -> link up.;
pub const PCS_MII_STATUS_AUTONEG_ABLE: c_uint = 0x0008 /* reads 1 (able to perform;
pub const PCS_MII_STATUS_REMOTE_FAULT: c_uint = 0x0010 /* 1 -> remote fault detected;
pub const PCS_MII_STATUS_AUTONEG_COMP: c_uint = 0x0020 /* 1 -> auto-negotiation;
pub const PCS_MII_STATUS_EXTEND_STATUS: c_uint = 0x0100 /* reads as 1. used as an;
// used during auto-negotiation.
// DEFAULT: 0x00E0
//
pub const REG_PCS_MII_ADVERT: c_uint = 0x9008 /* PCS MII advertisement;
pub const PCS_MII_ADVERT_FD: c_uint = 0x0020  /* advertise full duplex;
pub const PCS_MII_ADVERT_HD: c_uint = 0x0040  /* advertise half-duplex;
pub const PCS_MII_ADVERT_SYM_PAUSE: c_uint = 0x0080  /* advertise PAUSE;
pub const PCS_MII_ADVERT_ASYM_PAUSE: c_uint = 0x0100  /* advertises PAUSE;
pub const PCS_MII_ADVERT_RF_MASK: c_uint = 0x3000 /* remote fault. write bit13;
pub const PCS_MII_ADVERT_ACK: c_uint = 0x4000 /* (ro) */;
pub const PCS_MII_ADVERT_NEXT_PAGE: c_uint = 0x8000 /* (ro) forced 0x0 */;
// contents updated as a result of autonegotiation. layout and definitions
// identical to PCS_MII_ADVERT
//
pub const REG_PCS_MII_LPA: c_uint = 0x900C /* PCS MII link partner;

// DEFAULT: 0x0
pub const REG_PCS_CFG: c_uint = 0x9010 /* PCS config reg */;
pub const PCS_CFG_EN: c_uint = 0x01   /* enable PCS. must be;
pub const PCS_CFG_SD_OVERRIDE: c_uint = 0x02   /* sets signal detect to;
pub const PCS_CFG_SD_ACTIVE_LOW: c_uint = 0x04   /* changes interpretation;
pub const PCS_CFG_JITTER_STUDY_MASK: c_uint = 0x18   /* used to make jitter;
pub const PCS_CFG_10MS_TIMER_OVERRIDE: c_uint = 0x20   /* shortens 10-20ms auto-;
// used for diagnostic purposes. bits 20-22 autoclear on read
pub const REG_PCS_STATE_MACHINE: c_uint = 0x9014 /* (ro) PCS state machine;
pub const PCS_SM_TX_STATE_MASK: c_uint = 0x0000000F /* 0 and 1 indicate;
pub const PCS_SM_RX_STATE_MASK: c_uint = 0x000000F0 /* 0 indicates reception;
pub const PCS_SM_WORD_SYNC_STATE_MASK: c_uint = 0x00000700 /* 0 indicates loss of;
pub const PCS_SM_SEQ_DETECT_STATE_MASK: c_uint = 0x00001800 /* cycling through 0-3;
pub const PCS_SM_LINK_STATE_MASK: c_uint = 0x0001E000;
pub const SM_LINK_STATE_UP: c_uint = 0x00016000 /* link state is up */;
pub const PCS_SM_LOSS_LINK_C: c_uint = 0x00100000 /* loss of link due to;
pub const PCS_SM_LOSS_LINK_SYNC: c_uint = 0x00200000 /* loss of link due to;
pub const PCS_SM_LOSS_SIGNAL_DETECT: c_uint = 0x00400000 /* signal detect goes;
pub const PCS_SM_NO_LINK_BREAKLINK: c_uint = 0x01000000 /* link not up due to;
pub const PCS_SM_NO_LINK_SERDES: c_uint = 0x02000000 /* serdes being;
pub const PCS_SM_NO_LINK_C: c_uint = 0x04000000 /* C codes not stable or;
pub const PCS_SM_NO_LINK_SYNC: c_uint = 0x08000000 /* word sync not;
pub const PCS_SM_NO_LINK_WAIT_C: c_uint = 0x10000000 /* waiting for C codes;
pub const PCS_SM_NO_LINK_NO_IDLE: c_uint = 0x20000000 /* link partner continues;
// this register indicates interrupt changes in specific PCS MII status bits.
// PCS_INT may be masked at the ISR level. only a single bit is implemented
// for link status change.
//
pub const REG_PCS_INTR_STATUS: c_uint = 0x9018 /* PCS interrupt status */;
pub const PCS_INTR_STATUS_LINK_CHANGE: c_uint = 0x04   /* link status has changed;
// control which network interface is used. no more than one bit should
// be set.
// DEFAULT: none
//
pub const REG_PCS_DATAPATH_MODE: c_uint = 0x9050 /* datapath mode reg */;
pub const PCS_DATAPATH_MODE_MII: c_uint = 0x00 /* PCS is not used and;
pub const PCS_DATAPATH_MODE_SERDES: c_uint = 0x02 /* PCS is used via the;
// input to serdes chip or serialink block
pub const REG_PCS_SERDES_CTRL: c_uint = 0x9054 /* serdes control reg */;
pub const PCS_SERDES_CTRL_LOOPBACK: c_uint = 0x01   /* enable loopback on;
pub const PCS_SERDES_CTRL_SYNCD_EN: c_uint = 0x02   /* enable sync carrier;
pub const PCS_SERDES_CTRL_LOCKREF: c_uint = 0x04   /* frequency-lock RBC[0:1];
// multiplex test outputs into the PROM address (PA_3 through PA_0) pins.
// should be 0x0 for normal operations.
// 0b000          normal operation, PROM address[3:0] selected
// 0b001          rxdma req, rxdma ack, rxdma ready, rxdma read
// 0b010          rxmac req, rx ack, rx tag, rx clk shared
// 0b011          txmac req, tx ack, tx tag, tx retry req
// 0b100          tx tp3, tx tp2, tx tp1, tx tp0
// 0b101          R period RX, R period TX, R period HP, R period BIM
// DEFAULT: 0x0
//
pub const REG_PCS_SHARED_OUTPUT_SEL: c_uint = 0x9058 /* shared output select */;
pub const PCS_SOS_PROM_ADDR_MASK: c_uint = 0x0007;
// used for diagnostics. this register indicates progress of the SERDES
// boot up.
// 0b00       undergoing reset
// 0b01       waiting 500us while lockrefn is asserted
// 0b10       waiting for comma detect
// 0b11       receive data is synchronized
// DEFAULT: 0x0
//
pub const REG_PCS_SERDES_STATE: c_uint = 0x905C /* (ro) serdes state */;
pub const PCS_SERDES_STATE_MASK: c_uint = 0x03;
// used for diagnostics. indicates number of packets transmitted or received.
// counters rollover w/out generating an interrupt.
// DEFAULT: 0x0
//
pub const REG_PCS_PACKET_COUNT: c_uint = 0x9060 /* (ro) PCS packet counter */;
pub const PCS_PACKET_COUNT_TX: c_uint = 0x000007FF /* pkts xmitted by PCS */;
pub const PCS_PACKET_COUNT_RX: c_uint = 0x07FF0000 /* pkts recvd by PCS;
// LocalBus Devices. the following provides run-time access to the
// Cassini's PROM
//
pub const REG_EXPANSION_ROM_RUN_START: c_uint = 0x100000 /* expansion rom run time;
pub const REG_EXPANSION_ROM_RUN_END: c_uint = 0x17FFFF;
pub const REG_SECOND_LOCALBUS_START: c_uint = 0x180000 /* secondary local bus;
pub const REG_SECOND_LOCALBUS_END: c_uint = 0x1FFFFF;
// entropy device

pub const ENTROPY_STATUS_DRDY: c_uint = 0x01;
pub const ENTROPY_STATUS_BUSY: c_uint = 0x02;
pub const ENTROPY_STATUS_CIPHER: c_uint = 0x04;
pub const ENTROPY_STATUS_BYPASS_MASK: c_uint = 0x18;

pub const ENTROPY_MODE_KEY_MASK: c_uint = 0x07;
pub const ENTROPY_MODE_ENCRYPT: c_uint = 0x40;

pub const ENTROPY_RESET_DES_IO: c_uint = 0x01;
pub const ENTROPY_RESET_STC_MODE: c_uint = 0x02;
pub const ENTROPY_RESET_KEY_CACHE: c_uint = 0x04;
pub const ENTROPY_RESET_IV: c_uint = 0x08;

// phys of interest w/ their special mii registers
pub const PHY_LUCENT_B0: c_uint = 0x00437421;
pub const LUCENT_MII_REG: c_uint = 0x1F;
pub const PHY_NS_DP83065: c_uint = 0x20005c78;
pub const DP83065_MII_MEM: c_uint = 0x16;
pub const DP83065_MII_REGD: c_uint = 0x1D;
pub const DP83065_MII_REGE: c_uint = 0x1E;
pub const PHY_BROADCOM_5411: c_uint = 0x00206071;
pub const PHY_BROADCOM_B0: c_uint = 0x00206050;
pub const BROADCOM_MII_REG4: c_uint = 0x14;
pub const BROADCOM_MII_REG5: c_uint = 0x15;
pub const BROADCOM_MII_REG7: c_uint = 0x17;
pub const BROADCOM_MII_REG8: c_uint = 0x18;
pub const CAS_MII_ANNPTR: c_uint = 0x07;
pub const CAS_MII_ANNPRR: c_uint = 0x08;
pub const CAS_MII_1000_CTRL: c_uint = 0x09;
pub const CAS_MII_1000_STATUS: c_uint = 0x0A;
pub const CAS_MII_1000_EXTEND: c_uint = 0x0F;
pub const CAS_BMSR_1000_EXTEND: c_uint = 0x0100 /* supports 1000Base-T extended status */;
//
// if autoneg is disabled, here's the table:
// BMCR_SPEED100 = 100Mbps
// BMCR_SPEED1000 = 1000Mbps
// ~(BMCR_SPEED100 | BMCR_SPEED1000) = 10Mbps
//
pub const CAS_BMCR_SPEED1000: c_uint = 0x0040  /* Select 1000Mbps */;
pub const CAS_ADVERTISE_1000HALF: c_uint = 0x0100;
pub const CAS_ADVERTISE_1000FULL: c_uint = 0x0200;
pub const CAS_ADVERTISE_PAUSE: c_uint = 0x0400;
pub const CAS_ADVERTISE_ASYM_PAUSE: c_uint = 0x0800;
// regular lpa register

// 1000_STATUS register
pub const CAS_LPA_1000HALF: c_uint = 0x0400;
pub const CAS_LPA_1000FULL: c_uint = 0x0800;
pub const CAS_EXTEND_1000XFULL: c_uint = 0x8000;
pub const CAS_EXTEND_1000XHALF: c_uint = 0x4000;
pub const CAS_EXTEND_1000TFULL: c_uint = 0x2000;
pub const CAS_EXTEND_1000THALF: c_uint = 0x1000;
// cassini header parser firmware
// output info
// comparison

// output opcodes
pub const CL_REG: c_int = 0;
pub const LD_FID: c_int = 1;
pub const LD_SEQ: c_int = 2;
pub const LD_CTL: c_int = 3;
pub const LD_SAP: c_int = 4;
pub const LD_R1: c_int = 5;
pub const LD_L3: c_int = 6;
pub const LD_SUM: c_int = 7;
pub const LD_HDR: c_int = 8;
pub const IM_FID: c_int = 9;
pub const IM_SEQ: c_int = 10;
pub const IM_SAP: c_int = 11;
pub const IM_R1: c_int = 12;
pub const IM_CTL: c_int = 13;
pub const LD_LEN: c_int = 14;
pub const ST_FLG: c_int = 15;
// match setp #s for IP4TCP4
pub const S1_PCKT: c_int = 0;
pub const S1_VLAN: c_int = 1;
pub const S1_CFI: c_int = 2;
pub const S1_8023: c_int = 3;
pub const S1_LLC: c_int = 4;
pub const S1_LLCc: c_int = 5;
pub const S1_IPV4: c_int = 6;
pub const S1_IPV4c: c_int = 7;
pub const S1_IPV4F: c_int = 8;
pub const S1_TCP44: c_int = 9;
pub const S1_IPV6: c_int = 10;
pub const S1_IPV6L: c_int = 11;
pub const S1_IPV6c: c_int = 12;
pub const S1_TCP64: c_int = 13;
pub const S1_TCPSQ: c_int = 14;
pub const S1_TCPFG: c_int = 15;
pub const S1_TCPHL: c_int = 16;
pub const S1_TCPHc: c_int = 17;
pub const S1_CLNP: c_int = 18;
pub const S1_CLNP2: c_int = 19;
pub const S1_DROP: c_int = 20;
pub const S2_HTTP: c_int = 21;
pub const S1_ESP4: c_int = 22;
pub const S1_AH4: c_int = 23;
pub const S1_ESP6: c_int = 24;
pub const S1_AH6: c_int = 25;

//
// Alternate table load which excludes HTTP server traffic from reassembly.
// It is substantially similar to the basic table, with one extra state
// and a few extra compares.

// match step #s for IP4FRAG
pub const S3_IPV6c: c_int = 11;
pub const S3_TCP64: c_int = 12;
pub const S3_TCPSQ: c_int = 13;
pub const S3_TCPFG: c_int = 14;
pub const S3_TCPHL: c_int = 15;
pub const S3_TCPHc: c_int = 16;
pub const S3_FRAG: c_int = 17;
pub const S3_FOFF: c_int = 18;
pub const S3_CLNP: c_int = 19;

//
// Alternate table which does batching without reassembly
//

// Workaround for Cassini rev2 descriptor corruption problem.
// Does batching without reassembly, and sets the SAP to a known
// data pattern for all packets.
//

// "CFI?", /* 02 FIND CFI and If FIND go to S1_DROP
// 0x1000, 0x1000, OP_EQ,  0, S1_DROP,  1, S1_8023,  CL_REG, 0x000,  0, 0x0, 0x00

// @@@0xff00, 0x0600, OP_EQ, 18, S1_TCPSQ, 0, S1_ESP6,  LD_LEN, 0x03f,  1, 0x0, 0xffff,

// @@@0x00ff, 0x0032, OP_EQ,  0, S1_CLNP2, 0, S1_AH6, IM_CTL, 0x021, 1,  0x0, 0xffff,

// @@@0x00ff, 0x0033, OP_EQ,  0, S1_CLNP2, 0, S1_CLNP, IM_CTL, 0x021, 1,  0x0, 0xffff,

// phy types
pub const CAS_PHY_UNKNOWN: c_uint = 0x00;
pub const CAS_PHY_SERDES: c_uint = 0x01;
pub const CAS_PHY_MII_MDIO0: c_uint = 0x02;
pub const CAS_PHY_MII_MDIO1: c_uint = 0x04;

// _RING_INDEX is the index for the ring sizes to be used.  _RING_SIZE
// is the actual size. the default index for the various rings is
// 8. NOTE: there a bunch of alignment constraints for the rings. to
// deal with that, i just allocate rings to create the desired
// alignment. here are the constraints:
// RX DESC and COMP rings must be 8KB aligned
// TX DESC must be 2KB aligned.
// if you change the numbers, be cognizant of how the alignment will change
// in INIT_BLOCK as well.
//

pub const N_RX_COMP_RINGS: c_uint = 0x1 /* for mult. PCI interrupts */;
// number of flows that can go through re-assembly
pub const N_RX_FLOWS: c_int = 64;

// convert values

// min is 2k, but we can't do jumbo frames unless it's at least 8k

pub const TX_DESC_BUFLEN_MASK: c_uint = 0x0000000000003FFFULL /* buffer length in;
pub const TX_DESC_BUFLEN_SHIFT: c_int = 0;
pub const TX_DESC_CSUM_START_MASK: c_uint = 0x00000000001F8000ULL /* checksum start. #;
pub const TX_DESC_CSUM_START_SHIFT: c_int = 15;
pub const TX_DESC_CSUM_STUFF_MASK: c_uint = 0x000000001FE00000ULL /* checksum stuff.;
pub const TX_DESC_CSUM_STUFF_SHIFT: c_int = 21;
pub const TX_DESC_CSUM_EN: c_uint = 0x0000000020000000ULL /* enable checksum */;
pub const TX_DESC_EOF: c_uint = 0x0000000040000000ULL /* end of frame */;
pub const TX_DESC_SOF: c_uint = 0x0000000080000000ULL /* start of frame */;
pub const TX_DESC_INTME: c_uint = 0x0000000100000000ULL /* interrupt me */;
pub const TX_DESC_NO_CRC: c_uint = 0x0000000200000000ULL /* debugging only.;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cas_tx_desc {
    pub control: __le64,
    pub buffer: __le64,
}

// descriptor ring for free buffers contains page-sized buffers. the index
// value is not used by the hw in any way. it's just stored and returned in
// the completion ring.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cas_rx_desc {
    pub index: __le64,
    pub buffer: __le64,
}

// received packets are put on the completion ring.
// word 1
pub const RX_COMP1_DATA_SIZE_MASK: c_uint = 0x0000000007FFE000ULL;
pub const RX_COMP1_DATA_SIZE_SHIFT: c_int = 13;
pub const RX_COMP1_DATA_OFF_MASK: c_uint = 0x000001FFF8000000ULL;
pub const RX_COMP1_DATA_OFF_SHIFT: c_int = 27;
pub const RX_COMP1_DATA_INDEX_MASK: c_uint = 0x007FFE0000000000ULL;
pub const RX_COMP1_DATA_INDEX_SHIFT: c_int = 41;
pub const RX_COMP1_SKIP_MASK: c_uint = 0x0180000000000000ULL;
pub const RX_COMP1_SKIP_SHIFT: c_int = 55;
pub const RX_COMP1_RELEASE_NEXT: c_uint = 0x0200000000000000ULL;
pub const RX_COMP1_SPLIT_PKT: c_uint = 0x0400000000000000ULL;
pub const RX_COMP1_RELEASE_FLOW: c_uint = 0x0800000000000000ULL;
pub const RX_COMP1_RELEASE_DATA: c_uint = 0x1000000000000000ULL;
pub const RX_COMP1_RELEASE_HDR: c_uint = 0x2000000000000000ULL;
pub const RX_COMP1_TYPE_MASK: c_uint = 0xC000000000000000ULL;
pub const RX_COMP1_TYPE_SHIFT: c_int = 62;
// word 2
pub const RX_COMP2_NEXT_INDEX_MASK: c_uint = 0x00000007FFE00000ULL;
pub const RX_COMP2_NEXT_INDEX_SHIFT: c_int = 21;
pub const RX_COMP2_HDR_SIZE_MASK: c_uint = 0x00000FF800000000ULL;
pub const RX_COMP2_HDR_SIZE_SHIFT: c_int = 35;
pub const RX_COMP2_HDR_OFF_MASK: c_uint = 0x0003F00000000000ULL;
pub const RX_COMP2_HDR_OFF_SHIFT: c_int = 44;
pub const RX_COMP2_HDR_INDEX_MASK: c_uint = 0xFFFC000000000000ULL;
pub const RX_COMP2_HDR_INDEX_SHIFT: c_int = 50;
// word 3
pub const RX_COMP3_SMALL_PKT: c_uint = 0x0000000000000001ULL;
pub const RX_COMP3_JUMBO_PKT: c_uint = 0x0000000000000002ULL;
pub const RX_COMP3_JUMBO_HDR_SPLIT_EN: c_uint = 0x0000000000000004ULL;
pub const RX_COMP3_CSUM_START_MASK: c_uint = 0x000000000007F000ULL;
pub const RX_COMP3_CSUM_START_SHIFT: c_int = 12;
pub const RX_COMP3_FLOWID_MASK: c_uint = 0x0000000001F80000ULL;
pub const RX_COMP3_FLOWID_SHIFT: c_int = 19;
pub const RX_COMP3_OPCODE_MASK: c_uint = 0x000000000E000000ULL;
pub const RX_COMP3_OPCODE_SHIFT: c_int = 25;
pub const RX_COMP3_FORCE_FLAG: c_uint = 0x0000000010000000ULL;
pub const RX_COMP3_NO_ASSIST: c_uint = 0x0000000020000000ULL;
pub const RX_COMP3_LOAD_BAL_MASK: c_uint = 0x000001F800000000ULL;
pub const RX_COMP3_LOAD_BAL_SHIFT: c_int = 35;
pub const RX_PLUS_COMP3_ENC_PKT: c_uint = 0x0000020000000000ULL /* cas+ */;
pub const RX_COMP3_L3_HEAD_OFF_MASK: c_uint = 0x0000FE0000000000ULL /* cas */;
pub const RX_COMP3_L3_HEAD_OFF_SHIFT: c_int = 41;
pub const RX_PLUS_COMP_L3_HEAD_OFF_MASK: c_uint = 0x0000FC0000000000ULL /* cas+ */;
pub const RX_PLUS_COMP_L3_HEAD_OFF_SHIFT: c_int = 42;
pub const RX_COMP3_SAP_MASK: c_uint = 0xFFFF000000000000ULL;
pub const RX_COMP3_SAP_SHIFT: c_int = 48;
// word 4
pub const RX_COMP4_TCP_CSUM_MASK: c_uint = 0x000000000000FFFFULL;
pub const RX_COMP4_TCP_CSUM_SHIFT: c_int = 0;
pub const RX_COMP4_PKT_LEN_MASK: c_uint = 0x000000003FFF0000ULL;
pub const RX_COMP4_PKT_LEN_SHIFT: c_int = 16;
pub const RX_COMP4_PERFECT_MATCH_MASK: c_uint = 0x00000003C0000000ULL;
pub const RX_COMP4_PERFECT_MATCH_SHIFT: c_int = 30;
pub const RX_COMP4_ZERO: c_uint = 0x0000080000000000ULL;
pub const RX_COMP4_HASH_VAL_MASK: c_uint = 0x0FFFF00000000000ULL;
pub const RX_COMP4_HASH_VAL_SHIFT: c_int = 44;
pub const RX_COMP4_HASH_PASS: c_uint = 0x1000000000000000ULL;
pub const RX_COMP4_BAD: c_uint = 0x4000000000000000ULL;
pub const RX_COMP4_LEN_MISMATCH: c_uint = 0x8000000000000000ULL;
// we encode the following: ring/index/release. only 14 bits
// are usable.
// NOTE: the encoding is dependent upon RX_DESC_RING_SIZE and
// MAX_RX_DESC_RINGS.
pub const RX_INDEX_NUM_MASK: c_uint = 0x0000000000000FFFULL;
pub const RX_INDEX_NUM_SHIFT: c_int = 0;
pub const RX_INDEX_RING_MASK: c_uint = 0x0000000000001000ULL;
pub const RX_INDEX_RING_SHIFT: c_int = 12;
pub const RX_INDEX_RELEASE: c_uint = 0x0000000000002000ULL;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cas_rx_comp {
    pub word1: __le64,
    pub word2: __le64,
    pub word3: __le64,
    pub word4: __le64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum link_state {
    link_down = 0,	/* No link, will retry */
    link_aneg,	/* Autoneg in progress */
    link_force_try,	/* Try Forced link speed */
    link_force_ret,	/* Forced mode worked, retrying autoneg */
    link_force_ok,	/* Stay in forced mode */
    link_up		/* Link is up */
}

// some alignment constraints:
// TX DESC, RX DESC, and RX COMP must each be 8K aligned.
// TX COMPWB must be 8-byte aligned.
// to accomplish this, here's what we do:
//
// INIT_BLOCK_RX_COMP  = 64k (already aligned)
// INIT_BLOCK_RX_DESC  = 8k
// INIT_BLOCK_TX       = 8k
// INIT_BLOCK_RX1_DESC = 8k
// TX COMPWB
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cas_init_block {
    pub rxcs: [cas_rx_comp; N_RX_COMP_RINGS][INIT_BLOCK_RX_COMP],
    pub rxds: [cas_rx_desc; N_RX_DESC_RINGS][INIT_BLOCK_RX_DESC],
    pub txds: [cas_tx_desc; N_TX_RINGS][INIT_BLOCK_TX],
    pub tx_compwb: __le64,
}

// tiny buffers to deal with target abort issue. we allocate a bit
// over so that we don't have target abort issues with these buffers
// as well.
//
pub const TX_TINY_BUF_LEN: c_uint = 0x100;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cas_tiny_count {
    pub nbufs: c_int,
    pub used: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cas {
    pub /: *mut *mut spinlock_t lock; / for most bits,
    pub /: *mut *mut spinlock_t tx_lock[N_TX_RINGS]; / tx bits,
    pub /: *mut *mut spinlock_t stat_lock[N_TX_RINGS + 1]; / for stat gathering,
    pub /: *mut *mut spinlock_t rx_inuse_lock; / rx inuse list,
    pub /: *mut *mut spinlock_t rx_spare_lock; / rx spare list,
    pub regs: *mut void __iomem,
    pub tx_old: [int tx_new[N_TX_RINGS],; N_TX_RINGS],
    pub rx_old: [c_int; N_RX_DESC_RINGS],
    pub rx_new: [int rx_cur[N_RX_COMP_RINGS],; N_RX_COMP_RINGS],
    pub rx_last: [c_int; N_RX_DESC_RINGS],
    pub napi: napi_struct,
// Set when chip is actually in operational state
// (ie. not power managed)
    pub hw_running: c_int,
    pub opened: c_int,
    pub /: *mut *mut mutex pm_mutex; / open/close/suspend/resume,
    pub init_block: *mut cas_init_block,
    pub init_txds: [*mut cas_tx_desc; MAX_TX_RINGS],
    pub init_rxds: [*mut cas_rx_desc; MAX_RX_DESC_RINGS],
    pub init_rxcs: [*mut cas_rx_comp; MAX_RX_COMP_RINGS],
// we use sk_buffs for tx and pages for rx. the rx skbuffs
// are there for flow re-assembly.
    pub tx_skbs: [*mut sk_buff; N_TX_RINGS][TX_DESC_RING_SIZE],
    pub rx_flows: [sk_buff_head; N_RX_FLOWS],
    pub rx_pages: [*mut cas_page_t; N_RX_DESC_RINGS][RX_DESC_RING_SIZE],
    pub rx_inuse_list: list_head rx_spare_list,,
    pub rx_spares_needed: c_int,
// for small packets when copying would be quicker than
    pub tx_tiny_use: [cas_tiny_count; N_TX_RINGS][TX_DESC_RING_SIZE],
    pub tx_tiny_bufs: [*mut u8; N_TX_RINGS],
    pub msg_enable: u32,
// N_TX_RINGS must be >= N_RX_DESC_RINGS
    pub 1]: net_device_stats net_stats[N_TX_RINGS +,
    pub 2]: u32 pci_cfg[64 >>,
    pub pci_revision: u8,
    pub phy_type: c_int,
    pub phy_addr: c_int,
    pub phy_id: u32,
pub const CAS_FLAG_1000MB_CAP: c_uint = 0x00000001;
pub const CAS_FLAG_REG_PLUS: c_uint = 0x00000002;
pub const CAS_FLAG_TARGET_ABORT: c_uint = 0x00000004;
pub const CAS_FLAG_SATURN: c_uint = 0x00000008;
pub const CAS_FLAG_RXD_POST_MASK: c_uint = 0x000000F0;
pub const CAS_FLAG_RXD_POST_SHIFT: c_int = 4;

pub const CAS_FLAG_ENTROPY_DEV: c_uint = 0x00000100;
pub const CAS_FLAG_NO_HW_CSUM: c_uint = 0x00000200;
    pub cas_flags: u32,
    pub /: *mut *mut int packet_min; / minimum packet size,
    pub tx_fifo_size: c_int,
    pub rx_fifo_size: c_int,
    pub rx_pause_off: c_int,
    pub rx_pause_on: c_int,
    pub /: *mut *mut int crc_size; / 4 if half-duplex,
    pub pci_irq_INTC: c_int,
    pub /: *mut *mut int min_frame_size; / for tx fifo workaround,
// page size allocation
    pub page_size: c_int,
    pub page_order: c_int,
    pub mtu_stride: c_int,
    pub mac_rx_cfg: u32,
// Autoneg & PHY control
    pub link_cntl: c_int,
    pub link_fcntl: c_int,
    pub lstate: link_state,
    pub link_timer: timer_list,
    pub timer_ticks: c_int,
    pub reset_task: work_struct,

    pub reset_task_pending: core::sync::atomic::AtomicI32,

    pub reset_task_pending: core::sync::atomic::AtomicI32,
    pub reset_task_pending_mtu: core::sync::atomic::AtomicI32,
    pub reset_task_pending_spare: core::sync::atomic::AtomicI32,
    pub reset_task_pending_all: core::sync::atomic::AtomicI32,

// Link-down problem workaround
pub const LINK_TRANSITION_UNKNOWN: c_int = 0;
pub const LINK_TRANSITION_ON_FAILURE: c_int = 1;
pub const LINK_TRANSITION_STILL_FAILED: c_int = 2;
pub const LINK_TRANSITION_LINK_UP: c_int = 3;
pub const LINK_TRANSITION_LINK_CONFIG: c_int = 4;
pub const LINK_TRANSITION_LINK_DOWN: c_int = 5;
pub const LINK_TRANSITION_REQUESTED_RESET: c_int = 6;
    pub link_transition: c_int,
    pub link_transition_jiffies_valid: c_int,
    pub link_transition_jiffies: c_ulong,
// Tuning
    pub /: *mut *mut u8 orig_cacheline_size; / value when loaded,
pub const CAS_PREF_CACHELINE_SIZE: c_uint = 0x20	/* Minimum desired */;
// Diagnostic counters and state.
    pub /: *mut *mut int casreg_len; / reg-space size for dumping,
    pub pause_entered: u64,
    pub pause_last_time_recvd: u16,
    pub tx_tiny_dvma: [dma_addr_t block_dvma,; N_TX_RINGS],
    pub pdev: *mut pci_dev,
    pub dev: *mut net_device,

    pub of_node: *mut device_node,

// Firmware Info
    pub fw_load_addr: u16,
    pub fw_size: u32,
    pub fw_data: *mut u8,
}

pub const RX_FIFO_SIZE: c_int = 16384;
pub const EXPANSION_ROM_SIZE: c_int = 65536;
pub const CAS_MC_EXACT_MATCH_SIZE: c_int = 15;
pub const CAS_MC_HASH_SIZE: c_int = 256;

pub const TX_TARGET_ABORT_LEN: c_uint = 0x20;
pub const RX_SWIVEL_OFF_VAL: c_uint = 0x2;

pub const RX_BLANK_INTR_PKT_VAL: c_uint = 0x05;
pub const RX_BLANK_INTR_TIME_VAL: c_uint = 0x0F;

