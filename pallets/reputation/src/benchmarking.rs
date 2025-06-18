//! Benchmarking setup for pallet-reputation

use super::*;

#[allow(unused)]
use crate::Pallet as Reputation;
use frame_benchmarking::v1::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
    evaluate_task {
        let caller: T::AccountId = whitelisted_caller();
        let assignee: T::AccountId = account("assignee", 0, 0);

        // 创建并完成任务的前置操作
        // 注意：在实际基准测试中，您需要设置完整的任务创建和完成流程

    }: _(RawOrigin::Signed(caller), 0, 4, b"Good work".to_vec())

    update_reputation {
        let caller: T::AccountId = whitelisted_caller();
        let user: T::AccountId = account("user", 0, 0);
    }: _(RawOrigin::Root, user, 100)

    impl_benchmark_test_suite!(Reputation, crate::mock::new_test_ext(), crate::mock::Test);
}
