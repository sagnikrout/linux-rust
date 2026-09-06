//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/i825xx/lasi_82596.c
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

pub const PA_CPU_PORT_L_ACCESS: c_int = 4;
pub const PA_CHANNEL_ATTENTION: c_int = 8;
pub const OPT_SWAP_PORT: c_uint = 0x0001	/* Need to wordswp on the MPU port */;
pub const SYSBUS: c_uint = 0x0000006c;
// big endian CPU, 82596 "big" endian mode

pub const NONCOHERENT_DMA: c_int = 1;

    MODULE_AUTHOR("Richard Hirst");
    MODULE_DESCRIPTION("i82596 driver");
    MODULE_LICENSE("GPL");
    module_param(i596_debug, int, 0);
    MODULE_PARM_DESC(i596_debug, "lasi_82596 debug mask");
#[no_mangle]
pub unsafe extern "C" fn ca(dev: *mut net_device) {
    static inline void ca(struct net_device *dev)
    {
    gsc_writel(0, dev.base_addr + PA_CHANNEL_ATTENTION);
    }
#[no_mangle]
unsafe extern "C" fn mpu_port(dev: *mut net_device, c: c_int, x: dma_addr_t) {
    static void mpu_port(struct net_device *dev, int c, dma_addr_t x)
    {
    struct i596_private *lp = netdev_priv(dev);
    let mut v: u32 = (u32) (c) | (u32) (x);
    u16 a, b;
    if (lp.options & OPT_SWAP_PORT) {
    a = v >> 16;
    b = v & 0xffff;
    } else {
    a = v & 0xffff;
    b = v >> 16;
    }
    gsc_writel(a, dev.base_addr + PA_CPU_PORT_L_ACCESS);
    if (!running_on_qemu)
    udelay(1);
    gsc_writel(b, dev.base_addr + PA_CPU_PORT_L_ACCESS);
    }
pub const LAN_PROM_ADDR: c_uint = 0xF0810000;
    static int __init
    lan_init_chip(struct parisc_device *dev)
    {
    struct	net_device *netdevice;
    struct i596_private *lp;
    let mut retval: c_int = -ENOMEM;
    u8 addr[ETH_ALEN];
    int i;
    if (!dev.irq) {
    printk(KERN_ERR "%s: IRQ not found for i82596 at 0x%lx\n",
    __FILE__, (unsigned long)dev.hpa.start);
    return -ENODEV;
    }
    printk(KERN_INFO "Found i82596 at 0x%lx, IRQ %d\n",
    (unsigned long)dev.hpa.start, dev.irq);
    netdevice = alloc_etherdev(sizeof(struct i596_private));
    if (!netdevice)
    return -ENOMEM;
    SET_NETDEV_DEV(netdevice, &dev.dev);
    parisc_set_drvdata (dev, netdevice);
    netdevice.base_addr = dev.hpa.start;
    netdevice.irq = dev.irq;
    if (pdc_lan_station_id(addr, netdevice.base_addr)) {
    for (i = 0; i < 6; i++) {
    addr[i] = gsc_readb(LAN_PROM_ADDR + i);
    }
    printk(KERN_INFO
    "%s: MAC of HP700 LAN read from EEPROM\n", __FILE__);
    }
    eth_hw_addr_set(netdevice, addr);
    lp = netdev_priv(netdevice);
    lp.options = dev.id.sversion == 0x72 ? OPT_SWAP_PORT : 0;
    lp.dma = dma_alloc_noncoherent(&dev.dev,
    sizeof(struct i596_dma), &lp.dma_addr,
    DMA_BIDIRECTIONAL, GFP_KERNEL);
    if (!lp.dma)
    goto out_free_netdev;
    retval = i82596_probe(netdevice);
    if (retval)
    goto out_free_dma;
    return 0;
    out_free_dma:
    dma_free_noncoherent(&dev.dev, sizeof(struct i596_dma),
    lp.dma, lp.dma_addr, DMA_BIDIRECTIONAL);
    out_free_netdev:
    free_netdev(netdevice);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn lan_remove_chip(pdev: *mut parisc_device) -> void __exit {
    static void __exit lan_remove_chip(struct parisc_device *pdev)
    {
    struct net_device *dev = parisc_get_drvdata(pdev);
    struct i596_private *lp = netdev_priv(dev);
    unregister_netdev (dev);
    dma_free_noncoherent(&pdev.dev, sizeof(struct i596_private), lp.dma,
    lp.dma_addr, DMA_BIDIRECTIONAL);
    free_netdev (dev);
    }
    static const struct parisc_device_id lan_tbl[] __initconst = {
    { HPHW_FIO, HVERSION_REV_ANY_ID, HVERSION_ANY_ID, 0x0008a },
    { HPHW_FIO, HVERSION_REV_ANY_ID, HVERSION_ANY_ID, 0x00072 },
    { 0, }
    };
    MODULE_DEVICE_TABLE(parisc, lan_tbl);
    static struct parisc_driver lan_driver __refdata = {
    .name		= "lasi_82596",
    .id_table	= lan_tbl,
    .probe		= lan_init_chip,
    .remove         = __exit_p(lan_remove_chip),
    };
#[no_mangle]
unsafe extern "C" fn lasi_82596_init() -> c_int {
    static int lasi_82596_init(void)
    {
    printk(KERN_INFO LASI_82596_DRIVER_VERSION "\n");
    return register_parisc_driver(&lan_driver);
    }
    module_init(lasi_82596_init);
#[no_mangle]
unsafe extern "C" fn lasi_82596_exit() -> void __exit {
    static void __exit lasi_82596_exit(void)
    {
    unregister_parisc_driver(&lan_driver);
    }
    module_exit(lasi_82596_exit);
