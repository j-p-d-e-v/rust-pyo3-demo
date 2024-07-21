
#[cfg(test)]

mod tests {
    use super::*;
    use pyo3::prelude::*;
    use pyo3::types::{PyDict, PyList};
    use std::fs;

    #[test]
    fn test_example_with_parameters(){
        Python::with_gil(|py| {
            let mut code = fs::read_to_string("storage/example_with_parameters.py").unwrap();
            let module = PyModule::from_code_bound(py, code.as_str(), "", "").unwrap();
            let args = ("arg1234",);
            let kwargs = PyDict::new_bound(py);
            let values: Vec<u8> = Vec::from([1,2,3,4,5,6,7,8]);
            let items = PyList::new_bound(py, values);
            kwargs.set_item("values",items);
            let result = module.call_method("main", args, Some(&kwargs)).unwrap();
            println!("Parameters Result: {:#?}",result);
            assert!(true);
        });
    }

    #[test]
    fn example_async(){
        Python::with_gil(|py| {
            let mut code = fs::read_to_string("storage/example_async.py").unwrap();
            let module = PyModule::from_code_bound(py, code.as_str(), "", "").unwrap();
            let func = module.getattr("main").unwrap();
            let result = func.call0().unwrap();
            println!("ASYNC Python: {:#?}",result);
            assert!(true);
        });
    }
}