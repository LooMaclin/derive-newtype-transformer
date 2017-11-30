#[macro_use]
extern crate derive_newtype_deref;

#[cfg(test)]
mod tests {

    #[derive(NewtypeDeref)]
    struct Test(String);

    impl Test {
        fn new(value: String) -> Test {
            Test(value)
        }
    }

    #[derive(NewtypeDeref)]
    struct Test1(Test);

    impl From<Test> for Test1 {

        fn from(value: Test) -> Test1 {
            Test1(value)
        }
    }

    #[derive(NewtypeDeref)]
    struct Test2(Test1);

    impl From<Test1> for Test2 {

        fn from(value: Test1) -> Test2 {
            Test2(value)
        }
    }

    #[derive(NewtypeDeref)]
    struct NewtypeOverVec(pub Vec<String>);

    #[test]
    fn newtype_over_vec() {
        let newtype_over_vec = NewtypeOverVec(vec!["a".to_string(), "b".to_string()]);
        for i in newtype_over_vec.iter() {
            println!("i: {}", i);
        }
    }

    #[test]
    fn one_newtype() {
        let test = Test::new("abc".to_owned());
        let test_1 = Test1::from(test);
        let test_2 = Test2::from(test_1);

        assert_eq!("abc", ***test_2);
    }
}
