pub trait ToDto<T> {
    fn to_dto(&self) -> T;
}
