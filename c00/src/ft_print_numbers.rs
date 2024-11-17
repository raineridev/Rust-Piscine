
fn  ft_print_numbers() -> ()
{
    let mut number = 1;

    while number <= 9
    {
        print!("{}", number);
        number += 1;
    }
}


fn main()
{
    ft_print_numbers()
}
