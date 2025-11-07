pub mod checked {

    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NegativeLogarithm,
        NegativeSquareRoot,
    }

    pub type MathResult = Result<f64, MathError>;

    fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }

    fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            let res = x.sqrt();
            Ok(res)
        }
    }

    fn ln(x: f64) -> MathResult {
        if x <= 0.0 {
            Err(MathError::NegativeLogarithm)
        } else {
            let res = x.ln();
            Ok(res)
        }
    }

    pub fn op(x: f64, y: f64) -> MathResult {
        match div(x, y) {
            Err(e) => panic!("{:?}", e),
            Ok(ratio) => match ln(ratio) {
                Err(e) => panic!("{:?}", e),
                Ok(ln) => match sqrt(ln) {
                    Err(e) => panic!("{:?}", e),
                    Ok(sqrt) => Ok(sqrt),
                },
            },
        }
    }

}