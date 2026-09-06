//! Automatically rewritten from C to Rust
//! Source: drivers/usb/gadget/composite.c
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
// composite.c - infrastructure for Composite USB Gadgets
//
// Copyright (C) 2006-2008 David Brownell
//
// #define VERBOSE_DEBUG

//
// struct usb_os_string - represents OS String to be reported by a gadget
// @bLength: total length of the entire descritor, always 0x12
// @bDescriptorType: USB_DT_STRING
// @qwSignature: the OS String proper
// @bMS_VendorCode: code used by the host for subsequent requests
// @bPad: not used, must be zero
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_os_string {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub qwSignature: [__u8; OS_STRING_QW_SIGN_LEN],
    pub bMS_VendorCode: __u8,
    pub bPad: __u8,
    pub __packed: },
//
// The code in this file is utility code, used to build a gadget driver
// from one or more "function" drivers, one or more "configuration"
// objects, and a "usb_composite_driver" by gluing them together along
// with the relevant device-wide data.
//
    static struct usb_gadget_strings **get_containers_gs(
    struct usb_gadget_string_container *uc)
    {
    pub )uc->stash: *mut return (struct usb_gadget_strings,
    }
//
// function_descriptors() - get function descriptors for speed
// @f: the function
// @speed: the speed
//
// Returns the descriptors or NULL if not set.
//
    static struct usb_descriptor_header **
    function_descriptors(struct usb_function *f,
    enum usb_device_speed speed)
    {
    pub descriptors: *mut usb_descriptor_header,
//
// NOTE: we try to help gadget drivers which might not be setting
// max_speed appropriately.
//
    switch (speed) {
    case USB_SPEED_SUPER_PLUS:
    pub f->ssp_descriptors: descriptors =,
    if (descriptors)
    case USB_SPEED_SUPER:
    pub f->ss_descriptors: descriptors =,
    if (descriptors)
    case USB_SPEED_HIGH:
    pub f->hs_descriptors: descriptors =,
    if (descriptors)
    default:
    pub f->fs_descriptors: descriptors =,
    }
//
// if we can't find any descriptors at all, then this gadget deserves to
// Oops with a NULL pointer dereference
//
    pub descriptors: return,
    }
//
// next_desc() - advance to the next desc_type descriptor
// @t: currect pointer within descriptor array
// @desc_type: descriptor type
//
// Return: next desc_type descriptor or NULL
//
// Iterate over @t until either desc_type descriptor found or
// NULL (that indicates end of list) encountered
//
    static struct usb_descriptor_header**
    next_desc(struct usb_descriptor_header **t, u8 desc_type)
    {
    pub {: *mut *mut for (; t; t++),
    if ((*t).bDescriptorType == desc_type)
    pub t: return,
    }
    pub NULL: return,
    }
//
// for_each_desc() - iterate over desc_type descriptors in the
// descriptors list
// @start: pointer within descriptor array.
// @iter_desc: desc_type descriptor to use as the loop cursor
// @desc_type: wanted descriptr type
//

    pub \: for (iter_desc = next_desc(start, desc_type);,
    pub desc_type)): iter_desc; iter_desc = next_desc(iter_desc + 1,,
//
// config_ep_by_speed_and_alt() - configures the given endpoint
// according to gadget speed.
// @g: pointer to the gadget
// @f: usb function
// @_ep: the endpoint to configure
// @alt: alternate setting number
//
// Return: error code, 0 on success
//
// This function chooses the right descriptors for a given
// endpoint according to gadget speed and saves it in the
// endpoint desc field. If the endpoint already has a descriptor
// assigned to it - overwrites it with currently corresponding
// descriptor. The endpoint maxpacket field is updated according
// to the chosen descriptor.
// Note: the supplied function should hold all the descriptors
// for supported speeds
//
    int config_ep_by_speed_and_alt(struct usb_gadget *g,
    struct usb_function *f,
    struct usb_ep *_ep,
    u8 alt)
    {
    pub NULL: *mut *mut usb_endpoint_descriptor chosen_desc =,
    pub NULL: *mut *mut usb_interface_descriptor int_desc =,
    pub NULL: *mut *mut *mut usb_descriptor_header speed_desc =,
    pub NULL: *mut *mut usb_ss_ep_comp_descriptor comp_desc =,
    pub 0: int want_comp_desc =,
    pub /: *mut *mut *mut *mut usb_descriptor_header d_spd; / cursor for speed desc,
    pub cdev: *mut usb_composite_dev,
    pub false: bool incomplete_desc =,
    if (!g || !f || !_ep)
    pub -EIO: return,
// select desired speed
    switch (g.speed) {
    case USB_SPEED_SUPER_PLUS:
    if (f.ssp_descriptors) {
    pub f->ssp_descriptors: speed_desc =,
    pub 1: want_comp_desc =,
    }
    pub true: incomplete_desc =,
    case USB_SPEED_SUPER:
    if (f.ss_descriptors) {
    pub f->ss_descriptors: speed_desc =,
    pub 1: want_comp_desc =,
    }
    pub true: incomplete_desc =,
    case USB_SPEED_HIGH:
    if (f.hs_descriptors) {
    pub f->hs_descriptors: speed_desc =,
    }
    pub true: incomplete_desc =,
    default:
    pub f->fs_descriptors: speed_desc =,
    }
    pub get_gadget_data(g): cdev =,
    if (incomplete_desc)
    WARNING(cdev,
    "%s doesn't hold the descriptors for current speed\n",
// find correct alternate setting descriptor
    for_each_desc(speed_desc, d_spd, USB_DT_INTERFACE) {
    pub )*d_spd: *mut int_desc = (struct usb_interface_descriptor,
    if (int_desc.bAlternateSetting == alt) {
    pub d_spd: speed_desc =,
    pub intf_found: goto,
    }
    }
    pub -EIO: return,
    intf_found:
// find descriptors
    for_each_desc(speed_desc, d_spd, USB_DT_ENDPOINT) {
    pub )*d_spd: *mut chosen_desc = (struct usb_endpoint_descriptor,
    if (chosen_desc.bEndpointAddress == _ep.address)
    pub ep_found: goto,
    }
    pub -EIO: return,
    ep_found:
// commit results
    pub usb_endpoint_maxp(chosen_desc): _ep->maxpacket =,
    pub chosen_desc: _ep->desc =,
    pub NULL: _ep->comp_desc =,
    pub 0: _ep->maxburst =,
    pub 1: _ep->mult =,
    if (g.speed == USB_SPEED_HIGH && (usb_endpoint_xfer_isoc(_ep.desc) ||
    usb_endpoint_xfer_int(_ep.desc)))
    pub usb_endpoint_maxp_mult(_ep->desc): _ep->mult =,
    if (!want_comp_desc)
    pub 0: return,
//
// Companion descriptor should follow EP descriptor
// USB 3.0 spec, #9.6.7
//
    pub )*(++d_spd): *mut comp_desc = (struct usb_ss_ep_comp_descriptor,
    if (!comp_desc ||
    (comp_desc.bDescriptorType != USB_DT_SS_ENDPOINT_COMP))
    pub -EIO: return,
    pub comp_desc: _ep->comp_desc =,
    if (g.speed >= USB_SPEED_SUPER) {
    switch (usb_endpoint_type(_ep.desc)) {
    case USB_ENDPOINT_XFER_ISOC:
// mult: bits 1:0 of bmAttributes
    pub 1: _ep->mult = (comp_desc->bmAttributes & 0x3) +,
    case USB_ENDPOINT_XFER_BULK:
    case USB_ENDPOINT_XFER_INT:
    pub 1: _ep->maxburst = comp_desc->bMaxBurst +,
    default:
    if (comp_desc.bMaxBurst != 0)
    pub 0\n"): ERROR(cdev, "ep0 bMaxBurst must be,
    pub 1: _ep->maxburst =,
    }
    }
    pub 0: return,
    }
//
// config_ep_by_speed() - configures the given endpoint
// according to gadget speed.
// @g: pointer to the gadget
// @f: usb function
// @_ep: the endpoint to configure
//
// Return: error code, 0 on success
//
// This function chooses the right descriptors for a given
// endpoint according to gadget speed and saves it in the
// endpoint desc field. If the endpoint already has a descriptor
// assigned to it - overwrites it with currently corresponding
// descriptor. The endpoint maxpacket field is updated according
// to the chosen descriptor.
// Note: the supplied function should hold all the descriptors
// for supported speeds
//
    int config_ep_by_speed(struct usb_gadget *g,
    struct usb_function *f,
    struct usb_ep *_ep)
    {
    pub 0): return config_ep_by_speed_and_alt(g, f, _ep,,
    }
//
// usb_add_function() - add a function to a configuration
// @config: the configuration
// @function: the function being added
// Context: single threaded during gadget setup
//
// After initialization, each configuration must have one or more
// functions added to it.  Adding a function involves calling its @bind()
// method to allocate resources such as interface and string identifiers
// and endpoints.
//
// This function returns the value of the function's bind(), which is
// zero for success else a negative errno value.
//
    int usb_add_function(struct usb_configuration *config,
    struct usb_function *function)
    {
    pub -EINVAL: int value =,
    DBG(config.cdev, "adding '%s'/%p to config '%s'/%p\n",
    function.name, function,
    pub config): config->label,,
    if (!function.set_alt || !function.disable)
    pub done: goto,
    pub config: function->config =,
    pub &config->functions): list_add_tail(&function->list,,
    if (function.bind_deactivated) {
    pub usb_function_deactivate(function): value =,
    if (value)
    pub done: goto,
    }
// REVISIT *require* function->bind?
    if (function.bind) {
    pub function): value = function->bind(config,,
    if (value < 0) {
    pub NULL: function->config =,
    }
    } else
    pub 0: value =,
// We allow configurations that don't work at both speeds.
// If we run into a lowspeed Linux system, treat it the same
// as full speed ... it's the function drivers that will need
// to avoid bulk and ISO transfers.
//
    if (!config.fullspeed && function.fs_descriptors)
    pub true: config->fullspeed =,
    if (!config.highspeed && function.hs_descriptors)
    pub true: config->highspeed =,
    if (!config.superspeed && function.ss_descriptors)
    pub true: config->superspeed =,
    if (!config.superspeed_plus && function.ssp_descriptors)
    pub true: config->superspeed_plus =,
    done:
    if (value)
    DBG(config.cdev, "adding '%s'/%p -. %d\n",
    pub value): function->name, function,,
    pub value: return,
    }
#[no_mangle]
pub unsafe extern "C" fn usb_remove_function(c: *mut usb_configuration, f: *mut usb_function) {
    void usb_remove_function(struct usb_configuration *c, struct usb_function *f)
    {
    if (f.disable)
    pub 32): bitmap_zero(f->endpoints,,
    if (f.unbind)
    pub f): f->unbind(c,,
    if (f.bind_deactivated)
    }
//
// usb_function_deactivate - prevent function and gadget enumeration
// @function: the function that isn't yet ready to respond
//
// Blocks response of the gadget driver to host enumeration by
// preventing the data line pullup from being activated.  This is
// normally called during @bind() processing to change from the
// initial "ready to respond" state, or when a required resource
// becomes available.
//
// For example, drivers that serve as a passthrough to a userspace
// daemon can block enumeration unless that daemon (such as an OBEX,
// MTP, or print server) is ready to handle host requests.
//
// Not all systems support software control of their USB peripheral
// data pullups.
//
// Returns zero on success, else negative errno.
//
#[no_mangle]
pub unsafe extern "C" fn usb_function_deactivate(function: *mut usb_function) -> c_int {
    int usb_function_deactivate(struct usb_function *function)
    {
    pub function->config->cdev: *mut *mut usb_composite_dev cdev =,
    pub flags: c_ulong,
    pub 0: int status =,
    pub flags): spin_lock_irqsave(&cdev->lock,,
    if (cdev.deactivations == 0) {
    pub flags): spin_unlock_irqrestore(&cdev->lock,,
    pub usb_gadget_deactivate(cdev->gadget): status =,
    pub flags): spin_lock_irqsave(&cdev->lock,,
    }
    if (status == 0)
    pub flags): spin_unlock_irqrestore(&cdev->lock,,
    pub status: return,
    }
//
// usb_function_activate - allow function and gadget enumeration
// @function: function on which usb_function_activate() was called
//
// Reverses effect of usb_function_deactivate().  If no more functions
// are delaying their activation, the gadget driver will respond to
// host enumeration procedures.
//
// Returns zero on success, else negative errno.
//
#[no_mangle]
pub unsafe extern "C" fn usb_function_activate(function: *mut usb_function) -> c_int {
    int usb_function_activate(struct usb_function *function)
    {
    pub function->config->cdev: *mut *mut usb_composite_dev cdev =,
    pub flags: c_ulong,
    pub 0: int status =,
    pub flags): spin_lock_irqsave(&cdev->lock,,
    if (WARN_ON(cdev.deactivations == 0))
    pub -EINVAL: status =,
    else {
    if (cdev.deactivations == 0) {
    pub flags): spin_unlock_irqrestore(&cdev->lock,,
    pub usb_gadget_activate(cdev->gadget): status =,
    pub flags): spin_lock_irqsave(&cdev->lock,,
    }
    }
    pub flags): spin_unlock_irqrestore(&cdev->lock,,
    pub status: return,
    }
//
// usb_interface_id() - allocate an unused interface ID
// @config: configuration associated with the interface
// @function: function handling the interface
// Context: single threaded during gadget setup
//
// usb_interface_id() is called from usb_function.bind() callbacks to
// allocate new interface IDs.  The function driver will then store that
// ID in interface, association, CDC union, and other descriptors.  It
// will also handle any control requests targeted at that interface,
// particularly changing its altsetting via set_alt().  There may
// also be class-specific or vendor-specific requests to handle.
//
// All interface identifier should be allocated using this routine, to
// ensure that for example different functions don't wrongly assign
// different meanings to the same identifier.  Note that since interface
// identifiers are configuration-specific, functions used in more than
// one configuration (or more than once in a given configuration) need
// multiple versions of the relevant descriptors.
//
// Returns the interface ID which was allocated; or -ENODEV if no
// more interface IDs can be allocated.
//
    int usb_interface_id(struct usb_configuration *config,
    struct usb_function *function)
    {
    pub config->next_interface_id: unsigned id =,
    if (id < MAX_CONFIG_INTERFACES) {
    pub function: config->interface[id] =,
    pub 1: config->next_interface_id = id +,
    pub id: return,
    }
    pub -ENODEV: return,
    }
//
// usb_func_wakeup - sends function wake notification to the host.
// @func: function that sends the remote wakeup notification.
//
// Applicable to devices operating at enhanced superspeed when usb
// functions are put in function suspend state and armed for function
// remote wakeup. On completion, function wake notification is sent. If
// the device is in low power state it tries to bring the device to active
// state before sending the wake notification. Since it is a synchronous
// call, caller must take care of not calling it in interrupt context.
// For devices operating at lower speeds  returns negative errno.
//
// Returns zero on success, else negative errno.
//
#[no_mangle]
pub unsafe extern "C" fn usb_func_wakeup(func: *mut usb_function) -> c_int {
    int usb_func_wakeup(struct usb_function *func)
    {
    pub func->config->cdev->gadget: *mut *mut usb_gadget gadget =,
    pub id: c_int,
    if (!gadget.ops.func_wakeup)
    pub -EOPNOTSUPP: return,
    if (!func.func_wakeup_armed) {
    pub wakeup\n"): ERROR(func->config->cdev, "not armed for func remote,
    pub -EINVAL: return,
    }
    pub id++): for (id = 0; id < MAX_CONFIG_INTERFACES;,
    if (func.config.interface[id] == func)
    if (id == MAX_CONFIG_INTERFACES) {
    pub function\n"): ERROR(func->config->cdev, "Invalid,
    pub -EINVAL: return,
    }
    pub id): return gadget->ops->func_wakeup(gadget,,
    }
    static u8 encode_bMaxPower(enum usb_device_speed speed,
    struct usb_configuration *c)
    {
    pub val: unsigned,
    if (c.MaxPower || (c.bmAttributes & USB_CONFIG_ATT_SELFPOWER))
    pub c->MaxPower: val =,
    else
    pub CONFIG_USB_GADGET_VBUS_DRAW: val =,
    if (!val)
    pub 0: return,
    if (speed < USB_SPEED_SUPER)
    pub 2: return min(val, 500U) /,
    else
//
// USB 3.x supports up to 900mA, but since 900 isn't divisible
// by 8 the integral division will effectively cap to 896mA.
//
    pub 8: return min(val, 900U) /,
    }
    void check_remote_wakeup_config(struct usb_gadget *g,
    struct usb_configuration *c)
    {
    if (USB_CONFIG_ATT_WAKEUP & c.bmAttributes) {
// Reset the rw bit if gadget is not capable of it
    if (!g.wakeup_capable && g.ops.set_remote_wakeup) {
    WARN(c.cdev, "Clearing wakeup bit for config c.%d\n",
    pub ~USB_CONFIG_ATT_WAKEUP: c->bmAttributes &=,
    }
    }
    }
    static int config_buf(struct usb_configuration *config,
    enum usb_device_speed speed, void *buf, u8 type)
    {
    pub buf: *mut *mut usb_config_descriptor c =,
    pub USB_DT_CONFIG_SIZE: *mut *mut void next = buf +,
    pub len: c_int,
    pub f: *mut usb_function,
    pub status: c_int,
    pub USB_DT_CONFIG_SIZE: len = USB_COMP_EP0_BUFSIZ -,
// write the config descriptor
    pub buf: c =,
    pub USB_DT_CONFIG_SIZE: c->bLength =,
    pub type: c->bDescriptorType =,
// wTotalLength is written later
    pub config->next_interface_id: c->bNumInterfaces =,
    pub config->bConfigurationValue: c->bConfigurationValue =,
    pub config->iConfiguration: c->iConfiguration =,
    pub config->bmAttributes: c->bmAttributes = USB_CONFIG_ATT_ONE |,
    pub config): c->bMaxPower = encode_bMaxPower(speed,,
// There may be e.g. OTG descriptors
    if (config.descriptors) {
    status = usb_descriptor_fillbuf(next, len,
    if (status < 0)
    pub status: return,
    pub status: len -=,
    pub status: next +=,
    }
// add each function's descriptors
    list_for_each_entry(f, &config.functions, list) {
    pub descriptors: *mut usb_descriptor_header,
    pub speed): descriptors = function_descriptors(f,,
    if (!descriptors)
    status = usb_descriptor_fillbuf(next, len,
    pub descriptors): *const *const *const (struct usb_descriptor_header ),
    if (status < 0)
    pub status: return,
    pub status: len -=,
    pub status: next +=,
    }
    pub buf: len = next -,
    pub cpu_to_le16(len): c->wTotalLength =,
    pub len: return,
    }
#[no_mangle]
unsafe extern "C" fn config_desc(cdev: *mut usb_composite_dev, w_value: unsigned) -> c_int {
    static int config_desc(struct usb_composite_dev *cdev, unsigned w_value)
    {
    pub cdev->gadget: *mut *mut usb_gadget gadget =,
    pub c: *mut usb_configuration,
    pub pos: *mut list_head,
    pub 8: u8 type = w_value >>,
    pub USB_SPEED_UNKNOWN: enum usb_device_speed speed =,
    if (gadget.speed >= USB_SPEED_SUPER)
    pub gadget->speed: speed =,
#[no_mangle]
pub unsafe extern "C" fn if(_arg: gadget_is_dualspeed(gadget)) -> else {
    pub 0: int hs =,
    if (gadget.speed == USB_SPEED_HIGH)
    pub 1: hs =,
    if (type == USB_DT_OTHER_SPEED_CONFIG)
    pub !hs: hs =,
    if (hs)
    pub USB_SPEED_HIGH: speed =,
    }
// This is a lookup by config *INDEX*
    pub 0xff: w_value &=,
    pub &cdev->configs: pos =,
    pub cdev->os_desc_config: c =,
    if (c)
    pub check_config: goto,
    while ((pos = pos.next) !=  &cdev.configs) {
    pub list): *mut *mut c = list_entry(pos, typeof(c),,
// skip OS Descriptors config which is handled separately
    if (c == cdev.os_desc_config)
    check_config:
// ignore configs that won't work at this speed
    switch (speed) {
    case USB_SPEED_SUPER_PLUS:
    if (!c.superspeed_plus)
    case USB_SPEED_SUPER:
    if (!c.superspeed)
    case USB_SPEED_HIGH:
    if (!c.highspeed)
    default:
    if (!c.fullspeed)
    }
    if (w_value == 0)
    pub type): return config_buf(c, speed, cdev->req->buf,,
    }
    pub -EINVAL: return,
    }
#[no_mangle]
unsafe extern "C" fn count_configs(cdev: *mut usb_composite_dev, type: unsigned) -> c_int {
    static int count_configs(struct usb_composite_dev *cdev, unsigned type)
    {
    pub cdev->gadget: *mut *mut usb_gadget gadget =,
    pub c: *mut usb_configuration,
    pub 0: unsigned count =,
    pub 0: int hs =,
    pub 0: int ss =,
    pub 0: int ssp =,
    if (gadget_is_dualspeed(gadget)) {
    if (gadget.speed == USB_SPEED_HIGH)
    pub 1: hs =,
    if (gadget.speed == USB_SPEED_SUPER)
    pub 1: ss =,
    if (gadget.speed == USB_SPEED_SUPER_PLUS)
    pub 1: ssp =,
    if (type == USB_DT_DEVICE_QUALIFIER)
    pub !hs: hs =,
    }
    list_for_each_entry(c, &cdev.configs, list) {
// ignore configs that won't work at this speed
    if (ssp) {
    if (!c.superspeed_plus)
    } else if (ss) {
    if (!c.superspeed)
    } else if (hs) {
    if (!c.highspeed)
    } else {
    if (!c.fullspeed)
    }
    }
    pub count: return,
    }
//
// bos_desc() - prepares the BOS descriptor.
// @cdev: pointer to usb_composite device to generate the bos
// descriptor for
//
// This function generates the BOS (Binary Device Object)
// descriptor and its device capabilities descriptors. The BOS
// descriptor should be supported by a SuperSpeed device.
//
#[no_mangle]
unsafe extern "C" fn bos_desc(cdev: *mut usb_composite_dev) -> c_int {
    static int bos_desc(struct usb_composite_dev *cdev)
    {
    pub usb_ext: *mut usb_ext_cap_descriptor,
    pub dcd_config_params: usb_dcd_config_params,
    pub cdev->req->buf: *mut *mut usb_bos_descriptor bos =,
    pub 0: unsigned int besl =,
    pub USB_DT_BOS_SIZE: bos->bLength =,
    pub USB_DT_BOS: bos->bDescriptorType =,
    pub cpu_to_le16(USB_DT_BOS_SIZE): bos->wTotalLength =,
    pub 0: bos->bNumDeviceCaps =,
// Get Controller configuration
    if (cdev.gadget.ops.get_config_params) {
    cdev.gadget.ops.get_config_params(cdev.gadget,
    } else {
    dcd_config_params.besl_baseline =
    dcd_config_params.besl_deep =
    dcd_config_params.bU1devExitLat =
    dcd_config_params.bU2DevExitLat =
    }
    if (dcd_config_params.besl_baseline != USB_DEFAULT_BESL_UNSPECIFIED)
    besl = USB_BESL_BASELINE_VALID |
    if (dcd_config_params.besl_deep != USB_DEFAULT_BESL_UNSPECIFIED)
    besl |= USB_BESL_DEEP_VALID |
//
// A SuperSpeed device shall include the USB2.0 extension descriptor
// and shall support LPM when operating in USB2.0 HS mode.
//
    if (cdev.gadget.lpm_capable) {
    pub le16_to_cpu(bos->wTotalLength): usb_ext = cdev->req->buf +,
    pub USB_DT_USB_EXT_CAP_SIZE): le16_add_cpu(&bos->wTotalLength,,
    pub USB_DT_USB_EXT_CAP_SIZE: usb_ext->bLength =,
    pub USB_DT_DEVICE_CAPABILITY: usb_ext->bDescriptorType =,
    pub USB_CAP_TYPE_EXT: usb_ext->bDevCapabilityType =,
    usb_ext.bmAttributes = cpu_to_le32(USB_LPM_SUPPORT |
    pub besl): USB_BESL_SUPPORT |,
    }
//
// The Superspeed USB Capability descriptor shall be implemented by all
// SuperSpeed devices.
//
    if (gadget_is_superspeed(cdev.gadget)) {
    pub ss_cap: *mut usb_ss_cap_descriptor,
    pub le16_to_cpu(bos->wTotalLength): ss_cap = cdev->req->buf +,
    pub USB_DT_USB_SS_CAP_SIZE): le16_add_cpu(&bos->wTotalLength,,
    pub USB_DT_USB_SS_CAP_SIZE: ss_cap->bLength =,
    pub USB_DT_DEVICE_CAPABILITY: ss_cap->bDescriptorType =,
    pub USB_SS_CAP_TYPE: ss_cap->bDevCapabilityType =,
    pub /: *mut *mut ss_cap->bmAttributes = 0; / LTM is not supported yet,
    ss_cap.wSpeedSupported = cpu_to_le16(USB_LOW_SPEED_OPERATION |
    USB_FULL_SPEED_OPERATION |
    USB_HIGH_SPEED_OPERATION |
    pub USB_LOW_SPEED_OPERATION: ss_cap->bFunctionalitySupport =,
    pub dcd_config_params.bU1devExitLat: ss_cap->bU1devExitLat =,
    pub dcd_config_params.bU2DevExitLat: ss_cap->bU2DevExitLat =,
    }
// The SuperSpeedPlus USB Device Capability descriptor
    if (gadget_is_superspeed_plus(cdev.gadget)) {
    pub ssp_cap: *mut usb_ssp_cap_descriptor,
    pub 1: u8 ssac =,
    pub ssic: u8,
    pub i: c_int,
    if (cdev.gadget.max_ssp_rate == USB_SSP_GEN_2x2)
    pub 3: ssac =,
//
// Paired RX and TX sublink speed attributes share
// the same SSID.
//
    pub 1: ssic = (ssac + 1) / 2 -,
    pub le16_to_cpu(bos->wTotalLength): ssp_cap = cdev->req->buf +,
    pub USB_DT_USB_SSP_CAP_SIZE(ssac)): le16_add_cpu(&bos->wTotalLength,,
    pub USB_DT_USB_SSP_CAP_SIZE(ssac): ssp_cap->bLength =,
    pub USB_DT_DEVICE_CAPABILITY: ssp_cap->bDescriptorType =,
    pub USB_SSP_CAP_TYPE: ssp_cap->bDevCapabilityType =,
    pub 0: ssp_cap->bReserved =,
    pub 0: ssp_cap->wReserved =,
    ssp_cap.bmAttributes =
    cpu_to_le32(FIELD_PREP(USB_SSP_SUBLINK_SPEED_ATTRIBS, ssac) |
    pub ssic)): FIELD_PREP(USB_SSP_SUBLINK_SPEED_IDS,,
    ssp_cap.wFunctionalitySupport =
    cpu_to_le16(FIELD_PREP(USB_SSP_MIN_SUBLINK_SPEED_ATTRIBUTE_ID, 0) |
    FIELD_PREP(USB_SSP_MIN_RX_LANE_COUNT, 1) |
    pub 1)): FIELD_PREP(USB_SSP_MIN_TX_LANE_COUNT,,
//
// Use 1 SSID if the gadget supports up to gen2x1 or not
// specified:
// - SSID 0 for symmetric RX/TX sublink speed of 10 Gbps.
//
// Use 1 SSID if the gadget supports up to gen1x2:
// - SSID 0 for symmetric RX/TX sublink speed of 5 Gbps.
//
// Use 2 SSIDs if the gadget supports up to gen2x2:
// - SSID 0 for symmetric RX/TX sublink speed of 5 Gbps.
// - SSID 1 for symmetric RX/TX sublink speed of 10 Gbps.
//
    pub {: for (i = 0; i < ssac + 1; i++),
    pub ssid: u8,
    pub mantissa: u8,
    pub type: u8,
    pub 1: ssid = i >>,
    if (cdev.gadget.max_ssp_rate == USB_SSP_GEN_2x1 ||
    cdev.gadget.max_ssp_rate == USB_SSP_GEN_UNKNOWN)
    pub 10: mantissa =,
    else
    pub ssid: mantissa = 5 <<,
    if (i % 2)
    pub USB_SSP_SUBLINK_SPEED_ST_SYM_TX: type =,
    else
    pub USB_SSP_SUBLINK_SPEED_ST_SYM_RX: type =,
    ssp_cap.bmSublinkSpeedAttr[i] =
    cpu_to_le32(FIELD_PREP(USB_SSP_SUBLINK_SPEED_SSID, ssid) |
    FIELD_PREP(USB_SSP_SUBLINK_SPEED_LSE,
    USB_SSP_SUBLINK_SPEED_LSE_GBPS) |
    FIELD_PREP(USB_SSP_SUBLINK_SPEED_ST, type) |
    FIELD_PREP(USB_SSP_SUBLINK_SPEED_LP,
    USB_SSP_SUBLINK_SPEED_LP_SSP) |
    pub mantissa)): FIELD_PREP(USB_SSP_SUBLINK_SPEED_LSM,,
    }
    }
// The WebUSB Platform Capability descriptor
    if (cdev.use_webusb) {
    pub webusb_cap: *mut usb_plat_dev_cap_descriptor,
    pub webusb_cap_data: *mut usb_webusb_cap_data,
    pub WEBUSB_UUID: guid_t webusb_uuid =,
    pub le16_to_cpu(bos->wTotalLength): webusb_cap = cdev->req->buf +,
    pub webusb_cap->CapabilityData: *mut *mut webusb_cap_data = (struct usb_webusb_cap_data ),
    le16_add_cpu(&bos.wTotalLength,
    pub USB_DT_USB_PLAT_DEV_CAP_SIZE(USB_WEBUSB_CAP_DATA_SIZE): webusb_cap->bLength =,
    pub USB_DT_DEVICE_CAPABILITY: webusb_cap->bDescriptorType =,
    pub USB_PLAT_DEV_CAP_TYPE: webusb_cap->bDevCapabilityType =,
    pub 0: webusb_cap->bReserved =,
    pub &webusb_uuid): export_guid(webusb_cap->UUID,,
    if (cdev.bcd_webusb_version != 0)
    pub cpu_to_le16(cdev->bcd_webusb_version): webusb_cap_data->bcdVersion =,
    else
    pub WEBUSB_VERSION_1_00: webusb_cap_data->bcdVersion =,
    pub cdev->b_webusb_vendor_code: webusb_cap_data->bVendorCode =,
    if (strnlen(cdev.landing_page, sizeof(cdev.landing_page)) > 0)
    pub WEBUSB_LANDING_PAGE_PRESENT: webusb_cap_data->iLandingPage =,
    else
    pub WEBUSB_LANDING_PAGE_NOT_PRESENT: webusb_cap_data->iLandingPage =,
    }
    pub le16_to_cpu(bos->wTotalLength): return,
    }
#[no_mangle]
unsafe extern "C" fn device_qual(cdev: *mut usb_composite_dev) {
    static void device_qual(struct usb_composite_dev *cdev)
    {
    pub cdev->req->buf: *mut *mut usb_qualifier_descriptor qual =,
    pub sizeof(*qual): *mut qual->bLength =,
    pub USB_DT_DEVICE_QUALIFIER: qual->bDescriptorType =,
// POLICY: same bcdUSB and device type info at both speeds
    pub cdev->desc.bcdUSB: qual->bcdUSB =,
    pub cdev->desc.bDeviceClass: qual->bDeviceClass =,
    pub cdev->desc.bDeviceSubClass: qual->bDeviceSubClass =,
    pub cdev->desc.bDeviceProtocol: qual->bDeviceProtocol =,
// ASSUME same EP0 fifo size at both speeds
    pub cdev->gadget->ep0->maxpacket: qual->bMaxPacketSize0 =,
    pub USB_DT_DEVICE_QUALIFIER): qual->bNumConfigurations = count_configs(cdev,,
    pub 0: qual->bRESERVED =,
    }
// -------------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn reset_config(cdev: *mut usb_composite_dev) {
    static void reset_config(struct usb_composite_dev *cdev)
    {
    pub f: *mut usb_function,
    pub config\n"): DBG(cdev, "reset,
    list_for_each_entry(f, &cdev.config.functions, list) {
    if (f.disable)
// Section 9.1.1.6, disable remote wakeup when device is reset
    pub false: f->func_wakeup_armed =,
    pub 32): bitmap_zero(f->endpoints,,
    }
    pub NULL: cdev->config =,
    pub 0: cdev->delayed_status =,
    }
    static int set_config(struct usb_composite_dev *cdev,
    const struct usb_ctrlrequest *ctrl, unsigned number)
    {
    pub cdev->gadget: *mut *mut usb_gadget gadget =,
    pub iter: *mut *mut usb_configuration c = NULL,,
    pub -EINVAL: int result =,
    pub 100: unsigned power = gadget_is_otg(gadget) ? 8 :,
    pub tmp: c_int,
    if (number) {
    list_for_each_entry(iter, &cdev.configs, list) {
    if (iter.bConfigurationValue != number)
//
// We disable the FDs of the previous
// configuration only if the new configuration
// is a valid one
//
    if (cdev.config)
    pub iter: c =,
    pub 0: result =,
    }
    if (result < 0)
    pub done: goto,
    } else { /* Zero configuration value - need to reset the config */
    if (cdev.config)
    pub 0: result =,
    }
    DBG(cdev, "%s config #%d: %s\n",
    usb_speed_string(gadget.speed),
    pub "unconfigured"): number, c ? c->label :,
    if (!c)
    pub done: goto,
    pub USB_STATE_CONFIGURED): usb_gadget_set_state(gadget,,
    pub c: cdev->config =,
// Initialize all interfaces by setting them to altsetting zero.
    pub {: for (tmp = 0; tmp < MAX_CONFIG_INTERFACES; tmp++),
    pub c->interface[tmp]: *mut *mut usb_function f =,
    pub descriptors: *mut usb_descriptor_header,
    if (!f)
//
// Record which endpoints are used by the function. This is used
// to dispatch control requests targeted at that endpoint to the
// function's setup callback instead of the current
// configuration's setup callback.
//
    pub gadget->speed): descriptors = function_descriptors(f,,
    pub {: *mut *mut for (; descriptors; ++descriptors),
    pub ep: *mut usb_endpoint_descriptor,
    pub addr: c_int,
    if ((*descriptors).bDescriptorType != USB_DT_ENDPOINT)
    pub )*descriptors: *mut ep = (struct usb_endpoint_descriptor,
    addr = ((ep.bEndpointAddress & 0x80) >> 3)
    pub usb_endpoint_num(ep): |,
    pub f->endpoints): set_bit(addr,,
    }
    pub 0): result = f->set_alt(f, tmp,,
    if (result < 0) {
    DBG(cdev, "interface %d (%s/%p) alt 0 -. %d\n",
    pub result): tmp, f->name, f,,
    pub done: goto,
    }
    if (result == USB_GADGET_DELAYED_STATUS) {
    DBG(cdev,
    "%s: interface %d (%s) requested delayed status\n",
    pub f->name): __func__, tmp,,
    DBG(cdev, "delayed_status count %d\n",
    }
    }
// when we return, be sure our power usage is valid
    if (c.MaxPower || (c.bmAttributes & USB_CONFIG_ATT_SELFPOWER))
    pub c->MaxPower: power =,
    else
    pub CONFIG_USB_GADGET_VBUS_DRAW: power =,
    if (gadget.speed < USB_SPEED_SUPER)
    pub 500U): power = min(power,,
    else
    pub 900U): power = min(power,,
    if (USB_CONFIG_ATT_WAKEUP & c.bmAttributes)
    pub 1): usb_gadget_set_remote_wakeup(gadget,,
    else
    pub 0): usb_gadget_set_remote_wakeup(gadget,,
    done:
    if (power > USB_SELF_POWER_VBUS_MAX_DRAW ||
    (c && !(c.bmAttributes & USB_CONFIG_ATT_SELFPOWER)))
    else
    pub power): usb_gadget_vbus_draw(gadget,,
    if (result >= 0 && cdev.delayed_status)
    pub USB_GADGET_DELAYED_STATUS: result =,
    pub result: return,
    }
    int usb_add_config_only(struct usb_composite_dev *cdev,
    struct usb_configuration *config)
    {
    pub c: *mut usb_configuration,
    if (!config.bConfigurationValue)
    pub -EINVAL: return,
// Prevent duplicate configuration identifiers
    list_for_each_entry(c, &cdev.configs, list) {
    if (c.bConfigurationValue == config.bConfigurationValue)
    pub -EBUSY: return,
    }
    pub cdev: config->cdev =,
    pub &cdev->configs): list_add_tail(&config->list,,
    pub 0: config->next_interface_id =,
    pub sizeof(config->interface)): memset(config->interface, 0,,
    pub 0: return,
    }
//
// usb_add_config() - add a configuration to a device.
// @cdev: wraps the USB gadget
// @config: the configuration, with bConfigurationValue assigned
// @bind: the configuration's bind function
// Context: single threaded during gadget setup
//
// One of the main tasks of a composite @bind() routine is to
// add each of the configurations it supports, using this routine.
//
// This function returns the value of the configuration's @bind(), which
// is zero for success else a negative errno value.  Binding configurations
// assigns global resources including string IDs, and per-configuration
// resources such as interface IDs and endpoints.
//
    int usb_add_config(struct usb_composite_dev *cdev,
    struct usb_configuration *config,
    int (*bind)(struct usb_configuration *))
    {
    pub -EINVAL: int status =,
    if (!bind)
    pub done: goto,
    DBG(cdev, "adding config #%u '%s'/%p\n",
    config.bConfigurationValue,
    pub config): config->label,,
    pub config): status = usb_add_config_only(cdev,,
    if (status)
    pub done: goto,
    pub bind(config): status =,
    if (status == 0)
    pub usb_gadget_check_config(cdev->gadget): status =,
    if (status < 0) {
    while (!list_empty(&config.functions)) {
    pub f: *mut usb_function,
    f = list_first_entry(&config.functions,
    pub list): usb_function,,
    if (f.unbind) {
    DBG(cdev, "unbind function '%s'/%p\n",
    pub f): f->name,,
    pub f): f->unbind(config,,
// may free memory for "f"
    }
    }
    pub NULL: config->cdev =,
    } else {
    pub i: unsigned,
    DBG(cdev, "cfg %d/%p speeds:%s%s%s%s\n",
    config.bConfigurationValue, config,
    config.superspeed_plus ? " superplus" : "",
    config.superspeed ? " super" : "",
    config.highspeed ? " high" : "",
    config.fullspeed
    ? (gadget_is_dualspeed(cdev.gadget)
    ? " full"
    : " full/low")
    pub ""): :,
    pub {: for (i = 0; i < MAX_CONFIG_INTERFACES; i++),
    pub config->interface[i]: *mut *mut usb_function f =,
    if (!f)
    DBG(cdev, "  interface %d = %s/%p\n",
    pub f): i, f->name,,
    }
    }
// set_alt(), or next bind(), sets up ep->claimed as needed
    done:
    if (status)
    DBG(cdev, "added config '%s'/%u -. %d\n", config.label,
    pub status): config->bConfigurationValue,,
    pub status: return,
    }
    static void remove_config(struct usb_composite_dev *cdev,
    struct usb_configuration *config)
    {
    while (!list_empty(&config.functions)) {
    pub f: *mut usb_function,
    f = list_first_entry(&config.functions,
    pub list): usb_function,,
    pub f): usb_remove_function(config,,
    }
    if (config.unbind) {
    pub config): DBG(cdev, "unbind config '%s'/%p\n", config->label,,
// may free memory for "c"
    }
    }
// -------------------------------------------------------------------------
// We support strings in multiple languages ... string descriptor zero
// says which languages are supported.  The typical case will be that
// only one language (probably English) is used, with i18n handled on
// the host side.
//
#[no_mangle]
unsafe extern "C" fn collect_langs(sp: *mut usb_gadget_strings, buf: *mut __le16) {
    static void collect_langs(struct usb_gadget_strings **sp, __le16 *buf)
    {
    pub s: *const usb_gadget_strings,
    pub language: __le16,
    pub tmp: *mut __le16,
    while (*sp) {
    pub sp: *mut s =,
    pub cpu_to_le16(s->language): language =,
    pub {: *mut *mut for (tmp = buf; tmp && tmp < &buf[USB_MAX_STRING_LEN]; tmp++),
    if (*tmp == language)
    pub repeat: goto,
    }
// tmp++ = language;
    repeat:
    }
    }
    static int lookup_string(
    struct usb_gadget_strings	**sp,
    void				*buf,
    u16				language,
    int				id
    )
    {
    pub s: *mut usb_gadget_strings,
    pub value: c_int,
    while (*sp) {
    pub sp++: *mut s =,
    if (s.language != language)
    pub buf): value = usb_gadget_get_string(s, id,,
    if (value > 0)
    pub value: return,
    }
    pub -EINVAL: return,
    }
    static int get_string(struct usb_composite_dev *cdev,
    void *buf, u16 language, int id)
    {
    pub cdev->driver: *mut *mut usb_composite_driver composite =,
    pub uc: *mut usb_gadget_string_container,
    pub c: *mut usb_configuration,
    pub f: *mut usb_function,
    pub len: c_int,
// Yes, not only is USB's i18n support probably more than most
// folk will ever care about ... also, it's all supported here.
// (Except for UTF8 support for Unicode's "Astral Planes".)
//
// 0 == report all available language codes
    if (id == 0) {
    pub buf: *mut *mut usb_string_descriptor s =,
    pub sp: *mut usb_gadget_strings,
    pub 256): memset(s, 0,,
    pub USB_DT_STRING: s->bDescriptorType =,
    pub composite->strings: sp =,
    if (sp)
    pub s->wData): collect_langs(sp,,
    list_for_each_entry(c, &cdev.configs, list) {
    pub c->strings: sp =,
    if (sp)
    pub s->wData): collect_langs(sp,,
    list_for_each_entry(f, &c.functions, list) {
    pub f->strings: sp =,
    if (sp)
    pub s->wData): collect_langs(sp,,
    }
    }
    list_for_each_entry(uc, &cdev.gstrings, list) {
    pub sp: *mut usb_gadget_strings,
    pub get_containers_gs(uc): sp =,
    pub s->wData): collect_langs(sp,,
    }
    pub len++): for (len = 0; len <= USB_MAX_STRING_LEN && s->wData[len];,
    if (!len)
    pub -EINVAL: return,
    pub 1): *mut *mut s->bLength = 2  (len +,
    pub s->bLength: return,
    }
    if (cdev.use_os_string && language == 0 && id == OS_STRING_IDX) {
    pub buf: *mut *mut usb_os_string b =,
    pub sizeof(*b): *mut b->bLength =,
    pub USB_DT_STRING: b->bDescriptorType =,
    compiletime_assert(
    sizeof(b.qwSignature) == sizeof(cdev.qw_sign),
    pub qw_sign"): "qwSignature size must be equal to,
    pub sizeof(b->qwSignature)): memcpy(&b->qwSignature, cdev->qw_sign,,
    pub cdev->b_vendor_code: b->bMS_VendorCode =,
    pub 0: b->bPad =,
    pub sizeof(*b): *mut return,
    }
    list_for_each_entry(uc, &cdev.gstrings, list) {
    pub sp: *mut usb_gadget_strings,
    pub get_containers_gs(uc): sp =,
    pub id): len = lookup_string(sp, buf, language,,
    if (len > 0)
    pub len: return,
    }
// String IDs are device-scoped, so we look up each string
// table we're told about.  These lookups are infrequent;
// simpler-is-better here.
//
    if (composite.strings) {
    pub id): len = lookup_string(composite->strings, buf, language,,
    if (len > 0)
    pub len: return,
    }
    list_for_each_entry(c, &cdev.configs, list) {
    if (c.strings) {
    pub id): len = lookup_string(c->strings, buf, language,,
    if (len > 0)
    pub len: return,
    }
    list_for_each_entry(f, &c.functions, list) {
    if (!f.strings)
    pub id): len = lookup_string(f->strings, buf, language,,
    if (len > 0)
    pub len: return,
    }
    }
    pub -EINVAL: return,
    }
//
// usb_string_id() - allocate an unused string ID
// @cdev: the device whose string descriptor IDs are being allocated
// Context: single threaded during gadget setup
//
// @usb_string_id() is called from bind() callbacks to allocate
// string IDs.  Drivers for functions, configurations, or gadgets will
// then store that ID in the appropriate descriptors and string table.
//
// All string identifier should be allocated using this,
// @usb_string_ids_tab() or @usb_string_ids_n() routine, to ensure
// that for example different functions don't wrongly assign different
// meanings to the same identifier.
//
#[no_mangle]
pub unsafe extern "C" fn usb_string_id(cdev: *mut usb_composite_dev) -> c_int {
    int usb_string_id(struct usb_composite_dev *cdev)
    {
    if (cdev.next_string_id < 254) {
// string id 0 is reserved by USB spec for list of
// supported languages
// 255 reserved as well? -- mina86
    pub cdev->next_string_id: return,
    }
    pub -ENODEV: return,
    }
//
// usb_string_ids_tab() - allocate unused string IDs in batch
// @cdev: the device whose string descriptor IDs are being allocated
// @str: an array of usb_string objects to assign numbers to
// Context: single threaded during gadget setup
//
// @usb_string_ids() is called from bind() callbacks to allocate
// string IDs.  Drivers for functions, configurations, or gadgets will
// then copy IDs from the string table to the appropriate descriptors
// and string table for other languages.
//
// All string identifier should be allocated using this,
// @usb_string_id() or @usb_string_ids_n() routine, to ensure that for
// example different functions don't wrongly assign different meanings
// to the same identifier.
//
#[no_mangle]
pub unsafe extern "C" fn usb_string_ids_tab(cdev: *mut usb_composite_dev, str: *mut usb_string) -> c_int {
    int usb_string_ids_tab(struct usb_composite_dev *cdev, struct usb_string *str)
    {
    pub cdev->next_string_id: int next =,
    pub {: for (; str->s; ++str),
    if (unlikely(next >= 254))
    pub -ENODEV: return,
    pub ++next: str->id =,
    }
    pub next: cdev->next_string_id =,
    pub 0: return,
    }
    static struct usb_gadget_string_container *copy_gadget_strings(
    struct usb_gadget_strings **sp, unsigned n_gstrings,
    unsigned n_strings)
    {
    pub uc: *mut usb_gadget_string_container,
    pub gs_array: *mut usb_gadget_strings,
    pub gs: *mut usb_gadget_strings,
    pub s: *mut usb_string,
    pub mem: unsigned,
    pub n_gs: unsigned,
    pub n_s: unsigned,
    pub stash: *mut c_void,
    pub sizeof(*uc): *mut mem =,
    pub 1): *mut *mut *mut mem += sizeof(void )  (n_gstrings +,
    pub n_gstrings: *mut *mut mem += sizeof(struct usb_gadget_strings),
    pub (n_gstrings): *mut *mut *mut mem += sizeof(struct usb_string)  (n_strings + 1),
    pub GFP_KERNEL): uc = kmalloc(mem,,
    if (!uc)
    pub ERR_PTR(-ENOMEM): return,
    pub get_containers_gs(uc): gs_array =,
    pub uc->stash: stash =,
    pub 1): *mut *mut *mut stash += sizeof(void )  (n_gstrings +,
    pub {: for (n_gs = 0; n_gs < n_gstrings; n_gs++),
    pub org_s: *mut usb_string,
    pub stash: gs_array[n_gs] =,
    pub gs_array: [gs =; n_gs],
    pub usb_gadget_strings): stash += sizeof(struct,
    pub sp[n_gs]->language: gs->language =,
    pub stash: gs->strings =,
    pub sp[n_gs]->strings: org_s =,
    pub {: for (n_s = 0; n_s < n_strings; n_s++),
    pub stash: s =,
    pub usb_string): stash += sizeof(struct,
    if (org_s.s)
    pub org_s->s: s->s =,
    else
    pub "": s->s =,
    }
    pub stash: s =,
    pub NULL: s->s =,
    pub usb_string): stash += sizeof(struct,
    }
    pub NULL: gs_array[n_gs] =,
    pub uc: return,
    }
//
// usb_gstrings_attach() - attach gadget strings to a cdev and assign ids
// @cdev: the device whose string descriptor IDs are being allocated
// and attached.
// @sp: an array of usb_gadget_strings to attach.
// @n_strings: number of entries in each usb_strings array (sp[]->strings)
//
// This function will create a deep copy of usb_gadget_strings and usb_string
// and attach it to the cdev. The actual string (usb_string.s) will not be
// copied but only a referenced will be made. The struct usb_gadget_strings
// array may contain multiple languages and should be NULL terminated.
// The ->language pointer of each struct usb_gadget_strings has to contain the
// same amount of entries.
// For instance: sp[0] is en-US, sp[1] is es-ES. It is expected that the first
// usb_string entry of es-ES contains the translation of the first usb_string
// entry of en-US. Therefore both entries become the same id assign.
//
    struct usb_string *usb_gstrings_attach(struct usb_composite_dev *cdev,
    struct usb_gadget_strings **sp, unsigned n_strings)
    {
    pub uc: *mut usb_gadget_string_container,
    pub n_gs: *mut usb_gadget_strings,
    pub 0: unsigned n_gstrings =,
    pub i: unsigned,
    pub ret: c_int,
    pub i++): for (i = 0; sp[i];,
    if (!n_gstrings)
    pub ERR_PTR(-EINVAL): return,
    pub n_strings): uc = copy_gadget_strings(sp, n_gstrings,,
    if (IS_ERR(uc))
    pub ERR_CAST(uc): return,
    pub get_containers_gs(uc): n_gs =,
    pub n_gs[0]->strings): ret = usb_string_ids_tab(cdev,,
    if (ret)
    pub err: goto,
    pub {: for (i = 1; i < n_gstrings; i++),
    pub m_s: *mut usb_string,
    pub s: *mut usb_string,
    pub n: unsigned,
    pub n_gs[0]->strings: m_s =,
    pub n_gs[i]->strings: s =,
    pub {: for (n = 0; n < n_strings; n++),
    pub m_s->id: s->id =,
    }
    }
    pub &cdev->gstrings): list_add_tail(&uc->list,,
    pub n_gs[0]->strings: return,
    err:
    pub ERR_PTR(ret): return,
    }
//
// usb_string_ids_n() - allocate unused string IDs in batch
// @c: the device whose string descriptor IDs are being allocated
// @n: number of string IDs to allocate
// Context: single threaded during gadget setup
//
// Returns the first requested ID.  This ID and next @n-1 IDs are now
// valid IDs.  At least provided that @n is non-zero because if it
// is, returns last requested ID which is now very useful information.
//
// @usb_string_ids_n() is called from bind() callbacks to allocate
// string IDs.  Drivers for functions, configurations, or gadgets will
// then store that ID in the appropriate descriptors and string table.
//
// All string identifier should be allocated using this,
// @usb_string_id() or @usb_string_ids_n() routine, to ensure that for
// example different functions don't wrongly assign different meanings
// to the same identifier.
//
#[no_mangle]
pub unsafe extern "C" fn usb_string_ids_n(c: *mut usb_composite_dev, n: unsigned) -> c_int {
    int usb_string_ids_n(struct usb_composite_dev *c, unsigned n)
    {
    pub c->next_string_id: unsigned next =,
    if (unlikely(n > 254 || (unsigned)next + n > 254))
    pub -ENODEV: return,
    pub n: c->next_string_id +=,
    pub 1: return next +,
    }
// -------------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn composite_setup_complete(ep: *mut usb_ep, req: *mut usb_request) {
    static void composite_setup_complete(struct usb_ep *ep, struct usb_request *req)
    {
    pub cdev: *mut usb_composite_dev,
    if (req.status || req.actual != req.length)
    DBG((struct usb_composite_dev *) ep.driver_data,
    "setup complete -. %d, %d/%d\n",
    pub req->length): req->status, req->actual,,
//
// REVIST The same ep0 requests are shared with function drivers
// so they don't have to maintain the same ->complete() stubs.
//
// Because of that, we need to check for the validity of ->context
// here, even though we know we've set it to something useful.
//
    if (!req.context)
    pub req->context: cdev =,
    if (cdev.req == req)
    pub false: cdev->setup_pending =,
#[no_mangle]
pub unsafe extern "C" fn if(req: cdev->os_desc_req ==) -> else {
    else if (cdev.os_desc_req == req)
    pub false: cdev->os_desc_pending =,
    else
    pub req): WARN(1, "unknown request %p\n",,
    }
    static int composite_ep0_queue(struct usb_composite_dev *cdev,
    struct usb_request *req, gfp_t gfp_flags)
    {
    pub ret: c_int,
    pub gfp_flags): ret = usb_ep_queue(cdev->gadget->ep0, req,,
    if (ret == 0) {
    if (cdev.req == req)
    pub true: cdev->setup_pending =,
#[no_mangle]
pub unsafe extern "C" fn if(req: cdev->os_desc_req ==) -> else {
    else if (cdev.os_desc_req == req)
    pub true: cdev->os_desc_pending =,
    else
    pub req): WARN(1, "unknown request %p\n",,
    }
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn count_ext_compat(c: *mut usb_configuration) -> c_int {
    static int count_ext_compat(struct usb_configuration *c)
    {
    pub res: int i,,
    pub 0: res =,
    pub {: for (i = 0; i < c->next_interface_id; ++i),
    pub f: *mut usb_function,
    pub j: c_int,
    pub c->interface[i]: f =,
    pub {: for (j = 0; j < f->os_desc_n; ++j),
    pub d: *mut usb_os_desc,
    if (i != f.os_desc_table[j].if_id)
    pub f->os_desc_table[j].os_desc: d =,
    if (d && d.ext_compat_id)
    }
    }
    pub 255): BUG_ON(res >,
    pub res: return,
    }
#[no_mangle]
unsafe extern "C" fn fill_ext_compat(c: *mut usb_configuration, buf: *mut u8) -> c_int {
    static int fill_ext_compat(struct usb_configuration *c, u8 *buf)
    {
    pub count: int i,,
    pub 16: count =,
    pub 16: buf +=,
    pub {: for (i = 0; i < c->next_interface_id; ++i),
    pub f: *mut usb_function,
    pub j: c_int,
    pub c->interface[i]: f =,
    pub {: for (j = 0; j < f->os_desc_n; ++j),
    pub d: *mut usb_os_desc,
    if (i != f.os_desc_table[j].if_id)
    pub f->os_desc_table[j].os_desc: d =,
    if (d && d.ext_compat_id) {
// buf++ = i;
// buf++ = 0x01;
    pub 16): memcpy(buf, d->ext_compat_id,,
    pub 22: buf +=,
    } else {
// buf = 0x01;
    pub 23: buf +=,
    }
    pub 24: count +=,
    if (count + 24 >= USB_COMP_EP0_OS_DESC_BUFSIZ)
    pub count: return,
    }
    }
    pub count: return,
    }
#[no_mangle]
unsafe extern "C" fn count_ext_prop(c: *mut usb_configuration, interface: c_int) -> c_int {
    static int count_ext_prop(struct usb_configuration *c, int interface)
    {
    pub f: *mut usb_function,
    pub j: c_int,
    pub c->interface[interface]: f =,
    pub {: for (j = 0; j < f->os_desc_n; ++j),
    pub d: *mut usb_os_desc,
    if (interface != f.os_desc_table[j].if_id)
    pub f->os_desc_table[j].os_desc: d =,
    if (d && d.ext_compat_id)
    pub d->ext_prop_count: return,
    }
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn len_ext_prop(c: *mut usb_configuration, interface: c_int) -> c_int {
    static int len_ext_prop(struct usb_configuration *c, int interface)
    {
    pub f: *mut usb_function,
    pub d: *mut usb_os_desc,
    pub res: int j,,
    pub /: *mut *mut res = 10; / header length,
    pub c->interface[interface]: f =,
    pub {: for (j = 0; j < f->os_desc_n; ++j),
    if (interface != f.os_desc_table[j].if_id)
    pub f->os_desc_table[j].os_desc: d =,
    if (d)
    pub 4096): return min(res + d->ext_prop_len,,
    }
    pub res: return,
    }
#[no_mangle]
unsafe extern "C" fn fill_ext_prop(c: *mut usb_configuration, interface: c_int, buf: *mut u8) -> c_int {
    static int fill_ext_prop(struct usb_configuration *c, int interface, u8 *buf)
    {
    pub f: *mut usb_function,
    pub d: *mut usb_os_desc,
    pub ext_prop: *mut usb_os_desc_ext_prop,
    pub ret: int j, count, n,,
    pub c->interface[interface]: f =,
    pub /: *mut *mut count = 10; / header length,
    pub 10: buf +=,
    pub {: for (j = 0; j < f->os_desc_n; ++j),
    if (interface != f.os_desc_table[j].if_id)
    pub f->os_desc_table[j].os_desc: d =,
    if (d)
    list_for_each_entry(ext_prop, &d.ext_prop, entry) {
    n = ext_prop.data_len +
    pub 14: ext_prop->name_len +,
    if (count + n >= USB_COMP_EP0_OS_DESC_BUFSIZ)
    pub count: return,
    pub n): usb_ext_prop_put_size(buf,,
    pub ext_prop->type): usb_ext_prop_put_type(buf,,
    ret = usb_ext_prop_put_name(buf, ext_prop.name,
    if (ret < 0)
    pub ret: return,
    switch (ext_prop.type) {
    case USB_EXT_PROP_UNICODE:
    case USB_EXT_PROP_UNICODE_ENV:
    case USB_EXT_PROP_UNICODE_LINK:
    usb_ext_prop_put_unicode(buf, ret,
    ext_prop.data,
    case USB_EXT_PROP_BINARY:
    usb_ext_prop_put_binary(buf, ret,
    ext_prop.data,
    case USB_EXT_PROP_LE32:
// not implemented
    case USB_EXT_PROP_BE32:
// not implemented
    default:
    pub -EINVAL: return,
    }
    pub n: buf +=,
    pub n: count +=,
    }
    }
    pub count: return,
    }
//
// The setup() callback implements all the ep0 functionality that's
// not handled lower down, in hardware or the hardware driver(like
// device and endpoint feature flags, and their status).  It's all
// housekeeping for the gadget function we're implementing.  Most of
// the work is in config and function specific setup.
//
    int
    composite_setup(struct usb_gadget *gadget, const struct usb_ctrlrequest *ctrl)
    {
    pub get_gadget_data(gadget): *mut *mut usb_composite_dev cdev =,
    pub cdev->req: *mut *mut usb_request req =,
    pub -EOPNOTSUPP: int value =,
    pub 0: int status =,
    pub le16_to_cpu(ctrl->wIndex): u16 w_index =,
    pub 0xFF: u8 intf = w_index &,
    pub le16_to_cpu(ctrl->wValue): u16 w_value =,
    pub le16_to_cpu(ctrl->wLength): u16 w_length =,
    pub NULL: *mut *mut usb_function f =,
    pub iter: *mut usb_function,
    pub endp: u8,
    if (w_length > USB_COMP_EP0_BUFSIZ) {
    if (ctrl.bRequestType & USB_DIR_IN) {
// Cast away the const, we are going to overwrite on purpose.
    pub )&ctrl->wLength: *mut *mut __le16 temp = (__le16,
// temp = cpu_to_le16(USB_COMP_EP0_BUFSIZ);
    pub USB_COMP_EP0_BUFSIZ: w_length =,
    } else {
    pub done: goto,
    }
    }
// partial re-init of the response message; the function or the
// gadget might need to intercept e.g. a control-OUT completion
// when we delegate to it.
//
    pub 0: req->zero =,
    pub cdev: req->context =,
    pub composite_setup_complete: req->complete =,
    pub 0: req->length =,
    pub cdev: gadget->ep0->driver_data =,
//
// Don't let non-standard requests match any of the cases below
// by accident.
//
    if ((ctrl.bRequestType & USB_TYPE_MASK) != USB_TYPE_STANDARD)
    pub unknown: goto,
    switch (ctrl.bRequest) {
// we handle all standard USB descriptors
    case USB_REQ_GET_DESCRIPTOR:
    if (ctrl.bRequestType != USB_DIR_IN)
    pub unknown: goto,
    switch (w_value >> 8) {
    case USB_DT_DEVICE:
    cdev.desc.bNumConfigurations =
    pub USB_DT_DEVICE): count_configs(cdev,,
    cdev.desc.bMaxPacketSize0 =
    if (gadget_is_superspeed(gadget)) {
    if (gadget.speed >= USB_SPEED_SUPER) {
    pub cpu_to_le16(0x0320): cdev->desc.bcdUSB =,
    pub 9: cdev->desc.bMaxPacketSize0 =,
    } else {
    pub cpu_to_le16(0x0210): cdev->desc.bcdUSB =,
    }
    } else {
    if (gadget.lpm_capable || cdev.use_webusb)
    pub cpu_to_le16(0x0201): cdev->desc.bcdUSB =,
    else
    pub cpu_to_le16(0x0200): cdev->desc.bcdUSB =,
    }
    pub sizeof(cdev->desc)): value = min_t(u16, w_length,,
    pub value): memcpy(req->buf, &cdev->desc,,
    case USB_DT_DEVICE_QUALIFIER:
    if (!gadget_is_dualspeed(gadget) ||
    gadget.speed >= USB_SPEED_SUPER)
    value = min_t(int, w_length,
    pub usb_qualifier_descriptor)): sizeof(struct,
    case USB_DT_OTHER_SPEED_CONFIG:
    if (!gadget_is_dualspeed(gadget) ||
    gadget.speed >= USB_SPEED_SUPER)
    case USB_DT_CONFIG:
    pub w_value): value = config_desc(cdev,,
    if (value >= 0)
    pub value): value = min_t(u16, w_length,,
    case USB_DT_STRING:
    value = get_string(cdev, req.buf,
    pub 0xff): w_index, w_value &,
    if (value >= 0)
    pub value): value = min_t(u16, w_length,,
    case USB_DT_BOS:
    if (gadget_is_superspeed(gadget) ||
    gadget.lpm_capable || cdev.use_webusb) {
    pub bos_desc(cdev): value =,
    pub value): value = min_t(u16, w_length,,
    }
    case USB_DT_OTG:
    if (gadget_is_otg(gadget)) {
    pub config: *mut usb_configuration,
    pub 0: int otg_desc_len =,
    if (cdev.config)
    pub cdev->config: config =,
    else
    config = list_first_entry_or_null(
    &cdev.configs,
    struct usb_configuration,
    if (!config)
    pub done: goto,
    if (gadget.otg_caps &&
    (gadget.otg_caps.otg_rev >= 0x0200))
    otg_desc_len += sizeof(
    pub usb_otg20_descriptor): struct,
    else
    otg_desc_len += sizeof(
    pub usb_otg_descriptor): struct,
    pub otg_desc_len): value = min_t(int, w_length,,
    pub value): memcpy(req->buf, config->descriptors[0],,
    }
    }
// any number of configs can work
    case USB_REQ_SET_CONFIGURATION:
    if (ctrl.bRequestType != 0)
    pub unknown: goto,
    if (gadget_is_otg(gadget)) {
    if (gadget.a_hnp_support)
    pub available\n"): DBG(cdev, "HNP,
#[no_mangle]
pub unsafe extern "C" fn if(_arg: gadget->a_alt_hnp_support) -> else {
    else if (gadget.a_alt_hnp_support)
    pub port\n"): DBG(cdev, "HNP on another,
    else
    pub inactive\n"): VDBG(cdev, "HNP,
    }
    pub w_value): value = set_config(cdev, ctrl,,
    case USB_REQ_GET_CONFIGURATION:
    if (ctrl.bRequestType != USB_DIR_IN)
    pub unknown: goto,
    if (cdev.config)
// (u8 *)req->buf = cdev->config->bConfigurationValue;
    else
// (u8 *)req->buf = 0;
    pub 1): value = min_t(u16, w_length,,
// function drivers must handle get/set altsetting
    case USB_REQ_SET_INTERFACE:
    if (ctrl.bRequestType != USB_RECIP_INTERFACE)
    pub unknown: goto,
    if (!cdev.config || intf >= MAX_CONFIG_INTERFACES)
    pub cdev->config->interface[intf]: f =,
    if (!f)
//
// If there's no get_alt() method, we know only altsetting zero
// works. There is no need to check if set_alt() is not NULL
// as we check this in usb_add_function().
//
    if (w_value && !f.get_alt)
    pub w_value): value = f->set_alt(f, w_index,,
    if (value == USB_GADGET_DELAYED_STATUS) {
    DBG(cdev,
    "%s: interface %d (%s) requested delayed status\n",
    pub f->name): __func__, intf,,
    DBG(cdev, "delayed_status count %d\n",
    }
    case USB_REQ_GET_INTERFACE:
    if (ctrl.bRequestType != (USB_DIR_IN|USB_RECIP_INTERFACE))
    pub unknown: goto,
    if (!cdev.config || intf >= MAX_CONFIG_INTERFACES)
    pub cdev->config->interface[intf]: f =,
    if (!f)
// lots of interfaces only need altsetting zero...
    pub 0: value = f->get_alt ? f->get_alt(f, w_index) :,
    if (value < 0)
// ((u8 *)req->buf) = value;
    pub 1): value = min_t(u16, w_length,,
    case USB_REQ_GET_STATUS:
    if (gadget_is_otg(gadget) && gadget.hnp_polling_support &&
    (w_index == OTG_STS_SELECTOR)) {
    if (ctrl.bRequestType != (USB_DIR_IN |
    USB_RECIP_DEVICE))
    pub unknown: goto,
// ((u8 *)req->buf) = gadget->host_request_flag;
    pub 1: value =,
    }
//
// USB 3.0 additions:
// Function driver should handle get_status request. If such cb
// wasn't supplied we respond with default value = 0
// Note: function driver should supply such cb only for the
// first interface of the function
//
    if (!gadget_is_superspeed(gadget))
    pub unknown: goto,
    if (ctrl.bRequestType != (USB_DIR_IN | USB_RECIP_INTERFACE))
    pub unknown: goto,
    pub /: *mut *mut value = 2; / This is the length of the get_status reply,
    pub req->buf): put_unaligned_le16(0,,
    if (!cdev.config || intf >= MAX_CONFIG_INTERFACES)
    pub cdev->config->interface[intf]: f =,
    if (!f)
    if (f.get_status) {
    pub f->get_status(f): status =,
    if (status < 0)
// if D5 is not set, then device is not wakeup capable
    if (!(f.config.bmAttributes & USB_CONFIG_ATT_WAKEUP))
    pub USB_INTRF_STAT_FUNC_RW): status &= ~(USB_INTRF_STAT_FUNC_RW_CAP |,
    }
    pub req->buf): put_unaligned_le16(status & 0x0000ffff,,
//
// Function drivers should handle SetFeature/ClearFeature
// (FUNCTION_SUSPEND) request. function_suspend cb should be supplied
// only for the first interface of the function
//
    case USB_REQ_CLEAR_FEATURE:
    case USB_REQ_SET_FEATURE:
    if (!gadget_is_superspeed(gadget))
    pub unknown: goto,
    if (ctrl.bRequestType != (USB_DIR_OUT | USB_RECIP_INTERFACE))
    pub unknown: goto,
    switch (w_value) {
    case USB_INTRF_FUNC_SUSPEND:
    if (!cdev.config || intf >= MAX_CONFIG_INTERFACES)
    pub cdev->config->interface[intf]: f =,
    if (!f)
    pub 0: value =,
    if (f.func_suspend) {
    pub 8): value = f->func_suspend(f, w_index >>,
// SetFeature(FUNCTION_SUSPEND)
    } else if (ctrl.bRequest == USB_REQ_SET_FEATURE) {
    if (!(f.config.bmAttributes &
    USB_CONFIG_ATT_WAKEUP) &&
    (w_index & USB_INTRF_FUNC_SUSPEND_RW))
    f.func_wakeup_armed = !!(w_index &
    if (w_index & USB_INTRF_FUNC_SUSPEND_LP) {
    if (f.suspend && !f.func_suspended) {
    pub true: f->func_suspended =,
    }
//
// Handle cases where host sends function resume
// through SetFeature(FUNCTION_SUSPEND) but low power
// bit reset
//
    } else {
    if (f.resume && f.func_suspended) {
    pub false: f->func_suspended =,
    }
    }
// ClearFeature(FUNCTION_SUSPEND)
    } else if (ctrl.bRequest == USB_REQ_CLEAR_FEATURE) {
    pub false: f->func_wakeup_armed =,
    if (f.resume && f.func_suspended) {
    pub false: f->func_suspended =,
    }
    }
    if (value < 0) {
    ERROR(cdev,
    "func_suspend() returned error %d\n",
    pub 0: value =,
    }
    }
    default:
    unknown:
//
// OS descriptors handling
//
    if (cdev.use_os_string && cdev.os_desc_config &&
    (ctrl.bRequestType & USB_TYPE_VENDOR) &&
    ctrl.bRequest == cdev.b_vendor_code) {
    pub os_desc_cfg: *mut usb_configuration,
    pub buf: *mut u8,
    pub interface: c_int,
    pub 0: int count =,
    pub cdev->os_desc_req: req =,
    pub cdev: req->context =,
    pub composite_setup_complete: req->complete =,
    pub req->buf: buf =,
    pub cdev->os_desc_config: os_desc_cfg =,
    pub USB_COMP_EP0_OS_DESC_BUFSIZ): w_length = min_t(u16, w_length,,
    pub w_length): memset(buf, 0,,
    pub 0x01: buf[5] =,
    switch (ctrl.bRequestType & USB_RECIP_MASK) {
//
// The Microsoft CompatID OS Descriptor Spec(w_index = 0x4) and
// Extended Prop OS Desc Spec(w_index = 0x5) state that the
// HighByte of wValue is the InterfaceNumber and the LowByte is
// the PageNumber. This high/low byte ordering is incorrectly
// documented in the Spec. USB analyzer output on the below
// request packets show the high/low byte inverted i.e LowByte
// is the InterfaceNumber and the HighByte is the PageNumber.
// Since we dont support >64KB CompatID/ExtendedProp descriptors,
// PageNumber is set to 0. Hence verify that the HighByte is 0
// for below two cases.
//
    case USB_RECIP_DEVICE:
    if (w_index != 0x4 || (w_value >> 8))
    pub w_index: buf[6] =,
// Number of ext compat interfaces
    pub count_ext_compat(os_desc_cfg): count =,
    pub count: buf[8] =,
    pub /: *mut *mut *mut count = 24; / 24 B/ext compat desc,
    pub /: *mut *mut count += 16; / header,
    pub buf): put_unaligned_le32(count,,
    pub w_length: value =,
    if (w_length > 0x10) {
    pub buf): value = fill_ext_compat(os_desc_cfg,,
    pub value): value = min_t(u16, w_length,,
    }
    case USB_RECIP_INTERFACE:
    if (w_index != 0x5 || (w_value >> 8))
    pub 0xFF: interface = w_value &,
    if (interface >= MAX_CONFIG_INTERFACES ||
    !os_desc_cfg.interface[interface])
    pub w_index: buf[6] =,
    count = count_ext_prop(os_desc_cfg,
    pub 8): put_unaligned_le16(count, buf +,
    count = len_ext_prop(os_desc_cfg,
    pub buf): put_unaligned_le32(count,,
    pub w_length: value =,
    if (w_length > 0x0A) {
    value = fill_ext_prop(os_desc_cfg,
    pub buf): interface,,
    if (value >= 0)
    pub value): value = min_t(u16, w_length,,
    }
    }
    pub check_value: goto,
    }
//
// WebUSB URL descriptor handling, following:
// https://wicg.github.io/webusb/#device-requests
//
    if (cdev.use_webusb &&
    ctrl.bRequestType == (USB_DIR_IN | USB_TYPE_VENDOR) &&
    w_index == WEBUSB_GET_URL &&
    w_value == WEBUSB_LANDING_PAGE_PRESENT &&
    ctrl.bRequest == cdev.b_webusb_vendor_code) {
    pub landing_page_length: c_uint,
    pub landing_page_offset: c_uint,
    struct webusb_url_descriptor *url_descriptor =
    pub )cdev->req->buf: *mut (struct webusb_url_descriptor,
    pub WEBUSB_URL_DESCRIPTOR_TYPE: url_descriptor->bDescriptorType =,
    if (strncasecmp(cdev.landing_page, "https://",  8) == 0) {
    pub 8: landing_page_offset =,
    pub WEBUSB_URL_SCHEME_HTTPS: url_descriptor->bScheme =,
    } else if (strncasecmp(cdev.landing_page, "http://", 7) == 0) {
    pub 7: landing_page_offset =,
    pub WEBUSB_URL_SCHEME_HTTP: url_descriptor->bScheme =,
    } else {
    pub 0: landing_page_offset =,
    pub WEBUSB_URL_SCHEME_NONE: url_descriptor->bScheme =,
    }
    landing_page_length = strnlen(cdev.landing_page,
    sizeof(url_descriptor.URL)
    pub landing_page_offset): - WEBUSB_URL_DESCRIPTOR_HEADER_LENGTH +,
    if (w_length < WEBUSB_URL_DESCRIPTOR_HEADER_LENGTH)
    pub landing_page_offset: landing_page_length =,
    else if (w_length <
    WEBUSB_URL_DESCRIPTOR_HEADER_LENGTH + landing_page_length)
    landing_page_length = w_length
    pub landing_page_offset: - WEBUSB_URL_DESCRIPTOR_HEADER_LENGTH +,
    memcpy(url_descriptor.URL,
    cdev.landing_page + landing_page_offset,
    pub landing_page_offset): landing_page_length -,
    url_descriptor.bLength = landing_page_length
    pub WEBUSB_URL_DESCRIPTOR_HEADER_LENGTH: - landing_page_offset +,
    pub url_descriptor->bLength: value =,
    pub check_value: goto,
    }
    VDBG(cdev,
    "non-core control req%02x.%02x v%04x i%04x l%d\n",
    ctrl.bRequestType, ctrl.bRequest,
    pub w_length): w_value, w_index,,
// functions always handle their interfaces and endpoints...
// punt other recipients (other, WUSB, ...) to the current
// configuration code.
//
    if (cdev.config) {
    list_for_each_entry(f, &cdev.config.functions, list)
    if (f.req_match &&
    f.req_match(f, ctrl, false))
    pub try_fun_setup: goto,
    } else {
    pub c: *mut usb_configuration,
    list_for_each_entry(c, &cdev.configs, list)
    list_for_each_entry(f, &c.functions, list)
    if (f.req_match &&
    f.req_match(f, ctrl, true))
    pub try_fun_setup: goto,
    }
    pub NULL: f =,
    switch (ctrl.bRequestType & USB_RECIP_MASK) {
    case USB_RECIP_INTERFACE:
    if (!cdev.config || intf >= MAX_CONFIG_INTERFACES)
    pub cdev->config->interface[intf]: f =,
    case USB_RECIP_ENDPOINT:
    if (!cdev.config)
    pub 0x0f): endp = ((w_index & 0x80) >> 3) | (w_index &,
    list_for_each_entry(iter, &cdev.config.functions, list) {
    if (test_bit(endp, iter.endpoints)) {
    pub iter: f =,
    }
    }
    }
    try_fun_setup:
    if (f && f.setup)
    pub ctrl): value = f->setup(f,,
    else {
    pub c: *mut usb_configuration,
    pub cdev->config: c =,
    if (!c)
    pub done: goto,
// try current config's setup
    if (c.setup) {
    pub ctrl): value = c->setup(c,,
    pub done: goto,
    }
// try the only function in the current config
    if (!list_is_singular(&c.functions))
    pub done: goto,
    f = list_first_entry(&c.functions, struct usb_function,
    if (f.setup)
    pub ctrl): value = f->setup(f,,
    }
    pub done: goto,
    }
    check_value:
// respond with data transfer before status phase?
    if (value >= 0 && value != USB_GADGET_DELAYED_STATUS) {
    pub value: req->length =,
    pub cdev: req->context =,
    pub w_length: req->zero = value <,
    pub GFP_ATOMIC): value = composite_ep0_queue(cdev, req,,
    if (value < 0) {
    pub value): DBG(cdev, "ep_queue --> %d\n",,
    pub 0: req->status =,
    pub req): composite_setup_complete(gadget->ep0,,
    }
    } else if (value == USB_GADGET_DELAYED_STATUS && w_length != 0) {
    WARN(cdev,
    "%s: Delayed status not supported for w_length != 0",
    }
    done:
// device either stalls (value < 0) or reports success
    pub value: return,
    }
#[no_mangle]
unsafe extern "C" fn __composite_disconnect(gadget: *mut usb_gadget) {
    static void __composite_disconnect(struct usb_gadget *gadget)
    {
    pub get_gadget_data(gadget): *mut *mut usb_composite_dev cdev =,
    pub flags: c_ulong,
// REVISIT:  should we have config and device level
// disconnect callbacks?
//
    pub flags): spin_lock_irqsave(&cdev->lock,,
    pub 0: cdev->suspended =,
    if (cdev.config)
    if (cdev.driver.disconnect)
    pub flags): spin_unlock_irqrestore(&cdev->lock,,
    }
#[no_mangle]
pub unsafe extern "C" fn composite_disconnect(gadget: *mut usb_gadget) {
    void composite_disconnect(struct usb_gadget *gadget)
    {
    pub 0): usb_gadget_vbus_draw(gadget,,
    }
#[no_mangle]
pub unsafe extern "C" fn composite_reset(gadget: *mut usb_gadget) {
    void composite_reset(struct usb_gadget *gadget)
    {
//
// Section 1.4.13 Standard Downstream Port of the USB battery charging
// specification v1.2 states that a device connected on a SDP shall only
// draw at max 100mA while in a connected, but unconfigured state.
//
    pub 100): usb_gadget_vbus_draw(gadget,,
    }
// -------------------------------------------------------------------------
    static ssize_t suspended_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    pub dev_to_usb_gadget(dev): *mut *mut usb_gadget gadget =,
    pub get_gadget_data(gadget): *mut *mut usb_composite_dev cdev =,
    pub cdev->suspended): return sprintf(buf, "%d\n",,
    }
    pub DEVICE_ATTR_RO(suspended): static,
#[no_mangle]
unsafe extern "C" fn __composite_unbind(gadget: *mut usb_gadget, unbind_driver: bool) {
    static void __composite_unbind(struct usb_gadget *gadget, bool unbind_driver)
    {
    pub get_gadget_data(gadget): *mut *mut usb_composite_dev cdev =,
    pub cdev->driver->strings[0]: *mut *mut usb_gadget_strings gstr =,
    pub gstr->strings: *mut *mut usb_string dev_str =,
// composite_disconnect() must already have been called
// by the underlying peripheral controller driver!
// so there's no i/o concurrency that could affect the
// state protected by cdev->lock.
//
    while (!list_empty(&cdev.configs)) {
    pub c: *mut usb_configuration,
    c = list_first_entry(&cdev.configs,
    pub list): usb_configuration,,
    pub c): remove_config(cdev,,
    }
    if (cdev.driver.unbind && unbind_driver)
    if (dev_str[USB_GADGET_MANUFACTURER_IDX].s == cdev.def_manufacturer)
    pub "": dev_str[USB_GADGET_MANUFACTURER_IDX].s =,
    pub NULL): set_gadget_data(gadget,,
    }
#[no_mangle]
unsafe extern "C" fn composite_unbind(gadget: *mut usb_gadget) {
    static void composite_unbind(struct usb_gadget *gadget)
    {
    pub true): __composite_unbind(gadget,,
    }
    static void update_unchanged_dev_desc(struct usb_device_descriptor *new,
    const struct usb_device_descriptor *old)
    {
    pub idVendor: __le16,
    pub idProduct: __le16,
    pub bcdDevice: __le16,
    pub iSerialNumber: u8,
    pub iManufacturer: u8,
    pub iProduct: u8,
//
// these variables may have been set in
// usb_composite_overwrite_options()
//
    pub new->idVendor: idVendor =,
    pub new->idProduct: idProduct =,
    pub new->bcdDevice: bcdDevice =,
    pub new->iSerialNumber: iSerialNumber =,
    pub new->iManufacturer: iManufacturer =,
    pub new->iProduct: iProduct =,
// new = *old;
    if (idVendor)
    pub idVendor: new->idVendor =,
    if (idProduct)
    pub idProduct: new->idProduct =,
    if (bcdDevice)
    pub bcdDevice: new->bcdDevice =,
    else
    pub cpu_to_le16(get_default_bcdDevice()): new->bcdDevice =,
    if (iSerialNumber)
    pub iSerialNumber: new->iSerialNumber =,
    if (iManufacturer)
    pub iManufacturer: new->iManufacturer =,
    if (iProduct)
    pub iProduct: new->iProduct =,
    }
    int composite_dev_prepare(struct usb_composite_driver *composite,
    struct usb_composite_dev *cdev)
    {
    pub cdev->gadget: *mut *mut usb_gadget gadget =,
    pub -ENOMEM: int ret =,
// preallocate control response and buffer
    pub GFP_KERNEL): cdev->req = usb_ep_alloc_request(gadget->ep0,,
    if (!cdev.req)
    pub -ENOMEM: return,
    pub GFP_KERNEL): cdev->req->buf = kzalloc(USB_COMP_EP0_BUFSIZ,,
    if (!cdev.req.buf)
    pub fail: goto,
    pub &dev_attr_suspended): ret = device_create_file(&gadget->dev,,
    if (ret)
    pub fail_dev: goto,
    pub composite_setup_complete: cdev->req->complete =,
    pub cdev: cdev->req->context =,
    pub cdev: gadget->ep0->driver_data =,
    pub composite: cdev->driver =,
//
// As per USB compliance update, a device that is actively drawing
// more than 100mA from USB must report itself as bus-powered in
// the GetStatus(DEVICE) call.
//
    if (CONFIG_USB_GADGET_VBUS_DRAW <= USB_SELF_POWER_VBUS_MAX_DRAW)
// interface and string IDs start at zero via kzalloc.
// we force endpoints to start unassigned; few controller
// drivers will zero ep->driver_data.
//
    pub 0: return,
    fail_dev:
    fail:
    pub cdev->req): usb_ep_free_request(gadget->ep0,,
    pub NULL: cdev->req =,
    pub ret: return,
    }
    int composite_os_desc_req_prepare(struct usb_composite_dev *cdev,
    struct usb_ep *ep0)
    {
    pub 0: int ret =,
    pub GFP_KERNEL): cdev->os_desc_req = usb_ep_alloc_request(ep0,,
    if (!cdev.os_desc_req) {
    pub -ENOMEM: ret =,
    pub end: goto,
    }
    cdev.os_desc_req.buf = kmalloc(USB_COMP_EP0_OS_DESC_BUFSIZ,
    if (!cdev.os_desc_req.buf) {
    pub -ENOMEM: ret =,
    pub cdev->os_desc_req): usb_ep_free_request(ep0,,
//
// Set os_desc_req to NULL so that composite_dev_cleanup()
// will not try to free it again.
//
    pub NULL: cdev->os_desc_req =,
    pub end: goto,
    }
    pub cdev: cdev->os_desc_req->context =,
    pub composite_setup_complete: cdev->os_desc_req->complete =,
    end:
    pub ret: return,
    }
#[no_mangle]
pub unsafe extern "C" fn composite_dev_cleanup(cdev: *mut usb_composite_dev) {
    void composite_dev_cleanup(struct usb_composite_dev *cdev)
    {
    pub tmp: *mut *mut usb_gadget_string_container uc,,
    pub tmp_ep: *mut *mut usb_ep ep,,
    list_for_each_entry_safe(uc, tmp, &cdev.gstrings, list) {
    }
    if (cdev.os_desc_req) {
    if (cdev.os_desc_pending)
    pub cdev->os_desc_req): usb_ep_dequeue(cdev->gadget->ep0,,
    pub NULL: cdev->os_desc_req->buf =,
    pub cdev->os_desc_req): usb_ep_free_request(cdev->gadget->ep0,,
    pub NULL: cdev->os_desc_req =,
    }
    if (cdev.req) {
    if (cdev.setup_pending)
    pub cdev->req): usb_ep_dequeue(cdev->gadget->ep0,,
    pub NULL: cdev->req->buf =,
    pub cdev->req): usb_ep_free_request(cdev->gadget->ep0,,
    pub NULL: cdev->req =,
    }
    pub 0: cdev->next_string_id =,
    pub &dev_attr_suspended): device_remove_file(&cdev->gadget->dev,,
//
// Some UDC backends have a dynamic EP allocation scheme.
//
// In that case, the dispose() callback is used to notify the
// backend that the EPs are no longer in use.
//
// Note: The UDC backend can remove the EP from the ep_list as
// a result, so we need to use the _safe list iterator.
//
    list_for_each_entry_safe(ep, tmp_ep,
    &cdev.gadget.ep_list, ep_list) {
    if (ep.ops.dispose)
    }
    }
    static int composite_bind(struct usb_gadget *gadget,
    struct usb_gadget_driver *gdriver)
    {
    pub cdev: *mut usb_composite_dev,
    pub to_cdriver(gdriver): *mut *mut usb_composite_driver composite =,
    pub -ENOMEM: int status =,
    pub kzalloc_obj(*cdev): *mut cdev =,
    if (!cdev)
    pub status: return,
    pub gadget: cdev->gadget =,
    pub cdev): set_gadget_data(gadget,,
    pub cdev): status = composite_dev_prepare(composite,,
    if (status)
    pub fail: goto,
// composite gadget needs to assign strings for whole device (like
// serial number), register function drivers, potentially update
// power state and consumption, etc
//
    pub composite->bind(cdev): status =,
    if (status < 0)
    pub fail: goto,
    if (cdev.use_os_string) {
    pub gadget->ep0): status = composite_os_desc_req_prepare(cdev,,
    if (status)
    pub fail: goto,
    }
    pub composite->dev): update_unchanged_dev_desc(&cdev->desc,,
// has userspace failed to provide a serial number?
    if (composite.needs_serial && !cdev.desc.iSerialNumber)
    pub iSerialNumber\n"): WARNING(cdev, "userspace failed to provide,
    pub composite->name): INFO(cdev, "%s ready\n",,
    pub 0: return,
    fail:
    pub false): __composite_unbind(gadget,,
    pub status: return,
    }
// -------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn composite_suspend(gadget: *mut usb_gadget) {
    void composite_suspend(struct usb_gadget *gadget)
    {
    pub get_gadget_data(gadget): *mut *mut usb_composite_dev cdev =,
    pub f: *mut usb_function,
// REVISIT:  should we have config level
// suspend/resume callbacks?
//
    pub "suspend\n"): DBG(cdev,,
    if (cdev.config) {
    list_for_each_entry(f, &cdev.config.functions, list) {
    if (f.suspend)
    }
    }
    if (cdev.driver.suspend)
    pub 1: cdev->suspended =,
    if (cdev.config &&
    cdev.config.bmAttributes & USB_CONFIG_ATT_SELFPOWER)
    pub 2): usb_gadget_vbus_draw(gadget,,
    }
#[no_mangle]
pub unsafe extern "C" fn composite_resume(gadget: *mut usb_gadget) {
    void composite_resume(struct usb_gadget *gadget)
    {
    pub get_gadget_data(gadget): *mut *mut usb_composite_dev cdev =,
    pub f: *mut usb_function,
    pub maxpower: unsigned,
// REVISIT:  should we have config level
// suspend/resume callbacks?
//
    pub "resume\n"): DBG(cdev,,
    if (cdev.driver.resume)
    if (cdev.config) {
    list_for_each_entry(f, &cdev.config.functions, list) {
//
// Check for func_suspended flag to see if the function is
// in USB3 FUNCTION_SUSPEND state. In this case resume is
// done via FUNCTION_SUSPEND feature selector.
//
    if (f.resume && !f.func_suspended)
    }
    maxpower = cdev.config.MaxPower ?
    pub CONFIG_USB_GADGET_VBUS_DRAW: cdev->config->MaxPower :,
    if (gadget.speed < USB_SPEED_SUPER)
    pub 500U): maxpower = min(maxpower,,
    else
    pub 900U): maxpower = min(maxpower,,
    if (maxpower > USB_SELF_POWER_VBUS_MAX_DRAW ||
    !(cdev.config.bmAttributes & USB_CONFIG_ATT_SELFPOWER))
    else
    pub maxpower): usb_gadget_vbus_draw(gadget,,
    } else {
    pub CONFIG_USB_GADGET_VBUS_DRAW: maxpower =,
    pub 100U): maxpower = min(maxpower,,
    pub maxpower): usb_gadget_vbus_draw(gadget,,
    }
    pub 0: cdev->suspended =,
    }
// -------------------------------------------------------------------------
    static const struct usb_gadget_driver composite_driver_template = {
    .bind		= composite_bind,
    .unbind		= composite_unbind,
    .setup		= composite_setup,
    .reset		= composite_reset,
    .disconnect	= composite_disconnect,
    .suspend	= composite_suspend,
    .resume		= composite_resume,
    .driver	= {
    .owner		= THIS_MODULE,
    },
}

//
// usb_composite_probe() - register a composite driver
// @driver: the driver to register
//
// Context: single threaded during gadget setup
//
// This function is used to register drivers using the composite driver
// framework.  The return value is zero, or a negative errno value.
// Those values normally come from the driver's @bind method, which does
// all the work of setting up the driver to match the hardware.
//
// On successful return, the gadget is ready to respond to requests from
// the host, unless one of its components invokes usb_gadget_disconnect()
// while it was binding.  That would usually be done in order to wait for
// some userspace participation.
//
#[no_mangle]
pub unsafe extern "C" fn usb_composite_probe(driver: *mut usb_composite_driver) -> c_int {
    int usb_composite_probe(struct usb_composite_driver *driver)
    {
    struct usb_gadget_driver *gadget_driver;
    if (!driver || !driver.dev || !driver.bind)
    return -EINVAL;
    if (!driver.name)
    driver.name = "composite";
    driver.gadget_driver = composite_driver_template;
    gadget_driver = &driver.gadget_driver;
    gadget_driver.function =  (char *) driver.name;
    gadget_driver.driver.name = driver.name;
    gadget_driver.max_speed = driver.max_speed;
    return usb_gadget_register_driver(gadget_driver);
    }
    EXPORT_SYMBOL_GPL(usb_composite_probe);
//
// usb_composite_unregister() - unregister a composite driver
// @driver: the driver to unregister
//
// This function is used to unregister drivers using the composite
// driver framework.
//
#[no_mangle]
pub unsafe extern "C" fn usb_composite_unregister(driver: *mut usb_composite_driver) {
    void usb_composite_unregister(struct usb_composite_driver *driver)
    {
    usb_gadget_unregister_driver(&driver.gadget_driver);
    }
    EXPORT_SYMBOL_GPL(usb_composite_unregister);
//
// usb_composite_setup_continue() - Continue with the control transfer
// @cdev: the composite device who's control transfer was kept waiting
//
// This function must be called by the USB function driver to continue
// with the control transfer's data/status stage in case it had requested to
// delay the data/status stages. A USB function's setup handler (e.g. set_alt())
// can request the composite framework to delay the setup request's data/status
// stages by returning USB_GADGET_DELAYED_STATUS.
//
#[no_mangle]
pub unsafe extern "C" fn usb_composite_setup_continue(cdev: *mut usb_composite_dev) {
    void usb_composite_setup_continue(struct usb_composite_dev *cdev)
    {
    int			value;
    struct usb_request	*req = cdev.req;
    unsigned long		flags;
    DBG(cdev, "%s\n", __func__);
    spin_lock_irqsave(&cdev.lock, flags);
    if (cdev.delayed_status == 0) {
    WARN(cdev, "%s: Unexpected call\n", __func__);
    } else if (--cdev.delayed_status == 0) {
    DBG(cdev, "%s: Completing delayed status\n", __func__);
    req.length = 0;
    req.context = cdev;
    value = composite_ep0_queue(cdev, req, GFP_ATOMIC);
    if (value < 0) {
    DBG(cdev, "ep_queue -. %d\n", value);
    req.status = 0;
    composite_setup_complete(cdev.gadget.ep0, req);
    }
    }
    spin_unlock_irqrestore(&cdev.lock, flags);
    }
    EXPORT_SYMBOL_GPL(usb_composite_setup_continue);
    static char *composite_default_mfr(struct usb_gadget *gadget)
    {
    return kasprintf(GFP_KERNEL, "%s %s with %s", init_utsname().sysname,
    init_utsname().release, gadget.name);
    }
    void usb_composite_overwrite_options(struct usb_composite_dev *cdev,
    struct usb_composite_overwrite *covr)
    {
    struct usb_device_descriptor	*desc = &cdev.desc;
    struct usb_gadget_strings	*gstr = cdev.driver.strings[0];
    struct usb_string		*dev_str = gstr.strings;
    if (covr.idVendor)
    desc.idVendor = cpu_to_le16(covr.idVendor);
    if (covr.idProduct)
    desc.idProduct = cpu_to_le16(covr.idProduct);
    if (covr.bcdDevice)
    desc.bcdDevice = cpu_to_le16(covr.bcdDevice);
    if (covr.serial_number) {
    desc.iSerialNumber = dev_str[USB_GADGET_SERIAL_IDX].id;
    dev_str[USB_GADGET_SERIAL_IDX].s = covr.serial_number;
    }
    if (covr.manufacturer) {
    desc.iManufacturer = dev_str[USB_GADGET_MANUFACTURER_IDX].id;
    dev_str[USB_GADGET_MANUFACTURER_IDX].s = covr.manufacturer;
    } else if (!strlen(dev_str[USB_GADGET_MANUFACTURER_IDX].s)) {
    desc.iManufacturer = dev_str[USB_GADGET_MANUFACTURER_IDX].id;
    cdev.def_manufacturer = composite_default_mfr(cdev.gadget);
    dev_str[USB_GADGET_MANUFACTURER_IDX].s = cdev.def_manufacturer;
    }
    if (covr.product) {
    desc.iProduct = dev_str[USB_GADGET_PRODUCT_IDX].id;
    dev_str[USB_GADGET_PRODUCT_IDX].s = covr.product;
    }
    }
    EXPORT_SYMBOL_GPL(usb_composite_overwrite_options);
    MODULE_DESCRIPTION("infrastructure for Composite USB Gadgets");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("David Brownell");
