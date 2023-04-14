use nalgebra::{base::Matrix, Dyn, Dim, Const};


pub fn convolve<S>(image: Matrix<i32, Dyn, Dyn, S>, filter: Matrix<i32, Dyn, Dyn, S>)
where S: nalgebra::Storage<i32, Dyn, Dyn>
{
    let height:Dyn = Dyn(image.nrows() + filter.nrows() - 1);
    let width:Dyn = Dyn(image.ncols() + filter.ncols() - 1);

    let zero_padded: Matrix<i32, Dyn, Dyn, nalgebra::VecStorage<i32, Dyn, Dyn>> = zero_pad(filter, height, width);
    // let doubly_blocked_toeplitz: Vec<Matrix<i32, Dyn, Dyn, nalgebra::VecStorage<i32, Dyn, Dyn>>> = generate_2d_toeplitz(zero_padded);
}

fn generate_1d_toeplitz(vector: Matrix<i32, Dyn, Const<1>, nalgebra::VecStorage<i32, Dyn, Const<1>>>, ncols: usize)
-> Matrix<i32, Dyn, Dyn, nalgebra::VecStorage<i32, Dyn, Dyn>>
{
    let mut toeplitz: Matrix<i32,Dyn,Dyn, nalgebra::VecStorage<i32, Dyn, Dyn>> = Matrix::zeros_generic(Dyn(vector.nrows()), Dyn(ncols));
    let mut sliding_vector: Matrix<i32, Dyn, Const<1>, nalgebra::VecStorage<i32, Dyn, Const<1>>> = Matrix::zeros_generic(Dyn(vector.nrows()), Const);
    sliding_vector.copy_from(&vector);

    
    for i in 0..=ncols-1 {
        toeplitz.set_column(i, &sliding_vector);
        sliding_vector = sliding_vector.insert_row(0, 0);
        sliding_vector = sliding_vector.remove_row(vector.nrows());
    }

    toeplitz
}

fn _generate_2d_toeplitz<S>(_matrix: Matrix<i32, Dyn, Dyn, S>) {

}

fn zero_pad<S>(matrix: Matrix<i32, Dyn, Dyn, S>, height: Dyn, width: Dyn)
-> Matrix<i32, Dyn, Dyn, nalgebra::VecStorage<i32, Dyn, Dyn>>
where S: nalgebra::Storage<i32, Dyn, Dyn>
{
    let matrix_rows = matrix.nrows();
    let matrix_cols = matrix.ncols();

    let padded: Matrix<i32, Dyn, Dyn, nalgebra::VecStorage<i32, Dyn, Dyn>> = matrix.insert_rows(0, height.value()-matrix_rows, 0);
    padded.insert_columns(matrix_cols, width.value()-matrix_rows, 0)
}

fn _vectorize_matrix() {

}