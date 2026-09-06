//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/dec/tulip/pnic.c
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


//
    drivers/net/ethernet/dec/tulip/pnic.c
    Copyright 2000,2001  The Linux Kernel Team
    Written/copyright 1994-2001 by Donald Becker.
    This software may be used and distributed according to the terms
    of the GNU General Public License, incorporated herein by reference.
    Please submit bugs to http://bugzilla.kernel.org/ .
//

#[no_mangle]
pub unsafe extern "C" fn pnic_do_nway(dev: *mut net_device) {
    void pnic_do_nway(struct net_device *dev)
    {
    struct tulip_private *tp = netdev_priv(dev);
    void __iomem *ioaddr = tp.base_addr;
    let mut phy_reg: u32 = ioread32(ioaddr + 0xB8);
    u32 new_csr6;
    if (phy_reg & 0x78000000) { /* Ignore baseT4 */
    if (phy_reg & 0x20000000)		dev.if_port = 5;
    else if (phy_reg & 0x40000000)	dev.if_port = 3;
    else if (phy_reg & 0x10000000)	dev.if_port = 4;
    else if (phy_reg & 0x08000000)	dev.if_port = 0;
    tp.nwayset = 1;
    new_csr6 = (dev.if_port & 1) ? 0x01860000 : 0x00420000;
    iowrite32(0x32 | (dev.if_port & 1), ioaddr + CSR12);
    if (dev.if_port & 1)
    iowrite32(0x1F868, ioaddr + 0xB8);
    if (phy_reg & 0x30000000) {
    tp.full_duplex = 1;
    new_csr6 |= 0x00000200;
    }
    if (tulip_debug > 1)
    netdev_dbg(dev, "PNIC autonegotiated status %08x, %s\n",
    phy_reg, medianame[dev.if_port]);
    if (tp.csr6 != new_csr6) {
    tp.csr6 = new_csr6;
// Restart Tx
    tulip_restart_rxtx(tp);
    netif_trans_update(dev);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn pnic_lnk_change(dev: *mut net_device, csr5: c_int) {
    void pnic_lnk_change(struct net_device *dev, int csr5)
    {
    struct tulip_private *tp = netdev_priv(dev);
    void __iomem *ioaddr = tp.base_addr;
    let mut phy_reg: c_int = ioread32(ioaddr + 0xB8);
    if (tulip_debug > 1)
    netdev_dbg(dev, "PNIC link changed state %08x, CSR5 %08x\n",
    phy_reg, csr5);
    if (ioread32(ioaddr + CSR5) & TPLnkFail) {
    iowrite32((ioread32(ioaddr + CSR7) & ~TPLnkFail) | TPLnkPass, ioaddr + CSR7);
// If we use an external MII, then we mustn't use the
// internal negotiation.
//
    if (tulip_media_cap[dev.if_port] & MediaIsMII)
    return;
    if (! tp.nwayset || time_after(jiffies, dev_trans_start(dev) + 1*HZ)) {
    tp.csr6 = 0x00420000 | (tp.csr6 & 0x0000fdff);
    iowrite32(tp.csr6, ioaddr + CSR6);
    iowrite32(0x30, ioaddr + CSR12);
    iowrite32(0x0201F078, ioaddr + 0xB8); /* Turn on autonegotiation. */
    netif_trans_update(dev);
    }
    } else if (ioread32(ioaddr + CSR5) & TPLnkPass) {
    if (tulip_media_cap[dev.if_port] & MediaIsMII) {
    spin_lock(&tp.lock);
    tulip_check_duplex(dev);
    spin_unlock(&tp.lock);
    } else {
    pnic_do_nway(dev);
    }
    iowrite32((ioread32(ioaddr + CSR7) & ~TPLnkPass) | TPLnkFail, ioaddr + CSR7);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn pnic_timer(t: *mut timer_list) {
    void pnic_timer(struct timer_list *t)
    {
    struct tulip_private *tp = timer_container_of(tp, t, timer);
    struct net_device *dev = tp.dev;
    void __iomem *ioaddr = tp.base_addr;
    let mut next_tick: c_int = 60*HZ;
    if(!ioread32(ioaddr + CSR7)) {
// the timer was called due to a work overflow
// in the interrupt handler. Skip the connection
// checks, the nic is definitively speaking with
// his link partner.
//
    goto too_good_connection;
    }
    if (tulip_media_cap[dev.if_port] & MediaIsMII) {
    spin_lock_irq(&tp.lock);
    if (tulip_check_duplex(dev) > 0)
    next_tick = 3*HZ;
    spin_unlock_irq(&tp.lock);
    } else {
    let mut csr12: c_int = ioread32(ioaddr + CSR12);
    let mut new_csr6: c_int = tp.csr6 & ~0x40C40200;
    let mut phy_reg: c_int = ioread32(ioaddr + 0xB8);
    let mut csr5: c_int = ioread32(ioaddr + CSR5);
    if (tulip_debug > 1)
    netdev_dbg(dev, "PNIC timer PHY status %08x, %s CSR5 %08x\n",
    phy_reg, medianame[dev.if_port], csr5);
    if (phy_reg & 0x04000000) {	/* Remote link fault */
    iowrite32(0x0201F078, ioaddr + 0xB8);
    next_tick = 1*HZ;
    tp.nwayset = 0;
    } else if (phy_reg & 0x78000000) { /* Ignore baseT4 */
    pnic_do_nway(dev);
    next_tick = 60*HZ;
    } else if (csr5 & TPLnkFail) { /* 100baseTx link beat */
    if (tulip_debug > 1)
    netdev_dbg(dev, "%s link beat failed, CSR12 %04x, CSR5 %08x, PHY %03x\n",
    medianame[dev.if_port],
    csr12,
    ioread32(ioaddr + CSR5),
    ioread32(ioaddr + 0xB8));
    next_tick = 3*HZ;
    if (tp.medialock) {
    } else if (tp.nwayset  &&  (dev.if_port & 1)) {
    next_tick = 1*HZ;
    } else if (dev.if_port == 0) {
    dev.if_port = 3;
    iowrite32(0x33, ioaddr + CSR12);
    new_csr6 = 0x01860000;
    iowrite32(0x1F868, ioaddr + 0xB8);
    } else {
    dev.if_port = 0;
    iowrite32(0x32, ioaddr + CSR12);
    new_csr6 = 0x00420000;
    iowrite32(0x1F078, ioaddr + 0xB8);
    }
    if (tp.csr6 != new_csr6) {
    tp.csr6 = new_csr6;
// Restart Tx
    tulip_restart_rxtx(tp);
    netif_trans_update(dev);
    if (tulip_debug > 1)
    dev_info(&dev.dev,
    "Changing PNIC configuration to %s %s-duplex, CSR6 %08x\n",
    medianame[dev.if_port],
    tp.full_duplex ? "full" : "half",
    new_csr6);
    }
    }
    }
    too_good_connection:
    mod_timer(&tp.timer, RUN_AT(next_tick));
    if(!ioread32(ioaddr + CSR7)) {
    if (tulip_debug > 1)
    dev_info(&dev.dev, "sw timer wakeup\n");
    disable_irq(dev.irq);
    tulip_refill_rx(dev);
    enable_irq(dev.irq);
    iowrite32(tulip_tbl[tp.chip_id].valid_intrs, ioaddr + CSR7);
    }
    }
