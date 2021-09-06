#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod nation{
    pub mod goverment{
        pub fn govern(){}
    }
    mod congress{
        pub fn legislate(){}
    }
    mod court{
        fn legislate(){
            super::congress::legislate()
        }
    }
}