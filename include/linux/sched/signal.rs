//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sched/signal.h
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
// Types defining task->signal and task->sighand and APIs using them:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sighand_struct {
    pub siglock: spinlock_t,
    pub count: refcount_t,
    pub signalfd_wqh: wait_queue_head_t,
    pub action: [k_sigaction; _NSIG],
}

//
// Per-process accounting stats:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pacct_struct {
    pub ac_flag: c_int,
    pub ac_exitcode: c_long,
    pub ac_mem: c_ulong,
    pub ac_stime: u64 ac_utime,,
    pub ac_majflt: unsigned long ac_minflt,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_itimer {
    pub expires: u64,
    pub incr: u64,
}

//
// This is the atomic variant of task_cputime, which can be used for
// storing and updating task_cputime statistics without locking.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_cputime_atomic {
    pub utime: core::sync::atomic::AtomicI64,
    pub stime: core::sync::atomic::AtomicI64,
    pub sum_exec_runtime: core::sync::atomic::AtomicI64,
}

//
// struct thread_group_cputimer - thread group interval timer counts
// @cputime_atomic:	atomic thread group interval timers.
//
// This structure contains the version of task_cputime, above, that is
// used for thread group CPU timer calculations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thread_group_cputimer {
    pub cputime_atomic: task_cputime_atomic,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct multiprocess_signals {
    pub signal: sigset_t,
    pub node: hlist_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_thread {
    pub task: *mut task_struct,
    pub next: *mut core_thread,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_state {
    pub nr_threads: core::sync::atomic::AtomicI32,
    pub dumper: core_thread,
    pub startup: completion,
}

//
// NOTE! "signal_struct" does not have its own
// locking, because a shared signal_struct always
// implies a shared sighand_struct, so locking
// sighand_struct is always a proper superset of
// the locking of signal_struct.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct signal_struct {
    pub sigcnt: refcount_t,
    pub live: core::sync::atomic::AtomicI32,
    pub nr_threads: c_int,
    pub quick_threads: c_int,
    pub thread_head: list_head,
    pub /: *mut *mut wait_queue_head_t wait_chldexit; / for wait4(),
// current thread group signal load-balancing target:
    pub curr_target: *mut task_struct,
// shared signal handling:
    pub shared_pending: sigpending,
// For collecting multiprocess signals during fork
    pub multiprocess: hlist_head,
// thread group exit support
    pub group_exit_code: c_int,
// notify group_exec_task when notify_count is less or equal to 0
    pub notify_count: c_int,
    pub group_exec_task: *mut task_struct,
// thread group stop support, overloads group_exit_code too
    pub group_stop_count: c_int,
    pub /: *mut *mut *mut unsigned int flags; / see SIGNAL_ flags below,
    pub /: *mut *mut *mut core_state core_state; / coredumping support,
//
// PR_SET_CHILD_SUBREAPER marks a process, like a service
// manager, to re-parent orphan (double-forking) child processes
// to this process instead of 'init'. The service manager is
// able to receive SIGCHLD signals and is able to investigate
// the process until it calls wait(). All children of this
// process will inherit a flag if they should look for a
// child_subreaper process at exit.
//
    pub is_child_subreaper:1: c_uint,
    pub has_child_subreaper:1: c_uint,
    pub autoreap:1: c_uint,

// POSIX.1b Interval Timers
    pub timer_create_restore_ids:1: c_uint,
    pub next_posix_timer_id: core::sync::atomic::AtomicI32,
    pub posix_timers: hlist_head,
    pub ignored_posix_timers: hlist_head,
// ITIMER_REAL timer for the process
    pub real_timer: hrtimer,
    pub it_real_incr: ktime_t,
//
// ITIMER_PROF and ITIMER_VIRTUAL timers for the process, we use
// CPUCLOCK_PROF and CPUCLOCK_VIRT for indexing array as these
// values are defined to 0 and 1 respectively
//
    pub it: [cpu_itimer; 2],
//
// Thread group totals for process CPU timers.
// See thread_group_cputimer(), et al, for details.
//
    pub cputimer: thread_group_cputimer,

// Empty if CONFIG_POSIX_TIMERS=n
    pub posix_cputimers: posix_cputimers,
// PID/PID hash table linkage.
    pub pids: [*mut pid; PIDTYPE_MAX],
    pub tick_dep_mask: core::sync::atomic::AtomicI32,

    pub tty_old_pgrp: *mut pid,
// boolean value for session group leader
    pub leader: c_int,
    pub /: *mut *mut *mut tty_tty; / NULL if no tty,

    pub autogroup: *mut autogroup,

//
// Cumulative resource counters for dead threads in the group,
// and for reaped dead child processes forked by this group.
// Live threads maintain their own counters and add to these
// in __exit_signal, except for the group leader.
//
    pub stats_lock: seqlock_t,
    pub cstime: u64 utime, stime, cutime,,
    pub gtime: u64,
    pub cgtime: u64,
    pub prev_cputime: prev_cputime,
    pub cnivcsw: unsigned long nvcsw, nivcsw, cnvcsw,,
    pub cmaj_flt: unsigned long min_flt, maj_flt, cmin_flt,,
    pub coublock: unsigned long inblock, oublock, cinblock,,
    pub cmaxrss: unsigned long maxrss,,
    pub ioac: task_io_accounting,
//
// Cumulative ns of schedule CPU time fo dead threads in the
// group, not including a zombie group leader, (This only differs
// from jiffies_to_ns(utime + stime) if sched_clock uses something
// other than jiffies.)
//
    pub sum_sched_runtime: c_ulonglong,
//
// We don't bother to synchronize most readers of this at all,
// because there is no reader checking a limit that actually needs
// to get both rlim_cur and rlim_max atomically, and either one
// alone is a single word that can safely be read normally.
// getrlimit/setrlimit use task_lock(current->group_leader) to
// protect this instead of the siglock, because they really
// have no need to disable irqs.
//
    pub rlim: [rlimit; RLIM_NLIMITS],
    pub /: *mut *mut pacct_pacct; / per-process accounting information,

    pub stats: *mut taskstats,

    pub audit_tty: unsigned,
    pub tty_audit_buf: *mut tty_audit_buf,

    pub cgroup_threadgroup_rwsem: rw_semaphore,

//
// Thread is the potential origin of an oom condition; kill first on
// oom
//
    pub oom_flag_origin: bool,
    pub /: *mut *mut short oom_score_adj; / OOM kill score adjustment,
    pub value.: *mut *mut short oom_score_adj_min; / OOM kill score adjustment min,
// Only settable by CAP_SYS_RESOURCE.
    pub got: *mut *mut *mut mm_oom_mm; / recorded mm when the thread group,
// killed by the oom killer
    pub on: *mut *mut mutex cred_guard_mutex; / guard against foreign influences,
// credential calculations
// (notably. ptrace)
// Deprecated do not use in new code.
// Use exec_update_lock instead.
//
    pub is: *mut *mut rw_semaphore exec_update_lock; / Held while task_struct,
// being updated during exec,
// and may have inconsistent
// permissions.
//
    pub __randomize_layout: },
//
// Bits in flags field of signal_struct.
//
pub const SIGNAL_STOP_STOPPED: c_uint = 0x00000001 /* job control stop in effect */;
pub const SIGNAL_STOP_CONTINUED: c_uint = 0x00000002 /* SIGCONT since WCONTINUED reap */;
pub const SIGNAL_GROUP_EXIT: c_uint = 0x00000004 /* group exit in progress */;
//
// Pending notifications to parent.
//
pub const SIGNAL_CLD_STOPPED: c_uint = 0x00000010;
pub const SIGNAL_CLD_CONTINUED: c_uint = 0x00000020;

pub const SIGNAL_UNKILLABLE: c_uint = 0x00000040 /* for init: ignore fatal signals */;

    pub SIGNAL_GROUP_EXIT): WARN_ON(sig->flags &,
    pub flags: sig->flags = (sig->flags & ~SIGNAL_STOP_MASK) |,
    pub ): *mut extern void flush_signals(struct task_struct,
    pub ): *mut extern void ignore_signals(struct task_struct,
    pub force_default): *mut *mut extern void flush_signal_handlers(struct task_struct , int,
    pub type): *mut *mut *mut extern int dequeue_signal(sigset_t mask, kernel_siginfo_t info, enum pid_type,
    pub current: *mut *mut task_task =,
    pub __info: kernel_siginfo_t,
    pub __type: pid_type,
    pub ret: c_int,
    pub &__type): ret = dequeue_signal(&task->blocked, &__info,,
    pub ret: return,
    pub JOBCTL_STOPPED: current->jobctl |=,
    pub t): *mut task_struct,
    pub addr): *mut int force_sig_fault(int sig, int code, void __user,
    pub t): *mut *mut int send_sig_fault(int sig, int code, void __user addr, struct task_struct,
    pub short): *mut *mut int force_sig_mceerr(int code, void __user ,,
    pub ): *mut *mut int send_sig_mceerr(int code, void __user , short, struct task_struct,
    pub upper): *mut *mut *mut int force_sig_bnderr(void __user addr, void __user lower, void __user,
    pub pkey): *mut *mut int force_sig_pkuerr(void __user addr, u32,
    pub sig_data): *mut *mut int send_sig_perf(void __user addr, u32 type, u64,
    pub addr): *mut int force_sig_ptrace_errno_trap(int errno, void __user,
    pub trapno): *mut *mut int force_sig_fault_trapno(int sig, int code, void __user addr, int,
    pub t): *mut task_struct,
    pub force_coredump): int force_sig_seccomp(int syscall, int reason, bool,
    pub ): *mut *mut extern int send_sig_info(int, struct kernel_siginfo , struct task_struct,
    pub sig): extern void force_sigsegv(int,
    pub ): *mut extern int force_sig_info(struct kernel_siginfo,
    pub pgrp): *mut *mut extern int __kill_pgrp_info(int sig, struct kernel_siginfo info, struct pid,
    pub pid): *mut *mut extern int kill_pid_info(int sig, struct kernel_siginfo info, struct pid,
    pub ): *const cred,
    pub priv): *mut *mut extern int kill_pgrp(struct pid pid, int sig, int,
    pub priv): *mut *mut extern int kill_pid(struct pid pid, int sig, int,
    pub int): *mut *mut extern __must_check bool do_notify_parent(struct task_struct ,,
    pub parent): *mut *mut extern void __wake_up_parent(struct task_struct p, struct task_struct,
    pub force_sig(int): extern void,
    pub force_fatal_sig(int): extern void,
    pub force_exit_sig(int): extern void,
    pub int): *mut *mut extern int send_sig(int, struct task_struct ,,
    pub p): *mut extern int zap_other_threads(struct task_struct,
    pub ): *mut *mut extern int do_sigaction(int, struct k_sigaction , struct k_sigaction,
//
// Returns 'true' if kick_process() is needed to force a transition from
// user -> kernel to guarantee expedient run of TWA_SIGNAL based task_work.
//
    pub TASK_INTERRUPTIBLE): !wake_up_state(task,,
//
// Called to break out of interruptible wait loops, and enter the
// exit_to_user_mode_loop().
//
    pub TIF_SIGPENDING): set_tsk_thread_flag(current,,
    pub -ERESTARTNOINTR: return,
    pub unlikely(test_tsk_thread_flag(p,TIF_SIGPENDING)): return,
//
// TIF_NOTIFY_SIGNAL isn't really a signal, but it requires the same
// behavior in terms of ensuring that we break out of wait loops
// so that notify signal callbacks can be processed.
//
    pub 1: return,
    pub task_sigpending(p): return,
    pub SIGKILL)): return unlikely(sigismember(&p->pending.signal,,
    pub __fatal_signal_pending(p): return task_sigpending(p) &&,
    pub 0: return,
    pub 0: return,
    pub __fatal_signal_pending(p): return (state & TASK_INTERRUPTIBLE) ||,
//
// This should only be used in fault handlers to decide whether we
// should stop the current fault routine to handle the signals
// instead, especially with the case where we've got interrupted with
// a VM_FAULT_RETRY.
//
    pub signal_pending(current)))): (user_mode(regs) &&,
//
// Reevaluate whether the task has signals pending delivery.
// Wake the task if so.
// This is required every time the blocked sigset_t changes.
// callers must hold sighand->siglock.
//
    pub recalc_sigpending(void): extern void,
    pub calculate_sigpending(void): extern void,
    pub state): *mut *mut extern void signal_wake_up_state(struct task_struct t, unsigned int,
    pub 0: unsigned int state =,
    pub JOBCTL_TRACED): t->jobctl &= ~(JOBCTL_STOPPED |,
    pub __TASK_TRACED: state = TASK_WAKEKILL |,
    pub state): signal_wake_up_state(t,,
    pub 0: unsigned int state =,
    pub ~JOBCTL_TRACED: t->jobctl &=,
    pub __TASK_TRACED: state =,
    pub state): signal_wake_up_state(t,,
    pub task): *mut void task_join_group_stop(struct task_struct,

//
// Legacy restore_sigmask accessors.  These are inefficient on
// SMP architectures because they require atomic operations.
//
// set_restore_sigmask() - make sure saved_sigmask processing gets done
//
// This sets TIF_RESTORE_SIGMASK and ensures that the arch signal code
// will run before returning to user mode, to process the flag.  For
// all callers, TIF_SIGPENDING is already set or it's no harm to set
// it.  TIF_RESTORE_SIGMASK need not be in the set of bits that the
// arch code will notice on return to user mode, in case those bits
// are scarce.  We set TIF_SIGPENDING here to ensure that the arch
// signal code always gets run when TIF_RESTORE_SIGMASK is set.
//
    pub TIF_RESTORE_SIGMASK): clear_tsk_thread_flag(task,,
    pub TIF_RESTORE_SIGMASK): return test_tsk_thread_flag(task,,
    pub test_thread_flag(TIF_RESTORE_SIGMASK): return,
    pub test_and_clear_thread_flag(TIF_RESTORE_SIGMASK): return,

// Higher-quality implementation, used if TIF_RESTORE_SIGMASK doesn't exist.
    pub true: current->restore_sigmask =,
    pub false: task->restore_sigmask =,
    pub false: current->restore_sigmask =,
    pub current->restore_sigmask: return,
    pub task->restore_sigmask: return,
    pub false: return,
    pub false: current->restore_sigmask =,
    pub true: return,

    pub sigsetsize): *const *const extern int set_user_sigmask(sigset_t __user umask, size_t,
    pub &current->blocked: *mut *mut sigset_t res =,
    pub &current->saved_sigmask: res =,
    pub res: return,
    pub priv): return kill_pid(cad_pid, sig,,
// These can be the second arg to send_sig_info/send_group_sig_info.

    pub current->sas_ss_size: sp - current->sas_ss_sp <,

    pub current->sas_ss_size: sp - current->sas_ss_sp <=,

//
// True if we are on the alternate signal stack.
//
// If the signal stack is SS_AUTODISARM then, by construction, we
// can't be on the signal stack unless user code deliberately set
// SS_AUTODISARM when we were already on it.
//
// This improves reliability: if user state gets corrupted such that
// the stack pointer points very close to the end of the signal stack,
// then this check will enable the signal to be handled anyway.
//
    pub 0: return,
    pub __on_sig_stack(sp): return,
    pub SS_DISABLE: return,
    pub 0: return on_sig_stack(sp) ? SS_ONSTACK :,
    pub 0: p->sas_ss_sp =,
    pub 0: p->sas_ss_size =,
    pub SS_DISABLE: p->sas_ss_flags =,

    pub current->sas_ss_sp: return,

    pub current->sas_ss_size: return current->sas_ss_sp +,

    pub sp: return,
    pub ): *mut extern void __cleanup_sighand(struct sighand_struct,
    pub flush_itimer_signals(void): extern void,

    pub ): for (p = &init_task ; (p = next_task(p)) != &init_task ;,
    pub current_is_single_threaded(void): extern bool,
//
// Without tasklist/siglock it is only rcu-safe if g can't exit/exec,
// otherwise next_thread(t) will never reach g after list_del_rcu(g).
//

    pub ): for (t = p; (t = next_thread(t)) != p;,

// Careful: this is a double loop, 'break' won't work as expected.

    pub data): *mut *mut *mut typedef int (proc_visitor)(struct task_struct p, void,
    pub ): *mut *mut void walk_process_tree(struct task_struct top, proc_visitor, void,
    pub pid: *mut pid,
    pub task_pid(task): pid =,
    pub task->signal->pids[type]: pid =,
    pub pid: return,
    pub task->signal->pids[PIDTYPE_TGID]: return,
//
// Without tasklist or RCU lock it is not safe to dereference
// the result of task_pgrp/task_session even if task == current,
// we can race with another thread doing sys_setsid/sys_setpgid.
//
    pub task->signal->pids[PIDTYPE_PGID]: return,
    pub task->signal->pids[PIDTYPE_SID]: return,
    pub task->signal->nr_threads: return,
    pub 0: return p->exit_signal >=,
    pub p2->signal: return p1->signal ==,
//
// returns NULL if p is the last thread in the thread group
//
    pub p->group_leader: return __next_thread(p) ?:,
    pub &p->signal->thread_head): list_is_last(&p->thread_node,,

    pub &task->sighand->siglock): __cond_acquires(nonnull,,
    pub flags): *mut spin_unlock_irqrestore(&task->sighand->siglock,,

    pub task): *mut extern void lockdep_assert_task_sighand_held(struct task_struct,

    pub READ_ONCE(task->signal->rlim[limit].rlim_cur): return,
    pub READ_ONCE(task->signal->rlim[limit].rlim_max): return,
    pub limit): return task_rlimit(current,,
    pub limit): return task_rlimit_max(current,,
