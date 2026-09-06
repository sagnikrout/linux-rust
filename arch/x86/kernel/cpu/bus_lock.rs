//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/cpu/bus_lock.c
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

    enum split_lock_detect_state {
    sld_off = 0,
    sld_warn,
    sld_fatal,
    sld_ratelimit,
    };
//
// Default to sld_off because most systems do not support split lock detection.
// sld_state_setup() will switch this to sld_warn on systems that support
// split lock/bus lock detect, unless there is a command line override.
//
    let mut __ro_after_init: static enum split_lock_detect_state sld_state = sld_off;
    static u64 msr_test_ctrl_cache __ro_after_init;
//
// With a name like MSR_TEST_CTL it should go without saying, but don't touch
// MSR_TEST_CTL unless the CPU is one of the whitelisted models.  Writing it
// on CPUs that do not support SLD can cause fireworks, even when writing '0'.
//
    static bool cpu_model_supports_sld __ro_after_init;
    static const struct {
    const char			*option;
    enum split_lock_detect_state	state;
    } sld_options[] __initconst = {
    { "off",	sld_off   },
    { "warn",	sld_warn  },
    { "fatal",	sld_fatal },
    { "ratelimit:", sld_ratelimit },
    };
    static struct ratelimit_state bld_ratelimit;
    let mut sysctl_sld_mitigate: static unsigned int = 1;
    static DEFINE_SEMAPHORE(buslock_sem, 1);

    static const struct ctl_table sld_sysctls[] = {
    {
    .procname       = "split_lock_mitigate",
    .data           = &sysctl_sld_mitigate,
    .maxlen         = sizeof(unsigned int),
    .mode           = 0644,
    .proc_handler	= proc_douintvec_minmax,
    .extra1         = SYSCTL_ZERO,
    .extra2         = SYSCTL_ONE,
    },
    };
#[no_mangle]
unsafe extern "C" fn sld_mitigate_sysctl_init() -> int __init {
    static int __init sld_mitigate_sysctl_init(void)
    {
    register_sysctl_init("kernel", sld_sysctls);
    return 0;
    }
    late_initcall(sld_mitigate_sysctl_init);

#[no_mangle]
pub unsafe extern "C" fn match_option(arg: *const c_char, arglen: c_int, opt: *const c_char) -> bool {
    static inline bool match_option(const char *arg, int arglen, const char *opt)
    {
    let mut len: c_int = strlen(opt), ratelimit;
    if (strncmp(arg, opt, len))
    return false;
//
// Min ratelimit is 1 bus lock/sec.
// Max ratelimit is 1000 bus locks/sec.
//
    if (sscanf(arg, "ratelimit:%d", &ratelimit) == 1 &&
    ratelimit > 0 && ratelimit <= 1000) {
    ratelimit_state_init(&bld_ratelimit, HZ, ratelimit);
    ratelimit_set_flags(&bld_ratelimit, RATELIMIT_MSG_ON_RELEASE);
    return true;
    }
    let mut len: return = = arglen;
    }
#[no_mangle]
unsafe extern "C" fn split_lock_verify_msr(on: bool) -> bool {
    static bool split_lock_verify_msr(bool on)
    {
    u64 ctrl, tmp;
    if (rdmsrq_safe(MSR_TEST_CTRL, &ctrl))
    return false;
    if (on)
    ctrl |= MSR_TEST_CTRL_SPLIT_LOCK_DETECT;
    else
    ctrl &= ~MSR_TEST_CTRL_SPLIT_LOCK_DETECT;
    if (wrmsrq_safe(MSR_TEST_CTRL, ctrl))
    return false;
    rdmsrq(MSR_TEST_CTRL, tmp);
    let mut ctrl: return = = tmp;
    }
#[no_mangle]
unsafe extern "C" fn sld_state_setup() -> void __init {
    static void __init sld_state_setup(void)
    {
    let mut state: enum split_lock_detect_state = sld_warn;
    char arg[20];
    int i, ret;
    if (!boot_cpu_has(X86_FEATURE_SPLIT_LOCK_DETECT) &&
    !boot_cpu_has(X86_FEATURE_BUS_LOCK_DETECT))
    return;
    ret = cmdline_find_option(boot_command_line, "split_lock_detect",
    arg, sizeof(arg));
    if (ret >= 0) {
    for (i = 0; i < ARRAY_SIZE(sld_options); i++) {
    if (match_option(arg, ret, sld_options[i].option)) {
    state = sld_options[i].state;
    break;
    }
    }
    }
    sld_state = state;
    }
#[no_mangle]
unsafe extern "C" fn setup_split_lock_detect(arg: *mut c_char) -> __init int {
    static __init int setup_split_lock_detect(char *arg)
    {
    return 1;
    }
    __setup("split_lock_detect=", setup_split_lock_detect);
#[no_mangle]
unsafe extern "C" fn __split_lock_setup() -> void __init {
    static void __init __split_lock_setup(void)
    {
    if (!split_lock_verify_msr(false)) {
    pr_info("MSR access failed: Disabled\n");
    return;
    }
    rdmsrq(MSR_TEST_CTRL, msr_test_ctrl_cache);
    if (!split_lock_verify_msr(true)) {
    pr_info("MSR access failed: Disabled\n");
    return;
    }
// Restore the MSR to its cached value.
    wrmsrq(MSR_TEST_CTRL, msr_test_ctrl_cache);
    setup_force_cpu_cap(X86_FEATURE_SPLIT_LOCK_DETECT);
    }
//
// MSR_TEST_CTRL is per core, but we treat it like a per CPU MSR. Locking
// is not implemented as one thread could undo the setting of the other
// thread immediately after dropping the lock anyway.
//
#[no_mangle]
unsafe extern "C" fn sld_update_msr(on: bool) {
    static void sld_update_msr(bool on)
    {
    let mut test_ctrl_val: u64 = msr_test_ctrl_cache;
    if (on)
    test_ctrl_val |= MSR_TEST_CTRL_SPLIT_LOCK_DETECT;
    wrmsrq(MSR_TEST_CTRL, test_ctrl_val);
    }
#[no_mangle]
pub unsafe extern "C" fn split_lock_init() {
    void split_lock_init(void)
    {
//
// #DB for bus lock handles ratelimit and #AC for split lock is
// disabled.
//
    if (sld_state == sld_ratelimit) {
    split_lock_verify_msr(false);
    return;
    }
    if (cpu_model_supports_sld)
    split_lock_verify_msr(sld_state != sld_off);
    }
#[no_mangle]
unsafe extern "C" fn __split_lock_reenable_unlock(work: *mut work_struct) {
    static void __split_lock_reenable_unlock(struct work_struct *work)
    {
    sld_update_msr(true);
    up(&buslock_sem);
    }
    static DECLARE_DELAYED_WORK(sl_reenable_unlock, __split_lock_reenable_unlock);
#[no_mangle]
unsafe extern "C" fn __split_lock_reenable(work: *mut work_struct) {
    static void __split_lock_reenable(struct work_struct *work)
    {
    sld_update_msr(true);
    }
//
// In order for each CPU to schedule its delayed work independently of the
// others, delayed work struct must be per-CPU. This is not required when
// sysctl_sld_mitigate is enabled because of the semaphore that limits
// the number of simultaneously scheduled delayed works to 1.
//
    static DEFINE_PER_CPU(struct delayed_work, sl_reenable);
//
// Per-CPU delayed_work can't be statically initialized properly because
// the struct address is unknown. Thus per-CPU delayed_work structures
// have to be initialized during kernel initialization and after calling
// setup_per_cpu_areas().
//
#[no_mangle]
unsafe extern "C" fn setup_split_lock_delayed_work() -> int __init {
    static int __init setup_split_lock_delayed_work(void)
    {
    unsigned int cpu;
    for_each_possible_cpu(cpu) {
    struct delayed_work *work = per_cpu_ptr(&sl_reenable, cpu);
    INIT_DELAYED_WORK(work, __split_lock_reenable);
    }
    return 0;
    }
    pure_initcall(setup_split_lock_delayed_work);
//
// If a CPU goes offline with pending delayed work to re-enable split lock
// detection then the delayed work will be executed on some other CPU. That
// handles releasing the buslock_sem, but because it executes on a
// different CPU probably won't re-enable split lock detection. This is a
// problem on HT systems since the sibling CPU on the same core may then be
// left running with split lock detection disabled.
//
// Unconditionally re-enable detection here.
//
#[no_mangle]
unsafe extern "C" fn splitlock_cpu_offline(cpu: c_uint) -> c_int {
    static int splitlock_cpu_offline(unsigned int cpu)
    {
    sld_update_msr(true);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn split_lock_warn(ip: c_ulong) {
    static void split_lock_warn(unsigned long ip)
    {
    struct delayed_work *work;
    int cpu;
    let mut saved_sld_mitigate: c_uint = READ_ONCE(sysctl_sld_mitigate);
    if (!current.reported_split_lock)
    pr_warn_ratelimited("#AC: %s/%d took a split_lock trap at address: 0x%lx\n",
    current.comm, current.pid, ip);
    current.reported_split_lock = 1;
    if (saved_sld_mitigate) {
//
// misery factor #1:
// sleep 10ms before trying to execute split lock.
//
    if (msleep_interruptible(10) > 0)
    return;
//
// Misery factor #2:
// only allow one buslocked disabled core at a time.
//
    if (down_interruptible(&buslock_sem) == -EINTR)
    return;
    }
    cpu = get_cpu();
    work = saved_sld_mitigate ? &sl_reenable_unlock : per_cpu_ptr(&sl_reenable, cpu);
    schedule_delayed_work_on(cpu, work, 2);
// Disable split lock detection on this CPU to make progress
    sld_update_msr(false);
    put_cpu();
    }
#[no_mangle]
pub unsafe extern "C" fn handle_guest_split_lock(ip: c_ulong) -> bool {
    bool handle_guest_split_lock(unsigned long ip)
    {
    if (sld_state == sld_warn) {
    split_lock_warn(ip);
    return true;
    }
    pr_warn_once("#AC: %s/%d %s split_lock trap at address: 0x%lx\n",
    current.comm, current.pid,
    sld_state == sld_fatal ? "fatal" : "bogus", ip);
    current.thread.error_code = 0;
    current.thread.trap_nr = X86_TRAP_AC;
    force_sig_fault(SIGBUS, BUS_ADRALN, core::ptr::null_mut());
    return false;
    }
    EXPORT_SYMBOL_FOR_KVM(handle_guest_split_lock);
#[no_mangle]
pub unsafe extern "C" fn bus_lock_init() {
    void bus_lock_init(void)
    {
    u64 val;
    if (!boot_cpu_has(X86_FEATURE_BUS_LOCK_DETECT))
    return;
    rdmsrq(MSR_IA32_DEBUGCTLMSR, val);
    if ((boot_cpu_has(X86_FEATURE_SPLIT_LOCK_DETECT) &&
    (sld_state == sld_warn || sld_state == sld_fatal)) ||
    sld_state == sld_off) {
//
// Warn and fatal are handled by #AC for split lock if #AC for
// split lock is supported.
//
    val &= ~DEBUGCTLMSR_BUS_LOCK_DETECT;
    } else {
    val |= DEBUGCTLMSR_BUS_LOCK_DETECT;
    }
    wrmsrq(MSR_IA32_DEBUGCTLMSR, val);
    }
#[no_mangle]
pub unsafe extern "C" fn handle_user_split_lock(regs: *mut pt_regs, error_code: c_long) -> bool {
    bool handle_user_split_lock(struct pt_regs *regs, long error_code)
    {
    if ((regs.flags & X86_EFLAGS_AC) || sld_state == sld_fatal)
    return false;
    split_lock_warn(regs.ip);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn handle_bus_lock(regs: *mut pt_regs) {
    void handle_bus_lock(struct pt_regs *regs)
    {
    switch (sld_state) {
    case sld_off:
    break;
    case sld_ratelimit:
// Enforce no more than bld_ratelimit bus locks/sec.
    while (!__ratelimit(&bld_ratelimit))
    msleep(20);
// Warn on the bus lock.
    fallthrough;
    case sld_warn:
    pr_warn_ratelimited("#DB: %s/%d took a bus_lock trap at address: 0x%lx\n",
    current.comm, current.pid, regs.ip);
    break;
    case sld_fatal:
    force_sig_fault(SIGBUS, BUS_ADRALN, core::ptr::null_mut());
    break;
    }
    }
//
// CPU models that are known to have the per-core split-lock detection
// feature even though they do not enumerate IA32_CORE_CAPABILITIES.
//
    static const struct x86_cpu_id split_lock_cpu_ids[] __initconst = {
    X86_MATCH_VFM(INTEL_ICELAKE_X,	0),
    X86_MATCH_VFM(INTEL_ICELAKE_L,	0),
    X86_MATCH_VFM(INTEL_ICELAKE_D,	0),
    {}
    };
#[no_mangle]
unsafe extern "C" fn split_lock_setup(c: *mut cpuinfo_x86) -> void __init {
    static void __init split_lock_setup(struct cpuinfo_x86 *c)
    {
    const struct x86_cpu_id *m;
    u64 ia32_core_caps;
    if (boot_cpu_has(X86_FEATURE_HYPERVISOR))
    return;
// Check for CPUs that have support but do not enumerate it:
    m = x86_match_cpu(split_lock_cpu_ids);
    if (m)
    goto supported;
    if (!cpu_has(c, X86_FEATURE_CORE_CAPABILITIES))
    return;
//
// Not all bits in MSR_IA32_CORE_CAPS are architectural, but
// MSR_IA32_CORE_CAPS_SPLIT_LOCK_DETECT is.  All CPUs that set
// it have split lock detection.
//
    rdmsrq(MSR_IA32_CORE_CAPS, ia32_core_caps);
    if (ia32_core_caps & MSR_IA32_CORE_CAPS_SPLIT_LOCK_DETECT)
    goto supported;
// CPU is not in the model list and does not have the MSR bit:
    return;
    supported:
    cpu_model_supports_sld = true;
    __split_lock_setup();
    }
#[no_mangle]
unsafe extern "C" fn sld_state_show() {
    static void sld_state_show(void)
    {
    const char *action = "warning";
    if ((!boot_cpu_has(X86_FEATURE_BUS_LOCK_DETECT) &&
    !boot_cpu_has(X86_FEATURE_SPLIT_LOCK_DETECT)) ||
    (sld_state == sld_off))
    return;
    if (sld_state == sld_ratelimit) {
    if (boot_cpu_has(X86_FEATURE_BUS_LOCK_DETECT))
    pr_info("#DB: setting system wide bus lock rate limit to %u/sec\n", bld_ratelimit.burst);
    return;
    } else if (sld_state == sld_fatal) {
    action = "sending SIGBUS";
    }
    if (boot_cpu_has(X86_FEATURE_SPLIT_LOCK_DETECT)) {
    pr_info("#AC: crashing the kernel on kernel split_locks and %s on user-space split_locks\n", action);
//
// This is handling the case where a CPU goes offline at the
// moment where split lock detection is disabled in the warn
// setting, see split_lock_warn(). It doesn't have any effect
// in the fatal case.
//
    if (cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, "x86/splitlock", core::ptr::null_mut(), splitlock_cpu_offline) < 0)
    pr_warn("No splitlock CPU offline handler\n");
    } else if (boot_cpu_has(X86_FEATURE_BUS_LOCK_DETECT)) {
    pr_info("#DB: %s on user-space bus_locks\n", action);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn sld_setup(c: *mut cpuinfo_x86) -> void __init {
    void __init sld_setup(struct cpuinfo_x86 *c)
    {
    split_lock_setup(c);
    sld_state_setup();
    sld_state_show();
    }
