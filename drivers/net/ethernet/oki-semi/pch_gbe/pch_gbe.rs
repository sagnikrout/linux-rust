//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/oki-semi/pch_gbe/pch_gbe.h
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
// Copyright (C) 1999 - 2010 Intel Corporation.
// Copyright (C) 2010 OKI SEMICONDUCTOR Co., LTD.
//
// This code was derived from the Intel e1000e Linux driver.
//

//
// pch_gbe_regs_mac_adr - Structure holding values of mac address registers
// @high	Denotes the 1st to 4th byte from the initial of MAC address
// @low		Denotes the 5th to 6th byte from the initial of MAC address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pch_gbe_regs_mac_adr {
    pub high: u32,
    pub low: u32,
}

//
// pch_udc_regs - Structure holding values of MAC registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pch_gbe_regs {
    pub INT_ST: u32,
    pub INT_EN: u32,
    pub MODE: u32,
    pub RESET: u32,
    pub TCPIP_ACC: u32,
    pub EX_LIST: u32,
    pub INT_ST_HOLD: u32,
    pub PHY_INT_CTRL: u32,
    pub MAC_RX_EN: u32,
    pub RX_FCTRL: u32,
    pub PAUSE_REQ: u32,
    pub RX_MODE: u32,
    pub TX_MODE: u32,
    pub RX_FIFO_ST: u32,
    pub TX_FIFO_ST: u32,
    pub TX_FID: u32,
    pub TX_RESULT: u32,
    pub PAUSE_PKT1: u32,
    pub PAUSE_PKT2: u32,
    pub PAUSE_PKT3: u32,
    pub PAUSE_PKT4: u32,
    pub PAUSE_PKT5: u32,
    pub reserve: [u32; 2],
    pub mac_adr: [pch_gbe_regs_mac_adr; 16],
    pub ADDR_MASK: u32,
    pub MIIM: u32,
    pub MAC_ADDR_LOAD: u32,
    pub RGMII_ST: u32,
    pub RGMII_CTRL: u32,
    pub reserve3: [u32; 3],
    pub DMA_CTRL: u32,
    pub reserve4: [u32; 3],
    pub RX_DSC_BASE: u32,
    pub RX_DSC_SIZE: u32,
    pub RX_DSC_HW_P: u32,
    pub RX_DSC_HW_P_HLD: u32,
    pub RX_DSC_SW_P: u32,
    pub reserve5: [u32; 3],
    pub TX_DSC_BASE: u32,
    pub TX_DSC_SIZE: u32,
    pub TX_DSC_HW_P: u32,
    pub TX_DSC_HW_P_HLD: u32,
    pub TX_DSC_SW_P: u32,
    pub reserve6: [u32; 3],
    pub RX_DMA_ST: u32,
    pub TX_DMA_ST: u32,
    pub reserve7: [u32; 2],
    pub WOL_ST: u32,
    pub WOL_CTRL: u32,
    pub WOL_ADDR_MASK: u32,
}

// Interrupt Status
// Interrupt Status Hold
// Interrupt Enable
pub const PCH_GBE_INT_RX_DMA_CMPLT: c_uint = 0x00000001 /* Receive DMA Transfer Complete */;
pub const PCH_GBE_INT_RX_VALID: c_uint = 0x00000002 /* MAC Normal Receive Complete */;
pub const PCH_GBE_INT_RX_FRAME_ERR: c_uint = 0x00000004 /* Receive frame error */;
pub const PCH_GBE_INT_RX_FIFO_ERR: c_uint = 0x00000008 /* Receive FIFO Overflow */;
pub const PCH_GBE_INT_RX_DMA_ERR: c_uint = 0x00000010 /* Receive DMA Transfer Error */;
pub const PCH_GBE_INT_RX_DSC_EMP: c_uint = 0x00000020 /* Receive Descriptor Empty */;
pub const PCH_GBE_INT_TX_CMPLT: c_uint = 0x00000100 /* MAC Transmission Complete */;
pub const PCH_GBE_INT_TX_DMA_CMPLT: c_uint = 0x00000200 /* DMA Transfer Complete */;
pub const PCH_GBE_INT_TX_FIFO_ERR: c_uint = 0x00000400 /* Transmission FIFO underflow. */;
pub const PCH_GBE_INT_TX_DMA_ERR: c_uint = 0x00000800 /* Transmission DMA Error */;
pub const PCH_GBE_INT_PAUSE_CMPLT: c_uint = 0x00001000 /* Pause Transmission complete */;
pub const PCH_GBE_INT_MIIM_CMPLT: c_uint = 0x00010000 /* MIIM I/F Read completion */;
pub const PCH_GBE_INT_PHY_INT: c_uint = 0x00100000 /* Interruption from PHY */;
pub const PCH_GBE_INT_WOL_DET: c_uint = 0x01000000 /* Wake On LAN Event detection. */;
pub const PCH_GBE_INT_TCPIP_ERR: c_uint = 0x10000000 /* TCP/IP Accelerator Error */;
// Mode
pub const PCH_GBE_MODE_MII_ETHER: c_uint = 0x00000000  /* GIGA Ethernet Mode [MII] */;
pub const PCH_GBE_MODE_GMII_ETHER: c_uint = 0x80000000  /* GIGA Ethernet Mode [GMII] */;
pub const PCH_GBE_MODE_HALF_DUPLEX: c_uint = 0x00000000  /* Duplex Mode [half duplex] */;
pub const PCH_GBE_MODE_FULL_DUPLEX: c_uint = 0x40000000  /* Duplex Mode [full duplex] */;
pub const PCH_GBE_MODE_FR_BST: c_uint = 0x04000000  /* Frame bursting is done */;
// Reset
pub const PCH_GBE_ALL_RST: c_uint = 0x80000000  /* All reset */;
pub const PCH_GBE_TX_RST: c_uint = 0x00008000  /* TX MAC, TX FIFO, TX DMA reset */;
pub const PCH_GBE_RX_RST: c_uint = 0x00004000  /* RX MAC, RX FIFO, RX DMA reset */;
// TCP/IP Accelerator Control
pub const PCH_GBE_EX_LIST_EN: c_uint = 0x00000008  /* External List Enable */;
pub const PCH_GBE_RX_TCPIPACC_OFF: c_uint = 0x00000004  /* RX TCP/IP ACC Disabled */;
pub const PCH_GBE_TX_TCPIPACC_EN: c_uint = 0x00000002  /* TX TCP/IP ACC Enable */;
pub const PCH_GBE_RX_TCPIPACC_EN: c_uint = 0x00000001  /* RX TCP/IP ACC Enable */;
// MAC RX Enable
pub const PCH_GBE_MRE_MAC_RX_EN: c_uint = 0x00000001      /* MAC Receive Enable */;
// RX Flow Control
pub const PCH_GBE_FL_CTRL_EN: c_uint = 0x80000000  /* Pause packet is enabled */;
// Pause Packet Request
pub const PCH_GBE_PS_PKT_RQ: c_uint = 0x80000000  /* Pause packet Request */;
// RX Mode
pub const PCH_GBE_ADD_FIL_EN: c_uint = 0x80000000  /* Address Filtering Enable */;
// Multicast Filtering Enable
pub const PCH_GBE_MLT_FIL_EN: c_uint = 0x40000000;
// Receive Almost Empty Threshold
pub const PCH_GBE_RH_ALM_EMP_4: c_uint = 0x00000000      /* 4 words */;
pub const PCH_GBE_RH_ALM_EMP_8: c_uint = 0x00004000      /* 8 words */;
pub const PCH_GBE_RH_ALM_EMP_16: c_uint = 0x00008000      /* 16 words */;
pub const PCH_GBE_RH_ALM_EMP_32: c_uint = 0x0000C000      /* 32 words */;
// Receive Almost Full Threshold
pub const PCH_GBE_RH_ALM_FULL_4: c_uint = 0x00000000      /* 4 words */;
pub const PCH_GBE_RH_ALM_FULL_8: c_uint = 0x00001000      /* 8 words */;
pub const PCH_GBE_RH_ALM_FULL_16: c_uint = 0x00002000      /* 16 words */;
pub const PCH_GBE_RH_ALM_FULL_32: c_uint = 0x00003000      /* 32 words */;
// RX FIFO Read Trigger Threshold
pub const PCH_GBE_RH_RD_TRG_4: c_uint = 0x00000000      /* 4 words */;
pub const PCH_GBE_RH_RD_TRG_8: c_uint = 0x00000200      /* 8 words */;
pub const PCH_GBE_RH_RD_TRG_16: c_uint = 0x00000400      /* 16 words */;
pub const PCH_GBE_RH_RD_TRG_32: c_uint = 0x00000600      /* 32 words */;
pub const PCH_GBE_RH_RD_TRG_64: c_uint = 0x00000800      /* 64 words */;
pub const PCH_GBE_RH_RD_TRG_128: c_uint = 0x00000A00      /* 128 words */;
pub const PCH_GBE_RH_RD_TRG_256: c_uint = 0x00000C00      /* 256 words */;
pub const PCH_GBE_RH_RD_TRG_512: c_uint = 0x00000E00      /* 512 words */;
// Receive Descriptor bit definitions
pub const PCH_GBE_RXD_ACC_STAT_BCAST: c_uint = 0x00000400;
pub const PCH_GBE_RXD_ACC_STAT_MCAST: c_uint = 0x00000200;
pub const PCH_GBE_RXD_ACC_STAT_UCAST: c_uint = 0x00000100;
pub const PCH_GBE_RXD_ACC_STAT_TCPIPOK: c_uint = 0x000000C0;
pub const PCH_GBE_RXD_ACC_STAT_IPOK: c_uint = 0x00000080;
pub const PCH_GBE_RXD_ACC_STAT_TCPOK: c_uint = 0x00000040;
pub const PCH_GBE_RXD_ACC_STAT_IP6ERR: c_uint = 0x00000020;
pub const PCH_GBE_RXD_ACC_STAT_OFLIST: c_uint = 0x00000010;
pub const PCH_GBE_RXD_ACC_STAT_TYPEIP: c_uint = 0x00000008;
pub const PCH_GBE_RXD_ACC_STAT_MACL: c_uint = 0x00000004;
pub const PCH_GBE_RXD_ACC_STAT_PPPOE: c_uint = 0x00000002;
pub const PCH_GBE_RXD_ACC_STAT_VTAGT: c_uint = 0x00000001;
pub const PCH_GBE_RXD_GMAC_STAT_PAUSE: c_uint = 0x0200;
pub const PCH_GBE_RXD_GMAC_STAT_MARBR: c_uint = 0x0100;
pub const PCH_GBE_RXD_GMAC_STAT_MARMLT: c_uint = 0x0080;
pub const PCH_GBE_RXD_GMAC_STAT_MARIND: c_uint = 0x0040;
pub const PCH_GBE_RXD_GMAC_STAT_MARNOTMT: c_uint = 0x0020;
pub const PCH_GBE_RXD_GMAC_STAT_TLONG: c_uint = 0x0010;
pub const PCH_GBE_RXD_GMAC_STAT_TSHRT: c_uint = 0x0008;
pub const PCH_GBE_RXD_GMAC_STAT_NOTOCTAL: c_uint = 0x0004;
pub const PCH_GBE_RXD_GMAC_STAT_NBLERR: c_uint = 0x0002;
pub const PCH_GBE_RXD_GMAC_STAT_CRCERR: c_uint = 0x0001;
// Transmit Descriptor bit definitions
pub const PCH_GBE_TXD_CTRL_TCPIP_ACC_OFF: c_uint = 0x0008;
pub const PCH_GBE_TXD_CTRL_ITAG: c_uint = 0x0004;
pub const PCH_GBE_TXD_CTRL_ICRC: c_uint = 0x0002;
pub const PCH_GBE_TXD_CTRL_APAD: c_uint = 0x0001;
pub const PCH_GBE_TXD_WORDS_SHIFT: c_int = 2;
pub const PCH_GBE_TXD_GMAC_STAT_CMPLT: c_uint = 0x2000;
pub const PCH_GBE_TXD_GMAC_STAT_ABT: c_uint = 0x1000;
pub const PCH_GBE_TXD_GMAC_STAT_EXCOL: c_uint = 0x0800;
pub const PCH_GBE_TXD_GMAC_STAT_SNGCOL: c_uint = 0x0400;
pub const PCH_GBE_TXD_GMAC_STAT_MLTCOL: c_uint = 0x0200;
pub const PCH_GBE_TXD_GMAC_STAT_CRSER: c_uint = 0x0100;
pub const PCH_GBE_TXD_GMAC_STAT_TLNG: c_uint = 0x0080;
pub const PCH_GBE_TXD_GMAC_STAT_TSHRT: c_uint = 0x0040;
pub const PCH_GBE_TXD_GMAC_STAT_LTCOL: c_uint = 0x0020;
pub const PCH_GBE_TXD_GMAC_STAT_TFUNDFLW: c_uint = 0x0010;
pub const PCH_GBE_TXD_GMAC_STAT_RTYCNT_MASK: c_uint = 0x000F;
// TX Mode
pub const PCH_GBE_TM_NO_RTRY: c_uint = 0x80000000 /* No Retransmission */;
pub const PCH_GBE_TM_LONG_PKT: c_uint = 0x40000000 /* Long Packt TX Enable */;
pub const PCH_GBE_TM_ST_AND_FD: c_uint = 0x20000000 /* Stare and Forward */;
pub const PCH_GBE_TM_SHORT_PKT: c_uint = 0x10000000 /* Short Packet TX Enable */;
pub const PCH_GBE_TM_LTCOL_RETX: c_uint = 0x08000000 /* Retransmission at Late Collision */;
// Frame Start Threshold
pub const PCH_GBE_TM_TH_TX_STRT_4: c_uint = 0x00000000    /* 4 words */;
pub const PCH_GBE_TM_TH_TX_STRT_8: c_uint = 0x00004000    /* 8 words */;
pub const PCH_GBE_TM_TH_TX_STRT_16: c_uint = 0x00008000    /* 16 words */;
pub const PCH_GBE_TM_TH_TX_STRT_32: c_uint = 0x0000C000    /* 32 words */;
// Transmit Almost Empty Threshold
pub const PCH_GBE_TM_TH_ALM_EMP_4: c_uint = 0x00000000    /* 4 words */;
pub const PCH_GBE_TM_TH_ALM_EMP_8: c_uint = 0x00000800    /* 8 words */;
pub const PCH_GBE_TM_TH_ALM_EMP_16: c_uint = 0x00001000    /* 16 words */;
pub const PCH_GBE_TM_TH_ALM_EMP_32: c_uint = 0x00001800    /* 32 words */;
pub const PCH_GBE_TM_TH_ALM_EMP_64: c_uint = 0x00002000    /* 64 words */;
pub const PCH_GBE_TM_TH_ALM_EMP_128: c_uint = 0x00002800    /* 128 words */;
pub const PCH_GBE_TM_TH_ALM_EMP_256: c_uint = 0x00003000    /* 256 words */;
pub const PCH_GBE_TM_TH_ALM_EMP_512: c_uint = 0x00003800    /* 512 words */;
// Transmit Almost Full Threshold
pub const PCH_GBE_TM_TH_ALM_FULL_4: c_uint = 0x00000000    /* 4 words */;
pub const PCH_GBE_TM_TH_ALM_FULL_8: c_uint = 0x00000200    /* 8 words */;
pub const PCH_GBE_TM_TH_ALM_FULL_16: c_uint = 0x00000400    /* 16 words */;
pub const PCH_GBE_TM_TH_ALM_FULL_32: c_uint = 0x00000600    /* 32 words */;
// RX FIFO Status
pub const PCH_GBE_RF_ALM_FULL: c_uint = 0x80000000  /* RX FIFO is almost full. */;
pub const PCH_GBE_RF_ALM_EMP: c_uint = 0x40000000  /* RX FIFO is almost empty. */;
pub const PCH_GBE_RF_RD_TRG: c_uint = 0x20000000  /* Become more than RH_RD_TRG. */;
pub const PCH_GBE_RF_STRWD: c_uint = 0x1FFE0000  /* The word count of RX FIFO. */;
pub const PCH_GBE_RF_RCVING: c_uint = 0x00010000  /* Stored in RX FIFO. */;
// MAC Address Mask
pub const PCH_GBE_BUSY: c_uint = 0x80000000;
// MIIM
pub const PCH_GBE_MIIM_OPER_WRITE: c_uint = 0x04000000;
pub const PCH_GBE_MIIM_OPER_READ: c_uint = 0x00000000;
pub const PCH_GBE_MIIM_OPER_READY: c_uint = 0x04000000;
pub const PCH_GBE_MIIM_PHY_ADDR_SHIFT: c_int = 21;
pub const PCH_GBE_MIIM_REG_ADDR_SHIFT: c_int = 16;
// RGMII Status
pub const PCH_GBE_LINK_UP: c_uint = 0x80000008;
pub const PCH_GBE_RXC_SPEED_MSK: c_uint = 0x00000006;
pub const PCH_GBE_RXC_SPEED_2_5M: c_uint = 0x00000000    /* 2.5MHz */;
pub const PCH_GBE_RXC_SPEED_25M: c_uint = 0x00000002    /* 25MHz  */;
pub const PCH_GBE_RXC_SPEED_125M: c_uint = 0x00000004    /* 100MHz */;
pub const PCH_GBE_DUPLEX_FULL: c_uint = 0x00000001;
// RGMII Control
pub const PCH_GBE_CRS_SEL: c_uint = 0x00000010;
pub const PCH_GBE_RGMII_RATE_125M: c_uint = 0x00000000;
pub const PCH_GBE_RGMII_RATE_25M: c_uint = 0x00000008;
pub const PCH_GBE_RGMII_RATE_2_5M: c_uint = 0x0000000C;
pub const PCH_GBE_RGMII_MODE_GMII: c_uint = 0x00000000;
pub const PCH_GBE_RGMII_MODE_RGMII: c_uint = 0x00000002;
pub const PCH_GBE_CHIP_TYPE_EXTERNAL: c_uint = 0x00000000;
pub const PCH_GBE_CHIP_TYPE_INTERNAL: c_uint = 0x00000001;
// DMA Control
pub const PCH_GBE_RX_DMA_EN: c_uint = 0x00000002   /* Enables Receive DMA */;
pub const PCH_GBE_TX_DMA_EN: c_uint = 0x00000001   /* Enables Transmission DMA */;
// RX DMA STATUS
pub const PCH_GBE_IDLE_CHECK: c_uint = 0xFFFFFFFE;
// Wake On LAN Status
pub const PCH_GBE_WLS_BR: c_uint = 0x00000008 /* Broadcas Address */;
pub const PCH_GBE_WLS_MLT: c_uint = 0x00000004 /* Multicast Address */;
// The Frame registered in Address Recognizer
pub const PCH_GBE_WLS_IND: c_uint = 0x00000002;
pub const PCH_GBE_WLS_MP: c_uint = 0x00000001 /* Magic packet Address */;
// Wake On LAN Control
pub const PCH_GBE_WLC_WOL_MODE: c_uint = 0x00010000;
pub const PCH_GBE_WLC_IGN_TLONG: c_uint = 0x00000100;
pub const PCH_GBE_WLC_IGN_TSHRT: c_uint = 0x00000080;
pub const PCH_GBE_WLC_IGN_OCTER: c_uint = 0x00000040;
pub const PCH_GBE_WLC_IGN_NBLER: c_uint = 0x00000020;
pub const PCH_GBE_WLC_IGN_CRCER: c_uint = 0x00000010;
pub const PCH_GBE_WLC_BR: c_uint = 0x00000008;
pub const PCH_GBE_WLC_MLT: c_uint = 0x00000004;
pub const PCH_GBE_WLC_IND: c_uint = 0x00000002;
pub const PCH_GBE_WLC_MP: c_uint = 0x00000001;
// Wake On LAN Address Mask
pub const PCH_GBE_WLA_BUSY: c_uint = 0x80000000;
// TX/RX descriptor defines
pub const PCH_GBE_MAX_TXD: c_int = 4096;
pub const PCH_GBE_DEFAULT_TXD: c_int = 256;
pub const PCH_GBE_MIN_TXD: c_int = 8;
pub const PCH_GBE_MAX_RXD: c_int = 4096;
pub const PCH_GBE_DEFAULT_RXD: c_int = 256;
pub const PCH_GBE_MIN_RXD: c_int = 8;
// Number of Transmit and Receive Descriptors must be a multiple of 8
pub const PCH_GBE_TX_DESC_MULTIPLE: c_int = 8;
pub const PCH_GBE_RX_DESC_MULTIPLE: c_int = 8;
// Read/Write operation is done through MII Management IF

// flow control values
pub const PCH_GBE_FC_NONE: c_int = 0;
pub const PCH_GBE_FC_RX_PAUSE: c_int = 1;
pub const PCH_GBE_FC_TX_PAUSE: c_int = 2;
pub const PCH_GBE_FC_FULL: c_int = 3;

//
// struct pch_gbe_mac_info - MAC information
// @addr[6]:		Store the MAC address
// @fc:			Mode of flow control
// @fc_autoneg:		Auto negotiation enable for flow control setting
// @tx_fc_enable:	Enable flag of Transmit flow control
// @max_frame_size:	Max transmit frame size
// @min_frame_size:	Min transmit frame size
// @autoneg:		Auto negotiation enable
// @link_speed:		Link speed
// @link_duplex:	Link duplex
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pch_gbe_mac_info {
    pub addr: [u8; 6],
    pub fc: u8,
    pub fc_autoneg: u8,
    pub tx_fc_enable: u8,
    pub max_frame_size: u32,
    pub min_frame_size: u32,
    pub autoneg: u8,
    pub link_speed: u16,
    pub link_duplex: u16,
}

//
// struct pch_gbe_phy_info - PHY information
// @addr:		PHY address
// @id:			PHY's identifier
// @revision:		PHY's revision
// @reset_delay_us:	HW reset delay time[us]
// @autoneg_advertised:	Autoneg advertised
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pch_gbe_phy_info {
    pub addr: u32,
    pub id: u32,
    pub revision: u32,
    pub reset_delay_us: u32,
    pub autoneg_advertised: u16,
}

// !
// @ingroup Gigabit Ether driver Layer
// @struct  pch_gbe_hw
// @brief   Hardware information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pch_gbe_hw {
    pub back: *mut c_void,
    pub reg: *mut pch_gbe_regs __iomem,
    pub miim_lock: spinlock_t,
    pub mac: pch_gbe_mac_info,
    pub phy: pch_gbe_phy_info,
}

//
// struct pch_gbe_rx_desc - Receive Descriptor
// @buffer_addr:	RX Frame Buffer Address
// @tcp_ip_status:	TCP/IP Accelerator Status
// @rx_words_eob:	RX word count and Byte position
// @gbec_status:	GMAC Status
// @dma_status:		DMA Status
// @reserved1:		Reserved
// @reserved2:		Reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pch_gbe_rx_desc {
    pub buffer_addr: u32,
    pub tcp_ip_status: u32,
    pub rx_words_eob: u16,
    pub gbec_status: u16,
    pub dma_status: u8,
    pub reserved1: u8,
    pub reserved2: u16,
}

//
// struct pch_gbe_tx_desc - Transmit Descriptor
// @buffer_addr:	TX Frame Buffer Address
// @length:		Data buffer length
// @reserved1:		Reserved
// @tx_words_eob:	TX word count and Byte position
// @tx_frame_ctrl:	TX Frame Control
// @dma_status:		DMA Status
// @reserved2:		Reserved
// @gbec_status:	GMAC Status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pch_gbe_tx_desc {
    pub buffer_addr: u32,
    pub length: u16,
    pub reserved1: u16,
    pub tx_words_eob: u16,
    pub tx_frame_ctrl: u16,
    pub dma_status: u8,
    pub reserved2: u8,
    pub gbec_status: u16,
}

//
// struct pch_gbe_buffer - Buffer information
// @skb:	pointer to a socket buffer
// @dma:	DMA address
// @time_stamp:	time stamp
// @length:	data size
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pch_gbe_buffer {
    pub skb: *mut sk_buff,
    pub dma: dma_addr_t,
    pub rx_buffer: *mut c_uchar,
    pub time_stamp: c_ulong,
    pub length: u16,
    pub mapped: bool,
}

//
// struct pch_gbe_tx_ring - tx ring information
// @desc:	pointer to the descriptor ring memory
// @dma:	physical address of the descriptor ring
// @size:	length of descriptor ring in bytes
// @count:	number of descriptors in the ring
// @next_to_use:	next descriptor to associate a buffer with
// @next_to_clean:	next descriptor to check for DD status bit
// @buffer_info:	array of buffer information structs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pch_gbe_tx_ring {
    pub desc: *mut pch_gbe_tx_desc,
    pub dma: dma_addr_t,
    pub size: c_uint,
    pub count: c_uint,
    pub next_to_use: c_uint,
    pub next_to_clean: c_uint,
    pub buffer_info: *mut pch_gbe_buffer,
}

//
// struct pch_gbe_rx_ring - rx ring information
// @desc:	pointer to the descriptor ring memory
// @dma:	physical address of the descriptor ring
// @size:	length of descriptor ring in bytes
// @count:	number of descriptors in the ring
// @next_to_use:	next descriptor to associate a buffer with
// @next_to_clean:	next descriptor to check for DD status bit
// @buffer_info:	array of buffer information structs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pch_gbe_rx_ring {
    pub desc: *mut pch_gbe_rx_desc,
    pub dma: dma_addr_t,
    pub rx_buff_pool: *mut c_uchar,
    pub rx_buff_pool_logic: dma_addr_t,
    pub rx_buff_pool_size: c_uint,
    pub size: c_uint,
    pub count: c_uint,
    pub next_to_use: c_uint,
    pub next_to_clean: c_uint,
    pub buffer_info: *mut pch_gbe_buffer,
}

//
// struct pch_gbe_hw_stats - Statistics counters collected by the MAC
// @rx_packets:		    total packets received
// @tx_packets:		    total packets transmitted
// @rx_bytes:		    total bytes received
// @tx_bytes:		    total bytes transmitted
// @rx_errors:		    bad packets received
// @tx_errors:		    packet transmit problems
// @rx_dropped:		    no space in Linux buffers
// @tx_dropped:		    no space available in Linux
// @multicast:		    multicast packets received
// @collisions:		    collisions
// @rx_crc_errors:	    received packet with crc error
// @rx_frame_errors:	    received frame alignment error
// @rx_alloc_buff_failed:   allocate failure of a receive buffer
// @tx_length_errors:	    transmit length error
// @tx_aborted_errors:	    transmit aborted error
// @tx_carrier_errors:	    transmit carrier error
// @tx_timeout_count:	    Number of transmit timeout
// @tx_restart_count:	    Number of transmit restert
// @intr_rx_dsc_empty_count:	Interrupt count of receive descriptor empty
// @intr_rx_frame_err_count:	Interrupt count of receive frame error
// @intr_rx_fifo_err_count:	Interrupt count of receive FIFO error
// @intr_rx_dma_err_count:	Interrupt count of receive DMA error
// @intr_tx_fifo_err_count:	Interrupt count of transmit FIFO error
// @intr_tx_dma_err_count:	Interrupt count of transmit DMA error
// @intr_tcpip_err_count:	Interrupt count of TCP/IP Accelerator
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pch_gbe_hw_stats {
    pub rx_packets: u32,
    pub tx_packets: u32,
    pub rx_bytes: u32,
    pub tx_bytes: u32,
    pub rx_errors: u32,
    pub tx_errors: u32,
    pub rx_dropped: u32,
    pub tx_dropped: u32,
    pub multicast: u32,
    pub collisions: u32,
    pub rx_crc_errors: u32,
    pub rx_frame_errors: u32,
    pub rx_alloc_buff_failed: u32,
    pub tx_length_errors: u32,
    pub tx_aborted_errors: u32,
    pub tx_carrier_errors: u32,
    pub tx_timeout_count: u32,
    pub tx_restart_count: u32,
    pub intr_rx_dsc_empty_count: u32,
    pub intr_rx_frame_err_count: u32,
    pub intr_rx_fifo_err_count: u32,
    pub intr_rx_dma_err_count: u32,
    pub intr_tx_fifo_err_count: u32,
    pub intr_tx_dma_err_count: u32,
    pub intr_tcpip_err_count: u32,
}

//
// struct pch_gbe_privdata - PCI Device ID driver data
// @phy_tx_clk_delay:		Bool, configure the PHY TX delay in software
// @phy_disable_hibernate:	Bool, disable PHY hibernation
// @platform_init:		Platform initialization callback, called from
// probe, prior to PHY initialization.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pch_gbe_privdata {
    pub phy_tx_clk_delay: bool,
    pub phy_disable_hibernate: bool,
    pub pdev): *mut *mut int (platform_init)(struct pci_dev,
}

//
// struct pch_gbe_adapter - board specific private data structure
// @stats_lock:	Spinlock structure for status
// @ethtool_lock:	Spinlock structure for ethtool
// @irq_sem:		Semaphore for interrupt
// @netdev:		Pointer of network device structure
// @pdev:		Pointer of pci device structure
// @polling_netdev:	Pointer of polling network device structure
// @napi:		NAPI structure
// @hw:			Pointer of hardware structure
// @stats:		Hardware status
// @reset_task:		Reset task
// @mii:		MII information structure
// @watchdog_timer:	Watchdog timer list
// @wake_up_evt:	Wake up event
// @config_space:	Configuration space
// @msg_enable:		Driver message level
// @led_status:		LED status
// @tx_ring:		Pointer of Tx descriptor ring structure
// @rx_ring:		Pointer of Rx descriptor ring structure
// @rx_buffer_len:	Receive buffer length
// @tx_queue_len:	Transmit queue length
// @pch_gbe_privdata:	PCI Device ID driver_data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pch_gbe_adapter {
    pub stats_lock: spinlock_t,
    pub ethtool_lock: spinlock_t,
    pub irq_sem: core::sync::atomic::AtomicI32,
    pub netdev: *mut net_device,
    pub pdev: *mut pci_dev,
    pub irq: c_int,
    pub polling_netdev: *mut net_device,
    pub napi: napi_struct,
    pub hw: pch_gbe_hw,
    pub stats: pch_gbe_hw_stats,
    pub reset_task: work_struct,
    pub mii: mii_if_info,
    pub watchdog_timer: timer_list,
    pub wake_up_evt: u32,
    pub config_space: *mut u32,
    pub led_status: c_ulong,
    pub tx_ring: *mut pch_gbe_tx_ring,
    pub rx_ring: *mut pch_gbe_rx_ring,
    pub rx_buffer_len: c_ulong,
    pub tx_queue_len: c_ulong,
    pub rx_stop_flag: bool,
    pub hwts_tx_en: c_int,
    pub hwts_rx_en: c_int,
    pub ptp_pdev: *mut pci_dev,
    pub pdata: *mut pch_gbe_privdata,
}

// pch_gbe_main.c
extern "C" {
    pub fn pch_gbe_up(adapter: *mut pch_gbe_adapter) -> c_int;
}
extern "C" {
    pub fn pch_gbe_down(adapter: *mut pch_gbe_adapter);
}
extern "C" {
    pub fn pch_gbe_reinit_locked(adapter: *mut pch_gbe_adapter);
}
extern "C" {
    pub fn pch_gbe_reset(adapter: *mut pch_gbe_adapter);
}
extern "C" {
    pub fn pch_gbe_update_stats(adapter: *mut pch_gbe_adapter);
}
// pch_gbe_param.c
extern "C" {
    pub fn pch_gbe_check_options(adapter: *mut pch_gbe_adapter);
}
// pch_gbe_ethtool.c
extern "C" {
    pub fn pch_gbe_set_ethtool_ops(netdev: *mut net_device);
}
// pch_gbe_mac.c
extern "C" {
    pub fn pch_gbe_mac_force_mac_fc(hw: *mut pch_gbe_hw) -> i32;
}
