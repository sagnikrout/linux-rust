//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/scheduler/tests/tests_scheduler.c
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
// Copyright (c) 2025 Valve Corporation

//
// DRM scheduler tests exercise load balancing decisions ie. entity selection
// logic.
//
#[no_mangle]
unsafe extern "C" fn drm_sched_scheduler_init(test: *mut kunit) -> c_int {
    static int drm_sched_scheduler_init(struct kunit *test)
    {
    struct drm_mock_scheduler *sched;
    sched = drm_mock_sched_new(test, MAX_SCHEDULE_TIMEOUT);
    sched.base.credit_limit = 1;
    test.priv = sched;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn drm_sched_scheduler_init2(test: *mut kunit) -> c_int {
    static int drm_sched_scheduler_init2(struct kunit *test)
    {
    struct drm_mock_scheduler *sched;
    sched = drm_mock_sched_new(test, MAX_SCHEDULE_TIMEOUT);
    sched.base.credit_limit = 2;
    test.priv = sched;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn drm_sched_scheduler_exit(test: *mut kunit) {
    static void drm_sched_scheduler_exit(struct kunit *test)
    {
    struct drm_mock_scheduler *sched = test.priv;
    drm_mock_sched_fini(sched);
    }
#[no_mangle]
unsafe extern "C" fn drm_sched_scheduler_queue_overhead(test: *mut kunit) {
    static void drm_sched_scheduler_queue_overhead(struct kunit *test)
    {
    struct drm_mock_scheduler *sched = test.priv;
    struct drm_mock_sched_entity *entity;
    let mut job_us: c_uint = 1000;
    let mut jobs: c_uint = 1000;
    let mut total_us: c_uint = jobs * job_us;
    struct drm_mock_sched_job *job, *first;
    ktime_t start, end;
    bool done;
    int i;
//
// Deep queue job at a time processing (single credit).
//
// This measures the overhead of picking and processing a job at a time
// by comparing the ideal total "GPU" time of all submitted jobs versus
// the time actually taken.
//
    KUNIT_ASSERT_EQ(test, sched.base.credit_limit, 1);
    entity = drm_mock_sched_entity_new(test,
    DRM_SCHED_PRIORITY_NORMAL,
    sched);
    for (i = 0; i <= jobs; i++) {
    job = drm_mock_sched_job_new(test, entity);
    if (i == 0)
    first = job; /* Extra first job blocks the queue */
    else
    drm_mock_sched_job_set_duration_us(job, job_us);
    drm_mock_sched_job_submit(job);
    }
    done = drm_mock_sched_job_wait_scheduled(first, HZ);
    KUNIT_ASSERT_TRUE(test, done);
    start = ktime_get();
    i = drm_mock_sched_advance(sched, 1); /* Release the queue */
    KUNIT_ASSERT_EQ(test, i, 1);
// Wait with a safe margin to avoid every failing.
    done = drm_mock_sched_job_wait_finished(job,
    usecs_to_jiffies(total_us) * 5);
    end = ktime_get();
    KUNIT_ASSERT_TRUE(test, done);
    pr_info("Expected %uus, actual %lldus\n",
    total_us,
    ktime_to_us(ktime_sub(end, start)));
    drm_mock_sched_entity_free(entity);
    }
#[no_mangle]
unsafe extern "C" fn drm_sched_scheduler_ping_pong(test: *mut kunit) {
    static void drm_sched_scheduler_ping_pong(struct kunit *test)
    {
    struct drm_mock_sched_job *job, *first, *prev = core::ptr::null_mut();
    struct drm_mock_scheduler *sched = test.priv;
    struct drm_mock_sched_entity *entity[2];
    let mut job_us: c_uint = 1000;
    let mut jobs: c_uint = 1000;
    let mut total_us: c_uint = jobs * job_us;
    ktime_t start, end;
    bool done;
    int i;
//
// Two entitites in inter-dependency chain.
//
// This measures the overhead of picking and processing a job at a time,
// where each job depends on the previous one from the diffferent
// entity, by comparing the ideal total "GPU" time of all submitted jobs
// versus the time actually taken.
//
    KUNIT_ASSERT_EQ(test, sched.base.credit_limit, 1);
    for (i = 0; i < ARRAY_SIZE(entity); i++)
    entity[i] = drm_mock_sched_entity_new(test,
    DRM_SCHED_PRIORITY_NORMAL,
    sched);
    for (i = 0; i <= jobs; i++) {
    job = drm_mock_sched_job_new(test, entity[i & 1]);
    if (i == 0)
    first = job; /* Extra first job blocks the queue */
    else
    drm_mock_sched_job_set_duration_us(job, job_us);
    if (prev)
    drm_sched_job_add_dependency(&job.base,
    dma_fence_get(&prev.base.s_fence.finished));
    drm_mock_sched_job_submit(job);
    prev = job;
    }
    done = drm_mock_sched_job_wait_scheduled(first, HZ);
    KUNIT_ASSERT_TRUE(test, done);
    start = ktime_get();
    i = drm_mock_sched_advance(sched, 1); /* Release the queue */
    KUNIT_ASSERT_EQ(test, i, 1);
// Wait with a safe margin to avoid every failing.
    done = drm_mock_sched_job_wait_finished(job,
    usecs_to_jiffies(total_us) * 5);
    end = ktime_get();
    KUNIT_ASSERT_TRUE(test, done);
    pr_info("Expected %uus, actual %lldus\n",
    total_us,
    ktime_to_us(ktime_sub(end, start)));
    for (i = 0; i < ARRAY_SIZE(entity); i++)
    drm_mock_sched_entity_free(entity[i]);
    }
    static struct kunit_case drm_sched_scheduler_overhead_tests[] = {
    KUNIT_CASE_SLOW(drm_sched_scheduler_queue_overhead),
    KUNIT_CASE_SLOW(drm_sched_scheduler_ping_pong),
    {}
    };
    static struct kunit_suite drm_sched_scheduler_overhead = {
    .name = "drm_sched_scheduler_overhead_tests",
    .init = drm_sched_scheduler_init,
    .exit = drm_sched_scheduler_exit,
    .test_cases = drm_sched_scheduler_overhead_tests,
    };
//
// struct drm_sched_client_params - describe a workload emitted from a client
//
// A simulated client will create an entity with a scheduling @priority and emit
// jobs in a loop where each iteration will consist of:
//
// 1. Submit @job_cnt jobs, each with a set duration of @job_us.
// 2. If @sync is true wait for last submitted job to finish.
// 3. Sleep for @wait_us micro-seconds.
// 4. Repeat.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_sched_client_params {
    pub priority: enum drm_sched_priority,
    pub job_cnt: c_uint,
    pub job_us: c_uint,
    pub sync: bool,
    pub wait_us: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_sched_test_params {
    pub description: *const c_char,
    pub num_clients: c_uint,
    pub client: [drm_sched_client_params; 2],
}

    static const struct drm_sched_test_params drm_sched_cases[] = {
    {
    .description = "Normal priority and normal priority",
    .client[0] = {
    .priority = DRM_SCHED_PRIORITY_NORMAL,
    .job_cnt = 1,
    .job_us = 8000,
    .wait_us = 0,
    .sync = false,
    },
    .client[1] = {
    .priority = DRM_SCHED_PRIORITY_NORMAL,
    .job_cnt = 1,
    .job_us = 8000,
    .wait_us = 0,
    .sync = false,
    },
    },
    {
    .description = "Normal priority and low priority",
    .client[0] = {
    .priority = DRM_SCHED_PRIORITY_NORMAL,
    .job_cnt = 1,
    .job_us = 8000,
    .wait_us = 0,
    .sync = false,
    },
    .client[1] = {
    .priority = DRM_SCHED_PRIORITY_LOW,
    .job_cnt = 1,
    .job_us = 8000,
    .wait_us = 0,
    .sync = false,
    },
    },
    {
    .description = "High priority and normal priority",
    .client[0] = {
    .priority = DRM_SCHED_PRIORITY_HIGH,
    .job_cnt = 1,
    .job_us = 8000,
    .wait_us = 0,
    .sync = false,
    },
    .client[1] = {
    .priority = DRM_SCHED_PRIORITY_NORMAL,
    .job_cnt = 1,
    .job_us = 8000,
    .wait_us = 0,
    .sync = false,
    },
    },
    {
    .description = "High priority and low priority",
    .client[0] = {
    .priority = DRM_SCHED_PRIORITY_HIGH,
    .job_cnt = 1,
    .job_us = 8000,
    .wait_us = 0,
    .sync = false,
    },
    .client[1] = {
    .priority = DRM_SCHED_PRIORITY_LOW,
    .job_cnt = 1,
    .job_us = 8000,
    .wait_us = 0,
    .sync = false,
    },
    },
    {
    .description = "50% and 50%",
    .client[0] = {
    .priority = DRM_SCHED_PRIORITY_NORMAL,
    .job_cnt = 1,
    .job_us = 1500,
    .wait_us = 1500,
    .sync = true,
    },
    .client[1] = {
    .priority = DRM_SCHED_PRIORITY_NORMAL,
    .job_cnt = 1,
    .job_us = 2500,
    .wait_us = 2500,
    .sync = true,
    },
    },
    {
    .description = "50% and 50% low priority",
    .client[0] = {
    .priority = DRM_SCHED_PRIORITY_NORMAL,
    .job_cnt = 1,
    .job_us = 1500,
    .wait_us = 1500,
    .sync = true,
    },
    .client[1] = {
    .priority = DRM_SCHED_PRIORITY_LOW,
    .job_cnt = 1,
    .job_us = 2500,
    .wait_us = 2500,
    .sync = true,
    },
    },
    {
    .description = "50% high priority and 50%",
    .client[0] = {
    .priority = DRM_SCHED_PRIORITY_HIGH,
    .job_cnt = 1,
    .job_us = 1500,
    .wait_us = 1500,
    .sync = true,
    },
    .client[1] = {
    .priority = DRM_SCHED_PRIORITY_NORMAL,
    .job_cnt = 1,
    .job_us = 2500,
    .wait_us = 2500,
    .sync = true,
    },
    },
    {
    .description = "Low priority hog and interactive client",
    .client[0] = {
    .priority = DRM_SCHED_PRIORITY_LOW,
    .job_cnt = 3,
    .job_us = 2500,
    .wait_us = 500,
    .sync = false,
    },
    .client[1] = {
    .priority = DRM_SCHED_PRIORITY_NORMAL,
    .job_cnt = 1,
    .job_us = 500,
    .wait_us = 10000,
    .sync = true,
    },
    },
    {
    .description = "Heavy rendering and interactive client",
    .client[0] = {
    .priority = DRM_SCHED_PRIORITY_NORMAL,
    .job_cnt = 3,
    .job_us = 2500,
    .wait_us = 2500,
    .sync = true,
    },
    .client[1] = {
    .priority = DRM_SCHED_PRIORITY_NORMAL,
    .job_cnt = 1,
    .job_us = 1000,
    .wait_us = 9000,
    .sync = true,
    },
    },
    {
    .description = "Very heavy rendering and interactive client",
    .client[0] = {
    .priority = DRM_SCHED_PRIORITY_NORMAL,
    .job_cnt = 4,
    .job_us = 50000,
    .wait_us = 1,
    .sync = true,
    },
    .client[1] = {
    .priority = DRM_SCHED_PRIORITY_NORMAL,
    .job_cnt = 1,
    .job_us = 1000,
    .wait_us = 9000,
    .sync = true,
    },
    },
    };
    static void
    drm_sched_desc(const struct drm_sched_test_params *params, char *desc)
    {
    strscpy(desc, params.description, KUNIT_PARAM_DESC_SIZE);
    }
    KUNIT_ARRAY_PARAM(drm_sched_scheduler_two_clients,
    drm_sched_cases,
    drm_sched_desc);
//
// struct test_client_stats - track client stats
//
// For each client executing a simulated workload we track some timings for
// which we are interested in the minimum of all iterations (@min_us), maximum
// (@max_us) and the overall total for all iterations (@tot_us).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_client_stats {
    pub min_us: c_uint,
    pub max_us: c_uint,
    pub tot_us: c_ulong,
}

//
// struct test_client - a simulated userspace client submitting scheduler work
//
// Each client executing a simulated workload is represented by one of these.
//
// Each of them instantiates a scheduling @entity and executes a workloads as
// defined in @params. Based on those @params the theoretical execution time of
// the client is calculated as @ideal_duration, while the actual wall time is
// tracked in @duration (calculated based on the @start and @end client time-
// stamps).
//
// Numerical @id is assigned to each for logging purposes.
//
// @worker and @work are used to provide an independent execution context from
// which scheduler jobs are submitted.
//
// During execution statistics on how long it took to submit and execute one
// iteration (whether or not synchronous) is kept in @cycle_time, while
// @latency_time tracks the @cycle_time minus the ideal duration of the one
// cycle.
//
// Once the client has completed the set number of iterations it will write the
// completion status into @done.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_client {
    pub /: *mut *mut *mut kunit test; / Backpointer to the kunit test.,
    pub entity: *mut drm_mock_sched_entity,
    pub worker: *mut kthread_worker,
    pub work: kthread_work,
    pub params: drm_sched_client_params,
    pub id: c_uint,
    pub duration: ktime_t,
    pub ideal_duration: ktime_t,
    pub cycles: c_uint,
    pub cycle: c_uint,
    pub start: ktime_t,
    pub end: ktime_t,
    pub done: bool,
    pub cycle_time: test_client_stats,
    pub latency_time: test_client_stats,
}

    static void
    update_stats(struct test_client_stats *stats, unsigned int us)
    {
    if (us > stats.max_us)
    stats.max_us = us;
    if (us < stats.min_us)
    stats.min_us = us;
    stats.tot_us += us;
    }
    static unsigned int
    get_stats_avg(struct test_client_stats *stats, unsigned int cycles)
    {
    return div_u64(stats.tot_us, cycles);
    }
#[no_mangle]
unsafe extern "C" fn drm_sched_client_work(work: *mut kthread_work) {
    static void drm_sched_client_work(struct kthread_work *work)
    {
    struct test_client *client = container_of(work, typeof(*client), work);
    let mut sync_wait: c_long = MAX_SCHEDULE_TIMEOUT;
    unsigned int cycle, work_us, period_us;
    struct drm_mock_sched_job *job = core::ptr::null_mut();
    work_us = client.params.job_cnt * client.params.job_us;
    period_us = work_us + client.params.wait_us;
    client.cycles =
    DIV_ROUND_UP((unsigned int)ktime_to_us(client.duration),
    period_us);
    client.ideal_duration = us_to_ktime(client.cycles * period_us);
    client.start = ktime_get();
    for (cycle = 0; cycle < client.cycles; cycle++) {
    ktime_t cycle_time;
    unsigned int batch;
    unsigned long us;
    if (READ_ONCE(client.done))
    break;
    cycle_time = ktime_get();
    for (batch = 0; batch < client.params.job_cnt; batch++) {
    job = drm_mock_sched_job_new(client.test,
    client.entity);
    drm_mock_sched_job_set_duration_us(job,
    client.params.job_us);
    drm_mock_sched_job_submit(job);
    }
    if (client.params.sync)
    drm_mock_sched_job_wait_finished(job, sync_wait);
    cycle_time = ktime_sub(ktime_get(), cycle_time);
    us = ktime_to_us(cycle_time);
    update_stats(&client.cycle_time, us);
    if (ktime_to_us(cycle_time) >= (long)work_us)
    us = ktime_to_us(cycle_time) - work_us;
    else if (WARN_ON_ONCE(client.params.sync)) /* GPU job took less than expected. */
    us = 0;
    update_stats(&client.latency_time, us);
    WRITE_ONCE(client.cycle, cycle);
    if (READ_ONCE(client.done))
    break;
    if (client.params.wait_us)
    fsleep(client.params.wait_us);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !client->params.sync) -> else {
    else if (!client.params.sync)
    cond_resched(); /* Do not hog the CPU if fully async. */
    }
    client.done = drm_mock_sched_job_wait_finished(job, sync_wait);
    client.end = ktime_get();
    }
    static const char *prio_str(enum drm_sched_priority prio)
    {
    switch (prio) {
    case DRM_SCHED_PRIORITY_KERNEL:
    return "kernel";
    case DRM_SCHED_PRIORITY_LOW:
    return "low";
    case DRM_SCHED_PRIORITY_NORMAL:
    return "normal";
    case DRM_SCHED_PRIORITY_HIGH:
    return "high";
    default:
    return "???";
    }
    }
#[no_mangle]
unsafe extern "C" fn client_done(client: *mut test_client) -> bool {
    static bool client_done(struct test_client *client)
    {
    return READ_ONCE(client.done); /* READ_ONCE to document lockless read from a loop. */
    }
#[no_mangle]
unsafe extern "C" fn drm_sched_scheduler_two_clients_test(test: *mut kunit) {
    static void drm_sched_scheduler_two_clients_test(struct kunit *test)
    {
    const struct drm_sched_test_params *params = test.param_value;
    struct drm_mock_scheduler *sched = test.priv;
    struct test_client client[2] = { };
    unsigned int prev_cycle[2] = { };
    unsigned int i, j;
    ktime_t start;
//
// Same job stream from two clients.
//
    for (i = 0; i < ARRAY_SIZE(client); i++)
    client[i].entity =
    drm_mock_sched_entity_new(test,
    params.client[i].priority,
    sched);
    for (i = 0; i < ARRAY_SIZE(client); i++) {
    client[i].test = test;
    client[i].id = i;
    client[i].duration = ms_to_ktime(1000);
    client[i].params = params.client[i];
    client[i].cycle_time.min_us = ~0U;
    client[i].latency_time.min_us = ~0U;
    client[i].worker =
    kthread_create_worker(0, "%s-%u", __func__, i);
    if (IS_ERR(client[i].worker)) {
    for (j = 0; j < i; j++)
    kthread_destroy_worker(client[j].worker);
    KUNIT_FAIL(test, "Failed to create worker!\n");
    }
    kthread_init_work(&client[i].work, drm_sched_client_work);
    }
    for (i = 0; i < ARRAY_SIZE(client); i++)
    kthread_queue_work(client[i].worker, &client[i].work);
//
// The clients (workers) can be a mix of async (deep submission queue),
// sync (one job at a time), or something in between. Therefore it is
// difficult to display a single metric representing their progress.
//
// Each struct drm_sched_client_params describes the actual submission
// pattern which happens in the following steps:
// 1. Submit N jobs
// 2. Wait for last submitted job to finish
// 3. Sleep for U micro-seconds
// 4. Goto 1. for C cycles
//
// Where number of cycles is calculated to match the target client
// duration from the respective struct drm_sched_test_params.
//
// To asses scheduling behaviour what we output for both clients is:
// - pct: Percentage progress of the jobs submitted
// - cps: "Cycles" per second (where one cycle is one complete
// iteration from the above)
// -  qd: Number of outstanding jobs in the client/entity
//
    pr_info(" [pct] - Job submission progress\n"
    " [cps] - Cycles per second\n"
    "  [qd] - Number of outstanding jobs in the client/entity\n");
    pr_info("%s:\n\t            pct1 cps1 qd1;  pct2 cps2 qd2\n",
    params.description);
    start = ktime_get();
    while (!client_done(&client[0]) || !client_done(&client[1])) {
    let mut period_ms: c_uint = 100;
    let mut frequency: c_uint = 1000 / period_ms;
    unsigned int pct[2], qd[2], cycle[2], cps[2];
    for (i = 0; i < ARRAY_SIZE(client); i++) {
    qd[i] = spsc_queue_count(&client[i].entity.base.job_queue);
    cycle[i] = READ_ONCE(client[i].cycle);
    cps[i] = DIV_ROUND_UP(100 * frequency *
    (cycle[i] - prev_cycle[i]),
    100);
    if (client[i].cycles)
    pct[i] = DIV_ROUND_UP(100 * (1 + cycle[i]),
    client[i].cycles);
    else
    pct[i] = 0;
    prev_cycle[i] = cycle[i];
    }
    if (client_done(&client[0]))
    pr_info("\t+%6lldms:               ; %3u %5u %4u\n",
    ktime_to_ms(ktime_sub(ktime_get(), start)),
    pct[1], cps[1], qd[1]);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: client_done(&client[1])) -> else {
    else if (client_done(&client[1]))
    pr_info("\t+%6lldms: %3u %5u %4u;\n",
    ktime_to_ms(ktime_sub(ktime_get(), start)),
    pct[0], cps[0], qd[0]);
    else
    pr_info("\t+%6lldms: %3u %5u %4u; %3u %5u %4u\n",
    ktime_to_ms(ktime_sub(ktime_get(), start)),
    pct[0], cps[0], qd[0],
    pct[1], cps[1], qd[1]);
    msleep(period_ms);
    }
    for (i = 0; i < ARRAY_SIZE(client); i++) {
    kthread_flush_work(&client[i].work);
    kthread_destroy_worker(client[i].worker);
    }
    for (i = 0; i < ARRAY_SIZE(client); i++)
    KUNIT_ASSERT_TRUE(test, client[i].done);
    for (i = 0; i < ARRAY_SIZE(client); i++) {
    pr_info("    %u: prio=%s sync=%u elapsed_ms=%lldms (ideal_ms=%lldms) cycle_time(min,avg,max)=%u,%u,%u us latency_time(min,avg,max)=%u,%u,%u us",
    i,
    prio_str(params.client[i].priority),
    params.client[i].sync,
    ktime_to_ms(ktime_sub(client[i].end, client[i].start)),
    ktime_to_ms(client[i].ideal_duration),
    client[i].cycle_time.min_us,
    get_stats_avg(&client[i].cycle_time, client[i].cycles),
    client[i].cycle_time.max_us,
    client[i].latency_time.min_us,
    get_stats_avg(&client[i].latency_time, client[i].cycles),
    client[i].latency_time.max_us);
    drm_mock_sched_entity_free(client[i].entity);
    }
    }
    static struct kunit_case drm_sched_scheduler_two_clients_tests[] = {
    KUNIT_CASE_PARAM_ATTR(drm_sched_scheduler_two_clients_test,
    drm_sched_scheduler_two_clients_gen_params,
    { .speed = KUNIT_SPEED_SLOW }),
    {}
    };
    static struct kunit_suite drm_sched_scheduler_two_clients1 = {
    .name = "drm_sched_scheduler_two_clients_one_credit_tests",
    .init = drm_sched_scheduler_init,
    .exit = drm_sched_scheduler_exit,
    .test_cases = drm_sched_scheduler_two_clients_tests,
    };
    static struct kunit_suite drm_sched_scheduler_two_clients2 = {
    .name = "drm_sched_scheduler_two_clients_two_credits_tests",
    .init = drm_sched_scheduler_init2,
    .exit = drm_sched_scheduler_exit,
    .test_cases = drm_sched_scheduler_two_clients_tests,
    };
    static const struct drm_sched_test_params drm_sched_many_cases[] = {
    {
    .description = "2 clients",
    .num_clients = 2,
    .client[0] = {
    .priority = DRM_SCHED_PRIORITY_NORMAL,
    .job_cnt = 4,
    .job_us = 1000,
    .wait_us = 0,
    .sync = true,
    },
    },
    {
    .description = "3 clients",
    .num_clients = 3,
    .client[0] = {
    .priority = DRM_SCHED_PRIORITY_NORMAL,
    .job_cnt = 4,
    .job_us = 1000,
    .wait_us = 0,
    .sync = true,
    },
    },
    {
    .description = "7 clients",
    .num_clients = 7,
    .client[0] = {
    .priority = DRM_SCHED_PRIORITY_NORMAL,
    .job_cnt = 4,
    .job_us = 1000,
    .wait_us = 0,
    .sync = true,
    },
    },
    {
    .description = "13 clients",
    .num_clients = 13,
    .client[0] = {
    .priority = DRM_SCHED_PRIORITY_NORMAL,
    .job_cnt = 4,
    .job_us = 1000,
    .wait_us = 0,
    .sync = true,
    },
    },
    {
    .description = "31 clients",
    .num_clients = 31,
    .client[0] = {
    .priority = DRM_SCHED_PRIORITY_NORMAL,
    .job_cnt = 2,
    .job_us = 1000,
    .wait_us = 0,
    .sync = true,
    },
    },
    };
    KUNIT_ARRAY_PARAM(drm_sched_scheduler_many_clients,
    drm_sched_many_cases,
    drm_sched_desc);
#[no_mangle]
unsafe extern "C" fn drm_sched_scheduler_many_clients_test(test: *mut kunit) {
    static void drm_sched_scheduler_many_clients_test(struct kunit *test)
    {
    const struct drm_sched_test_params *params = test.param_value;
    struct drm_mock_scheduler *sched = test.priv;
    let mut clients: c_uint = params.num_clients;
    unsigned int i, j, delta_total = 0, loops = 0;
    struct test_client *client;
    unsigned int *prev_cycle;
    ktime_t start;
    char *buf;
//
// Many clients with deep-ish async queues.
//
    buf = kunit_kmalloc(test, PAGE_SIZE, GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, buf);
    client = kunit_kcalloc(test, clients, sizeof(*client), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, client);
    prev_cycle = kunit_kcalloc(test, clients, sizeof(*prev_cycle),
    GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, prev_cycle);
    for (i = 0; i < clients; i++)
    client[i].entity =
    drm_mock_sched_entity_new(test,
    DRM_SCHED_PRIORITY_NORMAL,
    sched);
    for (i = 0; i < clients; i++) {
    client[i].test = test;
    client[i].id = i;
    client[i].params = params.client[0];
    client[i].duration = ms_to_ktime(1000 / clients);
    client[i].cycle_time.min_us = ~0U;
    client[i].latency_time.min_us = ~0U;
    client[i].worker =
    kthread_create_worker(0, "%s-%u", __func__, i);
    if (IS_ERR(client[i].worker)) {
    for (j = 0; j < i; j++)
    kthread_destroy_worker(client[j].worker);
    KUNIT_FAIL(test, "Failed to create worker!\n");
    }
    kthread_init_work(&client[i].work, drm_sched_client_work);
    }
    for (i = 0; i < clients; i++)
    kthread_queue_work(client[i].worker, &client[i].work);
    start = ktime_get();
    pr_info("%u clients:\n\tt\t\tcycle:\t  min    avg    max : ...\n", clients);
    for (;;) {
    let mut min: c_uint = ~0;
    let mut max: c_uint = 0;
    let mut total: c_uint = 0;
    let mut done: bool = true;
    char pbuf[16];
    memset(buf, 0, PAGE_SIZE);
    for (i = 0; i < clients; i++) {
    unsigned int cycle, cycles;
// Read current progress from the threaded worker.
    cycle = READ_ONCE(client[i].cycle);
    cycles = READ_ONCE(client[i].cycles);
    snprintf(pbuf, sizeof(pbuf), " %3d", cycle);
    strncat(buf, pbuf, PAGE_SIZE);
    total += cycle;
    if (cycle < min)
    min = cycle;
    if (cycle > max)
    max = cycle;
    if (!min || (cycle + 1) < cycles)
    done = false;
    }
    loops++;
    delta_total += max - min;
    pr_info("\t+%6lldms\t\t  %3u  %3u  %3u :%s\n",
    ktime_to_ms(ktime_sub(ktime_get(), start)),
    min, DIV_ROUND_UP(total, clients), max, buf);
    if (done)
    break;
    msleep(100);
    }
    pr_info("    avg_max_min_delta(x100)=%u\n",
    loops ? DIV_ROUND_UP(delta_total * 100, loops) : 0);
    for (i = 0; i < clients; i++) {
    kthread_flush_work(&client[i].work);
    kthread_destroy_worker(client[i].worker);
    }
    for (i = 0; i < clients; i++)
    drm_mock_sched_entity_free(client[i].entity);
    }
    static struct kunit_case drm_sched_scheduler_many_clients_tests[] = {
    KUNIT_CASE_PARAM_ATTR(drm_sched_scheduler_many_clients_test,
    drm_sched_scheduler_many_clients_gen_params,
    { .speed = KUNIT_SPEED_SLOW }),
    {}
    };
    static struct kunit_suite drm_sched_scheduler_many_clients = {
    .name = "drm_sched_scheduler_many_clients_tests",
    .init = drm_sched_scheduler_init2,
    .exit = drm_sched_scheduler_exit,
    .test_cases = drm_sched_scheduler_many_clients_tests,
    };
    kunit_test_suites(&drm_sched_scheduler_overhead,
    &drm_sched_scheduler_two_clients1,
    &drm_sched_scheduler_two_clients2,
    &drm_sched_scheduler_many_clients);
