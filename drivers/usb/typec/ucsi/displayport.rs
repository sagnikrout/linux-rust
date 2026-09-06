//! Automatically rewritten from C to Rust
//! Source: drivers/usb/typec/ucsi/displayport.c
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
// UCSI DisplayPort Alternate Mode Support
//
// Copyright (C) 2018, Intel Corporation
// Author: Heikki Krogerus <heikki.krogerus@linux.intel.com>
//

    (UCSI_SET_NEW_CAM | ((_con_num_) << 16) | ((_enter_) << 23) |	\
    ((_cam_) << 24) | ((u64)(_am_) << 32))
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucsi_dp {
    pub data: typec_displayport_data,
    pub con: *mut ucsi_connector,
    pub alt: *mut typec_altmode,
    pub work: work_struct,
    pub offset: c_int,
    pub override: bool,
    pub initialized: bool,
    pub header: u32,
    pub vdo_data: *mut u32,
    pub vdo_size: u8,
}

//
// Note. Alternate mode control is optional feature in UCSI. It means that even
// if the system supports alternate modes, the OS may not be aware of them.
//
// In most cases however, the OS will be able to see the supported alternate
// modes, but it may still not be able to configure them, not even enter or exit
// them. That is because UCSI defines alt mode details and alt mode "overriding"
// as separate options.
//
// In case alt mode details are supported, but overriding is not, the driver
// will still display the supported pin assignments and configuration, but any
// changes the user attempts to do will lead into failure with return value of
// -EOPNOTSUPP.
//
#[no_mangle]
unsafe extern "C" fn ucsi_displayport_enter(alt: *mut typec_altmode, vdo: *mut u32) -> c_int {
    static int ucsi_displayport_enter(struct typec_altmode *alt, u32 *vdo)
    {
    struct ucsi_dp *dp = typec_altmode_get_drvdata(alt);
    struct ucsi *ucsi = dp.con.ucsi;
    int svdm_version;
    u64 command;
    let mut cur: u8 = 0;
    int ret;
    if (!ucsi_con_mutex_lock(dp.con))
    return -ENOTCONN;
    if (!dp.override && dp.initialized) {
    const struct typec_altmode *p = typec_altmode_get_partner(alt);
    dev_warn(&p.dev,
    "firmware doesn't support alternate mode overriding\n");
    ret = -EOPNOTSUPP;
    goto err_unlock;
    }
    command = UCSI_GET_CURRENT_CAM | UCSI_CONNECTOR_NUMBER(dp.con.num);
    ret = ucsi_send_command(ucsi, command, &cur, sizeof(cur));
    if (ret < 0) {
    if (ucsi.version > 0x0100)
    goto err_unlock;
    cur = 0xff;
    }
    if (cur != 0xff) {
    ret = dp.con.port_altmode[cur] == alt ? 0 : -EBUSY;
    goto err_unlock;
    }
//
// We can't send the New CAM command yet to the PPM as it needs the
// configuration value as well. Pretending that we have now entered the
// mode, and letting the alt mode driver continue.
//
    svdm_version = typec_altmode_get_svdm_version(alt);
    if (svdm_version < 0) {
    ret = svdm_version;
    goto err_unlock;
    }
    dp.header = VDO(USB_TYPEC_DP_SID, 1, svdm_version, CMD_ENTER_MODE);
    dp.header |= VDO_OPOS(USB_TYPEC_DP_MODE);
    dp.header |= VDO_CMDT(CMDT_RSP_ACK);
    dp.vdo_data = core::ptr::null_mut();
    dp.vdo_size = 1;
    schedule_work(&dp.work);
    ret = 0;
    err_unlock:
    ucsi_con_mutex_unlock(dp.con);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ucsi_displayport_exit(alt: *mut typec_altmode) -> c_int {
    static int ucsi_displayport_exit(struct typec_altmode *alt)
    {
    struct ucsi_dp *dp = typec_altmode_get_drvdata(alt);
    int svdm_version;
    u64 command;
    let mut ret: c_int = 0;
    if (!ucsi_con_mutex_lock(dp.con))
    return -ENOTCONN;
    if (!dp.override) {
    const struct typec_altmode *p = typec_altmode_get_partner(alt);
    dev_warn(&p.dev,
    "firmware doesn't support alternate mode overriding\n");
    ret = -EOPNOTSUPP;
    goto out_unlock;
    }
    command = UCSI_CMD_SET_NEW_CAM(dp.con.num, 0, dp.offset, 0);
    ret = ucsi_send_command(dp.con.ucsi, command, core::ptr::null_mut(), 0);
    if (ret < 0)
    goto out_unlock;
    svdm_version = typec_altmode_get_svdm_version(alt);
    if (svdm_version < 0) {
    ret = svdm_version;
    goto out_unlock;
    }
    dp.header = VDO(USB_TYPEC_DP_SID, 1, svdm_version, CMD_EXIT_MODE);
    dp.header |= VDO_OPOS(USB_TYPEC_DP_MODE);
    dp.header |= VDO_CMDT(CMDT_RSP_ACK);
    dp.vdo_data = core::ptr::null_mut();
    dp.vdo_size = 1;
    schedule_work(&dp.work);
    out_unlock:
    ucsi_con_mutex_unlock(dp.con);
    return ret;
    }
//
// We do not actually have access to the Status Update VDO, so we have to guess
// things.
//
#[no_mangle]
unsafe extern "C" fn ucsi_displayport_status_update(dp: *mut ucsi_dp) -> c_int {
    static int ucsi_displayport_status_update(struct ucsi_dp *dp)
    {
    let mut cap: u32 = dp.alt.vdo;
    dp.data.status = DP_STATUS_ENABLED;
//
// If pin assignement D is supported, claiming always
// that Multi-function is preferred.
//
    if (DP_CAP_CAPABILITY(cap) & DP_CAP_UFP_D) {
    dp.data.status |= DP_STATUS_CON_DFP_D;
    if (DP_CAP_UFP_D_PIN_ASSIGN(cap) & BIT(DP_PIN_ASSIGN_D))
    dp.data.status |= DP_STATUS_PREFER_MULTI_FUNC;
    } else {
    dp.data.status |= DP_STATUS_CON_UFP_D;
    if (DP_CAP_DFP_D_PIN_ASSIGN(cap) & BIT(DP_PIN_ASSIGN_D))
    dp.data.status |= DP_STATUS_PREFER_MULTI_FUNC;
    }
    dp.vdo_data = &dp.data.status;
    dp.vdo_size = 2;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ucsi_displayport_configure(dp: *mut ucsi_dp) -> c_int {
    static int ucsi_displayport_configure(struct ucsi_dp *dp)
    {
    u64 command;
    if (!dp.override)
    return 0;
    command = UCSI_CMD_SET_NEW_CAM(dp.con.num, 1, dp.offset, dp.data.conf);
    return ucsi_send_command(dp.con.ucsi, command, core::ptr::null_mut(), 0);
    }
    static int ucsi_displayport_vdm(struct typec_altmode *alt,
    u32 header, const u32 *data, int count)
    {
    struct ucsi_dp *dp = typec_altmode_get_drvdata(alt);
    let mut cmd_type: c_int = PD_VDO_CMDT(header);
    let mut cmd: c_int = PD_VDO_CMD(header);
    int svdm_version;
    if (!ucsi_con_mutex_lock(dp.con))
    return -ENOTCONN;
    if (!dp.override && dp.initialized) {
    const struct typec_altmode *p = typec_altmode_get_partner(alt);
    dev_warn(&p.dev,
    "firmware doesn't support alternate mode overriding\n");
    ucsi_con_mutex_unlock(dp.con);
    return -EOPNOTSUPP;
    }
    svdm_version = typec_altmode_get_svdm_version(alt);
    if (svdm_version < 0) {
    ucsi_con_mutex_unlock(dp.con);
    return svdm_version;
    }
    switch (cmd_type) {
    case CMDT_INIT:
    if (PD_VDO_SVDM_VER(header) < svdm_version) {
    typec_partner_set_svdm_version(dp.con.partner, PD_VDO_SVDM_VER(header));
    svdm_version = PD_VDO_SVDM_VER(header);
    }
    dp.header = VDO(USB_TYPEC_DP_SID, 1, svdm_version, cmd);
    dp.header |= VDO_OPOS(USB_TYPEC_DP_MODE);
    switch (cmd) {
    case DP_CMD_STATUS_UPDATE:
    if (ucsi_displayport_status_update(dp))
    dp.header |= VDO_CMDT(CMDT_RSP_NAK);
    else
    dp.header |= VDO_CMDT(CMDT_RSP_ACK);
    break;
    case DP_CMD_CONFIGURE:
    if (count < 2) {
    dp.header |= VDO_CMDT(CMDT_RSP_NAK);
    break;
    }
    dp.data.conf = *data;
    if (ucsi_displayport_configure(dp)) {
    dp.header |= VDO_CMDT(CMDT_RSP_NAK);
    } else {
    dp.header |= VDO_CMDT(CMDT_RSP_ACK);
    if (dp.initialized)
    ucsi_altmode_update_active(dp.con);
    else
    dp.initialized = true;
    }
    break;
    default:
    dp.header |= VDO_CMDT(CMDT_RSP_ACK);
    break;
    }
    schedule_work(&dp.work);
    break;
    default:
    break;
    }
    ucsi_con_mutex_unlock(dp.con);
    return 0;
    }
    static const struct typec_altmode_ops ucsi_displayport_ops = {
    .enter = ucsi_displayport_enter,
    .exit = ucsi_displayport_exit,
    .vdm = ucsi_displayport_vdm,
    };
#[no_mangle]
unsafe extern "C" fn ucsi_displayport_work(work: *mut work_struct) {
    static void ucsi_displayport_work(struct work_struct *work)
    {
    struct ucsi_dp *dp = container_of(work, struct ucsi_dp, work);
    int ret;
    ret = typec_altmode_vdm(dp.alt, dp.header,
    dp.vdo_data, dp.vdo_size);
    if (ret)
    dev_err(&dp.alt.dev, "VDM 0x%x failed\n", dp.header);
    dp.vdo_data = core::ptr::null_mut();
    dp.vdo_size = 0;
    dp.header = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ucsi_displayport_remove_partner(alt: *mut typec_altmode) {
    void ucsi_displayport_remove_partner(struct typec_altmode *alt)
    {
    struct ucsi_dp *dp;
    if (!alt)
    return;
    dp = typec_altmode_get_drvdata(alt);
    if (!dp)
    return;
    cancel_work_sync(&dp.work);
    dp.data.conf = 0;
    dp.data.status = 0;
    dp.initialized = false;
    }
    struct typec_altmode *ucsi_register_displayport(struct ucsi_connector *con,
    bool override, int offset,
    struct typec_altmode_desc *desc)
    {
    u8 all_assignments = BIT(DP_PIN_ASSIGN_C) | BIT(DP_PIN_ASSIGN_D) |
    BIT(DP_PIN_ASSIGN_E);
    struct typec_altmode *alt;
    struct ucsi_dp *dp;
// We can't rely on the firmware with the capabilities.
    desc.vdo |= DP_CAP_DP_SIGNALLING(0) | DP_CAP_RECEPTACLE;
// Claiming that we support all pin assignments
    desc.vdo |= all_assignments << 8;
    desc.vdo |= all_assignments << 16;
    alt = typec_port_register_altmode(con.port, desc);
    if (IS_ERR(alt))
    return alt;
    dp = devm_kzalloc(&alt.dev, sizeof(*dp), GFP_KERNEL);
    if (!dp) {
    typec_unregister_altmode(alt);
    return ERR_PTR(-ENOMEM);
    }
    INIT_WORK(&dp.work, ucsi_displayport_work);
    dp.override = override;
    dp.offset = offset;
    dp.con = con;
    dp.alt = alt;
    typec_altmode_set_ops(alt, &ucsi_displayport_ops);
    typec_altmode_set_drvdata(alt, dp);
    return alt;
    }
