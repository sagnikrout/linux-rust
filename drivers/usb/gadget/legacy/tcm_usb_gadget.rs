//! Automatically rewritten from C to Rust
//! Source: drivers/usb/gadget/legacy/tcm_usb_gadget.c
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
// Target based USB-Gadget
//
// UAS protocol handling, target callbacks, configfs handling,
// BBB (USB Mass Storage Class Bulk-Only (BBB) and Transport protocol handling.
//
// Author: Sebastian Andrzej Siewior <bigeasy at linutronix dot de>
//

    USB_GADGET_COMPOSITE_OPTIONS();
pub const UAS_VENDOR_ID: c_uint = 0x0525	/* NetChip */;
pub const UAS_PRODUCT_ID: c_uint = 0xa4a5	/* Linux-USB File-backed Storage Gadget */;
    static struct usb_device_descriptor usbg_device_desc = {
    .bLength =		sizeof(usbg_device_desc),
    .bDescriptorType =	USB_DT_DEVICE,
// .bcdUSB = DYNAMIC
    .bDeviceClass =		USB_CLASS_PER_INTERFACE,
    .idVendor =		cpu_to_le16(UAS_VENDOR_ID),
    .idProduct =		cpu_to_le16(UAS_PRODUCT_ID),
    .bNumConfigurations =   1,
    };

    static struct usb_string	usbg_us_strings[] = {
    [USB_GADGET_MANUFACTURER_IDX].s	= "Target Manufacturer",
    [USB_GADGET_PRODUCT_IDX].s	= "Target Product",
    [USB_GADGET_SERIAL_IDX].s	= "000000000001",
    [USB_G_STR_CONFIG].s		= "default config",
    { },
    };
    static struct usb_gadget_strings usbg_stringtab = {
    .language = 0x0409,
    .strings = usbg_us_strings,
    };
    static struct usb_gadget_strings *usbg_strings[] = {
    &usbg_stringtab,
    core::ptr::null_mut(),
    };
    static struct usb_function_instance *fi_tcm;
    static struct usb_function *f_tcm;
#[no_mangle]
unsafe extern "C" fn guas_unbind(cdev: *mut usb_composite_dev) -> c_int {
    static int guas_unbind(struct usb_composite_dev *cdev)
    {
    if (!IS_ERR_OR_NULL(f_tcm))
    usb_put_function(f_tcm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tcm_do_config(c: *mut usb_configuration) -> c_int {
    static int tcm_do_config(struct usb_configuration *c)
    {
    int status;
    f_tcm = usb_get_function(fi_tcm);
    if (IS_ERR(f_tcm))
    return PTR_ERR(f_tcm);
    status = usb_add_function(c, f_tcm);
    if (status < 0) {
    usb_put_function(f_tcm);
    return status;
    }
    return 0;
    }
    static struct usb_configuration usbg_config_driver = {
    .label                  = "Linux Target",
    .bConfigurationValue    = 1,
    .bmAttributes           = USB_CONFIG_ATT_SELFPOWER,
    };
    static int usbg_attach(struct usb_function_instance *f);
    static void usbg_detach(struct usb_function_instance *f);
#[no_mangle]
unsafe extern "C" fn usb_target_bind(cdev: *mut usb_composite_dev) -> c_int {
    static int usb_target_bind(struct usb_composite_dev *cdev)
    {
    int ret;
    ret = usb_string_ids_tab(cdev, usbg_us_strings);
    if (ret)
    return ret;
    usbg_device_desc.iManufacturer =
    usbg_us_strings[USB_GADGET_MANUFACTURER_IDX].id;
    usbg_device_desc.iProduct = usbg_us_strings[USB_GADGET_PRODUCT_IDX].id;
    usbg_device_desc.iSerialNumber =
    usbg_us_strings[USB_GADGET_SERIAL_IDX].id;
    usbg_config_driver.iConfiguration =
    usbg_us_strings[USB_G_STR_CONFIG].id;
    ret = usb_add_config(cdev, &usbg_config_driver, tcm_do_config);
    if (ret)
    return ret;
    usb_composite_overwrite_options(cdev, &coverwrite);
    return 0;
    }
    static struct usb_composite_driver usbg_driver = {
    .name           = "g_target",
    .dev            = &usbg_device_desc,
    .strings        = usbg_strings,
    .max_speed      = USB_SPEED_SUPER,
    .bind		= usb_target_bind,
    .unbind         = guas_unbind,
    };
#[no_mangle]
unsafe extern "C" fn usbg_attach(f: *mut usb_function_instance) -> c_int {
    static int usbg_attach(struct usb_function_instance *f)
    {
    return usb_composite_probe(&usbg_driver);
    }
#[no_mangle]
unsafe extern "C" fn usbg_detach(f: *mut usb_function_instance) {
    static void usbg_detach(struct usb_function_instance *f)
    {
    usb_composite_unregister(&usbg_driver);
    }
#[no_mangle]
unsafe extern "C" fn usb_target_gadget_init() -> int __init {
    static int __init usb_target_gadget_init(void)
    {
    struct f_tcm_opts *tcm_opts;
    fi_tcm = usb_get_function_instance("tcm");
    if (IS_ERR(fi_tcm))
    return PTR_ERR(fi_tcm);
    tcm_opts = container_of(fi_tcm, struct f_tcm_opts, func_inst);
    mutex_lock(&tcm_opts.dep_lock);
    tcm_opts.tcm_register_callback = usbg_attach;
    tcm_opts.tcm_unregister_callback = usbg_detach;
    tcm_opts.dependent = THIS_MODULE;
    tcm_opts.can_attach = true;
    tcm_opts.has_dep = true;
    mutex_unlock(&tcm_opts.dep_lock);
    fi_tcm.set_inst_name(fi_tcm, "tcm-legacy");
    return 0;
    }
    module_init(usb_target_gadget_init);
#[no_mangle]
unsafe extern "C" fn usb_target_gadget_exit() -> void __exit {
    static void __exit usb_target_gadget_exit(void)
    {
    if (!IS_ERR_OR_NULL(fi_tcm))
    usb_put_function_instance(fi_tcm);
    }
    module_exit(usb_target_gadget_exit);
    MODULE_AUTHOR("Sebastian Andrzej Siewior <bigeasy@linutronix.de>");
    MODULE_DESCRIPTION("usb-gadget fabric");
    MODULE_LICENSE("GPL v2");
