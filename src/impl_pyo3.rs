use super::ArcStr;
#[cfg(feature = "substr")]
use super::Substr;

use core::convert::Infallible;
use pyo3::{prelude::*, types::PyString};
use pyo3_stub_gen::{PyStubType, TypeInfo};

// Same to https://docs.rs/pyo3/0.23.3/src/pyo3/conversions/std/string.rs.html#188-201
impl<'py> IntoPyObject<'py> for ArcStr {
    type Target = PyString;
    type Output = Bound<'py, Self::Target>;
    type Error = Infallible;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        Ok(PyString::new(py, self.as_str()))
    }
}

// Same to https://docs.rs/pyo3/0.23.3/src/pyo3/conversions/std/string.rs.html#211-225
impl<'py> IntoPyObject<'py> for &ArcStr {
    type Target = PyString;
    type Output = Bound<'py, Self::Target>;
    type Error = Infallible;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        Ok(PyString::new(py, self.as_str()))
    }
}

// Same to https://docs.rs/pyo3/0.23.3/src/pyo3/conversions/std/string.rs.html#252-261
impl<'py> FromPyObject<'py> for ArcStr {
    #[inline]
    fn extract_bound(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        obj.downcast::<PyString>()?.to_cow().map(ArcStr::from)
    }
}

impl PyStubType for ArcStr {
    #[inline]
    fn type_output() -> TypeInfo {
        String::type_output()
    }
}

// Same to https://docs.rs/pyo3/0.23.3/src/pyo3/conversions/std/string.rs.html#188-201
#[cfg(feature = "substr")]
impl<'py> IntoPyObject<'py> for Substr {
    type Target = PyString;
    type Output = Bound<'py, Self::Target>;
    type Error = Infallible;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        Ok(PyString::new(py, self.as_str()))
    }
}

// Same to https://docs.rs/pyo3/0.23.3/src/pyo3/conversions/std/string.rs.html#211-225
#[cfg(feature = "substr")]
impl<'py> IntoPyObject<'py> for &Substr {
    type Target = PyString;
    type Output = Bound<'py, Self::Target>;
    type Error = Infallible;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        Ok(PyString::new(py, self.as_str()))
    }
}

// Same to https://docs.rs/pyo3/0.23.3/src/pyo3/conversions/std/string.rs.html#252-261
#[cfg(feature = "substr")]
impl<'py> FromPyObject<'py> for Substr {
    #[inline]
    fn extract_bound(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        obj.downcast::<PyString>()?.to_cow().map(Substr::from)
    }
}

#[cfg(feature = "substr")]
impl PyStubType for Substr {
    #[inline]
    fn type_output() -> TypeInfo {
        String::type_output()
    }
}
