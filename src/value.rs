#[derive(Debug, Clone)]
pub enum Value {
    Str(String),
    Int(i64),
    Float(f64),
    Boolean(bool),
    List(Vec<Value>),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Str(a), Value::Str(b)) => a == b,
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::List(a), Value::List(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for Value {}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Str(s) => write!(f, "{}", s),
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::List(l) => {
                write!(f, "[")?;
                for (i, v) in l.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
        }
    }
}

impl Value {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::Str(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_str_mut(&mut self) -> Option<&mut String> {
        match self {
            Value::Str(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(i) => Some(*i),
            _ => None,
        }
    }

    pub fn as_int_mut(&mut self) -> Option<&mut i64> {
        match self {
            Value::Int(i) => Some(i),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            Value::Float(f) => Some(*f),
            _ => None,
        }
    }

    pub fn as_float_mut(&mut self) -> Option<&mut f64> {
        match self {
            Value::Float(f) => Some(f),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_bool_mut(&mut self) -> Option<&mut bool> {
        match self {
            Value::Boolean(b) => Some(b),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<&Vec<Value>> {
        match self {
            Value::List(l) => Some(l),
            _ => None,
        }
    }

    pub fn as_list_mut(&mut self) -> Option<&mut Vec<Value>> {
        match self {
            Value::List(l) => Some(l),
            _ => None,
        }
    }
}

impl Into<Value> for String {
    fn into(self) -> Value {
        Value::Str(self)
    }
}

impl Into<Value> for i64 {
    fn into(self) -> Value {
        Value::Int(self)
    }
}

impl Into<Value> for f64 {
    fn into(self) -> Value {
        Value::Float(self)
    }
}

impl Into<Value> for bool {
    fn into(self) -> Value {
        Value::Boolean(self)
    }
}

impl Into<Value> for Vec<Value> {
    fn into(self) -> Value {
        Value::List(self)
    }
}
