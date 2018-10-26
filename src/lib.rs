fn _exp(x: f32) -> f32 {
    x.exp()
}
pub fn sigmoid(x: f32) -> f32 {
    //1.0 / (1.0 + _exp(-x))
    /*NOTE: doing -x.exp() yeilds wrong answer
        let newx: f32 = -x;
        1.0 / (1.0 + newx.exp())
     */
    1.0 / (1.0 + (-x).exp())
}

#[cfg(test)]
mod tests {

    use super::sigmoid;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_sigmoid() {
        let input: f32 = 7.0;
        //let expect: f32 = -0.0009127142;
        let expect: f32 = 0.999089;
        assert_eq!(sigmoid(input), expect);
    }
}
