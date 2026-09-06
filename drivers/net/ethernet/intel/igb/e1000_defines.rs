//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/igb/e1000_defines.h
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
// Copyright(c) 2007 - 2018 Intel Corporation.
// Number of Transmit and Receive Descriptors must be a multiple of 8
pub const REQ_TX_DESCRIPTOR_MULTIPLE: c_int = 8;
pub const REQ_RX_DESCRIPTOR_MULTIPLE: c_int = 8;
// Definitions for power management and wakeup registers
// Wake Up Control
pub const E1000_WUC_PME_EN: c_uint = 0x00000002 /* PME Enable */;
// Wake Up Filter Control
pub const E1000_WUFC_LNKC: c_uint = 0x00000001 /* Link Status Change Wakeup Enable */;
pub const E1000_WUFC_MAG: c_uint = 0x00000002 /* Magic Packet Wakeup Enable */;
pub const E1000_WUFC_EX: c_uint = 0x00000004 /* Directed Exact Wakeup Enable */;
pub const E1000_WUFC_MC: c_uint = 0x00000008 /* Directed Multicast Wakeup Enable */;
pub const E1000_WUFC_BC: c_uint = 0x00000010 /* Broadcast Wakeup Enable */;
// Wake Up Status
pub const E1000_WUS_EX: c_uint = 0x00000004 /* Directed Exact */;
pub const E1000_WUS_ARPD: c_uint = 0x00000020 /* Directed ARP Request */;
pub const E1000_WUS_IPV4: c_uint = 0x00000040 /* Directed IPv4 */;
pub const E1000_WUS_IPV6: c_uint = 0x00000080 /* Directed IPv6 */;
pub const E1000_WUS_NSD: c_uint = 0x00000400 /* Directed IPv6 Neighbor Solicitation */;
// Packet types that are enabled for wake packet delivery

// Wake Up Packet Length
pub const E1000_WUPL_MASK: c_uint = 0x00000FFF;
// Wake Up Packet Memory stores the first 128 bytes of the wake up packet
pub const E1000_WUPM_BYTES: c_int = 128;
// Extended Device Control
pub const E1000_CTRL_EXT_SDP2_DATA: c_uint = 0x00000040 /* Value of SW Defineable Pin 2 */;
pub const E1000_CTRL_EXT_SDP3_DATA: c_uint = 0x00000080 /* Value of SW Defineable Pin 3 */;
pub const E1000_CTRL_EXT_SDP2_DIR: c_uint = 0x00000400 /* SDP2 Data direction */;
pub const E1000_CTRL_EXT_SDP3_DIR: c_uint = 0x00000800 /* SDP3 Data direction */;
// Physical Func Reset Done Indication
pub const E1000_CTRL_EXT_PFRSTD: c_uint = 0x00004000;

pub const E1000_CTRL_EXT_LINK_MODE_MASK: c_uint = 0x00C00000;
pub const E1000_CTRL_EXT_LINK_MODE_PCIE_SERDES: c_uint = 0x00C00000;
pub const E1000_CTRL_EXT_LINK_MODE_1000BASE_KX: c_uint = 0x00400000;
pub const E1000_CTRL_EXT_LINK_MODE_SGMII: c_uint = 0x00800000;
pub const E1000_CTRL_EXT_LINK_MODE_GMII: c_uint = 0x00000000;
pub const E1000_CTRL_EXT_EIAME: c_uint = 0x01000000;
pub const E1000_CTRL_EXT_IRCA: c_uint = 0x00000001;
// Interrupt delay cancellation
// Driver loaded bit for FW
pub const E1000_CTRL_EXT_DRV_LOAD: c_uint = 0x10000000;
// Interrupt acknowledge Auto-mask
// Clear Interrupt timers after IMS clear
// packet buffer parity error detection enabled
// descriptor FIFO parity error detection enable
pub const E1000_CTRL_EXT_PBA_CLR: c_uint = 0x80000000 /* PBA Clear */;
pub const E1000_CTRL_EXT_PHYPDEN: c_uint = 0x00100000;
pub const E1000_I2CCMD_REG_ADDR_SHIFT: c_int = 16;
pub const E1000_I2CCMD_PHY_ADDR_SHIFT: c_int = 24;
pub const E1000_I2CCMD_OPCODE_READ: c_uint = 0x08000000;
pub const E1000_I2CCMD_OPCODE_WRITE: c_uint = 0x00000000;
pub const E1000_I2CCMD_READY: c_uint = 0x20000000;
pub const E1000_I2CCMD_ERROR: c_uint = 0x80000000;

pub const E1000_MAX_SGMII_PHY_REG_ADDR: c_int = 255;
pub const E1000_I2CCMD_PHY_TIMEOUT: c_int = 200;
pub const E1000_IVAR_VALID: c_uint = 0x80;
pub const E1000_GPIE_NSICR: c_uint = 0x00000001;
pub const E1000_GPIE_MSIX_MODE: c_uint = 0x00000010;
pub const E1000_GPIE_EIAME: c_uint = 0x40000000;
pub const E1000_GPIE_PBA: c_uint = 0x80000000;
// Receive Descriptor bit definitions
pub const E1000_RXD_STAT_DD: c_uint = 0x01    /* Descriptor Done */;
pub const E1000_RXD_STAT_EOP: c_uint = 0x02    /* End of Packet */;
pub const E1000_RXD_STAT_IXSM: c_uint = 0x04    /* Ignore checksum */;
pub const E1000_RXD_STAT_VP: c_uint = 0x08    /* IEEE VLAN Packet */;
pub const E1000_RXD_STAT_UDPCS: c_uint = 0x10    /* UDP xsum calculated */;
pub const E1000_RXD_STAT_TCPCS: c_uint = 0x20    /* TCP xsum calculated */;
pub const E1000_RXD_STAT_TS: c_uint = 0x10000 /* Pkt was time stamped */;
pub const E1000_RXDEXT_STATERR_LB: c_uint = 0x00040000;
pub const E1000_RXDEXT_STATERR_CE: c_uint = 0x01000000;
pub const E1000_RXDEXT_STATERR_SE: c_uint = 0x02000000;
pub const E1000_RXDEXT_STATERR_SEQ: c_uint = 0x04000000;
pub const E1000_RXDEXT_STATERR_CXE: c_uint = 0x10000000;
pub const E1000_RXDEXT_STATERR_TCPE: c_uint = 0x20000000;
pub const E1000_RXDEXT_STATERR_IPE: c_uint = 0x40000000;
pub const E1000_RXDEXT_STATERR_RXE: c_uint = 0x80000000;
// Same mask, but for extended and packet split descriptors

pub const E1000_MRQC_RSS_FIELD_IPV4_TCP: c_uint = 0x00010000;
pub const E1000_MRQC_RSS_FIELD_IPV4: c_uint = 0x00020000;
pub const E1000_MRQC_RSS_FIELD_IPV6_TCP_EX: c_uint = 0x00040000;
pub const E1000_MRQC_RSS_FIELD_IPV6: c_uint = 0x00100000;
pub const E1000_MRQC_RSS_FIELD_IPV6_TCP: c_uint = 0x00200000;
// Management Control
pub const E1000_MANC_SMBUS_EN: c_uint = 0x00000001 /* SMBus Enabled - RO */;
pub const E1000_MANC_ASF_EN: c_uint = 0x00000002 /* ASF Enabled - RO */;
pub const E1000_MANC_EN_BMC2OS: c_uint = 0x10000000 /* OSBMC is Enabled or not */;
// Enable Neighbor Discovery Filtering
pub const E1000_MANC_RCV_TCO_EN: c_uint = 0x00020000 /* Receive TCO Packets Enabled */;
pub const E1000_MANC_BLK_PHY_RST_ON_IDE: c_uint = 0x00040000 /* Block phy resets */;
// Enable MAC address filtering
pub const E1000_MANC_EN_MAC_ADDR_FILTER: c_uint = 0x00100000;
// Receive Control
pub const E1000_RCTL_EN: c_uint = 0x00000002    /* enable */;
pub const E1000_RCTL_SBP: c_uint = 0x00000004    /* store bad packet */;
pub const E1000_RCTL_UPE: c_uint = 0x00000008    /* unicast promiscuous enable */;
pub const E1000_RCTL_MPE: c_uint = 0x00000010    /* multicast promiscuous enab */;
pub const E1000_RCTL_LPE: c_uint = 0x00000020    /* long packet enable */;
pub const E1000_RCTL_LBM_MAC: c_uint = 0x00000040    /* MAC loopback mode */;
pub const E1000_RCTL_LBM_TCVR: c_uint = 0x000000C0    /* tcvr loopback mode */;
pub const E1000_RCTL_RDMTS_HALF: c_uint = 0x00000000    /* rx desc min threshold size */;

pub const E1000_RCTL_BAM: c_uint = 0x00008000    /* broadcast enable */;
pub const E1000_RCTL_SZ_512: c_uint = 0x00020000    /* rx buffer size 512 */;
pub const E1000_RCTL_SZ_256: c_uint = 0x00030000    /* rx buffer size 256 */;
pub const E1000_RCTL_VFE: c_uint = 0x00040000    /* vlan filter enable */;
pub const E1000_RCTL_CFIEN: c_uint = 0x00080000    /* canonical form enable */;
pub const E1000_RCTL_DPF: c_uint = 0x00400000    /* Discard Pause Frames */;
pub const E1000_RCTL_PMCF: c_uint = 0x00800000    /* pass MAC control frames */;
pub const E1000_RCTL_SECRC: c_uint = 0x04000000    /* Strip Ethernet CRC */;
// Use byte values for the following shift parameters
// Usage:
// psrctl |= (((ROUNDUP(value0, 128) >> E1000_PSRCTL_BSIZE0_SHIFT) &
// E1000_PSRCTL_BSIZE0_MASK) |
// ((ROUNDUP(value1, 1024) >> E1000_PSRCTL_BSIZE1_SHIFT) &
// E1000_PSRCTL_BSIZE1_MASK) |
// ((ROUNDUP(value2, 1024) << E1000_PSRCTL_BSIZE2_SHIFT) &
// E1000_PSRCTL_BSIZE2_MASK) |
// ((ROUNDUP(value3, 1024) << E1000_PSRCTL_BSIZE3_SHIFT) |;
// E1000_PSRCTL_BSIZE3_MASK))
// where value0 = [128..16256],  default=256
// value1 = [1024..64512], default=4096
// value2 = [0..64512],    default=4096
// value3 = [0..64512],    default=0
//
pub const E1000_PSRCTL_BSIZE0_MASK: c_uint = 0x0000007F;
pub const E1000_PSRCTL_BSIZE1_MASK: c_uint = 0x00003F00;
pub const E1000_PSRCTL_BSIZE2_MASK: c_uint = 0x003F0000;
pub const E1000_PSRCTL_BSIZE3_MASK: c_uint = 0x3F000000;

// SWFW_SYNC Definitions
pub const E1000_SWFW_EEP_SM: c_uint = 0x1;
pub const E1000_SWFW_PHY0_SM: c_uint = 0x2;
pub const E1000_SWFW_PHY1_SM: c_uint = 0x4;
pub const E1000_SWFW_PHY2_SM: c_uint = 0x20;
pub const E1000_SWFW_PHY3_SM: c_uint = 0x40;
// FACTPS Definitions
// Device Control
pub const E1000_CTRL_FD: c_uint = 0x00000001  /* Full duplex.0=half; 1=full */;
pub const E1000_CTRL_GIO_MASTER_DISABLE: c_uint = 0x00000004 /*Blocks new Master requests */;
pub const E1000_CTRL_LRST: c_uint = 0x00000008  /* Link reset. 0=normal,1=reset */;
pub const E1000_CTRL_ASDE: c_uint = 0x00000020  /* Auto-speed detect enable */;
pub const E1000_CTRL_SLU: c_uint = 0x00000040  /* Set link up (Force Link) */;
pub const E1000_CTRL_ILOS: c_uint = 0x00000080  /* Invert Loss-Of Signal */;
pub const E1000_CTRL_SPD_SEL: c_uint = 0x00000300  /* Speed Select Mask */;
pub const E1000_CTRL_SPD_100: c_uint = 0x00000100  /* Force 100Mb */;
pub const E1000_CTRL_SPD_1000: c_uint = 0x00000200  /* Force 1Gb */;
pub const E1000_CTRL_FRCSPD: c_uint = 0x00000800  /* Force Speed */;
pub const E1000_CTRL_FRCDPX: c_uint = 0x00001000  /* Force Duplex */;
// Defined polarity of Dock/Undock indication in SDP[0]
// Reset both PHY ports, through PHYRST_N pin
// enable link status from external LINK_0 and LINK_1 pins
pub const E1000_CTRL_SWDPIN0: c_uint = 0x00040000  /* SWDPIN 0 value */;
pub const E1000_CTRL_SWDPIN1: c_uint = 0x00080000  /* SWDPIN 1 value */;
pub const E1000_CTRL_ADVD3WUC: c_uint = 0x00100000  /* D3 WUC */;
pub const E1000_CTRL_EN_PHY_PWR_MGMT: c_uint = 0x00200000 /* PHY PM enable */;
pub const E1000_CTRL_SDP0_DIR: c_uint = 0x00400000  /* SDP0 Data direction */;
pub const E1000_CTRL_SDP1_DIR: c_uint = 0x00800000  /* SDP1 Data direction */;
pub const E1000_CTRL_RST: c_uint = 0x04000000  /* Global reset */;
pub const E1000_CTRL_RFCE: c_uint = 0x08000000  /* Receive Flow Control enable */;
pub const E1000_CTRL_TFCE: c_uint = 0x10000000  /* Transmit flow control enable */;
pub const E1000_CTRL_VME: c_uint = 0x40000000  /* IEEE VLAN mode enable */;
pub const E1000_CTRL_PHY_RST: c_uint = 0x80000000  /* PHY Reset */;
// Initiate an interrupt to manageability engine
pub const E1000_CTRL_I2C_ENA: c_uint = 0x02000000  /* I2C enable */;
// Bit definitions for the Management Data IO (MDIO) and Management Data
// Clock (MDC) pins in the Device Control Register.
//
pub const E1000_CONNSW_ENRGSRC: c_uint = 0x4;
pub const E1000_CONNSW_PHYSD: c_uint = 0x400;
pub const E1000_CONNSW_PHY_PDN: c_uint = 0x800;
pub const E1000_CONNSW_SERDESD: c_uint = 0x200;
pub const E1000_CONNSW_AUTOSENSE_CONF: c_uint = 0x2;
pub const E1000_CONNSW_AUTOSENSE_EN: c_uint = 0x1;
pub const E1000_PCS_CFG_PCS_EN: c_int = 8;
pub const E1000_PCS_LCTL_FLV_LINK_UP: c_int = 1;
pub const E1000_PCS_LCTL_FSV_100: c_int = 2;
pub const E1000_PCS_LCTL_FSV_1000: c_int = 4;
pub const E1000_PCS_LCTL_FDV_FULL: c_int = 8;
pub const E1000_PCS_LCTL_FSD: c_uint = 0x10;
pub const E1000_PCS_LCTL_FORCE_LINK: c_uint = 0x20;
pub const E1000_PCS_LCTL_FORCE_FCTRL: c_uint = 0x80;
pub const E1000_PCS_LCTL_AN_ENABLE: c_uint = 0x10000;
pub const E1000_PCS_LCTL_AN_RESTART: c_uint = 0x20000;
pub const E1000_PCS_LCTL_AN_TIMEOUT: c_uint = 0x40000;
pub const E1000_ENABLE_SERDES_LOOPBACK: c_uint = 0x0410;
pub const E1000_PCS_LSTS_LINK_OK: c_int = 1;
pub const E1000_PCS_LSTS_SPEED_100: c_int = 2;
pub const E1000_PCS_LSTS_SPEED_1000: c_int = 4;
pub const E1000_PCS_LSTS_DUPLEX_FULL: c_int = 8;
pub const E1000_PCS_LSTS_SYNK_OK: c_uint = 0x10;
// Device Status
pub const E1000_STATUS_FD: c_uint = 0x00000001      /* Full duplex.0=half,1=full */;
pub const E1000_STATUS_LU: c_uint = 0x00000002      /* Link up.0=no,1=link */;
pub const E1000_STATUS_FUNC_MASK: c_uint = 0x0000000C      /* PCI Function Mask */;
pub const E1000_STATUS_FUNC_SHIFT: c_int = 2;
pub const E1000_STATUS_FUNC_1: c_uint = 0x00000004      /* Function 1 */;
pub const E1000_STATUS_TXOFF: c_uint = 0x00000010      /* transmission paused */;
pub const E1000_STATUS_SPEED_100: c_uint = 0x00000040      /* Speed 100Mb/s */;
pub const E1000_STATUS_SPEED_1000: c_uint = 0x00000080      /* Speed 1000Mb/s */;
// Change in Dock/Undock state. Clear on write '0'.
// Status of Master requests.
pub const E1000_STATUS_GIO_MASTER_ENABLE: c_uint = 0x00080000;
// BMC external code execution disabled
pub const E1000_STATUS_2P5_SKU: c_uint = 0x00001000 /* Val of 2.5GBE SKU strap */;
pub const E1000_STATUS_2P5_SKU_OVER: c_uint = 0x00002000 /* Val of 2.5GBE SKU Over */;
// Constants used to intrepret the masked PCI-X bus speed.
pub const SPEED_10: c_int = 10;
pub const SPEED_100: c_int = 100;
pub const SPEED_1000: c_int = 1000;
pub const SPEED_2500: c_int = 2500;
pub const HALF_DUPLEX: c_int = 1;
pub const FULL_DUPLEX: c_int = 2;
pub const ADVERTISE_10_HALF: c_uint = 0x0001;
pub const ADVERTISE_10_FULL: c_uint = 0x0002;
pub const ADVERTISE_100_HALF: c_uint = 0x0004;
pub const ADVERTISE_100_FULL: c_uint = 0x0008;
pub const ADVERTISE_1000_HALF: c_uint = 0x0010 /* Not used, just FYI */;
pub const ADVERTISE_1000_FULL: c_uint = 0x0020;
// 1000/H is not supported, nor spec-compliant.

// LED Control
pub const E1000_LEDCTL_LED0_MODE_SHIFT: c_int = 0;
pub const E1000_LEDCTL_LED0_BLINK: c_uint = 0x00000080;
pub const E1000_LEDCTL_LED0_MODE_MASK: c_uint = 0x0000000F;
pub const E1000_LEDCTL_LED0_IVRT: c_uint = 0x00000040;
pub const E1000_LEDCTL_MODE_LED_ON: c_uint = 0xE;
pub const E1000_LEDCTL_MODE_LED_OFF: c_uint = 0xF;
// Transmit Descriptor bit definitions
pub const E1000_TXD_POPTS_IXSM: c_uint = 0x01       /* Insert IP checksum */;
pub const E1000_TXD_POPTS_TXSM: c_uint = 0x02       /* Insert TCP/UDP checksum */;
pub const E1000_TXD_CMD_EOP: c_uint = 0x01000000 /* End of Packet */;
pub const E1000_TXD_CMD_IFCS: c_uint = 0x02000000 /* Insert FCS (Ethernet CRC) */;
pub const E1000_TXD_CMD_RS: c_uint = 0x08000000 /* Report Status */;
pub const E1000_TXD_CMD_DEXT: c_uint = 0x20000000 /* Descriptor extension (0 = legacy) */;
pub const E1000_TXD_STAT_DD: c_uint = 0x00000001 /* Descriptor Done */;
// Extended desc bits for Linksec and timesync
// Transmit Control
pub const E1000_TCTL_EN: c_uint = 0x00000002    /* enable tx */;
pub const E1000_TCTL_PSP: c_uint = 0x00000008    /* pad short packets */;
pub const E1000_TCTL_CT: c_uint = 0x00000ff0    /* collision threshold */;
pub const E1000_TCTL_COLD: c_uint = 0x003ff000    /* collision distance */;
pub const E1000_TCTL_RTLC: c_uint = 0x01000000    /* Re-transmit on late collision */;
// DMA Coalescing register fields
pub const E1000_DMACR_DMACWT_MASK: c_uint = 0x00003FFF /* DMA Coal Watchdog Timer */;
pub const E1000_DMACR_DMACTHR_MASK: c_uint = 0x00FF0000 /* DMA Coal Rx Threshold */;
pub const E1000_DMACR_DMACTHR_SHIFT: c_int = 16;
pub const E1000_DMACR_DMAC_LX_MASK: c_uint = 0x30000000 /* Lx when no PCIe trans */;
pub const E1000_DMACR_DMAC_LX_SHIFT: c_int = 28;
pub const E1000_DMACR_DMAC_EN: c_uint = 0x80000000 /* Enable DMA Coalescing */;
// DMA Coalescing BMC-to-OS Watchdog Enable
pub const E1000_DMACR_DC_BMC2OSW_EN: c_uint = 0x00008000;
pub const E1000_DMCTXTH_DMCTTHR_MASK: c_uint = 0x00000FFF /* DMA Coal Tx Threshold */;
pub const E1000_DMCTLX_TTLX_MASK: c_uint = 0x00000FFF /* Time to LX request */;
pub const E1000_DMCRTRH_UTRESH_MASK: c_uint = 0x0007FFFF /* Rx Traffic Rate Thresh */;
pub const E1000_DMCRTRH_LRPRCW: c_uint = 0x80000000 /* Rx pkt rate curr window */;
pub const E1000_DMCCNT_CCOUNT_MASK: c_uint = 0x01FFFFFF /* DMA Coal Rx Current Cnt */;
pub const E1000_FCRTC_RTH_COAL_MASK: c_uint = 0x0003FFF0 /* FC Rx Thresh High val */;
pub const E1000_FCRTC_RTH_COAL_SHIFT: c_int = 4;
pub const E1000_PCIEMISC_LX_DECISION: c_uint = 0x00000080 /* Lx power decision */;
// Timestamp in Rx buffer
pub const E1000_RXPBS_CFG_TS_EN: c_uint = 0x80000000;
pub const I210_RXPBSIZE_DEFAULT: c_uint = 0x000000A2 /* RXPBSIZE default */;
pub const I210_RXPBSIZE_MASK: c_uint = 0x0000003F;
pub const I210_RXPBSIZE_PB_30KB: c_uint = 0x0000001E;
pub const I210_RXPBSIZE_PB_32KB: c_uint = 0x00000020;
pub const I210_TXPBSIZE_DEFAULT: c_uint = 0x04000014 /* TXPBSIZE default */;
pub const I210_TXPBSIZE_MASK: c_uint = 0xC0FFFFFF;

pub const I210_DTXMXPKTSZ_DEFAULT: c_uint = 0x00000098;
pub const I210_SR_QUEUES_NUM: c_int = 2;
// SerDes Control
pub const E1000_SCTL_DISABLE_SERDES_LOOPBACK: c_uint = 0x0400;
// Receive Checksum Control
pub const E1000_RXCSUM_IPOFL: c_uint = 0x00000100   /* IPv4 checksum offload */;
pub const E1000_RXCSUM_TUOFL: c_uint = 0x00000200   /* TCP / UDP checksum offload */;
pub const E1000_RXCSUM_CRCOFL: c_uint = 0x00000800   /* CRC32 offload enable */;
pub const E1000_RXCSUM_PCSD: c_uint = 0x00002000   /* packet checksum disabled */;
// Header split receive
pub const E1000_RFCTL_IPV6_EX_DIS: c_uint = 0x00010000;
pub const E1000_RFCTL_LEF: c_uint = 0x00040000;
// Collision related configuration parameters
pub const E1000_COLLISION_THRESHOLD: c_int = 15;
pub const E1000_CT_SHIFT: c_int = 4;
pub const E1000_COLLISION_DISTANCE: c_int = 63;
pub const E1000_COLD_SHIFT: c_int = 12;
// Ethertype field values
pub const ETHERNET_IEEE_VLAN_TYPE: c_uint = 0x8100  /* 802.3ac packet */;
// As per the EAS the maximum supported size is 9.5KB (9728 bytes)
pub const MAX_JUMBO_FRAME_SIZE: c_uint = 0x2600;
pub const MAX_STD_JUMBO_FRAME_SIZE: c_int = 9216;
// PBA constants
pub const E1000_PBA_34K: c_uint = 0x0022;
pub const E1000_PBA_64K: c_uint = 0x0040    /* 64KB */;
// SW Semaphore Register
pub const E1000_SWSM_SMBI: c_uint = 0x00000001 /* Driver Semaphore bit */;
pub const E1000_SWSM_SWESMBI: c_uint = 0x00000002 /* FW Semaphore bit */;
// Interrupt Cause Read
pub const E1000_ICR_TXDW: c_uint = 0x00000001 /* Transmit desc written back */;
pub const E1000_ICR_LSC: c_uint = 0x00000004 /* Link Status Change */;
pub const E1000_ICR_RXSEQ: c_uint = 0x00000008 /* rx sequence error */;
pub const E1000_ICR_RXDMT0: c_uint = 0x00000010 /* rx desc min. threshold (0) */;
pub const E1000_ICR_RXT0: c_uint = 0x00000080 /* rx timer intr (ring 0) */;
pub const E1000_ICR_VMMB: c_uint = 0x00000100 /* VM MB event */;
pub const E1000_ICR_TS: c_uint = 0x00080000 /* Time Sync Interrupt */;
pub const E1000_ICR_DRSTA: c_uint = 0x40000000 /* Device Reset Asserted */;
// If this bit asserted, the driver should claim the interrupt
pub const E1000_ICR_INT_ASSERTED: c_uint = 0x80000000;
// LAN connected device generates an interrupt
pub const E1000_ICR_DOUTSYNC: c_uint = 0x10000000 /* NIC DMA out of sync */;
// Extended Interrupt Cause Read
pub const E1000_EICR_RX_QUEUE0: c_uint = 0x00000001 /* Rx Queue 0 Interrupt */;
pub const E1000_EICR_RX_QUEUE1: c_uint = 0x00000002 /* Rx Queue 1 Interrupt */;
pub const E1000_EICR_RX_QUEUE2: c_uint = 0x00000004 /* Rx Queue 2 Interrupt */;
pub const E1000_EICR_RX_QUEUE3: c_uint = 0x00000008 /* Rx Queue 3 Interrupt */;
pub const E1000_EICR_TX_QUEUE0: c_uint = 0x00000100 /* Tx Queue 0 Interrupt */;
pub const E1000_EICR_TX_QUEUE1: c_uint = 0x00000200 /* Tx Queue 1 Interrupt */;
pub const E1000_EICR_TX_QUEUE2: c_uint = 0x00000400 /* Tx Queue 2 Interrupt */;
pub const E1000_EICR_TX_QUEUE3: c_uint = 0x00000800 /* Tx Queue 3 Interrupt */;
pub const E1000_EICR_OTHER: c_uint = 0x80000000 /* Interrupt Cause Active */;
// TCP Timer
// This defines the bits that are set in the Interrupt Mask
// Set/Read Register.  Each bit is documented below:
// o RXT0   = Receiver Timer Interrupt (ring 0)
// o TXDW   = Transmit Descriptor Written Back
// o RXDMT0 = Receive Descriptor Minimum Threshold hit (ring 0)
// o RXSEQ  = Receive Sequence Error
// o LSC    = Link Status Change
//

// Interrupt Mask Set

// Extended Interrupt Mask Set

// Interrupt Cause Set

// Extended Interrupt Cause Set
// E1000_EITR_CNT_IGNR is only for 82576 and newer
pub const E1000_EITR_CNT_IGNR: c_uint = 0x80000000 /* Don't reset counters on write */;
// Transmit Descriptor Control
// Enable the counting of descriptors still to be processed.
// Flow Control Constants
pub const FLOW_CONTROL_ADDRESS_LOW: c_uint = 0x00C28001;
pub const FLOW_CONTROL_ADDRESS_HIGH: c_uint = 0x00000100;
pub const FLOW_CONTROL_TYPE: c_uint = 0x8808;
// Transmit Config Word
pub const E1000_TXCW_ASM_DIR: c_uint = 0x00000100 /* TXCW astm pause direction */;
pub const E1000_TXCW_PAUSE: c_uint = 0x00000080 /* TXCW sym pause request */;
// 802.1q VLAN Packet Size

// Receive Address
// Number of high/low register pairs in the RAR. The RAR (Receive Address
// Registers) holds the directed and multicast addresses that we monitor.
// Technically, we have 16 spots.  However, we reserve one of these spots
// (RAR[15]) for our directed address used by controllers with
// manageability enabled, allowing us room for 15 multicast addresses.
//
pub const E1000_RAH_AV: c_uint = 0x80000000        /* Receive descriptor valid */;
pub const E1000_RAH_ASEL_SRC_ADDR: c_uint = 0x00010000;
pub const E1000_RAH_QSEL_ENABLE: c_uint = 0x10000000;
pub const E1000_RAL_MAC_ADDR_LEN: c_int = 4;
pub const E1000_RAH_MAC_ADDR_LEN: c_int = 2;
pub const E1000_RAH_POOL_MASK: c_uint = 0x03FC0000;
pub const E1000_RAH_POOL_1: c_uint = 0x00040000;
// Error Codes
pub const E1000_ERR_NVM: c_int = 1;
pub const E1000_ERR_PHY: c_int = 2;
pub const E1000_ERR_CONFIG: c_int = 3;
pub const E1000_ERR_PARAM: c_int = 4;
pub const E1000_ERR_MAC_INIT: c_int = 5;
pub const E1000_ERR_RESET: c_int = 9;
pub const E1000_ERR_MASTER_REQUESTS_PENDING: c_int = 10;
pub const E1000_BLK_PHY_RESET: c_int = 12;
pub const E1000_ERR_SWFW_SYNC: c_int = 13;
pub const E1000_NOT_IMPLEMENTED: c_int = 14;
pub const E1000_ERR_MBX: c_int = 15;
pub const E1000_ERR_INVALID_ARGUMENT: c_int = 16;
pub const E1000_ERR_NO_SPACE: c_int = 17;
pub const E1000_ERR_NVM_PBA_SECTION: c_int = 18;
pub const E1000_ERR_INVM_VALUE_NOT_FOUND: c_int = 19;
pub const E1000_ERR_I2C: c_int = 20;
// Loop limit on how long we wait for auto-negotiation to complete
pub const COPPER_LINK_UP_LIMIT: c_int = 10;
pub const PHY_AUTO_NEG_LIMIT: c_int = 45;
pub const PHY_FORCE_LIMIT: c_int = 20;
// Number of 100 microseconds we wait for PCI Express master disable
pub const MASTER_DISABLE_TIMEOUT: c_int = 800;
// Number of milliseconds we wait for PHY configuration done after MAC reset
pub const PHY_CFG_TIMEOUT: c_int = 100;
// Number of 2 milliseconds we wait for acquiring MDIO ownership.
// Number of milliseconds for NVM auto read done after MAC reset.
pub const AUTO_READ_DONE_TIMEOUT: c_int = 10;
// Flow Control
pub const E1000_FCRTL_XONE: c_uint = 0x80000000     /* Enable XON frame transmission */;
pub const E1000_TSYNCTXCTL_VALID: c_uint = 0x00000001 /* tx timestamp valid */;
pub const E1000_TSYNCTXCTL_ENABLED: c_uint = 0x00000010 /* enable tx timestampping */;
pub const E1000_TSYNCRXCTL_VALID: c_uint = 0x00000001 /* rx timestamp valid */;
pub const E1000_TSYNCRXCTL_TYPE_MASK: c_uint = 0x0000000E /* rx type mask */;
pub const E1000_TSYNCRXCTL_TYPE_L2_V2: c_uint = 0x00;
pub const E1000_TSYNCRXCTL_TYPE_L4_V1: c_uint = 0x02;
pub const E1000_TSYNCRXCTL_TYPE_L2_L4_V2: c_uint = 0x04;
pub const E1000_TSYNCRXCTL_TYPE_ALL: c_uint = 0x08;
pub const E1000_TSYNCRXCTL_TYPE_EVENT_V2: c_uint = 0x0A;
pub const E1000_TSYNCRXCTL_ENABLED: c_uint = 0x00000010 /* enable rx timestampping */;
pub const E1000_TSYNCRXCFG_PTP_V1_CTRLT_MASK: c_uint = 0x000000FF;
pub const E1000_TSYNCRXCFG_PTP_V1_SYNC_MESSAGE: c_uint = 0x00;
pub const E1000_TSYNCRXCFG_PTP_V1_DELAY_REQ_MESSAGE: c_uint = 0x01;
pub const E1000_TSYNCRXCFG_PTP_V1_FOLLOWUP_MESSAGE: c_uint = 0x02;
pub const E1000_TSYNCRXCFG_PTP_V1_DELAY_RESP_MESSAGE: c_uint = 0x03;
pub const E1000_TSYNCRXCFG_PTP_V1_MANAGEMENT_MESSAGE: c_uint = 0x04;
pub const E1000_TSYNCRXCFG_PTP_V2_MSGID_MASK: c_uint = 0x00000F00;
pub const E1000_TSYNCRXCFG_PTP_V2_SYNC_MESSAGE: c_uint = 0x0000;
pub const E1000_TSYNCRXCFG_PTP_V2_DELAY_REQ_MESSAGE: c_uint = 0x0100;
pub const E1000_TSYNCRXCFG_PTP_V2_PATH_DELAY_REQ_MESSAGE: c_uint = 0x0200;
pub const E1000_TSYNCRXCFG_PTP_V2_PATH_DELAY_RESP_MESSAGE: c_uint = 0x0300;
pub const E1000_TSYNCRXCFG_PTP_V2_FOLLOWUP_MESSAGE: c_uint = 0x0800;
pub const E1000_TSYNCRXCFG_PTP_V2_DELAY_RESP_MESSAGE: c_uint = 0x0900;
pub const E1000_TSYNCRXCFG_PTP_V2_PATH_DELAY_FOLLOWUP_MESSAGE: c_uint = 0x0A00;
pub const E1000_TSYNCRXCFG_PTP_V2_ANNOUNCE_MESSAGE: c_uint = 0x0B00;
pub const E1000_TSYNCRXCFG_PTP_V2_SIGNALLING_MESSAGE: c_uint = 0x0C00;
pub const E1000_TSYNCRXCFG_PTP_V2_MANAGEMENT_MESSAGE: c_uint = 0x0D00;
pub const E1000_TIMINCA_16NS_SHIFT: c_int = 24;
// Time Sync Interrupt Cause/Mask Register Bits

// TSAUXC Configuration Bits

// SDP Configuration Bits

pub const E1000_MDICNFG_EXT_MDIO: c_uint = 0x80000000      /* MDI ext/int destination */;
pub const E1000_MDICNFG_COM_MDIO: c_uint = 0x40000000      /* MDI shared w/ lan 0 */;
pub const E1000_MDICNFG_PHY_MASK: c_uint = 0x03E00000;
pub const E1000_MDICNFG_PHY_SHIFT: c_int = 21;
pub const E1000_MEDIA_PORT_COPPER: c_int = 1;
pub const E1000_MEDIA_PORT_OTHER: c_int = 2;
pub const E1000_M88E1112_AUTO_COPPER_SGMII: c_uint = 0x2;
pub const E1000_M88E1112_AUTO_COPPER_BASEX: c_uint = 0x3;
pub const E1000_M88E1112_STATUS_LINK: c_uint = 0x0004 /* Interface Link Bit */;
pub const E1000_M88E1112_MAC_CTRL_1: c_uint = 0x10;
pub const E1000_M88E1112_MAC_CTRL_1_MODE_MASK: c_uint = 0x0380 /* Mode Select */;
pub const E1000_M88E1112_MAC_CTRL_1_MODE_SHIFT: c_int = 7;
pub const E1000_M88E1112_PAGE_ADDR: c_uint = 0x16;
pub const E1000_M88E1112_STATUS: c_uint = 0x01;
pub const E1000_M88E1512_CFG_REG_1: c_uint = 0x0010;
pub const E1000_M88E1512_CFG_REG_2: c_uint = 0x0011;
pub const E1000_M88E1512_CFG_REG_3: c_uint = 0x0007;
pub const E1000_M88E1512_MODE: c_uint = 0x0014;
// PCI Express Control
pub const E1000_GCR_CMPL_TMOUT_MASK: c_uint = 0x0000F000;
pub const E1000_GCR_CMPL_TMOUT_10ms: c_uint = 0x00001000;
pub const E1000_GCR_CMPL_TMOUT_RESEND: c_uint = 0x00010000;
pub const E1000_GCR_CAP_VER2: c_uint = 0x00040000;
// mPHY Address Control and Data Registers
pub const E1000_MPHY_ADDR_CTL: c_uint = 0x0024 /* mPHY Address Control Register */;
pub const E1000_MPHY_ADDR_CTL_OFFSET_MASK: c_uint = 0xFFFF0000;
pub const E1000_MPHY_DATA: c_uint = 0x0E10 /* mPHY Data Register */;
// mPHY PCS CLK Register
pub const E1000_MPHY_PCS_CLK_REG_OFFSET: c_uint = 0x0004 /* mPHY PCS CLK AFE CSR Offset */;
// mPHY Near End Digital Loopback Override Bit
pub const E1000_MPHY_PCS_CLK_REG_DIGINELBEN: c_uint = 0x10;
pub const E1000_PCS_LCTL_FORCE_FCTRL: c_uint = 0x80;
pub const E1000_PCS_LSTS_AN_COMPLETE: c_uint = 0x10000;
// PHY Control Register
pub const MII_CR_FULL_DUPLEX: c_uint = 0x0100  /* FDX =1, half duplex =0 */;
pub const MII_CR_RESTART_AUTO_NEG: c_uint = 0x0200  /* Restart auto negotiation */;
pub const MII_CR_POWER_DOWN: c_uint = 0x0800  /* Power down */;
pub const MII_CR_AUTO_NEG_EN: c_uint = 0x1000  /* Auto Neg Enable */;
pub const MII_CR_LOOPBACK: c_uint = 0x4000  /* 0 = normal, 1 = loopback */;
pub const MII_CR_RESET: c_uint = 0x8000  /* 0 = normal, 1 = PHY reset */;
pub const MII_CR_SPEED_1000: c_uint = 0x0040;
pub const MII_CR_SPEED_100: c_uint = 0x2000;
pub const MII_CR_SPEED_10: c_uint = 0x0000;
// PHY Status Register
pub const MII_SR_LINK_STATUS: c_uint = 0x0004 /* Link Status 1 = link */;
pub const MII_SR_AUTONEG_COMPLETE: c_uint = 0x0020 /* Auto Neg Complete */;
// Autoneg Advertisement Register
pub const NWAY_AR_10T_HD_CAPS: c_uint = 0x0020   /* 10T   Half Duplex Capable */;
pub const NWAY_AR_10T_FD_CAPS: c_uint = 0x0040   /* 10T   Full Duplex Capable */;
pub const NWAY_AR_100TX_HD_CAPS: c_uint = 0x0080   /* 100TX Half Duplex Capable */;
pub const NWAY_AR_100TX_FD_CAPS: c_uint = 0x0100   /* 100TX Full Duplex Capable */;
pub const NWAY_AR_PAUSE: c_uint = 0x0400   /* Pause operation desired */;
pub const NWAY_AR_ASM_DIR: c_uint = 0x0800   /* Asymmetric Pause Direction bit */;
// Link Partner Ability Register (Base Page)
pub const NWAY_LPAR_PAUSE: c_uint = 0x0400 /* LP Pause operation desired */;
pub const NWAY_LPAR_ASM_DIR: c_uint = 0x0800 /* LP Asymmetric Pause Direction bit */;
// Autoneg Expansion Register
// 1000BASE-T Control Register
pub const CR_1000T_HD_CAPS: c_uint = 0x0100 /* Advertise 1000T HD capability */;
pub const CR_1000T_FD_CAPS: c_uint = 0x0200 /* Advertise 1000T FD capability  */;
pub const CR_1000T_MS_VALUE: c_uint = 0x0800 /* 1=Configure PHY as Master */;
// 0=Configure PHY as Slave
pub const CR_1000T_MS_ENABLE: c_uint = 0x1000 /* 1=Master/Slave manual config value */;
// 0=Automatic Master/Slave config
// 1000BASE-T Status Register
pub const SR_1000T_REMOTE_RX_STATUS: c_uint = 0x1000 /* Remote receiver OK */;
pub const SR_1000T_LOCAL_RX_STATUS: c_uint = 0x2000 /* Local receiver OK */;
// PHY 1000 MII Register/Bit Definitions
// PHY Registers defined by IEEE
pub const PHY_CONTROL: c_uint = 0x00 /* Control Register */;
pub const PHY_STATUS: c_uint = 0x01 /* Status Register */;
pub const PHY_ID1: c_uint = 0x02 /* Phy Id Reg (word 1) */;
pub const PHY_ID2: c_uint = 0x03 /* Phy Id Reg (word 2) */;
pub const PHY_AUTONEG_ADV: c_uint = 0x04 /* Autoneg Advertisement */;
pub const PHY_LP_ABILITY: c_uint = 0x05 /* Link Partner Ability (Base Page) */;
pub const PHY_1000T_CTRL: c_uint = 0x09 /* 1000Base-T Control Reg */;
pub const PHY_1000T_STATUS: c_uint = 0x0A /* 1000Base-T Status Reg */;
// NVM Control
pub const E1000_EECD_SK: c_uint = 0x00000001 /* NVM Clock */;
pub const E1000_EECD_CS: c_uint = 0x00000002 /* NVM Chip Select */;
pub const E1000_EECD_DI: c_uint = 0x00000004 /* NVM Data In */;
pub const E1000_EECD_DO: c_uint = 0x00000008 /* NVM Data Out */;
pub const E1000_EECD_REQ: c_uint = 0x00000040 /* NVM Access Request */;
pub const E1000_EECD_GNT: c_uint = 0x00000080 /* NVM Access Grant */;
pub const E1000_EECD_PRES: c_uint = 0x00000100 /* NVM Present */;
// NVM Addressing bits based on type 0=small, 1=large
pub const E1000_EECD_ADDR_BITS: c_uint = 0x00000400;

pub const E1000_EECD_AUTO_RD: c_uint = 0x00000200  /* NVM Auto Read done */;
pub const E1000_EECD_SIZE_EX_MASK: c_uint = 0x00007800  /* NVM Size */;
pub const E1000_EECD_SIZE_EX_SHIFT: c_int = 11;
pub const E1000_EECD_FLUPD_I210: c_uint = 0x00800000 /* Update FLASH */;
pub const E1000_EECD_FLUDONE_I210: c_uint = 0x04000000 /* Update FLASH done*/;
pub const E1000_EECD_FLASH_DETECTED_I210: c_uint = 0x00080000 /* FLASH detected */;
pub const E1000_FLUDONE_ATTEMPTS: c_int = 20000;

pub const E1000_I210_FIFO_SEL_RX: c_uint = 0x00;

pub const E1000_I210_FIFO_SEL_BMC2OS_TX: c_uint = 0x06;
pub const E1000_I210_FIFO_SEL_BMC2OS_RX: c_uint = 0x01;
pub const E1000_I210_FLASH_SECTOR_SIZE: c_uint = 0x1000 /* 4KB FLASH sector unit size */;
// Secure FLASH mode requires removing MSb
pub const E1000_I210_FW_PTR_MASK: c_uint = 0x7FFF;
// Firmware code revision field word offset
pub const E1000_I210_FW_VER_OFFSET: c_int = 328;
pub const E1000_EECD_FLUPD_I210: c_uint = 0x00800000 /* Update FLASH */;
pub const E1000_EECD_FLUDONE_I210: c_uint = 0x04000000 /* Update FLASH done*/;
pub const E1000_FLUDONE_ATTEMPTS: c_int = 20000;

pub const E1000_I210_FIFO_SEL_RX: c_uint = 0x00;

pub const E1000_I210_FIFO_SEL_BMC2OS_TX: c_uint = 0x06;
pub const E1000_I210_FIFO_SEL_BMC2OS_RX: c_uint = 0x01;
// Offset to data in NVM read/write registers
pub const E1000_NVM_RW_REG_DATA: c_int = 16;

// NVM Word Offsets
pub const NVM_COMPAT: c_uint = 0x0003;
pub const NVM_ID_LED_SETTINGS: c_uint = 0x0004 /* SERDES output amplitude */;
pub const NVM_VERSION: c_uint = 0x0005;
pub const NVM_INIT_CONTROL2_REG: c_uint = 0x000F;
pub const NVM_INIT_CONTROL3_PORT_B: c_uint = 0x0014;
pub const NVM_INIT_CONTROL3_PORT_A: c_uint = 0x0024;
pub const NVM_ALT_MAC_ADDR_PTR: c_uint = 0x0037;
pub const NVM_CHECKSUM_REG: c_uint = 0x003F;
pub const NVM_COMPATIBILITY_REG_3: c_uint = 0x0003;
pub const NVM_COMPATIBILITY_BIT_MASK: c_uint = 0x8000;
pub const NVM_MAC_ADDR: c_uint = 0x0000;
pub const NVM_SUB_DEV_ID: c_uint = 0x000B;
pub const NVM_SUB_VEN_ID: c_uint = 0x000C;
pub const NVM_DEV_ID: c_uint = 0x000D;
pub const NVM_VEN_ID: c_uint = 0x000E;
pub const NVM_INIT_CTRL_2: c_uint = 0x000F;
pub const NVM_INIT_CTRL_4: c_uint = 0x0013;
pub const NVM_LED_1_CFG: c_uint = 0x001C;
pub const NVM_LED_0_2_CFG: c_uint = 0x001F;
pub const NVM_ETRACK_WORD: c_uint = 0x0042;
pub const NVM_ETRACK_HIWORD: c_uint = 0x0043;
pub const NVM_COMB_VER_OFF: c_uint = 0x0083;
pub const NVM_COMB_VER_PTR: c_uint = 0x003d;
// NVM version defines
pub const NVM_MAJOR_MASK: c_uint = 0xF000;
pub const NVM_MINOR_MASK: c_uint = 0x0FF0;
pub const NVM_IMAGE_ID_MASK: c_uint = 0x000F;
pub const NVM_COMB_VER_MASK: c_uint = 0x00FF;
pub const NVM_MAJOR_SHIFT: c_int = 12;
pub const NVM_MINOR_SHIFT: c_int = 4;
pub const NVM_COMB_VER_SHFT: c_int = 8;
pub const NVM_VER_INVALID: c_uint = 0xFFFF;
pub const NVM_ETRACK_SHIFT: c_int = 16;
pub const NVM_ETRACK_VALID: c_uint = 0x8000;
pub const NVM_NEW_DEC_MASK: c_uint = 0x0F00;
pub const NVM_HEX_CONV: c_int = 16;
pub const NVM_HEX_TENS: c_int = 10;
pub const NVM_ETS_CFG: c_uint = 0x003E;
pub const NVM_ETS_LTHRES_DELTA_MASK: c_uint = 0x07C0;
pub const NVM_ETS_LTHRES_DELTA_SHIFT: c_int = 6;
pub const NVM_ETS_TYPE_MASK: c_uint = 0x0038;
pub const NVM_ETS_TYPE_SHIFT: c_int = 3;
pub const NVM_ETS_TYPE_EMC: c_uint = 0x000;
pub const NVM_ETS_NUM_SENSORS_MASK: c_uint = 0x0007;
pub const NVM_ETS_DATA_LOC_MASK: c_uint = 0x3C00;
pub const NVM_ETS_DATA_LOC_SHIFT: c_int = 10;
pub const NVM_ETS_DATA_INDEX_MASK: c_uint = 0x0300;
pub const NVM_ETS_DATA_INDEX_SHIFT: c_int = 8;
pub const NVM_ETS_DATA_HTHRESH_MASK: c_uint = 0x00FF;
pub const E1000_NVM_CFG_DONE_PORT_0: c_uint = 0x040000 /* MNG config cycle done */;
pub const E1000_NVM_CFG_DONE_PORT_1: c_uint = 0x080000 /* ...for second port */;
pub const E1000_NVM_CFG_DONE_PORT_2: c_uint = 0x100000 /* ...for third port */;
pub const E1000_NVM_CFG_DONE_PORT_3: c_uint = 0x200000 /* ...for fourth port */;

// Mask bits for fields in Word 0x24 of the NVM
pub const NVM_WORD24_COM_MDIO: c_uint = 0x0008 /* MDIO interface shared */;
pub const NVM_WORD24_EXT_MDIO: c_uint = 0x0004 /* MDIO accesses routed external */;
// Mask bits for fields in Word 0x0f of the NVM
pub const NVM_WORD0F_PAUSE_MASK: c_uint = 0x3000;
pub const NVM_WORD0F_ASM_DIR: c_uint = 0x2000;
// Mask bits for fields in Word 0x1a of the NVM
// length of string needed to store part num
pub const E1000_PBANUM_LENGTH: c_int = 11;
// For checksumming, the sum of all words in the NVM should equal 0xBABA.
pub const NVM_SUM: c_uint = 0xBABA;
pub const NVM_PBA_OFFSET_0: c_int = 8;
pub const NVM_PBA_OFFSET_1: c_int = 9;
pub const NVM_RESERVED_WORD: c_uint = 0xFFFF;
pub const NVM_PBA_PTR_GUARD: c_uint = 0xFAFA;
pub const NVM_WORD_SIZE_BASE_SHIFT: c_int = 6;
// NVM Commands - Microwire
// NVM Commands - SPI

pub const NVM_WRITE_OPCODE_SPI: c_uint = 0x02 /* NVM write opcode */;
pub const NVM_READ_OPCODE_SPI: c_uint = 0x03 /* NVM read opcode */;
pub const NVM_A8_OPCODE_SPI: c_uint = 0x08 /* opcode bit-3 = address bit-8 */;
pub const NVM_WREN_OPCODE_SPI: c_uint = 0x06 /* NVM set Write Enable latch */;
pub const NVM_RDSR_OPCODE_SPI: c_uint = 0x05 /* NVM read Status register */;
// SPI NVM Status Register
pub const NVM_STATUS_RDY_SPI: c_uint = 0x01;
// Word definitions for ID LED Settings
pub const ID_LED_RESERVED_0000: c_uint = 0x0000;
pub const ID_LED_RESERVED_FFFF: c_uint = 0xFFFF;

pub const ID_LED_DEF1_DEF2: c_uint = 0x1;
pub const ID_LED_DEF1_ON2: c_uint = 0x2;
pub const ID_LED_DEF1_OFF2: c_uint = 0x3;
pub const ID_LED_ON1_DEF2: c_uint = 0x4;
pub const ID_LED_ON1_ON2: c_uint = 0x5;
pub const ID_LED_ON1_OFF2: c_uint = 0x6;
pub const ID_LED_OFF1_DEF2: c_uint = 0x7;
pub const ID_LED_OFF1_ON2: c_uint = 0x8;
pub const ID_LED_OFF1_OFF2: c_uint = 0x9;
pub const IGP_ACTIVITY_LED_MASK: c_uint = 0xFFFFF0FF;
pub const IGP_ACTIVITY_LED_ENABLE: c_uint = 0x0300;
pub const IGP_LED3_MODE: c_uint = 0x07000000;
// PCI/PCI-X/PCI-EX Config space
pub const PCIE_DEVICE_CONTROL2: c_uint = 0x28;
pub const PCIE_DEVICE_CONTROL2_16ms: c_uint = 0x0005;
pub const PHY_REVISION_MASK: c_uint = 0xFFFFFFF0;
pub const MAX_PHY_REG_ADDRESS: c_uint = 0x1F  /* 5 bit address bus (0-0x1F) */;
pub const MAX_PHY_MULTI_PAGE_REG: c_uint = 0xF;
// Bit definitions for valid PHY IDs.
// I = Integrated
// E = External
//
pub const M88E1111_I_PHY_ID: c_uint = 0x01410CC0;
pub const M88E1112_E_PHY_ID: c_uint = 0x01410C90;
pub const I347AT4_E_PHY_ID: c_uint = 0x01410DC0;
pub const IGP03E1000_E_PHY_ID: c_uint = 0x02A80390;
pub const I82580_I_PHY_ID: c_uint = 0x015403A0;
pub const I350_I_PHY_ID: c_uint = 0x015403B0;
pub const M88_VENDOR: c_uint = 0x0141;
pub const I210_I_PHY_ID: c_uint = 0x01410C00;
pub const M88E1543_E_PHY_ID: c_uint = 0x01410EA0;
pub const M88E1512_E_PHY_ID: c_uint = 0x01410DD0;
pub const BCM54616_E_PHY_ID: c_uint = 0x03625D10;
// M88E1000 Specific Registers
pub const M88E1000_PHY_SPEC_CTRL: c_uint = 0x10  /* PHY Specific Control Register */;
pub const M88E1000_PHY_SPEC_STATUS: c_uint = 0x11  /* PHY Specific Status Register */;
pub const M88E1000_EXT_PHY_SPEC_CTRL: c_uint = 0x14  /* Extended PHY Specific Control */;
pub const M88E1000_PHY_PAGE_SELECT: c_uint = 0x1D  /* Reg 29 for page number setting */;
pub const M88E1000_PHY_GEN_CONTROL: c_uint = 0x1E  /* Its meaning depends on reg 29 */;
// M88E1000 PHY Specific Control Register
pub const M88E1000_PSCR_POLARITY_REVERSAL: c_uint = 0x0002 /* 1=Polarity Reversal enabled */;
// 1=CLK125 low, 0=CLK125 toggling
pub const M88E1000_PSCR_MDI_MANUAL_MODE: c_uint = 0x0000  /* MDI Crossover Mode bits 6:5 */;
// Manual MDI configuration
pub const M88E1000_PSCR_MDIX_MANUAL_MODE: c_uint = 0x0020  /* Manual MDIX configuration */;
// 1000BASE-T: Auto crossover, 100BASE-TX/10BASE-T: MDI Mode
pub const M88E1000_PSCR_AUTO_X_1000T: c_uint = 0x0040;
// Auto crossover enabled all speeds
pub const M88E1000_PSCR_AUTO_X_MODE: c_uint = 0x0060;
// 1=Enable Extended 10BASE-T distance (Lower 10BASE-T Rx Threshold
// 0=Normal 10BASE-T Rx Threshold
//
// 1=5-bit interface in 100BASE-TX, 0=MII interface in 100BASE-TX
pub const M88E1000_PSCR_ASSERT_CRS_ON_TX: c_uint = 0x0800 /* 1=Assert CRS on Transmit */;
// M88E1000 PHY Specific Status Register
pub const M88E1000_PSSR_REV_POLARITY: c_uint = 0x0002 /* 1=Polarity reversed */;
pub const M88E1000_PSSR_DOWNSHIFT: c_uint = 0x0020 /* 1=Downshifted */;
pub const M88E1000_PSSR_MDIX: c_uint = 0x0040 /* 1=MDIX; 0=MDI */;
// 0 = <50M
// 1 = 50-80M
// 2 = 80-110M
// 3 = 110-140M
// 4 = >140M
//
pub const M88E1000_PSSR_CABLE_LENGTH: c_uint = 0x0380;
pub const M88E1000_PSSR_SPEED: c_uint = 0xC000 /* Speed, bits 14:15 */;
pub const M88E1000_PSSR_1000MBS: c_uint = 0x8000 /* 10=1000Mbs */;
pub const M88E1000_PSSR_CABLE_LENGTH_SHIFT: c_int = 7;
// M88E1000 Extended PHY Specific Control Register
// 1 = Lost lock detect enabled.
// Will assert lost lock and bring
// link down if idle not seen
// within 1ms in 1000BASE-T
//
// Number of times we will attempt to autonegotiate before downshifting if we
// are the master
//
pub const M88E1000_EPSCR_MASTER_DOWNSHIFT_MASK: c_uint = 0x0C00;
pub const M88E1000_EPSCR_MASTER_DOWNSHIFT_1X: c_uint = 0x0000;
// Number of times we will attempt to autonegotiate before downshifting if we
// are the slave
//
pub const M88E1000_EPSCR_SLAVE_DOWNSHIFT_MASK: c_uint = 0x0300;
pub const M88E1000_EPSCR_SLAVE_DOWNSHIFT_1X: c_uint = 0x0100;
pub const M88E1000_EPSCR_TX_CLK_25: c_uint = 0x0070 /* 25  MHz TX_CLK */;
// Intel i347-AT4 Registers
pub const I347AT4_PCDL0: c_uint = 0x10 /* Pair 0 PHY Cable Diagnostics Length */;
pub const I347AT4_PCDL1: c_uint = 0x11 /* Pair 1 PHY Cable Diagnostics Length */;
pub const I347AT4_PCDL2: c_uint = 0x12 /* Pair 2 PHY Cable Diagnostics Length */;
pub const I347AT4_PCDL3: c_uint = 0x13 /* Pair 3 PHY Cable Diagnostics Length */;
pub const I347AT4_PCDC: c_uint = 0x15 /* PHY Cable Diagnostics Control */;
pub const I347AT4_PAGE_SELECT: c_uint = 0x16;
// i347-AT4 Extended PHY Specific Control Register
// Number of times we will attempt to autonegotiate before downshifting if we
// are the master
//
pub const I347AT4_PSCR_DOWNSHIFT_ENABLE: c_uint = 0x0800;
pub const I347AT4_PSCR_DOWNSHIFT_MASK: c_uint = 0x7000;
pub const I347AT4_PSCR_DOWNSHIFT_1X: c_uint = 0x0000;
pub const I347AT4_PSCR_DOWNSHIFT_2X: c_uint = 0x1000;
pub const I347AT4_PSCR_DOWNSHIFT_3X: c_uint = 0x2000;
pub const I347AT4_PSCR_DOWNSHIFT_4X: c_uint = 0x3000;
pub const I347AT4_PSCR_DOWNSHIFT_5X: c_uint = 0x4000;
pub const I347AT4_PSCR_DOWNSHIFT_6X: c_uint = 0x5000;
pub const I347AT4_PSCR_DOWNSHIFT_7X: c_uint = 0x6000;
pub const I347AT4_PSCR_DOWNSHIFT_8X: c_uint = 0x7000;
// i347-AT4 PHY Cable Diagnostics Control
pub const I347AT4_PCDC_CABLE_LENGTH_UNIT: c_uint = 0x0400 /* 0=cm 1=meters */;
// Marvell 1112 only registers
pub const M88E1112_VCT_DSP_DISTANCE: c_uint = 0x001A;
// M88EC018 Rev 2 specific DownShift settings
pub const M88EC018_EPSCR_DOWNSHIFT_COUNTER_MASK: c_uint = 0x0E00;
pub const M88EC018_EPSCR_DOWNSHIFT_COUNTER_5X: c_uint = 0x0800;
// MDI Control
pub const E1000_MDIC_DATA_MASK: c_uint = 0x0000FFFF;
pub const E1000_MDIC_REG_MASK: c_uint = 0x001F0000;
pub const E1000_MDIC_REG_SHIFT: c_int = 16;
pub const E1000_MDIC_PHY_MASK: c_uint = 0x03E00000;
pub const E1000_MDIC_PHY_SHIFT: c_int = 21;
pub const E1000_MDIC_OP_WRITE: c_uint = 0x04000000;
pub const E1000_MDIC_OP_READ: c_uint = 0x08000000;
pub const E1000_MDIC_READY: c_uint = 0x10000000;
pub const E1000_MDIC_INT_EN: c_uint = 0x20000000;
pub const E1000_MDIC_ERROR: c_uint = 0x40000000;
pub const E1000_MDIC_DEST: c_uint = 0x80000000;
// Thermal Sensor
pub const E1000_THSTAT_PWR_DOWN: c_uint = 0x00000001 /* Power Down Event */;
pub const E1000_THSTAT_LINK_THROTTLE: c_uint = 0x00000002 /* Link Speed Throttle Event */;
// Energy Efficient Ethernet
pub const E1000_IPCNFG_EEE_1G_AN: c_uint = 0x00000008  /* EEE Enable 1G AN */;
pub const E1000_IPCNFG_EEE_100M_AN: c_uint = 0x00000004  /* EEE Enable 100M AN */;
pub const E1000_EEER_TX_LPI_EN: c_uint = 0x00010000  /* EEE Tx LPI Enable */;
pub const E1000_EEER_RX_LPI_EN: c_uint = 0x00020000  /* EEE Rx LPI Enable */;
pub const E1000_EEER_FRC_AN: c_uint = 0x10000000  /* Enable EEE in loopback */;
pub const E1000_EEER_LPI_FC: c_uint = 0x00040000  /* EEE Enable on FC */;

pub const E1000_EEER_EEE_NEG: c_uint = 0x20000000  /* EEE capability nego */;
pub const E1000_EEE_LP_ADV_ADDR_I350: c_uint = 0x040F      /* EEE LP Advertisement */;

pub const E1000_MMDAC_FUNC_DATA: c_uint = 0x4000      /* Data, no post increment */;
pub const E1000_M88E1543_PAGE_ADDR: c_uint = 0x16       /* Page Offset Register */;
pub const E1000_M88E1543_EEE_CTRL_1: c_uint = 0x0;
pub const E1000_M88E1543_EEE_CTRL_1_MS: c_uint = 0x0001     /* EEE Master/Slave */;
pub const E1000_M88E1543_FIBER_CTRL: c_uint = 0x0;
pub const E1000_EEE_ADV_DEV_I354: c_int = 7;
pub const E1000_EEE_ADV_ADDR_I354: c_int = 60;

pub const E1000_PCS_STATUS_DEV_I354: c_int = 3;
pub const E1000_PCS_STATUS_ADDR_I354: c_int = 1;
pub const E1000_PCS_STATUS_TX_LPI_IND: c_uint = 0x0200     /* Tx in LPI state */;
pub const E1000_PCS_STATUS_RX_LPI_RCVD: c_uint = 0x0400;
pub const E1000_PCS_STATUS_TX_LPI_RCVD: c_uint = 0x0800;
// SerDes Control
pub const E1000_GEN_CTL_READY: c_uint = 0x80000000;
pub const E1000_GEN_CTL_ADDRESS_SHIFT: c_int = 8;
pub const E1000_GEN_POLL_TIMEOUT: c_int = 640;
pub const E1000_VFTA_ENTRY_SHIFT: c_int = 5;
pub const E1000_VFTA_ENTRY_MASK: c_uint = 0x7F;
pub const E1000_VFTA_ENTRY_BIT_SHIFT_MASK: c_uint = 0x1F;
// Tx Rate-Scheduler Config fields
pub const E1000_RTTBCNRC_RS_ENA: c_uint = 0x80000000;
pub const E1000_RTTBCNRC_RF_DEC_MASK: c_uint = 0x00003FFF;
pub const E1000_RTTBCNRC_RF_INT_SHIFT: c_int = 14;

pub const E1000_VLAPQF_QUEUE_MASK: c_uint = 0x03;
// TX Qav Control fields

// Fetch Time Delta - bits 31:16
//
// This field holds the value to be reduced from the launch time for
// fetch time decision. The FetchTimeDelta value is defined in 32 ns
// granularity.
//
// This field is 16 bits wide, and so the maximum value is:
//
// 65535 * 32 = 2097120 ~= 2.1 msec
//
// XXX: We are configuring the max value here since we couldn't come up
// with a reason for not doing so.
//

// TX Qav Credit Control fields
pub const E1000_TQAVCC_IDLESLOPE_MASK: c_uint = 0xFFFF;

// Transmit Descriptor Control fields

