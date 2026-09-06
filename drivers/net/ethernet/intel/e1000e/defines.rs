//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/e1000e/defines.h
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
// Copyright(c) 1999 - 2018 Intel Corporation.
// Number of Transmit and Receive Descriptors must be a multiple of 8
pub const REQ_TX_DESCRIPTOR_MULTIPLE: c_int = 8;
pub const REQ_RX_DESCRIPTOR_MULTIPLE: c_int = 8;
// Definitions for power management and wakeup registers
// Wake Up Control
pub const E1000_WUC_APME: c_uint = 0x00000001	/* APM Enable */;
pub const E1000_WUC_PME_EN: c_uint = 0x00000002	/* PME Enable */;
pub const E1000_WUC_PME_STATUS: c_uint = 0x00000004	/* PME Status */;
pub const E1000_WUC_APMPME: c_uint = 0x00000008	/* Assert PME on APM Wakeup */;
pub const E1000_WUC_PHY_WAKE: c_uint = 0x00000100	/* if PHY supports wakeup */;
// Wake Up Filter Control
pub const E1000_WUFC_LNKC: c_uint = 0x00000001 /* Link Status Change Wakeup Enable */;
pub const E1000_WUFC_MAG: c_uint = 0x00000002 /* Magic Packet Wakeup Enable */;
pub const E1000_WUFC_EX: c_uint = 0x00000004 /* Directed Exact Wakeup Enable */;
pub const E1000_WUFC_MC: c_uint = 0x00000008 /* Directed Multicast Wakeup Enable */;
pub const E1000_WUFC_BC: c_uint = 0x00000010 /* Broadcast Wakeup Enable */;
pub const E1000_WUFC_ARP: c_uint = 0x00000020 /* ARP Request Packet Wakeup Enable */;
// Wake Up Status

// Extended Device Control
pub const E1000_CTRL_EXT_LPCD: c_uint = 0x00000004     /* LCD Power Cycle Done */;
pub const E1000_CTRL_EXT_DPG_EN: c_uint = 0x00000008 /* Dynamic Power Gating Enable */;
pub const E1000_CTRL_EXT_SDP3_DATA: c_uint = 0x00000080 /* Value of SW Definable Pin 3 */;
pub const E1000_CTRL_EXT_FORCE_SMBUS: c_uint = 0x00000800 /* Force SMBus mode */;
pub const E1000_CTRL_EXT_EE_RST: c_uint = 0x00002000 /* Reinitialize from EEPROM */;
pub const E1000_CTRL_EXT_SPD_BYPS: c_uint = 0x00008000 /* Speed Select Bypass */;
pub const E1000_CTRL_EXT_RO_DIS: c_uint = 0x00020000 /* Relaxed Ordering disable */;
pub const E1000_CTRL_EXT_DMA_DYN_CLK_EN: c_uint = 0x00080000 /* DMA Dynamic Clock Gating */;
pub const E1000_CTRL_EXT_LINK_MODE_MASK: c_uint = 0x00C00000;
pub const E1000_CTRL_EXT_LINK_MODE_PCIE_SERDES: c_uint = 0x00C00000;
pub const E1000_CTRL_EXT_EIAME: c_uint = 0x01000000;
pub const E1000_CTRL_EXT_DRV_LOAD: c_uint = 0x10000000 /* Driver loaded bit for FW */;
pub const E1000_CTRL_EXT_IAME: c_uint = 0x08000000 /* Int ACK Auto-mask */;
pub const E1000_CTRL_EXT_PBA_CLR: c_uint = 0x80000000 /* PBA Clear */;
pub const E1000_CTRL_EXT_LSECCK: c_uint = 0x00001000;
pub const E1000_CTRL_EXT_PHYPDEN: c_uint = 0x00100000;
// Receive Descriptor bit definitions
pub const E1000_RXD_STAT_DD: c_uint = 0x01    /* Descriptor Done */;
pub const E1000_RXD_STAT_EOP: c_uint = 0x02    /* End of Packet */;
pub const E1000_RXD_STAT_IXSM: c_uint = 0x04    /* Ignore checksum */;
pub const E1000_RXD_STAT_VP: c_uint = 0x08    /* IEEE VLAN Packet */;
pub const E1000_RXD_STAT_UDPCS: c_uint = 0x10    /* UDP xsum calculated */;
pub const E1000_RXD_STAT_TCPCS: c_uint = 0x20    /* TCP xsum calculated */;
pub const E1000_RXD_ERR_CE: c_uint = 0x01    /* CRC Error */;
pub const E1000_RXD_ERR_SE: c_uint = 0x02    /* Symbol Error */;
pub const E1000_RXD_ERR_SEQ: c_uint = 0x04    /* Sequence Error */;
pub const E1000_RXD_ERR_CXE: c_uint = 0x10    /* Carrier Extension Error */;
pub const E1000_RXD_ERR_TCPE: c_uint = 0x20    /* TCP/UDP Checksum Error */;
pub const E1000_RXD_ERR_IPE: c_uint = 0x40    /* IP Checksum Error */;
pub const E1000_RXD_ERR_RXE: c_uint = 0x80    /* Rx Data Error */;
pub const E1000_RXD_SPC_VLAN_MASK: c_uint = 0x0FFF  /* VLAN ID is in lower 12 bits */;
pub const E1000_RXDEXT_STATERR_TST: c_uint = 0x00000100	/* Time Stamp taken */;
pub const E1000_RXDEXT_STATERR_CE: c_uint = 0x01000000;
pub const E1000_RXDEXT_STATERR_SE: c_uint = 0x02000000;
pub const E1000_RXDEXT_STATERR_SEQ: c_uint = 0x04000000;
pub const E1000_RXDEXT_STATERR_CXE: c_uint = 0x10000000;
pub const E1000_RXDEXT_STATERR_RXE: c_uint = 0x80000000;
// mask to determine if packets should be dropped due to frame errors

// Same mask, but for extended and packet split descriptors

pub const E1000_MRQC_RSS_FIELD_MASK: c_uint = 0xFFFF0000;
pub const E1000_MRQC_RSS_FIELD_IPV4_TCP: c_uint = 0x00010000;
pub const E1000_MRQC_RSS_FIELD_IPV4: c_uint = 0x00020000;
pub const E1000_MRQC_RSS_FIELD_IPV6_TCP_EX: c_uint = 0x00040000;
pub const E1000_MRQC_RSS_FIELD_IPV6: c_uint = 0x00100000;
pub const E1000_MRQC_RSS_FIELD_IPV6_TCP: c_uint = 0x00200000;
pub const E1000_RXDPS_HDRSTAT_HDRSP: c_uint = 0x00008000;
// Management Control
pub const E1000_MANC_SMBUS_EN: c_uint = 0x00000001 /* SMBus Enabled - RO */;
pub const E1000_MANC_ASF_EN: c_uint = 0x00000002 /* ASF Enabled - RO */;
pub const E1000_MANC_ARP_EN: c_uint = 0x00002000 /* Enable ARP Request Filtering */;
pub const E1000_MANC_RCV_TCO_EN: c_uint = 0x00020000 /* Receive TCO Packets Enabled */;
pub const E1000_MANC_BLK_PHY_RST_ON_IDE: c_uint = 0x00040000 /* Block phy resets */;
// Enable MAC address filtering
pub const E1000_MANC_EN_MAC_ADDR_FILTER: c_uint = 0x00100000;
// Enable MNG packets to host memory
pub const E1000_MANC_EN_MNG2HOST: c_uint = 0x00200000;
pub const E1000_MANC2H_PORT_623: c_uint = 0x00000020 /* Port 0x26f */;
pub const E1000_MANC2H_PORT_664: c_uint = 0x00000040 /* Port 0x298 */;
pub const E1000_MDEF_PORT_623: c_uint = 0x00000800 /* Port 0x26f */;
pub const E1000_MDEF_PORT_664: c_uint = 0x00000400 /* Port 0x298 */;
// Receive Control
pub const E1000_RCTL_EN: c_uint = 0x00000002    /* enable */;
pub const E1000_RCTL_SBP: c_uint = 0x00000004    /* store bad packet */;
pub const E1000_RCTL_UPE: c_uint = 0x00000008    /* unicast promiscuous enable */;
pub const E1000_RCTL_MPE: c_uint = 0x00000010    /* multicast promiscuous enab */;
pub const E1000_RCTL_LPE: c_uint = 0x00000020    /* long packet enable */;
pub const E1000_RCTL_LBM_NO: c_uint = 0x00000000    /* no loopback mode */;
pub const E1000_RCTL_LBM_MAC: c_uint = 0x00000040    /* MAC loopback mode */;
pub const E1000_RCTL_LBM_TCVR: c_uint = 0x000000C0    /* tcvr loopback mode */;
pub const E1000_RCTL_DTYP_PS: c_uint = 0x00000400    /* Packet Split descriptor */;
pub const E1000_RCTL_RDMTS_HALF: c_uint = 0x00000000    /* Rx desc min threshold size */;
pub const E1000_RCTL_RDMTS_HEX: c_uint = 0x00010000;

pub const E1000_RCTL_MO_3: c_uint = 0x00003000    /* multicast offset 15:4 */;
pub const E1000_RCTL_BAM: c_uint = 0x00008000    /* broadcast enable */;
// these buffer sizes are valid if E1000_RCTL_BSEX is 0
pub const E1000_RCTL_SZ_2048: c_uint = 0x00000000    /* Rx buffer size 2048 */;
pub const E1000_RCTL_SZ_1024: c_uint = 0x00010000    /* Rx buffer size 1024 */;
pub const E1000_RCTL_SZ_512: c_uint = 0x00020000    /* Rx buffer size 512 */;
pub const E1000_RCTL_SZ_256: c_uint = 0x00030000    /* Rx buffer size 256 */;
// these buffer sizes are valid if E1000_RCTL_BSEX is 1
pub const E1000_RCTL_SZ_16384: c_uint = 0x00010000    /* Rx buffer size 16384 */;
pub const E1000_RCTL_SZ_8192: c_uint = 0x00020000    /* Rx buffer size 8192 */;
pub const E1000_RCTL_SZ_4096: c_uint = 0x00030000    /* Rx buffer size 4096 */;
pub const E1000_RCTL_VFE: c_uint = 0x00040000    /* vlan filter enable */;
pub const E1000_RCTL_CFIEN: c_uint = 0x00080000    /* canonical form enable */;
pub const E1000_RCTL_CFI: c_uint = 0x00100000    /* canonical form indicator */;
pub const E1000_RCTL_DPF: c_uint = 0x00400000    /* Discard Pause Frames */;
pub const E1000_RCTL_PMCF: c_uint = 0x00800000    /* pass MAC control frames */;
pub const E1000_RCTL_BSEX: c_uint = 0x02000000    /* Buffer size extension */;
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
pub const E1000_SWFW_CSR_SM: c_uint = 0x8;
// Device Control
pub const E1000_CTRL_FD: c_uint = 0x00000001  /* Full duplex.0=half; 1=full */;
pub const E1000_CTRL_GIO_MASTER_DISABLE: c_uint = 0x00000004 /*Blocks new Master requests */;
pub const E1000_CTRL_LRST: c_uint = 0x00000008  /* Link reset. 0=normal,1=reset */;
pub const E1000_CTRL_ASDE: c_uint = 0x00000020  /* Auto-speed detect enable */;
pub const E1000_CTRL_SLU: c_uint = 0x00000040  /* Set link up (Force Link) */;
pub const E1000_CTRL_ILOS: c_uint = 0x00000080  /* Invert Loss-Of Signal */;
pub const E1000_CTRL_SPD_SEL: c_uint = 0x00000300  /* Speed Select Mask */;
pub const E1000_CTRL_SPD_10: c_uint = 0x00000000  /* Force 10Mb */;
pub const E1000_CTRL_SPD_100: c_uint = 0x00000100  /* Force 100Mb */;
pub const E1000_CTRL_SPD_1000: c_uint = 0x00000200  /* Force 1Gb */;
pub const E1000_CTRL_FRCSPD: c_uint = 0x00000800  /* Force Speed */;
pub const E1000_CTRL_FRCDPX: c_uint = 0x00001000  /* Force Duplex */;
pub const E1000_CTRL_LANPHYPC_OVERRIDE: c_uint = 0x00010000 /* SW control of LANPHYPC */;
pub const E1000_CTRL_LANPHYPC_VALUE: c_uint = 0x00020000 /* SW value of LANPHYPC */;
pub const E1000_CTRL_MEHE: c_uint = 0x00080000  /* Memory Error Handling Enable */;
pub const E1000_CTRL_SWDPIN0: c_uint = 0x00040000  /* SWDPIN 0 value */;
pub const E1000_CTRL_SWDPIN1: c_uint = 0x00080000  /* SWDPIN 1 value */;
pub const E1000_CTRL_ADVD3WUC: c_uint = 0x00100000  /* D3 WUC */;
pub const E1000_CTRL_EN_PHY_PWR_MGMT: c_uint = 0x00200000 /* PHY PM enable */;
pub const E1000_CTRL_SWDPIO0: c_uint = 0x00400000  /* SWDPIN 0 Input or output */;
pub const E1000_CTRL_RST: c_uint = 0x04000000  /* Global reset */;
pub const E1000_CTRL_RFCE: c_uint = 0x08000000  /* Receive Flow Control enable */;
pub const E1000_CTRL_TFCE: c_uint = 0x10000000  /* Transmit flow control enable */;
pub const E1000_CTRL_VME: c_uint = 0x40000000  /* IEEE VLAN mode enable */;
pub const E1000_CTRL_PHY_RST: c_uint = 0x80000000  /* PHY Reset */;
pub const E1000_PCS_LCTL_FORCE_FCTRL: c_uint = 0x80;
pub const E1000_PCS_LSTS_AN_COMPLETE: c_uint = 0x10000;
// Device Status
pub const E1000_STATUS_FD: c_uint = 0x00000001      /* Full duplex.0=half,1=full */;
pub const E1000_STATUS_LU: c_uint = 0x00000002      /* Link up.0=no,1=link */;
pub const E1000_STATUS_FUNC_MASK: c_uint = 0x0000000C      /* PCI Function Mask */;
pub const E1000_STATUS_FUNC_SHIFT: c_int = 2;
pub const E1000_STATUS_FUNC_1: c_uint = 0x00000004      /* Function 1 */;
pub const E1000_STATUS_TXOFF: c_uint = 0x00000010      /* transmission paused */;
pub const E1000_STATUS_SPEED_MASK: c_uint = 0x000000C0;
pub const E1000_STATUS_SPEED_10: c_uint = 0x00000000      /* Speed 10Mb/s */;
pub const E1000_STATUS_SPEED_100: c_uint = 0x00000040      /* Speed 100Mb/s */;
pub const E1000_STATUS_SPEED_1000: c_uint = 0x00000080      /* Speed 1000Mb/s */;
pub const E1000_STATUS_LAN_INIT_DONE: c_uint = 0x00000200   /* Lan Init Completion by NVM */;
pub const E1000_STATUS_PHYRA: c_uint = 0x00000400      /* PHY Reset Asserted */;
pub const E1000_STATUS_GIO_MASTER_ENABLE: c_uint = 0x00080000	/* Master Req status */;
// PCIm function state
pub const E1000_STATUS_PCIM_STATE: c_uint = 0x40000000;
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
pub const E1000_PHY_LED0_MODE_MASK: c_uint = 0x00000007;
pub const E1000_PHY_LED0_IVRT: c_uint = 0x00000008;
pub const E1000_PHY_LED0_MASK: c_uint = 0x0000001F;
pub const E1000_LEDCTL_LED0_MODE_MASK: c_uint = 0x0000000F;
pub const E1000_LEDCTL_LED0_MODE_SHIFT: c_int = 0;
pub const E1000_LEDCTL_LED0_IVRT: c_uint = 0x00000040;
pub const E1000_LEDCTL_LED0_BLINK: c_uint = 0x00000080;
pub const E1000_LEDCTL_MODE_LINK_UP: c_uint = 0x2;
pub const E1000_LEDCTL_MODE_LED_ON: c_uint = 0xE;
pub const E1000_LEDCTL_MODE_LED_OFF: c_uint = 0xF;
// Transmit Descriptor bit definitions
pub const E1000_TXD_DTYP_D: c_uint = 0x00100000 /* Data Descriptor */;
pub const E1000_TXD_POPTS_IXSM: c_uint = 0x01       /* Insert IP checksum */;
pub const E1000_TXD_POPTS_TXSM: c_uint = 0x02       /* Insert TCP/UDP checksum */;
pub const E1000_TXD_CMD_EOP: c_uint = 0x01000000 /* End of Packet */;
pub const E1000_TXD_CMD_IFCS: c_uint = 0x02000000 /* Insert FCS (Ethernet CRC) */;
pub const E1000_TXD_CMD_IC: c_uint = 0x04000000 /* Insert Checksum */;
pub const E1000_TXD_CMD_RS: c_uint = 0x08000000 /* Report Status */;
pub const E1000_TXD_CMD_RPS: c_uint = 0x10000000 /* Report Packet Sent */;
pub const E1000_TXD_CMD_DEXT: c_uint = 0x20000000 /* Descriptor extension (0 = legacy) */;
pub const E1000_TXD_CMD_VLE: c_uint = 0x40000000 /* Add VLAN tag */;
pub const E1000_TXD_CMD_IDE: c_uint = 0x80000000 /* Enable Tidv register */;
pub const E1000_TXD_STAT_DD: c_uint = 0x00000001 /* Descriptor Done */;
pub const E1000_TXD_STAT_EC: c_uint = 0x00000002 /* Excess Collisions */;
pub const E1000_TXD_STAT_LC: c_uint = 0x00000004 /* Late Collisions */;
pub const E1000_TXD_STAT_TU: c_uint = 0x00000008 /* Transmit underrun */;
pub const E1000_TXD_CMD_TCP: c_uint = 0x01000000 /* TCP packet */;
pub const E1000_TXD_CMD_IP: c_uint = 0x02000000 /* IP packet */;
pub const E1000_TXD_CMD_TSE: c_uint = 0x04000000 /* TCP Seg enable */;
pub const E1000_TXD_STAT_TC: c_uint = 0x00000004 /* Tx Underrun */;
pub const E1000_TXD_EXTCMD_TSTAMP: c_uint = 0x00000010 /* IEEE1588 Timestamp packet */;
// Transmit Control
pub const E1000_TCTL_EN: c_uint = 0x00000002    /* enable Tx */;
pub const E1000_TCTL_PSP: c_uint = 0x00000008    /* pad short packets */;
pub const E1000_TCTL_CT: c_uint = 0x00000ff0    /* collision threshold */;
pub const E1000_TCTL_COLD: c_uint = 0x003ff000    /* collision distance */;
pub const E1000_TCTL_RTLC: c_uint = 0x01000000    /* Re-transmit on late collision */;
pub const E1000_TCTL_MULR: c_uint = 0x10000000    /* Multiple request support */;
// SerDes Control
pub const E1000_SCTL_DISABLE_SERDES_LOOPBACK: c_uint = 0x0400;
pub const E1000_SCTL_ENABLE_SERDES_LOOPBACK: c_uint = 0x0410;
// Receive Checksum Control
pub const E1000_RXCSUM_TUOFL: c_uint = 0x00000200   /* TCP / UDP checksum offload */;
pub const E1000_RXCSUM_IPPCSE: c_uint = 0x00001000   /* IP payload checksum enable */;
pub const E1000_RXCSUM_PCSD: c_uint = 0x00002000   /* packet checksum disabled */;
// Header split receive
pub const E1000_RFCTL_NFSW_DIS: c_uint = 0x00000040;
pub const E1000_RFCTL_NFSR_DIS: c_uint = 0x00000080;
pub const E1000_RFCTL_ACK_DIS: c_uint = 0x00001000;
pub const E1000_RFCTL_EXTEN: c_uint = 0x00008000;
pub const E1000_RFCTL_IPV6_EX_DIS: c_uint = 0x00010000;
pub const E1000_RFCTL_NEW_IPV6_EXT_DIS: c_uint = 0x00020000;
// Collision related configuration parameters
pub const E1000_COLLISION_THRESHOLD: c_int = 15;
pub const E1000_CT_SHIFT: c_int = 4;
pub const E1000_COLLISION_DISTANCE: c_int = 63;
pub const E1000_COLD_SHIFT: c_int = 12;
// Default values for the transmit IPG register
pub const DEFAULT_82543_TIPG_IPGT_COPPER: c_int = 8;
pub const E1000_TIPG_IPGT_MASK: c_uint = 0x000003FF;
pub const DEFAULT_82543_TIPG_IPGR1: c_int = 8;
pub const E1000_TIPG_IPGR1_SHIFT: c_int = 10;
pub const DEFAULT_82543_TIPG_IPGR2: c_int = 6;
pub const DEFAULT_80003ES2LAN_TIPG_IPGR2: c_int = 7;
pub const E1000_TIPG_IPGR2_SHIFT: c_int = 20;
pub const MAX_JUMBO_FRAME_SIZE: c_uint = 0x3F00;
pub const E1000_TX_PTR_GAP: c_uint = 0x1F;
// Extended Configuration Control and Size
pub const E1000_EXTCNF_CTRL_MDIO_SW_OWNERSHIP: c_uint = 0x00000020;
pub const E1000_EXTCNF_CTRL_LCD_WRITE_ENABLE: c_uint = 0x00000001;
pub const E1000_EXTCNF_CTRL_OEM_WRITE_ENABLE: c_uint = 0x00000008;
pub const E1000_EXTCNF_CTRL_SWFLAG: c_uint = 0x00000020;
pub const E1000_EXTCNF_CTRL_GATE_PHY_CFG: c_uint = 0x00000080;
pub const E1000_EXTCNF_SIZE_EXT_PCIE_LENGTH_MASK: c_uint = 0x00FF0000;
pub const E1000_EXTCNF_SIZE_EXT_PCIE_LENGTH_SHIFT: c_int = 16;
pub const E1000_EXTCNF_CTRL_EXT_CNF_POINTER_MASK: c_uint = 0x0FFF0000;
pub const E1000_EXTCNF_CTRL_EXT_CNF_POINTER_SHIFT: c_int = 16;
pub const E1000_PHY_CTRL_D0A_LPLU: c_uint = 0x00000002;
pub const E1000_PHY_CTRL_NOND0A_LPLU: c_uint = 0x00000004;
pub const E1000_PHY_CTRL_NOND0A_GBE_DISABLE: c_uint = 0x00000008;
pub const E1000_PHY_CTRL_GBE_DISABLE: c_uint = 0x00000040;
pub const E1000_KABGTXD_BGSQLBIAS: c_uint = 0x00050000;
// Low Power IDLE Control

// PBA constants
pub const E1000_PBA_8K: c_uint = 0x0008    /* 8KB */;
pub const E1000_PBA_16K: c_uint = 0x0010    /* 16KB */;
pub const E1000_PBA_RXA_MASK: c_uint = 0xFFFF;

// Uncorrectable/correctable ECC Error counts and enable bits
pub const E1000_PBECCSTS_CORR_ERR_CNT_MASK: c_uint = 0x000000FF;
pub const E1000_PBECCSTS_UNCORR_ERR_CNT_MASK: c_uint = 0x0000FF00;
pub const E1000_PBECCSTS_UNCORR_ERR_CNT_SHIFT: c_int = 8;
pub const E1000_PBECCSTS_ECC_ENABLE: c_uint = 0x00010000;
pub const IFS_MAX: c_int = 80;
pub const IFS_MIN: c_int = 40;
pub const IFS_RATIO: c_int = 4;
pub const IFS_STEP: c_int = 10;
pub const MIN_NUM_XMITS: c_int = 1000;
// SW Semaphore Register
pub const E1000_SWSM_SMBI: c_uint = 0x00000001 /* Driver Semaphore bit */;
pub const E1000_SWSM_SWESMBI: c_uint = 0x00000002 /* FW Semaphore bit */;
pub const E1000_SWSM_DRV_LOAD: c_uint = 0x00000008 /* Driver Loaded Bit */;
pub const E1000_SWSM2_LOCK: c_uint = 0x00000002 /* Secondary driver semaphore bit */;
// Interrupt Cause Read
pub const E1000_ICR_TXDW: c_uint = 0x00000001 /* Transmit desc written back */;
pub const E1000_ICR_LSC: c_uint = 0x00000004 /* Link Status Change */;
pub const E1000_ICR_RXSEQ: c_uint = 0x00000008 /* Rx sequence error */;
pub const E1000_ICR_RXDMT0: c_uint = 0x00000010 /* Rx desc min. threshold (0) */;
pub const E1000_ICR_RXO: c_uint = 0x00000040 /* Receiver Overrun */;
pub const E1000_ICR_RXT0: c_uint = 0x00000080 /* Rx timer intr (ring 0) */;
pub const E1000_ICR_MDAC: c_uint = 0x00000200 /* MDIO Access Complete */;
pub const E1000_ICR_SRPD: c_uint = 0x00010000 /* Small Receive Packet Detected */;
pub const E1000_ICR_ACK: c_uint = 0x00020000 /* Receive ACK Frame Detected */;
pub const E1000_ICR_MNG: c_uint = 0x00040000 /* Manageability Event Detected */;
pub const E1000_ICR_ECCER: c_uint = 0x00400000 /* Uncorrectable ECC Error */;
// If this bit asserted, the driver should claim the interrupt
pub const E1000_ICR_INT_ASSERTED: c_uint = 0x80000000;
pub const E1000_ICR_RXQ0: c_uint = 0x00100000 /* Rx Queue 0 Interrupt */;
pub const E1000_ICR_RXQ1: c_uint = 0x00200000 /* Rx Queue 1 Interrupt */;
pub const E1000_ICR_TXQ0: c_uint = 0x00400000 /* Tx Queue 0 Interrupt */;
pub const E1000_ICR_TXQ1: c_uint = 0x00800000 /* Tx Queue 1 Interrupt */;
pub const E1000_ICR_OTHER: c_uint = 0x01000000 /* Other Interrupt */;
// PBA ECC Register
pub const E1000_PBA_ECC_COUNTER_MASK: c_uint = 0xFFF00000 /* ECC counter mask */;

pub const E1000_PBA_ECC_CORR_EN: c_uint = 0x00000001 /* ECC correction enable */;
pub const E1000_PBA_ECC_STAT_CLR: c_uint = 0x00000002 /* Clear ECC error counter */;
pub const E1000_PBA_ECC_INT_EN: c_uint = 0x00000004 /* Enable ICR bit 5 for ECC */;
// This defines the bits that are set in the Interrupt Mask
// Set/Read Register.  Each bit is documented below:
// o RXT0   = Receiver Timer Interrupt (ring 0)
// o TXDW   = Transmit Descriptor Written Back
// o RXDMT0 = Receive Descriptor Minimum Threshold hit (ring 0)
// o RXSEQ  = Receive Sequence Error
// o LSC    = Link Status Change
//

// These are all of the events related to the OTHER interrupt.
//

// Interrupt Mask Set

// Interrupt Cause Set

// Transmit Descriptor Control
pub const E1000_TXDCTL_PTHRESH: c_uint = 0x0000003F /* TXDCTL Prefetch Threshold */;
pub const E1000_TXDCTL_HTHRESH: c_uint = 0x00003F00 /* TXDCTL Host Threshold */;
pub const E1000_TXDCTL_WTHRESH: c_uint = 0x003F0000 /* TXDCTL Writeback Threshold */;
pub const E1000_TXDCTL_GRAN: c_uint = 0x01000000 /* TXDCTL Granularity */;
pub const E1000_TXDCTL_FULL_TX_DESC_WB: c_uint = 0x01010000 /* GRAN=1, WTHRESH=1 */;
pub const E1000_TXDCTL_MAX_TX_DESC_PREFETCH: c_uint = 0x0100001F /* GRAN=1, PTHRESH=31 */;
// Enable the counting of desc. still to be processed.
pub const E1000_TXDCTL_COUNT_DESC: c_uint = 0x00400000;
// Flow Control Constants
pub const FLOW_CONTROL_ADDRESS_LOW: c_uint = 0x00C28001;
pub const FLOW_CONTROL_ADDRESS_HIGH: c_uint = 0x00000100;
pub const FLOW_CONTROL_TYPE: c_uint = 0x8808;
// 802.1q VLAN Packet Size

// Receive Address
// Number of high/low register pairs in the RAR. The RAR (Receive Address
// Registers) holds the directed and multicast addresses that we monitor.
// Technically, we have 16 spots.  However, we reserve one of these spots
// (RAR[15]) for our directed address used by controllers with
// manageability enabled, allowing us room for 15 multicast addresses.
//
pub const E1000_RAR_ENTRIES: c_int = 15;
pub const E1000_RAH_AV: c_uint = 0x80000000        /* Receive descriptor valid */;
pub const E1000_RAL_MAC_ADDR_LEN: c_int = 4;
pub const E1000_RAH_MAC_ADDR_LEN: c_int = 2;
// Error Codes
pub const E1000_ERR_NVM: c_int = 1;
pub const E1000_ERR_PHY: c_int = 2;
pub const E1000_ERR_CONFIG: c_int = 3;
pub const E1000_ERR_PARAM: c_int = 4;
pub const E1000_ERR_MAC_INIT: c_int = 5;
pub const E1000_ERR_PHY_TYPE: c_int = 6;
pub const E1000_ERR_RESET: c_int = 9;
pub const E1000_ERR_MASTER_REQUESTS_PENDING: c_int = 10;
pub const E1000_ERR_HOST_INTERFACE_COMMAND: c_int = 11;
pub const E1000_BLK_PHY_RESET: c_int = 12;
pub const E1000_ERR_SWFW_SYNC: c_int = 13;
pub const E1000_NOT_IMPLEMENTED: c_int = 14;
pub const E1000_ERR_INVALID_ARGUMENT: c_int = 16;
pub const E1000_ERR_NO_SPACE: c_int = 17;
pub const E1000_ERR_NVM_PBA_SECTION: c_int = 18;
// Loop limit on how long we wait for auto-negotiation to complete
pub const FIBER_LINK_UP_LIMIT: c_int = 50;
pub const COPPER_LINK_UP_LIMIT: c_int = 10;
pub const PHY_AUTO_NEG_LIMIT: c_int = 45;
pub const PHY_FORCE_LIMIT: c_int = 20;
// Number of 100 microseconds we wait for PCI Express master disable
pub const MASTER_DISABLE_TIMEOUT: c_int = 800;
// Number of milliseconds we wait for PHY configuration done after MAC reset
pub const PHY_CFG_TIMEOUT: c_int = 100;
// Number of 2 milliseconds we wait for acquiring MDIO ownership.
pub const MDIO_OWNERSHIP_TIMEOUT: c_int = 10;
// Number of milliseconds for NVM auto read done after MAC reset.
pub const AUTO_READ_DONE_TIMEOUT: c_int = 10;
// Flow Control
pub const E1000_FCRTH_RTH: c_uint = 0x0000FFF8     /* Mask Bits[15:3] for RTH */;
pub const E1000_FCRTL_RTL: c_uint = 0x0000FFF8     /* Mask Bits[15:3] for RTL */;
pub const E1000_FCRTL_XONE: c_uint = 0x80000000     /* Enable XON frame transmission */;
// Transmit Configuration Word
pub const E1000_TXCW_FD: c_uint = 0x00000020        /* TXCW full duplex */;
pub const E1000_TXCW_PAUSE: c_uint = 0x00000080        /* TXCW sym pause request */;
pub const E1000_TXCW_ASM_DIR: c_uint = 0x00000100        /* TXCW astm pause direction */;
pub const E1000_TXCW_PAUSE_MASK: c_uint = 0x00000180        /* TXCW pause request mask */;
pub const E1000_TXCW_ANE: c_uint = 0x80000000        /* Auto-neg enable */;
// Receive Configuration Word
pub const E1000_RXCW_CW: c_uint = 0x0000ffff        /* RxConfigWord mask */;
pub const E1000_RXCW_IV: c_uint = 0x08000000        /* Receive config invalid */;
pub const E1000_RXCW_C: c_uint = 0x20000000        /* Receive config */;
pub const E1000_RXCW_SYNCH: c_uint = 0x40000000        /* Receive config synch */;
// HH Time Sync
pub const E1000_TSYNCTXCTL_MAX_ALLOWED_DLY_MASK: c_uint = 0x0000F000 /* max delay */;
pub const E1000_TSYNCTXCTL_SYNC_COMP: c_uint = 0x40000000 /* sync complete */;
pub const E1000_TSYNCTXCTL_START_SYNC: c_uint = 0x80000000 /* initiate sync */;
pub const E1000_TSYNCTXCTL_VALID: c_uint = 0x00000001 /* Tx timestamp valid */;
pub const E1000_TSYNCTXCTL_ENABLED: c_uint = 0x00000010 /* enable Tx timestamping */;
pub const E1000_TSYNCRXCTL_VALID: c_uint = 0x00000001 /* Rx timestamp valid */;
pub const E1000_TSYNCRXCTL_TYPE_MASK: c_uint = 0x0000000E /* Rx type mask */;
pub const E1000_TSYNCRXCTL_TYPE_L2_V2: c_uint = 0x00;
pub const E1000_TSYNCRXCTL_TYPE_L4_V1: c_uint = 0x02;
pub const E1000_TSYNCRXCTL_TYPE_L2_L4_V2: c_uint = 0x04;
pub const E1000_TSYNCRXCTL_TYPE_ALL: c_uint = 0x08;
pub const E1000_TSYNCRXCTL_TYPE_EVENT_V2: c_uint = 0x0A;
pub const E1000_TSYNCRXCTL_ENABLED: c_uint = 0x00000010 /* enable Rx timestamping */;
pub const E1000_TSYNCRXCTL_SYSCFI: c_uint = 0x00000020 /* Sys clock frequency */;
pub const E1000_RXMTRL_PTP_V1_SYNC_MESSAGE: c_uint = 0x00000000;
pub const E1000_RXMTRL_PTP_V1_DELAY_REQ_MESSAGE: c_uint = 0x00010000;
pub const E1000_RXMTRL_PTP_V2_SYNC_MESSAGE: c_uint = 0x00000000;
pub const E1000_RXMTRL_PTP_V2_DELAY_REQ_MESSAGE: c_uint = 0x01000000;
pub const E1000_TIMINCA_INCPERIOD_SHIFT: c_int = 24;
pub const E1000_TIMINCA_INCVALUE_MASK: c_uint = 0x00FFFFFF;
// PCI Express Control
pub const E1000_GCR_RXD_NO_SNOOP: c_uint = 0x00000001;
pub const E1000_GCR_RXDSCW_NO_SNOOP: c_uint = 0x00000002;
pub const E1000_GCR_RXDSCR_NO_SNOOP: c_uint = 0x00000004;
pub const E1000_GCR_TXD_NO_SNOOP: c_uint = 0x00000008;
pub const E1000_GCR_TXDSCW_NO_SNOOP: c_uint = 0x00000010;
pub const E1000_GCR_TXDSCR_NO_SNOOP: c_uint = 0x00000020;

// NVM Control
pub const E1000_EECD_SK: c_uint = 0x00000001 /* NVM Clock */;
pub const E1000_EECD_CS: c_uint = 0x00000002 /* NVM Chip Select */;
pub const E1000_EECD_DI: c_uint = 0x00000004 /* NVM Data In */;
pub const E1000_EECD_DO: c_uint = 0x00000008 /* NVM Data Out */;
pub const E1000_EECD_REQ: c_uint = 0x00000040 /* NVM Access Request */;
pub const E1000_EECD_GNT: c_uint = 0x00000080 /* NVM Access Grant */;
pub const E1000_EECD_PRES: c_uint = 0x00000100 /* NVM Present */;
pub const E1000_EECD_SIZE: c_uint = 0x00000200 /* NVM Size (0=64 word 1=256 word) */;
// NVM Addressing bits based on type (0-small, 1-large)
pub const E1000_EECD_ADDR_BITS: c_uint = 0x00000400;

pub const E1000_EECD_AUTO_RD: c_uint = 0x00000200  /* NVM Auto Read done */;
pub const E1000_EECD_SIZE_EX_MASK: c_uint = 0x00007800  /* NVM Size */;
pub const E1000_EECD_SIZE_EX_SHIFT: c_int = 11;
pub const E1000_EECD_FLUPD: c_uint = 0x00080000 /* Update FLASH */;
pub const E1000_EECD_AUPDEN: c_uint = 0x00100000 /* Enable Autonomous FLASH update */;
pub const E1000_EECD_SEC1VAL: c_uint = 0x00400000 /* Sector One Valid */;

pub const E1000_FLASH_UPDATES: c_int = 2000;
// NVM Word Offsets
pub const NVM_COMPAT: c_uint = 0x0003;
pub const NVM_ID_LED_SETTINGS: c_uint = 0x0004;
pub const NVM_FUTURE_INIT_WORD1: c_uint = 0x0019;
pub const NVM_COMPAT_VALID_CSUM: c_uint = 0x0001;
pub const NVM_FUTURE_INIT_WORD1_VALID_CSUM: c_uint = 0x0040;
pub const NVM_INIT_CONTROL2_REG: c_uint = 0x000F;
pub const NVM_INIT_CONTROL3_PORT_B: c_uint = 0x0014;
pub const NVM_INIT_3GIO_3: c_uint = 0x001A;
pub const NVM_INIT_CONTROL3_PORT_A: c_uint = 0x0024;
pub const NVM_CFG: c_uint = 0x0012;
pub const NVM_ALT_MAC_ADDR_PTR: c_uint = 0x0037;
pub const NVM_CHECKSUM_REG: c_uint = 0x003F;
pub const E1000_NVM_CFG_DONE_PORT_0: c_uint = 0x40000 /* MNG config cycle done */;
pub const E1000_NVM_CFG_DONE_PORT_1: c_uint = 0x80000 /* ...for second port */;
// Mask bits for fields in Word 0x0f of the NVM
pub const NVM_WORD0F_PAUSE_MASK: c_uint = 0x3000;
pub const NVM_WORD0F_PAUSE: c_uint = 0x1000;
pub const NVM_WORD0F_ASM_DIR: c_uint = 0x2000;
// Mask bits for fields in Word 0x1a of the NVM
pub const NVM_WORD1A_ASPM_MASK: c_uint = 0x000C;
// Mask bits for fields in Word 0x03 of the EEPROM
pub const NVM_COMPAT_LOM: c_uint = 0x0800;
// length of string needed to store PBA number
pub const E1000_PBANUM_LENGTH: c_int = 11;
// For checksumming, the sum of all words in the NVM should equal 0xBABA.
pub const NVM_SUM: c_uint = 0xBABA;
// Uninitialized ("empty") checksum word value
pub const NVM_CHECKSUM_UNINITIALIZED: c_uint = 0xFFFF;
// PBA (printed board assembly) number words
pub const NVM_PBA_OFFSET_0: c_int = 8;
pub const NVM_PBA_OFFSET_1: c_int = 9;
pub const NVM_PBA_PTR_GUARD: c_uint = 0xFAFA;
pub const NVM_WORD_SIZE_BASE_SHIFT: c_int = 6;
// NVM Commands - SPI

pub const NVM_READ_OPCODE_SPI: c_uint = 0x03 /* NVM read opcode */;
pub const NVM_WRITE_OPCODE_SPI: c_uint = 0x02 /* NVM write opcode */;
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
pub const PCI_HEADER_TYPE_REGISTER: c_uint = 0x0E;
pub const PHY_REVISION_MASK: c_uint = 0xFFFFFFF0;
pub const MAX_PHY_REG_ADDRESS: c_uint = 0x1F  /* 5 bit address bus (0-0x1F) */;
pub const MAX_PHY_MULTI_PAGE_REG: c_uint = 0xF;
// Bit definitions for valid PHY IDs.
// I = Integrated
// E = External
//
pub const M88E1000_E_PHY_ID: c_uint = 0x01410C50;
pub const M88E1000_I_PHY_ID: c_uint = 0x01410C30;
pub const M88E1011_I_PHY_ID: c_uint = 0x01410C20;
pub const IGP01E1000_I_PHY_ID: c_uint = 0x02A80380;
pub const M88E1111_I_PHY_ID: c_uint = 0x01410CC0;
pub const GG82563_E_PHY_ID: c_uint = 0x01410CA0;
pub const IGP03E1000_E_PHY_ID: c_uint = 0x02A80390;
pub const IFE_E_PHY_ID: c_uint = 0x02A80330;
pub const IFE_PLUS_E_PHY_ID: c_uint = 0x02A80320;
pub const IFE_C_E_PHY_ID: c_uint = 0x02A80310;
pub const BME1000_E_PHY_ID: c_uint = 0x01410CB0;
pub const BME1000_E_PHY_ID_R2: c_uint = 0x01410CB1;
pub const I82577_E_PHY_ID: c_uint = 0x01540050;
pub const I82578_E_PHY_ID: c_uint = 0x004DD040;
pub const I82579_E_PHY_ID: c_uint = 0x01540090;
pub const I217_E_PHY_ID: c_uint = 0x015400A0;
// M88E1000 Specific Registers
pub const M88E1000_PHY_SPEC_CTRL: c_uint = 0x10  /* PHY Specific Control Register */;
pub const M88E1000_PHY_SPEC_STATUS: c_uint = 0x11  /* PHY Specific Status Register */;
pub const M88E1000_EXT_PHY_SPEC_CTRL: c_uint = 0x14  /* Extended PHY Specific Control */;
pub const M88E1000_PHY_PAGE_SELECT: c_uint = 0x1D  /* Reg 29 for page number setting */;
pub const M88E1000_PHY_GEN_CONTROL: c_uint = 0x1E  /* Its meaning depends on reg 29 */;
// M88E1000 PHY Specific Control Register
pub const M88E1000_PSCR_POLARITY_REVERSAL: c_uint = 0x0002 /* 1=Polarity Reversal enabled */;
pub const M88E1000_PSCR_MDI_MANUAL_MODE: c_uint = 0x0000  /* MDI Crossover Mode bits 6:5 */;
// Manual MDI configuration
pub const M88E1000_PSCR_MDIX_MANUAL_MODE: c_uint = 0x0020  /* Manual MDIX configuration */;
// 1000BASE-T: Auto crossover, 100BASE-TX/10BASE-T: MDI Mode
pub const M88E1000_PSCR_AUTO_X_1000T: c_uint = 0x0040;
// Auto crossover enabled all speeds
pub const M88E1000_PSCR_AUTO_X_MODE: c_uint = 0x0060;
pub const M88E1000_PSCR_ASSERT_CRS_ON_TX: c_uint = 0x0800 /* 1=Assert CRS on Transmit */;
// M88E1000 PHY Specific Status Register
pub const M88E1000_PSSR_REV_POLARITY: c_uint = 0x0002 /* 1=Polarity reversed */;
pub const M88E1000_PSSR_DOWNSHIFT: c_uint = 0x0020 /* 1=Downshifted */;
pub const M88E1000_PSSR_MDIX: c_uint = 0x0040 /* 1=MDIX; 0=MDI */;
// 0=<50M; 1=50-80M; 2=80-110M; 3=110-140M; 4=>140M
pub const M88E1000_PSSR_CABLE_LENGTH: c_uint = 0x0380;
pub const M88E1000_PSSR_SPEED: c_uint = 0xC000 /* Speed, bits 14:15 */;
pub const M88E1000_PSSR_1000MBS: c_uint = 0x8000 /* 10=1000Mbs */;
pub const M88E1000_PSSR_CABLE_LENGTH_SHIFT: c_int = 7;
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
// M88EC018 Rev 2 specific DownShift settings
pub const M88EC018_EPSCR_DOWNSHIFT_COUNTER_MASK: c_uint = 0x0E00;
pub const M88EC018_EPSCR_DOWNSHIFT_COUNTER_5X: c_uint = 0x0800;
pub const I82578_EPSCR_DOWNSHIFT_ENABLE: c_uint = 0x0020;
pub const I82578_EPSCR_DOWNSHIFT_COUNTER_MASK: c_uint = 0x001C;
// BME1000 PHY Specific Control Register
pub const BME1000_PSCR_ENABLE_DOWNSHIFT: c_uint = 0x0800 /* 1 = enable downshift */;
// Bits...
// 15-5: page
// 4-0: register offset
//
pub const GG82563_PAGE_SHIFT: c_int = 5;

pub const GG82563_MIN_ALT_REG: c_int = 30;
// GG82563 Specific Registers

// Page 193 - Port Control Registers

// Page 194 - KMRN Registers

// MDI Control
pub const E1000_MDIC_REG_MASK: c_uint = 0x001F0000;
pub const E1000_MDIC_REG_SHIFT: c_int = 16;
pub const E1000_MDIC_PHY_SHIFT: c_int = 21;
pub const E1000_MDIC_OP_WRITE: c_uint = 0x04000000;
pub const E1000_MDIC_OP_READ: c_uint = 0x08000000;
pub const E1000_MDIC_READY: c_uint = 0x10000000;
pub const E1000_MDIC_ERROR: c_uint = 0x40000000;
// SerDes Control
pub const E1000_GEN_POLL_TIMEOUT: c_int = 640;
pub const E1000_FEXTNVM12_PHYPD_CTRL_MASK: c_uint = 0x00C00000;
pub const E1000_FEXTNVM12_PHYPD_CTRL_P1: c_uint = 0x00800000;
