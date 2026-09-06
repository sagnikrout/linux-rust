//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/libarena/selftests/test_rbtree.bpf.c
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


// SPDX-License-Identifier: LGPL-2.1 OR BSD-2-Clause

    typedef struct node_ctx __arena *node_ctx;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct node_ctx {
    pub rbnode: rbnode,
    pub next: node_ctx,
}

    static const u64 keys[] = { 51, 43,  37, 3, 301,  46, 383, 990, 776, 729, 871, 96, 189, 213,
    376, 167, 131, 939, 626, 119, 374, 700, 772, 154, 883, 620, 641, 5,
    428, 516, 105, 622, 988, 811, 931, 973, 246, 690, 934, 744, 210, 311,
    32, 255, 960, 830, 523, 429, 541, 738, 705, 774, 715, 446, 98, 578,
    777, 191, 279, 91, 767 };
    static const u64 morekeys[] = { 173, 636, 1201, 8642, 5957, 3617, 4586, 8053, 6551, 7592, 1748, 1589, 8644, 9918, 6977,
    4448, 5852, 4640, 9717, 2303, 7424, 7695, 2334, 8876, 8618, 5745, 7134, 2178, 5280, 2140, 1138,
    5083, 8922, 1516, 2437, 2488, 4307, 4329, 5088, 8456, 5938, 1441, 1684, 5750, 721, 1107, 2089,
    9737, 4687, 5016, 4849, 8193, 9603, 9147, 5992, 166, 6721, 812, 4144, 6237, 6509, 3466, 9255,
    7767, 3960, 6759, 2968, 6046, 9784, 8395, 2619, 1711, 528, 6424, 9084, 3179, 1342, 5676, 9445,
    5691, 6678, 8487, 1627, 998, 6178, 2229, 1987, 3319, 572, 169, 2161, 3018, 5439, 7287, 7265, 5995,
    5003, 5857, 2836, 5634, 4735, 9261, 8287, 5359, 533, 1406, 9573, 4026, 714, 3956, 1722, 6395,
    9648, 3887, 7185, 470, 4482, 4997, 841, 8913, 9946, 3999, 9357, 9847, 277, 8184, 8704, 6766, 3323,
    5468, 8638, 7905, 8858, 6142, 3685, 3452, 4689, 8878, 8836, 158, 831, 7914, 3031, 8374, 4921,
    4207, 3460, 5547, 3358, 1083, 4619, 7818, 2962, 4879, 4583, 2172, 8819, 9830, 1194, 2666, 9812,
    5704, 8432, 5916, 6007, 6609, 4791, 1985, 3226, 2478, 9605, 5236, 8079, 3042, 1965, 3539, 9704,
    4267, 6416, 760, 9968, 2983, 1190, 1964, 3211, 2870, 3106, 2794, 1542, 6916, 5986, 9096, 441,
    5894, 8353, 7765, 3757, 5732, 88, 3091, 5637, 6042, 8447, 4073, 6923, 5491, 7010, 3663, 5029,
    6162, 822, 4874, 7491, 5100, 3461, 6983, 2170, 1458, 1856, 648, 6272, 4887, 976, 2369, 5909, 4274,
    3324, 6968, 2312, 2271, 8891, 6268, 6581, 1610, 8880, 6194, 6144, 9764, 6915, 829, 3774, 2265,
    1752, 1314, 6377, 8760, 8004, 501, 4912, 9278, 1425, 9578, 7337, 307, 1885, 3151, 9617, 1647,
    2458, 3702, 6091, 8902, 5663, 9378, 7640, 3336, 557, 1644, 6848, 1559, 8821, 266, 4330, 9790,
    5920, 4222, 1143, 6248, 5792, 4847, 9726, 6303, 821, 6839, 6062, 7133, 3649, 9888, 2528, 1966,
    5456, 4914, 3615, 1543, 3206, 3353, 6097, 2800, 1424, 9094, 7920, 7243, 1394, 5464, 1707, 576,
    6524, 4261, 4187, 7889, 5336, 3377, 2921, 7244, 2766, 6584, 5514, 1387, 2957, 2258, 1077, 9979,
    1128, 876, 4056, 4668, 4532, 1982, 7093, 4184, 5460, 7588, 4704, 6717, 61, 3959, 1826, 2294, 18,
    8170, 9394, 8796, 7288, 7285, 7143, 148, 6676, 6603, 1051, 8225, 4169, 3230, 7697, 6971, 3454,
    7501, 9514, 394, 2339, 4993, 5606, 6060, 1297, 8273, 3012, 157, 8181, 6765, 7207, 1005, 8833, 1914,
    7456, 1846, 8375, 2741, 2074, 1712, 5286 };
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_rbtree_find_nonexistent() -> __weak int {
    __weak int test_rbtree_find_nonexistent(void)
    {
    let mut key: u64 = 0xdeadbeef;
    let mut value: u64 = 0;
    int ret;
    struct rbtree __arena *rbtree;
    rbtree = rb_create(RB_ALLOC, RB_DEFAULT);
    if (!rbtree)
    return 1;
// Should return -EINVAL
    ret = rb_find(rbtree, key, &value);
    if (!ret)
    return 2;
    return rb_destroy(rbtree);
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_rbtree_insert_existing() -> __weak int {
    __weak int test_rbtree_insert_existing(void)
    {
    let mut key: u64 = 525252;
    let mut value: u64 = 24;
    int ret;
    struct rbtree __arena *rbtree;
    rbtree = rb_create(RB_ALLOC, RB_DEFAULT);
    if (!rbtree)
    return 1;
    ret = rb_insert(rbtree, key, value);
    if (ret)
    return 2;
// Should return -EALREADY.
    ret = rb_insert(rbtree, key, value);
    if (ret != -EALREADY) {
    return 3;
    }
    return rb_destroy(rbtree);
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_rbtree_update_existing() -> __weak int {
    __weak int test_rbtree_update_existing(void)
    {
    let mut key: u64 = 33333;
    u64 value;
    int ret;
    struct rbtree __arena *rbtree;
    rbtree = rb_create(RB_ALLOC, RB_UPDATE);
    if (!rbtree)
    return 1;
    value = 52;
    ret = rb_insert(rbtree, key, value);
    if (ret)
    return 2;
    ret = rb_find(rbtree, key, &value);
    if (ret)
    return 3;
    if (value != 52)
    return 4;
    value = 65;
// Should succeed.
    ret = rb_insert(rbtree, key, value);
    if (ret)
    return 5;
// Should be updated.
    ret = rb_find(rbtree, key, &value);
    if (ret)
    return 6;
    if (value != 65)
    return 7;
    return rb_destroy(rbtree);
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_rbtree_insert_one() -> __weak int {
    __weak int test_rbtree_insert_one(void)
    {
    let mut key: u64 = 202020;
    let mut value: u64 = 0xbadcafe;
    int ret;
    struct rbtree __arena *rbtree;
    rbtree = rb_create(RB_ALLOC, RB_UPDATE);
    if (!rbtree)
    return 1;
    ret = rb_insert(rbtree, key, value);
    if (ret)
    return 2;
    ret = rb_find(rbtree, key, &value);
    if (ret)
    return 3;
    if (value != 0xbadcafe)
    return 4;
    return rb_destroy(rbtree);
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_rbtree_insert_ten() -> __weak int {
    __weak int test_rbtree_insert_ten(void)
    {
    u64 key, value;
    int ret, i;
    struct rbtree __arena *rbtree;
    rbtree = rb_create(RB_ALLOC, RB_UPDATE);
    if (!rbtree)
    return 1;
    for (i = 0; i < 10 && can_loop; i++) {
    key = keys[i];
    ret = rb_insert(rbtree, key, 2 * key);
    if (ret)
    return 2 + 3 * i;
// Read it back.
    ret = rb_find(rbtree, key, &value);
    if (ret)
    return 2 + 3 * i + 1;
    if (value != 2 * key)
    return 2 + 3 * i + 2;
    }
// Go find all inserted pairs.
    for (i = 0; i < 10 && can_loop; i++) {
    key = keys[i];
    ret = rb_find(rbtree, key, &value);
    if (ret)
    return 35 + 2 * i;
    if (value != 2 * key)
    return 35 + 2 * i + 1;
    }
    return rb_destroy(rbtree);
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_rbtree_duplicate() -> __weak int {
    __weak int test_rbtree_duplicate(void)
    {
    let mut key: u64 = 0x121212;
    u64 value;
    int ret, i;
    struct rbtree __arena *rbtree;
    rbtree = rb_create(RB_ALLOC, RB_DUPLICATE);
    if (!rbtree)
    return 1;
    for (i = 0; i < 10 && can_loop; i++) {
    ret = rb_insert(rbtree, key, 2 * key);
    if (ret)
    return 2 + 3 * i;
// Read it back.
    ret = rb_find(rbtree, key, &value);
    if (ret)
    return 2 + 3 * i + 1;
    if (value != 2 * key)
    return 2 + 3 * i + 2;
    }
// Go find all inserted copies and remove them.
    for (i = 0; i < 10 && can_loop; i++) {
    ret = rb_find(rbtree, key, &value);
    if (ret) {
    rb_print(rbtree);
    return 35 + 3 * i;
    }
    if (value != 2 * key)
    return 35 + 3 * i + 1;
    ret = rb_remove(rbtree, key);
    if (ret)
    return 35 + 3 * i + 2;
    }
    return rb_destroy(rbtree);
    }
    static inline int
    clean_up_noalloc_tree(struct rbtree __arena *rbtree)
    {
    node_ctx nodec;
    int ret;
    if (rbtree.alloc != RB_NOALLOC)
    return -EINVAL;
// Can't destroy an RB_NOALLOC tree that still has nodes.
    if (rb_destroy(rbtree) != -EBUSY)
    return -EINVAL;
    while (rbtree.root && can_loop) {
    nodec = (node_ctx)arena_container_of(rbtree.root, struct node_ctx, rbnode);
    ret = rb_remove_node(rbtree, &nodec.rbnode);
    if (ret)
    return ret;
    arena_free(nodec);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn insert_many(alloc: enum rbtree_alloc, insert: enum rbtree_insert_mode) -> c_int {
    int insert_many(enum rbtree_alloc alloc, enum rbtree_insert_mode insert)
    {
    let mut numkeys: usize = sizeof(keys) / sizeof(keys[0]);
    node_ctx nodec;
    u64 key, value;
    int ret;
    int i;
    struct rbtree __arena *rbtree;
    rbtree = rb_create(alloc, insert);
    if (!rbtree)
    return 1;
    for (i = 0; i < numkeys && can_loop; i++) {
    key = keys[i];
    if (rbtree.alloc != RB_ALLOC) {
    nodec = arena_malloc(sizeof(*nodec));
    if (!nodec) {
    arena_stderr("out of memory\n");
    return -ENOMEM;
    }
    nodec.rbnode.key = key;
    nodec.rbnode.value = 2 * key;
    ret = rb_insert_node(rbtree, &nodec.rbnode);
    } else {
    ret = rb_insert(rbtree, key, 2 * key);
    }
    if (ret)
    return 2 + 3 * i;
// Read it back.
    ret = rb_find(rbtree, key, &value);
    if (ret)
    return 2 + 3 * i + 1;
    if (value != 2 * key)
    return 2 + 3 * i + 2;
    }
// Go find all inserted pairs.
    for (i = 0; i < numkeys && can_loop; i++) {
    key = keys[i];
    ret = rb_find(rbtree, key, &value);
    if (ret)
    return 302 + 2 * i;
    if (value != 2 * key)
    return 302 + 2 * i + 1;
    }
// RB_ALLOC trees are destroyed while still having elements.
    if (rbtree.alloc == RB_ALLOC)
    return rb_destroy(rbtree);
// Otherwise manually clean up the tree.
    if (clean_up_noalloc_tree(rbtree))
    return 5;
    return rb_destroy(rbtree);
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_rbtree_remove_one() -> __weak int {
    __weak int test_rbtree_remove_one(void)
    {
    let mut key: u64 = 20, value = 5, newvalue;
    int ret;
    struct rbtree __arena *rbtree;
    rbtree = rb_create(RB_ALLOC, RB_DEFAULT);
    if (!rbtree)
    return 1;
    ret = rb_find(rbtree, key, &newvalue);
    if (!ret)
    return 2;
    ret = rb_insert(rbtree, key, value);
    if (ret)
    return 3;
    ret = rb_find(rbtree, key, &newvalue);
    if (ret || value != newvalue)
    return 4;
    ret = rb_remove(rbtree, key);
    if (ret)
    return 5;
    ret = rb_find(rbtree, key, &newvalue);
    if (!ret)
    return 6;
    return rb_destroy(rbtree);
    }
#[no_mangle]
unsafe extern "C" fn remove_many_verify_all_present(rbtree: *mut rbtree __arena) -> __always_inline int {
    static __always_inline int remove_many_verify_all_present(struct rbtree __arena *rbtree)
    {
    let mut numkeys: usize = sizeof(morekeys) / sizeof(morekeys[0]);
    u64 value;
    int ret;
    int i;
    for (i = 0; i < numkeys && can_loop; i++) {
    let mut key: u64 = morekeys[i];
    ret = rb_find(rbtree, key, &value);
    if (ret)
    return -1;
    if (value != 2 * key)
    return -1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn remove_many_verify_remaining(rbtree: *mut rbtree __arena) -> __always_inline int {
    static __always_inline int remove_many_verify_remaining(struct rbtree __arena *rbtree)
    {
    let mut numkeys: usize = sizeof(morekeys) / sizeof(morekeys[0]);
    u64 value;
    int ret;
    int i;
    for (i = 0; i < numkeys && can_loop; i += 2) {
    let mut key: u64 = morekeys[i];
    ret = rb_find(rbtree, key, &value);
    if (!ret)
    return -1;
    if (i + 1 >= numkeys)
    break;
    key = morekeys[i + 1];
    ret = rb_find(rbtree, key, &value);
    if (ret)
    return -1;
    if (value != 2 * key)
    return -1;
    }
    for (i = 1; i < numkeys && can_loop; i += 2) {
    let mut key: u64 = morekeys[i];
    ret = rb_find(rbtree, key, &value);
    if (ret)
    return -1;
    if (value != 2 * key)
    return -1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn remove_many_alloc(rbtree: *mut rbtree __arena) -> __noinline int {
    static __noinline int remove_many_alloc(struct rbtree __arena *rbtree)
    {
    let mut numkeys: usize = sizeof(morekeys) / sizeof(morekeys[0]);
    u64 value;
    int ret;
    int i;
    for (i = 0; i < numkeys && can_loop; i++) {
    let mut key: u64 = morekeys[i];
    ret = rb_insert(rbtree, key, 2 * key);
    if (ret)
    return -1;
    if (rb_integrity_check(rbtree)) {
    arena_stderr("iteration %d\n", i);
    return -EINVAL;
    }
    ret = rb_find(rbtree, key, &value);
    if (ret)
    return -1;
    if (value != 2 * key)
    return -1;
    }
    ret = remove_many_verify_all_present(rbtree);
    if (ret)
    return ret;
    for (i = 0; i < numkeys && can_loop; i += 2) {
    let mut key: u64 = morekeys[i];
    ret = rb_remove(rbtree, key);
    if (ret) {
    arena_stderr("Failed to remove %ld\n", key);
    return -1;
    }
    ret = rb_find(rbtree, key, &value);
    if (!ret)
    return -1;
    }
    return remove_many_verify_remaining(rbtree);
    }
#[no_mangle]
unsafe extern "C" fn remove_many_noalloc(rbtree: *mut rbtree __arena) -> __noinline int {
    static __noinline int remove_many_noalloc(struct rbtree __arena *rbtree)
    {
    let mut numkeys: usize = sizeof(morekeys) / sizeof(morekeys[0]);
    let mut first: node_ctx = core::ptr::null_mut(), last = core::ptr::null_mut();
    u64 value;
    int ret;
    int i;
    for (i = 0; i < numkeys && can_loop; i++) {
    let mut key: u64 = morekeys[i];
    let mut nodec: node_ctx = arena_malloc(sizeof(*nodec));
    if (!nodec) {
    arena_stderr("out of memory\n");
    return -ENOMEM;
    }
    nodec.rbnode.key = key;
    nodec.rbnode.value = 2 * key;
    nodec.next = core::ptr::null_mut();
    if (!first)
    first = nodec;
    if (last)
    last.next = nodec;
    last = nodec;
    ret = rb_insert_node(rbtree, &nodec.rbnode);
    if (ret)
    return -1;
    if (rb_integrity_check(rbtree)) {
    arena_stderr("iteration %d\n", i);
    return -EINVAL;
    }
    ret = rb_find(rbtree, key, &value);
    if (ret)
    return -1;
    if (value != 2 * key)
    return -1;
    }
    ret = remove_many_verify_all_present(rbtree);
    if (ret)
    return ret;
    for (i = 0; i < numkeys && can_loop; i += 2) {
    let mut key: u64 = morekeys[i];
    let mut nodec: node_ctx = first;
    if (!nodec || key != nodec.rbnode.key)
    return -1;
    first = nodec.next ? nodec.next.next : core::ptr::null_mut();
    ret = rb_remove_node(rbtree, &nodec.rbnode);
    if (ret) {
    arena_stderr("Failed to remove %ld\n", key);
    return -1;
    }
    ret = rb_find(rbtree, key, &value);
    if (!ret)
    return -1;
    }
    return remove_many_verify_remaining(rbtree);
    }
    static inline int remove_many(enum rbtree_alloc alloc,
    enum rbtree_insert_mode insert)
    {
    int ret;
    struct rbtree __arena *rbtree;
    rbtree = rb_create(alloc, insert);
    if (!rbtree)
    return -ENOMEM;
    ret = (alloc == RB_ALLOC) ? remove_many_alloc(rbtree)
    : remove_many_noalloc(rbtree);
    if (ret)
    return ret;
    if (alloc == RB_ALLOC)
    return rb_destroy(rbtree);
    ret = clean_up_noalloc_tree(rbtree);
    if (ret)
    return ret;
    return rb_destroy(rbtree);
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_rbtree_insert_many_update() -> __weak int {
    __weak int test_rbtree_insert_many_update(void)
    {
    return insert_many(RB_ALLOC, RB_UPDATE);
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_rbtree_insert_many_noalloc() -> __weak int {
    __weak int test_rbtree_insert_many_noalloc(void)
    {
    return insert_many(RB_NOALLOC, RB_DUPLICATE);
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_rbtree_remove_many_update() -> __weak int {
    __weak int test_rbtree_remove_many_update(void)
    {
    return remove_many(RB_ALLOC, RB_UPDATE);
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_rbtree_remove_many_noalloc() -> __weak int {
    __weak int test_rbtree_remove_many_noalloc(void)
    {
    return remove_many(RB_NOALLOC, RB_DUPLICATE);
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_rbtree_add_remove_circular() -> __weak int {
    __weak int test_rbtree_add_remove_circular(void)
    {
    let mut iters: usize = 60;
    let mut prefill: usize = 10;
    let mut numkeys: usize = 50;
    let mut prefix: usize = 400000;
    u64 value, rmval;
    let mut errval: c_int = 1;
    u64 key;
    int ret;
    int i;
    struct rbtree __arena *rbtree;
    rbtree = rb_create(RB_ALLOC, RB_UPDATE);
    if (!rbtree)
    return 1;
    for (i = 0; i < prefill && can_loop; i++) {
    ret = rb_insert(rbtree, prefix + (i % numkeys), i);
    if (ret)
    return errval;
    errval += 1;
    }
    errval = 2 * 1000 * 1000;
    for (i = 0; i < prefill && can_loop; i++) {
// Read it back.
    ret = rb_find(rbtree, prefix + (i % numkeys), &value);
    if (ret)
    return errval;
    if (value != i)
    return errval;
    }
    errval = 3 * 1000 * 1000;
    for (i = prefill; i < iters && can_loop; i++) {
    key = prefix + (i % numkeys);
    ret = rb_find(rbtree, key, &value);
    if (!ret) {
    arena_stderr("Key %d already present\n", key);
    return errval;
    }
    errval += 1;
    ret = rb_insert(rbtree, key, i);
    if (ret) {
    arena_stderr("ITERATION %d\n", i);
    rb_print(rbtree);
    return errval;
    }
    rmval = i - prefill;
    errval += 1;
    ret = rb_find(rbtree, prefix + (rmval % numkeys), &value);
    if (ret)
    return errval;
    errval += 1;
    if (value != rmval)
    return errval;
    errval += 1;
    ret = rb_remove(rbtree, prefix + (rmval % numkeys));
    if (ret) {
    arena_stderr("ITERATION %d\n", i);
    return errval;
    }
    errval += 1;
    }
    for (i = 0; i < numkeys && can_loop; i++) {
    rb_remove(rbtree, prefix + i);
    }
    return rb_destroy(rbtree);
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_rbtree_add_remove_circular_reverse() -> __weak int {
    __weak int test_rbtree_add_remove_circular_reverse(void)
    {
    let mut iters: usize = 110;
    let mut prefill: usize = 10;
    let mut numkeys: usize = 50;
    let mut prefix: usize = 500000;
    u64 value, rmval;
    let mut errval: c_int = 1;
    u64 key;
    int ret;
    int i;
    struct rbtree __arena *rbtree;
    rbtree = rb_create(RB_ALLOC, RB_UPDATE);
    if (!rbtree)
    return 1;
    for (i = 0; i < prefill && can_loop; i++) {
    ret = rb_insert(rbtree, prefix - (i % numkeys), i);
    if (ret)
    return errval;
    errval += 1;
    }
    errval = 2 * 1000 * 1000;
    for (i = 0; i < prefill && can_loop; i++) {
// Read it back.
    ret = rb_find(rbtree, prefix - (i % numkeys), &value);
    if (ret)
    return errval;
    if (value != i)
    return errval;
    }
    errval = 3 * 1000 * 1000;
    for (i = prefill; i < iters && can_loop; i++) {
    key = prefix - (i % numkeys);
    ret = rb_find(rbtree, key, &value);
    if (!ret) {
    arena_stderr("Key %d already present\n", key);
    return errval;
    }
    errval += 1;
    ret = rb_insert(rbtree, key, i);
    if (ret) {
    arena_stderr("error %d on insert\n", ret);
    rb_print(rbtree);
    return errval;
    }
    rmval = i - prefill;
    errval += 1;
    ret = rb_find(rbtree, prefix - (rmval % numkeys), &value);
    if (ret)
    return errval;
    errval += 1;
    if (value != rmval)
    return errval;
    errval += 1;
    ret = rb_remove(rbtree, prefix - (rmval % numkeys));
    if (ret)
    return errval;
    errval += 1;
    }
    errval = 4 * 1000 * 1000;
    for (i = 0; i < prefill && can_loop; i++) {
    ret = rb_remove(rbtree, prefix - i);
    if (ret) {
    arena_stderr("Did not remove %d, error %d\n", prefix - i, ret);
    return errval + i;
    }
    }
    return rb_destroy(rbtree);
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_rbtree_least_pop() -> __weak int {
    __weak int test_rbtree_least_pop(void)
    {
    let mut keys: usize = 10;
    u64 key, value;
    let mut errval: c_int = 1;
    int ret, i;
    struct rbtree __arena *rbtree;
    rbtree = rb_create(RB_ALLOC, RB_DEFAULT);
    if (!rbtree)
    return errval;
    errval += 1;
    for (i = 0; i < keys / 2 && can_loop; i++) {
    ret = rb_insert(rbtree, i, i);
    if (ret)
    return errval;
    errval += 1;
    ret = rb_insert(rbtree, keys - 1 - i, keys - 1 - i);
    if (ret)
    return errval;
    errval += 1;
    ret = rb_least(rbtree, &key, &value);
    if (ret)
    return errval;
    errval += 1;
    if (key != 0 || value != 0)
    return errval;
    errval += 1;
    }
    errval = 1000;
    for (i = 0; i < keys && can_loop; i++) {
    ret = rb_least(rbtree, &key, &value);
    if (ret) {
    arena_stderr("rb_least failed with %d\n", ret);
    return errval;
    }
    errval += 1;
    if (key != i || value != i) {
    arena_stderr("Got KV %ld/%ld expected %d\n", key, value, i);
    return errval;
    }
    errval += 1;
    ret = rb_pop(rbtree, &key, &value);
    if (ret) {
    arena_stderr("Error %d during pop on iter %d\n", ret, i);
    return errval;
    }
    errval += 1;
    if (key != i || value != i)
    return errval;
    }
    return rb_destroy(rbtree);
    }
// Reject rb_pop() for RB_NOALLOC trees.
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_rbtree_noalloc_pop() -> __weak int {
    __weak int test_rbtree_noalloc_pop(void)
    {
    let mut expect_value: u64 = 1;
    let mut expect_key: u64 = 0;
    struct rbtree __arena *rbtree;
    struct rbnode __arena *node;
    let mut value: u64 = 0;
    int ret;
    rbtree = rb_create(RB_NOALLOC, RB_DEFAULT);
    if (!rbtree)
    return 1;
    node = rb_node_alloc(expect_key, expect_value);
    if (!node) {
    rb_destroy(rbtree);
    return 2;
    }
    ret = rb_insert_node(rbtree, node);
    if (ret) {
    rb_node_free(node);
    rb_destroy(rbtree);
    return 3;
    }
    ret = rb_pop(rbtree, core::ptr::null_mut(), &value);
    if (ret != -EINVAL)
    return 4;
    ret = rb_find(rbtree, expect_key, &value);
    if (ret)
    return 5;
    if (value != expect_value)
    return 6;
    ret = rb_remove_node(rbtree, node);
    if (ret)
    return 7;
    rb_node_free(node);
    return rb_destroy(rbtree);
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_rbtree_alloc_check() -> __weak int {
    __weak int test_rbtree_alloc_check(void)
    {
    struct rbtree __arena *alloc, *noalloc;
    struct rbnode __arena *node;
    int ret;
    alloc = rb_create(RB_ALLOC, RB_DEFAULT);
    if (!alloc)
    return 1;
    noalloc = rb_create(RB_NOALLOC, RB_DEFAULT);
    if (!noalloc)
    return 2;
    node = rb_node_alloc(0, 0);
    if (!node)
    return 3;
//
// RB_ALLOC trees can use rb_insert, RB_NOALLOC trees can
// use rb_insert_node. RB_ALLOC and RB_NOALLOC trees cannot
// use each other's APIs.
//
// NOTE: This begs the question, why not different types? We
// want to partially share the API and that would require us
// to duplicate it.
//
    if (rb_insert(alloc, 0, 0))
    return 4;
    if (!rb_insert_node(alloc, node))
    return 5;
    if (!rb_remove_node(alloc, node))
    return 6;
    if (rb_remove(alloc, 0))
    return 7;
    if (rb_insert_node(noalloc, node))
    return 8;
    if (!rb_insert(noalloc, 0, 0))
    return 9;
    if (!rb_remove(noalloc, 0))
    return 10;
    if (rb_remove_node(noalloc, node))
    return 11;
    rb_node_free(node);
    ret = rb_destroy(alloc);
    if (ret)
    return ret;
    return rb_destroy(noalloc);
    }
