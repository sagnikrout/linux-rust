//! Automatically rewritten from C to Rust
//! Source: drivers/soc/qcom/pmic_glink_altmode.c
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
// Copyright (c) 2019-2020, The Linux Foundation. All rights reserved.
// Copyright (c) 2022, Linaro Ltd
//

pub const PMIC_GLINK_MAX_PORTS: c_int = 3;
pub const USBC_SC8180X_NOTIFY_IND: c_uint = 0x13;
pub const USBC_CMD_WRITE_REQ: c_uint = 0x15;
pub const USBC_NOTIFY_IND: c_uint = 0x16;
pub const ALTMODE_PAN_EN: c_uint = 0x10;
pub const ALTMODE_PAN_ACK: c_uint = 0x11;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbc_write_req {
    pub hdr: pmic_glink_hdr,
    pub cmd: __le32,
    pub arg: __le32,
    pub reserved: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbc_sc8280x_dp_data {
    pub 6: u8 pin_assignment :,
    pub 1: u8 hpd_state :,
    pub 1: u8 hpd_irq :,
    pub res: [u8; 7],
}

// Used for both TBT and USB4 notifications
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbc_sc8280x_tbt_data {
    pub 3: u8 usb_speed :,
    pub 3: u8 cable_type :,
// This field is NOP on USB4, all cables support rounded rates by spec
    pub 1: u8 rounded_cable :,
    pub 1: u8 power_limited :,
    pub res: [u8; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbc_notify {
    pub hdr: pmic_glink_hdr,
    pub port_idx: u8,
    pub orientation: u8,
    pub mux_ctrl: u8,
pub const MUX_CTRL_STATE_NO_CONN: c_int = 0;
pub const MUX_CTRL_STATE_USB3_ONLY: c_int = 1;
pub const MUX_CTRL_STATE_DP4LN: c_int = 2;
pub const MUX_CTRL_STATE_USB3_DP: c_int = 3;
pub const MUX_CTRL_STATE_TUNNELING: c_int = 4;
    pub res: u8,
    pub vid: __le16,
    pub svid: __le16,
    union usbc_sc8280x_extended_data {
    pub dp: usbc_sc8280x_dp_data,
    pub tbt: usbc_sc8280x_tbt_data,
    pub extended_data: },
    pub reserved: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbc_sc8180x_notify {
    pub hdr: pmic_glink_hdr,
    pub notification: __le32,
    pub reserved: [__le32; 2],
}

    enum pmic_glink_altmode_pin_assignment {
    DPAM_HPD_OUT,
    DPAM_HPD_A,
    DPAM_HPD_B,
    DPAM_HPD_C,
    DPAM_HPD_D,
    DPAM_HPD_E,
    DPAM_HPD_F,
    };
    struct pmic_glink_altmode;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmic_glink_altmode_port {
    pub altmode: *mut pmic_glink_altmode,
    pub index: c_uint,
    pub typec_switch: *mut typec_switch,
    pub typec_mux: *mut typec_mux,
    pub state: typec_mux_state,
    pub typec_retimer: *mut typec_retimer,
    pub retimer_state: typec_retimer_state,
    pub dp_alt: typec_altmode,
    pub tbt_alt: typec_altmode,
    pub work: work_struct,
    pub bridge: *mut auxiliary_device,
    pub orientation: enum typec_orientation,
    pub svid: u16,
    pub tbt_data: usbc_sc8280x_tbt_data,
    pub dp_data: u8,
    pub mode: u8,
    pub hpd_state: u8,
    pub hpd_irq: u8,
    pub mux_ctrl: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmic_glink_altmode {
    pub dev: *mut device,
    pub owner_id: c_uint,
// To synchronize WRITE_REQ acks
    pub lock: mutex,
    pub pan_ack: completion,
    pub client: *mut pmic_glink_client,
    pub enable_work: work_struct,
    pub ports: [pmic_glink_altmode_port; PMIC_GLINK_MAX_PORTS],
}

#[no_mangle]
unsafe extern "C" fn pmic_glink_altmode_request(altmode: *mut pmic_glink_altmode, cmd: u32, arg: u32) -> c_int {
    static int pmic_glink_altmode_request(struct pmic_glink_altmode *altmode, u32 cmd, u32 arg)
    {
    let mut req: usbc_write_req = {};
    unsigned long left;
    int ret;
//
// The USBC_CMD_WRITE_REQ ack doesn't identify the request, so wait for
// one ack at a time.
//
    guard(mutex)(&altmode.lock);
    req.hdr.owner = cpu_to_le32(altmode.owner_id);
    req.hdr.type = cpu_to_le32(PMIC_GLINK_REQ_RESP);
    req.hdr.opcode = cpu_to_le32(USBC_CMD_WRITE_REQ);
    req.cmd = cpu_to_le32(cmd);
    req.arg = cpu_to_le32(arg);
    ret = pmic_glink_send(altmode.client, &req, sizeof(req));
    if (ret) {
    dev_err(altmode.dev, "failed to send altmode request: %#x (%d)\n", cmd, ret);
    return ret;
    }
    left = wait_for_completion_timeout(&altmode.pan_ack, 5 * HZ);
    if (!left) {
    dev_err(altmode.dev, "timeout waiting for altmode request ack for: %#x\n", cmd);
    return -ETIMEDOUT;
    }
    return 0;
    }
    static void pmic_glink_altmode_enable_dp(struct pmic_glink_altmode *altmode,
    struct pmic_glink_altmode_port *port,
    u8 mode, bool hpd_state,
    bool hpd_irq)
    {
    let mut dp_data: typec_displayport_data = {};
    int ret;
    dp_data.status = DP_STATUS_ENABLED;
    if (hpd_state)
    dp_data.status |= DP_STATUS_HPD_STATE;
    if (hpd_irq)
    dp_data.status |= DP_STATUS_IRQ_HPD;
    dp_data.conf = DP_CONF_SET_PIN_ASSIGN(mode);
    port.state.alt = &port.dp_alt;
    port.state.data = &dp_data;
    port.state.mode = TYPEC_MODAL_STATE(mode);
    ret = typec_mux_set(port.typec_mux, &port.state);
    if (ret)
    dev_err(altmode.dev, "failed to switch mux to DP: %d\n", ret);
    port.retimer_state.alt = &port.dp_alt;
    port.retimer_state.data = &dp_data;
    port.retimer_state.mode = TYPEC_MODAL_STATE(mode);
    ret = typec_retimer_set(port.typec_retimer, &port.retimer_state);
    if (ret)
    dev_err(altmode.dev, "failed to setup retimer to DP: %d\n", ret);
    }
    static void pmic_glink_altmode_enable_tbt(struct pmic_glink_altmode *altmode,
    struct pmic_glink_altmode_port *port)
    {
    struct usbc_sc8280x_tbt_data *tbt = &port.tbt_data;
    let mut tbt_data: typec_thunderbolt_data = {};
    u32 cable_speed;
    int ret;
// Device Discover Mode VDO
    tbt_data.device_mode = TBT_MODE;
    tbt_data.device_mode |= TBT_SET_ADAPTER(TBT_ADAPTER_TBT3);
// Cable Discover Mode VDO
    tbt_data.cable_mode = TBT_MODE;
    if (tbt.usb_speed == 0) {
    cable_speed = TBT_CABLE_USB3_PASSIVE;
    } else if (tbt.usb_speed == 1) {
    cable_speed = TBT_CABLE_10_AND_20GBPS;
    } else {
    dev_err(altmode.dev,
    "Got illegal TBT3 cable speed value (%u), falling back to passive\n",
    tbt.usb_speed);
    cable_speed = TBT_CABLE_USB3_PASSIVE;
    }
    tbt_data.cable_mode |= TBT_SET_CABLE_SPEED(cable_speed);
    if (tbt.cable_type) {
    tbt_data.cable_mode |= TBT_CABLE_ACTIVE_PASSIVE;
    tbt_data.cable_mode |= TBT_SET_CABLE_ROUNDED(tbt.rounded_cable);
    }
// Enter Mode VDO
    tbt_data.enter_vdo |= TBT_MODE;
    tbt_data.enter_vdo |= TBT_ENTER_MODE_CABLE_SPEED(cable_speed);
    if (tbt.cable_type) {
    tbt_data.enter_vdo |= TBT_CABLE_ACTIVE_PASSIVE;
    tbt_data.enter_vdo |= TBT_SET_CABLE_ROUNDED(tbt.rounded_cable);
    }
    port.state.alt = &port.tbt_alt;
    port.state.data = &tbt_data;
    port.state.mode = TYPEC_MODAL_STATE(port.mode);
    ret = typec_mux_set(port.typec_mux, &port.state);
    if (ret)
    dev_err(altmode.dev, "failed to switch mux to USB: %d\n", ret);
    port.retimer_state.alt = &port.tbt_alt;
    port.retimer_state.data = &tbt_data;
    port.retimer_state.mode = TYPEC_MODAL_STATE(port.mode);
    ret = typec_retimer_set(port.typec_retimer, &port.retimer_state);
    if (ret)
    dev_err(altmode.dev, "failed to setup retimer to USB: %d\n", ret);
    }
    static void pmic_glink_altmode_enable_usb4(struct pmic_glink_altmode *altmode,
    struct pmic_glink_altmode_port *port)
    {
    struct usbc_sc8280x_tbt_data *tbt = &port.tbt_data;
    let mut data: enter_usb_data = {};
    int ret;
    data.eudo = FIELD_PREP(EUDO_USB_MODE_MASK, EUDO_USB_MODE_USB4);
    if (tbt.usb_speed == 0) {
    data.eudo |= FIELD_PREP(EUDO_CABLE_SPEED_MASK, EUDO_CABLE_SPEED_USB4_GEN2);
    } else if (tbt.usb_speed == 1) {
    data.eudo |= FIELD_PREP(EUDO_CABLE_SPEED_MASK, EUDO_CABLE_SPEED_USB4_GEN3);
    } else {
    pr_err("Got illegal USB4 cable speed value (%u), falling back to G2\n",
    tbt.usb_speed);
    data.eudo |= FIELD_PREP(EUDO_CABLE_SPEED_MASK, EUDO_CABLE_SPEED_USB4_GEN2);
    }
    data.eudo |= FIELD_PREP(EUDO_CABLE_TYPE_MASK, tbt.cable_type);
    port.state.alt = core::ptr::null_mut();
    port.state.data = &data;
    port.state.mode = TYPEC_MODE_USB4;
    ret = typec_mux_set(port.typec_mux, &port.state);
    if (ret)
    dev_err(altmode.dev, "failed to switch mux to USB: %d\n", ret);
    port.retimer_state.alt = core::ptr::null_mut();
    port.retimer_state.data = &data;
    port.retimer_state.mode = TYPEC_MODE_USB4;
    ret = typec_retimer_set(port.typec_retimer, &port.retimer_state);
    if (ret)
    dev_err(altmode.dev, "failed to setup retimer to USB: %d\n", ret);
    }
    static void pmic_glink_altmode_enable_usb(struct pmic_glink_altmode *altmode,
    struct pmic_glink_altmode_port *port)
    {
    int ret;
    port.state.alt = core::ptr::null_mut();
    port.state.data = core::ptr::null_mut();
    port.state.mode = TYPEC_STATE_USB;
    ret = typec_mux_set(port.typec_mux, &port.state);
    if (ret)
    dev_err(altmode.dev, "failed to switch mux to USB: %d\n", ret);
    port.retimer_state.alt = core::ptr::null_mut();
    port.retimer_state.data = core::ptr::null_mut();
    port.retimer_state.mode = TYPEC_STATE_USB;
    ret = typec_retimer_set(port.typec_retimer, &port.retimer_state);
    if (ret)
    dev_err(altmode.dev, "failed to setup retimer to USB: %d\n", ret);
    }
    static void pmic_glink_altmode_safe(struct pmic_glink_altmode *altmode,
    struct pmic_glink_altmode_port *port)
    {
    int ret;
    port.state.alt = core::ptr::null_mut();
    port.state.data = core::ptr::null_mut();
    port.state.mode = TYPEC_STATE_SAFE;
    ret = typec_mux_set(port.typec_mux, &port.state);
    if (ret)
    dev_err(altmode.dev, "failed to switch mux to safe mode: %d\n", ret);
    port.retimer_state.alt = core::ptr::null_mut();
    port.retimer_state.data = core::ptr::null_mut();
    port.retimer_state.mode = TYPEC_STATE_SAFE;
    ret = typec_retimer_set(port.typec_retimer, &port.retimer_state);
    if (ret)
    dev_err(altmode.dev, "failed to setup retimer to USB: %d\n", ret);
    }
#[no_mangle]
unsafe extern "C" fn pmic_glink_altmode_worker(work: *mut work_struct) {
    static void pmic_glink_altmode_worker(struct work_struct *work)
    {
    struct pmic_glink_altmode_port *alt_port = work_to_altmode_port(work);
    struct pmic_glink_altmode *altmode = alt_port.altmode;
    enum drm_connector_status conn_status;
    typec_switch_set(alt_port.typec_switch, alt_port.orientation);
//
// MUX_CTRL_STATE_DP4LN/USB3_DP may only be set if SVID=DP, but we need
// to special-case the SVID=DP && mux_ctrl=NO_CONN case to deliver a
// HPD notification
//
    if (alt_port.svid == USB_TYPEC_DP_SID) {
    if (alt_port.mux_ctrl == MUX_CTRL_STATE_NO_CONN) {
    pmic_glink_altmode_safe(altmode, alt_port);
    } else {
    pmic_glink_altmode_enable_dp(altmode, alt_port,
    alt_port.mode,
    alt_port.hpd_state,
    alt_port.hpd_irq);
    }
    if (alt_port.hpd_state)
    conn_status = connector_status_connected;
    else
    conn_status = connector_status_disconnected;
    drm_aux_hpd_bridge_notify(&alt_port.bridge.dev, conn_status);
    } else if (alt_port.mux_ctrl == MUX_CTRL_STATE_TUNNELING) {
    if (alt_port.svid == USB_TYPEC_TBT_SID)
    pmic_glink_altmode_enable_tbt(altmode, alt_port);
    else
    pmic_glink_altmode_enable_usb4(altmode, alt_port);
    } else if (alt_port.mux_ctrl == MUX_CTRL_STATE_USB3_ONLY) {
    pmic_glink_altmode_enable_usb(altmode, alt_port);
    } else if (alt_port.mux_ctrl == MUX_CTRL_STATE_NO_CONN) {
    pmic_glink_altmode_safe(altmode, alt_port);
    } else {
    dev_err(altmode.dev, "Got unknown mux_ctrl: %u on port %u, forcing safe mode\n",
    alt_port.mux_ctrl, alt_port.index);
    pmic_glink_altmode_safe(altmode, alt_port);
    }
    pmic_glink_altmode_request(altmode, ALTMODE_PAN_ACK, alt_port.index);
    }
#[no_mangle]
unsafe extern "C" fn pmic_glink_altmode_orientation(orientation: c_uint) -> enum typec_orientation {
    static enum typec_orientation pmic_glink_altmode_orientation(unsigned int orientation)
    {
    if (orientation == 0)
    return TYPEC_ORIENTATION_NORMAL;
#[no_mangle]
pub unsafe extern "C" fn if(1: orientation ==) -> else {
    else if (orientation == 1)
    return TYPEC_ORIENTATION_REVERSE;
    else
    return TYPEC_ORIENTATION_NONE;
    }
pub const SC8180X_PORT_MASK: c_uint = 0x000000ff;
pub const SC8180X_ORIENTATION_MASK: c_uint = 0x0000ff00;
pub const SC8180X_MUX_MASK: c_uint = 0x00ff0000;
pub const SC8180X_MODE_MASK: c_uint = 0x3f000000;
pub const SC8180X_HPD_STATE_MASK: c_uint = 0x40000000;
pub const SC8180X_HPD_IRQ_MASK: c_uint = 0x80000000;
    static void pmic_glink_altmode_sc8180xp_notify(struct pmic_glink_altmode *altmode,
    const void *data, size_t len)
    {
    struct pmic_glink_altmode_port *alt_port;
    const struct usbc_sc8180x_notify *msg;
    u32 notification;
    u8 orientation;
    u8 hpd_state;
    u8 hpd_irq;
    u16 svid;
    u8 port;
    u8 mode;
    u8 mux;
    if (len != sizeof(*msg)) {
    dev_warn(altmode.dev, "invalid length of USBC_NOTIFY indication: %zd\n", len);
    return;
    }
    msg = data;
    notification = le32_to_cpu(msg.notification);
    port = FIELD_GET(SC8180X_PORT_MASK, notification);
    orientation = FIELD_GET(SC8180X_ORIENTATION_MASK, notification);
    mux = FIELD_GET(SC8180X_MUX_MASK, notification);
    mode = FIELD_GET(SC8180X_MODE_MASK, notification);
    hpd_state = FIELD_GET(SC8180X_HPD_STATE_MASK, notification);
    hpd_irq = FIELD_GET(SC8180X_HPD_IRQ_MASK, notification);
    svid = mux == 2 ? USB_TYPEC_DP_SID : 0;
    if (port >= ARRAY_SIZE(altmode.ports) || !altmode.ports[port].altmode) {
    dev_dbg(altmode.dev, "notification on undefined port %d\n", port);
    return;
    }
    alt_port = &altmode.ports[port];
    alt_port.orientation = pmic_glink_altmode_orientation(orientation);
    alt_port.svid = svid;
    alt_port.mode = mode;
    alt_port.hpd_state = hpd_state;
    alt_port.hpd_irq = hpd_irq;
    schedule_work(&alt_port.work);
    }
pub const SC8280XP_DPAM_MASK: c_uint = 0x3f;

    static void pmic_glink_altmode_sc8280xp_notify(struct pmic_glink_altmode *altmode,
    u16 svid, const void *data, size_t len)
    {
    struct pmic_glink_altmode_port *alt_port;
    const struct usbc_sc8280x_tbt_data *tbt;
    const struct usbc_sc8280x_dp_data *dp;
    const struct usbc_notify *notify;
    u8 orientation;
    u8 port;
    if (len != sizeof(*notify)) {
    dev_warn(altmode.dev, "invalid length USBC_NOTIFY_IND: %zd\n",
    len);
    return;
    }
    notify = data;
    port = notify.port_idx;
    orientation = notify.orientation;
    if (port >= ARRAY_SIZE(altmode.ports) || !altmode.ports[port].altmode) {
    dev_dbg(altmode.dev, "notification on undefined port %d\n", port);
    return;
    }
    alt_port = &altmode.ports[port];
    alt_port.orientation = pmic_glink_altmode_orientation(orientation);
    alt_port.svid = svid;
    alt_port.mux_ctrl = notify.mux_ctrl;
    if (svid == USB_TYPEC_DP_SID) {
    dp = &notify.extended_data.dp;
    alt_port.mode = dp.pin_assignment - DPAM_HPD_A;
    alt_port.hpd_state = dp.hpd_state;
    alt_port.hpd_irq = dp.hpd_irq;
    } else if (alt_port.mux_ctrl == MUX_CTRL_STATE_TUNNELING) {
// Valid for both USB4 and TBT3
    tbt = &notify.extended_data.tbt;
    alt_port.tbt_data = *tbt;
    }
    schedule_work(&alt_port.work);
    }
#[no_mangle]
unsafe extern "C" fn pmic_glink_altmode_callback(data: *const c_void, len: usize, priv: *mut c_void) {
    static void pmic_glink_altmode_callback(const void *data, size_t len, void *priv)
    {
    struct pmic_glink_altmode *altmode = priv;
    const struct pmic_glink_hdr *hdr = data;
    u16 opcode;
    u16 svid;
    opcode = le32_to_cpu(hdr.opcode) & 0xff;
    svid = le32_to_cpu(hdr.opcode) >> 16;
    switch (opcode) {
    case USBC_CMD_WRITE_REQ:
    complete(&altmode.pan_ack);
    break;
    case USBC_NOTIFY_IND:
    pmic_glink_altmode_sc8280xp_notify(altmode, svid, data, len);
    break;
    case USBC_SC8180X_NOTIFY_IND:
    pmic_glink_altmode_sc8180xp_notify(altmode, data, len);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn pmic_glink_altmode_put_retimer(data: *mut c_void) {
    static void pmic_glink_altmode_put_retimer(void *data)
    {
    typec_retimer_put(data);
    }
#[no_mangle]
unsafe extern "C" fn pmic_glink_altmode_put_mux(data: *mut c_void) {
    static void pmic_glink_altmode_put_mux(void *data)
    {
    typec_mux_put(data);
    }
#[no_mangle]
unsafe extern "C" fn pmic_glink_altmode_put_switch(data: *mut c_void) {
    static void pmic_glink_altmode_put_switch(void *data)
    {
    typec_switch_put(data);
    }
#[no_mangle]
unsafe extern "C" fn pmic_glink_altmode_enable_worker(work: *mut work_struct) {
    static void pmic_glink_altmode_enable_worker(struct work_struct *work)
    {
    struct pmic_glink_altmode *altmode = work_to_altmode(work);
    int ret;
    ret = pmic_glink_altmode_request(altmode, ALTMODE_PAN_EN, 0);
    if (ret)
    dev_err(altmode.dev, "failed to request altmode notifications: %d\n", ret);
    }
#[no_mangle]
unsafe extern "C" fn pmic_glink_altmode_pdr_notify(priv: *mut c_void, state: c_int) {
    static void pmic_glink_altmode_pdr_notify(void *priv, int state)
    {
    struct pmic_glink_altmode *altmode = priv;
    if (state == SERVREG_SERVICE_STATE_UP)
    schedule_work(&altmode.enable_work);
    }
    static const struct of_device_id pmic_glink_altmode_of_quirks[] = {
    { .compatible = "qcom,sc8180x-pmic-glink", .data = (void *)PMIC_GLINK_OWNER_USBC },
    {}
    };
    static int pmic_glink_altmode_probe(struct auxiliary_device *adev,
    const struct auxiliary_device_id *id)
    {
    struct pmic_glink_altmode_port *alt_port;
    struct pmic_glink_altmode *altmode;
    const struct of_device_id *match;
    struct fwnode_handle *fwnode;
    struct device *dev = &adev.dev;
    u32 port;
    int ret;
    altmode = devm_kzalloc(dev, sizeof(*altmode), GFP_KERNEL);
    if (!altmode)
    return -ENOMEM;
    altmode.dev = dev;
    match = of_match_device(pmic_glink_altmode_of_quirks, dev.parent);
    if (match)
    altmode.owner_id = (unsigned long)match.data;
    else
    altmode.owner_id = PMIC_GLINK_OWNER_USBC_PAN;
    INIT_WORK(&altmode.enable_work, pmic_glink_altmode_enable_worker);
    init_completion(&altmode.pan_ack);
    mutex_init(&altmode.lock);
    device_for_each_child_node(dev, fwnode) {
    ret = fwnode_property_read_u32(fwnode, "reg", &port);
    if (ret < 0) {
    dev_err(dev, "missing reg property of %pOFn\n", fwnode);
    fwnode_handle_put(fwnode);
    return ret;
    }
    if (port >= ARRAY_SIZE(altmode.ports)) {
    dev_warn(dev, "invalid connector number, ignoring\n");
    continue;
    }
    if (altmode.ports[port].altmode) {
    dev_err(dev, "multiple connector definition for port %u\n", port);
    fwnode_handle_put(fwnode);
    return -EINVAL;
    }
    alt_port = &altmode.ports[port];
    alt_port.altmode = altmode;
    alt_port.index = port;
    INIT_WORK(&alt_port.work, pmic_glink_altmode_worker);
    alt_port.bridge = devm_drm_dp_hpd_bridge_alloc(dev, to_of_node(fwnode));
    if (IS_ERR(alt_port.bridge)) {
    fwnode_handle_put(fwnode);
    return PTR_ERR(alt_port.bridge);
    }
    alt_port.dp_alt.svid = USB_TYPEC_DP_SID;
    alt_port.dp_alt.mode = USB_TYPEC_DP_MODE;
    alt_port.dp_alt.active = 1;
    alt_port.tbt_alt.svid = USB_TYPEC_TBT_SID;
    alt_port.tbt_alt.mode = TYPEC_TBT_MODE;
    alt_port.tbt_alt.active = 1;
    alt_port.typec_mux = fwnode_typec_mux_get(fwnode);
    if (IS_ERR(alt_port.typec_mux)) {
    fwnode_handle_put(fwnode);
    return dev_err_probe(dev, PTR_ERR(alt_port.typec_mux),
    "failed to acquire mode-switch for port: %d\n",
    port);
    }
    ret = devm_add_action_or_reset(dev, pmic_glink_altmode_put_mux,
    alt_port.typec_mux);
    if (ret) {
    fwnode_handle_put(fwnode);
    return ret;
    }
    alt_port.typec_retimer = fwnode_typec_retimer_get(fwnode);
    if (IS_ERR(alt_port.typec_retimer)) {
    fwnode_handle_put(fwnode);
    return dev_err_probe(dev, PTR_ERR(alt_port.typec_retimer),
    "failed to acquire retimer-switch for port: %d\n",
    port);
    }
    ret = devm_add_action_or_reset(dev, pmic_glink_altmode_put_retimer,
    alt_port.typec_retimer);
    if (ret) {
    fwnode_handle_put(fwnode);
    return ret;
    }
    alt_port.typec_switch = fwnode_typec_switch_get(fwnode);
    if (IS_ERR(alt_port.typec_switch)) {
    fwnode_handle_put(fwnode);
    return dev_err_probe(dev, PTR_ERR(alt_port.typec_switch),
    "failed to acquire orientation-switch for port: %d\n",
    port);
    }
    ret = devm_add_action_or_reset(dev, pmic_glink_altmode_put_switch,
    alt_port.typec_switch);
    if (ret) {
    fwnode_handle_put(fwnode);
    return ret;
    }
    }
    for (port = 0; port < ARRAY_SIZE(altmode.ports); port++) {
    alt_port = &altmode.ports[port];
    if (!alt_port.bridge)
    continue;
    ret = devm_drm_dp_hpd_bridge_add(dev, alt_port.bridge);
    if (ret)
    return ret;
    }
    altmode.client = devm_pmic_glink_client_alloc(dev,
    altmode.owner_id,
    pmic_glink_altmode_callback,
    pmic_glink_altmode_pdr_notify,
    altmode);
    if (IS_ERR(altmode.client))
    return PTR_ERR(altmode.client);
    pmic_glink_client_register(altmode.client);
    return 0;
    }
    static const struct auxiliary_device_id pmic_glink_altmode_id_table[] = {
    { .name = "pmic_glink.altmode", },
    {},
    };
    MODULE_DEVICE_TABLE(auxiliary, pmic_glink_altmode_id_table);
    static struct auxiliary_driver pmic_glink_altmode_driver = {
    .name = "pmic_glink_altmode",
    .probe = pmic_glink_altmode_probe,
    .id_table = pmic_glink_altmode_id_table,
    };
    module_auxiliary_driver(pmic_glink_altmode_driver);
    MODULE_DESCRIPTION("Qualcomm PMIC GLINK Altmode driver");
    MODULE_LICENSE("GPL");
