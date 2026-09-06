//! Automatically rewritten from C to Rust
//! Source: arch/um/kernel/skas/uaccess.c
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
// Copyright (C) 2002 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
//

    pte_t *virt_to_pte(struct mm_struct *mm, unsigned long addr)
    {
    pgd_t *pgd;
    p4d_t *p4d;
    pud_t *pud;
    pmd_t *pmd;
    if (mm == core::ptr::null_mut())
    return core::ptr::null_mut();
    pgd = pgd_offset(mm, addr);
    if (!pgd_present(*pgd))
    return core::ptr::null_mut();
    p4d = p4d_offset(pgd, addr);
    if (!p4d_present(*p4d))
    return core::ptr::null_mut();
    pud = pud_offset(p4d, addr);
    if (!pud_present(*pud))
    return core::ptr::null_mut();
    pmd = pmd_offset(pud, addr);
    if (!pmd_present(*pmd))
    return core::ptr::null_mut();
    return pte_offset_kernel(pmd, addr);
    }
    static pte_t *maybe_map(unsigned long virt, int is_write)
    {
    pte_t *pte = virt_to_pte(current.mm, virt);
    int err, dummy_code;
    if ((pte == core::ptr::null_mut()) || !pte_present(*pte) ||
    (is_write && !pte_write(*pte))) {
    err = handle_page_fault(virt, 0, is_write, 1, &dummy_code);
    if (err)
    return core::ptr::null_mut();
    pte = virt_to_pte(current.mm, virt);
    }
    if (!pte_present(*pte))
    pte = core::ptr::null_mut();
    return pte;
    }
    static int do_op_one_page(unsigned long addr, int len, int is_write,
    int (*op)(unsigned long addr, int len, void *arg), void *arg)
    {
    struct page *page;
    pte_t *pte;
    int n;
    pte = maybe_map(addr, is_write);
    if (pte == core::ptr::null_mut())
    return -1;
    page = pte_page(*pte);

    pagefault_disable();
    addr = (unsigned long) page_address(page) +
    (addr & ~PAGE_MASK);

    addr = (unsigned long) kmap_atomic(page) +
    (addr & ~PAGE_MASK);

    n = (*op)(addr, len, arg);

    pagefault_enable();

    kunmap_atomic((void *)addr);

    return n;
    }
    static long buffer_op(unsigned long addr, int len, int is_write,
    int (*op)(unsigned long, int, void *), void *arg)
    {
    long size, remain, n;
    size = min(PAGE_ALIGN(addr) - addr, (unsigned long) len);
    remain = len;
    n = do_op_one_page(addr, size, is_write, op, arg);
    if (n != 0) {
    remain = (n < 0 ? remain : 0);
    goto out;
    }
    addr += size;
    remain -= size;
    if (remain == 0)
    goto out;
    while (addr < ((addr + remain) & PAGE_MASK)) {
    n = do_op_one_page(addr, PAGE_SIZE, is_write, op, arg);
    if (n != 0) {
    remain = (n < 0 ? remain : 0);
    goto out;
    }
    addr += PAGE_SIZE;
    remain -= PAGE_SIZE;
    }
    if (remain == 0)
    goto out;
    n = do_op_one_page(addr, remain, is_write, op, arg);
    if (n != 0) {
    remain = (n < 0 ? remain : 0);
    goto out;
    }
    return 0;
    out:
    return remain;
    }
#[no_mangle]
unsafe extern "C" fn copy_chunk_from_user(from: c_ulong, len: c_int, arg: *mut c_void) -> c_int {
    static int copy_chunk_from_user(unsigned long from, int len, void *arg)
    {
    unsigned long *to_ptr = arg, to = *to_ptr;
    memcpy((void *) to, (void *) from, len);
// to_ptr += len;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn raw_copy_from_user(to: *mut c_void, from: *const void __user, n: c_ulong) -> c_ulong {
    unsigned long raw_copy_from_user(void *to, const void __user *from, unsigned long n)
    {
    return buffer_op((unsigned long) from, n, 0, copy_chunk_from_user, &to);
    }
    EXPORT_SYMBOL(raw_copy_from_user);
#[no_mangle]
unsafe extern "C" fn copy_chunk_to_user(to: c_ulong, len: c_int, arg: *mut c_void) -> c_int {
    static int copy_chunk_to_user(unsigned long to, int len, void *arg)
    {
    unsigned long *from_ptr = arg, from = *from_ptr;
    memcpy((void *) to, (void *) from, len);
// from_ptr += len;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn raw_copy_to_user(to: *mut void __user, from: *const c_void, n: c_ulong) -> c_ulong {
    unsigned long raw_copy_to_user(void __user *to, const void *from, unsigned long n)
    {
    return buffer_op((unsigned long) to, n, 1, copy_chunk_to_user, &from);
    }
    EXPORT_SYMBOL(raw_copy_to_user);
#[no_mangle]
unsafe extern "C" fn strncpy_chunk_from_user(from: c_ulong, len: c_int, arg: *mut c_void) -> c_int {
    static int strncpy_chunk_from_user(unsigned long from, int len, void *arg)
    {
    char **to_ptr = arg, *to = *to_ptr;
    int n;
    n = strnlen((void *) from, len);
    memcpy_and_pad(to, len, (void *) from, n, 0);
// to_ptr += n;
    if (n < len)
    return 1;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn strncpy_from_user(dst: *mut c_char, src: *const char __user, count: c_long) -> c_long {
    long strncpy_from_user(char *dst, const char __user *src, long count)
    {
    long n;
    char *ptr = dst;
    if (!access_ok(src, 1))
    return -EFAULT;
    n = buffer_op((unsigned long) src, count, 0, strncpy_chunk_from_user,
    &ptr);
    if (n != 0)
    return -EFAULT;
    return strnlen(dst, count);
    }
    EXPORT_SYMBOL(strncpy_from_user);
#[no_mangle]
unsafe extern "C" fn clear_chunk(addr: c_ulong, len: c_int, unused: *mut c_void) -> c_int {
    static int clear_chunk(unsigned long addr, int len, void *unused)
    {
    memset((void *) addr, 0, len);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __clear_user(mem: *mut void __user, len: c_ulong) -> c_ulong {
    unsigned long __clear_user(void __user *mem, unsigned long len)
    {
    return buffer_op((unsigned long) mem, len, 1, clear_chunk, core::ptr::null_mut());
    }
    EXPORT_SYMBOL(__clear_user);
#[no_mangle]
unsafe extern "C" fn strnlen_chunk(str: c_ulong, len: c_int, arg: *mut c_void) -> c_int {
    static int strnlen_chunk(unsigned long str, int len, void *arg)
    {
    int *len_ptr = arg, n;
    n = strnlen((void *) str, len);
// len_ptr += n;
    if (n < len)
    return 1;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn strnlen_user(str: *const char __user, len: c_long) -> c_long {
    long strnlen_user(const char __user *str, long len)
    {
    let mut count: c_int = 0, n;
    if (!access_ok(str, 1))
    return -EFAULT;
    n = buffer_op((unsigned long) str, len, 0, strnlen_chunk, &count);
    if (n == 0)
    return count + 1;
    return 0;
    }
    EXPORT_SYMBOL(strnlen_user);
//
// arch_futex_atomic_op_inuser() - Atomic arithmetic operation with constant
// argument and comparison of the previous
// futex value with another constant.
//
// @op:		operation to execute
// @oparg:	argument to operation
// @oval:	old value at uaddr
// @uaddr:	pointer to user space address
//
// Return:
// 0 - On success
// -EFAULT - User access resulted in a page fault
// -EAGAIN - Atomic operation was unable to complete due to contention
// -ENOSYS - Operation not supported
//
#[no_mangle]
pub unsafe extern "C" fn arch_futex_atomic_op_inuser(op: c_int, oparg: u32, oval: *mut c_int, uaddr: *mut u32 __user) -> c_int {
    int arch_futex_atomic_op_inuser(int op, u32 oparg, int *oval, u32 __user *uaddr)
    {
    int oldval, ret;
    struct page *page;
    let mut addr: c_ulong = (unsigned long) uaddr;
    pte_t *pte;
    ret = -EFAULT;
    if (!access_ok(uaddr, sizeof(*uaddr)))
    return -EFAULT;
    preempt_disable();
    pte = maybe_map(addr, 1);
    if (pte == core::ptr::null_mut())
    goto out_inuser;
    page = pte_page(*pte);

    pagefault_disable();
    addr = (unsigned long) page_address(page) +
    (((unsigned long) addr) & ~PAGE_MASK);

    addr = (unsigned long) kmap_atomic(page) +
    ((unsigned long) addr & ~PAGE_MASK);

    uaddr = (u32 *) addr;
    oldval = *uaddr;
    ret = 0;
    switch (op) {
    case FUTEX_OP_SET:
// uaddr = oparg;
    break;
    case FUTEX_OP_ADD:
// uaddr += oparg;
    break;
    case FUTEX_OP_OR:
// uaddr |= oparg;
    break;
    case FUTEX_OP_ANDN:
// uaddr &= ~oparg;
    break;
    case FUTEX_OP_XOR:
// uaddr ^= oparg;
    break;
    default:
    ret = -ENOSYS;
    }

    pagefault_enable();

    kunmap_atomic((void *)addr);

    out_inuser:
    preempt_enable();
    if (ret == 0)
// oval = oldval;
    return ret;
    }
    EXPORT_SYMBOL(arch_futex_atomic_op_inuser);
//
// futex_atomic_cmpxchg_inatomic() - Compare and exchange the content of the
// uaddr with newval if the current value is
// oldval.
// @uval:	pointer to store content of @uaddr
// @uaddr:	pointer to user space address
// @oldval:	old value
// @newval:	new value to store to @uaddr
//
// Return:
// 0 - On success
// -EFAULT - User access resulted in a page fault
// -EAGAIN - Atomic operation was unable to complete due to contention
//
    int futex_atomic_cmpxchg_inatomic(u32 *uval, u32 __user *uaddr,
    u32 oldval, u32 newval)
    {
    struct page *page;
    pte_t *pte;
    let mut ret: c_int = -EFAULT;
    if (!access_ok(uaddr, sizeof(*uaddr)))
    return -EFAULT;
    preempt_disable();
    pte = maybe_map((unsigned long) uaddr, 1);
    if (pte == core::ptr::null_mut())
    goto out_inatomic;
    page = pte_page(*pte);

    pagefault_disable();
    uaddr = page_address(page) + (((unsigned long) uaddr) & ~PAGE_MASK);

    uaddr = kmap_atomic(page) + ((unsigned long) uaddr & ~PAGE_MASK);

// uval = *uaddr;
    ret = cmpxchg(uaddr, oldval, newval);

    pagefault_enable();

    kunmap_atomic(uaddr);

    ret = 0;
    out_inatomic:
    preempt_enable();
    return ret;
    }
    EXPORT_SYMBOL(futex_atomic_cmpxchg_inatomic);
