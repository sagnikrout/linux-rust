//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/virtio_net.h
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


// This header is BSD licensed so anyone can use the definitions to implement
// compatible drivers/servers.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// 3. Neither the name of IBM nor the names of its contributors
// may be used to endorse or promote products derived from this software
// without specific prior written permission.
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS ``AS IS'' AND
// ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED.  IN NO EVENT SHALL IBM OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
// OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
// LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
// OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
// SUCH DAMAGE.

// The feature bitmap for virtio net

// network

// Steering

// with the same MAC.
//

// GSO-over-UDP-tunnel packets
//

// GSO-over-UDP-tunnel
// packets with partial csum
// for the outer header
//

// GSO-over-UDP-tunnel packets
//

// GSO-over-UDP-tunnel
// packets with partial csum
// for the outer header
//
// Offloads bits corresponding to VIRTIO_NET_F_HOST_UDP_TUNNEL_GSO{,_CSUM}
// features
//
pub const VIRTIO_NET_F_GUEST_UDP_TUNNEL_GSO_MAPPED: c_int = 46;
pub const VIRTIO_NET_F_GUEST_UDP_TUNNEL_GSO_CSUM_MAPPED: c_int = 47;

// supported/enabled hash types

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_net_config {
// The config defining mac address (if VIRTIO_NET_F_MAC)
    pub mac: [__u8; ETH_ALEN],
// See VIRTIO_NET_F_STATUS and VIRTIO_NET_S_* above
    pub status: __virtio16,
// Maximum number of each of transmit and receive queues;
// see VIRTIO_NET_F_MQ and VIRTIO_NET_CTRL_MQ.
// Legal values are between 1 and 0x8000
//
    pub max_virtqueue_pairs: __virtio16,
// Default maximum transmit unit advice
    pub mtu: __virtio16,
//
// speed, in units of 1Mb. All values 0 to INT_MAX are legal.
// Any other value stands for unknown.
//
    pub speed: __le32,
//
// 0x00 - half duplex
// 0x01 - full duplex
// Any other value stands for unknown.
//
    pub duplex: __u8,
// maximum size of RSS key
    pub rss_max_key_size: __u8,
// maximum number of indirection table entries
    pub rss_max_indirection_table_length: __le16,
// bitmask of supported VIRTIO_NET_RSS_HASH_ types
    pub supported_hash_types: __le32,
    pub __attribute__((packed)): },
//
// This header comes first in the scatter-gather list.  If you don't
// specify GSO or CSUM features, you can simply ignore the header.
//
// This is bitwise-equivalent to the legacy struct virtio_net_hdr_mrg_rxbuf,
// only flattened.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_net_hdr_v1 {

    pub flags: __u8,

pub const VIRTIO_NET_HDR_GSO_UDP_TUNNEL_IPV4: c_uint = 0x20 /* UDPv4 tunnel present */;
pub const VIRTIO_NET_HDR_GSO_UDP_TUNNEL_IPV6: c_uint = 0x40 /* UDPv6 tunnel present */;

pub const VIRTIO_NET_HDR_GSO_ECN: c_uint = 0x80	/* TCP has ECN set */;
    pub gso_type: __u8,
    pub /: *mut *mut __virtio16 hdr_len; / Ethernet + IP + tcp/udp hdrs,
    pub /: *mut *mut __virtio16 gso_size; / Bytes to append to hdr_len per frame,
    pub csum_start: __virtio16,
    pub csum_offset: __virtio16,
}

// Checksum calculation
// Position to start checksumming from
// Offset after that to place checksum
// Receive Segment Coalescing
// Number of coalesced segments
// Number of duplicated acks
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_net_hdr_v1_hash {
    pub hdr: virtio_net_hdr_v1,
    pub hash_value_lo: __le16,
    pub hash_value_hi: __le16,
pub const VIRTIO_NET_HASH_REPORT_NONE: c_int = 0;
pub const VIRTIO_NET_HASH_REPORT_IPv4: c_int = 1;
pub const VIRTIO_NET_HASH_REPORT_TCPv4: c_int = 2;
pub const VIRTIO_NET_HASH_REPORT_UDPv4: c_int = 3;
pub const VIRTIO_NET_HASH_REPORT_IPv6: c_int = 4;
pub const VIRTIO_NET_HASH_REPORT_TCPv6: c_int = 5;
pub const VIRTIO_NET_HASH_REPORT_UDPv6: c_int = 6;
pub const VIRTIO_NET_HASH_REPORT_IPv6_EX: c_int = 7;
pub const VIRTIO_NET_HASH_REPORT_TCPv6_EX: c_int = 8;
pub const VIRTIO_NET_HASH_REPORT_UDPv6_EX: c_int = 9;
    pub hash_report: __le16,
    pub padding: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_net_hdr_v1_hash_tunnel {
    pub hash_hdr: virtio_net_hdr_v1_hash,
    pub outer_th_offset: __le16,
    pub inner_nh_offset: __le16,
}

// This header comes first in the scatter-gather list.
// For legacy virtio, if VIRTIO_F_ANY_LAYOUT is not negotiated, it must
// be the first element of the scatter-gather list.  If you don't
// specify GSO or CSUM features, you can simply ignore the header.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_net_hdr {
// See VIRTIO_NET_HDR_F_*
    pub flags: __u8,
// See VIRTIO_NET_HDR_GSO_*
    pub gso_type: __u8,
    pub /: *mut *mut __virtio16 hdr_len; / Ethernet + IP + tcp/udp hdrs,
    pub /: *mut *mut __virtio16 gso_size; / Bytes to append to hdr_len per frame,
    pub /: *mut *mut __virtio16 csum_start; / Position to start checksumming from,
    pub /: *mut *mut __virtio16 csum_offset; / Offset after that to place checksum,
}

// This is the version of the header to use when the MRG_RXBUF
// feature has been negotiated.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_net_hdr_mrg_rxbuf {
    pub hdr: virtio_net_hdr,
    pub /: *mut *mut __virtio16 num_buffers; / Number of merged rx buffers,
}

//
// Control virtqueue data structures
//
// The control virtqueue expects a header in the first sg entry
// and an ack/status response in the last entry.  Data for the
// command goes in between.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_net_ctrl_hdr {
    pub class: __u8,
    pub cmd: __u8,
    pub __attribute__((packed)): },
pub type virtio_net_ctrl_ack = __u8;
pub const VIRTIO_NET_OK: c_int = 0;
pub const VIRTIO_NET_ERR: c_int = 1;
//
// Control the RX mode, ie. promisucous, allmulti, etc...
// All commands require an "out" sg entry containing a 1 byte
// state value, zero = disable, non-zero = enable.  Commands
// 0 and 1 are supported with the VIRTIO_NET_F_CTRL_RX feature.
// Commands 2-5 are added with VIRTIO_NET_F_CTRL_RX_EXTRA.
//
pub const VIRTIO_NET_CTRL_RX: c_int = 0;
pub const VIRTIO_NET_CTRL_RX_PROMISC: c_int = 0;
pub const VIRTIO_NET_CTRL_RX_ALLMULTI: c_int = 1;
pub const VIRTIO_NET_CTRL_RX_ALLUNI: c_int = 2;
pub const VIRTIO_NET_CTRL_RX_NOMULTI: c_int = 3;
pub const VIRTIO_NET_CTRL_RX_NOUNI: c_int = 4;
pub const VIRTIO_NET_CTRL_RX_NOBCAST: c_int = 5;
//
// Control the MAC
//
// The MAC filter table is managed by the hypervisor, the guest should
// assume the size is infinite.  Filtering should be considered
// non-perfect, ie. based on hypervisor resources, the guest may
// received packets from sources not specified in the filter list.
//
// In addition to the class/cmd header, the TABLE_SET command requires
// two out scatterlists.  Each contains a 4 byte count of entries followed
// by a concatenated byte stream of the ETH_ALEN MAC addresses.  The
// first sg list contains unicast addresses, the second is for multicast.
// This functionality is present if the VIRTIO_NET_F_CTRL_RX feature
// is available.
//
// The ADDR_SET command requests one out scatterlist, it contains a
// 6 bytes MAC address. This functionality is present if the
// VIRTIO_NET_F_CTRL_MAC_ADDR feature is available.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_net_ctrl_mac {
    pub entries: __virtio32,
    pub macs: [__u8; ][ETH_ALEN],
    pub __attribute__((packed)): },
pub const VIRTIO_NET_CTRL_MAC: c_int = 1;
pub const VIRTIO_NET_CTRL_MAC_TABLE_SET: c_int = 0;
pub const VIRTIO_NET_CTRL_MAC_ADDR_SET: c_int = 1;
//
// Control VLAN filtering
//
// The VLAN filter table is controlled via a simple ADD/DEL interface.
// VLAN IDs not added may be filterd by the hypervisor.  Del is the
// opposite of add.  Both commands expect an out entry containing a 2
// byte VLAN ID.  VLAN filterting is available with the
// VIRTIO_NET_F_CTRL_VLAN feature bit.
//
pub const VIRTIO_NET_CTRL_VLAN: c_int = 2;
pub const VIRTIO_NET_CTRL_VLAN_ADD: c_int = 0;
pub const VIRTIO_NET_CTRL_VLAN_DEL: c_int = 1;
//
// Control link announce acknowledgement
//
// The command VIRTIO_NET_CTRL_ANNOUNCE_ACK is used to indicate that
// driver has recevied the notification; device would clear the
// VIRTIO_NET_S_ANNOUNCE bit in the status field after it receives
// this command.
//
pub const VIRTIO_NET_CTRL_ANNOUNCE: c_int = 3;
pub const VIRTIO_NET_CTRL_ANNOUNCE_ACK: c_int = 0;
//
// Control Receive Flow Steering
//
pub const VIRTIO_NET_CTRL_MQ: c_int = 4;
//
// The command VIRTIO_NET_CTRL_MQ_VQ_PAIRS_SET
// enables Receive Flow Steering, specifying the number of the transmit and
// receive queues that will be used. After the command is consumed and acked by
// the device, the device will not steer new packets on receive virtqueues
// other than specified nor read from transmit virtqueues other than specified.
// Accordingly, driver should not transmit new packets  on virtqueues other than
// specified.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_net_ctrl_mq {
    pub virtqueue_pairs: __virtio16,
}

pub const VIRTIO_NET_CTRL_MQ_VQ_PAIRS_SET: c_int = 0;
pub const VIRTIO_NET_CTRL_MQ_VQ_PAIRS_MIN: c_int = 1;
pub const VIRTIO_NET_CTRL_MQ_VQ_PAIRS_MAX: c_uint = 0x8000;
//
// The command VIRTIO_NET_CTRL_MQ_RSS_CONFIG has the same effect as
// VIRTIO_NET_CTRL_MQ_VQ_PAIRS_SET does and additionally configures
// the receive steering to use a hash calculated for incoming packet
// to decide on receive virtqueue to place the packet. The command
// also provides parameters to calculate a hash and receive virtqueue.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_net_rss_config {
    pub hash_types: __le32,
    pub indirection_table_mask: __le16,
    pub unclassified_queue: __le16,
    pub /]: *mut *mut __le16 indirection_table[1/ + indirection_table_mask,
    pub max_tx_vq: __le16,
    pub hash_key_length: __u8,
    pub /]: *mut *mut __u8 hash_key_data[/ hash_key_length,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_net_rss_config_hdr {
    pub hash_types: __le32,
    pub indirection_table_mask: __le16,
    pub unclassified_queue: __le16,
    pub /]: *mut *mut __le16 indirection_table[/ 1 + indirection_table_mask,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_net_rss_config_trailer {
    pub max_tx_vq: __le16,
    pub hash_key_length: __u8,
    pub /]: *mut *mut __u8 hash_key_data[/ hash_key_length,
}

pub const VIRTIO_NET_CTRL_MQ_RSS_CONFIG: c_int = 1;
//
// The command VIRTIO_NET_CTRL_MQ_HASH_CONFIG requests the device
// to include in the virtio header of the packet the value of the
// calculated hash and the report type of hash. It also provides
// parameters for hash calculation. The command requires feature
// VIRTIO_NET_F_HASH_REPORT to be negotiated to extend the
// layout of virtio header as defined in virtio_net_hdr_v1_hash.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_net_hash_config {
    pub hash_types: __le32,
// for compatibility with virtio_net_rss_config
    pub reserved: [__le16; 4],
    pub hash_key_length: __u8,
    pub /]: *mut *mut __u8 hash_key_data[/ hash_key_length,
}

pub const VIRTIO_NET_CTRL_MQ_HASH_CONFIG: c_int = 2;
//
// Control network offloads
//
// Reconfigures the network offloads that Guest can handle.
//
// Available with the VIRTIO_NET_F_CTRL_GUEST_OFFLOADS feature bit.
//
// Command data format matches the feature bit mask exactly.
//
// See VIRTIO_NET_F_GUEST_* for the list of offloads
// that can be enabled/disabled.
//
pub const VIRTIO_NET_CTRL_GUEST_OFFLOADS: c_int = 5;
pub const VIRTIO_NET_CTRL_GUEST_OFFLOADS_SET: c_int = 0;
//
// Control notifications coalescing.
//
// Request the device to change the notifications coalescing parameters.
//
// Available with the VIRTIO_NET_F_NOTF_COAL feature bit.
//
pub const VIRTIO_NET_CTRL_NOTF_COAL: c_int = 6;
//
// Set the tx-usecs/tx-max-packets parameters.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_net_ctrl_coal_tx {
// Maximum number of packets to send before a TX notification
    pub tx_max_packets: __le32,
// Maximum number of usecs to delay a TX notification
    pub tx_usecs: __le32,
}

pub const VIRTIO_NET_CTRL_NOTF_COAL_TX_SET: c_int = 0;
//
// Set the rx-usecs/rx-max-packets parameters.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_net_ctrl_coal_rx {
// Maximum number of packets to receive before a RX notification
    pub rx_max_packets: __le32,
// Maximum number of usecs to delay a RX notification
    pub rx_usecs: __le32,
}

pub const VIRTIO_NET_CTRL_NOTF_COAL_RX_SET: c_int = 1;
pub const VIRTIO_NET_CTRL_NOTF_COAL_VQ_SET: c_int = 2;
pub const VIRTIO_NET_CTRL_NOTF_COAL_VQ_GET: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_net_ctrl_coal {
    pub max_packets: __le32,
    pub max_usecs: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_net_ctrl_coal_vq {
    pub vqn: __le16,
    pub reserved: __le16,
    pub coal: virtio_net_ctrl_coal,
}

//
// Device Statistics
//
pub const VIRTIO_NET_CTRL_STATS: c_int = 8;
pub const VIRTIO_NET_CTRL_STATS_QUERY: c_int = 0;
pub const VIRTIO_NET_CTRL_STATS_GET: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_net_stats_capabilities {
    pub supported_stats_types: [__le64; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_net_ctrl_queue_stats {
    pub vq_index: __le16,
    pub reserved: [__le16; 3],
    pub types_bitmap: [__le64; 1],
    pub stats: [}; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_net_stats_reply_hdr {
pub const VIRTIO_NET_STATS_TYPE_REPLY_CVQ: c_int = 32;
pub const VIRTIO_NET_STATS_TYPE_REPLY_RX_BASIC: c_int = 0;
pub const VIRTIO_NET_STATS_TYPE_REPLY_RX_CSUM: c_int = 1;
pub const VIRTIO_NET_STATS_TYPE_REPLY_RX_GSO: c_int = 2;
pub const VIRTIO_NET_STATS_TYPE_REPLY_RX_SPEED: c_int = 3;
pub const VIRTIO_NET_STATS_TYPE_REPLY_TX_BASIC: c_int = 16;
pub const VIRTIO_NET_STATS_TYPE_REPLY_TX_CSUM: c_int = 17;
pub const VIRTIO_NET_STATS_TYPE_REPLY_TX_GSO: c_int = 18;
pub const VIRTIO_NET_STATS_TYPE_REPLY_TX_SPEED: c_int = 19;
    pub type: __u8,
    pub reserved: __u8,
    pub vq_index: __le16,
    pub reserved1: __le16,
    pub size: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_net_stats_cvq {
    pub hdr: virtio_net_stats_reply_hdr,
    pub command_num: __le64,
    pub ok_num: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_net_stats_rx_basic {
    pub hdr: virtio_net_stats_reply_hdr,
    pub rx_notifications: __le64,
    pub rx_packets: __le64,
    pub rx_bytes: __le64,
    pub rx_interrupts: __le64,
    pub rx_drops: __le64,
    pub rx_drop_overruns: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_net_stats_tx_basic {
    pub hdr: virtio_net_stats_reply_hdr,
    pub tx_notifications: __le64,
    pub tx_packets: __le64,
    pub tx_bytes: __le64,
    pub tx_interrupts: __le64,
    pub tx_drops: __le64,
    pub tx_drop_malformed: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_net_stats_rx_csum {
    pub hdr: virtio_net_stats_reply_hdr,
    pub rx_csum_valid: __le64,
    pub rx_needs_csum: __le64,
    pub rx_csum_none: __le64,
    pub rx_csum_bad: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_net_stats_tx_csum {
    pub hdr: virtio_net_stats_reply_hdr,
    pub tx_csum_none: __le64,
    pub tx_needs_csum: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_net_stats_rx_gso {
    pub hdr: virtio_net_stats_reply_hdr,
    pub rx_gso_packets: __le64,
    pub rx_gso_bytes: __le64,
    pub rx_gso_packets_coalesced: __le64,
    pub rx_gso_bytes_coalesced: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_net_stats_tx_gso {
    pub hdr: virtio_net_stats_reply_hdr,
    pub tx_gso_packets: __le64,
    pub tx_gso_bytes: __le64,
    pub tx_gso_segments: __le64,
    pub tx_gso_segments_bytes: __le64,
    pub tx_gso_packets_noseg: __le64,
    pub tx_gso_bytes_noseg: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_net_stats_rx_speed {
    pub hdr: virtio_net_stats_reply_hdr,
// rx_{packets,bytes}_allowance_exceeded are too long. So rename to
// short name.
//
    pub rx_ratelimit_packets: __le64,
    pub rx_ratelimit_bytes: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_net_stats_tx_speed {
    pub hdr: virtio_net_stats_reply_hdr,
// tx_{packets,bytes}_allowance_exceeded are too long. So rename to
// short name.
//
    pub tx_ratelimit_packets: __le64,
    pub tx_ratelimit_bytes: __le64,
}
