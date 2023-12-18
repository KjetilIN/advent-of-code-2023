#[cfg(test)]
mod test{
    use crate::prediction::{Prediction, PredictionMethods};

    #[test]

    fn test_predict_next_1(){
        let number_string = "0 3 6 9 12 15";

        let prediction = match Prediction::with_numbers(number_string){
            Ok(val) => val,
            Err(_) =>{
                panic!("Failed test: Constructor was illegal ");
            }
        };

        let next = prediction.predict_next_number();

        assert_eq!(next, 18);

    }

    #[test]
    fn test_predict_next_2(){
        let number_string = "1 3 6 10 15 21";

        let prediction = match Prediction::with_numbers(number_string){
            Ok(val) => val,
            Err(_) =>{
                panic!("Failed test: Constructor was illegal ");
            }
        };

        let next = prediction.predict_next_number();

        assert_eq!(next, 28);

    }

    #[test]
    fn test_predict_next_3(){
        let number_string = "10 13 16 21 30 45";

        let prediction = match Prediction::with_numbers(number_string){
            Ok(val) => val,
            Err(_) =>{
                panic!("Failed test: Constructor was illegal ");
            }
        };

        let next = prediction.predict_next_number();

        assert_eq!(next, 68);

    }

    #[test]
    fn test_predict_first_1(){
        let number_string = "10 13 16 21 30 45";

        let prediction = match Prediction::with_numbers(number_string){
            Ok(val) => val,
            Err(_) =>{
                panic!("Failed test: Constructor was illegal ");
            }
        };

        let next = prediction.predict_first_number();

        assert_eq!(next, 5);

    }

}