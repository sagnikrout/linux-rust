//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/uaccess.h
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
// Architectures that support memory tagging (assigning tags to memory regions,
// embedding these tags into addresses that point to these memory regions, and
// checking that the memory and the pointer tags match on memory accesses)
// redefine this macro to strip tags from pointers.
//
// Passing down mm_struct allows to define untagging rules on per-process
// basis.
//
// It's defined as noop for architectures that don't support memory tagging.
//

pub const can_do_masked_user_access(): c_int = 1;

pub const can_do_masked_user_access(): c_int = 0;

//
// Architectures should provide two primitives (raw_copy_{to,from}_user())
// and get rid of their private instances of copy_{to,from}_user() and
// __copy_{to,from}_user{,_inatomic}().
//
// raw_copy_{to,from}_user(to, from, size) should copy up to size bytes and
// return the amount left to copy.  They should assume that access_ok() has
// already been checked (and succeeded); they should *not* zero-pad anything.
// No KASAN or object size checks either - those belong here.
//
// Both of these functions should attempt to copy size bytes starting at from
// into the area starting at to.  They must not fetch or store anything
// outside of those areas.  Return value must be between 0 (everything
// copied successfully) and size (nothing copied).
//
// If raw_copy_{to,from}_user(to, from, size) returns N, size - N bytes starting
// at to must become equal to the bytes fetched from the corresponding area
// starting at from.  All data past to + size - N must be left unmodified.
//
// If copying succeeds, the return value must be 0.  If some data cannot be
// fetched, it is permitted to copy less than had been fetched; the only
// hard requirement is that not storing anything at all (i.e. returning size)
// should happen only when nothing could be copied.  In other words, you don't
// have to squeeze as much as possible - it is allowed, but not necessary.
//
// For raw_copy_from_user() to always points to kernel memory and no faults
// on store should happen.  Interpretation of from is affected by set_fs().
// For raw_copy_to_user() it's the other way round.
//
// Both can be inlined - it's up to architectures whether it wants to bother
// with that.  They should not be used directly; they are used to implement
// the 6 functions (copy_{to,from}_user(), __copy_{to,from}_user_inatomic())
// that are used instead.  Out of those, __... ones are inlined.  Plain
// copy_{to,from}_user() might or might not be inlined.  If you want them
// inlined, have asm/uaccess.h define INLINE_COPY_USER.
//
// NOTE: only copy_from_user() zero-pads the destination in case of short copy.
// Neither __copy_from_user() nor __copy_from_user_inatomic() zero anything
// at all; their callers absolutely must check the return value.
//
// Biarch ones should also provide raw_copy_in_user() - similar to the above,
// but both source and destination are __user pointers (affected by set_fs()
// as usual) and both source and destination can trigger faults.
//
// __copy_to_user_inatomic: - Copy a block of data into user space, with less checking.
// @to:   Destination address, in user space.
// @from: Source address, in kernel space.
// @n:    Number of bytes to copy.
//
// Context: User context only.
//
// Copy data from kernel space to user space.  Caller must check
// the specified block with access_ok() before calling this function.
// The caller should also make sure he pins the user space address
// so that we don't result in page fault and sleep.
//
extern "C" {
    pub fn raw_copy_to_user(_arg: to, _arg: from, _arg: n) -> return;
}
extern "C" {
    pub fn raw_copy_to_user(_arg: to, _arg: from, _arg: n) -> return;
}
//
// Architectures that #define INLINE_COPY_USER use this function
// directly in the normal copy_to/from_user(), the other ones go
// through an extern _copy_to/from_user(), which expands the same code
// here.
//
// Ensure that bad access_ok() speculation will not
// lead to nasty side effects *after* the copy is
// finished:
//

extern "C" {
    pub fn _copy_from_user(_arg: to, _arg: from, _arg: n) -> return;
}
extern "C" {
    pub fn _copy_to_user(_arg: to, _arg: from, _arg: n) -> return;
}

//
// Without arch opt-in this generic copy_mc_to_kernel() will not handle
// #MC (or arch equivalent) during source read.
//

//
// These routines enable/disable the pagefault handler. If disabled, it will
// not take any locks and go straight to the fixup table.
//
// User access methods will not sleep when called from a pagefault_disabled()
// environment.
//
// make sure to have issued the store before a pagefault
// can hit.
//
// make sure to issue those last loads/stores before enabling
// the pagefault handler again.
//
// Is the pagefault handler disabled? If so, user access methods will not sleep.
//
// The pagefault handler is in general disabled by pagefault_disable() or
// when in irq context (via in_atomic()).
//
// This function should only be used by the fault handlers. Other users should
// stick to pagefault_disabled().
// Please NEVER use preempt_disable() to disable the fault handler. With
// !CONFIG_PREEMPT_COUNT, this is like a NOP. So the handler won't be disabled.
// in_atomic() will report different values based on !CONFIG_PREEMPT_COUNT.
//

//
// probe_subpage_writeable: probe the user range for write faults at sub-page
// granularity (e.g. arm64 MTE)
// @uaddr: start of address range
// @size: size of address range
//
// Returns 0 on success, the number of bytes not probed on fault.
//
// It is expected that the caller checked for the write permission of each
// page in the range either by put_user() or GUP. The architecture port can
// implement a more efficient get_user() probing if the same sub-page faults
// are triggered by either a read or a write.
//

extern "C" {
    pub fn __copy_from_user_inatomic(_arg: to, _arg: from, _arg: n) -> return;
}

extern "C" {
    pub fn check_zeroed_user(from: *const void __user, size: usize) -> __must_check int;
}
//
// copy_struct_from_user: copy a struct from userspace
// @dst:   Destination address, in kernel space. This buffer must be @ksize
// bytes long.
// @ksize: Size of @dst struct.
// @src:   Source address, in userspace.
// @usize: (Alleged) size of @src struct.
//
// Copies a struct from userspace to kernel space, in a way that guarantees
// backwards-compatibility for struct syscall arguments (as long as future
// struct extensions are made such that all new fields are *appended* to the
// old struct, and zeroed-out new fields have the same meaning as the old
// struct).
//
// @ksize is just sizeof(*dst), and @usize should've been passed by userspace.
// The recommended usage is something like the following:
//
// SYSCALL_DEFINE2(foobar, const struct foo __user *, uarg, size_t, usize)
// {
// int err;
// struct foo karg = {};
//
// if (usize > PAGE_SIZE)
// return -E2BIG;
// if (usize < FOO_SIZE_VER0)
// return -EINVAL;
//
// err = copy_struct_from_user(&karg, sizeof(karg), uarg, usize);
// if (err)
// return err;
//
// // ...
// }
//
// There are three cases to consider:
// * If @usize == @ksize, then it's copied verbatim.
// * If @usize < @ksize, then the userspace has passed an old struct to a
// newer kernel. The rest of the trailing bytes in @dst (@ksize - @usize)
// are to be zero-filled.
// * If @usize > @ksize, then the userspace has passed a new struct to an
// older kernel. The trailing bytes unknown to the kernel (@usize - @ksize)
// are checked to ensure they are zeroed, otherwise -E2BIG is returned.
//
// Returns (in all cases, some data may have been copied):
// * -E2BIG:  (@usize > @ksize) and there are non-zero trailing bytes in @src.
// * -EFAULT: access to userspace failed.
//
// Double check if ksize is larger than a known object size.
// Deal with trailing bytes.
// Copy the interoperable parts of the struct.
//
// copy_struct_to_user: copy a struct to userspace
// @dst:   Destination address, in userspace. This buffer must be @ksize
// bytes long.
// @usize: (Alleged) size of @dst struct.
// @src:   Source address, in kernel space.
// @ksize: Size of @src struct.
// @ignored_trailing: Set to %true if there was a non-zero byte in @src that
// userspace cannot see because they are using an smaller struct.
//
// Copies a struct from kernel space to userspace, in a way that guarantees
// backwards-compatibility for struct syscall arguments (as long as future
// struct extensions are made such that all new fields are *appended* to the
// old struct, and zeroed-out new fields have the same meaning as the old
// struct).
//
// Some syscalls may wish to make sure that userspace knows about everything in
// the struct, and if there is a non-zero value that userspce doesn't know
// about, they want to return an error (such as -EMSGSIZE) or have some other
// fallback (such as adding a "you're missing some information" flag). If
// @ignored_trailing is non-%NULL, it will be set to %true if there was a
// non-zero byte that could not be copied to userspace (ie. was past @usize).
//
// While unconditionally returning an error in this case is the simplest
// solution, for maximum backward compatibility you should try to only return
// -EMSGSIZE if the user explicitly requested the data that couldn't be copied.
// Note that structure sizes can change due to header changes and simple
// recompilations without code changes(!), so if you care about
// @ignored_trailing you probably want to make sure that any new field data is
// associated with a flag. Otherwise you might assume that a program knows
// about data it does not.
//
// @ksize is just sizeof(*src), and @usize should've been passed by userspace.
// The recommended usage is something like the following:
//
// SYSCALL_DEFINE2(foobar, struct foo __user *, uarg, size_t, usize)
// {
// int err;
// bool ignored_trailing;
// struct foo karg = {};
//
// if (usize > PAGE_SIZE)
// return -E2BIG;
// if (usize < FOO_SIZE_VER0)
// return -EINVAL;
//
// // ... modify karg somehow ...
//
// err = copy_struct_to_user(uarg, usize, &karg, sizeof(karg),
// &ignored_trailing);
// if (err)
// return err;
// if (ignored_trailing)
// return -EMSGSIZE:
//
// // ...
// }
//
// There are three cases to consider:
// * If @usize == @ksize, then it's copied verbatim.
// * If @usize < @ksize, then the kernel is trying to pass userspace a newer
// struct than it supports. Thus we only copy the interoperable portions
// (@usize) and ignore the rest (but @ignored_trailing is set to %true if
// any of the trailing (@ksize - @usize) bytes are non-zero).
// * If @usize > @ksize, then the kernel is trying to pass userspace an older
// struct than userspace supports. In order to make sure the
// unknown-to-the-kernel fields don't contain garbage values, we zero the
// trailing (@usize - @ksize) bytes.
//
// Returns (in all cases, some data may have been copied):
// * -EFAULT: access to userspace failed.
//
// Double check if ksize is larger than a known object size.
// Deal with trailing bytes.
// ignored_trailing = usize < ksize &&
// Copy the interoperable parts of the struct.
// Deal with trailing bytes.
// ignored_trailing = dstsize < srcsize &&
// Copy the interoperable parts of the struct.
//
// This is like copy_struct_from_user(), but the
// src buffer was already copied into a kernel
// bounce buffer, so it will never return -EFAULT.
//
// Double check if ksize is larger than a known object size.
//
// This is like copy_struct_to_user(), but the
// dst buffer is a kernel bounce buffer instead
// of a direct userspace buffer, so it will never return -EFAULT.
//
// Double check if srcsize is larger than a known object size.
extern "C" {
    pub fn copy_from_kernel_nofault_allowed(unsafe_src: *const c_void, size: usize) -> bool;
}
extern "C" {
    pub fn copy_from_kernel_nofault(dst: *mut c_void, src: *const c_void, size: usize) -> c_long;
}
extern "C" {
    pub fn copy_to_kernel_nofault(dst: *mut c_void, src: *const c_void, size: usize) -> long notrace;
}
extern "C" {
    pub fn copy_from_user_nofault(dst: *mut c_void, src: *const void __user, size: usize) -> c_long;
}
extern "C" {
    pub fn strnlen_user_nofault(unsafe_addr: *const void __user, count: c_long) -> c_long;
}

//
// Wrap the architecture implementation so that @label can be outside of a
// cleanup() scope. A regular C goto works correctly, but ASM goto does
// not. Clang rejects such an attempt, but GCC silently emits buggy code.
//

// (type *)dst = data;				\

//
// get_kernel_nofault(): safely attempt to read from a location
// @val: read into this variable
// @ptr: address to read from
//
// Returns 0 on success, or -EFAULT.
//

//
// Wrap the architecture implementation so that @label can be outside of a
// cleanup() scope. A regular C goto works correctly, but ASM goto does
// not. Clang rejects such an attempt, but GCC silently emits buggy code.
//
// Some architectures use internal local labels already, but this extra
// indirection here is harmless because the compiler optimizes it out
// completely in any case. This construct just ensures that the ASM GOTO
// target is always in the local scope. The C goto 'label' works correctly
// when leaving a cleanup() scope.
//

// Define RW variant so the below _mode macro expansion works

// Scoped user access
// Cleanup wrapper functions
//
// __scoped_user_access_begin - Start a scoped user access
// @mode:	The mode of the access class (read, write, rw)
// @uptr:	The pointer to access user space memory
// @size:	Size of the access
// @elbl:	Error label to goto when the access region is rejected
//
// Internal helper for __scoped_user_access(). Don't use directly.
//

//
// __scoped_user_access - Open a scope for user access
// @mode:	The mode of the access class (read, write, rw)
// @uptr:	The pointer to access user space memory
// @size:	Size of the access
// @elbl:	Error label to goto when the access region is rejected. It
// must be placed outside the scope
//
// If the user access function inside the scope requires a fault label, it
// can use @elbl or a different label outside the scope, which requires
// that user access which is implemented with ASM GOTO has been properly
// wrapped. See unsafe_get_user() for reference.
//
// scoped_user_rw_access(ptr, efault) {
// unsafe_get_user(rval, &ptr->rval, efault);
// unsafe_put_user(wval, &ptr->wval, efault);
// }
// return 0;
// efault:
// return -EFAULT;
//
// The scope is internally implemented as a autoterminating nested for()
// loop, which can be left with 'return', 'break' and 'goto' at any
// point.
//
// When the scope is left user_##@_mode##_access_end() is automatically
// invoked.
//
// When the architecture supports masked user access and the access region
// which is determined by @uptr and @size is not a valid user space
// address, i.e. < TASK_SIZE, the scope sets the pointer to a faulting user
// space address and does not terminate early. This optimizes for the good
// case and lets the performance uncritical bad case go through the fault.
//
// The eventual modification of the pointer is limited to the scope.
// Outside of the scope the original pointer value is unmodified, so that
// the original pointer value is available for diagnostic purposes in an
// out of scope fault path.
//
// Nesting scoped user access into a user access scope is invalid and fails
// the build. Nesting into other guards, e.g. pagefault is safe.
//
// The masked variant does not check the size of the access and relies on a
// mapping hole (e.g. guard page) to catch an out of range pointer, the
// first access to user memory inside the scope has to be within
// @uptr ... @uptr + PAGE_SIZE - 1
//
// Don't use directly. Use scoped_masked_user_$MODE_access() instead.
//

// Force modified pointer usage within the scope */		\
//
// scoped_user_read_access_size - Start a scoped user read access with given size
// @usrc:	Pointer to the user space address to read from
// @size:	Size of the access starting from @usrc
// @elbl:	Error label to goto when the access region is rejected
//
// For further information see __scoped_user_access() above.
//

//
// scoped_user_read_access - Start a scoped user read access
// @usrc:	Pointer to the user space address to read from
// @elbl:	Error label to goto when the access region is rejected
//
// The size of the access starting from @usrc is determined via sizeof(*@usrc)).
//
// For further information see __scoped_user_access() above.
//

//
// scoped_user_write_access_size - Start a scoped user write access with given size
// @udst:	Pointer to the user space address to write to
// @size:	Size of the access starting from @udst
// @elbl:	Error label to goto when the access region is rejected
//
// For further information see __scoped_user_access() above.
//

//
// scoped_user_write_access - Start a scoped user write access
// @udst:	Pointer to the user space address to write to
// @elbl:	Error label to goto when the access region is rejected
//
// The size of the access starting from @udst is determined via sizeof(*@udst)).
//
// For further information see __scoped_user_access() above.
//

//
// scoped_user_rw_access_size - Start a scoped user read/write access with given size
// @uptr:	Pointer to the user space address to read from and write to
// @size:	Size of the access starting from @uptr
// @elbl:	Error label to goto when the access region is rejected
//
// For further information see __scoped_user_access() above.
//

//
// scoped_user_rw_access - Start a scoped user read/write access
// @uptr:	Pointer to the user space address to read from and write to
// @elbl:	Error label to goto when the access region is rejected
//
// The size of the access starting from @uptr is determined via sizeof(*@uptr)).
//
// For further information see __scoped_user_access() above.
//

//
// get_user_inline - Read user data inlined
// @val:	The variable to store the value read from user memory
// @usrc:	Pointer to the user space memory to read from
//
// Return: 0 if successful, -EFAULT when faulted
//
// Inlined variant of get_user(). Only use when there is a demonstrable
// performance reason.
//

//
// put_user_inline - Write to user memory inlined
// @val:	The value to write
// @udst:	Pointer to the user space memory to write to
//
// Return: 0 if successful, -EFAULT when faulted
//
// Inlined variant of put_user(). Only use when there is a demonstrable
// performance reason.
//

