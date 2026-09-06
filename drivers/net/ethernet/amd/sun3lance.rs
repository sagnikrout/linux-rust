//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/amd/sun3lance.c
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


// sun3lance.c: Ethernet driver for SUN3 Lance chip
//
    Sun3 Lance ethernet driver, by Sam Creasey (sammy@users.qual.net).
    This driver is a part of the linux kernel, and is thus distributed
    under the GNU General Public License.
    The values used in LANCE_OBIO and LANCE_IRQ seem to be empirically
    true for the correct IRQ and address of the lance registers.  They
    have not been widely tested, however.  What we probably need is a
    "proper" way to search for a device in the sun3's prom, but, alas,
    linux has no such thing.
    This driver is largely based on atarilance.c, by Roman Hodek.  Other
    sources of inspiration were the NetBSD sun3 am7990 driver, and the
    linux sparc lance driver (sunlance.c).
    There are more assumptions made throughout this driver, it almost
    certainly still needs work, but it does work at least for RARP/BOOTP and
    mounting the root NFS filesystem.
//
    static const char version[] =
    "sun3lance.c: v1.2 1/12/2001  Sam Creasey (sammy@sammy.net)\n";

// sun3/60 addr/irq for the lance chip.  If your sun is different,
    change this. */
pub const LANCE_OBIO: c_uint = 0x120000;

// Debug level:
// 0 = silent, print only serious errors
// 1 = normal, print error messages
// 2 = debug, print debug infos
// 3 = debug, print even more debug infos (packet data)
//
pub const LANCE_DEBUG: c_int = 0;

    let mut lance_debug: static int = LANCE_DEBUG;

    let mut lance_debug: static int = 1;

    module_param(lance_debug, int, 0);
    MODULE_PARM_DESC(lance_debug, "SUN3 Lance debug level (0-3)");
    MODULE_DESCRIPTION("Sun3/Sun3x on-board LANCE Ethernet driver");
    MODULE_LICENSE("GPL");

    do {  \
    if (lance_debug >= n)  \
    printk a; \
    } while( 0 )
// we're only using 32k of memory, so we use 4 TX
    buffers and 16 RX buffers.  These values are expressed as log2. */
pub const TX_LOG_RING_SIZE: c_int = 3;
pub const RX_LOG_RING_SIZE: c_int = 5;
// These are the derived values

// Definitions for packet buffer access:
pub const PKT_BUF_SZ: c_int = 1544;
// Get the address of a packet buffer corresponding to a given buffer head

// The LANCE Rx and Tx ring descriptors.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lance_rx_head {
    pub /: *mut *mut unsigned short base; / Low word of base addr,
    pub flag: volatile unsigned char,
    pub /: *mut *mut unsigned char base_hi; / High word of base addr (unused),
    pub /: *mut *mut short buf_length; / This length is 2s complement!,
    pub /: *mut *mut volatile short msg_length; / This length is "normal".,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lance_tx_head {
    pub /: *mut *mut unsigned short base; / Low word of base addr,
    pub flag: volatile unsigned char,
    pub /: *mut *mut unsigned char base_hi; / High word of base addr (unused),
    pub /: *mut *mut short length; / Length is 2s complement!,
    pub misc: volatile short,
}

// The LANCE initialization block, described in databook.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lance_init_block {
    pub /: *mut *mut unsigned short mode; / Pre-set mode,
    pub /: *mut *mut unsigned char hwaddr[6]; / Physical ethernet address,
    pub /: *mut *mut unsigned int filter[2]; / Multicast filter (unused).,
// Receive and transmit ring base, along with length bits.
    pub rdra: c_ushort,
    pub rlen: c_ushort,
    pub tdra: c_ushort,
    pub tlen: c_ushort,
    pub /: *mut *mut unsigned short pad[4]; / is thie needed?,
}

// The whole layout of the Lance shared memory
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lance_memory {
    pub init: lance_init_block,
    pub tx_head: [lance_tx_head; TX_RING_SIZE],
    pub rx_head: [lance_rx_head; RX_RING_SIZE],
    pub rx_data: [c_char; RX_RING_SIZE][PKT_BUF_SZ],
    pub tx_data: [c_char; TX_RING_SIZE][PKT_BUF_SZ],
}

// The driver's private device structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lance_private {
    pub iobase: *mut volatile unsigned short,
    pub mem: *mut lance_memory,
    pub /: *mut *mut int new_rx, new_tx; / The next free ring entry,
    pub /: *mut *mut int old_tx, old_rx; / ring entry to be processed,
// These two must be longs for set_bit()
    pub tx_full: c_long,
    pub lock: c_long,
}

// I/O register access macros

// Definitions for the Lance
// tx_head flags
pub const TMD1_ENP: c_uint = 0x01	/* end of packet */;
pub const TMD1_STP: c_uint = 0x02	/* start of packet */;
pub const TMD1_DEF: c_uint = 0x04	/* deferred */;
pub const TMD1_ONE: c_uint = 0x08	/* one retry needed */;
pub const TMD1_MORE: c_uint = 0x10	/* more than one retry needed */;
pub const TMD1_ERR: c_uint = 0x40	/* error summary */;
pub const TMD1_OWN: c_uint = 0x80	/* ownership (set: chip owns) */;

pub const TMD1_OWN_HOST: c_int = 0;
// tx_head misc field
pub const TMD3_TDR: c_uint = 0x03FF	/* Time Domain Reflectometry counter */;
pub const TMD3_RTRY: c_uint = 0x0400	/* failed after 16 retries */;
pub const TMD3_LCAR: c_uint = 0x0800	/* carrier lost */;
pub const TMD3_LCOL: c_uint = 0x1000	/* late collision */;
pub const TMD3_UFLO: c_uint = 0x4000	/* underflow (late memory) */;
pub const TMD3_BUFF: c_uint = 0x8000	/* buffering error (no ENP) */;
// rx_head flags
pub const RMD1_ENP: c_uint = 0x01	/* end of packet */;
pub const RMD1_STP: c_uint = 0x02	/* start of packet */;
pub const RMD1_BUFF: c_uint = 0x04	/* buffer error */;
pub const RMD1_CRC: c_uint = 0x08	/* CRC error */;
pub const RMD1_OFLO: c_uint = 0x10	/* overflow */;
pub const RMD1_FRAM: c_uint = 0x20	/* framing error */;
pub const RMD1_ERR: c_uint = 0x40	/* error summary */;
pub const RMD1_OWN: c_uint = 0x80	/* ownership (set: ship owns) */;

pub const RMD1_OWN_HOST: c_int = 0;
// register names

// CSR0
// (R=readable, W=writeable, S=set on write, C=clear on write)
pub const CSR0_INIT: c_uint = 0x0001		/* initialize (RS) */;
pub const CSR0_STRT: c_uint = 0x0002		/* start (RS) */;
pub const CSR0_STOP: c_uint = 0x0004		/* stop (RS) */;
pub const CSR0_TDMD: c_uint = 0x0008		/* transmit demand (RS) */;
pub const CSR0_TXON: c_uint = 0x0010		/* transmitter on (R) */;
pub const CSR0_RXON: c_uint = 0x0020		/* receiver on (R) */;
pub const CSR0_INEA: c_uint = 0x0040		/* interrupt enable (RW) */;
pub const CSR0_INTR: c_uint = 0x0080		/* interrupt active (R) */;
pub const CSR0_IDON: c_uint = 0x0100		/* initialization done (RC) */;
pub const CSR0_TINT: c_uint = 0x0200		/* transmitter interrupt (RC) */;
pub const CSR0_RINT: c_uint = 0x0400		/* receiver interrupt (RC) */;
pub const CSR0_MERR: c_uint = 0x0800		/* memory error (RC) */;
pub const CSR0_MISS: c_uint = 0x1000		/* missed frame (RC) */;
pub const CSR0_CERR: c_uint = 0x2000		/* carrier error (no heartbeat :-) (RC) */;
pub const CSR0_BABL: c_uint = 0x4000		/* babble: tx-ed too many bits (RC) */;
pub const CSR0_ERR: c_uint = 0x8000		/* error (RC) */;
// CSR3
pub const CSR3_BCON: c_uint = 0x0001		/* byte control */;
pub const CSR3_ACON: c_uint = 0x0002		/* ALE control */;
pub const CSR3_BSWP: c_uint = 0x0004		/* byte swap (1=big endian) */;
// Prototypes
    static int lance_probe( struct net_device *dev);
    static int lance_open( struct net_device *dev );
    static void lance_init_ring( struct net_device *dev );
    static netdev_tx_t lance_start_xmit(struct sk_buff *skb,
    struct net_device *dev);
    static irqreturn_t lance_interrupt( int irq, void *dev_id);
    static int lance_rx( struct net_device *dev );
    static int lance_close( struct net_device *dev );
    static void set_multicast_list( struct net_device *dev );
// End of Prototypes
#[no_mangle]
unsafe extern "C" fn sun3lance_probe() -> *mut net_device  __init {
    static struct net_device * __init sun3lance_probe(void)
    {
    struct net_device *dev;
    static int found;
    let mut err: c_int = -ENODEV;
    if (!MACH_IS_SUN3 && !MACH_IS_SUN3X)
    return ERR_PTR(-ENODEV);
// check that this machine has an onboard lance
    switch(idprom.id_machtype) {
    case SM_SUN3|SM_3_50:
    case SM_SUN3|SM_3_60:
    case SM_SUN3X|SM_3_80:
// these machines have lance
    break;
    default:
    return ERR_PTR(-ENODEV);
    }
    if (found)
    return ERR_PTR(-ENODEV);
    dev = alloc_etherdev(sizeof(struct lance_private));
    if (!dev)
    return ERR_PTR(-ENOMEM);
    if (!lance_probe(dev))
    goto out;
    err = register_netdev(dev);
    if (err)
    goto out1;
    found = 1;
    return dev;
    out1:

    iounmap((void __iomem *)dev.base_addr);

    out:
    free_netdev(dev);
    return ERR_PTR(err);
    }
    static const struct net_device_ops lance_netdev_ops = {
    .ndo_open		= lance_open,
    .ndo_stop		= lance_close,
    .ndo_start_xmit		= lance_start_xmit,
    .ndo_set_rx_mode	= set_multicast_list,
    .ndo_set_mac_address	= core::ptr::null_mut(),
    .ndo_validate_addr	= eth_validate_addr,
    };
#[no_mangle]
unsafe extern "C" fn lance_probe(dev: *mut net_device) -> int __init {
    static int __init lance_probe( struct net_device *dev)
    {
    unsigned long ioaddr;
    struct lance_private	*lp;
    static int 		did_version;
    volatile unsigned short *ioaddr_probe;
    unsigned short tmp1, tmp2;

    ioaddr = (unsigned long)ioremap(LANCE_OBIO, PAGE_SIZE);
    if (!ioaddr)
    return 0;

    ioaddr = SUN3X_LANCE;

// test to see if there's really a lance here
// (CSRO_INIT shouldn't be readable)
    ioaddr_probe = (volatile unsigned short *)ioaddr;
    tmp1 = ioaddr_probe[0];
    tmp2 = ioaddr_probe[1];
    ioaddr_probe[1] = CSR0;
    ioaddr_probe[0] = CSR0_INIT | CSR0_STOP;
    if(ioaddr_probe[0] != CSR0_STOP) {
    ioaddr_probe[0] = tmp1;
    ioaddr_probe[1] = tmp2;

    iounmap((void __iomem *)ioaddr);

    return 0;
    }
    lp = netdev_priv(dev);
// XXX - leak?
    MEM = dvma_malloc_align(sizeof(struct lance_memory), 0x10000);
    if (!MEM) {

    iounmap((void __iomem *)ioaddr);

    printk(KERN_WARNING "SUN3 Lance couldn't allocate DVMA memory\n");
    return 0;
    }
    lp.iobase = (volatile unsigned short *)ioaddr;
    dev.base_addr = (unsigned long)ioaddr; /* informational only */
    REGA(CSR0) = CSR0_STOP;
    if (request_irq(LANCE_IRQ, lance_interrupt, 0, "SUN3 Lance", dev) < 0) {

    iounmap((void __iomem *)ioaddr);

    dvma_free((void *)MEM);
    printk(KERN_WARNING "SUN3 Lance unable to allocate IRQ\n");
    return 0;
    }
    dev.irq = (unsigned short)LANCE_IRQ;
    printk("%s: SUN3 Lance at io %#lx, mem %#lx, irq %d, hwaddr ",
    dev.name,
    (unsigned long)ioaddr,
    (unsigned long)MEM,
    dev.irq);
// copy in the ethernet address from the prom
    eth_hw_addr_set(dev, idprom.id_ethaddr);
// tell the card it's ether address, bytes swapped
    MEM.init.hwaddr[0] = dev.dev_addr[1];
    MEM.init.hwaddr[1] = dev.dev_addr[0];
    MEM.init.hwaddr[2] = dev.dev_addr[3];
    MEM.init.hwaddr[3] = dev.dev_addr[2];
    MEM.init.hwaddr[4] = dev.dev_addr[5];
    MEM.init.hwaddr[5] = dev.dev_addr[4];
    printk("%pM\n", dev.dev_addr);
    MEM.init.mode = 0x0000;
    MEM.init.filter[0] = 0x00000000;
    MEM.init.filter[1] = 0x00000000;
    MEM.init.rdra = dvma_vtob(MEM.rx_head);
    MEM.init.rlen    = (RX_LOG_RING_SIZE << 13) |
    (dvma_vtob(MEM.rx_head) >> 16);
    MEM.init.tdra = dvma_vtob(MEM.tx_head);
    MEM.init.tlen    = (TX_LOG_RING_SIZE << 13) |
    (dvma_vtob(MEM.tx_head) >> 16);
    DPRINTK(2, ("initaddr: %08lx rx_ring: %08lx tx_ring: %08lx\n",
    dvma_vtob(&(MEM.init)), dvma_vtob(MEM.rx_head),
    (dvma_vtob(MEM.tx_head))));
    if (did_version++ == 0)
    printk( version );
    dev.netdev_ops = &lance_netdev_ops;
// KLUDGE -- REMOVE ME
    set_bit(__LINK_STATE_PRESENT, &dev.state);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn lance_open(dev: *mut net_device) -> c_int {
    static int lance_open( struct net_device *dev )
    {
    struct lance_private *lp = netdev_priv(dev);
    int i;
    DPRINTK( 2, ( "%s: lance_open()\n", dev.name ));
    REGA(CSR0) = CSR0_STOP;
    lance_init_ring(dev);
// From now on, AREG is kept to point to CSR0
    REGA(CSR0) = CSR0_INIT;
    i = 1000000;
    while (--i > 0)
    if (DREG & CSR0_IDON)
    break;
    if (i <= 0 || (DREG & CSR0_ERR)) {
    DPRINTK( 2, ( "lance_open(): opening %s failed, i=%d, csr0=%04x\n",
    dev.name, i, DREG ));
    DREG = CSR0_STOP;
    return -EIO;
    }
    DREG = CSR0_IDON | CSR0_STRT | CSR0_INEA;
    netif_start_queue(dev);
    DPRINTK( 2, ( "%s: LANCE is open, csr0 %04x\n", dev.name, DREG ));
    return 0;
    }
// Initialize the LANCE Rx and Tx rings.
#[no_mangle]
unsafe extern "C" fn lance_init_ring(dev: *mut net_device) {
    static void lance_init_ring( struct net_device *dev )
    {
    struct lance_private *lp = netdev_priv(dev);
    int i;
    lp.lock = 0;
    lp.tx_full = 0;
    lp.new_rx = lp.new_tx = 0;
    lp.old_rx = lp.old_tx = 0;
    for( i = 0; i < TX_RING_SIZE; i++ ) {
    MEM.tx_head[i].base = dvma_vtob(MEM.tx_data[i]);
    MEM.tx_head[i].flag = 0;
    MEM.tx_head[i].base_hi =
    (dvma_vtob(MEM.tx_data[i])) >>16;
    MEM.tx_head[i].length = 0;
    MEM.tx_head[i].misc = 0;
    }
    for( i = 0; i < RX_RING_SIZE; i++ ) {
    MEM.rx_head[i].base = dvma_vtob(MEM.rx_data[i]);
    MEM.rx_head[i].flag = RMD1_OWN_CHIP;
    MEM.rx_head[i].base_hi =
    (dvma_vtob(MEM.rx_data[i])) >> 16;
    MEM.rx_head[i].buf_length = -PKT_BUF_SZ | 0xf000;
    MEM.rx_head[i].msg_length = 0;
    }
// tell the card it's ether address, bytes swapped
    MEM.init.hwaddr[0] = dev.dev_addr[1];
    MEM.init.hwaddr[1] = dev.dev_addr[0];
    MEM.init.hwaddr[2] = dev.dev_addr[3];
    MEM.init.hwaddr[3] = dev.dev_addr[2];
    MEM.init.hwaddr[4] = dev.dev_addr[5];
    MEM.init.hwaddr[5] = dev.dev_addr[4];
    MEM.init.mode = 0x0000;
    MEM.init.filter[0] = 0x00000000;
    MEM.init.filter[1] = 0x00000000;
    MEM.init.rdra = dvma_vtob(MEM.rx_head);
    MEM.init.rlen    = (RX_LOG_RING_SIZE << 13) |
    (dvma_vtob(MEM.rx_head) >> 16);
    MEM.init.tdra = dvma_vtob(MEM.tx_head);
    MEM.init.tlen    = (TX_LOG_RING_SIZE << 13) |
    (dvma_vtob(MEM.tx_head) >> 16);
// tell the lance the address of its init block
    REGA(CSR1) = dvma_vtob(&(MEM.init));
    REGA(CSR2) = dvma_vtob(&(MEM.init)) >> 16;

    REGA(CSR3) = CSR3_BSWP | CSR3_ACON | CSR3_BCON;

    REGA(CSR3) = CSR3_BSWP;

    }
    static netdev_tx_t
    lance_start_xmit(struct sk_buff *skb, struct net_device *dev)
    {
    struct lance_private *lp = netdev_priv(dev);
    int entry, len;
    struct lance_tx_head *head;
    unsigned long flags;
    DPRINTK( 1, ( "%s: transmit start.\n",
    dev.name));
// Transmitter timeout, serious problems.
    if (netif_queue_stopped(dev)) {
    let mut tickssofar: c_int = jiffies - dev_trans_start(dev);
    if (tickssofar < HZ/5)
    return NETDEV_TX_BUSY;
    DPRINTK( 1, ( "%s: transmit timed out, status %04x, resetting.\n",
    dev.name, DREG ));
    DREG = CSR0_STOP;
//
// Always set BSWP after a STOP as STOP puts it back into
// little endian mode.
//
    REGA(CSR3) = CSR3_BSWP;
    dev.stats.tx_errors++;
    if(lance_debug >= 2) {
    int i;
    printk("Ring data: old_tx %d new_tx %d%s new_rx %d\n",
    lp.old_tx, lp.new_tx,
    lp.tx_full ? " (full)" : "",
    lp.new_rx );
    for( i = 0 ; i < RX_RING_SIZE; i++ )
    printk( "rx #%d: base=%04x blen=%04x mlen=%04x\n",
    i, MEM.rx_head[i].base,
    -MEM.rx_head[i].buf_length,
    MEM.rx_head[i].msg_length);
    for( i = 0 ; i < TX_RING_SIZE; i++ )
    printk("tx #%d: base=%04x len=%04x misc=%04x\n",
    i, MEM.tx_head[i].base,
    -MEM.tx_head[i].length,
    MEM.tx_head[i].misc );
    }
    lance_init_ring(dev);
    REGA( CSR0 ) = CSR0_INEA | CSR0_INIT | CSR0_STRT;
    netif_start_queue(dev);
    return NETDEV_TX_OK;
    }
// Block a timer-based transmit from overlapping.  This could better be
    done with atomic_swap(1, dev.tbusy), but set_bit() works as well. */
// Block a timer-based transmit from overlapping with us by
    stopping the queue for a bit... */
    netif_stop_queue(dev);
    if (test_and_set_bit( 0, (void*)&lp.lock ) != 0) {
    printk( "%s: tx queue lock!.\n", dev.name);
// don't clear dev->tbusy flag.
    return NETDEV_TX_BUSY;
    }
    AREG = CSR0;
    DPRINTK( 2, ( "%s: lance_start_xmit() called, csr0 %4.4x.\n",
    dev.name, DREG ));

// this weirdness doesn't appear on sun3...
    if(!(DREG & CSR0_INIT)) {
    DPRINTK( 1, ("INIT not set, reinitializing...\n"));
    REGA( CSR0 ) = CSR0_STOP;
    lance_init_ring(dev);
    REGA( CSR0 ) = CSR0_INIT | CSR0_STRT;
    }

// Fill in a Tx ring entry

    if (lance_debug >= 2) {
    printk( "%s: TX pkt %d type 0x%04x"
    " from %s to %s"
    " data at 0x%08x len %d\n",
    dev.name, lp.new_tx, ((u_short *)skb.data)[6],
    DEV_ADDR(&skb.data[6]), DEV_ADDR(skb.data),
    (int)skb.data, (int)skb.len );
    }

// We're not prepared for the int until the last flags are set/reset.
// And the int may happen already after setting the OWN_CHIP...
    local_irq_save(flags);
// Mask to ring buffer boundary.
    entry = lp.new_tx;
    head  = &(MEM.tx_head[entry]);
// Caution: the write order is important here, set the "ownership" bits
// last.
//
// the sun3's lance needs it's buffer padded to the minimum
    size */
    len = (ETH_ZLEN < skb.len) ? skb.len : ETH_ZLEN;
// head->length = -len;
    head.length = (-len) | 0xf000;
    head.misc = 0;
    skb_copy_from_linear_data(skb, PKTBUF_ADDR(head), skb.len);
    if (len != skb.len)
    memset(PKTBUF_ADDR(head) + skb.len, 0, len-skb.len);
    head.flag = TMD1_OWN_CHIP | TMD1_ENP | TMD1_STP;
    lp.new_tx = (lp.new_tx + 1) & TX_RING_MOD_MASK;
    dev.stats.tx_bytes += skb.len;
// Trigger an immediate send poll.
    REGA(CSR0) = CSR0_INEA | CSR0_TDMD | CSR0_STRT;
    AREG = CSR0;
    DPRINTK( 2, ( "%s: lance_start_xmit() exiting, csr0 %4.4x.\n",
    dev.name, DREG ));
    dev_kfree_skb(skb);
    lp.lock = 0;
    if ((MEM.tx_head[(entry+1) & TX_RING_MOD_MASK].flag & TMD1_OWN) ==
    TMD1_OWN_HOST)
    netif_start_queue(dev);
    local_irq_restore(flags);
    return NETDEV_TX_OK;
    }
// The LANCE interrupt handler.
#[no_mangle]
unsafe extern "C" fn lance_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t lance_interrupt( int irq, void *dev_id)
    {
    struct net_device *dev = dev_id;
    struct lance_private *lp = netdev_priv(dev);
    int csr0;
    still_more:
    flush_cache_all();
    AREG = CSR0;
    csr0 = DREG;
// ack interrupts
    DREG = csr0 & (CSR0_TINT | CSR0_RINT | CSR0_IDON);
// clear errors
    if(csr0 & CSR0_ERR)
    DREG = CSR0_BABL | CSR0_MERR | CSR0_CERR | CSR0_MISS;
    DPRINTK( 2, ( "%s: interrupt  csr0=%04x new csr=%04x.\n",
    dev.name, csr0, DREG ));
    if (csr0 & CSR0_TINT) {			/* Tx-done interrupt */
    let mut old_tx: c_int = lp.old_tx;
// if(lance_debug >= 3) {
// int i;
//
// printk("%s: tx int\n", dev->name);
//
// for(i = 0; i < TX_RING_SIZE; i++)
// printk("ring %d flag=%04x\n", i,
// MEM->tx_head[i].flag);
// }
    while( old_tx != lp.new_tx) {
    struct lance_tx_head *head = &(MEM.tx_head[old_tx]);
    DPRINTK(3, ("on tx_ring %d\n", old_tx));
    if (head.flag & TMD1_OWN_CHIP)
    break; /* It still hasn't been Txed */
    if (head.flag & TMD1_ERR) {
    let mut status: c_int = head.misc;
    dev.stats.tx_errors++;
    if (status & TMD3_RTRY) dev.stats.tx_aborted_errors++;
    if (status & TMD3_LCAR) dev.stats.tx_carrier_errors++;
    if (status & TMD3_LCOL) dev.stats.tx_window_errors++;
    if (status & (TMD3_UFLO | TMD3_BUFF)) {
    dev.stats.tx_fifo_errors++;
    printk("%s: Tx FIFO error\n",
    dev.name);
    REGA(CSR0) = CSR0_STOP;
    REGA(CSR3) = CSR3_BSWP;
    lance_init_ring(dev);
    REGA(CSR0) = CSR0_STRT | CSR0_INEA;
    return IRQ_HANDLED;
    }
    } else if(head.flag & (TMD1_ENP | TMD1_STP)) {
    head.flag &= ~(TMD1_ENP | TMD1_STP);
    if(head.flag & (TMD1_ONE | TMD1_MORE))
    dev.stats.collisions++;
    dev.stats.tx_packets++;
    DPRINTK(3, ("cleared tx ring %d\n", old_tx));
    }
    old_tx = (old_tx +1) & TX_RING_MOD_MASK;
    }
    lp.old_tx = old_tx;
    }
    if (netif_queue_stopped(dev)) {
// The ring is no longer full, clear tbusy.
    netif_start_queue(dev);
    netif_wake_queue(dev);
    }
    if (csr0 & CSR0_RINT)			/* Rx interrupt */
    lance_rx( dev );
// Log misc errors.
    if (csr0 & CSR0_BABL) dev.stats.tx_errors++; /* Tx babble. */
    if (csr0 & CSR0_MISS) dev.stats.rx_errors++; /* Missed a Rx frame. */
    if (csr0 & CSR0_MERR) {
    DPRINTK( 1, ( "%s: Bus master arbitration failure (?!?), "
    "status %04x.\n", dev.name, csr0 ));
// Restart the chip.
    REGA(CSR0) = CSR0_STOP;
    REGA(CSR3) = CSR3_BSWP;
    lance_init_ring(dev);
    REGA(CSR0) = CSR0_STRT | CSR0_INEA;
    }
// Clear any other interrupt, and set interrupt enable.
// DREG = CSR0_BABL | CSR0_CERR | CSR0_MISS | CSR0_MERR |
// CSR0_IDON | CSR0_INEA;
    REGA(CSR0) = CSR0_INEA;
    if(DREG & (CSR0_RINT | CSR0_TINT)) {
    DPRINTK(2, ("restarting interrupt, csr0=%#04x\n", DREG));
    goto still_more;
    }
    DPRINTK( 2, ( "%s: exiting interrupt, csr0=%#04x.\n",
    dev.name, DREG ));
    return IRQ_HANDLED;
    }
// get packet, toss into skbuff
#[no_mangle]
unsafe extern "C" fn lance_rx(dev: *mut net_device) -> c_int {
    static int lance_rx( struct net_device *dev )
    {
    struct lance_private *lp = netdev_priv(dev);
    let mut entry: c_int = lp.new_rx;
// If we own the next entry, it's a new packet. Send it up.
    while( (MEM.rx_head[entry].flag & RMD1_OWN) == RMD1_OWN_HOST ) {
    struct lance_rx_head *head = &(MEM.rx_head[entry]);
    let mut status: c_int = head.flag;
    if (status != (RMD1_ENP|RMD1_STP)) {  /* There was an error. */
// There is a tricky error noted by John Murphy,
    <murf@perftech.com> to Russ Nelson: Even with
    full-sized buffers it's possible for a jabber packet to use two
    buffers, with only the last correctly noting the error. */
    if (status & RMD1_ENP)	/* Only count a general error at the */
    dev.stats.rx_errors++; /* end of a packet.*/
    if (status & RMD1_FRAM) dev.stats.rx_frame_errors++;
    if (status & RMD1_OFLO) dev.stats.rx_over_errors++;
    if (status & RMD1_CRC) dev.stats.rx_crc_errors++;
    if (status & RMD1_BUFF) dev.stats.rx_fifo_errors++;
    head.flag &= (RMD1_ENP|RMD1_STP);
    } else {
// Malloc up new buffer, compatible with net-3.
// short pkt_len = head->msg_length;// & 0xfff;
    let mut pkt_len: c_short = (head.msg_length & 0xfff) - 4;
    struct sk_buff *skb;
    if (pkt_len < 60) {
    printk( "%s: Runt packet!\n", dev.name );
    dev.stats.rx_errors++;
    }
    else {
    skb = netdev_alloc_skb(dev, pkt_len + 2);
    if (!skb) {
    dev.stats.rx_dropped++;
    head.msg_length = 0;
    head.flag |= RMD1_OWN_CHIP;
    lp.new_rx = (lp.new_rx+1) &
    RX_RING_MOD_MASK;
    }

    if (lance_debug >= 3) {
    u_char *data = PKTBUF_ADDR(head);
    printk("%s: RX pkt %d type 0x%04x"
    " from %pM to %pM",
    dev.name, lp.new_tx, ((u_short *)data)[6],
    &data[6], data);
    printk(" data %02x %02x %02x %02x %02x %02x %02x %02x "
    "len %d at %08x\n",
    data[15], data[16], data[17], data[18],
    data[19], data[20], data[21], data[22],
    pkt_len, data);
    }

    if (lance_debug >= 3) {
    u_char *data = PKTBUF_ADDR(head);
    printk( "%s: RX pkt %d type 0x%04x len %d\n ", dev.name, entry, ((u_short *)data)[6], pkt_len);
    }
    skb_reserve( skb, 2 );	/* 16 byte align */
    skb_put( skb, pkt_len );	/* Make room */
    skb_copy_to_linear_data(skb,
    PKTBUF_ADDR(head),
    pkt_len);
    skb.protocol = eth_type_trans( skb, dev );
    netif_rx( skb );
    dev.stats.rx_packets++;
    dev.stats.rx_bytes += pkt_len;
    }
    }
// head->buf_length = -PKT_BUF_SZ | 0xf000;
    head.msg_length = 0;
    head.flag = RMD1_OWN_CHIP;
    entry = lp.new_rx = (lp.new_rx +1) & RX_RING_MOD_MASK;
    }
// From lance.c (Donald Becker):
// We should check that at least two ring entries are free.
    If not, we should free one and mark stats.rx_dropped++. */
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lance_close(dev: *mut net_device) -> c_int {
    static int lance_close( struct net_device *dev )
    {
    struct lance_private *lp = netdev_priv(dev);
    netif_stop_queue(dev);
    AREG = CSR0;
    DPRINTK( 2, ( "%s: Shutting down ethercard, status was %2.2x.\n",
    dev.name, DREG ));
// We stop the LANCE here -- it occasionally polls
    memory if we don't. */
    DREG = CSR0_STOP;
    return 0;
    }
// Set or clear the multicast filter for this adaptor.
    num_addrs == -1		Promiscuous mode, receive all packets
    num_addrs == 0		Normal mode, clear multicast list
    num_addrs > 0		Multicast mode, receive normal and MC packets, and do
    best-effort filtering.
//
// completely untested on a sun3
#[no_mangle]
unsafe extern "C" fn set_multicast_list(dev: *mut net_device) {
    static void set_multicast_list( struct net_device *dev )
    {
    struct lance_private *lp = netdev_priv(dev);
    if(netif_queue_stopped(dev))
// Only possible if board is already started
    return;
// We take the simple way out and always enable promiscuous mode.
    DREG = CSR0_STOP; /* Temporarily stop the lance. */
    if (dev.flags & IFF_PROMISC) {
// Log any net taps.
    DPRINTK( 3, ( "%s: Promiscuous mode enabled.\n", dev.name ));
    REGA( CSR15 ) = 0x8000; /* Set promiscuous mode */
    } else {
    short multicast_table[4];
    let mut num_addrs: c_int = netdev_mc_count(dev);
    int i;
// We don't use the multicast table, but rely on upper-layer
// filtering.
    memset( multicast_table, (num_addrs == 0) ? 0 : -1,
    sizeof(multicast_table) );
    for( i = 0; i < 4; i++ )
    REGA( CSR8+i ) = multicast_table[i];
    REGA( CSR15 ) = 0; /* Unset promiscuous mode */
    }
//
// Always set BSWP after a STOP as STOP puts it back into
// little endian mode.
//
    REGA( CSR3 ) = CSR3_BSWP;
// Resume normal operation and reset AREG to CSR0
    REGA( CSR0 ) = CSR0_IDON | CSR0_INEA | CSR0_STRT;
    }
    static struct net_device *sun3lance_dev;
#[no_mangle]
unsafe extern "C" fn sun3lance_init() -> int __init {
    static int __init sun3lance_init(void)
    {
    sun3lance_dev = sun3lance_probe();
    return PTR_ERR_OR_ZERO(sun3lance_dev);
    }
    module_init(sun3lance_init);
#[no_mangle]
unsafe extern "C" fn sun3lance_cleanup() -> void __exit {
    static void __exit sun3lance_cleanup(void)
    {
    unregister_netdev(sun3lance_dev);

    iounmap((void __iomem *)sun3lance_dev.base_addr);

    free_netdev(sun3lance_dev);
    }
    module_exit(sun3lance_cleanup);
