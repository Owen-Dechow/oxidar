use std::{collections::HashMap, rc::Rc};

#[derive(Debug, Clone)]
pub enum TemplateVar {
    Str(String),
    Num(f32),
    Bool(bool),
    None,
    Indexable(HashMap<&'static str, TemplateVar>),
}

impl TemplateVar {
    fn split_first_dot(input: &str) -> (&str, &str) {
        if let Some(index) = input.find('.') {
            let first_part = &input[..index];
            let second_part = &input[index + 1..];
            (first_part, second_part)
        } else {
            (input, "")
        }
    }

    pub fn string(&self) -> String {
        match self {
            TemplateVar::Str(s) => s.to_string(),
            TemplateVar::Num(n) => n.to_string(),
            TemplateVar::Bool(b) => b.to_string(),
            TemplateVar::None => "None".to_string(),
            TemplateVar::Indexable(..) => {
                todo!()
            }
        }
    }

    pub fn resolve(&self, key: &str) -> &TemplateVar {
        let (a, b) = Self::split_first_dot(key);

        if a == "" && b == "" {
            return self;
        }

        match self {
            TemplateVar::Indexable(hash_map) => {
                if a == "" {
                    todo!()
                }

                return match hash_map.get(a) {
                    Some(s) => s.resolve(b),
                    None => todo!(),
                };
            }
            _ => todo!(),
        }
    }
}

macro_rules! from_template_var_num {
    ($from:ty) => {
        impl From<&$from> for TemplateVar {
            fn from(value: &$from) -> Self {
                TemplateVar::Num(value.clone() as f32)
            }
        }

        impl From<$from> for TemplateVar {
            fn from(value: $from) -> Self {
                TemplateVar::Num(value as f32)
            }
        }
    };
}

from_template_var_num!(i8);
from_template_var_num!(i16);
from_template_var_num!(i32);
from_template_var_num!(i64);
from_template_var_num!(i128);

from_template_var_num!(u8);
from_template_var_num!(u16);
from_template_var_num!(u64);
from_template_var_num!(u128);

from_template_var_num!(f32);
from_template_var_num!(f64);
from_template_var_num!(usize);

macro_rules! from_template_var_str {
    ($from:ty) => {
        impl From<$from> for TemplateVar {
            fn from(value: $from) -> Self {
                TemplateVar::Str(value.to_string())
            }
        }
    };
}

from_template_var_str!(&String);
from_template_var_str!(String);
from_template_var_str!(char);
from_template_var_str!(&char);
from_template_var_str!(&str);

pub trait ToTemplateVar {
    fn to_template_var(&self) -> TemplateVar;
}

impl<T> From<&T> for TemplateVar
where
    T: ToTemplateVar,
{
    fn from(value: &T) -> Self {
        value.to_template_var()
    }
}

impl<T> From<&Option<T>> for TemplateVar
where
    TemplateVar: From<T>,
    T: Clone,
{
    fn from(value: &Option<T>) -> Self {
        match value {
            Some(s) => TemplateVar::from(s.clone()),
            None => TemplateVar::None,
        }
    }
}

impl<T> From<&Rc<T>> for TemplateVar
where
    TemplateVar: From<T>,
    T: Clone,
{
    fn from(value: &Rc<T>) -> Self {
        TemplateVar::from((**value).clone())
    }
}
