//! Automatically rewritten from C to Rust
//! Source: drivers/pci/controller/pcie-mt7621.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// BRIEF MODULE DESCRIPTION
// PCI init for Ralink RT2880 solution
//
// Copyright 2007 Ralink Inc. (bruce_chang@ralinktech.com.tw)
//
// May 2007 Bruce Chang
// Initial Release
//
// May 2009 Bruce Chang
// support RT2880/RT3883 PCIe
//
// May 2011 Bruce Chang
// support RT6855/MT7620 PCIe
//

// MediaTek-specific configuration registers
pub const PCIE_FTS_NUM: c_uint = 0x70c;

// Host-PCI bridge registers
pub const RALINK_PCI_PCICFG_ADDR: c_uint = 0x0000;
pub const RALINK_PCI_PCIMSK_ADDR: c_uint = 0x000c;
pub const RALINK_PCI_CONFIG_ADDR: c_uint = 0x0020;
pub const RALINK_PCI_CONFIG_DATA: c_uint = 0x0024;
pub const RALINK_PCI_MEMBASE: c_uint = 0x0028;
pub const RALINK_PCI_IOBASE: c_uint = 0x002c;
// PCIe RC control registers
pub const RALINK_PCI_ID: c_uint = 0x0030;
pub const RALINK_PCI_CLASS: c_uint = 0x0034;
pub const RALINK_PCI_SUBID: c_uint = 0x0038;
pub const RALINK_PCI_STATUS: c_uint = 0x0050;
// Some definition values

pub const PCIE_PORT_CNT: c_int = 3;
pub const INIT_PORTS_DELAY_MS: c_int = 100;
pub const PERST_DELAY_MS: c_int = 100;
//
// struct mt7621_pcie_port - PCIe port information
// @base: I/O mapped register base
// @list: port list
// @pcie: pointer to PCIe host info
// @clk: pointer to the port clock gate
// @phy: pointer to PHY control block
// @pcie_rst: pointer to port reset control
// @gpio_rst: gpio reset
// @slot: port slot
// @enabled: indicates if port is enabled
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7621_pcie_port {
    pub base: *mut void __iomem,
    pub list: list_head,
    pub pcie: *mut mt7621_pcie,
    pub clk: *mut clk,
    pub phy: *mut phy,
    pub pcie_rst: *mut reset_control,
    pub gpio_rst: *mut gpio_desc,
    pub slot: u32,
    pub enabled: bool,
}

//
// struct mt7621_pcie - PCIe host information
// @base: IO Mapped Register Base
// @dev: Pointer to PCIe device
// @ports: pointer to PCIe port information
// @resets_inverted: depends on chip revision
// reset lines are inverted.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7621_pcie {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub ports: list_head,
    pub resets_inverted: bool,
}

#[no_mangle]
pub unsafe extern "C" fn pcie_read(pcie: *mut mt7621_pcie, reg: u32) -> u32 {
    static inline u32 pcie_read(struct mt7621_pcie *pcie, u32 reg)
    {
    return readl_relaxed(pcie.base + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn pcie_write(pcie: *mut mt7621_pcie, val: u32, reg: u32) {
    static inline void pcie_write(struct mt7621_pcie *pcie, u32 val, u32 reg)
    {
    writel_relaxed(val, pcie.base + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn pcie_port_read(port: *mut mt7621_pcie_port, reg: u32) -> u32 {
    static inline u32 pcie_port_read(struct mt7621_pcie_port *port, u32 reg)
    {
    return readl_relaxed(port.base + reg);
    }
    static inline void pcie_port_write(struct mt7621_pcie_port *port,
    u32 val, u32 reg)
    {
    writel_relaxed(val, port.base + reg);
    }
    static void __iomem *mt7621_pcie_map_bus(struct pci_bus *bus,
    unsigned int devfn, int where)
    {
    struct mt7621_pcie *pcie = bus.sysdata;
    u32 address = PCI_CONF1_EXT_ADDRESS(bus.number, PCI_SLOT(devfn),
    PCI_FUNC(devfn), where);
    writel_relaxed(address, pcie.base + RALINK_PCI_CONFIG_ADDR);
    return pcie.base + RALINK_PCI_CONFIG_DATA + (where & 3);
    }
    static struct pci_ops mt7621_pcie_ops = {
    .map_bus	= mt7621_pcie_map_bus,
    .read		= pci_generic_config_read,
    .write		= pci_generic_config_write,
    };
#[no_mangle]
unsafe extern "C" fn read_config(pcie: *mut mt7621_pcie, dev: c_uint, reg: u32) -> u32 {
    static u32 read_config(struct mt7621_pcie *pcie, unsigned int dev, u32 reg)
    {
    let mut address: u32 = PCI_CONF1_EXT_ADDRESS(0, dev, 0, reg);
    pcie_write(pcie, address, RALINK_PCI_CONFIG_ADDR);
    return pcie_read(pcie, RALINK_PCI_CONFIG_DATA);
    }
    static void write_config(struct mt7621_pcie *pcie, unsigned int dev,
    u32 reg, u32 val)
    {
    let mut address: u32 = PCI_CONF1_EXT_ADDRESS(0, dev, 0, reg);
    pcie_write(pcie, address, RALINK_PCI_CONFIG_ADDR);
    pcie_write(pcie, val, RALINK_PCI_CONFIG_DATA);
    }
#[no_mangle]
pub unsafe extern "C" fn mt7621_rst_gpio_pcie_assert(port: *mut mt7621_pcie_port) {
    static inline void mt7621_rst_gpio_pcie_assert(struct mt7621_pcie_port *port)
    {
    if (port.gpio_rst)
    gpiod_set_value(port.gpio_rst, 1);
    }
#[no_mangle]
pub unsafe extern "C" fn mt7621_rst_gpio_pcie_deassert(port: *mut mt7621_pcie_port) {
    static inline void mt7621_rst_gpio_pcie_deassert(struct mt7621_pcie_port *port)
    {
    if (port.gpio_rst)
    gpiod_set_value(port.gpio_rst, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn mt7621_pcie_port_is_linkup(port: *mut mt7621_pcie_port) -> bool {
    static inline bool mt7621_pcie_port_is_linkup(struct mt7621_pcie_port *port)
    {
    return (pcie_port_read(port, RALINK_PCI_STATUS) & PCIE_PORT_LINKUP) != 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mt7621_control_assert(port: *mut mt7621_pcie_port) {
    static inline void mt7621_control_assert(struct mt7621_pcie_port *port)
    {
    struct mt7621_pcie *pcie = port.pcie;
    if (pcie.resets_inverted)
    reset_control_assert(port.pcie_rst);
    else
    reset_control_deassert(port.pcie_rst);
    }
#[no_mangle]
pub unsafe extern "C" fn mt7621_control_deassert(port: *mut mt7621_pcie_port) {
    static inline void mt7621_control_deassert(struct mt7621_pcie_port *port)
    {
    struct mt7621_pcie *pcie = port.pcie;
    if (pcie.resets_inverted)
    reset_control_deassert(port.pcie_rst);
    else
    reset_control_assert(port.pcie_rst);
    }
    static int mt7621_pcie_parse_port(struct mt7621_pcie *pcie,
    struct device_node *node,
    int slot)
    {
    struct mt7621_pcie_port *port;
    struct device *dev = pcie.dev;
    struct platform_device *pdev = to_platform_device(dev);
    char name[11];
    int err;
    port = devm_kzalloc(dev, sizeof(*port), GFP_KERNEL);
    if (!port)
    return -ENOMEM;
    port.base = devm_platform_ioremap_resource(pdev, slot + 1);
    if (IS_ERR(port.base))
    return PTR_ERR(port.base);
    port.clk = devm_get_clk_from_child(dev, node, core::ptr::null_mut());
    if (IS_ERR(port.clk)) {
    dev_err(dev, "failed to get pcie%d clock\n", slot);
    return PTR_ERR(port.clk);
    }
    port.pcie_rst = of_reset_control_get_exclusive(node, core::ptr::null_mut());
    if (PTR_ERR(port.pcie_rst) == -EPROBE_DEFER) {
    dev_err(dev, "failed to get pcie%d reset control\n", slot);
    return PTR_ERR(port.pcie_rst);
    }
    snprintf(name, sizeof(name), "pcie-phy%d", slot);
    port.phy = devm_of_phy_get(dev, node, name);
    if (IS_ERR(port.phy)) {
    dev_err(dev, "failed to get pcie-phy%d\n", slot);
    err = PTR_ERR(port.phy);
    goto remove_reset;
    }
    port.gpio_rst = devm_gpiod_get_index_optional(dev, "reset", slot,
    GPIOD_OUT_LOW);
    if (IS_ERR(port.gpio_rst)) {
    dev_err(dev, "failed to get GPIO for PCIe%d\n", slot);
    err = PTR_ERR(port.gpio_rst);
    goto remove_reset;
    }
    port.slot = slot;
    port.pcie = pcie;
    INIT_LIST_HEAD(&port.list);
    list_add_tail(&port.list, &pcie.ports);
    return 0;
    remove_reset:
    reset_control_put(port.pcie_rst);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn mt7621_pcie_parse_dt(pcie: *mut mt7621_pcie) -> c_int {
    static int mt7621_pcie_parse_dt(struct mt7621_pcie *pcie)
    {
    struct device *dev = pcie.dev;
    struct platform_device *pdev = to_platform_device(dev);
    struct device_node *node = dev.of_node;
    int err;
    pcie.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(pcie.base))
    return PTR_ERR(pcie.base);
    for_each_available_child_of_node_scoped(node, child) {
    int slot;
    err = of_pci_get_devfn(child);
    if (err < 0)
    return dev_err_probe(dev, err, "failed to parse devfn\n");
    slot = PCI_SLOT(err);
    err = mt7621_pcie_parse_port(pcie, child, slot);
    if (err)
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt7621_pcie_init_port(port: *mut mt7621_pcie_port) -> c_int {
    static int mt7621_pcie_init_port(struct mt7621_pcie_port *port)
    {
    struct mt7621_pcie *pcie = port.pcie;
    struct device *dev = pcie.dev;
    let mut slot: u32 = port.slot;
    int err;
    err = phy_init(port.phy);
    if (err) {
    dev_err(dev, "failed to initialize port%d phy\n", slot);
    return err;
    }
    err = phy_power_on(port.phy);
    if (err) {
    dev_err(dev, "failed to power on port%d phy\n", slot);
    phy_exit(port.phy);
    return err;
    }
    port.enabled = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt7621_pcie_reset_assert(pcie: *mut mt7621_pcie) {
    static void mt7621_pcie_reset_assert(struct mt7621_pcie *pcie)
    {
    struct mt7621_pcie_port *port;
    list_for_each_entry(port, &pcie.ports, list) {
// PCIe RC reset assert
    mt7621_control_assert(port);
// PCIe EP reset assert
    mt7621_rst_gpio_pcie_assert(port);
    }
    msleep(PERST_DELAY_MS);
    }
#[no_mangle]
unsafe extern "C" fn mt7621_pcie_reset_rc_deassert(pcie: *mut mt7621_pcie) {
    static void mt7621_pcie_reset_rc_deassert(struct mt7621_pcie *pcie)
    {
    struct mt7621_pcie_port *port;
    list_for_each_entry(port, &pcie.ports, list)
    mt7621_control_deassert(port);
    }
#[no_mangle]
unsafe extern "C" fn mt7621_pcie_reset_ep_deassert(pcie: *mut mt7621_pcie) {
    static void mt7621_pcie_reset_ep_deassert(struct mt7621_pcie *pcie)
    {
    struct mt7621_pcie_port *port;
    list_for_each_entry(port, &pcie.ports, list)
    mt7621_rst_gpio_pcie_deassert(port);
    msleep(PERST_DELAY_MS);
    }
#[no_mangle]
unsafe extern "C" fn mt7621_pcie_init_ports(pcie: *mut mt7621_pcie) -> c_int {
    static int mt7621_pcie_init_ports(struct mt7621_pcie *pcie)
    {
    struct device *dev = pcie.dev;
    struct mt7621_pcie_port *port, *tmp;
    let mut num_disabled: u8 = 0;
    int err;
    mt7621_pcie_reset_assert(pcie);
    mt7621_pcie_reset_rc_deassert(pcie);
    list_for_each_entry_safe(port, tmp, &pcie.ports, list) {
    let mut slot: u32 = port.slot;
    if (slot == 1) {
    port.enabled = true;
    continue;
    }
    err = mt7621_pcie_init_port(port);
    if (err) {
    dev_err(dev, "initializing port %d failed\n", slot);
    list_del(&port.list);
    }
    }
    msleep(INIT_PORTS_DELAY_MS);
    mt7621_pcie_reset_ep_deassert(pcie);
    tmp = core::ptr::null_mut();
    list_for_each_entry(port, &pcie.ports, list) {
    let mut slot: u32 = port.slot;
    if (!mt7621_pcie_port_is_linkup(port)) {
    dev_info(dev, "pcie%d no card, disable it (RST & CLK)\n",
    slot);
    mt7621_control_assert(port);
    port.enabled = false;
    num_disabled++;
    if (slot == 0) {
    tmp = port;
    continue;
    }
    if (slot == 1 && tmp && !tmp.enabled)
    phy_power_off(tmp.phy);
    }
    }
    return (num_disabled != PCIE_PORT_CNT) ? 0 : -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn mt7621_pcie_enable_port(port: *mut mt7621_pcie_port) {
    static void mt7621_pcie_enable_port(struct mt7621_pcie_port *port)
    {
    struct mt7621_pcie *pcie = port.pcie;
    let mut slot: u32 = port.slot;
    u32 val;
// enable pcie interrupt
    val = pcie_read(pcie, RALINK_PCI_PCIMSK_ADDR);
    val |= PCIE_PORT_INT_EN(slot);
    pcie_write(pcie, val, RALINK_PCI_PCIMSK_ADDR);
// map 2G DDR region
    pcie_port_write(port, PCIE_BAR_MAP_MAX | PCIE_BAR_ENABLE,
    PCI_BASE_ADDRESS_0);
// configure class code and revision ID
    pcie_port_write(port, PCIE_CLASS_CODE | PCIE_REVISION_ID,
    RALINK_PCI_CLASS);
// configure RC FTS number to 250 when it leaves L0s
    val = read_config(pcie, slot, PCIE_FTS_NUM);
    val &= ~PCIE_FTS_NUM_MASK;
    val |= PCIE_FTS_NUM_L0(0x50);
    write_config(pcie, slot, PCIE_FTS_NUM, val);
    }
#[no_mangle]
unsafe extern "C" fn mt7621_pcie_enable_ports(host: *mut pci_host_bridge) -> c_int {
    static int mt7621_pcie_enable_ports(struct pci_host_bridge *host)
    {
    struct mt7621_pcie *pcie = pci_host_bridge_priv(host);
    struct device *dev = pcie.dev;
    struct mt7621_pcie_port *port;
    struct resource_entry *entry;
    int err;
    entry = resource_list_first_type(&host.windows, IORESOURCE_IO);
    if (!entry) {
    dev_err(dev, "cannot get io resource\n");
    return -EINVAL;
    }
// Setup MEMWIN and IOWIN
    pcie_write(pcie, 0xffffffff, RALINK_PCI_MEMBASE);
    pcie_write(pcie, entry.res.start - entry.offset, RALINK_PCI_IOBASE);
    list_for_each_entry(port, &pcie.ports, list) {
    if (port.enabled) {
    err = clk_prepare_enable(port.clk);
    if (err) {
    dev_err(dev, "enabling clk pcie%d\n",
    port.slot);
    return err;
    }
    mt7621_pcie_enable_port(port);
    dev_info(dev, "PCIE%d enabled\n", port.slot);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt7621_pcie_register_host(host: *mut pci_host_bridge) -> c_int {
    static int mt7621_pcie_register_host(struct pci_host_bridge *host)
    {
    struct mt7621_pcie *pcie = pci_host_bridge_priv(host);
    host.ops = &mt7621_pcie_ops;
    host.sysdata = pcie;
    return pci_host_probe(host);
    }
    static const struct soc_device_attribute mt7621_pcie_quirks_match[] = {
    { .soc_id = "mt7621", .revision = "E2" },
    { /* sentinel */ }
    };
#[no_mangle]
unsafe extern "C" fn mt7621_pcie_probe(pdev: *mut platform_device) -> c_int {
    static int mt7621_pcie_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    const struct soc_device_attribute *attr;
    struct mt7621_pcie_port *port;
    struct mt7621_pcie *pcie;
    struct pci_host_bridge *bridge;
    int err;
    if (!dev.of_node)
    return -ENODEV;
    bridge = devm_pci_alloc_host_bridge(dev, sizeof(*pcie));
    if (!bridge)
    return -ENOMEM;
    pcie = pci_host_bridge_priv(bridge);
    pcie.dev = dev;
    platform_set_drvdata(pdev, pcie);
    INIT_LIST_HEAD(&pcie.ports);
    attr = soc_device_match(mt7621_pcie_quirks_match);
    if (attr)
    pcie.resets_inverted = true;
    err = mt7621_pcie_parse_dt(pcie);
    if (err) {
    dev_err(dev, "parsing DT failed\n");
    return err;
    }
    err = mt7621_pcie_init_ports(pcie);
    if (err) {
    dev_err(dev, "nothing connected in virtual bridges\n");
    return 0;
    }
    err = mt7621_pcie_enable_ports(bridge);
    if (err) {
    dev_err(dev, "error enabling pcie ports\n");
    goto remove_resets;
    }
    return mt7621_pcie_register_host(bridge);
    remove_resets:
    list_for_each_entry(port, &pcie.ports, list)
    reset_control_put(port.pcie_rst);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn mt7621_pcie_remove(pdev: *mut platform_device) {
    static void mt7621_pcie_remove(struct platform_device *pdev)
    {
    struct mt7621_pcie *pcie = platform_get_drvdata(pdev);
    struct mt7621_pcie_port *port;
    list_for_each_entry(port, &pcie.ports, list)
    reset_control_put(port.pcie_rst);
    }
    static const struct of_device_id mt7621_pcie_ids[] = {
    { .compatible = "mediatek,mt7621-pci" },
    {},
    };
    MODULE_DEVICE_TABLE(of, mt7621_pcie_ids);
    static struct platform_driver mt7621_pcie_driver = {
    .probe = mt7621_pcie_probe,
    .remove = mt7621_pcie_remove,
    .driver = {
    .name = "mt7621-pci",
    .of_match_table = mt7621_pcie_ids,
    },
    };
    builtin_platform_driver(mt7621_pcie_driver);
    MODULE_DESCRIPTION("MediaTek MT7621 PCIe host controller driver");
    MODULE_LICENSE("GPL v2");
