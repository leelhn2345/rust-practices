// We define a module by starting with the mod keyword and then specify the name
// of the module (in this case, front_of_house) and place curly brackets around
// the body of the module. Inside modules, we can have other modules, as in this
// case with the modules hosting and serving.

// crate
//  └── front_of_house
//  ├── hosting
//  │   ├── add_to_waitlist
//  │   └── seat_at_table
//  └── serving
//      ├── take_order
//      ├── serve_order
//      └── take_payment

mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
