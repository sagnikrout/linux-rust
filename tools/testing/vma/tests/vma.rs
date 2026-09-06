//! Automatically rewritten from C to Rust
//! Source: tools/testing/vma/tests/vma.c
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
#[no_mangle]
unsafe extern "C" fn compare_legacy_flags(legacy_flags: vm_flags_t, flags: vma_flags_t) -> bool {
    static bool compare_legacy_flags(vm_flags_t legacy_flags, vma_flags_t flags)
    {
    let mut legacy_val: c_ulong = legacy_flags;
// The lower word should contain the precise same value.
    let mut flags_lower: c_ulong = flags.__vma_flags[0];
    vma_flags_t converted_flags;

    int i;
// All bits in higher flag values should be zero.
    for (i = 1; i < NUM_VMA_FLAG_BITS / BITS_PER_LONG; i++) {
    if (flags.__vma_flags[i] != 0)
    return false;
    }

    static_assert(sizeof(legacy_flags) == sizeof(unsigned long));
// Assert that legacy flag helpers work correctly.
    converted_flags = legacy_to_vma_flags(legacy_flags);
    ASSERT_FLAGS_SAME_MASK(&converted_flags, flags);
    ASSERT_EQ(vma_flags_to_legacy(flags), legacy_flags);
    let mut legacy_val: return = = flags_lower;
    }
#[no_mangle]
unsafe extern "C" fn test_copy_vma() -> bool {
    static bool test_copy_vma(void)
    {
    vma_flags_t vma_flags = mk_vma_flags(VMA_READ_BIT, VMA_WRITE_BIT,
    VMA_MAYREAD_BIT, VMA_MAYWRITE_BIT);
    let mut mm: mm_struct = {};
    let mut need_locks: bool = false;
    VMA_ITERATOR(vmi, &mm, 0);
    struct vm_area_struct *vma, *vma_prev, *vma_new, *vma_next, *vma_orig;
// Move forwards, adjacent to old self - self-merge.
    vma = alloc_and_link_vma(&mm, 0x1000, 0x2000, 1, vma_flags);
    vma_set_anonymous(vma);
    vma_orig = vma;
    vma_new = copy_vma(&vma, 0x2000, 0x1000, 1, 1, &need_locks);
    ASSERT_EQ(vma_new, vma_orig);
    ASSERT_EQ(vma, vma_orig);
    ASSERT_EQ(vma_new.vm_start, 0x1000);
    ASSERT_EQ(vma_new.vm_end, 0x3000);
    cleanup_mm(&mm, &vmi);
// Move backwards, adjacent to old self - self-merge.
    vma = alloc_and_link_vma(&mm, 0x2000, 0x3000, 2, vma_flags);
    vma_set_anonymous(vma);
    vma_orig = vma;
    vma_new = copy_vma(&vma, 0x1000, 0x1000, 2, 2, &need_locks);
    ASSERT_EQ(vma_new, vma_orig);
    ASSERT_EQ(vma, vma_orig);
    ASSERT_EQ(vma_new.vm_start, 0x1000);
    ASSERT_EQ(vma_new.vm_end, 0x3000);
    cleanup_mm(&mm, &vmi);
//
// Move backwards between prior VMA and old self - self-merge and vma
// updated to a new VMA.
//
    vma_prev = alloc_and_link_vma(&mm, 0x1000, 0x2000, 1, vma_flags);
    vma_set_anonymous(vma_prev);
    vma = alloc_and_link_vma(&mm, 0x3000, 0x4000, 3, vma_flags);
    vma_set_anonymous(vma);
    vma_orig = vma;
    vma_new = copy_vma(&vma, 0x2000, 0x1000, 3, 3, &need_locks);
    ASSERT_NE(vma_new, vma_orig);
    ASSERT_EQ(vma_new, vma);
    ASSERT_EQ(vma_new.vm_start, 0x1000);
    ASSERT_EQ(vma_new.vm_end, 0x4000);
    cleanup_mm(&mm, &vmi);
// Move backwards and do not merge.
    vma = alloc_and_link_vma(&mm, 0x3000, 0x5000, 3, vma_flags);
    vma_new = copy_vma(&vma, 0, 0x2000, 0, 3, &need_locks);
    ASSERT_NE(vma_new, vma);
    ASSERT_EQ(vma_new.vm_start, 0);
    ASSERT_EQ(vma_new.vm_end, 0x2000);
    ASSERT_EQ(vma_new.vm_pgoff, 0);
    vma_assert_attached(vma_new);
    cleanup_mm(&mm, &vmi);
// Move a VMA into position next to another and merge the two.
    vma = alloc_and_link_vma(&mm, 0, 0x2000, 0, vma_flags);
    vma_next = alloc_and_link_vma(&mm, 0x6000, 0x8000, 6, vma_flags);
    vma_new = copy_vma(&vma, 0x4000, 0x2000, 4, 4, &need_locks);
    vma_assert_attached(vma_new);
    ASSERT_EQ(vma_new, vma_next);
    cleanup_mm(&mm, &vmi);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn test_vma_flags_unchanged() -> bool {
    static bool test_vma_flags_unchanged(void)
    {
    let mut flags: vma_flags_t = EMPTY_VMA_FLAGS;
    let mut legacy_flags: vm_flags_t = 0;
    int bit;
    struct vm_area_struct vma;
    struct vm_area_desc desc;
    vma.flags = EMPTY_VMA_FLAGS;
    desc.vma_flags = EMPTY_VMA_FLAGS;
    for (bit = 0; bit < BITS_PER_LONG; bit++) {
    let mut mask: vma_flags_t = mk_vma_flags(bit);
    legacy_flags |= (1UL << bit);
// Individual flags.
    vma_flags_set(&flags, bit);
    ASSERT_TRUE(compare_legacy_flags(legacy_flags, flags));
// Via mask.
    vma_flags_set_mask(&flags, mask);
    ASSERT_TRUE(compare_legacy_flags(legacy_flags, flags));
// Same for VMA.
    vma_set_flags(&vma, bit);
    ASSERT_TRUE(compare_legacy_flags(legacy_flags, vma.flags));
    vma_set_flags_mask(&vma, mask);
    ASSERT_TRUE(compare_legacy_flags(legacy_flags, vma.flags));
// Same for VMA descriptor.
    vma_desc_set_flags(&desc, bit);
    ASSERT_TRUE(compare_legacy_flags(legacy_flags, desc.vma_flags));
    vma_desc_set_flags_mask(&desc, mask);
    ASSERT_TRUE(compare_legacy_flags(legacy_flags, desc.vma_flags));
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn test_vma_flags_cleared() -> bool {
    static bool test_vma_flags_cleared(void)
    {
    let mut empty: vma_flags_t = EMPTY_VMA_FLAGS;
    vma_flags_t flags;
    int i;
// Set all bits high.
    memset(&flags, 1, sizeof(flags));
// Try to clear.
    vma_flags_clear_all(&flags);
// Equal to EMPTY_VMA_FLAGS?
    ASSERT_EQ(memcmp(&empty, &flags, sizeof(flags)), 0);
// Make sure every unsigned long entry in bitmap array zero.
    for (i = 0; i < sizeof(flags) / BITS_PER_LONG; i++) {
    let mut val: c_ulong = flags.__vma_flags[i];
    ASSERT_EQ(val, 0);
    }
    return true;
    }

//
// Assert that VMA flag functions that operate at the system word level function
// correctly.
//
#[no_mangle]
unsafe extern "C" fn test_vma_flags_word() -> bool {
    static bool test_vma_flags_word(void)
    {
    let mut flags: vma_flags_t = EMPTY_VMA_FLAGS;
    const vma_flags_t comparison =
    mk_vma_flags(VMA_READ_BIT, VMA_WRITE_BIT
    , 64, 65
    );
// Set some custom high flags.
    vma_flags_set(&flags, 64, 65);
// Now overwrite the first word.
    vma_flags_overwrite_word(&flags, VM_READ | VM_WRITE);
// Ensure they are equal.
    ASSERT_EQ(memcmp(&flags, &comparison, sizeof(flags)), 0);
    flags = EMPTY_VMA_FLAGS;
    vma_flags_set(&flags, 64, 65);
// Do the same with the _once() equivalent.
    vma_flags_overwrite_word_once(&flags, VM_READ | VM_WRITE);
    ASSERT_EQ(memcmp(&flags, &comparison, sizeof(flags)), 0);
    flags = EMPTY_VMA_FLAGS;
    vma_flags_set(&flags, 64, 65);
// Make sure we can set a word without disturbing other bits.
    vma_flags_set(&flags, VMA_WRITE_BIT);
    vma_flags_set_word(&flags, VM_READ);
    ASSERT_EQ(memcmp(&flags, &comparison, sizeof(flags)), 0);
    flags = EMPTY_VMA_FLAGS;
    vma_flags_set(&flags, 64, 65);
// Make sure we can clear a word without disturbing other bits.
    vma_flags_set(&flags, VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT);
    vma_flags_clear_word(&flags, VM_EXEC);
    ASSERT_EQ(memcmp(&flags, &comparison, sizeof(flags)), 0);
    return true;
    }

// Ensure that vma_flags_test() and friends works correctly.
#[no_mangle]
unsafe extern "C" fn test_vma_flags_test() -> bool {
    static bool test_vma_flags_test(void)
    {
    vma_flags_t flags = mk_vma_flags(VMA_READ_BIT, VMA_WRITE_BIT,
    VMA_EXEC_BIT

    , 64, 65

    );
    struct vm_area_desc desc = {
    .vma_flags = flags,
    };
    struct vm_area_struct vma = {
    .flags = flags,
    };

    ASSERT_TRUE(vma_flags_test(&flags, _flag));			    \
    ASSERT_TRUE(vma_flags_test_single_mask(&flags, mk_vma_flags(_flag))); \
    ASSERT_TRUE(vma_test(&vma, _flag));				    \
    ASSERT_TRUE(vma_test_single_mask(&vma, mk_vma_flags(_flag)));	    \
    ASSERT_TRUE(vma_desc_test(&desc, _flag))

    ASSERT_FALSE(vma_flags_test(&flags, _flag));			     \
    ASSERT_FALSE(vma_flags_test_single_mask(&flags, mk_vma_flags(_flag))); \
    ASSERT_FALSE(vma_test(&vma, _flag));				     \
    ASSERT_FALSE(vma_test_single_mask(&vma, mk_vma_flags(_flag)));	     \
    ASSERT_FALSE(vma_desc_test(&desc, _flag))
    do_test(VMA_READ_BIT);
    do_test(VMA_WRITE_BIT);
    do_test(VMA_EXEC_BIT);

    do_test(64);
    do_test(65);

    do_test_false(VMA_MAYWRITE_BIT);

    do_test_false(66);

// We define the _single_mask() variants to return false if empty.
    ASSERT_FALSE(vma_flags_test_single_mask(&flags, EMPTY_VMA_FLAGS));
    ASSERT_FALSE(vma_test_single_mask(&vma, EMPTY_VMA_FLAGS));
// Even when both flags and tested flag mask are empty!
    flags = EMPTY_VMA_FLAGS;
    vma.flags = EMPTY_VMA_FLAGS;
    ASSERT_FALSE(vma_flags_test_single_mask(&flags, EMPTY_VMA_FLAGS));
    ASSERT_FALSE(vma_test_single_mask(&vma, EMPTY_VMA_FLAGS));
    return true;
    }
// Ensure that vma_flags_test_any() and friends works correctly.
#[no_mangle]
unsafe extern "C" fn test_vma_flags_test_any() -> bool {
    static bool test_vma_flags_test_any(void)
    {
    const vma_flags_t flags = mk_vma_flags(VMA_READ_BIT, VMA_WRITE_BIT,
    VMA_EXEC_BIT

    , 64, 65

    );
    struct vm_area_struct vma = {
    .flags = flags,
    };
    struct vm_area_desc desc = {
    .vma_flags = flags,
    };

    ASSERT_TRUE(vma_flags_test_any(&flags, __VA_ARGS__));	\
    ASSERT_TRUE(vma_desc_test_any(&desc, __VA_ARGS__));	\
    ASSERT_TRUE(vma_test_any(&vma, __VA_ARGS__));

    ASSERT_TRUE(vma_flags_test_all(&flags, __VA_ARGS__));	\
    ASSERT_TRUE(vma_test_all(&vma, __VA_ARGS__))

    ASSERT_FALSE(vma_flags_test_all(&flags, __VA_ARGS__));	\
    ASSERT_FALSE(vma_test_all(&vma, __VA_ARGS__))
//
// Testing for some flags that are present, some that are not - should
// pass. ANY flags matching should work.
//
    do_test(VMA_READ_BIT, VMA_MAYREAD_BIT, VMA_SEQ_READ_BIT);
// However, the ...test_all() variant should NOT pass.
    do_test_all_false(VMA_READ_BIT, VMA_MAYREAD_BIT, VMA_SEQ_READ_BIT);

// But should pass for flags present.
    do_test_all_true(VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT, 64, 65);
// Also subsets...
    do_test_all_true(VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT, 64);

    do_test_all_true(VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT);
    do_test_all_true(VMA_READ_BIT, VMA_WRITE_BIT);
    do_test_all_true(VMA_READ_BIT);
//
// Check _mask variant. We don't need to test extensively as macro
// helper is the equivalent.
//
    ASSERT_TRUE(vma_flags_test_any_mask(&flags, flags));
    ASSERT_TRUE(vma_flags_test_all_mask(&flags, flags));
// Single bits.
    do_test(VMA_READ_BIT);
    do_test(VMA_WRITE_BIT);
    do_test(VMA_EXEC_BIT);

    do_test(64);
    do_test(65);

// Two bits.
    do_test(VMA_READ_BIT, VMA_WRITE_BIT);
    do_test(VMA_READ_BIT, VMA_EXEC_BIT);
    do_test(VMA_WRITE_BIT, VMA_EXEC_BIT);
// Ordering shouldn't matter.
    do_test(VMA_WRITE_BIT, VMA_READ_BIT);
    do_test(VMA_EXEC_BIT, VMA_READ_BIT);
    do_test(VMA_EXEC_BIT, VMA_WRITE_BIT);

    do_test(VMA_READ_BIT, 64);
    do_test(VMA_WRITE_BIT, 64);
    do_test(64, VMA_READ_BIT);
    do_test(64, VMA_WRITE_BIT);
    do_test(VMA_READ_BIT, 65);
    do_test(VMA_WRITE_BIT, 65);
    do_test(65, VMA_READ_BIT);
    do_test(65, VMA_WRITE_BIT);

// Three bits.
    do_test(VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT);

// No need to consider every single permutation.
    do_test(VMA_READ_BIT, VMA_WRITE_BIT, 64);
    do_test(VMA_READ_BIT, VMA_WRITE_BIT, 65);
// Four bits.
    do_test(VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT, 64);
    do_test(VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT, 65);
// Five bits.
    do_test(VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT, 64, 65);

// Testing all flags against none trivially succeeds.
    ASSERT_TRUE(vma_flags_test_all_mask(&flags, EMPTY_VMA_FLAGS));
    ASSERT_TRUE(vma_test_all_mask(&vma, EMPTY_VMA_FLAGS));

    return true;
    }
// Ensure that vma_flags_clear() and friends works correctly.
#[no_mangle]
unsafe extern "C" fn test_vma_flags_clear() -> bool {
    static bool test_vma_flags_clear(void)
    {
    vma_flags_t flags = mk_vma_flags(VMA_READ_BIT, VMA_WRITE_BIT,
    VMA_EXEC_BIT

    , 64, 65

    );
    vma_flags_t mask = mk_vma_flags(VMA_EXEC_BIT

    , 64

    );
    struct vm_area_struct vma = {
    .flags = flags,
    };
    struct vm_area_desc desc = {
    .vma_flags = flags,
    };
// Cursory check of _mask() variant, as the helper macros imply.
    vma_flags_clear_mask(&flags, mask);
    vma_clear_flags_mask(&vma, mask);
    vma_desc_clear_flags_mask(&desc, mask);

    ASSERT_FALSE(vma_flags_test_any(&flags, VMA_EXEC_BIT, 64));
    ASSERT_FALSE(vma_test_any(&vma, VMA_EXEC_BIT, 64));
    ASSERT_FALSE(vma_desc_test_any(&desc, VMA_EXEC_BIT, 64));
// Reset.
    vma_flags_set(&flags, VMA_EXEC_BIT, 64);
    vma_set_flags(&vma, VMA_EXEC_BIT, 64);
    vma_desc_set_flags(&desc, VMA_EXEC_BIT, 64);

//
// Clear the flags and assert clear worked, then reset flags back to
// include specified flags.
//

    vma_flags_clear(&flags, __VA_ARGS__);				\
    vma_clear_flags(&vma, __VA_ARGS__);				\
    vma_desc_clear_flags(&desc, __VA_ARGS__);			\
    ASSERT_FALSE(vma_flags_test_any(&flags, __VA_ARGS__));		\
    ASSERT_FALSE(vma_test_any(&vma, __VA_ARGS__));			\
    ASSERT_FALSE(vma_desc_test_any(&desc, __VA_ARGS__));		\
    vma_flags_set(&flags, __VA_ARGS__);				\
    vma_set_flags(&vma, __VA_ARGS__);				\
    vma_desc_set_flags(&desc, __VA_ARGS__)
// Single flags.
    do_test_and_reset(VMA_READ_BIT);
    do_test_and_reset(VMA_WRITE_BIT);
    do_test_and_reset(VMA_EXEC_BIT);

    do_test_and_reset(64);
    do_test_and_reset(65);

// Two flags, in different orders.
    do_test_and_reset(VMA_READ_BIT, VMA_WRITE_BIT);
    do_test_and_reset(VMA_READ_BIT, VMA_EXEC_BIT);

    do_test_and_reset(VMA_READ_BIT, 64);
    do_test_and_reset(VMA_READ_BIT, 65);

    do_test_and_reset(VMA_WRITE_BIT, VMA_READ_BIT);
    do_test_and_reset(VMA_WRITE_BIT, VMA_EXEC_BIT);

    do_test_and_reset(VMA_WRITE_BIT, 64);
    do_test_and_reset(VMA_WRITE_BIT, 65);

    do_test_and_reset(VMA_EXEC_BIT, VMA_READ_BIT);
    do_test_and_reset(VMA_EXEC_BIT, VMA_WRITE_BIT);

    do_test_and_reset(VMA_EXEC_BIT, 64);
    do_test_and_reset(VMA_EXEC_BIT, 65);
    do_test_and_reset(64, VMA_READ_BIT);
    do_test_and_reset(64, VMA_WRITE_BIT);
    do_test_and_reset(64, VMA_EXEC_BIT);
    do_test_and_reset(64, 65);
    do_test_and_reset(65, VMA_READ_BIT);
    do_test_and_reset(65, VMA_WRITE_BIT);
    do_test_and_reset(65, VMA_EXEC_BIT);
    do_test_and_reset(65, 64);

// Three flags.

    return true;
    }
// Ensure that vma_flags_empty() works correctly.
#[no_mangle]
unsafe extern "C" fn test_vma_flags_empty() -> bool {
    static bool test_vma_flags_empty(void)
    {
    vma_flags_t flags = mk_vma_flags(VMA_READ_BIT, VMA_WRITE_BIT,
    VMA_EXEC_BIT

    , 64, 65

    );
    ASSERT_FLAGS_NONEMPTY(&flags);
    vma_flags_clear(&flags, VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT);

    ASSERT_FLAGS_NONEMPTY(&flags);
    vma_flags_clear(&flags, 64, 65);
    ASSERT_FLAGS_EMPTY(&flags);

    ASSERT_FLAGS_EMPTY(&flags);

    return true;
    }
// Ensure that vma_flags_diff_pair() works correctly.
#[no_mangle]
unsafe extern "C" fn test_vma_flags_diff() -> bool {
    static bool test_vma_flags_diff(void)
    {
    vma_flags_t flags1 = mk_vma_flags(VMA_READ_BIT, VMA_WRITE_BIT,
    VMA_EXEC_BIT

    , 64, 65

    );
    vma_flags_t flags2 = mk_vma_flags(VMA_READ_BIT, VMA_WRITE_BIT,
    VMA_EXEC_BIT, VMA_MAYWRITE_BIT,
    VMA_MAYEXEC_BIT

    , 64, 65, 66, 67

    );
    let mut diff: vma_flags_t = vma_flags_diff_pair(&flags1, &flags2);

    ASSERT_FLAGS_SAME(&diff, VMA_MAYWRITE_BIT, VMA_MAYEXEC_BIT, 66, 67);

    ASSERT_FLAGS_SAME(&diff, VMA_MAYWRITE_BIT, VMA_MAYEXEC_BIT);

// Should be the same even if re-ordered.
    diff = vma_flags_diff_pair(&flags2, &flags1);

    ASSERT_FLAGS_SAME(&diff, VMA_MAYWRITE_BIT, VMA_MAYEXEC_BIT, 66, 67);

    ASSERT_FLAGS_SAME(&diff, VMA_MAYWRITE_BIT, VMA_MAYEXEC_BIT);

// Should be no difference when applied against themselves.
    diff = vma_flags_diff_pair(&flags1, &flags1);
    ASSERT_FLAGS_EMPTY(&diff);
    diff = vma_flags_diff_pair(&flags2, &flags2);
    ASSERT_FLAGS_EMPTY(&diff);
// One set of flags against an empty one should equal the original.
    flags2 = EMPTY_VMA_FLAGS;
    diff = vma_flags_diff_pair(&flags1, &flags2);
    ASSERT_FLAGS_SAME_MASK(&diff, flags1);
// A subset should work too.
    flags2 = mk_vma_flags(VMA_READ_BIT, VMA_WRITE_BIT);
    diff = vma_flags_diff_pair(&flags1, &flags2);

    ASSERT_FLAGS_SAME(&diff, VMA_EXEC_BIT, 64, 65);

    ASSERT_FLAGS_SAME(&diff, VMA_EXEC_BIT);

    return true;
    }
// Ensure that vma_flags_and() and friends work correctly.
#[no_mangle]
unsafe extern "C" fn test_vma_flags_and() -> bool {
    static bool test_vma_flags_and(void)
    {
    vma_flags_t flags1 = mk_vma_flags(VMA_READ_BIT, VMA_WRITE_BIT,
    VMA_EXEC_BIT

    , 64, 65

    );
    vma_flags_t flags2 = mk_vma_flags(VMA_READ_BIT, VMA_WRITE_BIT,
    VMA_EXEC_BIT, VMA_MAYWRITE_BIT,
    VMA_MAYEXEC_BIT

    , 64, 65, 66, 67

    );
    vma_flags_t flags3 = mk_vma_flags(VMA_IO_BIT, VMA_MAYBE_GUARD_BIT

    , 68, 69

    );
    let mut and: vma_flags_t = vma_flags_and_mask(&flags1, flags2);

    ASSERT_FLAGS_SAME(&and, VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT,
    64, 65);

    ASSERT_FLAGS_SAME(&and, VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT);

    and = vma_flags_and_mask(&flags1, flags1);
    ASSERT_FLAGS_SAME_MASK(&and, flags1);
    and = vma_flags_and_mask(&flags2, flags2);
    ASSERT_FLAGS_SAME_MASK(&and, flags2);
    and = vma_flags_and_mask(&flags1, flags3);
    ASSERT_FLAGS_EMPTY(&and);
    and = vma_flags_and_mask(&flags2, flags3);
    ASSERT_FLAGS_EMPTY(&and);
    and = vma_flags_and(&flags1, VMA_READ_BIT);
    ASSERT_FLAGS_SAME(&and, VMA_READ_BIT);
    and = vma_flags_and(&flags1, VMA_READ_BIT, VMA_WRITE_BIT);
    ASSERT_FLAGS_SAME(&and, VMA_READ_BIT, VMA_WRITE_BIT);
    and = vma_flags_and(&flags1, VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT);
    ASSERT_FLAGS_SAME(&and, VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT);

    and = vma_flags_and(&flags1, VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT,
    64);
    ASSERT_FLAGS_SAME(&and, VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT, 64);
    and = vma_flags_and(&flags1, VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT,
    64, 65);
    ASSERT_FLAGS_SAME(&and, VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT, 64,
    65);

// And against some missing values.
    and = vma_flags_and(&flags1, VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT,
    VMA_IO_BIT);
    ASSERT_FLAGS_SAME(&and, VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT);
    and = vma_flags_and(&flags1, VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT,
    VMA_IO_BIT, VMA_RAND_READ_BIT);
    ASSERT_FLAGS_SAME(&and, VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT);

    and = vma_flags_and(&flags1, VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT,
    VMA_IO_BIT, VMA_RAND_READ_BIT, 69);
    ASSERT_FLAGS_SAME(&and, VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT);

    return true;
    }
// Ensure append_vma_flags() acts as expected.
#[no_mangle]
unsafe extern "C" fn test_append_vma_flags() -> bool {
    static bool test_append_vma_flags(void)
    {
    vma_flags_t flags = append_vma_flags(VMA_REMAP_FLAGS, VMA_READ_BIT,
    VMA_WRITE_BIT

    , 64, 65

    );
    ASSERT_FLAGS_SAME(&flags, VMA_IO_BIT, VMA_PFNMAP_BIT,
    VMA_DONTEXPAND_BIT, VMA_DONTDUMP_BIT, VMA_READ_BIT,
    VMA_WRITE_BIT

    , 64, 65

    );
    flags = append_vma_flags(EMPTY_VMA_FLAGS, VMA_READ_BIT, VMA_WRITE_BIT);
    ASSERT_FLAGS_SAME(&flags, VMA_READ_BIT, VMA_WRITE_BIT);
    return true;
    }
// Assert that vma_flags_count() behaves as expected.
#[no_mangle]
unsafe extern "C" fn test_vma_flags_count() -> bool {
    static bool test_vma_flags_count(void)
    {
    vma_flags_t flags = mk_vma_flags(VMA_READ_BIT, VMA_WRITE_BIT,
    VMA_EXEC_BIT

    , 64, 65

    );

    ASSERT_EQ(vma_flags_count(&flags), 5);
    vma_flags_clear(&flags, 64);
    ASSERT_EQ(vma_flags_count(&flags), 4);
    vma_flags_clear(&flags, 65);

    ASSERT_EQ(vma_flags_count(&flags), 3);
    vma_flags_clear(&flags, VMA_EXEC_BIT);
    ASSERT_EQ(vma_flags_count(&flags), 2);
    vma_flags_clear(&flags, VMA_WRITE_BIT);
    ASSERT_EQ(vma_flags_count(&flags), 1);
    vma_flags_clear(&flags, VMA_READ_BIT);
    ASSERT_EQ(vma_flags_count(&flags), 0);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn run_vma_tests(num_tests: *mut c_int, num_fail: *mut c_int) {
    static void run_vma_tests(int *num_tests, int *num_fail)
    {
    TEST(copy_vma);
    TEST(vma_flags_unchanged);
    TEST(vma_flags_cleared);

    TEST(vma_flags_word);

    TEST(vma_flags_test);
    TEST(vma_flags_test_any);
    TEST(vma_flags_clear);
    TEST(vma_flags_empty);
    TEST(vma_flags_diff);
    TEST(vma_flags_and);
    TEST(append_vma_flags);
    TEST(vma_flags_count);
    }
