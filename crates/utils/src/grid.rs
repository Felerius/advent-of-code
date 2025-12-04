use ndarray::{ArrayView2, ShapeBuilder};

pub fn from_lines(lines: &str) -> ArrayView2<'_, u8> {
    let lines = lines.trim_end();
    let width = lines.lines().next().map_or(0, str::len);
    let height = lines.len().div_ceil(width + 1);
    assert_eq!(lines.len(), height * (width + 1) - 1);
    debug_assert!(lines.lines().all(|line| line.len() == width));

    ArrayView2::from_shape((height, width).strides((width + 1, 1)), lines.as_bytes())
        .expect("data size error")
}
