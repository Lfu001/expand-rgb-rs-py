use numpy::ndarray::s;
use numpy::{PyArray3, PyArrayMethods, ToPyArray};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

const K1: (i32, i32) = (1, 0);
const K2: (i32, i32) = (-1, 0);
const K3: (i32, i32) = (0, 1);
const K4: (i32, i32) = (0, -1);

const P0: [(i32, i32); 4] = [K1, K2, K3, K4];
const P1: [(i32, i32); 4] = [K1, K2, K4, K3];
const P2: [(i32, i32); 4] = [K1, K3, K2, K4];
const P3: [(i32, i32); 4] = [K1, K3, K4, K2];
const P4: [(i32, i32); 4] = [K1, K4, K2, K3];
const P5: [(i32, i32); 4] = [K1, K4, K3, K2];
const P6: [(i32, i32); 4] = [K2, K1, K3, K4];
const P7: [(i32, i32); 4] = [K2, K1, K4, K3];
const P8: [(i32, i32); 4] = [K2, K3, K1, K4];
const P9: [(i32, i32); 4] = [K2, K3, K4, K1];
const P10: [(i32, i32); 4] = [K2, K4, K1, K3];
const P11: [(i32, i32); 4] = [K2, K4, K3, K1];
const P12: [(i32, i32); 4] = [K3, K1, K2, K4];
const P13: [(i32, i32); 4] = [K3, K1, K4, K2];
const P14: [(i32, i32); 4] = [K3, K2, K1, K4];
const P15: [(i32, i32); 4] = [K3, K2, K4, K1];
const P16: [(i32, i32); 4] = [K3, K4, K1, K2];
const P17: [(i32, i32); 4] = [K3, K4, K2, K1];
const P18: [(i32, i32); 4] = [K4, K1, K2, K3];
const P19: [(i32, i32); 4] = [K4, K1, K3, K2];
const P20: [(i32, i32); 4] = [K4, K2, K1, K3];
const P21: [(i32, i32); 4] = [K4, K2, K3, K1];
const P22: [(i32, i32); 4] = [K4, K3, K1, K2];
const P23: [(i32, i32); 4] = [K4, K3, K2, K1];

/// Returns a random ordering of the 4 directions.
fn random_permutation() -> [(i32, i32); 4] {
    let i = rand::random::<u8>() % 24;
    match i {
        0 => P0,
        1 => P1,
        2 => P2,
        3 => P3,
        4 => P4,
        5 => P5,
        6 => P6,
        7 => P7,
        8 => P8,
        9 => P9,
        10 => P10,
        11 => P11,
        12 => P12,
        13 => P13,
        14 => P14,
        15 => P15,
        16 => P16,
        17 => P17,
        18 => P18,
        19 => P19,
        20 => P20,
        21 => P21,
        22 => P22,
        23 => P23,

        _ => unreachable!(),
    }
}

/// Processes the image with the given number of iterations.
///
/// # Arguments
/// * `image` - The image to process. Must be a 3D array (H, W, C) of 8-bit unsigned integers.
/// * `iterations` - The number of iterations to run.
#[pyfunction]
fn process<'py>(
    py: Python<'py>,
    image: &'py Bound<'py, PyArray3<u8>>,
    iterations: usize,
) -> Bound<'py, PyArray3<u8>> {
    let image = unsafe { image.as_array_mut() };
    let mut tmp = image.to_owned();

    let height: i32 = image.shape()[0] as i32;
    let width: i32 = image.shape()[1] as i32;
    for _ in 0..iterations {
        let mut result = tmp.to_owned();

        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let mut color = tmp.slice(s![y as usize, x as usize, ..]);
                if let Some(0) = color.get(3) {
                    let order = random_permutation();
                    for (i, j) in order {
                        if tmp[[(y + j) as usize, (x + i) as usize, 3]] != 0 {
                            color = tmp.slice(s![y + j, x + i, ..]);
                            break;
                        }
                    }
                }
                result.slice_mut(s![y, x, ..]).assign(&color);
            }
        }

        tmp.assign(&result);
    }
    tmp.slice_mut(s![.., .., 3..])
        .assign(&image.slice(s![.., .., 3..]));
    tmp.to_pyarray_bound(py)
}

/// A Python module implemented in Rust.
#[pymodule]
fn expand_rgb(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(process, m)?)?;
    Ok(())
}
