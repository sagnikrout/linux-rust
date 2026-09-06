//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/oki-semi/pch_gbe/pch_gbe_param.c
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
// Copyright (C) 1999 - 2010 Intel Corporation.
// Copyright (C) 2010 OKI SEMICONDUCTOR Co., LTD.
//
// This code was derived from the Intel e1000e Linux driver.
//

pub const OPTION_DISABLED: c_int = 0;
pub const OPTION_ENABLED: c_int = 1;
//
// TxDescriptors - Transmit Descriptor Count
// @Valid Range:   PCH_GBE_MIN_TXD - PCH_GBE_MAX_TXD
// @Default Value: PCH_GBE_DEFAULT_TXD
//
    let mut TxDescriptors: static int = OPTION_UNSET;
    module_param(TxDescriptors, int, 0);
    MODULE_PARM_DESC(TxDescriptors, "Number of transmit descriptors");
//
// RxDescriptors -Receive Descriptor Count
// @Valid Range:   PCH_GBE_MIN_RXD - PCH_GBE_MAX_RXD
// @Default Value: PCH_GBE_DEFAULT_RXD
//
    let mut RxDescriptors: static int = OPTION_UNSET;
    module_param(RxDescriptors, int, 0);
    MODULE_PARM_DESC(RxDescriptors, "Number of receive descriptors");
//
// Speed - User Specified Speed Override
// @Valid Range: 0, 10, 100, 1000
// - 0:    auto-negotiate at all supported speeds
// - 10:   only link at 10 Mbps
// - 100:  only link at 100 Mbps
// - 1000: only link at 1000 Mbps
// @Default Value: 0
//
    let mut Speed: static int = OPTION_UNSET;
    module_param(Speed, int, 0);
    MODULE_PARM_DESC(Speed, "Speed setting");
//
// Duplex - User Specified Duplex Override
// @Valid Range: 0-2
// - 0:  auto-negotiate for duplex
// - 1:  only link at half duplex
// - 2:  only link at full duplex
// @Default Value: 0
//
    let mut Duplex: static int = OPTION_UNSET;
    module_param(Duplex, int, 0);
    MODULE_PARM_DESC(Duplex, "Duplex setting");
pub const HALF_DUPLEX: c_int = 1;
pub const FULL_DUPLEX: c_int = 2;
//
// AutoNeg - Auto-negotiation Advertisement Override
// @Valid Range: 0x01-0x0F, 0x20-0x2F
//
// The AutoNeg value is a bit mask describing which speed and duplex
// combinations should be advertised during auto-negotiation.
// The supported speed and duplex modes are listed below
//
// Bit           7     6     5      4      3     2     1      0
// Speed (Mbps)  N/A   N/A   1000   N/A    100   100   10     10
// Duplex                    Full          Full  Half  Full   Half
//
// @Default Value: 0x2F (copper)
//
    let mut AutoNeg: static int = OPTION_UNSET;
    module_param(AutoNeg, int, 0);
    MODULE_PARM_DESC(AutoNeg, "Advertised auto-negotiation setting");
pub const PHY_ADVERTISE_10_HALF: c_uint = 0x0001;
pub const PHY_ADVERTISE_10_FULL: c_uint = 0x0002;
pub const PHY_ADVERTISE_100_HALF: c_uint = 0x0004;
pub const PHY_ADVERTISE_100_FULL: c_uint = 0x0008;
pub const PHY_ADVERTISE_1000_HALF: c_uint = 0x0010 /* Not used, just FYI */;
pub const PHY_ADVERTISE_1000_FULL: c_uint = 0x0020;
pub const PCH_AUTONEG_ADVERTISE_DEFAULT: c_uint = 0x2F;
//
// FlowControl - User Specified Flow Control Override
// @Valid Range: 0-3
// - 0:  No Flow Control
// - 1:  Rx only, respond to PAUSE frames but do not generate them
// - 2:  Tx only, generate PAUSE frames but ignore them on receive
// - 3:  Full Flow Control Support
// @Default Value: Read flow control settings from the EEPROM
//
    let mut FlowControl: static int = OPTION_UNSET;
    module_param(FlowControl, int, 0);
    MODULE_PARM_DESC(FlowControl, "Flow Control setting");
//
// XsumRX - Receive Checksum Offload Enable/Disable
// @Valid Range: 0, 1
// - 0:  disables all checksum offload
// - 1:  enables receive IP/TCP/UDP checksum offload
// @Default Value: PCH_GBE_DEFAULT_RX_CSUM
//
    let mut XsumRX: static int = OPTION_UNSET;
    module_param(XsumRX, int, 0);
    MODULE_PARM_DESC(XsumRX, "Disable or enable Receive Checksum offload");

//
// XsumTX - Transmit Checksum Offload Enable/Disable
// @Valid Range: 0, 1
// - 0:  disables all checksum offload
// - 1:  enables transmit IP/TCP/UDP checksum offload
// @Default Value: PCH_GBE_DEFAULT_TX_CSUM
//
    let mut XsumTX: static int = OPTION_UNSET;
    module_param(XsumTX, int, 0);
    MODULE_PARM_DESC(XsumTX, "Disable or enable Transmit Checksum offload");

//
// pch_gbe_option - Force the MAC's flow control settings
// @hw:	            Pointer to the HW structure
// Returns:
// 0:			Successful.
// Negative value:		Failed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pch_gbe_option {
    pub type: enum { enable_option, range_option, list_option },
    pub name: *mut c_char,
    pub err: *mut c_char,
    pub def: c_int,
    union {
    struct { /* range_option info */
    pub min: c_int,
    pub max: c_int,
    pub r: },
    struct { /* list_option info */
    pub nr: c_int,
    pub p: *const *const pch_gbe_opt_list { int i; char str; },
    pub l: },
    pub arg: },
}

    static const struct pch_gbe_opt_list speed_list[] = {
    { 0, "" },
    { SPEED_10, "" },
    { SPEED_100, "" },
    { SPEED_1000, "" }
    };
    static const struct pch_gbe_opt_list dplx_list[] = {
    { 0, "" },
    { HALF_DUPLEX, "" },
    { FULL_DUPLEX, "" }
    };
    static const struct pch_gbe_opt_list an_list[] =

    {{ 0x01, AA "10/HD" },
    { 0x02, AA "10/FD" },
    { 0x03, AA "10/FD, 10/HD" },
    { 0x04, AA "100/HD" },
    { 0x05, AA "100/HD, 10/HD" },
    { 0x06, AA "100/HD, 10/FD" },
    { 0x07, AA "100/HD, 10/FD, 10/HD" },
    { 0x08, AA "100/FD" },
    { 0x09, AA "100/FD, 10/HD" },
    { 0x0a, AA "100/FD, 10/FD" },
    { 0x0b, AA "100/FD, 10/FD, 10/HD" },
    { 0x0c, AA "100/FD, 100/HD" },
    { 0x0d, AA "100/FD, 100/HD, 10/HD" },
    { 0x0e, AA "100/FD, 100/HD, 10/FD" },
    { 0x0f, AA "100/FD, 100/HD, 10/FD, 10/HD" },
    { 0x20, AA "1000/FD" },
    { 0x21, AA "1000/FD, 10/HD" },
    { 0x22, AA "1000/FD, 10/FD" },
    { 0x23, AA "1000/FD, 10/FD, 10/HD" },
    { 0x24, AA "1000/FD, 100/HD" },
    { 0x25, AA "1000/FD, 100/HD, 10/HD" },
    { 0x26, AA "1000/FD, 100/HD, 10/FD" },
    { 0x27, AA "1000/FD, 100/HD, 10/FD, 10/HD" },
    { 0x28, AA "1000/FD, 100/FD" },
    { 0x29, AA "1000/FD, 100/FD, 10/HD" },
    { 0x2a, AA "1000/FD, 100/FD, 10/FD" },
    { 0x2b, AA "1000/FD, 100/FD, 10/FD, 10/HD" },
    { 0x2c, AA "1000/FD, 100/FD, 100/HD" },
    { 0x2d, AA "1000/FD, 100/FD, 100/HD, 10/HD" },
    { 0x2e, AA "1000/FD, 100/FD, 100/HD, 10/FD" },
    { 0x2f, AA "1000/FD, 100/FD, 100/HD, 10/FD, 10/HD" }
    };
    static const struct pch_gbe_opt_list fc_list[] = {
    { PCH_GBE_FC_NONE, "Flow Control Disabled" },
    { PCH_GBE_FC_RX_PAUSE, "Flow Control Receive Only" },
    { PCH_GBE_FC_TX_PAUSE, "Flow Control Transmit Only" },
    { PCH_GBE_FC_FULL, "Flow Control Enabled" }
    };
//
// pch_gbe_validate_option - Validate option
// @value:    value
// @opt:      option
// @adapter:  Board private structure
// Returns:
// 0:			Successful.
// Negative value:		Failed.
//
    static int pch_gbe_validate_option(int *value,
    const struct pch_gbe_option *opt,
    struct pch_gbe_adapter *adapter)
    {
    if (*value == OPTION_UNSET) {
// value = opt->def;
    return 0;
    }
    switch (opt.type) {
    case enable_option:
    switch (*value) {
    case OPTION_ENABLED:
    netdev_dbg(adapter.netdev, "%s Enabled\n", opt.name);
    return 0;
    case OPTION_DISABLED:
    netdev_dbg(adapter.netdev, "%s Disabled\n", opt.name);
    return 0;
    }
    break;
    case range_option:
    if (*value >= opt.arg.r.min && *value <= opt.arg.r.max) {
    netdev_dbg(adapter.netdev, "%s set to %i\n",
    opt.name, *value);
    return 0;
    }
    break;
    case list_option: {
    int i;
    const struct pch_gbe_opt_list *ent;
    for (i = 0; i < opt.arg.l.nr; i++) {
    ent = &opt.arg.l.p[i];
    if (*value == ent.i) {
    if (ent.str[0] != '\0')
    netdev_dbg(adapter.netdev, "%s\n",
    ent.str);
    return 0;
    }
    }
    }
    break;
    default:
    BUG();
    }
    netdev_dbg(adapter.netdev, "Invalid %s value specified (%i) %s\n",
    opt.name, *value, opt.err);
// value = opt->def;
    return -1;
    }
//
// pch_gbe_check_copper_options - Range Checking for Link Options, Copper Version
// @adapter:  Board private structure
//
#[no_mangle]
unsafe extern "C" fn pch_gbe_check_copper_options(adapter: *mut pch_gbe_adapter) {
    static void pch_gbe_check_copper_options(struct pch_gbe_adapter *adapter)
    {
    struct pch_gbe_hw *hw = &adapter.hw;
    int speed, dplx;
    { /* Speed */
    static const struct pch_gbe_option opt = {
    .type = list_option,
    .name = "Speed",
    .err  = "parameter ignored",
    .def  = 0,
    .arg  = { .l = { .nr = (int)ARRAY_SIZE(speed_list),
    .p = speed_list } }
    };
    speed = Speed;
    pch_gbe_validate_option(&speed, &opt, adapter);
    }
    { /* Duplex */
    static const struct pch_gbe_option opt = {
    .type = list_option,
    .name = "Duplex",
    .err  = "parameter ignored",
    .def  = 0,
    .arg  = { .l = { .nr = (int)ARRAY_SIZE(dplx_list),
    .p = dplx_list } }
    };
    dplx = Duplex;
    pch_gbe_validate_option(&dplx, &opt, adapter);
    }
    { /* Autoneg */
    static const struct pch_gbe_option opt = {
    .type = list_option,
    .name = "AutoNeg",
    .err  = "parameter ignored",
    .def  = PCH_AUTONEG_ADVERTISE_DEFAULT,
    .arg  = { .l = { .nr = (int)ARRAY_SIZE(an_list),
    .p = an_list} }
    };
    if (speed || dplx) {
    netdev_dbg(adapter.netdev,
    "AutoNeg specified along with Speed or Duplex, AutoNeg parameter ignored\n");
    hw.phy.autoneg_advertised = opt.def;
    } else {
    let mut tmp: c_int = AutoNeg;
    pch_gbe_validate_option(&tmp, &opt, adapter);
    hw.phy.autoneg_advertised = tmp;
    }
    }
    switch (speed + dplx) {
    case 0:
    hw.mac.autoneg = hw.mac.fc_autoneg = 1;
    if ((speed || dplx))
    netdev_dbg(adapter.netdev,
    "Speed and duplex autonegotiation enabled\n");
    hw.mac.link_speed = SPEED_10;
    hw.mac.link_duplex = DUPLEX_HALF;
    break;
    case HALF_DUPLEX:
    netdev_dbg(adapter.netdev,
    "Half Duplex specified without Speed\n");
    netdev_dbg(adapter.netdev,
    "Using Autonegotiation at Half Duplex only\n");
    hw.mac.autoneg = hw.mac.fc_autoneg = 1;
    hw.phy.autoneg_advertised = PHY_ADVERTISE_10_HALF |
    PHY_ADVERTISE_100_HALF;
    hw.mac.link_speed = SPEED_10;
    hw.mac.link_duplex = DUPLEX_HALF;
    break;
    case FULL_DUPLEX:
    netdev_dbg(adapter.netdev,
    "Full Duplex specified without Speed\n");
    netdev_dbg(adapter.netdev,
    "Using Autonegotiation at Full Duplex only\n");
    hw.mac.autoneg = hw.mac.fc_autoneg = 1;
    hw.phy.autoneg_advertised = PHY_ADVERTISE_10_FULL |
    PHY_ADVERTISE_100_FULL |
    PHY_ADVERTISE_1000_FULL;
    hw.mac.link_speed = SPEED_10;
    hw.mac.link_duplex = DUPLEX_FULL;
    break;
    case SPEED_10:
    netdev_dbg(adapter.netdev,
    "10 Mbps Speed specified without Duplex\n");
    netdev_dbg(adapter.netdev,
    "Using Autonegotiation at 10 Mbps only\n");
    hw.mac.autoneg = hw.mac.fc_autoneg = 1;
    hw.phy.autoneg_advertised = PHY_ADVERTISE_10_HALF |
    PHY_ADVERTISE_10_FULL;
    hw.mac.link_speed = SPEED_10;
    hw.mac.link_duplex = DUPLEX_HALF;
    break;
    case SPEED_10 + HALF_DUPLEX:
    netdev_dbg(adapter.netdev, "Forcing to 10 Mbps Half Duplex\n");
    hw.mac.autoneg = hw.mac.fc_autoneg = 0;
    hw.phy.autoneg_advertised = 0;
    hw.mac.link_speed = SPEED_10;
    hw.mac.link_duplex = DUPLEX_HALF;
    break;
    case SPEED_10 + FULL_DUPLEX:
    netdev_dbg(adapter.netdev, "Forcing to 10 Mbps Full Duplex\n");
    hw.mac.autoneg = hw.mac.fc_autoneg = 0;
    hw.phy.autoneg_advertised = 0;
    hw.mac.link_speed = SPEED_10;
    hw.mac.link_duplex = DUPLEX_FULL;
    break;
    case SPEED_100:
    netdev_dbg(adapter.netdev,
    "100 Mbps Speed specified without Duplex\n");
    netdev_dbg(adapter.netdev,
    "Using Autonegotiation at 100 Mbps only\n");
    hw.mac.autoneg = hw.mac.fc_autoneg = 1;
    hw.phy.autoneg_advertised = PHY_ADVERTISE_100_HALF |
    PHY_ADVERTISE_100_FULL;
    hw.mac.link_speed = SPEED_100;
    hw.mac.link_duplex = DUPLEX_HALF;
    break;
    case SPEED_100 + HALF_DUPLEX:
    netdev_dbg(adapter.netdev,
    "Forcing to 100 Mbps Half Duplex\n");
    hw.mac.autoneg = hw.mac.fc_autoneg = 0;
    hw.phy.autoneg_advertised = 0;
    hw.mac.link_speed = SPEED_100;
    hw.mac.link_duplex = DUPLEX_HALF;
    break;
    case SPEED_100 + FULL_DUPLEX:
    netdev_dbg(adapter.netdev,
    "Forcing to 100 Mbps Full Duplex\n");
    hw.mac.autoneg = hw.mac.fc_autoneg = 0;
    hw.phy.autoneg_advertised = 0;
    hw.mac.link_speed = SPEED_100;
    hw.mac.link_duplex = DUPLEX_FULL;
    break;
    case SPEED_1000:
    netdev_dbg(adapter.netdev,
    "1000 Mbps Speed specified without Duplex\n");
    goto full_duplex_only;
    case SPEED_1000 + HALF_DUPLEX:
    netdev_dbg(adapter.netdev,
    "Half Duplex is not supported at 1000 Mbps\n");
    fallthrough;
    case SPEED_1000 + FULL_DUPLEX:
    full_duplex_only:
    netdev_dbg(adapter.netdev,
    "Using Autonegotiation at 1000 Mbps Full Duplex only\n");
    hw.mac.autoneg = hw.mac.fc_autoneg = 1;
    hw.phy.autoneg_advertised = PHY_ADVERTISE_1000_FULL;
    hw.mac.link_speed = SPEED_1000;
    hw.mac.link_duplex = DUPLEX_FULL;
    break;
    default:
    BUG();
    }
    }
//
// pch_gbe_check_options - Range Checking for Command Line Parameters
// @adapter:  Board private structure
//
#[no_mangle]
pub unsafe extern "C" fn pch_gbe_check_options(adapter: *mut pch_gbe_adapter) {
    void pch_gbe_check_options(struct pch_gbe_adapter *adapter)
    {
    struct pch_gbe_hw *hw = &adapter.hw;
    struct net_device *dev = adapter.netdev;
    int val;
    { /* Transmit Descriptor Count */
    static const struct pch_gbe_option opt = {
    .type = range_option,
    .name = "Transmit Descriptors",
    .err  = "using default of "
    __MODULE_STRING(PCH_GBE_DEFAULT_TXD),
    .def  = PCH_GBE_DEFAULT_TXD,
    .arg  = { .r = { .min = PCH_GBE_MIN_TXD,
    .max = PCH_GBE_MAX_TXD } }
    };
    struct pch_gbe_tx_ring *tx_ring = adapter.tx_ring;
    tx_ring.count = TxDescriptors;
    pch_gbe_validate_option(&tx_ring.count, &opt, adapter);
    tx_ring.count = roundup(tx_ring.count,
    PCH_GBE_TX_DESC_MULTIPLE);
    }
    { /* Receive Descriptor Count */
    static const struct pch_gbe_option opt = {
    .type = range_option,
    .name = "Receive Descriptors",
    .err  = "using default of "
    __MODULE_STRING(PCH_GBE_DEFAULT_RXD),
    .def  = PCH_GBE_DEFAULT_RXD,
    .arg  = { .r = { .min = PCH_GBE_MIN_RXD,
    .max = PCH_GBE_MAX_RXD } }
    };
    struct pch_gbe_rx_ring *rx_ring = adapter.rx_ring;
    rx_ring.count = RxDescriptors;
    pch_gbe_validate_option(&rx_ring.count, &opt, adapter);
    rx_ring.count = roundup(rx_ring.count,
    PCH_GBE_RX_DESC_MULTIPLE);
    }
    { /* Checksum Offload Enable/Disable */
    static const struct pch_gbe_option opt = {
    .type = enable_option,
    .name = "Checksum Offload",
    .err  = "defaulting to Enabled",
    .def  = PCH_GBE_DEFAULT_RX_CSUM
    };
    val = XsumRX;
    pch_gbe_validate_option(&val, &opt, adapter);
    if (!val)
    dev.features &= ~NETIF_F_RXCSUM;
    }
    { /* Checksum Offload Enable/Disable */
    static const struct pch_gbe_option opt = {
    .type = enable_option,
    .name = "Checksum Offload",
    .err  = "defaulting to Enabled",
    .def  = PCH_GBE_DEFAULT_TX_CSUM
    };
    val = XsumTX;
    pch_gbe_validate_option(&val, &opt, adapter);
    if (!val)
    dev.features &= ~NETIF_F_CSUM_MASK;
    }
    { /* Flow Control */
    static const struct pch_gbe_option opt = {
    .type = list_option,
    .name = "Flow Control",
    .err  = "reading default settings from EEPROM",
    .def  = PCH_GBE_FC_DEFAULT,
    .arg  = { .l = { .nr = (int)ARRAY_SIZE(fc_list),
    .p = fc_list } }
    };
    let mut tmp: c_int = FlowControl;
    pch_gbe_validate_option(&tmp, &opt, adapter);
    hw.mac.fc = tmp;
    }
    pch_gbe_check_copper_options(adapter);
    }
