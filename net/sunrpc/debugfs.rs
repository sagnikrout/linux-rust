//! Automatically rewritten from C to Rust
//! Source: net/sunrpc/debugfs.c
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
// debugfs interface for sunrpc
//
// (c) 2014 Jeff Layton <jlayton@primarydata.com>
//

    static struct dentry *topdir;
    static struct dentry *rpc_clnt_dir;
    static struct dentry *rpc_xprt_dir;
    static int
    tasks_show(struct seq_file *f, void *v)
    {
    let mut xid: u32 = 0;
    struct rpc_task *task = v;
    struct rpc_clnt *clnt = task.tk_client;
    const char *rpc_waitq = "none";
    if (RPC_IS_QUEUED(task))
    rpc_waitq = rpc_qname(task.tk_waitqueue);
    if (task.tk_rqstp)
    xid = be32_to_cpu(task.tk_rqstp.rq_xid);
    seq_printf(f, "%5u %04x %6d 0x%x 0x%x %8ld %ps %sv%u %s a:%ps q:%s\n",
    task.tk_pid, task.tk_flags, task.tk_status,
    clnt.cl_clid, xid, rpc_task_timeout(task), task.tk_ops,
    clnt.cl_program.name, clnt.cl_vers, rpc_proc_name(task),
    task.tk_action, rpc_waitq);
    return 0;
    }
    static void *
    tasks_start(struct seq_file *f, loff_t *ppos)
    __acquires(&clnt.cl_lock)
    {
    struct rpc_clnt *clnt = f.private;
    let mut pos: loff_t = *ppos;
    struct rpc_task *task;
    spin_lock(&clnt.cl_lock);
    list_for_each_entry(task, &clnt.cl_tasks, tk_task)
    if (pos-- == 0)
    return task;
    return core::ptr::null_mut();
    }
    static void *
    tasks_next(struct seq_file *f, void *v, loff_t *pos)
    {
    struct rpc_clnt *clnt = f.private;
    struct rpc_task *task = v;
    struct list_head *next = task.tk_task.next;
    ++*pos;
// If there's another task on list, return it
    if (next == &clnt.cl_tasks)
    return core::ptr::null_mut();
    return list_entry(next, struct rpc_task, tk_task);
    }
    static void
    tasks_stop(struct seq_file *f, void *v)
    __releases(&clnt.cl_lock)
    {
    struct rpc_clnt *clnt = f.private;
    spin_unlock(&clnt.cl_lock);
    seq_printf(f, "clnt[%pISpc] RPC tasks[%d]\n",
    (struct sockaddr *)&clnt.cl_xprt.addr,
    atomic_read(&clnt.cl_task_count));
    }
    static const struct seq_operations tasks_seq_operations = {
    .start	= tasks_start,
    .next	= tasks_next,
    .stop	= tasks_stop,
    .show	= tasks_show,
    };
#[no_mangle]
unsafe extern "C" fn tasks_open(inode: *mut inode, filp: *mut file) -> c_int {
    static int tasks_open(struct inode *inode, struct file *filp)
    {
    let mut ret: c_int = seq_open(filp, &tasks_seq_operations);
    if (!ret) {
    struct seq_file *seq = filp.private_data;
    struct rpc_clnt *clnt = seq.private = inode.i_private;
    if (!refcount_inc_not_zero(&clnt.cl_count)) {
    seq_release(inode, filp);
    ret = -EINVAL;
    }
    }
    return ret;
    }
    static int
    tasks_release(struct inode *inode, struct file *filp)
    {
    struct seq_file *seq = filp.private_data;
    struct rpc_clnt *clnt = seq.private;
    rpc_release_client(clnt);
    return seq_release(inode, filp);
    }
    static const struct file_operations tasks_fops = {
    .owner		= THIS_MODULE,
    .open		= tasks_open,
    .read		= seq_read,
    .llseek		= seq_lseek,
    .release	= tasks_release,
    };
#[no_mangle]
unsafe extern "C" fn do_xprt_debugfs(clnt: *mut rpc_clnt, xprt: *mut rpc_xprt, numv: *mut c_void) -> c_int {
    static int do_xprt_debugfs(struct rpc_clnt *clnt, struct rpc_xprt *xprt, void *numv)
    {
    int len;
    char name[24]; /* enough for "../../rpc_xprt/ + 8 hex digits + core::ptr::null_mut() */
    char link[9]; /* enough for 8 hex digits + core::ptr::null_mut() */
    int *nump = numv;
    if (IS_ERR_OR_NULL(xprt.debugfs))
    return 0;
    len = snprintf(name, sizeof(name), "../../rpc_xprt/%s",
    xprt.debugfs.d_name.name);
    if (len >= sizeof(name))
    return -1;
    if (*nump == 0)
    strcpy(link, "xprt");
    else {
    len = snprintf(link, sizeof(link), "xprt%d", *nump);
    if (len >= sizeof(link))
    return -1;
    }
    debugfs_create_symlink(link, clnt.cl_debugfs, name);
    (*nump)++;
    return 0;
    }
    void
    rpc_clnt_debugfs_register(struct rpc_clnt *clnt)
    {
    int len;
    char name[9]; /* enough for 8 hex digits + core::ptr::null_mut() */
    let mut xprtnum: c_int = 0;
    len = snprintf(name, sizeof(name), "%x", clnt.cl_clid);
    if (len >= sizeof(name))
    return;
// make the per-client dir
    clnt.cl_debugfs = debugfs_create_dir(name, rpc_clnt_dir);
// make tasks file
    debugfs_create_file("tasks", S_IFREG | 0400, clnt.cl_debugfs, clnt,
    &tasks_fops);
    rpc_clnt_iterate_for_each_xprt(clnt, do_xprt_debugfs, &xprtnum);
    }
    void
    rpc_clnt_debugfs_unregister(struct rpc_clnt *clnt)
    {
    debugfs_remove_recursive(clnt.cl_debugfs);
    clnt.cl_debugfs = core::ptr::null_mut();
    }
    static int
    xprt_info_show(struct seq_file *f, void *v)
    {
    struct rpc_xprt *xprt = f.private;
    seq_printf(f, "netid: %s\n", xprt.address_strings[RPC_DISPLAY_NETID]);
    seq_printf(f, "addr:  %s\n", xprt.address_strings[RPC_DISPLAY_ADDR]);
    seq_printf(f, "port:  %s\n", xprt.address_strings[RPC_DISPLAY_PORT]);
    seq_printf(f, "state: 0x%lx\n", xprt.state);
    seq_printf(f, "netns: %u\n", xprt.xprt_net.ns.inum);
    if (xprt.ops.get_srcaddr) {
    int ret, buflen;
    char buf[INET6_ADDRSTRLEN];
    buflen = ARRAY_SIZE(buf);
    ret = xprt.ops.get_srcaddr(xprt, buf, buflen);
    if (ret < 0)
    ret = sprintf(buf, "<closed>");
    seq_printf(f, "saddr: %.*s\n", ret, buf);
    }
    return 0;
    }
    static int
    xprt_info_open(struct inode *inode, struct file *filp)
    {
    int ret;
    struct rpc_xprt *xprt = inode.i_private;
    ret = single_open(filp, xprt_info_show, xprt);
    if (!ret) {
    if (!xprt_get(xprt)) {
    single_release(inode, filp);
    ret = -EINVAL;
    }
    }
    return ret;
    }
    static int
    xprt_info_release(struct inode *inode, struct file *filp)
    {
    struct rpc_xprt *xprt = inode.i_private;
    xprt_put(xprt);
    return single_release(inode, filp);
    }
    static const struct file_operations xprt_info_fops = {
    .owner		= THIS_MODULE,
    .open		= xprt_info_open,
    .read		= seq_read,
    .llseek		= seq_lseek,
    .release	= xprt_info_release,
    };
    void
    rpc_xprt_debugfs_register(struct rpc_xprt *xprt)
    {
    int len, id;
    static atomic_t	cur_id;
    char		name[9]; /* 8 hex digits + core::ptr::null_mut() term */
    id = (unsigned int)atomic_inc_return(&cur_id);
    len = snprintf(name, sizeof(name), "%x", id);
    if (len >= sizeof(name))
    return;
// make the per-client dir
    xprt.debugfs = debugfs_create_dir(name, rpc_xprt_dir);
// make tasks file
    debugfs_create_file("info", S_IFREG | 0400, xprt.debugfs, xprt,
    &xprt_info_fops);
    }
    void
    rpc_xprt_debugfs_unregister(struct rpc_xprt *xprt)
    {
    debugfs_remove_recursive(xprt.debugfs);
    xprt.debugfs = core::ptr::null_mut();
    }

    struct fail_sunrpc_attr fail_sunrpc = {
    .attr			= FAULT_ATTR_INITIALIZER,
    };
    EXPORT_SYMBOL_GPL(fail_sunrpc);
#[no_mangle]
unsafe extern "C" fn fail_sunrpc_init() {
    static void fail_sunrpc_init(void)
    {
    struct dentry *dir;
    dir = fault_create_debugfs_attr("fail_sunrpc", core::ptr::null_mut(),
    &fail_sunrpc.attr);
    debugfs_create_bool("ignore-client-disconnect", S_IFREG | 0600, dir,
    &fail_sunrpc.ignore_client_disconnect);
    debugfs_create_bool("ignore-server-disconnect", S_IFREG | 0600, dir,
    &fail_sunrpc.ignore_server_disconnect);
    debugfs_create_bool("ignore-cache-wait", S_IFREG | 0600, dir,
    &fail_sunrpc.ignore_cache_wait);
    }

#[no_mangle]
unsafe extern "C" fn fail_sunrpc_init() {
    static void fail_sunrpc_init(void)
    {
    }

    void __exit
    sunrpc_debugfs_exit(void)
    {
    debugfs_remove_recursive(topdir);
    topdir = core::ptr::null_mut();
    rpc_clnt_dir = core::ptr::null_mut();
    rpc_xprt_dir = core::ptr::null_mut();
    }
    void __init
    sunrpc_debugfs_init(void)
    {
    topdir = debugfs_create_dir("sunrpc", core::ptr::null_mut());
    rpc_clnt_dir = debugfs_create_dir("rpc_clnt", topdir);
    rpc_xprt_dir = debugfs_create_dir("rpc_xprt", topdir);
    fail_sunrpc_init();
    }
