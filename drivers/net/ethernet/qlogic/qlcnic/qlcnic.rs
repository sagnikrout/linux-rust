//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qlogic/qlcnic/qlcnic.h
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
//
// QLogic qlcnic NIC Driver
// Copyright (c) 2009-2013 QLogic Corporation
//

pub const _QLCNIC_LINUX_MAJOR: c_int = 5;
pub const _QLCNIC_LINUX_MINOR: c_int = 3;
pub const _QLCNIC_LINUX_SUBVERSION: c_int = 66;

pub const QLCNIC_DRV_IDC_VER: c_uint = 0x01;

// version in image has weird encoding:
// 7:0  - major
// 15:8  - minor
// 31:16 - build (little endian)
//

// QLCNIC_FLASH_SECTOR_SIZE)

pub const QLCNIC_P3P_A0: c_uint = 0x50;
pub const QLCNIC_P3P_C0: c_uint = 0x58;

pub const FIRST_PAGE_GROUP_START: c_int = 0;
pub const FIRST_PAGE_GROUP_END: c_uint = 0x100000;

pub const QLCNIC_CT_DEFAULT_RX_BUF_LEN: c_int = 2048;
pub const QLCNIC_LRO_BUFFER_EXTRA: c_int = 2048;
// Tx defines
pub const QLCNIC_MAX_FRAGS_PER_TX: c_int = 14;
pub const MAX_TSO_HEADER_DESC: c_int = 2;
pub const MGMT_CMD_DESC_RESV: c_int = 4;

pub const QLCNIC_MAX_TX_TIMEOUTS: c_int = 2;
// Driver will use 1 Tx ring in INT-x/MSI/SRIOV mode.
pub const QLCNIC_SINGLE_RING: c_int = 1;
pub const QLCNIC_DEF_SDS_RINGS: c_int = 4;
pub const QLCNIC_DEF_TX_RINGS: c_int = 4;
pub const QLCNIC_MAX_VNIC_TX_RINGS: c_int = 4;
pub const QLCNIC_MAX_VNIC_SDS_RINGS: c_int = 4;
pub const QLCNIC_83XX_MINIMUM_VECTOR: c_int = 3;
pub const QLCNIC_82XX_MINIMUM_VECTOR: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlcnic_queue_type {
    QLCNIC_TX_QUEUE = 1,
    QLCNIC_RX_QUEUE,
}

// Operational mode for driver
pub const QLCNIC_VNIC_MODE: c_uint = 0xFF;
pub const QLCNIC_DEFAULT_MODE: c_uint = 0x0;
// Virtual NIC function count
pub const QLC_DEFAULT_VNIC_COUNT: c_int = 8;
pub const QLC_84XX_VNIC_COUNT: c_int = 16;
//
// Following are the states of the Phantom. Phantom will set them and
// Host will read to check if the fields are correct.
//
pub const PHAN_INITIALIZE_FAILED: c_uint = 0xffff;
pub const PHAN_INITIALIZE_COMPLETE: c_uint = 0xff01;
// Host writes the following to notify that it has done the init-handshake
pub const PHAN_INITIALIZE_ACK: c_uint = 0xf00f;
pub const PHAN_PEG_RCV_INITIALIZED: c_uint = 0xff01;
pub const NUM_RCV_DESC_RINGS: c_int = 3;
pub const RCV_RING_NORMAL: c_int = 0;
pub const RCV_RING_JUMBO: c_int = 1;
pub const MIN_CMD_DESCRIPTORS: c_int = 64;
pub const MIN_RCV_DESCRIPTORS: c_int = 64;
pub const MIN_JUMBO_DESCRIPTORS: c_int = 32;
pub const MAX_CMD_DESCRIPTORS: c_int = 1024;
pub const MAX_RCV_DESCRIPTORS_1G: c_int = 4096;
pub const MAX_RCV_DESCRIPTORS_10G: c_int = 8192;
pub const MAX_RCV_DESCRIPTORS_VF: c_int = 2048;
pub const MAX_JUMBO_RCV_DESCRIPTORS_1G: c_int = 512;
pub const MAX_JUMBO_RCV_DESCRIPTORS_10G: c_int = 1024;
pub const DEFAULT_RCV_DESCRIPTORS_1G: c_int = 2048;
pub const DEFAULT_RCV_DESCRIPTORS_10G: c_int = 4096;
pub const DEFAULT_RCV_DESCRIPTORS_VF: c_int = 1024;
pub const MAX_RDS_RINGS: c_int = 2;

//
// Following data structures describe the descriptors that will be used.
// Added fileds of tcpHdrSize and ipHdrSize, The driver needs to do it only when
// we are doing LSO (above the 1500 size packet) only.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_desc_type0 {
    pub /: *mut *mut u8 tcp_hdr_offset; / For LSO only,
    pub /: *mut *mut u8 ip_hdr_offset; / For LSO only,
    pub /: *mut *mut __le16 flags_opcode; / 15:13 unused, 12:7 opcode, 6:0 flags,
    pub /: *mut *mut __le32 nfrags__length; / 31:8 total len, 7:0 frag count,
    pub addr_buffer2: __le64,
    pub header,: *mut *mut __le16 encap_descr; / 15:10 offset of outer L3,
// 9:6 number of 32bit words in outer L3 header,
// 5 offload outer L4 checksum,
// 4 offload outer L3 checksum,
// 3 Inner L4 type, TCP=0, UDP=1,
// 2 Inner L3 type, IPv4=0, IPv6=1,
// 1 Outer L3 type,IPv4=0, IPv6=1,
// 0 type of encapsulation, GRE=0, VXLAN=1
//
    pub mss: __le16,
    pub /: *mut *mut u8 port_ctxid; / 7:4 ctxid 3:0 port,
    pub /: *mut *mut u8 hdr_length; / LSO only : MAC+IP+TCP Hdr size,
    pub /: *mut *mut u8 outer_hdr_length; / Encapsulation only,
    pub rsvd1: u8,
    pub addr_buffer3: __le64,
    pub addr_buffer1: __le64,
    pub buffer_length: [__le16; 4],
    pub addr_buffer4: __le64,
    pub eth_addr: [u8; ETH_ALEN],
    pub encapsulation,: *mut *mut __le16 vlan_TCI; / In case of,
// this is for outer VLAN
//
// C attribute field omitted
// Note: sizeof(rcv_desc) should always be a mutliple of 2
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcv_desc {
    pub reference_handle: __le16,
    pub reserved: __le16,
    pub /: *mut *mut __le32 buffer_length; / allocated buffer length (usually 2K),
    pub addr_buffer: __le64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct status_desc {
    pub status_desc_data: [__le64; 2],
// C attribute field omitted
// UNIFIED ROMIMAGE
pub const QLCNIC_UNI_FW_MIN_SIZE: c_uint = 0xc8000;
pub const QLCNIC_UNI_DIR_SECT_PRODUCT_TBL: c_uint = 0x0;
pub const QLCNIC_UNI_DIR_SECT_BOOTLD: c_uint = 0x6;
pub const QLCNIC_UNI_DIR_SECT_FW: c_uint = 0x7;
// Offsets
pub const QLCNIC_UNI_CHIP_REV_OFF: c_int = 10;
pub const QLCNIC_UNI_FLAGS_OFF: c_int = 11;
pub const QLCNIC_UNI_BIOS_VERSION_OFF: c_int = 12;
pub const QLCNIC_UNI_BOOTLD_IDX_OFF: c_int = 27;
pub const QLCNIC_UNI_FIRMWARE_IDX_OFF: c_int = 29;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uni_table_desc {
    pub findex: __le32,
    pub num_entries: __le32,
    pub entry_size: __le32,
    pub reserved: [__le32; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uni_data_desc {
    pub findex: __le32,
    pub size: __le32,
    pub reserved: [__le32; 5],
}

// Flash Defines and Structures
pub const QLCNIC_FLT_LOCATION: c_uint = 0x3F1000;
pub const QLCNIC_FDT_LOCATION: c_uint = 0x3F0000;
pub const QLCNIC_B0_FW_IMAGE_REGION: c_uint = 0x74;
pub const QLCNIC_C0_FW_IMAGE_REGION: c_uint = 0x97;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_flt_header {
    pub version: u16,
    pub len: u16,
    pub checksum: u16,
    pub reserved: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_flt_entry {
    pub region: u8,
    pub reserved0: u8,
    pub attrib: u8,
    pub reserved1: u8,
    pub size: u32,
    pub start_addr: u32,
    pub end_addr: u32,
}

// Flash Descriptor Table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_fdt {
    pub valid: u32,
    pub ver: u16,
    pub len: u16,
    pub cksum: u16,
    pub unused: u16,
    pub model: [u8; 16],
    pub mfg_id: u8,
    pub id: u16,
    pub flag: u8,
    pub erase_cmd: u8,
    pub alt_erase_cmd: u8,
    pub write_enable_cmd: u8,
    pub write_enable_bits: u8,
    pub write_statusreg_cmd: u8,
    pub unprotected_sec_cmd: u8,
    pub read_manuf_cmd: u8,
    pub block_size: u32,
    pub alt_block_size: u32,
    pub flash_size: u32,
    pub write_enable_data: u32,
    pub readid_addr_len: u8,
    pub write_disable_bits: u8,
    pub read_dev_id_len: u8,
    pub chip_erase_cmd: u8,
    pub read_timeo: u16,
    pub protected_sec_cmd: u8,
    pub resvd: [u8; 65],
}

// Magic number to let user know flash is programmed
pub const QLCNIC_BDINFO_MAGIC: c_uint = 0x12345678;
pub const QLCNIC_BRDTYPE_P3P_REF_QG: c_uint = 0x0021;
pub const QLCNIC_BRDTYPE_P3P_HMEZ: c_uint = 0x0022;
pub const QLCNIC_BRDTYPE_P3P_10G_CX4_LP: c_uint = 0x0023;
pub const QLCNIC_BRDTYPE_P3P_4_GB: c_uint = 0x0024;
pub const QLCNIC_BRDTYPE_P3P_IMEZ: c_uint = 0x0025;
pub const QLCNIC_BRDTYPE_P3P_10G_SFP_PLUS: c_uint = 0x0026;
pub const QLCNIC_BRDTYPE_P3P_10000_BASE_T: c_uint = 0x0027;
pub const QLCNIC_BRDTYPE_P3P_XG_LOM: c_uint = 0x0028;
pub const QLCNIC_BRDTYPE_P3P_4_GB_MM: c_uint = 0x0029;
pub const QLCNIC_BRDTYPE_P3P_10G_SFP_CT: c_uint = 0x002a;
pub const QLCNIC_BRDTYPE_P3P_10G_SFP_QT: c_uint = 0x002b;
pub const QLCNIC_BRDTYPE_P3P_10G_CX4: c_uint = 0x0031;
pub const QLCNIC_BRDTYPE_P3P_10G_XFP: c_uint = 0x0032;
pub const QLCNIC_BRDTYPE_P3P_10G_TP: c_uint = 0x0080;
pub const QLCNIC_MSIX_TABLE_OFFSET: c_uint = 0x44;
// Flash memory map
pub const QLCNIC_BRDCFG_START: c_uint = 0x4000		/* board config */;
pub const QLCNIC_BOOTLD_START: c_uint = 0x10000		/* bootld */;
pub const QLCNIC_IMAGE_START: c_uint = 0x43000		/* compressed image */;
pub const QLCNIC_USER_START: c_uint = 0x3E8000	/* Firmware info */;

pub const QLCNIC_UNIFIED_ROMIMAGE: c_int = 0;
pub const QLCNIC_FLASH_ROMIMAGE: c_int = 1;
pub const QLCNIC_UNKNOWN_ROMIMAGE: c_uint = 0xff;

// Number of status descriptors to handle per interrupt

//
// qlcnic_skb_frag{} is to contain mapping info for each SG list. This
// has to be freed when DMA is complete. This is part of qlcnic_tx_buffer{}.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_skb_frag {
    pub dma: u64,
    pub length: u64,
}

// Following defines are for the state of the buffers
pub const QLCNIC_BUFFER_FREE: c_int = 0;
pub const QLCNIC_BUFFER_BUSY: c_int = 1;
//
// There will be one qlcnic_buffer per skb packet.    These will be
// used to save the dma info for dma_unmap_page()
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_cmd_buffer {
    pub skb: *mut sk_buff,
    pub 1]: qlcnic_skb_frag frag_array[MAX_SKB_FRAGS +,
    pub frag_count: u32,
}

// In rx_buffer, we do not need multiple fragments as is a single buffer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_rx_buffer {
    pub ref_handle: u16,
    pub skb: *mut sk_buff,
    pub list: list_head,
    pub dma: u64,
}

// Board types
pub const QLCNIC_GBE: c_uint = 0x01;
pub const QLCNIC_XGBE: c_uint = 0x02;
//
// Interrupt coalescing defaults. The defaults are for 1500 MTU. It is
// adjusted based on configured MTU.
//
pub const QLCNIC_INTR_COAL_TYPE_RX: c_int = 1;
pub const QLCNIC_INTR_COAL_TYPE_TX: c_int = 2;
pub const QLCNIC_INTR_COAL_TYPE_RX_TX: c_int = 3;
pub const QLCNIC_DEF_INTR_COALESCE_RX_TIME_US: c_int = 3;
pub const QLCNIC_DEF_INTR_COALESCE_RX_PACKETS: c_int = 256;
pub const QLCNIC_DEF_INTR_COALESCE_TX_TIME_US: c_int = 64;
pub const QLCNIC_DEF_INTR_COALESCE_TX_PACKETS: c_int = 64;
pub const QLCNIC_INTR_DEFAULT: c_uint = 0x04;
pub const QLCNIC_CONFIG_INTR_COALESCE: c_int = 3;
pub const QLCNIC_DEV_INFO_SIZE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_nic_intr_coalesce {
    pub type: u8,
    pub sts_ring_mask: u8,
    pub rx_packets: u16,
    pub rx_time_us: u16,
    pub tx_packets: u16,
    pub tx_time_us: u16,
    pub flag: u16,
    pub timer_out: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_83xx_dump_template_hdr {
    pub type: u32,
    pub offset: u32,
    pub size: u32,
    pub cap_mask: u32,
    pub num_entries: u32,
    pub version: u32,
    pub timestamp: u32,
    pub checksum: u32,
    pub drv_cap_mask: u32,
    pub sys_info: [u32; 3],
    pub saved_state: [u32; 16],
    pub cap_sizes: [u32; 8],
    pub ocm_wnd_reg: [u32; 16],
    pub rsvd: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_82xx_dump_template_hdr {
    pub type: u32,
    pub offset: u32,
    pub size: u32,
    pub cap_mask: u32,
    pub num_entries: u32,
    pub version: u32,
    pub timestamp: u32,
    pub checksum: u32,
    pub drv_cap_mask: u32,
    pub sys_info: [u32; 3],
    pub saved_state: [u32; 16],
    pub cap_sizes: [u32; 8],
    pub rsvd: [u32; 7],
    pub capabilities: u32,
    pub rsvd1: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_fw_dump {
    pub /: *mut *mut u8 clr; / flag to indicate if dump is cleared,
    pub /: *mut *mut bool enable; / enable/disable dump,
    pub /: *mut *mut u32 size; / total size of the dump,
    pub /: *mut *mut u32 cap_mask; / Current capture mask,
    pub /: *mut *mut *mut void data; / dump data area,
    pub tmpl_hdr: *mut c_void,
    pub phys_addr: dma_addr_t,
    pub dma_buffer: *mut c_void,
    pub use_pex_dma: bool,
// Read only elements which are common between 82xx and 83xx
// template header. Update these values immediately after we read
// template header from Firmware
//
    pub tmpl_hdr_size: u32,
    pub version: u32,
    pub num_entries: u32,
    pub offset: u32,
}

//
// One hardware_context{} per adapter
// contains interrupt info as well shared hardware info.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_hardware_context {
    pub pci_base0: *mut void __iomem,
    pub ocm_win_crb: *mut void __iomem,
    pub pci_len0: c_ulong,
    pub crb_lock: rwlock_t,
    pub mem_lock: mutex,
    pub revision_id: u8,
    pub pci_func: u8,
    pub linkup: u8,
    pub loopback_state: u8,
    pub beacon_state: u8,
    pub has_link_events: u8,
    pub fw_type: u8,
    pub physical_port: u8,
    pub reset_context: u8,
    pub msix_supported: u8,
    pub max_mac_filters: u8,
    pub mc_enabled: u8,
    pub max_mc_count: u8,
    pub diag_test: u8,
    pub num_msix: u8,
    pub nic_mode: u8,
    pub diag_cnt: c_int,
    pub max_uc_count: u16,
    pub port_type: u16,
    pub board_type: u16,
    pub supported_type: u16,
    pub link_speed: u32,
    pub link_duplex: u16,
    pub link_autoneg: u16,
    pub module_type: u16,
    pub op_mode: u16,
    pub switch_mode: u16,
    pub max_tx_ques: u16,
    pub max_rx_ques: u16,
    pub max_mtu: u16,
    pub msg_enable: u32,
    pub total_nic_func: u16,
    pub max_pci_func: u16,
    pub max_vnic_func: u32,
    pub total_pci_func: u32,
    pub capabilities: u32,
    pub extra_capability: [u32; 3],
    pub temp: u32,
    pub int_vec_bit: u32,
    pub fw_hal_version: u32,
    pub port_config: u32,
    pub hw_ops: *mut qlcnic_hardware_ops,
    pub coal: qlcnic_nic_intr_coalesce,
    pub fw_dump: qlcnic_fw_dump,
    pub fdt: qlcnic_fdt,
    pub reset: qlc_83xx_reset,
    pub idc: qlc_83xx_idc,
    pub fw_info: *mut qlc_83xx_fw_info,
    pub intr_tbl: *mut qlcnic_intrpt_config,
    pub sriov: *mut qlcnic_sriov,
    pub reg_tbl: *mut u32,
    pub ext_reg_tbl: *mut u32,
    pub mbox_aen: [u32; QLC_83XX_MBX_AEN_CNT],
    pub mbox_reg: [u32; 4],
    pub mailbox: *mut qlcnic_mailbox,
    pub extend_lb_time: u8,
    pub phys_port_id: [u8; ETH_ALEN],
    pub lb_mode: u8,
    pub hwmon_dev: *mut device,
    pub post_mode: u32,
    pub run_post: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_adapter_stats {
    pub xmitcalled: u64,
    pub xmitfinished: u64,
    pub rxdropped: u64,
    pub txdropped: u64,
    pub csummed: u64,
    pub rx_pkts: u64,
    pub lro_pkts: u64,
    pub rxbytes: u64,
    pub txbytes: u64,
    pub lrobytes: u64,
    pub lso_frames: u64,
    pub encap_lso_frames: u64,
    pub encap_tx_csummed: u64,
    pub encap_rx_csummed: u64,
    pub xmit_on: u64,
    pub xmit_off: u64,
    pub skb_alloc_failure: u64,
    pub null_rxbuf: u64,
    pub rx_dma_map_error: u64,
    pub tx_dma_map_error: u64,
    pub spurious_intr: u64,
    pub mac_filter_limit_overrun: u64,
    pub mbx_spurious_intr: u64,
}

//
// Rcv Descriptor Context. One such per Rcv Descriptor. There may
// be one Rcv Descriptor for normal packets, one for jumbo and may be others.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_host_rds_ring {
    pub crb_rcv_producer: *mut void __iomem,
    pub desc_head: *mut rcv_desc,
    pub rx_buf_arr: *mut qlcnic_rx_buffer,
    pub num_desc: u32,
    pub producer: u32,
    pub dma_size: u32,
    pub skb_size: u32,
    pub flags: u32,
    pub free_list: list_head,
    pub lock: spinlock_t,
    pub phys_addr: dma_addr_t,
    pub ____cacheline_internodealigned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_host_sds_ring {
    pub consumer: u32,
    pub num_desc: u32,
    pub crb_sts_consumer: *mut void __iomem,
    pub tx_ring: *mut qlcnic_host_tx_ring,
    pub desc_head: *mut status_desc,
    pub adapter: *mut qlcnic_adapter,
    pub napi: napi_struct,
    pub free_list: [list_head; NUM_RCV_DESC_RINGS],
    pub crb_intr_mask: *mut void __iomem,
    pub irq: c_int,
    pub phys_addr: dma_addr_t,
    pub 12]: char name[IFNAMSIZ +,
    pub ____cacheline_internodealigned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_tx_queue_stats {
    pub xmit_on: u64,
    pub xmit_off: u64,
    pub xmit_called: u64,
    pub xmit_finished: u64,
    pub tx_bytes: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_host_tx_ring {
    pub irq: c_int,
    pub crb_intr_mask: *mut void __iomem,
    pub 12]: char name[IFNAMSIZ +,
    pub ctx_id: u16,
    pub state: u32,
    pub producer: u32,
    pub sw_consumer: u32,
    pub num_desc: u32,
    pub tx_stats: qlcnic_tx_queue_stats,
    pub crb_cmd_producer: *mut void __iomem,
    pub desc_head: *mut cmd_desc_type0,
    pub adapter: *mut qlcnic_adapter,
    pub napi: napi_struct,
    pub cmd_buf_arr: *mut qlcnic_cmd_buffer,
    pub hw_consumer: *mut __le32,
    pub phys_addr: dma_addr_t,
    pub hw_cons_phys_addr: dma_addr_t,
    pub txq: *mut netdev_queue,
// Lock to protect Tx descriptors cleanup
    pub tx_clean_lock: spinlock_t,
    pub ____cacheline_internodealigned_in_smp: },
//
// Receive context. There is one such structure per instance of the
// receive processing. Any state information that is relevant to
// the receive, and is must be in this structure. The global data may be
// present elsewhere.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_recv_context {
    pub rds_rings: *mut qlcnic_host_rds_ring,
    pub sds_rings: *mut qlcnic_host_sds_ring,
    pub state: u32,
    pub context_id: u16,
    pub virt_port: u16,
}

// HW context creation
pub const QLCNIC_OS_CRB_RETRY_COUNT: c_int = 4000;
pub const QLCNIC_CDRP_CMD_BIT: c_uint = 0x80000000;
//
// All responses must have the QLCNIC_CDRP_CMD_BIT cleared
// in the crb QLCNIC_CDRP_CRB_OFFSET.
//

pub const QLCNIC_CDRP_RSP_OK: c_uint = 0x00000001;
pub const QLCNIC_CDRP_RSP_FAIL: c_uint = 0x00000002;
pub const QLCNIC_CDRP_RSP_TIMEOUT: c_uint = 0x00000003;
//
// All commands must have the QLCNIC_CDRP_CMD_BIT set in
// the crb QLCNIC_CDRP_CRB_OFFSET.
//

pub const QLCNIC_RCODE_SUCCESS: c_int = 0;
pub const QLCNIC_RCODE_INVALID_ARGS: c_int = 6;
pub const QLCNIC_RCODE_NOT_SUPPORTED: c_int = 9;
pub const QLCNIC_RCODE_NOT_PERMITTED: c_int = 10;
pub const QLCNIC_RCODE_NOT_IMPL: c_int = 15;
pub const QLCNIC_RCODE_INVALID: c_int = 16;
pub const QLCNIC_RCODE_TIMEOUT: c_int = 17;
pub const QLCNIC_DESTROY_CTX_RESET: c_int = 0;
//
// Capabilities Announced
//

//
// Context state
//
pub const QLCNIC_HOST_CTX_STATE_FREED: c_int = 0;
pub const QLCNIC_HOST_CTX_STATE_ACTIVE: c_int = 2;
//
// Rx context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_hostrq_sds_ring {
    pub /: *mut *mut __le64 host_phys_addr; / Ring base addr,
    pub /: *mut *mut __le32 ring_size; / Ring entries,
    pub msi_index: __le16,
    pub /: *mut *mut __le16 rsvd; / Padding,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_hostrq_rds_ring {
    pub /: *mut *mut __le64 host_phys_addr; / Ring base addr,
    pub /: *mut *mut __le64 buff_size; / Packet buffer size,
    pub /: *mut *mut __le32 ring_size; / Ring entries,
    pub /: *mut *mut __le32 ring_kind; / Class of ring,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_hostrq_rx_ctx {
    pub /: *mut *mut __le64 host_rsp_dma_addr; / Response dma'd here,
    pub /: *mut *mut __le32 capabilities[4]; / Flag bit vector,
    pub /: *mut *mut __le32 host_int_crb_mode; / Interrupt crb usage,
    pub /: *mut *mut __le32 host_rds_crb_mode; / RDS crb usage,
// These ring offsets are relative to data[0] below
    pub /: *mut *mut __le32 rds_ring_offset; / Offset to RDS config,
    pub /: *mut *mut __le32 sds_ring_offset; / Offset to SDS config,
    pub /: *mut *mut __le16 num_rds_rings; / Count of RDS rings,
    pub /: *mut *mut __le16 num_sds_rings; / Count of SDS rings,
    pub valid_field_offset: __le16,
    pub txrx_sds_binding: u8,
    pub msix_handler: u8,
    pub expansion*/: *mut *mut u8 reserved[128]; / reserve space for future,
// MUST BE 64-bit aligned.
    pub data: [c_char; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_cardrsp_rds_ring {
    pub /: *mut *mut __le32 host_producer_crb; / Crb to use,
    pub /: *mut *mut __le32 rsvd1; / Padding,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_cardrsp_sds_ring {
    pub /: *mut *mut __le32 host_consumer_crb; / Crb to use,
    pub /: *mut *mut __le32 interrupt_crb; / Crb to use,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_cardrsp_rx_ctx {
// These ring offsets are relative to data[0] below
    pub /: *mut *mut __le32 rds_ring_offset; / Offset to RDS config,
    pub /: *mut *mut __le32 sds_ring_offset; / Offset to SDS config,
    pub /: *mut *mut __le32 host_ctx_state; / Starting State,
    pub /: *mut *mut __le32 num_fn_per_port; / How many PCI fn share the port,
    pub /: *mut *mut __le16 num_rds_rings; / Count of RDS rings,
    pub /: *mut *mut __le16 num_sds_rings; / Count of SDS rings,
    pub /: *mut *mut __le16 context_id; / Handle for context,
    pub /: *mut *mut u8 phys_port; / Physical id of port,
    pub /: *mut *mut u8 virt_port; / Virtual/Logical id of port,
    pub /: *mut *mut u8 reserved[128]; / save space for future expansion,
// MUST BE 64-bit aligned.
    pub data: [c_char; ],
    pub __packed: },

//
// Tx context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_hostrq_cds_ring {
    pub /: *mut *mut __le64 host_phys_addr; / Ring base addr,
    pub /: *mut *mut __le32 ring_size; / Ring entries,
    pub /: *mut *mut __le32 rsvd; / Padding,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_hostrq_tx_ctx {
    pub /: *mut *mut __le64 host_rsp_dma_addr; / Response dma'd here,
    pub /: *mut *mut __le64 cmd_cons_dma_addr; /,
    pub /: *mut *mut __le64 dummy_dma_addr; /,
    pub /: *mut *mut __le32 capabilities[4]; / Flag bit vector,
    pub /: *mut *mut __le32 host_int_crb_mode; / Interrupt crb usage,
    pub /: *mut *mut __le32 rsvd1; / Padding,
    pub /: *mut *mut __le16 rsvd2; / Padding,
    pub interrupt_ctl: __le16,
    pub msi_index: __le16,
    pub /: *mut *mut __le16 rsvd3; / Padding,
    pub /: *mut *mut qlcnic_hostrq_cds_ring cds_ring; / Desc of cds ring,
    pub /: *mut *mut u8 reserved[128]; / future expansion,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_cardrsp_cds_ring {
    pub /: *mut *mut __le32 host_producer_crb; / Crb to use,
    pub /: *mut *mut __le32 interrupt_crb; / Crb to use,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_cardrsp_tx_ctx {
    pub /: *mut *mut __le32 host_ctx_state; / Starting state,
    pub /: *mut *mut __le16 context_id; / Handle for context,
    pub /: *mut *mut u8 phys_port; / Physical id of port,
    pub /: *mut *mut u8 virt_port; / Virtual/Logical id of port,
    pub /: *mut *mut qlcnic_cardrsp_cds_ring cds_ring; / Card cds settings,
    pub /: *mut *mut u8 reserved[128]; / future expansion,
    pub __packed: },

// CRB
pub const QLCNIC_HOST_RDS_CRB_MODE_UNIQUE: c_int = 0;
pub const QLCNIC_HOST_RDS_CRB_MODE_SHARED: c_int = 1;
pub const QLCNIC_HOST_RDS_CRB_MODE_CUSTOM: c_int = 2;
pub const QLCNIC_HOST_RDS_CRB_MODE_MAX: c_int = 3;
pub const QLCNIC_HOST_INT_CRB_MODE_UNIQUE: c_int = 0;
pub const QLCNIC_HOST_INT_CRB_MODE_SHARED: c_int = 1;
pub const QLCNIC_HOST_INT_CRB_MODE_NORX: c_int = 2;
pub const QLCNIC_HOST_INT_CRB_MODE_NOTX: c_int = 3;
pub const QLCNIC_HOST_INT_CRB_MODE_NORXTX: c_int = 4;
// MAC
pub const MC_COUNT_P3P: c_int = 38;
pub const QLCNIC_MAC_NOOP: c_int = 0;
pub const QLCNIC_MAC_ADD: c_int = 1;
pub const QLCNIC_MAC_DEL: c_int = 2;
pub const QLCNIC_MAC_VLAN_ADD: c_int = 3;
pub const QLCNIC_MAC_VLAN_DEL: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlcnic_mac_type {
    QLCNIC_UNICAST_MAC,
    QLCNIC_MULTICAST_MAC,
    QLCNIC_BROADCAST_MAC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_mac_vlan_list {
    pub list: list_head,
    pub mac_addr: [u8; ETH_ALEN+2],
    pub vlan_id: u16,
    pub mac_type: qlcnic_mac_type,
}

// MAC Learn
pub const NO_MAC_LEARN: c_int = 0;
pub const DRV_MAC_LEARN: c_int = 1;
pub const FDB_MAC_LEARN: c_int = 2;
pub const QLCNIC_HOST_REQUEST: c_uint = 0x13;
pub const QLCNIC_REQUEST: c_uint = 0x14;
pub const QLCNIC_MAC_EVENT: c_uint = 0x1;
pub const QLCNIC_IP_UP: c_int = 2;
pub const QLCNIC_IP_DOWN: c_int = 3;
pub const QLCNIC_ILB_MODE: c_uint = 0x1;
pub const QLCNIC_ELB_MODE: c_uint = 0x2;
pub const QLCNIC_LB_MODE_MASK: c_uint = 0x3;
pub const QLCNIC_LINKEVENT: c_uint = 0x1;
pub const QLCNIC_LB_RESPONSE: c_uint = 0x2;

//
// Driver --> Firmware
//
pub const QLCNIC_H2C_OPCODE_CONFIG_RSS: c_uint = 0x1;
pub const QLCNIC_H2C_OPCODE_CONFIG_INTR_COALESCE: c_uint = 0x3;
pub const QLCNIC_H2C_OPCODE_CONFIG_LED: c_uint = 0x4;
pub const QLCNIC_H2C_OPCODE_LRO_REQUEST: c_uint = 0x7;
pub const QLCNIC_H2C_OPCODE_SET_MAC_RECEIVE_MODE: c_uint = 0xc;
pub const QLCNIC_H2C_OPCODE_CONFIG_IPADDR: c_uint = 0x12;
pub const QLCNIC_H2C_OPCODE_GET_LINKEVENT: c_uint = 0x15;
pub const QLCNIC_H2C_OPCODE_CONFIG_BRIDGING: c_uint = 0x17;
pub const QLCNIC_H2C_OPCODE_CONFIG_HW_LRO: c_uint = 0x18;
pub const QLCNIC_H2C_OPCODE_CONFIG_LOOPBACK: c_uint = 0x13;
//
// Firmware --> Driver
//
pub const QLCNIC_C2H_OPCODE_CONFIG_LOOPBACK: c_uint = 0x8f;
pub const QLCNIC_C2H_OPCODE_GET_LINKEVENT_RESPONSE: c_uint = 0x8D;
pub const QLCNIC_C2H_OPCODE_GET_DCB_AEN: c_uint = 0x90;

pub const QLCNIC_LRO_REQUEST_CLEANUP: c_int = 4;
// Capabilites received

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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_fw_msg {
    pub hdr: u64,
    pub body: [u64; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_nic_req {
    pub qhdr: __le64,
    pub req_hdr: __le64,
    pub words: [__le64; 6],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_mac_req {
    pub op: u8,
    pub tag: u8,
    pub mac_addr: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_vlan_req {
    pub vlan_id: __le16,
    pub rsvd: [__le16; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_ipaddr {
    pub ipv4: __be32,
    pub ipv6: [__be32; 4],
}

pub const QLCNIC_MSI_ENABLED: c_uint = 0x02;
pub const QLCNIC_MSIX_ENABLED: c_uint = 0x04;
pub const QLCNIC_LRO_ENABLED: c_uint = 0x01;
pub const QLCNIC_LRO_DISABLED: c_uint = 0x00;

pub const QLCNIC_DIAG_ENABLED: c_uint = 0x20;
pub const QLCNIC_ESWITCH_ENABLED: c_uint = 0x40;
pub const QLCNIC_ADAPTER_INITIALIZED: c_uint = 0x80;
pub const QLCNIC_TAGGING_ENABLED: c_uint = 0x100;
pub const QLCNIC_MACSPOOF: c_uint = 0x200;
pub const QLCNIC_MAC_OVERRIDE_DISABLED: c_uint = 0x400;
pub const QLCNIC_PROMISC_DISABLED: c_uint = 0x800;
pub const QLCNIC_NEED_FLR: c_uint = 0x1000;
pub const QLCNIC_FW_RESET_OWNER: c_uint = 0x2000;
pub const QLCNIC_FW_HANG: c_uint = 0x4000;
pub const QLCNIC_FW_LRO_MSS_CAP: c_uint = 0x8000;
pub const QLCNIC_TX_INTR_SHARED: c_uint = 0x10000;
pub const QLCNIC_APP_CHANGED_FLAGS: c_uint = 0x20000;
pub const QLCNIC_HAS_PHYS_PORT_ID: c_uint = 0x40000;
pub const QLCNIC_TSS_RSS: c_uint = 0x80000;
pub const QLCNIC_VLAN_FILTERING: c_uint = 0x800000;

pub const QLCNIC_BEACON_EANBLE: c_uint = 0xC;
pub const QLCNIC_BEACON_DISABLE: c_uint = 0xD;
pub const QLCNIC_BEACON_ON: c_int = 2;
pub const QLCNIC_BEACON_OFF: c_int = 0;
pub const QLCNIC_MSIX_TBL_SPACE: c_int = 8192;
pub const QLCNIC_PCI_REG_MSIX_TBL: c_uint = 0x44;
pub const QLCNIC_MSIX_TBL_PGSIZE: c_int = 4096;
pub const QLCNIC_ADAPTER_UP_MAGIC: c_int = 777;
pub const __QLCNIC_FW_ATTACHED: c_int = 0;
pub const __QLCNIC_DEV_UP: c_int = 1;
pub const __QLCNIC_RESETTING: c_int = 2;
pub const __QLCNIC_START_FW: c_int = 4;
pub const __QLCNIC_AER: c_int = 5;
pub const __QLCNIC_DIAG_RES_ALLOC: c_int = 6;
pub const __QLCNIC_LED_ENABLE: c_int = 7;
pub const __QLCNIC_ELB_INPROGRESS: c_int = 8;
pub const __QLCNIC_MULTI_TX_UNIQUE: c_int = 9;
pub const __QLCNIC_SRIOV_ENABLE: c_int = 10;
pub const __QLCNIC_SRIOV_CAPABLE: c_int = 11;
pub const __QLCNIC_MBX_POLL_ENABLE: c_int = 12;
pub const __QLCNIC_DIAG_MODE: c_int = 13;
pub const __QLCNIC_MAINTENANCE_MODE: c_int = 16;
pub const QLCNIC_INTERRUPT_TEST: c_int = 1;
pub const QLCNIC_LOOPBACK_TEST: c_int = 2;
pub const QLCNIC_LED_TEST: c_int = 3;
pub const QLCNIC_FILTER_AGE: c_int = 80;
pub const QLCNIC_READD_AGE: c_int = 20;
pub const QLCNIC_LB_MAX_FILTERS: c_int = 64;
pub const QLCNIC_LB_BUCKET_SIZE: c_int = 32;
pub const QLCNIC_ILB_MAX_RCV_LOOP: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_filter {
    pub fnode: hlist_node,
    pub faddr: [u8; ETH_ALEN],
    pub vlan_id: u16,
    pub ftime: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_filter_hash {
    pub fhead: *mut hlist_head,
    pub fnum: u8,
    pub fmax: u16,
    pub fbucket_size: u16,
}

// Mailbox specific data structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_mailbox {
    pub work_q: *mut workqueue_struct,
    pub adapter: *mut qlcnic_adapter,
    pub ops: *const qlcnic_mbx_ops,
    pub work: work_struct,
    pub completion: completion,
    pub cmd_q: list_head,
    pub status: c_ulong,
    pub /: *mut *mut spinlock_t queue_lock; / Mailbox queue lock,
    pub /: *mut *mut spinlock_t aen_lock; / Mailbox response/AEN lock,
    pub rsp_status: u32,
    pub num_cmds: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_adapter {
    pub ahw: *mut qlcnic_hardware_context,
    pub recv_ctx: *mut qlcnic_recv_context,
    pub tx_ring: *mut qlcnic_host_tx_ring,
    pub netdev: *mut net_device,
    pub pdev: *mut pci_dev,
    pub state: c_ulong,
    pub flags: u32,
    pub num_txd: u16,
    pub num_rxd: u16,
    pub num_jumbo_rxd: u16,
    pub max_rxd: u16,
    pub max_jumbo_rxd: u16,
    pub max_rds_rings: u8,
    pub /: *mut *mut u8 max_sds_rings; / max sds rings supported by adapter,
    pub /: *mut *mut u8 max_tx_rings; / max tx rings supported by adapter,
    pub /: *mut *mut u8 drv_tx_rings; / max tx rings supported by driver,
    pub /: *mut *mut u8 drv_sds_rings; / max sds rings supported by driver,
    pub /: *mut *mut u8 drv_tss_rings; / tss ring input,
    pub /: *mut *mut u8 drv_rss_rings; / rss ring input,
    pub rx_csum: u8,
    pub portnum: u8,
    pub fw_wait_cnt: u8,
    pub fw_fail_cnt: u8,
    pub tx_timeo_cnt: u8,
    pub need_fw_reset: u8,
    pub reset_ctx_cnt: u8,
    pub is_up: u16,
    pub rx_pvid: u16,
    pub tx_pvid: u16,
    pub irq: u32,
    pub heartbeat: u32,
    pub dev_state: u8,
    pub reset_ack_timeo: u8,
    pub dev_init_timeo: u8,
    pub mac_addr: [u8; ETH_ALEN],
    pub dev_rst_time: u64,
    pub drv_mac_learn: bool,
    pub fdb_mac_learn: bool,
    pub rx_mac_learn: bool,
    pub vlans: [c_ulong; BITS_TO_LONGS(VLAN_N_VID)],
    pub flash_mfg_id: u8,
    pub npars: *mut qlcnic_npar_info,
    pub eswitch: *mut qlcnic_eswitch,
    pub nic_ops: *mut qlcnic_nic_template,
    pub stats: qlcnic_adapter_stats,
    pub mac_list: list_head,
    pub tgt_mask_reg: *mut void __iomem,
    pub tgt_status_reg: *mut void __iomem,
    pub crb_int_state_reg: *mut void __iomem,
    pub isr_int_vec: *mut void __iomem,
    pub msix_entries: *mut msix_entry,
    pub qlcnic_wq: *mut workqueue_struct,
    pub fw_work: delayed_work,
    pub idc_aen_work: delayed_work,
    pub mbx_poll_work: delayed_work,
    pub dcb: *mut qlcnic_dcb,
    pub fhash: qlcnic_filter_hash,
    pub rx_fhash: qlcnic_filter_hash,
    pub vf_mc_list: list_head,
    pub mac_learn_lock: spinlock_t,
// spinlock for catching rcv filters for eswitch traffic
    pub rx_mac_learn_lock: spinlock_t,
    pub offset*/: *mut *mut u32 file_prd_off; /File fw product,
    pub fw_version: u32,
    pub offload_flags: u32,
    pub fw: *const firmware,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_info_le {
    pub pci_func: __le16,
    pub /: *mut *mut __le16 op_mode; / 1 = Priv, 2 = NP, 3 = NP passthru,
    pub phys_port: __le16,
    pub /: *mut *mut __le16 switch_mode; / 0 = disabled, 1 = int, 2 = ext,
    pub capabilities: __le32,
    pub max_mac_filters: u8,
    pub reserved1: u8,
    pub max_mtu: __le16,
    pub max_tx_ques: __le16,
    pub max_rx_ques: __le16,
    pub min_tx_bw: __le16,
    pub max_tx_bw: __le16,
    pub op_type: __le32,
    pub max_bw_reg_offset: __le16,
    pub max_linkspeed_reg_offset: __le16,
    pub capability1: __le32,
    pub capability2: __le32,
    pub capability3: __le32,
    pub max_tx_mac_filters: __le16,
    pub max_rx_mcast_mac_filters: __le16,
    pub max_rx_ucast_mac_filters: __le16,
    pub max_rx_ip_addr: __le16,
    pub max_rx_lro_flow: __le16,
    pub max_rx_status_rings: __le16,
    pub max_rx_buf_rings: __le16,
    pub max_tx_vlan_keys: __le16,
    pub total_pf: u8,
    pub total_rss_engines: u8,
    pub max_vports: __le16,
    pub linkstate_reg_offset: __le16,
    pub bit_offsets: __le16,
    pub max_local_ipv6_addrs: __le16,
    pub max_remote_ipv6_addrs: __le16,
    pub reserved2: [u8; 56],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_info {
    pub pci_func: u16,
    pub op_mode: u16,
    pub phys_port: u16,
    pub switch_mode: u16,
    pub capabilities: u32,
    pub max_mac_filters: u8,
    pub max_mtu: u16,
    pub max_tx_ques: u16,
    pub max_rx_ques: u16,
    pub min_tx_bw: u16,
    pub max_tx_bw: u16,
    pub op_type: u32,
    pub max_bw_reg_offset: u16,
    pub max_linkspeed_reg_offset: u16,
    pub capability1: u32,
    pub capability2: u32,
    pub capability3: u32,
    pub max_tx_mac_filters: u16,
    pub max_rx_mcast_mac_filters: u16,
    pub max_rx_ucast_mac_filters: u16,
    pub max_rx_ip_addr: u16,
    pub max_rx_lro_flow: u16,
    pub max_rx_status_rings: u16,
    pub max_rx_buf_rings: u16,
    pub max_tx_vlan_keys: u16,
    pub total_pf: u8,
    pub total_rss_engines: u8,
    pub max_vports: u16,
    pub linkstate_reg_offset: u16,
    pub bit_offsets: u16,
    pub max_local_ipv6_addrs: u16,
    pub max_remote_ipv6_addrs: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_pci_info_le {
    pub /: *mut *mut __le16 id; / pci function id,
    pub /: *mut *mut __le16 active; / 1 = Enabled,
    pub /: *mut *mut __le16 type; / 1 = NIC, 2 = FCoE, 3 = iSCSI,
    pub /: *mut *mut __le16 default_port; / default port number,
    pub /: *mut *mut __le16 tx_min_bw; / Multiple of 100mbpc,
    pub tx_max_bw: __le16,
    pub reserved1: [__le16; 2],
    pub mac: [u8; ETH_ALEN],
    pub func_count: __le16,
    pub reserved2: [u8; 104],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_pci_info {
    pub id: u16,
    pub active: u16,
    pub type: u16,
    pub default_port: u16,
    pub tx_min_bw: u16,
    pub tx_max_bw: u16,
    pub mac: [u8; ETH_ALEN],
    pub func_count: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_npar_info {
    pub eswitch_status: bool,
    pub pvid: u16,
    pub min_bw: u16,
    pub max_bw: u16,
    pub phy_port: u8,
    pub type: u8,
    pub active: u8,
    pub enable_pm: u8,
    pub dest_npar: u8,
    pub discard_tagged: u8,
    pub mac_override: u8,
    pub mac_anti_spoof: u8,
    pub promisc_mode: u8,
    pub offload_flags: u8,
    pub pci_func: u8,
    pub mac: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_eswitch {
    pub port: u8,
    pub active_vports: u8,
    pub active_vlans: u8,
    pub active_ucast_filters: u8,
    pub max_ucast_filters: u8,
    pub max_active_vlans: u8,
    pub flags: u32,

}

pub const MAX_VLAN_ID: c_int = 4095;
pub const MIN_VLAN_ID: c_int = 2;
pub const DEFAULT_MAC_LEARN: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_pci_func_cfg {
    pub func_type: u16,
    pub min_bw: u16,
    pub max_bw: u16,
    pub port_num: u16,
    pub pci_func: u8,
    pub func_state: u8,
    pub def_mac_addr: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_npar_func_cfg {
    pub fw_capab: u32,
    pub port_num: u16,
    pub min_bw: u16,
    pub max_bw: u16,
    pub max_tx_queues: u16,
    pub max_rx_queues: u16,
    pub pci_func: u8,
    pub op_mode: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_pm_func_cfg {
    pub pci_func: u8,
    pub action: u8,
    pub dest_npar: u8,
    pub reserved: [u8; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_esw_func_cfg {
    pub vlan_id: u16,
    pub op_mode: u8,
    pub op_type: u8,
    pub pci_func: u8,
    pub host_vlan_tag: u8,
    pub promisc_mode: u8,
    pub discard_tagged: u8,
    pub mac_override: u8,
    pub mac_anti_spoof: u8,
    pub offload_flags: u8,
    pub reserved: [u8; 5],
}

pub const QLCNIC_STATS_VERSION: c_int = 1;
pub const QLCNIC_STATS_PORT: c_int = 1;
pub const QLCNIC_STATS_ESWITCH: c_int = 2;
pub const QLCNIC_QUERY_RX_COUNTER: c_int = 0;
pub const QLCNIC_QUERY_TX_COUNTER: c_int = 1;
pub const QLCNIC_STATS_NOT_AVAIL: c_uint = 0xffffffffffffffffULL;

pub const QLCNIC_MAC_STATS: c_int = 1;
pub const QLCNIC_ESW_STATS: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_mac_statistics_le {
    pub mac_tx_frames: __le64,
    pub mac_tx_bytes: __le64,
    pub mac_tx_mcast_pkts: __le64,
    pub mac_tx_bcast_pkts: __le64,
    pub mac_tx_pause_cnt: __le64,
    pub mac_tx_ctrl_pkt: __le64,
    pub mac_tx_lt_64b_pkts: __le64,
    pub mac_tx_lt_127b_pkts: __le64,
    pub mac_tx_lt_255b_pkts: __le64,
    pub mac_tx_lt_511b_pkts: __le64,
    pub mac_tx_lt_1023b_pkts: __le64,
    pub mac_tx_lt_1518b_pkts: __le64,
    pub mac_tx_gt_1518b_pkts: __le64,
    pub rsvd1: [__le64; 3],
    pub mac_rx_frames: __le64,
    pub mac_rx_bytes: __le64,
    pub mac_rx_mcast_pkts: __le64,
    pub mac_rx_bcast_pkts: __le64,
    pub mac_rx_pause_cnt: __le64,
    pub mac_rx_ctrl_pkt: __le64,
    pub mac_rx_lt_64b_pkts: __le64,
    pub mac_rx_lt_127b_pkts: __le64,
    pub mac_rx_lt_255b_pkts: __le64,
    pub mac_rx_lt_511b_pkts: __le64,
    pub mac_rx_lt_1023b_pkts: __le64,
    pub mac_rx_lt_1518b_pkts: __le64,
    pub mac_rx_gt_1518b_pkts: __le64,
    pub rsvd2: [__le64; 3],
    pub mac_rx_length_error: __le64,
    pub mac_rx_length_small: __le64,
    pub mac_rx_length_large: __le64,
    pub mac_rx_jabber: __le64,
    pub mac_rx_dropped: __le64,
    pub mac_rx_crc_error: __le64,
    pub mac_align_error: __le64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_mac_statistics {
    pub mac_tx_frames: u64,
    pub mac_tx_bytes: u64,
    pub mac_tx_mcast_pkts: u64,
    pub mac_tx_bcast_pkts: u64,
    pub mac_tx_pause_cnt: u64,
    pub mac_tx_ctrl_pkt: u64,
    pub mac_tx_lt_64b_pkts: u64,
    pub mac_tx_lt_127b_pkts: u64,
    pub mac_tx_lt_255b_pkts: u64,
    pub mac_tx_lt_511b_pkts: u64,
    pub mac_tx_lt_1023b_pkts: u64,
    pub mac_tx_lt_1518b_pkts: u64,
    pub mac_tx_gt_1518b_pkts: u64,
    pub rsvd1: [u64; 3],
    pub mac_rx_frames: u64,
    pub mac_rx_bytes: u64,
    pub mac_rx_mcast_pkts: u64,
    pub mac_rx_bcast_pkts: u64,
    pub mac_rx_pause_cnt: u64,
    pub mac_rx_ctrl_pkt: u64,
    pub mac_rx_lt_64b_pkts: u64,
    pub mac_rx_lt_127b_pkts: u64,
    pub mac_rx_lt_255b_pkts: u64,
    pub mac_rx_lt_511b_pkts: u64,
    pub mac_rx_lt_1023b_pkts: u64,
    pub mac_rx_lt_1518b_pkts: u64,
    pub mac_rx_gt_1518b_pkts: u64,
    pub rsvd2: [u64; 3],
    pub mac_rx_length_error: u64,
    pub mac_rx_length_small: u64,
    pub mac_rx_length_large: u64,
    pub mac_rx_jabber: u64,
    pub mac_rx_dropped: u64,
    pub mac_rx_crc_error: u64,
    pub mac_align_error: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_esw_stats_le {
    pub context_id: __le16,
    pub version: __le16,
    pub size: __le16,
    pub unused: __le16,
    pub unicast_frames: __le64,
    pub multicast_frames: __le64,
    pub broadcast_frames: __le64,
    pub dropped_frames: __le64,
    pub errors: __le64,
    pub local_frames: __le64,
    pub numbytes: __le64,
    pub rsvd: [__le64; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __qlcnic_esw_statistics {
    pub context_id: u16,
    pub version: u16,
    pub size: u16,
    pub unused: u16,
    pub unicast_frames: u64,
    pub multicast_frames: u64,
    pub broadcast_frames: u64,
    pub dropped_frames: u64,
    pub errors: u64,
    pub local_frames: u64,
    pub numbytes: u64,
    pub rsvd: [u64; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_esw_statistics {
    pub rx: __qlcnic_esw_statistics,
    pub tx: __qlcnic_esw_statistics,
}

pub const QLCNIC_FORCE_FW_DUMP_KEY: c_uint = 0xdeadfeed;
pub const QLCNIC_ENABLE_FW_DUMP: c_uint = 0xaddfeed;
pub const QLCNIC_DISABLE_FW_DUMP: c_uint = 0xbadfeed;
pub const QLCNIC_FORCE_FW_RESET: c_uint = 0xdeaddead;
pub const QLCNIC_SET_QUIESCENT: c_uint = 0xadd00010;
pub const QLCNIC_RESET_QUIESCENT: c_uint = 0xadd00020;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _cdrp_cmd {
    pub num: u32,
    pub arg: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_cmd_args {
    pub completion: completion,
    pub list: list_head,
    pub req: _cdrp_cmd,
    pub rsp: _cdrp_cmd,
    pub rsp_status: core::sync::atomic::AtomicI32,
    pub pay_size: c_int,
    pub rsp_opcode: u32,
    pub total_cmds: u32,
    pub op_type: u32,
    pub type: u32,
    pub cmd_op: u32,
    pub /: *mut *mut *mut u32 hdr; / Back channel message header,
    pub /: *mut *mut *mut u32 pay; / Back channel message payload,
    pub func_num: u8,
}

extern "C" {
    pub fn qlcnic_fw_cmd_get_minidump_temp(adapter: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_fw_cmd_set_port(adapter: *mut qlcnic_adapter, config: u32) -> c_int;
}
extern "C" {
    pub fn qlcnic_pci_mem_write_2M(: *mut qlcnic_adapter, off: u64, data: u64) -> c_int;
}
extern "C" {
    pub fn qlcnic_pci_mem_read_2M(: *mut qlcnic_adapter, off: u64, data: *mut u64) -> c_int;
}

extern "C" {
    pub fn qlcnic_pcie_sem_lock(: *mut qlcnic_adapter, _arg: c_int, _arg: u32) -> c_int;
}
extern "C" {
    pub fn qlcnic_pcie_sem_unlock(: *mut qlcnic_adapter, _arg: c_int);
}

pub const __QLCNIC_MAX_LED_RATE: c_uint = 0xf;
pub const __QLCNIC_MAX_LED_STATE: c_uint = 0x2;
pub const MAX_CTL_CHECK: c_int = 1000;
extern "C" {
    pub fn qlcnic_prune_lb_filters(adapter: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_delete_lb_filters(adapter: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_dump_fw(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_enable_fw_dump_state(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_check_fw_dump_state(: *mut qlcnic_adapter) -> bool;
}
// Functions from qlcnic_init.c
extern "C" {
    pub fn qlcnic_schedule_work(: *mut qlcnic_adapter, _arg: work_func_t, _arg: c_int);
}
extern "C" {
    pub fn qlcnic_load_firmware(adapter: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_need_fw_reset(adapter: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_request_firmware(adapter: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_release_firmware(adapter: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_pinit_from_rom(adapter: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_setup_idc_param(adapter: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_check_flash_fw_ver(adapter: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_rom_fast_read(adapter: *mut qlcnic_adapter, addr: u32, valp: *mut u32) -> c_int;
}
extern "C" {
    pub fn qlcnic_alloc_sw_resources(adapter: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_free_sw_resources(adapter: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_alloc_hw_resources(adapter: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_free_hw_resources(adapter: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_fw_create_ctx(adapter: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_fw_destroy_ctx(adapter: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_reset_rx_buffers_list(adapter: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_release_rx_buffers(adapter: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_check_fw_status(adapter: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_set_multi(netdev: *mut net_device);
}
extern "C" {
    pub fn qlcnic_flush_mcast_mac(: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_nic_del_mac(: *mut qlcnic_adapter, : *const u8) -> c_int;
}
extern "C" {
    pub fn qlcnic_82xx_free_mac_list(adapter: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_82xx_read_phys_port_id(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_fw_cmd_set_mtu(adapter: *mut qlcnic_adapter, mtu: c_int) -> c_int;
}
extern "C" {
    pub fn qlcnic_fw_cmd_set_drv_version(: *mut qlcnic_adapter, _arg: u32) -> c_int;
}
extern "C" {
    pub fn qlcnic_change_mtu(netdev: *mut net_device, new_mtu: c_int) -> c_int;
}
extern "C" {
    pub fn qlcnic_set_features(netdev: *mut net_device, features: netdev_features_t) -> c_int;
}
extern "C" {
    pub fn qlcnic_config_bridged_mode(adapter: *mut qlcnic_adapter, enable: u32) -> c_int;
}
extern "C" {
    pub fn qlcnic_update_cmd_producer(: *mut qlcnic_host_tx_ring);
}
// Functions from qlcnic_ethtool.c
extern "C" {
    pub fn qlcnic_check_loopback_buff(: *mut c_uchar, []: u8) -> c_int;
}
extern "C" {
    pub fn qlcnic_do_lb_test(: *mut qlcnic_adapter, _arg: u8) -> c_int;
}
// Functions from qlcnic_main.c
extern "C" {
    pub fn qlcnic_reset_context(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_diag_free_res(netdev: *mut net_device, _arg: c_int);
}
extern "C" {
    pub fn qlcnic_diag_alloc_res(netdev: *mut net_device, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn qlcnic_xmit_frame(: *mut sk_buff, : *mut net_device) -> netdev_tx_t;
}
extern "C" {
    pub fn qlcnic_set_tx_ring_count(: *mut qlcnic_adapter, _arg: u8);
}
extern "C" {
    pub fn qlcnic_set_sds_ring_count(: *mut qlcnic_adapter, _arg: u8);
}
extern "C" {
    pub fn qlcnic_setup_rings(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_validate_rings(: *mut qlcnic_adapter, _arg: __u32, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn qlcnic_alloc_lb_filters_mem(adapter: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_enable_msix(: *mut qlcnic_adapter, _arg: u32) -> c_int;
}
extern "C" {
    pub fn qlcnic_set_drv_version(: *mut qlcnic_adapter);
}
// eSwitch management functions
extern "C" {
    pub fn qlcnic_config_port_mirroring(: *mut qlcnic_adapter, _arg: u8, _arg: u8, _arg: u8) -> c_int;
}
extern "C" {
    pub fn qlcnic_clear_esw_stats(adapter: *mut qlcnic_adapter, _arg: u8, _arg: u8, _arg: u8) -> c_int;
}
extern "C" {
    pub fn qlcnic_get_mac_stats(: *mut qlcnic_adapter, : *mut qlcnic_mac_statistics) -> c_int;
}
extern "C" {
    pub fn qlcnic_free_mbx_args(cmd: *mut qlcnic_cmd_args);
}
extern "C" {
    pub fn qlcnic_alloc_sds_rings(: *mut qlcnic_recv_context, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn qlcnic_free_sds_rings(: *mut qlcnic_recv_context);
}
extern "C" {
    pub fn qlcnic_advert_link_change(: *mut qlcnic_adapter, _arg: c_int);
}
extern "C" {
    pub fn qlcnic_free_tx_rings(: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_alloc_tx_rings(: *mut qlcnic_adapter, : *mut net_device) -> c_int;
}
extern "C" {
    pub fn qlcnic_dump_mbx(: *mut qlcnic_adapter, : *mut qlcnic_cmd_args);
}
extern "C" {
    pub fn qlcnic_create_sysfs_entries(adapter: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_remove_sysfs_entries(adapter: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_82xx_add_sysfs(adapter: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_82xx_remove_sysfs(adapter: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnicvf_config_bridged_mode(: *mut qlcnic_adapter, _arg: u32) -> c_int;
}
extern "C" {
    pub fn qlcnicvf_config_led(: *mut qlcnic_adapter, _arg: u32, _arg: u32) -> c_int;
}
extern "C" {
    pub fn qlcnic_setup_tss_rss_intr(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_down(: *mut qlcnic_adapter, : *mut net_device);
}
extern "C" {
    pub fn qlcnic_up(: *mut qlcnic_adapter, : *mut net_device) -> c_int;
}
extern "C" {
    pub fn __qlcnic_down(: *mut qlcnic_adapter, : *mut net_device);
}
extern "C" {
    pub fn qlcnic_detach(: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_teardown_intr(: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_attach(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn __qlcnic_up(: *mut qlcnic_adapter, : *mut net_device) -> c_int;
}
extern "C" {
    pub fn qlcnic_restore_indev_addr(: *mut net_device, long: unsigned);
}
extern "C" {
    pub fn qlcnic_check_temp(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_init_pci_info(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_set_default_offload_settings(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_reset_npar_config(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_set_eswitch_port_config(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_set_vxlan_port(adapter: *mut qlcnic_adapter, port: u16) -> c_int;
}
extern "C" {
    pub fn qlcnic_set_vxlan_parsing(adapter: *mut qlcnic_adapter, port: u16) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_configure_opmode(adapter: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_read_mac_addr(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_setup_netdev(: *mut qlcnic_adapter, : *mut net_device) -> c_int;
}
extern "C" {
    pub fn qlcnic_sriov_vf_set_multi(: *mut net_device);
}
extern "C" {
    pub fn qlcnic_is_valid_nic_func(: *mut qlcnic_adapter, _arg: u8) -> c_int;
}
//
// QLOGIC Board information
//
pub const QLCNIC_MAX_BOARD_NAME_LEN: c_int = 100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_board_info {
    pub vendor: c_ushort,
    pub device: c_ushort,
    pub sub_vendor: c_ushort,
    pub sub_device: c_ushort,
    pub short_name: [c_char; QLCNIC_MAX_BOARD_NAME_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_nic_template {
    pub u32): *mut *mut *mut int (config_bridged_mode) (struct qlcnic_adapter ,,
    pub u32): *mut *mut *mut int (config_led) (struct qlcnic_adapter , u32,,
    pub ): *mut *mut int (start_firmware) (struct qlcnic_adapter,
    pub ): *mut *mut int (init_driver) (struct qlcnic_adapter,
    pub u32): *mut *mut *mut void (request_reset) (struct qlcnic_adapter ,,
    pub ): *mut *mut void (cancel_idc_work) (struct qlcnic_adapter,
    pub ): *mut *mut *mut int (napi_add)(struct qlcnic_adapter , struct net_device,
    pub ): *mut *mut void (napi_del)(struct qlcnic_adapter,
    pub int): *mut *mut *mut void (config_ipaddr)(struct qlcnic_adapter , __be32,,
    pub ): *mut *mut irqreturn_t (clear_legacy_intr)(struct qlcnic_adapter,
    pub ): *mut *mut int (shutdown)(struct pci_dev,
    pub ): *mut *mut int (resume)(struct qlcnic_adapter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_mbx_ops {
    pub ): *mut *mut qlcnic_cmd_args , unsigned long,
    pub ): *mut *mut *mut void (dequeue_cmd) (struct qlcnic_adapter , struct qlcnic_cmd_args,
    pub ): *mut *mut *mut void (decode_resp) (struct qlcnic_adapter , struct qlcnic_cmd_args,
    pub ): *mut *mut *mut void (encode_cmd) (struct qlcnic_adapter , struct qlcnic_cmd_args,
    pub u8): *mut *mut *mut void (nofity_fw) (struct qlcnic_adapter ,,
}

extern "C" {
    pub fn qlcnic_83xx_init_mailbox_work(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_detach_mailbox_work(: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_83xx_reinit_mbx_work(mbx: *mut qlcnic_mailbox);
}
extern "C" {
    pub fn qlcnic_83xx_free_mailbox(mbx: *mut qlcnic_mailbox);
}
extern "C" {
    pub fn qlcnic_update_stats(: *mut qlcnic_adapter);
}
// Adapter hardware abstraction
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_hardware_ops {
    pub size_t): *mut *mut *mut *mut void (read_crb) (struct qlcnic_adapter , char , loff_t,,
    pub size_t): *mut *mut *mut *mut void (write_crb) (struct qlcnic_adapter , char , loff_t,,
    pub ): *mut *mut *mut int (read_reg) (struct qlcnic_adapter , ulong, int,
    pub u32): *mut *mut *mut int (write_reg) (struct qlcnic_adapter , ulong,,
    pub ): *mut *mut void (get_ocm_win) (struct qlcnic_hardware_context,
    pub u8): *mut *mut *mut *mut int (get_mac_address) (struct qlcnic_adapter , u8 ,,
    pub ): *mut *mut int (setup_intr) (struct qlcnic_adapter,
    pub u32): *mut *mut qlcnic_adapter ,,
    pub ): *mut *mut *mut int (mbx_cmd) (struct qlcnic_adapter , struct qlcnic_cmd_args,
    pub ): *mut *mut void (get_func_no) (struct qlcnic_adapter,
    pub ): *mut *mut int (api_lock) (struct qlcnic_adapter,
    pub ): *mut *mut void (api_unlock) (struct qlcnic_adapter,
    pub ): *mut *mut void (add_sysfs) (struct qlcnic_adapter,
    pub ): *mut *mut void (remove_sysfs) (struct qlcnic_adapter,
    pub ): *mut *mut void (process_lb_rcv_ring_diag) (struct qlcnic_host_sds_ring,
    pub ): *mut *mut int (create_rx_ctx) (struct qlcnic_adapter,
    pub int): *mut *mut qlcnic_host_tx_ring ,,
    pub ): *mut *mut void (del_rx_ctx) (struct qlcnic_adapter,
    pub ): *mut qlcnic_host_tx_ring,
    pub int): *mut *mut *mut int (setup_link_event) (struct qlcnic_adapter ,,
    pub u8): *mut *mut *mut *mut int (get_nic_info) (struct qlcnic_adapter , struct qlcnic_info ,,
    pub ): *mut *mut *mut int (get_pci_info) (struct qlcnic_adapter , struct qlcnic_pci_info,
    pub ): *mut *mut *mut int (set_nic_info) (struct qlcnic_adapter , struct qlcnic_info,
    pub u8): *mut *mut *mut *mut int (change_macvlan) (struct qlcnic_adapter , u8, u16,,
    pub ): *mut *mut void (napi_enable) (struct qlcnic_adapter,
    pub ): *mut *mut void (napi_disable) (struct qlcnic_adapter,
    pub ): *mut ethtool_coalesce,
    pub int): *mut *mut *mut int (config_rss) (struct qlcnic_adapter ,,
    pub int): *mut *mut *mut int (config_hw_lro) (struct qlcnic_adapter ,,
    pub u8): *mut *mut *mut int (config_loopback) (struct qlcnic_adapter ,,
    pub u8): *mut *mut *mut int (clear_loopback) (struct qlcnic_adapter ,,
    pub u32): *mut *mut *mut int (config_promisc_mode) (struct qlcnic_adapter ,,
    pub tx_ring): *mut u16 vlan, struct qlcnic_host_tx_ring,
    pub ): *mut *mut int (get_board_info) (struct qlcnic_adapter,
    pub ): *mut *mut void (set_mac_filter_count) (struct qlcnic_adapter,
    pub ): *mut *mut void (free_mac_list) (struct qlcnic_adapter,
    pub ): *mut *mut int (read_phys_port_id) (struct qlcnic_adapter,
    pub ): *mut *mut pci_ers_result_t (io_slot_reset) (struct pci_dev,
    pub ): *mut *mut void (io_resume) (struct pci_dev,
    pub ): *mut *mut void (get_beacon_state)(struct qlcnic_adapter,
    pub ): *mut qlcnic_host_sds_ring,
    pub ): *mut qlcnic_host_sds_ring,
    pub ): *mut qlcnic_host_tx_ring,
    pub ): *mut qlcnic_host_tx_ring,
    pub u32): *mut *mut *mut u32 (get_saved_state)(void ,,
    pub u32): *mut *mut *mut void (set_saved_state)(void , u32,,
    pub ): *mut *mut void (cache_tmpl_hdr_values)(struct qlcnic_fw_dump,
    pub int): *mut *mut *mut u32 (get_cap_size)(void ,,
    pub u32): *mut *mut *mut void (set_sys_info)(void , int,,
    pub u32): *mut *mut *mut void (store_cap_mask)(void ,,
    pub adapter): *mut *mut bool (encap_rx_offload) (struct qlcnic_adapter,
    pub adapter): *mut *mut bool (encap_tx_offload) (struct qlcnic_adapter,
}

extern "C" {
    pub fn test_bit(_arg: __QLCNIC_MULTI_TX_UNIQUE, _arg: &adapter->state) -> return;
}
// Enable MSI-x and INT-x interrupts
// Disable MSI-x and INT-x interrupts
// When operating in a muti tx mode, driver needs to write 0x1
// to src register, instead of 0x0 to disable receiving interrupt.
//
// When operating in a muti tx mode, driver needs to write 0x0
// to src register, instead of 0x1 to enable receiving interrupts.
//
extern "C" {
    pub fn test_and_set_bit(_arg: __QLCNIC_DIAG_MODE, _arg: &adapter->state) -> return;
}
extern "C" {
    pub fn test_bit(_arg: __QLCNIC_DIAG_MODE, _arg: &adapter->state) -> return;
}

pub const PCI_DEVICE_ID_QLOGIC_QLE824X: c_uint = 0x8020;
pub const PCI_DEVICE_ID_QLOGIC_QLE834X: c_uint = 0x8030;
pub const PCI_DEVICE_ID_QLOGIC_VF_QLE834X: c_uint = 0x8430;
pub const PCI_DEVICE_ID_QLOGIC_QLE8830: c_uint = 0x8830;
pub const PCI_DEVICE_ID_QLOGIC_VF_QLE8C30: c_uint = 0x8C30;
pub const PCI_DEVICE_ID_QLOGIC_QLE844X: c_uint = 0x8040;
pub const PCI_DEVICE_ID_QLOGIC_VF_QLE844X: c_uint = 0x8440;

// tmp = swab32(*tmp);

extern "C" {
    pub fn qlcnic_register_hwmon_dev(: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_unregister_hwmon_dev(: *mut qlcnic_adapter);
}

