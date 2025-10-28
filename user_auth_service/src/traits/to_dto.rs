use auto_impl::auto_impl;

#[auto_impl(&, Box)]
pub trait ToDto<T> {
    fn to_dto(&self) -> T;
}
