/*
 * This module defines the basic list and list type struct
 */

pub struct ListRef {
    name: String,
    address: Option<u64>,
}

impl ListRef {
    pub fn new(name: String) -> Self {
        ListRef {
            name,
            address: None,
        }
    }
}

pub struct List {
    l_type: ListRef,
    args: Vec<List>,
}

impl List {
    pub fn new(l_type: ListRef, args: Vec<List>) -> Self {
        List { l_type, args }
    }

    pub fn from_text(text: &String) -> Self {
        // Wraps string iterator call
        List::from_iterator(text.chars())
    }

    pub fn from_iterator(it: std::str::Chars) -> Self {
        let mut args: Vec<List> = Vec::new();
        let mut t: Option<ListRef> = None;

        while let Some(c) = it.next() {
            if c == '(' {
                // New list (recursive call)
            } else if c == ')' {
                // End list (return out)
            } else {
                // Add to current list
                if let Some(t) = t {
                    // Get required arg types, check that the new is of that
                    // type
                } else {
                    // First in the list is the name of the ListRef, so create
                    // a ListRef and add it

                }
            }
        }

        ()
    }
}