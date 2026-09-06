//! Automatically rewritten from C to Rust
//! Source: kernel/taskstats.c
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



// SPDX-License-Identifier: GPL-2.0-or-later
//
// taskstats.c - Export per-task statistics to userland
//
// Copyright (C) Shailabh Nagar, IBM Corp. 2006
// (C) Balbir Singh,   IBM Corp. 2006
//

//
// Maximum length of a cpumask that can be specified in
// the TASKSTATS_CMD_ATTR_REGISTER/DEREGISTER_CPUMASK attribute
//
// static DEFINE_PER_CPU(__u32, taskstats_seqnum);
    static int family_registered;
    struct kmem_cache *taskstats_cache;
    static struct genl_family family;
    static const struct nla_policy taskstats_cmd_get_policy[] = {
    [TASKSTATS_CMD_ATTR_PID]  = { .type = NLA_U32 },
    [TASKSTATS_CMD_ATTR_TGID] = { .type = NLA_U32 },
    [TASKSTATS_CMD_ATTR_REGISTER_CPUMASK] = { .type = NLA_STRING },
    [TASKSTATS_CMD_ATTR_DEREGISTER_CPUMASK] = { .type = NLA_STRING },};
    static const struct nla_policy cgroupstats_cmd_get_policy[] = {
    [CGROUPSTATS_CMD_ATTR_FD] = { .type = NLA_U32 },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct listener {
    pub list: list_head,
    pub pid: pid_t,
    pub valid: c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct listener_list {
    pub sem: rw_semaphore,
    pub list: list_head,
}
// static DEFINE_PER_CPU(struct listener_list, listener_array);
    enum actions {
    REGISTER,
    DEREGISTER,
    CPU_DONT_CARE
    };
    static int prepare_reply(struct genl_info *info, u8 cmd, struct sk_buff **skbp,
    size_t size)
    {
    struct sk_buff *skb;
    void *reply;
//
// If new attributes are added, please revisit this allocation
//
    skb = genlmsg_new(size, GFP_KERNEL);
    if (!skb)
    return -ENOMEM;
    if (!info) {
    let mut seq: c_int = this_cpu_inc_return(taskstats_seqnum) - 1;
    reply = genlmsg_put(skb, 0, seq, &family, 0, cmd);
    } else
    reply = genlmsg_put_reply(skb, info, &family, 0, cmd);
    if (reply == core::ptr::null_mut()) {
    nlmsg_free(skb);
    return -EINVAL;
    }
// skbp = skb;
    return 0;
    }
//
// Send taskstats data in @skb to listener with nl_pid @pid
//
#[no_mangle]
unsafe extern "C" fn send_reply(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    struct genlmsghdr *genlhdr = nlmsg_data(nlmsg_hdr(skb));
    void *reply = genlmsg_data(genlhdr);
    genlmsg_end(skb, reply);
    return genlmsg_reply(skb, info);
    }
//
// Send taskstats data in @skb to listeners registered for @cpu's exit data
//
    static void send_cpu_listeners(struct sk_buff *skb,
    struct listener_list *listeners)
    {
    struct genlmsghdr *genlhdr = nlmsg_data(nlmsg_hdr(skb));
    struct listener *s, *tmp;
    struct sk_buff *skb_next, *skb_cur = skb;
    void *reply = genlmsg_data(genlhdr);
    let mut delcount: c_int = 0;
    genlmsg_end(skb, reply);
    down_read(&listeners.sem);
    list_for_each_entry(s, &listeners.list, list) {
    int rc;
    skb_next = core::ptr::null_mut();
    if (!list_is_last(&s.list, &listeners.list)) {
    skb_next = skb_clone(skb_cur, GFP_KERNEL);
    if (!skb_next)
    break;
    }
    rc = genlmsg_unicast(&init_net, skb_cur, s.pid);
    if (rc == -ECONNREFUSED) {
    s.valid = 0;
    delcount++;
    }
    skb_cur = skb_next;
    }
    up_read(&listeners.sem);
    if (skb_cur)
    nlmsg_free(skb_cur);
    if (!delcount)
    return;
// Delete invalidated entries
    down_write(&listeners.sem);
    list_for_each_entry_safe(s, tmp, &listeners.list, list) {
    if (!s.valid) {
    list_del(&s.list);
    kfree(s);
    }
    }
    up_write(&listeners.sem);
    }
#[no_mangle]
unsafe extern "C" fn exe_add_tsk(stats: *mut taskstats, tsk: *mut task_struct) {
// No idea if I'm allowed to access that here, now.
    struct file *exe_file = get_task_exe_file(tsk);
    if (exe_file) {
// Following cp_new_stat64() in stat.c .
    stats.ac_exe_dev =
    huge_encode_dev(exe_file.f_inode.i_sb.s_dev);
    stats.ac_exe_inode = exe_file.f_inode.i_ino;
    fput(exe_file);
    } else {
    stats.ac_exe_dev = 0;
    stats.ac_exe_inode = 0;
    }
    }
    static void fill_stats(struct user_namespace *user_ns,
    struct pid_namespace *pid_ns,
    struct task_struct *tsk, struct taskstats *stats)
    {
    memset(stats, 0, sizeof(*stats));
//
// Each accounting subsystem adds calls to its functions to
// fill in relevant parts of struct taskstsats as follows
//
// per-task-foo(stats, tsk);
//
    delayacct_add_tsk(stats, tsk);
// fill in basic acct fields
    stats.version = TASKSTATS_VERSION;
    stats.nvcsw = tsk.nvcsw;
    stats.nivcsw = tsk.nivcsw;
    bacct_add_tsk(user_ns, pid_ns, stats, tsk);
// fill in extended acct fields
    xacct_add_tsk(stats, tsk);
// add executable info
    exe_add_tsk(stats, tsk);
    }
#[no_mangle]
unsafe extern "C" fn fill_stats_for_pid(pid: pid_t, stats: *mut taskstats) -> c_int {
    struct task_struct *tsk;
    tsk = find_get_task_by_vpid(pid);
    if (!tsk)
    return -ESRCH;
    fill_stats(current_user_ns(), task_active_pid_ns(current), tsk, stats);
    put_task_struct(tsk);
    return 0;
    }
    static void tgid_stats_add_task(struct taskstats *stats,
    struct task_struct *tsk, u64 now_ns)
    {
    u64 delta, utime, stime;
//
// Each accounting subsystem calls its functions here to
// accumulate its per-task stats for tsk, into the per-tgid structure
//
// per-task-foo(stats, tsk);
//
    delayacct_add_tsk(stats, tsk);
// calculate task elapsed time in nsec
    delta = now_ns - tsk.start_time;
// Convert to micro seconds
    do_div(delta, NSEC_PER_USEC);
    stats.ac_etime += delta;
    task_cputime(tsk, &utime, &stime);
    stats.ac_utime += div_u64(utime, NSEC_PER_USEC);
    stats.ac_stime += div_u64(stime, NSEC_PER_USEC);
    stats.nvcsw += tsk.nvcsw;
    stats.nivcsw += tsk.nivcsw;
    }
#[no_mangle]
unsafe extern "C" fn fill_stats_for_tgid(tgid: pid_t, stats: *mut taskstats) -> c_int {
    struct task_struct *tsk, *first;
    unsigned long flags;
    let mut rc: c_int = -ESRCH;
    u64 now_ns;
//
// Add additional stats from live tasks except zombie thread group
// leaders who are already counted with the dead tasks
//
    rcu_read_lock();
    first = find_task_by_vpid(tgid);
    if (!first || !lock_task_sighand(first, &flags))
    goto out;
    if (first.signal.stats)
    memcpy(stats, first.signal.stats, sizeof(*stats));
    else
    memset(stats, 0, sizeof(*stats));
    now_ns = ktime_get_ns();
    for_each_thread(first, tsk) {
    if (tsk.exit_state)
    continue;
    tgid_stats_add_task(stats, tsk, now_ns);
    }
    unlock_task_sighand(first, &flags);
    rc = 0;
    out:
    rcu_read_unlock();
    stats.version = TASKSTATS_VERSION;
//
// Accounting subsystems can also add calls here to modify
// fields of taskstats.
//
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn fill_tgid_exit(tsk: *mut task_struct) {
    unsigned long flags;
    u64 now_ns;
    spin_lock_irqsave(&tsk.sighand.siglock, flags);
    if (!tsk.signal.stats)
    goto ret;
    now_ns = ktime_get_ns();
    tgid_stats_add_task(tsk.signal.stats, tsk, now_ns);
    ret:
    spin_unlock_irqrestore(&tsk.sighand.siglock, flags);
    return;
    }
#[no_mangle]
unsafe extern "C" fn add_del_listener(pid: pid_t, mask: *const cpumask, isadd: c_int) -> c_int {
    struct listener_list *listeners;
    struct listener *s, *tmp, *s2;
    unsigned int cpu;
    let mut ret: c_int = 0;
    if (!cpumask_subset(mask, cpu_possible_mask))
    return -EINVAL;
    if (current_user_ns() != &init_user_ns)
    return -EINVAL;
    if (task_active_pid_ns(current) != &init_pid_ns)
    return -EINVAL;
    if (isadd == REGISTER) {
    for_each_cpu(cpu, mask) {
    s = kmalloc_node(sizeof(struct listener),
    GFP_KERNEL, cpu_to_node(cpu));
    if (!s) {
    ret = -ENOMEM;
    goto cleanup;
    }
    s.pid = pid;
    s.valid = 1;
    listeners = &per_cpu(listener_array, cpu);
    down_write(&listeners.sem);
    list_for_each_entry(s2, &listeners.list, list) {
    if (s2.pid == pid && s2.valid)
    goto exists;
    }
    list_add(&s.list, &listeners.list);
    s = core::ptr::null_mut();
    exists:
    up_write(&listeners.sem);
    kfree(s); /* nop if core::ptr::null_mut() */
    }
    return 0;
    }
// Deregister or cleanup
    cleanup:
    for_each_cpu(cpu, mask) {
    listeners = &per_cpu(listener_array, cpu);
    down_write(&listeners.sem);
    list_for_each_entry_safe(s, tmp, &listeners.list, list) {
    if (s.pid == pid) {
    list_del(&s.list);
    kfree(s);
    break;
    }
    }
    up_write(&listeners.sem);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn parse(na: *mut nlattr, mask: *mut cpumask) -> c_int {
    char *data;
    int len;
    int ret;
    len = nla_len(na);
    if (len > TASKSTATS_CPUMASK_MAXLEN)
    return -E2BIG;
    if (len < 1)
    return -EINVAL;
    data = nla_strdup(na, GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    ret = cpulist_parse(data, mask);
    kfree(data);
    return ret;
    }
    static struct taskstats *mk_reply(struct sk_buff *skb, int type, u32 pid)
    {
    struct nlattr *na, *ret;
    int aggr;
    aggr = (type == TASKSTATS_TYPE_PID)
    ? TASKSTATS_TYPE_AGGR_PID
    : TASKSTATS_TYPE_AGGR_TGID;
    na = nla_nest_start_noflag(skb, aggr);
    if (!na)
    goto err;
    if (nla_put(skb, type, sizeof(pid), &pid) < 0) {
    nla_nest_cancel(skb, na);
    goto err;
    }
    ret = nla_reserve_64bit(skb, TASKSTATS_TYPE_STATS,
    sizeof(struct taskstats), TASKSTATS_TYPE_NULL);
    if (!ret) {
    nla_nest_cancel(skb, na);
    goto err;
    }
    nla_nest_end(skb, na);
    return nla_data(ret);
    err:
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn cgroupstats_user_cmd(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    let mut rc: c_int = 0;
    struct sk_buff *rep_skb;
    struct cgroupstats *stats;
    struct nlattr *na;
    size_t size;
    u32 fd;
    na = info.attrs[CGROUPSTATS_CMD_ATTR_FD];
    if (!na)
    return -EINVAL;
    fd = nla_get_u32(info.attrs[CGROUPSTATS_CMD_ATTR_FD]);
    CLASS(fd, f)(fd);
    if (fd_empty(f))
    return -EBADF;
    size = nla_total_size(sizeof(struct cgroupstats));
    rc = prepare_reply(info, CGROUPSTATS_CMD_NEW, &rep_skb,
    size);
    if (rc < 0)
    return rc;
    na = nla_reserve(rep_skb, CGROUPSTATS_TYPE_CGROUP_STATS,
    sizeof(struct cgroupstats));
    if (na == core::ptr::null_mut()) {
    nlmsg_free(rep_skb);
    return -EMSGSIZE;
    }
    stats = nla_data(na);
    memset(stats, 0, sizeof(*stats));
    rc = cgroupstats_build(stats, fd_file(f).f_path.dentry);
    if (rc < 0) {
    nlmsg_free(rep_skb);
    return rc;
    }
    return send_reply(rep_skb, info);
    }
    static int cmd_attr_cpumask(struct genl_info *info, int attr,
    enum actions action)
    {
    cpumask_var_t mask __free(free_cpumask_var) = CPUMASK_VAR_NULL;
    int rc;
    if (!alloc_cpumask_var(&mask, GFP_KERNEL))
    return -ENOMEM;
    rc = parse(info.attrs[attr], mask);
    if (rc < 0)
    return rc;
    return add_del_listener(info.snd_portid, mask, action);
    }
#[no_mangle]
unsafe extern "C" fn taskstats_packet_size() -> usize {
    size_t size;
    size = nla_total_size(sizeof(u32)) +
    nla_total_size_64bit(sizeof(struct taskstats)) +
    nla_total_size(0);
    return size;
    }
#[no_mangle]
unsafe extern "C" fn cmd_attr_pid(info: *mut genl_info) -> c_int {
    struct taskstats *stats;
    struct sk_buff *rep_skb;
    size_t size;
    u32 pid;
    int rc;
    size = taskstats_packet_size();
    rc = prepare_reply(info, TASKSTATS_CMD_NEW, &rep_skb, size);
    if (rc < 0)
    return rc;
    rc = -EINVAL;
    pid = nla_get_u32(info.attrs[TASKSTATS_CMD_ATTR_PID]);
    stats = mk_reply(rep_skb, TASKSTATS_TYPE_PID, pid);
    if (!stats)
    goto err;
    rc = fill_stats_for_pid(pid, stats);
    if (rc < 0)
    goto err;
    return send_reply(rep_skb, info);
    err:
    nlmsg_free(rep_skb);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn cmd_attr_tgid(info: *mut genl_info) -> c_int {
    struct taskstats *stats;
    struct sk_buff *rep_skb;
    size_t size;
    u32 tgid;
    int rc;
    size = taskstats_packet_size();
    rc = prepare_reply(info, TASKSTATS_CMD_NEW, &rep_skb, size);
    if (rc < 0)
    return rc;
    rc = -EINVAL;
    tgid = nla_get_u32(info.attrs[TASKSTATS_CMD_ATTR_TGID]);
    stats = mk_reply(rep_skb, TASKSTATS_TYPE_TGID, tgid);
    if (!stats)
    goto err;
    rc = fill_stats_for_tgid(tgid, stats);
    if (rc < 0)
    goto err;
    return send_reply(rep_skb, info);
    err:
    nlmsg_free(rep_skb);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn taskstats_user_cmd(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    if (info.attrs[TASKSTATS_CMD_ATTR_REGISTER_CPUMASK])
    return cmd_attr_cpumask(info,
    TASKSTATS_CMD_ATTR_REGISTER_CPUMASK,
    REGISTER);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: info->attrs[TASKSTATS_CMD_ATTR_DEREGISTER_CPUMASK]) -> else {
    else if (info.attrs[TASKSTATS_CMD_ATTR_DEREGISTER_CPUMASK])
    return cmd_attr_cpumask(info,
    TASKSTATS_CMD_ATTR_DEREGISTER_CPUMASK,
    DEREGISTER);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: info->attrs[TASKSTATS_CMD_ATTR_PID]) -> else {
    else if (info.attrs[TASKSTATS_CMD_ATTR_PID])
    return cmd_attr_pid(info);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: info->attrs[TASKSTATS_CMD_ATTR_TGID]) -> else {
    else if (info.attrs[TASKSTATS_CMD_ATTR_TGID])
    return cmd_attr_tgid(info);
    else
    return -EINVAL;
    }
    static struct taskstats *taskstats_tgid_alloc(struct task_struct *tsk)
    {
    struct signal_struct *sig = tsk.signal;
    struct taskstats *stats_new, *stats;
// Pairs with smp_store_release() below.
    stats = smp_load_acquire(&sig.stats);
    if (stats || thread_group_empty(tsk))
    return stats;
// No problem if kmem_cache_zalloc() fails
    stats_new = kmem_cache_zalloc(taskstats_cache, GFP_KERNEL);
    spin_lock_irq(&tsk.sighand.siglock);
    stats = sig.stats;
    if (!stats) {
//
// Pairs with smp_store_release() above and order the
// kmem_cache_zalloc().
//
    smp_store_release(&sig.stats, stats_new);
    stats = stats_new;
    stats_new = core::ptr::null_mut();
    }
    spin_unlock_irq(&tsk.sighand.siglock);
    if (stats_new)
    kmem_cache_free(taskstats_cache, stats_new);
    return stats;
    }
// Send pid data out on exit
#[no_mangle]
pub unsafe extern "C" fn taskstats_exit(tsk: *mut task_struct, group_dead: c_int) {
    int rc;
    struct listener_list *listeners;
    struct taskstats *stats;
    struct sk_buff *rep_skb;
    size_t size;
    int is_thread_group;
    if (!family_registered)
    return;
//
// Size includes space for nested attributes
//
    size = taskstats_packet_size();
    is_thread_group = !!taskstats_tgid_alloc(tsk);
    if (is_thread_group) {
// PID + STATS + TGID + STATS
    size = 2 * size;
// fill the tsk->signal->stats structure
    fill_tgid_exit(tsk);
    }
    listeners = raw_cpu_ptr(&listener_array);
    if (list_empty(&listeners.list))
    return;
    rc = prepare_reply(core::ptr::null_mut(), TASKSTATS_CMD_NEW, &rep_skb, size);
    if (rc < 0)
    return;
    stats = mk_reply(rep_skb, TASKSTATS_TYPE_PID,
    task_pid_nr_ns(tsk, &init_pid_ns));
    if (!stats)
    goto err;
    fill_stats(&init_user_ns, &init_pid_ns, tsk, stats);
    if (group_dead)
    stats.ac_flag |= AGROUP;
//
// Doesn't matter if tsk is the leader or the last group member leaving
//
    if (!is_thread_group || !group_dead)
    goto send;
    stats = mk_reply(rep_skb, TASKSTATS_TYPE_TGID,
    task_tgid_nr_ns(tsk, &init_pid_ns));
    if (!stats)
    goto err;
    memcpy(stats, tsk.signal.stats, sizeof(*stats));
    stats.version = TASKSTATS_VERSION;
    send:
    send_cpu_listeners(rep_skb, listeners);
    return;
    err:
    nlmsg_free(rep_skb);
    }
    static const struct genl_ops taskstats_ops[] = {
    {
    .cmd		= TASKSTATS_CMD_GET,
    .validate = GENL_DONT_VALIDATE_STRICT | GENL_DONT_VALIDATE_DUMP,
    .doit		= taskstats_user_cmd,
    .policy		= taskstats_cmd_get_policy,
    .maxattr	= ARRAY_SIZE(taskstats_cmd_get_policy) - 1,
    .flags		= GENL_ADMIN_PERM,
    },
    {
    .cmd		= CGROUPSTATS_CMD_GET,
    .validate = GENL_DONT_VALIDATE_STRICT | GENL_DONT_VALIDATE_DUMP,
    .doit		= cgroupstats_user_cmd,
    .policy		= cgroupstats_cmd_get_policy,
    .maxattr	= ARRAY_SIZE(cgroupstats_cmd_get_policy) - 1,
    },
    };
    static struct genl_family family __ro_after_init = {
    .name		= TASKSTATS_GENL_NAME,
    .version	= TASKSTATS_GENL_VERSION,
    .module		= THIS_MODULE,
    .ops		= taskstats_ops,
    .n_ops		= ARRAY_SIZE(taskstats_ops),
    .resv_start_op	= CGROUPSTATS_CMD_GET + 1,
    .netnsok	= true,
    };
// Needed early in initialization
#[no_mangle]
pub unsafe extern "C" fn taskstats_init_early() -> c_int {
    unsigned int i;
    taskstats_cache = KMEM_CACHE(taskstats, SLAB_PANIC);
    for_each_possible_cpu(i) {
    INIT_LIST_HEAD(&(per_cpu(listener_array, i).list));
    init_rwsem(&(per_cpu(listener_array, i).sem));
    }
    }
#[no_mangle]
unsafe extern "C" fn taskstats_init() -> c_int {
    int rc;
    rc = genl_register_family(&family);
    if (rc)
    return rc;
    family_registered = 1;
    pr_info("registered taskstats version %d\n", TASKSTATS_GENL_VERSION);
    return 0;
    }
//
// late initcall ensures initialization of statistics collection
// mechanisms precedes initialization of the taskstats interface
//
    late_initcall(taskstats_init);

}
}
}
