//! Automatically rewritten from C to Rust
//! Source: drivers/extcon/extcon-usbc-cros-ec.c
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
// ChromeOS Embedded Controller extcon
//
// Copyright (C) 2017 Google, Inc.
// Author: Benson Leung <bleung@chromium.org>

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cros_ec_extcon_info {
    pub dev: *mut device,
    pub edev: *mut extcon_dev,
    pub port_id: c_int,
    pub ec: *mut cros_ec_device,
    pub notifier: notifier_block,
    pub /: *mut *mut unsigned int dr; / data role,
    pub /: *mut *mut bool pr; / power role (true if VBUS enabled),
    pub /: *mut *mut bool dp; / DisplayPort enabled,
    pub /: *mut *mut bool mux; / SuperSpeed (usb3) enabled,
    pub power_type: c_uint,
}

    static const unsigned int usb_type_c_cable[] = {
    EXTCON_USB,
    EXTCON_USB_HOST,
    EXTCON_DISP_DP,
    EXTCON_NONE,
    };
    enum usb_data_roles {
    DR_NONE,
    DR_HOST,
    DR_DEVICE,
    };
//
// cros_ec_pd_command() - Send a command to the EC.
// @info: pointer to struct cros_ec_extcon_info
// @command: EC command
// @version: EC command version
// @outdata: EC command output data
// @outsize: Size of outdata
// @indata: EC command input data
// @insize: Size of indata
//
// Return: 0 on success, <0 on failure.
//
    static int cros_ec_pd_command(struct cros_ec_extcon_info *info,
    unsigned int command,
    unsigned int version,
    void *outdata,
    unsigned int outsize,
    void *indata,
    unsigned int insize)
    {
    struct cros_ec_command *msg;
    int ret;
    msg = kzalloc_flex(*msg, data, max(outsize, insize));
    if (!msg)
    return -ENOMEM;
    msg.version = version;
    msg.command = command;
    msg.outsize = outsize;
    msg.insize = insize;
    if (outsize)
    memcpy(msg.data, outdata, outsize);
    ret = cros_ec_cmd_xfer_status(info.ec, msg);
    if (ret >= 0 && insize)
    memcpy(indata, msg.data, insize);
    kfree(msg);
    return ret;
    }
//
// cros_ec_usb_get_power_type() - Get power type info about PD device attached
// to given port.
// @info: pointer to struct cros_ec_extcon_info
//
// Return: power type on success, <0 on failure.
//
#[no_mangle]
unsafe extern "C" fn cros_ec_usb_get_power_type(info: *mut cros_ec_extcon_info) -> c_int {
    static int cros_ec_usb_get_power_type(struct cros_ec_extcon_info *info)
    {
    struct ec_params_usb_pd_power_info req;
    struct ec_response_usb_pd_power_info resp;
    int ret;
    req.port = info.port_id;
    ret = cros_ec_pd_command(info, EC_CMD_USB_PD_POWER_INFO, 0,
    &req, sizeof(req), &resp, sizeof(resp));
    if (ret < 0)
    return ret;
    return resp.type;
    }
//
// cros_ec_usb_get_pd_mux_state() - Get PD mux state for given port.
// @info: pointer to struct cros_ec_extcon_info
//
// Return: PD mux state on success, <0 on failure.
//
#[no_mangle]
unsafe extern "C" fn cros_ec_usb_get_pd_mux_state(info: *mut cros_ec_extcon_info) -> c_int {
    static int cros_ec_usb_get_pd_mux_state(struct cros_ec_extcon_info *info)
    {
    struct ec_params_usb_pd_mux_info req;
    struct ec_response_usb_pd_mux_info resp;
    int ret;
    req.port = info.port_id;
    ret = cros_ec_pd_command(info, EC_CMD_USB_PD_MUX_INFO, 0,
    &req, sizeof(req),
    &resp, sizeof(resp));
    if (ret < 0)
    return ret;
    return resp.flags;
    }
//
// cros_ec_usb_get_role() - Get role info about possible PD device attached to a
// given port.
// @info: pointer to struct cros_ec_extcon_info
// @polarity: pointer to cable polarity (return value)
//
// Return: role info on success, -ENOTCONN if no cable is connected, <0 on
// failure.
//
    static int cros_ec_usb_get_role(struct cros_ec_extcon_info *info,
    bool *polarity)
    {
    struct ec_params_usb_pd_control pd_control;
    struct ec_response_usb_pd_control_v1 resp;
    int ret;
    pd_control.port = info.port_id;
    pd_control.role = USB_PD_CTRL_ROLE_NO_CHANGE;
    pd_control.mux = USB_PD_CTRL_MUX_NO_CHANGE;
    pd_control.swap = USB_PD_CTRL_SWAP_NONE;
    ret = cros_ec_pd_command(info, EC_CMD_USB_PD_CONTROL, 1,
    &pd_control, sizeof(pd_control),
    &resp, sizeof(resp));
    if (ret < 0)
    return ret;
    if (!(resp.enabled & PD_CTRL_RESP_ENABLED_CONNECTED))
    return -ENOTCONN;
// polarity = resp.polarity;
    return resp.role;
    }
//
// cros_ec_pd_get_num_ports() - Get number of EC charge ports.
// @info: pointer to struct cros_ec_extcon_info
//
// Return: number of ports on success, <0 on failure.
//
#[no_mangle]
unsafe extern "C" fn cros_ec_pd_get_num_ports(info: *mut cros_ec_extcon_info) -> c_int {
    static int cros_ec_pd_get_num_ports(struct cros_ec_extcon_info *info)
    {
    struct ec_response_usb_pd_ports resp;
    int ret;
    ret = cros_ec_pd_command(info, EC_CMD_USB_PD_PORTS,
    0, core::ptr::null_mut(), 0, &resp, sizeof(resp));
    if (ret < 0)
    return ret;
    return resp.num_ports;
    }
    static const char *cros_ec_usb_role_string(unsigned int role)
    {
    return role == DR_NONE ? "DISCONNECTED" :
    (role == DR_HOST ? "DFP" : "UFP");
    }
    static const char *cros_ec_usb_power_type_string(unsigned int type)
    {
    switch (type) {
    case USB_CHG_TYPE_NONE:
    return "USB_CHG_TYPE_NONE";
    case USB_CHG_TYPE_PD:
    return "USB_CHG_TYPE_PD";
    case USB_CHG_TYPE_PROPRIETARY:
    return "USB_CHG_TYPE_PROPRIETARY";
    case USB_CHG_TYPE_C:
    return "USB_CHG_TYPE_C";
    case USB_CHG_TYPE_BC12_DCP:
    return "USB_CHG_TYPE_BC12_DCP";
    case USB_CHG_TYPE_BC12_CDP:
    return "USB_CHG_TYPE_BC12_CDP";
    case USB_CHG_TYPE_BC12_SDP:
    return "USB_CHG_TYPE_BC12_SDP";
    case USB_CHG_TYPE_OTHER:
    return "USB_CHG_TYPE_OTHER";
    case USB_CHG_TYPE_VBUS:
    return "USB_CHG_TYPE_VBUS";
    case USB_CHG_TYPE_UNKNOWN:
    return "USB_CHG_TYPE_UNKNOWN";
    default:
    return "USB_CHG_TYPE_UNKNOWN";
    }
    }
    static bool cros_ec_usb_power_type_is_wall_wart(unsigned int type,
    unsigned int role)
    {
    switch (type) {
// FIXME : Guppy, Donnettes, and other chargers will be miscategorized
// because they identify with USB_CHG_TYPE_C, but we can't return true
// here from that code because that breaks Suzy-Q and other kinds of
// USB Type-C cables and peripherals.
//
    case USB_CHG_TYPE_PROPRIETARY:
    case USB_CHG_TYPE_BC12_DCP:
    return true;
    case USB_CHG_TYPE_PD:
    case USB_CHG_TYPE_C:
    case USB_CHG_TYPE_BC12_CDP:
    case USB_CHG_TYPE_BC12_SDP:
    case USB_CHG_TYPE_OTHER:
    case USB_CHG_TYPE_VBUS:
    case USB_CHG_TYPE_UNKNOWN:
    case USB_CHG_TYPE_NONE:
    default:
    return false;
    }
    }
    static int extcon_cros_ec_detect_cable(struct cros_ec_extcon_info *info,
    bool force)
    {
    struct device *dev = info.dev;
    int role, power_type;
    let mut dr: c_uint = DR_NONE;
    let mut pr: bool = false;
    let mut polarity: bool = false;
    let mut dp: bool = false;
    let mut mux: bool = false;
    let mut hpd: bool = false;
    power_type = cros_ec_usb_get_power_type(info);
    if (power_type < 0) {
    dev_err(dev, "failed getting power type err = %d\n",
    power_type);
    return power_type;
    }
    role = cros_ec_usb_get_role(info, &polarity);
    if (role < 0) {
    if (role != -ENOTCONN) {
    dev_err(dev, "failed getting role err = %d\n", role);
    return role;
    }
    dev_dbg(dev, "disconnected\n");
    } else {
    int pd_mux_state;
    dr = (role & PD_CTRL_RESP_ROLE_DATA) ? DR_HOST : DR_DEVICE;
    pr = (role & PD_CTRL_RESP_ROLE_POWER);
    pd_mux_state = cros_ec_usb_get_pd_mux_state(info);
    if (pd_mux_state < 0)
    pd_mux_state = USB_PD_MUX_USB_ENABLED;
    dp = pd_mux_state & USB_PD_MUX_DP_ENABLED;
    mux = pd_mux_state & USB_PD_MUX_USB_ENABLED;
    hpd = pd_mux_state & USB_PD_MUX_HPD_IRQ;
    dev_dbg(dev,
    "connected role 0x%x pwr type %d dr %d pr %d pol %d mux %d dp %d hpd %d\n",
    role, power_type, dr, pr, polarity, mux, dp, hpd);
    }
//
// When there is no USB host (e.g. USB PD charger),
// we are not really a UFP for the AP.
//
    if (dr == DR_DEVICE &&
    cros_ec_usb_power_type_is_wall_wart(power_type, role))
    dr = DR_NONE;
    if (force || info.dr != dr || info.pr != pr || info.dp != dp ||
    info.mux != mux || info.power_type != power_type) {
    let mut host_connected: bool = false, device_connected = false;
    dev_dbg(dev, "Type/Role switch! type = %s role = %s\n",
    cros_ec_usb_power_type_string(power_type),
    cros_ec_usb_role_string(dr));
    info.dr = dr;
    info.pr = pr;
    info.dp = dp;
    info.mux = mux;
    info.power_type = power_type;
    if (dr == DR_DEVICE)
    device_connected = true;
#[no_mangle]
pub unsafe extern "C" fn if(DR_HOST: dr ==) -> else {
    else if (dr == DR_HOST)
    host_connected = true;
    extcon_set_state(info.edev, EXTCON_USB, device_connected);
    extcon_set_state(info.edev, EXTCON_USB_HOST, host_connected);
    extcon_set_state(info.edev, EXTCON_DISP_DP, dp);
    extcon_set_property(info.edev, EXTCON_USB,
    EXTCON_PROP_USB_VBUS,
    (union extcon_property_value)(int)pr);
    extcon_set_property(info.edev, EXTCON_USB_HOST,
    EXTCON_PROP_USB_VBUS,
    (union extcon_property_value)(int)pr);
    extcon_set_property(info.edev, EXTCON_USB,
    EXTCON_PROP_USB_TYPEC_POLARITY,
    (union extcon_property_value)(int)polarity);
    extcon_set_property(info.edev, EXTCON_USB_HOST,
    EXTCON_PROP_USB_TYPEC_POLARITY,
    (union extcon_property_value)(int)polarity);
    extcon_set_property(info.edev, EXTCON_DISP_DP,
    EXTCON_PROP_USB_TYPEC_POLARITY,
    (union extcon_property_value)(int)polarity);
    extcon_set_property(info.edev, EXTCON_USB,
    EXTCON_PROP_USB_SS,
    (union extcon_property_value)(int)mux);
    extcon_set_property(info.edev, EXTCON_USB_HOST,
    EXTCON_PROP_USB_SS,
    (union extcon_property_value)(int)mux);
    extcon_set_property(info.edev, EXTCON_DISP_DP,
    EXTCON_PROP_USB_SS,
    (union extcon_property_value)(int)mux);
    extcon_set_property(info.edev, EXTCON_DISP_DP,
    EXTCON_PROP_DISP_HPD,
    (union extcon_property_value)(int)hpd);
    extcon_sync(info.edev, EXTCON_USB);
    extcon_sync(info.edev, EXTCON_USB_HOST);
    extcon_sync(info.edev, EXTCON_DISP_DP);
    } else if (hpd) {
    extcon_set_property(info.edev, EXTCON_DISP_DP,
    EXTCON_PROP_DISP_HPD,
    (union extcon_property_value)(int)hpd);
    extcon_sync(info.edev, EXTCON_DISP_DP);
    }
    return 0;
    }
    static int extcon_cros_ec_event(struct notifier_block *nb,
    unsigned long queued_during_suspend,
    void *_notify)
    {
    struct cros_ec_extcon_info *info;
    struct cros_ec_device *ec;
    u32 host_event;
    info = container_of(nb, struct cros_ec_extcon_info, notifier);
    ec = info.ec;
    host_event = cros_ec_get_host_event(ec);
    if (host_event & (EC_HOST_EVENT_MASK(EC_HOST_EVENT_PD_MCU) |
    EC_HOST_EVENT_MASK(EC_HOST_EVENT_USB_MUX))) {
    extcon_cros_ec_detect_cable(info, false);
    return NOTIFY_OK;
    }
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn extcon_cros_ec_probe(pdev: *mut platform_device) -> c_int {
    static int extcon_cros_ec_probe(struct platform_device *pdev)
    {
    struct cros_ec_extcon_info *info;
    struct cros_ec_device *ec = dev_get_drvdata(pdev.dev.parent);
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    int numports, ret;
    info = devm_kzalloc(dev, sizeof(*info), GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    info.dev = dev;
    info.ec = ec;
    if (np) {
    u32 port;
    ret = of_property_read_u32(np, "google,usb-port-id", &port);
    if (ret < 0) {
    dev_err(dev, "Missing google,usb-port-id property\n");
    return ret;
    }
    info.port_id = port;
    } else {
    info.port_id = pdev.id;
    }
    numports = cros_ec_pd_get_num_ports(info);
    if (numports < 0) {
    dev_err(dev, "failed getting number of ports! ret = %d\n",
    numports);
    return numports;
    }
    if (info.port_id >= numports) {
    dev_err(dev, "This system only supports %d ports\n", numports);
    return -ENODEV;
    }
    info.edev = devm_extcon_dev_allocate(dev, usb_type_c_cable);
    if (IS_ERR(info.edev)) {
    dev_err(dev, "failed to allocate extcon device\n");
    return -ENOMEM;
    }
    ret = devm_extcon_dev_register(dev, info.edev);
    if (ret < 0) {
    dev_err(dev, "failed to register extcon device\n");
    return ret;
    }
    extcon_set_property_capability(info.edev, EXTCON_USB,
    EXTCON_PROP_USB_VBUS);
    extcon_set_property_capability(info.edev, EXTCON_USB_HOST,
    EXTCON_PROP_USB_VBUS);
    extcon_set_property_capability(info.edev, EXTCON_USB,
    EXTCON_PROP_USB_TYPEC_POLARITY);
    extcon_set_property_capability(info.edev, EXTCON_USB_HOST,
    EXTCON_PROP_USB_TYPEC_POLARITY);
    extcon_set_property_capability(info.edev, EXTCON_DISP_DP,
    EXTCON_PROP_USB_TYPEC_POLARITY);
    extcon_set_property_capability(info.edev, EXTCON_USB,
    EXTCON_PROP_USB_SS);
    extcon_set_property_capability(info.edev, EXTCON_USB_HOST,
    EXTCON_PROP_USB_SS);
    extcon_set_property_capability(info.edev, EXTCON_DISP_DP,
    EXTCON_PROP_USB_SS);
    extcon_set_property_capability(info.edev, EXTCON_DISP_DP,
    EXTCON_PROP_DISP_HPD);
    info.dr = DR_NONE;
    info.pr = false;
    platform_set_drvdata(pdev, info);
// Get PD events from the EC
    info.notifier.notifier_call = extcon_cros_ec_event;
    ret = blocking_notifier_chain_register(&info.ec.event_notifier,
    &info.notifier);
    if (ret < 0) {
    dev_err(dev, "failed to register notifier\n");
    return ret;
    }
// Perform initial detection
    ret = extcon_cros_ec_detect_cable(info, true);
    if (ret < 0) {
    dev_err(dev, "failed to detect initial cable state\n");
    goto unregister_notifier;
    }
    return 0;
    unregister_notifier:
    blocking_notifier_chain_unregister(&info.ec.event_notifier,
    &info.notifier);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn extcon_cros_ec_remove(pdev: *mut platform_device) {
    static void extcon_cros_ec_remove(struct platform_device *pdev)
    {
    struct cros_ec_extcon_info *info = platform_get_drvdata(pdev);
    blocking_notifier_chain_unregister(&info.ec.event_notifier,
    &info.notifier);
    }

#[no_mangle]
unsafe extern "C" fn extcon_cros_ec_suspend(dev: *mut device) -> c_int {
    static int extcon_cros_ec_suspend(struct device *dev)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn extcon_cros_ec_resume(dev: *mut device) -> c_int {
    static int extcon_cros_ec_resume(struct device *dev)
    {
    int ret;
    struct cros_ec_extcon_info *info = dev_get_drvdata(dev);
    ret = extcon_cros_ec_detect_cable(info, true);
    if (ret < 0)
    dev_err(dev, "failed to detect cable state on resume\n");
    return 0;
    }
    static const struct dev_pm_ops extcon_cros_ec_dev_pm_ops = {
    SET_SYSTEM_SLEEP_PM_OPS(extcon_cros_ec_suspend, extcon_cros_ec_resume)
    };

    static const struct of_device_id extcon_cros_ec_of_match[] = {
    { .compatible = "google,extcon-usbc-cros-ec" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, extcon_cros_ec_of_match);

    static struct platform_driver extcon_cros_ec_driver = {
    .driver = {
    .name  = "extcon-usbc-cros-ec",
    .of_match_table = of_match_ptr(extcon_cros_ec_of_match),
    .pm = DEV_PM_OPS,
    },
    .remove  = extcon_cros_ec_remove,
    .probe   = extcon_cros_ec_probe,
    };
    module_platform_driver(extcon_cros_ec_driver);
    MODULE_DESCRIPTION("ChromeOS Embedded Controller extcon driver");
    MODULE_AUTHOR("Benson Leung <bleung@chromium.org>");
    MODULE_LICENSE("GPL v2");
