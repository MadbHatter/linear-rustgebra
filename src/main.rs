use linalg::Matrix;

fn main() {
    let mut m: Matrix = Matrix::new(3,3);
    m.identity();

    let mut mcpy = m.copy();
    mcpy.map(|x| x+3.0);
    mcpy.print(); 

    m.print();

    let m1 = Matrix::from_file("src/2b2.txt");
    m1.print();
    println!("det(m1) = {}", m1.det());

    let mut m2 = Matrix::from_file("src/m1.txt");
    m2.map(|i| i*2.0);
    m2.print();
    println!("det(m2) = {}", m2.det());

    let m3 = Matrix::from_string("1 2 3 ; 4 5 6 ; 7 8 9");
    m3.print();
    println!("det(m3) = {}", m3.det());

    let m4 = Matrix::from_string("9 8 4 ; 2 3 7; 4 1 1");
    m4.print();
    println!("det(m4) = {}", m4.det());

    let m4t = m4.transpose();
    m4t.print();

    let m5 = Matrix::from_string("3 0 2 ; 2 0 -2 ; 0 1 1 ");
    m5.print();

    let m5i = m5.inverse();
    println!("inverse of m5:");
    m5i.print();

}
