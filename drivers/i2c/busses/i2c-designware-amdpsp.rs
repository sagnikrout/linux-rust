//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-designware-amdpsp.c
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

pub const PSP_I2C_RESERVATION_TIME_MS: c_int = 100;
pub const PSP_I2C_REQ_RETRY_CNT: c_int = 400;

pub const PSP_I2C_REQ_STS_OK: c_uint = 0x0;
pub const PSP_I2C_REQ_STS_BUS_BUSY: c_uint = 0x1;
pub const PSP_I2C_REQ_STS_INV_PARAM: c_uint = 0x3;
    enum psp_i2c_req_type {
    PSP_I2C_REQ_ACQUIRE,
    PSP_I2C_REQ_RELEASE,
    PSP_I2C_REQ_MAX
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_i2c_req {
    pub hdr: psp_req_buffer_hdr,
    pub type: enum psp_i2c_req_type,
}

    static DEFINE_MUTEX(psp_i2c_access_mutex);
    static unsigned long psp_i2c_sem_acquired;
    static u32 psp_i2c_access_count;
    static bool psp_i2c_mbox_fail;
    static struct device *psp_i2c_dev;
    static int (*_psp_send_i2c_req)(struct psp_i2c_req *req);
// Helper to verify status returned by PSP
#[no_mangle]
unsafe extern "C" fn check_i2c_req_sts(req: *mut psp_i2c_req) -> c_int {
    static int check_i2c_req_sts(struct psp_i2c_req *req)
    {
    u32 status;
// Status field in command-response buffer is updated by PSP
    status = READ_ONCE(req.hdr.status);
    switch (status) {
    case PSP_I2C_REQ_STS_OK:
    return 0;
    case PSP_I2C_REQ_STS_BUS_BUSY:
    return -EBUSY;
    case PSP_I2C_REQ_STS_INV_PARAM:
    default:
    return -EIO;
    }
    }
//
// Errors in x86-PSP i2c-arbitration protocol may occur at two levels:
// 1. mailbox communication - PSP is not operational or some IO errors with
// basic communication had happened.
// 2. i2c-requests - PSP refuses to grant i2c arbitration to x86 for too long.
//
// In order to distinguish between these in error handling code all mailbox
// communication errors on the first level (from CCP symbols) will be passed
// up and if -EIO is returned the second level will be checked.
//
#[no_mangle]
unsafe extern "C" fn psp_send_i2c_req_cezanne(req: *mut psp_i2c_req) -> c_int {
    static int psp_send_i2c_req_cezanne(struct psp_i2c_req *req)
    {
    int ret;
    ret = psp_send_platform_access_msg(PSP_I2C_REQ_BUS_CMD, (struct psp_request *)req);
    if (ret == -EIO)
    return check_i2c_req_sts(req);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn psp_send_i2c_req_doorbell(req: *mut psp_i2c_req) -> c_int {
    static int psp_send_i2c_req_doorbell(struct psp_i2c_req *req)
    {
    int ret;
    ret = psp_ring_platform_doorbell(req.type, &req.hdr.status);
    if (ret == -EIO)
    return check_i2c_req_sts(req);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn psp_send_i2c_req(i2c_req_type: enum psp_i2c_req_type) -> c_int {
    static int psp_send_i2c_req(enum psp_i2c_req_type i2c_req_type)
    {
    struct psp_i2c_req *req;
    unsigned long start;
    int status, ret;
// Allocate command-response buffer
    req = kzalloc_obj(*req);
    if (!req)
    return -ENOMEM;
    req.hdr.payload_size = sizeof(*req);
    req.type = i2c_req_type;
    start = jiffies;
    ret = read_poll_timeout(_psp_send_i2c_req, status,
    (status != -EBUSY),
    PSP_I2C_REQ_RETRY_DELAY_US,
    PSP_I2C_REQ_RETRY_CNT * PSP_I2C_REQ_RETRY_DELAY_US,
    0, req);
    if (ret) {
    dev_err(psp_i2c_dev, "Timed out waiting for PSP to %s I2C bus\n",
    (i2c_req_type == PSP_I2C_REQ_ACQUIRE) ?
    "release" : "acquire");
    goto cleanup;
    }
    ret = status;
    if (ret) {
    dev_err(psp_i2c_dev, "PSP communication error\n");
    goto cleanup;
    }
    dev_dbg(psp_i2c_dev, "Request accepted by PSP after %ums\n",
    jiffies_to_msecs(jiffies - start));
    cleanup:
    if (ret) {
    dev_err(psp_i2c_dev, "Assume i2c bus is for exclusive host usage\n");
    psp_i2c_mbox_fail = true;
    }
    kfree(req);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn release_bus() {
    static void release_bus(void)
    {
    int status;
    if (!psp_i2c_sem_acquired)
    return;
    status = psp_send_i2c_req(PSP_I2C_REQ_RELEASE);
    if (status)
    return;
    dev_dbg(psp_i2c_dev, "PSP semaphore held for %ums\n",
    jiffies_to_msecs(jiffies - psp_i2c_sem_acquired));
    psp_i2c_sem_acquired = 0;
    }
#[no_mangle]
unsafe extern "C" fn psp_release_i2c_bus_deferred(work: *mut work_struct) {
    static void psp_release_i2c_bus_deferred(struct work_struct *work)
    {
    guard(mutex)(&psp_i2c_access_mutex);
//
// If there is any pending transaction, cannot release the bus here.
// psp_release_i2c_bus() will take care of this later.
//
    if (psp_i2c_access_count)
    return;
    release_bus();
    }
    static DECLARE_DELAYED_WORK(release_queue, psp_release_i2c_bus_deferred);
#[no_mangle]
unsafe extern "C" fn psp_acquire_i2c_bus() -> c_int {
    static int psp_acquire_i2c_bus(void)
    {
    int status;
    guard(mutex)(&psp_i2c_access_mutex);
// Return early if mailbox malfunctioned
    if (psp_i2c_mbox_fail)
    return 0;
    psp_i2c_access_count++;
//
// No need to request bus arbitration once we are inside semaphore
// reservation period.
//
    if (psp_i2c_sem_acquired)
    return 0;
    status = psp_send_i2c_req(PSP_I2C_REQ_ACQUIRE);
    if (status)
    return 0;
    psp_i2c_sem_acquired = jiffies;
    schedule_delayed_work(&release_queue,
    msecs_to_jiffies(PSP_I2C_RESERVATION_TIME_MS));
//
// In case of errors with PSP arbitrator psp_i2c_mbox_fail variable is
// set above. As a consequence consecutive calls to acquire will bypass
// communication with PSP. At any case i2c bus is granted to the caller,
// thus always return success.
//
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn psp_release_i2c_bus() {
    static void psp_release_i2c_bus(void)
    {
    guard(mutex)(&psp_i2c_access_mutex);
// Return early if mailbox was malfunctioned
    if (psp_i2c_mbox_fail)
    return;
//
// If we are last owner of PSP semaphore, need to release arbitration
// via mailbox.
//
    psp_i2c_access_count--;
    if (psp_i2c_access_count)
    return;
//
// Send a release command to PSP if the semaphore reservation timeout
// elapsed but x86 still owns the controller.
//
    if (!delayed_work_pending(&release_queue))
    release_bus();
    }
//
// Locking methods are based on the default implementation from
// drivers/i2c/i2c-core-base.c, but with PSP acquire and release operations
// added. With this in place we can ensure that i2c clients on the bus shared
// with PSP are able to lock HW access to the bus for arbitrary number of
// operations - that is e.g. write-wait-read.
//
    static void i2c_adapter_dw_psp_lock_bus(struct i2c_adapter *adapter,
    unsigned int flags)
    {
    psp_acquire_i2c_bus();
    rt_mutex_lock_nested(&adapter.bus_lock, i2c_adapter_depth(adapter));
    }
    static int i2c_adapter_dw_psp_trylock_bus(struct i2c_adapter *adapter,
    unsigned int flags)
    {
    int ret;
    ret = rt_mutex_trylock(&adapter.bus_lock);
    if (ret)
    return ret;
    psp_acquire_i2c_bus();
    return ret;
    }
    static void i2c_adapter_dw_psp_unlock_bus(struct i2c_adapter *adapter,
    unsigned int flags)
    {
    psp_release_i2c_bus();
    rt_mutex_unlock(&adapter.bus_lock);
    }
    static const struct i2c_lock_operations i2c_dw_psp_lock_ops = {
    .lock_bus = i2c_adapter_dw_psp_lock_bus,
    .trylock_bus = i2c_adapter_dw_psp_trylock_bus,
    .unlock_bus = i2c_adapter_dw_psp_unlock_bus,
    };
#[no_mangle]
pub unsafe extern "C" fn i2c_dw_amdpsp_probe_lock_support(dev: *mut dw_i2c_dev) -> c_int {
    int i2c_dw_amdpsp_probe_lock_support(struct dw_i2c_dev *dev)
    {
    struct pci_dev *rdev;
    if (!IS_REACHABLE(CONFIG_CRYPTO_DEV_CCP_DD))
    return -ENODEV;
    if (!dev)
    return -ENODEV;
    if (!(dev.flags & ARBITRATION_SEMAPHORE))
    return -ENODEV;
// Allow to bind only one instance of a driver
    if (psp_i2c_dev)
    return -EEXIST;
// Cezanne uses platform mailbox, Mendocino and later use doorbell
    rdev = pci_get_domain_bus_and_slot(0, 0, PCI_DEVFN(0, 0));
    if (rdev.device == 0x1630)
    _psp_send_i2c_req = psp_send_i2c_req_cezanne;
    else
    _psp_send_i2c_req = psp_send_i2c_req_doorbell;
    pci_dev_put(rdev);
    if (psp_check_platform_access_status())
    return -EPROBE_DEFER;
    psp_i2c_dev = dev.dev;
    dev_info(psp_i2c_dev, "I2C bus managed by AMD PSP\n");
//
// Install global locking callbacks for adapter as well as internal i2c
// controller locks.
//
    dev.adapter.lock_ops = &i2c_dw_psp_lock_ops;
    dev.acquire_lock = psp_acquire_i2c_bus;
    dev.release_lock = psp_release_i2c_bus;
    return 0;
    }
