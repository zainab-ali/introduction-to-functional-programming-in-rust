
class: center, middle

# An Introduction to Functional Programming
## Zainab Ali
### https://zainab-ali.github.io/introduction-to-functional-programming-in-rust

---
class: center, middle

<img src="life.jpg" width="500">


<footer style="font-size: 14px">Source: edwardmonkton.com</footer>

---
class: center, middle

# Why functional programming?
## Ease of reasoning
---
class: middle
# Agenda
 - 🎂 Design a basic app
 - 🐛 Hunt for a bug
 - λ  Rewrite with functional programming techniques

---
class: middle

# Birthday app

```bash
./birthday <date-of-birth>   # Shortened to "dob"
```

## Today is 30th April 2025.

```bash
./birthday 2000-04-30   # My birthday is today
Happy birthday! Congratulations on becoming 25!
```

```bash
./birthday 2000-05-02   # My birthday is soon
It's not your birthday yet. Wait for 2 more days.
```

```bash
./birthday 2000-04-29   # My birthday was yesterday
You've already had your birthday. I hope you had fun!
```
---
class: middle
```rust
fn main() -> () {
    if birthday() == today() {
        println!("Happy birthday! Congratulations on becoming {}!", age());
    } else {
        if birthday() < today() {
            println!("You've already had your birthday. I hope you had fun!")
        } else {
            println!(
                "It's not your birthday yet. Wait for {} more days.",
                days_until_birthday()
            );
        }
    }
}

```

---
class: middle

# Rule

## If it's not my birthday yet, I should wait for at least one day.

---
class: middle

# 🐛 The bug

## It's close to midnight on the 29th April.
```bash
./birthday 2000-04-30
It's not your birthday yet. Wait for 0 more days.
```


---
class: middle
```rust
fn main() -> () {
    if birthday() == today() {
        println!("Happy birthday! Congratulations on becoming {}!", age());
    } else {
        if birthday() < today() {
            println!("You've already had your birthday. I hope you had fun!")
        } else {
            println!(
                "It's not your birthday yet. Wait for {} more days.",
                days_until_birthday()
            );
        }
    }
}

```
---
class: middle


```rust
fn today() -> NaiveDate {
    Utc::now().date_naive()
}
```

```rust
fn birthday() -> NaiveDate {
    date_of_birth().with_year(today().year()).unwrap()
}
```

```rust
fn days_until_birthday() -> i64 {
    birthday().signed_duration_since(today()).num_days()
}
```

---
class: middle

# Functional programming techniques
 - Write pure functions
 - Identify side effects
 - Modelling data with types
 - Write total functions
 
---
class: middle
# Pure functions

```rust
let x = 1 + 2;
// Is equivalent to
let x = 3;
```
```rust
let is_empty = Some(5).is_empty();
// Is equivalent to
let is_empty = false;
// And also equivalent to
let is_empty = match Some(5) {
 Some(_) => false,
 None => true
};
```

---
class: middle
# Side effects

```rust
let x = today();
// Is not equvalent to
let x = NaiveDate::of_ymd(2025,04,30);
```

```rust
let x = println!("Happy birthday!");
// Is not equivalent to
let x = ();
```

```rust
let x = sys::env::args();
// Is not equivalent to
let x = vec!["birthday", "2000-04-30"];
```
---
class: middle

# λ Rewriting the code

```rust
fn main() -> () {
    // Side effects
    let today = get_today();
    let input = get_arg();
	// Pure code
    let dob = calc_dob(input);
    let birthday = calc_birthday(today, dob);
	// Printing
    if birthday == today {
	   ...
    } else if birthday < today {
	   ...
    } else {
        println!(
            "It's not your birthday yet. Wait for {} more days.",
            calc_days_until_birthday(today, birthday)
        );
    }
}
```

---
class: middle

# Pure functions
## No side effects
 - `calc_dob`
 - `calc_days_until_birthday`
 - `calc_birthday`
 
## Side effects
 - `get_arg`
 - `get_today`
 
---
class: middle
# Tests

```rust
    #[test]
    fn test_days_until_birthday() {
        let today = NaiveDate::from_ymd(2025, 04, 29);
        let birthday = NaiveDate::from_ymd(2024, 04, 30);
        let result = calc_days_until_birthday(today, birthday);
        assert_eq!(result, 1);
    }
```

---
class: middle
# Tests?

```rust
    if birthday == today {
        println!(
            "Happy birthday! Congratulations on becoming {}!",
            calc_age(today, dob)
        );
    } else if birthday < today {
        println!("You've already had your birthday. I hope you had fun!")
    } else {
        println!(
            "It's not your birthday yet. Wait for {} more days.",
            calc_days_until_birthday(today, birthday)
        );
    }
```

---
class: middle
# Modelling output

```rust
enum Message {
    HappyBirthday { age: u32 },
    HadBirthday,
    Wait { days: u32 },
}
```

```rust
// Pure
fn calc_message(input: Option<String>, today: NaiveDate) -> Message
```
```rust
// Side effects
fn print_message(message: Message) -> ()
```

---
class: middle
```rust
fn calc_message(input: Option<String>, today: NaiveDate) -> Message {
    let dob = calc_dob(input);
    let birthday = calc_birthday(today, dob);
    if birthday == today {
        Message::HappyBirthday {
            age: calc_age(today, dob),
        }
    } else if birthday < today {
        Message::HadBirthday
    } else {
        Message::Wait {
            days: calc_days_until_birthday(today, birthday),
        }
    }
}
```
---
class: middle

```rust
fn print_message(message: Message) -> () {
    match message {
        HappyBirthday { age } => 
		  println!("Happy birthday! Congratulations on becoming {}!", age),
        HadBirthday => 
		  println!("You've already had your birthday. I hope you had fun!"),
        Wait { days } => 
		  println!("It's not your birthday yet. Wait for {} more days.", days),
    }
}
```
---
class: middle
# Summary

```rust
fn main() -> () {
    // Side effects
    let today = get_today();
    let input = get_arg();
    // Pure
    let message = calc_message(input, today);
    // Side effects
    print_message(message);
}
```

---
class: middle
# Recap
 - pure functions
 - side effects
 - modelling data
---
class: middle
# 🐛 More problems

```bash
./birthday 2030-04-30
panic!
```

```bash
./birthday 2000-42-50
panic!
```

```bash
./birthday tomorrow
panic!
```
---
class: middle

# Panic if 
 - I was born on a nonsensical date.
 - I will be born in the future.
 - I will be born `"tomorrow"`.

---
class: middle

# Totality
## An output for all possible inputs
---
class: middle

# Result

```rust
enum Error {
    NoDateOfBirth,
    BadDateOfBirth(String),
    BornInTheFuture,
}
```

```rust
fn calc_message(input: Option<String>, 
                today: NaiveDate) -> Result<Message, Error>
				
```

```rust
fn calc_dob(input: Option<String>, 
            today: NaiveDate) -> Result<NaiveDate, Error>
```

---
class: middle

```rust
fn calc_message(input: Option<String>, 
                today: NaiveDate) -> Result<Message, Error> {
    let dob = calc_dob(input, today)?;
    let birthday = calc_birthday(today, dob);
    let message = ...
	Ok(message)
}
```

---
class: middle

# Tests

```rust
    #[test]
    fn born_in_future() {
        let today = NaiveDate::from_ymd(2025, 04, 29);
        let result = calc_message(Some("2030-06-15".to_string()), today);
        assert_eq!(result, Err(Error::BornInTheFuture));
    }
```

---
class: middle

# We've learned to...
 - Write pure functions
 - Identify side effects
 - Model data
 - Write total functions
 
## Write bug-free testable code

---
class: center, middle

<img src="good_code.png" width="350">


<footer style="font-size: 14px">Source: xkcd.com</footer>

---
class: middle

# Find me
 - Blog: [kebab-ca.se](https://kebab-ca.se/presentations.html)
 - BlueSky: zainab.pureasync.com
 - Email: zainab@pureasync.com
 - LinkedIn: [zainab-ali-fp](https://uk.linkedin.com/in/zainab-ali-fp)
 - GitHub: zainab-ali

---
class: middle

# Books
 - [Functional Stream Processing](https://pureasync.gumroad.com/l/functional-stream-processing-in-scala)

# Upcoming talks
 - Your docs are a program, [Devoxx UK 2025](https://www.devoxx.co.uk/)
 - Your docs are a program, [LambdaDays 2025](https://lambdadays.org/lambdadays2025)
 - Functional Stream Processing Workshop, [Scala Days 2025](https://scaladays.org/workshops#workshops)

---
class: center, middle
# Thank you!
## Questions?
### https://zainab-ali.github.io/introduction-to-functional-programming-in-rust
### https://github.com/zainab-ali/introduction-to-functional-programming-in-rust
