//! Automatically rewritten from C Header to Rust Module
//! Source: include/trace/events/landlock.h
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
// Copyright © 2025 Microsoft Corporation
// Copyright © 2026 Cloudflare, Inc.
//

//
// Escapes @len bytes of an untrusted string into the trace sequence @p so it
// cannot inject field separators or control characters into the ftrace text
// output, and can be unambiguously recovered.  Called from the TP_printk() of
// the tracepoints that expose paths and process names.  @len is passed by the
// caller (rather than derived with strlen()) so a name that is not
// NUL-terminated or carries embedded NUL bytes (an abstract socket name) is
// escaped in full instead of being truncated at the first NUL.
//
// Return: a pointer into @p's buffer, or NULL if @src is NULL or the buffer is
// exhausted (normal when the trace buffer is full).
//
// Buffer exhaustion is normal when the trace buffer is full.
// We need some room for the final '\0'.
//
// Fills the dense per-domain-layer array layers (one access mask per layer,
// indexed by level - 1) from rule's sparse layer stack, keeping only the
// requested rights (access_request).  Layers with no matching rule entry get
// a zero mask.  Shared by the check_rule_fs and check_rule_net events.
//
// rule->layers is sorted by ascending level, with levels in the domain's
// [1, num_layers] range (see landlock_merge_ruleset()), so every entry maps
// to a slot.  A leftover entry would be a malformed rule; the zero-filled
// slots keep the output and the array bounds safe regardless.
//
// A leftover entry means an out-of-range or unsorted rule level.
//
// Renders the dense per-domain-layer access array as symbolic flag names for
// the grants field: layers wrapped in "{}", flags within a layer joined by
// "|", layers separated by ",", an empty layer rendered as nothing.
// Open-codes the flag walk because trace_print_flags_seq() NUL-terminates per
// call and so cannot be chained into a single field.  The shared names table
// covers every access right, so masked bits are always named.  Returns the
// trace_seq position like __print_flags().
//

// clang-format off
// Maps a shared _LANDLOCK_*_NAMES entry to a __print_flags() pair.

//
// DOC: Landlock trace events
//
// These guarantees and constraints hold for every Landlock tracepoint.
// A new tracepoint must uphold them, and an eBPF consumer can rely on
// them.
//
// Decision context
// ~~~~~~~~~~~~~~~~
//
// A denial event, together with the lifecycle events, exposes the full
// set of inputs the verdict consumed, so a consumer that tracked domain
// creation (landlock_create_ruleset, landlock_create_domain) can verify
// or reproduce the Landlock decision rather than merely observe it
// happened.  In who/what/why terms: who is the denying domain (the domain
// field, always the subject that enforced the policy, never the current
// task), what is the operation and its object, and why is every other
// input the verdict weighed.
//
// Lifecycle consistency
// ~~~~~~~~~~~~~~~~~~~~~~
//
// Lifecycle events are balanced: a creation event always has a matching
// deallocation event and vice versa, so an eBPF program can model object
// lifetimes from the trace stream without reconciliation logic.  A creation
// event fires while the object is still private to the calling thread
// (landlock_create_ruleset fires before the ruleset's file descriptor is
// installed, so it cannot race a concurrent :manpage:`close(2)`); if fd
// installation later fails and the ruleset is freed, free_ruleset still
// fires, keeping the pair balanced.  The domain pair (create_domain and
// free_domain) is balanced the same way: create_domain fires when the
// domain is created (under the ruleset lock, before thread-sync), and
// free_domain fires when it is freed.  A rare thread-sync failure aborts
// the just-created domain, which then emits both events (its creation, then
// an immediate free).  Denial events fire only for denials that actually
// happen.
//
// Pointer access
// ~~~~~~~~~~~~~~
//
// All pointer arguments in TP_PROTO are guaranteed non-NULL by the
// caller, but pointers reached through them may still be NULL (e.g.,
// hierarchy->parent at a root domain) and must be checked.  eBPF programs
// read these pointers via BTF for richer introspection than the
// TP_STRUCT__entry fields, which serve TP_printk display only.
//
// Mutable object pointers are passed while the caller holds the object's
// lock, so TP_fast_assign and a BTF reader see the exact object the event
// reports, a snapshot no concurrent writer can change: add_rule holds the
// modified ruleset's lock, and create_domain holds the ruleset lock across
// the emission (before the thread-sync wait) so the inspected ruleset is
// the one merged into the domain.  Objects immutable at the emission site
// (a domain after creation, a hierarchy at its last reference) need no
// lock.  A few values that no held lock protects are a best-effort
// lockless snapshot instead: a task's comm, and the deny_access_net struct
// sock (whose network hook holds no socket lock), matching how the sched
// and signal trace events sample comm.
//
// Field encoding
// ~~~~~~~~~~~~~~
//
// Fields that mirror the Landlock UAPI use the same C types and endianness
// (e.g. network ports are __u64 in host endianness, like
// landlock_net_port_attr.port).  Per-event details, such as where a value
// is byte-swapped, live in the field's own kdoc.
//
// Rule-check fields
// ~~~~~~~~~~~~~~~~~
//
// The check_rule events fire during an access check, once per matching
// rule, before the final allow-or-deny verdict.  They share domain (the
// enforcing domain being evaluated), access_request (the access mask being
// checked), and rule (the matching rule, with per-layer access masks).
//
// Denial fields
// ~~~~~~~~~~~~~
//
// Every denial event shares three fields.  domain is the ID of the
// innermost domain that blocked the access.  same_exec tells whether the
// current task is the same executable that entered that domain.  logged is
// the domain's audit-logging decision for this denial (its log_status is
// enabled and the per-execution flag selected by same_exec is set); a
// stateless ftrace filter can select the denials the domain submits to
// audit with logged==1, without reconstructing it from the per-execution
// log flags.  Denial events order their fields as domain, same_exec,
// logged, then blockers (deny_access events only), then the type-specific
// object fields, then any variable-length field.
//
// Relational referents
// ~~~~~~~~~~~~~~~~~~~~~
//
// A scope or ptrace verdict compares two domains, so the other party's
// domain is part of the decision context.  It is exposed as a scalar
// domain ID (0 when that party is unsandboxed): target_domain (signal),
// peer_domain (abstract unix socket), tracee_domain (ptrace).  With both
// IDs in the stream, a consumer that tracked domain creation can relate
// the two parties without kernel-internal state.  The ID is a scalar
// snapshot, not a live domain pointer that could dangle: an optional
// relational referent is a scalar (0 sentinel), not a nullable pointer.
//
// Prints a per-layer access mask array (the dynamic array @array) as symbolic
// flag names using the shared @flag_names list (a _LANDLOCK_*_NAMES macro).
// Stays outside CREATE_TRACE_POINTS: TP_printk is expanded in the print-output
// pass where that macro is undefined.
//

//
// landlock_create_ruleset - New ruleset created
//
// @ruleset: Newly created ruleset (never NULL); not yet shared via an fd,
// so no lock is needed.
//
// Emitted by sys_landlock_create_ruleset() while the new ruleset is still
// private to the calling thread, before its file descriptor is installed,
// so it cannot race a concurrent :manpage:`close(2)`.  Balanced by a
// matching landlock_free_ruleset event.
//
// landlock_free_ruleset - Ruleset freed
//
// @ruleset: Ruleset being freed (never NULL); at its last reference, so no
// lock is needed.
//
// Emitted when a ruleset's last reference is dropped (typically when
// the creating process closes the ruleset file descriptor).  Fires even
// when file-descriptor installation failed after creation, keeping the
// create/free pair balanced.
//
// landlock_add_rule_fs - Filesystem rule added to a ruleset
//
// @ruleset: Source ruleset (never NULL).
// @access_rights: Effective access mask stored in the rule, not the raw
// sys_landlock_add_rule() argument (unhandled rights
// added).
// @path: Filesystem path for the rule (never NULL).
// @pathname: Resolved absolute path string (never NULL; error placeholder
// on resolution failure).
//
// Emitted by sys_landlock_add_rule() under the modified ruleset's lock, so
// the reported ruleset is a stable snapshot that no concurrent writer can
// change.
//
// The inode number may not be the user-visible one,
// but it will be the same used by audit.
//
// landlock_add_rule_net - Network port rule added to a ruleset
//
// @ruleset: Source ruleset (never NULL).
// @access_rights: Effective access mask stored in the rule, not the raw
// sys_landlock_add_rule() argument (unhandled rights
// added).
// @port: Network port, the landlock_net_port_attr.port UAPI value
// forwarded directly.
//
// Emitted by sys_landlock_add_rule() under the modified ruleset's lock, so
// the reported ruleset is a stable snapshot that no concurrent writer can
// change.
//
// landlock_create_domain - New domain created
//
// @domain: Newly created domain (never NULL, immutable after creation).
// @domain->hierarchy->id is its unique ID, shared with the
// landlock_enforce_domain and landlock_free_domain events;
// @domain->hierarchy->details holds the requesting process.
// @ruleset: Source ruleset frozen into the domain (never NULL).  The
// ruleset lock is held across the emission, so a BPF program
// reading it via BTF sees the exact merged ruleset;
// @ruleset->id / @ruleset->version identify it.
//
// Emitted by sys_landlock_restrict_self() once, in the requesting
// thread's context, right after the merge and before thread-sync.  The
// flags-only path (ruleset_fd == -1) creates no domain and does not
// emit this event.  Paired with the per-thread landlock_enforce_domain
// (join on @domain->hierarchy->id) and balanced by a matching
// landlock_free_domain event.
//
// landlock_enforce_domain - Domain enforced on a thread
//
// @domain: Domain now enforced on the current thread (never NULL,
// immutable; read locklessly).  Correlate to
// landlock_create_domain via @domain->hierarchy->id for the
// source ruleset and requesting thread, or read
// @domain->hierarchy->details for the requesting process.
// @complete: Set on the single event that concludes the operation, after
// all its other enforcements; filter on it for one event per
// operation.
// @process_wide: The enforcement covers every eligible (non-exiting)
// thread of the process: set when the caller used
// %LANDLOCK_RESTRICT_SELF_TSYNC or the process is
// single-threaded.  A lone thread whose group still
// holds a zombie leader is not counted single-threaded,
// so process_wide == 0 never proves the opposite.
// @no_new_privs: The enforcing thread's no_new_privs state at
// enforcement time: 1 if set (by a prior
// :manpage:`prctl(2)` %PR_SET_NO_NEW_PRIVS or by
// %LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS), 0 if the domain
// was enforced with %CAP_SYS_ADMIN instead.
//
// Emitted for each thread sys_landlock_restrict_self() enforces the
// domain on, in that thread's own context, right after its
// commit_creds(), so it fires only once the thread is irreversibly
// enforcing the domain (aborted operations emit none).  Not
// balanced; every enforcement falls between the domain's
// landlock_create_domain and landlock_free_domain events.
//
// @complete == 1 && @process_wide == 1 means the whole process is
// sandboxed by @domain, durably (Landlock domains are monotonic and
// inherited on :manpage:`clone(2)`).
//
// landlock_free_domain - Domain freed
//
// @hierarchy: Hierarchy node being freed (never NULL).
//
// Emitted when the hierarchy node's last reference is dropped: its
// refcount reaches zero after all child domains have released their
// parent reference.  A committed domain is
// freed from a kworker via landlock_put_domain_deferred() (the credential
// free path runs in RCU context, where sleeping is forbidden), so the
// current task is not the sandboxed task that triggered the free.  Balanced
// by a matching landlock_create_domain event.
//
// landlock_check_rule_fs - Filesystem rule evaluated during access check
//
// @domain: Enforcing domain (never NULL).
// @rule: Matching rule with per-layer access masks (never NULL).
// @access_request: Access mask evaluated against the rule (the domain's
// handled mask during rename/link double-checks).
// @dentry: Filesystem dentry being checked (never NULL).
//
// Emitted for each rule that matches during a filesystem access check.
// The grants array shows the requested rights the rule grants at each
// domain layer.  See Documentation/trace/events-landlock.rst for how to
// interpret it.
//
// landlock_check_rule_net - Network port rule evaluated during access check
//
// @domain: Enforcing domain (never NULL).
// @rule: Matching rule with per-layer access masks (never NULL).
// @access_request: Access mask being requested.
// @port: Network port being checked (host endianness).
//
// Emitted for each rule that matches during a network access check.  The
// grants array shows the requested rights the rule grants at each domain
// layer.  See Documentation/trace/events-landlock.rst for how to
// interpret it.
//
// landlock_deny_access_fs - Filesystem access denied
//
// @hierarchy: Denying domain's hierarchy node (never NULL); its id is the
// domain field.
// @same_exec: Whether the current task entered the denying domain itself.
// @logged: The domain's audit-logging decision for this denial.
// @blockers: Access mask that was blocked (zero for a mount-topology
// change, whose only blocker is the operation itself).
// @path: Filesystem path that was denied (never NULL).
// @pathname: Resolved path string (never NULL; an error placeholder on
// resolution failure).
//
// Emitted when a Landlock domain denies a filesystem access.
//
// A negative dentry has no backing inode, so mirror the
// guard in dump_common_audit_data() and report inode 0.
//
// landlock_deny_access_net - Network access denied
//
// @hierarchy: Denying domain's hierarchy node (never NULL); its id is the
// domain field.
// @same_exec: Whether the current task entered the denying domain itself.
// @logged: The domain's audit-logging decision for this denial.
// @blockers: Access mask that was blocked.
// @sk: Socket object (never NULL), read without a socket lock, so its
// fields are a best-effort snapshot.  The denied endpoint is not
// available: the hook runs before :manpage:`bind(2)`
// :manpage:`connect(2)` sets the socket addresses.
// @sport: Source port in host endianness, set for bind denials (zero for
// an autobind/ephemeral port); zero for connect and send denials.
// @dport: Destination port in host endianness, set for connect and send
// denials; zero for bind denials, and also zero for a UDP send to
// an AF_UNSPEC address on an IPv6 socket (indistinguishable from a
// real destination port 0).  The bind-vs-connect direction is
// given by @blockers, not by which port is set.
//
// Emitted when a Landlock domain denies a network operation.
//
// The port fields are converted from the socket's network byte order to
// host endianness before emitting.
//
// landlock_deny_ptrace - Ptrace access denied by a Landlock domain
//
// @hierarchy: Denying domain's hierarchy node (never NULL); its id is the
// domain field.
// @same_exec: Whether the current task entered the denying domain itself.
// @logged: The domain's audit-logging decision for this denial.
// @tracee_domain_id: The tracee's Landlock domain ID, or 0 if the tracee
// is unsandboxed.
// @tracee: The target task ptrace acted on (never NULL).  tracee_pid is
// the init-namespace TGID (like audit's opid).
//
// Emitted when a Landlock domain denies a ptrace operation.
//
// landlock_deny_scope_signal - Signal delivery denied by
// LANDLOCK_SCOPE_SIGNAL
//
// @hierarchy: Denying domain's hierarchy node (never NULL); its id is the
// domain field.
// @same_exec: Whether the current task entered the denying domain itself.
// @logged: The domain's audit-logging decision for this denial.
// @target_domain_id: The target's Landlock domain ID, or 0 if the target
// is unsandboxed.
// @target: The task the signal was aimed at (never NULL).  target_pid is
// the init-namespace TGID (like audit's opid).
//
// Emitted when a Landlock domain denies signal delivery to a scoped-out
// target.
//
// landlock_deny_scope_abstract_unix_socket - Abstract unix socket access
// denied by LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET
//
// @hierarchy: Denying domain's hierarchy node (never NULL); its id is the
// domain field.
// @same_exec: Whether the current task entered the denying domain itself.
// @logged: The domain's audit-logging decision for this denial.
// @peer_domain_id: The peer's Landlock domain ID, or 0 if the peer is
// unsandboxed.
// @peer: Peer socket (never NULL).  peer_pid is best-effort: it is 0 for
// a datagram peer (no SO_PEERCRED), so sun_path is the reliable
// peer identifier.
//
// Emitted when a Landlock domain denies access to a scoped-out abstract
// unix socket.
//
// Abstract socket names are untrusted binary data from
// user space.  Use __string_len because abstract names
// are not NUL-terminated; their length is determined by
// addr->len.  unix_sk(peer)->addr is stable here because
// the caller (hook_unix_stream_connect or
// hook_unix_may_send) holds unix_state_lock(peer).
//
// Best-effort (0 for a datagram peer).  sk_peer_pid is
// canonically guarded by sk->sk_peer_lock, but the target
// peer's peercred is set once and not updated concurrently in
// these hooks, so this READ_ONCE() is safe; sun_path is the
// reliable identifier.
//

// This part must be outside protection

// clang-format on
