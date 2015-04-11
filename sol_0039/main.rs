#[macro_use] extern crate libeuler;

/// If p is the perimeter of a right angle triangle with integral length sides, {a,b,c}, there are
/// exactly three solutions for p = 120.
///
/// {20,48,52}, {24,45,51}, {30,40,50}
///
/// For which value of p ≤ 1000, is the number of solutions maximised?
fn main() {
    solutions! {
        sol naive {
            let mut max = 0;
            let mut maxval = 0;
            for i in 0..1000 {
                let triangles = get_triangles(i);
                if triangles.len() > max {
                    max = triangles.len();
                    maxval = i;
                }
            }

            maxval
        }
    }
}

#[derive(Debug)]
struct Triangle {
    one: i64,
    two: i64,
    three: i64
}

fn get_triangles(p: i64) -> Vec<Triangle> {
    let mut retval = Vec::new();

    for one in 1..p {
        for two in one..p {
            let three = p - one - two;
            if three as f64 == (one.pow(2) as f64 + two.pow(2) as f64).sqrt() {
                retval.push(Triangle {
                    one: one,
                    two: two,
                    three: three
                });
            }
        }
    }

    retval
}
