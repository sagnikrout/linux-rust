//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/synopsys/dwc-xlgmac.h
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


// Synopsys DesignWare Core Enterprise Ethernet (XLGMAC) Driver
//
// Copyright (c) 2017 Synopsys, Inc. (www.synopsys.com)
//
// This program is dual-licensed; you may select either version 2 of
// the GNU General Public License ("GPL") or BSD license ("BSD").
//
// This Synopsys DWC XLGMAC software driver and associated documentation
// (hereinafter the "Software") is an unsupported proprietary work of
// Synopsys, Inc. unless otherwise expressly agreed to in writing between
// Synopsys and you. The Software IS NOT an item of Licensed Software or a
// Licensed Product under any End User Software License Agreement or
// Agreement for Licensed Products with Synopsys or any supplement thereto.
// Synopsys is a registered trademark of Synopsys, Inc. Other names included
// in the SOFTWARE may be the trademarks of their respective owners.
//

// Descriptor related parameters
pub const XLGMAC_TX_DESC_CNT: c_int = 1024;

pub const XLGMAC_RX_DESC_CNT: c_int = 1024;

// Descriptors required for maximum contiguous TSO/GSO packet

// Maximum possible descriptors needed for a SKB

pub const XLGMAC_RX_BUF_ALIGN: c_int = 64;
// Maximum Size for Splitting the Header Data
// Keep in sync with SKB_ALLOC_SIZE
// 3'b000: 64 bytes, 3'b001: 128 bytes
// 3'b010: 256 bytes, 3'b011: 512 bytes
// 3'b100: 1023 bytes ,   3'b101'3'b111: Reserved
//
pub const XLGMAC_SPH_HDSMS_SIZE: c_int = 3;
pub const XLGMAC_SKB_ALLOC_SIZE: c_int = 512;
pub const XLGMAC_MAX_FIFO: c_int = 81920;
pub const XLGMAC_MAX_DMA_CHANNELS: c_int = 16;
pub const XLGMAC_DMA_STOP_TIMEOUT: c_int = 5;
pub const XLGMAC_DMA_INTERRUPT_MASK: c_uint = 0x31c7;
// Default coalescing parameters
pub const XLGMAC_INIT_DMA_TX_USECS: c_int = 1000;
pub const XLGMAC_INIT_DMA_TX_FRAMES: c_int = 25;
pub const XLGMAC_INIT_DMA_RX_USECS: c_int = 30;
pub const XLGMAC_INIT_DMA_RX_FRAMES: c_int = 25;
pub const XLGMAC_MAX_DMA_RIWT: c_uint = 0xff;
pub const XLGMAC_MIN_DMA_RIWT: c_uint = 0x01;
// Flow control queue count
pub const XLGMAC_MAX_FLOW_CONTROL_QUEUES: c_int = 8;
// System clock is 125 MHz
pub const XLGMAC_SYSCLOCK: c_int = 125000000;
// Maximum MAC address hash table size (256 bits = 8 bytes)
pub const XLGMAC_MAC_HASH_TABLE_SIZE: c_int = 8;
// Receive Side Scaling
pub const XLGMAC_RSS_HASH_KEY_SIZE: c_int = 40;
pub const XLGMAC_RSS_MAX_TABLE_SIZE: c_int = 256;
pub const XLGMAC_RSS_LOOKUP_TABLE_TYPE: c_int = 0;
pub const XLGMAC_RSS_HASH_KEY_TYPE: c_int = 1;
pub const XLGMAC_STD_PACKET_MTU: c_int = 1500;
pub const XLGMAC_JUMBO_PACKET_MTU: c_int = 9000;
// Helper macro for descriptor handling
// Always use XLGMAC_GET_DESC_DATA to access the descriptor data
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xlgmac_int {
    XLGMAC_INT_DMA_CH_SR_TI,
    XLGMAC_INT_DMA_CH_SR_TPS,
    XLGMAC_INT_DMA_CH_SR_TBU,
    XLGMAC_INT_DMA_CH_SR_RI,
    XLGMAC_INT_DMA_CH_SR_RBU,
    XLGMAC_INT_DMA_CH_SR_RPS,
    XLGMAC_INT_DMA_CH_SR_TI_RI,
    XLGMAC_INT_DMA_CH_SR_FBE,
    XLGMAC_INT_DMA_ALL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xlgmac_stats {
// MMC TX counters
    pub txoctetcount_gb: u64,
    pub txframecount_gb: u64,
    pub txbroadcastframes_g: u64,
    pub txmulticastframes_g: u64,
    pub tx64octets_gb: u64,
    pub tx65to127octets_gb: u64,
    pub tx128to255octets_gb: u64,
    pub tx256to511octets_gb: u64,
    pub tx512to1023octets_gb: u64,
    pub tx1024tomaxoctets_gb: u64,
    pub txunicastframes_gb: u64,
    pub txmulticastframes_gb: u64,
    pub txbroadcastframes_gb: u64,
    pub txunderflowerror: u64,
    pub txoctetcount_g: u64,
    pub txframecount_g: u64,
    pub txpauseframes: u64,
    pub txvlanframes_g: u64,
// MMC RX counters
    pub rxframecount_gb: u64,
    pub rxoctetcount_gb: u64,
    pub rxoctetcount_g: u64,
    pub rxbroadcastframes_g: u64,
    pub rxmulticastframes_g: u64,
    pub rxcrcerror: u64,
    pub rxrunterror: u64,
    pub rxjabbererror: u64,
    pub rxundersize_g: u64,
    pub rxoversize_g: u64,
    pub rx64octets_gb: u64,
    pub rx65to127octets_gb: u64,
    pub rx128to255octets_gb: u64,
    pub rx256to511octets_gb: u64,
    pub rx512to1023octets_gb: u64,
    pub rx1024tomaxoctets_gb: u64,
    pub rxunicastframes_g: u64,
    pub rxlengtherror: u64,
    pub rxoutofrangetype: u64,
    pub rxpauseframes: u64,
    pub rxfifooverflow: u64,
    pub rxvlanframes_gb: u64,
    pub rxwatchdogerror: u64,
// Extra counters
    pub tx_tso_packets: u64,
    pub rx_split_header_packets: u64,
    pub tx_process_stopped: u64,
    pub rx_process_stopped: u64,
    pub tx_buffer_unavailable: u64,
    pub rx_buffer_unavailable: u64,
    pub fatal_bus_error: u64,
    pub tx_vlan_packets: u64,
    pub rx_vlan_packets: u64,
    pub napi_poll_isr: u64,
    pub napi_poll_txtimer: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xlgmac_ring_buf {
    pub skb: *mut sk_buff,
    pub skb_dma: dma_addr_t,
    pub skb_len: c_uint,
}

// Common Tx and Rx DMA hardware descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xlgmac_dma_desc {
    pub desc0: __le32,
    pub desc1: __le32,
    pub desc2: __le32,
    pub desc3: __le32,
}

// Page allocation related values
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xlgmac_page_alloc {
    pub pages: *mut page,
    pub pages_len: c_uint,
    pub pages_offset: c_uint,
    pub pages_dma: dma_addr_t,
}

// Ring entry buffer data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xlgmac_buffer_data {
    pub pa: xlgmac_page_alloc,
    pub pa_unmap: xlgmac_page_alloc,
    pub dma_base: dma_addr_t,
    pub dma_off: c_ulong,
    pub dma_len: c_uint,
}

// Tx-related desc data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xlgmac_tx_desc_data {
    pub /: *mut *mut unsigned int packets; / BQL packet count,
    pub /: *mut *mut unsigned int bytes; / BQL byte count,
}

// Rx-related desc data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xlgmac_rx_desc_data {
    pub /: *mut *mut xlgmac_buffer_data hdr; / Header locations,
    pub /: *mut *mut xlgmac_buffer_data buf; / Payload locations,
    pub /: *mut *mut unsigned short hdr_len; / Length of received header,
    pub /: *mut *mut unsigned short len; / Length of received packet,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xlgmac_pkt_info {
    pub skb: *mut sk_buff,
    pub attributes: c_uint,
    pub errors: c_uint,
// descriptors needed for this packet
    pub desc_count: c_uint,
    pub length: c_uint,
    pub tx_packets: c_uint,
    pub tx_bytes: c_uint,
    pub header_len: c_uint,
    pub tcp_header_len: c_uint,
    pub tcp_payload_len: c_uint,
    pub mss: c_ushort,
    pub vlan_ctag: c_ushort,
    pub rx_tstamp: u64,
    pub rss_hash: u32,
    pub rss_hash_type: pkt_hash_types,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xlgmac_desc_data {
// dma_desc: Virtual address of descriptor
// dma_desc_addr: DMA address of descriptor
//
    pub dma_desc: *mut xlgmac_dma_desc,
    pub dma_desc_addr: dma_addr_t,
// skb: Virtual address of SKB
// skb_dma: DMA address of SKB data
// skb_dma_len: Length of SKB DMA area
//
    pub skb: *mut sk_buff,
    pub skb_dma: dma_addr_t,
    pub skb_dma_len: c_uint,
// Tx/Rx -related data
    pub tx: xlgmac_tx_desc_data,
    pub rx: xlgmac_rx_desc_data,
    pub mapped_as_page: c_uint,
// Incomplete receive save location.  If the budget is exhausted
// or the last descriptor (last normal descriptor or a following
// context descriptor) has not been DMA'd yet the current state
// of the receive processing needs to be saved.
//
    pub state_saved: c_uint,
    pub skb: *mut sk_buff,
    pub len: c_uint,
    pub error: c_uint,
    pub state: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xlgmac_ring {
// Per packet related information
    pub pkt_info: xlgmac_pkt_info,
// Virtual/DMA addresses of DMA descriptor list and the total count
    pub dma_desc_head: *mut xlgmac_dma_desc,
    pub dma_desc_head_addr: dma_addr_t,
    pub dma_desc_count: c_uint,
// Array of descriptor data corresponding the DMA descriptor
// (always use the XLGMAC_GET_DESC_DATA macro to access this data)
//
    pub desc_data_head: *mut xlgmac_desc_data,
// Page allocation for RX buffers
    pub rx_hdr_pa: xlgmac_page_alloc,
    pub rx_buf_pa: xlgmac_page_alloc,
// Ring index values
// cur   - Tx: index of descriptor to be used for current transfer
// Rx: index of descriptor to check for packet availability
// dirty - Tx: index of descriptor to check for transfer complete
// Rx: index of descriptor to check for buffer reallocation
//
    pub cur: c_uint,
    pub dirty: c_uint,
// Coalesce frame count used for interrupt bit setting
    pub coalesce_count: c_uint,
    pub xmit_more: c_uint,
    pub queue_stopped: c_uint,
    pub cur_mss: c_ushort,
    pub cur_vlan_ctag: c_ushort,
    pub tx: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xlgmac_channel {
    pub name: [c_char; 16],
// Address of private data area for device
    pub pdata: *mut xlgmac_pdata,
// Queue index and base address of queue's DMA registers
    pub queue_index: c_uint,
    pub dma_regs: *mut void __iomem,
// Per channel interrupt irq number
    pub dma_irq: c_int,
    pub 32]: char dma_irq_name[IFNAMSIZ +,
// Netdev related settings
    pub napi: napi_struct,
    pub saved_ier: c_uint,
    pub tx_timer_active: c_uint,
    pub tx_timer: timer_list,
    pub tx_ring: *mut xlgmac_ring,
    pub rx_ring: *mut xlgmac_ring,
    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xlgmac_desc_ops {
    pub pdata): *mut *mut int (alloc_channels_and_rings)(struct xlgmac_pdata,
    pub pdata): *mut *mut void (free_channels_and_rings)(struct xlgmac_pdata,
    pub skb): *mut sk_buff,
    pub desc_data): *mut xlgmac_desc_data,
    pub desc_data): *mut xlgmac_desc_data,
    pub pdata): *mut *mut void (tx_desc_init)(struct xlgmac_pdata,
    pub pdata): *mut *mut void (rx_desc_init)(struct xlgmac_pdata,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xlgmac_hw_ops {
    pub pdata): *mut *mut int (init)(struct xlgmac_pdata,
    pub pdata): *mut *mut int (exit)(struct xlgmac_pdata,
    pub dma_desc): *mut *mut int (tx_complete)(struct xlgmac_dma_desc,
    pub pdata): *mut *mut void (enable_tx)(struct xlgmac_pdata,
    pub pdata): *mut *mut void (disable_tx)(struct xlgmac_pdata,
    pub pdata): *mut *mut void (enable_rx)(struct xlgmac_pdata,
    pub pdata): *mut *mut void (disable_rx)(struct xlgmac_pdata,
    pub int_id): xlgmac_int,
    pub int_id): xlgmac_int,
    pub channel): *mut *mut void (dev_xmit)(struct xlgmac_channel,
    pub channel): *mut *mut int (dev_read)(struct xlgmac_channel,
    pub addr): *const *const *const int (set_mac_address)(struct xlgmac_pdata pdata, u8,
    pub pdata): *mut *mut int (config_rx_mode)(struct xlgmac_pdata,
    pub pdata): *mut *mut int (enable_rx_csum)(struct xlgmac_pdata,
    pub pdata): *mut *mut int (disable_rx_csum)(struct xlgmac_pdata,
// For MII speed configuration
    pub pdata): *mut *mut int (set_xlgmii_25000_speed)(struct xlgmac_pdata,
    pub pdata): *mut *mut int (set_xlgmii_40000_speed)(struct xlgmac_pdata,
    pub pdata): *mut *mut int (set_xlgmii_50000_speed)(struct xlgmac_pdata,
    pub pdata): *mut *mut int (set_xlgmii_100000_speed)(struct xlgmac_pdata,
// For descriptor related operation
    pub channel): *mut *mut void (tx_desc_init)(struct xlgmac_channel,
    pub channel): *mut *mut void (rx_desc_init)(struct xlgmac_channel,
    pub desc_data): *mut *mut void (tx_desc_reset)(struct xlgmac_desc_data,
    pub index): c_uint,
    pub dma_desc): *mut *mut int (is_last_desc)(struct xlgmac_dma_desc,
    pub dma_desc): *mut *mut int (is_context_desc)(struct xlgmac_dma_desc,
    pub ring): *mut xlgmac_ring,
// For Flow Control
    pub pdata): *mut *mut int (config_tx_flow_control)(struct xlgmac_pdata,
    pub pdata): *mut *mut int (config_rx_flow_control)(struct xlgmac_pdata,
// For Vlan related config
    pub pdata): *mut *mut int (enable_rx_vlan_stripping)(struct xlgmac_pdata,
    pub pdata): *mut *mut int (disable_rx_vlan_stripping)(struct xlgmac_pdata,
    pub pdata): *mut *mut int (enable_rx_vlan_filtering)(struct xlgmac_pdata,
    pub pdata): *mut *mut int (disable_rx_vlan_filtering)(struct xlgmac_pdata,
    pub pdata): *mut *mut int (update_vlan_hash_table)(struct xlgmac_pdata,
// For RX coalescing
    pub pdata): *mut *mut int (config_rx_coalesce)(struct xlgmac_pdata,
    pub pdata): *mut *mut int (config_tx_coalesce)(struct xlgmac_pdata,
    pub usec): c_uint,
    pub riwt): c_uint,
// For RX and TX threshold config
    pub val): c_uint,
    pub val): c_uint,
// For RX and TX Store and Forward Mode config
    pub val): c_uint,
    pub val): c_uint,
// For TX DMA Operate on Second Frame config
    pub pdata): *mut *mut int (config_osp_mode)(struct xlgmac_pdata,
// For RX and TX PBL config
    pub pdata): *mut *mut int (config_rx_pbl_val)(struct xlgmac_pdata,
    pub pdata): *mut *mut int (get_rx_pbl_val)(struct xlgmac_pdata,
    pub pdata): *mut *mut int (config_tx_pbl_val)(struct xlgmac_pdata,
    pub pdata): *mut *mut int (get_tx_pbl_val)(struct xlgmac_pdata,
    pub pdata): *mut *mut int (config_pblx8)(struct xlgmac_pdata,
// For MMC statistics
    pub pdata): *mut *mut void (rx_mmc_int)(struct xlgmac_pdata,
    pub pdata): *mut *mut void (tx_mmc_int)(struct xlgmac_pdata,
    pub pdata): *mut *mut void (read_mmc_stats)(struct xlgmac_pdata,
// For Receive Side Scaling
    pub pdata): *mut *mut int (enable_rss)(struct xlgmac_pdata,
    pub pdata): *mut *mut int (disable_rss)(struct xlgmac_pdata,
    pub key): *const u8,
    pub table): *const u32,
}

// This structure contains flags that indicate what hardware features
// or configurations are present in the device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xlgmac_hw_features {
// HW Version
    pub version: c_uint,
// HW Feature Register0
    pub /: *mut *mut unsigned int phyifsel; / PHY interface support,
    pub /: *mut *mut unsigned int vlhash; / VLAN Hash Filter,
    pub /: *mut *mut unsigned int sma; / SMA(MDIO) Interface,
    pub /: *mut *mut unsigned int rwk; / PMT remote wake-up packet,
    pub /: *mut *mut unsigned int mgk; / PMT magic packet,
    pub /: *mut *mut unsigned int mmc; / RMON module,
    pub /: *mut *mut unsigned int aoe; / ARP Offload,
    pub /: *mut *mut unsigned int ts; / IEEE 1588-2008 Advanced Timestamp,
    pub /: *mut *mut unsigned int eee; / Energy Efficient Ethernet,
    pub /: *mut *mut unsigned int tx_coe; / Tx Checksum Offload,
    pub /: *mut *mut unsigned int rx_coe; / Rx Checksum Offload,
    pub /: *mut *mut unsigned int addn_mac; / Additional MAC Addresses,
    pub /: *mut *mut unsigned int ts_src; / Timestamp Source,
    pub /: *mut *mut unsigned int sa_vlan_ins; / Source Address or VLAN Insertion,
// HW Feature Register1
    pub /: *mut *mut unsigned int rx_fifo_size; / MTL Receive FIFO Size,
    pub /: *mut *mut unsigned int tx_fifo_size; / MTL Transmit FIFO Size,
    pub /: *mut *mut unsigned int adv_ts_hi; / Advance Timestamping High Word,
    pub /: *mut *mut unsigned int dma_width; / DMA width,
    pub /: *mut *mut unsigned int dcb; / DCB Feature,
    pub /: *mut *mut unsigned int sph; / Split Header Feature,
    pub /: *mut *mut unsigned int tso; / TCP Segmentation Offload,
    pub /: *mut *mut unsigned int dma_debug; / DMA Debug Registers,
    pub /: *mut *mut unsigned int rss; / Receive Side Scaling,
    pub /: *mut *mut unsigned int tc_cnt; / Number of Traffic Classes,
    pub /: *mut *mut unsigned int hash_table_size; / Hash Table Size,
    pub /: *mut *mut unsigned int l3l4_filter_num; / Number of L3-L4 Filters,
// HW Feature Register2
    pub /: *mut *mut unsigned int rx_q_cnt; / Number of MTL Receive Queues,
    pub /: *mut *mut unsigned int tx_q_cnt; / Number of MTL Transmit Queues,
    pub /: *mut *mut unsigned int rx_ch_cnt; / Number of DMA Receive Channels,
    pub /: *mut *mut unsigned int tx_ch_cnt; / Number of DMA Transmit Channels,
    pub /: *mut *mut unsigned int pps_out_num; / Number of PPS outputs,
    pub /: *mut *mut unsigned int aux_snap_num; / Number of Aux snapshot inputs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xlgmac_resources {
    pub addr: *mut void __iomem,
    pub irq: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xlgmac_pdata {
    pub netdev: *mut net_device,
    pub dev: *mut device,
    pub hw_ops: xlgmac_hw_ops,
    pub desc_ops: xlgmac_desc_ops,
// Device statistics
    pub stats: xlgmac_stats,
    pub msg_enable: u32,
// MAC registers base
    pub mac_regs: *mut void __iomem,
// Hardware features of the device
    pub hw_feat: xlgmac_hw_features,
    pub restart_work: work_struct,
// Rings for Tx/Rx on a DMA channel
    pub channel_head: *mut xlgmac_channel,
    pub channel_count: c_uint,
    pub tx_ring_count: c_uint,
    pub rx_ring_count: c_uint,
    pub tx_desc_count: c_uint,
    pub rx_desc_count: c_uint,
    pub tx_q_count: c_uint,
    pub rx_q_count: c_uint,
// Tx/Rx common settings
    pub pblx8: c_uint,
// Tx settings
    pub tx_sf_mode: c_uint,
    pub tx_threshold: c_uint,
    pub tx_pbl: c_uint,
    pub tx_osp_mode: c_uint,
// Rx settings
    pub rx_sf_mode: c_uint,
    pub rx_threshold: c_uint,
    pub rx_pbl: c_uint,
// Tx coalescing settings
    pub tx_usecs: c_uint,
    pub tx_frames: c_uint,
// Rx coalescing settings
    pub rx_riwt: c_uint,
    pub rx_usecs: c_uint,
    pub rx_frames: c_uint,
// Current Rx buffer size
    pub rx_buf_size: c_uint,
// Flow control settings
    pub tx_pause: c_uint,
    pub rx_pause: c_uint,
// Device interrupt number
    pub dev_irq: c_int,
    pub per_channel_irq: c_uint,
    pub channel_irq: [c_int; XLGMAC_MAX_DMA_CHANNELS],
// Netdev related settings
    pub mac_addr: [c_uchar; ETH_ALEN],
    pub netdev_features: netdev_features_t,
    pub napi: napi_struct,
// Filtering support
    pub active_vlans: [c_ulong; BITS_TO_LONGS(VLAN_N_VID)],
// Device clocks
    pub sysclk_rate: c_ulong,
// RSS addressing mutex
    pub rss_mutex: mutex,
// Receive Side Scaling settings
    pub rss_key: [u8; XLGMAC_RSS_HASH_KEY_SIZE],
    pub rss_table: [u32; XLGMAC_RSS_MAX_TABLE_SIZE],
    pub rss_options: u32,
    pub phy_speed: c_int,
    pub drv_name: [c_char; 32],
    pub drv_ver: [c_char; 32],
}

extern "C" {
    pub fn xlgmac_init_desc_ops(desc_ops: *mut xlgmac_desc_ops);
}
extern "C" {
    pub fn xlgmac_init_hw_ops(hw_ops: *mut xlgmac_hw_ops);
}
extern "C" {
    pub fn xlgmac_get_all_hw_features(pdata: *mut xlgmac_pdata);
}
extern "C" {
    pub fn xlgmac_print_all_hw_features(pdata: *mut xlgmac_pdata);
}
extern "C" {
    pub fn xlgmac_drv_remove(dev: *mut device) -> c_int;
}
// For debug prints

