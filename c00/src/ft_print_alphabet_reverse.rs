
fn ft_print_alphabet_reverse() -> ()
{
    let mut alphabet = 'z';

    while alphabet >= 'a'
    {
        print!("{}", alphabet);
        alphabet = (alphabet as u8 - 1) as char;
    }
}

fn main()
{
    ft_print_alphabet_reverse();  
}