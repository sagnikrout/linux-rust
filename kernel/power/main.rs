//! Automatically rewritten from C to Rust
//! Source: kernel/power/main.c
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
// kernel/power/main.c - PM subsystem core functionality.
//
// Copyright (c) 2003 Patrick Mochel
// Copyright (c) 2003 Open Source Development Lab
//

//
// The following functions are used by the suspend/hibernate code to temporarily
// change gfp_allowed_mask in order to avoid using I/O during memory allocations
// while devices are suspended.  To avoid races with the suspend/hibernate code,
// they should always be called with system_transition_mutex held
// (gfp_allowed_mask also should only be modified with system_transition_mutex
// held, unless the suspend/hibernate code is guaranteed not to run in parallel
// with that modification).
//
    static unsigned int saved_gfp_count;
    static gfp_t saved_gfp_mask;
#[no_mangle]
pub unsafe extern "C" fn pm_restore_gfp_mask() {
    void pm_restore_gfp_mask(void)
    {
    WARN_ON(!mutex_is_locked(&system_transition_mutex));
    if (!saved_gfp_count || --saved_gfp_count)
    return;
    gfp_allowed_mask = saved_gfp_mask;
    saved_gfp_mask = 0;
    pm_pr_dbg("GFP mask restored\n");
    }
#[no_mangle]
pub unsafe extern "C" fn pm_restrict_gfp_mask() {
    void pm_restrict_gfp_mask(void)
    {
    WARN_ON(!mutex_is_locked(&system_transition_mutex));
    if (saved_gfp_count++) {
    WARN_ON((saved_gfp_mask & ~(__GFP_IO | __GFP_FS)) != gfp_allowed_mask);
    return;
    }
    saved_gfp_mask = gfp_allowed_mask;
    gfp_allowed_mask &= ~(__GFP_IO | __GFP_FS);
    pm_pr_dbg("GFP mask restricted\n");
    }
#[no_mangle]
pub unsafe extern "C" fn lock_system_sleep() -> c_uint {
    unsigned int lock_system_sleep(void)
    {
    let mut flags: c_uint = current.flags;
    current.flags |= PF_NOFREEZE;
    mutex_lock(&system_transition_mutex);
    return flags;
    }
    EXPORT_SYMBOL_GPL(lock_system_sleep);
#[no_mangle]
pub unsafe extern "C" fn unlock_system_sleep(flags: c_uint) {
    void unlock_system_sleep(unsigned int flags)
    {
    if (!(flags & PF_NOFREEZE))
    current.flags &= ~PF_NOFREEZE;
    mutex_unlock(&system_transition_mutex);
    }
    EXPORT_SYMBOL_GPL(unlock_system_sleep);
#[no_mangle]
pub unsafe extern "C" fn ksys_sync_helper() {
    void ksys_sync_helper(void)
    {
    ktime_t start;
    long elapsed_msecs;
    start = ktime_get();
    ksys_sync();
    elapsed_msecs = ktime_to_ms(ktime_sub(ktime_get(), start));
    pr_info("Filesystems sync: %ld.%03ld seconds\n",
    elapsed_msecs / MSEC_PER_SEC, elapsed_msecs % MSEC_PER_SEC);
    }
    EXPORT_SYMBOL_GPL(ksys_sync_helper);

// Wakeup events handling resolution while syncing file systems in jiffies
pub const PM_FS_SYNC_WAKEUP_RESOLUTION: c_int = 5;
    let mut pm_fs_sync_count: static atomic_t = ATOMIC_INIT(0);
    static struct workqueue_struct *pm_fs_sync_wq;
    static DECLARE_WAIT_QUEUE_HEAD(pm_fs_sync_wait);
#[no_mangle]
unsafe extern "C" fn pm_fs_sync_completed() -> bool {
    static bool pm_fs_sync_completed(void)
    {
    return atomic_read(&pm_fs_sync_count) == 0;
    }
#[no_mangle]
unsafe extern "C" fn pm_fs_sync_work_fn(work: *mut work_struct) {
    static void pm_fs_sync_work_fn(struct work_struct *work)
    {
    ksys_sync_helper();
    if (atomic_dec_and_test(&pm_fs_sync_count))
    wake_up(&pm_fs_sync_wait);
    }
    static DECLARE_WORK(pm_fs_sync_work, pm_fs_sync_work_fn);
//
// pm_sleep_fs_sync() - Sync file systems in an interruptible way
//
// Return: 0 on successful file system sync, or -EBUSY if the file system sync
// was aborted.
//
#[no_mangle]
pub unsafe extern "C" fn pm_sleep_fs_sync() -> c_int {
    int pm_sleep_fs_sync(void)
    {
    pm_wakeup_clear(0);
//
// Take back-to-back sleeps into account by queuing a subsequent fs sync
// only if the previous fs sync is running or is not queued. Multiple fs
// syncs increase the likelihood of saving the latest files immediately
// before sleep.
//
    if (!work_pending(&pm_fs_sync_work)) {
    atomic_inc(&pm_fs_sync_count);
    queue_work(pm_fs_sync_wq, &pm_fs_sync_work);
    }
    while (!pm_fs_sync_completed()) {
    if (pm_wakeup_pending())
    return -EBUSY;
    wait_event_timeout(pm_fs_sync_wait, pm_fs_sync_completed(),
    PM_FS_SYNC_WAKEUP_RESOLUTION);
    }
    return 0;
    }

// Routines for PM-transition notifications
    static BLOCKING_NOTIFIER_HEAD(pm_chain_head);
#[no_mangle]
pub unsafe extern "C" fn register_pm_notifier(nb: *mut notifier_block) -> c_int {
    int register_pm_notifier(struct notifier_block *nb)
    {
    return blocking_notifier_chain_register(&pm_chain_head, nb);
    }
    EXPORT_SYMBOL_GPL(register_pm_notifier);
#[no_mangle]
pub unsafe extern "C" fn unregister_pm_notifier(nb: *mut notifier_block) -> c_int {
    int unregister_pm_notifier(struct notifier_block *nb)
    {
    return blocking_notifier_chain_unregister(&pm_chain_head, nb);
    }
    EXPORT_SYMBOL_GPL(unregister_pm_notifier);
#[no_mangle]
pub unsafe extern "C" fn pm_notifier_call_chain_robust(val_up: c_ulong, val_down: c_ulong) -> c_int {
    int pm_notifier_call_chain_robust(unsigned long val_up, unsigned long val_down)
    {
    int ret;
    ret = blocking_notifier_call_chain_robust(&pm_chain_head, val_up, val_down, core::ptr::null_mut());
    return notifier_to_errno(ret);
    }
#[no_mangle]
pub unsafe extern "C" fn pm_notifier_call_chain(val: c_ulong) -> c_int {
    int pm_notifier_call_chain(unsigned long val)
    {
    return blocking_notifier_call_chain(&pm_chain_head, val, core::ptr::null_mut());
    }
// If set, devices may be suspended and resumed asynchronously.
    let mut pm_async_enabled: c_int = 1;
#[no_mangle]
unsafe extern "C" fn pm_async_setup(str: *mut c_char) -> int __init {
    static int __init pm_async_setup(char *str)
    {
    if (!strcmp(str, "off"))
    pm_async_enabled = 0;
    return 1;
    }
    __setup("pm_async=", pm_async_setup);
    static ssize_t pm_async_show(struct kobject *kobj, struct kobj_attribute *attr,
    char *buf)
    {
    return sysfs_emit(buf, "%d\n", pm_async_enabled);
    }
    static ssize_t pm_async_store(struct kobject *kobj, struct kobj_attribute *attr,
    const char *buf, size_t n)
    {
    unsigned long val;
    if (kstrtoul(buf, 10, &val))
    return -EINVAL;
    if (val > 1)
    return -EINVAL;
    pm_async_enabled = val;
    return n;
    }
    power_attr(pm_async);

    static ssize_t mem_sleep_show(struct kobject *kobj, struct kobj_attribute *attr,
    char *buf)
    {
    let mut count: isize = 0;
    suspend_state_t i;
    for (i = PM_SUSPEND_MIN; i < PM_SUSPEND_MAX; i++) {
    if (i >= PM_SUSPEND_MEM && cxl_mem_active())
    continue;
    if (mem_sleep_states[i]) {
    const char *label = mem_sleep_states[i];
    if (mem_sleep_current == i)
    count += sysfs_emit_at(buf, count, "[%s] ", label);
    else
    count += sysfs_emit_at(buf, count, "%s ", label);
    }
    }
// Convert the last space to a newline if needed.
    if (count > 0)
    buf[count - 1] = '\n';
    return count;
    }
#[no_mangle]
unsafe extern "C" fn decode_suspend_state(buf: *const c_char, n: usize) -> suspend_state_t {
    static suspend_state_t decode_suspend_state(const char *buf, size_t n)
    {
    suspend_state_t state;
    char *p;
    int len;
    p = memchr(buf, '\n', n);
    len = p ? p - buf : n;
    for (state = PM_SUSPEND_MIN; state < PM_SUSPEND_MAX; state++) {
    const char *label = mem_sleep_states[state];
    if (label && len == strlen(label) && !strncmp(buf, label, len))
    return state;
    }
    return PM_SUSPEND_ON;
    }
    static ssize_t mem_sleep_store(struct kobject *kobj, struct kobj_attribute *attr,
    const char *buf, size_t n)
    {
    suspend_state_t state;
    int error;
    error = pm_autosleep_lock();
    if (error)
    return error;
    if (pm_autosleep_state() > PM_SUSPEND_ON) {
    error = -EBUSY;
    goto out;
    }
    state = decode_suspend_state(buf, n);
    if (state < PM_SUSPEND_MAX && state > PM_SUSPEND_ON)
    mem_sleep_current = state;
    else
    error = -EINVAL;
    out:
    pm_autosleep_unlock();
    return error ? error : n;
    }
    power_attr(mem_sleep);
//
// sync_on_suspend: Sync file systems before suspend.
//
// show() returns whether file systems sync before suspend is enabled.
// store() accepts 0 or 1.  0 disables file systems sync and 1 enables it.
//
    let mut sync_on_suspend_enabled: bool = !IS_ENABLED(CONFIG_SUSPEND_SKIP_SYNC);
    static ssize_t sync_on_suspend_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    return sysfs_emit(buf, "%d\n", sync_on_suspend_enabled);
    }
    static ssize_t sync_on_suspend_store(struct kobject *kobj,
    struct kobj_attribute *attr,
    const char *buf, size_t n)
    {
    unsigned long val;
    if (kstrtoul(buf, 10, &val))
    return -EINVAL;
    if (val > 1)
    return -EINVAL;
    sync_on_suspend_enabled = !!val;
    return n;
    }
    power_attr(sync_on_suspend);

    let mut pm_test_level: c_int = TEST_NONE;
    static const char * const pm_tests[__TEST_AFTER_LAST] = {
    [TEST_NONE] = "none",
    [TEST_CORE] = "core",
    [TEST_CPUS] = "processors",
    [TEST_PLATFORM] = "platform",
    [TEST_DEVICES] = "devices",
    [TEST_FREEZER] = "freezer",
    };
    static ssize_t pm_test_show(struct kobject *kobj, struct kobj_attribute *attr,
    char *buf)
    {
    let mut count: isize = 0;
    int level;
    for (level = TEST_FIRST; level <= TEST_MAX; level++)
    if (pm_tests[level]) {
    if (level == pm_test_level)
    count += sysfs_emit_at(buf, count, "[%s] ", pm_tests[level]);
    else
    count += sysfs_emit_at(buf, count, "%s ", pm_tests[level]);
    }
// Convert the last space to a newline if needed.
    if (count > 0)
    buf[count - 1] = '\n';
    return count;
    }
    static ssize_t pm_test_store(struct kobject *kobj, struct kobj_attribute *attr,
    const char *buf, size_t n)
    {
    unsigned int sleep_flags;
    const char * const *s;
    let mut error: c_int = -EINVAL;
    int level;
    char *p;
    int len;
    p = memchr(buf, '\n', n);
    len = p ? p - buf : n;
    sleep_flags = lock_system_sleep();
    level = TEST_FIRST;
    for (s = &pm_tests[level]; level <= TEST_MAX; s++, level++)
    if (*s && len == strlen(*s) && !strncmp(buf, *s, len)) {
    pm_test_level = level;
    error = 0;
    break;
    }
    unlock_system_sleep(sleep_flags);
    return error ? error : n;
    }
    power_attr(pm_test);

pub const REC_FAILED_NUM: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct suspend_stats {
    pub step_failures: [c_uint; SUSPEND_NR_STEPS],
    pub success: c_uint,
    pub fail: c_uint,
    pub last_failed_dev: c_int,
    pub failed_devs: [c_char; REC_FAILED_NUM][40],
    pub last_failed_errno: c_int,
    pub errno: [c_int; REC_FAILED_NUM],
    pub last_failed_step: c_int,
    pub last_hw_sleep: u64,
    pub total_hw_sleep: u64,
    pub max_hw_sleep: u64,
    pub failed_steps: [enum suspend_stat_step; REC_FAILED_NUM],
}

    static struct suspend_stats suspend_stats;
    static DEFINE_MUTEX(suspend_stats_lock);
#[no_mangle]
pub unsafe extern "C" fn dpm_save_failed_dev(name: *const c_char) {
    void dpm_save_failed_dev(const char *name)
    {
    mutex_lock(&suspend_stats_lock);
    strscpy(suspend_stats.failed_devs[suspend_stats.last_failed_dev],
    name, sizeof(suspend_stats.failed_devs[0]));
    suspend_stats.last_failed_dev++;
    suspend_stats.last_failed_dev %= REC_FAILED_NUM;
    mutex_unlock(&suspend_stats_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn dpm_save_failed_step(step: enum suspend_stat_step) {
    void dpm_save_failed_step(enum suspend_stat_step step)
    {
    suspend_stats.step_failures[step-1]++;
    suspend_stats.failed_steps[suspend_stats.last_failed_step] = step;
    suspend_stats.last_failed_step++;
    suspend_stats.last_failed_step %= REC_FAILED_NUM;
    }
#[no_mangle]
pub unsafe extern "C" fn dpm_save_errno(err: c_int) {
    void dpm_save_errno(int err)
    {
    if (!err) {
    suspend_stats.success++;
    return;
    }
    suspend_stats.fail++;
    suspend_stats.errno[suspend_stats.last_failed_errno] = err;
    suspend_stats.last_failed_errno++;
    suspend_stats.last_failed_errno %= REC_FAILED_NUM;
    }
#[no_mangle]
pub unsafe extern "C" fn pm_report_hw_sleep_time(t: u64) {
    void pm_report_hw_sleep_time(u64 t)
    {
    suspend_stats.last_hw_sleep = t;
    suspend_stats.total_hw_sleep += t;
    }
    EXPORT_SYMBOL_GPL(pm_report_hw_sleep_time);
#[no_mangle]
pub unsafe extern "C" fn pm_report_max_hw_sleep(t: u64) {
    void pm_report_max_hw_sleep(u64 t)
    {
    suspend_stats.max_hw_sleep = t;
    }
    EXPORT_SYMBOL_GPL(pm_report_max_hw_sleep);
    static const char * const suspend_step_names[] = {
    [SUSPEND_WORKING] = "",
    [SUSPEND_FREEZE] = "freeze",
    [SUSPEND_PREPARE] = "prepare",
    [SUSPEND_SUSPEND] = "suspend",
    [SUSPEND_SUSPEND_LATE] = "suspend_late",
    [SUSPEND_SUSPEND_NOIRQ] = "suspend_noirq",
    [SUSPEND_RESUME_NOIRQ] = "resume_noirq",
    [SUSPEND_RESUME_EARLY] = "resume_early",
    [SUSPEND_RESUME] = "resume",
    };

    static ssize_t _name##_show(struct kobject *kobj,		\
    struct kobj_attribute *attr, char *buf)		\
    {								\
    return sysfs_emit(buf, format_str, suspend_stats._name);\
    }								\
    static struct kobj_attribute _name = __ATTR_RO(_name)
    suspend_attr(success, "%u\n");
    suspend_attr(fail, "%u\n");
    suspend_attr(last_hw_sleep, "%llu\n");
    suspend_attr(total_hw_sleep, "%llu\n");
    suspend_attr(max_hw_sleep, "%llu\n");

    static ssize_t _name##_show(struct kobject *kobj,		\
    struct kobj_attribute *attr, char *buf)		\
    {								\
    return sysfs_emit(buf, "%u\n",				\
    suspend_stats.step_failures[step-1]);	\
    }								\
    static struct kobj_attribute _name = __ATTR_RO(_name)
    suspend_step_attr(failed_freeze, SUSPEND_FREEZE);
    suspend_step_attr(failed_prepare, SUSPEND_PREPARE);
    suspend_step_attr(failed_suspend, SUSPEND_SUSPEND);
    suspend_step_attr(failed_suspend_late, SUSPEND_SUSPEND_LATE);
    suspend_step_attr(failed_suspend_noirq, SUSPEND_SUSPEND_NOIRQ);
    suspend_step_attr(failed_resume, SUSPEND_RESUME);
    suspend_step_attr(failed_resume_early, SUSPEND_RESUME_EARLY);
    suspend_step_attr(failed_resume_noirq, SUSPEND_RESUME_NOIRQ);
    static ssize_t last_failed_dev_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    int index;
    char *last_failed_dev = core::ptr::null_mut();
    index = suspend_stats.last_failed_dev + REC_FAILED_NUM - 1;
    index %= REC_FAILED_NUM;
    last_failed_dev = suspend_stats.failed_devs[index];
    return sysfs_emit(buf, "%s\n", last_failed_dev);
    }
    let mut last_failed_dev: static struct kobj_attribute = __ATTR_RO(last_failed_dev);
    static ssize_t last_failed_errno_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    int index;
    int last_failed_errno;
    index = suspend_stats.last_failed_errno + REC_FAILED_NUM - 1;
    index %= REC_FAILED_NUM;
    last_failed_errno = suspend_stats.errno[index];
    return sysfs_emit(buf, "%d\n", last_failed_errno);
    }
    let mut last_failed_errno: static struct kobj_attribute = __ATTR_RO(last_failed_errno);
    static ssize_t last_failed_step_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    enum suspend_stat_step step;
    int index;
    index = suspend_stats.last_failed_step + REC_FAILED_NUM - 1;
    index %= REC_FAILED_NUM;
    step = suspend_stats.failed_steps[index];
    return sysfs_emit(buf, "%s\n", suspend_step_names[step]);
    }
    let mut last_failed_step: static struct kobj_attribute = __ATTR_RO(last_failed_step);
    static struct attribute *suspend_attrs[] = {
    &success.attr,
    &fail.attr,
    &failed_freeze.attr,
    &failed_prepare.attr,
    &failed_suspend.attr,
    &failed_suspend_late.attr,
    &failed_suspend_noirq.attr,
    &failed_resume.attr,
    &failed_resume_early.attr,
    &failed_resume_noirq.attr,
    &last_failed_dev.attr,
    &last_failed_errno.attr,
    &last_failed_step.attr,
    &last_hw_sleep.attr,
    &total_hw_sleep.attr,
    &max_hw_sleep.attr,
    core::ptr::null_mut(),
    };
#[no_mangle]
unsafe extern "C" fn suspend_attr_is_visible(kobj: *mut kobject, attr: *mut attribute, idx: c_int) -> umode_t {
    static umode_t suspend_attr_is_visible(struct kobject *kobj, struct attribute *attr, int idx)
    {
    if (attr != &last_hw_sleep.attr &&
    attr != &total_hw_sleep.attr &&
    attr != &max_hw_sleep.attr)
    return 0444;

    if (acpi_gbl_FADT.flags & ACPI_FADT_LOW_POWER_S0)
    return 0444;

    return 0;
    }
    static const struct attribute_group suspend_attr_group = {
    .name = "suspend_stats",
    .attrs = suspend_attrs,
    .is_visible = suspend_attr_is_visible,
    };

#[no_mangle]
unsafe extern "C" fn suspend_stats_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    static int suspend_stats_show(struct seq_file *s, void *unused)
    {
    int i, index, last_dev, last_errno, last_step;
    enum suspend_stat_step step;
    last_dev = suspend_stats.last_failed_dev + REC_FAILED_NUM - 1;
    last_dev %= REC_FAILED_NUM;
    last_errno = suspend_stats.last_failed_errno + REC_FAILED_NUM - 1;
    last_errno %= REC_FAILED_NUM;
    last_step = suspend_stats.last_failed_step + REC_FAILED_NUM - 1;
    last_step %= REC_FAILED_NUM;
    seq_printf(s, "success: %u\nfail: %u\n",
    suspend_stats.success, suspend_stats.fail);
    for (step = SUSPEND_FREEZE; step <= SUSPEND_NR_STEPS; step++)
    seq_printf(s, "failed_%s: %u\n", suspend_step_names[step],
    suspend_stats.step_failures[step-1]);
    seq_printf(s,	"failures:\n  last_failed_dev:\t%-s\n",
    suspend_stats.failed_devs[last_dev]);
    for (i = 1; i < REC_FAILED_NUM; i++) {
    index = last_dev + REC_FAILED_NUM - i;
    index %= REC_FAILED_NUM;
    seq_printf(s, "\t\t\t%-s\n", suspend_stats.failed_devs[index]);
    }
    seq_printf(s,	"  last_failed_errno:\t%-d\n",
    suspend_stats.errno[last_errno]);
    for (i = 1; i < REC_FAILED_NUM; i++) {
    index = last_errno + REC_FAILED_NUM - i;
    index %= REC_FAILED_NUM;
    seq_printf(s, "\t\t\t%-d\n", suspend_stats.errno[index]);
    }
    seq_printf(s,	"  last_failed_step:\t%-s\n",
    suspend_step_names[suspend_stats.failed_steps[last_step]]);
    for (i = 1; i < REC_FAILED_NUM; i++) {
    index = last_step + REC_FAILED_NUM - i;
    index %= REC_FAILED_NUM;
    seq_printf(s, "\t\t\t%-s\n",
    suspend_step_names[suspend_stats.failed_steps[index]]);
    }
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(suspend_stats);
#[no_mangle]
unsafe extern "C" fn pm_debugfs_init() -> int __init {
    static int __init pm_debugfs_init(void)
    {
    debugfs_create_file("suspend_stats", S_IFREG | S_IRUGO,
    core::ptr::null_mut(), core::ptr::null_mut(), &suspend_stats_fops);
    return 0;
    }
    late_initcall(pm_debugfs_init);

#[no_mangle]
pub unsafe extern "C" fn pm_sleep_transition_in_progress() -> bool {
    bool pm_sleep_transition_in_progress(void)
    {
    return pm_suspend_in_progress() || hibernation_in_progress();
    }

//
// pm_print_times: print time taken by devices to suspend and resume.
//
// show() returns whether printing of suspend and resume times is enabled.
// store() accepts 0 or 1.  0 disables printing and 1 enables it.
//
    bool pm_print_times_enabled;
    static ssize_t pm_print_times_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    return sysfs_emit(buf, "%d\n", pm_print_times_enabled);
    }
    static ssize_t pm_print_times_store(struct kobject *kobj,
    struct kobj_attribute *attr,
    const char *buf, size_t n)
    {
    unsigned long val;
    if (kstrtoul(buf, 10, &val))
    return -EINVAL;
    if (val > 1)
    return -EINVAL;
    pm_print_times_enabled = !!val;
    return n;
    }
    power_attr(pm_print_times);
#[no_mangle]
pub unsafe extern "C" fn pm_print_times_init() {
    static inline void pm_print_times_init(void)
    {
    pm_print_times_enabled = initcall_debug;
    }
    static ssize_t pm_wakeup_irq_show(struct kobject *kobj,
    struct kobj_attribute *attr,
    char *buf)
    {
    if (!pm_wakeup_irq())
    return -ENODATA;
    return sysfs_emit(buf, "%u\n", pm_wakeup_irq());
    }
    power_attr_ro(pm_wakeup_irq);
    bool pm_debug_messages_on __read_mostly;
#[no_mangle]
pub unsafe extern "C" fn pm_debug_messages_should_print() -> bool {
    bool pm_debug_messages_should_print(void)
    {
    return pm_debug_messages_on && pm_sleep_transition_in_progress();
    }
    EXPORT_SYMBOL_GPL(pm_debug_messages_should_print);
    static ssize_t pm_debug_messages_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    return sysfs_emit(buf, "%d\n", pm_debug_messages_on);
    }
    static ssize_t pm_debug_messages_store(struct kobject *kobj,
    struct kobj_attribute *attr,
    const char *buf, size_t n)
    {
    unsigned long val;
    if (kstrtoul(buf, 10, &val))
    return -EINVAL;
    if (val > 1)
    return -EINVAL;
    pm_debug_messages_on = !!val;
    return n;
    }
    power_attr(pm_debug_messages);
#[no_mangle]
unsafe extern "C" fn pm_debug_messages_setup(str: *mut c_char) -> int __init {
    static int __init pm_debug_messages_setup(char *str)
    {
    pm_debug_messages_on = true;
    return 1;
    }
    __setup("pm_debug_messages", pm_debug_messages_setup);

    static inline void pm_print_times_init(void) {}

    struct kobject *power_kobj;
//
// state - control system sleep states.
//
// show() returns available sleep state labels, which may be "mem", "standby",
// "freeze" and "disk" (hibernation).
// See Documentation/admin-guide/pm/sleep-states.rst for a description of
// what they mean.
//
// store() accepts one of those strings, translates it into the proper
// enumerated value, and initiates a suspend transition.
//
    static ssize_t state_show(struct kobject *kobj, struct kobj_attribute *attr,
    char *buf)
    {
    let mut count: isize = 0;

    suspend_state_t i;
    for (i = PM_SUSPEND_MIN; i < PM_SUSPEND_MAX; i++)
    if (pm_states[i])
    count += sysfs_emit_at(buf, count, "%s ", pm_states[i]);

    if (hibernation_available())
    count += sysfs_emit_at(buf, count, "disk ");
// Convert the last space to a newline if needed.
    if (count > 0)
    buf[count - 1] = '\n';
    return count;
    }
#[no_mangle]
unsafe extern "C" fn decode_state(buf: *const c_char, n: usize) -> suspend_state_t {
    static suspend_state_t decode_state(const char *buf, size_t n)
    {

    suspend_state_t state;

    char *p;
    int len;
    p = memchr(buf, '\n', n);
    len = p ? p - buf : n;
// Check hibernation first.
    if (len == 4 && str_has_prefix(buf, "disk"))
    return PM_SUSPEND_MAX;

    for (state = PM_SUSPEND_MIN; state < PM_SUSPEND_MAX; state++) {
    const char *label = pm_states[state];
    if (label && len == strlen(label) && !strncmp(buf, label, len))
    return state;
    }

    return PM_SUSPEND_ON;
    }
    static ssize_t state_store(struct kobject *kobj, struct kobj_attribute *attr,
    const char *buf, size_t n)
    {
    suspend_state_t state;
    int error;
    error = pm_autosleep_lock();
    if (error)
    return error;
    if (pm_autosleep_state() > PM_SUSPEND_ON) {
    error = -EBUSY;
    goto out;
    }
    state = decode_state(buf, n);
    if (state < PM_SUSPEND_MAX) {
    if (state == PM_SUSPEND_MEM)
    state = mem_sleep_current;
    error = pm_suspend(state);
    } else if (state == PM_SUSPEND_MAX) {
    error = hibernate();
    } else {
    error = -EINVAL;
    }
    out:
    pm_autosleep_unlock();
    return error ? error : n;
    }
    power_attr(state);

//
// The 'wakeup_count' attribute, along with the functions defined in
// drivers/base/power/wakeup.c, provides a means by which wakeup events can be
// handled in a non-racy way.
//
// If a wakeup event occurs when the system is in a sleep state, it simply is
// woken up.  In turn, if an event that would wake the system up from a sleep
// state occurs when it is undergoing a transition to that sleep state, the
// transition should be aborted.  Moreover, if such an event occurs when the
// system is in the working state, an attempt to start a transition to the
// given sleep state should fail during certain period after the detection of
// the event.  Using the 'state' attribute alone is not sufficient to satisfy
// these requirements, because a wakeup event may occur exactly when 'state'
// is being written to and may be delivered to user space right before it is
// frozen, so the event will remain only partially processed until the system is
// woken up by another event.  In particular, it won't cause the transition to
// a sleep state to be aborted.
//
// This difficulty may be overcome if user space uses 'wakeup_count' before
// writing to 'state'.  It first should read from 'wakeup_count' and store
// the read value.  Then, after carrying out its own preparations for the system
// transition to a sleep state, it should write the stored value to
// 'wakeup_count'.  If that fails, at least one wakeup event has occurred since
// 'wakeup_count' was read and 'state' should not be written to.  Otherwise, it
// is allowed to write to 'state', but the transition will be aborted if there
// are any wakeup events detected after 'wakeup_count' was written to.
//
    static ssize_t wakeup_count_show(struct kobject *kobj,
    struct kobj_attribute *attr,
    char *buf)
    {
    unsigned int val;
    return pm_get_wakeup_count(&val, true) ?
    sysfs_emit(buf, "%u\n", val) : -EINTR;
    }
    static ssize_t wakeup_count_store(struct kobject *kobj,
    struct kobj_attribute *attr,
    const char *buf, size_t n)
    {
    unsigned int val;
    int error;
    error = pm_autosleep_lock();
    if (error)
    return error;
    if (pm_autosleep_state() > PM_SUSPEND_ON) {
    error = -EBUSY;
    goto out;
    }
    error = -EINVAL;
    if (sscanf(buf, "%u", &val) == 1) {
    if (pm_save_wakeup_count(val))
    error = n;
    else
    pm_print_active_wakeup_sources();
    }
    out:
    pm_autosleep_unlock();
    return error;
    }
    power_attr(wakeup_count);

    static ssize_t autosleep_show(struct kobject *kobj,
    struct kobj_attribute *attr,
    char *buf)
    {
    let mut state: suspend_state_t = pm_autosleep_state();
    if (state == PM_SUSPEND_ON)
    return sysfs_emit(buf, "off\n");

    if (state < PM_SUSPEND_MAX)
    return sysfs_emit(buf, "%s\n", pm_states[state] ?
    pm_states[state] : "error");

    return sysfs_emit(buf, "disk\n");

    return sysfs_emit(buf, "error\n");

    }
    static ssize_t autosleep_store(struct kobject *kobj,
    struct kobj_attribute *attr,
    const char *buf, size_t n)
    {
    let mut state: suspend_state_t = decode_state(buf, n);
    int error;
    if (state == PM_SUSPEND_ON
    && strcmp(buf, "off") && strcmp(buf, "off\n"))
    return -EINVAL;
    if (state == PM_SUSPEND_MEM)
    state = mem_sleep_current;
    error = pm_autosleep_set_state(state);
    return error ? error : n;
    }
    power_attr(autosleep);

    static ssize_t wake_lock_show(struct kobject *kobj,
    struct kobj_attribute *attr,
    char *buf)
    {
    return pm_show_wakelocks(buf, true);
    }
    static ssize_t wake_lock_store(struct kobject *kobj,
    struct kobj_attribute *attr,
    const char *buf, size_t n)
    {
    let mut error: c_int = pm_wake_lock(buf);
    return error ? error : n;
    }
    power_attr(wake_lock);
    static ssize_t wake_unlock_show(struct kobject *kobj,
    struct kobj_attribute *attr,
    char *buf)
    {
    return pm_show_wakelocks(buf, false);
    }
    static ssize_t wake_unlock_store(struct kobject *kobj,
    struct kobj_attribute *attr,
    const char *buf, size_t n)
    {
    let mut error: c_int = pm_wake_unlock(buf);
    return error ? error : n;
    }
    power_attr(wake_unlock);

    int pm_trace_enabled;
    static ssize_t pm_trace_show(struct kobject *kobj, struct kobj_attribute *attr,
    char *buf)
    {
    return sysfs_emit(buf, "%d\n", pm_trace_enabled);
    }
    static ssize_t
    pm_trace_store(struct kobject *kobj, struct kobj_attribute *attr,
    const char *buf, size_t n)
    {
    int val;
    if (sscanf(buf, "%d", &val) == 1) {
    pm_trace_enabled = !!val;
    if (pm_trace_enabled) {
    pr_warn("PM: Enabling pm_trace changes system date and time during resume.\n"
    "PM: Correct system time has to be restored manually after resume.\n");
    }
    return n;
    }
    return -EINVAL;
    }
    power_attr(pm_trace);
    static ssize_t pm_trace_dev_match_show(struct kobject *kobj,
    struct kobj_attribute *attr,
    char *buf)
    {
    return show_trace_dev_match(buf, PAGE_SIZE);
    }
    power_attr_ro(pm_trace_dev_match);

    static ssize_t pm_freeze_timeout_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    return sysfs_emit(buf, "%u\n", freeze_timeout_msecs);
    }
    static ssize_t pm_freeze_timeout_store(struct kobject *kobj,
    struct kobj_attribute *attr,
    const char *buf, size_t n)
    {
    unsigned long val;
    if (kstrtoul(buf, 10, &val))
    return -EINVAL;
    freeze_timeout_msecs = val;
    return n;
    }
    power_attr(pm_freeze_timeout);

    let mut filesystem_freeze_enabled: bool = false;
    static ssize_t freeze_filesystems_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    return sysfs_emit(buf, "%d\n", filesystem_freeze_enabled);
    }
    static ssize_t freeze_filesystems_store(struct kobject *kobj,
    struct kobj_attribute *attr,
    const char *buf, size_t n)
    {
    unsigned long val;
    if (kstrtoul(buf, 10, &val))
    return -EINVAL;
    if (val > 1)
    return -EINVAL;
    filesystem_freeze_enabled = !!val;
    return n;
    }
    power_attr(freeze_filesystems);

    static struct attribute * g[] = {
    &state_attr.attr,

    &pm_trace_attr.attr,
    &pm_trace_dev_match_attr.attr,

    &pm_async_attr.attr,
    &wakeup_count_attr.attr,

    &mem_sleep_attr.attr,
    &sync_on_suspend_attr.attr,

    &autosleep_attr.attr,

    &wake_lock_attr.attr,
    &wake_unlock_attr.attr,

    &pm_test_attr.attr,
    &pm_print_times_attr.attr,
    &pm_wakeup_irq_attr.attr,
    &pm_debug_messages_attr.attr,

    &pm_freeze_timeout_attr.attr,

    &freeze_filesystems_attr.attr,

    core::ptr::null_mut(),
    };
    static const struct attribute_group attr_group = {
    .attrs = g,
    };
    static const struct attribute_group *attr_groups[] = {
    &attr_group,

    &suspend_attr_group,

    core::ptr::null_mut(),
    };
    struct workqueue_struct *pm_wq;
    EXPORT_SYMBOL_GPL(pm_wq);
#[no_mangle]
unsafe extern "C" fn pm_start_workqueues() -> int __init {
    static int __init pm_start_workqueues(void)
    {
    pm_wq = alloc_workqueue("pm", WQ_UNBOUND, 0);
    if (!pm_wq)
    return -ENOMEM;

    pm_fs_sync_wq = alloc_ordered_workqueue("pm_fs_sync", 0);
    if (!pm_fs_sync_wq) {
    destroy_workqueue(pm_wq);
    return -ENOMEM;
    }

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pm_init() -> int __init {
    static int __init pm_init(void)
    {
    let mut error: c_int = pm_start_workqueues();
    if (error)
    return error;
    hibernate_image_size_init();
    hibernate_reserved_size_init();
    pm_states_init();
    power_kobj = kobject_create_and_add("power", core::ptr::null_mut());
    if (!power_kobj)
    return -ENOMEM;
    error = sysfs_create_groups(power_kobj, attr_groups);
    if (error)
    return error;
    pm_print_times_init();
    return pm_autosleep_init();
    }
    core_initcall(pm_init);
