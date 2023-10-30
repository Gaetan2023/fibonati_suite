
mod fibonachi{
   
    pub fn execut () -> i64 {
         let mut a =1;
     let mut b =1;
     let mut c = 1;
    
     while c< 30 {
         println!("b :{}",b);
         a= b;
         c +=1;
         b=a + b;
         
     }
     b
     }
    
     
 }
 
 
 fn main() {
     println!("Hello, world! this is fibonati_suite");
     println!("{}",crate::fibonachi::execut());
 
 
 }
 