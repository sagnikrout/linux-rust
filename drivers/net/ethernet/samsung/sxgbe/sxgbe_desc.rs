//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/samsung/sxgbe/sxgbe_desc.h
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
// 10G controller driver for Samsung SoCs
//
// Copyright (C) 2013 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Author: Siva Reddy Kallam <siva.kallam@samsung.com>
//
pub const SXGBE_DESC_SIZE_BYTES: c_int = 16;
// forward declaration
// Transmit checksum insertion control
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tdes_csum_insertion {
    cic_disabled		= 0,	/* Checksum Insertion Control */
    cic_only_ip		= 1,	/* Only IP header */
// IP header but pseudoheader is not calculated
    cic_no_pseudoheader	= 2,
    cic_full		= 3,	/* IP header and pseudoheader */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sxgbe_tx_norm_desc {
    pub /: *mut *mut u64 tdes01; / buf1 address,
// TX Read-Format Desc 2,3
// TDES2
    pub buf1_size:14: u32,
    pub vlan_tag_ctl:2: u32,
    pub buf2_size:14: u32,
    pub timestmp_enable:1: u32,
    pub int_on_com:1: u32,
// TDES3
    pub tcp_payload_len: u16,
    pub total_pkt_len:15: u32,
    pub reserved1:1: u32,
    pub pkt_len: },
    pub tx_pkt_len: },
    pub cksum_ctl:2: u16,
    pub tse_bit:1: u16,
    pub tcp_hdr_len:4: u16,
    pub sa_insert_ctl:3: u16,
    pub crc_pad_ctl:2: u16,
    pub last_desc:1: u16,
    pub first_desc:1: u16,
    pub ctxt_bit:1: u16,
    pub own_bit:1: u16,
    pub tx_rd_des23: },
// tx write back Desc 2,3
// WB TES2
    pub reserved1: u32,
// WB TES3
    pub reserved2:31: u32,
    pub own_bit:1: u32,
    pub tx_wb_des23: },
    pub tdes23: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sxgbe_rx_norm_desc {
    pub /: *mut *mut u64 rdes01; / buf1 address,
    pub out_vlan_tag:16: u32,
    pub in_vlan_tag:16: u32,
    pub rss_hash: u32,
    pub rx_wb_des01: },
    pub rdes01: },
// RX Read format Desc 2,3
// RDES2
    pub buf2_addr:62: u64,
// RDES3
    pub int_on_com:1: u32,
    pub own_bit:1: u32,
    pub rx_rd_des23: },
// RX write back
// WB RDES2
    pub hdr_len:10: u32,
    pub rdes2_reserved:2: u32,
    pub elrd_val:1: u32,
    pub iovt_sel:1: u32,
    pub res_pkt:1: u32,
    pub vlan_filter_match:1: u32,
    pub sa_filter_fail:1: u32,
    pub da_filter_fail:1: u32,
    pub hash_filter_pass:1: u32,
    pub macaddr_filter_match:8: u32,
    pub l3_filter_match:1: u32,
    pub l4_filter_match:1: u32,
    pub l34_filter_num:3: u32,
// WB RDES3
    pub pkt_len:14: u32,
    pub rdes3_reserved:1: u32,
    pub err_summary:1: u32,
    pub err_l2_type:4: u32,
    pub layer34_pkt_type:4: u32,
    pub no_coagulation_pkt:1: u32,
    pub in_seq_pkt:1: u32,
    pub rss_valid:1: u32,
    pub context_des_avail:1: u32,
    pub last_desc:1: u32,
    pub first_desc:1: u32,
    pub recv_context_desc:1: u32,
    pub own_bit:1: u32,
    pub rx_wb_des23: },
    pub rdes23: },
}

// Context descriptor structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sxgbe_tx_ctxt_desc {
    pub tstamp_lo: u32,
    pub tstamp_hi: u32,
    pub maxseg_size:15: u32,
    pub reserved1:1: u32,
    pub ivlan_tag:16: u32,
    pub vlan_tag:16: u32,
    pub vltag_valid:1: u32,
    pub ivlan_tag_valid:1: u32,
    pub ivlan_tag_ctl:2: u32,
    pub reserved2:3: u32,
    pub ctxt_desc_err:1: u32,
    pub reserved3:2: u32,
    pub ostc:1: u32,
    pub tcmssv:1: u32,
    pub reserved4:2: u32,
    pub ctxt_bit:1: u32,
    pub own_bit:1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sxgbe_rx_ctxt_desc {
    pub tstamp_lo: u32,
    pub tstamp_hi: u32,
    pub reserved1: u32,
    pub ptp_msgtype:4: u32,
    pub tstamp_available:1: u32,
    pub ptp_rsp_err:1: u32,
    pub tstamp_dropped:1: u32,
    pub reserved2:23: u32,
    pub rx_ctxt_desc:1: u32,
    pub own_bit:1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sxgbe_desc_ops {
// DMA TX descriptor ring initialization
    pub p): *mut *mut void (init_tx_desc)(struct sxgbe_tx_norm_desc,
// Invoked by the xmit function to prepare the tx descriptor
    pub tcp_payload_len): u32,
// Assign buffer lengths for descriptor
    pub cksum): int buf1_len, int pkt_len, int,
// Set VLAN control information
    pub vlan_ctl): *mut *mut *mut void (tx_vlanctl_desc)(struct sxgbe_tx_norm_desc p, int,
// Set the owner of the descriptor
    pub p): *mut *mut void (set_tx_owner)(struct sxgbe_tx_norm_desc,
// Get the owner of the descriptor
    pub p): *mut *mut int (get_tx_owner)(struct sxgbe_tx_norm_desc,
// Invoked by the xmit function to close the tx descriptor
    pub p): *mut *mut void (close_tx_desc)(struct sxgbe_tx_norm_desc,
// Clean the tx descriptor as soon as the tx irq is received
    pub p): *mut *mut void (release_tx_desc)(struct sxgbe_tx_norm_desc,
// Clear interrupt on tx frame completion. When this bit is
// set an interrupt happens as soon as the frame is transmitted
//
    pub p): *mut *mut void (clear_tx_ic)(struct sxgbe_tx_norm_desc,
// Last tx segment reports the transmit status
    pub p): *mut *mut int (get_tx_ls)(struct sxgbe_tx_norm_desc,
// Get the buffer size from the descriptor
    pub p): *mut *mut int (get_tx_len)(struct sxgbe_tx_norm_desc,
// Set tx timestamp enable bit
    pub p): *mut *mut void (tx_enable_tstamp)(struct sxgbe_tx_norm_desc,
// get tx timestamp status
    pub p): *mut *mut int (get_tx_timestamp_status)(struct sxgbe_tx_norm_desc,
// TX Context Descripto Specific
    pub p): *mut *mut void (tx_ctxt_desc_set_ctxt)(struct sxgbe_tx_ctxt_desc,
// Set the owner of the TX context descriptor
    pub p): *mut *mut void (tx_ctxt_desc_set_owner)(struct sxgbe_tx_ctxt_desc,
// Get the owner of the TX context descriptor
    pub p): *mut *mut int (get_tx_ctxt_owner)(struct sxgbe_tx_ctxt_desc,
// Set TX mss
    pub mss): *mut *mut *mut void (tx_ctxt_desc_set_mss)(struct sxgbe_tx_ctxt_desc p, u16,
// Set TX mss
    pub p): *mut *mut int (tx_ctxt_desc_get_mss)(struct sxgbe_tx_ctxt_desc,
// Set TX tcmssv
    pub p): *mut *mut void (tx_ctxt_desc_set_tcmssv)(struct sxgbe_tx_ctxt_desc,
// Reset TX ostc
    pub p): *mut *mut void (tx_ctxt_desc_reset_ostc)(struct sxgbe_tx_ctxt_desc,
// Set IVLAN information
    pub ivlan_ctl): c_int,
// Return IVLAN Tag
    pub p): *mut *mut int (tx_ctxt_desc_get_ivlantag)(struct sxgbe_tx_ctxt_desc,
// Set VLAN Tag
    pub vlan_tag): int is_vlanvalid, int,
// Return VLAN Tag
    pub p): *mut *mut int (tx_ctxt_desc_get_vlantag)(struct sxgbe_tx_ctxt_desc,
// Set Time stamp
    pub tstamp): u8 ostc_enable, u64,
// Close TX context descriptor
    pub p): *mut *mut void (close_tx_ctxt_desc)(struct sxgbe_tx_ctxt_desc,
// WB status of context descriptor
    pub p): *mut *mut int (get_tx_ctxt_cde)(struct sxgbe_tx_ctxt_desc,
// DMA RX descriptor ring initialization
    pub end): int mode, int,
// Get own bit
    pub p): *mut *mut int (get_rx_owner)(struct sxgbe_rx_norm_desc,
// Set own bit
    pub p): *mut *mut void (set_rx_owner)(struct sxgbe_rx_norm_desc,
// Set Interrupt on completion bit
    pub p): *mut *mut void (set_rx_int_on_com)(struct sxgbe_rx_norm_desc,
// Get the receive frame size
    pub p): *mut *mut int (get_rx_frame_len)(struct sxgbe_rx_norm_desc,
// Return first Descriptor status
    pub p): *mut *mut int (get_rx_fd_status)(struct sxgbe_rx_norm_desc,
// Return first Descriptor status
    pub p): *mut *mut int (get_rx_ld_status)(struct sxgbe_rx_norm_desc,
// Return the reception status looking at the RDES1
    pub checksum): *mut *mut sxgbe_extra_stats x, int,
// Get own bit
    pub p): *mut *mut int (get_rx_ctxt_owner)(struct sxgbe_rx_ctxt_desc,
// Set own bit
    pub p): *mut *mut void (set_rx_ctxt_owner)(struct sxgbe_rx_ctxt_desc,
// Return the reception status looking at Context control information
    pub x): *mut sxgbe_extra_stats,
// Get rx timestamp status
    pub p): *mut *mut int (get_rx_ctxt_tstamp_status)(struct sxgbe_rx_ctxt_desc,
// Get timestamp value for rx, need to check this
    pub p): *mut *mut u64 (get_timestamp)(struct sxgbe_rx_ctxt_desc,
}
