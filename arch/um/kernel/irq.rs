//! Automatically rewritten from C to Rust
//! Source: arch/um/kernel/irq.c
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
// Copyright (C) 2017 - Cambridge Greys Ltd
// Copyright (C) 2011 - 2014 Cisco Systems Inc
// Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
// Derived (i.e. mostly copied) from arch/i386/kernel/irq.c:
// Copyright (C) 1992, 1998 Linus Torvalds, Ingo Molnar
//

    DEFINE_PER_CPU_SHARED_ALIGNED(irq_cpustat_t, irq_stat);

// When epoll triggers we do not know why it did so
// we can also have different IRQs for read and write.
// This is why we keep a small irq_reg array for each fd -
// one entry per IRQ type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_reg {
    pub id: *mut c_void,
    pub irq: c_int,
// it's cheaper to store this than to query it
    pub events: c_int,
    pub active: bool,
    pub pending: bool,
    pub wakeup: bool,

    pub pending_event: bool,
    void (*timetravel_handler)(int, int, void *,
    pub ): *mut time_travel_event,
    pub event: time_travel_event,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_entry {
    pub list: list_head,
    pub fd: c_int,
    pub reg: [irq_reg; NUM_IRQ_TYPES],
    pub suspended: bool,
    pub sigio_workaround: bool,
}

    static DEFINE_RAW_SPINLOCK(irq_lock);
    static LIST_HEAD(active_fds);
    static DECLARE_BITMAP(irqs_allocated, UM_LAST_SIGNAL_IRQ);
    static bool irqs_suspended;

    static bool irqs_pending;

#[no_mangle]
unsafe extern "C" fn irq_io_loop(irq: *mut irq_reg, regs: *mut uml_pt_regs) {
    static void irq_io_loop(struct irq_reg *irq, struct uml_pt_regs *regs)
    {
//
// irq->active guards against reentry
// irq->pending accumulates pending requests
// if pending is raised the irq_handler is re-run
// until pending is cleared
//
    if (irq.active) {
    irq.active = false;
    do {
    irq.pending = false;
    do_IRQ(irq.irq, regs);
    } while (irq.pending);
    irq.active = true;
    } else {
    irq.pending = true;
    }
    }

#[no_mangle]
unsafe extern "C" fn irq_event_handler(ev: *mut time_travel_event) {
    static void irq_event_handler(struct time_travel_event *ev)
    {
    struct irq_reg *reg = container_of(ev, struct irq_reg, event);
// do nothing if suspended; just cause a wakeup and mark as pending
    if (irqs_suspended) {
    irqs_pending = true;
    reg.pending_event = true;
    return;
    }
    generic_handle_irq(reg.irq);
    }
    static bool irq_do_timetravel_handler(struct irq_entry *entry,
    enum um_irq_type t)
    {
    struct irq_reg *reg = &entry.reg[t];
    if (!reg.timetravel_handler)
    return false;
//
// Handle all messages - we might get multiple even while
// interrupts are already suspended, due to suspend order
// etc. Note that time_travel_add_irq_event() will not add
// an event twice, if it's pending already "first wins".
//
    reg.timetravel_handler(reg.irq, entry.fd, reg.id, &reg.event);
    if (!reg.event.pending)
    return false;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn irq_do_pending_events(timetravel_handlers_only: bool) {
    static void irq_do_pending_events(bool timetravel_handlers_only)
    {
    struct irq_entry *entry;
    if (!irqs_pending || timetravel_handlers_only)
    return;
    irqs_pending = false;
    list_for_each_entry(entry, &active_fds, list) {
    enum um_irq_type t;
    for (t = 0; t < NUM_IRQ_TYPES; t++) {
    struct irq_reg *reg = &entry.reg[t];
//
// Any timetravel_handler was invoked already, just
// directly run the IRQ.
//
    if (reg.pending_event) {
    irq_enter();
    generic_handle_irq(reg.irq);
    irq_exit();
    reg.pending_event = false;
    }
    }
    }
    }

    static bool irq_do_timetravel_handler(struct irq_entry *entry,
    enum um_irq_type t)
    {
    return false;
    }
#[no_mangle]
unsafe extern "C" fn irq_do_pending_events(timetravel_handlers_only: bool) {
    static void irq_do_pending_events(bool timetravel_handlers_only)
    {
    }

    static void sigio_reg_handler(int idx, struct irq_entry *entry, enum um_irq_type t,
    struct uml_pt_regs *regs,
    bool timetravel_handlers_only)
    {
    struct irq_reg *reg = &entry.reg[t];
    if (!reg.events)
    return;
    if (os_epoll_triggered(idx, reg.events) <= 0)
    return;
    if (irq_do_timetravel_handler(entry, t))
    return;
//
// If we're called to only run time-travel handlers then don't
// actually proceed but mark sigio as pending (if applicable).
// For suspend/resume, timetravel_handlers_only may be true
// despite time-travel not being configured and used.
//
    if (timetravel_handlers_only) {

    reg.pending_event = true;
    irqs_pending = true;
    mark_sigio_pending();

    return;
    }
    irq_io_loop(reg, regs);
    }
    static void _sigio_handler(struct uml_pt_regs *regs,
    bool timetravel_handlers_only)
    {
    struct irq_entry *irq_entry;
    int n, i;
    if (timetravel_handlers_only && !um_irq_timetravel_handler_used())
    return;
// Flush out pending events that were ignored due to time-travel.
    if (!irqs_suspended)
    irq_do_pending_events(timetravel_handlers_only);
    while (1) {
// This is now lockless - epoll keeps back-referencesto the irqs
// which have trigger it so there is no need to walk the irq
// list and lock it every time. We avoid locking by turning off
// IO for a specific fd by executing os_del_epoll_fd(fd) before
// we do any changes to the actual data structures
//
    n = os_waiting_for_events_epoll();
    if (n <= 0) {
    if (n == -EINTR)
    continue;
    else
    break;
    }
    for (i = 0; i < n ; i++) {
    enum um_irq_type t;
    irq_entry = os_epoll_get_data_pointer(i);
    for (t = 0; t < NUM_IRQ_TYPES; t++)
    sigio_reg_handler(i, irq_entry, t, regs,
    timetravel_handlers_only);
    }
    }
    if (!timetravel_handlers_only)
    free_irqs();
    }
    void sigio_handler(int sig, struct siginfo *unused_si, struct uml_pt_regs *regs,
    void *mc)
    {
    preempt_disable();
    _sigio_handler(regs, irqs_suspended);
    preempt_enable();
    }
    static struct irq_entry *get_irq_entry_by_fd(int fd)
    {
    struct irq_entry *walk;
    lockdep_assert_held(&irq_lock);
    list_for_each_entry(walk, &active_fds, list) {
    if (walk.fd == fd)
    return walk;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn remove_irq_entry(to_free: *mut irq_entry, remove: bool) {
    static void remove_irq_entry(struct irq_entry *to_free, bool remove)
    {
    if (!to_free)
    return;
    if (remove)
    os_del_epoll_fd(to_free.fd);
    list_del(&to_free.list);
    }
#[no_mangle]
unsafe extern "C" fn update_irq_entry(entry: *mut irq_entry) -> bool {
    static bool update_irq_entry(struct irq_entry *entry)
    {
    enum um_irq_type i;
    let mut events: c_int = 0;
    for (i = 0; i < NUM_IRQ_TYPES; i++)
    events |= entry.reg[i].events;
    if (events) {
// will modify (instead of add) if needed
    os_add_epoll_fd(events, entry.fd, entry);
    return true;
    }
    os_del_epoll_fd(entry.fd);
    return false;
    }
    static struct irq_entry *update_or_remove_irq_entry(struct irq_entry *entry)
    {
    if (update_irq_entry(entry))
    return core::ptr::null_mut();
    remove_irq_entry(entry, false);
    return entry;
    }
    static int activate_fd(int irq, int fd, enum um_irq_type type, void *dev_id,
    void (*timetravel_handler)(int, int, void *,
    struct time_travel_event *))
    {
    struct irq_entry *irq_entry, *to_free = core::ptr::null_mut();
    int err, events = os_event_mask(type);
    unsigned long flags;
    err = os_set_fd_async(fd);
    if (err < 0)
    goto out;
    raw_spin_lock_irqsave(&irq_lock, flags);
    irq_entry = get_irq_entry_by_fd(fd);
    if (irq_entry) {
    already:
// cannot register the same FD twice with the same type
    if (WARN_ON(irq_entry.reg[type].events)) {
    err = -EALREADY;
    goto out_unlock;
    }
// temporarily disable to avoid IRQ-side locking
    os_del_epoll_fd(fd);
    } else {
    struct irq_entry *new;
// don't restore interrupts
    raw_spin_unlock(&irq_lock);
    new = kzalloc_obj(*irq_entry, GFP_ATOMIC);
    if (!new) {
    local_irq_restore(flags);
    return -ENOMEM;
    }
    raw_spin_lock(&irq_lock);
    irq_entry = get_irq_entry_by_fd(fd);
    if (irq_entry) {
    to_free = new;
    goto already;
    }
    irq_entry = new;
    irq_entry.fd = fd;
    list_add_tail(&irq_entry.list, &active_fds);
    maybe_sigio_broken(fd);
    }
    irq_entry.reg[type].id = dev_id;
    irq_entry.reg[type].irq = irq;
    irq_entry.reg[type].active = true;
    irq_entry.reg[type].events = events;

    if (um_irq_timetravel_handler_used()) {
    irq_entry.reg[type].timetravel_handler = timetravel_handler;
    irq_entry.reg[type].event.fn = irq_event_handler;
    }

    WARN_ON(!update_irq_entry(irq_entry));
    err = 0;
    out_unlock:
    raw_spin_unlock_irqrestore(&irq_lock, flags);
    out:
    kfree(to_free);
    return err;
    }
//
// Remove the entry or entries for a specific FD, if you
// don't want to remove all the possible entries then use
// um_free_irq() or deactivate_fd() instead.
//
#[no_mangle]
pub unsafe extern "C" fn free_irq_by_fd(fd: c_int) {
    void free_irq_by_fd(int fd)
    {
    struct irq_entry *to_free;
    unsigned long flags;
    raw_spin_lock_irqsave(&irq_lock, flags);
    to_free = get_irq_entry_by_fd(fd);
    remove_irq_entry(to_free, true);
    raw_spin_unlock_irqrestore(&irq_lock, flags);
    kfree(to_free);
    }
    EXPORT_SYMBOL(free_irq_by_fd);
#[no_mangle]
unsafe extern "C" fn free_irq_by_irq_and_dev(irq: c_uint, dev: *mut c_void) {
    static void free_irq_by_irq_and_dev(unsigned int irq, void *dev)
    {
    struct irq_entry *entry, *to_free = core::ptr::null_mut();
    unsigned long flags;
    raw_spin_lock_irqsave(&irq_lock, flags);
    list_for_each_entry(entry, &active_fds, list) {
    enum um_irq_type i;
    for (i = 0; i < NUM_IRQ_TYPES; i++) {
    struct irq_reg *reg = &entry.reg[i];
    if (!reg.events)
    continue;
    if (reg.irq != irq)
    continue;
    if (reg.id != dev)
    continue;
    os_del_epoll_fd(entry.fd);
    reg.events = 0;
    to_free = update_or_remove_irq_entry(entry);
    goto out;
    }
    }
    out:
    raw_spin_unlock_irqrestore(&irq_lock, flags);
    kfree(to_free);
    }
#[no_mangle]
pub unsafe extern "C" fn deactivate_fd(fd: c_int, irqnum: c_int) {
    void deactivate_fd(int fd, int irqnum)
    {
    struct irq_entry *entry;
    unsigned long flags;
    enum um_irq_type i;
    os_del_epoll_fd(fd);
    raw_spin_lock_irqsave(&irq_lock, flags);
    entry = get_irq_entry_by_fd(fd);
    if (!entry)
    goto out;
    for (i = 0; i < NUM_IRQ_TYPES; i++) {
    if (!entry.reg[i].events)
    continue;
    if (entry.reg[i].irq == irqnum)
    entry.reg[i].events = 0;
    }
    entry = update_or_remove_irq_entry(entry);
    out:
    raw_spin_unlock_irqrestore(&irq_lock, flags);
    kfree(entry);
    ignore_sigio_fd(fd);
    }
    EXPORT_SYMBOL(deactivate_fd);
//
// Called just before shutdown in order to provide a clean exec
// environment in case the system is rebooting.  No locking because
// that would cause a pointless shutdown hang if something hadn't
// released the lock.
//
#[no_mangle]
pub unsafe extern "C" fn deactivate_all_fds() -> c_int {
    int deactivate_all_fds(void)
    {
    struct irq_entry *entry;
// Stop IO. The IRQ loop has no lock so this is our
// only way of making sure we are safe to dispose
// of all IRQ handlers
//
    os_set_ioignore();
// we can no longer call kfree() here so just deactivate
    list_for_each_entry(entry, &active_fds, list)
    os_del_epoll_fd(entry.fd);
    os_close_epoll_fd();
    return 0;
    }
//
// do_IRQ handles all normal device IRQs (the special
// SMP cross-CPU interrupts have their own specific
// handlers).
//
#[no_mangle]
pub unsafe extern "C" fn do_IRQ(irq: c_int, regs: *mut uml_pt_regs) -> c_uint {
    unsigned int do_IRQ(int irq, struct uml_pt_regs *regs)
    {
    struct pt_regs *old_regs = set_irq_regs((struct pt_regs *)regs);
    irq_enter();
    generic_handle_irq(irq);
    irq_exit();
    set_irq_regs(old_regs);
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn um_free_irq(irq: c_int, dev: *mut c_void) {
    void um_free_irq(int irq, void *dev)
    {
    if (WARN(irq < 0 || irq > UM_LAST_SIGNAL_IRQ,
    "freeing invalid irq %d", irq))
    return;
    free_irq_by_irq_and_dev(irq, dev);
    free_irq(irq, dev);
    clear_bit(irq, irqs_allocated);
    }
    EXPORT_SYMBOL(um_free_irq);
    static int
    _um_request_irq(int irq, int fd, enum um_irq_type type,
    irq_handler_t handler, unsigned long irqflags,
    const char *devname, void *dev_id,
    void (*timetravel_handler)(int, int, void *,
    struct time_travel_event *))
    {
    int err;
    if (irq == UM_IRQ_ALLOC) {
    int i;
    for (i = UM_FIRST_DYN_IRQ; i < NR_IRQS; i++) {
    if (!test_and_set_bit(i, irqs_allocated)) {
    irq = i;
    break;
    }
    }
    }
    if (irq < 0)
    return -ENOSPC;
    if (fd != -1) {
    err = activate_fd(irq, fd, type, dev_id, timetravel_handler);
    if (err)
    goto error;
    }
    err = request_irq(irq, handler, irqflags, devname, dev_id);
    if (err < 0)
    goto error;
    return irq;
    error:
    clear_bit(irq, irqs_allocated);
    return err;
    }
    int um_request_irq(int irq, int fd, enum um_irq_type type,
    irq_handler_t handler, unsigned long irqflags,
    const char *devname, void *dev_id)
    {
    return _um_request_irq(irq, fd, type, handler, irqflags,
    devname, dev_id, core::ptr::null_mut());
    }
    EXPORT_SYMBOL(um_request_irq);

    int um_request_irq_tt(int irq, int fd, enum um_irq_type type,
    irq_handler_t handler, unsigned long irqflags,
    const char *devname, void *dev_id,
    void (*timetravel_handler)(int, int, void *,
    struct time_travel_event *))
    {
    return _um_request_irq(irq, fd, type, handler, irqflags,
    devname, dev_id, timetravel_handler);
    }
    EXPORT_SYMBOL(um_request_irq_tt);
#[no_mangle]
pub unsafe extern "C" fn sigio_run_timetravel_handlers() {
    void sigio_run_timetravel_handlers(void)
    {
    _sigio_handler(core::ptr::null_mut(), true);
    }

#[no_mangle]
pub unsafe extern "C" fn um_irqs_suspend() {
    void um_irqs_suspend(void)
    {
    struct irq_entry *entry;
    unsigned long flags;
    irqs_suspended = true;
    raw_spin_lock_irqsave(&irq_lock, flags);
    list_for_each_entry(entry, &active_fds, list) {
    enum um_irq_type t;
    let mut clear: bool = true;
    for (t = 0; t < NUM_IRQ_TYPES; t++) {
    if (!entry.reg[t].events)
    continue;
//
// For the SIGIO_WRITE_IRQ, which is used to handle the
// SIGIO workaround thread, we need special handling:
// enable wake for it itself, but below we tell it about
// any FDs that should be suspended.
//
    if (entry.reg[t].wakeup ||
    entry.reg[t].irq == SIGIO_WRITE_IRQ

    || entry.reg[t].timetravel_handler

    ) {
    clear = false;
    break;
    }
    }
    if (clear) {
    entry.suspended = true;
    os_clear_fd_async(entry.fd);
    entry.sigio_workaround =
    !__ignore_sigio_fd(entry.fd);
    }
    }
    raw_spin_unlock_irqrestore(&irq_lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn um_irqs_resume() {
    void um_irqs_resume(void)
    {
    struct irq_entry *entry;
    unsigned long flags;
    raw_spin_lock_irqsave(&irq_lock, flags);
    list_for_each_entry(entry, &active_fds, list) {
    if (entry.suspended) {
    let mut err: c_int = os_set_fd_async(entry.fd);
    WARN(err < 0, "os_set_fd_async returned %d\n", err);
    entry.suspended = false;
    if (entry.sigio_workaround) {
    err = __add_sigio_fd(entry.fd);
    WARN(err < 0, "add_sigio_returned %d\n", err);
    }
    }
    }
    raw_spin_unlock_irqrestore(&irq_lock, flags);
    irqs_suspended = false;
    send_sigio_to_self();
    }
#[no_mangle]
unsafe extern "C" fn normal_irq_set_wake(d: *mut irq_data, on: c_uint) -> c_int {
    static int normal_irq_set_wake(struct irq_data *d, unsigned int on)
    {
    struct irq_entry *entry;
    unsigned long flags;
    raw_spin_lock_irqsave(&irq_lock, flags);
    list_for_each_entry(entry, &active_fds, list) {
    enum um_irq_type t;
    for (t = 0; t < NUM_IRQ_TYPES; t++) {
    if (!entry.reg[t].events)
    continue;
    if (entry.reg[t].irq != d.irq)
    continue;
    entry.reg[t].wakeup = on;
    goto unlock;
    }
    }
    unlock:
    raw_spin_unlock_irqrestore(&irq_lock, flags);
    return 0;
    }

//
// irq_chip must define at least enable/disable and ack when
// the edge handler is used.
//
#[no_mangle]
unsafe extern "C" fn dummy(d: *mut irq_data) {
    static void dummy(struct irq_data *d)
    {
    }
// This is used for everything other than the timer.
    static struct irq_chip normal_irq_type = {
    .name = "SIGIO",
    .irq_disable = dummy,
    .irq_enable = dummy,
    .irq_ack = dummy,
    .irq_mask = dummy,
    .irq_unmask = dummy,
    .irq_set_wake = normal_irq_set_wake,
    };
    static struct irq_chip alarm_irq_type = {
    .name = "SIGALRM",
    .irq_disable = dummy,
    .irq_enable = dummy,
    .irq_ack = dummy,
    .irq_mask = dummy,
    .irq_unmask = dummy,
    };
#[no_mangle]
pub unsafe extern "C" fn init_IRQ() -> void __init {
    void __init init_IRQ(void)
    {
    int i;
    irq_set_chip_and_handler(TIMER_IRQ, &alarm_irq_type, handle_percpu_irq);
    for (i = 1; i < UM_LAST_SIGNAL_IRQ; i++)
    irq_set_chip_and_handler(i, &normal_irq_type, handle_edge_irq);
// Initialize EPOLL Loop
    os_setup_epoll();
    }
#[no_mangle]
pub unsafe extern "C" fn arch_probe_nr_irqs() -> int __init {
    int __init arch_probe_nr_irqs(void)
    {
    return NR_IRQS;
    }
    void sigchld_handler(int sig, struct siginfo *unused_si,
    struct uml_pt_regs *regs, void *mc)
    {
    do_IRQ(SIGCHLD_IRQ, regs);
    }
//
// /proc/interrupts printing for arch specific interrupts
//
#[no_mangle]
pub unsafe extern "C" fn arch_show_interrupts(p: *mut seq_file, prec: c_int) -> c_int {
    int arch_show_interrupts(struct seq_file *p, int prec)
    {

    int cpu;
    seq_printf(p, "%*s: ", prec, "RES");
    for_each_online_cpu(cpu)
    seq_printf(p, "%10u ", irq_stats(cpu).irq_resched_count);
    seq_puts(p, " Rescheduling interrupts\n");
    seq_printf(p, "%*s: ", prec, "CAL");
    for_each_online_cpu(cpu)
    seq_printf(p, "%10u ", irq_stats(cpu).irq_call_count);
    seq_puts(p, " Function call interrupts\n");

    return 0;
    }
