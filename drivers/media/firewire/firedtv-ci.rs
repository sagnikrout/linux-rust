//! Automatically rewritten from C to Rust
//! Source: drivers/media/firewire/firedtv-ci.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// FireDTV driver (formerly known as FireSAT)
//
// Copyright (C) 2004 Andreas Monitzer <andy@monitzer.com>
// Copyright (C) 2008 Henrik Kurelid <henrik@kurelid.se>
//

pub const EN50221_TAG_APP_INFO_ENQUIRY: c_uint = 0x9f8020;
pub const EN50221_TAG_CA_INFO_ENQUIRY: c_uint = 0x9f8030;
pub const EN50221_TAG_CA_PMT: c_uint = 0x9f8032;
pub const EN50221_TAG_ENTER_MENU: c_uint = 0x9f8022;
#[no_mangle]
unsafe extern "C" fn fdtv_ca_ready(stat: *mut firedtv_tuner_status) -> c_int {
    static int fdtv_ca_ready(struct firedtv_tuner_status *stat)
    {
    return stat.ca_initialization_status	== 1 &&
    stat.ca_error_flag		== 0 &&
    stat.ca_dvb_flag		== 1 &&
    stat.ca_module_present_status	== 1;
    }
#[no_mangle]
unsafe extern "C" fn fdtv_get_ca_flags(stat: *mut firedtv_tuner_status) -> c_int {
    static int fdtv_get_ca_flags(struct firedtv_tuner_status *stat)
    {
    let mut flags: c_int = 0;
    if (stat.ca_module_present_status == 1)
    flags |= CA_CI_MODULE_PRESENT;
    if (stat.ca_initialization_status == 1 &&
    stat.ca_error_flag            == 0 &&
    stat.ca_dvb_flag              == 1)
    flags |= CA_CI_MODULE_READY;
    return flags;
    }
#[no_mangle]
unsafe extern "C" fn fdtv_ca_get_caps(arg: *mut c_void) -> c_int {
    static int fdtv_ca_get_caps(void *arg)
    {
    struct ca_caps *cap = arg;
    cap.slot_num = 1;
    cap.slot_type = CA_CI;
    cap.descr_num = 1;
    cap.descr_type = CA_ECD;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fdtv_ca_get_slot_info(fdtv: *mut firedtv, arg: *mut c_void) -> c_int {
    static int fdtv_ca_get_slot_info(struct firedtv *fdtv, void *arg)
    {
    struct firedtv_tuner_status stat;
    struct ca_slot_info *slot = arg;
    int err;
    err = avc_tuner_status(fdtv, &stat);
    if (err)
    return err;
    if (slot.num != 0)
    return -EACCES;
    slot.type = CA_CI;
    slot.flags = fdtv_get_ca_flags(&stat);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fdtv_ca_app_info(fdtv: *mut firedtv, arg: *mut c_void) -> c_int {
    static int fdtv_ca_app_info(struct firedtv *fdtv, void *arg)
    {
    struct ca_msg *reply = arg;
    return avc_ca_app_info(fdtv, reply.msg, &reply.length);
    }
#[no_mangle]
unsafe extern "C" fn fdtv_ca_info(fdtv: *mut firedtv, arg: *mut c_void) -> c_int {
    static int fdtv_ca_info(struct firedtv *fdtv, void *arg)
    {
    struct ca_msg *reply = arg;
    return avc_ca_info(fdtv, reply.msg, &reply.length);
    }
#[no_mangle]
unsafe extern "C" fn fdtv_ca_get_mmi(fdtv: *mut firedtv, arg: *mut c_void) -> c_int {
    static int fdtv_ca_get_mmi(struct firedtv *fdtv, void *arg)
    {
    struct ca_msg *reply = arg;
    return avc_ca_get_mmi(fdtv, reply.msg, &reply.length);
    }
#[no_mangle]
unsafe extern "C" fn fdtv_ca_get_msg(fdtv: *mut firedtv, arg: *mut c_void) -> c_int {
    static int fdtv_ca_get_msg(struct firedtv *fdtv, void *arg)
    {
    struct firedtv_tuner_status stat;
    int err;
    switch (fdtv.ca_last_command) {
    case EN50221_TAG_APP_INFO_ENQUIRY:
    err = fdtv_ca_app_info(fdtv, arg);
    break;
    case EN50221_TAG_CA_INFO_ENQUIRY:
    err = fdtv_ca_info(fdtv, arg);
    break;
    default:
    err = avc_tuner_status(fdtv, &stat);
    if (err)
    break;
    if (stat.ca_mmi == 1)
    err = fdtv_ca_get_mmi(fdtv, arg);
    else {
    dev_info(fdtv.device, "unhandled CA message 0x%08x\n",
    fdtv.ca_last_command);
    err = -EACCES;
    }
    }
    fdtv.ca_last_command = 0;
    return err;
    }
#[no_mangle]
unsafe extern "C" fn fdtv_ca_pmt(fdtv: *mut firedtv, arg: *mut c_void) -> c_int {
    static int fdtv_ca_pmt(struct firedtv *fdtv, void *arg)
    {
    struct ca_msg *msg = arg;
    int data_pos;
    int data_length;
    int i;
    data_pos = 4;
    if (msg.msg[3] & 0x80) {
    data_length = 0;
    for (i = 0; i < (msg.msg[3] & 0x7f); i++)
    data_length = (data_length << 8) + msg.msg[data_pos++];
    } else {
    data_length = msg.msg[3];
    }
    if (data_length > sizeof(msg.msg) - data_pos)
    return -EINVAL;
    return avc_ca_pmt(fdtv, &msg.msg[data_pos], data_length);
    }
#[no_mangle]
unsafe extern "C" fn fdtv_ca_send_msg(fdtv: *mut firedtv, arg: *mut c_void) -> c_int {
    static int fdtv_ca_send_msg(struct firedtv *fdtv, void *arg)
    {
    struct ca_msg *msg = arg;
    int err;
// Do we need a semaphore for this?
    fdtv.ca_last_command =
    (msg.msg[0] << 16) + (msg.msg[1] << 8) + msg.msg[2];
    switch (fdtv.ca_last_command) {
    case EN50221_TAG_CA_PMT:
    err = fdtv_ca_pmt(fdtv, arg);
    break;
    case EN50221_TAG_APP_INFO_ENQUIRY:
// handled in ca_get_msg
    err = 0;
    break;
    case EN50221_TAG_CA_INFO_ENQUIRY:
// handled in ca_get_msg
    err = 0;
    break;
    case EN50221_TAG_ENTER_MENU:
    err = avc_ca_enter_menu(fdtv);
    break;
    default:
    dev_err(fdtv.device, "unhandled CA message 0x%08x\n",
    fdtv.ca_last_command);
    err = -EACCES;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn fdtv_ca_ioctl(file: *mut file, cmd: c_uint, arg: *mut c_void) -> c_int {
    static int fdtv_ca_ioctl(struct file *file, unsigned int cmd, void *arg)
    {
    struct dvb_device *dvbdev = file.private_data;
    struct firedtv *fdtv = dvbdev.priv;
    struct firedtv_tuner_status stat;
    int err;
    switch (cmd) {
    case CA_RESET:
    err = avc_ca_reset(fdtv);
    break;
    case CA_GET_CAP:
    err = fdtv_ca_get_caps(arg);
    break;
    case CA_GET_SLOT_INFO:
    err = fdtv_ca_get_slot_info(fdtv, arg);
    break;
    case CA_GET_MSG:
    err = fdtv_ca_get_msg(fdtv, arg);
    break;
    case CA_SEND_MSG:
    err = fdtv_ca_send_msg(fdtv, arg);
    break;
    default:
    dev_info(fdtv.device, "unhandled CA ioctl %u\n", cmd);
    err = -EOPNOTSUPP;
    }
// FIXME Is this necessary?
    avc_tuner_status(fdtv, &stat);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn fdtv_ca_io_poll(file: *mut file, wait: *mut poll_table) -> __poll_t {
    static __poll_t fdtv_ca_io_poll(struct file *file, poll_table *wait)
    {
    return EPOLLIN;
    }
    static const struct file_operations fdtv_ca_fops = {
    .owner		= THIS_MODULE,
    .unlocked_ioctl	= dvb_generic_ioctl,
    .open		= dvb_generic_open,
    .release	= dvb_generic_release,
    .poll		= fdtv_ca_io_poll,
    .llseek		= noop_llseek,
    };
    static const struct dvb_device fdtv_ca = {
    .users		= 1,
    .readers	= 1,
    .writers	= 1,
    .fops		= &fdtv_ca_fops,
    .kernel_ioctl	= fdtv_ca_ioctl,
    };
#[no_mangle]
pub unsafe extern "C" fn fdtv_ca_register(fdtv: *mut firedtv) -> c_int {
    int fdtv_ca_register(struct firedtv *fdtv)
    {
    struct firedtv_tuner_status stat;
    int err;
    if (avc_tuner_status(fdtv, &stat))
    return -EINVAL;
    if (!fdtv_ca_ready(&stat))
    return -EFAULT;
    err = dvb_register_device(&fdtv.adapter, &fdtv.cadev,
    &fdtv_ca, fdtv, DVB_DEVICE_CA, 0);
    if (stat.ca_application_info == 0)
    dev_err(fdtv.device, "CaApplicationInfo is not set\n");
    if (stat.ca_date_time_request == 1)
    avc_ca_get_time_date(fdtv, &fdtv.ca_time_interval);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn fdtv_ca_release(fdtv: *mut firedtv) {
    void fdtv_ca_release(struct firedtv *fdtv)
    {
    dvb_unregister_device(fdtv.cadev);
    }
