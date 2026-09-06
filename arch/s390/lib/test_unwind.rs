//! Automatically rewritten from C to Rust
//! Source: arch/s390/lib/test_unwind.c
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
// Test module for unwind_for_each_frame
//

    static struct kunit *current_test;

    static bool force_bt;
    module_param_named(backtrace, force_bt, bool, 0444);
    MODULE_PARM_DESC(backtrace, "print backtraces for all tests");
//
// To avoid printk line limit split backtrace by lines
//
#[no_mangle]
unsafe extern "C" fn print_backtrace(bt: *mut c_char) {
    static void print_backtrace(char *bt)
    {
    char *p;
    while (true) {
    p = strsep(&bt, "\n");
    if (!p)
    break;
    kunit_err(current_test, "%s\n", p);
    }
    }
//
// Calls unwind_for_each_frame(task, regs, sp) and verifies that the result
// contains unwindme_func2 followed by unwindme_func1.
//
    static noinline int test_unwind(struct task_struct *task, struct pt_regs *regs,
    unsigned long sp)
    {
    int frame_count, prev_is_func2, seen_func2_func1, seen_arch_rethook_trampoline;
    let mut max_frames: c_int = 128;
    struct unwind_state state;
    let mut bt_pos: usize = 0;
    let mut ret: c_int = 0;
    char *bt;
    bt = kmalloc(BT_BUF_SIZE, GFP_ATOMIC);
    if (!bt) {
    kunit_err(current_test, "failed to allocate backtrace buffer\n");
    return -ENOMEM;
    }
// Unwind.
    frame_count = 0;
    prev_is_func2 = 0;
    seen_func2_func1 = 0;
    seen_arch_rethook_trampoline = 0;
    unwind_for_each_frame(&state, task, regs, sp) {
    let mut addr: c_ulong = unwind_get_return_address(&state);
    char sym[KSYM_SYMBOL_LEN];
    if (frame_count++ == max_frames)
    break;
    if (state.reliable && !addr) {
    kunit_err(current_test, "unwind state reliable but addr is 0\n");
    ret = -EINVAL;
    break;
    }
    sprint_symbol(sym, addr);
    if (bt_pos < BT_BUF_SIZE) {
    bt_pos += snprintf(bt + bt_pos, BT_BUF_SIZE - bt_pos,
    state.reliable ? " [%-7s%px] %pSR\n" :
    "([%-7s%px] %pSR)\n",
    stack_type_name(state.stack_info.type),
    (void *)state.sp, (void *)state.ip);
    if (bt_pos >= BT_BUF_SIZE)
    kunit_err(current_test, "backtrace buffer is too small\n");
    }
    frame_count += 1;
    if (prev_is_func2 && str_has_prefix(sym, "unwindme_func1"))
    seen_func2_func1 = 1;
    prev_is_func2 = str_has_prefix(sym, "unwindme_func2");
    if (str_has_prefix(sym, "arch_rethook_trampoline+0x0/"))
    seen_arch_rethook_trampoline = 1;
    }
// Check the results.
    if (unwind_error(&state)) {
    kunit_err(current_test, "unwind error\n");
    ret = -EINVAL;
    }
    if (!seen_func2_func1) {
    kunit_err(current_test, "unwindme_func2 and unwindme_func1 not found\n");
    ret = -EINVAL;
    }
    if (frame_count == max_frames) {
    kunit_err(current_test, "Maximum number of frames exceeded\n");
    ret = -EINVAL;
    }
    if (seen_arch_rethook_trampoline) {
    kunit_err(current_test, "arch_rethook_trampoline+0x0 in unwinding results\n");
    ret = -EINVAL;
    }
    if (ret || force_bt)
    print_backtrace(bt);
    kfree(bt);
    return ret;
    }
// State of the task being unwound.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct unwindme {
    pub flags: c_int,
    pub ret: c_int,
    pub task: *mut task_struct,
    pub task_ready: completion,
    pub task_wq: wait_queue_head_t,
    pub sp: c_ulong,
}

    static struct unwindme *unwindme;
// Values of unwindme.flags.
pub const UWM_DEFAULT: c_uint = 0x0;
pub const UWM_THREAD: c_uint = 0x1	/* Unwind a separate task. */;
pub const UWM_REGS: c_uint = 0x2	/* Pass regs to test_unwind(). */;
pub const UWM_SP: c_uint = 0x4	/* Pass sp to test_unwind(). */;
pub const UWM_CALLER: c_uint = 0x8	/* Unwind starting from caller. */;
pub const UWM_SWITCH_STACK: c_uint = 0x10	/* Use call_on_stack. */;
pub const UWM_IRQ: c_uint = 0x20	/* Unwind from irq context. */;
pub const UWM_PGM: c_uint = 0x40	/* Unwind from program check handler */;
pub const UWM_KPROBE_ON_FTRACE: c_uint = 0x80	/* Unwind from kprobe handler called via ftrace. */;
pub const UWM_FTRACE: c_uint = 0x100	/* Unwind from ftrace handler. */;
pub const UWM_KRETPROBE: c_uint = 0x200	/* Unwind through kretprobed function. */;
pub const UWM_KRETPROBE_HANDLER: c_uint = 0x400	/* Unwind from kretprobe handler. */;
#[no_mangle]
unsafe extern "C" fn fake_pt_regs() -> __always_inline struct pt_regs {
    static __always_inline struct pt_regs fake_pt_regs(void)
    {
    struct pt_regs regs;
    memset(&regs, 0, sizeof(regs));
    regs.gprs[15] = current_stack_pointer;
    asm volatile(
    "basr	%[psw_addr],0"
    : [psw_addr] "=d" (regs.psw.addr));
    return regs;
    }
#[no_mangle]
unsafe extern "C" fn kretprobe_ret_handler(ri: *mut kretprobe_instance, regs: *mut pt_regs) -> c_int {
    static int kretprobe_ret_handler(struct kretprobe_instance *ri, struct pt_regs *regs)
    {
    struct unwindme *u = unwindme;
    if (!(u.flags & UWM_KRETPROBE_HANDLER))
    return 0;
    u.ret = test_unwind(core::ptr::null_mut(), (u.flags & UWM_REGS) ? regs : core::ptr::null_mut(),
    (u.flags & UWM_SP) ? u.sp : 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_unwind_kretprobed_func(u: *mut unwindme) -> noinline notrace int {
    static noinline notrace int test_unwind_kretprobed_func(struct unwindme *u)
    {
    struct pt_regs regs;
    if (!(u.flags & UWM_KRETPROBE))
    return 0;
    regs = fake_pt_regs();
    return test_unwind(core::ptr::null_mut(), (u.flags & UWM_REGS) ? &regs : core::ptr::null_mut(),
    (u.flags & UWM_SP) ? u.sp : 0);
    }
#[no_mangle]
unsafe extern "C" fn test_unwind_kretprobed_func_caller(u: *mut unwindme) -> noinline int {
    static noinline int test_unwind_kretprobed_func_caller(struct unwindme *u)
    {
    return test_unwind_kretprobed_func(u);
    }
#[no_mangle]
unsafe extern "C" fn test_unwind_kretprobe(u: *mut unwindme) -> c_int {
    static int test_unwind_kretprobe(struct unwindme *u)
    {
    int ret;
    struct kretprobe my_kretprobe;
    if (!IS_ENABLED(CONFIG_KPROBES))
    kunit_skip(current_test, "requires CONFIG_KPROBES");
    u.ret = -1; /* make sure kprobe is called */
    unwindme = u;
    memset(&my_kretprobe, 0, sizeof(my_kretprobe));
    my_kretprobe.handler = kretprobe_ret_handler;
    my_kretprobe.maxactive = 1;
    my_kretprobe.kp.addr = (kprobe_opcode_t *)test_unwind_kretprobed_func;
    ret = register_kretprobe(&my_kretprobe);
    if (ret < 0) {
    kunit_err(current_test, "register_kretprobe failed %d\n", ret);
    return -EINVAL;
    }
    ret = test_unwind_kretprobed_func_caller(u);
    unregister_kretprobe(&my_kretprobe);
    unwindme = core::ptr::null_mut();
    if (u.flags & UWM_KRETPROBE_HANDLER)
    ret = u.ret;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn kprobe_pre_handler(p: *mut kprobe, regs: *mut pt_regs) -> c_int {
    static int kprobe_pre_handler(struct kprobe *p, struct pt_regs *regs)
    {
    struct unwindme *u = unwindme;
    u.ret = test_unwind(core::ptr::null_mut(), (u.flags & UWM_REGS) ? regs : core::ptr::null_mut(),
    (u.flags & UWM_SP) ? u.sp : 0);
    return 0;
    }
    extern const char test_unwind_kprobed_insn[];
#[no_mangle]
unsafe extern "C" fn test_unwind_kprobed_func() -> noinline void {
    static noinline void test_unwind_kprobed_func(void)
    {
    asm volatile(
    "	nopr	%%r7\n"
    "test_unwind_kprobed_insn:\n"
    "	nopr	%%r7"
    :);
    }
#[no_mangle]
unsafe extern "C" fn test_unwind_kprobe(u: *mut unwindme) -> c_int {
    static int test_unwind_kprobe(struct unwindme *u)
    {
    struct kprobe kp;
    int ret;
    if (!IS_ENABLED(CONFIG_KPROBES))
    kunit_skip(current_test, "requires CONFIG_KPROBES");
    if (!IS_ENABLED(CONFIG_KPROBES_ON_FTRACE) && u.flags & UWM_KPROBE_ON_FTRACE)
    kunit_skip(current_test, "requires CONFIG_KPROBES_ON_FTRACE");
    u.ret = -1; /* make sure kprobe is called */
    unwindme = u;
    memset(&kp, 0, sizeof(kp));
    kp.pre_handler = kprobe_pre_handler;
    kp.addr = u.flags & UWM_KPROBE_ON_FTRACE ?
    (kprobe_opcode_t *)test_unwind_kprobed_func :
    (kprobe_opcode_t *)test_unwind_kprobed_insn;
    ret = register_kprobe(&kp);
    if (ret < 0) {
    kunit_err(current_test, "register_kprobe failed %d\n", ret);
    return -EINVAL;
    }
    test_unwind_kprobed_func();
    unregister_kprobe(&kp);
    unwindme = core::ptr::null_mut();
    return u.ret;
    }
    static void notrace __used test_unwind_ftrace_handler(unsigned long ip,
    unsigned long parent_ip,
    struct ftrace_ops *fops,
    struct ftrace_regs *fregs)
    {
    struct unwindme *u = (struct unwindme *)arch_ftrace_regs(fregs).regs.gprs[2];
    u.ret = test_unwind(core::ptr::null_mut(), (u.flags & UWM_REGS) ? &arch_ftrace_regs(fregs).regs : core::ptr::null_mut(),
    (u.flags & UWM_SP) ? u.sp : 0);
    }
#[no_mangle]
unsafe extern "C" fn test_unwind_ftraced_func(u: *mut unwindme) -> noinline int {
    static noinline int test_unwind_ftraced_func(struct unwindme *u)
    {
    return READ_ONCE(u).ret;
    }
#[no_mangle]
unsafe extern "C" fn test_unwind_ftrace(u: *mut unwindme) -> c_int {
    static int test_unwind_ftrace(struct unwindme *u)
    {
    int ret;

    struct ftrace_ops *fops;
    fops = kunit_kzalloc(current_test, sizeof(*fops), GFP_KERNEL);
    fops.func = test_unwind_ftrace_handler;
    fops.flags = FTRACE_OPS_FL_DYNAMIC |
    FTRACE_OPS_FL_RECURSION |
    FTRACE_OPS_FL_SAVE_REGS |
    FTRACE_OPS_FL_PERMANENT;

    kunit_skip(current_test, "requires CONFIG_DYNAMIC_FTRACE");

    ret = ftrace_set_filter_ip(fops, (unsigned long)test_unwind_ftraced_func, 0, 0);
    if (ret) {
    kunit_err(current_test, "failed to set ftrace filter (%d)\n", ret);
    return -1;
    }
    ret = register_ftrace_function(fops);
    if (!ret) {
    ret = test_unwind_ftraced_func(u);
    unregister_ftrace_function(fops);
    } else {
    kunit_err(current_test, "failed to register ftrace handler (%d)\n", ret);
    }
    ftrace_set_filter_ip(fops, (unsigned long)test_unwind_ftraced_func, 1, 0);
    return ret;
    }
// This function may or may not appear in the backtrace.
#[no_mangle]
unsafe extern "C" fn unwindme_func4(u: *mut unwindme) -> noinline int {
    static noinline int unwindme_func4(struct unwindme *u)
    {
    if (!(u.flags & UWM_CALLER))
    u.sp = current_frame_address();
    if (u.flags & UWM_THREAD) {
    complete(&u.task_ready);
    wait_event(u.task_wq, kthread_should_park());
    kthread_parkme();
    return 0;
    } else if (u.flags & (UWM_PGM | UWM_KPROBE_ON_FTRACE)) {
    return test_unwind_kprobe(u);
    } else if (u.flags & (UWM_KRETPROBE | UWM_KRETPROBE_HANDLER)) {
    return test_unwind_kretprobe(u);
    } else if (u.flags & UWM_FTRACE) {
    return test_unwind_ftrace(u);
    } else {
    let mut regs: pt_regs = fake_pt_regs();
    return test_unwind(core::ptr::null_mut(),
    (u.flags & UWM_REGS) ? &regs : core::ptr::null_mut(),
    (u.flags & UWM_SP) ? u.sp : 0);
    }
    }
// This function may or may not appear in the backtrace.
#[no_mangle]
unsafe extern "C" fn unwindme_func3(u: *mut unwindme) -> noinline int {
    static noinline int unwindme_func3(struct unwindme *u)
    {
    u.sp = current_frame_address();
    return unwindme_func4(u);
    }
// This function must appear in the backtrace.
#[no_mangle]
unsafe extern "C" fn unwindme_func2(u: *mut unwindme) -> noinline int {
    static noinline int unwindme_func2(struct unwindme *u)
    {
    unsigned long flags, mflags;
    int rc;
    if (u.flags & UWM_SWITCH_STACK) {
    local_irq_save(flags);
    local_mcck_save(mflags);
    rc = call_on_stack(1, get_lowcore().nodat_stack,
    int, unwindme_func3, struct unwindme *, u);
    local_mcck_restore(mflags);
    local_irq_restore(flags);
    return rc;
    } else {
    return unwindme_func3(u);
    }
    }
// This function must follow unwindme_func2 in the backtrace.
#[no_mangle]
unsafe extern "C" fn unwindme_func1(u: *mut c_void) -> noinline int {
    static noinline int unwindme_func1(void *u)
    {
    return unwindme_func2((struct unwindme *)u);
    }
#[no_mangle]
unsafe extern "C" fn unwindme_timer_fn(unused: *mut timer_list) {
    static void unwindme_timer_fn(struct timer_list *unused)
    {
    struct unwindme *u = READ_ONCE(unwindme);
    if (u) {
    unwindme = core::ptr::null_mut();
    u.task = core::ptr::null_mut();
    u.ret = unwindme_func1(u);
    complete(&u.task_ready);
    }
    }
    static struct timer_list unwind_timer;
#[no_mangle]
unsafe extern "C" fn test_unwind_irq(u: *mut unwindme) -> c_int {
    static int test_unwind_irq(struct unwindme *u)
    {
    unwindme = u;
    init_completion(&u.task_ready);
    timer_setup(&unwind_timer, unwindme_timer_fn, 0);
    mod_timer(&unwind_timer, jiffies + 1);
    wait_for_completion(&u.task_ready);
    return u.ret;
    }
// Spawns a task and passes it to test_unwind().
#[no_mangle]
unsafe extern "C" fn test_unwind_task(u: *mut unwindme) -> c_int {
    static int test_unwind_task(struct unwindme *u)
    {
    struct task_struct *task;
    int ret;
// Initialize thread-related fields.
    init_completion(&u.task_ready);
    init_waitqueue_head(&u.task_wq);
//
// Start the task and wait until it reaches unwindme_func4() and sleeps
// in (task_ready, unwind_done] range.
//
    task = kthread_run(unwindme_func1, u, "%s", __func__);
    if (IS_ERR(task)) {
    kunit_err(current_test, "kthread_run() failed\n");
    return PTR_ERR(task);
    }
//
// Make sure task reaches unwindme_func4 before parking it,
// we might park it before kthread function has been executed otherwise
//
    wait_for_completion(&u.task_ready);
    kthread_park(task);
// Unwind.
    ret = test_unwind(task, core::ptr::null_mut(), (u.flags & UWM_SP) ? u.sp : 0);
    kthread_stop(task);
    return ret;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_params {
    pub flags: c_int,
    pub name: *mut c_char,
}

//
// Create required parameter list for tests
//

    static const struct test_params param_list[] = {
    TEST_WITH_FLAGS(UWM_DEFAULT),
    TEST_WITH_FLAGS(UWM_SP),
    TEST_WITH_FLAGS(UWM_REGS),
    TEST_WITH_FLAGS(UWM_SWITCH_STACK),
    TEST_WITH_FLAGS(UWM_SP | UWM_REGS),
    TEST_WITH_FLAGS(UWM_CALLER | UWM_SP),
    TEST_WITH_FLAGS(UWM_CALLER | UWM_SP | UWM_REGS),
    TEST_WITH_FLAGS(UWM_CALLER | UWM_SP | UWM_REGS | UWM_SWITCH_STACK),
    TEST_WITH_FLAGS(UWM_THREAD),
    TEST_WITH_FLAGS(UWM_THREAD | UWM_SP),
    TEST_WITH_FLAGS(UWM_THREAD | UWM_CALLER | UWM_SP),
    TEST_WITH_FLAGS(UWM_IRQ),
    TEST_WITH_FLAGS(UWM_IRQ | UWM_SWITCH_STACK),
    TEST_WITH_FLAGS(UWM_IRQ | UWM_SP),
    TEST_WITH_FLAGS(UWM_IRQ | UWM_REGS),
    TEST_WITH_FLAGS(UWM_IRQ | UWM_SP | UWM_REGS),
    TEST_WITH_FLAGS(UWM_IRQ | UWM_CALLER | UWM_SP),
    TEST_WITH_FLAGS(UWM_IRQ | UWM_CALLER | UWM_SP | UWM_REGS),
    TEST_WITH_FLAGS(UWM_IRQ | UWM_CALLER | UWM_SP | UWM_REGS | UWM_SWITCH_STACK),
    TEST_WITH_FLAGS(UWM_PGM),
    TEST_WITH_FLAGS(UWM_PGM | UWM_SP),
    TEST_WITH_FLAGS(UWM_PGM | UWM_REGS),
    TEST_WITH_FLAGS(UWM_PGM | UWM_SP | UWM_REGS),
    TEST_WITH_FLAGS(UWM_KPROBE_ON_FTRACE),
    TEST_WITH_FLAGS(UWM_KPROBE_ON_FTRACE | UWM_SP),
    TEST_WITH_FLAGS(UWM_KPROBE_ON_FTRACE | UWM_REGS),
    TEST_WITH_FLAGS(UWM_KPROBE_ON_FTRACE | UWM_SP | UWM_REGS),
    TEST_WITH_FLAGS(UWM_FTRACE),
    TEST_WITH_FLAGS(UWM_FTRACE | UWM_SP),
    TEST_WITH_FLAGS(UWM_FTRACE | UWM_REGS),
    TEST_WITH_FLAGS(UWM_FTRACE | UWM_SP | UWM_REGS),
    TEST_WITH_FLAGS(UWM_KRETPROBE),
    TEST_WITH_FLAGS(UWM_KRETPROBE | UWM_SP),
    TEST_WITH_FLAGS(UWM_KRETPROBE | UWM_REGS),
    TEST_WITH_FLAGS(UWM_KRETPROBE | UWM_SP | UWM_REGS),
    TEST_WITH_FLAGS(UWM_KRETPROBE_HANDLER),
    TEST_WITH_FLAGS(UWM_KRETPROBE_HANDLER | UWM_SP),
    TEST_WITH_FLAGS(UWM_KRETPROBE_HANDLER | UWM_REGS),
    TEST_WITH_FLAGS(UWM_KRETPROBE_HANDLER | UWM_SP | UWM_REGS),
    };
//
// Parameter description generator: required for KUNIT_ARRAY_PARAM()
//
#[no_mangle]
unsafe extern "C" fn get_desc(params: *const test_params, desc: *mut c_char) {
    static void get_desc(const struct test_params *params, char *desc)
    {
    strscpy(desc, params.name, KUNIT_PARAM_DESC_SIZE);
    }
//
// Create test_unwind_gen_params
//
    KUNIT_ARRAY_PARAM(test_unwind, param_list, get_desc);
#[no_mangle]
unsafe extern "C" fn test_unwind_flags(test: *mut kunit) {
    static void test_unwind_flags(struct kunit *test)
    {
    struct unwindme u;
    const struct test_params *params;
    current_test = test;
    params = (const struct test_params *)test.param_value;
    u.flags = params.flags;
    if (u.flags & UWM_THREAD)
    KUNIT_EXPECT_EQ(test, 0, test_unwind_task(&u));
#[no_mangle]
pub unsafe extern "C" fn if(UWM_IRQ: u.flags &) -> else {
    else if (u.flags & UWM_IRQ)
    KUNIT_EXPECT_EQ(test, 0, test_unwind_irq(&u));
    else
    KUNIT_EXPECT_EQ(test, 0, unwindme_func1(&u));
    }
    static struct kunit_case unwind_test_cases[] = {
    KUNIT_CASE_PARAM(test_unwind_flags, test_unwind_gen_params),
    {}
    };
    static struct kunit_suite test_unwind_suite = {
    .name = "test_unwind",
    .test_cases = unwind_test_cases,
    };
    kunit_test_suites(&test_unwind_suite);
    MODULE_DESCRIPTION("KUnit test for unwind_for_each_frame");
    MODULE_LICENSE("GPL");
