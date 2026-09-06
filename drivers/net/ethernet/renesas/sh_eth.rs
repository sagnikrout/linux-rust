//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/renesas/sh_eth.h
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
// SuperH Ethernet device driver
//
// Copyright (C) 2006-2012 Nobuhiro Iwamatsu
// Copyright (C) 2008-2012 Renesas Solutions Corp.
//

pub const TX_RING_MIN: c_int = 64;
pub const RX_RING_MIN: c_int = 64;
pub const TX_RING_MAX: c_int = 1024;
pub const RX_RING_MAX: c_int = 1024;
pub const PKT_BUF_SZ: c_int = 1538;
pub const SH_ETH_TSU_TIMEOUT_MS: c_int = 500;
pub const SH_ETH_TSU_CAM_ENTRIES: c_int = 32;
// IMPORTANT: To keep ethtool register dump working, add new
// register names immediately before SH_ETH_MAX_REGISTER_OFFSET.
//
// E-DMAC registers
// Ether registers
// TSU Absolute address
// TSU_ADR{H,L}{0..31} are assumed to be contiguous
// This value must be written at last.
// Driver's parameters

pub const SH_ETH_RX_ALIGN: c_int = 32;

pub const SH_ETH_RX_ALIGN: c_int = 2;

// Register's bits
//
// EDSR : sh7734, sh7757, sh7763, r8a7740, and r7s72100 only
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum EDSR_BIT {
    EDSR_ENT = 0x01, EDSR_ENR = 0x02,
}

// GECMR : sh7734, sh7763 and r8a7740 only
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GECMR_BIT {
    GECMR_10 = 0x0, GECMR_100 = 0x04, GECMR_1000 = 0x01,
}

// EDMR
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum EDMR_BIT {
    EDMR_NBST = 0x80,
    EDMR_EL = 0x40, /* Litte endian */
    EDMR_DL1 = 0x20, EDMR_DL0 = 0x10,
    EDMR_SRST_GETHER = 0x03,
    EDMR_SRST_ETHER = 0x01,
}

// EDTRR
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum EDTRR_BIT {
    EDTRR_TRNS_GETHER = 0x03,
    EDTRR_TRNS_ETHER = 0x01,
}

// EDRRR
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum EDRRR_BIT {
    EDRRR_R = 0x01,
}

// TPAUSER
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TPAUSER_BIT {
    TPAUSER_TPAUSE = 0x0000ffff,
    TPAUSER_UNLIMITED = 0,
}

// BCFR
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum BCFR_BIT {
    BCFR_RPAUSE = 0x0000ffff,
    BCFR_UNLIMITED = 0,
}

// PIR
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PIR_BIT {
    PIR_MDI = 0x08, PIR_MDO = 0x04, PIR_MMD = 0x02, PIR_MDC = 0x01,
}

// PSR
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PSR_BIT {

// EESR
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum EESR_BIT {
    EESR_TWB1	= 0x80000000,
    EESR_TWB	= 0x40000000,	/* same as TWB0 */
    EESR_TC1	= 0x20000000,
    EESR_TUC	= 0x10000000,
    EESR_ROC	= 0x08000000,
    EESR_TABT	= 0x04000000,
    EESR_RABT	= 0x02000000,
    EESR_RFRMER	= 0x01000000,	/* same as RFCOF */
    EESR_ADE	= 0x00800000,
    EESR_ECI	= 0x00400000,
    EESR_FTC	= 0x00200000,	/* same as TC or TC0 */
    EESR_TDE	= 0x00100000,
    EESR_TFE	= 0x00080000,	/* same as TFUF */
    EESR_FRC	= 0x00040000,	/* same as FR */
    EESR_RDE	= 0x00020000,
    EESR_RFE	= 0x00010000,
    EESR_CND	= 0x00000800,
    EESR_DLC	= 0x00000400,
    EESR_CD		= 0x00000200,
    EESR_TRO	= 0x00000100,
    EESR_RMAF	= 0x00000080,
    EESR_CEEF	= 0x00000040,
    EESR_CELF	= 0x00000020,
    EESR_RRF	= 0x00000010,
    EESR_RTLF	= 0x00000008,
    EESR_RTSF	= 0x00000004,
    EESR_PRE	= 0x00000002,
    EESR_CERF	= 0x00000001,
}

// EESIPR
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum EESIPR_BIT {
    EESIPR_TWB1IP	= 0x80000000,
    EESIPR_TWBIP	= 0x40000000,	/* same as TWB0IP */
    EESIPR_TC1IP	= 0x20000000,
    EESIPR_TUCIP	= 0x10000000,
    EESIPR_ROCIP	= 0x08000000,
    EESIPR_TABTIP	= 0x04000000,
    EESIPR_RABTIP	= 0x02000000,
    EESIPR_RFCOFIP	= 0x01000000,
    EESIPR_ADEIP	= 0x00800000,
    EESIPR_ECIIP	= 0x00400000,
    EESIPR_FTCIP	= 0x00200000,	/* same as TC0IP */
    EESIPR_TDEIP	= 0x00100000,
    EESIPR_TFUFIP	= 0x00080000,
    EESIPR_FRIP	= 0x00040000,
    EESIPR_RDEIP	= 0x00020000,
    EESIPR_RFOFIP	= 0x00010000,
    EESIPR_CNDIP	= 0x00000800,
    EESIPR_DLCIP	= 0x00000400,
    EESIPR_CDIP	= 0x00000200,
    EESIPR_TROIP	= 0x00000100,
    EESIPR_RMAFIP	= 0x00000080,
    EESIPR_CEEFIP	= 0x00000040,
    EESIPR_CELFIP	= 0x00000020,
    EESIPR_RRFIP	= 0x00000010,
    EESIPR_RTLFIP	= 0x00000008,
    EESIPR_RTSFIP	= 0x00000004,
    EESIPR_PREIP	= 0x00000002,
    EESIPR_CERFIP	= 0x00000001,
}

// FCFTR
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum FCFTR_BIT {
    FCFTR_RFF2 = 0x00040000, FCFTR_RFF1 = 0x00020000,
    FCFTR_RFF0 = 0x00010000, FCFTR_RFD2 = 0x00000004,
    FCFTR_RFD1 = 0x00000002, FCFTR_RFD0 = 0x00000001,
}

// RMCR
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RMCR_BIT {
    RMCR_RNC = 0x00000001,
}

// ECMR
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ECMR_BIT {
    ECMR_TRCCM = 0x04000000, ECMR_RCSC = 0x00800000,
    ECMR_DPAD = 0x00200000, ECMR_RZPF = 0x00100000,
    ECMR_ZPF = 0x00080000, ECMR_PFR = 0x00040000, ECMR_RXF = 0x00020000,
    ECMR_TXF = 0x00010000, ECMR_MCT = 0x00002000, ECMR_PRCEF = 0x00001000,
    ECMR_MPDE = 0x00000200, ECMR_RE = 0x00000040, ECMR_TE = 0x00000020,
    ECMR_RTM = 0x00000010, ECMR_ILB = 0x00000008, ECMR_ELB = 0x00000004,
    ECMR_DM = 0x00000002, ECMR_PRM = 0x00000001,
}

// ECSR
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ECSR_BIT {
    ECSR_BRCRX = 0x20, ECSR_PSRTO = 0x10,
    ECSR_LCHNG = 0x04,
    ECSR_MPD = 0x02, ECSR_ICD = 0x01,
}

// ECSIPR
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ECSIPR_BIT {
    ECSIPR_BRCRXIP = 0x20, ECSIPR_PSRTOIP = 0x10,
    ECSIPR_LCHNGIP = 0x04,
    ECSIPR_MPDIP = 0x02, ECSIPR_ICDIP = 0x01,
}

// APR
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum APR_BIT {
    APR_AP = 0x0000ffff,
}

// MPR
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MPR_BIT {
    MPR_MP = 0x0000ffff,
}

// TRSCER
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TRSCER_BIT {
    TRSCER_CNDCE	= 0x00000800,
    TRSCER_DLCCE	= 0x00000400,
    TRSCER_CDCE	= 0x00000200,
    TRSCER_TROCE	= 0x00000100,
    TRSCER_RMAFCE	= 0x00000080,
    TRSCER_RRFCE	= 0x00000010,
    TRSCER_RTLFCE	= 0x00000008,
    TRSCER_RTSFCE	= 0x00000004,
    TRSCER_PRECE	= 0x00000002,
    TRSCER_CERFCE	= 0x00000001,
}

// RPADIR
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RPADIR_BIT {
    RPADIR_PADS = 0x1f0000, RPADIR_PADR = 0xffff,
}

// FDR
pub const DEFAULT_FDR_INIT: c_uint = 0x00000707;
// ARSTR
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ARSTR_BIT {

// TSU_FWEN0
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TSU_FWEN0_BIT {
    TSU_FWEN0_0 = 0x00000001,
}

// TSU_ADSBSY
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TSU_ADSBSY_BIT {
    TSU_ADSBSY_0 = 0x00000001,
}

// TSU_TEN
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TSU_TEN_BIT {
    TSU_TEN_0 = 0x80000000,
}

// TSU_FWSL0
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TSU_FWSL0_BIT {
    TSU_FWSL0_FW50 = 0x1000, TSU_FWSL0_FW40 = 0x0800,
    TSU_FWSL0_FW30 = 0x0400, TSU_FWSL0_FW20 = 0x0200,
    TSU_FWSL0_FW10 = 0x0100, TSU_FWSL0_RMSA0 = 0x0010,
}

// TSU_FWSLC
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TSU_FWSLC_BIT {
    TSU_FWSLC_POSTENU = 0x2000, TSU_FWSLC_POSTENL = 0x1000,
    TSU_FWSLC_CAMSEL03 = 0x0080, TSU_FWSLC_CAMSEL02 = 0x0040,
    TSU_FWSLC_CAMSEL01 = 0x0020, TSU_FWSLC_CAMSEL00 = 0x0010,
    TSU_FWSLC_CAMSEL13 = 0x0008, TSU_FWSLC_CAMSEL12 = 0x0004,
    TSU_FWSLC_CAMSEL11 = 0x0002, TSU_FWSLC_CAMSEL10 = 0x0001,
}

// TSU_VTAGn
pub const TSU_VTAG_ENABLE: c_uint = 0x80000000;
pub const TSU_VTAG_VID_MASK: c_uint = 0x00000fff;
// The sh ether Tx buffer descriptors.
// This structure should be 20 bytes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_eth_txdesc {
    pub /: *mut *mut u32 status; / TD0,
    pub /: *mut *mut u32 len; / TD1,
    pub /: *mut *mut u32 addr; / TD2,
    pub /: *mut *mut u32 pad0; / padding data,
    pub __packed: } __aligned(2),
// Transmit descriptor 0 bits
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TD_STS_BIT {
    TD_TACT	= 0x80000000,
    TD_TDLE	= 0x40000000,
    TD_TFP1	= 0x20000000,
    TD_TFP0	= 0x10000000,
    TD_TFE	= 0x08000000,
    TD_TWBI	= 0x04000000,
}

// Transmit descriptor 1 bits
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TD_LEN_BIT {
    TD_TBL	= 0xffff0000,	/* transmit buffer length */
}

// The sh ether Rx buffer descriptors.
// This structure should be 20 bytes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_eth_rxdesc {
    pub /: *mut *mut u32 status; / RD0,
    pub /: *mut *mut u32 len; / RD1,
    pub /: *mut *mut u32 addr; / RD2,
    pub /: *mut *mut u32 pad0; / padding data,
    pub __packed: } __aligned(2),
// Receive descriptor 0 bits
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RD_STS_BIT {
    RD_RACT	= 0x80000000,
    RD_RDLE	= 0x40000000,
    RD_RFP1	= 0x20000000,
    RD_RFP0	= 0x10000000,
    RD_RFE	= 0x08000000,
    RD_RFS10 = 0x00000200,
    RD_RFS9	= 0x00000100,
    RD_RFS8	= 0x00000080,
    RD_RFS7	= 0x00000040,
    RD_RFS6	= 0x00000020,
    RD_RFS5	= 0x00000010,
    RD_RFS4	= 0x00000008,
    RD_RFS3	= 0x00000004,
    RD_RFS2	= 0x00000002,
    RD_RFS1	= 0x00000001,
}

// Receive descriptor 1 bits
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RD_LEN_BIT {
    RD_RFL	= 0x0000ffff,	/* receive frame  length */
    RD_RBL	= 0xffff0000,	/* receive buffer length */
}

// This structure is used by each CPU dependency handling.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_eth_cpu_data {
// mandatory functions
    pub ndev): *mut *mut int (soft_reset)(struct net_device,
// optional functions
    pub ndev): *mut *mut void (chip_reset)(struct net_device,
    pub ndev): *mut *mut void (set_duplex)(struct net_device,
    pub ndev): *mut *mut void (set_rate)(struct net_device,
// mandatory initialize value
    pub register_type: c_int,
    pub edtrr_trns: u32,
    pub eesipr_value: u32,
// optional initialize value
    pub ecsr_value: u32,
    pub ecsipr_value: u32,
    pub fdr_value: u32,
    pub fcftr_value: u32,
// interrupt checking mask
    pub tx_check: u32,
    pub eesr_err_check: u32,
// Error mask
    pub trscer_err_mask: u32,
// hardware features
    pub /: *mut *mut unsigned long irq_flags; / IRQ configuration flags,
    pub /: *mut *mut unsigned no_psr:1; / EtherC DOES NOT have PSR,
    pub /: *mut *mut unsigned apr:1; / EtherC has APR,
    pub /: *mut *mut unsigned mpr:1; / EtherC has MPR,
    pub /: *mut *mut unsigned tpauser:1; / EtherC has TPAUSER,
    pub /: *mut *mut unsigned gecmr:1; / EtherC has GECMR,
    pub /: *mut *mut unsigned bculr:1; / EtherC has BCULR,
    pub /: *mut *mut unsigned tsu:1; / EtherC has TSU,
    pub /: *mut *mut unsigned hw_swap:1; / E-DMAC has DE bit in EDMR,
    pub /: *mut *mut unsigned nbst:1; / E-DMAC has NBST bit in EDMR,
    pub /: *mut *mut unsigned rpadir:1; / E-DMAC has RPADIR,
    pub /: *mut *mut unsigned no_trimd:1; / E-DMAC DOES NOT have TRIMD,
    pub /: *mut *mut unsigned no_ade:1; / E-DMAC DOES NOT have ADE bit in EESR,
    pub /: *mut *mut unsigned no_xdfar:1; / E-DMAC DOES NOT have RDFAR/TDFAR,
    pub /: *mut *mut unsigned xdfar_rw:1; / E-DMAC has writeable RDFAR/TDFAR,
    pub /: *mut *mut unsigned csmr:1; / E-DMAC has CSMR,
    pub /: *mut *mut unsigned rx_csum:1; / EtherC has ECMR.RCSC,
    pub /: *mut *mut unsigned select_mii:1; / EtherC has RMII_MII (MII select register),
    pub /: *mut *mut unsigned rmiimode:1; / EtherC has RMIIMODE register,
    pub /: *mut *mut unsigned rtrate:1; / EtherC has RTRATE register,
    pub /: *mut *mut unsigned magic:1; / EtherC has ECMR.MPDE and ECSR.MPD,
    pub /: *mut *mut unsigned no_tx_cntrs:1; / EtherC DOES NOT have TX error counters,
    pub /: *mut *mut unsigned cexcr:1; / EtherC has CERCR/CEECR,
    pub /: *mut *mut unsigned dual_port:1; / Dual EtherC/E-DMAC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_eth_private {
    pub pdev: *mut platform_device,
    pub cd: *mut sh_eth_cpu_data,
    pub reg_offset: *const u16,
    pub addr: *mut void __iomem,
    pub tsu_addr: *mut void __iomem,
    pub clk: *mut clk,
    pub num_rx_ring: u32,
    pub num_tx_ring: u32,
    pub rx_desc_dma: dma_addr_t,
    pub tx_desc_dma: dma_addr_t,
    pub rx_ring: *mut sh_eth_rxdesc,
    pub tx_ring: *mut sh_eth_txdesc,
    pub rx_skbuff: *mut sk_buff,
    pub tx_skbuff: *mut sk_buff,
    pub /: *mut *mut spinlock_t lock; / Register access lock,
    pub /: *mut *mut u32 cur_rx, dirty_rx; / Producer/consumer ring indices,
    pub dirty_tx: u32 cur_tx,,
    pub /: *mut *mut u32 rx_buf_sz; / Based on MTU+slack.,
    pub napi: napi_struct,
    pub irq_enabled: bool,
// MII transceiver section.
    pub /: *mut *mut u32 phy_id; / PHY ID,
    pub /: *mut *mut *mut mii_bus mii_bus; / MDIO bus control,
    pub link: c_int,
    pub phy_interface: phy_interface_t,
    pub msg_enable: c_int,
    pub speed: c_int,
    pub duplex: c_int,
    pub /: *mut *mut int port; / for TSU,
    pub /: *mut *mut int vlan_num_ids; / for VLAN tag filter,
    pub no_ether_link:1: unsigned,
    pub ether_link_active_low:1: unsigned,
    pub is_opened:1: unsigned,
    pub wol_enabled:1: unsigned,
}
