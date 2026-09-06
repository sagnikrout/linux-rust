//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/b44.h
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

// Register layout. (These correspond to struct _bcmenettregs in bcm4400.)
pub const B44_DEVCTRL: c_uint = 0x0000UL /* Device Control */;
pub const DEVCTRL_MPM: c_uint = 0x00000040 /* Magic Packet PME Enable (B0 only) */;
pub const DEVCTRL_PFE: c_uint = 0x00000080 /* Pattern Filtering Enable */;
pub const DEVCTRL_IPP: c_uint = 0x00000400 /* Internal EPHY Present */;
pub const DEVCTRL_EPR: c_uint = 0x00008000 /* EPHY Reset */;
pub const DEVCTRL_PME: c_uint = 0x00001000 /* PHY Mode Enable */;
pub const DEVCTRL_PMCE: c_uint = 0x00002000 /* PHY Mode Clocks Enable */;
pub const DEVCTRL_PADDR: c_uint = 0x0007c000 /* PHY Address */;
pub const DEVCTRL_PADDR_SHIFT: c_int = 18;
pub const B44_BIST_STAT: c_uint = 0x000CUL /* Built-In Self-Test Status */;
pub const B44_WKUP_LEN: c_uint = 0x0010UL /* Wakeup Length */;
pub const WKUP_LEN_P0_MASK: c_uint = 0x0000007f /* Pattern 0 */;
pub const WKUP_LEN_D0: c_uint = 0x00000080;
pub const WKUP_LEN_P1_MASK: c_uint = 0x00007f00 /* Pattern 1 */;
pub const WKUP_LEN_P1_SHIFT: c_int = 8;
pub const WKUP_LEN_D1: c_uint = 0x00008000;
pub const WKUP_LEN_P2_MASK: c_uint = 0x007f0000 /* Pattern 2 */;
pub const WKUP_LEN_P2_SHIFT: c_int = 16;
pub const WKUP_LEN_D2: c_uint = 0x00000000;
pub const WKUP_LEN_P3_MASK: c_uint = 0x7f000000 /* Pattern 3 */;
pub const WKUP_LEN_P3_SHIFT: c_int = 24;
pub const WKUP_LEN_D3: c_uint = 0x80000000;
pub const WKUP_LEN_DISABLE: c_uint = 0x80808080;
pub const WKUP_LEN_ENABLE_TWO: c_uint = 0x80800000;
pub const WKUP_LEN_ENABLE_THREE: c_uint = 0x80000000;
pub const B44_ISTAT: c_uint = 0x0020UL /* Interrupt Status */;
pub const ISTAT_LS: c_uint = 0x00000020 /* Link Change (B0 only) */;
pub const ISTAT_PME: c_uint = 0x00000040 /* Power Management Event */;
pub const ISTAT_TO: c_uint = 0x00000080 /* General Purpose Timeout */;
pub const ISTAT_DSCE: c_uint = 0x00000400 /* Descriptor Error */;
pub const ISTAT_DATAE: c_uint = 0x00000800 /* Data Error */;
pub const ISTAT_DPE: c_uint = 0x00001000 /* Descr. Protocol Error */;
pub const ISTAT_RDU: c_uint = 0x00002000 /* Receive Descr. Underflow */;
pub const ISTAT_RFO: c_uint = 0x00004000 /* Receive FIFO Overflow */;
pub const ISTAT_TFU: c_uint = 0x00008000 /* Transmit FIFO Underflow */;
pub const ISTAT_RX: c_uint = 0x00010000 /* RX Interrupt */;
pub const ISTAT_TX: c_uint = 0x01000000 /* TX Interrupt */;
pub const ISTAT_EMAC: c_uint = 0x04000000 /* EMAC Interrupt */;
pub const ISTAT_MII_WRITE: c_uint = 0x08000000 /* MII Write Interrupt */;
pub const ISTAT_MII_READ: c_uint = 0x10000000 /* MII Read Interrupt */;

pub const B44_IMASK: c_uint = 0x0024UL /* Interrupt Mask */;

pub const B44_GPTIMER: c_uint = 0x0028UL /* General Purpose Timer */;
pub const B44_ADDR_LO: c_uint = 0x0088UL /* ENET Address Lo (B0 only) */;
pub const B44_ADDR_HI: c_uint = 0x008CUL /* ENET Address Hi (B0 only) */;
pub const B44_FILT_ADDR: c_uint = 0x0090UL /* ENET Filter Address */;
pub const B44_FILT_DATA: c_uint = 0x0094UL /* ENET Filter Data */;
pub const B44_TXBURST: c_uint = 0x00A0UL /* TX Max Burst Length */;
pub const B44_RXBURST: c_uint = 0x00A4UL /* RX Max Burst Length */;
pub const B44_MAC_CTRL: c_uint = 0x00A8UL /* MAC Control */;
pub const MAC_CTRL_CRC32_ENAB: c_uint = 0x00000001 /* CRC32 Generation Enable */;
pub const MAC_CTRL_PHY_PDOWN: c_uint = 0x00000004 /* Onchip EPHY Powerdown */;
pub const MAC_CTRL_PHY_EDET: c_uint = 0x00000008 /* Onchip EPHY Energy Detected */;
pub const MAC_CTRL_PHY_LEDCTRL: c_uint = 0x000000e0 /* Onchip EPHY LED Control */;
pub const MAC_CTRL_PHY_LEDCTRL_SHIFT: c_int = 5;
pub const B44_MAC_FLOW: c_uint = 0x00ACUL /* MAC Flow Control */;
pub const MAC_FLOW_RX_HI_WATER: c_uint = 0x000000ff /* Receive FIFO HI Water Mark */;
pub const MAC_FLOW_PAUSE_ENAB: c_uint = 0x00008000 /* Enable Pause Frame Generation */;
pub const B44_RCV_LAZY: c_uint = 0x0100UL /* Lazy Interrupt Control */;
pub const RCV_LAZY_TO_MASK: c_uint = 0x00ffffff /* Timeout */;
pub const RCV_LAZY_FC_MASK: c_uint = 0xff000000 /* Frame Count */;
pub const RCV_LAZY_FC_SHIFT: c_int = 24;
pub const B44_DMATX_CTRL: c_uint = 0x0200UL /* DMA TX Control */;
pub const DMATX_CTRL_ENABLE: c_uint = 0x00000001 /* Enable */;
pub const DMATX_CTRL_SUSPEND: c_uint = 0x00000002 /* Suepend Request */;
pub const DMATX_CTRL_LPBACK: c_uint = 0x00000004 /* Loopback Enable */;
pub const DMATX_CTRL_FAIRPRIOR: c_uint = 0x00000008 /* Fair Priority */;
pub const DMATX_CTRL_FLUSH: c_uint = 0x00000010 /* Flush Request */;
pub const B44_DMATX_ADDR: c_uint = 0x0204UL /* DMA TX Descriptor Ring Address */;
pub const B44_DMATX_PTR: c_uint = 0x0208UL /* DMA TX Last Posted Descriptor */;
pub const B44_DMATX_STAT: c_uint = 0x020CUL /* DMA TX Current Active Desc. + Status */;
pub const DMATX_STAT_CDMASK: c_uint = 0x00000fff /* Current Descriptor Mask */;
pub const DMATX_STAT_SMASK: c_uint = 0x0000f000 /* State Mask */;
pub const DMATX_STAT_SDISABLED: c_uint = 0x00000000 /* State Disabled */;
pub const DMATX_STAT_SACTIVE: c_uint = 0x00001000 /* State Active */;
pub const DMATX_STAT_SIDLE: c_uint = 0x00002000 /* State Idle Wait */;
pub const DMATX_STAT_SSTOPPED: c_uint = 0x00003000 /* State Stopped */;
pub const DMATX_STAT_SSUSP: c_uint = 0x00004000 /* State Suspend Pending */;
pub const DMATX_STAT_EMASK: c_uint = 0x000f0000 /* Error Mask */;
pub const DMATX_STAT_ENONE: c_uint = 0x00000000 /* Error None */;
pub const DMATX_STAT_EDPE: c_uint = 0x00010000 /* Error Desc. Protocol Error */;
pub const DMATX_STAT_EDFU: c_uint = 0x00020000 /* Error Data FIFO Underrun */;
pub const DMATX_STAT_EBEBR: c_uint = 0x00030000 /* Error Bus Error on Buffer Read */;
pub const DMATX_STAT_EBEDA: c_uint = 0x00040000 /* Error Bus Error on Desc. Access */;
pub const DMATX_STAT_FLUSHED: c_uint = 0x00100000 /* Flushed */;
pub const B44_DMARX_CTRL: c_uint = 0x0210UL /* DMA RX Control */;
pub const DMARX_CTRL_ENABLE: c_uint = 0x00000001 /* Enable */;
pub const DMARX_CTRL_ROMASK: c_uint = 0x000000fe /* Receive Offset Mask */;

pub const B44_DMARX_ADDR: c_uint = 0x0214UL /* DMA RX Descriptor Ring Address */;
pub const B44_DMARX_PTR: c_uint = 0x0218UL /* DMA RX Last Posted Descriptor */;
pub const B44_DMARX_STAT: c_uint = 0x021CUL /* DMA RX Current Active Desc. + Status */;
pub const DMARX_STAT_CDMASK: c_uint = 0x00000fff /* Current Descriptor Mask */;
pub const DMARX_STAT_SMASK: c_uint = 0x0000f000 /* State Mask */;
pub const DMARX_STAT_SDISABLED: c_uint = 0x00000000 /* State Disabled */;
pub const DMARX_STAT_SACTIVE: c_uint = 0x00001000 /* State Active */;
pub const DMARX_STAT_SIDLE: c_uint = 0x00002000 /* State Idle Wait */;
pub const DMARX_STAT_SSTOPPED: c_uint = 0x00003000 /* State Stopped */;
pub const DMARX_STAT_EMASK: c_uint = 0x000f0000 /* Error Mask */;
pub const DMARX_STAT_ENONE: c_uint = 0x00000000 /* Error None */;
pub const DMARX_STAT_EDPE: c_uint = 0x00010000 /* Error Desc. Protocol Error */;
pub const DMARX_STAT_EDFO: c_uint = 0x00020000 /* Error Data FIFO Overflow */;
pub const DMARX_STAT_EBEBW: c_uint = 0x00030000 /* Error Bus Error on Buffer Write */;
pub const DMARX_STAT_EBEDA: c_uint = 0x00040000 /* Error Bus Error on Desc. Access */;
pub const B44_DMAFIFO_AD: c_uint = 0x0220UL /* DMA FIFO Diag Address */;
pub const DMAFIFO_AD_OMASK: c_uint = 0x0000ffff /* Offset Mask */;
pub const DMAFIFO_AD_SMASK: c_uint = 0x000f0000 /* Select Mask */;
pub const DMAFIFO_AD_SXDD: c_uint = 0x00000000 /* Select Transmit DMA Data */;
pub const DMAFIFO_AD_SXDP: c_uint = 0x00010000 /* Select Transmit DMA Pointers */;
pub const DMAFIFO_AD_SRDD: c_uint = 0x00040000 /* Select Receive DMA Data */;
pub const DMAFIFO_AD_SRDP: c_uint = 0x00050000 /* Select Receive DMA Pointers */;
pub const DMAFIFO_AD_SXFD: c_uint = 0x00080000 /* Select Transmit FIFO Data */;
pub const DMAFIFO_AD_SXFP: c_uint = 0x00090000 /* Select Transmit FIFO Pointers */;
pub const DMAFIFO_AD_SRFD: c_uint = 0x000c0000 /* Select Receive FIFO Data */;
pub const DMAFIFO_AD_SRFP: c_uint = 0x000c0000 /* Select Receive FIFO Pointers */;
pub const B44_DMAFIFO_LO: c_uint = 0x0224UL /* DMA FIFO Diag Low Data */;
pub const B44_DMAFIFO_HI: c_uint = 0x0228UL /* DMA FIFO Diag High Data */;
pub const B44_RXCONFIG: c_uint = 0x0400UL /* EMAC RX Config */;
pub const RXCONFIG_DBCAST: c_uint = 0x00000001 /* Disable Broadcast */;
pub const RXCONFIG_ALLMULTI: c_uint = 0x00000002 /* Accept All Multicast */;
pub const RXCONFIG_NORX_WHILE_TX: c_uint = 0x00000004 /* Receive Disable While Transmitting */;
pub const RXCONFIG_PROMISC: c_uint = 0x00000008 /* Promiscuous Enable */;
pub const RXCONFIG_LPBACK: c_uint = 0x00000010 /* Loopback Enable */;
pub const RXCONFIG_FLOW: c_uint = 0x00000020 /* Flow Control Enable */;
pub const RXCONFIG_FLOW_ACCEPT: c_uint = 0x00000040 /* Accept Unicast Flow Control Frame */;
pub const RXCONFIG_RFILT: c_uint = 0x00000080 /* Reject Filter */;
pub const RXCONFIG_CAM_ABSENT: c_uint = 0x00000100 /* CAM Absent */;
pub const B44_RXMAXLEN: c_uint = 0x0404UL /* EMAC RX Max Packet Length */;
pub const B44_TXMAXLEN: c_uint = 0x0408UL /* EMAC TX Max Packet Length */;
pub const B44_MDIO_CTRL: c_uint = 0x0410UL /* EMAC MDIO Control */;
pub const MDIO_CTRL_MAXF_MASK: c_uint = 0x0000007f /* MDC Frequency */;
pub const MDIO_CTRL_PREAMBLE: c_uint = 0x00000080 /* MII Preamble Enable */;
pub const B44_MDIO_DATA: c_uint = 0x0414UL /* EMAC MDIO Data */;
pub const MDIO_DATA_DATA: c_uint = 0x0000ffff /* R/W Data */;
pub const MDIO_DATA_TA_MASK: c_uint = 0x00030000 /* Turnaround Value */;
pub const MDIO_DATA_TA_SHIFT: c_int = 16;
pub const MDIO_TA_VALID: c_int = 2;
pub const MDIO_DATA_RA_MASK: c_uint = 0x007c0000 /* Register Address */;
pub const MDIO_DATA_RA_SHIFT: c_int = 18;
pub const MDIO_DATA_PMD_MASK: c_uint = 0x0f800000 /* Physical Media Device */;
pub const MDIO_DATA_PMD_SHIFT: c_int = 23;
pub const MDIO_DATA_OP_MASK: c_uint = 0x30000000 /* Opcode */;
pub const MDIO_DATA_OP_SHIFT: c_int = 28;
pub const MDIO_OP_WRITE: c_int = 1;
pub const MDIO_OP_READ: c_int = 2;
pub const MDIO_DATA_SB_MASK: c_uint = 0xc0000000 /* Start Bits */;
pub const MDIO_DATA_SB_SHIFT: c_int = 30;
pub const MDIO_DATA_SB_START: c_uint = 0x40000000 /* Start Of Frame */;
pub const B44_EMAC_IMASK: c_uint = 0x0418UL /* EMAC Interrupt Mask */;
pub const B44_EMAC_ISTAT: c_uint = 0x041CUL /* EMAC Interrupt Status */;
pub const EMAC_INT_MII: c_uint = 0x00000001 /* MII MDIO Interrupt */;
pub const EMAC_INT_MIB: c_uint = 0x00000002 /* MIB Interrupt */;
pub const EMAC_INT_FLOW: c_uint = 0x00000003 /* Flow Control Interrupt */;
pub const B44_CAM_DATA_LO: c_uint = 0x0420UL /* EMAC CAM Data Low */;
pub const B44_CAM_DATA_HI: c_uint = 0x0424UL /* EMAC CAM Data High */;
pub const CAM_DATA_HI_VALID: c_uint = 0x00010000 /* Valid Bit */;
pub const B44_CAM_CTRL: c_uint = 0x0428UL /* EMAC CAM Control */;
pub const CAM_CTRL_ENABLE: c_uint = 0x00000001 /* CAM Enable */;
pub const CAM_CTRL_MSEL: c_uint = 0x00000002 /* Mask Select */;
pub const CAM_CTRL_READ: c_uint = 0x00000004 /* Read */;
pub const CAM_CTRL_WRITE: c_uint = 0x00000008 /* Read */;
pub const CAM_CTRL_INDEX_MASK: c_uint = 0x003f0000 /* Index Mask */;
pub const CAM_CTRL_INDEX_SHIFT: c_int = 16;
pub const CAM_CTRL_BUSY: c_uint = 0x80000000 /* CAM Busy */;
pub const B44_ENET_CTRL: c_uint = 0x042CUL /* EMAC ENET Control */;
pub const ENET_CTRL_ENABLE: c_uint = 0x00000001 /* EMAC Enable */;
pub const ENET_CTRL_DISABLE: c_uint = 0x00000002 /* EMAC Disable */;
pub const ENET_CTRL_SRST: c_uint = 0x00000004 /* EMAC Soft Reset */;
pub const ENET_CTRL_EPSEL: c_uint = 0x00000008 /* External PHY Select */;
pub const B44_TX_CTRL: c_uint = 0x0430UL /* EMAC TX Control */;
pub const TX_CTRL_DUPLEX: c_uint = 0x00000001 /* Full Duplex */;
pub const TX_CTRL_FMODE: c_uint = 0x00000002 /* Flow Mode */;
pub const TX_CTRL_SBENAB: c_uint = 0x00000004 /* Single Backoff Enable */;
pub const TX_CTRL_SMALL_SLOT: c_uint = 0x00000008 /* Small Slottime */;
pub const B44_TX_WMARK: c_uint = 0x0434UL /* EMAC TX Watermark */;
pub const B44_MIB_CTRL: c_uint = 0x0438UL /* EMAC MIB Control */;
pub const MIB_CTRL_CLR_ON_READ: c_uint = 0x00000001 /* Autoclear on Read */;
pub const B44_TX_GOOD_O: c_uint = 0x0500UL /* MIB TX Good Octets */;
pub const B44_TX_GOOD_P: c_uint = 0x0504UL /* MIB TX Good Packets */;
pub const B44_TX_O: c_uint = 0x0508UL /* MIB TX Octets */;
pub const B44_TX_P: c_uint = 0x050CUL /* MIB TX Packets */;
pub const B44_TX_BCAST: c_uint = 0x0510UL /* MIB TX Broadcast Packets */;
pub const B44_TX_MCAST: c_uint = 0x0514UL /* MIB TX Multicast Packets */;
pub const B44_TX_64: c_uint = 0x0518UL /* MIB TX <= 64 byte Packets */;
pub const B44_TX_65_127: c_uint = 0x051CUL /* MIB TX 65 to 127 byte Packets */;
pub const B44_TX_128_255: c_uint = 0x0520UL /* MIB TX 128 to 255 byte Packets */;
pub const B44_TX_256_511: c_uint = 0x0524UL /* MIB TX 256 to 511 byte Packets */;
pub const B44_TX_512_1023: c_uint = 0x0528UL /* MIB TX 512 to 1023 byte Packets */;
pub const B44_TX_1024_MAX: c_uint = 0x052CUL /* MIB TX 1024 to max byte Packets */;
pub const B44_TX_JABBER: c_uint = 0x0530UL /* MIB TX Jabber Packets */;
pub const B44_TX_OSIZE: c_uint = 0x0534UL /* MIB TX Oversize Packets */;
pub const B44_TX_FRAG: c_uint = 0x0538UL /* MIB TX Fragment Packets */;
pub const B44_TX_URUNS: c_uint = 0x053CUL /* MIB TX Underruns */;
pub const B44_TX_TCOLS: c_uint = 0x0540UL /* MIB TX Total Collisions */;
pub const B44_TX_SCOLS: c_uint = 0x0544UL /* MIB TX Single Collisions */;
pub const B44_TX_MCOLS: c_uint = 0x0548UL /* MIB TX Multiple Collisions */;
pub const B44_TX_ECOLS: c_uint = 0x054CUL /* MIB TX Excessive Collisions */;
pub const B44_TX_LCOLS: c_uint = 0x0550UL /* MIB TX Late Collisions */;
pub const B44_TX_DEFERED: c_uint = 0x0554UL /* MIB TX Defered Packets */;
pub const B44_TX_CLOST: c_uint = 0x0558UL /* MIB TX Carrier Lost */;
pub const B44_TX_PAUSE: c_uint = 0x055CUL /* MIB TX Pause Packets */;
pub const B44_RX_GOOD_O: c_uint = 0x0580UL /* MIB RX Good Octets */;
pub const B44_RX_GOOD_P: c_uint = 0x0584UL /* MIB RX Good Packets */;
pub const B44_RX_O: c_uint = 0x0588UL /* MIB RX Octets */;
pub const B44_RX_P: c_uint = 0x058CUL /* MIB RX Packets */;
pub const B44_RX_BCAST: c_uint = 0x0590UL /* MIB RX Broadcast Packets */;
pub const B44_RX_MCAST: c_uint = 0x0594UL /* MIB RX Multicast Packets */;
pub const B44_RX_64: c_uint = 0x0598UL /* MIB RX <= 64 byte Packets */;
pub const B44_RX_65_127: c_uint = 0x059CUL /* MIB RX 65 to 127 byte Packets */;
pub const B44_RX_128_255: c_uint = 0x05A0UL /* MIB RX 128 to 255 byte Packets */;
pub const B44_RX_256_511: c_uint = 0x05A4UL /* MIB RX 256 to 511 byte Packets */;
pub const B44_RX_512_1023: c_uint = 0x05A8UL /* MIB RX 512 to 1023 byte Packets */;
pub const B44_RX_1024_MAX: c_uint = 0x05ACUL /* MIB RX 1024 to max byte Packets */;
pub const B44_RX_JABBER: c_uint = 0x05B0UL /* MIB RX Jabber Packets */;
pub const B44_RX_OSIZE: c_uint = 0x05B4UL /* MIB RX Oversize Packets */;
pub const B44_RX_FRAG: c_uint = 0x05B8UL /* MIB RX Fragment Packets */;
pub const B44_RX_MISS: c_uint = 0x05BCUL /* MIB RX Missed Packets */;
pub const B44_RX_CRCA: c_uint = 0x05C0UL /* MIB RX CRC Align Errors */;
pub const B44_RX_USIZE: c_uint = 0x05C4UL /* MIB RX Undersize Packets */;
pub const B44_RX_CRC: c_uint = 0x05C8UL /* MIB RX CRC Errors */;
pub const B44_RX_ALIGN: c_uint = 0x05CCUL /* MIB RX Align Errors */;
pub const B44_RX_SYM: c_uint = 0x05D0UL /* MIB RX Symbol Errors */;
pub const B44_RX_PAUSE: c_uint = 0x05D4UL /* MIB RX Pause Packets */;
pub const B44_RX_NPAUSE: c_uint = 0x05D8UL /* MIB RX Non-Pause Packets */;
// 4400 PHY registers

pub const MII_AUXCTRL_DUPLEX: c_uint = 0x0001  /* Full Duplex */;
pub const MII_AUXCTRL_SPEED: c_uint = 0x0002  /* 1=100Mbps, 0=10Mbps */;
pub const MII_AUXCTRL_FORCED: c_uint = 0x0004	/* Forced 10/100 */;

pub const MII_ALEDCTRL_ALLMSK: c_uint = 0x7fff;

pub const MII_TLEDCTRL_ENABLE: c_uint = 0x0040;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_desc {
    pub ctrl: __le32,
    pub addr: __le32,
}

// There are only 12 bits in the DMA engine for descriptor offsetting
// so the table must be aligned on a boundary of this.
//
pub const DMA_TABLE_BYTES: c_int = 4096;
pub const DESC_CTRL_LEN: c_uint = 0x00001fff;
pub const DESC_CTRL_CMASK: c_uint = 0x0ff00000 /* Core specific bits */;
pub const DESC_CTRL_EOT: c_uint = 0x10000000 /* End of Table */;
pub const DESC_CTRL_IOC: c_uint = 0x20000000 /* Interrupt On Completion */;
pub const DESC_CTRL_EOF: c_uint = 0x40000000 /* End of Frame */;
pub const DESC_CTRL_SOF: c_uint = 0x80000000 /* Start of Frame */;
pub const RX_COPY_THRESHOLD: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_header {
    pub len: __le16,
    pub flags: __le16,
    pub pad: [__le16; 12],
}

pub const RX_HEADER_LEN: c_int = 28;
pub const RX_FLAG_OFIFO: c_uint = 0x00000001 /* FIFO Overflow */;
pub const RX_FLAG_CRCERR: c_uint = 0x00000002 /* CRC Error */;
pub const RX_FLAG_SERR: c_uint = 0x00000004 /* Receive Symbol Error */;
pub const RX_FLAG_ODD: c_uint = 0x00000008 /* Frame has odd number of nibbles */;
pub const RX_FLAG_LARGE: c_uint = 0x00000010 /* Frame is > RX MAX Length */;
pub const RX_FLAG_MCAST: c_uint = 0x00000020 /* Dest is Multicast Address */;
pub const RX_FLAG_BCAST: c_uint = 0x00000040 /* Dest is Broadcast Address */;
pub const RX_FLAG_MISS: c_uint = 0x00000080 /* Received due to promisc mode */;
pub const RX_FLAG_LAST: c_uint = 0x00000800 /* Last buffer in frame */;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ring_info {
    pub skb: *mut sk_buff,
    pub mapping: dma_addr_t,
}

pub const B44_MCAST_TABLE_SIZE: c_int = 32;
// no local phy regs, e.g: Broadcom switches pseudo-PHY

// no phy present at all
pub const B44_PHY_ADDR_NO_PHY: c_int = 31;
pub const B44_MDC_RATIO: c_int = 5000000;

// SW copy of device statistics, kept up to date by periodic timer
// which probes HW values. Check b44_stats_update if you mess with
// the layout
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b44_hw_stats {

    pub syncp: u64_stats_sync,
}

pub const B44_BOARDFLAG_ROBO: c_uint = 0x0010  /* Board has robo switch */;
pub const B44_BOARDFLAG_ADM: c_uint = 0x0080  /* Board has ADMtek switch */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b44 {
    pub lock: spinlock_t,
    pub istat: u32 imask,,
    pub tx_ring: *mut *mut dma_desc rx_ring,,
    pub tx_cons: u32 tx_prod,,
    pub rx_cons: u32 rx_prod,,
    pub rx_buffers: *mut ring_info,
    pub tx_buffers: *mut ring_info,
    pub napi: napi_struct,
    pub dma_offset: u32,
    pub flags: u32,
pub const B44_FLAG_B0_ANDLATER: c_uint = 0x00000001;
pub const B44_FLAG_BUGGY_TXPTR: c_uint = 0x00000002;
pub const B44_FLAG_REORDER_BUG: c_uint = 0x00000004;
pub const B44_FLAG_PAUSE_AUTO: c_uint = 0x00008000;
pub const B44_FLAG_FULL_DUPLEX: c_uint = 0x00010000;
pub const B44_FLAG_100_BASE_T: c_uint = 0x00020000;
pub const B44_FLAG_TX_PAUSE: c_uint = 0x00040000;
pub const B44_FLAG_RX_PAUSE: c_uint = 0x00080000;
pub const B44_FLAG_FORCE_LINK: c_uint = 0x00100000;
pub const B44_FLAG_ADV_10HALF: c_uint = 0x01000000;
pub const B44_FLAG_ADV_10FULL: c_uint = 0x02000000;
pub const B44_FLAG_ADV_100HALF: c_uint = 0x04000000;
pub const B44_FLAG_ADV_100FULL: c_uint = 0x08000000;
pub const B44_FLAG_EXTERNAL_PHY: c_uint = 0x10000000;
pub const B44_FLAG_RX_RING_HACK: c_uint = 0x20000000;
pub const B44_FLAG_TX_RING_HACK: c_uint = 0x40000000;
pub const B44_FLAG_WOL_ENABLE: c_uint = 0x80000000;
    pub msg_enable: u32,
    pub timer: timer_list,
    pub hw_stats: b44_hw_stats,
    pub sdev: *mut ssb_device,
    pub dev: *mut net_device,
    pub tx_ring_dma: dma_addr_t rx_ring_dma,,
    pub rx_pending: u32,
    pub tx_pending: u32,
    pub phy_addr: u8,
    pub force_copybreak: u8,
    pub mii_bus: *mut mii_bus,
    pub old_link: c_int,
    pub mii_if: mii_if_info,
}
