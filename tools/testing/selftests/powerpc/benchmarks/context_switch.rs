//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/benchmarks/context_switch.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Context switch microbenchmark.
//
// Copyright (C) 2015 Anton Blanchard <anton@au.ibm.com>, IBM
//
// Macro flag: #define _GNU_SOURCE

    let mut timeout: static unsigned int = 30;
    static int touch_vdso;
    struct timeval tv;
    let mut touch_fp: static int = 1;
    double fp;
    let mut touch_vector: static int = 1;
    vector int a, b, c;

    let mut touch_altivec: static int = 1;
//
// Note: LTO (Link Time Optimisation) doesn't play well with this function
// attribute. Be very careful enabling LTO for this test.
//
#[no_mangle]
unsafe extern "C" fn __attribute__(altivec_touch_fn(void: (__target__("no-vsx")))) {
    static void __attribute__((__target__("no-vsx"))) altivec_touch_fn(void)
    {
    c = a + b;
    }

#[no_mangle]
unsafe extern "C" fn touch() {
    static void touch(void)
    {
    if (touch_vdso)
    gettimeofday(&tv, core::ptr::null_mut());
    if (touch_fp)
    fp += 0.1;

    if (touch_altivec)
    altivec_touch_fn();

    if (touch_vector)
    c = a + b;
    asm volatile("# %0 %1 %2": : "r"(&tv), "r"(&fp), "r"(&c));
    }
#[no_mangle]
unsafe extern "C" fn start_thread_on(): *mut *mut *mut void (fn)(void, arg: *mut c_void, cpu: c_ulong) {
    static void start_thread_on(void *(*fn)(void *), void *arg, unsigned long cpu)
    {
    int rc;
    pthread_t tid;
    cpu_set_t cpuset;
    pthread_attr_t attr;
    CPU_ZERO(&cpuset);
    CPU_SET(cpu, &cpuset);
    rc = pthread_attr_init(&attr);
    if (rc) {
    errno = rc;
    perror("pthread_attr_init");
    exit(1);
    }
    rc = pthread_attr_setaffinity_np(&attr, sizeof(cpu_set_t), &cpuset);
    if (rc)	{
    errno = rc;
    perror("pthread_attr_setaffinity_np");
    exit(1);
    }
    rc = pthread_create(&tid, &attr, fn, arg);
    if (rc) {
    errno = rc;
    perror("pthread_create");
    exit(1);
    }
    }
#[no_mangle]
unsafe extern "C" fn start_process_on(): *mut *mut *mut void (fn)(void, arg: *mut c_void, cpu: c_ulong) {
    static void start_process_on(void *(*fn)(void *), void *arg, unsigned long cpu)
    {
    int pid, ncpus;
    cpu_set_t *cpuset;
    size_t size;
    pid = fork();
    if (pid == -1) {
    perror("fork");
    exit(1);
    }
    if (pid)
    return;
    ncpus = get_nprocs();
    size = CPU_ALLOC_SIZE(ncpus);
    cpuset = CPU_ALLOC(ncpus);
    if (!cpuset) {
    perror("malloc");
    exit(1);
    }
    CPU_ZERO_S(size, cpuset);
    CPU_SET_S(cpu, size, cpuset);
    if (sched_setaffinity(0, size, cpuset)) {
    perror("sched_setaffinity");
    CPU_FREE(cpuset);
    exit(1);
    }
    CPU_FREE(cpuset);
    fn(arg);
    exit(0);
    }
    static unsigned long iterations;
    static unsigned long iterations_prev;
#[no_mangle]
unsafe extern "C" fn sigalrm_handler(junk: c_int) {
    static void sigalrm_handler(int junk)
    {
    let mut i: c_ulong = iterations;
    printf("%ld\n", i - iterations_prev);
    iterations_prev = i;
    if (--timeout == 0)
    kill(0, SIGUSR1);
    alarm(1);
    }
#[no_mangle]
unsafe extern "C" fn sigusr1_handler(junk: c_int) {
    static void sigusr1_handler(int junk)
    {
    exit(0);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct actions {
    pub int): *mut *mut void (setup)(int,,
    pub ): *mut *mut *mut void (thread1)(void,
    pub ): *mut *mut *mut void (thread2)(void,
}

pub const READ: c_int = 0;
pub const WRITE: c_int = 1;
    static int pipe_fd1[2];
    static int pipe_fd2[2];
#[no_mangle]
unsafe extern "C" fn pipe_setup(cpu1: c_int, cpu2: c_int) {
    static void pipe_setup(int cpu1, int cpu2)
    {
    if (pipe(pipe_fd1) || pipe(pipe_fd2))
    exit(1);
    }
    static void *pipe_thread1(void *arg)
    {
    signal(SIGALRM, sigalrm_handler);
    alarm(1);
    while (1) {
    assert(read(pipe_fd1[READ], &c, 1) == 1);
    touch();
    assert(write(pipe_fd2[WRITE], &c, 1) == 1);
    touch();
    iterations += 2;
    }
    return core::ptr::null_mut();
    }
    static void *pipe_thread2(void *arg)
    {
    while (1) {
    assert(write(pipe_fd1[WRITE], &c, 1) == 1);
    touch();
    assert(read(pipe_fd2[READ], &c, 1) == 1);
    touch();
    }
    return core::ptr::null_mut();
    }
    static struct actions pipe_actions = {
    .setup = pipe_setup,
    .thread1 = pipe_thread1,
    .thread2 = pipe_thread2,
    };
#[no_mangle]
unsafe extern "C" fn yield_setup(cpu1: c_int, cpu2: c_int) {
    static void yield_setup(int cpu1, int cpu2)
    {
    if (cpu1 != cpu2) {
    fprintf(stderr, "Both threads must be on the same CPU for yield test\n");
    exit(1);
    }
    }
    static void *yield_thread1(void *arg)
    {
    signal(SIGALRM, sigalrm_handler);
    alarm(1);
    while (1) {
    sched_yield();
    touch();
    iterations += 2;
    }
    return core::ptr::null_mut();
    }
    static void *yield_thread2(void *arg)
    {
    while (1) {
    sched_yield();
    touch();
    }
    return core::ptr::null_mut();
    }
    static struct actions yield_actions = {
    .setup = yield_setup,
    .thread1 = yield_thread1,
    .thread2 = yield_thread2,
    };
    static long sys_futex(void *addr1, int op, int val1, struct timespec *timeout,
    void *addr2, int val3)
    {
    return syscall(SYS_futex, addr1, op, val1, timeout, addr2, val3);
    }
    static unsigned long cmpxchg(unsigned long *p, unsigned long expected,
    unsigned long desired)
    {
    let mut exp: c_ulong = expected;
    __atomic_compare_exchange_n(p, &exp, desired, 0,
    __ATOMIC_SEQ_CST, __ATOMIC_SEQ_CST);
    return exp;
    }
#[no_mangle]
unsafe extern "C" fn xchg(p: *mut c_ulong, val: c_ulong) -> c_ulong {
    static unsigned long xchg(unsigned long *p, unsigned long val)
    {
    return __atomic_exchange_n(p, val, __ATOMIC_SEQ_CST);
    }
    static int processes;
#[no_mangle]
unsafe extern "C" fn mutex_lock(m: *mut c_ulong) -> c_int {
    static int mutex_lock(unsigned long *m)
    {
    int c;
    let mut flags: c_int = FUTEX_WAIT;
    if (!processes)
    flags |= FUTEX_PRIVATE_FLAG;
    c = cmpxchg(m, 0, 1);
    if (!c)
    return 0;
    if (c == 1)
    c = xchg(m, 2);
    while (c) {
    sys_futex(m, flags, 2, core::ptr::null_mut(), core::ptr::null_mut(), 0);
    c = xchg(m, 2);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mutex_unlock(m: *mut c_ulong) -> c_int {
    static int mutex_unlock(unsigned long *m)
    {
    let mut flags: c_int = FUTEX_WAKE;
    if (!processes)
    flags |= FUTEX_PRIVATE_FLAG;
    if (*m == 2)
// m = 0;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: xchg(m, 1: 0) ==) -> else {
    else if (xchg(m, 0) == 1)
    return 0;
    sys_futex(m, flags, 1, core::ptr::null_mut(), core::ptr::null_mut(), 0);
    return 0;
    }
    static unsigned long *m1, *m2;
#[no_mangle]
unsafe extern "C" fn futex_setup(cpu1: c_int, cpu2: c_int) {
    static void futex_setup(int cpu1, int cpu2)
    {
    if (!processes) {
    static unsigned long _m1, _m2;
    m1 = &_m1;
    m2 = &_m2;
    } else {
    int shmid;
    void *shmaddr;
    shmid = shmget(IPC_PRIVATE, getpagesize(), SHM_R | SHM_W);
    if (shmid < 0) {
    perror("shmget");
    exit(1);
    }
    shmaddr = shmat(shmid, core::ptr::null_mut(), 0);
    if (shmaddr == (char *)-1) {
    perror("shmat");
    shmctl(shmid, IPC_RMID, core::ptr::null_mut());
    exit(1);
    }
    shmctl(shmid, IPC_RMID, core::ptr::null_mut());
    m1 = shmaddr;
    m2 = shmaddr + sizeof(*m1);
    }
// m1 = 0;
// m2 = 0;
    mutex_lock(m1);
    mutex_lock(m2);
    }
    static void *futex_thread1(void *arg)
    {
    signal(SIGALRM, sigalrm_handler);
    alarm(1);
    while (1) {
    mutex_lock(m2);
    mutex_unlock(m1);
    iterations += 2;
    }
    return core::ptr::null_mut();
    }
    static void *futex_thread2(void *arg)
    {
    while (1) {
    mutex_unlock(m2);
    mutex_lock(m1);
    }
    return core::ptr::null_mut();
    }
    static struct actions futex_actions = {
    .setup = futex_setup,
    .thread1 = futex_thread1,
    .thread2 = futex_thread2,
    };
    static struct option options[] = {
    { "test", required_argument, 0, 't' },
    { "process", no_argument, &processes, 1 },
    { "timeout", required_argument, 0, 's' },
    { "vdso", no_argument, &touch_vdso, 1 },
    { "no-fp", no_argument, &touch_fp, 0 },

    { "no-altivec", no_argument, &touch_altivec, 0 },

    { "no-vector", no_argument, &touch_vector, 0 },
    { 0, },
    };
#[no_mangle]
unsafe extern "C" fn usage() {
    static void usage(void)
    {
    fprintf(stderr, "Usage: context_switch2 <options> CPU1 CPU2\n\n");
    fprintf(stderr, "\t\t--test=X\tpipe, futex or yield (default)\n");
    fprintf(stderr, "\t\t--process\tUse processes (default threads)\n");
    fprintf(stderr, "\t\t--timeout=X\tDuration in seconds to run (default 30)\n");
    fprintf(stderr, "\t\t--vdso\t\ttouch VDSO\n");
    fprintf(stderr, "\t\t--no-fp\t\tDon't touch FP\n");

    fprintf(stderr, "\t\t--no-altivec\tDon't touch altivec\n");

    fprintf(stderr, "\t\t--no-vector\tDon't touch vector\n");
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    signed char c;
    struct actions *actions = &yield_actions;
    int cpu1;
    int cpu2;
    static void (*start_fn)(void *(*fn)(void *), void *arg, unsigned long cpu);
    while (1) {
    let mut option_index: c_int = 0;
    c = getopt_long(argc, argv, "", options, &option_index);
    if (c == -1)
    break;
    switch (c) {
    case 0:
    if (options[option_index].flag != 0)
    break;
    usage();
    exit(1);
    break;
    case 't':
    if (!strcmp(optarg, "pipe")) {
    actions = &pipe_actions;
    } else if (!strcmp(optarg, "yield")) {
    actions = &yield_actions;
    } else if (!strcmp(optarg, "futex")) {
    actions = &futex_actions;
    } else {
    usage();
    exit(1);
    }
    break;
    case 's':
    timeout = atoi(optarg);
    break;
    default:
    usage();
    exit(1);
    }
    }
    if (processes)
    start_fn = start_process_on;
    else
    start_fn = start_thread_on;
    if (((argc - optind) != 2)) {
    cpu1 = cpu2 = pick_online_cpu();
    } else {
    cpu1 = atoi(argv[optind++]);
    cpu2 = atoi(argv[optind++]);
    }
    printf("Using %s with ", processes ? "processes" : "threads");
    if (actions == &pipe_actions)
    printf("pipe");
#[no_mangle]
pub unsafe extern "C" fn if(&yield_actions: actions ==) -> else {
    else if (actions == &yield_actions)
    printf("yield");
    else
    printf("futex");
    if (!have_hwcap(PPC_FEATURE_HAS_ALTIVEC))
    touch_altivec = 0;
    if (!have_hwcap(PPC_FEATURE_HAS_VSX))
    touch_vector = 0;
    printf(" on cpus %d/%d touching FP:%s altivec:%s vector:%s vdso:%s\n",
    cpu1, cpu2, touch_fp ?  "yes" : "no", touch_altivec ? "yes" : "no",
    touch_vector ? "yes" : "no", touch_vdso ? "yes" : "no");
// Create a new process group so we can signal everyone for exit
    setpgid(getpid(), getpid());
    signal(SIGUSR1, sigusr1_handler);
    actions.setup(cpu1, cpu2);
    start_fn(actions.thread1, core::ptr::null_mut(), cpu1);
    start_fn(actions.thread2, core::ptr::null_mut(), cpu2);
    while (1)
    sleep(3600);
    return 0;
    }
