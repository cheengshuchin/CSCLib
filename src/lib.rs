//! # CSC Lib Crate
//!
//! `CSCLib`:
//! a collection of utilities to make 
//! life easy and more convenient.
//! :)
use std::{any::Any,collections::HashMap,time::{self, SystemTime},hash::Hash};
pub use self::CSCForLoop::*;
pub use self::CSCDefaultFunc::*;
pub use self::CSCCacheFunc::*;

/// Module for Function With Dynamic Default & Parameter
pub mod CSCDefaultFunc
  {
    use super::*;
    /// CSCDARG is Structure Data Type to Keep Track Parameter Changes
    /// Struct for CSCDARG
    #[derive(Debug)]
    pub struct CSCDARG<'a,FT>
      {
        func:FT,
        argn:Vec<&'a str>,
        argh:HashMap<&'a str,Box<dyn Any>>,
      }
    /// CSCDARG Self Implement
    impl<'a,FT> CSCDARG<'a,FT>
      {
        /// CSCDARG Constructor
        /// CSCDARG::new(wrapper_func)
        /// # example:
        /// ```
        ///   use csclib::*;
        ///   pub fn wrapper<'a>(args:Vec<Box<dyn Any>>)->(&'a str,usize)
        ///     {
        ///       let (a,b,c);
        ///       a=*args[0].downcast_ref::<&str>().unwrap();
        ///       b=*args[1].downcast_ref::<usize>().unwrap();
        ///       c=*args[2].downcast_ref::<usize>().unwrap();
        ///       tfunc(a,b,c)
        ///     }
        ///   pub fn tfunc(a:&str,b:usize,c:usize)->(&str,usize)
        ///     {
        ///       (a,b+b*b+c)
        ///     }
        ///   let itr;
        ///   itr=CSCDARG::new(wrapper)
        ///     .set(("a",""))  // set default value for parameter 'a'
        ///     .set(("b",2_usize)) // set default value for parameter 'b'
        ///     .set(("a","CSC")) // set value for parameter 'a'
        ///     .set(("b",8_usize)) //  set value for parameter 'b'
        ///     .set(("c",3_usize)) // set default value for parameter 'c'
        ///     .set(("c",5_usize)) // set value for parameter 'c'
        ///     .set(("t",13_usize)) // set default value for parameter 't', which never use
        ///     .set(("a",7_i32)) // set invalid value for parameter 'a' which cause panic
        ///     .call(); // return final result of execute wrapper function
        ///   println!("{:?}",itr);
        /// ```
        pub fn new(func:FT)->CSCDARG<'a,FT>
          {
            CSCDARG
              {
                func:func,
                argn:vec![],
                argh:HashMap::new(),
              }
          }
        /// Helper Functon to Set or Modify Position Parameter
        /// Each Parameter Have to  Follow Position With Format ("parameter_name",Assigned Repective Value)
        /// # example:
        /// ```
        ///   // --snip-- 
        ///   let itr;
        ///   itr=CSCDARG::new(wrapper)
        ///     .set(("a",""))  // set default value for parameter 'a'
        ///     // --snip-- 
        ///     .set(("t",13_usize)) // set default value for parameter 't', which never use
        /// ```
        pub fn set<G>(mut self,arg:(&'a str,G))->Self
          where
            G:'static
          {
            if !self.argn.contains(&arg.0)
              {
                self.argn.push(arg.0);
              }
            self.argh.insert(arg.0,Box::new(arg.1));
            self
          }
        /// Action Functon to Execute Wrapper 
        /// # example:
        /// ```
        ///   // --snip-- 
        ///   let itr;
        ///   itr=CSCDARG::new(wrapper)
        ///     .set(("a",""))  // set default value for parameter 'a'
        ///     // --snip-- 
        ///     .set(("t",13_usize)) // set default value for parameter 't', which never use
        ///     .call(); // return final result of execute wrapper function
        ///   println!("{:?}",itr);
        /// ```
        pub fn call<VT>(mut self)->VT
          where
          FT:Fn(Vec<Box<dyn Any>>)->VT,
          {
            let mut vs:Vec<Box<dyn Any>>;
            vs=vec![];
            for ky in self.argn
              {
                let dt=self.argh.remove(ky);
                match dt
                  {
                    Some(vl) => 
                      {
                        vs.push(vl);
                      },
                    None => {},
                  }
              }
            (self.func)(vs)
          }
      }
  }
/// Module to Make Function Cachable With TimeOut and Auto Memory Clean Up 
pub mod CSCCacheFunc
  {
    use super::*;
    /// CSCFCACHE is Structure Data Type to Keep Track Arg and Cache Changes
    /// Struct for CSCFCACHE
    #[derive(Debug)]
    pub struct CSCFCACHE<FT,KT,VT>
      where
        FT:Fn(KT)->VT,
      {
        func:FT,
        cachetimeout:f64,
        cachetime:HashMap<time::SystemTime,KT>,
        argtime:HashMap<KT,time::SystemTime>,
        value:HashMap<KT,VT>,
      }
    /// CSCFCACHE Self Implement
    impl<FT,KT,VT> CSCFCACHE<FT,KT,VT>
      where
        FT:Fn(KT)->VT,
      {
        /// CSCFCACHE Constructor
        /// CSCFCACHE::new(wrapper_func)
        /// # example:
        /// ```
        /// use csclib::*;
        /// #[derive(Debug,PartialEq,Eq,Hash,Clone,Copy)]
        /// pub struct CSCT
        ///   {
        ///     value:usize,
        ///     core:usize,
        ///   }
        /// pub fn cfunc(a:CSCT,b:usize,c:usize)->(CSCT,usize)
        ///   {
        ///     (a,b+b*b+c*a.value)
        ///   }
        /// pub fn wrap(argm:(CSCT,usize,usize))->(CSCT,usize)
        ///   {
        ///     let (a,b,c)=argm;
        ///     cfunc(a,b,c)
        ///   }
        /// let kk;
        /// kk=CSCFCACHE::new(wrap)
        ///   .cachetimeout(10.0);
        /// let mut rt;
        /// rt=kk
        ///   .call((CSCT{value:12,core:4},1,2));
        /// println!("{:?}",&rt);
        /// rt=kk
        ///   .call((CSCT{value:12,core:5},1,2));
        /// println!("{:?}",&rt);
        /// rt=kk
        ///   .call((CSCT{value:12,core:4},1,2));
        /// println!("{:?}",&rt);
        /// ```
        pub fn new(func:FT)->CSCFCACHE<FT,KT,VT>
          {
            CSCFCACHE 
              {
                func: func,
                cachetimeout:f64::default(),
                cachetime:HashMap::new(),
                argtime:HashMap::new(),
                value: HashMap::new(),
              }
          }
        /// Helper Functon to Set or Modify Cache TimeOut
        /// # example:
        /// ```
        /// // --snip-- 
        /// let kk;
        /// kk=CSCFCACHE::new(wrap)
        ///   .cachetimeout(10.0);
        /// ```
        pub fn cachetimeout (mut self, cachetimeout:f64)->Self
          {
            self.cachetimeout=cachetimeout;
            self
          }
        /// Action Functon to Execute Wrapper 
        /// # example:
        /// ```
        /// // --snip-- 
        /// let mut rt;
        /// rt=kk
        ///   .call((CSCT{value:12,core:4},1,2));
        /// println!("{:?}",&rt);
        /// rt=kk
        ///   .call((CSCT{value:12,core:5},1,2));
        /// println!("{:?}",&rt);
        /// rt=kk
        ///   .call((CSCT{value:12,core:4},1,2));
        /// println!("{:?}",&rt);
        /// ```
        pub fn call(&mut self,args:KT)->VT
          where
            KT:Eq+Hash+Copy,
            VT:Copy,
          {
            if self.cachetimeout!=0.0
              {
                {
                  let mut rv:Vec<(time::SystemTime,KT)>;
                  rv=vec![];
                  for (syt,arg) in self.cachetime.iter() 
                    {
                      match syt.elapsed() 
                        {
                          Ok(elapsed) => 
                            {
                              if elapsed.as_secs_f64()>self.cachetimeout
                                {
                                  rv.push((*syt,*arg));
                                }
                            },
                          Err(err) => 
                            {
                              println!("Error: {:#?}",err);
                            },
                        }
                    }
                  for ele in rv 
                    {
                      self.cachetime.remove(&ele.0);
                      self.argtime.remove(&ele.1);
                      self.value.remove(&ele.1);
                    }
                }
                {
                  let rs;
                  if self.value.contains_key(&args)
                    {
                      rs=self.value[&args];
                    }else{
                      rs=(self.func)(args);
                    };
                  {
                    let now;
                    let snowc;
                    now=SystemTime::now();
                    snowc=self.argtime.insert(args,now);
                    match snowc
                      {
                        Some(nowc)=>
                          {
                            self.cachetime.remove(&nowc);
                          },
                        None=>
                          {
                            self.value.insert(args, rs);
                          },
                      }
                    self.cachetime.insert(now,args);
                  }
                  rs
                }
              }else{
                let chk;
                chk=self.value.get(&args);
                let rs;
                match chk 
                  {
                    Some(v) =>
                      {
                        rs=*v
                      },
                    None => 
                      {
                        rs=(self.func)(args);
                        self.value.insert(args, rs); 
                      },
                  }
                rs
              }
          }
      }
  }
/// Module for simulating DIY For ... Loop
pub mod CSCForLoop
  {
    use std::ops::AddAssign;
    /// CSCFor is general "for...loop" with generic entry
    /// Struct for CSCFor
    #[derive(Debug,Clone,Copy)]
    pub struct CSCFor<T> 
      {
        count : T,
        ptr : T,
        start : T,
        step : T,
        end : T,
      }
    /// CSCFor Self Implement
    impl<T> CSCFor<T>
      where
        T : Default + Copy + Clone + ?Sized,
      {
        /// CSCFor Constructor
        /// CSCfor::new()
        /// To Create CSCFor With Default Parameter 
        /// # example:
        /// ```
        ///   use csclib::*;
        ///   let itr;
        ///   itr=CSCFor::new();
        /// ```
        pub fn new() -> CSCFor<T>
          {
            CSCFor 
            { 
              count : T::default(),
              ptr : T::default(),
              start : T::default(),
              step  : T::default(),
              end : T::default(),
            }
          }
        /// to assign start index for loop
        /// # example:
        /// ```
        ///   // --snip-- 
        ///   let itr;
        ///   itr=CSCFor::new()
        ///     .start(1.0);
        /// ```
        pub fn start(mut self, start:T) -> Self
          {
            self.start=start;
            self.ptr=start;
            self.count=start;
            self
          }
        /// to assign end index for loop
        /// # example:
        /// ```
        ///   // --snip-- 
        ///   let itr;
        ///   itr=CSCFor::new()
        ///     .start(1.0)
        ///     .end(10.0);
        /// ```
        pub fn end(mut self, end:T) -> Self
          {
            self.end=end;
            self
          }
        /// to assign each step interval for loop
        /// # example:
        /// ```
        ///   // --snip-- 
        ///   let itr;
        ///   itr=CSCFor::new()
        ///     .start(1.0)
        ///     .end(10.0)
        ///     .step(0.5);
        /// ```
        pub fn step(mut self, step:T) -> Self
          {
            self.step=step;
            self
          }
      }
    /// CSCFor Iterator
    impl<T> Iterator for CSCFor<T>
      where
        T:PartialOrd+AddAssign+Copy+Clone,
      {
        type Item=T;
        /// CSCFor Iter
        /// # example:
        /// ```
        ///   // --snip-- 
        ///   let itr;
        ///   itr=CSCFor::new()
        ///     .start(1.0)
        ///     .end(10.0)
        ///     .step(0.5);
        ///   for ele in itr{
        ///     println!("{}",ele);
        ///   };
        /// ```
        fn next(&mut self) -> Option<Self::Item> 
          {
            self.ptr=self.count;
            if self.count<self.end
              {
                self.count+=self.step;
                Some(self.ptr)
              } else {
                None
              }
          }
      }
  }