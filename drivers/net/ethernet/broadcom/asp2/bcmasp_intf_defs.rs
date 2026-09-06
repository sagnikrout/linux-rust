//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/asp2/bcmasp_intf_defs.h
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

pub const UMC_CMD: c_uint = 0x008;

pub const UMC_CMD_SPEED_SHIFT: c_uint = 0x2;
pub const UMC_CMD_SPEED_MASK: c_uint = 0x3;
pub const UMC_CMD_SPEED_10: c_uint = 0x0;
pub const UMC_CMD_SPEED_100: c_uint = 0x1;
pub const UMC_CMD_SPEED_1000: c_uint = 0x2;
pub const UMC_CMD_SPEED_2500: c_uint = 0x3;

pub const UMC_MAC0: c_uint = 0x0c;
pub const UMC_MAC1: c_uint = 0x10;
pub const UMC_FRM_LEN: c_uint = 0x14;
pub const UMC_EEE_CTRL: c_uint = 0x64;

pub const UMC_EEE_LPI_TIMER: c_uint = 0x68;
pub const UMC_PAUSE_CNTRL: c_uint = 0x330;
pub const UMC_TX_FLUSH: c_uint = 0x334;
pub const UMC_GR64: c_uint = 0x400;
pub const UMC_GR127: c_uint = 0x404;
pub const UMC_GR255: c_uint = 0x408;
pub const UMC_GR511: c_uint = 0x40c;
pub const UMC_GR1023: c_uint = 0x410;
pub const UMC_GR1518: c_uint = 0x414;
pub const UMC_GRMGV: c_uint = 0x418;
pub const UMC_GR2047: c_uint = 0x41c;
pub const UMC_GR4095: c_uint = 0x420;
pub const UMC_GR9216: c_uint = 0x424;
pub const UMC_GRPKT: c_uint = 0x428;
pub const UMC_GRBYT: c_uint = 0x42c;
pub const UMC_GRMCA: c_uint = 0x430;
pub const UMC_GRBCA: c_uint = 0x434;
pub const UMC_GRFCS: c_uint = 0x438;
pub const UMC_GRXCF: c_uint = 0x43c;
pub const UMC_GRXPF: c_uint = 0x440;
pub const UMC_GRXUO: c_uint = 0x444;
pub const UMC_GRALN: c_uint = 0x448;
pub const UMC_GRFLR: c_uint = 0x44c;
pub const UMC_GRCDE: c_uint = 0x450;
pub const UMC_GRFCR: c_uint = 0x454;
pub const UMC_GROVR: c_uint = 0x458;
pub const UMC_GRJBR: c_uint = 0x45c;
pub const UMC_GRMTUE: c_uint = 0x460;
pub const UMC_GRPOK: c_uint = 0x464;
pub const UMC_GRUC: c_uint = 0x468;
pub const UMC_GRPPP: c_uint = 0x46c;
pub const UMC_GRMCRC: c_uint = 0x470;
pub const UMC_TR64: c_uint = 0x480;
pub const UMC_TR127: c_uint = 0x484;
pub const UMC_TR255: c_uint = 0x488;
pub const UMC_TR511: c_uint = 0x48c;
pub const UMC_TR1023: c_uint = 0x490;
pub const UMC_TR1518: c_uint = 0x494;
pub const UMC_TRMGV: c_uint = 0x498;
pub const UMC_TR2047: c_uint = 0x49c;
pub const UMC_TR4095: c_uint = 0x4a0;
pub const UMC_TR9216: c_uint = 0x4a4;
pub const UMC_GTPKT: c_uint = 0x4a8;
pub const UMC_GTMCA: c_uint = 0x4ac;
pub const UMC_GTBCA: c_uint = 0x4b0;
pub const UMC_GTXPF: c_uint = 0x4b4;
pub const UMC_GTXCF: c_uint = 0x4b8;
pub const UMC_GTFCS: c_uint = 0x4bc;
pub const UMC_GTOVR: c_uint = 0x4c0;
pub const UMC_GTDRF: c_uint = 0x4c4;
pub const UMC_GTEDF: c_uint = 0x4c8;
pub const UMC_GTSCL: c_uint = 0x4cc;
pub const UMC_GTMCL: c_uint = 0x4d0;
pub const UMC_GTLCL: c_uint = 0x4d4;
pub const UMC_GTXCL: c_uint = 0x4d8;
pub const UMC_GTFRG: c_uint = 0x4dc;
pub const UMC_GTNCL: c_uint = 0x4e0;
pub const UMC_GTJBR: c_uint = 0x4e4;
pub const UMC_GTBYT: c_uint = 0x4e8;
pub const UMC_GTPOK: c_uint = 0x4ec;
pub const UMC_GTUC: c_uint = 0x4f0;
pub const UMC_RRPKT: c_uint = 0x500;
pub const UMC_RRUND: c_uint = 0x504;
pub const UMC_RRFRG: c_uint = 0x508;
pub const UMC_RRBYT: c_uint = 0x50c;
pub const UMC_MIB_CNTRL: c_uint = 0x580;

pub const UMC_RX_MAX_PKT_SZ: c_uint = 0x608;
pub const UMC_MPD_CTRL: c_uint = 0x620;

pub const UMC_PSW_MS: c_uint = 0x624;
pub const UMC_PSW_LS: c_uint = 0x628;
pub const UMAC2FB_OFFSET: c_uint = 0x9f044;
pub const UMAC2FB_CFG: c_uint = 0x0;

pub const UMAC2FB_CFG_CHID_SHIFT: c_int = 8;
pub const UMAC2FB_CFG_OK_SEND_SHIFT: c_int = 24;

pub const RGMII_EPHY_CNTRL: c_uint = 0x00;

pub const RGMII_OOB_CNTRL: c_uint = 0x0c;

pub const RGMII_PORT_CNTRL: c_uint = 0x60;
pub const RGMII_PORT_MODE_EPHY: c_int = 0;
pub const RGMII_PORT_MODE_GPHY: c_int = 1;
pub const RGMII_PORT_MODE_EXT_EPHY: c_int = 2;
pub const RGMII_PORT_MODE_EXT_GPHY: c_int = 3;
pub const RGMII_PORT_MODE_EXT_RVMII: c_int = 4;

pub const RGMII_SYS_LED_CNTRL: c_uint = 0x74;

pub const TX_SPB_DMA_READ: c_uint = 0x00;
pub const TX_SPB_DMA_BASE: c_uint = 0x08;
pub const TX_SPB_DMA_END: c_uint = 0x10;
pub const TX_SPB_DMA_VALID: c_uint = 0x18;
pub const TX_SPB_DMA_FIFO_CTRL: c_uint = 0x20;

pub const TX_SPB_DMA_FIFO_STATUS: c_uint = 0x24;

pub const TX_SPB_CTRL_ENABLE: c_uint = 0x0;

pub const TX_SPB_CTRL_XF_CTRL2: c_uint = 0x20;
pub const TX_SPB_CTRL_XF_BID_SHIFT: c_int = 16;

pub const TX_SPB_TOP_BLKOUT: c_uint = 0x0;
pub const TX_SPB_TOP_SPRE_BW_CTRL: c_uint = 0x4;

pub const TX_EPKT_C_CFG_MISC: c_uint = 0x0;

pub const TX_EPKT_C_CFG_MISC_PS_SHIFT: c_int = 14;
pub const TX_EPKT_C_CFG_MISC_FD_SHIFT: c_int = 20;

pub const TX_PAUSE_MAP_VECTOR: c_uint = 0x8;

pub const RX_EDPKT_DMA_WRITE: c_uint = 0x00;
pub const RX_EDPKT_DMA_READ: c_uint = 0x08;
pub const RX_EDPKT_DMA_BASE: c_uint = 0x10;
pub const RX_EDPKT_DMA_END: c_uint = 0x18;
pub const RX_EDPKT_DMA_VALID: c_uint = 0x20;
pub const RX_EDPKT_DMA_FULLNESS: c_uint = 0x28;
pub const RX_EDPKT_DMA_MIN_THRES: c_uint = 0x2c;
pub const RX_EDPKT_DMA_CH_XONOFF: c_uint = 0x30;

pub const RX_EDPKT_CFG_CFG0: c_uint = 0x0;
pub const RX_EDPKT_CFG_CFG0_DBUF_SHIFT: c_int = 9;
pub const RX_EDPKT_CFG_CFG0_RBUF: c_uint = 0x0;
pub const RX_EDPKT_CFG_CFG0_RBUF_4K: c_uint = 0x1;
pub const RX_EDPKT_CFG_CFG0_BUF_4K: c_uint = 0x2;
// EFRM STUFF, 0 = no byte stuff, 1 = two byte stuff

pub const RX_EDPKT_CFG_CFG0_BALN_SHIFT: c_int = 12;
pub const RX_EDPKT_CFG_CFG0_NO_ALN: c_int = 0;
pub const RX_EDPKT_CFG_CFG0_4_ALN: c_int = 2;
pub const RX_EDPKT_CFG_CFG0_64_ALN: c_int = 6;
pub const RX_EDPKT_RING_BUFFER_WRITE: c_uint = 0x38;
pub const RX_EDPKT_RING_BUFFER_READ: c_uint = 0x40;
pub const RX_EDPKT_RING_BUFFER_BASE: c_uint = 0x48;
pub const RX_EDPKT_RING_BUFFER_END: c_uint = 0x50;
pub const RX_EDPKT_RING_BUFFER_VALID: c_uint = 0x58;
pub const RX_EDPKT_CFG_ENABLE: c_uint = 0x6c;

pub const RX_SPB_DMA_READ: c_uint = 0x00;
pub const RX_SPB_DMA_BASE: c_uint = 0x08;
pub const RX_SPB_DMA_END: c_uint = 0x10;
pub const RX_SPB_DMA_VALID: c_uint = 0x18;
pub const RX_SPB_DMA_FIFO_CTRL: c_uint = 0x20;

pub const RX_SPB_DMA_FIFO_STATUS: c_uint = 0x24;

pub const RX_SPB_CTRL_ENABLE: c_uint = 0x00;

pub const RX_PAUSE_MAP_VECTOR: c_uint = 0x00;

pub const RX_SPB_TOP_BLKOUT: c_uint = 0x00;
//
// Number of 4 KB pages that make up the contiguous RBUF_4K streaming ring
// and the page pool used as copy-target SKB data areas.
//
pub const NUM_4K_BUFFERS: c_int = 32;

pub const DESC_SIZE: c_int = 16;

