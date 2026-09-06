//! Automatically rewritten from C to Rust
//! Source: drivers/usb/storage/uas.c
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
// USB Attached SCSI
// Note that this is not the same as the USB Mass Storage driver
//
// Copyright Hans de Goede <hdegoede@redhat.com> for Red Hat, Inc. 2013 - 2016
// Copyright Matthew Wilcox for Intel Corp, 2010
// Copyright Sarah Sharp for Intel Corp, 2010
//

pub const MAX_CMNDS: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uas_dev_info {
    pub intf: *mut usb_interface,
    pub udev: *mut usb_device,
    pub cmd_urbs: usb_anchor,
    pub sense_urbs: usb_anchor,
    pub data_urbs: usb_anchor,
    pub flags: u64,
    pub resetting: int qdepth,,
    pub data_out_pipe: unsigned cmd_pipe, status_pipe, data_in_pipe,,
    pub use_streams:1: unsigned,
    pub shutdown:1: unsigned,
    pub cmnd: [*mut scsi_cmnd; MAX_CMNDS],
    pub lock: spinlock_t,
    pub work: work_struct,
    pub /: *mut *mut work_scan_work; / for async scanning,
}

    enum {
    SUBMIT_STATUS_URB	= BIT(1),
    ALLOC_DATA_IN_URB	= BIT(2),
    SUBMIT_DATA_IN_URB	= BIT(3),
    ALLOC_DATA_OUT_URB	= BIT(4),
    SUBMIT_DATA_OUT_URB	= BIT(5),
    ALLOC_CMD_URB		= BIT(6),
    SUBMIT_CMD_URB		= BIT(7),
    COMMAND_INFLIGHT        = BIT(8),
    DATA_IN_URB_INFLIGHT    = BIT(9),
    DATA_OUT_URB_INFLIGHT   = BIT(10),
    COMMAND_ABORTED         = BIT(11),
    IS_IN_WORK_LIST         = BIT(12),
    };
// Overrides scsi_pointer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uas_cmd_info {
    pub state: c_uint,
    pub uas_tag: c_uint,
    pub cmd_urb: *mut urb,
    pub data_in_urb: *mut urb,
    pub data_out_urb: *mut urb,
}

// I hate forward declarations, but I actually have a loop
    static int uas_submit_urbs(struct scsi_cmnd *cmnd,
    struct uas_dev_info *devinfo);
    static void uas_do_work(struct work_struct *work);
    static int uas_try_complete(struct scsi_cmnd *cmnd, const char *caller);
    static void uas_free_streams(struct uas_dev_info *devinfo);
    static void uas_log_cmd_state(struct scsi_cmnd *cmnd, const char *prefix,
    int status);
//
// This driver needs its own workqueue, as we need to control memory allocation.
//
// In the course of error handling and power management uas_wait_for_pending_cmnds()
// needs to flush pending work items. In these contexts we cannot allocate memory
// by doing block IO as we would deadlock. For the same reason we cannot wait
// for anything allocating memory not heeding these constraints.
//
// So we have to control all work items that can be on the workqueue we flush.
// Hence we cannot share a queue and need our own.
//
    static struct workqueue_struct *workqueue;
#[no_mangle]
unsafe extern "C" fn uas_do_work(work: *mut work_struct) {
    static void uas_do_work(struct work_struct *work)
    {
    struct uas_dev_info *devinfo =
    container_of(work, struct uas_dev_info, work);
    struct uas_cmd_info *cmdinfo;
    struct scsi_cmnd *cmnd;
    unsigned long flags;
    int i, err;
    spin_lock_irqsave(&devinfo.lock, flags);
    if (devinfo.resetting)
    goto out;
    for (i = 0; i < devinfo.qdepth; i++) {
    if (!devinfo.cmnd[i])
    continue;
    cmnd = devinfo.cmnd[i];
    cmdinfo = scsi_cmd_priv(cmnd);
    if (!(cmdinfo.state & IS_IN_WORK_LIST))
    continue;
    err = uas_submit_urbs(cmnd, cmnd.device.hostdata);
    if (!err)
    cmdinfo.state &= ~IS_IN_WORK_LIST;
    else
    queue_work(workqueue, &devinfo.work);
    }
    out:
    spin_unlock_irqrestore(&devinfo.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn uas_scan_work(work: *mut work_struct) {
    static void uas_scan_work(struct work_struct *work)
    {
    struct uas_dev_info *devinfo =
    container_of(work, struct uas_dev_info, scan_work);
    struct Scsi_Host *shost = usb_get_intfdata(devinfo.intf);
    dev_dbg(&devinfo.intf.dev, "starting scan\n");
    scsi_scan_host(shost);
    dev_dbg(&devinfo.intf.dev, "scan complete\n");
    }
#[no_mangle]
unsafe extern "C" fn uas_add_work(cmnd: *mut scsi_cmnd) {
    static void uas_add_work(struct scsi_cmnd *cmnd)
    {
    struct uas_cmd_info *cmdinfo = scsi_cmd_priv(cmnd);
    struct uas_dev_info *devinfo = cmnd.device.hostdata;
    lockdep_assert_held(&devinfo.lock);
    cmdinfo.state |= IS_IN_WORK_LIST;
    queue_work(workqueue, &devinfo.work);
    }
#[no_mangle]
unsafe extern "C" fn uas_zap_pending(devinfo: *mut uas_dev_info, result: c_int) {
    static void uas_zap_pending(struct uas_dev_info *devinfo, int result)
    {
    struct uas_cmd_info *cmdinfo;
    struct scsi_cmnd *cmnd;
    unsigned long flags;
    int i, err;
    spin_lock_irqsave(&devinfo.lock, flags);
    for (i = 0; i < devinfo.qdepth; i++) {
    if (!devinfo.cmnd[i])
    continue;
    cmnd = devinfo.cmnd[i];
    cmdinfo = scsi_cmd_priv(cmnd);
    uas_log_cmd_state(cmnd, __func__, 0);
// Sense urbs were killed, clear COMMAND_INFLIGHT manually
    cmdinfo.state &= ~COMMAND_INFLIGHT;
    cmnd.result = result << 16;
    err = uas_try_complete(cmnd, __func__);
    WARN_ON(err != 0);
    }
    spin_unlock_irqrestore(&devinfo.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn uas_sense(urb: *mut urb, cmnd: *mut scsi_cmnd) {
    static void uas_sense(struct urb *urb, struct scsi_cmnd *cmnd)
    {
    struct sense_iu *sense_iu = urb.transfer_buffer;
    struct scsi_device *sdev = cmnd.device;
    if (urb.actual_length > 16) {
    let mut len: unsigned = be16_to_cpup(&sense_iu.len);
    if (len + 16 != urb.actual_length) {
    let mut newlen: c_int = min(len + 16, urb.actual_length) - 16;
    if (newlen < 0)
    newlen = 0;
    sdev_printk(KERN_INFO, sdev, "%s: urb length %d "
    "disagrees with IU sense data length %d, "
    "using %d bytes of sense data\n", __func__,
    urb.actual_length, len, newlen);
    len = newlen;
    }
    memcpy(cmnd.sense_buffer, sense_iu.sense, len);
    }
    cmnd.result = sense_iu.status;
    }
    static void uas_log_cmd_state(struct scsi_cmnd *cmnd, const char *prefix,
    int status)
    {
    struct uas_cmd_info *ci = scsi_cmd_priv(cmnd);
    if (status == -ENODEV) /* too late */
    return;
    scmd_printk(KERN_INFO, cmnd,
    "%s %d uas-tag %d inflight:%s%s%s%s%s%s%s%s%s%s%s%s ",
    prefix, status, ci.uas_tag,
    (ci.state & SUBMIT_STATUS_URB)     ? " s-st"  : "",
    (ci.state & ALLOC_DATA_IN_URB)     ? " a-in"  : "",
    (ci.state & SUBMIT_DATA_IN_URB)    ? " s-in"  : "",
    (ci.state & ALLOC_DATA_OUT_URB)    ? " a-out" : "",
    (ci.state & SUBMIT_DATA_OUT_URB)   ? " s-out" : "",
    (ci.state & ALLOC_CMD_URB)         ? " a-cmd" : "",
    (ci.state & SUBMIT_CMD_URB)        ? " s-cmd" : "",
    (ci.state & COMMAND_INFLIGHT)      ? " CMD"   : "",
    (ci.state & DATA_IN_URB_INFLIGHT)  ? " IN"    : "",
    (ci.state & DATA_OUT_URB_INFLIGHT) ? " OUT"   : "",
    (ci.state & COMMAND_ABORTED)       ? " abort" : "",
    (ci.state & IS_IN_WORK_LIST)       ? " work"  : "");
    scsi_print_command(cmnd);
    }
#[no_mangle]
unsafe extern "C" fn uas_free_unsubmitted_urbs(cmnd: *mut scsi_cmnd) {
    static void uas_free_unsubmitted_urbs(struct scsi_cmnd *cmnd)
    {
    struct uas_cmd_info *cmdinfo;
    if (!cmnd)
    return;
    cmdinfo = scsi_cmd_priv(cmnd);
    if (cmdinfo.state & SUBMIT_CMD_URB)
    usb_free_urb(cmdinfo.cmd_urb);
// data urbs may have never gotten their submit flag set
    if (!(cmdinfo.state & DATA_IN_URB_INFLIGHT))
    usb_free_urb(cmdinfo.data_in_urb);
    if (!(cmdinfo.state & DATA_OUT_URB_INFLIGHT))
    usb_free_urb(cmdinfo.data_out_urb);
    }
#[no_mangle]
unsafe extern "C" fn uas_try_complete(cmnd: *mut scsi_cmnd, caller: *const c_char) -> c_int {
    static int uas_try_complete(struct scsi_cmnd *cmnd, const char *caller)
    {
    struct uas_cmd_info *cmdinfo = scsi_cmd_priv(cmnd);
    struct uas_dev_info *devinfo = (void *)cmnd.device.hostdata;
    lockdep_assert_held(&devinfo.lock);
    if (cmdinfo.state & (COMMAND_INFLIGHT |
    DATA_IN_URB_INFLIGHT |
    DATA_OUT_URB_INFLIGHT |
    COMMAND_ABORTED))
    return -EBUSY;
    devinfo.cmnd[cmdinfo.uas_tag - 1] = core::ptr::null_mut();
    uas_free_unsubmitted_urbs(cmnd);
    scsi_done(cmnd);
    return 0;
    }
    static void uas_xfer_data(struct urb *urb, struct scsi_cmnd *cmnd,
    unsigned direction)
    {
    struct uas_cmd_info *cmdinfo = scsi_cmd_priv(cmnd);
    int err;
    cmdinfo.state |= direction | SUBMIT_STATUS_URB;
    err = uas_submit_urbs(cmnd, cmnd.device.hostdata);
    if (err) {
    uas_add_work(cmnd);
    }
    }
#[no_mangle]
unsafe extern "C" fn uas_evaluate_response_iu(riu: *mut response_iu, cmnd: *mut scsi_cmnd) -> bool {
    static bool uas_evaluate_response_iu(struct response_iu *riu, struct scsi_cmnd *cmnd)
    {
    let mut response_code: u8 = riu.response_code;
    switch (response_code) {
    case RC_INCORRECT_LUN:
    set_host_byte(cmnd, DID_BAD_TARGET);
    break;
    case RC_TMF_SUCCEEDED:
    set_host_byte(cmnd, DID_OK);
    break;
    case RC_TMF_NOT_SUPPORTED:
    set_host_byte(cmnd, DID_BAD_TARGET);
    break;
    default:
    uas_log_cmd_state(cmnd, "response iu", response_code);
    set_host_byte(cmnd, DID_ERROR);
    break;
    }
    let mut response_code: return = = RC_TMF_SUCCEEDED;
    }
#[no_mangle]
unsafe extern "C" fn uas_stat_cmplt(urb: *mut urb) {
    static void uas_stat_cmplt(struct urb *urb)
    {
    struct iu *iu = urb.transfer_buffer;
    struct Scsi_Host *shost = urb.context;
    struct uas_dev_info *devinfo = (struct uas_dev_info *)shost.hostdata;
    struct urb *data_in_urb = core::ptr::null_mut();
    struct urb *data_out_urb = core::ptr::null_mut();
    struct scsi_cmnd *cmnd;
    struct uas_cmd_info *cmdinfo;
    unsigned long flags;
    unsigned int idx;
    let mut status: c_int = urb.status;
    bool success;
    if (status) {
    if (status != -ENOENT && status != -ECONNRESET && status != -ESHUTDOWN)
    dev_err(&urb.dev.dev, "stat urb: status %d\n", status);
    goto bail;
    }
    idx = be16_to_cpup(&iu.tag) - 1;
    spin_lock_irqsave(&devinfo.lock, flags);
    if (devinfo.resetting)
    goto out;
    if (idx >= MAX_CMNDS || !devinfo.cmnd[idx]) {
    dev_err(&urb.dev.dev,
    "stat urb: no pending cmd for uas-tag %d\n", idx + 1);
    goto out;
    }
    cmnd = devinfo.cmnd[idx];
    cmdinfo = scsi_cmd_priv(cmnd);
    if (!(cmdinfo.state & COMMAND_INFLIGHT)) {
    uas_log_cmd_state(cmnd, "unexpected status cmplt", 0);
    goto out;
    }
    switch (iu.iu_id) {
    case IU_ID_STATUS:
    uas_sense(urb, cmnd);
    if (cmnd.result != 0) {
// cancel data transfers on error
    data_in_urb = usb_get_urb(cmdinfo.data_in_urb);
    data_out_urb = usb_get_urb(cmdinfo.data_out_urb);
    }
    cmdinfo.state &= ~COMMAND_INFLIGHT;
    uas_try_complete(cmnd, __func__);
    break;
    case IU_ID_READ_READY:
    if (!cmdinfo.data_in_urb ||
    (cmdinfo.state & DATA_IN_URB_INFLIGHT)) {
    uas_log_cmd_state(cmnd, "unexpected read rdy", 0);
    break;
    }
    uas_xfer_data(urb, cmnd, SUBMIT_DATA_IN_URB);
    break;
    case IU_ID_WRITE_READY:
    if (!cmdinfo.data_out_urb ||
    (cmdinfo.state & DATA_OUT_URB_INFLIGHT)) {
    uas_log_cmd_state(cmnd, "unexpected write rdy", 0);
    break;
    }
    uas_xfer_data(urb, cmnd, SUBMIT_DATA_OUT_URB);
    break;
    case IU_ID_RESPONSE:
    cmdinfo.state &= ~COMMAND_INFLIGHT;
    success = uas_evaluate_response_iu((struct response_iu *)iu, cmnd);
    if (!success) {
// Error, cancel data transfers
    data_in_urb = usb_get_urb(cmdinfo.data_in_urb);
    data_out_urb = usb_get_urb(cmdinfo.data_out_urb);
    }
    uas_try_complete(cmnd, __func__);
    break;
    default:
    uas_log_cmd_state(cmnd, "bogus IU", iu.iu_id);
    }
    spin_unlock_irqrestore(&devinfo.lock, flags);
    usb_free_urb(urb);
// Unlinking of data urbs must be done without holding the lock
    if (data_in_urb) {
    usb_unlink_urb(data_in_urb);
    usb_put_urb(data_in_urb);
    }
    if (data_out_urb) {
    usb_unlink_urb(data_out_urb);
    usb_put_urb(data_out_urb);
    }
    return;
    out:
    spin_unlock_irqrestore(&devinfo.lock, flags);
    bail:
    usb_free_urb(urb);
    }
#[no_mangle]
unsafe extern "C" fn uas_data_cmplt(urb: *mut urb) {
    static void uas_data_cmplt(struct urb *urb)
    {
    struct scsi_cmnd *cmnd = urb.context;
    struct uas_cmd_info *cmdinfo = scsi_cmd_priv(cmnd);
    struct uas_dev_info *devinfo = (void *)cmnd.device.hostdata;
    struct scsi_data_buffer *sdb = &cmnd.sdb;
    unsigned long flags;
    let mut status: c_int = urb.status;
    spin_lock_irqsave(&devinfo.lock, flags);
    if (cmdinfo.data_in_urb == urb) {
    cmdinfo.state &= ~DATA_IN_URB_INFLIGHT;
    cmdinfo.data_in_urb = core::ptr::null_mut();
    } else if (cmdinfo.data_out_urb == urb) {
    cmdinfo.state &= ~DATA_OUT_URB_INFLIGHT;
    cmdinfo.data_out_urb = core::ptr::null_mut();
    }
    if (devinfo.resetting)
    goto out;
// Data urbs should not complete before the cmd urb is submitted
    if (cmdinfo.state & SUBMIT_CMD_URB) {
    uas_log_cmd_state(cmnd, "unexpected data cmplt", 0);
    goto out;
    }
    if (status) {
    if (status != -ENOENT && status != -ECONNRESET && status != -ESHUTDOWN)
    uas_log_cmd_state(cmnd, "data cmplt err", status);
// error: no data transfered
    scsi_set_resid(cmnd, sdb.length);
    set_host_byte(cmnd, DID_ERROR);
    } else {
    scsi_set_resid(cmnd, sdb.length - urb.actual_length);
    }
    uas_try_complete(cmnd, __func__);
    out:
    spin_unlock_irqrestore(&devinfo.lock, flags);
    usb_free_urb(urb);
    }
#[no_mangle]
unsafe extern "C" fn uas_cmd_cmplt(urb: *mut urb) {
    static void uas_cmd_cmplt(struct urb *urb)
    {
    if (urb.status)
    dev_err(&urb.dev.dev, "cmd cmplt err %d\n", urb.status);
    usb_free_urb(urb);
    }
    static struct urb *uas_alloc_data_urb(struct uas_dev_info *devinfo, gfp_t gfp,
    struct scsi_cmnd *cmnd,
    enum dma_data_direction dir)
    {
    struct usb_device *udev = devinfo.udev;
    struct uas_cmd_info *cmdinfo = scsi_cmd_priv(cmnd);
    struct urb *urb = usb_alloc_urb(0, gfp);
    struct scsi_data_buffer *sdb = &cmnd.sdb;
    unsigned int pipe = (dir == DMA_FROM_DEVICE)
    ? devinfo.data_in_pipe : devinfo.data_out_pipe;
    if (!urb)
    goto out;
    usb_fill_bulk_urb(urb, udev, pipe, core::ptr::null_mut(), sdb.length,
    uas_data_cmplt, cmnd);
    if (devinfo.use_streams)
    urb.stream_id = cmdinfo.uas_tag;
    urb.num_sgs = udev.bus.sg_tablesize ? sdb.table.nents : 0;
    urb.sg = sdb.table.sgl;
    out:
    return urb;
    }
    static struct urb *uas_alloc_sense_urb(struct uas_dev_info *devinfo, gfp_t gfp,
    struct scsi_cmnd *cmnd)
    {
    struct usb_device *udev = devinfo.udev;
    struct uas_cmd_info *cmdinfo = scsi_cmd_priv(cmnd);
    struct urb *urb = usb_alloc_urb(0, gfp);
    struct sense_iu *iu;
    if (!urb)
    goto out;
    iu = kzalloc_obj(*iu, gfp);
    if (!iu)
    goto free;
    usb_fill_bulk_urb(urb, udev, devinfo.status_pipe, iu, sizeof(*iu),
    uas_stat_cmplt, cmnd.device.host);
    if (devinfo.use_streams)
    urb.stream_id = cmdinfo.uas_tag;
    urb.transfer_flags |= URB_FREE_BUFFER;
    out:
    return urb;
    free:
    usb_free_urb(urb);
    return core::ptr::null_mut();
    }
    static struct urb *uas_alloc_cmd_urb(struct uas_dev_info *devinfo, gfp_t gfp,
    struct scsi_cmnd *cmnd)
    {
    struct usb_device *udev = devinfo.udev;
    struct scsi_device *sdev = cmnd.device;
    struct uas_cmd_info *cmdinfo = scsi_cmd_priv(cmnd);
    struct urb *urb = usb_alloc_urb(0, gfp);
    struct command_iu *iu;
    int len;
    if (!urb)
    goto out;
    len = cmnd.cmd_len - 16;
    if (len < 0)
    len = 0;
    len = ALIGN(len, 4);
    iu = kzalloc(sizeof(*iu) + len, gfp);
    if (!iu)
    goto free;
    iu.iu_id = IU_ID_COMMAND;
    iu.tag = cpu_to_be16(cmdinfo.uas_tag);
    iu.prio_attr = UAS_SIMPLE_TAG;
    iu.len = len;
    int_to_scsilun(sdev.lun, &iu.lun);
    memcpy(iu.cdb, cmnd.cmnd, cmnd.cmd_len);
    usb_fill_bulk_urb(urb, udev, devinfo.cmd_pipe, iu, sizeof(*iu) + len,
    uas_cmd_cmplt, core::ptr::null_mut());
    urb.transfer_flags |= URB_FREE_BUFFER;
    out:
    return urb;
    free:
    usb_free_urb(urb);
    return core::ptr::null_mut();
    }
//
// Why should I request the Status IU before sending the Command IU?  Spec
// says to, but also says the device may receive them in any order.  Seems
// daft to me.
//
#[no_mangle]
unsafe extern "C" fn uas_submit_sense_urb(cmnd: *mut scsi_cmnd, gfp: gfp_t) -> c_int {
    static int uas_submit_sense_urb(struct scsi_cmnd *cmnd, gfp_t gfp)
    {
    struct uas_dev_info *devinfo = cmnd.device.hostdata;
    struct urb *urb;
    int err;
    urb = uas_alloc_sense_urb(devinfo, gfp, cmnd);
    if (!urb)
    return -ENOMEM;
    usb_anchor_urb(urb, &devinfo.sense_urbs);
    err = usb_submit_urb(urb, gfp);
    if (err) {
    usb_unanchor_urb(urb);
    uas_log_cmd_state(cmnd, "sense submit err", err);
    usb_free_urb(urb);
    }
    return err;
    }
    static int uas_submit_urbs(struct scsi_cmnd *cmnd,
    struct uas_dev_info *devinfo)
    {
    struct uas_cmd_info *cmdinfo = scsi_cmd_priv(cmnd);
    int err;
    lockdep_assert_held(&devinfo.lock);
    if (cmdinfo.state & SUBMIT_STATUS_URB) {
    err = uas_submit_sense_urb(cmnd, GFP_ATOMIC);
    if (err)
    return err;
    cmdinfo.state &= ~SUBMIT_STATUS_URB;
    }
    if (cmdinfo.state & ALLOC_DATA_IN_URB) {
    cmdinfo.data_in_urb = uas_alloc_data_urb(devinfo, GFP_ATOMIC,
    cmnd, DMA_FROM_DEVICE);
    if (!cmdinfo.data_in_urb)
    return -ENOMEM;
    cmdinfo.state &= ~ALLOC_DATA_IN_URB;
    }
    if (cmdinfo.state & SUBMIT_DATA_IN_URB) {
    usb_anchor_urb(cmdinfo.data_in_urb, &devinfo.data_urbs);
    err = usb_submit_urb(cmdinfo.data_in_urb, GFP_ATOMIC);
    if (err) {
    usb_unanchor_urb(cmdinfo.data_in_urb);
    uas_log_cmd_state(cmnd, "data in submit err", err);
    return err;
    }
    cmdinfo.state &= ~SUBMIT_DATA_IN_URB;
    cmdinfo.state |= DATA_IN_URB_INFLIGHT;
    }
    if (cmdinfo.state & ALLOC_DATA_OUT_URB) {
    cmdinfo.data_out_urb = uas_alloc_data_urb(devinfo, GFP_ATOMIC,
    cmnd, DMA_TO_DEVICE);
    if (!cmdinfo.data_out_urb)
    return -ENOMEM;
    cmdinfo.state &= ~ALLOC_DATA_OUT_URB;
    }
    if (cmdinfo.state & SUBMIT_DATA_OUT_URB) {
    usb_anchor_urb(cmdinfo.data_out_urb, &devinfo.data_urbs);
    err = usb_submit_urb(cmdinfo.data_out_urb, GFP_ATOMIC);
    if (err) {
    usb_unanchor_urb(cmdinfo.data_out_urb);
    uas_log_cmd_state(cmnd, "data out submit err", err);
    return err;
    }
    cmdinfo.state &= ~SUBMIT_DATA_OUT_URB;
    cmdinfo.state |= DATA_OUT_URB_INFLIGHT;
    }
    if (cmdinfo.state & ALLOC_CMD_URB) {
    cmdinfo.cmd_urb = uas_alloc_cmd_urb(devinfo, GFP_ATOMIC, cmnd);
    if (!cmdinfo.cmd_urb)
    return -ENOMEM;
    cmdinfo.state &= ~ALLOC_CMD_URB;
    }
    if (cmdinfo.state & SUBMIT_CMD_URB) {
    usb_anchor_urb(cmdinfo.cmd_urb, &devinfo.cmd_urbs);
    err = usb_submit_urb(cmdinfo.cmd_urb, GFP_ATOMIC);
    if (err) {
    usb_unanchor_urb(cmdinfo.cmd_urb);
    uas_log_cmd_state(cmnd, "cmd submit err", err);
    return err;
    }
    cmdinfo.cmd_urb = core::ptr::null_mut();
    cmdinfo.state &= ~SUBMIT_CMD_URB;
    cmdinfo.state |= COMMAND_INFLIGHT;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uas_queuecommand_lck(cmnd: *mut scsi_cmnd) -> enum scsi_qc_status {
    static enum scsi_qc_status uas_queuecommand_lck(struct scsi_cmnd *cmnd)
    {
    struct scsi_device *sdev = cmnd.device;
    struct uas_dev_info *devinfo = sdev.hostdata;
    struct uas_cmd_info *cmdinfo = scsi_cmd_priv(cmnd);
    unsigned long flags;
    int idx, err;
// Re-check scsi_block_requests now that we've the host-lock
    if (cmnd.device.host.host_self_blocked)
    return SCSI_MLQUEUE_DEVICE_BUSY;
    if ((devinfo.flags & US_FL_NO_ATA_1X) &&
    (cmnd.cmnd[0] == ATA_12 || cmnd.cmnd[0] == ATA_16)) {
    memcpy(cmnd.sense_buffer, usb_stor_sense_invalidCDB,
    sizeof(usb_stor_sense_invalidCDB));
    cmnd.result = SAM_STAT_CHECK_CONDITION;
    scsi_done(cmnd);
    return 0;
    }
    spin_lock_irqsave(&devinfo.lock, flags);
    if (devinfo.resetting) {
    set_host_byte(cmnd, DID_ERROR);
    scsi_done(cmnd);
    goto zombie;
    }
// Find a free uas-tag
    for (idx = 0; idx < devinfo.qdepth; idx++) {
    if (!devinfo.cmnd[idx])
    break;
    }
    if (idx == devinfo.qdepth) {
    spin_unlock_irqrestore(&devinfo.lock, flags);
    return SCSI_MLQUEUE_DEVICE_BUSY;
    }
    memset(cmdinfo, 0, sizeof(*cmdinfo));
    cmdinfo.uas_tag = idx + 1; /* uas-tag == usb-stream-id, so 1 based */
    cmdinfo.state = SUBMIT_STATUS_URB | ALLOC_CMD_URB | SUBMIT_CMD_URB;
    switch (cmnd.sc_data_direction) {
    case DMA_FROM_DEVICE:
    cmdinfo.state |= ALLOC_DATA_IN_URB | SUBMIT_DATA_IN_URB;
    break;
    case DMA_BIDIRECTIONAL:
    cmdinfo.state |= ALLOC_DATA_IN_URB | SUBMIT_DATA_IN_URB;
    fallthrough;
    case DMA_TO_DEVICE:
    cmdinfo.state |= ALLOC_DATA_OUT_URB | SUBMIT_DATA_OUT_URB;
    break;
    case DMA_NONE:
    break;
    }
    if (!devinfo.use_streams)
    cmdinfo.state &= ~(SUBMIT_DATA_IN_URB | SUBMIT_DATA_OUT_URB);
    err = uas_submit_urbs(cmnd, devinfo);
//
// in case of fatal errors the SCSI layer is peculiar
// a command that has finished is a success for the purpose
// of queueing, no matter how fatal the error
//
    if (err == -ENODEV) {
    if (cmdinfo.state & (COMMAND_INFLIGHT | DATA_IN_URB_INFLIGHT |
    DATA_OUT_URB_INFLIGHT))
    goto out;
    set_host_byte(cmnd, DID_NO_CONNECT);
    scsi_done(cmnd);
    goto zombie;
    }
    if (err) {
// If we did nothing, give up now
    if (cmdinfo.state & SUBMIT_STATUS_URB) {
    spin_unlock_irqrestore(&devinfo.lock, flags);
    return SCSI_MLQUEUE_DEVICE_BUSY;
    }
    uas_add_work(cmnd);
    }
    out:
    devinfo.cmnd[idx] = cmnd;
    zombie:
    spin_unlock_irqrestore(&devinfo.lock, flags);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn DEF_SCSI_QCMD(_arg: uas_queuecommand) -> static {
    static DEF_SCSI_QCMD(uas_queuecommand)
//
// For now we do not support actually sending an abort to the device, so
// this eh always fails. Still we must define it to make sure that we've
// dropped all references to the cmnd in question once this function exits.
//
#[no_mangle]
unsafe extern "C" fn uas_eh_abort_handler(cmnd: *mut scsi_cmnd) -> c_int {
    static int uas_eh_abort_handler(struct scsi_cmnd *cmnd)
    {
    struct uas_cmd_info *cmdinfo = scsi_cmd_priv(cmnd);
    struct uas_dev_info *devinfo = (void *)cmnd.device.hostdata;
    struct urb *data_in_urb = core::ptr::null_mut();
    struct urb *data_out_urb = core::ptr::null_mut();
    unsigned long flags;
    spin_lock_irqsave(&devinfo.lock, flags);
    uas_log_cmd_state(cmnd, __func__, 0);
// Ensure that try_complete does not call scsi_done
    cmdinfo.state |= COMMAND_ABORTED;
// Drop all refs to this cmnd, kill data urbs to break their ref
    devinfo.cmnd[cmdinfo.uas_tag - 1] = core::ptr::null_mut();
    if (cmdinfo.state & DATA_IN_URB_INFLIGHT)
    data_in_urb = usb_get_urb(cmdinfo.data_in_urb);
    if (cmdinfo.state & DATA_OUT_URB_INFLIGHT)
    data_out_urb = usb_get_urb(cmdinfo.data_out_urb);
    uas_free_unsubmitted_urbs(cmnd);
    spin_unlock_irqrestore(&devinfo.lock, flags);
    if (data_in_urb) {
    usb_kill_urb(data_in_urb);
    usb_put_urb(data_in_urb);
    }
    if (data_out_urb) {
    usb_kill_urb(data_out_urb);
    usb_put_urb(data_out_urb);
    }
    return FAILED;
    }
#[no_mangle]
unsafe extern "C" fn uas_eh_host_reset_handler(cmnd: *mut scsi_cmnd) -> c_int {
    static int uas_eh_host_reset_handler(struct scsi_cmnd *cmnd)
    {
    struct scsi_device *sdev = cmnd.device;
    struct uas_dev_info *devinfo = sdev.hostdata;
    struct usb_device *udev = devinfo.udev;
    unsigned long flags;
    int err;
    err = usb_lock_device_for_reset(udev, devinfo.intf);
    if (err) {
    shost_printk(KERN_ERR, sdev.host,
    "%s FAILED to get lock err %d\n", __func__, err);
    return FAILED;
    }
    shost_printk(KERN_INFO, sdev.host, "%s start\n", __func__);
    spin_lock_irqsave(&devinfo.lock, flags);
    devinfo.resetting = 1;
    spin_unlock_irqrestore(&devinfo.lock, flags);
    usb_kill_anchored_urbs(&devinfo.cmd_urbs);
    usb_kill_anchored_urbs(&devinfo.sense_urbs);
    usb_kill_anchored_urbs(&devinfo.data_urbs);
    uas_zap_pending(devinfo, DID_RESET);
    err = usb_reset_device(udev);
    spin_lock_irqsave(&devinfo.lock, flags);
    devinfo.resetting = 0;
    spin_unlock_irqrestore(&devinfo.lock, flags);
    usb_unlock_device(udev);
    if (err) {
    shost_printk(KERN_INFO, sdev.host, "%s FAILED err %d\n",
    __func__, err);
    return FAILED;
    }
    shost_printk(KERN_INFO, sdev.host, "%s success\n", __func__);
    return SUCCESS;
    }
#[no_mangle]
unsafe extern "C" fn uas_target_alloc(starget: *mut scsi_target) -> c_int {
    static int uas_target_alloc(struct scsi_target *starget)
    {
    struct uas_dev_info *devinfo = (struct uas_dev_info *)
    dev_to_shost(starget.dev.parent).hostdata;
    if (devinfo.flags & US_FL_NO_REPORT_LUNS)
    starget.no_report_luns = 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uas_sdev_init(sdev: *mut scsi_device) -> c_int {
    static int uas_sdev_init(struct scsi_device *sdev)
    {
    struct uas_dev_info *devinfo =
    (struct uas_dev_info *)sdev.host.hostdata;
//
// Some USB storage devices reset if the IO advice hints grouping mode
// page is queried. Hence skip that mode page.
//
    sdev.sdev_bflags |= BLIST_SKIP_IO_HINTS;
    sdev.hostdata = devinfo;
    return 0;
    }
    static int uas_sdev_configure(struct scsi_device *sdev,
    struct queue_limits *lim)
    {
    struct uas_dev_info *devinfo = sdev.hostdata;
    if (devinfo.flags & US_FL_MAX_SECTORS_64)
    lim.max_hw_sectors = 64;
#[no_mangle]
pub unsafe extern "C" fn if(US_FL_MAX_SECTORS_240: devinfo->flags &) -> else {
    else if (devinfo.flags & US_FL_MAX_SECTORS_240)
    lim.max_hw_sectors = 240;
    if (devinfo.flags & US_FL_NO_REPORT_OPCODES)
    sdev.no_report_opcodes = 1;
// A few buggy USB-ATA bridges don't understand FUA
    if (devinfo.flags & US_FL_BROKEN_FUA)
    sdev.broken_fua = 1;
// UAS also needs to support FL_ALWAYS_SYNC
    if (devinfo.flags & US_FL_ALWAYS_SYNC) {
    sdev.skip_ms_page_3f = 1;
    sdev.skip_ms_page_8 = 1;
    sdev.wce_default_on = 1;
    }
// Some disks cannot handle READ_CAPACITY_16
    if (devinfo.flags & US_FL_NO_READ_CAPACITY_16)
    sdev.no_read_capacity_16 = 1;
// Some disks cannot handle WRITE_SAME
    if (devinfo.flags & US_FL_NO_SAME)
    sdev.no_write_same = 1;
//
// Some disks return the total number of blocks in response
// to READ CAPACITY rather than the highest block number.
// If this device makes that mistake, tell the sd driver.
//
    if (devinfo.flags & US_FL_FIX_CAPACITY)
    sdev.fix_capacity = 1;
//
// in some cases we have to guess
//
    if (devinfo.flags & US_FL_CAPACITY_HEURISTICS)
    sdev.guess_capacity = 1;
//
// Some devices report generic values until the media has been
// accessed. Force a READ(10) prior to querying device
// characteristics.
//
    sdev.read_before_ms = 1;
//
// Some devices don't like MODE SENSE with page=0x3f,
// which is the command used for checking if a device
// is write-protected.  Now that we tell the sd driver
// to do a 192-byte transfer with this command the
// majority of devices work fine, but a few still can't
// handle it.  The sd driver will simply assume those
// devices are write-enabled.
//
    if (devinfo.flags & US_FL_NO_WP_DETECT)
    sdev.skip_ms_page_3f = 1;
    scsi_change_queue_depth(sdev, devinfo.qdepth - 2);
    return 0;
    }
    static const struct scsi_host_template uas_host_template = {
    .module = THIS_MODULE,
    .name = "uas",
    .queuecommand = uas_queuecommand,
    .target_alloc = uas_target_alloc,
    .sdev_init = uas_sdev_init,
    .sdev_configure = uas_sdev_configure,
    .eh_abort_handler = uas_eh_abort_handler,
    .eh_host_reset_handler = uas_eh_host_reset_handler,
    .this_id = -1,
    .skip_settle_delay = 1,
//
// The protocol has no requirements on alignment in the strict sense.
// Controllers may or may not have alignment restrictions.
// As this is not exported, we use an extremely conservative guess.
//
    .dma_alignment = 511,
    .dma_boundary = PAGE_SIZE - 1,
    .cmd_size = sizeof(struct uas_cmd_info),
    };

    vendorName, productName, useProtocol, useTransport, \
    initFunction, flags) \
    { USB_DEVICE_VER(id_vendor, id_product, bcdDeviceMin, bcdDeviceMax), \
    .driver_info = (flags) }
    static const struct usb_device_id uas_usb_ids[] = {

    { USB_INTERFACE_INFO(USB_CLASS_MASS_STORAGE, USB_SC_SCSI, USB_PR_BULK) },
    { USB_INTERFACE_INFO(USB_CLASS_MASS_STORAGE, USB_SC_SCSI, USB_PR_UAS) },
    { }
    };
    MODULE_DEVICE_TABLE(usb, uas_usb_ids);

    static int uas_switch_interface(struct usb_device *udev,
    struct usb_interface *intf)
    {
    struct usb_host_interface *alt;
    alt = uas_find_uas_alt_setting(intf);
    if (!alt)
    return -ENODEV;
    return usb_set_interface(udev, alt.desc.bInterfaceNumber,
    alt.desc.bAlternateSetting);
    }
#[no_mangle]
unsafe extern "C" fn uas_configure_endpoints(devinfo: *mut uas_dev_info) -> c_int {
    static int uas_configure_endpoints(struct uas_dev_info *devinfo)
    {
    struct usb_host_endpoint *eps[4] = { };
    struct usb_device *udev = devinfo.udev;
    int r;
    r = uas_find_endpoints(devinfo.intf.cur_altsetting, eps);
    if (r)
    return r;
    devinfo.cmd_pipe = usb_sndbulkpipe(udev,
    usb_endpoint_num(&eps[0].desc));
    devinfo.status_pipe = usb_rcvbulkpipe(udev,
    usb_endpoint_num(&eps[1].desc));
    devinfo.data_in_pipe = usb_rcvbulkpipe(udev,
    usb_endpoint_num(&eps[2].desc));
    devinfo.data_out_pipe = usb_sndbulkpipe(udev,
    usb_endpoint_num(&eps[3].desc));
    if (udev.speed < USB_SPEED_SUPER) {
    devinfo.qdepth = 32;
    devinfo.use_streams = 0;
    } else {
    devinfo.qdepth = usb_alloc_streams(devinfo.intf, eps + 1,
    3, MAX_CMNDS, GFP_NOIO);
    if (devinfo.qdepth < 0)
    return devinfo.qdepth;
    devinfo.use_streams = 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uas_free_streams(devinfo: *mut uas_dev_info) {
    static void uas_free_streams(struct uas_dev_info *devinfo)
    {
    struct usb_device *udev = devinfo.udev;
    struct usb_host_endpoint *eps[3];
    eps[0] = usb_pipe_endpoint(udev, devinfo.status_pipe);
    eps[1] = usb_pipe_endpoint(udev, devinfo.data_in_pipe);
    eps[2] = usb_pipe_endpoint(udev, devinfo.data_out_pipe);
    usb_free_streams(devinfo.intf, eps, 3, GFP_NOIO);
    }
#[no_mangle]
unsafe extern "C" fn uas_probe(intf: *mut usb_interface, id: *const usb_device_id) -> c_int {
    static int uas_probe(struct usb_interface *intf, const struct usb_device_id *id)
    {
    let mut result: c_int = -ENOMEM;
    struct Scsi_Host *shost = core::ptr::null_mut();
    struct uas_dev_info *devinfo;
    struct usb_device *udev = interface_to_usbdev(intf);
    u64 dev_flags;
    if (!uas_use_uas_driver(intf, id, &dev_flags))
    return -ENODEV;
    if (uas_switch_interface(udev, intf))
    return -ENODEV;
    shost = scsi_host_alloc(&uas_host_template,
    sizeof(struct uas_dev_info));
    if (!shost)
    goto set_alt0;
    shost.max_cmd_len = 16 + 252;
    shost.max_id = 1;
    shost.max_lun = 256;
    shost.max_channel = 0;
    shost.sg_tablesize = udev.bus.sg_tablesize;
    devinfo = (struct uas_dev_info *)shost.hostdata;
    devinfo.intf = intf;
    devinfo.udev = udev;
    devinfo.resetting = 0;
    devinfo.shutdown = 0;
    devinfo.flags = dev_flags;
    init_usb_anchor(&devinfo.cmd_urbs);
    init_usb_anchor(&devinfo.sense_urbs);
    init_usb_anchor(&devinfo.data_urbs);
    spin_lock_init(&devinfo.lock);
    INIT_WORK(&devinfo.work, uas_do_work);
    INIT_WORK(&devinfo.scan_work, uas_scan_work);
    result = uas_configure_endpoints(devinfo);
    if (result)
    goto set_alt0;
//
// 1 tag is reserved for untagged commands +
// 1 tag to avoid off by one errors in some bridge firmwares
//
    shost.can_queue = devinfo.qdepth - 2;
    usb_set_intfdata(intf, shost);
    result = scsi_add_host(shost, &intf.dev);
    if (result)
    goto free_streams;
// Submit the delayed_work for SCSI-device scanning
    schedule_work(&devinfo.scan_work);
    return result;
    free_streams:
    uas_free_streams(devinfo);
    usb_set_intfdata(intf, core::ptr::null_mut());
    set_alt0:
    usb_set_interface(udev, intf.altsetting[0].desc.bInterfaceNumber, 0);
    if (shost)
    scsi_host_put(shost);
    return result;
    }
#[no_mangle]
unsafe extern "C" fn uas_cmnd_list_empty(devinfo: *mut uas_dev_info) -> c_int {
    static int uas_cmnd_list_empty(struct uas_dev_info *devinfo)
    {
    unsigned long flags;
    int i, r = 1;
    spin_lock_irqsave(&devinfo.lock, flags);
    for (i = 0; i < devinfo.qdepth; i++) {
    if (devinfo.cmnd[i]) {
    r = 0; /* Not empty */
    break;
    }
    }
    spin_unlock_irqrestore(&devinfo.lock, flags);
    return r;
    }
//
// Wait for any pending cmnds to complete, on usb-2 sense_urbs may temporarily
// get empty while there still is more work to do due to sense-urbs completing
// with a READ/WRITE_READY iu code, so keep waiting until the list gets empty.
//
#[no_mangle]
unsafe extern "C" fn uas_wait_for_pending_cmnds(devinfo: *mut uas_dev_info) -> c_int {
    static int uas_wait_for_pending_cmnds(struct uas_dev_info *devinfo)
    {
    unsigned long start_time;
    int r;
    start_time = jiffies;
    do {
    flush_work(&devinfo.work);
    r = usb_wait_anchor_empty_timeout(&devinfo.sense_urbs, 5000);
    if (r == 0)
    return -ETIME;
    r = usb_wait_anchor_empty_timeout(&devinfo.data_urbs, 500);
    if (r == 0)
    return -ETIME;
    if (time_after(jiffies, start_time + 5 * HZ))
    return -ETIME;
    } while (!uas_cmnd_list_empty(devinfo));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uas_pre_reset(intf: *mut usb_interface) -> c_int {
    static int uas_pre_reset(struct usb_interface *intf)
    {
    struct Scsi_Host *shost = usb_get_intfdata(intf);
    struct uas_dev_info *devinfo = (struct uas_dev_info *)shost.hostdata;
    unsigned long flags;
    if (devinfo.shutdown)
    return 0;
// Block new requests
    spin_lock_irqsave(shost.host_lock, flags);
    scsi_block_requests(shost);
    spin_unlock_irqrestore(shost.host_lock, flags);
    if (uas_wait_for_pending_cmnds(devinfo) != 0) {
    shost_printk(KERN_ERR, shost, "%s: timed out\n", __func__);
    scsi_unblock_requests(shost);
    return 1;
    }
    uas_free_streams(devinfo);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uas_post_reset(intf: *mut usb_interface) -> c_int {
    static int uas_post_reset(struct usb_interface *intf)
    {
    struct Scsi_Host *shost = usb_get_intfdata(intf);
    struct uas_dev_info *devinfo = (struct uas_dev_info *)shost.hostdata;
    unsigned long flags;
    int err;
    if (devinfo.shutdown)
    return 0;
    err = uas_configure_endpoints(devinfo);
    if (err && err != -ENODEV)
    shost_printk(KERN_ERR, shost,
    "%s: alloc streams error %d after reset",
    __func__, err);
// we must unblock the host in every case lest we deadlock
    spin_lock_irqsave(shost.host_lock, flags);
    scsi_report_bus_reset(shost, 0);
    spin_unlock_irqrestore(shost.host_lock, flags);
    scsi_unblock_requests(shost);
    return err ? 1 : 0;
    }
#[no_mangle]
unsafe extern "C" fn uas_suspend(intf: *mut usb_interface, message: pm_message_t) -> c_int {
    static int uas_suspend(struct usb_interface *intf, pm_message_t message)
    {
    struct Scsi_Host *shost = usb_get_intfdata(intf);
    struct uas_dev_info *devinfo = (struct uas_dev_info *)shost.hostdata;
    if (uas_wait_for_pending_cmnds(devinfo) != 0) {
    shost_printk(KERN_ERR, shost, "%s: timed out\n", __func__);
    return -ETIME;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uas_resume(intf: *mut usb_interface) -> c_int {
    static int uas_resume(struct usb_interface *intf)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uas_reset_resume(intf: *mut usb_interface) -> c_int {
    static int uas_reset_resume(struct usb_interface *intf)
    {
    struct Scsi_Host *shost = usb_get_intfdata(intf);
    struct uas_dev_info *devinfo = (struct uas_dev_info *)shost.hostdata;
    unsigned long flags;
    int err;
    err = uas_configure_endpoints(devinfo);
    if (err) {
    shost_printk(KERN_ERR, shost,
    "%s: alloc streams error %d after reset",
    __func__, err);
    return -EIO;
    }
    spin_lock_irqsave(shost.host_lock, flags);
    scsi_report_bus_reset(shost, 0);
    spin_unlock_irqrestore(shost.host_lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uas_disconnect(intf: *mut usb_interface) {
    static void uas_disconnect(struct usb_interface *intf)
    {
    struct Scsi_Host *shost = usb_get_intfdata(intf);
    struct uas_dev_info *devinfo = (struct uas_dev_info *)shost.hostdata;
    unsigned long flags;
    spin_lock_irqsave(&devinfo.lock, flags);
    devinfo.resetting = 1;
    spin_unlock_irqrestore(&devinfo.lock, flags);
    cancel_work_sync(&devinfo.work);
    usb_kill_anchored_urbs(&devinfo.cmd_urbs);
    usb_kill_anchored_urbs(&devinfo.sense_urbs);
    usb_kill_anchored_urbs(&devinfo.data_urbs);
    uas_zap_pending(devinfo, DID_NO_CONNECT);
//
// Prevent SCSI scanning (if it hasn't started yet)
// or wait for the SCSI-scanning routine to stop.
//
    cancel_work_sync(&devinfo.scan_work);
    scsi_remove_host(shost);
    uas_free_streams(devinfo);
    scsi_host_put(shost);
    }
//
// Put the device back in usb-storage mode on shutdown, as some BIOS-es
// hang on reboot when the device is still in uas mode. Note the reset is
// necessary as some devices won't revert to usb-storage mode without it.
//
#[no_mangle]
unsafe extern "C" fn uas_shutdown(intf: *mut usb_interface) {
    static void uas_shutdown(struct usb_interface *intf)
    {
    struct usb_device *udev = interface_to_usbdev(intf);
    struct Scsi_Host *shost = usb_get_intfdata(intf);
    struct uas_dev_info *devinfo = (struct uas_dev_info *)shost.hostdata;
    if (system_state != SYSTEM_RESTART)
    return;
    devinfo.shutdown = 1;
    uas_free_streams(devinfo);
    usb_set_interface(udev, intf.altsetting[0].desc.bInterfaceNumber, 0);
    usb_reset_device(udev);
    }
    static struct usb_driver uas_driver = {
    .name = "uas",
    .probe = uas_probe,
    .disconnect = uas_disconnect,
    .pre_reset = uas_pre_reset,
    .post_reset = uas_post_reset,
    .suspend = uas_suspend,
    .resume = uas_resume,
    .reset_resume = uas_reset_resume,
    .shutdown = uas_shutdown,
    .id_table = uas_usb_ids,
    };
#[no_mangle]
unsafe extern "C" fn uas_init() -> int __init {
    static int __init uas_init(void)
    {
    int rv;
    workqueue = alloc_workqueue("uas", WQ_MEM_RECLAIM | WQ_PERCPU, 0);
    if (!workqueue)
    return -ENOMEM;
    rv = usb_register(&uas_driver);
    if (rv) {
    destroy_workqueue(workqueue);
    return -ENOMEM;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uas_exit() -> void __exit {
    static void __exit uas_exit(void)
    {
    usb_deregister(&uas_driver);
    destroy_workqueue(workqueue);
    }
    module_init(uas_init);
    module_exit(uas_exit);
    MODULE_DESCRIPTION("USB Attached SCSI driver");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("USB_STORAGE");
    MODULE_AUTHOR(
    "Hans de Goede <hdegoede@redhat.com>, Matthew Wilcox and Sarah Sharp");
