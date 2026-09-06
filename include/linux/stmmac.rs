//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/stmmac.h
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

pub const MTL_MAX_RX_QUEUES: c_int = 8;
pub const MTL_MAX_TX_QUEUES: c_int = 8;
pub const STMMAC_CH_MAX: c_int = 8;
pub const STMMAC_RX_COE_NONE: c_int = 0;
pub const STMMAC_RX_COE_TYPE1: c_int = 1;
pub const STMMAC_RX_COE_TYPE2: c_int = 2;
// Define the macros for CSR clock range parameters to be passed by
// platform code.
// This could also be configured at run time using CPU freq framework.
// MDC Clock Selection define
pub const STMMAC_CSR_60_100M: c_uint = 0x0	/* MDC = clk_csr_i/42 */;
pub const STMMAC_CSR_100_150M: c_uint = 0x1	/* MDC = clk_csr_i/62 */;
pub const STMMAC_CSR_20_35M: c_uint = 0x2	/* MDC = clk_csr_i/16 */;
pub const STMMAC_CSR_35_60M: c_uint = 0x3	/* MDC = clk_csr_i/26 */;
pub const STMMAC_CSR_150_250M: c_uint = 0x4	/* MDC = clk_csr_i/102 */;
pub const STMMAC_CSR_250_300M: c_uint = 0x5	/* MDC = clk_csr_i/124 */;
pub const STMMAC_CSR_300_500M: c_uint = 0x6	/* MDC = clk_csr_i/204 */;
pub const STMMAC_CSR_500_800M: c_uint = 0x7	/* MDC = clk_csr_i/324 */;
// MTL algorithms identifiers
pub const MTL_TX_ALGORITHM_WRR: c_uint = 0x0;
pub const MTL_TX_ALGORITHM_WFQ: c_uint = 0x1;
pub const MTL_TX_ALGORITHM_DWRR: c_uint = 0x2;
pub const MTL_TX_ALGORITHM_SP: c_uint = 0x3;
pub const MTL_RX_ALGORITHM_SP: c_uint = 0x4;
pub const MTL_RX_ALGORITHM_WSP: c_uint = 0x5;
// RX/TX Queue Mode
pub const MTL_QUEUE_AVB: c_uint = 0x0;
pub const MTL_QUEUE_DCB: c_uint = 0x1;
// The MDC clock could be set higher than the IEEE 802.3
// specified frequency limit 0f 2.5 MHz, by programming a clock divider
// of value different than the above defined values. The resultant MDIO
// clock frequency of 12.5 MHz is applicable for the interfacing chips
// supporting higher MDC clocks.
// The MDC clock selection macros need to be defined for MDC clock rate
// of 12.5 MHz, corresponding to the following selection.
//
pub const STMMAC_CSR_I_4: c_uint = 0x8	/* clk_csr_i/4 */;
pub const STMMAC_CSR_I_6: c_uint = 0x9	/* clk_csr_i/6 */;
pub const STMMAC_CSR_I_8: c_uint = 0xA	/* clk_csr_i/8 */;
pub const STMMAC_CSR_I_10: c_uint = 0xB	/* clk_csr_i/10 */;
pub const STMMAC_CSR_I_12: c_uint = 0xC	/* clk_csr_i/12 */;
pub const STMMAC_CSR_I_14: c_uint = 0xD	/* clk_csr_i/14 */;
pub const STMMAC_CSR_I_16: c_uint = 0xE	/* clk_csr_i/16 */;
pub const STMMAC_CSR_I_18: c_uint = 0xF	/* clk_csr_i/18 */;
// AXI DMA Burst length supported

// Platfrom data for platform device structure's platform_data field
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_mdio_bus_data {
    pub phy_mask: u32,
    pub pcs_mask: u32,
    pub irqs: *mut c_int,
    pub probed_phy_irq: c_int,
    pub needs_reset: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_dma_cfg {
// pbl: programmable burst limit
// txpbl: transmit programmable burst limit
// rxpbl: receive programmable burst limit
// If txpbl or rxpbl are zero, the value of pbl will be substituted.
// Range 0 - 63.
//
    pub pbl: c_int,
    pub txpbl: c_int,
    pub rxpbl: c_int,
// pblx8: multiplies pbl, txpbl, rxpbl by a factor of 8 for dwmac >=
// 3.50a, or a factor of 4 for previous versions.
//
    pub pblx8: bool,
// fixed_burst:
// when set, AXI bursts defined by axi_blen_regval are permitted.
// AHB uses SINGLE, INCR4, INCR8 or INCR16 during burst transfers.
// when clear, AXI and AHB use SINGLE or INCR bursts.
//
    pub fixed_burst: bool,
// mixed_burst:
// when set and fixed_burst is clear, AHB uses INCR for bursts > 16
// and SINGLE or INCRx for bursts <= 16.
//
    pub mixed_burst: bool,
// aal: address aligned bursts for AHB and AXI master interface
    pub aal: bool,
    pub dche: bool,
    pub eame: bool,
// multi_msi_en: stmmac core internal
    pub multi_msi_en: bool,
// atds: stmmac core internal
    pub atds: bool,
}

pub const AXI_BLEN: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_axi {
    pub axi_wr_osr_lmt: u32,
    pub axi_rd_osr_lmt: u32,
    pub axi_blen_regval: u32,
    pub axi_lpi_en: bool,
    pub axi_xit_frm: bool,
    pub axi_fb: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_rxq_cfg {
    pub chan: u32,
    pub prio: u32,
    pub mode_to_use: u8,
    pub pkt_route: u8,
    pub use_prio: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_txq_cfg {
    pub weight: u32,
// Credit Base Shaper parameters
    pub send_slope: u32,
    pub idle_slope: u32,
    pub high_credit: u32,
    pub low_credit: u32,
    pub prio: u32,
    pub tbs_en: c_int,
    pub use_prio: bool,
    pub coe_unsupported: bool,
    pub mode_to_use: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_safety_feature_cfg {
    pub tsoee: u32,
    pub mrxpee: u32,
    pub mestee: u32,
    pub mrxee: u32,
    pub mtxee: u32,
    pub epsi: u32,
    pub edpp: u32,
    pub prtyen: u32,
    pub tmouten: u32,
}

// Addresses that may be customized by a platform
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwmac4_addrs {
    pub dma_chan: u32,
    pub dma_chan_offset: u32,
    pub mtl_chan: u32,
    pub mtl_chan_offset: u32,
    pub mtl_ets_ctrl: u32,
    pub mtl_ets_ctrl_offset: u32,
    pub mtl_txq_weight: u32,
    pub mtl_txq_weight_offset: u32,
    pub mtl_send_slp_cred: u32,
    pub mtl_send_slp_cred_offset: u32,
    pub mtl_high_cred: u32,
    pub mtl_high_cred_offset: u32,
    pub mtl_low_cred: u32,
    pub mtl_low_cred_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dwmac_core_type {
    DWMAC_CORE_MAC100,
    DWMAC_CORE_GMAC,
    DWMAC_CORE_GMAC4,
    DWMAC_CORE_XGMAC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct plat_stmmacenet_data {
    pub core_type: dwmac_core_type,
    pub bus_id: c_int,
    pub phy_addr: c_int,
// MAC ----- optional PCS ----- SerDes ----- optional PHY ----- Media
// ^
// phy_interface
//
// The Synopsys dwmac core only covers the MAC and an optional
// integrated PCS. Where the integrated PCS is used with a SerDes,
// e.g. for 1000base-X or Cisco SGMII, the connection between the
// PCS and SerDes will be TBI.
//
// Where the Synopsys dwmac core has been instantiated with multiple
// interface modes, these are selected via core-external configuration
// which is sampled when the dwmac core is reset. How this is done is
// platform glue specific, but this defines the interface used from
// the Synopsys dwmac core to the rest of the SoC.
//
// Where PCS other than the optional integrated Synopsys dwmac PCS
// is used, this counts as "the rest of the SoC" in the above
// paragraph.
//
// phy_interface is the PHY-side interface - the interface used by
// an attached PHY or SFP etc. This is equivalent to the interface
// that phylink uses.
//
    pub phy_interface: phy_interface_t,
    pub mdio_bus_data: *mut stmmac_mdio_bus_data,
    pub phy_node: *mut device_node,
    pub mdio_node: *mut device_node,
    pub dma_cfg: *mut stmmac_dma_cfg,
    pub safety_feat_cfg: *mut stmmac_safety_feature_cfg,
    pub clk_csr: c_int,
    pub default_an_inband: bool,
    pub enh_desc: bool,
    pub tx_coe: bool,
    pub bugged_jumbo: bool,
    pub pmt: bool,
    pub force_sf_dma_mode: bool,
    pub force_thresh_dma_mode: bool,
    pub riwt_off: bool,
    pub rx_coe: c_int,
    pub max_speed: c_int,
    pub maxmtu: c_int,
    pub multicast_filter_bins: c_int,
    pub unicast_filter_entries: c_int,
    pub tx_fifo_size: c_int,
    pub rx_fifo_size: c_int,
    pub host_dma_width: u8,
    pub rx_queues_to_use: u8,
    pub tx_queues_to_use: u8,
    pub rx_sched_algorithm: u8,
    pub tx_sched_algorithm: u8,
    pub rx_queues_cfg: [stmmac_rxq_cfg; MTL_MAX_RX_QUEUES],
    pub tx_queues_cfg: [stmmac_txq_cfg; MTL_MAX_TX_QUEUES],
    pub interfaces): *mut c_ulong,
    pub phy_intf_sel): *mut *mut *mut int (set_phy_intf_sel)(void priv, u8,
    pub speed): phy_interface_t interface, int,
    pub mode): int speed, unsigned int,
    pub priv): *mut *mut int (fix_soc_reset)(struct stmmac_priv,
    pub priv): *mut *mut *mut int (serdes_powerup)(struct net_device ndev, void,
    pub priv): *mut *mut *mut void (serdes_powerdown)(struct net_device ndev, void,
    pub interface): phy_interface_t,
    pub priv): *mut *mut void (ptp_clk_freq_config)(struct stmmac_priv,
    pub priv): *mut *mut *mut int (init)(struct device dev, void,
    pub priv): *mut *mut *mut void (exit)(struct device dev, void,
    pub priv): *mut *mut *mut int (suspend)(struct device dev, void,
    pub priv): *mut *mut *mut int (resume)(struct device dev, void,
    pub mac): *mut *mut *mut int (mac_setup)(void priv, struct mac_device_info,
    pub enabled): *mut *mut *mut int (clks_config)(void priv, bool,
    pub ctx): *mut c_void,
    pub priv): *mut *mut void (dump_debug_regs)(void,
    pub priv): *mut *mut int (pcs_init)(struct stmmac_priv,
    pub priv): *mut *mut void (pcs_exit)(struct stmmac_priv,
    pub interface): phy_interface_t,
    pub bsp_priv: *mut c_void,
// stmmac clocks:
// stmmac_clk: CSR clock (which can be hclk_i, clk_csr_i, aclk_i,
// or clk_app_i depending on GMAC configuration). This clock
// generates the MDC clock.
//
// pclk: introduced for Imagination Technologies Pistachio board -
// see 5f9755d26fbf ("stmmac: Add an optional register interface
// clock"). This is probably used for cases where separate clocks
// are provided for the host interface and register interface. In
// this case, as the MDC clock is derived from stmmac_clk, pclk
// can only really be the "application clock" for the "host
// interface" and not the "register interface" aka CSR clock as
// it is never used when determining the divider for the MDC
// clock.
//
// clk_ptp_ref: optional PTP reference clock (clk_ptp_ref_i). When
// present, this clock increments the timestamp value. Otherwise,
// the rate of stmmac_clk will be used.
//
// clk_tx_i: MAC transmit clock, which will be 2.5MHz for 10M,
// 25MHz for 100M, or 125MHz for 1G irrespective of the interface
// mode. For the DWMAC PHY interface modes:
//
// GMII/MII	PHY's transmit clock for 10M (2.5MHz) or 100M (25MHz),
// or 125MHz local clock for 1G mode
// RMII	50MHz RMII clock divided by 2 or 20.
// RGMII	125MHz local clock divided by 1, 5, or 50.
// SGMII	125MHz SerDes clock divided by 1, 5, or 50.
// TBI/RTBI	125MHz SerDes clock
//
    pub stmmac_clk: *mut clk,
    pub pclk: *mut clk,
    pub clk_ptp_ref: *mut clk,
    pub clk_tx_i: *mut clk,
    pub clk_ptp_rate: c_ulong,
    pub clk_ref_rate: c_ulong,
    pub clks: *mut clk_bulk_data,
    pub num_clks: c_int,
    pub mult_fact_100ns: c_uint,
    pub ptp_max_adj: i32,
    pub cdc_error_adj: u32,
    pub stmmac_rst: *mut reset_control,
    pub stmmac_ahb_rst: *mut reset_control,
    pub axi: *mut stmmac_axi,
    pub rss_en: c_int,
    pub mac_port_sel_speed: c_int,
    pub vlan_fail_q: u8,
    pub provide_bus_info: bool,
    pub int_snapshot_num: c_int,
    pub msi_mac_vec: c_int,
    pub msi_wol_vec: c_int,
    pub msi_sfty_ce_vec: c_int,
    pub msi_sfty_ue_vec: c_int,
    pub msi_rx_base_vec: c_int,
    pub msi_tx_base_vec: c_int,
    pub dwmac4_addrs: *const dwmac4_addrs,
    pub flags: c_uint,
    pub __dma_cfg: stmmac_dma_cfg,
}
