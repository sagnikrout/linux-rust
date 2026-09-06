//! Automatically rewritten from C to Rust
//! Source: kernel/cgroup/debug.c
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
// Debug controller
//
// WARNING: This controller is for cgroup core debugging only.
// Its interfaces are unstable and subject to changes at any time.
//

    static struct cgroup_subsys_state *
    debug_css_alloc(struct cgroup_subsys_state *parent_css)
    {
    struct cgroup_subsys_state *css = kzalloc_obj(*css);
    if (!css)
    return ERR_PTR(-ENOMEM);
    return css;
    }
#[no_mangle]
unsafe extern "C" fn debug_css_free(css: *mut cgroup_subsys_state) {
    static void debug_css_free(struct cgroup_subsys_state *css)
    {
    kfree(css);
    }
//
// debug_taskcount_read - return the number of tasks in a cgroup.
// @cgrp: the cgroup in question
//
    static u64 debug_taskcount_read(struct cgroup_subsys_state *css,
    struct cftype *cft)
    {
    return cgroup_task_count(css.cgroup);
    }
#[no_mangle]
unsafe extern "C" fn current_css_set_read(seq: *mut seq_file, v: *mut c_void) -> c_int {
    static int current_css_set_read(struct seq_file *seq, void *v)
    {
    struct kernfs_open_file *of = seq.private;
    struct css_set *cset;
    struct cgroup_subsys *ss;
    struct cgroup_subsys_state *css;
    int i, refcnt;
    if (!cgroup_kn_lock_live(of.kn, false))
    return -ENODEV;
    spin_lock_irq(&css_set_lock);
    cset = task_css_set(current);
    refcnt = refcount_read(&cset.refcount);
    seq_printf(seq, "css_set %pK %d", cset, refcnt);
    if (refcnt > cset.nr_tasks)
    seq_printf(seq, " +%d", refcnt - cset.nr_tasks);
    seq_puts(seq, "\n");
//
// Print the css'es stored in the current css_set.
//
    for_each_subsys(ss, i) {
    css = cset.subsys[ss.id];
    if (!css)
    continue;
    seq_printf(seq, "%2d: %-4s\t- %p[%d]\n", ss.id, ss.name,
    css, css.id);
    }
    spin_unlock_irq(&css_set_lock);
    cgroup_kn_unlock(of.kn);
    return 0;
    }
    static u64 current_css_set_refcount_read(struct cgroup_subsys_state *css,
    struct cftype *cft)
    {
    u64 count;
    rcu_read_lock();
    count = refcount_read(&task_css_set(current).refcount);
    rcu_read_unlock();
    return count;
    }
#[no_mangle]
unsafe extern "C" fn current_css_set_cg_links_read(seq: *mut seq_file, v: *mut c_void) -> c_int {
    static int current_css_set_cg_links_read(struct seq_file *seq, void *v)
    {
    struct cgrp_cset_link *link;
    struct css_set *cset;
    char *name_buf;
    name_buf = kmalloc(NAME_MAX + 1, GFP_KERNEL);
    if (!name_buf)
    return -ENOMEM;
    spin_lock_irq(&css_set_lock);
    cset = task_css_set(current);
    list_for_each_entry(link, &cset.cgrp_links, cgrp_link) {
    struct cgroup *c = link.cgrp;
    cgroup_name(c, name_buf, NAME_MAX + 1);
    seq_printf(seq, "Root %d group %s\n",
    c.root.hierarchy_id, name_buf);
    }
    spin_unlock_irq(&css_set_lock);
    kfree(name_buf);
    return 0;
    }
pub const MAX_TASKS_SHOWN_PER_CSS: c_int = 25;
#[no_mangle]
unsafe extern "C" fn cgroup_css_links_read(seq: *mut seq_file, v: *mut c_void) -> c_int {
    static int cgroup_css_links_read(struct seq_file *seq, void *v)
    {
    struct cgroup_subsys_state *css = seq_css(seq);
    struct cgrp_cset_link *link;
    let mut dead_cnt: c_int = 0, extra_refs = 0, threaded_csets = 0;
    spin_lock_irq(&css_set_lock);
    list_for_each_entry(link, &css.cgroup.cset_links, cset_link) {
    struct css_set *cset = link.cset;
    struct task_struct *task;
    let mut count: c_int = 0;
    let mut refcnt: c_int = refcount_read(&cset.refcount);
//
// Print out the proc_cset and threaded_cset relationship
// and highlight difference between refcount and task_count.
//
    seq_printf(seq, "css_set %pK", cset);
    if (rcu_dereference_protected(cset.dom_cset, 1) != cset) {
    threaded_csets++;
    seq_printf(seq, "=>%pK", cset.dom_cset);
    }
    if (!list_empty(&cset.threaded_csets)) {
    struct css_set *tcset;
    let mut idx: c_int = 0;
    list_for_each_entry(tcset, &cset.threaded_csets,
    threaded_csets_node) {
    seq_puts(seq, idx ? "," : "<=");
    seq_printf(seq, "%pK", tcset);
    idx++;
    }
    } else {
    seq_printf(seq, " %d", refcnt);
    if (refcnt - cset.nr_tasks > 0) {
    let mut extra: c_int = refcnt - cset.nr_tasks;
    seq_printf(seq, " +%d", extra);
//
// Take out the one additional reference in
// init_css_set.
//
    if (cset == &init_css_set)
    extra--;
    extra_refs += extra;
    }
    }
    seq_puts(seq, "\n");
    list_for_each_entry(task, &cset.tasks, cg_list) {
    if (count++ <= MAX_TASKS_SHOWN_PER_CSS)
    seq_printf(seq, "  task %d\n",
    task_pid_vnr(task));
    }
    list_for_each_entry(task, &cset.mg_tasks, cg_list) {
    if (count++ <= MAX_TASKS_SHOWN_PER_CSS)
    seq_printf(seq, "  task %d\n",
    task_pid_vnr(task));
    }
// show # of overflowed tasks
    if (count > MAX_TASKS_SHOWN_PER_CSS)
    seq_printf(seq, "  ... (%d)\n",
    count - MAX_TASKS_SHOWN_PER_CSS);
    if (cset.dead) {
    seq_puts(seq, "    [dead]\n");
    dead_cnt++;
    }
    WARN_ON(count != cset.nr_tasks);
    }
    spin_unlock_irq(&css_set_lock);
    if (!dead_cnt && !extra_refs && !threaded_csets)
    return 0;
    seq_puts(seq, "\n");
    if (threaded_csets)
    seq_printf(seq, "threaded css_sets = %d\n", threaded_csets);
    if (extra_refs)
    seq_printf(seq, "extra references = %d\n", extra_refs);
    if (dead_cnt)
    seq_printf(seq, "dead css_sets = %d\n", dead_cnt);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cgroup_subsys_states_read(seq: *mut seq_file, v: *mut c_void) -> c_int {
    static int cgroup_subsys_states_read(struct seq_file *seq, void *v)
    {
    struct kernfs_open_file *of = seq.private;
    struct cgroup *cgrp;
    struct cgroup_subsys *ss;
    struct cgroup_subsys_state *css;
    char pbuf[16];
    int i;
    cgrp = cgroup_kn_lock_live(of.kn, false);
    if (!cgrp)
    return -ENODEV;
    for_each_subsys(ss, i) {
    css = rcu_dereference_check(cgrp.subsys[ss.id], true);
    if (!css)
    continue;
    pbuf[0] = '\0';
// Show the parent CSS if applicable
    if (css.parent)
    snprintf(pbuf, sizeof(pbuf) - 1, " P=%d",
    css.parent.id);
    seq_printf(seq, "%2d: %-4s\t- %p[%d] %d%s\n", ss.id, ss.name,
    css, css.id,
    atomic_read(&css.online_cnt), pbuf);
    }
    cgroup_kn_unlock(of.kn);
    return 0;
    }
    static void cgroup_masks_read_one(struct seq_file *seq, const char *name,
    u32 mask)
    {
    struct cgroup_subsys *ss;
    int ssid;
    let mut first: bool = true;
    seq_printf(seq, "%-17s: ", name);
    for_each_subsys(ss, ssid) {
    if (!(mask & (1 << ssid)))
    continue;
    if (!first)
    seq_puts(seq, ", ");
    seq_puts(seq, ss.name);
    first = false;
    }
    seq_putc(seq, '\n');
    }
#[no_mangle]
unsafe extern "C" fn cgroup_masks_read(seq: *mut seq_file, v: *mut c_void) -> c_int {
    static int cgroup_masks_read(struct seq_file *seq, void *v)
    {
    struct kernfs_open_file *of = seq.private;
    struct cgroup *cgrp;
    cgrp = cgroup_kn_lock_live(of.kn, false);
    if (!cgrp)
    return -ENODEV;
    cgroup_masks_read_one(seq, "subtree_control", cgrp.subtree_control);
    cgroup_masks_read_one(seq, "subtree_ss_mask", cgrp.subtree_ss_mask);
    cgroup_kn_unlock(of.kn);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn releasable_read(css: *mut cgroup_subsys_state, cft: *mut cftype) -> u64 {
    static u64 releasable_read(struct cgroup_subsys_state *css, struct cftype *cft)
    {
    return (!cgroup_is_populated(css.cgroup) &&
    !css_has_online_children(&css.cgroup.self));
    }
    static struct cftype debug_legacy_files[] =  {
    {
    .name = "taskcount",
    .read_u64 = debug_taskcount_read,
    },
    {
    .name = "current_css_set",
    .seq_show = current_css_set_read,
    .flags = CFTYPE_ONLY_ON_ROOT,
    },
    {
    .name = "current_css_set_refcount",
    .read_u64 = current_css_set_refcount_read,
    .flags = CFTYPE_ONLY_ON_ROOT,
    },
    {
    .name = "current_css_set_cg_links",
    .seq_show = current_css_set_cg_links_read,
    .flags = CFTYPE_ONLY_ON_ROOT,
    },
    {
    .name = "cgroup_css_links",
    .seq_show = cgroup_css_links_read,
    },
    {
    .name = "cgroup_subsys_states",
    .seq_show = cgroup_subsys_states_read,
    },
    {
    .name = "cgroup_masks",
    .seq_show = cgroup_masks_read,
    },
    {
    .name = "releasable",
    .read_u64 = releasable_read,
    },
    { }	/* terminate */
    };
    static struct cftype debug_files[] =  {
    {
    .name = "taskcount",
    .read_u64 = debug_taskcount_read,
    },
    {
    .name = "current_css_set",
    .seq_show = current_css_set_read,
    .flags = CFTYPE_ONLY_ON_ROOT,
    },
    {
    .name = "current_css_set_refcount",
    .read_u64 = current_css_set_refcount_read,
    .flags = CFTYPE_ONLY_ON_ROOT,
    },
    {
    .name = "current_css_set_cg_links",
    .seq_show = current_css_set_cg_links_read,
    .flags = CFTYPE_ONLY_ON_ROOT,
    },
    {
    .name = "css_links",
    .seq_show = cgroup_css_links_read,
    },
    {
    .name = "csses",
    .seq_show = cgroup_subsys_states_read,
    },
    {
    .name = "masks",
    .seq_show = cgroup_masks_read,
    },
    { }	/* terminate */
    };
    struct cgroup_subsys debug_cgrp_subsys = {
    .css_alloc	= debug_css_alloc,
    .css_free	= debug_css_free,
    .legacy_cftypes	= debug_legacy_files,
    };
//
// On v2, debug is an implicit controller enabled by "cgroup_debug" boot
// parameter.
//
#[no_mangle]
pub unsafe extern "C" fn enable_debug_cgroup() -> void __init {
    void __init enable_debug_cgroup(void)
    {
    debug_cgrp_subsys.dfl_cftypes = debug_files;
    debug_cgrp_subsys.implicit_on_dfl = true;
    debug_cgrp_subsys.threaded = true;
    }
