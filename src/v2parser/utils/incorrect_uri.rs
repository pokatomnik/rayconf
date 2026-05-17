use crate::{utils::to_err::ToAnyhow, v2parser::utils::constants::INCORRECT_URI_FORMAT_ERR_MSG};

pub(crate) trait IncorrectURI<T> {
    fn incorrect_uri(self) -> anyhow::Result<T>;
}

impl<T> IncorrectURI<T> for Option<T> {
    fn incorrect_uri(self) -> anyhow::Result<T> {
        self.anyhow(INCORRECT_URI_FORMAT_ERR_MSG)
    }
}

impl<T, E> IncorrectURI<T> for Result<T, E> {
    fn incorrect_uri(self) -> anyhow::Result<T> {
        self.anyhow(INCORRECT_URI_FORMAT_ERR_MSG)
    }
}
