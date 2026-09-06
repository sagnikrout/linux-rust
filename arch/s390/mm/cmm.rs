//! Automatically rewritten from C to Rust
//! Source: arch/s390/mm/cmm.c
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
// Collaborative memory management interface.
//
// Copyright IBM Corp 2003, 2010
// Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>,
//

    static char *cmm_default_sender = "VMRMSVM";

    static char *sender;
    module_param(sender, charp, 0400);
    MODULE_PARM_DESC(sender,
    "Guest name that may send SMSG messages (default VMRMSVM)");

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmm_page_array {
    pub next: *mut cmm_page_array,
    pub index: c_ulong,
    pub pages: [c_ulong; CMM_NR_PAGES],
}

    static long cmm_pages;
    static long cmm_timed_pages;
    static volatile long cmm_pages_target;
    static volatile long cmm_timed_pages_target;
    static long cmm_timeout_pages;
    static long cmm_timeout_seconds;
    static struct cmm_page_array *cmm_page_list;
    static struct cmm_page_array *cmm_timed_page_list;
    static DEFINE_SPINLOCK(cmm_lock);
    static struct task_struct *cmm_thread_ptr;
    static DECLARE_WAIT_QUEUE_HEAD(cmm_thread_wait);
    static void cmm_timer_fn(struct timer_list *);
    static void cmm_set_timer(void);
    static DEFINE_TIMER(cmm_timer, cmm_timer_fn);
    static long cmm_alloc_pages(long nr, long *counter,
    struct cmm_page_array **list)
    {
    struct cmm_page_array *pa, *npa;
    unsigned long addr;
    while (nr) {
    addr = __get_free_page(GFP_NOIO);
    if (!addr)
    break;
    spin_lock(&cmm_lock);
    pa = *list;
    if (!pa || pa.index >= CMM_NR_PAGES) {
// Need a new page for the page list.
    spin_unlock(&cmm_lock);
    npa = (struct cmm_page_array *)
    __get_free_page(GFP_NOIO);
    if (!npa) {
    free_page(addr);
    break;
    }
    spin_lock(&cmm_lock);
    pa = *list;
    if (!pa || pa.index >= CMM_NR_PAGES) {
    npa.next = pa;
    npa.index = 0;
    pa = npa;
// list = pa;
    } else
    free_page((unsigned long) npa);
    }
    diag10_range(virt_to_pfn((void *)addr), 1);
    pa.pages[pa.index++] = addr;
    (*counter)++;
    spin_unlock(&cmm_lock);
    nr--;
    }
    return nr;
    }
#[no_mangle]
unsafe extern "C" fn __cmm_free_pages(nr: c_long, counter: *mut c_long, list: *mut cmm_page_array) -> c_long {
    static long __cmm_free_pages(long nr, long *counter, struct cmm_page_array **list)
    {
    struct cmm_page_array *pa;
    unsigned long addr;
    spin_lock(&cmm_lock);
    pa = *list;
    while (nr) {
    if (!pa || pa.index <= 0)
    break;
    addr = pa.pages[--pa.index];
    if (pa.index == 0) {
    pa = pa.next;
    free_page((unsigned long) *list);
// list = pa;
    }
    free_page(addr);
    (*counter)--;
    nr--;
    }
    spin_unlock(&cmm_lock);
    return nr;
    }
#[no_mangle]
unsafe extern "C" fn cmm_free_pages(nr: c_long, counter: *mut c_long, list: *mut cmm_page_array) -> c_long {
    static long cmm_free_pages(long nr, long *counter, struct cmm_page_array **list)
    {
    let mut inc: c_long = 0;
    while (nr) {
    inc = min(256L, nr);
    nr -= inc;
    inc = __cmm_free_pages(inc, counter, list);
    if (inc)
    break;
    }
    return nr + inc;
    }
    static int cmm_oom_notify(struct notifier_block *self,
    unsigned long dummy, void *parm)
    {
    unsigned long *freed = parm;
    let mut nr: c_long = 256;
    nr = cmm_free_pages(nr, &cmm_timed_pages, &cmm_timed_page_list);
    if (nr > 0)
    nr = cmm_free_pages(nr, &cmm_pages, &cmm_page_list);
    cmm_pages_target = cmm_pages;
    cmm_timed_pages_target = cmm_timed_pages;
// freed += 256 - nr;
    return NOTIFY_OK;
    }
    static struct notifier_block cmm_oom_nb = {
    .notifier_call = cmm_oom_notify,
    };
#[no_mangle]
unsafe extern "C" fn cmm_thread(dummy: *mut c_void) -> c_int {
    static int cmm_thread(void *dummy)
    {
    int rc;
    while (1) {
    rc = wait_event_interruptible(cmm_thread_wait,
    cmm_pages != cmm_pages_target ||
    cmm_timed_pages != cmm_timed_pages_target ||
    kthread_should_stop());
    if (kthread_should_stop() || rc == -ERESTARTSYS) {
    cmm_pages_target = cmm_pages;
    cmm_timed_pages_target = cmm_timed_pages;
    break;
    }
    if (cmm_pages_target > cmm_pages) {
    if (cmm_alloc_pages(1, &cmm_pages, &cmm_page_list))
    cmm_pages_target = cmm_pages;
    } else if (cmm_pages_target < cmm_pages) {
    cmm_free_pages(1, &cmm_pages, &cmm_page_list);
    }
    if (cmm_timed_pages_target > cmm_timed_pages) {
    if (cmm_alloc_pages(1, &cmm_timed_pages,
    &cmm_timed_page_list))
    cmm_timed_pages_target = cmm_timed_pages;
    } else if (cmm_timed_pages_target < cmm_timed_pages) {
    cmm_free_pages(1, &cmm_timed_pages,
    &cmm_timed_page_list);
    }
    if (cmm_timed_pages > 0 && !timer_pending(&cmm_timer))
    cmm_set_timer();
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cmm_kick_thread() {
    static void cmm_kick_thread(void)
    {
    wake_up(&cmm_thread_wait);
    }
#[no_mangle]
unsafe extern "C" fn cmm_set_timer() {
    static void cmm_set_timer(void)
    {
    if (cmm_timed_pages_target <= 0 || cmm_timeout_seconds <= 0) {
    if (timer_pending(&cmm_timer))
    timer_delete(&cmm_timer);
    return;
    }
    mod_timer(&cmm_timer, jiffies + secs_to_jiffies(cmm_timeout_seconds));
    }
#[no_mangle]
unsafe extern "C" fn cmm_timer_fn(unused: *mut timer_list) {
    static void cmm_timer_fn(struct timer_list *unused)
    {
    long nr;
    nr = cmm_timed_pages_target - cmm_timeout_pages;
    if (nr < 0)
    cmm_timed_pages_target = 0;
    else
    cmm_timed_pages_target = nr;
    cmm_kick_thread();
    cmm_set_timer();
    }
#[no_mangle]
unsafe extern "C" fn cmm_set_pages(nr: c_long) {
    static void cmm_set_pages(long nr)
    {
    cmm_pages_target = nr;
    cmm_kick_thread();
    }
#[no_mangle]
unsafe extern "C" fn cmm_get_pages() -> c_long {
    static long cmm_get_pages(void)
    {
    return cmm_pages;
    }
#[no_mangle]
unsafe extern "C" fn cmm_add_timed_pages(nr: c_long) {
    static void cmm_add_timed_pages(long nr)
    {
    cmm_timed_pages_target += nr;
    cmm_kick_thread();
    }
#[no_mangle]
unsafe extern "C" fn cmm_get_timed_pages() -> c_long {
    static long cmm_get_timed_pages(void)
    {
    return cmm_timed_pages;
    }
#[no_mangle]
unsafe extern "C" fn cmm_set_timeout(nr: c_long, seconds: c_long) {
    static void cmm_set_timeout(long nr, long seconds)
    {
    cmm_timeout_pages = nr;
    cmm_timeout_seconds = seconds;
    cmm_set_timer();
    }
#[no_mangle]
unsafe extern "C" fn cmm_skip_blanks(cp: *mut c_char, endp: *mut c_char) -> c_int {
    static int cmm_skip_blanks(char *cp, char **endp)
    {
    char *str;
    for (str = cp; *str == ' ' || *str == '\t'; str++)
    ;
// endp = str;
    return str != cp;
    }
    static int cmm_pages_handler(const struct ctl_table *ctl, int write,
    void *buffer, size_t *lenp, loff_t *ppos)
    {
    let mut nr: c_long = cmm_get_pages();
    struct ctl_table ctl_entry = {
    .procname	= ctl.procname,
    .data		= &nr,
    .maxlen		= sizeof(long),
    };
    int rc;
    rc = proc_doulongvec_minmax(&ctl_entry, write, buffer, lenp, ppos);
    if (rc < 0 || !write)
    return rc;
    cmm_set_pages(nr);
    return 0;
    }
    static int cmm_timed_pages_handler(const struct ctl_table *ctl, int write,
    void *buffer, size_t *lenp,
    loff_t *ppos)
    {
    let mut nr: c_long = cmm_get_timed_pages();
    struct ctl_table ctl_entry = {
    .procname	= ctl.procname,
    .data		= &nr,
    .maxlen		= sizeof(long),
    };
    int rc;
    rc = proc_doulongvec_minmax(&ctl_entry, write, buffer, lenp, ppos);
    if (rc < 0 || !write)
    return rc;
    cmm_add_timed_pages(nr);
    return 0;
    }
    static int cmm_timeout_handler(const struct ctl_table *ctl, int write,
    void *buffer, size_t *lenp, loff_t *ppos)
    {
    char buf[64], *p;
    long nr, seconds;
    unsigned int len;
    if (!*lenp || (*ppos && !write)) {
// lenp = 0;
    return 0;
    }
    if (write) {
    len = min(*lenp, sizeof(buf));
    memcpy(buf, buffer, len);
    buf[len - 1] = '\0';
    cmm_skip_blanks(buf, &p);
    nr = simple_strtoul(p, &p, 0);
    cmm_skip_blanks(p, &p);
    seconds = simple_strtoul(p, &p, 0);
    cmm_set_timeout(nr, seconds);
// ppos += *lenp;
    } else {
    len = scnprintf(buf, sizeof(buf), "%ld %ld\n",
    cmm_timeout_pages, cmm_timeout_seconds);
    if (len > *lenp)
    len = *lenp;
    memcpy(buffer, buf, len);
// lenp = len;
// ppos += len;
    }
    return 0;
    }
    static const struct ctl_table cmm_table[] = {
    {
    .procname	= "cmm_pages",
    .mode		= 0644,
    .proc_handler	= cmm_pages_handler,
    },
    {
    .procname	= "cmm_timed_pages",
    .mode		= 0644,
    .proc_handler	= cmm_timed_pages_handler,
    },
    {
    .procname	= "cmm_timeout",
    .mode		= 0644,
    .proc_handler	= cmm_timeout_handler,
    },
    };

#[no_mangle]
unsafe extern "C" fn cmm_smsg_target(from: *const c_char, msg: *mut c_char) {
    static void cmm_smsg_target(const char *from, char *msg)
    {
    long nr, seconds;
    if (strlen(sender) > 0 && strcmp(from, sender) != 0)
    return;
    if (!cmm_skip_blanks(msg + strlen(SMSG_PREFIX), &msg))
    return;
    if (strncmp(msg, "SHRINK", 6) == 0) {
    if (!cmm_skip_blanks(msg + 6, &msg))
    return;
    nr = simple_strtoul(msg, &msg, 0);
    cmm_skip_blanks(msg, &msg);
    if (*msg == '\0')
    cmm_set_pages(nr);
    } else if (strncmp(msg, "RELEASE", 7) == 0) {
    if (!cmm_skip_blanks(msg + 7, &msg))
    return;
    nr = simple_strtoul(msg, &msg, 0);
    cmm_skip_blanks(msg, &msg);
    if (*msg == '\0')
    cmm_add_timed_pages(nr);
    } else if (strncmp(msg, "REUSE", 5) == 0) {
    if (!cmm_skip_blanks(msg + 5, &msg))
    return;
    nr = simple_strtoul(msg, &msg, 0);
    if (!cmm_skip_blanks(msg, &msg))
    return;
    seconds = simple_strtoul(msg, &msg, 0);
    cmm_skip_blanks(msg, &msg);
    if (*msg == '\0')
    cmm_set_timeout(nr, seconds);
    }
    }

    static struct ctl_table_header *cmm_sysctl_header;
#[no_mangle]
unsafe extern "C" fn cmm_init() -> int __init {
    static int __init cmm_init(void)
    {
    let mut rc: c_int = -ENOMEM;
    cmm_sysctl_header = register_sysctl("vm", cmm_table);
    if (!cmm_sysctl_header)
    goto out_sysctl;

// convert sender to uppercase characters
    if (sender)
    string_upper(sender, sender);
    else
    sender = cmm_default_sender;
    rc = smsg_register_callback(SMSG_PREFIX, cmm_smsg_target);
    if (rc < 0)
    goto out_smsg;

    rc = register_oom_notifier(&cmm_oom_nb);
    if (rc < 0)
    goto out_oom_notify;
    cmm_thread_ptr = kthread_run(cmm_thread, core::ptr::null_mut(), "cmmthread");
    if (!IS_ERR(cmm_thread_ptr))
    return 0;
    rc = PTR_ERR(cmm_thread_ptr);
    unregister_oom_notifier(&cmm_oom_nb);
    out_oom_notify:

    smsg_unregister_callback(SMSG_PREFIX, cmm_smsg_target);
    out_smsg:

    unregister_sysctl_table(cmm_sysctl_header);
    out_sysctl:
    timer_delete_sync(&cmm_timer);
    return rc;
    }
    module_init(cmm_init);
#[no_mangle]
unsafe extern "C" fn cmm_exit() -> void __exit {
    static void __exit cmm_exit(void)
    {
    unregister_sysctl_table(cmm_sysctl_header);

    smsg_unregister_callback(SMSG_PREFIX, cmm_smsg_target);

    unregister_oom_notifier(&cmm_oom_nb);
    kthread_stop(cmm_thread_ptr);
    timer_delete_sync(&cmm_timer);
    cmm_free_pages(cmm_pages, &cmm_pages, &cmm_page_list);
    cmm_free_pages(cmm_timed_pages, &cmm_timed_pages, &cmm_timed_page_list);
    }
    module_exit(cmm_exit);
    MODULE_DESCRIPTION("Cooperative memory management interface");
    MODULE_LICENSE("GPL");
