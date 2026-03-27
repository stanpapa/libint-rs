pub(crate) trait AsPin {
    type T;

    fn as_pin(&self) -> std::pin::Pin<&Self::T>;
}
