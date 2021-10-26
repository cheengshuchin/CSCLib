use csclib::*;
use std::{any::Any, fmt::Debug, thread, time::{self, SystemTime}};

#[derive(Debug,PartialEq,Eq,Hash,Clone,Copy)]
pub struct CSCT
    {
        value:usize,
        core:usize,
    }
#[derive(Debug,PartialEq,Eq,Hash,Clone,Copy)]
pub struct CSCarg
    {
        a:CSCT,
        b:usize,
        c:usize,
    }
pub fn cfunc(a:CSCT,b:usize,c:usize)->(CSCT,usize)
    {
        let now;
        now=SystemTime::now();
        println!("Sleeping");
        thread::sleep(time::Duration::from_secs_f64(0.5));
        let rs;
        rs=(a,b+b*b+c*a.value);
        println!("elapsed {}s",now.elapsed().unwrap().as_secs_f64());
        rs
    }
pub fn wrap(argm:CSCarg)->(CSCT,usize)
    {
        cfunc(argm.a,argm.b,argm.c)
    }
pub fn wrapper<'a>(args:Vec<Box<dyn Any>>)->(&'a str,usize)
    {
        let (a,b,c);
        a=*args[0].downcast_ref::<&str>().unwrap();
        b=*args[1].downcast_ref::<usize>().unwrap();
        c=*args[2].downcast_ref::<usize>().unwrap();
        tfunc(a,b,c)
    }
pub fn tfunc(a:&str,b:usize,c:usize)->(&str,usize)
    {
        let now;
        now=SystemTime::now();
        println!("Sleeping !");
        thread::sleep(time::Duration::from_secs_f64(0.5));
        println!("Wake !");
        let rs;
        rs=(a,b+b*b+c);
        println!("elapsed {}s",now.elapsed().unwrap().as_secs_f64());
        rs
    } 
fn main() 
    {
        println!("Hello, world!");
        let cs;
        cs=CSCDARG
            ::new(wrapper)
            .set(("a",""))
            .set(("b",2_usize))
            .set(("a","CSC"))
            .set(("b",8_usize))
            .set(("c",3_usize))
            .set(("c",5_usize))
            .set(("t",13_usize))
            //.set(("a",7_i32))
            .call();
        println!("{:#?}",&cs);
        let mut kk;
        kk=CSCFCACHE::new(wrap)
            .cachetimeout(10.0);
        let mut rt;
        rt=kk.call(CSCarg{a:CSCT{value:12,core:4},b:1,c:2});
        println!("{:#?}",&rt);
        rt=kk.call(CSCarg{a:CSCT{value:12,core:4},b:1,c:2});
        println!("{:#?}",&rt);
        rt=kk.call(CSCarg{a:CSCT{value:13,core:3},b:1,c:5});
        println!("{:#?}",&rt);
        rt=kk.call(CSCarg{a:CSCT{value:4,core:3},b:1,c:5});
        println!("{:#?}",&rt);
        rt=kk.call(CSCarg{a:CSCT{value:12,core:3},b:1,c:2});
        println!("{:#?}",&rt);
        rt=kk.call(CSCarg{a:CSCT{value:4,core:3},b:1,c:5});
        println!("{:#?}",&rt);
        rt=kk.call(CSCarg{a:CSCT{value:13,core:3},b:1,c:5});
        println!("{:#?}",&rt);
        rt=kk.call(CSCarg{a:CSCT{value:12,core:3},b:1,c:2});
        println!("{:#?}",&rt);
        rt=kk.call(CSCarg
            {
                a:CSCT
                    {
                        value:12,
                        core:3
                    },
                b:1,
                c:2}
            );
        println!("{:#?}",&rt);
        let stp;
        stp=CSCFor::new()
            .start(0)
            .step(1)
            .end(10);
        println!("{:#?}",&stp);
        for ele in stp
            {
                println!("{}",ele);
            }
        println!("{:#?}",&stp);
        let sf;
        sf=CSCFor::new()
            .start(1.0)
            .end(10.0)
            .step(0.5);
        println!("{:#?}",&sf);
        for ele in sf
            {
                println!("{}",ele);
            }
        println!("{:#?}",&sf);
        let sk;
        sk=CSCFor::new()
            .start(0)
            .step(1)
            .end(20)
            .zip(CSCFor::new()
                .start(0)
                .step(1)
                .end(20)
                .skip(1)
                )
            .map(|(x,y)| ((x*y)/2+(y+x))/4+1);
        println!("{:#?}",&sk);
        for ele in sk.clone()
            {
                println!("{:#?}",ele);
            }
        println!("{:#?}",&sk);
    }
