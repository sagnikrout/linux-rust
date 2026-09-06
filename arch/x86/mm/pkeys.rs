//! Automatically rewritten from C to Rust
//! Source: arch/x86/mm/pkeys.c
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
// Intel Memory Protection Keys management
// Copyright (c) 2015, Intel Corporation.
//

#[no_mangle]
pub unsafe extern "C" fn __execute_only_pkey(mm: *mut mm_struct) -> c_int {
    int __execute_only_pkey(struct mm_struct *mm)
    {
    let mut need_to_set_mm_pkey: bool = false;
    let mut execute_only_pkey: c_int = mm.context.execute_only_pkey;
    int ret;
// Do we need to assign a pkey for mm's execute-only maps?
    if (execute_only_pkey == -1) {
// Go allocate one to use, which might fail
    execute_only_pkey = mm_pkey_alloc(mm);
    if (execute_only_pkey < 0)
    return -1;
    need_to_set_mm_pkey = true;
    }
//
// We do not want to go through the relatively costly
// dance to set PKRU if we do not need to.  Check it
// first and assume that if the execute-only pkey is
// write-disabled that we do not have to set it
// ourselves.
//
    if (!need_to_set_mm_pkey &&
    !__pkru_allows_read(read_pkru(), execute_only_pkey)) {
    return execute_only_pkey;
    }
//
// Set up PKRU so that it denies access for everything
// other than execution.
//
    ret = arch_set_user_pkey_access(execute_only_pkey, PKEY_DISABLE_ACCESS);
//
// If the PKRU-set operation failed somehow, just return
// 0 and effectively disable execute-only support.
//
    if (ret) {
    mm_set_pkey_free(mm, execute_only_pkey);
    return -1;
    }
// We got one, store it and use it from here on out
    if (need_to_set_mm_pkey)
    mm.context.execute_only_pkey = execute_only_pkey;
    return execute_only_pkey;
    }
#[no_mangle]
pub unsafe extern "C" fn vma_is_pkey_exec_only(vma: *mut vm_area_struct) -> bool {
    static inline bool vma_is_pkey_exec_only(struct vm_area_struct *vma)
    {
// Do this check first since the vm_flags should be hot
    if ((vma.vm_flags & VM_ACCESS_FLAGS) != VM_EXEC)
    return false;
    if (vma_pkey(vma) != vma.vm_mm.context.execute_only_pkey)
    return false;
    return true;
    }
//
// This is only called for *plain* mprotect calls.
//
#[no_mangle]
pub unsafe extern "C" fn __arch_override_mprotect_pkey(vma: *mut vm_area_struct, prot: c_int, pkey: c_int) -> c_int {
    int __arch_override_mprotect_pkey(struct vm_area_struct *vma, int prot, int pkey)
    {
//
// Is this an mprotect_pkey() call?  If so, never
// override the value that came from the user.
//
    if (pkey != -1)
    return pkey;
//
// The mapping is execute-only.  Go try to get the
// execute-only protection key.  If we fail to do that,
// fall through as if we do not have execute-only
// support in this mm.
//
    if (prot == PROT_EXEC) {
    pkey = execute_only_pkey(vma.vm_mm);
    if (pkey > 0)
    return pkey;
    } else if (vma_is_pkey_exec_only(vma)) {
//
// Protections are *not* PROT_EXEC, but the mapping
// is using the exec-only pkey.  This mapping was
// PROT_EXEC and will no longer be.  Move back to
// the default pkey.
//
    return ARCH_DEFAULT_PKEY;
    }
//
// This is a vanilla, non-pkey mprotect (or we failed to
// setup execute-only), inherit the pkey from the VMA we
// are working on.
//
    return vma_pkey(vma);
    }

//
// Make the default PKRU value (at execve() time) as restrictive
// as possible.  This ensures that any threads clone()'d early
// in the process's lifetime will not accidentally get access
// to data which is pkey-protected later on.
//
    u32 init_pkru_value = PKRU_AD_MASK( 1) | PKRU_AD_MASK( 2) |
    PKRU_AD_MASK( 3) | PKRU_AD_MASK( 4) |
    PKRU_AD_MASK( 5) | PKRU_AD_MASK( 6) |
    PKRU_AD_MASK( 7) | PKRU_AD_MASK( 8) |
    PKRU_AD_MASK( 9) | PKRU_AD_MASK(10) |
    PKRU_AD_MASK(11) | PKRU_AD_MASK(12) |
    PKRU_AD_MASK(13) | PKRU_AD_MASK(14) |
    PKRU_AD_MASK(15);
    static ssize_t init_pkru_read_file(struct file *file, char __user *user_buf,
    size_t count, loff_t *ppos)
    {
    char buf[32];
    unsigned int len;
    len = sprintf(buf, "0x%x\n", init_pkru_value);
    return simple_read_from_buffer(user_buf, count, ppos, buf, len);
    }
    static ssize_t init_pkru_write_file(struct file *file,
    const char __user *user_buf, size_t count, loff_t *ppos)
    {
    char buf[32];
    ssize_t len;
    u32 new_init_pkru;
    len = min(count, sizeof(buf) - 1);
    if (copy_from_user(buf, user_buf, len))
    return -EFAULT;
// Make the buffer a valid string that we can not overrun
    buf[len] = '\0';
    if (kstrtouint(buf, 0, &new_init_pkru))
    return -EINVAL;
//
// Don't allow insane settings that will blow the system
// up immediately if someone attempts to disable access
// or writes to pkey 0.
//
    if (new_init_pkru & (PKRU_AD_BIT|PKRU_WD_BIT))
    return -EINVAL;
    WRITE_ONCE(init_pkru_value, new_init_pkru);
    return count;
    }
    static const struct file_operations fops_init_pkru = {
    .read = init_pkru_read_file,
    .write = init_pkru_write_file,
    .llseek = default_llseek,
    };
#[no_mangle]
unsafe extern "C" fn create_init_pkru_value() -> int __init {
    static int __init create_init_pkru_value(void)
    {
// Do not expose the file if pkeys are not supported.
    if (!cpu_feature_enabled(X86_FEATURE_OSPKE))
    return 0;
    debugfs_create_file("init_pkru", S_IRUSR | S_IWUSR,
    arch_debugfs_dir, core::ptr::null_mut(), &fops_init_pkru);
    return 0;
    }
    late_initcall(create_init_pkru_value);
#[no_mangle]
unsafe extern "C" fn setup_init_pkru(opt: *mut c_char) -> __init int {
    static __init int setup_init_pkru(char *opt)
    {
    u32 new_init_pkru;
    if (kstrtouint(opt, 0, &new_init_pkru))
    return 1;
    WRITE_ONCE(init_pkru_value, new_init_pkru);
    return 1;
    }
    __setup("init_pkru=", setup_init_pkru);
