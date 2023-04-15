use nalgebra::{base, Dyn, Dim, Const};
type DynamicMatrix = base::Matrix<i32, Dyn, Dyn, nalgebra::VecStorage<i32, Dyn, Dyn>>;
type DynamicVector = base::Matrix<i32, Dyn, Const<1>, nalgebra::VecStorage<i32, Dyn, Const<1>>>;
type Shape = (Dyn, Dyn);

// 2 dimensional convolution between two two matrices using Toeplitz Matrices
// algorithm source: https://assets.researchsquare.com/files/rs-2195496/v1_covered.pdf?c=1666848517
// output: (m-p+1) x (n-q+1) matrix
pub fn convolve_2d(image: DynamicMatrix, kernel: DynamicMatrix) -> DynamicMatrix {
    let image_shape: Shape = (Dyn(image.nrows()), Dyn(image.ncols())); 
    let kernel_shape: Shape = (Dyn(kernel.nrows()), Dyn(kernel.ncols())); 

    // computing the toeplitz of the kernel
    let toeplitz: DynamicMatrix = kernel_toeplitz(kernel, &image_shape, &kernel_shape);

    // computing the special row matrix of the image
    let row_matrix: DynamicMatrix = flattened_row_matrix(image, &image_shape, &kernel_shape);

    // this matrix product is the convolution of image and kernel
    row_matrix * toeplitz
}

// computing the toeplitz of a pxq kernel matrix for a mxn image matrix
// output: np x (n-q+1) toeplitz matrix
fn kernel_toeplitz(kernel: DynamicMatrix, image_shape: &Shape, kernel_shape: &Shape) -> DynamicMatrix {
    // shortcuts for our dimensions
    let (_, n) = image_shape;
    let (p, q) = kernel_shape;

    // initializing the toeplitz with correct dimensions
    let new_dimensions: Shape = (Dyn(n.value() * p.value()), Dyn(n.value() - q.value() + 1));
    let mut toeplitz: DynamicMatrix = base::Matrix::zeros_generic(new_dimensions.0, new_dimensions.1);

    // padding out the kernel with zeros
    let mut extended_kernel: DynamicMatrix = kernel;
    extended_kernel = extended_kernel.insert_rows(p.value(), n.value() - q.value(), 0);

    // flattening the extended kernel into a vector
    let mut sliding_column: DynamicVector = flattened(extended_kernel, new_dimensions.0);
    
    //computing the final toeplitz
    for i in 0..new_dimensions.1.value() {
        toeplitz.set_column(i, &sliding_column);
        
        // sliding down the entries of the sliding column
        sliding_column = sliding_column.insert_row(0, 0);
        sliding_column = sliding_column.remove_row(new_dimensions.0.value());
    }

    toeplitz
}

// computing a matrix whose columns consist of certain rows of the image that were flattened.
// output: (m-p+1) x np matrix
fn flattened_row_matrix(image: DynamicMatrix, image_shape: &Shape, kernel_shape: &Shape) -> DynamicMatrix {
    // shortcuts for our dimensions
    let (m, n) = image_shape;
    let (p, _) = kernel_shape;

    // initializing the row_matrix with correct dimensions
    let new_dimensions: Shape = (Dyn(m.value() - p.value() + 1), Dyn(n.value()*p.value()));
    let mut row_matrix: DynamicMatrix = base::Matrix::zeros_generic(new_dimensions.0, new_dimensions.1);

    // flattening the image matrix
    let mut flattened: DynamicVector = flattened(image.transpose(), Dyn(n.value()*m.value()));

    // computing the row matrix
    for (k, mut row) in row_matrix.row_iter_mut().enumerate() {
        let mut index = k*n.value() ;
        for element in row.iter_mut() {
            let curr = flattened.get_mut(index).unwrap();
            *element = *curr;
            index += 1;
        }
    }

    row_matrix
}

// flattens a given matrix into a vector
fn flattened(mut matrix: DynamicMatrix, length: Dyn) -> DynamicVector {
    let mut flattened_matrix: DynamicVector = base::Matrix::zeros_generic(length, Const);
    let mut index = 0;

    for element in flattened_matrix.iter_mut() {
        let curr = matrix.get_mut(index).unwrap();
        *element = *curr;
        index+=1;
    }

    flattened_matrix
}