//! Automatically rewritten from C to Rust
//! Source: drivers/media/usb/uvc/uvc_status.c
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
// uvc_status.c  --  USB Video Class driver - Status endpoint
//
// Copyright (C) 2005-2009
// Laurent Pinchart (laurent.pinchart@ideasonboard.com)
//

// --------------------------------------------------------------------------
// Input device
//

#[no_mangle]
unsafe extern "C" fn uvc_input_has_button(dev: *mut uvc_device) -> bool {
    static bool uvc_input_has_button(struct uvc_device *dev)
    {
    struct uvc_streaming *stream;
//
// The device has button events if both bTriggerSupport and
// bTriggerUsage are one. Otherwise the camera button does not
// exist or is handled automatically by the camera without host
// driver or client application intervention.
//
    list_for_each_entry(stream, &dev.streams, list) {
    if (stream.header.bTriggerSupport == 1 &&
    stream.header.bTriggerUsage == 1)
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn uvc_input_init(dev: *mut uvc_device) -> c_int {
    static int uvc_input_init(struct uvc_device *dev)
    {
    struct input_dev *input;
    int ret;
    if (!uvc_input_has_button(dev))
    return 0;
    input = input_allocate_device();
    if (input == core::ptr::null_mut())
    return -ENOMEM;
    usb_make_path(dev.udev, dev.input_phys, sizeof(dev.input_phys));
    strlcat(dev.input_phys, "/button", sizeof(dev.input_phys));
    input.name = dev.name;
    input.phys = dev.input_phys;
    usb_to_input_id(dev.udev, &input.id);
    input.dev.parent = &dev.intf.dev;
    __set_bit(EV_KEY, input.evbit);
    __set_bit(KEY_CAMERA, input.keybit);
    ret = input_register_device(input);
    if (ret < 0)
    goto error;
    dev.input = input;
    return 0;
    error:
    input_free_device(input);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn uvc_input_unregister(dev: *mut uvc_device) {
    static void uvc_input_unregister(struct uvc_device *dev)
    {
    if (dev.input)
    input_unregister_device(dev.input);
    }
    static void uvc_input_report_key(struct uvc_device *dev, unsigned int code,
    int value)
    {
    if (dev.input) {
    input_report_key(dev.input, code, value);
    input_sync(dev.input);
    }
    }

// Macro flag: #define uvc_input_init(dev)
// Macro flag: #define uvc_input_unregister(dev)

// --------------------------------------------------------------------------
// Status interrupt endpoint
//
    static void uvc_event_streaming(struct uvc_device *dev,
    struct uvc_status *status, int len)
    {
    if (len <= offsetof(struct uvc_status, bEvent)) {
    uvc_dbg(dev, STATUS,
    "Invalid streaming status event received\n");
    return;
    }
    if (status.bEvent == 0) {
    if (len <= offsetof(struct uvc_status, streaming))
    return;
    uvc_dbg(dev, STATUS, "Button (intf %u) %s len %d\n",
    status.bOriginator,
    status.streaming.button ? "pressed" : "released", len);
    uvc_input_report_key(dev, KEY_CAMERA, status.streaming.button);
    } else {
    uvc_dbg(dev, STATUS, "Stream %u error event %02x len %d\n",
    status.bOriginator, status.bEvent, len);
    }
    }
pub const UVC_CTRL_VALUE_CHANGE: c_int = 0;
pub const UVC_CTRL_INFO_CHANGE: c_int = 1;
pub const UVC_CTRL_FAILURE_CHANGE: c_int = 2;
pub const UVC_CTRL_MIN_CHANGE: c_int = 3;
pub const UVC_CTRL_MAX_CHANGE: c_int = 4;
    static struct uvc_control *uvc_event_entity_find_ctrl(struct uvc_entity *entity,
    u8 selector)
    {
    struct uvc_control *ctrl;
    unsigned int i;
    for (i = 0, ctrl = entity.controls; i < entity.ncontrols; i++, ctrl++)
    if (ctrl.info.selector == selector)
    return ctrl;
    return core::ptr::null_mut();
    }
    static struct uvc_control *uvc_event_find_ctrl(struct uvc_device *dev,
    const struct uvc_status *status,
    struct uvc_video_chain **chain)
    {
    list_for_each_entry((*chain), &dev.chains, list) {
    struct uvc_entity *entity;
    struct uvc_control *ctrl;
    list_for_each_entry(entity, &(*chain).entities, chain) {
    if (entity.id != status.bOriginator)
    continue;
    ctrl = uvc_event_entity_find_ctrl(entity,
    status.control.bSelector);
    if (ctrl)
    return ctrl;
    }
    }
    return core::ptr::null_mut();
    }
    static bool uvc_event_control(struct urb *urb,
    const struct uvc_status *status, int len)
    {
    static const char *attrs[] = { "value", "info", "failure", "min", "max" };
    struct uvc_device *dev = urb.context;
    struct uvc_video_chain *chain;
    struct uvc_control *ctrl;
    if (len < 6 || status.bEvent != 0 ||
    status.control.bAttribute >= ARRAY_SIZE(attrs)) {
    uvc_dbg(dev, STATUS, "Invalid control status event received\n");
    return false;
    }
    uvc_dbg(dev, STATUS, "Control %u/%u %s change len %d\n",
    status.bOriginator, status.control.bSelector,
    attrs[status.control.bAttribute], len);
// Find the control.
    ctrl = uvc_event_find_ctrl(dev, status, &chain);
    if (!ctrl)
    return false;
    switch (status.control.bAttribute) {
    case UVC_CTRL_VALUE_CHANGE:
    return uvc_ctrl_status_event_async(urb, chain, ctrl,
    status.control.bValue);
    case UVC_CTRL_INFO_CHANGE:
    case UVC_CTRL_FAILURE_CHANGE:
    case UVC_CTRL_MIN_CHANGE:
    case UVC_CTRL_MAX_CHANGE:
    break;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn uvc_status_complete(urb: *mut urb) {
    static void uvc_status_complete(struct urb *urb)
    {
    struct uvc_device *dev = urb.context;
    int len, ret;
    switch (urb.status) {
    case 0:
    break;
    case -ENOENT:		/* usb_kill_urb() called. */
    case -ECONNRESET:	/* usb_unlink_urb() called. */
    case -ESHUTDOWN:	/* The endpoint is being disabled. */
    case -EPROTO:		/* Device is disconnected (reported by some host controllers). */
    return;
    default:
    dev_warn(&dev.intf.dev,
    "Non-zero status (%d) in status completion handler.\n",
    urb.status);
    return;
    }
    len = urb.actual_length;
    if (len > 0) {
    switch (dev.status.bStatusType & 0x0f) {
    case UVC_STATUS_TYPE_CONTROL: {
    if (uvc_event_control(urb, dev.status, len))
// The URB will be resubmitted in work context.
    return;
    break;
    }
    case UVC_STATUS_TYPE_STREAMING: {
    uvc_event_streaming(dev, dev.status, len);
    break;
    }
    default:
    uvc_dbg(dev, STATUS, "Unknown status event type %u\n",
    dev.status.bStatusType);
    break;
    }
    }
// Resubmit the URB.
    urb.interval = dev.int_ep.desc.bInterval;
    ret = usb_submit_urb(urb, GFP_ATOMIC);
    if (ret < 0)
    dev_err(&dev.intf.dev,
    "Failed to resubmit status URB (%d).\n", ret);
    }
#[no_mangle]
pub unsafe extern "C" fn uvc_status_init(dev: *mut uvc_device) -> c_int {
    int uvc_status_init(struct uvc_device *dev)
    {
    struct usb_host_endpoint *ep = dev.int_ep;
    unsigned int pipe;
    int interval;
    mutex_init(&dev.status_lock);
    if (ep == core::ptr::null_mut())
    return 0;
    dev.status = kzalloc_obj(*dev.status);
    if (!dev.status)
    return -ENOMEM;
    dev.int_urb = usb_alloc_urb(0, GFP_KERNEL);
    if (!dev.int_urb) {
    kfree(dev.status);
    dev.status = core::ptr::null_mut();
    return -ENOMEM;
    }
    pipe = usb_rcvintpipe(dev.udev, ep.desc.bEndpointAddress);
//
// For high-speed interrupt endpoints, the bInterval value is used as
// an exponent of two. Some developers forgot about it.
//
    interval = ep.desc.bInterval;
    if (interval > 16 && dev.udev.speed == USB_SPEED_HIGH &&
    (dev.quirks & UVC_QUIRK_STATUS_INTERVAL))
    interval = fls(interval) - 1;
    usb_fill_int_urb(dev.int_urb, dev.udev, pipe,
    dev.status, sizeof(*dev.status), uvc_status_complete,
    dev, interval);
    uvc_input_init(dev);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn uvc_status_unregister(dev: *mut uvc_device) {
    void uvc_status_unregister(struct uvc_device *dev)
    {
    if (!dev.status)
    return;
    uvc_status_suspend(dev);
    uvc_input_unregister(dev);
    }
#[no_mangle]
pub unsafe extern "C" fn uvc_status_cleanup(dev: *mut uvc_device) {
    void uvc_status_cleanup(struct uvc_device *dev)
    {
    usb_free_urb(dev.int_urb);
    kfree(dev.status);
    }
#[no_mangle]
unsafe extern "C" fn uvc_status_start(dev: *mut uvc_device, flags: gfp_t) -> c_int {
    static int uvc_status_start(struct uvc_device *dev, gfp_t flags)
    {
    lockdep_assert_held(&dev.status_lock);
    if (!dev.int_urb)
    return 0;
//
// If the previous uvc_status_stop() call was from the async work,
// the work may still be running. Wait for it to finish before we submit
// the urb.
//
    flush_work(&dev.async_ctrl.work);
// Clear the flush status if we were previously stopped.
    smp_store_release(&dev.flush_status, false);
    return usb_submit_urb(dev.int_urb, flags);
    }
#[no_mangle]
unsafe extern "C" fn uvc_status_stop(dev: *mut uvc_device) {
    static void uvc_status_stop(struct uvc_device *dev)
    {
    struct uvc_ctrl_work *w = &dev.async_ctrl;
    lockdep_assert_held(&dev.status_lock);
    if (!dev.int_urb)
    return;
//
// Prevent the asynchronous control handler from requeing the URB. The
// barrier is needed so the flush_status change is visible to other
// CPUs running the asynchronous handler before usb_kill_urb() is
// called below.
//
    smp_store_release(&dev.flush_status, true);
//
// If we are called from the event work function, the URB is guaranteed
// to not be in flight as it has completed and has not been resubmitted.
// There's no need to cancel the work (which would deadlock), or to kill
// the URB.
//
    if (current_work() == &w.work)
    return;
//
// Cancel any pending asynchronous work. If any status event was queued,
// process it synchronously.
//
    if (cancel_work_sync(&w.work))
    uvc_ctrl_status_event(w.chain, w.ctrl, w.data);
// Kill the urb.
    usb_kill_urb(dev.int_urb);
//
// The URB completion handler may have queued asynchronous work. This
// won't resubmit the URB as flush_status is set, but it needs to be
// cancelled before returning or it could then race with a future
// uvc_status_start() call.
//
    if (cancel_work_sync(&w.work))
    uvc_ctrl_status_event(w.chain, w.ctrl, w.data);
    }
#[no_mangle]
pub unsafe extern "C" fn uvc_status_resume(dev: *mut uvc_device) -> c_int {
    int uvc_status_resume(struct uvc_device *dev)
    {
    guard(mutex)(&dev.status_lock);
    if (dev.status_users)
    return uvc_status_start(dev, GFP_NOIO);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn uvc_status_suspend(dev: *mut uvc_device) {
    void uvc_status_suspend(struct uvc_device *dev)
    {
    guard(mutex)(&dev.status_lock);
    if (dev.status_users)
    uvc_status_stop(dev);
    }
#[no_mangle]
pub unsafe extern "C" fn uvc_status_get(dev: *mut uvc_device) -> c_int {
    int uvc_status_get(struct uvc_device *dev)
    {
    int ret;
    guard(mutex)(&dev.status_lock);
    if (!dev.status_users) {
    ret = uvc_status_start(dev, GFP_KERNEL);
    if (ret)
    return ret;
    }
    dev.status_users++;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn uvc_status_put(dev: *mut uvc_device) {
    void uvc_status_put(struct uvc_device *dev)
    {
    guard(mutex)(&dev.status_lock);
    if (dev.status_users == 1)
    uvc_status_stop(dev);
    WARN_ON(!dev.status_users);
    if (dev.status_users)
    dev.status_users--;
    }
