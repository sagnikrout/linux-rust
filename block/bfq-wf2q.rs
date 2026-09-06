//! Automatically rewritten from C to Rust
//! Source: block/bfq-wf2q.c
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
// === KERNEL_MACRO_PRELUDE_START ===
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
macro_rules! DEFINE { ($($tt:tt)*) => {}; }
macro_rules! ARRAY_SIZE { ($($tt:tt)*) => { 1 }; }
macro_rules! container_of { ($($tt:tt)*) => { core::ptr::null_mut() }; }
macro_rules! sizeof { ($($tt:tt)*) => { 0usize }; }
macro_rules! IS_ENABLED { ($($tt:tt)*) => { false }; }
macro_rules! DECLARE_WORK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_WAKE_Q { ($($tt:tt)*) => {}; }
macro_rules! LLIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! LIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! SET_UID { ($($tt:tt)*) => {}; }
macro_rules! SET_GID { ($($tt:tt)*) => {}; }
macro_rules! list_for_each_entry { ($($tt:tt)*) => { if false }; }
macro_rules! list_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! llist_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! pr_info_once { ($($tt:tt)*) => {}; }
macro_rules! pr_info { ($($tt:tt)*) => {}; }
macro_rules! pr_warn { ($($tt:tt)*) => {}; }
macro_rules! pr_err { ($($tt:tt)*) => {}; }
macro_rules! pr_debug { ($($tt:tt)*) => {}; }
macro_rules! early_param { ($($tt:tt)*) => {}; }
macro_rules! BUILD_BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! WARN_ON { ($($tt:tt)*) => { false }; }
macro_rules! WARN_ON_ONCE { ($($tt:tt)*) => { false }; }
macro_rules! BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! BUG { () => {}; }
macro_rules! IS_ERR { ($($tt:tt)*) => { false }; }
macro_rules! PTR_ERR { ($($tt:tt)*) => { 0 }; }
macro_rules! ERR_PTR { ($($tt:tt)*) => { core::ptr::null_mut() }; }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cred { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct notifier_block { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_notifier_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_header { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_root { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_set { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kern_ipc_perm { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_params { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_queue { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msgseg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_sender { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_receiver { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sembuf { pub sem_num: u16, pub sem_op: i16, pub sem_flg: i16 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem_array { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmid_kernel { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shm_file_data { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wake_q_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct work_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct llist_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;
pub type key_t = i32;
pub type kuid_t = u32;
pub type kgid_t = u32;
pub type int = c_int;
pub type uint = c_uint;
pub type ulong = c_ulong;
pub type long = c_long;
pub type void = c_void;

// Standard Linux Error Codes
pub const EPERM: c_int = 1;
pub const ENOENT: c_int = 2;
pub const ESRCH: c_int = 3;
pub const EINTR: c_int = 4;
pub const EIO: c_int = 5;
pub const ENXIO: c_int = 6;
pub const E2BIG: c_int = 7;
pub const ENOEXEC: c_int = 8;
pub const EBADF: c_int = 9;
pub const ECHILD: c_int = 10;
pub const EAGAIN: c_int = 11;
pub const ENOMEM: c_int = 12;
pub const EACCES: c_int = 13;
pub const EFAULT: c_int = 14;
pub const EBUSY: c_int = 16;
pub const EEXIST: c_int = 17;
pub const EXDEV: c_int = 18;
pub const ENODEV: c_int = 19;
pub const ENOTDIR: c_int = 20;
pub const EISDIR: c_int = 21;
pub const EINVAL: c_int = 22;
pub const ENFILE: c_int = 23;
pub const EMFILE: c_int = 24;
pub const ENOSPC: c_int = 28;
pub const EROFS: c_int = 30;
pub const EIDRM: c_int = 43;
pub const EOPNOTSUPP: c_int = 95;
pub const ENOTSUPP: c_int = 524;

// Standard Memory Constants
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const GFP_KERNEL: c_uint = 0xcc0;
pub const GFP_ATOMIC: c_uint = 0x80000;
pub const GFP_NOWAIT: c_uint = 0;

// Standard Core Primitives
extern "C" {
    pub static current: *mut task_struct;
    pub fn printk(fmt: *const c_char, ...) -> c_int;
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    pub fn kfree(ptr: *mut c_void);
}
// === KERNEL_MACRO_PRELUDE_END ===


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Hierarchical Budget Worst-case Fair Weighted Fair Queueing
// (B-WF2Q+): hierarchical scheduling algorithm by which the BFQ I/O
// scheduler schedules generic entities. The latter can represent
// either single bfq queues (associated with processes) or groups of
// bfq queues (associated with cgroups).
//

//
// bfq_gt - compare two timestamps.
// @a: first ts.
// @b: second ts.
//
// Return @a > @b, dealing with wrapping correctly.
//
#[no_mangle]
unsafe extern "C" fn bfq_gt(a: u64, b: u64) -> c_int {
    return (s64)(a - b) > 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bfq_root_active_entity(tree: *mut rb_root) -> *mut c_void {
    let mut node = tree.rb_node;
    return rb_entry(node, bfq_entity, rb_node);
    }
#[no_mangle]
unsafe extern "C" fn bfq_class_idx(entity: *mut bfq_entity) -> c_uint {
    let mut bfqq = bfq_entity_to_bfqq(entity);
    return bfqq ? bfqq.ioprio_class - 1 :
    BFQ_DEFAULT_GRP_CLASS - 1;
    }
#[no_mangle]
pub unsafe extern "C" fn bfq_tot_busy_queues(bfqd: *mut bfq_data) -> c_uint {
    return bfqd.busy_queues[0] + bfqd.busy_queues[1] +
    bfqd.busy_queues[2];
    }
// forward_decl: bfq_lookup_next_entity;
// forward_decl: bfq_update_parent_budget;
//
// bfq_update_next_in_service - update sd->next_in_service
// @sd: sched_data for which to perform the update.
// @new_entity: if not NULL, pointer to the entity whose activation,
// requeueing or repositioning triggered the invocation of
// this function.
// @expiration: id true, this function is being invoked after the
// expiration of the in-service entity
//
// This function is called to update sd->next_in_service, which, in
// its turn, may change as a consequence of the insertion or
// extraction of an entity into/from one of the active trees of
// sd. These insertions/extractions occur as a consequence of
// activations/deactivations of entities, with some activations being
// 'true' activations, and other activations being requeueings (i.e.,
// implementing the second, requeueing phase of the mechanism used to
// reposition an entity in its active tree; see comments on
// __bfq_activate_entity and __bfq_requeue_entity for details). In
// both the last two activation sub-cases, new_entity points to the
// just activated or requeued entity.
//
// Returns true if sd->next_in_service changes in such a way that
// entity->parent may become the next_in_service for its parent
// entity.
//
#[no_mangle]
pub unsafe extern "C" fn bfq_update_next_in_service(sd: *mut bfq_sched_data, new_entity: *mut bfq_entity, expiration: bool) -> bool {
    let mut next_in_service = sd.next_in_service;
pub static mut parent_sched_may_change: bool = false;
pub static mut change_without_lookup: bool = false;
//
// If this update is triggered by the activation, requeueing
// or repositioning of an entity that does not coincide with
// sd->next_in_service, then a full lookup in the active tree
// can be avoided. In fact, it is enough to check whether the
// just-modified entity has the same priority as
// sd->next_in_service, is eligible and has a lower virtual
// finish time than sd->next_in_service. If this compound
// condition holds, then the new entity becomes the new
// next_in_service. Otherwise no change is needed.
//
    if (new_entity && new_entity != sd.next_in_service) {
//
// Flag used to decide whether to replace
// sd->next_in_service with new_entity. Tentatively
// set to true, and left as true if
// sd->next_in_service is NULL.
//
    change_without_lookup = true;
//
// If there is already a next_in_service candidate
// entity, then compare timestamps to decide whether
// to replace sd->service_tree with new_entity.
//
    if (next_in_service) {
    let mut new_entity_class_idx = bfq_class_idx(new_entity);
    let mut st = sd.service_tree + new_entity_class_idx;
    change_without_lookup =
    (new_entity_class_idx ==
    bfq_class_idx(next_in_service)
    &&
    !bfq_gt(new_entity.start, st.vtime)
    &&
    bfq_gt(next_in_service.finish,
    new_entity.finish));
    }
    if (change_without_lookup) {
    next_in_service = new_entity;
    }
    }
    if (!change_without_lookup) /* lookup needed */ {
    next_in_service = bfq_lookup_next_entity(sd, expiration);
    }
    if (next_in_service) {
    let mut new_budget_triggers_change = bfq_update_parent_budget(next_in_service);
    parent_sched_may_change = !sd.next_in_service ||
    new_budget_triggers_change;
    }
    sd.next_in_service = next_in_service;
    return parent_sched_may_change;
    }

//
// Returns true if this budget changes may let next_in_service->parent
// become the next_in_service entity for its parent entity.
//
#[no_mangle]
unsafe extern "C" fn bfq_update_parent_budget(next_in_service: *mut bfq_entity) -> bool {
pub static mut bfqg_entity: *mut c_void = core::ptr::null_mut();
pub static mut bfqg: *mut c_void = core::ptr::null_mut();
pub static mut group_sd: *mut c_void = core::ptr::null_mut();
pub static mut ret: bool = false;
    group_sd = next_in_service.sched_data;
    bfqg = container_of!(group_sd, bfq_group, sched_data);
//
// bfq_group's my_entity field is not NULL only if the group
// is not the root group. We must not touch the root entity
// as it must never become an in-service entity.
//
    bfqg_entity = bfqg.my_entity;
    if (bfqg_entity) {
    if (bfqg_entity.budget > next_in_service.budget) {
    ret = true;
    }
    bfqg_entity.budget = next_in_service.budget;
    }
    return ret;
    }
//
// This function tells whether entity stops being a candidate for next
// service, according to the restrictive definition of the field
// next_in_service. In particular, this function is invoked for an
// entity that is about to be set in service.
//
// If entity is a queue, then the entity is no longer a candidate for
// next service according to the that definition, because entity is
// about to become the in-service queue. This function then returns
// true if entity is a queue.
//
// In contrast, entity could still be a candidate for next service if
// it is not a queue, and has more than one active child. In fact,
// even if one of its children is about to be set in service, other
// active children may still be the next to serve, for the parent
// entity, even according to the above definition. As a consequence, a
// non-queue entity is not a candidate for next-service only if it has
// only one active child. And only if this condition holds, then this
// function returns true for a non-queue entity.
//
#[no_mangle]
unsafe extern "C" fn bfq_no_longer_next_in_service(entity: *mut bfq_entity) -> bool {
pub static mut bfqg: *mut c_void = core::ptr::null_mut();
    if (bfq_entity_to_bfqq(entity)) {
    return true;
    }
    bfqg = container_of!(entity, bfq_group, entity);
//
// The field active_entities does not always contain the
// actual number of active children entities: it happens to
// not account for the in-service entity in case the latter is
// removed from its active tree (which may get done after
// invoking the function bfq_no_longer_next_in_service in
// bfq_get_next_queue). Fortunately, here, i.e., while
// bfq_no_longer_next_in_service is not yet completed in
// bfq_get_next_queue, bfq_active_extract has not yet been
// invoked, and thus active_entities still coincides with the
// actual number of active entities.
//
    if (bfqg.active_entities == 1) {
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn bfq_inc_active_entities(entity: *mut bfq_entity) {
    let mut sd = entity.sched_data;
    let mut bfqg = container_of!(sd, bfq_group, sched_data);
    if (bfqg != bfqg.bfqd.root_group) {
    bfqg.active_entities += 1;
    }
    }
#[no_mangle]
unsafe extern "C" fn bfq_dec_active_entities(entity: *mut bfq_entity) {
    let mut sd = entity.sched_data;
    let mut bfqg = container_of!(sd, bfq_group, sched_data);
    if (bfqg != bfqg.bfqd.root_group) {
    bfqg.active_entities -= 1;
    }
    }

#[no_mangle]
unsafe extern "C" fn bfq_update_parent_budget(next_in_service: *mut bfq_entity) -> bool {
    return false;
    }
#[no_mangle]
unsafe extern "C" fn bfq_no_longer_next_in_service(entity: *mut bfq_entity) -> bool {
    return true;
    }
#[no_mangle]
unsafe extern "C" fn bfq_inc_active_entities(entity: *mut bfq_entity) {
    }
#[no_mangle]
unsafe extern "C" fn bfq_dec_active_entities(entity: *mut bfq_entity) {
    }

//
// Shift for timestamp calculations.  This actually limits the maximum
// service allowed in one timestamp delta (small shift values increase it),
// the maximum total weight that can be used for the queues in the system
// (big shift values increase it), and the period of virtual time
// wraparounds.
//
pub const WFQ_SERVICE_SHIFT: c_int = 22;
#[no_mangle]
pub unsafe extern "C" fn bfq_entity_to_bfqq(entity: *mut bfq_entity) -> *mut c_void {
    let mut bfqq = core::ptr::null_mut();
    if (!entity.my_sched_data) {
    bfqq = container_of!(entity, bfq_queue, entity);
    }
    return bfqq;
    }
//
// bfq_delta - map service into the virtual time domain.
// @service: amount of service.
// @weight: scale factor (weight of an entity or weight sum).
//
#[no_mangle]
unsafe extern "C" fn bfq_delta(service: c_ulong, weight: c_ulong) -> u64 {
    return div64_ul((u64)service << WFQ_SERVICE_SHIFT, weight);
    }
//
// bfq_calc_finish - assign the finish time to an entity.
// @entity: the entity to act upon.
// @service: the service to be charged to the entity.
//
#[no_mangle]
unsafe extern "C" fn bfq_calc_finish(entity: *mut bfq_entity, service: c_ulong) {
    let mut bfqq = bfq_entity_to_bfqq(entity);
    entity.finish = entity.start +
    bfq_delta(service, entity.weight);
    if (bfqq) {
    bfq_log_bfqq(bfqq.bfqd, bfqq,
    "calc_finish: serv %lu, w %d",
    service, entity.weight);
    bfq_log_bfqq(bfqq.bfqd, bfqq,
    "calc_finish: start %llu, finish %llu, delta %llu",
    entity.start, entity.finish,
    bfq_delta(service, entity.weight));
    }
    }
//
// bfq_entity_of - get an entity from a node.
// @node: the node field of the entity.
//
// Convert a node pointer to the relative entity.  This is used only
// to simplify the logic of some functions and not as the generic
// conversion mechanism because, e.g., in the tree walking functions,
// the check for a %NULL value would be redundant.
//
#[no_mangle]
pub unsafe extern "C" fn bfq_entity_of(node: *mut rb_node) -> *mut c_void {
    let mut entity = core::ptr::null_mut();
    if (node) {
    entity = rb_entry(node, bfq_entity, rb_node);
    }
    return entity;
    }
//
// bfq_extract - remove an entity from a tree.
// @root: the tree root.
// @entity: the entity to remove.
//
#[no_mangle]
unsafe extern "C" fn bfq_extract(root: *mut rb_root, entity: *mut bfq_entity) {
    entity.tree = core::ptr::null_mut();
    rb_erase(&entity.rb_node, root);
    }
//
// bfq_idle_extract - extract an entity from the idle tree.
// @st: the service tree of the owning @entity.
// @entity: the entity being removed.
//
#[no_mangle]
pub unsafe extern "C" fn bfq_idle_extract(st: *mut bfq_service_tree, entity: *mut bfq_entity) {
    let mut bfqq = bfq_entity_to_bfqq(entity);
pub static mut next: *mut c_void = core::ptr::null_mut();
    if (entity == st.first_idle) {
    next = rb_next(&entity.rb_node);
    st.first_idle = bfq_entity_of(next);
    }
    if (entity == st.last_idle) {
    next = rb_prev(&entity.rb_node);
    st.last_idle = bfq_entity_of(next);
    }
    bfq_extract(&st.idle, entity);
    if (bfqq) {
    list_del(&bfqq.bfqq_list);
    }
    }
//
// bfq_insert - generic tree insertion.
// @root: tree root.
// @entity: entity to insert.
//
// This is used for the idle and the active tree, since they are both
// ordered by finish time.
//
#[no_mangle]
unsafe extern "C" fn bfq_insert(root: *mut rb_root, entity: *mut bfq_entity) {
pub static mut entry: *mut c_void = core::ptr::null_mut();
    let mut node = &root.rb_node;
    let mut parent = core::ptr::null_mut();
    while (*node) {
    parent = *node;
    entry = rb_entry(parent, bfq_entity, rb_node);
    if (bfq_gt(entry.finish, entity.finish)) {
    node = &parent.rb_left;
    }
    else {
    node = &parent.rb_right;
    }
    }
    rb_link_node(&entity.rb_node, parent, node);
    rb_insert_color(&entity.rb_node, root);
    entity.tree = root;
    }
//
// bfq_update_min - update the min_start field of a entity.
// @entity: the entity to update.
// @node: one of its children.
//
// This function is called when @entity may store an invalid value for
// min_start due to updates to the active tree.  The function  assumes
// that the subtree rooted at @node (which may be its left or its right
// child) has a valid min_start value.
//
#[no_mangle]
unsafe extern "C" fn bfq_update_min(entity: *mut bfq_entity, node: *mut rb_node) {
pub static mut child: *mut c_void = core::ptr::null_mut();
    if (node) {
    child = rb_entry(node, bfq_entity, rb_node);
    if (bfq_gt(entity.min_start, child.min_start)) {
    entity.min_start = child.min_start;
    }
    }
    }
//
// bfq_update_active_node - recalculate min_start.
// @node: the node to update.
//
// @node may have changed position or one of its children may have moved,
// this function updates its min_start value.  The left and right subtrees
// are assumed to hold a correct min_start value.
//
#[no_mangle]
unsafe extern "C" fn bfq_update_active_node(node: *mut rb_node) {
    let mut entity = rb_entry(node, bfq_entity, rb_node);
    entity.min_start = entity.start;
    bfq_update_min(entity, node.rb_right);
    bfq_update_min(entity, node.rb_left);
    }
//
// bfq_update_active_tree - update min_start for the whole active tree.
// @node: the starting node.
//
// @node must be the deepest modified node after an update.  This function
// updates its min_start using the values held by its children, assuming
// that they did not change, and then updates all the nodes that may have
// changed in the path to the root.  The only nodes that may have changed
// are the ones in the path or their siblings.
//
#[no_mangle]
unsafe extern "C" fn bfq_update_active_tree(node: *mut rb_node) {
pub static mut parent: *mut c_void = core::ptr::null_mut();
// label;
    bfq_update_active_node(node);
    parent = rb_parent(node);
    if (!parent) {
    return;
    }
    if (node == parent.rb_left && parent.rb_right) {
    bfq_update_active_node(parent.rb_right);
    }

    else if (parent.rb_left) {
    bfq_update_active_node(parent.rb_left);
    }
    node = parent;
// goto;
    }
//
// bfq_active_insert - insert an entity in the active tree of its
// group/device.
// @st: the service tree of the entity.
// @entity: the entity being inserted.
//
// The active tree is ordered by finish time, but an extra key is kept
// per each node, containing the minimum value for the start times of
// its children (and the node itself), so it's possible to search for
// the eligible node with the lowest finish time in logarithmic time.
//
#[no_mangle]
pub unsafe extern "C" fn bfq_active_insert(st: *mut bfq_service_tree, entity: *mut bfq_entity) {
    let mut bfqq = bfq_entity_to_bfqq(entity);
    let mut node = &entity.rb_node;
    bfq_insert(&st.active, entity);
    if (node.rb_left) {
    node = node.rb_left;
    }

    else if (node.rb_right) {
    node = node.rb_right;
    }
    bfq_update_active_tree(node);
    if (bfqq) {
    list_add(&bfqq.bfqq_list, &bfqq.bfqd.active_list[bfqq.actuator_idx]);
    }
    bfq_inc_active_entities(entity);
    }
//
// bfq_ioprio_to_weight - calc a weight from an ioprio.
// @ioprio: the ioprio value to convert.
//
#[no_mangle]
pub unsafe extern "C" fn bfq_ioprio_to_weight(ioprio: c_int) -> c_ushort {
    return (IOPRIO_NR_LEVELS - ioprio) * BFQ_WEIGHT_CONVERSION_COEFF;
    }
//
// bfq_weight_to_ioprio - calc an ioprio from a weight.
// @weight: the weight value to convert.
//
// To preserve as much as possible the old only-ioprio user interface,
// 0 is used as an escape ioprio value for weights (numerically) equal or
// larger than IOPRIO_NR_LEVELS * BFQ_WEIGHT_CONVERSION_COEFF.
//
#[no_mangle]
unsafe extern "C" fn bfq_weight_to_ioprio(weight: c_int) -> c_ushort {
    return max_t(int, 0,
    IOPRIO_NR_LEVELS - weight / BFQ_WEIGHT_CONVERSION_COEFF);
    }
#[no_mangle]
unsafe extern "C" fn bfq_get_entity(entity: *mut bfq_entity) {
    let mut bfqq = bfq_entity_to_bfqq(entity);
    if (bfqq) {
    bfqq.ref += 1;
    bfq_log_bfqq(bfqq.bfqd, bfqq, "get_entity: %p %d",
    bfqq, bfqq.ref);
    }
    }
//
// bfq_find_deepest - find the deepest node that an extraction can modify.
// @node: the node being removed.
//
// Do the first step of an extraction in an rb tree, looking for the
// node that will replace @node, and returning the deepest node that
// the following modifications to the tree can touch.  If @node is the
// last node in the tree return %NULL.
//
#[no_mangle]
pub unsafe extern "C" fn bfq_find_deepest(node: *mut rb_node) -> *mut c_void {
pub static mut deepest: *mut c_void = core::ptr::null_mut();
    if (!node.rb_right && !node.rb_left) {
    deepest = rb_parent(node);
    }

    else if (!node.rb_right) {
    deepest = node.rb_left;
    }

    else if (!node.rb_left) {
    deepest = node.rb_right;
    }
    else {
    deepest = rb_next(node);
    if (deepest.rb_right) {
    deepest = deepest.rb_right;
    }

    else if (rb_parent(deepest) != node) {
    deepest = rb_parent(deepest);
    }
    }
    return deepest;
    }
//
// bfq_active_extract - remove an entity from the active tree.
// @st: the service_tree containing the tree.
// @entity: the entity being removed.
//
#[no_mangle]
pub unsafe extern "C" fn bfq_active_extract(st: *mut bfq_service_tree, entity: *mut bfq_entity) {
    let mut bfqq = bfq_entity_to_bfqq(entity);
pub static mut node: *mut c_void = core::ptr::null_mut();
    node = bfq_find_deepest(&entity.rb_node);
    bfq_extract(&st.active, entity);
    if (node) {
    bfq_update_active_tree(node);
    }
    if (bfqq) {
    list_del(&bfqq.bfqq_list);
    }
    bfq_dec_active_entities(entity);
    }
//
// bfq_idle_insert - insert an entity into the idle tree.
// @st: the service tree containing the tree.
// @entity: the entity to insert.
//
#[no_mangle]
pub unsafe extern "C" fn bfq_idle_insert(st: *mut bfq_service_tree, entity: *mut bfq_entity) {
    let mut bfqq = bfq_entity_to_bfqq(entity);
    let mut first_idle = st.first_idle;
    let mut last_idle = st.last_idle;
    if (!first_idle || bfq_gt(first_idle.finish, entity.finish)) {
    st.first_idle = entity;
    }
    if (!last_idle || bfq_gt(entity.finish, last_idle.finish)) {
    st.last_idle = entity;
    }
    bfq_insert(&st.idle, entity);
    if (bfqq) {
    list_add(&bfqq.bfqq_list, &bfqq.bfqd.idle_list);
    }
    }
//
// bfq_forget_entity - do not consider entity any longer for scheduling
// @st: the service tree.
// @entity: the entity being removed.
// @is_in_service: true if entity is currently the in-service entity.
//
// Forget everything about @entity. In addition, if entity represents
// a queue, and the latter is not in service, then release the service
// reference to the queue (the one taken through bfq_get_entity). In
// fact, in this case, there is really no more service reference to
// the queue, as the latter is also outside any service tree. If,
// instead, the queue is in service, then __bfq_bfqd_reset_in_service
// will take care of putting the reference when the queue finally
// stops being served.
//
#[no_mangle]
pub unsafe extern "C" fn bfq_forget_entity(st: *mut bfq_service_tree, entity: *mut bfq_entity, is_in_service: bool) {
    let mut bfqq = bfq_entity_to_bfqq(entity);
    entity.on_st_or_in_serv = false;
    st.wsum -= entity.weight;
    if (bfqq && !is_in_service) {
    bfq_put_queue(bfqq);
    }
    }
//
// bfq_put_idle_entity - release the idle tree ref of an entity.
// @st: service tree for the entity.
// @entity: the entity being released.
//
#[no_mangle]
pub unsafe extern "C" fn bfq_put_idle_entity(st: *mut bfq_service_tree, entity: *mut bfq_entity) {
    bfq_idle_extract(st, entity);
    bfq_forget_entity(st, entity,
    entity == entity.sched_data.in_service_entity);
    }
//
// bfq_forget_idle - update the idle tree if necessary.
// @st: the service tree to act upon.
//
// To preserve the global O(log N) complexity we only remove one entry here;
// as the idle tree will not grow indefinitely this can be done safely.
//
#[no_mangle]
unsafe extern "C" fn bfq_forget_idle(st: *mut bfq_service_tree) {
    let mut first_idle = st.first_idle;
    let mut last_idle = st.last_idle;
    if (RB_EMPTY_ROOT(&st.active) && last_idle &&
    !bfq_gt(last_idle.finish, st.vtime)) {
//
// Forget the whole idle tree, increasing the vtime past
// the last finish time of idle entities.
//
    st.vtime = last_idle.finish;
    }
    if (first_idle && !bfq_gt(first_idle.finish, st.vtime)) {
    bfq_put_idle_entity(st, first_idle);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bfq_entity_service_tree(entity: *mut bfq_entity) -> *mut c_void {
    let mut sched_data = entity.sched_data;
pub static mut idx: c_uint = 0;
    return sched_data.service_tree + idx;
    }
//
// Update weight and priority of entity. If update_class_too is true,
// then update the ioprio_class of entity too.
//
// The reason why the update of ioprio_class is controlled through the
// last parameter is as follows. Changing the ioprio class of an
// entity implies changing the destination service trees for that
// entity. If such a change occurred when the entity is already on one
// of the service trees for its previous class, then the state of the
// entity would become more complex: none of the new possible service
// trees for the entity, according to bfq_entity_service_tree(), would
// match any of the possible service trees on which the entity
// is. Complex operations involving these trees, such as entity
// activations and deactivations, should take into account this
// additional complexity.  To avoid this issue, this function is
// invoked with update_class_too unset in the points in the code where
// entity may happen to be on some tree.
//
#[no_mangle]
pub unsafe extern "C" fn __bfq_entity_update_weight_prio(old_st: *mut bfq_service_tree, entity: *mut bfq_entity, update_class_too: bool) -> *mut c_void {
    let mut new_st = old_st;
    if (entity.prio_changed) {
    let mut bfqq = bfq_entity_to_bfqq(entity);
    let mut prev_weight = 0;
    let mut new_weight = 0;
// Matches the smp_wmb() in bfq_group_set_weight.
    smp_rmb();
    old_st.wsum -= entity.weight;
    if (entity.new_weight != entity.orig_weight) {
    if (entity.new_weight < BFQ_MIN_WEIGHT ||
    entity.new_weight > BFQ_MAX_WEIGHT) {
    pr_crit("update_weight_prio: new_weight %d\n",
    entity.new_weight);
    if (entity.new_weight < BFQ_MIN_WEIGHT) {
    entity.new_weight = BFQ_MIN_WEIGHT;
    }
    else {
    entity.new_weight = BFQ_MAX_WEIGHT;
    }
    }
    entity.orig_weight = entity.new_weight;
    if (bfqq) {
    bfqq.ioprio =
    bfq_weight_to_ioprio(entity.orig_weight);
    }
    }
    if (bfqq && update_class_too) {
    bfqq.ioprio_class = bfqq.new_ioprio_class;
    }
//
// Reset prio_changed only if the ioprio_class change
// is not pending any longer.
//
    if (!bfqq || bfqq.ioprio_class == bfqq.new_ioprio_class) {
    entity.prio_changed = 0;
    }
//
// NOTE: here we may be changing the weight too early,
// this will cause unfairness.  The correct approach
// would have required additional complexity to defer
// weight changes to the proper time instants (i.e.,
// when entity->finish <= old_st->vtime).
//
    new_st = bfq_entity_service_tree(entity);
    prev_weight = entity.weight;
    new_weight = entity.orig_weight *
    (bfqq ? bfqq.wr_coeff : 1);
//
// If the weight of the entity changes, and the entity is a
// queue, remove the entity from its old weight counter (if
// there is a counter associated with the entity).
//
    if (prev_weight != new_weight && bfqq) {
    bfq_weights_tree_remove(bfqq);
    }
    entity.weight = new_weight;
//
// Add the entity, if it is not a weight-raised queue,
// to the counter associated with its new weight.
//
    if (prev_weight != new_weight && bfqq && bfqq.wr_coeff == 1) {
    bfq_weights_tree_add(bfqq);
    }
    new_st.wsum += entity.weight;
    if (new_st != old_st) {
    entity.start = new_st.vtime;
    }
    }
    return new_st;
    }
//
// bfq_bfqq_served - update the scheduler status after selection for
// service.
// @bfqq: the queue being served.
// @served: bytes to transfer.
//
// NOTE: this can be optimized, as the timestamps of upper level entities
// are synchronized every time a new bfqq is selected for service.  By now,
// we keep it to better check consistency.
//
#[no_mangle]
pub unsafe extern "C" fn bfq_bfqq_served(bfqq: *mut bfq_queue, served: c_int) {
    let mut entity = &bfqq.entity;
pub static mut st: *mut c_void = core::ptr::null_mut();
    if (!bfqq.service_from_backlogged) {
    bfqq.first_IO_time = jiffies;
    }
    if (bfqq.wr_coeff > 1) {
    bfqq.service_from_wr += served;
    }
    bfqq.service_from_backlogged += served;
    for_each_entity(entity) {
    st = bfq_entity_service_tree(entity);
    entity.service += served;
    st.vtime += bfq_delta(served, st.wsum);
    bfq_forget_idle(st);
    }
    bfq_log_bfqq(bfqq.bfqd, bfqq, "bfqq_served %d secs", served);
    }
//
// bfq_bfqq_charge_time - charge an amount of service equivalent to the length
// of the time interval during which bfqq has been in
// service.
// @bfqd: the device
// @bfqq: the queue that needs a service update.
// @time_ms: the amount of time during which the queue has received service
//
// If a queue does not consume its budget fast enough, then providing
// the queue with service fairness may impair throughput, more or less
// severely. For this reason, queues that consume their budget slowly
// are provided with time fairness instead of service fairness. This
// goal is achieved through the BFQ scheduling engine, even if such an
// engine works in the service, and not in the time domain. The trick
// is charging these queues with an inflated amount of service, equal
// to the amount of service that they would have received during their
// service slot if they had been fast, i.e., if their requests had
// been dispatched at a rate equal to the estimated peak rate.
//
// It is worth noting that time fairness can cause important
// distortions in terms of bandwidth distribution, on devices with
// internal queueing. The reason is that I/O requests dispatched
// during the service slot of a queue may be served after that service
// slot is finished, and may have a total processing time loosely
// correlated with the duration of the service slot. This is
// especially true for short service slots.
//
#[no_mangle]
pub unsafe extern "C" fn bfq_bfqq_charge_time(bfqd: *mut bfq_data, bfqq: *mut bfq_queue, time_ms: c_ulong) {
    let mut entity = &bfqq.entity;
pub static mut timeout_ms: c_ulong = 0;
pub static mut bounded_time_ms: c_ulong = 0;
    let mut serv_to_charge_for_time = (bfqd.bfq_max_budget * bounded_time_ms) / timeout_ms;
pub static mut tot_serv_to_charge: c_int = 0;
// Increase budget to avoid inconsistencies
    if (tot_serv_to_charge > entity.budget) {
    entity.budget = tot_serv_to_charge;
    }
    bfq_bfqq_served(bfqq,
    max_t(int, 0, tot_serv_to_charge - entity.service));
    }
#[no_mangle]
pub unsafe extern "C" fn bfq_update_fin_time_enqueue(entity: *mut bfq_entity, st: *mut bfq_service_tree, backshifted: bool) {
    let mut bfqq = bfq_entity_to_bfqq(entity);
//
// When this function is invoked, entity is not in any service
// tree, then it is safe to invoke next function with the last
// parameter set (see the comments on the function).
//
    st = __bfq_entity_update_weight_prio(st, entity, true);
    bfq_calc_finish(entity, entity.budget);
//
// If some queues enjoy backshifting for a while, then their
// (virtual) finish timestamps may happen to become lower and
// lower than the system virtual time.	In particular, if
// these queues often happen to be idle for short time
// periods, and during such time periods other queues with
// higher timestamps happen to be busy, then the backshifted
// timestamps of the former queues can become much lower than
// the system virtual time. In fact, to serve the queues with
// higher timestamps while the ones with lower timestamps are
// idle, the system virtual time may be pushed-up to much
// higher values than the finish timestamps of the idle
// queues. As a consequence, the finish timestamps of all new
// or newly activated queues may end up being much larger than
// those of lucky queues with backshifted timestamps. The
// latter queues may then monopolize the device for a lot of
// time. This would simply break service guarantees.
//
// To reduce this problem, push up a little bit the
// backshifted timestamps of the queue associated with this
// entity (only a queue can happen to have the backshifted
// flag set): just enough to let the finish timestamp of the
// queue be equal to the current value of the system virtual
// time. This may introduce a little unfairness among queues
// with backshifted timestamps, but it does not break
// worst-case fairness guarantees.
//
// As a special case, if bfqq is weight-raised, push up
// timestamps much less, to keep very low the probability that
// this push up causes the backshifted finish timestamps of
// weight-raised queues to become higher than the backshifted
// finish timestamps of non weight-raised queues.
//
    if (backshifted && bfq_gt(st.vtime, entity.finish)) {
pub static mut delta: c_ulong = 0;
    if (bfqq) {
    delta /= bfqq.wr_coeff;
    }
    entity.start += delta;
    entity.finish += delta;
    }
    bfq_active_insert(st, entity);
    }
//
// __bfq_activate_entity - handle activation of entity.
// @entity: the entity being activated.
// @non_blocking_wait_rq: true if entity was waiting for a request
//
// Called for a 'true' activation, i.e., if entity is not active and
// one of its children receives a new request.
//
// Basically, this function updates the timestamps of entity and
// inserts entity into its active tree, after possibly extracting it
// from its idle tree.
//
#[no_mangle]
pub unsafe extern "C" fn __bfq_activate_entity(entity: *mut bfq_entity, non_blocking_wait_rq: bool) {
    let mut st = bfq_entity_service_tree(entity);
pub static mut backshifted: bool = false;
    unsigned long long min_vstart;
// See comments on bfq_fqq_update_budg_for_activation
    if (non_blocking_wait_rq && bfq_gt(st.vtime, entity.finish)) {
    backshifted = true;
    min_vstart = entity.finish;
    } else {
    min_vstart = st.vtime;
    }
    if (entity.tree == &st.idle) {
//
// Must be on the idle tree, bfq_idle_extract() will
// check for that.
//
    bfq_idle_extract(st, entity);
    entity.start = bfq_gt(min_vstart, entity.finish) ?
    min_vstart : entity.finish;
    } else {
//
// The finish time of the entity may be invalid, and
// it is in the past for sure, otherwise the queue
// would have been on the idle tree.
//
    entity.start = min_vstart;
    st.wsum += entity.weight;
//
// entity is about to be inserted into a service tree,
// and then set in service: get a reference to make
// sure entity does not disappear until it is no
// longer in service or scheduled for service.
//
    bfq_get_entity(entity);
    entity.on_st_or_in_serv = true;
    }
    bfq_update_fin_time_enqueue(entity, st, backshifted);
    }
//
// __bfq_requeue_entity - handle requeueing or repositioning of an entity.
// @entity: the entity being requeued or repositioned.
//
// Requeueing is needed if this entity stops being served, which
// happens if a leaf descendant entity has expired. On the other hand,
// repositioning is needed if the next_inservice_entity for the child
// entity has changed. See the comments inside the function for
// details.
//
// Basically, this function: 1) removes entity from its active tree if
// present there, 2) updates the timestamps of entity and 3) inserts
// entity back into its active tree (in the new, right position for
// the new values of the timestamps).
//
#[no_mangle]
unsafe extern "C" fn __bfq_requeue_entity(entity: *mut bfq_entity) {
    let mut sd = entity.sched_data;
    let mut st = bfq_entity_service_tree(entity);
    if (entity == sd.in_service_entity) {
//
// We are requeueing the current in-service entity,
// which may have to be done for one of the following
// reasons:
// - entity represents the in-service queue, and the
// in-service queue is being requeued after an
// expiration;
// - entity represents a group, and its budget has
// changed because one of its child entities has
// just been either activated or requeued for some
// reason; the timestamps of the entity need then to
// be updated, and the entity needs to be enqueued
// or repositioned accordingly.
//
// In particular, before requeueing, the start time of
// the entity must be moved forward to account for the
// service that the entity has received while in
// service. This is done by the next instructions. The
// finish time will then be updated according to this
// new value of the start time, and to the budget of
// the entity.
//
    bfq_calc_finish(entity, entity.service);
    entity.start = entity.finish;
//
// In addition, if the entity had more than one child
// when set in service, then it was not extracted from
// the active tree. This implies that the position of
// the entity in the active tree may need to be
// changed now, because we have just updated the start
// time of the entity, and we will update its finish
// time in a moment (the requeueing is then, more
// precisely, a repositioning in this case). To
// implement this repositioning, we: 1) dequeue the
// entity here, 2) update the finish time and requeue
// the entity according to the new timestamps below.
//
    if (entity.tree) {
    bfq_active_extract(st, entity);
    }
    } else { /* The entity is already active, and not in service */
//
// In this case, this function gets called only if the
// next_in_service entity below this entity has
// changed, and this change has caused the budget of
// this entity to change, which, finally implies that
// the finish time of this entity must be
// updated. Such an update may cause the scheduling,
// i.e., the position in the active tree, of this
// entity to change. We handle this change by: 1)
// dequeueing the entity here, 2) updating the finish
// time and requeueing the entity according to the new
// timestamps below. This is the same approach as the
// non-extracted-entity sub-case above.
//
    bfq_active_extract(st, entity);
    }
    bfq_update_fin_time_enqueue(entity, st, false);
    }
#[no_mangle]
pub unsafe extern "C" fn __bfq_activate_requeue_entity(entity: *mut bfq_entity, non_blocking_wait_rq: bool) {
    let mut st = bfq_entity_service_tree(entity);
    if (entity.sched_data.in_service_entity == entity ||
    entity.tree == &st.active) {
//
// in service or already queued on the active tree,
// requeue or reposition
//
    __bfq_requeue_entity(entity);
    }
    else {
//
// Not in service and not queued on its active tree:
// the activity is idle and this is a true activation.
//
    __bfq_activate_entity(entity, non_blocking_wait_rq);
    }
    }
//
// bfq_activate_requeue_entity - activate or requeue an entity representing a
// bfq_queue, and activate, requeue or reposition
// all ancestors for which such an update becomes
// necessary.
// @entity: the entity to activate.
// @non_blocking_wait_rq: true if this entity was waiting for a request
// @requeue: true if this is a requeue, which implies that bfqq is
// being expired; thus ALL its ancestors stop being served and must
// therefore be requeued
// @expiration: true if this function is being invoked in the expiration path
// of the in-service queue
//
#[no_mangle]
pub unsafe extern "C" fn bfq_activate_requeue_entity(entity: *mut bfq_entity, non_blocking_wait_rq: bool, requeue: bool, expiration: bool) {
    for_each_entity(entity) {
    __bfq_activate_requeue_entity(entity, non_blocking_wait_rq);
    if (!bfq_update_next_in_service(entity.sched_data, entity,
    expiration) && !requeue) {
    break;
    }
    }
    }
//
// __bfq_deactivate_entity - update sched_data and service trees for
// entity, so as to represent entity as inactive
// @entity: the entity being deactivated.
// @ins_into_idle_tree: if false, the entity will not be put into the
// idle tree.
//
// If necessary and allowed, puts entity into the idle tree. NOTE:
// entity may be on no tree if in service.
//
#[no_mangle]
pub unsafe extern "C" fn __bfq_deactivate_entity(entity: *mut bfq_entity, ins_into_idle_tree: bool) -> bool {
    let mut sd = entity.sched_data;
pub static mut st: *mut c_void = core::ptr::null_mut();
    let mut is_in_service = 0;
    if (!entity.on_st_or_in_serv) // {
// entity never activated, or
// already inactive
//
    return false;
    }
//
// If we get here, then entity is active, which implies that
// bfq_group_set_parent has already been invoked for the group
// represented by entity. Therefore, the field
// entity->sched_data has been set, and we can safely use it.
//
    st = bfq_entity_service_tree(entity);
    is_in_service = entity == sd.in_service_entity;
    bfq_calc_finish(entity, entity.service);
    if (is_in_service) {
    sd.in_service_entity = core::ptr::null_mut();
    }
    else {
//
// Non in-service entity: nobody will take care of
// resetting its service counter on expiration. Do it
// now.
//
    entity.service = 0;
    }
    if (entity.tree == &st.active) {
    bfq_active_extract(st, entity);
    }

    else if (!is_in_service && entity.tree == &st.idle) {
    bfq_idle_extract(st, entity);
    }
    if (!ins_into_idle_tree || !bfq_gt(entity.finish, st.vtime)) {
    bfq_forget_entity(st, entity, is_in_service);
    }
    else {
    bfq_idle_insert(st, entity);
    }
    return true;
    }
//
// bfq_deactivate_entity - deactivate an entity representing a bfq_queue.
// @entity: the entity to deactivate.
// @ins_into_idle_tree: true if the entity can be put into the idle tree
// @expiration: true if this function is being invoked in the expiration path
// of the in-service queue
//
#[no_mangle]
pub unsafe extern "C" fn bfq_deactivate_entity(entity: *mut bfq_entity, ins_into_idle_tree: bool, expiration: bool) {
pub static mut sd: *mut c_void = core::ptr::null_mut();
    let mut parent = core::ptr::null_mut();
    for_each_entity_safe(entity, parent) {
    sd = entity.sched_data;
    if (!__bfq_deactivate_entity(entity, ins_into_idle_tree)) {
//
// entity is not in any tree any more, so
// this deactivation is a no-op, and there is
// nothing to change for upper-level entities
// (in case of expiration, this can never
// happen).
//
    return;
    }
    if (sd.next_in_service == entity) {
//
// entity was the next_in_service entity,
// then, since entity has just been
// deactivated, a new one must be found.
//
    bfq_update_next_in_service(sd, core::ptr::null_mut(), expiration);
    }
    if (sd.next_in_service || sd.in_service_entity) {
//
// The parent entity is still active, because
// either next_in_service or in_service_entity
// is not NULL. So, no further upwards
// deactivation must be performed.  Yet,
// next_in_service has changed.	Then the
// schedule does need to be updated upwards.
//
// NOTE If in_service_entity is not NULL, then
// next_in_service may happen to be NULL,
// although the parent entity is evidently
// active. This happens if 1) the entity
// pointed by in_service_entity is the only
// active entity in the parent entity, and 2)
// according to the definition of
// next_in_service, the in_service_entity
// cannot be considered as
// next_in_service. See the comments on the
// definition of next_in_service for details.
//
    break;
    }
//
// If we get here, then the parent is no more
// backlogged and we need to propagate the
// deactivation upwards. Thus let the loop go on.
//
// Also let parent be queued into the idle tree on
// deactivation, to preserve service guarantees, and
// assuming that who invoked this function does not
// need parent entities too to be removed completely.
//
    ins_into_idle_tree = true;
    }
//
// If the deactivation loop is fully executed, then there are
// no more entities to touch and next loop is not executed at
// all. Otherwise, requeue remaining entities if they are
// about to stop receiving service, or reposition them if this
// is not the case.
//
    entity = parent;
    for_each_entity(entity) {
//
// Invoke __bfq_requeue_entity on entity, even if
// already active, to requeue/reposition it in the
// active tree (because sd->next_in_service has
// changed)
//
    __bfq_requeue_entity(entity);
    sd = entity.sched_data;
    if (!bfq_update_next_in_service(sd, entity, expiration) &&
    !expiration) {
//
// next_in_service unchanged or not causing
// any change in entity->parent->sd, and no
// requeueing needed for expiration: stop
// here.
//
    break;
    }
    }
    }
//
// bfq_calc_vtime_jump - compute the value to which the vtime should jump,
// if needed, to have at least one entity eligible.
// @st: the service tree to act upon.
//
// Assumes that st is not empty.
//
#[no_mangle]
unsafe extern "C" fn bfq_calc_vtime_jump(st: *mut bfq_service_tree) -> u64 {
    let mut root_entity = bfq_root_active_entity(&st.active);
    if (bfq_gt(root_entity.min_start, st.vtime)) {
    return root_entity.min_start;
    }
    return st.vtime;
    }
#[no_mangle]
unsafe extern "C" fn bfq_update_vtime(st: *mut bfq_service_tree, new_value: u64) {
    if (new_value > st.vtime) {
    st.vtime = new_value;
    bfq_forget_idle(st);
    }
    }
//
// bfq_first_active_entity - find the eligible entity with
// the smallest finish time
// @st: the service tree to select from.
// @vtime: the system virtual to use as a reference for eligibility
//
// This function searches the first schedulable entity, starting from the
// root of the tree and going on the left every time on this side there is
// a subtree with at least one eligible (start <= vtime) entity. The path on
// the right is followed only if a) the left subtree contains no eligible
// entities and b) no eligible entity has been found yet.
//
#[no_mangle]
pub unsafe extern "C" fn bfq_first_active_entity(st: *mut bfq_service_tree, vtime: u64) -> *mut c_void {
    struct bfq_entity *entry, *first = core::ptr::null_mut();
    let mut node = st.active.rb_node;
    while (node) {
    entry = rb_entry(node, bfq_entity, rb_node);
// label;
    if (!bfq_gt(entry.start, vtime)) {
    first = entry;
    }
    if (node.rb_left) {
    entry = rb_entry(node.rb_left, bfq_entity, rb_node);
    if (!bfq_gt(entry.min_start, vtime)) {
    node = node.rb_left;
// goto;
    }
    }
    if (first) {
    break;
    }
    node = node.rb_right;
    }
    return first;
    }
//
// __bfq_lookup_next_entity - return the first eligible entity in @st.
// @st: the service tree.
// @in_service: whether or not there is an in-service entity for the sched_data
// this active tree belongs to.
//
// If there is no in-service entity for the sched_data st belongs to,
// then return the entity that will be set in service if:
// 1) the parent entity this st belongs to is set in service;
// 2) no entity belonging to such parent entity undergoes a state change
// that would influence the timestamps of the entity (e.g., becomes idle,
// becomes backlogged, changes its budget, ...).
//
// In this first case, update the virtual time in @st too (see the
// comments on this update inside the function).
//
// In contrast, if there is an in-service entity, then return the
// entity that would be set in service if not only the above
// conditions, but also the next one held true: the currently
// in-service entity, on expiration,
// 1) gets a finish time equal to the current one, or
// 2) is not eligible any more, or
// 3) is idle.
//
#[no_mangle]
pub unsafe extern "C" fn __bfq_lookup_next_entity(st: *mut bfq_service_tree, in_service: bool) -> *mut c_void {
pub static mut entity: *mut c_void = core::ptr::null_mut();
    let mut new_vtime = 0;
    if (RB_EMPTY_ROOT(&st.active)) {
    return core::ptr::null_mut();
    }
//
// Get the value of the system virtual time for which at
// least one entity is eligible.
//
    new_vtime = bfq_calc_vtime_jump(st);
//
// If there is no in-service entity for the sched_data this
// active tree belongs to, then push the system virtual time
// up to the value that guarantees that at least one entity is
// eligible. If, instead, there is an in-service entity, then
// do not make any such update, because there is already an
// eligible entity, namely the in-service one (even if the
// entity is not on st, because it was extracted when set in
// service).
//
    if (!in_service) {
    bfq_update_vtime(st, new_vtime);
    }
    entity = bfq_first_active_entity(st, new_vtime);
    return entity;
    }
//
// bfq_lookup_next_entity - return the first eligible entity in @sd.
// @sd: the sched_data.
// @expiration: true if we are on the expiration path of the in-service queue
//
// This function is invoked when there has been a change in the trees
// for sd, and we need to know what is the new next entity to serve
// after this change.
//
#[no_mangle]
pub unsafe extern "C" fn bfq_lookup_next_entity(sd: *mut bfq_sched_data, expiration: bool) -> *mut c_void {
    let mut st = sd.service_tree;
    let mut idle_class_st = st + (BFQ_IOPRIO_CLASSES - 1);
    let mut entity = core::ptr::null_mut();
pub static mut class_idx: c_int = 0;
//
// Choose from idle class, if needed to guarantee a minimum
// bandwidth to this class (and if there is some active entity
// in idle class). This should also mitigate
// priority-inversion problems in case a low priority task is
// holding file system resources.
//
    if (time_is_before_jiffies(sd.bfq_class_idle_last_service +
    BFQ_CL_IDLE_TIMEOUT)) {
    if (!RB_EMPTY_ROOT(&idle_class_st.active)) {
    class_idx = BFQ_IOPRIO_CLASSES - 1;
    }
// About to be served if backlogged, or not yet backlogged
    sd.bfq_class_idle_last_service = jiffies;
    }
//
// Find the next entity to serve for the highest-priority
// class, unless the idle class needs to be served.
//
    while (class_idx < BFQ_IOPRIO_CLASSES) {
//
// If expiration is true, then bfq_lookup_next_entity
// is being invoked as a part of the expiration path
// of the in-service queue. In this case, even if
// sd->in_service_entity is not NULL,
// sd->in_service_entity at this point is actually not
// in service any more, and, if needed, has already
// been properly queued or requeued into the right
// tree. The reason why sd->in_service_entity is still
// not NULL here, even if expiration is true, is that
// sd->in_service_entity is reset as a last step in the
// expiration path. So, if expiration is true, tell
// __bfq_lookup_next_entity that there is no
// sd->in_service_entity.
//
    entity = __bfq_lookup_next_entity(st + class_idx,
    sd.in_service_entity &&
    !expiration);
    if (entity) {
    break;
    }
    }
    return entity;
    }
#[no_mangle]
pub unsafe extern "C" fn next_queue_may_preempt(bfqd: *mut bfq_data) -> bool {
    let mut sd = &bfqd.root_group.sched_data;
    return sd.next_in_service != sd.in_service_entity;
    }
//
// Get next queue for service.
//
#[no_mangle]
pub unsafe extern "C" fn bfq_get_next_queue(bfqd: *mut bfq_data) -> *mut c_void {
    let mut entity = core::ptr::null_mut();
pub static mut sd: *mut c_void = core::ptr::null_mut();
pub static mut bfqq: *mut c_void = core::ptr::null_mut();
    if (bfq_tot_busy_queues(bfqd) == 0) {
    return core::ptr::null_mut();
    }
//
// Traverse the path from the root to the leaf entity to
// serve. Set in service all the entities visited along the
// way.
//
    sd = &bfqd.root_group.sched_data;
    while (sd ) {
//
// WARNING. We are about to set the in-service entity
// to sd->next_in_service, i.e., to the (cached) value
// returned by bfq_lookup_next_entity(sd) the last
// time it was invoked, i.e., the last time when the
// service order in sd changed as a consequence of the
// activation or deactivation of an entity. In this
// respect, if we execute bfq_lookup_next_entity(sd)
// in this very moment, it may, although with low
// probability, yield a different entity than that
// pointed to by sd->next_in_service. This rare event
// happens in case there was no CLASS_IDLE entity to
// serve for sd when bfq_lookup_next_entity(sd) was
// invoked for the last time, while there is now one
// such entity.
//
// If the above event happens, then the scheduling of
// such entity in CLASS_IDLE is postponed until the
// service of the sd->next_in_service entity
// finishes. In fact, when the latter is expired,
// bfq_lookup_next_entity(sd) gets called again,
// exactly to update sd->next_in_service.
//
// Make next_in_service entity become in_service_entity
    entity = sd.next_in_service;
    sd.in_service_entity = entity;
//
// If entity is no longer a candidate for next
// service, then it must be extracted from its active
// tree, so as to make sure that it won't be
// considered when computing next_in_service. See the
// comments on the function
// bfq_no_longer_next_in_service() for details.
//
    if (bfq_no_longer_next_in_service(entity)) {
    bfq_active_extract(bfq_entity_service_tree(entity),
    entity);
    }
//
// Even if entity is not to be extracted according to
// the above check, a descendant entity may get
// extracted in one of the next iterations of this
// loop. Such an event could cause a change in
// next_in_service for the level of the descendant
// entity, and thus possibly back to this level.
//
// However, we cannot perform the resulting needed
// update of next_in_service for this level before the
// end of the whole loop, because, to know which is
// the correct next-to-serve candidate entity for each
// level, we need first to find the leaf entity to set
// in service. In fact, only after we know which is
// the next-to-serve leaf entity, we can discover
// whether the parent entity of the leaf entity
// becomes the next-to-serve, and so on.
//
    }
    bfqq = bfq_entity_to_bfqq(entity);
//
// We can finally update all next-to-serve entities along the
// path from the leaf entity just set in service to the root.
//
    for_each_entity(entity) {
    let mut sd = entity.sched_data;
    if (!bfq_update_next_in_service(sd, core::ptr::null_mut(), false)) {
    break;
    }
    }
    return bfqq;
    }
// returns true if the in-service queue gets freed
#[no_mangle]
pub unsafe extern "C" fn __bfq_bfqd_reset_in_service(bfqd: *mut bfq_data) -> bool {
    let mut in_serv_bfqq = bfqd.in_service_queue;
    let mut in_serv_entity = &in_serv_bfqq.entity;
    let mut entity = in_serv_entity;
    bfq_clear_bfqq_wait_request(in_serv_bfqq);
    hrtimer_try_to_cancel(&bfqd.idle_slice_timer);
    bfqd.in_service_queue = core::ptr::null_mut();
//
// When this function is called, all in-service entities have
// been properly deactivated or requeued, so we can safely
// execute the final step: reset in_service_entity along the
// path from entity to the root.
//
    for_each_entity(entity) {
    entity.sched_data.in_service_entity = core::ptr::null_mut();
    }
//
// in_serv_entity is no longer in service, so, if it is in no
// service tree either, then release the service reference to
// the queue it represents (taken with bfq_get_entity).
//
    if (!in_serv_entity.on_st_or_in_serv) {
//
// If no process is referencing in_serv_bfqq any
// longer, then the service reference may be the only
// reference to the queue. If this is the case, then
// bfqq gets freed here.
//
pub static mut ref: c_int = 0;
    bfq_put_queue(in_serv_bfqq);
    if (ref == 1) {
    return true;
    }
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn bfq_deactivate_bfqq(bfqd: *mut bfq_data, bfqq: *mut bfq_queue, ins_into_idle_tree: bool, expiration: bool) {
    let mut entity = &bfqq.entity;
    bfq_deactivate_entity(entity, ins_into_idle_tree, expiration);
    }
#[no_mangle]
pub unsafe extern "C" fn bfq_activate_bfqq(bfqd: *mut bfq_data, bfqq: *mut bfq_queue) {
    let mut entity = &bfqq.entity;
    bfq_activate_requeue_entity(entity, bfq_bfqq_non_blocking_wait_rq(bfqq),
    false, false);
    bfq_clear_bfqq_non_blocking_wait_rq(bfqq);
    }
#[no_mangle]
pub unsafe extern "C" fn bfq_requeue_bfqq(bfqd: *mut bfq_data, bfqq: *mut bfq_queue, expiration: bool) {
    let mut entity = &bfqq.entity;
    bfq_activate_requeue_entity(entity, false,
    bfqq == bfqd.in_service_queue, expiration);
    }
#[no_mangle]
pub unsafe extern "C" fn bfq_add_bfqq_in_groups_with_pending_reqs(bfqq: *mut bfq_queue) {

    let mut entity = &bfqq.entity;
    if (!entity.in_groups_with_pending_reqs) {
    entity.in_groups_with_pending_reqs = true;
    if (!(bfqq_group(bfqq).num_queues_with_pending_reqs++)) {
    bfqq.bfqd.num_groups_with_pending_reqs += 1;
    }
    }

    }
#[no_mangle]
pub unsafe extern "C" fn bfq_del_bfqq_in_groups_with_pending_reqs(bfqq: *mut bfq_queue) {

    let mut entity = &bfqq.entity;
    if (entity.in_groups_with_pending_reqs) {
    entity.in_groups_with_pending_reqs = false;
    if (!(--bfqq_group(bfqq).num_queues_with_pending_reqs)) {
    bfqq.bfqd.num_groups_with_pending_reqs -= 1;
    }
    }

    }
//
// Called when the bfqq no longer has requests pending, remove it from
// the service tree. As a special case, it can be invoked during an
// expiration.
//
#[no_mangle]
pub unsafe extern "C" fn bfq_del_bfqq_busy(bfqq: *mut bfq_queue, expiration: bool) {
    let mut bfqd = bfqq.bfqd;
    bfq_log_bfqq(bfqd, bfqq, "del from busy");
    bfq_clear_bfqq_busy(bfqq);
    bfqd.busy_queues[bfqq.ioprio_class - 1]--;
    if (bfqq.wr_coeff > 1) {
    bfqd.wr_busy_queues -= 1;
    }
    bfqg_stats_update_dequeue(bfqq_group(bfqq));
    bfq_deactivate_bfqq(bfqd, bfqq, true, expiration);
    if (!bfqq.dispatched) {
    bfq_del_bfqq_in_groups_with_pending_reqs(bfqq);
//
// Next function is invoked last, because it causes bfqq to be
// freed. DO NOT use bfqq after the next function invocation.
//
    bfq_weights_tree_remove(bfqq);
    }
    }
//
// Called when an inactive queue receives a new request.
//
#[no_mangle]
pub unsafe extern "C" fn bfq_add_bfqq_busy(bfqq: *mut bfq_queue) {
    let mut bfqd = bfqq.bfqd;
    bfq_log_bfqq(bfqd, bfqq, "add to busy");
    bfq_activate_bfqq(bfqd, bfqq);
    bfq_mark_bfqq_busy(bfqq);
    bfqd.busy_queues[bfqq.ioprio_class - 1]++;
    if (!bfqq.dispatched) {
    bfq_add_bfqq_in_groups_with_pending_reqs(bfqq);
    if (bfqq.wr_coeff == 1) {
    bfq_weights_tree_add(bfqq);
    }
    }
    if (bfqq.wr_coeff > 1) {
    bfqd.wr_busy_queues += 1;
    }
// Move bfqq to the head of the woken list of its waker
    if (!hlist_unhashed(&bfqq.woken_list_node) &&
    &bfqq.woken_list_node != bfqq.waker_bfqq.woken_list.first) {
    hlist_del_init(&bfqq.woken_list_node);
    hlist_add_head(&bfqq.woken_list_node,
    &bfqq.waker_bfqq.woken_list);
    }
    }