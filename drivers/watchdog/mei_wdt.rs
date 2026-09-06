//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/mei_wdt.c
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
// Intel Management Engine Interface (Intel MEI) Linux driver
// Copyright (c) 2015, Intel Corporation.
//

//
// iAMT Watchdog Device
//

// Commands
pub const MEI_MANAGEMENT_CONTROL: c_uint = 0x02;
// MEI Management Control version number
pub const MEI_MC_VERSION_NUMBER: c_uint = 0x10;
// Sub Commands
pub const MEI_MC_START_WD_TIMER_REQ: c_uint = 0x13;
pub const MEI_MC_START_WD_TIMER_RES: c_uint = 0x83;
pub const MEI_WDT_STATUS_SUCCESS: c_int = 0;
pub const MEI_WDT_WDSTATE_NOT_REQUIRED: c_uint = 0x1;
pub const MEI_MC_STOP_WD_TIMER_REQ: c_uint = 0x14;
//
// enum mei_wdt_state - internal watchdog state
//
// @MEI_WDT_PROBE: wd in probing stage
// @MEI_WDT_IDLE: wd is idle and not opened
// @MEI_WDT_START: wd was opened, start was called
// @MEI_WDT_RUNNING: wd is expecting keep alive pings
// @MEI_WDT_STOPPING: wd is stopping and will move to IDLE
// @MEI_WDT_NOT_REQUIRED: wd device is not required
//
    enum mei_wdt_state {
    MEI_WDT_PROBE,
    MEI_WDT_IDLE,
    MEI_WDT_START,
    MEI_WDT_RUNNING,
    MEI_WDT_STOPPING,
    MEI_WDT_NOT_REQUIRED,
    };
    static const char *mei_wdt_state_str(enum mei_wdt_state state)
    {
    switch (state) {
    case MEI_WDT_PROBE:
    return "PROBE";
    case MEI_WDT_IDLE:
    return "IDLE";
    case MEI_WDT_START:
    return "START";
    case MEI_WDT_RUNNING:
    return "RUNNING";
    case MEI_WDT_STOPPING:
    return "STOPPING";
    case MEI_WDT_NOT_REQUIRED:
    return "NOT_REQUIRED";
    default:
    return "unknown";
    }
    }
//
// struct mei_wdt - mei watchdog driver
// @wdd: watchdog device
//
// @cldev: mei watchdog client device
// @state: watchdog internal state
// @resp_required: ping required response
// @response: ping response completion
// @unregister: unregister worker
// @reg_lock: watchdog device registration lock
// @timeout: watchdog current timeout
//
// @dbgfs_dir: debugfs dir entry
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_wdt {
    pub wdd: watchdog_device,
    pub cldev: *mut mei_cl_device,
    pub state: enum mei_wdt_state,
    pub resp_required: bool,
    pub response: completion,
    pub unregister: work_struct,
    pub reg_lock: mutex,
    pub timeout: u16,

    pub dbgfs_dir: *mut dentry,

}

//
// struct mei_mc_hdr - Management Control Command Header
//
// @command: Management Control (0x2)
// @bytecount: Number of bytes in the message beyond this byte
// @subcommand: Management Control Subcommand
// @versionnumber: Management Control Version (0x10)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_mc_hdr {
    pub command: u8,
    pub bytecount: u8,
    pub subcommand: u8,
    pub versionnumber: u8,
}

//
// struct mei_wdt_start_request - watchdog start/ping
//
// @hdr: Management Control Command Header
// @timeout: timeout value
// @reserved: reserved (legacy)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_wdt_start_request {
    pub hdr: mei_mc_hdr,
    pub timeout: u16,
    pub reserved: [u8; 17],
    pub __packed: },
//
// struct mei_wdt_start_response - watchdog start/ping response
//
// @hdr: Management Control Command Header
// @status: operation status
// @wdstate: watchdog status bit mask
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_wdt_start_response {
    pub hdr: mei_mc_hdr,
    pub status: u8,
    pub wdstate: u8,
    pub __packed: },
//
// struct mei_wdt_stop_request - watchdog stop
//
// @hdr: Management Control Command Header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_wdt_stop_request {
    pub hdr: mei_mc_hdr,
    pub __packed: },
//
// mei_wdt_ping - send wd start/ping command
//
// @wdt: mei watchdog device
//
// Return: 0 on success,
// negative errno code on failure
//
#[no_mangle]
unsafe extern "C" fn mei_wdt_ping(wdt: *mut mei_wdt) -> c_int {
    static int mei_wdt_ping(struct mei_wdt *wdt)
    {
    pub req: mei_wdt_start_request,
    pub sizeof(req): size_t req_len =,
    pub ret: c_int,
    pub req_len): memset(&req, 0,,
    pub MEI_MANAGEMENT_CONTROL: req.hdr.command =,
    pub subcommand): req.hdr.bytecount = req_len - offsetof(struct mei_mc_hdr,,
    pub MEI_MC_START_WD_TIMER_REQ: req.hdr.subcommand =,
    pub MEI_MC_VERSION_NUMBER: req.hdr.versionnumber =,
    pub wdt->timeout: req.timeout =,
    pub req_len): *mut *mut ret = mei_cldev_send(wdt->cldev, (u8 )&req,,
    if (ret < 0)
    pub ret: return,
    pub 0: return,
    }
//
// mei_wdt_stop - send wd stop command
//
// @wdt: mei watchdog device
//
// Return: 0 on success,
// negative errno code on failure
//
#[no_mangle]
unsafe extern "C" fn mei_wdt_stop(wdt: *mut mei_wdt) -> c_int {
    static int mei_wdt_stop(struct mei_wdt *wdt)
    {
    pub req: mei_wdt_stop_request,
    pub sizeof(req): size_t req_len =,
    pub ret: c_int,
    pub req_len): memset(&req, 0,,
    pub MEI_MANAGEMENT_CONTROL: req.hdr.command =,
    pub subcommand): req.hdr.bytecount = req_len - offsetof(struct mei_mc_hdr,,
    pub MEI_MC_STOP_WD_TIMER_REQ: req.hdr.subcommand =,
    pub MEI_MC_VERSION_NUMBER: req.hdr.versionnumber =,
    pub req_len): *mut *mut ret = mei_cldev_send(wdt->cldev, (u8 )&req,,
    if (ret < 0)
    pub ret: return,
    pub 0: return,
    }
//
// mei_wdt_ops_start - wd start command from the watchdog core.
//
// @wdd: watchdog device
//
// Return: 0 on success or -ENODEV;
//
#[no_mangle]
unsafe extern "C" fn mei_wdt_ops_start(wdd: *mut watchdog_device) -> c_int {
    static int mei_wdt_ops_start(struct watchdog_device *wdd)
    {
    pub watchdog_get_drvdata(wdd): *mut *mut mei_wdt wdt =,
    pub MEI_WDT_START: wdt->state =,
    pub wdt->timeout: wdd->timeout =,
    pub 0: return,
    }
//
// mei_wdt_ops_stop - wd stop command from the watchdog core.
//
// @wdd: watchdog device
//
// Return: 0 if success, negative errno code for failure
//
#[no_mangle]
unsafe extern "C" fn mei_wdt_ops_stop(wdd: *mut watchdog_device) -> c_int {
    static int mei_wdt_ops_stop(struct watchdog_device *wdd)
    {
    pub watchdog_get_drvdata(wdd): *mut *mut mei_wdt wdt =,
    pub ret: c_int,
    if (wdt.state != MEI_WDT_RUNNING)
    pub 0: return,
    pub MEI_WDT_STOPPING: wdt->state =,
    pub mei_wdt_stop(wdt): ret =,
    if (ret)
    pub ret: return,
    pub MEI_WDT_IDLE: wdt->state =,
    pub 0: return,
    }
//
// mei_wdt_ops_ping - wd ping command from the watchdog core.
//
// @wdd: watchdog device
//
// Return: 0 if success, negative errno code on failure
//
#[no_mangle]
unsafe extern "C" fn mei_wdt_ops_ping(wdd: *mut watchdog_device) -> c_int {
    static int mei_wdt_ops_ping(struct watchdog_device *wdd)
    {
    pub watchdog_get_drvdata(wdd): *mut *mut mei_wdt wdt =,
    pub ret: c_int,
    if (wdt.state != MEI_WDT_START && wdt.state != MEI_WDT_RUNNING)
    pub 0: return,
    if (wdt.resp_required)
    pub MEI_WDT_RUNNING: wdt->state =,
    pub mei_wdt_ping(wdt): ret =,
    if (ret)
    pub ret: return,
    if (wdt.resp_required)
    pub wait_for_completion_killable(&wdt->response): ret =,
    pub ret: return,
    }
//
// mei_wdt_ops_set_timeout - wd set timeout command from the watchdog core.
//
// @wdd: watchdog device
// @timeout: timeout value to set
//
// Return: 0 if success, negative errno code for failure
//
    static int mei_wdt_ops_set_timeout(struct watchdog_device *wdd,
    unsigned int timeout)
    {
    pub watchdog_get_drvdata(wdd): *mut *mut mei_wdt wdt =,
// valid value is already checked by the caller
    pub timeout: wdt->timeout =,
    pub timeout: wdd->timeout =,
    pub 0: return,
    }
    static const struct watchdog_ops wd_ops = {
    .owner       = THIS_MODULE,
    .start       = mei_wdt_ops_start,
    .stop        = mei_wdt_ops_stop,
    .ping        = mei_wdt_ops_ping,
    .set_timeout = mei_wdt_ops_set_timeout,
}

// not const as the firmware_version field need to be retrieved
    static struct watchdog_info wd_info = {
    .identity = INTEL_AMT_WATCHDOG_ID,
    .options  = WDIOF_KEEPALIVEPING |
    WDIOF_SETTIMEOUT |
    WDIOF_ALARMONLY,
    };
//
// __mei_wdt_is_registered - check if wdt is registered
//
// @wdt: mei watchdog device
//
// Return: true if the wdt is registered with the watchdog subsystem
// Locking: should be called under wdt->reg_lock
//
#[no_mangle]
pub unsafe extern "C" fn __mei_wdt_is_registered(wdt: *mut mei_wdt) -> bool {
    static inline bool __mei_wdt_is_registered(struct mei_wdt *wdt)
    {
    return !!watchdog_get_drvdata(&wdt.wdd);
    }
//
// mei_wdt_unregister - unregister from the watchdog subsystem
//
// @wdt: mei watchdog device
//
#[no_mangle]
unsafe extern "C" fn mei_wdt_unregister(wdt: *mut mei_wdt) {
    static void mei_wdt_unregister(struct mei_wdt *wdt)
    {
    mutex_lock(&wdt.reg_lock);
    if (__mei_wdt_is_registered(wdt)) {
    watchdog_unregister_device(&wdt.wdd);
    watchdog_set_drvdata(&wdt.wdd, core::ptr::null_mut());
    memset(&wdt.wdd, 0, sizeof(wdt.wdd));
    }
    mutex_unlock(&wdt.reg_lock);
    }
//
// mei_wdt_register - register with the watchdog subsystem
//
// @wdt: mei watchdog device
//
// Return: 0 if success, negative errno code for failure
//
#[no_mangle]
unsafe extern "C" fn mei_wdt_register(wdt: *mut mei_wdt) -> c_int {
    static int mei_wdt_register(struct mei_wdt *wdt)
    {
    struct device *dev;
    int ret;
    if (!wdt || !wdt.cldev)
    return -EINVAL;
    dev = &wdt.cldev.dev;
    mutex_lock(&wdt.reg_lock);
    if (__mei_wdt_is_registered(wdt)) {
    ret = 0;
    goto out;
    }
    wdt.wdd.info = &wd_info;
    wdt.wdd.ops = &wd_ops;
    wdt.wdd.parent = dev;
    wdt.wdd.timeout = MEI_WDT_DEFAULT_TIMEOUT;
    wdt.wdd.min_timeout = MEI_WDT_MIN_TIMEOUT;
    wdt.wdd.max_timeout = MEI_WDT_MAX_TIMEOUT;
    watchdog_set_drvdata(&wdt.wdd, wdt);
    watchdog_stop_on_reboot(&wdt.wdd);
    watchdog_stop_on_unregister(&wdt.wdd);
    ret = watchdog_register_device(&wdt.wdd);
    if (ret)
    watchdog_set_drvdata(&wdt.wdd, core::ptr::null_mut());
    wdt.state = MEI_WDT_IDLE;
    out:
    mutex_unlock(&wdt.reg_lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mei_wdt_unregister_work(work: *mut work_struct) {
    static void mei_wdt_unregister_work(struct work_struct *work)
    {
    struct mei_wdt *wdt = container_of(work, struct mei_wdt, unregister);
    mei_wdt_unregister(wdt);
    }
//
// mei_wdt_rx - callback for data receive
//
// @cldev: bus device
//
#[no_mangle]
unsafe extern "C" fn mei_wdt_rx(cldev: *mut mei_cl_device) {
    static void mei_wdt_rx(struct mei_cl_device *cldev)
    {
    struct mei_wdt *wdt = mei_cldev_get_drvdata(cldev);
    struct mei_wdt_start_response res;
    let mut res_len: usize = sizeof(res);
    int ret;
    ret = mei_cldev_recv(wdt.cldev, (u8 *)&res, res_len);
    if (ret < 0) {
    dev_err(&cldev.dev, "failure in recv %d\n", ret);
    return;
    }
// Empty response can be sent on stop
    if (ret == 0)
    return;
    if (ret < sizeof(struct mei_mc_hdr)) {
    dev_err(&cldev.dev, "recv small data %d\n", ret);
    return;
    }
    if (res.hdr.command != MEI_MANAGEMENT_CONTROL ||
    res.hdr.versionnumber != MEI_MC_VERSION_NUMBER) {
    dev_err(&cldev.dev, "wrong command received\n");
    return;
    }
    if (res.hdr.subcommand != MEI_MC_START_WD_TIMER_RES) {
    dev_warn(&cldev.dev, "unsupported command %d :%s[%d]\n",
    res.hdr.subcommand,
    mei_wdt_state_str(wdt.state),
    wdt.state);
    return;
    }
// Run the unregistration in a worker as this can be
// run only after ping completion, otherwise the flow will
// deadlock on watchdog core mutex.
//
    if (wdt.state == MEI_WDT_RUNNING) {
    if (res.wdstate & MEI_WDT_WDSTATE_NOT_REQUIRED) {
    wdt.state = MEI_WDT_NOT_REQUIRED;
    schedule_work(&wdt.unregister);
    }
    goto out;
    }
    if (wdt.state == MEI_WDT_PROBE) {
    if (res.wdstate & MEI_WDT_WDSTATE_NOT_REQUIRED) {
    wdt.state = MEI_WDT_NOT_REQUIRED;
    } else {
// stop the watchdog and register watchdog device
    mei_wdt_stop(wdt);
    mei_wdt_register(wdt);
    }
    return;
    }
    dev_warn(&cldev.dev, "not in correct state %s[%d]\n",
    mei_wdt_state_str(wdt.state), wdt.state);
    out:
    if (!completion_done(&wdt.response))
    complete(&wdt.response);
    }
//
// mei_wdt_notif - callback for event notification
//
// @cldev: bus device
//
#[no_mangle]
unsafe extern "C" fn mei_wdt_notif(cldev: *mut mei_cl_device) {
    static void mei_wdt_notif(struct mei_cl_device *cldev)
    {
    struct mei_wdt *wdt = mei_cldev_get_drvdata(cldev);
    if (wdt.state != MEI_WDT_NOT_REQUIRED)
    return;
    mei_wdt_register(wdt);
    }

    static ssize_t mei_dbgfs_read_activation(struct file *file, char __user *ubuf,
    size_t cnt, loff_t *ppos)
    {
    struct mei_wdt *wdt = file.private_data;
    let mut bufsz: usize = 32;
    char buf[32];
    ssize_t pos;
    mutex_lock(&wdt.reg_lock);
    pos = scnprintf(buf, bufsz, "%s\n",
    __mei_wdt_is_registered(wdt) ? "activated" : "deactivated");
    mutex_unlock(&wdt.reg_lock);
    return simple_read_from_buffer(ubuf, cnt, ppos, buf, pos);
    }
    static const struct file_operations dbgfs_fops_activation = {
    .open    = simple_open,
    .read    = mei_dbgfs_read_activation,
    .llseek  = generic_file_llseek,
    };
    static ssize_t mei_dbgfs_read_state(struct file *file, char __user *ubuf,
    size_t cnt, loff_t *ppos)
    {
    struct mei_wdt *wdt = file.private_data;
    char buf[32];
    ssize_t pos;
    pos = scnprintf(buf, sizeof(buf), "state: %s\n",
    mei_wdt_state_str(wdt.state));
    return simple_read_from_buffer(ubuf, cnt, ppos, buf, pos);
    }
    static const struct file_operations dbgfs_fops_state = {
    .open = simple_open,
    .read = mei_dbgfs_read_state,
    .llseek = generic_file_llseek,
    };
#[no_mangle]
unsafe extern "C" fn dbgfs_unregister(wdt: *mut mei_wdt) {
    static void dbgfs_unregister(struct mei_wdt *wdt)
    {
    debugfs_remove_recursive(wdt.dbgfs_dir);
    wdt.dbgfs_dir = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn dbgfs_register(wdt: *mut mei_wdt) {
    static void dbgfs_register(struct mei_wdt *wdt)
    {
    struct dentry *dir;
    dir = debugfs_create_dir(KBUILD_MODNAME, core::ptr::null_mut());
    wdt.dbgfs_dir = dir;
    debugfs_create_file("state", S_IRUSR, dir, wdt, &dbgfs_fops_state);
    debugfs_create_file("activation", S_IRUSR, dir, wdt,
    &dbgfs_fops_activation);
    }

    static inline void dbgfs_unregister(struct mei_wdt *wdt) {}
    static inline void dbgfs_register(struct mei_wdt *wdt) {}

    static int mei_wdt_probe(struct mei_cl_device *cldev,
    const struct mei_cl_device_id *id)
    {
    struct mei_wdt *wdt;
    int ret;
    wdt = kzalloc_obj(struct mei_wdt);
    if (!wdt)
    return -ENOMEM;
    wdt.timeout = MEI_WDT_DEFAULT_TIMEOUT;
    wdt.state = MEI_WDT_PROBE;
    wdt.cldev = cldev;
    wdt.resp_required = mei_cldev_ver(cldev) > 0x1;
    mutex_init(&wdt.reg_lock);
    init_completion(&wdt.response);
    INIT_WORK(&wdt.unregister, mei_wdt_unregister_work);
    mei_cldev_set_drvdata(cldev, wdt);
    ret = mei_cldev_enable(cldev);
    if (ret < 0) {
    dev_err(&cldev.dev, "Could not enable cl device\n");
    goto err_out;
    }
    ret = mei_cldev_register_rx_cb(wdt.cldev, mei_wdt_rx);
    if (ret) {
    dev_err(&cldev.dev, "Could not reg rx event ret=%d\n", ret);
    goto err_disable;
    }
    ret = mei_cldev_register_notif_cb(wdt.cldev, mei_wdt_notif);
// on legacy devices notification is not supported
//
    if (ret && ret != -EOPNOTSUPP) {
    dev_err(&cldev.dev, "Could not reg notif event ret=%d\n", ret);
    goto err_disable;
    }
    wd_info.firmware_version = mei_cldev_ver(cldev);
    if (wdt.resp_required)
    ret = mei_wdt_ping(wdt);
    else
    ret = mei_wdt_register(wdt);
    if (ret)
    goto err_disable;
    dbgfs_register(wdt);
    return 0;
    err_disable:
    mei_cldev_disable(cldev);
    err_out:
    kfree(wdt);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mei_wdt_remove(cldev: *mut mei_cl_device) {
    static void mei_wdt_remove(struct mei_cl_device *cldev)
    {
    struct mei_wdt *wdt = mei_cldev_get_drvdata(cldev);
// Free the caller in case of fw initiated or unexpected reset
    if (!completion_done(&wdt.response))
    complete(&wdt.response);
    cancel_work_sync(&wdt.unregister);
    mei_wdt_unregister(wdt);
    mei_cldev_disable(cldev);
    dbgfs_unregister(wdt);
    kfree(wdt);
    }

    0x89, 0x9D, 0xA9, 0x15, 0x14, 0xCB, 0x32, 0xAB)
    static const struct mei_cl_device_id mei_wdt_tbl[] = {
    { .uuid = MEI_UUID_WD, .version = MEI_CL_VERSION_ANY },
// required last entry
    { }
    };
    MODULE_DEVICE_TABLE(mei, mei_wdt_tbl);
    static struct mei_cl_driver mei_wdt_driver = {
    .id_table = mei_wdt_tbl,
    .name = KBUILD_MODNAME,
    .probe = mei_wdt_probe,
    .remove = mei_wdt_remove,
    };
    module_mei_cl_driver(mei_wdt_driver);
    MODULE_AUTHOR("Intel Corporation");
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Device driver for Intel MEI iAMT watchdog");
