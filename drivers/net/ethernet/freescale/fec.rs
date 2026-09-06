//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/freescale/fec.h
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
// fec.h  --  Fast Ethernet Controller for Motorola ColdFire SoC
// processors.
//
// (C) Copyright 2000-2005, Greg Ungerer (gerg@snapgear.com)
// (C) Copyright 2000-2001, Lineo (www.lineo.com)
//

//
// Just figures, Motorola would have to change the offsets for
// registers in the same peripheral device on different models
// of the ColdFire!
//
pub const FEC_IEVENT: c_uint = 0x004 /* Interrupt event reg */;
pub const FEC_IMASK: c_uint = 0x008 /* Interrupt mask reg */;
pub const FEC_R_DES_ACTIVE_0: c_uint = 0x010 /* Receive descriptor reg */;
pub const FEC_X_DES_ACTIVE_0: c_uint = 0x014 /* Transmit descriptor reg */;
pub const FEC_ECNTRL: c_uint = 0x024 /* Ethernet control reg */;
pub const FEC_MII_DATA: c_uint = 0x040 /* MII manage frame reg */;
pub const FEC_MII_SPEED: c_uint = 0x044 /* MII speed control reg */;
pub const FEC_MIB_CTRLSTAT: c_uint = 0x064 /* MIB control/status reg */;
pub const FEC_R_CNTRL: c_uint = 0x084 /* Receive control reg */;
pub const FEC_X_CNTRL: c_uint = 0x0c4 /* Transmit Control reg */;
pub const FEC_ADDR_LOW: c_uint = 0x0e4 /* Low 32bits MAC address */;
pub const FEC_ADDR_HIGH: c_uint = 0x0e8 /* High 16bits MAC address */;
pub const FEC_OPD: c_uint = 0x0ec /* Opcode + Pause duration */;
pub const FEC_TXIC0: c_uint = 0x0f0 /* Tx Interrupt Coalescing for ring 0 */;
pub const FEC_TXIC1: c_uint = 0x0f4 /* Tx Interrupt Coalescing for ring 1 */;
pub const FEC_TXIC2: c_uint = 0x0f8 /* Tx Interrupt Coalescing for ring 2 */;
pub const FEC_RXIC0: c_uint = 0x100 /* Rx Interrupt Coalescing for ring 0 */;
pub const FEC_RXIC1: c_uint = 0x104 /* Rx Interrupt Coalescing for ring 1 */;
pub const FEC_RXIC2: c_uint = 0x108 /* Rx Interrupt Coalescing for ring 2 */;
pub const FEC_HASH_TABLE_HIGH: c_uint = 0x118 /* High 32bits hash table */;
pub const FEC_HASH_TABLE_LOW: c_uint = 0x11c /* Low 32bits hash table */;
pub const FEC_GRP_HASH_TABLE_HIGH: c_uint = 0x120 /* High 32bits hash table */;
pub const FEC_GRP_HASH_TABLE_LOW: c_uint = 0x124 /* Low 32bits hash table */;
pub const FEC_X_WMRK: c_uint = 0x144 /* FIFO transmit water mark */;
pub const FEC_R_BOUND: c_uint = 0x14c /* FIFO receive bound reg */;
pub const FEC_R_FSTART: c_uint = 0x150 /* FIFO receive start reg */;
pub const FEC_R_DES_START_1: c_uint = 0x160 /* Receive descriptor ring 1 */;
pub const FEC_X_DES_START_1: c_uint = 0x164 /* Transmit descriptor ring 1 */;
pub const FEC_R_BUFF_SIZE_1: c_uint = 0x168 /* Maximum receive buff ring1 size */;
pub const FEC_R_DES_START_2: c_uint = 0x16c /* Receive descriptor ring 2 */;
pub const FEC_X_DES_START_2: c_uint = 0x170 /* Transmit descriptor ring 2 */;
pub const FEC_R_BUFF_SIZE_2: c_uint = 0x174 /* Maximum receive buff ring2 size */;
pub const FEC_R_DES_START_0: c_uint = 0x180 /* Receive descriptor ring */;
pub const FEC_X_DES_START_0: c_uint = 0x184 /* Transmit descriptor ring */;
pub const FEC_R_BUFF_SIZE_0: c_uint = 0x188 /* Maximum receive buff size */;
pub const FEC_R_FIFO_RSFL: c_uint = 0x190 /* Receive FIFO section full threshold */;
pub const FEC_R_FIFO_RSEM: c_uint = 0x194 /* Receive FIFO section empty threshold */;
pub const FEC_R_FIFO_RAEM: c_uint = 0x198 /* Receive FIFO almost empty threshold */;
pub const FEC_R_FIFO_RAFL: c_uint = 0x19c /* Receive FIFO almost full threshold */;
pub const FEC_FTRL: c_uint = 0x1b0 /* Frame truncation receive length*/;
pub const FEC_RACC: c_uint = 0x1c4 /* Receive Accelerator function */;
pub const FEC_RCMR_1: c_uint = 0x1c8 /* Receive classification match ring 1 */;
pub const FEC_RCMR_2: c_uint = 0x1cc /* Receive classification match ring 2 */;
pub const FEC_DMA_CFG_1: c_uint = 0x1d8 /* DMA class configuration for ring 1 */;
pub const FEC_DMA_CFG_2: c_uint = 0x1dc /* DMA class Configuration for ring 2 */;
pub const FEC_R_DES_ACTIVE_1: c_uint = 0x1e0 /* Rx descriptor active for ring 1 */;
pub const FEC_X_DES_ACTIVE_1: c_uint = 0x1e4 /* Tx descriptor active for ring 1 */;
pub const FEC_R_DES_ACTIVE_2: c_uint = 0x1e8 /* Rx descriptor active for ring 2 */;
pub const FEC_X_DES_ACTIVE_2: c_uint = 0x1ec /* Tx descriptor active for ring 2 */;
pub const FEC_QOS_SCHEME: c_uint = 0x1f0 /* Set multi queues Qos scheme */;
pub const FEC_LPI_SLEEP: c_uint = 0x1f4 /* Set IEEE802.3az LPI Sleep Ts time */;
pub const FEC_LPI_WAKE: c_uint = 0x1f8 /* Set IEEE802.3az LPI Wake Tw time */;
pub const FEC_MIIGSK_CFGR: c_uint = 0x300 /* MIIGSK Configuration reg */;
pub const FEC_MIIGSK_ENR: c_uint = 0x308 /* MIIGSK Enable reg */;
pub const BM_MIIGSK_CFGR_MII: c_uint = 0x00;
pub const BM_MIIGSK_CFGR_RMII: c_uint = 0x01;
pub const BM_MIIGSK_CFGR_FRCONT_10M: c_uint = 0x40;
pub const RMON_T_DROP: c_uint = 0x200 /* Count of frames not cntd correctly */;
pub const RMON_T_PACKETS: c_uint = 0x204 /* RMON TX packet count */;
pub const RMON_T_BC_PKT: c_uint = 0x208 /* RMON TX broadcast pkts */;
pub const RMON_T_MC_PKT: c_uint = 0x20c /* RMON TX multicast pkts */;
pub const RMON_T_CRC_ALIGN: c_uint = 0x210 /* RMON TX pkts with CRC align err */;
pub const RMON_T_UNDERSIZE: c_uint = 0x214 /* RMON TX pkts < 64 bytes, good CRC */;
pub const RMON_T_OVERSIZE: c_uint = 0x218 /* RMON TX pkts > MAX_FL bytes good CRC */;
pub const RMON_T_FRAG: c_uint = 0x21c /* RMON TX pkts < 64 bytes, bad CRC */;
pub const RMON_T_JAB: c_uint = 0x220 /* RMON TX pkts > MAX_FL bytes, bad CRC */;
pub const RMON_T_COL: c_uint = 0x224 /* RMON TX collision count */;
pub const RMON_T_P64: c_uint = 0x228 /* RMON TX 64 byte pkts */;
pub const RMON_T_P65TO127: c_uint = 0x22c /* RMON TX 65 to 127 byte pkts */;
pub const RMON_T_P128TO255: c_uint = 0x230 /* RMON TX 128 to 255 byte pkts */;
pub const RMON_T_P256TO511: c_uint = 0x234 /* RMON TX 256 to 511 byte pkts */;
pub const RMON_T_P512TO1023: c_uint = 0x238 /* RMON TX 512 to 1023 byte pkts */;
pub const RMON_T_P1024TO2047: c_uint = 0x23c /* RMON TX 1024 to 2047 byte pkts */;
pub const RMON_T_P_GTE2048: c_uint = 0x240 /* RMON TX pkts > 2048 bytes */;
pub const RMON_T_OCTETS: c_uint = 0x244 /* RMON TX octets */;
pub const IEEE_T_DROP: c_uint = 0x248 /* Count of frames not counted crtly */;
pub const IEEE_T_FRAME_OK: c_uint = 0x24c /* Frames tx'd OK */;
pub const IEEE_T_1COL: c_uint = 0x250 /* Frames tx'd with single collision */;
pub const IEEE_T_MCOL: c_uint = 0x254 /* Frames tx'd with multiple collision */;
pub const IEEE_T_DEF: c_uint = 0x258 /* Frames tx'd after deferral delay */;
pub const IEEE_T_LCOL: c_uint = 0x25c /* Frames tx'd with late collision */;
pub const IEEE_T_EXCOL: c_uint = 0x260 /* Frames tx'd with excessive collisions */;
pub const IEEE_T_MACERR: c_uint = 0x264 /* Frames tx'd with TX FIFO underrun */;
pub const IEEE_T_CSERR: c_uint = 0x268 /* Frames tx'd with carrier sense err */;
pub const IEEE_T_SQE: c_uint = 0x26c /* Frames tx'd with SQE err */;
pub const IEEE_T_FDXFC: c_uint = 0x270 /* Flow control pause frames tx'd */;
pub const IEEE_T_OCTETS_OK: c_uint = 0x274 /* Octet count for frames tx'd w/o err */;
pub const RMON_R_PACKETS: c_uint = 0x284 /* RMON RX packet count */;
pub const RMON_R_BC_PKT: c_uint = 0x288 /* RMON RX broadcast pkts */;
pub const RMON_R_MC_PKT: c_uint = 0x28c /* RMON RX multicast pkts */;
pub const RMON_R_CRC_ALIGN: c_uint = 0x290 /* RMON RX pkts with CRC alignment err */;
pub const RMON_R_UNDERSIZE: c_uint = 0x294 /* RMON RX pkts < 64 bytes, good CRC */;
pub const RMON_R_OVERSIZE: c_uint = 0x298 /* RMON RX pkts > MAX_FL bytes good CRC */;
pub const RMON_R_FRAG: c_uint = 0x29c /* RMON RX pkts < 64 bytes, bad CRC */;
pub const RMON_R_JAB: c_uint = 0x2a0 /* RMON RX pkts > MAX_FL bytes, bad CRC */;
pub const RMON_R_RESVD_O: c_uint = 0x2a4 /* Reserved */;
pub const RMON_R_P64: c_uint = 0x2a8 /* RMON RX 64 byte pkts */;
pub const RMON_R_P65TO127: c_uint = 0x2ac /* RMON RX 65 to 127 byte pkts */;
pub const RMON_R_P128TO255: c_uint = 0x2b0 /* RMON RX 128 to 255 byte pkts */;
pub const RMON_R_P256TO511: c_uint = 0x2b4 /* RMON RX 256 to 511 byte pkts */;
pub const RMON_R_P512TO1023: c_uint = 0x2b8 /* RMON RX 512 to 1023 byte pkts */;
pub const RMON_R_P1024TO2047: c_uint = 0x2bc /* RMON RX 1024 to 2047 byte pkts */;
pub const RMON_R_P_GTE2048: c_uint = 0x2c0 /* RMON RX pkts > 2048 bytes */;
pub const RMON_R_OCTETS: c_uint = 0x2c4 /* RMON RX octets */;
pub const IEEE_R_DROP: c_uint = 0x2c8 /* Count frames not counted correctly */;
pub const IEEE_R_FRAME_OK: c_uint = 0x2cc /* Frames rx'd OK */;
pub const IEEE_R_CRC: c_uint = 0x2d0 /* Frames rx'd with CRC err */;
pub const IEEE_R_ALIGN: c_uint = 0x2d4 /* Frames rx'd with alignment err */;
pub const IEEE_R_MACERR: c_uint = 0x2d8 /* Receive FIFO overflow count */;
pub const IEEE_R_FDXFC: c_uint = 0x2dc /* Flow control pause frames rx'd */;
pub const IEEE_R_OCTETS_OK: c_uint = 0x2e0 /* Octet cnt for frames rx'd w/o err */;

pub const FEC_ECNTRL: c_uint = 0x000 /* Ethernet control reg */;
pub const FEC_IEVENT: c_uint = 0x004 /* Interrupt even reg */;
pub const FEC_IMASK: c_uint = 0x008 /* Interrupt mask reg */;
pub const FEC_IVEC: c_uint = 0x00c /* Interrupt vec status reg */;
pub const FEC_R_DES_ACTIVE_0: c_uint = 0x010 /* Receive descriptor reg */;

pub const FEC_X_DES_ACTIVE_0: c_uint = 0x014 /* Transmit descriptor reg */;

pub const FEC_MII_DATA: c_uint = 0x040 /* MII manage frame reg */;
pub const FEC_MII_SPEED: c_uint = 0x044 /* MII speed control reg */;
pub const FEC_R_BOUND: c_uint = 0x08c /* FIFO receive bound reg */;
pub const FEC_R_FSTART: c_uint = 0x090 /* FIFO receive start reg */;
pub const FEC_X_WMRK: c_uint = 0x0a4 /* FIFO transmit water mark */;
pub const FEC_X_FSTART: c_uint = 0x0ac /* FIFO transmit start reg */;
pub const FEC_R_CNTRL: c_uint = 0x104 /* Receive control reg */;
pub const FEC_MAX_FRM_LEN: c_uint = 0x108 /* Maximum frame length reg */;
pub const FEC_X_CNTRL: c_uint = 0x144 /* Transmit Control reg */;
pub const FEC_ADDR_LOW: c_uint = 0x3c0 /* Low 32bits MAC address */;
pub const FEC_ADDR_HIGH: c_uint = 0x3c4 /* High 16bits MAC address */;
pub const FEC_GRP_HASH_TABLE_HIGH: c_uint = 0x3c8 /* High 32bits hash table */;
pub const FEC_GRP_HASH_TABLE_LOW: c_uint = 0x3cc /* Low 32bits hash table */;
pub const FEC_R_DES_START_0: c_uint = 0x3d0 /* Receive descriptor ring */;

pub const FEC_X_DES_START_0: c_uint = 0x3d4 /* Transmit descriptor ring */;

pub const FEC_R_BUFF_SIZE_0: c_uint = 0x3d8 /* Maximum receive buff size */;

pub const FEC_FIFO_RAM: c_uint = 0x400 /* FIFO RAM buffer */;
// Not existed in real chip
// Just for pass build.
//
pub const FEC_RCMR_1: c_uint = 0xfff;
pub const FEC_RCMR_2: c_uint = 0xfff;
pub const FEC_DMA_CFG_1: c_uint = 0xfff;
pub const FEC_DMA_CFG_2: c_uint = 0xfff;
pub const FEC_TXIC0: c_uint = 0xfff;
pub const FEC_TXIC1: c_uint = 0xfff;
pub const FEC_TXIC2: c_uint = 0xfff;
pub const FEC_RXIC0: c_uint = 0xfff;
pub const FEC_RXIC1: c_uint = 0xfff;
pub const FEC_RXIC2: c_uint = 0xfff;
pub const FEC_LPI_SLEEP: c_uint = 0xfff;
pub const FEC_LPI_WAKE: c_uint = 0xfff;

//
// Define the buffer descriptor structure.
//
// Evidently, ARM SoCs have the FEC block generated in a
// little endian mode so adjust endianness accordingly.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bufdesc {
    pub /: *mut *mut __fec16 cbd_datlen; / Data length,
    pub /: *mut *mut __fec16 cbd_sc; / Control and status info,
    pub /: *mut *mut __fec32 cbd_bufaddr; / Buffer address,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bufdesc {
    pub /: *mut *mut __fec16 cbd_sc; / Control and status info,
    pub /: *mut *mut __fec16 cbd_datlen; / Data length,
    pub /: *mut *mut __fec32 cbd_bufaddr; / Buffer address,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bufdesc_ex {
    pub desc: bufdesc,
    pub cbd_esc: __fec32,
    pub cbd_prot: __fec32,
    pub cbd_bdu: __fec32,
    pub ts: __fec32,
    pub res0: [__fec16; 4],
}

// Buffer descriptor control/status used by Ethernet receive.
//

// Enhanced buffer descriptor control/status used by Ethernet receive
pub const BD_ENET_RX_VLAN: c_uint = 0x00000004;
// Buffer descriptor control/status used by Ethernet transmit.
//

// enhanced buffer descriptor control/status used by Ethernet transmit
pub const BD_ENET_TX_INT: c_uint = 0x40000000;
pub const BD_ENET_TX_TS: c_uint = 0x20000000;
pub const BD_ENET_TX_PINS: c_uint = 0x10000000;
pub const BD_ENET_TX_IINS: c_uint = 0x08000000;
// This device has up to three irqs on some platforms
pub const FEC_IRQ_NUM: c_int = 3;
// Maximum number of queues supported
// ENET with AVB IP can support up to 3 independent tx queues and rx queues.
// User can point the queue number that is less than or equal to 3.
//
pub const FEC_ENET_MAX_TX_QS: c_int = 3;
pub const FEC_ENET_MAX_RX_QS: c_int = 3;

pub const IDLE_SLOPE_MASK: c_uint = 0xffff;
pub const IDLE_SLOPE_1: c_uint = 0x200 /* BW fraction: 0.5 */;
pub const IDLE_SLOPE_2: c_uint = 0x200 /* BW fraction: 0.5 */;

// The number of Tx and Rx buffers.  These are allocated from the page
// pool.  The code may assume these are power of two, so it is best
// to keep them that size.
// We don't need to allocate pages for the transmitter.  We just use
// the skbuffer directly.
//

pub const FEC_ENET_RX_PAGES: c_int = 256;

pub const FEC_ENET_TX_FRSIZE: c_int = 2048;

pub const FEC_XSK_TX_BUDGET_MAX: c_int = 256;
pub const BD_ENET_RX_INT: c_uint = 0x00800000;

pub const BD_ENET_RX_ICE: c_uint = 0x00000020;
pub const BD_ENET_RX_PCR: c_uint = 0x00000010;

// Interrupt events/masks.

// ENET interrupt coalescing macro define

pub const FEC_VLAN_TAG_LEN: c_uint = 0x04;
pub const FEC_ETHTYPE_LEN: c_uint = 0x02;
// Controller is ENET-MAC

// Controller needs driver to swap frame

// Controller uses gasket

// Controller has GBIT support

// Controller has extend desc buffer

// Controller has hardware checksum support

// Controller has hardware vlan support

// ENET IP errata ERR006358
//
// If the ready bit in the transmit buffer descriptor (TxBD[R]) is previously
// detected as not set during a prior frame transmission, then the
// ENET_TDAR[TDAR] bit is cleared at a later time, even if additional TxBDs
// were added to the ring and the ENET_TDAR[TDAR] bit is set. This results in
// frames not being transmitted until there is a 0-to-1 transition on
// ENET_TDAR[TDAR].
//

// ENET IP hw AVB
//
// i.MX6SX ENET IP add Audio Video Bridging (AVB) feature support.
// - Two class indicators on receive with configurable priority
// - Two class indicators and line speed timer on transmit allowing
// implementation class credit based shapers externally
// - Additional DMA registers provisioned to allow managing up to 3
// independent rings
//

// There is a TDAR race condition for mutliQ when the software sets TDAR
// and the UDMA clears TDAR simultaneously or in a small window (2-4 cycles).
// This will cause the udma_tx and udma_tx_arbiter state machines to hang.
// The issue exist at i.MX6SX enet IP.
//

// ENET Block Guide/ Chapter for the iMX6SX (PELE) address one issue:
// After set ENET_ATCR[Capture], there need some time cycles before the counter
// value is capture in the register clock domain.
// The wait-time-cycles is at least 6 clock cycles of the slower clock between
// the register clock and the 1588 clock. The 1588 ts_clk is fixed to 25Mhz,
// register clock is 66Mhz, so the wait-time-cycles must be greater than 240ns
// (40ns * 6).
//

// Controller has only one MDIO bus

// Controller supports RACC register

// Controller supports interrupt coalesce

// Interrupt doesn't wake CPU from deep idle

// The MIB counters should be cleared and enabled during
// initialisation.
//

// Only i.MX25/i.MX27/i.MX28 controller supports FRBR,FRSR registers,
// those FIFO receive registers are resolved in other platforms.
//

// Some FEC hardware blocks need the MMFR cleared at setup time to avoid
// the generation of an MII event. This must be avoided in the older
// FEC blocks where it will stop MII events being generated.
//

// Some link partners do not tolerate the momentary reset of the REF_CLK
// frequency when the RNCTL register is cleared by hardware reset.
//

// i.MX6SX ENET IP supports multiple queues (3 queues), use this quirk to
// represents this ENET IP.
//

// i.MX8MQ ENET IP version add new feature to support IEEE 802.3az EEE
// standard. For the transmission, MAC supply two user registers to set
// Sleep (TS) and Wake (TW) time.
//

// i.MX8QM ENET IP version add new feature to generate delayed TXC/RXC
// as an alternative option to make sure it works well with various PHYs.
// For the implementation of delayed clock, ENET takes synchronized 250MHz
// clocks to generate 2ns delay.
//

// i.MX8MQ SoC integration mix wakeup interrupt signal into "int2" interrupt line.

// i.MX6Q adds pm_qos support

// Not all FEC hardware block MDIOs support accesses in C45 mode.
// Older blocks in the ColdFire parts do not support it.
//

// Jumbo Frame support

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bufdesc_prop {
    pub qid: c_int,
// Address of Rx and Tx buffers
    pub base: *mut bufdesc,
    pub last: *mut bufdesc,
    pub cur: *mut bufdesc,
    pub reg_desc_active: *mut void __iomem,
    pub dma: dma_addr_t,
    pub ring_size: c_ushort,
    pub dsize: c_uchar,
    pub dsize_log2: c_uchar,
}

// The following must be the last one
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fec_txbuf_type {
    FEC_TXBUF_T_SKB,
    FEC_TXBUF_T_XDP_NDO,
    FEC_TXBUF_T_XDP_TX,
    FEC_TXBUF_T_XSK_XMIT,
    FEC_TXBUF_T_XSK_TX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fec_tx_buffer {
    pub buf_p: *mut c_void,
    pub type: fec_txbuf_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fec_enet_priv_tx_q {
    pub bd: bufdesc_prop,
    pub tx_bounce: [*mut c_uchar; TX_RING_SIZE],
    pub tx_buf: [fec_tx_buffer; TX_RING_SIZE],
    pub xsk_pool: *mut xsk_buff_pool,
    pub tx_stop_threshold: c_ushort,
    pub tx_wake_threshold: c_ushort,
    pub dirty_tx: *mut bufdesc,
    pub tso_hdrs: *mut c_char,
    pub tso_hdrs_dma: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union fec_rx_buffer {
    pub buf_p: *mut c_void,
    pub page: *mut page,
    pub xdp: *mut xdp_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fec_enet_priv_rx_q {
    pub bd: bufdesc_prop,
    pub rx_buf: [fec_rx_buffer; RX_RING_SIZE],
    pub xsk_pool: *mut xsk_buff_pool,
// page_pool
    pub page_pool: *mut page_pool,
    pub xdp_rxq: xdp_rxq_info,
    pub stats: [u32; XDP_STATS_TOTAL],
// rx queue number, in the range 0-7
    pub id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fec_stop_mode_gpr {
    pub gpr: *mut regmap,
    pub reg: u8,
    pub bit: u8,
}

// The FEC buffer descriptors track the ring buffers.  The rx_bd_base and
// tx_bd_base always point to the base of the buffer descriptors.  The
// cur_rx and cur_tx point to the currently available buffer.
// The dirty_tx tracks the current buffer that is being sent by the
// controller.  The cur_tx and dirty_tx are equal under both completely
// empty and completely full conditions.  The empty/ready indicator in
// the buffer descriptor determines the actual condition.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fec_enet_private {
// Hardware registers of the FEC device
    pub hwp: *mut void __iomem,
    pub netdev: *mut net_device,
    pub clk_ipg: *mut clk,
    pub clk_ahb: *mut clk,
    pub clk_ref: *mut clk,
    pub clk_enet_out: *mut clk,
    pub clk_ptp: *mut clk,
    pub clk_2x_txclk: *mut clk,
    pub ptp_clk_on: bool,
    pub ptp_clk_mutex: mutex,
    pub num_tx_queues: c_uint,
    pub num_rx_queues: c_uint,
    pub tx_queue: [*mut fec_enet_priv_tx_q; FEC_ENET_MAX_TX_QS],
    pub rx_queue: [*mut fec_enet_priv_rx_q; FEC_ENET_MAX_RX_QS],
    pub total_tx_ring_size: c_uint,
    pub total_rx_ring_size: c_uint,
    pub max_buf_size: c_uint,
    pub pagepool_order: c_uint,
    pub rx_frame_size: c_uint,
    pub pdev: *mut platform_device,
    pub dev_id: c_int,
// Phylib and MDIO interface
    pub mii_bus: *mut mii_bus,
    pub phy_speed: c_uint,
    pub phy_interface: phy_interface_t,
    pub phy_node: *mut device_node,
    pub rgmii_txc_dly: bool,
    pub rgmii_rxc_dly: bool,
    pub rpm_active: bool,
    pub link: c_int,
    pub full_duplex: c_int,
    pub speed: c_int,
    pub irq: [c_int; FEC_IRQ_NUM],
    pub bufdesc_ex: bool,
    pub pause_flag: c_int,
    pub wol_flag: c_int,
    pub wake_irq: c_int,
    pub quirks: u32,
    pub napi: napi_struct,
    pub csum_flags: c_int,
    pub tx_timeout_work: work_struct,
    pub ptp_clock: *mut ptp_clock,
    pub ptp_caps: ptp_clock_info,
    pub tmreg_lock: spinlock_t,
    pub cc: cyclecounter,
    pub tc: timecounter,
    pub cycle_speed: u32,
    pub hwts_rx_en: c_int,
    pub hwts_tx_en: c_int,
    pub time_keep: delayed_work,
    pub reg_phy: *mut regulator,
    pub stop_gpr: fec_stop_mode_gpr,
    pub pm_qos_req: pm_qos_request,
    pub tx_align: c_uint,
    pub rx_shift: c_uint,
// hw interrupt coalesce
    pub rx_pkts_itr: c_uint,
    pub rx_time_itr: c_uint,
    pub tx_pkts_itr: c_uint,
    pub tx_time_itr: c_uint,
    pub itr_clk_rate: c_uint,
    pub clk_ref_rate: c_uint,
// ptp clock period in ns
    pub ptp_inc: c_uint,
// pps
    pub pps_channel: c_int,
    pub reload_period: c_uint,
    pub pps_enable: c_int,
    pub next_counter: c_uint,
    pub perout_enable: bool,
    pub perout_timer: hrtimer,
    pub perout_stime: u64,
    pub ipc_handle: *mut imx_sc_ipc,
// XDP BPF Program
    pub xdp_prog: *mut bpf_prog,
    pub pps_enable: c_int,
    pub ns_phc: u64 ns_sys,,
    pub at_corr: u32,
    pub at_inc_corr: u8,
    pub ptp_saved_state: },
    pub ethtool_stats: [u64; ],
}

extern "C" {
    pub fn fec_ptp_init(pdev: *mut platform_device, irq_idx: c_int);
}
extern "C" {
    pub fn fec_ptp_restore_state(fep: *mut fec_enet_private);
}
extern "C" {
    pub fn fec_ptp_save_state(fep: *mut fec_enet_private);
}
extern "C" {
    pub fn fec_ptp_stop(pdev: *mut platform_device);
}
extern "C" {
    pub fn fec_ptp_start_cyclecounter(ndev: *mut net_device);
}
extern "C" {
    pub fn fec_ptp_get(ndev: *mut net_device, config: *mut kernel_hwtstamp_config);
}
//
