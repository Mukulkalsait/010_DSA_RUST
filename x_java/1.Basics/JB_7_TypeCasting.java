
/**
 *   ==================================================================================================================================================
 *   JB_7_TypeCasting
 *  
 *   GREEN:
 *   A.Type Conversion or( Widening or Implicit(unclear) converison...)
 *   B.Type Casting or (Narrowing or Explicit(clear) converison...)
 *
 *   A
 *   TODO: A.Type Conversion or(Widning or Implicit converison...)
 *   WARN: Conditions:
 *   1. Destination type -> Source Type { in size mentionsd in 2.Datatypes}
 *   2. Type Compitability (like number in number or char in numbeer (assci))
 *
 *   in type Conversion type is convertied into someting wider or bigger then actual 
 *   hence we dont know wahat actually is in variable hence conversion or widening or Implisite conversion.
 *
 *   FIX: the chain of ONE-DIRECTIONAL-CONVERSION
 *   byte->short->int->float->long->double
 *   small___<<<<<<<{size}<<<<<<<<<<___Big
 *
 *
 *   TODO: B.Type Casting or (Narrowing or Explicit converison...)
 *   WARN:
 *   type castion is nouthing but FORCEFULLY changing data types in JAVA....
 *   it is done in reverse manner (from bigger to smaller so its LOSSY conversio.)
 *
 *   in type Casting we cast (create) a smaller ( narrowing ) variabl which is fixed and we know what it is hence Explisite.
 *   FIX: there is another way of doing type casting thats in between NON-COMPITABLE ( like int into char/ string)
 *
 *   ==================================================================================================================================================
 */

import java.util.*;

public class JB_7_TypeCasting {
  public static void main(String[] args) {

    // TODO: A.Type Conversion or(Widning or Emplicit converison...)

    // eg. 1
    int a = 23;
    long b = a;
    System.out.print(b);
    System.out.println("The int a converted into lnog b becouse both are number and int is 4bite and long is 8 bite.");

    // eg. 2
    Scanner sc = new Scanner(System.in);
    float eg2_a = sc.nextFloat(); // this is giving error type mismatched type
    // cannot convert from float ot int...

    float eg2_b = sc.nextInt(); // whereas this will work without doubt cos flot is 8bit and we are saving int
                                // 4bit in it
    System.out.println(eg2_b);
    //
    //
    //
    //

    //
    //
    //
    //
    //
    //
    // TODO: B.Type Casting or (Narrowing or Explicit converison...)
    //
    // billow the value 23.234 is float but java will take it as double so use "f"
    // for telling its float and then converti into Into
    // this conversion will loose some data therefore
    // TODO: No matter how big number is after the (.) decimal it will be ignored
    // eg. 99.999999 will become 99 in int and now 100.
    // Tyope A: from big to small

    float a3 = 23.234f;
    int b3 = (int) a3;
    System.out.println(b3);

    //
    // TODO:
    // Type B: between unCOMPATABLE variables

    char ch = 'a';
    int charIntoCharCodeIntiger = ch;
    System.out.println(charIntoCharCodeIntiger);
    // here ch = a got directly converted ot its charcode thats 96; and this is how
    // the conversion happen.
    //
    //
    //
    //
    sc.close();
  }
}
