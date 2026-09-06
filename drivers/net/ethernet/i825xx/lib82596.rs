//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/i825xx/lib82596.c
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


// SPDX-License-Identifier: GPL-1.0+
// lasi_82596.c -- driver for the intel 82596 ethernet controller, as
    munged into HPPA boxen .
    This driver is based upon 82596.c, original credits are below...
    but there were too many hoops which HP wants jumped through to
    keep this code in there in a sane manner.
    3 primary sources of the mess --
    1) hppa needs *lots* of cacheline flushing to keep this kind of
    MMIO running.
    2) The 82596 needs to see all of its pointers as their physical
    address.  Thus virt_to_bus/bus_to_virt are *everywhere*.
    3) The implementation HP is using seems to be significantly pickier
    about when and how the command and RX units are started.  some
    command ordering was changed.
    Examination of the mach driver leads one to believe that there
    might be a saner way to pull this off...  anyone who feels like a
    full rewrite can be my guest.
    Split 02/13/2000 Sam Creasey (sammy@oh.verio.com)
    02/01/2000  Initial modifications for parisc by Helge Deller (deller@gmx.de)
    03/02/2000  changes for better/correct(?) cache-flushing (deller)
//
// 82596.c: A generic 82596 ethernet driver for linux.
//
    Based on Apricot.c
    Written 1994 by Mark Evans.
    This driver is for the Apricot 82596 bus-master interface
    Modularised 12/94 Mark Evans
    Modified to support the 82596 ethernet chips on 680x0 VME boards.
    by Richard Hirst <richard@sleepie.demon.co.uk>
    Renamed to be 82596.c
    980825:  Changed to receive directly in to sk_buffs which are
    allocated at open() time.  Eliminates copy on incoming frames
    (small ones are still copied).  Shared data now held in a
    non-cached page, so we can run on 68060 in copyback mode.
    TBD:
// look at deferring rx frames rather than discarding (as per tulip)
// handle tx ring full as per tulip
// performance test to tune rx_copybreak
    Most of my modifications relate to the braindead big-endian
    implementation by Intel.  When the i596 is operating in
    'big-endian' mode, it thinks a 32 bit value of 0x12345678
    should be stored as 0x56781234.  This is a real pain, when
    you have linked lists which are shared by the 680x0 and the
    i596.
    Driver skeleton
    Written 1993 by Donald Becker.
    Copyright 1993 United States Government as represented by the Director,
    National Security Agency.
    The author may be reached as becker@scyld.com, or C/O
    Scyld Computing Corporation, 410 Severn Ave., Suite 210, Annapolis MD 21403
//

// DEBUG flags
//
pub const DEB_INIT: c_uint = 0x0001;
pub const DEB_PROBE: c_uint = 0x0002;
pub const DEB_SERIOUS: c_uint = 0x0004;
pub const DEB_ERRORS: c_uint = 0x0008;
pub const DEB_MULTI: c_uint = 0x0010;
pub const DEB_TDR: c_uint = 0x0020;
pub const DEB_OPEN: c_uint = 0x0040;
pub const DEB_RESET: c_uint = 0x0080;
pub const DEB_ADDCMD: c_uint = 0x0100;
pub const DEB_STATUS: c_uint = 0x0200;
pub const DEB_STARTTX: c_uint = 0x0400;
pub const DEB_RXADDR: c_uint = 0x0800;
pub const DEB_TXADDR: c_uint = 0x1000;
pub const DEB_RXFRAME: c_uint = 0x2000;
pub const DEB_INTS: c_uint = 0x4000;
pub const DEB_STRUCT: c_uint = 0x8000;
pub const DEB_ANY: c_uint = 0xffff;

//
// The MPU_PORT command allows direct access to the 82596. With PORT access
// the following commands are available (p5-18). The 32-bit port command
// must be word-swapped with the most significant word written first.
// This only applies to VME boards.
//
pub const PORT_RESET: c_uint = 0x00	/* reset 82596 */;
pub const PORT_SELFTEST: c_uint = 0x01	/* selftest */;
pub const PORT_ALTSCP: c_uint = 0x02	/* alternate SCB address */;
pub const PORT_ALTDUMP: c_uint = 0x03	/* Alternate DUMP address */;
    let mut i596_debug: static int = (DEB_SERIOUS|DEB_PROBE);
// Copy frames shorter than rx_copybreak, otherwise pass on up in
// a full sized sk_buff.  Value of 100 stolen from tulip.c (!alpha).
//
    let mut rx_copybreak: static int = 100;
pub const PKT_BUF_SZ: c_int = 1536;
pub const MAX_MC_CNT: c_int = 64;
pub const ISCP_BUSY: c_uint = 0x0001;

pub const CMD_EOL: c_uint = 0x8000	/* The last command of the list, stop. */;
pub const CMD_SUSP: c_uint = 0x4000	/* Suspend after doing cmd. */;
pub const CMD_INTR: c_uint = 0x2000	/* Interrupt after doing cmd. */;
pub const CMD_FLEX: c_uint = 0x0008	/* Enable flexible memory model */;
    enum commands {
    CmdNOp = 0, CmdSASetup = 1, CmdConfigure = 2, CmdMulticastList = 3,
    CmdTx = 4, CmdTDR = 5, CmdDump = 6, CmdDiagnose = 7
    };
pub const STAT_C: c_uint = 0x8000	/* Set to 0 after execution */;
pub const STAT_B: c_uint = 0x4000	/* Command being executed */;
pub const STAT_OK: c_uint = 0x2000	/* Command executed ok */;
pub const STAT_A: c_uint = 0x1000	/* Command aborted */;
pub const CUC_START: c_uint = 0x0100;
pub const CUC_RESUME: c_uint = 0x0200;
pub const CUC_SUSPEND: c_uint = 0x0300;
pub const CUC_ABORT: c_uint = 0x0400;
pub const RX_START: c_uint = 0x0010;
pub const RX_RESUME: c_uint = 0x0020;
pub const RX_SUSPEND: c_uint = 0x0030;
pub const RX_ABORT: c_uint = 0x0040;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i596_reg {
    pub porthi: c_ushort,
    pub portlo: c_ushort,
    pub ca: u32,
}

pub const EOF: c_uint = 0x8000;
pub const SIZE_MASK: c_uint = 0x3fff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i596_tbd {
    pub size: c_ushort,
    pub pad: c_ushort,
    pub next: u32,
    pub data: u32,
    pub /: *mut *mut u32 cache_pad[5]; / Total 32 bytes...,
}

// The command structure has two 'next' pointers; v_next is the address of
// the next command as seen by the CPU, b_next is the address of the next
// command as seen by the 82596.  The b_next pointer, as used by the 82596
// always references the status field of the next command, rather than the
// v_next field, because the 82596 is unaware of v_next.  It may seem more
// logical to put v_next at the end of the structure, but we cannot do that
// because the 82596 expects other fields to be there, depending on command
// type.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i596_cmd {
    pub /: *mut *mut *mut i596_cmd v_next; / Address from CPUs viewpoint,
    pub status: c_ushort,
    pub command: c_ushort,
    pub /: *mut *mut u32 b_next; / Address from i596 viewpoint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_cmd {
    pub cmd: i596_cmd,
    pub tbd: u32,
    pub size: c_ushort,
    pub pad: c_ushort,
    pub /: *mut *mut *mut sk_buff skb; / So we can free it after tx,
    pub dma_addr: dma_addr_t,

    pub /: *mut *mut u32 cache_pad[6]; / Total 64 bytes...,

    pub /: *mut *mut u32 cache_pad[1]; / Total 32 bytes...,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tdr_cmd {
    pub cmd: i596_cmd,
    pub status: c_ushort,
    pub pad: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mc_cmd {
    pub cmd: i596_cmd,
    pub mc_cnt: c_short,
    pub mc_addrs: [*mut c_char; MAX_MC_CNT*6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sa_cmd {
    pub cmd: i596_cmd,
    pub eth_addr: [c_char; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cf_cmd {
    pub cmd: i596_cmd,
    pub i596_config: [c_char; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i596_rfd {
    pub stat: c_ushort,
    pub cmd: c_ushort,
    pub /: *mut *mut u32 b_next; / Address from i596 viewpoint,
    pub rbd: u32,
    pub count: c_ushort,
    pub size: c_ushort,
    pub /: *mut *mut *mut i596_rfd v_next; / Address from CPUs viewpoint,
    pub v_prev: *mut i596_rfd,

    pub /: *mut *mut u32 cache_pad[2]; / Total 32 bytes...,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i596_rbd {
// hardware data
    pub count: c_ushort,
    pub zero1: c_ushort,
    pub b_next: u32,
    pub /: *mut *mut u32 b_data; / Address from i596 viewpoint,
    pub size: c_ushort,
    pub zero2: c_ushort,
// driver data
    pub skb: *mut sk_buff,
    pub v_next: *mut i596_rbd,
    pub /: *mut *mut u32 b_addr; / This rbd addr from i596 view,
    pub /: *mut *mut *mut unsigned char v_data; / Address from CPUs viewpoint,
// Total 32 bytes...
    pub cache_pad: [u32; 4],
}

// These values as chosen so struct i596_dma fits in one page...
pub const TX_RING_SIZE: c_int = 32;
pub const RX_RING_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i596_scb {
    pub status: c_ushort,
    pub command: c_ushort,
    pub cmd: u32,
    pub rfd: u32,
    pub crc_err: u32,
    pub align_err: u32,
    pub resource_err: u32,
    pub over_err: u32,
    pub rcvdt_err: u32,
    pub short_err: u32,
    pub t_on: c_ushort,
    pub t_off: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i596_iscp {
    pub stat: u32,
    pub scb: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i596_scp {
    pub sysbus: u32,
    pub pad: u32,
    pub iscp: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i596_dma {
    pub __attribute__((aligned(32))): i596_scp scp,
    pub __attribute__((aligned(32))): volatile struct i596_iscp iscp,
    pub __attribute__((aligned(32))): volatile struct i596_scb scb,
    pub __attribute__((aligned(32))): sa_cmd sa_cmd,
    pub __attribute__((aligned(32))): cf_cmd cf_cmd,
    pub __attribute__((aligned(32))): tdr_cmd tdr_cmd,
    pub __attribute__((aligned(32))): mc_cmd mc_cmd,
    pub __attribute__((aligned(32))): i596_rfd rfds[RX_RING_SIZE],
    pub __attribute__((aligned(32))): i596_rbd rbds[RX_RING_SIZE],
    pub __attribute__((aligned(32))): tx_cmd tx_cmds[TX_RING_SIZE],
    pub __attribute__((aligned(32))): i596_tbd tbds[TX_RING_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i596_private {
    pub dma: *mut i596_dma,
    pub stat: u32,
    pub last_restart: c_int,
    pub rfd_head: *mut i596_rfd,
    pub rbd_head: *mut i596_rbd,
    pub cmd_tail: *mut i596_cmd,
    pub cmd_head: *mut i596_cmd,
    pub cmd_backlog: c_int,
    pub last_cmd: u32,
    pub next_tx_cmd: c_int,
    pub options: c_int,
    pub /: *mut *mut spinlock_t lock; / serialize access to chip,
    pub dma_addr: dma_addr_t,
    pub mpu_port: *mut void __iomem,
    pub ca: *mut void __iomem,
}

    static const char init_setup[] =
    {
    0x8E,		/* length, prefetch on */
    0xC8,		/* fifo to 8, monitor off */
    0x80,		/* don't save bad frames */
    0x2E,		/* No source address insertion, 8 byte preamble */
    0x00,		/* priority and backoff defaults */
    0x60,		/* interframe spacing */
    0x00,		/* slot time LSB */
    0xf2,		/* slot time and retries */
    0x00,		/* promiscuous mode */
    0x00,		/* collision detect */
    0x40,		/* minimum frame length */
    0xff,
    0x00,
    0x7f /*  *multi IA */ };
    static int i596_open(struct net_device *dev);
    static netdev_tx_t i596_start_xmit(struct sk_buff *skb, struct net_device *dev);
    static irqreturn_t i596_interrupt(int irq, void *dev_id);
    static int i596_close(struct net_device *dev);
    static void i596_add_cmd(struct net_device *dev, struct i596_cmd *cmd);
    static void i596_tx_timeout (struct net_device *dev, unsigned int txqueue);
    static void print_eth(unsigned char *buf, char *str);
    static void set_multicast_list(struct net_device *dev);
    static inline void ca(struct net_device *dev);
    static void mpu_port(struct net_device *dev, int c, dma_addr_t x);
    let mut rx_ring_size: static int = RX_RING_SIZE;
    let mut ticks_limit: static int = 100;
    let mut max_cmd_backlog: static int = TX_RING_SIZE-1;

    static void i596_poll_controller(struct net_device *dev);

#[no_mangle]
pub unsafe extern "C" fn virt_to_dma(lp: *mut i596_private, v: *mut volatile void) -> dma_addr_t {
    static inline dma_addr_t virt_to_dma(struct i596_private *lp, volatile void *v)
    {
    return lp.dma_addr + ((unsigned long)v - (unsigned long)lp.dma);
    }

    static inline void dma_sync_dev(struct net_device *ndev, volatile void *addr,
    size_t len)
    {
    dma_sync_single_for_device(ndev.dev.parent,
    virt_to_dma(netdev_priv(ndev), addr), len,
    DMA_BIDIRECTIONAL);
    }
    static inline void dma_sync_cpu(struct net_device *ndev, volatile void *addr,
    size_t len)
    {
    dma_sync_single_for_cpu(ndev.dev.parent,
    virt_to_dma(netdev_priv(ndev), addr), len,
    DMA_BIDIRECTIONAL);
    }

    static inline void dma_sync_dev(struct net_device *ndev, volatile void *addr,
    size_t len)
    {
    }
    static inline void dma_sync_cpu(struct net_device *ndev, volatile void *addr,
    size_t len)
    {
    }

#[no_mangle]
pub unsafe extern "C" fn wait_istat(dev: *mut net_device, dma: *mut i596_dma, delcnt: c_int, str: *mut c_char) -> c_int {
    static inline int wait_istat(struct net_device *dev, struct i596_dma *dma, int delcnt, char *str)
    {
    dma_sync_cpu(dev, &(dma.iscp), sizeof(struct i596_iscp));
    while (--delcnt && dma.iscp.stat) {
    udelay(10);
    dma_sync_cpu(dev, &(dma.iscp), sizeof(struct i596_iscp));
    }
    if (!delcnt) {
    printk(KERN_ERR "%s: %s, iscp.stat %04x, didn't clear\n",
    dev.name, str, SWAP16(dma.iscp.stat));
    return -1;
    } else
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn wait_cmd(dev: *mut net_device, dma: *mut i596_dma, delcnt: c_int, str: *mut c_char) -> c_int {
    static inline int wait_cmd(struct net_device *dev, struct i596_dma *dma, int delcnt, char *str)
    {
    dma_sync_cpu(dev, &(dma.scb), sizeof(struct i596_scb));
    while (--delcnt && dma.scb.command) {
    udelay(10);
    dma_sync_cpu(dev, &(dma.scb), sizeof(struct i596_scb));
    }
    if (!delcnt) {
    printk(KERN_ERR "%s: %s, status %4.4x, cmd %4.4x.\n",
    dev.name, str,
    SWAP16(dma.scb.status),
    SWAP16(dma.scb.command));
    return -1;
    } else
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i596_display_data(dev: *mut net_device) {
    static void i596_display_data(struct net_device *dev)
    {
    struct i596_private *lp = netdev_priv(dev);
    struct i596_dma *dma = lp.dma;
    struct i596_cmd *cmd;
    struct i596_rfd *rfd;
    struct i596_rbd *rbd;
    printk(KERN_DEBUG "lp and scp at %p, .sysbus = %08x, .iscp = %08x\n",
    &dma.scp, dma.scp.sysbus, SWAP32(dma.scp.iscp));
    printk(KERN_DEBUG "iscp at %p, iscp.stat = %08x, .scb = %08x\n",
    &dma.iscp, SWAP32(dma.iscp.stat), SWAP32(dma.iscp.scb));
    printk(KERN_DEBUG "scb at %p, scb.status = %04x, .command = %04x,"
    " .cmd = %08x, .rfd = %08x\n",
    &dma.scb, SWAP16(dma.scb.status), SWAP16(dma.scb.command),
    SWAP16(dma.scb.cmd), SWAP32(dma.scb.rfd));
    printk(KERN_DEBUG "   errors: crc %x, align %x, resource %x,"
    " over %x, rcvdt %x, short %x\n",
    SWAP32(dma.scb.crc_err), SWAP32(dma.scb.align_err),
    SWAP32(dma.scb.resource_err), SWAP32(dma.scb.over_err),
    SWAP32(dma.scb.rcvdt_err), SWAP32(dma.scb.short_err));
    cmd = lp.cmd_head;
    while (cmd != core::ptr::null_mut()) {
    printk(KERN_DEBUG
    "cmd at %p, .status = %04x, .command = %04x,"
    " .b_next = %08x\n",
    cmd, SWAP16(cmd.status), SWAP16(cmd.command),
    SWAP32(cmd.b_next));
    cmd = cmd.v_next;
    }
    rfd = lp.rfd_head;
    printk(KERN_DEBUG "rfd_head = %p\n", rfd);
    do {
    printk(KERN_DEBUG
    "   %p .stat %04x, .cmd %04x, b_next %08x, rbd %08x,"
    " count %04x\n",
    rfd, SWAP16(rfd.stat), SWAP16(rfd.cmd),
    SWAP32(rfd.b_next), SWAP32(rfd.rbd),
    SWAP16(rfd.count));
    rfd = rfd.v_next;
    } while (rfd != lp.rfd_head);
    rbd = lp.rbd_head;
    printk(KERN_DEBUG "rbd_head = %p\n", rbd);
    do {
    printk(KERN_DEBUG
    "   %p .count %04x, b_next %08x, b_data %08x,"
    " size %04x\n",
    rbd, SWAP16(rbd.count), SWAP32(rbd.b_next),
    SWAP32(rbd.b_data), SWAP16(rbd.size));
    rbd = rbd.v_next;
    } while (rbd != lp.rbd_head);
    dma_sync_cpu(dev, dma, sizeof(struct i596_dma));
    }
#[no_mangle]
pub unsafe extern "C" fn init_rx_bufs(dev: *mut net_device) -> c_int {
    static inline int init_rx_bufs(struct net_device *dev)
    {
    struct i596_private *lp = netdev_priv(dev);
    struct i596_dma *dma = lp.dma;
    int i;
    struct i596_rfd *rfd;
    struct i596_rbd *rbd;
// First build the Receive Buffer Descriptor List
    for (i = 0, rbd = dma.rbds; i < rx_ring_size; i++, rbd++) {
    dma_addr_t dma_addr;
    struct sk_buff *skb;
    skb = netdev_alloc_skb_ip_align(dev, PKT_BUF_SZ);
    if (skb == core::ptr::null_mut())
    return -1;
    dma_addr = dma_map_single(dev.dev.parent, skb.data,
    PKT_BUF_SZ, DMA_FROM_DEVICE);
    rbd.v_next = rbd+1;
    rbd.b_next = SWAP32(virt_to_dma(lp, rbd+1));
    rbd.b_addr = SWAP32(virt_to_dma(lp, rbd));
    rbd.skb = skb;
    rbd.v_data = skb.data;
    rbd.b_data = SWAP32(dma_addr);
    rbd.size = SWAP16(PKT_BUF_SZ);
    }
    lp.rbd_head = dma.rbds;
    rbd = dma.rbds + rx_ring_size - 1;
    rbd.v_next = dma.rbds;
    rbd.b_next = SWAP32(virt_to_dma(lp, dma.rbds));
// Now build the Receive Frame Descriptor List
    for (i = 0, rfd = dma.rfds; i < rx_ring_size; i++, rfd++) {
    rfd.rbd = I596_NULL;
    rfd.v_next = rfd+1;
    rfd.v_prev = rfd-1;
    rfd.b_next = SWAP32(virt_to_dma(lp, rfd+1));
    rfd.cmd = SWAP16(CMD_FLEX);
    }
    lp.rfd_head = dma.rfds;
    dma.scb.rfd = SWAP32(virt_to_dma(lp, dma.rfds));
    rfd = dma.rfds;
    rfd.rbd = SWAP32(virt_to_dma(lp, lp.rbd_head));
    rfd.v_prev = dma.rfds + rx_ring_size - 1;
    rfd = dma.rfds + rx_ring_size - 1;
    rfd.v_next = dma.rfds;
    rfd.b_next = SWAP32(virt_to_dma(lp, dma.rfds));
    rfd.cmd = SWAP16(CMD_EOL|CMD_FLEX);
    dma_sync_dev(dev, dma, sizeof(struct i596_dma));
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn remove_rx_bufs(dev: *mut net_device) {
    static inline void remove_rx_bufs(struct net_device *dev)
    {
    struct i596_private *lp = netdev_priv(dev);
    struct i596_rbd *rbd;
    int i;
    for (i = 0, rbd = lp.dma.rbds; i < rx_ring_size; i++, rbd++) {
    if (rbd.skb == core::ptr::null_mut())
    break;
    dma_unmap_single(dev.dev.parent,
    (dma_addr_t)SWAP32(rbd.b_data),
    PKT_BUF_SZ, DMA_FROM_DEVICE);
    dev_kfree_skb(rbd.skb);
    }
    }
#[no_mangle]
unsafe extern "C" fn rebuild_rx_bufs(dev: *mut net_device) {
    static void rebuild_rx_bufs(struct net_device *dev)
    {
    struct i596_private *lp = netdev_priv(dev);
    struct i596_dma *dma = lp.dma;
    int i;
// Ensure rx frame/buffer descriptors are tidy
    for (i = 0; i < rx_ring_size; i++) {
    dma.rfds[i].rbd = I596_NULL;
    dma.rfds[i].cmd = SWAP16(CMD_FLEX);
    }
    dma.rfds[rx_ring_size-1].cmd = SWAP16(CMD_EOL|CMD_FLEX);
    lp.rfd_head = dma.rfds;
    dma.scb.rfd = SWAP32(virt_to_dma(lp, dma.rfds));
    lp.rbd_head = dma.rbds;
    dma.rfds[0].rbd = SWAP32(virt_to_dma(lp, dma.rbds));
    dma_sync_dev(dev, dma, sizeof(struct i596_dma));
    }
#[no_mangle]
unsafe extern "C" fn init_i596_mem(dev: *mut net_device) -> c_int {
    static int init_i596_mem(struct net_device *dev)
    {
    struct i596_private *lp = netdev_priv(dev);
    struct i596_dma *dma = lp.dma;
    unsigned long flags;
    mpu_port(dev, PORT_RESET, 0);
    udelay(100);			/* Wait 100us - seems to help */
// change the scp address
    lp.last_cmd = jiffies;
    dma.scp.sysbus = SYSBUS;
    dma.scp.iscp = SWAP32(virt_to_dma(lp, &(dma.iscp)));
    dma.iscp.scb = SWAP32(virt_to_dma(lp, &(dma.scb)));
    dma.iscp.stat = SWAP32(ISCP_BUSY);
    lp.cmd_backlog = 0;
    lp.cmd_head = core::ptr::null_mut();
    dma.scb.cmd = I596_NULL;
    DEB(DEB_INIT, printk(KERN_DEBUG "%s: starting i82596.\n", dev.name));
    dma_sync_dev(dev, &(dma.scp), sizeof(struct i596_scp));
    dma_sync_dev(dev, &(dma.iscp), sizeof(struct i596_iscp));
    dma_sync_dev(dev, &(dma.scb), sizeof(struct i596_scb));
    mpu_port(dev, PORT_ALTSCP, virt_to_dma(lp, &dma.scp));
    ca(dev);
    if (wait_istat(dev, dma, 1000, "initialization timed out"))
    goto failed;
    DEB(DEB_INIT, printk(KERN_DEBUG
    "%s: i82596 initialization successful\n",
    dev.name));
    if (request_irq(dev.irq, i596_interrupt, 0, "i82596", dev)) {
    printk(KERN_ERR "%s: IRQ %d not free\n", dev.name, dev.irq);
    goto failed;
    }
// Ensure rx frame/buffer descriptors are tidy
    rebuild_rx_bufs(dev);
    dma.scb.command = 0;
    dma_sync_dev(dev, &(dma.scb), sizeof(struct i596_scb));
    DEB(DEB_INIT, printk(KERN_DEBUG
    "%s: queuing CmdConfigure\n", dev.name));
    memcpy(dma.cf_cmd.i596_config, init_setup, 14);
    dma.cf_cmd.cmd.command = SWAP16(CmdConfigure);
    dma_sync_dev(dev, &(dma.cf_cmd), sizeof(struct cf_cmd));
    i596_add_cmd(dev, &dma.cf_cmd.cmd);
    DEB(DEB_INIT, printk(KERN_DEBUG "%s: queuing CmdSASetup\n", dev.name));
    memcpy(dma.sa_cmd.eth_addr, dev.dev_addr, ETH_ALEN);
    dma.sa_cmd.cmd.command = SWAP16(CmdSASetup);
    dma_sync_dev(dev, &(dma.sa_cmd), sizeof(struct sa_cmd));
    i596_add_cmd(dev, &dma.sa_cmd.cmd);
    DEB(DEB_INIT, printk(KERN_DEBUG "%s: queuing CmdTDR\n", dev.name));
    dma.tdr_cmd.cmd.command = SWAP16(CmdTDR);
    dma_sync_dev(dev, &(dma.tdr_cmd), sizeof(struct tdr_cmd));
    i596_add_cmd(dev, &dma.tdr_cmd.cmd);
    spin_lock_irqsave (&lp.lock, flags);
    if (wait_cmd(dev, dma, 1000, "timed out waiting to issue RX_START")) {
    spin_unlock_irqrestore (&lp.lock, flags);
    goto failed_free_irq;
    }
    DEB(DEB_INIT, printk(KERN_DEBUG "%s: Issuing RX_START\n", dev.name));
    dma.scb.command = SWAP16(RX_START);
    dma.scb.rfd = SWAP32(virt_to_dma(lp, dma.rfds));
    dma_sync_dev(dev, &(dma.scb), sizeof(struct i596_scb));
    ca(dev);
    spin_unlock_irqrestore (&lp.lock, flags);
    if (wait_cmd(dev, dma, 1000, "RX_START not processed"))
    goto failed_free_irq;
    DEB(DEB_INIT, printk(KERN_DEBUG
    "%s: Receive unit started OK\n", dev.name));
    return 0;
    failed_free_irq:
    free_irq(dev.irq, dev);
    failed:
    printk(KERN_ERR "%s: Failed to initialise 82596\n", dev.name);
    mpu_port(dev, PORT_RESET, 0);
    return -1;
    }
#[no_mangle]
pub unsafe extern "C" fn i596_rx(dev: *mut net_device) -> c_int {
    static inline int i596_rx(struct net_device *dev)
    {
    struct i596_private *lp = netdev_priv(dev);
    struct i596_rfd *rfd;
    struct i596_rbd *rbd;
    let mut frames: c_int = 0;
    DEB(DEB_RXFRAME, printk(KERN_DEBUG
    "i596_rx(), rfd_head %p, rbd_head %p\n",
    lp.rfd_head, lp.rbd_head));
    rfd = lp.rfd_head;		/* Ref next frame to check */
    dma_sync_cpu(dev, rfd, sizeof(struct i596_rfd));
    while (rfd.stat & SWAP16(STAT_C)) {	/* Loop while complete frames */
    if (rfd.rbd == I596_NULL)
    rbd = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn if(lp->rbd_head->b_addr: rfd->rbd ==) -> else {
    rbd = lp.rbd_head;
    dma_sync_cpu(dev, rbd, sizeof(struct i596_rbd));
    } else {
    printk(KERN_ERR "%s: rbd chain broken!\n", dev.name);
// XXX Now what?
    rbd = core::ptr::null_mut();
    }
    DEB(DEB_RXFRAME, printk(KERN_DEBUG
    "  rfd %p, rfd.rbd %08x, rfd.stat %04x\n",
    rfd, rfd.rbd, rfd.stat));
    if (rbd != core::ptr::null_mut() && (rfd.stat & SWAP16(STAT_OK))) {
// a good frame
    let mut pkt_len: c_int = SWAP16(rbd.count) & 0x3fff;
    struct sk_buff *skb = rbd.skb;
    let mut rx_in_place: c_int = 0;
    DEB(DEB_RXADDR, print_eth(rbd.v_data, "received"));
    frames++;
// Check if the packet is long enough to just accept
// without copying to a properly sized skbuff.
//
    if (pkt_len > rx_copybreak) {
    struct sk_buff *newskb;
    dma_addr_t dma_addr;
    dma_unmap_single(dev.dev.parent,
    (dma_addr_t)SWAP32(rbd.b_data),
    PKT_BUF_SZ, DMA_FROM_DEVICE);
// Get fresh skbuff to replace filled one.
    newskb = netdev_alloc_skb_ip_align(dev,
    PKT_BUF_SZ);
    if (newskb == core::ptr::null_mut()) {
    skb = core::ptr::null_mut();	/* drop pkt */
    goto memory_squeeze;
    }
// Pass up the skb already on the Rx ring.
    skb_put(skb, pkt_len);
    rx_in_place = 1;
    rbd.skb = newskb;
    dma_addr = dma_map_single(dev.dev.parent,
    newskb.data,
    PKT_BUF_SZ,
    DMA_FROM_DEVICE);
    rbd.v_data = newskb.data;
    rbd.b_data = SWAP32(dma_addr);
    dma_sync_dev(dev, rbd, sizeof(struct i596_rbd));
    } else {
    skb = netdev_alloc_skb_ip_align(dev, pkt_len);
    }
    memory_squeeze:
    if (skb == core::ptr::null_mut()) {
// XXX tulip.c can defer packets here!!
    dev.stats.rx_dropped++;
    } else {
    if (!rx_in_place) {
// 16 byte align the data fields
    dma_sync_single_for_cpu(dev.dev.parent,
    (dma_addr_t)SWAP32(rbd.b_data),
    PKT_BUF_SZ, DMA_FROM_DEVICE);
    skb_put_data(skb, rbd.v_data,
    pkt_len);
    dma_sync_single_for_device(dev.dev.parent,
    (dma_addr_t)SWAP32(rbd.b_data),
    PKT_BUF_SZ, DMA_FROM_DEVICE);
    }
    skb.len = pkt_len;
    skb.protocol = eth_type_trans(skb, dev);
    netif_rx(skb);
    dev.stats.rx_packets++;
    dev.stats.rx_bytes += pkt_len;
    }
    } else {
    DEB(DEB_ERRORS, printk(KERN_DEBUG
    "%s: Error, rfd.stat = 0x%04x\n",
    dev.name, rfd.stat));
    dev.stats.rx_errors++;
    if (rfd.stat & SWAP16(0x0100))
    dev.stats.collisions++;
    if (rfd.stat & SWAP16(0x8000))
    dev.stats.rx_length_errors++;
    if (rfd.stat & SWAP16(0x0001))
    dev.stats.rx_over_errors++;
    if (rfd.stat & SWAP16(0x0002))
    dev.stats.rx_fifo_errors++;
    if (rfd.stat & SWAP16(0x0004))
    dev.stats.rx_frame_errors++;
    if (rfd.stat & SWAP16(0x0008))
    dev.stats.rx_crc_errors++;
    if (rfd.stat & SWAP16(0x0010))
    dev.stats.rx_length_errors++;
    }
// Clear the buffer descriptor count and EOF + F flags
    if (rbd != core::ptr::null_mut() && (rbd.count & SWAP16(0x4000))) {
    rbd.count = 0;
    lp.rbd_head = rbd.v_next;
    dma_sync_dev(dev, rbd, sizeof(struct i596_rbd));
    }
// Tidy the frame descriptor, marking it as end of list
    rfd.rbd = I596_NULL;
    rfd.stat = 0;
    rfd.cmd = SWAP16(CMD_EOL|CMD_FLEX);
    rfd.count = 0;
// Update record of next frame descriptor to process
    lp.dma.scb.rfd = rfd.b_next;
    lp.rfd_head = rfd.v_next;
    dma_sync_dev(dev, rfd, sizeof(struct i596_rfd));
// Remove end-of-list from old end descriptor
    rfd.v_prev.cmd = SWAP16(CMD_FLEX);
    dma_sync_dev(dev, rfd.v_prev, sizeof(struct i596_rfd));
    rfd = lp.rfd_head;
    dma_sync_cpu(dev, rfd, sizeof(struct i596_rfd));
    }
    DEB(DEB_RXFRAME, printk(KERN_DEBUG "frames %d\n", frames));
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn i596_cleanup_cmd(dev: *mut net_device, lp: *mut i596_private) {
    static inline void i596_cleanup_cmd(struct net_device *dev, struct i596_private *lp)
    {
    struct i596_cmd *ptr;
    while (lp.cmd_head != core::ptr::null_mut()) {
    ptr = lp.cmd_head;
    lp.cmd_head = ptr.v_next;
    lp.cmd_backlog--;
    switch (SWAP16(ptr.command) & 0x7) {
    case CmdTx:
    {
    struct tx_cmd *tx_cmd = (struct tx_cmd *) ptr;
    struct sk_buff *skb = tx_cmd.skb;
    dma_unmap_single(dev.dev.parent,
    tx_cmd.dma_addr,
    skb.len, DMA_TO_DEVICE);
    dev_kfree_skb(skb);
    dev.stats.tx_errors++;
    dev.stats.tx_aborted_errors++;
    ptr.v_next = core::ptr::null_mut();
    ptr.b_next = I596_NULL;
    tx_cmd.cmd.command = 0;  /* Mark as free */
    break;
    }
    default:
    ptr.v_next = core::ptr::null_mut();
    ptr.b_next = I596_NULL;
    }
    dma_sync_dev(dev, ptr, sizeof(struct i596_cmd));
    }
    wait_cmd(dev, lp.dma, 100, "i596_cleanup_cmd timed out");
    lp.dma.scb.cmd = I596_NULL;
    dma_sync_dev(dev, &(lp.dma.scb), sizeof(struct i596_scb));
    }
#[no_mangle]
pub unsafe extern "C" fn i596_reset(dev: *mut net_device, lp: *mut i596_private) {
    static inline void i596_reset(struct net_device *dev, struct i596_private *lp)
    {
    unsigned long flags;
    DEB(DEB_RESET, printk(KERN_DEBUG "i596_reset\n"));
    spin_lock_irqsave (&lp.lock, flags);
    wait_cmd(dev, lp.dma, 100, "i596_reset timed out");
    netif_stop_queue(dev);
// FIXME: this command might cause an lpmc
    lp.dma.scb.command = SWAP16(CUC_ABORT | RX_ABORT);
    dma_sync_dev(dev, &(lp.dma.scb), sizeof(struct i596_scb));
    ca(dev);
// wait for shutdown
    wait_cmd(dev, lp.dma, 1000, "i596_reset 2 timed out");
    spin_unlock_irqrestore (&lp.lock, flags);
    i596_cleanup_cmd(dev, lp);
    i596_rx(dev);
    netif_start_queue(dev);
    init_i596_mem(dev);
    }
#[no_mangle]
unsafe extern "C" fn i596_add_cmd(dev: *mut net_device, cmd: *mut i596_cmd) {
    static void i596_add_cmd(struct net_device *dev, struct i596_cmd *cmd)
    {
    struct i596_private *lp = netdev_priv(dev);
    struct i596_dma *dma = lp.dma;
    unsigned long flags;
    DEB(DEB_ADDCMD, printk(KERN_DEBUG "i596_add_cmd cmd_head %p\n",
    lp.cmd_head));
    cmd.status = 0;
    cmd.command |= SWAP16(CMD_EOL | CMD_INTR);
    cmd.v_next = core::ptr::null_mut();
    cmd.b_next = I596_NULL;
    dma_sync_dev(dev, cmd, sizeof(struct i596_cmd));
    spin_lock_irqsave (&lp.lock, flags);
    if (lp.cmd_head != core::ptr::null_mut()) {
    lp.cmd_tail.v_next = cmd;
    lp.cmd_tail.b_next = SWAP32(virt_to_dma(lp, &cmd.status));
    dma_sync_dev(dev, lp.cmd_tail, sizeof(struct i596_cmd));
    } else {
    lp.cmd_head = cmd;
    wait_cmd(dev, dma, 100, "i596_add_cmd timed out");
    dma.scb.cmd = SWAP32(virt_to_dma(lp, &cmd.status));
    dma.scb.command = SWAP16(CUC_START);
    dma_sync_dev(dev, &(dma.scb), sizeof(struct i596_scb));
    ca(dev);
    }
    lp.cmd_tail = cmd;
    lp.cmd_backlog++;
    spin_unlock_irqrestore (&lp.lock, flags);
    if (lp.cmd_backlog > max_cmd_backlog) {
    let mut tickssofar: c_ulong = jiffies - lp.last_cmd;
    if (tickssofar < ticks_limit)
    return;
    printk(KERN_ERR
    "%s: command unit timed out, status resetting.\n",
    dev.name);

    i596_reset(dev, lp);

    }
    }
#[no_mangle]
unsafe extern "C" fn i596_open(dev: *mut net_device) -> c_int {
    static int i596_open(struct net_device *dev)
    {
    DEB(DEB_OPEN, printk(KERN_DEBUG
    "%s: i596_open() irq %d.\n", dev.name, dev.irq));
    if (init_rx_bufs(dev)) {
    printk(KERN_ERR "%s: Failed to init rx bufs\n", dev.name);
    return -EAGAIN;
    }
    if (init_i596_mem(dev)) {
    printk(KERN_ERR "%s: Failed to init memory\n", dev.name);
    goto out_remove_rx_bufs;
    }
    netif_start_queue(dev);
    return 0;
    out_remove_rx_bufs:
    remove_rx_bufs(dev);
    return -EAGAIN;
    }
#[no_mangle]
unsafe extern "C" fn i596_tx_timeout(dev: *mut net_device, txqueue: c_uint) {
    static void i596_tx_timeout (struct net_device *dev, unsigned int txqueue)
    {
    struct i596_private *lp = netdev_priv(dev);
// Transmitter timeout, serious problems.
    DEB(DEB_ERRORS, printk(KERN_DEBUG
    "%s: transmit timed out, status resetting.\n",
    dev.name));
    dev.stats.tx_errors++;
// Try to restart the adaptor
    if (lp.last_restart == dev.stats.tx_packets) {
    DEB(DEB_ERRORS, printk(KERN_DEBUG "Resetting board.\n"));
// Shutdown and restart
    i596_reset (dev, lp);
    } else {
// Issue a channel attention signal
    DEB(DEB_ERRORS, printk(KERN_DEBUG "Kicking board.\n"));
    lp.dma.scb.command = SWAP16(CUC_START | RX_START);
    dma_sync_dev(dev, &(lp.dma.scb), sizeof(struct i596_scb));
    ca (dev);
    lp.last_restart = dev.stats.tx_packets;
    }
    netif_trans_update(dev); /* prevent tx timeout */
    netif_wake_queue (dev);
    }
#[no_mangle]
unsafe extern "C" fn i596_start_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t {
    static netdev_tx_t i596_start_xmit(struct sk_buff *skb, struct net_device *dev)
    {
    struct i596_private *lp = netdev_priv(dev);
    struct tx_cmd *tx_cmd;
    struct i596_tbd *tbd;
    let mut length: c_short = skb.len;
    DEB(DEB_STARTTX, printk(KERN_DEBUG
    "%s: i596_start_xmit(%x,%p) called\n",
    dev.name, skb.len, skb.data));
    if (length < ETH_ZLEN) {
    if (skb_padto(skb, ETH_ZLEN))
    return NETDEV_TX_OK;
    length = ETH_ZLEN;
    }
    netif_stop_queue(dev);
    tx_cmd = lp.dma.tx_cmds + lp.next_tx_cmd;
    tbd = lp.dma.tbds + lp.next_tx_cmd;
    if (tx_cmd.cmd.command) {
    DEB(DEB_ERRORS, printk(KERN_DEBUG
    "%s: xmit ring full, dropping packet.\n",
    dev.name));
    dev.stats.tx_dropped++;
    dev_kfree_skb_any(skb);
    } else {
    if (++lp.next_tx_cmd == TX_RING_SIZE)
    lp.next_tx_cmd = 0;
    tx_cmd.tbd = SWAP32(virt_to_dma(lp, tbd));
    tbd.next = I596_NULL;
    tx_cmd.cmd.command = SWAP16(CMD_FLEX | CmdTx);
    tx_cmd.skb = skb;
    tx_cmd.pad = 0;
    tx_cmd.size = 0;
    tbd.pad = 0;
    tbd.size = SWAP16(EOF | length);
    tx_cmd.dma_addr = dma_map_single(dev.dev.parent, skb.data,
    skb.len, DMA_TO_DEVICE);
    tbd.data = SWAP32(tx_cmd.dma_addr);
    DEB(DEB_TXADDR, print_eth(skb.data, "tx-queued"));
    dma_sync_dev(dev, tx_cmd, sizeof(struct tx_cmd));
    dma_sync_dev(dev, tbd, sizeof(struct i596_tbd));
    i596_add_cmd(dev, &tx_cmd.cmd);
    dev.stats.tx_packets++;
    dev.stats.tx_bytes += length;
    }
    netif_start_queue(dev);
    return NETDEV_TX_OK;
    }
#[no_mangle]
unsafe extern "C" fn print_eth(add: *mut c_uchar, str: *mut c_char) {
    static void print_eth(unsigned char *add, char *str)
    {
    printk(KERN_DEBUG "i596 0x%p, %pM -. %pM %02X%02X, %s\n",
    add, add + 6, add, add[12], add[13], str);
    }
    static const struct net_device_ops i596_netdev_ops = {
    .ndo_open		= i596_open,
    .ndo_stop		= i596_close,
    .ndo_start_xmit		= i596_start_xmit,
    .ndo_set_rx_mode	= set_multicast_list,
    .ndo_tx_timeout		= i596_tx_timeout,
    .ndo_validate_addr	= eth_validate_addr,
    .ndo_set_mac_address	= eth_mac_addr,

    .ndo_poll_controller	= i596_poll_controller,

    };
#[no_mangle]
unsafe extern "C" fn i82596_probe(dev: *mut net_device) -> c_int {
    static int i82596_probe(struct net_device *dev)
    {
    struct i596_private *lp = netdev_priv(dev);
    int ret;
// This lot is ensure things have been cache line aligned.
    BUILD_BUG_ON(sizeof(struct i596_rfd) != 32);
    BUILD_BUG_ON(sizeof(struct i596_rbd) &  31);
    BUILD_BUG_ON(sizeof(struct tx_cmd)   &  31);
    BUILD_BUG_ON(sizeof(struct i596_tbd) != 32);

    BUILD_BUG_ON(sizeof(struct i596_dma) > 4096);

    if (!dev.base_addr || !dev.irq)
    return -ENODEV;
    dev.netdev_ops = &i596_netdev_ops;
    dev.watchdog_timeo = TX_TIMEOUT;
    memset(lp.dma, 0, sizeof(struct i596_dma));
    lp.dma.scb.command = 0;
    lp.dma.scb.cmd = I596_NULL;
    lp.dma.scb.rfd = I596_NULL;
    spin_lock_init(&lp.lock);
    dma_sync_dev(dev, lp.dma, sizeof(struct i596_dma));
    ret = register_netdev(dev);
    if (ret)
    return ret;
    DEB(DEB_PROBE, printk(KERN_INFO "%s: 82596 at %#3lx, %pM IRQ %d.\n",
    dev.name, dev.base_addr, dev.dev_addr,
    dev.irq));
    DEB(DEB_INIT, printk(KERN_INFO
    "%s: dma at 0x%p (%d bytes), lp.scb at 0x%p\n",
    dev.name, lp.dma, (int)sizeof(struct i596_dma),
    &lp.dma.scb));
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn i596_poll_controller(dev: *mut net_device) {
    static void i596_poll_controller(struct net_device *dev)
    {
    disable_irq(dev.irq);
    i596_interrupt(dev.irq, dev);
    enable_irq(dev.irq);
    }

#[no_mangle]
unsafe extern "C" fn i596_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t i596_interrupt(int irq, void *dev_id)
    {
    struct net_device *dev = dev_id;
    struct i596_private *lp;
    struct i596_dma *dma;
    unsigned short status, ack_cmd = 0;
    lp = netdev_priv(dev);
    dma = lp.dma;
    spin_lock (&lp.lock);
    wait_cmd(dev, dma, 100, "i596 interrupt, timeout");
    status = SWAP16(dma.scb.status);
    DEB(DEB_INTS, printk(KERN_DEBUG
    "%s: i596 interrupt, IRQ %d, status %4.4x.\n",
    dev.name, dev.irq, status));
    ack_cmd = status & 0xf000;
    if (!ack_cmd) {
    DEB(DEB_ERRORS, printk(KERN_DEBUG
    "%s: interrupt with no events\n",
    dev.name));
    spin_unlock (&lp.lock);
    return IRQ_NONE;
    }
    if ((status & 0x8000) || (status & 0x2000)) {
    struct i596_cmd *ptr;
    if ((status & 0x8000))
    DEB(DEB_INTS,
    printk(KERN_DEBUG
    "%s: i596 interrupt completed command.\n",
    dev.name));
    if ((status & 0x2000))
    DEB(DEB_INTS,
    printk(KERN_DEBUG
    "%s: i596 interrupt command unit inactive %x.\n",
    dev.name, status & 0x0700));
    while (lp.cmd_head != core::ptr::null_mut()) {
    dma_sync_cpu(dev, lp.cmd_head, sizeof(struct i596_cmd));
    if (!(lp.cmd_head.status & SWAP16(STAT_C)))
    break;
    ptr = lp.cmd_head;
    DEB(DEB_STATUS,
    printk(KERN_DEBUG
    "cmd_head.status = %04x, .command = %04x\n",
    SWAP16(lp.cmd_head.status),
    SWAP16(lp.cmd_head.command)));
    lp.cmd_head = ptr.v_next;
    lp.cmd_backlog--;
    switch (SWAP16(ptr.command) & 0x7) {
    case CmdTx:
    {
    struct tx_cmd *tx_cmd = (struct tx_cmd *) ptr;
    struct sk_buff *skb = tx_cmd.skb;
    if (ptr.status & SWAP16(STAT_OK)) {
    DEB(DEB_TXADDR,
    print_eth(skb.data, "tx-done"));
    } else {
    dev.stats.tx_errors++;
    if (ptr.status & SWAP16(0x0020))
    dev.stats.collisions++;
    if (!(ptr.status & SWAP16(0x0040)))
    dev.stats.tx_heartbeat_errors++;
    if (ptr.status & SWAP16(0x0400))
    dev.stats.tx_carrier_errors++;
    if (ptr.status & SWAP16(0x0800))
    dev.stats.collisions++;
    if (ptr.status & SWAP16(0x1000))
    dev.stats.tx_aborted_errors++;
    }
    dma_unmap_single(dev.dev.parent,
    tx_cmd.dma_addr,
    skb.len, DMA_TO_DEVICE);
    dev_consume_skb_irq(skb);
    tx_cmd.cmd.command = 0; /* Mark free */
    break;
    }
    case CmdTDR:
    {
    let mut status: c_ushort = SWAP16(((struct tdr_cmd *)ptr).status);
    if (status & 0x8000) {
    DEB(DEB_ANY,
    printk(KERN_DEBUG "%s: link ok.\n",
    dev.name));
    } else {
    if (status & 0x4000)
    printk(KERN_ERR
    "%s: Transceiver problem.\n",
    dev.name);
    if (status & 0x2000)
    printk(KERN_ERR
    "%s: Termination problem.\n",
    dev.name);
    if (status & 0x1000)
    printk(KERN_ERR
    "%s: Short circuit.\n",
    dev.name);
    DEB(DEB_TDR,
    printk(KERN_DEBUG "%s: Time %d.\n",
    dev.name, status & 0x07ff));
    }
    break;
    }
    case CmdConfigure:
//
// Zap command so set_multicast_list() know
// it is free
//
    ptr.command = 0;
    break;
    }
    ptr.v_next = core::ptr::null_mut();
    ptr.b_next = I596_NULL;
    dma_sync_dev(dev, ptr, sizeof(struct i596_cmd));
    lp.last_cmd = jiffies;
    }
// This mess is arranging that only the last of any outstanding
// commands has the interrupt bit set.  Should probably really
// only add to the cmd queue when the CU is stopped.
//
    ptr = lp.cmd_head;
    while ((ptr != core::ptr::null_mut()) && (ptr != lp.cmd_tail)) {
    struct i596_cmd *prev = ptr;
    ptr.command &= SWAP16(0x1fff);
    ptr = ptr.v_next;
    dma_sync_dev(dev, prev, sizeof(struct i596_cmd));
    }
    if (lp.cmd_head != core::ptr::null_mut())
    ack_cmd |= CUC_START;
    dma.scb.cmd = SWAP32(virt_to_dma(lp, &lp.cmd_head.status));
    dma_sync_dev(dev, &dma.scb, sizeof(struct i596_scb));
    }
    if ((status & 0x1000) || (status & 0x4000)) {
    if ((status & 0x4000))
    DEB(DEB_INTS,
    printk(KERN_DEBUG
    "%s: i596 interrupt received a frame.\n",
    dev.name));
    i596_rx(dev);
// Only RX_START if stopped - RGH 07-07-96
    if (status & 0x1000) {
    if (netif_running(dev)) {
    DEB(DEB_ERRORS,
    printk(KERN_DEBUG
    "%s: i596 interrupt receive unit inactive, status 0x%x\n",
    dev.name, status));
    ack_cmd |= RX_START;
    dev.stats.rx_errors++;
    dev.stats.rx_fifo_errors++;
    rebuild_rx_bufs(dev);
    }
    }
    }
    wait_cmd(dev, dma, 100, "i596 interrupt, timeout");
    dma.scb.command = SWAP16(ack_cmd);
    dma_sync_dev(dev, &dma.scb, sizeof(struct i596_scb));
// DANGER: I suspect that some kind of interrupt
    acknowledgement aside from acking the 82596 might be needed
    here...  but it's running acceptably without */
    ca(dev);
    wait_cmd(dev, dma, 100, "i596 interrupt, exit timeout");
    DEB(DEB_INTS, printk(KERN_DEBUG "%s: exiting interrupt.\n", dev.name));
    spin_unlock (&lp.lock);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn i596_close(dev: *mut net_device) -> c_int {
    static int i596_close(struct net_device *dev)
    {
    struct i596_private *lp = netdev_priv(dev);
    unsigned long flags;
    netif_stop_queue(dev);
    DEB(DEB_INIT,
    printk(KERN_DEBUG
    "%s: Shutting down ethercard, status was %4.4x.\n",
    dev.name, SWAP16(lp.dma.scb.status)));
    spin_lock_irqsave(&lp.lock, flags);
    wait_cmd(dev, lp.dma, 100, "close1 timed out");
    lp.dma.scb.command = SWAP16(CUC_ABORT | RX_ABORT);
    dma_sync_dev(dev, &lp.dma.scb, sizeof(struct i596_scb));
    ca(dev);
    wait_cmd(dev, lp.dma, 100, "close2 timed out");
    spin_unlock_irqrestore(&lp.lock, flags);
    DEB(DEB_STRUCT, i596_display_data(dev));
    i596_cleanup_cmd(dev, lp);
    free_irq(dev.irq, dev);
    remove_rx_bufs(dev);
    return 0;
    }
//
// Set or clear the multicast filter for this adaptor.
//
#[no_mangle]
unsafe extern "C" fn set_multicast_list(dev: *mut net_device) {
    static void set_multicast_list(struct net_device *dev)
    {
    struct i596_private *lp = netdev_priv(dev);
    struct i596_dma *dma = lp.dma;
    let mut config: c_int = 0, cnt;
    DEB(DEB_MULTI,
    printk(KERN_DEBUG
    "%s: set multicast list, %d entries, promisc %s, allmulti %s\n",
    dev.name, netdev_mc_count(dev),
    dev.flags & IFF_PROMISC ? "ON" : "OFF",
    dev.flags & IFF_ALLMULTI ? "ON" : "OFF"));
    if ((dev.flags & IFF_PROMISC) &&
    !(dma.cf_cmd.i596_config[8] & 0x01)) {
    dma.cf_cmd.i596_config[8] |= 0x01;
    config = 1;
    }
    if (!(dev.flags & IFF_PROMISC) &&
    (dma.cf_cmd.i596_config[8] & 0x01)) {
    dma.cf_cmd.i596_config[8] &= ~0x01;
    config = 1;
    }
    if ((dev.flags & IFF_ALLMULTI) &&
    (dma.cf_cmd.i596_config[11] & 0x20)) {
    dma.cf_cmd.i596_config[11] &= ~0x20;
    config = 1;
    }
    if (!(dev.flags & IFF_ALLMULTI) &&
    !(dma.cf_cmd.i596_config[11] & 0x20)) {
    dma.cf_cmd.i596_config[11] |= 0x20;
    config = 1;
    }
    if (config) {
    if (dma.cf_cmd.cmd.command)
    printk(KERN_INFO
    "%s: config change request already queued\n",
    dev.name);
    else {
    dma.cf_cmd.cmd.command = SWAP16(CmdConfigure);
    dma_sync_dev(dev, &dma.cf_cmd, sizeof(struct cf_cmd));
    i596_add_cmd(dev, &dma.cf_cmd.cmd);
    }
    }
    cnt = netdev_mc_count(dev);
    if (cnt > MAX_MC_CNT) {
    cnt = MAX_MC_CNT;
    printk(KERN_NOTICE "%s: Only %d multicast addresses supported",
    dev.name, cnt);
    }
    if (!netdev_mc_empty(dev)) {
    struct netdev_hw_addr *ha;
    unsigned char *cp;
    struct mc_cmd *cmd;
    cmd = &dma.mc_cmd;
    cmd.cmd.command = SWAP16(CmdMulticastList);
    cmd.mc_cnt = SWAP16(netdev_mc_count(dev) * 6);
    cp = cmd.mc_addrs;
    netdev_for_each_mc_addr(ha, dev) {
    if (!cnt--)
    break;
    memcpy(cp, ha.addr, ETH_ALEN);
    if (i596_debug > 1)
    DEB(DEB_MULTI,
    printk(KERN_DEBUG
    "%s: Adding address %pM\n",
    dev.name, cp));
    cp += ETH_ALEN;
    }
    dma_sync_dev(dev, &dma.mc_cmd, sizeof(struct mc_cmd));
    i596_add_cmd(dev, &cmd.cmd);
    }
    }
