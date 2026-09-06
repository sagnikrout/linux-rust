//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/pseries/papr-vpd.c
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


// SPDX-License-Identifier: GPL-2.0-only

//
// struct rtas_ibm_get_vpd_params - Parameters (in and out) for ibm,get-vpd.
// @loc_code:  In: Caller-provided location code buffer. Must be RTAS-addressable.
// @work_area: In: Caller-provided work area buffer for results.
// @sequence:  In: Sequence number. Out: Next sequence number.
// @written:   Out: Bytes written by ibm,get-vpd to @work_area.
// @status:    Out: RTAS call status.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtas_ibm_get_vpd_params {
    pub loc_code: *const papr_location_code,
    pub work_area: *mut rtas_work_area,
    pub sequence: u32,
    pub written: u32,
    pub status: i32,
}

//
// rtas_ibm_get_vpd() - Call ibm,get-vpd to fill a work area buffer.
// @params: See &struct rtas_ibm_get_vpd_params.
//
// Calls ibm,get-vpd until it errors or successfully deposits data
// into the supplied work area. Handles RTAS retry statuses. Maps RTAS
// error statuses to reasonable errno values.
//
// The caller is expected to invoke rtas_ibm_get_vpd() multiple times
// to retrieve all the VPD for the provided location code. Only one
// sequence should be in progress at any time; starting a new sequence
// will disrupt any sequence already in progress. Serialization of VPD
// retrieval sequences is the responsibility of the caller.
//
// The caller should inspect @params.status to determine whether more
// calls are needed to complete the sequence.
//
// Context: May sleep.
// Return: -ve on error, 0 otherwise.
//
#[no_mangle]
unsafe extern "C" fn rtas_ibm_get_vpd(params: *mut rtas_ibm_get_vpd_params) -> c_int {
    static int rtas_ibm_get_vpd(struct rtas_ibm_get_vpd_params *params)
    {
    const struct papr_location_code *loc_code = params.loc_code;
    struct rtas_work_area *work_area = params.work_area;
    u32 rets[2];
    s32 fwrc;
    int ret;
    lockdep_assert_held(&rtas_ibm_get_vpd_lock);
    do {
    fwrc = rtas_call(rtas_function_token(RTAS_FN_IBM_GET_VPD), 4, 3,
    rets,
    __pa(loc_code),
    rtas_work_area_phys(work_area),
    rtas_work_area_size(work_area),
    params.sequence);
    } while (rtas_busy_delay(fwrc));
    switch (fwrc) {
    case RTAS_HARDWARE_ERROR:
    ret = -EIO;
    break;
    case RTAS_INVALID_PARAMETER:
    ret = -EINVAL;
    break;
    case RTAS_SEQ_START_OVER:
    ret = -EAGAIN;
    pr_info_ratelimited("VPD changed during retrieval, retrying\n");
    break;
    case RTAS_SEQ_MORE_DATA:
    params.sequence = rets[0];
    fallthrough;
    case RTAS_SEQ_COMPLETE:
    params.written = rets[1];
//
// Kernel or firmware bug, do not continue.
//
    if (WARN(params.written > rtas_work_area_size(work_area),
    "possible write beyond end of work area"))
    ret = -EFAULT;
    else
    ret = 0;
    break;
    default:
    ret = -EIO;
    pr_err_ratelimited("unexpected ibm,get-vpd status %d\n", fwrc);
    break;
    }
    params.status = fwrc;
    return ret;
    }
//
// Internal VPD sequence APIs. A VPD sequence is a series of calls to
// ibm,get-vpd for a given location code. The sequence ends when an
// error is encountered or all VPD for the location code has been
// returned.
//
// vpd_sequence_begin() - Begin a VPD retrieval sequence.
// @seq: vpd call parameters from sequence struct
//
// Context: May sleep.
//
#[no_mangle]
unsafe extern "C" fn vpd_sequence_begin(seq: *mut papr_rtas_sequence) {
    static void vpd_sequence_begin(struct papr_rtas_sequence *seq)
    {
    struct rtas_ibm_get_vpd_params *vpd_params;
//
// Use a static data structure for the location code passed to
// RTAS to ensure it's in the RMA and avoid a separate work
// area allocation. Guarded by the function lock.
//
    static struct papr_location_code static_loc_code;
    vpd_params =  (struct rtas_ibm_get_vpd_params *)seq.params;
//
// We could allocate the work area before acquiring the
// function lock, but that would allow concurrent requests to
// exhaust the limited work area pool for no benefit. So
// allocate the work area under the lock.
//
    mutex_lock(&rtas_ibm_get_vpd_lock);
    static_loc_code = *(struct papr_location_code *)vpd_params.loc_code;
    vpd_params =  (struct rtas_ibm_get_vpd_params *)seq.params;
    vpd_params.work_area = rtas_work_area_alloc(SZ_4K);
    vpd_params.loc_code = &static_loc_code;
    vpd_params.sequence = 1;
    vpd_params.status = 0;
    }
//
// vpd_sequence_end() - Finalize a VPD retrieval sequence.
// @seq: Sequence state.
//
// Releases resources obtained by vpd_sequence_begin().
//
#[no_mangle]
unsafe extern "C" fn vpd_sequence_end(seq: *mut papr_rtas_sequence) {
    static void vpd_sequence_end(struct papr_rtas_sequence *seq)
    {
    struct rtas_ibm_get_vpd_params *vpd_params;
    vpd_params =  (struct rtas_ibm_get_vpd_params *)seq.params;
    rtas_work_area_free(vpd_params.work_area);
    mutex_unlock(&rtas_ibm_get_vpd_lock);
    }
//
// Generator function to be passed to papr_rtas_blob_generate().
//
    static const char *vpd_sequence_fill_work_area(struct papr_rtas_sequence *seq,
    size_t *len)
    {
    struct rtas_ibm_get_vpd_params *p;
    bool init_state;
    p = (struct rtas_ibm_get_vpd_params *)seq.params;
    init_state = (p.written == 0) ? true : false;
    if (papr_rtas_sequence_should_stop(seq, p.status, init_state))
    return core::ptr::null_mut();
    if (papr_rtas_sequence_set_err(seq, rtas_ibm_get_vpd(p)))
    return core::ptr::null_mut();
// len = p->written;
    return rtas_work_area_raw_buf(p.work_area);
    }
    static const struct file_operations papr_vpd_handle_ops = {
    .read = papr_rtas_common_handle_read,
    .llseek = papr_rtas_common_handle_seek,
    .release = papr_rtas_common_handle_release,
    };
//
// papr_vpd_create_handle() - Create a fd-based handle for reading VPD.
// @ulc: Location code in user memory; defines the scope of the VPD to
// retrieve.
//
// Handler for PAPR_VPD_IOC_CREATE_HANDLE ioctl command. Validates
// @ulc and instantiates an immutable VPD "blob" for it. The blob is
// attached to a file descriptor for reading by user space. The memory
// backing the blob is freed when the file is released.
//
// The entire requested VPD is retrieved by this call and all
// necessary RTAS interactions are performed before returning the fd
// to user space. This keeps the read handler simple and ensures that
// the kernel can prevent interleaving of ibm,get-vpd call sequences.
//
// Return: The installed fd number if successful, -ve errno otherwise.
//
#[no_mangle]
unsafe extern "C" fn papr_vpd_create_handle(ulc: *mut papr_location_code __user) -> c_long {
    static long papr_vpd_create_handle(struct papr_location_code __user *ulc)
    {
    let mut vpd_params: rtas_ibm_get_vpd_params = {};
    let mut seq: papr_rtas_sequence = {};
    struct papr_location_code klc;
    int fd;
    if (copy_from_user(&klc, ulc, sizeof(klc)))
    return -EFAULT;
    if (!string_is_terminated(klc.str, ARRAY_SIZE(klc.str)))
    return -EINVAL;
    seq = (struct papr_rtas_sequence) {
    .begin = vpd_sequence_begin,
    .end = vpd_sequence_end,
    .work = vpd_sequence_fill_work_area,
    };
    vpd_params.loc_code = &klc;
    seq.params = (void *)&vpd_params;
    fd = papr_rtas_setup_file_interface(&seq, &papr_vpd_handle_ops,
    "[papr-vpd]");
    return fd;
    }
//
// Top-level ioctl handler for /dev/papr-vpd.
//
#[no_mangle]
unsafe extern "C" fn papr_vpd_dev_ioctl(filp: *mut file, ioctl: c_uint, arg: c_ulong) -> c_long {
    static long papr_vpd_dev_ioctl(struct file *filp, unsigned int ioctl, unsigned long arg)
    {
    void __user *argp = ( void __user *)arg;
    long ret;
    switch (ioctl) {
    case PAPR_VPD_IOC_CREATE_HANDLE:
    ret = papr_vpd_create_handle(argp);
    break;
    default:
    ret = -ENOIOCTLCMD;
    break;
    }
    return ret;
    }
    static const struct file_operations papr_vpd_ops = {
    .unlocked_ioctl = papr_vpd_dev_ioctl,
    };
    static struct miscdevice papr_vpd_dev = {
    .minor = MISC_DYNAMIC_MINOR,
    .name = "papr-vpd",
    .fops = &papr_vpd_ops,
    };
#[no_mangle]
unsafe extern "C" fn papr_vpd_init() -> __init int {
    static __init int papr_vpd_init(void)
    {
    if (!rtas_function_implemented(RTAS_FN_IBM_GET_VPD))
    return -ENODEV;
    return misc_register(&papr_vpd_dev);
    }
    machine_device_initcall(pseries, papr_vpd_init);
