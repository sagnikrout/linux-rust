//! Automatically rewritten from C to Rust
//! Source: arch/x86/entry/vdso/vma.c
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
// Copyright 2007 Andi Kleen, SUSE Labs.
//
// This contains most of the x86 vDSO kernel-side code.
//

    static_assert(VDSO_NR_PAGES + VDSO_NR_VCLOCK_PAGES == __VDSO_PAGES);
    unsigned int vclocks_used __read_mostly;

    let mut vdso64_enabled: unsigned int __read_mostly = 1;

#[no_mangle]
pub unsafe extern "C" fn init_vdso_image(image: *const vdso_image) -> int __init {
    int __init init_vdso_image(const struct vdso_image *image)
    {
    BUILD_BUG_ON(VDSO_CLOCKMODE_MAX >= 32);
    BUG_ON(image.size % PAGE_SIZE != 0);
    apply_alternatives((struct alt_instr *)(image.data + image.alt),
    (struct alt_instr *)(image.data + image.alt +
    image.alt_len));
    return 0;
    }
    struct linux_binprm;
    static vm_fault_t vdso_fault(const struct vm_special_mapping *sm,
    struct vm_area_struct *vma, struct vm_fault *vmf)
    {
    const struct vdso_image *image = vma.vm_mm.context.vdso_image;
    if (!image || (vmf.pgoff << PAGE_SHIFT) >= image.size)
    return VM_FAULT_SIGBUS;
    vmf.page = virt_to_page(image.data + (vmf.pgoff << PAGE_SHIFT));
    get_page(vmf.page);
    return 0;
    }
    static void vdso_fix_landing(const struct vdso_image *image,
    struct vm_area_struct *new_vma)
    {
    struct pt_regs *regs = current_pt_regs();
    unsigned long ipoffset = regs.ip -
    (unsigned long)current.mm.context.vdso;
    if (ipoffset < image.size)
    regs.ip = new_vma.vm_start + ipoffset;
    }

#[no_mangle]
unsafe extern "C" fn vdso_futex_robust_unlock_update_ips() {
    static void vdso_futex_robust_unlock_update_ips(void)
    {
    const struct vdso_image *image = current.mm.context.vdso_image;
    let mut vdso: c_ulong = (unsigned long) current.mm.context.vdso;
    struct futex_mm_data *fd = &current.mm.futex;
    let mut idx: c_uint = 0;
    futex_reset_cs_ranges(fd);

    futex_set_vdso_cs_range(fd, idx, vdso + image.sym___futex_list64_try_unlock_cs_start,
    vdso + image.sym___futex_list64_try_unlock_cs_end, false);
    idx++;

    futex_set_vdso_cs_range(fd, idx, vdso + image.sym___futex_list32_try_unlock_cs_start,
    vdso + image.sym___futex_list32_try_unlock_cs_end, true);

    }

    static inline void vdso_futex_robust_unlock_update_ips(void) { }

    static int vdso_mremap(const struct vm_special_mapping *sm,
    struct vm_area_struct *new_vma)
    {
    const struct vdso_image *image = current.mm.context.vdso_image;
    vdso_fix_landing(image, new_vma);
    current.mm.context.vdso = (void __user *)new_vma.vm_start;
    vdso_futex_robust_unlock_update_ips();
    return 0;
    }
    static vm_fault_t vvar_vclock_fault(const struct vm_special_mapping *sm,
    struct vm_area_struct *vma, struct vm_fault *vmf)
    {
    switch (vmf.pgoff) {
    case VDSO_PAGE_PVCLOCK_OFFSET:
    {
    struct pvclock_vsyscall_time_info *pvti =
    pvclock_get_pvti_cpu0_va();
    if (pvti && vclock_was_used(VDSO_CLOCKMODE_PVCLOCK))
    return vmf_insert_pfn_prot(vma, vmf.address,
    __pa(pvti) >> PAGE_SHIFT,
    pgprot_decrypted(vma.vm_page_prot));
    break;
    }
    case VDSO_PAGE_HVCLOCK_OFFSET:
    {
    let mut pfn: c_ulong = hv_get_tsc_pfn();
    if (pfn && vclock_was_used(VDSO_CLOCKMODE_HVCLOCK))
    return vmf_insert_pfn(vma, vmf.address, pfn);
    break;
    }
    }
    return VM_FAULT_SIGBUS;
    }
    static const struct vm_special_mapping vdso_mapping = {
    .name = "[vdso]",
    .fault = vdso_fault,
    .mremap = vdso_mremap,
    };
    static const struct vm_special_mapping vvar_vclock_mapping = {
    .name = "[vvar_vclock]",
    .fault = vvar_vclock_fault,
    };
//
// Add vdso and vvar mappings to current process.
// @image          - blob to map
// @addr           - request a specific address (zero to map at free addr)
//
#[no_mangle]
unsafe extern "C" fn map_vdso(image: *const vdso_image, addr: c_ulong) -> c_int {
    static int map_vdso(const struct vdso_image *image, unsigned long addr)
    {
    struct mm_struct *mm = current.mm;
    struct vm_area_struct *vma;
    unsigned long text_start;
    let mut ret: c_int = 0;
    if (mmap_write_lock_killable(mm))
    return -EINTR;
    addr = get_unmapped_area(core::ptr::null_mut(), addr,
    image.size + __VDSO_PAGES * PAGE_SIZE, 0, 0);
    if (IS_ERR_VALUE(addr)) {
    ret = addr;
    goto up_fail;
    }
    text_start = addr + __VDSO_PAGES * PAGE_SIZE;
//
// MAYWRITE to allow gdb to COW and set breakpoints
//
    vma = _install_special_mapping(mm,
    text_start,
    image.size,
    VM_READ|VM_EXEC|
    VM_MAYREAD|VM_MAYWRITE|VM_MAYEXEC|
    VM_SEALED_SYSMAP,
    &vdso_mapping);
    if (IS_ERR(vma)) {
    ret = PTR_ERR(vma);
    goto up_fail;
    }
    vma = vdso_install_vvar_mapping(mm, addr);
    if (IS_ERR(vma)) {
    ret = PTR_ERR(vma);
    do_munmap(mm, text_start, image.size, core::ptr::null_mut());
    goto up_fail;
    }
    vma = _install_special_mapping(mm,
    VDSO_VCLOCK_PAGES_START(addr),
    VDSO_NR_VCLOCK_PAGES * PAGE_SIZE,
    VM_READ|VM_MAYREAD|VM_IO|VM_DONTDUMP|
    VM_PFNMAP|VM_SEALED_SYSMAP,
    &vvar_vclock_mapping);
    if (IS_ERR(vma)) {
    ret = PTR_ERR(vma);
    do_munmap(mm, text_start, image.size, core::ptr::null_mut());
    do_munmap(mm, addr, VDSO_NR_PAGES * PAGE_SIZE, core::ptr::null_mut());
    goto up_fail;
    }
    current.mm.context.vdso = (void __user *)text_start;
    current.mm.context.vdso_image = image;
    vdso_futex_robust_unlock_update_ips();
    up_fail:
    mmap_write_unlock(mm);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn map_vdso_once(image: *const vdso_image, addr: c_ulong) -> c_int {
    int map_vdso_once(const struct vdso_image *image, unsigned long addr)
    {
    struct mm_struct *mm = current.mm;
    struct vm_area_struct *vma;
    VMA_ITERATOR(vmi, mm, 0);
    mmap_write_lock(mm);
//
// Check if we have already mapped vdso blob - fail to prevent
// abusing from userspace install_special_mapping, which may
// not do accounting and rlimit right.
// We could search vma near context.vdso, but it's a slowpath,
// so let's explicitly check all VMAs to be completely sure.
//
    for_each_vma(vmi, vma) {
    if (vma_is_special_mapping(vma, &vdso_mapping) ||
    vma_is_special_mapping(vma, &vdso_vvar_mapping) ||
    vma_is_special_mapping(vma, &vvar_vclock_mapping)) {
    mmap_write_unlock(mm);
    return -EEXIST;
    }
    }
    mmap_write_unlock(mm);
    return map_vdso(image, addr);
    }
#[no_mangle]
unsafe extern "C" fn load_vdso32() -> c_int {
    static int load_vdso32(void)
    {
    if (vdso32_enabled != 1)  /* Other values all mean "disabled" */
    return 0;
    return map_vdso(&vdso32_image, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_setup_additional_pages(bprm: *mut linux_binprm, uses_interp: c_int) -> c_int {
    int arch_setup_additional_pages(struct linux_binprm *bprm, int uses_interp)
    {
    if (IS_ENABLED(CONFIG_X86_64)) {
    if (!vdso64_enabled)
    return 0;
    return map_vdso(&vdso64_image, 0);
    }
    return load_vdso32();
    }

    int compat_arch_setup_additional_pages(struct linux_binprm *bprm,
    int uses_interp, bool x32)
    {
    if (IS_ENABLED(CONFIG_X86_X32_ABI) && x32) {
    if (!vdso64_enabled)
    return 0;
    return map_vdso(&vdsox32_image, 0);
    }
    if (IS_ENABLED(CONFIG_IA32_EMULATION))
    return load_vdso32();
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn arch_syscall_is_vdso_sigreturn(regs: *mut pt_regs) -> bool {
    bool arch_syscall_is_vdso_sigreturn(struct pt_regs *regs)
    {
    const struct vdso_image *image = current.mm.context.vdso_image;
    let mut vdso: c_ulong = (unsigned long) current.mm.context.vdso;
    if (in_ia32_syscall() && image == &vdso32_image) {
    if (regs.ip == vdso + image.sym_vdso32_sigreturn_landing_pad ||
    regs.ip == vdso + image.sym_vdso32_rt_sigreturn_landing_pad)
    return true;
    }
    return false;
    }

#[no_mangle]
unsafe extern "C" fn vdso_setup(s: *mut c_char) -> __init int {
    static __init int vdso_setup(char *s)
    {
    vdso64_enabled = simple_strtoul(s, core::ptr::null_mut(), 0);
    return 1;
    }
    __setup("vdso=", vdso_setup);
