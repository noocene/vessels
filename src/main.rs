use vessels::{core, core::Executor, export};

export! {
    let mut executor = core::<Executor>().unwrap();
    executor.spawn(Box::pin(async move {
        _EXPORT_safe_output(vec![202, 253]);
    }));
}
