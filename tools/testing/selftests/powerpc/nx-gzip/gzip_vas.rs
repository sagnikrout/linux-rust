//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/nx-gzip/gzip_vas.c
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
// Copyright 2020 IBM Corp.
//
// Author: Bulent Abali <abali@us.ibm.com>
//

// Macro flag: #define barrier()

// Macro flag: #define cpu_pri_default()
// Macro flag: #define cpu_pri_low()

    void *nx_fault_storage_address;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nx_handle {
    pub fd: c_int,
    pub function: c_int,
    pub paste_addr: *mut c_void,
}

#[no_mangle]
unsafe extern "C" fn open_device_nodes(devname: *mut c_char, pri: c_int, handle: *mut nx_handle) -> c_int {
    static int open_device_nodes(char *devname, int pri, struct nx_handle *handle)
    {
    int rc, fd;
    void *addr;
    struct vas_tx_win_open_attr txattr;
    fd = open(devname, O_RDWR);
    if (fd < 0) {
    fprintf(stderr, " open device name %s\n", devname);
    return -errno;
    }
    memset(&txattr, 0, sizeof(txattr));
    txattr.version = 1;
    txattr.vas_id = pri;
    rc = ioctl(fd, VAS_TX_WIN_OPEN, (unsigned long)&txattr);
    if (rc < 0) {
    fprintf(stderr, "ioctl() n %d, error %d\n", rc, errno);
    rc = -errno;
    goto out;
    }
    addr = mmap(core::ptr::null_mut(), 4096, PROT_READ|PROT_WRITE, MAP_SHARED, fd, 0ULL);
    if (addr == MAP_FAILED) {
    fprintf(stderr, "mmap() failed, errno %d\n", errno);
    rc = -errno;
    goto out;
    }
    handle.fd = fd;
    handle.paste_addr = (void *)((char *)addr + 0x400);
    rc = 0;
    out:
    close(fd);
    return rc;
    }
    void *nx_function_begin(int function, int pri)
    {
    int rc;
    char *devname = "/dev/crypto/nx-gzip";
    struct nx_handle *nxhandle;
    if (function != NX_FUNC_COMP_GZIP) {
    errno = EINVAL;
    fprintf(stderr, " NX_FUNC_COMP_GZIP not found\n");
    return core::ptr::null_mut();
    }
    nxhandle = malloc(sizeof(*nxhandle));
    if (!nxhandle) {
    errno = ENOMEM;
    fprintf(stderr, " No memory\n");
    return core::ptr::null_mut();
    }
    nxhandle.function = function;
    rc = open_device_nodes(devname, pri, nxhandle);
    if (rc < 0) {
    errno = -rc;
    fprintf(stderr, " open_device_nodes failed\n");
    return core::ptr::null_mut();
    }
    return nxhandle;
    }
#[no_mangle]
pub unsafe extern "C" fn nx_function_end(handle: *mut c_void) -> c_int {
    int nx_function_end(void *handle)
    {
    let mut rc: c_int = 0;
    struct nx_handle *nxhandle = handle;
    rc = munmap(nxhandle.paste_addr - 0x400, 4096);
    if (rc < 0) {
    fprintf(stderr, "munmap() failed, errno %d\n", errno);
    return rc;
    }
    close(nxhandle.fd);
    free(nxhandle);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn nx_wait_for_csb(cmdp: *mut nx_gzip_crb_cpb_t) -> c_int {
    static int nx_wait_for_csb(struct nx_gzip_crb_cpb_t *cmdp)
    {
    let mut poll: c_long = 0;
    uint64_t t;
// Save power and let other threads use the h/w. top may show
// 100% but only because OS doesn't know we slowed the this
// h/w thread while polling. We're letting other threads have
// higher throughput on the core.
//
    cpu_pri_low();

    t = __ppc_get_timebase();
    while (getnn(cmdp.crb.csb, csb_v) == 0) {
    ++poll;
    hwsync();
    cpu_pri_low();
// usleep(0) takes around 29000 ticks ~60 us.
// 300000 is spinning for about 600 us then
// start sleeping.
//
    if ((__ppc_get_timebase() - t) > USLEEP_TH) {
    cpu_pri_default();
    usleep(1);
    }
    if (poll > CSB_MAX_POLL)
    break;
// Fault address from signal handler
    if (nx_fault_storage_address) {
    cpu_pri_default();
    return -EAGAIN;
    }
    }
    cpu_pri_default();
// hw has updated csb and output buffer
    hwsync();
// Check CSB flags.
    if (getnn(cmdp.crb.csb, csb_v) == 0) {
    fprintf(stderr, "CSB still not valid after %d polls.\n",
    (int) poll);
    prt_err("CSB still not valid after %d polls, giving up.\n",
    (int) poll);
    return -ETIMEDOUT;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nxu_run_job(cmdp: *mut nx_gzip_crb_cpb_t, handle: *mut c_void) -> c_int {
    static int nxu_run_job(struct nx_gzip_crb_cpb_t *cmdp, void *handle)
    {
    int i, ret, retries;
    struct nx_handle *nxhandle = handle;
    assert(handle != core::ptr::null_mut());
    i = 0;
    retries = 5000;
    while (i++ < retries) {
    hwsync();
    vas_copy(&cmdp.crb, 0);
    ret = vas_paste(nxhandle.paste_addr, 0);
    hwsync();
    NXPRT(fprintf(stderr, "Paste attempt %d/%d returns 0x%x\n",
    i, retries, ret));
    if ((ret == 2) || (ret == 3)) {
    ret = nx_wait_for_csb(cmdp);
    if (!ret) {
    goto out;
    } else if (ret == -EAGAIN) {
    long x;
    prt_err("Touching address %p, 0x%lx\n",
    nx_fault_storage_address,
// (long *) nx_fault_storage_address);
    x = *(long *) nx_fault_storage_address;
// (long *) nx_fault_storage_address = x;
    nx_fault_storage_address = 0;
    continue;
    } else {
    prt_err("wait_for_csb() returns %d\n", ret);
    break;
    }
    } else {
    if (i < 10) {
// spin for few ticks

    uint64_t fail_spin;
    fail_spin = __ppc_get_timebase();
    while ((__ppc_get_timebase() - fail_spin) <
    SPIN_TH)
    ;
    } else {
// sleep
    let mut pr: c_uint = 0;
    if (pr++ % 100 == 0) {
    prt_err("Paste attempt %d/", i);
    prt_err("%d, failed pid= %d\n", retries,
    getpid());
    }
    usleep(1);
    }
    continue;
    }
    }
    out:
    cpu_pri_default();
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn nxu_submit_job(cmdp: *mut nx_gzip_crb_cpb_t, handle: *mut c_void) -> c_int {
    int nxu_submit_job(struct nx_gzip_crb_cpb_t *cmdp, void *handle)
    {
    int cc;
    cc = nxu_run_job(cmdp, handle);
    if (!cc)
    cc = getnn(cmdp.crb.csb, csb_cc);      /* CC Table 6-8 */
    return cc;
    }
#[no_mangle]
pub unsafe extern "C" fn nxu_sigsegv_handler(sig: c_int, info: *mut siginfo_t, ctx: *mut c_void) {
    void nxu_sigsegv_handler(int sig, siginfo_t *info, void *ctx)
    {
    fprintf(stderr, "%d: Got signal %d si_code %d, si_addr %p\n", getpid(),
    sig, info.si_code, info.si_addr);
    nx_fault_storage_address = info.si_addr;
    }
//
// Fault in pages prior to NX job submission.  wr=1 may be required to
// touch writeable pages.  System zero pages do not fault-in the page as
// intended.  Typically set wr=1 for NX target pages and set wr=0 for NX
// source pages.
//
#[no_mangle]
pub unsafe extern "C" fn nxu_touch_pages(buf: *mut c_void, buf_len: c_long, page_len: c_long, wr: c_int) -> c_int {
    int nxu_touch_pages(void *buf, long buf_len, long page_len, int wr)
    {
    char *begin = buf;
    char *end = (char *) buf + buf_len - 1;
    volatile char t;
    assert(buf_len >= 0 && !!buf);
    NXPRT(fprintf(stderr, "touch %p %p len 0x%lx wr=%d\n", buf,
    (buf + buf_len), buf_len, wr));
    if (buf_len <= 0 || buf == core::ptr::null_mut())
    return -1;
    do {
    t = *begin;
    if (wr)
// begin = t;
    begin = begin + page_len;
    } while (begin < end);
// When buf_sz is small or buf tail is in another page
    t = *end;
    if (wr)
// end = t;
    return 0;
    }
