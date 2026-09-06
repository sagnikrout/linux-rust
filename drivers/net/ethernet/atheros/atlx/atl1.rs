//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/atheros/atlx/atl1.h
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
// Copyright(c) 2005 - 2006 Attansic Corporation. All rights reserved.
// Copyright(c) 2006 - 2007 Chris Snook <csnook@redhat.com>
// Copyright(c) 2006 - 2008 Jay Cliburn <jcliburn@gmail.com>
//
// Derived from Intel e1000 driver
// Copyright(c) 1999 - 2005 Intel Corporation. All rights reserved.
//

// function prototypes needed by multiple files
extern "C" {
    pub fn atl1_hash_mc_addr(hw: *mut atl1_hw, mc_addr: *mut u8) -> static u32;
}
extern "C" {
    pub fn atl1_hash_set(hw: *mut atl1_hw, hash_value: u32) -> static void;
}
extern "C" {
    pub fn atl1_set_mac_addr(hw: *mut atl1_hw) -> static void;
}
extern "C" {
    pub fn atl1_check_link(adapter: *mut atl1_adapter) -> static u32;
}
// hardware definitions specific to L1
// Block IDLE Status Register
pub const IDLE_STATUS_RXMAC: c_uint = 0x1;
pub const IDLE_STATUS_TXMAC: c_uint = 0x2;
pub const IDLE_STATUS_RXQ: c_uint = 0x4;
pub const IDLE_STATUS_TXQ: c_uint = 0x8;
pub const IDLE_STATUS_DMAR: c_uint = 0x10;
pub const IDLE_STATUS_DMAW: c_uint = 0x20;
pub const IDLE_STATUS_SMB: c_uint = 0x40;
pub const IDLE_STATUS_CMB: c_uint = 0x80;
// MDIO Control Register
pub const MDIO_WAIT_TIMES: c_int = 30;
// MAC Control Register
pub const MAC_CTRL_TX_PAUSE: c_uint = 0x10000;
pub const MAC_CTRL_SCNT: c_uint = 0x20000;
pub const MAC_CTRL_SRST_TX: c_uint = 0x40000;
pub const MAC_CTRL_TX_SIMURST: c_uint = 0x80000;
pub const MAC_CTRL_SPEED_SHIFT: c_int = 20;
pub const MAC_CTRL_SPEED_MASK: c_uint = 0x300000;
pub const MAC_CTRL_SPEED_1000: c_uint = 0x2;
pub const MAC_CTRL_SPEED_10_100: c_uint = 0x1;
pub const MAC_CTRL_DBG_TX_BKPRESURE: c_uint = 0x400000;
pub const MAC_CTRL_TX_HUGE: c_uint = 0x800000;
pub const MAC_CTRL_RX_CHKSUM_EN: c_uint = 0x1000000;
pub const MAC_CTRL_DBG: c_uint = 0x8000000;
// Wake-On-Lan control register
pub const WOL_CLK_SWITCH_EN: c_uint = 0x8000;
pub const WOL_PT5_EN: c_uint = 0x200000;
pub const WOL_PT6_EN: c_uint = 0x400000;
pub const WOL_PT5_MATCH: c_uint = 0x8000000;
pub const WOL_PT6_MATCH: c_uint = 0x10000000;
// WOL Length ( 2 DWORD )
pub const REG_WOL_PATTERN_LEN: c_uint = 0x14A4;
pub const WOL_PT_LEN_MASK: c_uint = 0x7F;
pub const WOL_PT0_LEN_SHIFT: c_int = 0;
pub const WOL_PT1_LEN_SHIFT: c_int = 8;
pub const WOL_PT2_LEN_SHIFT: c_int = 16;
pub const WOL_PT3_LEN_SHIFT: c_int = 24;
pub const WOL_PT4_LEN_SHIFT: c_int = 0;
pub const WOL_PT5_LEN_SHIFT: c_int = 8;
pub const WOL_PT6_LEN_SHIFT: c_int = 16;
// Internal SRAM Partition Registers, low 32 bits
pub const REG_SRAM_RFD_LEN: c_uint = 0x1504;
pub const REG_SRAM_RRD_ADDR: c_uint = 0x1508;
pub const REG_SRAM_RRD_LEN: c_uint = 0x150C;
pub const REG_SRAM_TPD_ADDR: c_uint = 0x1510;
pub const REG_SRAM_TPD_LEN: c_uint = 0x1514;
pub const REG_SRAM_TRD_ADDR: c_uint = 0x1518;
pub const REG_SRAM_TRD_LEN: c_uint = 0x151C;
pub const REG_SRAM_RXF_ADDR: c_uint = 0x1520;
pub const REG_SRAM_RXF_LEN: c_uint = 0x1524;
pub const REG_SRAM_TXF_ADDR: c_uint = 0x1528;
pub const REG_SRAM_TXF_LEN: c_uint = 0x152C;
pub const REG_SRAM_TCPH_PATH_ADDR: c_uint = 0x1530;
pub const SRAM_TCPH_ADDR_MASK: c_uint = 0xFFF;
pub const SRAM_TCPH_ADDR_SHIFT: c_int = 0;
pub const SRAM_PATH_ADDR_MASK: c_uint = 0xFFF;
pub const SRAM_PATH_ADDR_SHIFT: c_int = 16;
// Load Ptr Register
pub const REG_LOAD_PTR: c_uint = 0x1534;
// Descriptor Control registers, low 32 bits
pub const REG_DESC_RFD_ADDR_LO: c_uint = 0x1544;
pub const REG_DESC_RRD_ADDR_LO: c_uint = 0x1548;
pub const REG_DESC_TPD_ADDR_LO: c_uint = 0x154C;
pub const REG_DESC_CMB_ADDR_LO: c_uint = 0x1550;
pub const REG_DESC_SMB_ADDR_LO: c_uint = 0x1554;
pub const REG_DESC_RFD_RRD_RING_SIZE: c_uint = 0x1558;
pub const DESC_RFD_RING_SIZE_MASK: c_uint = 0x7FF;
pub const DESC_RFD_RING_SIZE_SHIFT: c_int = 0;
pub const DESC_RRD_RING_SIZE_MASK: c_uint = 0x7FF;
pub const DESC_RRD_RING_SIZE_SHIFT: c_int = 16;
pub const REG_DESC_TPD_RING_SIZE: c_uint = 0x155C;
pub const DESC_TPD_RING_SIZE_MASK: c_uint = 0x3FF;
pub const DESC_TPD_RING_SIZE_SHIFT: c_int = 0;
// TXQ Control Register
pub const REG_TXQ_CTRL: c_uint = 0x1580;
pub const TXQ_CTRL_TPD_BURST_NUM_SHIFT: c_int = 0;
pub const TXQ_CTRL_TPD_BURST_NUM_MASK: c_uint = 0x1F;
pub const TXQ_CTRL_EN: c_uint = 0x20;
pub const TXQ_CTRL_ENH_MODE: c_uint = 0x40;
pub const TXQ_CTRL_TPD_FETCH_TH_SHIFT: c_int = 8;
pub const TXQ_CTRL_TPD_FETCH_TH_MASK: c_uint = 0x3F;
pub const TXQ_CTRL_TXF_BURST_NUM_SHIFT: c_int = 16;
pub const TXQ_CTRL_TXF_BURST_NUM_MASK: c_uint = 0xFFFF;
// Jumbo packet Threshold for task offload
pub const REG_TX_JUMBO_TASK_TH_TPD_IPG: c_uint = 0x1584;
pub const TX_JUMBO_TASK_TH_MASK: c_uint = 0x7FF;
pub const TX_JUMBO_TASK_TH_SHIFT: c_int = 0;
pub const TX_TPD_MIN_IPG_MASK: c_uint = 0x1F;
pub const TX_TPD_MIN_IPG_SHIFT: c_int = 16;
// RXQ Control Register
pub const REG_RXQ_CTRL: c_uint = 0x15A0;
pub const RXQ_CTRL_RFD_BURST_NUM_SHIFT: c_int = 0;
pub const RXQ_CTRL_RFD_BURST_NUM_MASK: c_uint = 0xFF;
pub const RXQ_CTRL_RRD_BURST_THRESH_SHIFT: c_int = 8;
pub const RXQ_CTRL_RRD_BURST_THRESH_MASK: c_uint = 0xFF;
pub const RXQ_CTRL_RFD_PREF_MIN_IPG_SHIFT: c_int = 16;
pub const RXQ_CTRL_RFD_PREF_MIN_IPG_MASK: c_uint = 0x1F;
pub const RXQ_CTRL_CUT_THRU_EN: c_uint = 0x40000000;
pub const RXQ_CTRL_EN: c_uint = 0x80000000;
// Rx jumbo packet threshold and rrd  retirement timer
pub const REG_RXQ_JMBOSZ_RRDTIM: c_uint = 0x15A4;
pub const RXQ_JMBOSZ_TH_MASK: c_uint = 0x7FF;
pub const RXQ_JMBOSZ_TH_SHIFT: c_int = 0;
pub const RXQ_JMBO_LKAH_MASK: c_uint = 0xF;
pub const RXQ_JMBO_LKAH_SHIFT: c_int = 11;
pub const RXQ_RRD_TIMER_MASK: c_uint = 0xFFFF;
pub const RXQ_RRD_TIMER_SHIFT: c_int = 16;
// RFD flow control register
pub const REG_RXQ_RXF_PAUSE_THRESH: c_uint = 0x15A8;
pub const RXQ_RXF_PAUSE_TH_HI_SHIFT: c_int = 16;
pub const RXQ_RXF_PAUSE_TH_HI_MASK: c_uint = 0xFFF;
pub const RXQ_RXF_PAUSE_TH_LO_SHIFT: c_int = 0;
pub const RXQ_RXF_PAUSE_TH_LO_MASK: c_uint = 0xFFF;
// RRD flow control register
pub const REG_RXQ_RRD_PAUSE_THRESH: c_uint = 0x15AC;
pub const RXQ_RRD_PAUSE_TH_HI_SHIFT: c_int = 0;
pub const RXQ_RRD_PAUSE_TH_HI_MASK: c_uint = 0xFFF;
pub const RXQ_RRD_PAUSE_TH_LO_SHIFT: c_int = 16;
pub const RXQ_RRD_PAUSE_TH_LO_MASK: c_uint = 0xFFF;
// DMA Engine Control Register
pub const REG_DMA_CTRL: c_uint = 0x15C0;
pub const DMA_CTRL_DMAR_IN_ORDER: c_uint = 0x1;
pub const DMA_CTRL_DMAR_ENH_ORDER: c_uint = 0x2;
pub const DMA_CTRL_DMAR_OUT_ORDER: c_uint = 0x4;
pub const DMA_CTRL_RCB_VALUE: c_uint = 0x8;
pub const DMA_CTRL_DMAR_BURST_LEN_SHIFT: c_int = 4;
pub const DMA_CTRL_DMAR_BURST_LEN_MASK: c_int = 7;
pub const DMA_CTRL_DMAW_BURST_LEN_SHIFT: c_int = 7;
pub const DMA_CTRL_DMAW_BURST_LEN_MASK: c_int = 7;
pub const DMA_CTRL_DMAR_EN: c_uint = 0x400;
pub const DMA_CTRL_DMAW_EN: c_uint = 0x800;
// CMB/SMB Control Register
pub const REG_CSMB_CTRL: c_uint = 0x15D0;
pub const CSMB_CTRL_CMB_NOW: c_int = 1;
pub const CSMB_CTRL_SMB_NOW: c_int = 2;
pub const CSMB_CTRL_CMB_EN: c_int = 4;
pub const CSMB_CTRL_SMB_EN: c_int = 8;
// CMB DMA Write Threshold Register
pub const REG_CMB_WRITE_TH: c_uint = 0x15D4;
pub const CMB_RRD_TH_SHIFT: c_int = 0;
pub const CMB_RRD_TH_MASK: c_uint = 0x7FF;
pub const CMB_TPD_TH_SHIFT: c_int = 16;
pub const CMB_TPD_TH_MASK: c_uint = 0x7FF;
// RX/TX count-down timer to trigger CMB-write. 2us resolution.
pub const REG_CMB_WRITE_TIMER: c_uint = 0x15D8;
pub const CMB_RX_TM_SHIFT: c_int = 0;
pub const CMB_RX_TM_MASK: c_uint = 0xFFFF;
pub const CMB_TX_TM_SHIFT: c_int = 16;
pub const CMB_TX_TM_MASK: c_uint = 0xFFFF;
// Number of packet received since last CMB write
pub const REG_CMB_RX_PKT_CNT: c_uint = 0x15DC;
// Number of packet transmitted since last CMB write
pub const REG_CMB_TX_PKT_CNT: c_uint = 0x15E0;
// SMB auto DMA timer register
pub const REG_SMB_TIMER: c_uint = 0x15E4;
// Mailbox Register
pub const REG_MAILBOX: c_uint = 0x15F0;
pub const MB_RFD_PROD_INDX_SHIFT: c_int = 0;
pub const MB_RFD_PROD_INDX_MASK: c_uint = 0x7FF;
pub const MB_RRD_CONS_INDX_SHIFT: c_int = 11;
pub const MB_RRD_CONS_INDX_MASK: c_uint = 0x7FF;
pub const MB_TPD_PROD_INDX_SHIFT: c_int = 22;
pub const MB_TPD_PROD_INDX_MASK: c_uint = 0x3FF;
// Interrupt Status Register
pub const ISR_SMB: c_uint = 0x1;
pub const ISR_TIMER: c_uint = 0x2;
pub const ISR_MANUAL: c_uint = 0x4;
pub const ISR_RXF_OV: c_uint = 0x8;
pub const ISR_RFD_UNRUN: c_uint = 0x10;
pub const ISR_RRD_OV: c_uint = 0x20;
pub const ISR_TXF_UNRUN: c_uint = 0x40;
pub const ISR_LINK: c_uint = 0x80;
pub const ISR_HOST_RFD_UNRUN: c_uint = 0x100;
pub const ISR_HOST_RRD_OV: c_uint = 0x200;
pub const ISR_DMAR_TO_RST: c_uint = 0x400;
pub const ISR_DMAW_TO_RST: c_uint = 0x800;
pub const ISR_GPHY: c_uint = 0x1000;
pub const ISR_RX_PKT: c_uint = 0x10000;
pub const ISR_TX_PKT: c_uint = 0x20000;
pub const ISR_TX_DMA: c_uint = 0x40000;
pub const ISR_RX_DMA: c_uint = 0x80000;
pub const ISR_CMB_RX: c_uint = 0x100000;
pub const ISR_CMB_TX: c_uint = 0x200000;
pub const ISR_MAC_RX: c_uint = 0x400000;
pub const ISR_MAC_TX: c_uint = 0x800000;
pub const ISR_DIS_SMB: c_uint = 0x20000000;
pub const ISR_DIS_DMA: c_uint = 0x40000000;
// Normal Interrupt mask without RX/TX enabled

// Normal Interrupt mask

// Debug Interrupt Mask  (enable all interrupt)

pub const MEDIA_TYPE_1000M_FULL: c_int = 1;
pub const MEDIA_TYPE_100M_FULL: c_int = 2;
pub const MEDIA_TYPE_100M_HALF: c_int = 3;
pub const MEDIA_TYPE_10M_FULL: c_int = 4;
pub const MEDIA_TYPE_10M_HALF: c_int = 5;
pub const AUTONEG_ADVERTISE_SPEED_DEFAULT: c_uint = 0x002F	/* All but 1000-Half */;
pub const MAX_JUMBO_FRAME_SIZE: c_int = 10240;
pub const ATL1_EEDUMP_LEN: c_int = 48;
// Statistics counters collected by the MAC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stats_msg_block {
// rx
    pub /: *mut *mut u32 rx_ok; / good RX packets,
    pub /: *mut *mut u32 rx_bcast; / good RX broadcast packets,
    pub /: *mut *mut u32 rx_mcast; / good RX multicast packets,
    pub /: *mut *mut u32 rx_pause; / RX pause frames,
    pub /: *mut *mut u32 rx_ctrl; / RX control packets other than pause frames,
    pub /: *mut *mut u32 rx_fcs_err; / RX packets with bad FCS,
    pub /: *mut *mut u32 rx_len_err; / RX packets with length != actual size,
    pub /: *mut *mut u32 rx_byte_cnt; / good bytes received. FCS is NOT included,
    pub /: *mut *mut u32 rx_runt; / RX packets < 64 bytes with good FCS,
    pub /: *mut *mut u32 rx_frag; / RX packets < 64 bytes with bad FCS,
    pub /: *mut *mut u32 rx_sz_64; / 64 byte RX packets,
    pub rx_sz_65_127: u32,
    pub rx_sz_128_255: u32,
    pub rx_sz_256_511: u32,
    pub rx_sz_512_1023: u32,
    pub rx_sz_1024_1518: u32,
    pub /: *mut *mut u32 rx_sz_1519_max; / 1519 byte to MTU RX packets,
    pub /: *mut *mut u32 rx_sz_ov; / truncated RX packets > MTU,
    pub /: *mut *mut u32 rx_rxf_ov; / frames dropped due to RX FIFO overflow,
    pub /: *mut *mut u32 rx_rrd_ov; / frames dropped due to RRD overflow,
    pub /: *mut *mut u32 rx_align_err; / alignment errors,
    pub /: *mut *mut u32 rx_bcast_byte_cnt; / RX broadcast bytes, excluding FCS,
    pub /: *mut *mut u32 rx_mcast_byte_cnt; / RX multicast bytes, excluding FCS,
    pub /: *mut *mut u32 rx_err_addr; / packets dropped due to address filtering,
// tx
    pub /: *mut *mut u32 tx_ok; / good TX packets,
    pub /: *mut *mut u32 tx_bcast; / good TX broadcast packets,
    pub /: *mut *mut u32 tx_mcast; / good TX multicast packets,
    pub /: *mut *mut u32 tx_pause; / TX pause frames,
    pub /: *mut *mut u32 tx_exc_defer; / TX packets deferred excessively,
    pub /: *mut *mut u32 tx_ctrl; / TX control frames, excluding pause frames,
    pub /: *mut *mut u32 tx_defer; / TX packets deferred,
    pub /: *mut *mut u32 tx_byte_cnt; / bytes transmitted, FCS is NOT included,
    pub /: *mut *mut u32 tx_sz_64; / 64 byte TX packets,
    pub tx_sz_65_127: u32,
    pub tx_sz_128_255: u32,
    pub tx_sz_256_511: u32,
    pub tx_sz_512_1023: u32,
    pub tx_sz_1024_1518: u32,
    pub /: *mut *mut u32 tx_sz_1519_max; / 1519 byte to MTU TX packets,
    pub /: *mut *mut u32 tx_1_col; / packets TX after a single collision,
    pub /: *mut *mut u32 tx_2_col; / packets TX after multiple collisions,
    pub /: *mut *mut u32 tx_late_col; / TX packets with late collisions,
    pub /: *mut *mut u32 tx_abort_col; / TX packets aborted w/excessive collisions,
    pub underrun: *mut *mut u32 tx_underrun; / TX packets aborted due to TX FIFO,
// or TRD FIFO underrun
    pub frame: *mut *mut u32 tx_rd_eop; / reads beyond the EOP into the next,
// when TRD was not written timely
    pub /: *mut *mut u32 tx_len_err; / TX packets where length != actual size,
    pub /: *mut *mut u32 tx_trunc; / TX packets truncated due to size > MTU,
    pub /: *mut *mut u32 tx_bcast_byte; / broadcast bytes transmitted, excluding FCS,
    pub /: *mut *mut u32 tx_mcast_byte; / multicast bytes transmitted, excluding FCS,
    pub to: *mut *mut u32 smb_updated; / 1: SMB Updated. This is used by software,
// indicate the statistics update. Software
// should clear this bit after retrieving the
// statistics information.
}

// Coalescing Message Block
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coals_msg_block {
    pub /: *mut *mut u32 int_stats; / interrupt status,
    pub /: *mut *mut u16 rrd_prod_idx; / TRD Producer Index.,
    pub /: *mut *mut u16 rfd_cons_idx; / RFD Consumer Index.,
    pub the: *mut *mut u16 update; / Selene sets this bit every time it DMAs,
// CMB to host memory. Software should clear
// this bit when CMB info is processed.
    pub /: *mut *mut u16 tpd_cons_idx; / TPD Consumer Index.,
}

// RRD descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_return_desc {
    pub /: *mut *mut u8 num_buf; / Number of RFD buffers used by the received packet,
    pub resved: u8,
    pub /: *mut *mut u16 buf_indx; / RFD Index of the first buffer,
    pub valid: u32,
    pub rx_chksum: u16,
    pub pkt_size: u16,
    pub xsum_sz: },
    pub xsz: },
    pub /: *mut *mut u16 pkt_flg; / Packet flags,
    pub /: *mut *mut u16 err_flg; / Error flags,
    pub resved2: u16,
    pub /: *mut *mut u16 vlan_tag; / VLAN TAG,
}

pub const PACKET_FLAG_ETH_TYPE: c_uint = 0x0080;
pub const PACKET_FLAG_VLAN_INS: c_uint = 0x0100;
pub const PACKET_FLAG_ERR: c_uint = 0x0200;
pub const PACKET_FLAG_IPV4: c_uint = 0x0400;
pub const PACKET_FLAG_UDP: c_uint = 0x0800;
pub const PACKET_FLAG_TCP: c_uint = 0x1000;
pub const PACKET_FLAG_BCAST: c_uint = 0x2000;
pub const PACKET_FLAG_MCAST: c_uint = 0x4000;
pub const PACKET_FLAG_PAUSE: c_uint = 0x8000;
pub const ERR_FLAG_CRC: c_uint = 0x0001;
pub const ERR_FLAG_CODE: c_uint = 0x0002;
pub const ERR_FLAG_DRIBBLE: c_uint = 0x0004;
pub const ERR_FLAG_RUNT: c_uint = 0x0008;
pub const ERR_FLAG_OV: c_uint = 0x0010;
pub const ERR_FLAG_TRUNC: c_uint = 0x0020;
pub const ERR_FLAG_IP_CHKSUM: c_uint = 0x0040;
pub const ERR_FLAG_L4_CHKSUM: c_uint = 0x0080;
pub const ERR_FLAG_LEN: c_uint = 0x0100;
pub const ERR_FLAG_DES_ADDR: c_uint = 0x0200;
// RFD descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_free_desc {
    pub /: *mut *mut __le64 buffer_addr; / Address of the descriptor's data buffer,
    pub /: *mut *mut __le16 buf_len; / Size of the receive buffer in host memory,
    pub the: *mut *mut u16 coalese; / Update consumer index to host after,
// reception of this frame
// __packed is required
    pub __packed: },
//
// The L1 transmit packet descriptor is comprised of four 32-bit words.
//
// 31					0
// +---------------------------------------+
// |	Word 0: Buffer addr lo 		|
// +---------------------------------------+
// |	Word 1: Buffer addr hi		|
// +---------------------------------------+
// |		Word 2			|
// +---------------------------------------+
// |		Word 3			|
// +---------------------------------------+
//
// Words 0 and 1 combine to form a 64-bit buffer address.
//
// Word 2 is self explanatory in the #define block below.
//
// Word 3 has two forms, depending upon the state of bits 3 and 4.
// If bits 3 and 4 are both zero, then bits 14:31 are unused by the
// hardware.  Otherwise, if either bit 3 or 4 is set, the definition
// of bits 14:31 vary according to the following depiction.
//
// 0	End of packet			0	End of packet
// 1	Coalesce			1	Coalesce
// 2	Insert VLAN tag			2	Insert VLAN tag
// 3	Custom csum enable = 0		3	Custom csum enable = 1
// 4	Segment enable = 1		4	Segment enable = 0
// 5	Generate IP checksum		5	Generate IP checksum
// 6	Generate TCP checksum		6	Generate TCP checksum
// 7	Generate UDP checksum		7	Generate UDP checksum
// 8	VLAN tagged			8	VLAN tagged
// 9	Ethernet frame type		9	Ethernet frame type
// 10-+ 					10-+
// 11 |	IP hdr length (10:13)		11 |	IP hdr length (10:13)
// 12 |	(num 32-bit words)		12 |	(num 32-bit words)
// 13-+					13-+
// 14-+					14	Unused
// 15 |	TCP hdr length (14:17)		15	Unused
// 16 |	(num 32-bit words)		16-+
// 17-+					17 |
// 18	Header TPD flag			18 |
// 19-+					19 |	Payload offset
// 20 |					20 |	    (16:23)
// 21 |					21 |
// 22 |					22 |
// 23 |					23-+
// 24 |					24-+
// 25 |	MSS (19:31)			25 |
// 26 |					26 |
// 27 |					27 |	Custom csum offset
// 28 |					28 |	     (24:31)
// 29 |					29 |
// 30 |					30 |
// 31-+					31-+
//
// tpd word 2
pub const TPD_BUFLEN_MASK: c_uint = 0x3FFF;
pub const TPD_BUFLEN_SHIFT: c_int = 0;
pub const TPD_DMAINT_MASK: c_uint = 0x0001;
pub const TPD_DMAINT_SHIFT: c_int = 14;
pub const TPD_PKTNT_MASK: c_uint = 0x0001;
pub const TPD_PKTINT_SHIFT: c_int = 15;
pub const TPD_VLANTAG_MASK: c_uint = 0xFFFF;
pub const TPD_VLANTAG_SHIFT: c_int = 16;
// tpd word 3 bits 0:13
pub const TPD_EOP_MASK: c_uint = 0x0001;
pub const TPD_EOP_SHIFT: c_int = 0;
pub const TPD_COALESCE_MASK: c_uint = 0x0001;
pub const TPD_COALESCE_SHIFT: c_int = 1;
pub const TPD_INS_VL_TAG_MASK: c_uint = 0x0001;
pub const TPD_INS_VL_TAG_SHIFT: c_int = 2;
pub const TPD_CUST_CSUM_EN_MASK: c_uint = 0x0001;
pub const TPD_CUST_CSUM_EN_SHIFT: c_int = 3;
pub const TPD_SEGMENT_EN_MASK: c_uint = 0x0001;
pub const TPD_SEGMENT_EN_SHIFT: c_int = 4;
pub const TPD_IP_CSUM_MASK: c_uint = 0x0001;
pub const TPD_IP_CSUM_SHIFT: c_int = 5;
pub const TPD_TCP_CSUM_MASK: c_uint = 0x0001;
pub const TPD_TCP_CSUM_SHIFT: c_int = 6;
pub const TPD_UDP_CSUM_MASK: c_uint = 0x0001;
pub const TPD_UDP_CSUM_SHIFT: c_int = 7;
pub const TPD_VL_TAGGED_MASK: c_uint = 0x0001;
pub const TPD_VL_TAGGED_SHIFT: c_int = 8;
pub const TPD_ETHTYPE_MASK: c_uint = 0x0001;
pub const TPD_ETHTYPE_SHIFT: c_int = 9;
pub const TPD_IPHL_MASK: c_uint = 0x000F;
pub const TPD_IPHL_SHIFT: c_int = 10;
// tpd word 3 bits 14:31 if segment enabled
pub const TPD_TCPHDRLEN_MASK: c_uint = 0x000F;
pub const TPD_TCPHDRLEN_SHIFT: c_int = 14;
pub const TPD_HDRFLAG_MASK: c_uint = 0x0001;
pub const TPD_HDRFLAG_SHIFT: c_int = 18;
pub const TPD_MSS_MASK: c_uint = 0x1FFF;
pub const TPD_MSS_SHIFT: c_int = 19;
// tpd word 3 bits 16:31 if custom csum enabled
pub const TPD_PLOADOFFSET_MASK: c_uint = 0x00FF;
pub const TPD_PLOADOFFSET_SHIFT: c_int = 16;
pub const TPD_CCSUMOFFSET_MASK: c_uint = 0x00FF;
pub const TPD_CCSUMOFFSET_SHIFT: c_int = 24;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_packet_desc {
    pub buffer_addr: __le64,
    pub word2: __le32,
    pub word3: __le32,
}

// DMA Order Settings
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atl1_dma_order {
    atl1_dma_ord_in = 1,
    atl1_dma_ord_enh = 2,
    atl1_dma_ord_out = 4
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atl1_dma_rcb {
    atl1_rcb_64 = 0,
    atl1_rcb_128 = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atl1_dma_req_block {
    atl1_dma_req_128 = 0,
    atl1_dma_req_256 = 1,
    atl1_dma_req_512 = 2,
    atl1_dma_req_1024 = 3,
    atl1_dma_req_2048 = 4,
    atl1_dma_req_4096 = 5
}

pub const ATL1_MAX_INTR: c_int = 3;
pub const ATL1_MAX_TX_BUF_LEN: c_uint = 0x3000	/* 12288 bytes */;
pub const ATL1_DEFAULT_TPD: c_int = 256;
pub const ATL1_MAX_TPD: c_int = 1024;
pub const ATL1_MIN_TPD: c_int = 64;
pub const ATL1_DEFAULT_RFD: c_int = 512;
pub const ATL1_MIN_RFD: c_int = 128;
pub const ATL1_MAX_RFD: c_int = 2048;
pub const ATL1_REG_COUNT: c_int = 1538;

//
// atl1_ring_header represents a single, contiguous block of DMA space
// mapped for the three descriptor rings (tpd, rfd, rrd) and the two
// message blocks (cmb, smb) described below
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atl1_ring_header {
    pub /: *mut *mut *mut void desc; / virtual address,
    pub address*/: *mut *mut dma_addr_t dma; / physical,
    pub /: *mut *mut unsigned int size; / length in bytes,
}

//
// atl1_buffer is wrapper around a pointer to a socket buffer
// so a DMA handle can be stored along with the skb
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atl1_buffer {
    pub /: *mut *mut *mut sk_buff skb; / socket buffer,
    pub /: *mut *mut u16 length; / rx buffer length,
    pub /: *mut *mut u16 alloced; / 1 if skb allocated,
    pub dma: dma_addr_t,
}

// transmit packet descriptor (tpd) ring
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atl1_tpd_ring {
    pub /: *mut *mut *mut void desc; / descriptor ring virtual address,
    pub /: *mut *mut dma_addr_t dma; / descriptor ring physical address,
    pub /: *mut *mut u16 size; / descriptor ring length in bytes,
    pub /: *mut *mut u16 count; / number of descriptors in the ring,
    pub /: *mut *mut u16 hw_idx; / hardware index,
    pub next_to_clean: core::sync::atomic::AtomicI32,
    pub next_to_use: core::sync::atomic::AtomicI32,
    pub buffer_info: *mut atl1_buffer,
}

// receive free descriptor (rfd) ring
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atl1_rfd_ring {
    pub /: *mut *mut *mut void desc; / descriptor ring virtual address,
    pub /: *mut *mut dma_addr_t dma; / descriptor ring physical address,
    pub /: *mut *mut u16 size; / descriptor ring length in bytes,
    pub /: *mut *mut u16 count; / number of descriptors in the ring,
    pub next_to_use: core::sync::atomic::AtomicI32,
    pub next_to_clean: u16,
    pub buffer_info: *mut atl1_buffer,
}

// receive return descriptor (rrd) ring
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atl1_rrd_ring {
    pub /: *mut *mut *mut void desc; / descriptor ring virtual address,
    pub /: *mut *mut dma_addr_t dma; / descriptor ring physical address,
    pub /: *mut *mut unsigned int size; / descriptor ring length in bytes,
    pub /: *mut *mut u16 count; / number of descriptors in the ring,
    pub next_to_use: u16,
    pub next_to_clean: core::sync::atomic::AtomicI32,
}

// coalescing message block (cmb)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atl1_cmb {
    pub cmb: *mut coals_msg_block,
    pub dma: dma_addr_t,
}

// statistics message block (smb)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atl1_smb {
    pub smb: *mut stats_msg_block,
    pub dma: dma_addr_t,
}

// Statistics counters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atl1_sft_stats {
    pub rx_packets: u64,
    pub tx_packets: u64,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub multicast: u64,
    pub collisions: u64,
    pub rx_errors: u64,
    pub rx_length_errors: u64,
    pub rx_crc_errors: u64,
    pub rx_dropped: u64,
    pub rx_frame_errors: u64,
    pub rx_fifo_errors: u64,
    pub rx_missed_errors: u64,
    pub tx_errors: u64,
    pub tx_fifo_errors: u64,
    pub tx_aborted_errors: u64,
    pub tx_window_errors: u64,
    pub tx_carrier_errors: u64,
    pub /: *mut *mut u64 tx_pause; / TX pause frames,
    pub /: *mut *mut u64 excecol; / TX packets w/ excessive collisions,
    pub /: *mut *mut u64 deffer; / TX packets deferred,
    pub /: *mut *mut u64 scc; / packets TX after a single collision,
    pub /: *mut *mut u64 mcc; / packets TX after multiple collisions,
    pub /: *mut *mut u64 latecol; / TX packets w/ late collisions,
    pub underrun: *mut *mut u64 tx_underrun; / TX packets aborted due to TX FIFO,
// or TRD FIFO underrun
    pub /: *mut *mut u64 tx_trunc; / TX packets truncated due to size > MTU,
    pub /: *mut *mut u64 rx_pause; / num Pause packets received.,
    pub rx_rrd_ov: u64,
    pub rx_trunc: u64,
}

// hardware structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atl1_hw {
    pub hw_addr: *mut u8 __iomem,
    pub back: *mut atl1_adapter,
    pub dma_ord: atl1_dma_order,
    pub rcb_value: atl1_dma_rcb,
    pub dmar_block: atl1_dma_req_block,
    pub dmaw_block: atl1_dma_req_block,
    pub preamble_len: u8,
    pub max_retry: u8,
    pub flow: *mut *mut u8 jam_ipg; / IPG to start JAM for collision based,
// control in half-duplex mode. In units of
// 8-bit time
    pub gap.: *mut *mut u8 ipgt; / Desired back to back inter-packet,
// The default is 96-bit time
    pub between: *mut *mut u8 min_ifg; / Minimum number of IFG to enforce in,
// receive frames. Frame gap below such IFP
// is dropped
    pub /: *mut *mut u8 ipgr1; / 64bit Carrier-Sense window,
    pub /: *mut *mut u8 ipgr2; / 96-bit IPG window,
    pub cache-aligned: *mut *mut u8 tpd_burst; / Number of TPD to prefetch in,
// burst. Each TPD is 16 bytes long
    pub cache-aligned: *mut *mut u8 rfd_burst; / Number of RFD to prefetch in,
// burst. Each RFD is 12 bytes long
    pub rfd_fetch_gap: u8,
    pub retired: *mut *mut u8 rrd_burst; / Threshold number of RRDs that can be,
// in a burst. Each RRD is 16 bytes long
    pub tpd_fetch_th: u8,
    pub tpd_fetch_gap: u8,
    pub tx_jumbo_task_th: u16,
    pub cache-: *mut *mut u16 txf_burst; / Number of data bytes to read in a,
// aligned burst. Each SRAM entry is 8 bytes
    pub VLAN: *mut *mut u16 rx_jumbo_th; / Jumbo packet size for non-VLAN packet.,
// packets should add 4 bytes
    pub rx_jumbo_lkah: u16,
    pub after: *mut *mut u16 rrd_ret_timer; / RRD retirement timer. Decrement by 1,
// every 512ns passes.
    pub /: *mut *mut u16 lcol; / Collision Window,
    pub cmb_tpd: u16,
    pub cmb_rrd: u16,
    pub cmb_rx_timer: u16,
    pub cmb_tx_timer: u16,
    pub smb_timer: u32,
    pub media_type: u16,
    pub autoneg_advertised: u16,
    pub mii_autoneg_adv_reg: u16,
    pub mii_1000t_ctrl_reg: u16,
    pub max_frame_size: u32,
    pub min_frame_size: u32,
    pub dev_rev: u16,
// spi flash
    pub flash_vendor: u8,
    pub mac_addr: [u8; ETH_ALEN],
    pub perm_mac_addr: [u8; ETH_ALEN],
    pub phy_configured: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atl1_adapter {
    pub netdev: *mut net_device,
    pub pdev: *mut pci_dev,
    pub soft_stats: atl1_sft_stats,
    pub rx_buffer_len: u32,
    pub wol: u32,
    pub link_speed: u16,
    pub link_duplex: u16,
    pub lock: spinlock_t,
    pub napi: napi_struct,
    pub reset_dev_task: work_struct,
    pub link_chg_task: work_struct,
    pub phy_config_timer: timer_list,
    pub phy_timer_pending: bool,
// all descriptor rings' memory
    pub ring_header: atl1_ring_header,
// TX
    pub tpd_ring: atl1_tpd_ring,
    pub mb_lock: spinlock_t,
// RX
    pub rfd_ring: atl1_rfd_ring,
    pub rrd_ring: atl1_rrd_ring,
    pub hw_csum_err: u64,
    pub hw_csum_good: u64,
    pub msg_enable: u32,
    pub /: *mut *mut u16 imt; / interrupt moderator timer (2us resolution),
    pub /: *mut *mut u16 ict; / interrupt clear timer (2us resolution,
    pub /: *mut *mut mii_if_info mii; / MII interface info,
//
// Use this value to check is napi handler allowed to
// enable ints or not
//
    pub int_enabled: bool,
    pub /: *mut *mut u32 bd_number; / board number,
    pub pci_using_64: bool,
    pub hw: atl1_hw,
    pub smb: atl1_smb,
    pub cmb: atl1_cmb,
}
