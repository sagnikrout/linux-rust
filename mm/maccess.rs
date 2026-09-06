//! Automatically rewritten from C to Rust
//! Source: mm/maccess.c
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
// Access kernel or user memory without faulting.
//

    bool __weak copy_from_kernel_nofault_allowed(const void *unsafe_src,
    size_t size)
    {
    return true;
    }
//
// The below only uses kmsan_check_memory() to ensure uninitialized kernel
// memory isn't leaked.
//

    while (len >= sizeof(type)) {					\
    __get_kernel_nofault(dst, src, type, err_label);	\
    kmsan_check_memory(src, sizeof(type));			\
    dst += sizeof(type);					\
    src += sizeof(type);					\
    len -= sizeof(type);					\
    }
#[no_mangle]
pub unsafe extern "C" fn copy_from_kernel_nofault(dst: *mut c_void, src: *const c_void, size: usize) -> c_long {
    long copy_from_kernel_nofault(void *dst, const void *src, size_t size)
    {
    let mut align: c_ulong = 0;
    if (!IS_ENABLED(CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS))
    align = (unsigned long)dst | (unsigned long)src;
    if (!copy_from_kernel_nofault_allowed(src, size))
    return -ERANGE;
    pagefault_disable();
    if (!(align & 7))
    copy_from_kernel_nofault_loop(dst, src, size, u64, Efault);
    if (!(align & 3))
    copy_from_kernel_nofault_loop(dst, src, size, u32, Efault);
    if (!(align & 1))
    copy_from_kernel_nofault_loop(dst, src, size, u16, Efault);
    copy_from_kernel_nofault_loop(dst, src, size, u8, Efault);
    pagefault_enable();
    return 0;
    Efault:
    pagefault_enable();
    return -EFAULT;
    }
    EXPORT_SYMBOL_GPL(copy_from_kernel_nofault);

    while (len >= sizeof(type)) {					\
    __put_kernel_nofault(dst, src, type, err_label);	\
    instrument_write(dst, sizeof(type));			\
    dst += sizeof(type);					\
    src += sizeof(type);					\
    len -= sizeof(type);					\
    }
#[no_mangle]
pub unsafe extern "C" fn copy_to_kernel_nofault(dst: *mut c_void, src: *const c_void, size: usize) -> c_long {
    long copy_to_kernel_nofault(void *dst, const void *src, size_t size)
    {
    let mut align: c_ulong = 0;
    if (!IS_ENABLED(CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS))
    align = (unsigned long)dst | (unsigned long)src;
    pagefault_disable();
    if (!(align & 7))
    copy_to_kernel_nofault_loop(dst, src, size, u64, Efault);
    if (!(align & 3))
    copy_to_kernel_nofault_loop(dst, src, size, u32, Efault);
    if (!(align & 1))
    copy_to_kernel_nofault_loop(dst, src, size, u16, Efault);
    copy_to_kernel_nofault_loop(dst, src, size, u8, Efault);
    pagefault_enable();
    return 0;
    Efault:
    pagefault_enable();
    return -EFAULT;
    }
#[no_mangle]
pub unsafe extern "C" fn strncpy_from_kernel_nofault(dst: *mut c_char, unsafe_addr: *const c_void, count: c_long) -> c_long {
    long strncpy_from_kernel_nofault(char *dst, const void *unsafe_addr, long count)
    {
    const void *src = unsafe_addr;
    if (unlikely(count <= 0))
    return 0;
    if (!copy_from_kernel_nofault_allowed(unsafe_addr, count))
    return -ERANGE;
    pagefault_disable();
    do {
    __get_kernel_nofault(dst, src, u8, Efault);
    dst++;
    src++;
    } while (dst[-1] && src - unsafe_addr < count);
    pagefault_enable();
    dst[-1] = '\0';
    return src - unsafe_addr;
    Efault:
    pagefault_enable();
    dst[0] = '\0';
    return -EFAULT;
    }
//
// copy_from_user_nofault(): safely attempt to read from a user-space location
// @dst: pointer to the buffer that shall take the data
// @src: address to read from. This must be a user address.
// @size: size of the data chunk
//
// Safely read from user address @src to the buffer at @dst. If a kernel fault
// happens, handle that and return -EFAULT.
//
#[no_mangle]
pub unsafe extern "C" fn copy_from_user_nofault(dst: *mut c_void, src: *const void __user, size: usize) -> c_long {
    long copy_from_user_nofault(void *dst, const void __user *src, size_t size)
    {
    let mut ret: c_long = -EFAULT;
    if (!__access_ok(src, size))
    return ret;
    if (!nmi_uaccess_okay())
    return ret;
    pagefault_disable();
    ret = __copy_from_user_inatomic(dst, src, size);
    pagefault_enable();
    if (ret)
    return -EFAULT;
    return 0;
    }
    EXPORT_SYMBOL_GPL(copy_from_user_nofault);
//
// copy_to_user_nofault(): safely attempt to write to a user-space location
// @dst: address to write to
// @src: pointer to the data that shall be written
// @size: size of the data chunk
//
// Safely write to address @dst from the buffer at @src.  If a kernel fault
// happens, handle that and return -EFAULT.
//
#[no_mangle]
pub unsafe extern "C" fn copy_to_user_nofault(dst: *mut void __user, src: *const c_void, size: usize) -> c_long {
    long copy_to_user_nofault(void __user *dst, const void *src, size_t size)
    {
    let mut ret: c_long = -EFAULT;
    if (access_ok(dst, size)) {
    pagefault_disable();
    ret = __copy_to_user_inatomic(dst, src, size);
    pagefault_enable();
    }
    if (ret)
    return -EFAULT;
    return 0;
    }
    EXPORT_SYMBOL_GPL(copy_to_user_nofault);
//
// strncpy_from_user_nofault: - Copy a NUL terminated string from unsafe user
// address.
// @dst:   Destination address, in kernel space.  This buffer must be at
// least @count bytes long.
// @unsafe_addr: Unsafe user address.
// @count: Maximum number of bytes to copy, including the trailing NUL.
//
// Copies a NUL-terminated string from unsafe user address to kernel buffer.
//
// On success, returns the length of the string INCLUDING the trailing NUL.
//
// If access fails, returns -EFAULT (some data may have been copied
// and the trailing NUL added).
//
// If @count is smaller than the length of the string, copies @count-1 bytes,
// sets the last byte of @dst buffer to NUL and returns @count.
//
    long strncpy_from_user_nofault(char *dst, const void __user *unsafe_addr,
    long count)
    {
    long ret;
    if (unlikely(count <= 0))
    return 0;
    pagefault_disable();
    ret = strncpy_from_user(dst, unsafe_addr, count);
    pagefault_enable();
    if (ret >= count) {
    ret = count;
    dst[ret - 1] = '\0';
    } else if (ret >= 0) {
    ret++;
    }
    return ret;
    }
//
// strnlen_user_nofault: - Get the size of a user string INCLUDING final NUL.
// @unsafe_addr: The string to measure.
// @count: Maximum count (including NUL)
//
// Get the size of a NUL-terminated string in user space without pagefault.
//
// Returns the size of the string INCLUDING the terminating NUL.
//
// If the string is too long, returns a number larger than @count. User
// has to check the return value against "> count".
// On exception (or invalid count), returns 0.
//
// Unlike strnlen_user, this can be used from IRQ handler etc. because
// it disables pagefaults.
//
#[no_mangle]
pub unsafe extern "C" fn strnlen_user_nofault(unsafe_addr: *const void __user, count: c_long) -> c_long {
    long strnlen_user_nofault(const void __user *unsafe_addr, long count)
    {
    int ret;
    pagefault_disable();
    ret = strnlen_user(unsafe_addr, count);
    pagefault_enable();
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn __copy_overflow(size: c_int, count: c_ulong) {
    void __copy_overflow(int size, unsigned long count)
    {
    WARN(1, "Buffer overflow detected (%d < %lu)!\n", size, count);
    }
    EXPORT_SYMBOL(__copy_overflow);
