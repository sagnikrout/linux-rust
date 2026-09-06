//! Automatically rewritten from C to Rust
//! Source: net/9p/trans_usbg.c
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
// trans_usbg.c - USB peripheral usb9pfs configuration driver and transport.
//
// Copyright (C) 2024 Michael Grzeschik <m.grzeschik@pengutronix.de>
//
// Gadget usb9pfs only needs two bulk endpoints, and will use the usb9pfs
// transport to mount host exported filesystem via usb gadget.
//
// +--------------------------+    |    +--------------------------+
// |  9PFS mounting client    |    |    |  9PFS exporting server   |
// SW |                          |    |    |                          |
// |   (this:trans_usbg)      |    |    |(e.g. diod or nfs-ganesha)|
// +-------------^------------+    |    +-------------^------------+
// |                 |                  |
// ------------------|------------------------------------|-------------
// |                 |                  |
// +-------------v------------+    |    +-------------v------------+
// |                          |    |    |                          |
// HW |   USB Device Controller  <--------->   USB Host Controller    |
// |                          |    |    |                          |
// +--------------------------+    |    +--------------------------+
//

pub const DEFAULT_BUFLEN: c_int = 16384;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct f_usb9pfs {
    pub client: *mut p9_client,
// 9p request lock for en/dequeue
    pub lock: spinlock_t,
    pub in_req: *mut usb_request,
    pub out_req: *mut usb_request,
    pub in_ep: *mut usb_ep,
    pub out_ep: *mut usb_ep,
    pub send: completion,
    pub received: completion,
    pub buflen: c_uint,
    pub function: usb_function,
}

    static inline struct f_usb9pfs *func_to_usb9pfs(struct usb_function *f)
    {
    return container_of(f, struct f_usb9pfs, function);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct f_usb9pfs_opts {
    pub func_inst: usb_function_instance,
    pub buflen: c_uint,
    pub dev: *mut f_usb9pfs_dev,
// Read/write access to configfs attributes is handled by configfs.
//
// This is to protect the data from concurrent access by read/write
// and create symlink/remove symlink.
//
    pub lock: mutex,
    pub refcnt: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f_usb9pfs_dev {
    pub usb9pfs: *mut f_usb9pfs,
    pub opts: *mut f_usb9pfs_opts,
    pub tag: [c_char; 41],
    pub inuse: bool,
    pub usb9pfs_instance: list_head,
}

    static DEFINE_MUTEX(usb9pfs_lock);
    static struct list_head usbg_instance_list;
    static int usb9pfs_queue_tx(struct f_usb9pfs *usb9pfs, struct p9_req_t *p9_tx_req,
    gfp_t gfp_flags)
    {
    struct usb_composite_dev *cdev = usb9pfs.function.config.cdev;
    struct usb_request *req = usb9pfs.in_req;
    int ret;
    if (!(p9_tx_req.tc.size % usb9pfs.in_ep.maxpacket))
    req.zero = 1;
    req.buf = p9_tx_req.tc.sdata;
    req.length = p9_tx_req.tc.size;
    req.context = p9_tx_req;
    dev_dbg(&cdev.gadget.dev, "%s usb9pfs send -. %d/%d, zero: %d\n",
    usb9pfs.in_ep.name, req.actual, req.length, req.zero);
    ret = usb_ep_queue(usb9pfs.in_ep, req, gfp_flags);
    if (ret)
    req.context = core::ptr::null_mut();
    dev_dbg(&cdev.gadget.dev, "tx submit -. %d\n", ret);
    return ret;
    }
    static int usb9pfs_queue_rx(struct f_usb9pfs *usb9pfs, struct usb_request *req,
    gfp_t gfp_flags)
    {
    struct usb_composite_dev *cdev = usb9pfs.function.config.cdev;
    int ret;
    ret = usb_ep_queue(usb9pfs.out_ep, req, gfp_flags);
    dev_dbg(&cdev.gadget.dev, "rx submit -. %d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn usb9pfs_transmit(usb9pfs: *mut f_usb9pfs, p9_req: *mut p9_req_t) -> c_int {
    static int usb9pfs_transmit(struct f_usb9pfs *usb9pfs, struct p9_req_t *p9_req)
    {
    let mut ret: c_int = 0;
    guard(spinlock_irqsave)(&usb9pfs.lock);
    ret = usb9pfs_queue_tx(usb9pfs, p9_req, GFP_ATOMIC);
    if (ret)
    return ret;
    list_del(&p9_req.req_list);
    p9_req_get(p9_req);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn usb9pfs_tx_complete(ep: *mut usb_ep, req: *mut usb_request) {
    static void usb9pfs_tx_complete(struct usb_ep *ep, struct usb_request *req)
    {
    struct f_usb9pfs *usb9pfs = ep.driver_data;
    struct usb_composite_dev *cdev = usb9pfs.function.config.cdev;
    struct p9_req_t *p9_tx_req = req.context;
    unsigned long flags;
// reset zero packages
    req.zero = 0;
    if (req.status) {
    dev_err(&cdev.gadget.dev, "%s usb9pfs complete -. %d, %d/%d\n",
    ep.name, req.status, req.actual, req.length);
    return;
    }
    dev_dbg(&cdev.gadget.dev, "%s usb9pfs complete -. %d, %d/%d\n",
    ep.name, req.status, req.actual, req.length);
    spin_lock_irqsave(&usb9pfs.lock, flags);
    WRITE_ONCE(p9_tx_req.status, REQ_STATUS_SENT);
    p9_req_put(usb9pfs.client, p9_tx_req);
    req.context = core::ptr::null_mut();
    spin_unlock_irqrestore(&usb9pfs.lock, flags);
    complete(&usb9pfs.send);
    }
    static struct p9_req_t *usb9pfs_rx_header(struct f_usb9pfs *usb9pfs, void *buf)
    {
    struct p9_req_t *p9_rx_req;
    struct p9_fcall	rc;
    int ret;
// start by reading header
    rc.sdata = buf;
    rc.offset = 0;
    rc.capacity = P9_HDRSZ;
    rc.size = P9_HDRSZ;
    p9_debug(P9_DEBUG_TRANS, "mux %p got %zu bytes\n", usb9pfs,
    rc.capacity - rc.offset);
    ret = p9_parse_header(&rc, &rc.size, core::ptr::null_mut(), core::ptr::null_mut(), 0);
    if (ret) {
    p9_debug(P9_DEBUG_ERROR,
    "error parsing header: %d\n", ret);
    return core::ptr::null_mut();
    }
    p9_debug(P9_DEBUG_TRANS,
    "mux %p pkt: size: %d bytes tag: %d\n",
    usb9pfs, rc.size, rc.tag);
    p9_rx_req = p9_tag_lookup(usb9pfs.client, rc.tag);
    if (!p9_rx_req || p9_rx_req.status != REQ_STATUS_SENT) {
    p9_debug(P9_DEBUG_ERROR, "Unexpected packet tag %d\n", rc.tag);
    return core::ptr::null_mut();
    }
    if (rc.size > p9_rx_req.rc.capacity) {
    p9_debug(P9_DEBUG_ERROR,
    "requested packet size too big: %d for tag %d with capacity %zd\n",
    rc.size, rc.tag, p9_rx_req.rc.capacity);
    p9_req_put(usb9pfs.client, p9_rx_req);
    return core::ptr::null_mut();
    }
    if (!p9_rx_req.rc.sdata) {
    p9_debug(P9_DEBUG_ERROR,
    "No recv fcall for tag %d (req %p), disconnecting!\n",
    rc.tag, p9_rx_req);
    p9_req_put(usb9pfs.client, p9_rx_req);
    return core::ptr::null_mut();
    }
    return p9_rx_req;
    }
#[no_mangle]
unsafe extern "C" fn usb9pfs_rx_complete(ep: *mut usb_ep, req: *mut usb_request) {
    static void usb9pfs_rx_complete(struct usb_ep *ep, struct usb_request *req)
    {
    struct f_usb9pfs *usb9pfs = ep.driver_data;
    struct usb_composite_dev *cdev = usb9pfs.function.config.cdev;
    struct p9_req_t *p9_rx_req;
    let mut req_size: c_uint = req.actual;
    let mut status: c_int = REQ_STATUS_RCVD;
    if (req.status) {
    dev_err(&cdev.gadget.dev, "%s usb9pfs complete -. %d, %d/%d\n",
    ep.name, req.status, req.actual, req.length);
    return;
    }
    p9_rx_req = usb9pfs_rx_header(usb9pfs, req.buf);
    if (!p9_rx_req)
    return;
    if (req_size > p9_rx_req.rc.capacity) {
    dev_err(&cdev.gadget.dev,
    "%s received data size %u exceeds buffer capacity %zu\n",
    ep.name, req_size, p9_rx_req.rc.capacity);
    req_size = 0;
    status = REQ_STATUS_ERROR;
    }
    memcpy(p9_rx_req.rc.sdata, req.buf, req_size);
    p9_rx_req.rc.size = req_size;
    p9_client_cb(usb9pfs.client, p9_rx_req, status);
    p9_req_put(usb9pfs.client, p9_rx_req);
    complete(&usb9pfs.received);
    }
#[no_mangle]
unsafe extern "C" fn disable_ep(cdev: *mut usb_composite_dev, ep: *mut usb_ep) {
    static void disable_ep(struct usb_composite_dev *cdev, struct usb_ep *ep)
    {
    int value;
    value = usb_ep_disable(ep);
    if (value < 0)
    dev_info(&cdev.gadget.dev,
    "disable %s -. %d\n", ep.name, value);
    }
#[no_mangle]
unsafe extern "C" fn disable_usb9pfs(usb9pfs: *mut f_usb9pfs) {
    static void disable_usb9pfs(struct f_usb9pfs *usb9pfs)
    {
    struct usb_composite_dev *cdev =
    usb9pfs.function.config.cdev;
    if (usb9pfs.in_req) {
    usb_ep_free_request(usb9pfs.in_ep, usb9pfs.in_req);
    usb9pfs.in_req = core::ptr::null_mut();
    }
    if (usb9pfs.out_req) {
    usb_ep_free_request(usb9pfs.out_ep, usb9pfs.out_req);
    usb9pfs.out_req = core::ptr::null_mut();
    }
    disable_ep(cdev, usb9pfs.in_ep);
    disable_ep(cdev, usb9pfs.out_ep);
    dev_dbg(&cdev.gadget.dev, "%s disabled\n",
    usb9pfs.function.name);
    }
    static int alloc_requests(struct usb_composite_dev *cdev,
    struct f_usb9pfs *usb9pfs)
    {
    int ret;
    usb9pfs.in_req = usb_ep_alloc_request(usb9pfs.in_ep, GFP_ATOMIC);
    if (!usb9pfs.in_req) {
    ret = -ENOENT;
    goto fail;
    }
    usb9pfs.out_req = alloc_ep_req(usb9pfs.out_ep, usb9pfs.buflen);
    if (!usb9pfs.out_req) {
    ret = -ENOENT;
    goto fail_in;
    }
    usb9pfs.in_req.complete = usb9pfs_tx_complete;
    usb9pfs.out_req.complete = usb9pfs_rx_complete;
// length will be set in complete routine
    usb9pfs.in_req.context = usb9pfs;
    usb9pfs.out_req.context = usb9pfs;
    return 0;
    fail_in:
    usb_ep_free_request(usb9pfs.in_ep, usb9pfs.in_req);
    fail:
    return ret;
    }
    static int enable_endpoint(struct usb_composite_dev *cdev,
    struct f_usb9pfs *usb9pfs, struct usb_ep *ep)
    {
    int ret;
    ret = config_ep_by_speed(cdev.gadget, &usb9pfs.function, ep);
    if (ret)
    return ret;
    ret = usb_ep_enable(ep);
    if (ret < 0)
    return ret;
    ep.driver_data = usb9pfs;
    return 0;
    }
    static int
    enable_usb9pfs(struct usb_composite_dev *cdev, struct f_usb9pfs *usb9pfs)
    {
    struct p9_client *client;
    let mut ret: c_int = 0;
    ret = enable_endpoint(cdev, usb9pfs, usb9pfs.in_ep);
    if (ret)
    goto out;
    ret = enable_endpoint(cdev, usb9pfs, usb9pfs.out_ep);
    if (ret)
    goto disable_in;
    ret = alloc_requests(cdev, usb9pfs);
    if (ret)
    goto disable_out;
    client = usb9pfs.client;
    if (client)
    client.status = Connected;
    dev_dbg(&cdev.gadget.dev, "%s enabled\n", usb9pfs.function.name);
    return 0;
    disable_out:
    usb_ep_disable(usb9pfs.out_ep);
    disable_in:
    usb_ep_disable(usb9pfs.in_ep);
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn p9_usbg_create(client: *mut p9_client, fc: *mut fs_context) -> c_int {
    static int p9_usbg_create(struct p9_client *client, struct fs_context *fc)
    {
    const char *devname = fc.source;
    struct f_usb9pfs_dev *dev;
    struct f_usb9pfs *usb9pfs;
    let mut ret: c_int = -ENOENT;
    let mut found: c_int = 0;
    if (!devname)
    return -EINVAL;
    guard(mutex)(&usb9pfs_lock);
    list_for_each_entry(dev, &usbg_instance_list, usb9pfs_instance) {
    if (!strncmp(devname, dev.tag, strlen(devname))) {
    if (!dev.inuse) {
    dev.inuse = true;
    found = 1;
    break;
    }
    ret = -EBUSY;
    break;
    }
    }
    if (!found) {
    pr_err("no channels available for device %s\n", devname);
    return ret;
    }
    usb9pfs = dev.usb9pfs;
    if (!usb9pfs)
    return -EINVAL;
    client.trans = (void *)usb9pfs;
    if (!usb9pfs.in_req)
    client.status = Disconnected;
    else
    client.status = Connected;
    usb9pfs.client = client;
    client.trans_mod.maxsize = usb9pfs.buflen;
    complete(&usb9pfs.received);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn usb9pfs_clear_tx(usb9pfs: *mut f_usb9pfs) {
    static void usb9pfs_clear_tx(struct f_usb9pfs *usb9pfs)
    {
    struct p9_req_t *req;
    guard(spinlock_irqsave)(&usb9pfs.lock);
    req = usb9pfs.in_req.context;
    if (!req)
    return;
    if (!req.t_err)
    req.t_err = -ECONNRESET;
    p9_client_cb(usb9pfs.client, req, REQ_STATUS_ERROR);
    }
#[no_mangle]
unsafe extern "C" fn p9_usbg_close(client: *mut p9_client) {
    static void p9_usbg_close(struct p9_client *client)
    {
    struct f_usb9pfs *usb9pfs;
    struct f_usb9pfs_dev *dev;
    struct f_usb9pfs_opts *opts;
    if (!client)
    return;
    usb9pfs = client.trans;
    if (!usb9pfs)
    return;
    client.status = Disconnected;
    usb9pfs_clear_tx(usb9pfs);
    opts = container_of(usb9pfs.function.fi,
    struct f_usb9pfs_opts, func_inst);
    dev = opts.dev;
    mutex_lock(&usb9pfs_lock);
    dev.inuse = false;
    mutex_unlock(&usb9pfs_lock);
    }
#[no_mangle]
unsafe extern "C" fn p9_usbg_request(client: *mut p9_client, p9_req: *mut p9_req_t) -> c_int {
    static int p9_usbg_request(struct p9_client *client, struct p9_req_t *p9_req)
    {
    struct f_usb9pfs *usb9pfs = client.trans;
    int ret;
    if (client.status != Connected)
    return -EBUSY;
    ret = wait_for_completion_killable(&usb9pfs.received);
    if (ret)
    return ret;
    ret = usb9pfs_transmit(usb9pfs, p9_req);
    if (ret)
    return ret;
    ret = wait_for_completion_killable(&usb9pfs.send);
    if (ret)
    return ret;
    return usb9pfs_queue_rx(usb9pfs, usb9pfs.out_req, GFP_ATOMIC);
    }
#[no_mangle]
unsafe extern "C" fn p9_usbg_cancel(client: *mut p9_client, req: *mut p9_req_t) -> c_int {
    static int p9_usbg_cancel(struct p9_client *client, struct p9_req_t *req)
    {
    struct f_usb9pfs *usb9pfs = client.trans;
    let mut ret: c_int = 1;
    p9_debug(P9_DEBUG_TRANS, "client %p req %p\n", client, req);
    guard(spinlock_irqsave)(&usb9pfs.lock);
    if (req.status == REQ_STATUS_UNSENT) {
    list_del(&req.req_list);
    WRITE_ONCE(req.status, REQ_STATUS_FLSHD);
    p9_req_put(client, req);
    ret = 0;
    }
    return ret;
    }
    static struct p9_trans_module p9_usbg_trans = {
    .name = "usbg",
    .create = p9_usbg_create,
    .close = p9_usbg_close,
    .request = p9_usbg_request,
    .cancel = p9_usbg_cancel,
    .supports_vmalloc = false,
    .owner = THIS_MODULE,
    };
// -------------------------------------------------------------------------
pub const USB_PROTOCOL_9PFS: c_uint = 0x09;
    static struct usb_interface_descriptor usb9pfs_intf = {
    .bLength =		sizeof(usb9pfs_intf),
    .bDescriptorType =	USB_DT_INTERFACE,
    .bNumEndpoints =	2,
    .bInterfaceClass =	USB_CLASS_VENDOR_SPEC,
    .bInterfaceSubClass =	USB_SUBCLASS_VENDOR_SPEC,
    .bInterfaceProtocol =   USB_PROTOCOL_9PFS,
// .iInterface = DYNAMIC
    };
// full speed support:
    static struct usb_endpoint_descriptor fs_usb9pfs_source_desc = {
    .bLength =		USB_DT_ENDPOINT_SIZE,
    .bDescriptorType =	USB_DT_ENDPOINT,
    .bEndpointAddress =	USB_DIR_IN,
    .bmAttributes =		USB_ENDPOINT_XFER_BULK,
    };
    static struct usb_endpoint_descriptor fs_usb9pfs_sink_desc = {
    .bLength =		USB_DT_ENDPOINT_SIZE,
    .bDescriptorType =	USB_DT_ENDPOINT,
    .bEndpointAddress =	USB_DIR_OUT,
    .bmAttributes =		USB_ENDPOINT_XFER_BULK,
    };
    static struct usb_descriptor_header *fs_usb9pfs_descs[] = {
    (struct usb_descriptor_header *)&usb9pfs_intf,
    (struct usb_descriptor_header *)&fs_usb9pfs_sink_desc,
    (struct usb_descriptor_header *)&fs_usb9pfs_source_desc,
    core::ptr::null_mut(),
    };
// high speed support:
    static struct usb_endpoint_descriptor hs_usb9pfs_source_desc = {
    .bLength =		USB_DT_ENDPOINT_SIZE,
    .bDescriptorType =	USB_DT_ENDPOINT,
    .bmAttributes =		USB_ENDPOINT_XFER_BULK,
    .wMaxPacketSize =	cpu_to_le16(512),
    };
    static struct usb_endpoint_descriptor hs_usb9pfs_sink_desc = {
    .bLength =		USB_DT_ENDPOINT_SIZE,
    .bDescriptorType =	USB_DT_ENDPOINT,
    .bmAttributes =		USB_ENDPOINT_XFER_BULK,
    .wMaxPacketSize =	cpu_to_le16(512),
    };
    static struct usb_descriptor_header *hs_usb9pfs_descs[] = {
    (struct usb_descriptor_header *)&usb9pfs_intf,
    (struct usb_descriptor_header *)&hs_usb9pfs_source_desc,
    (struct usb_descriptor_header *)&hs_usb9pfs_sink_desc,
    core::ptr::null_mut(),
    };
// super speed support:
    static struct usb_endpoint_descriptor ss_usb9pfs_source_desc = {
    .bLength =		USB_DT_ENDPOINT_SIZE,
    .bDescriptorType =	USB_DT_ENDPOINT,
    .bmAttributes =		USB_ENDPOINT_XFER_BULK,
    .wMaxPacketSize =	cpu_to_le16(1024),
    };
    static struct usb_ss_ep_comp_descriptor ss_usb9pfs_source_comp_desc = {
    .bLength =		USB_DT_SS_EP_COMP_SIZE,
    .bDescriptorType =	USB_DT_SS_ENDPOINT_COMP,
    .bMaxBurst =		0,
    .bmAttributes =		0,
    .wBytesPerInterval =	0,
    };
    static struct usb_endpoint_descriptor ss_usb9pfs_sink_desc = {
    .bLength =		USB_DT_ENDPOINT_SIZE,
    .bDescriptorType =	USB_DT_ENDPOINT,
    .bmAttributes =		USB_ENDPOINT_XFER_BULK,
    .wMaxPacketSize =	cpu_to_le16(1024),
    };
    static struct usb_ss_ep_comp_descriptor ss_usb9pfs_sink_comp_desc = {
    .bLength =		USB_DT_SS_EP_COMP_SIZE,
    .bDescriptorType =	USB_DT_SS_ENDPOINT_COMP,
    .bMaxBurst =		0,
    .bmAttributes =		0,
    .wBytesPerInterval =	0,
    };
    static struct usb_descriptor_header *ss_usb9pfs_descs[] = {
    (struct usb_descriptor_header *)&usb9pfs_intf,
    (struct usb_descriptor_header *)&ss_usb9pfs_source_desc,
    (struct usb_descriptor_header *)&ss_usb9pfs_source_comp_desc,
    (struct usb_descriptor_header *)&ss_usb9pfs_sink_desc,
    (struct usb_descriptor_header *)&ss_usb9pfs_sink_comp_desc,
    core::ptr::null_mut(),
    };
// function-specific strings:
    static struct usb_string strings_usb9pfs[] = {
    [0].s = "usb9pfs input to output",
    {  }			/* end of list */
    };
    static struct usb_gadget_strings stringtab_usb9pfs = {
    .language	= 0x0409,	/* en-us */
    .strings	= strings_usb9pfs,
    };
    static struct usb_gadget_strings *usb9pfs_strings[] = {
    &stringtab_usb9pfs,
    core::ptr::null_mut(),
    };
// -------------------------------------------------------------------------
    static int usb9pfs_func_bind(struct usb_configuration *c,
    struct usb_function *f)
    {
    struct f_usb9pfs *usb9pfs = func_to_usb9pfs(f);
    struct f_usb9pfs_opts *opts;
    struct usb_composite_dev *cdev = c.cdev;
    int ret;
    int id;
// allocate interface ID(s)
    id = usb_interface_id(c, f);
    if (id < 0)
    return id;
    usb9pfs_intf.bInterfaceNumber = id;
    id = usb_string_id(cdev);
    if (id < 0)
    return id;
    strings_usb9pfs[0].id = id;
    usb9pfs_intf.iInterface = id;
// allocate endpoints
    usb9pfs.in_ep = usb_ep_autoconfig(cdev.gadget,
    &fs_usb9pfs_source_desc);
    if (!usb9pfs.in_ep)
    goto autoconf_fail;
    usb9pfs.out_ep = usb_ep_autoconfig(cdev.gadget,
    &fs_usb9pfs_sink_desc);
    if (!usb9pfs.out_ep)
    goto autoconf_fail;
// support high speed hardware
    hs_usb9pfs_source_desc.bEndpointAddress =
    fs_usb9pfs_source_desc.bEndpointAddress;
    hs_usb9pfs_sink_desc.bEndpointAddress =
    fs_usb9pfs_sink_desc.bEndpointAddress;
// support super speed hardware
    ss_usb9pfs_source_desc.bEndpointAddress =
    fs_usb9pfs_source_desc.bEndpointAddress;
    ss_usb9pfs_sink_desc.bEndpointAddress =
    fs_usb9pfs_sink_desc.bEndpointAddress;
    ret = usb_assign_descriptors(f, fs_usb9pfs_descs, hs_usb9pfs_descs,
    ss_usb9pfs_descs, ss_usb9pfs_descs);
    if (ret)
    return ret;
    opts = container_of(f.fi, struct f_usb9pfs_opts, func_inst);
    opts.dev.usb9pfs = usb9pfs;
    dev_dbg(&cdev.gadget.dev, "%s speed %s: IN/%s, OUT/%s\n",
    (gadget_is_superspeed(c.cdev.gadget) ? "super" :
    (gadget_is_dualspeed(c.cdev.gadget) ? "dual" : "full")),
    f.name, usb9pfs.in_ep.name, usb9pfs.out_ep.name);
    return 0;
    autoconf_fail:
    ERROR(cdev, "%s: can't autoconfigure on %s\n",
    f.name, cdev.gadget.name);
    return -ENODEV;
    }
    static void usb9pfs_func_unbind(struct usb_configuration *c,
    struct usb_function *f)
    {
    struct f_usb9pfs *usb9pfs = func_to_usb9pfs(f);
    disable_usb9pfs(usb9pfs);
    }
#[no_mangle]
unsafe extern "C" fn usb9pfs_free_func(f: *mut usb_function) {
    static void usb9pfs_free_func(struct usb_function *f)
    {
    struct f_usb9pfs *usb9pfs = func_to_usb9pfs(f);
    struct f_usb9pfs_opts *opts;
    kfree(usb9pfs);
    opts = container_of(f.fi, struct f_usb9pfs_opts, func_inst);
    mutex_lock(&opts.lock);
    opts.refcnt--;
    mutex_unlock(&opts.lock);
    usb_free_all_descriptors(f);
    }
    static int usb9pfs_set_alt(struct usb_function *f,
    unsigned int intf, unsigned int alt)
    {
    struct f_usb9pfs *usb9pfs = func_to_usb9pfs(f);
    struct usb_composite_dev *cdev = f.config.cdev;
    return enable_usb9pfs(cdev, usb9pfs);
    }
#[no_mangle]
unsafe extern "C" fn usb9pfs_disable(f: *mut usb_function) {
    static void usb9pfs_disable(struct usb_function *f)
    {
    struct f_usb9pfs *usb9pfs = func_to_usb9pfs(f);
    usb9pfs_clear_tx(usb9pfs);
    }
    static struct usb_function *usb9pfs_alloc(struct usb_function_instance *fi)
    {
    struct f_usb9pfs_opts *usb9pfs_opts;
    struct f_usb9pfs *usb9pfs;
    usb9pfs = kzalloc_obj(*usb9pfs);
    if (!usb9pfs)
    return ERR_PTR(-ENOMEM);
    spin_lock_init(&usb9pfs.lock);
    init_completion(&usb9pfs.send);
    init_completion(&usb9pfs.received);
    usb9pfs_opts = container_of(fi, struct f_usb9pfs_opts, func_inst);
    mutex_lock(&usb9pfs_opts.lock);
    usb9pfs_opts.refcnt++;
    mutex_unlock(&usb9pfs_opts.lock);
    usb9pfs.buflen = usb9pfs_opts.buflen;
    usb9pfs.function.name = "usb9pfs";
    usb9pfs.function.bind = usb9pfs_func_bind;
    usb9pfs.function.unbind = usb9pfs_func_unbind;
    usb9pfs.function.set_alt = usb9pfs_set_alt;
    usb9pfs.function.disable = usb9pfs_disable;
    usb9pfs.function.strings = usb9pfs_strings;
    usb9pfs.function.free_func = usb9pfs_free_func;
    return &usb9pfs.function;
    }
    static inline struct f_usb9pfs_opts *to_f_usb9pfs_opts(struct config_item *item)
    {
    return container_of(to_config_group(item), struct f_usb9pfs_opts,
    func_inst.group);
    }
    static inline struct f_usb9pfs_opts *fi_to_f_usb9pfs_opts(struct usb_function_instance *fi)
    {
    return container_of(fi, struct f_usb9pfs_opts, func_inst);
    }
#[no_mangle]
unsafe extern "C" fn usb9pfs_attr_release(item: *mut config_item) {
    static void usb9pfs_attr_release(struct config_item *item)
    {
    struct f_usb9pfs_opts *usb9pfs_opts = to_f_usb9pfs_opts(item);
    usb_put_function_instance(&usb9pfs_opts.func_inst);
    }
    static const struct configfs_item_operations usb9pfs_item_ops = {
    .release		= usb9pfs_attr_release,
    };
#[no_mangle]
unsafe extern "C" fn f_usb9pfs_opts_buflen_show(item: *mut config_item, page: *mut c_char) -> isize {
    static ssize_t f_usb9pfs_opts_buflen_show(struct config_item *item, char *page)
    {
    struct f_usb9pfs_opts *opts = to_f_usb9pfs_opts(item);
    int ret;
    mutex_lock(&opts.lock);
    ret = sysfs_emit(page, "%d\n", opts.buflen);
    mutex_unlock(&opts.lock);
    return ret;
    }
    static ssize_t f_usb9pfs_opts_buflen_store(struct config_item *item,
    const char *page, size_t len)
    {
    struct f_usb9pfs_opts *opts = to_f_usb9pfs_opts(item);
    int ret;
    u32 num;
    guard(mutex)(&opts.lock);
    if (opts.refcnt)
    return -EBUSY;
    ret = kstrtou32(page, 0, &num);
    if (ret)
    return ret;
    opts.buflen = num;
    return len;
    }
    CONFIGFS_ATTR(f_usb9pfs_opts_, buflen);
    static struct configfs_attribute *usb9pfs_attrs[] = {
    &f_usb9pfs_opts_attr_buflen,
    core::ptr::null_mut(),
    };
    static const struct config_item_type usb9pfs_func_type = {
    .ct_item_ops	= &usb9pfs_item_ops,
    .ct_attrs	= usb9pfs_attrs,
    .ct_owner	= THIS_MODULE,
    };
    static struct f_usb9pfs_dev *_usb9pfs_do_find_dev(const char *tag)
    {
    struct f_usb9pfs_dev *usb9pfs_dev;
    if (!tag)
    return core::ptr::null_mut();
    list_for_each_entry(usb9pfs_dev, &usbg_instance_list, usb9pfs_instance) {
    if (strcmp(usb9pfs_dev.tag, tag) == 0)
    return usb9pfs_dev;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn usb9pfs_tag_instance(dev: *mut f_usb9pfs_dev, tag: *const c_char) -> c_int {
    static int usb9pfs_tag_instance(struct f_usb9pfs_dev *dev, const char *tag)
    {
    struct f_usb9pfs_dev *existing;
    let mut ret: c_int = 0;
    guard(mutex)(&usb9pfs_lock);
    existing = _usb9pfs_do_find_dev(tag);
    if (!existing)
    strscpy(dev.tag, tag, ARRAY_SIZE(dev.tag));
#[no_mangle]
pub unsafe extern "C" fn if(dev: existing !=) -> else {
    else if (existing != dev)
    ret = -EBUSY;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn usb9pfs_set_inst_tag(fi: *mut usb_function_instance, tag: *const c_char) -> c_int {
    static int usb9pfs_set_inst_tag(struct usb_function_instance *fi, const char *tag)
    {
    if (strlen(tag) >= sizeof_field(struct f_usb9pfs_dev, tag))
    return -ENAMETOOLONG;
    return usb9pfs_tag_instance(fi_to_f_usb9pfs_opts(fi).dev, tag);
    }
#[no_mangle]
unsafe extern "C" fn usb9pfs_free_instance(fi: *mut usb_function_instance) {
    static void usb9pfs_free_instance(struct usb_function_instance *fi)
    {
    struct f_usb9pfs_opts *usb9pfs_opts =
    container_of(fi, struct f_usb9pfs_opts, func_inst);
    struct f_usb9pfs_dev *dev = usb9pfs_opts.dev;
    mutex_lock(&usb9pfs_lock);
    list_del(&dev.usb9pfs_instance);
    mutex_unlock(&usb9pfs_lock);
    kfree(usb9pfs_opts);
    }
    static struct usb_function_instance *usb9pfs_alloc_instance(void)
    {
    struct f_usb9pfs_opts *usb9pfs_opts;
    struct f_usb9pfs_dev *dev;
    usb9pfs_opts = kzalloc_obj(*usb9pfs_opts);
    if (!usb9pfs_opts)
    return ERR_PTR(-ENOMEM);
    mutex_init(&usb9pfs_opts.lock);
    usb9pfs_opts.func_inst.set_inst_name = usb9pfs_set_inst_tag;
    usb9pfs_opts.func_inst.free_func_inst = usb9pfs_free_instance;
    usb9pfs_opts.buflen = DEFAULT_BUFLEN;
    dev = kzalloc_obj(*dev);
    if (!dev) {
    kfree(usb9pfs_opts);
    return ERR_PTR(-ENOMEM);
    }
    usb9pfs_opts.dev = dev;
    dev.opts = usb9pfs_opts;
    config_group_init_type_name(&usb9pfs_opts.func_inst.group, "",
    &usb9pfs_func_type);
    mutex_lock(&usb9pfs_lock);
    list_add_tail(&dev.usb9pfs_instance, &usbg_instance_list);
    mutex_unlock(&usb9pfs_lock);
    return &usb9pfs_opts.func_inst;
    }
    DECLARE_USB_FUNCTION(usb9pfs, usb9pfs_alloc_instance, usb9pfs_alloc);
#[no_mangle]
unsafe extern "C" fn usb9pfs_modinit() -> int __init {
    static int __init usb9pfs_modinit(void)
    {
    int ret;
    INIT_LIST_HEAD(&usbg_instance_list);
    ret = usb_function_register(&usb9pfsusb_func);
    if (!ret)
    v9fs_register_trans(&p9_usbg_trans);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn usb9pfs_modexit() -> void __exit {
    static void __exit usb9pfs_modexit(void)
    {
    usb_function_unregister(&usb9pfsusb_func);
    v9fs_unregister_trans(&p9_usbg_trans);
    }
    module_init(usb9pfs_modinit);
    module_exit(usb9pfs_modexit);
    MODULE_ALIAS_9P("usbg");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("USB gadget 9pfs transport");
    MODULE_AUTHOR("Michael Grzeschik");
