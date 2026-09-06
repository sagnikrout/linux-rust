//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/xilinx/xilinx_axienet.h
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
//
// Definitions for Xilinx Axi Ethernet device driver.
//
// Copyright (c) 2009 Secret Lab Technologies, Ltd.
// Copyright (c) 2010 - 2012 Xilinx, Inc. All rights reserved.
//

// Packet size info

// Configuration options
// Accept all incoming packets. Default: disabled (cleared)

// Jumbo frame support for Tx & Rx. Default: disabled (cleared)

// VLAN Rx & Tx frame support. Default: disabled (cleared)

// Enable recognition of flow control frames on Rx. Default: enabled (set)

// Strip FCS and PAD from incoming frames. Note: PAD from VLAN frames is not
// stripped. Default: disabled (set)
//

// Generate FCS field and add PAD automatically for outgoing frames.
// Default: enabled (set)
//

// Enable Length/Type error checking for incoming frames. When this option is
// set, the MAC will filter frames that have a mismatched type/length field
// and if XAE_OPTION_REPORT_RXERR is set, the user is notified when these
// types of frames are encountered. When this option is cleared, the MAC will
// allow these types of frames to be received. Default: enabled (set)
//

// Enable the transmitter. Default: enabled (set)

// Enable the receiver. Default: enabled (set)

// Default options set when device is initialized or reset

// Axi DMA Register definitions
pub const XAXIDMA_TX_CR_OFFSET: c_uint = 0x00000000 /* Channel control */;
pub const XAXIDMA_TX_SR_OFFSET: c_uint = 0x00000004 /* Status */;
pub const XAXIDMA_TX_CDESC_OFFSET: c_uint = 0x00000008 /* Current descriptor pointer */;
pub const XAXIDMA_TX_TDESC_OFFSET: c_uint = 0x00000010 /* Tail descriptor pointer */;
pub const XAXIDMA_RX_CR_OFFSET: c_uint = 0x00000030 /* Channel control */;
pub const XAXIDMA_RX_SR_OFFSET: c_uint = 0x00000034 /* Status */;
pub const XAXIDMA_RX_CDESC_OFFSET: c_uint = 0x00000038 /* Current descriptor pointer */;
pub const XAXIDMA_RX_TDESC_OFFSET: c_uint = 0x00000040 /* Tail descriptor pointer */;
pub const XAXIDMA_CR_RUNSTOP_MASK: c_uint = 0x00000001 /* Start/stop DMA channel */;
pub const XAXIDMA_CR_RESET_MASK: c_uint = 0x00000004 /* Reset DMA engine */;
pub const XAXIDMA_SR_HALT_MASK: c_uint = 0x00000001 /* Indicates DMA channel halted */;
pub const XAXIDMA_BD_NDESC_OFFSET: c_uint = 0x00 /* Next descriptor pointer */;
pub const XAXIDMA_BD_BUFA_OFFSET: c_uint = 0x08 /* Buffer address */;
pub const XAXIDMA_BD_CTRL_LEN_OFFSET: c_uint = 0x18 /* Control/buffer length */;
pub const XAXIDMA_BD_STS_OFFSET: c_uint = 0x1C /* Status */;
pub const XAXIDMA_BD_USR0_OFFSET: c_uint = 0x20 /* User IP specific word0 */;
pub const XAXIDMA_BD_USR1_OFFSET: c_uint = 0x24 /* User IP specific word1 */;
pub const XAXIDMA_BD_USR2_OFFSET: c_uint = 0x28 /* User IP specific word2 */;
pub const XAXIDMA_BD_USR3_OFFSET: c_uint = 0x2C /* User IP specific word3 */;
pub const XAXIDMA_BD_USR4_OFFSET: c_uint = 0x30 /* User IP specific word4 */;
pub const XAXIDMA_BD_ID_OFFSET: c_uint = 0x34 /* Sw ID */;
pub const XAXIDMA_BD_HAS_STSCNTRL_OFFSET: c_uint = 0x38 /* Whether has stscntrl strm */;
pub const XAXIDMA_BD_HAS_DRE_OFFSET: c_uint = 0x3C /* Whether has DRE */;

pub const XAXIDMA_BD_HAS_DRE_MASK: c_uint = 0xF00 /* Whether has DRE mask */;
pub const XAXIDMA_BD_WORDLEN_MASK: c_uint = 0xFF /* Whether has DRE mask */;

pub const XAXIDMA_BD_CTRL_TXSOF_MASK: c_uint = 0x08000000 /* First tx packet */;
pub const XAXIDMA_BD_CTRL_TXEOF_MASK: c_uint = 0x04000000 /* Last tx packet */;
pub const XAXIDMA_BD_CTRL_ALL_MASK: c_uint = 0x0C000000 /* All control bits */;
pub const XAXIDMA_DELAY_MASK: c_uint = 0xFF000000 /* Delay timeout counter */;
pub const XAXIDMA_COALESCE_MASK: c_uint = 0x00FF0000 /* Coalesce counter */;
pub const XAXIDMA_IRQ_IOC_MASK: c_uint = 0x00001000 /* Completion intr */;
pub const XAXIDMA_IRQ_DELAY_MASK: c_uint = 0x00002000 /* Delay interrupt */;
pub const XAXIDMA_IRQ_ERROR_MASK: c_uint = 0x00004000 /* Error interrupt */;
pub const XAXIDMA_IRQ_ALL_MASK: c_uint = 0x00007000 /* All interrupts */;
// Constant to convert delay counts to microseconds

// Default TX/RX Threshold and delay timer values for SGDMA mode
pub const XAXIDMA_DFT_TX_THRESHOLD: c_int = 24;
pub const XAXIDMA_DFT_TX_USEC: c_int = 50;
pub const XAXIDMA_DFT_RX_USEC: c_int = 16;
pub const XAXIDMA_BD_CTRL_TXSOF_MASK: c_uint = 0x08000000 /* First tx packet */;
pub const XAXIDMA_BD_CTRL_TXEOF_MASK: c_uint = 0x04000000 /* Last tx packet */;
pub const XAXIDMA_BD_CTRL_ALL_MASK: c_uint = 0x0C000000 /* All control bits */;

pub const XAXIDMA_BD_STS_COMPLETE_MASK: c_uint = 0x80000000 /* Completed */;
pub const XAXIDMA_BD_STS_DEC_ERR_MASK: c_uint = 0x40000000 /* Decode error */;
pub const XAXIDMA_BD_STS_SLV_ERR_MASK: c_uint = 0x20000000 /* Slave error */;
pub const XAXIDMA_BD_STS_INT_ERR_MASK: c_uint = 0x10000000 /* Internal err */;
pub const XAXIDMA_BD_STS_ALL_ERR_MASK: c_uint = 0x70000000 /* All errors */;
pub const XAXIDMA_BD_STS_RXSOF_MASK: c_uint = 0x08000000 /* First rx pkt */;
pub const XAXIDMA_BD_STS_RXEOF_MASK: c_uint = 0x04000000 /* Last rx pkt */;
pub const XAXIDMA_BD_STS_ALL_MASK: c_uint = 0xFC000000 /* All status bits */;
pub const XAXIDMA_BD_MINIMUM_ALIGNMENT: c_uint = 0x40;
// Axi Ethernet registers definition
pub const XAE_RAF_OFFSET: c_uint = 0x00000000 /* Reset and Address filter */;
pub const XAE_TPF_OFFSET: c_uint = 0x00000004 /* Tx Pause Frame */;
pub const XAE_IFGP_OFFSET: c_uint = 0x00000008 /* Tx Inter-frame gap adjustment*/;
pub const XAE_IS_OFFSET: c_uint = 0x0000000C /* Interrupt status */;
pub const XAE_IP_OFFSET: c_uint = 0x00000010 /* Interrupt pending */;
pub const XAE_IE_OFFSET: c_uint = 0x00000014 /* Interrupt enable */;
pub const XAE_TTAG_OFFSET: c_uint = 0x00000018 /* Tx VLAN TAG */;
pub const XAE_RTAG_OFFSET: c_uint = 0x0000001C /* Rx VLAN TAG */;
pub const XAE_UAWL_OFFSET: c_uint = 0x00000020 /* Unicast address word lower */;
pub const XAE_UAWU_OFFSET: c_uint = 0x00000024 /* Unicast address word upper */;
pub const XAE_TPID0_OFFSET: c_uint = 0x00000028 /* VLAN TPID0 register */;
pub const XAE_TPID1_OFFSET: c_uint = 0x0000002C /* VLAN TPID1 register */;
pub const XAE_PPST_OFFSET: c_uint = 0x00000030 /* PCS PMA Soft Temac Status Reg */;
pub const XAE_STATS_OFFSET: c_uint = 0x00000200 /* Statistics counters */;
pub const XAE_RCW0_OFFSET: c_uint = 0x00000400 /* Rx Configuration Word 0 */;
pub const XAE_RCW1_OFFSET: c_uint = 0x00000404 /* Rx Configuration Word 1 */;
pub const XAE_TC_OFFSET: c_uint = 0x00000408 /* Tx Configuration */;
pub const XAE_FCC_OFFSET: c_uint = 0x0000040C /* Flow Control Configuration */;
pub const XAE_EMMC_OFFSET: c_uint = 0x00000410 /* MAC speed configuration */;
pub const XAE_PHYC_OFFSET: c_uint = 0x00000414 /* RX Max Frame Configuration */;
pub const XAE_ID_OFFSET: c_uint = 0x000004F8 /* Identification register */;
pub const XAE_ABILITY_OFFSET: c_uint = 0x000004FC /* Ability Register offset */;
pub const XAE_MDIO_MC_OFFSET: c_uint = 0x00000500 /* MDIO Setup */;
pub const XAE_MDIO_MCR_OFFSET: c_uint = 0x00000504 /* MDIO Control */;
pub const XAE_MDIO_MWD_OFFSET: c_uint = 0x00000508 /* MDIO Write Data */;
pub const XAE_MDIO_MRD_OFFSET: c_uint = 0x0000050C /* MDIO Read Data */;
pub const XAE_UAW0_OFFSET: c_uint = 0x00000700 /* Unicast address word 0 */;
pub const XAE_UAW1_OFFSET: c_uint = 0x00000704 /* Unicast address word 1 */;
pub const XAE_FMI_OFFSET: c_uint = 0x00000708 /* Frame Filter Control */;
pub const XAE_FFE_OFFSET: c_uint = 0x0000070C /* Frame Filter Enable */;
pub const XAE_AF0_OFFSET: c_uint = 0x00000710 /* Address Filter 0 */;
pub const XAE_AF1_OFFSET: c_uint = 0x00000714 /* Address Filter 1 */;
pub const XAE_AM0_OFFSET: c_uint = 0x00000750 /* Frame Filter Mask Value Bytes 3-0 */;
pub const XAE_AM1_OFFSET: c_uint = 0x00000754 /* Frame Filter Mask Value Bytes 7-4 */;
pub const XAE_TX_VLAN_DATA_OFFSET: c_uint = 0x00004000 /* TX VLAN data table address */;
pub const XAE_RX_VLAN_DATA_OFFSET: c_uint = 0x00008000 /* RX VLAN data table address */;
pub const XAE_MCAST_TABLE_OFFSET: c_uint = 0x00020000 /* Multicast table address */;
// Bit Masks for Axi Ethernet RAF register
// Reject receive multicast destination address
pub const XAE_RAF_MCSTREJ_MASK: c_uint = 0x00000002;
// Reject receive broadcast destination address
pub const XAE_RAF_BCSTREJ_MASK: c_uint = 0x00000004;
pub const XAE_RAF_TXVTAGMODE_MASK: c_uint = 0x00000018 /* Tx VLAN TAG mode */;
pub const XAE_RAF_RXVTAGMODE_MASK: c_uint = 0x00000060 /* Rx VLAN TAG mode */;
pub const XAE_RAF_TXVSTRPMODE_MASK: c_uint = 0x00000180 /* Tx VLAN STRIP mode */;
pub const XAE_RAF_RXVSTRPMODE_MASK: c_uint = 0x00000600 /* Rx VLAN STRIP mode */;
pub const XAE_RAF_NEWFNCENBL_MASK: c_uint = 0x00000800 /* New function mode */;
// Extended Multicast Filtering mode
pub const XAE_RAF_EMULTIFLTRENBL_MASK: c_uint = 0x00001000;
pub const XAE_RAF_STATSRST_MASK: c_uint = 0x00002000 /* Stats. Counter Reset */;
pub const XAE_RAF_RXBADFRMEN_MASK: c_uint = 0x00004000 /* Recv Bad Frame Enable */;

// Bit Masks for Axi Ethernet TPF and IFGP registers
pub const XAE_TPF_TPFV_MASK: c_uint = 0x0000FFFF /* Tx pause frame value */;
// Transmit inter-frame gap adjustment value
pub const XAE_IFGP0_IFGP_MASK: c_uint = 0x0000007F;
// Bit Masks for Axi Ethernet IS, IE and IP registers, Same masks apply
// for all 3 registers.
//
// Hard register access complete
pub const XAE_INT_HARDACSCMPLT_MASK: c_uint = 0x00000001;
// Auto negotiation complete
pub const XAE_INT_AUTONEG_MASK: c_uint = 0x00000002;
pub const XAE_INT_RXCMPIT_MASK: c_uint = 0x00000004 /* Rx complete */;
pub const XAE_INT_RXRJECT_MASK: c_uint = 0x00000008 /* Rx frame rejected */;
pub const XAE_INT_RXFIFOOVR_MASK: c_uint = 0x00000010 /* Rx fifo overrun */;
pub const XAE_INT_TXCMPIT_MASK: c_uint = 0x00000020 /* Tx complete */;
pub const XAE_INT_RXDCMLOCK_MASK: c_uint = 0x00000040 /* Rx Dcm Lock */;
pub const XAE_INT_MGTRDY_MASK: c_uint = 0x00000080 /* MGT clock Lock */;
pub const XAE_INT_PHYRSTCMPLT_MASK: c_uint = 0x00000100 /* Phy Reset complete */;
pub const XAE_INT_ALL_MASK: c_uint = 0x0000003F /* All the ints */;
// INT bits that indicate receive errors

// Bit masks for Axi Ethernet VLAN TPID Word 0 register
pub const XAE_TPID_0_MASK: c_uint = 0x0000FFFF /* TPID 0 */;
pub const XAE_TPID_1_MASK: c_uint = 0xFFFF0000 /* TPID 1 */;
// Bit masks for Axi Ethernet VLAN TPID Word 1 register
pub const XAE_TPID_2_MASK: c_uint = 0x0000FFFF /* TPID 0 */;
pub const XAE_TPID_3_MASK: c_uint = 0xFFFF0000 /* TPID 1 */;
// Bit masks for Axi Ethernet RCW1 register
pub const XAE_RCW1_RST_MASK: c_uint = 0x80000000 /* Reset */;
pub const XAE_RCW1_JUM_MASK: c_uint = 0x40000000 /* Jumbo frame enable */;
// In-Band FCS enable (FCS not stripped)
pub const XAE_RCW1_FCS_MASK: c_uint = 0x20000000;
pub const XAE_RCW1_RX_MASK: c_uint = 0x10000000 /* Receiver enable */;
pub const XAE_RCW1_VLAN_MASK: c_uint = 0x08000000 /* VLAN frame enable */;
// Length/type field valid check disable
pub const XAE_RCW1_LT_DIS_MASK: c_uint = 0x02000000;
// Control frame Length check disable
pub const XAE_RCW1_CL_DIS_MASK: c_uint = 0x01000000;
// Pause frame source address bits [47:32]. Bits [31:0] are
// stored in register RCW0
//
pub const XAE_RCW1_PAUSEADDR_MASK: c_uint = 0x0000FFFF;
// Bit masks for Axi Ethernet TC register
pub const XAE_TC_RST_MASK: c_uint = 0x80000000 /* Reset */;
pub const XAE_TC_JUM_MASK: c_uint = 0x40000000 /* Jumbo frame enable */;
// In-Band FCS enable (FCS not generated)
pub const XAE_TC_FCS_MASK: c_uint = 0x20000000;
pub const XAE_TC_TX_MASK: c_uint = 0x10000000 /* Transmitter enable */;
pub const XAE_TC_VLAN_MASK: c_uint = 0x08000000 /* VLAN frame enable */;
// Inter-frame gap adjustment enable
pub const XAE_TC_IFG_MASK: c_uint = 0x02000000;
// Bit masks for Axi Ethernet FCC register
pub const XAE_FCC_FCRX_MASK: c_uint = 0x20000000 /* Rx flow control enable */;
pub const XAE_FCC_FCTX_MASK: c_uint = 0x40000000 /* Tx flow control enable */;
// Bit masks for Axi Ethernet EMMC register
pub const XAE_EMMC_LINKSPEED_MASK: c_uint = 0xC0000000 /* Link speed */;
pub const XAE_EMMC_RGMII_MASK: c_uint = 0x20000000 /* RGMII mode enable */;
pub const XAE_EMMC_SGMII_MASK: c_uint = 0x10000000 /* SGMII mode enable */;
pub const XAE_EMMC_GPCS_MASK: c_uint = 0x08000000 /* 1000BaseX mode enable */;
pub const XAE_EMMC_HOST_MASK: c_uint = 0x04000000 /* Host interface enable */;
pub const XAE_EMMC_TX16BIT: c_uint = 0x02000000 /* 16 bit Tx client enable */;
pub const XAE_EMMC_RX16BIT: c_uint = 0x01000000 /* 16 bit Rx client enable */;
pub const XAE_EMMC_LINKSPD_10: c_uint = 0x00000000 /* Link Speed mask for 10 Mbit */;
pub const XAE_EMMC_LINKSPD_100: c_uint = 0x40000000 /* Link Speed mask for 100 Mbit */;
pub const XAE_EMMC_LINKSPD_1000: c_uint = 0x80000000 /* Link Speed mask for 1000 Mbit */;
// Bit masks for Axi Ethernet PHYC register
pub const XAE_PHYC_SGMIILINKSPEED_MASK: c_uint = 0xC0000000 /* SGMII link speed mask*/;
pub const XAE_PHYC_RGMIILINKSPEED_MASK: c_uint = 0x0000000C /* RGMII link speed */;
pub const XAE_PHYC_RGMIIHD_MASK: c_uint = 0x00000002 /* RGMII Half-duplex */;
pub const XAE_PHYC_RGMIILINK_MASK: c_uint = 0x00000001 /* RGMII link status */;
pub const XAE_PHYC_RGLINKSPD_10: c_uint = 0x00000000 /* RGMII link 10 Mbit */;
pub const XAE_PHYC_RGLINKSPD_100: c_uint = 0x00000004 /* RGMII link 100 Mbit */;
pub const XAE_PHYC_RGLINKSPD_1000: c_uint = 0x00000008 /* RGMII link 1000 Mbit */;
pub const XAE_PHYC_SGLINKSPD_10: c_uint = 0x00000000 /* SGMII link 10 Mbit */;
pub const XAE_PHYC_SGLINKSPD_100: c_uint = 0x40000000 /* SGMII link 100 Mbit */;
pub const XAE_PHYC_SGLINKSPD_1000: c_uint = 0x80000000 /* SGMII link 1000 Mbit */;
// Bit masks for Axi Ethernet ability register

// Bit masks for Axi Ethernet MDIO interface MC register
pub const XAE_MDIO_MC_MDIOEN_MASK: c_uint = 0x00000040 /* MII management enable */;
pub const XAE_MDIO_MC_CLOCK_DIVIDE_MAX: c_uint = 0x3F	   /* Maximum MDIO divisor */;
// Bit masks for Axi Ethernet MDIO interface MCR register
pub const XAE_MDIO_MCR_PHYAD_MASK: c_uint = 0x1F000000 /* Phy Address Mask */;

pub const XAE_MDIO_MCR_REGAD_MASK: c_uint = 0x001F0000 /* Reg Address Mask */;

pub const XAE_MDIO_MCR_OP_MASK: c_uint = 0x0000C000 /* Operation Code Mask */;

pub const XAE_MDIO_MCR_OP_READ_MASK: c_uint = 0x00008000 /* Op Code Read Mask */;
pub const XAE_MDIO_MCR_OP_WRITE_MASK: c_uint = 0x00004000 /* Op Code Write Mask */;
pub const XAE_MDIO_MCR_INITIATE_MASK: c_uint = 0x00000800 /* Ready Mask */;
pub const XAE_MDIO_MCR_READY_MASK: c_uint = 0x00000080 /* Ready Mask */;
// Bit masks for Axi Ethernet MDIO interface MIS, MIP, MIE, MIC registers
pub const XAE_MDIO_INT_MIIM_RDY_MASK: c_uint = 0x00000001 /* MIIM Interrupt */;
// Bit masks for Axi Ethernet UAW1 register
// Station address bits [47:32]; Station address
// bits [31:0] are stored in register UAW0
//
pub const XAE_UAW1_UNICASTADDR_MASK: c_uint = 0x0000FFFF;
// Bit masks for Axi Ethernet FMC register
pub const XAE_FMI_PM_MASK: c_uint = 0x80000000 /* Promis. mode enable */;
pub const XAE_FMI_IND_MASK: c_uint = 0x00000003 /* Index Mask */;

// Defines for different options for C_PHY_TYPE parameter in Axi Ethernet IP
pub const XAE_PHY_TYPE_MII: c_int = 0;
pub const XAE_PHY_TYPE_GMII: c_int = 1;
pub const XAE_PHY_TYPE_RGMII_1_3: c_int = 2;
pub const XAE_PHY_TYPE_RGMII_2_0: c_int = 3;
pub const XAE_PHY_TYPE_SGMII: c_int = 4;
pub const XAE_PHY_TYPE_1000BASE_X: c_int = 5;
// Total number of entries in the hardware multicast table.
pub const XAE_MULTICAST_CAM_TABLE_NUM: c_int = 4;
// Axi Ethernet Synthesis features

pub const XAE_NO_CSUM_OFFLOAD: c_int = 0;
pub const XAE_FULL_CSUM_STATUS_MASK: c_uint = 0x00000038;
pub const XAE_IP_UDP_CSUM_VALIDATED: c_uint = 0x00000003;
pub const XAE_IP_TCP_CSUM_VALIDATED: c_uint = 0x00000002;
pub const DELAY_OF_ONE_MILLISEC: c_int = 1000;
// Xilinx PCS/PMA PHY register for switching 1000BaseX or SGMII
pub const XLNX_MII_STD_SELECT_REG: c_uint = 0x11;

// enum temac_stat - TEMAC statistics counters
//
// Index of statistics counters within the TEMAC. This must match the
// order/offset of hardware registers exactly.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum temac_stat {
    STAT_RX_BYTES = 0,
    STAT_TX_BYTES,
    STAT_UNDERSIZE_FRAMES,
    STAT_FRAGMENT_FRAMES,
    STAT_RX_64_BYTE_FRAMES,
    STAT_RX_65_127_BYTE_FRAMES,
    STAT_RX_128_255_BYTE_FRAMES,
    STAT_RX_256_511_BYTE_FRAMES,
    STAT_RX_512_1023_BYTE_FRAMES,
    STAT_RX_1024_MAX_BYTE_FRAMES,
    STAT_RX_OVERSIZE_FRAMES,
    STAT_TX_64_BYTE_FRAMES,
    STAT_TX_65_127_BYTE_FRAMES,
    STAT_TX_128_255_BYTE_FRAMES,
    STAT_TX_256_511_BYTE_FRAMES,
    STAT_TX_512_1023_BYTE_FRAMES,
    STAT_TX_1024_MAX_BYTE_FRAMES,
    STAT_TX_OVERSIZE_FRAMES,
    STAT_RX_GOOD_FRAMES,
    STAT_RX_FCS_ERRORS,
    STAT_RX_BROADCAST_FRAMES,
    STAT_RX_MULTICAST_FRAMES,
    STAT_RX_CONTROL_FRAMES,
    STAT_RX_LENGTH_ERRORS,
    STAT_RX_VLAN_FRAMES,
    STAT_RX_PAUSE_FRAMES,
    STAT_RX_CONTROL_OPCODE_ERRORS,
    STAT_TX_GOOD_FRAMES,
    STAT_TX_BROADCAST_FRAMES,
    STAT_TX_MULTICAST_FRAMES,
    STAT_TX_UNDERRUN_ERRORS,
    STAT_TX_CONTROL_FRAMES,
    STAT_TX_VLAN_FRAMES,
    STAT_TX_PAUSE_FRAMES,
    STAT_TX_SINGLE_COLLISION_FRAMES,
    STAT_TX_MULTIPLE_COLLISION_FRAMES,
    STAT_TX_DEFERRED_FRAMES,
    STAT_TX_LATE_COLLISIONS,
    STAT_TX_EXCESS_COLLISIONS,
    STAT_TX_EXCESS_DEFERRAL,
    STAT_RX_ALIGNMENT_ERRORS,
    STAT_TX_PFC_FRAMES,
    STAT_RX_PFC_FRAMES,
    STAT_USER_DEFINED0,
    STAT_USER_DEFINED1,
    STAT_USER_DEFINED2,
    STAT_COUNT,
}

//
// struct axidma_bd - Axi Dma buffer descriptor layout
// @next:         MM2S/S2MM Next Descriptor Pointer
// @next_msb:     MM2S/S2MM Next Descriptor Pointer (high 32 bits)
// @phys:         MM2S/S2MM Buffer Address
// @phys_msb:     MM2S/S2MM Buffer Address (high 32 bits)
// @reserved3:    Reserved and not used
// @reserved4:    Reserved and not used
// @cntrl:        MM2S/S2MM Control value
// @status:       MM2S/S2MM Status value
// @app0:         MM2S/S2MM User Application Field 0.
// @app1:         MM2S/S2MM User Application Field 1.
// @app2:         MM2S/S2MM User Application Field 2.
// @app3:         MM2S/S2MM User Application Field 3.
// @app4:         MM2S/S2MM User Application Field 4.
// @skb:          Pointer to SKB transferred using DMA
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct axidma_bd {
    pub /: *mut *mut u32 next; / Physical address of next buffer descriptor,
    pub /: *mut *mut u32 next_msb; / high 32 bits for IP >= v7.1, reserved on older IP,
    pub phys: u32,
    pub /: *mut *mut u32 phys_msb; / for IP >= v7.1, reserved for older IP,
    pub reserved3: u32,
    pub reserved4: u32,
    pub cntrl: u32,
    pub status: u32,
    pub app0: u32,
    pub /: *mut *mut u32 app1; / TX start << 16 | insert,
    pub /: *mut *mut u32 app2; / TX csum seed,
    pub app3: u32,
    pub /: *mut *mut u32 app4; / Last field used by HW,
    pub skb: *mut sk_buff,
    pub __aligned(XAXIDMA_BD_MINIMUM_ALIGNMENT): },
pub const XAE_NUM_MISC_CLOCKS: c_int = 3;
//
// struct skbuf_dma_descriptor - skb for each dma descriptor
// @sgl: Pointer for sglist.
// @desc: Pointer to dma descriptor.
// @dma_address: dma address of sglist.
// @skb: Pointer to SKB transferred using DMA
// @sg_len: number of entries in the sglist.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct skbuf_dma_descriptor {
    pub 1]: scatterlist sgl[MAX_SKB_FRAGS +,
    pub desc: *mut dma_async_tx_descriptor,
    pub dma_address: dma_addr_t,
    pub skb: *mut sk_buff,
    pub sg_len: c_int,
}

//
// struct axienet_local - axienet private per device data
// @ndev:	Pointer for net_device to which it will be attached.
// @dev:	Pointer to device structure
// @phylink:	Pointer to phylink instance
// @phylink_config: phylink configuration settings
// @pcs_phy:	Reference to PCS/PMA PHY if used
// @pcs:	phylink pcs structure for PCS PHY
// @switch_x_sgmii: Whether switchable 1000BaseX/SGMII mode is enabled in the core
// @axi_clk:	AXI4-Lite bus clock
// @misc_clks:	Misc ethernet clocks (AXI4-Stream, Ref, MGT clocks)
// @mii_bus:	Pointer to MII bus structure
// @mii_clk_div: MII bus clock divider value
// @regs_start: Resource start for axienet device addresses
// @regs:	Base address for the axienet_local device address space
// @dma_regs:	Base address for the axidma device address space
// @napi_rx:	NAPI RX control structure
// @rx_dim:     DIM state for the receive queue
// @rx_dim_enabled: Whether DIM is enabled or not
// @rx_irqs:    Number of interrupts
// @rx_cr_lock: Lock protecting @rx_dma_cr, its register, and @rx_dma_started
// @rx_dma_cr:  Nominal content of RX DMA control register
// @rx_dma_started: Set when RX DMA is started
// @rx_bd_v:	Virtual address of the RX buffer descriptor ring
// @rx_bd_p:	Physical address(start address) of the RX buffer descr. ring
// @rx_bd_num:	Size of RX buffer descriptor ring
// @rx_bd_ci:	Stores the index of the Rx buffer descriptor in the ring being
// accessed currently.
// @rx_packets: RX packet count for statistics
// @rx_bytes:	RX byte count for statistics
// @rx_stat_sync: Synchronization object for RX stats
// @napi_tx:	NAPI TX control structure
// @tx_cr_lock: Lock protecting @tx_dma_cr, its register, and @tx_dma_started
// @tx_dma_cr:  Nominal content of TX DMA control register
// @tx_dma_started: Set when TX DMA is started
// @tx_bd_v:	Virtual address of the TX buffer descriptor ring
// @tx_bd_p:	Physical address(start address) of the TX buffer descr. ring
// @tx_bd_num:	Size of TX buffer descriptor ring
// @tx_bd_ci:	Stores the next Tx buffer descriptor in the ring that may be
// complete. Only updated at runtime by TX NAPI poll.
// @tx_bd_tail:	Stores the index of the next Tx buffer descriptor in the ring
// to be populated.
// @tx_packets: TX packet count for statistics
// @tx_bytes:	TX byte count for statistics
// @tx_stat_sync: Synchronization object for TX stats
// @hw_stat_base: Base offset for statistics counters. This may be nonzero if
// the statistics counteres were reset or wrapped around.
// @hw_last_counter: Last-seen value of each statistic counter
// @reset_in_progress: Set while we are performing a reset and statistics
// counters may be invalid
// @hw_stats_seqcount: Sequence counter for @hw_stat_base, @hw_last_counter,
// and @reset_in_progress.
// @stats_lock: Lock for @hw_stats_seqcount
// @stats_work: Work for reading the hardware statistics counters often enough
// to catch overflows.
// @dma_err_task: Work structure to process Axi DMA errors
// @stopping:   Set when @dma_err_task shouldn't do anything because we are
// about to stop the device.
// @tx_irq:	Axidma TX IRQ number
// @rx_irq:	Axidma RX IRQ number
// @eth_irq:	Ethernet core IRQ number
// @phy_mode:	Phy type to identify between MII/GMII/RGMII/SGMII/1000 Base-X
// @options:	AxiEthernet option word
// @features:	Stores the extended features supported by the axienet hw
// @max_frm_size: Stores the maximum size of the frame that can be that
// Txed/Rxed in the existing hardware. If jumbo option is
// supported, the maximum frame size would be 9k. Else it is
// 1522 bytes (assuming support for basic VLAN)
// @rxmem:	Stores rx memory size for jumbo frame handling.
// @use_dmaengine: flag to check dmaengine framework usage.
// @tx_chan:	TX DMA channel.
// @rx_chan:	RX DMA channel.
// @tx_skb_ring: Pointer to TX skb ring buffer array.
// @rx_skb_ring: Pointer to RX skb ring buffer array.
// @tx_ring_head: TX skb ring buffer head index.
// @tx_ring_tail: TX skb ring buffer tail index.
// @rx_ring_head: RX skb ring buffer head index.
// @rx_ring_tail: RX skb ring buffer tail index.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct axienet_local {
    pub ndev: *mut net_device,
    pub dev: *mut device,
    pub phylink: *mut phylink,
    pub phylink_config: phylink_config,
    pub pcs_phy: *mut mdio_device,
    pub pcs: phylink_pcs,
    pub switch_x_sgmii: bool,
    pub axi_clk: *mut clk,
    pub misc_clks: [clk_bulk_data; XAE_NUM_MISC_CLOCKS],
    pub mii_bus: *mut mii_bus,
    pub mii_clk_div: u8,
    pub regs_start: resource_size_t,
    pub regs: *mut void __iomem,
    pub dma_regs: *mut void __iomem,
    pub napi_rx: napi_struct,
    pub rx_dim: dim,
    pub rx_dim_enabled: bool,
    pub rx_irqs: u16,
    pub rx_cr_lock: spinlock_t,
    pub rx_dma_cr: u32,
    pub rx_dma_started: bool,
    pub rx_bd_v: *mut axidma_bd,
    pub rx_bd_p: dma_addr_t,
    pub rx_bd_num: u32,
    pub rx_bd_ci: u32,
    pub rx_packets: u64_stats_t,
    pub rx_bytes: u64_stats_t,
    pub rx_stat_sync: u64_stats_sync,
    pub napi_tx: napi_struct,
    pub tx_cr_lock: spinlock_t,
    pub tx_dma_cr: u32,
    pub tx_dma_started: bool,
    pub tx_bd_v: *mut axidma_bd,
    pub tx_bd_p: dma_addr_t,
    pub tx_bd_num: u32,
    pub tx_bd_ci: u32,
    pub tx_bd_tail: u32,
    pub tx_packets: u64_stats_t,
    pub tx_bytes: u64_stats_t,
    pub tx_stat_sync: u64_stats_sync,
    pub hw_stat_base: [u64; STAT_COUNT],
    pub hw_last_counter: [u32; STAT_COUNT],
    pub hw_stats_seqcount: seqcount_mutex_t,
    pub stats_lock: mutex,
    pub stats_work: delayed_work,
    pub reset_in_progress: bool,
    pub dma_err_task: work_struct,
    pub stopping: bool,
    pub tx_irq: c_int,
    pub rx_irq: c_int,
    pub eth_irq: c_int,
    pub phy_mode: phy_interface_t,
    pub options: u32,
    pub features: u32,
    pub max_frm_size: u32,
    pub rxmem: u32,
    pub use_dmaengine: u8,
    pub tx_chan: *mut dma_chan,
    pub rx_chan: *mut dma_chan,
    pub tx_skb_ring: *mut skbuf_dma_descriptor,
    pub rx_skb_ring: *mut skbuf_dma_descriptor,
    pub tx_ring_head: c_int,
    pub tx_ring_tail: c_int,
    pub rx_ring_head: c_int,
    pub rx_ring_tail: c_int,
}

//
// struct axienet_option - Used to set axi ethernet hardware options
// @opt:	Option to be set.
// @reg:	Register offset to be written for setting the option
// @m_or:	Mask to be ORed for setting the option in the register
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct axienet_option {
    pub opt: u32,
    pub reg: u32,
    pub m_or: u32,
}

//
// axienet_ior - Memory mapped Axi Ethernet register read
// @lp:         Pointer to axienet local structure
// @offset:     Address offset from the base address of Axi Ethernet core
//
// Return: The contents of the Axi Ethernet register
//
// This function returns the contents of the corresponding register.
//
extern "C" {
    pub fn ioread32(offset: lp->regs +) -> return;
}
extern "C" {
    pub fn axienet_ior(_arg: lp, _arg: XAE_MDIO_MCR_OFFSET) -> return;
}
//
// axienet_iow - Memory mapped Axi Ethernet register write
// @lp:         Pointer to axienet local structure
// @offset:     Address offset from the base address of Axi Ethernet core
// @value:      Value to be written into the Axi Ethernet register
//
// This function writes the desired value into the corresponding Axi Ethernet
// register.
//
// axienet_dma_out32 - Memory mapped Axi DMA register write.
// @lp:		Pointer to axienet local structure
// @reg:	Address offset from the base address of the Axi DMA core
// @value:	Value to be written into the Axi DMA register
//
// This function writes the desired value into the corresponding Axi DMA
// register.
//

//
// axienet_dma_out64 - Memory mapped Axi DMA register write.
// @lp:		Pointer to axienet local structure
// @reg:	Address offset from the base address of the Axi DMA core
// @value:	Value to be written into the Axi DMA register
//
// This function writes the desired value into the corresponding Axi DMA
// register.
//

// Function prototypes visible in xilinx_axienet_mdio.c for other files
extern "C" {
    pub fn axienet_mdio_setup(lp: *mut axienet_local) -> c_int;
}
extern "C" {
    pub fn axienet_mdio_teardown(lp: *mut axienet_local);
}
