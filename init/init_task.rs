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


// SPDX-License-Identifier: GPL-2.0

    static struct signal_struct init_signals = {
    .nr_threads	= 1,
    .thread_head	= LIST_HEAD_INIT(init_task.thread_node),
    .wait_chldexit	= __WAIT_QUEUE_HEAD_INITIALIZER(init_signals.wait_chldexit),
    .shared_pending	= {
    .list = LIST_HEAD_INIT(init_signals.shared_pending.list),
    .signal =  {{0}}
    },
    .multiprocess	= HLIST_HEAD_INIT,
    .rlim		= INIT_RLIMITS,

    .cgroup_threadgroup_rwsem	= __RWSEM_INITIALIZER(init_signals.cgroup_threadgroup_rwsem),

    .cred_guard_mutex = __MUTEX_INITIALIZER(init_signals.cred_guard_mutex),
    .exec_update_lock = __RWSEM_INITIALIZER(init_signals.exec_update_lock),

    .posix_timers		= HLIST_HEAD_INIT,
    .ignored_posix_timers	= HLIST_HEAD_INIT,
    .cputimer		= {
    .cputime_atomic	= INIT_CPUTIME_ATOMIC,
    },

    INIT_CPU_TIMERS(init_signals)
    .pids = {
    [PIDTYPE_PID]	= &init_struct_pid,
    [PIDTYPE_TGID]	= &init_struct_pid,
    [PIDTYPE_PGID]	= &init_struct_pid,
    [PIDTYPE_SID]	= &init_struct_pid,
    },
    INIT_PREV_CPUTIME(init_signals)
    };
    static struct sighand_struct init_sighand = {
    .count		= REFCOUNT_INIT(1),
    .action		= { { { .sa_handler = SIG_DFL, } }, },
    .siglock	= __SPIN_LOCK_UNLOCKED(init_sighand.siglock),
    .signalfd_wqh	= __WAIT_QUEUE_HEAD_INITIALIZER(init_sighand.signalfd_wqh),
    };
// init to 2 - one for init_task, one to ensure it is never freed
    struct task_exec_state init_task_exec_state = {
    .count		= REFCOUNT_INIT(2),
    .dumpable	= TASK_DUMPABLE_OWNER,
    .user_ns	= &init_user_ns,
    };

    unsigned long init_shadow_call_stack[SCS_SIZE / sizeof(long)] = {
    [(SCS_SIZE / sizeof(long)) - 1] = SCS_END_MAGIC
    };

// init to 2 - one for init_task, one to ensure it is never freed
    let mut init_groups: static struct group_info = { .usage = REFCOUNT_INIT(2) };
//
// The initial credentials for the initial task
//
    static struct cred init_cred = {
    .usage			= ATOMIC_INIT(4),
    .uid			= GLOBAL_ROOT_UID,
    .gid			= GLOBAL_ROOT_GID,
    .suid			= GLOBAL_ROOT_UID,
    .sgid			= GLOBAL_ROOT_GID,
    .euid			= GLOBAL_ROOT_UID,
    .egid			= GLOBAL_ROOT_GID,
    .fsuid			= GLOBAL_ROOT_UID,
    .fsgid			= GLOBAL_ROOT_GID,
    .securebits		= SECUREBITS_DEFAULT,
    .cap_inheritable	= CAP_EMPTY_SET,
    .cap_permitted		= CAP_FULL_SET,
    .cap_effective		= CAP_FULL_SET,
    .cap_bset		= CAP_FULL_SET,
    .user			= INIT_USER,
    .user_ns		= &init_user_ns,
    .group_info		= &init_groups,
    .ucounts		= &init_ucounts,
    };
//
// Set up the first task table, touch at your own risk!. Base=0,
// limit=0x1fffff (=2MB)
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

    let mut __init_thread_info: thread_info init_thread_info = INIT_THREAD_INFO(init_task);
