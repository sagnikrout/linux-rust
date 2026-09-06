//! Automatically rewritten from C to Rust
//! Source: drivers/platform/raspberrypi/vchiq-interface/vchiq_dev.c
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (c) 2014 Raspberry Pi (Trading) Ltd. All rights reserved.
// Copyright (c) 2010-2012 Broadcom. All rights reserved.
//

    static const char *const ioctl_names[] = {
    "CONNECT",
    "SHUTDOWN",
    "CREATE_SERVICE",
    "REMOVE_SERVICE",
    "QUEUE_MESSAGE",
    "QUEUE_BULK_TRANSMIT",
    "QUEUE_BULK_RECEIVE",
    "AWAIT_COMPLETION",
    "DEQUEUE_MESSAGE",
    "GET_CLIENT_ID",
    "GET_CONFIG",
    "CLOSE_SERVICE",
    "USE_SERVICE",
    "RELEASE_SERVICE",
    "SET_SERVICE_OPTION",
    "DUMP_PHYS_MEM",
    "LIB_VERSION",
    "CLOSE_DELIVERED"
    };
    static_assert(ARRAY_SIZE(ioctl_names) == (VCHIQ_IOC_MAX + 1));
    static void
    user_service_free(void *userdata)
    {
    kfree(userdata);
    }
#[no_mangle]
unsafe extern "C" fn close_delivered(user_service: *mut user_service) {
    static void close_delivered(struct user_service *user_service)
    {
    dev_dbg(user_service.service.state.dev,
    "arm: (handle=%x)\n", user_service.service.handle);
    if (user_service.close_pending) {
// Allow the underlying service to be culled
    vchiq_service_put(user_service.service);
// Wake the user-thread blocked in close_ or remove_service
    complete(&user_service.close_event);
    user_service.close_pending = 0;
    }
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_io_copy_callback_context {
    pub element: *mut vchiq_element,
    pub element_offset: usize,
    pub elements_to_go: c_ulong,
}

    static ssize_t vchiq_ioc_copy_element_data(void *context, void *dest,
    size_t offset, size_t maxsize)
    {
    struct vchiq_io_copy_callback_context *cc = context;
    let mut total_bytes_copied: usize = 0;
    size_t bytes_this_round;
    while (total_bytes_copied < maxsize) {
    if (!cc.elements_to_go)
    return total_bytes_copied;
    if (!cc.element.size) {
    cc.elements_to_go--;
    cc.element++;
    cc.element_offset = 0;
    continue;
    }
    bytes_this_round = min(cc.element.size - cc.element_offset,
    maxsize - total_bytes_copied);
    if (copy_from_user(dest + total_bytes_copied,
    cc.element.data + cc.element_offset,
    bytes_this_round))
    return -EFAULT;
    cc.element_offset += bytes_this_round;
    total_bytes_copied += bytes_this_round;
    if (cc.element_offset == cc.element.size) {
    cc.elements_to_go--;
    cc.element++;
    cc.element_offset = 0;
    }
    }
    return maxsize;
    }
    static int
    vchiq_ioc_queue_message(struct vchiq_instance *instance, unsigned int handle,
    struct vchiq_element *elements, unsigned long count)
    {
    struct vchiq_io_copy_callback_context context;
    let mut status: c_int = 0;
    unsigned long i;
    let mut total_size: usize = 0;
    context.element = elements;
    context.element_offset = 0;
    context.elements_to_go = count;
    for (i = 0; i < count; i++) {
    if (!elements[i].data && elements[i].size != 0)
    return -EFAULT;
    total_size += elements[i].size;
    }
    status = vchiq_queue_message(instance, handle, vchiq_ioc_copy_element_data,
    &context, total_size);
    if (status == -EINVAL)
    return -EIO;
#[no_mangle]
pub unsafe extern "C" fn if(-EAGAIN: status ==) -> else {
    else if (status == -EAGAIN)
    return -EINTR;
    return 0;
    }
    static int vchiq_ioc_create_service(struct vchiq_instance *instance,
    struct vchiq_create_service *args)
    {
    struct user_service *user_service = core::ptr::null_mut();
    struct vchiq_service *service;
    let mut status: c_int = 0;
    struct vchiq_service_params_kernel params;
    int srvstate;
    if (args.is_open && !instance.connected)
    return -ENOTCONN;
    user_service = kmalloc_obj(*user_service);
    if (!user_service)
    return -ENOMEM;
    if (args.is_open) {
    srvstate = VCHIQ_SRVSTATE_OPENING;
    } else {
    srvstate = instance.connected ?
    VCHIQ_SRVSTATE_LISTENING : VCHIQ_SRVSTATE_HIDDEN;
    }
    params = (struct vchiq_service_params_kernel) {
    .fourcc   = args.params.fourcc,
    .callback = service_callback,
    .userdata = user_service,
    .version  = args.params.version,
    .version_min = args.params.version_min,
    };
    service = vchiq_add_service_internal(instance.state, &params,
    srvstate, instance,
    user_service_free);
    if (!service) {
    kfree(user_service);
    return -EEXIST;
    }
    user_service.service = service;
    user_service.userdata = args.params.userdata;
    user_service.instance = instance;
    user_service.is_vchi = (args.is_vchi != 0);
    user_service.dequeue_pending = 0;
    user_service.close_pending = 0;
    user_service.message_available_pos = instance.completion_remove - 1;
    user_service.msg_insert = 0;
    user_service.msg_remove = 0;
    init_completion(&user_service.insert_event);
    init_completion(&user_service.remove_event);
    init_completion(&user_service.close_event);
    if (args.is_open) {
    status = vchiq_open_service_internal(service, instance.pid);
    if (status) {
    vchiq_remove_service(instance, service.handle);
    return (status == -EAGAIN) ?
    -EINTR : -EIO;
    }
    }
    args.handle = service.handle;
    return 0;
    }
    static int vchiq_ioc_dequeue_message(struct vchiq_instance *instance,
    struct vchiq_dequeue_message *args)
    {
    struct user_service *user_service;
    struct vchiq_service *service;
    struct vchiq_header *header;
    int ret;
    DEBUG_INITIALISE(instance.state.local);
    DEBUG_TRACE(DEQUEUE_MESSAGE_LINE);
    service = find_service_for_instance(instance, args.handle);
    if (!service)
    return -EINVAL;
    user_service = (struct user_service *)service.base.userdata;
    if (user_service.is_vchi == 0) {
    ret = -EINVAL;
    goto out;
    }
    spin_lock(&service.state.msg_queue_spinlock);
    if (user_service.msg_remove == user_service.msg_insert) {
    if (!args.blocking) {
    spin_unlock(&service.state.msg_queue_spinlock);
    DEBUG_TRACE(DEQUEUE_MESSAGE_LINE);
    ret = -EWOULDBLOCK;
    goto out;
    }
    user_service.dequeue_pending = 1;
    ret = 0;
    do {
    spin_unlock(&service.state.msg_queue_spinlock);
    DEBUG_TRACE(DEQUEUE_MESSAGE_LINE);
    if (wait_for_completion_interruptible(&user_service.insert_event)) {
    dev_dbg(service.state.dev, "arm: DEQUEUE_MESSAGE interrupted\n");
    ret = -EINTR;
    break;
    }
    spin_lock(&service.state.msg_queue_spinlock);
    } while (user_service.msg_remove == user_service.msg_insert);
    if (ret)
    goto out;
    }
    if (WARN_ON_ONCE((int)(user_service.msg_insert -
    user_service.msg_remove) < 0)) {
    spin_unlock(&service.state.msg_queue_spinlock);
    ret = -EINVAL;
    goto out;
    }
    header = user_service.msg_queue[user_service.msg_remove &
    (MSG_QUEUE_SIZE - 1)];
    user_service.msg_remove++;
    spin_unlock(&service.state.msg_queue_spinlock);
    complete(&user_service.remove_event);
    if (!header) {
    ret = -ENOTCONN;
    } else if (header.size <= args.bufsize) {
// Copy to user space if msgbuf is not NULL
    if (!args.buf || (copy_to_user(args.buf, header.data, header.size) == 0)) {
    ret = header.size;
    vchiq_release_message(instance, service.handle, header);
    } else {
    ret = -EFAULT;
    }
    } else {
    dev_err(service.state.dev,
    "arm: header %p: bufsize %x < size %x\n",
    header, args.bufsize, header.size);
    WARN(1, "invalid size\n");
    ret = -EMSGSIZE;
    }
    DEBUG_TRACE(DEQUEUE_MESSAGE_LINE);
    out:
    vchiq_service_put(service);
    return ret;
    }
    static int vchiq_irq_queue_bulk_tx_rx(struct vchiq_instance *instance,
    struct vchiq_queue_bulk_transfer *args,
    enum vchiq_bulk_dir dir,
    enum vchiq_bulk_mode __user *mode)
    {
    struct vchiq_service *service;
    struct bulk_waiter_node *waiter = core::ptr::null_mut(), *iter;
    let mut bulk_params: vchiq_bulk = {};
    let mut status: c_int = 0;
    int ret;
    service = find_service_for_instance(instance, args.handle);
    if (!service)
    return -EINVAL;
    if (args.mode == VCHIQ_BULK_MODE_BLOCKING) {
    waiter = kzalloc_obj(*waiter);
    if (!waiter) {
    ret = -ENOMEM;
    goto out;
    }
    bulk_params.uoffset = args.data;
    bulk_params.mode = args.mode;
    bulk_params.size = args.size;
    bulk_params.dir = dir;
    bulk_params.waiter = &waiter.bulk_waiter;
    status = vchiq_bulk_xfer_blocking(instance, args.handle,
    &bulk_params);
    } else if (args.mode == VCHIQ_BULK_MODE_WAITING) {
    mutex_lock(&instance.bulk_waiter_list_mutex);
    list_for_each_entry(iter, &instance.bulk_waiter_list,
    list) {
    if (iter.pid == current.pid) {
    list_del(&iter.list);
    waiter = iter;
    break;
    }
    }
    mutex_unlock(&instance.bulk_waiter_list_mutex);
    if (!waiter) {
    dev_err(service.state.dev,
    "arm: no bulk_waiter found for pid %d\n", current.pid);
    ret = -ESRCH;
    goto out;
    }
    dev_dbg(service.state.dev, "arm: found bulk_waiter %p for pid %d\n",
    waiter, current.pid);
    status = vchiq_bulk_xfer_waiting(instance, args.handle,
    &waiter.bulk_waiter);
    } else {
    bulk_params.uoffset = args.data;
    bulk_params.mode = args.mode;
    bulk_params.size = args.size;
    bulk_params.dir = dir;
    bulk_params.cb_userdata = args.userdata;
    status = vchiq_bulk_xfer_callback(instance, args.handle,
    &bulk_params);
    }
    if (!waiter) {
    ret = 0;
    goto out;
    }
    if ((status != -EAGAIN) || fatal_signal_pending(current) ||
    !waiter.bulk_waiter.bulk) {
    if (waiter.bulk_waiter.bulk) {
// Cancel the signal when the transfer completes.
    spin_lock(&service.state.bulk_waiter_spinlock);
    waiter.bulk_waiter.bulk.waiter = core::ptr::null_mut();
    spin_unlock(&service.state.bulk_waiter_spinlock);
    }
    kfree(waiter);
    ret = 0;
    } else {
    const enum vchiq_bulk_mode mode_waiting =
    VCHIQ_BULK_MODE_WAITING;
    waiter.pid = current.pid;
    mutex_lock(&instance.bulk_waiter_list_mutex);
    list_add(&waiter.list, &instance.bulk_waiter_list);
    mutex_unlock(&instance.bulk_waiter_list_mutex);
    dev_dbg(service.state.dev, "arm: saved bulk_waiter %p for pid %d\n",
    waiter, current.pid);
    ret = put_user(mode_waiting, mode);
    }
    out:
    vchiq_service_put(service);
    if (ret)
    return ret;
#[no_mangle]
pub unsafe extern "C" fn if(-EINVAL: status ==) -> else {
    else if (status == -EINVAL)
    return -EIO;
#[no_mangle]
pub unsafe extern "C" fn if(-EAGAIN: status ==) -> else {
    else if (status == -EAGAIN)
    return -EINTR;
    return 0;
    }
// read a user pointer value from an array pointers in user space
#[no_mangle]
pub unsafe extern "C" fn vchiq_get_user_ptr(buf: *mut void __user, ubuf: *mut void __user, index: c_int) -> c_int {
    static inline int vchiq_get_user_ptr(void __user **buf, void __user *ubuf, int index)
    {
    int ret;
    if (in_compat_syscall()) {
    compat_uptr_t ptr32;
    compat_uptr_t __user *uptr = ubuf;
    ret = get_user(ptr32, uptr + index);
    if (ret)
    return ret;
// buf = compat_ptr(ptr32);
    } else {
    uintptr_t ptr, __user *uptr = ubuf;
    ret = get_user(ptr, uptr + index);
    if (ret)
    return ret;
// buf = (void __user *)ptr;
    }
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_completion_data32 {
    pub reason: enum vchiq_reason,
    pub header: compat_uptr_t,
    pub service_userdata: compat_uptr_t,
    pub cb_data: compat_uptr_t,
}

    static int vchiq_put_completion(struct vchiq_completion_data __user *buf,
    struct vchiq_completion_data *completion,
    int index)
    {
    struct vchiq_completion_data32 __user *buf32 = (void __user *)buf;
    if (in_compat_syscall()) {
    struct vchiq_completion_data32 tmp = {
    .reason		  = completion.reason,
    .header		  = ptr_to_compat(completion.header),
    .service_userdata = ptr_to_compat(completion.service_userdata),
    .cb_data	  = ptr_to_compat(completion.cb_userdata),
    };
    if (copy_to_user(&buf32[index], &tmp, sizeof(tmp)))
    return -EFAULT;
    } else {
    if (copy_to_user(&buf[index], completion, sizeof(*completion)))
    return -EFAULT;
    }
    return 0;
    }
    static int vchiq_ioc_await_completion(struct vchiq_instance *instance,
    struct vchiq_await_completion *args,
    int __user *msgbufcountp)
    {
    int msgbufcount;
    int remove;
    int ret;
    DEBUG_INITIALISE(instance.state.local);
    DEBUG_TRACE(AWAIT_COMPLETION_LINE);
    if (!instance.connected)
    return -ENOTCONN;
    mutex_lock(&instance.completion_mutex);
    DEBUG_TRACE(AWAIT_COMPLETION_LINE);
    while ((instance.completion_remove == instance.completion_insert) && !instance.closing) {
    int rc;
    DEBUG_TRACE(AWAIT_COMPLETION_LINE);
    mutex_unlock(&instance.completion_mutex);
    rc = wait_for_completion_interruptible(&instance.insert_event);
    mutex_lock(&instance.completion_mutex);
    if (rc) {
    DEBUG_TRACE(AWAIT_COMPLETION_LINE);
    dev_dbg(instance.state.dev, "arm: AWAIT_COMPLETION interrupted\n");
    ret = -EINTR;
    goto out;
    }
    }
    DEBUG_TRACE(AWAIT_COMPLETION_LINE);
    msgbufcount = args.msgbufcount;
    remove = instance.completion_remove;
    for (ret = 0; ret < args.count; ret++) {
    struct vchiq_completion_data_kernel *completion;
    struct vchiq_completion_data user_completion;
    struct vchiq_service *service;
    struct user_service *user_service;
    struct vchiq_header *header;
    if (remove == instance.completion_insert)
    break;
    completion = &instance.completions[remove & (MAX_COMPLETIONS - 1)];
//
// A read memory barrier is needed to stop
// prefetch of a stale completion record
//
    rmb();
    service = completion.service_userdata;
    user_service = service.base.userdata;
    memset(&user_completion, 0, sizeof(user_completion));
    user_completion = (struct vchiq_completion_data) {
    .reason = completion.reason,
    .service_userdata = user_service.userdata,
    };
    header = completion.header;
    if (header) {
    void __user *msgbuf;
    int msglen;
    msglen = header.size + sizeof(struct vchiq_header);
// This must be a VCHIQ-style service
    if (args.msgbufsize < msglen) {
    dev_err(service.state.dev,
    "arm: header %p: msgbufsize %x < msglen %x\n",
    header, args.msgbufsize, msglen);
    WARN(1, "invalid message size\n");
    if (ret == 0)
    ret = -EMSGSIZE;
    break;
    }
    if (msgbufcount <= 0)
// Stall here for lack of a buffer for the message.
    break;
// Get the pointer from user space
    msgbufcount--;
    if (vchiq_get_user_ptr(&msgbuf, args.msgbufs,
    msgbufcount)) {
    if (ret == 0)
    ret = -EFAULT;
    break;
    }
// Copy the message to user space
    if (copy_to_user(msgbuf, header, msglen)) {
    if (ret == 0)
    ret = -EFAULT;
    break;
    }
// Now it has been copied, the message can be released.
    vchiq_release_message(instance, service.handle, header);
// The completion must point to the msgbuf.
    user_completion.header = msgbuf;
    }
    if ((completion.reason == VCHIQ_SERVICE_CLOSED) &&
    !instance.use_close_delivered)
    vchiq_service_put(service);
    user_completion.cb_userdata = completion.cb_userdata;
    if (vchiq_put_completion(args.buf, &user_completion, ret)) {
    if (ret == 0)
    ret = -EFAULT;
    break;
    }
//
// Ensure that the above copy has completed
// before advancing the remove pointer.
//
    mb();
    remove++;
    instance.completion_remove = remove;
    }
    if (msgbufcount != args.msgbufcount) {
    if (put_user(msgbufcount, msgbufcountp))
    ret = -EFAULT;
    }
    out:
    if (ret)
    complete(&instance.remove_event);
    mutex_unlock(&instance.completion_mutex);
    DEBUG_TRACE(AWAIT_COMPLETION_LINE);
    return ret;
    }
    static long
    vchiq_ioctl(struct file *file, unsigned int cmd, unsigned long arg)
    {
    struct vchiq_instance *instance = file.private_data;
    let mut status: c_int = 0;
    struct vchiq_service *service = core::ptr::null_mut();
    let mut ret: c_long = 0;
    int i, rc;
    dev_dbg(instance.state.dev, "arm: instance %p, cmd %s, arg %lx\n", instance,
    ((_IOC_TYPE(cmd) == VCHIQ_IOC_MAGIC) && (_IOC_NR(cmd) <= VCHIQ_IOC_MAX)) ?
    ioctl_names[_IOC_NR(cmd)] : "<invalid>", arg);
    switch (cmd) {
    case VCHIQ_IOC_SHUTDOWN:
    if (!instance.connected)
    break;
// Remove all services
    i = 0;
    while ((service = next_service_by_instance(instance.state,
    instance, &i))) {
    status = vchiq_remove_service(instance, service.handle);
    vchiq_service_put(service);
    if (status)
    break;
    }
    service = core::ptr::null_mut();
    if (!status) {
// Wake the completion thread and ask it to exit
    instance.closing = 1;
    complete(&instance.insert_event);
    }
    break;
    case VCHIQ_IOC_CONNECT:
    if (instance.connected) {
    ret = -EINVAL;
    break;
    }
    rc = mutex_lock_killable(&instance.state.mutex);
    if (rc) {
    dev_err(instance.state.dev,
    "arm: vchiq: connect: could not lock mutex for state %d: %d\n",
    instance.state.id, rc);
    ret = -EINTR;
    break;
    }
    status = vchiq_connect_internal(instance.state, instance);
    mutex_unlock(&instance.state.mutex);
    if (!status)
    instance.connected = 1;
    else
    dev_err(instance.state.dev,
    "arm: vchiq: could not connect: %d\n", status);
    break;
    case VCHIQ_IOC_CREATE_SERVICE: {
    struct vchiq_create_service __user *argp;
    struct vchiq_create_service args;
    argp = (void __user *)arg;
    if (copy_from_user(&args, argp, sizeof(args))) {
    ret = -EFAULT;
    break;
    }
    ret = vchiq_ioc_create_service(instance, &args);
    if (ret < 0)
    break;
    if (put_user(args.handle, &argp.handle)) {
    vchiq_remove_service(instance, args.handle);
    ret = -EFAULT;
    }
    } break;
    case VCHIQ_IOC_CLOSE_SERVICE:
    case VCHIQ_IOC_REMOVE_SERVICE: {
    let mut handle: c_uint = (unsigned int)arg;
    struct user_service *user_service;
    service = find_service_for_instance(instance, handle);
    if (!service) {
    ret = -EINVAL;
    break;
    }
    user_service = service.base.userdata;
//
// close_pending is false on first entry, and when the
// wait in vchiq_close_service has been interrupted.
//
    if (!user_service.close_pending) {
    status = (cmd == VCHIQ_IOC_CLOSE_SERVICE) ?
    vchiq_close_service(instance, service.handle) :
    vchiq_remove_service(instance, service.handle);
    if (status)
    break;
    }
//
// close_pending is true once the underlying service
// has been closed until the client library calls the
// CLOSE_DELIVERED ioctl, signalling close_event.
//
    if (user_service.close_pending &&
    wait_for_completion_interruptible(&user_service.close_event))
    status = -EAGAIN;
    break;
    }
    case VCHIQ_IOC_USE_SERVICE:
    case VCHIQ_IOC_RELEASE_SERVICE:	{
    let mut handle: c_uint = (unsigned int)arg;
    service = find_service_for_instance(instance, handle);
    if (service) {
    ret = (cmd == VCHIQ_IOC_USE_SERVICE) ?
    vchiq_use_service_internal(service) :
    vchiq_release_service_internal(service);
    if (ret) {
    dev_err(instance.state.dev,
    "suspend: cmd %s returned error %ld for service %p4cc:%03d\n",
    (cmd == VCHIQ_IOC_USE_SERVICE) ?
    "VCHIQ_IOC_USE_SERVICE" :
    "VCHIQ_IOC_RELEASE_SERVICE",
    ret, &service.base.fourcc,
    service.client_id);
    }
    } else {
    ret = -EINVAL;
    }
    } break;
    case VCHIQ_IOC_QUEUE_MESSAGE: {
    struct vchiq_queue_message args;
    if (copy_from_user(&args, (const void __user *)arg,
    sizeof(args))) {
    ret = -EFAULT;
    break;
    }
    service = find_service_for_instance(instance, args.handle);
    if (service && (args.count <= MAX_ELEMENTS)) {
// Copy elements into kernel space
    struct vchiq_element elements[MAX_ELEMENTS];
    if (copy_from_user(elements, args.elements,
    args.count * sizeof(struct vchiq_element)) == 0)
    ret = vchiq_ioc_queue_message(instance, args.handle, elements,
    args.count);
    else
    ret = -EFAULT;
    } else {
    ret = -EINVAL;
    }
    } break;
    case VCHIQ_IOC_QUEUE_BULK_TRANSMIT:
    case VCHIQ_IOC_QUEUE_BULK_RECEIVE: {
    struct vchiq_queue_bulk_transfer args;
    struct vchiq_queue_bulk_transfer __user *argp;
    enum vchiq_bulk_dir dir =
    (cmd == VCHIQ_IOC_QUEUE_BULK_TRANSMIT) ?
    VCHIQ_BULK_TRANSMIT : VCHIQ_BULK_RECEIVE;
    argp = (void __user *)arg;
    if (copy_from_user(&args, argp, sizeof(args))) {
    ret = -EFAULT;
    break;
    }
    ret = vchiq_irq_queue_bulk_tx_rx(instance, &args,
    dir, &argp.mode);
    } break;
    case VCHIQ_IOC_AWAIT_COMPLETION: {
    struct vchiq_await_completion args;
    struct vchiq_await_completion __user *argp;
    argp = (void __user *)arg;
    if (copy_from_user(&args, argp, sizeof(args))) {
    ret = -EFAULT;
    break;
    }
    ret = vchiq_ioc_await_completion(instance, &args,
    &argp.msgbufcount);
    } break;
    case VCHIQ_IOC_DEQUEUE_MESSAGE: {
    struct vchiq_dequeue_message args;
    if (copy_from_user(&args, (const void __user *)arg,
    sizeof(args))) {
    ret = -EFAULT;
    break;
    }
    ret = vchiq_ioc_dequeue_message(instance, &args);
    } break;
    case VCHIQ_IOC_GET_CLIENT_ID: {
    let mut handle: c_uint = (unsigned int)arg;
    ret = vchiq_get_client_id(instance, handle);
    } break;
    case VCHIQ_IOC_GET_CONFIG: {
    struct vchiq_get_config args;
    struct vchiq_config config;
    if (copy_from_user(&args, (const void __user *)arg,
    sizeof(args))) {
    ret = -EFAULT;
    break;
    }
    if (args.config_size > sizeof(config)) {
    ret = -EINVAL;
    break;
    }
    vchiq_get_config(&config);
    if (copy_to_user(args.pconfig, &config, args.config_size)) {
    ret = -EFAULT;
    break;
    }
    } break;
    case VCHIQ_IOC_SET_SERVICE_OPTION: {
    struct vchiq_set_service_option args;
    if (copy_from_user(&args, (const void __user *)arg,
    sizeof(args))) {
    ret = -EFAULT;
    break;
    }
    service = find_service_for_instance(instance, args.handle);
    if (!service) {
    ret = -EINVAL;
    break;
    }
    ret = vchiq_set_service_option(instance, args.handle, args.option,
    args.value);
    } break;
    case VCHIQ_IOC_LIB_VERSION: {
    let mut lib_version: c_uint = (unsigned int)arg;
    if (lib_version < VCHIQ_VERSION_MIN)
    ret = -EINVAL;
#[no_mangle]
pub unsafe extern "C" fn if(VCHIQ_VERSION_CLOSE_DELIVERED: lib_version >=) -> else {
    else if (lib_version >= VCHIQ_VERSION_CLOSE_DELIVERED)
    instance.use_close_delivered = 1;
    } break;
    case VCHIQ_IOC_CLOSE_DELIVERED: {
    let mut handle: c_uint = (unsigned int)arg;
    service = find_closed_service_for_instance(instance, handle);
    if (service) {
    struct user_service *user_service =
    (struct user_service *)service.base.userdata;
    close_delivered(user_service);
    } else {
    ret = -EINVAL;
    }
    } break;
    default:
    ret = -ENOTTY;
    break;
    }
    if (service)
    vchiq_service_put(service);
    if (ret == 0) {
    if (status == -EINVAL)
    ret = -EIO;
#[no_mangle]
pub unsafe extern "C" fn if(-EAGAIN: status ==) -> else {
    else if (status == -EAGAIN)
    ret = -EINTR;
    }
    if (!status && (ret < 0) && (ret != -EINTR) && (ret != -EWOULDBLOCK)) {
    dev_dbg(instance.state.dev,
    "arm: ioctl instance %p, cmd %s . status %d, %ld\n",
    instance, (_IOC_NR(cmd) <= VCHIQ_IOC_MAX) ?
    ioctl_names[_IOC_NR(cmd)] : "<invalid>", status, ret);
    } else {
    dev_dbg(instance.state.dev,
    "arm: ioctl instance %p, cmd %s . status %d\n, %ld\n",
    instance, (_IOC_NR(cmd) <= VCHIQ_IOC_MAX) ?
    ioctl_names[_IOC_NR(cmd)] : "<invalid>", status, ret);
    }
    return ret;
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_service_params32 {
    pub fourcc: c_int,
    pub callback: compat_uptr_t,
    pub userdata: compat_uptr_t,
    pub /: *mut *mut short version; / Increment for non-trivial changes,
    pub /: *mut *mut short version_min; / Update for incompatible changes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_create_service32 {
    pub params: vchiq_service_params32,
    pub is_open: c_int,
    pub is_vchi: c_int,
    pub /: *mut *mut unsigned int handle; / OUT,
}

    _IOWR(VCHIQ_IOC_MAGIC, 2, struct vchiq_create_service32)
    static long
    vchiq_compat_ioctl_create_service(struct file *file, unsigned int cmd,
    struct vchiq_create_service32 __user *ptrargs32)
    {
    struct vchiq_create_service args;
    struct vchiq_create_service32 args32;
    struct vchiq_instance *instance = file.private_data;
    long ret;
    if (copy_from_user(&args32, ptrargs32, sizeof(args32)))
    return -EFAULT;
    args = (struct vchiq_create_service) {
    .params = {
    .fourcc	     = args32.params.fourcc,
    .callback    = compat_ptr(args32.params.callback),
    .userdata    = compat_ptr(args32.params.userdata),
    .version     = args32.params.version,
    .version_min = args32.params.version_min,
    },
    .is_open = args32.is_open,
    .is_vchi = args32.is_vchi,
    .handle  = args32.handle,
    };
    ret = vchiq_ioc_create_service(instance, &args);
    if (ret < 0)
    return ret;
    if (put_user(args.handle, &ptrargs32.handle)) {
    vchiq_remove_service(instance, args.handle);
    return -EFAULT;
    }
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_element32 {
    pub data: compat_uptr_t,
    pub size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_queue_message32 {
    pub handle: c_uint,
    pub count: c_uint,
    pub elements: compat_uptr_t,
}

    _IOW(VCHIQ_IOC_MAGIC,  4, struct vchiq_queue_message32)
    static long
    vchiq_compat_ioctl_queue_message(struct file *file,
    unsigned int cmd,
    struct vchiq_queue_message32 __user *arg)
    {
    struct vchiq_queue_message args;
    struct vchiq_queue_message32 args32;
    struct vchiq_service *service;
    struct vchiq_instance *instance = file.private_data;
    int ret;
    if (copy_from_user(&args32, arg, sizeof(args32)))
    return -EFAULT;
    args = (struct vchiq_queue_message) {
    .handle   = args32.handle,
    .count    = args32.count,
    .elements = compat_ptr(args32.elements),
    };
    if (args32.count > MAX_ELEMENTS)
    return -EINVAL;
    service = find_service_for_instance(instance, args.handle);
    if (!service)
    return -EINVAL;
    if (args32.elements && args32.count) {
    struct vchiq_element32 element32[MAX_ELEMENTS];
    struct vchiq_element elements[MAX_ELEMENTS];
    unsigned int count;
    if (copy_from_user(&element32, args.elements,
    sizeof(element32))) {
    vchiq_service_put(service);
    return -EFAULT;
    }
    for (count = 0; count < args32.count; count++) {
    elements[count].data =
    compat_ptr(element32[count].data);
    elements[count].size = element32[count].size;
    }
    ret = vchiq_ioc_queue_message(instance, args.handle, elements,
    args.count);
    } else {
    ret = -EINVAL;
    }
    vchiq_service_put(service);
    return ret;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_queue_bulk_transfer32 {
    pub handle: c_uint,
    pub data: compat_uptr_t,
    pub size: c_uint,
    pub userdata: compat_uptr_t,
    pub mode: enum vchiq_bulk_mode,
}

    _IOWR(VCHIQ_IOC_MAGIC, 5, struct vchiq_queue_bulk_transfer32)

    _IOWR(VCHIQ_IOC_MAGIC, 6, struct vchiq_queue_bulk_transfer32)
    static long
    vchiq_compat_ioctl_queue_bulk(struct file *file,
    unsigned int cmd,
    struct vchiq_queue_bulk_transfer32 __user *argp)
    {
    struct vchiq_queue_bulk_transfer32 args32;
    struct vchiq_queue_bulk_transfer args;
    enum vchiq_bulk_dir dir = (cmd == VCHIQ_IOC_QUEUE_BULK_TRANSMIT32) ?
    VCHIQ_BULK_TRANSMIT : VCHIQ_BULK_RECEIVE;
    if (copy_from_user(&args32, argp, sizeof(args32)))
    return -EFAULT;
    args = (struct vchiq_queue_bulk_transfer) {
    .handle   = args32.handle,
    .data	  = compat_ptr(args32.data),
    .size	  = args32.size,
    .userdata = compat_ptr(args32.userdata),
    .mode	  = args32.mode,
    };
    return vchiq_irq_queue_bulk_tx_rx(file.private_data, &args,
    dir, &argp.mode);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_await_completion32 {
    pub count: c_uint,
    pub buf: compat_uptr_t,
    pub msgbufsize: c_uint,
    pub /: *mut *mut unsigned int msgbufcount; / IN/OUT,
    pub msgbufs: compat_uptr_t,
}

    _IOWR(VCHIQ_IOC_MAGIC, 7, struct vchiq_await_completion32)
    static long
    vchiq_compat_ioctl_await_completion(struct file *file,
    unsigned int cmd,
    struct vchiq_await_completion32 __user *argp)
    {
    struct vchiq_await_completion args;
    struct vchiq_await_completion32 args32;
    if (copy_from_user(&args32, argp, sizeof(args32)))
    return -EFAULT;
    args = (struct vchiq_await_completion) {
    .count		= args32.count,
    .buf		= compat_ptr(args32.buf),
    .msgbufsize	= args32.msgbufsize,
    .msgbufcount	= args32.msgbufcount,
    .msgbufs	= compat_ptr(args32.msgbufs),
    };
    return vchiq_ioc_await_completion(file.private_data, &args,
    &argp.msgbufcount);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_dequeue_message32 {
    pub handle: c_uint,
    pub blocking: c_int,
    pub bufsize: c_uint,
    pub buf: compat_uptr_t,
}

    _IOWR(VCHIQ_IOC_MAGIC, 8, struct vchiq_dequeue_message32)
    static long
    vchiq_compat_ioctl_dequeue_message(struct file *file,
    unsigned int cmd,
    struct vchiq_dequeue_message32 __user *arg)
    {
    struct vchiq_dequeue_message32 args32;
    struct vchiq_dequeue_message args;
    if (copy_from_user(&args32, arg, sizeof(args32)))
    return -EFAULT;
    args = (struct vchiq_dequeue_message) {
    .handle		= args32.handle,
    .blocking	= args32.blocking,
    .bufsize	= args32.bufsize,
    .buf		= compat_ptr(args32.buf),
    };
    return vchiq_ioc_dequeue_message(file.private_data, &args);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_get_config32 {
    pub config_size: c_uint,
    pub pconfig: compat_uptr_t,
}

    _IOWR(VCHIQ_IOC_MAGIC, 10, struct vchiq_get_config32)
    static long
    vchiq_compat_ioctl_get_config(struct file *file,
    unsigned int cmd,
    struct vchiq_get_config32 __user *arg)
    {
    struct vchiq_get_config32 args32;
    struct vchiq_config config;
    void __user *ptr;
    if (copy_from_user(&args32, arg, sizeof(args32)))
    return -EFAULT;
    if (args32.config_size > sizeof(config))
    return -EINVAL;
    vchiq_get_config(&config);
    ptr = compat_ptr(args32.pconfig);
    if (copy_to_user(ptr, &config, args32.config_size))
    return -EFAULT;
    return 0;
    }
    static long
    vchiq_compat_ioctl(struct file *file, unsigned int cmd, unsigned long arg)
    {
    void __user *argp = compat_ptr(arg);
    switch (cmd) {
    case VCHIQ_IOC_CREATE_SERVICE32:
    return vchiq_compat_ioctl_create_service(file, cmd, argp);
    case VCHIQ_IOC_QUEUE_MESSAGE32:
    return vchiq_compat_ioctl_queue_message(file, cmd, argp);
    case VCHIQ_IOC_QUEUE_BULK_TRANSMIT32:
    case VCHIQ_IOC_QUEUE_BULK_RECEIVE32:
    return vchiq_compat_ioctl_queue_bulk(file, cmd, argp);
    case VCHIQ_IOC_AWAIT_COMPLETION32:
    return vchiq_compat_ioctl_await_completion(file, cmd, argp);
    case VCHIQ_IOC_DEQUEUE_MESSAGE32:
    return vchiq_compat_ioctl_dequeue_message(file, cmd, argp);
    case VCHIQ_IOC_GET_CONFIG32:
    return vchiq_compat_ioctl_get_config(file, cmd, argp);
    default:
    return vchiq_ioctl(file, cmd, (unsigned long)argp);
    }
    }

#[no_mangle]
unsafe extern "C" fn vchiq_open(inode: *mut inode, file: *mut file) -> c_int {
    static int vchiq_open(struct inode *inode, struct file *file)
    {
    struct miscdevice *vchiq_miscdev = file.private_data;
    struct vchiq_drv_mgmt *mgmt = dev_get_drvdata(vchiq_miscdev.parent);
    struct vchiq_state *state = &mgmt.state;
    struct vchiq_instance *instance;
    dev_dbg(state.dev, "arm: vchiq open\n");
    if (!vchiq_remote_initialised(state)) {
    dev_dbg(state.dev, "arm: vchiq has no connection to VideoCore\n");
    return -ENOTCONN;
    }
    instance = kzalloc_obj(*instance);
    if (!instance)
    return -ENOMEM;
    instance.state = state;
    instance.pid = current.tgid;
    vchiq_debugfs_add_instance(instance);
    init_completion(&instance.insert_event);
    init_completion(&instance.remove_event);
    mutex_init(&instance.completion_mutex);
    mutex_init(&instance.bulk_waiter_list_mutex);
    INIT_LIST_HEAD(&instance.bulk_waiter_list);
    file.private_data = instance;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vchiq_release(inode: *mut inode, file: *mut file) -> c_int {
    static int vchiq_release(struct inode *inode, struct file *file)
    {
    struct vchiq_instance *instance = file.private_data;
    struct vchiq_state *state = instance.state;
    struct vchiq_service *service;
    let mut ret: c_int = 0;
    int i;
    dev_dbg(state.dev, "arm: instance=%p\n", instance);
    if (!vchiq_remote_initialised(state)) {
    ret = -EPERM;
    goto out;
    }
// Ensure videocore is awake to allow termination.
    vchiq_use_internal(instance.state, core::ptr::null_mut(), USE_TYPE_VCHIQ);
    mutex_lock(&instance.completion_mutex);
// Wake the completion thread and ask it to exit
    instance.closing = 1;
    complete(&instance.insert_event);
    mutex_unlock(&instance.completion_mutex);
// Wake the slot handler if the completion queue is full.
    complete(&instance.remove_event);
// Mark all services for termination...
    i = 0;
    while ((service = next_service_by_instance(state, instance, &i))) {
    struct user_service *user_service = service.base.userdata;
// Wake the slot handler if the msg queue is full.
    complete(&user_service.remove_event);
    vchiq_terminate_service_internal(service);
    vchiq_service_put(service);
    }
// ...and wait for them to die
    i = 0;
    while ((service = next_service_by_instance(state, instance, &i))) {
    struct user_service *user_service = service.base.userdata;
    wait_for_completion(&service.remove_event);
    if (WARN_ON(service.srvstate != VCHIQ_SRVSTATE_FREE)) {
    vchiq_service_put(service);
    break;
    }
    spin_lock(&service.state.msg_queue_spinlock);
    while (user_service.msg_remove != user_service.msg_insert) {
    struct vchiq_header *header;
    let mut m: c_int = user_service.msg_remove & (MSG_QUEUE_SIZE - 1);
    header = user_service.msg_queue[m];
    user_service.msg_remove++;
    spin_unlock(&service.state.msg_queue_spinlock);
    if (header)
    vchiq_release_message(instance, service.handle, header);
    spin_lock(&service.state.msg_queue_spinlock);
    }
    spin_unlock(&service.state.msg_queue_spinlock);
    vchiq_service_put(service);
    }
// Release any closed services
    while (instance.completion_remove != instance.completion_insert) {
    struct vchiq_completion_data_kernel *completion;
    struct vchiq_service *service;
    completion = &instance.completions[instance.completion_remove
    & (MAX_COMPLETIONS - 1)];
    service = completion.service_userdata;
    if (completion.reason == VCHIQ_SERVICE_CLOSED) {
    struct user_service *user_service =
    service.base.userdata;
// Wake any blocked user-thread
    if (instance.use_close_delivered)
    complete(&user_service.close_event);
    vchiq_service_put(service);
    }
    instance.completion_remove++;
    }
// Release the PEER service count.
    vchiq_release_internal(instance.state, core::ptr::null_mut());
    free_bulk_waiter(instance);
    vchiq_debugfs_remove_instance(instance);
    kfree(instance);
    file.private_data = core::ptr::null_mut();
    out:
    return ret;
    }
    static const struct file_operations
    vchiq_fops = {
    .owner = THIS_MODULE,
    .unlocked_ioctl = vchiq_ioctl,

    .compat_ioctl = vchiq_compat_ioctl,

    .open = vchiq_open,
    .release = vchiq_release,
    };
    static struct miscdevice vchiq_miscdev = {
    .fops = &vchiq_fops,
    .minor = MISC_DYNAMIC_MINOR,
    .name = "vchiq",
    };
//
// vchiq_register_chrdev - Register the char driver for vchiq
// and create the necessary class and
// device files in userspace.
// @parent:	The parent of the char device.
//
// Returns 0 on success else returns the error code.
//
#[no_mangle]
pub unsafe extern "C" fn vchiq_register_chrdev(parent: *mut device) -> c_int {
    int vchiq_register_chrdev(struct device *parent)
    {
    vchiq_miscdev.parent = parent;
    return misc_register(&vchiq_miscdev);
    }
//
// vchiq_deregister_chrdev	- Deregister and cleanup the vchiq char
// driver and device files
//
#[no_mangle]
pub unsafe extern "C" fn vchiq_deregister_chrdev() {
    void vchiq_deregister_chrdev(void)
    {
    misc_deregister(&vchiq_miscdev);
    }
