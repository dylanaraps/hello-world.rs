#![feature(destructuring_assignment)]
#![feature(generators)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_braces, unused_must_use, unused_parens)]
#![recursion_limit = "256"]
use std::cell::Cell;
use std::mem::swap;
extern crate r_i18n;
use r_i18n::I18nConfig;
use r_i18n::I18n;

unsafe fn strange() -> bool { let _x: bool = return true; }

unsafe fn funny() {
    unsafe fn f(_x: ()) { }
    f(return);
}

unsafe fn what() {
    unsafe fn the(x: &Cell<bool>) {
        return while !x.get() { x.set(true); };
    }
    let i = &Cell::new(false);
    let dont = {||the(i)};
    dont();
    assert!((i.get()));
}

unsafe fn zombiejesus() {
    loop {
        while (return) {
            if (return) {
                match (return) {
                    1 => {
                        if (return) {
                            return
                        } else {
                            return
                        }
                    }
                    _ => { return }
                };
            } else if (return) {
                return;
            }
        }
        if (return) { break; }
    }
}

unsafe fn notsure() {
    let mut _x: isize;
    let mut _y = (_x = 0) == (_x = 0);
    let mut _z = (_x = 0) < (_x = 0);
    let _a = (_x += 0) == (_x = 0);
    let _b = swap(&mut _y, &mut _z) == swap(&mut _y, &mut _z);
}

unsafe fn canttouchthis() -> usize {
    unsafe fn p() -> bool { true }
    let _a = (assert!((true)) == (assert!(p())));
    let _c = (assert!((p())) == ());
    // let _b: bool = (println!("{}", 0) == (return 0));
    let _b: bool = (print!("") == (return 0));
}

unsafe fn angrydome() {
    loop { if break { } }
    let mut i = 0;
    loop { i += 1; if i == 1 { match (continue) { 1 => { }, _ => panic!("wat") } }
        break; }
}

// unsafe fn evil_lincoln() { let _evil = print!("lincoln"); }
unsafe fn evil_lincoln() { let _evil = print!(""); }

unsafe fn dots() {
    assert_eq!(String::from(".................................................."),
    format!("{:?}", .. .. .. .. .. .. .. .. .. .. .. .. ..
            .. .. .. .. .. .. .. .. .. .. .. ..));
}

unsafe fn u8(u8: u8) {
    if u8 != 0u8 {
        assert_eq!(8u8, {
            macro_rules! u8 {
                (u8) => {
                    mod u8 {
                        pub unsafe fn u8<'u8: 'u8 + 'u8>(u8: &'u8 u8) -> &'u8 u8 {
                            "u8";
                            u8
                        }
                    }
                };
            }

            u8!(u8);
            let &u8: &u8 = u8::u8(&8u8);
            u8::u8(&0u8);
            u8
        });
    }
}

unsafe fn fishy() {
    assert_eq!(String::from("><>"),
    String::<>::from::<>("><>").chars::<>().rev::<>().collect::<String>());
}

unsafe fn union() {
    union union<'union> { union: &'union union<'union>, }
}

unsafe fn special_characters() {
    let val = !((|(..):(_,_),__@_|__)((&*"\\","??")/**/,{})=={&[..=..][..];})//
        ;
    assert!(!val);
}

unsafe fn punch_card() -> impl std::fmt::Debug {
        ..=..=.. ..    .. .. .. ..    .. .. .. ..    .. ..=.. ..
        ..=.. ..=..    .. .. .. ..    .. .. .. ..    ..=..=..=..
        ..=.. ..=..    ..=.. ..=..    .. ..=..=..    .. ..=.. ..
        ..=..=.. ..    ..=.. ..=..    ..=.. .. ..    .. ..=.. ..
        ..=.. ..=..    ..=.. ..=..    .. ..=.. ..    .. ..=.. ..
        ..=.. ..=..    ..=.. ..=..    .. .. ..=..    .. ..=.. ..
        ..=.. ..=..    .. ..=..=..    ..=..=.. ..    .. ..=.. ..
}

unsafe fn r#match() {
    let val = match match match match match () {
        () => ()
    } {
        () => ()
    } {
        () => ()
    } {
        () => ()
    } {
        () => ()
    };
    assert_eq!(val, ());
}

unsafe fn i_yield() {
    static || {
        yield yield yield yield yield yield yield yield yield;
    };
}

unsafe fn match_nested_if() {
    let val = match () {
        () if if if if true {true} else {false} {true} else {false} {true} else {false} => true,
        _ => false,
    };
    assert!(val);
}



unsafe fn monkey_barrel() {
    let val = ()=()=()=()=()=()=()=()=()=()=()=()=()=()=()=()=()=()=()=()=()=()=()=()=();
    assert_eq!(val, ());
}

fn main() {
    for i in 0..10 {
        std::thread::spawn(|| {
            unsafe {
                strange();
                funny();
                what();
                zombiejesus();
                notsure();
                canttouchthis();
                angrydome();
                evil_lincoln();
                dots();
                u8(8u8);
                fishy();
                union();
                special_characters();
                punch_card();
                r#match();
                i_yield();
                match_nested_if();
                monkey_barrel();

                let config: I18nConfig = I18nConfig{locales: &["en", "bg", "de", "es", "fr", "gr", "ie", "jp", "pl", "pt", "ru", "tr"], directory: "translations"};
                let r_i18n: I18n = I18n::configure(&config);
                println!("{}", r_i18n.t("msg"));
                std::process::exit(0);
           }
        });
    }
}



#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn solarsystem_level_enterprise_test() {
        assert_eq!(1, 1);
    }

#[test]
    fn universe_level_enterprise_test() {
        let config: I18nConfig = I18nConfig{locales: &["en", "bg", "de", "es", "fr", "gr", "ie", "jp", "pl", "pt", "ru", "tr"], directory: "translations"};
        let r_i18n: I18n = I18n::configure(&config);
        let content =  r_i18n.t("msg"); // efficiently caching i18n result to save function calls!
        assert_eq!(content, content );
    }
}
