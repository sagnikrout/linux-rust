//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/rseq/param_test.c
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


// SPDX-License-Identifier: LGPL-2.1
// Macro flag: #define _GNU_SOURCE

#[no_mangle]
pub unsafe extern "C" fn rseq_gettid() -> pid_t {
    static inline pid_t rseq_gettid(void)
    {
    return syscall(__NR_gettid);
    }
pub const NR_INJECT: c_int = 9;
    static int loop_cnt[NR_INJECT + 1];
    static int loop_cnt_1 asm("asm_loop_cnt_1") __attribute__((used));
    static int loop_cnt_2 asm("asm_loop_cnt_2") __attribute__((used));
    static int loop_cnt_3 asm("asm_loop_cnt_3") __attribute__((used));
    static int loop_cnt_4 asm("asm_loop_cnt_4") __attribute__((used));
    static int loop_cnt_5 asm("asm_loop_cnt_5") __attribute__((used));
    static int loop_cnt_6 asm("asm_loop_cnt_6") __attribute__((used));
    static int opt_modulo, verbose;
    static int opt_yield, opt_signal, opt_sleep,
    opt_disable_rseq, opt_threads = 200,
    opt_disable_mod = 0, opt_test = 's';
    static bool opt_rseq_legacy;
    let mut opt_reps: static long long = 5000;
#[no_mangle]
unsafe extern "C" fn __attribute__(_arg: (tls_model("initial-exec"))) -> __thread {
    static __thread __attribute__((tls_model("initial-exec")))
    unsigned int signals_delivered;

#[no_mangle]
unsafe extern "C" fn __attribute__(_arg: (tls_model("initial-exec"), _arg: unused)) -> __thread {
    static __thread __attribute__((tls_model("initial-exec"), unused))
    unsigned int yield_mod_cnt, nr_abort;

    do {						\
    if (verbose)				\
    printf(fmt, ## __VA_ARGS__);	\
    } while (0)

    , INJECT_ASM_REG

    "mov asm_loop_cnt_" #n ", %%" INJECT_ASM_REG "\n\t" \
    "test %%" INJECT_ASM_REG ",%%" INJECT_ASM_REG "\n\t" \
    "jz 333f\n\t" \
    "222:\n\t" \
    "dec %%" INJECT_ASM_REG "\n\t" \
    "jnz 222b\n\t" \
    "333:\n\t"

    , INJECT_ASM_REG_P \
    , INJECT_ASM_REG

    "lea asm_loop_cnt_" #n "(%%rip), %%" INJECT_ASM_REG_P "\n\t" \
    "mov (%%" INJECT_ASM_REG_P "), %%" INJECT_ASM_REG "\n\t" \
    "test %%" INJECT_ASM_REG ",%%" INJECT_ASM_REG "\n\t" \
    "jz 333f\n\t" \
    "222:\n\t" \
    "dec %%" INJECT_ASM_REG "\n\t" \
    "jnz 222b\n\t" \
    "333:\n\t"

    , [loop_cnt_1]"m"(loop_cnt[1]) \
    , [loop_cnt_2]"m"(loop_cnt[2]) \
    , [loop_cnt_3]"m"(loop_cnt[3]) \
    , [loop_cnt_4]"m"(loop_cnt[4]) \
    , [loop_cnt_5]"m"(loop_cnt[5]) \
    , [loop_cnt_6]"m"(loop_cnt[6])

    , INJECT_ASM_REG

    "l %%" INJECT_ASM_REG ", %[loop_cnt_" #n "]\n\t" \
    "ltr %%" INJECT_ASM_REG ", %%" INJECT_ASM_REG "\n\t" \
    "je 333f\n\t" \
    "222:\n\t" \
    "ahi %%" INJECT_ASM_REG ", -1\n\t" \
    "jnz 222b\n\t" \
    "333:\n\t"

    , [loop_cnt_1]"m"(loop_cnt[1]) \
    , [loop_cnt_2]"m"(loop_cnt[2]) \
    , [loop_cnt_3]"m"(loop_cnt[3]) \
    , [loop_cnt_4]"m"(loop_cnt[4]) \
    , [loop_cnt_5]"m"(loop_cnt[5]) \
    , [loop_cnt_6]"m"(loop_cnt[6])

    , INJECT_ASM_REG

    "ldr " INJECT_ASM_REG ", %[loop_cnt_" #n "]\n\t" \
    "cmp " INJECT_ASM_REG ", #0\n\t" \
    "beq 333f\n\t" \
    "222:\n\t" \
    "subs " INJECT_ASM_REG ", #1\n\t" \
    "bne 222b\n\t" \
    "333:\n\t"

    , [loop_cnt_1] "Qo" (loop_cnt[1]) \
    , [loop_cnt_2] "Qo" (loop_cnt[2]) \
    , [loop_cnt_3] "Qo" (loop_cnt[3]) \
    , [loop_cnt_4] "Qo" (loop_cnt[4]) \
    , [loop_cnt_5] "Qo" (loop_cnt[5]) \
    , [loop_cnt_6] "Qo" (loop_cnt[6])

    "	ldr	" INJECT_ASM_REG ", %[loop_cnt_" #n "]\n"	\
    "	cbz	" INJECT_ASM_REG ", 333f\n"			\
    "222:\n"							\
    "	sub	" INJECT_ASM_REG ", " INJECT_ASM_REG ", #1\n"	\
    "	cbnz	" INJECT_ASM_REG ", 222b\n"			\
    "333:\n"

    , [loop_cnt_1]"m"(loop_cnt[1]) \
    , [loop_cnt_2]"m"(loop_cnt[2]) \
    , [loop_cnt_3]"m"(loop_cnt[3]) \
    , [loop_cnt_4]"m"(loop_cnt[4]) \
    , [loop_cnt_5]"m"(loop_cnt[5]) \
    , [loop_cnt_6]"m"(loop_cnt[6])

    , INJECT_ASM_REG

    "lwz %%" INJECT_ASM_REG ", %[loop_cnt_" #n "]\n\t" \
    "cmpwi %%" INJECT_ASM_REG ", 0\n\t" \
    "beq 333f\n\t" \
    "222:\n\t" \
    "subic. %%" INJECT_ASM_REG ", %%" INJECT_ASM_REG ", 1\n\t" \
    "bne 222b\n\t" \
    "333:\n\t"

    , [loop_cnt_1]"m"(loop_cnt[1]) \
    , [loop_cnt_2]"m"(loop_cnt[2]) \
    , [loop_cnt_3]"m"(loop_cnt[3]) \
    , [loop_cnt_4]"m"(loop_cnt[4]) \
    , [loop_cnt_5]"m"(loop_cnt[5]) \
    , [loop_cnt_6]"m"(loop_cnt[6])

    , INJECT_ASM_REG

    "lw " INJECT_ASM_REG ", %[loop_cnt_" #n "]\n\t" \
    "beqz " INJECT_ASM_REG ", 333f\n\t" \
    "222:\n\t" \
    "addiu " INJECT_ASM_REG ", -1\n\t" \
    "bnez " INJECT_ASM_REG ", 222b\n\t" \
    "333:\n\t"

    , [loop_cnt_1]"m"(loop_cnt[1]) \
    , [loop_cnt_2]"m"(loop_cnt[2]) \
    , [loop_cnt_3]"m"(loop_cnt[3]) \
    , [loop_cnt_4]"m"(loop_cnt[4]) \
    , [loop_cnt_5]"m"(loop_cnt[5]) \
    , [loop_cnt_6]"m"(loop_cnt[6])

    , INJECT_ASM_REG

    "lw " INJECT_ASM_REG ", %[loop_cnt_" #n "]\n\t"		\
    "beqz " INJECT_ASM_REG ", 333f\n\t"			\
    "222:\n\t"						\
    "addi  " INJECT_ASM_REG "," INJECT_ASM_REG ", -1\n\t"	\
    "bnez " INJECT_ASM_REG ", 222b\n\t"			\
    "333:\n\t"

    , [loop_cnt_1]"m"(loop_cnt[1]) \
    , [loop_cnt_2]"m"(loop_cnt[2]) \
    , [loop_cnt_3]"m"(loop_cnt[3]) \
    , [loop_cnt_4]"m"(loop_cnt[4]) \
    , [loop_cnt_5]"m"(loop_cnt[5]) \
    , [loop_cnt_6]"m"(loop_cnt[6])

    , INJECT_ASM_REG

    "l.lwz   " INJECT_ASM_REG ", %[loop_cnt_" #n "]\n\t"	\
    "l.sfeqi " INJECT_ASM_REG ", 0\n\t"			\
    "l.bf 333f\n\t"						\
    " l.nop\n\t"						\
    "222:\n\t"						\
    "l.addi  " INJECT_ASM_REG "," INJECT_ASM_REG ", -1\n\t"	\
    "l.sfeqi " INJECT_ASM_REG ", 0\n\t"			\
    "l.bf 222f\n\t"						\
    " l.nop\n\t"						\
    "333:\n\t"

    nr_abort++;

    { \
    int loc_i, loc_nr_loops = loop_cnt[n]; \
    \
    for (loc_i = 0; loc_i < loc_nr_loops; loc_i++) { \
    rseq_barrier(); \
    } \
    if (loc_nr_loops == -1 && opt_modulo) { \
    if (yield_mod_cnt == opt_modulo - 1) { \
    if (opt_sleep > 0) \
    poll(core::ptr::null_mut(), 0, opt_sleep); \
    if (opt_yield) \
    sched_yield(); \
    if (opt_signal) \
    raise(SIGUSR1); \
    yield_mod_cnt = 0; \
    } else { \
    yield_mod_cnt++; \
    } \
    } \
    }

    let mut opt_mo: static enum rseq_mo = RSEQ_MO_RELAXED;

// Macro flag: #define TEST_MEMBARRIER
#[no_mangle]
unsafe extern "C" fn sys_membarrier(cmd: c_int, flags: c_int, cpu_id: c_int) -> c_int {
    static int sys_membarrier(int cmd, int flags, int cpu_id)
    {
    return syscall(__NR_membarrier, cmd, flags, cpu_id);
    }

    static
#[no_mangle]
pub unsafe extern "C" fn get_current_cpu_id() -> c_int {
    int get_current_cpu_id(void)
    {
    return rseq_current_mm_cid();
    }
    static
#[no_mangle]
pub unsafe extern "C" fn rseq_validate_cpu_id() -> bool {
    bool rseq_validate_cpu_id(void)
    {
    return rseq_mm_cid_available();
    }
    static
#[no_mangle]
pub unsafe extern "C" fn rseq_use_cpu_index() -> bool {
    bool rseq_use_cpu_index(void)
    {
    return false;	/* Use mm_cid */
    }

//
// Membarrier does not currently support targeting a mm_cid, so
// issue the barrier on all cpus.
//
    static
#[no_mangle]
pub unsafe extern "C" fn rseq_membarrier_expedited(cpu: c_int) -> c_int {
    int rseq_membarrier_expedited(int cpu)
    {
    return sys_membarrier(MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ,
    0, 0);
    }

    static
#[no_mangle]
pub unsafe extern "C" fn get_current_cpu_id() -> c_int {
    int get_current_cpu_id(void)
    {
    return rseq_cpu_start();
    }
    static
#[no_mangle]
pub unsafe extern "C" fn rseq_validate_cpu_id() -> bool {
    bool rseq_validate_cpu_id(void)
    {
    return rseq_current_cpu_raw() >= 0;
    }
    static
#[no_mangle]
pub unsafe extern "C" fn rseq_use_cpu_index() -> bool {
    bool rseq_use_cpu_index(void)
    {
    return true;	/* Use cpu_id as index. */
    }

    static
#[no_mangle]
pub unsafe extern "C" fn rseq_membarrier_expedited(cpu: c_int) -> c_int {
    int rseq_membarrier_expedited(int cpu)
    {
    return sys_membarrier(MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ,
    MEMBARRIER_CMD_FLAG_CPU, cpu);
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct percpu_lock_entry {
    pub v: intptr_t,
    pub __attribute__((aligned(128))): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct percpu_lock {
    pub c: [percpu_lock_entry; CPU_SETSIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_data_entry {
    pub count: intptr_t,
    pub __attribute__((aligned(128))): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spinlock_test_data {
    pub lock: percpu_lock,
    pub c: [test_data_entry; CPU_SETSIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spinlock_thread_test_data {
    pub data: *mut spinlock_test_data,
    pub reps: c_longlong,
    pub reg: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inc_test_data {
    pub c: [test_data_entry; CPU_SETSIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inc_thread_test_data {
    pub data: *mut inc_test_data,
    pub reps: c_longlong,
    pub reg: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct percpu_list_node {
    pub data: intptr_t,
    pub next: *mut percpu_list_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct percpu_list_entry {
    pub head: *mut percpu_list_node,
    pub __attribute__((aligned(128))): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct percpu_list {
    pub c: [percpu_list_entry; CPU_SETSIZE],
}

pub const BUFFER_ITEM_PER_CPU: c_int = 100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct percpu_buffer_node {
    pub data: intptr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct percpu_buffer_entry {
    pub offset: intptr_t,
    pub buflen: intptr_t,
    pub array: *mut percpu_buffer_node,
    pub __attribute__((aligned(128))): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct percpu_buffer {
    pub c: [percpu_buffer_entry; CPU_SETSIZE],
}

pub const MEMCPY_BUFFER_ITEM_PER_CPU: c_int = 100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct percpu_memcpy_buffer_node {
    pub data1: intptr_t,
    pub data2: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct percpu_memcpy_buffer_entry {
    pub offset: intptr_t,
    pub buflen: intptr_t,
    pub array: *mut percpu_memcpy_buffer_node,
    pub __attribute__((aligned(128))): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct percpu_memcpy_buffer {
    pub c: [percpu_memcpy_buffer_entry; CPU_SETSIZE],
}

// A simple percpu spinlock. Grabs lock on current cpu.
#[no_mangle]
unsafe extern "C" fn rseq_this_cpu_lock(lock: *mut percpu_lock) -> c_int {
    static int rseq_this_cpu_lock(struct percpu_lock *lock)
    {
    int cpu;
    for (;;) {
    int ret;
    cpu = get_current_cpu_id();
    if (cpu < 0) {
    fprintf(stderr, "pid: %d: tid: %d, cpu: %d: cid: %d\n",
    getpid(), (int) rseq_gettid(), rseq_current_cpu_raw(), cpu);
    abort();
    }
    ret = rseq_cmpeqv_storev(RSEQ_MO_RELAXED, RSEQ_PERCPU,
    &lock.c[cpu].v,
    0, 1, cpu);
    if (rseq_likely(!ret))
    break;
// Retry if comparison fails or rseq aborts.
    }
//
// Acquire semantic when taking lock after control dependency.
// Matches rseq_smp_store_release().
//
    rseq_smp_acquire__after_ctrl_dep();
    return cpu;
    }
#[no_mangle]
unsafe extern "C" fn rseq_percpu_unlock(lock: *mut percpu_lock, cpu: c_int) {
    static void rseq_percpu_unlock(struct percpu_lock *lock, int cpu)
    {
    assert(lock.c[cpu].v == 1);
//
// Release lock, with release semantic. Matches
// rseq_smp_acquire__after_ctrl_dep().
//
    rseq_smp_store_release(&lock.c[cpu].v, 0);
    }
    void *test_percpu_spinlock_thread(void *arg)
    {
    struct spinlock_thread_test_data *thread_data = arg;
    struct spinlock_test_data *data = thread_data.data;
    long long i, reps;
    if (!opt_disable_rseq && thread_data.reg &&
    __rseq_register_current_thread(rseq_no_glibc, opt_rseq_legacy))
    abort();
    reps = thread_data.reps;
    for (i = 0; i < reps; i++) {
    let mut cpu: c_int = rseq_this_cpu_lock(&data.lock);
    data.c[cpu].count++;
    rseq_percpu_unlock(&data.lock, cpu);

    if (i != 0 && !(i % (reps / 10)))
    printf_verbose("tid %d: count %lld\n",
    (int) rseq_gettid(), i);

    }
    printf_verbose("tid %d: number of rseq abort: %d, signals delivered: %u\n",
    (int) rseq_gettid(), nr_abort, signals_delivered);
    if (!opt_disable_rseq && thread_data.reg &&
    rseq_unregister_current_thread())
    abort();
    return core::ptr::null_mut();
    }
//
// A simple test which implements a sharded counter using a per-cpu
// lock.  Obviously real applications might prefer to simply use a
// per-cpu increment; however, this is reasonable for a test and the
// lock can be extended to synchronize more complicated operations.
//
#[no_mangle]
pub unsafe extern "C" fn test_percpu_spinlock() {
    void test_percpu_spinlock(void)
    {
    let mut num_threads: c_int = opt_threads;
    int i, ret;
    uint64_t sum;
    pthread_t test_threads[num_threads];
    struct spinlock_test_data data;
    struct spinlock_thread_test_data thread_data[num_threads];
    memset(&data, 0, sizeof(data));
    for (i = 0; i < num_threads; i++) {
    thread_data[i].reps = opt_reps;
    if (opt_disable_mod <= 0 || (i % opt_disable_mod))
    thread_data[i].reg = 1;
    else
    thread_data[i].reg = 0;
    thread_data[i].data = &data;
    ret = pthread_create(&test_threads[i], core::ptr::null_mut(),
    test_percpu_spinlock_thread,
    &thread_data[i]);
    if (ret) {
    errno = ret;
    perror("pthread_create");
    abort();
    }
    }
    for (i = 0; i < num_threads; i++) {
    ret = pthread_join(test_threads[i], core::ptr::null_mut());
    if (ret) {
    errno = ret;
    perror("pthread_join");
    abort();
    }
    }
    sum = 0;
    for (i = 0; i < CPU_SETSIZE; i++)
    sum += data.c[i].count;
    assert(sum == (uint64_t)opt_reps * num_threads);
    }
    void *test_percpu_inc_thread(void *arg)
    {
    struct inc_thread_test_data *thread_data = arg;
    struct inc_test_data *data = thread_data.data;
    long long i, reps;
    if (!opt_disable_rseq && thread_data.reg &&
    __rseq_register_current_thread(rseq_no_glibc, opt_rseq_legacy))
    abort();
    reps = thread_data.reps;
    for (i = 0; i < reps; i++) {
    int ret;
    do {
    int cpu;
    cpu = get_current_cpu_id();
    ret = rseq_addv(RSEQ_MO_RELAXED, RSEQ_PERCPU,
    &data.c[cpu].count, 1, cpu);
    } while (rseq_unlikely(ret));

    if (i != 0 && !(i % (reps / 10)))
    printf_verbose("tid %d: count %lld\n",
    (int) rseq_gettid(), i);

    }
    printf_verbose("tid %d: number of rseq abort: %d, signals delivered: %u\n",
    (int) rseq_gettid(), nr_abort, signals_delivered);
    if (!opt_disable_rseq && thread_data.reg &&
    rseq_unregister_current_thread())
    abort();
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn test_percpu_inc() {
    void test_percpu_inc(void)
    {
    let mut num_threads: c_int = opt_threads;
    int i, ret;
    uint64_t sum;
    pthread_t test_threads[num_threads];
    struct inc_test_data data;
    struct inc_thread_test_data thread_data[num_threads];
    memset(&data, 0, sizeof(data));
    for (i = 0; i < num_threads; i++) {
    thread_data[i].reps = opt_reps;
    if (opt_disable_mod <= 0 || (i % opt_disable_mod))
    thread_data[i].reg = 1;
    else
    thread_data[i].reg = 0;
    thread_data[i].data = &data;
    ret = pthread_create(&test_threads[i], core::ptr::null_mut(),
    test_percpu_inc_thread,
    &thread_data[i]);
    if (ret) {
    errno = ret;
    perror("pthread_create");
    abort();
    }
    }
    for (i = 0; i < num_threads; i++) {
    ret = pthread_join(test_threads[i], core::ptr::null_mut());
    if (ret) {
    errno = ret;
    perror("pthread_join");
    abort();
    }
    }
    sum = 0;
    for (i = 0; i < CPU_SETSIZE; i++)
    sum += data.c[i].count;
    assert(sum == (uint64_t)opt_reps * num_threads);
    }
    void this_cpu_list_push(struct percpu_list *list,
    struct percpu_list_node *node,
    int *_cpu)
    {
    int cpu;
    for (;;) {
    intptr_t *targetptr, newval, expect;
    int ret;
    cpu = get_current_cpu_id();
// Load list->c[cpu].head with single-copy atomicity.
    expect = (intptr_t)RSEQ_READ_ONCE(list.c[cpu].head);
    newval = (intptr_t)node;
    targetptr = (intptr_t *)&list.c[cpu].head;
    node.next = (struct percpu_list_node *)expect;
    ret = rseq_cmpeqv_storev(RSEQ_MO_RELAXED, RSEQ_PERCPU,
    targetptr, expect, newval, cpu);
    if (rseq_likely(!ret))
    break;
// Retry if comparison fails or rseq aborts.
    }
    if (_cpu)
// _cpu = cpu;
    }
//
// Unlike a traditional lock-less linked list; the availability of a
// rseq primitive allows us to implement pop without concerns over
// ABA-type races.
//
    struct percpu_list_node *this_cpu_list_pop(struct percpu_list *list,
    int *_cpu)
    {
    struct percpu_list_node *node = core::ptr::null_mut();
    int cpu;
    for (;;) {
    struct percpu_list_node *head;
    intptr_t *targetptr, expectnot, *load;
    long offset;
    int ret;
    cpu = get_current_cpu_id();
    targetptr = (intptr_t *)&list.c[cpu].head;
    expectnot = (intptr_t)core::ptr::null_mut();
    offset = offsetof(struct percpu_list_node, next);
    load = (intptr_t *)&head;
    ret = rseq_cmpnev_storeoffp_load(RSEQ_MO_RELAXED, RSEQ_PERCPU,
    targetptr, expectnot,
    offset, load, cpu);
    if (rseq_likely(!ret)) {
    node = head;
    break;
    }
    if (ret > 0)
    break;
// Retry if rseq aborts.
    }
    if (_cpu)
// _cpu = cpu;
    return node;
    }
//
// __percpu_list_pop is not safe against concurrent accesses. Should
// only be used on lists that are not concurrently modified.
//
    struct percpu_list_node *__percpu_list_pop(struct percpu_list *list, int cpu)
    {
    struct percpu_list_node *node;
    node = list.c[cpu].head;
    if (!node)
    return core::ptr::null_mut();
    list.c[cpu].head = node.next;
    return node;
    }
    void *test_percpu_list_thread(void *arg)
    {
    long long i, reps;
    struct percpu_list *list = (struct percpu_list *)arg;
    if (!opt_disable_rseq && __rseq_register_current_thread(rseq_no_glibc, opt_rseq_legacy))
    abort();
    reps = opt_reps;
    for (i = 0; i < reps; i++) {
    struct percpu_list_node *node;
    node = this_cpu_list_pop(list, core::ptr::null_mut());
    if (opt_yield)
    sched_yield();  /* encourage shuffling */
    if (node)
    this_cpu_list_push(list, node, core::ptr::null_mut());
    }
    printf_verbose("tid %d: number of rseq abort: %d, signals delivered: %u\n",
    (int) rseq_gettid(), nr_abort, signals_delivered);
    if (!opt_disable_rseq && rseq_unregister_current_thread())
    abort();
    return core::ptr::null_mut();
    }
// Simultaneous modification to a per-cpu linked list from many threads.
#[no_mangle]
pub unsafe extern "C" fn test_percpu_list() {
    void test_percpu_list(void)
    {
    let mut num_threads: c_int = opt_threads;
    int i, j, ret;
    let mut sum: u64 = 0, expected_sum = 0;
    struct percpu_list list;
    pthread_t test_threads[num_threads];
    cpu_set_t allowed_cpus;
    memset(&list, 0, sizeof(list));
// Generate list entries for every usable cpu.
    sched_getaffinity(0, sizeof(allowed_cpus), &allowed_cpus);
    for (i = 0; i < CPU_SETSIZE; i++) {
    if (rseq_use_cpu_index() && !CPU_ISSET(i, &allowed_cpus))
    continue;
    for (j = 1; j <= 100; j++) {
    struct percpu_list_node *node;
    expected_sum += j;
    node = malloc(sizeof(*node));
    assert(node);
    node.data = j;
    node.next = list.c[i].head;
    list.c[i].head = node;
    }
    }
    for (i = 0; i < num_threads; i++) {
    ret = pthread_create(&test_threads[i], core::ptr::null_mut(),
    test_percpu_list_thread, &list);
    if (ret) {
    errno = ret;
    perror("pthread_create");
    abort();
    }
    }
    for (i = 0; i < num_threads; i++) {
    ret = pthread_join(test_threads[i], core::ptr::null_mut());
    if (ret) {
    errno = ret;
    perror("pthread_join");
    abort();
    }
    }
    for (i = 0; i < CPU_SETSIZE; i++) {
    struct percpu_list_node *node;
    if (rseq_use_cpu_index() && !CPU_ISSET(i, &allowed_cpus))
    continue;
    while ((node = __percpu_list_pop(&list, i))) {
    sum += node.data;
    free(node);
    }
    }
//
// All entries should now be accounted for (unless some external
// actor is interfering with our allowed affinity while this
// test is running).
//
    assert(sum == expected_sum);
    }
    bool this_cpu_buffer_push(struct percpu_buffer *buffer,
    struct percpu_buffer_node *node,
    int *_cpu)
    {
    let mut result: bool = false;
    int cpu;
    for (;;) {
    intptr_t *targetptr_spec, newval_spec;
    intptr_t *targetptr_final, newval_final;
    intptr_t offset;
    int ret;
    cpu = get_current_cpu_id();
    offset = RSEQ_READ_ONCE(buffer.c[cpu].offset);
    if (offset == buffer.c[cpu].buflen)
    break;
    newval_spec = (intptr_t)node;
    targetptr_spec = (intptr_t *)&buffer.c[cpu].array[offset];
    newval_final = offset + 1;
    targetptr_final = &buffer.c[cpu].offset;
    ret = rseq_cmpeqv_trystorev_storev(opt_mo, RSEQ_PERCPU,
    targetptr_final, offset, targetptr_spec,
    newval_spec, newval_final, cpu);
    if (rseq_likely(!ret)) {
    result = true;
    break;
    }
// Retry if comparison fails or rseq aborts.
    }
    if (_cpu)
// _cpu = cpu;
    return result;
    }
    struct percpu_buffer_node *this_cpu_buffer_pop(struct percpu_buffer *buffer,
    int *_cpu)
    {
    struct percpu_buffer_node *head;
    int cpu;
    for (;;) {
    intptr_t *targetptr, newval;
    intptr_t offset;
    int ret;
    cpu = get_current_cpu_id();
// Load offset with single-copy atomicity.
    offset = RSEQ_READ_ONCE(buffer.c[cpu].offset);
    if (offset == 0) {
    head = core::ptr::null_mut();
    break;
    }
    head = RSEQ_READ_ONCE(buffer.c[cpu].array[offset - 1]);
    newval = offset - 1;
    targetptr = (intptr_t *)&buffer.c[cpu].offset;
    ret = rseq_cmpeqv_cmpeqv_storev(RSEQ_MO_RELAXED, RSEQ_PERCPU,
    targetptr, offset,
    (intptr_t *)&buffer.c[cpu].array[offset - 1],
    (intptr_t)head, newval, cpu);
    if (rseq_likely(!ret))
    break;
// Retry if comparison fails or rseq aborts.
    }
    if (_cpu)
// _cpu = cpu;
    return head;
    }
//
// __percpu_buffer_pop is not safe against concurrent accesses. Should
// only be used on buffers that are not concurrently modified.
//
    struct percpu_buffer_node *__percpu_buffer_pop(struct percpu_buffer *buffer,
    int cpu)
    {
    struct percpu_buffer_node *head;
    intptr_t offset;
    offset = buffer.c[cpu].offset;
    if (offset == 0)
    return core::ptr::null_mut();
    head = buffer.c[cpu].array[offset - 1];
    buffer.c[cpu].offset = offset - 1;
    return head;
    }
    void *test_percpu_buffer_thread(void *arg)
    {
    long long i, reps;
    struct percpu_buffer *buffer = (struct percpu_buffer *)arg;
    if (!opt_disable_rseq && __rseq_register_current_thread(rseq_no_glibc, opt_rseq_legacy))
    abort();
    reps = opt_reps;
    for (i = 0; i < reps; i++) {
    struct percpu_buffer_node *node;
    node = this_cpu_buffer_pop(buffer, core::ptr::null_mut());
    if (opt_yield)
    sched_yield();  /* encourage shuffling */
    if (node) {
    if (!this_cpu_buffer_push(buffer, node, core::ptr::null_mut())) {
// Should increase buffer size.
    abort();
    }
    }
    }
    printf_verbose("tid %d: number of rseq abort: %d, signals delivered: %u\n",
    (int) rseq_gettid(), nr_abort, signals_delivered);
    if (!opt_disable_rseq && rseq_unregister_current_thread())
    abort();
    return core::ptr::null_mut();
    }
// Simultaneous modification to a per-cpu buffer from many threads.
#[no_mangle]
pub unsafe extern "C" fn test_percpu_buffer() {
    void test_percpu_buffer(void)
    {
    let mut num_threads: c_int = opt_threads;
    int i, j, ret;
    let mut sum: u64 = 0, expected_sum = 0;
    struct percpu_buffer buffer;
    pthread_t test_threads[num_threads];
    cpu_set_t allowed_cpus;
    memset(&buffer, 0, sizeof(buffer));
// Generate list entries for every usable cpu.
    sched_getaffinity(0, sizeof(allowed_cpus), &allowed_cpus);
    for (i = 0; i < CPU_SETSIZE; i++) {
    if (rseq_use_cpu_index() && !CPU_ISSET(i, &allowed_cpus))
    continue;
// Worse-case is every item in same CPU.
    buffer.c[i].array =
    malloc(sizeof(*buffer.c[i].array) * CPU_SETSIZE *
    BUFFER_ITEM_PER_CPU);
    assert(buffer.c[i].array);
    buffer.c[i].buflen = CPU_SETSIZE * BUFFER_ITEM_PER_CPU;
    for (j = 1; j <= BUFFER_ITEM_PER_CPU; j++) {
    struct percpu_buffer_node *node;
    expected_sum += j;
//
// We could theoretically put the word-sized
// "data" directly in the buffer. However, we
// want to model objects that would not fit
// within a single word, so allocate an object
// for each node.
//
    node = malloc(sizeof(*node));
    assert(node);
    node.data = j;
    buffer.c[i].array[j - 1] = node;
    buffer.c[i].offset++;
    }
    }
    for (i = 0; i < num_threads; i++) {
    ret = pthread_create(&test_threads[i], core::ptr::null_mut(),
    test_percpu_buffer_thread, &buffer);
    if (ret) {
    errno = ret;
    perror("pthread_create");
    abort();
    }
    }
    for (i = 0; i < num_threads; i++) {
    ret = pthread_join(test_threads[i], core::ptr::null_mut());
    if (ret) {
    errno = ret;
    perror("pthread_join");
    abort();
    }
    }
    for (i = 0; i < CPU_SETSIZE; i++) {
    struct percpu_buffer_node *node;
    if (rseq_use_cpu_index() && !CPU_ISSET(i, &allowed_cpus))
    continue;
    while ((node = __percpu_buffer_pop(&buffer, i))) {
    sum += node.data;
    free(node);
    }
    free(buffer.c[i].array);
    }
//
// All entries should now be accounted for (unless some external
// actor is interfering with our allowed affinity while this
// test is running).
//
    assert(sum == expected_sum);
    }
    bool this_cpu_memcpy_buffer_push(struct percpu_memcpy_buffer *buffer,
    struct percpu_memcpy_buffer_node item,
    int *_cpu)
    {
    let mut result: bool = false;
    int cpu;
    for (;;) {
    intptr_t *targetptr_final, newval_final, offset;
    char *destptr, *srcptr;
    size_t copylen;
    int ret;
    cpu = get_current_cpu_id();
// Load offset with single-copy atomicity.
    offset = RSEQ_READ_ONCE(buffer.c[cpu].offset);
    if (offset == buffer.c[cpu].buflen)
    break;
    destptr = (char *)&buffer.c[cpu].array[offset];
    srcptr = (char *)&item;
// copylen must be <= 4kB.
    copylen = sizeof(item);
    newval_final = offset + 1;
    targetptr_final = &buffer.c[cpu].offset;
    ret = rseq_cmpeqv_trymemcpy_storev(
    opt_mo, RSEQ_PERCPU,
    targetptr_final, offset,
    destptr, srcptr, copylen,
    newval_final, cpu);
    if (rseq_likely(!ret)) {
    result = true;
    break;
    }
// Retry if comparison fails or rseq aborts.
    }
    if (_cpu)
// _cpu = cpu;
    return result;
    }
    bool this_cpu_memcpy_buffer_pop(struct percpu_memcpy_buffer *buffer,
    struct percpu_memcpy_buffer_node *item,
    int *_cpu)
    {
    let mut result: bool = false;
    int cpu;
    for (;;) {
    intptr_t *targetptr_final, newval_final, offset;
    char *destptr, *srcptr;
    size_t copylen;
    int ret;
    cpu = get_current_cpu_id();
// Load offset with single-copy atomicity.
    offset = RSEQ_READ_ONCE(buffer.c[cpu].offset);
    if (offset == 0)
    break;
    destptr = (char *)item;
    srcptr = (char *)&buffer.c[cpu].array[offset - 1];
// copylen must be <= 4kB.
    copylen = sizeof(*item);
    newval_final = offset - 1;
    targetptr_final = &buffer.c[cpu].offset;
    ret = rseq_cmpeqv_trymemcpy_storev(RSEQ_MO_RELAXED, RSEQ_PERCPU,
    targetptr_final, offset, destptr, srcptr, copylen,
    newval_final, cpu);
    if (rseq_likely(!ret)) {
    result = true;
    break;
    }
// Retry if comparison fails or rseq aborts.
    }
    if (_cpu)
// _cpu = cpu;
    return result;
    }
//
// __percpu_memcpy_buffer_pop is not safe against concurrent accesses. Should
// only be used on buffers that are not concurrently modified.
//
    bool __percpu_memcpy_buffer_pop(struct percpu_memcpy_buffer *buffer,
    struct percpu_memcpy_buffer_node *item,
    int cpu)
    {
    intptr_t offset;
    offset = buffer.c[cpu].offset;
    if (offset == 0)
    return false;
    memcpy(item, &buffer.c[cpu].array[offset - 1], sizeof(*item));
    buffer.c[cpu].offset = offset - 1;
    return true;
    }
    void *test_percpu_memcpy_buffer_thread(void *arg)
    {
    long long i, reps;
    struct percpu_memcpy_buffer *buffer = (struct percpu_memcpy_buffer *)arg;
    if (!opt_disable_rseq && __rseq_register_current_thread(rseq_no_glibc, opt_rseq_legacy))
    abort();
    reps = opt_reps;
    for (i = 0; i < reps; i++) {
    struct percpu_memcpy_buffer_node item;
    bool result;
    result = this_cpu_memcpy_buffer_pop(buffer, &item, core::ptr::null_mut());
    if (opt_yield)
    sched_yield();  /* encourage shuffling */
    if (result) {
    if (!this_cpu_memcpy_buffer_push(buffer, item, core::ptr::null_mut())) {
// Should increase buffer size.
    abort();
    }
    }
    }
    printf_verbose("tid %d: number of rseq abort: %d, signals delivered: %u\n",
    (int) rseq_gettid(), nr_abort, signals_delivered);
    if (!opt_disable_rseq && rseq_unregister_current_thread())
    abort();
    return core::ptr::null_mut();
    }
// Simultaneous modification to a per-cpu buffer from many threads.
#[no_mangle]
pub unsafe extern "C" fn test_percpu_memcpy_buffer() {
    void test_percpu_memcpy_buffer(void)
    {
    let mut num_threads: c_int = opt_threads;
    int i, j, ret;
    let mut sum: u64 = 0, expected_sum = 0;
    struct percpu_memcpy_buffer buffer;
    pthread_t test_threads[num_threads];
    cpu_set_t allowed_cpus;
    memset(&buffer, 0, sizeof(buffer));
// Generate list entries for every usable cpu.
    sched_getaffinity(0, sizeof(allowed_cpus), &allowed_cpus);
    for (i = 0; i < CPU_SETSIZE; i++) {
    if (rseq_use_cpu_index() && !CPU_ISSET(i, &allowed_cpus))
    continue;
// Worse-case is every item in same CPU.
    buffer.c[i].array =
    malloc(sizeof(*buffer.c[i].array) * CPU_SETSIZE *
    MEMCPY_BUFFER_ITEM_PER_CPU);
    assert(buffer.c[i].array);
    buffer.c[i].buflen = CPU_SETSIZE * MEMCPY_BUFFER_ITEM_PER_CPU;
    for (j = 1; j <= MEMCPY_BUFFER_ITEM_PER_CPU; j++) {
    expected_sum += 2 * j + 1;
//
// We could theoretically put the word-sized
// "data" directly in the buffer. However, we
// want to model objects that would not fit
// within a single word, so allocate an object
// for each node.
//
    buffer.c[i].array[j - 1].data1 = j;
    buffer.c[i].array[j - 1].data2 = j + 1;
    buffer.c[i].offset++;
    }
    }
    for (i = 0; i < num_threads; i++) {
    ret = pthread_create(&test_threads[i], core::ptr::null_mut(),
    test_percpu_memcpy_buffer_thread,
    &buffer);
    if (ret) {
    errno = ret;
    perror("pthread_create");
    abort();
    }
    }
    for (i = 0; i < num_threads; i++) {
    ret = pthread_join(test_threads[i], core::ptr::null_mut());
    if (ret) {
    errno = ret;
    perror("pthread_join");
    abort();
    }
    }
    for (i = 0; i < CPU_SETSIZE; i++) {
    struct percpu_memcpy_buffer_node item;
    if (rseq_use_cpu_index() && !CPU_ISSET(i, &allowed_cpus))
    continue;
    while (__percpu_memcpy_buffer_pop(&buffer, &item, i)) {
    sum += item.data1;
    sum += item.data2;
    }
    free(buffer.c[i].array);
    }
//
// All entries should now be accounted for (unless some external
// actor is interfering with our allowed affinity while this
// test is running).
//
    assert(sum == expected_sum);
    }
#[no_mangle]
unsafe extern "C" fn test_signal_interrupt_handler(signo: c_int) {
    static void test_signal_interrupt_handler(int signo)
    {
    signals_delivered++;
    }
#[no_mangle]
unsafe extern "C" fn set_signal_handler() -> c_int {
    static int set_signal_handler(void)
    {
    let mut ret: c_int = 0;
    struct sigaction sa;
    sigset_t sigset;
    ret = sigemptyset(&sigset);
    if (ret < 0) {
    perror("sigemptyset");
    return ret;
    }
    sa.sa_handler = test_signal_interrupt_handler;
    sa.sa_mask = sigset;
    sa.sa_flags = 0;
    ret = sigaction(SIGUSR1, &sa, core::ptr::null_mut());
    if (ret < 0) {
    perror("sigaction");
    return ret;
    }
    printf_verbose("Signal handler set for SIGUSR1\n");
    return ret;
    }
// Test MEMBARRIER_CMD_PRIVATE_RESTART_RSEQ_ON_CPU membarrier command.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_membarrier_thread_args {
    pub stop: c_int,
    pub percpu_list_ptr: intptr_t,
}

// Worker threads modify data in their "active" percpu lists.
    void *test_membarrier_worker_thread(void *arg)
    {
    struct test_membarrier_thread_args *args =
    (struct test_membarrier_thread_args *)arg;
    let mut iters: c_int = opt_reps;
    int i;
    if (__rseq_register_current_thread(rseq_no_glibc, opt_rseq_legacy)) {
    fprintf(stderr, "Error: rseq_register_current_thread(...) failed(%d): %s\n",
    errno, strerror(errno));
    abort();
    }
// Wait for initialization.
    while (!__atomic_load_n(&args.percpu_list_ptr, __ATOMIC_ACQUIRE)) {}
    for (i = 0; i < iters; ++i) {
    int ret;
    do {
    let mut cpu: c_int = get_current_cpu_id();
    ret = rseq_offset_deref_addv(RSEQ_MO_RELAXED, RSEQ_PERCPU,
    &args.percpu_list_ptr,
    sizeof(struct percpu_list_entry) * cpu, 1, cpu);
    } while (rseq_unlikely(ret));
    }
    if (rseq_unregister_current_thread()) {
    fprintf(stderr, "Error: rseq_unregister_current_thread(...) failed(%d): %s\n",
    errno, strerror(errno));
    abort();
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn test_membarrier_init_percpu_list(list: *mut percpu_list) {
    void test_membarrier_init_percpu_list(struct percpu_list *list)
    {
    int i;
    memset(list, 0, sizeof(*list));
    for (i = 0; i < CPU_SETSIZE; i++) {
    struct percpu_list_node *node;
    node = malloc(sizeof(*node));
    assert(node);
    node.data = 0;
    node.next = core::ptr::null_mut();
    list.c[i].head = node;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn test_membarrier_free_percpu_list(list: *mut percpu_list) {
    void test_membarrier_free_percpu_list(struct percpu_list *list)
    {
    int i;
    for (i = 0; i < CPU_SETSIZE; i++)
    free(list.c[i].head);
    }
//
// The manager thread swaps per-cpu lists that worker threads see,
// and validates that there are no unexpected modifications.
//
    void *test_membarrier_manager_thread(void *arg)
    {
    struct test_membarrier_thread_args *args =
    (struct test_membarrier_thread_args *)arg;
    struct percpu_list list_a, list_b;
    let mut expect_a: intptr_t = 0, expect_b = 0;
    let mut cpu_a: c_int = 0, cpu_b = 0;
    if (__rseq_register_current_thread(rseq_no_glibc, opt_rseq_legacy)) {
    fprintf(stderr, "Error: rseq_register_current_thread(...) failed(%d): %s\n",
    errno, strerror(errno));
    abort();
    }
// Init lists.
    test_membarrier_init_percpu_list(&list_a);
    test_membarrier_init_percpu_list(&list_b);
    __atomic_store_n(&args.percpu_list_ptr, (intptr_t)&list_a, __ATOMIC_RELEASE);
    while (!__atomic_load_n(&args.stop, __ATOMIC_ACQUIRE)) {
// list_a is "active".
    cpu_a = rand() % CPU_SETSIZE;
//
// As list_b is "inactive", we should never see changes
// to list_b.
//
    if (expect_b != __atomic_load_n(&list_b.c[cpu_b].head.data, __ATOMIC_ACQUIRE)) {
    fprintf(stderr, "Membarrier test failed\n");
    abort();
    }
// Make list_b "active".
    __atomic_store_n(&args.percpu_list_ptr, (intptr_t)&list_b, __ATOMIC_RELEASE);
    if (rseq_membarrier_expedited(cpu_a) &&
    errno != ENXIO /* missing CPU */) {
    perror("sys_membarrier");
    abort();
    }
//
// Cpu A should now only modify list_b, so the values
// in list_a should be stable.
//
    expect_a = __atomic_load_n(&list_a.c[cpu_a].head.data, __ATOMIC_ACQUIRE);
    cpu_b = rand() % CPU_SETSIZE;
//
// As list_a is "inactive", we should never see changes
// to list_a.
//
    if (expect_a != __atomic_load_n(&list_a.c[cpu_a].head.data, __ATOMIC_ACQUIRE)) {
    fprintf(stderr, "Membarrier test failed\n");
    abort();
    }
// Make list_a "active".
    __atomic_store_n(&args.percpu_list_ptr, (intptr_t)&list_a, __ATOMIC_RELEASE);
    if (rseq_membarrier_expedited(cpu_b) &&
    errno != ENXIO /* missing CPU*/) {
    perror("sys_membarrier");
    abort();
    }
// Remember a value from list_b.
    expect_b = __atomic_load_n(&list_b.c[cpu_b].head.data, __ATOMIC_ACQUIRE);
    }
    test_membarrier_free_percpu_list(&list_a);
    test_membarrier_free_percpu_list(&list_b);
    if (rseq_unregister_current_thread()) {
    fprintf(stderr, "Error: rseq_unregister_current_thread(...) failed(%d): %s\n",
    errno, strerror(errno));
    abort();
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn test_membarrier() {
    void test_membarrier(void)
    {
    let mut num_threads: c_int = opt_threads;
    struct test_membarrier_thread_args thread_args;
    pthread_t worker_threads[num_threads];
    pthread_t manager_thread;
    int i, ret;
    if (sys_membarrier(MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_RSEQ, 0, 0)) {
    perror("sys_membarrier");
    abort();
    }
    thread_args.stop = 0;
    thread_args.percpu_list_ptr = 0;
    ret = pthread_create(&manager_thread, core::ptr::null_mut(),
    test_membarrier_manager_thread, &thread_args);
    if (ret) {
    errno = ret;
    perror("pthread_create");
    abort();
    }
    for (i = 0; i < num_threads; i++) {
    ret = pthread_create(&worker_threads[i], core::ptr::null_mut(),
    test_membarrier_worker_thread, &thread_args);
    if (ret) {
    errno = ret;
    perror("pthread_create");
    abort();
    }
    }
    for (i = 0; i < num_threads; i++) {
    ret = pthread_join(worker_threads[i], core::ptr::null_mut());
    if (ret) {
    errno = ret;
    perror("pthread_join");
    abort();
    }
    }
    __atomic_store_n(&thread_args.stop, 1, __ATOMIC_RELEASE);
    ret = pthread_join(manager_thread, core::ptr::null_mut());
    if (ret) {
    errno = ret;
    perror("pthread_join");
    abort();
    }
    }

#[no_mangle]
pub unsafe extern "C" fn test_membarrier() {
    void test_membarrier(void)
    {
    fprintf(stderr, "rseq_offset_deref_addv is not implemented on this architecture. "
    "Skipping membarrier test.\n");
    }

#[no_mangle]
unsafe extern "C" fn show_usage(argc: c_int, argv: *mut c_char) {
    static void show_usage(int argc, char **argv)
    {
    printf("Usage : %s <OPTIONS>\n",
    argv[0]);
    printf("OPTIONS:\n");
    printf("	[-1 loops] Number of loops for delay injection 1\n");
    printf("	[-2 loops] Number of loops for delay injection 2\n");
    printf("	[-3 loops] Number of loops for delay injection 3\n");
    printf("	[-4 loops] Number of loops for delay injection 4\n");
    printf("	[-5 loops] Number of loops for delay injection 5\n");
    printf("	[-6 loops] Number of loops for delay injection 6\n");
    printf("	[-7 loops] Number of loops for delay injection 7 (-1 to enable -m)\n");
    printf("	[-8 loops] Number of loops for delay injection 8 (-1 to enable -m)\n");
    printf("	[-9 loops] Number of loops for delay injection 9 (-1 to enable -m)\n");
    printf("	[-m N] Yield/sleep/kill every modulo N (default 0: disabled) (>= 0)\n");
    printf("	[-y] Yield\n");
    printf("	[-k] Kill thread with signal\n");
    printf("	[-s S] S: =0: disabled (default), >0: sleep time (ms)\n");
    printf("	[-t N] Number of threads (default 200)\n");
    printf("	[-r N] Number of repetitions per thread (default 5000)\n");
    printf("	[-d] Disable rseq system call (no initialization)\n");
    printf("	[-D M] Disable rseq for each M threads\n");
    printf("	[-T test] Choose test: (s)pinlock, (l)ist, (b)uffer, (m)emcpy, (i)ncrement, membarrie(r)\n");
    printf("	[-M] Push into buffer and memcpy buffer with memory barriers.\n");
    printf("	[-O] Test with optimized RSEQ\n");
    printf("	[-v] Verbose output.\n");
    printf("	[-h] Show this help.\n");
    printf("\n");
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    int i;
    for (i = 1; i < argc; i++) {
    if (argv[i][0] != '-')
    continue;
    switch (argv[i][1]) {
    case '1':
    case '2':
    case '3':
    case '4':
    case '5':
    case '6':
    case '7':
    case '8':
    case '9':
    if (argc < i + 2) {
    show_usage(argc, argv);
    goto error;
    }
    loop_cnt[argv[i][1] - '0'] = atol(argv[i + 1]);
    i++;
    break;
    case 'm':
    if (argc < i + 2) {
    show_usage(argc, argv);
    goto error;
    }
    opt_modulo = atol(argv[i + 1]);
    if (opt_modulo < 0) {
    show_usage(argc, argv);
    goto error;
    }
    i++;
    break;
    case 's':
    if (argc < i + 2) {
    show_usage(argc, argv);
    goto error;
    }
    opt_sleep = atol(argv[i + 1]);
    if (opt_sleep < 0) {
    show_usage(argc, argv);
    goto error;
    }
    i++;
    break;
    case 'y':
    opt_yield = 1;
    break;
    case 'k':
    opt_signal = 1;
    break;
    case 'd':
    opt_disable_rseq = 1;
    break;
    case 'D':
    if (argc < i + 2) {
    show_usage(argc, argv);
    goto error;
    }
    opt_disable_mod = atol(argv[i + 1]);
    if (opt_disable_mod < 0) {
    show_usage(argc, argv);
    goto error;
    }
    i++;
    break;
    case 't':
    if (argc < i + 2) {
    show_usage(argc, argv);
    goto error;
    }
    opt_threads = atol(argv[i + 1]);
    if (opt_threads < 0) {
    show_usage(argc, argv);
    goto error;
    }
    i++;
    break;
    case 'r':
    if (argc < i + 2) {
    show_usage(argc, argv);
    goto error;
    }
    opt_reps = atoll(argv[i + 1]);
    if (opt_reps < 0) {
    show_usage(argc, argv);
    goto error;
    }
    i++;
    break;
    case 'h':
    show_usage(argc, argv);
    goto end;
    case 'T':
    if (argc < i + 2) {
    show_usage(argc, argv);
    goto error;
    }
    opt_test = *argv[i + 1];
    switch (opt_test) {
    case 's':
    case 'l':
    case 'i':
    case 'b':
    case 'm':
    case 'r':
    break;
    default:
    show_usage(argc, argv);
    goto error;
    }
    i++;
    break;
    case 'v':
    verbose = 1;
    break;
    case 'M':
    opt_mo = RSEQ_MO_RELEASE;
    break;
    case 'L':
    opt_rseq_legacy = true;
    break;
    default:
    show_usage(argc, argv);
    goto error;
    }
    }
    loop_cnt_1 = loop_cnt[1];
    loop_cnt_2 = loop_cnt[2];
    loop_cnt_3 = loop_cnt[3];
    loop_cnt_4 = loop_cnt[4];
    loop_cnt_5 = loop_cnt[5];
    loop_cnt_6 = loop_cnt[6];
    if (set_signal_handler())
    goto error;
    if (!opt_disable_rseq && __rseq_register_current_thread(rseq_no_glibc, opt_rseq_legacy))
    goto error;
    if (!opt_disable_rseq && !rseq_validate_cpu_id()) {
    fprintf(stderr, "Error: cpu id getter unavailable\n");
    goto error;
    }
    switch (opt_test) {
    case 's':
    printf_verbose("spinlock\n");
    test_percpu_spinlock();
    break;
    case 'l':
    printf_verbose("linked list\n");
    test_percpu_list();
    break;
    case 'b':
    printf_verbose("buffer\n");
    test_percpu_buffer();
    break;
    case 'm':
    printf_verbose("memcpy buffer\n");
    test_percpu_memcpy_buffer();
    break;
    case 'i':
    printf_verbose("counter increment\n");
    test_percpu_inc();
    break;
    case 'r':
    printf_verbose("membarrier\n");
    test_membarrier();
    break;
    }
    if (!opt_disable_rseq && rseq_unregister_current_thread())
    abort();
    end:
    return 0;
    error:
    return -1;
    }
