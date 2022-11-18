[toc]]
## 基本语法
### 1.所有权
所有权设计基于如下基本特性展开：
* 编译型语言
* Rustc -> Move 语义
* core -> Copy trait, Sized trait
* std -> reexport Copy / Sized
* 所有权是内存管理基石
* 编译时必须知道类型大小，因为它是基于栈来管理内存
* CPP RAII机制

* Rust中每个值都被一个变量所拥有，该变量称为值的所有者
* 一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
* 当所有者（变量）离开作用域范围时，这个值将被丢弃 drop

Rust的所有权规则，解决了谁真正拥有数据的生杀大权问题，**让堆上数据的多重引用不复存在**，这是她最大的优势。
但是这也让代码变得复杂，尤其是只存储栈上的简单数据，如果要避免所有权转移后不能访问，我们就需要复制copy, 会很麻烦，且效率不高。
Rust提供了两个解决方案：
* Copy 语义： 如果你不希望所有权被转移，也即不用默认的Move语义，则可以使用Copy语义。如果一个数据结构实现了Copy trait, 那么就会使用Copy语义，如下小结【Move && Copy trait】中介绍的数据结构。这样在赋值或者传参时，值会自动按位拷贝（浅拷贝）
* Borrow语义：不希望所有权转移，又无法使用Copy语义（比如String, Vecu<u8>）, 则可以使用Borrow语义
#### Move && Copy trait

``` rust
fn is_copy<T: Copy>() {}
fn types_impl_copy_trait() {
    is_copy::<bool>();
    is_copy::<char>();
    // all iXX and uXX, usize/isize, fXX implement Copy trait
    is_copy::<i8>();
    is_copy::<u64>();
    is_copy::<i64>();
    is_copy::<usize>;
    // func (actually a pointer) is Copy
    is_copy::<fn()>();
    // raw pointer is Copy
    is_copy::<*const String>();
    is_copy::<*mut String>();
    // immutable reference is Copy
    is_copy::<&[Vec<u8>]>();
    is_copy::<&String>();
    // array/tuple with val which is Copy is Copy
    is_copy::<[u8; 4]>(); // ? 啥含义
    is_copy::<(&str, &str)>();
}
fn types_not_impl_copy_trait() {
    // unsized or dynamic size type is not Copy
    is_copy::<str>();
    is_copy::<[u8]>();
    is_copy::<Vec<u8>>();
    is_copy::<String>();
    // mutable refrence is not Copy
    is_copy::<&mut String>();
    // array/tuple with val that not Copy is not Copy
    is_copy::<[Vec<u8>; 4]>(); // ?
    is_copy::<(String, u32)>(); // ?
}
fn main() {
    types_impl_copy_trait(); // 支持Copy trait 编译通过
    types_not_impl_copy_trait(); // 编译失败
}
```
![39fe46df093e7c1ff35fd5ad1b8a4f39.png](en-resource://database/2180:1)

#### borrow 借用/引用
Borrow 语义通过引用语法（& 或者 &mut）来实现。
Rust没有**传引用**的概念，Rust所有的参数传递都是**传值**，不管是copy还是move

### 2.变量,函数,结构体,枚举,控制流
#### 2.1定义变量： 不可变 let / 可变 let mut
#### 2.2定义函数： fn
* Rust函数参数类型和返回值的类型必须显示定义，如果没有返回值可以省略，返回unit.
* 函数内部提前返回使用return 关键字，否则最后一个表达式就是其返回值
* 如果最后一个表达式后添加了 ；分号，隐含其返回值为unit
``` rust
fn pi() -> f64 { // 返回类型是f64
    3.1415926  // 没有分号，返回值就是3.1415923
}
fn not_pi() { // rust中，函数没有返回值，那么返回值为unit, 符合为()
    3.1415926; // 有分号返回的就是; unit
}
fn main() {
    let is_pi = pi();
    let is_unit1 = not_pi();
    let is_unit2 = {
        pi();
    };
    println!("is_pi: {:?}, is_unit1: {:?}, is_unit2: {:?}", is_pi, is_unit1, is_unit2); 
    // 输出 is_pi: 3.1415926, is_unit1: (), is_unit2: ()
}
```
#### 2.3结构体、枚举
``` rust

#[derive(Debug)]
enum Gender {
    Unspecified = 0,
    Female = 1,
    Male = 2,
}
#[derive(Debug, Copy, Clone)]
struct UserId(u64);
#[derive(Debug, Copy, Clone)]
struct TopicId(u64);
#[derive(Debug)]
struct User {
    id : UserId,
    name : String,
    gender : Gender,
}
#[derive(Debug)]
struct Topic {
    id : TopicId,
    name : String,
    owner : UserId,
}
// 定义聊天室中可能发生的事件
#[derive(Debug)]
enum Event {
    Join((UserId, TopicId)), // 为啥两层括号？
    Leave((UserId, TopicId)),
    Message((UserId, TopicId, String)),
}
fn main() {
    let alice = User { id: UserId(1), name: "Alice".into(), gender: Gender::Female };
    let bob = User { id: UserId(2), name: "Bob".into(), gender: Gender::Male };
    let topic = Topic { id: TopicId(1), name: "rust".into(), owner: UserId(1) };
    let event1 = Event::Join((alice.id, topic.id));
    let event2 = Event::Join((bob.id, topic.id));
    let event3 = Event::Message((alice.id, topic.id, "hello world!".into()));
    println!("e1: {:?}, e2: {:?}, e3: {:?}", event1, event2, event3);
    /* 输出
    e1: Join((UserId(1), TopicId(1))), e2: Join((UserId(2), TopicId(1))), e3: Message((UserId(1), TopicId(1), "hello world!"))
    */
}
```
#### 2.4 控制流 while/loop/for/break/continue
``` rust
loop {
    ...
    if i >= n {
        break;
    }
}
while i < n {
    i += 1；
}
for i in 2..n {
}
```
#### 2.5 类型&str 和 String
字符串字面值 let s = "hello", s是被硬编码进程序里中的字符串值，类型是 `&str`
字符串字面值不适用于所有场景，原因如下：
* 字符串字面值是**不可变**的，因为被硬编码到程序中
* 并非所有的字符串的值都能在编写代码时得知，有很多是用户输入场景
此时 就需要**动态字符串类型  String**, 该 类型被分配到堆上，可以**动态伸缩**，也就可以存储在**编译时**大小未知的文本。
```  rust
let mut s = String::from("hello");
s.push_str(", world!");
pr
intln!("{}", s);
```

### 3. 模式匹配
#### match 关键字匹配
#### if let / while let 简单匹配
``` rust
fn process_event(event : &Event) {
    // 使用match做模式匹配
    match event {
        Event::Join((uid, _tid)) => println!("user {:?} joined", uid),
        Event::Leave((uid, tid)) => println!("user {:?}, left {:?}", uid, tid),
        Event::Message((_, _, msg)) => println!("broadcast: {}", msg),
    }
    // 使用if let/ while let也可以做简单的模式匹配
    if let Event::Message((_, _, msg)) = event {
        println!("broadcast: {}", msg);
    }
}
fn main() {
    let alice = User { id: UserId(1), name: "Alice".into(), gender: Gender::Female };
    let bob = User { id: UserId(2), name: "Bob".into(), gender: Gender::Male };
    let topic = Topic { id: TopicId(1), name: "rust".into(), owner: UserId(1) };
    let event1 = Event::Join((alice.id, topic.id));
    let event2 = Event::Join((bob.id, topic.id));
    let event3 = Event::Message((alice.id, topic.id, "hello world!".into()));
    // e1: Join((UserId(1), TopicId(1))), e2: Join((UserId(2), TopicId(1))), e3: Message((UserId(1), TopicId(1), "hello world!"))
    println!("e1: {:?}, e2: {:?}, e3: {:?}", event1, event2, event3);
    process_event(&event1); // user UserId(1) joined
    process_event(&event2); // user UserId(2) joined
    process_event(&event3); // broadcast: hello world!
}
```

### 4. 错误处理
#### 错误封装 Result<T, E>
#### 错误传播 ？

### 5.项目组织
* 多模块： mod
* 单项目/库: crate
* 多项目： workspace

### 6.类型系统

### 7.trait
trait是Rust中的接口，她定义了类型使用这个接口的行为。
trait中定义了一系列的方法接口，这些方法也被称为**关联函数**（associate function）
在trait中，方法可以有缺省的方法。比如对于Write trait,只需要实现write和flush这两个方法，其他都有缺省实现。
``` rust
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>; // 子类必须要实现此接口
    fn flush(&mut self) -> Result<()>;
    fn write_all(&mut self, buf: &[u8]) -> Result<()> { ... } // 有缺省实现
    fn by_ref(&mut self) -> &mut Self where Self：Sized { ... } 
```
在trait定义的方法中，会有两个特殊关键字 Self和self
####  trait作用
1. 接口
2. 类型标记
3. 静态分发（泛型和trait bound）
4. 动态分发 （trait object）
5. 重载（OverLoading）

####  标准库内置trait
| No.| 分类 | trait 类型|
|--|--|--|
|1|比较相关|PartialEq/Eq/PartialOrd/Add/Sub|
|2|索引|Index/IndexMut|
|3|闭包|Fn/FnMut/FnOnce|
|4|格式化字符串|Display/Debug|
|5|所有权|Copy/Clone/Drop|
|6|默认值 |Default|
|7|错误处理 |Error|
|8|Hash|Hash|
|9|迭代器|Iterator|
|10|类型转换|From/Into|
|11|智能指针|Box/Cow/Deref/DerefMut|
|12|类型转换| AsRef/AsMut/Borrow/BorrowMut/ToOwned|
|13|线程安全 |Send/Sync|
|14|运行时反射 |Any|

#### 7.1 关键字：Self && self
* Self: 首字母大写的Self代表**当前的类型**，比如File类型实现了Write, 那么实现过程中使用的Self就指代File
* self: 首字母小写的self 在用作方法的第一个参数时，实际是 **self: Self**的缩写
    
|缩写用法  |等同于  | 含义|
| --- | --- | ---|
| self |self: Self  |不可变变量|
| &self |self: &Self  |不可变引用|
| &mut self| self: &mut Self | 可变引用| 

``` rust

use std::fmt;
use std::io::Write;
struct BufBuilder {
    buf: Vec<u8>,
}
impl BufBuilder {
    pub fn new() -> Self {
        Self {
            buf: Vec::with_capacity(1024),
        }
    }
}
// 实现Debug trait, 打印字符串
impl fmt::Debug for BufBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.buf))
    }
}
impl Write for BufBuilder {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buf.extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> { // 即使不用，也必须要实现
        Ok(())
    }
}
fn main() {
    let mut buf = BufBuilder::new();
    buf.write_all(b"hello, world!").unwrap();
    println!("{:?}", buf);
}
```

#### 7.2 定义并实现一个trait
``` rust
use std::str::FromStr;
use regex::Regex;
pub trait Parse {
    fn parse(s: &str) -> Self;
}
impl <T> Parse for T
where
    T: FromStr + Default,
{
    fn parse(s: &str) -> Self {
        let re: Regex = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
        // 生成一个创建缺省值的闭包，简化后续代码
        // Default::default()返回的类型根据上下文能推导出来，是Self
        let d = || Default::default();
        if let Some(captures) = re.captures(s) {
            captures
                .get(0)
                .map_or(d(), |s| s.as_str().parse().unwrap_or(d()))
        } else {
            d()
        }
    }
}
fn main() {
    println!("result: {}", u8::parse("223 hello world"));
    println!("result: {}", f64::parse("223.25 hello world"));
}
```
#### 7.3 带关联类型的trait
Rust允许trait内部包含关联类型，实现时和关联函数一样，她也需要实现关联类型。
Parse trait 添加关联类型, 添加关联类型Error, 这样Parse trait就可以在出错时返回合理的错误了。
``` rust
pub trait Parse {
    type Error; // 关联类型type用作输出
    fn parse(s: &str) -> result<Self, Self::Error>;
}
```
完整实现
``` rust
use std::str::FromStr;
use regex::Regex;
pub trait Parse {
    type Error;
    fn parse(s: &str) -> Result<Self, Self::Error>
        where
            Self: Sized;
}
impl <T> Parse for T
where
    T: FromStr + Default,
{
    // 定义关联类型Error为String
    type Error = String;
    fn parse(s: &str) -> Result<Self, Self::Error> {
        let re: Regex = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
        if let Some(captures) = re.captures(s) {
            // 当出错时我们返回Err(String)
            captures
                .get(0)
                .map_or(Err("failed to capture".to_string()), |s| {
                    s.as_str()
                        .parse()
                        .map_err(|_err| "failed to parse captured string".to_string())
                })
        } else {
            Err("oh, failed to parse string".to_string())
        }
    }
}
fn main() {
    println!("result: {:?}", u8::parse("223 hello world")); // 打印为啥有时是{}， 有时是 {:?}
    println!("result: {:?}", f64::parse("223.25 hello world"));
    println!("result: {:?}", i8::parse("223.25 hello world"));
}
```
#### 7.4 支持泛型的trait (特设多态)
``` rust
// Rhs参数是此trait的泛型参数， 默认是Self, 也可以是其他类型
pub trait Add<Rhs = Self> {
     type Output;
     #[must_use]
     fn add(self, rhs: Rhs) -> Self::Output;
 }
```
以复数相加为例，使用这个trait
``` rust
use std::ops::Add;
#[derive(Debug)]
struct Complex {
    real: f64,
    imagine: f64,
}
impl Complex {
    pub fn new(real: f64, imagine: f64) -> Self {
        Self {real, imagine}
    }
}
// 对Complex类型的实现
impl Add for Complex {
    type Output = Self;
    // 注意 add第一个参数是self,会移动所有权
    fn add(self, rhs: Self) -> Self::Output {
        let real = self.real + rhs.real;
        let imagine = self.imagine + rhs.imagine;
        Self::new(real, imagine)
    }
}
// 不想移动所以权，可以为 &Complex实现add, 这样可以做 &c1 + &c2
impl Add for &Complex {
    // 注意这里不能是Self了，因为此时Self是 &Complex
    type Output = Complex;
    // 注意 add第一个参数是self,会移动所有权
    fn add(self, rhs: Self) -> Self::Output {
        let real = self.real + rhs.real;
        let imagine = self.imagine + rhs.imagine;
        Complex::new(real, imagine)
    }
}
// 因为Add<Rhs = Self>是个泛型trait, 我们可以为Complex实现Add<f64>
impl Add<f64> for &Complex {
    type Output = Complex;
    // 泛型的应用：rhs现在是f64了
    fn add(self, rhs: f64) -> Self::Output {
        let real = self.real + rhs;
        Complex::new(real, self.imagine)
    }
}
fn main() {
    let c1 = Complex::new(1.0, 1f64);
    let c2 = Complex::new(2 as f64, 3.0);
   
    println!("{:?}", &c1 + &c2);
    println!("{:?}", &c1 + 5.0);
    println!("{:?}", c1 + c2);
    // c1 c2所有权已被移动，所以下面这句无法编译
    // println!("{:?}", c1 + c2);;
}
```
#### 7.5 静态分派和动态分派(dynamic dispatching)Trait Object
##### 7.5.1 静态分派 -- 使用泛型函数
``` rust
trait Animal {
    fn name(&self) -> &`static str;
}
fn name(animal：impl Animal) -> &'static str {
    animal.name()
}
// impl Animal 是 T: Animal的简写 
fn name<T: Animal>(animal: T) -> &`static str;
```
##### 7.5.2 动态分派
Trait Object, 表现为&dyn Trait 或者 Box<dyn Trait>
Trait Object底层逻辑就是胖指针，其中一个指针指向数据本身，另一个指向**vtable 虚函数表**。
vtable是一张静态的表，Rust在编译时会为使用了trait object的类型的trait实现生成一张表，放在可执行文件中（TEXT或者RODATA段）
``` rust
pub trait Formatter {
    fn format(&self, input: &mut String) -> bool;
}
pub fn format(input: &mut String, formatters: Vec<&dyn Formatter>) {
    for formatter in formatters {
        formatter.format(input);
    }
}
int main {
    let mut text = "hello, world".to_string();
    let html: &dyn Formatter = &HtmlFormatter;
    let rust: &dyn Formatter = &RustFormatter;
    let formatters = vec![html, rust];
    format(&mut text, formaters);
    
    println!("text: {}", text);
}
```

#### 7.6 必须要掌握的trait
##### 7.6.1 Clone trait
``` rust
pub trait Clone {
    fn clone(&self) -> Self;
    
    fn clone_from(&mut self, source: &Self) { // 有缺省实现
        *self = source.clone()
    }
}
```
a = b.clone();
a.clone_from(&b); // 可以避免内存分配，提高效率

Clone trait可以通过派生宏直接实现，简化代码。
如果定义的结构体中每一个字段都实现了 Clone trait, 则可以使用 `#[derive(Clone)]`

##### 7.6.2 Copy trait
Copy trait 没有额外的方法，只是一个标记trait (marker trait). trait定义
``` rust
pub trait Copy: Clone {}
```
##### 7.6.3 Drop trait
``` rust
pub trait Drop {
    fn drop(&mut self);
}
```
需要注意，Copy trait和Drop trait是互斥的，两者不能共存。
当你尝试为同一种数据类型实现Copy，也实现Drop, 编译器会报错。
因为： Copy是按位做浅拷贝，那么她默认拷贝的数据没有需要释放的资源；而drop而恰恰是为了释放资源而生的。

##### 7.6.4 标记trait： Sized/Send/Sync/Unpin
* **Sized trait**: 用于标记有具体大小的类型，在使用泛型参数时，Rust编译器会自动为泛型参数加上Sized约束。
比如：
``` rust
struct Date<T> { // struct Data<T: Sized>
    inner: T，
}
```
对于可变类型T, Rust提供了 **？Sized**
定义了 T: ?Sized, 那么T就是任意大小。
``` rust
pub enum Cow<'a, B: ?Sized + 'a> where B: ToOwned,
{
    // 借用的数据， 引用大小固定
    Borrowed(&'a B),
    // 拥有的数据，B 可变 可以是[T] 或者str类型
    Owned(<B as ToOwned>::Owned),
}
```
* **Send/Sync trait**
``` rust
pub unsafe auto trait Send {}
pub unsafe auto trait Sync {}
```
auto 意味着编译器会在合适的场合，自动为数据结构添加她们的实现。
Send/Sync 是Rust并发安全的基础：
 * 如果一个类型T实现了Send trait, 意味着T可以安全地从一个线程移动到另一个线程，也就是说所有权可以在线程间移动
 * 如果一个类型T实现了Sync trait, 则意味着&T 可以安全地在多个线程中共享。 一个类型T满足Sync trait, 当且仅当 &T 满足Send trait.
 * 换句话说，一个类型T: Send, 那么T在某个线程中独占访问是线程安全的；
     如果一个类型是T: Sync, 那么T在线程间的只读共享是安全的。
     
    不支持Send/Sync的数据结构有
    |类型|备注|
    |--|--|
    裸指针 `*const T *mut T`| 不安全
    UnsafeCell<T>|不支持Sync
    引用计数Rc| Send和Sync都不支持，所以Rc无法跨线程
    
##### 7.6.5 类型转换相关trait：From<T>/Into<T>/AsRef<T>/AsMut<T>
* **From<T>/Into<T> trait**
``` rust
// 实现了From<T> 会自动实现Into<T>
pub trait From<T> {
    fn from(T) -> Self;
}

pub trait Into<T> {
    fn into(self) -> T;
}
```
代码用例
``` rust
let s = String::from("hello, world!");
let s: String = "hello, world".into();

let v = s.into(); // v的类型可以根据上下文推导得出
let v: u64 = s.into(); // 或者可以显示标注类型
```

``` rust
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
fn print(v: impl Into<IpAddr>) {
    println!("{:?}", v.into());
}
fn main() {
    let v4: Ipv4Addr = "2.2.2.2".parse().unwrap();
    let v6: Ipv6Addr = "::1".parse().unwrap();
   
    // IpAddr 实现了From<[u8; 4]>, 转换IPv4地址
    print([1, 1, 1, 1]);
    // Ipadd From<u16; 8> 转换为ipv6地址
    print([0xfe80, 0, 0, 0, 0xde, 0x4f, 0xf2, 0xabe]);
    // Ipv4Addr实现了 From<Ipv4Addr>
    print(v4);
    // Ipv6Addr实现了 From<Ipv6Addr>
    print(v6);
}
```

* **AsRef<T>/AsMut<T> trait**
是从引用到引用的转换

``` rust
pub trait AsRef<T> where T: ?Sized {
    fn as_ref(&ref) -> &T;
}
pub trait AsMut<T> where T: ?Sized {
    fn as_mut(&mut self) -> &mut T;
}
```

##### 7.6.6 操作符相关trait： Deref/DerefMut
解引用
只读解引用： `Deref(*v)`
可变解引用： `DerefMut(*v = ..)`
``` rust
pub trait Deref {
    // 解引用出来的结果类型
    type Target: ?Sized;
    fn deref(&ref) -> &Self::Target;
}
// DerefMut 继承了 Deref,  另外再额外提供了一个 deref_mut方法，用来获取可变解析用
pub trait DerefMut: Deref {
    fn deref_mut(&mut self) -> &mut Self::Target;
}
```
用例
``` rust
let mut x = 42;
let y = &mut x;
// 解引用，内部调用 DerefMut(其实现就是 *self)
*y += 1;
```

##### 7.6.7 其他trait: Debug/Display / Default
初始化一个数据结构时，可以部分初始化，然后剩余的部分使用Default::default()
``` rust
put trait Default {
    fn default() -> Self;
}
```
``` rust
use std::fmt;
#[derive(Clone, Debug, Default)]
struct Developer {
    name: String,
    age: u8,
    lang: Language,
}
// enum 不能derive Default
enum Language {
    Rust,
    Haskell,
}
// 手工实现Default
impl Default for Language {
    fn default() -> Self {
        Language::Rust
    }
}
impl Developer {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            ..Default::default()
        }
    }
}
```
### 8.智能指针
String 对堆上的值有所有权， 但是&str是没有所有权的，这是Rust智能指针和普通胖指针的区别。
``` rust
pub struct String {
    vec: Vec<u8>,
}
```
和普通指针不同的是， String实现了Deref和DerefMut, 这使得在String解引用时，会得到&str
``` rust
impl ops::Deref for String {
    type Target = str;
    fn deref(&self) -> &str {
        unsafe { str::from_utf8_unchecked(&self.vec) }
    }
}
```
定义：
在Rust中，凡是需要做资源回收的数据结构，且实现了Deref/DerefMut/Drop，都是智能指针。
String, 堆上创建的内存Box<T>, Vec<T>, Rc<T>, Arc<T>, PathBuf, 提供写时clone的Cow<'a B>, MutexGuard<T>, RwLockReadGuard<T>, RwLockWriteGuard<T> 都是智能指针

#### 8.1 Box<T>
在堆上申请内存
``` rust
struct Matrix {
    data: [u8; 505],
}
impl Default for Matrix {
    fn default() -> Self {
        Self { data: [0; 505] }
    }
}
let data = Box::new(Matrix::default());
```

#### 8.2 Cow<'a, B>
Cow 是Rust下提供写时克隆（Copy  on Write）的一个智能指针: 包裹一个只读借用，但如果调用者需要所有权或者需要修改内容时， 那么她就会clone借用的数据。
定义：使用enum实现Cow的borrow（只读借用）或者owned（对类型B拥有所有权）的分发
``` rust
pub enum Cow<'a, B> where B: 'a + ToOwned + ?Sized {
    Borrowed(&'a B),
    Owned(<B as ToOwned>::Owned),
}
```

因为Cow<'a, B>是智能指针，所以自然就需要实现Deref trait:
```rust
impl<B: ?Sized + ToOwned> Deref for Cow<'_, B> {
    type Target = B;
    fn deref(&self)  ->  &B {
        match *self {
            Borrowed(borrowed) => borrowed,
            Owned(ref owned) => owned.borrow(),
        }
    }
}
```
根据self是Borrowed还是Owned, 分别取其内容，生成引用：
对于Borrowed, 直接就是引用；
对于 Owned, 调用其borrow()方法，获得引用。
Cow虽是一个enum,  但是通过Deref的实现，可以获得统一的体验，比如Cow<str>, 使用的感觉和&str/String基本是一致的。
这里根据enum的不同状态来进行统一分发是第三种分发手段，之前讲过的有使用泛型参数做静态分发和使用trait object做动态分发。

##### 实例
``` rust
use std::borrow::Cow;
#[derive(Debug, Deserialize)]
struct User<'input> {
    #[serde(borrow)]
    name: Cow<'input, str>,
    age: u8,
}
fn main() {
    let input = r#"{"name": "Tyr", "age": 18}"#;
    let user: User = serde_json::from_str(input).unwrap();
   
   // 输出 borrowed Tyr
    match user.name {
        Cow::Borrowed(x) => println!("borrowed {}", x),
        Cow::Owned(x) => println!("owned {}", x),
    }
}
```

``` rust
use std::borrow::Cow;
use url::Url;
fn main() {
    let url = Url::parse("https://tyr.com/rust?page=1024&sort=desc&extra=hello%20world").unwrap();
    let mut pairs = url.query_pairs();
    assert_eq!(pairs.count(), 3);
    let (mut k, v) = pairs.next().unwrap();
    // 因为k, v都是Cow<str>, 她们用起来和&str或者String一样
    // 此刻，她们都是 Borrowed
    println!("key: {}, v: {}", k, v);
    // 当修改发生时，k 变成了Owned
    k.to_mut().push_str("_add_tail");
    print_pairs((k, v));
    print_pairs(pairs.next().unwrap());
    print_pairs(pairs.next().unwrap());
}
fn print_pairs(pair: (Cow<str>, Cow<str>)) {
    println!("key: {}, value: {}", show_cow(pair.0), show_cow(pair.1));
}
fn show_cow(cow: Cow<str>) -> String {
    match cow {
        Cow::Borrowed(x) => format!("Borrowed {}", x),
        Cow::Owned(x) => format!("owned {}", x),
    }
}
/* 输出
key: page, v: 1024
key: owned page_add_tail, value: Borrowed 1024
key: Borrowed sort, value: Borrowed desc
key: Borrowed extra, value: owned hello world
*/
```
#### 8.3 MutexGuard<T>
MutexGuard<T> 不但可以通过Deref提供良好的用户体验，还通过Drop trait来确保使用到的内存以外的资源在退出时进行释放。
MutexGuard<T>这个结构是在调用Mutex::lock生成的
 ``` rust
pub fn lock(&self) -> LockResult<MutexGuard<'_, T>> {
    unsafe {
        self.inner.raw_lock();
        MutexGuard::new(self)
    }
}
```
首先，她会取得锁资源，如果拿不到，会在这里等待；如果拿到了，会把Mutex结构体的引用传递给MutexGuard.
看一下MutexGuard的定义以及她的Deref和Drop的实现
``` rust
//  这里用must_use, 当你得到了却不使用MutexGuard时会报警
#[must_use = "if unused the Mutex will immediately unlock"]
pub struct MutexGuard<'a, T: ?Sized + 'a> {
    lock: &'a Mutex<T>,
    poison: poison::Guard,
}

impl<T: ?Sized> Deref for MutexGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe {
            &*self.lock.data.get()
        }
    }
 }
 
 impl<T: ?Sized> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &*self.lock.data.get()  }
    }
 }
 
 impl<T: ?Sized> Drop for MutexGuard<'_, T> {
     #[inline]
     fn drop(&mut self) {
        unsafe {
            self.lock.poison.done(&self.poison);
            self.lock.inner.raw_unlock();
        }
    }
 }
```
从上述drop代码可以看出，当MutexGuard结束时，Mutex会做unlock, 自动释放互斥锁。

#### 8.4 自己实现智能指针 MyString

* 设计背景：   **Rust下String在栈上会占用24Bytes**，然后在堆上存放字符串实际内容，对于较短的字符串是一种内存浪费
* 设计目标：   在字符串长到一定程度，再使用标准字符串，否则只在栈上存放字符串内容
* 实现方案： 
    参考Cow, 用enum处理：当字符串小于N字节时，直接用栈上的数组；否则用String。
    怎么设计呢，之前在内存管理部分讲过，当使用enum时，内存占用还包括额外的 **tag+为了对齐而使用的padding**。因为**String结构是8字节对齐**， 所以enum 最小就是 8 + 24 = 32个字节。
    **设计数据结构**：
    内部inline用一个字节表示字符串长度，用30个字节表示字符串内容，再加一个字节的tag, 正好构成32字节。可以和String放在一个enum里使用，暂且称之为MyString.
    |1Byte    |1Byte|                  30Bytes                   |
    |inline    | len  |                  data                        |
    |standard|                  | String(ptr|capacity|length)  |  // ptr指向堆内存实际内容地址
    
    为了让MyString 表现行为和&str一致，可以通过实Deref trait让String可以被解引用为&str.
    除此之外还可以实现Debug/Display 和 From<T>trait, 让MyString使用起来更方便。
``` rust
use std::{fmt, ops::Deref, str};
const MINI_STRING_MAX_LEN : usize = 30;
struct MiniString {
    len: u8,
    data: [u8; MINI_STRING_MAX_LEN],
}
impl MiniString {
    // 这里new接口不能暴露出去，保证传入的v的字节长度小于等于30
    fn new(v: impl AsRef<str>) -> Self {
        let bytes = v.as_ref().as_bytes();
        // 我们在拷贝内容时一定要使用字符串的字节长度
        let len = bytes.len();
        let mut data = [0u8; MINI_STRING_MAX_LEN];
        data[..len].copy_from_slice(bytes);
        Self {
            len: len as u8,
            data,
        }
    }
}
impl Deref for MiniString {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        // 由于生成MiniString的接口是隐藏的，她只能来自字符串，所以下面这行是安全的
        str::from_utf8(&self.data[..self.len as usize]).unwrap()
        // 也可以直接用unsafe版本
        // unsafe {
        //     str::from_utf8_unchecked(&self.data[..self.len as usize])
        // }
    }
}
impl fmt::Debug for MiniString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 这里由于实现了Deref trait, 可以直接得到一个&str输出
        write!(f, "{}", self.deref())
    }
}
#[derive(Debug)]
enum MyString {
    Inline(MiniString),
    Standard(String),
}
// 实现Deref接口对两种不同场景统一得到&str
impl Deref for MyString {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        match *self {
            MyString::Inline(ref v) => v.deref(),
            MyString::Standard(ref v) => v.deref(),
        }
    }
}
impl From<&str> for MyString {
    fn from(s: &str) -> Self {
        match s.len() > MINI_STRING_MAX_LEN {
            true => Self::Standard(s.to_owned()),
            _ => Self::Inline(MiniString::new(s)),
        }
    }
}
impl fmt::Display for MyString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}
fn main() {
    let len1 = std::mem::size_of::<MyString>();
    let len2 = std::mem::size_of::<MiniString>();
    println!("Len: Mystring {}, MiniString {}", len1, len2);
    let s1: MyString = "hello,world".into();
    let s2: MyString = "这是一个超过了三十个字节的很长很长很长的字符串".into();
    // debug 输出
    println!("s1: {:?}, s2: {:?}", s1, s2);
    // display输出
    println!("s1:{}({} bytes, {} chars), s2: {}({}bytes, {}chars)",
        s1, s1.len(), s1.chars().count(),
        s2, s2.len(), s2.chars().count()
     );
   
    // MyString 可以使用一切&str接口， 感谢Rust的自动Deref
    assert!(s1.ends_with("world"));
    assert!(s2.starts_with("这"));
}
/*
* 以上打印输出：
Len: Mystring 32, MiniString 31
s1: Inline(hello,world), s2: Standard("这是一个超过了三十个字节的很长很长很长的字符串")
s1:hello,world(11 bytes, 11 chars), s2: 这是一个超过了三十个字节的很长很长很长的字符串(69bytes, 23chars)
*/
```
### 9.数据结构
|分类|子类型|包含的数据结构| 含义|
|--|--|--|--|
|**原生类型**|基本类型| bool/ i8/i16/i32/u64/isize/usize/f32/f64| |
|原生类型|指针和引用| `*const T/ *mut T / &T / &mut T`||
|原生类型|集合容器| slice/str/array| |
|原生类型|特定容器| tuple | |
|**容器类型**|特定容器| Option<T> | 表达有值或者无值|
|容器类型|特定容器| Result<T, E> | |
|容器类型|特定容器| Cell<T>/RefCell<T> |单线程下内部可变性 |
|容器类型|特定容器| Rc<T>/Arc<T> |单线程/多线程引用计数 |
|容器类型|特定容器| Cow<'a, B> |写时克隆 |
|容器类型|特定容器| Box<T> |分配到堆内存 |
|容器类型|集合容器| Vec<T> | |
|容器类型|集合容器| String | |
|容器类型|集合容器| VecDeque<T> |循环缓冲区 |
|容器类型|集合容器| LinkedList<T> | 双向链表  |
|容器类型|集合容器| BinaryHeap<T>|二叉堆(最大堆) |
|容器类型|集合容器| HashMap<K, V>/BTreeMap<K, V>|哈希表/有序哈希表 |
|容器类型|集合容器| HashSet<T>/BTreeSet<T>|哈希集/有序哈希集 |
|**系统相关**|IO抽象| TcpStream/TcpListener UdpSocket SocketAddr| |
|系统相关|IO抽象| File MetaData PathBuf/Path OsString/OsStr| |
|系统相关|IO抽象| Thread JoinHandle| 线程 线程句柄 |
|系统相关|并发抽象| AtomicXXX| 并发原语 |
|系统相关|并发抽象| Mutex<T>/RwLock<T>| 共享内存 |
|系统相关|并发抽象| ConVar| 同步原语 |
|系统相关|并发抽象| Channel<T>| 并发通道 |
#### 9.1 切片 slice
##### 9.1.1 切片 DST(Dynamically Sized Type)定义和使用
在Rust中，切片是描述一组同一类型、长度不确定、在内存中连续存放的数据结构，用[T]来表述。
切片一般只出现在数据结构定义中，不能直接访问，在使用时主要有以下几种形式：
* &[T] 表示一个只读的切片引用
* &mut[T] 可写的切片引用
* Box<T> 堆上分配的切片
##### 9.2 Vec<T>和[T; n] 会转化为&[T]
Vec<T> 实现了Deref trait, 而array内建了到&[T]的解引用。
实例：
``` rust

```

##### 9.3 &Vec<T> 和 &[T]区别 
``` rust
let v = vec![1, 2, 3, 4];
let v1 = &v; // &Vec<T>
let v2 = v.as_slice();
let v3 = &v[..]
```
*  v是栈上指针，有【ptr1 + capacity + len】,其中ptr指向堆上实际数据内容；
* v1是v的引用/borrow, 栈上 【ptr2】, 其中ptr2指向ptr1
* v2和v3是slice, 栈上 【ptr3 + len】, 其中ptr3指向Vec<T>堆上数据的实际内容。

##### 9.4 切片和迭代器 Iterator
迭代器可以说是切片的孪生兄弟，切片是集合数据的视图，而迭代器定义了对集合数据的各种各样的访问操作。
iterator trait 有大量方法，大多情况下我们只需她的**关联类型Item**和**next()方法**。
* Item: 定义了每次从迭代器中取出的数据类型
* next() 是从迭代器中取下一个值的方法， 当返回None时，表示么有数据了
``` rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```
##### 9.5 特殊的切片 &str
String是特殊的Vec<u8>, 所以在String上做切片，也是一个特殊的结构&str.

##### 9.6 &String 和 &str的区别
String在解引用是会转换为&str
``` rust
let s = String:from("hello");
let s1 = &s;
let s2 = s.as_str();
let s3 = &s[..];
```
同上&Vec<T> 和 &[T]区别 :
s是栈上指针，有【ptr1 + capacity + len】，指向堆内存数据
s1是s的borrow, 栈上【ptr2】
s2/s3 相同的栈指针【ptr3 + len】
##### 9.7 Box<[T]> 和  Vec<T>
* Vec<T> 有额外的capacity, 可以增长；
* 而 Box<[T]> 一旦固定下来，没有capacity， 也无法增长。

|类型|相同点|不同点|
|--|--|--|
|&[T]|都在栈上有一个包含长度的胖指针|指向的位置 可以是栈arr,也可以是堆Vec，&[T]只是借用|
|Box<[T]> | 同上|只会指向堆，Box<T> 对数据**有所以权**|

Box<T>目前只能从Vec<T>中转换得到
``` rust
let mut v1 = vec![2, 3, 4];
v1.push(5);
// 从Vec<T>转换到Box<[T]>, 会丢弃多余的capacity
let b1 = v1.into_boxed_slice();
let mut b2 = b1.clone();
// Box<[T]> 固定大小，可以修改内部数据，但是不能push 增加数据。
b2[0] = 1;

let v2 = b1.into_vec(); // 又可以从Box<[T]>转换到Vec<T>
```
当需要在堆上创建固定大小数据且不希望自动增长，可以先创建Vec<T>, 再转换为Box<[T]>.
tokio的broadcast channel就使用了Box<[T]>.
##### 9.8 TODO slice和不同数据结构转换的总结图
todo: 画一下

#### 9.2 哈希表 HashMap
##### 9.2.1 使用
``` rust

use std::collections::HashMap;
fn main() {
    let mut map = HashMap::new();
    explain("empty", &map);
    map.insert('a', 1);
    explain("add 1", &map);
    map.insert('b', 3);
    map.insert('c', 2);
    explain("add 3", &map);
    map.insert('d', 4);
    explain("add 4", &map);
    //
    assert_eq!(map.get(&'a'), Some(&1));
    assert_eq!(map.get_key_value(&'b'), Some((&'b', &2)));
    map.remove(&'a');
    assert_eq!(map.contains_key(&'a'), false);
    assert_eq!(map.get(&'a'), None);
    explain("revomed", &map);
    // shrink后哈希表变小
    map.shrink_to_fit();
    explain("shrinked", &map);
}
fn explain<K, V>(name: &str, map: &HashMap<K, V>) {
    println!("{}: len: {}, cap: {}", name, map.len(), map.capacity());
}

/*
输出：
empty: len: 0, cap: 0
add 1: len: 1, cap: 3
add 3: len: 3, cap: 3
add 4: len: 4, cap: 7
*/
```

##### 9.2.2 原理
二次探寻
SMID查表
##### 9.2.3 HashMap的数据结构

### 10. 闭包 FnOnce/FnMut/Fn
#### 10.1 闭包的几种用法
* 方式1： 闭包a捕获了上下文中的a和b, 并通过**引用**来使用这两个自由变量
``` rust
闭包的表述形式1： |arg| { code };
let a = |msg: &str| { println!("{} {}", a, b); };
```
* 方式2： 除了用引用捕获自由变量外，还可以通过move关键字 `move |args| { code }`
    move会把变量的所有权从当前作用域移动到闭包的作用域。
#### 10.2 闭包的本质
定义：
闭包是一种匿名类型，一旦声明，就会产生一个新的类型 ，但这个新类型无法被其他地方使用。
这个类型 就像一个结构体，会包含所有 捕获的变量。

``` rust
fn main() {
    let c1 = || println!("hello, xhl");
    let c2 = |i: i32| println!("hello, {}",  i);
}

```
闭包的大小和参数、局部变量都无关，只跟捕获的变量有关。

### rust高阶：混合范式
#### 1.混合范式概述
* 命令式编程 70%
   -  面向过程 35%
    - 面向对象 35%
 * 函数式编程 30%
 总的来说，Rust是**面向类型系统**的编程语言，其抽象方式是**泛型和trait**. 
 在GAT稳定之前，其抽象金字塔还不是完整的，GAT出现后，抽象金字塔才算完整。
 
 #### 2.Rust与面向过程
 * C代码可以直译为Rust代码： 
C2Rust Demonstration
网址： https://c2rust.com
* Rust 可以方便的绑定到C接口
* C2rust: 将C代码迁移到Rust
https://www.5axxw.com/wiki/content/11eqsp

#### 3.Rust与面向对象
* Rust没有提供继承，而是面向组合： 组合优于继承

* trait对象 - Ad-hoc多态-动多态

* 泛型 - 参数化多态 - 静多态

#### 4. Rust与函数式编程
* 不可变
* 表达式
* 模式匹配
* 函数闭包一等公民
* 代数数据类型
* 高阶类型（GAT != High Rank Type）

##### 4.1 代数数据类型
Enum
Struct
##### 4.2 GAT General Association Type 
###### 4.2.1 关联类型 Associated Types
关联类型，就是对泛型类型区分了输入和输出类型，
**关联类型即输出类型**，这样使得trait更具工程优势。
``` rust
// Self和Rhs是输入类型
trait Add<Rhs> {
    type Sum; // Sum是输出类型
    fn add(&self, &Rhs) -> Sum;
}

impl Add<int> for int {
    type Sum = int;
    fn add(&self, rhs: &int) -> int {
        ...
    }
}

impl Add<Complex> for int {
    type Sum = Complex;
    fn add(&self, rhs: &Complex) -> Complex {
        ... 
    }
}
```
输入类型，用户在使用trait时可以指定，但是输出类型只有在实现trait时才可以指定。
关联类型赋予了trait的工程优势：
* 可读性和可扩展性。高内聚低耦合，更利于扩展性
* 易于重构。添加新的关联类型不会破坏现有的代码。

###### 4.2.2  泛型trait VS 关联类型（Associated Types）
那么问题来了，什么时候用泛型trait，什么时候用关联类型呢？
拿标准库中内置trait做个比较：
``` rust
// std::convert::From
put trait From<T> {  // 泛型trait
    fn from(T) -> Self;
}

// std::ops::Deref
pub trait Deref {
    type Target: ?Sized;  // 关联类型
    fn deref(&self) -> &Self::Target;
}
```
共同点：都允许延迟决定使用什么类型来实现该trait。
在任何使用关联类型的地方，都可以使用泛型trait来替代她，反过来则不一定。

差异：
有明显差异，那就是泛型trait允许为同一类型实现多次相同的trait, 比如From<T>:
``` rust
impl From<&str> for String { ... }
impl From<&String> for String { ... }
impl From<&mut str> for String { ... }
```
但是关联类型trait, 只允许为同一类型实现一次：
``` rust
impl Deref for String {
    type Target = str;
    #[inline]
    fn deref(&self) -> &str {
        unsafe {
            str::from_utf8_unchecked(&self.vec)
        }
    }
}
```
进一步从From<T>和Deref自身的语义来说：
*   From<T> 本身是类型转换，同一类型可以转换为多个类型，ok
*   Deref 本身表示解引用语义（智能指针），一个有指针语义的类型通过解引用后只允许得到一个确定且唯一的类型结果，否则就乱套了。

所以，需要从泛型trait和关联类型的语言特性和具体trait想表达的语义两个层面，来综合考虑哪一种trait形式更好。
###### 4.2.3 GAT 引入
在1.65之后就需要增加关联类型构造器（即，泛型关联类型）了
关联类型的一些使用规则：
* 关联类型出现在impl块和trait中
* trait关联类型有一个隐含的trait限定： core::marker::Sized
* trait关联类型包含 self/&self/&mut self接收者的时候称为trait方法。

**为什么需要泛型关联类型GAT**
rust-lang.github.io/generic-associated-types-initiative/design_patterns.html

✍️ Design patterns - Generic Associated Types Initiative (rust-lang.github.io)



### x.trait
