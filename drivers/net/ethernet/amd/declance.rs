//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/amd/declance.c
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
// Lance ethernet driver for the MIPS processor based
// DECstation family
//
// adopted from sunlance.c by Richard van den Berg
//
// Copyright (C) 2002, 2003, 2005, 2006  Maciej W. Rozycki
//
// additional sources:
// - PMAD-AA TURBOchannel Ethernet Module Functional Specification,
// Revision 1.2
//
// History:
//
// v0.001: The kernel accepts the code and it shows the hardware address.
//
// v0.002: Removed most sparc stuff, left only some module and dma stuff.
//
// v0.003: Enhanced base address calculation from proposals by
// Harald Koerfgen and Thomas Riemer.
//
// v0.004: lance-regs is pointing at the right addresses, added prom
// check. First start of address mapping and DMA.
//
// v0.005: started to play around with LANCE-DMA. This driver will not
// work for non IOASIC lances. HK
//
// v0.006: added pointer arrays to lance_private and setup routine for
// them in dec_lance_init. HK
//
// v0.007: Big shit. The LANCE seems to use a different DMA mechanism to
// access the init block. This looks like one (short) word at a
// time, but the smallest amount the IOASIC can transfer is a
// (long) word. So we have a 2-2 padding here. Changed
// lance_init_block accordingly. The 16-16 padding for the buffers
// seems to be correct. HK
//
// v0.008: mods to make PMAX_LANCE work. 01/09/1999 triemer
//
// v0.009: Module support fixes, multiple interfaces support, various
// bits. macro
//
// v0.010: Fixes for the PMAD mapping of the LANCE buffer and for the
// PMAX requirement to only use halfword accesses to the
// buffer. macro
//
// v0.011: Converted the PMAD to the driver model. macro
//

    static const char version[] =
    "declance.c: v0.011 by Linux MIPS DECstation task force\n";
    MODULE_AUTHOR("Linux MIPS DECstation task force");
    MODULE_DESCRIPTION("DEC LANCE (DECstation onboard, PMAD-xx) driver");
    MODULE_LICENSE("GPL");

//
// card types
//
pub const ASIC_LANCE: c_int = 1;
pub const PMAD_LANCE: c_int = 2;
pub const PMAX_LANCE: c_int = 3;
pub const LE_CSR0: c_int = 0;
pub const LE_CSR1: c_int = 1;
pub const LE_CSR2: c_int = 2;
pub const LE_CSR3: c_int = 3;
pub const LE_MO_PROM: c_uint = 0x8000	/* Enable promiscuous mode */;
pub const LE_C0_ERR: c_uint = 0x8000	/* Error: set if BAB, SQE, MISS or ME is set */;
pub const LE_C0_BABL: c_uint = 0x4000	/* BAB:  Babble: tx timeout. */;
pub const LE_C0_CERR: c_uint = 0x2000	/* SQE:  Signal quality error */;
pub const LE_C0_MISS: c_uint = 0x1000	/* MISS: Missed a packet */;
pub const LE_C0_MERR: c_uint = 0x0800	/* ME:   Memory error */;
pub const LE_C0_RINT: c_uint = 0x0400	/* Received interrupt */;
pub const LE_C0_TINT: c_uint = 0x0200	/* Transmitter Interrupt */;
pub const LE_C0_IDON: c_uint = 0x0100	/* IFIN: Init finished. */;
pub const LE_C0_INTR: c_uint = 0x0080	/* Interrupt or error */;
pub const LE_C0_INEA: c_uint = 0x0040	/* Interrupt enable */;
pub const LE_C0_RXON: c_uint = 0x0020	/* Receiver on */;
pub const LE_C0_TXON: c_uint = 0x0010	/* Transmitter on */;
pub const LE_C0_TDMD: c_uint = 0x0008	/* Transmitter demand */;
pub const LE_C0_STOP: c_uint = 0x0004	/* Stop the card */;
pub const LE_C0_STRT: c_uint = 0x0002	/* Start the card */;
pub const LE_C0_INIT: c_uint = 0x0001	/* Init the card */;
pub const LE_C3_BSWP: c_uint = 0x4	/* SWAP */;
pub const LE_C3_ACON: c_uint = 0x2	/* ALE Control */;
pub const LE_C3_BCON: c_uint = 0x1	/* Byte control */;
// Receive message descriptor 1
pub const LE_R1_OWN: c_uint = 0x8000	/* Who owns the entry */;
pub const LE_R1_ERR: c_uint = 0x4000	/* Error: if FRA, OFL, CRC or BUF is set */;
pub const LE_R1_FRA: c_uint = 0x2000	/* FRA: Frame error */;
pub const LE_R1_OFL: c_uint = 0x1000	/* OFL: Frame overflow */;
pub const LE_R1_CRC: c_uint = 0x0800	/* CRC error */;
pub const LE_R1_BUF: c_uint = 0x0400	/* BUF: Buffer error */;
pub const LE_R1_SOP: c_uint = 0x0200	/* Start of packet */;
pub const LE_R1_EOP: c_uint = 0x0100	/* End of packet */;
pub const LE_R1_POK: c_uint = 0x0300	/* Packet is complete: SOP + EOP */;
// Transmit message descriptor 1
pub const LE_T1_OWN: c_uint = 0x8000	/* Lance owns the packet */;
pub const LE_T1_ERR: c_uint = 0x4000	/* Error summary */;
pub const LE_T1_EMORE: c_uint = 0x1000	/* Error: more than one retry needed */;
pub const LE_T1_EONE: c_uint = 0x0800	/* Error: one retry needed */;
pub const LE_T1_EDEF: c_uint = 0x0400	/* Error: deferred */;
pub const LE_T1_SOP: c_uint = 0x0200	/* Start of packet */;
pub const LE_T1_EOP: c_uint = 0x0100	/* End of packet */;
pub const LE_T1_POK: c_uint = 0x0300	/* Packet is complete: SOP + EOP */;
pub const LE_T3_BUF: c_uint = 0x8000	/* Buffer error */;
pub const LE_T3_UFL: c_uint = 0x4000	/* Error underflow */;
pub const LE_T3_LCOL: c_uint = 0x1000	/* Error late collision */;
pub const LE_T3_CLOS: c_uint = 0x0800	/* Error carrier loss */;
pub const LE_T3_RTY: c_uint = 0x0400	/* Error retry */;
pub const LE_T3_TDR: c_uint = 0x03ff	/* Time Domain Reflectometry counter */;
// Define: 2^4 Tx buffers and 2^4 Rx buffers

pub const LANCE_LOG_TX_BUFFERS: c_int = 4;
pub const LANCE_LOG_RX_BUFFERS: c_int = 4;

pub const PKT_BUF_SZ: c_int = 1536;

pub const ZERO: c_int = 0;
//
// The DS2100/3100 have a linear 64 kB buffer which supports halfword
// accesses only.  Each halfword of the buffer is word-aligned in the
// CPU address space.
//
// The PMAD-AA has a 128 kB buffer on-board.
//
// The IOASIC LANCE devices use a shared memory region.  This region
// as seen from the CPU is (max) 128 kB long and has to be on an 128 kB
// boundary.  The LANCE sees this as a 64 kB long continuous memory
// region.
//
// The LANCE's DMA address is used as an index in this buffer and DMA
// takes place in bursts of eight 16-bit words which are packed into
// four 32-bit words by the IOASIC.  This leads to a strange padding:
// 16 bytes of valid data followed by a 16 byte gap :-(.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lance_rx_desc {
    pub /: *mut *mut unsigned short rmd0; / low address of packet,
    pub packet: *mut *mut unsigned short rmd1; / high address of,
    and descriptor bits */
    pub (negative!): *mut *mut short length; / 2s complement,
    of buffer length */
    pub /: *mut *mut unsigned short mblength; / actual number of bytes received,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lance_tx_desc {
    pub /: *mut *mut unsigned short tmd0; / low address of packet,
    pub packet: *mut *mut unsigned short tmd1; / high address of,
    and descriptor bits */
    pub (negative!): *mut *mut short length; / 2s complement,
    of buffer length */
    pub misc: c_ushort,
}

// First part of the LANCE initialization block, described in databook.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lance_init_block {
    pub /: *mut *mut unsigned short mode; / pre-set mode (reg. 15),
    pub /: *mut *mut unsigned short phys_addr[3]; / physical ethernet address,
    pub /: *mut *mut unsigned short filter[4]; / multicast filter,
// Receive and transmit ring base, along with extra bits.
    pub /: *mut *mut unsigned short rx_ptr; / receive descriptor addr,
    pub /: *mut *mut unsigned short rx_len; / receive len and high addr,
    pub /: *mut *mut unsigned short tx_ptr; / transmit descriptor addr,
    pub /: *mut *mut unsigned short tx_len; / transmit len and high addr,
    pub gap: [c_short; 4],
// The buffer descriptors
    pub brx_ring: [lance_rx_desc; RX_RING_SIZE],
    pub btx_ring: [lance_tx_desc; TX_RING_SIZE],
}

    (type == ASIC_LANCE || type == PMAX_LANCE ? off << 1 : off)

    shift_off(offsetof(struct lance_init_block, rt), type)

    ((volatile u16 *)((u8 *)(ib) + lib_off(rt, type)))

    shift_off(offsetof(struct lance_rx_desc, rt), type)

    ((volatile u16 *)((u8 *)(rd) + rds_off(rt, type)))

    shift_off(offsetof(struct lance_tx_desc, rt), type)

    ((volatile u16 *)((u8 *)(td) + tds_off(rt, type)))
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lance_private {
    pub next: *mut net_device,
    pub type: c_int,
    pub dma_irq: c_int,
    pub ll: *mut volatile struct lance_regs,
    pub lock: spinlock_t,
    pub tx_new: int rx_new,,
    pub tx_old: int rx_old,,
    pub busmaster_regval: c_ushort,
    pub multicast_timer: timer_list,
    pub dev: *mut net_device,
// Pointers to the ring buffers as seen from the CPU
    pub rx_buf_ptr_cpu: [*mut c_char; RX_RING_SIZE],
    pub tx_buf_ptr_cpu: [*mut c_char; TX_RING_SIZE],
// Pointers to the ring buffers as seen from the LANCE
    pub rx_buf_ptr_lnc: [c_uint; RX_RING_SIZE],
    pub tx_buf_ptr_lnc: [c_uint; TX_RING_SIZE],
}

    lp.tx_old+TX_RING_MOD_MASK-lp.tx_new:\
    lp.tx_old - lp.tx_new-1)
// The lance control ports are at an absolute address, machine and tc-slot
// dependent.
// DECstations do only 32-bit access and the LANCE uses 16 bit addresses,
// so we have to give the structure an extra member making rap pointing
// at the right address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lance_regs {
    pub /: *mut *mut volatile unsigned short rdp; / register data port,
    pub pad: c_ushort,
    pub /: *mut *mut volatile unsigned short rap; / register address port,
}

    let mut dec_lance_debug: c_int = 2;
    static struct tc_driver dec_lance_tc_driver;
    static struct net_device *root_lance_dev;
#[no_mangle]
pub unsafe extern "C" fn writereg(regptr: *mut volatile unsigned short, value: c_short) {
    static inline void writereg(volatile unsigned short *regptr, short value)
    {
// regptr = value;
    iob();
    }
// Load the CSR registers
#[no_mangle]
unsafe extern "C" fn load_csrs(lp: *mut lance_private) {
    static void load_csrs(struct lance_private *lp)
    {
    volatile struct lance_regs *ll = lp.ll;
    uint leptr;
// The address space as seen from the LANCE
// begins at address 0. HK
//
    leptr = 0;
    writereg(&ll.rap, LE_CSR1);
    writereg(&ll.rdp, (leptr & 0xFFFF));
    writereg(&ll.rap, LE_CSR2);
    writereg(&ll.rdp, leptr >> 16);
    writereg(&ll.rap, LE_CSR3);
    writereg(&ll.rdp, lp.busmaster_regval);
// Point back to csr0
    writereg(&ll.rap, LE_CSR0);
    }
//
// Our specialized copy routines
//
#[no_mangle]
unsafe extern "C" fn cp_to_buf(type: c_int, to: *mut c_void, from: *const c_void, len: c_int) {
    static void cp_to_buf(const int type, void *to, const void *from, int len)
    {
    unsigned short *tp;
    const unsigned short *fp;
    unsigned short clen;
    unsigned char *rtp;
    const unsigned char *rfp;
    if (type == PMAD_LANCE) {
    memcpy(to, from, len);
    } else if (type == PMAX_LANCE) {
    clen = len >> 1;
    tp = to;
    fp = from;
    while (clen--) {
// tp++ = *fp++;
    tp++;
    }
    clen = len & 1;
    rtp = (unsigned char *)tp;
    rfp = (const unsigned char *)fp;
    while (clen--) {
// rtp++ = *rfp++;
    }
    } else {
//
// copy 16 Byte chunks
//
    clen = len >> 4;
    tp = to;
    fp = from;
    while (clen--) {
// tp++ = *fp++;
    tp += 8;
    }
//
// do the rest, if any.
//
    clen = len & 15;
    rtp = (unsigned char *)tp;
    rfp = (const unsigned char *)fp;
    while (clen--) {
// rtp++ = *rfp++;
    }
    }
    iob();
    }
#[no_mangle]
unsafe extern "C" fn cp_from_buf(type: c_int, to: *mut c_void, from: *const c_void, len: c_int) {
    static void cp_from_buf(const int type, void *to, const void *from, int len)
    {
    unsigned short *tp;
    const unsigned short *fp;
    unsigned short clen;
    unsigned char *rtp;
    const unsigned char *rfp;
    if (type == PMAD_LANCE) {
    memcpy(to, from, len);
    } else if (type == PMAX_LANCE) {
    clen = len >> 1;
    tp = to;
    fp = from;
    while (clen--) {
// tp++ = *fp++;
    fp++;
    }
    clen = len & 1;
    rtp = (unsigned char *)tp;
    rfp = (const unsigned char *)fp;
    while (clen--) {
// rtp++ = *rfp++;
    }
    } else {
//
// copy 16 Byte chunks
//
    clen = len >> 4;
    tp = to;
    fp = from;
    while (clen--) {
// tp++ = *fp++;
    fp += 8;
    }
//
// do the rest, if any.
//
    clen = len & 15;
    rtp = (unsigned char *)tp;
    rfp = (const unsigned char *)fp;
    while (clen--) {
// rtp++ = *rfp++;
    }
    }
    }
// Setup the Lance Rx and Tx rings
#[no_mangle]
unsafe extern "C" fn lance_init_ring(dev: *mut net_device) {
    static void lance_init_ring(struct net_device *dev)
    {
    struct lance_private *lp = netdev_priv(dev);
    volatile u16 *ib = (volatile u16 *)dev.mem_start;
    uint leptr;
    int i;
// Lock out other processes while setting up hardware
    netif_stop_queue(dev);
    lp.rx_new = lp.tx_new = 0;
    lp.rx_old = lp.tx_old = 0;
// Copy the ethernet address to the lance init block.
// XXX bit 0 of the physical address registers has to be zero
//
// lib_ptr(ib, phys_addr[0], lp->type) = (dev->dev_addr[1] << 8) |
    dev.dev_addr[0];
// lib_ptr(ib, phys_addr[1], lp->type) = (dev->dev_addr[3] << 8) |
    dev.dev_addr[2];
// lib_ptr(ib, phys_addr[2], lp->type) = (dev->dev_addr[5] << 8) |
    dev.dev_addr[4];
// Setup the initialization block
// Setup rx descriptor pointer
    leptr = offsetof(struct lance_init_block, brx_ring);
// lib_ptr(ib, rx_len, lp->type) = (LANCE_LOG_RX_BUFFERS << 13) |
    (leptr >> 16);
// lib_ptr(ib, rx_ptr, lp->type) = leptr;
    if (ZERO)
    printk("RX ptr: %8.8x(%8.8x)\n",
    leptr, (uint)lib_off(brx_ring, lp.type));
// Setup tx descriptor pointer
    leptr = offsetof(struct lance_init_block, btx_ring);
// lib_ptr(ib, tx_len, lp->type) = (LANCE_LOG_TX_BUFFERS << 13) |
    (leptr >> 16);
// lib_ptr(ib, tx_ptr, lp->type) = leptr;
    if (ZERO)
    printk("TX ptr: %8.8x(%8.8x)\n",
    leptr, (uint)lib_off(btx_ring, lp.type));
    if (ZERO)
    printk("TX rings:\n");
// Setup the Tx ring entries
    for (i = 0; i < TX_RING_SIZE; i++) {
    leptr = lp.tx_buf_ptr_lnc[i];
// lib_ptr(ib, btx_ring[i].tmd0, lp->type) = leptr;
// lib_ptr(ib, btx_ring[i].tmd1, lp->type) = (leptr >> 16) &
    0xff;
// lib_ptr(ib, btx_ring[i].length, lp->type) = 0xf000;
// The ones required by tmd2
// lib_ptr(ib, btx_ring[i].misc, lp->type) = 0;
    if (i < 3 && ZERO)
    printk("%d: %8.8x(%p)\n",
    i, leptr, lp.tx_buf_ptr_cpu[i]);
    }
// Setup the Rx ring entries
    if (ZERO)
    printk("RX rings:\n");
    for (i = 0; i < RX_RING_SIZE; i++) {
    leptr = lp.rx_buf_ptr_lnc[i];
// lib_ptr(ib, brx_ring[i].rmd0, lp->type) = leptr;
// lib_ptr(ib, brx_ring[i].rmd1, lp->type) = ((leptr >> 16) &
    0xff) |
    LE_R1_OWN;
// lib_ptr(ib, brx_ring[i].length, lp->type) = -RX_BUFF_SIZE |
    0xf000;
// lib_ptr(ib, brx_ring[i].mblength, lp->type) = 0;
    if (i < 3 && ZERO)
    printk("%d: %8.8x(%p)\n",
    i, leptr, lp.rx_buf_ptr_cpu[i]);
    }
    iob();
    }
#[no_mangle]
unsafe extern "C" fn init_restart_lance(lp: *mut lance_private) -> c_int {
    static int init_restart_lance(struct lance_private *lp)
    {
    volatile struct lance_regs *ll = lp.ll;
    int i;
    writereg(&ll.rap, LE_CSR0);
    writereg(&ll.rdp, LE_C0_INIT);
// Wait for the lance to complete initialization
    for (i = 0; (i < 100) && !(ll.rdp & LE_C0_IDON); i++) {
    udelay(10);
    }
    if ((i == 100) || (ll.rdp & LE_C0_ERR)) {
    printk("LANCE unopened after %d ticks, csr0=%4.4x.\n",
    i, ll.rdp);
    return -1;
    }
    if ((ll.rdp & LE_C0_ERR)) {
    printk("LANCE unopened after %d ticks, csr0=%4.4x.\n",
    i, ll.rdp);
    return -1;
    }
    writereg(&ll.rdp, LE_C0_IDON);
    writereg(&ll.rdp, LE_C0_STRT);
    writereg(&ll.rdp, LE_C0_INEA);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lance_rx(dev: *mut net_device) -> c_int {
    static int lance_rx(struct net_device *dev)
    {
    struct lance_private *lp = netdev_priv(dev);
    volatile u16 *ib = (volatile u16 *)dev.mem_start;
    volatile u16 *rd;
    unsigned short bits;
    int entry, len;
    struct sk_buff *skb;

    {
    int i;
    printk("[");
    for (i = 0; i < RX_RING_SIZE; i++) {
    if (i == lp.rx_new)
    printk("%s", *lib_ptr(ib, brx_ring[i].rmd1,
    lp.type) &
    LE_R1_OWN ? "_" : "X");
    else
    printk("%s", *lib_ptr(ib, brx_ring[i].rmd1,
    lp.type) &
    LE_R1_OWN ? "." : "1");
    }
    printk("]");
    }

    for (rd = lib_ptr(ib, brx_ring[lp.rx_new], lp.type);
    !((bits = *rds_ptr(rd, rmd1, lp.type)) & LE_R1_OWN);
    rd = lib_ptr(ib, brx_ring[lp.rx_new], lp.type)) {
    entry = lp.rx_new;
// We got an incomplete frame?
    if ((bits & LE_R1_POK) != LE_R1_POK) {
    dev.stats.rx_over_errors++;
    dev.stats.rx_errors++;
    } else if (bits & LE_R1_ERR) {
// Count only the end frame as a rx error,
// not the beginning
//
    if (bits & LE_R1_BUF)
    dev.stats.rx_fifo_errors++;
    if (bits & LE_R1_CRC)
    dev.stats.rx_crc_errors++;
    if (bits & LE_R1_OFL)
    dev.stats.rx_over_errors++;
    if (bits & LE_R1_FRA)
    dev.stats.rx_frame_errors++;
    if (bits & LE_R1_EOP)
    dev.stats.rx_errors++;
    } else {
    len = (*rds_ptr(rd, mblength, lp.type) & 0xfff) - 4;
    skb = netdev_alloc_skb(dev, len + 2);
    if (!skb) {
    dev.stats.rx_dropped++;
// rds_ptr(rd, mblength, lp->type) = 0;
// rds_ptr(rd, rmd1, lp->type) =
    ((lp.rx_buf_ptr_lnc[entry] >> 16) &
    0xff) | LE_R1_OWN;
    lp.rx_new = (entry + 1) & RX_RING_MOD_MASK;
    return 0;
    }
    dev.stats.rx_bytes += len;
    skb_reserve(skb, 2);	/* 16 byte align */
    skb_put(skb, len);	/* make room */
    cp_from_buf(lp.type, skb.data,
    lp.rx_buf_ptr_cpu[entry], len);
    skb.protocol = eth_type_trans(skb, dev);
    netif_rx(skb);
    dev.stats.rx_packets++;
    }
// Return the packet to the pool
// rds_ptr(rd, mblength, lp->type) = 0;
// rds_ptr(rd, length, lp->type) = -RX_BUFF_SIZE | 0xf000;
// rds_ptr(rd, rmd1, lp->type) =
    ((lp.rx_buf_ptr_lnc[entry] >> 16) & 0xff) | LE_R1_OWN;
    lp.rx_new = (entry + 1) & RX_RING_MOD_MASK;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lance_tx(dev: *mut net_device) {
    static void lance_tx(struct net_device *dev)
    {
    struct lance_private *lp = netdev_priv(dev);
    volatile u16 *ib = (volatile u16 *)dev.mem_start;
    volatile struct lance_regs *ll = lp.ll;
    volatile u16 *td;
    int i, j;
    int status;
    j = lp.tx_old;
    spin_lock(&lp.lock);
    for (i = j; i != lp.tx_new; i = j) {
    td = lib_ptr(ib, btx_ring[i], lp.type);
// If we hit a packet not owned by us, stop
    if (*tds_ptr(td, tmd1, lp.type) & LE_T1_OWN)
    break;
    if (*tds_ptr(td, tmd1, lp.type) & LE_T1_ERR) {
    status = *tds_ptr(td, misc, lp.type);
    dev.stats.tx_errors++;
    if (status & LE_T3_RTY)
    dev.stats.tx_aborted_errors++;
    if (status & LE_T3_LCOL)
    dev.stats.tx_window_errors++;
    if (status & LE_T3_CLOS) {
    dev.stats.tx_carrier_errors++;
    printk("%s: Carrier Lost\n", dev.name);
// Stop the lance
    writereg(&ll.rap, LE_CSR0);
    writereg(&ll.rdp, LE_C0_STOP);
    lance_init_ring(dev);
    load_csrs(lp);
    init_restart_lance(lp);
    goto out;
    }
// Buffer errors and underflows turn off the
// transmitter, restart the adapter.
//
    if (status & (LE_T3_BUF | LE_T3_UFL)) {
    dev.stats.tx_fifo_errors++;
    printk("%s: Tx: ERR_BUF|ERR_UFL, restarting\n",
    dev.name);
// Stop the lance
    writereg(&ll.rap, LE_CSR0);
    writereg(&ll.rdp, LE_C0_STOP);
    lance_init_ring(dev);
    load_csrs(lp);
    init_restart_lance(lp);
    goto out;
    }
    } else if ((*tds_ptr(td, tmd1, lp.type) & LE_T1_POK) ==
    LE_T1_POK) {
//
// So we don't count the packet more than once.
//
// tds_ptr(td, tmd1, lp->type) &= ~(LE_T1_POK);
// One collision before packet was sent.
    if (*tds_ptr(td, tmd1, lp.type) & LE_T1_EONE)
    dev.stats.collisions++;
// More than one collision, be optimistic.
    if (*tds_ptr(td, tmd1, lp.type) & LE_T1_EMORE)
    dev.stats.collisions += 2;
    dev.stats.tx_packets++;
    }
    j = (j + 1) & TX_RING_MOD_MASK;
    }
    lp.tx_old = j;
    out:
    if (netif_queue_stopped(dev) &&
    TX_BUFFS_AVAIL > 0)
    netif_wake_queue(dev);
    spin_unlock(&lp.lock);
    }
#[no_mangle]
unsafe extern "C" fn lance_dma_merr_int(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t lance_dma_merr_int(int irq, void *dev_id)
    {
    struct net_device *dev = dev_id;
    let mut ldp: u64 = ioasic_read(IO_REG_LANCE_DMA_P);
    pr_err_ratelimited("%s: DMA error at %#010llx\n", dev.name,
    (ldp & 0x1f) << 29 | (ldp & 0xffffffe0) >> 3);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn lance_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t lance_interrupt(int irq, void *dev_id)
    {
    struct net_device *dev = dev_id;
    struct lance_private *lp = netdev_priv(dev);
    volatile struct lance_regs *ll = lp.ll;
    int csr0;
    writereg(&ll.rap, LE_CSR0);
    csr0 = ll.rdp;
// Acknowledge all the interrupt sources ASAP
    writereg(&ll.rdp, csr0 & (LE_C0_INTR | LE_C0_TINT | LE_C0_RINT));
    if ((csr0 & LE_C0_ERR)) {
// Clear the error condition
    writereg(&ll.rdp, LE_C0_BABL | LE_C0_ERR | LE_C0_MISS |
    LE_C0_CERR | LE_C0_MERR);
    }
    if (csr0 & LE_C0_RINT)
    lance_rx(dev);
    if (csr0 & LE_C0_TINT)
    lance_tx(dev);
    if (csr0 & LE_C0_BABL)
    dev.stats.tx_errors++;
    if (csr0 & LE_C0_MISS)
    dev.stats.rx_errors++;
    if (csr0 & LE_C0_MERR) {
    printk("%s: Memory error, status %04x\n", dev.name, csr0);
    writereg(&ll.rdp, LE_C0_STOP);
    lance_init_ring(dev);
    load_csrs(lp);
    init_restart_lance(lp);
    netif_wake_queue(dev);
    }
    writereg(&ll.rdp, LE_C0_INEA);
    writereg(&ll.rdp, LE_C0_INEA);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn lance_open(dev: *mut net_device) -> c_int {
    static int lance_open(struct net_device *dev)
    {
    volatile u16 *ib = (volatile u16 *)dev.mem_start;
    struct lance_private *lp = netdev_priv(dev);
    volatile struct lance_regs *ll = lp.ll;
    let mut status: c_int = 0;
// Stop the Lance
    writereg(&ll.rap, LE_CSR0);
    writereg(&ll.rdp, LE_C0_STOP);
// Set mode and clear multicast filter only at device open,
// so that lance_init_ring() called at any error will not
// forget multicast filters.
//
// BTW it is common bug in all lance drivers! --ANK
//
// lib_ptr(ib, mode, lp->type) = 0;
// lib_ptr(ib, filter[0], lp->type) = 0;
// lib_ptr(ib, filter[1], lp->type) = 0;
// lib_ptr(ib, filter[2], lp->type) = 0;
// lib_ptr(ib, filter[3], lp->type) = 0;
    lance_init_ring(dev);
    load_csrs(lp);
    netif_start_queue(dev);
// Associate IRQ with lance_interrupt
    if (request_irq(dev.irq, lance_interrupt, 0, "lance", dev)) {
    printk("%s: Can't get IRQ %d\n", dev.name, dev.irq);
    return -EAGAIN;
    }
    if (lp.dma_irq >= 0) {
    unsigned long flags;
    if (request_irq(lp.dma_irq, lance_dma_merr_int, 0,
    "lance error", dev)) {
    free_irq(dev.irq, dev);
    printk("%s: Can't get DMA IRQ %d\n", dev.name,
    lp.dma_irq);
    return -EAGAIN;
    }
    spin_lock_irqsave(&ioasic_ssr_lock, flags);
    fast_mb();
// Enable I/O ASIC LANCE DMA.
    ioasic_write(IO_REG_SSR,
    ioasic_read(IO_REG_SSR) | IO_SSR_LANCE_DMA_EN);
    fast_mb();
    spin_unlock_irqrestore(&ioasic_ssr_lock, flags);
    }
    status = init_restart_lance(lp);
    return status;
    }
#[no_mangle]
unsafe extern "C" fn lance_close(dev: *mut net_device) -> c_int {
    static int lance_close(struct net_device *dev)
    {
    struct lance_private *lp = netdev_priv(dev);
    volatile struct lance_regs *ll = lp.ll;
    netif_stop_queue(dev);
    timer_delete_sync(&lp.multicast_timer);
// Stop the card
    writereg(&ll.rap, LE_CSR0);
    writereg(&ll.rdp, LE_C0_STOP);
    if (lp.dma_irq >= 0) {
    unsigned long flags;
    spin_lock_irqsave(&ioasic_ssr_lock, flags);
    fast_mb();
// Disable I/O ASIC LANCE DMA.
    ioasic_write(IO_REG_SSR,
    ioasic_read(IO_REG_SSR) & ~IO_SSR_LANCE_DMA_EN);
    fast_iob();
    spin_unlock_irqrestore(&ioasic_ssr_lock, flags);
    free_irq(lp.dma_irq, dev);
    }
    free_irq(dev.irq, dev);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn lance_reset(dev: *mut net_device) -> c_int {
    static inline int lance_reset(struct net_device *dev)
    {
    struct lance_private *lp = netdev_priv(dev);
    volatile struct lance_regs *ll = lp.ll;
    int status;
// Stop the lance
    writereg(&ll.rap, LE_CSR0);
    writereg(&ll.rdp, LE_C0_STOP);
    lance_init_ring(dev);
    load_csrs(lp);
    netif_trans_update(dev); /* prevent tx timeout */
    status = init_restart_lance(lp);
    return status;
    }
#[no_mangle]
unsafe extern "C" fn lance_tx_timeout(dev: *mut net_device, txqueue: c_uint) {
    static void lance_tx_timeout(struct net_device *dev, unsigned int txqueue)
    {
    struct lance_private *lp = netdev_priv(dev);
    volatile struct lance_regs *ll = lp.ll;
    printk(KERN_ERR "%s: transmit timed out, status %04x, reset\n",
    dev.name, ll.rdp);
    lance_reset(dev);
    netif_wake_queue(dev);
    }
#[no_mangle]
unsafe extern "C" fn lance_start_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t {
    static netdev_tx_t lance_start_xmit(struct sk_buff *skb, struct net_device *dev)
    {
    struct lance_private *lp = netdev_priv(dev);
    volatile struct lance_regs *ll = lp.ll;
    volatile u16 *ib = (volatile u16 *)dev.mem_start;
    unsigned long flags;
    int entry, len;
    len = skb.len;
    if (len < ETH_ZLEN) {
    if (skb_padto(skb, ETH_ZLEN))
    return NETDEV_TX_OK;
    len = ETH_ZLEN;
    }
    dev.stats.tx_bytes += len;
    spin_lock_irqsave(&lp.lock, flags);
    entry = lp.tx_new;
// lib_ptr(ib, btx_ring[entry].length, lp->type) = (-len);
// lib_ptr(ib, btx_ring[entry].misc, lp->type) = 0;
    cp_to_buf(lp.type, lp.tx_buf_ptr_cpu[entry], skb.data, len);
// Now, give the packet to the lance
// lib_ptr(ib, btx_ring[entry].tmd1, lp->type) =
    ((lp.tx_buf_ptr_lnc[entry] >> 16) & 0xff) |
    (LE_T1_POK | LE_T1_OWN);
    lp.tx_new = (entry + 1) & TX_RING_MOD_MASK;
    if (TX_BUFFS_AVAIL <= 0)
    netif_stop_queue(dev);
// Kick the lance: transmit now
    writereg(&ll.rdp, LE_C0_INEA | LE_C0_TDMD);
    spin_unlock_irqrestore(&lp.lock, flags);
    dev_kfree_skb(skb);
    return NETDEV_TX_OK;
    }
#[no_mangle]
unsafe extern "C" fn lance_load_multicast(dev: *mut net_device) {
    static void lance_load_multicast(struct net_device *dev)
    {
    struct lance_private *lp = netdev_priv(dev);
    volatile u16 *ib = (volatile u16 *)dev.mem_start;
    struct netdev_hw_addr *ha;
    u32 crc;
// set all multicast bits
    if (dev.flags & IFF_ALLMULTI) {
// lib_ptr(ib, filter[0], lp->type) = 0xffff;
// lib_ptr(ib, filter[1], lp->type) = 0xffff;
// lib_ptr(ib, filter[2], lp->type) = 0xffff;
// lib_ptr(ib, filter[3], lp->type) = 0xffff;
    return;
    }
// clear the multicast filter
// lib_ptr(ib, filter[0], lp->type) = 0;
// lib_ptr(ib, filter[1], lp->type) = 0;
// lib_ptr(ib, filter[2], lp->type) = 0;
// lib_ptr(ib, filter[3], lp->type) = 0;
// Add addresses
    netdev_for_each_mc_addr(ha, dev) {
    crc = ether_crc_le(ETH_ALEN, ha.addr);
    crc = crc >> 26;
// lib_ptr(ib, filter[crc >> 4], lp->type) |= 1 << (crc & 0xf);
    }
    }
#[no_mangle]
unsafe extern "C" fn lance_set_multicast(dev: *mut net_device) {
    static void lance_set_multicast(struct net_device *dev)
    {
    struct lance_private *lp = netdev_priv(dev);
    volatile u16 *ib = (volatile u16 *)dev.mem_start;
    volatile struct lance_regs *ll = lp.ll;
    if (!netif_running(dev))
    return;
    if (lp.tx_old != lp.tx_new) {
    mod_timer(&lp.multicast_timer, jiffies + 4 * HZ/100);
    netif_wake_queue(dev);
    return;
    }
    netif_stop_queue(dev);
    writereg(&ll.rap, LE_CSR0);
    writereg(&ll.rdp, LE_C0_STOP);
    lance_init_ring(dev);
    if (dev.flags & IFF_PROMISC) {
// lib_ptr(ib, mode, lp->type) |= LE_MO_PROM;
    } else {
// lib_ptr(ib, mode, lp->type) &= ~LE_MO_PROM;
    lance_load_multicast(dev);
    }
    load_csrs(lp);
    init_restart_lance(lp);
    netif_wake_queue(dev);
    }
#[no_mangle]
unsafe extern "C" fn lance_set_multicast_retry(t: *mut timer_list) {
    static void lance_set_multicast_retry(struct timer_list *t)
    {
    struct lance_private *lp = timer_container_of(lp, t, multicast_timer);
    struct net_device *dev = lp.dev;
    lance_set_multicast(dev);
    }
    static const struct net_device_ops lance_netdev_ops = {
    .ndo_open		= lance_open,
    .ndo_stop		= lance_close,
    .ndo_start_xmit		= lance_start_xmit,
    .ndo_tx_timeout		= lance_tx_timeout,
    .ndo_set_rx_mode	= lance_set_multicast,
    .ndo_validate_addr	= eth_validate_addr,
    .ndo_set_mac_address	= eth_mac_addr,
    };
#[no_mangle]
unsafe extern "C" fn dec_lance_probe(bdev: *mut device, type: c_int) -> c_int {
    static int dec_lance_probe(struct device *bdev, const int type)
    {
    static unsigned version_printed;
    static const char fmt[] = "declance%d";
    char name[10];
    struct net_device *dev;
    struct lance_private *lp;
    volatile struct lance_regs *ll;
    let mut start: resource_size_t = 0, len = 0;
    int i, ret;
    unsigned long esar_base;
    unsigned char *esar;
    u8 addr[ETH_ALEN];
    const char *desc;
    if (dec_lance_debug && version_printed++ == 0)
    printk(version);
    if (bdev)
    snprintf(name, sizeof(name), "%s", dev_name(bdev));
    else {
    i = 0;
    dev = root_lance_dev;
    while (dev) {
    i++;
    lp = netdev_priv(dev);
    dev = lp.next;
    }
    snprintf(name, sizeof(name), fmt, i);
    }
    dev = alloc_etherdev(sizeof(struct lance_private));
    if (!dev) {
    ret = -ENOMEM;
    goto err_out;
    }
//
// alloc_etherdev ensures the data structures used by the LANCE
// are aligned.
//
    lp = netdev_priv(dev);
    spin_lock_init(&lp.lock);
    lp.type = type;
    switch (type) {
    case ASIC_LANCE:
    dev.base_addr = CKSEG1ADDR(dec_kn_slot_base + IOASIC_LANCE);
// buffer space for the on-board LANCE shared memory
//
// FIXME: ugly hack!
//
    dev.mem_start = CKSEG1ADDR(0x00020000);
    dev.mem_end = dev.mem_start + 0x00020000;
    dev.irq = dec_interrupt[DEC_IRQ_LANCE];
    esar_base = CKSEG1ADDR(dec_kn_slot_base + IOASIC_ESAR);
// Workaround crash with booting KN04 2.1k from Disk
    memset((void *)dev.mem_start, 0,
    dev.mem_end - dev.mem_start);
//
// setup the pointer arrays, this sucks [tm] :-(
//
    for (i = 0; i < RX_RING_SIZE; i++) {
    lp.rx_buf_ptr_cpu[i] =
    (char *)(dev.mem_start + 2 * BUF_OFFSET_CPU +
    2 * i * RX_BUFF_SIZE);
    lp.rx_buf_ptr_lnc[i] =
    (BUF_OFFSET_LNC + i * RX_BUFF_SIZE);
    }
    for (i = 0; i < TX_RING_SIZE; i++) {
    lp.tx_buf_ptr_cpu[i] =
    (char *)(dev.mem_start + 2 * BUF_OFFSET_CPU +
    2 * RX_RING_SIZE * RX_BUFF_SIZE +
    2 * i * TX_BUFF_SIZE);
    lp.tx_buf_ptr_lnc[i] =
    (BUF_OFFSET_LNC +
    RX_RING_SIZE * RX_BUFF_SIZE +
    i * TX_BUFF_SIZE);
    }
// Setup I/O ASIC LANCE DMA.
    lp.dma_irq = dec_interrupt[DEC_IRQ_LANCE_MERR];
    ioasic_write(IO_REG_LANCE_DMA_P,
    CPHYSADDR(dev.mem_start) << 3);
    break;

    case PMAD_LANCE:
    dev_set_drvdata(bdev, dev);
    start = to_tc_dev(bdev).resource.start;
    len = to_tc_dev(bdev).resource.end - start + 1;
    if (!request_mem_region(start, len, dev_name(bdev))) {
    printk(KERN_ERR
    "%s: Unable to reserve MMIO resource\n",
    dev_name(bdev));
    ret = -EBUSY;
    goto err_out_dev;
    }
    dev.mem_start = CKSEG1ADDR(start);
    dev.mem_end = dev.mem_start + 0x100000;
    dev.base_addr = dev.mem_start + 0x100000;
    dev.irq = to_tc_dev(bdev).interrupt;
    esar_base = dev.mem_start + 0x1c0002;
    lp.dma_irq = -1;
    for (i = 0; i < RX_RING_SIZE; i++) {
    lp.rx_buf_ptr_cpu[i] =
    (char *)(dev.mem_start + BUF_OFFSET_CPU +
    i * RX_BUFF_SIZE);
    lp.rx_buf_ptr_lnc[i] =
    (BUF_OFFSET_LNC + i * RX_BUFF_SIZE);
    }
    for (i = 0; i < TX_RING_SIZE; i++) {
    lp.tx_buf_ptr_cpu[i] =
    (char *)(dev.mem_start + BUF_OFFSET_CPU +
    RX_RING_SIZE * RX_BUFF_SIZE +
    i * TX_BUFF_SIZE);
    lp.tx_buf_ptr_lnc[i] =
    (BUF_OFFSET_LNC +
    RX_RING_SIZE * RX_BUFF_SIZE +
    i * TX_BUFF_SIZE);
    }
    break;

    case PMAX_LANCE:
    dev.irq = dec_interrupt[DEC_IRQ_LANCE];
    dev.base_addr = CKSEG1ADDR(KN01_SLOT_BASE + KN01_LANCE);
    dev.mem_start = CKSEG1ADDR(KN01_SLOT_BASE + KN01_LANCE_MEM);
    dev.mem_end = dev.mem_start + KN01_SLOT_SIZE;
    esar_base = CKSEG1ADDR(KN01_SLOT_BASE + KN01_ESAR + 1);
    lp.dma_irq = -1;
//
// setup the pointer arrays, this sucks [tm] :-(
//
    for (i = 0; i < RX_RING_SIZE; i++) {
    lp.rx_buf_ptr_cpu[i] =
    (char *)(dev.mem_start + 2 * BUF_OFFSET_CPU +
    2 * i * RX_BUFF_SIZE);
    lp.rx_buf_ptr_lnc[i] =
    (BUF_OFFSET_LNC + i * RX_BUFF_SIZE);
    }
    for (i = 0; i < TX_RING_SIZE; i++) {
    lp.tx_buf_ptr_cpu[i] =
    (char *)(dev.mem_start + 2 * BUF_OFFSET_CPU +
    2 * RX_RING_SIZE * RX_BUFF_SIZE +
    2 * i * TX_BUFF_SIZE);
    lp.tx_buf_ptr_lnc[i] =
    (BUF_OFFSET_LNC +
    RX_RING_SIZE * RX_BUFF_SIZE +
    i * TX_BUFF_SIZE);
    }
    break;
    default:
    printk(KERN_ERR "%s: declance_init called with unknown type\n",
    name);
    ret = -ENODEV;
    goto err_out_dev;
    }
    ll = (struct lance_regs *) dev.base_addr;
    esar = (unsigned char *) esar_base;
// prom checks
// First, check for test pattern
    if (esar[0x60] != 0xff && esar[0x64] != 0x00 &&
    esar[0x68] != 0x55 && esar[0x6c] != 0xaa) {
    printk(KERN_ERR
    "%s: Ethernet station address prom not found!\n",
    name);
    ret = -ENODEV;
    goto err_out_resource;
    }
// Check the prom contents
    for (i = 0; i < 8; i++) {
    if (esar[i * 4] != esar[0x3c - i * 4] &&
    esar[i * 4] != esar[0x40 + i * 4] &&
    esar[0x3c - i * 4] != esar[0x40 + i * 4]) {
    printk(KERN_ERR "%s: Something is wrong with the "
    "ethernet station address prom!\n", name);
    ret = -ENODEV;
    goto err_out_resource;
    }
    }
// Copy the ethernet address to the device structure, later to the
// lance initialization block so the lance gets it every time it's
// (re)initialized.
//
    switch (type) {
    case ASIC_LANCE:
    desc = "IOASIC onboard LANCE";
    break;
    case PMAD_LANCE:
    desc = "PMAD-AA";
    break;
    case PMAX_LANCE:
    desc = "PMAX onboard LANCE";
    break;
    }
    for (i = 0; i < 6; i++)
    addr[i] = esar[i * 4];
    eth_hw_addr_set(dev, addr);
    printk("%s: %s, addr = %pM, irq = %d\n",
    name, desc, dev.dev_addr, dev.irq);
    dev.netdev_ops = &lance_netdev_ops;
    dev.watchdog_timeo = 5*HZ;
// lp->ll is the location of the registers for lance card
    lp.ll = ll;
// busmaster_regval (CSR3) should be zero according to the PMAD-AA
// specification.
//
    lp.busmaster_regval = 0;
    dev.dma = 0;
// We cannot sleep if the chip is busy during a
// multicast list update event, because such events
// can occur from interrupts (ex. IPv6).  So we
// use a timer to try again later when necessary. -DaveM
//
    lp.dev = dev;
    timer_setup(&lp.multicast_timer, lance_set_multicast_retry, 0);
    ret = register_netdev(dev);
    if (ret) {
    printk(KERN_ERR
    "%s: Unable to register netdev, aborting.\n", name);
    goto err_out_resource;
    }
    if (!bdev) {
    lp.next = root_lance_dev;
    root_lance_dev = dev;
    }
    printk("%s: registered as %s.\n", name, dev.name);
    return 0;
    err_out_resource:
    if (bdev)
    release_mem_region(start, len);
    err_out_dev:
    free_netdev(dev);
    err_out:
    return ret;
    }
// Find all the lance cards on the system and initialize them
#[no_mangle]
unsafe extern "C" fn dec_lance_platform_probe() -> int __init {
    static int __init dec_lance_platform_probe(void)
    {
    let mut count: c_int = 0;
    if (dec_interrupt[DEC_IRQ_LANCE] >= 0) {
    if (dec_interrupt[DEC_IRQ_LANCE_MERR] >= 0) {
    if (dec_lance_probe(core::ptr::null_mut(), ASIC_LANCE) >= 0)
    count++;
    } else if (!TURBOCHANNEL) {
    if (dec_lance_probe(core::ptr::null_mut(), PMAX_LANCE) >= 0)
    count++;
    }
    }
    return (count > 0) ? 0 : -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn dec_lance_platform_remove() -> void __exit {
    static void __exit dec_lance_platform_remove(void)
    {
    while (root_lance_dev) {
    struct net_device *dev = root_lance_dev;
    struct lance_private *lp = netdev_priv(dev);
    unregister_netdev(dev);
    root_lance_dev = lp.next;
    free_netdev(dev);
    }
    }

    static int dec_lance_tc_probe(struct device *dev);
    static int dec_lance_tc_remove(struct device *dev);
    static const struct tc_device_id dec_lance_tc_table[] = {
    { "DEC     ", "PMAD-AA " },
    { }
    };
    MODULE_DEVICE_TABLE(tc, dec_lance_tc_table);
    static struct tc_driver dec_lance_tc_driver = {
    .id_table	= dec_lance_tc_table,
    .driver		= {
    .name	= "declance",
    .bus	= &tc_bus_type,
    .probe	= dec_lance_tc_probe,
    .remove	= dec_lance_tc_remove,
    },
    };
#[no_mangle]
unsafe extern "C" fn dec_lance_tc_probe(dev: *mut device) -> c_int {
    static int dec_lance_tc_probe(struct device *dev)
    {
    let mut status: c_int = dec_lance_probe(dev, PMAD_LANCE);
    if (!status)
    get_device(dev);
    return status;
    }
#[no_mangle]
unsafe extern "C" fn dec_lance_remove(bdev: *mut device) {
    static void dec_lance_remove(struct device *bdev)
    {
    struct net_device *dev = dev_get_drvdata(bdev);
    resource_size_t start, len;
    unregister_netdev(dev);
    start = to_tc_dev(bdev).resource.start;
    len = to_tc_dev(bdev).resource.end - start + 1;
    release_mem_region(start, len);
    free_netdev(dev);
    }
#[no_mangle]
unsafe extern "C" fn dec_lance_tc_remove(dev: *mut device) -> c_int {
    static int dec_lance_tc_remove(struct device *dev)
    {
    put_device(dev);
    dec_lance_remove(dev);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn dec_lance_init() -> int __init {
    static int __init dec_lance_init(void)
    {
    int status;
    status = tc_register_driver(&dec_lance_tc_driver);
    if (!status)
    dec_lance_platform_probe();
    return status;
    }
#[no_mangle]
unsafe extern "C" fn dec_lance_exit() -> void __exit {
    static void __exit dec_lance_exit(void)
    {
    dec_lance_platform_remove();
    tc_unregister_driver(&dec_lance_tc_driver);
    }
    module_init(dec_lance_init);
    module_exit(dec_lance_exit);
