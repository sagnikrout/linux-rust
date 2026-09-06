//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/nxp/lpc_eth.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// drivers/net/ethernet/nxp/lpc_eth.c
//
// Author: Kevin Wells <kevin.wells@nxp.com>
//
// Copyright (C) 2010 NXP Semiconductors
// Copyright (C) 2012 Roland Stigge <stigge@antcom.de>
//

pub const ENET_MAXF_SIZE: c_int = 1536;
pub const ENET_RX_DESC: c_int = 48;
pub const ENET_TX_DESC: c_int = 16;
pub const NAPI_WEIGHT: c_int = 16;
//
// Ethernet MAC controller Register offsets
//

//
// mac1 register definitions
//

//
// mac2 register definitions
//

//
// ipgt register definitions
//

//
// ipgr register definitions
//

//
// clrt register definitions
//

//
// maxf register definitions
//

//
// supp register definitions
//

//
// test register definitions
//

//
// mcfg register definitions
//

pub const LPC_MCFG_CLOCK_HOST_DIV_4: c_int = 0;
pub const LPC_MCFG_CLOCK_HOST_DIV_6: c_int = 2;
pub const LPC_MCFG_CLOCK_HOST_DIV_8: c_int = 3;
pub const LPC_MCFG_CLOCK_HOST_DIV_10: c_int = 4;
pub const LPC_MCFG_CLOCK_HOST_DIV_14: c_int = 5;
pub const LPC_MCFG_CLOCK_HOST_DIV_20: c_int = 6;
pub const LPC_MCFG_CLOCK_HOST_DIV_28: c_int = 7;

//
// mcmd register definitions
//

//
// madr register definitions
//

//
// mwtd register definitions
//

//
// mrdd register definitions
//
pub const LPC_MRDD_READ_MASK: c_uint = 0xFFFF;
//
// mind register definitions
//

//
// command register definitions
//

//
// status register definitions
//

//
// tsv0 register definitions
//

//
// tsv1 register definitions
//

//
// rsv register definitions
//

//
// flowcontrolcounter register definitions
//

//
// flowcontrolstatus register definitions
//

//
// rxfilterctrl, rxfilterwolstatus, and rxfilterwolclear shared
// register definitions
//

//
// rxfilterctrl register definitions
//

//
// rxfilterwolstatus/rxfilterwolclear register definitions
//

//
// intstatus, intenable, intclear, and Intset shared register
// definitions
//

//
// powerdown register definitions
//

#[no_mangle]
unsafe extern "C" fn lpc_phy_interface_mode(dev: *mut device) -> phy_interface_t {
    static phy_interface_t lpc_phy_interface_mode(struct device *dev)
    {
    if (dev && dev.of_node) {
    const char *mode = of_get_property(dev.of_node,
    "phy-mode", core::ptr::null_mut());
    if (mode && !strcmp(mode, "mii"))
    return PHY_INTERFACE_MODE_MII;
    }
    return PHY_INTERFACE_MODE_RMII;
    }
#[no_mangle]
unsafe extern "C" fn use_iram_for_net(dev: *mut device) -> bool {
    static bool use_iram_for_net(struct device *dev)
    {
    if (dev && dev.of_node)
    return of_property_read_bool(dev.of_node, "use-iram");
    return false;
    }
// Receive Status information word
pub const RXSTATUS_SIZE: c_uint = 0x000007FF;

    (RXSTATUS_NODESC | RXSTATUS_OVERRUN | RXSTATUS_ALIGN | \
    RXSTATUS_RANGE | RXSTATUS_LENGTH | RXSTATUS_SYMBOL | RXSTATUS_CRC)
// Receive Descriptor control word
pub const RXDESC_CONTROL_SIZE: c_uint = 0x000007FF;

// Transmit Status information word

// Transmit Descriptor control word
pub const TXDESC_CONTROL_SIZE: c_uint = 0x000007FF;

//
// Structure of a TX/RX descriptors and RX status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct txrx_desc_t {
    pub packet: __le32,
    pub control: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_status_t {
    pub statusinfo: __le32,
    pub statushashcrc: __le32,
}

//
// Device driver data structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netdata_local {
    pub pdev: *mut platform_device,
    pub ndev: *mut net_device,
    pub phy_node: *mut device_node,
    pub lock: spinlock_t,
    pub net_base: *mut void __iomem,
    pub msg_enable: u32,
    pub skblen: [c_uint; ENET_TX_DESC],
    pub last_tx_idx: c_uint,
    pub num_used_tx_buffs: c_uint,
    pub mii_bus: *mut mii_bus,
    pub clk: *mut clk,
    pub dma_buff_base_p: dma_addr_t,
    pub dma_buff_base_v: *mut c_void,
    pub dma_buff_size: usize,
    pub tx_desc_v: *mut txrx_desc_t,
    pub tx_stat_v: *mut u32,
    pub tx_buff_v: *mut c_void,
    pub rx_desc_v: *mut txrx_desc_t,
    pub rx_stat_v: *mut rx_status_t,
    pub rx_buff_v: *mut c_void,
    pub link: c_int,
    pub speed: c_int,
    pub duplex: c_int,
    pub napi: napi_struct,
}

//
// MAC support functions
//
#[no_mangle]
unsafe extern "C" fn __lpc_set_mac(pldat: *mut netdata_local, mac: *const u8) {
    static void __lpc_set_mac(struct netdata_local *pldat, const u8 *mac)
    {
    u32 tmp;
// Set station address
    tmp = mac[0] | ((u32)mac[1] << 8);
    writel(tmp, LPC_ENET_SA2(pldat.net_base));
    tmp = mac[2] | ((u32)mac[3] << 8);
    writel(tmp, LPC_ENET_SA1(pldat.net_base));
    tmp = mac[4] | ((u32)mac[5] << 8);
    writel(tmp, LPC_ENET_SA0(pldat.net_base));
    netdev_dbg(pldat.ndev, "Ethernet MAC address %pM\n", mac);
    }
#[no_mangle]
unsafe extern "C" fn __lpc_get_mac(pldat: *mut netdata_local, mac: *mut u8) {
    static void __lpc_get_mac(struct netdata_local *pldat, u8 *mac)
    {
    u32 tmp;
// Get station address
    tmp = readl(LPC_ENET_SA2(pldat.net_base));
    mac[0] = tmp & 0xFF;
    mac[1] = tmp >> 8;
    tmp = readl(LPC_ENET_SA1(pldat.net_base));
    mac[2] = tmp & 0xFF;
    mac[3] = tmp >> 8;
    tmp = readl(LPC_ENET_SA0(pldat.net_base));
    mac[4] = tmp & 0xFF;
    mac[5] = tmp >> 8;
    }
#[no_mangle]
unsafe extern "C" fn __lpc_params_setup(pldat: *mut netdata_local) {
    static void __lpc_params_setup(struct netdata_local *pldat)
    {
    u32 tmp;
    if (pldat.duplex == DUPLEX_FULL) {
    tmp = readl(LPC_ENET_MAC2(pldat.net_base));
    tmp |= LPC_MAC2_FULL_DUPLEX;
    writel(tmp, LPC_ENET_MAC2(pldat.net_base));
    tmp = readl(LPC_ENET_COMMAND(pldat.net_base));
    tmp |= LPC_COMMAND_FULLDUPLEX;
    writel(tmp, LPC_ENET_COMMAND(pldat.net_base));
    writel(LPC_IPGT_LOAD(0x15), LPC_ENET_IPGT(pldat.net_base));
    } else {
    tmp = readl(LPC_ENET_MAC2(pldat.net_base));
    tmp &= ~LPC_MAC2_FULL_DUPLEX;
    writel(tmp, LPC_ENET_MAC2(pldat.net_base));
    tmp = readl(LPC_ENET_COMMAND(pldat.net_base));
    tmp &= ~LPC_COMMAND_FULLDUPLEX;
    writel(tmp, LPC_ENET_COMMAND(pldat.net_base));
    writel(LPC_IPGT_LOAD(0x12), LPC_ENET_IPGT(pldat.net_base));
    }
    if (pldat.speed == SPEED_100)
    writel(LPC_SUPP_SPEED, LPC_ENET_SUPP(pldat.net_base));
    else
    writel(0, LPC_ENET_SUPP(pldat.net_base));
    }
#[no_mangle]
unsafe extern "C" fn __lpc_eth_reset(pldat: *mut netdata_local) {
    static void __lpc_eth_reset(struct netdata_local *pldat)
    {
// Reset all MAC logic
    writel((LPC_MAC1_RESET_TX | LPC_MAC1_RESET_MCS_TX | LPC_MAC1_RESET_RX |
    LPC_MAC1_RESET_MCS_RX | LPC_MAC1_SIMULATION_RESET |
    LPC_MAC1_SOFT_RESET), LPC_ENET_MAC1(pldat.net_base));
    writel((LPC_COMMAND_REG_RESET | LPC_COMMAND_TXRESET |
    LPC_COMMAND_RXRESET), LPC_ENET_COMMAND(pldat.net_base));
    }
#[no_mangle]
unsafe extern "C" fn __lpc_mii_mngt_reset(pldat: *mut netdata_local) -> c_int {
    static int __lpc_mii_mngt_reset(struct netdata_local *pldat)
    {
// Reset MII management hardware
    writel(LPC_MCFG_RESET_MII_MGMT, LPC_ENET_MCFG(pldat.net_base));
// Setup MII clock to slowest rate with a /28 divider
    writel(LPC_MCFG_CLOCK_SELECT(LPC_MCFG_CLOCK_HOST_DIV_28),
    LPC_ENET_MCFG(pldat.net_base));
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __va_to_pa(addr: *mut c_void, pldat: *mut netdata_local) -> phys_addr_t {
    static inline phys_addr_t __va_to_pa(void *addr, struct netdata_local *pldat)
    {
    phys_addr_t phaddr;
    phaddr = addr - pldat.dma_buff_base_v;
    phaddr += pldat.dma_buff_base_p;
    return phaddr;
    }
#[no_mangle]
unsafe extern "C" fn lpc_eth_enable_int(regbase: *mut void __iomem) {
    static void lpc_eth_enable_int(void __iomem *regbase)
    {
    writel((LPC_MACINT_RXDONEINTEN | LPC_MACINT_TXDONEINTEN),
    LPC_ENET_INTENABLE(regbase));
    }
#[no_mangle]
unsafe extern "C" fn lpc_eth_disable_int(regbase: *mut void __iomem) {
    static void lpc_eth_disable_int(void __iomem *regbase)
    {
    writel(0, LPC_ENET_INTENABLE(regbase));
    }
// Setup TX/RX descriptors
#[no_mangle]
unsafe extern "C" fn __lpc_txrx_desc_setup(pldat: *mut netdata_local) {
    static void __lpc_txrx_desc_setup(struct netdata_local *pldat)
    {
    u32 *ptxstat;
    void *tbuff;
    int i;
    struct txrx_desc_t *ptxrxdesc;
    struct rx_status_t *prxstat;
    tbuff = PTR_ALIGN(pldat.dma_buff_base_v, 16);
// Setup TX descriptors, status, and buffers
    pldat.tx_desc_v = tbuff;
    tbuff += sizeof(struct txrx_desc_t) * ENET_TX_DESC;
    pldat.tx_stat_v = tbuff;
    tbuff += sizeof(u32) * ENET_TX_DESC;
    tbuff = PTR_ALIGN(tbuff, 16);
    pldat.tx_buff_v = tbuff;
    tbuff += ENET_MAXF_SIZE * ENET_TX_DESC;
// Setup RX descriptors, status, and buffers
    pldat.rx_desc_v = tbuff;
    tbuff += sizeof(struct txrx_desc_t) * ENET_RX_DESC;
    tbuff = PTR_ALIGN(tbuff, 16);
    pldat.rx_stat_v = tbuff;
    tbuff += sizeof(struct rx_status_t) * ENET_RX_DESC;
    tbuff = PTR_ALIGN(tbuff, 16);
    pldat.rx_buff_v = tbuff;
    tbuff += ENET_MAXF_SIZE * ENET_RX_DESC;
// Map the TX descriptors to the TX buffers in hardware
    for (i = 0; i < ENET_TX_DESC; i++) {
    ptxstat = &pldat.tx_stat_v[i];
    ptxrxdesc = &pldat.tx_desc_v[i];
    ptxrxdesc.packet = __va_to_pa(
    pldat.tx_buff_v + i * ENET_MAXF_SIZE, pldat);
    ptxrxdesc.control = 0;
// ptxstat = 0;
    }
// Map the RX descriptors to the RX buffers in hardware
    for (i = 0; i < ENET_RX_DESC; i++) {
    prxstat = &pldat.rx_stat_v[i];
    ptxrxdesc = &pldat.rx_desc_v[i];
    ptxrxdesc.packet = __va_to_pa(
    pldat.rx_buff_v + i * ENET_MAXF_SIZE, pldat);
    ptxrxdesc.control = RXDESC_CONTROL_INT | (ENET_MAXF_SIZE - 1);
    prxstat.statusinfo = 0;
    prxstat.statushashcrc = 0;
    }
// Setup base addresses in hardware to point to buffers and
// descriptors
//
    writel((ENET_TX_DESC - 1),
    LPC_ENET_TXDESCRIPTORNUMBER(pldat.net_base));
    writel(__va_to_pa(pldat.tx_desc_v, pldat),
    LPC_ENET_TXDESCRIPTOR(pldat.net_base));
    writel(__va_to_pa(pldat.tx_stat_v, pldat),
    LPC_ENET_TXSTATUS(pldat.net_base));
    writel((ENET_RX_DESC - 1),
    LPC_ENET_RXDESCRIPTORNUMBER(pldat.net_base));
    writel(__va_to_pa(pldat.rx_desc_v, pldat),
    LPC_ENET_RXDESCRIPTOR(pldat.net_base));
    writel(__va_to_pa(pldat.rx_stat_v, pldat),
    LPC_ENET_RXSTATUS(pldat.net_base));
    }
#[no_mangle]
unsafe extern "C" fn __lpc_eth_init(pldat: *mut netdata_local) {
    static void __lpc_eth_init(struct netdata_local *pldat)
    {
    u32 tmp;
// Disable controller and reset
    tmp = readl(LPC_ENET_COMMAND(pldat.net_base));
    tmp &= ~LPC_COMMAND_RXENABLE | LPC_COMMAND_TXENABLE;
    writel(tmp, LPC_ENET_COMMAND(pldat.net_base));
    tmp = readl(LPC_ENET_MAC1(pldat.net_base));
    tmp &= ~LPC_MAC1_RECV_ENABLE;
    writel(tmp, LPC_ENET_MAC1(pldat.net_base));
// Initial MAC setup
    writel(LPC_MAC1_PASS_ALL_RX_FRAMES, LPC_ENET_MAC1(pldat.net_base));
    writel((LPC_MAC2_PAD_CRC_ENABLE | LPC_MAC2_CRC_ENABLE),
    LPC_ENET_MAC2(pldat.net_base));
    writel(ENET_MAXF_SIZE, LPC_ENET_MAXF(pldat.net_base));
// Collision window, gap
    writel((LPC_CLRT_LOAD_RETRY_MAX(0xF) |
    LPC_CLRT_LOAD_COLLISION_WINDOW(0x37)),
    LPC_ENET_CLRT(pldat.net_base));
    writel(LPC_IPGR_LOAD_PART2(0x12), LPC_ENET_IPGR(pldat.net_base));
    if (lpc_phy_interface_mode(&pldat.pdev.dev) == PHY_INTERFACE_MODE_MII)
    writel(LPC_COMMAND_PASSRUNTFRAME,
    LPC_ENET_COMMAND(pldat.net_base));
    else {
    writel((LPC_COMMAND_PASSRUNTFRAME | LPC_COMMAND_RMII),
    LPC_ENET_COMMAND(pldat.net_base));
    writel(LPC_SUPP_RESET_RMII, LPC_ENET_SUPP(pldat.net_base));
    }
    __lpc_params_setup(pldat);
// Setup TX and RX descriptors
    __lpc_txrx_desc_setup(pldat);
// Setup packet filtering
    writel((LPC_RXFLTRW_ACCEPTUBROADCAST | LPC_RXFLTRW_ACCEPTPERFECT),
    LPC_ENET_RXFILTER_CTRL(pldat.net_base));
// Get the next TX buffer output index
    pldat.num_used_tx_buffs = 0;
    pldat.last_tx_idx =
    readl(LPC_ENET_TXCONSUMEINDEX(pldat.net_base));
// Clear and enable interrupts
    writel(0xFFFF, LPC_ENET_INTCLEAR(pldat.net_base));
    smp_wmb();
    lpc_eth_enable_int(pldat.net_base);
// Enable controller
    tmp = readl(LPC_ENET_COMMAND(pldat.net_base));
    tmp |= LPC_COMMAND_RXENABLE | LPC_COMMAND_TXENABLE;
    writel(tmp, LPC_ENET_COMMAND(pldat.net_base));
    tmp = readl(LPC_ENET_MAC1(pldat.net_base));
    tmp |= LPC_MAC1_RECV_ENABLE;
    writel(tmp, LPC_ENET_MAC1(pldat.net_base));
    }
#[no_mangle]
unsafe extern "C" fn __lpc_eth_shutdown(pldat: *mut netdata_local) {
    static void __lpc_eth_shutdown(struct netdata_local *pldat)
    {
// Reset ethernet and power down PHY
    __lpc_eth_reset(pldat);
    writel(0, LPC_ENET_MAC1(pldat.net_base));
    writel(0, LPC_ENET_MAC2(pldat.net_base));
    }
//
// MAC<--->PHY support functions
//
#[no_mangle]
unsafe extern "C" fn lpc_mdio_read(bus: *mut mii_bus, phy_id: c_int, phyreg: c_int) -> c_int {
    static int lpc_mdio_read(struct mii_bus *bus, int phy_id, int phyreg)
    {
    struct netdata_local *pldat = bus.priv;
    let mut timeout: c_ulong = jiffies + msecs_to_jiffies(100);
    int lps;
    writel(((phy_id << 8) | phyreg), LPC_ENET_MADR(pldat.net_base));
    writel(LPC_MCMD_READ, LPC_ENET_MCMD(pldat.net_base));
// Wait for unbusy status
    while (readl(LPC_ENET_MIND(pldat.net_base)) & LPC_MIND_BUSY) {
    if (time_after(jiffies, timeout))
    return -EIO;
    cpu_relax();
    }
    lps = readl(LPC_ENET_MRDD(pldat.net_base));
    writel(0, LPC_ENET_MCMD(pldat.net_base));
    return lps;
    }
    static int lpc_mdio_write(struct mii_bus *bus, int phy_id, int phyreg,
    u16 phydata)
    {
    struct netdata_local *pldat = bus.priv;
    let mut timeout: c_ulong = jiffies + msecs_to_jiffies(100);
    writel(((phy_id << 8) | phyreg), LPC_ENET_MADR(pldat.net_base));
    writel(phydata, LPC_ENET_MWTD(pldat.net_base));
// Wait for completion
    while (readl(LPC_ENET_MIND(pldat.net_base)) & LPC_MIND_BUSY) {
    if (time_after(jiffies, timeout))
    return -EIO;
    cpu_relax();
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpc_mdio_reset(bus: *mut mii_bus) -> c_int {
    static int lpc_mdio_reset(struct mii_bus *bus)
    {
    return __lpc_mii_mngt_reset((struct netdata_local *)bus.priv);
    }
#[no_mangle]
unsafe extern "C" fn lpc_handle_link_change(ndev: *mut net_device) {
    static void lpc_handle_link_change(struct net_device *ndev)
    {
    struct netdata_local *pldat = netdev_priv(ndev);
    struct phy_device *phydev = ndev.phydev;
    unsigned long flags;
    let mut status_change: bool = false;
    spin_lock_irqsave(&pldat.lock, flags);
    if (phydev.link) {
    if ((pldat.speed != phydev.speed) ||
    (pldat.duplex != phydev.duplex)) {
    pldat.speed = phydev.speed;
    pldat.duplex = phydev.duplex;
    status_change = true;
    }
    }
    if (phydev.link != pldat.link) {
    if (!phydev.link) {
    pldat.speed = 0;
    pldat.duplex = -1;
    }
    pldat.link = phydev.link;
    status_change = true;
    }
    spin_unlock_irqrestore(&pldat.lock, flags);
    if (status_change)
    __lpc_params_setup(pldat);
    }
#[no_mangle]
unsafe extern "C" fn lpc_mii_probe(ndev: *mut net_device) -> c_int {
    static int lpc_mii_probe(struct net_device *ndev)
    {
    struct netdata_local *pldat = netdev_priv(ndev);
    struct phy_device *phydev;
// Attach to the PHY
    if (lpc_phy_interface_mode(&pldat.pdev.dev) == PHY_INTERFACE_MODE_MII)
    netdev_info(ndev, "using MII interface\n");
    else
    netdev_info(ndev, "using RMII interface\n");
    if (pldat.phy_node)
    phydev =  of_phy_find_device(pldat.phy_node);
    else
    phydev = phy_find_first(pldat.mii_bus);
    if (!phydev) {
    netdev_err(ndev, "no PHY found\n");
    return -ENODEV;
    }
    phydev = phy_connect(ndev, phydev_name(phydev),
    &lpc_handle_link_change,
    lpc_phy_interface_mode(&pldat.pdev.dev));
    if (IS_ERR(phydev)) {
    netdev_err(ndev, "Could not attach to PHY\n");
    return PTR_ERR(phydev);
    }
    phy_set_max_speed(phydev, SPEED_100);
    pldat.link = 0;
    pldat.speed = 0;
    pldat.duplex = -1;
    phy_attached_info(phydev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpc_mii_init(pldat: *mut netdata_local) -> c_int {
    static int lpc_mii_init(struct netdata_local *pldat)
    {
    struct device_node *node;
    let mut err: c_int = -ENXIO;
    pldat.mii_bus = mdiobus_alloc();
    if (!pldat.mii_bus) {
    err = -ENOMEM;
    goto err_out;
    }
// Setup MII mode
    if (lpc_phy_interface_mode(&pldat.pdev.dev) == PHY_INTERFACE_MODE_MII)
    writel(LPC_COMMAND_PASSRUNTFRAME,
    LPC_ENET_COMMAND(pldat.net_base));
    else {
    writel((LPC_COMMAND_PASSRUNTFRAME | LPC_COMMAND_RMII),
    LPC_ENET_COMMAND(pldat.net_base));
    writel(LPC_SUPP_RESET_RMII, LPC_ENET_SUPP(pldat.net_base));
    }
    pldat.mii_bus.name = "lpc_mii_bus";
    pldat.mii_bus.read = &lpc_mdio_read;
    pldat.mii_bus.write = &lpc_mdio_write;
    pldat.mii_bus.reset = &lpc_mdio_reset;
    snprintf(pldat.mii_bus.id, MII_BUS_ID_SIZE, "%s-%x",
    pldat.pdev.name, pldat.pdev.id);
    pldat.mii_bus.priv = pldat;
    pldat.mii_bus.parent = &pldat.pdev.dev;
    node = of_get_child_by_name(pldat.pdev.dev.of_node, "mdio");
    err = of_mdiobus_register(pldat.mii_bus, node);
    of_node_put(node);
    if (err)
    goto err_out_unregister_bus;
    err = lpc_mii_probe(pldat.ndev);
    if (err)
    goto err_out_unregister_bus;
    return 0;
    err_out_unregister_bus:
    mdiobus_unregister(pldat.mii_bus);
    mdiobus_free(pldat.mii_bus);
    err_out:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn __lpc_handle_xmit(ndev: *mut net_device) {
    static void __lpc_handle_xmit(struct net_device *ndev)
    {
    struct netdata_local *pldat = netdev_priv(ndev);
    u32 txcidx, *ptxstat, txstat;
    txcidx = readl(LPC_ENET_TXCONSUMEINDEX(pldat.net_base));
    while (pldat.last_tx_idx != txcidx) {
    let mut skblen: c_uint = pldat.skblen[pldat.last_tx_idx];
// A buffer is available, get buffer status
    ptxstat = &pldat.tx_stat_v[pldat.last_tx_idx];
    txstat = *ptxstat;
// Next buffer and decrement used buffer counter
    pldat.num_used_tx_buffs--;
    pldat.last_tx_idx++;
    if (pldat.last_tx_idx >= ENET_TX_DESC)
    pldat.last_tx_idx = 0;
// Update collision counter
    ndev.stats.collisions += TXSTATUS_COLLISIONS_GET(txstat);
// Any errors occurred?
    if (txstat & TXSTATUS_ERROR) {
    if (txstat & TXSTATUS_UNDERRUN) {
// FIFO underrun
    ndev.stats.tx_fifo_errors++;
    }
    if (txstat & TXSTATUS_LATECOLL) {
// Late collision
    ndev.stats.tx_aborted_errors++;
    }
    if (txstat & TXSTATUS_EXCESSCOLL) {
// Excessive collision
    ndev.stats.tx_aborted_errors++;
    }
    if (txstat & TXSTATUS_EXCESSDEFER) {
// Defer limit
    ndev.stats.tx_aborted_errors++;
    }
    ndev.stats.tx_errors++;
    } else {
// Update stats
    ndev.stats.tx_packets++;
    ndev.stats.tx_bytes += skblen;
    }
    txcidx = readl(LPC_ENET_TXCONSUMEINDEX(pldat.net_base));
    }
    if (pldat.num_used_tx_buffs <= ENET_TX_DESC/2) {
    if (netif_queue_stopped(ndev))
    netif_wake_queue(ndev);
    }
    }
#[no_mangle]
unsafe extern "C" fn __lpc_handle_recv(ndev: *mut net_device, budget: c_int) -> c_int {
    static int __lpc_handle_recv(struct net_device *ndev, int budget)
    {
    struct netdata_local *pldat = netdev_priv(ndev);
    struct sk_buff *skb;
    u32 rxconsidx, len, ethst;
    struct rx_status_t *prxstat;
    let mut rx_done: c_int = 0;
// Get the current RX buffer indexes
    rxconsidx = readl(LPC_ENET_RXCONSUMEINDEX(pldat.net_base));
    while (rx_done < budget && rxconsidx !=
    readl(LPC_ENET_RXPRODUCEINDEX(pldat.net_base))) {
// Get pointer to receive status
    prxstat = &pldat.rx_stat_v[rxconsidx];
    len = (prxstat.statusinfo & RXSTATUS_SIZE) + 1;
// Status error?
    ethst = prxstat.statusinfo;
    if ((ethst & (RXSTATUS_ERROR | RXSTATUS_STATUS_ERROR)) ==
    (RXSTATUS_ERROR | RXSTATUS_RANGE))
    ethst &= ~RXSTATUS_ERROR;
    if (ethst & RXSTATUS_ERROR) {
    let mut si: c_int = prxstat.statusinfo;
// Check statuses
    if (si & RXSTATUS_OVERRUN) {
// Overrun error
    ndev.stats.rx_fifo_errors++;
    } else if (si & RXSTATUS_CRC) {
// CRC error
    ndev.stats.rx_crc_errors++;
    } else if (si & RXSTATUS_LENGTH) {
// Length error
    ndev.stats.rx_length_errors++;
    } else if (si & RXSTATUS_ERROR) {
// Other error
    ndev.stats.rx_length_errors++;
    }
    ndev.stats.rx_errors++;
    } else {
// Packet is good
    skb = dev_alloc_skb(len);
    if (!skb) {
    ndev.stats.rx_dropped++;
    } else {
// Copy packet from buffer
    skb_put_data(skb,
    pldat.rx_buff_v + rxconsidx * ENET_MAXF_SIZE,
    len);
// Pass to upper layer
    skb.protocol = eth_type_trans(skb, ndev);
    netif_receive_skb(skb);
    ndev.stats.rx_packets++;
    ndev.stats.rx_bytes += len;
    }
    }
// Increment consume index
    rxconsidx = rxconsidx + 1;
    if (rxconsidx >= ENET_RX_DESC)
    rxconsidx = 0;
    writel(rxconsidx,
    LPC_ENET_RXCONSUMEINDEX(pldat.net_base));
    rx_done++;
    }
    return rx_done;
    }
#[no_mangle]
unsafe extern "C" fn lpc_eth_poll(napi: *mut napi_struct, budget: c_int) -> c_int {
    static int lpc_eth_poll(struct napi_struct *napi, int budget)
    {
    struct netdata_local *pldat = container_of(napi,
    struct netdata_local, napi);
    struct net_device *ndev = pldat.ndev;
    let mut rx_done: c_int = 0;
    struct netdev_queue *txq = netdev_get_tx_queue(ndev, 0);
    __netif_tx_lock(txq, smp_processor_id());
    __lpc_handle_xmit(ndev);
    __netif_tx_unlock(txq);
    rx_done = __lpc_handle_recv(ndev, budget);
    if (rx_done < budget) {
    napi_complete_done(napi, rx_done);
    lpc_eth_enable_int(pldat.net_base);
    }
    return rx_done;
    }
#[no_mangle]
unsafe extern "C" fn __lpc_eth_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t __lpc_eth_interrupt(int irq, void *dev_id)
    {
    struct net_device *ndev = dev_id;
    struct netdata_local *pldat = netdev_priv(ndev);
    u32 tmp;
    spin_lock(&pldat.lock);
    tmp = readl(LPC_ENET_INTSTATUS(pldat.net_base));
// Clear interrupts
    writel(tmp, LPC_ENET_INTCLEAR(pldat.net_base));
    lpc_eth_disable_int(pldat.net_base);
    if (likely(napi_schedule_prep(&pldat.napi)))
    __napi_schedule(&pldat.napi);
    spin_unlock(&pldat.lock);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn lpc_eth_close(ndev: *mut net_device) -> c_int {
    static int lpc_eth_close(struct net_device *ndev)
    {
    unsigned long flags;
    struct netdata_local *pldat = netdev_priv(ndev);
    if (netif_msg_ifdown(pldat))
    dev_dbg(&pldat.pdev.dev, "shutting down %s\n", ndev.name);
    napi_disable(&pldat.napi);
    netif_stop_queue(ndev);
    spin_lock_irqsave(&pldat.lock, flags);
    __lpc_eth_reset(pldat);
    netif_carrier_off(ndev);
    writel(0, LPC_ENET_MAC1(pldat.net_base));
    writel(0, LPC_ENET_MAC2(pldat.net_base));
    spin_unlock_irqrestore(&pldat.lock, flags);
    if (ndev.phydev)
    phy_stop(ndev.phydev);
    clk_disable_unprepare(pldat.clk);
    return 0;
    }
    static netdev_tx_t lpc_eth_hard_start_xmit(struct sk_buff *skb,
    struct net_device *ndev)
    {
    struct netdata_local *pldat = netdev_priv(ndev);
    u32 len, txidx;
    u32 *ptxstat;
    struct txrx_desc_t *ptxrxdesc;
    len = skb.len;
    spin_lock_irq(&pldat.lock);
    if (pldat.num_used_tx_buffs >= (ENET_TX_DESC - 1)) {
// This function should never be called when there are no
// buffers
//
    netif_stop_queue(ndev);
    spin_unlock_irq(&pldat.lock);
    WARN(1, "BUG! TX request when no free TX buffers!\n");
    return NETDEV_TX_BUSY;
    }
// Get the next TX descriptor index
    txidx = readl(LPC_ENET_TXPRODUCEINDEX(pldat.net_base));
// Setup control for the transfer
    ptxstat = &pldat.tx_stat_v[txidx];
// ptxstat = 0;
    ptxrxdesc = &pldat.tx_desc_v[txidx];
    ptxrxdesc.control =
    (len - 1) | TXDESC_CONTROL_LAST | TXDESC_CONTROL_INT;
// Copy data to the DMA buffer
    memcpy(pldat.tx_buff_v + txidx * ENET_MAXF_SIZE, skb.data, len);
// Save the buffer and increment the buffer counter
    pldat.skblen[txidx] = len;
    pldat.num_used_tx_buffs++;
// Start transmit
    txidx++;
    if (txidx >= ENET_TX_DESC)
    txidx = 0;
    writel(txidx, LPC_ENET_TXPRODUCEINDEX(pldat.net_base));
// Stop queue if no more TX buffers
    if (pldat.num_used_tx_buffs >= (ENET_TX_DESC - 1))
    netif_stop_queue(ndev);
    spin_unlock_irq(&pldat.lock);
    dev_kfree_skb(skb);
    return NETDEV_TX_OK;
    }
#[no_mangle]
unsafe extern "C" fn lpc_set_mac_address(ndev: *mut net_device, p: *mut c_void) -> c_int {
    static int lpc_set_mac_address(struct net_device *ndev, void *p)
    {
    struct sockaddr *addr = p;
    struct netdata_local *pldat = netdev_priv(ndev);
    unsigned long flags;
    if (!is_valid_ether_addr(addr.sa_data))
    return -EADDRNOTAVAIL;
    eth_hw_addr_set(ndev, addr.sa_data);
    spin_lock_irqsave(&pldat.lock, flags);
// Set station address
    __lpc_set_mac(pldat, ndev.dev_addr);
    spin_unlock_irqrestore(&pldat.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpc_eth_set_multicast_list(ndev: *mut net_device) {
    static void lpc_eth_set_multicast_list(struct net_device *ndev)
    {
    struct netdata_local *pldat = netdev_priv(ndev);
    struct netdev_hw_addr_list *mcptr = &ndev.mc;
    struct netdev_hw_addr *ha;
    u32 tmp32, hash_val, hashlo, hashhi;
    unsigned long flags;
    spin_lock_irqsave(&pldat.lock, flags);
// Set station address
    __lpc_set_mac(pldat, ndev.dev_addr);
    tmp32 =  LPC_RXFLTRW_ACCEPTUBROADCAST | LPC_RXFLTRW_ACCEPTPERFECT;
    if (ndev.flags & IFF_PROMISC)
    tmp32 |= LPC_RXFLTRW_ACCEPTUNICAST |
    LPC_RXFLTRW_ACCEPTUMULTICAST;
    if (ndev.flags & IFF_ALLMULTI)
    tmp32 |= LPC_RXFLTRW_ACCEPTUMULTICAST;
    if (netdev_hw_addr_list_count(mcptr))
    tmp32 |= LPC_RXFLTRW_ACCEPTUMULTICASTHASH;
    writel(tmp32, LPC_ENET_RXFILTER_CTRL(pldat.net_base));
// Set initial hash table
    hashlo = 0x0;
    hashhi = 0x0;
// 64 bits : multicast address in hash table
    netdev_hw_addr_list_for_each(ha, mcptr) {
    hash_val = (ether_crc(6, ha.addr) >> 23) & 0x3F;
    if (hash_val >= 32)
    hashhi |= 1 << (hash_val - 32);
    else
    hashlo |= 1 << hash_val;
    }
    writel(hashlo, LPC_ENET_HASHFILTERL(pldat.net_base));
    writel(hashhi, LPC_ENET_HASHFILTERH(pldat.net_base));
    spin_unlock_irqrestore(&pldat.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn lpc_eth_open(ndev: *mut net_device) -> c_int {
    static int lpc_eth_open(struct net_device *ndev)
    {
    struct netdata_local *pldat = netdev_priv(ndev);
    int ret;
    if (netif_msg_ifup(pldat))
    dev_dbg(&pldat.pdev.dev, "enabling %s\n", ndev.name);
    ret = clk_prepare_enable(pldat.clk);
    if (ret)
    return ret;
// Suspended PHY makes LPC ethernet core block, so resume now
    phy_resume(ndev.phydev);
// Reset and initialize
    __lpc_eth_reset(pldat);
    __lpc_eth_init(pldat);
// schedule a link state check
    phy_start(ndev.phydev);
    netif_start_queue(ndev);
    napi_enable(&pldat.napi);
    return 0;
    }
//
// Ethtool ops
//
    static void lpc_eth_ethtool_getdrvinfo(struct net_device *ndev,
    struct ethtool_drvinfo *info)
    {
    strscpy(info.driver, MODNAME, sizeof(info.driver));
    strscpy(info.version, DRV_VERSION, sizeof(info.version));
    strscpy(info.bus_info, dev_name(ndev.dev.parent),
    sizeof(info.bus_info));
    }
#[no_mangle]
unsafe extern "C" fn lpc_eth_ethtool_getmsglevel(ndev: *mut net_device) -> u32 {
    static u32 lpc_eth_ethtool_getmsglevel(struct net_device *ndev)
    {
    struct netdata_local *pldat = netdev_priv(ndev);
    return pldat.msg_enable;
    }
#[no_mangle]
unsafe extern "C" fn lpc_eth_ethtool_setmsglevel(ndev: *mut net_device, level: u32) {
    static void lpc_eth_ethtool_setmsglevel(struct net_device *ndev, u32 level)
    {
    struct netdata_local *pldat = netdev_priv(ndev);
    pldat.msg_enable = level;
    }
    static const struct ethtool_ops lpc_eth_ethtool_ops = {
    .get_drvinfo	= lpc_eth_ethtool_getdrvinfo,
    .get_msglevel	= lpc_eth_ethtool_getmsglevel,
    .set_msglevel	= lpc_eth_ethtool_setmsglevel,
    .get_link	= ethtool_op_get_link,
    .get_link_ksettings = phy_ethtool_get_link_ksettings,
    .set_link_ksettings = phy_ethtool_set_link_ksettings,
    };
    static const struct net_device_ops lpc_netdev_ops = {
    .ndo_open		= lpc_eth_open,
    .ndo_stop		= lpc_eth_close,
    .ndo_start_xmit		= lpc_eth_hard_start_xmit,
    .ndo_set_rx_mode	= lpc_eth_set_multicast_list,
    .ndo_eth_ioctl		= phy_do_ioctl_running,
    .ndo_set_mac_address	= lpc_set_mac_address,
    .ndo_validate_addr	= eth_validate_addr,
    };
#[no_mangle]
unsafe extern "C" fn lpc_eth_drv_probe(pdev: *mut platform_device) -> c_int {
    static int lpc_eth_drv_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct netdata_local *pldat;
    struct net_device *ndev;
    dma_addr_t dma_handle;
    struct resource *res;
    u8 addr[ETH_ALEN];
    int irq, ret;
// Setup network interface for RMII or MII mode
    lpc32xx_set_phy_interface_mode(lpc_phy_interface_mode(dev));
// Get platform resources
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    irq = platform_get_irq(pdev, 0);
    if (!res || irq < 0) {
    dev_err(dev, "error getting resources.\n");
    ret = -ENXIO;
    goto err_exit;
    }
// Allocate net driver data structure
    ndev = alloc_etherdev(sizeof(struct netdata_local));
    if (!ndev) {
    dev_err(dev, "could not allocate device.\n");
    ret = -ENOMEM;
    goto err_exit;
    }
    SET_NETDEV_DEV(ndev, dev);
    pldat = netdev_priv(ndev);
    pldat.pdev = pdev;
    pldat.ndev = ndev;
    spin_lock_init(&pldat.lock);
// Save resources
    ndev.irq = irq;
// Get clock for the device
    pldat.clk = clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(pldat.clk)) {
    dev_err(dev, "error getting clock.\n");
    ret = PTR_ERR(pldat.clk);
    goto err_out_free_dev;
    }
// Enable network clock
    ret = clk_prepare_enable(pldat.clk);
    if (ret)
    goto err_out_clk_put;
// Map IO space
    pldat.net_base = ioremap(res.start, resource_size(res));
    if (!pldat.net_base) {
    dev_err(dev, "failed to map registers\n");
    ret = -ENOMEM;
    goto err_out_disable_clocks;
    }
    ret = request_irq(ndev.irq, __lpc_eth_interrupt, 0,
    ndev.name, ndev);
    if (ret) {
    dev_err(dev, "error requesting interrupt.\n");
    goto err_out_iounmap;
    }
// Setup driver functions
    ndev.netdev_ops = &lpc_netdev_ops;
    ndev.ethtool_ops = &lpc_eth_ethtool_ops;
    ndev.watchdog_timeo = msecs_to_jiffies(2500);
// Get size of DMA buffers/descriptors region
    pldat.dma_buff_size = (ENET_TX_DESC + ENET_RX_DESC) * (ENET_MAXF_SIZE +
    sizeof(struct txrx_desc_t) + sizeof(struct rx_status_t));
    if (use_iram_for_net(dev)) {
    if (pldat.dma_buff_size >
    lpc32xx_return_iram(&pldat.dma_buff_base_v, &dma_handle)) {
    pldat.dma_buff_base_v = core::ptr::null_mut();
    pldat.dma_buff_size = 0;
    netdev_err(ndev,
    "IRAM not big enough for net buffers, using SDRAM instead.\n");
    }
    }
    if (pldat.dma_buff_base_v == core::ptr::null_mut()) {
    ret = dma_coerce_mask_and_coherent(dev, DMA_BIT_MASK(32));
    if (ret)
    goto err_out_free_irq;
    pldat.dma_buff_size = PAGE_ALIGN(pldat.dma_buff_size);
// Allocate a chunk of memory for the DMA ethernet buffers
// and descriptors
//
    pldat.dma_buff_base_v =
    dma_alloc_coherent(dev,
    pldat.dma_buff_size, &dma_handle,
    GFP_KERNEL);
    if (pldat.dma_buff_base_v == core::ptr::null_mut()) {
    ret = -ENOMEM;
    goto err_out_free_irq;
    }
    }
    pldat.dma_buff_base_p = dma_handle;
    netdev_dbg(ndev, "IO address space     :%pR\n", res);
    netdev_dbg(ndev, "IO address size      :%zd\n",
    (size_t)resource_size(res));
    netdev_dbg(ndev, "IO address (mapped)  :0x%p\n",
    pldat.net_base);
    netdev_dbg(ndev, "IRQ number           :%d\n", ndev.irq);
    netdev_dbg(ndev, "DMA buffer size      :%zd\n", pldat.dma_buff_size);
    netdev_dbg(ndev, "DMA buffer P address :%pad\n",
    &pldat.dma_buff_base_p);
    netdev_dbg(ndev, "DMA buffer V address :0x%p\n",
    pldat.dma_buff_base_v);
    pldat.phy_node = of_parse_phandle(np, "phy-handle", 0);
// Get MAC address from current HW setting (POR state is all zeros)
    __lpc_get_mac(pldat, addr);
    eth_hw_addr_set(ndev, addr);
    if (!is_valid_ether_addr(ndev.dev_addr)) {
    of_get_ethdev_address(np, ndev);
    }
    if (!is_valid_ether_addr(ndev.dev_addr))
    eth_hw_addr_random(ndev);
// then shut everything down to save power
    __lpc_eth_shutdown(pldat);
// Set default parameters
    pldat.msg_enable = NETIF_MSG_LINK;
// Force an MII interface reset and clock setup
    __lpc_mii_mngt_reset(pldat);
// Force default PHY interface setup in chip, this will probably be
// changed by the PHY driver
//
    pldat.link = 0;
    pldat.speed = 100;
    pldat.duplex = DUPLEX_FULL;
    __lpc_params_setup(pldat);
    netif_napi_add_weight(ndev, &pldat.napi, lpc_eth_poll, NAPI_WEIGHT);
    ret = register_netdev(ndev);
    if (ret) {
    dev_err(dev, "Cannot register net device, aborting.\n");
    goto err_out_dma_unmap;
    }
    platform_set_drvdata(pdev, ndev);
    ret = lpc_mii_init(pldat);
    if (ret)
    goto err_out_unregister_netdev;
    netdev_info(ndev, "LPC mac at 0x%08lx irq %d\n",
    (unsigned long)res.start, ndev.irq);
    device_init_wakeup(dev, 1);
    device_set_wakeup_enable(dev, 0);
    return 0;
    err_out_unregister_netdev:
    unregister_netdev(ndev);
    err_out_dma_unmap:
    if (!use_iram_for_net(dev) ||
    pldat.dma_buff_size > lpc32xx_return_iram(core::ptr::null_mut(), core::ptr::null_mut()))
    dma_free_coherent(dev, pldat.dma_buff_size,
    pldat.dma_buff_base_v,
    pldat.dma_buff_base_p);
    err_out_free_irq:
    free_irq(ndev.irq, ndev);
    err_out_iounmap:
    iounmap(pldat.net_base);
    err_out_disable_clocks:
    clk_disable_unprepare(pldat.clk);
    err_out_clk_put:
    clk_put(pldat.clk);
    err_out_free_dev:
    free_netdev(ndev);
    err_exit:
    pr_err("%s: not found (%d).\n", MODNAME, ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn lpc_eth_drv_remove(pdev: *mut platform_device) {
    static void lpc_eth_drv_remove(struct platform_device *pdev)
    {
    struct net_device *ndev = platform_get_drvdata(pdev);
    struct netdata_local *pldat = netdev_priv(ndev);
    unregister_netdev(ndev);
    if (!use_iram_for_net(&pldat.pdev.dev) ||
    pldat.dma_buff_size > lpc32xx_return_iram(core::ptr::null_mut(), core::ptr::null_mut()))
    dma_free_coherent(&pldat.pdev.dev, pldat.dma_buff_size,
    pldat.dma_buff_base_v,
    pldat.dma_buff_base_p);
    free_irq(ndev.irq, ndev);
    iounmap(pldat.net_base);
    mdiobus_unregister(pldat.mii_bus);
    mdiobus_free(pldat.mii_bus);
    clk_disable_unprepare(pldat.clk);
    clk_put(pldat.clk);
    free_netdev(ndev);
    }

    static int lpc_eth_drv_suspend(struct platform_device *pdev,
    pm_message_t state)
    {
    struct net_device *ndev = platform_get_drvdata(pdev);
    struct netdata_local *pldat = netdev_priv(ndev);
    if (device_may_wakeup(&pdev.dev))
    enable_irq_wake(ndev.irq);
    if (ndev) {
    if (netif_running(ndev)) {
    netif_device_detach(ndev);
    __lpc_eth_shutdown(pldat);
    clk_disable_unprepare(pldat.clk);
//
// Reset again now clock is disable to be sure
// EMC_MDC is down
//
    __lpc_eth_reset(pldat);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpc_eth_drv_resume(pdev: *mut platform_device) -> c_int {
    static int lpc_eth_drv_resume(struct platform_device *pdev)
    {
    struct net_device *ndev = platform_get_drvdata(pdev);
    struct netdata_local *pldat;
    int ret;
    if (device_may_wakeup(&pdev.dev))
    disable_irq_wake(ndev.irq);
    if (ndev) {
    if (netif_running(ndev)) {
    pldat = netdev_priv(ndev);
// Enable interface clock
    ret = clk_enable(pldat.clk);
    if (ret)
    return ret;
// Reset and initialize
    __lpc_eth_reset(pldat);
    __lpc_eth_init(pldat);
    netif_device_attach(ndev);
    }
    }
    return 0;
    }

    static const struct of_device_id lpc_eth_match[] = {
    { .compatible = "nxp,lpc-eth" },
    { }
    };
    MODULE_DEVICE_TABLE(of, lpc_eth_match);
    static struct platform_driver lpc_eth_driver = {
    .probe		= lpc_eth_drv_probe,
    .remove		= lpc_eth_drv_remove,

    .suspend	= lpc_eth_drv_suspend,
    .resume		= lpc_eth_drv_resume,

    .driver		= {
    .name	= MODNAME,
    .of_match_table = lpc_eth_match,
    },
    };
    module_platform_driver(lpc_eth_driver);
    MODULE_AUTHOR("Kevin Wells <kevin.wells@nxp.com>");
    MODULE_AUTHOR("Roland Stigge <stigge@antcom.de>");
    MODULE_DESCRIPTION("LPC Ethernet Driver");
    MODULE_LICENSE("GPL");
