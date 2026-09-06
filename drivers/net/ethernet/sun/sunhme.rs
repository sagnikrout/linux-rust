//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sun/sunhme.h
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
// $Id: sunhme.h,v 1.33 2001/08/03 06:23:04 davem Exp $
// sunhme.h: Definitions for Sparc HME/BigMac 10/100baseT ethernet driver.
// Also known as the "Happy Meal".
//
// Copyright (C) 1996, 1999 David S. Miller (davem@redhat.com)
//

// Happy Meal global registers.
pub const GREG_SWRESET: c_uint = 0x000UL	/* Software Reset  */;
pub const GREG_CFG: c_uint = 0x004UL	/* Config Register */;
pub const GREG_STAT: c_uint = 0x100UL	/* Status          */;
pub const GREG_IMASK: c_uint = 0x104UL	/* Interrupt Mask  */;
pub const GREG_REG_SIZE: c_uint = 0x108UL;
// Global reset register.
pub const GREG_RESET_ETX: c_uint = 0x01;
pub const GREG_RESET_ERX: c_uint = 0x02;
pub const GREG_RESET_ALL: c_uint = 0x03;
// Global config register.
pub const GREG_CFG_BURSTMSK: c_uint = 0x03;
pub const GREG_CFG_BURST16: c_uint = 0x00;
pub const GREG_CFG_BURST32: c_uint = 0x01;
pub const GREG_CFG_BURST64: c_uint = 0x02;
pub const GREG_CFG_64BIT: c_uint = 0x04;
pub const GREG_CFG_PARITY: c_uint = 0x08;
pub const GREG_CFG_RESV: c_uint = 0x10;
// Global status register.
pub const GREG_STAT_GOTFRAME: c_uint = 0x00000001 /* Received a frame                         */;
pub const GREG_STAT_RCNTEXP: c_uint = 0x00000002 /* Receive frame counter expired            */;
pub const GREG_STAT_ACNTEXP: c_uint = 0x00000004 /* Align-error counter expired              */;
pub const GREG_STAT_CCNTEXP: c_uint = 0x00000008 /* CRC-error counter expired                */;
pub const GREG_STAT_LCNTEXP: c_uint = 0x00000010 /* Length-error counter expired             */;
pub const GREG_STAT_RFIFOVF: c_uint = 0x00000020 /* Receive FIFO overflow                    */;
pub const GREG_STAT_CVCNTEXP: c_uint = 0x00000040 /* Code-violation counter expired           */;
pub const GREG_STAT_STSTERR: c_uint = 0x00000080 /* Test error in XIF for SQE                */;
pub const GREG_STAT_SENTFRAME: c_uint = 0x00000100 /* Transmitted a frame                      */;
pub const GREG_STAT_TFIFO_UND: c_uint = 0x00000200 /* Transmit FIFO underrun                   */;
pub const GREG_STAT_MAXPKTERR: c_uint = 0x00000400 /* Max-packet size error                    */;
pub const GREG_STAT_NCNTEXP: c_uint = 0x00000800 /* Normal-collision counter expired         */;
pub const GREG_STAT_ECNTEXP: c_uint = 0x00001000 /* Excess-collision counter expired         */;
pub const GREG_STAT_LCCNTEXP: c_uint = 0x00002000 /* Late-collision counter expired           */;
pub const GREG_STAT_FCNTEXP: c_uint = 0x00004000 /* First-collision counter expired          */;
pub const GREG_STAT_DTIMEXP: c_uint = 0x00008000 /* Defer-timer expired                      */;
pub const GREG_STAT_RXTOHOST: c_uint = 0x00010000 /* Moved from receive-FIFO to host memory   */;
pub const GREG_STAT_NORXD: c_uint = 0x00020000 /* No more receive descriptors              */;
pub const GREG_STAT_RXERR: c_uint = 0x00040000 /* Error during receive dma                 */;
pub const GREG_STAT_RXLATERR: c_uint = 0x00080000 /* Late error during receive dma            */;
pub const GREG_STAT_RXPERR: c_uint = 0x00100000 /* Parity error during receive dma          */;
pub const GREG_STAT_RXTERR: c_uint = 0x00200000 /* Tag error during receive dma             */;
pub const GREG_STAT_EOPERR: c_uint = 0x00400000 /* Transmit descriptor did not have EOP set */;
pub const GREG_STAT_MIFIRQ: c_uint = 0x00800000 /* MIF is signaling an interrupt condition  */;
pub const GREG_STAT_HOSTTOTX: c_uint = 0x01000000 /* Moved from host memory to transmit-FIFO  */;
pub const GREG_STAT_TXALL: c_uint = 0x02000000 /* Transmitted all packets in the tx-fifo   */;
pub const GREG_STAT_TXEACK: c_uint = 0x04000000 /* Error during transmit dma                */;
pub const GREG_STAT_TXLERR: c_uint = 0x08000000 /* Late error during transmit dma           */;
pub const GREG_STAT_TXPERR: c_uint = 0x10000000 /* Parity error during transmit dma         */;
pub const GREG_STAT_TXTERR: c_uint = 0x20000000 /* Tag error during transmit dma            */;
pub const GREG_STAT_SLVERR: c_uint = 0x40000000 /* PIO access got an error                  */;
pub const GREG_STAT_SLVPERR: c_uint = 0x80000000 /* PIO access got a parity error            */;
// All interesting error conditions.
pub const GREG_STAT_ERRORS: c_uint = 0xfc7efefc;
// Global interrupt mask register.
pub const GREG_IMASK_GOTFRAME: c_uint = 0x00000001 /* Received a frame                         */;
pub const GREG_IMASK_RCNTEXP: c_uint = 0x00000002 /* Receive frame counter expired            */;
pub const GREG_IMASK_ACNTEXP: c_uint = 0x00000004 /* Align-error counter expired              */;
pub const GREG_IMASK_CCNTEXP: c_uint = 0x00000008 /* CRC-error counter expired                */;
pub const GREG_IMASK_LCNTEXP: c_uint = 0x00000010 /* Length-error counter expired             */;
pub const GREG_IMASK_RFIFOVF: c_uint = 0x00000020 /* Receive FIFO overflow                    */;
pub const GREG_IMASK_CVCNTEXP: c_uint = 0x00000040 /* Code-violation counter expired           */;
pub const GREG_IMASK_STSTERR: c_uint = 0x00000080 /* Test error in XIF for SQE                */;
pub const GREG_IMASK_SENTFRAME: c_uint = 0x00000100 /* Transmitted a frame                      */;
pub const GREG_IMASK_TFIFO_UND: c_uint = 0x00000200 /* Transmit FIFO underrun                   */;
pub const GREG_IMASK_MAXPKTERR: c_uint = 0x00000400 /* Max-packet size error                    */;
pub const GREG_IMASK_NCNTEXP: c_uint = 0x00000800 /* Normal-collision counter expired         */;
pub const GREG_IMASK_ECNTEXP: c_uint = 0x00001000 /* Excess-collision counter expired         */;
pub const GREG_IMASK_LCCNTEXP: c_uint = 0x00002000 /* Late-collision counter expired           */;
pub const GREG_IMASK_FCNTEXP: c_uint = 0x00004000 /* First-collision counter expired          */;
pub const GREG_IMASK_DTIMEXP: c_uint = 0x00008000 /* Defer-timer expired                      */;
pub const GREG_IMASK_RXTOHOST: c_uint = 0x00010000 /* Moved from receive-FIFO to host memory   */;
pub const GREG_IMASK_NORXD: c_uint = 0x00020000 /* No more receive descriptors              */;
pub const GREG_IMASK_RXERR: c_uint = 0x00040000 /* Error during receive dma                 */;
pub const GREG_IMASK_RXLATERR: c_uint = 0x00080000 /* Late error during receive dma            */;
pub const GREG_IMASK_RXPERR: c_uint = 0x00100000 /* Parity error during receive dma          */;
pub const GREG_IMASK_RXTERR: c_uint = 0x00200000 /* Tag error during receive dma             */;
pub const GREG_IMASK_EOPERR: c_uint = 0x00400000 /* Transmit descriptor did not have EOP set */;
pub const GREG_IMASK_MIFIRQ: c_uint = 0x00800000 /* MIF is signaling an interrupt condition  */;
pub const GREG_IMASK_HOSTTOTX: c_uint = 0x01000000 /* Moved from host memory to transmit-FIFO  */;
pub const GREG_IMASK_TXALL: c_uint = 0x02000000 /* Transmitted all packets in the tx-fifo   */;
pub const GREG_IMASK_TXEACK: c_uint = 0x04000000 /* Error during transmit dma                */;
pub const GREG_IMASK_TXLERR: c_uint = 0x08000000 /* Late error during transmit dma           */;
pub const GREG_IMASK_TXPERR: c_uint = 0x10000000 /* Parity error during transmit dma         */;
pub const GREG_IMASK_TXTERR: c_uint = 0x20000000 /* Tag error during transmit dma            */;
pub const GREG_IMASK_SLVERR: c_uint = 0x40000000 /* PIO access got an error                  */;
pub const GREG_IMASK_SLVPERR: c_uint = 0x80000000 /* PIO access got a parity error            */;
// Happy Meal external transmitter registers.
pub const ETX_PENDING: c_uint = 0x00UL	/* Transmit pending/wakeup register */;
pub const ETX_CFG: c_uint = 0x04UL	/* Transmit config register         */;
pub const ETX_RING: c_uint = 0x08UL	/* Transmit ring pointer            */;
pub const ETX_BBASE: c_uint = 0x0cUL	/* Transmit buffer base             */;
pub const ETX_BDISP: c_uint = 0x10UL	/* Transmit buffer displacement     */;
pub const ETX_FIFOWPTR: c_uint = 0x14UL	/* FIFO write ptr                   */;
pub const ETX_FIFOSWPTR: c_uint = 0x18UL	/* FIFO write ptr (shadow register) */;
pub const ETX_FIFORPTR: c_uint = 0x1cUL	/* FIFO read ptr                    */;
pub const ETX_FIFOSRPTR: c_uint = 0x20UL	/* FIFO read ptr (shadow register)  */;
pub const ETX_FIFOPCNT: c_uint = 0x24UL	/* FIFO packet counter              */;
pub const ETX_SMACHINE: c_uint = 0x28UL	/* Transmitter state machine        */;
pub const ETX_RSIZE: c_uint = 0x2cUL	/* Ring descriptor size             */;
pub const ETX_BPTR: c_uint = 0x30UL	/* Transmit data buffer ptr         */;
pub const ETX_REG_SIZE: c_uint = 0x34UL;
// ETX transmit pending register.
pub const ETX_TP_DMAWAKEUP: c_uint = 0x00000001 /* Restart transmit dma             */;
// ETX config register.
pub const ETX_CFG_DMAENABLE: c_uint = 0x00000001 /* Enable transmit dma              */;
pub const ETX_CFG_FIFOTHRESH: c_uint = 0x000003fe /* Transmit FIFO threshold          */;
pub const ETX_CFG_IRQDAFTER: c_uint = 0x00000400 /* Interrupt after TX-FIFO drained  */;
pub const ETX_CFG_IRQDBEFORE: c_uint = 0x00000000 /* Interrupt before TX-FIFO drained */;
pub const ETX_RSIZE_SHIFT: c_int = 4;
// Happy Meal external receiver registers.
pub const ERX_CFG: c_uint = 0x00UL	/* Receiver config register         */;
pub const ERX_RING: c_uint = 0x04UL	/* Receiver ring ptr                */;
pub const ERX_BPTR: c_uint = 0x08UL	/* Receiver buffer ptr              */;
pub const ERX_FIFOWPTR: c_uint = 0x0cUL	/* FIFO write ptr                   */;
pub const ERX_FIFOSWPTR: c_uint = 0x10UL	/* FIFO write ptr (shadow register) */;
pub const ERX_FIFORPTR: c_uint = 0x14UL	/* FIFO read ptr                    */;
pub const ERX_FIFOSRPTR: c_uint = 0x18UL	/* FIFO read ptr (shadow register)  */;
pub const ERX_SMACHINE: c_uint = 0x1cUL	/* Receiver state machine           */;
pub const ERX_REG_SIZE: c_uint = 0x20UL;
// ERX config register.
pub const ERX_CFG_DMAENABLE: c_uint = 0x00000001 /* Enable receive DMA        */;
pub const ERX_CFG_RESV1: c_uint = 0x00000006 /* Unused...                 */;
pub const ERX_CFG_BYTEOFFSET: c_uint = 0x00000038 /* Receive first byte offset */;
pub const ERX_CFG_RESV2: c_uint = 0x000001c0 /* Unused...                 */;
pub const ERX_CFG_SIZE32: c_uint = 0x00000000 /* Receive ring size == 32   */;
pub const ERX_CFG_SIZE64: c_uint = 0x00000200 /* Receive ring size == 64   */;
pub const ERX_CFG_SIZE128: c_uint = 0x00000400 /* Receive ring size == 128  */;
pub const ERX_CFG_SIZE256: c_uint = 0x00000600 /* Receive ring size == 256  */;
pub const ERX_CFG_RESV3: c_uint = 0x0000f800 /* Unused...                 */;
pub const ERX_CFG_CSUMSTART: c_uint = 0x007f0000 /* Offset of checksum start,;
// in halfwords.
// I'd like a Big Mac, small fries, small coke, and SparcLinux please.
pub const BMAC_XIFCFG: c_uint = 0x0000UL	/* XIF config register                */;
// 0x4-->0x204, reserved
pub const BMAC_TXSWRESET: c_uint = 0x208UL	/* Transmitter software reset         */;
pub const BMAC_TXCFG: c_uint = 0x20cUL	/* Transmitter config register        */;
pub const BMAC_IGAP1: c_uint = 0x210UL	/* Inter-packet gap 1                 */;
pub const BMAC_IGAP2: c_uint = 0x214UL	/* Inter-packet gap 2                 */;
pub const BMAC_ALIMIT: c_uint = 0x218UL	/* Transmit attempt limit             */;
pub const BMAC_STIME: c_uint = 0x21cUL	/* Transmit slot time                 */;
pub const BMAC_PLEN: c_uint = 0x220UL	/* Size of transmit preamble          */;
pub const BMAC_PPAT: c_uint = 0x224UL	/* Pattern for transmit preamble      */;
pub const BMAC_TXSDELIM: c_uint = 0x228UL	/* Transmit delimiter                 */;
pub const BMAC_JSIZE: c_uint = 0x22cUL	/* Jam size                           */;
pub const BMAC_TXMAX: c_uint = 0x230UL	/* Transmit max pkt size              */;
pub const BMAC_TXMIN: c_uint = 0x234UL	/* Transmit min pkt size              */;
pub const BMAC_PATTEMPT: c_uint = 0x238UL	/* Count of transmit peak attempts    */;
pub const BMAC_DTCTR: c_uint = 0x23cUL	/* Transmit defer timer               */;
pub const BMAC_NCCTR: c_uint = 0x240UL	/* Transmit normal-collision counter  */;
pub const BMAC_FCCTR: c_uint = 0x244UL	/* Transmit first-collision counter   */;
pub const BMAC_EXCTR: c_uint = 0x248UL	/* Transmit excess-collision counter  */;
pub const BMAC_LTCTR: c_uint = 0x24cUL	/* Transmit late-collision counter    */;
pub const BMAC_RSEED: c_uint = 0x250UL	/* Transmit random number seed        */;
pub const BMAC_TXSMACHINE: c_uint = 0x254UL	/* Transmit state machine             */;
// 0x258-->0x304, reserved
pub const BMAC_RXSWRESET: c_uint = 0x308UL	/* Receiver software reset            */;
pub const BMAC_RXCFG: c_uint = 0x30cUL	/* Receiver config register           */;
pub const BMAC_RXMAX: c_uint = 0x310UL	/* Receive max pkt size               */;
pub const BMAC_RXMIN: c_uint = 0x314UL	/* Receive min pkt size               */;
pub const BMAC_MACADDR2: c_uint = 0x318UL	/* Ether address register 2           */;
pub const BMAC_MACADDR1: c_uint = 0x31cUL	/* Ether address register 1           */;
pub const BMAC_MACADDR0: c_uint = 0x320UL	/* Ether address register 0           */;
pub const BMAC_FRCTR: c_uint = 0x324UL	/* Receive frame receive counter      */;
pub const BMAC_GLECTR: c_uint = 0x328UL	/* Receive giant-length error counter */;
pub const BMAC_UNALECTR: c_uint = 0x32cUL	/* Receive unaligned error counter    */;
pub const BMAC_RCRCECTR: c_uint = 0x330UL	/* Receive CRC error counter          */;
pub const BMAC_RXSMACHINE: c_uint = 0x334UL	/* Receiver state machine             */;
pub const BMAC_RXCVALID: c_uint = 0x338UL	/* Receiver code violation            */;
// 0x33c, reserved
pub const BMAC_HTABLE3: c_uint = 0x340UL	/* Hash table 3                       */;
pub const BMAC_HTABLE2: c_uint = 0x344UL	/* Hash table 2                       */;
pub const BMAC_HTABLE1: c_uint = 0x348UL	/* Hash table 1                       */;
pub const BMAC_HTABLE0: c_uint = 0x34cUL	/* Hash table 0                       */;
pub const BMAC_AFILTER2: c_uint = 0x350UL	/* Address filter 2                   */;
pub const BMAC_AFILTER1: c_uint = 0x354UL	/* Address filter 1                   */;
pub const BMAC_AFILTER0: c_uint = 0x358UL	/* Address filter 0                   */;
pub const BMAC_AFMASK: c_uint = 0x35cUL	/* Address filter mask                */;
pub const BMAC_REG_SIZE: c_uint = 0x360UL;
// BigMac XIF config register.
pub const BIGMAC_XCFG_ODENABLE: c_uint = 0x00000001 /* Output driver enable         */;
pub const BIGMAC_XCFG_XLBACK: c_uint = 0x00000002 /* Loopback-mode XIF enable     */;
pub const BIGMAC_XCFG_MLBACK: c_uint = 0x00000004 /* Loopback-mode MII enable     */;
pub const BIGMAC_XCFG_MIIDISAB: c_uint = 0x00000008 /* MII receive buffer disable   */;
pub const BIGMAC_XCFG_SQENABLE: c_uint = 0x00000010 /* SQE test enable              */;
pub const BIGMAC_XCFG_SQETWIN: c_uint = 0x000003e0 /* SQE time window              */;
pub const BIGMAC_XCFG_LANCE: c_uint = 0x00000010 /* Lance mode enable            */;
pub const BIGMAC_XCFG_LIPG0: c_uint = 0x000003e0 /* Lance mode IPG0              */;
// BigMac transmit config register.
pub const BIGMAC_TXCFG_ENABLE: c_uint = 0x00000001 /* Enable the transmitter       */;
pub const BIGMAC_TXCFG_SMODE: c_uint = 0x00000020 /* Enable slow transmit mode    */;
pub const BIGMAC_TXCFG_CIGN: c_uint = 0x00000040 /* Ignore transmit collisions   */;
pub const BIGMAC_TXCFG_FCSOFF: c_uint = 0x00000080 /* Do not emit FCS              */;
pub const BIGMAC_TXCFG_DBACKOFF: c_uint = 0x00000100 /* Disable backoff              */;
pub const BIGMAC_TXCFG_FULLDPLX: c_uint = 0x00000200 /* Enable full-duplex           */;
pub const BIGMAC_TXCFG_DGIVEUP: c_uint = 0x00000400 /* Don't give up on transmits   */;
// BigMac receive config register.
pub const BIGMAC_RXCFG_ENABLE: c_uint = 0x00000001 /* Enable the receiver             */;
pub const BIGMAC_RXCFG_PSTRIP: c_uint = 0x00000020 /* Pad byte strip enable           */;
pub const BIGMAC_RXCFG_PMISC: c_uint = 0x00000040 /* Enable promiscuous mode          */;
pub const BIGMAC_RXCFG_DERR: c_uint = 0x00000080 /* Disable error checking          */;
pub const BIGMAC_RXCFG_DCRCS: c_uint = 0x00000100 /* Disable CRC stripping           */;
pub const BIGMAC_RXCFG_REJME: c_uint = 0x00000200 /* Reject packets addressed to me  */;
pub const BIGMAC_RXCFG_PGRP: c_uint = 0x00000400 /* Enable promisc group mode       */;
pub const BIGMAC_RXCFG_HENABLE: c_uint = 0x00000800 /* Enable the hash filter          */;
pub const BIGMAC_RXCFG_AENABLE: c_uint = 0x00001000 /* Enable the address filter       */;
// These are the "Management Interface" (ie. MIF) registers of the transceiver.
pub const TCVR_BBCLOCK: c_uint = 0x00UL	/* Bit bang clock register          */;
pub const TCVR_BBDATA: c_uint = 0x04UL	/* Bit bang data register           */;
pub const TCVR_BBOENAB: c_uint = 0x08UL	/* Bit bang output enable           */;
pub const TCVR_FRAME: c_uint = 0x0cUL	/* Frame control/data register      */;
pub const TCVR_CFG: c_uint = 0x10UL	/* MIF config register              */;
pub const TCVR_IMASK: c_uint = 0x14UL	/* MIF interrupt mask               */;
pub const TCVR_STATUS: c_uint = 0x18UL	/* MIF status                       */;
pub const TCVR_SMACHINE: c_uint = 0x1cUL	/* MIF state machine                */;
pub const TCVR_REG_SIZE: c_uint = 0x20UL;
// Frame commands.
pub const FRAME_WRITE: c_uint = 0x50020000;
pub const FRAME_READ: c_uint = 0x60020000;
// Transceiver config register
pub const TCV_CFG_PSELECT: c_uint = 0x00000001 /* Select PHY                      */;
pub const TCV_CFG_PENABLE: c_uint = 0x00000002 /* Enable MIF polling              */;
pub const TCV_CFG_BENABLE: c_uint = 0x00000004 /* Enable the "bit banger" oh baby */;
pub const TCV_CFG_PREGADDR: c_uint = 0x000000f8 /* Address of poll register        */;
pub const TCV_CFG_MDIO0: c_uint = 0x00000100 /* MDIO zero, data/attached        */;
pub const TCV_CFG_MDIO1: c_uint = 0x00000200 /* MDIO one,  data/attached        */;
pub const TCV_CFG_PDADDR: c_uint = 0x00007c00 /* Device PHY address polling      */;
// Here are some PHY addresses.

// Transceiver status register
pub const TCV_STAT_BASIC: c_uint = 0xffff0000 /* The "basic" part                */;
pub const TCV_STAT_NORMAL: c_uint = 0x0000ffff /* The "non-basic" part            */;
// Inside the Happy Meal transceiver is the physical layer, they use an
// implementations for National Semiconductor, part number DP83840VCE.
// You can retrieve the data sheets and programming docs for this beast
// from http://www.national.com
//
// The DP83840 is capable of both 10 and 100Mbps ethernet, in both
// half and full duplex mode.  It also supports auto negotiation.
//
// But.... THIS THING IS A PAIN IN THE ASS TO PROGRAM!
// Debugging eeprom burnt code is more fun than programming this chip!
//
// Generic MII registers defined in linux/mii.h, these below
// are DP83840 specific.
//
pub const DP83840_CSCONFIG: c_uint = 0x17        /* CS configuration            */;
// The Carrier Sense config register.
pub const CSCONFIG_RESV1: c_uint = 0x0001  /* Unused...                   */;
pub const CSCONFIG_LED4: c_uint = 0x0002  /* Pin for full-dplx LED4      */;
pub const CSCONFIG_LED1: c_uint = 0x0004  /* Pin for conn-status LED1    */;
pub const CSCONFIG_RESV2: c_uint = 0x0008  /* Unused...                   */;
pub const CSCONFIG_TCVDISAB: c_uint = 0x0010  /* Turns off the transceiver   */;
pub const CSCONFIG_DFBYPASS: c_uint = 0x0020  /* Bypass disconnect function  */;
pub const CSCONFIG_GLFORCE: c_uint = 0x0040  /* Good link force for 100mbps */;
pub const CSCONFIG_CLKTRISTATE: c_uint = 0x0080  /* Tristate 25m clock          */;
pub const CSCONFIG_RESV3: c_uint = 0x0700  /* Unused...                   */;
pub const CSCONFIG_ENCODE: c_uint = 0x0800  /* 1=MLT-3, 0=binary           */;
pub const CSCONFIG_RENABLE: c_uint = 0x1000  /* Repeater mode enable        */;
pub const CSCONFIG_TCDISABLE: c_uint = 0x2000  /* Disable timeout counter     */;
pub const CSCONFIG_RESV4: c_uint = 0x4000  /* Unused...                   */;
pub const CSCONFIG_NDISABLE: c_uint = 0x8000  /* Disable NRZI                */;
// Happy Meal descriptor rings and such.
// All descriptor rings must be aligned on a 2K boundary.
// All receive buffers must be 64 byte aligned.
// Always write the address first before setting the ownership
// bits to avoid races with the hardware scanning the ring.
//
pub type hme32 = u32 ;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct happy_meal_rxd {
    pub rx_flags: hme32,
    pub rx_addr: hme32,
}

pub const RXFLAG_OWN: c_uint = 0x80000000 /* 1 = hardware, 0 = software */;
pub const RXFLAG_OVERFLOW: c_uint = 0x40000000 /* 1 = buffer overflow        */;
pub const RXFLAG_SIZE: c_uint = 0x3fff0000 /* Size of the buffer         */;
pub const RXFLAG_CSUM: c_uint = 0x0000ffff /* HW computed checksum       */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct happy_meal_txd {
    pub tx_flags: hme32,
    pub tx_addr: hme32,
}

pub const TXFLAG_OWN: c_uint = 0x80000000 /* 1 = hardware, 0 = software */;
pub const TXFLAG_SOP: c_uint = 0x40000000 /* 1 = start of packet        */;
pub const TXFLAG_EOP: c_uint = 0x20000000 /* 1 = end of packet          */;
pub const TXFLAG_CSENABLE: c_uint = 0x10000000 /* 1 = enable hw-checksums    */;
pub const TXFLAG_CSLOCATION: c_uint = 0x0ff00000 /* Where to stick the csum    */;
pub const TXFLAG_CSBUFBEGIN: c_uint = 0x000fc000 /* Where to begin checksum    */;
pub const TXFLAG_SIZE: c_uint = 0x00003fff /* Size of the packet         */;

pub const TX_RING_MAXSIZE: c_int = 256;
pub const RX_RING_MAXSIZE: c_int = 256;
// We use a 14 byte offset for checksum computation.

pub const RX_OFFSET: c_int = 2;

pub const RX_COPY_THRESHOLD: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hmeal_init_block {
    pub happy_meal_rxd: [happy_meal_rxd; RX_RING_MAXSIZE],
    pub happy_meal_txd: [happy_meal_txd; TX_RING_MAXSIZE],
}

// Now software state stuff.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum happy_transceiver {
    external = 0,
    internal = 1,
    none     = 2,
}

// Timer state engine.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum happy_timer_state {
    arbwait  = 0,  /* Waiting for auto negotiation to complete.          */
    lupwait  = 1,  /* Auto-neg complete, awaiting link-up status.        */
    ltrywait = 2,  /* Forcing try of all modes, from fastest to slowest. */
    asleep   = 3,  /* Time inactive.                                     */
}

// Happy happy, joy joy!
#[repr(C)]
#[derive(Copy, Clone)]
pub struct happy_meal {
    pub /: *mut *mut *mut void __iomem gregs; / Happy meal global registers,
    pub /: *mut *mut *mut hmeal_init_block happy_block; / RX and TX descriptors (CPU addr),

    pub ): *mut *mut u32 (read_desc32)(hme32,
    pub u32): *mut *mut *mut void (write_txd)(struct happy_meal_txd , u32,,
    pub u32): *mut *mut *mut void (write_rxd)(struct happy_meal_rxd , u32,,

// This is either an platform_device or a pci_dev.
    pub happy_dev: *mut c_void,
    pub dma_dev: *mut device,
    pub happy_lock: spinlock_t,
    pub rx_skbs: [*mut sk_buff; RX_RING_SIZE],
    pub tx_skbs: [*mut sk_buff; TX_RING_SIZE],
    pub tx_old: int rx_new, tx_new, rx_old,,

    pub ): *mut *mut u32 (read32)(void __iomem,
    pub u32): *mut *mut *mut void (write32)(void __iomem ,,

    pub /: *mut *mut *mut void __iomem etxregs; / External transmitter regs,
    pub /: *mut *mut *mut void __iomem erxregs; / External receiver regs,
    pub /: *mut *mut *mut void __iomem bigmacregs; / BIGMAC core regs,
    pub /: *mut *mut *mut void __iomem tcvregs; / MIF transceiver regs,
    pub /: *mut *mut dma_addr_t hblock_dvma; / DVMA visible address happy block,
    pub /: *mut *mut unsigned int happy_flags; / Driver state flags,
    pub irq: c_int,
    pub /: *mut *mut happy_transceiver tcvr_type; / Kind of transceiver in use,
    pub /: *mut *mut unsigned int happy_bursts; / Get your mind out of the gutter,
    pub /: *mut *mut unsigned int paddr; / PHY address for transceiver,
    pub /: *mut *mut unsigned short hm_revision; / Happy meal revision,
    pub /: *mut *mut unsigned short sw_bmcr; / SW copy of BMCR,
    pub /: *mut *mut unsigned short sw_bmsr; / SW copy of BMSR,
    pub /: *mut *mut unsigned short sw_physid1; / SW copy of PHYSID1,
    pub /: *mut *mut unsigned short sw_physid2; / SW copy of PHYSID2,
    pub /: *mut *mut unsigned short sw_advertise; / SW copy of ADVERTISE,
    pub /: *mut *mut unsigned short sw_lpa; / SW copy of LPA,
    pub /: *mut *mut unsigned short sw_expansion; / SW copy of EXPANSION,
    pub /: *mut *mut unsigned short sw_csconfig; / SW copy of CSCONFIG,
    pub /: *mut *mut unsigned int auto_speed; / Auto-nego link speed,
    pub /: *mut *mut unsigned int forced_speed; / Force mode link speed,
    pub /: *mut *mut unsigned int poll_data; / MIF poll data,
    pub /: *mut *mut unsigned int poll_flag; / MIF poll flag,
    pub /: *mut *mut unsigned int linkcheck; / Have we checked the link yet?,
    pub /: *mut *mut unsigned int lnkup; / Is the link up as far as we know?,
    pub /: *mut *mut unsigned int lnkdown; / Trying to force the link down?,
    pub /: *mut *mut unsigned int lnkcnt; / Counter for link-up attempts.,
    pub /: *mut *mut timer_list happy_timer; / To watch the link when coming up.,
    pub /: *mut *mut happy_timer_state timer_state; / State of the auto-neg timer.,
    pub /: *mut *mut unsigned int timer_ticks; / Number of clicks at each state.,
    pub /: *mut *mut *mut net_device dev; / Backpointer,
    pub /: *mut *mut *mut quattro qfe_parent; / For Quattro cards,
    pub /: *mut *mut int qfe_ent; / Which instance on quattro,
}

// Here are the happy flags.
pub const HFLAG_FENABLE: c_uint = 0x00000002      /* The MII frame is enabled          */;
pub const HFLAG_LANCE: c_uint = 0x00000004      /* We are using lance-mode           */;
pub const HFLAG_RXENABLE: c_uint = 0x00000008      /* Receiver is enabled               */;
pub const HFLAG_AUTO: c_uint = 0x00000010      /* Using auto-negotiation, 0 = force */;
pub const HFLAG_FULL: c_uint = 0x00000020      /* Full duplex enable                */;
pub const HFLAG_MACFULL: c_uint = 0x00000040      /* Using full duplex in the MAC      */;
pub const HFLAG_RXCV: c_uint = 0x00000100      /* XXX RXCV ENABLE                   */;
pub const HFLAG_INIT: c_uint = 0x00000200      /* Init called at least once         */;
pub const HFLAG_LINKUP: c_uint = 0x00000400      /* 1 = Link is up                    */;
pub const HFLAG_PCI: c_uint = 0x00000800      /* PCI based Happy Meal              */;
pub const HFLAG_QUATTRO: c_uint = 0x00001000      /* On QFE/Quattro card	       */;

// Support for QFE/Quattro cards.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct quattro {
    pub happy_meals: [*mut net_device; 4],
// This is either a sbus_dev or a pci_dev.
    pub quattro_dev: *mut c_void,
    pub next: *mut quattro,
// PROM ranges, if any.
    pub ranges: [linux_prom_ranges; 8],
    pub nranges: c_int,
}

// We use this to acquire receive skb's that we can DMA directly into.

