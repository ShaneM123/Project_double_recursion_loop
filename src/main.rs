fn main() {

    initiate_project_recursion();
}

fn initiate_project_recursion() {
    for x in 0..10000{
        initiate_project_recursion();

    }
}
