//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bgmac.h
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

pub const BGMAC_DEV_CTL: c_uint = 0x000;
pub const BGMAC_DC_TSM: c_uint = 0x00000002;
pub const BGMAC_DC_CFCO: c_uint = 0x00000004;
pub const BGMAC_DC_RLSS: c_uint = 0x00000008;
pub const BGMAC_DC_MROR: c_uint = 0x00000010;
pub const BGMAC_DC_FCM_MASK: c_uint = 0x00000060;
pub const BGMAC_DC_FCM_SHIFT: c_int = 5;
pub const BGMAC_DC_NAE: c_uint = 0x00000080;
pub const BGMAC_DC_TF: c_uint = 0x00000100;
pub const BGMAC_DC_RDS_MASK: c_uint = 0x00030000;
pub const BGMAC_DC_RDS_SHIFT: c_int = 16;
pub const BGMAC_DC_TDS_MASK: c_uint = 0x000c0000;
pub const BGMAC_DC_TDS_SHIFT: c_int = 18;
pub const BGMAC_DEV_STATUS: c_uint = 0x004		/* Configuration of the interface */;
pub const BGMAC_DS_RBF: c_uint = 0x00000001;
pub const BGMAC_DS_RDF: c_uint = 0x00000002;
pub const BGMAC_DS_RIF: c_uint = 0x00000004;
pub const BGMAC_DS_TBF: c_uint = 0x00000008;
pub const BGMAC_DS_TDF: c_uint = 0x00000010;
pub const BGMAC_DS_TIF: c_uint = 0x00000020;
pub const BGMAC_DS_PO: c_uint = 0x00000040;
pub const BGMAC_DS_MM_MASK: c_uint = 0x00000300	/* Mode of the interface */;
pub const BGMAC_DS_MM_SHIFT: c_int = 8;
pub const BGMAC_BIST_STATUS: c_uint = 0x00c;
pub const BGMAC_INT_STATUS: c_uint = 0x020		/* Interrupt status */;
pub const BGMAC_IS_MRO: c_uint = 0x00000001;
pub const BGMAC_IS_MTO: c_uint = 0x00000002;
pub const BGMAC_IS_TFD: c_uint = 0x00000004;
pub const BGMAC_IS_LS: c_uint = 0x00000008;
pub const BGMAC_IS_MDIO: c_uint = 0x00000010;
pub const BGMAC_IS_MR: c_uint = 0x00000020;
pub const BGMAC_IS_MT: c_uint = 0x00000040;
pub const BGMAC_IS_TO: c_uint = 0x00000080;
pub const BGMAC_IS_DESC_ERR: c_uint = 0x00000400	/* Descriptor error */;
pub const BGMAC_IS_DATA_ERR: c_uint = 0x00000800	/* Data error */;
pub const BGMAC_IS_DESC_PROT_ERR: c_uint = 0x00001000	/* Descriptor protocol error */;
pub const BGMAC_IS_RX_DESC_UNDERF: c_uint = 0x00002000	/* Receive descriptor underflow */;
pub const BGMAC_IS_RX_F_OVERF: c_uint = 0x00004000	/* Receive FIFO overflow */;
pub const BGMAC_IS_TX_F_UNDERF: c_uint = 0x00008000	/* Transmit FIFO underflow */;
pub const BGMAC_IS_RX: c_uint = 0x00010000	/* Interrupt for RX queue 0 */;
pub const BGMAC_IS_TX0: c_uint = 0x01000000	/* Interrupt for TX queue 0 */;
pub const BGMAC_IS_TX1: c_uint = 0x02000000	/* Interrupt for TX queue 1 */;
pub const BGMAC_IS_TX2: c_uint = 0x04000000	/* Interrupt for TX queue 2 */;
pub const BGMAC_IS_TX3: c_uint = 0x08000000	/* Interrupt for TX queue 3 */;
pub const BGMAC_IS_TX_MASK: c_uint = 0x0f000000;
pub const BGMAC_IS_INTMASK: c_uint = 0x0f01fcff;
pub const BGMAC_IS_ERRMASK: c_uint = 0x0000fc00;
pub const BGMAC_INT_MASK: c_uint = 0x024		/* Interrupt mask */;
pub const BGMAC_GP_TIMER: c_uint = 0x028;
pub const BGMAC_INT_RECV_LAZY: c_uint = 0x100;
pub const BGMAC_IRL_TO_MASK: c_uint = 0x00ffffff;
pub const BGMAC_IRL_FC_MASK: c_uint = 0xff000000;

pub const BGMAC_FLOW_CTL_THRESH: c_uint = 0x104		/* Flow control thresholds */;
pub const BGMAC_WRRTHRESH: c_uint = 0x108;
pub const BGMAC_GMAC_IDLE_CNT_THRESH: c_uint = 0x10c;
pub const BGMAC_PHY_ACCESS: c_uint = 0x180		/* PHY access address */;
pub const BGMAC_PA_DATA_MASK: c_uint = 0x0000ffff;
pub const BGMAC_PA_ADDR_MASK: c_uint = 0x001f0000;
pub const BGMAC_PA_ADDR_SHIFT: c_int = 16;
pub const BGMAC_PA_REG_MASK: c_uint = 0x1f000000;
pub const BGMAC_PA_REG_SHIFT: c_int = 24;
pub const BGMAC_PA_WRITE: c_uint = 0x20000000;
pub const BGMAC_PA_START: c_uint = 0x40000000;
pub const BGMAC_PHY_CNTL: c_uint = 0x188		/* PHY control address */;
pub const BGMAC_PC_EPA_MASK: c_uint = 0x0000001f;
pub const BGMAC_PC_MCT_MASK: c_uint = 0x007f0000;
pub const BGMAC_PC_MCT_SHIFT: c_int = 16;
pub const BGMAC_PC_MTE: c_uint = 0x00800000;
pub const BGMAC_TXQ_CTL: c_uint = 0x18c;
pub const BGMAC_TXQ_CTL_DBT_MASK: c_uint = 0x00000fff;
pub const BGMAC_TXQ_CTL_DBT_SHIFT: c_int = 0;
pub const BGMAC_RXQ_CTL: c_uint = 0x190;
pub const BGMAC_RXQ_CTL_DBT_MASK: c_uint = 0x00000fff;
pub const BGMAC_RXQ_CTL_DBT_SHIFT: c_int = 0;
pub const BGMAC_RXQ_CTL_PTE: c_uint = 0x00001000;
pub const BGMAC_RXQ_CTL_MDP_MASK: c_uint = 0x3f000000;
pub const BGMAC_RXQ_CTL_MDP_SHIFT: c_int = 24;
pub const BGMAC_GPIO_SELECT: c_uint = 0x194;
pub const BGMAC_GPIO_OUTPUT_EN: c_uint = 0x198;
// For 0x1e0 see BCMA_CLKCTLST. Below are BGMAC specific bits
pub const BGMAC_BCMA_CLKCTLST_MISC_PLL_REQ: c_uint = 0x00000100;
pub const BGMAC_BCMA_CLKCTLST_MISC_PLL_ST: c_uint = 0x01000000;
pub const BGMAC_HW_WAR: c_uint = 0x1e4;
pub const BGMAC_PWR_CTL: c_uint = 0x1e8;
pub const BGMAC_DMA_BASE0: c_uint = 0x200		/* Tx and Rx controller */;
pub const BGMAC_DMA_BASE1: c_uint = 0x240		/* Tx controller only */;
pub const BGMAC_DMA_BASE2: c_uint = 0x280		/* Tx controller only */;
pub const BGMAC_DMA_BASE3: c_uint = 0x2C0		/* Tx controller only */;
pub const BGMAC_TX_GOOD_OCTETS: c_uint = 0x300;
pub const BGMAC_TX_GOOD_OCTETS_HIGH: c_uint = 0x304;
pub const BGMAC_TX_GOOD_PKTS: c_uint = 0x308;
pub const BGMAC_TX_OCTETS: c_uint = 0x30c;
pub const BGMAC_TX_OCTETS_HIGH: c_uint = 0x310;
pub const BGMAC_TX_PKTS: c_uint = 0x314;
pub const BGMAC_TX_BROADCAST_PKTS: c_uint = 0x318;
pub const BGMAC_TX_MULTICAST_PKTS: c_uint = 0x31c;
pub const BGMAC_TX_LEN_64: c_uint = 0x320;
pub const BGMAC_TX_LEN_65_TO_127: c_uint = 0x324;
pub const BGMAC_TX_LEN_128_TO_255: c_uint = 0x328;
pub const BGMAC_TX_LEN_256_TO_511: c_uint = 0x32c;
pub const BGMAC_TX_LEN_512_TO_1023: c_uint = 0x330;
pub const BGMAC_TX_LEN_1024_TO_1522: c_uint = 0x334;
pub const BGMAC_TX_LEN_1523_TO_2047: c_uint = 0x338;
pub const BGMAC_TX_LEN_2048_TO_4095: c_uint = 0x33c;
pub const BGMAC_TX_LEN_4096_TO_8191: c_uint = 0x340;
pub const BGMAC_TX_LEN_8192_TO_MAX: c_uint = 0x344;
pub const BGMAC_TX_JABBER_PKTS: c_uint = 0x348		/* Error */;
pub const BGMAC_TX_OVERSIZE_PKTS: c_uint = 0x34c		/* Error */;
pub const BGMAC_TX_FRAGMENT_PKTS: c_uint = 0x350;
pub const BGMAC_TX_UNDERRUNS: c_uint = 0x354		/* Error */;
pub const BGMAC_TX_TOTAL_COLS: c_uint = 0x358;
pub const BGMAC_TX_SINGLE_COLS: c_uint = 0x35c;
pub const BGMAC_TX_MULTIPLE_COLS: c_uint = 0x360;
pub const BGMAC_TX_EXCESSIVE_COLS: c_uint = 0x364		/* Error */;
pub const BGMAC_TX_LATE_COLS: c_uint = 0x368		/* Error */;
pub const BGMAC_TX_DEFERED: c_uint = 0x36c;
pub const BGMAC_TX_CARRIER_LOST: c_uint = 0x370;
pub const BGMAC_TX_PAUSE_PKTS: c_uint = 0x374;
pub const BGMAC_TX_UNI_PKTS: c_uint = 0x378;
pub const BGMAC_TX_Q0_PKTS: c_uint = 0x37c;
pub const BGMAC_TX_Q0_OCTETS: c_uint = 0x380;
pub const BGMAC_TX_Q0_OCTETS_HIGH: c_uint = 0x384;
pub const BGMAC_TX_Q1_PKTS: c_uint = 0x388;
pub const BGMAC_TX_Q1_OCTETS: c_uint = 0x38c;
pub const BGMAC_TX_Q1_OCTETS_HIGH: c_uint = 0x390;
pub const BGMAC_TX_Q2_PKTS: c_uint = 0x394;
pub const BGMAC_TX_Q2_OCTETS: c_uint = 0x398;
pub const BGMAC_TX_Q2_OCTETS_HIGH: c_uint = 0x39c;
pub const BGMAC_TX_Q3_PKTS: c_uint = 0x3a0;
pub const BGMAC_TX_Q3_OCTETS: c_uint = 0x3a4;
pub const BGMAC_TX_Q3_OCTETS_HIGH: c_uint = 0x3a8;
pub const BGMAC_RX_GOOD_OCTETS: c_uint = 0x3b0;
pub const BGMAC_RX_GOOD_OCTETS_HIGH: c_uint = 0x3b4;
pub const BGMAC_RX_GOOD_PKTS: c_uint = 0x3b8;
pub const BGMAC_RX_OCTETS: c_uint = 0x3bc;
pub const BGMAC_RX_OCTETS_HIGH: c_uint = 0x3c0;
pub const BGMAC_RX_PKTS: c_uint = 0x3c4;
pub const BGMAC_RX_BROADCAST_PKTS: c_uint = 0x3c8;
pub const BGMAC_RX_MULTICAST_PKTS: c_uint = 0x3cc;
pub const BGMAC_RX_LEN_64: c_uint = 0x3d0;
pub const BGMAC_RX_LEN_65_TO_127: c_uint = 0x3d4;
pub const BGMAC_RX_LEN_128_TO_255: c_uint = 0x3d8;
pub const BGMAC_RX_LEN_256_TO_511: c_uint = 0x3dc;
pub const BGMAC_RX_LEN_512_TO_1023: c_uint = 0x3e0;
pub const BGMAC_RX_LEN_1024_TO_1522: c_uint = 0x3e4;
pub const BGMAC_RX_LEN_1523_TO_2047: c_uint = 0x3e8;
pub const BGMAC_RX_LEN_2048_TO_4095: c_uint = 0x3ec;
pub const BGMAC_RX_LEN_4096_TO_8191: c_uint = 0x3f0;
pub const BGMAC_RX_LEN_8192_TO_MAX: c_uint = 0x3f4;
pub const BGMAC_RX_JABBER_PKTS: c_uint = 0x3f8		/* Error */;
pub const BGMAC_RX_OVERSIZE_PKTS: c_uint = 0x3fc		/* Error */;
pub const BGMAC_RX_FRAGMENT_PKTS: c_uint = 0x400;
pub const BGMAC_RX_MISSED_PKTS: c_uint = 0x404		/* Error */;
pub const BGMAC_RX_CRC_ALIGN_ERRS: c_uint = 0x408		/* Error */;
pub const BGMAC_RX_UNDERSIZE: c_uint = 0x40c		/* Error */;
pub const BGMAC_RX_CRC_ERRS: c_uint = 0x410		/* Error */;
pub const BGMAC_RX_ALIGN_ERRS: c_uint = 0x414		/* Error */;
pub const BGMAC_RX_SYMBOL_ERRS: c_uint = 0x418		/* Error */;
pub const BGMAC_RX_PAUSE_PKTS: c_uint = 0x41c;
pub const BGMAC_RX_NONPAUSE_PKTS: c_uint = 0x420;
pub const BGMAC_RX_SACHANGES: c_uint = 0x424;
pub const BGMAC_RX_UNI_PKTS: c_uint = 0x428;
pub const BGMAC_UNIMAC: c_uint = 0x800;
// BCMA GMAC core specific IO Control (BCMA_IOCTL) flags
pub const BGMAC_BCMA_IOCTL_SW_CLKEN: c_uint = 0x00000004	/* PHY Clock Enable */;
pub const BGMAC_BCMA_IOCTL_SW_RESET: c_uint = 0x00000008	/* PHY Reset */;
// The IOCTL values appear to be different in NS, NSP, and NS2, and do not match
// the values directly above
//

// BCMA GMAC core specific IO status (BCMA_IOST) flags
pub const BGMAC_BCMA_IOST_ATTACHED: c_uint = 0x00000800;

pub const BGMAC_DMA_TX_CTL: c_uint = 0x00;
pub const BGMAC_DMA_TX_ENABLE: c_uint = 0x00000001;
pub const BGMAC_DMA_TX_SUSPEND: c_uint = 0x00000002;
pub const BGMAC_DMA_TX_LOOPBACK: c_uint = 0x00000004;
pub const BGMAC_DMA_TX_FLUSH: c_uint = 0x00000010;
pub const BGMAC_DMA_TX_MR_MASK: c_uint = 0x000000C0	/* Multiple outstanding reads */;
pub const BGMAC_DMA_TX_MR_SHIFT: c_int = 6;
pub const BGMAC_DMA_TX_MR_1: c_int = 0;
pub const BGMAC_DMA_TX_MR_2: c_int = 1;
pub const BGMAC_DMA_TX_PARITY_DISABLE: c_uint = 0x00000800;
pub const BGMAC_DMA_TX_ADDREXT_MASK: c_uint = 0x00030000;
pub const BGMAC_DMA_TX_ADDREXT_SHIFT: c_int = 16;
pub const BGMAC_DMA_TX_BL_MASK: c_uint = 0x001C0000	/* BurstLen bits */;
pub const BGMAC_DMA_TX_BL_SHIFT: c_int = 18;
pub const BGMAC_DMA_TX_BL_16: c_int = 0;
pub const BGMAC_DMA_TX_BL_32: c_int = 1;
pub const BGMAC_DMA_TX_BL_64: c_int = 2;
pub const BGMAC_DMA_TX_BL_128: c_int = 3;
pub const BGMAC_DMA_TX_BL_256: c_int = 4;
pub const BGMAC_DMA_TX_BL_512: c_int = 5;
pub const BGMAC_DMA_TX_BL_1024: c_int = 6;
pub const BGMAC_DMA_TX_PC_MASK: c_uint = 0x00E00000	/* Prefetch control */;
pub const BGMAC_DMA_TX_PC_SHIFT: c_int = 21;
pub const BGMAC_DMA_TX_PC_0: c_int = 0;
pub const BGMAC_DMA_TX_PC_4: c_int = 1;
pub const BGMAC_DMA_TX_PC_8: c_int = 2;
pub const BGMAC_DMA_TX_PC_16: c_int = 3;
pub const BGMAC_DMA_TX_PT_MASK: c_uint = 0x03000000	/* Prefetch threshold */;
pub const BGMAC_DMA_TX_PT_SHIFT: c_int = 24;
pub const BGMAC_DMA_TX_PT_1: c_int = 0;
pub const BGMAC_DMA_TX_PT_2: c_int = 1;
pub const BGMAC_DMA_TX_PT_4: c_int = 2;
pub const BGMAC_DMA_TX_PT_8: c_int = 3;
pub const BGMAC_DMA_TX_INDEX: c_uint = 0x04;
pub const BGMAC_DMA_TX_RINGLO: c_uint = 0x08;
pub const BGMAC_DMA_TX_RINGHI: c_uint = 0x0C;
pub const BGMAC_DMA_TX_STATUS: c_uint = 0x10;
pub const BGMAC_DMA_TX_STATDPTR: c_uint = 0x00001FFF;
pub const BGMAC_DMA_TX_STAT: c_uint = 0xF0000000;
pub const BGMAC_DMA_TX_STAT_DISABLED: c_uint = 0x00000000;
pub const BGMAC_DMA_TX_STAT_ACTIVE: c_uint = 0x10000000;
pub const BGMAC_DMA_TX_STAT_IDLEWAIT: c_uint = 0x20000000;
pub const BGMAC_DMA_TX_STAT_STOPPED: c_uint = 0x30000000;
pub const BGMAC_DMA_TX_STAT_SUSP: c_uint = 0x40000000;
pub const BGMAC_DMA_TX_ERROR: c_uint = 0x14;
pub const BGMAC_DMA_TX_ERRDPTR: c_uint = 0x0001FFFF;
pub const BGMAC_DMA_TX_ERR: c_uint = 0xF0000000;
pub const BGMAC_DMA_TX_ERR_NOERR: c_uint = 0x00000000;
pub const BGMAC_DMA_TX_ERR_PROT: c_uint = 0x10000000;
pub const BGMAC_DMA_TX_ERR_UNDERRUN: c_uint = 0x20000000;
pub const BGMAC_DMA_TX_ERR_TRANSFER: c_uint = 0x30000000;
pub const BGMAC_DMA_TX_ERR_DESCREAD: c_uint = 0x40000000;
pub const BGMAC_DMA_TX_ERR_CORE: c_uint = 0x50000000;
pub const BGMAC_DMA_RX_CTL: c_uint = 0x20;
pub const BGMAC_DMA_RX_ENABLE: c_uint = 0x00000001;
pub const BGMAC_DMA_RX_FRAME_OFFSET_MASK: c_uint = 0x000000FE;
pub const BGMAC_DMA_RX_FRAME_OFFSET_SHIFT: c_int = 1;
pub const BGMAC_DMA_RX_DIRECT_FIFO: c_uint = 0x00000100;
pub const BGMAC_DMA_RX_OVERFLOW_CONT: c_uint = 0x00000400;
pub const BGMAC_DMA_RX_PARITY_DISABLE: c_uint = 0x00000800;
pub const BGMAC_DMA_RX_MR_MASK: c_uint = 0x000000C0	/* Multiple outstanding reads */;
pub const BGMAC_DMA_RX_MR_SHIFT: c_int = 6;
pub const BGMAC_DMA_TX_MR_1: c_int = 0;
pub const BGMAC_DMA_TX_MR_2: c_int = 1;
pub const BGMAC_DMA_RX_ADDREXT_MASK: c_uint = 0x00030000;
pub const BGMAC_DMA_RX_ADDREXT_SHIFT: c_int = 16;
pub const BGMAC_DMA_RX_BL_MASK: c_uint = 0x001C0000	/* BurstLen bits */;
pub const BGMAC_DMA_RX_BL_SHIFT: c_int = 18;
pub const BGMAC_DMA_RX_BL_16: c_int = 0;
pub const BGMAC_DMA_RX_BL_32: c_int = 1;
pub const BGMAC_DMA_RX_BL_64: c_int = 2;
pub const BGMAC_DMA_RX_BL_128: c_int = 3;
pub const BGMAC_DMA_RX_BL_256: c_int = 4;
pub const BGMAC_DMA_RX_BL_512: c_int = 5;
pub const BGMAC_DMA_RX_BL_1024: c_int = 6;
pub const BGMAC_DMA_RX_PC_MASK: c_uint = 0x00E00000	/* Prefetch control */;
pub const BGMAC_DMA_RX_PC_SHIFT: c_int = 21;
pub const BGMAC_DMA_RX_PC_0: c_int = 0;
pub const BGMAC_DMA_RX_PC_4: c_int = 1;
pub const BGMAC_DMA_RX_PC_8: c_int = 2;
pub const BGMAC_DMA_RX_PC_16: c_int = 3;
pub const BGMAC_DMA_RX_PT_MASK: c_uint = 0x03000000	/* Prefetch threshold */;
pub const BGMAC_DMA_RX_PT_SHIFT: c_int = 24;
pub const BGMAC_DMA_RX_PT_1: c_int = 0;
pub const BGMAC_DMA_RX_PT_2: c_int = 1;
pub const BGMAC_DMA_RX_PT_4: c_int = 2;
pub const BGMAC_DMA_RX_PT_8: c_int = 3;
pub const BGMAC_DMA_RX_INDEX: c_uint = 0x24;
pub const BGMAC_DMA_RX_RINGLO: c_uint = 0x28;
pub const BGMAC_DMA_RX_RINGHI: c_uint = 0x2C;
pub const BGMAC_DMA_RX_STATUS: c_uint = 0x30;
pub const BGMAC_DMA_RX_STATDPTR: c_uint = 0x00001FFF;
pub const BGMAC_DMA_RX_STAT: c_uint = 0xF0000000;
pub const BGMAC_DMA_RX_STAT_DISABLED: c_uint = 0x00000000;
pub const BGMAC_DMA_RX_STAT_ACTIVE: c_uint = 0x10000000;
pub const BGMAC_DMA_RX_STAT_IDLEWAIT: c_uint = 0x20000000;
pub const BGMAC_DMA_RX_STAT_STOPPED: c_uint = 0x30000000;
pub const BGMAC_DMA_RX_STAT_SUSP: c_uint = 0x40000000;
pub const BGMAC_DMA_RX_ERROR: c_uint = 0x34;
pub const BGMAC_DMA_RX_ERRDPTR: c_uint = 0x0001FFFF;
pub const BGMAC_DMA_RX_ERR: c_uint = 0xF0000000;
pub const BGMAC_DMA_RX_ERR_NOERR: c_uint = 0x00000000;
pub const BGMAC_DMA_RX_ERR_PROT: c_uint = 0x10000000;
pub const BGMAC_DMA_RX_ERR_UNDERRUN: c_uint = 0x20000000;
pub const BGMAC_DMA_RX_ERR_TRANSFER: c_uint = 0x30000000;
pub const BGMAC_DMA_RX_ERR_DESCREAD: c_uint = 0x40000000;
pub const BGMAC_DMA_RX_ERR_CORE: c_uint = 0x50000000;
pub const BGMAC_DESC_CTL0_EOT: c_uint = 0x10000000	/* End of ring */;
pub const BGMAC_DESC_CTL0_IOC: c_uint = 0x20000000	/* IRQ on complete */;
pub const BGMAC_DESC_CTL0_EOF: c_uint = 0x40000000	/* End of frame */;
pub const BGMAC_DESC_CTL0_SOF: c_uint = 0x80000000	/* Start of frame */;
pub const BGMAC_DESC_CTL1_LEN: c_uint = 0x00003FFF;

pub const BGMAC_PHY_MASK: c_uint = 0x1F;
pub const BGMAC_MAX_TX_RINGS: c_int = 4;
pub const BGMAC_MAX_RX_RINGS: c_int = 1;
pub const BGMAC_TX_RING_SLOTS: c_int = 128;
pub const BGMAC_RX_RING_SLOTS: c_int = 512;

pub const BGMAC_RX_MAX_FRAME_SIZE: c_int = 1536;

pub const BGMAC_BFL_ENETROBO: c_uint = 0x0010		/* has ephy roboswitch spi */;
pub const BGMAC_BFL_ENETADM: c_uint = 0x0080		/* has ADMtek switch */;
pub const BGMAC_BFL_ENETVLAN: c_uint = 0x0100		/* can do vlan */;
pub const BGMAC_CHIPCTL_1_IF_TYPE_MASK: c_uint = 0x00000030;
pub const BGMAC_CHIPCTL_1_IF_TYPE_RMII: c_uint = 0x00000000;
pub const BGMAC_CHIPCTL_1_IF_TYPE_MII: c_uint = 0x00000010;
pub const BGMAC_CHIPCTL_1_IF_TYPE_RGMII: c_uint = 0x00000020;
pub const BGMAC_CHIPCTL_1_SW_TYPE_MASK: c_uint = 0x000000C0;
pub const BGMAC_CHIPCTL_1_SW_TYPE_EPHY: c_uint = 0x00000000;
pub const BGMAC_CHIPCTL_1_SW_TYPE_EPHYMII: c_uint = 0x00000040;
pub const BGMAC_CHIPCTL_1_SW_TYPE_EPHYRMII: c_uint = 0x00000080;
pub const BGMAC_CHIPCTL_1_SW_TYPE_RGMII: c_uint = 0x000000C0;
pub const BGMAC_CHIPCTL_1_RXC_DLL_BYPASS: c_uint = 0x00010000;
pub const BGMAC_CHIPCTL_4_IF_TYPE_MASK: c_uint = 0x00003000;
pub const BGMAC_CHIPCTL_4_IF_TYPE_RMII: c_uint = 0x00000000;
pub const BGMAC_CHIPCTL_4_IF_TYPE_MII: c_uint = 0x00001000;
pub const BGMAC_CHIPCTL_4_IF_TYPE_RGMII: c_uint = 0x00002000;
pub const BGMAC_CHIPCTL_4_SW_TYPE_MASK: c_uint = 0x0000C000;
pub const BGMAC_CHIPCTL_4_SW_TYPE_EPHY: c_uint = 0x00000000;
pub const BGMAC_CHIPCTL_4_SW_TYPE_EPHYMII: c_uint = 0x00004000;
pub const BGMAC_CHIPCTL_4_SW_TYPE_EPHYRMII: c_uint = 0x00008000;
pub const BGMAC_CHIPCTL_4_SW_TYPE_RGMII: c_uint = 0x0000C000;
pub const BGMAC_CHIPCTL_7_IF_TYPE_MASK: c_uint = 0x000000C0;
pub const BGMAC_CHIPCTL_7_IF_TYPE_RMII: c_uint = 0x00000000;
pub const BGMAC_CHIPCTL_7_IF_TYPE_MII: c_uint = 0x00000040;
pub const BGMAC_CHIPCTL_7_IF_TYPE_RGMII: c_uint = 0x00000080;

// Feature Flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bgmac_slot_info {
    pub skb: *mut sk_buff,
    pub buf: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bgmac_dma_desc {
    pub ctl0: __le32,
    pub ctl1: __le32,
    pub addr_low: __le32,
    pub addr_high: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bgmac_dma_ring_type {
    BGMAC_DMA_RING_TX,
    BGMAC_DMA_RING_RX,
}

//
// bgmac_dma_ring - contains info about DMA ring (either TX or RX one)
// @start: index of the first slot containing data
// @end: index of a slot that can *not* be read (yet)
//
// Be really aware of the specific @end meaning. It's an index of a slot *after
// the one containing data that can be read. If @start equals @end the ring is
// empty.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bgmac_dma_ring {
    pub start: u32,
    pub end: u32,
    pub cpu_base: *mut bgmac_dma_desc,
    pub dma_base: dma_addr_t,
    pub /: *mut *mut u32 index_base; / Used for unaligned rings only, otherwise 0,
    pub mmio_base: u16,
    pub unaligned: bool,
    pub slots: [bgmac_slot_info; BGMAC_RX_RING_SLOTS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bgmac_rx_header {
    pub len: __le16,
    pub flags: __le16,
    pub pad: [__le16; 12],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bgmac {
    pub base: *mut void __iomem,
    pub idm_base: *mut void __iomem,
    pub nicpm_base: *mut void __iomem,
    pub plat: },
    pub core: *mut bcma_device,
// Reference to CMN core for BCM4706
    pub cmn: *mut bcma_device,
    pub bcma: },
}

// DMA
// Stats
// Int
// Current MAC state
extern "C" {
    pub fn bgmac_enet_probe(bgmac: *mut bgmac) -> c_int;
}
extern "C" {
    pub fn bgmac_enet_remove(bgmac: *mut bgmac);
}
extern "C" {
    pub fn bgmac_adjust_link(net_dev: *mut net_device);
}
extern "C" {
    pub fn bgmac_phy_connect_direct(bgmac: *mut bgmac) -> c_int;
}
extern "C" {
    pub fn bgmac_enet_suspend(bgmac: *mut bgmac) -> c_int;
}
extern "C" {
    pub fn bgmac_enet_resume(bgmac: *mut bgmac) -> c_int;
}
extern "C" {
    pub fn bcma_mdio_mii_unregister(mii_bus: *mut mii_bus);
}
extern "C" {
    pub fn bgmac_read(_arg: bgmac, offset: BGMAC_UNIMAC +) -> return;
}
