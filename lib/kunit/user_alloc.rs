//! Automatically rewritten from C to Rust
//! Source: lib/kunit/user_alloc.c
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
// KUnit userspace memory allocation resource management.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kunit_vm_mmap_resource {
    pub addr: c_ulong,
    pub size: usize,
}

// vm_mmap() arguments
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kunit_vm_mmap_params {
    pub file: *mut file,
    pub addr: c_ulong,
    pub len: c_ulong,
    pub prot: c_ulong,
    pub flag: c_ulong,
    pub offset: c_ulong,
}

#[no_mangle]
pub unsafe extern "C" fn kunit_attach_mm() -> c_int {
    int kunit_attach_mm(void)
    {
    struct mm_struct *mm;
    if (current.mm)
    return 0;
// arch_pick_mmap_layout() is only sane with MMU systems.
    if (!IS_ENABLED(CONFIG_MMU))
    return -EINVAL;
    mm = mm_alloc();
    if (!mm)
    return -ENOMEM;
// Define the task size.
    mm.task_size = TASK_SIZE;
// Make sure we can allocate new VMAs.
    arch_pick_mmap_layout(mm, &current.signal.rlim[RLIMIT_STACK]);
// Attach the mm. It will be cleaned up when the process dies.
    kthread_use_mm(mm);
    return 0;
    }
    EXPORT_SYMBOL_GPL(kunit_attach_mm);
#[no_mangle]
unsafe extern "C" fn kunit_vm_mmap_init(res: *mut kunit_resource, context: *mut c_void) -> c_int {
    static int kunit_vm_mmap_init(struct kunit_resource *res, void *context)
    {
    struct kunit_vm_mmap_params *p = context;
    struct kunit_vm_mmap_resource vres;
    int ret;
    ret = kunit_attach_mm();
    if (ret)
    return ret;
    vres.size = p.len;
    vres.addr = vm_mmap(p.file, p.addr, p.len, p.prot, p.flag, p.offset);
    if (!vres.addr)
    return -ENOMEM;
    res.data = kmemdup(&vres, sizeof(vres), GFP_KERNEL);
    if (!res.data) {
    vm_munmap(vres.addr, vres.size);
    return -ENOMEM;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kunit_vm_mmap_free(res: *mut kunit_resource) {
    static void kunit_vm_mmap_free(struct kunit_resource *res)
    {
    struct kunit_vm_mmap_resource *vres = res.data;
//
// Since this is executed from the test monitoring process,
// the test's mm has already been torn down. We don't need
// to run vm_munmap(vres->addr, vres->size), only clean up
// the vres.
//
    kfree(vres);
    res.data = core::ptr::null_mut();
    }
    unsigned long kunit_vm_mmap(struct kunit *test, struct file *file,
    unsigned long addr, unsigned long len,
    unsigned long prot, unsigned long flag,
    unsigned long offset)
    {
    struct kunit_vm_mmap_params params = {
    .file = file,
    .addr = addr,
    .len = len,
    .prot = prot,
    .flag = flag,
    .offset = offset,
    };
    struct kunit_vm_mmap_resource *vres;
    vres = kunit_alloc_resource(test,
    kunit_vm_mmap_init,
    kunit_vm_mmap_free,
    GFP_KERNEL,
    &params);
    if (vres)
    return vres.addr;
    return 0;
    }
    EXPORT_SYMBOL_GPL(kunit_vm_mmap);
    MODULE_IMPORT_NS("EXPORTED_FOR_KUNIT_TESTING");
