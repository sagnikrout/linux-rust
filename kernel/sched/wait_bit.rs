//! Automatically rewritten from C to Rust
//! Source: kernel/sched/wait_bit.c
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
// The implementation of the wait_bit*() and related waiting APIs:
//
pub const WAIT_TABLE_BITS: c_int = 8;

    static wait_queue_head_t bit_wait_table[WAIT_TABLE_SIZE] __cacheline_aligned;
    wait_queue_head_t *bit_waitqueue(unsigned long *word, int bit)
    {
    let mut shift: c_int = BITS_PER_LONG == 32 ? 5 : 6;
    let mut val: c_ulong = (unsigned long)word << shift | bit;
    return bit_wait_table + hash_long(val, WAIT_TABLE_BITS);
    }
    EXPORT_SYMBOL(bit_waitqueue);
#[no_mangle]
pub unsafe extern "C" fn wake_bit_function(wq_entry: *mut wait_queue_entry, mode: unsigned, sync: c_int, arg: *mut c_void) -> c_int {
    int wake_bit_function(struct wait_queue_entry *wq_entry, unsigned mode, int sync, void *arg)
    {
    struct wait_bit_key *key = arg;
    struct wait_bit_queue_entry *wait_bit = container_of(wq_entry, struct wait_bit_queue_entry, wq_entry);
    if (wait_bit.key.flags != key.flags ||
    wait_bit.key.bit_nr != key.bit_nr ||
    test_bit(key.bit_nr, key.flags))
    return 0;
    return autoremove_wake_function(wq_entry, mode, sync, key);
    }
    EXPORT_SYMBOL(wake_bit_function);
//
// To allow interruptible waiting and asynchronous (i.e. non-blocking)
// waiting, the actions of __wait_on_bit() and __wait_on_bit_lock() are
// permitted return codes. Nonzero return codes halt waiting and return.
//
    int __sched
    __wait_on_bit(struct wait_queue_head *wq_head, struct wait_bit_queue_entry *wbq_entry,
    wait_bit_action_f *action, unsigned mode)
    {
    let mut ret: c_int = 0;
    do {
    prepare_to_wait(wq_head, &wbq_entry.wq_entry, mode);
    if (test_bit(wbq_entry.key.bit_nr, wbq_entry.key.flags))
    ret = (*action)(&wbq_entry.key, mode);
    } while (test_bit_acquire(wbq_entry.key.bit_nr, wbq_entry.key.flags) && !ret);
    finish_wait(wq_head, &wbq_entry.wq_entry);
    return ret;
    }
    EXPORT_SYMBOL(__wait_on_bit);
    int __sched out_of_line_wait_on_bit(unsigned long *word, int bit,
    wait_bit_action_f *action, unsigned mode)
    {
    struct wait_queue_head *wq_head = bit_waitqueue(word, bit);
    DEFINE_WAIT_BIT(wq_entry, word, bit);
    return __wait_on_bit(wq_head, &wq_entry, action, mode);
    }
    EXPORT_SYMBOL(out_of_line_wait_on_bit);
    int __sched out_of_line_wait_on_bit_timeout(
    unsigned long *word, int bit, wait_bit_action_f *action,
    unsigned mode, unsigned long timeout)
    {
    struct wait_queue_head *wq_head = bit_waitqueue(word, bit);
    DEFINE_WAIT_BIT(wq_entry, word, bit);
    wq_entry.key.timeout = jiffies + timeout;
    return __wait_on_bit(wq_head, &wq_entry, action, mode);
    }
    EXPORT_SYMBOL_GPL(out_of_line_wait_on_bit_timeout);
    int __sched
    __wait_on_bit_lock(struct wait_queue_head *wq_head, struct wait_bit_queue_entry *wbq_entry,
    wait_bit_action_f *action, unsigned mode)
    {
    let mut ret: c_int = 0;
    for (;;) {
    prepare_to_wait_exclusive(wq_head, &wbq_entry.wq_entry, mode);
    if (test_bit(wbq_entry.key.bit_nr, wbq_entry.key.flags)) {
    ret = action(&wbq_entry.key, mode);
//
// See the comment in prepare_to_wait_event().
// finish_wait() does not necessarily takes wwq_head->lock,
// but test_and_set_bit() implies mb() which pairs with
// smp_mb__after_atomic() before wake_up_page().
//
    if (ret)
    finish_wait(wq_head, &wbq_entry.wq_entry);
    }
    if (!test_and_set_bit(wbq_entry.key.bit_nr, wbq_entry.key.flags)) {
    if (!ret)
    finish_wait(wq_head, &wbq_entry.wq_entry);
    return 0;
    } else if (ret) {
    return ret;
    }
    }
    }
    EXPORT_SYMBOL(__wait_on_bit_lock);
    int __sched out_of_line_wait_on_bit_lock(unsigned long *word, int bit,
    wait_bit_action_f *action, unsigned mode)
    {
    struct wait_queue_head *wq_head = bit_waitqueue(word, bit);
    DEFINE_WAIT_BIT(wq_entry, word, bit);
    return __wait_on_bit_lock(wq_head, &wq_entry, action, mode);
    }
    EXPORT_SYMBOL(out_of_line_wait_on_bit_lock);
#[no_mangle]
pub unsafe extern "C" fn __wake_up_bit(wq_head: *mut wait_queue_head, word: *mut c_ulong, bit: c_int) {
    void __wake_up_bit(struct wait_queue_head *wq_head, unsigned long *word, int bit)
    {
    let mut key: wait_bit_key = __WAIT_BIT_KEY_INITIALIZER(word, bit);
    if (waitqueue_active(wq_head))
    __wake_up(wq_head, TASK_NORMAL, 1, &key);
    }
    EXPORT_SYMBOL(__wake_up_bit);
//
// wake_up_bit - wake up waiters on a bit
// @word: the address containing the bit being waited on
// @bit: the bit at that address being waited on
//
// Wake up any process waiting in wait_on_bit() or similar for the
// given bit to be cleared.
//
// The wake-up is sent to tasks in a waitqueue selected by hash from a
// shared pool.  Only those tasks on that queue which have requested
// wake_up on this specific address and bit will be woken, and only if the
// bit is clear.
//
// In order for this to function properly there must be a full memory
// barrier after the bit is cleared and before this function is called.
// If the bit was cleared atomically, such as a by clear_bit() then
// smb_mb__after_atomic() can be used, othwewise smb_mb() is needed.
// If the bit was cleared with a fully-ordered operation, no further
// barrier is required.
//
// Normally the bit should be cleared by an operation with RELEASE
// semantics so that any changes to memory made before the bit is
// cleared are guaranteed to be visible after the matching wait_on_bit()
// completes.
//
#[no_mangle]
pub unsafe extern "C" fn wake_up_bit(word: *mut c_ulong, bit: c_int) {
    void wake_up_bit(unsigned long *word, int bit)
    {
    __wake_up_bit(bit_waitqueue(word, bit), word, bit);
    }
    EXPORT_SYMBOL(wake_up_bit);
    wait_queue_head_t *__var_waitqueue(void *p)
    {
    return bit_wait_table + hash_ptr(p, WAIT_TABLE_BITS);
    }
    EXPORT_SYMBOL(__var_waitqueue);
    struct wait_bit_key *__var_wake_key(struct wait_queue_entry *wq_entry, void *arg)
    {
    struct wait_bit_key *key = arg;
    struct wait_bit_queue_entry *wbq_entry =
    container_of(wq_entry, struct wait_bit_queue_entry, wq_entry);
    if (wbq_entry.key.flags != key.flags ||
    wbq_entry.key.bit_nr != key.bit_nr)
    return core::ptr::null_mut();
    return key;
    }
    static int var_wake_function(struct wait_queue_entry *wq_entry, unsigned int mode,
    int sync, void *arg)
    {
    struct wait_bit_key *key = __var_wake_key(wq_entry, arg);
    if (!key)
    return 0;
    return autoremove_wake_function(wq_entry, mode, sync, key);
    }
#[no_mangle]
pub unsafe extern "C" fn init_wait_var_entry(wbq_entry: *mut wait_bit_queue_entry, var: *mut c_void, flags: c_int) {
    void init_wait_var_entry(struct wait_bit_queue_entry *wbq_entry, void *var, int flags)
    {
// wbq_entry = (struct wait_bit_queue_entry){
    .key = {
    .flags	= (var),
    .bit_nr = -1,
    },
    .wq_entry = {
    .flags	 = flags,
    .private = current,
    .func	 = var_wake_function,
    .entry	 = LIST_HEAD_INIT(wbq_entry.wq_entry.entry),
    },
    };
    }
    EXPORT_SYMBOL(init_wait_var_entry);
//
// wake_up_var - wake up waiters on a variable (kernel address)
// @var: the address of the variable being waited on
//
// Wake up any process waiting in wait_var_event() or similar for the
// given variable to change.  wait_var_event() can be waiting for an
// arbitrary condition to be true and associates that condition with an
// address.  Calling wake_up_var() suggests that the condition has been
// made true, but does not strictly require the condtion to use the
// address given.
//
// The wake-up is sent to tasks in a waitqueue selected by hash from a
// shared pool.  Only those tasks on that queue which have requested
// wake_up on this specific address will be woken.
//
// In order for this to function properly there must be a full memory
// barrier after the variable is updated (or more accurately, after the
// condition waited on has been made to be true) and before this function
// is called.  If the variable was updated atomically, such as a by
// atomic_dec() then smb_mb__after_atomic() can be used.  If the
// variable was updated by a fully ordered operation such as
// atomic_dec_and_test() then no extra barrier is required.  Otherwise
// smb_mb() is needed.
//
// Normally the variable should be updated (the condition should be made
// to be true) by an operation with RELEASE semantics such as
// smp_store_release() so that any changes to memory made before the
// variable was updated are guaranteed to be visible after the matching
// wait_var_event() completes.
//
#[no_mangle]
pub unsafe extern "C" fn wake_up_var(var: *mut c_void) {
    void wake_up_var(void *var)
    {
    __wake_up_bit(__var_waitqueue(var), var, -1);
    }
    EXPORT_SYMBOL(wake_up_var);
#[no_mangle]
pub unsafe extern "C" fn bit_wait(word: *mut wait_bit_key, mode: c_int) -> __sched int {
    __sched int bit_wait(struct wait_bit_key *word, int mode)
    {
    schedule();
    if (signal_pending_state(mode, current))
    return -EINTR;
    return 0;
    }
    EXPORT_SYMBOL(bit_wait);
#[no_mangle]
pub unsafe extern "C" fn bit_wait_io(word: *mut wait_bit_key, mode: c_int) -> __sched int {
    __sched int bit_wait_io(struct wait_bit_key *word, int mode)
    {
    io_schedule();
    if (signal_pending_state(mode, current))
    return -EINTR;
    return 0;
    }
    EXPORT_SYMBOL(bit_wait_io);
#[no_mangle]
pub unsafe extern "C" fn bit_wait_timeout(word: *mut wait_bit_key, mode: c_int) -> __sched int {
    __sched int bit_wait_timeout(struct wait_bit_key *word, int mode)
    {
    let mut now: c_ulong = READ_ONCE(jiffies);
    if (time_after_eq(now, word.timeout))
    return -EAGAIN;
    schedule_timeout(word.timeout - now);
    if (signal_pending_state(mode, current))
    return -EINTR;
    return 0;
    }
    EXPORT_SYMBOL_GPL(bit_wait_timeout);
#[no_mangle]
pub unsafe extern "C" fn wait_bit_init() -> void __init {
    void __init wait_bit_init(void)
    {
    int i;
    for (i = 0; i < WAIT_TABLE_SIZE; i++)
    init_waitqueue_head(bit_wait_table + i);
    }
