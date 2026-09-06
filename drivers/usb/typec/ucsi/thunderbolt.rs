//! Automatically rewritten from C to Rust
//! Source: drivers/usb/typec/ucsi/thunderbolt.c
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
// UCSI Thunderbolt Alternate Mode Support
//
// Copyright 2026 Google LLC
//

//
// struct ucsi_tbt - Thunderbolt Alternate Mode private data structure
// @con: Pointer to UCSI connector structure
// @alt: Pointer to typec altmode structure
// @work: Work structure
// @cam: An offset into the list of alternate modes supported by the PPM
// @header: VDO header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucsi_tbt {
    pub con: *mut ucsi_connector,
    pub alt: *mut typec_altmode,
    pub work: work_struct,
    pub cam: c_int,
    pub header: u32,
}

#[no_mangle]
unsafe extern "C" fn ucsi_thunderbolt_work(work: *mut work_struct) {
    static void ucsi_thunderbolt_work(struct work_struct *work)
    {
    struct ucsi_tbt *tbt = container_of(work, struct ucsi_tbt, work);
    if (typec_altmode_vdm(tbt.alt, tbt.header, core::ptr::null_mut(), 0))
    dev_err(&tbt.alt.dev, "VDM 0x%x failed\n", tbt.header);
    tbt.header = 0;
    }
    static int ucsi_thunderbolt_set_altmode(struct ucsi_tbt *tbt,
    bool enter, u32 vdo)
    {
    int svdm_version;
    int cmd;
    int ret;
    u64 command = UCSI_SET_NEW_CAM |
    UCSI_CONNECTOR_NUMBER(tbt.con.num) |
    UCSI_SET_NEW_CAM_SET_AM(tbt.cam) |
    ((u64)vdo << 32);
    if (enter)
    command |= (1 << 23);
    ret = ucsi_send_command(tbt.con.ucsi, command, core::ptr::null_mut(), 0);
    if (ret < 0)
    return ret;
    svdm_version = typec_altmode_get_svdm_version(tbt.alt);
    if (svdm_version < 0)
    return svdm_version;
    if (enter)
    cmd = CMD_ENTER_MODE;
    else
    cmd = CMD_EXIT_MODE;
    tbt.header = VDO(USB_TYPEC_TBT_SID, 1, svdm_version, cmd);
    tbt.header |= VDO_OPOS(TYPEC_TBT_MODE);
    tbt.header |= VDO_CMDT(CMDT_RSP_ACK);
    schedule_work(&tbt.work);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ucsi_thunderbolt_enter(alt: *mut typec_altmode, vdo: *mut u32) -> c_int {
    static int ucsi_thunderbolt_enter(struct typec_altmode *alt, u32 *vdo)
    {
    struct ucsi_tbt *tbt = typec_altmode_get_drvdata(alt);
    struct ucsi_connector *con = tbt.con;
    u64 command;
    let mut cur: u8 = 0;
    int ret;
    if (!ucsi_con_mutex_lock(con))
    return -ENOTCONN;
    command = UCSI_GET_CURRENT_CAM | UCSI_CONNECTOR_NUMBER(con.num);
    ret = ucsi_send_command(con.ucsi, command, &cur, sizeof(cur));
    if (ret < 0) {
    if (con.ucsi.version > 0x0100)
    goto err_unlock;
    cur = 0xff;
    }
    if (cur != 0xff) {
    if (cur >= UCSI_MAX_ALTMODES || con.port_altmode[cur] != alt)
    ret = -EBUSY;
    else
    ret = 0;
    goto err_unlock;
    }
    ret = ucsi_thunderbolt_set_altmode(tbt, true, *vdo);
    ucsi_altmode_update_active(tbt.con);
    err_unlock:
    ucsi_con_mutex_unlock(con);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ucsi_thunderbolt_exit(alt: *mut typec_altmode) -> c_int {
    static int ucsi_thunderbolt_exit(struct typec_altmode *alt)
    {
    struct ucsi_tbt *tbt = typec_altmode_get_drvdata(alt);
    int ret;
    if (!ucsi_con_mutex_lock(tbt.con))
    return -ENOTCONN;
    ret = ucsi_thunderbolt_set_altmode(tbt, false, 0);
    ucsi_con_mutex_unlock(tbt.con);
    return ret;
    }
    static int ucsi_thunderbolt_vdm(struct typec_altmode *alt,
    u32 header, const u32 *data, int count)
    {
    struct ucsi_tbt *tbt = typec_altmode_get_drvdata(alt);
    let mut cmd_type: c_int = PD_VDO_CMDT(header);
    let mut cmd: c_int = PD_VDO_CMD(header);
    int svdm_version;
    if (!ucsi_con_mutex_lock(tbt.con))
    return -ENOTCONN;
    svdm_version = typec_altmode_get_svdm_version(alt);
    if (svdm_version < 0) {
    ucsi_con_mutex_unlock(tbt.con);
    return svdm_version;
    }
    switch (cmd_type) {
    case CMDT_INIT:
    if (PD_VDO_SVDM_VER(header) < svdm_version) {
    svdm_version = PD_VDO_SVDM_VER(header);
    typec_partner_set_svdm_version(tbt.con.partner, svdm_version);
    }
    tbt.header = VDO(USB_TYPEC_TBT_SID, 1, svdm_version, cmd);
    tbt.header |= VDO_OPOS(TYPEC_TBT_MODE);
    tbt.header |= VDO_CMDT(CMDT_RSP_ACK);
    schedule_work(&tbt.work);
    break;
    default:
    break;
    }
    ucsi_con_mutex_unlock(tbt.con);
    return 0;
    }
    static const struct typec_altmode_ops ucsi_thunderbolt_ops = {
    .enter = ucsi_thunderbolt_enter,
    .exit = ucsi_thunderbolt_exit,
    .vdm = ucsi_thunderbolt_vdm,
    };
    struct typec_altmode *ucsi_register_thunderbolt(struct ucsi_connector *con,
    bool override, int offset,
    struct typec_altmode_desc *desc)
    {
    struct typec_altmode *alt;
    struct ucsi_tbt *tbt;
    alt = typec_port_register_altmode(con.port, desc);
    if (IS_ERR(alt) || !override)
    return alt;
    tbt = devm_kzalloc(&alt.dev, sizeof(*tbt), GFP_KERNEL);
    if (!tbt) {
    typec_unregister_altmode(alt);
    return ERR_PTR(-ENOMEM);
    }
    tbt.cam = offset;
    tbt.con = con;
    tbt.alt = alt;
    INIT_WORK(&tbt.work, ucsi_thunderbolt_work);
    typec_altmode_set_drvdata(alt, tbt);
    typec_altmode_set_ops(alt, &ucsi_thunderbolt_ops);
    return alt;
    }
#[no_mangle]
pub unsafe extern "C" fn ucsi_thunderbolt_remove_partner(alt: *mut typec_altmode) {
    void ucsi_thunderbolt_remove_partner(struct typec_altmode *alt)
    {
    struct ucsi_tbt *tbt;
    if (alt) {
    tbt = typec_altmode_get_drvdata(alt);
    if (tbt)
    cancel_work_sync(&tbt.work);
    }
    }
