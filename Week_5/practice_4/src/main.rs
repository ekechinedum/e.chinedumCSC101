    fn main(){ 
        let fullname = "Chibudum John Umeh" ; 
        let department =   "Computer Science";
        let uni = "Pan-Atlantic University" ;

        let mut school = "School of Science";.to_string();
        // push string 
        school .push_str(" and Technology"); 
     
     println!("name is: {}", fullname); 
        // check length 
       println("length my fullname is: {}" , fullname.len()) 
        printLn!("I am a student of {} Department", department) ; 
        printLn! ("{}" , school) ; 
        printLn!("{}" , uni) ;
}
