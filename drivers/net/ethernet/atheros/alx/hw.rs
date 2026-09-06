//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/atheros/alx/hw.h
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


//
// Copyright (c) 2013 Johannes Berg <johannes@sipsolutions.net>
//
// This file is free software: you may copy, redistribute and/or modify it
// under the terms of the GNU General Public License as published by the
// Free Software Foundation, either version 2 of the License, or (at your
// option) any later version.
//
// This file is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
//
// This file incorporates work covered by the following copyright and
// permission notice:
//
// Copyright (c) 2012 Qualcomm Atheros, Inc.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

// Transmit Packet Descriptor, contains 4 32-bit words.
//
// 31               16               0
// +----------------+----------------+
// |    vlan-tag    |   buf length   |
// +----------------+----------------+
// |              Word 1             |
// +----------------+----------------+
// |      Word 2: buf addr lo        |
// +----------------+----------------+
// |      Word 3: buf addr hi        |
// +----------------+----------------+
//
// Word 2 and 3 combine to form a 64-bit buffer address
//
// Word 1 has three forms, depending on the state of bit 8/12/13:
// if bit8 =='1', the definition is just for custom checksum offload.
// if bit8 == '0' && bit12 == '1' && bit13 == '1', the *FIRST* descriptor
// for the skb is special for LSO V2, Word 2 become total skb length ,
// Word 3 is meaningless.
// other condition, the definition is for general skb or ip/tcp/udp
// checksum or LSO(TSO) offload.
//
// Here is the depiction:
//
// 0-+                                  0-+
// 1 |                                  1 |
// 2 |                                  2 |
// 3 |    Payload offset                3 |    L4 header offset
// 4 |        (7:0)                     4 |        (7:0)
// 5 |                                  5 |
// 6 |                                  6 |
// 7-+                                  7-+
// 8      Custom csum enable = 1        8      Custom csum enable = 0
// 9      General IPv4 checksum         9      General IPv4 checksum
// 10     General TCP checksum          10     General TCP checksum
// 11     General UDP checksum          11     General UDP checksum
// 12     Large Send Segment enable     12     Large Send Segment enable
// 13     Large Send Segment type       13     Large Send Segment type
// 14     VLAN tagged                   14     VLAN tagged
// 15     Insert VLAN tag               15     Insert VLAN tag
// 16     IPv4 packet                   16     IPv4 packet
// 17     Ethernet frame type           17     Ethernet frame type
// 18-+                                 18-+
// 19 |                                 19 |
// 20 |                                 20 |
// 21 |   Custom csum offset            21 |
// 22 |       (25:18)                   22 |
// 23 |                                 23 |   MSS (30:18)
// 24 |                                 24 |
// 25-+                                 25 |
// 26-+                                 26 |
// 27 |                                 27 |
// 28 |   Reserved                      28 |
// 29 |                                 29 |
// 30-+                                 30-+
// 31     End of packet                 31     End of packet
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct alx_txd {
    pub len: __le16,
    pub vlan_tag: __le16,
    pub word1: __le32,
    pub addr: __le64,
    pub pkt_len: __le32,
    pub resvd: __le32,
    pub l: },
    pub adrl: },
    pub __packed: },
// tpd word 1
pub const TPD_CXSUMSTART_MASK: c_uint = 0x00FF;
pub const TPD_CXSUMSTART_SHIFT: c_int = 0;
pub const TPD_L4HDROFFSET_MASK: c_uint = 0x00FF;
pub const TPD_L4HDROFFSET_SHIFT: c_int = 0;
pub const TPD_CXSUM_EN_MASK: c_uint = 0x0001;
pub const TPD_CXSUM_EN_SHIFT: c_int = 8;
pub const TPD_IP_XSUM_MASK: c_uint = 0x0001;
pub const TPD_IP_XSUM_SHIFT: c_int = 9;
pub const TPD_TCP_XSUM_MASK: c_uint = 0x0001;
pub const TPD_TCP_XSUM_SHIFT: c_int = 10;
pub const TPD_UDP_XSUM_MASK: c_uint = 0x0001;
pub const TPD_UDP_XSUM_SHIFT: c_int = 11;
pub const TPD_LSO_EN_MASK: c_uint = 0x0001;
pub const TPD_LSO_EN_SHIFT: c_int = 12;
pub const TPD_LSO_V2_MASK: c_uint = 0x0001;
pub const TPD_LSO_V2_SHIFT: c_int = 13;
pub const TPD_VLTAGGED_MASK: c_uint = 0x0001;
pub const TPD_VLTAGGED_SHIFT: c_int = 14;
pub const TPD_INS_VLTAG_MASK: c_uint = 0x0001;
pub const TPD_INS_VLTAG_SHIFT: c_int = 15;
pub const TPD_IPV4_MASK: c_uint = 0x0001;
pub const TPD_IPV4_SHIFT: c_int = 16;
pub const TPD_ETHTYPE_MASK: c_uint = 0x0001;
pub const TPD_ETHTYPE_SHIFT: c_int = 17;
pub const TPD_CXSUMOFFSET_MASK: c_uint = 0x00FF;
pub const TPD_CXSUMOFFSET_SHIFT: c_int = 18;
pub const TPD_MSS_MASK: c_uint = 0x1FFF;
pub const TPD_MSS_SHIFT: c_int = 18;
pub const TPD_EOP_MASK: c_uint = 0x0001;
pub const TPD_EOP_SHIFT: c_int = 31;

// Receive Free Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct alx_rfd {
    pub is: *mut *mut __le64 addr; / data buffer address, length,
// declared in register --- every
// buffer has the same size
//
    pub __packed: },
// Receive Return Descriptor, contains 4 32-bit words.
//
// 31               16               0
// +----------------+----------------+
// |              Word 0             |
// +----------------+----------------+
// |     Word 1: RSS Hash value      |
// +----------------+----------------+
// |              Word 2             |
// +----------------+----------------+
// |              Word 3             |
// +----------------+----------------+
//
// Word 0 depiction         &            Word 2 depiction:
//
// 0--+                                 0--+
// 1  |                                 1  |
// 2  |                                 2  |
// 3  |                                 3  |
// 4  |                                 4  |
// 5  |                                 5  |
// 6  |                                 6  |
// 7  |    IP payload checksum          7  |     VLAN tag
// 8  |         (15:0)                  8  |      (15:0)
// 9  |                                 9  |
// 10 |                                 10 |
// 11 |                                 11 |
// 12 |                                 12 |
// 13 |                                 13 |
// 14 |                                 14 |
// 15-+                                 15-+
// 16-+                                 16-+
// 17 |     Number of RFDs              17 |
// 18 |        (19:16)                  18 |
// 19-+                                 19 |     Protocol ID
// 20-+                                 20 |      (23:16)
// 21 |                                 21 |
// 22 |                                 22 |
// 23 |                                 23-+
// 24 |                                 24 |     Reserved
// 25 |     Start index of RFD-ring     25-+
// 26 |         (31:20)                 26 |     RSS Q-num (27:25)
// 27 |                                 27-+
// 28 |                                 28-+
// 29 |                                 29 |     RSS Hash algorithm
// 30 |                                 30 |      (31:28)
// 31-+                                 31-+
//
// Word 3 depiction:
//
// 0--+
// 1  |
// 2  |
// 3  |
// 4  |
// 5  |
// 6  |
// 7  |    Packet length (include FCS)
// 8  |         (13:0)
// 9  |
// 10 |
// 11 |
// 12 |
// 13-+
// 14      L4 Header checksum error
// 15      IPv4 checksum error
// 16      VLAN tagged
// 17-+
// 18 |    Protocol ID (19:17)
// 19-+
// 20      Receive error summary
// 21      FCS(CRC) error
// 22      Frame alignment error
// 23      Truncated packet
// 24      Runt packet
// 25      Incomplete packet due to insufficient rx-desc
// 26      Broadcast packet
// 27      Multicast packet
// 28      Ethernet type (EII or 802.3)
// 29      FIFO overflow
// 30      Length error (for 802.3, length field mismatch with actual len)
// 31      Updated, indicate to driver that this RRD is refreshed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct alx_rrd {
    pub word0: __le32,
    pub rss_hash: __le32,
    pub word2: __le32,
    pub word3: __le32,
    pub __packed: },
// rrd word 0
pub const RRD_XSUM_MASK: c_uint = 0xFFFF;
pub const RRD_XSUM_SHIFT: c_int = 0;
pub const RRD_NOR_MASK: c_uint = 0x000F;
pub const RRD_NOR_SHIFT: c_int = 16;
pub const RRD_SI_MASK: c_uint = 0x0FFF;
pub const RRD_SI_SHIFT: c_int = 20;
// rrd word 2
pub const RRD_VLTAG_MASK: c_uint = 0xFFFF;
pub const RRD_VLTAG_SHIFT: c_int = 0;
pub const RRD_PID_MASK: c_uint = 0x00FF;
pub const RRD_PID_SHIFT: c_int = 16;
// non-ip packet
pub const RRD_PID_NONIP: c_int = 0;
// ipv4(only)
pub const RRD_PID_IPV4: c_int = 1;
// tcp/ipv6
pub const RRD_PID_IPV6TCP: c_int = 2;
// tcp/ipv4
pub const RRD_PID_IPV4TCP: c_int = 3;
// udp/ipv6
pub const RRD_PID_IPV6UDP: c_int = 4;
// udp/ipv4
pub const RRD_PID_IPV4UDP: c_int = 5;
// ipv6(only)
pub const RRD_PID_IPV6: c_int = 6;
// LLDP packet
pub const RRD_PID_LLDP: c_int = 7;
// 1588 packet
pub const RRD_PID_1588: c_int = 8;
pub const RRD_RSSQ_MASK: c_uint = 0x0007;
pub const RRD_RSSQ_SHIFT: c_int = 25;
pub const RRD_RSSALG_MASK: c_uint = 0x000F;
pub const RRD_RSSALG_SHIFT: c_int = 28;
pub const RRD_RSSALG_TCPV6: c_uint = 0x1;
pub const RRD_RSSALG_IPV6: c_uint = 0x2;
pub const RRD_RSSALG_TCPV4: c_uint = 0x4;
pub const RRD_RSSALG_IPV4: c_uint = 0x8;
// rrd word 3
pub const RRD_PKTLEN_MASK: c_uint = 0x3FFF;
pub const RRD_PKTLEN_SHIFT: c_int = 0;
pub const RRD_ERR_L4_MASK: c_uint = 0x0001;
pub const RRD_ERR_L4_SHIFT: c_int = 14;
pub const RRD_ERR_IPV4_MASK: c_uint = 0x0001;
pub const RRD_ERR_IPV4_SHIFT: c_int = 15;
pub const RRD_VLTAGGED_MASK: c_uint = 0x0001;
pub const RRD_VLTAGGED_SHIFT: c_int = 16;
pub const RRD_OLD_PID_MASK: c_uint = 0x0007;
pub const RRD_OLD_PID_SHIFT: c_int = 17;
pub const RRD_ERR_RES_MASK: c_uint = 0x0001;
pub const RRD_ERR_RES_SHIFT: c_int = 20;
pub const RRD_ERR_FCS_MASK: c_uint = 0x0001;
pub const RRD_ERR_FCS_SHIFT: c_int = 21;
pub const RRD_ERR_FAE_MASK: c_uint = 0x0001;
pub const RRD_ERR_FAE_SHIFT: c_int = 22;
pub const RRD_ERR_TRUNC_MASK: c_uint = 0x0001;
pub const RRD_ERR_TRUNC_SHIFT: c_int = 23;
pub const RRD_ERR_RUNT_MASK: c_uint = 0x0001;
pub const RRD_ERR_RUNT_SHIFT: c_int = 24;
pub const RRD_ERR_ICMP_MASK: c_uint = 0x0001;
pub const RRD_ERR_ICMP_SHIFT: c_int = 25;
pub const RRD_BCAST_MASK: c_uint = 0x0001;
pub const RRD_BCAST_SHIFT: c_int = 26;
pub const RRD_MCAST_MASK: c_uint = 0x0001;
pub const RRD_MCAST_SHIFT: c_int = 27;
pub const RRD_ETHTYPE_MASK: c_uint = 0x0001;
pub const RRD_ETHTYPE_SHIFT: c_int = 28;
pub const RRD_ERR_FIFOV_MASK: c_uint = 0x0001;
pub const RRD_ERR_FIFOV_SHIFT: c_int = 29;
pub const RRD_ERR_LEN_MASK: c_uint = 0x0001;
pub const RRD_ERR_LEN_SHIFT: c_int = 30;
pub const RRD_UPDATED_MASK: c_uint = 0x0001;
pub const RRD_UPDATED_SHIFT: c_int = 31;
pub const ALX_MAX_SETUP_LNK_CYCLE: c_int = 50;
// for FlowControl
pub const ALX_FC_RX: c_uint = 0x01;
pub const ALX_FC_TX: c_uint = 0x02;
pub const ALX_FC_ANEG: c_uint = 0x04;
// for sleep control
pub const ALX_SLEEP_WOL_PHY: c_uint = 0x00000001;
pub const ALX_SLEEP_WOL_MAGIC: c_uint = 0x00000002;
pub const ALX_SLEEP_CIFS: c_uint = 0x00000004;

// for RSS hash type
pub const ALX_RSS_HASH_TYPE_IPV4: c_uint = 0x1;
pub const ALX_RSS_HASH_TYPE_IPV4_TCP: c_uint = 0x2;
pub const ALX_RSS_HASH_TYPE_IPV6: c_uint = 0x4;
pub const ALX_RSS_HASH_TYPE_IPV6_TCP: c_uint = 0x8;

pub const ALX_FRAME_PAD: c_int = 16;

pub const ALX_MAX_RX_QUEUES: c_int = 8;
pub const ALX_MAX_TX_QUEUES: c_int = 4;
pub const ALX_MAX_HANDLED_INTRS: c_int = 5;

// Statistics counters collected by the MAC
//
// The order of the fields must match the strings in alx_gstrings_stats
// All stats fields should be u64
// See ethtool.c
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct alx_hw_stats {
// rx
    pub /: *mut *mut u64 rx_ok; / good RX packets,
    pub /: *mut *mut u64 rx_bcast; / good RX broadcast packets,
    pub /: *mut *mut u64 rx_mcast; / good RX multicast packets,
    pub /: *mut *mut u64 rx_pause; / RX pause frames,
    pub /: *mut *mut u64 rx_ctrl; / RX control packets other than pause frames,
    pub /: *mut *mut u64 rx_fcs_err; / RX packets with bad FCS,
    pub /: *mut *mut u64 rx_len_err; / RX packets with length != actual size,
    pub /: *mut *mut u64 rx_byte_cnt; / good bytes received. FCS is NOT included,
    pub /: *mut *mut u64 rx_runt; / RX packets < 64 bytes with good FCS,
    pub /: *mut *mut u64 rx_frag; / RX packets < 64 bytes with bad FCS,
    pub /: *mut *mut u64 rx_sz_64B; / 64 byte RX packets,
    pub /: *mut *mut u64 rx_sz_127B; / 65-127 byte RX packets,
    pub /: *mut *mut u64 rx_sz_255B; / 128-255 byte RX packets,
    pub /: *mut *mut u64 rx_sz_511B; / 256-511 byte RX packets,
    pub /: *mut *mut u64 rx_sz_1023B; / 512-1023 byte RX packets,
    pub /: *mut *mut u64 rx_sz_1518B; / 1024-1518 byte RX packets,
    pub /: *mut *mut u64 rx_sz_max; / 1519 byte to MTU RX packets,
    pub /: *mut *mut u64 rx_ov_sz; / truncated RX packets, size > MTU,
    pub /: *mut *mut u64 rx_ov_rxf; / frames dropped due to RX FIFO overflow,
    pub /: *mut *mut u64 rx_ov_rrd; / frames dropped due to RRD overflow,
    pub /: *mut *mut u64 rx_align_err; / alignment errors,
    pub /: *mut *mut u64 rx_bc_byte_cnt; / RX broadcast bytes, excluding FCS,
    pub /: *mut *mut u64 rx_mc_byte_cnt; / RX multicast bytes, excluding FCS,
    pub /: *mut *mut u64 rx_err_addr; / packets dropped due to address filtering,
// tx
    pub /: *mut *mut u64 tx_ok; / good TX packets,
    pub /: *mut *mut u64 tx_bcast; / good TX broadcast packets,
    pub /: *mut *mut u64 tx_mcast; / good TX multicast packets,
    pub /: *mut *mut u64 tx_pause; / TX pause frames,
    pub /: *mut *mut u64 tx_exc_defer; / TX packets deferred excessively,
    pub /: *mut *mut u64 tx_ctrl; / TX control frames, excluding pause frames,
    pub /: *mut *mut u64 tx_defer; / TX packets deferred,
    pub /: *mut *mut u64 tx_byte_cnt; / bytes transmitted, FCS is NOT included,
    pub /: *mut *mut u64 tx_sz_64B; / 64 byte TX packets,
    pub /: *mut *mut u64 tx_sz_127B; / 65-127 byte TX packets,
    pub /: *mut *mut u64 tx_sz_255B; / 128-255 byte TX packets,
    pub /: *mut *mut u64 tx_sz_511B; / 256-511 byte TX packets,
    pub /: *mut *mut u64 tx_sz_1023B; / 512-1023 byte TX packets,
    pub /: *mut *mut u64 tx_sz_1518B; / 1024-1518 byte TX packets,
    pub /: *mut *mut u64 tx_sz_max; / 1519 byte to MTU TX packets,
    pub /: *mut *mut u64 tx_single_col; / packets TX after a single collision,
    pub /: *mut *mut u64 tx_multi_col; / packets TX after multiple collisions,
    pub /: *mut *mut u64 tx_late_col; / TX packets with late collisions,
    pub /: *mut *mut u64 tx_abort_col; / TX packets aborted w/excessive collisions,
    pub underrun: *mut *mut u64 tx_underrun; / TX packets aborted due to TX FIFO,
// or TRD FIFO underrun
//
    pub frame: *mut *mut u64 tx_trd_eop; / reads beyond the EOP into the next,
// when TRD was not written timely
//
    pub /: *mut *mut u64 tx_len_err; / TX packets where length != actual size,
    pub /: *mut *mut u64 tx_trunc; / TX packets truncated due to size > MTU,
    pub /: *mut *mut u64 tx_bc_byte_cnt; / broadcast bytes transmitted, excluding FCS,
    pub /: *mut *mut u64 tx_mc_byte_cnt; / multicast bytes transmitted, excluding FCS,
    pub update: u64,
}

// maximum interrupt vectors for msix
pub const ALX_MAX_MSIX_INTRS: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct alx_hw {
    pub pdev: *mut pci_dev,
    pub hw_addr: *mut u8 __iomem,
// current & permanent mac addr
    pub mac_addr: [u8; ETH_ALEN],
    pub perm_addr: [u8; ETH_ALEN],
    pub mtu: u16,
    pub imt: u16,
    pub dma_chnl: u8,
    pub max_dma_chnl: u8,
// tpd threshold to trig INT
    pub ith_tpd: u32,
    pub rx_ctrl: u32,
    pub mc_hash: [u32; 2],
    pub smb_timer: u32,
// SPEED_* + DUPLEX_*, SPEED_UNKNOWN if link is down
    pub link_speed: c_int,
    pub duplex: u8,
// auto-neg advertisement or force mode config
    pub flowctrl: u8,
    pub adv_cfg: u32,
    pub mdio_lock: spinlock_t,
    pub mdio: mdio_if_info,
    pub phy_id: [u16; 2],
// PHY link patch flag
    pub lnk_patch: bool,
// cumulated stats from the hardware (registers are cleared on read)
    pub stats: alx_hw_stats,
}

extern "C" {
    pub fn readw(reg: hw->hw_addr +) -> return;
}
extern "C" {
    pub fn readl(reg: hw->hw_addr +) -> return;
}
extern "C" {
    pub fn alx_get_perm_macaddr(hw: *mut alx_hw, addr: *mut u8) -> c_int;
}
extern "C" {
    pub fn alx_reset_phy(hw: *mut alx_hw);
}
extern "C" {
    pub fn alx_reset_pcie(hw: *mut alx_hw);
}
extern "C" {
    pub fn alx_enable_aspm(hw: *mut alx_hw, l0s_en: bool, l1_en: bool);
}
extern "C" {
    pub fn alx_setup_speed_duplex(hw: *mut alx_hw, ethadv: u32, flowctrl: u8) -> c_int;
}
extern "C" {
    pub fn alx_post_phy_link(hw: *mut alx_hw);
}
extern "C" {
    pub fn alx_read_phy_reg(hw: *mut alx_hw, reg: u16, phy_data: *mut u16) -> c_int;
}
extern "C" {
    pub fn alx_write_phy_reg(hw: *mut alx_hw, reg: u16, phy_data: u16) -> c_int;
}
extern "C" {
    pub fn alx_read_phy_ext(hw: *mut alx_hw, dev: u8, reg: u16, pdata: *mut u16) -> c_int;
}
extern "C" {
    pub fn alx_write_phy_ext(hw: *mut alx_hw, dev: u8, reg: u16, data: u16) -> c_int;
}
extern "C" {
    pub fn alx_read_phy_link(hw: *mut alx_hw) -> c_int;
}
extern "C" {
    pub fn alx_clear_phy_intr(hw: *mut alx_hw) -> c_int;
}
extern "C" {
    pub fn alx_cfg_mac_flowcontrol(hw: *mut alx_hw, fc: u8);
}
extern "C" {
    pub fn alx_start_mac(hw: *mut alx_hw);
}
extern "C" {
    pub fn alx_reset_mac(hw: *mut alx_hw) -> c_int;
}
extern "C" {
    pub fn alx_set_macaddr(hw: *mut alx_hw, addr: *const u8);
}
extern "C" {
    pub fn alx_phy_configured(hw: *mut alx_hw) -> bool;
}
extern "C" {
    pub fn alx_configure_basic(hw: *mut alx_hw);
}
extern "C" {
    pub fn alx_mask_msix(hw: *mut alx_hw, index: c_int, mask: bool);
}
extern "C" {
    pub fn alx_disable_rss(hw: *mut alx_hw);
}
extern "C" {
    pub fn alx_get_phy_info(hw: *mut alx_hw) -> bool;
}
extern "C" {
    pub fn alx_update_hw_stats(hw: *mut alx_hw);
}
