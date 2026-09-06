//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cortina/gemini.h
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
// Register definitions for Gemini GMAC Ethernet device driver
//
// Copyright (C) 2006 Storlink, Corp.
// Copyright (C) 2008-2009 Paulius Zaleckas <paulius.zaleckas@teltonika.lt>
// Copyright (C) 2010 Michał Mirosław <mirq-linux@rere.qmqm.pl>
// Copytight (C) 2017 Linus Walleij <linus.walleij@linaro.org>
//

// Base Registers
pub const TOE_NONTOE_QUE_HDR_BASE: c_uint = 0x2000;
pub const TOE_TOE_QUE_HDR_BASE: c_uint = 0x3000;
// Queue ID
pub const TOE_SW_FREE_QID: c_uint = 0x00;
pub const TOE_HW_FREE_QID: c_uint = 0x01;
pub const TOE_GMAC0_SW_TXQ0_QID: c_uint = 0x02;
pub const TOE_GMAC0_SW_TXQ1_QID: c_uint = 0x03;
pub const TOE_GMAC0_SW_TXQ2_QID: c_uint = 0x04;
pub const TOE_GMAC0_SW_TXQ3_QID: c_uint = 0x05;
pub const TOE_GMAC0_SW_TXQ4_QID: c_uint = 0x06;
pub const TOE_GMAC0_SW_TXQ5_QID: c_uint = 0x07;
pub const TOE_GMAC0_HW_TXQ0_QID: c_uint = 0x08;
pub const TOE_GMAC0_HW_TXQ1_QID: c_uint = 0x09;
pub const TOE_GMAC0_HW_TXQ2_QID: c_uint = 0x0A;
pub const TOE_GMAC0_HW_TXQ3_QID: c_uint = 0x0B;
pub const TOE_GMAC1_SW_TXQ0_QID: c_uint = 0x12;
pub const TOE_GMAC1_SW_TXQ1_QID: c_uint = 0x13;
pub const TOE_GMAC1_SW_TXQ2_QID: c_uint = 0x14;
pub const TOE_GMAC1_SW_TXQ3_QID: c_uint = 0x15;
pub const TOE_GMAC1_SW_TXQ4_QID: c_uint = 0x16;
pub const TOE_GMAC1_SW_TXQ5_QID: c_uint = 0x17;
pub const TOE_GMAC1_HW_TXQ0_QID: c_uint = 0x18;
pub const TOE_GMAC1_HW_TXQ1_QID: c_uint = 0x19;
pub const TOE_GMAC1_HW_TXQ2_QID: c_uint = 0x1A;
pub const TOE_GMAC1_HW_TXQ3_QID: c_uint = 0x1B;
pub const TOE_GMAC0_DEFAULT_QID: c_uint = 0x20;
pub const TOE_GMAC1_DEFAULT_QID: c_uint = 0x21;

// TOE DMA Queue Size should be 2^n, n = 6...12
// TOE DMA Queues are the following queue types:
// SW Free Queue, HW Free Queue,
// GMAC 0/1 SW TX Q0-5, and GMAC 0/1 HW TX Q0-5
// The base address and descriptor number are configured at
// DMA Queues Descriptor Ring Base Address/Size Register (offset 0x0004)
//

// Global registers
pub const GLOBAL_TOE_VERSION_REG: c_uint = 0x0000;
pub const GLOBAL_SW_FREEQ_BASE_SIZE_REG: c_uint = 0x0004;
pub const GLOBAL_HW_FREEQ_BASE_SIZE_REG: c_uint = 0x0008;
pub const GLOBAL_DMA_SKB_SIZE_REG: c_uint = 0x0010;
pub const GLOBAL_SWFQ_RWPTR_REG: c_uint = 0x0014;
pub const GLOBAL_HWFQ_RWPTR_REG: c_uint = 0x0018;
pub const GLOBAL_INTERRUPT_STATUS_0_REG: c_uint = 0x0020;
pub const GLOBAL_INTERRUPT_ENABLE_0_REG: c_uint = 0x0024;
pub const GLOBAL_INTERRUPT_SELECT_0_REG: c_uint = 0x0028;
pub const GLOBAL_INTERRUPT_STATUS_1_REG: c_uint = 0x0030;
pub const GLOBAL_INTERRUPT_ENABLE_1_REG: c_uint = 0x0034;
pub const GLOBAL_INTERRUPT_SELECT_1_REG: c_uint = 0x0038;
pub const GLOBAL_INTERRUPT_STATUS_2_REG: c_uint = 0x0040;
pub const GLOBAL_INTERRUPT_ENABLE_2_REG: c_uint = 0x0044;
pub const GLOBAL_INTERRUPT_SELECT_2_REG: c_uint = 0x0048;
pub const GLOBAL_INTERRUPT_STATUS_3_REG: c_uint = 0x0050;
pub const GLOBAL_INTERRUPT_ENABLE_3_REG: c_uint = 0x0054;
pub const GLOBAL_INTERRUPT_SELECT_3_REG: c_uint = 0x0058;
pub const GLOBAL_INTERRUPT_STATUS_4_REG: c_uint = 0x0060;
pub const GLOBAL_INTERRUPT_ENABLE_4_REG: c_uint = 0x0064;
pub const GLOBAL_INTERRUPT_SELECT_4_REG: c_uint = 0x0068;
pub const GLOBAL_HASH_TABLE_BASE_REG: c_uint = 0x006C;
pub const GLOBAL_QUEUE_THRESHOLD_REG: c_uint = 0x0070;
// GMAC 0/1 DMA/TOE register
pub const GMAC_DMA_CTRL_REG: c_uint = 0x0000;
pub const GMAC_TX_WEIGHTING_CTRL_0_REG: c_uint = 0x0004;
pub const GMAC_TX_WEIGHTING_CTRL_1_REG: c_uint = 0x0008;
pub const GMAC_SW_TX_QUEUE0_PTR_REG: c_uint = 0x000C;
pub const GMAC_SW_TX_QUEUE1_PTR_REG: c_uint = 0x0010;
pub const GMAC_SW_TX_QUEUE2_PTR_REG: c_uint = 0x0014;
pub const GMAC_SW_TX_QUEUE3_PTR_REG: c_uint = 0x0018;
pub const GMAC_SW_TX_QUEUE4_PTR_REG: c_uint = 0x001C;
pub const GMAC_SW_TX_QUEUE5_PTR_REG: c_uint = 0x0020;

pub const GMAC_HW_TX_QUEUE0_PTR_REG: c_uint = 0x0024;
pub const GMAC_HW_TX_QUEUE1_PTR_REG: c_uint = 0x0028;
pub const GMAC_HW_TX_QUEUE2_PTR_REG: c_uint = 0x002C;
pub const GMAC_HW_TX_QUEUE3_PTR_REG: c_uint = 0x0030;

pub const GMAC_DMA_TX_FIRST_DESC_REG: c_uint = 0x0038;
pub const GMAC_DMA_TX_CURR_DESC_REG: c_uint = 0x003C;
pub const GMAC_DMA_TX_DESC_WORD0_REG: c_uint = 0x0040;
pub const GMAC_DMA_TX_DESC_WORD1_REG: c_uint = 0x0044;
pub const GMAC_DMA_TX_DESC_WORD2_REG: c_uint = 0x0048;
pub const GMAC_DMA_TX_DESC_WORD3_REG: c_uint = 0x004C;
pub const GMAC_SW_TX_QUEUE_BASE_REG: c_uint = 0x0050;
pub const GMAC_HW_TX_QUEUE_BASE_REG: c_uint = 0x0054;
pub const GMAC_DMA_RX_FIRST_DESC_REG: c_uint = 0x0058;
pub const GMAC_DMA_RX_CURR_DESC_REG: c_uint = 0x005C;
pub const GMAC_DMA_RX_DESC_WORD0_REG: c_uint = 0x0060;
pub const GMAC_DMA_RX_DESC_WORD1_REG: c_uint = 0x0064;
pub const GMAC_DMA_RX_DESC_WORD2_REG: c_uint = 0x0068;
pub const GMAC_DMA_RX_DESC_WORD3_REG: c_uint = 0x006C;
pub const GMAC_HASH_ENGINE_REG0: c_uint = 0x0070;
pub const GMAC_HASH_ENGINE_REG1: c_uint = 0x0074;
// matching rule 0 Control register 0
pub const GMAC_MR0CR0: c_uint = 0x0078;
pub const GMAC_MR0CR1: c_uint = 0x007C;
pub const GMAC_MR0CR2: c_uint = 0x0080;
pub const GMAC_MR1CR0: c_uint = 0x0084;
pub const GMAC_MR1CR1: c_uint = 0x0088;
pub const GMAC_MR1CR2: c_uint = 0x008C;
pub const GMAC_MR2CR0: c_uint = 0x0090;
pub const GMAC_MR2CR1: c_uint = 0x0094;
pub const GMAC_MR2CR2: c_uint = 0x0098;
pub const GMAC_MR3CR0: c_uint = 0x009C;
pub const GMAC_MR3CR1: c_uint = 0x00A0;
pub const GMAC_MR3CR2: c_uint = 0x00A4;
// Support Protocol Register 0
pub const GMAC_SPR0: c_uint = 0x00A8;
pub const GMAC_SPR1: c_uint = 0x00AC;
pub const GMAC_SPR2: c_uint = 0x00B0;
pub const GMAC_SPR3: c_uint = 0x00B4;
pub const GMAC_SPR4: c_uint = 0x00B8;
pub const GMAC_SPR5: c_uint = 0x00BC;
pub const GMAC_SPR6: c_uint = 0x00C0;
pub const GMAC_SPR7: c_uint = 0x00C4;
// GMAC Hash/Rx/Tx AHB Weighting register
pub const GMAC_AHB_WEIGHT_REG: c_uint = 0x00C8;
// TOE GMAC 0/1 register
pub const GMAC_STA_ADD0: c_uint = 0x0000;
pub const GMAC_STA_ADD1: c_uint = 0x0004;
pub const GMAC_STA_ADD2: c_uint = 0x0008;
pub const GMAC_RX_FLTR: c_uint = 0x000c;
pub const GMAC_MCAST_FIL0: c_uint = 0x0010;
pub const GMAC_MCAST_FIL1: c_uint = 0x0014;
pub const GMAC_CONFIG0: c_uint = 0x0018;
pub const GMAC_CONFIG1: c_uint = 0x001c;
pub const GMAC_CONFIG2: c_uint = 0x0020;
pub const GMAC_CONFIG3: c_uint = 0x0024;
pub const GMAC_RESERVED: c_uint = 0x0028;
pub const GMAC_STATUS: c_uint = 0x002c;
pub const GMAC_IN_DISCARDS: c_uint = 0x0030;
pub const GMAC_IN_ERRORS: c_uint = 0x0034;
pub const GMAC_IN_MCAST: c_uint = 0x0038;
pub const GMAC_IN_BCAST: c_uint = 0x003c;
pub const GMAC_IN_MAC1: c_uint = 0x0040	/* for STA 1 MAC Address */;
pub const GMAC_IN_MAC2: c_uint = 0x0044	/* for STA 2 MAC Address */;
pub const RX_STATS_NUM: c_int = 6;
// DMA Queues description Ring Base Address/Size Register (offset 0x0004)
#[repr(C)]
#[derive(Copy, Clone)]
pub union dma_q_base_size {
    pub bits32: c_uint,
    pub base_size: c_uint,
}

// DMA SKB Buffer register (offset 0x0008)
#[repr(C)]
#[derive(Copy, Clone)]
pub union dma_skb_size {
    pub bits32: c_uint,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bit_0008 {
    pub /: *mut *mut unsigned int sw_skb_size : 16; / SW Free poll SKB Size,
    pub /: *mut *mut unsigned int hw_skb_size : 16; / HW Free poll SKB Size,
    pub bits: },
}

// DMA SW Free Queue Read/Write Pointer Register (offset 0x000c)
#[repr(C)]
#[derive(Copy, Clone)]
pub union dma_rwptr {
    pub bits32: c_uint,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bit_000c {
    pub /: *mut *mut unsigned int rptr : 16; / Read Ptr, RO,
    pub /: *mut *mut unsigned int wptr : 16; / Write Ptr, RW,
    pub bits: },
}

// Interrupt Status Register 0	(offset 0x0020)
// Interrupt Mask Register 0	(offset 0x0024)
// Interrupt Select Register 0	(offset 0x0028)
//

// Interrupt Status Register 1	(offset 0x0030)
// Interrupt Mask Register 1	(offset 0x0034)
// Interrupt Select Register 1	(offset 0x0038)
//

pub const TOE_CLASS_RX_INT_BITS: c_uint = 0xfffc;
// Interrupt Status Register 2	(offset 0x0040)
// Interrupt Mask Register 2	(offset 0x0044)
// Interrupt Select Register 2	(offset 0x0048)
//

// Interrupt Status Register 3	(offset 0x0050)
// Interrupt Mask Register 3	(offset 0x0054)
// Interrupt Select Register 3	(offset 0x0058)
//

// Interrupt Status Register 4	(offset 0x0060)
// Interrupt Mask Register 4	(offset 0x0064)
// Interrupt Select Register 4	(offset 0x0068)
//

pub const CLASS_RX_FULL_INT_BITS: c_uint = 0xfffc;
// GLOBAL_QUEUE_THRESHOLD_REG	(offset 0x0070)
#[repr(C)]
#[derive(Copy, Clone)]
pub union queue_threshold {
    pub bits32: c_uint,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bit_0070_2 {
// 7:0 Software Free Queue Empty Threshold
    pub swfq_empty:8: c_uint,
// 15:8 Hardware Free Queue Empty Threshold
    pub hwfq_empty:8: c_uint,
// 23:16
    pub intrq:8: c_uint,
// 31:24
    pub toe_class:8: c_uint,
    pub bits: },
}

// GMAC DMA Control Register
// GMAC0 offset 0x8000
// GMAC1 offset 0xC000
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union gmac_dma_ctrl {
    pub bits32: c_uint,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bit_8000 {
// bit 1:0 Peripheral Bus Width
    pub td_bus:2: c_uint,
// bit 3:2 TxDMA max burst size for every AHB request
    pub td_burst_size:2: c_uint,
// bit 7:4 TxDMA protection control
    pub td_prot:4: c_uint,
// bit 9:8 Peripheral Bus Width
    pub rd_bus:2: c_uint,
// bit 11:10 DMA max burst size for every AHB request
    pub rd_burst_size:2: c_uint,
// bit 15:12 DMA Protection Control
    pub rd_prot:4: c_uint,
// bit 17:16
    pub rd_insert_bytes:2: c_uint,
// bit 27:18
    pub reserved:10: c_uint,
// bit 28 1: Drop, 0: Accept
    pub drop_small_ack:1: c_uint,
// bit 29 Loopback TxDMA to RxDMA
    pub loopback:1: c_uint,
// bit 30 Tx DMA Enable
    pub td_enable:1: c_uint,
// bit 31 Rx DMA Enable
    pub rd_enable:1: c_uint,
    pub bits: },
}

// GMAC Tx Weighting Control Register 0
// GMAC0 offset 0x8004
// GMAC1 offset 0xC004
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union gmac_tx_wcr0 {
    pub bits32: c_uint,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bit_8004 {
// bit 5:0 HW TX Queue 3
    pub hw_tq0:6: c_uint,
// bit 11:6 HW TX Queue 2
    pub hw_tq1:6: c_uint,
// bit 17:12 HW TX Queue 1
    pub hw_tq2:6: c_uint,
// bit 23:18 HW TX Queue 0
    pub hw_tq3:6: c_uint,
// bit 31:24
    pub reserved:8: c_uint,
    pub bits: },
}

// GMAC Tx Weighting Control Register 1
// GMAC0 offset 0x8008
// GMAC1 offset 0xC008
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union gmac_tx_wcr1 {
    pub bits32: c_uint,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bit_8008 {
// bit 4:0 SW TX Queue 0
    pub sw_tq0:5: c_uint,
// bit 9:5 SW TX Queue 1
    pub sw_tq1:5: c_uint,
// bit 14:10 SW TX Queue 2
    pub sw_tq2:5: c_uint,
// bit 19:15 SW TX Queue 3
    pub sw_tq3:5: c_uint,
// bit 24:20 SW TX Queue 4
    pub sw_tq4:5: c_uint,
// bit 29:25 SW TX Queue 5
    pub sw_tq5:5: c_uint,
// bit 31:30
    pub reserved:2: c_uint,
    pub bits: },
}

// GMAC DMA Tx Description Word 0 Register
// GMAC0 offset 0x8040
// GMAC1 offset 0xC040
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union gmac_txdesc_0 {
    pub bits32: c_uint,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bit_8040 {
// bit 15:0 Transfer size
    pub buffer_size:16: c_uint,
// bit 21:16 number of descriptors used for the current frame
    pub desc_count:6: c_uint,
// bit 22 Tx Status, 1: Successful 0: Failed
    pub status_tx_ok:1: c_uint,
// bit 28:23 Tx Status, Reserved bits
    pub status_rvd:6: c_uint,
// bit 29 protocol error during processing this descriptor
    pub perr:1: c_uint,
// bit 30 data error during processing this descriptor
    pub derr:1: c_uint,
// bit 31
    pub reserved:1: c_uint,
    pub bits: },
}

// GMAC DMA Tx Description Word 1 Register
// GMAC0 offset 0x8044
// GMAC1 offset 0xC044
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union gmac_txdesc_1 {
    pub bits32: c_uint,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct txdesc_word1 {
// bit 15: 0 Tx Frame Byte Count
    pub byte_count:16: c_uint,
// bit 16 TSS segmentation use MTU setting
    pub mtu_enable:1: c_uint,
// bit 17 IPV4 Header Checksum Enable
    pub ip_chksum:1: c_uint,
// bit 18 IPV6 Tx Enable
    pub ipv6_enable:1: c_uint,
// bit 19 TCP Checksum Enable
    pub tcp_chksum:1: c_uint,
// bit 20 UDP Checksum Enable
    pub udp_chksum:1: c_uint,
// bit 21 Bypass HW offload engine
    pub bypass_tss:1: c_uint,
// bit 22 Don't update IP length field
    pub ip_fixed_len:1: c_uint,
// bit 31:23 Tx Flag, Reserved
    pub reserved:9: c_uint,
    pub bits: },
}

// GMAC DMA Tx Description Word 2 Register
// GMAC0 offset 0x8048
// GMAC1 offset 0xC048
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union gmac_txdesc_2 {
    pub bits32: c_uint,
    pub buf_adr: c_uint,
}

// GMAC DMA Tx Description Word 3 Register
// GMAC0 offset 0x804C
// GMAC1 offset 0xC04C
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union gmac_txdesc_3 {
    pub bits32: c_uint,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct txdesc_word3 {
// bit 12: 0 Tx Frame Byte Count
    pub mtu_size:13: c_uint,
// bit 28:13
    pub reserved:16: c_uint,
// bit 29 End of frame interrupt enable
    pub eofie:1: c_uint,
// bit 31:30 11: only one, 10: first, 01: last, 00: linking
    pub sof_eof:2: c_uint,
    pub bits: },
}

pub const SOF_EOF_BIT_MASK: c_uint = 0x3fffffff;
pub const SOF_BIT: c_uint = 0x80000000;
pub const EOF_BIT: c_uint = 0x40000000;

pub const MTU_SIZE_BIT_MASK: c_uint = 0x7ff /* Max MTU 2047 bytes */;
// GMAC Tx Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gmac_txdesc {
    pub word0: gmac_txdesc_0,
    pub word1: gmac_txdesc_1,
    pub word2: gmac_txdesc_2,
    pub word3: gmac_txdesc_3,
}

// GMAC DMA Rx Description Word 0 Register
// GMAC0 offset 0x8060
// GMAC1 offset 0xC060
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union gmac_rxdesc_0 {
    pub bits32: c_uint,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bit_8060 {
// bit 15:0 number of descriptors used for the current frame
    pub buffer_size:16: c_uint,
// bit 21:16 number of descriptors used for the current frame
    pub desc_count:6: c_uint,
// bit 24:22 Status of rx frame
    pub status:4: c_uint,
// bit 28:26 Check Sum Status
    pub chksum_status:3: c_uint,
// bit 29 protocol error during processing this descriptor
    pub perr:1: c_uint,
// bit 30 data error during processing this descriptor
    pub derr:1: c_uint,
// bit 31 TOE/CIS Queue Full dropped packet to default queue
    pub drop:1: c_uint,
    pub bits: },
}

pub const RX_CHKSUM_IP_UDP_TCP_OK: c_int = 0;
pub const RX_CHKSUM_IP_OK_ONLY: c_int = 1;
pub const RX_CHKSUM_NONE: c_int = 2;
pub const RX_CHKSUM_IP_ERR_UNKNOWN: c_int = 4;
pub const RX_CHKSUM_IP_ERR: c_int = 5;
pub const RX_CHKSUM_TCP_UDP_ERR: c_int = 6;
pub const RX_CHKSUM_NUM: c_int = 8;
pub const RX_STATUS_GOOD_FRAME: c_int = 0;
pub const RX_STATUS_TOO_LONG_GOOD_CRC: c_int = 1;
pub const RX_STATUS_RUNT_FRAME: c_int = 2;
pub const RX_STATUS_SFD_NOT_FOUND: c_int = 3;
pub const RX_STATUS_CRC_ERROR: c_int = 4;
pub const RX_STATUS_TOO_LONG_BAD_CRC: c_int = 5;
pub const RX_STATUS_ALIGNMENT_ERROR: c_int = 6;
pub const RX_STATUS_TOO_LONG_BAD_ALIGN: c_int = 7;
pub const RX_STATUS_RX_ERR: c_int = 8;
pub const RX_STATUS_DA_FILTERED: c_int = 9;
pub const RX_STATUS_BUFFER_FULL: c_int = 10;
pub const RX_STATUS_NUM: c_int = 16;

// GMAC DMA Rx Description Word 1 Register
// GMAC0 offset 0x8064
// GMAC1 offset 0xC064
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union gmac_rxdesc_1 {
    pub bits32: c_uint,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxdesc_word1 {
// bit 15: 0 Rx Frame Byte Count
    pub byte_count:16: c_uint,
// bit 31:16 Software ID
    pub sw_id:16: c_uint,
    pub bits: },
}

// GMAC DMA Rx Description Word 2 Register
// GMAC0 offset 0x8068
// GMAC1 offset 0xC068
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union gmac_rxdesc_2 {
    pub bits32: c_uint,
    pub buf_adr: c_uint,
}

pub const RX_INSERT_NONE: c_int = 0;
pub const RX_INSERT_1_BYTE: c_int = 1;
pub const RX_INSERT_2_BYTE: c_int = 2;
pub const RX_INSERT_3_BYTE: c_int = 3;
// GMAC DMA Rx Description Word 3 Register
// GMAC0 offset 0x806C
// GMAC1 offset 0xC06C
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union gmac_rxdesc_3 {
    pub bits32: c_uint,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxdesc_word3 {
// bit 7: 0 L3 data offset
    pub l3_offset:8: c_uint,
// bit 15: 8 L4 data offset
    pub l4_offset:8: c_uint,
// bit 23: 16 L7 data offset
    pub l7_offset:8: c_uint,
// bit 24 Duplicated ACK detected
    pub dup_ack:1: c_uint,
// bit 25 abnormal case found
    pub abnormal:1: c_uint,
// bit 26 IPV4 option or IPV6 extension header
    pub option:1: c_uint,
// bit 27 Out of Sequence packet
    pub out_of_seq:1: c_uint,
// bit 28 Control Flag is present
    pub ctrl_flag:1: c_uint,
// bit 29 End of frame interrupt enable
    pub eofie:1: c_uint,
// bit 31:30 11: only one, 10: first, 01: last, 00: linking
    pub sof_eof:2: c_uint,
    pub bits: },
}

// GMAC Rx Descriptor, this is simply fitted over the queue registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gmac_rxdesc {
    pub word0: gmac_rxdesc_0,
    pub word1: gmac_rxdesc_1,
    pub word2: gmac_rxdesc_2,
    pub word3: gmac_rxdesc_3,
}

// GMAC Matching Rule Control Register 0
// GMAC0 offset 0x8078
// GMAC1 offset 0xC078
//

pub const MR_SPR_BITS: c_uint = 0xff;
// GMAC_AHB_WEIGHT registers
// GMAC0 offset 0x80C8
// GMAC1 offset 0xC0C8
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union gmac_ahb_weight {
    pub bits32: c_uint,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bit_80C8 {
// 4:0
    pub hash_weight:5: c_uint,
// 9:5
    pub rx_weight:5: c_uint,
// 14:10
    pub tx_weight:5: c_uint,
// 19:15 Rx Data Pre Request FIFO Threshold
    pub pre_req:5: c_uint,
// 24:20 DMA TqCtrl to Start tqDV FIFO Threshold
    pub tq_dv_threshold:5: c_uint,
// 31:25
    pub reserved:7: c_uint,
    pub bits: },
}

// GMAC RX FLTR
// GMAC0 Offset 0xA00C
// GMAC1 Offset 0xE00C
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union gmac_rx_fltr {
    pub bits32: c_uint,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bit1_000c {
// Enable receive of unicast frames that are sent to STA
// address
//
    pub unicast:1: c_uint,
// Enable receive of multicast frames that pass multicast
// filter
//
    pub multicast:1: c_uint,
// Enable receive of broadcast frames
    pub broadcast:1: c_uint,
// Enable receive of all frames
    pub promiscuous:1: c_uint,
// Enable receive of all error frames
    pub error:1: c_uint,
    pub reserved:27: c_uint,
    pub bits: },
}

// GMAC Configuration 0
// GMAC0 Offset 0xA018
// GMAC1 Offset 0xE018
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union gmac_config0 {
    pub bits32: c_uint,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bit1_0018 {
// 0: disable transmit
    pub dis_tx:1: c_uint,
// 1: disable receive
    pub dis_rx:1: c_uint,
// 2: transmit data loopback enable
    pub loop_back:1: c_uint,
// 3: flow control also trigged by Rx queues
    pub flow_ctrl:1: c_uint,
// 4-7: adjust IFG from 96+/-56
    pub adj_ifg:4: c_uint,
// 8-10 maximum receive frame length allowed
    pub max_len:3: c_uint,
// 11: disable back-off function
    pub dis_bkoff:1: c_uint,
// 12: disable 16 collisions abort function
    pub dis_col:1: c_uint,
// 13: speed up timers in simulation
    pub sim_test:1: c_uint,
// 14: RX flow control enable
    pub rx_fc_en:1: c_uint,
// 15: TX flow control enable
    pub tx_fc_en:1: c_uint,
// 16: RGMII in-band status enable
    pub rgmii_en:1: c_uint,
// 17: IPv4 RX Checksum enable
    pub ipv4_rx_chksum:1: c_uint,
// 18: IPv6 RX Checksum enable
    pub ipv6_rx_chksum:1: c_uint,
// 19: Remove Rx VLAN tag
    pub rx_tag_remove:1: c_uint,
// 20
    pub rgmm_edge:1: c_uint,
// 21
    pub rxc_inv:1: c_uint,
// 22
    pub ipv6_exthdr_order:1: c_uint,
// 23
    pub rx_err_detect:1: c_uint,
// 24
    pub port0_chk_hwq:1: c_uint,
// 25
    pub port1_chk_hwq:1: c_uint,
// 26
    pub port0_chk_toeq:1: c_uint,
// 27
    pub port1_chk_toeq:1: c_uint,
// 28
    pub port0_chk_classq:1: c_uint,
// 29
    pub port1_chk_classq:1: c_uint,
// 30, 31
    pub reserved:2: c_uint,
    pub bits: },
}

pub const CONFIG0_MAXLEN_SHIFT: c_int = 8;

pub const CONFIG0_MAXLEN_1536: c_int = 0;
pub const CONFIG0_MAXLEN_1518: c_int = 1;
pub const CONFIG0_MAXLEN_1522: c_int = 2;
pub const CONFIG0_MAXLEN_1548: c_int = 3;

pub const CONFIG0_MAXLEN_1518__6: c_int = 6;
pub const CONFIG0_MAXLEN_1518__7: c_int = 7;
// GMAC Configuration 1
// GMAC0 Offset 0xA01C
// GMAC1 Offset 0xE01C
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union gmac_config1 {
    pub bits32: c_uint,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bit1_001c {
// Flow control set threshold
    pub set_threshold:8: c_uint,
// Flow control release threshold
    pub rel_threshold:8: c_uint,
    pub reserved:16: c_uint,
    pub bits: },
}

pub const GMAC_FLOWCTRL_SET_MAX: c_int = 32;
pub const GMAC_FLOWCTRL_SET_MIN: c_int = 0;
pub const GMAC_FLOWCTRL_RELEASE_MAX: c_int = 32;
pub const GMAC_FLOWCTRL_RELEASE_MIN: c_int = 0;
// GMAC Configuration 2
// GMAC0 Offset 0xA020
// GMAC1 Offset 0xE020
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union gmac_config2 {
    pub bits32: c_uint,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bit1_0020 {
// Flow control set threshold
    pub set_threshold:16: c_uint,
// Flow control release threshold
    pub rel_threshold:16: c_uint,
    pub bits: },
}

// GMAC Configuration 3
// GMAC0 Offset 0xA024
// GMAC1 Offset 0xE024
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union gmac_config3 {
    pub bits32: c_uint,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bit1_0024 {
// Flow control set threshold
    pub set_threshold:16: c_uint,
// Flow control release threshold
    pub rel_threshold:16: c_uint,
    pub bits: },
}

// GMAC STATUS
// GMAC0 Offset 0xA02C
// GMAC1 Offset 0xE02C
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union gmac_status {
    pub bits32: c_uint,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bit1_002c {
// Link status
    pub link:1: c_uint,
// Link speed(00->2.5M 01->25M 10->125M)
    pub speed:2: c_uint,
// Duplex mode
    pub duplex:1: c_uint,
    pub reserved_1:1: c_uint,
// PHY interface type
    pub mii_rmii:2: c_uint,
    pub reserved_2:25: c_uint,
    pub bits: },
}

pub const GMAC_SPEED_10: c_int = 0;
pub const GMAC_SPEED_100: c_int = 1;
pub const GMAC_SPEED_1000: c_int = 2;
pub const GMAC_PHY_MII: c_int = 0;
pub const GMAC_PHY_GMII: c_int = 1;
pub const GMAC_PHY_RGMII_100_10: c_int = 2;
pub const GMAC_PHY_RGMII_1000: c_int = 3;
// Queue Header
// (1) TOE Queue Header
// (2) Non-TOE Queue Header
// (3) Interrupt Queue Header
//
// memory Layout
// TOE Queue Header
// 0x60003000 +---------------------------+ 0x0000
// |     TOE Queue 0 Header    |
// |         8 * 4 Bytes	    |
// +---------------------------+ 0x0020
// |     TOE Queue 1 Header    |
// |         8 * 4 Bytes	    |
// +---------------------------+ 0x0040
// |          ......           |
// |                           |
// +---------------------------+
//
// Non TOE Queue Header
// 0x60002000 +---------------------------+ 0x0000
// |   Default Queue 0 Header  |
// |         2 * 4 Bytes       |
// +---------------------------+ 0x0008
// |   Default Queue 1 Header  |
// |         2 * 4 Bytes       |
// +---------------------------+ 0x0010
// |   Classification Queue 0  |
// |	  2 * 4 Bytes       |
// +---------------------------+
// |   Classification Queue 1  |
// |	  2 * 4 Bytes       |
// +---------------------------+ (n * 8 + 0x10)
// |		...	    |
// |	  2 * 4 Bytes	    |
// +---------------------------+ (13 * 8 + 0x10)
// |   Classification Queue 13 |
// |	  2 * 4 Bytes	    |
// +---------------------------+ 0x80
// |      Interrupt Queue 0    |
// |	  2 * 4 Bytes	    |
// +---------------------------+
// |      Interrupt Queue 1    |
// |	  2 * 4 Bytes	    |
// +---------------------------+
// |      Interrupt Queue 2    |
// |	  2 * 4 Bytes	    |
// +---------------------------+
// |      Interrupt Queue 3    |
// |	  2 * 4 Bytes	    |
// +---------------------------+
//

// NONTOE Queue Header Word 0
#[repr(C)]
#[derive(Copy, Clone)]
pub union nontoe_qhdr0 {
    pub bits32: c_uint,
    pub base_size: c_uint,
}

// NONTOE Queue Header Word 1
#[repr(C)]
#[derive(Copy, Clone)]
pub union nontoe_qhdr1 {
    pub bits32: c_uint,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bit_nonqhdr1 {
// bit 15:0
    pub rptr:16: c_uint,
// bit 31:16
    pub wptr:16: c_uint,
    pub bits: },
}

// Non-TOE Queue Header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nontoe_qhdr {
    pub word0: nontoe_qhdr0,
    pub word1: nontoe_qhdr1,
}
