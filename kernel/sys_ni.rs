//! Automatically rewritten from C to Rust
//! Source: kernel/sys_ni.c
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
// === KERNEL_MACRO_PRELUDE_START ===
macro_rules! EXPORT_SYMBOL { ($($tt:tt)*) => {}; }
macro_rules! EXPORT_SYMBOL_GPL { ($($tt:tt)*) => {}; }
macro_rules! MODULE_LICENSE { ($($tt:tt)*) => {}; }
macro_rules! MODULE_AUTHOR { ($($tt:tt)*) => {}; }
macro_rules! MODULE_DESCRIPTION { ($($tt:tt)*) => {}; }
macro_rules! MODULE_ALIAS { ($($tt:tt)*) => {}; }
macro_rules! module_init { ($($tt:tt)*) => {}; }
macro_rules! module_exit { ($($tt:tt)*) => {}; }
macro_rules! early_initcall { ($($tt:tt)*) => {}; }
macro_rules! core_initcall { ($($tt:tt)*) => {}; }
macro_rules! postcore_initcall { ($($tt:tt)*) => {}; }
macro_rules! arch_initcall { ($($tt:tt)*) => {}; }
macro_rules! subsys_initcall { ($($tt:tt)*) => {}; }
macro_rules! fs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DEFINE { ($($tt:tt)*) => {}; }
macro_rules! ARRAY_SIZE { ($($tt:tt)*) => { 1 }; }
macro_rules! container_of { ($($tt:tt)*) => { core::ptr::null_mut() }; }
macro_rules! sizeof { ($($tt:tt)*) => { 0usize }; }
macro_rules! IS_ENABLED { ($($tt:tt)*) => { false }; }
macro_rules! DECLARE_WORK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_WAKE_Q { ($($tt:tt)*) => {}; }
macro_rules! LLIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! LIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! SET_UID { ($($tt:tt)*) => {}; }
macro_rules! SET_GID { ($($tt:tt)*) => {}; }
macro_rules! list_for_each_entry { ($($tt:tt)*) => { if false }; }
macro_rules! list_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! llist_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! pr_info_once { ($($tt:tt)*) => {}; }
macro_rules! pr_info { ($($tt:tt)*) => {}; }
macro_rules! pr_warn { ($($tt:tt)*) => {}; }
macro_rules! pr_err { ($($tt:tt)*) => {}; }
macro_rules! pr_debug { ($($tt:tt)*) => {}; }
macro_rules! early_param { ($($tt:tt)*) => {}; }
macro_rules! BUILD_BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! WARN_ON { ($($tt:tt)*) => { false }; }
macro_rules! WARN_ON_ONCE { ($($tt:tt)*) => { false }; }
macro_rules! BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! BUG { () => {}; }
macro_rules! IS_ERR { ($($tt:tt)*) => { false }; }
macro_rules! PTR_ERR { ($($tt:tt)*) => { 0 }; }
macro_rules! ERR_PTR { ($($tt:tt)*) => { core::ptr::null_mut() }; }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cred { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct notifier_block { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_notifier_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_header { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_root { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_set { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kern_ipc_perm { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_params { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_queue { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msgseg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_sender { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_receiver { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sembuf { pub sem_num: u16, pub sem_op: i16, pub sem_flg: i16 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem_array { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmid_kernel { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shm_file_data { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wake_q_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct work_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct llist_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;
pub type key_t = i32;
pub type kuid_t = u32;
pub type kgid_t = u32;
pub type int = c_int;
pub type uint = c_uint;
pub type ulong = c_ulong;
pub type long = c_long;
pub type void = c_void;

// Standard Linux Error Codes
pub const EPERM: c_int = 1;
pub const ENOENT: c_int = 2;
pub const ESRCH: c_int = 3;
pub const EINTR: c_int = 4;
pub const EIO: c_int = 5;
pub const ENXIO: c_int = 6;
pub const E2BIG: c_int = 7;
pub const ENOEXEC: c_int = 8;
pub const EBADF: c_int = 9;
pub const ECHILD: c_int = 10;
pub const EAGAIN: c_int = 11;
pub const ENOMEM: c_int = 12;
pub const EACCES: c_int = 13;
pub const EFAULT: c_int = 14;
pub const EBUSY: c_int = 16;
pub const EEXIST: c_int = 17;
pub const EXDEV: c_int = 18;
pub const ENODEV: c_int = 19;
pub const ENOTDIR: c_int = 20;
pub const EISDIR: c_int = 21;
pub const EINVAL: c_int = 22;
pub const ENFILE: c_int = 23;
pub const EMFILE: c_int = 24;
pub const ENOSPC: c_int = 28;
pub const EROFS: c_int = 30;
pub const EIDRM: c_int = 43;
pub const EOPNOTSUPP: c_int = 95;
pub const ENOTSUPP: c_int = 524;

// Standard Memory Constants
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const GFP_KERNEL: c_uint = 0xcc0;
pub const GFP_ATOMIC: c_uint = 0x80000;
pub const GFP_NOWAIT: c_uint = 0;

// Standard Core Primitives
extern "C" {
    pub static current: *mut task_struct;
    pub fn printk(fmt: *const c_char, ...) -> c_int;
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    pub fn kfree(ptr: *mut c_void);
}
// === KERNEL_MACRO_PRELUDE_END ===

macro_rules! EXPORT_SYMBOL { ($($tt:tt)*) => {}; }
macro_rules! EXPORT_SYMBOL_GPL { ($($tt:tt)*) => {}; }
macro_rules! MODULE_LICENSE { ($($tt:tt)*) => {}; }
macro_rules! MODULE_AUTHOR { ($($tt:tt)*) => {}; }
macro_rules! MODULE_DESCRIPTION { ($($tt:tt)*) => {}; }
macro_rules! MODULE_ALIAS { ($($tt:tt)*) => {}; }
macro_rules! module_init { ($($tt:tt)*) => {}; }
macro_rules! module_exit { ($($tt:tt)*) => {}; }
macro_rules! early_initcall { ($($tt:tt)*) => {}; }
macro_rules! core_initcall { ($($tt:tt)*) => {}; }
macro_rules! postcore_initcall { ($($tt:tt)*) => {}; }
macro_rules! arch_initcall { ($($tt:tt)*) => {}; }
macro_rules! subsys_initcall { ($($tt:tt)*) => {}; }
macro_rules! fs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }



// SPDX-License-Identifier: GPL-2.0

// Architectures may override COND_SYSCALL and COND_SYSCALL_COMPAT

// we can't #include <linux/syscalls.h> here,
    but tell gcc to not warn with -Wmissing-prototypes  */
    asmlinkage long sys_ni_syscall(void);
//
// Non-implemented system calls get redirected here.
//
#[no_mangle]
pub unsafe extern "C" fn sys_ni_syscall() -> asmlinkage long {
    return -ENOSYS;
    }

//
// This list is kept in the same order as include/uapi/asm-generic/unistd.h.
// Architecture specific entries go below, followed by deprecated or obsolete
// system calls.
//
    COND_SYSCALL(io_setup);
    COND_SYSCALL_COMPAT(io_setup);
    COND_SYSCALL(io_destroy);
    COND_SYSCALL(io_submit);
    COND_SYSCALL_COMPAT(io_submit);
    COND_SYSCALL(io_cancel);
    COND_SYSCALL(io_getevents_time32);
    COND_SYSCALL(io_getevents);
    COND_SYSCALL(io_pgetevents_time32);
    COND_SYSCALL(io_pgetevents);
    COND_SYSCALL_COMPAT(io_pgetevents);
    COND_SYSCALL_COMPAT(io_pgetevents_time64);
    COND_SYSCALL(io_uring_setup);
    COND_SYSCALL(io_uring_enter);
    COND_SYSCALL(io_uring_register);
    COND_SYSCALL(eventfd2);
    COND_SYSCALL(epoll_create1);
    COND_SYSCALL(epoll_ctl);
    COND_SYSCALL(epoll_pwait);
    COND_SYSCALL_COMPAT(epoll_pwait);
    COND_SYSCALL(epoll_pwait2);
    COND_SYSCALL_COMPAT(epoll_pwait2);
    COND_SYSCALL(inotify_init1);
    COND_SYSCALL(inotify_add_watch);
    COND_SYSCALL(inotify_rm_watch);
    COND_SYSCALL(ioprio_set);
    COND_SYSCALL(ioprio_get);
    COND_SYSCALL(flock);
    COND_SYSCALL(quotactl);
    COND_SYSCALL(quotactl_fd);
    COND_SYSCALL(signalfd4);
    COND_SYSCALL_COMPAT(signalfd4);
    COND_SYSCALL(timerfd_create);
    COND_SYSCALL(timerfd_settime);
    COND_SYSCALL(timerfd_settime32);
    COND_SYSCALL(timerfd_gettime);
    COND_SYSCALL(timerfd_gettime32);
    COND_SYSCALL(acct);
    COND_SYSCALL(capget);
    COND_SYSCALL(capset);
    COND_SYSCALL(futex);
    COND_SYSCALL(futex_time32);
    COND_SYSCALL(set_robust_list);
    COND_SYSCALL_COMPAT(set_robust_list);
    COND_SYSCALL(get_robust_list);
    COND_SYSCALL_COMPAT(get_robust_list);
    COND_SYSCALL(futex_waitv);
    COND_SYSCALL(futex_wake);
    COND_SYSCALL(futex_wait);
    COND_SYSCALL(futex_requeue);
    COND_SYSCALL(kexec_load);
    COND_SYSCALL_COMPAT(kexec_load);
    COND_SYSCALL(init_module);
    COND_SYSCALL(delete_module);
    COND_SYSCALL(syslog);
    COND_SYSCALL(setregid);
    COND_SYSCALL(setgid);
    COND_SYSCALL(setreuid);
    COND_SYSCALL(setuid);
    COND_SYSCALL(setresuid);
    COND_SYSCALL(getresuid);
    COND_SYSCALL(setresgid);
    COND_SYSCALL(getresgid);
    COND_SYSCALL(setfsuid);
    COND_SYSCALL(setfsgid);
    COND_SYSCALL(setgroups);
    COND_SYSCALL(getgroups);
    COND_SYSCALL(mq_open);
    COND_SYSCALL_COMPAT(mq_open);
    COND_SYSCALL(mq_unlink);
    COND_SYSCALL(mq_timedsend);
    COND_SYSCALL(mq_timedsend_time32);
    COND_SYSCALL(mq_timedreceive);
    COND_SYSCALL(mq_timedreceive_time32);
    COND_SYSCALL(mq_notify);
    COND_SYSCALL_COMPAT(mq_notify);
    COND_SYSCALL(mq_getsetattr);
    COND_SYSCALL_COMPAT(mq_getsetattr);
    COND_SYSCALL(msgget);
    COND_SYSCALL(old_msgctl);
    COND_SYSCALL(msgctl);
    COND_SYSCALL_COMPAT(msgctl);
    COND_SYSCALL_COMPAT(old_msgctl);
    COND_SYSCALL(msgrcv);
    COND_SYSCALL_COMPAT(msgrcv);
    COND_SYSCALL(msgsnd);
    COND_SYSCALL_COMPAT(msgsnd);
    COND_SYSCALL(semget);
    COND_SYSCALL(old_semctl);
    COND_SYSCALL(semctl);
    COND_SYSCALL_COMPAT(semctl);
    COND_SYSCALL_COMPAT(old_semctl);
    COND_SYSCALL(semtimedop);
    COND_SYSCALL(semtimedop_time32);
    COND_SYSCALL(semop);
    COND_SYSCALL(shmget);
    COND_SYSCALL(old_shmctl);
    COND_SYSCALL(shmctl);
    COND_SYSCALL_COMPAT(shmctl);
    COND_SYSCALL_COMPAT(old_shmctl);
    COND_SYSCALL(shmat);
    COND_SYSCALL_COMPAT(shmat);
    COND_SYSCALL(shmdt);
    COND_SYSCALL(socket);
    COND_SYSCALL(socketpair);
    COND_SYSCALL(bind);
    COND_SYSCALL(listen);
    COND_SYSCALL(accept);
    COND_SYSCALL(connect);
    COND_SYSCALL(getsockname);
    COND_SYSCALL(getpeername);
    COND_SYSCALL(setsockopt);
    COND_SYSCALL_COMPAT(setsockopt);
    COND_SYSCALL(getsockopt);
    COND_SYSCALL_COMPAT(getsockopt);
    COND_SYSCALL(sendto);
    COND_SYSCALL(shutdown);
    COND_SYSCALL(recvfrom);
    COND_SYSCALL_COMPAT(recvfrom);
    COND_SYSCALL(sendmsg);
    COND_SYSCALL_COMPAT(sendmsg);
    COND_SYSCALL(recvmsg);
    COND_SYSCALL_COMPAT(recvmsg);
    COND_SYSCALL(mremap);
    COND_SYSCALL(add_key);
    COND_SYSCALL(request_key);
    COND_SYSCALL(keyctl);
    COND_SYSCALL_COMPAT(keyctl);
    COND_SYSCALL(landlock_create_ruleset);
    COND_SYSCALL(landlock_add_rule);
    COND_SYSCALL(landlock_restrict_self);
    COND_SYSCALL(fadvise64_64);
    COND_SYSCALL_COMPAT(fadvise64_64);
    COND_SYSCALL(lsm_get_self_attr);
    COND_SYSCALL(lsm_set_self_attr);
    COND_SYSCALL(lsm_list_modules);
// CONFIG_MMU only
    COND_SYSCALL(swapon);
    COND_SYSCALL(swapoff);
    COND_SYSCALL(mprotect);
    COND_SYSCALL(msync);
    COND_SYSCALL(mlock);
    COND_SYSCALL(munlock);
    COND_SYSCALL(mlockall);
    COND_SYSCALL(munlockall);
    COND_SYSCALL(mincore);
    COND_SYSCALL(madvise);
    COND_SYSCALL(process_madvise);
    COND_SYSCALL(process_mrelease);
    COND_SYSCALL(remap_file_pages);
    COND_SYSCALL(mbind);
    COND_SYSCALL(get_mempolicy);
    COND_SYSCALL(set_mempolicy);
    COND_SYSCALL(migrate_pages);
    COND_SYSCALL(move_pages);
    COND_SYSCALL(set_mempolicy_home_node);
    COND_SYSCALL(cachestat);
    COND_SYSCALL(mseal);
    COND_SYSCALL(perf_event_open);
    COND_SYSCALL(accept4);
    COND_SYSCALL(recvmmsg);
    COND_SYSCALL(recvmmsg_time32);
    COND_SYSCALL_COMPAT(recvmmsg_time32);
    COND_SYSCALL_COMPAT(recvmmsg_time64);
// Posix timer syscalls may be configured out
    COND_SYSCALL(timer_create);
    COND_SYSCALL(timer_gettime);
    COND_SYSCALL(timer_getoverrun);
    COND_SYSCALL(timer_settime);
    COND_SYSCALL(timer_delete);
    COND_SYSCALL(clock_adjtime);
    COND_SYSCALL(getitimer);
    COND_SYSCALL(setitimer);
    COND_SYSCALL(alarm);
    COND_SYSCALL_COMPAT(timer_create);
    COND_SYSCALL_COMPAT(getitimer);
    COND_SYSCALL_COMPAT(setitimer);
//
// Architecture specific syscalls: see further below
//
// fanotify
    COND_SYSCALL(fanotify_init);
    COND_SYSCALL(fanotify_mark);
// open by handle
    COND_SYSCALL(name_to_handle_at);
    COND_SYSCALL(open_by_handle_at);
    COND_SYSCALL_COMPAT(open_by_handle_at);
    COND_SYSCALL(sendmmsg);
    COND_SYSCALL_COMPAT(sendmmsg);
    COND_SYSCALL(process_vm_readv);
    COND_SYSCALL_COMPAT(process_vm_readv);
    COND_SYSCALL(process_vm_writev);
    COND_SYSCALL_COMPAT(process_vm_writev);
// compare kernel pointers
    COND_SYSCALL(kcmp);
    COND_SYSCALL(finit_module);
// operate on Secure Computing state
    COND_SYSCALL(seccomp);
    COND_SYSCALL(memfd_create);
// access BPF programs and maps
    COND_SYSCALL(bpf);
// execveat
    COND_SYSCALL(execveat);
    COND_SYSCALL(userfaultfd);
// membarrier
    COND_SYSCALL(membarrier);
    COND_SYSCALL(mlock2);
    COND_SYSCALL(copy_file_range);
// memory protection keys
    COND_SYSCALL(pkey_mprotect);
    COND_SYSCALL(pkey_alloc);
    COND_SYSCALL(pkey_free);
// memfd_secret
    COND_SYSCALL(memfd_secret);
//
// Architecture specific weak syscall entries.
//
// pciconfig: alpha, arm, arm64, ia64, sparc
    COND_SYSCALL(pciconfig_read);
    COND_SYSCALL(pciconfig_write);
    COND_SYSCALL(pciconfig_iobase);
// sys_socketcall: arm, mips, x86, ...
    COND_SYSCALL(socketcall);
    COND_SYSCALL_COMPAT(socketcall);
// compat syscalls for arm64, x86, ...
    COND_SYSCALL_COMPAT(fanotify_mark);
// x86
    COND_SYSCALL(vm86old);
    COND_SYSCALL(modify_ldt);
    COND_SYSCALL(vm86);
    COND_SYSCALL(kexec_file_load);
    COND_SYSCALL(map_shadow_stack);
// s390
    COND_SYSCALL(s390_pci_mmio_read);
    COND_SYSCALL(s390_pci_mmio_write);
    COND_SYSCALL(s390_ipc);
    COND_SYSCALL_COMPAT(s390_ipc);
// powerpc
    COND_SYSCALL(rtas);
    COND_SYSCALL(spu_run);
    COND_SYSCALL(spu_create);
    COND_SYSCALL(subpage_prot);
//
// Deprecated system calls which are still defined in
// include/uapi/asm-generic/unistd.h and wanted by >= 1 arch
//
// __ARCH_WANT_SYSCALL_NO_FLAGS
    COND_SYSCALL(epoll_create);
    COND_SYSCALL(inotify_init);
    COND_SYSCALL(eventfd);
    COND_SYSCALL(signalfd);
    COND_SYSCALL_COMPAT(signalfd);
// __ARCH_WANT_SYSCALL_OFF_T
    COND_SYSCALL(fadvise64);
// __ARCH_WANT_SYSCALL_DEPRECATED
    COND_SYSCALL(epoll_wait);
    COND_SYSCALL(recv);
    COND_SYSCALL_COMPAT(recv);
    COND_SYSCALL(send);
    COND_SYSCALL(uselib);
// optional: time32
    COND_SYSCALL(time32);
    COND_SYSCALL(stime32);
    COND_SYSCALL(utime32);
    COND_SYSCALL(adjtimex_time32);
    COND_SYSCALL(sched_rr_get_interval_time32);
    COND_SYSCALL(nanosleep_time32);
    COND_SYSCALL(rt_sigtimedwait_time32);
    COND_SYSCALL_COMPAT(rt_sigtimedwait_time32);
    COND_SYSCALL(timer_settime32);
    COND_SYSCALL(timer_gettime32);
    COND_SYSCALL(clock_settime32);
    COND_SYSCALL(clock_gettime32);
    COND_SYSCALL(clock_getres_time32);
    COND_SYSCALL(clock_nanosleep_time32);
    COND_SYSCALL(utimes_time32);
    COND_SYSCALL(futimesat_time32);
    COND_SYSCALL(pselect6_time32);
    COND_SYSCALL_COMPAT(pselect6_time32);
    COND_SYSCALL(ppoll_time32);
    COND_SYSCALL_COMPAT(ppoll_time32);
    COND_SYSCALL(utimensat_time32);
    COND_SYSCALL(clock_adjtime32);
    COND_SYSCALL(gettimeofday);
    COND_SYSCALL_COMPAT(gettimeofday);
    COND_SYSCALL(time);
    COND_SYSCALL(stime);
//
// The syscalls below are not found in include/uapi/asm-generic/unistd.h
//
// obsolete: SGETMASK_SYSCALL
    COND_SYSCALL(sgetmask);
    COND_SYSCALL(ssetmask);
// obsolete: SYSFS_SYSCALL
    COND_SYSCALL(sysfs);
// obsolete: __ARCH_WANT_SYS_IPC
    COND_SYSCALL(ipc);
    COND_SYSCALL_COMPAT(ipc);
// obsolete: UID16
    COND_SYSCALL(chown16);
    COND_SYSCALL(fchown16);
    COND_SYSCALL(getegid16);
    COND_SYSCALL(geteuid16);
    COND_SYSCALL(getgid16);
    COND_SYSCALL(getgroups16);
    COND_SYSCALL(getresgid16);
    COND_SYSCALL(getresuid16);
    COND_SYSCALL(getuid16);
    COND_SYSCALL(lchown16);
    COND_SYSCALL(setfsgid16);
    COND_SYSCALL(setfsuid16);
    COND_SYSCALL(setgid16);
    COND_SYSCALL(setgroups16);
    COND_SYSCALL(setregid16);
    COND_SYSCALL(setresgid16);
    COND_SYSCALL(setresuid16);
    COND_SYSCALL(setreuid16);
    COND_SYSCALL(setuid16);
// restartable sequence
    COND_SYSCALL(rseq);
    COND_SYSCALL(rseq_slice_yield);
    COND_SYSCALL(uretprobe);
    COND_SYSCALL(uprobe);