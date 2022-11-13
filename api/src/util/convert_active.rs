use sea_orm::ActiveValue;

pub trait Convert<T>
where
    T: Into<sea_orm::Value>,
{
    fn convert(&self, default: T) -> ActiveValue<T>;
}
impl<T> Convert<T> for Option<T>
where
    T: Into<sea_orm::Value> + Clone,
{
    fn convert(&self, default: T) -> ActiveValue<T> {
        match self {
            Some(value) => ActiveValue::Set(value.clone()),
            None => ActiveValue::Unchanged(default),
        }
    }
}
