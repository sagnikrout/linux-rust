//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/vdso.c
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
// Copyright (C) 2004 Benjamin Herrenschmidt, IBM Corp.
// <benh@kernel.crashing.org>
//

    static_assert(__VDSO_PAGES == VDSO_NR_PAGES);
// The alignment of the vDSO

    extern char vdso32_start, vdso32_end;
    extern char vdso64_start, vdso64_end;
    static int vdso_mremap(const struct vm_special_mapping *sm, struct vm_area_struct *new_vma,
    unsigned long text_size)
    {
    let mut new_size: c_ulong = new_vma.vm_end - new_vma.vm_start;
    if (new_size != text_size)
    return -EINVAL;
    current.mm.context.vdso = (void __user *)new_vma.vm_start;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vdso32_mremap(sm: *const vm_special_mapping, new_vma: *mut vm_area_struct) -> c_int {
    static int vdso32_mremap(const struct vm_special_mapping *sm, struct vm_area_struct *new_vma)
    {
    return vdso_mremap(sm, new_vma, &vdso32_end - &vdso32_start);
    }
#[no_mangle]
unsafe extern "C" fn vdso64_mremap(sm: *const vm_special_mapping, new_vma: *mut vm_area_struct) -> c_int {
    static int vdso64_mremap(const struct vm_special_mapping *sm, struct vm_area_struct *new_vma)
    {
    return vdso_mremap(sm, new_vma, &vdso64_end - &vdso64_start);
    }
#[no_mangle]
unsafe extern "C" fn vdso_close(sm: *const vm_special_mapping, vma: *mut vm_area_struct) {
    static void vdso_close(const struct vm_special_mapping *sm, struct vm_area_struct *vma)
    {
    struct mm_struct *mm = vma.vm_mm;
//
// close() is called for munmap() but also for mremap(). In the mremap()
// case the vdso pointer has already been updated by the mremap() hook
// above, so it must not be set to NULL here.
//
    if (vma.vm_start != (unsigned long)mm.context.vdso)
    return;
    mm.context.vdso = core::ptr::null_mut();
    }
    static struct vm_special_mapping vdso32_spec __ro_after_init = {
    .name = "[vdso]",
    .mremap = vdso32_mremap,
    .close = vdso_close,
    };
    static struct vm_special_mapping vdso64_spec __ro_after_init = {
    .name = "[vdso]",
    .mremap = vdso64_mremap,
    .close = vdso_close,
    };
//
// This is called from binfmt_elf, we create the special vma for the
// vDSO and insert it into the mm struct tree
//
#[no_mangle]
unsafe extern "C" fn __arch_setup_additional_pages(bprm: *mut linux_binprm, uses_interp: c_int) -> c_int {
    static int __arch_setup_additional_pages(struct linux_binprm *bprm, int uses_interp)
    {
    unsigned long vdso_size, vdso_base, mappings_size;
    struct vm_special_mapping *vdso_spec;
    let mut vvar_size: c_ulong = VDSO_NR_PAGES * PAGE_SIZE;
    struct mm_struct *mm = current.mm;
    struct vm_area_struct *vma;
    if (is_32bit_task()) {
    vdso_spec = &vdso32_spec;
    vdso_size = &vdso32_end - &vdso32_start;
    } else {
    vdso_spec = &vdso64_spec;
    vdso_size = &vdso64_end - &vdso64_start;
    }
    mappings_size = vdso_size + vvar_size;
    mappings_size += (VDSO_ALIGNMENT - 1) & PAGE_MASK;
//
// Pick a base address for the vDSO in process space.
// Add enough to the size so that the result can be aligned.
//
    vdso_base = get_unmapped_area(core::ptr::null_mut(), 0, mappings_size, 0, 0);
    if (IS_ERR_VALUE(vdso_base))
    return vdso_base;
// Add required alignment.
    vdso_base = ALIGN(vdso_base, VDSO_ALIGNMENT);
    vma = vdso_install_vvar_mapping(mm, vdso_base);
    if (IS_ERR(vma))
    return PTR_ERR(vma);
//
// our vma flags don't have VM_WRITE so by default, the process isn't
// allowed to write those pages.
// gdb can break that with ptrace interface, and thus trigger COW on
// those pages but it's then your responsibility to never do that on
// the "data" page of the vDSO or you'll stop getting kernel updates
// and your nice userland gettimeofday will be totally dead.
// It's fine to use that for setting breakpoints in the vDSO code
// pages though.
//
    vma = _install_special_mapping(mm, vdso_base + vvar_size, vdso_size,
    VM_READ | VM_EXEC | VM_MAYREAD |
    VM_MAYWRITE | VM_MAYEXEC, vdso_spec);
    if (IS_ERR(vma)) {
    do_munmap(mm, vdso_base, vvar_size, core::ptr::null_mut());
    return PTR_ERR(vma);
    }
// Now that the mappings are in place, set the mm VDSO pointer
    mm.context.vdso = (void __user *)vdso_base + vvar_size;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_setup_additional_pages(bprm: *mut linux_binprm, uses_interp: c_int) -> c_int {
    int arch_setup_additional_pages(struct linux_binprm *bprm, int uses_interp)
    {
    struct mm_struct *mm = current.mm;
    int rc;
    mm.context.vdso = core::ptr::null_mut();
    if (mmap_write_lock_killable(mm))
    return -EINTR;
    rc = __arch_setup_additional_pages(bprm, uses_interp);
    mmap_write_unlock(mm);
    return rc;
    }

    void *__start = (void *)VDSO##bits##_SYMBOL(&vdso##bits##_start, sec##_start);	\
    void *__end = (void *)VDSO##bits##_SYMBOL(&vdso##bits##_start, sec##_end);	\
    \
    do_##type##_fixups((value), __start, __end);					\
    } while (0)
#[no_mangle]
unsafe extern "C" fn vdso_fixup_features() -> void __init {
    static void __init vdso_fixup_features(void)
    {

    VDSO_DO_FIXUPS(feature, cur_cpu_spec.cpu_features, 64, ftr_fixup);
    VDSO_DO_FIXUPS(feature, cur_cpu_spec.mmu_features, 64, mmu_ftr_fixup);
    VDSO_DO_FIXUPS(feature, powerpc_firmware_features, 64, fw_ftr_fixup);
    VDSO_DO_FIXUPS(lwsync, cur_cpu_spec.cpu_features, 64, lwsync_fixup);

    VDSO_DO_FIXUPS(feature, cur_cpu_spec.cpu_features, 32, ftr_fixup);
    VDSO_DO_FIXUPS(feature, cur_cpu_spec.mmu_features, 32, mmu_ftr_fixup);

    VDSO_DO_FIXUPS(feature, powerpc_firmware_features, 32, fw_ftr_fixup);

    VDSO_DO_FIXUPS(lwsync, cur_cpu_spec.cpu_features, 32, lwsync_fixup);

    }
//
// Called from setup_arch to initialize the bitmap of available
// syscalls in the systemcfg page
//
#[no_mangle]
unsafe extern "C" fn vdso_setup_syscall_map() -> void __init {
    static void __init vdso_setup_syscall_map(void)
    {
    unsigned int i;
    for (i = 0; i < NR_syscalls; i++) {
    if (sys_call_table[i] != (void *)&sys_ni_syscall)
    vdso_k_arch_data.syscall_map[i >> 5] |= 0x80000000UL >> (i & 0x1f);
    if (IS_ENABLED(CONFIG_COMPAT) &&
    compat_sys_call_table[i] != (void *)&sys_ni_syscall)
    vdso_k_arch_data.compat_syscall_map[i >> 5] |= 0x80000000UL >> (i & 0x1f);
    }
    }

#[no_mangle]
pub unsafe extern "C" fn vdso_getcpu_init() -> c_int {
    int vdso_getcpu_init(void)
    {
    unsigned long cpu, node, val;
//
// SPRG_VDSO contains the CPU in the bottom 16 bits and the NUMA node
// in the next 16 bits.  The VDSO uses this to implement getcpu().
//
    cpu = get_cpu();
    WARN_ON_ONCE(cpu > 0xffff);
    node = cpu_to_node(cpu);
    WARN_ON_ONCE(node > 0xffff);
    val = (cpu & 0xffff) | ((node & 0xffff) << 16);
    mtspr(SPRN_SPRG_VDSO_WRITE, val);
    get_paca().sprg_vdso = val;
    put_cpu();
    return 0;
    }
// We need to call this before SMP init
    early_initcall(vdso_getcpu_init);

#[no_mangle]
unsafe extern "C" fn vdso_setup_pages(start: *mut c_void, end: *mut c_void) -> *mut *mut page  __init {
    static struct page ** __init vdso_setup_pages(void *start, void *end)
    {
    int i;
    struct page **pagelist;
    let mut pages: c_int = (end - start) >> PAGE_SHIFT;
    pagelist = kzalloc_objs(struct page *, pages + 1);
    if (!pagelist)
    panic("%s: Cannot allocate page list for VDSO", __func__);
    for (i = 0; i < pages; i++)
    pagelist[i] = virt_to_page(start + i * PAGE_SIZE);
    return pagelist;
    }
#[no_mangle]
unsafe extern "C" fn vdso_init() -> int __init {
    static int __init vdso_init(void)
    {

    vdso_k_arch_data.dcache_block_size = ppc64_caches.l1d.block_size;
    vdso_k_arch_data.icache_block_size = ppc64_caches.l1i.block_size;
    vdso_k_arch_data.dcache_log_block_size = ppc64_caches.l1d.log_block_size;
    vdso_k_arch_data.icache_log_block_size = ppc64_caches.l1i.log_block_size;

    vdso_setup_syscall_map();
    vdso_fixup_features();
    if (IS_ENABLED(CONFIG_VDSO32))
    vdso32_spec.pages = vdso_setup_pages(&vdso32_start, &vdso32_end);
    if (IS_ENABLED(CONFIG_PPC64))
    vdso64_spec.pages = vdso_setup_pages(&vdso64_start, &vdso64_end);
    smp_wmb();
    return 0;
    }
    arch_initcall(vdso_init);
