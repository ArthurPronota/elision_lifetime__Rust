// получить первый байт
fn first_byte(s: &str) ->&str{
    &s[0..1]
}

struct Cashe<'a>(&'a str) ;

impl<'a> Cashe<'a> {
    fn get(&self, key: &'a str) ->&str{
        if self.0.starts_with(key) {
            self.0
        } else {
            first_byte(self.0)
        }
    }
}

fn main() {
    let v = Cashe("abcdef") ;
    println!("r1: {}", v.get("abc")) ;  // r1: abcdef
    println!("r2: {}", v.get("xyz")) ;  // r2: a
}
