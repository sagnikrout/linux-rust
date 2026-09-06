//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/8390/ax88796.c
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
// drivers/net/ethernet/8390/ax88796.c
//
// Copyright 2005,2007 Simtec Electronics
// Ben Dooks <ben@simtec.co.uk>
//
// Asix AX88796 10/100 Ethernet controller support
// Based on ne.c, by Donald Becker, et-al.
//

// Rename the lib8390.c functions to show that they are in this driver

// force unsigned long back to 'void __iomem *'

// define EI_SHIFT() to take into account our register offsets

// Ensure we have our RCR base value
// Macro flag: #define AX88796_PLATFORM
    static unsigned char version[] = "ax88796.c: Copyright 2005,2007 Simtec Electronics\n";

// from ne.c

pub const NE1SM_START_PG: c_uint = 0x20	/* First page of TX buffer */;
pub const NE1SM_STOP_PG: c_uint = 0x40	/* Last page +1 of RX ring */;
pub const NESM_START_PG: c_uint = 0x40	/* First page of TX buffer */;
pub const NESM_STOP_PG: c_uint = 0x80	/* Last page +1 of RX ring */;

// device private data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ax_device {
    pub mii_bus: *mut mii_bus,
    pub bb_ctrl: mdiobb_ctrl,
    pub addr_memr: *mut void __iomem,
    pub reg_memr: u8,
    pub link: c_int,
    pub speed: c_int,
    pub duplex: c_int,
    pub map2: *mut void __iomem,
    pub plat: *const ax_plat_data,
    pub running: c_uchar,
    pub resume_open: c_uchar,
    pub irqflags: c_uint,
    pub reg_offsets: [u32; 0x20],
}

    static inline struct ax_device *to_ax_dev(struct net_device *dev)
    {
    struct ei_device *ei_local = netdev_priv(dev);
    return (struct ax_device *)(ei_local + 1);
    }
#[no_mangle]
pub unsafe extern "C" fn ax_NS8390_reinit(dev: *mut net_device) {
    void ax_NS8390_reinit(struct net_device *dev)
    {
    ax_NS8390_init(dev, 1);
    }
    EXPORT_SYMBOL_GPL(ax_NS8390_reinit);
//
// ax_initial_check
//
// do an initial probe for the card to check whether it exists
// and is functional
//
#[no_mangle]
unsafe extern "C" fn ax_initial_check(dev: *mut net_device) -> c_int {
    static int ax_initial_check(struct net_device *dev)
    {
    struct ei_device *ei_local = netdev_priv(dev);
    void __iomem *ioaddr = ei_local.mem;
    int reg0;
    int regd;
    reg0 = ei_inb(ioaddr);
    if (reg0 == 0xFF)
    return -ENODEV;
    ei_outb(E8390_NODMA + E8390_PAGE1 + E8390_STOP, ioaddr + E8390_CMD);
    regd = ei_inb(ioaddr + 0x0d);
    ei_outb(0xff, ioaddr + 0x0d);
    ei_outb(E8390_NODMA + E8390_PAGE0, ioaddr + E8390_CMD);
    ei_inb(ioaddr + EN0_COUNTER0); /* Clear the counter by reading. */
    if (ei_inb(ioaddr + EN0_COUNTER0) != 0) {
    ei_outb(reg0, ioaddr);
    ei_outb(regd, ioaddr + 0x0d);	/* Restore the old values. */
    return -ENODEV;
    }
    return 0;
    }
//
// Hard reset the card. This used to pause for the same period that a
// 8390 reset command required, but that shouldn't be necessary.
//
#[no_mangle]
unsafe extern "C" fn ax_reset_8390(dev: *mut net_device) {
    static void ax_reset_8390(struct net_device *dev)
    {
    struct ei_device *ei_local = netdev_priv(dev);
    let mut reset_start_time: c_ulong = jiffies;
    void __iomem *addr = (void __iomem *)dev.base_addr;
    netif_dbg(ei_local, hw, dev, "resetting the 8390 t=%ld...\n", jiffies);
    ei_outb(ei_inb(addr + NE_RESET), addr + NE_RESET);
    ei_local.txing = 0;
    ei_local.dmaing = 0;
// This check _should_not_ be necessary, omit eventually.
    while ((ei_inb(addr + EN0_ISR) & ENISR_RESET) == 0) {
    if (time_after(jiffies, reset_start_time + 2 * HZ / 100)) {
    netdev_warn(dev, "%s: did not complete.\n", __func__);
    break;
    }
    }
    ei_outb(ENISR_RESET, addr + EN0_ISR);	/* Ack intr. */
    }
// Wrapper for __ei_interrupt for platforms that have a platform-specific
// way to find out whether the interrupt request might be caused by
// the ax88796 chip.
//
#[no_mangle]
unsafe extern "C" fn ax_ei_interrupt_filtered(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t ax_ei_interrupt_filtered(int irq, void *dev_id)
    {
    struct net_device *dev = dev_id;
    struct ax_device *ax = to_ax_dev(dev);
    struct platform_device *pdev = to_platform_device(dev.dev.parent);
    if (!ax.plat.check_irq(pdev))
    return IRQ_NONE;
    return ax_ei_interrupt(irq, dev_id);
    }
    static void ax_get_8390_hdr(struct net_device *dev, struct e8390_pkt_hdr *hdr,
    int ring_page)
    {
    struct ei_device *ei_local = netdev_priv(dev);
    void __iomem *nic_base = ei_local.mem;
// This *shouldn't* happen. If it does, it's the last thing you'll see
    if (ei_local.dmaing) {
    netdev_err(dev, "DMAing conflict in %s "
    "[DMAstat:%d][irqlock:%d].\n",
    __func__,
    ei_local.dmaing, ei_local.irqlock);
    return;
    }
    ei_local.dmaing |= 0x01;
    ei_outb(E8390_NODMA + E8390_PAGE0 + E8390_START, nic_base + NE_CMD);
    ei_outb(sizeof(struct e8390_pkt_hdr), nic_base + EN0_RCNTLO);
    ei_outb(0, nic_base + EN0_RCNTHI);
    ei_outb(0, nic_base + EN0_RSARLO);		/* On page boundary */
    ei_outb(ring_page, nic_base + EN0_RSARHI);
    ei_outb(E8390_RREAD+E8390_START, nic_base + NE_CMD);
    if (ei_local.word16)
    ioread16_rep(nic_base + NE_DATAPORT, hdr,
    sizeof(struct e8390_pkt_hdr) >> 1);
    else
    ioread8_rep(nic_base + NE_DATAPORT, hdr,
    sizeof(struct e8390_pkt_hdr));
    ei_outb(ENISR_RDC, nic_base + EN0_ISR);	/* Ack intr. */
    ei_local.dmaing &= ~0x01;
    le16_to_cpus(&hdr.count);
    }
//
// Block input and output, similar to the Crynwr packet driver. If
// you are porting to a new ethercard, look at the packet driver
// source for hints. The NEx000 doesn't share the on-board packet
// memory -- you have to put the packet out through the "remote DMA"
// dataport using ei_outb.
//
    static void ax_block_input(struct net_device *dev, int count,
    struct sk_buff *skb, int ring_offset)
    {
    struct ei_device *ei_local = netdev_priv(dev);
    void __iomem *nic_base = ei_local.mem;
    char *buf = skb.data;
    if (ei_local.dmaing) {
    netdev_err(dev,
    "DMAing conflict in %s "
    "[DMAstat:%d][irqlock:%d].\n",
    __func__,
    ei_local.dmaing, ei_local.irqlock);
    return;
    }
    ei_local.dmaing |= 0x01;
    ei_outb(E8390_NODMA+E8390_PAGE0+E8390_START, nic_base + NE_CMD);
    ei_outb(count & 0xff, nic_base + EN0_RCNTLO);
    ei_outb(count >> 8, nic_base + EN0_RCNTHI);
    ei_outb(ring_offset & 0xff, nic_base + EN0_RSARLO);
    ei_outb(ring_offset >> 8, nic_base + EN0_RSARHI);
    ei_outb(E8390_RREAD+E8390_START, nic_base + NE_CMD);
    if (ei_local.word16) {
    ioread16_rep(nic_base + NE_DATAPORT, buf, count >> 1);
    if (count & 0x01)
    buf[count-1] = ei_inb(nic_base + NE_DATAPORT);
    } else {
    ioread8_rep(nic_base + NE_DATAPORT, buf, count);
    }
    ei_local.dmaing &= ~1;
    }
    static void ax_block_output(struct net_device *dev, int count,
    const unsigned char *buf, const int start_page)
    {
    struct ei_device *ei_local = netdev_priv(dev);
    void __iomem *nic_base = ei_local.mem;
    unsigned long dma_start;
//
// Round the count up for word writes. Do we need to do this?
// What effect will an odd byte count have on the 8390?  I
// should check someday.
//
    if (ei_local.word16 && (count & 0x01))
    count++;
// This *shouldn't* happen. If it does, it's the last thing you'll see
    if (ei_local.dmaing) {
    netdev_err(dev, "DMAing conflict in %s."
    "[DMAstat:%d][irqlock:%d]\n",
    __func__,
    ei_local.dmaing, ei_local.irqlock);
    return;
    }
    ei_local.dmaing |= 0x01;
// We should already be in page 0, but to be safe...
    ei_outb(E8390_PAGE0+E8390_START+E8390_NODMA, nic_base + NE_CMD);
    ei_outb(ENISR_RDC, nic_base + EN0_ISR);
// Now the normal output.
    ei_outb(count & 0xff, nic_base + EN0_RCNTLO);
    ei_outb(count >> 8, nic_base + EN0_RCNTHI);
    ei_outb(0x00, nic_base + EN0_RSARLO);
    ei_outb(start_page, nic_base + EN0_RSARHI);
    ei_outb(E8390_RWRITE+E8390_START, nic_base + NE_CMD);
    if (ei_local.word16)
    iowrite16_rep(nic_base + NE_DATAPORT, buf, count >> 1);
    else
    iowrite8_rep(nic_base + NE_DATAPORT, buf, count);
    dma_start = jiffies;
    while ((ei_inb(nic_base + EN0_ISR) & ENISR_RDC) == 0) {
    if (time_after(jiffies, dma_start + 2 * HZ / 100)) { /* 20ms */
    netdev_warn(dev, "timeout waiting for Tx RDC.\n");
    ax_reset_8390(dev);
    ax_NS8390_init(dev, 1);
    break;
    }
    }
    ei_outb(ENISR_RDC, nic_base + EN0_ISR);	/* Ack intr. */
    ei_local.dmaing &= ~0x01;
    }
// definitions for accessing MII/EEPROM interface

#[no_mangle]
unsafe extern "C" fn ax_handle_link_change(dev: *mut net_device) {
    static void ax_handle_link_change(struct net_device *dev)
    {
    struct ax_device  *ax = to_ax_dev(dev);
    struct phy_device *phy_dev = dev.phydev;
    let mut status_change: c_int = 0;
    if (phy_dev.link && ((ax.speed != phy_dev.speed) ||
    (ax.duplex != phy_dev.duplex))) {
    ax.speed = phy_dev.speed;
    ax.duplex = phy_dev.duplex;
    status_change = 1;
    }
    if (phy_dev.link != ax.link) {
    if (!phy_dev.link) {
    ax.speed = 0;
    ax.duplex = -1;
    }
    ax.link = phy_dev.link;
    status_change = 1;
    }
    if (status_change)
    phy_print_status(phy_dev);
    }
#[no_mangle]
unsafe extern "C" fn ax_mii_probe(dev: *mut net_device) -> c_int {
    static int ax_mii_probe(struct net_device *dev)
    {
    struct ax_device  *ax = to_ax_dev(dev);
    struct phy_device *phy_dev = core::ptr::null_mut();
    int ret;
// find the first phy
    phy_dev = phy_find_first(ax.mii_bus);
    if (!phy_dev) {
    netdev_err(dev, "no PHY found\n");
    return -ENODEV;
    }
    ret = phy_connect_direct(dev, phy_dev, ax_handle_link_change,
    PHY_INTERFACE_MODE_MII);
    if (ret) {
    netdev_err(dev, "Could not attach to PHY\n");
    return ret;
    }
    phy_set_max_speed(phy_dev, SPEED_100);
    netdev_info(dev, "PHY driver [%s] (mii_bus:phy_addr=%s, irq=%d)\n",
    phy_dev.drv.name, phydev_name(phy_dev), phy_dev.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ax_phy_switch(dev: *mut net_device, on: c_int) {
    static void ax_phy_switch(struct net_device *dev, int on)
    {
    struct ei_device *ei_local = netdev_priv(dev);
    struct ax_device *ax = to_ax_dev(dev);
    let mut reg_gpoc: u8 = ax.plat.gpoc_val;
    if (!!on)
    reg_gpoc &= ~AX_GPOC_PPDSET;
    else
    reg_gpoc |= AX_GPOC_PPDSET;
    ei_outb(reg_gpoc, ei_local.mem + EI_SHIFT(0x17));
    }
#[no_mangle]
unsafe extern "C" fn ax_bb_mdc(ctrl: *mut mdiobb_ctrl, level: c_int) {
    static void ax_bb_mdc(struct mdiobb_ctrl *ctrl, int level)
    {
    struct ax_device *ax = container_of(ctrl, struct ax_device, bb_ctrl);
    if (level)
    ax.reg_memr |= AX_MEMR_MDC;
    else
    ax.reg_memr &= ~AX_MEMR_MDC;
    ei_outb(ax.reg_memr, ax.addr_memr);
    }
#[no_mangle]
unsafe extern "C" fn ax_bb_dir(ctrl: *mut mdiobb_ctrl, output: c_int) {
    static void ax_bb_dir(struct mdiobb_ctrl *ctrl, int output)
    {
    struct ax_device *ax = container_of(ctrl, struct ax_device, bb_ctrl);
    if (output)
    ax.reg_memr &= ~AX_MEMR_MDIR;
    else
    ax.reg_memr |= AX_MEMR_MDIR;
    ei_outb(ax.reg_memr, ax.addr_memr);
    }
#[no_mangle]
unsafe extern "C" fn ax_bb_set_data(ctrl: *mut mdiobb_ctrl, value: c_int) {
    static void ax_bb_set_data(struct mdiobb_ctrl *ctrl, int value)
    {
    struct ax_device *ax = container_of(ctrl, struct ax_device, bb_ctrl);
    if (value)
    ax.reg_memr |= AX_MEMR_MDO;
    else
    ax.reg_memr &= ~AX_MEMR_MDO;
    ei_outb(ax.reg_memr, ax.addr_memr);
    }
#[no_mangle]
unsafe extern "C" fn ax_bb_get_data(ctrl: *mut mdiobb_ctrl) -> c_int {
    static int ax_bb_get_data(struct mdiobb_ctrl *ctrl)
    {
    struct ax_device *ax = container_of(ctrl, struct ax_device, bb_ctrl);
    let mut reg_memr: c_int = ei_inb(ax.addr_memr);
    return reg_memr & AX_MEMR_MDI ? 1 : 0;
    }
    static const struct mdiobb_ops bb_ops = {
    .owner = THIS_MODULE,
    .set_mdc = ax_bb_mdc,
    .set_mdio_dir = ax_bb_dir,
    .set_mdio_data = ax_bb_set_data,
    .get_mdio_data = ax_bb_get_data,
    };
#[no_mangle]
unsafe extern "C" fn ax_mii_init(dev: *mut net_device) -> c_int {
    static int ax_mii_init(struct net_device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev.dev.parent);
    struct ei_device *ei_local = netdev_priv(dev);
    struct ax_device *ax = to_ax_dev(dev);
    int err;
    ax.bb_ctrl.ops = &bb_ops;
    ax.addr_memr = ei_local.mem + AX_MEMR;
    ax.mii_bus = alloc_mdio_bitbang(&ax.bb_ctrl);
    if (!ax.mii_bus) {
    err = -ENOMEM;
    goto out;
    }
    ax.mii_bus.name = "ax88796_mii_bus";
    ax.mii_bus.parent = dev.dev.parent;
    snprintf(ax.mii_bus.id, MII_BUS_ID_SIZE, "%s-%x",
    pdev.name, pdev.id);
    err = mdiobus_register(ax.mii_bus);
    if (err)
    goto out_free_mdio_bitbang;
    return 0;
    out_free_mdio_bitbang:
    free_mdio_bitbang(ax.mii_bus);
    out:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ax_open(dev: *mut net_device) -> c_int {
    static int ax_open(struct net_device *dev)
    {
    struct ax_device *ax = to_ax_dev(dev);
    int ret;
    netdev_dbg(dev, "open\n");
    ret = ax_mii_init(dev);
    if (ret)
    goto failed_mii;
    if (ax.plat.check_irq)
    ret = request_irq(dev.irq, ax_ei_interrupt_filtered,
    ax.irqflags, dev.name, dev);
    else
    ret = request_irq(dev.irq, ax_ei_interrupt, ax.irqflags,
    dev.name, dev);
    if (ret)
    goto failed_request_irq;
// turn the phy on (if turned off)
    ax_phy_switch(dev, 1);
    ret = ax_mii_probe(dev);
    if (ret)
    goto failed_mii_probe;
    phy_start(dev.phydev);
    ret = ax_ei_open(dev);
    if (ret)
    goto failed_ax_ei_open;
    ax.running = 1;
    return 0;
    failed_ax_ei_open:
    phy_disconnect(dev.phydev);
    failed_mii_probe:
    ax_phy_switch(dev, 0);
    free_irq(dev.irq, dev);
    failed_request_irq:
// unregister mdiobus
    mdiobus_unregister(ax.mii_bus);
    free_mdio_bitbang(ax.mii_bus);
    failed_mii:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ax_close(dev: *mut net_device) -> c_int {
    static int ax_close(struct net_device *dev)
    {
    struct ax_device *ax = to_ax_dev(dev);
    netdev_dbg(dev, "close\n");
    ax.running = 0;
    wmb();
    ax_ei_close(dev);
// turn the phy off
    ax_phy_switch(dev, 0);
    phy_disconnect(dev.phydev);
    free_irq(dev.irq, dev);
    mdiobus_unregister(ax.mii_bus);
    free_mdio_bitbang(ax.mii_bus);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ax_ioctl(dev: *mut net_device, req: *mut ifreq, cmd: c_int) -> c_int {
    static int ax_ioctl(struct net_device *dev, struct ifreq *req, int cmd)
    {
    struct phy_device *phy_dev = dev.phydev;
    if (!netif_running(dev))
    return -EINVAL;
    if (!phy_dev)
    return -ENODEV;
    return phy_mii_ioctl(phy_dev, req, cmd);
    }
// ethtool ops
    static void ax_get_drvinfo(struct net_device *dev,
    struct ethtool_drvinfo *info)
    {
    struct platform_device *pdev = to_platform_device(dev.dev.parent);
    strscpy(info.driver, DRV_NAME, sizeof(info.driver));
    strscpy(info.version, DRV_VERSION, sizeof(info.version));
    strscpy(info.bus_info, pdev.name, sizeof(info.bus_info));
    }
#[no_mangle]
unsafe extern "C" fn ax_get_msglevel(dev: *mut net_device) -> u32 {
    static u32 ax_get_msglevel(struct net_device *dev)
    {
    struct ei_device *ei_local = netdev_priv(dev);
    return ei_local.msg_enable;
    }
#[no_mangle]
unsafe extern "C" fn ax_set_msglevel(dev: *mut net_device, v: u32) {
    static void ax_set_msglevel(struct net_device *dev, u32 v)
    {
    struct ei_device *ei_local = netdev_priv(dev);
    ei_local.msg_enable = v;
    }
    static const struct ethtool_ops ax_ethtool_ops = {
    .get_drvinfo		= ax_get_drvinfo,
    .get_link		= ethtool_op_get_link,
    .get_ts_info		= ethtool_op_get_ts_info,
    .get_msglevel		= ax_get_msglevel,
    .set_msglevel		= ax_set_msglevel,
    .get_link_ksettings	= phy_ethtool_get_link_ksettings,
    .set_link_ksettings	= phy_ethtool_set_link_ksettings,
    };

#[no_mangle]
unsafe extern "C" fn ax_eeprom_register_read(eeprom: *mut eeprom_93cx6) {
    static void ax_eeprom_register_read(struct eeprom_93cx6 *eeprom)
    {
    struct ei_device *ei_local = eeprom.data;
    let mut reg: u8 = ei_inb(ei_local.mem + AX_MEMR);
    eeprom.reg_data_in = reg & AX_MEMR_EEI;
    eeprom.reg_data_out = reg & AX_MEMR_EEO; /* Input pin */
    eeprom.reg_data_clock = reg & AX_MEMR_EECLK;
    eeprom.reg_chip_select = reg & AX_MEMR_EECS;
    }
#[no_mangle]
unsafe extern "C" fn ax_eeprom_register_write(eeprom: *mut eeprom_93cx6) {
    static void ax_eeprom_register_write(struct eeprom_93cx6 *eeprom)
    {
    struct ei_device *ei_local = eeprom.data;
    let mut reg: u8 = ei_inb(ei_local.mem + AX_MEMR);
    reg &= ~(AX_MEMR_EEI | AX_MEMR_EECLK | AX_MEMR_EECS);
    if (eeprom.reg_data_in)
    reg |= AX_MEMR_EEI;
    if (eeprom.reg_data_clock)
    reg |= AX_MEMR_EECLK;
    if (eeprom.reg_chip_select)
    reg |= AX_MEMR_EECS;
    ei_outb(reg, ei_local.mem + AX_MEMR);
    udelay(10);
    }

    static const struct net_device_ops ax_netdev_ops = {
    .ndo_open		= ax_open,
    .ndo_stop		= ax_close,
    .ndo_eth_ioctl		= ax_ioctl,
    .ndo_start_xmit		= ax_ei_start_xmit,
    .ndo_tx_timeout		= ax_ei_tx_timeout,
    .ndo_get_stats		= ax_ei_get_stats,
    .ndo_set_rx_mode	= ax_ei_set_multicast_list,
    .ndo_validate_addr	= eth_validate_addr,
    .ndo_set_mac_address	= eth_mac_addr,

    .ndo_poll_controller	= ax_ei_poll,

    };
// setup code
#[no_mangle]
unsafe extern "C" fn ax_initial_setup(dev: *mut net_device, ei_local: *mut ei_device) {
    static void ax_initial_setup(struct net_device *dev, struct ei_device *ei_local)
    {
    void __iomem *ioaddr = ei_local.mem;
    struct ax_device *ax = to_ax_dev(dev);
// Select page 0
    ei_outb(E8390_NODMA + E8390_PAGE0 + E8390_STOP, ioaddr + E8390_CMD);
// set to byte access
    ei_outb(ax.plat.dcr_val & ~1, ioaddr + EN0_DCFG);
    ei_outb(ax.plat.gpoc_val, ioaddr + EI_SHIFT(0x17));
    }
//
// ax_init_dev
//
// initialise the specified device, taking care to note the MAC
// address it may already have (if configured), ensure
// the device is ready to be used by lib8390.c and registerd with
// the network layer.
//
#[no_mangle]
unsafe extern "C" fn ax_init_dev(dev: *mut net_device) -> c_int {
    static int ax_init_dev(struct net_device *dev)
    {
    struct ei_device *ei_local = netdev_priv(dev);
    struct ax_device *ax = to_ax_dev(dev);
    void __iomem *ioaddr = ei_local.mem;
    unsigned int start_page;
    unsigned int stop_page;
    int ret;
    int i;
    ret = ax_initial_check(dev);
    if (ret)
    goto err_out;
// setup goes here
    ax_initial_setup(dev, ei_local);
// read the mac from the card prom if we need it
    if (ax.plat.flags & AXFLG_HAS_EEPROM) {
    unsigned char SA_prom[32];
    ei_outb(6, ioaddr + EN0_RCNTLO);
    ei_outb(0, ioaddr + EN0_RCNTHI);
    ei_outb(0, ioaddr + EN0_RSARLO);
    ei_outb(0, ioaddr + EN0_RSARHI);
    ei_outb(E8390_RREAD + E8390_START, ioaddr + NE_CMD);
    for (i = 0; i < sizeof(SA_prom); i += 2) {
    SA_prom[i] = ei_inb(ioaddr + NE_DATAPORT);
    SA_prom[i + 1] = ei_inb(ioaddr + NE_DATAPORT);
    }
    ei_outb(ENISR_RDC, ioaddr + EN0_ISR);	/* Ack intr. */
    if (ax.plat.wordlength == 2)
    for (i = 0; i < 16; i++)
    SA_prom[i] = SA_prom[i+i];
    eth_hw_addr_set(dev, SA_prom);
    }

    if (ax.plat.flags & AXFLG_HAS_93CX6) {
    unsigned char mac_addr[ETH_ALEN];
    struct eeprom_93cx6 eeprom;
    eeprom.data = ei_local;
    eeprom.register_read = ax_eeprom_register_read;
    eeprom.register_write = ax_eeprom_register_write;
    eeprom.width = PCI_EEPROM_WIDTH_93C56;
    eeprom_93cx6_multiread(&eeprom, 0,
    (__le16  *)mac_addr,
    sizeof(mac_addr) >> 1);
    eth_hw_addr_set(dev, mac_addr);
    }

    if (ax.plat.wordlength == 2) {
// We must set the 8390 for word mode.
    ei_outb(ax.plat.dcr_val, ei_local.mem + EN0_DCFG);
    start_page = NESM_START_PG;
    stop_page = NESM_STOP_PG;
    } else {
    start_page = NE1SM_START_PG;
    stop_page = NE1SM_STOP_PG;
    }
// load the mac-address from the device
    if (ax.plat.flags & AXFLG_MAC_FROMDEV) {
    u8 addr[ETH_ALEN];
    ei_outb(E8390_NODMA + E8390_PAGE1 + E8390_STOP,
    ei_local.mem + E8390_CMD); /* 0x61 */
    for (i = 0; i < ETH_ALEN; i++)
    addr[i] = ei_inb(ioaddr + EN1_PHYS_SHIFT(i));
    eth_hw_addr_set(dev, addr);
    }
    if ((ax.plat.flags & AXFLG_MAC_FROMPLATFORM) &&
    ax.plat.mac_addr)
    eth_hw_addr_set(dev, ax.plat.mac_addr);
    if (!is_valid_ether_addr(dev.dev_addr)) {
    eth_hw_addr_random(dev);
    dev_info(&dev.dev, "Using random MAC address: %pM\n",
    dev.dev_addr);
    }
    ax_reset_8390(dev);
    ei_local.name = "AX88796";
    ei_local.tx_start_page = start_page;
    ei_local.stop_page = stop_page;
    ei_local.word16 = (ax.plat.wordlength == 2);
    ei_local.rx_start_page = start_page + TX_PAGES;

// Allow the packet buffer size to be overridden by know-it-alls.
    ei_local.stop_page = ei_local.tx_start_page + PACKETBUF_MEMSIZE;

    ei_local.reset_8390 = &ax_reset_8390;
    if (ax.plat.block_input)
    ei_local.block_input = ax.plat.block_input;
    else
    ei_local.block_input = &ax_block_input;
    if (ax.plat.block_output)
    ei_local.block_output = ax.plat.block_output;
    else
    ei_local.block_output = &ax_block_output;
    ei_local.get_8390_hdr = &ax_get_8390_hdr;
    ei_local.priv = 0;
    dev.netdev_ops = &ax_netdev_ops;
    dev.ethtool_ops = &ax_ethtool_ops;
    ax_NS8390_init(dev, 0);
    ret = register_netdev(dev);
    if (ret)
    goto err_out;
    netdev_info(dev, "%dbit, irq %d, %lx, MAC: %pM\n",
    ei_local.word16 ? 16 : 8, dev.irq, dev.base_addr,
    dev.dev_addr);
    return 0;
    err_out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ax_remove(pdev: *mut platform_device) {
    static void ax_remove(struct platform_device *pdev)
    {
    struct net_device *dev = platform_get_drvdata(pdev);
    struct ei_device *ei_local = netdev_priv(dev);
    struct ax_device *ax = to_ax_dev(dev);
    struct resource *mem;
    unregister_netdev(dev);
    iounmap(ei_local.mem);
    mem = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    release_mem_region(mem.start, resource_size(mem));
    if (ax.map2) {
    iounmap(ax.map2);
    mem = platform_get_resource(pdev, IORESOURCE_MEM, 1);
    release_mem_region(mem.start, resource_size(mem));
    }
    platform_set_drvdata(pdev, core::ptr::null_mut());
    free_netdev(dev);
    }
//
// ax_probe
//
// This is the entry point when the platform device system uses to
// notify us of a new device to attach to. Allocate memory, find the
// resources and information passed, and map the necessary registers.
//
#[no_mangle]
unsafe extern "C" fn ax_probe(pdev: *mut platform_device) -> c_int {
    static int ax_probe(struct platform_device *pdev)
    {
    struct net_device *dev;
    struct ei_device *ei_local;
    struct ax_device *ax;
    struct resource *irq, *mem, *mem2;
    unsigned long mem_size, mem2_size = 0;
    let mut ret: c_int = 0;
    dev = ax__alloc_ei_netdev(sizeof(struct ax_device));
    if (dev == core::ptr::null_mut())
    return -ENOMEM;
// ok, let's setup our device
    SET_NETDEV_DEV(dev, &pdev.dev);
    ei_local = netdev_priv(dev);
    ax = to_ax_dev(dev);
    ax.plat = dev_get_platdata(&pdev.dev);
    platform_set_drvdata(pdev, dev);
    ei_local.rxcr_base = ax.plat.rcr_val;
// find the platform resources
    irq = platform_get_resource(pdev, IORESOURCE_IRQ, 0);
    if (!irq) {
    dev_err(&pdev.dev, "no IRQ specified\n");
    ret = -ENXIO;
    goto exit_mem;
    }
    dev.irq = irq.start;
    ax.irqflags = irq.flags & IRQF_TRIGGER_MASK;
    if (irq.flags &  IORESOURCE_IRQ_SHAREABLE)
    ax.irqflags |= IRQF_SHARED;
    mem = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!mem) {
    dev_err(&pdev.dev, "no MEM specified\n");
    ret = -ENXIO;
    goto exit_mem;
    }
    mem_size = resource_size(mem);
//
// setup the register offsets from either the platform data or
// by using the size of the resource provided
//
    if (ax.plat.reg_offsets)
    ei_local.reg_offset = ax.plat.reg_offsets;
    else {
    ei_local.reg_offset = ax.reg_offsets;
    for (ret = 0; ret < 0x18; ret++)
    ax.reg_offsets[ret] = (mem_size / 0x18) * ret;
    }
    if (!request_mem_region(mem.start, mem_size, pdev.name)) {
    dev_err(&pdev.dev, "cannot reserve registers\n");
    ret = -ENXIO;
    goto exit_mem;
    }
    ei_local.mem = ioremap(mem.start, mem_size);
    dev.base_addr = (unsigned long)ei_local.mem;
    if (ei_local.mem == core::ptr::null_mut()) {
    dev_err(&pdev.dev, "Cannot ioremap area %pR\n", mem);
    ret = -ENXIO;
    goto exit_req;
    }
// look for reset area
    mem2 = platform_get_resource(pdev, IORESOURCE_MEM, 1);
    if (!mem2) {
    if (!ax.plat.reg_offsets) {
    for (ret = 0; ret < 0x20; ret++)
    ax.reg_offsets[ret] = (mem_size / 0x20) * ret;
    }
    } else {
    mem2_size = resource_size(mem2);
    if (!request_mem_region(mem2.start, mem2_size, pdev.name)) {
    dev_err(&pdev.dev, "cannot reserve registers\n");
    ret = -ENXIO;
    goto exit_mem1;
    }
    ax.map2 = ioremap(mem2.start, mem2_size);
    if (!ax.map2) {
    dev_err(&pdev.dev, "cannot map reset register\n");
    ret = -ENXIO;
    goto exit_mem2;
    }
    ei_local.reg_offset[0x1f] = ax.map2 - ei_local.mem;
    }
// got resources, now initialise and register device
    ret = ax_init_dev(dev);
    if (!ret)
    return 0;
    if (!ax.map2)
    goto exit_mem1;
    iounmap(ax.map2);
    exit_mem2:
    if (mem2)
    release_mem_region(mem2.start, mem2_size);
    exit_mem1:
    iounmap(ei_local.mem);
    exit_req:
    release_mem_region(mem.start, mem_size);
    exit_mem:
    platform_set_drvdata(pdev, core::ptr::null_mut());
    free_netdev(dev);
    return ret;
    }
// suspend and resume

#[no_mangle]
unsafe extern "C" fn ax_suspend(dev: *mut platform_device, state: pm_message_t) -> c_int {
    static int ax_suspend(struct platform_device *dev, pm_message_t state)
    {
    struct net_device *ndev = platform_get_drvdata(dev);
    struct ax_device *ax = to_ax_dev(ndev);
    ax.resume_open = ax.running;
    netif_device_detach(ndev);
    ax_close(ndev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ax_resume(pdev: *mut platform_device) -> c_int {
    static int ax_resume(struct platform_device *pdev)
    {
    struct net_device *ndev = platform_get_drvdata(pdev);
    struct ax_device *ax = to_ax_dev(ndev);
    ax_initial_setup(ndev, netdev_priv(ndev));
    ax_NS8390_init(ndev, ax.resume_open);
    netif_device_attach(ndev);
    if (ax.resume_open)
    ax_open(ndev);
    return 0;
    }

    static struct platform_driver axdrv = {
    .driver	= {
    .name		= "ax88796",
    },
    .probe		= ax_probe,
    .remove		= ax_remove,
    .suspend	= ax_suspend,
    .resume		= ax_resume,
    };
    module_platform_driver(axdrv);
    MODULE_DESCRIPTION("AX88796 10/100 Ethernet platform driver");
    MODULE_AUTHOR("Ben Dooks, <ben@simtec.co.uk>");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:ax88796");
