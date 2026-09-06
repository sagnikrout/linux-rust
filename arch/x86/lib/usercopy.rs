//! Automatically rewritten from C to Rust
//! Source: arch/x86/lib/usercopy.c
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


//
// User address space access functions.
//
// For licencing details see kernel-base/COPYING
//

//
// copy_from_user_nmi - NMI safe copy from user
// @to:		Pointer to the destination buffer
// @from:	Pointer to a user space address of the current task
// @n:		Number of bytes to copy
//
// Returns: The number of not copied bytes. 0 is success, i.e. all bytes copied
//
// Contrary to other copy_from_user() variants this function can be called
// from NMI context. Despite the name it is not restricted to be called
// from NMI context. It is safe to be called from any other context as
// well. It disables pagefaults across the copy which means a fault will
// abort the copy.
//
// For NMI context invocations this relies on the nested NMI work to allow
// atomic faults from the NMI path; the nested NMI paths are careful to
// preserve CR2.
//
    unsigned long
    copy_from_user_nmi(void *to, const void __user *from, unsigned long n)
    {
    unsigned long ret;
    if (!__access_ok(from, n))
    return n;
    if (!nmi_uaccess_okay())
    return n;
//
// Even though this function is typically called from NMI/IRQ context
// disable pagefaults so that its behaviour is consistent even when
// called from other contexts.
//
    pagefault_disable();
    instrument_copy_from_user_before(to, from, n);
    ret = raw_copy_from_user(to, from, n);
    instrument_copy_from_user_after(to, from, n, ret);
    pagefault_enable();
    return ret;
    }
    EXPORT_SYMBOL_GPL(copy_from_user_nmi);
