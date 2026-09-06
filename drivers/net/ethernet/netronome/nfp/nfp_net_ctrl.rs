//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/netronome/nfp/nfp_net_ctrl.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2015-2018 Netronome Systems, Inc.
// nfp_net_ctrl.h
// Netronome network device driver: Control BAR layout
// Authors: Jakub Kicinski <jakub.kicinski@netronome.com>
// Jason McMullan <jason.mcmullan@netronome.com>
// Rolf Neugebauer <rolf.neugebauer@netronome.com>
// Brad Petrus <brad.petrus@netronome.com>
//

// 64-bit per app capabilities

// Configuration BAR size.
//
// The configuration BAR is 8K in size, but due to
// THB-350, 32k needs to be reserved.
//

// Offset in Freelist buffer where packet starts on RX
pub const NFP_NET_RX_OFFSET: c_int = 32;
// LSO parameters
// %NFP_NET_LSO_MAX_HDR_SZ:	Maximum header size supported for LSO frames
// %NFP_NET_LSO_MAX_SEGS:	Maximum number of segments LSO frame can produce
//
pub const NFP_NET_LSO_MAX_HDR_SZ: c_int = 255;
pub const NFP_NET_LSO_MAX_SEGS: c_int = 64;
// working with metadata vlan api (NFD version >= 2.0)

// Prepend field types
pub const NFP_NET_META_FIELD_SIZE: c_int = 4;

pub const NFP_NET_META_MARK: c_int = 2;

pub const NFP_NET_META_PORTID: c_int = 5;

pub const NFP_NET_META_CONN_HANDLE: c_int = 7;

// Prepend field sizes
pub const NFP_NET_META_VLAN_SIZE: c_int = 4;
pub const NFP_NET_META_PORTID_SIZE: c_int = 4;
pub const NFP_NET_META_CONN_HANDLE_SIZE: c_int = 8;
pub const NFP_NET_META_IPSEC_SIZE: c_int = 4;
pub const NFP_NET_META_IPSEC_FIELD_SIZE: c_int = 12;
// Hash type pre-pended when a RSS hash was computed
pub const NFP_NET_RSS_NONE: c_int = 0;
pub const NFP_NET_RSS_IPV4: c_int = 1;
pub const NFP_NET_RSS_IPV6: c_int = 2;
pub const NFP_NET_RSS_IPV6_EX: c_int = 3;
pub const NFP_NET_RSS_IPV4_TCP: c_int = 4;
pub const NFP_NET_RSS_IPV6_TCP: c_int = 5;
pub const NFP_NET_RSS_IPV6_EX_TCP: c_int = 6;
pub const NFP_NET_RSS_IPV4_UDP: c_int = 7;
pub const NFP_NET_RSS_IPV6_UDP: c_int = 8;
pub const NFP_NET_RSS_IPV6_EX_UDP: c_int = 9;
// Ring counts
// %NFP_NET_TXR_MAX:	     Maximum number of TX rings
// %NFP_NET_RXR_MAX:	     Maximum number of RX rings
//
pub const NFP_NET_TXR_MAX: c_int = 64;
pub const NFP_NET_RXR_MAX: c_int = 64;
// Read/Write config words (0x0000 - 0x002c)
// %NFP_NET_CFG_CTRL:	     Global control
// %NFP_NET_CFG_UPDATE:      Indicate which fields are updated
// %NFP_NET_CFG_TXRS_ENABLE: Bitmask of enabled TX rings
// %NFP_NET_CFG_RXRS_ENABLE: Bitmask of enabled RX rings
// %NFP_NET_CFG_MTU:	     Set MTU size
// %NFP_NET_CFG_FLBUFSZ:     Set freelist buffer size (must be larger than MTU)
// %NFP_NET_CFG_EXN:	     MSI-X table entry for exceptions
// %NFP_NET_CFG_LSC:	     MSI-X table entry for link state changes
// %NFP_NET_CFG_MACADDR:     MAC address
//
// TODO:
// - define Error details in UPDATE
//
pub const NFP_NET_CFG_CTRL: c_uint = 0x0000;

pub const NFP_NET_CFG_UPDATE: c_uint = 0x0004;

pub const NFP_NET_CFG_TXRS_ENABLE: c_uint = 0x0008;
pub const NFP_NET_CFG_RXRS_ENABLE: c_uint = 0x0010;
pub const NFP_NET_CFG_MTU: c_uint = 0x0018;
pub const NFP_NET_CFG_FLBUFSZ: c_uint = 0x001c;
pub const NFP_NET_CFG_EXN: c_uint = 0x001f;
pub const NFP_NET_CFG_LSC: c_uint = 0x0020;
pub const NFP_NET_CFG_MACADDR: c_uint = 0x0024;
// Read-only words (0x0030 - 0x0050):
// %NFP_NET_CFG_VERSION:     Firmware version number
// %NFP_NET_CFG_STS:	     Status
// %NFP_NET_CFG_CAP:	     Capabilities (same bits as %NFP_NET_CFG_CTRL)
// %NFP_NET_CFG_MAX_TXRINGS: Maximum number of TX rings
// %NFP_NET_CFG_MAX_RXRINGS: Maximum number of RX rings
// %NFP_NET_CFG_MAX_MTU:     Maximum support MTU
// %NFP_NET_CFG_START_TXQ:   Start Queue Control Queue to use for TX (PF only)
// %NFP_NET_CFG_START_RXQ:   Start Queue Control Queue to use for RX (PF only)
//
// TODO:
// - define more STS bits
//
pub const NFP_NET_CFG_VERSION: c_uint = 0x0030;

pub const NFP_NET_CFG_VERSION_DP_NFD3: c_int = 0;
pub const NFP_NET_CFG_VERSION_DP_NFDK: c_int = 1;
pub const NFP_NET_CFG_VERSION_DP_MASK: c_int = 1;

pub const NFP_NET_CFG_VERSION_CLASS_GENERIC: c_int = 0;

pub const NFP_NET_CFG_STS: c_uint = 0x0034;

// Link rate
pub const NFP_NET_CFG_STS_LINK_RATE_SHIFT: c_int = 1;
pub const NFP_NET_CFG_STS_LINK_RATE_MASK: c_uint = 0xF;

pub const NFP_NET_CFG_STS_LINK_RATE_UNSUPPORTED: c_int = 0;
pub const NFP_NET_CFG_STS_LINK_RATE_UNKNOWN: c_int = 1;
pub const NFP_NET_CFG_STS_LINK_RATE_1G: c_int = 2;
pub const NFP_NET_CFG_STS_LINK_RATE_10G: c_int = 3;
pub const NFP_NET_CFG_STS_LINK_RATE_25G: c_int = 4;
pub const NFP_NET_CFG_STS_LINK_RATE_40G: c_int = 5;
pub const NFP_NET_CFG_STS_LINK_RATE_50G: c_int = 6;
pub const NFP_NET_CFG_STS_LINK_RATE_100G: c_int = 7;
// NSP Link rate is a 16-bit word. It's determined by NSP and
// written to CFG BAR by NFP driver.
//
pub const NFP_NET_CFG_STS_NSP_LINK_RATE: c_uint = 0x0036;
pub const NFP_NET_CFG_CAP: c_uint = 0x0038;
pub const NFP_NET_CFG_MAX_TXRINGS: c_uint = 0x003c;
pub const NFP_NET_CFG_MAX_RXRINGS: c_uint = 0x0040;
pub const NFP_NET_CFG_MAX_MTU: c_uint = 0x0044;
// Next two words are being used by VFs for solving THB350 issue
pub const NFP_NET_CFG_START_TXQ: c_uint = 0x0048;
pub const NFP_NET_CFG_START_RXQ: c_uint = 0x004c;
// Prepend configuration
//
pub const NFP_NET_CFG_RX_OFFSET: c_uint = 0x0050;

// RSS capabilities
// %NFP_NET_CFG_RSS_CAP_HFUNC:	supported hash functions (same bits as
// %NFP_NET_CFG_RSS_HFUNC)
//
pub const NFP_NET_CFG_RSS_CAP: c_uint = 0x0054;
pub const NFP_NET_CFG_RSS_CAP_HFUNC: c_uint = 0xff000000;
// TLV area start
// %NFP_NET_CFG_TLV_BASE:	start anchor of the TLV area
//
pub const NFP_NET_CFG_TLV_BASE: c_uint = 0x0058;
// VXLAN/UDP encap configuration
// %NFP_NET_CFG_VXLAN_PORT:	Base address of table of tunnels' UDP dst ports
// %NFP_NET_CFG_VXLAN_SZ:	Size of the UDP port table in bytes
//
pub const NFP_NET_CFG_VXLAN_PORT: c_uint = 0x0060;
pub const NFP_NET_CFG_VXLAN_SZ: c_uint = 0x0008;
// BPF section
// %NFP_NET_CFG_BPF_ABI:	BPF ABI version
// %NFP_NET_CFG_BPF_CAP:	BPF capabilities
// %NFP_NET_CFG_BPF_MAX_LEN:	Maximum size of JITed BPF code in bytes
// %NFP_NET_CFG_BPF_START:	Offset at which BPF will be loaded
// %NFP_NET_CFG_BPF_DONE:	Offset to jump to on exit
// %NFP_NET_CFG_BPF_STACK_SZ:	Total size of stack area in 64B chunks
// %NFP_NET_CFG_BPF_INL_MTU:	Packet data split offset in 64B chunks
// %NFP_NET_CFG_BPF_SIZE:	Size of the JITed BPF code in instructions
// %NFP_NET_CFG_BPF_ADDR:	DMA address of the buffer with JITed BPF code
//
pub const NFP_NET_CFG_BPF_ABI: c_uint = 0x0080;
pub const NFP_NET_CFG_BPF_CAP: c_uint = 0x0081;

pub const NFP_NET_CFG_BPF_MAX_LEN: c_uint = 0x0082;
pub const NFP_NET_CFG_BPF_START: c_uint = 0x0084;
pub const NFP_NET_CFG_BPF_DONE: c_uint = 0x0086;
pub const NFP_NET_CFG_BPF_STACK_SZ: c_uint = 0x0088;
pub const NFP_NET_CFG_BPF_INL_MTU: c_uint = 0x0089;
pub const NFP_NET_CFG_BPF_SIZE: c_uint = 0x008e;
pub const NFP_NET_CFG_BPF_ADDR: c_uint = 0x0090;

// 3 words reserved for extended ctrl words (0x0098 - 0x00a4)
// 3 words reserved for extended cap words (0x00a4 - 0x00b0)
// Currently only one word is used, can be extended in future.
//
pub const NFP_NET_CFG_CTRL_WORD1: c_uint = 0x0098;

pub const NFP_NET_CFG_CAP_WORD1: c_uint = 0x00a4;
// 16B reserved for future use (0x00b0 - 0x00c0)
pub const NFP_NET_CFG_RESERVED: c_uint = 0x00b0;
pub const NFP_NET_CFG_RESERVED_SZ: c_uint = 0x0010;
// RSS configuration (0x0100 - 0x01ac):
// Used only when NFP_NET_CFG_CTRL_RSS is enabled
// %NFP_NET_CFG_RSS_CFG:     RSS configuration word
// %NFP_NET_CFG_RSS_KEY:     RSS "secret" key
// %NFP_NET_CFG_RSS_ITBL:    RSS indirection table
//
pub const NFP_NET_CFG_RSS_BASE: c_uint = 0x0100;

pub const NFP_NET_CFG_RSS_HFUNC: c_uint = 0xff000000;

pub const NFP_NET_CFG_RSS_HFUNCS: c_int = 3;

pub const NFP_NET_CFG_RSS_KEY_SZ: c_uint = 0x28;

pub const NFP_NET_CFG_RSS_ITBL_SZ: c_uint = 0x80;
// TX ring configuration (0x200 - 0x800)
// %NFP_NET_CFG_TXR_BASE:    Base offset for TX ring configuration
// %NFP_NET_CFG_TXR_ADDR:    Per TX ring DMA address (8B entries)
// %NFP_NET_CFG_TXR_WB_ADDR: Per TX ring write back DMA address (8B entries)
// %NFP_NET_CFG_TXR_SZ:      Per TX ring ring size (1B entries)
// %NFP_NET_CFG_TXR_VEC:     Per TX ring MSI-X table entry (1B entries)
// %NFP_NET_CFG_TXR_PRIO:    Per TX ring priority (1B entries)
// %NFP_NET_CFG_TXR_IRQ_MOD: Per TX ring interrupt moderation packet
//
pub const NFP_NET_CFG_TXR_BASE: c_uint = 0x0200;

// RX ring configuration (0x0800 - 0x0c00)
// %NFP_NET_CFG_RXR_BASE:    Base offset for RX ring configuration
// %NFP_NET_CFG_RXR_ADDR:    Per RX ring DMA address (8B entries)
// %NFP_NET_CFG_RXR_SZ:      Per RX ring ring size (1B entries)
// %NFP_NET_CFG_RXR_VEC:     Per RX ring MSI-X table entry (1B entries)
// %NFP_NET_CFG_RXR_PRIO:    Per RX ring priority (1B entries)
// %NFP_NET_CFG_RXR_IRQ_MOD: Per RX ring interrupt moderation (4B entries)
//
pub const NFP_NET_CFG_RXR_BASE: c_uint = 0x0800;

// Interrupt Control/Cause registers (0x0c00 - 0x0d00)
// These registers are only used when MSI-X auto-masking is not
// enabled (%NFP_NET_CFG_CTRL_MSIXAUTO not set).  The array is index
// by MSI-X entry and are 1B in size.  If an entry is zero, the
// corresponding entry is enabled.  If the FW generates an interrupt,
// it writes a cause into the corresponding field.  This also masks
// the MSI-X entry and the host driver must clear the register to
// re-enable the interrupt.
//
pub const NFP_NET_CFG_ICR_BASE: c_uint = 0x0c00;

pub const NFP_NET_CFG_ICR_UNMASKED: c_uint = 0x0;
pub const NFP_NET_CFG_ICR_RXTX: c_uint = 0x1;
pub const NFP_NET_CFG_ICR_LSC: c_uint = 0x2;
// General device stats (0x0d00 - 0x0d90)
// all counters are 64bit.
//
pub const NFP_NET_CFG_STATS_BASE: c_uint = 0x0d00;

// Per ring stats (0x1000 - 0x1800)
// options, 64bit per entry
// %NFP_NET_CFG_TXR_STATS:   TX ring statistics (Packet and Byte count)
// %NFP_NET_CFG_RXR_STATS:   RX ring statistics (Packet and Byte count)
//
pub const NFP_NET_CFG_TXR_STATS_BASE: c_uint = 0x1000;

pub const NFP_NET_CFG_RXR_STATS_BASE: c_uint = 0x1400;

// General use mailbox area (0x1800 - 0x19ff)
// 4B used for update command and 4B return code
// followed by a max of 504B of variable length value
//
pub const NFP_NET_CFG_MBOX_BASE: c_uint = 0x1800;
pub const NFP_NET_CFG_MBOX_VAL_MAX_SZ: c_uint = 0x1F8;
pub const NFP_NET_CFG_MBOX_SIMPLE_CMD: c_uint = 0x0;
pub const NFP_NET_CFG_MBOX_SIMPLE_RET: c_uint = 0x4;
pub const NFP_NET_CFG_MBOX_SIMPLE_VAL: c_uint = 0x8;
pub const NFP_NET_CFG_MBOX_CMD_CTAG_FILTER_ADD: c_int = 1;
pub const NFP_NET_CFG_MBOX_CMD_CTAG_FILTER_KILL: c_int = 2;
pub const NFP_NET_CFG_MBOX_CMD_IPSEC: c_int = 3;
pub const NFP_NET_CFG_MBOX_CMD_PCI_DSCP_PRIOMAP_SET: c_int = 5;
pub const NFP_NET_CFG_MBOX_CMD_TLV_CMSG: c_int = 6;
pub const NFP_NET_CFG_MBOX_CMD_DCB_UPDATE: c_int = 7;
pub const NFP_NET_CFG_MBOX_CMD_MULTICAST_ADD: c_int = 8;
pub const NFP_NET_CFG_MBOX_CMD_MULTICAST_DEL: c_int = 9;
pub const NFP_NET_CFG_MBOX_CMD_FLOW_STEER: c_int = 10;
// VLAN filtering using general use mailbox
// %NFP_NET_CFG_VLAN_FILTER:		Base address of VLAN filter mailbox
// %NFP_NET_CFG_VLAN_FILTER_VID:	VLAN ID to filter
// %NFP_NET_CFG_VLAN_FILTER_PROTO:	VLAN proto to filter
// %NFP_NET_CFG_VXLAN_SZ:		Size of the VLAN filter mailbox in bytes
//

pub const NFP_NET_CFG_VLAN_FILTER_SZ: c_uint = 0x0004;
// Multicast filtering using general use mailbox
// %NFP_NET_CFG_MULTICAST:		Base address of Multicast filter mailbox
// %NFP_NET_CFG_MULTICAST_MAC_HI:	High 32-bits of Multicast MAC address
// %NFP_NET_CFG_MULTICAST_MAC_LO:	Low 16-bits of Multicast MAC address
// %NFP_NET_CFG_MULTICAST_SZ:		Size of the Multicast filter mailbox in bytes
//

pub const NFP_NET_CFG_MULTICAST_SZ: c_uint = 0x0006;
// Max size of FS rules in bytes
pub const NFP_NET_CFG_FS_SZ: c_uint = 0x0054;
// Sub commands for FS
// TLV capabilities
// %NFP_NET_CFG_TLV_TYPE:	Offset of type within the TLV
// %NFP_NET_CFG_TLV_TYPE_REQUIRED: Driver must be able to parse the TLV
// %NFP_NET_CFG_TLV_LENGTH:	Offset of length within the TLV
// %NFP_NET_CFG_TLV_LENGTH_INC: TLV length increments
// %NFP_NET_CFG_TLV_VALUE:	Offset of value with the TLV
//
// List of simple TLV structures, first one starts at %NFP_NET_CFG_TLV_BASE.
// Last structure must be of type %NFP_NET_CFG_TLV_TYPE_END.  Presence of TLVs
// is indicated by %NFP_NET_CFG_TLV_BASE being non-zero.  TLV structures may
// fill the entire remainder of the BAR or be shorter.  FW must make sure TLVs
// don't conflict with other features which allocate space beyond
// %NFP_NET_CFG_TLV_BASE.  %NFP_NET_CFG_TLV_TYPE_RESERVED should be used to wrap
// space used by such features.
// Note that the 4 byte TLV header is not counted in %NFP_NET_CFG_TLV_LENGTH.
//
pub const NFP_NET_CFG_TLV_TYPE: c_uint = 0x00;
pub const NFP_NET_CFG_TLV_TYPE_REQUIRED: c_uint = 0x8000;
pub const NFP_NET_CFG_TLV_LENGTH: c_uint = 0x02;
pub const NFP_NET_CFG_TLV_LENGTH_INC: c_int = 4;
pub const NFP_NET_CFG_TLV_VALUE: c_uint = 0x04;
pub const NFP_NET_CFG_TLV_HEADER_REQUIRED: c_uint = 0x80000000;
pub const NFP_NET_CFG_TLV_HEADER_TYPE: c_uint = 0x7fff0000;
pub const NFP_NET_CFG_TLV_HEADER_LENGTH: c_uint = 0x0000ffff;
// Capability TLV types
//
// %NFP_NET_CFG_TLV_TYPE_UNKNOWN:
// Special TLV type to catch bugs, should never be encountered.  Drivers should
// treat encountering this type as error and refuse to probe.
//
// %NFP_NET_CFG_TLV_TYPE_RESERVED:
// Reserved space, may contain legacy fixed-offset fields, or be used for
// padding.  The use of this type should be otherwise avoided.
//
// %NFP_NET_CFG_TLV_TYPE_END:
// Empty, end of TLV list.  Must be the last TLV.  Drivers will stop processing
// further TLVs when encountered.
//
// %NFP_NET_CFG_TLV_TYPE_ME_FREQ:
// Single word, ME frequency in MHz as used in calculation for
// %NFP_NET_CFG_RXR_IRQ_MOD and %NFP_NET_CFG_TXR_IRQ_MOD.
//
// %NFP_NET_CFG_TLV_TYPE_MBOX:
// Variable, mailbox area.  Overwrites the default location which is
// %NFP_NET_CFG_MBOX_BASE and length %NFP_NET_CFG_MBOX_VAL_MAX_SZ.
//
// %NFP_NET_CFG_TLV_TYPE_EXPERIMENTAL0:
// %NFP_NET_CFG_TLV_TYPE_EXPERIMENTAL1:
// Variable, experimental IDs.  IDs designated for internal development and
// experiments before a stable TLV ID has been allocated to a feature.  Should
// never be present in production firmware.
//
// %NFP_NET_CFG_TLV_TYPE_REPR_CAP:
// Single word, equivalent of %NFP_NET_CFG_CAP for representors, features which
// can be used on representors.
//
// %NFP_NET_CFG_TLV_TYPE_MBOX_CMSG_TYPES:
// Variable, bitmap of control message types supported by the mailbox handler.
// Bit 0 corresponds to message type 0, bit 1 to 1, etc.  Control messages are
// encapsulated into simple TLVs, with an end TLV and written to the Mailbox.
//
// %NFP_NET_CFG_TLV_TYPE_CRYPTO_OPS:
// 8 words, bitmaps of supported and enabled crypto operations.
// First 16B (4 words) contains a bitmap of supported crypto operations,
// and next 16B contain the enabled operations.
// This capability is made obsolete by ones with better sync methods.
//
// %NFP_NET_CFG_TLV_TYPE_VNIC_STATS:
// Variable, per-vNIC statistics, data should be 8B aligned (FW should insert
// zero-length RESERVED TLV to pad).
// TLV data has two sections.  First is an array of statistics' IDs (2B each).
// Second 8B statistics themselves.  Statistics are 8B aligned, meaning there
// may be a padding between sections.
// Number of statistics can be determined as floor(tlv.length / (2 + 8)).
// This TLV overwrites %NFP_NET_CFG_STATS_* values (statistics in this TLV
// duplicate the old ones, so driver should be careful not to unnecessarily
// render both).
//
// %NFP_NET_CFG_TLV_TYPE_CRYPTO_OPS_RX_SCAN:
// Same as %NFP_NET_CFG_TLV_TYPE_CRYPTO_OPS, but crypto TLS does stream scan
// RX sync, rather than kernel-assisted sync.
//
pub const NFP_NET_CFG_TLV_TYPE_UNKNOWN: c_int = 0;
pub const NFP_NET_CFG_TLV_TYPE_RESERVED: c_int = 1;
pub const NFP_NET_CFG_TLV_TYPE_END: c_int = 2;
pub const NFP_NET_CFG_TLV_TYPE_ME_FREQ: c_int = 3;
pub const NFP_NET_CFG_TLV_TYPE_MBOX: c_int = 4;
pub const NFP_NET_CFG_TLV_TYPE_EXPERIMENTAL0: c_int = 5;
pub const NFP_NET_CFG_TLV_TYPE_EXPERIMENTAL1: c_int = 6;
pub const NFP_NET_CFG_TLV_TYPE_REPR_CAP: c_int = 7;
pub const NFP_NET_CFG_TLV_TYPE_MBOX_CMSG_TYPES: c_int = 10;

pub const NFP_NET_CFG_TLV_TYPE_VNIC_STATS: c_int = 12;
pub const NFP_NET_CFG_TLV_TYPE_CRYPTO_OPS_RX_SCAN: c_int = 13;
// struct nfp_net_tlv_caps - parsed control BAR TLV capabilities
// @me_freq_mhz:	ME clock_freq (MHz)
// @mbox_off:		vNIC mailbox area offset
// @mbox_len:		vNIC mailbox area length
// @repr_cap:		capabilities for representors
// @mbox_cmsg_types:	cmsgs which can be passed through the mailbox
// @crypto_ops:		supported crypto operations
// @crypto_enable_off:	offset of crypto ops enable region
// @vnic_stats_off:	offset of vNIC stats area
// @vnic_stats_cnt:	number of vNIC stats
// @tls_resync_ss:	TLS resync will be performed via stream scan
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_net_tlv_caps {
    pub me_freq_mhz: u32,
    pub mbox_off: c_uint,
    pub mbox_len: c_uint,
    pub repr_cap: u32,
    pub mbox_cmsg_types: u32,
    pub crypto_ops: u32,
    pub crypto_enable_off: c_uint,
    pub vnic_stats_off: c_uint,
    pub vnic_stats_cnt: c_uint,
    pub tls_resync_ss:1: c_uint,
}
