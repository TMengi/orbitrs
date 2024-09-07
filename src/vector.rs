use crate::vector_ops;

pub struct Vector3 {
    elem: [f64; 3],
}

impl Vector3 {
    fn norm(&self) -> f64 {
        vector_ops::vector_2norm(&self.elem)
    }

    fn safe_normalize(&mut self) {
        vector_ops::safe_normalize(&mut self.elem)
    }
}
