use auto_impl::auto_impl;

#[auto_impl(&, Box)]
pub trait IntoDto<T> {
    fn into_dto(&self) -> T;
}
