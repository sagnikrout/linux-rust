//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/microchip/lan966x/lan966x_dcb.c
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

    enum lan966x_dcb_apptrust_values {
    LAN966X_DCB_APPTRUST_EMPTY,
    LAN966X_DCB_APPTRUST_DSCP,
    LAN966X_DCB_APPTRUST_PCP,
    LAN966X_DCB_APPTRUST_DSCP_PCP,
    __LAN966X_DCB_APPTRUST_MAX
    };
    static const struct lan966x_dcb_apptrust {
    u8 selectors[IEEE_8021QAZ_APP_SEL_MAX + 1];
    int nselectors;
    } *lan966x_port_apptrust[NUM_PHYS_PORTS];
    static const char *lan966x_dcb_apptrust_names[__LAN966X_DCB_APPTRUST_MAX] = {
    [LAN966X_DCB_APPTRUST_EMPTY]    = "empty",
    [LAN966X_DCB_APPTRUST_DSCP]     = "dscp",
    [LAN966X_DCB_APPTRUST_PCP]      = "pcp",
    [LAN966X_DCB_APPTRUST_DSCP_PCP] = "dscp pcp"
    };
// Lan966x supported apptrust policies
    static const struct lan966x_dcb_apptrust
    lan966x_dcb_apptrust_policies[__LAN966X_DCB_APPTRUST_MAX] = {
// Empty *must* be first
    [LAN966X_DCB_APPTRUST_EMPTY]    = { { 0 }, 0 },
    [LAN966X_DCB_APPTRUST_DSCP]     = { { IEEE_8021QAZ_APP_SEL_DSCP }, 1 },
    [LAN966X_DCB_APPTRUST_PCP]      = { { DCB_APP_SEL_PCP }, 1 },
    [LAN966X_DCB_APPTRUST_DSCP_PCP] = { { IEEE_8021QAZ_APP_SEL_DSCP,
    DCB_APP_SEL_PCP }, 2 },
    };
#[no_mangle]
unsafe extern "C" fn lan966x_dcb_apptrust_contains(portno: c_int, selector: u8) -> bool {
    static bool lan966x_dcb_apptrust_contains(int portno, u8 selector)
    {
    const struct lan966x_dcb_apptrust *conf = lan966x_port_apptrust[portno];
    for (int i = 0; i < conf.nselectors; i++)
    if (conf.selectors[i] == selector)
    return true;
    return false;
    }
#[no_mangle]
unsafe extern "C" fn lan966x_dcb_app_update(dev: *mut net_device) {
    static void lan966x_dcb_app_update(struct net_device *dev)
    {
    let mut dscp_rewr_map: dcb_ieee_app_prio_map = {0};
    let mut pcp_rewr_map: dcb_rewr_prio_pcp_map = {0};
    struct lan966x_port *port = netdev_priv(dev);
    let mut qos: lan966x_port_qos = {0};
    struct dcb_app app_itr;
    let mut dscp_rewr: bool = false;
    let mut pcp_rewr: bool = false;
// Get pcp ingress mapping
    for (int i = 0; i < ARRAY_SIZE(qos.pcp.map); i++) {
    app_itr.selector = DCB_APP_SEL_PCP;
    app_itr.protocol = i;
    qos.pcp.map[i] = dcb_getapp(dev, &app_itr);
    }
// Get dscp ingress mapping
    for (int i = 0; i < ARRAY_SIZE(qos.dscp.map); i++) {
    app_itr.selector = IEEE_8021QAZ_APP_SEL_DSCP;
    app_itr.protocol = i;
    qos.dscp.map[i] = dcb_getapp(dev, &app_itr);
    }
// Get default prio
    qos.default_prio = dcb_ieee_getapp_default_prio_mask(dev);
    if (qos.default_prio)
    qos.default_prio = fls(qos.default_prio) - 1;
// Get pcp rewrite mapping
    dcb_getrewr_prio_pcp_mask_map(dev, &pcp_rewr_map);
    for (int i = 0; i < ARRAY_SIZE(pcp_rewr_map.map); i++) {
    if (!pcp_rewr_map.map[i])
    continue;
    pcp_rewr = true;
    qos.pcp_rewr.map[i] = fls(pcp_rewr_map.map[i]) - 1;
    }
// Get dscp rewrite mapping
    dcb_getrewr_prio_dscp_mask_map(dev, &dscp_rewr_map);
    for (int i = 0; i < ARRAY_SIZE(dscp_rewr_map.map); i++) {
    if (!dscp_rewr_map.map[i])
    continue;
    dscp_rewr = true;
    qos.dscp_rewr.map[i] = fls64(dscp_rewr_map.map[i]) - 1;
    }
// Enable use of pcp for queue classification
    if (lan966x_dcb_apptrust_contains(port.chip_port, DCB_APP_SEL_PCP)) {
    qos.pcp.enable = true;
    if (pcp_rewr)
    qos.pcp_rewr.enable = true;
    }
// Enable use of dscp for queue classification
    if (lan966x_dcb_apptrust_contains(port.chip_port, IEEE_8021QAZ_APP_SEL_DSCP)) {
    qos.dscp.enable = true;
    if (dscp_rewr)
    qos.dscp_rewr.enable = true;
    }
    lan966x_port_qos_set(port, &qos);
    }
// DSCP mapping is global for all ports, so set and delete app entries are
// replicated for each port.
//
    static int lan966x_dcb_ieee_dscp_setdel(struct net_device *dev,
    struct dcb_app *app,
    int (*setdel)(struct net_device *,
    struct dcb_app *))
    {
    struct lan966x_port *port = netdev_priv(dev);
    struct lan966x *lan966x = port.lan966x;
    int err;
    for (int i = 0; i < NUM_PHYS_PORTS; i++) {
    port = lan966x.ports[i];
    if (!port)
    continue;
    err = setdel(port.dev, app);
    if (err)
    return err;
    }
    return 0;
    }
    static int lan966x_dcb_app_validate(struct net_device *dev,
    const struct dcb_app *app)
    {
    let mut err: c_int = 0;
    switch (app.selector) {
// Default priority checks
    case IEEE_8021QAZ_APP_SEL_ETHERTYPE:
    if (app.protocol)
    err = -EINVAL;
#[no_mangle]
pub unsafe extern "C" fn if(NUM_PRIO_QUEUES: app->priority >=) -> else {
    else if (app.priority >= NUM_PRIO_QUEUES)
    err = -ERANGE;
    break;
// Dscp checks
    case IEEE_8021QAZ_APP_SEL_DSCP:
    if (app.protocol >= LAN966X_PORT_QOS_DSCP_COUNT)
    err = -EINVAL;
#[no_mangle]
pub unsafe extern "C" fn if(NUM_PRIO_QUEUES: app->priority >=) -> else {
    else if (app.priority >= NUM_PRIO_QUEUES)
    err = -ERANGE;
    break;
// Pcp checks
    case DCB_APP_SEL_PCP:
    if (app.protocol >= LAN966X_PORT_QOS_PCP_DEI_COUNT)
    err = -EINVAL;
#[no_mangle]
pub unsafe extern "C" fn if(NUM_PRIO_QUEUES: app->priority >=) -> else {
    else if (app.priority >= NUM_PRIO_QUEUES)
    err = -ERANGE;
    break;
    default:
    err = -EINVAL;
    break;
    }
    if (err)
    netdev_err(dev, "Invalid entry: %d:%d\n", app.protocol,
    app.priority);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn lan966x_dcb_ieee_delapp(dev: *mut net_device, app: *mut dcb_app) -> c_int {
    static int lan966x_dcb_ieee_delapp(struct net_device *dev, struct dcb_app *app)
    {
    int err;
    if (app.selector == IEEE_8021QAZ_APP_SEL_DSCP)
    err = lan966x_dcb_ieee_dscp_setdel(dev, app, dcb_ieee_delapp);
    else
    err = dcb_ieee_delapp(dev, app);
    if (err)
    return err;
    lan966x_dcb_app_update(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lan966x_dcb_ieee_setapp(dev: *mut net_device, app: *mut dcb_app) -> c_int {
    static int lan966x_dcb_ieee_setapp(struct net_device *dev, struct dcb_app *app)
    {
    struct dcb_app app_itr;
    int err;
    u8 prio;
    err = lan966x_dcb_app_validate(dev, app);
    if (err)
    return err;
// Delete current mapping, if it exists
    prio = dcb_getapp(dev, app);
    if (prio) {
    app_itr = *app;
    app_itr.priority = prio;
    lan966x_dcb_ieee_delapp(dev, &app_itr);
    }
    if (app.selector == IEEE_8021QAZ_APP_SEL_DSCP)
    err = lan966x_dcb_ieee_dscp_setdel(dev, app, dcb_ieee_setapp);
    else
    err = dcb_ieee_setapp(dev, app);
    if (err)
    return err;
    lan966x_dcb_app_update(dev);
    return 0;
    }
    static int lan966x_dcb_apptrust_validate(struct net_device *dev,
    u8 *selectors,
    int nselectors)
    {
    for (int i = 0; i < ARRAY_SIZE(lan966x_dcb_apptrust_policies); i++) {
    bool match;
    if (lan966x_dcb_apptrust_policies[i].nselectors != nselectors)
    continue;
    match = true;
    for (int j = 0; j < nselectors; j++) {
    if (lan966x_dcb_apptrust_policies[i].selectors[j] !=
// (selectors + j)) {
    match = false;
    break;
    }
    }
    if (match)
    return i;
    }
    netdev_err(dev, "Valid apptrust configurations are:\n");
    for (int i = 0; i < ARRAY_SIZE(lan966x_dcb_apptrust_names); i++)
    pr_info("order: %s\n", lan966x_dcb_apptrust_names[i]);
    return -EOPNOTSUPP;
    }
    static int lan966x_dcb_setapptrust(struct net_device *dev,
    u8 *selectors,
    int nselectors)
    {
    struct lan966x_port *port = netdev_priv(dev);
    int idx;
    idx = lan966x_dcb_apptrust_validate(dev, selectors, nselectors);
    if (idx < 0)
    return idx;
    lan966x_port_apptrust[port.chip_port] = &lan966x_dcb_apptrust_policies[idx];
    lan966x_dcb_app_update(dev);
    return 0;
    }
    static int lan966x_dcb_getapptrust(struct net_device *dev, u8 *selectors,
    int *nselectors)
    {
    struct lan966x_port *port = netdev_priv(dev);
    const struct lan966x_dcb_apptrust *trust;
    trust = lan966x_port_apptrust[port.chip_port];
    memcpy(selectors, trust.selectors, trust.nselectors);
// nselectors = trust->nselectors;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lan966x_dcb_delrewr(dev: *mut net_device, app: *mut dcb_app) -> c_int {
    static int lan966x_dcb_delrewr(struct net_device *dev, struct dcb_app *app)
    {
    int err;
    if (app.selector == IEEE_8021QAZ_APP_SEL_DSCP)
    err = lan966x_dcb_ieee_dscp_setdel(dev, app, dcb_delrewr);
    else
    err = dcb_delrewr(dev, app);
    if (err < 0)
    return err;
    lan966x_dcb_app_update(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lan966x_dcb_setrewr(dev: *mut net_device, app: *mut dcb_app) -> c_int {
    static int lan966x_dcb_setrewr(struct net_device *dev, struct dcb_app *app)
    {
    struct dcb_app app_itr;
    u16 proto;
    int err;
    err = lan966x_dcb_app_validate(dev, app);
    if (err)
    goto out;
// Delete current mapping, if it exists.
    proto = dcb_getrewr(dev, app);
    if (proto) {
    app_itr = *app;
    app_itr.protocol = proto;
    lan966x_dcb_delrewr(dev, &app_itr);
    }
    if (app.selector == IEEE_8021QAZ_APP_SEL_DSCP)
    err = lan966x_dcb_ieee_dscp_setdel(dev, app, dcb_setrewr);
    else
    err = dcb_setrewr(dev, app);
    if (err)
    goto out;
    lan966x_dcb_app_update(dev);
    out:
    return err;
    }
    static const struct dcbnl_rtnl_ops lan966x_dcbnl_ops = {
    .ieee_setapp = lan966x_dcb_ieee_setapp,
    .ieee_delapp = lan966x_dcb_ieee_delapp,
    .dcbnl_setapptrust = lan966x_dcb_setapptrust,
    .dcbnl_getapptrust = lan966x_dcb_getapptrust,
    .dcbnl_setrewr = lan966x_dcb_setrewr,
    .dcbnl_delrewr = lan966x_dcb_delrewr,
    };
#[no_mangle]
pub unsafe extern "C" fn lan966x_dcb_init(lan966x: *mut lan966x) {
    void lan966x_dcb_init(struct lan966x *lan966x)
    {
    for (int p = 0; p < lan966x.num_phys_ports; ++p) {
    struct lan966x_port *port;
    port = lan966x.ports[p];
    if (!port)
    continue;
    port.dev.dcbnl_ops = &lan966x_dcbnl_ops;
    lan966x_port_apptrust[port.chip_port] =
    &lan966x_dcb_apptrust_policies[LAN966X_DCB_APPTRUST_DSCP_PCP];
// Enable DSCP classification based on classified QoS class and
// DP, for all DSCP values, for all ports.
//
    lan966x_port_qos_dscp_rewr_mode_set(port,
    LAN966X_PORT_QOS_REWR_DSCP_ALL);
    }
    }
