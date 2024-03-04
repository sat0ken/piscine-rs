use std::io::stdout;
use std::os::fd::AsFd;
use nix::unistd::write;
pub fn ft_putchar(c: char) {
    let binding = stdout();
    let stdout = binding.as_fd();
    write(stdout, &[u8::try_from(c).unwrap()]).unwrap();
}

pub fn ft_print_alphabet() {
    for c in 'a'..= 'z' {
        ft_putchar(c);
    }
}

pub fn ft_print_reverse_alphabet() {
    for c in ('a'..= 'z').rev() {
        ft_putchar(c);
    }
}

pub fn ft_print_numbers() {
    for c in '0'..= '9' {
        ft_putchar(c);
    }
}

pub fn ft_is_negative(n: i32) {
    if n < 0 {
        ft_putchar('N');
    } else {
        ft_putchar('P');
    }
}

pub fn ft_print_comb() {
    for c in '0'..= '7' {
        for c1 in '1'..= '8' {
            for c2 in '2'..= '9' {
                ft_putchar(c);
                ft_putchar(c1);
                ft_putchar(c2);
                if c == '7' && c1 == '8'  && c2 == '9' {
                    return;
                }
                ft_putchar(',');
                ft_putchar(' ');
            }
        }
    }
}
