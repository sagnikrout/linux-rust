//! Automatically rewritten from C Header to Rust Module
//! Source: mm/damon/tests/core-kunit.h
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
// Data Access Monitor Unit Tests
//

//
// Test kdamond_reset_aggregated()
//
// DAMON checks access to each region and aggregates this information as the
// access frequency of each region.  In detail, it increases '->nr_accesses' of
// regions that an access has confirmed.  'kdamond_reset_aggregated()' flushes
// the aggregated information ('->nr_accesses' of each regions) to the result
// buffer.  As a result of the flushing, the '->nr_accesses' of regions are
// initialized to zero.
//
// '->nr_accesses' should be zeroed
// regions should be preserved
// targets also should be preserved
// 0-112, 114-130, 130-156, 156-170, 170-230, 230-10170
//
// When the total region count is already above max_nr_regions / 2,
// kdamond_split_regions() must keep refining the resolution by splitting a
// fraction of the regions (making progress), without exceeding
// max_nr_regions.
//
// Keep the split arithmetic independent of the page size
// Above max_nr_regions / 2, so the blanket-split path is skipped
// Still made progress ...
// ... but did not overshoot the configured maximum
// DAMON_OPS_VADDR is registered only if CONFIG_DAMON_VADDR is set
// DAMON_OPS_VADDR is ensured to be registered
// Double-registration is prohibited
// Unknown ops id cannot be registered
// Registration should success after unregistration
// Check double-registration failure again
// Initial build up on empty target.
// Un-intersecting regions should be removed.
//
// Holes should be filled up with new regions.
//
// old:       [4,   16)        [24,     32)
// new:         [8,                 28)
// expect:      [8, 16)[16,24),[24, 28)
//
// New regions should be able to be appended.
//
// old:       [0, 4)[4,    17)
// new:       [0,       15)     [25, 40)
// expect:    [0, 4)[4, 15)     [25, 40)
//
// New regions should be able to be inserted.
//
// old:       [0, 4)                      [42,    52)
// new:       [0,       15)     [25, 40)    [44, 50)
// expect:    [0,       15)     [25, 40)    [44, 50)
//
// current value, last value, remaining window (bp)
//
// Test damon_nr_accesses_mvsum(), which wraps damon_mvsum() with the
// monitoring intervals of the context.  With a sample interval of 1 and an
// aggregation interval of 10, an aggregation window is 10 sample intervals
// long.  Each row below specifies the passed sample intervals, the next
// aggregation time in sample intervals, the current and last nr_accesses of a
// region, and the expected return value.
//
// passed, next_aggr, nr_accesses, last_nr_accesses, expect
//
// When nr_src_goals is smaller than dst_goals,
// damos_commit_quota_goals() will kfree() the dst goals.
// Make it kfree()-able.
//
// Only power of two min_region_sz is allowed.
// region in the range
// region before the range
// region after the range
// region started before the range
// filter should have split the region
// region started in the range
// filter should have split the region
//
// If current score is lower than the goal, which is always 10,000
// (read the comment on damon_feed_loop_next_input()'s comment), next
// input should be higher than the last input.
//
// If current score is higher than the goal, next input should be lower
// than the last input.
//
// The next input depends on the distance between the current score and
// the goal
//
// No filter is installed.  Allow by default on both core and ops layer
// filtering stages, since there are no filters at all.
//
// A core-handled allow-filter is installed.
// Reject by default on core layer filtering stage due to the last
// core-layer-filter's behavior.
// Allow by default on ops layer filtering stage due to the absence of
// ops layer filters.
//
// A core-handled reject-filter is installed.
// Allow by default on core layer filtering stage due to the last
// core-layer-filter's behavior.
// Allow by default on ops layer filtering stage due to the absence of
// ops layer filters.
//
// A core-handled reject-filter and ops-handled allow-filter are installed.
// Allow by default on core layer filtering stage due to the existence
// of the ops-handled filter.
// Reject by default on ops layer filtering stage due to the last
// ops-layer-filter's behavior.
//
// A core-handled allow-filter and ops-handled allow-filter are
// installed.
// Allow by default on core layer filtering stage due to the existence
// of the ops-handled filter.
// Reject by default on ops layer filtering stage due to the last
// ops-layer-filter's behavior.
//
// common, expected setup
// no zero size limit
// max size should be aligned by min_region_sz
//
// when min_nr_regions and min_region_sz conflicts, min_region_sz wins.
//
// Verify that damos_walk() rejects new requests when
// walk_control_obsolete is set.
//
// This tests the invariant introduced by:
// commit 33c3f6c2b48c ("mm/damon/core: fix damos_walk() vs kdamond_fn() exit race")
//
// Simulate shutdown phase

