use image::Luma;
use nalgebra::{DMatrix, DVector};

fn load_image_to_grayscale_matrix(path: &str) -> DMatrix<u8> {
    let img = image::open(path).unwrap().to_luma8();
    let (width, height) = img.dimensions();
    let pixels: Vec<u8> = img.pixels().map(|p| p.0[0]).collect();
    DMatrix::from_row_slice(height as usize, width as usize, &pixels)
}

fn save_matrix_as_image(matrix: &DMatrix<u8>, path: &str) {
    let width = matrix.ncols() as u32;
    let height = matrix.nrows() as u32;
    println!("w: {width} h: {height}");
    let mut img = image::GrayImage::new(width, height);

    // The iter impl:
    // Iterates through this matrix coordinates in column-major order.
    // That's why height is used.
    for (i, pixel) in matrix.iter().enumerate() {
        let x = (i / height as usize) as u32;
        let y = (i % height as usize) as u32;
        img.put_pixel(x, y, Luma([*pixel]));
    }

    img.save(path).unwrap();
}

fn main() {
    let img_matrix = load_image_to_grayscale_matrix("imgs/earth_lower_res.jpg");
    let matrix_f64: DMatrix<f64> = img_matrix.map(|x| x as f64);
    let svd = matrix_f64.svd(true, true);
    let u = svd.u.unwrap();
    let s = svd.singular_values;
    let v_t = svd.v_t.unwrap();

    let mut s_mat = DMatrix::<f64>::zeros(s.len(), s.len());
    let s_vector = DVector::from_row_slice(s.as_slice());
    s_mat.set_diagonal(&s_vector);

    let mut compressed = DMatrix::<f64>::zeros(u.nrows(), v_t.ncols());
    for i in 0..50 {
        let sigma = *s.get(i).unwrap();
        let u_col_i = u.column(i);
        let v_t_row_i = v_t.row(i);
        // Here it's not necessary to transpose v_t_row_i to compute the outer product.
        // [u_col_i](m x 1) * [v_t_row_i](1 x n) -> (m x n)
        compressed += sigma * u_col_i * v_t_row_i;
    }

    let img_reconstructed = u * s_mat * v_t;
    let img_reconstructed: DMatrix<u8> = img_reconstructed.map(|x| x as u8);
    save_matrix_as_image(&img_reconstructed, "imgs/grayscale.jpg");

    let img_compressed: DMatrix<u8> = compressed.map(|x| x as u8);
    save_matrix_as_image(&img_compressed, "imgs/grayscale_compressed.jpg");
}
