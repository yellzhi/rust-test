#[cfg(test)]
mod tests{
    use car14_cmd::search;

    #[test]
    fn search_test(){
        let dec = "   hello   \
         nfsdafn hello  \
         dfsabfdhasbfhj";
        let query = "hello";
        let ret = search(query, dec);
        println!("{:?}", ret)
    }
}