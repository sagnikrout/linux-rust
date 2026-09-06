//! Automatically rewritten from C to Rust
//! Source: drivers/usb/gadget/legacy/serial.c
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
// serial.c -- USB gadget serial driver
//
// Copyright (C) 2003 Al Borchers (alborchers@steinerpoint.com)
// Copyright (C) 2008 by David Brownell
// Copyright (C) 2008 by Nokia Corporation
//

// Defines

pub const GS_VERSION_NUM: c_uint = 0x2400;

// -------------------------------------------------------------------------
    USB_GADGET_COMPOSITE_OPTIONS();
// Thanks to NetChip Technologies for donating this product ID.
//
// DO NOT REUSE THESE IDs with a protocol-incompatible driver!!  Ever!!
// Instead:  allocate your own, using normal USB-IF procedures.
//
pub const GS_VENDOR_ID: c_uint = 0x0525	/* NetChip */;
pub const GS_PRODUCT_ID: c_uint = 0xa4a6	/* Linux-USB Serial Gadget */;
pub const GS_CDC_PRODUCT_ID: c_uint = 0xa4a7	/* ... as CDC-ACM */;
pub const GS_CDC_OBEX_PRODUCT_ID: c_uint = 0xa4a9	/* ... as CDC-OBEX */;
// string IDs are assigned dynamically

    static struct usb_string strings_dev[] = {
    [USB_GADGET_MANUFACTURER_IDX].s = "",
    [USB_GADGET_PRODUCT_IDX].s = GS_VERSION_NAME,
    [USB_GADGET_SERIAL_IDX].s = "",
    [STRING_DESCRIPTION_IDX].s = core::ptr::null_mut() /* updated; f(use_acm) */,
    {  } /* end of list */
    };
    static struct usb_gadget_strings stringtab_dev = {
    .language	= 0x0409,	/* en-us */
    .strings	= strings_dev,
    };
    static struct usb_gadget_strings *dev_strings[] = {
    &stringtab_dev,
    core::ptr::null_mut(),
    };
    static struct usb_device_descriptor device_desc = {
    .bLength =		USB_DT_DEVICE_SIZE,
    .bDescriptorType =	USB_DT_DEVICE,
// .bcdUSB = DYNAMIC
// .bDeviceClass = f(use_acm)
    .bDeviceSubClass =	0,
    .bDeviceProtocol =	0,
// .bMaxPacketSize0 = f(hardware)
    .idVendor =		cpu_to_le16(GS_VENDOR_ID),
// .idProduct =	f(use_acm)
    .bcdDevice = cpu_to_le16(GS_VERSION_NUM),
// .iManufacturer = DYNAMIC
// .iProduct = DYNAMIC
    .bNumConfigurations =	1,
    };
    static const struct usb_descriptor_header *otg_desc[2];
// -------------------------------------------------------------------------
// Module
    MODULE_DESCRIPTION(GS_VERSION_NAME);
    MODULE_AUTHOR("Al Borchers");
    MODULE_AUTHOR("David Brownell");
    MODULE_LICENSE("GPL");
    let mut use_acm: static bool = true;
    module_param(use_acm, bool, 0);
    MODULE_PARM_DESC(use_acm, "Use CDC ACM, default=yes");
    let mut use_obex: static bool = false;
    module_param(use_obex, bool, 0);
    MODULE_PARM_DESC(use_obex, "Use CDC OBEX, default=no");
    let mut n_ports: static unsigned = 1;
    module_param(n_ports, uint, 0);
    MODULE_PARM_DESC(n_ports, "number of ports to create, default=1");
    let mut enable: static bool = true;
    static int switch_gserial_enable(bool do_enable);
#[no_mangle]
unsafe extern "C" fn enable_set(s: *const c_char, kp: *const kernel_param) -> c_int {
    static int enable_set(const char *s, const struct kernel_param *kp)
    {
    bool do_enable;
    int ret;
    if (!s)	/* called for no-arg enable == default */
    return 0;
    ret = kstrtobool(s, &do_enable);
    if (ret || enable == do_enable)
    return ret;
    ret = switch_gserial_enable(do_enable);
    if (!ret)
    enable = do_enable;
    return ret;
    }
    static const struct kernel_param_ops enable_ops = {
    .set = enable_set,
    .get = param_get_bool,
    };
    module_param_cb(enable, &enable_ops, &enable, 0644);
// -------------------------------------------------------------------------
    static struct usb_configuration serial_config_driver = {
// .label = f(use_acm)
// .bConfigurationValue = f(use_acm)
// .iConfiguration = DYNAMIC
    .bmAttributes	= USB_CONFIG_ATT_SELFPOWER,
    };
    static struct usb_function_instance *fi_serial[MAX_U_SERIAL_PORTS];
    static struct usb_function *f_serial[MAX_U_SERIAL_PORTS];
    static int serial_register_ports(struct usb_composite_dev *cdev,
    struct usb_configuration *c, const char *f_name)
    {
    int i;
    int ret;
    ret = usb_add_config_only(cdev, c);
    if (ret)
    goto out;
    for (i = 0; i < n_ports; i++) {
    fi_serial[i] = usb_get_function_instance(f_name);
    if (IS_ERR(fi_serial[i])) {
    ret = PTR_ERR(fi_serial[i]);
    goto fail;
    }
    f_serial[i] = usb_get_function(fi_serial[i]);
    if (IS_ERR(f_serial[i])) {
    ret = PTR_ERR(f_serial[i]);
    goto err_get_func;
    }
    ret = usb_add_function(c, f_serial[i]);
    if (ret)
    goto err_add_func;
    }
    return 0;
    err_add_func:
    usb_put_function(f_serial[i]);
    err_get_func:
    usb_put_function_instance(fi_serial[i]);
    fail:
    i--;
    while (i >= 0) {
    usb_remove_function(c, f_serial[i]);
    usb_put_function(f_serial[i]);
    usb_put_function_instance(fi_serial[i]);
    i--;
    }
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn gs_bind(cdev: *mut usb_composite_dev) -> c_int {
    static int gs_bind(struct usb_composite_dev *cdev)
    {
    int			status;
// Allocate string descriptor numbers ... note that string
// contents can be overridden by the composite_dev glue.
//
    status = usb_string_ids_tab(cdev, strings_dev);
    if (status < 0)
    goto fail;
    device_desc.iManufacturer = strings_dev[USB_GADGET_MANUFACTURER_IDX].id;
    device_desc.iProduct = strings_dev[USB_GADGET_PRODUCT_IDX].id;
    status = strings_dev[STRING_DESCRIPTION_IDX].id;
    serial_config_driver.iConfiguration = status;
    if (gadget_is_otg(cdev.gadget)) {
    if (!otg_desc[0]) {
    struct usb_descriptor_header *usb_desc;
    usb_desc = usb_otg_descriptor_alloc(cdev.gadget);
    if (!usb_desc) {
    status = -ENOMEM;
    goto fail;
    }
    usb_otg_descriptor_init(cdev.gadget, usb_desc);
    otg_desc[0] = usb_desc;
    otg_desc[1] = core::ptr::null_mut();
    }
    serial_config_driver.descriptors = otg_desc;
    serial_config_driver.bmAttributes |= USB_CONFIG_ATT_WAKEUP;
    }
// register our configuration
    if (use_acm) {
    status  = serial_register_ports(cdev, &serial_config_driver,
    "acm");
    usb_ep_autoconfig_reset(cdev.gadget);
    } else if (use_obex)
    status = serial_register_ports(cdev, &serial_config_driver,
    "obex");
    else {
    status = serial_register_ports(cdev, &serial_config_driver,
    "gser");
    }
    if (status < 0)
    goto fail1;
    usb_composite_overwrite_options(cdev, &coverwrite);
    INFO(cdev, "%s\n", GS_VERSION_NAME);
    return 0;
    fail1:
    kfree(otg_desc[0]);
    otg_desc[0] = core::ptr::null_mut();
    fail:
    return status;
    }
#[no_mangle]
unsafe extern "C" fn gs_unbind(cdev: *mut usb_composite_dev) -> c_int {
    static int gs_unbind(struct usb_composite_dev *cdev)
    {
    int i;
    for (i = 0; i < n_ports; i++) {
    usb_put_function(f_serial[i]);
    usb_put_function_instance(fi_serial[i]);
    }
    kfree(otg_desc[0]);
    otg_desc[0] = core::ptr::null_mut();
    return 0;
    }
    static struct usb_composite_driver gserial_driver = {
    .name		= "g_serial",
    .dev		= &device_desc,
    .strings	= dev_strings,
    .max_speed	= USB_SPEED_SUPER,
    .bind		= gs_bind,
    .unbind		= gs_unbind,
    };
#[no_mangle]
unsafe extern "C" fn switch_gserial_enable(do_enable: bool) -> c_int {
    static int switch_gserial_enable(bool do_enable)
    {
    if (!serial_config_driver.label)
// gserial_init() was not called, yet
    return 0;
    if (do_enable)
    return usb_composite_probe(&gserial_driver);
    usb_composite_unregister(&gserial_driver);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gserial_init() -> int __init {
    static int __init gserial_init(void)
    {
// We *could* export two configs; that'd be much cleaner...
// but neither of these product IDs was defined that way.
//
    if (use_acm) {
    serial_config_driver.label = "CDC ACM config";
    serial_config_driver.bConfigurationValue = 2;
    device_desc.bDeviceClass = USB_CLASS_COMM;
    device_desc.idProduct =
    cpu_to_le16(GS_CDC_PRODUCT_ID);
    } else if (use_obex) {
    serial_config_driver.label = "CDC OBEX config";
    serial_config_driver.bConfigurationValue = 3;
    device_desc.bDeviceClass = USB_CLASS_COMM;
    device_desc.idProduct =
    cpu_to_le16(GS_CDC_OBEX_PRODUCT_ID);
    } else {
    serial_config_driver.label = "Generic Serial config";
    serial_config_driver.bConfigurationValue = 1;
    device_desc.bDeviceClass = USB_CLASS_VENDOR_SPEC;
    device_desc.idProduct =
    cpu_to_le16(GS_PRODUCT_ID);
    }
    strings_dev[STRING_DESCRIPTION_IDX].s = serial_config_driver.label;
    if (!enable)
    return 0;
    return usb_composite_probe(&gserial_driver);
    }
    module_init(gserial_init);
#[no_mangle]
unsafe extern "C" fn gserial_cleanup() -> void __exit {
    static void __exit gserial_cleanup(void)
    {
    if (enable)
    usb_composite_unregister(&gserial_driver);
    }
    module_exit(gserial_cleanup);
