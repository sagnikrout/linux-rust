//! Automatically rewritten from C to Rust
//! Source: net/sunrpc/sched.c
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
// linux/net/sunrpc/sched.c
//
// Scheduling for synchronous and asynchronous RPC requests.
//
// Copyright (C) 1996 Olaf Kirch, <okir@monad.swb.de>
//
// TCP NFS related read + write fixes
// (C) 1999 Dave Airlie, University of Limerick, Ireland <airlied@linux.ie>
//

// Macro flag: #define CREATE_TRACE_POINTS

//
// RPC slabs and memory pools
//

    static struct kmem_cache	*rpc_task_slabp __read_mostly;
    static struct kmem_cache	*rpc_buffer_slabp __read_mostly;
    static mempool_t	*rpc_task_mempool __read_mostly;
    static mempool_t	*rpc_buffer_mempool __read_mostly;
    static void			rpc_async_schedule(struct work_struct *);
    static void			 rpc_release_task(struct rpc_task *task);
    static void __rpc_queue_timer_fn(struct work_struct *);
//
// RPC tasks sit here while waiting for conditions to improve.
//
    static struct rpc_wait_queue delay_queue;
//
// rpciod-related stuff
//
    struct workqueue_struct *rpciod_workqueue __read_mostly;
    struct workqueue_struct *xprtiod_workqueue __read_mostly;
    EXPORT_SYMBOL_GPL(xprtiod_workqueue);
#[no_mangle]
pub unsafe extern "C" fn rpc_task_gfp_mask() -> gfp_t {
    gfp_t rpc_task_gfp_mask(void)
    {
    if (current.flags & PF_WQ_WORKER)
    return GFP_KERNEL | __GFP_NORETRY | __GFP_NOWARN;
    return GFP_KERNEL;
    }
    EXPORT_SYMBOL_GPL(rpc_task_gfp_mask);
#[no_mangle]
pub unsafe extern "C" fn rpc_task_set_rpc_status(task: *mut rpc_task, rpc_status: c_int) -> bool {
    bool rpc_task_set_rpc_status(struct rpc_task *task, int rpc_status)
    {
    if (cmpxchg(&task.tk_rpc_status, 0, rpc_status) == 0)
    return true;
    return false;
    }
    unsigned long
    rpc_task_timeout(const struct rpc_task *task)
    {
    let mut timeout: c_ulong = READ_ONCE(task.tk_timeout);
    if (timeout != 0) {
    let mut now: c_ulong = jiffies;
    if (time_before(now, timeout))
    return timeout - now;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(rpc_task_timeout);
//
// Disable the timer for a given RPC task. Should be called with
// queue->lock and bh_disabled in order to avoid races within
// rpc_run_timer().
//
    static void
    __rpc_disable_timer(struct rpc_wait_queue *queue, struct rpc_task *task)
    {
    if (list_empty(&task.u.tk_wait.timer_list))
    return;
    task.tk_timeout = 0;
    list_del(&task.u.tk_wait.timer_list);
    if (list_empty(&queue.timer_list.list))
    cancel_delayed_work(&queue.timer_list.dwork);
    }
    static void
    rpc_set_queue_timer(struct rpc_wait_queue *queue, unsigned long expires)
    {
    let mut now: c_ulong = jiffies;
    queue.timer_list.expires = expires;
    if (time_before_eq(expires, now))
    expires = 0;
    else
    expires -= now;
    mod_delayed_work(rpciod_workqueue, &queue.timer_list.dwork, expires);
    }
//
// Set up a timer for the current task.
//
    static void
    __rpc_add_timer(struct rpc_wait_queue *queue, struct rpc_task *task,
    unsigned long timeout)
    {
    task.tk_timeout = timeout;
    if (list_empty(&queue.timer_list.list) || time_before(timeout, queue.timer_list.expires))
    rpc_set_queue_timer(queue, timeout);
    list_add(&task.u.tk_wait.timer_list, &queue.timer_list.list);
    }
#[no_mangle]
unsafe extern "C" fn rpc_set_waitqueue_priority(queue: *mut rpc_wait_queue, priority: c_int) {
    static void rpc_set_waitqueue_priority(struct rpc_wait_queue *queue, int priority)
    {
    if (queue.priority != priority) {
    queue.priority = priority;
    queue.nr = 1U << priority;
    }
    }
#[no_mangle]
unsafe extern "C" fn rpc_reset_waitqueue_priority(queue: *mut rpc_wait_queue) {
    static void rpc_reset_waitqueue_priority(struct rpc_wait_queue *queue)
    {
    rpc_set_waitqueue_priority(queue, queue.maxpriority);
    }
//
// Add a request to a queue list
//
    static void
    __rpc_list_enqueue_task(struct list_head *q, struct rpc_task *task)
    {
    struct rpc_task *t;
    list_for_each_entry(t, q, u.tk_wait.list) {
    if (t.tk_owner == task.tk_owner) {
    list_add_tail(&task.u.tk_wait.links,
    &t.u.tk_wait.links);
// Cache the queue head in task->u.tk_wait.list
    task.u.tk_wait.list.next = q;
    task.u.tk_wait.list.prev = core::ptr::null_mut();
    return;
    }
    }
    INIT_LIST_HEAD(&task.u.tk_wait.links);
    list_add_tail(&task.u.tk_wait.list, q);
    }
//
// Remove request from a queue list
//
    static void
    __rpc_list_dequeue_task(struct rpc_task *task)
    {
    struct list_head *q;
    struct rpc_task *t;
    if (task.u.tk_wait.list.prev == core::ptr::null_mut()) {
    list_del(&task.u.tk_wait.links);
    return;
    }
    if (!list_empty(&task.u.tk_wait.links)) {
    t = list_first_entry(&task.u.tk_wait.links,
    struct rpc_task,
    u.tk_wait.links);
// Assume __rpc_list_enqueue_task() cached the queue head
    q = t.u.tk_wait.list.next;
    list_add_tail(&t.u.tk_wait.list, q);
    list_del(&task.u.tk_wait.links);
    }
    list_del(&task.u.tk_wait.list);
    }
//
// Add new request to a priority queue.
//
    static void __rpc_add_wait_queue_priority(struct rpc_wait_queue *queue,
    struct rpc_task *task,
    unsigned char queue_priority)
    {
    if (unlikely(queue_priority > queue.maxpriority))
    queue_priority = queue.maxpriority;
    __rpc_list_enqueue_task(&queue.tasks[queue_priority], task);
    }
//
// Add new request to wait queue.
//
    static void __rpc_add_wait_queue(struct rpc_wait_queue *queue,
    struct rpc_task *task,
    unsigned char queue_priority)
    {
    INIT_LIST_HEAD(&task.u.tk_wait.timer_list);
    if (RPC_IS_PRIORITY(queue))
    __rpc_add_wait_queue_priority(queue, task, queue_priority);
    else
    list_add_tail(&task.u.tk_wait.list, &queue.tasks[0]);
    task.tk_waitqueue = queue;
    queue.qlen++;
// barrier matches the read in rpc_wake_up_task_queue_locked()
    smp_wmb();
    rpc_set_queued(task);
    }
//
// Remove request from a priority queue.
//
#[no_mangle]
unsafe extern "C" fn __rpc_remove_wait_queue_priority(task: *mut rpc_task) {
    static void __rpc_remove_wait_queue_priority(struct rpc_task *task)
    {
    __rpc_list_dequeue_task(task);
    }
//
// Remove request from queue.
// Note: must be called with spin lock held.
//
#[no_mangle]
unsafe extern "C" fn __rpc_remove_wait_queue(queue: *mut rpc_wait_queue, task: *mut rpc_task) {
    static void __rpc_remove_wait_queue(struct rpc_wait_queue *queue, struct rpc_task *task)
    {
    __rpc_disable_timer(queue, task);
    if (RPC_IS_PRIORITY(queue))
    __rpc_remove_wait_queue_priority(task);
    else
    list_del(&task.u.tk_wait.list);
    queue.qlen--;
    }
#[no_mangle]
unsafe extern "C" fn __rpc_init_priority_wait_queue(queue: *mut rpc_wait_queue, qname: *const c_char, nr_queues: c_uchar) {
    static void __rpc_init_priority_wait_queue(struct rpc_wait_queue *queue, const char *qname, unsigned char nr_queues)
    {
    int i;
    spin_lock_init(&queue.lock);
    for (i = 0; i < ARRAY_SIZE(queue.tasks); i++)
    INIT_LIST_HEAD(&queue.tasks[i]);
    queue.maxpriority = nr_queues - 1;
    rpc_reset_waitqueue_priority(queue);
    queue.qlen = 0;
    queue.timer_list.expires = 0;
    INIT_DELAYED_WORK(&queue.timer_list.dwork, __rpc_queue_timer_fn);
    INIT_LIST_HEAD(&queue.timer_list.list);
    rpc_assign_waitqueue_name(queue, qname);
    }
#[no_mangle]
pub unsafe extern "C" fn rpc_init_priority_wait_queue(queue: *mut rpc_wait_queue, qname: *const c_char) {
    void rpc_init_priority_wait_queue(struct rpc_wait_queue *queue, const char *qname)
    {
    __rpc_init_priority_wait_queue(queue, qname, RPC_NR_PRIORITY);
    }
    EXPORT_SYMBOL_GPL(rpc_init_priority_wait_queue);
#[no_mangle]
pub unsafe extern "C" fn rpc_init_wait_queue(queue: *mut rpc_wait_queue, qname: *const c_char) {
    void rpc_init_wait_queue(struct rpc_wait_queue *queue, const char *qname)
    {
    __rpc_init_priority_wait_queue(queue, qname, 1);
    }
    EXPORT_SYMBOL_GPL(rpc_init_wait_queue);
#[no_mangle]
pub unsafe extern "C" fn rpc_destroy_wait_queue(queue: *mut rpc_wait_queue) {
    void rpc_destroy_wait_queue(struct rpc_wait_queue *queue)
    {
    cancel_delayed_work_sync(&queue.timer_list.dwork);
    }
    EXPORT_SYMBOL_GPL(rpc_destroy_wait_queue);
#[no_mangle]
unsafe extern "C" fn rpc_wait_bit_killable(key: *mut wait_bit_key, mode: c_int) -> c_int {
    static int rpc_wait_bit_killable(struct wait_bit_key *key, int mode)
    {
    schedule();
    if (signal_pending_state(mode, current))
    return -ERESTARTSYS;
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn rpc_task_set_debuginfo(task: *mut rpc_task) {
    static void rpc_task_set_debuginfo(struct rpc_task *task)
    {
    struct rpc_clnt *clnt = task.tk_client;
// Might be a task carrying a reverse-direction operation
    if (!clnt) {
    static atomic_t rpc_pid;
    task.tk_pid = atomic_inc_return(&rpc_pid);
    return;
    }
    task.tk_pid = atomic_inc_return(&clnt.cl_pid);
    }

#[no_mangle]
pub unsafe extern "C" fn rpc_task_set_debuginfo(task: *mut rpc_task) {
    static inline void rpc_task_set_debuginfo(struct rpc_task *task)
    {
    }

#[no_mangle]
unsafe extern "C" fn rpc_set_active(task: *mut rpc_task) {
    static void rpc_set_active(struct rpc_task *task)
    {
    rpc_task_set_debuginfo(task);
    set_bit(RPC_TASK_ACTIVE, &task.tk_runstate);
    trace_rpc_task_begin(task, core::ptr::null_mut());
    }
//
// Mark an RPC call as having completed by clearing the 'active' bit
// and then waking up all tasks that were sleeping.
//
#[no_mangle]
unsafe extern "C" fn rpc_complete_task(task: *mut rpc_task) -> c_int {
    static int rpc_complete_task(struct rpc_task *task)
    {
    void *m = &task.tk_runstate;
    wait_queue_head_t *wq = bit_waitqueue(m, RPC_TASK_ACTIVE);
    let mut k: wait_bit_key = __WAIT_BIT_KEY_INITIALIZER(m, RPC_TASK_ACTIVE);
    unsigned long flags;
    int ret;
    trace_rpc_task_complete(task, core::ptr::null_mut());
    spin_lock_irqsave(&wq.lock, flags);
    clear_bit(RPC_TASK_ACTIVE, &task.tk_runstate);
    ret = atomic_dec_and_test(&task.tk_count);
    if (waitqueue_active(wq))
    __wake_up_locked_key(wq, TASK_NORMAL, &k);
    spin_unlock_irqrestore(&wq.lock, flags);
    return ret;
    }
//
// Allow callers to wait for completion of an RPC call
//
// Note the use of out_of_line_wait_on_bit() rather than wait_on_bit()
// to enforce taking of the wq->lock and hence avoid races with
// rpc_complete_task().
//
#[no_mangle]
pub unsafe extern "C" fn rpc_wait_for_completion_task(task: *mut rpc_task) -> c_int {
    int rpc_wait_for_completion_task(struct rpc_task *task)
    {
    return out_of_line_wait_on_bit(&task.tk_runstate, RPC_TASK_ACTIVE,
    rpc_wait_bit_killable, TASK_KILLABLE|TASK_FREEZABLE_UNSAFE);
    }
    EXPORT_SYMBOL_GPL(rpc_wait_for_completion_task);
//
// Make an RPC task runnable.
//
// Note: If the task is ASYNC, and is being made runnable after sitting on an
// rpc_wait_queue, this must be called with the queue spinlock held to protect
// the wait queue operation.
// Note the ordering of rpc_test_and_set_running() and rpc_clear_queued(),
// which is needed to ensure that __rpc_execute() doesn't loop (due to the
// lockless RPC_IS_QUEUED() test) before we've had a chance to test
// the RPC_TASK_RUNNING flag.
//
    static void rpc_make_runnable(struct workqueue_struct *wq,
    struct rpc_task *task)
    {
    let mut need_wakeup: bool = !rpc_test_and_set_running(task);
    rpc_clear_queued(task);
    if (!need_wakeup)
    return;
    if (RPC_IS_ASYNC(task)) {
    INIT_WORK(&task.u.tk_work, rpc_async_schedule);
    queue_work(wq, &task.u.tk_work);
    } else {
    smp_mb__after_atomic();
    wake_up_bit(&task.tk_runstate, RPC_TASK_QUEUED);
    }
    }
//
// Prepare for sleeping on a wait queue.
// By always appending tasks to the list we ensure FIFO behavior.
// NB: An RPC task will only receive interrupt-driven events as long
// as it's on a wait queue.
//
    static void __rpc_do_sleep_on_priority(struct rpc_wait_queue *q,
    struct rpc_task *task,
    unsigned char queue_priority)
    {
    trace_rpc_task_sleep(task, q);
    __rpc_add_wait_queue(q, task, queue_priority);
    }
    static void __rpc_sleep_on_priority(struct rpc_wait_queue *q,
    struct rpc_task *task,
    unsigned char queue_priority)
    {
    if (WARN_ON_ONCE(RPC_IS_QUEUED(task)))
    return;
    __rpc_do_sleep_on_priority(q, task, queue_priority);
    }
    static void __rpc_sleep_on_priority_timeout(struct rpc_wait_queue *q,
    struct rpc_task *task, unsigned long timeout,
    unsigned char queue_priority)
    {
    if (WARN_ON_ONCE(RPC_IS_QUEUED(task)))
    return;
    if (time_is_after_jiffies(timeout)) {
    __rpc_do_sleep_on_priority(q, task, queue_priority);
    __rpc_add_timer(q, task, timeout);
    } else
    task.tk_status = -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn rpc_set_tk_callback(task: *mut rpc_task, action: rpc_action) {
    static void rpc_set_tk_callback(struct rpc_task *task, rpc_action action)
    {
    if (action && !WARN_ON_ONCE(task.tk_callback != core::ptr::null_mut()))
    task.tk_callback = action;
    }
#[no_mangle]
unsafe extern "C" fn rpc_sleep_check_activated(task: *mut rpc_task) -> bool {
    static bool rpc_sleep_check_activated(struct rpc_task *task)
    {
// We shouldn't ever put an inactive task to sleep
    if (WARN_ON_ONCE(!RPC_IS_ACTIVATED(task))) {
    task.tk_status = -EIO;
    rpc_put_task_async(task);
    return false;
    }
    return true;
    }
    void rpc_sleep_on_timeout(struct rpc_wait_queue *q, struct rpc_task *task,
    rpc_action action, unsigned long timeout)
    {
    if (!rpc_sleep_check_activated(task))
    return;
    rpc_set_tk_callback(task, action);
//
// Protect the queue operations.
//
    spin_lock(&q.lock);
    __rpc_sleep_on_priority_timeout(q, task, timeout, task.tk_priority);
    spin_unlock(&q.lock);
    }
    EXPORT_SYMBOL_GPL(rpc_sleep_on_timeout);
    void rpc_sleep_on(struct rpc_wait_queue *q, struct rpc_task *task,
    rpc_action action)
    {
    if (!rpc_sleep_check_activated(task))
    return;
    rpc_set_tk_callback(task, action);
    WARN_ON_ONCE(task.tk_timeout != 0);
//
// Protect the queue operations.
//
    spin_lock(&q.lock);
    __rpc_sleep_on_priority(q, task, task.tk_priority);
    spin_unlock(&q.lock);
    }
    EXPORT_SYMBOL_GPL(rpc_sleep_on);
    void rpc_sleep_on_priority_timeout(struct rpc_wait_queue *q,
    struct rpc_task *task, unsigned long timeout, int priority)
    {
    if (!rpc_sleep_check_activated(task))
    return;
    priority -= RPC_PRIORITY_LOW;
//
// Protect the queue operations.
//
    spin_lock(&q.lock);
    __rpc_sleep_on_priority_timeout(q, task, timeout, priority);
    spin_unlock(&q.lock);
    }
    EXPORT_SYMBOL_GPL(rpc_sleep_on_priority_timeout);
    void rpc_sleep_on_priority(struct rpc_wait_queue *q, struct rpc_task *task,
    int priority)
    {
    if (!rpc_sleep_check_activated(task))
    return;
    WARN_ON_ONCE(task.tk_timeout != 0);
    priority -= RPC_PRIORITY_LOW;
//
// Protect the queue operations.
//
    spin_lock(&q.lock);
    __rpc_sleep_on_priority(q, task, priority);
    spin_unlock(&q.lock);
    }
    EXPORT_SYMBOL_GPL(rpc_sleep_on_priority);
//
// __rpc_do_wake_up_task_on_wq - wake up a single rpc_task
// @wq: workqueue on which to run task
// @queue: wait queue
// @task: task to be woken up
//
// Caller must hold queue->lock, and have cleared the task queued flag.
//
    static void __rpc_do_wake_up_task_on_wq(struct workqueue_struct *wq,
    struct rpc_wait_queue *queue,
    struct rpc_task *task)
    {
// Has the task been executed yet? If not, we cannot wake it up!
    if (!RPC_IS_ACTIVATED(task)) {
    printk(KERN_ERR "RPC: Inactive task (%p) being woken up!\n", task);
    return;
    }
    trace_rpc_task_wakeup(task, queue);
    __rpc_remove_wait_queue(queue, task);
    rpc_make_runnable(wq, task);
    }
//
// Wake up a queued task while the queue lock is being held
//
    static struct rpc_task *
    rpc_wake_up_task_on_wq_queue_action_locked(struct workqueue_struct *wq,
    struct rpc_wait_queue *queue, struct rpc_task *task,
    bool (*action)(struct rpc_task *, void *), void *data)
    {
    if (RPC_IS_QUEUED(task)) {
    smp_rmb();
    if (task.tk_waitqueue == queue) {
    if (action == core::ptr::null_mut() || action(task, data)) {
    __rpc_do_wake_up_task_on_wq(wq, queue, task);
    return task;
    }
    }
    }
    return core::ptr::null_mut();
    }
//
// Wake up a queued task while the queue lock is being held
//
    static void rpc_wake_up_task_queue_locked(struct rpc_wait_queue *queue,
    struct rpc_task *task)
    {
    rpc_wake_up_task_on_wq_queue_action_locked(rpciod_workqueue, queue,
    task, core::ptr::null_mut(), core::ptr::null_mut());
    }
//
// Wake up a task on a specific queue
//
#[no_mangle]
pub unsafe extern "C" fn rpc_wake_up_queued_task(queue: *mut rpc_wait_queue, task: *mut rpc_task) {
    void rpc_wake_up_queued_task(struct rpc_wait_queue *queue, struct rpc_task *task)
    {
    if (!RPC_IS_QUEUED(task))
    return;
    spin_lock(&queue.lock);
    rpc_wake_up_task_queue_locked(queue, task);
    spin_unlock(&queue.lock);
    }
    EXPORT_SYMBOL_GPL(rpc_wake_up_queued_task);
#[no_mangle]
unsafe extern "C" fn rpc_task_action_set_status(task: *mut rpc_task, status: *mut c_void) -> bool {
    static bool rpc_task_action_set_status(struct rpc_task *task, void *status)
    {
    task.tk_status = *(int *)status;
    return true;
    }
    static void
    rpc_wake_up_task_queue_set_status_locked(struct rpc_wait_queue *queue,
    struct rpc_task *task, int status)
    {
    rpc_wake_up_task_on_wq_queue_action_locked(rpciod_workqueue, queue,
    task, rpc_task_action_set_status, &status);
    }
//
// rpc_wake_up_queued_task_set_status - wake up a task and set task->tk_status
// @queue: pointer to rpc_wait_queue
// @task: pointer to rpc_task
// @status: integer error value
//
// If @task is queued on @queue, then it is woken up, and @task->tk_status is
// set to the value of @status.
//
    void
    rpc_wake_up_queued_task_set_status(struct rpc_wait_queue *queue,
    struct rpc_task *task, int status)
    {
    if (!RPC_IS_QUEUED(task))
    return;
    spin_lock(&queue.lock);
    rpc_wake_up_task_queue_set_status_locked(queue, task, status);
    spin_unlock(&queue.lock);
    }
//
// Wake up the next task on a priority queue.
//
    static struct rpc_task *__rpc_find_next_queued_priority(struct rpc_wait_queue *queue)
    {
    struct list_head *q;
    struct rpc_task *task;
//
// Service the privileged queue.
//
    q = &queue.tasks[RPC_NR_PRIORITY - 1];
    if (queue.maxpriority > RPC_PRIORITY_PRIVILEGED && !list_empty(q)) {
    task = list_first_entry(q, struct rpc_task, u.tk_wait.list);
    goto out;
    }
//
// Service a batch of tasks from a single owner.
//
    q = &queue.tasks[queue.priority];
    if (!list_empty(q) && queue.nr) {
    queue.nr--;
    task = list_first_entry(q, struct rpc_task, u.tk_wait.list);
    goto out;
    }
//
// Service the next queue.
//
    do {
    if (q == &queue.tasks[0])
    q = &queue.tasks[queue.maxpriority];
    else
    q = q - 1;
    if (!list_empty(q)) {
    task = list_first_entry(q, struct rpc_task, u.tk_wait.list);
    goto new_queue;
    }
    } while (q != &queue.tasks[queue.priority]);
    rpc_reset_waitqueue_priority(queue);
    return core::ptr::null_mut();
    new_queue:
    rpc_set_waitqueue_priority(queue, (unsigned int)(q - &queue.tasks[0]));
    out:
    return task;
    }
    static struct rpc_task *__rpc_find_next_queued(struct rpc_wait_queue *queue)
    {
    if (RPC_IS_PRIORITY(queue))
    return __rpc_find_next_queued_priority(queue);
    if (!list_empty(&queue.tasks[0]))
    return list_first_entry(&queue.tasks[0], struct rpc_task, u.tk_wait.list);
    return core::ptr::null_mut();
    }
//
// Wake up the first task on the wait queue.
//
    struct rpc_task *rpc_wake_up_first_on_wq(struct workqueue_struct *wq,
    struct rpc_wait_queue *queue,
    bool (*func)(struct rpc_task *, void *), void *data)
    {
    struct rpc_task	*task = core::ptr::null_mut();
    spin_lock(&queue.lock);
    task = __rpc_find_next_queued(queue);
    if (task != core::ptr::null_mut())
    task = rpc_wake_up_task_on_wq_queue_action_locked(wq, queue,
    task, func, data);
    spin_unlock(&queue.lock);
    return task;
    }
//
// Wake up the first task on the wait queue.
//
    struct rpc_task *rpc_wake_up_first(struct rpc_wait_queue *queue,
    bool (*func)(struct rpc_task *, void *), void *data)
    {
    return rpc_wake_up_first_on_wq(rpciod_workqueue, queue, func, data);
    }
    EXPORT_SYMBOL_GPL(rpc_wake_up_first);
#[no_mangle]
unsafe extern "C" fn rpc_wake_up_next_func(task: *mut rpc_task, data: *mut c_void) -> bool {
    static bool rpc_wake_up_next_func(struct rpc_task *task, void *data)
    {
    return true;
    }
//
// Wake up the next task on the wait queue.
//
    struct rpc_task *rpc_wake_up_next(struct rpc_wait_queue *queue)
    {
    return rpc_wake_up_first(queue, rpc_wake_up_next_func, core::ptr::null_mut());
    }
    EXPORT_SYMBOL_GPL(rpc_wake_up_next);
//
// rpc_wake_up_locked - wake up all rpc_tasks
// @queue: rpc_wait_queue on which the tasks are sleeping
//
#[no_mangle]
unsafe extern "C" fn rpc_wake_up_locked(queue: *mut rpc_wait_queue) {
    static void rpc_wake_up_locked(struct rpc_wait_queue *queue)
    {
    struct rpc_task *task;
    for (;;) {
    task = __rpc_find_next_queued(queue);
    if (task == core::ptr::null_mut())
    break;
    rpc_wake_up_task_queue_locked(queue, task);
    }
    }
//
// rpc_wake_up - wake up all rpc_tasks
// @queue: rpc_wait_queue on which the tasks are sleeping
//
// Grabs queue->lock
//
#[no_mangle]
pub unsafe extern "C" fn rpc_wake_up(queue: *mut rpc_wait_queue) {
    void rpc_wake_up(struct rpc_wait_queue *queue)
    {
    spin_lock(&queue.lock);
    rpc_wake_up_locked(queue);
    spin_unlock(&queue.lock);
    }
    EXPORT_SYMBOL_GPL(rpc_wake_up);
//
// rpc_wake_up_status_locked - wake up all rpc_tasks and set their status value.
// @queue: rpc_wait_queue on which the tasks are sleeping
// @status: status value to set
//
#[no_mangle]
unsafe extern "C" fn rpc_wake_up_status_locked(queue: *mut rpc_wait_queue, status: c_int) {
    static void rpc_wake_up_status_locked(struct rpc_wait_queue *queue, int status)
    {
    struct rpc_task *task;
    for (;;) {
    task = __rpc_find_next_queued(queue);
    if (task == core::ptr::null_mut())
    break;
    rpc_wake_up_task_queue_set_status_locked(queue, task, status);
    }
    }
//
// rpc_wake_up_status - wake up all rpc_tasks and set their status value.
// @queue: rpc_wait_queue on which the tasks are sleeping
// @status: status value to set
//
// Grabs queue->lock
//
#[no_mangle]
pub unsafe extern "C" fn rpc_wake_up_status(queue: *mut rpc_wait_queue, status: c_int) {
    void rpc_wake_up_status(struct rpc_wait_queue *queue, int status)
    {
    spin_lock(&queue.lock);
    rpc_wake_up_status_locked(queue, status);
    spin_unlock(&queue.lock);
    }
    EXPORT_SYMBOL_GPL(rpc_wake_up_status);
#[no_mangle]
unsafe extern "C" fn __rpc_queue_timer_fn(work: *mut work_struct) {
    static void __rpc_queue_timer_fn(struct work_struct *work)
    {
    struct rpc_wait_queue *queue = container_of(work,
    struct rpc_wait_queue,
    timer_list.dwork.work);
    struct rpc_task *task, *n;
    unsigned long expires, now, timeo;
    spin_lock(&queue.lock);
    expires = now = jiffies;
    list_for_each_entry_safe(task, n, &queue.timer_list.list, u.tk_wait.timer_list) {
    timeo = task.tk_timeout;
    if (time_after_eq(now, timeo)) {
    trace_rpc_task_timeout(task, task.tk_action);
    task.tk_status = -ETIMEDOUT;
    rpc_wake_up_task_queue_locked(queue, task);
    continue;
    }
    if (expires == now || time_after(expires, timeo))
    expires = timeo;
    }
    if (!list_empty(&queue.timer_list.list))
    rpc_set_queue_timer(queue, expires);
    spin_unlock(&queue.lock);
    }
#[no_mangle]
unsafe extern "C" fn __rpc_atrun(task: *mut rpc_task) {
    static void __rpc_atrun(struct rpc_task *task)
    {
    if (task.tk_status == -ETIMEDOUT)
    task.tk_status = 0;
    }
//
// Run a task at a later time
//
#[no_mangle]
pub unsafe extern "C" fn rpc_delay(task: *mut rpc_task, delay: c_ulong) {
    void rpc_delay(struct rpc_task *task, unsigned long delay)
    {
    rpc_sleep_on_timeout(&delay_queue, task, __rpc_atrun, jiffies + delay);
    }
    EXPORT_SYMBOL_GPL(rpc_delay);
//
// Helper to call task->tk_ops->rpc_call_prepare
//
#[no_mangle]
pub unsafe extern "C" fn rpc_prepare_task(task: *mut rpc_task) {
    void rpc_prepare_task(struct rpc_task *task)
    {
    task.tk_ops.rpc_call_prepare(task, task.tk_calldata);
    }
    static void
    rpc_init_task_statistics(struct rpc_task *task)
    {
// Initialize retry counters
    task.tk_garb_retry = 2;
    task.tk_cred_retry = 2;
// starting timestamp
    task.tk_start = ktime_get();
    }
    static void
    rpc_reset_task_statistics(struct rpc_task *task)
    {
    task.tk_timeouts = 0;
    task.tk_flags &= ~(RPC_CALL_MAJORSEEN|RPC_TASK_SENT);
    rpc_init_task_statistics(task);
    }
//
// Helper that calls task->tk_ops->rpc_call_done if it exists
//
#[no_mangle]
pub unsafe extern "C" fn rpc_exit_task(task: *mut rpc_task) {
    void rpc_exit_task(struct rpc_task *task)
    {
    trace_rpc_task_end(task, task.tk_action);
    task.tk_action = core::ptr::null_mut();
    if (task.tk_ops.rpc_count_stats)
    task.tk_ops.rpc_count_stats(task, task.tk_calldata);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: task->tk_client) -> else {
    else if (task.tk_client)
    rpc_count_iostats(task, task.tk_client.cl_metrics);
    if (task.tk_ops.rpc_call_done != core::ptr::null_mut()) {
    trace_rpc_task_call_done(task, task.tk_ops.rpc_call_done);
    task.tk_ops.rpc_call_done(task, task.tk_calldata);
    if (task.tk_action != core::ptr::null_mut()) {
// Always release the RPC slot and buffer memory
    xprt_release(task);
    rpc_reset_task_statistics(task);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn rpc_signal_task(task: *mut rpc_task) {
    void rpc_signal_task(struct rpc_task *task)
    {
    struct rpc_wait_queue *queue;
    if (!RPC_IS_ACTIVATED(task))
    return;
    if (!rpc_task_set_rpc_status(task, -ERESTARTSYS))
    return;
    trace_rpc_task_signalled(task, task.tk_action);
    queue = READ_ONCE(task.tk_waitqueue);
    if (queue)
    rpc_wake_up_queued_task(queue, task);
    }
#[no_mangle]
pub unsafe extern "C" fn rpc_task_try_cancel(task: *mut rpc_task, error: c_int) {
    void rpc_task_try_cancel(struct rpc_task *task, int error)
    {
    struct rpc_wait_queue *queue;
    if (!rpc_task_set_rpc_status(task, error))
    return;
    queue = READ_ONCE(task.tk_waitqueue);
    if (queue)
    rpc_wake_up_queued_task(queue, task);
    }
#[no_mangle]
pub unsafe extern "C" fn rpc_exit(task: *mut rpc_task, status: c_int) {
    void rpc_exit(struct rpc_task *task, int status)
    {
    task.tk_status = status;
    task.tk_action = rpc_exit_task;
    rpc_wake_up_queued_task(task.tk_waitqueue, task);
    }
    EXPORT_SYMBOL_GPL(rpc_exit);
#[no_mangle]
pub unsafe extern "C" fn rpc_release_calldata(ops: *const rpc_call_ops, calldata: *mut c_void) {
    void rpc_release_calldata(const struct rpc_call_ops *ops, void *calldata)
    {
    if (ops.rpc_release != core::ptr::null_mut())
    ops.rpc_release(calldata);
    }
#[no_mangle]
unsafe extern "C" fn xprt_needs_memalloc(xprt: *mut rpc_xprt, tk: *mut rpc_task) -> bool {
    static bool xprt_needs_memalloc(struct rpc_xprt *xprt, struct rpc_task *tk)
    {
    if (!xprt)
    return false;
    if (!atomic_read(&xprt.swapper))
    return false;
    return test_bit(XPRT_LOCKED, &xprt.state) && xprt.snd_task == tk;
    }
//
// This is the RPC `scheduler' (or rather, the finite state machine).
//
#[no_mangle]
unsafe extern "C" fn __rpc_execute(task: *mut rpc_task) {
    static void __rpc_execute(struct rpc_task *task)
    {
    struct rpc_wait_queue *queue;
    let mut task_is_async: c_int = RPC_IS_ASYNC(task);
    let mut status: c_int = 0;
    let mut pflags: c_ulong = current.flags;
    WARN_ON_ONCE(RPC_IS_QUEUED(task));
    if (RPC_IS_QUEUED(task))
    return;
    for (;;) {
    void (*do_action)(struct rpc_task *);
//
// Perform the next FSM step or a pending callback.
//
// tk_action may be NULL if the task has been killed.
//
    do_action = task.tk_action;
// Tasks with an RPC error status should exit
    if (do_action && do_action != rpc_exit_task &&
    (status = READ_ONCE(task.tk_rpc_status)) != 0) {
    task.tk_status = status;
    do_action = rpc_exit_task;
    }
// Callbacks override all actions
    if (task.tk_callback) {
    do_action = task.tk_callback;
    task.tk_callback = core::ptr::null_mut();
    }
    if (!do_action)
    break;
    if (RPC_IS_SWAPPER(task) ||
    xprt_needs_memalloc(task.tk_xprt, task))
    current.flags |= PF_MEMALLOC;
    trace_rpc_task_run_action(task, do_action);
    do_action(task);
//
// Lockless check for whether task is sleeping or not.
//
    if (!RPC_IS_QUEUED(task)) {
    cond_resched();
    continue;
    }
//
// The queue->lock protects against races with
// rpc_make_runnable().
//
// Note that once we clear RPC_TASK_RUNNING on an asynchronous
// rpc_task, rpc_make_runnable() can assign it to a
// different workqueue. We therefore cannot assume that the
// rpc_task pointer may still be dereferenced.
//
    queue = task.tk_waitqueue;
    spin_lock(&queue.lock);
    if (!RPC_IS_QUEUED(task)) {
    spin_unlock(&queue.lock);
    continue;
    }
// Wake up any task that has an exit status
    if (READ_ONCE(task.tk_rpc_status) != 0) {
    rpc_wake_up_task_queue_locked(queue, task);
    spin_unlock(&queue.lock);
    continue;
    }
    rpc_clear_running(task);
    spin_unlock(&queue.lock);
    if (task_is_async)
    goto out;
// sync task: sleep here
    trace_rpc_task_sync_sleep(task, task.tk_action);
    status = out_of_line_wait_on_bit(&task.tk_runstate,
    RPC_TASK_QUEUED, rpc_wait_bit_killable,
    TASK_KILLABLE|TASK_FREEZABLE);
    if (status < 0) {
//
// When a sync task receives a signal, it exits with
// -ERESTARTSYS. In order to catch any callbacks that
// clean up after sleeping on some queue, we don't
// break the loop here, but go around once more.
//
    rpc_signal_task(task);
    }
    trace_rpc_task_sync_wake(task, task.tk_action);
    }
// Release all resources associated with the task
    rpc_release_task(task);
    out:
    current_restore_flags(pflags, PF_MEMALLOC);
    }
//
// User-visible entry point to the scheduler.
//
// This may be called recursively if e.g. an async NFS task updates
// the attributes and finds that dirty pages must be flushed.
// NOTE: Upon exit of this function the task is guaranteed to be
// released. In particular note that tk_release() will have
// been called, so your task memory may have been freed.
//
#[no_mangle]
pub unsafe extern "C" fn rpc_execute(task: *mut rpc_task) {
    void rpc_execute(struct rpc_task *task)
    {
    let mut is_async: bool = RPC_IS_ASYNC(task);
    rpc_set_active(task);
    rpc_make_runnable(rpciod_workqueue, task);
    if (!is_async) {
    let mut pflags: c_uint = memalloc_nofs_save();
    __rpc_execute(task);
    memalloc_nofs_restore(pflags);
    }
    }
#[no_mangle]
unsafe extern "C" fn rpc_async_schedule(work: *mut work_struct) {
    static void rpc_async_schedule(struct work_struct *work)
    {
    let mut pflags: c_uint = memalloc_nofs_save();
    __rpc_execute(container_of(work, struct rpc_task, u.tk_work));
    memalloc_nofs_restore(pflags);
    }
//
// rpc_malloc - allocate RPC buffer resources
// @task: RPC task
//
// A single memory region is allocated, which is split between the
// RPC call and RPC reply that this task is being used for. When
// this RPC is retired, the memory is released by calling rpc_free.
//
// To prevent rpciod from hanging, this allocator never sleeps,
// returning -ENOMEM and suppressing warning if the request cannot
// be serviced immediately. The caller can arrange to sleep in a
// way that is safe for rpciod.
//
// Most requests are 'small' (under 2KiB) and can be serviced from a
// mempool, ensuring that NFS reads and writes can always proceed,
// and that there is good locality of reference for these buffers.
//
#[no_mangle]
pub unsafe extern "C" fn rpc_malloc(task: *mut rpc_task) -> c_int {
    int rpc_malloc(struct rpc_task *task)
    {
    struct rpc_rqst *rqst = task.tk_rqstp;
    let mut size: usize = rqst.rq_callsize + rqst.rq_rcvsize;
    struct rpc_buffer *buf;
    let mut gfp: gfp_t = rpc_task_gfp_mask();
    size += sizeof(struct rpc_buffer);
    if (size <= RPC_BUFFER_MAXSIZE) {
    buf = kmem_cache_alloc(rpc_buffer_slabp, gfp);
// Reach for the mempool if dynamic allocation fails
    if (!buf && RPC_IS_ASYNC(task))
    buf = mempool_alloc(rpc_buffer_mempool, GFP_NOWAIT);
    } else
    buf = kmalloc(size, gfp);
    if (!buf)
    return -ENOMEM;
    buf.len = size;
    rqst.rq_buffer = buf.data;
    rqst.rq_rbuffer = (char *)rqst.rq_buffer + rqst.rq_callsize;
    return 0;
    }
//
// rpc_free - free RPC buffer resources allocated via rpc_malloc
// @task: RPC task
//
#[no_mangle]
pub unsafe extern "C" fn rpc_free(task: *mut rpc_task) {
    void rpc_free(struct rpc_task *task)
    {
    void *buffer = task.tk_rqstp.rq_buffer;
    size_t size;
    struct rpc_buffer *buf;
    buf = container_of(buffer, struct rpc_buffer, data);
    size = buf.len;
    if (size <= RPC_BUFFER_MAXSIZE)
    mempool_free(buf, rpc_buffer_mempool);
    else
    kfree(buf);
    }
//
// Creation and deletion of RPC task structures
//
#[no_mangle]
unsafe extern "C" fn rpc_init_task(task: *mut rpc_task, task_setup_data: *const rpc_task_setup) {
    static void rpc_init_task(struct rpc_task *task, const struct rpc_task_setup *task_setup_data)
    {
    memset(task, 0, sizeof(*task));
    atomic_set(&task.tk_count, 1);
    task.tk_flags  = task_setup_data.flags;
    task.tk_ops = task_setup_data.callback_ops;
    task.tk_calldata = task_setup_data.callback_data;
    INIT_LIST_HEAD(&task.tk_task);
    task.tk_priority = task_setup_data.priority - RPC_PRIORITY_LOW;
    task.tk_owner = current.tgid;
// Initialize workqueue for async tasks
    task.tk_workqueue = task_setup_data.workqueue;
    task.tk_xprt = rpc_task_get_xprt(task_setup_data.rpc_client,
    xprt_get(task_setup_data.rpc_xprt));
    task.tk_op_cred = get_rpccred(task_setup_data.rpc_op_cred);
    if (task.tk_ops.rpc_call_prepare != core::ptr::null_mut())
    task.tk_action = rpc_prepare_task;
    rpc_init_task_statistics(task);
    }
    static struct rpc_task *rpc_alloc_task(void)
    {
    struct rpc_task *task;
    task = kmem_cache_alloc(rpc_task_slabp, rpc_task_gfp_mask());
    if (task)
    return task;
    return mempool_alloc(rpc_task_mempool, GFP_NOWAIT);
    }
//
// Create a new task for the specified client.
//
    struct rpc_task *rpc_new_task(const struct rpc_task_setup *setup_data)
    {
    struct rpc_task	*task = setup_data.task;
    let mut flags: c_ushort = 0;
    if (task == core::ptr::null_mut()) {
    task = rpc_alloc_task();
    if (task == core::ptr::null_mut()) {
    rpc_release_calldata(setup_data.callback_ops,
    setup_data.callback_data);
    return ERR_PTR(-ENOMEM);
    }
    flags = RPC_TASK_DYNAMIC;
    }
    rpc_init_task(task, setup_data);
    task.tk_flags |= flags;
    return task;
    }
//
// rpc_free_task - release rpc task and perform cleanups
//
// Note that we free up the rpc_task _after_ rpc_release_calldata()
// in order to work around a workqueue dependency issue.
//
// Tejun Heo states:
// "Workqueue currently considers two work items to be the same if they're
// on the same address and won't execute them concurrently - ie. it
// makes a work item which is queued again while being executed wait
// for the previous execution to complete.
//
// If a work function frees the work item, and then waits for an event
// which should be performed by another work item and *that* work item
// recycles the freed work item, it can create a false dependency loop.
// There really is no reliable way to detect this short of verifying
// every memory free."
//
#[no_mangle]
unsafe extern "C" fn rpc_free_task(task: *mut rpc_task) {
    static void rpc_free_task(struct rpc_task *task)
    {
    let mut tk_flags: c_ushort = task.tk_flags;
    put_rpccred(task.tk_op_cred);
    rpc_release_calldata(task.tk_ops, task.tk_calldata);
    if (tk_flags & RPC_TASK_DYNAMIC)
    mempool_free(task, rpc_task_mempool);
    }
#[no_mangle]
unsafe extern "C" fn rpc_async_release(work: *mut work_struct) {
    static void rpc_async_release(struct work_struct *work)
    {
    let mut pflags: c_uint = memalloc_nofs_save();
    rpc_free_task(container_of(work, struct rpc_task, u.tk_work));
    memalloc_nofs_restore(pflags);
    }
#[no_mangle]
unsafe extern "C" fn rpc_release_resources_task(task: *mut rpc_task) {
    static void rpc_release_resources_task(struct rpc_task *task)
    {
    xprt_release(task);
    if (task.tk_msg.rpc_cred) {
    if (!(task.tk_flags & RPC_TASK_CRED_NOREF))
    put_cred(task.tk_msg.rpc_cred);
    task.tk_msg.rpc_cred = core::ptr::null_mut();
    }
    rpc_task_release_client(task);
    }
    static void rpc_final_put_task(struct rpc_task *task,
    struct workqueue_struct *q)
    {
    if (q != core::ptr::null_mut()) {
    INIT_WORK(&task.u.tk_work, rpc_async_release);
    queue_work(q, &task.u.tk_work);
    } else
    rpc_free_task(task);
    }
#[no_mangle]
unsafe extern "C" fn rpc_do_put_task(task: *mut rpc_task, q: *mut workqueue_struct) {
    static void rpc_do_put_task(struct rpc_task *task, struct workqueue_struct *q)
    {
    if (atomic_dec_and_test(&task.tk_count)) {
    rpc_release_resources_task(task);
    rpc_final_put_task(task, q);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn rpc_put_task(task: *mut rpc_task) {
    void rpc_put_task(struct rpc_task *task)
    {
    rpc_do_put_task(task, core::ptr::null_mut());
    }
    EXPORT_SYMBOL_GPL(rpc_put_task);
#[no_mangle]
pub unsafe extern "C" fn rpc_put_task_async(task: *mut rpc_task) {
    void rpc_put_task_async(struct rpc_task *task)
    {
    rpc_do_put_task(task, task.tk_workqueue);
    }
    EXPORT_SYMBOL_GPL(rpc_put_task_async);
#[no_mangle]
unsafe extern "C" fn rpc_release_task(task: *mut rpc_task) {
    static void rpc_release_task(struct rpc_task *task)
    {
    WARN_ON_ONCE(RPC_IS_QUEUED(task));
    rpc_release_resources_task(task);
//
// Note: at this point we have been removed from rpc_clnt->cl_tasks,
// so it should be safe to use task->tk_count as a test for whether
// or not any other processes still hold references to our rpc_task.
//
    if (atomic_read(&task.tk_count) != 1 + !RPC_IS_ASYNC(task)) {
// Wake up anyone who may be waiting for task completion
    if (!rpc_complete_task(task))
    return;
    } else {
    if (!atomic_dec_and_test(&task.tk_count))
    return;
    }
    rpc_final_put_task(task, task.tk_workqueue);
    }
#[no_mangle]
pub unsafe extern "C" fn rpciod_up() -> c_int {
    int rpciod_up(void)
    {
    return try_module_get(THIS_MODULE) ? 0 : -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn rpciod_down() {
    void rpciod_down(void)
    {
    module_put(THIS_MODULE);
    }
//
// Start up the rpciod workqueue.
//
#[no_mangle]
unsafe extern "C" fn rpciod_start() -> c_int {
    static int rpciod_start(void)
    {
    struct workqueue_struct *wq;
//
// Create the rpciod thread and wait for it to start.
//
    wq = alloc_workqueue("rpciod", WQ_MEM_RECLAIM | WQ_UNBOUND, 0);
    if (!wq)
    goto out_failed;
    rpciod_workqueue = wq;
    wq = alloc_workqueue("xprtiod", WQ_UNBOUND | WQ_MEM_RECLAIM, 0);
    if (!wq)
    goto free_rpciod;
    xprtiod_workqueue = wq;
    return 1;
    free_rpciod:
    wq = rpciod_workqueue;
    rpciod_workqueue = core::ptr::null_mut();
    destroy_workqueue(wq);
    out_failed:
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rpciod_stop() {
    static void rpciod_stop(void)
    {
    struct workqueue_struct *wq = core::ptr::null_mut();
    if (rpciod_workqueue == core::ptr::null_mut())
    return;
    wq = rpciod_workqueue;
    rpciod_workqueue = core::ptr::null_mut();
    destroy_workqueue(wq);
    wq = xprtiod_workqueue;
    xprtiod_workqueue = core::ptr::null_mut();
    destroy_workqueue(wq);
    }
    void
    rpc_destroy_mempool(void)
    {
    rpciod_stop();
    mempool_destroy(rpc_buffer_mempool);
    mempool_destroy(rpc_task_mempool);
    kmem_cache_destroy(rpc_task_slabp);
    kmem_cache_destroy(rpc_buffer_slabp);
    rpc_destroy_wait_queue(&delay_queue);
    }
    int
    rpc_init_mempool(void)
    {
//
// The following is not strictly a mempool initialisation,
// but there is no harm in doing it here
//
    rpc_init_wait_queue(&delay_queue, "delayq");
    if (!rpciod_start())
    goto err_nomem;
    rpc_task_slabp = kmem_cache_create("rpc_tasks",
    sizeof(struct rpc_task),
    0, SLAB_HWCACHE_ALIGN,
    core::ptr::null_mut());
    if (!rpc_task_slabp)
    goto err_nomem;
    rpc_buffer_slabp = kmem_cache_create("rpc_buffers",
    RPC_BUFFER_MAXSIZE,
    0, SLAB_HWCACHE_ALIGN,
    core::ptr::null_mut());
    if (!rpc_buffer_slabp)
    goto err_nomem;
    rpc_task_mempool = mempool_create_slab_pool(RPC_TASK_POOLSIZE,
    rpc_task_slabp);
    if (!rpc_task_mempool)
    goto err_nomem;
    rpc_buffer_mempool = mempool_create_slab_pool(RPC_BUFFER_POOLSIZE,
    rpc_buffer_slabp);
    if (!rpc_buffer_mempool)
    goto err_nomem;
    return 0;
    err_nomem:
    rpc_destroy_mempool();
    return -ENOMEM;
    }
