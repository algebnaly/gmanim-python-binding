use pyo3::prelude::*;

#[pyclass(subclass)]
#[derive(Clone)]
struct Base {
    val: i32,
}

#[pyclass(extends=Base)]
struct Child {}

#[pymethods]
impl Child {
    #[new]
    fn new(val: i32) -> (Self, Base) {
        (Child {}, Base { val })
    }
}

#[pyfunction]
fn take_base(base: Base) -> i32 {
    base.val
}

fn main() {
    pyo3::Python::with_gil(|py| {
        let child_cls = py.get_type::<Child>();
        let child = child_cls.call1((42,)).unwrap();
        // try to extract Base
        let base: Base = child.extract().unwrap();
        assert_eq!(base.val, 42);
        println!("Success");
    });
}
