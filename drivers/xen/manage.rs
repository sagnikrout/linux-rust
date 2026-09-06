//! Automatically rewritten from C to Rust
//! Source: drivers/xen/manage.c
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
// Handle extern requests for shutdown, reboot and sysrq
//

    enum shutdown_state {
    SHUTDOWN_INVALID = -1,
    SHUTDOWN_POWEROFF = 0,
    SHUTDOWN_SUSPEND = 2,
// Code 3 is SHUTDOWN_CRASH, which we don't use because the domain can only
    report a crash, not be instructed to crash!
    HALT is the same as POWEROFF, as far as we're concerned.  The tools use
    the distinction when we return the reason code to them.  */
    SHUTDOWN_HALT = 4,
    };
// Ignore multiple shutdown requests.
    let mut shutting_down: static enum shutdown_state = SHUTDOWN_INVALID;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct suspend_info {
    pub cancelled: c_int,
}

    static RAW_NOTIFIER_HEAD(xen_resume_notifier);
#[no_mangle]
pub unsafe extern "C" fn xen_resume_notifier_register(nb: *mut notifier_block) {
    void xen_resume_notifier_register(struct notifier_block *nb)
    {
    raw_notifier_chain_register(&xen_resume_notifier, nb);
    }
    EXPORT_SYMBOL_GPL(xen_resume_notifier_register);

#[no_mangle]
unsafe extern "C" fn xen_suspend(data: *mut c_void) -> c_int {
    static int xen_suspend(void *data)
    {
    struct suspend_info *si = data;
    int err;
    BUG_ON(!irqs_disabled());
    err = syscore_suspend();
    if (err) {
    pr_err("%s: system core suspend failed: %d\n", __func__, err);
    return err;
    }
    gnttab_suspend();
    xen_manage_runstate_time(-1);
    xen_arch_pre_suspend();
    si.cancelled = HYPERVISOR_suspend(xen_pv_domain()
    ? virt_to_gfn(xen_start_info)
    : 0);
    xen_arch_post_suspend(si.cancelled);
    xen_manage_runstate_time(si.cancelled ? 1 : 0);
    gnttab_resume();
    if (!si.cancelled) {
    xen_irq_resume();
    xen_timer_resume();
    }
    syscore_resume();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn do_suspend() {
    static void do_suspend(void)
    {
    int err;
    struct suspend_info si;
    shutting_down = SHUTDOWN_SUSPEND;
    if (!mutex_trylock(&system_transition_mutex))
    {
    pr_err("%s: failed to take system_transition_mutex\n", __func__);
    goto out;
    }
    err = freeze_processes();
    if (err) {
    pr_err("%s: freeze processes failed %d\n", __func__, err);
    goto out_unlock;
    }
    err = freeze_kernel_threads();
    if (err) {
    pr_err("%s: freeze kernel threads failed %d\n", __func__, err);
    goto out_thaw;
    }
    err = dpm_suspend_start(PMSG_FREEZE);
    if (err) {
    pr_err("%s: dpm_suspend_start %d\n", __func__, err);
    goto out_resume_end;
    }
    printk(KERN_DEBUG "suspending xenstore...\n");
    xs_suspend();
    err = dpm_suspend_end(PMSG_FREEZE);
    if (err) {
    pr_err("dpm_suspend_end failed: %d\n", err);
    si.cancelled = 0;
    goto out_resume;
    }
    xen_arch_suspend();
    si.cancelled = 1;
    err = stop_machine(xen_suspend, &si, cpumask_of(0));
// Resume console as early as possible.
    if (!si.cancelled)
    xen_console_resume();
    raw_notifier_call_chain(&xen_resume_notifier, 0, core::ptr::null_mut());
    xen_arch_resume();
    dpm_resume_start(si.cancelled ? PMSG_THAW : PMSG_RESTORE);
    if (err) {
    pr_err("failed to start xen_suspend: %d\n", err);
    si.cancelled = 1;
    }
    out_resume:
    if (!si.cancelled)
    xs_resume();
    else
    xs_suspend_cancel();
    out_resume_end:
    dpm_resume_end(si.cancelled ? PMSG_THAW : PMSG_RESTORE);
    out_thaw:
    thaw_processes();
    out_unlock:
    mutex_unlock(&system_transition_mutex);
    out:
    shutting_down = SHUTDOWN_INVALID;
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shutdown_handler {
pub const SHUTDOWN_CMD_SIZE: c_int = 11;
    pub command: [c_char; SHUTDOWN_CMD_SIZE],
    pub flag: bool,
    pub (*cb)(void): *mut c_void,
}

#[no_mangle]
unsafe extern "C" fn poweroff_nb(cb: *mut notifier_block, code: c_ulong, unused: *mut c_void) -> c_int {
    static int poweroff_nb(struct notifier_block *cb, unsigned long code, void *unused)
    {
    switch (code) {
    case SYS_DOWN:
    case SYS_HALT:
    case SYS_POWER_OFF:
    shutting_down = SHUTDOWN_POWEROFF;
    break;
    default:
    break;
    }
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn do_poweroff() {
    static void do_poweroff(void)
    {
    switch (system_state) {
    case SYSTEM_BOOTING:
    case SYSTEM_SCHEDULING:
    orderly_poweroff(true);
    break;
    case SYSTEM_RUNNING:
    orderly_poweroff(false);
    break;
    default:
// Don't do it when we are halting/rebooting.
    pr_info("Ignoring Xen toolstack shutdown.\n");
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn do_reboot() {
    static void do_reboot(void)
    {
    shutting_down = SHUTDOWN_POWEROFF; /* ? */
    orderly_reboot();
    }
    static const struct shutdown_handler shutdown_handlers[] = {
    { "poweroff",	true,	do_poweroff },
    { "halt",	false,	do_poweroff },
    { "reboot",	true,	do_reboot   },

    { "suspend",	true,	do_suspend  },

    };
    static void shutdown_handler(struct xenbus_watch *watch,
    const char *path, const char *token)
    {
    char *str;
    struct xenbus_transaction xbt;
    int err;
    int idx;
    if (shutting_down != SHUTDOWN_INVALID)
    return;
    again:
    err = xenbus_transaction_start(&xbt);
    if (err)
    return;
    str = (char *)xenbus_read(xbt, "control", "shutdown", core::ptr::null_mut());
// Ignore read errors and empty reads.
    if (XENBUS_IS_ERR_READ(str)) {
    xenbus_transaction_end(xbt, 1);
    return;
    }
    for (idx = 0; idx < ARRAY_SIZE(shutdown_handlers); idx++) {
    if (strcmp(str, shutdown_handlers[idx].command) == 0)
    break;
    }
// Only acknowledge commands which we are prepared to handle.
    if (idx < ARRAY_SIZE(shutdown_handlers))
    xenbus_write(xbt, "control", "shutdown", "");
    err = xenbus_transaction_end(xbt, 0);
    if (err == -EAGAIN) {
    kfree(str);
    goto again;
    }
    if (idx < ARRAY_SIZE(shutdown_handlers)) {
    shutdown_handlers[idx].cb();
    } else {
    pr_info("Ignoring shutdown request: %s\n", str);
    shutting_down = SHUTDOWN_INVALID;
    }
    kfree(str);
    }

    static void sysrq_handler(struct xenbus_watch *watch, const char *path,
    const char *token)
    {
    let mut sysrq_key: c_char = '\0';
    struct xenbus_transaction xbt;
    int err;
    again:
    err = xenbus_transaction_start(&xbt);
    if (err)
    return;
    err = xenbus_scanf(xbt, "control", "sysrq", "%c", &sysrq_key);
    if (err < 0) {
//
// The Xenstore watch fires directly after registering it and
// after a suspend/resume cycle. So ENOENT is no error but
// might happen in those cases. ERANGE is observed when we get
// an empty value (''), this happens when we acknowledge the
// request by writing '\0' below.
//
    if (err != -ENOENT && err != -ERANGE)
    pr_err("Error %d reading sysrq code in control/sysrq\n",
    err);
    xenbus_transaction_end(xbt, 1);
    return;
    }
    if (sysrq_key != '\0') {
    err = xenbus_printf(xbt, "control", "sysrq", "%c", '\0');
    if (err) {
    pr_err("%s: Error %d writing sysrq in control/sysrq\n",
    __func__, err);
    xenbus_transaction_end(xbt, 1);
    return;
    }
    }
    err = xenbus_transaction_end(xbt, 0);
    if (err == -EAGAIN)
    goto again;
    if (sysrq_key != '\0')
    handle_sysrq(sysrq_key);
    }
    static struct xenbus_watch sysrq_watch = {
    .node = "control/sysrq",
    .callback = sysrq_handler
    };

    static struct xenbus_watch shutdown_watch = {
    .node = "control/shutdown",
    .callback = shutdown_handler
    };
    static struct notifier_block xen_reboot_nb = {
    .notifier_call = poweroff_nb,
    };
#[no_mangle]
unsafe extern "C" fn setup_shutdown_watcher() -> c_int {
    static int setup_shutdown_watcher(void)
    {
    int err;
    int idx;

    char node[FEATURE_PATH_SIZE];
    err = register_xenbus_watch(&shutdown_watch);
    if (err) {
    pr_err("Failed to set shutdown watcher\n");
    return err;
    }

    err = register_xenbus_watch(&sysrq_watch);
    if (err) {
    pr_err("Failed to set sysrq watcher\n");
    goto err_unregister_shutdown;
    }

    for (idx = 0; idx < ARRAY_SIZE(shutdown_handlers); idx++) {
    if (!shutdown_handlers[idx].flag)
    continue;
    snprintf(node, FEATURE_PATH_SIZE, "feature-%s",
    shutdown_handlers[idx].command);
    err = xenbus_printf(XBT_NIL, "control", node, "%u", 1);
    if (err) {
    pr_err("%s: Error %d writing %s\n", __func__,
    err, node);
    goto err_remove_features;
    }
    }
    return 0;
    err_remove_features:
    while (--idx >= 0) {
    if (!shutdown_handlers[idx].flag)
    continue;
    snprintf(node, FEATURE_PATH_SIZE, "feature-%s",
    shutdown_handlers[idx].command);
    xenbus_rm(XBT_NIL, "control", node);
    }

    unregister_xenbus_watch(&sysrq_watch);
    err_unregister_shutdown:

    unregister_xenbus_watch(&shutdown_watch);
    return err;
    }
    static int shutdown_event(struct notifier_block *notifier,
    unsigned long event,
    void *data)
    {
    setup_shutdown_watcher();
    return NOTIFY_DONE;
    }
#[no_mangle]
pub unsafe extern "C" fn xen_setup_shutdown_event() -> c_int {
    int xen_setup_shutdown_event(void)
    {
    static struct notifier_block xenstore_notifier = {
    .notifier_call = shutdown_event
    };
    if (!xen_domain())
    return -ENODEV;
    register_xenstore_notifier(&xenstore_notifier);
    register_reboot_notifier(&xen_reboot_nb);
    return 0;
    }
    EXPORT_SYMBOL_GPL(xen_setup_shutdown_event);
    subsys_initcall(xen_setup_shutdown_event);
