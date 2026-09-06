//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bcm4908_enet.h
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
pub const ENET_CONTROL: c_uint = 0x000;
pub const ENET_MIB_CTRL: c_uint = 0x004;
pub const ENET_MIB_CTRL_CLR_MIB: c_uint = 0x00000001;
pub const ENET_RX_ERR_MASK: c_uint = 0x008;
pub const ENET_MIB_MAX_PKT_SIZE: c_uint = 0x00C;
pub const ENET_MIB_MAX_PKT_SIZE_VAL: c_uint = 0x00003fff;
pub const ENET_DIAG_OUT: c_uint = 0x01c;
pub const ENET_ENABLE_DROP_PKT: c_uint = 0x020;
pub const ENET_IRQ_ENABLE: c_uint = 0x024;
pub const ENET_IRQ_ENABLE_OVFL: c_uint = 0x00000001;
pub const ENET_GMAC_STATUS: c_uint = 0x028;
pub const ENET_GMAC_STATUS_ETH_SPEED_MASK: c_uint = 0x00000003;
pub const ENET_GMAC_STATUS_ETH_SPEED_10: c_uint = 0x00000000;
pub const ENET_GMAC_STATUS_ETH_SPEED_100: c_uint = 0x00000001;
pub const ENET_GMAC_STATUS_ETH_SPEED_1000: c_uint = 0x00000002;
pub const ENET_GMAC_STATUS_HD: c_uint = 0x00000004;
pub const ENET_GMAC_STATUS_AUTO_CFG_EN: c_uint = 0x00000008;
pub const ENET_GMAC_STATUS_LINK_UP: c_uint = 0x00000010;
pub const ENET_IRQ_STATUS: c_uint = 0x02c;
pub const ENET_IRQ_STATUS_OVFL: c_uint = 0x00000001;
pub const ENET_OVERFLOW_COUNTER: c_uint = 0x030;
pub const ENET_FLUSH: c_uint = 0x034;
pub const ENET_FLUSH_RXFIFO_FLUSH: c_uint = 0x00000001;
pub const ENET_FLUSH_TXFIFO_FLUSH: c_uint = 0x00000002;
pub const ENET_RSV_SELECT: c_uint = 0x038;
pub const ENET_BP_FORCE: c_uint = 0x03c;
pub const ENET_BP_FORCE_FORCE: c_uint = 0x00000001;
pub const ENET_DMA_RX_OK_TO_SEND_COUNT: c_uint = 0x040;
pub const ENET_DMA_RX_OK_TO_SEND_COUNT_VAL: c_uint = 0x0000000f;
pub const ENET_TX_CRC_CTRL: c_uint = 0x044;
pub const ENET_MIB: c_uint = 0x200;
pub const ENET_UNIMAC: c_uint = 0x400;
pub const ENET_DMA: c_uint = 0x800;
pub const ENET_DMA_CONTROLLER_CFG: c_uint = 0x800;
pub const ENET_DMA_CTRL_CFG_MASTER_EN: c_uint = 0x00000001;
pub const ENET_DMA_CTRL_CFG_FLOWC_CH1_EN: c_uint = 0x00000002;
pub const ENET_DMA_CTRL_CFG_FLOWC_CH3_EN: c_uint = 0x00000004;
pub const ENET_DMA_FLOWCTL_CH1_THRESH_LO: c_uint = 0x804;
pub const ENET_DMA_FLOWCTL_CH1_THRESH_HI: c_uint = 0x808;
pub const ENET_DMA_FLOWCTL_CH1_ALLOC: c_uint = 0x80c;
pub const ENET_DMA_FLOWCTL_CH1_ALLOC_FORCE: c_uint = 0x80000000;
pub const ENET_DMA_FLOWCTL_CH3_THRESH_LO: c_uint = 0x810;
pub const ENET_DMA_FLOWCTL_CH3_THRESH_HI: c_uint = 0x814;
pub const ENET_DMA_FLOWCTL_CH3_ALLOC: c_uint = 0x818;
pub const ENET_DMA_FLOWCTL_CH5_THRESH_LO: c_uint = 0x81C;
pub const ENET_DMA_FLOWCTL_CH5_THRESH_HI: c_uint = 0x820;
pub const ENET_DMA_FLOWCTL_CH5_ALLOC: c_uint = 0x824;
pub const ENET_DMA_FLOWCTL_CH7_THRESH_LO: c_uint = 0x828;
pub const ENET_DMA_FLOWCTL_CH7_THRESH_HI: c_uint = 0x82C;
pub const ENET_DMA_FLOWCTL_CH7_ALLOC: c_uint = 0x830;
pub const ENET_DMA_CTRL_CHANNEL_RESET: c_uint = 0x834;
pub const ENET_DMA_CTRL_CHANNEL_DEBUG: c_uint = 0x838;
pub const ENET_DMA_CTRL_GLOBAL_INTERRUPT_STATUS: c_uint = 0x840;
pub const ENET_DMA_CTRL_GLOBAL_INTERRUPT_MASK: c_uint = 0x844;
pub const ENET_DMA_CH0_CFG: c_uint = 0xa00		/* RX */;
pub const ENET_DMA_CH1_CFG: c_uint = 0xa10		/* TX */;
pub const ENET_DMA_CH0_STATE_RAM: c_uint = 0xc00		/* RX */;
pub const ENET_DMA_CH1_STATE_RAM: c_uint = 0xc10		/* TX */;
pub const ENET_DMA_CH_CFG: c_uint = 0x00		/* assorted configuration */;
pub const ENET_DMA_CH_CFG_ENABLE: c_uint = 0x00000001	/* set to enable channel */;
pub const ENET_DMA_CH_CFG_PKT_HALT: c_uint = 0x00000002	/* idle after an EOP flag is detected */;
pub const ENET_DMA_CH_CFG_BURST_HALT: c_uint = 0x00000004	/* idle after finish current memory burst */;
pub const ENET_DMA_CH_CFG_INT_STAT: c_uint = 0x04		/* interrupts control and status */;
pub const ENET_DMA_CH_CFG_INT_MASK: c_uint = 0x08		/* interrupts mask */;
pub const ENET_DMA_CH_CFG_INT_BUFF_DONE: c_uint = 0x00000001	/* buffer done */;
pub const ENET_DMA_CH_CFG_INT_DONE: c_uint = 0x00000002	/* packet xfer complete */;
pub const ENET_DMA_CH_CFG_INT_NO_DESC: c_uint = 0x00000004	/* no valid descriptors */;
pub const ENET_DMA_CH_CFG_INT_RX_ERROR: c_uint = 0x00000008	/* rxdma detect client protocol error */;
pub const ENET_DMA_CH_CFG_MAX_BURST: c_uint = 0x0c		/* max burst length permitted */;
pub const ENET_DMA_CH_CFG_MAX_BURST_DESCSIZE_SEL: c_uint = 0x00040000	/* DMA Descriptor Size Selection */;
pub const ENET_DMA_CH_CFG_SIZE: c_uint = 0x10;
pub const ENET_DMA_CH_STATE_RAM_BASE_DESC_PTR: c_uint = 0x00		/* descriptor ring start address */;
pub const ENET_DMA_CH_STATE_RAM_STATE_DATA: c_uint = 0x04		/* state/bytes done/ring offset */;
pub const ENET_DMA_CH_STATE_RAM_DESC_LEN_STATUS: c_uint = 0x08		/* buffer descriptor status and len */;
pub const ENET_DMA_CH_STATE_RAM_DESC_BASE_BUFPTR: c_uint = 0x0c		/* buffer descrpitor current processing */;
pub const ENET_DMA_CH_STATE_RAM_SIZE: c_uint = 0x10;
pub const DMA_CTL_STATUS_APPEND_CRC: c_uint = 0x00000100;
pub const DMA_CTL_STATUS_APPEND_BRCM_TAG: c_uint = 0x00000200;
pub const DMA_CTL_STATUS_PRIO: c_uint = 0x00000C00  /* Prio for Tx */;
pub const DMA_CTL_STATUS_WRAP: c_uint = 0x00001000  /* */;
pub const DMA_CTL_STATUS_SOP: c_uint = 0x00002000  /* first buffer in packet */;
pub const DMA_CTL_STATUS_EOP: c_uint = 0x00004000  /* last buffer in packet */;
pub const DMA_CTL_STATUS_OWN: c_uint = 0x00008000  /* cleared by DMA, set by SW */;
pub const DMA_CTL_LEN_DESC_BUFLENGTH: c_uint = 0x0fff0000;
pub const DMA_CTL_LEN_DESC_BUFLENGTH_SHIFT: c_int = 16;
pub const DMA_CTL_LEN_DESC_MULTICAST: c_uint = 0x40000000;
pub const DMA_CTL_LEN_DESC_USEFPM: c_uint = 0x80000000;
