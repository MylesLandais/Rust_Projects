use futures::future;
use tokio::prelude::*;
use tokio_channel::{oneshot, mpsc};
use std::fmt::Debug;


pub enum Request<M>{
    Put(M),
    Since(usize,oneshot::Sender<Vec<M>>),

}

pub struct ShoutBox<M>{
    store:Vec<M>,
    ch_r: mpsc::Receiver<Request<M>>,
}

impl<M> ShoutBox<M>{
    pub fn new()-> (Self,mpsc::Sender<Request<M>>){
        let (ch_s,ch_r) = mpsc::channel(10);
        (
            ShoutBox{store: Vec::new(), ch_r},
            ch_s,
        )
    }
}

impl<M: Debug + Clone> Future for ShoutBox<M>{
    type Item = ();
    type Error = ();
    fn poll(&mut self)->Result<Async<Self::Item>, Self::Error>{
        loop{
            let rq = match{self.ch_r.poll()?}{
                Async::NotReady =>return Ok(Async::NotReady),
                Async::Ready(Some(v))=>v,
                Async::Ready(None)=>return Ok(Async::Ready(())),
            };
            match rq{
                Request::Put(m)=>{
                    println!("You got mail {:?}",m); 
                    self.store.push(m);
                }
                Request::Since(n, ch) => {
                    println!("You got request {:?}", n);
                    let res = if n >= self.store.len(){
                        Vec::new()
                    } else {
                        Vec::from(&self.store[n..])
                    };
                    ch.send(res).ok();
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use futures::future::lazy;

    #[test] 
    fn it_works() {
        let f = lazy(||{
            let (f,ch_s) = ShoutBox::new();
            tokio::spawn(f);
            for i in 0 .. 5 {
                let ch2 = ch_s.clone();
                let (os_s,os_r) = oneshot::channel();
                let f2 = ch_s
                    .clone()
                    .send(Request::Put(3))
                    .and_then(|_| ch2.send(Request::Since(0, os_s)))
                    .map_err(|e| println!("{:?}",e))
                    .and_then(|_| os_r.map_err(|_|()))
                    .map( move |res| println!("Res {} = {:?}",i,res))
                    .map_err(|e| println!("{:?}",e));
                tokio::spawn(f2);
            }
            Ok(())
        });

        tokio::run(f);
    }
}
