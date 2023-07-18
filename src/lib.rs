use pyo3::prelude::*;


/// Функция скорости движения людского потока через проем
///
/// # Arguments
/// * `projection_area` - площадь проекции элемента
/// * `width` - ширина проема
/// * `density` - плотность людского потока в элементе, для которого определяется скорость
///
/// # Returns
/// Скорость, м/мин
#[pyfunction]
fn speed_through_transit(projection_area: f64, width: f64, density: f64) -> PyResult<f64> {
    let mut v0 = 100.0;
    let a = 0.295;
    let d0 = 0.65;

    if density > d0 {
      let d = density * projection_area;

      // m = 1 if D <= 0.5 else 1.25 - 0.5 * D
      let m = match d <= 0.5 {
        true => 1.0,
        false => 1.25 - 0.5 * d
      };

      let velocity = velocity(v0, a, d0, density).expect("Velocity calculation error");
      let mut q = velocity * d * m;

      if d >= 0.9 {
        q = match width < 1.6 {
          true => 2.5 + 3.75 * width,
          false => 8.5
        }
      }

      v0 = q / d
    }

    Ok(v0)
}

/// Функция скорости. Базовая зависимость, которая позволяет определить скорость людского
/// потока по его плотности
///
/// # Parameters
/// * `v0` - начальная скорость потока, м./мин.
/// * `a` - коэффициент вида пути
/// * `d0` - допустимая плотность людского потока на участке, чел./м2
/// * `d` - текущая плотность людского потока на участке, чел./м2
///
/// # Return
/// Скорость людского потока, м/мин
#[pyfunction]
fn velocity(v0: f64, a: f64, d0: f64, d: f64) -> PyResult<f64> {
  Ok(v0 * (1.0 - a * (d / d0).ln()))
}

/// A Python module implemented in Rust.
#[pymodule]
fn evacuation_py_utils(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(speed_through_transit, m)?)?;
    m.add_function(wrap_pyfunction!(velocity, m)?)?;
    Ok(())
}
