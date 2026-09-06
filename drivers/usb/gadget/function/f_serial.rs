//! Automatically rewritten from C to Rust
//! Source: drivers/usb/gadget/function/f_serial.c
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
// f_serial.c - generic USB serial function driver
//
// Copyright (C) 2003 Al Borchers (alborchers@steinerpoint.com)
// Copyright (C) 2008 by David Brownell
// Copyright (C) 2008 by Nokia Corporation
//

//
// This function packages a simple "generic serial" port with no real
// control mechanisms, just raw data transfer over two bulk endpoints.
//
// Because it's not standardized, this isn't as interoperable as the
// CDC ACM driver.  However, for many purposes it's just as functional
// if you can arrange appropriate host side drivers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct f_gser {
    pub port: gserial,
    pub data_id: u8,
    pub port_num: u8,
}

    static inline struct f_gser *func_to_gser(struct usb_function *f)
    {
    return container_of(f, struct f_gser, port.func);
    }
// -------------------------------------------------------------------------
// interface descriptor:
    static struct usb_interface_descriptor gser_interface_desc = {
    .bLength =		USB_DT_INTERFACE_SIZE,
    .bDescriptorType =	USB_DT_INTERFACE,
// .bInterfaceNumber = DYNAMIC
    .bNumEndpoints =	2,
    .bInterfaceClass =	USB_CLASS_VENDOR_SPEC,
    .bInterfaceSubClass =	0,
    .bInterfaceProtocol =	0,
// .iInterface = DYNAMIC
    };
// full speed support:
    static struct usb_endpoint_descriptor gser_fs_in_desc = {
    .bLength =		USB_DT_ENDPOINT_SIZE,
    .bDescriptorType =	USB_DT_ENDPOINT,
    .bEndpointAddress =	USB_DIR_IN,
    .bmAttributes =		USB_ENDPOINT_XFER_BULK,
    };
    static struct usb_endpoint_descriptor gser_fs_out_desc = {
    .bLength =		USB_DT_ENDPOINT_SIZE,
    .bDescriptorType =	USB_DT_ENDPOINT,
    .bEndpointAddress =	USB_DIR_OUT,
    .bmAttributes =		USB_ENDPOINT_XFER_BULK,
    };
    static struct usb_descriptor_header *gser_fs_function[] = {
    (struct usb_descriptor_header *) &gser_interface_desc,
    (struct usb_descriptor_header *) &gser_fs_in_desc,
    (struct usb_descriptor_header *) &gser_fs_out_desc,
    core::ptr::null_mut(),
    };
// high speed support:
    static struct usb_endpoint_descriptor gser_hs_in_desc = {
    .bLength =		USB_DT_ENDPOINT_SIZE,
    .bDescriptorType =	USB_DT_ENDPOINT,
    .bmAttributes =		USB_ENDPOINT_XFER_BULK,
    .wMaxPacketSize =	cpu_to_le16(512),
    };
    static struct usb_endpoint_descriptor gser_hs_out_desc = {
    .bLength =		USB_DT_ENDPOINT_SIZE,
    .bDescriptorType =	USB_DT_ENDPOINT,
    .bmAttributes =		USB_ENDPOINT_XFER_BULK,
    .wMaxPacketSize =	cpu_to_le16(512),
    };
    static struct usb_descriptor_header *gser_hs_function[] = {
    (struct usb_descriptor_header *) &gser_interface_desc,
    (struct usb_descriptor_header *) &gser_hs_in_desc,
    (struct usb_descriptor_header *) &gser_hs_out_desc,
    core::ptr::null_mut(),
    };
    static struct usb_endpoint_descriptor gser_ss_in_desc = {
    .bLength =		USB_DT_ENDPOINT_SIZE,
    .bDescriptorType =	USB_DT_ENDPOINT,
    .bmAttributes =		USB_ENDPOINT_XFER_BULK,
    .wMaxPacketSize =	cpu_to_le16(1024),
    };
    static struct usb_endpoint_descriptor gser_ss_out_desc = {
    .bLength =		USB_DT_ENDPOINT_SIZE,
    .bDescriptorType =	USB_DT_ENDPOINT,
    .bmAttributes =		USB_ENDPOINT_XFER_BULK,
    .wMaxPacketSize =	cpu_to_le16(1024),
    };
    static struct usb_ss_ep_comp_descriptor gser_ss_bulk_comp_desc = {
    .bLength =              sizeof gser_ss_bulk_comp_desc,
    .bDescriptorType =      USB_DT_SS_ENDPOINT_COMP,
    };
    static struct usb_descriptor_header *gser_ss_function[] = {
    (struct usb_descriptor_header *) &gser_interface_desc,
    (struct usb_descriptor_header *) &gser_ss_in_desc,
    (struct usb_descriptor_header *) &gser_ss_bulk_comp_desc,
    (struct usb_descriptor_header *) &gser_ss_out_desc,
    (struct usb_descriptor_header *) &gser_ss_bulk_comp_desc,
    core::ptr::null_mut(),
    };
// string descriptors:
    static struct usb_string gser_string_defs[] = {
    [0].s = "Generic Serial",
    {  } /* end of list */
    };
    static struct usb_gadget_strings gser_string_table = {
    .language =		0x0409,	/* en-us */
    .strings =		gser_string_defs,
    };
    static struct usb_gadget_strings *gser_strings[] = {
    &gser_string_table,
    core::ptr::null_mut(),
    };
// -------------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn gser_set_alt(f: *mut usb_function, intf: unsigned, alt: unsigned) -> c_int {
    static int gser_set_alt(struct usb_function *f, unsigned intf, unsigned alt)
    {
    struct f_gser		*gser = func_to_gser(f);
    struct usb_composite_dev *cdev = f.config.cdev;
// we know alt == 0, so this is an activation or a reset
    if (gser.port.in.enabled) {
    dev_dbg(&cdev.gadget.dev,
    "reset generic ttyGS%d\n", gser.port_num);
    gserial_disconnect(&gser.port);
    }
    if (!gser.port.in.desc || !gser.port.out.desc) {
    dev_dbg(&cdev.gadget.dev,
    "activate generic ttyGS%d\n", gser.port_num);
    if (config_ep_by_speed(cdev.gadget, f, gser.port.in) ||
    config_ep_by_speed(cdev.gadget, f, gser.port.out)) {
    gser.port.in.desc = core::ptr::null_mut();
    gser.port.out.desc = core::ptr::null_mut();
    return -EINVAL;
    }
    }
    gserial_connect(&gser.port, gser.port_num);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gser_disable(f: *mut usb_function) {
    static void gser_disable(struct usb_function *f)
    {
    struct f_gser	*gser = func_to_gser(f);
    struct usb_composite_dev *cdev = f.config.cdev;
    dev_dbg(&cdev.gadget.dev,
    "generic ttyGS%d deactivated\n", gser.port_num);
    gserial_disconnect(&gser.port);
    }
// -------------------------------------------------------------------------
// serial function driver setup/binding
#[no_mangle]
unsafe extern "C" fn gser_bind(c: *mut usb_configuration, f: *mut usb_function) -> c_int {
    static int gser_bind(struct usb_configuration *c, struct usb_function *f)
    {
    struct usb_composite_dev *cdev = c.cdev;
    struct f_gser		*gser = func_to_gser(f);
    int			status;
    struct usb_ep		*ep;
// REVISIT might want instance-specific strings to help
// distinguish instances ...
//
// maybe allocate device-global string ID
    if (gser_string_defs[0].id == 0) {
    status = usb_string_id(c.cdev);
    if (status < 0)
    return status;
    gser_string_defs[0].id = status;
    }
// allocate instance-specific interface IDs
    status = usb_interface_id(c, f);
    if (status < 0)
    goto fail;
    gser.data_id = status;
    gser_interface_desc.bInterfaceNumber = status;
    status = -ENODEV;
// allocate instance-specific endpoints
    ep = usb_ep_autoconfig(cdev.gadget, &gser_fs_in_desc);
    if (!ep)
    goto fail;
    gser.port.in = ep;
    ep = usb_ep_autoconfig(cdev.gadget, &gser_fs_out_desc);
    if (!ep)
    goto fail;
    gser.port.out = ep;
// support all relevant hardware speeds... we expect that when
// hardware is dual speed, all bulk-capable endpoints work at
// both speeds
//
    gser_hs_in_desc.bEndpointAddress = gser_fs_in_desc.bEndpointAddress;
    gser_hs_out_desc.bEndpointAddress = gser_fs_out_desc.bEndpointAddress;
    gser_ss_in_desc.bEndpointAddress = gser_fs_in_desc.bEndpointAddress;
    gser_ss_out_desc.bEndpointAddress = gser_fs_out_desc.bEndpointAddress;
    status = usb_assign_descriptors(f, gser_fs_function, gser_hs_function,
    gser_ss_function, gser_ss_function);
    if (status)
    goto fail;
    dev_dbg(&cdev.gadget.dev, "generic ttyGS%d: IN/%s OUT/%s\n",
    gser.port_num,
    gser.port.in.name, gser.port.out.name);
    return 0;
    fail:
    ERROR(cdev, "%s: can't bind, err %d\n", f.name, status);
    return status;
    }
    static inline struct f_serial_opts *to_f_serial_opts(struct config_item *item)
    {
    return container_of(to_config_group(item), struct f_serial_opts,
    func_inst.group);
    }
#[no_mangle]
unsafe extern "C" fn serial_attr_release(item: *mut config_item) {
    static void serial_attr_release(struct config_item *item)
    {
    struct f_serial_opts *opts = to_f_serial_opts(item);
    usb_put_function_instance(&opts.func_inst);
    }
    static const struct configfs_item_operations serial_item_ops = {
    .release	= serial_attr_release,
    };

    static ssize_t f_serial_console_store(struct config_item *item,
    const char *page, size_t count)
    {
    return gserial_set_console(to_f_serial_opts(item).port_num,
    page, count);
    }
#[no_mangle]
unsafe extern "C" fn f_serial_console_show(item: *mut config_item, page: *mut c_char) -> isize {
    static ssize_t f_serial_console_show(struct config_item *item, char *page)
    {
    return gserial_get_console(to_f_serial_opts(item).port_num, page);
    }
    CONFIGFS_ATTR(f_serial_, console);

#[no_mangle]
unsafe extern "C" fn f_serial_port_num_show(item: *mut config_item, page: *mut c_char) -> isize {
    static ssize_t f_serial_port_num_show(struct config_item *item, char *page)
    {
    return sprintf(page, "%u\n", to_f_serial_opts(item).port_num);
    }
    CONFIGFS_ATTR_RO(f_serial_, port_num);
    static struct configfs_attribute *acm_attrs[] = {

    &f_serial_attr_console,

    &f_serial_attr_port_num,
    core::ptr::null_mut(),
    };
    static const struct config_item_type serial_func_type = {
    .ct_item_ops	= &serial_item_ops,
    .ct_attrs	= acm_attrs,
    .ct_owner	= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn gser_free_inst(f: *mut usb_function_instance) {
    static void gser_free_inst(struct usb_function_instance *f)
    {
    struct f_serial_opts *opts;
    opts = container_of(f, struct f_serial_opts, func_inst);
    gserial_free_line(opts.port_num);
    kfree(opts);
    }
    static struct usb_function_instance *gser_alloc_inst(void)
    {
    struct f_serial_opts *opts;
    int ret;
    opts = kzalloc_obj(*opts);
    if (!opts)
    return ERR_PTR(-ENOMEM);
    opts.func_inst.free_func_inst = gser_free_inst;
    ret = gserial_alloc_line(&opts.port_num);
    if (ret) {
    kfree(opts);
    return ERR_PTR(ret);
    }
    config_group_init_type_name(&opts.func_inst.group, "",
    &serial_func_type);
    return &opts.func_inst;
    }
#[no_mangle]
unsafe extern "C" fn gser_free(f: *mut usb_function) {
    static void gser_free(struct usb_function *f)
    {
    struct f_gser *serial;
    serial = func_to_gser(f);
    kfree(serial);
    }
#[no_mangle]
unsafe extern "C" fn gser_unbind(c: *mut usb_configuration, f: *mut usb_function) {
    static void gser_unbind(struct usb_configuration *c, struct usb_function *f)
    {
    struct f_gser	*gser = func_to_gser(f);
// Ensure port is disconnected before unbinding
    gserial_disconnect(&gser.port);
    usb_free_all_descriptors(f);
    }
#[no_mangle]
unsafe extern "C" fn gser_resume(f: *mut usb_function) {
    static void gser_resume(struct usb_function *f)
    {
    struct f_gser *gser = func_to_gser(f);
    gserial_resume(&gser.port);
    }
#[no_mangle]
unsafe extern "C" fn gser_suspend(f: *mut usb_function) {
    static void gser_suspend(struct usb_function *f)
    {
    struct f_gser *gser = func_to_gser(f);
    gserial_suspend(&gser.port);
    }
#[no_mangle]
unsafe extern "C" fn gser_get_status(f: *mut usb_function) -> c_int {
    static int gser_get_status(struct usb_function *f)
    {
    return (f.func_wakeup_armed ? USB_INTRF_STAT_FUNC_RW : 0) |
    USB_INTRF_STAT_FUNC_RW_CAP;
    }
    static struct usb_function *gser_alloc(struct usb_function_instance *fi)
    {
    struct f_gser	*gser;
    struct f_serial_opts *opts;
// allocate and initialize one new instance
    gser = kzalloc_obj(*gser);
    if (!gser)
    return ERR_PTR(-ENOMEM);
    opts = container_of(fi, struct f_serial_opts, func_inst);
    gser.port_num = opts.port_num;
    gser.port.func.name = "gser";
    gser.port.func.strings = gser_strings;
    gser.port.func.bind = gser_bind;
    gser.port.func.unbind = gser_unbind;
    gser.port.func.set_alt = gser_set_alt;
    gser.port.func.disable = gser_disable;
    gser.port.func.free_func = gser_free;
    gser.port.func.resume = gser_resume;
    gser.port.func.suspend = gser_suspend;
    gser.port.func.get_status = gser_get_status;
    return &gser.port.func;
    }
    DECLARE_USB_FUNCTION_INIT(gser, gser_alloc_inst, gser_alloc);
    MODULE_DESCRIPTION("generic USB serial function driver");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Al Borchers");
    MODULE_AUTHOR("David Brownell");
