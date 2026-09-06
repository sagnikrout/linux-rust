//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/uprobe_multi_test.c
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

    static char test_data[] = "test_data";
#[no_mangle]
pub unsafe extern "C" fn uprobe_multi_func_1() -> noinline void {
    noinline void uprobe_multi_func_1(void)
    {
    asm volatile ("");
    }
#[no_mangle]
pub unsafe extern "C" fn uprobe_multi_func_2() -> noinline void {
    noinline void uprobe_multi_func_2(void)
    {
    asm volatile ("");
    }
#[no_mangle]
pub unsafe extern "C" fn uprobe_multi_func_3() -> noinline void {
    noinline void uprobe_multi_func_3(void)
    {
    asm volatile ("");
    }
#[no_mangle]
pub unsafe extern "C" fn usdt_trigger() -> noinline void {
    noinline void usdt_trigger(void)
    {
    STAP_PROBE(test, pid_filter_usdt);
    }
#[no_mangle]
pub unsafe extern "C" fn uprobe_session_recursive(i: c_int) -> noinline void {
    noinline void uprobe_session_recursive(int i)
    {
    if (i)
    uprobe_session_recursive(i - 1);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct child {
    pub go: [c_int; 2],
    pub /: *mut *mut int c2p[2]; / child -> parent channel,
    pub pid: c_int,
    pub tid: c_int,
    pub thread: pthread_t,
    pub stack: [c_char; 65536],
}

#[no_mangle]
unsafe extern "C" fn release_child(child: *mut child) {
    static void release_child(struct child *child)
    {
    int child_status;
    if (!child)
    return;
    close(child.go[1]);
    close(child.go[0]);
    if (child.thread) {
    pthread_join(child.thread, core::ptr::null_mut());
    child.thread = 0;
    }
    close(child.c2p[0]);
    close(child.c2p[1]);
    if (child.pid > 0)
    waitpid(child.pid, &child_status, 0);
    }
#[no_mangle]
unsafe extern "C" fn kick_child(child: *mut child) {
    static void kick_child(struct child *child)
    {
    let mut c: c_char = 1;
    if (child) {
    write(child.go[1], &c, 1);
    release_child(child);
    }
    fflush(core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn child_func(arg: *mut c_void) -> c_int {
    static int child_func(void *arg)
    {
    struct child *child = arg;
    int err, c;
    close(child.go[1]);
// wait for parent's kick
    err = read(child.go[0], &c, 1);
    if (err != 1)
    exit(err);
    uprobe_multi_func_1();
    uprobe_multi_func_2();
    uprobe_multi_func_3();
    usdt_trigger();
    exit(errno);
    }
#[no_mangle]
unsafe extern "C" fn spawn_child_flag(child: *mut child, clone_vm: bool) -> c_int {
    static int spawn_child_flag(struct child *child, bool clone_vm)
    {
// pipe to notify child to execute the trigger functions
    if (pipe(child.go))
    return -1;
    if (clone_vm) {
    child.pid = child.tid = clone(child_func, child.stack + sizeof(child.stack)/2,
    CLONE_VM|SIGCHLD, child);
    } else {
    child.pid = child.tid = fork();
    }
    if (child.pid < 0) {
    release_child(child);
    errno = EINVAL;
    return -1;
    }
// fork-ed child
    if (!clone_vm && child.pid == 0)
    child_func(child);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spawn_child(child: *mut child) -> c_int {
    static int spawn_child(struct child *child)
    {
    return spawn_child_flag(child, false);
    }
    static void *child_thread(void *ctx)
    {
    struct child *child = ctx;
    let mut c: c_int = 0, err;
    child.tid = sys_gettid();
// let parent know we are ready
    err = write(child.c2p[1], &c, 1);
    if (err != 1)
    pthread_exit(&err);
// wait for parent's kick
    err = read(child.go[0], &c, 1);
    if (err != 1)
    pthread_exit(&err);
    uprobe_multi_func_1();
    uprobe_multi_func_2();
    uprobe_multi_func_3();
    usdt_trigger();
    err = 0;
    pthread_exit(&err);
    }
#[no_mangle]
unsafe extern "C" fn spawn_thread(child: *mut child) -> c_int {
    static int spawn_thread(struct child *child)
    {
    int c, err;
// pipe to notify child to execute the trigger functions
    if (pipe(child.go))
    return -1;
// pipe to notify parent that child thread is ready
    if (pipe(child.c2p)) {
    close(child.go[0]);
    close(child.go[1]);
    return -1;
    }
    child.pid = getpid();
    err = pthread_create(&child.thread, core::ptr::null_mut(), child_thread, child);
    if (err) {
    err = -errno;
    close(child.go[0]);
    close(child.go[1]);
    close(child.c2p[0]);
    close(child.c2p[1]);
    errno = -err;
    return -1;
    }
    err = read(child.c2p[0], &c, 1);
    if (!ASSERT_EQ(err, 1, "child_thread_ready"))
    return -1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uprobe_multi_test_run(skel: *mut uprobe_multi, child: *mut child) {
    static void uprobe_multi_test_run(struct uprobe_multi *skel, struct child *child)
    {
    skel.bss.uprobe_multi_func_1_addr = (__u64) uprobe_multi_func_1;
    skel.bss.uprobe_multi_func_2_addr = (__u64) uprobe_multi_func_2;
    skel.bss.uprobe_multi_func_3_addr = (__u64) uprobe_multi_func_3;
    skel.bss.user_ptr = test_data;
//
// Disable pid check in bpf program if we are pid filter test,
// because the probe should be executed only by child->pid
// passed at the probe attach.
//
    skel.bss.pid = child ? 0 : getpid();
    skel.bss.expect_pid = child ? child.pid : 0;
// trigger all probes, if we are testing child *process*, just to make
// sure that PID filtering doesn't let through activations from wrong
// PIDs; when we test child *thread*, we don't want to do this to
// avoid double counting number of triggering events
//
    if (!child || !child.thread) {
    uprobe_multi_func_1();
    uprobe_multi_func_2();
    uprobe_multi_func_3();
    usdt_trigger();
    }
    if (child)
    kick_child(child);
//
// There are 2 entry and 2 exit probe called for each uprobe_multi_func_[123]
// function and each sleepable probe (6) increments uprobe_multi_sleep_result.
//
    ASSERT_EQ(skel.bss.uprobe_multi_func_1_result, 2, "uprobe_multi_func_1_result");
    ASSERT_EQ(skel.bss.uprobe_multi_func_2_result, 2, "uprobe_multi_func_2_result");
    ASSERT_EQ(skel.bss.uprobe_multi_func_3_result, 2, "uprobe_multi_func_3_result");
    ASSERT_EQ(skel.bss.uretprobe_multi_func_1_result, 2, "uretprobe_multi_func_1_result");
    ASSERT_EQ(skel.bss.uretprobe_multi_func_2_result, 2, "uretprobe_multi_func_2_result");
    ASSERT_EQ(skel.bss.uretprobe_multi_func_3_result, 2, "uretprobe_multi_func_3_result");
    ASSERT_EQ(skel.bss.uprobe_multi_sleep_result, 6, "uprobe_multi_sleep_result");
    ASSERT_FALSE(skel.bss.bad_pid_seen, "bad_pid_seen");
    if (child) {
    ASSERT_EQ(skel.bss.child_pid, child.pid, "uprobe_multi_child_pid");
    ASSERT_EQ(skel.bss.child_tid, child.tid, "uprobe_multi_child_tid");
    }
    }
#[no_mangle]
unsafe extern "C" fn test_skel_api() {
    static void test_skel_api(void)
    {
    struct uprobe_multi *skel = core::ptr::null_mut();
    int err;
    skel = uprobe_multi__open_and_load();
    if (!ASSERT_OK_PTR(skel, "uprobe_multi__open_and_load"))
    goto cleanup;
    err = uprobe_multi__attach(skel);
    if (!ASSERT_OK(err, "uprobe_multi__attach"))
    goto cleanup;
    uprobe_multi_test_run(skel, core::ptr::null_mut());
    cleanup:
    uprobe_multi__destroy(skel);
    }
    static void
    __test_attach_api(const char *binary, const char *pattern, struct bpf_uprobe_multi_opts *opts,
    struct child *child)
    {
    let mut pid: pid_t = child ? child.pid : -1;
    struct uprobe_multi *skel = core::ptr::null_mut();
    skel = uprobe_multi__open_and_load();
    if (!ASSERT_OK_PTR(skel, "uprobe_multi__open_and_load"))
    goto cleanup;
    opts.retprobe = false;
    skel.links.uprobe = bpf_program__attach_uprobe_multi(skel.progs.uprobe, pid,
    binary, pattern, opts);
    if (!ASSERT_OK_PTR(skel.links.uprobe, "bpf_program__attach_uprobe_multi"))
    goto cleanup;
    opts.retprobe = true;
    skel.links.uretprobe = bpf_program__attach_uprobe_multi(skel.progs.uretprobe, pid,
    binary, pattern, opts);
    if (!ASSERT_OK_PTR(skel.links.uretprobe, "bpf_program__attach_uprobe_multi"))
    goto cleanup;
    opts.retprobe = false;
    skel.links.uprobe_sleep = bpf_program__attach_uprobe_multi(skel.progs.uprobe_sleep, pid,
    binary, pattern, opts);
    if (!ASSERT_OK_PTR(skel.links.uprobe_sleep, "bpf_program__attach_uprobe_multi"))
    goto cleanup;
    opts.retprobe = true;
    skel.links.uretprobe_sleep = bpf_program__attach_uprobe_multi(skel.progs.uretprobe_sleep,
    pid, binary, pattern, opts);
    if (!ASSERT_OK_PTR(skel.links.uretprobe_sleep, "bpf_program__attach_uprobe_multi"))
    goto cleanup;
    opts.retprobe = false;
    skel.links.uprobe_extra = bpf_program__attach_uprobe_multi(skel.progs.uprobe_extra, -1,
    binary, pattern, opts);
    if (!ASSERT_OK_PTR(skel.links.uprobe_extra, "bpf_program__attach_uprobe_multi"))
    goto cleanup;
// Attach (uprobe-backed) USDTs
    skel.links.usdt_pid = bpf_program__attach_usdt(skel.progs.usdt_pid, pid, binary,
    "test", "pid_filter_usdt", core::ptr::null_mut());
    if (!ASSERT_OK_PTR(skel.links.usdt_pid, "attach_usdt_pid"))
    goto cleanup;
    skel.links.usdt_extra = bpf_program__attach_usdt(skel.progs.usdt_extra, -1, binary,
    "test", "pid_filter_usdt", core::ptr::null_mut());
    if (!ASSERT_OK_PTR(skel.links.usdt_extra, "attach_usdt_extra"))
    goto cleanup;
    uprobe_multi_test_run(skel, child);
    ASSERT_FALSE(skel.bss.bad_pid_seen_usdt, "bad_pid_seen_usdt");
    if (child) {
    ASSERT_EQ(skel.bss.child_pid_usdt, child.pid, "usdt_multi_child_pid");
    ASSERT_EQ(skel.bss.child_tid_usdt, child.tid, "usdt_multi_child_tid");
    }
    cleanup:
    uprobe_multi__destroy(skel);
    }
    static void
    test_attach_api(const char *binary, const char *pattern, struct bpf_uprobe_multi_opts *opts)
    {
    static struct child child;
    memset(&child, 0, sizeof(child));
// no pid filter
    __test_attach_api(binary, pattern, opts, core::ptr::null_mut());
// pid filter
    if (!ASSERT_OK(spawn_child(&child), "spawn_child"))
    return;
    __test_attach_api(binary, pattern, opts, &child);
// pid filter (thread)
    if (!ASSERT_OK(spawn_thread(&child), "spawn_thread"))
    return;
    __test_attach_api(binary, pattern, opts, &child);
    }
#[no_mangle]
unsafe extern "C" fn test_attach_api_pattern() {
    static void test_attach_api_pattern(void)
    {
    LIBBPF_OPTS(bpf_uprobe_multi_opts, opts);
    test_attach_api("/proc/self/exe", "uprobe_multi_func_*", &opts);
    test_attach_api("/proc/self/exe", "uprobe_multi_func_?", &opts);
    }
#[no_mangle]
unsafe extern "C" fn test_attach_api_syms() {
    static void test_attach_api_syms(void)
    {
    LIBBPF_OPTS(bpf_uprobe_multi_opts, opts);
    const char *syms[3] = {
    "uprobe_multi_func_1",
    "uprobe_multi_func_2",
    "uprobe_multi_func_3",
    };
    opts.syms = syms;
    opts.cnt = ARRAY_SIZE(syms);
    test_attach_api("/proc/self/exe", core::ptr::null_mut(), &opts);
    }
#[no_mangle]
unsafe extern "C" fn test_attach_api_fails() {
    static void test_attach_api_fails(void)
    {
    LIBBPF_OPTS(bpf_link_create_opts, opts);
    const char *path = "/proc/self/exe";
    struct uprobe_multi *skel = core::ptr::null_mut();
    int prog_fd, link_fd = -1;
    let mut offset: c_ulong = 0;
    skel = uprobe_multi__open_and_load();
    if (!ASSERT_OK_PTR(skel, "uprobe_multi__open_and_load"))
    goto cleanup;
    prog_fd = bpf_program__fd(skel.progs.uprobe_extra);
// abnormal cnt
    opts.uprobe_multi.path = path;
    opts.uprobe_multi.offsets = &offset;
    opts.uprobe_multi.cnt = INT_MAX;
    link_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if (!ASSERT_ERR(link_fd, "link_fd"))
    goto cleanup;
    if (!ASSERT_EQ(link_fd, -E2BIG, "big cnt"))
    goto cleanup;
// cnt is 0
    LIBBPF_OPTS_RESET(opts,
    .uprobe_multi.path = path,
    .uprobe_multi.offsets = (unsigned long *) &offset,
    );
    link_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if (!ASSERT_ERR(link_fd, "link_fd"))
    goto cleanup;
    if (!ASSERT_EQ(link_fd, -EINVAL, "cnt_is_zero"))
    goto cleanup;
// negative offset
    offset = -1;
    opts.uprobe_multi.path = path;
    opts.uprobe_multi.offsets = (unsigned long *) &offset;
    opts.uprobe_multi.cnt = 1;
    link_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if (!ASSERT_ERR(link_fd, "link_fd"))
    goto cleanup;
    if (!ASSERT_EQ(link_fd, -EINVAL, "offset_is_negative"))
    goto cleanup;
// offsets is NULL
    LIBBPF_OPTS_RESET(opts,
    .uprobe_multi.path = path,
    .uprobe_multi.cnt = 1,
    );
    link_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if (!ASSERT_ERR(link_fd, "link_fd"))
    goto cleanup;
    if (!ASSERT_EQ(link_fd, -EINVAL, "offsets_is_null"))
    goto cleanup;
// wrong offsets pointer
    LIBBPF_OPTS_RESET(opts,
    .uprobe_multi.path = path,
    .uprobe_multi.offsets = (unsigned long *) 1,
    .uprobe_multi.cnt = 1,
    );
    link_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if (!ASSERT_ERR(link_fd, "link_fd"))
    goto cleanup;
    if (!ASSERT_EQ(link_fd, -EFAULT, "offsets_is_wrong"))
    goto cleanup;
// path is NULL
    offset = 1;
    LIBBPF_OPTS_RESET(opts,
    .uprobe_multi.offsets = (unsigned long *) &offset,
    .uprobe_multi.cnt = 1,
    );
    link_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if (!ASSERT_ERR(link_fd, "link_fd"))
    goto cleanup;
    if (!ASSERT_EQ(link_fd, -EINVAL, "path_is_null"))
    goto cleanup;
// wrong path pointer
    LIBBPF_OPTS_RESET(opts,
    .uprobe_multi.path = (const char *) 1,
    .uprobe_multi.offsets = (unsigned long *) &offset,
    .uprobe_multi.cnt = 1,
    );
    link_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if (!ASSERT_ERR(link_fd, "link_fd"))
    goto cleanup;
    if (!ASSERT_EQ(link_fd, -EFAULT, "path_is_wrong"))
    goto cleanup;
// wrong path type
    LIBBPF_OPTS_RESET(opts,
    .uprobe_multi.path = "/",
    .uprobe_multi.offsets = (unsigned long *) &offset,
    .uprobe_multi.cnt = 1,
    );
    link_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if (!ASSERT_ERR(link_fd, "link_fd"))
    goto cleanup;
    if (!ASSERT_EQ(link_fd, -EBADF, "path_is_wrong_type"))
    goto cleanup;
// wrong cookies pointer
    LIBBPF_OPTS_RESET(opts,
    .uprobe_multi.path = path,
    .uprobe_multi.offsets = (unsigned long *) &offset,
    .uprobe_multi.cookies = (__u64 *) 1ULL,
    .uprobe_multi.cnt = 1,
    );
    link_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if (!ASSERT_ERR(link_fd, "link_fd"))
    goto cleanup;
    if (!ASSERT_EQ(link_fd, -EFAULT, "cookies_is_wrong"))
    goto cleanup;
// wrong ref_ctr_offsets pointer
    LIBBPF_OPTS_RESET(opts,
    .uprobe_multi.path = path,
    .uprobe_multi.offsets = (unsigned long *) &offset,
    .uprobe_multi.cookies = (__u64 *) &offset,
    .uprobe_multi.ref_ctr_offsets = (unsigned long *) 1,
    .uprobe_multi.cnt = 1,
    );
    link_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if (!ASSERT_ERR(link_fd, "link_fd"))
    goto cleanup;
    if (!ASSERT_EQ(link_fd, -EFAULT, "ref_ctr_offsets_is_wrong"))
    goto cleanup;
// wrong flags
    LIBBPF_OPTS_RESET(opts,
    .uprobe_multi.flags = 1 << 31,
    );
    link_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if (!ASSERT_ERR(link_fd, "link_fd"))
    goto cleanup;
    if (!ASSERT_EQ(link_fd, -EINVAL, "wrong_flags"))
    goto cleanup;
// wrong pid
    LIBBPF_OPTS_RESET(opts,
    .uprobe_multi.path = path,
    .uprobe_multi.offsets = (unsigned long *) &offset,
    .uprobe_multi.cnt = 1,
    .uprobe_multi.pid = -2,
    );
    link_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if (!ASSERT_ERR(link_fd, "link_fd"))
    goto cleanup;
    if (!ASSERT_EQ(link_fd, -EINVAL, "pid_is_wrong"))
    goto cleanup;
// wrong path_fd
    LIBBPF_OPTS_RESET(opts,
    .uprobe_multi.path = core::ptr::null_mut(),
    .uprobe_multi.path_fd = -1,
    .uprobe_multi.flags = BPF_F_UPROBE_MULTI_PATH_FD,
    .uprobe_multi.offsets = (unsigned long *)&offset,
    .uprobe_multi.cnt = 1,
    );
    link_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if (!ASSERT_ERR(link_fd, "link_fd"))
    goto cleanup;
    if (!ASSERT_EQ(link_fd, -EBADF, "path_fd_is_wrong"))
    goto cleanup;
// path and path_fd both set with BPF_F_UPROBE_MULTI_PATH_FD flag
    LIBBPF_OPTS_RESET(opts,
    .uprobe_multi.path = path,
    .uprobe_multi.path_fd = 1,
    .uprobe_multi.flags = BPF_F_UPROBE_MULTI_PATH_FD,
    .uprobe_multi.offsets = (unsigned long *)&offset,
    .uprobe_multi.cnt = 1,
    );
    link_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if (!ASSERT_ERR(link_fd, "link_fd"))
    goto cleanup;
    ASSERT_EQ(link_fd, -EINVAL, "path_and_path_fd_together");
    cleanup:
    if (link_fd >= 0)
    close(link_fd);
    uprobe_multi__destroy(skel);
    }

#[no_mangle]
pub unsafe extern "C" fn uprobe_multi_error_func() -> noinline void {
    noinline void uprobe_multi_error_func(void)
    {
//
// If --fcf-protection=branch is enabled the gcc generates endbr as
// first instruction, so marking the exact address of int3 with the
// symbol to be used in the attach_uprobe_fail_trap test below.
//
    asm volatile (
    ".globl uprobe_multi_error_func_int3;	\n"
    "uprobe_multi_error_func_int3:		\n"
    "int3					\n"
    );
    }
//
// Attaching uprobe on uprobe_multi_error_func results in error
// because it already starts with int3 instruction.
//
#[no_mangle]
unsafe extern "C" fn attach_uprobe_fail_trap(skel: *mut uprobe_multi) {
    static void attach_uprobe_fail_trap(struct uprobe_multi *skel)
    {
    LIBBPF_OPTS(bpf_uprobe_multi_opts, opts);
    const char *syms[4] = {
    "uprobe_multi_func_1",
    "uprobe_multi_func_2",
    "uprobe_multi_func_3",
    "uprobe_multi_error_func_int3",
    };
    opts.syms = syms;
    opts.cnt = ARRAY_SIZE(syms);
    skel.links.uprobe = bpf_program__attach_uprobe_multi(skel.progs.uprobe, -1,
    "/proc/self/exe", core::ptr::null_mut(), &opts);
    if (!ASSERT_ERR_PTR(skel.links.uprobe, "bpf_program__attach_uprobe_multi")) {
    bpf_link__destroy(skel.links.uprobe);
    skel.links.uprobe = core::ptr::null_mut();
    }
    }

    static void attach_uprobe_fail_trap(struct uprobe_multi *skel) { }

    short sema_1 __used, sema_2 __used;
#[no_mangle]
unsafe extern "C" fn attach_uprobe_fail_refctr(skel: *mut uprobe_multi) {
    static void attach_uprobe_fail_refctr(struct uprobe_multi *skel)
    {
    unsigned long *tmp_offsets = core::ptr::null_mut(), *tmp_ref_ctr_offsets = core::ptr::null_mut();
    unsigned long offsets[3], ref_ctr_offsets[3];
    LIBBPF_OPTS(bpf_link_create_opts, opts);
    const char *path = "/proc/self/exe";
    const char *syms[3] = {
    "uprobe_multi_func_1",
    "uprobe_multi_func_2",
    };
    const char *sema[3] = {
    "sema_1",
    "sema_2",
    };
    int prog_fd, link_fd, err;
    prog_fd = bpf_program__fd(skel.progs.uprobe_extra);
    err = elf_resolve_syms_offsets("/proc/self/exe", 2, (const char **) &syms,
    &tmp_offsets, STT_FUNC);
    if (!ASSERT_OK(err, "elf_resolve_syms_offsets_func"))
    return;
    err = elf_resolve_syms_offsets("/proc/self/exe", 2, (const char **) &sema,
    &tmp_ref_ctr_offsets, STT_OBJECT);
    if (!ASSERT_OK(err, "elf_resolve_syms_offsets_sema"))
    goto cleanup;
//
// We attach to 3 uprobes on 2 functions, so 2 uprobes share single function,
// but with different ref_ctr_offset which is not allowed and results in fail.
//
    offsets[0] = tmp_offsets[0]; /* uprobe_multi_func_1 */
    offsets[1] = tmp_offsets[1]; /* uprobe_multi_func_2 */
    offsets[2] = tmp_offsets[1]; /* uprobe_multi_func_2 */
    ref_ctr_offsets[0] = tmp_ref_ctr_offsets[0]; /* sema_1 */
    ref_ctr_offsets[1] = tmp_ref_ctr_offsets[1]; /* sema_2 */
    ref_ctr_offsets[2] = tmp_ref_ctr_offsets[0]; /* sema_1, error */
    opts.uprobe_multi.path = path;
    opts.uprobe_multi.offsets = (const unsigned long *) &offsets;
    opts.uprobe_multi.ref_ctr_offsets = (const unsigned long *) &ref_ctr_offsets;
    opts.uprobe_multi.cnt = 3;
    link_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if (!ASSERT_ERR(link_fd, "link_fd"))
    close(link_fd);
    cleanup:
    free(tmp_ref_ctr_offsets);
    free(tmp_offsets);
    }
#[no_mangle]
unsafe extern "C" fn test_attach_uprobe_fails() {
    static void test_attach_uprobe_fails(void)
    {
    struct uprobe_multi *skel = core::ptr::null_mut();
    skel = uprobe_multi__open_and_load();
    if (!ASSERT_OK_PTR(skel, "uprobe_multi__open_and_load"))
    return;
// attach fails due to adding uprobe on trap instruction, x86_64 only
    attach_uprobe_fail_trap(skel);
// attach fail due to wrong ref_ctr_offs on one of the uprobes
    attach_uprobe_fail_refctr(skel);
    uprobe_multi__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn __test_link_api(child: *mut child) {
    static void __test_link_api(struct child *child)
    {
    int prog_fd, link1_fd = -1, link2_fd = -1, link3_fd = -1, link4_fd = -1;
    LIBBPF_OPTS(bpf_link_create_opts, opts);
    const char *path = "/proc/self/exe";
    struct uprobe_multi *skel = core::ptr::null_mut();
    unsigned long *offsets = core::ptr::null_mut();
    const char *syms[3] = {
    "uprobe_multi_func_1",
    "uprobe_multi_func_2",
    "uprobe_multi_func_3",
    };
    let mut link_extra_fd: c_int = -1;
    int err;
    err = elf_resolve_syms_offsets(path, 3, syms, (unsigned long **) &offsets, STT_FUNC);
    if (!ASSERT_OK(err, "elf_resolve_syms_offsets"))
    return;
    opts.uprobe_multi.path = path;
    opts.uprobe_multi.offsets = offsets;
    opts.uprobe_multi.cnt = ARRAY_SIZE(syms);
    opts.uprobe_multi.pid = child ? child.pid : 0;
    skel = uprobe_multi__open_and_load();
    if (!ASSERT_OK_PTR(skel, "uprobe_multi__open_and_load"))
    goto cleanup;
    opts.kprobe_multi.flags = 0;
    prog_fd = bpf_program__fd(skel.progs.uprobe);
    link1_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if (!ASSERT_GE(link1_fd, 0, "link1_fd"))
    goto cleanup;
    opts.kprobe_multi.flags = BPF_F_UPROBE_MULTI_RETURN;
    prog_fd = bpf_program__fd(skel.progs.uretprobe);
    link2_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if (!ASSERT_GE(link2_fd, 0, "link2_fd"))
    goto cleanup;
    opts.kprobe_multi.flags = 0;
    prog_fd = bpf_program__fd(skel.progs.uprobe_sleep);
    link3_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if (!ASSERT_GE(link3_fd, 0, "link3_fd"))
    goto cleanup;
    opts.kprobe_multi.flags = BPF_F_UPROBE_MULTI_RETURN;
    prog_fd = bpf_program__fd(skel.progs.uretprobe_sleep);
    link4_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if (!ASSERT_GE(link4_fd, 0, "link4_fd"))
    goto cleanup;
    opts.kprobe_multi.flags = 0;
    opts.uprobe_multi.pid = 0;
    prog_fd = bpf_program__fd(skel.progs.uprobe_extra);
    link_extra_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if (!ASSERT_GE(link_extra_fd, 0, "link_extra_fd"))
    goto cleanup;
    uprobe_multi_test_run(skel, child);
    cleanup:
    if (link1_fd >= 0)
    close(link1_fd);
    if (link2_fd >= 0)
    close(link2_fd);
    if (link3_fd >= 0)
    close(link3_fd);
    if (link4_fd >= 0)
    close(link4_fd);
    if (link_extra_fd >= 0)
    close(link_extra_fd);
    uprobe_multi__destroy(skel);
    free(offsets);
    }
#[no_mangle]
unsafe extern "C" fn test_link_api() {
    static void test_link_api(void)
    {
    static struct child child;
// no pid filter
    __test_link_api(core::ptr::null_mut());
// pid filter
    if (!ASSERT_OK(spawn_child(&child), "spawn_child"))
    return;
    __test_link_api(&child);
// pid filter (thread)
    if (!ASSERT_OK(spawn_thread(&child), "spawn_thread"))
    return;
    __test_link_api(&child);
    }
#[no_mangle]
unsafe extern "C" fn test_link_api_path_fd() {
    static void test_link_api_path_fd(void)
    {
    LIBBPF_OPTS(bpf_link_create_opts, opts);
    const char *resolve_path = "/proc/self/exe";
    int prog_fd, link_fd = -1, path_fd = -1;
    struct uprobe_multi *skel = core::ptr::null_mut();
    unsigned long *offsets = core::ptr::null_mut();
    const char *syms[3] = {
    "uprobe_multi_func_1",
    "uprobe_multi_func_2",
    "uprobe_multi_func_3",
    };
    int err;
    err = elf_resolve_syms_offsets(resolve_path, ARRAY_SIZE(syms), syms,
    &offsets, STT_FUNC);
    if (!ASSERT_OK(err, "elf_resolve_syms_offsets"))
    return;
    path_fd = open(resolve_path, O_RDONLY);
    if (!ASSERT_GE(path_fd, 0, "path_fd"))
    goto cleanup;
    opts.uprobe_multi.path_fd = path_fd;
    opts.uprobe_multi.offsets = offsets;
    opts.uprobe_multi.cnt = ARRAY_SIZE(syms);
    opts.uprobe_multi.flags = BPF_F_UPROBE_MULTI_PATH_FD;
    skel = uprobe_multi__open_and_load();
    if (!ASSERT_OK_PTR(skel, "uprobe_multi__open_and_load"))
    goto cleanup;
    prog_fd = bpf_program__fd(skel.progs.uprobe);
    link_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_UPROBE_MULTI, &opts);
    if (!ASSERT_GE(link_fd, 0, "bpf_link_create"))
    goto cleanup;
    skel.bss.uprobe_multi_func_1_addr = (__u64)uprobe_multi_func_1;
    skel.bss.uprobe_multi_func_2_addr = (__u64)uprobe_multi_func_2;
    skel.bss.uprobe_multi_func_3_addr = (__u64)uprobe_multi_func_3;
    skel.bss.pid = getpid();
    uprobe_multi_func_1();
    uprobe_multi_func_2();
    uprobe_multi_func_3();
    ASSERT_EQ(skel.bss.uprobe_multi_func_1_result, 1, "uprobe_multi_func_1_result");
    ASSERT_EQ(skel.bss.uprobe_multi_func_2_result, 1, "uprobe_multi_func_2_result");
    ASSERT_EQ(skel.bss.uprobe_multi_func_3_result, 1, "uprobe_multi_func_3_result");
    cleanup:
    if (link_fd >= 0)
    close(link_fd);
    if (path_fd >= 0)
    close(path_fd);
    uprobe_multi__destroy(skel);
    free(offsets);
    }
    static struct bpf_program *
    get_program(struct uprobe_multi_consumers *skel, int prog)
    {
    switch (prog) {
    case 0:
    return skel.progs.uprobe_0;
    case 1:
    return skel.progs.uprobe_1;
    case 2:
    return skel.progs.uprobe_2;
    case 3:
    return skel.progs.uprobe_3;
    default:
    ASSERT_FAIL("get_program");
    return core::ptr::null_mut();
    }
    }
    static struct bpf_link **
    get_link(struct uprobe_multi_consumers *skel, int link)
    {
    switch (link) {
    case 0:
    return &skel.links.uprobe_0;
    case 1:
    return &skel.links.uprobe_1;
    case 2:
    return &skel.links.uprobe_2;
    case 3:
    return &skel.links.uprobe_3;
    default:
    ASSERT_FAIL("get_link");
    return core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn uprobe_attach(skel: *mut uprobe_multi_consumers, idx: c_int, offset: c_ulong) -> c_int {
    static int uprobe_attach(struct uprobe_multi_consumers *skel, int idx, unsigned long offset)
    {
    struct bpf_program *prog = get_program(skel, idx);
    struct bpf_link **link = get_link(skel, idx);
    LIBBPF_OPTS(bpf_uprobe_multi_opts, opts);
    if (!prog || !link)
    return -1;
    opts.offsets = &offset;
    opts.cnt = 1;
//
// bit/prog: 0 uprobe entry
// bit/prog: 1 uprobe return
// bit/prog: 2 uprobe session without return
// bit/prog: 3 uprobe session with return
//
    opts.retprobe = idx == 1;
    opts.session  = idx == 2 || idx == 3;
// link = bpf_program__attach_uprobe_multi(prog, 0, "/proc/self/exe", NULL, &opts);
    if (!ASSERT_OK_PTR(*link, "bpf_program__attach_uprobe_multi"))
    return -1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uprobe_detach(skel: *mut uprobe_multi_consumers, idx: c_int) {
    static void uprobe_detach(struct uprobe_multi_consumers *skel, int idx)
    {
    struct bpf_link **link = get_link(skel, idx);
    bpf_link__destroy(*link);
// link = NULL;
    }
#[no_mangle]
unsafe extern "C" fn test_bit(bit: c_int, val: c_ulong) -> bool {
    static bool test_bit(int bit, unsigned long val)
    {
    return val & (1 << bit);
    }
    noinline int
    uprobe_consumer_test(struct uprobe_multi_consumers *skel,
    unsigned long before, unsigned long after,
    unsigned long offset)
    {
    int idx;
// detach uprobe for each unset programs in 'before' state ...
    for (idx = 0; idx < 4; idx++) {
    if (test_bit(idx, before) && !test_bit(idx, after))
    uprobe_detach(skel, idx);
    }
// ... and attach all new programs in 'after' state
    for (idx = 0; idx < 4; idx++) {
    if (!test_bit(idx, before) && test_bit(idx, after)) {
    if (!ASSERT_OK(uprobe_attach(skel, idx, offset), "uprobe_attach_after"))
    return -1;
    }
    }
    return 0;
    }
//
// We generate 16 consumer_testX functions that will have uprobe installed on
// and will be called in separate threads. All function pointer are stored in
// "consumers" section and each thread will pick one function based on index.
//
    extern const void *__start_consumers;

    noinline int func(struct uprobe_multi_consumers *skel, unsigned long before,	\
    unsigned long after, unsigned long offset)			\
    {										\
    return uprobe_consumer_test(skel, before, after, offset);		\
    }										\
    void *__ ## func __used __attribute__((section("consumers"))) = (void *) func;

    C16
    typedef int (*test_t)(struct uprobe_multi_consumers *, unsigned long,
    unsigned long, unsigned long);
    static int consumer_test(struct uprobe_multi_consumers *skel,
    unsigned long before, unsigned long after,
    test_t test, unsigned long offset)
    {
    int err, idx, ret = -1;
    printf("consumer_test before %lu after %lu\n", before, after);
// 'before' is each, we attach uprobe for every set idx
    for (idx = 0; idx < 4; idx++) {
    if (test_bit(idx, before)) {
    if (!ASSERT_OK(uprobe_attach(skel, idx, offset), "uprobe_attach_before"))
    goto cleanup;
    }
    }
    err = test(skel, before, after, offset);
    if (!ASSERT_EQ(err, 0, "uprobe_consumer_test"))
    goto cleanup;
    for (idx = 0; idx < 4; idx++) {
    bool uret_stays, uret_survives;
    const char *fmt = "BUG";
    let mut val: __u64 = 0;
    switch (idx) {
    case 0:
//
// uprobe entry
// +1 if define in 'before'
//
    if (test_bit(idx, before))
    val++;
    fmt = "prog 0: uprobe";
    break;
    case 1:
//
// To trigger uretprobe consumer, the uretprobe under test either stayed from
// before to after (uret_stays + test_bit) or uretprobe instance survived and
// we have uretprobe active in after (uret_survives + test_bit)
//
    uret_stays = before & after & 0b0110;
    uret_survives = ((before & 0b0110) && (after & 0b0110) && (before & 0b1001));
    if ((uret_stays || uret_survives) && test_bit(idx, after))
    val++;
    fmt = "prog 1: uretprobe";
    break;
    case 2:
//
// session with return
// +1 if defined in 'before'
// +1 if defined in 'after'
//
    if (test_bit(idx, before)) {
    val++;
    if (test_bit(idx, after))
    val++;
    }
    fmt = "prog 2: session with return";
    break;
    case 3:
//
// session without return
// +1 if defined in 'before'
//
    if (test_bit(idx, before))
    val++;
    fmt = "prog 3: session with NO return";
    break;
    }
    if (!ASSERT_EQ(skel.bss.uprobe_result[idx], val, fmt))
    goto cleanup;
    skel.bss.uprobe_result[idx] = 0;
    }
    ret = 0;
    cleanup:
    for (idx = 0; idx < 4; idx++)
    uprobe_detach(skel, idx);
    return ret;
    }
pub const CONSUMER_MAX: c_int = 16;
//
// Each thread runs 1/16 of the load by running test for single
// 'before' number (based on thread index) and full scale of
// 'after' numbers.
//
    static void *consumer_thread(void *arg)
    {
    let mut idx: c_ulong = (unsigned long) arg;
    struct uprobe_multi_consumers *skel;
    unsigned long offset;
    const void *func;
    int after;
    skel = uprobe_multi_consumers__open_and_load();
    if (!ASSERT_OK_PTR(skel, "uprobe_multi_consumers__open_and_load"))
    return core::ptr::null_mut();
    func = *((&__start_consumers) + idx);
    offset = get_uprobe_offset(func);
    if (!ASSERT_GE(offset, 0, "uprobe_offset"))
    goto out;
    for (after = 0; after < CONSUMER_MAX; after++)
    if (consumer_test(skel, idx, after, func, offset))
    goto out;
    out:
    uprobe_multi_consumers__destroy(skel);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn test_consumers() {
    static void test_consumers(void)
    {
    pthread_t pt[CONSUMER_MAX];
    unsigned long idx;
    int err;
//
// The idea of this test is to try all possible combinations of
// uprobes consumers attached on single function.
//
// - 1 uprobe entry consumer
// - 1 uprobe exit consumer
// - 1 uprobe session with return
// - 1 uprobe session without return
//
// The test uses 4 uprobes attached on single function, but that
// translates into single uprobe with 4 consumers in kernel.
//
// The before/after values present the state of attached consumers
// before and after the probed function:
//
// bit/prog 0 : uprobe entry
// bit/prog 1 : uprobe return
//
// For example for:
//
// before = 0b01
// after  = 0b10
//
// it means that before we call 'uprobe_consumer_test' we attach
// uprobes defined in 'before' value:
//
// - bit/prog 1: uprobe entry
//
// uprobe_consumer_test is called and inside it we attach and detach
// uprobes based on 'after' value:
//
// - bit/prog 0: is detached
// - bit/prog 1: is attached
//
// uprobe_consumer_test returns and we check counters values increased
// by bpf programs on each uprobe to match the expected count based on
// before/after bits.
//
    for (idx = 0; idx < CONSUMER_MAX; idx++) {
    err = pthread_create(&pt[idx], core::ptr::null_mut(), consumer_thread, (void *) idx);
    if (!ASSERT_OK(err, "pthread_create"))
    break;
    }
    while (idx)
    pthread_join(pt[--idx], core::ptr::null_mut());
    }
    static struct bpf_program *uprobe_multi_program(struct uprobe_multi_pid_filter *skel, int idx)
    {
    switch (idx) {
    case 0: return skel.progs.uprobe_multi_0;
    case 1: return skel.progs.uprobe_multi_1;
    case 2: return skel.progs.uprobe_multi_2;
    }
    return core::ptr::null_mut();
    }
pub const TASKS: c_int = 3;
#[no_mangle]
unsafe extern "C" fn run_pid_filter(skel: *mut uprobe_multi_pid_filter, clone_vm: bool, retprobe: bool) {
    static void run_pid_filter(struct uprobe_multi_pid_filter *skel, bool clone_vm, bool retprobe)
    {
    LIBBPF_OPTS(bpf_uprobe_multi_opts, opts, .retprobe = retprobe);
    struct bpf_link *link[TASKS] = {};
    struct child child[TASKS] = {};
    int i;
    memset(skel.bss.test, 0, sizeof(skel.bss.test));
    for (i = 0; i < TASKS; i++) {
    if (!ASSERT_OK(spawn_child_flag(&child[i], clone_vm), "spawn_child"))
    goto cleanup;
    skel.bss.pids[i] = child[i].pid;
    }
    for (i = 0; i < TASKS; i++) {
    link[i] = bpf_program__attach_uprobe_multi(uprobe_multi_program(skel, i),
    child[i].pid, "/proc/self/exe",
    "uprobe_multi_func_1", &opts);
    if (!ASSERT_OK_PTR(link[i], "bpf_program__attach_uprobe_multi"))
    goto cleanup;
    }
    for (i = 0; i < TASKS; i++)
    kick_child(&child[i]);
    for (i = 0; i < TASKS; i++) {
    ASSERT_EQ(skel.bss.test[i][0], 1, "pid");
    ASSERT_EQ(skel.bss.test[i][1], 0, "unknown");
    }
    cleanup:
    for (i = 0; i < TASKS; i++)
    bpf_link__destroy(link[i]);
    for (i = 0; i < TASKS; i++)
    release_child(&child[i]);
    }
#[no_mangle]
unsafe extern "C" fn test_pid_filter_process(clone_vm: bool) {
    static void test_pid_filter_process(bool clone_vm)
    {
    struct uprobe_multi_pid_filter *skel;
    skel = uprobe_multi_pid_filter__open_and_load();
    if (!ASSERT_OK_PTR(skel, "uprobe_multi_pid_filter__open_and_load"))
    return;
    run_pid_filter(skel, clone_vm, false);
    run_pid_filter(skel, clone_vm, true);
    uprobe_multi_pid_filter__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_session_skel_api() {
    static void test_session_skel_api(void)
    {
    struct uprobe_multi_session *skel = core::ptr::null_mut();
    LIBBPF_OPTS(bpf_kprobe_multi_opts, opts);
    struct bpf_link *link = core::ptr::null_mut();
    int err;
    skel = uprobe_multi_session__open_and_load();
    if (!ASSERT_OK_PTR(skel, "uprobe_multi_session__open_and_load"))
    goto cleanup;
    skel.bss.pid = getpid();
    skel.bss.user_ptr = test_data;
    err = uprobe_multi_session__attach(skel);
    if (!ASSERT_OK(err, "uprobe_multi_session__attach"))
    goto cleanup;
// trigger all probes
    skel.bss.uprobe_multi_func_1_addr = (__u64) uprobe_multi_func_1;
    skel.bss.uprobe_multi_func_2_addr = (__u64) uprobe_multi_func_2;
    skel.bss.uprobe_multi_func_3_addr = (__u64) uprobe_multi_func_3;
    uprobe_multi_func_1();
    uprobe_multi_func_2();
    uprobe_multi_func_3();
//
// We expect 2 for uprobe_multi_func_2 because it runs both entry/return probe,
// uprobe_multi_func_[13] run just the entry probe. All expected numbers are
// doubled, because we run extra test for sleepable session.
//
    ASSERT_EQ(skel.bss.uprobe_session_result[0], 2, "uprobe_multi_func_1_result");
    ASSERT_EQ(skel.bss.uprobe_session_result[1], 4, "uprobe_multi_func_2_result");
    ASSERT_EQ(skel.bss.uprobe_session_result[2], 2, "uprobe_multi_func_3_result");
// We expect increase in 3 entry and 1 return session calls -> 4
    ASSERT_EQ(skel.bss.uprobe_multi_sleep_result, 4, "uprobe_multi_sleep_result");
    cleanup:
    bpf_link__destroy(link);
    uprobe_multi_session__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_session_single_skel_api() {
    static void test_session_single_skel_api(void)
    {
    struct uprobe_multi_session_single *skel = core::ptr::null_mut();
    LIBBPF_OPTS(bpf_kprobe_multi_opts, opts);
    int err;
    skel = uprobe_multi_session_single__open_and_load();
    if (!ASSERT_OK_PTR(skel, "uprobe_multi_session_single__open_and_load"))
    goto cleanup;
    skel.bss.pid = getpid();
    err = uprobe_multi_session_single__attach(skel);
    if (!ASSERT_OK(err, "uprobe_multi_session_single__attach"))
    goto cleanup;
    uprobe_multi_func_1();
//
// We expect consumer 0 and 2 to trigger just entry handler (value 1)
// and consumer 1 to hit both (value 2).
//
    ASSERT_EQ(skel.bss.uprobe_session_result[0], 1, "uprobe_session_result_0");
    ASSERT_EQ(skel.bss.uprobe_session_result[1], 2, "uprobe_session_result_1");
    ASSERT_EQ(skel.bss.uprobe_session_result[2], 1, "uprobe_session_result_2");
    cleanup:
    uprobe_multi_session_single__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_session_cookie_skel_api() {
    static void test_session_cookie_skel_api(void)
    {
    struct uprobe_multi_session_cookie *skel = core::ptr::null_mut();
    int err;
    skel = uprobe_multi_session_cookie__open_and_load();
    if (!ASSERT_OK_PTR(skel, "uprobe_multi_session_cookie__open_and_load"))
    goto cleanup;
    skel.bss.pid = getpid();
    err = uprobe_multi_session_cookie__attach(skel);
    if (!ASSERT_OK(err, "uprobe_multi_session_cookie__attach"))
    goto cleanup;
// trigger all probes
    uprobe_multi_func_1();
    uprobe_multi_func_2();
    uprobe_multi_func_3();
    ASSERT_EQ(skel.bss.test_uprobe_1_result, 1, "test_uprobe_1_result");
    ASSERT_EQ(skel.bss.test_uprobe_2_result, 2, "test_uprobe_2_result");
    ASSERT_EQ(skel.bss.test_uprobe_3_result, 3, "test_uprobe_3_result");
    cleanup:
    uprobe_multi_session_cookie__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_session_recursive_skel_api() {
    static void test_session_recursive_skel_api(void)
    {
    struct uprobe_multi_session_recursive *skel = core::ptr::null_mut();
    int i, err;
    skel = uprobe_multi_session_recursive__open_and_load();
    if (!ASSERT_OK_PTR(skel, "uprobe_multi_session_recursive__open_and_load"))
    goto cleanup;
    skel.bss.pid = getpid();
    err = uprobe_multi_session_recursive__attach(skel);
    if (!ASSERT_OK(err, "uprobe_multi_session_recursive__attach"))
    goto cleanup;
    for (i = 0; i < ARRAY_SIZE(skel.bss.test_uprobe_cookie_entry); i++)
    skel.bss.test_uprobe_cookie_entry[i] = i + 1;
    uprobe_session_recursive(5);
//
// entry uprobe:
// uprobe_session_recursive(5) {             *cookie = 1, return 0
// uprobe_session_recursive(4) {           *cookie = 2, return 1
// uprobe_session_recursive(3) {         *cookie = 3, return 0
// uprobe_session_recursive(2) {       *cookie = 4, return 1
// uprobe_session_recursive(1) {     *cookie = 5, return 0
// uprobe_session_recursive(0) {   *cookie = 6, return 1
// return uprobe:
// } i = 0                          not executed
// } i = 1                            test_uprobe_cookie_return[0] = 5
// } i = 2                              not executed
// } i = 3                                test_uprobe_cookie_return[1] = 3
// } i = 4                                  not executed
// } i = 5                                    test_uprobe_cookie_return[2] = 1
//
    ASSERT_EQ(skel.bss.idx_entry, 6, "idx_entry");
    ASSERT_EQ(skel.bss.idx_return, 3, "idx_return");
    ASSERT_EQ(skel.bss.test_uprobe_cookie_return[0], 5, "test_uprobe_cookie_return[0]");
    ASSERT_EQ(skel.bss.test_uprobe_cookie_return[1], 3, "test_uprobe_cookie_return[1]");
    ASSERT_EQ(skel.bss.test_uprobe_cookie_return[2], 1, "test_uprobe_cookie_return[2]");
    cleanup:
    uprobe_multi_session_recursive__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_bench_attach_uprobe() {
    static void test_bench_attach_uprobe(void)
    {
    let mut attach_start_ns: c_long = 0, attach_end_ns = 0;
    struct uprobe_multi_bench *skel = core::ptr::null_mut();
    long detach_start_ns, detach_end_ns;
    double attach_delta, detach_delta;
    int err;
    skel = uprobe_multi_bench__open_and_load();
    if (!ASSERT_OK_PTR(skel, "uprobe_multi_bench__open_and_load"))
    goto cleanup;
    attach_start_ns = get_time_ns();
    err = uprobe_multi_bench__attach(skel);
    if (!ASSERT_OK(err, "uprobe_multi_bench__attach"))
    goto cleanup;
    attach_end_ns = get_time_ns();
    system("./uprobe_multi bench");
    ASSERT_EQ(skel.bss.count, 50000, "uprobes_count");
    cleanup:
    detach_start_ns = get_time_ns();
    uprobe_multi_bench__destroy(skel);
    detach_end_ns = get_time_ns();
    attach_delta = (attach_end_ns - attach_start_ns) / 1000000000.0;
    detach_delta = (detach_end_ns - detach_start_ns) / 1000000000.0;
    printf("%s: attached in %7.3lfs\n", __func__, attach_delta);
    printf("%s: detached in %7.3lfs\n", __func__, detach_delta);
    }
#[no_mangle]
unsafe extern "C" fn test_bench_attach_usdt() {
    static void test_bench_attach_usdt(void)
    {
    let mut attach_start_ns: c_long = 0, attach_end_ns = 0;
    struct uprobe_multi_usdt *skel = core::ptr::null_mut();
    long detach_start_ns, detach_end_ns;
    double attach_delta, detach_delta;
    skel = uprobe_multi_usdt__open_and_load();
    if (!ASSERT_OK_PTR(skel, "uprobe_multi__open"))
    goto cleanup;
    attach_start_ns = get_time_ns();
    skel.links.usdt0 = bpf_program__attach_usdt(skel.progs.usdt0, -1, "./uprobe_multi",
    "test", "usdt", core::ptr::null_mut());
    if (!ASSERT_OK_PTR(skel.links.usdt0, "bpf_program__attach_usdt"))
    goto cleanup;
    attach_end_ns = get_time_ns();
    system("./uprobe_multi usdt");
    ASSERT_EQ(skel.bss.count, 50000, "usdt_count");
    cleanup:
    detach_start_ns = get_time_ns();
    uprobe_multi_usdt__destroy(skel);
    detach_end_ns = get_time_ns();
    attach_delta = (attach_end_ns - attach_start_ns) / 1000000000.0;
    detach_delta = (detach_end_ns - detach_start_ns) / 1000000000.0;
    printf("%s: attached in %7.3lfs\n", __func__, attach_delta);
    printf("%s: detached in %7.3lfs\n", __func__, detach_delta);
    }
#[no_mangle]
pub unsafe extern "C" fn test_uprobe_multi_test() {
    void test_uprobe_multi_test(void)
    {
    if (test__start_subtest("skel_api"))
    test_skel_api();
    if (test__start_subtest("attach_api_pattern"))
    test_attach_api_pattern();
    if (test__start_subtest("attach_api_syms"))
    test_attach_api_syms();
    if (test__start_subtest("link_api"))
    test_link_api();
    if (test__start_subtest("link_api_path_fd"))
    test_link_api_path_fd();
    if (test__start_subtest("bench_uprobe"))
    test_bench_attach_uprobe();
    if (test__start_subtest("bench_usdt"))
    test_bench_attach_usdt();
    if (test__start_subtest("attach_api_fails"))
    test_attach_api_fails();
    if (test__start_subtest("attach_uprobe_fails"))
    test_attach_uprobe_fails();
    if (test__start_subtest("consumers"))
    test_consumers();
    if (test__start_subtest("filter_fork"))
    test_pid_filter_process(false);
    if (test__start_subtest("filter_clone_vm"))
    test_pid_filter_process(true);
    if (test__start_subtest("session"))
    test_session_skel_api();
    if (test__start_subtest("session_single"))
    test_session_single_skel_api();
    if (test__start_subtest("session_cookie"))
    test_session_cookie_skel_api();
    if (test__start_subtest("session_cookie_recursive"))
    test_session_recursive_skel_api();
    RUN_TESTS(uprobe_multi_verifier);
    }
