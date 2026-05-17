pub(crate) trait ToAnyhow<T> {
    fn anyhow(self, description: &'static str) -> anyhow::Result<T>;
}

impl<T> ToAnyhow<T> for Option<T> {
    fn anyhow(self, description: &'static str) -> anyhow::Result<T> {
        self.ok_or_else(|| anyhow::Error::msg(description))
    }
}

impl<T, E> ToAnyhow<T> for Result<T, E> {
    fn anyhow(self, description: &'static str) -> anyhow::Result<T> {
        self.map_err(|_| anyhow::Error::msg(description))
    }
}
