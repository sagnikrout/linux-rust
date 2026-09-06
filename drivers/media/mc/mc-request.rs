//! Automatically rewritten from C to Rust
//! Source: drivers/media/mc/mc-request.c
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
//
// Media device request objects
//
// Copyright 2018 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
// Copyright (C) 2018 Intel Corporation
// Copyright (C) 2018 Google, Inc.
//
// Author: Hans Verkuil <hverkuil@kernel.org>
// Author: Sakari Ailus <sakari.ailus@linux.intel.com>
//

    static const char * const request_state[] = {
    [MEDIA_REQUEST_STATE_IDLE]	 = "idle",
    [MEDIA_REQUEST_STATE_VALIDATING] = "validating",
    [MEDIA_REQUEST_STATE_QUEUED]	 = "queued",
    [MEDIA_REQUEST_STATE_COMPLETE]	 = "complete",
    [MEDIA_REQUEST_STATE_CLEANING]	 = "cleaning",
    [MEDIA_REQUEST_STATE_UPDATING]	 = "updating",
    };
    static const char *
    media_request_state_str(enum media_request_state state)
    {
    BUILD_BUG_ON(ARRAY_SIZE(request_state) != NR_OF_MEDIA_REQUEST_STATE);
    if (WARN_ON(state >= ARRAY_SIZE(request_state)))
    return "invalid";
    return request_state[state];
    }
#[no_mangle]
unsafe extern "C" fn media_request_clean(req: *mut media_request) {
    static void media_request_clean(struct media_request *req)
    {
    struct media_request_object *obj, *obj_safe;
// Just a sanity check. No other code path is allowed to change this.
    WARN_ON(req.state != MEDIA_REQUEST_STATE_CLEANING);
    WARN_ON(req.updating_count);
    WARN_ON(req.access_count);
    list_for_each_entry_safe(obj, obj_safe, &req.objects, list) {
    media_request_object_unbind(obj);
    media_request_object_put(obj);
    }
    req.updating_count = 0;
    req.access_count = 0;
    WARN_ON(req.num_incomplete_objects);
    req.num_incomplete_objects = 0;
    req.manual_completion = false;
    wake_up_interruptible_all(&req.poll_wait);
    }
#[no_mangle]
unsafe extern "C" fn media_request_release(kref: *mut kref) {
    static void media_request_release(struct kref *kref)
    {
    struct media_request *req =
    container_of(kref, struct media_request, kref);
    struct media_device *mdev = req.mdev;
    dev_dbg(mdev.dev, "request: release %s\n", req.debug_str);
// No other users, no need for a spinlock
    req.state = MEDIA_REQUEST_STATE_CLEANING;
    media_request_clean(req);
    if (mdev.ops.req_free)
    mdev.ops.req_free(req);
    else
    kfree(req);
    atomic_dec(&mdev.num_requests);
    }
#[no_mangle]
pub unsafe extern "C" fn media_request_put(req: *mut media_request) {
    void media_request_put(struct media_request *req)
    {
    kref_put(&req.kref, media_request_release);
    }
    EXPORT_SYMBOL_GPL(media_request_put);
#[no_mangle]
unsafe extern "C" fn media_request_close(inode: *mut inode, filp: *mut file) -> c_int {
    static int media_request_close(struct inode *inode, struct file *filp)
    {
    struct media_request *req = filp.private_data;
    media_request_put(req);
    return 0;
    }
    static __poll_t media_request_poll(struct file *filp,
    struct poll_table_struct *wait)
    {
    struct media_request *req = filp.private_data;
    unsigned long flags;
    let mut ret: __poll_t = 0;
    if (!(poll_requested_events(wait) & EPOLLPRI))
    return 0;
    poll_wait(filp, &req.poll_wait, wait);
    spin_lock_irqsave(&req.lock, flags);
    if (req.state == MEDIA_REQUEST_STATE_COMPLETE) {
    ret = EPOLLPRI;
    goto unlock;
    }
    if (req.state != MEDIA_REQUEST_STATE_QUEUED) {
    ret = EPOLLERR;
    goto unlock;
    }
    unlock:
    spin_unlock_irqrestore(&req.lock, flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn media_request_ioctl_queue(req: *mut media_request) -> c_long {
    static long media_request_ioctl_queue(struct media_request *req)
    {
    struct media_device *mdev = req.mdev;
    enum media_request_state state;
    unsigned long flags;
    int ret;
    dev_dbg(mdev.dev, "request: queue %s\n", req.debug_str);
//
// Ensure the request that is validated will be the one that gets queued
// next by serialising the queueing process. This mutex is also used
// to serialize with canceling a vb2 queue and with setting values such
// as controls in a request.
//
    mutex_lock(&mdev.req_queue_mutex);
    media_request_get(req);
    spin_lock_irqsave(&req.lock, flags);
    if (req.state == MEDIA_REQUEST_STATE_IDLE)
    req.state = MEDIA_REQUEST_STATE_VALIDATING;
    state = req.state;
    spin_unlock_irqrestore(&req.lock, flags);
    if (state != MEDIA_REQUEST_STATE_VALIDATING) {
    dev_dbg(mdev.dev,
    "request: unable to queue %s, request in state %s\n",
    req.debug_str, media_request_state_str(state));
    media_request_put(req);
    mutex_unlock(&mdev.req_queue_mutex);
    return -EBUSY;
    }
    ret = mdev.ops.req_validate(req);
//
// If the req_validate was successful, then we mark the state as QUEUED
// and call req_queue. The reason we set the state first is that this
// allows req_queue to unbind or complete the queued objects in case
// they are immediately 'consumed'. State changes from QUEUED to another
// state can only happen if either the driver changes the state or if
// the user cancels the vb2 queue. The driver can only change the state
// after each object is queued through the req_queue op (and note that
// that op cannot fail), so setting the state to QUEUED up front is
// safe.
//
// The other reason for changing the state is if the vb2 queue is
// canceled, and that uses the req_queue_mutex which is still locked
// while req_queue is called, so that's safe as well.
//
    spin_lock_irqsave(&req.lock, flags);
    req.state = ret ? MEDIA_REQUEST_STATE_IDLE
    : MEDIA_REQUEST_STATE_QUEUED;
    spin_unlock_irqrestore(&req.lock, flags);
    if (!ret)
    mdev.ops.req_queue(req);
    mutex_unlock(&mdev.req_queue_mutex);
    if (ret) {
    dev_dbg(mdev.dev, "request: can't queue %s (%d)\n",
    req.debug_str, ret);
    media_request_put(req);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn media_request_ioctl_reinit(req: *mut media_request) -> c_long {
    static long media_request_ioctl_reinit(struct media_request *req)
    {
    struct media_device *mdev = req.mdev;
    unsigned long flags;
    mutex_lock(&mdev.req_queue_mutex);
    spin_lock_irqsave(&req.lock, flags);
    if (req.state != MEDIA_REQUEST_STATE_IDLE &&
    req.state != MEDIA_REQUEST_STATE_COMPLETE) {
    dev_dbg(mdev.dev,
    "request: %s not in idle or complete state, cannot reinit\n",
    req.debug_str);
    spin_unlock_irqrestore(&req.lock, flags);
    mutex_unlock(&mdev.req_queue_mutex);
    return -EBUSY;
    }
    if (req.access_count) {
    dev_dbg(mdev.dev,
    "request: %s is being accessed, cannot reinit\n",
    req.debug_str);
    spin_unlock_irqrestore(&req.lock, flags);
    mutex_unlock(&mdev.req_queue_mutex);
    return -EBUSY;
    }
    req.state = MEDIA_REQUEST_STATE_CLEANING;
    spin_unlock_irqrestore(&req.lock, flags);
    media_request_clean(req);
    spin_lock_irqsave(&req.lock, flags);
    req.state = MEDIA_REQUEST_STATE_IDLE;
    spin_unlock_irqrestore(&req.lock, flags);
    mutex_unlock(&mdev.req_queue_mutex);
    return 0;
    }
    static long media_request_ioctl(struct file *filp, unsigned int cmd,
    unsigned long arg)
    {
    struct media_request *req = filp.private_data;
    switch (cmd) {
    case MEDIA_REQUEST_IOC_QUEUE:
    return media_request_ioctl_queue(req);
    case MEDIA_REQUEST_IOC_REINIT:
    return media_request_ioctl_reinit(req);
    default:
    return -ENOIOCTLCMD;
    }
    }
    static const struct file_operations request_fops = {
    .owner = THIS_MODULE,
    .poll = media_request_poll,
    .unlocked_ioctl = media_request_ioctl,

    .compat_ioctl = media_request_ioctl,

    .release = media_request_close,
    };
    struct media_request *
    media_request_get_by_fd(struct media_device *mdev, int request_fd)
    {
    struct media_request *req;
    if (!mdev || !mdev.ops ||
    !mdev.ops.req_validate || !mdev.ops.req_queue)
    return ERR_PTR(-EBADR);
    CLASS(fd, f)(request_fd);
    if (fd_empty(f))
    goto err;
    if (fd_file(f).f_op != &request_fops)
    goto err;
    req = fd_file(f).private_data;
    if (req.mdev != mdev)
    goto err;
//
// Note: as long as someone has an open filehandle of the request,
// the request can never be released. The fdget() above ensures that
// even if userspace closes the request filehandle, the release()
// fop won't be called, so the media_request_get() always succeeds
// and there is no race condition where the request was released
// before media_request_get() is called.
//
    media_request_get(req);
    return req;
    err:
    dev_dbg(mdev.dev, "cannot find request_fd %d\n", request_fd);
    return ERR_PTR(-EINVAL);
    }
    EXPORT_SYMBOL_GPL(media_request_get_by_fd);
#[no_mangle]
pub unsafe extern "C" fn media_request_alloc(mdev: *mut media_device, alloc_fd: *mut c_int) -> c_int {
    int media_request_alloc(struct media_device *mdev, int *alloc_fd)
    {
    struct media_request *req;
    int ret;
// Either both are NULL or both are non-NULL
    if (WARN_ON(!mdev.ops.req_alloc ^ !mdev.ops.req_free))
    return -ENOMEM;
    if (mdev.ops.req_alloc)
    req = mdev.ops.req_alloc(mdev);
    else
    req = kzalloc_obj(*req);
    if (!req)
    return -ENOMEM;
    req.mdev = mdev;
    req.state = MEDIA_REQUEST_STATE_IDLE;
    req.num_incomplete_objects = 0;
    req.manual_completion = false;
    kref_init(&req.kref);
    INIT_LIST_HEAD(&req.objects);
    spin_lock_init(&req.lock);
    init_waitqueue_head(&req.poll_wait);
    req.updating_count = 0;
    req.access_count = 0;
    FD_PREPARE(fdf, O_CLOEXEC,
    anon_inode_getfile("request", &request_fops, core::ptr::null_mut(),
    O_CLOEXEC));
    if (fdf.err) {
    ret = fdf.err;
    goto err_free_req;
    }
    fd_prepare_file(fdf).private_data = req;
    snprintf(req.debug_str, sizeof(req.debug_str), "%u:%d",
    atomic_inc_return(&mdev.request_id), fd_prepare_fd(fdf));
    atomic_inc(&mdev.num_requests);
    dev_dbg(mdev.dev, "request: allocated %s\n", req.debug_str);
// alloc_fd = fd_publish(fdf);
    return 0;
    err_free_req:
    if (mdev.ops.req_free)
    mdev.ops.req_free(req);
    else
    kfree(req);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn media_request_object_release(kref: *mut kref) {
    static void media_request_object_release(struct kref *kref)
    {
    struct media_request_object *obj =
    container_of(kref, struct media_request_object, kref);
    struct media_request *req = obj.req;
    struct media_device *mdev = obj.mdev;
    if (WARN_ON(req))
    media_request_object_unbind(obj);
    obj.ops.release(obj);
    atomic_dec(&mdev.num_request_objects);
    }
    struct media_request_object *
    media_request_object_find(struct media_request *req,
    const struct media_request_object_ops *ops,
    void *priv)
    {
    struct media_request_object *obj;
    struct media_request_object *found = core::ptr::null_mut();
    unsigned long flags;
    if (WARN_ON(!ops || !priv))
    return core::ptr::null_mut();
    spin_lock_irqsave(&req.lock, flags);
    list_for_each_entry(obj, &req.objects, list) {
    if (obj.ops == ops && obj.priv == priv) {
    media_request_object_get(obj);
    found = obj;
    break;
    }
    }
    spin_unlock_irqrestore(&req.lock, flags);
    return found;
    }
    EXPORT_SYMBOL_GPL(media_request_object_find);
#[no_mangle]
pub unsafe extern "C" fn media_request_object_put(obj: *mut media_request_object) {
    void media_request_object_put(struct media_request_object *obj)
    {
    kref_put(&obj.kref, media_request_object_release);
    }
    EXPORT_SYMBOL_GPL(media_request_object_put);
#[no_mangle]
pub unsafe extern "C" fn media_request_object_init(obj: *mut media_request_object) {
    void media_request_object_init(struct media_request_object *obj)
    {
    obj.ops = core::ptr::null_mut();
    obj.req = core::ptr::null_mut();
    obj.priv = core::ptr::null_mut();
    obj.completed = false;
    INIT_LIST_HEAD(&obj.list);
    kref_init(&obj.kref);
    }
    EXPORT_SYMBOL_GPL(media_request_object_init);
    int media_request_object_bind(struct media_request *req,
    const struct media_request_object_ops *ops,
    void *priv, bool is_buffer,
    struct media_request_object *obj)
    {
    unsigned long flags;
    let mut ret: c_int = -EBUSY;
    if (WARN_ON(!ops.release))
    return -EBADR;
    spin_lock_irqsave(&req.lock, flags);
    if (WARN_ON(req.state != MEDIA_REQUEST_STATE_UPDATING &&
    req.state != MEDIA_REQUEST_STATE_QUEUED))
    goto unlock;
    obj.req = req;
    obj.ops = ops;
    obj.priv = priv;
    obj.mdev = req.mdev;
    if (is_buffer)
    list_add_tail(&obj.list, &req.objects);
    else
    list_add(&obj.list, &req.objects);
    req.num_incomplete_objects++;
    ret = 0;
    atomic_inc(&obj.mdev.num_request_objects);
    unlock:
    spin_unlock_irqrestore(&req.lock, flags);
    return ret;
    }
    EXPORT_SYMBOL_GPL(media_request_object_bind);
#[no_mangle]
pub unsafe extern "C" fn media_request_object_unbind(obj: *mut media_request_object) {
    void media_request_object_unbind(struct media_request_object *obj)
    {
    struct media_request *req = obj.req;
    unsigned long flags;
    let mut completed: bool = false;
    if (WARN_ON(!req))
    return;
    spin_lock_irqsave(&req.lock, flags);
    list_del(&obj.list);
    obj.req = core::ptr::null_mut();
    if (req.state == MEDIA_REQUEST_STATE_COMPLETE)
    goto unlock;
    if (WARN_ON(req.state == MEDIA_REQUEST_STATE_VALIDATING))
    goto unlock;
    if (req.state == MEDIA_REQUEST_STATE_CLEANING) {
    if (!obj.completed)
    req.num_incomplete_objects--;
    goto unlock;
    }
    if (WARN_ON(!req.num_incomplete_objects))
    goto unlock;
    req.num_incomplete_objects--;
    if (req.state == MEDIA_REQUEST_STATE_QUEUED &&
    !req.num_incomplete_objects && !req.manual_completion) {
    req.state = MEDIA_REQUEST_STATE_COMPLETE;
    completed = true;
    wake_up_interruptible_all(&req.poll_wait);
    }
    unlock:
    spin_unlock_irqrestore(&req.lock, flags);
    if (obj.ops.unbind)
    obj.ops.unbind(obj);
    if (completed)
    media_request_put(req);
    }
    EXPORT_SYMBOL_GPL(media_request_object_unbind);
#[no_mangle]
pub unsafe extern "C" fn media_request_object_complete(obj: *mut media_request_object) {
    void media_request_object_complete(struct media_request_object *obj)
    {
    struct media_request *req = obj.req;
    unsigned long flags;
    let mut completed: bool = false;
    spin_lock_irqsave(&req.lock, flags);
    if (obj.completed)
    goto unlock;
    obj.completed = true;
    if (WARN_ON(!req.num_incomplete_objects) ||
    WARN_ON(req.state != MEDIA_REQUEST_STATE_QUEUED))
    goto unlock;
    if (!--req.num_incomplete_objects && !req.manual_completion) {
    req.state = MEDIA_REQUEST_STATE_COMPLETE;
    wake_up_interruptible_all(&req.poll_wait);
    completed = true;
    }
    unlock:
    spin_unlock_irqrestore(&req.lock, flags);
    if (completed)
    media_request_put(req);
    }
    EXPORT_SYMBOL_GPL(media_request_object_complete);
#[no_mangle]
pub unsafe extern "C" fn media_request_manual_complete(req: *mut media_request) {
    void media_request_manual_complete(struct media_request *req)
    {
    let mut completed: bool = false;
    unsigned long flags;
    if (WARN_ON_ONCE(!req))
    return;
    spin_lock_irqsave(&req.lock, flags);
    if (WARN_ON_ONCE(!req.manual_completion))
    goto unlock;
    if (WARN_ON_ONCE(req.state != MEDIA_REQUEST_STATE_QUEUED))
    goto unlock;
    req.manual_completion = false;
//
// It is expected that all other objects in this request are
// completed when this function is called. WARN if that is
// not the case.
//
    if (!WARN_ON(req.num_incomplete_objects)) {
    req.state = MEDIA_REQUEST_STATE_COMPLETE;
    wake_up_interruptible_all(&req.poll_wait);
    completed = true;
    }
    unlock:
    spin_unlock_irqrestore(&req.lock, flags);
    if (completed)
    media_request_put(req);
    }
    EXPORT_SYMBOL_GPL(media_request_manual_complete);
