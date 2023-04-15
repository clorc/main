use nalgebra::{base, Dyn, Dim, Const};
type DynamicMatrix = base::Matrix<i32, Dyn, Dyn, nalgebra::VecStorage<i32, Dyn, Dyn>>;
type DynamicVector = base::Matrix<i32, Dyn, Const<1>, nalgebra::VecStorage<i32, Dyn, Const<1>>>;


// the convolution
pub fn convolve(image: DynamicMatrix, filter: DynamicMatrix)
{
    let height = Dyn(image.nrows() + filter.nrows() - 1);
    let width = Dyn(image.ncols() + filter.ncols() - 1);

    let zero_padded: DynamicMatrix = zero_pad(filter, height, width);
    generate_2d_toeplitz(zero_padded, height.value(), width.value(), image.nrows(), image.ncols());
}

// compute the toeplitz matrix of a vector
fn generate_1d_toeplitz(vector: DynamicVector, ncols: usize) -> DynamicMatrix {
    // initializing our toeplitz with the correct dimensions
    let mut toeplitz: DynamicMatrix = base::Matrix::zeros_generic(Dyn(vector.nrows()), Dyn(ncols));

    // copying the vector we're computing the toeplitz for as mutable vector
    let mut sliding_vector: DynamicVector = base::Matrix::zeros_generic(Dyn(vector.nrows()), Const);
    sliding_vector.copy_from(&vector);

    
    for i in 0..=ncols-1 {
        toeplitz.set_column(i, &sliding_vector);

        // sliding down the entries of the sliding vector
        sliding_vector = sliding_vector.insert_row(0, 0);
        sliding_vector = sliding_vector.remove_row(vector.nrows());
    }

    toeplitz
}

// compute the toeplitz matrix of a vector
fn generate_2d_toeplitz(matrix: DynamicMatrix, ncols: usize, nrows:usize, image_rows: usize, image_cols: usize) {
    let mut toeplitz_vector: Vec<DynamicMatrix> = Vec::new();

    // computing the toeplitz of each row of the matrix
    for i in 0..matrix.nrows() {
        toeplitz_vector.push(generate_1d_toeplitz(matrix.row(matrix.nrows()-i-1).transpose(), ncols));
    }

    // initializing the block matrix
    let mut block_toeplitz: DynamicMatrix = base::Matrix::zeros_generic(Dyn(toeplitz_vector.len()*nrows), Dyn(image_rows*image_cols));
    
    for j in 0..image_rows {
        for i in 0..toeplitz_vector.len() {
            println!("block matrix coordinates are: ({}, {})", i,j);
            block_toeplitz.generic_view_mut((i*nrows, (image_rows-j-1)*ncols), (Dyn(nrows), Dyn(ncols))).copy_from(&toeplitz_vector[i]);
        }
    }
    
    println!("{}", block_toeplitz);
}

// resizing a matrix to new "height x width" size, with the old entries in the bottom right and zeros everywhere else
// height >= matrix.nrows and width >= matrix.ncols 
fn zero_pad(matrix: DynamicMatrix, height: Dyn, width: Dyn)
-> DynamicMatrix
{
    let matrix_rows = matrix.nrows();
    let matrix_cols = matrix.ncols();

    assert!((height.value() >= matrix_rows) && (width.value() >= matrix_cols), "Given Dimensions are too small to fit the matrix.");

    let padded: DynamicMatrix = matrix.insert_rows(0, height.value()-matrix_rows, 0);
    padded.insert_columns(matrix_cols, width.value()-matrix_rows, 0)
}

fn _vectorize_matrix() {

}