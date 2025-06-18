#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::{pallet::*, types::*};
use frame_benchmarking::v2::*;
use frame_support::{assert_ok, traits::Get};
use frame_system::RawOrigin;
use sp_std::vec;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn create_task() {
        let caller: T::AccountId = whitelisted_caller();
        let title = vec![1u8; T::MaxTitleLength::get() as usize];
        let description = vec![1u8; T::MaxDescriptionLength::get() as usize];

        #[extrinsic_call]
        create_task(
            RawOrigin::Signed(caller),
            title,
            description,
            Priority::Medium.into(),
            5,
            100u32.into(),
            None,
        );

        assert_eq!(NextTaskId::<T>::get(), 1);
    }

    #[benchmark]
    fn update_task() {
        let caller: T::AccountId = whitelisted_caller();
        let title = vec![1u8; T::MaxTitleLength::get() as usize];
        let description = vec![1u8; T::MaxDescriptionLength::get() as usize];

        // 先创建一个任务
        assert_ok!(Pallet::<T>::create_task(
            RawOrigin::Signed(caller.clone()).into(),
            title.clone(),
            description.clone(),
            Priority::Medium.into(),
            5,
            100u32.into(),
            None,
        ));

        let new_title = vec![2u8; T::MaxTitleLength::get() as usize];
        let new_description = vec![2u8; T::MaxDescriptionLength::get() as usize];

        #[extrinsic_call]
        update_task(
            RawOrigin::Signed(caller),
            0,
            Some(new_title),
            Some(new_description),
            Some(Priority::High.into()),
            Some(7),
            Some(200u32.into()),
            Some(Some(1000u32.into())),
        );

        let task = Tasks::<T>::get(0).unwrap();
        assert_eq!(task.priority, Priority::High.into());
    }

    #[benchmark]
    fn change_task_status() {
        let caller: T::AccountId = whitelisted_caller();
        let title = vec![1u8; T::MaxTitleLength::get() as usize];
        let description = vec![1u8; T::MaxDescriptionLength::get() as usize];

        // 先创建一个任务
        assert_ok!(Pallet::<T>::create_task(
            RawOrigin::Signed(caller.clone()).into(),
            title,
            description,
            Priority::Medium.into(),
            5,
            100u32.into(),
            None,
        ));

        #[extrinsic_call]
        change_task_status(RawOrigin::Signed(caller), 0, TaskStatus::InProgress.into());

        let task = Tasks::<T>::get(0).unwrap();
        assert_eq!(task.status, TaskStatus::InProgress.into());
    }

    #[benchmark]
    fn assign_task() {
        let caller: T::AccountId = whitelisted_caller();
        let assignee: T::AccountId = account("assignee", 0, 0);
        let title = vec![1u8; T::MaxTitleLength::get() as usize];
        let description = vec![1u8; T::MaxDescriptionLength::get() as usize];

        // 先创建一个任务
        assert_ok!(Pallet::<T>::create_task(
            RawOrigin::Signed(caller.clone()).into(),
            title,
            description,
            Priority::Medium.into(),
            5,
            100u32.into(),
            None,
        ));

        #[extrinsic_call]
        assign_task(RawOrigin::Signed(caller), 0, assignee.clone());

        let task = Tasks::<T>::get(0).unwrap();
        assert_eq!(task.assignee, Some(assignee));
    }

    #[benchmark]
    fn unassign_task() {
        let caller: T::AccountId = whitelisted_caller();
        let assignee: T::AccountId = account("assignee", 0, 0);
        let title = vec![1u8; T::MaxTitleLength::get() as usize];
        let description = vec![1u8; T::MaxDescriptionLength::get() as usize];

        // 先创建一个任务
        assert_ok!(Pallet::<T>::create_task(
            RawOrigin::Signed(caller.clone()).into(),
            title,
            description,
            Priority::Medium.into(),
            5,
            100u32.into(),
            None,
        ));

        // 分配任务
        assert_ok!(Pallet::<T>::assign_task(
            RawOrigin::Signed(caller.clone()).into(),
            0,
            assignee,
        ));

        #[extrinsic_call]
        unassign_task(RawOrigin::Signed(caller), 0);

        let task = Tasks::<T>::get(0).unwrap();
        assert_eq!(task.assignee, None);
    }

    #[benchmark]
    fn delete_task() {
        let caller: T::AccountId = whitelisted_caller();
        let title = vec![1u8; T::MaxTitleLength::get() as usize];
        let description = vec![1u8; T::MaxDescriptionLength::get() as usize];

        // 先创建一个任务
        assert_ok!(Pallet::<T>::create_task(
            RawOrigin::Signed(caller.clone()).into(),
            title,
            description,
            Priority::Medium.into(),
            5,
            100u32.into(),
            None,
        ));

        #[extrinsic_call]
        delete_task(RawOrigin::Signed(caller), 0);

        assert_eq!(Tasks::<T>::get(0), None);
    }

    impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}
