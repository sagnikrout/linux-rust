//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/mm/book3s64/pkeys.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// PowerPC Memory Protection Keys management
//
// Copyright 2017, Ram Pai, IBM Corporation.
//

    int  num_pkey;		/* Max number of pkeys supported */
//
// Keys marked in the reservation list cannot be allocated by  userspace
//
    u32 reserved_allocation_mask __ro_after_init;
// Bits set for the initially allocated keys
    static u32 initial_allocation_mask __ro_after_init;
//
// Even if we allocate keys with sys_pkey_alloc(), we need to make sure
// other thread still find the access denied using the same keys.
//
    let mut __ro_after_init: u64 default_amr = ~0x0UL;
    let mut __ro_after_init: u64 default_iamr = 0x5555555555555555UL;
    u64 default_uamor __ro_after_init;
    EXPORT_SYMBOL(default_amr);
//
// Key used to implement PROT_EXEC mmap. Denies READ/WRITE
// We pick key 2 because 0 is special key and 1 is reserved as per ISA.
//
    let mut execute_only_key: static int = 2;
    static bool pkey_execute_disable_supported;
pub const AMR_BITS_PER_PKEY: c_int = 2;
pub const AMR_RD_BIT: c_uint = 0x1UL;
pub const AMR_WR_BIT: c_uint = 0x2UL;
pub const IAMR_EX_BIT: c_uint = 0x1UL;

    static int __init dt_scan_storage_keys(unsigned long node,
    const char *uname, int depth,
    void *data)
    {
    const char *type = of_get_flat_dt_prop(node, "device_type", core::ptr::null_mut());
    const __be32 *prop;
    int *pkeys_total = (int *) data;
// We are scanning "cpu" nodes only
    if (type == core::ptr::null_mut() || strcmp(type, "cpu") != 0)
    return 0;
    prop = of_get_flat_dt_prop(node, "ibm,processor-storage-keys", core::ptr::null_mut());
    if (!prop)
    return 0;
// pkeys_total = be32_to_cpu(prop[0]);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn scan_pkey_feature() -> int __init {
    static int __init scan_pkey_feature(void)
    {
    int ret;
    let mut pkeys_total: c_int = 0;
//
// Pkey is not supported with Radix translation.
//
    if (early_radix_enabled())
    return 0;
    ret = of_scan_flat_dt(dt_scan_storage_keys, &pkeys_total);
    if (ret == 0) {
//
// Let's assume 32 pkeys on P8/P9 bare metal, if its not defined by device
// tree. We make this exception since some version of skiboot forgot to
// expose this property on power8/9.
//
    if (!firmware_has_feature(FW_FEATURE_LPAR)) {
    let mut pvr: c_ulong = mfspr(SPRN_PVR);
    if (PVR_VER(pvr) == PVR_POWER8 || PVR_VER(pvr) == PVR_POWER8E ||
    PVR_VER(pvr) == PVR_POWER8NVL || PVR_VER(pvr) == PVR_POWER9 ||
    PVR_VER(pvr) == PVR_HX_C2000)
    pkeys_total = 32;
    }
    }

//
// Adjust the upper limit, based on the number of bits supported by
// arch-neutral code.
//
    pkeys_total = min_t(int, pkeys_total,
    ((ARCH_VM_PKEY_FLAGS >> VM_PKEY_SHIFT) + 1));

    return pkeys_total;
    }
#[no_mangle]
pub unsafe extern "C" fn pkey_early_init_devtree() -> void __init {
    void __init pkey_early_init_devtree(void)
    {
    int pkeys_total, i;

//
// We define PKEY_DISABLE_EXECUTE in addition to the arch-neutral
// generic defines for PKEY_DISABLE_ACCESS and PKEY_DISABLE_WRITE.
// Ensure that the bits a distinct.
//
    BUILD_BUG_ON(PKEY_DISABLE_EXECUTE &
    (PKEY_DISABLE_ACCESS | PKEY_DISABLE_WRITE));
//
// pkey_to_vmflag_bits() assumes that the pkey bits are contiguous
// in the vmaflag. Make sure that is really the case.
//
    BUILD_BUG_ON(__builtin_clzl(ARCH_VM_PKEY_FLAGS >> VM_PKEY_SHIFT) +
    __builtin_popcountl(ARCH_VM_PKEY_FLAGS >> VM_PKEY_SHIFT)
    != (sizeof(u64) * BITS_PER_BYTE));

//
// Only P7 and above supports SPRN_AMR update with MSR[PR] = 1
//
    if (!early_cpu_has_feature(CPU_FTR_ARCH_206))
    return;
// scan the device tree for pkey feature
    pkeys_total = scan_pkey_feature();
    if (!pkeys_total)
    goto out;
// Allow all keys to be modified by default
    default_uamor = ~0x0UL;
    cur_cpu_spec.mmu_features |= MMU_FTR_PKEY;
//
// The device tree cannot be relied to indicate support for
// execute_disable support. Instead we use a PVR check.
//
    if (pvr_version_is(PVR_POWER7) || pvr_version_is(PVR_POWER7p))
    pkey_execute_disable_supported = false;
    else
    pkey_execute_disable_supported = true;

//
// The OS can manage only 8 pkeys due to its inability to represent them
// in the Linux 4K PTE. Mark all other keys reserved.
//
    num_pkey = min(8, pkeys_total);

    num_pkey = pkeys_total;

    if (unlikely(num_pkey <= execute_only_key) || !pkey_execute_disable_supported) {
//
// Insufficient number of keys to support
// execute only key. Mark it unavailable.
//
    execute_only_key = -1;
    } else {
//
// Mark the execute_only_pkey as not available for
// user allocation via pkey_alloc.
//
    reserved_allocation_mask |= (0x1 << execute_only_key);
//
// Deny READ/WRITE for execute_only_key.
// Allow execute in IAMR.
//
    default_amr  |= (0x3ul << pkeyshift(execute_only_key));
    default_iamr &= ~(0x1ul << pkeyshift(execute_only_key));
//
// Clear the uamor bits for this key.
//
    default_uamor &= ~(0x3ul << pkeyshift(execute_only_key));
    }
    if (unlikely(num_pkey <= 3)) {
//
// Insufficient number of keys to support
// KUAP/KUEP feature.
//
    disable_kuep = true;
    disable_kuap = true;
    WARN(1, "Disabling kernel user protection due to low (%d) max supported keys\n", num_pkey);
    } else {
// handle key which is used by kernel for KAUP
    reserved_allocation_mask |= (0x1 << 3);
//
// Mark access for kup_key in default amr so that
// we continue to operate with that AMR in
// copy_to/from_user().
//
    default_amr   &= ~(0x3ul << pkeyshift(3));
    default_iamr  &= ~(0x1ul << pkeyshift(3));
    default_uamor &= ~(0x3ul << pkeyshift(3));
    }
//
// Allow access for only key 0. And prevent any other modification.
//
    default_amr   &= ~(0x3ul << pkeyshift(0));
    default_iamr  &= ~(0x1ul << pkeyshift(0));
    default_uamor &= ~(0x3ul << pkeyshift(0));
//
// key 0 is special in that we want to consider it an allocated
// key which is preallocated. We don't allow changing AMR bits
// w.r.t key 0. But one can pkey_free(key0)
//
    initial_allocation_mask |= (0x1 << 0);
//
// key 1 is recommended not to be used. PowerISA(3.0) page 1015,
// programming note.
//
    reserved_allocation_mask |= (0x1 << 1);
    default_uamor &= ~(0x3ul << pkeyshift(1));
//
// Prevent the usage of OS reserved keys. Update UAMOR
// for those keys. Also mark the rest of the bits in the
// 32 bit mask as reserved.
//
    for (i = num_pkey; i < 32 ; i++) {
    reserved_allocation_mask |= (0x1 << i);
    default_uamor &= ~(0x3ul << pkeyshift(i));
    }
//
// Prevent the allocation of reserved keys too.
//
    initial_allocation_mask |= reserved_allocation_mask;
    pr_info("Enabling pkeys with max key count %d\n", num_pkey);
    out:
//
// Setup uamor on boot cpu
//
    mtspr(SPRN_UAMOR, default_uamor);
    return;
    }

#[no_mangle]
pub unsafe extern "C" fn setup_kuep(disabled: bool) {
    void setup_kuep(bool disabled)
    {
    if (disabled)
    return;
//
// On hash if PKEY feature is not enabled, disable KUAP too.
//
    if (!early_radix_enabled() && !early_mmu_has_feature(MMU_FTR_PKEY))
    return;
    if (smp_processor_id() == boot_cpuid) {
    pr_info("Activating Kernel Userspace Execution Prevention\n");
    cur_cpu_spec.mmu_features |= MMU_FTR_BOOK3S_KUEP;
    }
//
// Radix always uses key0 of the IAMR to determine if an access is
// allowed. We set bit 0 (IBM bit 1) of key0, to prevent instruction
// fetch.
//
    mtspr(SPRN_IAMR, AMR_KUEP_BLOCKED);
    isync();
    }

#[no_mangle]
pub unsafe extern "C" fn setup_kuap(disabled: bool) {
    void setup_kuap(bool disabled)
    {
    if (disabled)
    return;
//
// On hash if PKEY feature is not enabled, disable KUAP too.
//
    if (!early_radix_enabled() && !early_mmu_has_feature(MMU_FTR_PKEY))
    return;
    if (smp_processor_id() == boot_cpuid) {
    pr_info("Activating Kernel Userspace Access Prevention\n");
    cur_cpu_spec.mmu_features |= MMU_FTR_KUAP;
    }
//
// Set the default kernel AMR values on all cpus.
//
    mtspr(SPRN_AMR, AMR_KUAP_BLOCKED);
    isync();
    }

#[no_mangle]
pub unsafe extern "C" fn pkey_mm_init(mm: *mut mm_struct) {
    void pkey_mm_init(struct mm_struct *mm)
    {
    if (!mmu_has_feature(MMU_FTR_PKEY))
    return;
    mm_pkey_allocation_map(mm) = initial_allocation_mask;
    mm.context.execute_only_pkey = execute_only_key;
    }
#[no_mangle]
pub unsafe extern "C" fn init_amr(pkey: c_int, init_bits: u8) {
    static inline void init_amr(int pkey, u8 init_bits)
    {
    let mut new_amr_bits: u64 = (((u64)init_bits & 0x3UL) << pkeyshift(pkey));
    let mut old_amr: u64 = current_thread_amr() & ~((u64)(0x3ul) << pkeyshift(pkey));
    current.thread.regs.amr = old_amr | new_amr_bits;
    }
#[no_mangle]
pub unsafe extern "C" fn init_iamr(pkey: c_int, init_bits: u8) {
    static inline void init_iamr(int pkey, u8 init_bits)
    {
    let mut new_iamr_bits: u64 = (((u64)init_bits & 0x1UL) << pkeyshift(pkey));
    let mut old_iamr: u64 = current_thread_iamr() & ~((u64)(0x1ul) << pkeyshift(pkey));
    if (!likely(pkey_execute_disable_supported))
    return;
    current.thread.regs.iamr = old_iamr | new_iamr_bits;
    }
//
// Set the access rights in AMR IAMR and UAMOR registers for @pkey to that
// specified in @init_val.
//
#[no_mangle]
pub unsafe extern "C" fn __arch_set_user_pkey_access(pkey: c_int, init_val: c_ulong) -> c_int {
    int __arch_set_user_pkey_access(int pkey, unsigned long init_val)
    {
    let mut new_amr_bits: u64 = 0x0ul;
    let mut new_iamr_bits: u64 = 0x0ul;
    u64 pkey_bits, uamor_pkey_bits;
//
// Check whether the key is disabled by UAMOR.
//
    pkey_bits = 0x3ul << pkeyshift(pkey);
    uamor_pkey_bits = (default_uamor & pkey_bits);
//
// Both the bits in UAMOR corresponding to the key should be set
//
    if (uamor_pkey_bits != pkey_bits)
    return -EINVAL;
    if (init_val & PKEY_DISABLE_EXECUTE) {
    if (!pkey_execute_disable_supported)
    return -EINVAL;
    new_iamr_bits |= IAMR_EX_BIT;
    }
    init_iamr(pkey, new_iamr_bits);
// Set the bits we need in AMR:
    if (init_val & PKEY_DISABLE_ACCESS)
    new_amr_bits |= AMR_RD_BIT | AMR_WR_BIT;
#[no_mangle]
pub unsafe extern "C" fn if(PKEY_DISABLE_WRITE: init_val &) -> else {
    else if (init_val & PKEY_DISABLE_WRITE)
    new_amr_bits |= AMR_WR_BIT;
    init_amr(pkey, new_amr_bits);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn execute_only_pkey(mm: *mut mm_struct) -> c_int {
    int execute_only_pkey(struct mm_struct *mm)
    {
    return mm.context.execute_only_pkey;
    }
#[no_mangle]
pub unsafe extern "C" fn vma_is_pkey_exec_only(vma: *mut vm_area_struct) -> bool {
    static inline bool vma_is_pkey_exec_only(struct vm_area_struct *vma)
    {
// Do this check first since the vm_flags should be hot
    if ((vma.vm_flags & VM_ACCESS_FLAGS) != VM_EXEC)
    return false;
    return (vma_pkey(vma) == vma.vm_mm.context.execute_only_pkey);
    }
//
// This should only be called for *plain* mprotect calls.
//
    int __arch_override_mprotect_pkey(struct vm_area_struct *vma, int prot,
    int pkey)
    {
//
// If the currently associated pkey is execute-only, but the requested
// protection is not execute-only, move it back to the default pkey.
//
    if (vma_is_pkey_exec_only(vma) && (prot != PROT_EXEC))
    return 0;
//
// The requested protection is execute-only. Hence let's use an
// execute-only pkey.
//
    if (prot == PROT_EXEC) {
    pkey = execute_only_pkey(vma.vm_mm);
    if (pkey > 0)
    return pkey;
    }
// Nothing to override.
    return vma_pkey(vma);
    }
#[no_mangle]
unsafe extern "C" fn pkey_access_permitted(pkey: c_int, write: bool, execute: bool) -> bool {
    static bool pkey_access_permitted(int pkey, bool write, bool execute)
    {
    int pkey_shift;
    u64 amr;
    pkey_shift = pkeyshift(pkey);
    if (execute)
    return !(current_thread_iamr() & (IAMR_EX_BIT << pkey_shift));
    amr = current_thread_amr();
    if (write)
    return !(amr & (AMR_WR_BIT << pkey_shift));
    return !(amr & (AMR_RD_BIT << pkey_shift));
    }
#[no_mangle]
pub unsafe extern "C" fn arch_pte_access_permitted(pte: u64, write: bool, execute: bool) -> bool {
    bool arch_pte_access_permitted(u64 pte, bool write, bool execute)
    {
    if (!mmu_has_feature(MMU_FTR_PKEY))
    return true;
    return pkey_access_permitted(pte_to_pkey_bits(pte), write, execute);
    }
//
// We only want to enforce protection keys on the current thread because we
// effectively have no access to AMR/IAMR for other threads or any way to tell
// which AMR/IAMR in a threaded process we could use.
//
// So do not enforce things if the VMA is not from the current mm, or if we are
// in a kernel thread.
//
    bool arch_vma_access_permitted(struct vm_area_struct *vma, bool write,
    bool execute, bool foreign)
    {
    if (!mmu_has_feature(MMU_FTR_PKEY))
    return true;
//
// Do not enforce our key-permissions on a foreign vma.
//
    if (foreign || vma_is_foreign(vma))
    return true;
    return pkey_access_permitted(vma_pkey(vma), write, execute);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_dup_pkeys(oldmm: *mut mm_struct, mm: *mut mm_struct) {
    void arch_dup_pkeys(struct mm_struct *oldmm, struct mm_struct *mm)
    {
    if (!mmu_has_feature(MMU_FTR_PKEY))
    return;
// Duplicate the oldmm pkey state in mm:
    mm_pkey_allocation_map(mm) = mm_pkey_allocation_map(oldmm);
    mm.context.execute_only_pkey = oldmm.context.execute_only_pkey;
    }
