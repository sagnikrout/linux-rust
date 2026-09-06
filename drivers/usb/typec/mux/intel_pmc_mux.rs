//! Automatically rewritten from C to Rust
//! Source: drivers/usb/typec/mux/intel_pmc_mux.c
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
// Driver for Intel PMC USB mux control
//
// Copyright (C) 2020 Intel Corporation
// Author: Heikki Krogerus <heikki.krogerus@linux.intel.com>
//

pub const PMC_USBC_CMD: c_uint = 0xa7;
// Response status bits

// "Usage" OOB Message field values
    enum {
    PMC_USB_CONNECT,
    PMC_USB_DISCONNECT,
    PMC_USB_SAFE_MODE,
    PMC_USB_ALT_MODE,
    PMC_USB_DP_HPD,
    };
pub const PMC_USB_MSG_USB2_PORT_SHIFT: c_int = 0;
pub const PMC_USB_MSG_USB3_PORT_SHIFT: c_int = 4;
pub const PMC_USB_MSG_UFP_SHIFT: c_int = 4;
pub const PMC_USB_MSG_ORI_HSL_SHIFT: c_int = 5;
pub const PMC_USB_MSG_ORI_AUX_SHIFT: c_int = 6;
// Alt Mode Request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct altmode_req {
    pub usage: u8,
    pub mode_type: u8,
    pub mode_id: u8,
    pub reserved: u8,
    pub mode_data: u32,
    pub __packed: },
pub const PMC_USB_MODE_TYPE_SHIFT: c_int = 4;
    enum {
    PMC_USB_MODE_TYPE_USB,
    PMC_USB_MODE_TYPE_DP,
    PMC_USB_MODE_TYPE_TBT,
}

// Common Mode Data bits

pub const PMC_USB_ALTMODE_ORI_SHIFT: c_int = 1;
pub const PMC_USB_ALTMODE_UFP_SHIFT: c_int = 3;
// DP specific Mode Data bits
pub const PMC_USB_ALTMODE_DP_MODE_SHIFT: c_int = 8;
// TBT specific Mode Data bits

pub const PMC_USB_ALTMODE_CABLE_USB31: c_int = 1;
pub const PMC_USB_ALTMODE_CABLE_10GPS: c_int = 2;
pub const PMC_USB_ALTMODE_CABLE_20GPS: c_int = 3;

// Display HPD Request bits

//
// Input Output Manager (IOM) PORT STATUS
//

pub const IOM_PORT_STATUS_ACTIVITY_TYPE_SHIFT: c_int = 6;
pub const IOM_PORT_STATUS_ACTIVITY_TYPE_USB: c_uint = 0x03;
// activity type: Safe Mode
pub const IOM_PORT_STATUS_ACTIVITY_TYPE_SAFE_MODE: c_uint = 0x04;
// activity type: Display Port
pub const IOM_PORT_STATUS_ACTIVITY_TYPE_DP: c_uint = 0x05;
// activity type: Display Port Multi Function Device
pub const IOM_PORT_STATUS_ACTIVITY_TYPE_DP_MFD: c_uint = 0x06;
// activity type: Thunderbolt
pub const IOM_PORT_STATUS_ACTIVITY_TYPE_TBT: c_uint = 0x07;
pub const IOM_PORT_STATUS_ACTIVITY_TYPE_ALT_MODE_USB: c_uint = 0x0c;
pub const IOM_PORT_STATUS_ACTIVITY_TYPE_ALT_MODE_TBT_USB: c_uint = 0x0d;
// Upstream Facing Port Information

// Display Port Hot Plug Detect status

pub const IOM_PORT_STATUS_DHPD_HPD_STATUS_SHIFT: c_int = 12;
pub const IOM_PORT_STATUS_DHPD_HPD_STATUS_ASSERT: c_uint = 0x01;

    ((((_status_) & IOM_PORT_STATUS_ACTIVITY_TYPE_MASK) >>		\
    IOM_PORT_STATUS_ACTIVITY_TYPE_SHIFT) ==			\
    (IOM_PORT_STATUS_ACTIVITY_TYPE_##_type_))

    ((((_status_) & IOM_PORT_STATUS_DHPD_HPD_STATUS_MASK) >>	\
    IOM_PORT_STATUS_DHPD_HPD_STATUS_SHIFT) &			\
    IOM_PORT_STATUS_DHPD_HPD_STATUS_ASSERT)
// IOM port status register

pub const IOM_PORT_STATUS_REGS_SZ_4: c_int = 0;
pub const IOM_PORT_STATUS_REGS_SZ_8: c_int = 1;

    ((_d_) & ~IOM_PORT_STATUS_REGS_SZ_MASK)

    (4 << ((_d_) & IOM_PORT_STATUS_REGS_SZ_MASK))
    struct pmc_usb;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmc_usb_port {
    pub num: c_int,
    pub iom_status: u32,
    pub pmc: *mut pmc_usb,
    pub typec_mux: *mut typec_mux_dev,
    pub typec_sw: *mut typec_switch_dev,
    pub usb_sw: *mut usb_role_switch,
    pub orientation: enum typec_orientation,
    pub role: enum usb_role,
    pub usb2_port: u8,
    pub usb3_port: u8,
    pub sbu_orientation: enum typec_orientation,
    pub hsl_orientation: enum typec_orientation,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmc_usb {
    pub num_ports: u8,
    pub dev: *mut device,
    pub ipc: *mut intel_scu_ipc_dev,
    pub iom_adev: *mut acpi_device,
    pub iom_base: *mut void __iomem,
    pub iom_port_status_offset: u32,
    pub iom_port_status_size: u8,
    pub dentry: *mut dentry,
    pub __counted_by(num_ports): pmc_usb_port port[],
}

    static struct dentry *pmc_mux_debugfs_root;
#[no_mangle]
unsafe extern "C" fn update_port_status(port: *mut pmc_usb_port) {
    static void update_port_status(struct pmc_usb_port *port)
    {
    u8 port_num;
// SoC expects the USB Type-C port numbers to start with 0
    port_num = port.usb3_port - 1;
    port.iom_status = readl(port.pmc.iom_base +
    port.pmc.iom_port_status_offset +
    port_num * port.pmc.iom_port_status_size);
    }
#[no_mangle]
unsafe extern "C" fn sbu_orientation(port: *mut pmc_usb_port) -> c_int {
    static int sbu_orientation(struct pmc_usb_port *port)
    {
    if (port.sbu_orientation)
    return port.sbu_orientation - 1;
    return port.orientation - 1;
    }
#[no_mangle]
unsafe extern "C" fn hsl_orientation(port: *mut pmc_usb_port) -> c_int {
    static int hsl_orientation(struct pmc_usb_port *port)
    {
    if (port.hsl_orientation)
    return port.hsl_orientation - 1;
    return port.orientation - 1;
    }
#[no_mangle]
unsafe extern "C" fn is_pmc_mux_tbt(adev: *mut acpi_device) -> bool {
    static bool is_pmc_mux_tbt(struct acpi_device *adev)
    {
    return acpi_dev_hid_uid_match(adev, "INTC1072", core::ptr::null_mut()) ||
    acpi_dev_hid_uid_match(adev, "INTC1079", core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn pmc_usb_send_command(ipc: *mut intel_scu_ipc_dev, msg: *mut u8, len: u32) -> c_int {
    static int pmc_usb_send_command(struct intel_scu_ipc_dev *ipc, u8 *msg, u32 len)
    {
    u8 response[4];
    u8 status_res;
    int ret;
//
// Error bit will always be 0 with the USBC command.
// Status can be checked from the response message if the
// function intel_scu_ipc_dev_command succeeds.
//
    ret = intel_scu_ipc_dev_command(ipc, PMC_USBC_CMD, 0, msg,
    len, response, sizeof(response));
    if (ret)
    return ret;
    status_res = (msg[0] & 0xf) < PMC_USB_SAFE_MODE ?
    response[2] : response[1];
    if (status_res & PMC_USB_RESP_STATUS_FAILURE) {
    if (status_res & PMC_USB_RESP_STATUS_FATAL)
    return -EIO;
    return -EBUSY;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pmc_usb_command(port: *mut pmc_usb_port, msg: *mut u8, len: u32) -> c_int {
    static int pmc_usb_command(struct pmc_usb_port *port, u8 *msg, u32 len)
    {
    let mut retry_count: c_int = 3;
    int ret;
//
// If PMC is busy then retry the command once again
//
    while (retry_count--) {
    ret = pmc_usb_send_command(port.pmc.ipc, msg, len);
    if (ret != -EBUSY)
    break;
    }
    return ret;
    }
    static int
    pmc_usb_mux_dp_hpd(struct pmc_usb_port *port, struct typec_displayport_data *dp)
    {
    u8 msg[2] = { };
    int ret;
    msg[0] = PMC_USB_DP_HPD;
    msg[0] |= port.usb3_port << PMC_USB_MSG_USB3_PORT_SHIFT;
// Configure HPD first if HPD,IRQ comes together
    if (!IOM_PORT_HPD_ASSERTED(port.iom_status) &&
    dp.status & DP_STATUS_IRQ_HPD &&
    dp.status & DP_STATUS_HPD_STATE) {
    msg[1] = PMC_USB_DP_HPD_LVL;
    ret = pmc_usb_command(port, msg, sizeof(msg));
    if (ret)
    return ret;
    }
    if (dp.status & DP_STATUS_IRQ_HPD)
    msg[1] = PMC_USB_DP_HPD_IRQ;
    if (dp.status & DP_STATUS_HPD_STATE)
    msg[1] |= PMC_USB_DP_HPD_LVL;
    return pmc_usb_command(port, msg, sizeof(msg));
    }
    static int
    pmc_usb_mux_dp(struct pmc_usb_port *port, struct typec_mux_state *state)
    {
    struct typec_displayport_data *data = state.data;
    let mut req: altmode_req = { };
    int ret;
    if (IOM_PORT_ACTIVITY_IS(port.iom_status, DP) ||
    IOM_PORT_ACTIVITY_IS(port.iom_status, DP_MFD)) {
    if (IOM_PORT_HPD_ASSERTED(port.iom_status) &&
    (!(data.status & DP_STATUS_IRQ_HPD) &&
    data.status & DP_STATUS_HPD_STATE))
    return 0;
    return pmc_usb_mux_dp_hpd(port, state.data);
    }
    req.usage = PMC_USB_ALT_MODE;
    req.usage |= port.usb3_port << PMC_USB_MSG_USB3_PORT_SHIFT;
    req.mode_type = PMC_USB_MODE_TYPE_DP << PMC_USB_MODE_TYPE_SHIFT;
    req.mode_data = (port.orientation - 1) << PMC_USB_ALTMODE_ORI_SHIFT;
    req.mode_data |= (port.role - 1) << PMC_USB_ALTMODE_UFP_SHIFT;
    req.mode_data |= (state.mode - TYPEC_STATE_MODAL) <<
    PMC_USB_ALTMODE_DP_MODE_SHIFT;
    if (!is_pmc_mux_tbt(port.pmc.iom_adev)) {
    u8 cable_speed = (data.conf & DP_CONF_SIGNALLING_MASK) >>
    DP_CONF_SIGNALLING_SHIFT;
    u8 cable_type = (data.conf & DP_CONF_CABLE_TYPE_MASK) >>
    DP_CONF_CABLE_TYPE_SHIFT;
    req.mode_data |= PMC_USB_ALTMODE_CABLE_SPD(cable_speed);
    if (cable_type == DP_CONF_CABLE_TYPE_OPTICAL)
    req.mode_data |= PMC_USB_ALTMODE_CABLE_TYPE;
#[no_mangle]
pub unsafe extern "C" fn if(DP_CONF_CABLE_TYPE_RE_TIMER: cable_type ==) -> else {
    else if (cable_type == DP_CONF_CABLE_TYPE_RE_TIMER)
    req.mode_data |= PMC_USB_ALTMODE_ACTIVE_CABLE |
    PMC_USB_ALTMODE_RETIMER_CABLE;
#[no_mangle]
pub unsafe extern "C" fn if(DP_CONF_CABLE_TYPE_RE_DRIVER: cable_type ==) -> else {
    else if (cable_type == DP_CONF_CABLE_TYPE_RE_DRIVER)
    req.mode_data |= PMC_USB_ALTMODE_ACTIVE_CABLE;
    }
    ret = pmc_usb_command(port, (void *)&req, sizeof(req));
    if (ret)
    return ret;
    if (data.status & (DP_STATUS_IRQ_HPD | DP_STATUS_HPD_STATE))
    return pmc_usb_mux_dp_hpd(port, state.data);
    return 0;
    }
    static int
    pmc_usb_mux_tbt(struct pmc_usb_port *port, struct typec_mux_state *state)
    {
    struct typec_thunderbolt_data *data = state.data;
    let mut cable_rounded: u8 = TBT_CABLE_ROUNDED_SUPPORT(data.cable_mode);
    let mut cable_speed: u8 = TBT_CABLE_SPEED(data.cable_mode);
    let mut req: altmode_req = { };
    if (IOM_PORT_ACTIVITY_IS(port.iom_status, TBT) ||
    IOM_PORT_ACTIVITY_IS(port.iom_status, ALT_MODE_TBT_USB))
    return 0;
    req.usage = PMC_USB_ALT_MODE;
    req.usage |= port.usb3_port << PMC_USB_MSG_USB3_PORT_SHIFT;
    req.mode_type = PMC_USB_MODE_TYPE_TBT << PMC_USB_MODE_TYPE_SHIFT;
    req.mode_data = (port.orientation - 1) << PMC_USB_ALTMODE_ORI_SHIFT;
    req.mode_data |= (port.role - 1) << PMC_USB_ALTMODE_UFP_SHIFT;
    if (TBT_ADAPTER(data.device_mode) == TBT_ADAPTER_TBT3)
    req.mode_data |= PMC_USB_ALTMODE_TBT_TYPE;
    if (data.cable_mode & TBT_CABLE_OPTICAL)
    req.mode_data |= PMC_USB_ALTMODE_CABLE_TYPE;
    if (data.cable_mode & TBT_CABLE_LINK_TRAINING)
    req.mode_data |= PMC_USB_ALTMODE_ACTIVE_LINK;
    if (acpi_dev_hid_uid_match(port.pmc.iom_adev, "INTC1072", core::ptr::null_mut()) ||
    acpi_dev_hid_uid_match(port.pmc.iom_adev, "INTC1079", core::ptr::null_mut())) {
    if ((data.enter_vdo & TBT_ENTER_MODE_ACTIVE_CABLE) ||
    (data.cable_mode & TBT_CABLE_RETIMER))
    req.mode_data |= PMC_USB_ALTMODE_RETIMER_CABLE;
    } else {
    if (data.enter_vdo & TBT_ENTER_MODE_ACTIVE_CABLE)
    req.mode_data |= PMC_USB_ALTMODE_ACTIVE_CABLE;
    if (data.cable_mode & TBT_CABLE_RETIMER)
    req.mode_data |= PMC_USB_ALTMODE_RETIMER_CABLE;
    }
    req.mode_data |= PMC_USB_ALTMODE_CABLE_SPD(cable_speed);
    req.mode_data |= PMC_USB_ALTMODE_TBT_GEN(cable_rounded);
    return pmc_usb_command(port, (void *)&req, sizeof(req));
    }
    static int
    pmc_usb_mux_usb4(struct pmc_usb_port *port, struct typec_mux_state *state)
    {
    struct enter_usb_data *data = state.data;
    let mut req: altmode_req = { };
    u8 cable_speed;
    if (IOM_PORT_ACTIVITY_IS(port.iom_status, TBT) ||
    IOM_PORT_ACTIVITY_IS(port.iom_status, ALT_MODE_TBT_USB))
    return 0;
    req.usage = PMC_USB_ALT_MODE;
    req.usage |= port.usb3_port << PMC_USB_MSG_USB3_PORT_SHIFT;
    req.mode_type = PMC_USB_MODE_TYPE_TBT << PMC_USB_MODE_TYPE_SHIFT;
// USB4 Mode
    req.mode_data = PMC_USB_ALTMODE_FORCE_LSR;
    if (data.active_link_training)
    req.mode_data |= PMC_USB_ALTMODE_ACTIVE_LINK;
    req.mode_data |= (port.orientation - 1) << PMC_USB_ALTMODE_ORI_SHIFT;
    req.mode_data |= (port.role - 1) << PMC_USB_ALTMODE_UFP_SHIFT;
    switch ((data.eudo & EUDO_CABLE_TYPE_MASK) >> EUDO_CABLE_TYPE_SHIFT) {
    case EUDO_CABLE_TYPE_PASSIVE:
    break;
    case EUDO_CABLE_TYPE_OPTICAL:
    req.mode_data |= PMC_USB_ALTMODE_CABLE_TYPE;
    fallthrough;
    case EUDO_CABLE_TYPE_RE_TIMER:
    if (!acpi_dev_hid_uid_match(port.pmc.iom_adev, "INTC1072", core::ptr::null_mut()) ||
    !acpi_dev_hid_uid_match(port.pmc.iom_adev, "INTC1079", core::ptr::null_mut()))
    req.mode_data |= PMC_USB_ALTMODE_RETIMER_CABLE;
    fallthrough;
    default:
    if (acpi_dev_hid_uid_match(port.pmc.iom_adev, "INTC1072", core::ptr::null_mut()) ||
    acpi_dev_hid_uid_match(port.pmc.iom_adev, "INTC1079", core::ptr::null_mut()))
    req.mode_data |= PMC_USB_ALTMODE_RETIMER_CABLE;
    else
    req.mode_data |= PMC_USB_ALTMODE_ACTIVE_CABLE;
// Configure data rate to rounded in the case of Active TBT3
// and USB4 cables.
//
    req.mode_data |= PMC_USB_ALTMODE_TBT_GEN(1);
    break;
    }
    cable_speed = (data.eudo & EUDO_CABLE_SPEED_MASK) >> EUDO_CABLE_SPEED_SHIFT;
    req.mode_data |= PMC_USB_ALTMODE_CABLE_SPD(cable_speed);
    return pmc_usb_command(port, (void *)&req, sizeof(req));
    }
    static int pmc_usb_mux_safe_state(struct pmc_usb_port *port,
    struct typec_mux_state *state)
    {
    u8 msg;
    if (IOM_PORT_ACTIVITY_IS(port.iom_status, SAFE_MODE))
    return 0;
    if ((IOM_PORT_ACTIVITY_IS(port.iom_status, DP) ||
    IOM_PORT_ACTIVITY_IS(port.iom_status, DP_MFD)) &&
    state.alt && state.alt.svid == USB_TYPEC_DP_SID)
    return 0;
    if ((IOM_PORT_ACTIVITY_IS(port.iom_status, TBT) ||
    IOM_PORT_ACTIVITY_IS(port.iom_status, ALT_MODE_TBT_USB)) &&
    state.alt && state.alt.svid == USB_TYPEC_TBT_SID)
    return 0;
    msg = PMC_USB_SAFE_MODE;
    msg |= port.usb3_port << PMC_USB_MSG_USB3_PORT_SHIFT;
    return pmc_usb_command(port, &msg, sizeof(msg));
    }
#[no_mangle]
unsafe extern "C" fn pmc_usb_disconnect(port: *mut pmc_usb_port) -> c_int {
    static int pmc_usb_disconnect(struct pmc_usb_port *port)
    {
    let mut data: typec_displayport_data = { };
    u8 msg[2];
    if (!(port.iom_status & IOM_PORT_STATUS_CONNECTED))
    return 0;
// Clear DisplayPort HPD if it's still asserted.
    if (IOM_PORT_HPD_ASSERTED(port.iom_status))
    pmc_usb_mux_dp_hpd(port, &data);
    msg[0] = PMC_USB_DISCONNECT;
    msg[0] |= port.usb3_port << PMC_USB_MSG_USB3_PORT_SHIFT;
    msg[1] = port.usb2_port << PMC_USB_MSG_USB2_PORT_SHIFT;
    return pmc_usb_command(port, msg, sizeof(msg));
    }
#[no_mangle]
unsafe extern "C" fn pmc_usb_connect(port: *mut pmc_usb_port, role: enum usb_role) -> c_int {
    static int pmc_usb_connect(struct pmc_usb_port *port, enum usb_role role)
    {
    let mut ufp: u8 = role == USB_ROLE_DEVICE ? 1 : 0;
    u8 msg[2];
    int ret;
    if (port.orientation == TYPEC_ORIENTATION_NONE)
    return -EINVAL;
    if (port.iom_status & IOM_PORT_STATUS_CONNECTED) {
    if (port.role == role || port.role == USB_ROLE_NONE)
    return 0;
// Role swap
    ret = pmc_usb_disconnect(port);
    if (ret)
    return ret;
    }
    msg[0] = PMC_USB_CONNECT;
    msg[0] |= port.usb3_port << PMC_USB_MSG_USB3_PORT_SHIFT;
    msg[1] = port.usb2_port << PMC_USB_MSG_USB2_PORT_SHIFT;
    msg[1] |= ufp << PMC_USB_MSG_UFP_SHIFT;
    msg[1] |= hsl_orientation(port) << PMC_USB_MSG_ORI_HSL_SHIFT;
    msg[1] |= sbu_orientation(port) << PMC_USB_MSG_ORI_AUX_SHIFT;
    return pmc_usb_command(port, msg, sizeof(msg));
    }
    static int
    pmc_usb_mux_set(struct typec_mux_dev *mux, struct typec_mux_state *state)
    {
    struct pmc_usb_port *port = typec_mux_get_drvdata(mux);
    update_port_status(port);
    if (port.orientation == TYPEC_ORIENTATION_NONE || port.role == USB_ROLE_NONE)
    return 0;
    if (state.mode == TYPEC_STATE_SAFE)
    return pmc_usb_mux_safe_state(port, state);
    if (state.mode == TYPEC_STATE_USB)
    return pmc_usb_connect(port, port.role);
    if (state.alt) {
    switch (state.alt.svid) {
    case USB_TYPEC_TBT_SID:
    return pmc_usb_mux_tbt(port, state);
    case USB_TYPEC_DP_SID:
    return pmc_usb_mux_dp(port, state);
    }
    } else {
    switch (state.mode) {
    case TYPEC_MODE_USB2:
// REVISIT: Try with usb3_port set to 0?
    break;
    case TYPEC_MODE_USB3:
    return pmc_usb_connect(port, port.role);
    case TYPEC_MODE_USB4:
    return pmc_usb_mux_usb4(port, state);
    }
    }
    return -EOPNOTSUPP;
    }
    static int pmc_usb_set_orientation(struct typec_switch_dev *sw,
    enum typec_orientation orientation)
    {
    struct pmc_usb_port *port = typec_switch_get_drvdata(sw);
    update_port_status(port);
    port.orientation = orientation;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pmc_usb_set_role(sw: *mut usb_role_switch, role: enum usb_role) -> c_int {
    static int pmc_usb_set_role(struct usb_role_switch *sw, enum usb_role role)
    {
    struct pmc_usb_port *port = usb_role_switch_get_drvdata(sw);
    int ret;
    update_port_status(port);
    if (role == USB_ROLE_NONE)
    ret = pmc_usb_disconnect(port);
    else
    ret = pmc_usb_connect(port, role);
    port.role = role;
    return ret;
    }
    static int pmc_usb_register_port(struct pmc_usb *pmc, int index,
    struct fwnode_handle *fwnode)
    {
    struct pmc_usb_port *port = &pmc.port[index];
    let mut desc: usb_role_switch_desc = { };
    let mut sw_desc: typec_switch_desc = { };
    let mut mux_desc: typec_mux_desc = { };
    const char *str;
    int ret;
    ret = fwnode_property_read_u8(fwnode, "usb2-port-number", &port.usb2_port);
    if (ret)
    return ret;
    ret = fwnode_property_read_u8(fwnode, "usb3-port-number", &port.usb3_port);
    if (ret)
    return ret;
    ret = fwnode_property_read_string(fwnode, "sbu-orientation", &str);
    if (!ret)
    port.sbu_orientation = typec_find_orientation(str);
    ret = fwnode_property_read_string(fwnode, "hsl-orientation", &str);
    if (!ret)
    port.hsl_orientation = typec_find_orientation(str);
    port.num = index;
    port.pmc = pmc;
    sw_desc.fwnode = fwnode;
    sw_desc.drvdata = port;
    sw_desc.name = fwnode_get_name(fwnode);
    sw_desc.set = pmc_usb_set_orientation;
    port.typec_sw = typec_switch_register(pmc.dev, &sw_desc);
    if (IS_ERR(port.typec_sw))
    return PTR_ERR(port.typec_sw);
    mux_desc.fwnode = fwnode;
    mux_desc.drvdata = port;
    mux_desc.name = fwnode_get_name(fwnode);
    mux_desc.set = pmc_usb_mux_set;
    port.typec_mux = typec_mux_register(pmc.dev, &mux_desc);
    if (IS_ERR(port.typec_mux)) {
    ret = PTR_ERR(port.typec_mux);
    goto err_unregister_switch;
    }
    desc.fwnode = fwnode;
    desc.driver_data = port;
    desc.name = fwnode_get_name(fwnode);
    desc.set = pmc_usb_set_role;
    desc.allow_userspace_control = true;
    port.usb_sw = usb_role_switch_register(pmc.dev, &desc);
    if (IS_ERR(port.usb_sw)) {
    ret = PTR_ERR(port.usb_sw);
    goto err_unregister_mux;
    }
    return 0;
    err_unregister_mux:
    typec_mux_unregister(port.typec_mux);
    err_unregister_switch:
    typec_switch_unregister(port.typec_sw);
    return ret;
    }
// IOM ACPI IDs and IOM_PORT_STATUS_OFFSET
    static const struct acpi_device_id iom_acpi_ids[] = {
// TigerLake
    { "INTC1072", IOM_PORT_STATUS_REGS(0x560, IOM_PORT_STATUS_REGS_SZ_4) },
// AlderLake
    { "INTC1079", IOM_PORT_STATUS_REGS(0x160, IOM_PORT_STATUS_REGS_SZ_4) },
// Meteor Lake
    { "INTC107A", IOM_PORT_STATUS_REGS(0x160, IOM_PORT_STATUS_REGS_SZ_4) },
// Lunar Lake
    { "INTC10EA", IOM_PORT_STATUS_REGS(0x150, IOM_PORT_STATUS_REGS_SZ_8) },
    {}
    };
#[no_mangle]
unsafe extern "C" fn pmc_usb_probe_iom(pmc: *mut pmc_usb) -> c_int {
    static int pmc_usb_probe_iom(struct pmc_usb *pmc)
    {
    struct list_head resource_list;
    struct resource_entry *rentry;
    static const struct acpi_device_id *dev_id;
    struct acpi_device *adev = core::ptr::null_mut();
    int ret;
    for (dev_id = &iom_acpi_ids[0]; dev_id.id[0]; dev_id++) {
    adev = acpi_dev_get_first_match_dev(dev_id.id, core::ptr::null_mut(), -1);
    if (adev)
    break;
    }
    if (!adev)
    return -ENODEV;
    pmc.iom_port_status_offset = IOM_PORT_STATUS_REGS_OFFSET(dev_id.driver_data);
    pmc.iom_port_status_size = IOM_PORT_STATUS_REGS_SIZE(dev_id.driver_data);
    INIT_LIST_HEAD(&resource_list);
    ret = acpi_dev_get_memory_resources(adev, &resource_list);
    if (ret < 0) {
    acpi_dev_put(adev);
    return ret;
    }
    rentry = list_first_entry_or_null(&resource_list, struct resource_entry, node);
    if (rentry)
    pmc.iom_base = devm_ioremap_resource(pmc.dev, rentry.res);
    acpi_dev_free_resource_list(&resource_list);
    if (!pmc.iom_base) {
    acpi_dev_put(adev);
    return -ENOMEM;
    }
    if (IS_ERR(pmc.iom_base)) {
    acpi_dev_put(adev);
    return PTR_ERR(pmc.iom_base);
    }
    pmc.iom_adev = adev;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn port_iom_status_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    static int port_iom_status_show(struct seq_file *s, void *unused)
    {
    struct pmc_usb_port *port = s.private;
    update_port_status(port);
    seq_printf(s, "0x%08x\n", port.iom_status);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(port_iom_status);
#[no_mangle]
unsafe extern "C" fn pmc_mux_port_debugfs_init(port: *mut pmc_usb_port) {
    static void pmc_mux_port_debugfs_init(struct pmc_usb_port *port)
    {
    struct dentry *debugfs_dir;
    char name[8];
    snprintf(name, sizeof(name), "port%d", port.usb3_port - 1);
    debugfs_dir = debugfs_create_dir(name, port.pmc.dentry);
    debugfs_create_file("iom_status", 0400, debugfs_dir, port,
    &port_iom_status_fops);
    }
#[no_mangle]
unsafe extern "C" fn pmc_usb_probe(pdev: *mut platform_device) -> c_int {
    static int pmc_usb_probe(struct platform_device *pdev)
    {
    struct fwnode_handle *fwnode = core::ptr::null_mut();
    struct pmc_usb *pmc;
    let mut num_ports: u8 = 0;
    let mut i: c_int = 0;
    int ret;
    device_for_each_child_node(&pdev.dev, fwnode)
    num_ports++;
// The IOM microcontroller has a limitation of max 4 ports.
    if (num_ports > 4) {
    dev_err(&pdev.dev, "driver limited to 4 ports\n");
    return -ERANGE;
    }
    pmc = devm_kzalloc(&pdev.dev, struct_size(pmc, port, num_ports), GFP_KERNEL);
    if (!pmc)
    return -ENOMEM;
    pmc.num_ports = num_ports;
    pmc.ipc = devm_intel_scu_ipc_dev_get(&pdev.dev);
    if (!pmc.ipc)
    return -EPROBE_DEFER;
    pmc.dev = &pdev.dev;
    ret = pmc_usb_probe_iom(pmc);
    if (ret)
    return ret;
    pmc.dentry = debugfs_create_dir(dev_name(pmc.dev), pmc_mux_debugfs_root);
//
// For every physical USB connector (USB2 and USB3 combo) there is a
// child ACPI device node under the PMC mux ACPI device object.
//
    for (i = 0; i < pmc.num_ports; i++) {
    fwnode = device_get_next_child_node(pmc.dev, fwnode);
    if (!fwnode)
    break;
    ret = pmc_usb_register_port(pmc, i, fwnode);
    if (ret) {
    fwnode_handle_put(fwnode);
    goto err_remove_ports;
    }
    pmc_mux_port_debugfs_init(&pmc.port[i]);
    }
    platform_set_drvdata(pdev, pmc);
    return 0;
    err_remove_ports:
    for (i = 0; i < pmc.num_ports; i++) {
    typec_switch_unregister(pmc.port[i].typec_sw);
    typec_mux_unregister(pmc.port[i].typec_mux);
    usb_role_switch_unregister(pmc.port[i].usb_sw);
    }
    acpi_dev_put(pmc.iom_adev);
    debugfs_remove(pmc.dentry);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pmc_usb_remove(pdev: *mut platform_device) {
    static void pmc_usb_remove(struct platform_device *pdev)
    {
    struct pmc_usb *pmc = platform_get_drvdata(pdev);
    int i;
    for (i = 0; i < pmc.num_ports; i++) {
    typec_switch_unregister(pmc.port[i].typec_sw);
    typec_mux_unregister(pmc.port[i].typec_mux);
    usb_role_switch_unregister(pmc.port[i].usb_sw);
    }
    acpi_dev_put(pmc.iom_adev);
    debugfs_remove(pmc.dentry);
    }
    static const struct acpi_device_id pmc_usb_acpi_ids[] = {
    { "INTC105C", },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, pmc_usb_acpi_ids);
    static struct platform_driver pmc_usb_driver = {
    .driver = {
    .name = "intel_pmc_usb",
    .acpi_match_table = ACPI_PTR(pmc_usb_acpi_ids),
    },
    .probe = pmc_usb_probe,
    .remove = pmc_usb_remove,
    };
#[no_mangle]
unsafe extern "C" fn pmc_usb_init() -> int __init {
    static int __init pmc_usb_init(void)
    {
    pmc_mux_debugfs_root = debugfs_create_dir("intel_pmc_mux", usb_debug_root);
    return platform_driver_register(&pmc_usb_driver);
    }
    module_init(pmc_usb_init);
#[no_mangle]
unsafe extern "C" fn pmc_usb_exit() -> void __exit {
    static void __exit pmc_usb_exit(void)
    {
    platform_driver_unregister(&pmc_usb_driver);
    debugfs_remove(pmc_mux_debugfs_root);
    }
    module_exit(pmc_usb_exit);
    MODULE_AUTHOR("Heikki Krogerus <heikki.krogerus@linux.intel.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Intel PMC USB mux control");
