//! Automatically rewritten from C to Rust
//! Source: net/unix/garbage.c
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
// NET3:	Garbage Collector For AF_UNIX sockets
//
// Garbage Collector:
// Copyright (C) Barak A. Pearlmutter.
//
// Chopped about by Alan Cox 22/3/96 to make it fit the AF_UNIX socket problem.
// If it doesn't work blame me, it worked when Barak sent it.
//
// Assumptions:
//
// - object w/ a bit
// - free list
//
// Current optimizations:
//
// - explicit stack instead of recursion
// - tail recurse on first born instead of immediate push/pop
// - we gather the stuff that should not be killed into tree
// and stack is just a path from root to the current pointer.
//
// Future optimizations:
//
// - don't just push entire root set; process in place
//
// Fixes:
// Alan Cox	07 Sept	1997	Vmalloc internal stack as needed.
// Cope with changing max_files.
// Al Viro		11 Oct 1998
// Graph may have cycles. That is, we can send the descriptor
// of foo to bar and vice versa. Current code chokes on that.
// Fix: move SCM_RIGHTS ones into the separate list and then
// skb_free() them all instead of doing explicit fput's.
// Another problem: since fput() may block somebody may
// create a new unix_socket when we are in the middle of sweep
// phase. Fix: revert the logic wrt MARKED. Mark everything
// upon the beginning and unmark non-junk ones.
//
// [12 Oct 1998] AAARGH! New code purges all SCM_RIGHTS
// sent to connect()'ed but still not accept()'ed sockets.
// Fixed. Old code had slightly different problem here:
// extra fput() in situation when we passed the descriptor via
// such socket and closed it (descriptor). That would happen on
// each unix_gc() until the accept(). Since the struct file in
// question would go to the free list and might be reused...
// That might be the reason of random oopses on filp_close()
// in unrelated processes.
//
// AV		28 Feb 1999
// Kill the explicit allocation of stack. Now we keep the tree
// with root in dummy + pointer (gc_current) to one of the nodes.
// Stack is represented as path from gc_current to dummy. Unmark
// now means "add to tree". Push == "make it a son of gc_current".
// Pop == "move gc_current to parent". We keep only pointers to
// parents (->gc_tree).
// AV		1 Mar 1999
// Damn. Added missing check for ->dead in listen queues scanning.
//
// Miklos Szeredi 25 Jun 2007
// Reimplement with a cycle collecting algorithm. This should
// solve several problems with the previous code, like being racy
// wrt receive and holding up unrelated socket operations.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct unix_vertex {
    pub edges: list_head,
    pub entry: list_head,
    pub scc_entry: list_head,
    pub out_degree: c_ulong,
    pub index: c_ulong,
    pub scc_index: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct unix_edge {
    pub predecessor: *mut unix_sock,
    pub successor: *mut unix_sock,
    pub vertex_entry: list_head,
    pub stack_entry: list_head,
}

    struct unix_sock *unix_get_socket(struct file *filp)
    {
    struct inode *inode = file_inode(filp);
// Socket ?
    if (S_ISSOCK(inode.i_mode) && !(filp.f_mode & FMODE_PATH)) {
    struct socket *sock = SOCKET_I(inode);
    const struct proto_ops *ops;
    struct sock *sk = sock.sk;
    ops = READ_ONCE(sock.ops);
// PF_UNIX ?
    if (sk && ops && ops.family == PF_UNIX)
    return unix_sk(sk);
    }
    return core::ptr::null_mut();
    }
    static struct unix_vertex *unix_edge_successor(struct unix_edge *edge)
    {
// If an embryo socket has a fd,
// the listener indirectly holds the fd's refcnt.
//
    if (edge.successor.listener)
    return unix_sk(edge.successor.listener).vertex;
    return edge.successor.vertex;
    }
    enum {
    UNIX_GRAPH_NOT_CYCLIC,
    UNIX_GRAPH_MAYBE_CYCLIC,
    UNIX_GRAPH_CYCLIC,
    };
    static unsigned char unix_graph_state;
#[no_mangle]
unsafe extern "C" fn unix_update_graph(vertex: *mut unix_vertex) {
    static void unix_update_graph(struct unix_vertex *vertex)
    {
// If the receiver socket is not inflight, no cyclic
// reference could be formed.
//
    if (!vertex)
    return;
    WRITE_ONCE(unix_graph_state, UNIX_GRAPH_MAYBE_CYCLIC);
    }
    static LIST_HEAD(unix_unvisited_vertices);
    enum unix_vertex_index {
    UNIX_VERTEX_INDEX_MARK1,
    UNIX_VERTEX_INDEX_MARK2,
    UNIX_VERTEX_INDEX_START,
    };
    let mut unix_vertex_unvisited_index: static unsigned long = UNIX_VERTEX_INDEX_MARK1;
    let mut unix_vertex_max_scc_index: static unsigned long = UNIX_VERTEX_INDEX_START;
#[no_mangle]
unsafe extern "C" fn unix_add_edge(fpl: *mut scm_fp_list, edge: *mut unix_edge) {
    static void unix_add_edge(struct scm_fp_list *fpl, struct unix_edge *edge)
    {
    struct unix_vertex *vertex = edge.predecessor.vertex;
    if (!vertex) {
    vertex = list_first_entry(&fpl.vertices, typeof(*vertex), entry);
    vertex.index = unix_vertex_unvisited_index;
    vertex.scc_index = ++unix_vertex_max_scc_index;
    vertex.out_degree = 0;
    INIT_LIST_HEAD(&vertex.edges);
    INIT_LIST_HEAD(&vertex.scc_entry);
    list_move_tail(&vertex.entry, &unix_unvisited_vertices);
    edge.predecessor.vertex = vertex;
    }
    vertex.out_degree++;
    list_add_tail(&edge.vertex_entry, &vertex.edges);
    unix_update_graph(unix_edge_successor(edge));
    }
#[no_mangle]
unsafe extern "C" fn unix_del_edge(fpl: *mut scm_fp_list, edge: *mut unix_edge) {
    static void unix_del_edge(struct scm_fp_list *fpl, struct unix_edge *edge)
    {
    struct unix_vertex *vertex = edge.predecessor.vertex;
    if (!fpl.dead)
    unix_update_graph(unix_edge_successor(edge));
    list_del(&edge.vertex_entry);
    vertex.out_degree--;
    if (!vertex.out_degree) {
    edge.predecessor.vertex = core::ptr::null_mut();
    list_move_tail(&vertex.entry, &fpl.vertices);
    list_del(&vertex.scc_entry);
    }
    }
#[no_mangle]
unsafe extern "C" fn unix_free_vertices(fpl: *mut scm_fp_list) {
    static void unix_free_vertices(struct scm_fp_list *fpl)
    {
    struct unix_vertex *vertex, *next_vertex;
    list_for_each_entry_safe(vertex, next_vertex, &fpl.vertices, entry) {
    list_del(&vertex.entry);
    kfree(vertex);
    }
    }
    static __cacheline_aligned_in_smp DEFINE_SPINLOCK(unix_gc_lock);
#[no_mangle]
pub unsafe extern "C" fn unix_add_edges(fpl: *mut scm_fp_list, receiver: *mut unix_sock) {
    void unix_add_edges(struct scm_fp_list *fpl, struct unix_sock *receiver)
    {
    let mut i: c_int = 0, j = 0;
    spin_lock(&unix_gc_lock);
    if (!fpl.count_unix)
    goto out;
    do {
    struct unix_sock *inflight = unix_get_socket(fpl.fp[j++]);
    struct unix_edge *edge;
    if (!inflight)
    continue;
    edge = fpl.edges + i++;
    edge.predecessor = inflight;
    edge.successor = receiver;
    unix_add_edge(fpl, edge);
    } while (i < fpl.count_unix);
    receiver.scm_stat.nr_unix_fds += fpl.count_unix;
    out:
    WRITE_ONCE(fpl.user.unix_inflight, fpl.user.unix_inflight + fpl.count);
    spin_unlock(&unix_gc_lock);
    fpl.inflight = true;
    unix_free_vertices(fpl);
    }
#[no_mangle]
pub unsafe extern "C" fn unix_del_edges(fpl: *mut scm_fp_list) {
    void unix_del_edges(struct scm_fp_list *fpl)
    {
    struct unix_sock *receiver;
    let mut i: c_int = 0;
    spin_lock(&unix_gc_lock);
    if (!fpl.count_unix)
    goto out;
    do {
    struct unix_edge *edge = fpl.edges + i++;
    unix_del_edge(fpl, edge);
    } while (i < fpl.count_unix);
    if (!fpl.dead) {
    receiver = fpl.edges[0].successor;
    receiver.scm_stat.nr_unix_fds -= fpl.count_unix;
    }
    out:
    WRITE_ONCE(fpl.user.unix_inflight, fpl.user.unix_inflight - fpl.count);
    spin_unlock(&unix_gc_lock);
    fpl.inflight = false;
    }
#[no_mangle]
pub unsafe extern "C" fn unix_update_edges(receiver: *mut unix_sock) {
    void unix_update_edges(struct unix_sock *receiver)
    {
// nr_unix_fds is only updated under unix_state_lock().
// If it's 0 here, the embryo socket is not part of the
// inflight graph, and GC will not see it, so no lock needed.
//
    if (!receiver.scm_stat.nr_unix_fds) {
    receiver.listener = core::ptr::null_mut();
    } else {
    spin_lock(&unix_gc_lock);
    unix_update_graph(unix_sk(receiver.listener).vertex);
    receiver.listener = core::ptr::null_mut();
    spin_unlock(&unix_gc_lock);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn unix_prepare_fpl(fpl: *mut scm_fp_list) -> c_int {
    int unix_prepare_fpl(struct scm_fp_list *fpl)
    {
    struct unix_vertex *vertex;
    int i;
    if (!fpl.count_unix)
    return 0;
    for (i = 0; i < fpl.count_unix; i++) {
    vertex = kmalloc_obj(*vertex);
    if (!vertex)
    goto err;
    list_add(&vertex.entry, &fpl.vertices);
    }
    fpl.edges = kvmalloc_objs(*fpl.edges, fpl.count_unix,
    GFP_KERNEL_ACCOUNT);
    if (!fpl.edges)
    goto err;
    unix_schedule_gc(fpl.user);
    return 0;
    err:
    unix_free_vertices(fpl);
    return -ENOMEM;
    }
#[no_mangle]
pub unsafe extern "C" fn unix_destroy_fpl(fpl: *mut scm_fp_list) {
    void unix_destroy_fpl(struct scm_fp_list *fpl)
    {
    if (fpl.inflight)
    unix_del_edges(fpl);
    kvfree(fpl.edges);
    unix_free_vertices(fpl);
    }
    static bool gc_in_progress;
    let mut unix_peek_seq: static seqcount_t = SEQCNT_ZERO(unix_peek_seq);
#[no_mangle]
pub unsafe extern "C" fn unix_peek_fpl(fpl: *mut scm_fp_list) {
    void unix_peek_fpl(struct scm_fp_list *fpl)
    {
    static DEFINE_SPINLOCK(unix_peek_lock);
    if (!fpl || !fpl.count_unix)
    return;
    if (!READ_ONCE(gc_in_progress))
    return;
// Invalidate the final refcnt check in unix_vertex_dead().
    spin_lock(&unix_peek_lock);
    raw_write_seqcount_barrier(&unix_peek_seq);
    spin_unlock(&unix_peek_lock);
    }
#[no_mangle]
unsafe extern "C" fn unix_vertex_dead(vertex: *mut unix_vertex) -> bool {
    static bool unix_vertex_dead(struct unix_vertex *vertex)
    {
    struct unix_edge *edge;
    struct unix_sock *u;
    long total_ref;
    list_for_each_entry(edge, &vertex.edges, vertex_entry) {
    struct unix_vertex *next_vertex = unix_edge_successor(edge);
// The vertex's fd can be received by a non-inflight socket.
    if (!next_vertex)
    return false;
// The vertex's fd can be received by an inflight socket in
// another SCC.
//
    if (next_vertex.scc_index != vertex.scc_index)
    return false;
    }
// No receiver exists out of the same SCC.
    edge = list_first_entry(&vertex.edges, typeof(*edge), vertex_entry);
    u = edge.predecessor;
    total_ref = file_count(u.sk.sk_socket.file);
// If not close()d, total_ref > out_degree.
    if (total_ref != vertex.out_degree)
    return false;
    return true;
    }
    static LIST_HEAD(unix_visited_vertices);
    let mut unix_vertex_grouped_index: static unsigned long = UNIX_VERTEX_INDEX_MARK2;
#[no_mangle]
unsafe extern "C" fn unix_scc_dead(scc: *mut list_head, fast: bool) -> bool {
    static bool unix_scc_dead(struct list_head *scc, bool fast)
    {
    struct unix_vertex *vertex;
    let mut scc_dead: bool = true;
    unsigned int seq;
    seq = read_seqcount_begin(&unix_peek_seq);
    list_for_each_entry_reverse(vertex, scc, scc_entry) {
// Don't restart DFS from this vertex.
    list_move_tail(&vertex.entry, &unix_visited_vertices);
// Mark vertex as off-stack for __unix_walk_scc().
    if (!fast)
    vertex.index = unix_vertex_grouped_index;
    if (scc_dead)
    scc_dead = unix_vertex_dead(vertex);
    }
// If MSG_PEEK intervened, defer this SCC to the next round.
    if (read_seqcount_retry(&unix_peek_seq, seq))
    return false;
    return scc_dead;
    }
#[no_mangle]
unsafe extern "C" fn unix_collect_skb(scc: *mut list_head, hitlist: *mut sk_buff_head) {
    static void unix_collect_skb(struct list_head *scc, struct sk_buff_head *hitlist)
    {
    struct unix_vertex *vertex;
    list_for_each_entry_reverse(vertex, scc, scc_entry) {
    struct sk_buff_head *queue;
    struct unix_edge *edge;
    struct unix_sock *u;
    edge = list_first_entry(&vertex.edges, typeof(*edge), vertex_entry);
    u = edge.predecessor;
    queue = &u.sk.sk_receive_queue;
    spin_lock(&queue.lock);
    if (u.sk.sk_state == TCP_LISTEN) {
    struct sk_buff *skb;
    skb_queue_walk(queue, skb) {
    struct sk_buff_head *embryo_queue = &skb.sk.sk_receive_queue;
    spin_lock(&embryo_queue.lock);
    skb_queue_splice_init(embryo_queue, hitlist);
    spin_unlock(&embryo_queue.lock);
    }
    } else {
    skb_queue_splice_init(queue, hitlist);
    }
    spin_unlock(&queue.lock);
    }
    }
#[no_mangle]
unsafe extern "C" fn unix_scc_cyclic(scc: *mut list_head) -> bool {
    static bool unix_scc_cyclic(struct list_head *scc)
    {
    struct unix_vertex *vertex;
    struct unix_edge *edge;
// SCC containing multiple vertices ?
    if (!list_is_singular(scc))
    return true;
    vertex = list_first_entry(scc, typeof(*vertex), scc_entry);
// Self-reference or a embryo-listener circle ?
    list_for_each_entry(edge, &vertex.edges, vertex_entry) {
    if (unix_edge_successor(edge) == vertex)
    return true;
    }
    return false;
    }
    static unsigned long __unix_walk_scc(struct unix_vertex *vertex,
    unsigned long *last_index,
    struct sk_buff_head *hitlist)
    {
    let mut cyclic_sccs: c_ulong = 0;
    LIST_HEAD(vertex_stack);
    struct unix_edge *edge;
    LIST_HEAD(edge_stack);
    next_vertex:
// Push vertex to vertex_stack and mark it as on-stack
// (index >= UNIX_VERTEX_INDEX_START).
// The vertex will be popped when finalising SCC later.
//
    list_add(&vertex.scc_entry, &vertex_stack);
    vertex.index = *last_index;
    vertex.scc_index = *last_index;
    (*last_index)++;
// Explore neighbour vertices (receivers of the current vertex's fd).
    list_for_each_entry(edge, &vertex.edges, vertex_entry) {
    struct unix_vertex *next_vertex = unix_edge_successor(edge);
    if (!next_vertex)
    continue;
    if (next_vertex.index == unix_vertex_unvisited_index) {
// Iterative deepening depth first search
//
// 1. Push a forward edge to edge_stack and set
// the successor to vertex for the next iteration.
//
    list_add(&edge.stack_entry, &edge_stack);
    vertex = next_vertex;
    goto next_vertex;
// 2. Pop the edge directed to the current vertex
// and restore the ancestor for backtracking.
//
    prev_vertex:
    edge = list_first_entry(&edge_stack, typeof(*edge), stack_entry);
    list_del_init(&edge.stack_entry);
    next_vertex = vertex;
    vertex = edge.predecessor.vertex;
// If the successor has a smaller scc_index, two vertices
// are in the same SCC, so propagate the smaller scc_index
// to skip SCC finalisation.
//
    vertex.scc_index = min(vertex.scc_index, next_vertex.scc_index);
    } else if (next_vertex.index != unix_vertex_grouped_index) {
// Loop detected by a back/cross edge.
//
// The successor is on vertex_stack, so two vertices are in
// the same SCC.  If the successor has a smaller *scc_index*,
// propagate it to skip SCC finalisation.
//
    vertex.scc_index = min(vertex.scc_index, next_vertex.scc_index);
    } else {
// The successor was already grouped as another SCC
    }
    }
    if (vertex.index == vertex.scc_index) {
    struct list_head scc;
// SCC finalised.
//
// If the scc_index was not updated, all the vertices above on
// vertex_stack are in the same SCC.  Group them using scc_entry.
//
    __list_cut_position(&scc, &vertex_stack, &vertex.scc_entry);
    if (unix_scc_dead(&scc, false)) {
    unix_collect_skb(&scc, hitlist);
    } else {
    if (unix_vertex_max_scc_index < vertex.scc_index)
    unix_vertex_max_scc_index = vertex.scc_index;
    if (unix_scc_cyclic(&scc))
    cyclic_sccs++;
    }
    list_del(&scc);
    }
// Need backtracking ?
    if (!list_empty(&edge_stack))
    goto prev_vertex;
    return cyclic_sccs;
    }
    static unsigned long unix_graph_cyclic_sccs;
#[no_mangle]
unsafe extern "C" fn unix_walk_scc(hitlist: *mut sk_buff_head) {
    static void unix_walk_scc(struct sk_buff_head *hitlist)
    {
    let mut last_index: c_ulong = UNIX_VERTEX_INDEX_START;
    let mut cyclic_sccs: c_ulong = 0;
    unix_vertex_max_scc_index = UNIX_VERTEX_INDEX_START;
// Visit every vertex exactly once.
// __unix_walk_scc() moves visited vertices to unix_visited_vertices.
//
    while (!list_empty(&unix_unvisited_vertices)) {
    struct unix_vertex *vertex;
    vertex = list_first_entry(&unix_unvisited_vertices, typeof(*vertex), entry);
    cyclic_sccs += __unix_walk_scc(vertex, &last_index, hitlist);
    }
    list_replace_init(&unix_visited_vertices, &unix_unvisited_vertices);
    swap(unix_vertex_unvisited_index, unix_vertex_grouped_index);
    WRITE_ONCE(unix_graph_cyclic_sccs, cyclic_sccs);
    WRITE_ONCE(unix_graph_state,
    cyclic_sccs ? UNIX_GRAPH_CYCLIC : UNIX_GRAPH_NOT_CYCLIC);
    }
#[no_mangle]
unsafe extern "C" fn unix_walk_scc_fast(hitlist: *mut sk_buff_head) {
    static void unix_walk_scc_fast(struct sk_buff_head *hitlist)
    {
    let mut cyclic_sccs: c_ulong = unix_graph_cyclic_sccs;
    while (!list_empty(&unix_unvisited_vertices)) {
    struct unix_vertex *vertex;
    struct list_head scc;
    vertex = list_first_entry(&unix_unvisited_vertices, typeof(*vertex), entry);
    list_add(&scc, &vertex.scc_entry);
    if (unix_scc_dead(&scc, true)) {
    cyclic_sccs--;
    unix_collect_skb(&scc, hitlist);
    }
    list_del(&scc);
    }
    list_replace_init(&unix_visited_vertices, &unix_unvisited_vertices);
    WRITE_ONCE(unix_graph_cyclic_sccs, cyclic_sccs);
    WRITE_ONCE(unix_graph_state,
    cyclic_sccs ? UNIX_GRAPH_CYCLIC : UNIX_GRAPH_NOT_CYCLIC);
    }
#[no_mangle]
unsafe extern "C" fn unix_gc(work: *mut work_struct) {
    static void unix_gc(struct work_struct *work)
    {
    struct sk_buff_head hitlist;
    struct sk_buff *skb;
    WRITE_ONCE(gc_in_progress, true);
    spin_lock(&unix_gc_lock);
    if (unix_graph_state == UNIX_GRAPH_NOT_CYCLIC) {
    spin_unlock(&unix_gc_lock);
    goto skip_gc;
    }
    __skb_queue_head_init(&hitlist);
    if (unix_graph_state == UNIX_GRAPH_CYCLIC)
    unix_walk_scc_fast(&hitlist);
    else
    unix_walk_scc(&hitlist);
    spin_unlock(&unix_gc_lock);
    skb_queue_walk(&hitlist, skb) {
    if (UNIXCB(skb).fp)
    UNIXCB(skb).fp.dead = true;
    }
    __skb_queue_purge_reason(&hitlist, SKB_DROP_REASON_SOCKET_CLOSE);
    skip_gc:
    WRITE_ONCE(gc_in_progress, false);
    }
    static DECLARE_WORK(unix_gc_work, unix_gc);

#[no_mangle]
pub unsafe extern "C" fn unix_schedule_gc(user: *mut user_struct) {
    void unix_schedule_gc(struct user_struct *user)
    {
    if (READ_ONCE(unix_graph_state) == UNIX_GRAPH_NOT_CYCLIC)
    return;
// Penalise users who want to send AF_UNIX sockets
// but whose sockets have not been received yet.
//
    if (user &&
    READ_ONCE(user.unix_inflight) < UNIX_INFLIGHT_SANE_USER)
    return;
    if (!READ_ONCE(gc_in_progress))
    queue_work(system_dfl_wq, &unix_gc_work);
    if (user && READ_ONCE(unix_graph_cyclic_sccs))
    flush_work(&unix_gc_work);
    }
