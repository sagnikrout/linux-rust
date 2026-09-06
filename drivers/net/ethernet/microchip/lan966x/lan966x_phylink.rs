//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/microchip/lan966x/lan966x_phylink.c
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

    static struct phylink_pcs *lan966x_phylink_mac_select(struct phylink_config *config,
    phy_interface_t interface)
    {
    struct lan966x_port *port = netdev_priv(to_net_dev(config.dev));
    return &port.phylink_pcs;
    }
    static void lan966x_phylink_mac_config(struct phylink_config *config,
    unsigned int mode,
    const struct phylink_link_state *state)
    {
    }
    static int lan966x_phylink_mac_prepare(struct phylink_config *config,
    unsigned int mode,
    phy_interface_t iface)
    {
    struct lan966x_port *port = netdev_priv(to_net_dev(config.dev));
    let mut serdes_mode: phy_interface_t = iface;
    int err;
    if (port.serdes) {
    err = phy_set_mode_ext(port.serdes, PHY_MODE_ETHERNET,
    serdes_mode);
    if (err) {
    netdev_err(to_net_dev(config.dev),
    "Could not set mode of SerDes\n");
    return err;
    }
    }
    return 0;
    }
    static void lan966x_phylink_mac_link_up(struct phylink_config *config,
    struct phy_device *phy,
    unsigned int mode,
    phy_interface_t interface,
    int speed, int duplex,
    bool tx_pause, bool rx_pause)
    {
    struct lan966x_port *port = netdev_priv(to_net_dev(config.dev));
    struct lan966x_port_config *port_config = &port.config;
    port_config.duplex = duplex;
    port_config.speed = speed;
    port_config.pause = 0;
    port_config.pause |= tx_pause ? MLO_PAUSE_TX : 0;
    port_config.pause |= rx_pause ? MLO_PAUSE_RX : 0;
    if (phy_interface_mode_is_rgmii(interface))
    phy_set_speed(port.serdes, speed);
    lan966x_port_config_up(port);
    }
    static void lan966x_phylink_mac_link_down(struct phylink_config *config,
    unsigned int mode,
    phy_interface_t interface)
    {
    struct lan966x_port *port = netdev_priv(to_net_dev(config.dev));
    struct lan966x *lan966x = port.lan966x;
    lan966x_port_config_down(port);
// Take PCS out of reset
    lan_rmw(DEV_CLOCK_CFG_PCS_RX_RST_SET(0) |
    DEV_CLOCK_CFG_PCS_TX_RST_SET(0),
    DEV_CLOCK_CFG_PCS_RX_RST |
    DEV_CLOCK_CFG_PCS_TX_RST,
    lan966x, DEV_CLOCK_CFG(port.chip_port));
    }
    static struct lan966x_port *lan966x_pcs_to_port(struct phylink_pcs *pcs)
    {
    return container_of(pcs, struct lan966x_port, phylink_pcs);
    }
    static void lan966x_pcs_get_state(struct phylink_pcs *pcs,
    unsigned int neg_mode,
    struct phylink_link_state *state)
    {
    struct lan966x_port *port = lan966x_pcs_to_port(pcs);
    lan966x_port_status_get(port, neg_mode, state);
    }
    static int lan966x_pcs_config(struct phylink_pcs *pcs, unsigned int neg_mode,
    phy_interface_t interface,
    const unsigned long *advertising,
    bool permit_pause_to_mac)
    {
    struct lan966x_port *port = lan966x_pcs_to_port(pcs);
    struct lan966x_port_config config;
    int ret;
    config = port.config;
    config.portmode = interface;
    config.inband = neg_mode & PHYLINK_PCS_NEG_INBAND;
    config.autoneg = neg_mode == PHYLINK_PCS_NEG_INBAND_ENABLED;
    config.advertising = advertising;
    ret = lan966x_port_pcs_set(port, &config);
    if (ret)
    netdev_err(port.dev, "port PCS config failed: %d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn lan966x_pcs_aneg_restart(pcs: *mut phylink_pcs) {
    static void lan966x_pcs_aneg_restart(struct phylink_pcs *pcs)
    {
// Currently not used
    }
    const struct phylink_mac_ops lan966x_phylink_mac_ops = {
    .mac_select_pcs = lan966x_phylink_mac_select,
    .mac_config = lan966x_phylink_mac_config,
    .mac_prepare = lan966x_phylink_mac_prepare,
    .mac_link_down = lan966x_phylink_mac_link_down,
    .mac_link_up = lan966x_phylink_mac_link_up,
    };
    const struct phylink_pcs_ops lan966x_phylink_pcs_ops = {
    .pcs_get_state = lan966x_pcs_get_state,
    .pcs_config = lan966x_pcs_config,
    .pcs_an_restart = lan966x_pcs_aneg_restart,
    };
