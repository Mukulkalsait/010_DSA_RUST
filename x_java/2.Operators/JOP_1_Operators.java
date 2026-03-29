
// BLUE:
// OPERANDS and OPERATOR
// "a, b,and var is OPERANDS".
// Symbols that tell compiler to perform certain operation. ( +,-,*,/ )
//
// WARN: Types of Operators:
// Arithmetic ;(binary unary & ternary)
// Relational;
// Assignment;
// Logicals;
// Bitwise;
//
//

public class JOP_1_Operators {
  public static void main(String[] args) {
      
    }

    // BLUE:
    // 1 Arithmetic operators:
    // TEST:
    // A. Unary operators
    // only one operand eg. a++ , b-- , ++c, --d;
    // WARN:[here a += 10 and a-= 12; are considerd as assigment operators so chechk
    // there.]
    // TEST:
    // B. Binary operators:
    // only 2 operands. With one Operator.
    // eg. +(add), -(subtracts), *(multiply), / (divide), %(Reminder);
    int A = 12;
    int B = 5;

    System.out.println("Addition is =" + (A + B));
    System.out.println("Subtraction is =" + (A - B));
    System.out.println("Multiplication is =" + (A * B));
    System.out.println("Division is =" + (A / B));
    System.out.println("Modulo is =" + (A % B));

    // FIX:
    // 2 Relational Operator:
    // DIV:
    // shows relation ---
    // 1. ==
    // 2. !=
    // 3. >
    // 4. <
    // 5. >=
    // 6. <=
    // the billow two will not exixit in java as types are allerady difined in ti so
    // no type chacking is required.
    // // 7. ===
    // // 8. !==
    //
    // int a1 = ;
    // int b1 = ;

    System.out.println("A = 12; and B = 5");
    System.out.println("A == B is " + (A == B));
    System.out.println("A != B is " + (A != B));
    System.out.println("A > B is " + (A > B));

    System.out.println("A < B is " + (A < B));
    System.out.println("A >= B is " + (A >= B));
    System.out.println("A <= B is " + (A <= B));

    // TAG:
    // Assignment operators
    // 1. =
    // 2. +=
    // 3. -=
    // 4. *=
    // 5. /=
    // 6. %=
    //

    System.out.println("A = 12; and B = 5");
    System.out.println("A = b will assign A the value of B.");
    System.out.println("A += n will add n in A eg (A += 2) is " + (A += 2));
    System.out.println("now value of A is =" + A);
    System.out.println("A -= n will substract n in A eg (A _= 2) is " + (A -= 2));
    System.out.println("now value of A is =" + A);
    System.out.println("A *= n will multyply n in A eg (A *= 2) is " + (A *= 2));
    System.out.println("now value of A is =" + A);
    System.out.println("A /= n will divide n in A eg (A /= 2) is " + (A /= 2));
    System.out.println("now value of A is =" + A);
    System.out.println("A %= n will add n in A eg (A %= 2) is " + (A %= 2));
    System.out.println("now value of A is =" + A);

    System.out.println("We will reassign value of A and B to 12 and 5 respectively now..." + (A = 12));
    System.out.println("now value of A and B is = " + A + " & " + B);

    // FIX:
    // Logicals operators:
    // TEST:
    // 1 . && ( and )
    // 2 . || ( or )
    // 3 . ! ( nor)
    // we allready know how to use && and || so only "!" nor will be shown in
    // exampole.

    System.out.println(" the '!' is not but opposit maker it will make true false and voice versa !!!");
    System.out.println("!(5>3) 5>3 is true but ! will make it = " + !(5 > 3));
  }}
