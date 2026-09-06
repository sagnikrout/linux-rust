//! Automatically rewritten from C to Rust
//! Source: arch/um/os-Linux/skas/process.c
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
// Copyright (C) 2015 Thomas Meyer (thomas@m3y3r.de)
// Copyright (C) 2002- 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
//

#[no_mangle]
pub unsafe extern "C" fn is_skas_winch(pid: c_int, fd: c_int, data: *mut c_void) -> c_int {
    int is_skas_winch(int pid, int fd, void *data)
    {
    let mut pid: return = = getpgrp();
    }
    static const char *ptrace_reg_name(int idx)
    {

    switch (idx) {

    R(BX);
    R(CX);
    R(DI);
    R(SI);
    R(DX);
    R(BP);
    R(AX);
    R(R8);
    R(R9);
    R(R10);
    R(R11);
    R(R12);
    R(R13);
    R(R14);
    R(R15);
    R(ORIG_AX);
    R(CS);
    R(SS);
    R(EFLAGS);

    R(IP);
    R(SP);
    R(EFLAGS);
    R(AX);
    R(BX);
    R(CX);
    R(DX);
    R(SI);
    R(DI);
    R(BP);
    R(CS);
    R(SS);
    R(DS);
    R(FS);
    R(ES);
    R(GS);
    R(ORIG_AX);

    }
    return "";
    }
#[no_mangle]
unsafe extern "C" fn ptrace_dump_regs(pid: c_int) -> c_int {
    static int ptrace_dump_regs(int pid)
    {
    unsigned long regs[MAX_REG_NR];
    int i;
    if (ptrace(PTRACE_GETREGS, pid, 0, regs) < 0)
    return -errno;
    printk(UM_KERN_ERR "Stub registers -\n");
    for (i = 0; i < ARRAY_SIZE(regs); i++) {
    const char *regname = ptrace_reg_name(i);
    printk(UM_KERN_ERR "\t%s\t(%2d): %lx\n", regname, i, regs[i]);
    }
    return 0;
    }
//
// Signals that are OK to receive in the stub - we'll just continue it.
// SIGWINCH will happen when UML is inside a detached screen.
//

// Signals that the stub will finish with - anything else is an error

#[no_mangle]
pub unsafe extern "C" fn wait_stub_done(pid: c_int) {
    void wait_stub_done(int pid)
    {
    int n, status, err;
    while (1) {
    CATCH_EINTR(n = waitpid(pid, &status, WUNTRACED | __WALL));
    if ((n < 0) || !WIFSTOPPED(status))
    goto bad_wait;
    if (((1 << WSTOPSIG(status)) & STUB_SIG_MASK) == 0)
    break;
    err = ptrace(PTRACE_CONT, pid, 0, 0);
    if (err) {
    printk(UM_KERN_ERR "%s : continue failed, errno = %d\n",
    __func__, errno);
    fatal_sigsegv();
    }
    }
    if (((1 << WSTOPSIG(status)) & STUB_DONE_MASK) != 0)
    return;
    bad_wait:
    err = ptrace_dump_regs(pid);
    if (err)
    printk(UM_KERN_ERR "Failed to get registers from stub, errno = %d\n",
    -err);
    printk(UM_KERN_ERR "%s : failed to wait for SIGTRAP, pid = %d, n = %d, errno = %d, status = 0x%x\n",
    __func__, pid, n, errno, status);
    fatal_sigsegv();
    }
#[no_mangle]
pub unsafe extern "C" fn wait_stub_done_seccomp(mm_idp: *mut mm_id, running: c_int, wait_sigsys: c_int) {
    void wait_stub_done_seccomp(struct mm_id *mm_idp, int running, int wait_sigsys)
    {
    struct stub_data *data = (void *)mm_idp.stack;
    int ret;
    do {
    let mut byte: c_char = 0;
    struct iovec iov = {
    .iov_base = (void *)&byte,
    .iov_len = sizeof(byte),
    };
    union {
    char data[CMSG_SPACE(sizeof(mm_idp.syscall_fd_map))];
    struct cmsghdr align;
    } ctrl;
    struct msghdr msgh = {
    .msg_iov = &iov,
    .msg_iovlen = 1,
    };
    if (!running) {
    if (mm_idp.syscall_fd_num) {
    unsigned int fds_size =
    sizeof(int) * mm_idp.syscall_fd_num;
    struct cmsghdr *cmsg;
    msgh.msg_control = ctrl.data;
    msgh.msg_controllen = CMSG_SPACE(fds_size);
    cmsg = CMSG_FIRSTHDR(&msgh);
    cmsg.cmsg_level = SOL_SOCKET;
    cmsg.cmsg_type = SCM_RIGHTS;
    cmsg.cmsg_len = CMSG_LEN(fds_size);
    memcpy(CMSG_DATA(cmsg), mm_idp.syscall_fd_map,
    fds_size);
    CATCH_EINTR(syscall(__NR_sendmsg, mm_idp.sock,
    &msgh, 0));
    }
    data.signal = 0;
    data.futex = FUTEX_IN_CHILD;
    CATCH_EINTR(syscall(__NR_futex, &data.futex,
    FUTEX_WAKE, 1, core::ptr::null_mut(), core::ptr::null_mut(), 0));
    }
    do {
//
// We need to check whether the child is still alive
// before and after the FUTEX_WAIT call. Before, in
// case it just died but we still updated data->futex
// to FUTEX_IN_CHILD. And after, in case it died while
// we were waiting (and SIGCHLD woke us up, see the
// IRQ handler in mmu.c).
//
// Either way, if PID is negative, then we have no
// choice but to kill the task.
//
    if (__READ_ONCE(mm_idp.pid) < 0)
    goto out_kill;
    ret = syscall(__NR_futex, &data.futex,
    FUTEX_WAIT, FUTEX_IN_CHILD,
    core::ptr::null_mut(), core::ptr::null_mut(), 0);
    if (ret < 0 && errno != EINTR && errno != EAGAIN) {
    printk(UM_KERN_ERR "%s : FUTEX_WAIT failed, errno = %d\n",
    __func__, errno);
    goto out_kill;
    }
    } while (data.futex == FUTEX_IN_CHILD);
    if (__READ_ONCE(mm_idp.pid) < 0)
    goto out_kill;
    running = 0;
// We may receive a SIGALRM before SIGSYS, iterate again.
    } while (wait_sigsys && data.signal == SIGALRM);
    if (data.mctx_offset > sizeof(data.sigstack) - sizeof(mcontext_t)) {
    printk(UM_KERN_ERR "%s : invalid mcontext offset", __func__);
    goto out_kill;
    }
    if (wait_sigsys && data.signal != SIGSYS) {
    printk(UM_KERN_ERR "%s : expected SIGSYS but got %d",
    __func__, data.signal);
    goto out_kill;
    }
    return;
    out_kill:
    printk(UM_KERN_ERR "%s : failed to wait for stub, pid = %d, errno = %d\n",
    __func__, mm_idp.pid, errno);
// This is not true inside start_userspace
    if (current_mm_id() == mm_idp)
    fatal_sigsegv();
    }
    extern unsigned long current_stub_stack(void);
#[no_mangle]
unsafe extern "C" fn get_skas_faultinfo(pid: c_int, fi: *mut faultinfo) {
    static void get_skas_faultinfo(int pid, struct faultinfo *fi)
    {
    int err;
    err = ptrace(PTRACE_CONT, pid, 0, SIGSEGV);
    if (err) {
    printk(UM_KERN_ERR "Failed to continue stub, pid = %d, "
    "errno = %d\n", pid, errno);
    fatal_sigsegv();
    }
    wait_stub_done(pid);
//
// faultinfo is prepared by the stub_segv_handler at start of
// the stub stack page. We just have to copy it.
//
    memcpy(fi, (void *)current_stub_stack(), sizeof(*fi));
    }
#[no_mangle]
unsafe extern "C" fn handle_trap(regs: *mut uml_pt_regs) {
    static void handle_trap(struct uml_pt_regs *regs)
    {
    if ((UPT_IP(regs) >= STUB_START) && (UPT_IP(regs) < STUB_END))
    fatal_sigsegv();
    handle_syscall(regs);
    }
    extern char __syscall_stub_start[];
    static int stub_exe_fd;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tramp_data {
    pub stub_data: *mut stub_data,
// 0 is inherited, 1 is the kernel side
    pub sockpair: [c_int; 2],
}

#[no_mangle]
unsafe extern "C" fn userspace_tramp(data: *mut c_void) -> c_int {
    static int userspace_tramp(void *data)
    {
    struct tramp_data *tramp_data = data;
    char *const argv[] = { "uml-userspace", core::ptr::null_mut() };
    unsigned long long offset;
    struct stub_init_data init_data = {
    .seccomp = using_seccomp,
    .stub_start = STUB_START,
    };
    int ret;
    if (using_seccomp) {
    init_data.signal_handler = STUB_CODE +
    (unsigned long) stub_signal_interrupt -
    (unsigned long) __syscall_stub_start;
    init_data.signal_restorer = STUB_CODE +
    (unsigned long) stub_signal_restorer -
    (unsigned long) __syscall_stub_start;
    } else {
    init_data.signal_handler = STUB_CODE +
    (unsigned long) stub_segv_handler -
    (unsigned long) __syscall_stub_start;
    init_data.signal_restorer = 0;
    }
    init_data.stub_code_fd = phys_mapping(uml_to_phys(__syscall_stub_start),
    &offset);
    init_data.stub_code_offset = MMAP_OFFSET(offset);
    init_data.stub_data_fd = phys_mapping(uml_to_phys(tramp_data.stub_data),
    &offset);
    init_data.stub_data_offset = MMAP_OFFSET(offset);
//
// Avoid leaking unneeded FDs to the stub by setting CLOEXEC on all FDs
// and then unsetting it on all memory related FDs.
// This is not strictly necessary from a safety perspective.
//
    syscall(__NR_close_range, 0, ~0U, CLOSE_RANGE_CLOEXEC);
    fcntl(init_data.stub_data_fd, F_SETFD, 0);
// dup2 signaling FD/socket to STDIN
    if (dup2(tramp_data.sockpair[0], 0) < 0)
    exit(3);
    close(tramp_data.sockpair[0]);
// Write init_data and close write side
    ret = write(tramp_data.sockpair[1], &init_data, sizeof(init_data));
    close(tramp_data.sockpair[1]);
    if (ret != sizeof(init_data))
    exit(4);
// Raw execveat for compatibility with older libc versions
    syscall(__NR_execveat, stub_exe_fd, (unsigned long)"",
    (unsigned long)argv, core::ptr::null_mut(), AT_EMPTY_PATH);
    exit(5);
    }
    extern char stub_exe_start[];
    extern char stub_exe_end[];
    extern char *tempdir;

pub const MFD_EXEC: c_uint = 0x0010U;

#[no_mangle]
unsafe extern "C" fn init_stub_exe_fd() -> int __init {
    static int __init init_stub_exe_fd(void)
    {
    let mut written: usize = 0;
    char *tmpfile = core::ptr::null_mut();
    stub_exe_fd = memfd_create("uml-userspace",
    MFD_EXEC | MFD_CLOEXEC | MFD_ALLOW_SEALING);
    if (stub_exe_fd < 0) {
    printk(UM_KERN_INFO "Could not create executable memfd, using temporary file!");
    tmpfile = malloc(strlen(tempdir) +
    strlen(STUB_EXE_NAME_TEMPLATE) + 1);
    if (tmpfile == core::ptr::null_mut())
    panic("Failed to allocate memory for stub binary name");
    strcpy(tmpfile, tempdir);
    strcat(tmpfile, STUB_EXE_NAME_TEMPLATE);
    stub_exe_fd = mkstemp(tmpfile);
    if (stub_exe_fd < 0)
    panic("Could not create temporary file for stub binary: %d",
    -errno);
    }
    while (written < stub_exe_end - stub_exe_start) {
    ssize_t res = write(stub_exe_fd, stub_exe_start + written,
    stub_exe_end - stub_exe_start - written);
    if (res < 0) {
    if (errno == EINTR)
    continue;
    if (tmpfile)
    unlink(tmpfile);
    panic("Failed write stub binary: %d", -errno);
    }
    written += res;
    }
    if (!tmpfile) {
    fcntl(stub_exe_fd, F_ADD_SEALS,
    F_SEAL_WRITE | F_SEAL_SHRINK | F_SEAL_GROW | F_SEAL_SEAL);
    } else {
    if (fchmod(stub_exe_fd, 00500) < 0) {
    unlink(tmpfile);
    panic("Could not make stub binary executable: %d",
    -errno);
    }
    close(stub_exe_fd);
    stub_exe_fd = open(tmpfile, O_RDONLY | O_CLOEXEC | O_NOFOLLOW);
    if (stub_exe_fd < 0) {
    unlink(tmpfile);
    panic("Could not reopen stub binary: %d", -errno);
    }
    unlink(tmpfile);
    free(tmpfile);
    }
    return 0;
    }
    __initcall(init_stub_exe_fd);
    int using_seccomp;
//
// start_userspace() - prepare a new userspace process
// @mm_id: The corresponding struct mm_id
//
// Setups a new temporary stack page that is used while userspace_tramp() runs
// Clones the kernel process into a new userspace process, with FDs only.
//
// Return: When positive: the process id of the new userspace process,
// when negative: an error number.
// FIXME: can PIDs become negative?!
//
#[no_mangle]
pub unsafe extern "C" fn start_userspace(mm_id: *mut mm_id) -> c_int {
    int start_userspace(struct mm_id *mm_id)
    {
    struct stub_data *proc_data = (void *)mm_id.stack;
    struct tramp_data tramp_data = {
    .stub_data = proc_data,
    };
    void *stack;
    unsigned long sp;
    int status, n, err;
// setup a temporary stack page
    stack = mmap(core::ptr::null_mut(), UM_KERN_PAGE_SIZE,
    PROT_READ | PROT_WRITE | PROT_EXEC,
    MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    if (stack == MAP_FAILED) {
    err = -errno;
    printk(UM_KERN_ERR "%s : mmap failed, errno = %d\n",
    __func__, errno);
    return err;
    }
// set stack pointer to the end of the stack page, so it can grow downwards
    sp = (unsigned long)stack + UM_KERN_PAGE_SIZE;
// socket pair for init data and SECCOMP FD passing (no CLOEXEC here)
    if (socketpair(AF_UNIX, SOCK_STREAM, 0, tramp_data.sockpair)) {
    err = -errno;
    printk(UM_KERN_ERR "%s : socketpair failed, errno = %d\n",
    __func__, errno);
    return err;
    }
    if (using_seccomp)
    proc_data.futex = FUTEX_IN_CHILD;
    mm_id.pid = clone(userspace_tramp, (void *) sp,
    CLONE_VFORK | CLONE_VM | SIGCHLD,
    (void *)&tramp_data);
    if (mm_id.pid < 0) {
    err = -errno;
    printk(UM_KERN_ERR "%s : clone failed, errno = %d\n",
    __func__, errno);
    goto out_close;
    }
    if (using_seccomp) {
    wait_stub_done_seccomp(mm_id, 1, 1);
    } else {
    do {
    CATCH_EINTR(n = waitpid(mm_id.pid, &status,
    WUNTRACED | __WALL));
    if (n < 0) {
    err = -errno;
    printk(UM_KERN_ERR "%s : wait failed, errno = %d\n",
    __func__, errno);
    goto out_kill;
    }
    } while (WIFSTOPPED(status) && (WSTOPSIG(status) == SIGALRM));
    if (!WIFSTOPPED(status) || (WSTOPSIG(status) != SIGSTOP)) {
    err = -EINVAL;
    printk(UM_KERN_ERR "%s : expected SIGSTOP, got status = %d\n",
    __func__, status);
    goto out_kill;
    }
    if (ptrace(PTRACE_SETOPTIONS, mm_id.pid, core::ptr::null_mut(),
    (void *) PTRACE_O_TRACESYSGOOD) < 0) {
    err = -errno;
    printk(UM_KERN_ERR "%s : PTRACE_SETOPTIONS failed, errno = %d\n",
    __func__, errno);
    goto out_kill;
    }
    }
    if (munmap(stack, UM_KERN_PAGE_SIZE) < 0) {
    err = -errno;
    printk(UM_KERN_ERR "%s : munmap failed, errno = %d\n",
    __func__, errno);
    goto out_kill;
    }
    close(tramp_data.sockpair[0]);
    if (using_seccomp)
    mm_id.sock = tramp_data.sockpair[1];
    else
    close(tramp_data.sockpair[1]);
    return 0;
    out_kill:
    os_kill_ptraced_process(mm_id.pid, 1);
    out_close:
    close(tramp_data.sockpair[0]);
    close(tramp_data.sockpair[1]);
    mm_id.pid = -1;
    return err;
    }
    static int unscheduled_userspace_iterations;
    extern unsigned long tt_extra_sched_jiffies;
#[no_mangle]
pub unsafe extern "C" fn userspace(regs: *mut uml_pt_regs) {
    void userspace(struct uml_pt_regs *regs)
    {
    int err, status, op;
    siginfo_t si_local;
    siginfo_t *si;
    int sig;
// Handle any immediate reschedules or signals
    interrupt_end();
    while (1) {
    struct mm_id *mm_id = current_mm_id();
//
// At any given time, only one CPU thread can enter the
// turnstile to operate on the same stub process, including
// executing stub system calls (mmap and munmap).
//
    enter_turnstile(mm_id);
//
// When we are in time-travel mode, userspace can theoretically
// do a *lot* of work without being scheduled. The problem with
// this is that it will prevent kernel bookkeeping (primarily
// the RCU) from running and this can for example cause OOM
// situations.
//
// This code accounts a jiffie against the scheduling clock
// after the defined userspace iterations in the same thread.
// By doing so the situation is effectively prevented.
//
    if (time_travel_mode == TT_MODE_INFCPU ||
    time_travel_mode == TT_MODE_EXTERNAL) {

    if (CONFIG_UML_MAX_USERSPACE_ITERATIONS &&
    unscheduled_userspace_iterations++ >
    CONFIG_UML_MAX_USERSPACE_ITERATIONS) {
    tt_extra_sched_jiffies += 1;
    unscheduled_userspace_iterations = 0;
    }

    }
    time_travel_print_bc_msg();
    current_mm_sync();
    if (using_seccomp) {
    struct stub_data *proc_data = (void *) mm_id.stack;
    err = set_stub_state(regs, proc_data, singlestepping());
    if (err) {
    printk(UM_KERN_ERR "%s - failed to set regs: %d",
    __func__, err);
    fatal_sigsegv();
    }
// Must have been reset by the syscall caller
    if (proc_data.restart_wait != 0)
    panic("Programming error: Flag to only run syscalls in child was not cleared!");
// Mark pending syscalls for flushing
    proc_data.syscall_data_len = mm_id.syscall_data_len;
    wait_stub_done_seccomp(mm_id, 0, 0);
    sig = proc_data.signal;
    if (sig == SIGTRAP && proc_data.err != 0) {
    printk(UM_KERN_ERR "%s - Error flushing stub syscalls",
    __func__);
    syscall_stub_dump_error(mm_id);
    mm_id.syscall_data_len = proc_data.err;
    fatal_sigsegv();
    }
    mm_id.syscall_data_len = 0;
    mm_id.syscall_fd_num = 0;
    err = get_stub_state(regs, proc_data, core::ptr::null_mut());
    if (err) {
    printk(UM_KERN_ERR "%s - failed to get regs: %d",
    __func__, err);
    fatal_sigsegv();
    }
    if (proc_data.si_offset > sizeof(proc_data.sigstack) - sizeof(*si))
    panic("%s - Invalid siginfo offset from child", __func__);
    si = &si_local;
    memcpy(si, &proc_data.sigstack[proc_data.si_offset], sizeof(*si));
    regs.is_user = 1;
// Fill in ORIG_RAX and extract fault information
    PT_SYSCALL_NR(regs.gp) = si.si_syscall;
    if (sig == SIGSEGV) {
    mcontext_t *mcontext = (void *)&proc_data.sigstack[proc_data.mctx_offset];
    GET_FAULTINFO_FROM_MC(regs.faultinfo, mcontext);
    }
    } else {
    let mut pid: c_int = mm_id.pid;
// Flush out any pending syscalls
    err = syscall_stub_flush(mm_id);
    if (err) {
    if (err == -ENOMEM)
    report_enomem();
    printk(UM_KERN_ERR "%s - Error flushing stub syscalls: %d",
    __func__, -err);
    fatal_sigsegv();
    }
//
// This can legitimately fail if the process loads a
// bogus value into a segment register.  It will
// segfault and PTRACE_GETREGS will read that value
// out of the process.  However, PTRACE_SETREGS will
// fail.  In this case, there is nothing to do but
// just kill the process.
//
    if (ptrace(PTRACE_SETREGS, pid, 0, regs.gp)) {
    printk(UM_KERN_ERR "%s - ptrace set regs failed, errno = %d\n",
    __func__, errno);
    fatal_sigsegv();
    }
    if (put_fp_registers(pid, regs.fp)) {
    printk(UM_KERN_ERR "%s - ptrace set fp regs failed, errno = %d\n",
    __func__, errno);
    fatal_sigsegv();
    }
    if (singlestepping())
    op = PTRACE_SYSEMU_SINGLESTEP;
    else
    op = PTRACE_SYSEMU;
    if (ptrace(op, pid, 0, 0)) {
    printk(UM_KERN_ERR "%s - ptrace continue failed, op = %d, errno = %d\n",
    __func__, op, errno);
    fatal_sigsegv();
    }
    CATCH_EINTR(err = waitpid(pid, &status, WUNTRACED | __WALL));
    if (err < 0) {
    printk(UM_KERN_ERR "%s - wait failed, errno = %d\n",
    __func__, errno);
    fatal_sigsegv();
    }
    regs.is_user = 1;
    if (ptrace(PTRACE_GETREGS, pid, 0, regs.gp)) {
    printk(UM_KERN_ERR "%s - PTRACE_GETREGS failed, errno = %d\n",
    __func__, errno);
    fatal_sigsegv();
    }
    if (get_fp_registers(pid, regs.fp)) {
    printk(UM_KERN_ERR "%s -  get_fp_registers failed, errno = %d\n",
    __func__, errno);
    fatal_sigsegv();
    }
    if (WIFSTOPPED(status)) {
    sig = WSTOPSIG(status);
//
// These signal handlers need the si argument
// and SIGSEGV needs the faultinfo.
// The SIGIO and SIGALARM handlers which constitute
// the majority of invocations, do not use it.
//
    switch (sig) {
    case SIGSEGV:
    get_skas_faultinfo(pid,
    &regs.faultinfo);
    fallthrough;
    case SIGTRAP:
    case SIGILL:
    case SIGBUS:
    case SIGFPE:
    case SIGWINCH:
    ptrace(PTRACE_GETSIGINFO, pid, 0,
    (struct siginfo *)&si_local);
    si = &si_local;
    break;
    default:
    si = core::ptr::null_mut();
    break;
    }
    } else {
    sig = 0;
    }
    }
    exit_turnstile(mm_id);
    UPT_SYSCALL_NR(regs) = -1; /* Assume: It's not a syscall */
    if (sig) {
    switch (sig) {
    case SIGSEGV:
    if (using_seccomp || PTRACE_FULL_FAULTINFO)
    (*sig_info[SIGSEGV])(SIGSEGV,
    (struct siginfo *)si,
    regs, core::ptr::null_mut());
    else
    segv(regs.faultinfo, 0, 1, core::ptr::null_mut(), core::ptr::null_mut());
    break;
    case SIGSYS:
    handle_syscall(regs);
    break;
    case SIGTRAP + 0x80:
    handle_trap(regs);
    break;
    case SIGTRAP:
    relay_signal(SIGTRAP, (struct siginfo *)si, regs, core::ptr::null_mut());
    break;
    case SIGALRM:
    break;
    case SIGIO:
    case SIGILL:
    case SIGBUS:
    case SIGFPE:
    case SIGWINCH:
    block_signals_trace();
    (*sig_info[sig])(sig, (struct siginfo *)si, regs, core::ptr::null_mut());
    unblock_signals_trace();
    break;
    default:
    printk(UM_KERN_ERR "%s - child stopped with signal %d\n",
    __func__, sig);
    fatal_sigsegv();
    }
    interrupt_end();
// Avoid -ERESTARTSYS handling in host
    if (PT_SYSCALL_NR_OFFSET != PT_SYSCALL_RET_OFFSET)
    PT_SYSCALL_NR(regs.gp) = -1;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn new_thread(stack: *mut c_void, buf: *mut jmp_buf, (*handler)(void): *mut c_void) {
    void new_thread(void *stack, jmp_buf *buf, void (*handler)(void))
    {
    (*buf)[0].JB_IP = (unsigned long) handler;
    (*buf)[0].JB_SP = (unsigned long) stack + UM_THREAD_SIZE -
    sizeof(void *);
    }
pub const INIT_JMP_NEW_THREAD: c_int = 0;
pub const INIT_JMP_CALLBACK: c_int = 1;
pub const INIT_JMP_HALT: c_int = 2;
pub const INIT_JMP_REBOOT: c_int = 3;
#[no_mangle]
pub unsafe extern "C" fn switch_threads(me: *mut jmp_buf, you: *mut jmp_buf) {
    void switch_threads(jmp_buf *me, jmp_buf *you)
    {
    unscheduled_userspace_iterations = 0;
    if (UML_SETJMP(me) == 0)
    UML_LONGJMP(you, 1);
    }
    static jmp_buf initial_jmpbuf;
    static __thread void (*cb_proc)(void *arg);
    static __thread void *cb_arg;
    static __thread jmp_buf *cb_back;
#[no_mangle]
pub unsafe extern "C" fn start_idle_thread(stack: *mut c_void, switch_buf: *mut jmp_buf) -> c_int {
    int start_idle_thread(void *stack, jmp_buf *switch_buf)
    {
    int n;
    set_handler(SIGWINCH);
//
// Can't use UML_SETJMP or UML_LONGJMP here because they save
// and restore signals, with the possible side-effect of
// trying to handle any signals which came when they were
// blocked, which can't be done on this stack.
// Signals must be blocked when jumping back here and restored
// after returning to the jumper.
//
    n = setjmp(initial_jmpbuf);
    switch (n) {
    case INIT_JMP_NEW_THREAD:
    (*switch_buf)[0].JB_IP = (unsigned long) uml_finishsetup;
    (*switch_buf)[0].JB_SP = (unsigned long) stack +
    UM_THREAD_SIZE - sizeof(void *);
    break;
    case INIT_JMP_CALLBACK:
    (*cb_proc)(cb_arg);
    longjmp(*cb_back, 1);
    break;
    case INIT_JMP_HALT:
    kmalloc_ok = 0;
    return 0;
    case INIT_JMP_REBOOT:
    kmalloc_ok = 0;
    return 1;
    default:
    printk(UM_KERN_ERR "Bad sigsetjmp return in %s - %d\n",
    __func__, n);
    fatal_sigsegv();
    }
    longjmp(*switch_buf, 1);
// unreachable
    printk(UM_KERN_ERR "impossible long jump!");
    fatal_sigsegv();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn initial_thread_cb_skas(): *mut *mut void (proc)(void, arg: *mut c_void) {
    void initial_thread_cb_skas(void (*proc)(void *), void *arg)
    {
    jmp_buf here;
    cb_proc = proc;
    cb_arg = arg;
    cb_back = &here;
    initial_jmpbuf_lock();
    if (UML_SETJMP(&here) == 0)
    UML_LONGJMP(&initial_jmpbuf, INIT_JMP_CALLBACK);
    initial_jmpbuf_unlock();
    cb_proc = core::ptr::null_mut();
    cb_arg = core::ptr::null_mut();
    cb_back = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn halt_skas() {
    void halt_skas(void)
    {
    initial_jmpbuf_lock();
    UML_LONGJMP(&initial_jmpbuf, INIT_JMP_HALT);
// unreachable
    }
    static bool noreboot;
#[no_mangle]
unsafe extern "C" fn noreboot_cmd_param(str: *mut c_char, add: *mut c_int) -> int __init {
    static int __init noreboot_cmd_param(char *str, int *add)
    {
// add = 0;
    noreboot = true;
    return 0;
    }
    __uml_setup("noreboot", noreboot_cmd_param,
    "noreboot\n"
    "    Rather than rebooting, exit always, akin to QEMU's -no-reboot option.\n"
    "    This is useful if you're using CONFIG_PANIC_TIMEOUT in order to catch\n"
    "    crashes in CI\n\n");
#[no_mangle]
pub unsafe extern "C" fn reboot_skas() {
    void reboot_skas(void)
    {
    initial_jmpbuf_lock();
    UML_LONGJMP(&initial_jmpbuf, noreboot ? INIT_JMP_HALT : INIT_JMP_REBOOT);
// unreachable
    }
