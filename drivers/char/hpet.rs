//! Automatically rewritten from C to Rust
//! Source: drivers/char/hpet.c
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
// Intel & MS High Precision Event Timer Implementation.
//
// Copyright (C) 2003 Intel Corporation
// Venki Pallipadi
// (c) Copyright 2004 Hewlett-Packard Development Company, L.P.
// Bob Picco <robert.picco@hp.com>
//

//
// The High Precision Event Timer driver.
// This driver is closely modelled after the rtc.c driver.
// See HPET spec revision 1.
//

// WARNING -- don't get confused.  These macros are never used
// to write the (single) counter, and rarely to read it.
// They're badly named; to fix, someday.
//

    static DEFINE_MUTEX(hpet_mutex); /* replaces BKL */
    static u32 hpet_nhpet, hpet_max_freq = HPET_USER_FREQ;
// A lock for concurrent access by app and isr hpet activity.
    static DEFINE_SPINLOCK(hpet_lock);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpet_dev {
    pub hd_hpets: *mut hpets,
    pub hd_hpet: *mut hpet __iomem,
    pub hd_timer: *mut hpet_timer __iomem,
    pub hd_ireqfreq: c_ulong,
    pub hd_irqdata: c_ulong,
    pub hd_waitqueue: wait_queue_head_t,
    pub hd_async_queue: *mut fasync_struct,
    pub hd_flags: c_uint,
    pub hd_irq: c_uint,
    pub hd_hdwirq: c_uint,
    pub hd_name: [c_char; HPET_DEV_NAME],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpets {
    pub hp_next: *mut hpets,
    pub hp_hpet: *mut hpet __iomem,
    pub hp_hpet_phys: c_ulong,
    pub hp_tick_freq: c_ulonglong,
    pub hp_delta: c_ulong,
    pub hp_ntimer: c_uint,
    pub hp_which: c_uint,
    pub __counted_by(hp_ntimer): hpet_dev hp_dev[],
}

    static struct hpets *hpets;
pub const HPET_OPEN: c_uint = 0x0001;
pub const HPET_IE: c_uint = 0x0002	/* interrupt enabled */;
pub const HPET_PERIODIC: c_uint = 0x0004;
pub const HPET_SHARED_IRQ: c_uint = 0x0008;
#[no_mangle]
unsafe extern "C" fn hpet_interrupt(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t hpet_interrupt(int irq, void *data)
    {
    struct hpet_dev *devp;
    unsigned long isr;
    devp = data;
    isr = 1 << (devp - devp.hd_hpets.hp_dev);
    if ((devp.hd_flags & HPET_SHARED_IRQ) &&
    !(isr & readl(&devp.hd_hpet.hpet_isr)))
    return IRQ_NONE;
    spin_lock(&hpet_lock);
    devp.hd_irqdata++;
//
// For non-periodic timers, increment the accumulator.
// This has the effect of treating non-periodic like periodic.
//
    if ((devp.hd_flags & (HPET_IE | HPET_PERIODIC)) == HPET_IE) {
    unsigned long t, mc, base, k;
    struct hpet __iomem *hpet = devp.hd_hpet;
    struct hpets *hpetp = devp.hd_hpets;
    t = devp.hd_ireqfreq;
    read_counter(&devp.hd_timer.hpet_compare);
    mc = read_counter(&hpet.hpet_mc);
// The time for the next interrupt would logically be t + m,
// however, if we are very unlucky and the interrupt is delayed
// for longer than t then we will completely miss the next
// interrupt if we set t + m and an application will hang.
// Therefore we need to make a more complex computation assuming
// that there exists a k for which the following is true:
// k * t + base < mc + delta
// (k + 1) * t + base > mc + delta
// where t is the interval in hpet ticks for the given freq,
// base is the theoretical start value 0 < base < t,
// mc is the main counter value at the time of the interrupt,
// delta is the time it takes to write the a value to the
// comparator.
// k may then be computed as (mc - base + delta) / t .
//
    base = mc % t;
    k = (mc - base + hpetp.hp_delta) / t;
    write_counter(t * (k + 1) + base,
    &devp.hd_timer.hpet_compare);
    }
    if (devp.hd_flags & HPET_SHARED_IRQ)
    writel(isr, &devp.hd_hpet.hpet_isr);
    spin_unlock(&hpet_lock);
    wake_up_interruptible(&devp.hd_waitqueue);
    kill_fasync(&devp.hd_async_queue, SIGIO, POLL_IN);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn hpet_timer_set_irq(devp: *mut hpet_dev) {
    static void hpet_timer_set_irq(struct hpet_dev *devp)
    {
    let mut nr_irqs: c_uint = irq_get_nr_irqs();
    unsigned long v;
    int irq, gsi;
    struct hpet_timer __iomem *timer;
    spin_lock_irq(&hpet_lock);
    if (devp.hd_hdwirq) {
    spin_unlock_irq(&hpet_lock);
    return;
    }
    timer = devp.hd_timer;
// we prefer level triggered mode
    v = readl(&timer.hpet_config);
    if (!(v & Tn_INT_TYPE_CNF_MASK)) {
    v |= Tn_INT_TYPE_CNF_MASK;
    writel(v, &timer.hpet_config);
    }
    spin_unlock_irq(&hpet_lock);
    v = (readq(&timer.hpet_config) & Tn_INT_ROUTE_CAP_MASK) >>
    Tn_INT_ROUTE_CAP_SHIFT;
//
// In PIC mode, skip IRQ0-4, IRQ6-9, IRQ12-15 which is always used by
// legacy device. In IO APIC mode, we skip all the legacy IRQS.
//
    if (acpi_irq_model == ACPI_IRQ_MODEL_PIC)
    v &= ~0xf3df;
    else
    v &= ~0xffff;
    for_each_set_bit(irq, &v, HPET_MAX_IRQ) {
    if (irq >= nr_irqs) {
    irq = HPET_MAX_IRQ;
    break;
    }
    gsi = acpi_register_gsi(core::ptr::null_mut(), irq, ACPI_LEVEL_SENSITIVE,
    ACPI_ACTIVE_LOW);
    if (gsi > 0)
    break;
// FIXME: Setup interrupt source table
    }
    if (irq < HPET_MAX_IRQ) {
    spin_lock_irq(&hpet_lock);
    v = readl(&timer.hpet_config);
    v |= irq << Tn_INT_ROUTE_CNF_SHIFT;
    writel(v, &timer.hpet_config);
    devp.hd_hdwirq = gsi;
    spin_unlock_irq(&hpet_lock);
    }
    return;
    }
#[no_mangle]
unsafe extern "C" fn hpet_open(inode: *mut inode, file: *mut file) -> c_int {
    static int hpet_open(struct inode *inode, struct file *file)
    {
    struct hpet_dev *devp;
    struct hpets *hpetp;
    int i;
    if (file.f_mode & FMODE_WRITE)
    return -EINVAL;
    mutex_lock(&hpet_mutex);
    spin_lock_irq(&hpet_lock);
    for (devp = core::ptr::null_mut(), hpetp = hpets; hpetp && !devp; hpetp = hpetp.hp_next)
    for (i = 0; i < hpetp.hp_ntimer; i++)
    if (hpetp.hp_dev[i].hd_flags & HPET_OPEN) {
    continue;
    } else {
    devp = &hpetp.hp_dev[i];
    break;
    }
    if (!devp) {
    spin_unlock_irq(&hpet_lock);
    mutex_unlock(&hpet_mutex);
    return -EBUSY;
    }
    file.private_data = devp;
    devp.hd_irqdata = 0;
    devp.hd_flags |= HPET_OPEN;
    spin_unlock_irq(&hpet_lock);
    mutex_unlock(&hpet_mutex);
    hpet_timer_set_irq(devp);
    return 0;
    }
    static ssize_t
    hpet_read(struct file *file, char __user *buf, size_t count, loff_t * ppos)
    {
    DECLARE_WAITQUEUE(wait, current);
    unsigned long data;
    ssize_t retval;
    struct hpet_dev *devp;
    devp = file.private_data;
    if (!devp.hd_ireqfreq)
    return -EIO;
    if (in_compat_syscall()) {
    if (count < sizeof(compat_ulong_t))
    return -EINVAL;
    } else {
    if (count < sizeof(unsigned long))
    return -EINVAL;
    }
    add_wait_queue(&devp.hd_waitqueue, &wait);
    for ( ; ; ) {
    set_current_state(TASK_INTERRUPTIBLE);
    spin_lock_irq(&hpet_lock);
    data = devp.hd_irqdata;
    devp.hd_irqdata = 0;
    spin_unlock_irq(&hpet_lock);
    if (data) {
    break;
    } else if (file.f_flags & O_NONBLOCK) {
    retval = -EAGAIN;
    goto out;
    } else if (signal_pending(current)) {
    retval = -ERESTARTSYS;
    goto out;
    }
    schedule();
    }
    if (in_compat_syscall()) {
    retval = put_user(data, (compat_ulong_t __user *)buf);
    if (!retval)
    retval = sizeof(compat_ulong_t);
    } else {
    retval = put_user(data, (unsigned long __user *)buf);
    if (!retval)
    retval = sizeof(unsigned long);
    }
    out:
    __set_current_state(TASK_RUNNING);
    remove_wait_queue(&devp.hd_waitqueue, &wait);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn hpet_poll(file: *mut file, wait: *mut *mut poll_table) -> __poll_t {
    static __poll_t hpet_poll(struct file *file, poll_table * wait)
    {
    unsigned long v;
    struct hpet_dev *devp;
    devp = file.private_data;
    if (!devp.hd_ireqfreq)
    return 0;
    poll_wait(file, &devp.hd_waitqueue, wait);
    spin_lock_irq(&hpet_lock);
    v = devp.hd_irqdata;
    spin_unlock_irq(&hpet_lock);
    if (v != 0)
    return EPOLLIN | EPOLLRDNORM;
    return 0;
    }

    let mut hpet_mmap_enabled: static int = 1;

    let mut hpet_mmap_enabled: static int = 0;

#[no_mangle]
unsafe extern "C" fn hpet_mmap_enable(str: *mut c_char) -> __init int {
    static __init int hpet_mmap_enable(char *str)
    {
    get_option(&str, &hpet_mmap_enabled);
    pr_info("HPET mmap %s\n", hpet_mmap_enabled ? "enabled" : "disabled");
    return 1;
    }
    __setup("hpet_mmap=", hpet_mmap_enable);
#[no_mangle]
unsafe extern "C" fn hpet_mmap_prepare(desc: *mut vm_area_desc) -> c_int {
    static int hpet_mmap_prepare(struct vm_area_desc *desc)
    {
    struct file *file = desc.file;
    struct hpet_dev *devp;
    unsigned long addr;
    if (!hpet_mmap_enabled)
    return -EACCES;
    devp = file.private_data;
    addr = devp.hd_hpets.hp_hpet_phys;
    if (addr & (PAGE_SIZE - 1))
    return -ENOSYS;
    desc.page_prot = pgprot_noncached(desc.page_prot);
    mmap_action_simple_ioremap(desc, addr, PAGE_SIZE);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn hpet_mmap_prepare(desc: *mut vm_area_desc) -> c_int {
    static int hpet_mmap_prepare(struct vm_area_desc *desc)
    {
    return -ENOSYS;
    }

#[no_mangle]
unsafe extern "C" fn hpet_fasync(fd: c_int, file: *mut file, on: c_int) -> c_int {
    static int hpet_fasync(int fd, struct file *file, int on)
    {
    struct hpet_dev *devp;
    devp = file.private_data;
    if (fasync_helper(fd, file, on, &devp.hd_async_queue) >= 0)
    return 0;
    else
    return -EIO;
    }
#[no_mangle]
unsafe extern "C" fn hpet_release(inode: *mut inode, file: *mut file) -> c_int {
    static int hpet_release(struct inode *inode, struct file *file)
    {
    struct hpet_dev *devp;
    struct hpet_timer __iomem *timer;
    let mut irq: c_int = 0;
    devp = file.private_data;
    timer = devp.hd_timer;
    spin_lock_irq(&hpet_lock);
    writeq((readq(&timer.hpet_config) & ~Tn_INT_ENB_CNF_MASK),
    &timer.hpet_config);
    irq = devp.hd_irq;
    devp.hd_irq = 0;
    devp.hd_ireqfreq = 0;
    if (devp.hd_flags & HPET_PERIODIC
    && readq(&timer.hpet_config) & Tn_TYPE_CNF_MASK) {
    unsigned long v;
    v = readq(&timer.hpet_config);
    v ^= Tn_TYPE_CNF_MASK;
    writeq(v, &timer.hpet_config);
    }
    devp.hd_flags &= ~(HPET_OPEN | HPET_IE | HPET_PERIODIC);
    spin_unlock_irq(&hpet_lock);
    if (irq)
    free_irq(irq, devp);
    file.private_data = core::ptr::null_mut();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hpet_ioctl_ieon(devp: *mut hpet_dev) -> c_int {
    static int hpet_ioctl_ieon(struct hpet_dev *devp)
    {
    struct hpet_timer __iomem *timer;
    struct hpet __iomem *hpet;
    struct hpets *hpetp;
    int irq;
    unsigned long g, v, t, m;
    unsigned long flags, isr;
    timer = devp.hd_timer;
    hpet = devp.hd_hpet;
    hpetp = devp.hd_hpets;
    if (!devp.hd_ireqfreq)
    return -EIO;
    spin_lock_irq(&hpet_lock);
    if (devp.hd_flags & HPET_IE) {
    spin_unlock_irq(&hpet_lock);
    return -EBUSY;
    }
    devp.hd_flags |= HPET_IE;
    if (readl(&timer.hpet_config) & Tn_INT_TYPE_CNF_MASK)
    devp.hd_flags |= HPET_SHARED_IRQ;
    spin_unlock_irq(&hpet_lock);
    irq = devp.hd_hdwirq;
    if (irq) {
    unsigned long irq_flags;
    if (devp.hd_flags & HPET_SHARED_IRQ) {
//
// To prevent the interrupt handler from seeing an
// unwanted interrupt status bit, program the timer
// so that it will not fire in the near future ...
//
    writel(readl(&timer.hpet_config) & ~Tn_TYPE_CNF_MASK,
    &timer.hpet_config);
    write_counter(read_counter(&hpet.hpet_mc),
    &timer.hpet_compare);
// ... and clear any left-over status.
    isr = 1 << (devp - devp.hd_hpets.hp_dev);
    writel(isr, &hpet.hpet_isr);
    }
    sprintf(devp.hd_name, "hpet%d", (int)(devp - hpetp.hp_dev));
    irq_flags = devp.hd_flags & HPET_SHARED_IRQ ? IRQF_SHARED : 0;
    if (request_irq(irq, hpet_interrupt, irq_flags,
    devp.hd_name, (void *)devp)) {
    printk(KERN_ERR "hpet: IRQ %d is not free\n", irq);
    irq = 0;
    }
    }
    if (irq == 0) {
    spin_lock_irq(&hpet_lock);
    devp.hd_flags ^= HPET_IE;
    spin_unlock_irq(&hpet_lock);
    return -EIO;
    }
    devp.hd_irq = irq;
    t = devp.hd_ireqfreq;
    v = readq(&timer.hpet_config);
// 64-bit comparators are not yet supported through the ioctls,
// so force this into 32-bit mode if it supports both modes
//
    g = v | Tn_32MODE_CNF_MASK | Tn_INT_ENB_CNF_MASK;
    if (devp.hd_flags & HPET_PERIODIC) {
    g |= Tn_TYPE_CNF_MASK;
    v |= Tn_TYPE_CNF_MASK | Tn_VAL_SET_CNF_MASK;
    writeq(v, &timer.hpet_config);
    local_irq_save(flags);
//
// NOTE: First we modify the hidden accumulator
// register supported by periodic-capable comparators.
// We never want to modify the (single) counter; that
// would affect all the comparators. The value written
// is the counter value when the first interrupt is due.
//
    m = read_counter(&hpet.hpet_mc);
    write_counter(t + m + hpetp.hp_delta, &timer.hpet_compare);
//
// Then we modify the comparator, indicating the period
// for subsequent interrupt.
//
    write_counter(t, &timer.hpet_compare);
    } else {
    local_irq_save(flags);
    m = read_counter(&hpet.hpet_mc);
    write_counter(t + m + hpetp.hp_delta, &timer.hpet_compare);
    }
    if (devp.hd_flags & HPET_SHARED_IRQ) {
    isr = 1 << (devp - devp.hd_hpets.hp_dev);
    writel(isr, &hpet.hpet_isr);
    }
    writeq(g, &timer.hpet_config);
    local_irq_restore(flags);
    return 0;
    }
// converts Hz to number of timer ticks
    static inline unsigned long hpet_time_div(struct hpets *hpets,
    unsigned long dis)
    {
    unsigned long long m;
    m = hpets.hp_tick_freq + (dis >> 1);
    return div64_ul(m, dis);
    }
    static int
    hpet_ioctl_common(struct hpet_dev *devp, unsigned int cmd, unsigned long arg,
    struct hpet_info *info)
    {
    struct hpet_timer __iomem *timer;
    struct hpets *hpetp;
    int err;
    unsigned long v;
    switch (cmd) {
    case HPET_IE_OFF:
    case HPET_INFO:
    case HPET_EPI:
    case HPET_DPI:
    case HPET_IRQFREQ:
    timer = devp.hd_timer;
    hpetp = devp.hd_hpets;
    break;
    case HPET_IE_ON:
    return hpet_ioctl_ieon(devp);
    default:
    return -EINVAL;
    }
    err = 0;
    switch (cmd) {
    case HPET_IE_OFF:
    if ((devp.hd_flags & HPET_IE) == 0)
    break;
    v = readq(&timer.hpet_config);
    v &= ~Tn_INT_ENB_CNF_MASK;
    writeq(v, &timer.hpet_config);
    if (devp.hd_irq) {
    free_irq(devp.hd_irq, devp);
    devp.hd_irq = 0;
    }
    devp.hd_flags ^= HPET_IE;
    break;
    case HPET_INFO:
    {
    memset(info, 0, sizeof(*info));
    if (devp.hd_ireqfreq)
    info.hi_ireqfreq =
    hpet_time_div(hpetp, devp.hd_ireqfreq);
    info.hi_flags =
    readq(&timer.hpet_config) & Tn_PER_INT_CAP_MASK;
    info.hi_hpet = hpetp.hp_which;
    info.hi_timer = devp - hpetp.hp_dev;
    break;
    }
    case HPET_EPI:
    v = readq(&timer.hpet_config);
    if ((v & Tn_PER_INT_CAP_MASK) == 0) {
    err = -ENXIO;
    break;
    }
    devp.hd_flags |= HPET_PERIODIC;
    break;
    case HPET_DPI:
    v = readq(&timer.hpet_config);
    if ((v & Tn_PER_INT_CAP_MASK) == 0) {
    err = -ENXIO;
    break;
    }
    if (devp.hd_flags & HPET_PERIODIC &&
    readq(&timer.hpet_config) & Tn_TYPE_CNF_MASK) {
    v = readq(&timer.hpet_config);
    v ^= Tn_TYPE_CNF_MASK;
    writeq(v, &timer.hpet_config);
    }
    devp.hd_flags &= ~HPET_PERIODIC;
    break;
    case HPET_IRQFREQ:
    if ((arg > hpet_max_freq) &&
    !capable(CAP_SYS_RESOURCE)) {
    err = -EACCES;
    break;
    }
    if (!arg) {
    err = -EINVAL;
    break;
    }
    devp.hd_ireqfreq = hpet_time_div(hpetp, arg);
    }
    return err;
    }
    static long
    hpet_ioctl(struct file *file, unsigned int cmd, unsigned long arg)
    {
    struct hpet_info info;
    int err;
    mutex_lock(&hpet_mutex);
    err = hpet_ioctl_common(file.private_data, cmd, arg, &info);
    mutex_unlock(&hpet_mutex);
    if ((cmd == HPET_INFO) && !err &&
    (copy_to_user((void __user *)arg, &info, sizeof(info))))
    err = -EFAULT;
    return err;
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_hpet_info {
    pub /: *mut *mut compat_ulong_t hi_ireqfreq; / Hz,
    pub /: *mut *mut compat_ulong_t hi_flags; / information,
    pub hi_hpet: c_ushort,
    pub hi_timer: c_ushort,
}

// 32-bit types would lead to different command codes which should be
// translated into 64-bit ones before passed to hpet_ioctl_common
//

    static long
    hpet_compat_ioctl(struct file *file, unsigned int cmd, unsigned long arg)
    {
    struct hpet_info info;
    int err;
    if (cmd == COMPAT_HPET_INFO)
    cmd = HPET_INFO;
    if (cmd == COMPAT_HPET_IRQFREQ)
    cmd = HPET_IRQFREQ;
    mutex_lock(&hpet_mutex);
    err = hpet_ioctl_common(file.private_data, cmd, arg, &info);
    mutex_unlock(&hpet_mutex);
    if ((cmd == HPET_INFO) && !err) {
    struct compat_hpet_info __user *u = compat_ptr(arg);
    if (put_user(info.hi_ireqfreq, &u.hi_ireqfreq) ||
    put_user(info.hi_flags, &u.hi_flags) ||
    put_user(info.hi_hpet, &u.hi_hpet) ||
    put_user(info.hi_timer, &u.hi_timer))
    err = -EFAULT;
    }
    return err;
    }

    static const struct file_operations hpet_fops = {
    .owner = THIS_MODULE,
    .read = hpet_read,
    .poll = hpet_poll,
    .unlocked_ioctl = hpet_ioctl,

    .compat_ioctl = hpet_compat_ioctl,

    .open = hpet_open,
    .release = hpet_release,
    .fasync = hpet_fasync,
    .mmap_prepare = hpet_mmap_prepare,
    };
#[no_mangle]
unsafe extern "C" fn hpet_is_known(hdp: *mut hpet_data) -> c_int {
    static int hpet_is_known(struct hpet_data *hdp)
    {
    struct hpets *hpetp;
    for (hpetp = hpets; hpetp; hpetp = hpetp.hp_next)
    if (hpetp.hp_hpet_phys == hdp.hd_phys_address)
    return 1;
    return 0;
    }
    static const struct ctl_table hpet_table[] = {
    {
    .procname = "max-user-freq",
    .data = &hpet_max_freq,
    .maxlen = sizeof(int),
    .mode = 0644,
    .proc_handler = proc_dointvec,
    },
    };
    static struct ctl_table_header *sysctl_header;
//
// Adjustment for when arming the timer with
// initial conditions.  That is, main counter
// ticks expired before interrupts are enabled.
//

#[no_mangle]
unsafe extern "C" fn __hpet_calibrate(hpetp: *mut hpets) -> c_ulong {
    static unsigned long __hpet_calibrate(struct hpets *hpetp)
    {
    struct hpet_timer __iomem *timer = core::ptr::null_mut();
    unsigned long t, m, count, i, flags, start;
    struct hpet_dev *devp;
    int j;
    struct hpet __iomem *hpet;
    for (j = 0, devp = hpetp.hp_dev; j < hpetp.hp_ntimer; j++, devp++)
    if ((devp.hd_flags & HPET_OPEN) == 0) {
    timer = devp.hd_timer;
    break;
    }
    if (!timer)
    return 0;
    hpet = hpetp.hp_hpet;
    t = read_counter(&timer.hpet_compare);
    i = 0;
    count = hpet_time_div(hpetp, TICK_CALIBRATE);
    local_irq_save(flags);
    start = read_counter(&hpet.hpet_mc);
    do {
    m = read_counter(&hpet.hpet_mc);
    write_counter(t + m + hpetp.hp_delta, &timer.hpet_compare);
    } while (i++, (m - start) < count);
    local_irq_restore(flags);
    return (m - start) / i;
    }
#[no_mangle]
unsafe extern "C" fn hpet_calibrate(hpetp: *mut hpets) -> c_ulong {
    static unsigned long hpet_calibrate(struct hpets *hpetp)
    {
    let mut ret: c_ulong = ~0UL;
    unsigned long tmp;
//
// Try to calibrate until return value becomes stable small value.
// If SMI interruption occurs in calibration loop, the return value
// will be big. This avoids its impact.
//
    for ( ; ; ) {
    tmp = __hpet_calibrate(hpetp);
    if (ret <= tmp)
    break;
    ret = tmp;
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn hpet_alloc(hdp: *mut hpet_data) -> c_int {
    int hpet_alloc(struct hpet_data *hdp)
    {
    u64 cap, mcfg;
    struct hpet_dev *devp;
    u32 i, ntimer;
    struct hpets *hpetp;
    struct hpet __iomem *hpet;
    static struct hpets *last;
    u32 period;
    unsigned long long temp;
    u32 remainder;
//
// hpet_alloc can be called by platform dependent code.
// If platform dependent code has allocated the hpet that
// ACPI has also reported, then we catch it here.
//
    if (hpet_is_known(hdp)) {
    printk(KERN_DEBUG "%s: duplicate HPET ignored\n",
    __func__);
    return 0;
    }
    hpetp = kzalloc_flex(*hpetp, hp_dev, hdp.hd_nirqs);
    if (!hpetp)
    return -ENOMEM;
    hpetp.hp_which = hpet_nhpet++;
    hpetp.hp_hpet = hdp.hd_address;
    hpetp.hp_hpet_phys = hdp.hd_phys_address;
    hpetp.hp_ntimer = hdp.hd_nirqs;
    for (i = 0; i < hdp.hd_nirqs; i++)
    hpetp.hp_dev[i].hd_hdwirq = hdp.hd_irq[i];
    hpet = hpetp.hp_hpet;
    cap = readq(&hpet.hpet_cap);
    ntimer = ((cap & HPET_NUM_TIM_CAP_MASK) >> HPET_NUM_TIM_CAP_SHIFT) + 1;
    if (hpetp.hp_ntimer != ntimer) {
    printk(KERN_WARNING "hpet: number irqs doesn't agree"
    " with number of timers\n");
    kfree(hpetp);
    return -ENODEV;
    }
    if (last)
    last.hp_next = hpetp;
    else
    hpets = hpetp;
    last = hpetp;
    period = (cap & HPET_COUNTER_CLK_PERIOD_MASK) >>
    HPET_COUNTER_CLK_PERIOD_SHIFT; /* fs, 10^-15 */
    temp = 1000000000000000uLL; /* 10^15 femtoseconds per second */
    temp += period >> 1; /* round */
    do_div(temp, period);
    hpetp.hp_tick_freq = temp; /* ticks per second */
    printk(KERN_INFO "hpet%u: at MMIO 0x%lx, IRQ%s",
    hpetp.hp_which, hdp.hd_phys_address,
    str_plural(hpetp.hp_ntimer));
    for (i = 0; i < hpetp.hp_ntimer; i++)
    printk(KERN_CONT "%s %u", i > 0 ? "," : "", hdp.hd_irq[i]);
    printk(KERN_CONT "\n");
    temp = hpetp.hp_tick_freq;
    remainder = do_div(temp, 1000000);
    printk(KERN_INFO
    "hpet%u: %u comparators, %d-bit %u.%06u MHz counter\n",
    hpetp.hp_which, hpetp.hp_ntimer,
    cap & HPET_COUNTER_SIZE_MASK ? 64 : 32,
    (unsigned) temp, remainder);
    mcfg = readq(&hpet.hpet_config);
    if ((mcfg & HPET_ENABLE_CNF_MASK) == 0) {
    write_counter(0L, &hpet.hpet_mc);
    mcfg |= HPET_ENABLE_CNF_MASK;
    writeq(mcfg, &hpet.hpet_config);
    }
    for (i = 0, devp = hpetp.hp_dev; i < hpetp.hp_ntimer; i++, devp++) {
    struct hpet_timer __iomem *timer;
    timer = &hpet.hpet_timers[devp - hpetp.hp_dev];
    devp.hd_hpets = hpetp;
    devp.hd_hpet = hpet;
    devp.hd_timer = timer;
//
// If the timer was reserved by platform code,
// then make timer unavailable for opens.
//
    if (hdp.hd_state & (1 << i)) {
    devp.hd_flags = HPET_OPEN;
    continue;
    }
    init_waitqueue_head(&devp.hd_waitqueue);
    }
    hpetp.hp_delta = hpet_calibrate(hpetp);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hpet_resources(res: *mut acpi_resource, data: *mut c_void) -> acpi_status {
    static acpi_status hpet_resources(struct acpi_resource *res, void *data)
    {
    struct hpet_data *hdp;
    acpi_status status;
    struct acpi_resource_address64 addr;
    hdp = data;
    status = acpi_resource_to_address64(res, &addr);
    if (ACPI_SUCCESS(status)) {
    hdp.hd_phys_address = addr.address.minimum;
    hdp.hd_address = ioremap(addr.address.minimum, addr.address.address_length);
    if (!hdp.hd_address)
    return AE_ERROR;
    if (hpet_is_known(hdp)) {
    iounmap(hdp.hd_address);
    return AE_ALREADY_EXISTS;
    }
    } else if (res.type == ACPI_RESOURCE_TYPE_FIXED_MEMORY32) {
    struct acpi_resource_fixed_memory32 *fixmem32;
    fixmem32 = &res.data.fixed_memory32;
    hdp.hd_phys_address = fixmem32.address;
    hdp.hd_address = ioremap(fixmem32.address,
    HPET_RANGE_SIZE);
    if (!hdp.hd_address)
    return AE_ERROR;
    if (hpet_is_known(hdp)) {
    iounmap(hdp.hd_address);
    return AE_ALREADY_EXISTS;
    }
    } else if (res.type == ACPI_RESOURCE_TYPE_EXTENDED_IRQ) {
    struct acpi_resource_extended_irq *irqp;
    int i, irq;
    irqp = &res.data.extended_irq;
    for (i = 0; i < irqp.interrupt_count; i++) {
    if (hdp.hd_nirqs >= HPET_MAX_TIMERS)
    break;
    irq = acpi_register_gsi(core::ptr::null_mut(), irqp.interrupts[i],
    irqp.triggering,
    irqp.polarity);
    if (irq < 0)
    return AE_ERROR;
    hdp.hd_irq[hdp.hd_nirqs] = irq;
    hdp.hd_nirqs++;
    }
    }
    return AE_OK;
    }
#[no_mangle]
unsafe extern "C" fn hpet_acpi_probe(pdev: *mut platform_device) -> c_int {
    static int hpet_acpi_probe(struct platform_device *pdev)
    {
    struct acpi_device *device;
    acpi_status result;
    struct hpet_data data;
    device = ACPI_COMPANION(&pdev.dev);
    if (!device)
    return -ENODEV;
    memset(&data, 0, sizeof(data));
    result =
    acpi_walk_resources(device.handle, METHOD_NAME__CRS,
    hpet_resources, &data);
    if (ACPI_FAILURE(result))
    return -ENODEV;
    if (!data.hd_address || !data.hd_nirqs) {
    if (data.hd_address)
    iounmap(data.hd_address);
    printk("%s: no address or irqs in _CRS\n", __func__);
    return -ENODEV;
    }
    return hpet_alloc(&data);
    }
    static const struct acpi_device_id hpet_device_ids[] = {
    {"PNP0103", 0},
    {"", 0},
    };
    static struct platform_driver hpet_acpi_driver = {
    .probe = hpet_acpi_probe,
    .driver = {
    .name = "hpet_acpi",
    .acpi_match_table = hpet_device_ids,
    },
    };
    let mut hpet_misc: static struct miscdevice = { HPET_MINOR, "hpet", &hpet_fops };
#[no_mangle]
unsafe extern "C" fn hpet_init() -> int __init {
    static int __init hpet_init(void)
    {
    int result;
    result = misc_register(&hpet_misc);
    if (result < 0)
    return -ENODEV;
    sysctl_header = register_sysctl("dev/hpet", hpet_table);
    result = platform_driver_register(&hpet_acpi_driver);
    if (result < 0) {
    unregister_sysctl_table(sysctl_header);
    misc_deregister(&hpet_misc);
    return result;
    }
    return 0;
    }
    device_initcall(hpet_init);
//
    MODULE_AUTHOR("Bob Picco <Robert.Picco@hp.com>");
    MODULE_LICENSE("GPL");
//
