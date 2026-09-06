//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/diag/diag324.c
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
// Request power readings for resources in a computing environment via
// diag 0x324. diag 0x324 stores the power readings in the power information
// block (pib).
//
// Copyright IBM Corp. 2024
//

    enum subcode {
    DIAG324_SUBC_0 = 0,
    DIAG324_SUBC_1 = 1,
    DIAG324_SUBC_2 = 2,
    };
    enum retcode {
    DIAG324_RET_SUCCESS		= 0x0001,
    DIAG324_RET_SUBC_NOTAVAIL	= 0x0103,
    DIAG324_RET_INSUFFICIENT_SIZE	= 0x0104,
    DIAG324_RET_READING_UNAVAILABLE	= 0x0105,
    };
    union diag324_response {
    u64 response;
    struct {
    u64 installed	: 32;
    u64		: 16;
    u64 rc		: 16;
    } sc0;
    struct {
    u64 format	: 16;
    u64		: 16;
    u64 pib_len	: 16;
    u64 rc		: 16;
    } sc1;
    struct {
    u64		: 48;
    u64 rc		: 16;
    } sc2;
    };
    union diag324_request {
    u64 request;
    struct {
    u64		: 32;
    u64 allocated	: 16;
    u64		: 12;
    u64 sc		: 4;
    } sc2;
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pib {
    pub 8: u32 :,
    pub 8: u32 num :,
    pub 16: u32 len :,
    pub 24: u32 :,
    pub 8: u32 hlen :,
    pub 64: u64 :,
    pub intv: u64,
    pub r: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pibdata {
    pub pib: *mut pib,
    pub expire: ktime_t,
    pub sequence: u64,
    pub len: usize,
    pub rc: c_int,
}

    static DEFINE_MUTEX(pibmutex);
    static struct pibdata pibdata;

    static void pibwork_handler(struct work_struct *work);
    static DECLARE_DELAYED_WORK(pibwork, pibwork_handler);
#[no_mangle]
unsafe extern "C" fn diag324(subcode: c_ulong, addr: *mut c_void) -> c_ulong {
    static unsigned long diag324(unsigned long subcode, void *addr)
    {
    let mut rp: union register_pair = { .even = (unsigned long)addr };
    diag_stat_inc(DIAG_STAT_X324);
    asm volatile("diag	%[rp],%[subcode],0x324"
    : [rp] "+d" (rp.pair)
    : [subcode] "d" (subcode)
    : "memory");
    return rp.odd;
    }
#[no_mangle]
unsafe extern "C" fn pibwork_handler(work: *mut work_struct) {
    static void pibwork_handler(struct work_struct *work)
    {
    struct pibdata *data = &pibdata;
    ktime_t timedout;
    mutex_lock(&pibmutex);
    timedout = ktime_add_ns(data.expire, PIBWORK_DELAY);
    if (ktime_before(ktime_get(), timedout)) {
    mod_delayed_work(system_percpu_wq, &pibwork, nsecs_to_jiffies(PIBWORK_DELAY));
    goto out;
    }
    vfree(data.pib);
    data.pib = core::ptr::null_mut();
    out:
    mutex_unlock(&pibmutex);
    }
#[no_mangle]
unsafe extern "C" fn pib_update(data: *mut pibdata) {
    static void pib_update(struct pibdata *data)
    {
    let mut req: union diag324_request = { .sc2.sc = DIAG324_SUBC_2, .sc2.allocated = data.len };
    union diag324_response res;
    int rc;
    memset(data.pib, 0, data.len);
    res.response = diag324(req.request, data.pib);
    switch (res.sc2.rc) {
    case DIAG324_RET_SUCCESS:
    rc = 0;
    break;
    case DIAG324_RET_SUBC_NOTAVAIL:
    rc = -ENOENT;
    break;
    case DIAG324_RET_INSUFFICIENT_SIZE:
    rc = -EMSGSIZE;
    break;
    case DIAG324_RET_READING_UNAVAILABLE:
    rc = -EBUSY;
    break;
    default:
    rc = -EINVAL;
    }
    data.rc = rc;
    }
#[no_mangle]
pub unsafe extern "C" fn diag324_pibbuf(arg: c_ulong) -> c_long {
    long diag324_pibbuf(unsigned long arg)
    {
    struct diag324_pib __user *udata = (struct diag324_pib __user *)arg;
    struct pibdata *data = &pibdata;
    let mut first: static bool = true;
    u64 address;
    int rc;
    if (!data.len)
    return -EOPNOTSUPP;
    if (get_user(address, &udata.address))
    return -EFAULT;
    mutex_lock(&pibmutex);
    rc = -ENOMEM;
    if (!data.pib)
    data.pib = vmalloc(data.len);
    if (!data.pib)
    goto out;
    if (first || ktime_after(ktime_get(), data.expire)) {
    pib_update(data);
    data.sequence++;
    data.expire = ktime_add_ns(ktime_get(), tod_to_ns(data.pib.intv));
    mod_delayed_work(system_percpu_wq, &pibwork, nsecs_to_jiffies(PIBWORK_DELAY));
    first = false;
    }
    rc = data.rc;
    if (rc != 0 && rc != -EBUSY)
    goto out;
    rc = copy_to_user((void __user *)address, data.pib, data.pib.len);
    rc |= put_user(data.sequence, &udata.sequence);
    rc = rc ? -EFAULT : data.rc;
    out:
    mutex_unlock(&pibmutex);
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn diag324_piblen(arg: c_ulong) -> c_long {
    long diag324_piblen(unsigned long arg)
    {
    struct pibdata *data = &pibdata;
    if (!data.len)
    return -EOPNOTSUPP;
    if (put_user(data.len, (size_t __user *)arg))
    return -EFAULT;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn diag324_init() -> int __init {
    static int __init diag324_init(void)
    {
    union diag324_response res;
    unsigned long installed;
    if (!sclp.has_diag324)
    return -EOPNOTSUPP;
    res.response = diag324(DIAG324_SUBC_0, core::ptr::null_mut());
    if (res.sc0.rc != DIAG324_RET_SUCCESS)
    return -EOPNOTSUPP;
    installed = res.response;
    if (!test_bit_inv(DIAG324_SUBC_1, &installed))
    return -EOPNOTSUPP;
    if (!test_bit_inv(DIAG324_SUBC_2, &installed))
    return -EOPNOTSUPP;
    res.response = diag324(DIAG324_SUBC_1, core::ptr::null_mut());
    if (res.sc1.rc != DIAG324_RET_SUCCESS)
    return -EOPNOTSUPP;
    pibdata.len = res.sc1.pib_len;
    return 0;
    }
    device_initcall(diag324_init);
