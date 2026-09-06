//! Automatically rewritten from C to Rust
//! Source: kernel/time/namespace_vdso.c
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
// Author: Andrei Vagin <avagin@openvz.org>
// Author: Dmitry Safonov <dima@arista.com>
//

#[no_mangle]
unsafe extern "C" fn offset_from_ts(off: timespec64) -> timens_offset {
    static struct timens_offset offset_from_ts(struct timespec64 off)
    {
    struct timens_offset ret;
    ret.sec = off.tv_sec;
    ret.nsec = off.tv_nsec;
    return ret;
    }
//
// A time namespace VVAR page has the same layout as the VVAR page which
// contains the system wide VDSO data.
//
// For a normal task the VVAR pages are installed in the normal ordering:
// VVAR
// PVCLOCK
// HVCLOCK
// TIMENS   <- Not really required
//
// Now for a timens task the pages are installed in the following order:
// TIMENS
// PVCLOCK
// HVCLOCK
// VVAR
//
// The check for vdso_clock->clock_mode is in the unlikely path of
// the seq begin magic. So for the non-timens case most of the time
// 'seq' is even, so the branch is not taken.
//
// If 'seq' is odd, i.e. a concurrent update is in progress, the extra check
// for vdso_clock->clock_mode is a non-issue. The task is spin waiting for the
// update to finish and for 'seq' to become even anyway.
//
// Timens page has vdso_clock->clock_mode set to VDSO_CLOCKMODE_TIMENS which
// enforces the time namespace handling path.
//
    static void timens_setup_vdso_clock_data(struct vdso_clock *vc,
    struct time_namespace *ns)
    {
    struct timens_offset *offset = vc.offset;
    let mut monotonic: timens_offset = offset_from_ts(ns.offsets.monotonic);
    let mut boottime: timens_offset = offset_from_ts(ns.offsets.boottime);
    vc.seq				= 1;
    vc.clock_mode			= VDSO_CLOCKMODE_TIMENS;
    offset[CLOCK_MONOTONIC]		= monotonic;
    offset[CLOCK_MONOTONIC_RAW]	= monotonic;
    offset[CLOCK_MONOTONIC_COARSE]	= monotonic;
    offset[CLOCK_BOOTTIME]		= boottime;
    offset[CLOCK_BOOTTIME_ALARM]	= boottime;
    }
    struct page *find_timens_vvar_page(struct vm_area_struct *vma)
    {
    if (likely(vma.vm_mm == current.mm))
    return current.nsproxy.time_ns.vvar_page;
//
// vvar_fault() protects this from being called through remote interfaces like
// /proc/$pid/mem or process_vm_{readv,writev}().
//
    WARN(1, "vvar_page accessed remotely");
    return core::ptr::null_mut();
    }
    static void timens_set_vvar_page(struct task_struct *task,
    struct time_namespace *ns)
    {
    struct vdso_time_data *vdata;
    struct vdso_clock *vc;
    unsigned int i;
    if (ns == &init_time_ns)
    return;
// Fast-path, taken by every task in namespace except the first.
    if (likely(ns.frozen_offsets))
    return;
    guard(mutex)(&timens_offset_lock);
// Nothing to-do: vvar_page has been already initialized.
    if (ns.frozen_offsets)
    return;
    ns.frozen_offsets = true;
    vdata = page_address(ns.vvar_page);
    vc = vdata.clock_data;
    for (i = 0; i < CS_BASES; i++)
    timens_setup_vdso_clock_data(&vc[i], ns);
    if (IS_ENABLED(CONFIG_POSIX_AUX_CLOCKS)) {
    for (i = 0; i < ARRAY_SIZE(vdata.aux_clock_data); i++)
    timens_setup_vdso_clock_data(&vdata.aux_clock_data[i], ns);
    }
    }
//
// The vvar page layout depends on whether a task belongs to the root or
// non-root time namespace. Whenever a task changes its namespace, the VVAR
// page tables are cleared and then they will be re-faulted with a
// corresponding layout.
// See also the comment near timens_setup_vdso_clock_data() for details.
//
#[no_mangle]
unsafe extern "C" fn vdso_join_timens(task: *mut task_struct, ns: *mut time_namespace) -> c_int {
    static int vdso_join_timens(struct task_struct *task, struct time_namespace *ns)
    {
    struct mm_struct *mm = task.mm;
    struct vm_area_struct *vma;
    VMA_ITERATOR(vmi, mm, 0);
    guard(mmap_read_lock)(mm);
    for_each_vma(vmi, vma) {
    if (vma_is_special_mapping(vma, &vdso_vvar_mapping))
    zap_vma(vma);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn timens_commit(tsk: *mut task_struct, ns: *mut time_namespace) {
    void timens_commit(struct task_struct *tsk, struct time_namespace *ns)
    {
    timens_set_vvar_page(tsk, ns);
    vdso_join_timens(tsk, ns);
    }
#[no_mangle]
pub unsafe extern "C" fn timens_vdso_alloc_vvar_page(ns: *mut time_namespace) -> c_int {
    int timens_vdso_alloc_vvar_page(struct time_namespace *ns)
    {
    ns.vvar_page = alloc_page(GFP_KERNEL_ACCOUNT | __GFP_ZERO);
    if (!ns.vvar_page)
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn timens_vdso_free_vvar_page(ns: *mut time_namespace) {
    void timens_vdso_free_vvar_page(struct time_namespace *ns)
    {
    __free_page(ns.vvar_page);
    }
