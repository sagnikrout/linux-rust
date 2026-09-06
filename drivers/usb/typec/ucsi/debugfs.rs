//! Automatically rewritten from C to Rust
//! Source: drivers/usb/typec/ucsi/debugfs.c
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
// UCSI debugfs interface
//
// Copyright (C) 2023 Intel Corporation
//
// Authors: Rajaram Regupathy <rajaram.regupathy@intel.com>
// Gopal Saranya <saranya.gopal@intel.com>
//

    static struct dentry *ucsi_debugfs_root;
#[no_mangle]
unsafe extern "C" fn ucsi_cmd(data: *mut c_void, val: u64) -> c_int {
    static int ucsi_cmd(void *data, u64 val)
    {
    struct ucsi *ucsi = data;
    int ret;
    memset(&ucsi.debugfs.response, 0, sizeof(ucsi.debugfs.response));
    ucsi.debugfs.status = 0;
    switch (UCSI_COMMAND(val)) {
    case UCSI_SET_CCOM:
    case UCSI_SET_UOR:
    case UCSI_SET_PDR:
    case UCSI_CONNECTOR_RESET:
    case UCSI_SET_SINK_PATH:
    case UCSI_SET_NEW_CAM:
    case UCSI_SET_USB:
    case UCSI_SET_POWER_LEVEL:
    case UCSI_READ_POWER_LEVEL:
    ret = ucsi_send_command(ucsi, val, core::ptr::null_mut(), 0);
    break;
    case UCSI_SET_PDOS:
    ret = ucsi_write_message_out_command(ucsi, val, core::ptr::null_mut(), 0,
    ucsi.debugfs.message_out,
    UCSI_COMMAND_DATA_LEN(val));
    break;
    case UCSI_GET_CAPABILITY:
    case UCSI_GET_CONNECTOR_CAPABILITY:
    case UCSI_GET_ALTERNATE_MODES:
    case UCSI_GET_CAM_SUPPORTED:
    case UCSI_GET_CURRENT_CAM:
    case UCSI_GET_PDOS:
    case UCSI_GET_CABLE_PROPERTY:
    case UCSI_GET_CONNECTOR_STATUS:
    case UCSI_GET_ERROR_STATUS:
    case UCSI_GET_PD_MESSAGE:
    case UCSI_GET_ATTENTION_VDO:
    case UCSI_GET_CAM_CS:
    case UCSI_GET_LPM_PPM_INFO:
    ret = ucsi_send_command(ucsi, val,
    &ucsi.debugfs.response,
    sizeof(ucsi.debugfs.response));
    break;
    default:
    ret = -EOPNOTSUPP;
    }
    if (ret < 0) {
    ucsi.debugfs.status = ret;
    return ret;
    }
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(ucsi_cmd_fops, core::ptr::null_mut(), ucsi_cmd, "0x%llx\n");
#[no_mangle]
unsafe extern "C" fn ucsi_resp_show(s: *mut seq_file, not_used: *mut c_void) -> c_int {
    static int ucsi_resp_show(struct seq_file *s, void *not_used)
    {
    struct ucsi *ucsi = s.private;
    if (ucsi.debugfs.status)
    return ucsi.debugfs.status;
    seq_printf(s, "0x%016llx%016llx%016llx\n", ucsi.debugfs.response.ext,
    ucsi.debugfs.response.high, ucsi.debugfs.response.low);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(ucsi_resp);
#[no_mangle]
unsafe extern "C" fn ucsi_peak_curr_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int ucsi_peak_curr_show(struct seq_file *m, void *v)
    {
    struct ucsi *ucsi = m.private;
    seq_printf(m, "%u mA\n", ucsi.connector.peak_current);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(ucsi_peak_curr);
#[no_mangle]
unsafe extern "C" fn ucsi_avg_curr_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int ucsi_avg_curr_show(struct seq_file *m, void *v)
    {
    struct ucsi *ucsi = m.private;
    seq_printf(m, "%u mA\n", ucsi.connector.avg_current);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(ucsi_avg_curr);
#[no_mangle]
unsafe extern "C" fn ucsi_vbus_volt_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int ucsi_vbus_volt_show(struct seq_file *m, void *v)
    {
    struct ucsi *ucsi = m.private;
    seq_printf(m, "%u mV\n", ucsi.connector.vbus_voltage);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(ucsi_vbus_volt);
    static ssize_t ucsi_message_out_write(struct file *file,
    const char __user *data, size_t count, loff_t *ppos)
    {
    struct ucsi *ucsi = file.private_data;
    int ret;
    char *buf __free(kfree) = memdup_user_nul(data, count);
    if (IS_ERR(buf))
    return PTR_ERR(buf);
    ret = hex2bin(ucsi.debugfs.message_out, buf,
    min(count / 2, sizeof(ucsi.debugfs.message_out)));
    if (ret)
    return ret;
    return count;
    }
    static const struct file_operations ucsi_message_out_fops = {
    .open = simple_open,
    .write = ucsi_message_out_write,
    .llseek = generic_file_llseek,
    };
#[no_mangle]
pub unsafe extern "C" fn ucsi_debugfs_register(ucsi: *mut ucsi) {
    void ucsi_debugfs_register(struct ucsi *ucsi)
    {
    ucsi.debugfs = kzalloc_obj(*ucsi.debugfs);
    if (!ucsi.debugfs)
    return;
    ucsi.debugfs.dentry = debugfs_create_dir(dev_name(ucsi.dev), ucsi_debugfs_root);
    debugfs_create_file("command", 0200, ucsi.debugfs.dentry, ucsi, &ucsi_cmd_fops);
    debugfs_create_file("response", 0400, ucsi.debugfs.dentry, ucsi, &ucsi_resp_fops);
    debugfs_create_file("peak_current", 0400, ucsi.debugfs.dentry, ucsi, &ucsi_peak_curr_fops);
    debugfs_create_file("avg_current", 0400, ucsi.debugfs.dentry, ucsi, &ucsi_avg_curr_fops);
    debugfs_create_file("vbus_voltage", 0400, ucsi.debugfs.dentry, ucsi, &ucsi_vbus_volt_fops);
    debugfs_create_file("message_out", 0200, ucsi.debugfs.dentry, ucsi,
    &ucsi_message_out_fops);
    }
#[no_mangle]
pub unsafe extern "C" fn ucsi_debugfs_unregister(ucsi: *mut ucsi) {
    void ucsi_debugfs_unregister(struct ucsi *ucsi)
    {
    if (IS_ERR_OR_NULL(ucsi) || !ucsi.debugfs)
    return;
    debugfs_remove_recursive(ucsi.debugfs.dentry);
    kfree(ucsi.debugfs);
    ucsi.debugfs = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn ucsi_debugfs_init() {
    void ucsi_debugfs_init(void)
    {
    ucsi_debugfs_root = debugfs_create_dir("ucsi", usb_debug_root);
    }
#[no_mangle]
pub unsafe extern "C" fn ucsi_debugfs_exit() {
    void ucsi_debugfs_exit(void)
    {
    debugfs_remove(ucsi_debugfs_root);
    }
