//! Automatically rewritten from C to Rust
//! Source: ipc/syscall.c
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
// sys_ipc() is the old de-multiplexer for the SysV IPC calls.
//
// This is really horribly ugly, and new architectures should just wire up
// the individual syscalls instead.
//

    int ksys_ipc(unsigned int call, int first, unsigned long second,
    unsigned long third, void __user * ptr, long fifth)
    {
    int version, ret;
    version = call >> 16; /* hack for backward compatibility */
    call &= 0xffff;
    switch (call) {
    case SEMOP:
    return ksys_semtimedop(first, (struct sembuf __user *)ptr,
    second, core::ptr::null_mut());
    case SEMTIMEDOP:
    if (IS_ENABLED(CONFIG_64BIT))
    return ksys_semtimedop(first, ptr, second,
    (const struct __kernel_timespec __user *)fifth);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: IS_ENABLED(CONFIG_COMPAT_32BIT_TIME)) -> else {
    else if (IS_ENABLED(CONFIG_COMPAT_32BIT_TIME))
    return compat_ksys_semtimedop(first, ptr, second,
    (const struct old_timespec32 __user *)fifth);
    else
    return -ENOSYS;
    case SEMGET:
    return ksys_semget(first, second, third);
    case SEMCTL: {
    unsigned long arg;
    if (!ptr)
    return -EINVAL;
    if (get_user(arg, (unsigned long __user *) ptr))
    return -EFAULT;
    return ksys_old_semctl(first, second, third, arg);
    }
    case MSGSND:
    return ksys_msgsnd(first, (struct msgbuf __user *) ptr,
    second, third);
    case MSGRCV:
    switch (version) {
    case 0: {
    struct ipc_kludge tmp;
    if (!ptr)
    return -EINVAL;
    if (copy_from_user(&tmp,
    (struct ipc_kludge __user *) ptr,
    sizeof(tmp)))
    return -EFAULT;
    return ksys_msgrcv(first, tmp.msgp, second,
    tmp.msgtyp, third);
    }
    default:
    return ksys_msgrcv(first,
    (struct msgbuf __user *) ptr,
    second, fifth, third);
    }
    case MSGGET:
    return ksys_msgget((key_t) first, second);
    case MSGCTL:
    return ksys_old_msgctl(first, second,
    (struct msqid_ds __user *)ptr);
    case SHMAT:
    switch (version) {
    default: {
    unsigned long raddr;
    ret = do_shmat(first, (char __user *)ptr,
    second, &raddr, SHMLBA);
    if (ret)
    return ret;
    return put_user(raddr, (unsigned long __user *) third);
    }
    case 1:
//
// This was the entry point for kernel-originating calls
// from iBCS2 in 2.2 days.
//
    return -EINVAL;
    }
    case SHMDT:
    return ksys_shmdt((char __user *)ptr);
    case SHMGET:
    return ksys_shmget(first, second, third);
    case SHMCTL:
    return ksys_old_shmctl(first, second,
    (struct shmid_ds __user *) ptr);
    default:
    return -ENOSYS;
    }
    }
    SYSCALL_DEFINE6(ipc, unsigned int, call, int, first, unsigned long, second,
    unsigned long, third, void __user *, ptr, long, fifth)
    {
    return ksys_ipc(call, first, second, third, ptr, fifth);
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc_kludge {
    pub msgp: compat_uptr_t,
    pub msgtyp: compat_long_t,
}

    int compat_ksys_ipc(u32 call, int first, int second,
    u32 third, compat_uptr_t ptr, u32 fifth)
    {
    int version;
    u32 pad;
    version = call >> 16; /* hack for backward compatibility */
    call &= 0xffff;
    switch (call) {
    case SEMOP:
// struct sembuf is the same on 32 and 64bit :))
    return ksys_semtimedop(first, compat_ptr(ptr), second, core::ptr::null_mut());
    case SEMTIMEDOP:
    if (!IS_ENABLED(CONFIG_COMPAT_32BIT_TIME))
    return -ENOSYS;
    return compat_ksys_semtimedop(first, compat_ptr(ptr), second,
    compat_ptr(fifth));
    case SEMGET:
    return ksys_semget(first, second, third);
    case SEMCTL:
    if (!ptr)
    return -EINVAL;
    if (get_user(pad, (u32 __user *) compat_ptr(ptr)))
    return -EFAULT;
    return compat_ksys_old_semctl(first, second, third, pad);
    case MSGSND:
    return compat_ksys_msgsnd(first, ptr, second, third);
    case MSGRCV: {
    void __user *uptr = compat_ptr(ptr);
    if (first < 0 || second < 0)
    return -EINVAL;
    if (!version) {
    struct compat_ipc_kludge ipck;
    if (!uptr)
    return -EINVAL;
    if (copy_from_user(&ipck, uptr, sizeof(ipck)))
    return -EFAULT;
    return compat_ksys_msgrcv(first, ipck.msgp, second,
    ipck.msgtyp, third);
    }
    return compat_ksys_msgrcv(first, ptr, second, fifth, third);
    }
    case MSGGET:
    return ksys_msgget(first, second);
    case MSGCTL:
    return compat_ksys_old_msgctl(first, second, compat_ptr(ptr));
    case SHMAT: {
    int err;
    unsigned long raddr;
    if (version == 1)
    return -EINVAL;
    err = do_shmat(first, compat_ptr(ptr), second, &raddr,
    COMPAT_SHMLBA);
    if (err < 0)
    return err;
    return put_user(raddr, (compat_ulong_t __user *)compat_ptr(third));
    }
    case SHMDT:
    return ksys_shmdt(compat_ptr(ptr));
    case SHMGET:
    return ksys_shmget(first, (unsigned int)second, third);
    case SHMCTL:
    return compat_ksys_old_shmctl(first, second, compat_ptr(ptr));
    }
    return -ENOSYS;
    }
    COMPAT_SYSCALL_DEFINE6(ipc, u32, call, int, first, int, second,
    u32, third, compat_uptr_t, ptr, u32, fifth)
    {
    return compat_ksys_ipc(call, first, second, third, ptr, fifth);
    }

