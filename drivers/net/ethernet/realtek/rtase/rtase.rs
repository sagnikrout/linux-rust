//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/realtek/rtase/rtase.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// rtase is the Linux device driver released for Realtek Automotive Switch
// controllers with PCI-Express interface.
//
// Copyright(c) 2024 Realtek Semiconductor Corp.
//
pub const RTASE_HW_VER_MASK: c_uint = 0x7C800000;
pub const RTASE_HW_VER_906X_7XA: c_uint = 0x00800000;
pub const RTASE_HW_VER_906X_7XC: c_uint = 0x04000000;
pub const RTASE_HW_VER_907XD_V1: c_uint = 0x04800000;
pub const RTASE_HW_VER_907XD_VA: c_uint = 0x08000000;
pub const RTASE_RX_DMA_BURST_256: c_int = 4;
pub const RTASE_TX_DMA_BURST_UNLIMITED: c_int = 7;

// 3 means InterFrameGap = the shortest one
pub const RTASE_INTERFRAMEGAP: c_uint = 0x03;
pub const RTASE_REGS_SIZE: c_int = 256;
pub const RTASE_PCI_REGS_SIZE: c_uint = 0x100;

pub const RTASE_VLAN_FILTER_ENTRY_NUM: c_int = 32;
pub const RTASE_NUM_TX_QUEUE: c_int = 8;
pub const RTASE_NUM_RX_QUEUE: c_int = 4;
pub const RTASE_TXQ_CTRL: c_int = 1;
pub const RTASE_FUNC_TXQ_NUM: c_int = 1;
pub const RTASE_FUNC_RXQ_NUM: c_int = 1;
pub const RTASE_INTERRUPT_NUM: c_int = 1;

pub const RTASE_MITI_DEFAULT_TIME: c_int = 128;
pub const RTASE_MITI_MAX_TIME: c_int = 491520;

pub const RTASE_MITI_DEFAULT_PKT_NUM: c_int = 64;
pub const RTASE_MITI_MAX_PKT_NUM_IDX: c_int = 3;
pub const RTASE_MITI_MAX_PKT_NUM_UNIT: c_int = 16;
pub const RTASE_MITI_MAX_PKT_NUM: c_int = 240;
pub const RTASE_MITI_COUNT_BIT_NUM: c_int = 4;
pub const RTASE_NUM_MSIX: c_int = 4;
pub const RTASE_DWORD_MOD: c_int = 16;
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtase_registers {
    RTASE_MAC0   = 0x0000,
    RTASE_MAC4   = 0x0004,
    RTASE_MAR0   = 0x0008,
    RTASE_MAR1   = 0x000C,
    RTASE_DTCCR0 = 0x0010,
    RTASE_DTCCR4 = 0x0014,

    RTASE_FCR = 0x0018,

    RTASE_LBK_CTRL = 0x001A,

    RTASE_TX_DESC_ADDR0   = 0x0020,
    RTASE_TX_DESC_ADDR4   = 0x0024,
    RTASE_TX_DESC_COMMAND = 0x0028,

    RTASE_BOOT_CTL  = 0x6004,
    RTASE_CLKSW_SET = 0x6018,

    RTASE_CHIP_CMD = 0x0037,

    RTASE_IMR0 = 0x0038,
    RTASE_ISR0 = 0x003C,

    RTASE_IMR1 = 0x0800,
    RTASE_ISR1 = 0x0802,

    RTASE_EPHY_ISR = 0x6014,
    RTASE_EPHY_IMR = 0x6016,

    RTASE_TX_CONFIG_0 = 0x0040,

// DMA burst value (0-7) is shift this many bits

    RTASE_RX_CONFIG_0 = 0x0044,

    RTASE_ACCEPT_RUNT | RTASE_ACCEPT_BROADCAST | \
    RTASE_ACCEPT_MULTICAST | RTASE_ACCEPT_MYPHYS | \
    RTASE_ACCEPT_ALLPHYS)

    RTASE_RX_CONFIG_1 = 0x0046,

    RTASE_EEM = 0x0050,
pub const RTASE_EEM_UNLOCK: c_uint = 0xC0;

    RTASE_TDFNR  = 0x0057,
    RTASE_TPPOLL = 0x0090,
    RTASE_PDR    = 0x00B0,
    RTASE_FIFOR  = 0x00D3,

    RTASE_RMS       = 0x00DA,
    RTASE_CPLUS_CMD = 0x00E0,

    RTASE_GPHY_STD_00 = 0x6024,

    RTASE_Q0_RX_DESC_ADDR0 = 0x00E4,
    RTASE_Q0_RX_DESC_ADDR4 = 0x00E8,
    RTASE_Q1_RX_DESC_ADDR0 = 0x4000,
    RTASE_Q1_RX_DESC_ADDR4 = 0x4004,
    RTASE_MTPS             = 0x00EC,

    RTASE_MISC = 0x00F2,

    RTASE_TFUN_CTRL = 0x0400,

    RTASE_TX_CONFIG_1 = 0x203E,

    RTASE_TOKSEL      = 0x2046,
    RTASE_TXQCRDT_0   = 0x2500,
    RTASE_RFIFONFULL  = 0x4406,
    RTASE_INT_MITI_TX = 0x0A00,
    RTASE_INT_MITI_RX = 0x0A80,

    RTASE_VLAN_ENTRY_0 = 0xAC80,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtase_desc_status_bit {
    RTASE_DESC_OWN = BIT(31), /* Descriptor is owned by NIC */
    RTASE_RING_END = BIT(30), /* End of descriptor ring */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtase_sw_flag_content {
    RTASE_SWF_MSI_ENABLED  = BIT(1),
    RTASE_SWF_MSIX_ENABLED = BIT(2),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtase_parse_result {
    RTASE_PARSE_OK,
    RTASE_PARSE_SKIP,
    RTASE_PARSE_DROP,
}

pub const RSVD_MASK: c_uint = 0x3FFFC000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtase_tx_desc {
    pub opts1: __le32,
    pub opts2: __le32,
    pub addr: __le64,
    pub opts3: __le32,
    pub reserved1: __le32,
    pub reserved2: __le32,
    pub reserved3: __le32,
    pub __packed: },
// ------ offset 0 of tx descriptor ------

// ------ offset 4 of tx descriptor ------

#[repr(C)]
#[derive(Copy, Clone)]
pub union rtase_rx_desc {
    pub header_buf_addr: __le64,
    pub reserved1: __le32,
    pub opts_header_len: __le32,
    pub addr: __le64,
    pub reserved2: __le32,
    pub opts1: __le32,
    pub desc_cmd: } __packed,
    pub reserved1: __le32,
    pub reserved2: __le32,
    pub rss: __le32,
    pub opts4: __le32,
    pub reserved3: __le32,
    pub opts3: __le32,
    pub opts2: __le32,
    pub opts1: __le32,
    pub desc_status: } __packed,
    pub __packed: },
// ------ offset 28 of rx descriptor ------

pub const RTASE_NUM_DESC: c_int = 1024;
pub const RTASE_TX_BUDGET_DEFAULT: c_int = 256;

// txqos hardware definitions
pub const RTASE_1T_CLOCK: c_int = 64;
pub const RTASE_1T_POWER: c_int = 10000000;
pub const RTASE_IDLESLOPE_INT_SHIFT: c_int = 25;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtase_int_vector {
    pub tp: *mut rtase_private,
    pub irq: c_uint,
    pub name: [c_char; RTASE_IVEC_NAME_SIZE],
    pub index: u16,
    pub imr_addr: u16,
    pub isr_addr: u16,
    pub imr: u32,
    pub ring_list: list_head,
    pub napi: napi_struct,
    pub budget): *mut *mut *mut int (poll)(struct napi_struct napi, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtase_ring {
    pub ivec: *mut rtase_int_vector,
    pub desc: *mut c_void,
    pub phy_addr: dma_addr_t,
    pub cur_idx: u32,
    pub dirty_idx: u32,
    pub index: u16,
    pub type: u8,
    pub skbuff: [*mut sk_buff; RTASE_NUM_DESC],
    pub data_buf: [*mut c_void; RTASE_NUM_DESC],
    pub len: [u32; RTASE_NUM_DESC],
    pub data_phy_addr: [dma_addr_t; RTASE_NUM_DESC],
    pub mis: },
    pub ring_entry: list_head,
    pub budget): *mut *mut *mut int (ring_handler)(struct rtase_ring ring, int,
    pub alloc_fail: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtase_txqos {
    pub hicredit: c_int,
    pub locredit: c_int,
    pub idleslope: c_int,
    pub sendslope: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtase_stats {
    pub tx_dropped: u64,
    pub rx_dropped: u64,
    pub multicast: u64,
    pub rx_errors: u64,
    pub rx_length_errors: u64,
    pub rx_crc_errors: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtase_private {
    pub mmio_addr: *mut void __iomem,
    pub sw_flag: u32,
    pub pdev: *mut pci_dev,
    pub dev: *mut net_device,
    pub rx_buf_sz: u32,
    pub page_pool: *mut page_pool,
    pub tx_ring: [rtase_ring; RTASE_NUM_TX_QUEUE],
    pub tx_qos: [rtase_txqos; RTASE_NUM_TX_QUEUE],
    pub rx_ring: [rtase_ring; RTASE_NUM_RX_QUEUE],
    pub tally_vaddr: *mut rtase_counters,
    pub tally_paddr: dma_addr_t,
    pub vlan_filter_ctrl: u32,
    pub vlan_filter_vid: [u16; RTASE_VLAN_FILTER_ENTRY_NUM],
    pub msix_entry: [msix_entry; RTASE_NUM_MSIX],
    pub int_vector: [rtase_int_vector; RTASE_NUM_MSIX],
    pub stats: rtase_stats,
    pub tx_queue_ctrl: u16,
    pub func_tx_queue_num: u16,
    pub func_rx_queue_num: u16,
    pub int_nums: u16,
    pub tx_int_mit: u16,
    pub rx_int_mit: u16,
    pub hw_ver: u32,
}

pub const RTASE_LSO_64K: c_int = 64000;

pub const RTASE_MIN_PAD_LEN: c_int = 47;
