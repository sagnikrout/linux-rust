//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/microchip/sparx5/lan969x/lan969x_rgmii.c
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
// Microchip lan969x Switch driver
//
// Copyright (c) 2024 Microchip Technology Inc. and its subsidiaries.
//

// Tx clock selectors

// Port speed selectors

// Clock delay selectors

// Get the tx clock selector based on the port speed.
#[no_mangle]
unsafe extern "C" fn lan969x_rgmii_get_clk_sel(speed: c_int) -> c_int {
    static int lan969x_rgmii_get_clk_sel(int speed)
    {
    return (speed == SPEED_10  ? LAN969X_RGMII_TX_CLK_SEL_2M5MHZ :
    speed == SPEED_100 ? LAN969X_RGMII_TX_CLK_SEL_25MHZ :
    LAN969X_RGMII_TX_CLK_SEL_125MHZ);
    }
// Get the port speed selector based on the port speed.
#[no_mangle]
unsafe extern "C" fn lan969x_rgmii_get_speed_sel(speed: c_int) -> c_int {
    static int lan969x_rgmii_get_speed_sel(int speed)
    {
    return (speed == SPEED_10  ? LAN969X_RGMII_SPEED_SEL_10 :
    speed == SPEED_100 ? LAN969X_RGMII_SPEED_SEL_100 :
    LAN969X_RGMII_SPEED_SEL_1000);
    }
// Get the clock delay selector based on the clock delay in picoseconds.
    static int lan969x_rgmii_get_clk_delay_sel(struct sparx5_port *port,
    u32 delay_ps, u32 *clk_delay_sel)
    {
    switch (delay_ps) {
    case 0:
// Hardware default selector.
// clk_delay_sel = LAN969X_RGMII_CLK_DELAY_SEL_2_5_NS;
    break;
    case 1000:
// clk_delay_sel = LAN969X_RGMII_CLK_DELAY_SEL_1_0_NS;
    break;
    case 1700:
// clk_delay_sel = LAN969X_RGMII_CLK_DELAY_SEL_1_7_NS;
    break;
    case 2000:
// clk_delay_sel = LAN969X_RGMII_CLK_DELAY_SEL_2_0_NS;
    break;
    case 2500:
// clk_delay_sel = LAN969X_RGMII_CLK_DELAY_SEL_2_5_NS;
    break;
    case 3000:
// clk_delay_sel = LAN969X_RGMII_CLK_DELAY_SEL_3_0_NS;
    break;
    case 3300:
// clk_delay_sel = LAN969X_RGMII_CLK_DELAY_SEL_3_3_NS;
    break;
    default:
    dev_err(port.sparx5.dev, "Invalid RGMII delay: %u", delay_ps);
    return -EINVAL;
    }
    return 0;
    }
// Configure the RGMII tx clock frequency.
    static void lan969x_rgmii_tx_clk_config(struct sparx5_port *port,
    struct sparx5_port_config *conf)
    {
    let mut clk_sel: u32 = lan969x_rgmii_get_clk_sel(conf.speed);
    let mut idx: u32 = RGMII_PORT_IDX(port);
// Take the RGMII clock domain out of reset and set tx clock
// frequency.
//
    spx5_rmw(HSIO_WRAP_RGMII_CFG_TX_CLK_CFG_SET(clk_sel) |
    HSIO_WRAP_RGMII_CFG_RGMII_TX_RST_SET(0) |
    HSIO_WRAP_RGMII_CFG_RGMII_RX_RST_SET(0),
    HSIO_WRAP_RGMII_CFG_TX_CLK_CFG |
    HSIO_WRAP_RGMII_CFG_RGMII_TX_RST |
    HSIO_WRAP_RGMII_CFG_RGMII_RX_RST,
    port.sparx5, HSIO_WRAP_RGMII_CFG(idx));
    }
// Configure the RGMII port device.
    static void lan969x_rgmii_port_device_config(struct sparx5_port *port,
    struct sparx5_port_config *conf)
    {
    u32 dtag, dotag, etype, speed_sel, idx = RGMII_PORT_IDX(port);
    speed_sel = lan969x_rgmii_get_speed_sel(conf.speed);
    etype = (port.vlan_type == SPX5_VLAN_PORT_TYPE_S_CUSTOM ?
    port.custom_etype :
    port.vlan_type == SPX5_VLAN_PORT_TYPE_C ?
    ETH_P_8021Q : ETH_P_8021AD);
    dtag = port.max_vlan_tags == SPX5_PORT_MAX_TAGS_TWO;
    dotag = port.max_vlan_tags != SPX5_PORT_MAX_TAGS_NONE;
// Enable the MAC.
    spx5_wr(DEVRGMII_MAC_ENA_CFG_RX_ENA_SET(1) |
    DEVRGMII_MAC_ENA_CFG_TX_ENA_SET(1),
    port.sparx5, DEVRGMII_MAC_ENA_CFG(idx));
// Configure the Inter Frame Gap.
    spx5_wr(DEVRGMII_MAC_IFG_CFG_TX_IFG_SET(LAN969X_RGMII_IFG_TX) |
    DEVRGMII_MAC_IFG_CFG_RX_IFG1_SET(LAN969X_RGMII_IFG_RX1) |
    DEVRGMII_MAC_IFG_CFG_RX_IFG2_SET(LAN969X_RGMII_IFG_RX2),
    port.sparx5, DEVRGMII_MAC_IFG_CFG(idx));
// Configure port data rate.
    spx5_wr(DEVRGMII_DEV_RST_CTRL_SPEED_SEL_SET(speed_sel),
    port.sparx5, DEVRGMII_DEV_RST_CTRL(idx));
// Configure VLAN awareness.
    spx5_wr(DEVRGMII_MAC_TAGS_CFG_TAG_ID_SET(etype) |
    DEVRGMII_MAC_TAGS_CFG_PB_ENA_SET(dtag) |
    DEVRGMII_MAC_TAGS_CFG_VLAN_AWR_ENA_SET(dotag) |
    DEVRGMII_MAC_TAGS_CFG_VLAN_LEN_AWR_ENA_SET(dotag),
    port.sparx5,
    DEVRGMII_MAC_TAGS_CFG(idx));
    }
// Configure the RGMII delay lines in the MAC.
//
// We use the rx-internal-delay-ps" and "tx-internal-delay-ps" properties to
// configure the rx and tx delays for the MAC. If these properties are missing
// or set to zero, the MAC will not apply any delay.
//
// The PHY side delays are determined by the PHY mode
// (e.g. PHY_INTERFACE_MODE_RGMII_{ID, RXID, TXID}), and ignored by the MAC side
// entirely.
//
    static int lan969x_rgmii_delay_config(struct sparx5_port *port,
    struct sparx5_port_config *conf)
    {
    u32 tx_clk_sel, rx_clk_sel, tx_delay_ps = 0, rx_delay_ps = 0;
    let mut idx: u32 = RGMII_PORT_IDX(port);
    int err;
    of_property_read_u32(port.of_node, "rx-internal-delay-ps",
    &rx_delay_ps);
    of_property_read_u32(port.of_node, "tx-internal-delay-ps",
    &tx_delay_ps);
    err = lan969x_rgmii_get_clk_delay_sel(port, rx_delay_ps, &rx_clk_sel);
    if (err)
    return err;
    err = lan969x_rgmii_get_clk_delay_sel(port, tx_delay_ps, &tx_clk_sel);
    if (err)
    return err;
// Configure rx delay.
    spx5_rmw(HSIO_WRAP_DLL_CFG_DLL_RST_SET(0) |
    HSIO_WRAP_DLL_CFG_DLL_ENA_SET(1) |
    HSIO_WRAP_DLL_CFG_DLL_CLK_ENA_SET(!!rx_delay_ps) |
    HSIO_WRAP_DLL_CFG_DLL_CLK_SEL_SET(rx_clk_sel),
    HSIO_WRAP_DLL_CFG_DLL_RST |
    HSIO_WRAP_DLL_CFG_DLL_ENA |
    HSIO_WRAP_DLL_CFG_DLL_CLK_ENA |
    HSIO_WRAP_DLL_CFG_DLL_CLK_SEL,
    port.sparx5, HSIO_WRAP_DLL_CFG(idx, 0));
// Configure tx delay.
    spx5_rmw(HSIO_WRAP_DLL_CFG_DLL_RST_SET(0) |
    HSIO_WRAP_DLL_CFG_DLL_ENA_SET(1) |
    HSIO_WRAP_DLL_CFG_DLL_CLK_ENA_SET(!!tx_delay_ps) |
    HSIO_WRAP_DLL_CFG_DLL_CLK_SEL_SET(tx_clk_sel),
    HSIO_WRAP_DLL_CFG_DLL_RST |
    HSIO_WRAP_DLL_CFG_DLL_ENA |
    HSIO_WRAP_DLL_CFG_DLL_CLK_ENA |
    HSIO_WRAP_DLL_CFG_DLL_CLK_SEL,
    port.sparx5, HSIO_WRAP_DLL_CFG(idx, 1));
    return 0;
    }
// Configure GPIO's to be used as RGMII interface.
#[no_mangle]
unsafe extern "C" fn lan969x_rgmii_gpio_config(port: *mut sparx5_port) {
    static void lan969x_rgmii_gpio_config(struct sparx5_port *port)
    {
    let mut idx: u32 = RGMII_PORT_IDX(port);
// Enable the RGMII on the GPIOs.
    spx5_wr(HSIO_WRAP_XMII_CFG_GPIO_XMII_CFG_SET(1), port.sparx5,
    HSIO_WRAP_XMII_CFG(!idx));
    }
    int lan969x_port_config_rgmii(struct sparx5_port *port,
    struct sparx5_port_config *conf)
    {
    int err;
    err = lan969x_rgmii_delay_config(port, conf);
    if (err)
    return err;
    lan969x_rgmii_tx_clk_config(port, conf);
    lan969x_rgmii_gpio_config(port);
    lan969x_rgmii_port_device_config(port, conf);
    return 0;
    }
