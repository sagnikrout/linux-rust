//! Automatically rewritten from C to Rust
//! Source: drivers/ptp/ptp_clock.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// PTP 1588 clock support
//
// Copyright (C) 2010 OMICRON electronics GmbH
//

pub const PTP_MAX_ALARMS: c_int = 4;

    const struct class ptp_class = {
    .name = "ptp",
    .dev_groups = ptp_groups
    };
// private globals
    static dev_t ptp_devt;
    static DEFINE_XARRAY_ALLOC(ptp_clocks_map);
// time stamp event queue operations
#[no_mangle]
pub unsafe extern "C" fn queue_free(q: *mut timestamp_event_queue) -> c_int {
    static inline int queue_free(struct timestamp_event_queue *q)
    {
    return PTP_MAX_TIMESTAMPS - queue_cnt(q) - 1;
    }
    static void enqueue_external_timestamp(struct timestamp_event_queue *queue,
    struct ptp_clock_event *src)
    {
    struct ptp_extts_event *dst;
    struct timespec64 offset_ts;
    unsigned long flags;
    s64 seconds;
    u32 remainder;
    if (src.type == PTP_CLOCK_EXTTS) {
    seconds = div_u64_rem(src.timestamp, 1000000000, &remainder);
    } else if (src.type == PTP_CLOCK_EXTOFF) {
    offset_ts = ns_to_timespec64(src.offset);
    seconds = offset_ts.tv_sec;
    remainder = offset_ts.tv_nsec;
    } else {
    WARN(1, "%s: unknown type %d\n", __func__, src.type);
    return;
    }
    spin_lock_irqsave(&queue.lock, flags);
    dst = &queue.buf[queue.tail];
    dst.index = src.index;
    dst.flags = PTP_EXTTS_EVENT_VALID;
    dst.t.sec = seconds;
    dst.t.nsec = remainder;
    if (src.type == PTP_CLOCK_EXTOFF)
    dst.flags |= PTP_EXT_OFFSET;
// Both WRITE_ONCE() are paired with READ_ONCE() in queue_cnt()
    if (!queue_free(queue))
    WRITE_ONCE(queue.head, (queue.head + 1) % PTP_MAX_TIMESTAMPS);
    WRITE_ONCE(queue.tail, (queue.tail + 1) % PTP_MAX_TIMESTAMPS);
    spin_unlock_irqrestore(&queue.lock, flags);
    }
// posix clock implementation
#[no_mangle]
unsafe extern "C" fn ptp_clock_getres(pc: *mut posix_clock, tp: *mut timespec64) -> c_int {
    static int ptp_clock_getres(struct posix_clock *pc, struct timespec64 *tp)
    {
    tp.tv_sec = 0;
    tp.tv_nsec = 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ptp_clock_settime(pc: *mut posix_clock, tp: *const timespec64) -> c_int {
    static int ptp_clock_settime(struct posix_clock *pc, const struct timespec64 *tp)
    {
    struct ptp_clock *ptp = container_of(pc, struct ptp_clock, clock);
    if (ptp_clock_freerun(ptp)) {
    pr_err_ratelimited("ptp: physical clock is free running\n");
    return -EBUSY;
    }
    if (!timespec64_valid_settod(tp))
    return -EINVAL;
    return  ptp.info.settime64(ptp.info, tp);
    }
#[no_mangle]
unsafe extern "C" fn ptp_clock_gettime(pc: *mut posix_clock, tp: *mut timespec64) -> c_int {
    static int ptp_clock_gettime(struct posix_clock *pc, struct timespec64 *tp)
    {
    struct ptp_clock *ptp = container_of(pc, struct ptp_clock, clock);
    int err;
    if (ptp.info.gettimex64)
    err = ptp.info.gettimex64(ptp.info, tp, core::ptr::null_mut());
    else
    err = ptp.info.gettime64(ptp.info, tp);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ptp_clock_adjtime(pc: *mut posix_clock, tx: *mut __kernel_timex) -> c_int {
    static int ptp_clock_adjtime(struct posix_clock *pc, struct __kernel_timex *tx)
    {
    struct ptp_clock *ptp = container_of(pc, struct ptp_clock, clock);
    struct ptp_clock_info *ops;
    let mut err: c_int = -EOPNOTSUPP;
    if (tx.modes & (ADJ_SETOFFSET | ADJ_FREQUENCY | ADJ_OFFSET) &&
    ptp_clock_freerun(ptp)) {
    pr_err("ptp: physical clock is free running\n");
    return -EBUSY;
    }
    ops = ptp.info;
    if (tx.modes & ADJ_SETOFFSET) {
    struct timespec64 ts, ts2;
    ktime_t kt;
    s64 delta;
    ts.tv_sec  = tx.time.tv_sec;
    ts.tv_nsec = tx.time.tv_usec;
    if (!(tx.modes & ADJ_NANO))
    ts.tv_nsec *= 1000;
    if ((unsigned long) ts.tv_nsec >= NSEC_PER_SEC)
    return -EINVAL;
// Make sure the offset is valid
    err = ptp_clock_gettime(pc, &ts2);
    if (err)
    return err;
    ts2 = timespec64_add(ts2, ts);
    if (!timespec64_valid_settod(&ts2))
    return -EINVAL;
    kt = timespec64_to_ktime(ts);
    delta = ktime_to_ns(kt);
    err = ops.adjtime(ops, delta);
    } else if (tx.modes & ADJ_FREQUENCY) {
    long ppb;
    s64 tmp;
//
// scaled_ppm_to_ppb() multiplies (1 + freq) by 125 in s64;
// reject a ->freq large enough to overflow that, which would
// otherwise wrap the result back into the max_adj range.
//
    if (check_add_overflow((s64)tx.freq, (s64)1, &tmp) ||
    check_mul_overflow(tmp, (s64)125, &tmp))
    return -ERANGE;
    ppb = scaled_ppm_to_ppb(tx.freq);
    if (ppb > ops.max_adj || ppb < -ops.max_adj)
    return -ERANGE;
    err = ops.adjfine(ops, tx.freq);
    if (!err)
    ptp.dialed_frequency = tx.freq;
    } else if (tx.modes & ADJ_OFFSET) {
    if (ops.adjphase) {
    let mut max_phase_adj: i32 = ops.getmaxphase(ops);
    let mut offset: i32 = tx.offset;
    if (!(tx.modes & ADJ_NANO))
    offset *= NSEC_PER_USEC;
    if (offset > max_phase_adj || offset < -max_phase_adj)
    return -ERANGE;
    err = ops.adjphase(ops, offset);
    }
    } else if (tx.modes == 0) {
    tx.freq = ptp.dialed_frequency;
    err = 0;
    }
    return err;
    }
    static struct posix_clock_operations ptp_clock_ops = {
    .owner		= THIS_MODULE,
    .clock_adjtime	= ptp_clock_adjtime,
    .clock_gettime	= ptp_clock_gettime,
    .clock_getres	= ptp_clock_getres,
    .clock_settime	= ptp_clock_settime,
    .ioctl		= ptp_ioctl,
    .open		= ptp_open,
    .release	= ptp_release,
    .poll		= ptp_poll,
    .read		= ptp_read,
    };
#[no_mangle]
unsafe extern "C" fn ptp_clock_release(dev: *mut device) {
    static void ptp_clock_release(struct device *dev)
    {
    struct ptp_clock *ptp = container_of(dev, struct ptp_clock, dev);
    struct timestamp_event_queue *tsevq;
    unsigned long flags;
    ptp_cleanup_pin_groups(ptp);
    kfree(ptp.vclock_index);
    mutex_destroy(&ptp.pincfg_mux);
    mutex_destroy(&ptp.n_vclocks_mux);
// Delete first entry
    spin_lock_irqsave(&ptp.tsevqs_lock, flags);
    tsevq = list_first_entry(&ptp.tsevqs, struct timestamp_event_queue,
    qlist);
    list_del(&tsevq.qlist);
    spin_unlock_irqrestore(&ptp.tsevqs_lock, flags);
    bitmap_free(tsevq.mask);
    kfree(tsevq);
    debugfs_remove(ptp.debugfs_root);
    xa_erase(&ptp_clocks_map, ptp.index);
    kfree(ptp);
    }
#[no_mangle]
unsafe extern "C" fn ptp_getcycles64(info: *mut ptp_clock_info, ts: *mut timespec64) -> c_int {
    static int ptp_getcycles64(struct ptp_clock_info *info, struct timespec64 *ts)
    {
    if (info.getcyclesx64)
    return info.getcyclesx64(info, ts, core::ptr::null_mut());
    else
    return info.gettime64(info, ts);
    }
#[no_mangle]
unsafe extern "C" fn ptp_enable(ptp: *mut ptp_clock_info, request: *mut ptp_clock_request, on: c_int) -> c_int {
    static int ptp_enable(struct ptp_clock_info *ptp, struct ptp_clock_request *request, int on)
    {
    return -EOPNOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn ptp_aux_kworker(work: *mut kthread_work) {
    static void ptp_aux_kworker(struct kthread_work *work)
    {
    struct ptp_clock *ptp = container_of(work, struct ptp_clock,
    aux_work.work);
    struct ptp_clock_info *info = ptp.info;
    long delay;
    delay = info.do_aux_work(info);
    if (delay >= 0)
    kthread_queue_delayed_work(ptp.kworker, &ptp.aux_work, delay);
    }
    static ssize_t ptp_n_perout_loopback_read(struct file *filep,
    char __user *buffer,
    size_t count, loff_t *pos)
    {
    struct ptp_clock *ptp = filep.private_data;
    char buf[12] = {};
    snprintf(buf, sizeof(buf), "%d\n", ptp.info.n_per_lp);
    return simple_read_from_buffer(buffer, count, pos, buf, strlen(buf));
    }
    static const struct file_operations ptp_n_perout_loopback_fops = {
    .owner	= THIS_MODULE,
    .open	= simple_open,
    .read	= ptp_n_perout_loopback_read,
    };
    static ssize_t ptp_perout_loopback_write(struct file *filep,
    const char __user *buffer,
    size_t count, loff_t *ppos)
    {
    struct ptp_clock *ptp = filep.private_data;
    struct ptp_clock_info *ops = ptp.info;
    unsigned int index, enable;
    int len, cnt, err;
    char buf[32] = {};
    if (*ppos || !count)
    return -EINVAL;
    if (count >= sizeof(buf))
    return -ENOSPC;
    len = simple_write_to_buffer(buf, sizeof(buf) - 1,
    ppos, buffer, count);
    if (len < 0)
    return len;
    buf[len] = '\0';
    cnt = sscanf(buf, "%u %u", &index, &enable);
    if (cnt != 2)
    return -EINVAL;
    if (index >= ops.n_per_lp)
    return -EINVAL;
    if (enable != 0 && enable != 1)
    return -EINVAL;
    err = ops.perout_loopback(ops, index, enable);
    if (err)
    return err;
    return count;
    }
    static const struct file_operations ptp_perout_loopback_ops = {
    .owner   = THIS_MODULE,
    .open    = simple_open,
    .write	 = ptp_perout_loopback_write,
    };
// public interface
    struct ptp_clock *ptp_clock_register(struct ptp_clock_info *info,
    struct device *parent)
    {
    struct ptp_clock *ptp;
    struct timestamp_event_queue *queue = core::ptr::null_mut();
    int err, index, major = MAJOR(ptp_devt);
    char debugfsname[16];
    size_t size;
    if (WARN_ON_ONCE(info.n_alarm > PTP_MAX_ALARMS ||
    (!info.gettimex64 && !info.gettime64) ||
    !info.settime64))
    return ERR_PTR(-EINVAL);
// Initialize a clock structure.
    ptp = kzalloc_obj(struct ptp_clock);
    if (!ptp) {
    err = -ENOMEM;
    goto no_memory;
    }
    err = xa_alloc(&ptp_clocks_map, &index, ptp, xa_limit_31b,
    GFP_KERNEL);
    if (err)
    goto no_slot;
    ptp.clock.ops = ptp_clock_ops;
    ptp.info = info;
    ptp.devid = MKDEV(major, index);
    ptp.index = index;
    INIT_LIST_HEAD(&ptp.tsevqs);
    queue = kzalloc_obj(*queue);
    if (!queue) {
    err = -ENOMEM;
    goto no_memory_queue;
    }
    list_add_tail(&queue.qlist, &ptp.tsevqs);
    spin_lock_init(&ptp.tsevqs_lock);
    queue.mask = bitmap_alloc(PTP_MAX_CHANNELS, GFP_KERNEL);
    if (!queue.mask) {
    err = -ENOMEM;
    goto no_memory_bitmap;
    }
    bitmap_set(queue.mask, 0, PTP_MAX_CHANNELS);
    spin_lock_init(&queue.lock);
    mutex_init(&ptp.pincfg_mux);
    mutex_init(&ptp.n_vclocks_mux);
    init_waitqueue_head(&ptp.tsev_wq);
    if (ptp.info.getcycles64 || ptp.info.getcyclesx64) {
    ptp.has_cycles = true;
    if (!ptp.info.getcycles64 && ptp.info.getcyclesx64)
    ptp.info.getcycles64 = ptp_getcycles64;
    } else {
// Free running cycle counter not supported, use time.
    ptp.info.getcycles64 = ptp_getcycles64;
    if (ptp.info.gettimex64)
    ptp.info.getcyclesx64 = ptp.info.gettimex64;
    if (ptp.info.getcrosststamp)
    ptp.info.getcrosscycles = ptp.info.getcrosststamp;
    }
    if (!ptp.info.enable)
    ptp.info.enable = ptp_enable;
    if (ptp.info.do_aux_work) {
    kthread_init_delayed_work(&ptp.aux_work, ptp_aux_kworker);
    ptp.kworker = kthread_run_worker(0, "ptp%d", ptp.index);
    if (IS_ERR(ptp.kworker)) {
    err = PTR_ERR(ptp.kworker);
    pr_err("failed to create ptp aux_worker %d\n", err);
    goto kworker_err;
    }
    }
// PTP virtual clock is being registered under physical clock
    if (parent && parent.class && parent.class.name &&
    strcmp(parent.class.name, "ptp") == 0)
    ptp.is_virtual_clock = true;
    if (!ptp.is_virtual_clock) {
    ptp.max_vclocks = PTP_DEFAULT_MAX_VCLOCKS;
    size = sizeof(int) * ptp.max_vclocks;
    ptp.vclock_index = kzalloc(size, GFP_KERNEL);
    if (!ptp.vclock_index) {
    err = -ENOMEM;
    goto no_mem_for_vclocks;
    }
    }
    err = ptp_populate_pin_groups(ptp);
    if (err)
    goto no_pin_groups;
// Register a new PPS source.
    if (info.pps) {
    struct pps_source_info pps;
    memset(&pps, 0, sizeof(pps));
    snprintf(pps.name, PPS_MAX_NAME_LEN, "ptp%d", index);
    pps.mode = PTP_PPS_MODE;
    pps.owner = info.owner;
    ptp.pps_source = pps_register_source(&pps, PTP_PPS_DEFAULTS);
    if (IS_ERR(ptp.pps_source)) {
    err = PTR_ERR(ptp.pps_source);
    pr_err("failed to register pps source\n");
    goto no_pps;
    }
    ptp.pps_source.lookup_cookie = ptp;
    }
// Initialize a new device of our class in our clock structure.
    device_initialize(&ptp.dev);
    ptp.dev.devt = ptp.devid;
    ptp.dev.class = &ptp_class;
    ptp.dev.parent = parent;
    ptp.dev.groups = ptp.pin_attr_groups;
    ptp.dev.release = ptp_clock_release;
    dev_set_drvdata(&ptp.dev, ptp);
    dev_set_name(&ptp.dev, "ptp%d", ptp.index);
// Create a posix clock and link it to the device.
    err = posix_clock_register(&ptp.clock, &ptp.dev);
    if (err) {
    if (ptp.pps_source)
    pps_unregister_source(ptp.pps_source);
    if (ptp.kworker)
    kthread_destroy_worker(ptp.kworker);
    put_device(&ptp.dev);
    pr_err("failed to create posix clock\n");
    return ERR_PTR(err);
    }
// Debugfs initialization
    snprintf(debugfsname, sizeof(debugfsname), "ptp%d", ptp.index);
    ptp.debugfs_root = debugfs_create_dir(debugfsname, core::ptr::null_mut());
    if (info.n_per_lp > 0 && info.perout_loopback) {
    debugfs_create_file("n_perout_loopback", 0400, ptp.debugfs_root,
    ptp, &ptp_n_perout_loopback_fops);
    debugfs_create_file("perout_loopback", 0200, ptp.debugfs_root,
    ptp, &ptp_perout_loopback_ops);
    }
    return ptp;
    no_pps:
    ptp_cleanup_pin_groups(ptp);
    no_pin_groups:
    kfree(ptp.vclock_index);
    no_mem_for_vclocks:
    if (ptp.kworker)
    kthread_destroy_worker(ptp.kworker);
    kworker_err:
    mutex_destroy(&ptp.pincfg_mux);
    mutex_destroy(&ptp.n_vclocks_mux);
    bitmap_free(queue.mask);
    no_memory_bitmap:
    list_del(&queue.qlist);
    kfree(queue);
    no_memory_queue:
    xa_erase(&ptp_clocks_map, index);
    no_slot:
    kfree(ptp);
    no_memory:
    return ERR_PTR(err);
    }
    EXPORT_SYMBOL(ptp_clock_register);
#[no_mangle]
unsafe extern "C" fn unregister_vclock(dev: *mut device, data: *mut c_void) -> c_int {
    static int unregister_vclock(struct device *dev, void *data)
    {
    struct ptp_clock *ptp = dev_get_drvdata(dev);
    ptp_vclock_unregister(info_to_vclock(ptp.info));
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ptp_clock_unregister(ptp: *mut ptp_clock) -> c_int {
    int ptp_clock_unregister(struct ptp_clock *ptp)
    {
    if (ptp_vclock_in_use(ptp)) {
    device_for_each_child(&ptp.dev, core::ptr::null_mut(), unregister_vclock);
    }
// Get the device to stop posix_clock_unregister() doing the last put
// and freeing the structure(s)
//
    get_device(&ptp.dev);
// Wake up any userspace waiting for an event.
    ptp.defunct = 1;
    wake_up_interruptible(&ptp.tsev_wq);
// Tear down the POSIX clock, which removes the user interface.
    posix_clock_unregister(&ptp.clock);
// Disable all sources of event generation.
    ptp_disable_all_events(ptp);
    if (ptp.kworker) {
    kthread_cancel_delayed_work_sync(&ptp.aux_work);
    kthread_destroy_worker(ptp.kworker);
    }
// Release the clock's resources.
    if (ptp.pps_source)
    pps_unregister_source(ptp.pps_source);
// The final put, normally here, will invoke ptp_clock_release().
    put_device(&ptp.dev);
    return 0;
    }
    EXPORT_SYMBOL(ptp_clock_unregister);
#[no_mangle]
pub unsafe extern "C" fn ptp_clock_event(ptp: *mut ptp_clock, event: *mut ptp_clock_event) {
    void ptp_clock_event(struct ptp_clock *ptp, struct ptp_clock_event *event)
    {
    struct timestamp_event_queue *tsevq;
    struct pps_event_time evt;
    unsigned long flags;
    switch (event.type) {
    case PTP_CLOCK_ALARM:
    break;
    case PTP_CLOCK_EXTTS:
    case PTP_CLOCK_EXTOFF:
// Enqueue timestamp on selected queues
    spin_lock_irqsave(&ptp.tsevqs_lock, flags);
    list_for_each_entry(tsevq, &ptp.tsevqs, qlist) {
    if (test_bit((unsigned int)event.index, tsevq.mask))
    enqueue_external_timestamp(tsevq, event);
    }
    spin_unlock_irqrestore(&ptp.tsevqs_lock, flags);
    wake_up_interruptible(&ptp.tsev_wq);
    break;
    case PTP_CLOCK_PPS:
    pps_get_ts(&evt);
    pps_event(ptp.pps_source, &evt, PTP_PPS_EVENT, core::ptr::null_mut());
    break;
    case PTP_CLOCK_PPSUSR:
    pps_event(ptp.pps_source, &event.pps_times,
    PTP_PPS_EVENT, core::ptr::null_mut());
    break;
    }
    }
    EXPORT_SYMBOL(ptp_clock_event);
#[no_mangle]
pub unsafe extern "C" fn ptp_clock_index(ptp: *mut ptp_clock) -> c_int {
    int ptp_clock_index(struct ptp_clock *ptp)
    {
    return ptp.index;
    }
    EXPORT_SYMBOL(ptp_clock_index);
#[no_mangle]
unsafe extern "C" fn ptp_clock_of_node_match(dev: *mut device, data: *const c_void) -> c_int {
    static int ptp_clock_of_node_match(struct device *dev, const void *data)
    {
    const struct device_node *parent_np = data;
    return (dev.parent && dev_of_node(dev.parent) == parent_np);
    }
#[no_mangle]
pub unsafe extern "C" fn ptp_clock_index_by_of_node(np: *mut device_node) -> c_int {
    int ptp_clock_index_by_of_node(struct device_node *np)
    {
    struct ptp_clock *ptp;
    struct device *dev;
    int phc_index;
    dev = class_find_device(&ptp_class, core::ptr::null_mut(), np,
    ptp_clock_of_node_match);
    if (!dev)
    return -1;
    ptp = dev_get_drvdata(dev);
    phc_index = ptp_clock_index(ptp);
    put_device(dev);
    return phc_index;
    }
    EXPORT_SYMBOL_GPL(ptp_clock_index_by_of_node);
#[no_mangle]
unsafe extern "C" fn ptp_clock_dev_match(dev: *mut device, data: *const c_void) -> c_int {
    static int ptp_clock_dev_match(struct device *dev, const void *data)
    {
    const struct device *parent = data;
    return dev.parent == parent;
    }
#[no_mangle]
pub unsafe extern "C" fn ptp_clock_index_by_dev(parent: *mut device) -> c_int {
    int ptp_clock_index_by_dev(struct device *parent)
    {
    struct ptp_clock *ptp;
    struct device *dev;
    int phc_index;
    dev = class_find_device(&ptp_class, core::ptr::null_mut(), parent,
    ptp_clock_dev_match);
    if (!dev)
    return -1;
    ptp = dev_get_drvdata(dev);
    phc_index = ptp_clock_index(ptp);
    put_device(dev);
    return phc_index;
    }
    EXPORT_SYMBOL_GPL(ptp_clock_index_by_dev);
    int ptp_find_pin(struct ptp_clock *ptp,
    enum ptp_pin_function func, unsigned int chan)
    {
    struct ptp_pin_desc *pin = core::ptr::null_mut();
    int i;
    for (i = 0; i < ptp.info.n_pins; i++) {
    if (ptp.info.pin_config[i].func == func &&
    ptp.info.pin_config[i].chan == chan) {
    pin = &ptp.info.pin_config[i];
    break;
    }
    }
    return pin ? i : -1;
    }
    EXPORT_SYMBOL(ptp_find_pin);
    int ptp_find_pin_unlocked(struct ptp_clock *ptp,
    enum ptp_pin_function func, unsigned int chan)
    {
    int result;
    mutex_lock(&ptp.pincfg_mux);
    result = ptp_find_pin(ptp, func, chan);
    mutex_unlock(&ptp.pincfg_mux);
    return result;
    }
    EXPORT_SYMBOL(ptp_find_pin_unlocked);
#[no_mangle]
pub unsafe extern "C" fn ptp_schedule_worker(ptp: *mut ptp_clock, delay: c_ulong) -> c_int {
    int ptp_schedule_worker(struct ptp_clock *ptp, unsigned long delay)
    {
    return kthread_mod_delayed_work(ptp.kworker, &ptp.aux_work, delay);
    }
    EXPORT_SYMBOL(ptp_schedule_worker);
#[no_mangle]
pub unsafe extern "C" fn ptp_cancel_worker_sync(ptp: *mut ptp_clock) {
    void ptp_cancel_worker_sync(struct ptp_clock *ptp)
    {
    kthread_cancel_delayed_work_sync(&ptp.aux_work);
    }
    EXPORT_SYMBOL(ptp_cancel_worker_sync);
// module operations
#[no_mangle]
unsafe extern "C" fn ptp_exit() -> void __exit {
    static void __exit ptp_exit(void)
    {
    class_unregister(&ptp_class);
    unregister_chrdev_region(ptp_devt, MINORMASK + 1);
    xa_destroy(&ptp_clocks_map);
    }
#[no_mangle]
unsafe extern "C" fn ptp_init() -> int __init {
    static int __init ptp_init(void)
    {
    int err;
    err = class_register(&ptp_class);
    if (err) {
    pr_err("ptp: failed to allocate class\n");
    return err;
    }
    err = alloc_chrdev_region(&ptp_devt, 0, MINORMASK + 1, "ptp");
    if (err < 0) {
    pr_err("ptp: failed to allocate device region\n");
    goto no_region;
    }
    pr_info("PTP clock support registered\n");
    return 0;
    no_region:
    class_unregister(&ptp_class);
    return err;
    }
    subsys_initcall(ptp_init);
    module_exit(ptp_exit);
    MODULE_AUTHOR("Richard Cochran <richardcochran@gmail.com>");
    MODULE_DESCRIPTION("PTP clocks support");
    MODULE_LICENSE("GPL");
