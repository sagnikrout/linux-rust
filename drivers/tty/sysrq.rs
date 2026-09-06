//! Automatically rewritten from C to Rust
//! Source: drivers/tty/sysrq.c
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
// Linux Magic System Request Key Hacks
//
// (c) 1997 Martin Mares <mj@atrey.karlin.mff.cuni.cz>
// based on ideas by Pavel Machek <pavel@atrey.karlin.mff.cuni.cz>
//
// (c) 2000 Crutcher Dunnavant <crutcher+kernel@datastacks.com>
// overhauled to use key registration
// based upon discusions in irc://irc.openprojects.net/#kernelnewbies
//
// Copyright (c) 2010 Dmitry Torokhov
// Input handler conversion
//

// Whether we react on sysrq keys or just ignore them
    let mut sysrq_enabled: static int __read_mostly = CONFIG_MAGIC_SYSRQ_DEFAULT_ENABLE;
    static bool __read_mostly sysrq_always_enabled;
#[no_mangle]
unsafe extern "C" fn sysrq_on() -> bool {
    static bool sysrq_on(void)
    {
    return sysrq_enabled || sysrq_always_enabled;
    }
//
// sysrq_mask - Getter for sysrq_enabled mask.
//
// Return: 1 if sysrq is always enabled, enabled sysrq_key_op mask otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn sysrq_mask() -> c_int {
    int sysrq_mask(void)
    {
    if (sysrq_always_enabled)
    return 1;
    return sysrq_enabled;
    }
    EXPORT_SYMBOL_GPL(sysrq_mask);
//
// A value of 1 means 'all', other nonzero values are an op mask:
//
#[no_mangle]
unsafe extern "C" fn sysrq_on_mask(mask: c_int) -> bool {
    static bool sysrq_on_mask(int mask)
    {
    return sysrq_always_enabled ||
    sysrq_enabled == 1 ||
    (sysrq_enabled & mask);
    }
#[no_mangle]
unsafe extern "C" fn sysrq_always_enabled_setup(str: *mut c_char) -> int __init {
    static int __init sysrq_always_enabled_setup(char *str)
    {
    sysrq_always_enabled = true;
    pr_info("sysrq always enabled.\n");
    return 1;
    }
    __setup("sysrq_always_enabled", sysrq_always_enabled_setup);
#[no_mangle]
unsafe extern "C" fn sysrq_handle_loglevel(key: u8) {
    static void sysrq_handle_loglevel(u8 key)
    {
    let mut loglevel: u8 = key - '0';
    console_loglevel = CONSOLE_LOGLEVEL_DEFAULT;
    pr_info("Loglevel set to %u\n", loglevel);
    console_loglevel = loglevel;
    }
    static const struct sysrq_key_op sysrq_loglevel_op = {
    .handler	= sysrq_handle_loglevel,
    .help_msg	= "loglevel(0-9)",
    .action_msg	= "Changing Loglevel",
    .enable_mask	= SYSRQ_ENABLE_LOG,
    };

#[no_mangle]
unsafe extern "C" fn sysrq_handle_SAK(key: u8) {
    static void sysrq_handle_SAK(u8 key)
    {
    struct work_struct *SAK_work = &vc_cons[fg_console].SAK_work;
    schedule_work(SAK_work);
    }
    static const struct sysrq_key_op sysrq_SAK_op = {
    .handler	= sysrq_handle_SAK,
    .help_msg	= "sak(k)",
    .action_msg	= "SAK",
    .enable_mask	= SYSRQ_ENABLE_KEYBOARD,
    };

#[no_mangle]
unsafe extern "C" fn sysrq_handle_unraw(key: u8) {
    static void sysrq_handle_unraw(u8 key)
    {
    vt_reset_unicode(fg_console);
    }
    static const struct sysrq_key_op sysrq_unraw_op = {
    .handler	= sysrq_handle_unraw,
    .help_msg	= "unraw(r)",
    .action_msg	= "Keyboard mode set to system default",
    .enable_mask	= SYSRQ_ENABLE_KEYBOARD,
    };

#[no_mangle]
unsafe extern "C" fn sysrq_handle_crash(key: u8) {
    static void sysrq_handle_crash(u8 key)
    {
// release the RCU read lock before crashing
    rcu_read_unlock();
    panic("sysrq triggered crash\n");
    }
    static const struct sysrq_key_op sysrq_crash_op = {
    .handler	= sysrq_handle_crash,
    .help_msg	= "crash(c)",
    .action_msg	= "Trigger a crash",
    .enable_mask	= SYSRQ_ENABLE_DUMP,
    };
#[no_mangle]
unsafe extern "C" fn sysrq_handle_reboot(key: u8) {
    static void sysrq_handle_reboot(u8 key)
    {
    lockdep_off();
    local_irq_enable();
    emergency_restart();
    }
    static const struct sysrq_key_op sysrq_reboot_op = {
    .handler	= sysrq_handle_reboot,
    .help_msg	= "reboot(b)",
    .action_msg	= "Resetting",
    .enable_mask	= SYSRQ_ENABLE_BOOT,
    };
    const struct sysrq_key_op *__sysrq_reboot_op = &sysrq_reboot_op;
#[no_mangle]
unsafe extern "C" fn sysrq_handle_sync(key: u8) {
    static void sysrq_handle_sync(u8 key)
    {
    emergency_sync();
    }
    static const struct sysrq_key_op sysrq_sync_op = {
    .handler	= sysrq_handle_sync,
    .help_msg	= "sync(s)",
    .action_msg	= "Emergency Sync",
    .enable_mask	= SYSRQ_ENABLE_SYNC,
    };
#[no_mangle]
unsafe extern "C" fn sysrq_handle_show_timers(key: u8) {
    static void sysrq_handle_show_timers(u8 key)
    {
    sysrq_timer_list_show();
    }
    static const struct sysrq_key_op sysrq_show_timers_op = {
    .handler	= sysrq_handle_show_timers,
    .help_msg	= "show-all-timers(q)",
    .action_msg	= "Show clockevent devices & pending hrtimers (no others)",
    };
#[no_mangle]
unsafe extern "C" fn sysrq_handle_mountro(key: u8) {
    static void sysrq_handle_mountro(u8 key)
    {
    emergency_remount();
    }
    static const struct sysrq_key_op sysrq_mountro_op = {
    .handler	= sysrq_handle_mountro,
    .help_msg	= "unmount(u)",
    .action_msg	= "Emergency Remount R/O",
    .enable_mask	= SYSRQ_ENABLE_REMOUNT,
    };

#[no_mangle]
unsafe extern "C" fn sysrq_handle_showlocks(key: u8) {
    static void sysrq_handle_showlocks(u8 key)
    {
    debug_show_all_locks();
    }
    static const struct sysrq_key_op sysrq_showlocks_op = {
    .handler	= sysrq_handle_showlocks,
    .help_msg	= "show-all-locks(d)",
    .action_msg	= "Show Locks Held",
    };

    static DEFINE_RAW_SPINLOCK(show_lock);
#[no_mangle]
unsafe extern "C" fn showacpu(dummy: *mut c_void) {
    static void showacpu(void *dummy)
    {
    unsigned long flags;
// Idle CPUs have no interesting backtrace.
    if (idle_cpu(smp_processor_id())) {
    pr_info("CPU%d: backtrace skipped as idling\n", smp_processor_id());
    return;
    }
    raw_spin_lock_irqsave(&show_lock, flags);
    pr_info("CPU%d:\n", smp_processor_id());
    show_stack(core::ptr::null_mut(), core::ptr::null_mut(), KERN_INFO);
    raw_spin_unlock_irqrestore(&show_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn sysrq_showregs_othercpus(dummy: *mut work_struct) {
    static void sysrq_showregs_othercpus(struct work_struct *dummy)
    {
    smp_call_function(showacpu, core::ptr::null_mut(), 0);
    }
    static DECLARE_WORK(sysrq_showallcpus, sysrq_showregs_othercpus);
#[no_mangle]
unsafe extern "C" fn sysrq_handle_showallcpus(key: u8) {
    static void sysrq_handle_showallcpus(u8 key)
    {
//
// Fall back to the workqueue based printing if the
// backtrace printing did not succeed or the
// architecture has no support for it:
//
    if (!trigger_all_cpu_backtrace()) {
    struct pt_regs *regs = core::ptr::null_mut();
    if (in_hardirq())
    regs = get_irq_regs();
    pr_info("CPU%d:\n", get_cpu());
    if (regs)
    show_regs(regs);
    else
    show_stack(core::ptr::null_mut(), core::ptr::null_mut(), KERN_INFO);
    schedule_work(&sysrq_showallcpus);
    put_cpu();
    }
    }
    static const struct sysrq_key_op sysrq_showallcpus_op = {
    .handler	= sysrq_handle_showallcpus,
    .help_msg	= "show-backtrace-all-active-cpus(l)",
    .action_msg	= "Show backtrace of all active CPUs",
    .enable_mask	= SYSRQ_ENABLE_DUMP,
    };

#[no_mangle]
unsafe extern "C" fn sysrq_handle_showregs(key: u8) {
    static void sysrq_handle_showregs(u8 key)
    {
    struct pt_regs *regs = core::ptr::null_mut();
    if (in_hardirq())
    regs = get_irq_regs();
    if (regs)
    show_regs(regs);
    perf_event_print_debug();
    }
    static const struct sysrq_key_op sysrq_showregs_op = {
    .handler	= sysrq_handle_showregs,
    .help_msg	= "show-registers(p)",
    .action_msg	= "Show Regs",
    .enable_mask	= SYSRQ_ENABLE_DUMP,
    };
#[no_mangle]
unsafe extern "C" fn sysrq_handle_showstate(key: u8) {
    static void sysrq_handle_showstate(u8 key)
    {
    show_state();
    show_all_workqueues();
    }
    static const struct sysrq_key_op sysrq_showstate_op = {
    .handler	= sysrq_handle_showstate,
    .help_msg	= "show-task-states(t)",
    .action_msg	= "Show State",
    .enable_mask	= SYSRQ_ENABLE_DUMP,
    };
#[no_mangle]
unsafe extern "C" fn sysrq_handle_showstate_blocked(key: u8) {
    static void sysrq_handle_showstate_blocked(u8 key)
    {
    show_state_filter(TASK_UNINTERRUPTIBLE);
    }
    static const struct sysrq_key_op sysrq_showstate_blocked_op = {
    .handler	= sysrq_handle_showstate_blocked,
    .help_msg	= "show-blocked-tasks(w)",
    .action_msg	= "Show Blocked State",
    .enable_mask	= SYSRQ_ENABLE_DUMP,
    };

#[no_mangle]
unsafe extern "C" fn sysrq_ftrace_dump(key: u8) {
    static void sysrq_ftrace_dump(u8 key)
    {
    ftrace_dump(DUMP_ALL);
    }
    static const struct sysrq_key_op sysrq_ftrace_dump_op = {
    .handler	= sysrq_ftrace_dump,
    .help_msg	= "dump-ftrace-buffer(z)",
    .action_msg	= "Dump ftrace buffer",
    .enable_mask	= SYSRQ_ENABLE_DUMP,
    };

#[no_mangle]
unsafe extern "C" fn sysrq_handle_showmem(key: u8) {
    static void sysrq_handle_showmem(u8 key)
    {
    show_mem();
    }
    static const struct sysrq_key_op sysrq_showmem_op = {
    .handler	= sysrq_handle_showmem,
    .help_msg	= "show-memory-usage(m)",
    .action_msg	= "Show Memory",
    .enable_mask	= SYSRQ_ENABLE_DUMP,
    };
//
// Signal sysrq helper function.  Sends a signal to all user processes.
//
#[no_mangle]
unsafe extern "C" fn send_sig_all(sig: c_int) {
    static void send_sig_all(int sig)
    {
    struct task_struct *p;
    read_lock(&tasklist_lock);
    for_each_process(p) {
    if (p.flags & PF_KTHREAD)
    continue;
    if (is_global_init(p))
    continue;
    do_send_sig_info(sig, SEND_SIG_PRIV, p, PIDTYPE_MAX);
    }
    read_unlock(&tasklist_lock);
    }
#[no_mangle]
unsafe extern "C" fn sysrq_handle_term(key: u8) {
    static void sysrq_handle_term(u8 key)
    {
    send_sig_all(SIGTERM);
    console_loglevel = CONSOLE_LOGLEVEL_DEBUG;
    }
    static const struct sysrq_key_op sysrq_term_op = {
    .handler	= sysrq_handle_term,
    .help_msg	= "terminate-all-tasks(e)",
    .action_msg	= "Terminate All Tasks",
    .enable_mask	= SYSRQ_ENABLE_SIGNAL,
    };
#[no_mangle]
unsafe extern "C" fn moom_callback(ignored: *mut work_struct) {
    static void moom_callback(struct work_struct *ignored)
    {
    let mut gfp_mask: gfp_t = GFP_KERNEL;
    struct oom_control oc = {
    .zonelist = node_zonelist(first_memory_node, gfp_mask),
    .nodemask = core::ptr::null_mut(),
    .memcg = core::ptr::null_mut(),
    .gfp_mask = gfp_mask,
    .order = -1,
    };
    mutex_lock(&oom_lock);
    if (!out_of_memory(&oc))
    pr_info("OOM request ignored. No task eligible\n");
    mutex_unlock(&oom_lock);
    }
    static DECLARE_WORK(moom_work, moom_callback);
#[no_mangle]
unsafe extern "C" fn sysrq_handle_moom(key: u8) {
    static void sysrq_handle_moom(u8 key)
    {
    schedule_work(&moom_work);
    }
    static const struct sysrq_key_op sysrq_moom_op = {
    .handler	= sysrq_handle_moom,
    .help_msg	= "memory-full-oom-kill(f)",
    .action_msg	= "Manual OOM execution",
    .enable_mask	= SYSRQ_ENABLE_SIGNAL,
    };

#[no_mangle]
unsafe extern "C" fn sysrq_handle_thaw(key: u8) {
    static void sysrq_handle_thaw(u8 key)
    {
    emergency_thaw_all();
    }
    static const struct sysrq_key_op sysrq_thaw_op = {
    .handler	= sysrq_handle_thaw,
    .help_msg	= "thaw-filesystems(j)",
    .action_msg	= "Emergency Thaw of all frozen filesystems",
    .enable_mask	= SYSRQ_ENABLE_SIGNAL,
    };

#[no_mangle]
unsafe extern "C" fn sysrq_handle_kill(key: u8) {
    static void sysrq_handle_kill(u8 key)
    {
    send_sig_all(SIGKILL);
    console_loglevel = CONSOLE_LOGLEVEL_DEBUG;
    }
    static const struct sysrq_key_op sysrq_kill_op = {
    .handler	= sysrq_handle_kill,
    .help_msg	= "kill-all-tasks(i)",
    .action_msg	= "Kill All Tasks",
    .enable_mask	= SYSRQ_ENABLE_SIGNAL,
    };
#[no_mangle]
unsafe extern "C" fn sysrq_handle_unrt(key: u8) {
    static void sysrq_handle_unrt(u8 key)
    {
    normalize_rt_tasks();
    }
    static const struct sysrq_key_op sysrq_unrt_op = {
    .handler	= sysrq_handle_unrt,
    .help_msg	= "nice-all-RT-tasks(n)",
    .action_msg	= "Nice All RT Tasks",
    .enable_mask	= SYSRQ_ENABLE_RTNICE,
    };
#[no_mangle]
unsafe extern "C" fn sysrq_handle_replay_logs(key: u8) {
    static void sysrq_handle_replay_logs(u8 key)
    {
    console_try_replay_all();
    }
    static struct sysrq_key_op sysrq_replay_logs_op = {
    .handler        = sysrq_handle_replay_logs,
    .help_msg       = "replay-kernel-logs(R)",
    .action_msg     = "Replay kernel logs on consoles",
    .enable_mask    = SYSRQ_ENABLE_DUMP,
    };
// Key Operations table and lock
    static DEFINE_SPINLOCK(sysrq_key_table_lock);
    static const struct sysrq_key_op *sysrq_key_table[62] = {
    &sysrq_loglevel_op,		/* 0 */
    &sysrq_loglevel_op,		/* 1 */
    &sysrq_loglevel_op,		/* 2 */
    &sysrq_loglevel_op,		/* 3 */
    &sysrq_loglevel_op,		/* 4 */
    &sysrq_loglevel_op,		/* 5 */
    &sysrq_loglevel_op,		/* 6 */
    &sysrq_loglevel_op,		/* 7 */
    &sysrq_loglevel_op,		/* 8 */
    &sysrq_loglevel_op,		/* 9 */
//
// a: Don't use for system provided sysrqs, it is handled specially on
// sparc and will never arrive.
//
    core::ptr::null_mut(),				/* a */
    &sysrq_reboot_op,		/* b */
    &sysrq_crash_op,		/* c */
    &sysrq_showlocks_op,		/* d */
    &sysrq_term_op,			/* e */
    &sysrq_moom_op,			/* f */
// g: May be registered for the kernel debugger
    core::ptr::null_mut(),				/* g */
    core::ptr::null_mut(),				/* h - reserved for help */
    &sysrq_kill_op,			/* i */
    &sysrq_thaw_op,			/* j */
    &sysrq_SAK_op,			/* k */
    &sysrq_showallcpus_op,		/* l */
    &sysrq_showmem_op,		/* m */
    &sysrq_unrt_op,			/* n */
// o: This will often be registered as 'Off' at init time
    core::ptr::null_mut(),				/* o */
    &sysrq_showregs_op,		/* p */
    &sysrq_show_timers_op,		/* q */
    &sysrq_unraw_op,		/* r */
    &sysrq_sync_op,			/* s */
    &sysrq_showstate_op,		/* t */
    &sysrq_mountro_op,		/* u */
// v: May be registered for frame buffer console restore
    core::ptr::null_mut(),				/* v */
    &sysrq_showstate_blocked_op,	/* w */
// x: May be registered on mips for TLB dump
// x: May be registered on ppc/powerpc for xmon
// x: May be registered on sparc64 for global PMU dump
    core::ptr::null_mut(),				/* x */
// y: May be registered on sparc64 for global register dump
    core::ptr::null_mut(),				/* y */
    &sysrq_ftrace_dump_op,		/* z */
    core::ptr::null_mut(),				/* A */
    core::ptr::null_mut(),				/* B */
    core::ptr::null_mut(),				/* C */
    core::ptr::null_mut(),				/* D */
    core::ptr::null_mut(),				/* E */
    core::ptr::null_mut(),				/* F */
    core::ptr::null_mut(),				/* G */
    core::ptr::null_mut(),				/* H */
    core::ptr::null_mut(),				/* I */
    core::ptr::null_mut(),				/* J */
    core::ptr::null_mut(),				/* K */
    core::ptr::null_mut(),				/* L */
    core::ptr::null_mut(),				/* M */
    core::ptr::null_mut(),				/* N */
    core::ptr::null_mut(),				/* O */
    core::ptr::null_mut(),				/* P */
    core::ptr::null_mut(),				/* Q */
    &sysrq_replay_logs_op,		/* R */
// S: May be registered by sched_ext for resetting
    core::ptr::null_mut(),				/* S */
    core::ptr::null_mut(),				/* T */
    core::ptr::null_mut(),				/* U */
    core::ptr::null_mut(),				/* V */
    core::ptr::null_mut(),				/* W */
    core::ptr::null_mut(),				/* X */
    core::ptr::null_mut(),				/* Y */
    core::ptr::null_mut(),				/* Z */
    };
// key2index calculation, -1 on invalid index
#[no_mangle]
unsafe extern "C" fn sysrq_key_table_key2index(key: u8) -> c_int {
    static int sysrq_key_table_key2index(u8 key)
    {
    switch (key) {
    case '0' ... '9':
    return key - '0';
    case 'a' ... 'z':
    return key - 'a' + 10;
    case 'A' ... 'Z':
    return key - 'A' + 10 + 26;
    default:
    return -1;
    }
    }
//
// get and put functions for the table, exposed to modules.
//
    static const struct sysrq_key_op *__sysrq_get_key_op(u8 key)
    {
    const struct sysrq_key_op *op_p = core::ptr::null_mut();
    int i;
    i = sysrq_key_table_key2index(key);
    if (i != -1)
    op_p = sysrq_key_table[i];
    return op_p;
    }
#[no_mangle]
unsafe extern "C" fn __sysrq_put_key_op(key: u8, op_p: *const sysrq_key_op) {
    static void __sysrq_put_key_op(u8 key, const struct sysrq_key_op *op_p)
    {
    let mut i: c_int = sysrq_key_table_key2index(key);
    if (i != -1)
    sysrq_key_table[i] = op_p;
    }
#[no_mangle]
pub unsafe extern "C" fn __handle_sysrq(key: u8, check_mask: bool) {
    void __handle_sysrq(u8 key, bool check_mask)
    {
    const struct sysrq_key_op *op_p;
    int orig_suppress_printk;
    int i;
    orig_suppress_printk = suppress_printk;
    suppress_printk = 0;
    rcu_sysrq_start();
    rcu_read_lock();
//
// Enter in the force_console context so that sysrq header is shown to
// provide the user with positive feedback.  We do not simply emit this
// at KERN_EMERG as that would change message routing in the consumers
// of /proc/kmsg.
//
    printk_force_console_enter();
    op_p = __sysrq_get_key_op(key);
    if (op_p) {
//
// Should we check for enabled operations (/proc/sysrq-trigger
// should not) and is the invoked operation enabled?
//
    if (!check_mask || sysrq_on_mask(op_p.enable_mask)) {
    pr_info("%s\n", op_p.action_msg);
    printk_force_console_exit();
    op_p.handler(key);
    } else {
    pr_info("This sysrq operation is disabled.\n");
    printk_force_console_exit();
    }
    } else {
    pr_info("HELP : ");
// Only print the help msg once per handler
    for (i = 0; i < ARRAY_SIZE(sysrq_key_table); i++) {
    if (sysrq_key_table[i]) {
    int j;
    for (j = 0; sysrq_key_table[i] !=
    sysrq_key_table[j]; j++)
    ;
    if (j != i)
    continue;
    pr_cont("%s ", sysrq_key_table[i].help_msg);
    }
    }
    pr_cont("\n");
    printk_force_console_exit();
    }
    rcu_read_unlock();
    rcu_sysrq_end();
    suppress_printk = orig_suppress_printk;
    }
#[no_mangle]
pub unsafe extern "C" fn handle_sysrq(key: u8) {
    void handle_sysrq(u8 key)
    {
    if (sysrq_on())
    __handle_sysrq(key, true);
    }
    EXPORT_SYMBOL(handle_sysrq);

    static int sysrq_reset_downtime_ms;
// Simple translation table for the SysRq keys
    static const unsigned char sysrq_xlate[KEY_CNT] =
    "\000\0331234567890-=\177\t"                    /* 0x00 - 0x0f */
    "qwertyuiop[]\r\000as"                          /* 0x10 - 0x1f */
    "dfghjkl;'`\000\\zxcv"                          /* 0x20 - 0x2f */
    "bnm,./\000*\000 \000\201\202\203\204\205"      /* 0x30 - 0x3f */
    "\206\207\210\211\212\000\000789-456+1"         /* 0x40 - 0x4f */
    "230\177\000\000\213\214\000\000\000\000\000\000\000\000\000\000" /* 0x50 - 0x5f */
    "\r\000/";                                      /* 0x60 - 0x6f */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sysrq_state {
    pub handle: input_handle,
    pub reinject_work: work_struct,
    pub key_down: [c_ulong; BITS_TO_LONGS(KEY_CNT)],
    pub alt: c_uint,
    pub alt_use: c_uint,
    pub shift: c_uint,
    pub shift_use: c_uint,
    pub active: bool,
    pub need_reinject: bool,
    pub reinjecting: bool,
// reset sequence handling
    pub reset_canceled: bool,
    pub reset_requested: bool,
    pub reset_keybit: [c_ulong; BITS_TO_LONGS(KEY_CNT)],
    pub reset_seq_len: c_int,
    pub reset_seq_cnt: c_int,
    pub reset_seq_version: c_int,
    pub keyreset_timer: timer_list,
}

    static unsigned short sysrq_reset_seq[SYSRQ_KEY_RESET_MAX];
    static unsigned int sysrq_reset_seq_len;
    let mut sysrq_reset_seq_version: static unsigned int = 1;
#[no_mangle]
unsafe extern "C" fn sysrq_parse_reset_sequence(state: *mut sysrq_state) {
    static void sysrq_parse_reset_sequence(struct sysrq_state *state)
    {
    int i;
    unsigned short key;
    state.reset_seq_cnt = 0;
    for (i = 0; i < sysrq_reset_seq_len; i++) {
    key = sysrq_reset_seq[i];
    if (key == KEY_RESERVED || key > KEY_MAX)
    break;
    __set_bit(key, state.reset_keybit);
    state.reset_seq_len++;
    if (test_bit(key, state.key_down))
    state.reset_seq_cnt++;
    }
// Disable reset until old keys are not released
    state.reset_canceled = state.reset_seq_cnt != 0;
    state.reset_seq_version = sysrq_reset_seq_version;
    }
#[no_mangle]
unsafe extern "C" fn sysrq_do_reset(t: *mut timer_list) {
    static void sysrq_do_reset(struct timer_list *t)
    {
    struct sysrq_state *state = timer_container_of(state, t,
    keyreset_timer);
    state.reset_requested = true;
    orderly_reboot();
    }
#[no_mangle]
unsafe extern "C" fn sysrq_handle_reset_request(state: *mut sysrq_state) {
    static void sysrq_handle_reset_request(struct sysrq_state *state)
    {
    if (state.reset_requested)
    __handle_sysrq(sysrq_xlate[KEY_B], false);
    if (sysrq_reset_downtime_ms)
    mod_timer(&state.keyreset_timer,
    jiffies + msecs_to_jiffies(sysrq_reset_downtime_ms));
    else
    sysrq_do_reset(&state.keyreset_timer);
    }
    static void sysrq_detect_reset_sequence(struct sysrq_state *state,
    unsigned int code, int value)
    {
    if (!test_bit(code, state.reset_keybit)) {
//
// Pressing any key _not_ in reset sequence cancels
// the reset sequence.  Also cancelling the timer in
// case additional keys were pressed after a reset
// has been requested.
//
    if (value && state.reset_seq_cnt) {
    state.reset_canceled = true;
    timer_delete(&state.keyreset_timer);
    }
    } else if (value == 0) {
//
// Key release - all keys in the reset sequence need
// to be pressed and held for the reset timeout
// to hold.
//
    timer_delete(&state.keyreset_timer);
    if (--state.reset_seq_cnt == 0)
    state.reset_canceled = false;
    } else if (value == 1) {
// key press, not autorepeat
    if (++state.reset_seq_cnt == state.reset_seq_len &&
    !state.reset_canceled) {
    sysrq_handle_reset_request(state);
    }
    }
    }

#[no_mangle]
unsafe extern "C" fn sysrq_of_get_keyreset_config() {
    static void sysrq_of_get_keyreset_config(void)
    {
    u32 key;
    struct device_node *np;
    np = of_find_node_by_path("/chosen/linux,sysrq-reset-seq");
    if (!np) {
    pr_debug("No sysrq node found");
    return;
    }
// Reset in case a __weak definition was present
    sysrq_reset_seq_len = 0;
    of_property_for_each_u32(np, "keyset", key) {
    if (key == KEY_RESERVED || key > KEY_MAX ||
    sysrq_reset_seq_len == SYSRQ_KEY_RESET_MAX)
    break;
    sysrq_reset_seq[sysrq_reset_seq_len++] = (unsigned short)key;
    }
// Get reset timeout if any.
    of_property_read_u32(np, "timeout-ms", &sysrq_reset_downtime_ms);
    of_node_put(np);
    }

#[no_mangle]
unsafe extern "C" fn sysrq_of_get_keyreset_config() {
    static void sysrq_of_get_keyreset_config(void)
    {
    }

#[no_mangle]
unsafe extern "C" fn sysrq_reinject_alt_sysrq(work: *mut work_struct) {
    static void sysrq_reinject_alt_sysrq(struct work_struct *work)
    {
    struct sysrq_state *sysrq =
    container_of(work, struct sysrq_state, reinject_work);
    struct input_handle *handle = &sysrq.handle;
    let mut alt_code: c_uint = sysrq.alt_use;
    if (sysrq.need_reinject) {
// we do not want the assignment to be reordered
    sysrq.reinjecting = true;
    mb();
// Simulate press and release of Alt + SysRq
    input_inject_event(handle, EV_KEY, alt_code, 1);
    input_inject_event(handle, EV_KEY, KEY_SYSRQ, 1);
    input_inject_event(handle, EV_SYN, SYN_REPORT, 1);
    input_inject_event(handle, EV_KEY, KEY_SYSRQ, 0);
    input_inject_event(handle, EV_KEY, alt_code, 0);
    input_inject_event(handle, EV_SYN, SYN_REPORT, 1);
    mb();
    sysrq.reinjecting = false;
    }
    }
    static bool sysrq_handle_keypress(struct sysrq_state *sysrq,
    unsigned int code, int value)
    {
    let mut was_active: bool = sysrq.active;
    bool suppress;
    switch (code) {
    case KEY_LEFTALT:
    case KEY_RIGHTALT:
    if (!value) {
// One of ALTs is being released
    if (sysrq.active && code == sysrq.alt_use)
    sysrq.active = false;
    sysrq.alt = KEY_RESERVED;
    } else if (value != 2) {
    sysrq.alt = code;
    sysrq.need_reinject = false;
    }
    break;
    case KEY_LEFTSHIFT:
    case KEY_RIGHTSHIFT:
    if (!value)
    sysrq.shift = KEY_RESERVED;
#[no_mangle]
pub unsafe extern "C" fn if(2: value !=) -> else {
    else if (value != 2)
    sysrq.shift = code;
    if (sysrq.active)
    sysrq.shift_use = sysrq.shift;
    break;
    case KEY_SYSRQ:
    if (value == 1 && sysrq.alt != KEY_RESERVED) {
    sysrq.active = true;
    sysrq.alt_use = sysrq.alt;
// either RESERVED (for released) or actual code
    sysrq.shift_use = sysrq.shift;
//
// If nothing else will be pressed we'll need
// to re-inject Alt-SysRq keysroke.
//
    sysrq.need_reinject = true;
    }
//
// Pretend that sysrq was never pressed at all. This
// is needed to properly handle KGDB which will try
// to release all keys after exiting debugger. If we
// do not clear key bit it KGDB will end up sending
// release events for Alt and SysRq, potentially
// triggering print screen function.
//
    if (sysrq.active)
    clear_bit(KEY_SYSRQ, sysrq.handle.dev.key);
    break;
    default:
    if (sysrq.active && value && value != 2) {
    let mut c: c_uchar = sysrq_xlate[code];
    sysrq.need_reinject = false;
    if (sysrq.shift_use != KEY_RESERVED)
    c = toupper(c);
    __handle_sysrq(c, true);
    }
    break;
    }
    suppress = sysrq.active;
    if (!sysrq.active) {
//
// See if reset sequence has changed since the last time.
//
    if (sysrq.reset_seq_version != sysrq_reset_seq_version)
    sysrq_parse_reset_sequence(sysrq);
//
// If we are not suppressing key presses keep track of
// keyboard state so we can release keys that have been
// pressed before entering SysRq mode.
//
    if (value)
    set_bit(code, sysrq.key_down);
    else
    clear_bit(code, sysrq.key_down);
    if (was_active)
    schedule_work(&sysrq.reinject_work);
// Check for reset sequence
    sysrq_detect_reset_sequence(sysrq, code, value);
    } else if (value == 0 && test_and_clear_bit(code, sysrq.key_down)) {
//
// Pass on release events for keys that was pressed before
// entering SysRq mode.
//
    suppress = false;
    }
    return suppress;
    }
    static bool sysrq_filter(struct input_handle *handle,
    unsigned int type, unsigned int code, int value)
    {
    struct sysrq_state *sysrq = handle.private;
    bool suppress;
//
// Do not filter anything if we are in the process of re-injecting
// Alt+SysRq combination.
//
    if (sysrq.reinjecting)
    return false;
    switch (type) {
    case EV_SYN:
    suppress = false;
    break;
    case EV_KEY:
    suppress = sysrq_handle_keypress(sysrq, code, value);
    break;
    default:
    suppress = sysrq.active;
    break;
    }
    return suppress;
    }
    static int sysrq_connect(struct input_handler *handler,
    struct input_dev *dev,
    const struct input_device_id *id)
    {
    struct sysrq_state *sysrq;
    int error;
    sysrq = kzalloc_obj(struct sysrq_state);
    if (!sysrq)
    return -ENOMEM;
    INIT_WORK(&sysrq.reinject_work, sysrq_reinject_alt_sysrq);
    sysrq.handle.dev = dev;
    sysrq.handle.handler = handler;
    sysrq.handle.name = "sysrq";
    sysrq.handle.private = sysrq;
    timer_setup(&sysrq.keyreset_timer, sysrq_do_reset, 0);
    error = input_register_handle(&sysrq.handle);
    if (error) {
    pr_err("Failed to register input sysrq handler, error %d\n",
    error);
    goto err_free;
    }
    error = input_open_device(&sysrq.handle);
    if (error) {
    pr_err("Failed to open input device, error %d\n", error);
    goto err_unregister;
    }
    return 0;
    err_unregister:
    input_unregister_handle(&sysrq.handle);
    err_free:
    kfree(sysrq);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn sysrq_disconnect(handle: *mut input_handle) {
    static void sysrq_disconnect(struct input_handle *handle)
    {
    struct sysrq_state *sysrq = handle.private;
    input_close_device(handle);
    cancel_work_sync(&sysrq.reinject_work);
    timer_shutdown_sync(&sysrq.keyreset_timer);
    input_unregister_handle(handle);
    kfree(sysrq);
    }
//
// We are matching on KEY_LEFTALT instead of KEY_SYSRQ because not all
// keyboards have SysRq key predefined and so user may add it to keymap
// later, but we expect all such keyboards to have left alt.
//
    static const struct input_device_id sysrq_ids[] = {
    {
    .flags = INPUT_DEVICE_ID_MATCH_EVBIT |
    INPUT_DEVICE_ID_MATCH_KEYBIT,
    .evbit = { [BIT_WORD(EV_KEY)] = BIT_MASK(EV_KEY) },
    .keybit = { [BIT_WORD(KEY_LEFTALT)] = BIT_MASK(KEY_LEFTALT) },
    },
    { },
    };
    static struct input_handler sysrq_handler = {
    .filter		= sysrq_filter,
    .connect	= sysrq_connect,
    .disconnect	= sysrq_disconnect,
    .name		= "sysrq",
    .id_table	= sysrq_ids,
    };
#[no_mangle]
pub unsafe extern "C" fn sysrq_register_handler() {
    static inline void sysrq_register_handler(void)
    {
    int error;
    sysrq_of_get_keyreset_config();
    error = input_register_handler(&sysrq_handler);
    if (error)
    pr_err("Failed to register input handler, error %d", error);
    }
#[no_mangle]
pub unsafe extern "C" fn sysrq_unregister_handler() {
    static inline void sysrq_unregister_handler(void)
    {
    input_unregister_handler(&sysrq_handler);
    }
    static int sysrq_reset_seq_param_set(const char *buffer,
    const struct kernel_param *kp)
    {
    unsigned long val;
    int error;
    error = kstrtoul(buffer, 0, &val);
    if (error < 0)
    return error;
    if (val > KEY_MAX)
    return -EINVAL;
// ((unsigned short *)kp->arg) = val;
    sysrq_reset_seq_version++;
    return 0;
    }
    static const struct kernel_param_ops param_ops_sysrq_reset_seq = {
    .get	= param_get_ushort,
    .set	= sysrq_reset_seq_param_set,
    };

    __param_check(name, p, unsigned short)
//
// not really modular, but the easiest way to keep compat with existing
// bootargs behaviour is to continue using module_param here.
//
    module_param_array_named(reset_seq, sysrq_reset_seq, sysrq_reset_seq,
    &sysrq_reset_seq_len, 0644);
    module_param_named(sysrq_downtime_ms, sysrq_reset_downtime_ms, int, 0644);

#[no_mangle]
pub unsafe extern "C" fn sysrq_register_handler() {
    static inline void sysrq_register_handler(void)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn sysrq_unregister_handler() {
    static inline void sysrq_unregister_handler(void)
    {
    }

#[no_mangle]
pub unsafe extern "C" fn sysrq_toggle_support(enable_mask: c_int) -> c_int {
    int sysrq_toggle_support(int enable_mask)
    {
    let mut was_enabled: bool = sysrq_on();
    sysrq_enabled = enable_mask;
    if (was_enabled != sysrq_on()) {
    if (sysrq_on())
    sysrq_register_handler();
    else
    sysrq_unregister_handler();
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(sysrq_toggle_support);
    static int sysrq_sysctl_handler(const struct ctl_table *table, int write,
    void *buffer, size_t *lenp, loff_t *ppos)
    {
    int tmp, ret;
    let mut t: ctl_table = *table;
    tmp = sysrq_mask();
    t.data = &tmp;
//
// Behaves like do_proc_dointvec as t does not have min nor max.
//
    ret = proc_dointvec_minmax(&t, write, buffer, lenp, ppos);
    if (ret)
    return ret;
    if (write)
    sysrq_toggle_support(tmp);
    return 0;
    }
    static const struct ctl_table sysrq_sysctl_table[] = {
    {
    .procname	= "sysrq",
    .data		= core::ptr::null_mut(),
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= sysrq_sysctl_handler,
    },
    };
#[no_mangle]
unsafe extern "C" fn init_sysrq_sysctl() -> int __init {
    static int __init init_sysrq_sysctl(void)
    {
    register_sysctl_init("kernel", sysrq_sysctl_table);
    return 0;
    }
    subsys_initcall(init_sysrq_sysctl);
    static int __sysrq_swap_key_ops(u8 key, const struct sysrq_key_op *insert_op_p,
    const struct sysrq_key_op *remove_op_p)
    {
    int retval;
    spin_lock(&sysrq_key_table_lock);
    if (__sysrq_get_key_op(key) == remove_op_p) {
    __sysrq_put_key_op(key, insert_op_p);
    retval = 0;
    } else {
    retval = -1;
    }
    spin_unlock(&sysrq_key_table_lock);
//
// A concurrent __handle_sysrq either got the old op or the new op.
// Wait for it to go away before returning, so the code for an old
// op is not freed (eg. on module unload) while it is in use.
//
    synchronize_rcu();
    return retval;
    }
#[no_mangle]
pub unsafe extern "C" fn register_sysrq_key(key: u8, op_p: *const sysrq_key_op) -> c_int {
    int register_sysrq_key(u8 key, const struct sysrq_key_op *op_p)
    {
    return __sysrq_swap_key_ops(key, op_p, core::ptr::null_mut());
    }
    EXPORT_SYMBOL(register_sysrq_key);
#[no_mangle]
pub unsafe extern "C" fn unregister_sysrq_key(key: u8, op_p: *const sysrq_key_op) -> c_int {
    int unregister_sysrq_key(u8 key, const struct sysrq_key_op *op_p)
    {
    return __sysrq_swap_key_ops(key, core::ptr::null_mut(), op_p);
    }
    EXPORT_SYMBOL(unregister_sysrq_key);

//
// writing 'C' to /proc/sysrq-trigger is like sysrq-C
// Normally, only the first character written is processed.
// However, if the first character is an underscore,
// all characters are processed.
//
    static ssize_t write_sysrq_trigger(struct file *file, const char __user *buf,
    size_t count, loff_t *ppos)
    {
    let mut bulk: bool = false;
    size_t i;
    for (i = 0; i < count; i++) {
    char c;
    if (get_user(c, buf + i))
    return -EFAULT;
    if (c == '_')
    bulk = true;
    else
    __handle_sysrq(c, false);
    if (!bulk)
    break;
    }
    return count;
    }
    static const struct proc_ops sysrq_trigger_proc_ops = {
    .proc_write	= write_sysrq_trigger,
    .proc_lseek	= noop_llseek,
    };
#[no_mangle]
unsafe extern "C" fn sysrq_init_procfs() {
    static void sysrq_init_procfs(void)
    {
    if (!proc_create("sysrq-trigger", S_IWUSR, core::ptr::null_mut(),
    &sysrq_trigger_proc_ops))
    pr_err("Failed to register proc interface\n");
    }

#[no_mangle]
pub unsafe extern "C" fn sysrq_init_procfs() {
    static inline void sysrq_init_procfs(void)
    {
    }

#[no_mangle]
unsafe extern "C" fn sysrq_init() -> int __init {
    static int __init sysrq_init(void)
    {
    sysrq_init_procfs();
    if (sysrq_on())
    sysrq_register_handler();
    return 0;
    }
    device_initcall(sysrq_init);
