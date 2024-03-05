## 1장 변수
```
러스트에서 변수는 기본적으로 불편형(immutable)이다. 
안전하고 쉽게 동시성을 활용하기 위해 그렇게 고안되었다. 
물론 러스트에도 mut 키워드를 이용해 바인딩된 변수를 변경할수 있다.
```

변수에는 mut 키워드가 올수 있는 let과 mut 키워드가 올 수 없는 const 키워드로 나눌수 있다. 

#### const
const 를 사용하기 위해서는 변수명 뒤에 변수의 크기를 명시해야된다.
```rust
	 const PI: f64= 3.14159265359;
```



#### 섀도잉
let 키워드로 선언된 변수는 이전 변수명과 동일한 변수명으로 선언될수 있다. 변수명만 덮어쓰게 되는것이고 변수자체는 컴파일러에게 숨겨지는 개념이라고 볼수있다.
```rust
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");//10출력
    }

    println!("The value of x is: {x}");//6출력

```
예시처럼 스코프 내에서는 x가 10으로 변경된거처럼 보이지만 스코프 밖으로 돌아왔을때 x의 값은 변경되지 않고 그대로인것을 볼수있다.
```rust

 let spaces ="   ";
 let spaces = spaces.len();
 
 let mut spaces ="    ";
 spaces=spaces.len(); //error

```
mut의 경우 같은 자리의 값을 변경한다고 볼수 있기때문에 이 예시에서 타입이 일치하지 않아 컴파일 타임에 오류가 일어나지만 섀도잉의 경우 전혀 다른 위치에 데이터가 저장되기 때문에 다른 자료형을 대입해도 에러가 나지 않는다.

#### 자료형
정수형 : i8 i16 u8 u16 ~i128 u128 ,usize isize(아키텍처의 크기)
i 의경우 -2^x-1~2^x-1,u의 경우 0~2^x-1 의 범위의 정수를 표현 할 수 있다.
부동소수형: f32 f64 
불형 : bool 
true 와 false 이 리터럴이 들어간다. 
문자형 : char
ASCII 와 유니코드 모두 사용가능핟.
##### 정수의 리터럴
러스트에서 숫자리터럴은 다음 과 같이 표현 될 수 있다.
Number literals	Example
Decimal	98_222 큰 숫자 사이 사이에 '_'를 사용해 가독성을 높임
16진수	0xff ox 를 붙여 0~9 a~f 까지의 16진수를 표현 할 수 있다.
8진수	0o77 
2진수	0b1111_0000 2진수 표현 _를 사용 할 수 있다.
Byte (u8 only)	b'A' ASCII 문자에 해당하는 바이트리터럴을 나타낸다 (u8에만 사용가능)

##### 튜플형
 복합요소로 구성된 하나의 변수 구조 분해해서도 사용할수 있다.
 ```rust
 	
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;
 ```
 튜플의 값에 접근하기 위해선 .을 이용하면된다. 튜플은 첫번째 값부터 0 부터 시작하면 된다.
 ```rust
  let x = tup.0;/500;
  
 ```
#### unit형 ()
#### never type   !
 never type은 절대 정상적인 값이 반환되지 않는다는것을 의미 합니다.


####

 ### 함수
 함수의 매개변수와 반환값의 자료형은 생략가능하고
 void의 경우 반환값을 생략 가능하다.
 다음과 같이 표현된다.
  ```rust
  fn 함수명(){}
 fn 함수명(매개변수:자료형,매개변수:자료형){}
 fn 함수명()->(반환값:자료형){}
  ```
 #### 코드블럭
 명령문과 표현식을 구분하자면
 let x=10; 같은 문장은 x에 10을 대입하는 명령문이고
 x+20 은 값으로 평가될수 있는 표현식이다.
 러스트에서는 특이하게 블록또한 표현식으로 사용될수 있는데 예를 들어 아래 코드는 y에 대입될때 아래 코드 블럭이 실행되며 마지막줄인 ;이 들어가지 않은 x+1로 평가될수 있는 표현식으로 볼수있다. 
 익명 함수와는 구분지어서 사용되며 최초 1회만 실행되는 특성이 있다.
```rust
let y = {
        let x = 3;
        x + 1
    };
```



#### if문
러스트의 조건식에서는 표현식에 괄호를 칠 필요없다.
if문 블럭도 하나의 코드블럭이기 때문에 마지막 한줄에 세미콜론이 없으면 표현식을 평가된다
```rust
fn fun_if(x:i32){
    if(x>10){
        println!("big");
    }else if(x==10){
        println!("equal");
    }else {
        println!("mall");
    }
    let result:&str= if(x%2==0){"even"}else{"odd"};
    println!("{}",result);
}
```

#### String
let s="stirng" 와같은 경우 문자열은 코드 안에 박히는 윈시값으로 취급된다.
그렇기 때문에 참조형 String(&str)을 사용을 해야 String 에서 재공해주는 