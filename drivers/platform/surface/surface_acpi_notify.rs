//! Automatically rewritten from C to Rust
//! Source: drivers/platform/surface/surface_acpi_notify.c
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
//
// Driver for the Surface ACPI Notify (SAN) interface/shim.
//
// Translates communication from ACPI to Surface System Aggregator Module
// (SSAM/SAM) requests and back, specifically SAM-over-SSH. Translates SSAM
// events back to ACPI notifications. Allows handling of discrete GPU
// notifications sent from ACPI via the SAN interface by providing them to any
// registered external driver.
//
// Copyright (C) 2019-2022 Maximilian Luz <luzmaximilian@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct san_data {
    pub dev: *mut device,
    pub ctrl: *mut ssam_controller,
    pub info: acpi_connection_info,
    pub nf_bat: ssam_event_notifier,
    pub nf_tmp: ssam_event_notifier,
}

    container_of(ptr, struct san_data, member)
    static struct workqueue_struct *san_wq;
// -- dGPU notifier interface. ----------------------------------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct san_rqsg_if {
    pub lock: rw_semaphore,
    pub dev: *mut device,
    pub nh: blocking_notifier_head,
}

    static struct san_rqsg_if san_rqsg_if = {
    .lock = __RWSEM_INITIALIZER(san_rqsg_if.lock),
    .dev = core::ptr::null_mut(),
    .nh = BLOCKING_NOTIFIER_INIT(san_rqsg_if.nh),
    };
#[no_mangle]
unsafe extern "C" fn san_set_rqsg_interface_device(dev: *mut device) -> c_int {
    static int san_set_rqsg_interface_device(struct device *dev)
    {
    let mut status: c_int = 0;
    down_write(&san_rqsg_if.lock);
    if (!san_rqsg_if.dev && dev)
    san_rqsg_if.dev = dev;
    else
    status = -EBUSY;
    up_write(&san_rqsg_if.lock);
    return status;
    }
//
// san_client_link() - Link client as consumer to SAN device.
// @client: The client to link.
//
// Sets up a device link between the provided client device as consumer and
// the SAN device as provider. This function can be used to ensure that the
// SAN interface has been set up and will be set up for as long as the driver
// of the client device is bound. This guarantees that, during that time, all
// dGPU events will be received by any registered notifier.
//
// The link will be automatically removed once the client device's driver is
// unbound.
//
// Return: Returns zero on success, %-ENXIO if the SAN interface has not been
// set up yet, and %-ENOMEM if device link creation failed.
//
#[no_mangle]
pub unsafe extern "C" fn san_client_link(client: *mut device) -> c_int {
    int san_client_link(struct device *client)
    {
    let mut flags: u32 = DL_FLAG_PM_RUNTIME | DL_FLAG_AUTOREMOVE_CONSUMER;
    struct device_link *link;
    down_read(&san_rqsg_if.lock);
    if (!san_rqsg_if.dev) {
    up_read(&san_rqsg_if.lock);
    return -ENXIO;
    }
    link = device_link_add(client, san_rqsg_if.dev, flags);
    if (!link) {
    up_read(&san_rqsg_if.lock);
    return -ENOMEM;
    }
    if (READ_ONCE(link.status) == DL_STATE_SUPPLIER_UNBIND) {
    up_read(&san_rqsg_if.lock);
    return -ENXIO;
    }
    up_read(&san_rqsg_if.lock);
    return 0;
    }
    EXPORT_SYMBOL_GPL(san_client_link);
//
// san_dgpu_notifier_register() - Register a SAN dGPU notifier.
// @nb: The notifier-block to register.
//
// Registers a SAN dGPU notifier, receiving any new SAN dGPU events sent from
// ACPI. The registered notifier will be called with &struct san_dgpu_event
// as notifier data and the command ID of that event as notifier action.
//
#[no_mangle]
pub unsafe extern "C" fn san_dgpu_notifier_register(nb: *mut notifier_block) -> c_int {
    int san_dgpu_notifier_register(struct notifier_block *nb)
    {
    return blocking_notifier_chain_register(&san_rqsg_if.nh, nb);
    }
    EXPORT_SYMBOL_GPL(san_dgpu_notifier_register);
//
// san_dgpu_notifier_unregister() - Unregister a SAN dGPU notifier.
// @nb: The notifier-block to unregister.
//
#[no_mangle]
pub unsafe extern "C" fn san_dgpu_notifier_unregister(nb: *mut notifier_block) -> c_int {
    int san_dgpu_notifier_unregister(struct notifier_block *nb)
    {
    return blocking_notifier_chain_unregister(&san_rqsg_if.nh, nb);
    }
    EXPORT_SYMBOL_GPL(san_dgpu_notifier_unregister);
#[no_mangle]
unsafe extern "C" fn san_dgpu_notifier_call(evt: *mut san_dgpu_event) -> c_int {
    static int san_dgpu_notifier_call(struct san_dgpu_event *evt)
    {
    int ret;
    ret = blocking_notifier_call_chain(&san_rqsg_if.nh, evt.command, evt);
    return notifier_to_errno(ret);
    }
// -- ACPI _DSM event relay. ------------------------------------------------
pub const SAN_DSM_REVISION: c_int = 0;
// 93b666c5-70c6-469f-a215-3d487c91ab3c
    static const guid_t SAN_DSM_UUID =
    GUID_INIT(0x93b666c5, 0x70c6, 0x469f, 0xa2, 0x15, 0x3d,
    0x48, 0x7c, 0x91, 0xab, 0x3c);
    enum san_dsm_event_fn {
    SAN_DSM_EVENT_FN_BAT1_STAT = 0x03,
    SAN_DSM_EVENT_FN_BAT1_INFO = 0x04,
    SAN_DSM_EVENT_FN_ADP1_STAT = 0x05,
    SAN_DSM_EVENT_FN_ADP1_INFO = 0x06,
    SAN_DSM_EVENT_FN_BAT2_STAT = 0x07,
    SAN_DSM_EVENT_FN_BAT2_INFO = 0x08,
    SAN_DSM_EVENT_FN_THERMAL   = 0x09,
    SAN_DSM_EVENT_FN_DPTF      = 0x0a,
    };
    enum sam_event_cid_bat {
    SAM_EVENT_CID_BAT_BIX  = 0x15,
    SAM_EVENT_CID_BAT_BST  = 0x16,
    SAM_EVENT_CID_BAT_ADP  = 0x17,
    SAM_EVENT_CID_BAT_PROT = 0x18,
    SAM_EVENT_CID_BAT_DPTF = 0x4f,
    };
    enum sam_event_cid_tmp {
    SAM_EVENT_CID_TMP_TRIP = 0x0b,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct san_event_work {
    pub work: delayed_work,
    pub dev: *mut device,
    pub /: *mut *mut ssam_event event; / must be last,
}

    static int san_acpi_notify_event(struct device *dev, u64 func,
    union acpi_object *param)
    {
    let mut san: acpi_handle = ACPI_HANDLE(dev);
    union acpi_object *obj;
    let mut status: c_int = 0;
    if (!acpi_check_dsm(san, &SAN_DSM_UUID, SAN_DSM_REVISION, BIT_ULL(func)))
    return 0;
    dev_dbg(dev, "notify event %#04llx\n", func);
    obj = acpi_evaluate_dsm_typed(san, &SAN_DSM_UUID, SAN_DSM_REVISION,
    func, param, ACPI_TYPE_BUFFER);
    if (!obj)
    return -EFAULT;
    if (obj.buffer.length != 1 || obj.buffer.pointer[0] != 0) {
    dev_err(dev, "got unexpected result from _DSM\n");
    status = -EPROTO;
    }
    ACPI_FREE(obj);
    return status;
    }
#[no_mangle]
unsafe extern "C" fn san_evt_bat_adp(dev: *mut device, event: *const ssam_event) -> c_int {
    static int san_evt_bat_adp(struct device *dev, const struct ssam_event *event)
    {
    int status;
    status = san_acpi_notify_event(dev, SAN_DSM_EVENT_FN_ADP1_STAT, core::ptr::null_mut());
    if (status)
    return status;
//
// Ensure that the battery states get updated correctly. When the
// battery is fully charged and an adapter is plugged in, it sometimes
// is not updated correctly, instead showing it as charging.
// Explicitly trigger battery updates to fix this.
//
    status = san_acpi_notify_event(dev, SAN_DSM_EVENT_FN_BAT1_STAT, core::ptr::null_mut());
    if (status)
    return status;
    return san_acpi_notify_event(dev, SAN_DSM_EVENT_FN_BAT2_STAT, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn san_evt_bat_bix(dev: *mut device, event: *const ssam_event) -> c_int {
    static int san_evt_bat_bix(struct device *dev, const struct ssam_event *event)
    {
    enum san_dsm_event_fn fn;
    if (event.instance_id == 0x02)
    fn = SAN_DSM_EVENT_FN_BAT2_INFO;
    else
    fn = SAN_DSM_EVENT_FN_BAT1_INFO;
    return san_acpi_notify_event(dev, fn, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn san_evt_bat_bst(dev: *mut device, event: *const ssam_event) -> c_int {
    static int san_evt_bat_bst(struct device *dev, const struct ssam_event *event)
    {
    enum san_dsm_event_fn fn;
    if (event.instance_id == 0x02)
    fn = SAN_DSM_EVENT_FN_BAT2_STAT;
    else
    fn = SAN_DSM_EVENT_FN_BAT1_STAT;
    return san_acpi_notify_event(dev, fn, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn san_evt_bat_dptf(dev: *mut device, event: *const ssam_event) -> c_int {
    static int san_evt_bat_dptf(struct device *dev, const struct ssam_event *event)
    {
    union acpi_object payload;
//
// The Surface ACPI expects a buffer and not a package. It specifically
// checks for ObjectType (Arg3) == 0x03. This will cause a warning in
// acpica/nsarguments.c, but that warning can be safely ignored.
//
    payload.type = ACPI_TYPE_BUFFER;
    payload.buffer.length = event.length;
    payload.buffer.pointer = (u8 *)&event.data[0];
    return san_acpi_notify_event(dev, SAN_DSM_EVENT_FN_DPTF, &payload);
    }
#[no_mangle]
unsafe extern "C" fn san_evt_bat_delay(cid: u8) -> c_ulong {
    static unsigned long san_evt_bat_delay(u8 cid)
    {
    switch (cid) {
    case SAM_EVENT_CID_BAT_ADP:
//
// Wait for battery state to update before signaling adapter
// change.
//
    return msecs_to_jiffies(5000);
    case SAM_EVENT_CID_BAT_BST:
// Ensure we do not miss anything important due to caching.
    return msecs_to_jiffies(2000);
    default:
    return 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn san_evt_bat(event: *const ssam_event, dev: *mut device) -> bool {
    static bool san_evt_bat(const struct ssam_event *event, struct device *dev)
    {
    int status;
    switch (event.command_id) {
    case SAM_EVENT_CID_BAT_BIX:
    status = san_evt_bat_bix(dev, event);
    break;
    case SAM_EVENT_CID_BAT_BST:
    status = san_evt_bat_bst(dev, event);
    break;
    case SAM_EVENT_CID_BAT_ADP:
    status = san_evt_bat_adp(dev, event);
    break;
    case SAM_EVENT_CID_BAT_PROT:
//
// TODO: Implement support for battery protection status change
// event.
//
    return true;
    case SAM_EVENT_CID_BAT_DPTF:
    status = san_evt_bat_dptf(dev, event);
    break;
    default:
    return false;
    }
    if (status) {
    dev_err(dev, "error handling power event (cid = %#04x)\n",
    event.command_id);
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn san_evt_bat_workfn(work: *mut work_struct) {
    static void san_evt_bat_workfn(struct work_struct *work)
    {
    struct san_event_work *ev;
    ev = container_of(work, struct san_event_work, work.work);
    san_evt_bat(&ev.event, ev.dev);
    kfree(ev);
    }
    static u32 san_evt_bat_nf(struct ssam_event_notifier *nf,
    const struct ssam_event *event)
    {
    struct san_data *d = to_san_data(nf, nf_bat);
    struct san_event_work *work;
    let mut delay: c_ulong = san_evt_bat_delay(event.command_id);
    if (delay == 0)
    return san_evt_bat(event, d.dev) ? SSAM_NOTIF_HANDLED : 0;
    work = kzalloc(sizeof(*work) + event.length, GFP_KERNEL);
    if (!work)
    return ssam_notifier_from_errno(-ENOMEM);
    INIT_DELAYED_WORK(&work.work, san_evt_bat_workfn);
    work.dev = d.dev;
    work.event = *event;
    memcpy(work.event.data, event.data, event.length);
    queue_delayed_work(san_wq, &work.work, delay);
    return SSAM_NOTIF_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn san_evt_tmp_trip(dev: *mut device, event: *const ssam_event) -> c_int {
    static int san_evt_tmp_trip(struct device *dev, const struct ssam_event *event)
    {
    union acpi_object param;
//
// The Surface ACPI expects an integer and not a package. This will
// cause a warning in acpica/nsarguments.c, but that warning can be
// safely ignored.
//
    param.type = ACPI_TYPE_INTEGER;
    param.integer.value = event.instance_id;
    return san_acpi_notify_event(dev, SAN_DSM_EVENT_FN_THERMAL, &param);
    }
#[no_mangle]
unsafe extern "C" fn san_evt_tmp(event: *const ssam_event, dev: *mut device) -> bool {
    static bool san_evt_tmp(const struct ssam_event *event, struct device *dev)
    {
    int status;
    switch (event.command_id) {
    case SAM_EVENT_CID_TMP_TRIP:
    status = san_evt_tmp_trip(dev, event);
    break;
    default:
    return false;
    }
    if (status) {
    dev_err(dev, "error handling thermal event (cid = %#04x)\n",
    event.command_id);
    }
    return true;
    }
    static u32 san_evt_tmp_nf(struct ssam_event_notifier *nf,
    const struct ssam_event *event)
    {
    struct san_data *d = to_san_data(nf, nf_tmp);
    return san_evt_tmp(event, d.dev) ? SSAM_NOTIF_HANDLED : 0;
    }
// -- ACPI GSB OperationRegion handler --------------------------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsb_data_in {
    pub cv: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsb_data_rqsx {
    pub /: *mut *mut u8 cv; / Command value (san_gsb_request_cv).,
    pub /: *mut *mut u8 tc; / Target category.,
    pub /: *mut *mut u8 tid; / Target ID.,
    pub /: *mut *mut u8 iid; / Instance ID.,
    pub /: *mut *mut u8 snc; / Expect-response-flag.,
    pub /: *mut *mut u8 cid; / Command ID.,
    pub /: *mut *mut u16 cdl; / Payload length.,
    pub /: *mut *mut u8 pld[]; / Payload.,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsb_data_etwl {
    pub /: *mut *mut u8 cv; / Command value (should be 0x02).,
    pub /: *mut *mut u8 etw3; / Unknown.,
    pub /: *mut *mut u8 etw4; / Unknown.,
    pub /: *mut *mut u8 msg[]; / Error message (ASCIIZ).,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsb_data_out {
    pub /: *mut *mut u8 status; / _SSH communication status.,
    pub /: *mut *mut u8 len; / _SSH payload length.,
    pub /: *mut *mut u8 pld[]; / _SSH payload.,
    pub __packed: },
    union gsb_buffer_data {
    pub /: *mut *mut gsb_data_in in; / Common input.,
    pub /: *mut *mut gsb_data_rqsx rqsx; / RQSX input.,
    pub /: *mut *mut gsb_data_etwl etwl; / ETWL input.,
    pub /: *mut *mut gsb_data_out out; / Output.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsb_buffer {
    pub /: *mut *mut u8 status; / GSB AttribRawProcess status.,
    pub /: *mut *mut u8 len; / GSB AttribRawProcess length.,
    pub data: union gsb_buffer_data,
    pub __packed: },

pub const SAN_GSB_COMMAND: c_int = 0;
    enum san_gsb_request_cv {
    SAN_GSB_REQUEST_CV_RQST = 0x01,
    SAN_GSB_REQUEST_CV_ETWL = 0x02,
    SAN_GSB_REQUEST_CV_RQSG = 0x03,
}

pub const SAN_REQUEST_NUM_TRIES: c_int = 5;
#[no_mangle]
unsafe extern "C" fn san_etwl(d: *mut san_data, b: *mut gsb_buffer) -> acpi_status {
    static acpi_status san_etwl(struct san_data *d, struct gsb_buffer *b)
    {
    struct gsb_data_etwl *etwl = &b.data.etwl;
    if (b.len < sizeof(struct gsb_data_etwl)) {
    dev_err(d.dev, "invalid ETWL package (len = %d)\n", b.len);
    return AE_OK;
    }
    dev_err(d.dev, "ETWL(%#04x, %#04x): %.*s\n", etwl.etw3, etwl.etw4,
    (unsigned int)(b.len - sizeof(struct gsb_data_etwl)),
    (char *)etwl.msg);
// Indicate success.
    b.status = 0x00;
    b.len = 0x00;
    return AE_OK;
    }
    static
    struct gsb_data_rqsx *san_validate_rqsx(struct device *dev, const char *type,
    struct gsb_buffer *b)
    {
    struct gsb_data_rqsx *rqsx = &b.data.rqsx;
    if (b.len < sizeof(struct gsb_data_rqsx)) {
    dev_err(dev, "invalid %s package (len = %d)\n", type, b.len);
    return core::ptr::null_mut();
    }
    if (get_unaligned(&rqsx.cdl) != b.len - sizeof(struct gsb_data_rqsx)) {
    dev_err(dev, "bogus %s package (len = %d, cdl = %d)\n",
    type, b.len, get_unaligned(&rqsx.cdl));
    return core::ptr::null_mut();
    }
    if (get_unaligned(&rqsx.cdl) > SAN_GSB_MAX_RQSX_PAYLOAD) {
    dev_err(dev, "payload for %s package too large (cdl = %d)\n",
    type, get_unaligned(&rqsx.cdl));
    return core::ptr::null_mut();
    }
    return rqsx;
    }
#[no_mangle]
unsafe extern "C" fn gsb_rqsx_response_error(gsb: *mut gsb_buffer, status: c_int) {
    static void gsb_rqsx_response_error(struct gsb_buffer *gsb, int status)
    {
    gsb.status = 0x00;
    gsb.len = 0x02;
    gsb.data.out.status = (u8)(-status);
    gsb.data.out.len = 0x00;
    }
#[no_mangle]
unsafe extern "C" fn gsb_rqsx_response_success(gsb: *mut gsb_buffer, ptr: *mut u8, len: usize) {
    static void gsb_rqsx_response_success(struct gsb_buffer *gsb, u8 *ptr, size_t len)
    {
    gsb.status = 0x00;
    gsb.len = len + 2;
    gsb.data.out.status = 0x00;
    gsb.data.out.len = len;
    if (len)
    memcpy(&gsb.data.out.pld[0], ptr, len);
    }
    static acpi_status san_rqst_fixup_suspended(struct san_data *d,
    struct ssam_request *rqst,
    struct gsb_buffer *gsb)
    {
    if (rqst.target_category == SSAM_SSH_TC_BAS && rqst.command_id == 0x0D) {
    let mut base_state: u8 = 1;
// Base state quirk:
// The base state may be queried from ACPI when the EC is still
// suspended. In this case it will return '-EPERM'. This query
// will only be triggered from the ACPI lid GPE interrupt, thus
// we are either in laptop or studio mode (base status 0x01 or
// 0x02). Furthermore, we will only get here if the device (and
// EC) have been suspended.
//
// We now assume that the device is in laptop mode (0x01). This
// has the drawback that it will wake the device when unfolding
// it in studio mode, but it also allows us to avoid actively
// waiting for the EC to wake up, which may incur a notable
// delay.
//
    dev_dbg(d.dev, "rqst: fixup: base-state quirk\n");
    gsb_rqsx_response_success(gsb, &base_state, sizeof(base_state));
    return AE_OK;
    }
    gsb_rqsx_response_error(gsb, -ENXIO);
    return AE_OK;
    }
#[no_mangle]
unsafe extern "C" fn san_rqst(d: *mut san_data, buffer: *mut gsb_buffer) -> acpi_status {
    static acpi_status san_rqst(struct san_data *d, struct gsb_buffer *buffer)
    {
    u8 rspbuf[SAN_GSB_MAX_RESPONSE];
    struct gsb_data_rqsx *gsb_rqst;
    struct ssam_request rqst;
    struct ssam_response rsp;
    let mut status: c_int = 0;
    gsb_rqst = san_validate_rqsx(d.dev, "RQST", buffer);
    if (!gsb_rqst)
    return AE_OK;
    rqst.target_category = gsb_rqst.tc;
    rqst.target_id = gsb_rqst.tid;
    rqst.command_id = gsb_rqst.cid;
    rqst.instance_id = gsb_rqst.iid;
    rqst.flags = gsb_rqst.snc ? SSAM_REQUEST_HAS_RESPONSE : 0;
    rqst.length = get_unaligned(&gsb_rqst.cdl);
    rqst.payload = &gsb_rqst.pld[0];
    rsp.capacity = ARRAY_SIZE(rspbuf);
    rsp.length = 0;
    rsp.pointer = &rspbuf[0];
// Handle suspended device.
    if (d.dev.power.is_suspended) {
    dev_warn(d.dev, "rqst: device is suspended, not executing\n");
    return san_rqst_fixup_suspended(d, &rqst, buffer);
    }
    status = __ssam_retry(ssam_request_do_sync_onstack, SAN_REQUEST_NUM_TRIES,
    d.ctrl, &rqst, &rsp, SAN_GSB_MAX_RQSX_PAYLOAD);
    if (!status) {
    gsb_rqsx_response_success(buffer, rsp.pointer, rsp.length);
    } else {
    dev_err(d.dev, "rqst: failed with error %d\n", status);
    gsb_rqsx_response_error(buffer, status);
    }
    return AE_OK;
    }
#[no_mangle]
unsafe extern "C" fn san_rqsg(d: *mut san_data, buffer: *mut gsb_buffer) -> acpi_status {
    static acpi_status san_rqsg(struct san_data *d, struct gsb_buffer *buffer)
    {
    struct gsb_data_rqsx *gsb_rqsg;
    struct san_dgpu_event evt;
    int status;
    gsb_rqsg = san_validate_rqsx(d.dev, "RQSG", buffer);
    if (!gsb_rqsg)
    return AE_OK;
    evt.category = gsb_rqsg.tc;
    evt.target = gsb_rqsg.tid;
    evt.command = gsb_rqsg.cid;
    evt.instance = gsb_rqsg.iid;
    evt.length = get_unaligned(&gsb_rqsg.cdl);
    evt.payload = &gsb_rqsg.pld[0];
    status = san_dgpu_notifier_call(&evt);
    if (!status) {
    gsb_rqsx_response_success(buffer, core::ptr::null_mut(), 0);
    } else {
    dev_err(d.dev, "rqsg: failed with error %d\n", status);
    gsb_rqsx_response_error(buffer, status);
    }
    return AE_OK;
    }
    static acpi_status san_opreg_handler(u32 function, acpi_physical_address command,
    u32 bits, u64 *value64, void *opreg_context,
    void *region_context)
    {
    struct san_data *d = to_san_data(opreg_context, info);
    struct gsb_buffer *buffer = (struct gsb_buffer *)value64;
    let mut accessor_type: c_int = (function & 0xFFFF0000) >> 16;
    if (command != SAN_GSB_COMMAND) {
    dev_warn(d.dev, "unsupported command: %#04llx\n", command);
    return AE_OK;
    }
    if (accessor_type != ACPI_GSB_ACCESS_ATTRIB_RAW_PROCESS) {
    dev_err(d.dev, "invalid access type: %#04x\n", accessor_type);
    return AE_OK;
    }
// Buffer must have at least contain the command-value.
    if (buffer.len == 0) {
    dev_err(d.dev, "request-package too small\n");
    return AE_OK;
    }
    switch (buffer.data.in.cv) {
    case SAN_GSB_REQUEST_CV_RQST:
    return san_rqst(d, buffer);
    case SAN_GSB_REQUEST_CV_ETWL:
    return san_etwl(d, buffer);
    case SAN_GSB_REQUEST_CV_RQSG:
    return san_rqsg(d, buffer);
    default:
    dev_warn(d.dev, "unsupported SAN0 request (cv: %#04x)\n",
    buffer.data.in.cv);
    return AE_OK;
    }
    }
// -- Driver setup. ---------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn san_events_register(pdev: *mut platform_device) -> c_int {
    static int san_events_register(struct platform_device *pdev)
    {
    struct san_data *d = platform_get_drvdata(pdev);
    int status;
    d.nf_bat.base.priority = 1;
    d.nf_bat.base.fn = san_evt_bat_nf;
    d.nf_bat.event.reg = SSAM_EVENT_REGISTRY_SAM;
    d.nf_bat.event.id.target_category = SSAM_SSH_TC_BAT;
    d.nf_bat.event.id.instance = 0;
    d.nf_bat.event.mask = SSAM_EVENT_MASK_TARGET;
    d.nf_bat.event.flags = SSAM_EVENT_SEQUENCED;
    d.nf_tmp.base.priority = 1;
    d.nf_tmp.base.fn = san_evt_tmp_nf;
    d.nf_tmp.event.reg = SSAM_EVENT_REGISTRY_SAM;
    d.nf_tmp.event.id.target_category = SSAM_SSH_TC_TMP;
    d.nf_tmp.event.id.instance = 0;
    d.nf_tmp.event.mask = SSAM_EVENT_MASK_TARGET;
    d.nf_tmp.event.flags = SSAM_EVENT_SEQUENCED;
    status = ssam_notifier_register(d.ctrl, &d.nf_bat);
    if (status)
    return status;
    status = ssam_notifier_register(d.ctrl, &d.nf_tmp);
    if (status)
    ssam_notifier_unregister(d.ctrl, &d.nf_bat);
    return status;
    }
#[no_mangle]
unsafe extern "C" fn san_events_unregister(pdev: *mut platform_device) {
    static void san_events_unregister(struct platform_device *pdev)
    {
    struct san_data *d = platform_get_drvdata(pdev);
    ssam_notifier_unregister(d.ctrl, &d.nf_bat);
    ssam_notifier_unregister(d.ctrl, &d.nf_tmp);
    }

    do {										\
    char *path = "<error getting consumer path>";				\
    struct acpi_buffer buffer = {						\
    .length = ACPI_ALLOCATE_BUFFER,					\
    .pointer = core::ptr::null_mut(),						\
    };									\
    \
    if (ACPI_SUCCESS(acpi_get_name(handle, ACPI_FULL_PATHNAME, &buffer)))	\
    path = buffer.pointer;						\
    \
    dev_##level(dev, "[%s]: " fmt, path, ##__VA_ARGS__);			\
    kfree(buffer.pointer);							\
    } while (0)

    san_consumer_printk(dbg, dev, handle, fmt, ##__VA_ARGS__)

    san_consumer_printk(warn, dev, handle, fmt, ##__VA_ARGS__)
    static acpi_status san_consumer_setup(acpi_handle handle, u32 lvl,
    void *context, void **rv)
    {
    let mut flags: u32 = DL_FLAG_PM_RUNTIME | DL_FLAG_AUTOREMOVE_SUPPLIER;
    struct platform_device *pdev = context;
    struct acpi_device *adev;
    struct device_link *link;
    if (!acpi_device_dep(handle, ACPI_HANDLE(&pdev.dev)))
    return AE_OK;
// Ignore ACPI devices that are not present.
    adev = acpi_fetch_acpi_dev(handle);
    if (!adev)
    return AE_OK;
    san_consumer_dbg(&pdev.dev, handle, "creating device link\n");
// Try to set up device links, ignore but log errors.
    link = device_link_add(&adev.dev, &pdev.dev, flags);
    if (!link) {
    san_consumer_warn(&pdev.dev, handle, "failed to create device link\n");
    return AE_OK;
    }
    return AE_OK;
    }
#[no_mangle]
unsafe extern "C" fn san_consumer_links_setup(pdev: *mut platform_device) -> c_int {
    static int san_consumer_links_setup(struct platform_device *pdev)
    {
    acpi_status status;
    status = acpi_walk_namespace(ACPI_TYPE_DEVICE, ACPI_ROOT_OBJECT,
    ACPI_UINT32_MAX, san_consumer_setup, core::ptr::null_mut(),
    pdev, core::ptr::null_mut());
    return status ? -EFAULT : 0;
    }
#[no_mangle]
unsafe extern "C" fn san_probe(pdev: *mut platform_device) -> c_int {
    static int san_probe(struct platform_device *pdev)
    {
    struct ssam_controller *ctrl;
    struct acpi_device *san;
    struct san_data *data;
    acpi_status astatus;
    int status;
    san = ACPI_COMPANION(&pdev.dev);
    if (!san)
    return -ENODEV;
    ctrl = ssam_client_bind(&pdev.dev);
    if (IS_ERR(ctrl))
    return PTR_ERR(ctrl) == -ENODEV ? -EPROBE_DEFER : PTR_ERR(ctrl);
    status = san_consumer_links_setup(pdev);
    if (status)
    return status;
    data = devm_kzalloc(&pdev.dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.dev = &pdev.dev;
    data.ctrl = ctrl;
    platform_set_drvdata(pdev, data);
    astatus = acpi_install_address_space_handler(san.handle,
    ACPI_ADR_SPACE_GSBUS,
    &san_opreg_handler, core::ptr::null_mut(),
    &data.info);
    if (ACPI_FAILURE(astatus))
    return -ENXIO;
    status = san_events_register(pdev);
    if (status)
    goto err_enable_events;
    status = san_set_rqsg_interface_device(&pdev.dev);
    if (status)
    goto err_install_dev;
    acpi_dev_clear_dependencies(san);
    return 0;
    err_install_dev:
    san_events_unregister(pdev);
    err_enable_events:
    acpi_remove_address_space_handler(san, ACPI_ADR_SPACE_GSBUS,
    &san_opreg_handler);
    return status;
    }
#[no_mangle]
unsafe extern "C" fn san_remove(pdev: *mut platform_device) {
    static void san_remove(struct platform_device *pdev)
    {
    let mut san: acpi_handle = ACPI_HANDLE(&pdev.dev);
    san_set_rqsg_interface_device(core::ptr::null_mut());
    acpi_remove_address_space_handler(san, ACPI_ADR_SPACE_GSBUS,
    &san_opreg_handler);
    san_events_unregister(pdev);
//
// We have unregistered our event sources. Now we need to ensure that
// all delayed works they may have spawned are run to completion.
//
    flush_workqueue(san_wq);
    }
    static const struct acpi_device_id san_match[] = {
    { "MSHW0091" },
    { },
    };
    MODULE_DEVICE_TABLE(acpi, san_match);
    static struct platform_driver surface_acpi_notify = {
    .probe = san_probe,
    .remove = san_remove,
    .driver = {
    .name = "surface_acpi_notify",
    .acpi_match_table = san_match,
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    };
#[no_mangle]
unsafe extern "C" fn san_init() -> int __init {
    static int __init san_init(void)
    {
    int ret;
    san_wq = alloc_workqueue("san_wq", WQ_PERCPU, 0);
    if (!san_wq)
    return -ENOMEM;
    ret = platform_driver_register(&surface_acpi_notify);
    if (ret)
    destroy_workqueue(san_wq);
    return ret;
    }
    module_init(san_init);
#[no_mangle]
unsafe extern "C" fn san_exit() -> void __exit {
    static void __exit san_exit(void)
    {
    platform_driver_unregister(&surface_acpi_notify);
    destroy_workqueue(san_wq);
    }
    module_exit(san_exit);
    MODULE_AUTHOR("Maximilian Luz <luzmaximilian@gmail.com>");
    MODULE_DESCRIPTION("Surface ACPI Notify driver for Surface System Aggregator Module");
    MODULE_LICENSE("GPL");
