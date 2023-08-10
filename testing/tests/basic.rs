#[cfg(test)]
#[cfg_attr(
    feature = "tokio-executor",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(feature = "async-std-executor", async_std::test)]
async fn test_basic() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let metadata = hotshot_testing::test_builder::TestMetadata::default();
    metadata
        .gen_launcher::<hotshot_testing::node_types::SequencingTestTypes, hotshot_testing::node_types::SequencingMemoryImpl>()
        .launch()
        .run_test()
        .await;
}

#[cfg(test)]
#[cfg_attr(
    feature = "tokio-executor",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(feature = "async-std-executor", async_std::test)]
async fn test_with_failures_one() {
    use std::time::Duration;

    use hotshot_testing::{
        completion_task::TimeBasedCompletionTaskDescription, spinning_task::SpinningTaskDescription,
    };

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let mut metadata =
        hotshot_testing::test_builder::TestMetadata::default_more_nodes_less_success();
    let dead_nodes = vec![hotshot_testing::spinning_task::ChangeNode {
        idx: 10,
        updown: hotshot_testing::spinning_task::UpDown::Down,
    }];

    metadata.spinning_properties = SpinningTaskDescription {
        node_changes: vec![(std::time::Duration::new(4, 0), dead_nodes)],
    };
    metadata
        .gen_launcher::<hotshot_testing::node_types::SequencingTestTypes, hotshot_testing::node_types::SequencingMemoryImpl>()
        .launch()
        .run_test()
        .await;
}

#[cfg(test)]
#[cfg_attr(
    feature = "tokio-executor",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(feature = "async-std-executor", async_std::test)]
async fn test_with_failures_half_f() {
    use std::time::Duration;

    use hotshot_testing::{
        completion_task::TimeBasedCompletionTaskDescription,
        overall_safety_task::OverallSafetyPropertiesDescription,
        spinning_task::SpinningTaskDescription,
    };

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let mut metadata =
        hotshot_testing::test_builder::TestMetadata::default_more_nodes_less_success();
    let dead_nodes = vec![
        hotshot_testing::spinning_task::ChangeNode {
            idx: 5,
            updown: hotshot_testing::spinning_task::UpDown::Down,
        },
        hotshot_testing::spinning_task::ChangeNode {
            idx: 10,
            updown: hotshot_testing::spinning_task::UpDown::Down,
        },
        hotshot_testing::spinning_task::ChangeNode {
            idx: 15,
            updown: hotshot_testing::spinning_task::UpDown::Down,
        },
    ];

    metadata.spinning_properties = SpinningTaskDescription {
        node_changes: vec![(std::time::Duration::new(4, 0), dead_nodes)],
    };
    metadata
        .gen_launcher::<hotshot_testing::node_types::SequencingTestTypes, hotshot_testing::node_types::SequencingMemoryImpl>()
        .launch()
        .run_test()
        .await;
}

#[cfg(test)]
#[cfg_attr(
    feature = "tokio-executor",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(feature = "async-std-executor", async_std::test)]
async fn test_with_failures_f() {
    use std::time::Duration;

    use hotshot_testing::{
        completion_task::TimeBasedCompletionTaskDescription,
        overall_safety_task::OverallSafetyPropertiesDescription,
        spinning_task::SpinningTaskDescription,
    };

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let mut metadata =
        hotshot_testing::test_builder::TestMetadata::default_more_nodes_less_success();
    let dead_nodes = vec![
        hotshot_testing::spinning_task::ChangeNode {
            idx: 5,
            updown: hotshot_testing::spinning_task::UpDown::Down,
        },
        hotshot_testing::spinning_task::ChangeNode {
            idx: 6,
            updown: hotshot_testing::spinning_task::UpDown::Down,
        },
        hotshot_testing::spinning_task::ChangeNode {
            idx: 10,
            updown: hotshot_testing::spinning_task::UpDown::Down,
        },
        hotshot_testing::spinning_task::ChangeNode {
            idx: 11,
            updown: hotshot_testing::spinning_task::UpDown::Down,
        },
        hotshot_testing::spinning_task::ChangeNode {
            idx: 15,
            updown: hotshot_testing::spinning_task::UpDown::Down,
        },
        hotshot_testing::spinning_task::ChangeNode {
            idx: 16,
            updown: hotshot_testing::spinning_task::UpDown::Down,
        },
    ];

    metadata.spinning_properties = SpinningTaskDescription {
        node_changes: vec![(std::time::Duration::new(4, 0), dead_nodes)],
    };
    metadata
        .gen_launcher::<hotshot_testing::node_types::SequencingTestTypes, hotshot_testing::node_types::SequencingMemoryImpl>()
        .launch()
        .run_test()
        .await;
}
