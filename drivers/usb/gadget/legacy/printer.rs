//! Automatically rewritten from C to Rust
//! Source: drivers/usb/gadget/legacy/printer.c
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
// printer.c -- Printer gadget driver
//
// Copyright (C) 2003-2005 David Brownell
// Copyright (C) 2006 Craig W. Nadler
//

    USB_GADGET_COMPOSITE_OPTIONS();

    static const char shortname [] = "printer";

// -------------------------------------------------------------------------
// DO NOT REUSE THESE IDs with a protocol-incompatible driver!!  Ever!!
// Instead:  allocate your own, using normal USB-IF procedures.
//
// Thanks to NetChip Technologies for donating this product ID.
//
pub const PRINTER_VENDOR_NUM: c_uint = 0x0525		/* NetChip */;
pub const PRINTER_PRODUCT_NUM: c_uint = 0xa4a8		/* Linux-USB Printer Gadget */;
// Some systems will want different product identifiers published in the
// device descriptor, either numbers or strings or both.  These string
// parameters are in UTF-8 (superset of ASCII's 7 bit characters).
//
    module_param_named(iSerialNum, coverwrite.serial_number, charp, S_IRUGO);
    MODULE_PARM_DESC(iSerialNum, "1");
    static char *iPNPstring;
    module_param(iPNPstring, charp, S_IRUGO);
    MODULE_PARM_DESC(iPNPstring, "MFG:linux;MDL:g_printer;CLS:PRINTER;SN:1;");
// Number of requests to allocate per endpoint, not used for ep0.
    let mut qlen: static unsigned = 10;
    module_param(qlen, uint, S_IRUGO|S_IWUSR);
    MODULE_PARM_DESC(qlen, "The number of 8k buffers to use per endpoint");

    static struct usb_function_instance *fi_printer;
    static struct usb_function *f_printer;
// -------------------------------------------------------------------------
//
// DESCRIPTORS ... most are static, but strings and (full) configuration
// descriptors are built on demand.
//
    static struct usb_device_descriptor device_desc = {
    .bLength =		sizeof device_desc,
    .bDescriptorType =	USB_DT_DEVICE,
// .bcdUSB = DYNAMIC
    .bDeviceClass =		USB_CLASS_PER_INTERFACE,
    .bDeviceSubClass =	0,
    .bDeviceProtocol =	0,
    .idVendor =		cpu_to_le16(PRINTER_VENDOR_NUM),
    .idProduct =		cpu_to_le16(PRINTER_PRODUCT_NUM),
    .bNumConfigurations =	1
    };
    static const struct usb_descriptor_header *otg_desc[2];
// -------------------------------------------------------------------------
// descriptors that are built on-demand
    static char				product_desc [40] = DRIVER_DESC;
    static char				serial_num [40] = "1";
    static char				*pnp_string =
    "MFG:linux;MDL:g_printer;CLS:PRINTER;SN:1;";
// static strings, in UTF-8
    static struct usb_string		strings [] = {
    [USB_GADGET_MANUFACTURER_IDX].s = "",
    [USB_GADGET_PRODUCT_IDX].s = product_desc,
    [USB_GADGET_SERIAL_IDX].s =	serial_num,
    {  }		/* end of list */
    };
    static struct usb_gadget_strings	stringtab_dev = {
    .language	= 0x0409,	/* en-us */
    .strings	= strings,
    };
    static struct usb_gadget_strings *dev_strings[] = {
    &stringtab_dev,
    core::ptr::null_mut(),
    };
    static struct usb_configuration printer_cfg_driver = {
    .label			= "printer",
    .bConfigurationValue	= 1,
    .bmAttributes		= USB_CONFIG_ATT_ONE | USB_CONFIG_ATT_SELFPOWER,
    };
#[no_mangle]
unsafe extern "C" fn printer_do_config(c: *mut usb_configuration) -> c_int {
    static int printer_do_config(struct usb_configuration *c)
    {
    struct usb_gadget	*gadget = c.cdev.gadget;
    let mut status: c_int = 0;
    usb_ep_autoconfig_reset(gadget);
    usb_gadget_set_selfpowered(gadget);
    if (gadget_is_otg(gadget)) {
    printer_cfg_driver.descriptors = otg_desc;
    printer_cfg_driver.bmAttributes |= USB_CONFIG_ATT_WAKEUP;
    }
    f_printer = usb_get_function(fi_printer);
    if (IS_ERR(f_printer))
    return PTR_ERR(f_printer);
    status = usb_add_function(c, f_printer);
    if (status < 0)
    usb_put_function(f_printer);
    return status;
    }
#[no_mangle]
unsafe extern "C" fn printer_bind(cdev: *mut usb_composite_dev) -> c_int {
    static int printer_bind(struct usb_composite_dev *cdev)
    {
    struct f_printer_opts *opts;
    int ret;
    fi_printer = usb_get_function_instance("printer");
    if (IS_ERR(fi_printer))
    return PTR_ERR(fi_printer);
    opts = container_of(fi_printer, struct f_printer_opts, func_inst);
    opts.minor = 0;
    opts.q_len = QLEN;
    if (iPNPstring) {
    opts.pnp_string = kstrdup(iPNPstring, GFP_KERNEL);
    if (!opts.pnp_string) {
    ret = -ENOMEM;
    goto fail_put_func_inst;
    }
    opts.pnp_string_allocated = true;
//
// we don't free this memory in case of error
// as printer cleanup func will do this for us
//
    } else {
    opts.pnp_string = pnp_string;
    }
    ret = usb_string_ids_tab(cdev, strings);
    if (ret < 0)
    goto fail_put_func_inst;
    device_desc.iManufacturer = strings[USB_GADGET_MANUFACTURER_IDX].id;
    device_desc.iProduct = strings[USB_GADGET_PRODUCT_IDX].id;
    device_desc.iSerialNumber = strings[USB_GADGET_SERIAL_IDX].id;
    if (gadget_is_otg(cdev.gadget) && !otg_desc[0]) {
    struct usb_descriptor_header *usb_desc;
    usb_desc = usb_otg_descriptor_alloc(cdev.gadget);
    if (!usb_desc) {
    ret = -ENOMEM;
    goto fail_put_func_inst;
    }
    usb_otg_descriptor_init(cdev.gadget, usb_desc);
    otg_desc[0] = usb_desc;
    otg_desc[1] = core::ptr::null_mut();
    }
    ret = usb_add_config(cdev, &printer_cfg_driver, printer_do_config);
    if (ret)
    goto fail_free_otg_desc;
    usb_composite_overwrite_options(cdev, &coverwrite);
    return ret;
    fail_free_otg_desc:
    kfree(otg_desc[0]);
    otg_desc[0] = core::ptr::null_mut();
    fail_put_func_inst:
    usb_put_function_instance(fi_printer);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn printer_unbind(cdev: *mut usb_composite_dev) -> c_int {
    static int printer_unbind(struct usb_composite_dev *cdev)
    {
    usb_put_function(f_printer);
    usb_put_function_instance(fi_printer);
    kfree(otg_desc[0]);
    otg_desc[0] = core::ptr::null_mut();
    return 0;
    }
    static struct usb_composite_driver printer_driver = {
    .name           = shortname,
    .dev            = &device_desc,
    .strings        = dev_strings,
    .max_speed      = USB_SPEED_SUPER,
    .bind		= printer_bind,
    .unbind		= printer_unbind,
    };
    module_usb_composite_driver(printer_driver);
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_AUTHOR("Craig Nadler");
    MODULE_LICENSE("GPL");
