//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/acpica/nsalloc.c
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Module Name: nsalloc - Namespace allocation and deletion utilities
//

    ACPI_MODULE_NAME("nsalloc")
//
// FUNCTION:    acpi_ns_create_node
//
// PARAMETERS:  name            - Name of the new node (4 char ACPI name)
//
// RETURN:      New namespace node (Null on failure)
//
// DESCRIPTION: Create a namespace node
//
    struct acpi_namespace_node *acpi_ns_create_node(u32 name)
    {
    struct acpi_namespace_node *node;

    u32 temp;

    ACPI_FUNCTION_TRACE(ns_create_node);
    node = acpi_os_acquire_object(acpi_gbl_namespace_cache);
    if (!node) {
    return_PTR(core::ptr::null_mut());
    }
    ACPI_MEM_TRACKING(acpi_gbl_ns_node_list.total_allocated++);

    temp = acpi_gbl_ns_node_list.total_allocated -
    acpi_gbl_ns_node_list.total_freed;
    if (temp > acpi_gbl_ns_node_list.max_occupied) {
    acpi_gbl_ns_node_list.max_occupied = temp;
    }

    node.name.integer = name;
    ACPI_SET_DESCRIPTOR_TYPE(node, ACPI_DESC_TYPE_NAMED);
    return_PTR(node);
    }
//
// FUNCTION:    acpi_ns_delete_node
//
// PARAMETERS:  node            - Node to be deleted
//
// RETURN:      None
//
// DESCRIPTION: Delete a namespace node. All node deletions must come through
// here. Detaches any attached objects, including any attached
// data. If a handler is associated with attached data, it is
// invoked before the node is deleted.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ns_delete_node(node: *mut acpi_namespace_node) {
    void acpi_ns_delete_node(struct acpi_namespace_node *node)
    {
    union acpi_operand_object *obj_desc;
    union acpi_operand_object *next_desc;
    ACPI_FUNCTION_NAME(ns_delete_node);
    if (!node) {
    return_VOID;
    }
// Detach an object if there is one
    acpi_ns_detach_object(node);
//
// Delete an attached data object list if present (objects that were
// attached via acpi_attach_data). Note: After any normal object is
// detached above, the only possible remaining object(s) are data
// objects, in a linked list.
//
    obj_desc = node.object;
    while (obj_desc && (obj_desc.common.type == ACPI_TYPE_LOCAL_DATA)) {
// Invoke the attached data deletion handler if present
    if (obj_desc.data.handler) {
    obj_desc.data.handler(node, obj_desc.data.pointer);
    }
    next_desc = obj_desc.common.next_object;
    acpi_ut_remove_reference(obj_desc);
    obj_desc = next_desc;
    }
// Special case for the statically allocated root node
    if (node == acpi_gbl_root_node) {
    return;
    }
// Now we can delete the node
    (void)acpi_os_release_object(acpi_gbl_namespace_cache, node);
    ACPI_MEM_TRACKING(acpi_gbl_ns_node_list.total_freed++);
    ACPI_DEBUG_PRINT((ACPI_DB_ALLOCATIONS, "Node %p, Remaining %X\n",
    node, acpi_gbl_current_node_count));
    }
//
// FUNCTION:    acpi_ns_remove_node
//
// PARAMETERS:  node            - Node to be removed/deleted
//
// RETURN:      None
//
// DESCRIPTION: Remove (unlink) and delete a namespace node
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ns_remove_node(node: *mut acpi_namespace_node) {
    void acpi_ns_remove_node(struct acpi_namespace_node *node)
    {
    struct acpi_namespace_node *parent_node;
    struct acpi_namespace_node *prev_node;
    struct acpi_namespace_node *next_node;
    ACPI_FUNCTION_TRACE_PTR(ns_remove_node, node);
    parent_node = node.parent;
    prev_node = core::ptr::null_mut();
    next_node = parent_node.child;
// Find the node that is the previous peer in the parent's child list
    while (next_node != node) {
    prev_node = next_node;
    next_node = next_node.peer;
    }
    if (prev_node) {
// Node is not first child, unlink it
    prev_node.peer = node.peer;
    } else {
//
// Node is first child (has no previous peer).
// Link peer list to parent
//
    parent_node.child = node.peer;
    }
// Delete the node and any attached objects
    acpi_ns_delete_node(node);
    return_VOID;
    }
//
// FUNCTION:    acpi_ns_install_node
//
// PARAMETERS:  walk_state      - Current state of the walk
// parent_node     - The parent of the new Node
// node            - The new Node to install
// type            - ACPI object type of the new Node
//
// RETURN:      None
//
// DESCRIPTION: Initialize a new namespace node and install it amongst
// its peers.
//
// Note: Current namespace lookup is linear search. This appears
// to be sufficient as namespace searches consume only a small
// fraction of the execution time of the ACPI subsystem.
//
    void acpi_ns_install_node(struct acpi_walk_state *walk_state, struct acpi_namespace_node *parent_node,	/* Parent */
    struct acpi_namespace_node *node,	/* New Child */
    acpi_object_type type)
    {
    let mut owner_id: acpi_owner_id = 0;
    struct acpi_namespace_node *child_node;
    ACPI_FUNCTION_TRACE(ns_install_node);
    if (walk_state) {
//
// Get the owner ID from the Walk state. The owner ID is used to
// track table deletion and deletion of objects created by methods.
//
    owner_id = walk_state.owner_id;
    if ((walk_state.method_desc) &&
    (parent_node != walk_state.method_node)) {
//
// A method is creating a new node that is not a child of the
// method (it is non-local). Mark the executing method as having
// modified the namespace. This is used for cleanup when the
// method exits.
//
    walk_state.method_desc.method.info_flags |=
    ACPI_METHOD_MODIFIED_NAMESPACE;
    }
    }
// Link the new entry into the parent and existing children
    node.peer = core::ptr::null_mut();
    node.parent = parent_node;
    child_node = parent_node.child;
    if (!child_node) {
    parent_node.child = node;
    } else {
// Add node to the end of the peer list
    while (child_node.peer) {
    child_node = child_node.peer;
    }
    child_node.peer = node;
    }
// Init the new entry
    node.owner_id = owner_id;
    node.type = (u8) type;
    ACPI_DEBUG_PRINT((ACPI_DB_NAMES,
    "%4.4s (%s) [Node %p Owner %3.3X] added to %4.4s (%s) [Node %p]\n",
    acpi_ut_get_node_name(node),
    acpi_ut_get_type_name(node.type), node, owner_id,
    acpi_ut_get_node_name(parent_node),
    acpi_ut_get_type_name(parent_node.type),
    parent_node));
    return_VOID;
    }
//
// FUNCTION:    acpi_ns_delete_children
//
// PARAMETERS:  parent_node     - Delete this objects children
//
// RETURN:      None.
//
// DESCRIPTION: Delete all children of the parent object. In other words,
// deletes a "scope".
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ns_delete_children(parent_node: *mut acpi_namespace_node) {
    void acpi_ns_delete_children(struct acpi_namespace_node *parent_node)
    {
    struct acpi_namespace_node *next_node;
    struct acpi_namespace_node *node_to_delete;
    ACPI_FUNCTION_TRACE_PTR(ns_delete_children, parent_node);
    if (!parent_node) {
    return_VOID;
    }
// Deallocate all children at this level
    next_node = parent_node.child;
    while (next_node) {
// Grandchildren should have all been deleted already
    if (next_node.child) {
    ACPI_ERROR((AE_INFO, "Found a grandchild! P=%p C=%p",
    parent_node, next_node));
    }
//
// Delete this child node and move on to the next child in the list.
// No need to unlink the node since we are deleting the entire branch.
//
    node_to_delete = next_node;
    next_node = next_node.peer;
    acpi_ns_delete_node(node_to_delete);
    }
// Clear the parent's child pointer
    parent_node.child = core::ptr::null_mut();
    return_VOID;
    }
//
// FUNCTION:    acpi_ns_delete_namespace_subtree
//
// PARAMETERS:  parent_node     - Root of the subtree to be deleted
//
// RETURN:      None.
//
// DESCRIPTION: Delete a subtree of the namespace. This includes all objects
// stored within the subtree.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ns_delete_namespace_subtree(parent_node: *mut acpi_namespace_node) {
    void acpi_ns_delete_namespace_subtree(struct acpi_namespace_node *parent_node)
    {
    struct acpi_namespace_node *child_node = core::ptr::null_mut();
    let mut level: u32 = 1;
    acpi_status status;
    ACPI_FUNCTION_TRACE(ns_delete_namespace_subtree);
    if (!parent_node) {
    return_VOID;
    }
// Lock namespace for possible update
    status = acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE);
    if (ACPI_FAILURE(status)) {
    return_VOID;
    }
//
// Traverse the tree of objects until we bubble back up
// to where we started.
//
    while (level > 0) {
// Get the next node in this scope (NULL if none)
    child_node = acpi_ns_get_next_node(parent_node, child_node);
    if (child_node) {
// Found a child node - detach any attached object
    acpi_ns_detach_object(child_node);
// Check if this node has any children
    if (child_node.child) {
//
// There is at least one child of this node,
// visit the node
//
    level++;
    parent_node = child_node;
    child_node = core::ptr::null_mut();
    }
    } else {
//
// No more children of this parent node.
// Move up to the grandparent.
//
    level--;
//
// Now delete all of the children of this parent
// all at the same time.
//
    acpi_ns_delete_children(parent_node);
// New "last child" is this parent node
    child_node = parent_node;
// Move up the tree to the grandparent
    parent_node = parent_node.parent;
    }
    }
    (void)acpi_ut_release_mutex(ACPI_MTX_NAMESPACE);
    return_VOID;
    }
//
// FUNCTION:    acpi_ns_delete_namespace_by_owner
//
// PARAMETERS:  owner_id    - All nodes with this owner will be deleted
//
// RETURN:      Status
//
// DESCRIPTION: Delete entries within the namespace that are owned by a
// specific ID. Used to delete entire ACPI tables. All
// reference counts are updated.
//
// MUTEX:       Locks namespace during deletion walk.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ns_delete_namespace_by_owner(owner_id: acpi_owner_id) {
    void acpi_ns_delete_namespace_by_owner(acpi_owner_id owner_id)
    {
    struct acpi_namespace_node *child_node;
    struct acpi_namespace_node *deletion_node;
    struct acpi_namespace_node *parent_node;
    u32 level;
    acpi_status status;
    ACPI_FUNCTION_TRACE_U32(ns_delete_namespace_by_owner, owner_id);
    if (owner_id == 0) {
    return_VOID;
    }
// Lock namespace for possible update
    status = acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE);
    if (ACPI_FAILURE(status)) {
    return_VOID;
    }
    deletion_node = core::ptr::null_mut();
    parent_node = acpi_gbl_root_node;
    child_node = core::ptr::null_mut();
    level = 1;
//
// Traverse the tree of nodes until we bubble back up
// to where we started.
//
    while (level > 0) {
//
// Get the next child of this parent node. When child_node is NULL,
// the first child of the parent is returned
//
    child_node = acpi_ns_get_next_node(parent_node, child_node);
    if (deletion_node) {
    acpi_ns_delete_children(deletion_node);
    acpi_ns_remove_node(deletion_node);
    deletion_node = core::ptr::null_mut();
    }
    if (child_node) {
    if (child_node.owner_id == owner_id) {
// Found a matching child node - detach any attached object
    acpi_ns_detach_object(child_node);
    }
// Check if this node has any children
    if (child_node.child) {
//
// There is at least one child of this node,
// visit the node
//
    level++;
    parent_node = child_node;
    child_node = core::ptr::null_mut();
    } else if (child_node.owner_id == owner_id) {
    deletion_node = child_node;
    }
    } else {
//
// No more children of this parent node.
// Move up to the grandparent.
//
    level--;
    if (level != 0) {
    if (parent_node.owner_id == owner_id) {
    deletion_node = parent_node;
    }
    }
// New "last child" is this parent node
    child_node = parent_node;
// Move up the tree to the grandparent
    parent_node = parent_node.parent;
    }
    }
    (void)acpi_ut_release_mutex(ACPI_MTX_NAMESPACE);
    return_VOID;
    }
