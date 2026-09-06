//! Automatically rewritten from C to Rust
//! Source: drivers/usb/gadget/function/f_acm.c
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
// f_acm.c -- USB CDC serial (ACM) function driver
//
// Copyright (C) 2003 Al Borchers (alborchers@steinerpoint.com)
// Copyright (C) 2008 by David Brownell
// Copyright (C) 2008 by Nokia Corporation
// Copyright (C) 2009 by Samsung Electronics
// Author: Michal Nazarewicz (mina86@mina86.com)
//
// #define VERBOSE_DEBUG

//
// This CDC ACM function support just wraps control functions and
// notifications around the generic serial-over-usb code.
//
// Because CDC ACM is standardized by the USB-IF, many host operating
// systems have drivers for it.  Accordingly, ACM is the preferred
// interop solution for serial-port type connections.  The control
// models are often not necessary, and in any case don't do much in
// this bare-bones implementation.
//
// Note that even MS-Windows has some support for ACM.  However, that
// support is somewhat broken because when you use ACM in a composite
// device, having multiple interfaces confuses the poor OS.  It doesn't
// seem to understand CDC Union descriptors.  The new "association"
// descriptors (roughly equivalent to CDC Unions) may sometimes help.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct f_acm {
    pub port: gserial,
    pub data_id: u8 ctrl_id,,
    pub port_num: u8,
    pub bInterfaceProtocol: u8,
    pub pending: u8,
// lock is mostly for pending and notify_req ... they get accessed
// by callbacks both from tty (open/close/break) under its spinlock,
// and notify_req.complete() which can't use that lock.
//
    pub lock: spinlock_t,
    pub notify: *mut usb_ep,
    pub notify_req: *mut usb_request,
    pub /: *mut *mut usb_cdc_line_coding port_line_coding; / 8-N-1 etc,
// SetControlLineState request -- CDC 1.1 section 6.2.14 (INPUT)
    pub port_handshake_bits: u16,
// SerialState notification -- CDC 1.1 section 6.3.5 (OUTPUT)
    pub serial_state: u16,
}

    static inline struct f_acm *func_to_acm(struct usb_function *f)
    {
    return container_of(f, struct f_acm, port.func);
    }
    static inline struct f_acm *port_to_acm(struct gserial *p)
    {
    return container_of(p, struct f_acm, port);
    }
// -------------------------------------------------------------------------
// notification endpoint uses smallish and infrequent fixed-size messages
pub const GS_NOTIFY_INTERVAL_MS: c_int = 32;

// interface and class descriptors:
    static struct usb_interface_assoc_descriptor
    acm_iad_descriptor = {
    .bLength =		sizeof acm_iad_descriptor,
    .bDescriptorType =	USB_DT_INTERFACE_ASSOCIATION,
// .bFirstInterface =	DYNAMIC,
    .bInterfaceCount = 	2,	// control + data
    .bFunctionClass =	USB_CLASS_COMM,
    .bFunctionSubClass =	USB_CDC_SUBCLASS_ACM,
// .bFunctionProtocol = DYNAMIC
// .iFunction =		DYNAMIC
    };
    static struct usb_interface_descriptor acm_control_interface_desc = {
    .bLength =		USB_DT_INTERFACE_SIZE,
    .bDescriptorType =	USB_DT_INTERFACE,
// .bInterfaceNumber = DYNAMIC
    .bNumEndpoints =	1,
    .bInterfaceClass =	USB_CLASS_COMM,
    .bInterfaceSubClass =	USB_CDC_SUBCLASS_ACM,
// .bInterfaceProtocol = DYNAMIC
// .iInterface = DYNAMIC
    };
    static struct usb_interface_descriptor acm_data_interface_desc = {
    .bLength =		USB_DT_INTERFACE_SIZE,
    .bDescriptorType =	USB_DT_INTERFACE,
// .bInterfaceNumber = DYNAMIC
    .bNumEndpoints =	2,
    .bInterfaceClass =	USB_CLASS_CDC_DATA,
    .bInterfaceSubClass =	0,
    .bInterfaceProtocol =	0,
// .iInterface = DYNAMIC
    };
    static struct usb_cdc_header_desc acm_header_desc = {
    .bLength =		sizeof(acm_header_desc),
    .bDescriptorType =	USB_DT_CS_INTERFACE,
    .bDescriptorSubType =	USB_CDC_HEADER_TYPE,
    .bcdCDC =		cpu_to_le16(0x0110),
    };
    static struct usb_cdc_call_mgmt_descriptor
    acm_call_mgmt_descriptor = {
    .bLength =		sizeof(acm_call_mgmt_descriptor),
    .bDescriptorType =	USB_DT_CS_INTERFACE,
    .bDescriptorSubType =	USB_CDC_CALL_MANAGEMENT_TYPE,
    .bmCapabilities =	0,
// .bDataInterface = DYNAMIC
    };
    static struct usb_cdc_acm_descriptor acm_descriptor = {
    .bLength =		sizeof(acm_descriptor),
    .bDescriptorType =	USB_DT_CS_INTERFACE,
    .bDescriptorSubType =	USB_CDC_ACM_TYPE,
    .bmCapabilities =	USB_CDC_CAP_LINE,
    };
    static struct usb_cdc_union_desc acm_union_desc = {
    .bLength =		sizeof(acm_union_desc),
    .bDescriptorType =	USB_DT_CS_INTERFACE,
    .bDescriptorSubType =	USB_CDC_UNION_TYPE,
// .bMasterInterface0 =	DYNAMIC
// .bSlaveInterface0 =	DYNAMIC
    };
// full speed support:
    static struct usb_endpoint_descriptor acm_fs_notify_desc = {
    .bLength =		USB_DT_ENDPOINT_SIZE,
    .bDescriptorType =	USB_DT_ENDPOINT,
    .bEndpointAddress =	USB_DIR_IN,
    .bmAttributes =		USB_ENDPOINT_XFER_INT,
    .wMaxPacketSize =	cpu_to_le16(GS_NOTIFY_MAXPACKET),
    .bInterval =		GS_NOTIFY_INTERVAL_MS,
    };
    static struct usb_endpoint_descriptor acm_fs_in_desc = {
    .bLength =		USB_DT_ENDPOINT_SIZE,
    .bDescriptorType =	USB_DT_ENDPOINT,
    .bEndpointAddress =	USB_DIR_IN,
    .bmAttributes =		USB_ENDPOINT_XFER_BULK,
    };
    static struct usb_endpoint_descriptor acm_fs_out_desc = {
    .bLength =		USB_DT_ENDPOINT_SIZE,
    .bDescriptorType =	USB_DT_ENDPOINT,
    .bEndpointAddress =	USB_DIR_OUT,
    .bmAttributes =		USB_ENDPOINT_XFER_BULK,
    };
    static struct usb_descriptor_header *acm_fs_function[] = {
    (struct usb_descriptor_header *) &acm_iad_descriptor,
    (struct usb_descriptor_header *) &acm_control_interface_desc,
    (struct usb_descriptor_header *) &acm_header_desc,
    (struct usb_descriptor_header *) &acm_call_mgmt_descriptor,
    (struct usb_descriptor_header *) &acm_descriptor,
    (struct usb_descriptor_header *) &acm_union_desc,
    (struct usb_descriptor_header *) &acm_fs_notify_desc,
    (struct usb_descriptor_header *) &acm_data_interface_desc,
    (struct usb_descriptor_header *) &acm_fs_in_desc,
    (struct usb_descriptor_header *) &acm_fs_out_desc,
    core::ptr::null_mut(),
    };
// high speed support:
    static struct usb_endpoint_descriptor acm_hs_notify_desc = {
    .bLength =		USB_DT_ENDPOINT_SIZE,
    .bDescriptorType =	USB_DT_ENDPOINT,
    .bEndpointAddress =	USB_DIR_IN,
    .bmAttributes =		USB_ENDPOINT_XFER_INT,
    .wMaxPacketSize =	cpu_to_le16(GS_NOTIFY_MAXPACKET),
    .bInterval =		USB_MS_TO_HS_INTERVAL(GS_NOTIFY_INTERVAL_MS),
    };
    static struct usb_endpoint_descriptor acm_hs_in_desc = {
    .bLength =		USB_DT_ENDPOINT_SIZE,
    .bDescriptorType =	USB_DT_ENDPOINT,
    .bmAttributes =		USB_ENDPOINT_XFER_BULK,
    .wMaxPacketSize =	cpu_to_le16(512),
    };
    static struct usb_endpoint_descriptor acm_hs_out_desc = {
    .bLength =		USB_DT_ENDPOINT_SIZE,
    .bDescriptorType =	USB_DT_ENDPOINT,
    .bmAttributes =		USB_ENDPOINT_XFER_BULK,
    .wMaxPacketSize =	cpu_to_le16(512),
    };
    static struct usb_descriptor_header *acm_hs_function[] = {
    (struct usb_descriptor_header *) &acm_iad_descriptor,
    (struct usb_descriptor_header *) &acm_control_interface_desc,
    (struct usb_descriptor_header *) &acm_header_desc,
    (struct usb_descriptor_header *) &acm_call_mgmt_descriptor,
    (struct usb_descriptor_header *) &acm_descriptor,
    (struct usb_descriptor_header *) &acm_union_desc,
    (struct usb_descriptor_header *) &acm_hs_notify_desc,
    (struct usb_descriptor_header *) &acm_data_interface_desc,
    (struct usb_descriptor_header *) &acm_hs_in_desc,
    (struct usb_descriptor_header *) &acm_hs_out_desc,
    core::ptr::null_mut(),
    };
    static struct usb_endpoint_descriptor acm_ss_in_desc = {
    .bLength =		USB_DT_ENDPOINT_SIZE,
    .bDescriptorType =	USB_DT_ENDPOINT,
    .bmAttributes =		USB_ENDPOINT_XFER_BULK,
    .wMaxPacketSize =	cpu_to_le16(1024),
    };
    static struct usb_endpoint_descriptor acm_ss_out_desc = {
    .bLength =		USB_DT_ENDPOINT_SIZE,
    .bDescriptorType =	USB_DT_ENDPOINT,
    .bmAttributes =		USB_ENDPOINT_XFER_BULK,
    .wMaxPacketSize =	cpu_to_le16(1024),
    };
    static struct usb_ss_ep_comp_descriptor acm_ss_bulk_comp_desc = {
    .bLength =              sizeof acm_ss_bulk_comp_desc,
    .bDescriptorType =      USB_DT_SS_ENDPOINT_COMP,
    };
    static struct usb_descriptor_header *acm_ss_function[] = {
    (struct usb_descriptor_header *) &acm_iad_descriptor,
    (struct usb_descriptor_header *) &acm_control_interface_desc,
    (struct usb_descriptor_header *) &acm_header_desc,
    (struct usb_descriptor_header *) &acm_call_mgmt_descriptor,
    (struct usb_descriptor_header *) &acm_descriptor,
    (struct usb_descriptor_header *) &acm_union_desc,
    (struct usb_descriptor_header *) &acm_hs_notify_desc,
    (struct usb_descriptor_header *) &acm_ss_bulk_comp_desc,
    (struct usb_descriptor_header *) &acm_data_interface_desc,
    (struct usb_descriptor_header *) &acm_ss_in_desc,
    (struct usb_descriptor_header *) &acm_ss_bulk_comp_desc,
    (struct usb_descriptor_header *) &acm_ss_out_desc,
    (struct usb_descriptor_header *) &acm_ss_bulk_comp_desc,
    core::ptr::null_mut(),
    };
// string descriptors:
pub const ACM_CTRL_IDX: c_int = 0;
pub const ACM_DATA_IDX: c_int = 1;
pub const ACM_IAD_IDX: c_int = 2;
// static strings, in UTF-8
    static struct usb_string acm_string_defs[] = {
    [ACM_CTRL_IDX].s = "CDC Abstract Control Model (ACM)",
    [ACM_DATA_IDX].s = "CDC ACM Data",
    [ACM_IAD_IDX ].s = "CDC Serial",
    {  } /* end of list */
    };
    static struct usb_gadget_strings acm_string_table = {
    .language =		0x0409,	/* en-us */
    .strings =		acm_string_defs,
    };
    static struct usb_gadget_strings *acm_strings[] = {
    &acm_string_table,
    core::ptr::null_mut(),
    };
// -------------------------------------------------------------------------
// ACM control ... data handling is delegated to tty library code.
// The main task of this function is to activate and deactivate
// that code based on device state; track parameters like line
// speed, handshake state, and so on; and issue notifications.
//
    static void acm_complete_set_line_coding(struct usb_ep *ep,
    struct usb_request *req)
    {
    struct f_acm	*acm = ep.driver_data;
    struct usb_composite_dev *cdev = acm.port.func.config.cdev;
    if (req.status != 0) {
    dev_dbg(&cdev.gadget.dev, "acm ttyGS%d completion, err %d\n",
    acm.port_num, req.status);
    return;
    }
// normal completion
    if (req.actual != sizeof(acm.port_line_coding)) {
    dev_dbg(&cdev.gadget.dev, "acm ttyGS%d short resp, len %d\n",
    acm.port_num, req.actual);
    usb_ep_set_halt(ep);
    } else {
    struct usb_cdc_line_coding	*value = req.buf;
// REVISIT:  we currently just remember this data.
// If we change that, (a) validate it first, then
// (b) update whatever hardware needs updating,
// (c) worry about locking.  This is information on
// the order of 9600-8-N-1 ... most of which means
// nothing unless we control a real RS232 line.
//
    acm.port_line_coding = *value;
    }
    }
    static int acm_send_break(struct gserial *port, int duration);
#[no_mangle]
unsafe extern "C" fn acm_setup(f: *mut usb_function, ctrl: *const usb_ctrlrequest) -> c_int {
    static int acm_setup(struct usb_function *f, const struct usb_ctrlrequest *ctrl)
    {
    struct f_acm		*acm = func_to_acm(f);
    struct usb_composite_dev *cdev = f.config.cdev;
    struct usb_request	*req = cdev.req;
    let mut value: c_int = -EOPNOTSUPP;
    let mut w_index: u16 = le16_to_cpu(ctrl.wIndex);
    let mut w_value: u16 = le16_to_cpu(ctrl.wValue);
    let mut w_length: u16 = le16_to_cpu(ctrl.wLength);
// composite driver infrastructure handles everything except
// CDC class messages; interface activation uses set_alt().
//
// Note CDC spec table 4 lists the ACM request profile.  It requires
// encapsulated command support ... we don't handle any, and respond
// to them by stalling.  Options include get/set/clear comm features
// (not that useful) and SEND_BREAK.
//
    switch ((ctrl.bRequestType << 8) | ctrl.bRequest) {
// SET_LINE_CODING ... just read and save what the host sends
    case ((USB_DIR_OUT | USB_TYPE_CLASS | USB_RECIP_INTERFACE) << 8)
    | USB_CDC_REQ_SET_LINE_CODING:
    if (w_length != sizeof(struct usb_cdc_line_coding)
    || w_index != acm.ctrl_id)
    goto invalid;
    value = w_length;
    cdev.gadget.ep0.driver_data = acm;
    req.complete = acm_complete_set_line_coding;
    break;
// GET_LINE_CODING ... return what host sent, or initial value
    case ((USB_DIR_IN | USB_TYPE_CLASS | USB_RECIP_INTERFACE) << 8)
    | USB_CDC_REQ_GET_LINE_CODING:
    if (w_index != acm.ctrl_id)
    goto invalid;
    value = min_t(unsigned, w_length,
    sizeof(struct usb_cdc_line_coding));
    memcpy(req.buf, &acm.port_line_coding, value);
    break;
// SET_CONTROL_LINE_STATE ... save what the host sent
    case ((USB_DIR_OUT | USB_TYPE_CLASS | USB_RECIP_INTERFACE) << 8)
    | USB_CDC_REQ_SET_CONTROL_LINE_STATE:
    if (w_index != acm.ctrl_id)
    goto invalid;
    value = 0;
// FIXME we should not allow data to flow until the
// host sets the USB_CDC_CTRL_DTR bit; and when it clears
// that bit, we should return to that no-flow state.
//
    acm.port_handshake_bits = w_value;
    break;
    case ((USB_DIR_OUT | USB_TYPE_CLASS | USB_RECIP_INTERFACE) << 8)
    | USB_CDC_REQ_SEND_BREAK:
    if (w_index != acm.ctrl_id)
    goto invalid;
    acm_send_break(&acm.port, w_value);
    break;
    default:
    invalid:
    dev_vdbg(&cdev.gadget.dev,
    "invalid control req%02x.%02x v%04x i%04x l%d\n",
    ctrl.bRequestType, ctrl.bRequest,
    w_value, w_index, w_length);
    }
// respond with data transfer or status phase?
    if (value >= 0) {
    dev_dbg(&cdev.gadget.dev,
    "acm ttyGS%d req%02x.%02x v%04x i%04x l%d\n",
    acm.port_num, ctrl.bRequestType, ctrl.bRequest,
    w_value, w_index, w_length);
    req.zero = 0;
    req.length = value;
    value = usb_ep_queue(cdev.gadget.ep0, req, GFP_ATOMIC);
    if (value < 0)
    ERROR(cdev, "acm response on ttyGS%d, err %d\n",
    acm.port_num, value);
    }
// device either stalls (value < 0) or reports success
    return value;
    }
#[no_mangle]
unsafe extern "C" fn acm_set_alt(f: *mut usb_function, intf: unsigned, alt: unsigned) -> c_int {
    static int acm_set_alt(struct usb_function *f, unsigned intf, unsigned alt)
    {
    struct f_acm		*acm = func_to_acm(f);
    struct usb_composite_dev *cdev = f.config.cdev;
// we know alt == 0, so this is an activation or a reset
    if (intf == acm.ctrl_id) {
    if (acm.notify.enabled) {
    dev_vdbg(&cdev.gadget.dev,
    "reset acm control interface %d\n", intf);
    usb_ep_disable(acm.notify);
    }
    if (!acm.notify.desc)
    if (config_ep_by_speed(cdev.gadget, f, acm.notify))
    return -EINVAL;
    usb_ep_enable(acm.notify);
    } else if (intf == acm.data_id) {
    if (acm.notify.enabled) {
    dev_dbg(&cdev.gadget.dev,
    "reset acm ttyGS%d\n", acm.port_num);
    gserial_disconnect(&acm.port);
    }
    if (!acm.port.in.desc || !acm.port.out.desc) {
    dev_dbg(&cdev.gadget.dev,
    "activate acm ttyGS%d\n", acm.port_num);
    if (config_ep_by_speed(cdev.gadget, f,
    acm.port.in) ||
    config_ep_by_speed(cdev.gadget, f,
    acm.port.out)) {
    acm.port.in.desc = core::ptr::null_mut();
    acm.port.out.desc = core::ptr::null_mut();
    return -EINVAL;
    }
    }
    gserial_connect(&acm.port, acm.port_num);
    } else
    return -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn acm_disable(f: *mut usb_function) {
    static void acm_disable(struct usb_function *f)
    {
    struct f_acm	*acm = func_to_acm(f);
    struct usb_composite_dev *cdev = f.config.cdev;
    dev_dbg(&cdev.gadget.dev, "acm ttyGS%d deactivated\n", acm.port_num);
    gserial_disconnect(&acm.port);
    usb_ep_disable(acm.notify);
    }
// -------------------------------------------------------------------------
//
// acm_cdc_notify - issue CDC notification to host
// @acm: wraps host to be notified
// @type: notification type
// @value: Refer to cdc specs, wValue field.
// @data: data to be sent
// @length: size of data
// Context: irqs blocked, acm->lock held, acm_notify_req non-null
//
// Returns zero on success or a negative errno.
//
// See section 6.3.5 of the CDC 1.1 specification for information
// about the only notification we issue:  SerialState change.
//
    static int acm_cdc_notify(struct f_acm *acm, u8 type, u16 value,
    void *data, unsigned length)
    {
    struct usb_ep			*ep = acm.notify;
    struct usb_request		*req;
    struct usb_cdc_notification	*notify;
    let mut len: unsigned = sizeof(*notify) + length;
    void				*buf;
    int				status;
    req = acm.notify_req;
    acm.notify_req = core::ptr::null_mut();
    acm.pending = false;
    req.length = len;
    notify = req.buf;
    buf = notify + 1;
    notify.bmRequestType = USB_DIR_IN | USB_TYPE_CLASS
    | USB_RECIP_INTERFACE;
    notify.bNotificationType = type;
    notify.wValue = cpu_to_le16(value);
    notify.wIndex = cpu_to_le16(acm.ctrl_id);
    notify.wLength = cpu_to_le16(length);
    memcpy(buf, data, length);
// ep_queue() can complete immediately if it fills the fifo...
    spin_unlock(&acm.lock);
    status = usb_ep_queue(ep, req, GFP_ATOMIC);
    spin_lock(&acm.lock);
    if (status < 0) {
    ERROR(acm.port.func.config.cdev,
    "acm ttyGS%d can't notify serial state, %d\n",
    acm.port_num, status);
    acm.notify_req = req;
    }
    return status;
    }
#[no_mangle]
unsafe extern "C" fn acm_notify_serial_state(acm: *mut f_acm) -> c_int {
    static int acm_notify_serial_state(struct f_acm *acm)
    {
    struct usb_composite_dev *cdev = acm.port.func.config.cdev;
    int			status;
    __le16			serial_state;
    spin_lock(&acm.lock);
    if (acm.notify_req) {
    dev_dbg(&cdev.gadget.dev, "acm ttyGS%d serial state %04x\n",
    acm.port_num, acm.serial_state);
    serial_state = cpu_to_le16(acm.serial_state);
    status = acm_cdc_notify(acm, USB_CDC_NOTIFY_SERIAL_STATE,
    0, &serial_state, sizeof(acm.serial_state));
    } else {
    acm.pending = true;
    status = 0;
    }
    spin_unlock(&acm.lock);
    return status;
    }
#[no_mangle]
unsafe extern "C" fn acm_cdc_notify_complete(ep: *mut usb_ep, req: *mut usb_request) {
    static void acm_cdc_notify_complete(struct usb_ep *ep, struct usb_request *req)
    {
    struct f_acm		*acm = req.context;
    let mut doit: u8 = false;
// on this call path we do NOT hold the port spinlock,
// which is why ACM needs its own spinlock
//
    spin_lock(&acm.lock);
    if (req.status != -ESHUTDOWN)
    doit = acm.pending;
    acm.notify_req = req;
    spin_unlock(&acm.lock);
    if (doit)
    acm_notify_serial_state(acm);
    }
// connect == the TTY link is open
#[no_mangle]
unsafe extern "C" fn acm_connect(port: *mut gserial) {
    static void acm_connect(struct gserial *port)
    {
    struct f_acm		*acm = port_to_acm(port);
    acm.serial_state |= USB_CDC_SERIAL_STATE_DSR | USB_CDC_SERIAL_STATE_DCD;
    acm_notify_serial_state(acm);
    }
#[no_mangle]
unsafe extern "C" fn acm_disconnect(port: *mut gserial) {
    static void acm_disconnect(struct gserial *port)
    {
    struct f_acm		*acm = port_to_acm(port);
    acm.serial_state &= ~(USB_CDC_SERIAL_STATE_DSR | USB_CDC_SERIAL_STATE_DCD);
    acm_notify_serial_state(acm);
    }
#[no_mangle]
unsafe extern "C" fn acm_send_break(port: *mut gserial, duration: c_int) -> c_int {
    static int acm_send_break(struct gserial *port, int duration)
    {
    struct f_acm		*acm = port_to_acm(port);
    u16			state;
    state = acm.serial_state;
    state &= ~USB_CDC_SERIAL_STATE_BREAK;
    if (duration)
    state |= USB_CDC_SERIAL_STATE_BREAK;
    acm.serial_state = state;
    return acm_notify_serial_state(acm);
    }
// -------------------------------------------------------------------------
// ACM function driver setup/binding
    static int
    acm_bind(struct usb_configuration *c, struct usb_function *f)
    {
    struct usb_composite_dev *cdev = c.cdev;
    struct f_acm		*acm = func_to_acm(f);
    struct usb_string	*us;
    int			status;
    struct usb_ep		*ep;
    struct usb_request	*request __free(free_usb_request) = core::ptr::null_mut();
// REVISIT might want instance-specific strings to help
// distinguish instances ...
//
// maybe allocate device-global string IDs, and patch descriptors
    us = usb_gstrings_attach(cdev, acm_strings,
    ARRAY_SIZE(acm_string_defs));
    if (IS_ERR(us))
    return PTR_ERR(us);
    acm_control_interface_desc.iInterface = us[ACM_CTRL_IDX].id;
    acm_data_interface_desc.iInterface = us[ACM_DATA_IDX].id;
    acm_iad_descriptor.iFunction = us[ACM_IAD_IDX].id;
// allocate instance-specific interface IDs, and patch descriptors
    status = usb_interface_id(c, f);
    if (status < 0)
    return status;
    acm.ctrl_id = status;
    acm_iad_descriptor.bFirstInterface = status;
    acm_control_interface_desc.bInterfaceNumber = status;
    acm_union_desc .bMasterInterface0 = status;
    status = usb_interface_id(c, f);
    if (status < 0)
    return status;
    acm.data_id = status;
    acm_data_interface_desc.bInterfaceNumber = status;
    acm_union_desc.bSlaveInterface0 = status;
    acm_call_mgmt_descriptor.bDataInterface = status;
// allocate instance-specific endpoints
    ep = usb_ep_autoconfig(cdev.gadget, &acm_fs_in_desc);
    if (!ep)
    return -ENODEV;
    acm.port.in = ep;
    ep = usb_ep_autoconfig(cdev.gadget, &acm_fs_out_desc);
    if (!ep)
    return -ENODEV;
    acm.port.out = ep;
    ep = usb_ep_autoconfig(cdev.gadget, &acm_fs_notify_desc);
    if (!ep)
    return -ENODEV;
    acm.notify = ep;
    acm_iad_descriptor.bFunctionProtocol = acm.bInterfaceProtocol;
    acm_control_interface_desc.bInterfaceProtocol = acm.bInterfaceProtocol;
// allocate notification
    request = gs_alloc_req(ep,
    sizeof(struct usb_cdc_notification) + 2,
    GFP_KERNEL);
    if (!request)
    return -ENODEV;
    request.complete = acm_cdc_notify_complete;
    request.context = acm;
// support all relevant hardware speeds... we expect that when
// hardware is dual speed, all bulk-capable endpoints work at
// both speeds
//
    acm_hs_in_desc.bEndpointAddress = acm_fs_in_desc.bEndpointAddress;
    acm_hs_out_desc.bEndpointAddress = acm_fs_out_desc.bEndpointAddress;
    acm_hs_notify_desc.bEndpointAddress =
    acm_fs_notify_desc.bEndpointAddress;
    acm_ss_in_desc.bEndpointAddress = acm_fs_in_desc.bEndpointAddress;
    acm_ss_out_desc.bEndpointAddress = acm_fs_out_desc.bEndpointAddress;
    status = usb_assign_descriptors(f, acm_fs_function, acm_hs_function,
    acm_ss_function, acm_ss_function);
    if (status)
    return status;
    acm.notify_req = no_free_ptr(request);
    dev_dbg(&cdev.gadget.dev,
    "acm ttyGS%d: IN/%s OUT/%s NOTIFY/%s\n",
    acm.port_num,
    acm.port.in.name, acm.port.out.name,
    acm.notify.name);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn acm_unbind(c: *mut usb_configuration, f: *mut usb_function) {
    static void acm_unbind(struct usb_configuration *c, struct usb_function *f)
    {
    struct f_acm		*acm = func_to_acm(f);
    acm_string_defs[0].id = 0;
    usb_free_all_descriptors(f);
    if (acm.notify_req)
    gs_free_req(acm.notify, acm.notify_req);
    }
#[no_mangle]
unsafe extern "C" fn acm_free_func(f: *mut usb_function) {
    static void acm_free_func(struct usb_function *f)
    {
    struct f_acm		*acm = func_to_acm(f);
    struct f_serial_opts	*opts;
    opts = container_of(f.fi, struct f_serial_opts, func_inst);
    kfree(acm);
    mutex_lock(&opts.lock);
    opts.instances--;
    mutex_unlock(&opts.lock);
    }
#[no_mangle]
unsafe extern "C" fn acm_resume(f: *mut usb_function) {
    static void acm_resume(struct usb_function *f)
    {
    struct f_acm *acm = func_to_acm(f);
    gserial_resume(&acm.port);
    }
#[no_mangle]
unsafe extern "C" fn acm_suspend(f: *mut usb_function) {
    static void acm_suspend(struct usb_function *f)
    {
    struct f_acm *acm = func_to_acm(f);
    gserial_suspend(&acm.port);
    }
    static struct usb_function *acm_alloc_func(struct usb_function_instance *fi)
    {
    struct f_serial_opts *opts;
    struct f_acm *acm;
    acm = kzalloc_obj(*acm);
    if (!acm)
    return ERR_PTR(-ENOMEM);
    spin_lock_init(&acm.lock);
    acm.port.connect = acm_connect;
    acm.port.disconnect = acm_disconnect;
    acm.port.send_break = acm_send_break;
    acm.port.func.name = "acm";
    acm.port.func.strings = acm_strings;
// descriptors are per-instance copies
    acm.port.func.bind = acm_bind;
    acm.port.func.set_alt = acm_set_alt;
    acm.port.func.setup = acm_setup;
    acm.port.func.disable = acm_disable;
    opts = container_of(fi, struct f_serial_opts, func_inst);
    mutex_lock(&opts.lock);
    acm.port_num = opts.port_num;
    acm.bInterfaceProtocol = opts.protocol;
    opts.instances++;
    mutex_unlock(&opts.lock);
    acm.port.func.unbind = acm_unbind;
    acm.port.func.free_func = acm_free_func;
    acm.port.func.resume = acm_resume;
    acm.port.func.suspend = acm_suspend;
    return &acm.port.func;
    }
    static inline struct f_serial_opts *to_f_serial_opts(struct config_item *item)
    {
    return container_of(to_config_group(item), struct f_serial_opts,
    func_inst.group);
    }
#[no_mangle]
unsafe extern "C" fn acm_attr_release(item: *mut config_item) {
    static void acm_attr_release(struct config_item *item)
    {
    struct f_serial_opts *opts = to_f_serial_opts(item);
    usb_put_function_instance(&opts.func_inst);
    }
    static const struct configfs_item_operations acm_item_ops = {
    .release                = acm_attr_release,
    };

    static ssize_t f_acm_console_store(struct config_item *item,
    const char *page, size_t count)
    {
    return gserial_set_console(to_f_serial_opts(item).port_num,
    page, count);
    }
#[no_mangle]
unsafe extern "C" fn f_acm_console_show(item: *mut config_item, page: *mut c_char) -> isize {
    static ssize_t f_acm_console_show(struct config_item *item, char *page)
    {
    return gserial_get_console(to_f_serial_opts(item).port_num, page);
    }
    CONFIGFS_ATTR(f_acm_, console);

#[no_mangle]
unsafe extern "C" fn f_acm_port_num_show(item: *mut config_item, page: *mut c_char) -> isize {
    static ssize_t f_acm_port_num_show(struct config_item *item, char *page)
    {
    return sprintf(page, "%u\n", to_f_serial_opts(item).port_num);
    }
    CONFIGFS_ATTR_RO(f_acm_, port_num);
#[no_mangle]
unsafe extern "C" fn f_acm_protocol_show(item: *mut config_item, page: *mut c_char) -> isize {
    static ssize_t f_acm_protocol_show(struct config_item *item, char *page)
    {
    return sprintf(page, "%u\n", to_f_serial_opts(item).protocol);
    }
    static ssize_t f_acm_protocol_store(struct config_item *item,
    const char *page, size_t count)
    {
    struct f_serial_opts *opts = to_f_serial_opts(item);
    int ret;
    mutex_lock(&opts.lock);
    if (opts.instances) {
    ret = -EBUSY;
    goto out;
    }
    ret = kstrtou8(page, 0, &opts.protocol);
    if (ret)
    goto out;
    ret = count;
    out:
    mutex_unlock(&opts.lock);
    return ret;
    }
    CONFIGFS_ATTR(f_acm_, protocol);
    static struct configfs_attribute *acm_attrs[] = {

    &f_acm_attr_console,

    &f_acm_attr_port_num,
    &f_acm_attr_protocol,
    core::ptr::null_mut(),
    };
    static const struct config_item_type acm_func_type = {
    .ct_item_ops    = &acm_item_ops,
    .ct_attrs	= acm_attrs,
    .ct_owner       = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn acm_free_instance(fi: *mut usb_function_instance) {
    static void acm_free_instance(struct usb_function_instance *fi)
    {
    struct f_serial_opts *opts;
    opts = container_of(fi, struct f_serial_opts, func_inst);
    gserial_free_line(opts.port_num);
    mutex_destroy(&opts.lock);
    kfree(opts);
    }
    static struct usb_function_instance *acm_alloc_instance(void)
    {
    struct f_serial_opts *opts;
    int ret;
    opts = kzalloc_obj(*opts);
    if (!opts)
    return ERR_PTR(-ENOMEM);
    opts.protocol = USB_CDC_ACM_PROTO_AT_V25TER;
    opts.func_inst.free_func_inst = acm_free_instance;
    mutex_init(&opts.lock);
    ret = gserial_alloc_line(&opts.port_num);
    if (ret) {
    kfree(opts);
    return ERR_PTR(ret);
    }
    config_group_init_type_name(&opts.func_inst.group, "",
    &acm_func_type);
    return &opts.func_inst;
    }
    DECLARE_USB_FUNCTION_INIT(acm, acm_alloc_instance, acm_alloc_func);
    MODULE_DESCRIPTION("USB CDC serial (ACM) function driver");
    MODULE_LICENSE("GPL");
