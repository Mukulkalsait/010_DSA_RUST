import java.util.Scanner;

/**
 * JCS_1_IfEsle
 */
public class JCS_1_IfEsle {
  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);
    int age = sc.nextInt();

    if (age >= 18) {
      System.out.println("adult !!!");
    } else if (age >= 12 && age < 18) {
      System.out.println("Teenager !!!");
    } else if (age < 0) {
      System.out.println("Yet to be born ;-)");
    } else {
      System.out.println("Kids !!!");
    }

    // FIX:
    // here if we use only if instead of else if, in cases where age is "18" both
    // conditions might be applied ( if we use age <= 18 in second case.).
    sc.close();
  }

}
