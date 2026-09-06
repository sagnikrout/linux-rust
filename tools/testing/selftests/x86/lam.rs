//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/x86/lam.c
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

// LAM modes, these definitions were copied from kernel code
pub const LAM_NONE: c_int = 0;
pub const LAM_U57_BITS: c_int = 6;

// arch prctl for LAM
pub const ARCH_GET_UNTAG_MASK: c_uint = 0x4001;
pub const ARCH_ENABLE_TAGGED_ADDR: c_uint = 0x4002;
pub const ARCH_GET_MAX_TAG_BITS: c_uint = 0x4003;
pub const ARCH_FORCE_TAGGED_SVA: c_uint = 0x4004;
// Specified test function bits
pub const FUNC_MALLOC: c_uint = 0x1;
pub const FUNC_BITS: c_uint = 0x2;
pub const FUNC_MMAP: c_uint = 0x4;
pub const FUNC_SYSCALL: c_uint = 0x8;
pub const FUNC_URING: c_uint = 0x10;
pub const FUNC_INHERITE: c_uint = 0x20;
pub const FUNC_PASID: c_uint = 0x40;
// get_user() pointer test cases
pub const GET_USER_USER: c_int = 0;
pub const GET_USER_KERNEL_TOP: c_int = 1;
pub const GET_USER_KERNEL_BOT: c_int = 2;
pub const GET_USER_KERNEL: c_int = 3;
pub const TEST_MASK: c_uint = 0x7f;

pub const MALLOC_LEN: c_int = 32;

pub const STACK_SIZE: c_int = 65536;

    __asm__ __volatile__("" : : : "memory");	\
    })
pub const URING_QUEUE_SZ: c_int = 1;
pub const URING_BLOCK_SZ: c_int = 2048;
// Pasid test define
pub const LAM_CMD_BIT: c_uint = 0x1;
pub const PAS_CMD_BIT: c_uint = 0x2;
pub const SVA_CMD_BIT: c_uint = 0x4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct testcases {
    pub later: c_uint,
    pub /: *mut *mut int expected; / 2: SIGSEGV Error; 1: other errors,
    pub lam: c_ulong,
    pub addr: u64,
    pub cmd: u64,
    pub test): *mut *mut int (test_func)(struct testcases,
    pub msg: *const c_char,
}

// Used by CQ of uring, source file handler and file's size
#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_io {
    pub file_fd: c_int,
    pub file_sz: off_t,
    pub iovecs: [iovec; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_queue {
    pub head: *mut c_uint,
    pub tail: *mut c_uint,
    pub ring_mask: *mut c_uint,
    pub ring_entries: *mut c_uint,
    pub flags: *mut c_uint,
    pub array: *mut c_uint,
    union {
    pub cqes: *mut io_uring_cqe,
    pub sqes: *mut io_uring_sqe,
    pub queue: },
    pub ring_sz: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_ring {
    pub ring_fd: c_int,
    pub sq_ring: io_uring_queue,
    pub cq_ring: io_uring_queue,
}

    int tests_cnt;
    jmp_buf segv_env;
#[no_mangle]
unsafe extern "C" fn segv_handler(sig: c_int) {
    static void segv_handler(int sig)
    {
    ksft_print_msg("Get segmentation fault(%d).", sig);
    siglongjmp(segv_env, 1);
    }
#[no_mangle]
pub unsafe extern "C" fn lam_is_available() -> c_int {
    static inline int lam_is_available(void)
    {
    unsigned int cpuinfo[4];
    let mut bits: c_ulong = 0;
    int ret;
    __cpuid_count(0x7, 1, cpuinfo[0], cpuinfo[1], cpuinfo[2], cpuinfo[3]);
// Check if cpu supports LAM
    if (!(cpuinfo[0] & (1 << 26))) {
    ksft_print_msg("LAM is not supported!\n");
    return 0;
    }
// Return 0 if CONFIG_ADDRESS_MASKING is not set
    ret = syscall(SYS_arch_prctl, ARCH_GET_MAX_TAG_BITS, &bits);
    if (ret) {
    ksft_print_msg("LAM is disabled in the kernel!\n");
    return 0;
    }
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn la57_enabled() -> c_int {
    static inline int la57_enabled(void)
    {
    int ret;
    void *p;
    p = mmap((void *)HIGH_ADDR, PAGE_SIZE, PROT_READ | PROT_WRITE,
    MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED, -1, 0);
    ret = p == MAP_FAILED ? 0 : 1;
    munmap(p, PAGE_SIZE);
    return ret;
    }
//
// Set tagged address and read back untag mask.
// check if the untagged mask is expected.
//
// @return:
// 0: Set LAM mode successfully
// others: failed to set LAM
//
#[no_mangle]
unsafe extern "C" fn set_lam(lam: c_ulong) -> c_int {
    static int set_lam(unsigned long lam)
    {
    let mut ret: c_int = 0;
    let mut ptr: u64 = 0;
    if (lam != LAM_U57_BITS && lam != LAM_NONE)
    return -1;
// Skip check return
    syscall(SYS_arch_prctl, ARCH_ENABLE_TAGGED_ADDR, lam);
// Get untagged mask
    syscall(SYS_arch_prctl, ARCH_GET_UNTAG_MASK, &ptr);
// Check mask returned is expected
    if (lam == LAM_U57_BITS)
    ret = (ptr != ~(LAM_U57_MASK));
#[no_mangle]
pub unsafe extern "C" fn if(LAM_NONE: lam ==) -> else {
    else if (lam == LAM_NONE)
    ret = (ptr != -1ULL);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn get_default_tag_bits() -> c_ulong {
    static unsigned long get_default_tag_bits(void)
    {
    pid_t pid;
    let mut lam: c_int = LAM_NONE;
    let mut ret: c_int = 0;
    pid = fork();
    if (pid < 0) {
    perror("Fork failed.");
    } else if (pid == 0) {
// Set LAM mode in child process
    if (set_lam(LAM_U57_BITS) == 0)
    lam = LAM_U57_BITS;
    else
    lam = LAM_NONE;
    exit(lam);
    } else {
    wait(&ret);
    lam = WEXITSTATUS(ret);
    }
    return lam;
    }
//
// Set tagged address and read back untag mask.
// check if the untag mask is expected.
//
#[no_mangle]
unsafe extern "C" fn get_lam() -> c_int {
    static int get_lam(void)
    {
    let mut ptr: u64 = 0;
    let mut ret: c_int = -1;
// Get untagged mask
    if (syscall(SYS_arch_prctl, ARCH_GET_UNTAG_MASK, &ptr) == -1)
    return -1;
// Check mask returned is expected
    if (ptr == ~(LAM_U57_MASK))
    ret = LAM_U57_BITS;
#[no_mangle]
pub unsafe extern "C" fn if(-1ULL: ptr ==) -> else {
    else if (ptr == -1ULL)
    ret = LAM_NONE;
    return ret;
    }
// According to LAM mode, set metadata in high bits
#[no_mangle]
unsafe extern "C" fn set_metadata(src: u64, lam: c_ulong) -> u64 {
    static uint64_t set_metadata(uint64_t src, unsigned long lam)
    {
    uint64_t metadata;
    srand(time(core::ptr::null_mut()));
    switch (lam) {
    case LAM_U57_BITS: /* Set metadata in bits 62:57 */
// Get a random non-zero value as metadata
    metadata = (rand() % ((1UL << LAM_U57_BITS) - 1) + 1) << 57;
    metadata |= (src & ~(LAM_U57_MASK));
    break;
    default:
    metadata = src;
    break;
    }
    return metadata;
    }
//
// Set metadata in user pointer, compare new pointer with original pointer.
// both pointers should point to the same address.
//
// @return:
// 0: value on the pointer with metadata and value on original are same
// 1: not same.
//
#[no_mangle]
unsafe extern "C" fn handle_lam_test(src: *mut c_void, lam: c_uint) -> c_int {
    static int handle_lam_test(void *src, unsigned int lam)
    {
    char *ptr;
    strcpy((char *)src, "USER POINTER");
    ptr = (char *)set_metadata((uint64_t)src, lam);
    if (src == ptr)
    return 0;
// Copy a string into the pointer with metadata
    strcpy((char *)ptr, "METADATA POINTER");
    return (!!strcmp((char *)src, (char *)ptr));
    }
#[no_mangle]
pub unsafe extern "C" fn handle_max_bits(test: *mut testcases) -> c_int {
    int handle_max_bits(struct testcases *test)
    {
    let mut exp_bits: c_ulong = get_default_tag_bits();
    let mut bits: c_ulong = 0;
    if (exp_bits != LAM_NONE)
    exp_bits = LAM_U57_BITS;
// Get LAM max tag bits
    if (syscall(SYS_arch_prctl, ARCH_GET_MAX_TAG_BITS, &bits) == -1)
    return 1;
    return (exp_bits != bits);
    }
//
// Test lam feature through dereference pointer get from malloc.
// @return 0: Pass test. 1: Get failure during test 2: Get SIGSEGV
//
#[no_mangle]
unsafe extern "C" fn handle_malloc(test: *mut testcases) -> c_int {
    static int handle_malloc(struct testcases *test)
    {
    char *ptr = core::ptr::null_mut();
    let mut ret: c_int = 0;
    if (test.later == 0 && test.lam != 0)
    if (set_lam(test.lam) == -1)
    return 1;
    ptr = (char *)malloc(MALLOC_LEN);
    if (ptr == core::ptr::null_mut()) {
    perror("malloc() failure\n");
    return 1;
    }
// Set signal handler
    if (sigsetjmp(segv_env, 1) == 0) {
    signal(SIGSEGV, segv_handler);
    ret = handle_lam_test(ptr, test.lam);
    } else {
    ret = 2;
    }
    if (test.later != 0 && test.lam != 0)
    if (set_lam(test.lam) == -1 && ret == 0)
    ret = 1;
    free(ptr);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn handle_mmap(test: *mut testcases) -> c_int {
    static int handle_mmap(struct testcases *test)
    {
    void *ptr;
    let mut flags: c_uint = MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED;
    let mut ret: c_int = 0;
    if (test.later == 0 && test.lam != 0)
    if (set_lam(test.lam) != 0)
    return 1;
    ptr = mmap((void *)test.addr, PAGE_SIZE, PROT_READ | PROT_WRITE,
    flags, -1, 0);
    if (ptr == MAP_FAILED) {
    if (test.addr == HIGH_ADDR)
    if (!la57_enabled())
    return 3; /* unsupport LA57 */
    return 1;
    }
    if (test.later != 0 && test.lam != 0)
    if (set_lam(test.lam) != 0)
    ret = 1;
    if (ret == 0) {
    if (sigsetjmp(segv_env, 1) == 0) {
    signal(SIGSEGV, segv_handler);
    ret = handle_lam_test(ptr, test.lam);
    } else {
    ret = 2;
    }
    }
    munmap(ptr, PAGE_SIZE);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn handle_syscall(test: *mut testcases) -> c_int {
    static int handle_syscall(struct testcases *test)
    {
    struct utsname unme, *pu;
    let mut ret: c_int = 0;
    if (test.later == 0 && test.lam != 0)
    if (set_lam(test.lam) != 0)
    return 1;
    if (sigsetjmp(segv_env, 1) == 0) {
    signal(SIGSEGV, segv_handler);
    pu = (struct utsname *)set_metadata((uint64_t)&unme, test.lam);
    ret = uname(pu);
    if (ret < 0)
    ret = 1;
    } else {
    ret = 2;
    }
    if (test.later != 0 && test.lam != 0)
    if (set_lam(test.lam) != -1 && ret == 0)
    ret = 1;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn get_user_syscall(test: *mut testcases) -> c_int {
    static int get_user_syscall(struct testcases *test)
    {
    uint64_t ptr_address, bitmask;
    int fd, ret = 0;
    void *ptr;
    if (la57_enabled()) {
    bitmask = L5_SIGN_EXT_MASK;
    ptr_address = HIGH_ADDR;
    } else {
    bitmask = L4_SIGN_EXT_MASK;
    ptr_address = LOW_ADDR;
    }
    ptr = mmap((void *)ptr_address, PAGE_SIZE, PROT_READ | PROT_WRITE,
    MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED, -1, 0);
    if (ptr == MAP_FAILED) {
    perror("failed to map byte to pass into get_user");
    return 1;
    }
    if (set_lam(test.lam) != 0) {
    ret = 2;
    goto error;
    }
    fd = memfd_create("lam_ioctl", 0);
    if (fd == -1) {
    munmap(ptr, PAGE_SIZE);
    exit(EXIT_FAILURE);
    }
    switch (test.later) {
    case GET_USER_USER:
// Control group - properly tagged user pointer
    ptr = (void *)set_metadata((uint64_t)ptr, test.lam);
    break;
    case GET_USER_KERNEL_TOP:
// Kernel address with top bit cleared
    bitmask &= (bitmask >> 1);
    ptr = (void *)((uint64_t)ptr | bitmask);
    break;
    case GET_USER_KERNEL_BOT:
// Kernel address with bottom sign-extension bit cleared
    bitmask &= (bitmask << 1);
    ptr = (void *)((uint64_t)ptr | bitmask);
    break;
    case GET_USER_KERNEL:
// Try to pass a kernel address
    ptr = (void *)((uint64_t)ptr | bitmask);
    break;
    default:
    printf("Invalid test case value passed!\n");
    break;
    }
//
// Use FIOASYNC ioctl because it utilizes get_user() internally and is
// very non-invasive to the system. Pass differently tagged pointers to
// get_user() in order to verify that valid user pointers are going
// through and invalid kernel/non-canonical pointers are not.
//
    if (ioctl(fd, FIOASYNC, ptr) != 0)
    ret = 1;
    close(fd);
    error:
    munmap(ptr, PAGE_SIZE);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_uring_setup(entries: c_uint, p: *mut io_uring_params) -> c_int {
    int sys_uring_setup(unsigned int entries, struct io_uring_params *p)
    {
    return (int)syscall(__NR_io_uring_setup, entries, p);
    }
#[no_mangle]
pub unsafe extern "C" fn sys_uring_enter(fd: c_int, to: c_uint, min: c_uint, flags: c_uint) -> c_int {
    int sys_uring_enter(int fd, unsigned int to, unsigned int min, unsigned int flags)
    {
    return (int)syscall(__NR_io_uring_enter, fd, to, min, flags, core::ptr::null_mut(), 0);
    }
// Init submission queue and completion queue
#[no_mangle]
pub unsafe extern "C" fn mmap_io_uring(p: io_uring_params, s: *mut io_ring) -> c_int {
    int mmap_io_uring(struct io_uring_params p, struct io_ring *s)
    {
    struct io_uring_queue *sring = &s.sq_ring;
    struct io_uring_queue *cring = &s.cq_ring;
    sring.ring_sz = p.sq_off.array + p.sq_entries * sizeof(unsigned int);
    cring.ring_sz = p.cq_off.cqes + p.cq_entries * sizeof(struct io_uring_cqe);
    if (p.features & IORING_FEAT_SINGLE_MMAP) {
    if (cring.ring_sz > sring.ring_sz)
    sring.ring_sz = cring.ring_sz;
    cring.ring_sz = sring.ring_sz;
    }
    void *sq_ptr = mmap(0, sring.ring_sz, PROT_READ | PROT_WRITE,
    MAP_SHARED | MAP_POPULATE, s.ring_fd,
    IORING_OFF_SQ_RING);
    if (sq_ptr == MAP_FAILED) {
    perror("sub-queue!");
    return 1;
    }
    void *cq_ptr = sq_ptr;
    if (!(p.features & IORING_FEAT_SINGLE_MMAP)) {
    cq_ptr = mmap(0, cring.ring_sz, PROT_READ | PROT_WRITE,
    MAP_SHARED | MAP_POPULATE, s.ring_fd,
    IORING_OFF_CQ_RING);
    if (cq_ptr == MAP_FAILED) {
    perror("cpl-queue!");
    munmap(sq_ptr, sring.ring_sz);
    return 1;
    }
    }
    sring.head = sq_ptr + p.sq_off.head;
    sring.tail = sq_ptr + p.sq_off.tail;
    sring.ring_mask = sq_ptr + p.sq_off.ring_mask;
    sring.ring_entries = sq_ptr + p.sq_off.ring_entries;
    sring.flags = sq_ptr + p.sq_off.flags;
    sring.array = sq_ptr + p.sq_off.array;
// Map a queue as mem map
    s.sq_ring.queue.sqes = mmap(0, p.sq_entries * sizeof(struct io_uring_sqe),
    PROT_READ | PROT_WRITE, MAP_SHARED | MAP_POPULATE,
    s.ring_fd, IORING_OFF_SQES);
    if (s.sq_ring.queue.sqes == MAP_FAILED) {
    munmap(sq_ptr, sring.ring_sz);
    if (sq_ptr != cq_ptr) {
    ksft_print_msg("failed to mmap uring queue!");
    munmap(cq_ptr, cring.ring_sz);
    return 1;
    }
    }
    cring.head = cq_ptr + p.cq_off.head;
    cring.tail = cq_ptr + p.cq_off.tail;
    cring.ring_mask = cq_ptr + p.cq_off.ring_mask;
    cring.ring_entries = cq_ptr + p.cq_off.ring_entries;
    cring.queue.cqes = cq_ptr + p.cq_off.cqes;
    return 0;
    }
// Init io_uring queues
#[no_mangle]
pub unsafe extern "C" fn setup_io_uring(s: *mut io_ring) -> c_int {
    int setup_io_uring(struct io_ring *s)
    {
    struct io_uring_params para;
    memset(&para, 0, sizeof(para));
    s.ring_fd = sys_uring_setup(URING_QUEUE_SZ, &para);
    if (s.ring_fd < 0)
    return 1;
    return mmap_io_uring(para, s);
    }
//
// Get data from completion queue. the data buffer saved the file data
// return 0: success; others: error;
//
#[no_mangle]
pub unsafe extern "C" fn handle_uring_cq(s: *mut io_ring) -> c_int {
    int handle_uring_cq(struct io_ring *s)
    {
    struct file_io *fi = core::ptr::null_mut();
    struct io_uring_queue *cring = &s.cq_ring;
    struct io_uring_cqe *cqe;
    unsigned int head;
    let mut len: off_t = 0;
    head = *cring.head;
    do {
    barrier();
    if (head == *cring.tail)
    break;
// Get the entry
    cqe = &cring.queue.cqes[head & *s.cq_ring.ring_mask];
    fi = (struct file_io *)cqe.user_data;
    if (cqe.res < 0)
    break;
    let mut blocks: c_int = (int)(fi.file_sz + URING_BLOCK_SZ - 1) / URING_BLOCK_SZ;
    for (int i = 0; i < blocks; i++)
    len += fi.iovecs[i].iov_len;
    head++;
    } while (1);
// cring->head = head;
    barrier();
    return (len != fi.file_sz);
    }
//
// Submit squeue. specify via IORING_OP_READV.
// the buffer need to be set metadata according to LAM mode
//
#[no_mangle]
pub unsafe extern "C" fn handle_uring_sq(ring: *mut io_ring, fi: *mut file_io, lam: c_ulong) -> c_int {
    int handle_uring_sq(struct io_ring *ring, struct file_io *fi, unsigned long lam)
    {
    let mut file_fd: c_int = fi.file_fd;
    struct io_uring_queue *sring = &ring.sq_ring;
    let mut index: c_uint = 0, cur_block = 0, tail = 0, next_tail = 0;
    struct io_uring_sqe *sqe;
    let mut remain: off_t = fi.file_sz;
    let mut blocks: c_int = (int)(remain + URING_BLOCK_SZ - 1) / URING_BLOCK_SZ;
    while (remain) {
    let mut bytes: off_t = remain;
    void *buf;
    if (bytes > URING_BLOCK_SZ)
    bytes = URING_BLOCK_SZ;
    fi.iovecs[cur_block].iov_len = bytes;
    if (posix_memalign(&buf, URING_BLOCK_SZ, URING_BLOCK_SZ))
    return 1;
    fi.iovecs[cur_block].iov_base = (void *)set_metadata((uint64_t)buf, lam);
    remain -= bytes;
    cur_block++;
    }
    next_tail = *sring.tail;
    tail = next_tail;
    next_tail++;
    barrier();
    index = tail & *ring.sq_ring.ring_mask;
    sqe = &ring.sq_ring.queue.sqes[index];
    sqe.fd = file_fd;
    sqe.flags = 0;
    sqe.opcode = IORING_OP_READV;
    sqe.addr = (unsigned long)fi.iovecs;
    sqe.len = blocks;
    sqe.off = 0;
    sqe.user_data = (uint64_t)fi;
    sring.array[index] = index;
    tail = next_tail;
    if (*sring.tail != tail) {
// sring->tail = tail;
    barrier();
    }
    if (sys_uring_enter(ring.ring_fd, 1, 1, IORING_ENTER_GETEVENTS) < 0)
    return 1;
    return 0;
    }
//
// Test LAM in async I/O and io_uring, read current binery through io_uring
// Set metadata in pointers to iovecs buffer.
//
#[no_mangle]
pub unsafe extern "C" fn do_uring(lam: c_ulong) -> c_int {
    int do_uring(unsigned long lam)
    {
    struct io_ring *ring;
    struct file_io *fi;
    struct stat st;
    let mut ret: c_int = 1;
    char path[PATH_MAX] = {0};
// get current process path
    if (readlink("/proc/self/exe", path, PATH_MAX - 1) <= 0)
    return 1;
    let mut file_fd: c_int = open(path, O_RDONLY);
    if (file_fd < 0)
    return 1;
    if (fstat(file_fd, &st) < 0)
    goto cleanup;
    let mut file_sz: off_t = st.st_size;
    let mut blocks: c_int = (int)(file_sz + URING_BLOCK_SZ - 1) / URING_BLOCK_SZ;
    fi = malloc(sizeof(*fi) + sizeof(struct iovec) * blocks);
    if (!fi)
    goto cleanup;
    fi.file_sz = file_sz;
    fi.file_fd = file_fd;
    ring = malloc(sizeof(*ring));
    if (!ring) {
    free(fi);
    goto cleanup;
    }
    memset(ring, 0, sizeof(struct io_ring));
    if (setup_io_uring(ring))
    goto out;
    if (handle_uring_sq(ring, fi, lam))
    goto out;
    ret = handle_uring_cq(ring);
    out:
    free(ring);
    for (int i = 0; i < blocks; i++) {
    if (fi.iovecs[i].iov_base) {
    let mut addr: u64 = ((uint64_t)fi.iovecs[i].iov_base);
    switch (lam) {
    case LAM_U57_BITS: /* Clear bits 62:57 */
    addr = (addr & ~(LAM_U57_MASK));
    break;
    }
    free((void *)addr);
    fi.iovecs[i].iov_base = core::ptr::null_mut();
    }
    }
    free(fi);
    cleanup:
    close(file_fd);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn handle_uring(test: *mut testcases) -> c_int {
    int handle_uring(struct testcases *test)
    {
    let mut ret: c_int = 0;
    if (test.later == 0 && test.lam != 0)
    if (set_lam(test.lam) != 0)
    return 1;
    if (sigsetjmp(segv_env, 1) == 0) {
    signal(SIGSEGV, segv_handler);
    ret = do_uring(test.lam);
    } else {
    ret = 2;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn fork_test(test: *mut testcases) -> c_int {
    static int fork_test(struct testcases *test)
    {
    int ret, child_ret;
    pid_t pid;
    pid = fork();
    if (pid < 0) {
    perror("Fork failed.");
    ret = 1;
    } else if (pid == 0) {
    ret = test.test_func(test);
    exit(ret);
    } else {
    wait(&child_ret);
    ret = WEXITSTATUS(child_ret);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn handle_execve(test: *mut testcases) -> c_int {
    static int handle_execve(struct testcases *test)
    {
    int ret, child_ret;
    let mut lam: c_int = test.lam;
    pid_t pid;
    pid = fork();
    if (pid < 0) {
    perror("Fork failed.");
    ret = 1;
    } else if (pid == 0) {
    char path[PATH_MAX] = {0};
// Set LAM mode in parent process
    if (set_lam(lam) != 0)
    return 1;
// Get current binary's path and the binary was run by execve
    if (readlink("/proc/self/exe", path, PATH_MAX - 1) <= 0)
    exit(-1);
// run binary to get LAM mode and return to parent process
    if (execlp(path, path, "-t 0x0", core::ptr::null_mut()) < 0) {
    perror("error on exec");
    exit(-1);
    }
    } else {
    wait(&child_ret);
    ret = WEXITSTATUS(child_ret);
    if (ret != LAM_NONE)
    return 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn handle_inheritance(test: *mut testcases) -> c_int {
    static int handle_inheritance(struct testcases *test)
    {
    int ret, child_ret;
    let mut lam: c_int = test.lam;
    pid_t pid;
// Set LAM mode in parent process
    if (set_lam(lam) != 0)
    return 1;
    pid = fork();
    if (pid < 0) {
    perror("Fork failed.");
    return 1;
    } else if (pid == 0) {
// Set LAM mode in parent process
    let mut child_lam: c_int = get_lam();
    exit(child_lam);
    } else {
    wait(&child_ret);
    ret = WEXITSTATUS(child_ret);
    if (lam != ret)
    return 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn thread_fn_get_lam(arg: *mut c_void) -> c_int {
    static int thread_fn_get_lam(void *arg)
    {
    return get_lam();
    }
#[no_mangle]
unsafe extern "C" fn thread_fn_set_lam(arg: *mut c_void) -> c_int {
    static int thread_fn_set_lam(void *arg)
    {
    struct testcases *test = arg;
    return set_lam(test.lam);
    }
#[no_mangle]
unsafe extern "C" fn handle_thread(test: *mut testcases) -> c_int {
    static int handle_thread(struct testcases *test)
    {
    char stack[STACK_SIZE];
    int ret, child_ret;
    let mut lam: c_int = 0;
    pid_t pid;
// Set LAM mode in parent process
    if (!test.later) {
    lam = test.lam;
    if (set_lam(lam) != 0)
    return 1;
    }
    pid = clone(thread_fn_get_lam, stack + STACK_SIZE,
    SIGCHLD | CLONE_FILES | CLONE_FS | CLONE_VM, core::ptr::null_mut());
    if (pid < 0) {
    perror("Clone failed.");
    return 1;
    }
    waitpid(pid, &child_ret, 0);
    ret = WEXITSTATUS(child_ret);
    if (lam != ret)
    return 1;
    if (test.later) {
    if (set_lam(test.lam) != 0)
    return 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn handle_thread_enable(test: *mut testcases) -> c_int {
    static int handle_thread_enable(struct testcases *test)
    {
    char stack[STACK_SIZE];
    int ret, child_ret;
    let mut lam: c_int = test.lam;
    pid_t pid;
    pid = clone(thread_fn_set_lam, stack + STACK_SIZE,
    SIGCHLD | CLONE_FILES | CLONE_FS | CLONE_VM, test);
    if (pid < 0) {
    perror("Clone failed.");
    return 1;
    }
    waitpid(pid, &child_ret, 0);
    ret = WEXITSTATUS(child_ret);
    if (lam != ret)
    return 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn run_test(test: *mut testcases, count: c_int) {
    static void run_test(struct testcases *test, int count)
    {
    int i, ret = 0;
    for (i = 0; i < count; i++) {
    struct testcases *t = test + i;
// fork a process to run test case
    tests_cnt++;
    ret = fork_test(t);
// return 3 is not support LA57, the case should be skipped
    if (ret == 3) {
    ksft_test_result_skip("%s", t.msg);
    continue;
    }
    if (ret != 0)
    ret = (t.expected == ret);
    else
    ret = !(t.expected);
    ksft_test_result(ret, "%s", t.msg);
    }
    }
    static struct testcases uring_cases[] = {
    {
    .later = 0,
    .lam = LAM_U57_BITS,
    .test_func = handle_uring,
    .msg = "URING: LAM_U57. Dereferencing pointer with metadata\n",
    },
    {
    .later = 1,
    .expected = 1,
    .lam = LAM_U57_BITS,
    .test_func = handle_uring,
    .msg = "URING:[Negative] Disable LAM. Dereferencing pointer with metadata.\n",
    },
    };
    static struct testcases malloc_cases[] = {
    {
    .later = 0,
    .lam = LAM_U57_BITS,
    .test_func = handle_malloc,
    .msg = "MALLOC: LAM_U57. Dereferencing pointer with metadata\n",
    },
    {
    .later = 1,
    .expected = 2,
    .lam = LAM_U57_BITS,
    .test_func = handle_malloc,
    .msg = "MALLOC:[Negative] Disable LAM. Dereferencing pointer with metadata.\n",
    },
    };
    static struct testcases bits_cases[] = {
    {
    .test_func = handle_max_bits,
    .msg = "BITS: Check default tag bits\n",
    },
    };
    static struct testcases syscall_cases[] = {
    {
    .later = 0,
    .lam = LAM_U57_BITS,
    .test_func = handle_syscall,
    .msg = "SYSCALL: LAM_U57. syscall with metadata\n",
    },
    {
    .later = 1,
    .expected = 1,
    .lam = LAM_U57_BITS,
    .test_func = handle_syscall,
    .msg = "SYSCALL:[Negative] Disable LAM. Dereferencing pointer with metadata.\n",
    },
    {
    .later = GET_USER_USER,
    .lam = LAM_U57_BITS,
    .test_func = get_user_syscall,
    .msg = "GET_USER: get_user() and pass a properly tagged user pointer.\n",
    },
    {
    .later = GET_USER_KERNEL_TOP,
    .expected = 1,
    .lam = LAM_U57_BITS,
    .test_func = get_user_syscall,
    .msg = "GET_USER:[Negative] get_user() with a kernel pointer and the top bit cleared.\n",
    },
    {
    .later = GET_USER_KERNEL_BOT,
    .expected = 1,
    .lam = LAM_U57_BITS,
    .test_func = get_user_syscall,
    .msg = "GET_USER:[Negative] get_user() with a kernel pointer and the bottom sign-extension bit cleared.\n",
    },
    {
    .later = GET_USER_KERNEL,
    .expected = 1,
    .lam = LAM_U57_BITS,
    .test_func = get_user_syscall,
    .msg = "GET_USER:[Negative] get_user() and pass a kernel pointer.\n",
    },
    };
    static struct testcases mmap_cases[] = {
    {
    .later = 1,
    .expected = 0,
    .lam = LAM_U57_BITS,
    .addr = HIGH_ADDR,
    .test_func = handle_mmap,
    .msg = "MMAP: First mmap high address, then set LAM_U57.\n",
    },
    {
    .later = 0,
    .expected = 0,
    .lam = LAM_U57_BITS,
    .addr = HIGH_ADDR,
    .test_func = handle_mmap,
    .msg = "MMAP: First LAM_U57, then High address.\n",
    },
    {
    .later = 0,
    .expected = 0,
    .lam = LAM_U57_BITS,
    .addr = LOW_ADDR,
    .test_func = handle_mmap,
    .msg = "MMAP: First LAM_U57, then Low address.\n",
    },
    };
    static struct testcases inheritance_cases[] = {
    {
    .expected = 0,
    .lam = LAM_U57_BITS,
    .test_func = handle_inheritance,
    .msg = "FORK: LAM_U57, child process should get LAM mode same as parent\n",
    },
    {
    .expected = 0,
    .lam = LAM_U57_BITS,
    .test_func = handle_thread,
    .msg = "THREAD: LAM_U57, child thread should get LAM mode same as parent\n",
    },
    {
    .expected = 1,
    .lam = LAM_U57_BITS,
    .test_func = handle_thread_enable,
    .msg = "THREAD: [NEGATIVE] Enable LAM in child.\n",
    },
    {
    .expected = 1,
    .later = 1,
    .lam = LAM_U57_BITS,
    .test_func = handle_thread,
    .msg = "THREAD: [NEGATIVE] Enable LAM in parent after thread created.\n",
    },
    {
    .expected = 0,
    .lam = LAM_U57_BITS,
    .test_func = handle_execve,
    .msg = "EXECVE: LAM_U57, child process should get disabled LAM mode\n",
    },
    };
#[no_mangle]
unsafe extern "C" fn cmd_help() {
    static void cmd_help(void)
    {
    printf("usage: lam [-h] [-t test list]\n");
    printf("\t-t test list: run tests specified in the test list, default:0x%x\n", TEST_MASK);
    printf("\t\t0x1:malloc; 0x2:max_bits; 0x4:mmap; 0x8:syscall; 0x10:io_uring; 0x20:inherit;\n");
    printf("\t-h: help\n");
    }
// Check for file existence
#[no_mangle]
pub unsafe extern "C" fn file_Exists(fileName: *const c_char) -> u8 {
    uint8_t file_Exists(const char *fileName)
    {
    struct stat buffer;
    let mut ret: u8 = (stat(fileName, &buffer) == 0);
    return ret;
    }
// Sysfs idxd files
    const char *dsa_configs[] = {
    "echo 1 > /sys/bus/dsa/devices/dsa0/wq0.1/group_id",
    "echo shared > /sys/bus/dsa/devices/dsa0/wq0.1/mode",
    "echo 10 > /sys/bus/dsa/devices/dsa0/wq0.1/priority",
    "echo 16 > /sys/bus/dsa/devices/dsa0/wq0.1/size",
    "echo 15 > /sys/bus/dsa/devices/dsa0/wq0.1/threshold",
    "echo user > /sys/bus/dsa/devices/dsa0/wq0.1/type",
    "echo MyApp1 > /sys/bus/dsa/devices/dsa0/wq0.1/name",
    "echo 1 > /sys/bus/dsa/devices/dsa0/engine0.1/group_id",
    "echo dsa0 > /sys/bus/dsa/drivers/idxd/bind",
// bind files and devices, generated a device file in /dev
    "echo wq0.1 > /sys/bus/dsa/drivers/user/bind",
    };
// DSA device file
    const char *dsaDeviceFile = "/dev/dsa/wq0.1";
// file for io
    const char *dsaPasidEnable = "/sys/bus/dsa/devices/dsa0/pasid_enabled";
//
// DSA depends on kernel cmdline "intel_iommu=on,sm_on"
// return pasid_enabled (0: disable 1:enable)
//
#[no_mangle]
pub unsafe extern "C" fn Check_DSA_Kernel_Setting() -> c_int {
    int Check_DSA_Kernel_Setting(void)
    {
    char command[256] = "";
    char buf[256] = "";
    char *ptr;
    let mut rv: c_int = -1;
    snprintf(command, sizeof(command) - 1, "cat %s", dsaPasidEnable);
    FILE *cmd = popen(command, "r");
    if (cmd) {
    while (fgets(buf, sizeof(buf) - 1, cmd) != core::ptr::null_mut());
    pclose(cmd);
    rv = strtol(buf, &ptr, 16);
    }
    return rv;
    }
//
// Config DSA's sysfs files as shared DSA's WQ.
// Generated a device file /dev/dsa/wq0.1
// Return:  0 OK; 1 Failed; 3 Skip(SVA disabled).
//
#[no_mangle]
pub unsafe extern "C" fn Dsa_Init_Sysfs() -> c_int {
    int Dsa_Init_Sysfs(void)
    {
    let mut len: c_uint = ARRAY_SIZE(dsa_configs);
    const char **p = dsa_configs;
    if (file_Exists(dsaDeviceFile) == 1)
    return 0;
// check the idxd driver
    if (file_Exists(dsaPasidEnable) != 1) {
    printf("Please make sure idxd driver was loaded\n");
    return 3;
    }
// Check SVA feature
    if (Check_DSA_Kernel_Setting() != 1) {
    printf("Please enable SVA.(Add intel_iommu=on,sm_on in kernel cmdline)\n");
    return 3;
    }
// Check the idxd device file on /dev/dsa/
    for (int i = 0; i < len; i++) {
    if (system(p[i]))
    return 1;
    }
// After config, /dev/dsa/wq0.1 should be generated
    return (file_Exists(dsaDeviceFile) != 1);
    }
//
// Open DSA device file, triger API: iommu_sva_alloc_pasid
//
    void *allocate_dsa_pasid(void)
    {
    int fd;
    void *wq;
    fd = open(dsaDeviceFile, O_RDWR);
    if (fd < 0) {
    perror("open");
    return MAP_FAILED;
    }
    wq = mmap(core::ptr::null_mut(), 0x1000, PROT_WRITE,
    MAP_SHARED | MAP_POPULATE, fd, 0);
    close(fd);
    if (wq == MAP_FAILED)
    perror("mmap");
    return wq;
    }
#[no_mangle]
pub unsafe extern "C" fn set_force_svm() -> c_int {
    int set_force_svm(void)
    {
    let mut ret: c_int = 0;
    ret = syscall(SYS_arch_prctl, ARCH_FORCE_TAGGED_SVA);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn handle_pasid(test: *mut testcases) -> c_int {
    int handle_pasid(struct testcases *test)
    {
    let mut tmp: c_uint = test.cmd;
    let mut runed: c_uint = 0x0;
    let mut ret: c_int = 0;
    void *wq = core::ptr::null_mut();
    ret = Dsa_Init_Sysfs();
    if (ret != 0)
    return ret;
    for (int i = 0; i < 3; i++) {
    let mut err: c_int = 0;
    if (tmp & 0x1) {
// run set lam mode
    if ((runed & 0x1) == 0)	{
    err = set_lam(LAM_U57_BITS);
    runed = runed | 0x1;
    } else
    err = 1;
    } else if (tmp & 0x4) {
// run force svm
    if ((runed & 0x4) == 0)	{
    err = set_force_svm();
    runed = runed | 0x4;
    } else
    err = 1;
    } else if (tmp & 0x2) {
// run allocate pasid
    if ((runed & 0x2) == 0) {
    runed = runed | 0x2;
    wq = allocate_dsa_pasid();
    if (wq == MAP_FAILED)
    err = 1;
    } else
    err = 1;
    }
    ret = ret + err;
    if (ret > 0)
    break;
    tmp = tmp >> 4;
    }
    if (wq != MAP_FAILED && wq != core::ptr::null_mut())
    if (munmap(wq, 0x1000))
    printf("munmap failed %d\n", errno);
    if (runed != 0x7)
    ret = 1;
    return (ret != 0);
    }
//
// Pasid test depends on idxd and SVA, kernel should enable iommu and sm.
// command line(intel_iommu=on,sm_on)
//
    static struct testcases pasid_cases[] = {
    {
    .expected = 1,
    .cmd = PAS_CMD(LAM_CMD_BIT, PAS_CMD_BIT, SVA_CMD_BIT),
    .test_func = handle_pasid,
    .msg = "PASID: [Negative] Execute LAM, PASID, SVA in sequence\n",
    },
    {
    .expected = 0,
    .cmd = PAS_CMD(LAM_CMD_BIT, SVA_CMD_BIT, PAS_CMD_BIT),
    .test_func = handle_pasid,
    .msg = "PASID: Execute LAM, SVA, PASID in sequence\n",
    },
    {
    .expected = 1,
    .cmd = PAS_CMD(PAS_CMD_BIT, LAM_CMD_BIT, SVA_CMD_BIT),
    .test_func = handle_pasid,
    .msg = "PASID: [Negative] Execute PASID, LAM, SVA in sequence\n",
    },
    {
    .expected = 0,
    .cmd = PAS_CMD(PAS_CMD_BIT, SVA_CMD_BIT, LAM_CMD_BIT),
    .test_func = handle_pasid,
    .msg = "PASID: Execute PASID, SVA, LAM in sequence\n",
    },
    {
    .expected = 0,
    .cmd = PAS_CMD(SVA_CMD_BIT, LAM_CMD_BIT, PAS_CMD_BIT),
    .test_func = handle_pasid,
    .msg = "PASID: Execute SVA, LAM, PASID in sequence\n",
    },
    {
    .expected = 0,
    .cmd = PAS_CMD(SVA_CMD_BIT, PAS_CMD_BIT, LAM_CMD_BIT),
    .test_func = handle_pasid,
    .msg = "PASID: Execute SVA, PASID, LAM in sequence\n",
    },
    };
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    let mut c: c_int = 0;
    let mut tests: c_uint = TEST_MASK;
    tests_cnt = 0;
    if (!lam_is_available())
    return KSFT_SKIP;
    while ((c = getopt(argc, argv, "ht:")) != -1) {
    switch (c) {
    case 't':
    tests = strtoul(optarg, core::ptr::null_mut(), 16);
    if (tests && !(tests & TEST_MASK)) {
    ksft_print_msg("Invalid argument!\n");
    return -1;
    }
    break;
    case 'h':
    cmd_help();
    return 0;
    default:
    ksft_print_msg("Invalid argument\n");
    return -1;
    }
    }
//
// When tests is 0, it is not a real test case;
// the option used by test case(execve) to check the lam mode in
// process generated by execve, the process read back lam mode and
// check with lam mode in parent process.
//
    if (!tests)
    return (get_lam());
// Run test cases
    if (tests & FUNC_MALLOC)
    run_test(malloc_cases, ARRAY_SIZE(malloc_cases));
    if (tests & FUNC_BITS)
    run_test(bits_cases, ARRAY_SIZE(bits_cases));
    if (tests & FUNC_MMAP)
    run_test(mmap_cases, ARRAY_SIZE(mmap_cases));
    if (tests & FUNC_SYSCALL)
    run_test(syscall_cases, ARRAY_SIZE(syscall_cases));
    if (tests & FUNC_URING)
    run_test(uring_cases, ARRAY_SIZE(uring_cases));
    if (tests & FUNC_INHERITE)
    run_test(inheritance_cases, ARRAY_SIZE(inheritance_cases));
    if (tests & FUNC_PASID)
    run_test(pasid_cases, ARRAY_SIZE(pasid_cases));
    ksft_set_plan(tests_cnt);
    ksft_exit_pass();
    }
