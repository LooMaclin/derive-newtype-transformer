#[macro_use]
extern crate derive_newtype_transformer;

#[cfg(test)]
mod tests {

    #[derive(NewtypeTransformer)]
    struct Test(String);

    impl Test {
        fn new(value: String) -> Test {
            Test(value)
        }
    }

    #[derive(NewtypeTransformer)]
    struct Test1(Test);

    impl From<Test> for Test1 {

        fn from(value: Test) -> Test1 {
            Test1(value)
        }
    }

    #[derive(NewtypeTransformer)]
    struct Test2(Test1);

    impl From<Test1> for Test2 {

        fn from(value: Test1) -> Test2 {
            Test2(value)
        }
    }

    #[test]
    fn one_newtype() {
        let test = Test::new("abc".to_owned());
        let test_1 = Test1::from(test);
        let test_2 = Test2::from(test_1);

        assert_eq!("abc", test_2.as_test1().as_test().as_string());
    }
}
