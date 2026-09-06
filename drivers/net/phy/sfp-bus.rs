//! Automatically rewritten from C to Rust
//! Source: drivers/net/phy/sfp-bus.c
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
// struct sfp_bus - internal representation of a sfp bus
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfp_bus {
// private:
    pub kref: kref,
    pub node: list_head,
    pub fwnode: *const fwnode_handle,
    pub socket_ops: *const sfp_socket_ops,
    pub sfp_dev: *mut device,
    pub sfp: *mut sfp,
    pub upstream_ops: *const sfp_upstream_ops,
    pub upstream: *mut c_void,
    pub phydev: *mut phy_device,
    pub registered: bool,
    pub started: bool,
    pub caps: sfp_module_caps,
}

    const struct sfp_module_caps *sfp_get_module_caps(struct sfp_bus *bus)
    {
    return &bus.caps;
    }
    EXPORT_SYMBOL_GPL(sfp_get_module_caps);
    static void sfp_module_parse_port(struct sfp_bus *bus,
    const struct sfp_eeprom_id *id)
    {
    int port;
// port is the physical connector, set this from the connector field.
    switch (id.base.connector) {
    case SFF8024_CONNECTOR_SC:
    case SFF8024_CONNECTOR_FIBERJACK:
    case SFF8024_CONNECTOR_LC:
    case SFF8024_CONNECTOR_MT_RJ:
    case SFF8024_CONNECTOR_MU:
    case SFF8024_CONNECTOR_OPTICAL_PIGTAIL:
    case SFF8024_CONNECTOR_MPO_1X12:
    case SFF8024_CONNECTOR_MPO_2X16:
    port = PORT_FIBRE;
    break;
    case SFF8024_CONNECTOR_RJ45:
    port = PORT_TP;
    break;
    case SFF8024_CONNECTOR_COPPER_PIGTAIL:
    port = PORT_DA;
    break;
    case SFF8024_CONNECTOR_UNSPEC:
    if (id.base.e1000_base_t) {
    port = PORT_TP;
    break;
    }
    fallthrough;
    case SFF8024_CONNECTOR_SG: /* guess */
    case SFF8024_CONNECTOR_HSSDC_II:
    case SFF8024_CONNECTOR_NOSEPARATE:
    case SFF8024_CONNECTOR_MXC_2X16:
    port = PORT_OTHER;
    break;
    default:
    dev_warn(bus.sfp_dev, "SFP: unknown connector id 0x%02x\n",
    id.base.connector);
    port = PORT_OTHER;
    break;
    }
    switch (port) {
    case PORT_FIBRE:
    phylink_set(bus.caps.link_modes, FIBRE);
    break;
    case PORT_TP:
    phylink_set(bus.caps.link_modes, TP);
    break;
    }
    bus.caps.port = port;
    }
    static void sfp_module_parse_may_have_phy(struct sfp_bus *bus,
    const struct sfp_eeprom_id *id)
    {
    if (id.base.e1000_base_t) {
    bus.caps.may_have_phy = true;
    return;
    }
    if (id.base.phys_id != SFF8024_ID_DWDM_SFP) {
    switch (id.base.extended_cc) {
    case SFF8024_ECC_10GBASE_T_SFI:
    case SFF8024_ECC_10GBASE_T_SR:
    case SFF8024_ECC_5GBASE_T:
    case SFF8024_ECC_2_5GBASE_T:
    bus.caps.may_have_phy = true;
    return;
    }
    }
    bus.caps.may_have_phy = false;
    }
    static void sfp_module_parse_support(struct sfp_bus *bus,
    const struct sfp_eeprom_id *id)
    {
    unsigned long *interfaces = bus.caps.interfaces;
    unsigned long *modes = bus.caps.link_modes;
    unsigned int br_min, br_nom, br_max;
// Decode the bitrate information to MBd
    br_min = br_nom = br_max = 0;
    if (id.base.br_nominal) {
    if (id.base.br_nominal != 255) {
    br_nom = id.base.br_nominal * 100;
    br_min = br_nom - id.base.br_nominal * id.ext.br_min;
    br_max = br_nom + id.base.br_nominal * id.ext.br_max;
    } else if (id.ext.br_max) {
    br_nom = 250 * id.ext.br_max;
    br_max = br_nom + br_nom * id.ext.br_min / 100;
    br_min = br_nom - br_nom * id.ext.br_min / 100;
    }
// When using passive cables, in case neither BR,min nor BR,max
// are specified, set br_min to 0 as the nominal value is then
// used as the maximum.
//
    if (br_min == br_max && id.base.sfp_ct_passive)
    br_min = 0;
    }
// Set ethtool support from the compliance fields.
    if (id.base.e10g_base_sr) {
    phylink_set(modes, 10000baseSR_Full);
    __set_bit(PHY_INTERFACE_MODE_10GBASER, interfaces);
    }
    if (id.base.e10g_base_lr) {
    phylink_set(modes, 10000baseLR_Full);
    __set_bit(PHY_INTERFACE_MODE_10GBASER, interfaces);
    }
    if (id.base.e10g_base_lrm) {
    phylink_set(modes, 10000baseLRM_Full);
    __set_bit(PHY_INTERFACE_MODE_10GBASER, interfaces);
    }
    if (id.base.e10g_base_er) {
    phylink_set(modes, 10000baseER_Full);
    __set_bit(PHY_INTERFACE_MODE_10GBASER, interfaces);
    }
    if (id.base.e1000_base_sx ||
    id.base.e1000_base_lx ||
    id.base.e1000_base_cx) {
    phylink_set(modes, 1000baseX_Full);
    __set_bit(PHY_INTERFACE_MODE_1000BASEX, interfaces);
    }
    if (id.base.e1000_base_t) {
    phylink_set(modes, 1000baseT_Half);
    phylink_set(modes, 1000baseT_Full);
    __set_bit(PHY_INTERFACE_MODE_1000BASEX, interfaces);
    __set_bit(PHY_INTERFACE_MODE_SGMII, interfaces);
    }
// 1000Base-PX or 1000Base-BX10
    if ((id.base.e_base_px || id.base.e_base_bx10) &&
    br_min <= 1300 && br_max >= 1200) {
    phylink_set(modes, 1000baseX_Full);
    __set_bit(PHY_INTERFACE_MODE_1000BASEX, interfaces);
    }
// 100Base-FX, 100Base-LX, 100Base-PX, 100Base-BX10
    if (id.base.e100_base_fx || id.base.e100_base_lx) {
    phylink_set(modes, 100baseFX_Full);
    __set_bit(PHY_INTERFACE_MODE_100BASEX, interfaces);
    }
    if ((id.base.e_base_px || id.base.e_base_bx10) && br_nom == 100) {
    phylink_set(modes, 100baseFX_Full);
    __set_bit(PHY_INTERFACE_MODE_100BASEX, interfaces);
    }
// For active or passive cables, select the link modes
// based on the bit rates and the cable compliance bytes.
//
    if ((id.base.sfp_ct_passive || id.base.sfp_ct_active) && br_nom) {
// This may look odd, but some manufacturers use 12000MBd
    if (br_min <= 12000 && br_max >= 10300) {
    phylink_set(modes, 10000baseCR_Full);
    __set_bit(PHY_INTERFACE_MODE_10GBASER, interfaces);
    }
    if (br_min <= 3200 && br_max >= 3100) {
    phylink_set(modes, 2500baseX_Full);
    __set_bit(PHY_INTERFACE_MODE_2500BASEX, interfaces);
    }
    if (br_min <= 1300 && br_max >= 1200) {
    phylink_set(modes, 1000baseX_Full);
    __set_bit(PHY_INTERFACE_MODE_1000BASEX, interfaces);
    }
    }
    if (id.base.sfp_ct_passive) {
    if (id.base.passive.sff8431_app_e) {
    phylink_set(modes, 10000baseCR_Full);
    __set_bit(PHY_INTERFACE_MODE_10GBASER, interfaces);
    }
    }
    if (id.base.sfp_ct_active) {
    if (id.base.active.sff8431_app_e ||
    id.base.active.sff8431_lim) {
    phylink_set(modes, 10000baseCR_Full);
    __set_bit(PHY_INTERFACE_MODE_10GBASER, interfaces);
    }
    }
    switch (id.base.extended_cc) {
    case SFF8024_ECC_UNSPEC:
    break;
    case SFF8024_ECC_100G_25GAUI_C2M_AOC:
    if (br_min <= 28000 && br_max >= 25000) {
// 25GBASE-R, possibly with FEC
    __set_bit(PHY_INTERFACE_MODE_25GBASER, interfaces);
// There is currently no link mode for 25000base
// with unspecified range, reuse SR.
//
    phylink_set(modes, 25000baseSR_Full);
    }
    break;
    case SFF8024_ECC_100GBASE_SR4_25GBASE_SR:
    phylink_set(modes, 100000baseSR4_Full);
    phylink_set(modes, 25000baseSR_Full);
    __set_bit(PHY_INTERFACE_MODE_25GBASER, interfaces);
    break;
    case SFF8024_ECC_100GBASE_LR4_25GBASE_LR:
    case SFF8024_ECC_100GBASE_ER4_25GBASE_ER:
    phylink_set(modes, 100000baseLR4_ER4_Full);
    break;
    case SFF8024_ECC_100GBASE_CR4:
    phylink_set(modes, 100000baseCR4_Full);
    fallthrough;
    case SFF8024_ECC_25GBASE_CR_S:
    case SFF8024_ECC_25GBASE_CR_N:
    phylink_set(modes, 25000baseCR_Full);
    __set_bit(PHY_INTERFACE_MODE_25GBASER, interfaces);
    break;
    case SFF8024_ECC_10GBASE_T_SFI:
    case SFF8024_ECC_10GBASE_T_SR:
    phylink_set(modes, 10000baseT_Full);
    __set_bit(PHY_INTERFACE_MODE_10GBASER, interfaces);
    break;
    case SFF8024_ECC_5GBASE_T:
    phylink_set(modes, 5000baseT_Full);
    __set_bit(PHY_INTERFACE_MODE_5GBASER, interfaces);
    break;
    case SFF8024_ECC_2_5GBASE_T:
    phylink_set(modes, 2500baseT_Full);
    __set_bit(PHY_INTERFACE_MODE_2500BASEX, interfaces);
    break;
    default:
    dev_warn(bus.sfp_dev,
    "Unknown/unsupported extended compliance code: 0x%02x\n",
    id.base.extended_cc);
    break;
    }
// For fibre channel SFP, derive possible BaseX modes
    if (id.base.fc_speed_100 ||
    id.base.fc_speed_200 ||
    id.base.fc_speed_400) {
    if (id.base.br_nominal >= 31) {
    phylink_set(modes, 2500baseX_Full);
    __set_bit(PHY_INTERFACE_MODE_2500BASEX, interfaces);
    }
    if (id.base.br_nominal >= 12) {
    phylink_set(modes, 1000baseX_Full);
    __set_bit(PHY_INTERFACE_MODE_1000BASEX, interfaces);
    }
    }
// If we haven't discovered any modes that this module supports, try
// the bitrate to determine supported modes. Some BiDi modules (eg,
// 1310nm/1550nm) are not 1000BASE-BX compliant due to the differing
// wavelengths, so do not set any transceiver bits.
//
// Do the same for modules supporting 2500BASE-X. Note that some
// modules use 2500Mbaud rather than 3100 or 3200Mbaud for
// 2500BASE-X, so we allow some slack here.
//
    if (linkmode_empty(modes) && br_nom) {
    if (br_min <= 1300 && br_max >= 1200) {
    phylink_set(modes, 1000baseX_Full);
    __set_bit(PHY_INTERFACE_MODE_1000BASEX, interfaces);
    }
    if (br_min <= 3200 && br_max >= 2500) {
    phylink_set(modes, 2500baseX_Full);
    __set_bit(PHY_INTERFACE_MODE_2500BASEX, interfaces);
    }
    }
    phylink_set(modes, Autoneg);
    phylink_set(modes, Pause);
    phylink_set(modes, Asym_Pause);
    }
    static void sfp_init_module(struct sfp_bus *bus,
    const struct sfp_eeprom_id *id,
    const struct sfp_quirk *quirk)
    {
    memset(&bus.caps, 0, sizeof(bus.caps));
    sfp_module_parse_support(bus, id);
    sfp_module_parse_port(bus, id);
    sfp_module_parse_may_have_phy(bus, id);
    if (quirk && quirk.support)
    quirk.support(id, &bus.caps);
    }
//
// sfp_select_interface() - Select appropriate phy_interface_t mode
// @bus: a pointer to the &struct sfp_bus structure for the sfp module
// @link_modes: ethtool link modes mask
//
// Derive the phy_interface_t mode for the SFP module from the link
// modes mask.
//
    phy_interface_t sfp_select_interface(struct sfp_bus *bus,
    const unsigned long *link_modes)
    {
    if (phylink_test(link_modes, 25000baseCR_Full) ||
    phylink_test(link_modes, 25000baseKR_Full) ||
    phylink_test(link_modes, 25000baseSR_Full))
    return PHY_INTERFACE_MODE_25GBASER;
    if (phylink_test(link_modes, 10000baseCR_Full) ||
    phylink_test(link_modes, 10000baseSR_Full) ||
    phylink_test(link_modes, 10000baseLR_Full) ||
    phylink_test(link_modes, 10000baseLRM_Full) ||
    phylink_test(link_modes, 10000baseER_Full) ||
    phylink_test(link_modes, 10000baseT_Full))
    return PHY_INTERFACE_MODE_10GBASER;
    if (phylink_test(link_modes, 5000baseT_Full))
    return PHY_INTERFACE_MODE_5GBASER;
    if (phylink_test(link_modes, 2500baseX_Full) ||
    phylink_test(link_modes, 2500baseT_Full))
    return PHY_INTERFACE_MODE_2500BASEX;
    if (phylink_test(link_modes, 1000baseT_Half) ||
    phylink_test(link_modes, 1000baseT_Full))
    return PHY_INTERFACE_MODE_SGMII;
    if (phylink_test(link_modes, 1000baseX_Full))
    return PHY_INTERFACE_MODE_1000BASEX;
    if (phylink_test(link_modes, 100baseFX_Full))
    return PHY_INTERFACE_MODE_100BASEX;
    dev_warn(bus.sfp_dev, "Unable to ascertain link mode\n");
    return PHY_INTERFACE_MODE_NA;
    }
    EXPORT_SYMBOL_GPL(sfp_select_interface);
    static LIST_HEAD(sfp_buses);
    static DEFINE_MUTEX(sfp_mutex);
    static const struct sfp_upstream_ops *sfp_get_upstream_ops(struct sfp_bus *bus)
    {
    return bus.registered ? bus.upstream_ops : core::ptr::null_mut();
    }
    static struct sfp_bus *sfp_bus_get(const struct fwnode_handle *fwnode)
    {
    struct sfp_bus *sfp, *new, *found = core::ptr::null_mut();
    new = kzalloc_obj(*new);
    mutex_lock(&sfp_mutex);
    list_for_each_entry(sfp, &sfp_buses, node) {
    if (sfp.fwnode == fwnode) {
    kref_get(&sfp.kref);
    found = sfp;
    break;
    }
    }
    if (!found && new) {
    kref_init(&new.kref);
    new.fwnode = fwnode;
    list_add(&new.node, &sfp_buses);
    found = new;
    new = core::ptr::null_mut();
    }
    mutex_unlock(&sfp_mutex);
    kfree(new);
    return found;
    }
#[no_mangle]
unsafe extern "C" fn sfp_bus_release(kref: *mut kref) {
    static void sfp_bus_release(struct kref *kref)
    {
    struct sfp_bus *bus = container_of(kref, struct sfp_bus, kref);
    list_del(&bus.node);
    mutex_unlock(&sfp_mutex);
    kfree(bus);
    }
//
// sfp_bus_put() - put a reference on the &struct sfp_bus
// @bus: the &struct sfp_bus found via sfp_bus_find_fwnode()
//
// Put a reference on the &struct sfp_bus and free the underlying structure
// if this was the last reference.
//
#[no_mangle]
pub unsafe extern "C" fn sfp_bus_put(bus: *mut sfp_bus) {
    void sfp_bus_put(struct sfp_bus *bus)
    {
    if (bus)
    kref_put_mutex(&bus.kref, sfp_bus_release, &sfp_mutex);
    }
    EXPORT_SYMBOL_GPL(sfp_bus_put);
#[no_mangle]
unsafe extern "C" fn sfp_register_bus(bus: *mut sfp_bus) -> c_int {
    static int sfp_register_bus(struct sfp_bus *bus)
    {
    const struct sfp_upstream_ops *ops = bus.upstream_ops;
    int ret;
    if (ops) {
    if (ops.link_down)
    ops.link_down(bus.upstream);
    if (ops.connect_phy && bus.phydev) {
    ret = ops.connect_phy(bus.upstream, bus.phydev);
    if (ret)
    return ret;
    }
    }
    bus.registered = true;
    bus.socket_ops.attach(bus.sfp);
    if (bus.started)
    bus.socket_ops.start(bus.sfp);
    bus.upstream_ops.attach(bus.upstream, bus);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sfp_unregister_bus(bus: *mut sfp_bus) {
    static void sfp_unregister_bus(struct sfp_bus *bus)
    {
    const struct sfp_upstream_ops *ops = bus.upstream_ops;
    if (bus.registered) {
    bus.upstream_ops.detach(bus.upstream, bus);
    if (bus.started)
    bus.socket_ops.stop(bus.sfp);
    bus.socket_ops.detach(bus.sfp);
    if (bus.phydev && ops && ops.disconnect_phy)
    ops.disconnect_phy(bus.upstream, bus.phydev);
    }
    bus.registered = false;
    }
//
// sfp_get_module_info() - Get the ethtool_modinfo for a SFP module
// @bus: a pointer to the &struct sfp_bus structure for the sfp module
// @modinfo: a &struct ethtool_modinfo
//
// Fill in the type and eeprom_len parameters in @modinfo for a module on
// the sfp bus specified by @bus.
//
// Returns 0 on success or a negative errno number.
//
#[no_mangle]
pub unsafe extern "C" fn sfp_get_module_info(bus: *mut sfp_bus, modinfo: *mut ethtool_modinfo) -> c_int {
    int sfp_get_module_info(struct sfp_bus *bus, struct ethtool_modinfo *modinfo)
    {
    return bus.socket_ops.module_info(bus.sfp, modinfo);
    }
    EXPORT_SYMBOL_GPL(sfp_get_module_info);
//
// sfp_get_module_eeprom() - Read the SFP module EEPROM
// @bus: a pointer to the &struct sfp_bus structure for the sfp module
// @ee: a &struct ethtool_eeprom
// @data: buffer to contain the EEPROM data (must be at least @ee->len bytes)
//
// Read the EEPROM as specified by the supplied @ee. See the documentation
// for &struct ethtool_eeprom for the region to be read.
//
// Returns 0 on success or a negative errno number.
//
    int sfp_get_module_eeprom(struct sfp_bus *bus, struct ethtool_eeprom *ee,
    u8 *data)
    {
    return bus.socket_ops.module_eeprom(bus.sfp, ee, data);
    }
    EXPORT_SYMBOL_GPL(sfp_get_module_eeprom);
//
// sfp_get_module_eeprom_by_page() - Read a page from the SFP module EEPROM
// @bus: a pointer to the &struct sfp_bus structure for the sfp module
// @page: a &struct ethtool_module_eeprom
// @extack: extack for reporting problems
//
// Read an EEPROM page as specified by the supplied @page. See the
// documentation for &struct ethtool_module_eeprom for the page to be read.
//
// Returns 0 on success or a negative errno number. More error
// information might be provided via extack
//
    int sfp_get_module_eeprom_by_page(struct sfp_bus *bus,
    const struct ethtool_module_eeprom *page,
    struct netlink_ext_ack *extack)
    {
    return bus.socket_ops.module_eeprom_by_page(bus.sfp, page, extack);
    }
    EXPORT_SYMBOL_GPL(sfp_get_module_eeprom_by_page);
//
// sfp_upstream_start() - Inform the SFP that the network device is up
// @bus: a pointer to the &struct sfp_bus structure for the sfp module
//
// Inform the SFP socket that the network device is now up, so that the
// module can be enabled by allowing TX_DISABLE to be deasserted. This
// should be called from the network device driver's &struct net_device_ops
// ndo_open() method.
//
#[no_mangle]
pub unsafe extern "C" fn sfp_upstream_start(bus: *mut sfp_bus) {
    void sfp_upstream_start(struct sfp_bus *bus)
    {
    if (bus.registered)
    bus.socket_ops.start(bus.sfp);
    bus.started = true;
    }
    EXPORT_SYMBOL_GPL(sfp_upstream_start);
//
// sfp_upstream_stop() - Inform the SFP that the network device is down
// @bus: a pointer to the &struct sfp_bus structure for the sfp module
//
// Inform the SFP socket that the network device is now up, so that the
// module can be disabled by asserting TX_DISABLE, disabling the laser
// in optical modules. This should be called from the network device
// driver's &struct net_device_ops ndo_stop() method.
//
#[no_mangle]
pub unsafe extern "C" fn sfp_upstream_stop(bus: *mut sfp_bus) {
    void sfp_upstream_stop(struct sfp_bus *bus)
    {
    if (bus.registered)
    bus.socket_ops.stop(bus.sfp);
    bus.started = false;
    }
    EXPORT_SYMBOL_GPL(sfp_upstream_stop);
#[no_mangle]
unsafe extern "C" fn sfp_upstream_clear(bus: *mut sfp_bus) {
    static void sfp_upstream_clear(struct sfp_bus *bus)
    {
    bus.upstream_ops = core::ptr::null_mut();
    bus.upstream = core::ptr::null_mut();
    }
//
// sfp_upstream_set_signal_rate() - set data signalling rate
// @bus: a pointer to the &struct sfp_bus structure for the sfp module
// @rate_kbd: signalling rate in units of 1000 baud
//
// Configure the rate select settings on the SFP module for the signalling
// rate (not the same as the data rate).
//
// Locks that may be held:
// Phylink's state_mutex
// rtnl lock
// SFP's sm_mutex
//
#[no_mangle]
pub unsafe extern "C" fn sfp_upstream_set_signal_rate(bus: *mut sfp_bus, rate_kbd: c_uint) {
    void sfp_upstream_set_signal_rate(struct sfp_bus *bus, unsigned int rate_kbd)
    {
    if (bus.registered)
    bus.socket_ops.set_signal_rate(bus.sfp, rate_kbd);
    }
    EXPORT_SYMBOL_GPL(sfp_upstream_set_signal_rate);
//
// sfp_bus_find_fwnode() - parse and locate the SFP bus from fwnode
// @fwnode: firmware node for the parent device (MAC or PHY)
//
// Parse the parent device's firmware node for a SFP bus, and locate
// the sfp_bus structure, incrementing its reference count.  This must
// be put via sfp_bus_put() when done.
//
// Returns:
// - on success, a pointer to the sfp_bus structure,
// - %NULL if no SFP is specified,
// - on failure, an error pointer value:
//
// - corresponding to the errors detailed for
// fwnode_property_get_reference_args().
// - %-ENOMEM if we failed to allocate the bus.
// - an error from the upstream's connect_phy() method.
//
    struct sfp_bus *sfp_bus_find_fwnode(const struct fwnode_handle *fwnode)
    {
    struct fwnode_reference_args ref;
    struct sfp_bus *bus;
    int ret;
    ret = fwnode_property_get_reference_args(fwnode, "sfp", core::ptr::null_mut(),
    0, 0, &ref);
    if (ret == -ENOENT)
    return core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn if(0: ret <) -> else {
    else if (ret < 0)
    return ERR_PTR(ret);
    if (!fwnode_device_is_available(ref.fwnode)) {
    fwnode_handle_put(ref.fwnode);
    return core::ptr::null_mut();
    }
    bus = sfp_bus_get(ref.fwnode);
    fwnode_handle_put(ref.fwnode);
    if (!bus)
    return ERR_PTR(-ENOMEM);
    return bus;
    }
    EXPORT_SYMBOL_GPL(sfp_bus_find_fwnode);
//
// sfp_bus_add_upstream() - parse and register the neighbouring device
// @bus: the &struct sfp_bus found via sfp_bus_find_fwnode()
// @upstream: the upstream private data
// @ops: the upstream's &struct sfp_upstream_ops
//
// Add upstream driver for the SFP bus, and if the bus is complete, register
// the SFP bus using sfp_register_upstream().  This takes a reference on the
// bus, so it is safe to put the bus after this call.
//
// Returns:
// - on success, a pointer to the sfp_bus structure,
// - %NULL if no SFP is specified,
// - on failure, an error pointer value:
//
// - corresponding to the errors detailed for
// fwnode_property_get_reference_args().
// - %-ENOMEM if we failed to allocate the bus.
// - an error from the upstream's connect_phy() method.
//
    int sfp_bus_add_upstream(struct sfp_bus *bus, void *upstream,
    const struct sfp_upstream_ops *ops)
    {
    int ret;
// If no bus, return success
    if (!bus)
    return 0;
    rtnl_lock();
    kref_get(&bus.kref);
    bus.upstream_ops = ops;
    bus.upstream = upstream;
    if (bus.sfp) {
    ret = sfp_register_bus(bus);
    if (ret)
    sfp_upstream_clear(bus);
    } else {
    ret = 0;
    }
    rtnl_unlock();
    if (ret)
    sfp_bus_put(bus);
    return ret;
    }
    EXPORT_SYMBOL_GPL(sfp_bus_add_upstream);
//
// sfp_bus_del_upstream() - Delete a sfp bus
// @bus: a pointer to the &struct sfp_bus structure for the sfp module
//
// Delete a previously registered upstream connection for the SFP
// module. @bus should have been added by sfp_bus_add_upstream().
//
#[no_mangle]
pub unsafe extern "C" fn sfp_bus_del_upstream(bus: *mut sfp_bus) {
    void sfp_bus_del_upstream(struct sfp_bus *bus)
    {
    if (bus) {
    rtnl_lock();
    if (bus.sfp)
    sfp_unregister_bus(bus);
    sfp_upstream_clear(bus);
    rtnl_unlock();
    sfp_bus_put(bus);
    }
    }
    EXPORT_SYMBOL_GPL(sfp_bus_del_upstream);
//
// sfp_get_name() - Get the SFP device name
// @bus: a pointer to the &struct sfp_bus structure for the sfp module
//
// Gets the SFP device's name, if @bus has a registered socket. Callers must
// hold RTNL, and the returned name is only valid until RTNL is released.
//
// Returns:
// - The name of the SFP device registered with sfp_register_socket()
// - %NULL if no device was registered on @bus
//
    const char *sfp_get_name(struct sfp_bus *bus)
    {
    ASSERT_RTNL();
    if (bus.sfp_dev)
    return dev_name(bus.sfp_dev);
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(sfp_get_name);
// Socket driver entry points
#[no_mangle]
pub unsafe extern "C" fn sfp_add_phy(bus: *mut sfp_bus, phydev: *mut phy_device) -> c_int {
    int sfp_add_phy(struct sfp_bus *bus, struct phy_device *phydev)
    {
    const struct sfp_upstream_ops *ops = sfp_get_upstream_ops(bus);
    let mut ret: c_int = 0;
    if (ops && ops.connect_phy)
    ret = ops.connect_phy(bus.upstream, phydev);
    if (ret == 0)
    bus.phydev = phydev;
    return ret;
    }
    EXPORT_SYMBOL_GPL(sfp_add_phy);
#[no_mangle]
pub unsafe extern "C" fn sfp_remove_phy(bus: *mut sfp_bus) {
    void sfp_remove_phy(struct sfp_bus *bus)
    {
    const struct sfp_upstream_ops *ops = sfp_get_upstream_ops(bus);
    if (ops && ops.disconnect_phy)
    ops.disconnect_phy(bus.upstream, bus.phydev);
    bus.phydev = core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(sfp_remove_phy);
#[no_mangle]
pub unsafe extern "C" fn sfp_link_up(bus: *mut sfp_bus) {
    void sfp_link_up(struct sfp_bus *bus)
    {
    const struct sfp_upstream_ops *ops = sfp_get_upstream_ops(bus);
    if (ops && ops.link_up)
    ops.link_up(bus.upstream);
    }
    EXPORT_SYMBOL_GPL(sfp_link_up);
#[no_mangle]
pub unsafe extern "C" fn sfp_link_down(bus: *mut sfp_bus) {
    void sfp_link_down(struct sfp_bus *bus)
    {
    const struct sfp_upstream_ops *ops = sfp_get_upstream_ops(bus);
    if (ops && ops.link_down)
    ops.link_down(bus.upstream);
    }
    EXPORT_SYMBOL_GPL(sfp_link_down);
    int sfp_module_insert(struct sfp_bus *bus, const struct sfp_eeprom_id *id,
    const struct sfp_quirk *quirk)
    {
    const struct sfp_upstream_ops *ops = sfp_get_upstream_ops(bus);
    let mut ret: c_int = 0;
    sfp_init_module(bus, id, quirk);
    if (ops && ops.module_insert)
    ret = ops.module_insert(bus.upstream, id);
    return ret;
    }
    EXPORT_SYMBOL_GPL(sfp_module_insert);
#[no_mangle]
pub unsafe extern "C" fn sfp_module_remove(bus: *mut sfp_bus) {
    void sfp_module_remove(struct sfp_bus *bus)
    {
    const struct sfp_upstream_ops *ops = sfp_get_upstream_ops(bus);
    if (ops && ops.module_remove)
    ops.module_remove(bus.upstream);
    }
    EXPORT_SYMBOL_GPL(sfp_module_remove);
#[no_mangle]
pub unsafe extern "C" fn sfp_module_start(bus: *mut sfp_bus) -> c_int {
    int sfp_module_start(struct sfp_bus *bus)
    {
    const struct sfp_upstream_ops *ops = sfp_get_upstream_ops(bus);
    let mut ret: c_int = 0;
    if (ops && ops.module_start)
    ret = ops.module_start(bus.upstream);
    return ret;
    }
    EXPORT_SYMBOL_GPL(sfp_module_start);
#[no_mangle]
pub unsafe extern "C" fn sfp_module_stop(bus: *mut sfp_bus) {
    void sfp_module_stop(struct sfp_bus *bus)
    {
    const struct sfp_upstream_ops *ops = sfp_get_upstream_ops(bus);
    if (ops && ops.module_stop)
    ops.module_stop(bus.upstream);
    }
    EXPORT_SYMBOL_GPL(sfp_module_stop);
#[no_mangle]
unsafe extern "C" fn sfp_socket_clear(bus: *mut sfp_bus) {
    static void sfp_socket_clear(struct sfp_bus *bus)
    {
    bus.sfp_dev = core::ptr::null_mut();
    bus.sfp = core::ptr::null_mut();
    bus.socket_ops = core::ptr::null_mut();
    }
    struct sfp_bus *sfp_register_socket(struct device *dev, struct sfp *sfp,
    const struct sfp_socket_ops *ops)
    {
    struct sfp_bus *bus = sfp_bus_get(dev.fwnode);
    let mut ret: c_int = 0;
    if (bus) {
    rtnl_lock();
    bus.sfp_dev = dev;
    bus.sfp = sfp;
    bus.socket_ops = ops;
    if (bus.upstream_ops) {
    ret = sfp_register_bus(bus);
    if (ret)
    sfp_socket_clear(bus);
    }
    rtnl_unlock();
    }
    if (ret) {
    sfp_bus_put(bus);
    bus = core::ptr::null_mut();
    }
    return bus;
    }
    EXPORT_SYMBOL_GPL(sfp_register_socket);
#[no_mangle]
pub unsafe extern "C" fn sfp_unregister_socket(bus: *mut sfp_bus) {
    void sfp_unregister_socket(struct sfp_bus *bus)
    {
    rtnl_lock();
    if (bus.upstream_ops)
    sfp_unregister_bus(bus);
    sfp_socket_clear(bus);
    rtnl_unlock();
    sfp_bus_put(bus);
    }
    EXPORT_SYMBOL_GPL(sfp_unregister_socket);
