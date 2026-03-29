import java.util.*;

public class JFM_1_funBasic {

  // FIX: a function inside a class is know as METHODS
  // but all fun in JAVA are inside class so all FUNCTOIN = METHODS in java.

  public static int CalcualteSum(int n1, int n2) { // TEST: (int n1 , int n2) are FORMAL PARAMERERS
    int sum = n1 + n2;
    return sum;
  }

  //
  // TEST:
  //
  //
  // WARN: the main function should be allways at the end.
  // all the abouve fun should be inside class

  public static void main(String[] args) {

    // TODO: here abouve we see PUBLIC STATIC MAIN VOID [functionName()]
    // 1. public static = access modiriers.
    // 2. void = Return typed
    // 3. main = name of fucntion.

    Scanner sc = new Scanner(System.in);

    System.out.println("Please input num1:");
    int a = sc.nextInt();
    System.out.println("Please input num2:");
    int b = sc.nextInt();

    System.out.println(CalcualteSum(a, b)); // TEST: (a,b) are ARGUMENTS or ACTUAL-PAREMERERS
    // NOTE: OR
    // int sum = CalcualteSum(a, b);
    // System.out.println(sum);

    // PERF: : RETURN TYPES
    // _________
    // 1VOID:
    // void is a type that dosent return anything to java-compaioer
    // it dosent have tyope so its void, hence no "return" needed.
    // _________
    // 2.INT
    // in INT return type we have to use "return <name>;" stetment so that it can
    // return certain value.
    // wihtout that it will through error.
    sc.close();
  }
}
