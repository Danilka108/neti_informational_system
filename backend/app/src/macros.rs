#[macro_export]
macro_rules! dyn_dependency {
    ($dep:path) => {
        dyn_dependency!(Box: $dep)
    };

    (Box: $dep:path) => {
        ::std::boxed::Box<dyn $dep + Send + Sync>
    };

    (Arc: $dep:path) => {
        ::std::sync::Arc<dyn $dep + Send + Sync>
    };
}
