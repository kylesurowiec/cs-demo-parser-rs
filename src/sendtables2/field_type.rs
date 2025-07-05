#[derive(Clone, Debug)]
pub struct FieldType {
    pub base_type: String,
    pub generic_type: Option<Box<FieldType>>,
    pub pointer: bool,
    pub count: usize,
}

impl FieldType {
    pub fn new(mut name: &str) -> Self {
        let mut pointer = false;
        if name.ends_with('*') {
            pointer = true;
            name = &name[..name.len() - 1];
        }
        let mut count = 0;
        if let Some(idx) = name.rfind('[')
            && name.ends_with(']') {
                if let Ok(n) = name[idx + 1..name.len() - 1].parse() {
                    count = n;
                }
                name = &name[..idx];
            }
        let generic_type = if let Some(start) = name.find('<') {
            if let Some(end) = name.rfind('>') {
                let inner = &name[start + 1..end];
                let base = &name[..start];
                name = base;
                Some(Box::new(FieldType::new(inner)))
            } else {
                None
            }
        } else {
            None
        };
        Self {
            base_type: name.to_string(),
            generic_type,
            pointer,
            count,
        }
    }
}
