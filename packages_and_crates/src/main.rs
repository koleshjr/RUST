fn notes(){

    let packages_and_crates = format! (r#"
        crate is the smallest amount of code that the Rust compiler considers at a time
        crates can contain modules and the modules may be defined in other files that get compiled with the crate
        There are two types of crates: a binary crate or a library crate
            Binary crates: programs you can compile to an executable that you can run, such as a command line program or a server
                        each must contain a function called main
            Library crates: don't have the main function and  they do not compile to an executaable
                        define functionality intended to be shared with multiple projects
                        mostly crate is used interchangeably with library

            The crate root is a source file that the Rust compiler starts from and makes up the root module of your crate

        Packages: a bundle of one or more crates that provides a set of functionality
            contains a Cargo.toml that describes how to build those crates
            Cargo is a package that contains the binary crate for the command line tool we've been using
            Also has a library crate that the binary crate depends on
            Other projects can depend on the Cargo library crate to use the same logic the Cargo command-line tool uses 
        
        A package can contain as many library crates but only one binary crate


        What happens when we create a package?? cargo new package_name
            we get a Cargo.toml file , there is also a src/main.rs
            Cargo follows a convention that src/main.rs is the crate root of a binary crate with the same name as the package
            likewise cargo knows that if the package directory contains src/lib.rs the package contains a library crate with the same name as the package
            src/lib.rs is its root
            cargo passes the crate root files to rustc to build the library or binary

        if we have a package containing src/main.rs and src/lib.rs it has two crates: a binary and  a library, both with the same name as the package(what you have named ur project)
        A package can have multiple binary crates by placing files in the src/bin directory, each file will be a separate binary crate
            

                
        "#);

    let modules_paths_use = format! (r#"
        Rules to Follow when oranizing your Rust Code:
            start from the crate root: src/main.rs for binary crates and lib.rs for library crates
            declaring modules: 
                in the crate root file say you declare mod garden; The compiler will look for the modules code in:
                    inline , within curly brackets that replaces the semicolon following mod garden
                    in the file src/garden.rs
                    in the file src/garden/mod.rs

            Declaring submodules:
                in any other file other than the crate root, you can define sub modules
                You might declare mod vegetables; in src/garden.rs
                The compiler will look for the submodule's code within the directory named for the parent module in these places:
                    inline directly following mod vegetables , within curly brackets instead of the semi colon
                    in the file src/garden/vegetable.rs
                    in the file src/garden/vegetable.mod.rs

            Paths to code in Modules:
                once a module is part of your crate you can refer to code in that module from anywhere else in that same crate, 
                this is as long as the privacy rules allow, using the path to the code
                for example an Asparagus type in the garden vegetables module would be found at:
                    crate::garden::vegetables::Asparagus

            Private vs Public:
                Code within a module is private from its parent modules by default
                to make a module public declare it with pub mod instead of just mod
                to make items within a public module public as well use pub before their declarations

            The use keyword:
                within a scope the use keyword creates shortcuts to items to avoid repetition of long paths
                we can shorten this: crate::garden::vegetables::Asparagus by using:
                    use crate::garden::vegetables::Asparagus; 
                then use just Asparagus in our code
 
    
    "#);


    let group_related_code = format! (r#"
    
        Modules let us organize code within a crate for readability and easy reuse
        Modules also allow us to control the privacy of items , becuase code within a module is private by default
        But we can choose to make modules and items within them public, exposing them to external code
    
        for example:
            create a new library called restaurant:
                cargo new restaurant --lib
            
            inside src/lib.rs add the code below:
                //parent module 
                mod front_of_house{{
                    //sub-module or child module
                    mod hosting {{
                        //can hold functions, structs, enums, etc
                        fn add_to_waitlist(){{}}

                        fn seat_at_table(){{}}
                    }}
                    
                    //sub-module
                    mod serving {{
                        fn take_order(){{}}
                        fn serve_order(){{}}
                        fn take_payment(){{}}
                    }}
                }}
    
    "#);


    let paths = format! (r#"
    
            We use a path to show where to find an item in a module tree
            A path can take two forms:
                An absolute path: full path starting from the crate root
                A relative path: starts from the current module ans uses self, super or an identifier in the current module

            The convention is to specify absolute paths because its more likely we will want to move code definitions and item calls independently of each other
            to access the items of a module you'll have to give the access yourself since they are private and that is done with the key word pub
            If you want to make an item like a function or struct private, you put it in a module

            items in a parent module cannot use the private items inside child modules but items in child module can use its ancestors items



            Example:
                //while this is not public, the eat_at_restaurant can access it since that function is defined within this module
                mod front_of_house {{

                    //exposing a module private child module using pub
                    pub mod hosting{{
                        //you also have to expose the items inside the child module that you want the parent to access
                        pub fn add_to_waitlist(){{}}
                    }}
                }}

                pub fn eat_at_restaurant(){{
                    //absolute path

                    crate::front_of_house::hosting::add_to_waitlist()

                    //relative path
                    front_of_house::hosting::add_to_waitlist()


                }}

            Rust Best practices:
                for packages with both src/lib.rs and src/main.rs the module tree should be in the src/lib.rs
                this way any public items can be used in the binary crate by starting paths with the name of the package
                The binary crate becomes a user of the library crate, it can only use the public API
                This helps you design a good API; not only are you the author you are also a client


            Start Relative Paths with super:
                super represents the parent path
                we use super so that we can have fewer places to update code in the future if we decide to reorganize the crate's module tree

                Example:
                    fn deliver_order(){{}}
                    mod back_of_house{{
                        fn fix_incorrect_order(){{
                            cook_order();
                            super::deliver_order(); //we use super here instead of calling crate::deliver_order();

                        }}
                    }}

            

    
    "#);

    let make_structs_and_enums_public = format!(r#"
    
        if we use a pub before a struct , we make the struct pub , but not its contents
        we can make each field to be public or not on a case by case basis
        But for enums when we use a pub keyword before it, then every attribute becomes public


        Examples: Structs
                mod back_of_house{{

                    pub struct BreakFast{{
                        pub toast: String,      //this becomes public
                        seasonal_fruit: String, //this remains private
                    }}

                    impl BreakFast{{
                        pub fn summer(toast: &str)-> BreakFast{{
                            BrekFast{{
                                toast: String::from(toast),//can change thats why we set it to public
                                seasonal_fruit: String::from("peaches"),//default value
                            }}

                        }}
                    }}
                }}    

                pub fn eat_at_restaurant(){{
                    //order a breakfast in the summer with rye toast
                    let mut meal = back_of_house::BreakFast::summer("Rye");
                    meal.toast = String::from("Wheat"); //this is correct
                    meal.seasonal_fruit = String:;from("apple"); //this is wrong and fails
                    println!("I'd like {{}} toast please", meal.toast);
                }}

        Examples: Enum
                mod back_of_house {{
                    pub enum Appetizer{{
                        soup,
                        salad,
                    }}
                }}

                pub fn eat_at_restaurant(){{

                    //we can access all these because we made the enum public
                    let order 1 = back_of_house::Appetizer::soup; 
                    let order 2 = back_of_house::Appetizer::salad;
                }}
                
    
    "#);


    let use_keyword = format! (r#"
        
        use brings paths into scope
        we use the use keyword to avoid long repetitions, by creating a shortcut.

        Example:
                mod front_of_house {{
                    pub mod hosting{{
                        pub fn add_to_waitlist(){{}}
                    }}
                }}

                //create a shortcut now you will only use hosting to call items inside hosting
                //we dont use the path all the way to the function, only to the last parent, why? To show that the function is not locally defined
                use crate::front_of_house::hosting; 
                pub fn eat_at_restaurant(){{
                    hosting::add_to_waitlist();
                }}
        
        use only creates the shortcut for the particular scope in which the use occurs 

        Example that fails:
            mod front_of_house {{
                pub mod hosting{{
                    pub fn add_to_waitlist(){{}}
                }}
            }}

            use crate::front_of_house::hosting; //create a shortcut now you will only use hosting to call items inside hosting

            //when we move the eat_at_restaurant func into a new module named customer which is a different scope the body wont compile
            //move the use within the customer module too, or reference the shortcut in the parent module with super::hosting within the child customer module
            mod customer{{
                pub fn eat_at_restaurant(){{
                    hosting::add_to_waitlist();
                }}
            }}

        Example showing creating idiomattic use path and why we dont call the use all the way to the function: This applies to functions

            structs,enums and other items with use: its idiomatic to specify the full path
                use std::collections::HasMap;
                fn main(){{
                    let mut hao = HashMap::new();
                    map.insert(1,2);
                }}

            The only exception to this idiomatic expression is if we ar bringing two items with the same name into scope
                use std::fmt;
                use std::io; //you can differentiate the two though using as keyword-> use std::io as IoResult;

                fn function1() ->fmt::Result{{
                    //snip
                }}
                
                fn function2() -> io::Result or IoResult{{
                    //snip
                }}


        Re-exporting Names with pub use:
            when we bring a name into scope with the use keyword, the name available in the new scope is private
            to enable the code that calls our code to refer to that name as if it had been defined in that code's cope we can combine pub and use(Re-exporting)
            This is called re-exporting because we are bringing an item into scope but also making that item available for others to bring into their scope

                mod front_of_house {{
                    pub mod hosting{{
                        pub fn add_to_waitlist(){{

                        }}
                    }}
                }}


                //before adding pub , external code would have to call restaurant::front_of_house::hosting::add_to_waitlist()

                pub use crate::front_of_house::hosting;

                //after adding pub the code calls restaurant::hosting::add_to_waitlist()
                //with pub use we can write our code with one structure but expose a different structure

                pub fn eat_at_restaurant(){{
                    hosting::add_to_waitlist();
                }}


            


    
    
    
    "#);


    let using_external_packages = format!(r#"
                pull them from crates.io 
                then define them in your cargo.toml
                    like: rand = "0.8.5"

                and now we have to bring them into scope:
                    like use rand::Rng;

                and we can use nested paths to bring multiple items from one package into scope:
                    like use std::{{cmp::Ordering, io}};
                    instead of:
                            use std::cmp::Ordering;
                            use std::io;

                    like use std::io::{{self, Write}};
                    instead of:
                            use std::io;
                            use std::io::Write

                if you want to bring all public items:
                    use std::collections::*;

                    mostly the lob operator is used to bring everything under test into the tests module not in our programs becuase it can lead to confusions
                    but can also be used as a prelude pattern
    
    
    
    "#);


    let separate_diff_files = format!(r#"
    
            separate files makes code easier to navigate
            we extract modules into files instead of having all the modules defined in the crate root file in this case src/lib.rs and also works with src/main.rs

            1st extract the front_of_house module to its own file how?
                remove the code in the curly brackets for the front_house module we just wrote earlier leaving:
                    only: mod front_of_house; 
                    Now src/lib.rs will contain:
                        mod front_of_house;
                        pub use crate::front_of_house::hosting;

                        pub fn eat_at_restaurant(){{
                            hosting::add_to_waitlist();
                        }}

                Next place the code that was in the curly brackets into a new file named src/front_of_house.rs
                        pub mod hosting{{
                            pub fn add_to_waitlist(){{}}
                        }}

                Next we will extract the hsoting module to its own file how?:
                        src/front_of_house.rs 
                            pub mod hosting; //becuase hosting is a child of front_of_house.rs

                Then we create a new file: src/front_of_house/hosting.rs
                        pub fn add_to_waitlist(){}
    
    "#)






        println!("<<<<<<<---------------------------------------------------------------------------Packages and Crates----------------------------------------------------------------->>>>>>>>>>>>>");
        println!("{}", packages_and_crates);
        println! ();

        println!("<<<<<<<-------------------------------------------------------------------Defining Modules to control scope and privacy----------------------------------------------------------------->>>>>>>>>>>>>");
        println!("{}", modules_paths_use);
        println! ();        

        println!("<<<<<<<-------------------------------------------------------------------Grouping Related Code in Modules----------------------------------------------------------------->>>>>>>>>>>>>");
        println!("{}", group_related_code);
        println! ();                

        println!("<<<<<<<----------------------------------------------------------------Paths For referring to an Item in the Module Tree----------------------------------------------------------------->>>>>>>>>>>>>");
        println!("{}", paths);
        println! ();                

        println!("<<<<<<<----------------------------------------------------------------Make enums and structs public----------------------------------------------------------------->>>>>>>>>>>>>");
        println!("{}", make_structs_and_enums_public);
        println! (); 

        println!("<<<<<<<----------------------------------------------------------------Bringing paths into scope with the use keyword----------------------------------------------------------------->>>>>>>>>>>>>");
        println!("{}", use_keyword);
        println! ();                


        println!("<<<<<<<----------------------------------------------------------------Using external Packages----------------------------------------------------------------->>>>>>>>>>>>>");
        println!("{}", using_external_packages);
        println! ();               

        println!("<<<<<<<----------------------------------------------------------------Separating Modules Into Different Files----------------------------------------------------------------->>>>>>>>>>>>>");
        println!("{}", separate_diff_files);
        println! ();                    




}

fn main(){
    notes()
}