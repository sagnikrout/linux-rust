//! Automatically rewritten from C to Rust
//! Source: drivers/net/dsa/mt7628.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Mediatek MT7628 Embedded Switch (ESW) DSA driver
// Copyright (C) 2026 Joris Vaisvila <joey@tinyisr.com>
//
// Portions derived from OpenWRT esw_rt3050 driver:
// Copyright (C) 2009-2015 John Crispin <blogic@openwrt.org>
// Copyright (C) 2009-2015 Felix Fietkau <nbd@nbd.name>
// Copyright (C) 2013-2015 Michael Lee <igvtee@gmail.com>
// Copyright (C) 2016 Vittorio Gambaletta <openwrt@vittgam.net>
//

pub const MT7628_ESW_REG_IMR: c_uint = 0x04;
pub const MT7628_ESW_REG_FCT0: c_uint = 0x08;
pub const MT7628_ESW_REG_PFC1: c_uint = 0x14;

pub const MT7628_ESW_REG_SOCPC: c_uint = 0x8c;
pub const MT7628_ESW_REG_POC0: c_uint = 0x90;
pub const MT7628_ESW_REG_POC2: c_uint = 0x98;
pub const MT7628_ESW_REG_SGC: c_uint = 0x9c;
pub const MT7628_ESW_REG_PCR0: c_uint = 0xc0;
pub const MT7628_ESW_REG_PCR1: c_uint = 0xc4;
pub const MT7628_ESW_REG_FPA2: c_uint = 0xc8;
pub const MT7628_ESW_REG_FCT2: c_uint = 0xcc;
pub const MT7628_ESW_REG_SGC2: c_uint = 0xe4;

pub const MT7628_ESW_PVID_S: c_int = 12;

    (MT7628_ESW_PVID_S * ((port) % 2))

    (MT7628_ESW_PVID_M << MT7628_ESW_PVID_SHIFT(port))

    (((pvid) & MT7628_ESW_PVID_M) << MT7628_ESW_PVID_SHIFT(port))
pub const MT7628_ESW_VID_S: c_int = 12;

    (MT7628_ESW_VID_S * ((vlan) % 2))

    (MT7628_ESW_VID_M << MT7628_ESW_VID_SHIFT(vlan))

    (((vid) & MT7628_ESW_VID_M) << MT7628_ESW_VID_SHIFT(vlan))
pub const MT7628_ESW_VMSC_S: c_int = 8;

    (MT7628_ESW_VMSC_S * ((vlan) % 4))

    (MT7628_ESW_VMSC_M << MT7628_ESW_VMSC_SHIFT(vlan))

    (((vmsc) & MT7628_ESW_VMSC_M) << MT7628_ESW_VMSC_SHIFT(vlan))
pub const MT7628_ESW_VUB_S: c_int = 7;

    (MT7628_ESW_VUB_S * ((vlan) % 4))

    (MT7628_ESW_VUB_M << MT7628_ESW_VUB_SHIFT(vlan))

    (((vub) & MT7628_ESW_VUB_M) << MT7628_ESW_VUB_SHIFT(vlan))

pub const MT7628_ESW_NUM_PORTS: c_int = 7;
pub const MT7628_NUM_VLANS: c_int = 16;
    static const struct regmap_config mt7628_esw_regmap_cfg = {
    .name = "mt7628-esw",
    .reg_bits = 32,
    .val_bits = 32,
    .reg_stride = 4,
    .fast_io = true,
    .reg_format_endian = REGMAP_ENDIAN_LITTLE,
    .val_format_endian = REGMAP_ENDIAN_LITTLE,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7628_vlan {
    pub active: bool,
    pub members: u8,
    pub untag: u8,
    pub vid: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7628_esw {
    pub rst_ephy: *mut reset_control,
    pub rst_esw: *mut reset_control,
    pub regmap: *mut regmap,
    pub ds: *mut dsa_switch,
    pub tag_8021q_pvid: [u16; MT7628_ESW_NUM_PORTS],
    pub vlans: [mt7628_vlan; MT7628_NUM_VLANS],
    pub dev: *mut device,
}

#[no_mangle]
unsafe extern "C" fn mt7628_mii_read(bus: *mut mii_bus, port: c_int, regnum: c_int) -> c_int {
    static int mt7628_mii_read(struct mii_bus *bus, int port, int regnum)
    {
    struct mt7628_esw *esw = bus.priv;
    int ret;
    u32 val;
//
// RD_DONE bit is read to clear. Read PCR1 once to acknowledge any
// stale completion indicator before starting a new transaction.
//
    ret = regmap_read(esw.regmap, MT7628_ESW_REG_PCR1, &val);
    if (ret)
    goto out;
    ret = regmap_write(esw.regmap, MT7628_ESW_REG_PCR0,
    FIELD_PREP(MT7628_ESW_PCR0_CPU_PHY_REG,
    regnum) |
    FIELD_PREP(MT7628_ESW_PCR0_CPU_PHY_ADDR,
    port) | MT7628_ESW_PCR0_RD_PHY_CMD);
    if (ret)
    goto out;
    ret = regmap_read_poll_timeout(esw.regmap, MT7628_ESW_REG_PCR1, val,
    (val & MT7628_ESW_PCR1_RD_DONE), 10,
    5000);
    if (ret)
    goto out;
    return FIELD_GET(MT7628_ESW_PCR1_RD_DATA, val);
    out:
    dev_err(&bus.dev, "read failed. MDIO timeout?\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mt7628_mii_write(bus: *mut mii_bus, port: c_int, regnum: c_int, dat: u16) -> c_int {
    static int mt7628_mii_write(struct mii_bus *bus, int port, int regnum, u16 dat)
    {
    struct mt7628_esw *esw = bus.priv;
    u32 val;
    int ret;
//
// WT_DONE bit is read to clear. Read PCR1 once to acknowledge any
// stale completion indicator before starting a new transaction.
//
    ret = regmap_read(esw.regmap, MT7628_ESW_REG_PCR1, &val);
    if (ret)
    goto out;
    ret = regmap_write(esw.regmap, MT7628_ESW_REG_PCR0,
    FIELD_PREP(MT7628_ESW_PCR0_WT_NWAY_DATA, dat) |
    FIELD_PREP(MT7628_ESW_PCR0_CPU_PHY_REG,
    regnum) |
    FIELD_PREP(MT7628_ESW_PCR0_CPU_PHY_ADDR,
    port) | MT7628_ESW_PCR0_WT_PHY_CMD);
    if (ret)
    goto out;
    ret = regmap_read_poll_timeout(esw.regmap, MT7628_ESW_REG_PCR1, val,
    (val & MT7628_ESW_PCR1_WT_DONE), 10,
    5000);
    if (ret)
    goto out;
    return 0;
    out:
    dev_err(&bus.dev, "write failed. MDIO timeout?\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mt7628_setup_internal_mdio(ds: *mut dsa_switch) -> c_int {
    static int mt7628_setup_internal_mdio(struct dsa_switch *ds)
    {
    struct mt7628_esw *esw = ds.priv;
    struct device *dev = ds.dev;
    struct mii_bus *bus;
    bus = devm_mdiobus_alloc(dev);
    if (!bus)
    return -ENOMEM;
    bus.name = "MT7628 internal MDIO bus";
    snprintf(bus.id, MII_BUS_ID_SIZE, "%s-mii", dev_name(dev));
    bus.priv = esw;
    bus.read = mt7628_mii_read;
    bus.write = mt7628_mii_write;
    bus.parent = dev;
    ds.user_mii_bus = bus;
    bus.phy_mask = ~ds.phys_mii_mask;
    return devm_mdiobus_register(dev, bus);
    }
#[no_mangle]
unsafe extern "C" fn mt7628_switch_init(ds: *mut dsa_switch) {
    static void mt7628_switch_init(struct dsa_switch *ds)
    {
    struct mt7628_esw *esw = ds.priv;
    regmap_write(esw.regmap, MT7628_ESW_REG_FCT0,
    FIELD_PREP(MT7628_ESW_FCT0_DROP_SET_TH, 0x50) |
    FIELD_PREP(MT7628_ESW_FCT0_DROP_RLS_TH, 0x78) |
    FIELD_PREP(MT7628_ESW_FCT0_FC_SET_TH, 0xa0) |
    FIELD_PREP(MT7628_ESW_FCT0_FC_RLS_TH, 0xc8));
    regmap_write(esw.regmap, MT7628_ESW_REG_FCT2,
    FIELD_PREP(MT7628_ESW_FCT2_MC_PER_PORT_TH, 0xc) |
    FIELD_PREP(MT7628_ESW_FCT2_MUST_DROP_SET_TH, 0x10) |
    FIELD_PREP(MT7628_ESW_FCT2_MUST_DROP_RLS_TH, 0x12));
//
// general switch configuration:
// 300s aging interval
// broadcast storm prevention disabled
// max packet length 1536 bytes
// disable collision 16 packet abort and late collision abort
// use xor48 for address hashing
// disable tx backoff
// 10 packet back pressure jam
// disable was_transmit
// jam until BP condition released
// 30ms LED flash
// rmc tb fault to all ports
// unmatched IGMP as broadcast
//
    regmap_write(esw.regmap, MT7628_ESW_REG_SGC,
    FIELD_PREP(MT7628_ESW_SGC_AGING_INTERVAL, 1) |
    FIELD_PREP(MT7628_ESW_BC_STORM_PROT, 0) |
    FIELD_PREP(MT7628_ESW_PKT_MAX_LEN, 0) |
    MT7628_ESW_DIS_PKT_ABORT |
    FIELD_PREP(MT7628_ESW_ADDRESS_HASH_ALG, 1) |
    MT7628_ESW_DISABLE_TX_BACKOFF |
    FIELD_PREP(MT7628_ESW_BP_JAM_CNT, 10) |
    FIELD_PREP(MT7628_ESW_DISMIIPORT_WASTX, 0) |
    FIELD_PREP(MT7628_ESW_BP_MODE, 0b10) |
    FIELD_PREP(MT7628_ESW_LED_FLASH_TIME, 0) |
    FIELD_PREP(MT7628_ESW_RMC_RULE, 0) |
    FIELD_PREP(MT7628_ESW_IP_MULT_RULE, 0));
    regmap_write(esw.regmap, MT7628_ESW_REG_SOCPC,
    MT7628_ESW_SOCPC_CRC_PADDING |
    FIELD_PREP(MT7628_ESW_SOCPC_DISUN2CPU,
    MT7628_ESW_PORTS_CPU) |
    FIELD_PREP(MT7628_ESW_SOCPC_DISMC2CPU,
    MT7628_ESW_PORTS_CPU) |
    FIELD_PREP(MT7628_ESW_SOCPC_DISBC2CPU,
    MT7628_ESW_PORTS_CPU));
    regmap_set_bits(esw.regmap, MT7628_ESW_REG_FPA2,
    MT7628_ESW_FPA2_FORCE_RGMII_EN1 |
    MT7628_ESW_FPA2_FORCE_RGMII_LINK1 |
    MT7628_ESW_FPA2_AP_EN);
    regmap_update_bits(esw.regmap, MT7628_ESW_REG_FPA2,
    MT7628_ESW_FPA2_EXT_PHY_ADDR_BASE,
    FIELD_PREP(MT7628_ESW_FPA2_EXT_PHY_ADDR_BASE, 31));
// disable all interrupts
    regmap_write(esw.regmap, MT7628_ESW_REG_IMR, 0);
// enable MT7628 DSA tag on CPU port
    regmap_write(esw.regmap, MT7628_ESW_REG_SGC2,
    MT7628_ESW_SGC2_SPECIAL_TAG_EN |
    FIELD_PREP(MT7628_ESW_SGC2_TX_CPU_TPID_BIT_MAP,
    MT7628_ESW_PORTS_CPU));
//
// Double tag feature allows switch to always append the port PVID VLAN tag
// regardless of if the incoming packet already has a VLAN tag.
// This is enabled to simulate VLAN unawareness.
//
    regmap_set_bits(esw.regmap, MT7628_ESW_REG_SGC2,
    FIELD_PREP(MT7628_ESW_SGC2_DOUBLE_TAG_EN,
    MT7628_ESW_PORTS_NOCPU));
    regmap_set_bits(esw.regmap, MT7628_ESW_REG_POC2,
    MT7628_ESW_POC2_PER_VLAN_UNTAG_EN);
    regmap_update_bits(esw.regmap, MT7628_ESW_REG_PFC1,
    MT7628_ESW_PFC1_EN_VLAN,
    FIELD_PREP(MT7628_ESW_PFC1_EN_VLAN,
    MT7628_ESW_PORTS_ALL));
    }
    static void mt7628_esw_set_pvid(struct mt7628_esw *esw, unsigned int port,
    unsigned int pvid)
    {
    regmap_update_bits(esw.regmap, MT7628_ESW_REG_PVIDC(port),
    MT7628_ESW_PVID_MASK(port),
    MT7628_ESW_PVID_PREP(port, pvid));
    }
    static void mt7628_esw_set_vlan_id(struct mt7628_esw *esw, unsigned int vlan,
    unsigned int vid)
    {
    regmap_update_bits(esw.regmap, MT7628_ESW_REG_VLANI(vlan),
    MT7628_ESW_VID_MASK(vlan),
    MT7628_ESW_VID_PREP(vlan, vid));
    }
    static void mt7628_esw_set_vmsc(struct mt7628_esw *esw, unsigned int vlan,
    unsigned int msc)
    {
    regmap_update_bits(esw.regmap, MT7628_ESW_REG_VMSC(vlan),
    MT7628_ESW_VMSC_MASK(vlan),
    MT7628_ESW_VMSC_PREP(vlan, msc));
    }
    static void mt7628_esw_set_vub(struct mt7628_esw *esw, unsigned int vlan,
    unsigned int vub)
    {
    regmap_update_bits(esw.regmap, MT7628_ESW_REG_VUB(vlan),
    MT7628_ESW_VUB_MASK(vlan),
    MT7628_ESW_VUB_PREP(vlan, vub));
    }
#[no_mangle]
unsafe extern "C" fn mt7628_vlan_sync(ds: *mut dsa_switch) {
    static void mt7628_vlan_sync(struct dsa_switch *ds)
    {
    struct mt7628_esw *esw = ds.priv;
    int i;
    for (i = 0; i < MT7628_NUM_VLANS; i++) {
    struct mt7628_vlan *vlan = &esw.vlans[i];
    mt7628_esw_set_vmsc(esw, i, vlan.members);
    mt7628_esw_set_vlan_id(esw, i, vlan.vid);
    mt7628_esw_set_vub(esw, i, vlan.untag);
    }
    for (i = 0; i < ds.num_ports; i++)
    mt7628_esw_set_pvid(esw, i, esw.tag_8021q_pvid[i]);
    }
#[no_mangle]
unsafe extern "C" fn mt7628_setup(ds: *mut dsa_switch) -> c_int {
    static int mt7628_setup(struct dsa_switch *ds)
    {
    struct mt7628_esw *esw = ds.priv;
    int ret;
    ret = reset_control_reset(esw.rst_esw);
    if (ret)
    return ret;
    usleep_range(1000, 2000);
    ret = reset_control_reset(esw.rst_ephy);
    if (ret)
    return ret;
    usleep_range(1000, 2000);
//
// all MMIO reads hang if esw is not out of reset
// ephy needs extra time to get out of reset or it ends up misconfigured
//
    mt7628_switch_init(ds);
    ret = mt7628_setup_internal_mdio(ds);
    if (ret)
    return ret;
    rtnl_lock();
    ret = dsa_tag_8021q_register(ds, htons(ETH_P_8021Q));
    rtnl_unlock();
    return ret;
    }
    static int mt7628_port_enable(struct dsa_switch *ds, int port,
    struct phy_device *phy)
    {
    struct mt7628_esw *esw = ds.priv;
//
// All switch ports are disabled by esw reset and enabled here by DSA.
//
    regmap_clear_bits(esw.regmap, MT7628_ESW_REG_POC0,
    FIELD_PREP(MT7628_ESW_POC0_PORT_DISABLE, BIT(port)));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt7628_port_disable(ds: *mut dsa_switch, port: c_int) {
    static void mt7628_port_disable(struct dsa_switch *ds, int port)
    {
    struct mt7628_esw *esw = ds.priv;
    regmap_set_bits(esw.regmap, MT7628_ESW_REG_POC0,
    FIELD_PREP(MT7628_ESW_POC0_PORT_DISABLE, BIT(port)));
    }
    static enum dsa_tag_protocol
    mt7628_get_tag_proto(struct dsa_switch *ds, int port, enum dsa_tag_protocol mp)
    {
    return DSA_TAG_PROTO_MT7628;
    }
    static void mt7628_phylink_get_caps(struct dsa_switch *ds, int port,
    struct phylink_config *config)
    {
    switch (port) {
    case 6:
    config.mac_capabilities |= MAC_1000;
    fallthrough;
    case 0 ... 4:
    config.mac_capabilities |= MAC_100 | MAC_10;
    __set_bit(PHY_INTERFACE_MODE_INTERNAL,
    config.supported_interfaces);
    break;
    default:
    break;		/* port 5 does not exist on MT7628 */
    }
    }
    static int mt7628_dsa_8021q_vlan_add(struct dsa_switch *ds, int port,
    u16 vid, u16 flags)
    {
    struct mt7628_esw *esw = ds.priv;
    struct mt7628_vlan *vlan = core::ptr::null_mut();
    int i;
    for (i = 0; i < MT7628_NUM_VLANS; i++) {
    struct mt7628_vlan *check_vlan = &esw.vlans[i];
    if (!check_vlan.active && !vlan)
    vlan = check_vlan;
    if (check_vlan.active && check_vlan.vid == vid) {
    vlan = check_vlan;
    break;
    }
    }
    if (!vlan)
    return -ENOSPC;
    vlan.vid = vid;
    vlan.active = true;
    vlan.members |= BIT(port);
    if (flags & BRIDGE_VLAN_INFO_PVID)
    esw.tag_8021q_pvid[port] = vid;
    if (flags & BRIDGE_VLAN_INFO_UNTAGGED)
    vlan.untag |= BIT(port);
    mt7628_vlan_sync(ds);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt7628_dsa_8021q_vlan_del(ds: *mut dsa_switch, port: c_int, vid: u16) -> c_int {
    static int mt7628_dsa_8021q_vlan_del(struct dsa_switch *ds, int port, u16 vid)
    {
    struct mt7628_esw *esw = ds.priv;
    struct mt7628_vlan *vlan = core::ptr::null_mut();
    int i;
    for (i = 0; i < MT7628_NUM_VLANS; i++) {
    struct mt7628_vlan *check_vlan = &esw.vlans[i];
    if (!check_vlan.active || check_vlan.vid != vid)
    continue;
    vlan = check_vlan;
    break;
    }
    if (!vlan)
    return -ENOENT;
    if (esw.tag_8021q_pvid[port] == vid)
    esw.tag_8021q_pvid[port] = 0;
    vlan.members &= ~BIT(port);
    vlan.untag &= ~BIT(port);
    if (!vlan.members) {
    vlan.active = false;
    vlan.vid = 0;
    }
    mt7628_vlan_sync(ds);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt7628_teardown(ds: *mut dsa_switch) {
    static void mt7628_teardown(struct dsa_switch *ds)
    {
    rtnl_lock();
    dsa_tag_8021q_unregister(ds);
    rtnl_unlock();
    }
    static const struct dsa_switch_ops mt7628_switch_ops = {
    .get_tag_protocol = mt7628_get_tag_proto,
    .setup = mt7628_setup,
    .teardown = mt7628_teardown,
    .port_enable = mt7628_port_enable,
    .port_disable = mt7628_port_disable,
    .phylink_get_caps = mt7628_phylink_get_caps,
    .tag_8021q_vlan_add = mt7628_dsa_8021q_vlan_add,
    .tag_8021q_vlan_del = mt7628_dsa_8021q_vlan_del,
    };
#[no_mangle]
unsafe extern "C" fn mt7628_probe(pdev: *mut platform_device) -> c_int {
    static int mt7628_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct mt7628_esw *esw;
    struct dsa_switch *ds;
    void __iomem *base;
    ds = devm_kzalloc(&pdev.dev, sizeof(*ds), GFP_KERNEL);
    if (!ds)
    return -ENOMEM;
    esw = devm_kzalloc(&pdev.dev, sizeof(*esw), GFP_KERNEL);
    if (!esw)
    return -ENOMEM;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    esw.regmap = devm_regmap_init_mmio(&pdev.dev, base,
    &mt7628_esw_regmap_cfg);
    if (IS_ERR(esw.regmap))
    return PTR_ERR(esw.regmap);
    esw.rst_ephy = devm_reset_control_get_exclusive(&pdev.dev, "ephy");
    if (IS_ERR(esw.rst_ephy))
    return dev_err_probe(dev, PTR_ERR(esw.rst_ephy),
    "failed to get EPHY reset\n");
    esw.rst_esw = devm_reset_control_get_exclusive(&pdev.dev, "esw");
    if (IS_ERR(esw.rst_esw))
    return dev_err_probe(dev, PTR_ERR(esw.rst_esw),
    "failed to get ESW reset\n");
    ds.dev = dev;
    ds.num_ports = MT7628_ESW_NUM_PORTS;
    ds.ops = &mt7628_switch_ops;
    ds.priv = esw;
    esw.ds = ds;
    esw.dev = dev;
    dev_set_drvdata(dev, esw);
    return dsa_register_switch(ds);
    }
#[no_mangle]
unsafe extern "C" fn mt7628_remove(pdev: *mut platform_device) {
    static void mt7628_remove(struct platform_device *pdev)
    {
    struct mt7628_esw *esw = platform_get_drvdata(pdev);
    if (!esw)
    return;
    dsa_unregister_switch(esw.ds);
    }
#[no_mangle]
unsafe extern "C" fn mt7628_shutdown(pdev: *mut platform_device) {
    static void mt7628_shutdown(struct platform_device *pdev)
    {
    struct mt7628_esw *esw = platform_get_drvdata(pdev);
    if (!esw)
    return;
    dsa_switch_shutdown(esw.ds);
    dev_set_drvdata(&pdev.dev, core::ptr::null_mut());
    }
    static const struct of_device_id mt7628_of_match[] = {
    { .compatible = "mediatek,mt7628-esw" },
    {}
    };
    MODULE_DEVICE_TABLE(of, mt7628_of_match);
    static struct platform_driver mt7628_driver = {
    .driver = {
    .name = "mt7628-esw",
    .of_match_table = mt7628_of_match,
    },
    .probe = mt7628_probe,
    .remove = mt7628_remove,
    .shutdown = mt7628_shutdown,
    };
    module_platform_driver(mt7628_driver);
    MODULE_AUTHOR("Joris Vaisvila <joey@tinyisr.com>");
    MODULE_DESCRIPTION("Driver for Mediatek MT7628 embedded switch");
    MODULE_LICENSE("GPL");
