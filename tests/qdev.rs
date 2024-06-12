#![allow(unused)]

use std::{f32::consts::PI, ptr::null};

use anyhow::{Ok, Result};
use axum::Json;
use serde::de;
use serde_json::json;

async fn post_test() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8300")?;
    let req_create_post = hc.do_post(
        "/api/post/create", 
        json!({
            "title": "test",
            "author" : 100000,
            "brief": "广告招租位",
            "content" : "test",
            "tags": ["广告", "测试"]
        })
    );

    req_create_post.await?.print().await?;

    let req_delete_post = hc.do_post(
        "/api/post/delete", 
        json!({
            "pid": "f354c332-a2ad-42a6-bdae-99b535395ef1",
        })
    );

    req_delete_post.await?.print().await?;

    let req_update_post = hc.do_post(
        "/api/post/update", 
        json!({
            "pid": "dfebd659-9cd2-4c9a-8a42-85fbf561bd9a",
            "title": "update",
            "brief": "广告招租位update",
            "content": "更新！！！！",
            "tags": ["广告", "测试", "更新"]
        })
    );

    req_update_post.await?.print().await?;

    let req_list_post = hc.do_get(
        "/api/post/infolist"
    );
    req_list_post.await?.print().await?;

    let get_content = hc.do_get(
        "/api/post/content?pid=dfebd659-9cd2-4c9a-8a42-85fbf561bd9a"
    );
    get_content.await?.print().await?;
    Ok(())
}


async fn user_test() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8300")?;
    let req_register_user = hc.do_post(
        "/api/user/register", 
        json!({
            "name": "root",
            "passwd":  "root",
        })
    );
    req_register_user.await?.print().await?;

    let delete_user = hc.do_post(
        "/api/user/delete", 
        json!({
            "uid": 10000002
        })
    );
    delete_user.await?.print().await?;

    let update_user = hc.do_post(
        "/api/user/update", 
        json!({
            "uid": 10000000,
            "passwd": "root",
            "name": "root",
            "avatar": null,
            "bio": null,
        })
    );
    update_user.await?.print().await?;
    
    let login_bad = hc.do_post(
        "/api/user/login",
        json!({
            "uid": 10000000,
            "passwd": "toot",
        })
    );
    login_bad.await?.print().await?;
    
    let login_notfound = hc.do_post(
        "/api/user/login",
        json!({
            "uid": 1000,
            "passwd": "toot",
        })
    );
    login_notfound.await?.print().await?;

    let login_ok = hc.do_post(
        "/api/user/login",
        json!({
            "uid": 10000000,
            "passwd": "root",
        })
    );
    login_ok.await?.print().await?;

    Ok(())
}


async fn meg_test() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8300")?;
    let remark = hc.do_post(
        "/api/user/remark", 
        json!({
            "uid": 10000000,
            "pid": "dfebd659-9cd2-4c9a-8a42-85fbf561bd9a",
            "content": "remark",
        })
    );
    remark.await?.print().await?;
    let remark_list = hc.do_get(
        "/api/post/remarks?pid=dfebd659-9cd2-4c9a-8a42-85fbf561bd9a"
    );
    remark_list.await?.print().await?;

    let chat_list = hc.do_get(
        "/api/chatlist"
    );
    chat_list.await?.print().await?;

    let chat = hc.do_post(
        "/api/user/chat", 
        json!({
            "uid": 10000000,
            "content": "chat",
        })
    );
    chat.await?.print().await?;
    Ok(())
}

async fn basic_tset() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8300")?;
    hc.do_get("/api/basicinfo").await?.print().await?;
    let update = hc.do_post(
        "/api/updateinfo", 
        json!({
            "sitebasicinfo": {
                "title": "9999",
                "subtitle": "",
                "description": "",
                "author": "",
                "favicon": "",
                "avatar": ""
            },
            "mylinks": {
                "github": "",
                "bilibili": "",
                "zhihu": "",
                "qq": "",
                "wechat": "",
                "gitee": ""
            }
        })
    ).await?.print().await?;
    Ok(())
}

#[tokio::test]
async fn qdev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8300")?;

    hc.do_get("/").await?.print().await?;

    // post_test().await?;

    // meg_test().await?;

    // user_test().await?;
    
    // basic_tset().await?;
    add_post().await?;

    Ok(())
}

async fn add_post() -> Result<()> {
    let hc = httpc_test::new_client("http://124.223.209.159:8300")?;
    let post1 = hc.do_post(
        "/api/post/create", 
        json!({
            "title": "可持久化数据结构",
            "author" : 10000000,
            "brief": "咕咕",
            "content" : 
            r#"
咕咕咕。。。
            "#,
            "tags": ["数据结构", "算法竞赛"]
        })
    );
    post1.await?;

    let post2 = hc.do_post(
        "/api/post/create", 
        json!({
            "title": "闵可夫斯基和",
            "author" : 10000000,
            "brief": "计算几何进阶! 点集的加法! 凸包合并!",
            "content" : 
            r#"
定义: 对于欧式空间上的两个点集 $A, B$, 存在点集 $C$ 满足 $\{a + b \in C|a \in A,b\in B \}$ （$a, b$均为向量的形式）, 则称 $C$ 为$A, B$的闵可夫斯基和, 即
$$
    C = A + B
$$

差 (different):
$C = A - B = \{a - b | a \in A, b \in B\} = A + (-B)$

表示对$\forall a \in A, \forall b \in B, \exists c \in C (c = a - b)$

或者可以这样理解$\forall c \in C, \exists b \in B, \exists a \in A (a =  c + b)$

==warning==: 这里的加号和减号不是一对可以互逆的运算, $(A - B) + B \neq A$

[wikipedia](https://en.wikipedia.org/wiki/Minkowski_addition)

- 例题(第一道过的洛谷黑题):

[战争](https://www.luogu.com.cn/problem/P4557)

题目大意: 有两个部落$A,B$, 两个部落分别有$n, m$个人, 如两个部落所围成的凸包没有重合的点, 则不会发生战争. 现在$B$部落的全部人打算向$(x, y)$向量迁移, 有$q$次询问, 问是否会发生战争.

思路是找到一个点集$C$, 满足对 $\forall c \in C, \exists b \in B, c + b \in A$, 这和前面差的定义是相同的, 故此题转化为了求$C = A - B$, 然后每次询问二分判断点是否在$C$内.

参考代码 (包含了**闵可夫斯基和**以及**判断一个点是否在凸包内**):
```cpp
#include<bits/stdc++.h>
using namespace std;
using Real = long long;
struct Point {
    Real x, y;
    Point() {}
    Point(Real x, Real y) : x(x), y(y) {}
    friend istream& operator >> (istream &is, Point& v) {
        return is >> v.x >> v.y;
    }
    Point& operator+= (const Point& p) {
        x += p.x, y += p.y;
        return *this;
    }
    Point& operator-= (const Point& p) {
        x -= p.x, y -= p.y;
        return *this;
    }
    Point operator+ (const Point& p) const {
        return Point(*this) += p;
    }
    Point operator- (const Point& p) const {
        return Point(*this) -= p;
    }
    Point operator- () {
        return Point(-x, -y);
    }
    friend Real crs(const Point& a, const Point& b) {
        return a.x * b.y - a.y * b.x;
    }
    bool operator< (const Point& p) const {
        if(x != p.x) return x < p.x;
        return y < p.y;
    }
};
using Points = vector<Point>;
enum position { CCW = 1, CW = -1, ON = 0 };
int ccw(const Point& a, Point b, Point c) {
    b -= a, c -= a;
    if(crs(b, c) > 0) return CCW;
    if(crs(b, c) < 0) return CW;
    return ON;
}
int ccw(Point a, Point b) {
    return ccw(Point(0, 0), a, b);
}
Points Convexhell(Points& ps) {
    int n = ps.size(), k = 0;
    Points res(2 * n);
    sort(ps.begin(), ps.end());
    for(int i = 0; i < n; res[k ++] = ps[i ++]) {
        while(k >= 2 && ccw(res[k - 2], res[k - 1], ps[i]) != CCW) {
            k --;
        }
    }
    for(int i = n - 2, t = k + 1; i >= 0; res[k ++] = ps[i --]) {
        while(k >= t && ccw(res[k - 2], res[k - 1], ps[i]) != CCW) {
            k --;
        }
    }
    res.resize(k - 1);
    return res;
}
Points Minkowski(Points& A, Points& B) {
    int n = A.size(), m = B.size();
    Points res (n + m + 1);
    res[0] = A[0] + B[0];
    int k = 1, i = 0, j = 0;
    int cA = 0, cB = 0;
    while(cA < n && cB < m) {
        int nxi = (i + 1) % n, nxj = (j + 1) % m;
        Point x = A[nxi] - A[i];
        Point y = B[nxj] - B[j];
        if(ccw(x, y) == CCW) {
            res[k ++] = res[k - 1] + x;
            i = nxi, cA ++;
        } else if(ccw(x, y) == CW) {
            res[k ++] = res[k - 1] + y;
            j = nxj, cB ++;
        } else {
            res[k ++] = res[k - 1] + x + y;
            j = nxj, i = nxi;
            cA ++, cB ++;
        }
    }
    while(cA < n) {
        int nxi = (i + 1) % n;
        res[k ++] = res[k - 1] + A[nxi] - A[i];
        cA ++, i = nxi;
    }
    while(cB < m) {
        int nxj = (j + 1) % m;
        res[k ++] = res[k - 1] + B[nxj] - B[j];
        cB ++, j = nxj;
    }
    res.resize(k - 1);
    return res;
}
bool check_in(Point& p, Points& ps) {
    int n = ps.size();
    int O = 0, l = 1, r = n - 1;
    if(ccw(ps[O], ps[r], p) == CCW || ccw(ps[O], ps[l], p) == CW) {
        return false;
    }
    while(l + 1 < r) {
        int m = l + r >> 1;
        if(ccw(ps[O], ps[m], p) == CCW) {
            l = m;
        } else {
            r = m;
        }
    }
    if(ccw(ps[l], ps[r], p) == CW) {
        return false;
    } else {
        return true;
    }
}
void solve() {
    int n, m, q;
    cin >> n >> m >> q;
    Points A(n), B(m);
    for(int i = 0; i < n; i ++) {
        cin >> A[i];
    }
    for(int i = 0; i < m; i ++) {
        cin >> B[i];
        B[i] = -B[i];
    }
    A = Convexhell(A); n = A.size();
    B = Convexhell(B); m = B.size();
    Points C = Minkowski(A, B);
    while(q --) {
        Point query;
        cin >> query;
        if(check_in(query, C)) {
            cout << "1\n";
        } else {
            cout << "0\n";
        }
    }
}
int main() {
    ios::sync_with_stdio(false), cin.tie(0);
    int _ = 1;
    while(_ --) solve();
}
```
            "#,
            "tags": ["计算集合", "算法竞赛"]
        })
    );
    post2.await?;


    let post3 = hc.do_post(
        "/api/post/create", 
        json!({
            "title": "rust 小记",
            "author" : 10000000,
            "brief": "rust是一种比较新的语言，对标cpp，拥有优越的性能和安全的内存，非常适合高并发的软件，相较于其他语言来说比较难入门，这里给出了部分简单语法。",
            "content" : 
            r#"
## 入门
### 环境配置
#### 安装
- Linux
```shell
$ curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
```
- windows
前往 https://www.rust-lang.org/install.html 并按照说明安装 Rust

#### 检查

```shell
$ rustc --version
```

#### 更新

```shell
$ rustup update
```

#### 卸载

```shell
$ rustup self uninstall
```

#### 本地文档

```shell
rustup doc
```

### 编译命令与Cargo

#### rustc

和gcc差不多的用法
```shell
$ rust [-g] {filename.rs} [-o]
```
#### Cargo
- 创建一个项目
```shell
$ cargo new {dirname} 
```
- 构建
```shell
$ cargo build [--release]
```
只确保通过编译
```shell
cargo check
```
- 运行
```shell
$ cargo run
```

### 变量

rust中变量默认设置为不可变, 只能被覆盖, 需要加`mut`前缀设置为可变. 
`const`修饰常量, 常量可以是个算式

#### 整型

|长度   | 有符号   |无符号 |
|------ |------   |-------|
|8-bit  |`	i8  ` |`	u8 `|
|16-bit |`	i16 ` |`	u16`|
|32-bit |`	i32	` |`u32 ` |
|64-bit	|`i64  `  |`	u64`|
|128-bit|`i128	` |`u128` |
|arch	  |`isize`  |`usize`|

向$0$取整

#### 浮点型

`f32`, `f64`

#### 布尔型

`bool`: `enum { true, false }`

#### 字符类型

UTF-8编码(2-bit)
`char`

#### 元组
`(i32, i32, i32)`
`.`访问

#### 数组
`[i32; 10]` $10$个`i32`变量

#### 枚举类型
```rust
enum IpAddr{
    V4: (u8, u8, u8, u8),
    V6: String,
}
```
枚举类型中的值是可选项，用`::`来构造一个实例
```rust
let home = IpAddr::V4(192, 0, 0, 10);
```
- 特殊的枚举类型`Option`
这是一个定义在标准库内的类型
```rust
enum Option<T> {
    None,
    Some(T),
}
```

### 函数
无返回值 (void) 的函数
```rust
fn function() {
    // -- skip -- 
}
```

带返回值的函数
```rust
fn function() -> i32 {
    // 两中返回值的方式都是可以的
    5
    //return 5;
}
```

### 控制流 

#### if
兼容 C 风格, 可以带`()`
```rust
if x < 5 {
    x += 1;
} else if x < 10 {
    x += 2;
} else {
    x += 3;
}
```

#### loop
无限循环体
```rust
loop {
    x += 1;
    if x >= 100 {
        break;
    }
}
```

#### while
兼容 C 风格, 可以带`()`
```rust
while x < 5 {
    x += 1;
}
```

#### for
遍历集合
```rust
let a = [0, 1, 2, 3, 4, 5];
for i in a {
    print!("{}", i);
}
```
逆序遍历索引
```rust
for i in (0..5).rev() {
    // --skip--
}
```

#### match
类似 C 语言中的switch
```rust
enum IpAddr {
    V4(String),
    V6(String),
}
let home = IpAddr::V4(String::from("192, 0, 0, 1"));
let ip = match home {
    IpAddr::V4(s) => {
        s
    },
    IpAddr::V6(s) => s
    // => 表示产生的效果

    other => String::from("0, 0, 0, 0"),
    //match 是穷尽的 
};
```

#### if let
弱化版的match
```rust
if let IpAddr::V4(s) = home{
    println!("{}", s);
} else {
    cnt += 1;
}
println!("{}", cnt);
```

### 结构体定义与实例

贴个自己写的 BIT 基本就会用了

```rust
fn main () {
    let mut bit = BIT::new(100);
    for i in (1..bit.N) {
        bit.add(i, 1);
    }
    for i in (1..bit.N) {
        println!("{}", bit.get(i));
    }
}
struct BIT {
    N: usize,
    p: Vec<i32>,
}

impl BIT {
    fn new(n: usize) -> Self {
        BIT {
            N: n,
            p: vec![0; n + 1],
        }
    }
    fn add(&mut self, mut x: usize, y: i32) {
        while x < self.N {
            self.p[x] += y;
            x += x & (!x + 1);
        }
    }
    fn get(&self, mut x: usize) -> i32 {
        let mut res = 0;
        while x > 0 {
            res += self.p[x];
            x -= x & (!x + 1);
        }
        res
    }
}
```

### 几种常见的封装集合
#### Vector
定义
```rust
let v1 = vec![1, 2, 3, 4];
let v2 = vec![0; 5];
let  mut v3: Vec<i32> = Vec::new();
```
几个常用的函数
- push
- pop
- append
```rust
// 将 v2 中的元素移动到 v1
v1.append(&mut v2);
```
- sort | sort_by
```rust
// 逆序排序
v1.sort_by(|a, b| b.cmp(a));
```
- is_empty
- len
- resize

#### String
定义
```rust
let s1 = String::from("hello, ");
let s2 = "world!".to_string();
let s3 = s1 + &s2;
```
采用 UTF-8 编码，利用索引引用的时候两个连续的索引表示一位`char`。
#### Hash Map
定义
```rust
let mut map = HashMap::new();
map.insert("red".to_string(), 1);
map.insert("blue".to_string(), 2);
map.insert("red".to_string(), 10);  // 覆盖
map.entry("red".to_string()).or_insert(10); // 不存在则插入一个值
let conut = map.entry("bule".to_string()).or_insert(0);
*conut += 1; // 更新map中的数据
let x = map.get(&"red".to_string()).copied().unwrap_or(0); // 得到一个不可变的值
```
            "#,
            "tags": ["编程语言", "Rust"]
        })
    );
    post3.await?;

    let post4 = hc.do_post(
        "/api/post/create", 
        json!({
            "title": "ArchLinux安装指南",
            "author" : 10000000,
            "brief": "ArchLinux入指南，简述了一些步骤以及其中的一些概念。",
            "content" : 
            r#"
以下是我自己在真机上安装过多次可行的方法。

在真机安装和在虚拟机上安装还是有很大的区别，此文章简单说明了其中的一些必要的步骤。

## 准备安装介质

从镜像网站上 https://mirrors.ustc.edu.cn/ 获取映像文件。

用 `rufus` 等烧录文件将映像以 `DD` 形式烧录到U盘，`ISO` 不行，至少在我的电脑上存在这样的问题。

## 启动到Live环境

Live 环境中包含的 Archlinux 安装所需的工具。

电脑重启后按 F2 进入 BIOS（不同厂商的电脑进入 BIOS 的按键可能不一样）。

如果是双系统安装，需要在 BIOS 中将 Sevure Boot 设置为 Disable，装完后也可以重新启用。

### 联网

利用 `iwctl` 进行联网

`device list` 查看已安装网卡

`station DEVICE scan` 扫描周边的 wifi

`station DEVICE get-networks` 显示扫描到的 wifi

`station --passphrase=PASSPHRASE station DEVICE connect SSID ` 连接到所选的 wifi

### 启动方式的选择

查看启动发生BIOS or UEFI

```shell
[ -d /sys/firmware/efi ] && echo UEFI || echo BIOS
```

### 磁盘分区

创建磁盘分区：需要一个根分区和一个EFI系统分区（UEFI 启动方式需要这个分区），可以选择创建一个交换分区。

如果是双系统， EFI 分区可以直接挂载 Windows 已经创建好的 EFI 分区。

交换分区是一种虚拟内存的表现形式，系统为了应付需要用到大量内存的场景，将磁盘上的一部分空间当作内存使用。

> **swap文件创建规则**（参照oracle官方文档设定的标准）:
> - 4G以内的物理内存，SWAP 设置为内存的2倍。
> - 4-8G的物理内存，SWAP 等于内存大小。
> - 8-64G 的物理内存，SWAP 设置为8G。
> - 64-256G物理内存，SWAP 设置为16G。

利用`fdisk -l`查看当前磁盘分区状况，找到所需要安装系统的磁盘，利用`cfdisk`进行分盘（这是一个图形化的分区修改界面）。

如果是一块未分区的磁盘，会提示使用哪种分区表，按照官方文档的说法 BIOS 启动应采用 MAR 分区表，UEFI 启动应采用 GPT 分区表。

> 具体详见 [wiki-引导加载程序](https://wiki.archlinuxcn.org/wiki/Arch_%E7%9A%84%E5%90%AF%E5%8A%A8%E6%B5%81%E7%A8%8B#Boot_loader)

虚拟机分区例子（传统引导）：

![1](/images/Archlinux/imgae.png)

#### 格式化分区

> 详见 [wiki-文件系统](https://wiki.archlinuxcn.org/wiki/%E6%96%87%E4%BB%B6%E7%B3%BB%E7%BB%9F#%E5%88%9B%E5%BB%BA%E6%96%87%E4%BB%B6%E7%B3%BB%E7%BB%9F)

```shell
mkfs.ext4 /dev/root_partition（根分区）
mkswap /dev/swap_partition（交换空间分区）
mkfs.fat -F 32 /dev/efi_system_partition（EFI 分区）
```

#### 挂载分区

> 挂载：指的就是将设备文件中的顶级目录连接到 Linux 根目录下的某一目录（最好是空目录），访问此目录就等同于访问设备文件。

```shell
mount /dev/root_partition（根分区） /mnt
mount --mkdir /dev/efi_system_partition（EFI 系统分区） /mnt/boot
swapon /dev/swap_partition（交换空间分区）
```
要注意挂载顺序，一定是先挂载根分区再挂载 EFI 分区。

### 安装

1. 选择镜像

官方所有的镜像源：https://archlinux.org/download/

```shell
# 备份
cp /etc/pacman.d/mirrorlist /etc/pacman.d/mirrorlist.bk
# 选择在中国的最快镜像源
reflector --verbose --country 'China' -l 200 -p https --sort rate --save /etc/pacman.d/mirrorlist
# 清华园镜像
Server = https：//mirrors.tuna.tsinghua.edu.cn/archlinux/$repo/os$arch
# 中科大镜像
Server = https://mirrors.ustc.edu.cn/archlinux/$repo/os$arch
# 更新
pacman -Sy
```

文件 /etc/pacman.d/mirrorlist 定义了软件包会从哪个镜像下载，优先级从上向下依次递减。

2. 安装基本系统

```shell
pacstrap /mnt base base-devel linux linux-firmware linux-headers
```

其中 `pacstryp` 是一个安装脚本。

其中 base 和 base-devel 包含一系列的系统软件，必须安装。linux是内核，linux-firmware 是一些驱动，linux-headers 是内核头文件。

当然，内核也可以选择不是原版的 Linux，比如 Linux-lts，可以在 wiki 上找到其他版本的内核 [wiki-内核](https://wiki.archlinuxcn.org/wiki/%E5%86%85%E6%A0%B8)。

## 配置新系统

**创建Fstab文件**

用以下命令生成 fstab 文件 (用 -U 或 -L 选项设置 UUID 或卷标)：

```shell
genfstab -U /mnt >> /mnt/etc/fstab
```

`fastab` 该文件包含了当前分区挂载情况，生成之后建议检查里面的内容是否正确。

**chroot 到新安装的系统：**

从 Live 环境进入挂载点。

```shell
arch-chroot /mnt
```

**下载一些必要的软件**

利用 `pacman` 进行安装。

```shell
pacman -Syy # 更新软件包列表
```

---

`networkmanager` 连接互联网

`net-tools` 包含 ifconfig 等命令

`vim` 文本编辑器

`dhcpcd` 分配 ip 地址

`openssh` ssh 服务

`git`

`grub` 用来引导系统

`os-prober` 双系统可选安装，可以选择进入那个系统

`efibootmgr` UEFI引导必装

`intel-ucode/amd-ucode` CPU 微码必装

`man` 查看软件包的文档

`ntfs-3g` 访问 ntfs 格式的磁盘，双系统必装

`noto-fonts-cjk` 和 `noto-fonts-emoji` 谷歌设计的中文字体

**设置中文字符**

编辑 locale.gen 将中文（zh_CN_UTF-8）以及英文（en_US_UTF-8）前的注释去掉。

输入`locale-gen`设置字符集。

在文件 `/etc/locale.conf` 中写入 `LANG=en_US.UTF-8` 保存。

```shell
echo LANG=en_US.UTF-8 > /etc/locale.conf
```

这里需要设置成英文，在 TTY 状态下中文会乱码。

> TTY 没有安装图形界面时的状态。

**设置时区(以上海为例)**

```shell
ln -sf /usr/share/zoneinfo/Asia/Shanghai /etc/localtime
```

生成`/etc/adjtime`

```shell
hwclock --systohc
```

**配置主机名**

```shell
echo YOURPC > /etc/hostname
```

以及配置密码

```shell
passwd
```

**自启动 NetworkManager， ssh，dhcpcd**

```shell
systemctl enable NetworkManager sshd dhcpcd
```

### 安装引导程序

GRUB 默认不支持 os-prober，需要做如下修改：

编辑`/etc/default/grub`，将最后一行`#GRUB_DISABLE_OS_PROBER=false`取消注释

对于UEFI

```shell
grub-install --target=x86_64-efi --efi-directory=/boot --bootloader-id=GRUB
grub-mkconfig -o /boot/grub/grub.cfg
```

对于BIOS

```shell
grub-install --target=i386-pc /dev/sda
grub-mkconfig -o /boot/grub/grub.cfg
```

退出重启
```shell
exit
reboot
```

双系统重启后可能还是找不到 Windows 的启动引导，可以在进入系统后再执行一次`grub-mkconfig -o /boot/grub/grub.cfg`，也可以自己手动在grub里添加引导入口。

### 配置本地用户

进入新系统后可以用 NetworkManager 联网

显示 wifi 列表

```shell
nmcli dev wifi list
```

连接指定 wifi

```shell
nmcli dev wifi connect SSID password PASSWORD
```

创建普通用户

```shell
useradd -m -G wheel YOURNAME
passwd YOURNAME
```

wheel 是 linux 本身包含的一个用户组。

配置 sudo

编辑`/etc/sudoers`，取消`#wheel ALL=(ALL:ALL) ALL`的注释。

`su YOURNAME` 进入本地用户，下文中的命令中若出现`$`则表示在本地用户执行的。

### 网络配置

将下面内容写入到 `/etc/hosts`
```
127.0.0.1   localhost
::1         localhost
127.0.1.1   yourPCname.localdomain  yourPCname
```
### 配置AUR

全称 Arch User Repository, 简称 AUR。

修改 `/etc/pacman.conf`

将 `[multilib]` 的两行取消注释，这个仓库包含 Arch 官方软件仓库的 32 位软件和链接库。

配置 Archlinuxcn

添加两行

```
[archlinuxcn]
Server = https://mirrors.ustc.edu.cn/archlinuxcn/$arch
```

这个仓库是 Arch Linux 中文社区驱动的非官方用户仓库。包含中文用户常用软件、工具、字体\美化包等。

安装 `archlinuxcn-keyring` 导入 GPG 密钥

**安装paru 或者 yay**

```shell
$ sudo pacman -S paru
```

### 安装驱动

#### 显卡驱动

我的笔记本是 Amd 的核显和 Nvidia 的独显，装的内核是 Linux，这里举个例子。

```shell
$ sudo pacman -S xf86-video-amdgpu mesa lib32-mesa 
$ sudo pacman -S nvidia nvidia-utils lib32-nvidia-utils
```
同时把 `kms` 从 `/etc/mkinitcpio.conf` 里的 HOOKS 数组中移除，并重新生成 initramfs。 这能防止 initramfs 包含 `nouveau` 模块，以确保内核在早启动阶段不会加载它。

```shell
$ mkinitcpio -p linux
```

Nvidia 驱动的安装详见 [wiki-Nvidia](https://wiki.archlinuxcn.org/wiki/NVIDIA)

### 安装桌面环境

这里简单介绍对 Gnome 的一些配置。

> 在我的电脑上 Gnomo 账户注销再登陆就会直接卡住 QAQ，考虑找别的桌面替代。

#### 安装 Gnome 桌面

```shell
$ sudo pacman -S gnome gnome-tweaks gnome-extra gdm
```

**开机自启动**
```shell
$ sudo systemctl enable gdm
```

新安装好的Gnomo用起来有诸多不便，没有系统托盘，没有 Dock 栏，没有桌面图标等等，我们需要安装拓展插件来弥补这些功能的缺失。

**安装支持库**

```shell
$ sudo pacman -S gnome-browser-connector
```

**以下是我比较建议安装的几个插件:**

自定义主题

[User Themes](https://extensions.gnome.org/extension/19/user-themes/)

显示桌面图标

[Desktop Icons NG (DING)](https://extensions.gnome.org/extension/2087/desktop-icons-ng-ding/)

显示暂时回到桌面图标

[Show Desktop Button](https://extensions.gnome.org/extension/1194/show-desktop-button/)

Dock栏

[Dash to Dock](https://extensions.gnome.org/extension/307/dash-to-dock/)

系统托盘

[AppIndicator and KStatusNotifierItem Support](https://extensions.gnome.org/extension/615/appindicator-support/)
            "#,
            "tags": ["操作系统", "Linux"]
        })
    );
    post4.await?;

    Ok(())
}