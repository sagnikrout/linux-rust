//! Automatically rewritten from C Header to Rust Module
//! Source: include/trace/events/rcu.h
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
// Tracepoint for start/end markers used for utilization calculations.
// By convention, the string is of the following forms:
//
// "Start <activity>" -- Mark the start of the specified activity,
// such as "context switch".  Nesting is permitted.
// "End <activity>" -- Mark the end of the specified activity.
//
// An "@" character within "<activity>" is a comment character: Data
// reduction scripts will ignore the "@" and the remainder of the line.
//

//
// Tracepoint for grace-period events.  Takes a string identifying the
// RCU flavor, the grace-period number, and a string identifying the
// grace-period-related event as follows:
//
// "AccReadyCB": CPU accelerates new callbacks to RCU_NEXT_READY_TAIL.
// "AccWaitCB": CPU accelerates new callbacks to RCU_WAIT_TAIL.
// "newreq": Request a new grace period.
// "start": Start a grace period.
// "cpustart": CPU first notices a grace-period start.
// "cpuqs": CPU passes through a quiescent state.
// "cpuonl": CPU comes online.
// "cpuofl": CPU goes offline.
// "cpuofl-bgp": CPU goes offline while blocking a grace period.
// "reqwait": GP kthread sleeps waiting for grace-period request.
// "reqwaitsig": GP kthread awakened by signal from reqwait state.
// "fqswait": GP kthread waiting until time to force quiescent states.
// "fqsstart": GP kthread starts forcing quiescent states.
// "fqsend": GP kthread done forcing quiescent states.
// "fqswaitsig": GP kthread awakened by signal from fqswait state.
// "end": End a grace period.
// "cpuend": CPU first notices a grace-period end.
//
// Tracepoint for future grace-period events.  The caller should pull
// the data from the rcu_node structure, other than rcuname, which comes
// from the rcu_state structure, and event, which is one of the following:
//
// "Cleanup": Clean up rcu_node structure after previous GP.
// "CleanupMore": Clean up, and another GP is needed.
// "EndWait": Complete wait.
// "NoGPkthread": The RCU grace-period kthread has not yet started.
// "Prestarted": Someone beat us to the request
// "Startedleaf": Leaf node marked for future GP.
// "Startedleafroot": All nodes from leaf to root marked for future GP.
// "Startedroot": Requested a nocb grace period based on root-node data.
// "Startleaf": Request a grace period based on leaf-node data.
// "StartWait": Start waiting for the requested grace period.
//
// Tracepoint for grace-period-initialization events.  These are
// distinguished by the type of RCU, the new grace-period number, the
// rcu_node structure level, the starting and ending CPU covered by the
// rcu_node structure, and the mask of CPUs that will be waited for.
// All but the type of RCU are extracted from the rcu_node structure.
//
// Tracepoint for expedited grace-period events.  Takes a string identifying
// the RCU flavor, the expedited grace-period sequence number, and a string
// identifying the grace-period-related event as follows:
//
// "snap": Captured snapshot of expedited grace period sequence number.
// "start": Started a real expedited grace period.
// "reset": Started resetting the tree
// "select": Started selecting the CPUs to wait on.
// "selectofl": Selected CPU partially offline.
// "startwait": Started waiting on selected CPUs.
// "end": Ended a real expedited grace period.
// "endwake": Woke piggybackers up.
// "done": Someone else did the expedited grace period for us.
//
// Tracepoint for expedited grace-period funnel-locking events.  Takes a
// string identifying the RCU flavor, an integer identifying the rcu_node
// combining-tree level, another pair of integers identifying the lowest-
// and highest-numbered CPU associated with the current rcu_node structure,
// and a string.  identifying the grace-period-related event as follows:
//
// "nxtlvl": Advance to next level of rcu_node funnel
// "wait": Wait for someone else to do expedited GP
//

//
// Tracepoint for RCU no-CBs CPU callback handoffs.  This event is intended
// to assist debugging of these handoffs.
//
// The first argument is the name of the RCU flavor, and the second is
// the number of the offloaded CPU are extracted.  The third and final
// argument is a string as follows:
//
// "AlreadyAwake": The to-be-awakened rcuo kthread is already awake.
// "Bypass": rcuo GP kthread sees non-empty ->nocb_bypass.
// "CBSleep": rcuo CB kthread sleeping waiting for CBs.
// "Check": rcuo GP kthread checking specified CPU for work.
// "DeferredWake": Timer expired or polled check, time to wake.
// "DoWake": The to-be-awakened rcuo kthread needs to be awakened.
// "EndSleep": Done waiting for GP for !rcu_nocb_poll.
// "FirstBQ": New CB to empty ->nocb_bypass (->cblist maybe non-empty).
// "FirstBQnoWake": FirstBQ plus rcuo kthread need not be awakened.
// "FirstBQwake": FirstBQ plus rcuo kthread must be awakened.
// "FirstQ": New CB to empty ->cblist (->nocb_bypass maybe non-empty).
// "NeedWaitGP": rcuo GP kthread must wait on a grace period.
// "Poll": Start of new polling cycle for rcu_nocb_poll.
// "Sleep": Sleep waiting for GP for !rcu_nocb_poll.
// "Timer": Deferred-wake timer expired.
// "WakeEmptyIsDeferred": Wake rcuo kthread later, first CB to empty list.
// "WakeEmpty": Wake rcuo kthread, first CB to empty list.
// "WakeNot": Don't wake rcuo kthread.
// "WakeNotPoll": Don't wake rcuo kthread because it is polling.
// "WakeOvfIsDeferred": Wake rcuo kthread later, CB list is huge.
// "WakeBypassIsDeferred": Wake rcuo kthread later, bypass list is contended.
// "WokeEmpty": rcuo CB kthread woke to find empty list.
//

//
// Tracepoint for tasks blocking within preemptible-RCU read-side
// critical sections.  Track the type of RCU (which one day might
// include SRCU), the grace-period number that the task is blocking
// (the current or the next), and the task's PID.
//
// Tracepoint for tasks that blocked within a given preemptible-RCU
// read-side critical section exiting that critical section.  Track the
// type of RCU (which one day might include SRCU) and the task's PID.
//
// Tracepoint for quiescent-state-reporting events.  These are
// distinguished by the type of RCU, the grace-period number, the
// mask of quiescent lower-level entities, the rcu_node structure level,
// the starting and ending CPU covered by the rcu_node structure, and
// whether there are any blocked tasks blocking the current grace period.
// All but the type of RCU are extracted from the rcu_node structure.
//
// Tracepoint for quiescent states detected by force_quiescent_state().
// These trace events include the type of RCU, the grace-period number
// that was blocked by the CPU, the CPU itself, and the type of quiescent
// state, which can be "dti" for dyntick-idle mode or "kick" when kicking
// a CPU that has been in dyntick-idle mode for too long.
//
// Tracepoint for RCU stall events. Takes a string identifying the RCU flavor
// and a string identifying which function detected the RCU stall as follows:
//
// "StallDetected": Scheduler-tick detects other CPU's stalls.
// "SelfDetected": Scheduler-tick detects a current CPU's stall.
// "ExpeditedStall": Expedited grace period detects stalls.
//

//
// Tracepoint for dyntick-idle entry/exit events.  These take 2 strings
// as argument:
// polarity: "Start", "End", "StillWatching" for entering, exiting or still not
// being in EQS mode.
// context: "USER" or "IDLE" or "IRQ".
// NMIs nested in IRQs are inferred with nesting > 1 in IRQ context.
//
// These events also take a pair of numbers, which indicate the nesting
// depth before and after the event of interest, and a third number that is
// the RCU_WATCHING counter.  Note that task-related and interrupt-related
// events use two separate counters, and that the "++=" and "--=" events
// for irq/NMI will change the counter by two, otherwise by one.
//
// Tracepoint for the registration of a single RCU callback function.
// The first argument is the type of RCU, the second argument is
// a pointer to the RCU callback itself, the third element is the
// number of lazy callbacks queued, and the fourth element is the
// total number of callbacks queued.
//
// Tracepoint for marking the beginning rcu_do_batch, performed to start
// RCU callback invocation.  The first argument is the RCU flavor,
// the second is the number of lazy callbacks queued, the third is
// the total number of callbacks queued, and the fourth argument is
// the current RCU-callback batch limit.
//
// Tracepoint for the invocation of a single RCU callback function.
// The first argument is the type of RCU, and the second argument is
// a pointer to the RCU callback itself.
//
// Tracepoint for the invocation of a single RCU callback of the special
// kvfree() form.  The first argument is the RCU flavor, the second
// argument is a pointer to the RCU callback, and the third argument
// is the offset of the callback within the enclosing RCU-protected
// data structure.
//
// Tracepoint for the invocation of a single RCU callback of the special
// kfree_bulk() form. The first argument is the RCU flavor, the second
// argument is a number of elements in array to free, the third is an
// address of the array holding nr_records entries.
//
// Tracepoint for a normal synchronize_rcu() states. The first argument
// is the RCU flavor, the second argument is a pointer to rcu_head the
// last one is an event.
//
// Tracepoint for exiting rcu_do_batch after RCU callbacks have been
// invoked.  The first argument is the name of the RCU flavor,
// the second argument is number of callbacks actually invoked,
// the third argument (cb) is whether or not any of the callbacks that
// were ready to invoke at the beginning of this batch are still
// queued, the fourth argument (nr) is the return value of need_resched(),
// the fifth argument (iit) is 1 if the current task is the idle task,
// and the sixth argument (risk) is the return value from
// rcu_is_callbacks_kthread().
//
// Tracepoint for rcutorture readers.  The first argument is the name
// of the RCU flavor from rcutorture's viewpoint and the second argument
// is the callback address.  The third argument is the start time in
// seconds, and the last two arguments are the grace period numbers
// at the beginning and end of the read, respectively.  Note that the
// callback address can be NULL.
//
pub const RCUTORTURENAME_LEN: c_int = 8;
//
// Tracepoint for rcu_barrier() execution.  The string "s" describes
// the rcu_barrier phase:
// "Begin": rcu_barrier() started.
// "CB": An rcu_barrier_callback() invoked a callback, not the last.
// "EarlyExit": rcu_barrier() piggybacked, thus early exit.
// "Inc1": rcu_barrier() piggyback check counter incremented.
// "Inc2": rcu_barrier() piggyback check counter incremented.
// "IRQ": An rcu_barrier_callback() callback posted on remote CPU.
// "IRQNQ": An rcu_barrier_callback() callback found no callbacks.
// "LastCB": An rcu_barrier_callback() invoked the last callback.
// "NQ": rcu_barrier() found a CPU with no callbacks.
// "OnlineQ": rcu_barrier() found online CPU with callbacks.
// The "cpu" argument is the CPU or -1 if meaningless, the "cnt" argument
// is the count of remaining callbacks, and "done" is the piggybacking count.
//

// This part must be outside protection
