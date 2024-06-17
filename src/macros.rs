#[macro_export]
macro_rules! update_fields {
    ($dto:expr, $model:expr, { $($field:ident),* $(,)? }) => {
        $(
            if let Some(value) = $dto.$field.clone() {
                $model.$field = Set(value.into());
            }
        )*
    };
}