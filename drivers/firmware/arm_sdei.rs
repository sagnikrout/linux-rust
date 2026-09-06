//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/arm_sdei.c
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
// Copyright (C) 2017 Arm Ltd.

//
// The call to use to reach the firmware.
//
    static asmlinkage void (*sdei_firmware_call)(unsigned long function_id,
    unsigned long arg0, unsigned long arg1,
    unsigned long arg2, unsigned long arg3,
    unsigned long arg4, struct arm_smccc_res *res);
// entry point from firmware to arch asm code
    static unsigned long sdei_entry_point;
    static int sdei_hp_state;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdei_event {
// These three are protected by the sdei_list_lock
    pub list: list_head,
    pub reregister: bool,
    pub reenable: bool,
    pub event_num: u32,
    pub type: u8,
    pub priority: u8,
// This pointer is handed to firmware as the event argument.
    union {
// Shared events
    pub registered: *mut sdei_registered_event,
// CPU private events
    pub private_registered: *mut sdei_registered_event __percpu,
}

    };
// Take the mutex for any API call or modification. Take the mutex first.
    static DEFINE_MUTEX(sdei_events_lock);
// and then hold this when modifying the list
    static DEFINE_SPINLOCK(sdei_list_lock);
    static LIST_HEAD(sdei_list);
// Private events are registered/enabled via IPI passing one of these
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdei_crosscall_args {
    pub event: *mut sdei_event,
    pub errors: core::sync::atomic::AtomicI32,
    pub first_error: c_int,
}

    do {					\
    arg.event = event;		\
    arg.first_error = 0;		\
    atomic_set(&arg.errors, 0);	\
    } while (0)
    static inline int sdei_do_local_call(smp_call_func_t fn,
    struct sdei_event *event)
    {
    struct sdei_crosscall_args arg;
    CROSSCALL_INIT(arg, event);
    fn(&arg);
    return arg.first_error;
    }
    static inline int sdei_do_cross_call(smp_call_func_t fn,
    struct sdei_event *event)
    {
    struct sdei_crosscall_args arg;
    CROSSCALL_INIT(arg, event);
    on_each_cpu(fn, &arg, true);
    return arg.first_error;
    }
    static inline void
    sdei_cross_call_return(struct sdei_crosscall_args *arg, int err)
    {
    if (err && (atomic_inc_return(&arg.errors) == 1))
    arg.first_error = err;
    }
#[no_mangle]
unsafe extern "C" fn sdei_to_linux_errno(sdei_err: c_ulong) -> c_int {
    static int sdei_to_linux_errno(unsigned long sdei_err)
    {
    switch (sdei_err) {
    case SDEI_NOT_SUPPORTED:
    return -EOPNOTSUPP;
    case SDEI_INVALID_PARAMETERS:
    return -EINVAL;
    case SDEI_DENIED:
    return -EPERM;
    case SDEI_PENDING:
    return -EINPROGRESS;
    case SDEI_OUT_OF_RESOURCE:
    return -ENOMEM;
    }
    return 0;
    }
    static int invoke_sdei_fn(unsigned long function_id, unsigned long arg0,
    unsigned long arg1, unsigned long arg2,
    unsigned long arg3, unsigned long arg4,
    u64 *result)
    {
    int err;
    struct arm_smccc_res res;
    if (sdei_firmware_call) {
    sdei_firmware_call(function_id, arg0, arg1, arg2, arg3, arg4,
    &res);
    err = sdei_to_linux_errno(res.a0);
    } else {
//
// !sdei_firmware_call means we failed to probe or called
// sdei_mark_interface_broken(). -EIO is not an error returned
// by sdei_to_linux_errno() and is used to suppress messages
// from this driver.
//
    err = -EIO;
    res.a0 = SDEI_NOT_SUPPORTED;
    }
    if (result)
// result = res.a0;
    return err;
    }
    NOKPROBE_SYMBOL(invoke_sdei_fn);
    static struct sdei_event *sdei_event_find(u32 event_num)
    {
    struct sdei_event *e, *found = core::ptr::null_mut();
    lockdep_assert_held(&sdei_events_lock);
    spin_lock(&sdei_list_lock);
    list_for_each_entry(e, &sdei_list, list) {
    if (e.event_num == event_num) {
    found = e;
    break;
    }
    }
    spin_unlock(&sdei_list_lock);
    return found;
    }
#[no_mangle]
pub unsafe extern "C" fn sdei_api_event_context(query: u32, result: *mut u64) -> c_int {
    int sdei_api_event_context(u32 query, u64 *result)
    {
    return invoke_sdei_fn(SDEI_1_0_FN_SDEI_EVENT_CONTEXT, query, 0, 0, 0, 0,
    result);
    }
    NOKPROBE_SYMBOL(sdei_api_event_context);
#[no_mangle]
unsafe extern "C" fn sdei_api_event_get_info(event: u32, info: u32, result: *mut u64) -> c_int {
    static int sdei_api_event_get_info(u32 event, u32 info, u64 *result)
    {
    return invoke_sdei_fn(SDEI_1_0_FN_SDEI_EVENT_GET_INFO, event, info, 0,
    0, 0, result);
    }
    static struct sdei_event *sdei_event_create(u32 event_num,
    sdei_event_callback *cb,
    void *cb_arg)
    {
    int err;
    u64 result;
    struct sdei_event *event;
    struct sdei_registered_event *reg;
    lockdep_assert_held(&sdei_events_lock);
    event = kzalloc_obj(*event);
    if (!event) {
    err = -ENOMEM;
    goto fail;
    }
    INIT_LIST_HEAD(&event.list);
    event.event_num = event_num;
    err = sdei_api_event_get_info(event_num, SDEI_EVENT_INFO_EV_PRIORITY,
    &result);
    if (err)
    goto fail;
    event.priority = result;
    err = sdei_api_event_get_info(event_num, SDEI_EVENT_INFO_EV_TYPE,
    &result);
    if (err)
    goto fail;
    event.type = result;
    if (event.type == SDEI_EVENT_TYPE_SHARED) {
    reg = kzalloc_obj(*reg);
    if (!reg) {
    err = -ENOMEM;
    goto fail;
    }
    reg.event_num = event.event_num;
    reg.priority = event.priority;
    reg.callback = cb;
    reg.callback_arg = cb_arg;
    event.registered = reg;
    } else {
    int cpu;
    struct sdei_registered_event __percpu *regs;
    regs = alloc_percpu(struct sdei_registered_event);
    if (!regs) {
    err = -ENOMEM;
    goto fail;
    }
    for_each_possible_cpu(cpu) {
    reg = per_cpu_ptr(regs, cpu);
    reg.event_num = event.event_num;
    reg.priority = event.priority;
    reg.callback = cb;
    reg.callback_arg = cb_arg;
    }
    event.private_registered = regs;
    }
    spin_lock(&sdei_list_lock);
    list_add(&event.list, &sdei_list);
    spin_unlock(&sdei_list_lock);
    return event;
    fail:
    kfree(event);
    return ERR_PTR(err);
    }
#[no_mangle]
unsafe extern "C" fn sdei_event_destroy_llocked(event: *mut sdei_event) {
    static void sdei_event_destroy_llocked(struct sdei_event *event)
    {
    lockdep_assert_held(&sdei_events_lock);
    lockdep_assert_held(&sdei_list_lock);
    list_del(&event.list);
    if (event.type == SDEI_EVENT_TYPE_SHARED)
    kfree(event.registered);
    else
    free_percpu(event.private_registered);
    kfree(event);
    }
#[no_mangle]
unsafe extern "C" fn sdei_event_destroy(event: *mut sdei_event) {
    static void sdei_event_destroy(struct sdei_event *event)
    {
    spin_lock(&sdei_list_lock);
    sdei_event_destroy_llocked(event);
    spin_unlock(&sdei_list_lock);
    }
#[no_mangle]
unsafe extern "C" fn sdei_api_get_version(version: *mut u64) -> c_int {
    static int sdei_api_get_version(u64 *version)
    {
    return invoke_sdei_fn(SDEI_1_0_FN_SDEI_VERSION, 0, 0, 0, 0, 0, version);
    }
#[no_mangle]
pub unsafe extern "C" fn sdei_mask_local_cpu() -> c_int {
    int sdei_mask_local_cpu(void)
    {
    int err;
    err = invoke_sdei_fn(SDEI_1_0_FN_SDEI_PE_MASK, 0, 0, 0, 0, 0, core::ptr::null_mut());
    if (err && err != -EIO) {
    pr_warn_once("failed to mask CPU[%u]: %d\n",
    smp_processor_id(), err);
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn _ipi_mask_cpu(ignored: *mut c_void) {
    static void _ipi_mask_cpu(void *ignored)
    {
    WARN_ON_ONCE(preemptible());
    sdei_mask_local_cpu();
    }
#[no_mangle]
pub unsafe extern "C" fn sdei_unmask_local_cpu() -> c_int {
    int sdei_unmask_local_cpu(void)
    {
    int err;
    err = invoke_sdei_fn(SDEI_1_0_FN_SDEI_PE_UNMASK, 0, 0, 0, 0, 0, core::ptr::null_mut());
    if (err && err != -EIO) {
    pr_warn_once("failed to unmask CPU[%u]: %d\n",
    smp_processor_id(), err);
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn _ipi_unmask_cpu(ignored: *mut c_void) {
    static void _ipi_unmask_cpu(void *ignored)
    {
    WARN_ON_ONCE(preemptible());
    sdei_unmask_local_cpu();
    }
//
// Signal the software-signalled event (event 0) to @mpidr. Does nothing
// but the SMC -- no locks, no event lookup -- so it is safe from NMI
// crash context (e.g. the cross-CPU NMI service).
//
#[no_mangle]
pub unsafe extern "C" fn sdei_event_signal(event_num: u32, mpidr: u64) -> c_int {
    int sdei_event_signal(u32 event_num, u64 mpidr)
    {
    return invoke_sdei_fn(SDEI_1_0_FN_SDEI_EVENT_SIGNAL, event_num,
    mpidr, 0, 0, 0, core::ptr::null_mut());
    }
    NOKPROBE_SYMBOL(sdei_event_signal);
//
// Was SDEI firmware probed and is it usable?  Lets optional consumers skip
// registering an event -- and the warning a failed registration emits -- on
// systems with no SDEI.
//
#[no_mangle]
pub unsafe extern "C" fn sdei_is_present() -> bool {
    bool sdei_is_present(void)
    {
    return sdei_firmware_call;
    }
#[no_mangle]
unsafe extern "C" fn _ipi_private_reset(ignored: *mut c_void) {
    static void _ipi_private_reset(void *ignored)
    {
    int err;
    WARN_ON_ONCE(preemptible());
    err = invoke_sdei_fn(SDEI_1_0_FN_SDEI_PRIVATE_RESET, 0, 0, 0, 0, 0,
    core::ptr::null_mut());
    if (err && err != -EIO)
    pr_warn_once("failed to reset CPU[%u]: %d\n",
    smp_processor_id(), err);
    }
#[no_mangle]
unsafe extern "C" fn sdei_api_shared_reset() -> c_int {
    static int sdei_api_shared_reset(void)
    {
    return invoke_sdei_fn(SDEI_1_0_FN_SDEI_SHARED_RESET, 0, 0, 0, 0, 0,
    core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn sdei_mark_interface_broken() {
    static void sdei_mark_interface_broken(void)
    {
    pr_err("disabling SDEI firmware interface\n");
    on_each_cpu(&_ipi_mask_cpu, core::ptr::null_mut(), true);
    sdei_firmware_call = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn sdei_platform_reset() -> c_int {
    static int sdei_platform_reset(void)
    {
    int err;
    on_each_cpu(&_ipi_private_reset, core::ptr::null_mut(), true);
    err = sdei_api_shared_reset();
    if (err) {
    pr_err("Failed to reset platform: %d\n", err);
    sdei_mark_interface_broken();
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn sdei_api_event_enable(event_num: u32) -> c_int {
    static int sdei_api_event_enable(u32 event_num)
    {
    return invoke_sdei_fn(SDEI_1_0_FN_SDEI_EVENT_ENABLE, event_num, 0, 0, 0,
    0, core::ptr::null_mut());
    }
// Called directly by the hotplug callbacks
#[no_mangle]
unsafe extern "C" fn _local_event_enable(data: *mut c_void) {
    static void _local_event_enable(void *data)
    {
    int err;
    struct sdei_crosscall_args *arg = data;
    err = sdei_api_event_enable(arg.event.event_num);
    sdei_cross_call_return(arg, err);
    }
#[no_mangle]
pub unsafe extern "C" fn sdei_event_enable(event_num: u32) -> c_int {
    int sdei_event_enable(u32 event_num)
    {
    let mut err: c_int = -EINVAL;
    struct sdei_event *event;
    mutex_lock(&sdei_events_lock);
    event = sdei_event_find(event_num);
    if (!event) {
    mutex_unlock(&sdei_events_lock);
    return -ENOENT;
    }
    cpus_read_lock();
    if (event.type == SDEI_EVENT_TYPE_SHARED)
    err = sdei_api_event_enable(event.event_num);
    else
    err = sdei_do_cross_call(_local_event_enable, event);
    if (!err) {
    spin_lock(&sdei_list_lock);
    event.reenable = true;
    spin_unlock(&sdei_list_lock);
    }
    cpus_read_unlock();
    mutex_unlock(&sdei_events_lock);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn sdei_api_event_disable(event_num: u32) -> c_int {
    static int sdei_api_event_disable(u32 event_num)
    {
    return invoke_sdei_fn(SDEI_1_0_FN_SDEI_EVENT_DISABLE, event_num, 0, 0,
    0, 0, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn _ipi_event_disable(data: *mut c_void) {
    static void _ipi_event_disable(void *data)
    {
    int err;
    struct sdei_crosscall_args *arg = data;
    err = sdei_api_event_disable(arg.event.event_num);
    sdei_cross_call_return(arg, err);
    }
#[no_mangle]
pub unsafe extern "C" fn sdei_event_disable(event_num: u32) -> c_int {
    int sdei_event_disable(u32 event_num)
    {
    let mut err: c_int = -EINVAL;
    struct sdei_event *event;
    mutex_lock(&sdei_events_lock);
    event = sdei_event_find(event_num);
    if (!event) {
    mutex_unlock(&sdei_events_lock);
    return -ENOENT;
    }
    spin_lock(&sdei_list_lock);
    event.reenable = false;
    spin_unlock(&sdei_list_lock);
    if (event.type == SDEI_EVENT_TYPE_SHARED)
    err = sdei_api_event_disable(event.event_num);
    else
    err = sdei_do_cross_call(_ipi_event_disable, event);
    mutex_unlock(&sdei_events_lock);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn sdei_api_event_unregister(event_num: u32) -> c_int {
    static int sdei_api_event_unregister(u32 event_num)
    {
    return invoke_sdei_fn(SDEI_1_0_FN_SDEI_EVENT_UNREGISTER, event_num, 0,
    0, 0, 0, core::ptr::null_mut());
    }
// Called directly by the hotplug callbacks
#[no_mangle]
unsafe extern "C" fn _local_event_unregister(data: *mut c_void) {
    static void _local_event_unregister(void *data)
    {
    int err;
    struct sdei_crosscall_args *arg = data;
    err = sdei_api_event_unregister(arg.event.event_num);
    sdei_cross_call_return(arg, err);
    }
#[no_mangle]
pub unsafe extern "C" fn sdei_event_unregister(event_num: u32) -> c_int {
    int sdei_event_unregister(u32 event_num)
    {
    int err;
    struct sdei_event *event;
    WARN_ON(in_nmi());
    mutex_lock(&sdei_events_lock);
    event = sdei_event_find(event_num);
    if (!event) {
    pr_warn("Event %u not registered\n", event_num);
    err = -ENOENT;
    goto unlock;
    }
    spin_lock(&sdei_list_lock);
    event.reregister = false;
    event.reenable = false;
    spin_unlock(&sdei_list_lock);
    if (event.type == SDEI_EVENT_TYPE_SHARED)
    err = sdei_api_event_unregister(event.event_num);
    else
    err = sdei_do_cross_call(_local_event_unregister, event);
    if (err)
    goto unlock;
    sdei_event_destroy(event);
    unlock:
    mutex_unlock(&sdei_events_lock);
    return err;
    }
//
// unregister events, but don't destroy them as they are re-registered by
// sdei_reregister_shared().
//
#[no_mangle]
unsafe extern "C" fn sdei_unregister_shared() -> c_int {
    static int sdei_unregister_shared(void)
    {
    let mut err: c_int = 0;
    struct sdei_event *event;
    mutex_lock(&sdei_events_lock);
    spin_lock(&sdei_list_lock);
    list_for_each_entry(event, &sdei_list, list) {
    if (event.type != SDEI_EVENT_TYPE_SHARED)
    continue;
    err = sdei_api_event_unregister(event.event_num);
    if (err)
    break;
    }
    spin_unlock(&sdei_list_lock);
    mutex_unlock(&sdei_events_lock);
    return err;
    }
    static int sdei_api_event_register(u32 event_num, unsigned long entry_point,
    void *arg, u64 flags, u64 affinity)
    {
    return invoke_sdei_fn(SDEI_1_0_FN_SDEI_EVENT_REGISTER, event_num,
    (unsigned long)entry_point, (unsigned long)arg,
    flags, affinity, core::ptr::null_mut());
    }
// Called directly by the hotplug callbacks
#[no_mangle]
unsafe extern "C" fn _local_event_register(data: *mut c_void) {
    static void _local_event_register(void *data)
    {
    int err;
    struct sdei_registered_event *reg;
    struct sdei_crosscall_args *arg = data;
    reg = per_cpu_ptr(arg.event.private_registered, smp_processor_id());
    err = sdei_api_event_register(arg.event.event_num, sdei_entry_point,
    reg, 0, 0);
    sdei_cross_call_return(arg, err);
    }
#[no_mangle]
pub unsafe extern "C" fn sdei_event_register(event_num: u32, cb: *mut sdei_event_callback, arg: *mut c_void) -> c_int {
    int sdei_event_register(u32 event_num, sdei_event_callback *cb, void *arg)
    {
    int err;
    struct sdei_event *event;
    WARN_ON(in_nmi());
    mutex_lock(&sdei_events_lock);
    if (sdei_event_find(event_num)) {
    pr_warn("Event %u already registered\n", event_num);
    err = -EBUSY;
    goto unlock;
    }
    event = sdei_event_create(event_num, cb, arg);
    if (IS_ERR(event)) {
    err = PTR_ERR(event);
    pr_warn("Failed to create event %u: %d\n", event_num, err);
    goto unlock;
    }
    cpus_read_lock();
    if (event.type == SDEI_EVENT_TYPE_SHARED) {
    err = sdei_api_event_register(event.event_num,
    sdei_entry_point,
    event.registered,
    SDEI_EVENT_REGISTER_RM_ANY, 0);
    } else {
    err = sdei_do_cross_call(_local_event_register, event);
    if (err)
    sdei_do_cross_call(_local_event_unregister, event);
    }
    if (err) {
    sdei_event_destroy(event);
    pr_warn("Failed to register event %u: %d\n", event_num, err);
    goto cpu_unlock;
    }
    spin_lock(&sdei_list_lock);
    event.reregister = true;
    spin_unlock(&sdei_list_lock);
    cpu_unlock:
    cpus_read_unlock();
    unlock:
    mutex_unlock(&sdei_events_lock);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn sdei_reregister_shared() -> c_int {
    static int sdei_reregister_shared(void)
    {
    let mut err: c_int = 0;
    struct sdei_event *event;
    mutex_lock(&sdei_events_lock);
    spin_lock(&sdei_list_lock);
    list_for_each_entry(event, &sdei_list, list) {
    if (event.type != SDEI_EVENT_TYPE_SHARED)
    continue;
    if (event.reregister) {
    err = sdei_api_event_register(event.event_num,
    sdei_entry_point, event.registered,
    SDEI_EVENT_REGISTER_RM_ANY, 0);
    if (err) {
    pr_err("Failed to re-register event %u\n",
    event.event_num);
    sdei_event_destroy_llocked(event);
    break;
    }
    }
    if (event.reenable) {
    err = sdei_api_event_enable(event.event_num);
    if (err) {
    pr_err("Failed to re-enable event %u\n",
    event.event_num);
    break;
    }
    }
    }
    spin_unlock(&sdei_list_lock);
    mutex_unlock(&sdei_events_lock);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn sdei_cpuhp_down(cpu: c_uint) -> c_int {
    static int sdei_cpuhp_down(unsigned int cpu)
    {
    struct sdei_event *event;
    int err;
// un-register private events
    spin_lock(&sdei_list_lock);
    list_for_each_entry(event, &sdei_list, list) {
    if (event.type == SDEI_EVENT_TYPE_SHARED)
    continue;
    err = sdei_do_local_call(_local_event_unregister, event);
    if (err) {
    pr_err("Failed to unregister event %u: %d\n",
    event.event_num, err);
    }
    }
    spin_unlock(&sdei_list_lock);
    return sdei_mask_local_cpu();
    }
#[no_mangle]
unsafe extern "C" fn sdei_cpuhp_up(cpu: c_uint) -> c_int {
    static int sdei_cpuhp_up(unsigned int cpu)
    {
    struct sdei_event *event;
    int err;
// re-register/enable private events
    spin_lock(&sdei_list_lock);
    list_for_each_entry(event, &sdei_list, list) {
    if (event.type == SDEI_EVENT_TYPE_SHARED)
    continue;
    if (event.reregister) {
    err = sdei_do_local_call(_local_event_register, event);
    if (err) {
    pr_err("Failed to re-register event %u: %d\n",
    event.event_num, err);
    }
    }
    if (event.reenable) {
    err = sdei_do_local_call(_local_event_enable, event);
    if (err) {
    pr_err("Failed to re-enable event %u: %d\n",
    event.event_num, err);
    }
    }
    }
    spin_unlock(&sdei_list_lock);
    return sdei_unmask_local_cpu();
    }
// When entering idle, mask/unmask events for this cpu
    static int sdei_pm_notifier(struct notifier_block *nb, unsigned long action,
    void *data)
    {
    int rv;
    WARN_ON_ONCE(preemptible());
    switch (action) {
    case CPU_PM_ENTER:
    rv = sdei_mask_local_cpu();
    break;
    case CPU_PM_EXIT:
    case CPU_PM_ENTER_FAILED:
    rv = sdei_unmask_local_cpu();
    break;
    default:
    return NOTIFY_DONE;
    }
    if (rv)
    return notifier_from_errno(rv);
    return NOTIFY_OK;
    }
    static struct notifier_block sdei_pm_nb = {
    .notifier_call = sdei_pm_notifier,
    };
#[no_mangle]
unsafe extern "C" fn sdei_device_suspend(dev: *mut device) -> c_int {
    static int sdei_device_suspend(struct device *dev)
    {
    on_each_cpu(_ipi_mask_cpu, core::ptr::null_mut(), true);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sdei_device_resume(dev: *mut device) -> c_int {
    static int sdei_device_resume(struct device *dev)
    {
    on_each_cpu(_ipi_unmask_cpu, core::ptr::null_mut(), true);
    return 0;
    }
//
// We need all events to be reregistered when we resume from hibernate.
//
// The sequence is freeze->thaw. Reboot. freeze->restore. We unregister
// events during freeze, then re-register and re-enable them during thaw
// and restore.
//
#[no_mangle]
unsafe extern "C" fn sdei_device_freeze(dev: *mut device) -> c_int {
    static int sdei_device_freeze(struct device *dev)
    {
    int err;
// unregister private events
    cpuhp_remove_state(sdei_hp_state);
    err = sdei_unregister_shared();
    if (err)
    return err;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sdei_device_thaw(dev: *mut device) -> c_int {
    static int sdei_device_thaw(struct device *dev)
    {
    int err;
// re-register shared events
    err = sdei_reregister_shared();
    if (err) {
    pr_warn("Failed to re-register shared events...\n");
    sdei_mark_interface_broken();
    return err;
    }
    err = cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, "SDEI",
    &sdei_cpuhp_up, &sdei_cpuhp_down);
    if (err < 0) {
    pr_warn("Failed to re-register CPU hotplug notifier...\n");
    return err;
    }
    sdei_hp_state = err;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sdei_device_restore(dev: *mut device) -> c_int {
    static int sdei_device_restore(struct device *dev)
    {
    int err;
    err = sdei_platform_reset();
    if (err)
    return err;
    return sdei_device_thaw(dev);
    }
    static const struct dev_pm_ops sdei_pm_ops = {
    .suspend = sdei_device_suspend,
    .resume = sdei_device_resume,
    .freeze = sdei_device_freeze,
    .thaw = sdei_device_thaw,
    .restore = sdei_device_restore,
    };
//
// Mask all CPUs and unregister all events on panic, reboot or kexec.
//
    static int sdei_reboot_notifier(struct notifier_block *nb, unsigned long action,
    void *data)
    {
//
// We are going to reset the interface, after this there is no point
// doing work when we take CPUs offline.
//
    cpuhp_remove_state(sdei_hp_state);
    sdei_platform_reset();
    return NOTIFY_OK;
    }
    static struct notifier_block sdei_reboot_nb = {
    .notifier_call = sdei_reboot_notifier,
    };
    static void sdei_smccc_smc(unsigned long function_id,
    unsigned long arg0, unsigned long arg1,
    unsigned long arg2, unsigned long arg3,
    unsigned long arg4, struct arm_smccc_res *res)
    {
    arm_smccc_smc(function_id, arg0, arg1, arg2, arg3, arg4, 0, 0, res);
    }
    NOKPROBE_SYMBOL(sdei_smccc_smc);
    static void sdei_smccc_hvc(unsigned long function_id,
    unsigned long arg0, unsigned long arg1,
    unsigned long arg2, unsigned long arg3,
    unsigned long arg4, struct arm_smccc_res *res)
    {
    arm_smccc_hvc(function_id, arg0, arg1, arg2, arg3, arg4, 0, 0, res);
    }
    NOKPROBE_SYMBOL(sdei_smccc_hvc);
    int sdei_register_ghes(struct ghes *ghes, sdei_event_callback *normal_cb,
    sdei_event_callback *critical_cb)
    {
    int err;
    u64 result;
    u32 event_num;
    sdei_event_callback *cb;
    if (!IS_ENABLED(CONFIG_ACPI_APEI_GHES))
    return -EOPNOTSUPP;
    event_num = ghes.generic.notify.vector;
    if (event_num == 0) {
//
// Event 0 is reserved by the specification for
// SDEI_EVENT_SIGNAL.
//
    return -EINVAL;
    }
    err = sdei_api_event_get_info(event_num, SDEI_EVENT_INFO_EV_PRIORITY,
    &result);
    if (err)
    return err;
    if (result == SDEI_EVENT_PRIORITY_CRITICAL)
    cb = critical_cb;
    else
    cb = normal_cb;
    err = sdei_event_register(event_num, cb, ghes);
    if (!err)
    err = sdei_event_enable(event_num);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn sdei_unregister_ghes(ghes: *mut ghes) -> c_int {
    int sdei_unregister_ghes(struct ghes *ghes)
    {
    int i;
    int err;
    let mut event_num: u32 = ghes.generic.notify.vector;
    might_sleep();
    if (!IS_ENABLED(CONFIG_ACPI_APEI_GHES))
    return -EOPNOTSUPP;
//
// The event may be running on another CPU. Disable it
// to stop new events, then try to unregister a few times.
//
    err = sdei_event_disable(event_num);
    if (err)
    return err;
    for (i = 0; i < 3; i++) {
    err = sdei_event_unregister(event_num);
    if (err != -EINPROGRESS)
    break;
    schedule();
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn sdei_get_conduit(pdev: *mut platform_device) -> c_int {
    static int sdei_get_conduit(struct platform_device *pdev)
    {
    const char *method;
    struct device_node *np = pdev.dev.of_node;
    sdei_firmware_call = core::ptr::null_mut();
    if (np) {
    if (of_property_read_string(np, "method", &method)) {
    pr_warn("missing \"method\" property\n");
    return SMCCC_CONDUIT_NONE;
    }
    if (!strcmp("hvc", method)) {
    sdei_firmware_call = &sdei_smccc_hvc;
    return SMCCC_CONDUIT_HVC;
    } else if (!strcmp("smc", method)) {
    sdei_firmware_call = &sdei_smccc_smc;
    return SMCCC_CONDUIT_SMC;
    }
    pr_warn("invalid \"method\" property: %s\n", method);
    } else if (!acpi_disabled) {
    if (acpi_psci_use_hvc()) {
    sdei_firmware_call = &sdei_smccc_hvc;
    return SMCCC_CONDUIT_HVC;
    } else {
    sdei_firmware_call = &sdei_smccc_smc;
    return SMCCC_CONDUIT_SMC;
    }
    }
    return SMCCC_CONDUIT_NONE;
    }
#[no_mangle]
unsafe extern "C" fn sdei_probe(pdev: *mut platform_device) -> c_int {
    static int sdei_probe(struct platform_device *pdev)
    {
    int err;
    let mut ver: u64 = 0;
    int conduit;
    conduit = sdei_get_conduit(pdev);
    if (!sdei_firmware_call)
    return 0;
    err = sdei_api_get_version(&ver);
    if (err) {
    pr_err("Failed to get SDEI version: %d\n", err);
    sdei_mark_interface_broken();
    return err;
    }
    pr_info("SDEIv%d.%d (0x%x) detected in firmware.\n",
    (int)SDEI_VERSION_MAJOR(ver), (int)SDEI_VERSION_MINOR(ver),
    (int)SDEI_VERSION_VENDOR(ver));
    if (SDEI_VERSION_MAJOR(ver) != 1) {
    pr_warn("Conflicting SDEI version detected.\n");
    sdei_mark_interface_broken();
    return -EINVAL;
    }
    err = sdei_platform_reset();
    if (err)
    return err;
    sdei_entry_point = sdei_arch_get_entry_point(conduit);
    if (!sdei_entry_point) {
// Not supported due to hardware or boot configuration
    sdei_mark_interface_broken();
    return 0;
    }
    err = cpu_pm_register_notifier(&sdei_pm_nb);
    if (err) {
    pr_warn("Failed to register CPU PM notifier...\n");
    goto error;
    }
    err = register_reboot_notifier(&sdei_reboot_nb);
    if (err) {
    pr_warn("Failed to register reboot notifier...\n");
    goto remove_cpupm;
    }
    err = cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, "SDEI",
    &sdei_cpuhp_up, &sdei_cpuhp_down);
    if (err < 0) {
    pr_warn("Failed to register CPU hotplug notifier...\n");
    goto remove_reboot;
    }
    sdei_hp_state = err;
    return 0;
    remove_reboot:
    unregister_reboot_notifier(&sdei_reboot_nb);
    remove_cpupm:
    cpu_pm_unregister_notifier(&sdei_pm_nb);
    error:
    sdei_mark_interface_broken();
    return err;
    }
    static const struct of_device_id sdei_of_match[] = {
    { .compatible = "arm,sdei-1.0" },
    {}
    };
    static struct platform_driver sdei_driver = {
    .driver		= {
    .name			= "sdei",
    .pm			= &sdei_pm_ops,
    .of_match_table		= sdei_of_match,
    },
    .probe		= sdei_probe,
    };
#[no_mangle]
unsafe extern "C" fn sdei_present_acpi() -> bool __init {
    static bool __init sdei_present_acpi(void)
    {
    acpi_status status;
    struct acpi_table_header *sdei_table_header;
    if (acpi_disabled)
    return false;
    status = acpi_get_table(ACPI_SIG_SDEI, 0, &sdei_table_header);
    if (ACPI_FAILURE(status) && status != AE_NOT_FOUND) {
    const char *msg = acpi_format_exception(status);
    pr_info("Failed to get ACPI:SDEI table, %s\n", msg);
    }
    if (ACPI_FAILURE(status))
    return false;
    acpi_put_table(sdei_table_header);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn acpi_sdei_init() -> void __init {
    void __init acpi_sdei_init(void)
    {
    struct platform_device *pdev;
    int ret;
    if (!sdei_present_acpi())
    return;
    pdev = platform_device_register_simple(sdei_driver.driver.name,
    0, core::ptr::null_mut(), 0);
    if (IS_ERR(pdev)) {
    ret = PTR_ERR(pdev);
    platform_driver_unregister(&sdei_driver);
    pr_info("Failed to register ACPI:SDEI platform device %d\n",
    ret);
    }
    }
#[no_mangle]
unsafe extern "C" fn sdei_init() -> int __init {
    static int __init sdei_init(void)
    {
    return platform_driver_register(&sdei_driver);
    }
    arch_initcall(sdei_init);
    int sdei_event_handler(struct pt_regs *regs,
    struct sdei_registered_event *arg)
    {
    int err;
    let mut event_num: u32 = arg.event_num;
    err = arg.callback(event_num, regs, arg.callback_arg);
    if (err)
    pr_err_ratelimited("event %u on CPU %u failed with error: %d\n",
    event_num, smp_processor_id(), err);
    return err;
    }
    NOKPROBE_SYMBOL(sdei_event_handler);
#[no_mangle]
pub unsafe extern "C" fn sdei_handler_abort() {
    void sdei_handler_abort(void)
    {
//
// If the crash happened in an SDEI event handler then we need to
// finish the handler with the firmware so that we can have working
// interrupts in the crash kernel.
//
    if (__this_cpu_read(sdei_active_critical_event)) {
    pr_warn("still in SDEI critical event context, attempting to finish handler.\n");
    __sdei_handler_abort();
    __this_cpu_write(sdei_active_critical_event, core::ptr::null_mut());
    }
    if (__this_cpu_read(sdei_active_normal_event)) {
    pr_warn("still in SDEI normal event context, attempting to finish handler.\n");
    __sdei_handler_abort();
    __this_cpu_write(sdei_active_normal_event, core::ptr::null_mut());
    }
    }
