//! Automatically rewritten from C to Rust
//! Source: kernel/irq/irq_test.c
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


// SPDX-License-Identifier: LGPL-2.1+

#[no_mangle]
unsafe extern "C" fn noop_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    return IRQ_HANDLED;
    }
#[no_mangle]
pub unsafe extern "C" fn noop(data: *mut irq_data) { }
#[no_mangle]
pub unsafe extern "C" fn noop_ret(data: *mut irq_data) -> c_uint { return 0; }
#[no_mangle]
pub unsafe extern "C" fn noop_affinity(data: *mut irq_data, dest: *mut cpumask, force: bool) -> c_int {
    irq_data_update_effective_affinity(data, dest);
    return 0;
    }
pub static mut irq_chip: usize = 0;
#[no_mangle]
unsafe extern "C" fn irq_test_setup_fake_irq(test: *mut kunit, affd: *mut irq_affinity_desc) -> c_int {
pub static mut desc: *mut c_void = core::ptr::null_mut();
    let mut virq = 0;
    virq = irq_domain_alloc_descs(-1, 1, 0, NUMA_NO_NODE, affd);
    KUNIT_ASSERT_GE(test, virq, 0);
    irq_set_chip_and_handler(virq, &fake_irq_chip, handle_simple_irq);
    desc = irq_to_desc(virq);
    KUNIT_ASSERT_PTR_NE(test, desc, core::ptr::null_mut());
// On some architectures, IRQs are NOREQUEST | NOPROBE by default.
    irq_settings_clr_norequest(desc);
    return virq;
    }
#[no_mangle]
unsafe extern "C" fn irq_disable_depth_test(test: *mut kunit) {
pub static mut desc: *mut c_void = core::ptr::null_mut();
    let mut virq = 0;
    let mut ret = 0;
    virq = irq_test_setup_fake_irq(test, core::ptr::null_mut());
    desc = irq_to_desc(virq);
    KUNIT_ASSERT_PTR_NE(test, desc, core::ptr::null_mut());
    ret = request_irq(virq, noop_handler, 0, "test_irq", core::ptr::null_mut());
    KUNIT_ASSERT_EQ(test, ret, 0);
    KUNIT_EXPECT_EQ(test, desc.depth, 0);
    disable_irq(virq);
    KUNIT_EXPECT_EQ(test, desc.depth, 1);
    enable_irq(virq);
    KUNIT_EXPECT_EQ(test, desc.depth, 0);
    free_irq(virq, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn irq_free_disabled_test(test: *mut kunit) {
pub static mut desc: *mut c_void = core::ptr::null_mut();
    let mut virq = 0;
    let mut ret = 0;
    virq = irq_test_setup_fake_irq(test, core::ptr::null_mut());
    desc = irq_to_desc(virq);
    KUNIT_ASSERT_PTR_NE(test, desc, core::ptr::null_mut());
    ret = request_irq(virq, noop_handler, 0, "test_irq", core::ptr::null_mut());
    KUNIT_ASSERT_EQ(test, ret, 0);
    KUNIT_EXPECT_EQ(test, desc.depth, 0);
    disable_irq(virq);
    KUNIT_EXPECT_EQ(test, desc.depth, 1);
    free_irq(virq, core::ptr::null_mut());
    KUNIT_EXPECT_GE(test, desc.depth, 1);
    ret = request_irq(virq, noop_handler, 0, "test_irq", core::ptr::null_mut());
    KUNIT_ASSERT_EQ(test, ret, 0);
    KUNIT_EXPECT_EQ(test, desc.depth, 0);
    free_irq(virq, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn irq_shutdown_depth_test(test: *mut kunit) {
pub static mut desc: *mut c_void = core::ptr::null_mut();
pub static mut data: *mut c_void = core::ptr::null_mut();
    let mut virq = 0;
    let mut ret = 0;
pub static mut irq_affinity_desc: usize = 0;
    if (!IS_ENABLED!(CONFIG_SMP)) {
    kunit_skip(test, "requires CONFIG_SMP for managed shutdown");
    }
    virq = irq_test_setup_fake_irq(test, &affinity);
    desc = irq_to_desc(virq);
    KUNIT_ASSERT_PTR_NE(test, desc, core::ptr::null_mut());
    data = irq_desc_get_irq_data(desc);
    KUNIT_ASSERT_PTR_NE(test, data, core::ptr::null_mut());
    ret = request_irq(virq, noop_handler, 0, "test_irq", core::ptr::null_mut());
    KUNIT_ASSERT_EQ(test, ret, 0);
    KUNIT_EXPECT_TRUE(test, irqd_is_activated(data));
    KUNIT_EXPECT_TRUE(test, irqd_is_started(data));
    KUNIT_EXPECT_TRUE(test, irqd_affinity_is_managed(data));
    KUNIT_EXPECT_EQ(test, desc.depth, 0);
    disable_irq(virq);
    KUNIT_EXPECT_EQ(test, desc.depth, 1);
    scoped_guard(raw_spinlock_irqsave, &desc.lock)
    irq_shutdown_and_deactivate(desc);
    KUNIT_EXPECT_FALSE(test, irqd_is_activated(data));
    KUNIT_EXPECT_FALSE(test, irqd_is_started(data));
    KUNIT_EXPECT_EQ(test, irq_activate(desc), 0);

    irq_startup_managed(desc);

    KUNIT_EXPECT_EQ(test, desc.depth, 1);
    enable_irq(virq);
    KUNIT_EXPECT_EQ(test, desc.depth, 0);
    free_irq(virq, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn irq_cpuhotplug_test(test: *mut kunit) {
pub static mut desc: *mut c_void = core::ptr::null_mut();
pub static mut data: *mut c_void = core::ptr::null_mut();
    let mut virq = 0;
    let mut ret = 0;
pub static mut irq_affinity_desc: usize = 0;
    if (!IS_ENABLED!(CONFIG_SMP)) {
    kunit_skip(test, "requires CONFIG_SMP for CPU hotplug");
    }
    if (!get_cpu_device(1)) {
    kunit_skip(test, "requires more than 1 CPU for CPU hotplug");
    }
    if (!cpu_is_hotpluggable(1)) {
    kunit_skip(test, "CPU 1 must be hotpluggable");
    }
    if (!cpu_online(1)) {
    kunit_skip(test, "CPU 1 must be online");
    }
    cpumask_copy(&affinity.mask, cpumask_of(1));
    virq = irq_test_setup_fake_irq(test, &affinity);
    desc = irq_to_desc(virq);
    KUNIT_ASSERT_PTR_NE(test, desc, core::ptr::null_mut());
    data = irq_desc_get_irq_data(desc);
    KUNIT_ASSERT_PTR_NE(test, data, core::ptr::null_mut());
    ret = request_irq(virq, noop_handler, 0, "test_irq", core::ptr::null_mut());
    KUNIT_ASSERT_EQ(test, ret, 0);
    KUNIT_EXPECT_TRUE(test, irqd_is_activated(data));
    KUNIT_EXPECT_TRUE(test, irqd_is_started(data));
    KUNIT_EXPECT_TRUE(test, irqd_affinity_is_managed(data));
    KUNIT_EXPECT_EQ(test, desc.depth, 0);
    disable_irq(virq);
    KUNIT_EXPECT_EQ(test, desc.depth, 1);
    KUNIT_EXPECT_EQ(test, remove_cpu(1), 0);
    KUNIT_EXPECT_GE(test, desc.depth, 1);
    KUNIT_EXPECT_EQ(test, add_cpu(1), 0);
    KUNIT_EXPECT_EQ(test, desc.depth, 1);
    enable_irq(virq);
    KUNIT_EXPECT_TRUE(test, irqd_is_activated(data));
    KUNIT_EXPECT_TRUE(test, irqd_is_started(data));
    KUNIT_EXPECT_EQ(test, desc.depth, 0);
    free_irq(virq, core::ptr::null_mut());
    }
pub static mut kunit_case: usize = 0;
pub static mut kunit_suite: usize = 0;
    kunit_test_suite(irq_test_suite);
    MODULE_DESCRIPTION("IRQ unit test suite");
    MODULE_LICENSE("GPL");