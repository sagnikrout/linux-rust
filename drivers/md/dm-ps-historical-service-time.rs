//! Automatically rewritten from C to Rust
//! Source: drivers/md/dm-ps-historical-service-time.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Historical Service Time
//
// Keeps a time-weighted exponential moving average of the historical
// service time. Estimates future service time based on the historical
// service time and the number of outstanding requests.
//
// Marks paths stale if they have not finished within hst
// num_paths. If a path is stale and unused, we will send a single
// request to probe in case the path has improved. This situation
// generally arises if the path is so much worse than others that it
// will never have the best estimated service time, or if the entire
// multipath device is unused. If a path is stale and in use, limit the
// number of requests it can receive with the assumption that the path
// has become degraded.
//
// To avoid repeatedly calculating exponents for time weighting, times
// are split into HST_WEIGHT_COUNT buckets each (1 >> HST_BUCKET_SHIFT)
// ns, and the weighting is pre-calculated.
//

pub const HST_MIN_IO: c_int = 1;

pub const HST_FIXED_95: c_int = 972;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct selector {
    pub valid_paths: list_head,
    pub failed_paths: list_head,
    pub valid_count: c_int,
    pub lock: spinlock_t,
    pub weights: [c_uint; HST_WEIGHT_COUNT],
    pub threshold_multiplier: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct path_info {
    pub list: list_head,
    pub path: *mut dm_path,
    pub repeat_count: c_uint,
    pub lock: spinlock_t,
    pub /: *mut *mut u64 historical_service_time; / Fixed point,
    pub stale_after: u64,
    pub last_finish: u64,
    pub outstanding: u64,
}

//
// fixed_power - compute: x^n, in O(log n) time
//
// @x:         base of the power
// @frac_bits: fractional bits of @x
// @n:         power to raise @x to.
//
// By exploiting the relation between the definition of the natural power
// function: x^n := x*x*...*x (x multiplied by itself for n times), and
// the binary encoding of numbers used by computers: n := \Sum n_i * 2^i,
// (where: n_i \elem {0, 1}, the binary vector representing n),
// we find: x^n := x^(\Sum n_i * 2^i) := \Prod x^(n_i * 2^i), which is
// of course trivially computable in O(log_2 n), the length of our binary
// vector.
//
// (see: kernel/sched/loadavg.c)
//
#[no_mangle]
unsafe extern "C" fn fixed_power(x: u64, frac_bits: c_uint, n: c_uint) -> u64 {
    static u64 fixed_power(u64 x, unsigned int frac_bits, unsigned int n)
    {
    let mut result: c_ulong = 1UL << frac_bits;
    if (n) {
    for (;;) {
    if (n & 1) {
    result *= x;
    result += 1UL << (frac_bits - 1);
    result >>= frac_bits;
    }
    n >>= 1;
    if (!n)
    break;
    x *= x;
    x += 1UL << (frac_bits - 1);
    x >>= frac_bits;
    }
    }
    return result;
    }
//
// Calculate the next value of an exponential moving average
// a_1 = a_0 * e + a * (1 - e)
//
// @last: [0, ULLONG_MAX >> HST_FIXED_SHIFT]
// @next: [0, ULLONG_MAX >> HST_FIXED_SHIFT]
// @weight: [0, HST_FIXED_1]
//
// Note:
// To account for multiple periods in the same calculation,
// a_n = a_0 * e^n + a * (1 - e^n),
// so call fixed_ema(last, next, pow(weight, N))
//
#[no_mangle]
unsafe extern "C" fn fixed_ema(last: u64, next: u64, weight: u64) -> u64 {
    static u64 fixed_ema(u64 last, u64 next, u64 weight)
    {
    last *= weight;
    last += next * (HST_FIXED_1 - weight);
    last += 1ULL << (HST_FIXED_SHIFT - 1);
    return last >> HST_FIXED_SHIFT;
    }
    static struct selector *alloc_selector(void)
    {
    struct selector *s = kmalloc_obj(*s);
    if (s) {
    INIT_LIST_HEAD(&s.valid_paths);
    INIT_LIST_HEAD(&s.failed_paths);
    spin_lock_init(&s.lock);
    s.valid_count = 0;
    }
    return s;
    }
//
// Get the weight for a given time span.
//
#[no_mangle]
unsafe extern "C" fn hst_weight(ps: *mut path_selector, delta: u64) -> u64 {
    static u64 hst_weight(struct path_selector *ps, u64 delta)
    {
    struct selector *s = ps.context;
    int bucket = clamp(delta >> HST_BUCKET_SHIFT, 0ULL,
    HST_WEIGHT_COUNT - 1);
    return s.weights[bucket];
    }
//
// Set up the weights array.
//
// weights[len-1] = 0
// weights[n] = base ^ (n + 1)
//
#[no_mangle]
unsafe extern "C" fn hst_set_weights(ps: *mut path_selector, base: c_uint) {
    static void hst_set_weights(struct path_selector *ps, unsigned int base)
    {
    struct selector *s = ps.context;
    int i;
    if (base >= HST_FIXED_1)
    return;
    for (i = 0; i < HST_WEIGHT_COUNT - 1; i++)
    s.weights[i] = fixed_power(base, HST_FIXED_SHIFT, i + 1);
    s.weights[HST_WEIGHT_COUNT - 1] = 0;
    }
#[no_mangle]
unsafe extern "C" fn hst_create(ps: *mut path_selector, argc: c_uint, argv: *mut c_char) -> c_int {
    static int hst_create(struct path_selector *ps, unsigned int argc, char **argv)
    {
    struct selector *s;
    let mut base_weight: c_uint = HST_FIXED_95;
    let mut threshold_multiplier: c_uint = 0;
    char dummy;
//
// Arguments: [<base_weight> [<threshold_multiplier>]]
// <base_weight>: Base weight for ema [0, 1024) 10-bit fixed point. A
// value of 0 will completely ignore any history.
// If not given, default (HST_FIXED_95) is used.
// <threshold_multiplier>: Minimum threshold multiplier for paths to
// be considered different. That is, a path is
// considered different iff (p1 > N * p2) where p1
// is the path with higher service time. A threshold
// of 1 or 0 has no effect. Defaults to 0.
//
    if (argc > 2)
    return -EINVAL;
    if (argc && (sscanf(argv[0], "%u%c", &base_weight, &dummy) != 1 ||
    base_weight >= HST_FIXED_1)) {
    return -EINVAL;
    }
    if (argc > 1 && (sscanf(argv[1], "%u%c",
    &threshold_multiplier, &dummy) != 1)) {
    return -EINVAL;
    }
    s = alloc_selector();
    if (!s)
    return -ENOMEM;
    ps.context = s;
    hst_set_weights(ps, base_weight);
    s.threshold_multiplier = threshold_multiplier;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn free_paths(paths: *mut list_head) {
    static void free_paths(struct list_head *paths)
    {
    struct path_info *pi, *next;
    list_for_each_entry_safe(pi, next, paths, list) {
    list_del(&pi.list);
    kfree(pi);
    }
    }
#[no_mangle]
unsafe extern "C" fn hst_destroy(ps: *mut path_selector) {
    static void hst_destroy(struct path_selector *ps)
    {
    struct selector *s = ps.context;
    free_paths(&s.valid_paths);
    free_paths(&s.failed_paths);
    kfree(s);
    ps.context = core::ptr::null_mut();
    }
    static int hst_status(struct path_selector *ps, struct dm_path *path,
    status_type_t type, char *result, unsigned int maxlen)
    {
    let mut sz: c_uint = 0;
    struct path_info *pi;
    if (!path) {
    struct selector *s = ps.context;
    DMEMIT("2 %u %u ", s.weights[0], s.threshold_multiplier);
    } else {
    pi = path.pscontext;
    switch (type) {
    case STATUSTYPE_INFO:
    DMEMIT("%llu %llu %llu ", pi.historical_service_time,
    pi.outstanding, pi.stale_after);
    break;
    case STATUSTYPE_TABLE:
    DMEMIT("0 ");
    break;
    case STATUSTYPE_IMA:
// result = '\0';
    break;
    }
    }
    return sz;
    }
    static int hst_add_path(struct path_selector *ps, struct dm_path *path,
    int argc, char **argv, char **error)
    {
    struct selector *s = ps.context;
    struct path_info *pi;
    let mut repeat_count: c_uint = HST_MIN_IO;
    char dummy;
    unsigned long flags;
//
// Arguments: [<repeat_count>]
// <repeat_count>: The number of I/Os before switching path.
// If not given, default (HST_MIN_IO) is used.
//
    if (argc > 1) {
// error = "historical-service-time ps: incorrect number of arguments";
    return -EINVAL;
    }
    if (argc && (sscanf(argv[0], "%u%c", &repeat_count, &dummy) != 1)) {
// error = "historical-service-time ps: invalid repeat count";
    return -EINVAL;
    }
// allocate the path
    pi = kmalloc_obj(*pi);
    if (!pi) {
// error = "historical-service-time ps: Error allocating path context";
    return -ENOMEM;
    }
    pi.path = path;
    pi.repeat_count = repeat_count;
    pi.historical_service_time = HST_FIXED_1;
    spin_lock_init(&pi.lock);
    pi.outstanding = 0;
    pi.stale_after = 0;
    pi.last_finish = 0;
    path.pscontext = pi;
    spin_lock_irqsave(&s.lock, flags);
    list_add_tail(&pi.list, &s.valid_paths);
    s.valid_count++;
    spin_unlock_irqrestore(&s.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hst_fail_path(ps: *mut path_selector, path: *mut dm_path) {
    static void hst_fail_path(struct path_selector *ps, struct dm_path *path)
    {
    struct selector *s = ps.context;
    struct path_info *pi = path.pscontext;
    unsigned long flags;
    spin_lock_irqsave(&s.lock, flags);
    list_move(&pi.list, &s.failed_paths);
    s.valid_count--;
    spin_unlock_irqrestore(&s.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn hst_reinstate_path(ps: *mut path_selector, path: *mut dm_path) -> c_int {
    static int hst_reinstate_path(struct path_selector *ps, struct dm_path *path)
    {
    struct selector *s = ps.context;
    struct path_info *pi = path.pscontext;
    unsigned long flags;
    spin_lock_irqsave(&s.lock, flags);
    list_move_tail(&pi.list, &s.valid_paths);
    s.valid_count++;
    spin_unlock_irqrestore(&s.lock, flags);
    return 0;
    }
    static void hst_fill_compare(struct path_info *pi, u64 *hst,
    u64 *out, u64 *stale)
    {
    unsigned long flags;
    spin_lock_irqsave(&pi.lock, flags);
// hst = pi->historical_service_time;
// out = pi->outstanding;
// stale = pi->stale_after;
    spin_unlock_irqrestore(&pi.lock, flags);
    }
//
// Compare the estimated service time of 2 paths, pi1 and pi2,
// for the incoming I/O.
//
// Returns:
// < 0 : pi1 is better
// 0   : no difference between pi1 and pi2
// > 0 : pi2 is better
//
    static long long hst_compare(struct path_info *pi1, struct path_info *pi2,
    u64 time_now, struct path_selector *ps)
    {
    struct selector *s = ps.context;
    u64 hst1, hst2;
    long long out1, out2, stale1, stale2;
    int pi2_better, over_threshold;
    hst_fill_compare(pi1, &hst1, &out1, &stale1);
    hst_fill_compare(pi2, &hst2, &out2, &stale2);
// Check here if estimated latency for two paths are too similar.
// If this is the case, we skip extra calculation and just compare
// outstanding requests. In this case, any unloaded paths will
// be preferred.
//
    if (hst1 > hst2)
    over_threshold = hst1 > (s.threshold_multiplier * hst2);
    else
    over_threshold = hst2 > (s.threshold_multiplier * hst1);
    if (!over_threshold)
    return out1 - out2;
//
// If an unloaded path is stale, choose it. If both paths are unloaded,
// choose path that is the most stale.
// (If one path is loaded, choose the other)
//
    if ((!out1 && stale1 < time_now) || (!out2 && stale2 < time_now) ||
    (!out1 && !out2))
    return (!out2 * stale1) - (!out1 * stale2);
// Compare estimated service time. If outstanding is the same, we
// don't need to multiply
//
    if (out1 == out2) {
    pi2_better = hst1 > hst2;
    } else {
// Potential overflow with out >= 1024
    if (unlikely(out1 >= HST_MAX_INFLIGHT ||
    out2 >= HST_MAX_INFLIGHT)) {
// If over 1023 in-flights, we may overflow if hst
// is at max. (With this shift we still overflow at
// 1048576 in-flights, which is high enough).
//
    hst1 >>= HST_FIXED_SHIFT;
    hst2 >>= HST_FIXED_SHIFT;
    }
    pi2_better = (1 + out1) * hst1 > (1 + out2) * hst2;
    }
// In the case that the 'winner' is stale, limit to equal usage.
    if (pi2_better) {
    if (stale2 < time_now)
    return out1 - out2;
    return 1;
    }
    if (stale1 < time_now)
    return out1 - out2;
    return -1;
    }
    static struct dm_path *hst_select_path(struct path_selector *ps,
    size_t nr_bytes)
    {
    struct selector *s = ps.context;
    struct path_info *pi = core::ptr::null_mut(), *best = core::ptr::null_mut();
    let mut time_now: u64 = ktime_get_ns();
    struct dm_path *ret = core::ptr::null_mut();
    unsigned long flags;
    spin_lock_irqsave(&s.lock, flags);
    if (list_empty(&s.valid_paths))
    goto out;
    list_for_each_entry(pi, &s.valid_paths, list) {
    if (!best || (hst_compare(pi, best, time_now, ps) < 0))
    best = pi;
    }
    if (!best)
    goto out;
// Move last used path to end (least preferred in case of ties)
    list_move_tail(&best.list, &s.valid_paths);
    ret = best.path;
    out:
    spin_unlock_irqrestore(&s.lock, flags);
    return ret;
    }
    static int hst_start_io(struct path_selector *ps, struct dm_path *path,
    size_t nr_bytes)
    {
    struct path_info *pi = path.pscontext;
    unsigned long flags;
    spin_lock_irqsave(&pi.lock, flags);
    pi.outstanding++;
    spin_unlock_irqrestore(&pi.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn path_service_time(pi: *mut path_info, start_time: u64) -> u64 {
    static u64 path_service_time(struct path_info *pi, u64 start_time)
    {
    let mut now: u64 = ktime_get_ns();
// if a previous disk request has finished after this IO was
// sent to the hardware, pretend the submission happened
// serially.
//
    if (time_after64(pi.last_finish, start_time))
    start_time = pi.last_finish;
    pi.last_finish = now;
    if (time_before64(now, start_time))
    return 0;
    return now - start_time;
    }
    static int hst_end_io(struct path_selector *ps, struct dm_path *path,
    size_t nr_bytes, u64 start_time)
    {
    struct path_info *pi = path.pscontext;
    struct selector *s = ps.context;
    unsigned long flags;
    u64 st;
    spin_lock_irqsave(&pi.lock, flags);
    st = path_service_time(pi, start_time);
    pi.outstanding--;
    pi.historical_service_time =
    fixed_ema(pi.historical_service_time,
    min(st * HST_FIXED_1, HST_FIXED_MAX),
    hst_weight(ps, st));
//
// On request end, mark path as fresh. If a path hasn't
// finished any requests within the fresh period, the estimated
// service time is considered too optimistic and we limit the
// maximum requests on that path.
//
    pi.stale_after = pi.last_finish +
    (s.valid_count * (pi.historical_service_time >> HST_FIXED_SHIFT));
    spin_unlock_irqrestore(&pi.lock, flags);
    return 0;
    }
    static struct path_selector_type hst_ps = {
    .name		= "historical-service-time",
    .module		= THIS_MODULE,
    .features	= DM_PS_USE_HR_TIMER,
    .table_args	= 1,
    .info_args	= 3,
    .create		= hst_create,
    .destroy	= hst_destroy,
    .status		= hst_status,
    .add_path	= hst_add_path,
    .fail_path	= hst_fail_path,
    .reinstate_path	= hst_reinstate_path,
    .select_path	= hst_select_path,
    .start_io	= hst_start_io,
    .end_io		= hst_end_io,
    };
#[no_mangle]
unsafe extern "C" fn dm_hst_init() -> int __init {
    static int __init dm_hst_init(void)
    {
    let mut r: c_int = dm_register_path_selector(&hst_ps);
    if (r < 0) {
    DMERR("register failed %d", r);
    return r;
    }
    DMINFO("version " HST_VERSION " loaded");
    return r;
    }
#[no_mangle]
unsafe extern "C" fn dm_hst_exit() -> void __exit {
    static void __exit dm_hst_exit(void)
    {
    dm_unregister_path_selector(&hst_ps);
    }
    module_init(dm_hst_init);
    module_exit(dm_hst_exit);
    MODULE_DESCRIPTION(DM_NAME " measured service time oriented path selector");
    MODULE_AUTHOR("Khazhismel Kumykov <khazhy@google.com>");
    MODULE_LICENSE("GPL");
