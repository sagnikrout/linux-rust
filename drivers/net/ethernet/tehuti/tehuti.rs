//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/tehuti/tehuti.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Tehuti Networks(R) Network Driver
// Copyright (C) 2007 Tehuti Networks Ltd. All rights reserved
//

// Compile Time Switches
// start
// Macro flag: #define BDX_TSO
// Macro flag: #define BDX_LLTX
// Macro flag: #define BDX_DELAY_WPTR
// #define BDX_MSI
// end

// ioctl ops
pub const BDX_OP_READ: c_int = 1;
pub const BDX_OP_WRITE: c_int = 2;
// RX copy break size
pub const BDX_COPYBREAK: c_int = 257;

// netdev tx queue len for Luxor. default value is, btw, 1000
// ifcontig eth1 txqueuelen 3000 - to change it at runtime
pub const BDX_NDEV_TXQ_LEN: c_int = 3000;
// Max MTU for Jumbo Frame mode, per tehutinetworks.net Features FAQ is 16k

pub const FIFO_SIZE: c_int = 4096;
pub const FIFO_EXTRA_SPACE: c_int = 1024;

pub const LUXOR_MAX_PORT: c_int = 2;
pub const BDX_MAX_RX_DONE: c_int = 150;
pub const BDX_TXF_DESC_SZ: c_int = 16;

pub const BDX_MIN_TX_LEVEL: c_int = 256;
pub const BDX_NO_UPD_PACKETS: c_int = 40;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_nic {
    pub port_num: c_int,
    pub regs: *mut void __iomem,
    pub irq_type: c_int,
    pub priv: [*mut bdx_priv; LUXOR_MAX_PORT],
}

pub const PCK_TH_MULT: c_int = 128;
pub const INT_COAL_MULT: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fifo {
    pub /: *mut *mut dma_addr_t da; / physical address of fifo (used by HW),
    pub /: *mut *mut *mut char va; / virtual address of fifo (used by SW),
    pub registers,: *mut *mut u32 rptr, wptr; / cached values of RPTR and WPTR,
    pub reg_CFG1: u16 reg_CFG0,,
    pub reg_WPTR: u16 reg_RPTR,,
    pub /: *mut *mut u16 memsz; / memory size allocated for fifo,
    pub size_mask: u16,
    pub /: *mut *mut u16 pktsz; / skb packet size to allocate,
    pub /: *mut *mut u16 rcvno; / number of buffers that come from this RXF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct txf_fifo {
    pub /: *mut *mut fifo m; / minimal set of variables used by all fifos,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct txd_fifo {
    pub /: *mut *mut fifo m; / minimal set of variables used by all fifos,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxf_fifo {
    pub /: *mut *mut fifo m; / minimal set of variables used by all fifos,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxd_fifo {
    pub /: *mut *mut fifo m; / minimal set of variables used by all fifos,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_map {
    pub dma: u64,
    pub skb: *mut sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxdb {
    pub stack: *mut c_int,
    pub elems: *mut rx_map,
    pub nelem: c_int,
    pub top: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union bdx_dma_addr {
    pub dma: dma_addr_t,
    pub skb: *mut sk_buff,
}

// Entry in the db.
// if len == 0 addr is dma
// if len != 0 addr is skb
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_map {
    pub addr: bdx_dma_addr,
    pub len: c_int,
}

// tx database - implemented as circular fifo buffer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct txdb {
    pub /: *mut *mut *mut tx_map start; / points to the first element,
    pub /: *mut *mut *mut tx_map end; / points just AFTER the last element,
    pub /: *mut *mut *mut tx_map rptr; / points to the next element to read,
    pub /: *mut *mut *mut tx_map wptr; / points to the next element to write,
    pub /: *mut *mut int size; / number of elements in the db,
}

// Internal stats structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdx_stats {
    pub /: *mut *mut u64 InUCast; / 0x7200,
    pub /: *mut *mut u64 InMCast; / 0x7210,
    pub /: *mut *mut u64 InBCast; / 0x7220,
    pub /: *mut *mut u64 InPkts; / 0x7230,
    pub /: *mut *mut u64 InErrors; / 0x7240,
    pub /: *mut *mut u64 InDropped; / 0x7250,
    pub /: *mut *mut u64 FrameTooLong; / 0x7260,
    pub /: *mut *mut u64 FrameSequenceErrors; / 0x7270,
    pub /: *mut *mut u64 InVLAN; / 0x7280,
    pub /: *mut *mut u64 InDroppedDFE; / 0x7290,
    pub /: *mut *mut u64 InDroppedIntFull; / 0x72A0,
    pub /: *mut *mut u64 InFrameAlignErrors; / 0x72B0,
// 0x72C0-0x72E0 RSRV
    pub /: *mut *mut u64 OutUCast; / 0x72F0,
    pub /: *mut *mut u64 OutMCast; / 0x7300,
    pub /: *mut *mut u64 OutBCast; / 0x7310,
    pub /: *mut *mut u64 OutPkts; / 0x7320,
// 0x7330-0x7360 RSRV
    pub /: *mut *mut u64 OutVLAN; / 0x7370,
    pub /: *mut *mut u64 InUCastOctects; / 0x7380,
    pub /: *mut *mut u64 OutUCastOctects; / 0x7390,
// 0x73A0-0x73B0 RSRV
    pub /: *mut *mut u64 InBCastOctects; / 0x73C0,
    pub /: *mut *mut u64 OutBCastOctects; / 0x73D0,
    pub /: *mut *mut u64 InOctects; / 0x73E0,
    pub /: *mut *mut u64 OutOctects; / 0x73F0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdx_priv {
    pub pBdxRegs: *mut void __iomem,
    pub ndev: *mut net_device,
    pub napi: napi_struct,
// RX FIFOs: 1 for data (full) descs, and 2 for free descs
    pub rxd_fifo0: rxd_fifo,
    pub rxf_fifo0: rxf_fifo,
    pub /: *mut *mut *mut rxdb rxdb; / rx dbs to store skb pointers,
    pub napi_stop: c_int,
// Tx FIFOs: 1 for data desc, 1 for empty (acks) desc
    pub txd_fifo0: txd_fifo,
    pub txf_fifo0: txf_fifo,
    pub txdb: txdb,
    pub tx_level: c_int,

    pub tx_update_mark: c_int,
    pub tx_noupd: c_int,

    pub /: *mut *mut spinlock_t tx_lock; / dev->lltx mode,
// rarely used
    pub port: u8,
    pub msg_enable: u32,
    pub stats_flag: c_int,
    pub hw_stats: bdx_stats,
    pub pdev: *mut pci_dev,
    pub nic: *mut pci_nic,
    pub txd_size: u8,
    pub txf_size: u8,
    pub rxd_size: u8,
    pub rxf_size: u8,
    pub rdintcm: u32,
    pub tdintcm: u32,
}

// RX FREE descriptor - 64bit
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxf_desc {
    pub /: *mut *mut u32 info; / Buffer Count + Info - described below,
    pub /: *mut *mut u32 va_lo; / VAdr[31:0],
    pub /: *mut *mut u32 va_hi; / VAdr[63:32],
    pub /: *mut *mut u32 pa_lo; / PAdr[31:0],
    pub /: *mut *mut u32 pa_hi; / PAdr[63:32],
    pub /: *mut *mut u32 len; / Buffer Length,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxd_desc {
    pub rxd_val1: u32,
    pub len: u16,
    pub rxd_vlan: u16,
    pub va_lo: u32,
    pub va_hi: u32,
}

// PBL describes each virtual buffer to be
// transmitted from the host.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pbl {
    pub pa_lo: u32,
    pub pa_hi: u32,
    pub len: u32,
}

// First word for TXD descriptor. It means: type = 3 for regular Tx packet,
// hw_csum = 7 for ip+udp+tcp hw checksums

#[repr(C)]
#[derive(Copy, Clone)]
pub struct txd_desc {
    pub txd_val1: u32,
    pub mss: u16,
    pub length: u16,
    pub va_lo: u32,
    pub va_hi: u32,
    pub /: *mut *mut pbl pbl[]; / Fragments,
    pub __packed: },
// Register region size
pub const BDX_REGS_SIZE: c_uint = 0x1000;
// Registers from 0x0000-0x00fc were remapped to 0x4000-0x40fc
pub const regTXD_CFG1_0: c_uint = 0x4000;
pub const regRXF_CFG1_0: c_uint = 0x4010;
pub const regRXD_CFG1_0: c_uint = 0x4020;
pub const regTXF_CFG1_0: c_uint = 0x4030;
pub const regTXD_CFG0_0: c_uint = 0x4040;
pub const regRXF_CFG0_0: c_uint = 0x4050;
pub const regRXD_CFG0_0: c_uint = 0x4060;
pub const regTXF_CFG0_0: c_uint = 0x4070;
pub const regTXD_WPTR_0: c_uint = 0x4080;
pub const regRXF_WPTR_0: c_uint = 0x4090;
pub const regRXD_WPTR_0: c_uint = 0x40A0;
pub const regTXF_WPTR_0: c_uint = 0x40B0;
pub const regTXD_RPTR_0: c_uint = 0x40C0;
pub const regRXF_RPTR_0: c_uint = 0x40D0;
pub const regRXD_RPTR_0: c_uint = 0x40E0;
pub const regTXF_RPTR_0: c_uint = 0x40F0;
pub const regTXF_RPTR_3: c_uint = 0x40FC;
// hardware versioning
pub const FW_VER: c_uint = 0x5010;
pub const SROM_VER: c_uint = 0x5020;
pub const FPGA_VER: c_uint = 0x5030;
pub const FPGA_SEED: c_uint = 0x5040;
// Registers from 0x0100-0x0150 were remapped to 0x5100-0x5150

pub const regISR0: c_uint = 0x5100;

pub const regIMR0: c_uint = 0x5110;
pub const regRDINTCM0: c_uint = 0x5120;
pub const regRDINTCM2: c_uint = 0x5128;
pub const regTDINTCM0: c_uint = 0x5130;
pub const regISR_MSK0: c_uint = 0x5140;
pub const regINIT_SEMAPHORE: c_uint = 0x5170;
pub const regINIT_STATUS: c_uint = 0x5180;
pub const regMAC_LNK_STAT: c_uint = 0x0200;
pub const MAC_LINK_STAT: c_uint = 0x4	/* Link state */;
pub const regGMAC_RXF_A: c_uint = 0x1240;
pub const regUNC_MAC0_A: c_uint = 0x1250;
pub const regUNC_MAC1_A: c_uint = 0x1260;
pub const regUNC_MAC2_A: c_uint = 0x1270;
pub const regVLAN_0: c_uint = 0x1800;
pub const regMAX_FRAME_A: c_uint = 0x12C0;
pub const regRX_MAC_MCST0: c_uint = 0x1A80;
pub const regRX_MAC_MCST1: c_uint = 0x1A84;
pub const MAC_MCST_NUM: c_int = 15;
pub const regRX_MCST_HASH0: c_uint = 0x1A00;
pub const MAC_MCST_HASH_NUM: c_int = 8;
pub const regVPC: c_uint = 0x2300;
pub const regVIC: c_uint = 0x2320;
pub const regVGLB: c_uint = 0x2340;
pub const regCLKPLL: c_uint = 0x5000;
// for 10G only
pub const regREVISION: c_uint = 0x6000;
pub const regSCRATCH: c_uint = 0x6004;
pub const regCTRLST: c_uint = 0x6008;
pub const regMAC_ADDR_0: c_uint = 0x600C;
pub const regMAC_ADDR_1: c_uint = 0x6010;
pub const regFRM_LENGTH: c_uint = 0x6014;
pub const regPAUSE_QUANT: c_uint = 0x6018;
pub const regRX_FIFO_SECTION: c_uint = 0x601C;
pub const regTX_FIFO_SECTION: c_uint = 0x6020;
pub const regRX_FULLNESS: c_uint = 0x6024;
pub const regTX_FULLNESS: c_uint = 0x6028;
pub const regHASHTABLE: c_uint = 0x602C;
pub const regMDIO_ST: c_uint = 0x6030;
pub const regMDIO_CTL: c_uint = 0x6034;
pub const regMDIO_DATA: c_uint = 0x6038;
pub const regMDIO_ADDR: c_uint = 0x603C;
pub const regRST_PORT: c_uint = 0x7000;
pub const regDIS_PORT: c_uint = 0x7010;
pub const regRST_QU: c_uint = 0x7020;
pub const regDIS_QU: c_uint = 0x7030;
pub const regCTRLST_TX_ENA: c_uint = 0x0001;
pub const regCTRLST_RX_ENA: c_uint = 0x0002;
pub const regCTRLST_PRM_ENA: c_uint = 0x0010;
pub const regCTRLST_PAD_ENA: c_uint = 0x0020;

pub const regRX_FLT: c_uint = 0x1400;
// TXD TXF RXF RXD  CONFIG 0x0000 --- 0x007c
pub const TX_RX_CFG1_BASE: c_uint = 0xffffffff	/*0-31 */;
pub const TX_RX_CFG0_BASE: c_uint = 0xfffff000	/*31:12 */;
pub const TX_RX_CFG0_RSVD: c_uint = 0x0ffc	/*11:2 */;
pub const TX_RX_CFG0_SIZE: c_uint = 0x0003	/*1:0 */;
// TXD TXF RXF RXD  WRITE 0x0080 --- 0x00BC
pub const TXF_WPTR_WR_PTR: c_uint = 0x7ff8	/*14:3 */;
// TXD TXF RXF RXD  READ  0x00CO --- 0x00FC
pub const TXF_RPTR_RD_PTR: c_uint = 0x7ff8	/*14:3 */;
pub const TXF_WPTR_MASK: c_uint = 0x7ff0	/* last 4 bits are dropped;
// size is rounded to 16
// regISR 0x0100
// regIMR 0x0110
pub const IMR_INPROG: c_uint = 0x80000000	/*31 */;
pub const IR_LNKCHG1: c_uint = 0x10000000	/*28 */;
pub const IR_LNKCHG0: c_uint = 0x08000000	/*27 */;
pub const IR_GPIO: c_uint = 0x04000000	/*26 */;
pub const IR_RFRSH: c_uint = 0x02000000	/*25 */;
pub const IR_RSVD: c_uint = 0x01000000	/*24 */;
pub const IR_SWI: c_uint = 0x00800000	/*23 */;
pub const IR_RX_FREE_3: c_uint = 0x00400000	/*22 */;
pub const IR_RX_FREE_2: c_uint = 0x00200000	/*21 */;
pub const IR_RX_FREE_1: c_uint = 0x00100000	/*20 */;
pub const IR_RX_FREE_0: c_uint = 0x00080000	/*19 */;
pub const IR_TX_FREE_3: c_uint = 0x00040000	/*18 */;
pub const IR_TX_FREE_2: c_uint = 0x00020000	/*17 */;
pub const IR_TX_FREE_1: c_uint = 0x00010000	/*16 */;
pub const IR_TX_FREE_0: c_uint = 0x00008000	/*15 */;
pub const IR_RX_DESC_3: c_uint = 0x00004000	/*14 */;
pub const IR_RX_DESC_2: c_uint = 0x00002000	/*13 */;
pub const IR_RX_DESC_1: c_uint = 0x00001000	/*12 */;
pub const IR_RX_DESC_0: c_uint = 0x00000800	/*11 */;
pub const IR_PSE: c_uint = 0x00000400	/*10 */;
pub const IR_TMR3: c_uint = 0x00000200	/*9 */;
pub const IR_TMR2: c_uint = 0x00000100	/*8 */;
pub const IR_TMR1: c_uint = 0x00000080	/*7 */;
pub const IR_TMR0: c_uint = 0x00000040	/*6 */;
pub const IR_VNT: c_uint = 0x00000020	/*5 */;
pub const IR_RxFL: c_uint = 0x00000010	/*4 */;
pub const IR_SDPERR: c_uint = 0x00000008	/*3 */;
pub const IR_TR: c_uint = 0x00000004	/*2 */;
pub const IR_PCIE_LINK: c_uint = 0x00000002	/*1 */;
pub const IR_PCIE_TOUT: c_uint = 0x00000001	/*0 */;

pub const IR_ALL: c_uint = 0xfdfffff7;
pub const IR_LNKCHG0_ofst: c_int = 27;
pub const GMAC_RX_FILTER_OSEN: c_uint = 0x1000	/* shared OS enable */;
pub const GMAC_RX_FILTER_TXFC: c_uint = 0x0400	/* Tx flow control */;
pub const GMAC_RX_FILTER_RSV0: c_uint = 0x0200	/* reserved */;
pub const GMAC_RX_FILTER_FDA: c_uint = 0x0100	/* filter out direct address */;
pub const GMAC_RX_FILTER_AOF: c_uint = 0x0080	/* accept over run */;
pub const GMAC_RX_FILTER_ACF: c_uint = 0x0040	/* accept control frames */;
pub const GMAC_RX_FILTER_ARUNT: c_uint = 0x0020	/* accept under run */;
pub const GMAC_RX_FILTER_ACRC: c_uint = 0x0010	/* accept crc error */;
pub const GMAC_RX_FILTER_AM: c_uint = 0x0008	/* accept multicast */;
pub const GMAC_RX_FILTER_AB: c_uint = 0x0004	/* accept broadcast */;
pub const GMAC_RX_FILTER_PRM: c_uint = 0x0001	/* [0:1] promiscuous mode */;
pub const MAX_FRAME_AB_VAL: c_uint = 0x3fff	/* 13:0 */;
pub const CLKPLL_PLLLKD: c_uint = 0x0200	/*9 */;
pub const CLKPLL_RSTEND: c_uint = 0x0100	/*8 */;
pub const CLKPLL_SFTRST: c_uint = 0x0001	/*0 */;

//
// PCI-E Device Control Register (Offset 0x88)
// Source: Luxor Data Sheet, 7.1.3.3.3
//
pub const PCI_DEV_CTRL_REG: c_uint = 0x88;

//
// PCI-E Link Status Register (Offset 0x92)
// Source: Luxor Data Sheet, 7.1.3.3.7
//
pub const PCI_LINK_STATUS_REG: c_uint = 0x92;

// Debugging Macros

    pub \: pr_err("%s:%-5d: ENTER\n", __func__, __LINE__);,

    pub \: pr_err("%s:%-5d: RETURN\n", __func__, __LINE__);,
    pub \: return args;,

    pub \: pr_err(fmt, ##args);,

