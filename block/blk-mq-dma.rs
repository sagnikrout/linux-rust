//! Automatically rewritten from C to Rust
//! Source: block/blk-mq-dma.c
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
// === KERNEL_MACRO_PRELUDE_START ===
macro_rules! EXPORT_SYMBOL { ($($tt:tt)*) => {}; }
macro_rules! EXPORT_SYMBOL_GPL { ($($tt:tt)*) => {}; }
macro_rules! MODULE_LICENSE { ($($tt:tt)*) => {}; }
macro_rules! MODULE_AUTHOR { ($($tt:tt)*) => {}; }
macro_rules! MODULE_DESCRIPTION { ($($tt:tt)*) => {}; }
macro_rules! MODULE_ALIAS { ($($tt:tt)*) => {}; }
macro_rules! module_init { ($($tt:tt)*) => {}; }
macro_rules! module_exit { ($($tt:tt)*) => {}; }
macro_rules! early_initcall { ($($tt:tt)*) => {}; }
macro_rules! core_initcall { ($($tt:tt)*) => {}; }
macro_rules! postcore_initcall { ($($tt:tt)*) => {}; }
macro_rules! arch_initcall { ($($tt:tt)*) => {}; }
macro_rules! subsys_initcall { ($($tt:tt)*) => {}; }
macro_rules! fs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DEFINE { ($($tt:tt)*) => {}; }
macro_rules! ARRAY_SIZE { ($($tt:tt)*) => { 1 }; }
macro_rules! container_of { ($($tt:tt)*) => { core::ptr::null_mut() }; }
macro_rules! sizeof { ($($tt:tt)*) => { 0usize }; }
macro_rules! IS_ENABLED { ($($tt:tt)*) => { false }; }
macro_rules! DECLARE_WORK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_WAKE_Q { ($($tt:tt)*) => {}; }
macro_rules! LLIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! LIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! SET_UID { ($($tt:tt)*) => {}; }
macro_rules! SET_GID { ($($tt:tt)*) => {}; }
macro_rules! list_for_each_entry { ($($tt:tt)*) => { if false }; }
macro_rules! list_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! llist_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! pr_info_once { ($($tt:tt)*) => {}; }
macro_rules! pr_info { ($($tt:tt)*) => {}; }
macro_rules! pr_warn { ($($tt:tt)*) => {}; }
macro_rules! pr_err { ($($tt:tt)*) => {}; }
macro_rules! pr_debug { ($($tt:tt)*) => {}; }
macro_rules! early_param { ($($tt:tt)*) => {}; }
macro_rules! BUILD_BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! WARN_ON { ($($tt:tt)*) => { false }; }
macro_rules! WARN_ON_ONCE { ($($tt:tt)*) => { false }; }
macro_rules! BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! BUG { () => {}; }
macro_rules! IS_ERR { ($($tt:tt)*) => { false }; }
macro_rules! PTR_ERR { ($($tt:tt)*) => { 0 }; }
macro_rules! ERR_PTR { ($($tt:tt)*) => { core::ptr::null_mut() }; }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cred { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct notifier_block { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_notifier_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_header { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_root { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_set { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kern_ipc_perm { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_params { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_queue { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msgseg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_sender { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_receiver { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sembuf { pub sem_num: u16, pub sem_op: i16, pub sem_flg: i16 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem_array { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmid_kernel { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shm_file_data { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wake_q_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct work_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct llist_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;
pub type key_t = i32;
pub type kuid_t = u32;
pub type kgid_t = u32;
pub type int = c_int;
pub type uint = c_uint;
pub type ulong = c_ulong;
pub type long = c_long;
pub type void = c_void;

// Standard Linux Error Codes
pub const EPERM: c_int = 1;
pub const ENOENT: c_int = 2;
pub const ESRCH: c_int = 3;
pub const EINTR: c_int = 4;
pub const EIO: c_int = 5;
pub const ENXIO: c_int = 6;
pub const E2BIG: c_int = 7;
pub const ENOEXEC: c_int = 8;
pub const EBADF: c_int = 9;
pub const ECHILD: c_int = 10;
pub const EAGAIN: c_int = 11;
pub const ENOMEM: c_int = 12;
pub const EACCES: c_int = 13;
pub const EFAULT: c_int = 14;
pub const EBUSY: c_int = 16;
pub const EEXIST: c_int = 17;
pub const EXDEV: c_int = 18;
pub const ENODEV: c_int = 19;
pub const ENOTDIR: c_int = 20;
pub const EISDIR: c_int = 21;
pub const EINVAL: c_int = 22;
pub const ENFILE: c_int = 23;
pub const EMFILE: c_int = 24;
pub const ENOSPC: c_int = 28;
pub const EROFS: c_int = 30;
pub const EIDRM: c_int = 43;
pub const EOPNOTSUPP: c_int = 95;
pub const ENOTSUPP: c_int = 524;

// Standard Memory Constants
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const GFP_KERNEL: c_uint = 0xcc0;
pub const GFP_ATOMIC: c_uint = 0x80000;
pub const GFP_NOWAIT: c_uint = 0;

// Standard Core Primitives
extern "C" {
    pub static current: *mut task_struct;
    pub fn printk(fmt: *const c_char, ...) -> c_int;
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    pub fn kfree(ptr: *mut c_void);
}
// === KERNEL_MACRO_PRELUDE_END ===


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2025 Christoph Hellwig
//

#[no_mangle]
unsafe extern "C" fn __blk_map_iter_next(iter: *mut blk_map_iter) -> bool {
    if (iter.iter.bi_size) {
    return true;
    }
    if (!iter.bio || !iter.bio.bi_next) {
    return false;
    }
    iter.bio = iter.bio.bi_next;
    if (iter.is_integrity) {
    iter.iter = bio_integrity(iter.bio).bip_iter;
    iter.bvecs = bio_integrity(iter.bio).bip_vec;
    } else {
    iter.iter = iter.bio.bi_iter;
    iter.bvecs = iter.bio.bi_io_vec;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_map_iter_next(req: *mut request, iter: *mut blk_map_iter, vec: *mut phys_vec) -> bool {
    let mut max_size = 0;
pub static mut bv: usize = 0;
    if (!iter.iter.bi_size) {
    return false;
    }
    bv = mp_bvec_iter_bvec(iter.bvecs, iter.iter);
    vec.paddr = bvec_phys(&bv);
    max_size = get_max_segment_size(&req.q.limits, vec.paddr, UINT_MAX);
    bv.bv_len = min(bv.bv_len, max_size);
    bvec_iter_advance_single(iter.bvecs, &iter.iter, bv.bv_len);
//
// If we are entirely done with this bi_io_vec entry, check if the next
// one could be merged into it.  This typically happens when moving to
// the next bio, but some callers also don't pack bvecs tight.
//
    while (!iter.iter.bi_size || !iter.iter.bi_offset) {
pub static mut next: usize = 0;
    if (!__blk_map_iter_next(iter)) {
    break;
    }
    next = mp_bvec_iter_bvec(iter.bvecs, iter.iter);
    if (bv.bv_len + next.bv_len > max_size ||
    !biovec_phys_mergeable(req.q, &bv, &next)) {
    break;
    }
    bv.bv_len += next.bv_len;
    bvec_iter_advance_single(iter.bvecs, &iter.iter, next.bv_len);
    }
    vec.len = bv.bv_len;
    return true;
    }
//
// The IOVA-based DMA API wants to be able to coalesce at the minimal IOMMU page
// size granularity (which is guaranteed to be <= PAGE_SIZE and usually 4k), so
// we need to ensure our segments are aligned to this as well.
//
// Note that there is no point in using the slightly more complicated IOVA based
// path for single segment mappings.
//
#[no_mangle]
pub unsafe extern "C" fn blk_can_dma_map_iova(req: *mut request, dma_dev: *mut device) -> bool {
    return !(req_phys_gap_mask(req) & dma_get_merge_boundary(dma_dev));
    }
#[no_mangle]
unsafe extern "C" fn blk_dma_map_bus(iter: *mut blk_dma_iter, vec: *mut phys_vec) -> bool {
    iter.addr = pci_p2pdma_bus_addr_map(iter.p2pdma.mem, vec.paddr);
    iter.len = vec.len;
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_dma_map_direct(req: *mut request, dma_dev: *mut device, iter: *mut blk_dma_iter, vec: *mut phys_vec) -> bool {
pub static mut attrs: c_uint = 0;
    if (iter.p2pdma.map == PCI_P2PDMA_MAP_THRU_HOST_BRIDGE) {
    attrs |= DMA_ATTR_MMIO;
    }
    iter.addr = dma_map_phys(dma_dev, vec.paddr, vec.len,
    rq_dma_dir(req), attrs);
    if (dma_mapping_error(dma_dev, iter.addr)) {
    iter.status = BLK_STS_RESOURCE;
    return false;
    }
    iter.len = vec.len;
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_rq_dma_map_iova(req: *mut request, dma_dev: *mut device, state: *mut dma_iova_state, iter: *mut blk_dma_iter, vec: *mut phys_vec) -> bool {
pub static mut dir: dma_data_direction = 0;
pub static mut attrs: c_uint = 0;
pub static mut mapped: usize = 0;
    let mut error = 0;
    iter.addr = state.addr;
    iter.len = dma_iova_size(state);
    if (iter.p2pdma.map == PCI_P2PDMA_MAP_THRU_HOST_BRIDGE) {
    attrs |= DMA_ATTR_MMIO;
    }
    do {
    error = dma_iova_link(dma_dev, state, vec.paddr, mapped,
    vec.len, dir, attrs);
    if (error) {
// goto;
    }
    mapped += vec.len;
    } while (blk_map_iter_next(req, &iter.iter, vec));
    error = dma_iova_sync(dma_dev, state, 0, mapped);
    if (error) {
// goto;
    }
    return true;
// label;
    dma_iova_destroy(dma_dev, state, mapped, dir, attrs);
    iter.status = errno_to_blk_status(error);
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_rq_map_iter_init(rq: *mut request, iter: *mut blk_map_iter) {
    let mut bio = rq.bio;
    if (rq.rq_flags & RQF_SPECIAL_PAYLOAD) {
// iter = (blk_map_iter) {
    .bvecs = &rq.special_vec,
    .iter = {
    .bi_size = rq.special_vec.bv_len,
    }
    };
    } else if (bio) {
// iter = (blk_map_iter) {
    .bio = bio,
    .bvecs = bio.bi_io_vec,
    .iter = bio.bi_iter,
    };
    } else {
// the internal flush request may not have bio attached
// iter = (blk_map_iter) {};
    }
    }
#[no_mangle]
pub unsafe extern "C" fn blk_dma_map_iter_start(req: *mut request, dma_dev: *mut device, state: *mut dma_iova_state, iter: *mut blk_dma_iter, total_len: c_uint) -> bool {
pub static mut vec: usize = 0;
    memset(&iter.p2pdma, 0, sizeof!(iter.p2pdma));
    iter.status = BLK_STS_OK;
    iter.p2pdma.map = PCI_P2PDMA_MAP_NONE;
//
// Grab the first segment ASAP because we'll need it to check for P2P
// transfers.
//
    if (!blk_map_iter_next(req, &iter.iter, &vec)) {
    return false;
    }
    switch (pci_p2pdma_state(&iter.p2pdma, dma_dev,
    phys_to_page(vec.paddr))) {
    case PCI_P2PDMA_MAP_BUS_ADDR:
    return blk_dma_map_bus(iter, &vec);
    case PCI_P2PDMA_MAP_THRU_HOST_BRIDGE:
//
// P2P transfers through the host bridge are treated the
// same as non-P2P transfers below and during unmap.
//
    case PCI_P2PDMA_MAP_NONE:
    break;
// label;
    iter.status = BLK_STS_INVAL;
    return false;
    }
    if (blk_can_dma_map_iova(req, dma_dev) &&
    dma_iova_try_alloc(dma_dev, state, vec.paddr, total_len)) {
    return blk_rq_dma_map_iova(req, dma_dev, state, iter, &vec);
    }
    memset(state, 0, sizeof!(*state));
    return blk_dma_map_direct(req, dma_dev, iter, &vec);
    }
//
// blk_rq_dma_map_iter_start - map the first DMA segment for a request
// @req:	request to map
// @dma_dev:	device to map to
// @state:	DMA IOVA state
// @iter:	block layer DMA iterator
//
// Start DMA mapping @req to @dma_dev.  @state and @iter are provided by the
// caller and don't need to be initialized.  @state needs to be stored for use
// at unmap time, @iter is only needed at map time.
//
// Returns %false if there is no segment to map, including due to an error, or
// %true ft it did map a segment.
//
// If a segment was mapped, the DMA address for it is returned in @iter.addr and
// the length in @iter.len.  If no segment was mapped the status code is
// returned in @iter.status.
//
// The caller can call blk_rq_dma_map_coalesce() to check if further segments
// need to be mapped after this, or go straight to blk_rq_dma_map_iter_next()
// to try to map the following segments.
//
#[no_mangle]
pub unsafe extern "C" fn blk_rq_dma_map_iter_start(req: *mut request, dma_dev: *mut device, state: *mut dma_iova_state, iter: *mut blk_dma_iter) -> bool {
    blk_rq_map_iter_init(req, &iter.iter);
    return blk_dma_map_iter_start(req, dma_dev, state, iter,
    blk_rq_payload_bytes(req));
    }
    EXPORT_SYMBOL_GPL(blk_rq_dma_map_iter_start);
//
// blk_rq_dma_map_iter_next - map the next DMA segment for a request
// @req:	request to map
// @dma_dev:	device to map to
// @iter:	block layer DMA iterator
//
// Iterate to the next mapping after a previous call to
// blk_rq_dma_map_iter_start().  See there for a detailed description of the
// arguments.
//
// Returns %false if there is no segment to map, including due to an error, or
// %true ft it did map a segment.
//
// If a segment was mapped, the DMA address for it is returned in @iter.addr and
// the length in @iter.len.  If no segment was mapped the status code is
// returned in @iter.status.
//
#[no_mangle]
pub unsafe extern "C" fn blk_rq_dma_map_iter_next(req: *mut request, dma_dev: *mut device, iter: *mut blk_dma_iter) -> bool {
pub static mut vec: usize = 0;
    if (!blk_map_iter_next(req, &iter.iter, &vec)) {
    return false;
    }
    if (iter.p2pdma.map == PCI_P2PDMA_MAP_BUS_ADDR) {
    return blk_dma_map_bus(iter, &vec);
    }
    return blk_dma_map_direct(req, dma_dev, iter, &vec);
    }
    EXPORT_SYMBOL_GPL(blk_rq_dma_map_iter_next);
#[no_mangle]
pub unsafe extern "C" fn blk_next_sg(sg: *mut *mut scatterlist, sglist: *mut scatterlist) -> *mut c_void {
    if (!*sg) {
    return sglist;
    }
//
// If the driver previously mapped a shorter list, we could see a
// termination bit prematurely unless it fully inits the sg table
// on each mapping. We KNOW that there must be more entries here
// or the driver would be buggy, so force clear the termination bit
// to avoid doing a full sg_init_table() in drivers for each command.
//
    sg_unmark_end(*sg);
    return sg_next(*sg);
    }
//
// Map a request to scatterlist, return number of sg entries setup. Caller
// must make sure sg can hold rq->nr_phys_segments entries.
//
#[no_mangle]
pub unsafe extern "C" fn __blk_rq_map_sg(rq: *mut request, sglist: *mut scatterlist, last_sg: *mut *mut scatterlist) -> c_int {
pub static mut iter: usize = 0;
pub static mut vec: usize = 0;
pub static mut nsegs: c_int = 0;
    blk_rq_map_iter_init(rq, &iter);
    while (blk_map_iter_next(rq, &iter, &vec)) {
// last_sg = blk_next_sg(last_sg, sglist);
    WARN_ON_ONCE!(overflows_type(vec.len, unsigned int));
    sg_set_page(*last_sg, phys_to_page(vec.paddr), vec.len,
    offset_in_page(vec.paddr));
    nsegs += 1;
    }
    if (*last_sg) {
    sg_mark_end(*last_sg);
    }
//
// Something must have been wrong if the figured number of
// segment is bigger than number of req's physical segments
//
    WARN_ON!(nsegs > blk_rq_nr_phys_segments(rq));
    return nsegs;
    }
    EXPORT_SYMBOL(__blk_rq_map_sg);

//
// blk_rq_integrity_dma_map_iter_start - map the first integrity DMA segment
// for a request
// @req:	request to map
// @dma_dev:	device to map to
// @state:	DMA IOVA state
// @iter:	block layer DMA iterator
//
// Start DMA mapping @req integrity data to @dma_dev.  @state and @iter are
// provided by the caller and don't need to be initialized.  @state needs to be
// stored for use at unmap time, @iter is only needed at map time.
//
// Returns %false if there is no segment to map, including due to an error, or
// %true if it did map a segment.
//
// If a segment was mapped, the DMA address for it is returned in @iter.addr
// and the length in @iter.len.  If no segment was mapped the status code is
// returned in @iter.status.
//
// The caller can call blk_rq_dma_map_coalesce() to check if further segments
// need to be mapped after this, or go straight to blk_rq_dma_map_iter_next()
// to try to map the following segments.
//
#[no_mangle]
pub unsafe extern "C" fn blk_rq_integrity_dma_map_iter_start(req: *mut request, dma_dev: *mut device, state: *mut dma_iova_state, iter: *mut blk_dma_iter) -> bool {
    unsigned len = bio_integrity_bytes(&req.q.limits.integrity,
    blk_rq_sectors(req));
    let mut bio = req.bio;
    iter.iter = (blk_map_iter) {
    .bio = bio,
    .iter = bio_integrity(bio).bip_iter,
    .bvecs = bio_integrity(bio).bip_vec,
    .is_integrity = true,
    };
    return blk_dma_map_iter_start(req, dma_dev, state, iter, len);
    }
    EXPORT_SYMBOL_GPL(blk_rq_integrity_dma_map_iter_start);
//
// blk_rq_integrity_dma_map_iter_next - map the next integrity DMA segment for
// a request
// @req:	request to map
// @dma_dev:	device to map to
// @state:	DMA IOVA state
// @iter:	block layer DMA iterator
//
// Iterate to the next integrity mapping after a previous call to
// blk_rq_integrity_dma_map_iter_start().  See there for a detailed description
// of the arguments.
//
// Returns %false if there is no segment to map, including due to an error, or
// %true if it did map a segment.
//
// If a segment was mapped, the DMA address for it is returned in @iter.addr and
// the length in @iter.len.  If no segment was mapped the status code is
// returned in @iter.status.
//
#[no_mangle]
pub unsafe extern "C" fn blk_rq_integrity_dma_map_iter_next(req: *mut request, dma_dev: *mut device, iter: *mut blk_dma_iter) -> bool {
pub static mut vec: usize = 0;
    if (!blk_map_iter_next(req, &iter.iter, &vec)) {
    return false;
    }
    if (iter.p2pdma.map == PCI_P2PDMA_MAP_BUS_ADDR) {
    return blk_dma_map_bus(iter, &vec);
    }
    return blk_dma_map_direct(req, dma_dev, iter, &vec);
    }
    EXPORT_SYMBOL_GPL(blk_rq_integrity_dma_map_iter_next);
//
// blk_rq_map_integrity_sg - Map integrity metadata into a scatterlist
// @rq:		request to map
// @sglist:	target scatterlist
//
// Description: Map the integrity vectors in request into a
// scatterlist.  The scatterlist must be big enough to hold all
// elements.  I.e. sized using blk_rq_count_integrity_sg() or
// rq->nr_integrity_segments.
//
#[no_mangle]
pub unsafe extern "C" fn blk_rq_map_integrity_sg(rq: *mut request, sglist: *mut scatterlist) -> c_int {
    let mut q = rq.q;
    let mut sg = core::ptr::null_mut();
    let mut bio = rq.bio;
pub static mut segments: c_uint = 0;
pub static mut vec: usize = 0;
pub static mut blk_map_iter: usize = 0;
    while (blk_map_iter_next(rq, &iter, &vec)) {
    sg = blk_next_sg(&sg, sglist);
    WARN_ON_ONCE!(overflows_type(vec.len, unsigned int));
    sg_set_page(sg, phys_to_page(vec.paddr), vec.len,
    offset_in_page(vec.paddr));
    segments += 1;
    }
    if (sg) {
    sg_mark_end(sg);
    }
//
// Something must have been wrong if the figured number of segment
// is bigger than number of req's physical integrity segments
//
    BUG_ON!(segments > rq.nr_integrity_segments);
    BUG_ON!(segments > queue_max_integrity_segments(q));
    return segments;
    }
    EXPORT_SYMBOL(blk_rq_map_integrity_sg);