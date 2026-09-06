//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bcm63xx_enet.h
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

// default number of descriptor
pub const BCMENET_DEF_RX_DESC: c_int = 64;
pub const BCMENET_DEF_TX_DESC: c_int = 32;
// maximum burst len for dma (4 bytes unit)
pub const BCMENET_DMA_MAXBURST: c_int = 16;
pub const BCMENETSW_DMA_MAXBURST: c_int = 8;
// tx transmit threshold (4 bytes unit), fifo is 256 bytes, the value
// must be low enough so that a DMA transfer of above burst length can
// not overflow the fifo
pub const BCMENET_TX_FIFO_TRESH: c_int = 32;
//
// hardware maximum rx/tx packet size including FCS, max mtu is
// actually 2047, but if we set max rx size register to 2047 we won't
// get overflow information if packet size is 2048 or above
//
pub const BCMENET_MAX_MTU: c_int = 2046;
//
// MIB Counters register definitions
//
pub const ETH_MIB_TX_GD_OCTETS: c_int = 0;
pub const ETH_MIB_TX_GD_PKTS: c_int = 1;
pub const ETH_MIB_TX_ALL_OCTETS: c_int = 2;
pub const ETH_MIB_TX_ALL_PKTS: c_int = 3;
pub const ETH_MIB_TX_BRDCAST: c_int = 4;
pub const ETH_MIB_TX_MULT: c_int = 5;
pub const ETH_MIB_TX_64: c_int = 6;
pub const ETH_MIB_TX_65_127: c_int = 7;
pub const ETH_MIB_TX_128_255: c_int = 8;
pub const ETH_MIB_TX_256_511: c_int = 9;
pub const ETH_MIB_TX_512_1023: c_int = 10;
pub const ETH_MIB_TX_1024_MAX: c_int = 11;
pub const ETH_MIB_TX_JAB: c_int = 12;
pub const ETH_MIB_TX_OVR: c_int = 13;
pub const ETH_MIB_TX_FRAG: c_int = 14;
pub const ETH_MIB_TX_UNDERRUN: c_int = 15;
pub const ETH_MIB_TX_COL: c_int = 16;
pub const ETH_MIB_TX_1_COL: c_int = 17;
pub const ETH_MIB_TX_M_COL: c_int = 18;
pub const ETH_MIB_TX_EX_COL: c_int = 19;
pub const ETH_MIB_TX_LATE: c_int = 20;
pub const ETH_MIB_TX_DEF: c_int = 21;
pub const ETH_MIB_TX_CRS: c_int = 22;
pub const ETH_MIB_TX_PAUSE: c_int = 23;
pub const ETH_MIB_RX_GD_OCTETS: c_int = 32;
pub const ETH_MIB_RX_GD_PKTS: c_int = 33;
pub const ETH_MIB_RX_ALL_OCTETS: c_int = 34;
pub const ETH_MIB_RX_ALL_PKTS: c_int = 35;
pub const ETH_MIB_RX_BRDCAST: c_int = 36;
pub const ETH_MIB_RX_MULT: c_int = 37;
pub const ETH_MIB_RX_64: c_int = 38;
pub const ETH_MIB_RX_65_127: c_int = 39;
pub const ETH_MIB_RX_128_255: c_int = 40;
pub const ETH_MIB_RX_256_511: c_int = 41;
pub const ETH_MIB_RX_512_1023: c_int = 42;
pub const ETH_MIB_RX_1024_MAX: c_int = 43;
pub const ETH_MIB_RX_JAB: c_int = 44;
pub const ETH_MIB_RX_OVR: c_int = 45;
pub const ETH_MIB_RX_FRAG: c_int = 46;
pub const ETH_MIB_RX_DROP: c_int = 47;
pub const ETH_MIB_RX_CRC_ALIGN: c_int = 48;
pub const ETH_MIB_RX_UND: c_int = 49;
pub const ETH_MIB_RX_CRC: c_int = 50;
pub const ETH_MIB_RX_ALIGN: c_int = 51;
pub const ETH_MIB_RX_SYM: c_int = 52;
pub const ETH_MIB_RX_PAUSE: c_int = 53;
pub const ETH_MIB_RX_CNTRL: c_int = 54;
//
// SW MIB Counters register definitions
//
pub const ETHSW_MIB_TX_ALL_OCT: c_int = 0;
pub const ETHSW_MIB_TX_DROP_PKTS: c_int = 2;
pub const ETHSW_MIB_TX_QOS_PKTS: c_int = 3;
pub const ETHSW_MIB_TX_BRDCAST: c_int = 4;
pub const ETHSW_MIB_TX_MULT: c_int = 5;
pub const ETHSW_MIB_TX_UNI: c_int = 6;
pub const ETHSW_MIB_TX_COL: c_int = 7;
pub const ETHSW_MIB_TX_1_COL: c_int = 8;
pub const ETHSW_MIB_TX_M_COL: c_int = 9;
pub const ETHSW_MIB_TX_DEF: c_int = 10;
pub const ETHSW_MIB_TX_LATE: c_int = 11;
pub const ETHSW_MIB_TX_EX_COL: c_int = 12;
pub const ETHSW_MIB_TX_PAUSE: c_int = 14;
pub const ETHSW_MIB_TX_QOS_OCT: c_int = 15;
pub const ETHSW_MIB_RX_ALL_OCT: c_int = 17;
pub const ETHSW_MIB_RX_UND: c_int = 19;
pub const ETHSW_MIB_RX_PAUSE: c_int = 20;
pub const ETHSW_MIB_RX_64: c_int = 21;
pub const ETHSW_MIB_RX_65_127: c_int = 22;
pub const ETHSW_MIB_RX_128_255: c_int = 23;
pub const ETHSW_MIB_RX_256_511: c_int = 24;
pub const ETHSW_MIB_RX_512_1023: c_int = 25;
pub const ETHSW_MIB_RX_1024_1522: c_int = 26;
pub const ETHSW_MIB_RX_OVR: c_int = 27;
pub const ETHSW_MIB_RX_JAB: c_int = 28;
pub const ETHSW_MIB_RX_ALIGN: c_int = 29;
pub const ETHSW_MIB_RX_CRC: c_int = 30;
pub const ETHSW_MIB_RX_GD_OCT: c_int = 31;
pub const ETHSW_MIB_RX_DROP: c_int = 33;
pub const ETHSW_MIB_RX_UNI: c_int = 34;
pub const ETHSW_MIB_RX_MULT: c_int = 35;
pub const ETHSW_MIB_RX_BRDCAST: c_int = 36;
pub const ETHSW_MIB_RX_SA_CHANGE: c_int = 37;
pub const ETHSW_MIB_RX_FRAG: c_int = 38;
pub const ETHSW_MIB_RX_OVR_DISC: c_int = 39;
pub const ETHSW_MIB_RX_SYM: c_int = 40;
pub const ETHSW_MIB_RX_QOS_PKTS: c_int = 41;
pub const ETHSW_MIB_RX_QOS_OCT: c_int = 42;
pub const ETHSW_MIB_RX_1523_2047: c_int = 44;
pub const ETHSW_MIB_RX_2048_4095: c_int = 45;
pub const ETHSW_MIB_RX_4096_8191: c_int = 46;
pub const ETHSW_MIB_RX_8192_9728: c_int = 47;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_enet_mib_counters {
    pub tx_gd_octets: u64,
    pub tx_gd_pkts: u32,
    pub tx_all_octets: u32,
    pub tx_all_pkts: u32,
    pub tx_unicast: u32,
    pub tx_brdcast: u32,
    pub tx_mult: u32,
    pub tx_64: u32,
    pub tx_65_127: u32,
    pub tx_128_255: u32,
    pub tx_256_511: u32,
    pub tx_512_1023: u32,
    pub tx_1024_max: u32,
    pub tx_1523_2047: u32,
    pub tx_2048_4095: u32,
    pub tx_4096_8191: u32,
    pub tx_8192_9728: u32,
    pub tx_jab: u32,
    pub tx_drop: u32,
    pub tx_ovr: u32,
    pub tx_frag: u32,
    pub tx_underrun: u32,
    pub tx_col: u32,
    pub tx_1_col: u32,
    pub tx_m_col: u32,
    pub tx_ex_col: u32,
    pub tx_late: u32,
    pub tx_def: u32,
    pub tx_crs: u32,
    pub tx_pause: u32,
    pub rx_gd_octets: u64,
    pub rx_gd_pkts: u32,
    pub rx_all_octets: u32,
    pub rx_all_pkts: u32,
    pub rx_brdcast: u32,
    pub rx_unicast: u32,
    pub rx_mult: u32,
    pub rx_64: u32,
    pub rx_65_127: u32,
    pub rx_128_255: u32,
    pub rx_256_511: u32,
    pub rx_512_1023: u32,
    pub rx_1024_max: u32,
    pub rx_jab: u32,
    pub rx_ovr: u32,
    pub rx_frag: u32,
    pub rx_drop: u32,
    pub rx_crc_align: u32,
    pub rx_und: u32,
    pub rx_crc: u32,
    pub rx_align: u32,
    pub rx_sym: u32,
    pub rx_pause: u32,
    pub rx_cntrl: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_enet_priv {
// base remapped address of device
    pub base: *mut void __iomem,
// mac irq, rx_dma irq, tx_dma irq
    pub irq: c_int,
    pub irq_rx: c_int,
    pub irq_tx: c_int,
// hw view of rx & tx dma ring
    pub rx_desc_dma: dma_addr_t,
    pub tx_desc_dma: dma_addr_t,
// allocated size (in bytes) for rx & tx dma ring
    pub rx_desc_alloc_size: c_uint,
    pub tx_desc_alloc_size: c_uint,
    pub napi: napi_struct,
// dma channel id for rx
    pub rx_chan: c_int,
// number of dma desc in rx ring
    pub rx_ring_size: c_int,
// cpu view of rx dma ring
    pub rx_desc_cpu: *mut bcm_enet_desc,
// current number of armed descriptor given to hardware for rx
    pub rx_desc_count: c_int,
// next rx descriptor to fetch from hardware
    pub rx_curr_desc: c_int,
// next dirty rx descriptor to refill
    pub rx_dirty_desc: c_int,
// size of allocated rx buffers
    pub rx_buf_size: c_uint,
// allocated rx buffer offset
    pub rx_buf_offset: c_uint,
// size of allocated rx frag
    pub rx_frag_size: c_uint,
// list of buffer given to hw for rx
    pub rx_buf: *mut c_void,
// used when rx skb allocation failed, so we defer rx queue
// refill
    pub rx_timeout: timer_list,
// lock rx_timeout against rx normal operation
    pub rx_lock: spinlock_t,
// dma channel id for tx
    pub tx_chan: c_int,
// number of dma desc in tx ring
    pub tx_ring_size: c_int,
// maximum dma burst size
    pub dma_maxburst: c_int,
// cpu view of rx dma ring
    pub tx_desc_cpu: *mut bcm_enet_desc,
// number of available descriptor for tx
    pub tx_desc_count: c_int,
// next tx descriptor avaiable
    pub tx_curr_desc: c_int,
// next dirty tx descriptor to reclaim
    pub tx_dirty_desc: c_int,
// list of skb given to hw for tx
    pub tx_skb: *mut sk_buff,
// lock used by tx reclaim and xmit
    pub tx_lock: spinlock_t,
// set if internal phy is ignored and external mii interface
// is selected
    pub use_external_mii: c_int,
// set if a phy is connected, phy address must be known,
// probing is not possible
    pub has_phy: c_int,
    pub phy_id: c_int,
// set if connected phy has an associated irq
    pub has_phy_interrupt: c_int,
    pub phy_interrupt: c_int,
// used when a phy is connected (phylib used)
    pub mii_bus: *mut mii_bus,
    pub old_link: c_int,
    pub old_duplex: c_int,
    pub old_pause: c_int,
// used when no phy is connected
    pub force_speed_100: c_int,
    pub force_duplex_full: c_int,
// pause parameters
    pub pause_auto: c_int,
    pub pause_rx: c_int,
    pub pause_tx: c_int,
// stats
    pub mib: bcm_enet_mib_counters,
// after mib interrupt, mib registers update is done in this
// work queue
    pub mib_update_task: work_struct,
// lock mib update between userspace request and workqueue
    pub mib_update_lock: mutex,
// mac clock
    pub mac_clk: *mut clk,
// phy clock if internal phy is used
    pub phy_clk: *mut clk,
// network device reference
    pub net_dev: *mut net_device,
// platform device reference
    pub pdev: *mut platform_device,
// maximum hardware transmit/receive size
    pub hw_mtu: c_uint,
    pub enet_is_sw: bool,
// port mapping for switch devices
    pub num_ports: c_int,
    pub used_ports: [bcm63xx_enetsw_port; ENETSW_MAX_PORT],
    pub sw_port_link: [c_int; ENETSW_MAX_PORT],
// used to poll switch port state
    pub swphy_poll: timer_list,
    pub enetsw_mdio_lock: spinlock_t,
// dma channel enable mask
    pub dma_chan_en_mask: u32,
// dma channel interrupt mask
    pub dma_chan_int_mask: u32,
// DMA engine has internal SRAM
    pub dma_has_sram: bool,
// dma channel width
    pub dma_chan_width: c_uint,
// dma descriptor shift value
    pub dma_desc_shift: c_uint,
}
