
fn ft_print_alphabet() -> ()
{
    let mut alphabet = 'a';

    while alphabet <= 'z'
    {
        print!("{}", alphabet);
        alphabet = (alphabet as u8 + 1) as char;
    }
}

fn main()
{
    ft_print_alphabet();  
}