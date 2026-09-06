//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/notifier.h
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
// Routines to manage notifier chains for passing status changes to any
// interested routines. We need this instead of hard coded call lists so
// that modules can poke their nose into the innards. The network devices
// needed them so here they are for the rest of you.
//
// Alan Cox <Alan.Cox@linux.org>
//

//
// Notifier chains are of four types:
//
// Atomic notifier chains: Chain callbacks run in interrupt/atomic
// context. Callouts are not allowed to block.
// Blocking notifier chains: Chain callbacks run in process context.
// Callouts are allowed to block.
// Raw notifier chains: There are no restrictions on callbacks,
// registration, or unregistration.  All locking and protection
// must be provided by the caller.
// SRCU notifier chains: A variant of blocking notifier chains, with
// the same restrictions.
//
// atomic_notifier_chain_register() may be called from an atomic context,
// but blocking_notifier_chain_register() and srcu_notifier_chain_register()
// must be called from a process context.  Ditto for the corresponding
// _unregister() routines.
//
// atomic_notifier_chain_unregister(), blocking_notifier_chain_unregister(),
// and srcu_notifier_chain_unregister() _must not_ be called from within
// the call chain.
//
// SRCU notifier chains are an alternative form of blocking notifier chains.
// They use SRCU (Sleepable Read-Copy Update) instead of rw-semaphores for
// protection of the chain links.  This means there is _very_ low overhead
// in srcu_notifier_call_chain(): no cache bounces and no memory barriers.
// As compensation, srcu_notifier_chain_unregister() is rather expensive.
// SRCU notifier chains should be used when the chain will be called very
// often but notifier_blocks will seldom be removed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct notifier_block {
    pub notifier_call: notifier_fn_t,
    pub next: *mut notifier_block __rcu,
    pub priority: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atomic_notifier_head {
    pub lock: spinlock_t,
    pub head: *mut notifier_block __rcu,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct blocking_notifier_head {
    pub rwsem: rw_semaphore,
    pub head: *mut notifier_block __rcu,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_notifier_head {
    pub head: *mut notifier_block __rcu,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct srcu_notifier_head {
    pub mutex: mutex,
    pub srcuu: srcu_usage,
    pub srcu: srcu_struct,
    pub head: *mut notifier_block __rcu,
}

// srcu_notifier_heads must be cleaned up dynamically
extern "C" {
    pub fn srcu_init_notifier_head(nh: *mut srcu_notifier_head);
}

extern "C" {
    pub fn atomic_notifier_call_chain_is_empty(nh: *mut atomic_notifier_head) -> bool;
}
pub const NOTIFY_DONE: c_uint = 0x0000		/* Don't care */;
pub const NOTIFY_OK: c_uint = 0x0001		/* Suits me */;
pub const NOTIFY_STOP_MASK: c_uint = 0x8000		/* Don't call further */;

// Bad/Veto action
//
// Clean way to return from the notifier and stop further calls.
//

// Encapsulate (negative) errno value (in particular, NOTIFY_BAD <=> EPERM).
// Restore (negative) errno value from notify return value.
//
// Declared notifiers so far. I can imagine quite a few more chains
// over time (eg laptop power reset chains, reboot chain (to clean
// device units up), device [un]mount chain, module load/unload chain,
// low memory chain, screenblank chain (for plug in modular screenblankers)
// VC switch chains (for loadable kernel svgalib VC switch helpers) etc...
//
// CPU notfiers are defined in include/linux/cpu.h.
// netdevice notifiers are defined in include/linux/netdevice.h
// reboot notifiers are defined in include/linux/reboot.h.
// Hibernation and suspend events are defined in include/linux/suspend.h.
// Virtual Terminal events are defined in include/linux/vt.h.
pub const NETLINK_URELEASE: c_uint = 0x0001	/* Unicast netlink socket released */;
// Console keyboard events.
// Note: KBD_KEYCODE is always sent before KBD_UNBOUND_KEYCODE, KBD_UNICODE and
// KBD_KEYSYM.
pub const KBD_KEYCODE: c_uint = 0x0001 /* Keyboard keycode, called before any other */;
pub const KBD_UNBOUND_KEYCODE: c_uint = 0x0002 /* Keyboard keycode which is not bound to any other */;
pub const KBD_UNICODE: c_uint = 0x0003 /* Keyboard unicode */;
pub const KBD_KEYSYM: c_uint = 0x0004 /* Keyboard keysym */;
pub const KBD_POST_KEYSYM: c_uint = 0x0005 /* Called after keyboard keysym interpretation */;

