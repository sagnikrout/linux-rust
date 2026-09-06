//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qlogic/netxen/netxen_nic.h
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
// Copyright (C) 2003 - 2009 NetXen, Inc.
// Copyright (C) 2009 - QLogic Corporation.
// All rights reserved.
//

pub const _NETXEN_NIC_LINUX_MAJOR: c_int = 4;
pub const _NETXEN_NIC_LINUX_MINOR: c_int = 0;
pub const _NETXEN_NIC_LINUX_SUBVERSION: c_int = 82;

// version in image has weird encoding:
// 7:0  - major
// 15:8  - minor
// 31:16 - build (little endian)
//

// NETXEN_FLASH_SECTOR_SIZE)

pub const NETXEN_RCV_PRODUCER_OFFSET: c_int = 0;
pub const NETXEN_RCV_PEG_DB_ID: c_int = 2;
pub const NETXEN_HOST_DUMMY_DMA_SIZE: c_int = 1024;
pub const FLASH_SUCCESS: c_int = 0;

//
// normalize a 64MB crb address to 32MB PCI window
// To use NETXEN_CRB_NORMALIZE, window _must_ be set to 1
//

pub const NX_P2_C0: c_uint = 0x24;
pub const NX_P2_C1: c_uint = 0x25;
pub const NX_P3_A0: c_uint = 0x30;
pub const NX_P3_A2: c_uint = 0x30;
pub const NX_P3_B0: c_uint = 0x40;
pub const NX_P3_B1: c_uint = 0x41;
pub const NX_P3_B2: c_uint = 0x42;
pub const NX_P3P_A0: c_uint = 0x50;

pub const FIRST_PAGE_GROUP_START: c_int = 0;
pub const FIRST_PAGE_GROUP_END: c_uint = 0x100000;
pub const SECOND_PAGE_GROUP_START: c_uint = 0x6000000;
pub const SECOND_PAGE_GROUP_END: c_uint = 0x68BC000;
pub const THIRD_PAGE_GROUP_START: c_uint = 0x70E4000;
pub const THIRD_PAGE_GROUP_END: c_uint = 0x8000000;

pub const NX_ETHERMTU: c_int = 1500;

pub const NX_P2_RX_BUF_MAX_LEN: c_int = 1760;

pub const NX_CT_DEFAULT_RX_BUF_LEN: c_int = 2048;
pub const NX_LRO_BUFFER_EXTRA: c_int = 2048;

//
// Maximum number of ring contexts
//
pub const MAX_RING_CTX: c_int = 1;
// Opcodes to be used with the commands
pub const TX_ETHER_PKT: c_uint = 0x01;
pub const TX_TCP_PKT: c_uint = 0x02;
pub const TX_UDP_PKT: c_uint = 0x03;
pub const TX_IP_PKT: c_uint = 0x04;
pub const TX_TCP_LSO: c_uint = 0x05;
pub const TX_TCP_LSO6: c_uint = 0x06;
pub const TX_IPSEC: c_uint = 0x07;
pub const TX_IPSEC_CMD: c_uint = 0x0a;
pub const TX_TCPV6_PKT: c_uint = 0x0b;
pub const TX_UDPV6_PKT: c_uint = 0x0c;
// The following opcodes are for internal consumption.
pub const NETXEN_CONTROL_OP: c_uint = 0x10;
pub const PEGNET_REQUEST: c_uint = 0x11;
pub const MAX_NUM_CARDS: c_int = 4;
pub const NETXEN_MAX_FRAGS_PER_TX: c_int = 14;
pub const MAX_TSO_HEADER_DESC: c_int = 2;
pub const MGMT_CMD_DESC_RESV: c_int = 4;

pub const NX_MAX_TX_TIMEOUTS: c_int = 2;
//
// Following are the states of the Phantom. Phantom will set them and
// Host will read to check if the fields are correct.
//
pub const PHAN_INITIALIZE_START: c_uint = 0xff00;
pub const PHAN_INITIALIZE_FAILED: c_uint = 0xffff;
pub const PHAN_INITIALIZE_COMPLETE: c_uint = 0xff01;
// Host writes the following to notify that it has done the init-handshake
pub const PHAN_INITIALIZE_ACK: c_uint = 0xf00f;
pub const NUM_RCV_DESC_RINGS: c_int = 3;
pub const NUM_STS_DESC_RINGS: c_int = 4;
pub const RCV_RING_NORMAL: c_int = 0;
pub const RCV_RING_JUMBO: c_int = 1;
pub const RCV_RING_LRO: c_int = 2;
pub const MIN_CMD_DESCRIPTORS: c_int = 64;
pub const MIN_RCV_DESCRIPTORS: c_int = 64;
pub const MIN_JUMBO_DESCRIPTORS: c_int = 32;
pub const MAX_CMD_DESCRIPTORS: c_int = 1024;
pub const MAX_RCV_DESCRIPTORS_1G: c_int = 4096;
pub const MAX_RCV_DESCRIPTORS_10G: c_int = 8192;
pub const MAX_JUMBO_RCV_DESCRIPTORS_1G: c_int = 512;
pub const MAX_JUMBO_RCV_DESCRIPTORS_10G: c_int = 1024;
pub const MAX_LRO_RCV_DESCRIPTORS: c_int = 8;
pub const DEFAULT_RCV_DESCRIPTORS_1G: c_int = 2048;
pub const DEFAULT_RCV_DESCRIPTORS_10G: c_int = 4096;
pub const NETXEN_CTX_SIGNATURE: c_uint = 0xdee0;
pub const NETXEN_CTX_SIGNATURE_V2: c_uint = 0x0002dee0;
pub const NETXEN_CTX_RESET: c_uint = 0xbad0;
pub const NETXEN_CTX_D3_RESET: c_uint = 0xacc0;

pub const PHAN_PEG_RCV_INITIALIZED: c_uint = 0xff01;
pub const PHAN_PEG_RCV_START_INITIALIZE: c_uint = 0xff00;

pub const MPORT_SINGLE_FUNCTION_MODE: c_uint = 0x1111;
pub const MPORT_MULTI_FUNCTION_MODE: c_uint = 0x2222;
pub const NX_MAX_PCI_FUNC: c_int = 8;
//
// NetXen host-peg signal message structure
//
// Bit 0-1		: peg_id => 0x2 for tx and 01 for rx
// Bit 2		: priv_id => must be 1
// Bit 3-17	: count => for doorbell
// Bit 18-27	: ctx_id => Context id
// Bit 28-31	: opcode
//
pub type netxen_ctx_msg = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netxen_rcv_ring {
    pub addr: __le64,
    pub size: __le32,
    pub rsrvd: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netxen_sts_ring {
    pub addr: __le64,
    pub size: __le32,
    pub msi_index: __le16,
    pub rsvd: __le16,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netxen_ring_ctx {
// one command ring
    pub cmd_consumer_offset: __le64,
    pub cmd_ring_addr: __le64,
    pub cmd_ring_size: __le32,
    pub rsrvd: __le32,
// three receive rings
    pub rcv_rings: [netxen_rcv_ring; NUM_RCV_DESC_RINGS],
    pub sts_ring_addr: __le64,
    pub sts_ring_size: __le32,
    pub ctx_id: __le32,
    pub rsrvd_2: [__le64; 3],
    pub sts_ring_count: __le32,
    pub rsrvd_3: __le32,
    pub sts_rings: [netxen_sts_ring; NUM_STS_DESC_RINGS],
// C attribute field omitted
//
// Following data structures describe the descriptors that will be used.
// Added fileds of tcpHdrSize and ipHdrSize, The driver needs to do it only when
// we are doing LSO (above the 1500 size packet) only.
//
// The size of reference handle been changed to 16 bits to pass the MSS fields
// for the LSO packet
//
pub const FLAGS_CHECKSUM_ENABLED: c_uint = 0x01;
pub const FLAGS_LSO_ENABLED: c_uint = 0x02;
pub const FLAGS_IPSEC_SA_ADD: c_uint = 0x04;
pub const FLAGS_IPSEC_SA_DELETE: c_uint = 0x08;
pub const FLAGS_VLAN_TAGGED: c_uint = 0x10;
pub const FLAGS_VLAN_OOB: c_uint = 0x40;

    pub cpu_to_le16(v): (cmd_desc)->vlan_TCI =,

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_desc_type0 {
    pub /: *mut *mut u8 tcp_hdr_offset; / For LSO only,
    pub /: *mut *mut u8 ip_hdr_offset; / For LSO only,
    pub /: *mut *mut __le16 flags_opcode; / 15:13 unused, 12:7 opcode, 6:0 flags,
    pub /: *mut *mut __le32 nfrags__length; / 31:8 total len, 7:0 frag count,
    pub addr_buffer2: __le64,
    pub reference_handle: __le16,
    pub mss: __le16,
    pub /: *mut *mut u8 port_ctxid; / 7:4 ctxid 3:0 port,
    pub /: *mut *mut u8 total_hdr_length; / LSO only : MAC+IP+TCP Hdr size,
    pub /: *mut *mut __le16 conn_id; / IPSec offoad only,
    pub addr_buffer3: __le64,
    pub addr_buffer1: __le64,
    pub buffer_length: [__le16; 4],
    pub addr_buffer4: __le64,
    pub reserved2: __le32,
    pub reserved: __le16,
    pub vlan_TCI: __le16,
// C attribute field omitted
// Note: sizeof(rcv_desc) should always be a multiple of 2
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcv_desc {
    pub reference_handle: __le16,
    pub reserved: __le16,
    pub /: *mut *mut __le32 buffer_length; / allocated buffer length (usually 2K),
    pub addr_buffer: __le64,
}

// opcode field in status_desc
pub const NETXEN_NIC_SYN_OFFLOAD: c_uint = 0x03;
pub const NETXEN_NIC_RXPKT_DESC: c_uint = 0x04;
pub const NETXEN_OLD_RXPKT_DESC: c_uint = 0x3f;
pub const NETXEN_NIC_RESPONSE_DESC: c_uint = 0x05;
pub const NETXEN_NIC_LRO_DESC: c_uint = 0x12;
// for status field in status_desc

// owner bits of status_desc

// Status descriptor:
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct status_desc {
    pub status_desc_data: [__le64; 2],
// C attribute field omitted
// UNIFIED ROMIMAGE
pub const NX_UNI_DIR_SECT_PRODUCT_TBL: c_uint = 0x0;
pub const NX_UNI_DIR_SECT_BOOTLD: c_uint = 0x6;
pub const NX_UNI_DIR_SECT_FW: c_uint = 0x7;
// Offsets
pub const NX_UNI_CHIP_REV_OFF: c_int = 10;
pub const NX_UNI_FLAGS_OFF: c_int = 11;
pub const NX_UNI_BIOS_VERSION_OFF: c_int = 12;
pub const NX_UNI_BOOTLD_IDX_OFF: c_int = 27;
pub const NX_UNI_FIRMWARE_IDX_OFF: c_int = 29;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uni_table_desc {
    pub findex: u32,
    pub num_entries: u32,
    pub entry_size: u32,
    pub reserved: [u32; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uni_data_desc {
    pub findex: u32,
    pub size: u32,
    pub reserved: [u32; 5],
}

// UNIFIED ROMIMAGE
// The version of the main data structure
pub const NETXEN_BDINFO_VERSION: c_int = 1;
// Magic number to let user know flash is programmed
pub const NETXEN_BDINFO_MAGIC: c_uint = 0x12345678;
// Max number of Gig ports on a Phantom board
pub const NETXEN_MAX_PORTS: c_int = 4;
pub const NETXEN_BRDTYPE_P1_BD: c_uint = 0x0000;
pub const NETXEN_BRDTYPE_P1_SB: c_uint = 0x0001;
pub const NETXEN_BRDTYPE_P1_SMAX: c_uint = 0x0002;
pub const NETXEN_BRDTYPE_P1_SOCK: c_uint = 0x0003;
pub const NETXEN_BRDTYPE_P2_SOCK_31: c_uint = 0x0008;
pub const NETXEN_BRDTYPE_P2_SOCK_35: c_uint = 0x0009;
pub const NETXEN_BRDTYPE_P2_SB35_4G: c_uint = 0x000a;
pub const NETXEN_BRDTYPE_P2_SB31_10G: c_uint = 0x000b;
pub const NETXEN_BRDTYPE_P2_SB31_2G: c_uint = 0x000c;
pub const NETXEN_BRDTYPE_P2_SB31_10G_IMEZ: c_uint = 0x000d;
pub const NETXEN_BRDTYPE_P2_SB31_10G_HMEZ: c_uint = 0x000e;
pub const NETXEN_BRDTYPE_P2_SB31_10G_CX4: c_uint = 0x000f;
pub const NETXEN_BRDTYPE_P3_REF_QG: c_uint = 0x0021;
pub const NETXEN_BRDTYPE_P3_HMEZ: c_uint = 0x0022;
pub const NETXEN_BRDTYPE_P3_10G_CX4_LP: c_uint = 0x0023;
pub const NETXEN_BRDTYPE_P3_4_GB: c_uint = 0x0024;
pub const NETXEN_BRDTYPE_P3_IMEZ: c_uint = 0x0025;
pub const NETXEN_BRDTYPE_P3_10G_SFP_PLUS: c_uint = 0x0026;
pub const NETXEN_BRDTYPE_P3_10000_BASE_T: c_uint = 0x0027;
pub const NETXEN_BRDTYPE_P3_XG_LOM: c_uint = 0x0028;
pub const NETXEN_BRDTYPE_P3_4_GB_MM: c_uint = 0x0029;
pub const NETXEN_BRDTYPE_P3_10G_SFP_CT: c_uint = 0x002a;
pub const NETXEN_BRDTYPE_P3_10G_SFP_QT: c_uint = 0x002b;
pub const NETXEN_BRDTYPE_P3_10G_CX4: c_uint = 0x0031;
pub const NETXEN_BRDTYPE_P3_10G_XFP: c_uint = 0x0032;
pub const NETXEN_BRDTYPE_P3_10G_TP: c_uint = 0x0080;
// Flash memory map

pub const NETXEN_BRDCFG_START: c_uint = 0x4000	/* board config */;
pub const NETXEN_INITCODE_START: c_uint = 0x6000	/* pegtune code */;
pub const NETXEN_BOOTLD_START: c_uint = 0x10000	/* bootld */;
pub const NETXEN_IMAGE_START: c_uint = 0x43000	/* compressed image */;
pub const NETXEN_SECONDARY_START: c_uint = 0x200000	/* backup images */;
pub const NETXEN_PXE_START: c_uint = 0x3E0000	/* PXE boot rom */;
pub const NETXEN_USER_START: c_uint = 0x3E8000	/* Firmware info */;
pub const NETXEN_FIXED_START: c_uint = 0x3F0000	/* backup of crbinit */;

pub const NX_P2_MN_ROMIMAGE: c_int = 0;
pub const NX_P3_CT_ROMIMAGE: c_int = 1;
pub const NX_P3_MN_ROMIMAGE: c_int = 2;
pub const NX_UNIFIED_ROMIMAGE: c_int = 3;
pub const NX_FLASH_ROMIMAGE: c_int = 4;
pub const NX_UNKNOWN_ROMIMAGE: c_uint = 0xff;

// Number of status descriptors to handle per interrupt

//
// netxen_skb_frag{} is to contain mapping info for each SG list. This
// has to be freed when DMA is complete. This is part of netxen_tx_buffer{}.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netxen_skb_frag {
    pub dma: u64,
    pub length: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netxen_recv_crb {
    pub crb_rcv_producer: [u32; NUM_RCV_DESC_RINGS],
    pub crb_sts_consumer: [u32; NUM_STS_DESC_RINGS],
    pub sw_int_mask: [u32; NUM_STS_DESC_RINGS],
}

// Following defines are for the state of the buffers
pub const NETXEN_BUFFER_FREE: c_int = 0;
pub const NETXEN_BUFFER_BUSY: c_int = 1;
//
// There will be one netxen_buffer per skb packet.    These will be
// used to save the dma info for pci_unmap_page()
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netxen_cmd_buffer {
    pub skb: *mut sk_buff,
    pub 1]: netxen_skb_frag frag_array[MAX_SKB_FRAGS +,
    pub frag_count: u32,
}

// In rx_buffer, we do not need multiple fragments as is a single buffer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netxen_rx_buffer {
    pub list: list_head,
    pub skb: *mut sk_buff,
    pub dma: u64,
    pub ref_handle: u16,
    pub state: u16,
}

// Board types
pub const NETXEN_NIC_GBE: c_uint = 0x01;
pub const NETXEN_NIC_XGBE: c_uint = 0x02;
//
// One hardware_context{} per adapter
// contains interrupt info as well shared hardware info.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netxen_hardware_context {
    pub pci_base0: *mut void __iomem,
    pub pci_base1: *mut void __iomem,
    pub pci_base2: *mut void __iomem,
    pub db_base: *mut void __iomem,
    pub ocm_win_crb: *mut void __iomem,
    pub db_len: c_ulong,
    pub pci_len0: c_ulong,
    pub ocm_win: u32,
    pub crb_win: u32,
    pub crb_lock: rwlock_t,
    pub mem_lock: spinlock_t,
    pub cut_through: u8,
    pub revision_id: u8,
    pub pci_func: u8,
    pub linkup: u8,
    pub port_type: u16,
    pub board_type: u16,
}

pub const ETHERNET_FCS_SIZE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netxen_adapter_stats {
    pub xmitcalled: u64,
    pub xmitfinished: u64,
    pub rxdropped: u64,
    pub txdropped: u64,
    pub csummed: u64,
    pub rx_pkts: u64,
    pub lro_pkts: u64,
    pub rxbytes: u64,
    pub txbytes: u64,
}

//
// Rcv Descriptor Context. One such per Rcv Descriptor. There may
// be one Rcv Descriptor for normal packets, one for jumbo and may be others.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nx_host_rds_ring {
    pub producer: u32,
    pub num_desc: u32,
    pub dma_size: u32,
    pub skb_size: u32,
    pub flags: u32,
    pub crb_rcv_producer: *mut void __iomem,
    pub desc_head: *mut rcv_desc,
    pub rx_buf_arr: *mut netxen_rx_buffer,
    pub free_list: list_head,
    pub lock: spinlock_t,
    pub phys_addr: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nx_host_sds_ring {
    pub consumer: u32,
    pub num_desc: u32,
    pub crb_sts_consumer: *mut void __iomem,
    pub crb_intr_mask: *mut void __iomem,
    pub desc_head: *mut status_desc,
    pub adapter: *mut netxen_adapter,
    pub napi: napi_struct,
    pub free_list: [list_head; NUM_RCV_DESC_RINGS],
    pub irq: c_int,
    pub phys_addr: dma_addr_t,
    pub name: [c_char; IFNAMSIZ+4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nx_host_tx_ring {
    pub producer: u32,
    pub hw_consumer: *mut __le32,
    pub sw_consumer: u32,
    pub crb_cmd_producer: *mut void __iomem,
    pub crb_cmd_consumer: *mut void __iomem,
    pub num_desc: u32,
    pub txq: *mut netdev_queue,
    pub cmd_buf_arr: *mut netxen_cmd_buffer,
    pub desc_head: *mut cmd_desc_type0,
    pub phys_addr: dma_addr_t,
}

//
// Receive context. There is one such structure per instance of the
// receive processing. Any state information that is relevant to
// the receive, and is must be in this structure. The global data may be
// present elsewhere.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netxen_recv_context {
    pub state: u32,
    pub context_id: u16,
    pub virt_port: u16,
    pub rds_rings: *mut nx_host_rds_ring,
    pub sds_rings: *mut nx_host_sds_ring,
    pub hwctx: *mut netxen_ring_ctx,
    pub phys_addr: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _cdrp_cmd {
    pub cmd: u32,
    pub arg1: u32,
    pub arg2: u32,
    pub arg3: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netxen_cmd_args {
    pub req: _cdrp_cmd,
    pub rsp: _cdrp_cmd,
}

// New HW context creation
pub const NX_OS_CRB_RETRY_COUNT: c_int = 4000;

pub const NX_CDRP_CLEAR: c_uint = 0x00000000;
pub const NX_CDRP_CMD_BIT: c_uint = 0x80000000;
//
// All responses must have the NX_CDRP_CMD_BIT cleared
// in the crb NX_CDRP_CRB_OFFSET.
//

pub const NX_CDRP_RSP_OK: c_uint = 0x00000001;
pub const NX_CDRP_RSP_FAIL: c_uint = 0x00000002;
pub const NX_CDRP_RSP_TIMEOUT: c_uint = 0x00000003;
//
// All commands must have the NX_CDRP_CMD_BIT set in
// the crb NX_CDRP_CRB_OFFSET.
//

pub const NX_CDRP_CMD_SUBMIT_CAPABILITIES: c_uint = 0x00000001;
pub const NX_CDRP_CMD_READ_MAX_RDS_PER_CTX: c_uint = 0x00000002;
pub const NX_CDRP_CMD_READ_MAX_SDS_PER_CTX: c_uint = 0x00000003;
pub const NX_CDRP_CMD_READ_MAX_RULES_PER_CTX: c_uint = 0x00000004;
pub const NX_CDRP_CMD_READ_MAX_RX_CTX: c_uint = 0x00000005;
pub const NX_CDRP_CMD_READ_MAX_TX_CTX: c_uint = 0x00000006;
pub const NX_CDRP_CMD_CREATE_RX_CTX: c_uint = 0x00000007;
pub const NX_CDRP_CMD_DESTROY_RX_CTX: c_uint = 0x00000008;
pub const NX_CDRP_CMD_CREATE_TX_CTX: c_uint = 0x00000009;
pub const NX_CDRP_CMD_DESTROY_TX_CTX: c_uint = 0x0000000a;
pub const NX_CDRP_CMD_SETUP_STATISTICS: c_uint = 0x0000000e;
pub const NX_CDRP_CMD_GET_STATISTICS: c_uint = 0x0000000f;
pub const NX_CDRP_CMD_DELETE_STATISTICS: c_uint = 0x00000010;
pub const NX_CDRP_CMD_SET_MTU: c_uint = 0x00000012;
pub const NX_CDRP_CMD_READ_PHY: c_uint = 0x00000013;
pub const NX_CDRP_CMD_WRITE_PHY: c_uint = 0x00000014;
pub const NX_CDRP_CMD_READ_HW_REG: c_uint = 0x00000015;
pub const NX_CDRP_CMD_GET_FLOW_CTL: c_uint = 0x00000016;
pub const NX_CDRP_CMD_SET_FLOW_CTL: c_uint = 0x00000017;
pub const NX_CDRP_CMD_READ_MAX_MTU: c_uint = 0x00000018;
pub const NX_CDRP_CMD_READ_MAX_LRO: c_uint = 0x00000019;
pub const NX_CDRP_CMD_CONFIGURE_TOE: c_uint = 0x0000001a;
pub const NX_CDRP_CMD_FUNC_ATTRIB: c_uint = 0x0000001b;
pub const NX_CDRP_CMD_READ_PEXQ_PARAMETERS: c_uint = 0x0000001c;
pub const NX_CDRP_CMD_GET_LIC_CAPABILITIES: c_uint = 0x0000001d;
pub const NX_CDRP_CMD_READ_MAX_LRO_PER_BOARD: c_uint = 0x0000001e;
pub const NX_CDRP_CMD_CONFIG_GBE_PORT: c_uint = 0x0000001f;
pub const NX_CDRP_CMD_MAX: c_uint = 0x00000020;
pub const NX_RCODE_SUCCESS: c_int = 0;
pub const NX_RCODE_NO_HOST_MEM: c_int = 1;
pub const NX_RCODE_NO_HOST_RESOURCE: c_int = 2;
pub const NX_RCODE_NO_CARD_CRB: c_int = 3;
pub const NX_RCODE_NO_CARD_MEM: c_int = 4;
pub const NX_RCODE_NO_CARD_RESOURCE: c_int = 5;
pub const NX_RCODE_INVALID_ARGS: c_int = 6;
pub const NX_RCODE_INVALID_ACTION: c_int = 7;
pub const NX_RCODE_INVALID_STATE: c_int = 8;
pub const NX_RCODE_NOT_SUPPORTED: c_int = 9;
pub const NX_RCODE_NOT_PERMITTED: c_int = 10;
pub const NX_RCODE_NOT_READY: c_int = 11;
pub const NX_RCODE_DOES_NOT_EXIST: c_int = 12;
pub const NX_RCODE_ALREADY_EXISTS: c_int = 13;
pub const NX_RCODE_BAD_SIGNATURE: c_int = 14;
pub const NX_RCODE_CMD_NOT_IMPL: c_int = 15;
pub const NX_RCODE_CMD_INVALID: c_int = 16;
pub const NX_RCODE_TIMEOUT: c_int = 17;
pub const NX_RCODE_CMD_FAILED: c_int = 18;
pub const NX_RCODE_MAX_EXCEEDED: c_int = 19;
pub const NX_RCODE_MAX: c_int = 20;
pub const NX_DESTROY_CTX_RESET: c_int = 0;
pub const NX_DESTROY_CTX_D3_RESET: c_int = 1;
pub const NX_DESTROY_CTX_MAX: c_int = 2;
//
// Capabilities
//

//
// Context state
//
pub const NX_HOST_CTX_STATE_FREED: c_int = 0;
pub const NX_HOST_CTX_STATE_ALLOCATED: c_int = 1;
pub const NX_HOST_CTX_STATE_ACTIVE: c_int = 2;
pub const NX_HOST_CTX_STATE_DISABLED: c_int = 3;
pub const NX_HOST_CTX_STATE_QUIESCED: c_int = 4;
pub const NX_HOST_CTX_STATE_MAX: c_int = 5;
//
// Rx context
//
// These ring offsets are relative to data[0] below
// MUST BE 64-bit aligned.
// These ring offsets are relative to data[0] below
// MUST BE 64-bit aligned.

//
// Tx context
//

// CRB
pub const NX_HOST_RDS_CRB_MODE_UNIQUE: c_int = 0;
pub const NX_HOST_RDS_CRB_MODE_SHARED: c_int = 1;
pub const NX_HOST_RDS_CRB_MODE_CUSTOM: c_int = 2;
pub const NX_HOST_RDS_CRB_MODE_MAX: c_int = 3;
pub const NX_HOST_INT_CRB_MODE_UNIQUE: c_int = 0;
pub const NX_HOST_INT_CRB_MODE_SHARED: c_int = 1;
pub const NX_HOST_INT_CRB_MODE_NORX: c_int = 2;
pub const NX_HOST_INT_CRB_MODE_NOTX: c_int = 3;
pub const NX_HOST_INT_CRB_MODE_NORXTX: c_int = 4;
// MAC
pub const MC_COUNT_P2: c_int = 16;
pub const MC_COUNT_P3: c_int = 38;
pub const NETXEN_MAC_NOOP: c_int = 0;
pub const NETXEN_MAC_ADD: c_int = 1;
pub const NETXEN_MAC_DEL: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nx_ip_list {
    pub list: list_head,
    pub ip_addr: __be32,
    pub master: bool,
}

//
// Interrupt coalescing defaults. The defaults are for 1500 MTU. It is
// adjusted based on configured MTU.
//
pub const NETXEN_DEFAULT_INTR_COALESCE_RX_TIME_US: c_int = 3;
pub const NETXEN_DEFAULT_INTR_COALESCE_RX_PACKETS: c_int = 256;
pub const NETXEN_DEFAULT_INTR_COALESCE_TX_PACKETS: c_int = 64;
pub const NETXEN_DEFAULT_INTR_COALESCE_TX_TIME_US: c_int = 4;
pub const NETXEN_NIC_INTR_DEFAULT: c_uint = 0x04;
pub const NX_HOST_REQUEST: c_uint = 0x13;
pub const NX_NIC_REQUEST: c_uint = 0x14;
pub const NX_MAC_EVENT: c_uint = 0x1;
pub const NX_IP_UP: c_int = 2;
pub const NX_IP_DOWN: c_int = 3;
//
// Driver --> Firmware
//
pub const NX_NIC_H2C_OPCODE_START: c_int = 0;
pub const NX_NIC_H2C_OPCODE_CONFIG_RSS: c_int = 1;
pub const NX_NIC_H2C_OPCODE_CONFIG_RSS_TBL: c_int = 2;
pub const NX_NIC_H2C_OPCODE_CONFIG_INTR_COALESCE: c_int = 3;
pub const NX_NIC_H2C_OPCODE_CONFIG_LED: c_int = 4;
pub const NX_NIC_H2C_OPCODE_CONFIG_PROMISCUOUS: c_int = 5;
pub const NX_NIC_H2C_OPCODE_CONFIG_L2_MAC: c_int = 6;
pub const NX_NIC_H2C_OPCODE_LRO_REQUEST: c_int = 7;
pub const NX_NIC_H2C_OPCODE_GET_SNMP_STATS: c_int = 8;
pub const NX_NIC_H2C_OPCODE_PROXY_START_REQUEST: c_int = 9;
pub const NX_NIC_H2C_OPCODE_PROXY_STOP_REQUEST: c_int = 10;
pub const NX_NIC_H2C_OPCODE_PROXY_SET_MTU: c_int = 11;
pub const NX_NIC_H2C_OPCODE_PROXY_SET_VPORT_MISS_MODE: c_int = 12;
pub const NX_NIC_H2C_OPCODE_GET_FINGER_PRINT_REQUEST: c_int = 13;
pub const NX_NIC_H2C_OPCODE_INSTALL_LICENSE_REQUEST: c_int = 14;
pub const NX_NIC_H2C_OPCODE_GET_LICENSE_CAPABILITY_REQUEST: c_int = 15;
pub const NX_NIC_H2C_OPCODE_GET_NET_STATS: c_int = 16;
pub const NX_NIC_H2C_OPCODE_PROXY_UPDATE_P2V: c_int = 17;
pub const NX_NIC_H2C_OPCODE_CONFIG_IPADDR: c_int = 18;
pub const NX_NIC_H2C_OPCODE_CONFIG_LOOPBACK: c_int = 19;
pub const NX_NIC_H2C_OPCODE_PROXY_STOP_DONE: c_int = 20;
pub const NX_NIC_H2C_OPCODE_GET_LINKEVENT: c_int = 21;
pub const NX_NIC_C2C_OPCODE: c_int = 22;
pub const NX_NIC_H2C_OPCODE_CONFIG_BRIDGING: c_int = 23;
pub const NX_NIC_H2C_OPCODE_CONFIG_HW_LRO: c_int = 24;
pub const NX_NIC_H2C_OPCODE_LAST: c_int = 25;
//
// Firmware --> Driver
//
pub const NX_NIC_C2H_OPCODE_START: c_int = 128;
pub const NX_NIC_C2H_OPCODE_CONFIG_RSS_RESPONSE: c_int = 129;
pub const NX_NIC_C2H_OPCODE_CONFIG_RSS_TBL_RESPONSE: c_int = 130;
pub const NX_NIC_C2H_OPCODE_CONFIG_MAC_RESPONSE: c_int = 131;
pub const NX_NIC_C2H_OPCODE_CONFIG_PROMISCUOUS_RESPONSE: c_int = 132;
pub const NX_NIC_C2H_OPCODE_CONFIG_L2_MAC_RESPONSE: c_int = 133;
pub const NX_NIC_C2H_OPCODE_LRO_DELETE_RESPONSE: c_int = 134;
pub const NX_NIC_C2H_OPCODE_LRO_ADD_FAILURE_RESPONSE: c_int = 135;
pub const NX_NIC_C2H_OPCODE_GET_SNMP_STATS: c_int = 136;
pub const NX_NIC_C2H_OPCODE_GET_FINGER_PRINT_REPLY: c_int = 137;
pub const NX_NIC_C2H_OPCODE_INSTALL_LICENSE_REPLY: c_int = 138;
pub const NX_NIC_C2H_OPCODE_GET_LICENSE_CAPABILITIES_REPLY: c_int = 139;
pub const NX_NIC_C2H_OPCODE_GET_NET_STATS_RESPONSE: c_int = 140;
pub const NX_NIC_C2H_OPCODE_GET_LINKEVENT_RESPONSE: c_int = 141;
pub const NX_NIC_C2H_OPCODE_LAST: c_int = 142;

pub const NX_NIC_LRO_REQUEST_FIRST: c_int = 0;
pub const NX_NIC_LRO_REQUEST_ADD_FLOW: c_int = 1;
pub const NX_NIC_LRO_REQUEST_DELETE_FLOW: c_int = 2;
pub const NX_NIC_LRO_REQUEST_TIMER: c_int = 3;
pub const NX_NIC_LRO_REQUEST_CLEANUP: c_int = 4;
pub const NX_NIC_LRO_REQUEST_ADD_FLOW_SCHEDULED: c_int = 5;
pub const NX_TOE_LRO_REQUEST_ADD_FLOW: c_int = 6;
pub const NX_TOE_LRO_REQUEST_ADD_FLOW_RESPONSE: c_int = 7;
pub const NX_TOE_LRO_REQUEST_DELETE_FLOW: c_int = 8;
pub const NX_TOE_LRO_REQUEST_DELETE_FLOW_RESPONSE: c_int = 9;
pub const NX_TOE_LRO_REQUEST_TIMER: c_int = 10;
pub const NX_NIC_LRO_REQUEST_LAST: c_int = 11;

// module types
pub const LINKEVENT_MODULE_NOT_PRESENT: c_int = 1;
pub const LINKEVENT_MODULE_OPTICAL_UNKNOWN: c_int = 2;
pub const LINKEVENT_MODULE_OPTICAL_SRLR: c_int = 3;
pub const LINKEVENT_MODULE_OPTICAL_LRM: c_int = 4;
pub const LINKEVENT_MODULE_OPTICAL_SFP_1G: c_int = 5;
pub const LINKEVENT_MODULE_TWINAX_UNSUPPORTED_CABLE: c_int = 6;
pub const LINKEVENT_MODULE_TWINAX_UNSUPPORTED_CABLELEN: c_int = 7;
pub const LINKEVENT_MODULE_TWINAX: c_int = 8;
pub const LINKSPEED_10GBPS: c_int = 10000;
pub const LINKSPEED_1GBPS: c_int = 1000;
pub const LINKSPEED_100MBPS: c_int = 100;
pub const LINKSPEED_10MBPS: c_int = 10;
pub const LINKSPEED_ENCODED_10MBPS: c_int = 0;
pub const LINKSPEED_ENCODED_100MBPS: c_int = 1;
pub const LINKSPEED_ENCODED_1GBPS: c_int = 2;
pub const LINKEVENT_AUTONEG_DISABLED: c_int = 0;
pub const LINKEVENT_AUTONEG_ENABLED: c_int = 1;
pub const LINKEVENT_HALF_DUPLEX: c_int = 0;
pub const LINKEVENT_FULL_DUPLEX: c_int = 1;
pub const LINKEVENT_LINKSPEED_MBPS: c_int = 0;
pub const LINKEVENT_LINKSPEED_ENCODED: c_int = 1;
pub const AUTO_FW_RESET_ENABLED: c_uint = 0xEF10AF12;
pub const AUTO_FW_RESET_DISABLED: c_uint = 0xDCBAAF12;
// firmware response header:
// 63:58 - message type
// 57:56 - owner
// 55:53 - desc count
// 52:48 - reserved
// 47:40 - completion id
// 39:32 - opcode
// 31:16 - error code
// 15:00 - reserved
//

pub const MAX_PENDING_DESC_BLOCK_SIZE: c_int = 64;
pub const NETXEN_NIC_MSI_ENABLED: c_uint = 0x02;
pub const NETXEN_NIC_MSIX_ENABLED: c_uint = 0x04;
pub const NETXEN_NIC_LRO_ENABLED: c_uint = 0x08;
pub const NETXEN_NIC_LRO_DISABLED: c_uint = 0x00;

pub const NETXEN_NIC_DIAG_ENABLED: c_uint = 0x20;
pub const NETXEN_FW_RESET_OWNER: c_uint = 0x40;
pub const NETXEN_FW_MSS_CAP: c_uint = 0x80;

pub const NETXEN_MSIX_TBL_SPACE: c_int = 8192;
pub const NETXEN_PCI_REG_MSIX_TBL: c_uint = 0x44;
pub const NETXEN_DB_MAPSIZE_BYTES: c_uint = 0x1000;
pub const NETXEN_ADAPTER_UP_MAGIC: c_int = 777;
pub const NETXEN_NIC_PEG_TUNE: c_int = 0;
pub const __NX_FW_ATTACHED: c_int = 0;
pub const __NX_DEV_UP: c_int = 1;
pub const __NX_RESETTING: c_int = 2;
// Mini Coredump FW supported version
pub const NX_MD_SUPPORT_MAJOR: c_int = 4;
pub const NX_MD_SUPPORT_MINOR: c_int = 0;
pub const NX_MD_SUPPORT_SUBVERSION: c_int = 579;

// Mini Coredump mask level
pub const NX_DUMP_MASK_MIN: c_uint = 0x03;
pub const NX_DUMP_MASK_DEF: c_uint = 0x1f;
pub const NX_DUMP_MASK_MAX: c_uint = 0xff;
// Mini Coredump CDRP commands
pub const NX_CDRP_CMD_TEMP_SIZE: c_uint = 0x0000002f;
pub const NX_CDRP_CMD_GET_TEMP_HDR: c_uint = 0x00000030;
pub const NX_DUMP_STATE_ARRAY_LEN: c_int = 16;
pub const NX_DUMP_CAP_SIZE_ARRAY_LEN: c_int = 8;
// Mini Coredump sysfs entries flags
pub const NX_FORCE_FW_DUMP_KEY: c_uint = 0xdeadfeed;
pub const NX_ENABLE_FW_DUMP: c_uint = 0xaddfeed;
pub const NX_DISABLE_FW_DUMP: c_uint = 0xbadfeed;
pub const NX_FORCE_FW_RESET: c_uint = 0xdeaddead;
// Flash read/write address
pub const NX_FW_DUMP_REG1: c_uint = 0x00130060;
pub const NX_FW_DUMP_REG2: c_uint = 0x001e0000;
pub const NX_FLASH_SEM2_LK: c_uint = 0x0013C010;
pub const NX_FLASH_SEM2_ULK: c_uint = 0x0013C014;
pub const NX_FLASH_LOCK_ID: c_uint = 0x001B2100;
pub const FLASH_ROM_WINDOW: c_uint = 0x42110030;
pub const FLASH_ROM_DATA: c_uint = 0x42150000;
// Mini Coredump register read/write routine

// data = readl((void __iomem *) (bar0 + NX_FW_DUMP_REG2 +        \

//
pub const RDNOP: c_int = 0;
pub const RDCRB: c_int = 1;
pub const RDMUX: c_int = 2;
pub const QUEUE: c_int = 3;
pub const BOARD: c_int = 4;
pub const RDSRE: c_int = 5;
pub const RDOCM: c_int = 6;
pub const PREGS: c_int = 7;
pub const L1DTG: c_int = 8;
pub const L1ITG: c_int = 9;
pub const CACHE: c_int = 10;
pub const L1DAT: c_int = 11;
pub const L1INS: c_int = 12;
pub const RDSTK: c_int = 13;
pub const RDCON: c_int = 14;
pub const L2DTG: c_int = 21;
pub const L2ITG: c_int = 22;
pub const L2DAT: c_int = 23;
pub const L2INS: c_int = 24;
pub const RDOC3: c_int = 25;
pub const MEMBK: c_int = 32;
pub const RDROM: c_int = 71;
pub const RDMEM: c_int = 72;
pub const RDMN: c_int = 73;
pub const INFOR: c_int = 81;
pub const CNTRL: c_int = 98;
pub const TLHDR: c_int = 99;
pub const RDEND: c_int = 255;
pub const PRIMQ: c_int = 103;
pub const SQG2Q: c_int = 104;
pub const SQG3Q: c_int = 105;
//
// Opcodes for Control Entries.
// These Flags are bit fields.
//
pub const NX_DUMP_WCRB: c_uint = 0x01;
pub const NX_DUMP_RWCRB: c_uint = 0x02;
pub const NX_DUMP_ANDCRB: c_uint = 0x04;
pub const NX_DUMP_ORCRB: c_uint = 0x08;
pub const NX_DUMP_POLLCRB: c_uint = 0x10;
pub const NX_DUMP_RD_SAVE: c_uint = 0x20;
pub const NX_DUMP_WRT_SAVED: c_uint = 0x40;
pub const NX_DUMP_MOD_SAVE_ST: c_uint = 0x80;
// Driver Flags
pub const NX_DUMP_SKIP: c_uint = 0x80	/*  driver skipped this entry  */;
pub const NX_DUMP_SIZE_ERR: c_uint = 0x40	/*entry size vs capture size mismatch*/;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netxen_minidump {
    pub /: *mut *mut u32 pos; / position in the dump buffer,
    pub /: *mut *mut u8 fw_supports_md; / FW supports Mini cordump,
    pub /: *mut *mut u8 has_valid_dump; / indicates valid dump,
    pub /: *mut *mut u8 md_capture_mask; / driver capture mask,
    pub /: *mut *mut u8 md_enabled; / Turn Mini Coredump on/off,
    pub /: *mut *mut u32 md_dump_size; / Total FW Mini Coredump size,
    pub /: *mut *mut u32 md_capture_size; / FW dump capture size,
    pub /: *mut *mut u32 md_template_size; / FW template size,
    pub /: *mut *mut u32 md_template_ver; / FW template version,
    pub /: *mut *mut u64 md_timestamp; / FW Mini dump timestamp,
    pub /: *mut *mut *mut void md_template; / FW template will be stored,
    pub /: *mut *mut *mut void md_capture_buff; / FW dump will be stored,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netxen_minidump_template_hdr {
    pub entry_type: u32,
    pub first_entry_offset: u32,
    pub size_of_template: u32,
    pub capture_mask: u32,
    pub num_of_entries: u32,
    pub version: u32,
    pub driver_timestamp: u32,
    pub checksum: u32,
    pub driver_capture_mask: u32,
    pub driver_info_word2: u32,
    pub driver_info_word3: u32,
    pub driver_info_word4: u32,
    pub saved_state_array: [u32; NX_DUMP_STATE_ARRAY_LEN],
    pub capture_size_array: [u32; NX_DUMP_CAP_SIZE_ARRAY_LEN],
    pub rsvd: [u32; ],
}

// Common Entry Header:  Common to All Entry Types
//
// Driver Code is for driver to write some info about the entry.
// Currently not used.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netxen_common_entry_hdr {
    pub entry_type: u32,
    pub entry_size: u32,
    pub entry_capture_size: u32,
    pub entry_capture_mask: u8,
    pub entry_code: u8,
    pub driver_code: u8,
    pub driver_flags: u8,
}

// Generic Entry Including Header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netxen_minidump_entry {
    pub hdr: netxen_common_entry_hdr,
    pub entry_data00: u32,
    pub entry_data01: u32,
    pub entry_data02: u32,
    pub entry_data03: u32,
    pub entry_data04: u32,
    pub entry_data05: u32,
    pub entry_data06: u32,
    pub entry_data07: u32,
}

// Read ROM Header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netxen_minidump_entry_rdrom {
    pub h: netxen_common_entry_hdr,
    pub select_addr_reg: u32,
}

// Read CRB and Control Entry Header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netxen_minidump_entry_crb {
    pub h: netxen_common_entry_hdr,
    pub addr: u32,
    pub addr_stride: u8,
    pub state_index_a: u8,
    pub poll_timeout: u16,
}

// Read Memory and MN Header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netxen_minidump_entry_rdmem {
    pub h: netxen_common_entry_hdr,
    pub select_addr_reg: u32,
}

// Read Cache L1 and L2 Header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netxen_minidump_entry_cache {
    pub h: netxen_common_entry_hdr,
    pub tag_reg_addr: u32,
    pub tag_value_stride: u16,
    pub init_tag_value: u16,
}

// Read OCM Header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netxen_minidump_entry_rdocm {
    pub h: netxen_common_entry_hdr,
    pub rsvd_0: u32,
    pub rsvd_1: u32,
}

// Read MUX Header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netxen_minidump_entry_mux {
    pub h: netxen_common_entry_hdr,
    pub select_addr: u32,
    pub rsvd_0: u32,
}

// Read Queue Header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netxen_minidump_entry_queue {
    pub h: netxen_common_entry_hdr,
    pub select_addr: u32,
    pub queue_id_stride: u16,
    pub rsvd_0: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netxen_dummy_dma {
    pub addr: *mut c_void,
    pub phys_addr: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netxen_adapter {
    pub ahw: netxen_hardware_context,
    pub netdev: *mut net_device,
    pub pdev: *mut pci_dev,
    pub mac_list: list_head,
    pub ip_list: list_head,
    pub tx_clean_lock: spinlock_t,
    pub num_txd: u16,
    pub num_rxd: u16,
    pub num_jumbo_rxd: u16,
    pub num_lro_rxd: u16,
    pub max_rds_rings: u8,
    pub max_sds_rings: u8,
    pub driver_mismatch: u8,
    pub msix_supported: u8,
    pub __pad: u8,
    pub pci_using_dac: u8,
    pub portnum: u8,
    pub physical_port: u8,
    pub mc_enabled: u8,
    pub max_mc_count: u8,
    pub rss_supported: u8,
    pub link_changed: u8,
    pub fw_wait_cnt: u8,
    pub fw_fail_cnt: u8,
    pub tx_timeo_cnt: u8,
    pub need_fw_reset: u8,
    pub has_link_events: u8,
    pub fw_type: u8,
    pub tx_context_id: u16,
    pub mtu: u16,
    pub is_up: u16,
    pub link_speed: u16,
    pub link_duplex: u16,
    pub link_autoneg: u16,
    pub module_type: u16,
    pub capabilities: u32,
    pub flags: u32,
    pub irq: u32,
    pub temp: u32,
    pub int_vec_bit: u32,
    pub heartbit: u32,
    pub mac_addr: [u8; ETH_ALEN],
    pub stats: netxen_adapter_stats,
    pub recv_ctx: netxen_recv_context,
    pub tx_ring: *mut nx_host_tx_ring,
    pub ): *mut *mut *mut int (macaddr_set) (struct netxen_adapter , u8,
    pub int): *mut *mut *mut int (set_mtu) (struct netxen_adapter ,,
    pub u32): *mut *mut *mut int (set_promisc) (struct netxen_adapter ,,
    pub ): *mut *mut void (set_multi) (struct net_device,
    pub ): *mut *mut *mut int (phy_read) (struct netxen_adapter , u32 reg, u32,
    pub val): *mut *mut *mut int (phy_write) (struct netxen_adapter , u32 reg, u32,
    pub int): *mut *mut *mut int (init_port) (struct netxen_adapter ,,
    pub ): *mut *mut int (stop_port) (struct netxen_adapter,
    pub ulong): *mut *mut *mut u32 (crb_read)(struct netxen_adapter ,,
    pub u32): *mut *mut *mut int (crb_write)(struct netxen_adapter , ulong,,
    pub ): *mut *mut *mut int (pci_mem_read)(struct netxen_adapter , u64, u64,
    pub u64): *mut *mut *mut int (pci_mem_write)(struct netxen_adapter , u64,,
    pub ): *mut *mut *mut int (pci_set_window)(struct netxen_adapter , u64, u32,
    pub ): *mut *mut *mut u32 (io_read)(struct netxen_adapter , void __iomem,
    pub u32): *mut *mut *mut *mut void (io_write)(struct netxen_adapter , void __iomem ,,
    pub tgt_mask_reg: *mut void __iomem,
    pub pci_int_reg: *mut void __iomem,
    pub tgt_status_reg: *mut void __iomem,
    pub crb_int_state_reg: *mut void __iomem,
    pub isr_int_vec: *mut void __iomem,
    pub msix_entries: [msix_entry; MSIX_ENTRIES_PER_ADAPTER],
    pub dummy_dma: netxen_dummy_dma,
    pub fw_work: delayed_work,
    pub tx_timeout_task: work_struct,
    pub coal: nx_nic_intr_coalesce_t,
    pub state: c_ulong,
    pub offset*/: *mut *mut __le32 file_prd_off; /File fw product,
    pub fw_version: u32,
    pub fw: *const firmware,
    pub /: *mut *mut netxen_minidump mdump; / mdump ptr,
    pub /: *mut *mut int fw_mdump_rdy; / for mdump ready,
}

extern "C" {
    pub fn nx_fw_cmd_query_phy(adapter: *mut netxen_adapter, reg: u32, val: *mut u32) -> c_int;
}
extern "C" {
    pub fn nx_fw_cmd_set_phy(adapter: *mut netxen_adapter, reg: u32, val: u32) -> c_int;
}

extern "C" {
    pub fn netxen_pcie_sem_lock(: *mut netxen_adapter, _arg: c_int, _arg: u32) -> c_int;
}
extern "C" {
    pub fn netxen_pcie_sem_unlock(: *mut netxen_adapter, _arg: c_int);
}

extern "C" {
    pub fn netxen_nic_get_board_info(adapter: *mut netxen_adapter) -> c_int;
}
extern "C" {
    pub fn netxen_nic_wol_supported(adapter: *mut netxen_adapter) -> c_int;
}
// Functions from netxen_nic_init.c
extern "C" {
    pub fn netxen_init_dummy_dma(adapter: *mut netxen_adapter) -> c_int;
}
extern "C" {
    pub fn netxen_free_dummy_dma(adapter: *mut netxen_adapter);
}
extern "C" {
    pub fn netxen_check_flash_fw_compatibility(adapter: *mut netxen_adapter) -> c_int;
}
extern "C" {
    pub fn netxen_phantom_init(adapter: *mut netxen_adapter, pegtune_val: c_int) -> c_int;
}
extern "C" {
    pub fn netxen_load_firmware(adapter: *mut netxen_adapter) -> c_int;
}
extern "C" {
    pub fn netxen_need_fw_reset(adapter: *mut netxen_adapter) -> c_int;
}
extern "C" {
    pub fn netxen_request_firmware(adapter: *mut netxen_adapter);
}
extern "C" {
    pub fn netxen_release_firmware(adapter: *mut netxen_adapter);
}
extern "C" {
    pub fn netxen_pinit_from_rom(adapter: *mut netxen_adapter) -> c_int;
}
extern "C" {
    pub fn netxen_rom_fast_read(adapter: *mut netxen_adapter, addr: c_int, valp: *mut c_int) -> c_int;
}
extern "C" {
    pub fn netxen_flash_unlock(adapter: *mut netxen_adapter) -> c_int;
}
extern "C" {
    pub fn netxen_backup_crbinit(adapter: *mut netxen_adapter) -> c_int;
}
extern "C" {
    pub fn netxen_flash_erase_secondary(adapter: *mut netxen_adapter) -> c_int;
}
extern "C" {
    pub fn netxen_flash_erase_primary(adapter: *mut netxen_adapter) -> c_int;
}
extern "C" {
    pub fn netxen_halt_pegs(adapter: *mut netxen_adapter);
}
extern "C" {
    pub fn netxen_rom_se(adapter: *mut netxen_adapter, addr: c_int) -> c_int;
}
extern "C" {
    pub fn netxen_alloc_sw_resources(adapter: *mut netxen_adapter) -> c_int;
}
extern "C" {
    pub fn netxen_free_sw_resources(adapter: *mut netxen_adapter);
}
extern "C" {
    pub fn netxen_setup_hwops(adapter: *mut netxen_adapter);
}
extern "C" {
    pub fn netxen_alloc_hw_resources(adapter: *mut netxen_adapter) -> c_int;
}
extern "C" {
    pub fn netxen_free_hw_resources(adapter: *mut netxen_adapter);
}
extern "C" {
    pub fn netxen_release_rx_buffers(adapter: *mut netxen_adapter);
}
extern "C" {
    pub fn netxen_release_tx_buffers(adapter: *mut netxen_adapter);
}
extern "C" {
    pub fn netxen_init_firmware(adapter: *mut netxen_adapter) -> c_int;
}
extern "C" {
    pub fn netxen_nic_clear_stats(adapter: *mut netxen_adapter);
}
extern "C" {
    pub fn netxen_watchdog_task(work: *mut work_struct);
}
extern "C" {
    pub fn netxen_process_cmd_ring(adapter: *mut netxen_adapter) -> c_int;
}
extern "C" {
    pub fn netxen_process_rcv_ring(sds_ring: *mut nx_host_sds_ring, max: c_int) -> c_int;
}
extern "C" {
    pub fn netxen_p3_free_mac_list(adapter: *mut netxen_adapter);
}
extern "C" {
    pub fn netxen_config_intr_coalesce(adapter: *mut netxen_adapter) -> c_int;
}
extern "C" {
    pub fn netxen_config_rss(adapter: *mut netxen_adapter, enable: c_int) -> c_int;
}
extern "C" {
    pub fn netxen_config_ipaddr(adapter: *mut netxen_adapter, ip: __be32, cmd: c_int) -> c_int;
}
extern "C" {
    pub fn netxen_linkevent_request(adapter: *mut netxen_adapter, enable: c_int) -> c_int;
}
extern "C" {
    pub fn netxen_advert_link_change(adapter: *mut netxen_adapter, linkup: c_int);
}
extern "C" {
    pub fn netxen_pci_camqm_read_2M(: *mut netxen_adapter, _arg: u64, : *mut u64);
}
extern "C" {
    pub fn netxen_pci_camqm_write_2M(: *mut netxen_adapter, _arg: u64, _arg: u64);
}
extern "C" {
    pub fn nx_fw_cmd_set_mtu(adapter: *mut netxen_adapter, mtu: c_int) -> c_int;
}
extern "C" {
    pub fn netxen_nic_change_mtu(netdev: *mut net_device, new_mtu: c_int) -> c_int;
}
extern "C" {
    pub fn netxen_config_hw_lro(adapter: *mut netxen_adapter, enable: c_int) -> c_int;
}
extern "C" {
    pub fn netxen_config_bridged_mode(adapter: *mut netxen_adapter, enable: c_int) -> c_int;
}
extern "C" {
    pub fn netxen_send_lro_cleanup(adapter: *mut netxen_adapter) -> c_int;
}
extern "C" {
    pub fn netxen_setup_minidump(adapter: *mut netxen_adapter) -> c_int;
}
extern "C" {
    pub fn netxen_dump_fw(adapter: *mut netxen_adapter);
}
// Functions from netxen_nic_main.c
extern "C" {
    pub fn netxen_nic_reset_context(: *mut netxen_adapter) -> c_int;
}
extern "C" {
    pub fn nx_dev_request_reset(adapter: *mut netxen_adapter) -> c_int;
}
//
// NetXen Board information
//
pub const NETXEN_MAX_SHORT_NAME: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netxen_brdinfo {
    pub /: *mut *mut int brdtype; / type of board,
    pub /: *mut *mut long ports; / max no of physical ports,
    pub short_name: [c_char; NETXEN_MAX_SHORT_NAME],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netxen_dimm_cfg {
    pub presence: u8,
    pub mem_type: u8,
    pub dimm_type: u8,
    pub size: u32,
}

extern "C" {
    pub fn netxen_get_flash_mac_addr(adapter: *mut netxen_adapter, mac: *mut u64) -> c_int;
}
extern "C" {
    pub fn netxen_p3_get_mac_addr(adapter: *mut netxen_adapter, mac: *mut u64) -> c_int;
}
extern "C" {
    pub fn netxen_change_ringparam(adapter: *mut netxen_adapter);
}
