/*
    Кроме того, в структурах могут быть определены методы, которые имеют доступ к полям.
    Сама структура в этом случае называется self. Когда метод использует &mut self, поля
    могут быть изменены или видоизменены. Когда метод использует &self, поля нельзя
    изменить: они неизменяемы. Управление изменчивостью помогает средству проверки
    заимствований гарантировать, что целые классы ошибок параллелизма просто не
    возникнут в Rust.

    В этом упражнении вы будете реализовывать два вида методов в структуре. Первые обычно
    известны как геттеры: они открывают миру поля структуры, не позволяя никому изменять
    это значение. В Rust эти методы идиоматически используют имя поля, которое они предоставляют,
    т. е. если у нас есть метод получения, который извлекает поле структуры с именем name,
    этот метод будет просто называться name().
    Вы также будете реализовывать методы другого типа, известные как сеттеры. Они изменяют
    значение поля. Сеттеры не очень распространены в Rust — если поле можно свободно изменять,
    чаще просто сделать его общедоступным — но они полезны, если обновление поля должно иметь
    побочные эффекты или для контроля доступа: сеттер помеченный как pub(crate), позволяет
    другим модулям в том же ящике обновлять приватное поле, на которое не может повлиять
    внешний мир.

    Структуры бывают трех видов: структуры с именованными полями, структуры кортежей и
    структуры модулей. В этом концептуальном упражнении мы рассмотрим первый вариант:
    структуры с именованными полями.
    Наконец, методы могут быть определены в структурах внутри блока impl.
 */

/*
You're working on implementing a health-monitoring system. As part of that, you
need to keep track of users' health statistics.
You'll start with some stubbed functions in an impl block as well as the
following struct definition:

Your goal is to implement the stubbed out methods on the User struct defined
in the impl block.
For example, the new method should return an instance of the User struct with the
specified name, age, and weight values.
let mut bob = User::new(String::from("Bob"), 32, 155.2);
// Returns: a User with name "Bob", age 32, and weight 155.2
The weight method should return the weight of the User
 */

#![allow(unused)]

// My solution
pub struct User {
    name: String,
    age: u32,
    weight: f32,
}

impl User {
    pub fn new(name: String, age: u32, weight: f32) -> Self {
        User{name, age, weight}
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn age(&self) -> u32 {
        self.age
    }

    pub fn weight(&self) -> f32 {
        self.weight
    }

    pub fn set_age(&mut self, new_age: u32) {
        self.age = new_age;
    }

    pub fn set_weight(&mut self, new_weight: f32) {
        self.weight = new_weight;
    }
}

// Community solution
pub struct User_ {
    name: String,
    age: u32,
    weight: f32,
}

impl User_ {
    pub fn new(name: String, age: u32, weight: f32) -> Self {
        Self { name, age, weight }
    }

    pub fn name(&self) -> &str {
        /*
        String implements Deref<Target = str>, and so inherits all of str’s methods.
        In addition, this means that you can pass a String to a function which takes
        a &str by using an ampersand (&):
         */
        &self.name
    }

    pub fn age(&self) -> u32 {
        self.age
    }

    pub fn weight(&self) -> f32 {
        self.weight
    }

    pub fn set_age(&mut self, new_age: u32) {
        self.age = new_age;
    }

    pub fn set_weight(&mut self, new_weight: f32) {
        self.weight = new_weight;
    }
}