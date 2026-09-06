//! Automatically rewritten from C to Rust
//! Source: net/9p/trans_xen.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// linux/fs/9p/trans_xen
//
// Xen transport layer.
//
// Copyright (C) 2017 by Stefano Stabellini <stefano@aporeto.com>
//

pub const XEN_9PFS_NUM_RINGS: c_int = 2;
pub const XEN_9PFS_RING_ORDER: c_int = 9;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_9pfs_header {
    pub size: u32,
    pub id: u8,
    pub tag: u16,
// uint8_t sdata[];
    pub __attribute__((packed)): },
// One per ring, more than one per 9pfs share
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_9pfs_dataring {
    pub priv: *mut xen_9pfs_front_priv,
    pub intf: *mut xen_9pfs_data_intf,
    pub ref: grant_ref_t,
    pub evtchn: c_int,
    pub irq: c_int,
// protect a ring from concurrent accesses
    pub lock: spinlock_t,
    pub data: xen_9pfs_data,
    pub wq: wait_queue_head_t,
    pub work: work_struct,
}

// One per 9pfs share
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_9pfs_front_priv {
    pub list: list_head,
    pub dev: *mut xenbus_device,
    pub tag: *mut c_char,
    pub client: *mut p9_client,
    pub rings: *mut xen_9pfs_dataring,
}

    static LIST_HEAD(xen_9pfs_devs);
    static DEFINE_RWLOCK(xen_9pfs_lock);
// We don't currently allow canceling of requests
#[no_mangle]
unsafe extern "C" fn p9_xen_cancel(client: *mut p9_client, req: *mut p9_req_t) -> c_int {
    static int p9_xen_cancel(struct p9_client *client, struct p9_req_t *req)
    {
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn p9_xen_create(client: *mut p9_client, fc: *mut fs_context) -> c_int {
    static int p9_xen_create(struct p9_client *client, struct fs_context *fc)
    {
    const char *addr = fc.source;
    struct xen_9pfs_front_priv *priv;
    if (addr == core::ptr::null_mut())
    return -EINVAL;
    read_lock(&xen_9pfs_lock);
    list_for_each_entry(priv, &xen_9pfs_devs, list) {
    if (!strcmp(priv.tag, addr)) {
    priv.client = client;
    read_unlock(&xen_9pfs_lock);
    return 0;
    }
    }
    read_unlock(&xen_9pfs_lock);
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn p9_xen_close(client: *mut p9_client) {
    static void p9_xen_close(struct p9_client *client)
    {
    struct xen_9pfs_front_priv *priv;
    read_lock(&xen_9pfs_lock);
    list_for_each_entry(priv, &xen_9pfs_devs, list) {
    if (priv.client == client) {
    priv.client = core::ptr::null_mut();
    read_unlock(&xen_9pfs_lock);
    return;
    }
    }
    read_unlock(&xen_9pfs_lock);
    }
#[no_mangle]
unsafe extern "C" fn p9_xen_write_todo(ring: *mut xen_9pfs_dataring, size: RING_IDX) -> bool {
    static bool p9_xen_write_todo(struct xen_9pfs_dataring *ring, RING_IDX size)
    {
    RING_IDX cons, prod;
    cons = ring.intf.out_cons;
    prod = ring.intf.out_prod;
    virt_mb();
    return XEN_9PFS_RING_SIZE(ring) -
    xen_9pfs_queued(prod, cons, XEN_9PFS_RING_SIZE(ring)) >= size;
    }
#[no_mangle]
unsafe extern "C" fn p9_xen_request(client: *mut p9_client, p9_req: *mut p9_req_t) -> c_int {
    static int p9_xen_request(struct p9_client *client, struct p9_req_t *p9_req)
    {
    struct xen_9pfs_front_priv *priv;
    RING_IDX cons, prod, masked_cons, masked_prod;
    unsigned long flags;
    let mut size: u32 = p9_req.tc.size;
    struct xen_9pfs_dataring *ring;
    int num;
    read_lock(&xen_9pfs_lock);
    list_for_each_entry(priv, &xen_9pfs_devs, list) {
    if (priv.client == client)
    break;
    }
    read_unlock(&xen_9pfs_lock);
    if (list_entry_is_head(priv, &xen_9pfs_devs, list))
    return -EINVAL;
    num = p9_req.tc.tag % XEN_9PFS_NUM_RINGS;
    ring = &priv.rings[num];
    again:
    while (io_wait_event_killable(ring.wq,
    p9_xen_write_todo(ring, size)) != 0)
    ;
    spin_lock_irqsave(&ring.lock, flags);
    cons = ring.intf.out_cons;
    prod = ring.intf.out_prod;
    virt_mb();
    if (XEN_9PFS_RING_SIZE(ring) -
    xen_9pfs_queued(prod, cons, XEN_9PFS_RING_SIZE(ring)) < size) {
    spin_unlock_irqrestore(&ring.lock, flags);
    goto again;
    }
    masked_prod = xen_9pfs_mask(prod, XEN_9PFS_RING_SIZE(ring));
    masked_cons = xen_9pfs_mask(cons, XEN_9PFS_RING_SIZE(ring));
    xen_9pfs_write_packet(ring.data.out, p9_req.tc.sdata, size,
    &masked_prod, masked_cons,
    XEN_9PFS_RING_SIZE(ring));
    WRITE_ONCE(p9_req.status, REQ_STATUS_SENT);
    virt_wmb();			/* write ring before updating pointer */
    prod += size;
    ring.intf.out_prod = prod;
    spin_unlock_irqrestore(&ring.lock, flags);
    notify_remote_via_irq(ring.irq);
    p9_req_put(client, p9_req);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn p9_xen_response(work: *mut work_struct) {
    static void p9_xen_response(struct work_struct *work)
    {
    struct xen_9pfs_front_priv *priv;
    struct xen_9pfs_dataring *ring;
    RING_IDX cons, prod, masked_cons, masked_prod;
    struct xen_9pfs_header h;
    struct p9_req_t *req;
    int status;
    ring = container_of(work, struct xen_9pfs_dataring, work);
    priv = ring.priv;
    while (1) {
    cons = ring.intf.in_cons;
    prod = ring.intf.in_prod;
    virt_rmb();
    if (xen_9pfs_queued(prod, cons, XEN_9PFS_RING_SIZE(ring)) <
    sizeof(h)) {
    notify_remote_via_irq(ring.irq);
    return;
    }
    masked_prod = xen_9pfs_mask(prod, XEN_9PFS_RING_SIZE(ring));
    masked_cons = xen_9pfs_mask(cons, XEN_9PFS_RING_SIZE(ring));
// First, read just the header
    xen_9pfs_read_packet(&h, ring.data.in, sizeof(h),
    masked_prod, &masked_cons,
    XEN_9PFS_RING_SIZE(ring));
    req = p9_tag_lookup(priv.client, h.tag);
    if (!req || req.status != REQ_STATUS_SENT) {
    dev_warn(&priv.dev.dev, "Wrong req tag=%x\n", h.tag);
    cons += h.size;
    virt_mb();
    ring.intf.in_cons = cons;
    continue;
    }
    if (h.size > req.rc.capacity) {
    dev_warn(&priv.dev.dev,
    "requested packet size too big: %d for tag %d with capacity %zd\n",
    h.size, h.tag, req.rc.capacity);
    WRITE_ONCE(req.status, REQ_STATUS_ERROR);
    goto recv_error;
    }
    req.rc.size = h.size;
    req.rc.id = h.id;
    req.rc.tag = h.tag;
    req.rc.offset = 0;
    masked_cons = xen_9pfs_mask(cons, XEN_9PFS_RING_SIZE(ring));
// Then, read the whole packet (including the header)
    xen_9pfs_read_packet(req.rc.sdata, ring.data.in, h.size,
    masked_prod, &masked_cons,
    XEN_9PFS_RING_SIZE(ring));
    recv_error:
    virt_mb();
    cons += h.size;
    ring.intf.in_cons = cons;
    status = (req.status != REQ_STATUS_ERROR) ?
    REQ_STATUS_RCVD : REQ_STATUS_ERROR;
    p9_client_cb(priv.client, req, status);
    }
    }
#[no_mangle]
unsafe extern "C" fn xen_9pfs_front_event_handler(irq: c_int, r: *mut c_void) -> irqreturn_t {
    static irqreturn_t xen_9pfs_front_event_handler(int irq, void *r)
    {
    struct xen_9pfs_dataring *ring = r;
    if (!ring || !ring.priv.client) {
// ignore spurious interrupt
    return IRQ_HANDLED;
    }
    wake_up_interruptible(&ring.wq);
    schedule_work(&ring.work);
    return IRQ_HANDLED;
    }
    static struct p9_trans_module p9_xen_trans = {
    .name = "xen",
    .maxsize = 1 << (XEN_9PFS_RING_ORDER + XEN_PAGE_SHIFT - 2),
    .pooled_rbuffers = false,
    .def = true,
    .supports_vmalloc = false,
    .create = p9_xen_create,
    .close = p9_xen_close,
    .request = p9_xen_request,
    .cancel = p9_xen_cancel,
    .owner = THIS_MODULE,
    };
    static const struct xenbus_device_id xen_9pfs_front_ids[] = {
    { "9pfs" },
    { "" }
    };
#[no_mangle]
unsafe extern "C" fn xen_9pfs_front_free(priv: *mut xen_9pfs_front_priv) {
    static void xen_9pfs_front_free(struct xen_9pfs_front_priv *priv)
    {
    int i, j;
    if (priv.rings) {
    for (i = 0; i < XEN_9PFS_NUM_RINGS; i++) {
    struct xen_9pfs_dataring *ring = &priv.rings[i];
    cancel_work_sync(&ring.work);
    if (!ring.intf)
    break;
    if (ring.irq >= 0) {
    unbind_from_irqhandler(ring.irq, ring);
    ring.irq = -1;
    }
    if (ring.data.in) {
    for (j = 0; j < (1 << ring.intf.ring_order);
    j++) {
    grant_ref_t ref;
    ref = ring.intf.ref[j];
    gnttab_end_foreign_access(ref, core::ptr::null_mut());
    ring.intf.ref[j] = INVALID_GRANT_REF;
    }
    free_pages_exact(ring.data.in,
    1UL << (ring.intf.ring_order +
    XEN_PAGE_SHIFT));
    ring.data.in = core::ptr::null_mut();
    ring.data.out = core::ptr::null_mut();
    }
    if (ring.ref != INVALID_GRANT_REF) {
    gnttab_end_foreign_access(ring.ref, core::ptr::null_mut());
    ring.ref = INVALID_GRANT_REF;
    }
    free_page((unsigned long)ring.intf);
    ring.intf = core::ptr::null_mut();
    }
    kfree(priv.rings);
    }
    kfree(priv.tag);
    kfree(priv);
    }
#[no_mangle]
unsafe extern "C" fn xen_9pfs_front_remove(dev: *mut xenbus_device) {
    static void xen_9pfs_front_remove(struct xenbus_device *dev)
    {
    struct xen_9pfs_front_priv *priv;
    write_lock(&xen_9pfs_lock);
    priv = dev_get_drvdata(&dev.dev);
    if (priv == core::ptr::null_mut()) {
    write_unlock(&xen_9pfs_lock);
    return;
    }
    dev_set_drvdata(&dev.dev, core::ptr::null_mut());
    list_del(&priv.list);
    write_unlock(&xen_9pfs_lock);
    xen_9pfs_front_free(priv);
    }
    static int xen_9pfs_front_alloc_dataring(struct xenbus_device *dev,
    struct xen_9pfs_dataring *ring,
    unsigned int order)
    {
    let mut i: c_int = 0;
    let mut ret: c_int = -ENOMEM;
    void *bytes = core::ptr::null_mut();
    ring.intf = core::ptr::null_mut();
    ring.data.in = core::ptr::null_mut();
    ring.data.out = core::ptr::null_mut();
    ring.ref = INVALID_GRANT_REF;
    ring.irq = -1;
    init_waitqueue_head(&ring.wq);
    spin_lock_init(&ring.lock);
    INIT_WORK(&ring.work, p9_xen_response);
    ring.intf = (struct xen_9pfs_data_intf *)get_zeroed_page(GFP_KERNEL);
    if (!ring.intf)
    return ret;
    ret = gnttab_grant_foreign_access(dev.otherend_id,
    virt_to_gfn(ring.intf), 0);
    if (ret < 0)
    goto out;
    ring.ref = ret;
    bytes = alloc_pages_exact(1UL << (order + XEN_PAGE_SHIFT),
    GFP_KERNEL | __GFP_ZERO);
    if (!bytes) {
    ret = -ENOMEM;
    goto out;
    }
    for (; i < (1 << order); i++) {
    ret = gnttab_grant_foreign_access(
    dev.otherend_id, virt_to_gfn(bytes) + i, 0);
    if (ret < 0)
    goto out;
    ring.intf.ref[i] = ret;
    }
    ring.intf.ring_order = order;
    ring.data.in = bytes;
    ring.data.out = bytes + XEN_FLEX_RING_SIZE(order);
    ret = xenbus_alloc_evtchn(dev, &ring.evtchn);
    if (ret)
    goto out;
    ring.irq = bind_evtchn_to_irqhandler(ring.evtchn,
    xen_9pfs_front_event_handler,
    0, "xen_9pfs-frontend", ring);
    if (ring.irq >= 0)
    return 0;
    xenbus_free_evtchn(dev, ring.evtchn);
    ret = ring.irq;
    out:
    if (bytes) {
    for (i--; i >= 0; i--)
    gnttab_end_foreign_access(ring.intf.ref[i], core::ptr::null_mut());
    free_pages_exact(bytes, 1UL << (order + XEN_PAGE_SHIFT));
    ring.data.in = core::ptr::null_mut();
    ring.data.out = core::ptr::null_mut();
    }
    if (ring.ref != INVALID_GRANT_REF) {
    gnttab_end_foreign_access(ring.ref, core::ptr::null_mut());
    ring.ref = INVALID_GRANT_REF;
    }
    if (ring.intf) {
    free_page((unsigned long)ring.intf);
    ring.intf = core::ptr::null_mut();
    }
    ring.irq = -1;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn xen_9pfs_front_init(dev: *mut xenbus_device) -> c_int {
    static int xen_9pfs_front_init(struct xenbus_device *dev)
    {
    int ret, i;
    struct xenbus_transaction xbt;
    struct xen_9pfs_front_priv *priv;
    char *versions, *v, *token;
    let mut version_1: bool = false;
    unsigned int max_rings, max_ring_order, len = 0, version;
    versions = xenbus_read(XBT_NIL, dev.otherend, "versions", &len);
    if (IS_ERR(versions))
    return PTR_ERR(versions);
    for (v = versions; (token = strsep(&v, ",")); ) {
    if (!*token)
    continue;
    ret = kstrtouint(token, 10, &version);
    if (ret) {
    kfree(versions);
    return ret;
    }
    if (version == 1)
    version_1 = true;
    }
    kfree(versions);
    if (!version_1)
    return -EINVAL;
    max_rings = xenbus_read_unsigned(dev.otherend, "max-rings", 0);
    if (max_rings < XEN_9PFS_NUM_RINGS)
    return -EINVAL;
    max_ring_order = xenbus_read_unsigned(dev.otherend,
    "max-ring-page-order", 0);
    if (max_ring_order > XEN_9PFS_RING_ORDER)
    max_ring_order = XEN_9PFS_RING_ORDER;
    if (p9_xen_trans.maxsize > XEN_FLEX_RING_SIZE(max_ring_order))
    p9_xen_trans.maxsize = XEN_FLEX_RING_SIZE(max_ring_order) / 2;
    priv = kzalloc_obj(*priv);
    if (!priv)
    return -ENOMEM;
    priv.dev = dev;
    priv.rings = kzalloc_objs(*priv.rings, XEN_9PFS_NUM_RINGS);
    if (!priv.rings) {
    kfree(priv);
    return -ENOMEM;
    }
    for (i = 0; i < XEN_9PFS_NUM_RINGS; i++) {
    priv.rings[i].priv = priv;
    ret = xen_9pfs_front_alloc_dataring(dev, &priv.rings[i],
    max_ring_order);
    if (ret < 0)
    goto error;
    }
    again:
    ret = xenbus_transaction_start(&xbt);
    if (ret) {
    xenbus_dev_fatal(dev, ret, "starting transaction");
    goto error;
    }
    ret = xenbus_printf(xbt, dev.nodename, "version", "%u", 1);
    if (ret)
    goto error_xenbus;
    ret = xenbus_printf(xbt, dev.nodename, "num-rings", "%u",
    XEN_9PFS_NUM_RINGS);
    if (ret)
    goto error_xenbus;
    for (i = 0; i < XEN_9PFS_NUM_RINGS; i++) {
    char str[16];
    BUILD_BUG_ON(XEN_9PFS_NUM_RINGS > 9);
    sprintf(str, "ring-ref%d", i);
    ret = xenbus_printf(xbt, dev.nodename, str, "%d",
    priv.rings[i].ref);
    if (ret)
    goto error_xenbus;
    sprintf(str, "event-channel-%d", i);
    ret = xenbus_printf(xbt, dev.nodename, str, "%u",
    priv.rings[i].evtchn);
    if (ret)
    goto error_xenbus;
    }
    priv.tag = xenbus_read(xbt, dev.nodename, "tag", core::ptr::null_mut());
    if (IS_ERR(priv.tag)) {
    ret = PTR_ERR(priv.tag);
    goto error_xenbus;
    }
    ret = xenbus_transaction_end(xbt, 0);
    if (ret) {
    if (ret == -EAGAIN)
    goto again;
    xenbus_dev_fatal(dev, ret, "completing transaction");
    goto error;
    }
    write_lock(&xen_9pfs_lock);
    dev_set_drvdata(&dev.dev, priv);
    list_add_tail(&priv.list, &xen_9pfs_devs);
    write_unlock(&xen_9pfs_lock);
    xenbus_switch_state(dev, XenbusStateInitialised);
    return 0;
    error_xenbus:
    xenbus_transaction_end(xbt, 1);
    xenbus_dev_fatal(dev, ret, "writing xenstore");
    error:
    xen_9pfs_front_free(priv);
    return ret;
    }
    static int xen_9pfs_front_probe(struct xenbus_device *dev,
    const struct xenbus_device_id *id)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xen_9pfs_front_resume(dev: *mut xenbus_device) -> c_int {
    static int xen_9pfs_front_resume(struct xenbus_device *dev)
    {
    dev_warn(&dev.dev, "suspend/resume unsupported\n");
    return 0;
    }
    static void xen_9pfs_front_changed(struct xenbus_device *dev,
    enum xenbus_state backend_state)
    {
    switch (backend_state) {
    case XenbusStateReconfiguring:
    case XenbusStateReconfigured:
    case XenbusStateInitialising:
    case XenbusStateInitialised:
    case XenbusStateUnknown:
    break;
    case XenbusStateInitWait:
    if (dev.state != XenbusStateInitialising)
    break;
    xen_9pfs_front_init(dev);
    break;
    case XenbusStateConnected:
    xenbus_switch_state(dev, XenbusStateConnected);
    break;
    case XenbusStateClosed:
    if (dev.state == XenbusStateClosed)
    break;
    fallthrough;	/* Missed the backend's CLOSING state */
    case XenbusStateClosing:
    xenbus_frontend_closed(dev);
    break;
    }
    }
    static struct xenbus_driver xen_9pfs_front_driver = {
    .ids = xen_9pfs_front_ids,
    .probe = xen_9pfs_front_probe,
    .remove = xen_9pfs_front_remove,
    .resume = xen_9pfs_front_resume,
    .otherend_changed = xen_9pfs_front_changed,
    };
#[no_mangle]
unsafe extern "C" fn p9_trans_xen_init() -> int __init {
    static int __init p9_trans_xen_init(void)
    {
    int rc;
    if (!xen_domain())
    return -ENODEV;
    pr_info("Initialising Xen transport for 9pfs\n");
    v9fs_register_trans(&p9_xen_trans);
    rc = xenbus_register_frontend(&xen_9pfs_front_driver);
    if (rc)
    v9fs_unregister_trans(&p9_xen_trans);
    return rc;
    }
    module_init(p9_trans_xen_init);
    MODULE_ALIAS_9P("xen");
#[no_mangle]
unsafe extern "C" fn p9_trans_xen_exit() -> void __exit {
    static void __exit p9_trans_xen_exit(void)
    {
    v9fs_unregister_trans(&p9_xen_trans);
    return xenbus_unregister_driver(&xen_9pfs_front_driver);
    }
    module_exit(p9_trans_xen_exit);
    MODULE_ALIAS("xen:9pfs");
    MODULE_AUTHOR("Stefano Stabellini <stefano@aporeto.com>");
    MODULE_DESCRIPTION("Xen Transport for 9P");
    MODULE_LICENSE("GPL");
