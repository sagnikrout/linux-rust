//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/uprobe_syscall.c
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

// Macro flag: #define _GNU_SOURCE

    __attribute__((aligned(16)))
#[no_mangle]
pub unsafe extern "C" fn uprobe_regs_trigger() -> __nocf_check __weak __naked unsigned long {
    __nocf_check __weak __naked unsigned long uprobe_regs_trigger(void)
    {
    asm volatile (
    ".byte 0x66, 0x2e, 0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00\n" /* nop10 */
    "movq $0xdeadbeef, %rax\n"
    "ret\n"
    );
    }
#[no_mangle]
pub unsafe extern "C" fn uprobe_regs(before: *mut pt_regs, after: *mut pt_regs) -> __naked void {
    __naked void uprobe_regs(struct pt_regs *before, struct pt_regs *after)
    {
    asm volatile (
    "movq %r15,   0(%rdi)\n"
    "movq %r14,   8(%rdi)\n"
    "movq %r13,  16(%rdi)\n"
    "movq %r12,  24(%rdi)\n"
    "movq %rbp,  32(%rdi)\n"
    "movq %rbx,  40(%rdi)\n"
    "movq %r11,  48(%rdi)\n"
    "movq %r10,  56(%rdi)\n"
    "movq  %r9,  64(%rdi)\n"
    "movq  %r8,  72(%rdi)\n"
    "movq %rax,  80(%rdi)\n"
    "movq %rcx,  88(%rdi)\n"
    "movq %rdx,  96(%rdi)\n"
    "movq %rsi, 104(%rdi)\n"
    "movq %rdi, 112(%rdi)\n"
    "movq   $0, 120(%rdi)\n" /* orig_rax */
    "movq   $0, 128(%rdi)\n" /* rip      */
    "movq   $0, 136(%rdi)\n" /* cs       */
    "pushq %rax\n"
    "pushf\n"
    "pop %rax\n"
    "movq %rax, 144(%rdi)\n" /* eflags   */
    "pop %rax\n"
    "movq %rsp, 152(%rdi)\n" /* rsp      */
    "movq   $0, 160(%rdi)\n" /* ss       */
// save 2nd argument
    "pushq %rsi\n"
    "call uprobe_regs_trigger\n"
// save  return value and load 2nd argument pointer to rax
    "pushq %rax\n"
    "movq 8(%rsp), %rax\n"
    "movq %r15,   0(%rax)\n"
    "movq %r14,   8(%rax)\n"
    "movq %r13,  16(%rax)\n"
    "movq %r12,  24(%rax)\n"
    "movq %rbp,  32(%rax)\n"
    "movq %rbx,  40(%rax)\n"
    "movq %r11,  48(%rax)\n"
    "movq %r10,  56(%rax)\n"
    "movq  %r9,  64(%rax)\n"
    "movq  %r8,  72(%rax)\n"
    "movq %rcx,  88(%rax)\n"
    "movq %rdx,  96(%rax)\n"
    "movq %rsi, 104(%rax)\n"
    "movq %rdi, 112(%rax)\n"
    "movq   $0, 120(%rax)\n" /* orig_rax */
    "movq   $0, 128(%rax)\n" /* rip      */
    "movq   $0, 136(%rax)\n" /* cs       */
// restore return value and 2nd argument
    "pop %rax\n"
    "pop %rsi\n"
    "movq %rax,  80(%rsi)\n"
    "pushf\n"
    "pop %rax\n"
    "movq %rax, 144(%rsi)\n" /* eflags   */
    "movq %rsp, 152(%rsi)\n" /* rsp      */
    "movq   $0, 160(%rsi)\n" /* ss       */
    "ret\n"
    );
    }
#[no_mangle]
unsafe extern "C" fn test_uprobe_regs_equal(retprobe: bool) {
    static void test_uprobe_regs_equal(bool retprobe)
    {
    LIBBPF_OPTS(bpf_uprobe_opts, opts,
    .retprobe = retprobe,
    );
    struct uprobe_syscall *skel = core::ptr::null_mut();
    let mut before: pt_regs = {}, after = {};
    unsigned long *pb = (unsigned long *) &before;
    unsigned long *pa = (unsigned long *) &after;
    unsigned long *pp;
    unsigned long offset;
    unsigned int i, cnt;
    offset = get_uprobe_offset(&uprobe_regs_trigger);
    if (!ASSERT_GE(offset, 0, "get_uprobe_offset"))
    return;
    skel = uprobe_syscall__open_and_load();
    if (!ASSERT_OK_PTR(skel, "uprobe_syscall__open_and_load"))
    goto cleanup;
    skel.links.probe = bpf_program__attach_uprobe_opts(skel.progs.probe,
    0, "/proc/self/exe", offset, &opts);
    if (!ASSERT_OK_PTR(skel.links.probe, "bpf_program__attach_uprobe_opts"))
    goto cleanup;
// make sure uprobe gets optimized
    if (!retprobe)
    uprobe_regs_trigger();
    uprobe_regs(&before, &after);
    pp = (unsigned long *) &skel.bss.regs;
    cnt = sizeof(before)/sizeof(*pb);
    for (i = 0; i < cnt; i++) {
    let mut offset: c_uint = i * sizeof(unsigned long);
//
// Check register before and after uprobe_regs_trigger call
// that triggers the uretprobe.
//
    switch (offset) {
    case offsetof(struct pt_regs, rax):
    ASSERT_EQ(pa[i], 0xdeadbeef, "return value");
    break;
    default:
    if (!ASSERT_EQ(pb[i], pa[i], "register before-after value check"))
    fprintf(stdout, "failed register offset %u\n", offset);
    }
//
// Check register seen from bpf program and register after
// uprobe_regs_trigger call (with rax exception, check below).
//
    switch (offset) {
//
// These values will be different (not set in uretprobe_regs),
// we don't care.
//
    case offsetof(struct pt_regs, orig_rax):
    case offsetof(struct pt_regs, rip):
    case offsetof(struct pt_regs, cs):
    case offsetof(struct pt_regs, rsp):
    case offsetof(struct pt_regs, ss):
    break;
//
// uprobe does not see return value in rax, it needs to see the
// original (before) rax value
//
    case offsetof(struct pt_regs, rax):
    if (!retprobe) {
    ASSERT_EQ(pp[i], pb[i], "uprobe rax prog-before value check");
    break;
    }
    default:
    if (!ASSERT_EQ(pp[i], pa[i], "register prog-after value check"))
    fprintf(stdout, "failed register offset %u\n", offset);
    }
    }
    cleanup:
    uprobe_syscall__destroy(skel);
    }

#[no_mangle]
unsafe extern "C" fn write_bpf_testmod_uprobe(offset: c_ulong) -> c_int {
    static int write_bpf_testmod_uprobe(unsigned long offset)
    {
    size_t n, ret;
    char buf[30];
    int fd;
    n = sprintf(buf, "%lu", offset);
    fd = open(BPF_TESTMOD_UPROBE_TEST_FILE, O_WRONLY);
    if (fd < 0)
    return -errno;
    ret = write(fd, buf, n);
    close(fd);
    return ret != n ? (int) ret : 0;
    }
#[no_mangle]
unsafe extern "C" fn test_regs_change() {
    static void test_regs_change(void)
    {
    let mut before: pt_regs = {}, after = {};
    unsigned long *pb = (unsigned long *) &before;
    unsigned long *pa = (unsigned long *) &after;
    let mut cnt: c_ulong = sizeof(before)/sizeof(*pb);
    unsigned int i, err, offset;
    offset = get_uprobe_offset(uprobe_regs_trigger);
    err = write_bpf_testmod_uprobe(offset);
    if (!ASSERT_OK(err, "register_uprobe"))
    return;
// make sure uprobe gets optimized
    uprobe_regs_trigger();
    uprobe_regs(&before, &after);
    err = write_bpf_testmod_uprobe(0);
    if (!ASSERT_OK(err, "unregister_uprobe"))
    return;
    for (i = 0; i < cnt; i++) {
    let mut offset: c_uint = i * sizeof(unsigned long);
    switch (offset) {
    case offsetof(struct pt_regs, rax):
    ASSERT_EQ(pa[i], 0x12345678deadbeef, "rax");
    break;
    case offsetof(struct pt_regs, rcx):
    ASSERT_EQ(pa[i], 0x87654321feebdaed, "rcx");
    break;
    case offsetof(struct pt_regs, r11):
    ASSERT_EQ(pa[i], (__u64) -1, "r11");
    break;
    default:
    if (!ASSERT_EQ(pa[i], pb[i], "register before-after value check"))
    fprintf(stdout, "failed register offset %u\n", offset);
    }
    }
    }

pub const __NR_uretprobe: c_int = 335;

#[no_mangle]
pub unsafe extern "C" fn uretprobe_syscall_call_1() -> __naked unsigned long {
    __naked unsigned long uretprobe_syscall_call_1(void)
    {
//
// Pretend we are uretprobe trampoline to trigger the return
// probe invocation in order to verify we get SIGILL.
//
    asm volatile (
    "pushq %rax\n"
    "pushq %rcx\n"
    "pushq %r11\n"
    "movq $" __stringify(__NR_uretprobe) ", %rax\n"
    "syscall\n"
    "popq %r11\n"
    "popq %rcx\n"
    "retq\n"
    );
    }
#[no_mangle]
pub unsafe extern "C" fn uretprobe_syscall_call() -> __naked unsigned long {
    __naked unsigned long uretprobe_syscall_call(void)
    {
    asm volatile (
    "call uretprobe_syscall_call_1\n"
    "retq\n"
    );
    }
#[no_mangle]
unsafe extern "C" fn test_uretprobe_syscall_call() {
    static void test_uretprobe_syscall_call(void)
    {
    LIBBPF_OPTS(bpf_uprobe_multi_opts, opts,
    .retprobe = true,
    );
    struct uprobe_syscall_executed *skel;
    int pid, status, err, go[2], c = 0;
    struct bpf_link *link;
    if (!ASSERT_OK(pipe(go), "pipe"))
    return;
    skel = uprobe_syscall_executed__open_and_load();
    if (!ASSERT_OK_PTR(skel, "uprobe_syscall_executed__open_and_load"))
    goto cleanup;
    pid = fork();
    if (!ASSERT_GE(pid, 0, "fork"))
    goto cleanup;
// child
    if (pid == 0) {
    close(go[1]);
// wait for parent's kick
    err = read(go[0], &c, 1);
    if (err != 1)
    exit(-1);
    uretprobe_syscall_call();
    _exit(0);
    }
    skel.bss.pid = pid;
    link = bpf_program__attach_uprobe_multi(skel.progs.test_uretprobe_multi,
    pid, "/proc/self/exe",
    "uretprobe_syscall_call", &opts);
    if (!ASSERT_OK_PTR(link, "bpf_program__attach_uprobe_multi"))
    goto cleanup;
    skel.links.test_uretprobe_multi = link;
// kick the child
    write(go[1], &c, 1);
    err = waitpid(pid, &status, 0);
    ASSERT_EQ(err, pid, "waitpid");
// verify the child got killed with SIGILL
    ASSERT_EQ(WIFSIGNALED(status), 1, "WIFSIGNALED");
    ASSERT_EQ(WTERMSIG(status), SIGILL, "WTERMSIG");
// verify the uretprobe program wasn't called
    ASSERT_EQ(skel.bss.executed, 0, "executed");
    cleanup:
    uprobe_syscall_executed__destroy(skel);
    close(go[1]);
    close(go[0]);
    }

    __attribute__((aligned(16)))
#[no_mangle]
pub unsafe extern "C" fn uprobe_test() -> __nocf_check __weak __naked void {
    __nocf_check __weak __naked void uprobe_test(void)
    {
    asm volatile (
    ".byte 0x66, 0x2e, 0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00\n" /* nop10 */
    "ret\n"
    );
    }
    __attribute__((aligned(16)))
#[no_mangle]
pub unsafe extern "C" fn usdt_test() -> __nocf_check __weak void {
    __nocf_check __weak void usdt_test(void)
    {
    USDT(optimized_uprobe, usdt);
    }
//
// Assembly-level red zone clobbering test. Stores known values in the
// red zone (below RSP), executes a nop10 (uprobe site), and checks that
// the values survived. Returns 0 if intact, 1 if clobbered.
//
// The nop5 optimization used CALL (which pushes a return address to
// [rsp-8]), the value at -8(%rsp) was overwritten. The nop10 optimization
// should escape that by moving stackpointer below the redzone before
// doing the CALL.
//
// Align the code at 64 bytes, to make sure nop10 is not on page boundary.
//
    __attribute__((aligned(64)))
#[no_mangle]
pub unsafe extern "C" fn uprobe_red_zone_test() -> __nocf_check __weak __naked unsigned long {
    __nocf_check __weak __naked unsigned long uprobe_red_zone_test(void)
    {
    asm volatile (
    "movabs $0x1111111111111111, %%rax\n"
    "movq   %%rax, -8(%%rsp)\n"
    "movabs $0x2222222222222222, %%rax\n"
    "movq   %%rax, -16(%%rsp)\n"
    "movabs $0x3333333333333333, %%rax\n"
    "movq   %%rax, -24(%%rsp)\n"
    ".byte 0x66, 0x2e, 0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00\n" /* nop10: uprobe site */
    "movabs $0x1111111111111111, %%rax\n"
    "cmpq   %%rax, -8(%%rsp)\n"
    "jne    1f\n"
    "movabs $0x2222222222222222, %%rax\n"
    "cmpq   %%rax, -16(%%rsp)\n"
    "jne    1f\n"
    "movabs $0x3333333333333333, %%rax\n"
    "cmpq   %%rax, -24(%%rsp)\n"
    "jne    1f\n"
    "xorl   %%eax, %%eax\n"
    "retq\n"
    "1:\n"
    "movl   $1, %%eax\n"
    "retq\n"
    ::: "rax", "memory"
    );
    }
#[no_mangle]
unsafe extern "C" fn find_uprobes_trampoline(tramp_addr: *mut c_void) -> c_int {
    static int find_uprobes_trampoline(void *tramp_addr)
    {
    void *start, *end;
    char line[128];
    let mut ret: c_int = -1;
    FILE *maps;
    maps = fopen("/proc/self/maps", "r");
    if (!maps) {
    fprintf(stderr, "cannot open maps\n");
    return -1;
    }
    while (fgets(line, sizeof(line), maps)) {
    let mut m: c_int = -1;
// We care only about private r-x mappings.
    if (sscanf(line, "%p-%p r-xp %*x %*x:%*x %*u %n", &start, &end, &m) != 2)
    continue;
    if (m < 0)
    continue;
    if (!strncmp(&line[m], TRAMP, sizeof(TRAMP)-1) && (start == tramp_addr)) {
    ret = 0;
    break;
    }
    }
    fclose(maps);
    return ret;
    }
    static unsigned char nop10[10]  = { 0x66, 0x2e, 0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00 };
    static unsigned char lea_rsp[5] = { 0x48, 0x8d, 0x64, 0x24, 0x80 };
    static void *find_nop10(void *fn)
    {
    int i;
    for (i = 0; i < 128; i++) {
    if (!memcmp(nop10, fn + i, 10))
    return fn + i;
    }
    return core::ptr::null_mut();
    }
    typedef void (__attribute__((nocf_check)) *trigger_t)(void);
    static void check_attach_notrigger(struct uprobe_syscall_executed *skel,
    void *addr, int executed)
    {
    unsigned char *op = addr;
// Make sure bpf program was not executed.
    ASSERT_EQ(skel.bss.executed, executed, "executed");
    ASSERT_EQ(*op, 0xcc, "int3");
    }
    static void *check_attach(struct uprobe_syscall_executed *skel, trigger_t trigger,
    void *addr, int executed)
    {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __arch_relative_insn {
    pub op: __u8,
    pub raddr: __s32,
    pub call: *mut } __packed,
    pub NULL: *mut *mut void tramp =,
// Uprobe gets optimized after first trigger, so let's press twice.
// Make sure bpf program got executed..
    pub "executed"): ASSERT_EQ(skel->bss->executed, executed,,
// .. and check the trampoline is as expected.
    pub "lea_rsp"): ASSERT_OK(memcmp(addr, lea_rsp, 5),,
    pub 5): *mut *mut call = (struct __arch_relative_insn )(addr +,
    pub call->raddr: *mut *mut tramp = (void ) (call + 1) +,
    pub "call"): ASSERT_EQ(call->op, 0xe8,,
    pub "uprobes_trampoline"): ASSERT_OK(find_uprobes_trampoline(tramp),,
    pub tramp: return,
    }
#[no_mangle]
unsafe extern "C" fn check_detach(addr: *mut c_void, tramp: *mut c_void) -> bool {
    static bool check_detach(void *addr, void *tramp)
    {
    pub }: static unsigned char nop10_prefix[] = { 0x66, 0x2e, 0x0f, 0x1f, 0x84,
    pub true: bool ok =,
// [uprobes_trampoline] stays after detach
    pub "uprobes_trampoline"): ok &= ASSERT_OK(find_uprobes_trampoline(tramp),,
    pub "nop10_prefix"): ok &= ASSERT_OK(memcmp(addr, nop10_prefix, 5),,
    pub ok: return,
    }
    static void *check(struct uprobe_syscall_executed *skel, struct bpf_link *link,
    trigger_t trigger, void *addr, int executed)
    {
    pub tramp: *mut c_void,
    pub executed): tramp = check_attach(skel, trigger, addr,,
    pub tramp): check_detach(addr,,
    pub tramp: return,
    }
#[no_mangle]
unsafe extern "C" fn test_uprobe_legacy() {
    static void test_uprobe_legacy(void)
    {
    pub NULL: *mut *mut uprobe_syscall_executed skel =,
    LIBBPF_OPTS(bpf_uprobe_opts, opts,
    .retprobe = true,
    pub link: *mut bpf_link,
    pub offset: c_ulong,
    pub tramp: *mut c_void,
    pub get_uprobe_offset(&uprobe_test): offset =,
    if (!ASSERT_GE(offset, 0, "get_uprobe_offset"))
    pub cleanup: goto,
// uprobe
    pub uprobe_syscall_executed__open_and_load(): skel =,
    if (!ASSERT_OK_PTR(skel, "uprobe_syscall_executed__open_and_load"))
    pub getpid(): skel->bss->pid =,
    link = bpf_program__attach_uprobe_opts(skel.progs.test_uprobe,
    pub NULL): 0, "/proc/self/exe", offset,,
    if (!ASSERT_OK_PTR(link, "bpf_program__attach_uprobe_opts"))
    pub cleanup: goto,
    pub 2): tramp = check(skel, link, uprobe_test, uprobe_test,,
// reattach and detach without triggering optimization
    link = bpf_program__attach_uprobe_opts(skel.progs.test_uprobe,
    pub NULL): 0, "/proc/self/exe", offset,,
    if (!ASSERT_OK_PTR(link, "bpf_program__attach_uprobe_opts"))
    pub cleanup: goto,
    pub 2): check_attach_notrigger(skel, uprobe_test,,
    if (!check_detach(uprobe_test, tramp))
    pub cleanup: goto,
    pub "executed_no_probe"): ASSERT_EQ(skel->bss->executed, 2,,
// reattach with triggering optimization
    link = bpf_program__attach_uprobe_opts(skel.progs.test_uprobe,
    pub NULL): 0, "/proc/self/exe", offset,,
    if (!ASSERT_OK_PTR(link, "bpf_program__attach_uprobe_opts"))
    pub cleanup: goto,
    pub 4): check(skel, link, uprobe_test, uprobe_test,,
// uretprobe
    pub 0: skel->bss->executed =,
    link = bpf_program__attach_uprobe_opts(skel.progs.test_uretprobe,
    pub &opts): 0, "/proc/self/exe", offset,,
    if (!ASSERT_OK_PTR(link, "bpf_program__attach_uprobe_opts"))
    pub cleanup: goto,
    pub 2): check(skel, link, uprobe_test, uprobe_test,,
    cleanup:
    }
#[no_mangle]
unsafe extern "C" fn test_uprobe_multi() {
    static void test_uprobe_multi(void)
    {
    pub NULL: *mut *mut uprobe_syscall_executed skel =,
    pub opts): LIBBPF_OPTS(bpf_uprobe_multi_opts,,
    pub link: *mut bpf_link,
    pub offset: c_ulong,
    pub tramp: *mut c_void,
    pub get_uprobe_offset(&uprobe_test): offset =,
    if (!ASSERT_GE(offset, 0, "get_uprobe_offset"))
    pub cleanup: goto,
    pub &offset: opts.offsets =,
    pub 1: opts.cnt =,
    pub uprobe_syscall_executed__open_and_load(): skel =,
    if (!ASSERT_OK_PTR(skel, "uprobe_syscall_executed__open_and_load"))
    pub getpid(): skel->bss->pid =,
// uprobe.multi
    link = bpf_program__attach_uprobe_multi(skel.progs.test_uprobe_multi,
    pub &opts): 0, "/proc/self/exe", NULL,,
    if (!ASSERT_OK_PTR(link, "bpf_program__attach_uprobe_multi"))
    pub cleanup: goto,
    pub 2): tramp = check(skel, link, uprobe_test, uprobe_test,,
// reattach and detach without triggering optimization
    link = bpf_program__attach_uprobe_multi(skel.progs.test_uprobe_multi,
    pub &opts): 0, "/proc/self/exe", NULL,,
    if (!ASSERT_OK_PTR(link, "bpf_program__attach_uprobe_multi"))
    pub cleanup: goto,
    pub 2): check_attach_notrigger(skel, uprobe_test,,
    if (!check_detach(uprobe_test, tramp))
    pub cleanup: goto,
    pub "executed_no_probe"): ASSERT_EQ(skel->bss->executed, 2,,
// reattach with triggering optimization
    link = bpf_program__attach_uprobe_multi(skel.progs.test_uprobe_multi,
    pub &opts): 0, "/proc/self/exe", NULL,,
    if (!ASSERT_OK_PTR(link, "bpf_program__attach_uprobe_multi"))
    pub cleanup: goto,
    pub 4): check(skel, link, uprobe_test, uprobe_test,,
// uretprobe.multi
    pub 0: skel->bss->executed =,
    pub true: opts.retprobe =,
    link = bpf_program__attach_uprobe_multi(skel.progs.test_uretprobe_multi,
    pub &opts): 0, "/proc/self/exe", NULL,,
    if (!ASSERT_OK_PTR(link, "bpf_program__attach_uprobe_multi"))
    pub cleanup: goto,
    pub 2): check(skel, link, uprobe_test, uprobe_test,,
    cleanup:
    }
#[no_mangle]
unsafe extern "C" fn test_uprobe_session() {
    static void test_uprobe_session(void)
    {
    pub NULL: *mut *mut uprobe_syscall_executed skel =,
    LIBBPF_OPTS(bpf_uprobe_multi_opts, opts,
    .session = true,
    pub link: *mut bpf_link,
    pub offset: c_ulong,
    pub tramp: *mut c_void,
    pub get_uprobe_offset(&uprobe_test): offset =,
    if (!ASSERT_GE(offset, 0, "get_uprobe_offset"))
    pub cleanup: goto,
    pub &offset: opts.offsets =,
    pub 1: opts.cnt =,
    pub uprobe_syscall_executed__open_and_load(): skel =,
    if (!ASSERT_OK_PTR(skel, "uprobe_syscall_executed__open_and_load"))
    pub getpid(): skel->bss->pid =,
    link = bpf_program__attach_uprobe_multi(skel.progs.test_uprobe_session,
    pub &opts): 0, "/proc/self/exe", NULL,,
    if (!ASSERT_OK_PTR(link, "bpf_program__attach_uprobe_multi"))
    pub cleanup: goto,
    pub 4): tramp = check(skel, link, uprobe_test, uprobe_test,,
// reattach and detach without triggering optimization
    link = bpf_program__attach_uprobe_multi(skel.progs.test_uprobe_session,
    pub &opts): 0, "/proc/self/exe", NULL,,
    if (!ASSERT_OK_PTR(link, "bpf_program__attach_uprobe_multi"))
    pub cleanup: goto,
    pub 4): check_attach_notrigger(skel, uprobe_test,,
    if (!check_detach(uprobe_test, tramp))
    pub cleanup: goto,
    pub "executed_no_probe"): ASSERT_EQ(skel->bss->executed, 4,,
// reattach with triggering optimization
    link = bpf_program__attach_uprobe_multi(skel.progs.test_uprobe_session,
    pub &opts): 0, "/proc/self/exe", NULL,,
    if (!ASSERT_OK_PTR(link, "bpf_program__attach_uprobe_multi"))
    pub cleanup: goto,
    pub 8): check(skel, link, uprobe_test, uprobe_test,,
    cleanup:
    }
#[no_mangle]
unsafe extern "C" fn test_uprobe_usdt() {
    static void test_uprobe_usdt(void)
    {
    pub skel: *mut uprobe_syscall_executed,
    pub link: *mut bpf_link,
    pub tramp: *mut *mut void addr,,
    pub 0: errno =,
    pub find_nop10(usdt_test): addr =,
    if (!ASSERT_OK_PTR(addr, "find_nop10"))
    pub uprobe_syscall_executed__open_and_load(): skel =,
    if (!ASSERT_OK_PTR(skel, "uprobe_syscall_executed__open_and_load"))
    pub getpid(): skel->bss->pid =,
    link = bpf_program__attach_usdt(skel.progs.test_usdt,
    -1 /* all PIDs */, "/proc/self/exe",
    pub NULL): "optimized_uprobe", "usdt",,
    if (!ASSERT_OK_PTR(link, "bpf_program__attach_usdt"))
    pub cleanup: goto,
    pub 2): tramp = check(skel, link, usdt_test, addr,,
// reattach and detach without triggering optimization
    link = bpf_program__attach_usdt(skel.progs.test_usdt,
    -1 /* all PIDs */, "/proc/self/exe",
    pub NULL): "optimized_uprobe", "usdt",,
    if (!ASSERT_OK_PTR(link, "bpf_program__attach_usdt"))
    pub cleanup: goto,
    pub 2): check_attach_notrigger(skel, addr,,
    if (!check_detach(addr, tramp))
    pub cleanup: goto,
    pub "executed_no_probe"): ASSERT_EQ(skel->bss->executed, 2,,
// reattach with triggering optimization
    link = bpf_program__attach_usdt(skel.progs.test_usdt,
    -1 /* all PIDs */, "/proc/self/exe",
    pub NULL): "optimized_uprobe", "usdt",,
    if (!ASSERT_OK_PTR(link, "bpf_program__attach_usdt"))
    pub cleanup: goto,
    pub 4): check(skel, link, usdt_test, addr,,
    cleanup:
    }
//
// Borrowed from tools/testing/selftests/x86/test_shadow_stack.c.
//
// For use in inline enablement of shadow stack.
//
// The program can't return from the point where shadow stack gets enabled
// because there will be no address on the shadow stack. So it can't use
// syscall() for enablement, since it is a function.
//
// Based on code from nolibc.h. Keep a copy here because this can't pull
// in all of nolibc.h.
//

    ({								\
    pub \: long _ret;,
    pub \: register long _num asm("eax") = __NR_arch_prctl;,
    pub \: register long _arg1 asm("rdi") = (long)(arg1);,
    pub \: register long _arg2 asm("rsi") = (long)(arg2);,
    \
    asm volatile (						\
    "syscall\n"					\
    : "=a"(_ret)					\
    : "r"(_arg1), "r"(_arg2),			\
    "0"(_num)					\
    : "rcx", "r11", "memory", "cc"			\
    pub \: );,
    pub \: _ret;,
    })

pub const ARCH_SHSTK_ENABLE: c_uint = 0x5001;
pub const ARCH_SHSTK_DISABLE: c_uint = 0x5002;

#[no_mangle]
unsafe extern "C" fn test_uretprobe_shadow_stack() {
    static void test_uretprobe_shadow_stack(void)
    {
    if (ARCH_PRCTL(ARCH_SHSTK_ENABLE, ARCH_SHSTK_SHSTK)) {
    }
// Run all the tests with shadow stack in place.
    pub ARCH_SHSTK_SHSTK): ARCH_PRCTL(ARCH_SHSTK_DISABLE,,
    }
    pub race_stop: static volatile bool,
    pub USDT_DEFINE_SEMA(race): static,
    static void *worker_trigger(void *arg)
    {
    pub 0: unsigned long rounds =,
    while (!race_stop) {
    }
    pub rounds): printf("tid %ld trigger rounds: %lu\n", sys_gettid(),,
    pub NULL: return,
    }
    static void *worker_attach(void *arg)
    {
    pub opts): LIBBPF_OPTS(bpf_uprobe_opts,,
    pub skel: *mut uprobe_syscall_executed,
    pub offset: unsigned long rounds = 0,,
    const char *sema[2] = {
    __stringify(USDT_SEMA(race)),
    core::ptr::null_mut(),
}

    unsigned long *ref;
    int err;
    offset = get_uprobe_offset(&uprobe_test);
    if (!ASSERT_GE(offset, 0, "get_uprobe_offset"))
    return core::ptr::null_mut();
    err = elf_resolve_syms_offsets("/proc/self/exe", 1, (const char **) &sema, &ref, STT_OBJECT);
    if (!ASSERT_OK(err, "elf_resolve_syms_offsets_sema"))
    return core::ptr::null_mut();
    opts.ref_ctr_offset = *ref;
    skel = uprobe_syscall_executed__open_and_load();
    if (!ASSERT_OK_PTR(skel, "uprobe_syscall_executed__open_and_load"))
    return core::ptr::null_mut();
    skel.bss.pid = getpid();
    while (!race_stop) {
    skel.links.test_uprobe = bpf_program__attach_uprobe_opts(skel.progs.test_uprobe,
    0, "/proc/self/exe", offset, &opts);
    if (!ASSERT_OK_PTR(skel.links.test_uprobe, "bpf_program__attach_uprobe_opts"))
    break;
    bpf_link__destroy(skel.links.test_uprobe);
    skel.links.test_uprobe = core::ptr::null_mut();
    rounds++;
    }
    printf("tid %ld attach rounds: %lu hits: %d\n", sys_gettid(), rounds, skel.bss.executed);
    uprobe_syscall_executed__destroy(skel);
    free(ref);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn race_msec() -> useconds_t {
    static useconds_t race_msec(void)
    {
    char *env;
    env = getenv("BPF_SELFTESTS_UPROBE_SYSCALL_RACE_MSEC");
    if (env)
    return atoi(env);
// default duration is 500ms
    return 500;
    }
#[no_mangle]
unsafe extern "C" fn test_uprobe_race() {
    static void test_uprobe_race(void)
    {
    int err, i, nr_threads;
    pthread_t *threads;
    nr_threads = libbpf_num_possible_cpus();
    if (!ASSERT_GT(nr_threads, 0, "libbpf_num_possible_cpus"))
    return;
    nr_threads = max(2, nr_threads);
    threads = alloca(sizeof(*threads) * nr_threads);
    if (!ASSERT_OK_PTR(threads, "malloc"))
    return;
    for (i = 0; i < nr_threads; i++) {
    err = pthread_create(&threads[i], core::ptr::null_mut(), i % 2 ? worker_trigger : worker_attach,
    core::ptr::null_mut());
    if (!ASSERT_OK(err, "pthread_create"))
    goto cleanup;
    }
    usleep(race_msec() * 1000);
    cleanup:
    race_stop = true;
    for (nr_threads = i, i = 0; i < nr_threads; i++)
    pthread_join(threads[i], core::ptr::null_mut());
    ASSERT_FALSE(USDT_SEMA_IS_ACTIVE(race), "race_semaphore");
    }

pub const __NR_uprobe: c_int = 336;

#[no_mangle]
unsafe extern "C" fn test_uprobe_red_zone() {
    static void test_uprobe_red_zone(void)
    {
    struct uprobe_syscall_executed *skel;
    struct bpf_link *link;
    void *nop10_addr;
    size_t offset;
    int i;
    nop10_addr = find_nop10(uprobe_red_zone_test);
    if (!ASSERT_NEQ(nop10_addr, core::ptr::null_mut(), "find_nop10"))
    return;
    skel = uprobe_syscall_executed__open_and_load();
    if (!ASSERT_OK_PTR(skel, "open_and_load"))
    return;
    offset = get_uprobe_offset(nop10_addr);
    link = bpf_program__attach_uprobe_opts(skel.progs.test_uprobe,
    0, "/proc/self/exe", offset, core::ptr::null_mut());
    if (!ASSERT_OK_PTR(link, "attach_uprobe"))
    goto cleanup;
    for (i = 0; i < 10; i++)
    ASSERT_EQ(uprobe_red_zone_test(), 0, "red_zone_intact");
    bpf_link__destroy(link);
    cleanup:
    uprobe_syscall_executed__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_uprobe_error() {
    static void test_uprobe_error(void)
    {
    let mut err: c_long = syscall(__NR_uprobe);
    ASSERT_EQ(err, -1, "error");
    ASSERT_EQ(errno, EPROTO, "errno");
    }
    __attribute__((aligned(16)))
#[no_mangle]
pub unsafe extern "C" fn uprobe_fork_test() -> __nocf_check __weak __naked void {
    __nocf_check __weak __naked void uprobe_fork_test(void)
    {
    asm volatile (
    ".byte 0x66, 0x2e, 0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00\n" /* nop10 */
    "ret\n"
    );
    }
#[no_mangle]
unsafe extern "C" fn child_func(arg: *mut c_void) -> noreturn int {
    static noreturn int child_func(void *arg)
    {
    struct uprobe_syscall_executed *skel = arg;
// Make sure the child's probe is still there and optimized..
    if (memcmp(uprobe_fork_test, lea_rsp, sizeof(lea_rsp)))
    _exit(1);
    skel.bss.pid = getpid();
// .. and it executes properly.
    uprobe_fork_test();
    if (skel.bss.executed != 3)
    _exit(2);
    _exit(0);
    }
#[no_mangle]
unsafe extern "C" fn test_uprobe_fork_optimized(clone_vm: bool) {
    static void test_uprobe_fork_optimized(bool clone_vm)
    {
    struct uprobe_syscall_executed *skel = core::ptr::null_mut();
    unsigned long offset;
    int pid, status, err;
    char stack[65535];
    offset = get_uprobe_offset(&uprobe_fork_test);
    if (!ASSERT_GE(offset, 0, "get_uprobe_offset"))
    return;
    skel = uprobe_syscall_executed__open_and_load();
    if (!ASSERT_OK_PTR(skel, "open_and_load"))
    goto cleanup;
    skel.links.test_uprobe = bpf_program__attach_uprobe_opts(skel.progs.test_uprobe,
    -1, "/proc/self/exe", offset, core::ptr::null_mut());
    if (!ASSERT_OK_PTR(skel.links.test_uprobe, "attach_uprobe"))
    goto cleanup;
    skel.bss.pid = getpid();
// Trigger optimization of uprobe in uprobe_fork_test.
    uprobe_fork_test();
    uprobe_fork_test();
// Make sure it got optimied.
    if (!ASSERT_OK(memcmp(uprobe_fork_test, lea_rsp, sizeof(lea_rsp)), "optimized"))
    goto cleanup;
    if (clone_vm) {
    pid = clone(child_func, stack + sizeof(stack), CLONE_VM|SIGCHLD, skel);
    if (!ASSERT_GT(pid, 0, "clone"))
    goto cleanup;
    } else {
    pid = fork();
    if (!ASSERT_GE(pid, 0, "fork"))
    goto cleanup;
    if (pid == 0)
    child_func(skel);
    }
// Wait for the child and verify it exited properly with 0.
    err = waitpid(pid, &status, 0);
    if (ASSERT_EQ(err, pid, "waitpid")) {
    ASSERT_EQ(WIFEXITED(status), 1, "child_exited");
    ASSERT_EQ(WEXITSTATUS(status), 0, "child_exit_code");
    }
    cleanup:
    uprobe_syscall_executed__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn __test_uprobe_syscall() {
    static void __test_uprobe_syscall(void)
    {
    if (test__start_subtest("uretprobe_regs_equal"))
    test_uprobe_regs_equal(true);
    if (test__start_subtest("uretprobe_syscall_call"))
    test_uretprobe_syscall_call();
    if (test__start_subtest("uretprobe_shadow_stack"))
    test_uretprobe_shadow_stack();
    if (test__start_subtest("uprobe_legacy"))
    test_uprobe_legacy();
    if (test__start_subtest("uprobe_multi"))
    test_uprobe_multi();
    if (test__start_subtest("uprobe_session"))
    test_uprobe_session();
    if (test__start_subtest("uprobe_usdt"))
    test_uprobe_usdt();
    if (test__start_subtest("uprobe_race"))
    test_uprobe_race();
    if (test__start_subtest("uprobe_red_zone"))
    test_uprobe_red_zone();
    if (test__start_subtest("uprobe_optimized_fork"))
    test_uprobe_fork_optimized(false);
    if (test__start_subtest("uprobe_optimized_clone_vm"))
    test_uprobe_fork_optimized(true);
    if (test__start_subtest("uprobe_error"))
    test_uprobe_error();
    if (test__start_subtest("uprobe_regs_equal"))
    test_uprobe_regs_equal(false);
    if (test__start_subtest("regs_change"))
    test_regs_change();
    }

#[no_mangle]
unsafe extern "C" fn __test_uprobe_syscall() {
    static void __test_uprobe_syscall(void)
    {
    test__skip();
    }

#[no_mangle]
pub unsafe extern "C" fn test_uprobe_syscall() {
    void test_uprobe_syscall(void)
    {
    __test_uprobe_syscall();
    }
