use gf::GF;
use nalgebra::{DMatrix, Matrix, VecStorage};

fn main() {
    let m: DMatrix<u8> = nalgebra::dmatrix![1,1,1; 1,2,3; 1,4,9];

    println!("{}", m);

    let m: DMatrix<GF<u8>> = transmute_matrix(m);

    println!("{}", m);
    let inv = get_invers(m.clone());
    println!("{}", inv);
    println!("{}", m * inv);
}

fn inverse(matrix: DMatrix<u8>) -> DMatrix<u8> {
    let mut res = DMatrix::identity(matrix.nrows(), matrix.ncols());
    let mut tmp: DMatrix<GF<u8>> = transmute_matrix(matrix);

    for i in 0..tmp.ncols() {
        // if tmp[i][i] == 0 && !find_and_swap_nonzero_in_row(i, size, &mut tmp, &mut res) {
        //   size = size - 1;
        // }

        let element = GF::from(tmp[(i, i)].inverse());
        let mut t = tmp.index_mut((i, ..));
        let mut r = res.index_mut((i, ..));

        t *= element;
        r *= element;

        for j in 0..tmp.nrows() {
            if j == i {
                continue;
            }
            let coeff = *tmp.index((j, i));
            if coeff.0 == 0 {
                continue;
            }
            let (tmpi, mut tmpj) = tmp.rows_range_pair_mut(i, j);
            let (resi, mut resj) = res.rows_range_pair_mut(i, j);

            tmpj -= tmpi * coeff;
            resj -= resi * coeff;
        }
    }

    // we could assert here that tmp is now an identity matrix

    return transmute_matrix(res);
}

fn get_invers(m: DMatrix<GF<u8>>) -> DMatrix<GF<u8>> {
    assert_eq!(m.nrows(), m.ncols());
    let n = m.nrows();
    let mut augmented_matrix = DMatrix::zeros(n, n * 2);
    augmented_matrix.index_mut((.., ..n)).copy_from(&m);
    augmented_matrix
        .index_mut((.., n..))
        .copy_from(&DMatrix::identity(n, n));

    for i in 0..n {
        for j in 0..n {
            if j == i {
                continue;
            }
            let ratio = augmented_matrix[(j, i)] / augmented_matrix[(i, i)];
            for k in 0..(2 * n) {
                augmented_matrix[(j, k)] =
                    augmented_matrix[(j, k)] - (ratio * augmented_matrix[(i, k)]);
            }
        }
    }

    for (i, mut row) in augmented_matrix.row_iter_mut().enumerate() {
        row /= row[i];
    }

    // for i in 0..n {
    //     for j in n..(2 * n) {
    //         augmented_matrix[(i, j)] = augmented_matrix[(i, j)] / augmented_matrix[(i, i)];
    //     }
    // }

    augmented_matrix.remove_columns(0, n)
}

fn transmute_matrix<T, U>(Matrix { data, .. }: DMatrix<T>) -> DMatrix<U>
where
    T: std::fmt::Debug,
    U: std::fmt::Debug,
{
    let (nrows, ncols) = nalgebra::RawStorage::shape(&data);
    let v: Vec<T> = data.into();
    let (ptr, len, cap) = {
        let mut me = std::mem::ManuallyDrop::new(v);
        (me.as_mut_ptr(), me.len(), me.capacity())
    };

    let v = unsafe {
        // We can now make changes to the components, such as
        // transmuting the raw pointer to a compatible type.
        let ptr = ptr as *mut U;

        Vec::from_raw_parts(ptr, len, cap)
    };

    let data = VecStorage::new(nrows, ncols, v);
    DMatrix::<U>::from_data(data)
}
