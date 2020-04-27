use futures::future;
use tokio::prelude::*;

pub struct ShoutBox{}

impl Future for ShoutBox{
    type Item = String;
    type Error = ();
    fn poll(&mut self)->Result<Async<Self::Item>, Self::Error>{
        Ok(Async::Ready("hello".to_string()))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test] 
    fn it_works() {
        let f = ShoutBox {};
        let p = f.map(|s| println!("{}", s));

        println!("Beginning");

        tokio::run(p);

        println!("Ending");

        assert_eq!(2 + 2, 4);
    }
}
