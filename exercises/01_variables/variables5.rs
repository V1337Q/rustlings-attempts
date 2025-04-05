fn main() {
    let number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {}", number);

    // TODO: Fix the compiler error by changing the line below without renaming the variable.
    let number = 3;
    // Re-declaring the variable 'number' which causes what we call Variable Shadowing.
    // Coulda just comment the last two lines though. Haha.
    println!("Number plus two is: {}", number + 2);
}
