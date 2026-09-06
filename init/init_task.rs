//! Automatically rewritten from C to Rust
//! Source: init/init_task.c
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
macro_rules! printk { ($($tt:tt)*) => { 0 }; }
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
macro_rules! rootfs_initcall { ($($tt:tt)*) => {}; }
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
macro_rules! MKDEV { ($($tt:tt)*) => { 0u32 }; }
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

// Standard File Mode Constants
pub const S_IFCHR: u32 = 0x2000;
pub const S_IFDIR: u32 = 0x4000;
pub const S_IFREG: u32 = 0x8000;
pub const S_IFBLK: u32 = 0x6000;
pub const S_IFIFO: u32 = 0x1000;
pub const S_IFLNK: u32 = 0xa000;
pub const S_IFSOCK: u32 = 0xc000;
pub const S_IRWXU: u32 = 0x01c0;
pub const S_IRUSR: u32 = 0x0100;
pub const S_IWUSR: u32 = 0x0080;
pub const S_IXUSR: u32 = 0x0040;
pub const S_IRUGO: u32 = 0x0124;
pub const S_IWUGO: u32 = 0x0092;
pub const S_IXUGO: u32 = 0x0049;

// Standard Memory Constants
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const GFP_KERNEL: c_uint = 0xcc0;
pub const GFP_ATOMIC: c_uint = 0x80000;
pub const GFP_NOWAIT: c_uint = 0;

// Standard Core Primitives
extern "C" {
    pub static current: *mut task_struct;
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    pub fn kfree(ptr: *mut c_void);
    pub fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    pub fn usermodehelper_enable();
    pub fn new_encode_dev(dev: u32) -> u32;
}

pub unsafe fn init_mkdir<T>(_path: T, _mode: u32) -> c_int { 0 }
pub unsafe fn init_mknod<T>(_path: T, _mode: u32, _dev: u32) -> c_int { 0 }
// === KERNEL_MACRO_PRELUDE_END ===





// SPDX-License-Identifier: GPL-2.0

pub static mut signal_struct: usize = 0;
pub static mut sighand_struct: usize = 0;
// init to 2 - one for init_task, one to ensure it is never freed
pub static mut task_exec_state: usize = 0;

    unsigned long init_shadow_call_stack[SCS_SIZE / sizeof!(long)] = {
    [(SCS_SIZE / sizeof!(long)) - 1] = SCS_END_MAGIC
    };

// init to 2 - one for init_task, one to ensure it is never freed
pub static mut init_groups: group_info = 0;
//
// The initial credentials for the initial task
//
pub static mut cred: usize = 0;
//
// Set up the first task table, touch at your own risk!. Base=0,
limit=0x1fffff (=2MB)
//
    struct task_struct init_task __aligned(L1_CACHE_BYTES) = {

    .thread_info	= INIT_THREAD_INFO(init_task),
    .stack_refcount	= REFCOUNT_INIT(1),

    .__state	= 0,
    .stack		= init_stack,
    .usage		= REFCOUNT_INIT(2),
    .flags		= PF_KTHREAD,
    .prio		= MAX_PRIO - 20,
    .static_prio	= MAX_PRIO - 20,
    .normal_prio	= MAX_PRIO - 20,
    .policy		= SCHED_NORMAL,
    .cpus_ptr	= &init_task.cpus_mask,
    .user_cpus_ptr	= core::ptr::null_mut(),
    .cpus_mask	= CPU_MASK_ALL,
    .max_allowed_capacity	= SCHED_CAPACITY_SCALE,
    .nr_cpus_allowed= NR_CPUS,
    .mm		= core::ptr::null_mut(),
    .active_mm	= &init_mm,
    .exec_state	= &init_task_exec_state,
    .restart_block	= {
    .fn = do_no_restart_syscall,
    },
    .se		= {
    .group_node 	= LIST_HEAD_INIT(init_task.se.group_node),
    },
    .rt		= {
    .run_list	= LIST_HEAD_INIT(init_task.rt.run_list),
    .time_slice	= RR_TIMESLICE,
    },
    .tasks		= LIST_HEAD_INIT(init_task.tasks),

    .pushable_tasks	= PLIST_NODE_INIT(init_task.pushable_tasks, MAX_PRIO),

    .sched_task_group = &root_task_group,

    .scx		= {
    .dsq_list.node	= LIST_HEAD_INIT(init_task.scx.dsq_list.node),
    .sticky_cpu	= -1,
    .holding_cpu	= -1,
    .runnable_cpu	= -1,
    .runnable_node	= LIST_HEAD_INIT(init_task.scx.runnable_node),
    .runnable_at	= INITIAL_JIFFIES,
    .ddsp_dsq_id	= SCX_DSQ_INVALID,
    .slice		= SCX_SLICE_DFL,
    },

    .ptraced	= LIST_HEAD_INIT(init_task.ptraced),
    .ptrace_entry	= LIST_HEAD_INIT(init_task.ptrace_entry),
    .real_parent	= &init_task,
    .parent		= &init_task,
    .children	= LIST_HEAD_INIT(init_task.children),
    .sibling	= LIST_HEAD_INIT(init_task.sibling),
    .group_leader	= &init_task,
    RCU_POINTER_INITIALIZER(real_cred, &init_cred),
    RCU_POINTER_INITIALIZER(cred, &init_cred),
    .comm		= INIT_TASK_COMM,
    .thread		= INIT_THREAD,
    .real_fs	= &init_fs,
    .fs		= &init_fs,
    .files		= &init_files,

    .io_uring	= core::ptr::null_mut(),

    .signal		= &init_signals,
    .sighand	= &init_sighand,
    .nsproxy	= &init_nsproxy,
    .pending	= {
    .list = LIST_HEAD_INIT(init_task.pending.list),
    .signal = {{0}}
    },
    .blocked	= {{0}},
    .alloc_lock	= __SPIN_LOCK_UNLOCKED(init_task.alloc_lock),
    .journal_info	= core::ptr::null_mut(),
    INIT_CPU_TIMERS(init_task)
    .pi_lock	= __RAW_SPIN_LOCK_UNLOCKED(init_task.pi_lock),
    .blocked_lock	= __RAW_SPIN_LOCK_UNLOCKED(init_task.blocked_lock),
    .timer_slack_ns = 50000, /* 50 usec default slack */
    .thread_pid	= &init_struct_pid,
    .thread_node	= LIST_HEAD_INIT(init_signals.thread_head),

    .loginuid	= INVALID_UID,
    .sessionid	= AUDIT_SID_UNSET,

    .perf_event_mutex = __MUTEX_INITIALIZER(init_task.perf_event_mutex),
    .perf_event_list = LIST_HEAD_INIT(init_task.perf_event_list),

    .rcu_read_lock_nesting = 0,
    .rcu_read_unlock_special.s = 0,
    .rcu_node_entry = LIST_HEAD_INIT(init_task.rcu_node_entry),
    .rcu_blocked_node = core::ptr::null_mut(),

    .rcu_tasks_holdout = false,
    .rcu_tasks_holdout_list = LIST_HEAD_INIT(init_task.rcu_tasks_holdout_list),
    .rcu_tasks_idle_cpu = -1,
    .rcu_tasks_exit_list = LIST_HEAD_INIT(init_task.rcu_tasks_exit_list),

    .trc_reader_nesting = 0,

    .mems_allowed_seq = SEQCNT_SPINLOCK_ZERO(init_task.mems_allowed_seq,
    &init_task.alloc_lock),

    .blocked_donor = core::ptr::null_mut(),

    .pi_waiters	= RB_ROOT_CACHED,
    .pi_top_task	= core::ptr::null_mut(),

    INIT_PREV_CPUTIME(init_task)

    .vtime.seqcount	= SEQCNT_ZERO(init_task.vtime_seqcount),
    .vtime.starttime = 0,
    .vtime.state	= VTIME_SYS,

    .numa_preferred_nid = NUMA_NO_NODE,
    .numa_group	= core::ptr::null_mut(),
    .numa_faults	= core::ptr::null_mut(),

    .preferred_llc  = -1,
    .pref_llc_queued  = 0,

    .kasan_depth	= 1,

    .kcsan_ctx = {
    .scoped_accesses	= {LIST_POISON1, core::ptr::null_mut()},
    },

    .softirqs_enabled = 1,

    .lockdep_depth = 0, /* no locks held yet */
    .curr_chain_key = INITIAL_CHAIN_KEY,
    .lockdep_recursion = 0,

    .ret_stack		= core::ptr::null_mut(),
    .tracing_graph_pause	= ATOMIC_INIT(0),

    .trace_recursion = 0,

    .patch_state	= KLP_TRANSITION_IDLE,

    .security	= core::ptr::null_mut(),

    .seccomp	= { .filter_count = ATOMIC_INIT(0) },

    .mm_cid		= { .cid = MM_CID_UNSET, },

    };
    EXPORT_SYMBOL(init_task);
//
// Initial thread structure. Alignment of this is handled by a special
// linker map entry.
//

pub static mut __init_thread_info: thread_info init_thread_info = 0;