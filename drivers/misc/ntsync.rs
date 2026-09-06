//! Automatically rewritten from C to Rust
//! Source: drivers/misc/ntsync.c
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
// ntsync.c - Kernel driver for NT synchronization primitives
//
// Copyright (C) 2024 Elizabeth Figura <zfigura@codeweavers.com>
//

    enum ntsync_type {
    NTSYNC_TYPE_SEM,
    NTSYNC_TYPE_MUTEX,
    NTSYNC_TYPE_EVENT,
    };
//
// Individual synchronization primitives are represented by
// struct ntsync_obj, and each primitive is backed by a file.
//
// The whole namespace is represented by a struct ntsync_device also
// backed by a file.
//
// Both rely on struct file for reference counting. Individual
// ntsync_obj objects take a reference to the device when created.
// Wait operations take a reference to each object being waited on for
// the duration of the wait.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntsync_obj {
    pub lock: spinlock_t,
    pub dev_locked: c_int,
    pub type: enum ntsync_type,
    pub file: *mut file,
    pub dev: *mut ntsync_device,
// The following fields are protected by the object lock.
    union {
    struct {
    pub count: __u32,
    pub max: __u32,
    pub sem: },
    struct {
    pub count: __u32,
    pub owner: pid_t,
    pub ownerdead: bool,
    pub mutex: },
    struct {
    pub manual: bool,
    pub signaled: bool,
    pub event: },
    pub u: },
//
// any_waiters is protected by the object lock, but all_waiters is
// protected by the device wait_all_lock.
//
    pub any_waiters: list_head,
    pub all_waiters: list_head,
//
// Hint describing how many tasks are queued on this object in a
// wait-all operation.
//
// Any time we do a wake, we may need to wake "all" waiters as well as
// "any" waiters. In order to atomically wake "all" waiters, we must
// lock all of the objects, and that means grabbing the wait_all_lock
// below (and, due to lock ordering rules, before locking this object).
// However, wait-all is a rare operation, and grabbing the wait-all
// lock for every wake would create unnecessary contention.
// Therefore we first check whether all_hint is zero, and, if it is,
// we skip trying to wake "all" waiters.
//
// Since wait requests must originate from user-space threads, we're
// limited here by PID_MAX_LIMIT, so there's no risk of overflow.
//
    pub all_hint: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntsync_q_entry {
    pub node: list_head,
    pub q: *mut ntsync_q,
    pub obj: *mut ntsync_obj,
    pub index: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntsync_q {
    pub task: *mut task_struct,
    pub owner: __u32,
//
// Protected via atomic_try_cmpxchg(). Only the thread that wins the
// compare-and-swap may actually change object states and wake this
// task.
//
    pub signaled: core::sync::atomic::AtomicI32,
    pub all: bool,
    pub ownerdead: bool,
    pub count: __u32,
    pub entries: [ntsync_q_entry; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntsync_device {
//
// Wait-all operations must atomically grab all objects, and be totally
// ordered with respect to each other and wait-any operations.
// If one thread is trying to acquire several objects, another thread
// cannot touch the object at the same time.
//
// This device-wide lock is used to serialize wait-for-all
// operations, and operations on an object that is involved in a
// wait-for-all.
//
    pub wait_all_lock: mutex,
    pub file: *mut file,
}

//
// Single objects are locked using obj->lock.
//
// Multiple objects are 'locked' while holding dev->wait_all_lock.
// In this case however, individual objects are not locked by holding
// obj->lock, but by setting obj->dev_locked.
//
// This means that in order to lock a single object, the sequence is slightly
// more complicated than usual. Specifically it needs to check obj->dev_locked
// after acquiring obj->lock, if set, it needs to drop the lock and acquire
// dev->wait_all_lock in order to serialize against the multi-object operation.
//
#[no_mangle]
unsafe extern "C" fn dev_lock_obj(dev: *mut ntsync_device, obj: *mut ntsync_obj) {
    static void dev_lock_obj(struct ntsync_device *dev, struct ntsync_obj *obj)
    {
    lockdep_assert_held(&dev.wait_all_lock);
    lockdep_assert(obj.dev == dev);
    spin_lock(&obj.lock);
//
// By setting obj->dev_locked inside obj->lock, it is ensured that
// anyone holding obj->lock must see the value.
//
    obj.dev_locked = 1;
    spin_unlock(&obj.lock);
    }
#[no_mangle]
unsafe extern "C" fn dev_unlock_obj(dev: *mut ntsync_device, obj: *mut ntsync_obj) {
    static void dev_unlock_obj(struct ntsync_device *dev, struct ntsync_obj *obj)
    {
    lockdep_assert_held(&dev.wait_all_lock);
    lockdep_assert(obj.dev == dev);
    spin_lock(&obj.lock);
    obj.dev_locked = 0;
    spin_unlock(&obj.lock);
    }
#[no_mangle]
unsafe extern "C" fn obj_lock(obj: *mut ntsync_obj) {
    static void obj_lock(struct ntsync_obj *obj)
    {
    struct ntsync_device *dev = obj.dev;
    for (;;) {
    spin_lock(&obj.lock);
    if (likely(!obj.dev_locked))
    break;
    spin_unlock(&obj.lock);
    mutex_lock(&dev.wait_all_lock);
    spin_lock(&obj.lock);
//
// obj->dev_locked should be set and released under the same
// wait_all_lock section, since we now own this lock, it should
// be clear.
//
    lockdep_assert(!obj.dev_locked);
    spin_unlock(&obj.lock);
    mutex_unlock(&dev.wait_all_lock);
    }
    }
#[no_mangle]
unsafe extern "C" fn obj_unlock(obj: *mut ntsync_obj) {
    static void obj_unlock(struct ntsync_obj *obj)
    {
    spin_unlock(&obj.lock);
    }
#[no_mangle]
unsafe extern "C" fn ntsync_lock_obj(dev: *mut ntsync_device, obj: *mut ntsync_obj) -> bool {
    static bool ntsync_lock_obj(struct ntsync_device *dev, struct ntsync_obj *obj)
    {
    bool all;
    obj_lock(obj);
    all = atomic_read(&obj.all_hint);
    if (unlikely(all)) {
    obj_unlock(obj);
    mutex_lock(&dev.wait_all_lock);
    dev_lock_obj(dev, obj);
    }
    return all;
    }
#[no_mangle]
unsafe extern "C" fn ntsync_unlock_obj(dev: *mut ntsync_device, obj: *mut ntsync_obj, all: bool) {
    static void ntsync_unlock_obj(struct ntsync_device *dev, struct ntsync_obj *obj, bool all)
    {
    if (all) {
    dev_unlock_obj(dev, obj);
    mutex_unlock(&dev.wait_all_lock);
    } else {
    obj_unlock(obj);
    }
    }

    lockdep_assert((lockdep_is_held(&(obj).lock) != LOCK_STATE_NOT_HELD) || \
    ((lockdep_is_held(&(obj).dev.wait_all_lock) != LOCK_STATE_NOT_HELD) && \
    (obj).dev_locked))
#[no_mangle]
unsafe extern "C" fn is_signaled(obj: *mut ntsync_obj, owner: __u32) -> bool {
    static bool is_signaled(struct ntsync_obj *obj, __u32 owner)
    {
    ntsync_assert_held(obj);
    switch (obj.type) {
    case NTSYNC_TYPE_SEM:
    return !!obj.u.sem.count;
    case NTSYNC_TYPE_MUTEX:
    if (obj.u.mutex.owner && obj.u.mutex.owner != owner)
    return false;
    return obj.u.mutex.count < UINT_MAX;
    case NTSYNC_TYPE_EVENT:
    return obj.u.event.signaled;
    }
    WARN(1, "bad object type %#x\n", obj.type);
    return false;
    }
//
// "locked_obj" is an optional pointer to an object which is already locked and
// should not be locked again. This is necessary so that changing an object's
// state and waking it can be a single atomic operation.
//
    static void try_wake_all(struct ntsync_device *dev, struct ntsync_q *q,
    struct ntsync_obj *locked_obj)
    {
    let mut count: __u32 = q.count;
    let mut can_wake: bool = true;
    let mut signaled: c_int = -1;
    __u32 i;
    lockdep_assert_held(&dev.wait_all_lock);
    if (locked_obj)
    lockdep_assert(locked_obj.dev_locked);
    for (i = 0; i < count; i++) {
    if (q.entries[i].obj != locked_obj)
    dev_lock_obj(dev, q.entries[i].obj);
    }
    for (i = 0; i < count; i++) {
    if (!is_signaled(q.entries[i].obj, q.owner)) {
    can_wake = false;
    break;
    }
    }
    if (can_wake && atomic_try_cmpxchg(&q.signaled, &signaled, 0)) {
    for (i = 0; i < count; i++) {
    struct ntsync_obj *obj = q.entries[i].obj;
    switch (obj.type) {
    case NTSYNC_TYPE_SEM:
    obj.u.sem.count--;
    break;
    case NTSYNC_TYPE_MUTEX:
    if (obj.u.mutex.ownerdead)
    q.ownerdead = true;
    obj.u.mutex.ownerdead = false;
    obj.u.mutex.count++;
    obj.u.mutex.owner = q.owner;
    break;
    case NTSYNC_TYPE_EVENT:
    if (!obj.u.event.manual)
    obj.u.event.signaled = false;
    break;
    }
    }
    wake_up_process(q.task);
    }
    for (i = 0; i < count; i++) {
    if (q.entries[i].obj != locked_obj)
    dev_unlock_obj(dev, q.entries[i].obj);
    }
    }
#[no_mangle]
unsafe extern "C" fn try_wake_all_obj(dev: *mut ntsync_device, obj: *mut ntsync_obj) {
    static void try_wake_all_obj(struct ntsync_device *dev, struct ntsync_obj *obj)
    {
    struct ntsync_q_entry *entry;
    lockdep_assert_held(&dev.wait_all_lock);
    lockdep_assert(obj.dev_locked);
    list_for_each_entry(entry, &obj.all_waiters, node)
    try_wake_all(dev, entry.q, obj);
    }
#[no_mangle]
unsafe extern "C" fn try_wake_any_sem(sem: *mut ntsync_obj) {
    static void try_wake_any_sem(struct ntsync_obj *sem)
    {
    struct ntsync_q_entry *entry;
    ntsync_assert_held(sem);
    lockdep_assert(sem.type == NTSYNC_TYPE_SEM);
    list_for_each_entry(entry, &sem.any_waiters, node) {
    struct ntsync_q *q = entry.q;
    let mut signaled: c_int = -1;
    if (!sem.u.sem.count)
    break;
    if (atomic_try_cmpxchg(&q.signaled, &signaled, entry.index)) {
    sem.u.sem.count--;
    wake_up_process(q.task);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn try_wake_any_mutex(mutex: *mut ntsync_obj) {
    static void try_wake_any_mutex(struct ntsync_obj *mutex)
    {
    struct ntsync_q_entry *entry;
    ntsync_assert_held(mutex);
    lockdep_assert(mutex.type == NTSYNC_TYPE_MUTEX);
    list_for_each_entry(entry, &mutex.any_waiters, node) {
    struct ntsync_q *q = entry.q;
    let mut signaled: c_int = -1;
    if (mutex.u.mutex.count == UINT_MAX)
    break;
    if (mutex.u.mutex.owner && mutex.u.mutex.owner != q.owner)
    continue;
    if (atomic_try_cmpxchg(&q.signaled, &signaled, entry.index)) {
    if (mutex.u.mutex.ownerdead)
    q.ownerdead = true;
    mutex.u.mutex.ownerdead = false;
    mutex.u.mutex.count++;
    mutex.u.mutex.owner = q.owner;
    wake_up_process(q.task);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn try_wake_any_event(event: *mut ntsync_obj) {
    static void try_wake_any_event(struct ntsync_obj *event)
    {
    struct ntsync_q_entry *entry;
    ntsync_assert_held(event);
    lockdep_assert(event.type == NTSYNC_TYPE_EVENT);
    list_for_each_entry(entry, &event.any_waiters, node) {
    struct ntsync_q *q = entry.q;
    let mut signaled: c_int = -1;
    if (!event.u.event.signaled)
    break;
    if (atomic_try_cmpxchg(&q.signaled, &signaled, entry.index)) {
    if (!event.u.event.manual)
    event.u.event.signaled = false;
    wake_up_process(q.task);
    }
    }
    }
//
// Actually change the semaphore state, returning -EOVERFLOW if it is made
// invalid.
//
#[no_mangle]
unsafe extern "C" fn release_sem_state(sem: *mut ntsync_obj, count: __u32) -> c_int {
    static int release_sem_state(struct ntsync_obj *sem, __u32 count)
    {
    __u32 sum;
    ntsync_assert_held(sem);
    if (check_add_overflow(sem.u.sem.count, count, &sum) ||
    sum > sem.u.sem.max)
    return -EOVERFLOW;
    sem.u.sem.count = sum;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ntsync_sem_release(sem: *mut ntsync_obj, argp: *mut void __user) -> c_int {
    static int ntsync_sem_release(struct ntsync_obj *sem, void __user *argp)
    {
    struct ntsync_device *dev = sem.dev;
    __u32 __user *user_args = argp;
    __u32 prev_count;
    __u32 args;
    bool all;
    int ret;
    if (copy_from_user(&args, argp, sizeof(args)))
    return -EFAULT;
    if (sem.type != NTSYNC_TYPE_SEM)
    return -EINVAL;
    all = ntsync_lock_obj(dev, sem);
    prev_count = sem.u.sem.count;
    ret = release_sem_state(sem, args);
    if (!ret) {
    if (all)
    try_wake_all_obj(dev, sem);
    try_wake_any_sem(sem);
    }
    ntsync_unlock_obj(dev, sem, all);
    if (!ret && put_user(prev_count, user_args))
    ret = -EFAULT;
    return ret;
    }
//
// Actually change the mutex state, returning -EPERM if not the owner.
//
    static int unlock_mutex_state(struct ntsync_obj *mutex,
    const struct ntsync_mutex_args *args)
    {
    ntsync_assert_held(mutex);
    if (mutex.u.mutex.owner != args.owner)
    return -EPERM;
    if (!--mutex.u.mutex.count)
    mutex.u.mutex.owner = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ntsync_mutex_unlock(mutex: *mut ntsync_obj, argp: *mut void __user) -> c_int {
    static int ntsync_mutex_unlock(struct ntsync_obj *mutex, void __user *argp)
    {
    struct ntsync_mutex_args __user *user_args = argp;
    struct ntsync_device *dev = mutex.dev;
    struct ntsync_mutex_args args;
    __u32 prev_count;
    bool all;
    int ret;
    if (copy_from_user(&args, argp, sizeof(args)))
    return -EFAULT;
    if (!args.owner)
    return -EINVAL;
    if (mutex.type != NTSYNC_TYPE_MUTEX)
    return -EINVAL;
    all = ntsync_lock_obj(dev, mutex);
    prev_count = mutex.u.mutex.count;
    ret = unlock_mutex_state(mutex, &args);
    if (!ret) {
    if (all)
    try_wake_all_obj(dev, mutex);
    try_wake_any_mutex(mutex);
    }
    ntsync_unlock_obj(dev, mutex, all);
    if (!ret && put_user(prev_count, &user_args.count))
    ret = -EFAULT;
    return ret;
    }
//
// Actually change the mutex state to mark its owner as dead,
// returning -EPERM if not the owner.
//
#[no_mangle]
unsafe extern "C" fn kill_mutex_state(mutex: *mut ntsync_obj, owner: __u32) -> c_int {
    static int kill_mutex_state(struct ntsync_obj *mutex, __u32 owner)
    {
    ntsync_assert_held(mutex);
    if (mutex.u.mutex.owner != owner)
    return -EPERM;
    mutex.u.mutex.ownerdead = true;
    mutex.u.mutex.owner = 0;
    mutex.u.mutex.count = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ntsync_mutex_kill(mutex: *mut ntsync_obj, argp: *mut void __user) -> c_int {
    static int ntsync_mutex_kill(struct ntsync_obj *mutex, void __user *argp)
    {
    struct ntsync_device *dev = mutex.dev;
    __u32 owner;
    bool all;
    int ret;
    if (get_user(owner, (__u32 __user *)argp))
    return -EFAULT;
    if (!owner)
    return -EINVAL;
    if (mutex.type != NTSYNC_TYPE_MUTEX)
    return -EINVAL;
    all = ntsync_lock_obj(dev, mutex);
    ret = kill_mutex_state(mutex, owner);
    if (!ret) {
    if (all)
    try_wake_all_obj(dev, mutex);
    try_wake_any_mutex(mutex);
    }
    ntsync_unlock_obj(dev, mutex, all);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ntsync_event_set(event: *mut ntsync_obj, argp: *mut void __user, pulse: bool) -> c_int {
    static int ntsync_event_set(struct ntsync_obj *event, void __user *argp, bool pulse)
    {
    struct ntsync_device *dev = event.dev;
    __u32 prev_state;
    bool all;
    if (event.type != NTSYNC_TYPE_EVENT)
    return -EINVAL;
    all = ntsync_lock_obj(dev, event);
    prev_state = event.u.event.signaled;
    event.u.event.signaled = true;
    if (all)
    try_wake_all_obj(dev, event);
    try_wake_any_event(event);
    if (pulse)
    event.u.event.signaled = false;
    ntsync_unlock_obj(dev, event, all);
    if (put_user(prev_state, (__u32 __user *)argp))
    return -EFAULT;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ntsync_event_reset(event: *mut ntsync_obj, argp: *mut void __user) -> c_int {
    static int ntsync_event_reset(struct ntsync_obj *event, void __user *argp)
    {
    struct ntsync_device *dev = event.dev;
    __u32 prev_state;
    bool all;
    if (event.type != NTSYNC_TYPE_EVENT)
    return -EINVAL;
    all = ntsync_lock_obj(dev, event);
    prev_state = event.u.event.signaled;
    event.u.event.signaled = false;
    ntsync_unlock_obj(dev, event, all);
    if (put_user(prev_state, (__u32 __user *)argp))
    return -EFAULT;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ntsync_sem_read(sem: *mut ntsync_obj, argp: *mut void __user) -> c_int {
    static int ntsync_sem_read(struct ntsync_obj *sem, void __user *argp)
    {
    struct ntsync_sem_args __user *user_args = argp;
    struct ntsync_device *dev = sem.dev;
    struct ntsync_sem_args args;
    bool all;
    if (sem.type != NTSYNC_TYPE_SEM)
    return -EINVAL;
    all = ntsync_lock_obj(dev, sem);
    args.count = sem.u.sem.count;
    args.max = sem.u.sem.max;
    ntsync_unlock_obj(dev, sem, all);
    if (copy_to_user(user_args, &args, sizeof(args)))
    return -EFAULT;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ntsync_mutex_read(mutex: *mut ntsync_obj, argp: *mut void __user) -> c_int {
    static int ntsync_mutex_read(struct ntsync_obj *mutex, void __user *argp)
    {
    struct ntsync_mutex_args __user *user_args = argp;
    struct ntsync_device *dev = mutex.dev;
    struct ntsync_mutex_args args;
    bool all;
    int ret;
    if (mutex.type != NTSYNC_TYPE_MUTEX)
    return -EINVAL;
    all = ntsync_lock_obj(dev, mutex);
    args.count = mutex.u.mutex.count;
    args.owner = mutex.u.mutex.owner;
    ret = mutex.u.mutex.ownerdead ? -EOWNERDEAD : 0;
    ntsync_unlock_obj(dev, mutex, all);
    if (copy_to_user(user_args, &args, sizeof(args)))
    return -EFAULT;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ntsync_event_read(event: *mut ntsync_obj, argp: *mut void __user) -> c_int {
    static int ntsync_event_read(struct ntsync_obj *event, void __user *argp)
    {
    struct ntsync_event_args __user *user_args = argp;
    struct ntsync_device *dev = event.dev;
    struct ntsync_event_args args;
    bool all;
    if (event.type != NTSYNC_TYPE_EVENT)
    return -EINVAL;
    all = ntsync_lock_obj(dev, event);
    args.manual = event.u.event.manual;
    args.signaled = event.u.event.signaled;
    ntsync_unlock_obj(dev, event, all);
    if (copy_to_user(user_args, &args, sizeof(args)))
    return -EFAULT;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ntsync_free_obj(obj: *mut ntsync_obj) {
    static void ntsync_free_obj(struct ntsync_obj *obj)
    {
    fput(obj.dev.file);
    kfree(obj);
    }
#[no_mangle]
unsafe extern "C" fn ntsync_obj_release(inode: *mut inode, file: *mut file) -> c_int {
    static int ntsync_obj_release(struct inode *inode, struct file *file)
    {
    ntsync_free_obj(file.private_data);
    return 0;
    }
    static long ntsync_obj_ioctl(struct file *file, unsigned int cmd,
    unsigned long parm)
    {
    struct ntsync_obj *obj = file.private_data;
    void __user *argp = (void __user *)parm;
    switch (cmd) {
    case NTSYNC_IOC_SEM_RELEASE:
    return ntsync_sem_release(obj, argp);
    case NTSYNC_IOC_SEM_READ:
    return ntsync_sem_read(obj, argp);
    case NTSYNC_IOC_MUTEX_UNLOCK:
    return ntsync_mutex_unlock(obj, argp);
    case NTSYNC_IOC_MUTEX_KILL:
    return ntsync_mutex_kill(obj, argp);
    case NTSYNC_IOC_MUTEX_READ:
    return ntsync_mutex_read(obj, argp);
    case NTSYNC_IOC_EVENT_SET:
    return ntsync_event_set(obj, argp, false);
    case NTSYNC_IOC_EVENT_RESET:
    return ntsync_event_reset(obj, argp);
    case NTSYNC_IOC_EVENT_PULSE:
    return ntsync_event_set(obj, argp, true);
    case NTSYNC_IOC_EVENT_READ:
    return ntsync_event_read(obj, argp);
    default:
    return -ENOIOCTLCMD;
    }
    }
    static const struct file_operations ntsync_obj_fops = {
    .owner		= THIS_MODULE,
    .release	= ntsync_obj_release,
    .unlocked_ioctl	= ntsync_obj_ioctl,
    .compat_ioctl	= compat_ptr_ioctl,
    };
    static struct ntsync_obj *ntsync_alloc_obj(struct ntsync_device *dev,
    enum ntsync_type type)
    {
    struct ntsync_obj *obj;
    obj = kzalloc_obj(*obj);
    if (!obj)
    return core::ptr::null_mut();
    obj.type = type;
    obj.dev = dev;
    get_file(dev.file);
    spin_lock_init(&obj.lock);
    INIT_LIST_HEAD(&obj.any_waiters);
    INIT_LIST_HEAD(&obj.all_waiters);
    atomic_set(&obj.all_hint, 0);
    return obj;
    }
#[no_mangle]
unsafe extern "C" fn ntsync_obj_get_fd(obj: *mut ntsync_obj) -> c_int {
    static int ntsync_obj_get_fd(struct ntsync_obj *obj)
    {
    FD_PREPARE(fdf, O_CLOEXEC,
    anon_inode_getfile("ntsync", &ntsync_obj_fops, obj, O_RDWR));
    if (fdf.err)
    return fdf.err;
    obj.file = fd_prepare_file(fdf);
    return fd_publish(fdf);
    }
#[no_mangle]
unsafe extern "C" fn ntsync_create_sem(dev: *mut ntsync_device, argp: *mut void __user) -> c_int {
    static int ntsync_create_sem(struct ntsync_device *dev, void __user *argp)
    {
    struct ntsync_sem_args args;
    struct ntsync_obj *sem;
    int fd;
    if (copy_from_user(&args, argp, sizeof(args)))
    return -EFAULT;
    if (args.count > args.max)
    return -EINVAL;
    sem = ntsync_alloc_obj(dev, NTSYNC_TYPE_SEM);
    if (!sem)
    return -ENOMEM;
    sem.u.sem.count = args.count;
    sem.u.sem.max = args.max;
    fd = ntsync_obj_get_fd(sem);
    if (fd < 0)
    ntsync_free_obj(sem);
    return fd;
    }
#[no_mangle]
unsafe extern "C" fn ntsync_create_mutex(dev: *mut ntsync_device, argp: *mut void __user) -> c_int {
    static int ntsync_create_mutex(struct ntsync_device *dev, void __user *argp)
    {
    struct ntsync_mutex_args args;
    struct ntsync_obj *mutex;
    int fd;
    if (copy_from_user(&args, argp, sizeof(args)))
    return -EFAULT;
    if (!args.owner != !args.count)
    return -EINVAL;
    mutex = ntsync_alloc_obj(dev, NTSYNC_TYPE_MUTEX);
    if (!mutex)
    return -ENOMEM;
    mutex.u.mutex.count = args.count;
    mutex.u.mutex.owner = args.owner;
    fd = ntsync_obj_get_fd(mutex);
    if (fd < 0)
    ntsync_free_obj(mutex);
    return fd;
    }
#[no_mangle]
unsafe extern "C" fn ntsync_create_event(dev: *mut ntsync_device, argp: *mut void __user) -> c_int {
    static int ntsync_create_event(struct ntsync_device *dev, void __user *argp)
    {
    struct ntsync_event_args args;
    struct ntsync_obj *event;
    int fd;
    if (copy_from_user(&args, argp, sizeof(args)))
    return -EFAULT;
    event = ntsync_alloc_obj(dev, NTSYNC_TYPE_EVENT);
    if (!event)
    return -ENOMEM;
    event.u.event.manual = args.manual;
    event.u.event.signaled = args.signaled;
    fd = ntsync_obj_get_fd(event);
    if (fd < 0)
    ntsync_free_obj(event);
    return fd;
    }
    static struct ntsync_obj *get_obj(struct ntsync_device *dev, int fd)
    {
    struct file *file = fget(fd);
    struct ntsync_obj *obj;
    if (!file)
    return core::ptr::null_mut();
    if (file.f_op != &ntsync_obj_fops) {
    fput(file);
    return core::ptr::null_mut();
    }
    obj = file.private_data;
    if (obj.dev != dev) {
    fput(file);
    return core::ptr::null_mut();
    }
    return obj;
    }
#[no_mangle]
unsafe extern "C" fn put_obj(obj: *mut ntsync_obj) {
    static void put_obj(struct ntsync_obj *obj)
    {
    fput(obj.file);
    }
#[no_mangle]
unsafe extern "C" fn ntsync_schedule(q: *const ntsync_q, args: *const ntsync_wait_args) -> c_int {
    static int ntsync_schedule(const struct ntsync_q *q, const struct ntsync_wait_args *args)
    {
    let mut timeout: ktime_t = ns_to_ktime(args.timeout);
    let mut clock: clockid_t = CLOCK_MONOTONIC;
    ktime_t *timeout_ptr;
    let mut ret: c_int = 0;
    timeout_ptr = (args.timeout == U64_MAX ? core::ptr::null_mut() : &timeout);
    if (args.flags & NTSYNC_WAIT_REALTIME)
    clock = CLOCK_REALTIME;
    else
    timeout = timens_ktime_to_host(clock, timeout);
    do {
    if (signal_pending(current)) {
    ret = -ERESTARTSYS;
    break;
    }
    set_current_state(TASK_INTERRUPTIBLE);
    if (atomic_read(&q.signaled) != -1) {
    ret = 0;
    break;
    }
    ret = schedule_hrtimeout_range_clock(timeout_ptr, 0, HRTIMER_MODE_ABS, clock);
    } while (ret < 0);
    __set_current_state(TASK_RUNNING);
    return ret;
    }
//
// Allocate and initialize the ntsync_q structure, but do not queue us yet.
//
    static int setup_wait(struct ntsync_device *dev,
    const struct ntsync_wait_args *args, bool all,
    struct ntsync_q **ret_q)
    {
    int fds[NTSYNC_MAX_WAIT_COUNT + 1];
    let mut count: __u32 = args.count;
    let mut size: usize = array_size(count, sizeof(fds[0]));
    struct ntsync_q *q;
    __u32 total_count;
    __u32 i, j;
    if (args.pad || (args.flags & ~NTSYNC_WAIT_REALTIME))
    return -EINVAL;
    if (!args.owner)
    return -EINVAL;
    if (size >= sizeof(fds))
    return -EINVAL;
    total_count = count;
    if (args.alert)
    total_count++;
    if (copy_from_user(fds, u64_to_user_ptr(args.objs), size))
    return -EFAULT;
    if (args.alert)
    fds[count] = args.alert;
    q = kmalloc_flex(*q, entries, total_count);
    if (!q)
    return -ENOMEM;
    q.task = current;
    q.owner = args.owner;
    atomic_set(&q.signaled, -1);
    q.all = all;
    q.ownerdead = false;
    q.count = count;
    for (i = 0; i < total_count; i++) {
    struct ntsync_q_entry *entry = &q.entries[i];
    struct ntsync_obj *obj = get_obj(dev, fds[i]);
    if (!obj)
    goto err;
    if (all) {
// Check that the objects are all distinct.
    for (j = 0; j < i; j++) {
    if (obj == q.entries[j].obj) {
    put_obj(obj);
    goto err;
    }
    }
    }
    entry.obj = obj;
    entry.q = q;
    entry.index = i;
    }
// ret_q = q;
    return 0;
    err:
    for (j = 0; j < i; j++)
    put_obj(q.entries[j].obj);
    kfree(q);
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn try_wake_any_obj(obj: *mut ntsync_obj) {
    static void try_wake_any_obj(struct ntsync_obj *obj)
    {
    switch (obj.type) {
    case NTSYNC_TYPE_SEM:
    try_wake_any_sem(obj);
    break;
    case NTSYNC_TYPE_MUTEX:
    try_wake_any_mutex(obj);
    break;
    case NTSYNC_TYPE_EVENT:
    try_wake_any_event(obj);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn ntsync_wait_any(dev: *mut ntsync_device, argp: *mut void __user) -> c_int {
    static int ntsync_wait_any(struct ntsync_device *dev, void __user *argp)
    {
    struct ntsync_wait_args args;
    __u32 i, total_count;
    struct ntsync_q *q;
    int signaled;
    bool all;
    int ret;
    if (copy_from_user(&args, argp, sizeof(args)))
    return -EFAULT;
    ret = setup_wait(dev, &args, false, &q);
    if (ret < 0)
    return ret;
    total_count = args.count;
    if (args.alert)
    total_count++;
// queue ourselves
    for (i = 0; i < total_count; i++) {
    struct ntsync_q_entry *entry = &q.entries[i];
    struct ntsync_obj *obj = entry.obj;
    all = ntsync_lock_obj(dev, obj);
    list_add_tail(&entry.node, &obj.any_waiters);
    ntsync_unlock_obj(dev, obj, all);
    }
//
// Check if we are already signaled.
//
// Note that the API requires that normal objects are checked before
// the alert event. Hence we queue the alert event last, and check
// objects in order.
//
    for (i = 0; i < total_count; i++) {
    struct ntsync_obj *obj = q.entries[i].obj;
    if (atomic_read(&q.signaled) != -1)
    break;
    all = ntsync_lock_obj(dev, obj);
    try_wake_any_obj(obj);
    ntsync_unlock_obj(dev, obj, all);
    }
// sleep
    ret = ntsync_schedule(q, &args);
// and finally, unqueue
    for (i = 0; i < total_count; i++) {
    struct ntsync_q_entry *entry = &q.entries[i];
    struct ntsync_obj *obj = entry.obj;
    all = ntsync_lock_obj(dev, obj);
    list_del(&entry.node);
    ntsync_unlock_obj(dev, obj, all);
    put_obj(obj);
    }
    signaled = atomic_read(&q.signaled);
    if (signaled != -1) {
    struct ntsync_wait_args __user *user_args = argp;
// even if we caught a signal, we need to communicate success
    ret = q.ownerdead ? -EOWNERDEAD : 0;
    if (put_user(signaled, &user_args.index))
    ret = -EFAULT;
    } else if (!ret) {
    ret = -ETIMEDOUT;
    }
    kfree(q);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ntsync_wait_all(dev: *mut ntsync_device, argp: *mut void __user) -> c_int {
    static int ntsync_wait_all(struct ntsync_device *dev, void __user *argp)
    {
    struct ntsync_wait_args args;
    struct ntsync_q *q;
    int signaled;
    __u32 i;
    int ret;
    if (copy_from_user(&args, argp, sizeof(args)))
    return -EFAULT;
    ret = setup_wait(dev, &args, true, &q);
    if (ret < 0)
    return ret;
// queue ourselves
    mutex_lock(&dev.wait_all_lock);
    for (i = 0; i < args.count; i++) {
    struct ntsync_q_entry *entry = &q.entries[i];
    struct ntsync_obj *obj = entry.obj;
    atomic_inc(&obj.all_hint);
//
// obj->all_waiters is protected by dev->wait_all_lock rather
// than obj->lock, so there is no need to acquire obj->lock
// here.
//
    list_add_tail(&entry.node, &obj.all_waiters);
    }
    if (args.alert) {
    struct ntsync_q_entry *entry = &q.entries[args.count];
    struct ntsync_obj *obj = entry.obj;
    dev_lock_obj(dev, obj);
    list_add_tail(&entry.node, &obj.any_waiters);
    dev_unlock_obj(dev, obj);
    }
// check if we are already signaled
    try_wake_all(dev, q, core::ptr::null_mut());
    mutex_unlock(&dev.wait_all_lock);
//
// Check if the alert event is signaled, making sure to do so only
// after checking if the other objects are signaled.
//
    if (args.alert) {
    struct ntsync_obj *obj = q.entries[args.count].obj;
    if (atomic_read(&q.signaled) == -1) {
    let mut all: bool = ntsync_lock_obj(dev, obj);
    try_wake_any_obj(obj);
    ntsync_unlock_obj(dev, obj, all);
    }
    }
// sleep
    ret = ntsync_schedule(q, &args);
// and finally, unqueue
    mutex_lock(&dev.wait_all_lock);
    for (i = 0; i < args.count; i++) {
    struct ntsync_q_entry *entry = &q.entries[i];
    struct ntsync_obj *obj = entry.obj;
//
// obj->all_waiters is protected by dev->wait_all_lock rather
// than obj->lock, so there is no need to acquire it here.
//
    list_del(&entry.node);
    atomic_dec(&obj.all_hint);
    put_obj(obj);
    }
    mutex_unlock(&dev.wait_all_lock);
    if (args.alert) {
    struct ntsync_q_entry *entry = &q.entries[args.count];
    struct ntsync_obj *obj = entry.obj;
    bool all;
    all = ntsync_lock_obj(dev, obj);
    list_del(&entry.node);
    ntsync_unlock_obj(dev, obj, all);
    put_obj(obj);
    }
    signaled = atomic_read(&q.signaled);
    if (signaled != -1) {
    struct ntsync_wait_args __user *user_args = argp;
// even if we caught a signal, we need to communicate success
    ret = q.ownerdead ? -EOWNERDEAD : 0;
    if (put_user(signaled, &user_args.index))
    ret = -EFAULT;
    } else if (!ret) {
    ret = -ETIMEDOUT;
    }
    kfree(q);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ntsync_char_open(inode: *mut inode, file: *mut file) -> c_int {
    static int ntsync_char_open(struct inode *inode, struct file *file)
    {
    struct ntsync_device *dev;
    dev = kzalloc_obj(*dev);
    if (!dev)
    return -ENOMEM;
    mutex_init(&dev.wait_all_lock);
    file.private_data = dev;
    dev.file = file;
    return nonseekable_open(inode, file);
    }
#[no_mangle]
unsafe extern "C" fn ntsync_char_release(inode: *mut inode, file: *mut file) -> c_int {
    static int ntsync_char_release(struct inode *inode, struct file *file)
    {
    struct ntsync_device *dev = file.private_data;
    kfree(dev);
    return 0;
    }
    static long ntsync_char_ioctl(struct file *file, unsigned int cmd,
    unsigned long parm)
    {
    struct ntsync_device *dev = file.private_data;
    void __user *argp = (void __user *)parm;
    switch (cmd) {
    case NTSYNC_IOC_CREATE_EVENT:
    return ntsync_create_event(dev, argp);
    case NTSYNC_IOC_CREATE_MUTEX:
    return ntsync_create_mutex(dev, argp);
    case NTSYNC_IOC_CREATE_SEM:
    return ntsync_create_sem(dev, argp);
    case NTSYNC_IOC_WAIT_ALL:
    return ntsync_wait_all(dev, argp);
    case NTSYNC_IOC_WAIT_ANY:
    return ntsync_wait_any(dev, argp);
    default:
    return -ENOIOCTLCMD;
    }
    }
    static const struct file_operations ntsync_fops = {
    .owner		= THIS_MODULE,
    .open		= ntsync_char_open,
    .release	= ntsync_char_release,
    .unlocked_ioctl	= ntsync_char_ioctl,
    .compat_ioctl	= compat_ptr_ioctl,
    };
    static struct miscdevice ntsync_misc = {
    .minor		= MISC_DYNAMIC_MINOR,
    .name		= NTSYNC_NAME,
    .fops		= &ntsync_fops,
    .mode		= 0666,
    };
    module_misc_device(ntsync_misc);
    MODULE_AUTHOR("Elizabeth Figura <zfigura@codeweavers.com>");
    MODULE_DESCRIPTION("Kernel driver for NT synchronization primitives");
    MODULE_LICENSE("GPL");
