//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/i825xx/sni_82596.c
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
// sni_82596.c -- driver for intel 82596 ethernet controller, as
// used in older SNI RM machines
//

    static const char sni_82596_string[] = "snirm_82596";
pub const SYSBUS: c_uint = 0x00004400;
// big endian CPU, 82596 little endian

pub const OPT_MPU_16BIT: c_uint = 0x01;

    MODULE_AUTHOR("Thomas Bogendoerfer");
    MODULE_DESCRIPTION("i82596 driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:snirm_82596");
    module_param(i596_debug, int, 0);
    MODULE_PARM_DESC(i596_debug, "82596 debug mask");
#[no_mangle]
pub unsafe extern "C" fn ca(dev: *mut net_device) {
    static inline void ca(struct net_device *dev)
    {
    struct i596_private *lp = netdev_priv(dev);
    writel(0, lp.ca);
    }
#[no_mangle]
unsafe extern "C" fn mpu_port(dev: *mut net_device, c: c_int, x: dma_addr_t) {
    static void mpu_port(struct net_device *dev, int c, dma_addr_t x)
    {
    struct i596_private *lp = netdev_priv(dev);
    let mut v: u32 = (u32) (c) | (u32) (x);
    if (lp.options & OPT_MPU_16BIT) {
    writew(v & 0xffff, lp.mpu_port);
    wmb();  /* order writes to MPU port */
    udelay(1);
    writew(v >> 16, lp.mpu_port);
    } else {
    writel(v, lp.mpu_port);
    wmb();  /* order writes to MPU port */
    udelay(1);
    writel(v, lp.mpu_port);
    }
    }
#[no_mangle]
unsafe extern "C" fn sni_82596_probe(dev: *mut platform_device) -> c_int {
    static int sni_82596_probe(struct platform_device *dev)
    {
    struct	net_device *netdevice;
    struct i596_private *lp;
    struct  resource *res, *ca, *idprom, *options;
    let mut retval: c_int = -ENOMEM;
    void __iomem *mpu_addr;
    void __iomem *ca_addr;
    u8 __iomem *eth_addr;
    u8 mac[ETH_ALEN];
    res = platform_get_resource(dev, IORESOURCE_MEM, 0);
    ca = platform_get_resource(dev, IORESOURCE_MEM, 1);
    options = platform_get_resource(dev, 0, 0);
    idprom = platform_get_resource(dev, IORESOURCE_MEM, 2);
    if (!res || !ca || !options || !idprom)
    return -ENODEV;
    mpu_addr = ioremap(res.start, 4);
    if (!mpu_addr)
    return -ENOMEM;
    ca_addr = ioremap(ca.start, 4);
    if (!ca_addr)
    goto probe_failed_free_mpu;
    printk(KERN_INFO "Found i82596 at 0x%x\n", res.start);
    netdevice = alloc_etherdev(sizeof(struct i596_private));
    if (!netdevice)
    goto probe_failed_free_ca;
    SET_NETDEV_DEV(netdevice, &dev.dev);
    platform_set_drvdata (dev, netdevice);
    netdevice.base_addr = res.start;
    netdevice.irq = platform_get_irq(dev, 0);
    eth_addr = ioremap(idprom.start, 0x10);
    if (!eth_addr)
    goto probe_failed;
// someone seems to like messed up stuff
    mac[0] = readb(eth_addr + 0x0b);
    mac[1] = readb(eth_addr + 0x0a);
    mac[2] = readb(eth_addr + 0x09);
    mac[3] = readb(eth_addr + 0x08);
    mac[4] = readb(eth_addr + 0x07);
    mac[5] = readb(eth_addr + 0x06);
    eth_hw_addr_set(netdevice, mac);
    iounmap(eth_addr);
    if (netdevice.irq < 0) {
    printk(KERN_ERR "%s: IRQ not found for i82596 at 0x%lx\n",
    __FILE__, netdevice.base_addr);
    retval = netdevice.irq;
    goto probe_failed;
    }
    lp = netdev_priv(netdevice);
    lp.options = options.flags & IORESOURCE_BITS;
    lp.ca = ca_addr;
    lp.mpu_port = mpu_addr;
    lp.dma = dma_alloc_coherent(&dev.dev, sizeof(struct i596_dma),
    &lp.dma_addr, GFP_KERNEL);
    if (!lp.dma)
    goto probe_failed;
    retval = i82596_probe(netdevice);
    if (retval)
    goto probe_failed_free_dma;
    return 0;
    probe_failed_free_dma:
    dma_free_coherent(&dev.dev, sizeof(struct i596_dma), lp.dma,
    lp.dma_addr);
    probe_failed:
    free_netdev(netdevice);
    probe_failed_free_ca:
    iounmap(ca_addr);
    probe_failed_free_mpu:
    iounmap(mpu_addr);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn sni_82596_driver_remove(pdev: *mut platform_device) {
    static void sni_82596_driver_remove(struct platform_device *pdev)
    {
    struct net_device *dev = platform_get_drvdata(pdev);
    struct i596_private *lp = netdev_priv(dev);
    unregister_netdev(dev);
    dma_free_coherent(&pdev.dev, sizeof(struct i596_private), lp.dma,
    lp.dma_addr);
    iounmap(lp.ca);
    iounmap(lp.mpu_port);
    free_netdev (dev);
    }
    static struct platform_driver sni_82596_driver = {
    .probe	= sni_82596_probe,
    .remove = sni_82596_driver_remove,
    .driver	= {
    .name	= sni_82596_string,
    },
    };
#[no_mangle]
unsafe extern "C" fn sni_82596_init() -> c_int {
    static int sni_82596_init(void)
    {
    printk(KERN_INFO SNI_82596_DRIVER_VERSION "\n");
    return platform_driver_register(&sni_82596_driver);
    }
#[no_mangle]
unsafe extern "C" fn sni_82596_exit() -> void __exit {
    static void __exit sni_82596_exit(void)
    {
    platform_driver_unregister(&sni_82596_driver);
    }
    module_init(sni_82596_init);
    module_exit(sni_82596_exit);
