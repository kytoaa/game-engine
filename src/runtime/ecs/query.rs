use std::any;

pub struct QueryBuilder<T> {}
impl<T> QueryBuilder<T> {
    pub fn get() -> QueryBuilder<T> {
        QueryBuilder::<T> {}
    }
}

pub trait Query {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_builder_tests() {
        struct Test;

        let test = 3;
    }
}
