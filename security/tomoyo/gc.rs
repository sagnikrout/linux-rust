//! Automatically rewritten from C to Rust
//! Source: security/tomoyo/gc.c
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
// security/tomoyo/gc.c
//
// Copyright (C) 2005-2011  NTT DATA CORPORATION
//

//
// tomoyo_memory_free - Free memory for elements.
//
// @ptr:  Pointer to allocated memory.
//
// Returns nothing.
//
// Caller holds tomoyo_policy_lock mutex.
//
#[no_mangle]
pub unsafe extern "C" fn tomoyo_memory_free(ptr: *mut c_void) {
    static inline void tomoyo_memory_free(void *ptr)
    {
    tomoyo_memory_used[TOMOYO_MEMORY_POLICY] -= ksize(ptr);
    kfree(ptr);
    }
// Lock for protecting tomoyo_io_buffer_list.
    static DEFINE_SPINLOCK(tomoyo_io_buffer_list_lock);
// The list for "struct tomoyo_io_buffer".
    static __guarded_by(&tomoyo_io_buffer_list_lock) LIST_HEAD(tomoyo_io_buffer_list);
//
// tomoyo_struct_used_by_io_buffer - Check whether the list element is used by /sys/kernel/security/tomoyo/ users or not.
//
// @element: Pointer to "struct list_head".
//
// Returns true if @element is used by /sys/kernel/security/tomoyo/ users,
// false otherwise.
//
#[no_mangle]
unsafe extern "C" fn tomoyo_struct_used_by_io_buffer(element: *const list_head) -> bool {
    static bool tomoyo_struct_used_by_io_buffer(const struct list_head *element)
    {
    struct tomoyo_io_buffer *head;
    let mut in_use: bool = false;
    spin_lock(&tomoyo_io_buffer_list_lock);
    list_for_each_entry(head, &tomoyo_io_buffer_list, list) {
    head.users++;
    spin_unlock(&tomoyo_io_buffer_list_lock);
    mutex_lock(&head.io_sem);
    if (head.r.domain == element || head.r.group == element ||
    head.r.acl == element || &head.w.domain.list == element)
    in_use = true;
    mutex_unlock(&head.io_sem);
    spin_lock(&tomoyo_io_buffer_list_lock);
    head.users--;
    if (in_use)
    break;
    }
    spin_unlock(&tomoyo_io_buffer_list_lock);
    return in_use;
    }
//
// tomoyo_name_used_by_io_buffer - Check whether the string is used by /sys/kernel/security/tomoyo/ users or not.
//
// @string: String to check.
//
// Returns true if @string is used by /sys/kernel/security/tomoyo/ users,
// false otherwise.
//
#[no_mangle]
unsafe extern "C" fn tomoyo_name_used_by_io_buffer(string: *const c_char) -> bool {
    static bool tomoyo_name_used_by_io_buffer(const char *string)
    {
    struct tomoyo_io_buffer *head;
    let mut size: usize = strlen(string) + 1;
    let mut in_use: bool = false;
    spin_lock(&tomoyo_io_buffer_list_lock);
    list_for_each_entry(head, &tomoyo_io_buffer_list, list) {
    int i;
    head.users++;
    spin_unlock(&tomoyo_io_buffer_list_lock);
    mutex_lock(&head.io_sem);
    for (i = 0; i < TOMOYO_MAX_IO_READ_QUEUE; i++) {
    const char *w = head.r.w[i];
    if (w < string || w > string + size)
    continue;
    in_use = true;
    break;
    }
    mutex_unlock(&head.io_sem);
    spin_lock(&tomoyo_io_buffer_list_lock);
    head.users--;
    if (in_use)
    break;
    }
    spin_unlock(&tomoyo_io_buffer_list_lock);
    return in_use;
    }
//
// tomoyo_del_transition_control - Delete members in "struct tomoyo_transition_control".
//
// @element: Pointer to "struct list_head".
//
// Returns nothing.
//
#[no_mangle]
pub unsafe extern "C" fn tomoyo_del_transition_control(element: *mut list_head) {
    static inline void tomoyo_del_transition_control(struct list_head *element)
    {
    struct tomoyo_transition_control *ptr =
    container_of(element, typeof(*ptr), head.list);
    tomoyo_put_name(ptr.domainname);
    tomoyo_put_name(ptr.program);
    }
//
// tomoyo_del_aggregator - Delete members in "struct tomoyo_aggregator".
//
// @element: Pointer to "struct list_head".
//
// Returns nothing.
//
#[no_mangle]
pub unsafe extern "C" fn tomoyo_del_aggregator(element: *mut list_head) {
    static inline void tomoyo_del_aggregator(struct list_head *element)
    {
    struct tomoyo_aggregator *ptr =
    container_of(element, typeof(*ptr), head.list);
    tomoyo_put_name(ptr.original_name);
    tomoyo_put_name(ptr.aggregated_name);
    }
//
// tomoyo_del_manager - Delete members in "struct tomoyo_manager".
//
// @element: Pointer to "struct list_head".
//
// Returns nothing.
//
#[no_mangle]
pub unsafe extern "C" fn tomoyo_del_manager(element: *mut list_head) {
    static inline void tomoyo_del_manager(struct list_head *element)
    {
    struct tomoyo_manager *ptr =
    container_of(element, typeof(*ptr), head.list);
    tomoyo_put_name(ptr.manager);
    }
//
// tomoyo_del_acl - Delete members in "struct tomoyo_acl_info".
//
// @element: Pointer to "struct list_head".
//
// Returns nothing.
//
#[no_mangle]
unsafe extern "C" fn tomoyo_del_acl(element: *mut list_head) {
    static void tomoyo_del_acl(struct list_head *element)
    {
    struct tomoyo_acl_info *acl =
    container_of(element, typeof(*acl), list);
    tomoyo_put_condition(acl.cond);
    switch (acl.type) {
    case TOMOYO_TYPE_PATH_ACL:
    {
    struct tomoyo_path_acl *entry
    = container_of(acl, typeof(*entry), head);
    tomoyo_put_name_union(&entry.name);
    }
    break;
    case TOMOYO_TYPE_PATH2_ACL:
    {
    struct tomoyo_path2_acl *entry
    = container_of(acl, typeof(*entry), head);
    tomoyo_put_name_union(&entry.name1);
    tomoyo_put_name_union(&entry.name2);
    }
    break;
    case TOMOYO_TYPE_PATH_NUMBER_ACL:
    {
    struct tomoyo_path_number_acl *entry
    = container_of(acl, typeof(*entry), head);
    tomoyo_put_name_union(&entry.name);
    tomoyo_put_number_union(&entry.number);
    }
    break;
    case TOMOYO_TYPE_MKDEV_ACL:
    {
    struct tomoyo_mkdev_acl *entry
    = container_of(acl, typeof(*entry), head);
    tomoyo_put_name_union(&entry.name);
    tomoyo_put_number_union(&entry.mode);
    tomoyo_put_number_union(&entry.major);
    tomoyo_put_number_union(&entry.minor);
    }
    break;
    case TOMOYO_TYPE_MOUNT_ACL:
    {
    struct tomoyo_mount_acl *entry
    = container_of(acl, typeof(*entry), head);
    tomoyo_put_name_union(&entry.dev_name);
    tomoyo_put_name_union(&entry.dir_name);
    tomoyo_put_name_union(&entry.fs_type);
    tomoyo_put_number_union(&entry.flags);
    }
    break;
    case TOMOYO_TYPE_ENV_ACL:
    {
    struct tomoyo_env_acl *entry =
    container_of(acl, typeof(*entry), head);
    tomoyo_put_name(entry.env);
    }
    break;
    case TOMOYO_TYPE_INET_ACL:
    {
    struct tomoyo_inet_acl *entry =
    container_of(acl, typeof(*entry), head);
    tomoyo_put_group(entry.address.group);
    tomoyo_put_number_union(&entry.port);
    }
    break;
    case TOMOYO_TYPE_UNIX_ACL:
    {
    struct tomoyo_unix_acl *entry =
    container_of(acl, typeof(*entry), head);
    tomoyo_put_name_union(&entry.name);
    }
    break;
    case TOMOYO_TYPE_MANUAL_TASK_ACL:
    {
    struct tomoyo_task_acl *entry =
    container_of(acl, typeof(*entry), head);
    tomoyo_put_name(entry.domainname);
    }
    break;
    }
    }
//
// tomoyo_del_domain - Delete members in "struct tomoyo_domain_info".
//
// @element: Pointer to "struct list_head".
//
// Returns nothing.
//
// Caller holds tomoyo_policy_lock mutex.
//
#[no_mangle]
pub unsafe extern "C" fn tomoyo_del_domain(element: *mut list_head) {
    static inline void tomoyo_del_domain(struct list_head *element)
    {
    struct tomoyo_domain_info *domain =
    container_of(element, typeof(*domain), list);
    struct tomoyo_acl_info *acl;
    struct tomoyo_acl_info *tmp;
//
// Since this domain is referenced from neither
// "struct tomoyo_io_buffer" nor "struct cred"->security, we can delete
// elements without checking for is_deleted flag.
//
    list_for_each_entry_safe(acl, tmp, &domain.acl_info_list, list) {
    tomoyo_del_acl(&acl.list);
    tomoyo_memory_free(acl);
    }
    tomoyo_put_name(domain.domainname);
    }
//
// tomoyo_del_condition - Delete members in "struct tomoyo_condition".
//
// @element: Pointer to "struct list_head".
//
// Returns nothing.
//
#[no_mangle]
pub unsafe extern "C" fn tomoyo_del_condition(element: *mut list_head) {
    void tomoyo_del_condition(struct list_head *element)
    {
    struct tomoyo_condition *cond = container_of(element, typeof(*cond),
    head.list);
    let mut condc: u16 = cond.condc;
    let mut numbers_count: u16 = cond.numbers_count;
    let mut names_count: u16 = cond.names_count;
    let mut argc: u16 = cond.argc;
    let mut envc: u16 = cond.envc;
    unsigned int i;
    const struct tomoyo_condition_element *condp
    = (const struct tomoyo_condition_element *) (cond + 1);
    struct tomoyo_number_union *numbers_p
    = (struct tomoyo_number_union *) (condp + condc);
    struct tomoyo_name_union *names_p
    = (struct tomoyo_name_union *) (numbers_p + numbers_count);
    const struct tomoyo_argv *argv
    = (const struct tomoyo_argv *) (names_p + names_count);
    const struct tomoyo_envp *envp
    = (const struct tomoyo_envp *) (argv + argc);
    for (i = 0; i < numbers_count; i++)
    tomoyo_put_number_union(numbers_p++);
    for (i = 0; i < names_count; i++)
    tomoyo_put_name_union(names_p++);
    for (i = 0; i < argc; argv++, i++)
    tomoyo_put_name(argv.value);
    for (i = 0; i < envc; envp++, i++) {
    tomoyo_put_name(envp.name);
    tomoyo_put_name(envp.value);
    }
    }
//
// tomoyo_del_name - Delete members in "struct tomoyo_name".
//
// @element: Pointer to "struct list_head".
//
// Returns nothing.
//
#[no_mangle]
pub unsafe extern "C" fn tomoyo_del_name(element: *mut list_head) {
    static inline void tomoyo_del_name(struct list_head *element)
    {
// Nothing to do.
    }
//
// tomoyo_del_path_group - Delete members in "struct tomoyo_path_group".
//
// @element: Pointer to "struct list_head".
//
// Returns nothing.
//
#[no_mangle]
pub unsafe extern "C" fn tomoyo_del_path_group(element: *mut list_head) {
    static inline void tomoyo_del_path_group(struct list_head *element)
    {
    struct tomoyo_path_group *member =
    container_of(element, typeof(*member), head.list);
    tomoyo_put_name(member.member_name);
    }
//
// tomoyo_del_group - Delete "struct tomoyo_group".
//
// @element: Pointer to "struct list_head".
//
// Returns nothing.
//
#[no_mangle]
pub unsafe extern "C" fn tomoyo_del_group(element: *mut list_head) {
    static inline void tomoyo_del_group(struct list_head *element)
    {
    struct tomoyo_group *group =
    container_of(element, typeof(*group), head.list);
    tomoyo_put_name(group.group_name);
    }
//
// tomoyo_del_address_group - Delete members in "struct tomoyo_address_group".
//
// @element: Pointer to "struct list_head".
//
// Returns nothing.
//
#[no_mangle]
pub unsafe extern "C" fn tomoyo_del_address_group(element: *mut list_head) {
    static inline void tomoyo_del_address_group(struct list_head *element)
    {
// Nothing to do.
    }
//
// tomoyo_del_number_group - Delete members in "struct tomoyo_number_group".
//
// @element: Pointer to "struct list_head".
//
// Returns nothing.
//
#[no_mangle]
pub unsafe extern "C" fn tomoyo_del_number_group(element: *mut list_head) {
    static inline void tomoyo_del_number_group(struct list_head *element)
    {
// Nothing to do.
    }
//
// tomoyo_try_to_gc - Try to kfree() an entry.
//
// @type:    One of values in "enum tomoyo_policy_id".
// @element: Pointer to "struct list_head".
//
// Returns nothing.
//
// Caller holds tomoyo_policy_lock mutex.
//
    static void tomoyo_try_to_gc(const enum tomoyo_policy_id type,
    struct list_head *element)
    __must_hold(&tomoyo_policy_lock)
    {
//
// __list_del_entry() guarantees that the list element became no longer
// reachable from the list which the element was originally on (e.g.
// tomoyo_domain_list). Also, synchronize_srcu() guarantees that the
// list element became no longer referenced by syscall users.
//
    __list_del_entry(element);
    mutex_unlock(&tomoyo_policy_lock);
    synchronize_srcu(&tomoyo_ss);
//
// However, there are two users which may still be using the list
// element. We need to defer until both users forget this element.
//
// Don't kfree() until "struct tomoyo_io_buffer"->r.{domain,group,acl}
// and "struct tomoyo_io_buffer"->w.domain forget this element.
//
    if (tomoyo_struct_used_by_io_buffer(element))
    goto reinject;
    switch (type) {
    case TOMOYO_ID_TRANSITION_CONTROL:
    tomoyo_del_transition_control(element);
    break;
    case TOMOYO_ID_MANAGER:
    tomoyo_del_manager(element);
    break;
    case TOMOYO_ID_AGGREGATOR:
    tomoyo_del_aggregator(element);
    break;
    case TOMOYO_ID_GROUP:
    tomoyo_del_group(element);
    break;
    case TOMOYO_ID_PATH_GROUP:
    tomoyo_del_path_group(element);
    break;
    case TOMOYO_ID_ADDRESS_GROUP:
    tomoyo_del_address_group(element);
    break;
    case TOMOYO_ID_NUMBER_GROUP:
    tomoyo_del_number_group(element);
    break;
    case TOMOYO_ID_CONDITION:
    tomoyo_del_condition(element);
    break;
    case TOMOYO_ID_NAME:
//
// Don't kfree() until all "struct tomoyo_io_buffer"->r.w[]
// forget this element.
//
    if (tomoyo_name_used_by_io_buffer
    (container_of(element, typeof(struct tomoyo_name),
    head.list).entry.name))
    goto reinject;
    tomoyo_del_name(element);
    break;
    case TOMOYO_ID_ACL:
    tomoyo_del_acl(element);
    break;
    case TOMOYO_ID_DOMAIN:
//
// Don't kfree() until all "struct cred"->security forget this
// element.
//
    if (atomic_read(&container_of
    (element, typeof(struct tomoyo_domain_info),
    list).users))
    goto reinject;
    break;
    case TOMOYO_MAX_POLICY:
    break;
    }
    mutex_lock(&tomoyo_policy_lock);
    if (type == TOMOYO_ID_DOMAIN)
    tomoyo_del_domain(element);
    tomoyo_memory_free(element);
    return;
    reinject:
//
// We can safely reinject this element here because
// (1) Appending list elements and removing list elements are protected
// by tomoyo_policy_lock mutex.
// (2) Only this function removes list elements and this function is
// exclusively executed by tomoyo_gc_mutex mutex.
// are true.
//
    mutex_lock(&tomoyo_policy_lock);
    list_add_rcu(element, element.prev);
    }
//
// tomoyo_collect_member - Delete elements with "struct tomoyo_acl_head".
//
// @id:          One of values in "enum tomoyo_policy_id".
// @member_list: Pointer to "struct list_head".
//
// Returns nothing.
//
    static void tomoyo_collect_member(const enum tomoyo_policy_id id,
    struct list_head *member_list)
    __must_hold(&tomoyo_policy_lock)
    {
    struct tomoyo_acl_head *member;
    struct tomoyo_acl_head *tmp;
    list_for_each_entry_safe(member, tmp, member_list, list) {
    if (!member.is_deleted)
    continue;
    member.is_deleted = TOMOYO_GC_IN_PROGRESS;
    tomoyo_try_to_gc(id, &member.list);
    }
    }
//
// tomoyo_collect_acl - Delete elements in "struct tomoyo_domain_info".
//
// @list: Pointer to "struct list_head".
//
// Returns nothing.
//
#[no_mangle]
unsafe extern "C" fn tomoyo_collect_acl(list: *mut list_head) {
    static void tomoyo_collect_acl(struct list_head *list)
    __must_hold(&tomoyo_policy_lock)
    {
    struct tomoyo_acl_info *acl;
    struct tomoyo_acl_info *tmp;
    list_for_each_entry_safe(acl, tmp, list, list) {
    if (!acl.is_deleted)
    continue;
    acl.is_deleted = TOMOYO_GC_IN_PROGRESS;
    tomoyo_try_to_gc(TOMOYO_ID_ACL, &acl.list);
    }
    }
//
// tomoyo_collect_entry - Try to kfree() deleted elements.
//
// Returns nothing.
//
#[no_mangle]
unsafe extern "C" fn tomoyo_collect_entry() {
    static void tomoyo_collect_entry(void)
    {
    int i;
    enum tomoyo_policy_id id;
    struct tomoyo_policy_namespace *ns;
    mutex_lock(&tomoyo_policy_lock);
    {
    struct tomoyo_domain_info *domain;
    struct tomoyo_domain_info *tmp;
    list_for_each_entry_safe(domain, tmp, &tomoyo_domain_list,
    list) {
    tomoyo_collect_acl(&domain.acl_info_list);
    if (!domain.is_deleted || atomic_read(&domain.users))
    continue;
    tomoyo_try_to_gc(TOMOYO_ID_DOMAIN, &domain.list);
    }
    }
    list_for_each_entry(ns, &tomoyo_namespace_list, namespace_list) {
    for (id = 0; id < TOMOYO_MAX_POLICY; id++)
    tomoyo_collect_member(id, &ns.policy_list[id]);
    for (i = 0; i < TOMOYO_MAX_ACL_GROUPS; i++)
    tomoyo_collect_acl(&ns.acl_group[i]);
    }
    {
    struct tomoyo_shared_acl_head *ptr;
    struct tomoyo_shared_acl_head *tmp;
    list_for_each_entry_safe(ptr, tmp, &tomoyo_condition_list,
    list) {
    if (atomic_read(&ptr.users) > 0)
    continue;
    atomic_set(&ptr.users, TOMOYO_GC_IN_PROGRESS);
    tomoyo_try_to_gc(TOMOYO_ID_CONDITION, &ptr.list);
    }
    }
    list_for_each_entry(ns, &tomoyo_namespace_list, namespace_list) {
    for (i = 0; i < TOMOYO_MAX_GROUP; i++) {
    struct list_head *list = &ns.group_list[i];
    struct tomoyo_group *group;
    struct tomoyo_group *tmp;
    switch (i) {
    case 0:
    id = TOMOYO_ID_PATH_GROUP;
    break;
    case 1:
    id = TOMOYO_ID_NUMBER_GROUP;
    break;
    default:
    id = TOMOYO_ID_ADDRESS_GROUP;
    break;
    }
    list_for_each_entry_safe(group, tmp, list, head.list) {
    tomoyo_collect_member(id, &group.member_list);
    if (!list_empty(&group.member_list) ||
    atomic_read(&group.head.users) > 0)
    continue;
    atomic_set(&group.head.users,
    TOMOYO_GC_IN_PROGRESS);
    tomoyo_try_to_gc(TOMOYO_ID_GROUP,
    &group.head.list);
    }
    }
    }
    for (i = 0; i < TOMOYO_MAX_HASH; i++) {
    struct list_head *list = &tomoyo_name_list[i];
    struct tomoyo_shared_acl_head *ptr;
    struct tomoyo_shared_acl_head *tmp;
    list_for_each_entry_safe(ptr, tmp, list, list) {
    if (atomic_read(&ptr.users) > 0)
    continue;
    atomic_set(&ptr.users, TOMOYO_GC_IN_PROGRESS);
    tomoyo_try_to_gc(TOMOYO_ID_NAME, &ptr.list);
    }
    }
    mutex_unlock(&tomoyo_policy_lock);
    }
//
// tomoyo_gc_thread - Garbage collector thread function.
//
// @unused: Unused.
//
// Returns 0.
//
#[no_mangle]
unsafe extern "C" fn tomoyo_gc_thread(unused: *mut c_void) -> c_int {
    static int tomoyo_gc_thread(void *unused)
    {
// Garbage collector thread is exclusive.
    static DEFINE_MUTEX(tomoyo_gc_mutex);
    if (!mutex_trylock(&tomoyo_gc_mutex))
    goto out;
    tomoyo_collect_entry();
    {
    struct tomoyo_io_buffer *head;
    struct tomoyo_io_buffer *tmp;
    spin_lock(&tomoyo_io_buffer_list_lock);
    list_for_each_entry_safe(head, tmp, &tomoyo_io_buffer_list,
    list) {
    if (head.users)
    continue;
    list_del(&head.list);
// Safe destruction because no users are left.
    context_unsafe(
    kfree(head.read_buf);
    kfree(head.write_buf);
    );
    kfree(head);
    }
    spin_unlock(&tomoyo_io_buffer_list_lock);
    }
    mutex_unlock(&tomoyo_gc_mutex);
    out:
// This acts as do_exit(0).
    return 0;
    }
//
// tomoyo_notify_gc - Register/unregister /sys/kernel/security/tomoyo/ users.
//
// @head:        Pointer to "struct tomoyo_io_buffer".
// @is_register: True if register, false if unregister.
//
// Returns nothing.
//
#[no_mangle]
pub unsafe extern "C" fn tomoyo_notify_gc(head: *mut tomoyo_io_buffer, is_register: bool) {
    void tomoyo_notify_gc(struct tomoyo_io_buffer *head, const bool is_register)
    {
    let mut is_write: bool = false;
    spin_lock(&tomoyo_io_buffer_list_lock);
    if (is_register) {
    head.users = 1;
    list_add(&head.list, &tomoyo_io_buffer_list);
    } else {
//
// tomoyo_write_control() can concurrently update write_buf from
// a non-NULL to new non-NULL pointer with io_sem held.
//
    is_write = data_race(head.write_buf != core::ptr::null_mut());
    if (!--head.users) {
    list_del(&head.list);
// Safe destruction because no users are left.
    context_unsafe(
    kfree(head.read_buf);
    kfree(head.write_buf);
    );
    kfree(head);
    }
    }
    spin_unlock(&tomoyo_io_buffer_list_lock);
    if (is_write)
    kthread_run(tomoyo_gc_thread, core::ptr::null_mut(), "GC for TOMOYO");
    }
