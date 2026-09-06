//! Automatically rewritten from C to Rust
//! Source: drivers/net/dsa/dsa_loop.c
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
// Distributed Switch Architecture loopback driver
//
// Copyright (C) 2016, Florian Fainelli <f.fainelli@gmail.com>
//

pub const DSA_LOOP_NUM_PORTS: c_int = 6;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_loop_vlan {
    pub members: u16,
    pub untagged: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_loop_mib_entry {
    pub name: [c_char; ETH_GSTRING_LEN],
    pub val: c_ulong,
}

    enum dsa_loop_mib_counters {
    DSA_LOOP_PHY_READ_OK,
    DSA_LOOP_PHY_READ_ERR,
    DSA_LOOP_PHY_WRITE_OK,
    DSA_LOOP_PHY_WRITE_ERR,
    __DSA_LOOP_CNT_MAX,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_loop_port {
    pub mib: [dsa_loop_mib_entry; __DSA_LOOP_CNT_MAX],
    pub pvid: u16,
    pub mtu: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_loop_priv {
    pub bus: *mut mii_bus,
    pub port_base: c_uint,
    pub vlans: [dsa_loop_vlan; VLAN_N_VID],
    pub netdev: *mut net_device,
    pub ports: [dsa_loop_port; DSA_MAX_PORTS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_loop_pdata {
// Must be first, such that dsa_register_switch() can access this
// without gory pointer manipulations
//
    pub cd: dsa_chip_data,
    pub name: *const c_char,
    pub enabled_ports: c_uint,
    pub netdev: *const c_char,
}

    static struct dsa_loop_mib_entry dsa_loop_mibs[] = {
    [DSA_LOOP_PHY_READ_OK]	= { "phy_read_ok", },
    [DSA_LOOP_PHY_READ_ERR]	= { "phy_read_err", },
    [DSA_LOOP_PHY_WRITE_OK] = { "phy_write_ok", },
    [DSA_LOOP_PHY_WRITE_ERR] = { "phy_write_err", },
    };
    static struct phy_device *phydevs[PHY_MAX_ADDR];
    static struct mdio_device *switch_mdiodev;
    enum dsa_loop_devlink_resource_id {
    DSA_LOOP_DEVLINK_PARAM_ID_NONE,  /* DEVLINK_RESOURCE_ID_PARENT_TOP */
    DSA_LOOP_DEVLINK_PARAM_ID_VTU,
    };
#[no_mangle]
unsafe extern "C" fn dsa_loop_devlink_vtu_get(priv: *mut c_void) -> u64 {
    static u64 dsa_loop_devlink_vtu_get(void *priv)
    {
    struct dsa_loop_priv *ps = priv;
    unsigned int i, count = 0;
    struct dsa_loop_vlan *vl;
    for (i = 0; i < ARRAY_SIZE(ps.vlans); i++) {
    vl = &ps.vlans[i];
    if (vl.members)
    count++;
    }
    return count;
    }
#[no_mangle]
unsafe extern "C" fn dsa_loop_setup_devlink_resources(ds: *mut dsa_switch) -> c_int {
    static int dsa_loop_setup_devlink_resources(struct dsa_switch *ds)
    {
    struct devlink_resource_size_params size_params;
    struct dsa_loop_priv *ps = ds.priv;
    int err;
    devlink_resource_size_params_init(&size_params, ARRAY_SIZE(ps.vlans),
    ARRAY_SIZE(ps.vlans),
    1, DEVLINK_RESOURCE_UNIT_ENTRY);
    err = dsa_devlink_resource_register(ds, "VTU", ARRAY_SIZE(ps.vlans),
    DSA_LOOP_DEVLINK_PARAM_ID_VTU,
    DEVLINK_RESOURCE_ID_PARENT_TOP,
    &size_params);
    if (err)
    goto out;
    dsa_devlink_resource_occ_get_register(ds,
    DSA_LOOP_DEVLINK_PARAM_ID_VTU,
    dsa_loop_devlink_vtu_get, ps);
    return 0;
    out:
    dsa_devlink_resources_unregister(ds);
    return err;
    }
    static enum dsa_tag_protocol dsa_loop_get_protocol(struct dsa_switch *ds,
    int port,
    enum dsa_tag_protocol mp)
    {
    dev_dbg(ds.dev, "%s: port: %d\n", __func__, port);
    return DSA_TAG_PROTO_NONE;
    }
#[no_mangle]
unsafe extern "C" fn dsa_loop_setup(ds: *mut dsa_switch) -> c_int {
    static int dsa_loop_setup(struct dsa_switch *ds)
    {
    struct dsa_loop_priv *ps = ds.priv;
    unsigned int i;
    for (i = 0; i < ds.num_ports; i++)
    memcpy(ps.ports[i].mib, dsa_loop_mibs,
    sizeof(dsa_loop_mibs));
    dev_dbg(ds.dev, "%s\n", __func__);
    return dsa_loop_setup_devlink_resources(ds);
    }
#[no_mangle]
unsafe extern "C" fn dsa_loop_teardown(ds: *mut dsa_switch) {
    static void dsa_loop_teardown(struct dsa_switch *ds)
    {
    dsa_devlink_resources_unregister(ds);
    }
#[no_mangle]
unsafe extern "C" fn dsa_loop_get_sset_count(ds: *mut dsa_switch, port: c_int, sset: c_int) -> c_int {
    static int dsa_loop_get_sset_count(struct dsa_switch *ds, int port, int sset)
    {
    if (sset != ETH_SS_STATS && sset != ETH_SS_PHY_STATS)
    return 0;
    return __DSA_LOOP_CNT_MAX;
    }
    static void dsa_loop_get_strings(struct dsa_switch *ds, int port,
    u32 stringset, uint8_t *data)
    {
    struct dsa_loop_priv *ps = ds.priv;
    unsigned int i;
    if (stringset != ETH_SS_STATS && stringset != ETH_SS_PHY_STATS)
    return;
    for (i = 0; i < __DSA_LOOP_CNT_MAX; i++)
    ethtool_puts(&data, ps.ports[port].mib[i].name);
    }
    static void dsa_loop_get_ethtool_stats(struct dsa_switch *ds, int port,
    uint64_t *data)
    {
    struct dsa_loop_priv *ps = ds.priv;
    unsigned int i;
    for (i = 0; i < __DSA_LOOP_CNT_MAX; i++)
    data[i] = ps.ports[port].mib[i].val;
    }
#[no_mangle]
unsafe extern "C" fn dsa_loop_phy_read(ds: *mut dsa_switch, port: c_int, regnum: c_int) -> c_int {
    static int dsa_loop_phy_read(struct dsa_switch *ds, int port, int regnum)
    {
    struct dsa_loop_priv *ps = ds.priv;
    struct mii_bus *bus = ps.bus;
    int ret;
    ret = mdiobus_read_nested(bus, ps.port_base + port, regnum);
    if (ret < 0)
    ps.ports[port].mib[DSA_LOOP_PHY_READ_ERR].val++;
    else
    ps.ports[port].mib[DSA_LOOP_PHY_READ_OK].val++;
    return ret;
    }
    static int dsa_loop_phy_write(struct dsa_switch *ds, int port,
    int regnum, u16 value)
    {
    struct dsa_loop_priv *ps = ds.priv;
    struct mii_bus *bus = ps.bus;
    int ret;
    ret = mdiobus_write_nested(bus, ps.port_base + port, regnum, value);
    if (ret < 0)
    ps.ports[port].mib[DSA_LOOP_PHY_WRITE_ERR].val++;
    else
    ps.ports[port].mib[DSA_LOOP_PHY_WRITE_OK].val++;
    return ret;
    }
    static int dsa_loop_port_bridge_join(struct dsa_switch *ds, int port,
    struct dsa_bridge bridge,
    bool *tx_fwd_offload,
    struct netlink_ext_ack *extack)
    {
    dev_dbg(ds.dev, "%s: port: %d, bridge: %s\n",
    __func__, port, bridge.dev.name);
    return 0;
    }
    static void dsa_loop_port_bridge_leave(struct dsa_switch *ds, int port,
    struct dsa_bridge bridge)
    {
    dev_dbg(ds.dev, "%s: port: %d, bridge: %s\n",
    __func__, port, bridge.dev.name);
    }
    static void dsa_loop_port_stp_state_set(struct dsa_switch *ds, int port,
    u8 state)
    {
    dev_dbg(ds.dev, "%s: port: %d, state: %d\n",
    __func__, port, state);
    }
    static int dsa_loop_port_vlan_filtering(struct dsa_switch *ds, int port,
    bool vlan_filtering,
    struct netlink_ext_ack *extack)
    {
    dev_dbg(ds.dev, "%s: port: %d, vlan_filtering: %d\n",
    __func__, port, vlan_filtering);
    return 0;
    }
    static int dsa_loop_port_vlan_add(struct dsa_switch *ds, int port,
    const struct switchdev_obj_port_vlan *vlan,
    struct netlink_ext_ack *extack)
    {
    let mut untagged: bool = vlan.flags & BRIDGE_VLAN_INFO_UNTAGGED;
    let mut pvid: bool = vlan.flags & BRIDGE_VLAN_INFO_PVID;
    struct dsa_loop_priv *ps = ds.priv;
    struct mii_bus *bus = ps.bus;
    struct dsa_loop_vlan *vl;
    if (vlan.vid >= ARRAY_SIZE(ps.vlans))
    return -ERANGE;
// Just do a sleeping operation to make lockdep checks effective
    mdiobus_read(bus, ps.port_base + port, MII_BMSR);
    vl = &ps.vlans[vlan.vid];
    vl.members |= BIT(port);
    if (untagged)
    vl.untagged |= BIT(port);
    else
    vl.untagged &= ~BIT(port);
    dev_dbg(ds.dev, "%s: port: %d vlan: %d, %stagged, pvid: %d\n",
    __func__, port, vlan.vid, untagged ? "un" : "", pvid);
    if (pvid)
    ps.ports[port].pvid = vlan.vid;
    return 0;
    }
    static int dsa_loop_port_vlan_del(struct dsa_switch *ds, int port,
    const struct switchdev_obj_port_vlan *vlan)
    {
    let mut untagged: bool = vlan.flags & BRIDGE_VLAN_INFO_UNTAGGED;
    struct dsa_loop_priv *ps = ds.priv;
    let mut pvid: u16 = ps.ports[port].pvid;
    struct mii_bus *bus = ps.bus;
    struct dsa_loop_vlan *vl;
// Just do a sleeping operation to make lockdep checks effective
    mdiobus_read(bus, ps.port_base + port, MII_BMSR);
    vl = &ps.vlans[vlan.vid];
    vl.members &= ~BIT(port);
    if (untagged)
    vl.untagged &= ~BIT(port);
    if (pvid == vlan.vid)
    pvid = 1;
    dev_dbg(ds.dev, "%s: port: %d vlan: %d, %stagged, pvid: %d\n",
    __func__, port, vlan.vid, untagged ? "un" : "", pvid);
    ps.ports[port].pvid = pvid;
    return 0;
    }
    static int dsa_loop_port_change_mtu(struct dsa_switch *ds, int port,
    int new_mtu)
    {
    struct dsa_loop_priv *priv = ds.priv;
    priv.ports[port].mtu = new_mtu;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dsa_loop_port_max_mtu(ds: *mut dsa_switch, port: c_int) -> c_int {
    static int dsa_loop_port_max_mtu(struct dsa_switch *ds, int port)
    {
    return ETH_MAX_MTU;
    }
    static void dsa_loop_phylink_get_caps(struct dsa_switch *dsa, int port,
    struct phylink_config *config)
    {
    bitmap_fill(config.supported_interfaces, PHY_INTERFACE_MODE_MAX);
    __clear_bit(PHY_INTERFACE_MODE_NA, config.supported_interfaces);
    config.mac_capabilities = ~0;
    }
    static const struct dsa_switch_ops dsa_loop_driver = {
    .get_tag_protocol	= dsa_loop_get_protocol,
    .setup			= dsa_loop_setup,
    .teardown		= dsa_loop_teardown,
    .get_strings		= dsa_loop_get_strings,
    .get_ethtool_stats	= dsa_loop_get_ethtool_stats,
    .get_sset_count		= dsa_loop_get_sset_count,
    .get_ethtool_phy_stats	= dsa_loop_get_ethtool_stats,
    .phy_read		= dsa_loop_phy_read,
    .phy_write		= dsa_loop_phy_write,
    .port_bridge_join	= dsa_loop_port_bridge_join,
    .port_bridge_leave	= dsa_loop_port_bridge_leave,
    .port_stp_state_set	= dsa_loop_port_stp_state_set,
    .port_vlan_filtering	= dsa_loop_port_vlan_filtering,
    .port_vlan_add		= dsa_loop_port_vlan_add,
    .port_vlan_del		= dsa_loop_port_vlan_del,
    .port_change_mtu	= dsa_loop_port_change_mtu,
    .port_max_mtu		= dsa_loop_port_max_mtu,
    .phylink_get_caps	= dsa_loop_phylink_get_caps,
    };
#[no_mangle]
unsafe extern "C" fn dsa_loop_drv_probe(mdiodev: *mut mdio_device) -> c_int {
    static int dsa_loop_drv_probe(struct mdio_device *mdiodev)
    {
    struct dsa_loop_pdata *pdata = mdiodev.dev.platform_data;
    struct dsa_loop_priv *ps;
    struct dsa_switch *ds;
    int ret;
    if (!pdata)
    return -ENODEV;
    ds = devm_kzalloc(&mdiodev.dev, sizeof(*ds), GFP_KERNEL);
    if (!ds)
    return -ENOMEM;
    ds.dev = &mdiodev.dev;
    ds.num_ports = DSA_LOOP_NUM_PORTS;
    ps = devm_kzalloc(&mdiodev.dev, sizeof(*ps), GFP_KERNEL);
    if (!ps)
    return -ENOMEM;
    ps.netdev = dev_get_by_name(&init_net, pdata.netdev);
    if (!ps.netdev)
    return -EPROBE_DEFER;
    pdata.cd.netdev[DSA_LOOP_CPU_PORT] = &ps.netdev.dev;
    ds.dev = &mdiodev.dev;
    ds.ops = &dsa_loop_driver;
    ds.priv = ps;
    ps.bus = mdiodev.bus;
    dev_set_drvdata(&mdiodev.dev, ds);
    ret = dsa_register_switch(ds);
    if (!ret)
    dev_info(&mdiodev.dev, "%s: 0x%0x\n",
    pdata.name, pdata.enabled_ports);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dsa_loop_drv_remove(mdiodev: *mut mdio_device) {
    static void dsa_loop_drv_remove(struct mdio_device *mdiodev)
    {
    struct dsa_switch *ds = dev_get_drvdata(&mdiodev.dev);
    struct dsa_loop_priv *ps;
    if (!ds)
    return;
    ps = ds.priv;
    dsa_unregister_switch(ds);
    dev_put(ps.netdev);
    }
#[no_mangle]
unsafe extern "C" fn dsa_loop_drv_shutdown(mdiodev: *mut mdio_device) {
    static void dsa_loop_drv_shutdown(struct mdio_device *mdiodev)
    {
    struct dsa_switch *ds = dev_get_drvdata(&mdiodev.dev);
    if (!ds)
    return;
    dsa_switch_shutdown(ds);
    dev_set_drvdata(&mdiodev.dev, core::ptr::null_mut());
    }
    static struct mdio_driver dsa_loop_drv = {
    .mdiodrv.driver	= {
    .name	= "dsa-loop",
    },
    .probe	= dsa_loop_drv_probe,
    .remove	= dsa_loop_drv_remove,
    .shutdown = dsa_loop_drv_shutdown,
    };
    static int dsa_loop_bus_match(struct device *dev,
    const struct device_driver *drv)
    {
    let mut drv: return = = &dsa_loop_drv.mdiodrv.driver;
    }
#[no_mangle]
unsafe extern "C" fn dsa_loop_phydevs_unregister() {
    static void dsa_loop_phydevs_unregister(void)
    {
    for (int i = 0; i < NUM_FIXED_PHYS; i++) {
    if (!IS_ERR(phydevs[i]))
    fixed_phy_unregister(phydevs[i]);
    }
    }
#[no_mangle]
unsafe extern "C" fn dsa_loop_create_switch_mdiodev() -> int __init {
    static int __init dsa_loop_create_switch_mdiodev(void)
    {
    static struct dsa_loop_pdata dsa_loop_pdata = {
    .cd = {
    .port_names[0] = "lan1",
    .port_names[1] = "lan2",
    .port_names[2] = "lan3",
    .port_names[3] = "lan4",
    .port_names[DSA_LOOP_CPU_PORT] = "cpu",
    },
    .name = "DSA mockup driver",
    .enabled_ports = 0x1f,
    .netdev = "eth0",
    };
    struct mii_bus *bus;
    let mut ret: c_int = -ENODEV;
    bus = mdio_find_bus("fixed-0");
    if (WARN_ON(!bus))
    return ret;
    switch_mdiodev = mdio_device_create(bus, 31);
    if (IS_ERR(switch_mdiodev))
    goto out;
    switch_mdiodev.bus_match = dsa_loop_bus_match;
    switch_mdiodev.dev.platform_data = &dsa_loop_pdata;
    ret = mdio_device_register(switch_mdiodev);
    if (ret)
    mdio_device_free(switch_mdiodev);
    out:
    put_device(&bus.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dsa_loop_init() -> int __init {
    static int __init dsa_loop_init(void)
    {
    unsigned int i;
    int ret;
    ret = dsa_loop_create_switch_mdiodev();
    if (ret)
    return ret;
    for (i = 0; i < NUM_FIXED_PHYS; i++)
    phydevs[i] = fixed_phy_register_100fd();
    ret = mdio_driver_register(&dsa_loop_drv);
    if (ret) {
    dsa_loop_phydevs_unregister();
    mdio_device_remove(switch_mdiodev);
    mdio_device_free(switch_mdiodev);
    }
    return ret;
    }
    module_init(dsa_loop_init);
#[no_mangle]
unsafe extern "C" fn dsa_loop_exit() -> void __exit {
    static void __exit dsa_loop_exit(void)
    {
    mdio_driver_unregister(&dsa_loop_drv);
    dsa_loop_phydevs_unregister();
    mdio_device_remove(switch_mdiodev);
    mdio_device_free(switch_mdiodev);
    }
    module_exit(dsa_loop_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Florian Fainelli");
    MODULE_DESCRIPTION("DSA loopback driver");
