//! Automatically rewritten from C to Rust
//! Source: arch/um/os-Linux/start_up.c
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
// Copyright (C) 2021 Benjamin Berg <benjamin@sipsolutions.net>
// Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
//

#[no_mangle]
unsafe extern "C" fn ptrace_child() {
    static void ptrace_child(void)
    {
    int ret;
// Calling os_getpid because some libcs cached getpid incorrectly
    let mut pid: c_int = os_getpid(), ppid = getppid();
    int sc_result;
    if (change_sig(SIGWINCH, 0) < 0 ||
    ptrace(PTRACE_TRACEME, 0, 0, 0) < 0) {
    perror("ptrace");
    kill(pid, SIGKILL);
    }
    kill(pid, SIGSTOP);
//
// This syscall will be intercepted by the parent. Don't call more than
// once, please.
//
    sc_result = os_getpid();
    if (sc_result == pid)
// Nothing modified by the parent, we are running normally.
    ret = 1;
#[no_mangle]
pub unsafe extern "C" fn if(ppid: sc_result ==) -> else {
    else if (sc_result == ppid)
//
// Expected in check_ptrace and check_sysemu when they succeed
// in modifying the stack frame
//
    ret = 0;
    else
// Serious trouble! This could be caused by a bug in host 2.6
// SKAS3/2.6 patch before release -V6, together with a bug in
// the UML code itself.
//
    ret = 2;
    exit(ret);
    }
#[no_mangle]
unsafe extern "C" fn fatal_perror(str: *const c_char) {
    static void fatal_perror(const char *str)
    {
    perror(str);
    exit(1);
    }
#[no_mangle]
unsafe extern "C" fn fatal(fmt: *mut c_char, ...) {
    static void fatal(char *fmt, ...)
    {
    va_list list;
    va_start(list, fmt);
    vfprintf(stderr, fmt, list);
    va_end(list);
    exit(1);
    }
#[no_mangle]
unsafe extern "C" fn non_fatal(fmt: *mut c_char, ...) {
    static void non_fatal(char *fmt, ...)
    {
    va_list list;
    va_start(list, fmt);
    vfprintf(stderr, fmt, list);
    va_end(list);
    }
#[no_mangle]
unsafe extern "C" fn start_ptraced_child() -> c_int {
    static int start_ptraced_child(void)
    {
    int pid, n, status;
    fflush(stdout);
    pid = fork();
    if (pid == 0)
    ptrace_child();
#[no_mangle]
pub unsafe extern "C" fn if(0: pid <) -> else {
    else if (pid < 0)
    fatal_perror("start_ptraced_child : fork failed");
    CATCH_EINTR(n = waitpid(pid, &status, WUNTRACED));
    if (n < 0)
    fatal_perror("check_ptrace : waitpid failed");
    if (!WIFSTOPPED(status) || (WSTOPSIG(status) != SIGSTOP))
    fatal("check_ptrace : expected SIGSTOP, got status = %d",
    status);
    return pid;
    }
#[no_mangle]
unsafe extern "C" fn stop_ptraced_child(pid: c_int, exitcode: c_int) {
    static void stop_ptraced_child(int pid, int exitcode)
    {
    int status, n;
    if (ptrace(PTRACE_CONT, pid, 0, 0) < 0)
    fatal_perror("stop_ptraced_child : ptrace failed");
    CATCH_EINTR(n = waitpid(pid, &status, 0));
    if (!WIFEXITED(status) || (WEXITSTATUS(status) != exitcode)) {
    let mut exit_with: c_int = WEXITSTATUS(status);
    fatal("stop_ptraced_child : child exited with exitcode %d, "
    "while expecting %d; status 0x%x\n", exit_with,
    exitcode, status);
    }
    }
#[no_mangle]
unsafe extern "C" fn check_sysemu() -> void __init {
    static void __init check_sysemu(void)
    {
    int pid, n, status, count=0;
    os_info("Checking syscall emulation for ptrace...");
    pid = start_ptraced_child();
    if ((ptrace(PTRACE_SETOPTIONS, pid, 0,
    (void *) PTRACE_O_TRACESYSGOOD) < 0))
    fatal_perror("check_sysemu: PTRACE_SETOPTIONS failed");
    while (1) {
    count++;
    if (ptrace(PTRACE_SYSEMU_SINGLESTEP, pid, 0, 0) < 0)
    goto fail;
    CATCH_EINTR(n = waitpid(pid, &status, WUNTRACED));
    if (n < 0)
    fatal_perror("check_sysemu: wait failed");
    if (WIFSTOPPED(status) &&
    (WSTOPSIG(status) == (SIGTRAP|0x80))) {
    if (!count) {
    non_fatal("check_sysemu: SYSEMU_SINGLESTEP "
    "doesn't singlestep");
    goto fail;
    }
    n = ptrace(PTRACE_POKEUSER, pid, PT_SYSCALL_RET_OFFSET,
    os_getpid());
    if (n < 0)
    fatal_perror("check_sysemu : failed to modify "
    "system call return");
    break;
    }
#[no_mangle]
pub unsafe extern "C" fn if(SIGTRAP): WIFSTOPPED(status) && (WSTOPSIG(status) ==) -> else {
    else if (WIFSTOPPED(status) && (WSTOPSIG(status) == SIGTRAP))
    count++;
    else {
    non_fatal("check_sysemu: expected SIGTRAP or "
    "(SIGTRAP | 0x80), got status = %d\n",
    status);
    goto fail;
    }
    }
    stop_ptraced_child(pid, 0);
    os_info("OK\n");
    return;
    fail:
    stop_ptraced_child(pid, 1);
    fatal("missing\n");
    }
#[no_mangle]
unsafe extern "C" fn check_ptrace() -> void __init {
    static void __init check_ptrace(void)
    {
    int pid, syscall, n, status;
    os_info("Checking that ptrace can change system call numbers...");
    pid = start_ptraced_child();
    if ((ptrace(PTRACE_SETOPTIONS, pid, 0,
    (void *) PTRACE_O_TRACESYSGOOD) < 0))
    fatal_perror("check_ptrace: PTRACE_SETOPTIONS failed");
    while (1) {
    if (ptrace(PTRACE_SYSCALL, pid, 0, 0) < 0)
    fatal_perror("check_ptrace : ptrace failed");
    CATCH_EINTR(n = waitpid(pid, &status, WUNTRACED));
    if (n < 0)
    fatal_perror("check_ptrace : wait failed");
    if (!WIFSTOPPED(status) ||
    (WSTOPSIG(status) != (SIGTRAP | 0x80)))
    fatal("check_ptrace : expected (SIGTRAP|0x80), "
    "got status = %d", status);
    syscall = ptrace(PTRACE_PEEKUSER, pid, PT_SYSCALL_NR_OFFSET,
    0);
    if (syscall == __NR_getpid) {
    n = ptrace(PTRACE_POKEUSER, pid, PT_SYSCALL_NR_OFFSET,
    __NR_getppid);
    if (n < 0)
    fatal_perror("check_ptrace : failed to modify "
    "system call");
    break;
    }
    }
    stop_ptraced_child(pid, 0);
    os_info("OK\n");
    check_sysemu();
    }
    extern unsigned long host_fp_size;
    extern unsigned long exec_regs[MAX_REG_NR];
    extern unsigned long *exec_fp_regs;
    __initdata static struct stub_data *seccomp_test_stub_data;
#[no_mangle]
unsafe extern "C" fn sigsys_handler(sig: c_int, info: *mut siginfo_t, p: *mut c_void) -> void __init {
    static void __init sigsys_handler(int sig, siginfo_t *info, void *p)
    {
    ucontext_t *uc = p;
// Stow away the location of the mcontext in the stack
    seccomp_test_stub_data.mctx_offset = (unsigned long)&uc.uc_mcontext -
    (unsigned long)&seccomp_test_stub_data.sigstack[0];
// Prevent libc from clearing memory (mctx_offset in particular)
    syscall(__NR_exit, 0);
    }
#[no_mangle]
unsafe extern "C" fn seccomp_helper(data: *mut c_void) -> int __init {
    static int __init seccomp_helper(void *data)
    {
    static struct sock_filter filter[] = {
    BPF_STMT(BPF_LD | BPF_W | BPF_ABS,
    offsetof(struct seccomp_data, nr)),
    BPF_JUMP(BPF_JMP | BPF_JEQ | BPF_K, __NR_clock_nanosleep, 1, 0),
    BPF_STMT(BPF_RET | BPF_K, SECCOMP_RET_ALLOW),
    BPF_STMT(BPF_RET | BPF_K, SECCOMP_RET_TRAP),
    };
    static struct sock_fprog prog = {
    .len = ARRAY_SIZE(filter),
    .filter = filter,
    };
    struct sigaction sa;
// close_range is needed for the stub
    if (stub_syscall3(__NR_close_range, 1, ~0U, 0))
    exit(1);
    set_sigstack(seccomp_test_stub_data.sigstack,
    sizeof(seccomp_test_stub_data.sigstack));
    sa.sa_flags = SA_ONSTACK | SA_NODEFER | SA_SIGINFO;
    sa.sa_sigaction = (void *) sigsys_handler;
    sa.sa_restorer = core::ptr::null_mut();
    if (sigaction(SIGSYS, &sa, core::ptr::null_mut()) < 0)
    exit(2);
    prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0);
    if (syscall(__NR_seccomp, SECCOMP_SET_MODE_FILTER,
    SECCOMP_FILTER_FLAG_TSYNC, &prog) != 0)
    exit(3);
    sleep(0);
// Never reached.
    _exit(4);
    }
#[no_mangle]
unsafe extern "C" fn init_seccomp() -> bool __init {
    static bool __init init_seccomp(void)
    {
    int pid;
    int status;
    int n;
    unsigned long sp;
//
// We check that we can install a seccomp filter and then exit(0)
// from a trapped syscall.
//
// Note that we cannot verify that no seccomp filter already exists
// for a syscall that results in the process/thread to be killed.
//
    os_info("Checking that seccomp filters can be installed...");
    seccomp_test_stub_data = mmap(0, sizeof(*seccomp_test_stub_data),
    PROT_READ | PROT_WRITE,
    MAP_SHARED | MAP_ANON, 0, 0);
// Use the syscall data area as stack, we just need something
    sp = (unsigned long)&seccomp_test_stub_data.syscall_data +
    sizeof(seccomp_test_stub_data.syscall_data) -
    sizeof(void *);
    pid = clone(seccomp_helper, (void *)sp, CLONE_VFORK | CLONE_VM, core::ptr::null_mut());
    if (pid < 0)
    fatal_perror("check_seccomp : clone failed");
    CATCH_EINTR(n = waitpid(pid, &status, __WCLONE));
    if (n < 0)
    fatal_perror("check_seccomp : waitpid failed");
    if (WIFEXITED(status) && WEXITSTATUS(status) == 0) {
    struct uml_pt_regs *regs;
    unsigned long fp_size;
    int r;
// Fill in the host_fp_size from the mcontext.
    regs = calloc(1, sizeof(struct uml_pt_regs));
    get_stub_state(regs, seccomp_test_stub_data, &fp_size);
    host_fp_size = fp_size;
    free(regs);
// Repeat with the correct size
    regs = calloc(1, sizeof(struct uml_pt_regs) + host_fp_size);
    r = get_stub_state(regs, seccomp_test_stub_data, core::ptr::null_mut());
// Store as the default startup registers
    exec_fp_regs = malloc(host_fp_size);
    memcpy(exec_regs, regs.gp, sizeof(exec_regs));
    memcpy(exec_fp_regs, regs.fp, host_fp_size);
    munmap(seccomp_test_stub_data, sizeof(*seccomp_test_stub_data));
    free(regs);
    if (r) {
    os_info("failed to fetch registers: %d\n", r);
    return false;
    }
    os_info("OK\n");
    return true;
    }
    if (WIFEXITED(status) && WEXITSTATUS(status) == 2)
    os_info("missing\n");
    else
    os_info("error\n");
    munmap(seccomp_test_stub_data, sizeof(*seccomp_test_stub_data));
    return false;
    }
#[no_mangle]
unsafe extern "C" fn check_coredump_limit() -> void __init {
    static void __init check_coredump_limit(void)
    {
    struct rlimit lim;
    let mut err: c_int = getrlimit(RLIMIT_CORE, &lim);
    if (err) {
    perror("Getting core dump limit");
    return;
    }
    os_info("Core dump limits :\n\tsoft - ");
    if (lim.rlim_cur == RLIM_INFINITY)
    os_info("NONE\n");
    else
    os_info("%llu\n", (unsigned long long)lim.rlim_cur);
    os_info("\thard - ");
    if (lim.rlim_max == RLIM_INFINITY)
    os_info("NONE\n");
    else
    os_info("%llu\n", (unsigned long long)lim.rlim_max);
    }
    void  __init get_host_cpu_features(
    void (*flags_helper_func)(char *line),
    void (*cache_helper_func)(char *line))
    {
    FILE *cpuinfo;
    char *line = core::ptr::null_mut();
    let mut len: usize = 0;
    let mut done_parsing: c_int = 0;
    cpuinfo = fopen("/proc/cpuinfo", "r");
    if (cpuinfo == core::ptr::null_mut()) {
    os_info("Failed to get host CPU features\n");
    } else {
    while ((getline(&line, &len, cpuinfo)) != -1) {
    if (strstr(line, "flags")) {
    flags_helper_func(line);
    done_parsing++;
    }
    if (strstr(line, "cache_alignment")) {
    cache_helper_func(line);
    done_parsing++;
    }
    free(line);
    line = core::ptr::null_mut();
    if (done_parsing > 1)
    break;
    }
    fclose(cpuinfo);
    }
    }
    static int seccomp_config __initdata;
#[no_mangle]
unsafe extern "C" fn uml_seccomp_config(line: *mut c_char, add: *mut c_int) -> int __init {
    static int __init uml_seccomp_config(char *line, int *add)
    {
// add = 0;
    if (strcmp(line, "off") == 0)
    seccomp_config = 0;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcmp(line, 0: "auto") ==) -> else {
    else if (strcmp(line, "auto") == 0)
    seccomp_config = 1;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcmp(line, 0: "on") ==) -> else {
    else if (strcmp(line, "on") == 0)
    seccomp_config = 2;
    else
    fatal("Invalid seccomp option '%s', expected on/auto/off\n",
    line);
    return 0;
    }
    __uml_setup("seccomp=", uml_seccomp_config,
    "seccomp=<on/auto/off>\n"
    "    Configure whether or not SECCOMP is used. With SECCOMP, userspace\n"
    "    processes work collaboratively with the kernel instead of being\n"
    "    traced using ptrace. All syscalls from the application are caught and\n"
    "    redirected using a signal. This signal handler in turn is permitted to\n"
    "    do the selected set of syscalls to communicate with the UML kernel and\n"
    "    do the required memory management.\n"
    "\n"
    "    This method is overall faster than the ptrace based userspace, primarily\n"
    "    because it reduces the number of context switches for (minor) page faults.\n"
    "\n"
    "    However, the SECCOMP filter is not (yet) restrictive enough to prevent\n"
    "    userspace from reading and writing all physical memory. Userspace\n"
    "    processes could also trick the stub into disabling SIGALRM which\n"
    "    prevents it from being interrupted for scheduling purposes.\n"
    "\n"
    "    This is insecure and should only be used with a trusted userspace\n\n"
    );
#[no_mangle]
pub unsafe extern "C" fn os_early_checks() -> void __init {
    void __init os_early_checks(void)
    {
    int pid;
// Print out the core dump limits early
    check_coredump_limit();
// Need to check this early because mmapping happens before the
// kernel is running.
//
    check_tmpexec();
    if (seccomp_config) {
    if (init_seccomp()) {
    using_seccomp = 1;
    return;
    }
    if (seccomp_config == 2)
    fatal("SECCOMP userspace requested but not functional!\n");
    }
    if (uml_ncpus > 1)
    fatal("SMP is not supported with PTRACE userspace.\n");
    using_seccomp = 0;
    check_ptrace();
    pid = start_ptraced_child();
    if (init_pid_registers(pid))
    fatal("Failed to initialize default registers");
    stop_ptraced_child(pid, 1);
    }
