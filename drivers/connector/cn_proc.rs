//! Automatically rewritten from C to Rust
//! Source: drivers/connector/cn_proc.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// cn_proc.c - process events connector
//
// Copyright (C) Matt Helsley, IBM Corp. 2005
// Based on cn_fork.c by Guillaume Thouvenin <guillaume.thouvenin@bull.net>
// Original copyright notice follows:
// Copyright (C) 2005 BULL SA.
//

//
// Size of a cn_msg followed by a proc_event structure.  Since the
// sizeof struct cn_msg is a multiple of 4 bytes, but not 8 bytes, we
// add one 4-byte word to the size here, and then start the actual
// cn_msg structure 4 bytes into the stack buffer.  The result is that
// the immediately following proc_event structure is aligned to 8 bytes.
//

// See comment above; we test our assumption about sizeof struct cn_msg here.
    static inline struct cn_msg *buffer_to_cn_msg(__u8 *buffer)
    {
    BUILD_BUG_ON(sizeof(struct cn_msg) != 20);
    return (struct cn_msg *)(buffer + 4);
    }
    let mut proc_event_num_listeners: static atomic_t = ATOMIC_INIT(0);
    let mut cn_proc_event_id: static struct cb_id = { CN_IDX_PROC, CN_VAL_PROC };
// local_event.count is used as the sequence number of the netlink message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct local_event {
    pub lock: local_lock_t,
    pub count: __u32,
}

    static DEFINE_PER_CPU(struct local_event, local_event) = {
    .lock = INIT_LOCAL_LOCK(lock),
    };
#[no_mangle]
unsafe extern "C" fn cn_filter(dsk: *mut sock, skb: *mut sk_buff, data: *mut c_void) -> c_int {
    static int cn_filter(struct sock *dsk, struct sk_buff *skb, void *data)
    {
    __u32 what, exit_code, *ptr;
    enum proc_cn_mcast_op mc_op;
    uintptr_t val;
    if (!dsk || !dsk.sk_user_data || !data)
    return 0;
    ptr = (__u32 *)data;
    what = *ptr++;
    exit_code = *ptr;
    val = ((struct proc_input *)(dsk.sk_user_data)).event_type;
    mc_op = ((struct proc_input *)(dsk.sk_user_data)).mcast_op;
    if (mc_op == PROC_CN_MCAST_IGNORE)
    return 1;
    if ((__u32)val == PROC_EVENT_ALL)
    return 0;
//
// Drop packet if we have to report only non-zero exit status
// (PROC_EVENT_NONZERO_EXIT) and exit status is 0
//
    if (((__u32)val & PROC_EVENT_NONZERO_EXIT) &&
    (what == PROC_EVENT_EXIT)) {
    if (exit_code)
    return 0;
    }
    if ((__u32)val & what)
    return 0;
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn send_msg(msg: *mut cn_msg) {
    static inline void send_msg(struct cn_msg *msg)
    {
    __u32 filter_data[2];
    local_lock(&local_event.lock);
    msg.seq = __this_cpu_inc_return(local_event.count) - 1;
    ((struct proc_event *)msg.data).cpu = smp_processor_id();
//
// local_lock() disables preemption during send to ensure the messages
// are ordered according to their sequence numbers.
//
// If cn_netlink_send() fails, the data is not sent.
//
    filter_data[0] = ((struct proc_event *)msg.data).what;
    if (filter_data[0] == PROC_EVENT_EXIT) {
    filter_data[1] =
    ((struct proc_event *)msg.data).event_data.exit.exit_code;
    } else {
    filter_data[1] = 0;
    }
    cn_netlink_send_mult(msg, msg.len, 0, CN_IDX_PROC, GFP_NOWAIT,
    cn_filter, (void *)filter_data);
    local_unlock(&local_event.lock);
    }
#[no_mangle]
pub unsafe extern "C" fn proc_fork_connector(task: *mut task_struct) {
    void proc_fork_connector(struct task_struct *task)
    {
    struct cn_msg *msg;
    struct proc_event *ev;
    __u8 buffer[CN_PROC_MSG_SIZE] __aligned(8);
    struct task_struct *parent;
    if (atomic_read(&proc_event_num_listeners) < 1)
    return;
    msg = buffer_to_cn_msg(buffer);
    ev = (struct proc_event *)msg.data;
    memset(&ev.event_data, 0, sizeof(ev.event_data));
    ev.timestamp_ns = ktime_get_ns();
    ev.what = PROC_EVENT_FORK;
    rcu_read_lock();
    parent = rcu_dereference(task.real_parent);
    ev.event_data.fork.parent_pid = parent.pid;
    ev.event_data.fork.parent_tgid = parent.tgid;
    rcu_read_unlock();
    ev.event_data.fork.child_pid = task.pid;
    ev.event_data.fork.child_tgid = task.tgid;
    memcpy(&msg.id, &cn_proc_event_id, sizeof(msg.id));
    msg.ack = 0; /* not used */
    msg.len = sizeof(*ev);
    msg.flags = 0; /* not used */
    send_msg(msg);
    }
#[no_mangle]
pub unsafe extern "C" fn proc_exec_connector(task: *mut task_struct) {
    void proc_exec_connector(struct task_struct *task)
    {
    struct cn_msg *msg;
    struct proc_event *ev;
    __u8 buffer[CN_PROC_MSG_SIZE] __aligned(8);
    if (atomic_read(&proc_event_num_listeners) < 1)
    return;
    msg = buffer_to_cn_msg(buffer);
    ev = (struct proc_event *)msg.data;
    memset(&ev.event_data, 0, sizeof(ev.event_data));
    ev.timestamp_ns = ktime_get_ns();
    ev.what = PROC_EVENT_EXEC;
    ev.event_data.exec.process_pid = task.pid;
    ev.event_data.exec.process_tgid = task.tgid;
    memcpy(&msg.id, &cn_proc_event_id, sizeof(msg.id));
    msg.ack = 0; /* not used */
    msg.len = sizeof(*ev);
    msg.flags = 0; /* not used */
    send_msg(msg);
    }
#[no_mangle]
pub unsafe extern "C" fn proc_id_connector(task: *mut task_struct, which_id: c_int) {
    void proc_id_connector(struct task_struct *task, int which_id)
    {
    struct cn_msg *msg;
    struct proc_event *ev;
    __u8 buffer[CN_PROC_MSG_SIZE] __aligned(8);
    const struct cred *cred;
    if (atomic_read(&proc_event_num_listeners) < 1)
    return;
    msg = buffer_to_cn_msg(buffer);
    ev = (struct proc_event *)msg.data;
    memset(&ev.event_data, 0, sizeof(ev.event_data));
    ev.what = which_id;
    ev.event_data.id.process_pid = task.pid;
    ev.event_data.id.process_tgid = task.tgid;
    rcu_read_lock();
    cred = __task_cred(task);
    if (which_id == PROC_EVENT_UID) {
    ev.event_data.id.r.ruid = from_kuid_munged(&init_user_ns, cred.uid);
    ev.event_data.id.e.euid = from_kuid_munged(&init_user_ns, cred.euid);
    } else if (which_id == PROC_EVENT_GID) {
    ev.event_data.id.r.rgid = from_kgid_munged(&init_user_ns, cred.gid);
    ev.event_data.id.e.egid = from_kgid_munged(&init_user_ns, cred.egid);
    } else {
    rcu_read_unlock();
    return;
    }
    rcu_read_unlock();
    ev.timestamp_ns = ktime_get_ns();
    memcpy(&msg.id, &cn_proc_event_id, sizeof(msg.id));
    msg.ack = 0; /* not used */
    msg.len = sizeof(*ev);
    msg.flags = 0; /* not used */
    send_msg(msg);
    }
#[no_mangle]
pub unsafe extern "C" fn proc_sid_connector(task: *mut task_struct) {
    void proc_sid_connector(struct task_struct *task)
    {
    struct cn_msg *msg;
    struct proc_event *ev;
    __u8 buffer[CN_PROC_MSG_SIZE] __aligned(8);
    if (atomic_read(&proc_event_num_listeners) < 1)
    return;
    msg = buffer_to_cn_msg(buffer);
    ev = (struct proc_event *)msg.data;
    memset(&ev.event_data, 0, sizeof(ev.event_data));
    ev.timestamp_ns = ktime_get_ns();
    ev.what = PROC_EVENT_SID;
    ev.event_data.sid.process_pid = task.pid;
    ev.event_data.sid.process_tgid = task.tgid;
    memcpy(&msg.id, &cn_proc_event_id, sizeof(msg.id));
    msg.ack = 0; /* not used */
    msg.len = sizeof(*ev);
    msg.flags = 0; /* not used */
    send_msg(msg);
    }
#[no_mangle]
pub unsafe extern "C" fn proc_ptrace_connector(task: *mut task_struct, ptrace_id: c_int) {
    void proc_ptrace_connector(struct task_struct *task, int ptrace_id)
    {
    struct cn_msg *msg;
    struct proc_event *ev;
    __u8 buffer[CN_PROC_MSG_SIZE] __aligned(8);
    if (atomic_read(&proc_event_num_listeners) < 1)
    return;
    msg = buffer_to_cn_msg(buffer);
    ev = (struct proc_event *)msg.data;
    memset(&ev.event_data, 0, sizeof(ev.event_data));
    ev.timestamp_ns = ktime_get_ns();
    ev.what = PROC_EVENT_PTRACE;
    ev.event_data.ptrace.process_pid  = task.pid;
    ev.event_data.ptrace.process_tgid = task.tgid;
    if (ptrace_id == PTRACE_ATTACH) {
    ev.event_data.ptrace.tracer_pid  = current.pid;
    ev.event_data.ptrace.tracer_tgid = current.tgid;
    } else if (ptrace_id == PTRACE_DETACH) {
    ev.event_data.ptrace.tracer_pid  = 0;
    ev.event_data.ptrace.tracer_tgid = 0;
    } else
    return;
    memcpy(&msg.id, &cn_proc_event_id, sizeof(msg.id));
    msg.ack = 0; /* not used */
    msg.len = sizeof(*ev);
    msg.flags = 0; /* not used */
    send_msg(msg);
    }
#[no_mangle]
pub unsafe extern "C" fn proc_comm_connector(task: *mut task_struct) {
    void proc_comm_connector(struct task_struct *task)
    {
    struct cn_msg *msg;
    struct proc_event *ev;
    __u8 buffer[CN_PROC_MSG_SIZE] __aligned(8);
    if (atomic_read(&proc_event_num_listeners) < 1)
    return;
    msg = buffer_to_cn_msg(buffer);
    ev = (struct proc_event *)msg.data;
    memset(&ev.event_data, 0, sizeof(ev.event_data));
    ev.timestamp_ns = ktime_get_ns();
    ev.what = PROC_EVENT_COMM;
    ev.event_data.comm.process_pid  = task.pid;
    ev.event_data.comm.process_tgid = task.tgid;
    get_task_comm(ev.event_data.comm.comm, task);
    memcpy(&msg.id, &cn_proc_event_id, sizeof(msg.id));
    msg.ack = 0; /* not used */
    msg.len = sizeof(*ev);
    msg.flags = 0; /* not used */
    send_msg(msg);
    }
#[no_mangle]
pub unsafe extern "C" fn proc_coredump_connector(task: *mut task_struct) {
    void proc_coredump_connector(struct task_struct *task)
    {
    struct cn_msg *msg;
    struct proc_event *ev;
    struct task_struct *parent;
    __u8 buffer[CN_PROC_MSG_SIZE] __aligned(8);
    if (atomic_read(&proc_event_num_listeners) < 1)
    return;
    msg = buffer_to_cn_msg(buffer);
    ev = (struct proc_event *)msg.data;
    memset(&ev.event_data, 0, sizeof(ev.event_data));
    ev.timestamp_ns = ktime_get_ns();
    ev.what = PROC_EVENT_COREDUMP;
    ev.event_data.coredump.process_pid = task.pid;
    ev.event_data.coredump.process_tgid = task.tgid;
    rcu_read_lock();
    if (pid_alive(task)) {
    parent = rcu_dereference(task.real_parent);
    ev.event_data.coredump.parent_pid = parent.pid;
    ev.event_data.coredump.parent_tgid = parent.tgid;
    }
    rcu_read_unlock();
    memcpy(&msg.id, &cn_proc_event_id, sizeof(msg.id));
    msg.ack = 0; /* not used */
    msg.len = sizeof(*ev);
    msg.flags = 0; /* not used */
    send_msg(msg);
    }
#[no_mangle]
pub unsafe extern "C" fn proc_exit_connector(task: *mut task_struct) {
    void proc_exit_connector(struct task_struct *task)
    {
    struct cn_msg *msg;
    struct proc_event *ev;
    struct task_struct *parent;
    __u8 buffer[CN_PROC_MSG_SIZE] __aligned(8);
    if (atomic_read(&proc_event_num_listeners) < 1)
    return;
    msg = buffer_to_cn_msg(buffer);
    ev = (struct proc_event *)msg.data;
    memset(&ev.event_data, 0, sizeof(ev.event_data));
    ev.timestamp_ns = ktime_get_ns();
    ev.what = PROC_EVENT_EXIT;
    ev.event_data.exit.process_pid = task.pid;
    ev.event_data.exit.process_tgid = task.tgid;
    ev.event_data.exit.exit_code = task.exit_code;
    ev.event_data.exit.exit_signal = task.exit_signal;
    rcu_read_lock();
    if (pid_alive(task)) {
    parent = rcu_dereference(task.real_parent);
    ev.event_data.exit.parent_pid = parent.pid;
    ev.event_data.exit.parent_tgid = parent.tgid;
    }
    rcu_read_unlock();
    memcpy(&msg.id, &cn_proc_event_id, sizeof(msg.id));
    msg.ack = 0; /* not used */
    msg.len = sizeof(*ev);
    msg.flags = 0; /* not used */
    send_msg(msg);
    }
//
// Send an acknowledgement message to userspace
//
// Use 0 for success, EFOO otherwise.
// Note: this is the negative of conventional kernel error
// values because it's not being returned via syscall return
// mechanisms.
//
#[no_mangle]
unsafe extern "C" fn cn_proc_ack(err: c_int, rcvd_seq: c_int, rcvd_ack: c_int) {
    static void cn_proc_ack(int err, int rcvd_seq, int rcvd_ack)
    {
    struct cn_msg *msg;
    struct proc_event *ev;
    __u8 buffer[CN_PROC_MSG_SIZE] __aligned(8);
    if (atomic_read(&proc_event_num_listeners) < 1)
    return;
    msg = buffer_to_cn_msg(buffer);
    ev = (struct proc_event *)msg.data;
    memset(&ev.event_data, 0, sizeof(ev.event_data));
    msg.seq = rcvd_seq;
    ev.timestamp_ns = ktime_get_ns();
    ev.cpu = -1;
    ev.what = PROC_EVENT_NONE;
    ev.event_data.ack.err = err;
    memcpy(&msg.id, &cn_proc_event_id, sizeof(msg.id));
    msg.ack = rcvd_ack + 1;
    msg.len = sizeof(*ev);
    msg.flags = 0; /* not used */
    send_msg(msg);
    }
//
// cn_proc_mcast_ctl
// @msg: message sent from userspace via the connector
// @nsp: NETLINK_CB of the client's socket buffer
//
    static void cn_proc_mcast_ctl(struct cn_msg *msg,
    struct netlink_skb_parms *nsp)
    {
    let mut mc_op: enum proc_cn_mcast_op = 0, prev_mc_op = 0;
    struct proc_input *pinput = core::ptr::null_mut();
    let mut ev_type: enum proc_cn_event = 0;
    let mut err: c_int = 0, initial = 0;
    struct sock *sk = core::ptr::null_mut();
//
// Events are reported with respect to the initial pid
// and user namespaces so ignore requestors from
// other namespaces.
//
    if ((current_user_ns() != &init_user_ns) ||
    !task_is_in_init_pid_ns(current))
    return;
    if (msg.len == sizeof(*pinput)) {
    pinput = (struct proc_input *)msg.data;
    mc_op = pinput.mcast_op;
    ev_type = pinput.event_type;
    } else if (msg.len == sizeof(mc_op)) {
    mc_op = *((enum proc_cn_mcast_op *)msg.data);
    ev_type = PROC_EVENT_ALL;
    } else {
    return;
    }
    ev_type = valid_event((enum proc_cn_event)ev_type);
    if (ev_type == PROC_EVENT_NONE)
    ev_type = PROC_EVENT_ALL;
    if (nsp.sk) {
    sk = nsp.sk;
    if (sk.sk_user_data == core::ptr::null_mut()) {
    sk.sk_user_data = kzalloc_obj(struct proc_input);
    if (sk.sk_user_data == core::ptr::null_mut()) {
    err = ENOMEM;
    goto out;
    }
    initial = 1;
    } else {
    prev_mc_op =
    ((struct proc_input *)(sk.sk_user_data)).mcast_op;
    }
    ((struct proc_input *)(sk.sk_user_data)).event_type =
    ev_type;
    ((struct proc_input *)(sk.sk_user_data)).mcast_op = mc_op;
    }
    switch (mc_op) {
    case PROC_CN_MCAST_LISTEN:
    if (initial || (prev_mc_op != PROC_CN_MCAST_LISTEN))
    atomic_inc(&proc_event_num_listeners);
    break;
    case PROC_CN_MCAST_IGNORE:
    if (!initial && (prev_mc_op != PROC_CN_MCAST_IGNORE))
    atomic_dec(&proc_event_num_listeners);
    ((struct proc_input *)(sk.sk_user_data)).event_type =
    PROC_EVENT_NONE;
    break;
    default:
    err = EINVAL;
    break;
    }
    out:
    cn_proc_ack(err, msg.seq, msg.ack);
    }
//
// cn_proc_init - initialization entry point
//
// Adds the connector callback to the connector driver.
//
#[no_mangle]
unsafe extern "C" fn cn_proc_init() -> int __init {
    static int __init cn_proc_init(void)
    {
    int err = cn_add_callback(&cn_proc_event_id,
    "cn_proc",
    &cn_proc_mcast_ctl);
    if (err) {
    pr_warn("cn_proc failed to register\n");
    return err;
    }
    return 0;
    }
    device_initcall(cn_proc_init);
