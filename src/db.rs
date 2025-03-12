pub use oxidar_derive::Model;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct ModelQuery<T> {
    _marker: PhantomData<T>,
    model_name: String,
    fields: Vec<String>,
}

impl<T> ModelQuery<T> {
    pub fn new<A>(model_name: &str) -> ModelQuery<A> {
        ModelQuery {
            model_name: model_name.to_string(),
            fields: Vec::new(),
            _marker: PhantomData,
        }
    }

    // pub fn FilterEq(self, field: &str, )
    pub fn resolve(&self) -> T {
        todo!()
    }
}

pub trait Select<T> {
    fn s(self, select: T) -> Self;
}

impl<T> Select<&str> for ModelQuery<T> {
    fn s(self, select: &str) -> Self {
        let mut fields = self.fields;
        fields.push(select.to_string());

        return ModelQuery {
            model_name: self.model_name,
            fields,
            _marker: PhantomData,
        };
    }
}

impl<T> Select<&[&str]> for ModelQuery<T> {
    fn s(self, select: &[&str]) -> Self {
        let mut fields = self.fields;

        for select in select {
            fields.push(select.to_string());
        }

        return ModelQuery {
            model_name: self.model_name,
            fields,
            _marker: PhantomData,
        };
    }
}
