//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/8390/mcf8390.c
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
// Support for ColdFire CPU based boards using a NS8390 Ethernet device.
//
// Derived from the many other 8390 drivers.
//
// (C) Copyright 2012,  Greg Ungerer <gerg@uclinux.org>
//

    static const char version[] =
    "mcf8390.c: (15-06-2012) Greg Ungerer <gerg@uclinux.org>";
pub const NE_CMD: c_uint = 0x00;
pub const NE_DATAPORT: c_uint = 0x10	/* NatSemi-defined port window offset */;
pub const NE_RESET: c_uint = 0x1f	/* Issue a read to reset ,a write to clear */;
pub const NE_EN0_ISR: c_uint = 0x07;
pub const NE_EN0_DCFG: c_uint = 0x0e;
pub const NE_EN0_RSARLO: c_uint = 0x08;
pub const NE_EN0_RSARHI: c_uint = 0x09;
pub const NE_EN0_RCNTLO: c_uint = 0x0a;
pub const NE_EN0_RXCR: c_uint = 0x0c;
pub const NE_EN0_TXCR: c_uint = 0x0d;
pub const NE_EN0_RCNTHI: c_uint = 0x0b;
pub const NE_EN0_IMR: c_uint = 0x0f;
pub const NESM_START_PG: c_uint = 0x40	/* First page of TX buffer */;
pub const NESM_STOP_PG: c_uint = 0x80	/* Last page +1 of RX ring */;

//
// A lot of the ColdFire boards use a separate address region for odd offset
// register addresses. The following functions convert and map as required.
// Note that the data port accesses are treated a little differently, and
// always accessed via the insX/outsX functions.
//
#[no_mangle]
pub unsafe extern "C" fn NE_PTR(addr: u32) -> u32 {
    static inline u32 NE_PTR(u32 addr)
    {
    if (addr & 1)
    return addr - 1 + NE2000_ODDOFFSET;
    return addr;
    }
#[no_mangle]
pub unsafe extern "C" fn NE_DATA_PTR(addr: u32) -> u32 {
    static inline u32 NE_DATA_PTR(u32 addr)
    {
    return addr;
    }
#[no_mangle]
pub unsafe extern "C" fn ei_outb(val: u32, addr: u32) {
    void ei_outb(u32 val, u32 addr)
    {
    NE2000_BYTE *rp;
    rp = (NE2000_BYTE *) NE_PTR(addr);
// rp = RSWAP(val);
    }

#[no_mangle]
pub unsafe extern "C" fn ei_inb(addr: u32) -> u8 {
    u8 ei_inb(u32 addr)
    {
    NE2000_BYTE *rp, val;
    rp = (NE2000_BYTE *) NE_PTR(addr);
    val = *rp;
    return (u8) (RSWAP(val) & 0xff);
    }
#[no_mangle]
pub unsafe extern "C" fn ei_insb(addr: u32, vbuf: *mut c_void, len: c_int) {
    void ei_insb(u32 addr, void *vbuf, int len)
    {
    NE2000_BYTE *rp, val;
    u8 *buf;
    buf = (u8 *) vbuf;
    rp = (NE2000_BYTE *) NE_DATA_PTR(addr);
    for (; (len > 0); len--) {
    val = *rp;
// buf++ = RSWAP(val);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ei_insw(addr: u32, vbuf: *mut c_void, len: c_int) {
    void ei_insw(u32 addr, void *vbuf, int len)
    {
    volatile u16 *rp;
    u16 w, *buf;
    buf = (u16 *) vbuf;
    rp = (volatile u16 *) NE_DATA_PTR(addr);
    for (; (len > 0); len--) {
    w = *rp;
// buf++ = BSWAP(w);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ei_outsb(addr: u32, vbuf: *const c_void, len: c_int) {
    void ei_outsb(u32 addr, const void *vbuf, int len)
    {
    NE2000_BYTE *rp, val;
    u8 *buf;
    buf = (u8 *) vbuf;
    rp = (NE2000_BYTE *) NE_DATA_PTR(addr);
    for (; (len > 0); len--) {
    val = *buf++;
// rp = RSWAP(val);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ei_outsw(addr: u32, vbuf: *const c_void, len: c_int) {
    void ei_outsw(u32 addr, const void *vbuf, int len)
    {
    volatile u16 *rp;
    u16 w, *buf;
    buf = (u16 *) vbuf;
    rp = (volatile u16 *) NE_DATA_PTR(addr);
    for (; (len > 0); len--) {
    w = *buf++;
// rp = BSWAP(w);
    }
    }

//
// Hard reset the card. This used to pause for the same period that a
// 8390 reset command required, but that shouldn't be necessary.
//
#[no_mangle]
unsafe extern "C" fn mcf8390_reset_8390(dev: *mut net_device) {
    static void mcf8390_reset_8390(struct net_device *dev)
    {
    let mut reset_start_time: c_ulong = jiffies;
    let mut addr: u32 = dev.base_addr;
    struct ei_device *ei_local = netdev_priv(dev);
    netif_dbg(ei_local, hw, dev, "resetting the 8390 t=%ld...\n", jiffies);
    ei_outb(ei_inb(addr + NE_RESET), addr + NE_RESET);
    ei_status.txing = 0;
    ei_status.dmaing = 0;
// This check _should_not_ be necessary, omit eventually.
    while ((ei_inb(addr + NE_EN0_ISR) & ENISR_RESET) == 0) {
    if (time_after(jiffies, reset_start_time + 2 * HZ / 100)) {
    netdev_warn(dev, "%s: did not complete\n", __func__);
    break;
    }
    }
    ei_outb(ENISR_RESET, addr + NE_EN0_ISR);
    }
//
// This *shouldn't* happen.
// If it does, it's the last thing you'll see
//
    static void mcf8390_dmaing_err(const char *func, struct net_device *dev,
    struct ei_device *ei_local)
    {
    netdev_err(dev, "%s: DMAing conflict [DMAstat:%d][irqlock:%d]\n",
    func, ei_local.dmaing, ei_local.irqlock);
    }
//
// Grab the 8390 specific header. Similar to the block_input routine, but
// we don't need to be concerned with ring wrap as the header will be at
// the start of a page, so we optimize accordingly.
//
    static void mcf8390_get_8390_hdr(struct net_device *dev,
    struct e8390_pkt_hdr *hdr, int ring_page)
    {
    struct ei_device *ei_local = netdev_priv(dev);
    let mut addr: u32 = dev.base_addr;
    if (ei_local.dmaing) {
    mcf8390_dmaing_err(__func__, dev, ei_local);
    return;
    }
    ei_local.dmaing |= 0x01;
    ei_outb(E8390_NODMA + E8390_PAGE0 + E8390_START, addr + NE_CMD);
    ei_outb(ENISR_RDC, addr + NE_EN0_ISR);
    ei_outb(sizeof(struct e8390_pkt_hdr), addr + NE_EN0_RCNTLO);
    ei_outb(0, addr + NE_EN0_RCNTHI);
    ei_outb(0, addr + NE_EN0_RSARLO);		/* On page boundary */
    ei_outb(ring_page, addr + NE_EN0_RSARHI);
    ei_outb(E8390_RREAD + E8390_START, addr + NE_CMD);
    ei_insw(addr + NE_DATAPORT, hdr, sizeof(struct e8390_pkt_hdr) >> 1);
    outb(ENISR_RDC, addr + NE_EN0_ISR);	/* Ack intr */
    ei_local.dmaing &= ~0x01;
    hdr.count = cpu_to_le16(hdr.count);
    }
//
// Block input and output, similar to the Crynwr packet driver.
// If you are porting to a new ethercard, look at the packet driver source
// for hints. The NEx000 doesn't share the on-board packet memory --
// you have to put the packet out through the "remote DMA" dataport
// using z_writeb.
//
    static void mcf8390_block_input(struct net_device *dev, int count,
    struct sk_buff *skb, int ring_offset)
    {
    struct ei_device *ei_local = netdev_priv(dev);
    let mut addr: u32 = dev.base_addr;
    char *buf = skb.data;
    if (ei_local.dmaing) {
    mcf8390_dmaing_err(__func__, dev, ei_local);
    return;
    }
    ei_local.dmaing |= 0x01;
    ei_outb(E8390_NODMA + E8390_PAGE0 + E8390_START, addr + NE_CMD);
    ei_outb(ENISR_RDC, addr + NE_EN0_ISR);
    ei_outb(count & 0xff, addr + NE_EN0_RCNTLO);
    ei_outb(count >> 8, addr + NE_EN0_RCNTHI);
    ei_outb(ring_offset & 0xff, addr + NE_EN0_RSARLO);
    ei_outb(ring_offset >> 8, addr + NE_EN0_RSARHI);
    ei_outb(E8390_RREAD + E8390_START, addr + NE_CMD);
    ei_insw(addr + NE_DATAPORT, buf, count >> 1);
    if (count & 1)
    buf[count - 1] = ei_inb(addr + NE_DATAPORT);
    ei_outb(ENISR_RDC, addr + NE_EN0_ISR);	/* Ack intr */
    ei_local.dmaing &= ~0x01;
    }
    static void mcf8390_block_output(struct net_device *dev, int count,
    const unsigned char *buf,
    const int start_page)
    {
    struct ei_device *ei_local = netdev_priv(dev);
    let mut addr: u32 = dev.base_addr;
    unsigned long dma_start;
// Make sure we transfer all bytes if 16bit IO writes
    if (count & 0x1)
    count++;
    if (ei_local.dmaing) {
    mcf8390_dmaing_err(__func__, dev, ei_local);
    return;
    }
    ei_local.dmaing |= 0x01;
// We should already be in page 0, but to be safe...
    ei_outb(E8390_PAGE0 + E8390_START + E8390_NODMA, addr + NE_CMD);
    ei_outb(ENISR_RDC, addr + NE_EN0_ISR);
// Now the normal output.
    ei_outb(count & 0xff, addr + NE_EN0_RCNTLO);
    ei_outb(count >> 8, addr + NE_EN0_RCNTHI);
    ei_outb(0x00, addr + NE_EN0_RSARLO);
    ei_outb(start_page, addr + NE_EN0_RSARHI);
    ei_outb(E8390_RWRITE + E8390_START, addr + NE_CMD);
    ei_outsw(addr + NE_DATAPORT, buf, count >> 1);
    dma_start = jiffies;
    while ((ei_inb(addr + NE_EN0_ISR) & ENISR_RDC) == 0) {
    if (time_after(jiffies, dma_start + 2 * HZ / 100)) { /* 20ms */
    netdev_warn(dev, "timeout waiting for Tx RDC\n");
    mcf8390_reset_8390(dev);
    __NS8390_init(dev, 1);
    break;
    }
    }
    ei_outb(ENISR_RDC, addr + NE_EN0_ISR);	/* Ack intr */
    ei_local.dmaing &= ~0x01;
    }
    static const struct net_device_ops mcf8390_netdev_ops = {
    .ndo_open		= __ei_open,
    .ndo_stop		= __ei_close,
    .ndo_start_xmit		= __ei_start_xmit,
    .ndo_tx_timeout		= __ei_tx_timeout,
    .ndo_get_stats		= __ei_get_stats,
    .ndo_set_rx_mode	= __ei_set_multicast_list,
    .ndo_validate_addr	= eth_validate_addr,
    .ndo_set_mac_address	= eth_mac_addr,

    .ndo_poll_controller	= __ei_poll,

    };
#[no_mangle]
unsafe extern "C" fn mcf8390_init(dev: *mut net_device) -> c_int {
    static int mcf8390_init(struct net_device *dev)
    {
    static u32 offsets[] = {
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
    0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
    };
    struct ei_device *ei_local = netdev_priv(dev);
    unsigned char SA_prom[32];
    let mut addr: u32 = dev.base_addr;
    int start_page, stop_page;
    int i, ret;
    mcf8390_reset_8390(dev);
//
// Read the 16 bytes of station address PROM.
// We must first initialize registers,
// similar to NS8390_init(eifdev, 0).
// We can't reliably read the SAPROM address without this.
// (I learned the hard way!).
//
    {
    static const struct {
    u32 value;
    u32 offset;
    } program_seq[] = {
    {E8390_NODMA + E8390_PAGE0 + E8390_STOP, NE_CMD},
// Select page 0
    {0x48,	NE_EN0_DCFG},	/* 0x48: Set byte-wide access */
    {0x00,	NE_EN0_RCNTLO},	/* Clear the count regs */
    {0x00,	NE_EN0_RCNTHI},
    {0x00,	NE_EN0_IMR},	/* Mask completion irq */
    {0xFF,	NE_EN0_ISR},
    {E8390_RXOFF, NE_EN0_RXCR}, /* 0x20 Set to monitor */
    {E8390_TXOFF, NE_EN0_TXCR}, /* 0x02 and loopback mode */
    {32,	NE_EN0_RCNTLO},
    {0x00,	NE_EN0_RCNTHI},
    {0x00,	NE_EN0_RSARLO},	/* DMA starting at 0x0000 */
    {0x00,	NE_EN0_RSARHI},
    {E8390_RREAD + E8390_START, NE_CMD},
    };
    for (i = 0; i < ARRAY_SIZE(program_seq); i++) {
    ei_outb(program_seq[i].value,
    addr + program_seq[i].offset);
    }
    }
    for (i = 0; i < 16; i++) {
    SA_prom[i] = ei_inb(addr + NE_DATAPORT);
    ei_inb(addr + NE_DATAPORT);
    }
// We must set the 8390 for word mode.
    ei_outb(0x49, addr + NE_EN0_DCFG);
    start_page = NESM_START_PG;
    stop_page = NESM_STOP_PG;
// Install the Interrupt handler
    ret = request_irq(dev.irq, __ei_interrupt, 0, dev.name, dev);
    if (ret)
    return ret;
    eth_hw_addr_set(dev, SA_prom);
    netdev_dbg(dev, "Found ethernet address: %pM\n", dev.dev_addr);
    ei_local.name = "mcf8390";
    ei_local.tx_start_page = start_page;
    ei_local.stop_page = stop_page;
    ei_local.word16 = 1;
    ei_local.rx_start_page = start_page + TX_PAGES;
    ei_local.reset_8390 = mcf8390_reset_8390;
    ei_local.block_input = mcf8390_block_input;
    ei_local.block_output = mcf8390_block_output;
    ei_local.get_8390_hdr = mcf8390_get_8390_hdr;
    ei_local.reg_offset = offsets;
    dev.netdev_ops = &mcf8390_netdev_ops;
    __NS8390_init(dev, 0);
    ret = register_netdev(dev);
    if (ret) {
    free_irq(dev.irq, dev);
    return ret;
    }
    netdev_info(dev, "addr=0x%08x irq=%d, Ethernet Address %pM\n",
    addr, dev.irq, dev.dev_addr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mcf8390_probe(pdev: *mut platform_device) -> c_int {
    static int mcf8390_probe(struct platform_device *pdev)
    {
    struct net_device *dev;
    struct resource *mem;
    resource_size_t msize;
    int ret, irq;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return -ENXIO;
    mem = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (mem == core::ptr::null_mut()) {
    dev_err(&pdev.dev, "no memory address specified?\n");
    return -ENXIO;
    }
    msize = resource_size(mem);
    if (!request_mem_region(mem.start, msize, pdev.name))
    return -EBUSY;
    dev = ____alloc_ei_netdev(0);
    if (dev == core::ptr::null_mut()) {
    release_mem_region(mem.start, msize);
    return -ENOMEM;
    }
    SET_NETDEV_DEV(dev, &pdev.dev);
    platform_set_drvdata(pdev, dev);
    dev.irq = irq;
    dev.base_addr = mem.start;
    ret = mcf8390_init(dev);
    if (ret) {
    release_mem_region(mem.start, msize);
    free_netdev(dev);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mcf8390_remove(pdev: *mut platform_device) {
    static void mcf8390_remove(struct platform_device *pdev)
    {
    struct net_device *dev = platform_get_drvdata(pdev);
    struct resource *mem;
    unregister_netdev(dev);
    mem = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    release_mem_region(mem.start, resource_size(mem));
    free_netdev(dev);
    }
    static struct platform_driver mcf8390_drv = {
    .driver = {
    .name	= "mcf8390",
    },
    .probe		= mcf8390_probe,
    .remove		= mcf8390_remove,
    };
    module_platform_driver(mcf8390_drv);
    MODULE_DESCRIPTION("MCF8390 ColdFire NS8390 driver");
    MODULE_AUTHOR("Greg Ungerer <gerg@uclinux.org>");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:mcf8390");
